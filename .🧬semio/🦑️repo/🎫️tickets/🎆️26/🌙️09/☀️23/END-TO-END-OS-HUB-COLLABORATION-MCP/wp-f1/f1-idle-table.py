#!/usr/bin/env python3
"""📊️ F1 — renders the idle-cost census JSON files as one markdown table per role (kind rows), for the report."""
import json, sys

def rows(path):
    try:
        return json.load(open(path))["rows"]
    except FileNotFoundError:
        return []

def fmt(row, before=None):
    i, h = row.get("idle"), row.get("heap") or {}
    if not i:
        return f"| {row['key']} | — | — | — | — | — | — | — | {row.get('openDetail') or 'not measured'} |"
    verdict = "**OFFENDER**" if row.get("offender") else "clean"
    note = ""
    if before and before.get("idle"):
        b = before["idle"]
        note = f"before F1: {b['mainFramesPerSec']} fps, {b['mainBusyPct']} % → fixed"
    ac = row.get("afterClose") or {}
    return f"| {row['key']} | {i['mainBusyPct']} / {i['mainCpuPct']} | {i['mainFramesPerSec']} | {i['rafPerSec']} | {i['compositorDrawsPerSec']} | {i['workersBusyPct']} | {h.get('jsUsedMB','-')} | {h.get('mainWasmMB','-')} / {h.get('workersBackingMB','-')} | {verdict}{(' — ' + note) if note else ''}{' — after close ' + str(ac.get('taskPct')) + ' %' if (ac.get('taskPct') or 0) > 1 else ''} |"

def main():
    editors, viewers = rows("generated/f1-idle-r1-editors.json"), rows("generated/f1-idle-r1-viewers.json")
    before = {f"{r['key']}#{r['role']}": r for r in rows("generated/f1-idle-r1-loops.json")}
    after = {f"{r['key']}#{r['role']}": r for path in ("generated/f1-idle-r1-loops-after.json", "generated/f1-idle-r2-hung-editors.json", "generated/f1-idle-r2-hung-viewers.json", "generated/f1-idle-r2-curation.json") for r in rows(path)}
    for title, table in (("editor", editors), ("viewer", viewers)):
        seen = {}
        for r in table:
            seen[r["key"]] = r
        for key, r in after.items():
            k, role = key.split("#")
            if role == title:
                seen[k] = r
        print(f"\n#### {title}s ({len(seen)} kinds)\n")
        print("| kind | main busy % (wall / cpu) | main frames/s | rAF/s | compositor draws/s | workers busy % | JS heap MB | wasm MB (main / workers) | verdict |")
        print("|---|---|---|---|---|---|---|---|---|")
        for k in sorted(seen):
            print(fmt(seen[k], before.get(f"{k}#{title}") if before.get(f"{k}#{title}", {}).get("offender") else None))

main()
