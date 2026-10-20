package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"go/ast"
	"go/parser"
	"go/token"
	"os"
	"strings"
)

type Doc struct {
	File   string `json:"file"`
	Line   int    `json:"line"`
	Offset int    `json:"offset"`
	Text   string `json:"text"`
	Decl   string `json:"decl"`
	Raw    string `json:"raw"`
}

func first(cg *ast.CommentGroup) *ast.Comment {
	if cg == nil || len(cg.List) == 0 {
		return nil
	}
	return cg.List[0]
}

func main() {
	var docs []Doc
	sc := bufio.NewScanner(os.Stdin)
	sc.Buffer(make([]byte, 1<<20), 1<<20)
	for sc.Scan() {
		path := strings.TrimSpace(sc.Text())
		if path == "" {
			continue
		}
		fset := token.NewFileSet()
		f, err := parser.ParseFile(fset, path, nil, parser.ParseComments)
		if err != nil {
			fmt.Fprintln(os.Stderr, "ERR", path, err)
			continue
		}
		add := func(cg *ast.CommentGroup, decl string) {
			if cg == nil {
				return
			}
			var c *ast.Comment
			for _, x := range cg.List {
				t := x.Text
				if strings.HasPrefix(t, "//go:") || strings.HasPrefix(t, "// #region") || strings.HasPrefix(t, "// #endregion") || strings.HasPrefix(t, "//nolint") || strings.HasPrefix(t, "//line ") || strings.HasPrefix(t, "//export ") {
					continue
				}
				if strings.HasPrefix(t, "/*") {
					continue
				}
				if strings.TrimSpace(strings.TrimPrefix(t, "//")) == "" {
					continue
				}
				c = x
				break
			}
			if c == nil {
				return
			}
			p := fset.Position(c.Slash)
			docs = append(docs, Doc{File: path, Line: p.Line, Offset: p.Offset, Text: strings.TrimPrefix(c.Text, "//"), Decl: decl, Raw: c.Text})
		}
		if f.Doc != nil {
			add(f.Doc, "package "+f.Name.Name)
		}
		ast.Inspect(f, func(n ast.Node) bool {
			switch d := n.(type) {
			case *ast.FuncDecl:
				add(d.Doc, "func "+d.Name.Name)
			case *ast.GenDecl:
				if d.Doc != nil && !(len(d.Specs) == 1 && specDoc(d.Specs[0]) != nil) {
					name := ""
					if len(d.Specs) > 0 {
						name = d.Tok.String() + " " + specName(d.Specs[0])
					}
					add(d.Doc, name)
				}
			case *ast.TypeSpec:
				add(d.Doc, "type "+d.Name.Name)
			case *ast.ValueSpec:
				add(d.Doc, "value "+d.Names[0].Name)
			case *ast.Field:
				nm := ""
				if len(d.Names) > 0 {
					nm = d.Names[0].Name
				}
				add(d.Doc, "field "+nm)
			}
			return true
		})
	}
	json.NewEncoder(os.Stdout).Encode(docs)
}

func specDoc(s ast.Spec) *ast.CommentGroup {
	switch x := s.(type) {
	case *ast.TypeSpec:
		return x.Doc
	case *ast.ValueSpec:
		return x.Doc
	}
	return nil
}

func specName(s ast.Spec) string {
	switch x := s.(type) {
	case *ast.TypeSpec:
		return x.Name.Name
	case *ast.ValueSpec:
		return x.Names[0].Name
	}
	return ""
}
