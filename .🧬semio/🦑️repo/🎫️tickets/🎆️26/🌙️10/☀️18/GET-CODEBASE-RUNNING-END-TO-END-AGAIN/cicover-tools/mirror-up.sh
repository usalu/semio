#!/bin/sh
# 🪞 Makes the mirror `cv1` complete for the CI laws (creates it when absent): `sh mirror-up.sh`; prints the mirror root. Files that the default closure misses are COPIED from live (read only).
set -eu
here=$(cd "$(dirname "$0")" && pwd)
live=$(python3 -I -c 'import os,sys;h=sys.argv[1]
while not os.path.isdir(os.path.join(h,"🧰️framework")) and h!="/":h=os.path.dirname(h)
print(h)' "$here")
MM="python3 -I $live/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/mirror-tools/mirror.py"
root=$($MM list | awk -F'\t' '$1=="cv1"{print $4}')
if [ -z "$root" ]; then
  $MM create cv1 --only "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️ci-report" .github >&2
  root=$($MM list | awk -F'\t' '$1=="cv1"{print $4}')
fi
cd "$live"
D="🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard"
for rel in "$D/🧪️tests/🏗️ci" "$D/🧬️schema/🪟️windows/🎟️coverage" "$D/🧫️fixtures/🪟️windows/🎟️coverage" "$D/📦️packages/🦀️rust/📋️project.json" "$D/🛤️task-routes" "🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📌️runtime-pin" "🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🗂️registry"; do
  mkdir -p "$root/$(dirname "$rel")"
  [ -e "$root/$rel" ] || cp -R "$rel" "$root/$rel"
done
echo "$root"
