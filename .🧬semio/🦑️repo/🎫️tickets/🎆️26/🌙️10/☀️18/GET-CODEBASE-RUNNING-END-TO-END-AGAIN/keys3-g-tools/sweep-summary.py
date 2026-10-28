#!/usr/bin/env python3
"""keys3-g: one line per oracle report of the sweep (tags k3-*, k3b-*, ...): target id, clean/passed, rounds, and the undeclared reads by class. Streams the small reports only.
Classes: C1 module-resolution config (package.json, tsconfig.json), PJ a project.json, CG cargo file (Cargo.toml/lock/toolchain/config), UI a file under the ui module, LIB the build library, SRC other source, OTHER.
usage: sweep-summary.py [tag-prefix] -> prints a markdown table"""
import json, glob, os, re, sys
G = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/undeclared-g/reports"
prefix = sys.argv[1] if len(sys.argv) > 1 else "k3"
def klass(path):
    name = path.rsplit("/", 1)[-1]
    if name in ("package.json", "tsconfig.json"): return "C1"
    if name == "📋️project.json": return "PJ"
    if name in ("Cargo.toml", "Cargo.lock", "rust-toolchain.toml", "config.toml"): return "CG"
    if "🔨️modules/📚️library/" in path: return "LIB"
    if "🖱️ui/" in path: return "UI"
    if re.search(r"\.(ts|tsx|mjs|js|rs|json|css|html)$", name): return "SRC"
    return "OTHER"
rows = []
for plan in sorted(glob.glob(f"{G}/{prefix}*.plan.json"), key=os.path.getmtime):
    tag = os.path.basename(plan)[:-len(".plan.json")]
    report = f"{G}/{tag}.json"
    try: p = json.load(open(plan, encoding="utf8"))
    except Exception: continue
    if not os.path.exists(report): rows.append((tag, p.get("id"), "running", "", "")); continue
    r = json.load(open(report, encoding="utf8")); learned = [x for x in r.get("learned", []) if isinstance(x, str)]
    classes = {}
    for path in learned: classes.setdefault(klass(path), []).append(path)
    rr = r.get("rounds"); rounds = len(rr) if isinstance(rr, list) else rr; last = rr[-1] if isinstance(rr, list) and rr else {}; ending = f" exit={last.get('exit')} \"{(last.get('tail') or '').strip().splitlines()[-1][:70] if (last.get('tail') or '').strip() else ''}\"" if last else ""
    rows.append((tag, p.get("id"), f"clean={r.get('clean')} passed={r.get('passed')} rounds={rounds}{ending}", ", ".join(f"{k} {len(v)}" for k, v in sorted(classes.items())), "; ".join(x for k in ("PJ", "CG", "SRC", "UI", "LIB", "OTHER") for x in classes.get(k, [])[:3])[:300]))
print("| tag | target | result | undeclared reads | non-C1 examples |\n|---|---|---|---|---|")
for row in rows: print("| " + " | ".join(str(x) for x in row) + " |")
