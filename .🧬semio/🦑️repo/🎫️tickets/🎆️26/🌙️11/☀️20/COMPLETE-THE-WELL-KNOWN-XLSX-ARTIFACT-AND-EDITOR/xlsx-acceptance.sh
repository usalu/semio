#!/bin/zsh
# 🧾 Reproducible XLSX editor acceptance: native gates through `repo run`, artifact-acceptance contract test, then both browser journeys in en and de against a served S shell.
# Usage: xlsx-acceptance.sh [serve-url]   (default http://127.0.0.1:6072/, a shell served by claude-fulls-serve.sh with the rebuilt stdio-office module activated)
# Every step's exit status is recorded in 🗑️generated/acceptance/status.tsv; the script exits non-zero if any step fails or never ran.
T=${0:A:h}; R=/Users/ueli/Documents/semio; SERVE=${1:-http://127.0.0.1:6072/}
BUN=$R/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
O=$T/🗑️generated/acceptance; mkdir -p $O; : > $O/status.tsv
STEPS=(gate-rs-check gate-rs-test gate-rs-sqlite gate-ts-check gate-ts-test contract journey-en journey-de features-en features-de)
step() {
  local name=$1; shift
  "$@" > $O/$name.log 2>&1; local code=$?
  print -r -- "$name	$code" >> $O/status.tsv; print -r -- "[acceptance] $name exit=$code"
}
guard() { MIN_DISK_GIB=20 $T/claude-resource-guard.sh "$@"; }
cd $R
step gate-rs-check guard repo run @semio-tech/stdio-xlsx-rs:check --lang=en
step gate-rs-test guard repo run @semio-tech/stdio-xlsx-rs:test --lang=en
step gate-rs-sqlite guard repo run @semio-tech/stdio-xlsx-rs:test-snapshot-sqlite --lang=en
step gate-ts-check repo run @semio-tech/stdio-xlsx:check --lang=en
step gate-ts-test repo run @semio-tech/stdio-xlsx:test --lang=en
step contract repo test-file "✏️s/🔌️plugins/🗄️stdio/🧪️tests/🎯️artifact-acceptance/🧪️contract/🟦️.ts"
cd $T
step journey-en $BUN e2e-journey.ts en $SERVE
step journey-de $BUN e2e-journey.ts de $SERVE
step features-en $BUN e2e-features.ts en $SERVE
step features-de $BUN e2e-features.ts de $SERVE
failed=0
for s in $STEPS; do
  code=$(awk -F'\t' -v s=$s '$1==s{print $2}' $O/status.tsv)
  [ "$code" = 0 ] || { print -r -- "[acceptance] FAILED or missing: $s (${code:-never ran})"; failed=1; }
done
print -r -- "[acceptance] overall exit=$failed"
exit $failed
