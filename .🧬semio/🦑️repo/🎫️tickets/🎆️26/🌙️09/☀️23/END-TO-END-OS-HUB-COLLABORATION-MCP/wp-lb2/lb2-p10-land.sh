#!/bin/zsh
# ✏️ LB2 p10 landing (rule 22, test-only): apply → native `check --tests` of the stdio package (compile-atomic, reverts on red)
# → the `editor_catalog` law on the live tree for the record (red classes left to p9/p11/p12, which land in T6) → re-run the
# p9/p11/p12 dry runs (L1's precondition). Capture: .🧬semio/🌐hub/s14-lb2-captures/<name>.txt
setopt no_bg_nice
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🌐hub/s14-lb2-captures"
capture="$C/$1.txt"
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-lb2-target"
{
echo "START $(date '+%F %T') apply"
python3 .tmp-ticket/wp-lb2/lb2-p10-editor-catalog.py --write || exit 3
zsh .tmp-ticket/📜️fleet-mutex.sh native lb2 -- zsh -c 'nice -n 15 cargo check --message-format short -p semio-s-plugin-stdio --tests; echo "CHECK rc=$?"' > "$C/$1-check.txt" 2>&1
tail -3 "$C/$1-check.txt"
if ! /usr/bin/grep -q "CHECK rc=0" "$C/$1-check.txt"; then python3 .tmp-ticket/wp-lb2/lb2-p10-editor-catalog.py --revert; echo "P10 REVERTED (check)"; exit 4; fi
echo "LANDED $(date '+%F %T')"
zsh .tmp-ticket/📜️fleet-mutex.sh native lb2 -- zsh -c 'nice -n 15 cargo test -p semio-s-plugin-stdio --test editor_catalog --no-fail-fast -- --test-threads 4; echo "LAW rc=$?"' > "$C/$1-law.txt" 2>&1
/usr/bin/grep -E "^test result|LAW rc=" "$C/$1-law.txt"
for s in lb2-p9-hosted-artifacts lb2-p11-editor-documents lb2-p12-document-schema-identity; do echo "$s: $(python3 .tmp-ticket/wp-lb2/$s.py --dry-run | tail -1)"; done
echo "END $(date '+%F %T')"
} > "$capture" 2>&1
