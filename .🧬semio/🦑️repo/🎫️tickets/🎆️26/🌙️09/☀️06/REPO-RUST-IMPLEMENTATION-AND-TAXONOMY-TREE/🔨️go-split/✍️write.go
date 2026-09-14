// ✍️write materialises the split packages on disk.
package main

import (
	"fmt"
	"go/ast"
	"go/parser"
	"go/token"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
)

// 🚚️splitRegionStart marks the block every emitted declaration lands in.
const splitRegionStart = "// #region 🚚️Split"

// 🚚️splitRegionEnd closes the emitted block.
const splitRegionEnd = "// #endregion 🚚️Split"

var pendingRegionRe = regexp.MustCompile(`(?s)\n// #region [^\n]*Pending\n.*?\n// #endregion [^\n]*Pending\n`)
var splitRegionRe = regexp.MustCompile(`(?s)\n` + regexp.QuoteMeta(splitRegionStart) + `\n.*?\n` + regexp.QuoteMeta(splitRegionEnd) + `\n`)

// 📦️writePackage appends one module's emitted declarations to its Go package, creating the package
// when the wave-1 agents did not.
func writePackage(module, body string, imports map[string]string, deps map[string]bool, aliases map[string]string) {
	dir := moduleDir(module)
	pkg := packageNames[module]
	must(os.MkdirAll(dir, 0o755))
	goFile := filepath.Join(dir, "🐹️.go")

	head := ""
	existing := ""
	if raw, err := os.ReadFile(goFile); err == nil {
		text := string(raw)
		text = splitRegionRe.ReplaceAllString(text, "\n")
		text = pendingRegionRe.ReplaceAllString(text, "\n")
		fset := token.NewFileSet()
		f, perr := parser.ParseFile(fset, goFile, text, parser.ParseComments)
		must(perr)
		for _, imp := range f.Imports {
			path := strings.Trim(imp.Path.Value, `"`)
			alias := canonicalAlias(path)
			if imp.Name != nil {
				alias = imp.Name.Name
			}
			imports[path] = alias
		}
		cut := len(text)
		if len(f.Decls) > 0 {
			cut = fset.Position(f.Decls[0].Pos()).Offset
			if gd, ok := f.Decls[0].(*ast.GenDecl); ok && gd.Tok == token.IMPORT {
				cut = fset.Position(gd.End()).Offset
				existing = strings.TrimLeft(text[cut:], "\n")
				cut = fset.Position(gd.Pos()).Offset
			} else {
				existing = text[cut:]
			}
		}
		head = text[:cut]
		if idx := strings.Index(head, "\npackage "); idx >= 0 {
			nl := strings.Index(head[idx+1:], "\n")
			head = head[:idx+1+nl+1]
		}
	} else {
		head = fmt.Sprintf("// #region 🧲️Header\n\n// 2026 Ueli Saluz <ueli@semio-tech.com>\n\n"+
			"// %s is the %s domain of the semio repository tooling, split out of the pre-split godfile.\n\n"+
			"// #endregion 🧲️Header\n\npackage %s\n", modulePaths[module], pkg, pkg)
	}

	for dep := range deps {
		if dep == module {
			continue
		}
		imports[modulePath(dep)] = aliases[dep]
	}

	var out strings.Builder
	out.WriteString(strings.TrimRight(head, "\n"))
	out.WriteString("\n\n")
	out.WriteString(renderImports(imports))
	if existing != "" {
		out.WriteString("\n")
		out.WriteString(strings.Trim(existing, "\n"))
		out.WriteString("\n")
	}
	out.WriteString("\n" + splitRegionStart + "\n\n")
	out.WriteString(strings.Trim(body, "\n"))
	out.WriteString("\n\n" + splitRegionEnd + "\n")

	must(writeFile(goFile, out.String()))
}

// 📥️renderImports writes a merged, sorted import block.
func renderImports(imports map[string]string) string {
	if len(imports) == 0 {
		return ""
	}
	var std, local []string
	for path, alias := range imports {
		line := fmt.Sprintf("\t%s %q", alias, path)
		if strings.HasPrefix(path, "github.com/") {
			local = append(local, line)
		} else {
			std = append(std, line)
		}
	}
	sort.Strings(std)
	sort.Strings(local)
	var b strings.Builder
	b.WriteString("import (\n")
	b.WriteString(strings.Join(std, "\n"))
	if len(local) > 0 {
		if len(std) > 0 {
			b.WriteString("\n\n")
		}
		b.WriteString(strings.Join(local, "\n"))
	}
	b.WriteString("\n)\n")
	return b.String()
}

// 🧾️finalizeModules regenerates go.mod, the nx project and the script of every split package from
// the imports its files actually carry, then rewrites the workspace file.
func finalizeModules() {
	pathOf := map[string]string{}
	for m := range packageNames {
		pathOf[modulePath(m)] = m
	}
	var used []string
	for module := range packageNames {
		dir := moduleDir(module)
		entries, err := os.ReadDir(dir)
		if err != nil {
			continue
		}
		deps := map[string]bool{}
		found := false
		for _, e := range entries {
			if e.IsDir() || !strings.HasSuffix(e.Name(), ".go") {
				continue
			}
			found = true
			raw, err := os.ReadFile(filepath.Join(dir, e.Name()))
			must(err)
			fset := token.NewFileSet()
			f, err := parser.ParseFile(fset, e.Name(), raw, parser.ImportsOnly)
			must(err)
			for _, imp := range f.Imports {
				path := strings.Trim(imp.Path.Value, "\"")
				if target, ok := pathOf[path]; ok && target != module {
					deps[target] = true
				}
			}
		}
		if !found {
			continue
		}
		used = append(used, module)
		var names []string
		for dep := range deps {
			names = append(names, dep)
		}
		sort.Strings(names)
		var b strings.Builder
		fmt.Fprintf(&b, "module %s\n\ngo 1.25\n", modulePath(module))
		if len(names) > 0 {
			b.WriteString("\nrequire (\n")
			for _, dep := range names {
				fmt.Fprintf(&b, "\t%s v0.0.0\n", modulePath(dep))
			}
			b.WriteString(")\n")
			for _, dep := range names {
				rel, err := filepath.Rel(dir, moduleDir(dep))
				must(err)
				fmt.Fprintf(&b, "\nreplace %s => %s\n", modulePath(dep), filepath.ToSlash(rel))
			}
		}
		must(writeFile(filepath.Join(dir, "go.mod"), b.String()))
		if _, err := os.Stat(filepath.Join(dir, "📋️project.json")); err != nil {
			must(writeFile(filepath.Join(dir, "📋️project.json"), projectJSON(module)))
		}
		if _, err := os.Stat(filepath.Join(dir, "📜️script.ts")); err != nil {
			must(writeFile(filepath.Join(dir, "📜️script.ts"), scriptTS(module)))
		}
	}
	sort.Strings(used)
	fmt.Println("modules with Go sources:", strings.Join(used, " "))
}

// 📋️projectJSON is the nx project definition of a Go package: the four test levels every
// repository package exposes, addressed by the package's own script.
func projectJSON(module string) string {
	dir := fmt.Sprintf("🧰️framework/🛍️products/🦑️repo/🔨️modules/%s/📦️packages/🐹️go", modulePaths[module])
	var targets []string
	for _, level := range []struct{ Target, Segments string }{
		{"test", "test"}, {"test-quick", "test quick"}, {"test-long", "test long"}, {"test-exhaustive", "test exhaustive"},
	} {
		targets = append(targets, fmt.Sprintf(`    %q: {
      "executor": "nx:run-commands",
      "options": {
        "cwd": %q,
        "command": "bun ./📜️script.ts %s"
      }
    }`, level.Target, dir, level.Segments))
	}
	return fmt.Sprintf(`{
  "name": "@semio-tech/repo-%s-go",
  "$schema": "../../../../../../../node_modules/nx/schemas/project-schema.json",
  "projectType": "library",
  "targets": {
%s
  }
}
`, packageNames[module], strings.Join(targets, ",\n"))
}

// 📜️scriptTS is the bun entry point of a Go package: one router with the level-aware test target.
func scriptTS(module string) string {
	return fmt.Sprintf(`#!/usr/bin/env bun
/** 🧭️ `+"`"+`repo-%s-go`+"`"+` router: `+"`"+`bun ./📜️script.ts test`+"`"+`. */
import { BundleScript, ScriptRouter, goCoverageArgs, goLevelTestArgs, resolveTestLevel, runBundleScriptMain, runTestBudgeted } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { level, rest } = resolveTestLevel(segments);
    const tags = level === "exhaustive" ? ["-tags", "exhaustive"] : [];
    runTestBudgeted("go", ["test", "./...", ...tags, ...goLevelTestArgs(level), ...goCoverageArgs(this.repoRoot, import.meta.dir), ...rest], { cwd: import.meta.dir });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
`, packageNames[module])
}

// 💾️writeFile writes a file with Unix newlines.
func writeFile(path, content string) error {
	tmp := path + ".tmp"
	if err := os.WriteFile(tmp, []byte(content), 0o644); err != nil {
		return err
	}
	return os.Rename(tmp, path)
}

// 🗃️existingNames returns the top-level names and method keys a package already declares before the
// split adds anything, so the splitter can drop what the wave-1 agents already moved.
func existingNames(module string) map[string]bool {
	out := map[string]bool{}
	raw, err := os.ReadFile(filepath.Join(moduleDir(module), "🐹️.go"))
	if err != nil {
		return out
	}
	text := splitRegionRe.ReplaceAllString(string(raw), "\n")
	text = pendingRegionRe.ReplaceAllString(text, "\n")
	fset := token.NewFileSet()
	f, err := parser.ParseFile(fset, "🐹️.go", text, parser.ParseComments)
	if err != nil {
		return out
	}
	for _, d := range f.Decls {
		switch n := d.(type) {
		case *ast.FuncDecl:
			recv := ""
			if n.Recv != nil && len(n.Recv.List) > 0 {
				recv = recvTypeName(n.Recv.List[0].Type)
			}
			out[identKey(recv, n.Name.Name)] = true
		case *ast.GenDecl:
			for _, spec := range n.Specs {
				switch sp := spec.(type) {
				case *ast.TypeSpec:
					out[sp.Name.Name] = true
				case *ast.ValueSpec:
					for _, nm := range sp.Names {
						out[nm.Name] = true
					}
				}
			}
		}
	}
	return out
}
