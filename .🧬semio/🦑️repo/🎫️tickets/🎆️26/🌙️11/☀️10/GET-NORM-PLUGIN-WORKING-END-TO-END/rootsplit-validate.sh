#!/usr/bin/env bash
T="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END"
cd /home/deck/semio
until grep -q "^rc=" "$T/🗑️generated/rootsplit-ws-check1.log"; do sleep 5; done
for t in "@semio-tech/value-ts:verify-shared-dynamic-value" "@semio-tech/framework-os:verify-resolved-host-context" "repo-lib:placeholder"; do
  case "$t" in *placeholder) t="@semio-tech/repo-lib:workspaces-check";; esac
  n=$(echo "$t" | tr '/:@' '___')
  timeout 2400 repo run "$t" > "$T/🗑️generated/rootsplit-run-$n.log" 2>&1
  echo "rc=$?" >> "$T/🗑️generated/rootsplit-run-$n.log"
done
for t in "repo:graph-check" "@semio-tech/framework-process:test-command-owner"; do
  n=$(echo "$t" | tr '/:@' '___')
  timeout 2400 repo run "$t" > "$T/🗑️generated/rootsplit-run-$n.log" 2>&1
  echo "rc=$?" >> "$T/🗑️generated/rootsplit-run-$n.log"
done
echo done > "$T/🗑️generated/rootsplit-validate.done"
