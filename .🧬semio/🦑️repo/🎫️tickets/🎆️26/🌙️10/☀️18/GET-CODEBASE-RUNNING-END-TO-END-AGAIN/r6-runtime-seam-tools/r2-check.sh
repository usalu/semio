#!/bin/sh
# Re-applies the batch to a pristine copy, rebuilds the git twin and compares tsc before/after. Usage: r2-check.sh
T="$(cd "$(dirname "$0")" && pwd)"; B=/private/tmp/runtime-seam/p3; C=/private/tmp/runtime-seam/w3; O=/private/tmp/runtime-seam/out2; mkdir -p "$O"
wait_load() { while [ "$(sysctl -n vm.loadavg | awk '{print int($2)}')" -ge 30 ]; do sleep 20; done; }
cd "$T" && python3 place.py > /dev/null; wait_load; ./iterate.sh "$O/iter.out"; tail -2 "$O/iter.out" | cut -c1-300
wait_load; (cd "$C" && rm -rf .git && git init -q . && printf 'node_modules\ntsconfig.touched.json\n' > .gitignore && git add -A > /dev/null 2>&1 && git -c commit.gpgsign=false -c user.name=x -c user.email=x@x commit -qm applied > /dev/null 2>&1)
wait_load; python3 "$T/mk-tsconfig.py" "$B" "$C" > "$O/tsconfig.out" 2>&1
wait_load; (cd "$C" && nice -n 10 node /Users/ueli/Documents/semio/node_modules/typescript/bin/tsc -p tsconfig.touched.json --pretty false > "$O/tsc-w3.out" 2>&1; echo "EXIT $?" >> "$O/tsc-w3.out")
python3 /private/tmp/runtime-seam/tscdiff.py "$O/tsc-p3.out" "$O/tsc-w3.out" | /usr/bin/grep -v TS6142 | head -30
