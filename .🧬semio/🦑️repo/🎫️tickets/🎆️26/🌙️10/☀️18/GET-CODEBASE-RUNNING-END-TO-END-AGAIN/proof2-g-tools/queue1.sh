#!/bin/sh
# queue1: proofs 3 (lease-g targets), 4 (ts-g vitest cap), 9 (go-g Go tests), one official run at a time, graph policy current
cd "$(dirname "$0")/../../../../../../../.." || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; G="$C/🗑️generated/proof2-g"
export PROOF_LIMIT_S=7200 SEMIO_BUILD_GRAPH_POLICY=current
M="python3 $C/proof2-g-tools/measure.py"
$M "$G/p3" lease bun repo run-many --projects=@semio-tech/framework-process --targets=test-resource-lease-standing,test-resource-lease-standing-live --parallel=1 > "$G/p3.out" 2>&1
$M "$G/p4" course-test bun repo run @teaching/architecture-course:test > "$G/p4.out" 2>&1
GO=@semio-tech/repo-cli-go,@semio-tech/repo-workspace-go,@semio-tech/repo-graphql-go,@semio-tech/repo-tickets-go,@semio-tech/repo-hooks-go,@semio-tech/repo-tree-go,@semio-tech/repo-providers-go,@semio-tech/repo-launcher-go
$M "$G/p9" go bun repo --tool=bazel run-many --projects=$GO --targets=test --parallel=1 > "$G/p9.out" 2>&1
echo finished > "$G/queue1.done"
