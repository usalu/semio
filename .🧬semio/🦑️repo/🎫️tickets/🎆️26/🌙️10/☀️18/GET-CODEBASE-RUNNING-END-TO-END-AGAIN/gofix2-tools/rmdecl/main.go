package main

import (
	"fmt"
	"go/ast"
	"go/parser"
	"go/token"
	"os"
	"sort"
	"strconv"
	"strings"
)

// usage: rmdecl <file> <line>...   removes the top-level decl (or value/type spec) starting at each line
func main() {
	path := os.Args[1]
	var lines []int
	for _, a := range os.Args[2:] {
		n, _ := strconv.Atoi(a)
		lines = append(lines, n)
	}
	src, _ := os.ReadFile(path)
	fset := token.NewFileSet()
	f, err := parser.ParseFile(fset, path, src, parser.ParseComments)
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
	type cut struct{ a, b int }
	var cuts []cut
	want := map[int]bool{}
	for _, l := range lines {
		want[l] = true
	}
	off := func(p token.Pos) int { return fset.Position(p).Offset }
	for _, d := range f.Decls {
		switch x := d.(type) {
		case *ast.FuncDecl:
			if want[fset.Position(x.Pos()).Line] {
				start := x.Pos()
				if x.Doc != nil {
					start = x.Doc.Pos()
				}
				cuts = append(cuts, cut{off(start), off(x.End())})
			}
		case *ast.GenDecl:
			if len(x.Specs) == 1 && !x.Lparen.IsValid() {
				if want[fset.Position(x.Pos()).Line] || want[fset.Position(x.Specs[0].Pos()).Line] {
					start := x.Pos()
					if x.Doc != nil {
						start = x.Doc.Pos()
					}
					cuts = append(cuts, cut{off(start), off(x.End())})
				}
				continue
			}
			var keep []ast.Spec
			var rem []ast.Spec
			for _, s := range x.Specs {
				if want[fset.Position(s.Pos()).Line] {
					rem = append(rem, s)
				} else {
					keep = append(keep, s)
				}
			}
			if len(rem) == 0 {
				continue
			}
			if len(keep) == 0 {
				start := x.Pos()
				if x.Doc != nil {
					start = x.Doc.Pos()
				}
				cuts = append(cuts, cut{off(start), off(x.End())})
				continue
			}
			for _, s := range rem {
				var doc *ast.CommentGroup
				switch y := s.(type) {
				case *ast.ValueSpec:
					doc = y.Doc
				case *ast.TypeSpec:
					doc = y.Doc
				}
				start := s.Pos()
				if doc != nil {
					start = doc.Pos()
				}
				cuts = append(cuts, cut{off(start), off(s.End())})
			}
		}
	}
	sort.Slice(cuts, func(i, j int) bool { return cuts[i].a > cuts[j].a })
	out := string(src)
	for _, c := range cuts {
		a, b := c.a, c.b
		for a > 0 && (out[a-1] == ' ' || out[a-1] == '\t') {
			a--
		}
		for b < len(out) && out[b] != '\n' {
			b++
		}
		if b < len(out) {
			b++
		}
		for b < len(out) && out[b] == '\n' && a > 0 && out[a-1] == '\n' && (a < 2 || out[a-2] == '\n') {
			b++
		}
		out = out[:a] + out[b:]
	}
	out = strings.TrimRight(out, "\n") + "\n"
	os.WriteFile(path, []byte(out), 0o644)
	fmt.Printf("%s: removed %d\n", path, len(cuts))
}
