#!/bin/zsh
# 🧪️ H14 overlay proof of t6-queue rows 34 + 36 (os-kernel directory schema = frozen guest crate, so never the live tree):
# APFS clones of `🧰️framework`, `✏️s`, `🌎️hub` (+ root manifests) under `.🧬semio/🌐hub/s14-h14-overlay-r36`, build outputs
# pruned, rows 34 then 36 applied there (hold A is already in the live tree), PRIVATE build-dir + target inside the overlay.
#   setup                 clone + prune + apply (refuses an existing overlay)
#   proof <capture> [step…]  the steps (default all) through ONE overlay-lane hold (nice 15, CARGO_INCREMENTAL=0, offline)
#   drop                  delete the overlay (record it in 📓️wp-h14.md)
setopt no_bg_nice
R=/Users/ueli/Documents/semio
W=$R/.tmp-ticket/wp-h14
OV="$R/.🧬semio/🌐hub/s14-h14-overlay-r36"
L="$R/.🧬semio/🌐hub/s14-h14-logs"
case "$1" in
  setup)
    [ -e "$OV" ] && { echo "overlay exists: $OV"; exit 2; }
    mkdir -p "$OV" "$L" || exit 2
    for d in 🧰️framework ✏️s 🌎️hub .cargo; do cp -c -R "$R/$d" "$OV/$d" || exit 2; done
    for f in Cargo.toml Cargo.lock rust-toolchain.toml rustfmt.toml nx.json 📋️project.json; do cp "$R/$f" "$OV/$f" || exit 2; done
    find "$OV/🧰️framework" "$OV/✏️s" "$OV/🌎️hub" \( -name node_modules -o -name target -o -name dist \) -type d -prune -exec rm -rf {} + || exit 2
    H14_ROOT="$OV" python3 "$W/h14-creation-rule.py" --dry-run || exit 3
    H14_ROOT="$OV" python3 "$W/h14-catalog-bound.py" --write || exit 3
    H14_ROOT="$OV" python3 "$W/h14-creation-standard.py" --write || exit 3
    du -sh "$OV" ;;
  proof)
    out="$L/$2.txt"; tag="$2"; shift 2
    cd "$OV" || exit 2
    export CARGO_INCREMENTAL=0 NX_DAEMON=false CARGO_BUILD_BUILD_DIR="$OV/.h14-build" CARGO_TARGET_DIR="$OV/.h14-target"
    echo "QUEUED $(date '+%F %T')" > "$out"
    zsh "$R/.tmp-ticket/📜️fleet-mutex.sh" overlay h14 -- nice -n 15 zsh -c '
      tag="$1"; only=("${@:2}")
      step() { local name="$1"; shift; (( ${#only} == 0 || ${only[(Ie)$name]} )) || return 0; echo "STEP $name START $(date +%T)"; "$@" > "'"$L"'/$tag-$name.out" 2>&1; echo "STEP $name rc=$? END $(date +%T)"; /usr/bin/grep -E "^test result|^error(\[|:)|FAILED|panicked" "'"$L"'/$tag-$name.out" | head -40; }
      step kernel cargo check --offline --keep-going -p semio-framework-os-kernel --lib --tests
      step hub cargo check --offline --keep-going -p semio-hub --lib --bins --tests
      step hub-laws cargo test --offline --no-fail-fast -p semio-hub --lib -- trusted_catalog creation::
      step census cargo test --offline --no-fail-fast -p semio-hub --lib -- the_creation_catalog_offers_or_names_every_kind_an_editor_opens_with_its_reason every_committed_kind_and_standard_an_editor_opens_has_one_creating_editor every_committed_editor_that_edits_a_document_opens_a_kind_through_the_one_rule
      step mcp cargo check --offline --keep-going -p semio-framework-os-mcp --lib --tests
      step wgpu cargo check --offline --keep-going -p semio-framework-os-renderer-wgpu --lib --tests
      echo "PROOF END $(date +%T)"
    ' zsh "$tag" "$@" >> "$out" 2>&1
    echo "LANE-EXIT rc=$? $(date '+%F %T')" >> "$out" ;;
  drop)
    rm -rf "$OV" && echo "dropped $OV" ;;
  *) echo "usage: zsh h14-overlay-r36.sh setup | proof <capture> | drop"; exit 2 ;;
esac
