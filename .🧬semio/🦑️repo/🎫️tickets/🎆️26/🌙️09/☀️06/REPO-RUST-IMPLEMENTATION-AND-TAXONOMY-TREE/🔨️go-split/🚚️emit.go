// 🚚️emit writes the assigned declarations into the domain Go packages.
package main

import (
	"fmt"
	"go/ast"
	"go/token"
	"path/filepath"
	"sort"
	"strings"
	"unicode"
)

// 🧱️unit is one emitted top-level item: a function, a whole cohesive declaration group, or a
// single specification of a group.
type unit struct {
	Module  string
	Src     *sourceFile
	Node    ast.Node
	Group   *ast.GenDecl
	Whole   bool
	Names   []*decl
	Order   int
	Region  string
	StartLn int
}

// 🔁️replacement is one byte range of the original source to rewrite.
type replacement struct {
	Start int
	End   int
	Text  string
}

// 📦️modulePath is the Go module path of a split target.
func modulePath(module string) string { return "github.com/usalu/semio/repo/" + packageNames[module] }

// 📁️moduleDir is the Go package directory of a split target.
func moduleDir(module string) string {
	return filepath.Join(repoRoot, "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules",
		modulePaths[module], "📦️packages", "🐹️go")
}

// 🔑️declRefKey is the file-qualified identity of a declaration; two same-named declarations from
// different source packages must not share a rename or a usage set.
func declRefKey(d *decl) string { return d.Origin.Label + ":" + d.Name }

// 🔠️exportName capitalises the first rune of a name.
func exportName(name string) string {
	if name == "" {
		return name
	}
	r := []rune(name)
	r[0] = unicode.ToUpper(r[0])
	return string(r)
}

// 🚚️emit performs the whole code generation pass.
func emit(cfg *config, decls []*decl, byName, byKey map[string]*decl, topNames map[string]bool, tests []*sourceFile) {
	pathToModule := map[string]string{
		"github.com/usalu/semio/repo/client/internal/command":      "cli",
		"github.com/usalu/semio/repo/client/internal/templatefunc": "model",
		"github.com/usalu/semio/repo/client/internal/mcp":          "cli",
		"github.com/usalu/semio/repo/client/internal/mcpserver":    "cli",
	}
	for m := range packageNames {
		pathToModule[modulePath(m)] = m
	}

	testOwners, testHelpers := assignTests(cfg, tests, byName, topNames)

	// 🏷️decide the exported name of every declaration.
	usedFrom := map[string]map[string]bool{}
	for _, d := range decls {
		for _, ref := range d.Refs {
			t, ok := lookup(d.Origin, ref, byName)
			if !ok {
				continue
			}
			ref = declRefKey(t)
			if usedFrom[ref] == nil {
				usedFrom[ref] = map[string]bool{}
			}
			usedFrom[ref][d.Module] = true
		}
	}
	for name, mods := range testOwners.refModules {
		t, ok := byName[name]
		if !ok {
			continue
		}
		key := declRefKey(t)
		if usedFrom[key] == nil {
			usedFrom[key] = map[string]bool{}
		}
		for m := range mods {
			usedFrom[key][m] = true
		}
	}
	exported := 0
	for _, d := range decls {
		if d.Kind == "method" || d.Name == "init" {
			continue
		}
		final := d.Name
		if !ast.IsExported(d.Name) {
			for m := range usedFrom[declRefKey(d)] {
				if m != d.Module {
					final = exportName(d.Name)
					exported++
					break
				}
			}
		}
		if forced, ok := cfg.Renames[declRefKey(d)]; ok {
			final = forced
		} else if forced, ok := cfg.Renames[d.Name]; ok {
			final = forced
		}
		d.Export = final
	}

	// 🚨️collide guards against two declarations landing on one name in one package.
	seen := map[string]string{}
	for _, d := range decls {
		if d.Kind == "method" || d.Name == "init" {
			continue
		}
		key := d.Module + "." + d.Export
		if prev, ok := seen[key]; ok && prev != d.Name {
			fmt.Printf("[COLLISION] %s: %s and %s\n", key, prev, d.Name)
		}
		seen[key] = d.Name
	}

	// 🗑️drop what the wave-1 packages already declare; the split must not redeclare it.
	present := map[string]map[string]bool{}
	for m := range packageNames {
		present[m] = existingNames(m)
	}
	dropTypes := map[string]map[string]bool{}
	var dropped []string
	for _, d := range decls {
		if d.Name == "init" {
			continue
		}
		if d.Kind == "method" {
			continue
		}
		if present[d.Module][d.Name] || present[d.Module][d.Export] {
			d.Dropped = true
			dropped = append(dropped, d.Module+"."+d.Name)
			if dropTypes[d.Module] == nil {
				dropTypes[d.Module] = map[string]bool{}
			}
			dropTypes[d.Module][d.Name] = true
		}
	}
	for _, d := range decls {
		if d.Kind == "method" && (dropTypes[d.Module][d.Receiver] || present[d.Module][identKey(d.Receiver, d.Name)]) {
			d.Dropped = true
			dropped = append(dropped, d.Module+"."+d.Receiver+"."+d.Name)
		}
	}
	for _, name := range cfg.Drop {
		for _, d := range decls {
			if identKey(d.Receiver, d.Name) == name {
				d.Dropped = true
			}
		}
	}
	sort.Strings(dropped)
	fmt.Println("dropped (already in the wave-1 package):", len(dropped))
	for _, name := range dropped {
		fmt.Println("  -", name)
	}

	live := decls[:0:0]
	for _, d := range decls {
		if !d.Dropped {
			live = append(live, d)
		}
	}
	units := buildUnits(live)
	byModule := map[string][]*unit{}
	for _, u := range units {
		byModule[u.Module] = append(byModule[u.Module], u)
	}

	for module, us := range byModule {
		sort.SliceStable(us, func(i, j int) bool { return us[i].Order < us[j].Order })
		body, imports, deps, aliases := renderUnits(module, us, byName, topNames, pathToModule)
		writePackage(module, body, imports, deps, aliases)
	}
	writeTests(testOwners, testHelpers, byName, topNames, pathToModule)
	for m := range packageNames {
		pruneModule(moduleDir(m))
	}
	finalizeModules()
	fmt.Printf("emitted modules=%d exportedRenames=%d\n", len(byModule), exported)
}

// 🧮️buildUnits groups declarations into emittable units, keeping iota and implicit constant groups
// whole because Go cannot split them.
func buildUnits(decls []*decl) []*unit {
	groups := map[*ast.GenDecl][]*decl{}
	var order []*ast.GenDecl
	var out []*unit
	for i, d := range decls {
		if d.Group == nil {
			out = append(out, &unit{Module: d.Module, Src: d.Origin, Node: d.Node, Names: []*decl{d},
				Order: d.StartLine*100 + i%100, Region: d.Region, StartLn: d.StartLine})
			continue
		}
		if _, ok := groups[d.Group]; !ok {
			order = append(order, d.Group)
		}
		groups[d.Group] = append(groups[d.Group], d)
	}
	for _, g := range order {
		members := groups[g]
		if groupIsCohesive(g) {
			mod := members[0].Module
			for _, m := range members {
				m.Module = mod
			}
			out = append(out, &unit{Module: mod, Src: members[0].Origin, Node: g, Group: g, Whole: true,
				Names: members, Order: members[0].StartLine * 100, Region: members[0].Region,
				StartLn: members[0].StartLine})
			continue
		}
		bySpec := map[ast.Node][]*decl{}
		var specOrder []ast.Node
		for _, m := range members {
			if _, ok := bySpec[m.Node]; !ok {
				specOrder = append(specOrder, m.Node)
			}
			bySpec[m.Node] = append(bySpec[m.Node], m)
		}
		for _, sp := range specOrder {
			ms := bySpec[sp]
			mod := ms[0].Module
			for _, m := range ms {
				m.Module = mod
			}
			whole := len(g.Specs) == 1
			node := sp
			if whole {
				node = g
			}
			out = append(out, &unit{Module: mod, Src: ms[0].Origin, Node: node, Group: g, Whole: whole,
				Names: ms, Order: ms[0].StartLine * 100, Region: ms[0].Region, StartLn: ms[0].StartLine})
		}
	}
	sort.SliceStable(out, func(i, j int) bool { return out[i].Order < out[j].Order })
	return out
}

// 🔗️groupIsCohesive reports whether a declaration group must move as one piece.
func groupIsCohesive(g *ast.GenDecl) bool {
	if g.Tok != token.CONST {
		return false
	}
	for i, spec := range g.Specs {
		vs, ok := spec.(*ast.ValueSpec)
		if !ok {
			continue
		}
		if len(vs.Values) == 0 && i > 0 {
			return true
		}
		for _, v := range vs.Values {
			found := false
			ast.Inspect(v, func(n ast.Node) bool {
				if id, ok := n.(*ast.Ident); ok && id.Name == "iota" {
					found = true
				}
				return true
			})
			if found {
				return true
			}
		}
	}
	return false
}

// ✍️renderUnits rewrites every unit of one module into final source text.
func renderUnits(module string, units []*unit, byName map[string]*decl, topNames map[string]bool, pathToModule map[string]string) (string, map[string]string, map[string]bool, map[string]string) {
	var out strings.Builder
	imports := map[string]string{}
	deps := map[string]bool{}
	shadowed := map[string]bool{}
	for _, u := range units {
		for name := range collectLocals(u.Node) {
			shadowed[name] = true
		}
	}
	aliases := packageAliases(shadowed)
	region := ""
	for _, u := range units {
		if u.Region != region {
			if region != "" {
				out.WriteString("// #endregion " + region + "\n\n")
			}
			region = u.Region
			out.WriteString("// #region " + region + "\n\n")
		}
		out.WriteString(renderUnit(module, u, byName, topNames, pathToModule, imports, deps, aliases))
		out.WriteString("\n\n")
	}
	if region != "" {
		out.WriteString("// #endregion " + region + "\n")
	}
	return out.String(), imports, deps, aliases
}

// 🏷️packageAliases picks an import alias per split module that no local name shadows.
func packageAliases(shadowed map[string]bool) map[string]string {
	out := map[string]string{}
	for m, name := range packageNames {
		if shadowed[name] {
			name += "pkg"
		}
		out[m] = name
	}
	return out
}

// 🖋️renderUnit rewrites one unit's source bytes.
func renderUnit(module string, u *unit, byName map[string]*decl, topNames map[string]bool, pathToModule map[string]string, imports map[string]string, deps map[string]bool, aliases map[string]string) string {
	src := u.Src
	node := u.Node
	start := src.Fset.Position(node.Pos()).Offset
	end := src.Fset.Position(node.End()).Offset
	docText := ""
	if doc := docOf(node, u.Group, u.Whole); doc != nil {
		if o := src.Fset.Position(doc.Pos()).Offset; o < start {
			docText = string(src.Src[o:start])
		}
	}
	var reps []replacement
	locals := collectLocals(node)
	for _, d := range u.Names {
		delete(locals, d.Name)
	}
	skip := buildSkipSet(node, src.Alias, nil)

	addRef := func(id *ast.Ident) {
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
			addRef(x)
			return false
		}
		return true
	}
	ast.Inspect(node, visit)
	// 🏷️the declared names themselves.
	for _, d := range u.Names {
		if d.Kind == "method" || d.Name == "init" {
			continue
		}
		if d.Export == d.Name {
			continue
		}
		for _, id := range declaredIdents(u, d.Name) {
			p := src.Fset.Position(id.Pos()).Offset
			reps = append(reps, replacement{p, p + len(d.Name), d.Export})
		}
	}
	text := applyReplacements(src.Src[start:end], start, reps)
	if u.Group != nil && !u.Whole {
		text = groupKeyword(u.Group) + " " + text
	}
	return docText + text
}

// 📝️docOf returns the doc comment attached to a node.
func docOf(node ast.Node, group *ast.GenDecl, whole bool) *ast.CommentGroup {
	switch x := node.(type) {
	case *ast.FuncDecl:
		return x.Doc
	case *ast.GenDecl:
		return x.Doc
	case *ast.TypeSpec:
		if x.Doc != nil {
			return x.Doc
		}
		if group != nil && len(group.Specs) == 1 {
			return group.Doc
		}
	case *ast.ValueSpec:
		if x.Doc != nil {
			return x.Doc
		}
		if group != nil && len(group.Specs) == 1 {
			return group.Doc
		}
	}
	return nil
}

// 🔑️groupKeyword returns the keyword a lone specification needs when it leaves its group.
func groupKeyword(g *ast.GenDecl) string {
	switch g.Tok {
	case token.CONST:
		return "const"
	case token.VAR:
		return "var"
	}
	return "type"
}

// 🏷️declaredIdents finds the identifier nodes that declare a name inside a unit.
func declaredIdents(u *unit, name string) []*ast.Ident {
	var out []*ast.Ident
	switch x := u.Node.(type) {
	case *ast.FuncDecl:
		if x.Name.Name == name {
			out = append(out, x.Name)
		}
	case *ast.TypeSpec:
		if x.Name.Name == name {
			out = append(out, x.Name)
		}
	case *ast.ValueSpec:
		for _, nm := range x.Names {
			if nm.Name == name {
				out = append(out, nm)
			}
		}
	case *ast.GenDecl:
		for _, spec := range x.Specs {
			switch s := spec.(type) {
			case *ast.TypeSpec:
				if s.Name.Name == name {
					out = append(out, s.Name)
				}
			case *ast.ValueSpec:
				for _, nm := range s.Names {
					if nm.Name == name {
						out = append(out, nm)
					}
				}
			}
		}
	}
	return out
}

// 🧵️applyReplacements splices the rewrites into a source slice.
func applyReplacements(src []byte, base int, reps []replacement) string {
	sort.Slice(reps, func(i, j int) bool { return reps[i].Start < reps[j].Start })
	var out strings.Builder
	cursor := 0
	for _, r := range reps {
		s, e := r.Start-base, r.End-base
		if s < cursor || s < 0 || e > len(src) {
			continue
		}
		out.Write(src[cursor:s])
		out.WriteString(r.Text)
		cursor = e
	}
	out.Write(src[cursor:])
	return out.String()
}

// 🏷️canonicalAlias picks one stable alias per imported package path.
func canonicalAlias(path string) string {
	parts := strings.Split(path, "/")
	last := parts[len(parts)-1]
	switch path {
	case "math/rand":
		return "rand"
	}
	return last
}
