#!/usr/bin/env python3
"""📋️ sf3: turns the `--check-applied` summary and logs into the markdown table of the result file."""
import json, os, re
F = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
G = F + "/🗑️generated"
rows = []
for line in open(f"{G}/sf3-check-summary.txt", encoding="utf-8"):
    m = re.match(r"(\w{8}) (\S+) exit=(\d+) seconds=(\d+) tries=(\d+)", line)
    if not m:
        continue
    key, art, rc, sec, tries = m.groups()
    text = open(f"{G}/sf3-ck-{key}.log", encoding="utf-8", errors="replace").read()
    i = text.find('\n{\n  "base"')
    info = {}
    if i >= 0:
        j = text.find("\n}\n", i) + 2
        try:
            info = json.loads(text[i:j])
        except Exception:
            info = {}
    lay = info.get("layout", {})
    n, t = lay.get("native", {}), lay.get("--test", {})
    short = art.split("/")[2] + "/" + art.split("/")[4]
    note = "" if info else ("no report: " + (re.findall(r"Missing Rust source input[^\n]{0,90}", text) or ["see log"])[0])
    rows.append((short, rc, sec, n, t, len(info.get("admission", [])), len(info.get("shared", [])), len(info.get("siblings", [])), note))
print("| Artifact | Exit | s | Native base / combined / extra | Test base / combined / extra | Hidden | Shared | Siblings |")
print("|---|---:|---:|---|---|---:|---:|---:|")
for short, rc, sec, n, t, adm, sh, sib, note in sorted(rows):
    f = lambda d: f"{d.get('base','?')} / {d.get('combined','?')} / {d.get('onlyCombined','?')}"
    print(f"| `{short}` | {rc} | {sec} | {f(n)} | {f(t)} | {adm} | {sh} | {sib} |" + (f" {note}" if note else ""))
print("\nartifacts", len(rows), "exit0", sum(1 for r in rows if r[1] == "0"), "nonzero", [r[0] for r in rows if r[1] != "0"])
