package main

import (
	"bufio"
	"fmt"
	"go/ast"
	"go/parser"
	"go/token"
	"os"
	"path"
	"sort"
	"strconv"
	"strings"
)

type decl struct {
	idx        int
	start, end int
	kind       string
	name       string
	recv       string
}

func isRegion(l string) bool {
	t := strings.TrimSpace(l)
	return strings.HasPrefix(t, "// #region") || strings.HasPrefix(t, "// #endregion")
}

func load(file string) ([]string, *ast.File, *token.FileSet, []decl, string) {
	src, err := os.ReadFile(file)
	if err != nil {
		panic(err)
	}
	text := string(src)
	lines := strings.Split(text, "\n")
	fset := token.NewFileSet()
	f, err := parser.ParseFile(fset, file, src, parser.ParseComments)
	if err != nil {
		panic(err)
	}
	var ds []decl
	prevEnd := fset.Position(f.Name.End()).Line
	for _, d := range f.Decls {
		if g, ok := d.(*ast.GenDecl); ok && g.Tok == token.IMPORT {
			prevEnd = fset.Position(g.End()).Line
			continue
		}
		end := fset.Position(d.End()).Line
		start := prevEnd + 1
		for start <= end && (strings.TrimSpace(lines[start-1]) == "" || isRegion(lines[start-1])) {
			start++
		}
		dd := decl{idx: len(ds), start: start, end: end}
		switch x := d.(type) {
		case *ast.FuncDecl:
			dd.kind = "func"
			dd.name = x.Name.Name
			if x.Recv != nil && len(x.Recv.List) > 0 {
				t := x.Recv.List[0].Type
				if s, ok := t.(*ast.StarExpr); ok {
					t = s.X
				}
				if ix, ok := t.(*ast.IndexExpr); ok {
					t = ix.X
				}
				if ix, ok := t.(*ast.IndexListExpr); ok {
					t = ix.X
				}
				dd.recv = t.(*ast.Ident).Name
				dd.kind = "method"
			}
		case *ast.GenDecl:
			dd.kind = x.Tok.String()
			var names []string
			for _, s := range x.Specs {
				switch sp := s.(type) {
				case *ast.TypeSpec:
					names = append(names, sp.Name.Name)
				case *ast.ValueSpec:
					for _, n := range sp.Names {
						names = append(names, n.Name)
					}
				}
			}
			dd.name = strings.Join(names, ",")
		}
		ds = append(ds, dd)
		prevEnd = end
	}
	return lines, f, fset, ds, text
}

func main() {
	cmd := os.Args[1]
	file := os.Args[2]
	lines, f, _, ds, _ := load(file)
	switch cmd {
	case "dump":
		for _, d := range ds {
			n := d.name
			if d.recv != "" {
				n = d.recv + "." + d.name
			}
			if len(n) > 70 {
				n = n[:70] + "…"
			}
			fmt.Printf("%4d %5d-%5d (%4d) %-6s %s\n", d.idx, d.start, d.end, d.end-d.start+1, d.kind, n)
		}
		_ = lines
	case "norm":
		inImp := false
		var out []string
		for _, l := range lines {
			t := strings.TrimSpace(l)
			if inImp {
				if t == ")" {
					inImp = false
				}
				continue
			}
			if strings.HasPrefix(t, "import (") {
				inImp = true
				continue
			}
			if t == "" || isRegion(l) || strings.HasPrefix(t, "package ") || (strings.HasPrefix(t, "import ") && !strings.Contains(t, "(")) {
				continue
			}
			out = append(out, l)
		}
		fmt.Println(strings.Join(out, "\n"))
	case "split":
		spec := os.Args[3]
		outDir := path.Dir(file)
		doSplit(file, spec, outDir, lines, f, ds)
	}
}

type group struct {
	file   string
	region string
	items []string
}

func doSplit(file, specFile, outDir string, lines []string, f *ast.File, ds []decl) {
	fh, err := os.Open(specFile)
	if err != nil {
		panic(err)
	}
	var groups []group
	sc := bufio.NewScanner(fh)
	sc.Buffer(make([]byte, 1<<20), 1<<20)
	for sc.Scan() {
		l := strings.TrimSpace(sc.Text())
		if l == "" || strings.HasPrefix(l, "#") {
			continue
		}
		i := strings.Index(l, ":")
		fl := strings.TrimSpace(l[:i])
		g := group{file: fl}
		if j := strings.Index(fl, "|"); j >= 0 {
			g.file = fl[:j]
			g.region = fl[j+1:]
		}
		g.items = strings.Fields(l[i+1:])
		groups = append(groups, g)
	}
	assign := map[int]int{}
	byName := map[string]int{}
	for i, d := range ds {
		key := d.name
		if d.recv == "" {
			if _, ok := byName[key]; ok && d.kind != "func" {
			}
			byName[key] = i
		}
	}
	for gi, g := range groups {
		for _, it := range g.items {
			if strings.HasPrefix(it, "#") && len(it) > 1 {
				a := strings.Split(it[1:], "-")
				lo, _ := strconv.Atoi(a[0])
				hi := lo
				if len(a) > 1 {
					hi, _ = strconv.Atoi(a[1])
				}
				for k := lo; k <= hi; k++ {
					assign[k] = gi
				}
				continue
			}
			found := false
			for i, d := range ds {
				match := false
				if strings.Contains(it, ".") {
					p := strings.SplitN(it, ".", 2)
					match = d.recv == p[0] && d.name == p[1]
				} else if d.recv == it {
					match = true
				} else if d.recv == "" {
					for _, n := range strings.Split(d.name, ",") {
						if n == it {
							match = true
						}
					}
				}
				if match {
					found = true
					if strings.Contains(it, ".") || d.recv == "" || true {
						if _, ok := assign[i]; ok && assign[i] != gi {
							_ = ok
						}
						assign[i] = gi
					}
				}
			}
			if !found {
				fmt.Fprintf(os.Stderr, "WARN: item %q matched nothing\n", it)
			}
		}
	}
	missing := 0
	for i, d := range ds {
		if _, ok := assign[i]; !ok {
			fmt.Fprintf(os.Stderr, "UNASSIGNED %d %d-%d %s %s.%s\n", i, d.start, d.end, d.kind, d.recv, d.name)
			missing++
		}
	}
	if missing > 0 {
		os.Exit(1)
	}
	var header []string
	var pkgLine int
	for i, l := range lines {
		if strings.HasPrefix(l, "package ") {
			pkgLine = i
			break
		}
	}
	header = lines[:pkgLine+1]
	type imp struct{ name, path, local string }
	var imps []imp
	for _, im := range f.Imports {
		p, _ := strconv.Unquote(im.Path.Value)
		local := path.Base(p)
		name := ""
		if im.Name != nil {
			name = im.Name.Name
			local = name
		}
		imps = append(imps, imp{name, p, local})
	}
	for gi, g := range groups {
		var body []string
		var idxs []int
		for i := range ds {
			if assign[i] == gi {
				idxs = append(idxs, i)
			}
		}
		sort.Ints(idxs)
		for _, i := range idxs {
			d := ds[i]
			body = append(body, lines[d.start-1:d.end]...)
			body = append(body, "")
		}
		bodyText := strings.Join(body, "\n")
		used := map[string]bool{}
		bf, err := parser.ParseFile(token.NewFileSet(), "", "package x\n"+bodyText, 0)
		if err != nil {
			panic(fmt.Sprintf("%s: %v", g.file, err))
		}
		ast.Inspect(bf, func(n ast.Node) bool {
			if s, ok := n.(*ast.SelectorExpr); ok {
				if id, ok := s.X.(*ast.Ident); ok && id.Obj == nil {
					used[id.Name] = true
				}
			}
			return true
		})
		var out strings.Builder
		hdr := header
		if gi != 0 {
			var h2 []string
			for _, l := range header {
				if strings.HasPrefix(l, "//go:build") || strings.HasPrefix(l, "// +build") {
					h2 = append(h2, l)
				}
			}
			if len(h2) > 0 {
				h2 = append(h2, "")
			}
			h2 = append(h2, header[len(header)-1])
			hdr = h2
		}
		out.WriteString(strings.Join(hdr, "\n"))
		out.WriteString("\n\n")
		var std, ext []imp
		for _, im := range imps {
			if used[im.local] {
				if strings.Contains(strings.Split(im.path, "/")[0], ".") {
					ext = append(ext, im)
				} else {
					std = append(std, im)
				}
			}
		}
		if len(std)+len(ext) > 0 {
			out.WriteString("import (\n")
			w := func(l []imp) {
				for _, im := range l {
					if im.name != "" {
						fmt.Fprintf(&out, "\t%s %q\n", im.name, im.path)
					} else {
						fmt.Fprintf(&out, "\t%q\n", im.path)
					}
				}
			}
			w(std)
			if len(std) > 0 && len(ext) > 0 {
				out.WriteString("\n")
			}
			w(ext)
			out.WriteString(")\n\n")
		}
		if g.region != "" {
			fmt.Fprintf(&out, "// #region %s\n\n", g.region)
		}
		out.WriteString(strings.TrimRight(bodyText, "\n"))
		if g.region != "" {
			fmt.Fprintf(&out, "\n\n// #endregion %s", g.region)
		}
		out.WriteString("\n")
		target := outDir + "/" + g.file
		if os.Getenv("DRY") != "" {
			target = os.Getenv("DRY") + "/" + g.file
		}
		if err := os.WriteFile(target, []byte(out.String()), 0o644); err != nil {
			panic(err)
		}
		total := 0
		for _, i := range idxs {
			total += ds[i].end - ds[i].start + 1
		}
		fmt.Printf("%-40s decls=%3d lines~%d\n", g.file, len(idxs), strings.Count(out.String(), "\n"))
	}
}
