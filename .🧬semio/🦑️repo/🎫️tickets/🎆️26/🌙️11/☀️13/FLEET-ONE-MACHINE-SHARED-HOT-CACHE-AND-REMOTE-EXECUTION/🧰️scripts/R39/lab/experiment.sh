#!/bin/zsh
# usage: experiment.sh <scratch dir> <overlay dir or "-"> ; ONE real experiment: real cargo, real wrapper, real rustc, three tiny crates, private target dir
S=${1:A}; OVL=$2
LIVE=/Users/Shared/semio
UC="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache"
HERE=${0:A:h}
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
TC=/Users/ueli/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/bin
PRE="$LIVE/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R5/lab/overlay-preload.ts"
LOAD=$(sysctl -n vm.loadavg | awk '{print int($2)}'); [[ $LOAD -gt 30 ]] && { echo "load $LOAD > 30: stop"; exit 2; }
rm -rf "$S"; mkdir -p "$S/ws/a/src" "$S/ws/b/src" "$S/ws/c/src" "$S/ws/.cargo" "$S/cache" "$S/cas" "$S/target"
cd "$S/ws"
cat > Cargo.toml <<'T'
[workspace]
members = ["a","b","c"]
resolver = "2"
T
cat > .cargo/config.toml <<'T'
[unstable]
embed-metadata = "DoNotEmbed"
checksum-freshness = true
T
for c in a b c; do printf '[package]\nname = "%s"\nversion = "0.1.0"\nedition = "2021"\n' $c > $c/Cargo.toml; done
printf '\n[dependencies]\na = { path = "../a" }\n' >> b/Cargo.toml
printf '\n[dependencies]\na = { path = "../a" }\nb = { path = "../b" }\n' >> c/Cargo.toml
printf 'pub struct S { pub f: u32 }\npub fn answer(x: u32) -> u32 {\n    x + 1\n}\n' > a/src/lib.rs
printf 'pub fn twice(x: u32) -> u32 { a::answer(x) * 2 }\npub fn make() -> a::S { a::S { f: 1 } }\n' > b/src/lib.rs
printf 'pub fn thrice(x: u32) -> u32 { b::twice(x) + a::answer(x) }\n' > c/src/lib.rs
if [[ "$OVL" == "-" ]]; then
 printf '#!/bin/sh\nexec %s %s/%s/📜️script.ts "$@"\n' "$BUN" "$LIVE" "$UC" > "$S/wrap.sh"
else
 printf '#!/bin/sh\nexport R5_OVERLAY=%s R5_LIVE=%s NODE_PATH=%s/node_modules\nexec %s --preload "%s" %s/%s/📜️script.ts "$@"\n' "$OVL" "$LIVE" "$LIVE" "$BUN" "$PRE" "$OVL" "$UC" > "$S/wrap.sh"
fi
chmod +x "$S/wrap.sh"
export PATH="$TC:$PATH" CARGO_TARGET_DIR="$S/target" RUSTC_WRAPPER="$S/wrap.sh" RUSTC_BOOTSTRAP=1 RUSTUP_TOOLCHAIN=nightly-2026-07-20-aarch64-apple-darwin
export SEMIO_UNIT_CACHE_ROOT="$S/cache" SEMIO_UNIT_CACHE_LANE=d "SEMIO_UNIT_CACHE_INNER=[\"$TC/rustc\"]" SEMIO_UNIT_CACHE_TOOLCHAIN=r39 SEMIO_CAS_ROOT="$S/cas" SEMIO_UNIT_CACHE_WORKSPACE_ROOT="$S/ws"
unset SEMIO_FLEET_GROUP SEMIO_REMOTE_CACHE_ENDPOINT SEMIO_REMOTE_CACHE_ACCESS SEMIO_REMOTE_CACHE_TOKEN
MARK=0
now(){ perl -MTime::HiRes=time -e 'printf "%.2f", time'; }
step(){ local label=$1; shift; local t0=$(now); "$@" >"$S/last.log" 2>&1; local rc=$?; local t1=$(now); local out; out=$($BUN "$HERE/outcomes.ts" "$S/cache" $MARK); MARK=$(echo "$out" | sed -E 's/.*"total":([0-9]+).*/\1/'); echo "$label | exit $rc | $(printf "%.1f" $((t1-t0))) s | $(echo "$out" | sed -E 's/.*"steps":"([^"]*)".*/\1/')"; [[ $rc -ne 0 ]] && tail -15 "$S/last.log"; }
echo "=== wrapper: $([[ "$OVL" == "-" ]] && echo live || echo overlay) ; load $LOAD"
step "check cold                      " cargo check --offline
step "check again (nothing changed)   " cargo check --offline
printf 'pub struct S { pub f: u32 }\npub fn answer(x: u32) -> u32 {\n    x + 100\n}\n' > a/src/lib.rs
step "check after body edit of a      " cargo check --offline
printf '\npub fn extra() -> u8 { 0 }\n' >> a/src/lib.rs
step "check after signature edit of a " cargo check --offline
printf 'pub struct S { pub f: u32 }\npub fn answer(x: u32) -> u32 {\n    x + 1\n}\n' > a/src/lib.rs
step "build cold (code, not cut off)  " cargo build --offline
printf 'pub struct S { pub f: u32 }\npub fn answer(x: u32) -> u32 {\n    x + 7\n}\n' > a/src/lib.rs
step "build after body edit of a      " cargo build --offline
du -sk "$S" | awk '{print "scratch KB: "$1}'
