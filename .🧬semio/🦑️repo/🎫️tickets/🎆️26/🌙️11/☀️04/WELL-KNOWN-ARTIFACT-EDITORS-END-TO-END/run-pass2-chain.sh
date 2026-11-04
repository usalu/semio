#!/usr/bin/env bash
cd /home/deck/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️04/WELL-KNOWN-ARTIFACT-EDITORS-END-TO-END"
G="$T/🗑️generated"
until grep -q '^rc=' "$G/[DEBUG]hub-stdio-laws.log" 2>/dev/null; do sleep 30; done
run() { local name="$1"; shift; "$@" > "$G/[DEBUG]p2-$name.log" 2>&1; echo "rc=$?" >> "$G/[DEBUG]p2-$name.log"; }
run composition-prepare repo run @semio-tech/stdio-plugin:composition-prepare
run native-codec-projection repo run @semio-tech/stdio-plugin:native-codec-projection
run components-describe repo run workspace:components describe stdio --profile prototype
A="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"
for d in "$A"/*/; do
  [ -f "$d/Cargo.toml" ] && grep -q '^\[workspace\]' "$d/Cargo.toml" || continue
  k=$(basename "$d" | sed 's/[^[:alnum:]]//g'); [ -n "$k" ] || k=$(basename "$d" | md5sum | cut -c1-6)
  (cd "$d" && repo cargo --store=shared --configuration=prototype -- test --workspace --no-fail-fast) > "$G/[DEBUG]p2-cargo-$k.log" 2>&1; echo "rc=$?" >> "$G/[DEBUG]p2-cargo-$k.log"
done
repo tools exec bun -- test "./✏️s/🔌️plugins/🗄️stdio/🧪️tests/🎯️artifact-acceptance/🧪️contract/🟦️.ts" > "$G/[DEBUG]p2-contract.log" 2>&1; echo "rc=$?" >> "$G/[DEBUG]p2-contract.log"
