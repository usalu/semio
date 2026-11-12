#!/usr/bin/env bash
T="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END"
cd /home/deck/semio
for t in "@semio-tech/repo-lib:workspaces-check" "@semio-tech/value-ts:verify-shared-dynamic-value" "@semio-tech/framework-os:verify-resolved-host-context" "repo-policy:verify-zero-touch"; do
  n=$(echo "$t" | tr '/:@' '___')
  timeout 3700 repo run "$t" > "$T/🗑️generated/rootsplit-run2-$n.log" 2>&1
  echo "rc=$?" >> "$T/🗑️generated/rootsplit-run2-$n.log"
done
echo done > "$T/🗑️generated/rootsplit-validate2.done"
