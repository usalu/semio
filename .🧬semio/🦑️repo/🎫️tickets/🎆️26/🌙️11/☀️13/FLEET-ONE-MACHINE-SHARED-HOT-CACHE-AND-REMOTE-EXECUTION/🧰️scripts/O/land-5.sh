#!/bin/zsh
# Package O landing 5 (run only while holding landing.lock): checksum flag in the action allow-list, policy-refusal retry in the scheduler, worker-policy decline reason; then the tests of every member touched.
set -e
T="/Users/Shared/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
RE="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution"
grep -q claude-package-O "$T/landing.lock/owner" || { echo "lock not mine"; exit 3; }
python3 - "$RE" <<'PY'
import sys,pathlib
RE=pathlib.Path(sys.argv[1])
p=RE/"🧱️actions/🟦️.ts"; t=p.read_text()
old='/^(?:threads=[0-9]{1,3}|embed-metadata=(?:yes|no))$/u.test(value)'
assert t.count(old)==1
p.write_text(t.replace(old,'/^(?:threads=[0-9]{1,3}|embed-metadata=(?:yes|no)|checksum-hash-algorithm=(?:md5|sha1|sha256|blake3))$/u.test(value)'))
q=RE/"🧱️actions/🧪️tests/🟦️.ts"; s=q.read_text()
a='"-Z","embed-metadata=no",'
assert s.count(a)==1
s=s.replace(a,'"-Z","embed-metadata=no","-Z","checksum-hash-algorithm=blake3",')
b='["-Z","threads=x"],'
assert s.count(b)==1
s=s.replace(b,'["-Z","threads=x"],["-Z","checksum-hash-algorithm=../x"],["-Z","checksum-hash-algorithm=crc"],')
q.write_text(s)
PY
python3 "$T/🧰️scripts/O/patch-policy-retry.py" "$RE"
python3 "$T/🧰️scripts/O/patch-worker-policy-reason.py" "$RE"
BT=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/O/bt.sh
for d in "🧱️actions/🧪️tests" "🗓️scheduler/🧪️tests" "🚀️offload/🧪️tests" "🚀️offload/🧪️tests/🌐️cluster" "🧪️tests" "🧪️tests/🌐️fleet"; do echo "-- $d"; $BT "$RE/$d" "🟦️.ts" 2>&1 | grep -E "^ *[0-9]+ (pass|fail)|\(fail\)|error:" ; done
