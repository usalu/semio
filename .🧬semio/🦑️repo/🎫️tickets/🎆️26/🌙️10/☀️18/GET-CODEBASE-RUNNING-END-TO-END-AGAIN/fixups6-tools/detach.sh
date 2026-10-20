#!/bin/sh
for ov in "$@"; do
  mount | grep -F " on $ov/" | sed -E 's# on .*##' | while read dev; do hdiutil detach "$dev" >/dev/null 2>&1 || hdiutil detach -force "$dev" >/dev/null 2>&1; done
done
sleep 1
