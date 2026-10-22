#!/usr/bin/env python3
"""Writes the language-agnostic corpus of the language law (hand-authored expectations; both implementations must agree with them)."""
import json, sys
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
OUT = "staged/files/" + L + "/⚖️laws/🗣️language-catalogs/🧪️tests/🗣️corpus/🔣️.json"
A0 = {"version": 1, "ceiling": 0, "files": {}, "defaultsCeiling": 0, "defaults": {}}
def al(ceiling, files, defaults_ceiling=0, defaults=None): return {"version": 1, "ceiling": ceiling, "files": files, "defaultsCeiling": defaults_ceiling, "defaults": defaults or {}}
def src(path, text): return {"path": path, "text": text}
def js(o): return json.dumps(o, ensure_ascii=False)
def case(name, sources, expected, allowlist=A0): return {"name": name, "sources": sources, "allowlist": allowlist, "expected": expected}
def f(rule, path, detail): return {"rule": rule, "path": path, "detail": detail}
P = L + "/x"
cases = [
 case("a clean bilingual catalog", [src(P + "/a.json", js({"version": 1, "languages": ["en", "de"], "messages": {"a": {"en": "A {x}", "de": "A {x}"}}}))], []),
 case("german first", [src(P + "/a.json", js({"languages": ["de", "en"]}))], [f("languages-not-en-de", P + "/a.json", "")]),
 case("upper case languages", [src(P + "/a.json", js({"nested": {"languages": ["EN", "DE"]}}))], [f("languages-not-en-de", P + "/a.json", "/nested")]),
 case("implementation languages are not texts", [src(P + "/a.json", js({"languages": ["ts", "go", "rs"]}))], []),
 case("a leaf without german", [src(P + "/a.json", js({"labels": {"k": {"en": "A"}}}))], [f("leaf-missing-language", P + "/a.json", "/labels/k")]),
 case("a leaf with a third language", [src(P + "/a.json", js({"k": {"en": "A", "de": "A", "fr": "A"}}))], [f("leaf-missing-language", P + "/a.json", "/k")]),
 case("a leaf with different placeholders", [src(P + "/a.json", js({"k": {"en": "A {x}", "de": "A {y}"}}))], [f("placeholder-mismatch", P + "/a.json", "/k")]),
 case("a table with a missing key and a placeholder drift", [src(P + "/a.json", js({"t": {"en": {"a": "A {x}", "b": "B"}, "de": {"a": "A", "c": "C"}}}))], [f("placeholder-mismatch", P + "/a.json", "/t/a"), f("table-key-mismatch", P + "/a.json", "/t/b"), f("table-key-mismatch", P + "/a.json", "/t/c")]),
 case("a table that agrees", [src(P + "/a.json", js({"t": {"en": {"a": "A {x}", "n": {"b": "B"}}, "de": {"a": "A {x}", "n": {"b": "B"}}}}))], []),
 case("language keys of one other object are not leaves", [src(P + "/a.json", js({"k": {"en": "A", "de": "A", "other": 1}}))], []),
 case("invalid json is skipped", [src(P + "/a.json", "{")], []),
 case("an english fallback", [src(P + "/a.ts", "const l = value ?? \"en\";\nconst m = value||'en';\n")], [f("default-language", P + "/a.ts", "1:en-fallback"), f("default-language", P + "/a.ts", "2:en-fallback")]),
 case("the first offered language", [src(P + "/a.mjs", "return languages[0]!;\nreturn protocol.languages [ 0 ];\n")], [f("default-language", P + "/a.mjs", "1:first-language"), f("default-language", P + "/a.mjs", "2:first-language")]),
 case("a german-or-english ternary", [src(P + "/a.ts", "return v.startsWith(\"de\")?\"de\":\"en\";\n")], [f("default-language", P + "/a.ts", "1:de-or-en")]),
 case("the dashboard locale", [src(P + "/a.ts", "const n = [\"SEMIO_DASHBOARD_LOCALE\", \"LANG\"];\n")], [f("default-language", P + "/a.ts", "1:dashboard-locale")]),
 case("a returned english", [src(P + "/a.ts", "function f() { return \"en\"; }\n")], [f("default-language", P + "/a.ts", "1:en-return")]),
 case("an assumed language within its allowance", [src(P + "/a.ts", "const l = value ?? \"en\";\n")], [], al(0, {}, 1, {P + "/a.ts": 1})),
 case("assumed languages above their allowance are all named", [src(P + "/a.ts", "const l = value ?? \"en\";\nreturn \"en\";\n")], [f("default-language", P + "/a.ts", "1:en-fallback"), f("default-language", P + "/a.ts", "2:en-return")], al(0, {}, 1, {P + "/a.ts": 1})),
 case("an assumed-language allowance above the lines", [src(P + "/a.ts", "export const a = 1;\n")], [f("allowance-stale", P + "/a.ts", "defaults allowed=2 actual=0")], al(0, {}, 2, {P + "/a.ts": 2})),
 case("an assumed-language allowance of a file that is gone", [src(P + "/b.ts", "export const a = 1;\n")], [f("allowance-stale", P + "/gone.ts", "defaults allowed=1 actual=0")], al(0, {}, 1, {P + "/gone.ts": 1})),
 case("an assumed-language ceiling that grew", [src(P + "/a.ts", "const l = value ?? \"en\";\n")], [f("ceiling-grown", "allowlist", "defaults ceiling=0 total=1")], al(0, {}, 0, {P + "/a.ts": 1})),
 case("comments never count", [src(P + "/a.ts", "// value ?? \"en\"\n/* languages[0] */\n * SEMIO_DASHBOARD_LOCALE\n")], []),
 case("the law's own directory is exempt from the default scan", [src(L + "/⚖️laws/🗣️language-catalogs/🟦️.ts", "const x = /\\?\\?\\s*[\"']en[\"']/u; value ?? \"en\";\n")], []),
 case("an exact allowance", [src(P + "/a.ts", "console.log(\"Hello brave world\");\nthrow new Error(`Unknown ${name} here`);\n")], [], al(2, {P + "/a.ts": 2})),
 case("a literal above its allowance", [src(P + "/a.ts", "console.error('Done with it');\nprocess.stderr.write(\"two words\\n\");\n")], [f("prose-literal", P + "/a.ts", "allowed=1 actual=2")], al(1, {P + "/a.ts": 1})),
 case("a file that is not listed holds none", [src(P + "/a.ts", "throw Error(\"Required thing missing\");\n")], [f("prose-literal", P + "/a.ts", "allowed=0 actual=1")]),
 case("an allowance above the literals", [src(P + "/a.ts", "console.log(\"Only one here\");\n")], [f("allowance-stale", P + "/a.ts", "allowed=3 actual=1")], al(3, {P + "/a.ts": 3})),
 case("an allowance of a file that is gone", [src(P + "/a.ts", "export const a = 1;\n")], [f("allowance-stale", P + "/gone.ts", "allowed=2 actual=0")], al(2, {P + "/gone.ts": 2})),
 case("a ceiling that grew", [src(P + "/a.ts", "console.log(\"Hello brave world\");\n")], [f("ceiling-grown", "allowlist", "ceiling=0 total=1")], al(0, {P + "/a.ts": 1})),
 case("code, identifiers and single words are not prose", [src(P + "/a.ts", "console.log(\"ok\");\nthrow Error(\"E_CODE_ONE\");\nthrow new Error(`${a}`);\nconsole.log(\"[tag] x\");\nconsole.log(\"a=b c=d\");\nconsole.log(\"{\\\"a\\\": 1}\");\n")], []),
 case("the call must start the literal", [src(P + "/a.ts", "console.log(message, \"two words here\");\nconst x = format(\"two words here\");\n")], []),
 case("tests and other json are not scanned for prose", [src(P + "/a.json", js({"message": "two words here"}))], []),
]
prose = [
 {"text": "console.log(\"Hello world\");", "count": 1},
 {"text": "console.log('Hello world'); console.error(\"Another line\");", "count": 2},
 {"text": "throw new Error(\"Original thing required\");", "count": 1},
 {"text": "throw Error(\"Original thing required\");", "count": 1},
 {"text": "Error(\"single\")", "count": 0},
 {"text": "process.stdout.write(\"Two words\");", "count": 1},
 {"text": "const a = 1; // console.log(\"Hello world\");", "count": 1},
 {"text": "  // console.log(\"Hello world\");", "count": 0},
 {"text": "console.log(`Total ${count} files checked`);", "count": 1},
 {"text": "console.log(`${a} ${b}`);", "count": 0},
 {"text": "console.log(\"import x from y\");", "count": 0},
 {"text": "console.log(\"return the value here\");", "count": 1},
 {"text": "console.log(\"a (b) c\");", "count": 0},
 {"text": "console.log(\"Fertig mit der Arbeit\");", "count": 1},
]
corpus = {"version": 1, "cases": [{k: v for k, v in c.items()} for c in cases], "prose": prose}
open(OUT, "w", encoding="utf8").write(json.dumps(corpus, ensure_ascii=False, indent=1) + "\n")
print(len(cases), "cases")
