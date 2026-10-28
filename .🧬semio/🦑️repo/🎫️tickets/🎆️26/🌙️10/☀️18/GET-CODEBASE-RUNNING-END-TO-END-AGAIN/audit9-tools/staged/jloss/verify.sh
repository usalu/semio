#!/bin/sh
# 🕳️ Laws of batch `jloss` after apply: sh verify.sh <mirror-root>
# Builds the new watcher, its stall build (--cfg semio_watch_stall) and three negative controls with rustc in a temp dir outside the root:
# base+stall must FAIL the replay law (it records `user-dropped`), the base watcher must FAIL the QoS law (utility priority 20),
# the new watcher without the replay buffer must FAIL the replay law (a replayed fence row overtakes earlier changes; nondeterministic, up to 6 runs, WARN only).
set -e
here=$(cd "$(dirname "$0")" && pwd); root=${1:?mirror root}
cd "$root"
R=./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident
RS="$R/🔭️watch/🦀️rust/🦀️.rs"; LAW="$R/🧪️tests/🔭️watch/🕳️drops/🟦️.ts"
python3 -I "$here/stage.py" check --root "$root"
out=$(mktemp -d "${TMPDIR:-/tmp}/jlv.XXXXXX"); FAILED=0
law() { name=$1; shift; if "$@" >"$out/law.txt" 2>&1; then echo "PASS  $name"; else echo "FAIL  $name"; tail -25 "$out/law.txt"; FAILED=1; fi; }
bite() { name=$1; shift; if "$@" >"$out/neg.txt" 2>&1; then echo "FAIL  negative control did not bite: $name"; FAILED=1; else echo "PASS  negative control bites: $name"; grep -E "Expected|Received|lost|missing|toBeGreaterThanOrEqual" "$out/neg.txt" | head -4; fi; }
build() { rustc --crate-name semio_repo_watch --edition 2021 -O "$@"; }
law "watcher unit tests (drop actions, replay resume, writer overflow, inotify batches, clone kinds, verbatim paths)" sh -c "rustc --crate-name semio_repo_watch --edition 2021 --test '$RS' -o '$out/unit' && '$out/unit'"
for target in x86_64-pc-windows-msvc aarch64-pc-windows-msvc x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu aarch64-apple-darwin x86_64-apple-darwin; do
  if rustup target list --installed 2>/dev/null | grep -qx "$target"; then law "watcher type-checks for $target without warnings" sh -c "rustc --crate-name semio_repo_watch --edition 2021 --target $target --emit=metadata -D warnings -o '$out/$target.rmeta' '$RS'"; else echo "SKIP  watcher type-check for $target (std not installed)"; fi
done
mkdir -p "$out/new" "$out/base"
build "$RS" -D warnings -o "$out/new/semio-repo-watch"
build "$RS" --cfg semio_watch_stall -o "$out/stall"
build "$here/base/00.rs" -o "$out/base/semio-repo-watch"
python3 -I - "$here/base/00.rs" "$RS" "$out" <<'EOF'
import sys
base, new, out = sys.argv[1:4]
cb = "        let paths = paths as *const *const c_char;\n        let mut changes: BTreeMap<String, Change> = BTreeMap::new();"
b = open(base).read(); assert b.count(cb) == 1
open(out + "/base-stall.rs", "w").write(b.replace(cb, "        std::thread::sleep(std::time::Duration::from_millis(20));\n" + cb))
n = open(new).read(); buffered = "if since == SINCE_NOW { None } else { Some(BTreeMap::new()) }"; assert n.count(buffered) == 1
open(out + "/unbuffered.rs", "w").write(n.replace(buffered, "None"))
EOF
build "$out/base-stall.rs" -o "$out/base-stall"
build "$out/unbuffered.rs" --cfg semio_watch_stall -o "$out/unbuffered-stall"
law "drop laws (replay is exact and fence-ordered; callback queue >= default priority)" env SEMIO_REPO_WATCH_BINARY="$out/new/semio-repo-watch" SEMIO_REPO_WATCH_STALL_BINARY="$out/stall" bun test "$LAW"
law "watcher + supervisor laws with the new watcher" env SEMIO_REPO_WATCH_BINARY="$out/new/semio-repo-watch" bun test "$R/🧪️tests/🔭️watch/🟦️.ts"
law "journal laws with the new watcher" env SEMIO_REPO_WATCH_BINARY="$out/new/semio-repo-watch" bun test "$R/📜️journal/🧪️tests/🟦️.ts"
law "provision laws (pin names the new source)" env SEMIO_REPO_WATCH_BINARY="$out/new/semio-repo-watch" bun test "$R/🧪️tests/🏗️provision/🟦️.ts"
bite "base watcher with the same stall records user-dropped" env SEMIO_REPO_WATCH_BINARY="$out/new/semio-repo-watch" SEMIO_REPO_WATCH_STALL_BINARY="$out/base-stall" bun test "$LAW" -t "stalled"
bite "base watcher runs its callback queue at utility priority" env SEMIO_REPO_WATCH_BINARY="$out/base/semio-repo-watch" SEMIO_REPO_WATCH_STALL_BINARY="$out/stall" bun test "$LAW" -t "priority"
if sh -c 'for run in 1 2 3 4 5 6; do "$@" >/dev/null 2>&1 || exit 1; done' sh env SEMIO_REPO_WATCH_BINARY="$out/new/semio-repo-watch" SEMIO_REPO_WATCH_STALL_BINARY="$out/unbuffered-stall" bun test "$LAW" -t "stalled"; then echo "WARN  negative control did not bite in 6 runs (fseventsd replayed in order every time): a replay emitted unbuffered lets a fence row overtake earlier changes"; else echo "PASS  negative control bites (within 6 runs; fseventsd replays out of event-id order in about one run of three): a replay emitted unbuffered lets a fence row overtake earlier changes"; fi
rm -rf "$out"
[ "$FAILED" = 0 ] && echo "jloss laws: GREEN" || { echo "jloss laws: RED"; exit 1; }
