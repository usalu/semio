#!/usr/bin/env python3
"""Moves the English Error/console/stderr literals of the Bazel executor, scopes and the source closure (and the two consumers) into the build catalog.
usage: ckey2-apply-code.py --check|--apply [--root <dir>]   (--root: a scratch tree with the same relative layout, for a dry run)"""
import json, re, sys, os
HERE = os.path.dirname(os.path.abspath(__file__))
ROWS = {r["key"]: r for r in json.load(open(os.path.join(HERE, "ckey2-messages.json"), encoding="utf8"))["messages"]}
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
FILES = {
    LIB + "🔧️tools/🏗️bazel/🎬️executor/🟦️.ts": "../../../",
    LIB + "🔧️tools/🏗️bazel/🎬️scopes/🟦️.ts": "../../../",
    LIB + "📥️source/🔏️closure/🟦️.ts": "../../",
}
LIT = re.compile(r'"(?:[^"\\\n]|\\.)*"|\'(?:[^\'\\\n]|\\.)*\'|`(?:[^`\\]|\\.)*`')
def skip_string(t, i):
    q = t[i]; i += 1
    while i < len(t):
        c = t[i]
        if c == "\\": i += 2; continue
        if q == "`" and c == "$" and t[i + 1:i + 2] == "{":
            i = skip_braces(t, i + 1); continue
        if c == q: return i + 1
        i += 1
    raise ValueError("unterminated string")
def skip_braces(t, i):
    depth = 0
    while i < len(t):
        c = t[i]
        if c in "\"'`": i = skip_string(t, i); continue
        if c == "{": depth += 1
        elif c == "}":
            depth -= 1
            if depth == 0: return i + 1
        i += 1
    raise ValueError("unbalanced braces")
def close_paren(t, i):
    depth = 0
    while i < len(t):
        c = t[i]
        if c in "\"'`": i = skip_string(t, i); continue
        if c == "(": depth += 1
        elif c == ")":
            depth -= 1
            if depth == 0: return i
        i += 1
    raise ValueError("unbalanced parens")
def template_parts(lit):
    body = lit[1:-1]; parts = []; last = 0; i = 0
    while i < len(body):
        if body[i] == "\\": i += 2; continue
        if body[i] == "$" and body[i + 1:i + 2] == "{":
            j = skip_braces(body, i + 1)
            parts.append(("text", body[last:i])); parts.append(("expr", body[i + 2:j - 1])); last = j; i = j; continue
        i += 1
    parts.append(("text", body[last:])); return parts
def decode(s): return s.encode("utf8").decode("unicode_escape").encode("latin1").decode("utf8") if "\\" in s else s
def by_en():
    table = {}
    for r in ROWS.values():
        table.setdefault(re.sub(r"\{\w+\}", "{}", r["en"]), []).append(r)
    return table
TABLE = by_en()
BAZEL = LIB + "🔧️tools/🏗️bazel/"
EXTRA = {
    BAZEL + "🎬️scopes/🟦️.ts": [("console.warn(`[bazel] the declared implementation scope table is refused (${String((error as Error).message)}), every action keeps the full executor closure\\n[bazel] die deklarierte Implementierungs-Scope-Tabelle wird abgelehnt, jede Aktion behält die volle Executor-Hülle`)", 'sayBuildV1("bazel.scope-table-refused",{reason:String((error as Error).message)},"warn")')],
    BAZEL + "🎬️executor/🟦️.ts": [('process.stderr.write("[bazel] Cleanup refusal retained after primary: "+message+"\\n")', 'sayBuildV1("bazel.cleanup-refusal-retained",{message},"warn")')],
}
CONSUMERS = [
    (BAZEL + "📜️script.ts", '!/^Implementation Source is not admitted by parent:/u.test(String((error as Error)?.message))', '!(isRepoErrorV1(error)&&error.key==="build.closure.source-not-admitted")'),
    (LIB + "📥️source/🔏️closure/🧪️tests/🕸️resolution/🟦️.ts", 'changed=error.message==="Captured implementation Source differs from admitted parent"', 'changed=error.key==="build.closure.source-differs"'),
]
SCRIPT_IMPORT_ANCHOR = 'import {CATALOG_LANGUAGES_V1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";\n'
SCRIPT_IMPORT = 'import {isRepoErrorV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🚨️error/🟦️.ts";\n'
def lookup(en_template, names_count):
    rows = TABLE.get(en_template, [])
    rows = [r for r in rows if len(re.findall(r"\{\w+\}", r["en"])) == names_count]
    return rows[0] if rows else None
def transform(text):
    out = []; pos = 0; count = 0; missing = []
    for m in re.finditer(r"(?:\bnew\s+|\bthrow\s+)*\bError\(", text):
        start = m.start(); open_idx = m.end() - 1
        head = m.group(0)
        close = close_paren(text, open_idx); args = text[open_idx + 1:close].strip()
        first = LIT.match(args)
        if not first: continue
        lit = first.group(0); rest = args[len(lit):].strip()
        values = []
        if lit[0] == "`":
            parts = template_parts(lit)
            en_parts = []
            joined = False
            for kind, value in parts:
                if kind == "text" and "\\n" in value and not joined and any(k == "expr" for k, _ in parts):
                    pass
            texts = []; exprs = []
            for kind, value in parts:
                if kind == "text": texts.append(value)
                else: exprs.append(value); texts.append("{}")
            template = "".join(texts)
            if "\\n" in template:
                template = template.split("\\n")[0]; exprs = exprs[:len(re.findall(r"\{\}", template))]
            template = decode(template); values = exprs
        elif rest.startswith("+"):
            template = decode(lit[1:-1]) + "{}"; values = [rest[1:].strip()]
        else:
            template = decode(lit[1:-1]); template = template.split("\n")[0]
        row = lookup(template, len(values))
        if row is None:
            if re.search(r"[A-Za-z]{3,}\s+[A-Za-z]{2,}", template): missing.append(template)
            continue
        names = re.findall(r"\{(\w+)\}", row["en"])
        payload = "" if not names else ",{" + ",".join(f"{n}:String({v})" for n, v in zip(names, values)) + "}"
        replacement = ("throw " if "throw" in head else "") + f'refuseBuildV1("{row["code"]}","{row["key"]}"{payload})'
        out.append(text[pos:start]); out.append(replacement); pos = close + 1; count += 1
    out.append(text[pos:]); return "".join(out), count, missing
def main():
    mode = sys.argv[1]; root = sys.argv[sys.argv.index("--root") + 1] if "--root" in sys.argv else "."
    for rel, up in FILES.items():
        path = os.path.join(root, rel); text = open(path, encoding="utf8").read()
        new, count, missing = transform(text)
        for old, repl in EXTRA.get(rel, []):
            assert new.count(old) == 1, (rel[-40:], old[:60])
            new = new.replace(old, repl)
        if "refuseBuildV1" in new and "texts/🟦️.ts" not in new:
            names = ",".join(n for n in ("refuseBuildV1", "sayBuildV1") if n + "(" in new)
            new = f'import{{{names}}}from"{up}📖️texts/🟦️.ts";\n' + new
        print(rel[-60:], "converted", count)
        if mode == "--apply": open(path, "w", encoding="utf8").write(new)
    for rel, old, repl in CONSUMERS:
        path = os.path.join(root, rel); text = open(path, encoding="utf8").read()
        assert text.count(old) == 1, (rel[-40:], old[:60])
        text = text.replace(old, repl)
        if rel.endswith("📜️script.ts"):
            assert text.count(SCRIPT_IMPORT_ANCHOR) == 1
            text = text.replace(SCRIPT_IMPORT_ANCHOR, SCRIPT_IMPORT_ANCHOR + SCRIPT_IMPORT)
        print(rel[-60:], "consumer updated")
        if mode == "--apply": open(path, "w", encoding="utf8").write(text)
main()
