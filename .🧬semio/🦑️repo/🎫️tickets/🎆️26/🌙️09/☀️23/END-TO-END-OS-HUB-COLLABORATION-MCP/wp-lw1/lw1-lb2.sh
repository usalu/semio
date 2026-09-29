#!/bin/zsh
# ⚖️ LW1 → LB2 laws (T3 lb2-p3-wg11 SDK half, capacity-true): SDK table laws, renderer-react typecheck + Interpreter vitest (LB2's joint
# runbook). p5 docx/xlsx libs ran in the U6 block (u6-laws-1).
R=/Users/ueli/Documents/semio
cargo test --offline --no-fail-fast -p semio-framework-plugin --lib -- home_shaped_rows table_kit table_window; echo "LW1-STEP sdk-table-laws rc=$?"
cd "$R/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript" && bun ./📜️script.ts typecheck; echo "LW1-STEP renderer-react-typecheck rc=$?"
bun ./📜️script.ts test long "🗣️Interpreter/🟦️.tsx"; echo "LW1-STEP interpreter-vitest rc=$?"
