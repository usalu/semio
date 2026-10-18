package main

import (
	"fmt"
	"go/ast"
	"go/parser"
	"go/token"
	"os"
	"path/filepath"
	"sort"
	"strings"
)

type declInfo struct {
	key      string
	name     string
	recvType string
	file     string
	line     int
	idents   map[string]bool
	isTest   bool
}

func main() {
	root := os.Args[1]
	roots := map[string]bool{}
	for _, r := range strings.Split(os.Args[2], ",") {
		roots[r] = true
	}
	fset := token.NewFileSet()
	var decls []*declInfo
	filepath.Walk(root, func(p string, info os.FileInfo, err error) error {
		if err != nil {
			return nil
		}
		if info.IsDir() && (info.Name() == "internal" || info.Name() == "📦️packages") {
			return filepath.SkipDir
		}
		if info.IsDir() || !strings.HasSuffix(p, ".go") {
			return nil
		}
		f, err := parser.ParseFile(fset, p, nil, 0)
		if err != nil {
			return nil
		}
		rel, _ := filepath.Rel(root, p)
		isTest := strings.HasPrefix(rel, "🧪️tests")
		collect := func(n ast.Node) map[string]bool {
			m := map[string]bool{}
			ast.Inspect(n, func(x ast.Node) bool {
				if id, ok := x.(*ast.Ident); ok {
					m[id.Name] = true
				}
				return true
			})
			return m
		}
		for _, d := range f.Decls {
			switch x := d.(type) {
			case *ast.FuncDecl:
				di := &declInfo{name: x.Name.Name, file: rel, line: fset.Position(x.Pos()).Line, idents: collect(x), isTest: isTest}
				if x.Recv != nil && len(x.Recv.List) > 0 {
					t := x.Recv.List[0].Type
					if s, ok := t.(*ast.StarExpr); ok {
						t = s.X
					}
					if id, ok := t.(*ast.Ident); ok {
						di.recvType = id.Name
					}
				}
				di.key = fmt.Sprintf("%s:%d %s", rel, di.line, func() string {
					if di.recvType != "" {
						return di.recvType + "." + di.name
					}
					return di.name
				}())
				decls = append(decls, di)
			case *ast.GenDecl:
				for _, s := range x.Specs {
					switch y := s.(type) {
					case *ast.TypeSpec:
						di := &declInfo{name: y.Name.Name, file: rel, line: fset.Position(y.Pos()).Line, idents: collect(y), isTest: isTest}
						di.key = fmt.Sprintf("%s:%d %s", rel, di.line, di.name)
						decls = append(decls, di)
					case *ast.ValueSpec:
						for _, n := range y.Names {
							di := &declInfo{name: n.Name, file: rel, line: fset.Position(y.Pos()).Line, idents: collect(y), isTest: isTest}
							di.key = fmt.Sprintf("%s:%d %s", rel, di.line, di.name)
							decls = append(decls, di)
						}
					}
				}
			}
		}
		return nil
	})
	byName := map[string][]*declInfo{}
	for _, d := range decls {
		byName[d.name] = append(byName[d.name], d)
	}
	reached := map[*declInfo]bool{}
	var queue []*declInfo
	mark := func(d *declInfo) {
		if !reached[d] {
			reached[d] = true
			queue = append(queue, d)
		}
	}
	for _, d := range decls {
		if d.isTest || d.name == "init" || d.name == "_" || roots[d.name] {
			mark(d)
		}
	}
	for len(queue) > 0 {
		d := queue[0]
		queue = queue[1:]
		for id := range d.idents {
			for _, t := range byName[id] {
				mark(t)
			}
		}
		if d.recvType == "" {
			for _, m := range decls {
				if m.recvType == d.name {
					mark(m)
				}
			}
		}
	}
	var dead []string
	for _, d := range decls {
		if !reached[d] && !d.isTest {
			dead = append(dead, d.key)
		}
	}
	sort.Strings(dead)
	for _, k := range dead {
		fmt.Println(k)
	}
}
