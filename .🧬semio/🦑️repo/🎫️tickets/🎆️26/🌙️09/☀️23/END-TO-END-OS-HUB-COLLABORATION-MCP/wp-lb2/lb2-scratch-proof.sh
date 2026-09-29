#!/bin/zsh
# 🧪️ LB2 scratch proof of p9 → p10 → p11 → p12 → p13 → p14 (overlay lane: `lb2-p9-scratch.sh run <capture> zsh <this> <tag> [step…]`,
# cwd = scratch, private build-dir/target from the caller). One capture per step: .🧬semio/🌐hub/s14-lb2-captures/<tag>-<step>.out
setopt no_bg_nice
C="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-lb2-captures"
tag="$1"; shift
only=("$@")
dump="$C/$tag-dump-initial"
mkdir -p "$dump"
step() {
  local name="$1"; shift
  (( ${#only} == 0 || ${only[(Ie)$name]} )) || return 0
  echo "STEP $name START $(date '+%T')"
  "$@" > "$C/$tag-$name.out" 2>&1
  echo "STEP $name rc=$? END $(date '+%T')"
  /usr/bin/grep -E "^test result|^error(\[|:)" "$C/$tag-$name.out" | head -40
}
roots=(bmp wav epw binary ifc gif semio avi dwg dxf jpg las pdf png pptx tiff xml zip docx xlsx step)
root_args=(); root_features=()
for r in $roots; do root_args+=(-p "semio-s-artifact-stdio-$r"); root_features+=("semio-s-artifact-stdio-$r/component-app-assembly"); done
families=(image media cad bim mesh pdf office semio binary)
family_args=(); for f in $families; do family_args+=(-p "semio-s-plugin-stdio-$f"); done
step catalog env LB2_DUMP_DIR="$dump" cargo test --offline -p semio-s-plugin-stdio --lib --test shipped_fleet --test editor_catalog --no-fail-fast -- --test-threads 4 --nocapture
step families cargo test --offline --no-fail-fast $family_args --lib -- descriptor_is_fresh
step sdk cargo test --offline --no-fail-fast -p semio-framework-plugin --lib builder:: -- --test-threads 1
step schema cargo test --offline --no-fail-fast -p semio-framework-schema -p semio-framework-schema-registry -p semio-s-artifact-stdio-contract --lib
step value cargo test --offline --no-fail-fast -p semio-framework-replication --lib edit_through_value
step roots cargo test --offline --no-fail-fast $root_args --features "${(j:,:)root_features}" --lib -- --test-threads 4
step pptxregen zsh -c 'cargo test --offline -p semio-s-artifact-stdio-pptx --features semio-s-artifact-stdio-pptx/component-app-assembly --lib -- --ignored zzz_write_demo_fixtures && cargo test --offline -p semio-s-artifact-stdio-pptx --features semio-s-artifact-stdio-pptx/component-app-assembly --lib fixture_honesty_law'
mkdir -p "$C/$tag-part21-oracle"
step part21 zsh -c "cargo test --offline -p semio-s-artifact-stdio-contract --lib part21 && SEMIO_PART21_ORACLE_OUT='$C/$tag-part21-oracle' cargo test --offline -p semio-s-artifact-stdio-ifc --features semio-s-artifact-stdio-ifc/component-app-assembly --lib committed_ifc2x3_fixtures_read_through_the_canonical_part21_codec && python3 /Users/ueli/Documents/semio/.tmp-ticket/wp-lb2/lb2-p16-part21-oracle.py '$C/$tag-part21-oracle'"
echo "PROOF END $(date '+%T')"
