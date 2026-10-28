#!/bin/sh
# 🧪 FOR gate-g: xplat3-g batch verify (rebase-g). `sh C/xplat3-g-tools/verify.sh [short-root]`: builds a short-root MIRROR (clones of the process/runtime modules and of the repo product modules, never the live tree: path limit 256), `apply.py check` (19 x ready, no DRIFT), `apply.py apply --root <mirror>` (backups go beside the mirror), then the 9 single-file laws of the batch there, each through the graph-free official envelope (`ver-f-tools/proc-run.ts`: leases/native laws refuse a plain bun test; one `pass/fail` line each; the runtime test has 2 expected failures that are red on the live tree too), then `revert` and a final `check`. Exit 1 on DRIFT, a failed apply/revert or a failing law.
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; T="$C/xplat3-g-tools"
M="🧰️framework/🔨️modules"; R="🧰️framework/🛍️products/🦑️repo/🔨️modules"; L="$R/📚️library"
OV="${1:-/private/tmp/claude-501/rb/xplat3}"
python3 -I "$T/apply.py" check | awk '{print $1}' | sort | uniq -c | tr -s ' \n' ' '; echo "(live, read-only)"
python3 -I "$T/apply.py" check | grep -q DRIFT && { echo "DRIFT on the live tree"; exit 1; }
sh "$C/rebase-g-tools/mirror.sh" "$OV" "$M" "$R" >/dev/null || exit 1
python3 -I "$T/apply.py" apply --root "$OV" >/dev/null || { echo "apply FAILED"; exit 1; }
fail=0
for f in "$L/🏗️build/🔭️witness/🧪️tests/🟦️.ts" "$M/📁️filesystem/🧾️observation/🔁️churn/🧪️tests/🔒️busy/🟦️.ts" "$R/🚀️launcher/🧪️tests/📥️install-contract/🟦️.ts" "$M/⚙️runtime/🧪️tests/🟦️.ts" \
  "$L/🏗️build/🧹️cache-gc/👯️dedupe/🧪️tests/🟦️.ts" "$M/🏃️process/🔒️leases/♻️retirement/🧹️sweep/🧪️tests/🟦️.ts" "$M/🏃️process/🔒️leases/🚪️admission/🧪️tests/🟦️.ts" \
  "$M/⚙️runtime/🟢️node/🧲️native/🧪️tests/🟦️.ts" "$M/⚙️runtime/🟢️node/🧲️native/🏗️addon/🧪️tests/🟦️.ts"; do
  out=$( VF_WALL=300 bun "$C/ver-f-tools/proc-run.ts" file "$OV/$f" 2>&1 | grep -E "^ *[0-9]+ (pass|fail|skip)$" | tr -s ' \n' ' ' )
  want=" 0 fail"; case "$f" in "$M/⚙️runtime/🧪️tests/🟦️.ts") want=" 2 fail";; esac
  printf '%s  %s (expected%s)\n' "${f#🧰️framework/}" "$out" "$want"; case "$out" in *"$want"*) ;; *) fail=1;; esac
done
python3 -I "$T/apply.py" revert --root "$OV" >/dev/null || { echo "revert FAILED"; fail=1; }
printf 'mirror reverted: '; python3 -I "$T/apply.py" check --root "$OV" | awk '{print $1}' | sort | uniq -c | tr -s ' \n' ' '; echo
rm -rf "$OV"; exit $fail
