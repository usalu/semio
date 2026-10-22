"""Increment 3 (c9): the law closes. Console and stream prose have no allowance any more, assumed languages have none either, machine lines ([DEBUG], [DATA]) are not prose,
hidden files are not sources; the allowlist keeps only the English `Error` literals of the modules that are not migrated yet."""
import json
from lib import *

LAW = L + "/⚖️laws/🗣️language-catalogs"
TS = LAW + "/🟦️.ts"
PY = LAW + "/🧪️tests/🗣️corpus/🐍️checker.py"
TEST = LAW + "/🧪️tests/🗣️corpus/🟦️.ts"
CORPUS = LAW + "/🧪️tests/🗣️corpus/🔣️.json"
CORPUS_SCHEMA = LAW + "/🧪️tests/🗣️corpus/🧬️schema/🔣️.json"
ALLOW_SCHEMA = LAW + "/🔒️allowlist/🧬️schema/🔣️.json"

FINDINGS_TS = '''export function languageLawFindingsV1(sources: readonly LanguageLawSourceV1[], allowlist: LanguageLawAllowlistV1): readonly LanguageLawFindingV1[] {
  const out: LanguageLawFindingV1[] = [], seen = new Set<string>();
  for (const source of sources) {
    if (source.path.endsWith(".json")) out.push(...catalogFindingsV1(source));
    else if (SOURCE_FILE.test(source.path)) {
      if (!source.path.includes(OWN_DIRECTORY)) out.push(...defaultLanguageFindingsV1(source));
      const output = proseOutputSitesV1(source.text), actual = proseSitesV1(source.text) - output, allowed = allowlist.files[source.path] ?? 0;
      seen.add(source.path);
      if (output > 0) out.push({ rule: "console-prose", path: source.path, detail: `count=${output}` });
      if (actual > allowed) out.push({ rule: "prose-literal", path: source.path, detail: `allowed=${allowed} actual=${actual}` });
      else if (actual < allowed) out.push({ rule: "allowance-stale", path: source.path, detail: `allowed=${allowed} actual=${actual}` });
    }
  }
  for (const path of Object.keys(allowlist.files)) if (!seen.has(path)) out.push({ rule: "allowance-stale", path, detail: `allowed=${allowlist.files[path]} actual=0` });
  const total = Object.values(allowlist.files).reduce((sum, count) => sum + count, 0);
  if (total > allowlist.ceiling) out.push({ rule: "ceiling-grown", path: "allowlist", detail: `ceiling=${allowlist.ceiling} total=${total}` });
  return out.sort((a, b) => compare(a.path, b.path) || compare(a.rule, b.rule) || compare(a.detail, b.detail));
}

'''

CUT_TS = '''export function languageLawAllowlistV1(sources: readonly LanguageLawSourceV1[], ceiling?: number): LanguageLawAllowlistV1 {
  const files: Record<string, number> = {};
  for (const source of sources) {
    if (!SOURCE_FILE.test(source.path)) continue;
    const count = proseSitesV1(source.text) - proseOutputSitesV1(source.text);
    if (count > 0) files[source.path] = count;
  }
  return Object.freeze({ version: 1, ceiling: ceiling ?? Object.values(files).reduce((total, count) => total + count, 0), files: Object.freeze(files) });
}
'''


def cut_between(text, start, end):
    a = text.index(start)
    b = text.index(end, a)
    return a, b


def law_ts(tree):
    t = tree.text(TS)
    t = t.replace("export type LanguageLawAllowlistV1 = Readonly<{ version: 1; ceiling: number; files: Readonly<Record<string, number>>; defaultsCeiling: number; defaults: Readonly<Record<string, number>> }>;", "export type LanguageLawAllowlistV1 = Readonly<{ version: 1; ceiling: number; files: Readonly<Record<string, number>> }>;")
    t = t.replace("/** 🔒️ The ratchet of English prose literals per source file; the shape is `🔒️allowlist/🧬️schema/🔣️.json`. */", "/** 🔒️ The ratchet of English `Error` literals per source file; the shape is `🔒️allowlist/🧬️schema/🔣️.json`. Console and stream prose and assumed languages have no allowance. */")
    t = t.replace('"prose-literal", "allowance-stale", "ceiling-grown"]', '"prose-literal", "console-prose", "allowance-stale", "ceiling-grown"]')
    t = t.replace('const PROSE_WORD = /[A-Za-z]{3,}/u;', 'const PROSE_WORD = /[A-Za-z]{3,}/u;\nconst MACHINE = /^\\[(?:DEBUG|DATA)\\] /u;')
    t = t.replace("export function isProseV1(text: string): boolean {\n", "export function isProseV1(text: string): boolean {\n  if (MACHINE.test(text)) return false;\n")
    a, b = cut_between(t, "/** 🧮️ How many English prose literals", "function walkCatalog(")
    t = t[:a] + '''/** 🧮️ How many English prose literals a source passes to `Error(`, `console.*(` and `stdout|stderr.write(` (`outputOnly`: only the last two, the lines a person reads). */
function countSites(text: string, indexes: readonly number[]): number {
  let count = 0;
  for (const line of text.split("\\n")) {
    if (isComment(line)) continue;
    for (const index of indexes) {
      for (const match of line.matchAll(SITES[index]!)) {
        const literal = match[1]!;
        if (literal.startsWith("`") && literal.includes("\\n")) continue;
        if (isProseV1(literal.slice(1, -1).replace(/\\$\\{[^}]*\\}/gu, "XX"))) count++;
      }
    }
  }
  return count;
}

/** 🧮️ The English prose literals passed to `Error(`, `console.*(` and `stdout|stderr.write(`. */
export function proseSitesV1(text: string): number {
  return countSites(text, [0, 1, 2]);
}

/** 📺️ The English prose literals passed to `console.*(` and `stdout|stderr.write(`: the lines a person reads, which belong to a catalog. */
export function proseOutputSitesV1(text: string): number {
  return countSites(text, [1, 2]);
}

''' + t[b:]
    a, b = cut_between(t, "/** 🔬️ Every finding of every rule", "/** 📚️ Reads the JSON catalogs")
    t = t[:a] + "/** 🔬️ Every finding of every rule over the sources and the allowlist, sorted by file, rule and detail. */\n" + FINDINGS_TS + t[b:]
    t = t.replace("      if (entry.isSymbolicLink()) continue;", '      if (entry.isSymbolicLink() || entry.name.startsWith(".")) continue;')
    a, b = cut_between(t, "/** 🪜️ Cuts the allowlist from the sources", "/** 📋️ The findings of the language law as policy breaches. */")
    t = t[:a] + "/** 🪜️ Cuts the allowlist from the sources: every file with its count of English `Error` literals; the ceiling may only be passed lower than the total it replaces. */\nexport " + CUT_TS + "\n" + t[b:]
    t = t.replace("a leaf", "a leaf") if False else t
    tree.put(TS, t)


def law_py(tree):
    t = tree.text(PY)
    t = t.replace('DEFAULTS = [', 'MACHINE = re.compile(r"^\\[(?:DEBUG|DATA)\\] ")\nDEFAULTS = [')
    t = t.replace("def prose(text):\n", "def prose(text):\n    if MACHINE.search(text):\n        return False\n")
    a, b = cut_between(t, "def sites(text):", "def is_dict(")
    t = t[:a] + '''def count_sites(text, indexes):
    count = 0
    for line in text.split("\\n"):
        if comment(line):
            continue
        for index in indexes:
            for match in SITES[index].finditer(line):
                literal = match.group(1)
                if literal.startswith("`") and "\\n" in literal:
                    continue
                if prose(re.sub(r"\\$\\{[^}]*\\}", "XX", literal[1:-1])):
                    count += 1
    return count


def sites(text):
    return count_sites(text, [0, 1, 2])


def output_sites(text):
    return count_sites(text, [1, 2])


''' + t[b:]
    a, b = cut_between(t, "def assumed_lines(text):", "def is_dict(") if "def assumed_lines(text):" in t and t.index("def assumed_lines(text):") < t.index("def is_dict(") else (None, None)
    a, b = cut_between(t, "def findings(sources, allowlist):", "def read(workspace):")
    t = t[:a] + '''def findings(sources, allowlist):
    out, seen = [], set()
    for source in sources:
        path, text = source["path"], source["text"]
        if path.endswith(".json"):
            try:
                document = json.loads(text)
            except ValueError:
                continue
            walk(document, "", path, out)
        elif SOURCE.search(path):
            if OWN not in path:
                for row in assumed_lines(text):
                    out.append({"rule": row["rule"], "path": path, "detail": row["detail"]})
            output = output_sites(text)
            actual, allowed = sites(text) - output, allowlist["files"].get(path, 0)
            seen.add(path)
            if output > 0:
                out.append({"rule": "console-prose", "path": path, "detail": "count=%d" % output})
            if actual > allowed:
                out.append({"rule": "prose-literal", "path": path, "detail": "allowed=%d actual=%d" % (allowed, actual)})
            elif actual < allowed:
                out.append({"rule": "allowance-stale", "path": path, "detail": "allowed=%d actual=%d" % (allowed, actual)})
    for path in allowlist["files"]:
        if path not in seen:
            out.append({"rule": "allowance-stale", "path": path, "detail": "allowed=%d actual=0" % allowlist["files"][path]})
    total = sum(allowlist["files"].values())
    if total > allowlist["ceiling"]:
        out.append({"rule": "ceiling-grown", "path": "allowlist", "detail": "ceiling=%d total=%d" % (allowlist["ceiling"], total)})
    return sorted(out, key=lambda row: (row["path"], row["rule"], row["detail"]))


''' + t[b:]
    a, b = cut_between(t, "def cut(sources):", "def main():")
    t = t[:a] + '''def cut(sources):
    files = {}
    for source in sources:
        if SOURCE.search(source["path"]):
            count = sites(source["text"]) - output_sites(source["text"])
            if count > 0:
                files[source["path"]] = count
    return {"version": 1, "ceiling": sum(files.values()), "files": files}


''' + t[b:]
    t = t.replace('''"prose": [sites(row["text"]) for row in corpus["prose"]]}, sys.stdout''', '''"prose": [sites(row["text"]) for row in corpus["prose"]], "output": [output_sites(row["text"]) for row in corpus["output"]]}, sys.stdout''')
    t = t.replace("                directories[:] = sorted(name for name in directories if name not in SKIP and not os.path.islink(os.path.join(current, name)))", "                directories[:] = sorted(name for name in directories if name not in SKIP and not name.startswith(\".\") and not os.path.islink(os.path.join(current, name)))") if False else t
    t = t.replace('if os.path.islink(full) or not os.path.isfile(full) or not (name.endswith(".json") or SOURCE.search(name)):', 'if name.startswith(".") or os.path.islink(full) or not os.path.isfile(full) or not (name.endswith(".json") or SOURCE.search(name)):')
    t = t.replace('directories[:] = sorted(name for name in directories if name not in SKIP and not os.path.islink(os.path.join(current, name)))', 'directories[:] = sorted(name for name in directories if name not in SKIP and not name.startswith(".") and not os.path.islink(os.path.join(current, name)))')
    tree.put(PY, t)


DROPPED = {"an assumed language within its allowance", "assumed languages above their allowance are all named", "an assumed-language allowance above the lines", "an assumed-language allowance of a file that is gone", "an assumed-language ceiling that grew"}


def corpus(tree):
    d = json.loads(tree.text(CORPUS))
    cases = []
    P0 = L + "/x/a.ts"
    for row in d["cases"]:
        if row["name"] in DROPPED:
            continue
        row["allowlist"] = {"version": 1, "ceiling": row["allowlist"]["ceiling"], "files": row["allowlist"]["files"]}
        cases.append(row)
    by = {row["name"]: row for row in cases}

    def put(name, sources, expected, files=None, ceiling=0):
        by[name]["sources"], by[name]["expected"] = sources, expected
        by[name]["allowlist"] = {"version": 1, "ceiling": ceiling, "files": files or {}}

    src = lambda text: [{"path": P0, "text": text}]
    f = lambda rule, path, detail: {"rule": rule, "path": path, "detail": detail}
    put("an exact allowance", src('throw new Error(`Unknown ${name} here`);\nthrow Error("Required thing missing");\n'), [], {P0: 2}, 2)
    put("a literal above its allowance", src('throw new Error("Done with it");\nthrow Error("two words here");\n'), [f("prose-literal", P0, "allowed=1 actual=2")], {P0: 1}, 1)
    put("an allowance above the literals", src('throw new Error("Only one here");\n'), [f("allowance-stale", P0, "allowed=3 actual=1")], {P0: 3}, 3)
    put("a ceiling that grew", src('throw new Error("Hello brave world");\n'), [f("ceiling-grown", "allowlist", "ceiling=0 total=1")], {P0: 1}, 0)
    order = [r["name"] for r in cases]
    extra = [
        {"name": "console and stream prose has no allowance", "sources": src('console.log("Hello brave world");\nprocess.stderr.write("two words\\n");\n'), "allowlist": {"version": 1, "ceiling": 2, "files": {P0: 2}}, "expected": [f("allowance-stale", P0, "allowed=2 actual=0"), f("console-prose", P0, "count=2")]},
        {"name": "console prose beside an error literal is counted apart", "sources": src('console.error("Done with it");\nthrow new Error("Required thing missing");\n'), "allowlist": {"version": 1, "ceiling": 1, "files": {P0: 1}}, "expected": [f("console-prose", P0, "count=1")]},
        {"name": "machine lines are not prose", "sources": src('console.error("[DEBUG] two words here");\nconsole.error("[DATA] more words here "+x);\nthrow new Error("[DATA] something broke here");\n'), "allowlist": {"version": 1, "ceiling": 0, "files": {}}, "expected": []},
        {"name": "hidden files are not read by the repository scan but a path is judged as given", "sources": [{"path": L + "/x/.hidden.ts", "text": 'throw new Error("Required thing missing");\n'}], "allowlist": {"version": 1, "ceiling": 0, "files": {}}, "expected": [f("prose-literal", L + "/x/.hidden.ts", "allowed=0 actual=1")]},
        {"name": "an assumed language is never tolerated", "sources": src('const l = value ?? "en";\n'), "allowlist": {"version": 1, "ceiling": 5, "files": {P0: 1}}, "expected": [f("allowance-stale", P0, "allowed=1 actual=0"), f("default-language", P0, "1:en-fallback")]},
    ]
    d["cases"] = cases + extra
    d["prose"] += [{"text": 'console.log("[DEBUG] two words here");', "count": 0}, {"text": 'throw new Error("[DATA] two words here");', "count": 0}, {"text": 'console.error("[DATA] more words here "+x);', "count": 0}]
    d["output"] = [{"text": 'console.log("Hello world");', "count": 1}, {"text": 'throw new Error("Original thing required");', "count": 0}, {"text": 'process.stderr.write("Two words");', "count": 1}, {"text": 'console.log("Hello world"); throw Error("Original thing required");', "count": 1}, {"text": 'console.log("[DEBUG] two words here");', "count": 0}]
    tree.put(CORPUS, json.dumps(d, ensure_ascii=False, indent=1) + "\n")
    s = tree.text(CORPUS_SCHEMA)
    s = s.replace('"required": ["version", "cases", "prose"],', '"required": ["version", "cases", "prose", "output"],')
    s = s.replace('"required": ["version", "ceiling", "files", "defaultsCeiling", "defaults"],', '"required": ["version", "ceiling", "files"],')
    s = s.replace('''              "files": { "type": "object", "additionalProperties": { "type": "integer", "minimum": 1 } },
              "defaultsCeiling": { "type": "integer", "minimum": 0 },
              "defaults": { "type": "object", "additionalProperties": { "type": "integer", "minimum": 1 } }''', '''              "files": { "type": "object", "additionalProperties": { "type": "integer", "minimum": 1 } }''')
    s = s.replace('"prose-literal", "allowance-stale"', '"prose-literal", "console-prose", "allowance-stale"')
    s = s.replace('''    "prose": {
      "type": "array",
      "minItems": 1,
      "items": {
        "type": "object",
        "additionalProperties": false,
        "required": ["text", "count"],
        "properties": { "text": { "type": "string" }, "count": { "type": "integer", "minimum": 0 } }
      }
    }''', '''    "prose": {
      "type": "array",
      "minItems": 1,
      "items": {
        "type": "object",
        "additionalProperties": false,
        "required": ["text", "count"],
        "properties": { "text": { "type": "string" }, "count": { "type": "integer", "minimum": 0 } }
      }
    },
    "output": {
      "type": "array",
      "minItems": 1,
      "items": {
        "type": "object",
        "additionalProperties": false,
        "required": ["text", "count"],
        "properties": { "text": { "type": "string" }, "count": { "type": "integer", "minimum": 0 } }
      }
    }''')
    tree.put(CORPUS_SCHEMA, s)
    a = tree.text(ALLOW_SCHEMA)
    a = a.replace('"required": ["version", "ceiling", "files", "defaultsCeiling", "defaults"],', '"required": ["version", "ceiling", "files"],')
    a = a.replace(''',
    "defaultsCeiling": { "type": "integer", "minimum": 0 },
    "defaults": {
      "type": "object",
      "propertyNames": { "pattern": "^[^/].*\\\\.(?:ts|mts|mjs|cjs)$" },
      "additionalProperties": { "type": "integer", "minimum": 1 }
    }''', "")
    a = a.replace("how many English prose literals (the first argument of `Error(`, `console.*(` and `stdout|stderr.write(`) and how many lines that assume a language (an English fallback, the first offered language, a German-or-English ternary) each source file of the repo build library and of the process module still holds. A file may only lose them; a file that is not listed holds none; each ceiling is the total when the list was last cut and the totals may only shrink.", "how many English prose literals as the first argument of `Error(` each source file of the repo build library and of the process module still holds; console and stream prose and assumed languages have no allowance at all. A file may only lose literals; a file that is not listed holds none; `ceiling` is the total when the list was last cut and the total may only shrink.")
    tree.put(ALLOW_SCHEMA, a)


def test_file(tree):
    t = tree.text(TEST)
    t = t.replace("import { LANGUAGE_LAW_RULES_V1, isProseV1, languageLawAllowlistV1, languageLawFindingsV1, proseSitesV1, readLanguageLawSourcesV1, type LanguageLawAllowlistV1 }", "import { LANGUAGE_LAW_RULES_V1, isProseV1, languageLawAllowlistV1, languageLawFindingsV1, proseOutputSitesV1, proseSitesV1, readLanguageLawSourcesV1, type LanguageLawAllowlistV1 }")
    t = t.replace("  expect(isProseV1(\"Hello world\")).toBe(true);", "  for (const row of corpus.output) expect(proseOutputSitesV1(row.text), row.text).toBe(row.count);\n  expect(isProseV1(\"Hello world\")).toBe(true);\n  expect(isProseV1(\"[DEBUG] two words here\")).toBe(false);")
    t = t.replace("const other = python([\"cases\"], JSON.stringify(corpus)) as { cases: { name: string; findings: unknown[] }[]; prose: number[] };", "const other = python([\"cases\"], JSON.stringify(corpus)) as { cases: { name: string; findings: unknown[] }[]; prose: number[]; output: number[] };")
    t = t.replace("  expect(other.prose).toEqual(corpus.prose.map((row) => row.count));", "  expect(other.prose).toEqual(corpus.prose.map((row) => row.count));\n  expect(other.output).toEqual(corpus.output.map((row) => row.count));")
    t = t.replace("languageLawAllowlistV1(sources, { ceiling: allowlist.ceiling, defaultsCeiling: allowlist.defaultsCeiling })", "languageLawAllowlistV1(sources, allowlist.ceiling)")
    tree.put(TEST, t)


def run(tree):
    law_ts(tree)
    law_py(tree)
    corpus(tree)
    test_file(tree)
