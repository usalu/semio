#!/usr/bin/env python3
"""🧫️ Rename every `🧪️fixtures` directory under `💻️os` to the taxonomy name `🧫️fixtures`
(`🔣️taxonomy.json.testFixturesDirName`) and rewrite every repo-wide reader.

Each renamed directory gets the shortest trailing path anchor that is unique across *all*
`🧪️fixtures` directories in the repository, so a reader string is only rewritten when it
unambiguously names an os directory this pass renamed. Non-anchored leftovers are reported.
"""
import os, re, subprocess, sys, json

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "../../../../../../.."))
OS_REL = "🧰️framework/🛍️products/💻️os"
OLD, NEW = "🧪️fixtures", "🧫️fixtures"
SKIP = {"node_modules", ".git", "dist", "target", "🗑️generated", "🤖️generated"}

def all_fixture_dirs():
    out = []
    for r, d, f in os.walk(ROOT):
        d[:] = [x for x in d if x not in SKIP]
        rel = os.path.relpath(r, ROOT)
        if rel.startswith(".🧬semio"):
            d[:] = []
            continue
        for x in d:
            if x == OLD:
                out.append(os.path.relpath(os.path.join(r, x), ROOT))
    return sorted(out)

def anchors(mine, everything):
    """Shortest trailing-segment anchor per os dir that no other repo dir shares."""
    result = {}
    for p in mine:
        segs = p.split("/")
        for n in range(2, len(segs) + 1):
            a = "/".join(segs[-n:])
            if not any(q != p and q.endswith("/" + a) for q in everything):
                result[p] = a
                break
        else:
            result[p] = p
    return result

def repo_files():
    out = subprocess.run(["git", "ls-files", "-z"], cwd=ROOT, capture_output=True)
    return [p for p in out.stdout.decode().split("\0") if p]

def main():
    apply = "--apply" in sys.argv
    every = all_fixture_dirs()
    mine = [p for p in every if p.startswith(OS_REL + "/")]
    anc = anchors(mine, every)
    subs = sorted(set(anc.values()), key=lambda a: -len(a))
    print(f"[DEBUG] repo 🧪️fixtures dirs={len(every)} os={len(mine)} anchors={len(subs)}")
    for p, a in sorted(anc.items()):
        if a.count("/") > 1:
            print(f"[DEBUG] long anchor {a}")

    touched, hits = [], 0
    for rel in repo_files():
        if rel.startswith(".🧬semio/") or any(part in SKIP for part in rel.split("/")):
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
        new, n = text, 0
        for a in subs:
            new, k = re.subn(re.escape(a), a[: -len(OLD)] + NEW, new)
            n += k
        if n:
            hits += n
            touched.append((rel, n))
            if apply:
                open(ap, "w", encoding="utf-8").write(new)
    print(f"[DEBUG] readers: files={len(touched)} substitutions={hits}")

    if apply:
        for p in sorted(mine, key=lambda q: -q.count("/")):
            os.rename(os.path.join(ROOT, p), os.path.join(ROOT, p[: -len(OLD)] + NEW))
        print(f"[DEBUG] renamed {len(mine)} directories")

    json.dump({"os_dirs": mine, "anchors": anc, "touched": touched},
              open(os.path.join(HERE, "🗑️generated", "wp4b-os-fixture-rename.json"), "w"),
              ensure_ascii=False, indent=1)
    for rel, n in sorted(touched, key=lambda x: -x[1]):
        print(f"   {n:4d}  {rel}")

main()
