#!/usr/bin/env python3
"""🧭️ Second pass of the os fixture rename: rewrite *relative* `🧪️fixtures` references
(`include_str!`, TS imports, `join(dir, "…")`) whose resolved target no longer exists but
whose `🧫️fixtures` twin does. Anything unresolvable is reported, never guessed.
"""
import os, re, subprocess, sys, json

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "../../../../../../.."))
OLD, NEW = "🧪️fixtures", "🧫️fixtures"
SKIP = {"node_modules", ".git", "dist", "target", "🗑️generated", "🤖️generated"}
# a quoted path literal that mentions the old directory name
LIT = re.compile(r'"((?:[^"\\\n]|\\.)*' + re.escape(OLD) + r'(?:[^"\\\n]|\\.)*)"')

def repo_files():
    out = subprocess.run(["git", "ls-files", "-z"], cwd=ROOT, capture_output=True)
    return [p for p in out.stdout.decode().split("\0") if p]

def main():
    apply = "--apply" in sys.argv
    changed, unresolved = [], []
    for rel in repo_files():
        if rel.startswith(".🧬semio/") or any(p in SKIP for p in rel.split("/")):
            continue
        ap = os.path.join(ROOT, rel)
        if not os.path.isfile(ap):
            continue
        try:
            text = open(ap, encoding="utf-8").read()
        except (UnicodeDecodeError, IsADirectoryError):
            continue
        if OLD not in text:
            continue
        base = os.path.dirname(ap)
        hits = 0

        def repl(m):
            nonlocal hits
            lit = m.group(1)
            if lit.startswith("/") or "://" in lit:
                unresolved.append((rel, lit, "absolute-or-url"))
                return m.group(0)
            target = os.path.normpath(os.path.join(base, lit))
            twin = os.path.normpath(os.path.join(base, lit.replace(OLD, NEW)))
            if not os.path.exists(target) and os.path.exists(twin):
                hits += 1
                return '"' + lit.replace(OLD, NEW) + '"'
            if os.path.exists(target):
                return m.group(0)                      # a real, still-existing 🧪️fixtures path
            unresolved.append((rel, lit, "unresolved"))
            return m.group(0)

        new = LIT.sub(repl, text)
        if hits:
            changed.append((rel, hits))
            if apply:
                open(ap, "w", encoding="utf-8").write(new)
    print(f"[DEBUG] relative pass: files={len(changed)} substitutions={sum(n for _, n in changed)}")
    for rel, n in changed:
        print(f"   {n:3d}  {rel}")
    print(f"[DEBUG] unresolved literals={len(unresolved)}")
    for rel, lit, why in unresolved[:60]:
        print(f"   {why:22s} {rel} :: {lit}")
    json.dump({"changed": changed, "unresolved": unresolved},
              open(os.path.join(HERE, "🗑️generated", "wp4b-os-fixture-relative.json"), "w"),
              ensure_ascii=False, indent=1)

main()
