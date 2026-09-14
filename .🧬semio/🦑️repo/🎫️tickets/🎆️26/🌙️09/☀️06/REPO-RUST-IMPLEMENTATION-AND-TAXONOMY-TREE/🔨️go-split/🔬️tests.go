// 🔬️tests splits the godfile's test suites across the domain packages.
package main

import (
	"fmt"
	"go/ast"
	"os"
	"path/filepath"
	"sort"
	"strings"
)

// 🧪️testDecl is one top-level declaration of a test file.
type testDecl struct {
	Name    string
	Kind    string
	Src     *sourceFile
	Node    ast.Node
	Group   *ast.GenDecl
	IsTest  bool
	Refs    []string
	Helpers []string
	Order   int
}

// 🗂️testPlan is the outcome of the test triage.
type testPlan struct {
	owner      map[string]string
	refModules map[string]map[string]bool
	decls      []*testDecl
	byName     map[string]*testDecl
}

// 🧭️assignTests decides which package each test and helper of the godfile test files belongs to.
func assignTests(cfg *config, tests []*sourceFile, byName map[string]*decl, topNames map[string]bool) (*testPlan, map[string][]*testDecl) {
	plan := &testPlan{owner: map[string]string{}, refModules: map[string]map[string]bool{}, byName: map[string]*testDecl{}}
	localNames := map[string]bool{}
	for _, src := range tests {
		for _, d := range src.File.Decls {
			switch n := d.(type) {
			case *ast.FuncDecl:
				if n.Recv != nil {
					continue
				}
				localNames[n.Name.Name] = true
			case *ast.GenDecl:
				for _, spec := range n.Specs {
					switch s := spec.(type) {
					case *ast.TypeSpec:
						localNames[s.Name.Name] = true
					case *ast.ValueSpec:
						for _, nm := range s.Names {
							localNames[nm.Name] = true
						}
					}
				}
			}
		}
	}
	order := 0
	for _, src := range tests {
		for _, d := range src.File.Decls {
			switch n := d.(type) {
			case *ast.FuncDecl:
				order++
				td := &testDecl{Name: n.Name.Name, Kind: "func", Src: src, Node: n, Order: order,
					IsTest: strings.HasPrefix(n.Name.Name, "Test") || strings.HasPrefix(n.Name.Name, "Benchmark") || strings.HasPrefix(n.Name.Name, "Fuzz")}
				if n.Recv != nil {
					td.IsTest = false
					td.Name = recvTypeName(n.Recv.List[0].Type) + "." + n.Name.Name
					td.Kind = "method"
				}
				plan.decls = append(plan.decls, td)
			case *ast.GenDecl:
				if n.Tok.String() == "import" {
					continue
				}
				order++
				name := ""
				for _, spec := range n.Specs {
					switch s := spec.(type) {
					case *ast.TypeSpec:
						name = s.Name.Name
					case *ast.ValueSpec:
						if len(s.Names) > 0 && name == "" {
							name = s.Names[0].Name
						}
					}
				}
				plan.decls = append(plan.decls, &testDecl{Name: name, Kind: "gen", Src: src, Node: n, Group: n, Order: order})
			}
		}
	}
	for _, td := range plan.decls {
		if td.Kind != "method" {
			plan.byName[td.Name] = td
		}
	}
	for _, td := range plan.decls {
		locals := collectLocals(td.Node)
		skip := buildSkipSet(td.Node, td.Src.Alias, nil)
		prod := map[string]bool{}
		help := map[string]bool{}
		ast.Inspect(td.Node, func(n ast.Node) bool {
			id, ok := n.(*ast.Ident)
			if !ok || skip[id] || locals[id.Name] {
				return true
			}
			if localNames[id.Name] && id.Name != td.Name {
				help[id.Name] = true
			} else if topNames[id.Name] {
				prod[id.Name] = true
			}
			return true
		})
		td.Refs = sortedKeys(prod)
		td.Helpers = sortedKeys(help)
	}

	rank := func(m string) float64 { return levels[m] }
	var moduleOf func(td *testDecl, seen map[string]bool) string
	moduleOf = func(td *testDecl, seen map[string]bool) string {
		best := ""
		consider := func(m string) {
			if m == "" {
				return
			}
			if best == "" || rank(m) > rank(best) {
				best = m
			}
		}
		for _, r := range td.Refs {
			if d, ok := byName[r]; ok {
				consider(d.Module)
			}
		}
		for _, h := range td.Helpers {
			if seen[h] {
				continue
			}
			seen[h] = true
			if hd, ok := plan.byName[h]; ok {
				consider(moduleOf(hd, seen))
			}
		}
		return best
	}
	for _, td := range plan.decls {
		if !td.IsTest {
			continue
		}
		m := moduleOf(td, map[string]bool{td.Name: true})
		if forced, ok := cfg.TestOverrides[td.Name]; ok {
			m = forced
		}
		if m == "" {
			m = "cli"
		}
		plan.owner[td.Name] = m
	}

	// 🧵️pull the helper closure into every owning module.
	perModule := map[string][]*testDecl{}
	added := map[string]bool{}
	var pull func(module string, td *testDecl)
	pull = func(module string, td *testDecl) {
		key := module + "\x00" + td.Name + fmt.Sprint(td.Order)
		if added[key] {
			return
		}
		added[key] = true
		perModule[module] = append(perModule[module], td)
		for _, r := range td.Refs {
			if d, ok := byName[r]; ok {
				if plan.refModules[r] == nil {
					plan.refModules[r] = map[string]bool{}
				}
				plan.refModules[r][module] = true
				_ = d
			}
		}
		for _, h := range td.Helpers {
			if hd, ok := plan.byName[h]; ok {
				pull(module, hd)
			}
		}
		if td.Kind == "gen" || td.Kind == "func" {
			for _, other := range plan.decls {
				if other.Kind == "method" && strings.HasPrefix(other.Name, td.Name+".") {
					pull(module, other)
				}
			}
		}
	}
	for _, td := range plan.decls {
		if !td.IsTest {
			continue
		}
		pull(plan.owner[td.Name], td)
	}
	return plan, perModule
}

// ✍️writeTests renders one test file per module.
func writeTests(plan *testPlan, perModule map[string][]*testDecl, byName map[string]*decl, topNames map[string]bool, pathToModule map[string]string) {
	for module, decls := range perModule {
		sort.SliceStable(decls, func(i, j int) bool { return decls[i].Order < decls[j].Order })
		exhaustiveOwned := map[string]bool{}
		for _, td := range decls {
			if td.IsTest && strings.HasPrefix(td.Name, "TestExhaustive") {
				exhaustiveOwned[td.Name] = true
			}
		}
		var plain, deep []*testDecl
		for _, td := range decls {
			if td.IsTest && exhaustiveOwned[td.Name] {
				deep = append(deep, td)
				continue
			}
			plain = append(plain, td)
		}
		writeTestFile(module, "🔬️_test.go", false, plain, byName, pathToModule)
		if len(exhaustiveOwned) > 0 {
			writeTestFile(module, "🔭️exhaustive_test.go", true, deep, byName, pathToModule)
		} else {
			os.Remove(filepath.Join(moduleDir(module), "🔭️exhaustive_test.go"))
		}
	}
}

// 📝️writeTestFile renders one test file, optionally behind the exhaustive build tag.
func writeTestFile(module, name string, tagged bool, decls []*testDecl, byName map[string]*decl, pathToModule map[string]string) {
	if len(decls) == 0 {
		os.Remove(filepath.Join(moduleDir(module), name))
		return
	}
	imports := map[string]string{}
	deps := map[string]bool{}
	shadowed := map[string]bool{}
	for _, td := range decls {
		for name := range collectLocals(td.Node) {
			shadowed[name] = true
		}
	}
	aliases := packageAliases(shadowed)
	var body strings.Builder
	for _, td := range decls {
		body.WriteString(renderTestDecl(module, td, byName, pathToModule, imports, deps, aliases))
		body.WriteString("\n\n")
	}
	for dep := range deps {
		if dep != module {
			imports[modulePath(dep)] = aliases[dep]
		}
	}
	var out strings.Builder
	if tagged {
		out.WriteString("//go:build exhaustive\n\n")
	}
	out.WriteString(fmt.Sprintf("// 🔬️ Tests of the %s domain, split out of the pre-split godfile suite.\n\npackage %s\n\n",
		packageNames[module], packageNames[module]))
	out.WriteString(renderImports(imports))
	out.WriteString("\n")
	out.WriteString(strings.Trim(body.String(), "\n"))
	out.WriteString("\n")
	must(writeFile(filepath.Join(moduleDir(module), name), out.String()))
}

// 🖋️renderTestDecl rewrites one test declaration for its new package.
func renderTestDecl(module string, td *testDecl, byName map[string]*decl, pathToModule map[string]string, imports map[string]string, deps map[string]bool, aliases map[string]string) string {
	src := td.Src
	start := src.Fset.Position(td.Node.Pos()).Offset
	end := src.Fset.Position(td.Node.End()).Offset
	if fd, ok := td.Node.(*ast.FuncDecl); ok && fd.Doc != nil {
		if o := src.Fset.Position(fd.Doc.Pos()).Offset; o < start {
			start = o
		}
	}
	if gd, ok := td.Node.(*ast.GenDecl); ok && gd.Doc != nil {
		if o := src.Fset.Position(gd.Doc.Pos()).Offset; o < start {
			start = o
		}
	}
	locals := collectLocals(td.Node)
	skip := buildSkipSet(td.Node, src.Alias, nil)
	var reps []replacement
	rewriteRef := func(id *ast.Ident) {
		if skip[id] || locals[id.Name] {
			return
		}
		t, ok := lookup(src, id.Name, byName)
		if !ok {
			return
		}
		p := src.Fset.Position(id.Pos()).Offset
		text := t.Export
		if t.Module != module {
			text = aliases[t.Module] + "." + t.Export
			deps[t.Module] = true
		}
		if text != id.Name {
			reps = append(reps, replacement{p, p + len(id.Name), text})
		}
	}
	var visit func(ast.Node) bool
	visit = func(n ast.Node) bool {
		switch x := n.(type) {
		case *ast.SelectorExpr:
			if id, ok := x.X.(*ast.Ident); ok {
				if path, isImport := src.Alias[id.Name]; isImport && !locals[id.Name] {
					s := src.Fset.Position(id.Pos()).Offset
					e := src.Fset.Position(x.Sel.Pos()).Offset
					if target, mine := pathToModule[path]; mine {
						if target == module {
							reps = append(reps, replacement{s, e, ""})
						} else {
							reps = append(reps, replacement{s, e, aliases[target] + "."})
							deps[target] = true
						}
					} else {
						alias := canonicalAlias(path)
						imports[path] = alias
						if alias != id.Name {
							reps = append(reps, replacement{s, e, alias + "."})
						}
					}
					return false
				}
			}
			ast.Inspect(x.X, visit)
			return false
		case *ast.Ident:
			rewriteRef(x)
			return false
		}
		return true
	}
	ast.Inspect(td.Node, visit)
	return applyReplacements(src.Src[start:end], start, reps)
}

