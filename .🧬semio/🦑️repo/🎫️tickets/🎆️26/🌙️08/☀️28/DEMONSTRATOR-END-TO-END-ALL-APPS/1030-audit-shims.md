# 🔎️ 1030 Shim Audit (lane shimaudit)

STATUS: done (read-only). Scope: uncommitted diff under `🧰️framework/🔨️modules`, `🧰️framework/🛍️products`, `✏️s/🔌️plugins`, `🌎️hub` (183 files, 2666 added lines; 176 files modified within 48 h). Method: added lines extracted from `git diff -U0` (saved in `C/🗑️generated/1030/shimaudit-added.txt`) and grep-scanned per rule. Not a line-by-line read of every file, so treat "no finding" as "no pattern hit".

## Findings

| # | file:line | rule | evidence | owner lane |
|---|-----------|------|----------|------------|
| 1 | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🔬️bestest-load/🦀️.rs:42` | `[DEBUG] ` temp log must be removed | `eprintln!("[DEBUG] owned {spec:#?}")` | energy |
| 2 | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧬️original/🧪️tests/🦀️.rs:22` | `[DEBUG] ` temp log | `eprintln!("[DEBUG] sizes diff=... patch=... shared=...")` | energy |
| 3 | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧪️tests/loads/receiver-tests.rs:250` | `[DEBUG] ` temp log | `eprintln!("[DEBUG] {e:?}")` | fem |
| 4 | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🔨️modules/🏠️host/🧰️owned/🚀️store-initialization/🦀️.rs:24-25` | no `.unwrap()/.expect()` in non-test code | `raster_document_store_owners_source_demands().expect("Raster owner catalog source layout")` and `.expect(...)` on the fund call in production init code | runtime |
| 5 | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/✏️editor/🎚️config/🦀️.rs:121` | no `.expect()` in non-test code | `variants.iter().find(...).map(...).expect("v...")` inside a config lookup, reachable at runtime | hubplug |
| 6 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌱️initialization/♻️retirement/🦀️.rs:44-49` | naming / dual path review (compat-adjacent) | new `cold_replay_document_store_owners[_source_demands]` used by 4 plugin editors (generation2d, generation3d, process3d, plus playground `cold_mutation_apply_preparation_factory`); the "cold" family runs beside the retained path. Verify it is the only replay path and not a parallel legacy route | runtime |
| 7 | `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🔺️diff/🦀️.rs:92-101` | dual API review | `apply_diff_cold_into` and `apply_diff_cold` sit next to `commit_retained_diff`; doc says the central applier is the only entry. Two entry points for one diff must be justified as distinct semantics (scratch vs live), not as an alternate path | mutderive |
| 8 | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/⚙️compute/🦀️.rs:34` | no-op / ignored input in a hook | `pub fn cold_turn_fault(_failure: OriginalTurnFailure) -> WidgetFault` discards its argument and returns a constant fault. Either thread the failure into the fault or delete the parameter | gen |
| 9 | same file, line 24 | naming (test-oracle vocabulary in production code) | docstring "Runs one cold-oracle stepping turn" on a non-test `pub fn with_cold_step_context` | stepctx |
| 10 | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/✅️analysis-check/🦀️.rs:237,362` | naming | `found.cold_insert("shape"...)` and `cold_insert("volume"...)`; `cold_` prefix on a generic insert is unclear, check it is not a second insert API beside a warm one | gen |
| 11 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts:373-376` | no swallowed/dual paths; `!` non-null assertion | `candidateProviderOnce` + memoized `candidateProvider` with `candidateMemo.get(key)!`. Not a shim; the non-null assertion is unchecked by the type system | deps |
| 12 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧬️schema/📥️install.json:4903-4904` | name review (fallback vocabulary) | `"unownedFallback": "not-consulted"`, `"ambientFallback": "not-consulted"`. Values declare that no fallback is used, so this reads as an explicit ban, not a fallback path. Consider renaming to avoid the word fallback | deps |
| 13 | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/🧬️schema/🧬️mutations/🦀️.rs:353,358` | `//#region` markers in production code | same file | are `//#region` / `//#endregion` comment markers. Low severity; they are comments outside definitions, but delete for concise-code rule | hubplug |

## Checked and clean

- `#[allow(...)]`, `eslint-disable`, `@ts-ignore`, `as any`: no added hits.
- Legacy/compat/shim/bridge/interim/adapter names: no added hits, except the json field names in row 12.
- `[DEBUG]` prints outside tests: none beyond rows 1-3.
- `unsafe impl`: one, `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🌳️value/🦀️.rs:51` (`Send` for `NativeReceiveSlot`). It has a soundness docstring (🧵️) naming the three field contracts and a law test. Compliant, owner resid/runtime to confirm the law test exists.
- Doc comments: every `///`/`//!`/`/**` starting a doc block begins with an emoji. Four continuation lines were flagged by the grep but are not docstring starts.
- Comments inside function bodies in added TS: none. In added Rust: only the two `//#region` markers (row 13).
- `let _ =`: only in a test file (`🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧪️tests/🦀️.rs:95`, compile-time Send check). Fine.
- `todo!/unimplemented!`: none. Empty trait default bodies: only test helpers.
- Unwrap/expect in test files were not counted (`🧪️tests`, `tests/`, `🔬️`).

## Owner summary

- energy: rows 1, 2
- fem: row 3
- runtime: rows 4, 6
- mutderive: row 7
- gen: rows 8, 10
- stepctx: row 9
- hubplug: rows 5, 13
- deps: rows 11, 12

Verdict: 5 rule violations to fix (rows 1-5), plus 1 minor comment-marker issue (row 13), 3 review items (rows 6-8), 4 naming/notes (9-12). No compat shims, adapters or legacy paths were introduced in the scoped diff.
