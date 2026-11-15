#!/bin/bash
# 🔏 Prints inode, modification time, link target and code signature of the two repo launchers (read-only). Usage: launcher-facts.sh <label>
echo "== $1 $(date -u +%Y-%m-%dT%H:%M:%SZ)"
L="$HOME/.local/share/semio/launcher/bin/repo"; N="$HOME/.local/bin/repo"
ls -li "$L" "$N" 2>&1
for f in "$L" "$N"; do [ -L "$f" ] && echo "readlink $f -> $(readlink "$f")"; echo "mtime $f: $(stat -L -f '%Sm' -t '%Y-%m-%dT%H:%M:%SZ' "$f" 2>&1) (UTC printed via TZ)"; done
echo "codesign launcher:"; codesign -dvvv "$L" 2>&1 | grep -E 'Identifier|CDHash'
ls -la "$HOME/.local/share/semio/launcher/" "$HOME/.local/share/semio/launcher/versions" 2>/dev/null | awk '{print $1,$5,$6,$7,$8,$9,$10,$11}'
