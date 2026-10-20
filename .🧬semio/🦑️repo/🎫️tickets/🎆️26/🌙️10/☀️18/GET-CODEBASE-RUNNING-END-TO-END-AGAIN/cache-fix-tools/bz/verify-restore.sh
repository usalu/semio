#!/bin/bash
# usage: verify-restore.sh   proves that a Bazel cache hit (action cache, disk cache after `bazel clean`, remote cache in a fresh output base) restores the REAL declared outputs into the checkout; prints PASS/FAIL per phase. Takes about 8 minutes; writes only under the ticket scratch tree and /private/tmp/bzlcf*.
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
B="$C/cache-fix-tools/bz"; FX="$PWD/$C/🗑️generated/cachefix-bz/fx/restore"; BZ="$PWD/.cache/b/t/m/bazel/bin/45e9388abf21d110/bazel"; RC="$PWD/.bazelrc"
PORT=18765; REMOTE="--disk_cache= --remote_cache=http://127.0.0.1:$PORT --noremote_cache_compression --remote_upload_local_results"
fail=0; pass() { echo "PASS $1"; }; bad() { echo "FAIL $1"; fail=1; }
wipe() { chmod -R u+w "$@" 2>/dev/null; rm -rf "$@"; }
outs() { rm -rf "$FX/out" "$FX/dist" "$FX/result" "$FX/extra.txt" "$FX/stamp"; }
sums() { (cd "$FX" && shasum -a 256 out/a.txt out/sub/b.bin out/run.sh dist/c.json result/r.txt 2>/dev/null | cut -c1-24); }
shut() { for d in "$@"; do (cd /private/tmp/$d/ws 2>/dev/null && "$BZ" --output_base=/private/tmp/$d/ob --install_base=/private/tmp/$d/install --nosystem_rc --nohome_rc --bazelrc="$RC" shutdown >/dev/null 2>&1); done; }
has() { command grep -q "$1" "/private/tmp/bzlcf/logs/$2.log"; }
shut bzlcf bzlcf-r1 bzlcf-r2 bzlcf-r3; wipe /private/tmp/bzlcf /private/tmp/bzlcf-r1 /private/tmp/bzlcf-r2 /private/tmp/bzlcf-r3; mkdir -p /private/tmp/bzlcf/logs
bun "$B/fixture.ts" >/dev/null; outs
echo "== P1 cold: generate+consume execute and the declared outputs appear in the checkout"
"$B/go.sh" p1 fx-restore:consume >/dev/null; has "2 local" p1 && pass "P1 executed 2 actions locally" || bad "P1 execution"; sums > /private/tmp/bzlcf/golden.sums; [ -x "$FX/out/run.sh" ] && [ -f "$FX/dist/c.json" ] && pass "P1 declared outputs exist (file, nested file, directory, executable bit)" || bad "P1 outputs"
echo "== P2 action-cache hit without clean: deleting a dependency output is NOT restored when only the consumer is selected"
outs; "$B/go.sh" p2a fx-restore:consume >/dev/null; [ -f "$FX/result/r.txt" ] && [ ! -e "$FX/out/a.txt" ] && pass "P2 consumer outputs restored, unselected dependency outputs not (documented gap: Nx restores both)" || bad "P2 expectation"
echo "== P3 explicit selection restores the producer too"
"$B/go.sh" p3 fx-restore:consume,fx-restore:generate >/dev/null; [ "$(sums)" = "$(cat /private/tmp/bzlcf/golden.sums)" ] && pass "P3 byte-identical restore after action-cache hit" || bad "P3 restore"
echo "== P4 bazel clean + deleted outputs: disk-cache hit restores the real outputs"
outs; (cd /private/tmp/bzlcf/ws && "$BZ" --output_base=/private/tmp/bzlcf/ob --install_base=/private/tmp/bzlcf/install --nosystem_rc --nohome_rc --bazelrc="$RC" clean >/dev/null 2>&1)
"$B/go.sh" p4 fx-restore:consume,fx-restore:generate >/dev/null; has "2 disk cache hit" p4 && pass "P4 two disk cache hits" || bad "P4 hits"; [ "$(sums)" = "$(cat /private/tmp/bzlcf/golden.sums)" ] && [ -x "$FX/out/run.sh" ] && [ ! -e "$FX/extra.txt" ] && pass "P4 outputs byte-identical, mode kept, undeclared file not restored" || bad "P4 restore"
echo "== P5 uncached task executes every time"
"$B/go.sh" p5a fx-restore:stamp >/dev/null; a=$(cat "$FX/stamp/s.txt"); "$B/go.sh" p5b fx-restore:stamp >/dev/null; b=$(cat "$FX/stamp/s.txt"); [ "$a" != "$b" ] && pass "P5 stamp re-executed" || bad "P5"
echo "== P6 declared output that is never produced"
"$B/go.sh" p6 fx-restore:missing >/dev/null; has "Build completed successfully" p6 && echo "FINDING P6 the build succeeds and caches a result whose declared output does not exist (captureBazelSnapshotV1 swallows ENOENT)" || bad "P6"
echo "== P7 remote cache, two fresh output bases, no disk cache"
for attempt in 1 2 3; do
  (bun "$B/http-cache.ts" $PORT > /private/tmp/bzlcf/http-cache.log 2>&1 & echo $! > /private/tmp/bzlcf/http.pid); sleep 1
  shut bzlcf-r1 bzlcf-r2; wipe /private/tmp/bzlcf-r1 /private/tmp/bzlcf-r2; outs
  BZL_BASE=/private/tmp/bzlcf-r1 "$B/go.sh" p7a-$attempt fx-restore:consume,fx-restore:generate $REMOTE >/dev/null
  outs
  BZL_BASE=/private/tmp/bzlcf-r2 "$B/go.sh" p7b-$attempt fx-restore:consume,fx-restore:generate $REMOTE >/dev/null
  kill $(cat /private/tmp/bzlcf/http.pid) 2>/dev/null
  a=$(shasum -a 256 /private/tmp/bzlcf-r1/ws/implementation.json | cut -c1-12); b=$(shasum -a 256 /private/tmp/bzlcf-r2/ws/implementation.json | cut -c1-12)
  [ "$a" = "$b" ] && break; echo "(attempt $attempt: a peer edited the captured implementation closure between the two runs; retrying)"
done
has "2 remote cache hit" p7b-$attempt && pass "P7 two remote cache hits in a fresh output base" || bad "P7 hits"
[ "$(sums)" = "$(cat /private/tmp/bzlcf/golden.sums)" ] && [ -x "$FX/out/run.sh" ] && [ ! -e "$FX/extra.txt" ] && pass "P7 real outputs restored byte-identical from the remote cache" || bad "P7 restore"
echo "== P8 --remote_download_minimal breaks the restore"
(bun "$B/http-cache.ts" $PORT > /private/tmp/bzlcf/http-cache.log 2>&1 & echo $! > /private/tmp/bzlcf/http.pid); sleep 1
shut bzlcf-r1 bzlcf-r3; wipe /private/tmp/bzlcf-r1 /private/tmp/bzlcf-r3; outs
BZL_BASE=/private/tmp/bzlcf-r1 "$B/go.sh" p8a fx-restore:consume $REMOTE >/dev/null; outs
BZL_BASE=/private/tmp/bzlcf-r3 "$B/go.sh" p8b fx-restore:consume $REMOTE --remote_download_minimal >/dev/null
kill $(cat /private/tmp/bzlcf/http.pid) 2>/dev/null
has "snapshot.json" p8b && has "ENOENT" p8b && echo "FINDING P8 remote_download_minimal leaves bin/<label>.snapshot absent, so the post-build restore fails with ENOENT and no real output is restored (keep --remote_download_outputs=toplevel or all)" || echo "NOTE P8 not reproduced (a peer edit changed the key, or the download mode kept the snapshot)"
shut bzlcf bzlcf-r1 bzlcf-r2 bzlcf-r3
echo "== done fail=$fail"; exit $fail
