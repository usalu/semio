#!/bin/zsh
# ⚖️ LW1 → T6 round 2 part 1 (LB2 rows 3–5c, S19 row 3b) on the live tree in the owners' own forms (`lb2-scratch-proof.sh`,
# `lb2-scratch2-proof.sh`, `s19-t6-proof.sh`): stdio shipped_fleet + editor_catalog, family descriptor freshness, SDK builder, schema +
# stdio contract, value edit_through_value, the 21 stdio roots (app-assembly). Descriptor-freshness / committed-descriptor census reds
# are EXPECTED until the chain's describe regen. Part 2: block g2.
R=/Users/ueli/Documents/semio
source "$R/.tmp-ticket/wp-lw1/lw1-budget.sh"
cd "$R" || exit 2
roots=(bmp wav epw binary ifc gif semio avi dwg dxf jpg las pdf png pptx tiff xml zip docx xlsx step)
root_args=(); root_features=(); for r in $roots; do root_args+=(-p "semio-s-artifact-stdio-$r"); root_features+=("semio-s-artifact-stdio-$r/component-app-assembly"); done
families=(image media cad bim mesh pdf office semio binary); family_args=(); for f in $families; do family_args+=(-p "semio-s-plugin-stdio-$f"); done
step stdio-catalog-fleet 900 cargo test --offline -p semio-s-plugin-stdio --lib --test shipped_fleet --test editor_catalog --no-fail-fast -- --test-threads 4
step families-descriptor-fresh 900 cargo test --offline --no-fail-fast $family_args --lib -- descriptor_is_fresh
step sdk-builder 420 cargo test --offline --no-fail-fast -p semio-framework-plugin --lib builder:: -- --test-threads 1
step schema-contract 600 cargo test --offline --no-fail-fast -p semio-framework-schema -p semio-framework-schema-registry -p semio-s-artifact-stdio-contract --lib
step value-edit-through 420 cargo test --offline --no-fail-fast -p semio-framework-replication --lib edit_through_value
step stdio-roots 1500 cargo test --offline --no-fail-fast $root_args --features "${(j:,:)root_features}" --lib -- --test-threads 4
finish
