package main

import (
	"encoding/json"
	"fmt"
	"go/ast"
	"go/parser"
	"go/token"
	"os"
	"strings"
)

type Decl struct {
	Idx   int    `json:"idx"`
	Kind  string `json:"kind"`
	Name  string `json:"name"`
	Recv  string `json:"recv"`
	Start int    `json:"start"`
	End   int    `json:"end"`
}

func recvName(e ast.Expr) string {
	switch t := e.(type) {
	case *ast.StarExpr:
		return recvName(t.X)
	case *ast.Ident:
		return t.Name
	case *ast.IndexExpr:
		return recvName(t.X)
	case *ast.IndexListExpr:
		return recvName(t.X)
	}
	return "?"
}

func main() {
	fs := token.NewFileSet()
	f, err := parser.ParseFile(fs, os.Args[1], nil, parser.ParseComments)
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
	var out []Decl
	i := 0
	for _, d := range f.Decls {
		var doc *ast.CommentGroup
		dd := Decl{Idx: i}
		switch t := d.(type) {
		case *ast.FuncDecl:
			dd.Kind = "func"
			dd.Name = t.Name.Name
			if t.Recv != nil && len(t.Recv.List) > 0 {
				dd.Recv = recvName(t.Recv.List[0].Type)
				dd.Kind = "method"
			}
			doc = t.Doc
		case *ast.GenDecl:
			if t.Tok == token.IMPORT {
				continue
			}
			dd.Kind = strings.ToLower(t.Tok.String())
			var names []string
			for _, s := range t.Specs {
				switch sp := s.(type) {
				case *ast.TypeSpec:
					names = append(names, sp.Name.Name)
				case *ast.ValueSpec:
					for _, n := range sp.Names {
						names = append(names, n.Name)
					}
				}
			}
			dd.Name = strings.Join(names, ",")
			doc = t.Doc
		}
		s := fs.Position(d.Pos()).Line
		if doc != nil {
			s = fs.Position(doc.Pos()).Line
		}
		dd.Start = s
		dd.End = fs.Position(d.End()).Line
		out = append(out, dd)
		i++
	}
	b, _ := json.MarshalIndent(out, "", " ")
	fmt.Println(string(b))
}
