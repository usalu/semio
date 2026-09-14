// 🧩️parse turns the godfile into region-tagged, module-assigned top-level declarations.
package main

import (
	"encoding/json"
	"fmt"
	"go/ast"
	"go/token"
	"os"
	"regexp"
	"sort"
	"strings"
)

// 🗺️regionEvent is one #region/#endregion marker line found in a source file.
type regionEvent struct {
	Line int
	Kind string
	Name string
}

// 📍️regionBreakpoint records the innermost open region name starting at a given line.
type regionBreakpoint struct {
	Line int
	Top  string
}

var regionMarkerRe = regexp.MustCompile(`^\s*//\s*#(region|endregion)\s*(.*)$`)

// 🧵️buildRegionBreakpoints performs a blind depth-only stack walk over the region markers with the
// one hand-authored splice the symbol-graph tool established for the unlabeled Policies block.
func buildRegionBreakpoints(lines []string) []regionBreakpoint {
	var events []regionEvent
	for i, l := range lines {
		m := regionMarkerRe.FindStringSubmatch(l)
		if m == nil {
			continue
		}
		events = append(events, regionEvent{Line: i + 1, Kind: m[1], Name: strings.TrimSpace(m[2])})
	}
	var out []regionBreakpoint
	var stack []string
	top := func() string {
		if len(stack) == 0 {
			return ""
		}
		return stack[len(stack)-1]
	}
	spliced := false
	for _, e := range events {
		if e.Kind == "region" {
			stack = append(stack, e.Name)
		} else if len(stack) > 0 {
			stack = stack[:len(stack)-1]
		}
		out = append(out, regionBreakpoint{Line: e.Line, Top: top()})
		if !spliced && e.Kind == "endregion" && e.Name == "📝️Sections" {
			stack = append(stack, "Policies")
			out = append(out, regionBreakpoint{Line: e.Line, Top: top()})
			spliced = true
		}
	}
	return out
}

// 🔎️regionAt returns the innermost open region name at a 1-based line.
func regionAt(bps []regionBreakpoint, line int) string {
	lo, hi := 0, len(bps)-1
	res := ""
	for lo <= hi {
		mid := (lo + hi) / 2
		if bps[mid].Line <= line {
			res = bps[mid].Top
			lo = mid + 1
		} else {
			hi = mid - 1
		}
	}
	return res
}

// 📜️decl is one flattened top-level declaration of the godfile.
type decl struct {
	Name      string              `json:"name"`
	Kind      string              `json:"kind"`
	Receiver  string              `json:"receiver,omitempty"`
	File      string              `json:"file"`
	StartLine int                 `json:"startLine"`
	EndLine   int                 `json:"endLine"`
	Region    string              `json:"region"`
	Module    string              `json:"module"`
	Refs      []string            `json:"refs"`
	Imports   map[string][]string `json:"importsUsed,omitempty"`

	Node      ast.Node          `json:"-"`
	Group     *ast.GenDecl      `json:"-"`
	FileAlias map[string]string `json:"-"`
	Export    string            `json:"-"`
	Dropped   bool              `json:"-"`
	Origin    *sourceFile       `json:"-"`
}

// 🗄️sourceFile is one parsed input file with the bytes needed for verbatim slicing.
type sourceFile struct {
	Label   string
	Path    string
	Src     []byte
	File    *ast.File
	Alias   map[string]string
	Fset    *token.FileSet
	Offsets []int
}

// 🔠️identKey builds the lookup key for a receiver-qualified method.
func identKey(recv, name string) string {
	if recv == "" {
		return name
	}
	return recv + "." + name
}

// 🧹️recvTypeName strips pointer and generic decoration from a receiver type expression.
func recvTypeName(e ast.Expr) string {
	switch t := e.(type) {
	case *ast.StarExpr:
		return recvTypeName(t.X)
	case *ast.Ident:
		return t.Name
	case *ast.IndexExpr:
		return recvTypeName(t.X)
	case *ast.IndexListExpr:
		return recvTypeName(t.X)
	}
	return fmt.Sprintf("%T", e)
}

// 🏷️importAliases maps every import alias of a file to its package path.
func importAliases(f *ast.File) map[string]string {
	out := map[string]string{}
	for _, imp := range f.Imports {
		path := strings.Trim(imp.Path.Value, `"`)
		alias := ""
		if imp.Name != nil {
			alias = imp.Name.Name
		} else {
			parts := strings.Split(path, "/")
			alias = parts[len(parts)-1]
		}
		if alias == "_" || alias == "." {
			continue
		}
		out[alias] = path
	}
	return out
}

// 🗺️regionResolver is the hand-authored region→module table plus line overrides.
type regionResolver struct {
	NameToModule  map[string]string `json:"nameToModule"`
	LineOverrides []lineOverride    `json:"lineOverrides"`
}

// 📏️lineOverride forces a module for a snapshot line range.
type lineOverride struct {
	Start  int    `json:"start"`
	End    int    `json:"end"`
	Module string `json:"module"`
	Note   string `json:"note"`
}

// 📥️loadRegionModules reads the region→module table.
func loadRegionModules(path string) *regionResolver {
	b, err := os.ReadFile(path)
	must(err)
	var r regionResolver
	must(json.Unmarshal(b, &r))
	return &r
}

// 🎯️resolve answers the module for a region label at a line.
func (r *regionResolver) resolve(region string, line int) string {
	for _, o := range r.LineOverrides {
		if line >= o.Start && line <= o.End {
			return o.Module
		}
	}
	if m, ok := r.NameToModule[region]; ok {
		return m
	}
	return "UNMAPPED"
}

// 🧮️extractDecls flattens every top-level declaration of a file.
func extractDecls(src *sourceFile, resolve func(line int) (string, string)) []*decl {
	var out []*decl
	for _, d := range src.File.Decls {
		switch n := d.(type) {
		case *ast.FuncDecl:
			recv, kind := "", "func"
			if n.Recv != nil && len(n.Recv.List) > 0 {
				recv = recvTypeName(n.Recv.List[0].Type)
				kind = "method"
			}
			start := src.Fset.Position(n.Pos()).Line
			end := src.Fset.Position(n.End()).Line
			region, module := resolve(start)
			out = append(out, &decl{Name: n.Name.Name, Kind: kind, Receiver: recv, File: src.Label,
				StartLine: start, EndLine: end, Region: region, Module: module,
				Node: n, FileAlias: src.Alias, Origin: src})
		case *ast.GenDecl:
			if n.Tok == token.IMPORT {
				continue
			}
			for _, spec := range n.Specs {
				switch s := spec.(type) {
				case *ast.TypeSpec:
					start := src.Fset.Position(s.Pos()).Line
					end := src.Fset.Position(s.End()).Line
					region, module := resolve(start)
					out = append(out, &decl{Name: s.Name.Name, Kind: "type", File: src.Label,
						StartLine: start, EndLine: end, Region: region, Module: module,
						Node: s, Group: n, FileAlias: src.Alias, Origin: src})
				case *ast.ValueSpec:
					kind := "var"
					if n.Tok == token.CONST {
						kind = "const"
					}
					start := src.Fset.Position(s.Pos()).Line
					end := src.Fset.Position(s.End()).Line
					region, module := resolve(start)
					for _, nm := range s.Names {
						if nm.Name == "_" {
							continue
						}
						out = append(out, &decl{Name: nm.Name, Kind: kind, File: src.Label,
							StartLine: start, EndLine: end, Region: region, Module: module,
							Node: s, Group: n, FileAlias: src.Alias, Origin: src})
					}
				}
			}
		}
	}
	return out
}

// 🧺️collectLocals gathers every name bound locally inside a declaration.
func collectLocals(n ast.Node) map[string]bool {
	locals := map[string]bool{}
	addFieldList := func(fl *ast.FieldList) {
		if fl == nil {
			return
		}
		for _, f := range fl.List {
			for _, nm := range f.Names {
				locals[nm.Name] = true
			}
		}
	}
	if fd, ok := n.(*ast.FuncDecl); ok {
		addFieldList(fd.Recv)
		if fd.Type != nil {
			addFieldList(fd.Type.Params)
			addFieldList(fd.Type.Results)
			addFieldList(fd.Type.TypeParams)
		}
	}
	ast.Inspect(n, func(node ast.Node) bool {
		switch x := node.(type) {
		case *ast.AssignStmt:
			if x.Tok == token.DEFINE {
				for _, lhs := range x.Lhs {
					if id, ok := lhs.(*ast.Ident); ok {
						locals[id.Name] = true
					}
				}
			}
		case *ast.RangeStmt:
			if x.Tok == token.DEFINE {
				if id, ok := x.Key.(*ast.Ident); ok {
					locals[id.Name] = true
				}
				if x.Value != nil {
					if id, ok := x.Value.(*ast.Ident); ok {
						locals[id.Name] = true
					}
				}
			}
		case *ast.GenDecl:
			for _, spec := range x.Specs {
				switch s := spec.(type) {
				case *ast.ValueSpec:
					for _, nm := range s.Names {
						locals[nm.Name] = true
					}
				case *ast.TypeSpec:
					locals[s.Name.Name] = true
				}
			}
		case *ast.FuncLit:
			addFieldList(x.Type.Params)
			addFieldList(x.Type.Results)
		case *ast.TypeSwitchStmt:
			if as, ok := x.Assign.(*ast.AssignStmt); ok {
				for _, lhs := range as.Lhs {
					if id, ok := lhs.(*ast.Ident); ok {
						locals[id.Name] = true
					}
				}
			}
		case *ast.LabeledStmt:
			locals[x.Label.Name] = true
		}
		return true
	})
	return locals
}

// 🚫️buildSkipSet marks every identifier of a declaration that is not a reference to a package-level
// name: declared names, struct and interface field names, labels, selector members, import aliases
// and struct-literal keys.
func buildSkipSet(n ast.Node, alias map[string]string, imports map[string]map[string]bool) map[*ast.Ident]bool {
	skip := map[*ast.Ident]bool{}
	markFields := func(fl *ast.FieldList) {
		if fl == nil {
			return
		}
		for _, f := range fl.List {
			for _, nm := range f.Names {
				skip[nm] = true
			}
		}
	}
	nested := map[*ast.CompositeLit]bool{}
	var markLit func(lit *ast.CompositeLit, structish bool)
	markLit = func(lit *ast.CompositeLit, structish bool) {
		isStruct := structish
		childStruct := true
		if lit.Type != nil {
			isStruct = typeIsStructish(lit.Type)
			childStruct = elementIsStructish(lit.Type)
		}
		for _, elt := range lit.Elts {
			if kv, ok := elt.(*ast.KeyValueExpr); ok {
				if isStruct {
					if id, ok := kv.Key.(*ast.Ident); ok {
						skip[id] = true
					}
				}
				if inner, ok := kv.Value.(*ast.CompositeLit); ok {
					nested[inner] = true
					markLit(inner, childStruct)
				}
				continue
			}
			if inner, ok := elt.(*ast.CompositeLit); ok {
				nested[inner] = true
				markLit(inner, childStruct)
			}
		}
	}
	ast.Inspect(n, func(node ast.Node) bool {
		switch x := node.(type) {
		case *ast.FuncDecl:
			skip[x.Name] = true
			markFields(x.Recv)
			if x.Type != nil {
				markFields(x.Type.Params)
				markFields(x.Type.Results)
				markFields(x.Type.TypeParams)
			}
		case *ast.FuncLit:
			markFields(x.Type.Params)
			markFields(x.Type.Results)
		case *ast.FuncType:
			markFields(x.Params)
			markFields(x.Results)
			markFields(x.TypeParams)
		case *ast.StructType:
			markFields(x.Fields)
		case *ast.InterfaceType:
			markFields(x.Methods)
		case *ast.LabeledStmt:
			skip[x.Label] = true
		case *ast.BranchStmt:
			if x.Label != nil {
				skip[x.Label] = true
			}
		case *ast.SelectorExpr:
			skip[x.Sel] = true
			if id, ok := x.X.(*ast.Ident); ok {
				if pkgPath, isImport := alias[id.Name]; isImport {
					skip[id] = true
					if imports != nil {
						if imports[pkgPath] == nil {
							imports[pkgPath] = map[string]bool{}
						}
						imports[pkgPath][x.Sel.Name] = true
					}
				}
			}
		case *ast.CompositeLit:
			if !nested[x] {
				markLit(x, true)
			}
		}
		return true
	})
	return skip
}

// 🧱️typeIsStructish answers whether a composite literal of this type has struct field keys.
func typeIsStructish(e ast.Expr) bool {
	switch t := e.(type) {
	case *ast.StarExpr:
		return typeIsStructish(t.X)
	case *ast.MapType, *ast.ArrayType:
		return false
	case *ast.ParenExpr:
		return typeIsStructish(t.X)
	}
	return true
}

// 🧬️elementIsStructish answers whether the elements of a composite literal of this type are structs.
func elementIsStructish(e ast.Expr) bool {
	switch t := e.(type) {
	case *ast.StarExpr:
		return elementIsStructish(t.X)
	case *ast.ArrayType:
		return typeIsStructish(t.Elt)
	case *ast.MapType:
		return typeIsStructish(t.Value)
	}
	return true
}

// 🕸️declRefs collects the package-level names a declaration references.
func declRefs(d *decl, topNames map[string]bool) ([]string, map[string][]string) {
	locals := collectLocals(d.Node)
	alias := map[string]string{}
	for a, path := range d.FileAlias {
		if !locals[a] {
			alias[a] = path
		}
	}
	imports := map[string]map[string]bool{}
	skip := buildSkipSet(d.Node, alias, imports)
	refs := map[string]bool{}
	ast.Inspect(d.Node, func(node ast.Node) bool {
		id, ok := node.(*ast.Ident)
		if !ok || skip[id] {
			return true
		}
		if topNames[id.Name] && !locals[id.Name] {
			refs[id.Name] = true
		}
		return true
	})
	var out map[string][]string
	if len(imports) > 0 {
		out = map[string][]string{}
		for k, v := range imports {
			out[k] = sortedKeys(v)
		}
	}
	return sortedKeys(refs), out
}

// 🔤️sortedKeys returns the sorted keys of a set.
func sortedKeys(m map[string]bool) []string {
	out := make([]string, 0, len(m))
	for k := range m {
		out = append(out, k)
	}
	sort.Strings(out)
	return out
}

// 💥️must panics on error.
func must(err error) {
	if err != nil {
		panic(err)
	}
}
