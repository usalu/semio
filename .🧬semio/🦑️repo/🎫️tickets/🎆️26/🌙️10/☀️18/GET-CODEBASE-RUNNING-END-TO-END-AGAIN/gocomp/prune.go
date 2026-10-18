package main

import (
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

func main() {
	for _, p := range os.Args[1:] {
		src, err := os.ReadFile(p)
		if err != nil {
			panic(err)
		}
		fs := token.NewFileSet()
		f, err := parser.ParseFile(fs, p, src, parser.ParseComments)
		if err != nil {
			fmt.Fprintln(os.Stderr, p, err)
			os.Exit(1)
		}
		used := map[string]bool{}
		ast.Inspect(f, func(n ast.Node) bool {
			if s, ok := n.(*ast.SelectorExpr); ok {
				if id, ok := s.X.(*ast.Ident); ok && id.Obj == nil {
					used[id.Name] = true
				}
			}
			return true
		})
		var imp *ast.GenDecl
		for _, d := range f.Decls {
			if g, ok := d.(*ast.GenDecl); ok && g.Tok == token.IMPORT {
				imp = g
				break
			}
		}
		if imp == nil {
			continue
		}
		type kp struct{ path, line string }
		var kps []kp
		for _, s := range imp.Specs {
			is := s.(*ast.ImportSpec)
			ip, _ := strconv.Unquote(is.Path.Value)
			name := path.Base(ip)
			if is.Name != nil {
				name = is.Name.Name
			}
			if name == "_" || name == "." || used[name] {
				line := "\t" + is.Path.Value
				if is.Name != nil {
					line = "\t" + is.Name.Name + " " + is.Path.Value
				}
				kps = append(kps, kp{ip, line})
			}
		}
		sort.Slice(kps, func(i, j int) bool { return kps[i].path < kps[j].path })
		var keep []string
		for _, k := range kps {
			keep = append(keep, k.line)
		}
		start := fs.Position(imp.Pos()).Offset
		end := fs.Position(imp.End()).Offset
		var block string
		if len(keep) > 0 {
			block = "import (\n" + strings.Join(keep, "\n") + "\n)"
		}
		out := string(src[:start]) + block + string(src[end:])
		if err := os.WriteFile(p, []byte(out), 0644); err != nil {
			panic(err)
		}
	}
}
