#!/bin/sh
# usage: summ.sh <log> ; streams one cycle log and prints the metrics line: graph generation/plan time, capacity wait, Bazel actions/process mix, tests
l="$1"
echo "log=$(basename "$l") lines=$(wc -l < "$l")"
/usr/bin/grep -a "^graph: generation\|^graph: refreshing\|^graph: [0-9]* of\|admitted \|Build completed\|Executed [0-9]* out of\|[0-9]* processes:\|critical path\|PASSED\|FAILED\|staged [0-9]* deliverables\|Elapsed time\|^\[bazel\] .*cache" "$l" | cut -c1-230 | head -40
/usr/bin/grep -ac "^\[capacity\] waiting" "$l" | sed 's/^/capacity-wait-lines=/'
