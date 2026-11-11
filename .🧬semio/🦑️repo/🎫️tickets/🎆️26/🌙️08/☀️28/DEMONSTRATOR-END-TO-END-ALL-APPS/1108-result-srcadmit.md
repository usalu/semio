# srcadmit result

Tokenizer (`selectRustSourceV1` in `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts`, using `rustTokens`/`rustTokenPairs`) is CORRECT; not touched.
The Rust was genuinely unbalanced (rustc would also reject it): file `.../💡️inferences/🧷️wave3-rows/🦀️.rs`, three lines of the form `copy(&format!(A), &format!(B), annex)` were missing the closing `)` of `copy(` (every sibling line has `copy(&format!(A), &format!(B)), annex`):
- `check_fatigue_model` FLM3 `info_check` line (`document.bridge_span), annex));`)
- `check_local_fire` flame line 113 (`/ 1e6), annex));`)
- `check_local_fire` plume line 126 (`row.ceiling_height), annex));`)
Fixed by adding `)` on each. Token-pair scan (bun script over all 22 files of the crate) now reports 0 unpaired delimiters; per-line depth scan clean.

Rerun: `"$C/1105b-tools/chk.sh" W-srcadmit-1 🌎️hub/Cargo.toml wasm -p semio-s-artifact-norm-en1991-inferences` (queued behind capacity at time of writing).

## Verification W-srcadmit-1
No "Unbalanced Rust source delimiter" refusal any more; cargo ran (rc 1). Remaining errors are all in `semio-plugin-job` upstream (`../../🏛️reserved/🦀️.rs`, `🏛️registration/🦀️.rs`, `🦀️.rs:736`: RetireOwned/RetirementCursor trait drift, `admit_shared_retirement`), none in the en1991 crate: owned by the retirement-trait peers, not this lane.
Hub-wide `hub.sh` should now pass admission; en1991-inferences itself compiles only once semio-plugin-job is green.

STATUS: FIXED-verified W-srcadmit-1
