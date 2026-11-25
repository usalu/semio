#!/usr/bin/env bash
# 🎭 Source this FIRST inside the snippet given to native-cargo.sh. It runs in that private mount namespace only:
# each framework file whose foreign debug hook points at a path of another machine is covered by a copy in the tmpfs
# whose hook points at debug-ledger-stub.rs. Before that, the two folders under live edit (value, store) are frozen as a copy in
# the tmpfs, so a build sees one state of them. The shared tree is not written. Every cover is printed with its time.
read -r _ _ span < /proc/self/uid_map; [ "$span" != 4294967295 ] || { echo "mask-dbg: refusing outside a private user namespace" >&2; exit 97; }
mkdir -p /tmp/mask
cp "$NATIVE_REPO_ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-PUZZLE-PLUGIN-WORKING-END-TO-END/debug-ledger-stub.rs" /tmp/mask/stub.rs
mask_once() {
  while IFS= read -r -d '' f; do
    i="/tmp/mask/$(date +%s%N).rs"; sed -E 's#\#\[path="/Users/[^"]*debug-ledger\.rs"\]#\#[path="/tmp/mask/stub.rs"]#' "$f" > "$i"
    mount --bind "$i" "$f" && ! grep -q '"/Users/' "$f" && echo "mask-dbg: $(date +%T) covered $f (sha $(sha256sum < "$i" | cut -c1-12))"
  done < <(grep -rlZ --include='*.rs' '#\[path="/Users/[^"]*debug-ledger\.rs"\]' "$@" 2>/dev/null)
}
for frozen in "🧰️framework/🔨️modules/🌱️value" "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store"; do
  i="/tmp/mask/frozen-$(echo "$frozen" | md5sum | cut -c1-8)"; mkdir -p "$i"
  (cd "$NATIVE_REPO_ROOT/$frozen" && tar -cf - --exclude=node_modules --exclude=target --exclude='🗑️generated' .) | tar -xf - -C "$i"
  mount --bind "$i" "$NATIVE_REPO_ROOT/$frozen" && echo "mask-dbg: $(date +%T) frozen $frozen ($(find "$i" -type f | wc -l) files)"
done
mask_once "$NATIVE_REPO_ROOT/🧰️framework/🔨️modules" "$NATIVE_REPO_ROOT/🧰️framework/🛍️products/💻️os/🔨️modules"
