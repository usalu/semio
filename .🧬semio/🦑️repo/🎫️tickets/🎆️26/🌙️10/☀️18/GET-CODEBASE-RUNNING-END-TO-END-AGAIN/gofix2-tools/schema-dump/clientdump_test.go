package client

import (
	"fmt"
	"os"
	"sort"
	"strings"
	"testing"

	graphql "github.com/usalu/semio/repo/client/internal/graphql"
)

func tref(t graphql.Type) string {
	switch x := t.(type) {
	case *graphql.NonNull:
		return tref(x.OfType) + "!"
	case *graphql.List:
		return "[" + tref(x.OfType) + "]"
	case *graphql.Scalar:
		return x.Name
	case *graphql.Object:
		return x.Name
	case *graphql.Interface:
		return x.Name
	case *graphql.Union:
		return x.Name
	case *graphql.Enum:
		return x.Name
	case *graphql.InputObject:
		return x.Name
	}
	return "?"
}

func TestZZDumpClientSchema(t *testing.T) {
	schema, err := buildSchema(NewResolver(t.TempDir()))
	if err != nil {
		t.Fatal(err)
	}
	seen := map[string]bool{}
	var out []string
	var visit func(graphql.Type)
	visit = func(ty graphql.Type) {
		switch x := ty.(type) {
		case *graphql.NonNull:
			visit(x.OfType)
		case *graphql.List:
			visit(x.OfType)
		case *graphql.Object:
			if seen[x.Name] {
				return
			}
			seen[x.Name] = true
			names := []string{}
			f := x.Fields()
			for n := range f {
				names = append(names, n)
			}
			sort.Strings(names)
			out = append(out, "object "+x.Name)
			for _, n := range names {
				fld := f[n]
				args := []string{}
				for an, a := range fld.Args {
					args = append(args, an+":"+tref(a.Type))
					visit(a.Type)
				}
				sort.Strings(args)
				out = append(out, fmt.Sprintf("  field %s.%s(%s): %s", x.Name, n, strings.Join(args, ","), tref(fld.Type)))
				visit(fld.Type)
			}
		case *graphql.Interface:
			if seen[x.Name] {
				return
			}
			seen[x.Name] = true
			out = append(out, "interface "+x.Name)
		case *graphql.Union:
			if seen[x.Name] {
				return
			}
			seen[x.Name] = true
			ns := []string{}
			for _, o := range x.Types {
				ns = append(ns, o.Name)
				visit(o)
			}
			sort.Strings(ns)
			out = append(out, "union "+x.Name+" = "+strings.Join(ns, "|"))
		case *graphql.Enum:
			if seen[x.Name] {
				return
			}
			seen[x.Name] = true
			ns := []string{}
			for n := range x.Values {
				ns = append(ns, n)
			}
			sort.Strings(ns)
			out = append(out, "enum "+x.Name+" = "+strings.Join(ns, "|"))
		case *graphql.InputObject:
			if seen[x.Name] {
				return
			}
			seen[x.Name] = true
			ns := []string{}
			for n, f := range x.Fields {
				ns = append(ns, n+":"+tref(f.Type))
				visit(f.Type)
			}
			sort.Strings(ns)
			out = append(out, "input "+x.Name+" = "+strings.Join(ns, ","))
		}
	}
	visit(schema.Query)
	visit(schema.Mutation)
	sort.Strings(out)
	os.WriteFile(os.Getenv("DUMP_OUT"), []byte(strings.Join(out, "\n")+"\n"), 0o644)
}
