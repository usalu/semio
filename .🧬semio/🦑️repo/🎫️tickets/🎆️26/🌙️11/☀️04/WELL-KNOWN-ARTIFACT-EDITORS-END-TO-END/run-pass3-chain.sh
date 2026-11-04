#!/usr/bin/env bash
# Pass 3: lease-friendly generators, then a value-ordered, duplicate-avoiding cargo chain. Gate: T05 creates the GATE file after `dependencies ensure` plus activation, so the JS dependency lease is free.
cd /home/deck/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️04/WELL-KNOWN-ARTIFACT-EDITORS-END-TO-END"
G="$T/🗑️generated"
until [ -e "$G/GATE-T05-activation-done" ] && ! kill -0 939199 2>/dev/null; do sleep 30; done
run() { local name="$1"; shift; "$@" > "$G/[DEBUG]p3-$name.log" 2>&1; echo "rc=$?" >> "$G/[DEBUG]p3-$name.log"; }
run native-codec-projection repo run @semio-tech/stdio-plugin:native-codec-projection
run components-describe repo run workspace:components describe stdio --profile prototype
A="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"
busy() { for p in $(pgrep -f "repo cargo"); do readlink "/proc/$p/cwd" 2>/dev/null; done; }
for d in $(for x in "$A"/*/; do [ -f "$x/Cargo.toml" ] && grep -q '^\[workspace\]' "$x/Cargo.toml" && echo "$(git status --short -- "$x" | wc -l) $x"; done | sort -rn | cut -d' ' -f2-); do
  if busy | grep -qF "$(cd "$d" && pwd)"; then echo "skip busy $d" >> "$G/[DEBUG]p3-skipped.log"; continue; fi
  k=$(basename "$d" | sed 's/[^[:alnum:]]//g'); [ -n "$k" ] || k=$(basename "$d" | md5sum | cut -c1-6)
  (cd "$d" && repo cargo --store=shared --configuration=prototype -- test --workspace --no-fail-fast) > "$G/[DEBUG]p3-cargo-$k.log" 2>&1; echo "rc=$?" >> "$G/[DEBUG]p3-cargo-$k.log"
done
(cd "🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust" && repo cargo --store=shared --configuration=prototype -- test -p semio-hub-stdio --test editor_catalog --test shipped_fleet --test format_resolution --test semio_natural_dispatch --test native_openable_provider --test component_assembly) > "$G/[DEBUG]p3-hub-stdio-laws.log" 2>&1; echo "rc=$?" >> "$G/[DEBUG]p3-hub-stdio-laws.log"
repo tools exec bun -- test "./✏️s/🔌️plugins/🗄️stdio/🧪️tests/🎯️artifact-acceptance/🧪️contract/🟦️.ts" > "$G/[DEBUG]p3-contract.log" 2>&1; echo "rc=$?" >> "$G/[DEBUG]p3-contract.log"
