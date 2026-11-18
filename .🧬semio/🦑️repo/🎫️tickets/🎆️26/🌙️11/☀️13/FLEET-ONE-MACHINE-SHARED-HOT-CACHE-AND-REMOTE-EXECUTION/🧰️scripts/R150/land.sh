#!/bin/zsh
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R150
R=/Users/ueli/Documents/semio
T=$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
FW="🧰️framework"
DESC="$FW/🔨️modules/📁️filesystem/🧾️observation"
PLAT="$FW/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/🌐️platform"
GEN="$FW/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation"
L="$R/$FW/🛍️products/🦑️repo/🔨️modules/📚️library"
FILES=("$DESC/📁️descriptor/🧬️schema/🔣️.json" "$DESC/📁️descriptor/🟦️.ts" "$GEN/💾️storage/🟦️.ts" "$GEN/🧪️tests/🚪️ingress/🔣️.json" "$GEN/🧪️tests/🚪️ingress/🟦️.ts" "$GEN/🧪️tests/🚪️ingress/🧬️schema/🔣️.json" "$PLAT/🧮️read-plan/🧬️schema/🔣️.json" "$PLAT/🧮️read-plan/🟦️.ts" "$PLAT/🟦️.ts")
NEWDIR="$PLAT/🧮️read-plan/🧪️tests/🪣️chunks"
restore() {
  for f in $FILES; do cp -p "$SC/backup/$f" "$R/$f"; done
  rm -f "$R/$NEWDIR/🟦️.ts" "$R/$NEWDIR/🔣️.json" "$R/$NEWDIR/oracle.py"; rmdir "$R/$NEWDIR" 2>/dev/null
}
while ! mkdir "$T/landing.lock" 2>/dev/null; do echo "held: $(cat $T/landing.lock/owner 2>/dev/null)"; sleep 10; done
echo "claude-R150 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
START=$(date -u +%H:%M:%S)
cd $R
mkdir -p $SC/backup $SC/fresh
for f in $FILES; do mkdir -p "$SC/backup/$(dirname $f)" "$SC/fresh/$(dirname $f)"; cp -p "$R/$f" "$SC/backup/$f"; cp "$R/$f" "$SC/fresh/$f"; done
python3 "$T/🧰️scripts/R150/edit.py" "$SC/fresh" || { rm -r "$T/landing.lock"; echo ANCHORFAIL; exit 3; }
for f in $FILES; do cp "$SC/fresh/$f" "$R/$f"; done
mkdir -p "$R/$NEWDIR"; cp "$SC/newtest-ts.txt" "$R/$NEWDIR/🟦️.ts"; cp "$SC/newtest-json.txt" "$R/$NEWDIR/🔣️.json"; cp "$SC/newtest-oracle.txt" "$R/$NEWDIR/oracle.py"
OK=1
env_t() { env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN SEMIO_TEST_ARTIFACT_DIR=$SC/dout "$@"; }
t() { local out="$SC/win-$3.txt"; (cd "$1" && env_t $BUN test "$2" --timeout 120000 > $out 2>&1) || { echo "RED $1 $2"; grep -E "\(fail\)|error:" $out | head -8; return 1; }; }
mkdir -p $SC/dout
pids=()
t "$R/$NEWDIR" ./🟦️.ts chunks & pids+=($!)
t "$R/$PLAT/🧮️read-plan/🧪️tests" ./🟦️.ts readplan & pids+=($!)
t "$R/$GEN/🧪️tests/🚪️ingress" ./🟦️.ts ingress & pids+=($!)
t "$R/$DESC/🧪️tests/📁️descriptor" ./🟦️.ts desc & pids+=($!)
t "$R/$DESC/🧪️tests/📄️publication" ./🟦️.ts pub & pids+=($!)
t "$R/$DESC/🧪️tests/📄️publication/🧵️lifecycle" ./🟦️.ts life & pids+=($!)
t "$R/$DESC/🧪️tests/🎯️errno-consensus" ./🟦️.ts errno & pids+=($!)
t "$L/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🧪️tests/🛃️preflight" ./🟦️.ts pre1 & pids+=($!)
t "$L/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🪆️heavy-questions/🧪️tests" ./🟦️.ts pre2 & pids+=($!)
for p in $pids; do wait $p || OK=0; done
H="$L/🧪️testing/🌱️genuine-birth/📜️script.ts"; PB=$R/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
env_t $PB "$H" --file="$L/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🧪️tests/📥️parent/🟦️.ts" --pattern="fresh input reservation" --birth=setup --root="$R" > $SC/win-parent.txt 2>&1 || { echo "RED parent reservation"; grep -E "\(fail\)|error:" $SC/win-parent.txt | head; OK=0; }
if [ $OK = 1 ]; then echo "GREEN $START $(date -u +%H:%M:%S)"; else restore; echo RESTORED; fi
rm -r "$T/landing.lock"
