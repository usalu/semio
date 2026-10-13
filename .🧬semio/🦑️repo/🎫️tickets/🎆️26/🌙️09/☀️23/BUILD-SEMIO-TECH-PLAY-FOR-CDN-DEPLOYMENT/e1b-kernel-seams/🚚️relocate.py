#!/usr/bin/env python3
"""🚚️ Relocates named top-level items (with every impl of a moved type and their leading docs and attributes) from
one os store source file into a target module file, without leaving any re-export behind (wave E1b).

Usage: relocate.py --from SRC.rs --to DST.rs --items A,B,C [--doc-file HEADER.txt] [--write]
A new target file starts with the header and the source file's import preamble (pruned later by rustc); an existing
target gets the items appended. Inherent impls and impls of foreign traits follow their self type; impls of a trait
declared in the source file follow the trait when the trait moves. Paths are not rewritten: compile errors drive that.
"""
import argparse, importlib.util, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("ri", os.path.join(HERE, "🧱️rust_items.py"))
ri = importlib.util.module_from_spec(spec); spec.loader.exec_module(ri)
NAMED = {"fn", "struct", "enum", "trait", "union", "type", "static", "const", "macro_rules"}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--from", dest="src", required=True)
    ap.add_argument("--to", dest="dst", required=True)
    ap.add_argument("--items", required=True)
    ap.add_argument("--doc-file")
    ap.add_argument("--write", action="store_true")
    a = ap.parse_args()
    names = {n.strip() for n in a.items.split(",") if n.strip()}
    raw = open(a.src, encoding="utf-8").read()
    cl = ri.clean(raw)
    its, _ = ri.items(raw, cl, 0, len(cl))
    found = {it["name"] for it in its if it["kind"] in NAMED and it["name"] in names}
    if names - found: sys.exit(f"not found at top level: {sorted(names - found)}")
    local_traits = {it["name"] for it in its if it["kind"] == "trait"}
    chosen = []
    for it in its:
        if it["kind"] in NAMED and it["name"] in names: chosen.append(it)
        elif it["kind"] == "impl":
            tr, st = it.get("trait"), it.get("self")
            if tr in names or (st in names and not (tr in local_traits and tr not in names)): chosen.append(it)
    moved, cuts = [], []
    for it in chosen:
        lead = raw[it["start"]:it["head"]]
        keep = "\n".join(l for l in lead.split("\n") if re.match(r"\s*//#(end)?region", l))
        body_lead = "\n".join(l for l in lead.split("\n") if not re.match(r"\s*//#(end)?region", l)).strip("\n")
        moved.append((body_lead + "\n" if body_lead else "") + raw[it["head"]:it["end"]])
        cuts.append((it["start"], it["end"], keep))
    out, pos = [], 0
    for s, e, keep in cuts:
        out.append(raw[pos:s]); pos = e
        if keep.strip(): out.append("\n" + keep.strip("\n"))
    out.append(raw[pos:])
    rest = re.sub(r"\n{3,}", "\n\n", "".join(out))
    body = "\n\n".join(m.strip("\n") for m in moved) + "\n"
    if os.path.exists(a.dst):
        content = open(a.dst, encoding="utf-8").read().rstrip("\n") + "\n\n" + body
    else:
        header = open(a.doc_file, encoding="utf-8").read().rstrip("\n") + "\n" if a.doc_file else ""
        pre = []
        for it in its:
            if it["kind"] != "use": continue
            gates = re.findall(r"^\s*(#\[cfg\(.*\)\])\s*$", raw[it["start"]:it["head"]], re.M)
            pre.append((gates[-1] + "\n" if gates else "") + raw[it["head"]:it["end"]].strip())
        content = header + "\n".join(dict.fromkeys(pre)) + "\n\n" + body
    print(f"[relocate] {len(chosen)} items, {body.count(chr(10))} lines -> {a.dst}")
    for it in chosen: print(f"   {it['kind']:8} {it.get('name') or (str(it.get('trait')) + ' for ' + str(it.get('self')))} @{ri.line_of(raw, it['head'])}")
    if a.write:
        os.makedirs(os.path.dirname(a.dst), exist_ok=True)
        open(a.dst, "w", encoding="utf-8").write(content)
        open(a.src, "w", encoding="utf-8").write(rest)


if __name__ == "__main__":
    main()
