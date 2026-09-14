// 🏗️gosplit splits the repo godfile into the domain Go packages of the taxonomy-tree plan.
package main

import (
	"encoding/json"
	"fmt"
	"go/ast"
	"go/parser"
	"go/token"
	"os"
	"path/filepath"
	"sort"
	"strings"
)

// 🎛️config is the hand-authored input that steers the split.
type config struct {
	Symbols        map[string]string `json:"symbols"`
	Prefixes       []prefixRule      `json:"prefixes"`
	Regions        map[string]string `json:"regions"`
	LineOverrides  []lineOverride    `json:"lineOverrides"`
	ForceUnchanged []string          `json:"forceUnexported"`
	Drop           []string          `json:"drop"`
	Renames        map[string]string `json:"renames"`
	TestOverrides  map[string]string `json:"testOverrides"`
}

// 🔠️prefixRule routes every declaration whose name carries a prefix to one module.
type prefixRule struct {
	Prefix  string   `json:"prefix"`
	Module  string   `json:"module"`
	Kinds   []string `json:"kinds"`
	Exclude []string `json:"exclude"`
}

// ✅️matches answers whether a declaration falls under the rule.
func (r prefixRule) matches(d *decl) bool {
	if !strings.HasPrefix(d.Name, r.Prefix) {
		return false
	}
	for _, e := range r.Exclude {
		if d.Name == e {
			return false
		}
	}
	if len(r.Kinds) == 0 {
		return true
	}
	for _, k := range r.Kinds {
		if d.Kind == k {
			return true
		}
	}
	return false
}

// 🧭️levels is the plan §2 dependency ladder; lower may not import higher.
// The fractional ranks inside a plan level are the intra-level order Go's ban on import cycles
// forces; the integer part is the plan §2 level.
var levels = map[string]float64{
	"yaml": 0.0, "search": 0.0, "identity": 0.1, "workspace": 0.2, "model": 0.3,
	"languages": 1.0, "events": 1.1, "providers": 1.2,
	"statutes": 2.0, "codebase": 2.1, "metrics": 2.2,
	"move": 3.0, "contributors": 3.1, "goals": 3.2, "tickets": 3.3, "todos": 3.4,
	"testrunner": 3.5, "hooks": 3.6, "tree": 3.7,
	"graphql": 4.0,
	"mcp": 5.0, "cli": 5.1,
	"dashboard": 6.0,
}

// 📁️modulePaths maps a module key to its folder under 🔨️modules.
var modulePaths = map[string]string{
	"workspace": "🏠️workspace", "identity": "🪪️identity", "yaml": "🧾️yaml", "search": "🔎️search",
	"model": "📐️model", "languages": "🗣️languages", "events": "📡️events", "providers": "🧩️providers",
	"codebase": "🗂️codebase", "statutes": "📜️statutes", "metrics": "📊️metrics",
	"tickets": "🎫️tickets", "goals": "🎯️goals", "contributors": "🧑️contributors", "todos": "📝️todos",
	"tree": "🌳️tree", "move": "🚚️move", "testrunner": "🏃️test-runner", "hooks": "🪝️hooks",
	"graphql": "🔗️graphql", "mcp": "🔌️mcp", "cli": "⌨️cli",
}

// 📛️packageNames maps a module key to its Go package name.
var packageNames = map[string]string{
	"workspace": "workspace", "identity": "identity", "yaml": "yaml", "search": "search",
	"model": "model", "languages": "languages", "events": "events", "providers": "providers",
	"codebase": "codebase", "statutes": "statutes", "metrics": "metrics",
	"tickets": "tickets", "goals": "goals", "contributors": "contributors", "todos": "todos",
	"tree": "tree", "move": "move", "testrunner": "testrunner", "hooks": "hooks",
	"graphql": "graphql", "mcp": "mcp", "cli": "cli",
}

// 📏️internalLineBase pushes the `internal/…` helper packages past the godfile's own line numbers so
// the emitted order stays deterministic and the helper regions land in one block at the end.
const internalLineBase = 100000

// 📏️internalLineSpan is the per-file stride inside that reserved range.
const internalLineSpan = 10000

var ticketDir string
var repoRoot string

// 🗂️byFile indexes every declaration by its origin file, so two same-named declarations coming from
// different source packages (`client.Command` vs `internal/command.Command`) stay distinct.
var byFile = map[string]map[string]*decl{}

// 🔎️lookup resolves a plain identifier of one source file, file-scope first, package-scope second.
func lookup(src *sourceFile, name string, byName map[string]*decl) (*decl, bool) {
	if src != nil {
		if d, ok := byFile[src.Path][name]; ok {
			return d, true
		}
	}
	d, ok := byName[name]
	return d, ok
}

func main() {
	wd, err := os.Getwd()
	must(err)
	ticketDir = filepath.Dir(wd)
	repoRoot = findRepoRoot(ticketDir)

	cfg := loadConfig(filepath.Join(wd, "🔣️symbol-overrides.json"))
	region := loadRegionModules(filepath.Join(ticketDir, "🔨️go-symbol-graph", "🔣️region-modules.json"))
	for k, v := range cfg.Regions {
		region.NameToModule[k] = v
	}
	region.LineOverrides = append(cfg.LineOverrides, region.LineOverrides...)

	clientDir := filepath.Join(repoRoot, "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "💻️client", "⌨️cli")
	comp := parseSource(filepath.Join(clientDir, "🧩️component.go"))
	events := parseSource(filepath.Join(clientDir, "📤️event_export.go"))
	tests := parseSource(filepath.Join(clientDir, "🔬️component_test.go"))
	contract := parseSource(filepath.Join(clientDir, "🤝️g1_contract_test.go"))
	commandTest := parseSource(filepath.Join(clientDir, "internal", "command", "🧪️command_test.go"))

	bps := buildRegionBreakpoints(strings.Split(string(comp.Src), "\n"))
	decls := extractDecls(comp, func(line int) (string, string) {
		r := regionAt(bps, line)
		return r, region.resolve(r, line)
	})
	decls = append(decls, extractDecls(events, func(line int) (string, string) {
		return "📤️Event Export", "cli"
	})...)
	internals := []struct{ Rel, Region string }{
		{"internal/command/🎮️command.go", "🎮️Command Framework"},
		{"internal/templatefunc/🪄️templatefunc.go", "🪄️Template Functions"},
		{"internal/mcp/🔌️mcp.go", "🔌️Mcp Builders"},
		{"internal/mcpserver/🖥️server.go", "🖥️Mcp Server Adapter"},
	}
	for i, in := range internals {
		src := parseSource(filepath.Join(clientDir, filepath.FromSlash(in.Rel)))
		label := in.Region
		base := internalLineBase + i*internalLineSpan
		for _, d := range extractDecls(src, func(int) (string, string) { return label, "cli" }) {
			d.StartLine += base
			d.EndLine += base
			decls = append(decls, d)
		}
	}

	byName := map[string]*decl{}
	byKey := map[string]*decl{}
	topNames := map[string]bool{}
	for _, d := range decls {
		if d.Name == "init" && d.Kind == "func" {
			continue
		}
		byKey[identKey(d.Receiver, d.Name)] = d
		if d.Kind != "method" {
			topNames[d.Name] = true
			if _, taken := byName[d.Name]; !taken {
				byName[d.Name] = d
			}
			if byFile[d.Origin.Path] == nil {
				byFile[d.Origin.Path] = map[string]*decl{}
			}
			byFile[d.Origin.Path][d.Name] = d
		}
	}

	// 🧪️the internal package's own test file shares that package's namespace, not the godfile's.
	byFile[commandTest.Path] = byFile[filepath.Join(clientDir, "internal", "command", "🎮️command.go")]

	// 🎯️symbol overrides win over the region table.
	for _, d := range decls {
		for _, r := range cfg.Prefixes {
			if r.matches(d) {
				d.Module = r.Module
			}
		}
		if m, ok := cfg.Symbols[identKey(d.Receiver, d.Name)]; ok {
			d.Module = m
		} else if d.Kind != "method" {
			if m, ok := cfg.Symbols[d.Name]; ok {
				d.Module = m
			}
		}
	}
	// 🩹️methods must live with their receiver type; Go has no other option.
	movedMethods := 0
	for _, d := range decls {
		if d.Kind != "method" {
			continue
		}
		owner, ok := lookup(d.Origin, d.Receiver, byName)
		if !ok || owner.Kind != "type" {
			continue
		}
		if owner.Module != d.Module {
			d.Module = owner.Module
			movedMethods++
		}
	}

	for _, d := range decls {
		d.Refs, d.Imports = declRefs(d, topNames)
	}

	mods := buildModules(decls, byKey, byName)
	outDir := filepath.Join(ticketDir, "🗑️generated", "go-split")
	must(os.MkdirAll(outDir, 0o755))
	writeJSON(filepath.Join(outDir, "modules.json"), mods)
	dumpDecls(filepath.Join(outDir, "decls.jsonl"), decls)
	scc := analyze(mods)
	writeJSON(filepath.Join(outDir, "scc-analysis.json"), scc)

	unmapped := 0
	for _, d := range decls {
		if d.Module == "UNMAPPED" {
			unmapped++
		}
	}
	fmt.Printf("decls=%d methodsRehomed=%d unmapped=%d violations=%d cycles=%d\n",
		len(decls), movedMethods, unmapped, len(scc.Violations), len(scc.Cycles))
	total := 0
	for _, v := range scc.Violations {
		total += len(v.Symbols)
	}
	fmt.Printf("violating symbol edges=%d\n", total)
	for _, v := range scc.Violations {
		fmt.Printf("  %-12s -> %-12s L%.1f->L%.1f  %d\n", v.From, v.To, v.FromLevel, v.ToLevel, len(v.Symbols))
	}
	for _, c := range scc.Cycles {
		fmt.Println("  CYCLE:", strings.Join(c, ", "))
	}

	if len(os.Args) > 1 && os.Args[1] == "emit" {
		emit(cfg, decls, byName, byKey, topNames, []*sourceFile{tests, contract, commandTest})
	}
}

// 🌱️findRepoRoot walks up until it sees the repo marker directory.
func findRepoRoot(start string) string {
	dir := start
	for {
		if _, err := os.Stat(filepath.Join(dir, "go.work")); err == nil {
			return dir
		}
		parent := filepath.Dir(dir)
		if parent == dir {
			panic("repo root not found")
		}
		dir = parent
	}
}

// 📥️parseSource reads and parses one Go file with its comments.
func parseSource(path string) *sourceFile {
	src, err := os.ReadFile(path)
	must(err)
	fset := token.NewFileSet()
	f, err := parser.ParseFile(fset, path, src, parser.ParseComments)
	must(err)
	return &sourceFile{Label: filepath.Base(path), Path: path, Src: src, File: f, Alias: importAliases(f), Fset: fset}
}

// 📥️loadConfig reads the hand-authored override file.
func loadConfig(path string) *config {
	cfg := &config{Symbols: map[string]string{}, Regions: map[string]string{}, Renames: map[string]string{}, TestOverrides: map[string]string{}}
	b, err := os.ReadFile(path)
	if err != nil {
		return cfg
	}
	must(json.Unmarshal(b, cfg))
	if cfg.Symbols == nil {
		cfg.Symbols = map[string]string{}
	}
	if cfg.Renames == nil {
		cfg.Renames = map[string]string{}
	}
	if cfg.TestOverrides == nil {
		cfg.TestOverrides = map[string]string{}
	}
	return cfg
}

// 📦️moduleInfo aggregates declarations per module.
type moduleInfo struct {
	DeclCount  int                        `json:"declCount"`
	Lines      int                        `json:"lines"`
	EdgesTo    map[string]map[string]bool `json:"-"`
	EdgesToOut map[string][]string        `json:"edgesTo"`
}

// 🧮️buildModules aggregates the module graph from the assigned declarations.
func buildModules(decls []*decl, byKey, byName map[string]*decl) map[string]*moduleInfo {
	pathOf := map[string]string{
		"github.com/usalu/semio/repo/client/internal/command":      "cli",
		"github.com/usalu/semio/repo/client/internal/templatefunc": "model",
		"github.com/usalu/semio/repo/client/internal/mcp":          "cli",
		"github.com/usalu/semio/repo/client/internal/mcpserver":    "cli",
	}
	for m := range packageNames {
		pathOf["github.com/usalu/semio/repo/"+packageNames[m]] = m
	}
	mods := map[string]*moduleInfo{}
	get := func(m string) *moduleInfo {
		if mods[m] == nil {
			mods[m] = &moduleInfo{EdgesTo: map[string]map[string]bool{}, EdgesToOut: map[string][]string{}}
		}
		return mods[m]
	}
	for _, d := range decls {
		mi := get(d.Module)
		mi.DeclCount++
		mi.Lines += d.EndLine - d.StartLine + 1
		for path, syms := range d.Imports {
			target, ok := pathOf[path]
			if !ok || target == d.Module {
				continue
			}
			if mi.EdgesTo[target] == nil {
				mi.EdgesTo[target] = map[string]bool{}
			}
			for _, sym := range syms {
				mi.EdgesTo[target][fmt.Sprintf("%s -> import %s", identKey(d.Receiver, d.Name), sym)] = true
			}
		}
		for _, ref := range d.Refs {
			target, ok := lookup(d.Origin, ref, byName)
			if !ok || target.Module == d.Module {
				continue
			}
			if mi.EdgesTo[target.Module] == nil {
				mi.EdgesTo[target.Module] = map[string]bool{}
			}
			mi.EdgesTo[target.Module][fmt.Sprintf("%s -> %s", identKey(d.Receiver, d.Name), ref)] = true
		}
	}
	for _, mi := range mods {
		for target, set := range mi.EdgesTo {
			mi.EdgesToOut[target] = sortedKeys(set)
		}
	}
	return mods
}

// 🚨️violation is one edge that points up the dependency ladder.
type violation struct {
	From      string   `json:"from"`
	To        string   `json:"to"`
	FromLevel float64  `json:"fromLevel"`
	ToLevel   float64  `json:"toLevel"`
	Symbols   []string `json:"symbols"`
}

// 📊️sccResult is the DAG verdict for one split.
type sccResult struct {
	Cycles     [][]string  `json:"cycles"`
	Violations []violation `json:"violations"`
}

// 🔄️analyze finds cycles and level violations in the module graph.
func analyze(mods map[string]*moduleInfo) sccResult {
	var names []string
	for m := range mods {
		names = append(names, m)
	}
	sort.Strings(names)
	index, low, onStack := map[string]int{}, map[string]int{}, map[string]bool{}
	var stack []string
	counter := 0
	var sccs [][]string
	var strongconnect func(string)
	strongconnect = func(v string) {
		index[v], low[v] = counter, counter
		counter++
		stack = append(stack, v)
		onStack[v] = true
		var targets []string
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
			} else if onStack[w] && index[w] < low[v] {
				low[v] = index[w]
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
	var vs []violation
	for _, from := range names {
		fl, ok := levels[from]
		if !ok {
			continue
		}
		var targets []string
		for t := range mods[from].EdgesTo {
			targets = append(targets, t)
		}
		sort.Strings(targets)
		for _, to := range targets {
			tl, ok := levels[to]
			if !ok || to == from || tl <= fl {
				continue
			}
			vs = append(vs, violation{From: from, To: to, FromLevel: fl, ToLevel: tl, Symbols: mods[from].EdgesToOut[to]})
		}
	}
	sort.Slice(vs, func(i, j int) bool { return len(vs[i].Symbols) > len(vs[j].Symbols) })
	if sccs == nil {
		sccs = [][]string{}
	}
	if vs == nil {
		vs = []violation{}
	}
	return sccResult{Cycles: sccs, Violations: vs}
}

// 🧾️dumpDecls writes one JSON row per assigned declaration.
func dumpDecls(path string, decls []*decl) {
	f, err := os.Create(path)
	must(err)
	defer f.Close()
	enc := json.NewEncoder(f)
	for _, d := range decls {
		must(enc.Encode(d))
	}
}

// 💾️writeJSON writes indented JSON.
func writeJSON(path string, v interface{}) {
	b, err := json.MarshalIndent(v, "", "  ")
	must(err)
	must(os.WriteFile(path, b, 0o644))
}

var _ = ast.Inspect
