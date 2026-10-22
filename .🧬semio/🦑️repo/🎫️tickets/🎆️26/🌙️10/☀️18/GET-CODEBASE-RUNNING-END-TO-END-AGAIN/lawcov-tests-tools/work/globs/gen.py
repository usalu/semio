"""📝 Authoring aid: writes the glob dialect corpus (corpus.json): the three glob compilers of the build (workspace units, parity paths, input precision) and the paths each selects."""
import json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"
ALL = ["ws", "parity", "input"]
rows = []


def row(pattern, engines, matches, rejects, mini="agrees", note=None):
    rows.append({"pattern": pattern, "engines": engines, "matches": matches, "rejects": rejects, "minimatch": mini, **({"note": note} if note else {})})


row("*.ts", ALL, ["a.ts", ".ts", "x.y.ts"], ["a/b.ts", "a.tsx", "ts", "a.ts/b"])
row("a/*.ts", ALL, ["a/b.ts", "a/.ts"], ["a/b/c.ts", "b/b.ts", "a.ts", "xa/b.ts"])
row("a/**", ALL, ["a/b", "a/b/c", "a/.x/y"], ["a", "b/a/c", "ab/c"])
row("a/**/b", ALL, ["a/b", "a/x/b", "a/x/y/b", "a/.x/b"], ["a/bb", "a/b/c", "ab", "b", "x/a/b"])
row("**/b", ALL, ["b", "x/b", "x/y/b", ".x/b"], ["bb", "b/c", "xb", "x/bb"])
row("**", ALL, ["a", "a/b/c", ".x", "é/ü"], [])
row("**/*.ts", ALL, ["a.ts", "x/a.ts", "x/y/a.ts", "x/.ts"], ["a.tsx", "x/a.js", "a.ts/b"])
row("*", ALL, ["a", ".x", "é"], ["a/b"])
row("*/x", ALL, ["a/x", ".h/x"], ["x", "a/b/x", "a/xx"])
row("a/*/b", ALL, ["a/x/b"], ["a/b", "a/x/y/b"])
row(".x/*", ALL, [".x/y", ".x/.y"], [".x/y/z", "x/y", ".x"])
row("a/b", ALL, ["a/b"], ["a/b/c", "x/a/b", "a/bb", "a", "A/b"])
row("a.b+c", ALL, ["a.b+c"], ["aXb+c", "a.bbc", "a.b+cc", "a.bc"])
row("(a)", ALL, ["(a)"], ["a"])
row("a|b", ALL, ["a|b"], ["a", "b"])
row("$x", ALL, ["$x"], ["x", ""])
row("^a$", ALL, ["^a$"], ["a"])
row("é/*.ts", ALL, ["é/a.ts"], ["e/a.ts", "é/a/b.ts"])
row("📜️script.ts", ALL, ["📜️script.ts"], ["📜script.ts", "x/📜️script.ts"])
row("**/📜️script.ts", ALL, ["📜️script.ts", "p/📜️script.ts", "p/q/📜️script.ts"], ["📜script.ts", "p/📜️script.tsx"])
row("a/**", ALL, ["a/b\nc", "a/b/c\nd"], ["a\nb/c"], note="a path with a line break is still a path")
row("*", ALL, ["a\nb", "\n"], ["a\n/b"], note="a path with a line break is still a path")
row("a?b", ["parity", "input"], ["aXb", "a?b", "aéb", "a.b"], ["ab", "a/b", "aXXb"])
row("a?b", ["parity", "input"], ["a😀b"], ["a😀😀b"], mini="differs", note="a question mark stands for one code point")
row("a?b", ["ws"], ["a?b"], ["aXb", "a.b"], mini="differs", note="the workspace unit compiler has no question mark wildcard")
row("a/?", ["parity", "input"], ["a/b"], ["a", "a/", "a/bc", "a/b/c"])
row("{a,b}/c", ["input"], ["a/c", "b/c"], ["c/c", "a/b/c", "{a,b}/c", "ab/c"])
row("{a,b}/c", ["ws", "parity"], ["{a,b}/c"], ["a/c", "b/c"], mini="differs", note="only the input compiler expands braces")
row("x{a,b}y{c,d}z", ["input"], ["xayc z".replace(" ", ""), "xaydz", "xbycz", "xbydz"], ["xyz", "xabycz", "xay,cz"])
row("{a,{b,c}}/x", ["input"], ["a/x", "b/x", "c/x"], ["d/x", "{b,c}/x", "a,b/x"])
row("a,b", ALL, ["a,b"], ["a", "b"])
row("a}", ALL, ["a}"], ["a"])
row("{a}", ["input"], ["a"], ["{a}", ""], mini="differs", note="a single member group is a group")
row("x{}", ["input"], ["x"], ["x{}"], mini="differs", note="an empty brace group selects nothing extra")
row("{a,b}", ["input"], ["a", "b"], ["ab", "a,b"])
row("a/{b,c}.ts", ["input"], ["a/b.ts", "a/c.ts"], ["a/d.ts", "a/b/c.ts", "a/b,c.ts"])
row("a/[bc].ts", ALL, ["a/[bc].ts"], ["a/b.ts", "a/c.ts"], mini="differs", note="character classes are literal")
row("a\\b", ALL, ["a\\b"], ["ab", "a/b"], mini="differs", note="a backslash is literal, not an escape")
row("a**b", ALL, ["ab", "a/b", "axxb", "a/x/b"], ["a", "b", "ba"], mini="differs", note="a double star inside a segment still crosses directories")
row("a/b**", ALL, ["a/b", "a/bc", "a/b/c"], ["a/", "b", "a"], mini="differs", note="a double star inside a segment still crosses directories")
row("***", ALL, ["a", "a/b", ""], [], mini="differs", note="three stars are a double star and a star")
row("**/**", ALL, ["a", "a/b/c"], [])
row("a/**/**/b", ALL, ["a/b", "a/x/b", "a/x/y/b"], ["a/bb", "a/x/c"])
row("**/", ALL, ["a/", "a/b/"], ["a", "a/b"])
row("a//b", ALL, ["a//b"], ["a/b"], mini="differs", note="no path normalisation")
row("./a", ALL, ["./a"], ["a"], note="no path normalisation")
row("", ALL, [""], ["a"])
errors = [
    {"pattern": "{a,b", "engine": "input", "name": "an unterminated brace group is refused by the expression compiler"},
    {"pattern": "a,{b", "engine": "input", "name": "an unterminated brace group after a comma is refused"},
]
json.dump({"version": 1, "engines": ALL, "rows": rows, "errors": errors}, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
print(len(rows))
