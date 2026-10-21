#!/usr/bin/env python3
# 🔎 Static scan: for every owned generator contract, does a literal module input of its producer target (named inputs expanded) import, directly or through relative imports, a file under an output root of the same producer? Those targets are refused by the generated-task admission ("cannot consume its own output") unless they declare sourceInputObservation: freshness.
import json, os, re, subprocess, sys
ROOT = "/Users/ueli/Documents/semio"; L = ROOT + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
tax = json.load(open(L + "/🔣️taxonomy.json"))["generatorContracts"]
nx = json.load(open(ROOT + "/nx.json")).get("namedInputs", {})
manifests = {}
for f in subprocess.run(["git", "-C", ROOT, "-c", "core.quotepath=false", "ls-files", "-z", "--", "*📋️project.json"], capture_output=True).stdout.decode().split("\0"):
    if not f or f.startswith(".🧬semio/"): continue
    try: p = json.load(open(os.path.join(ROOT, f)))
    except Exception: continue
    manifests[p["name"]] = (f, p)
SPEC = re.compile(r"""(?:from\s*|import\s*\(\s*|import\s*|require\s*\(\s*)["'](\.{1,2}/[^"']+)["']""")
def expand(names, project, seen=()):
    out = []
    for n in names:
        if isinstance(n, str) and not n.startswith(("{", "!", "^")) and (n in project.get("namedInputs", {}) or n in nx) and n not in seen:
            out += expand((project.get("namedInputs", {}).get(n) or nx.get(n)), project, seen + (n,))
        else: out.append(n)
    return out
rows = []
for key, c in tax.items():
    if c.get("ownership") != "owned": continue
    roots = [r["path"] for r in c["outputRoots"]]
    for tid in filter(None, [c.get("target"), c.get("previewTarget")]):
        proj, tname = tid.rsplit(":", 1)
        if proj not in manifests: rows.append((tid, "no manifest")); continue
        f, p = manifests[proj]; t = (p.get("targets") or {}).get(tname)
        if not t: rows.append((tid, "no target")); continue
        if ((t.get("metadata") or {}).get("semio") or {}).get("sourceInputObservation"): continue
        rootdir = os.path.dirname(f)
        entries = []
        for i in expand(t.get("inputs") or [], p):
            if isinstance(i, str) and re.search(r"\.[cm]?[jt]sx?$", i) and not re.search(r"[*?{}\[\]]", re.sub(r"^\{(?:projectRoot|workspaceRoot)\}", "", i)) and re.match(r"^\{(?:projectRoot|workspaceRoot)\}/", i):
                entries.append(os.path.normpath(os.path.join(ROOT, i.replace("{projectRoot}", rootdir).replace("{workspaceRoot}", "."))))
        seen, pending, hit = set(), list(entries), []
        while pending:
            file = pending.pop()
            if file in seen or not os.path.isfile(file): continue
            seen.add(file)
            try: text = open(file, encoding="utf8").read()
            except Exception: continue
            for m in SPEC.finditer(text):
                target = os.path.normpath(os.path.join(os.path.dirname(file), m.group(1)))
                rel = os.path.relpath(target, ROOT)
                if any(rel == r or rel.startswith(r + "/") for r in roots): hit.append((os.path.relpath(file, ROOT), m.group(1))); continue
                for ext in ("", ".ts", ".tsx", ".js", ".mjs", "/🟦️.ts", "/🟨️.mjs"):
                    if os.path.isfile(target + ext): pending.append(target + ext); break
        if hit: rows.append((tid, hit[:3]))
for tid, why in rows: print(tid, "->", why)
print("scanned", sum(1 for c in tax.values() if c.get("ownership") == "owned"), "owned contracts;", len(rows), "offending")
