#!/bin/sh
# usage: restage-all.sh   re-anchors every staged set on the real input it will see, predecessors first (tax-stage on the live tree, the rest on the planner's materialized predecessor tree)
set -e
T="$(cd "$(dirname "$0")/../staged-merge-tools" && pwd)"
for id in tax-stage tsc-stage walkperf bzl-stage fixups6; do
  echo "== restage $id"
  bun "$T/plan.ts" restage "$id"
done
