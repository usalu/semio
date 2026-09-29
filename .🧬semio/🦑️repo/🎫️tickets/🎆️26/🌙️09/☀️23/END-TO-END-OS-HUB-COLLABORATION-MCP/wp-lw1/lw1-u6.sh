#!/bin/zsh
# ⚖️ LW1 → U6 laws (T3 u6-fragment-annotation-siblings, u6-bcf-mesh-publication, u6-publication-fault-terminal, u6-xlsx-canonical-save,
# u6-docx-set-page-owner), in U6's form: `cargo test --lib [--features <crate>/component-app-assembly]` per crate + the AJV oracle.
R=/Users/ueli/Documents/semio
cargo test --offline --no-fail-fast -p semio-framework-schema --lib; echo "LW1-STEP schema-lib rc=$?"
cd "$R/🧰️framework/🔨️modules/🧬️schema" && bun test "./🧪️tests/🩹️fragment-validation-oracle/🟦️.ts"; echo "LW1-STEP fragment-ajv-oracle rc=$?"
cd "$R"
for c in semio-s-artifact-stdio-wav semio-s-artifact-stdio-bcf semio-s-artifact-stdio-semio semio-s-artifact-stdio-xlsx semio-s-artifact-stdio-docx; do
  cargo test --offline --no-fail-fast --lib --features "$c/component-app-assembly" -p "$c" -- --test-threads 4; echo "LW1-STEP $c rc=$?"
done
