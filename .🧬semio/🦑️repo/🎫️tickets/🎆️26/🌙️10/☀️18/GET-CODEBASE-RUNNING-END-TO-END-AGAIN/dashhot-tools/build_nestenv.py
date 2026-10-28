#!/usr/bin/env python3
"""🎛️ Rebuilds `staged/nestenv` from the CURRENT live files: the per-action Bazel control URL a nested `repo run` inherits is authority (the cancellation channel of the parent action), never a planner input, so the world witness records no read of it and the nested plan of `repo-dashboard-rs:compile` can be reused. usage: python3 -I build_nestenv.py [live-root]"""
import hashlib, json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
LIVE = os.path.abspath(sys.argv[1]) if len(sys.argv) > 1 else "/Users/ueli/Documents/semio"
STAGE = os.path.join(HERE, "staged", "nestenv")
W = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔭️witness/"
def sha(data): return hashlib.sha256(data).hexdigest()
def schema(text):
    old = '"authorityEnvironment": ["SEMIO_SCRIPT_PROCESS_INVOCATION", "SEMIO_HOST_CAPACITY_GRANT", "SEMIO_HOST_CAPACITY_PLAN"]'
    if text.count(old) != 1: sys.exit("anchor drift: authorityEnvironment")
    return text.replace(old, old[:-1] + ', "SEMIO_BAZEL_CONTROL_URL"]')
def law(text):
    old = 'SEMIO_REPO_SEAL:"sealed",SEMIO_REPO_IMPLEMENTATION:"rust"});'
    new = 'SEMIO_REPO_SEAL:"sealed",SEMIO_REPO_IMPLEMENTATION:"rust",SEMIO_BAZEL_CONTROL_URL:"tcp://127.0.0.1:1/x",SEMIO_HOST_CAPACITY_GRANT:"g"});'
    old2 = 'for(const name of["SEMIO_BUILD_PARALLEL","SEMIO_LEASE_PRIORITY","SEMIO_REPO_SEAL","SEMIO_REPO_IMPLEMENTATION","PWD","SHLVL","_"])'
    new2 = 'for(const name of["SEMIO_BUILD_PARALLEL","SEMIO_LEASE_PRIORITY","SEMIO_REPO_SEAL","SEMIO_REPO_IMPLEMENTATION","SEMIO_BAZEL_CONTROL_URL","SEMIO_HOST_CAPACITY_GRANT","PWD","SHLVL","_"])'
    for a, b in ((old, new), (old2, new2)):
        if text.count(a) != 1: sys.exit("anchor drift: " + a[:50])
        text = text.replace(a, b)
    return text
rows = {W + "🧬️schema/🔣️.json": schema, W + "🧪️tests/🟦️.ts": law}
shutil.rmtree(STAGE, ignore_errors=True); os.makedirs(os.path.join(STAGE, "base")); os.makedirs(os.path.join(STAGE, "files"))
manifest = {}
for index, (rel, change) in enumerate(rows.items()):
    base = open(os.path.join(LIVE, rel), "rb").read(); new = change(base.decode("utf8")).encode("utf8")
    key = f"{index:02d}{os.path.splitext(rel)[1]}"
    open(os.path.join(STAGE, "base", key), "wb").write(base); open(os.path.join(STAGE, "files", key), "wb").write(new)
    manifest[rel] = {"file": key, "base": sha(base), "new": sha(new)}
json.dump(manifest, open(os.path.join(STAGE, "manifest.json"), "w", encoding="utf8"), indent=1, ensure_ascii=False)
stage = open(os.path.join(HERE, "stage.py"), encoding="utf8").read().replace("🧊 Staged batch `fresh` (lane dashhot): a dashboard build whose installation is current for its captured sources and its executable's own Cargo dep-info runs no nested compile; the installation records that dep-info.", "🎛️ Staged batch `nestenv` (lane dashhot): the inherited per-action Bazel control URL is witness authority, so a nested `repo run` inside a Bazel action can reuse its plan.")
open(os.path.join(STAGE, "stage.py"), "w", encoding="utf8").write(stage)
open(os.path.join(STAGE, "verify.sh"), "w", encoding="utf8").write("""#!/bin/sh
# 🎛️ Laws of batch `nestenv` after apply: sh verify.sh <root>  (witness suite incl. the extended ignored/authority law, plan-reuse suites)
set -u
root=$(cd "$1" && pwd -P); here=$(cd "$(dirname "$0")" && pwd); fail=0
python3 -I "$here/stage.py" check "$root" | grep -v "^applied"; python3 -I "$here/stage.py" check "$root" >/dev/null || fail=1
cd "$root"; B=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build
for file in "$B/🔭️witness/🧪️tests/🟦️.ts" "$B/🔧️tools/🏗️bazel/♻️plan-reuse/🧪️tests/🟦️.ts" "$B/🔧️tools/🏗️bazel/♻️plan-reuse/🧪️tests/🧭️ignore-sources/🟦️.ts"; do
  out=$(bun test --timeout 300000 "$file" 2>&1); echo "${file##*/build/}: $(echo "$out" | grep -E "^ *[0-9]+ (pass|fail)" | tr '\\n' ' ')"; echo "$out" | grep -E "^\\(fail\\)" | cut -c1-200
  echo "$out" | grep -qE "^ *[1-9][0-9]* fail" && fail=1; echo "$out" | grep -qE "^ *[1-9][0-9]* pass" || fail=1
done
[ $fail -eq 0 ] && echo "VERIFY nestenv: GREEN" || echo "VERIFY nestenv: RED"; exit $fail
""")
print(f"staged {len(manifest)} files in {STAGE}")
