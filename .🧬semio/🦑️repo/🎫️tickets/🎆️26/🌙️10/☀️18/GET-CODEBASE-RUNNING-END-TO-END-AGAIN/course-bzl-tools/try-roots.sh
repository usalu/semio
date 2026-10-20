#!/bin/sh
# usage: try-roots.sh <step[,step]> : runs the course series --check for the given steps against the live tree and against the conc-bazel-applied overlay (/private/tmp/cb/root, built by compat-check.py)
here=$(cd "$(dirname "$0")" && pwd)
for root in "$(cd "$here/../../../../../../../.." && pwd)" /private/tmp/cb/root; do
  echo "### $root"; python3 -I "$here/staged/apply-course.py" --check --root "$root" --only "$1" 2>&1 | grep -E "^(refused|pending|applied|skipped)|anchor|failures" | cut -c1-260
done
