package main

import (
	"bufio"
	"fmt"
	"go/ast"
	"go/format"
	"go/parser"
	"go/token"
	"os"
	"path/filepath"
	"sort"
	"strconv"
	"strings"
)

type imp struct{ alias, path string }

type outFile struct {
	name, desc string
	ranges     [][2]int
}

func fail(f string, a ...any) {
	fmt.Fprintf(os.Stderr, "ERROR: "+f+"\n", a...)
	os.Exit(1)
}

func normName(n string) string {
	n = strings.ReplaceAll(n, "️", "")
	i := 0
	for i < len(n) {
		r := []rune(n[i:])[0]
		if r < 0x80 {
			break
		}
		i += len(string(r))
	}
	if i == 0 {
		return n
	}
	return n[:i] + "️" + n[i:]
}

func marker(l string) (kind, name string) {
	t := strings.TrimSpace(l)
	for _, p := range []string{"// #region ", "//#region "} {
		if strings.HasPrefix(t, p) {
			return "region", strings.TrimSpace(t[len(p):])
		}
	}
	for _, p := range []string{"// #endregion ", "//#endregion "} {
		if strings.HasPrefix(t, p) {
			return "endregion", strings.TrimSpace(t[len(p):])
		}
	}
	return "", ""
}

func ignored(n string) bool { return strings.Contains(n, "Split") || strings.Contains(n, "Header") }

func main() {
	srcPath, planPath, outDir, style, apply := os.Args[1], os.Args[2], os.Args[3], os.Args[4], os.Args[5] == "apply"
	raw, err := os.ReadFile(srcPath)
	if err != nil {
		fail("%v", err)
	}
	lines := strings.Split(strings.TrimSuffix(string(raw), "\n"), "\n")
	fs := token.NewFileSet()
	file, err := parser.ParseFile(fs, srcPath, raw, parser.ParseComments)
	if err != nil {
		fail("%v", err)
	}
	pkg := file.Name.Name
	var imps []imp
	impEnd := 0
	for _, d := range file.Decls {
		if gd, ok := d.(*ast.GenDecl); ok && gd.Tok == token.IMPORT {
			impEnd = fs.Position(gd.End()).Line
			for _, s := range gd.Specs {
				is := s.(*ast.ImportSpec)
				p, _ := strconv.Unquote(is.Path.Value)
				a := ""
				if is.Name != nil {
					a = is.Name.Name
				}
				imps = append(imps, imp{a, p})
			}
		}
	}
	// stack before each line
	stackAt := make([][]string, len(lines)+2)
	var st []string
	for i, l := range lines {
		ln := i + 1
		stackAt[ln] = append([]string(nil), st...)
		k, n := marker(l)
		if ignored(n) {
			continue
		}
		if k == "region" {
			if len(st) == 0 || st[len(st)-1] != n {
				st = append(st, n)
			}
		} else if k == "endregion" {
			for j := len(st) - 1; j >= 0; j-- {
				if st[j] == n {
					st = st[:j]
					break
				}
			}
		}
	}
	var outs []*outFile
	pf, _ := os.Open(planPath)
	sc := bufio.NewScanner(pf)
	for sc.Scan() {
		t := sc.Text()
		if strings.TrimSpace(t) == "" || strings.HasPrefix(t, "#") {
			continue
		}
		parts := strings.Split(t, "\t")
		if len(parts) != 3 {
			fail("bad plan line %q", t)
		}
		o := &outFile{name: normName(parts[0]), desc: parts[1]}
		for _, r := range strings.Split(parts[2], ",") {
			ab := strings.Split(strings.TrimSpace(r), "-")
			a, _ := strconv.Atoi(ab[0])
			b, _ := strconv.Atoi(ab[len(ab)-1])
			o.ranges = append(o.ranges, [2]int{a, b})
		}
		sort.Slice(o.ranges, func(i, j int) bool { return o.ranges[i][0] < o.ranges[j][0] })
		outs = append(outs, o)
	}
	cover := make([]int, len(lines)+2)
	for _, o := range outs {
		for _, r := range o.ranges {
			if r[0] <= impEnd || r[1] > len(lines) {
				fail("range %v out of bounds (impEnd %d)", r, impEnd)
			}
			for i := r[0]; i <= r[1]; i++ {
				cover[i]++
			}
		}
	}
	bad := false
	for i := impEnd + 1; i <= len(lines); i++ {
		l := strings.TrimSpace(lines[i-1])
		k, _ := marker(lines[i-1])
		if cover[i] > 1 {
			fmt.Fprintf(os.Stderr, "line %d covered %d times\n", i, cover[i])
			bad = true
		}
		if cover[i] == 0 && l != "" && k == "" {
			fmt.Fprintf(os.Stderr, "line %d uncovered: %s\n", i, lines[i-1])
			bad = true
		}
	}
	if bad {
		os.Exit(1)
	}
	for _, o := range outs {
		var body []string
		var fsk []string
		emit := func(s string) { body = append(body, s) }
		for _, r := range o.ranges {
			a, b := r[0], r[1]
			for a <= b {
				k, _ := marker(lines[a-1])
				if strings.TrimSpace(lines[a-1]) == "" || k != "" {
					a++
				} else {
					break
				}
			}
			for b >= a {
				k, _ := marker(lines[b-1])
				if strings.TrimSpace(lines[b-1]) == "" || k != "" {
					b--
				} else {
					break
				}
			}
			if a > b {
				continue
			}
			want := stackAt[a]
			c := 0
			for c < len(fsk) && c < len(want) && fsk[c] == want[c] {
				c++
			}
			for len(fsk) > c {
				emit("")
				emit("// #endregion " + fsk[len(fsk)-1])
				fsk = fsk[:len(fsk)-1]
			}
			for _, w := range want[c:] {
				emit("")
				emit("// #region " + w)
				emit("")
				fsk = append(fsk, w)
			}
			if len(body) > 0 {
				emit("")
			}
			for i := a; i <= b; i++ {
				l := lines[i-1]
				k, n := marker(l)
				if ignored(n) && k != "" {
					continue
				}
				if k == "region" {
					if len(fsk) > 0 && fsk[len(fsk)-1] == n {
						continue
					}
					fsk = append(fsk, n)
				} else if k == "endregion" {
					idx := -1
					for j := len(fsk) - 1; j >= 0; j-- {
						if fsk[j] == n {
							idx = j
							break
						}
					}
					if idx < 0 {
						continue
					}
					for len(fsk)-1 > idx {
						emit("// #endregion " + fsk[len(fsk)-1])
						fsk = fsk[:len(fsk)-1]
					}
					fsk = fsk[:idx]
				}
				emit(l)
			}
		}
		for len(fsk) > 0 {
			emit("")
			emit("// #endregion " + fsk[len(fsk)-1])
			fsk = fsk[:len(fsk)-1]
		}
		bs := strings.Join(body, "\n")
		bf, err := parser.ParseFile(token.NewFileSet(), "", "package "+pkg+"\n"+bs, 0)
		if err != nil {
			fail("%s: parse body: %v", o.name, err)
		}
		used := map[string]bool{}
		ast.Inspect(bf, func(n ast.Node) bool {
			if se, ok := n.(*ast.SelectorExpr); ok {
				if id, ok := se.X.(*ast.Ident); ok && id.Obj == nil {
					used[id.Name] = true
				}
			}
			return true
		})
		var std, ext []string
		for _, im := range imps {
			nm := im.alias
			if nm == "" {
				nm = filepath.Base(im.path)
			}
			if !used[nm] {
				continue
			}
			s := "\t" + strconv.Quote(im.path)
			if im.alias != "" {
				s = "\t" + im.alias + " " + strconv.Quote(im.path)
			}
			if strings.Contains(strings.Split(im.path, "/")[0], ".") {
				ext = append(ext, s)
			} else {
				std = append(std, s)
			}
		}
		var sb strings.Builder
		switch style {
		case "header":
			sb.WriteString("// #region 🧲️Header\n\n// 2026 Ueli Saluz <ueli@semio-tech.com>\n\n// " + o.desc + "\n\n// #endregion 🧲️Header\n\n")
		case "plain":
			sb.WriteString("// " + o.desc + "\n\n")
		case "exh":
			sb.WriteString("//go:build exhaustive\n\n// " + o.desc + "\n\n")
		}
		sb.WriteString("package " + pkg + "\n\n")
		if len(std)+len(ext) > 0 {
			sb.WriteString("import (\n")
			sb.WriteString(strings.Join(std, "\n"))
			if len(std) > 0 && len(ext) > 0 {
				sb.WriteString("\n\n")
			}
			sb.WriteString(strings.Join(ext, "\n"))
			sb.WriteString("\n)\n")
		}
		sb.WriteString(bs)
		sb.WriteString("\n")
		out, err := format.Source([]byte(sb.String()))
		if err != nil {
			fail("%s: format: %v", o.name, err)
		}
		n := len(strings.Split(string(out), "\n")) - 1
		fmt.Printf("%-50s %5d lines\n", o.name, n)
		if apply {
			if err := os.WriteFile(filepath.Join(outDir, o.name), out, 0o644); err != nil {
				fail("%v", err)
			}
		}
	}
}
