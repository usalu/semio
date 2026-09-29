#!/bin/zsh
# 🧪️ LB2 p9 scratch (rule 23: SDK-wide set, ≥ 20 files): APFS clones of `🧰️framework` (node_modules pruned) + the stdio
# plugin tree under `.🧬semio/🌐hub/s14-lb2-scratch`, a stdio-only workspace (path deps join as auto-members), PRIVATE
# build-dir + target inside the scratch (a clone built on the shared build-dir poisons it). The live tree is never written.
#   setup                      clone + workspace manifest (idempotent: refuses an existing scratch)
#   resync                     re-clone the sources from the live tree (build-dir and target stay)
#   run <capture> <cmd…>       one command in the scratch through the overlay lane (nice 15, CARGO_INCREMENTAL=0)
#   drop                       delete the scratch (record it in 📓️wp-lb2.md)
setopt no_bg_nice
R=/Users/ueli/Documents/semio
SC="${LB2_SCRATCH:-$R/.🧬semio/🌐hub/s14-lb2-scratch}"
CAP="$R/.🧬semio/🌐hub/s14-lb2-captures"
case "$1" in
  setup)
    [ -e "$SC" ] && { echo "scratch exists: $SC"; exit 2; }
    mkdir -p "$SC/✏️s/🔌️plugins" "$CAP" || exit 2
    cp -c -R "$R/🧰️framework" "$SC/🧰️framework" || exit 2
    find "$SC/🧰️framework" -name node_modules -type d -prune -exec rm -rf {} + || exit 2
    cp -c -R "$R/✏️s/🔌️plugins/🗄️stdio" "$SC/✏️s/🔌️plugins/🗄️stdio" || exit 2
    cp -c -R "$R/.cargo" "$SC/.cargo" || exit 2
    for f in Cargo.lock nx.json 📋️project.json rust-toolchain.toml rustfmt.toml; do cp "$R/$f" "$SC/$f" || exit 2; done
    python3 - "$R/Cargo.toml" "$SC/Cargo.toml" <<'EOF'
import re, sys
text = open(sys.argv[1], encoding="utf-8").read()
start = text.index("members = [")
end = text.index("]", start) + 1
member = '    "✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust",\n'
open(sys.argv[2], "w", encoding="utf-8").write(text[:start] + "members = [\n" + member + "]" + text[end:])
EOF
    du -sh "$SC" ;;
  resync)
    [ -d "$SC" ] || { echo "no scratch: $SC"; exit 2; }
    rm -rf "$SC/🧰️framework" "$SC/✏️s/🔌️plugins/🗄️stdio" || exit 2
    cp -c -R "$R/🧰️framework" "$SC/🧰️framework" || exit 2
    find "$SC/🧰️framework" -name node_modules -type d -prune -exec rm -rf {} + || exit 2
    cp -c -R "$R/✏️s/🔌️plugins/🗄️stdio" "$SC/✏️s/🔌️plugins/🗄️stdio" || exit 2
    for f in Cargo.lock nx.json 📋️project.json rust-toolchain.toml rustfmt.toml; do cp "$R/$f" "$SC/$f" || exit 2; done
    echo "resynced $(date '+%F %T')" ;;
  run)
    out="$CAP/$2.txt"; shift 2
    export CARGO_INCREMENTAL=0 NX_DAEMON=false CARGO_BUILD_BUILD_DIR="$SC/.lb2-build" CARGO_TARGET_DIR="$SC/.lb2-target"
    cd "$SC" || exit 2
    echo "QUEUED $(date '+%F %T') $*" > "$out"
    zsh "$R/.tmp-ticket/📜️fleet-mutex.sh" overlay lb2 -- nice -n 15 zsh -c "$*" >> "$out" 2>&1
    rc=$?
    echo "LANE-EXIT rc=$rc $(date '+%F %T')" >> "$out"
    exit $rc ;;
  drop)
    rm -rf "$SC" && echo "dropped $SC" ;;
  *) echo "usage: zsh lb2-p9-scratch.sh setup | run <capture> <cmd…> | drop"; exit 2 ;;
esac
