// 🧹️prune drops the imports a written file does not use and formats the result.
package main

import (
	"go/ast"
	"go/format"
	"go/parser"
	"go/token"
	"os"
	"path/filepath"
	"strings"
)

// 🧹️pruneAndFormat rewrites one Go file without its unused imports, gofmt-formatted.
func pruneAndFormat(path string) error {
	raw, err := os.ReadFile(path)
	if err != nil {
		return err
	}
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, filepath.Base(path), raw, parser.ParseComments)
	if err != nil {
		return err
	}
	used := map[string]bool{}
	ast.Inspect(file, func(n ast.Node) bool {
		if sel, ok := n.(*ast.SelectorExpr); ok {
			if id, ok := sel.X.(*ast.Ident); ok {
				used[id.Name] = true
			}
		}
		return true
	})
	for _, decl := range file.Decls {
		gd, ok := decl.(*ast.GenDecl)
		if !ok || gd.Tok != token.IMPORT {
			continue
		}
		kept := gd.Specs[:0]
		for _, spec := range gd.Specs {
			imp := spec.(*ast.ImportSpec)
			name := canonicalAlias(strings.Trim(imp.Path.Value, `"`))
			if imp.Name != nil {
				name = imp.Name.Name
			}
			if used[name] || name == "_" || name == "." {
				kept = append(kept, spec)
			}
		}
		gd.Specs = kept
		if len(kept) == 0 {
			gd.Lparen = token.NoPos
			gd.Rparen = token.NoPos
		}
	}
	var out strings.Builder
	if err := format.Node(&out, fset, file); err != nil {
		return err
	}
	text := out.String()
	if len(gdEmpty(file)) > 0 {
		text = strings.Replace(text, "import ()\n\n", "", 1)
		text = strings.Replace(text, "import ()\n", "", 1)
	}
	return writeFile(path, text)
}

// 🫙️gdEmpty reports the import declarations that ended up empty.
func gdEmpty(file *ast.File) []*ast.GenDecl {
	var out []*ast.GenDecl
	for _, decl := range file.Decls {
		if gd, ok := decl.(*ast.GenDecl); ok && gd.Tok == token.IMPORT && len(gd.Specs) == 0 {
			out = append(out, gd)
		}
	}
	return out
}

// 🧽️pruneModule cleans every Go file of one package directory.
func pruneModule(dir string) {
	entries, err := os.ReadDir(dir)
	if err != nil {
		return
	}
	for _, e := range entries {
		if e.IsDir() || !strings.HasSuffix(e.Name(), ".go") {
			continue
		}
		if err := pruneAndFormat(filepath.Join(dir, e.Name())); err != nil {
			println("[prune]", filepath.Join(dir, e.Name()), err.Error())
		}
	}
}
