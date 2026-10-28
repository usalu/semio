#!/bin/sh
# queue2: proof 9 per Go project (a failing project must not hide the others), then proof 4 rerun with a vitest process census
cd "$(dirname "$0")/../../../../../../../.." || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; G="$C/🗑️generated/proof2-g"
export PROOF_LIMIT_S=7200 SEMIO_BUILD_GRAPH_POLICY=current
M="python3 $C/proof2-g-tools/measure.py"
for p in repo-cli-go repo-graphql-go repo-tickets-go repo-hooks-go repo-tree-go repo-providers-go repo-launcher-go repo-workspace-go; do
  $M "$G/p9" "go-$p" bun repo --tool=bazel run "@semio-tech/$p:test" >> "$G/p9.rows" 2>&1
done
$M "$G/p9" "go-repo-mcp" bun repo --tool=bazel run repo-mcp:test >> "$G/p9.rows" 2>&1
PROOF_CENSUS='vitest|maxWorkers' $M "$G/p4" course-test2 bun repo run @teaching/architecture-course:test --skip-cache >> "$G/p9.rows" 2>&1
echo finished > "$G/queue2.done"
