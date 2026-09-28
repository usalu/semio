#!/bin/zsh
# ✅ LB2 session 14b items 2 + 5 on the live tree (after the overnight Codex stdio edits), one native-lane hold per cargo:
# lb-p4 check (stdio plugin + semio artifact, default features), shipped_fleet law, lb-p1 brep lib tests, item 2's native proof
# (editor_catalog laws of the text family: setSnapshotValue through the host-addressed action + undo/redo, details en + de), brep parity.
cd /Users/ueli/Documents/semio || exit 2
zsh .tmp-ticket/wp-lb2/cargo-lane.sh i5-check-default-2 check --keep-going -p semio-s-plugin-stdio -p semio-s-artifact-stdio-semio --lib --tests
zsh .tmp-ticket/wp-lb2/cargo-lane.sh i5-test-shipped-2 test -p semio-s-plugin-stdio --test shipped_fleet --no-fail-fast
zsh .tmp-ticket/wp-lb2/cargo-lane.sh i5-test-semio-lib-2 test -p semio-s-artifact-stdio-semio --lib --no-fail-fast -- brep
zsh .tmp-ticket/wp-lb2/cargo-lane.sh i2-editor-catalog-2 test -p semio-s-plugin-stdio --features full-app-catalog --test editor_catalog --no-fail-fast -- html_editor md_editor txt_editor json_any_editor json_i_json_editor xml_any_editor xml_valid_editor csv_editor tsv_editor
zsh .tmp-ticket/wp-lb2/parity-lane.sh i5-parity-brep-2 🧊️mutate-semio-brep
echo "ALL DONE $(date '+%H:%M:%S')" > .tmp-ticket/wp-lb2/generated/i5-done-2.txt
