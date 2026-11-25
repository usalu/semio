#!/usr/bin/env bash
# 🔁 After the other parts: re-syncs the copy, then runs the wire crates and the editors that need the history and insert fixes. Waits for copy-chain6.sh.
set -u
T="$(cd "$(dirname "$0")" && pwd)"; G="$T/🗑️generated"; A="🏅️standards/🔖️1/🪆️subsets/✳️any"; N="✏️s/🔌️plugins/📕️norm/🗿️artifacts"; P="📦️packages/🦀️rust"
export RUST_MIN_STACK=67108864
until grep -q '^done6' "$G/copy-summary.txt" 2>/dev/null; do sleep 30; done
"$T/copy-sync.sh" > "$G/copy-sync7.log" 2>&1
one() { "$T/copy-test.sh" "$@"; code=$?; echo "$(date +%H:%M) $1 exit $code $(grep -a '^test result' "$G/$1.log" | tail -3 | tr '\n' ' ') $(grep -aE '^error' "$G/$1.log" | head -2 | cut -c1-140 | tr '\n' ' ')" >> "$G/copy-summary.txt"; }
W=$(grep -rl '^name = "semio-os-store-operation-wire"' "$T/../../../../../../../🧰️framework/🛍️products/💻️os" --include=Cargo.toml 2>/dev/null | head -1); W="$(dirname "${W#"$(cd "$T/../../../../../../.." && pwd)"/}")"
one cp-wire "$W" test -p semio-os-store-operation-wire --offline -j 2
for f in ⚖️en1990 🌍️en1997 🫨️en1998 🏛️en1992 ⚡️din18599; do fam="${f//[^a-z0-9]/}"; one "cp2-editor-$fam" "$N/$f/$A/✏️editor/$P" test -p "semio-s-artifact-norm-$fam-editor" --offline --no-fail-fast -j 2 -- --test-threads=4 --skip latency_bench; done
one cp2-livecore "✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🧲️live-core/$P" test -p semio-s-plugin-norm-app-surface-live-core --offline --no-fail-fast -j 2
for f in 🏋️en1991 🏛️en1992 🪨️en1996 🌍️en1997 🧱️din4108 🌬️din16798 🧩️en1994; do fam="${f//[^a-z0-9]/}"; one "cp2-scene-$fam" "$N/$f/$A/🎬️scene/$P" test -p "semio-s-artifact-norm-$fam-scene" --offline --no-fail-fast -j 2; done
echo done7 >> "$G/copy-summary.txt"
