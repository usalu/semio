#!/usr/bin/env bash
cd /home/deck/semio
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️04/WELL-KNOWN-ARTIFACT-EDITORS-END-TO-END/🗑️generated"
until grep -q '^rc=' "$G/[DEBUG]native-codec-projection.log" 2>/dev/null; do sleep 20; done
repo run workspace:components describe stdio --profile prototype > "$G/[DEBUG]components-describe-stdio.log" 2>&1; echo "rc=$?" >> "$G/[DEBUG]components-describe-stdio.log"
(cd "🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust" && repo cargo --store=shared --configuration=prototype -- test -p semio-hub-stdio --test editor_catalog --test shipped_fleet --test format_resolution --test semio_natural_dispatch --test native_openable_provider --test component_assembly) > "$G/[DEBUG]hub-stdio-laws.log" 2>&1; echo "rc=$?" >> "$G/[DEBUG]hub-stdio-laws.log"
