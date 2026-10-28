"""📦 Builds the staged batches of lane germanqa from the CURRENT live files: python3 -I stage_all.py <root> <staged-dir>
One batch per catalog group (`process`, `libbuild`, `libcache`, `libother`, `mcp`, `modules`, `docs`), file-disjoint; each has manifest.json {rel:{file,base,new}}, files/, base/, stage.py (mode-keeping template), files.json, tests.txt and verify.sh."""
import hashlib, json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import build

C = os.path.dirname(HERE)
TEMPLATE = os.path.join(C, "xplat5-tools", "stage.template.py")
P = "🧰️framework/🛍️products/🦑️repo/"
GROUPS = ["process", "libbuild", "libcache", "libother", "mcp", "modules", "docs"]


def group(rel):
    if rel.startswith("🧰️framework/🔨️modules/🏃️process/"): return "process"
    if rel.endswith(".md"): return "docs"
    m = rel[len(P):] if rel.startswith(P) else rel
    if m.startswith("🔨️modules/📚️library/"):
        top = m[len("🔨️modules/📚️library/"):].split("/")[0]
        return {"🏗️build": "libbuild", "⚡️caching": "libcache"}.get(top, "libother")
    if m.startswith("🔨️modules/💻️client/🔌️mcp/") or m.startswith("🔨️modules/🔌️mcp/"): return "mcp"
    return "modules"


def test_dirs(root, rels):
    out = []
    for rel in rels:
        d = os.path.dirname(rel)
        cands = []
        if "/🧪️tests" in d + "/": cands.append(d)
        cur = d
        while cur and len(cur.split("/")) > 4:
            cands.append(os.path.join(cur, "🧪️tests")); cur = os.path.dirname(cur)
        for c in cands:
            if os.path.exists(os.path.join(root, c, "🟦️.ts")) and c not in out: out.append(c); break
    return out


VERIFY = """#!/bin/sh
# 🇩🇪 Laws of batch `{name}` (lane germanqa): VERIFY_ROOT=<repository or mirror root> sh verify.sh   (default root: the checkout above the ticket)
# before apply the law is RED on this batch's files (counts printed, informational); after apply it must be GREEN, then the test directories next to the changed files run.
set -eu
here=$(cd "$(dirname "$0")" && pwd)
root=${{VERIFY_ROOT:-$(python3 -I -c 'import os,sys;h=sys.argv[1]
while not os.path.isdir(os.path.join(h,"🧰️framework")) and h!="/":h=os.path.dirname(h)
print(h)' "$here")}}
law="$here/../../law_germanqa.py"
python3 -I "$law" --selftest
python3 -I "$here/../../law_edits.py" "$here"
echo "### before apply (expected RED unless already applied)"
python3 -I "$here/stage.py" check --root "$root" >/dev/null 2>&1 && python3 -I "$law" "$root" --files-from "$here/files.json" --limit 1 | tail -1 || true
python3 -I "$here/stage.py" apply --root "$root"
python3 -I "$here/stage.py" check --root "$root"
python3 -I "$law" "$root" --files-from "$here/files.json" --limit 5
cd "$root"
if [ -n "${{SEMIO_TEST_ARTIFACT_DIR:-}}" ] && [ -s "$here/tests.txt" ]; then
  while IFS= read -r d; do [ -n "$d" ] && (cd "$root/$d" && echo "### $d" && bun test --timeout 60000 ./🟦️.ts 2>&1 | grep -E '^\\(fail\\)|^ *[0-9]+ (pass|fail)$' || true); done < "$here/tests.txt"
else echo "tests skipped: set SEMIO_TEST_ARTIFACT_DIR (short path under the ticket generated folder) to run $(wc -l < "$here/tests.txt") test directories"; fi
echo "batch {name}: law GREEN"
"""


def main():
    root, out = os.path.abspath(sys.argv[1]), os.path.abspath(sys.argv[2])
    changes = build.build(root)
    by = {g: {} for g in GROUPS}
    for rel, row in changes.items(): by[group(rel)][rel] = row
    if os.path.isdir(out): shutil.rmtree(out)
    summary = {}
    for g, rows in by.items():
        if not rows: continue
        d = os.path.join(out, g)
        os.makedirs(os.path.join(d, "files")); os.makedirs(os.path.join(d, "base"))
        manifest = {}
        for i, (rel, (base, new, n)) in enumerate(sorted(rows.items())):
            name = f"{i:03d}{os.path.splitext(rel)[1]}"
            open(os.path.join(d, "base", name), "wb").write(base); open(os.path.join(d, "files", name), "wb").write(new)
            manifest[rel] = {"file": name, "base": hashlib.sha256(base).hexdigest(), "new": hashlib.sha256(new).hexdigest()}
        json.dump(manifest, open(os.path.join(d, "manifest.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
        json.dump(sorted(rows), open(os.path.join(d, "files.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
        open(os.path.join(d, "tests.txt"), "w", encoding="utf-8").write("\n".join(test_dirs(root, sorted(rows))) + "\n")
        shutil.copyfile(TEMPLATE, os.path.join(d, "stage.py"))
        t = open(os.path.join(d, "stage.py"), encoding="utf-8").read().replace("Staged batch `universe` (lane rustedit): print's Bazel plan becomes reusable although a dependency reads a root-anchored pattern.", f"Staged batch `{g}` (lane germanqa): German text review (real umlauts, one glossary term per concept, grammar).")
        open(os.path.join(d, "stage.py"), "w", encoding="utf-8").write(t)
        open(os.path.join(d, "verify.sh"), "w", encoding="utf-8").write(VERIFY.format(name=g))
        summary[g] = len(rows)
    print(summary)


main()
