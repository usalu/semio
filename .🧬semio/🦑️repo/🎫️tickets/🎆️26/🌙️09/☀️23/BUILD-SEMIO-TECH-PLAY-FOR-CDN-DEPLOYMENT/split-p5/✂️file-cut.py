#!/usr/bin/env python3
"""✂️ Cuts one Rust source file into several crate root files along line ranges (P5 split lane).

Each output piece receives: its docstring header, the original file's external `use` lines narrowed to the names the
piece mentions, one `use <crate>::{…}` line per lower piece it references (names defined there), and its line ranges
in original order. Items of a piece that a higher piece references are made `pub` (fields included) so the cut
compiles across crates; nothing is re-exported.

usage: ✂️file-cut.py <plan.json> plan|apply
plan: {"source": <file>, "pieces": [{"name", "crate", "out", "doc": [..], "ranges": ["a-b", ..], "extra_uses": [..]}],
       "drop": ["a-b", ..]}   (pieces listed lowest first; the first piece may write back to the source path)
"""
import json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import importlib.util
spec = importlib.util.spec_from_file_location("items", os.path.join(os.path.dirname(os.path.abspath(__file__)), "🧩️items.py"))

def load_items():
    src = open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "🧩️items.py"), encoding="utf-8").read().rsplit("\nmain()", 1)[0]
    ns = {"__file__": os.path.join(os.path.dirname(os.path.abspath(__file__)), "🧩️items.py")}
    exec(compile(src, "items", "exec"), ns)
    return ns

NS = load_items()

def rng(s):
    a, b = s.split("-"); return int(a), int(b)

def main():
    plan = json.load(open(sys.argv[1], encoding="utf-8")); mode = sys.argv[2]
    path = plan["source"]
    raw = open(path, encoding="utf-8").read(); lines = raw.split("\n")
    items = NS["items"](path)
    pieces = plan["pieces"]
    def piece_of(line):
        for i, p in enumerate(pieces):
            for r in p["ranges"]:
                a, b = rng(r)
                if a <= line <= b: return i
        return None
    uses = [it for it in items if it["kind"] == "use"]
    defined = {}
    for it in items:
        if it["kind"] in ("use", "mod") : continue
        if it["kind"] == "impl": continue
        defined.setdefault(it["name"], []).append(it)
    for it in items:
        it["piece"] = piece_of(it["start"])
        it["refs"] = sorted(n for n in defined if n != it["name"] and re.search(r"(?<![\w:])" + re.escape(n) + r"\b", it["body"]))
    owner = {n: its[0]["piece"] for n, its in defined.items()}
    unassigned = [(it["start"], it["name"]) for it in items if it["piece"] is None and it["kind"] not in ("use",) and not any(rng(d)[0] <= it["start"] <= rng(d)[1] for d in plan.get("drop", []))]
    if unassigned: print("UNASSIGNED", unassigned)
    # references
    need = [dict() for _ in pieces]   # piece -> {lower piece: set(names)}
    exported = set()
    for it in items:
        if it["piece"] is None or it["kind"] == "use": continue
        for n in it["refs"]:
            o = owner.get(n)
            if o is None or o == it["piece"]: continue
            if o > it["piece"]:
                print(f"UPWARD {pieces[it['piece']]['name']} -> {pieces[o]['name']}: {it['name']}@{it['start']} uses {n}")
                continue
            need[it["piece"]].setdefault(o, set()).add(n); exported.add(n)
    # impl methods used across pieces: publicize every impl fn of a type defined in a lower piece that is referenced upward
    out_text = []
    for i, p in enumerate(pieces):
        body_lines = []
        for r in p["ranges"]:
            a, b = rng(r)
            body_lines.append((a, lines[a - 1:b]))
        body_lines.sort()
        body = "\n".join("\n".join(chunk) for _, chunk in body_lines)
        # publicize
        def pub_item(m):
            return m.group(0)
        for n in sorted(exported):
            if owner.get(n) != i: continue
            body = re.sub(r"(?m)^(\s*)(?:pub\(crate\)\s+)?(fn|struct|enum|const|static|trait|type)\s+" + re.escape(n) + r"\b", lambda m: f"{m.group(1)}pub {m.group(2)} {n}", body)
            body = re.sub(r"(?m)^(\s*)pub pub ", r"\1pub ", body)
        if any(owner.get(n) == i for n in exported):
            types = [n for n in exported if owner.get(n) == i and defined[n][0]["kind"] in ("struct", "enum")]
            for t in types:
                # make fields pub inside the struct body
                m = re.search(r"(?m)^pub struct " + re.escape(t) + r"\b[^{;(]*\{", body)
                if m:
                    j = m.end(); depth = 1; k = j
                    while k < len(body) and depth:
                        if body[k] == "{": depth += 1
                        elif body[k] == "}": depth -= 1
                        k += 1
                    inner = body[j:k - 1]
                    inner = re.sub(r"(?m)^(\s+)(?!pub\b)(?!#)(?!//)([a-z_][a-z0-9_]*\s*:)", r"\1pub \2", inner)
                    body = body[:j] + inner + body[k - 1:]
                m = re.search(r"(?m)^pub struct " + re.escape(t) + r"\s*(<[^>]*>)?\s*\(", body)
                if m:
                    j = m.end(); depth = 1; k = j
                    while k < len(body) and depth:
                        if body[k] == "(": depth += 1
                        elif body[k] == ")": depth -= 1
                        k += 1
                    inner = body[j:k - 1]
                    inner = ", ".join(x if x.strip().startswith("pub") else "pub " + x.strip() for x in inner.split(",") if x.strip())
                    body = body[:j] + inner + body[k - 1:]
            # impl methods of exported types become pub
            for t in types:
                for m in list(re.finditer(r"(?m)^impl(?:<[^{]*?>)?\s+" + re.escape(t) + r"\b[^{]*\{", body))[::-1]:
                    j = m.end(); depth = 1; k = j
                    while k < len(body) and depth:
                        if body[k] == "{": depth += 1
                        elif body[k] == "}": depth -= 1
                        k += 1
                    inner = re.sub(r"(?m)^(    )(const fn|fn|const)\b", r"\1pub \2", body[j:k - 1])
                    body = body[:j] + inner + body[k - 1:]
        # imports
        head = list(p.get("doc", [])) + [""]
        for it in uses:
            text = it["body"].strip()
            if text.startswith("use crate::"): continue
            m = re.match(r"use\s+([\w:]+)::\{(.*)\};$", re.sub(r"\s+", " ", text), re.S)
            if m:
                names = [x.strip() for x in m.group(2).split(",") if x.strip()]
                keep = [x for x in names if re.search(r"(?<![\w])" + re.escape(x.split(" as ")[-1].strip()) + r"\b", body)]
                if keep: head.append(f"use {m.group(1)}::{{{', '.join(keep)}}};")
                continue
            m = re.match(r"use\s+([\w:]+)::(\w+);$", text)
            if m:
                if re.search(r"(?<![\w])" + re.escape(m.group(2)) + r"\b", body): head.append(text)
                continue
            head.append(text)
        for extra in p.get("extra_uses", []): head.append(extra)
        for o, names in sorted(need[i].items()):
            head.append(f"use {pieces[o]['crate']}::{{{', '.join(sorted(names))}}};")
        text = "\n".join(head) + "\n\n" + body.strip("\n") + "\n"
        out_text.append((p["out"], text))
        print(f"{p['name']}: {text.count(chr(10))} lines -> {os.path.relpath(p['out'])}")
    if mode == "apply":
        for out, text in out_text:
            os.makedirs(os.path.dirname(out), exist_ok=True)
            open(out, "w", encoding="utf-8").write(text)

main()
