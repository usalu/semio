#!/bin/sh
# Full verification chain over a pristine tree B and its transformed twin C. Usage: verify-all.sh B C OUTDIR
B="$1"; C="$2"; O="$3"; T="$(cd "$(dirname "$0")" && pwd)"
wait_load() { while [ "$(sysctl -n vm.loadavg | awk '{print int($2)}')" -ge 30 ]; do sleep 20; done; }
mkdir -p "$O"
wait_load; (cd "$T" && /usr/bin/grep -rlE "Bun|bun:|\"bun\"|'bun'|versions\.bun|import\.meta\.(dir|path|main)" --include='*.ts' --include='*.tsx' --include='*.mjs' --include='*.cjs' --include='*.js' --include='*.mts' -r "$C/🧰️framework" 2>/dev/null | sed "s#^$C/##" | sort > "$O/candidates2.txt")
wait_load; (cd "$T" && nice -n 10 bun apply.ts --root "$C" --candidates "$O/candidates2.txt" > "$O/idempotence.out" 2>&1; echo "EXIT $?" >> "$O/idempotence.out")
wait_load; (cd "$C" && git init -q . && printf 'node_modules\ntsconfig.touched.json\n' > .gitignore && git add -A >/dev/null 2>&1 && git -c commit.gpgsign=false -c user.name=x -c user.email=x@x commit -qm applied >/dev/null 2>&1)
wait_load; (cd "$C" && SEMIO_REPOSITORY_ROOT="$C" nice -n 10 bun test "./🧰️framework/🔨️modules/⚙️runtime/🧪️tests/🟦️.ts" > "$O/module-tests.out" 2>&1; echo "EXIT $?" >> "$O/module-tests.out")
wait_load; python3 "$T/mk-tsconfig.py" "$B" "$C" > "$O/tsconfig.out" 2>&1
for tree in "$B" "$C"; do wait_load; n=$(basename "$tree"); (cd "$tree" && nice -n 10 node /Users/ueli/Documents/semio/node_modules/typescript/bin/tsc -p tsconfig.touched.json --pretty false > "$O/tsc-$n.out" 2>&1; echo "EXIT $?" >> "$O/tsc-$n.out"); done
wait_load; (cd "$T" && python3 suites.py "$B" "$O/suites-before.json" > "$O/suites-before.out" 2>&1)
wait_load; (cd "$T" && python3 suites.py "$C" "$O/suites-after.json" > "$O/suites-after.out" 2>&1)
echo DONE > "$O/done"
