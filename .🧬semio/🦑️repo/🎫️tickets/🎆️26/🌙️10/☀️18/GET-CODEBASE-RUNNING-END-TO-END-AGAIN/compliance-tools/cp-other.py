#!/usr/bin/env python3
"""🔎️ Compliance scan of the non-TypeScript files (py, rs, json, md, feature, ps1): python3 -I cp-other.py <list> <out.jsonl>. Streams one row per file."""
import ast, io, json, os, re, sys, sysconfig, tokenize
STDLIB = {n.split('.')[0] for n in os.listdir(os.path.join(sysconfig.get_paths()['stdlib'], 'lib-dynload'))} | set(sys.builtin_module_names) | {n.split('.')[0] for n in os.listdir(sysconfig.get_paths()['stdlib'])} | set(getattr(sys, 'stdlib_module_names', ()))

ROOT = os.environ.get("CP_ROOT", "/Users/ueli/Documents/semio")
LEAD = re.compile(r"^(?:[\U0001F000-\U0001FAFF☀-➿⬀-⯿⌀-⏿←-⇿■-◿⤴⤵〰〽㊗㊙©®‼⁉™ℹ]|[0-9#*]️?⃣|[∀-⋿])")
LEGACY = re.compile(r"\b(legacy|deprecated|deprecation|compat|compatibility|backward|backwards|back-compat|shim|polyfill|workaround|hack|fixme|todo)\b", re.I)
UMLAUT = re.compile(r"[äöüÄÖÜß]")
ASCII_DE = re.compile(r"\b\w*(?:fuer|ueber|pruef|aend|oeffn|koenn|muess|waehl|zaehl|laeuf|gueltig|waehrend|fuehr|loesch|schliess|groess|wuerd|verfueg|zurueck|hoeh|naechst|spaet|frueh|raeum|stuetz|ergaenz|beruecksicht|rueck|eintraeg|staerk|laeng|moeglich|moegl)\w*", re.I)
EN_WORDS = re.compile(r"\b(the|and|is|are|not|with|from|has|have|was|were|this|that|when|before|after|because|already|instead|until|while|which)\b", re.I)
DE_WORDS = re.compile(r"\b(der|die|das|und|ist|sind|nicht|mit|von|hat|haben|wurde|wird|dieser|diese|wenn|vor|nach|weil|bereits|statt|bis|waehrend|während|oder|ein|eine|einen|kein|keine|auf|im|zu|fuer|für)\b", re.I)


def lead(text):
    t = text.lstrip()
    m = LEAD.match(t)
    return m.group(0) if m else None


def py_rows(src):
    rows = []
    try:
        tree = ast.parse(src)
    except SyntaxError as e:
        return [("syntax-error", e.lineno or 0, str(e))]
    keys = {}
    def doc_of(node, name):
        d = ast.get_docstring(node, clean=False)
        if d is None:
            return
        line = node.body[0].lineno
        k = lead(d)
        if k is None:
            rows.append(("doc-not-emoji", line, f"{name}: {d.strip()[:40]!r}"))
        else:
            keys.setdefault(k.replace("️", ""), []).append(line)
    doc_of(tree, "module")
    funcs = []
    for node in ast.walk(tree):
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
            doc_of(node, node.name)
            if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
                funcs.append(node)
    for k, lines in keys.items():
        if len(lines) > 1:
            for line in lines:
                rows.append(("dup-emoji", line, f"{k} x{len(lines)} ({','.join(map(str, lines))})"))
    spans = [(f.body[0].lineno if not (isinstance(f.body[0], ast.Expr) and isinstance(getattr(f.body[0], 'value', None), ast.Constant) and isinstance(f.body[0].value.value, str)) else (f.body[1].lineno if len(f.body) > 1 else f.end_lineno + 1), f.end_lineno) for f in funcs]
    try:
        for tok in tokenize.generate_tokens(io.StringIO(src).readline):
            if tok.type == tokenize.COMMENT:
                line = tok.start[0]
                text = tok.string
                if line == 1 and text.startswith("#!"):
                    continue
                if any(a <= line <= b for a, b in spans):
                    rows.append(("comment-in-body", line, text[:100]))
    except tokenize.TokenError:
        pass
    mods = set()
    for node in ast.walk(tree):
        if isinstance(node, ast.Import):
            mods.update(a.name.split(".")[0] for a in node.names)
        elif isinstance(node, ast.ImportFrom) and node.level == 0 and node.module:
            mods.add(node.module.split(".")[0])
    for m in sorted(mods):
        if m not in STDLIB:
            rows.append(("external-import", 0, m))
    return rows


def rs_rows(src):
    rows = []
    keys = {}
    run = None
    opened = False
    depth = 0
    for i, raw in enumerate(src.split("\n"), 1):
        s = raw.strip()
        m = re.match(r"^//[/!](?!/)", s)
        if m:
            kind = s[:3]
            if run != kind:
                opened = False
            run = kind
            body = s[3:].strip()
            if not opened and body:
                opened = True
                k = lead(body)
                if k is None:
                    rows.append(("doc-not-emoji", i, body[:50]))
                else:
                    keys.setdefault(k.replace("️", ""), []).append(i)
            continue
        run = None
        opened = False
        if s.startswith("//") and depth > 0 and not s.startswith("///"):
            rows.append(("comment-in-body", i, s[:100]))
        elif "//" in s and depth > 0 and not re.search(r"[\"'].*//.*[\"']", s):
            rows.append(("trailing-comment-in-body", i, s[:100]))
        depth += raw.count("{") - raw.count("}")
    for k, lines in keys.items():
        if len(lines) > 1:
            for line in lines:
                rows.append(("dup-emoji", line, f"{k} x{len(lines)} ({','.join(map(str, lines))})"))
    return rows


def walk_json(value, path, out, file):
    if isinstance(value, dict):
        keys = set(value.keys())
        if keys == {"en", "de"} and all(isinstance(v, str) for v in value.values()):
            en, de = value["en"], value["de"]
            if en == de and len(en.split()) > 1:
                out.append(("de-equals-en", 0, f"{path}: {en[:60]}"))
            if EN_WORDS.search(de) and len(EN_WORDS.findall(de)) >= 2 and len(DE_WORDS.findall(de)) == 0:
                out.append(("de-looks-english", 0, f"{path}: {de[:80]}"))
            if ASCII_DE.search(de):
                out.append(("de-ascii-fallback", 0, f"{path}: {ASCII_DE.search(de).group(0)} | {de[:70]}"))
            if UMLAUT.search(de):
                out.append(("de-real-umlaut", 0, path))
            if not de.strip():
                out.append(("de-empty", 0, path))
            return
        for k, v in value.items():
            walk_json(v, f"{path}/{k}", out, file)
    elif isinstance(value, list):
        for i, v in enumerate(value):
            walk_json(v, f"{path}/{i}", out, file)


def json_rows(src):
    rows = []
    try:
        doc = json.loads(src)
    except Exception as e:
        return [("json-invalid", 0, str(e)[:100])]
    walk_json(doc, "", rows, "")
    return rows


def md_rows(src):
    return []


def main():
    lst, out = sys.argv[1], sys.argv[2]
    with open(out, "w", encoding="utf8") as o:
        for line in open(lst, encoding="utf8"):
            path = line.rstrip("\n")
            if not path:
                continue
            try:
                src = open(f"{ROOT}/{path}", encoding="utf8").read()
            except Exception as e:
                o.write(json.dumps({"file": path, "error": str(e)}) + "\n")
                continue
            rows = []
            if path.endswith(".py"):
                rows += py_rows(src)
            elif path.endswith(".rs"):
                rows += rs_rows(src)
            elif path.endswith(".json"):
                rows += json_rows(src)
            for i, raw in enumerate(src.split("\n"), 1):
                if "[DEBUG]" in raw:
                    rows.append(("debug-tag", i, raw.strip()[:100]))
                if not path.endswith(".json") and LEGACY.search(raw):
                    rows.append(("legacy-word", i, raw.strip()[:140]))
            o.write(json.dumps({"file": path, "rows": [{"rule": r, "line": l, "detail": d} for r, l, d in rows], "lines": src.count("\n") + 1}, ensure_ascii=False) + "\n")


main()
