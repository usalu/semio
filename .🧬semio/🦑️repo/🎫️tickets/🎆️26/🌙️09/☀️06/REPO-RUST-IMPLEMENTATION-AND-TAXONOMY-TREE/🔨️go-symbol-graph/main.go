// 🔬️symbolgraph parses the client snapshot and emits a decl/region/module graph for the taxonomy-tree ticket.
package main

import (
	"encoding/json"
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

// 🗺️regionEvent is one #region/#endregion marker line found in a source file.
type regionEvent struct {
	Line int
	Kind string // "region" | "endregion"
	Name string
}

// 📍regionBreakpoint records the innermost open region name starting at a given line.
type regionBreakpoint struct {
	Line int
	Top  string // "" if no region open
}

var regionMarkerRe = regexp.MustCompile(`^\s*//\s*#(region|endregion)\s*(.*)$`)

// 🧵buildRegionBreakpoints performs a blind (depth-only) stack walk over #region/#endregion
// markers, with one hand-authored fix-up: the file has a stray, unmatched
// "// #endregion 🧊️Policies" at line 21613 closing an unlabeled block that starts right after
// the "// #endregion 📝️Sections" at line 18420 (no "// #region" was ever written for it). We
// splice in a virtual "Policies" push immediately after line 18420 so the stack stays balanced
// for everything that follows, per the ticket's explicit instruction.
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
	policiesSpliced := false
	for _, e := range events {
		if e.Kind == "region" {
			stack = append(stack, e.Name)
		} else {
			if len(stack) > 0 {
				stack = stack[:len(stack)-1]
			}
		}
		out = append(out, regionBreakpoint{Line: e.Line, Top: top()})
		if !policiesSpliced && e.Kind == "endregion" && e.Name == "📝️Sections" {
			stack = append(stack, "Policies")
			out = append(out, regionBreakpoint{Line: e.Line, Top: top()})
			policiesSpliced = true
		}
	}
	return out
}

// 🔎regionAt returns the innermost open region name at a given 1-based line via binary search.
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

// 📜decl is one flattened top-level declaration.
type decl struct {
	Name      string   `json:"name"`
	Kind      string   `json:"kind"` // func|method|type|var|const
	Receiver  string   `json:"receiver,omitempty"`
	File      string   `json:"file"`
	StartLine int      `json:"startLine"`
	EndLine   int      `json:"endLine"`
	Region    string   `json:"region"`
	Module    string   `json:"module"`
	Refs      []string `json:"refs"`
	Imports   map[string][]string `json:"importsUsed,omitempty"`
	Node      ast.Node `json:"-"`
	FileAlias map[string]string `json:"-"`
}

// 🔠identKey builds the lookup key for a receiver-qualified method.
func identKey(recv, name string) string {
	if recv == "" {
		return name
	}
	return recv + "." + name
}

// 🧹recvTypeName strips pointer/generic decoration from a receiver type expr.
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

func main() {
	ticketDir := mustFindTicketDir()
	snapshotDir := filepath.Join(ticketDir, "🗑️generated", "go-snapshot", "client")
	outDir := filepath.Join(ticketDir, "🗑️generated", "go-symbol-graph")
	os.MkdirAll(outDir, 0o755)

	fset := token.NewFileSet()

	compPath := filepath.Join(snapshotDir, "🧩️component.go")
	eventPath := filepath.Join(snapshotDir, "📤️event_export.go")
	testPath := filepath.Join(snapshotDir, "🔬️component_test.go")

	compSrc, err := os.ReadFile(compPath)
	must(err)
	compFile, err := parser.ParseFile(fset, compPath, compSrc, parser.ParseComments)
	must(err)

	eventSrc, err := os.ReadFile(eventPath)
	must(err)
	eventFile, err := parser.ParseFile(fset, eventPath, eventSrc, parser.ParseComments)
	must(err)

	testSrc, err := os.ReadFile(testPath)
	must(err)
	testFile, err := parser.ParseFile(fset, testPath, testSrc, parser.ParseComments)
	must(err)

	compLines := strings.Split(string(compSrc), "\n")
	bps := buildRegionBreakpoints(compLines)

	region := loadRegionModules(filepath.Join(ticketDir, "🔨️go-symbol-graph", "🔣️region-modules.json"))

	compAlias := importAliases(compFile)
	eventAlias := importAliases(eventFile)
	testAlias := importAliases(testFile)

	var decls []*decl
	decls = append(decls, extractDecls(fset, compFile, "🧩️component.go", compAlias, func(line int) (string, string) {
		r := regionAt(bps, line)
		return r, region.resolve(r, line)
	})...)
	decls = append(decls, extractDecls(fset, eventFile, "📤️event_export.go", eventAlias, func(line int) (string, string) {
		return "EventExport", "events"
	})...)

	topNames := map[string]bool{}   // bare func/type/var/const names (not methods)
	allDeclNames := map[string]*decl{} // identKey -> decl (methods included, keyed with receiver)
	for _, d := range decls {
		allDeclNames[identKey(d.Receiver, d.Name)] = d
		if d.Kind != "method" {
			topNames[d.Name] = true
		}
	}

	// 🔗compute refs for each production decl
	for _, d := range decls {
		locals := collectLocals(d.Node)
		refs := map[string]bool{}
		imports := map[string]map[string]bool{}
		walkRefs(d.Node, d.FileAlias, locals, topNames, refs, imports)
		d.Refs = sortedKeys(refs)
		if len(imports) > 0 {
			d.Imports = map[string][]string{}
			for k, v := range imports {
				d.Imports[k] = sortedKeys(v)
			}
		}
	}

	// 📤write decls.jsonl
	writeDeclsJSONL(filepath.Join(outDir, "decls.jsonl"), decls)

	// 🧮module aggregation
	modules := buildModules(decls)
	writeJSON(filepath.Join(outDir, "modules.json"), modules)

	// 🧭test function -> module references (step 6)
	testAliasFlat := testAlias
	_ = testAliasFlat
	testRefs := extractTestModuleRefs(fset, testFile, testAlias, topNames, allDeclNames, decls)
	writeJSON(filepath.Join(outDir, "test-module-refs.json"), testRefs)

	// 🔄SCC + DAG violation analysis
	sccOut := analyzeGraph(modules)
	writeJSON(filepath.Join(outDir, "scc-analysis.json"), sccOut)

	fmt.Println("decls:", len(decls))
	fmt.Println("modules:", len(modules))
	fmt.Println("wrote outputs to", outDir)
}

func must(err error) {
	if err != nil {
		panic(err)
	}
}

func mustFindTicketDir() string {
	wd, err := os.Getwd()
	must(err)
	// wd is $TICKET/🔨️go-symbol-graph
	return filepath.Dir(wd)
}

func importAliases(f *ast.File) map[string]string {
	out := map[string]string{}
	for _, imp := range f.Imports {
		path := strings.Trim(imp.Path.Value, `"`)
		var alias string
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

type regionResolver struct {
	NameToModule   map[string]string          `json:"nameToModule"`
	LineOverrides  []lineOverride              `json:"lineOverrides"`
	Conflicts      []string                    `json:"conflicts"`
}

type lineOverride struct {
	Start  int    `json:"start"`
	End    int    `json:"end"`
	Module string `json:"module"`
	Note   string `json:"note"`
}

func loadRegionModules(path string) *regionResolver {
	b, err := os.ReadFile(path)
	if err != nil {
		return &regionResolver{NameToModule: map[string]string{}}
	}
	var r regionResolver
	must(json.Unmarshal(b, &r))
	return &r
}

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

type regionLineFn func(line int) (region string, module string)

func extractDecls(fset *token.FileSet, f *ast.File, fileLabel string, alias map[string]string, resolve regionLineFn) []*decl {
	var out []*decl
	for _, d := range f.Decls {
		switch n := d.(type) {
		case *ast.FuncDecl:
			recv := ""
			kind := "func"
			if n.Recv != nil && len(n.Recv.List) > 0 {
				recv = recvTypeName(n.Recv.List[0].Type)
				kind = "method"
			}
			start := fset.Position(n.Pos()).Line
			end := fset.Position(n.End()).Line
			region, module := resolve(start)
			out = append(out, &decl{
				Name: n.Name.Name, Kind: kind, Receiver: recv, File: fileLabel,
				StartLine: start, EndLine: end, Region: region, Module: module,
				Node: n, FileAlias: alias,
			})
		case *ast.GenDecl:
			for _, spec := range n.Specs {
				switch s := spec.(type) {
				case *ast.TypeSpec:
					start := fset.Position(s.Pos()).Line
					end := fset.Position(s.End()).Line
					region, module := resolve(start)
					out = append(out, &decl{
						Name: s.Name.Name, Kind: "type", File: fileLabel,
						StartLine: start, EndLine: end, Region: region, Module: module,
						Node: s, FileAlias: alias,
					})
				case *ast.ValueSpec:
					kind := "var"
					if n.Tok == token.CONST {
						kind = "const"
					}
					start := fset.Position(s.Pos()).Line
					end := fset.Position(s.End()).Line
					region, module := resolve(start)
					for _, nm := range s.Names {
						if nm.Name == "_" {
							continue
						}
						out = append(out, &decl{
							Name: nm.Name, Kind: kind, File: fileLabel,
							StartLine: start, EndLine: end, Region: region, Module: module,
							Node: s, FileAlias: alias,
						})
					}
				}
			}
		}
	}
	return out
}

// 🧺collectLocals gathers names bound locally within a decl (params/results/receiver, := , var/const/type,
// range vars, type-switch assign, func literal params/results) so they can be excluded from top-level refs.
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
			if fd.Type.TypeParams != nil {
				addFieldList(fd.Type.TypeParams)
			}
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

// 🕸️walkRefs collects references to top-level names and to imported-package selector usage.
func walkRefs(n ast.Node, alias map[string]string, locals, topNames map[string]bool, refs map[string]bool, imports map[string]map[string]bool) {
	var visit func(ast.Node) bool
	visit = func(node ast.Node) bool {
		switch x := node.(type) {
		case *ast.SelectorExpr:
			if id, ok := x.X.(*ast.Ident); ok {
				if pkgPath, isImport := alias[id.Name]; isImport {
					if imports[pkgPath] == nil {
						imports[pkgPath] = map[string]bool{}
					}
					imports[pkgPath][x.Sel.Name] = true
					return false
				}
			}
			ast.Inspect(x.X, visit)
			return false
		case *ast.Ident:
			if topNames[x.Name] && !locals[x.Name] {
				refs[x.Name] = true
			}
		}
		return true
	}
	ast.Inspect(n, visit)
}

func sortedKeys(m map[string]bool) []string {
	out := make([]string, 0, len(m))
	for k := range m {
		out = append(out, k)
	}
	sort.Strings(out)
	return out
}

func writeDeclsJSONL(path string, decls []*decl) {
	f, err := os.Create(path)
	must(err)
	defer f.Close()
	enc := json.NewEncoder(f)
	for _, d := range decls {
		must(enc.Encode(d))
	}
}

func writeJSON(path string, v interface{}) {
	b, err := json.MarshalIndent(v, "", "  ")
	must(err)
	must(os.WriteFile(path, b, 0o644))
}

// 📦moduleInfo aggregates decls per module.
type moduleInfo struct {
	DeclCount  int                 `json:"declCount"`
	Lines      int                 `json:"lines"`
	ImportsUsed map[string][]string `json:"importsUsed,omitempty"`
	EdgesTo    map[string]map[string]bool `json:"-"`
	EdgesToOut map[string][]string `json:"edgesTo"`
}

func buildModules(decls []*decl) map[string]*moduleInfo {
	byKey := map[string]*decl{}
	for _, d := range decls {
		byKey[identKey(d.Receiver, d.Name)] = d
		if d.Kind != "method" {
			byKey[d.Name] = d
		}
	}
	mods := map[string]*moduleInfo{}
	get := func(m string) *moduleInfo {
		if mods[m] == nil {
			mods[m] = &moduleInfo{ImportsUsed: map[string][]string{}, EdgesTo: map[string]map[string]bool{}, EdgesToOut: map[string][]string{}}
		}
		return mods[m]
	}
	for _, d := range decls {
		mi := get(d.Module)
		mi.DeclCount++
		mi.Lines += d.EndLine - d.StartLine + 1
		for pkg, syms := range d.Imports {
			set := map[string]bool{}
			for _, s := range mi.ImportsUsed[pkg] {
				set[s] = true
			}
			for _, s := range syms {
				set[s] = true
			}
			mi.ImportsUsed[pkg] = sortedKeys(set)
		}
		for _, ref := range d.Refs {
			target, ok := byKey[ref]
			if !ok {
				continue
			}
			if target.Module == d.Module {
				continue
			}
			if mi.EdgesTo[target.Module] == nil {
				mi.EdgesTo[target.Module] = map[string]bool{}
			}
			symbolEdge := fmt.Sprintf("%s.%s -> %s.%s", d.Module, identKey(d.Receiver, d.Name), target.Module, ref)
			mi.EdgesTo[target.Module][symbolEdge] = true
		}
	}
	for _, mi := range mods {
		for target, set := range mi.EdgesTo {
			mi.EdgesToOut[target] = sortedKeys(set)
		}
	}
	return mods
}

// 🧪extractTestModuleRefs maps each test func to the modules its referenced names live in.
func extractTestModuleRefs(fset *token.FileSet, f *ast.File, alias map[string]string, topNames map[string]bool, allDeclNames map[string]*decl, decls []*decl) map[string][]string {
	nameToModule := map[string]string{}
	for _, d := range decls {
		if d.Kind != "method" {
			nameToModule[d.Name] = d.Module
		}
	}
	out := map[string][]string{}
	for _, d := range f.Decls {
		fd, ok := d.(*ast.FuncDecl)
		if !ok || !strings.HasPrefix(fd.Name.Name, "Test") {
			continue
		}
		locals := collectLocals(fd)
		refs := map[string]bool{}
		imports := map[string]map[string]bool{}
		walkRefs(fd, alias, locals, topNames, refs, imports)
		modSet := map[string]bool{}
		for r := range refs {
			if m, ok := nameToModule[r]; ok {
				modSet[m] = true
			}
		}
		out[fd.Name.Name] = sortedKeys(modSet)
	}
	_ = fset
	return out
}

// 🧭dagLevel is the ticket-plan §2 dependency level per module (lower may not import higher).
var dagLevel = map[string]int{
	"workspace": 0, "identity": 0, "yaml": 0, "search": 0, "model": 0,
	"languages": 1, "events": 1, "providers": 1,
	"codebase": 2, "statutes": 2, "metrics": 2,
	"tickets": 3, "goals": 3, "contributors": 3, "todos": 3, "tree": 3, "move": 3, "testrunner": 3, "hooks": 3,
	"graphql": 4,
	"mcp": 5, "cli": 5,
	"dashboard": 6,
}

type sccResult struct {
	Cycles     [][]string          `json:"cycles"`
	Violations []violation         `json:"violations"`
	SharedSymbols []sharedSymbol   `json:"sharedSymbolsUsedByOver5Modules"`
}

type violation struct {
	From       string   `json:"from"`
	To         string   `json:"to"`
	FromLevel  int      `json:"fromLevel"`
	ToLevel    int      `json:"toLevel"`
	Symbols    []string `json:"symbols"`
}

type sharedSymbol struct {
	Symbol       string   `json:"symbol"`
	DefinedIn    string   `json:"definedIn"`
	UsedByModules []string `json:"usedByModules"`
}

func analyzeGraph(mods map[string]*moduleInfo) sccResult {
	// Tarjan SCC over module graph
	index := map[string]int{}
	low := map[string]int{}
	onStack := map[string]bool{}
	var stack []string
	counter := 0
	var sccs [][]string
	var names []string
	for m := range mods {
		names = append(names, m)
	}
	sort.Strings(names)
	var strongconnect func(v string)
	strongconnect = func(v string) {
		index[v] = counter
		low[v] = counter
		counter++
		stack = append(stack, v)
		onStack[v] = true
		targets := make([]string, 0)
		for t := range mods[v].EdgesTo {
			targets = append(targets, t)
		}
		sort.Strings(targets)
		for _, w := range targets {
			if mods[w] == nil {
				continue
			}
			if _, seen := index[w]; !seen {
				strongconnect(w)
				if low[w] < low[v] {
					low[v] = low[w]
				}
			} else if onStack[w] {
				if index[w] < low[v] {
					low[v] = index[w]
				}
			}
		}
		if low[v] == index[v] {
			var comp []string
			for {
				n := len(stack) - 1
				w := stack[n]
				stack = stack[:n]
				onStack[w] = false
				comp = append(comp, w)
				if w == v {
					break
				}
			}
			if len(comp) > 1 {
				sort.Strings(comp)
				sccs = append(sccs, comp)
			}
		}
	}
	for _, m := range names {
		if _, seen := index[m]; !seen {
			strongconnect(m)
		}
	}

	var violations []violation
	for _, from := range names {
		fl, ok1 := dagLevel[from]
		if !ok1 {
			continue
		}
		var targets []string
		for t := range mods[from].EdgesTo {
			targets = append(targets, t)
		}
		sort.Strings(targets)
		for _, to := range targets {
			tl, ok2 := dagLevel[to]
			if !ok2 || to == from {
				continue
			}
			if tl > fl {
				violations = append(violations, violation{From: from, To: to, FromLevel: fl, ToLevel: tl, Symbols: mods[from].EdgesToOut[to]})
			}
		}
	}

	// shared symbols: defined in module X, referenced (edge target) from >5 distinct modules
	usedBy := map[string]map[string]bool{} // symbol(module.name) -> set of source modules
	for from, mi := range mods {
		for to, syms := range mi.EdgesToOut {
			for _, s := range syms {
				// symbol edge format: "from.decl -> to.target"
				parts := strings.SplitN(s, " -> ", 2)
				if len(parts) != 2 {
					continue
				}
				target := parts[1]
				if usedBy[target] == nil {
					usedBy[target] = map[string]bool{}
				}
				usedBy[target][from] = true
			}
			_ = to
		}
	}
	var shared []sharedSymbol
	for sym, froms := range usedBy {
		if len(froms) > 5 {
			parts := strings.SplitN(sym, ".", 2)
			def := ""
			if len(parts) == 2 {
				def = parts[0]
			}
			shared = append(shared, sharedSymbol{Symbol: sym, DefinedIn: def, UsedByModules: sortedKeys(froms)})
		}
	}
	sort.Slice(shared, func(i, j int) bool { return len(shared[i].UsedByModules) > len(shared[j].UsedByModules) })

	return sccResult{Cycles: sccs, Violations: violations, SharedSymbols: shared}
}
