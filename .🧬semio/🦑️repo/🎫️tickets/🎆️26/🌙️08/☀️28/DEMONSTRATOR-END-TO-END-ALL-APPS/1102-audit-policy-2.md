STATUS: FINDINGS - 1 leftover [DEBUG] eprintln in production (flow lane), 18 non-test unwrap/expect/unreachable added in uncommitted tree, 0 new #[allow], 0 TODO/FIXME, 0 comments inside bodies; limit constants and English-only UI strings need review (unverified).

# 1102 Policy Audit 2 (aud-policy, read-only, rerun)

Scope: demonstrator closure paths `🧰️framework/`, `✏️s/`, `🌎️hub/`, `♻️mit-bestand/`. Commits since 2026-11-02 00:00 in these paths: 0. Uncommitted diff: 803 files, 16807 added lines (`git diff -U0 HEAD`). Test paths (`🧪️`, `tests/`, `*_tests.rs`, `testing/`, `*.test.*`) excluded from production checks. Rust unwrap/panic hits additionally excluded when after a `#[cfg(test)]` marker in the same file. Raw data: `/private/tmp/claude-501/-Users-ueli-Documents-semio/2c8171ff-0be5-4b23-8c30-2258199d4040/scratchpad`.

## Leftover [DEBUG] logs (production)

| file:line | rule | snippet | owner |
|---|---|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🏁️host-close/🦀️.rs`:85 | leftover [DEBUG] eprintln (same line also `.expect("cold host copy authority")`) | `eprintln!("[DEBUG] ...` in while-loop body | flow |

Excluded (tests, not production): `🧪️tests/.../undo-redo-and-gesture-journal-laws/🦀️.rs`:142 and the `dependencies/🧪️tests` TS line.

## #[allow] / limits / compat / comments / UI

| file:line | rule | snippet | owner |
|---|---|---|---|
| `🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust/🦀️.rs`:4 | none: `//!` doc text mentions `#![allow(async_fn_in_trait)]`, not an attribute | n/a | framework |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts`:75 | compat wording in new function name `requireCompatibleCargoFlagViewsV1` (review naming) | `export function requireCompatibleCargoFlagViewsV1(` | repo-tooling |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts`:988 | `adapter` identifier in new bazel publish script (review for adapter layer) | `const {workspace,plan,implementation,adapter,scopes}=await profilePhaseV1(` | repo-tooling |
| new numeric limit constants, prod (not diffed against base, raise unverified), e.g. `♻️`/`✏️s` schema `maximumItems:4096`, `maxSegments:65536`, `maxNodes:1024`, `max_pixels: 16_777_216`, `max_edges: 65536`, `maximum_items: 4096` | limits must not be raised | see scratch list | owners per path (gen/cad/framework) |
| `🧰️framework/🔨️modules/◻️2d/🔍️trace/🟦️.ts`:160 | new `setTimeout(` deadline (review value) | `setTimeout(` | framework (2d) |
| comments inside definitions | none found in prod code (indented `//` search: 0) | n/a | - |
| single-language UI strings | NOT verified: heuristic found 1 candidate, a build receipt (not UI); `♻️mit-bestand` UI lines unchecked for de counterpart | n/a | demonstrator |

Fuel/stack: no new fuel or stack constant raises detected in prod code by pattern; capacity zero-initialisers (`capacityBytes:0`, `capacity_bytes:0`) are grants, not limits.

## Unwrap / expect / unreachable in non-test code (uncommitted, cfg(test) filtered)

| owner | file:line | rule | snippet | owner lane |
|---|---|---|---|---|
| cad | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🦀️.rs`:137 | .expect( | `fn advance(&mut self,s:ChildAdmissionSource<'_>,g:RetainedCloneGrant)-` | cad |
| cad | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/📷️raster/🦀️.rs`:106 | unreachable! | `"nodes"=>{if self.current.is_none(){if self.next==self.input.nodes.len` | cad |
| cad | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔎️lookup/🔗️owned/🦀️.rs`:46 | .expect( | `let (layers, progress) = self.source.as_ref().expect("retained native ` | cad |
| cad | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔎️lookup/🔗️owned/🦀️.rs`:70 | .expect(,unreachable! | `let (layers, progress) = self.current.as_ref().expect("compared native` | cad |
| cad | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔎️lookup/🦀️.rs`:83 | .expect( | `let step = self.comparison.compare(current.project(1, crate::layer_id)` | cad |
| cad | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`:68 | .expect( | `if matches!(cursor.close_step(grant).result.expect("Draw owner retirem` | cad |
| cad | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️geometry/🎯️picking/🎨️paint/🦀️.rs`:73 | unreachable! | `"cleanup"=>{if self.retire_one(ledger)?{self.phase="complete";}return ` | cad |
| framework | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🏁️host-close/🦀️.rs`:85 | .expect( | `while !retirement.terminal_is_empty(){let copy=retirement.next_close_c` | framework |
| framework | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🦀️.rs`:41 | .expect( | `while !self.queue.has_reserved_slot(){self.reserve_push(RetainedCloneG` | framework |
| framework | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/🔗️history/📜️report/🦀️.rs`:40 | .unwrap() | `if owner.history_source.is_none(){let slot=if let Some(order)=order{*o` | framework |
| framework | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/🔗️history/📜️report/🦀️.rs`:55 | .unwrap() | `let base=replay.state.as_ref().ok_or_else(\|\|VcsError::HistoryReplaying` | framework |
| framework | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📄️docs/📥️staging/🪜️maintenance-ladder/🦀️.rs`:259 | .expect( | `let owner=active.hydration.as_mut().expect("recursive document parent ` | framework |
| gen | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️artifact-editor-impl/🦀️.rs`:450 | .unwrap() | `"flowTessellateCancelResolve"=>{let input_valid=original_reply_valid(&` | gen |
| gen | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🔭️viewer-and-interaction-topology/🦀️.rs`:245 | .unwrap() | `"flowTessellateCancelResolve"=>{let input_valid=original_reply_valid(&` | gen |
| gen | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🔨️modules/🏠️host/🦀️.rs`:184 | .expect( | `owner.close_step(grant).result.expect("cold displaced replay close");` | gen |
| stdio | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🚪️io/🪟️details/work.rs`:20 | .unwrap() | `fn work_demands(&self,input:&ArtifactCommandInputs<'_,EditorApp<E>>,bo` | stdio |
| stdio | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🏷️templates/✍️apply/staging-native/🦀️.rs`:149 | .unwrap() | `19=>{if !self.query.terminal_is_empty(){self.query.close_step(cx)?;}el` | stdio |
| 🖨️raster | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🖼️asset-content-and-patch/🦀️.rs`:238 | .expect( | `owner.close_step(grant).result.expect("Raster cold retirement closes w` | 🖨️raster |

Total: 18 hits in production code.
