STATUS: FINDINGS - 0 TODO/FIXME, 0 paths >=256, 0 migration files; 9 leftover [DEBUG] eprintln (framework lane, must be removed), 3 new #[allow] (1 emitted in hub), ~873 non-test unwrap/expect lines added (framework dominant), 1 "temporary adapter" and 1 "legacy" naming in new code; 17 unwrap hits uncommitted.

# 1102 Policy Audit (aud-policy, read-only)

Scope: uncommitted diff (`git diff -U0 HEAD`, 293 files) plus commits since 2026-10-30 (`15d755f053c`, `ccac6866cc1`, `ee829ef7c40`, `651c804f155`) under `🧰️framework/`, `✏️s/`, `🌎️hub/`, `♻️mit-bestand/`. Added lines only. Test paths (`🧪️`, `tests/`, `*_tests.rs`, `tests.rs`) excluded from production checks; Rust hits additionally filtered for lines after a `#[cfg(test)]` marker in the same file. Commit line numbers refer to that commit's version; `wt` = working tree.

Raw data (scratch, not in the tree): `/private/tmp/claude-501/-Users-ueli-Documents-semio/2c8171ff-0be5-4b23-8c30-2258199d4040/scratchpad/` (`addnum.txt`, `prod_unwrap_owned.json`, `scan.json`).

## Owner: framework

| file:line | rule | snippet | owner |
|---|---|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/📬️publication/🔁️stall-witness-and-unit-advance/🦀️.rs`:59 (15d755f) | leftover [DEBUG] log | `eprintln!("[DEBUG] stall {verb} streak=...` | framework |
| `🧰️framework/🔨️modules/🧵️job/📬️outcome/👷️worker/🦀️.rs`:32 (15d755f) | leftover [DEBUG] log (static counter) | `eprintln!("[DEBUG] admit_step builder n=...` | framework |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/🚪️session/🦀️.rs`:26 (15d755f) | leftover [DEBUG] log | `eprintln!("[DEBUG] admit_pending n=...` | framework |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🔄️full-operation/🦀️.rs`:214 (15d755f) | leftover [DEBUG] log | `eprintln!("[DEBUG] drive_worker_step n=...` | framework |
| same file:320 (15d755f) | leftover [DEBUG] log, fuel-modulo gated | `eprintln!("[DEBUG] worker poll {other:?} ...` | framework |
| `🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs`:209 (ccac6866) | new #[allow] | `#[allow(clippy::too_many_arguments)]` | framework |
| same file:533 (ccac6866) | new #[allow] | `#[allow(clippy::too_many_arguments)]` | framework |
| `🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🤝️kerning/🟦️.ts` (ccac6866) | legacy naming / possible compat path | `private legacyReader:FontReader\|null=null;` | framework |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🛠️build-tooling/🟦️.ts` (15d755f) | "temporary adapter" in docstring (compat wording) | `/** ⚛️ React's temporary build adapter behind the UI package...` | framework (ui) |
| `🧰️framework/🔨️modules/🧵️job/🕰️clock-identity-and-budget/🦀️.rs` (5 first-line docs) | docstring not starting with emoji | `/// the two-bound budget, cancellation, the clock...` | framework |
| `🧰️framework/🔨️modules/🧵️job/📬️outcome/📦️builder/🦀️.rs`, `🛠️tool-machine/⌨️typing/🦀️.rs`, `🛎️services/🧰️compute-pool/🦀️.rs` (15d755f / ccac) | docstring not starting with emoji | `/// runs on \`caller\`, the original source that funds...` | framework |
| `🧰️framework/🔨️modules/🧵️job/📬️outcome/👷️worker/🦀️.rs` and `🔨️modules/🔌️plugin/👷️job/🏛️reserved/🦀️.rs` (ccac) | "generated migration" wording in docstring | `/// Admits each proof row only when ... generated migration` | framework |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🕸️wasm-build/🟦️.ts`:159 (ccac) | console.log in production (not [DEBUG]) | `console.log(\`[${logPrefix}] owned ${policy.mode}/...` | process |
| `🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts` (15d755f) | 16 added limit/budget/timeout constants (not diffed against base; unverified) | e.g. `maximum...=...` | framework |
| `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs` and 800+ other framework rs lines (ccac 580 / 15d755f 269 / ee829 7) | non-test `.unwrap()`/`.expect(` added (873 total after cfg(test) filter) | top files: `🏪️store/📬️command/🛬️receiving/🪜️cursor/🦀️.rs` 24, `🌱️value/🗂️ordered/🦀️.rs` 14, `🧵️job/📬️outcome/⏸️continuation/🎟️origin/🦀️.rs` 11, `🪟️window/🎚️config/📥️retained/📦️typed-pack-load/🛬️native/🦀️.rs` 11, `🔲️pixels/🧩️compositing/🗂️layers/📋️plan/🦀️.rs` 10, `◻️2d/🔍️trace/🦀️.rs` 10 | framework |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/🔗️history/📜️report/🦀️.rs`:40,48,55 (wt) | uncommitted non-test unwrap | `owner.history_source.is_none()...` / `ok_or_else(...)?` | framework |
| `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/📥️default/🦀️.rs` (doc block) | compile_fail doc only, no violation | n/a | - |

## Owner: repo-tooling

| file:line | rule | snippet | owner |
|---|---|---|---|
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/⚖️truth/🟦️.ts`:225,227 (ccac) | console.error/log in production, cap `slice(0, 100)` (raised output limit?) | `for (const leak of leaks.slice(0, 100)) console.error(...)` | repo-tooling |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🟦️.ts`:250,270 (ee829) | console.log/console.error in production; catch swallows non-abort errors | `catch(error){if(...aborted)throw error;console.error(error);}` | repo-tooling |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧰️native/🔌️port/🟦️.ts`:35 (ee829) | console.log as port logger | `log: (text: string) => console.log(text),` | repo-tooling |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🎮️command/🟦️.ts`:120 (ee829) | console-free; listed for review of cleanup path only | n/a | repo-tooling |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts` (wt) | requireCompatibleCargoFlagViewsV1 new, untested by unit row per brief; name contains "Compatible" (review) | `export function requireCompatibleCargoFlagViewsV1(...)` | repo-tooling |
| `🧰️framework/...🦑️repo/...` Go files (ee829, `🐹️.go`) | ~93 added `//` lines, all top-level doc comments (not in bodies); no violation found | e.g. `// 🧭️registeredBatchTarget recognizes ...` | repo-tooling |

## Owner: stdio

| file:line | rule | snippet | owner |
|---|---|---|---|
| `🌎️hub/🧩️compositions/🗄️stdio/🧩️composition/🟦️.ts`:53 (ccac) | new #[allow] emitted into generated Rust source | `const source = \`//! ... #![allow(async_fn_in_trait)]` | stdio (hub) |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🏭️pack-tables.ts`:19-20 (15d755f) | console.log in production schema tool | `console.log("program sha256",programDigest(program));` | stdio |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📚️catalog/🛠️extract/🏭️pack-all.ts`:5 (ccac) | console.* in production extract tool (verify) | `for(const standard of standards){...` | stdio |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🚪️io/💾️binary/🎒️record-document/🟦️.ts`:2-3 (ccac) | license header comment, not a violation | `// 2026 Ueli Saluz ...` | stdio |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🚪️io/🪟️details/field_edit.rs`:35,36 (wt) | uncommitted non-test unwrap | `self.receiving.as_mut().unwrap().tak...` | stdio |
| `✏️s/.../🏗️ifc/.../🪟️details/work.rs`:20,29 (wt) | uncommitted non-test unwrap | `if self.pending.is_none()&&...` | stdio |
| `✏️s/.../🏗️ifc/🏅️standards/🔖️4x3/.../🏷️templates/✍️apply/staging-native/🦀️.rs`:149 (wt) | uncommitted non-test unwrap | `19=>{... self.query.close_s...` | stdio |
| stdio other rs unwrap (15 more, ccac/15d) | non-test `.unwrap()`/`.expect(` added | `✏️s/.../🧬️schema/⚖️validation/🧮️evaluator/🦀️.rs` 6 hits | stdio |
| `✏️s/.../🗄️stdio/.../🗿️artifacts/📽️pptx/.../🧬️mutations/🦀️.rs` (ccac) | "markup-compatibility" is the OOXML spec term; not a compat layer | n/a | stdio |

## Owner: gen (generation3d, wfc, puzzle, procedural)

| file:line | rule | snippet | owner |
|---|---|---|---|
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️artifact-editor-impl/🦀️.rs`:450 (wt) | uncommitted non-test unwrap | `"flowTessellateCancelResolve"=>{let input_valid=original_r...` | gen |
| `✏️s/.../🌀️procedural/.../👁️viewer/🔭️viewer-and-interaction-topology/🦀️.rs`:245 (wt) | uncommitted non-test unwrap | same pattern | gen |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/.../🧬️mutations/🔤️text/↩️inverse/🦀️.rs`:37,38 (wt) | uncommitted non-test unwrap | `1=>{let index=self.plan.unwrap().index.unwrap();` | gen |
| `✏️s/.../🧩️puzzle/.../🟢️node/🗑️delete-node/🎮️prepare/↩️inverse/🦀️.rs`:62,86,128 (wt) | uncommitted non-test unwrap | `let step=self.node.advance(source.project(1,...` | gen |
| `✏️s/.../🧩️puzzle/.../📸️snapshot/🔎️lookup/🦀️.rs`:70 (wt) | uncommitted non-test unwrap | `match self.comparison.compare(source.project(1, ...` | gen |
| gen other rs unwrap (~11 more, ccac) | non-test `.unwrap()`/`.expect(` added | `🧩️puzzle`, `🀄️wfc` | gen |

## Owner: fem / energy / cad / flow / gis

| file:line | rule | snippet | owner |
|---|---|---|---|
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/.../⚖️cold-laws/🦀️.rs` (ccac) | non-test unwrap/expect added, 13 hits | `let mut backward = mutation.inverse(...` | fem |
| `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/.../📸️snapshot/🧊️reference/🦀️.rs` (ccac) | "Compatibility value access" doc wording (review for compat layer); 7 unwrap hits | `/// ♿️ Compatibility value access` | energy |
| `✏️s/🔌️plugins/🔋️energy/.../📸️snapshot/🧬️original/🦀️.rs` (15d755f) | `macro_rules! alias` added (review) | `macro_rules! alias{($field:expr)=>{...` | energy |
| `✏️s/🔌️plugins/🖍️draw/.../🧬️schema/🎬️scene/...🧪️tests/` (15d755f) | `#[allow(unused_imports)]` in tests only; not a violation | `#[allow(unused_imports)] use ...` | cad |
| `✏️s/🔌️plugins/📏️layout/.../✏️editor/🧪️tests/` (ccac) | println! in tests only; not a violation | `println!("sdk-actor-admission=...` | cad |
| `✏️s/🔌️plugins/🖍️draw/.../🧬️schema/🎬️scene/` cad-area unwrap (wt, ccac) | 7 cad unwrap hits | n/a in wt list | cad |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🥽️mesh/🧮️operator-core/🦀️.rs` (15d755f) | `Default::default()` struct init with `...` (no violation) | n/a | flow |
| `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/.../📬️one-item-preparation/🦀️.rs`:370 (wt) | uncommitted non-test unwrap | `if let Some(slot) = self.factories.iter_mut().find(...` | gis |

## Repo-wide notes

- TODO/FIXME/XXX/HACK in added code: 0.
- Migration or legacy/compat/shim/adapter/deprecated file names in the closure: none.
- Paths >=256 chars: none (longest absolute path 212 code points; repo-relative max 212).
- Comments inside function bodies: none found in production Rust/TS, except `♻️mit-bestand/🧺️demonstrator/🟦️.tsx`:51-52 (15d755f) which may sit inside a component body (verify placement; owner demonstrator). `#region` markers in `♻️mit-bestand/🧺️demonstrator/⚛️locale-picker.tsx`:1,3 and `🌐️locale/⚛️.ts`:1 are top-level, acceptable.
- Single-language UI strings: NOT checked (no tool support in this pass); unverified.
- Limits raised: 289 added non-test lines set max/limit/timeout/budget values; base values were not diffed, so raises are unverified.
- Heuristics: unwrap filter treats everything after the first `#[cfg(test)]` in a file as test code; `*_tests.rs` skipped. Some hits may be in helper fns before a test module.
