#!/bin/sh
# 🔭️ Prints a dump through view.py without mass markers, profile lines and the material list last. usage: view2.sh <asset id> [max line length]
HERE=$(cd "$(dirname "$0")" && pwd)
D="$HERE/../../🗑️generated/linux-s4/dump"
python3 "$HERE/view.py" "$D/$1.txt" "${2:-300}" | grep -v '^materials\|^   {"\|^mass-\|^emitters  minAir\|^supports $'
grep -m1 '^materials' "$D/$1.txt"
