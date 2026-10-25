#!/bin/sh
cd /Users/ueli/Documents/semio/.cache/b/p/m/bazel/workspaces || exit 1
for s in s0 s1 s2 s3 d0 d1 d2 d3; do for p in $s/*; do [ -d "$p" ] || continue; printf '%s %s %s\n' "$p" "$(du -sk "$p" 2>/dev/null | cut -f1)" "$(ls -ld "$p" | awk '{print $6,$7,$8}')"; done; done
echo DONE
