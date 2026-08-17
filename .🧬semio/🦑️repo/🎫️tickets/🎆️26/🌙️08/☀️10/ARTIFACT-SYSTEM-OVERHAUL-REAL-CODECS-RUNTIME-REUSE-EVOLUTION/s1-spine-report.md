# S1 Spine Wave — Report

Plan: `~/.claude/plans/the-current-schemas-are-scalable-journal.md`. W0 recon: `w0-recon-report.md` in this folder. Baseline confirmed at session start: `cargo test -p semio-s-plugin-stdio --lib` → 318 passed, 0 failed.

## Step A — S-1 (additive)

Added a new standalone `pub trait DiffAlgebra<P> { fn inverse(&self, base: &P) -> Self; fn between(base: &P, other: &P) -> Self; fn is_empty(&self) -> bool; }` next to `MutationDiff` in `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs`, immediately after the `MutationDiff` trait, following the file's own `DiffCodec` precedent (separate trait, doc comment citing the seeded-policy adoption path, zero implementors this wave). Also added the normative absorb-contract doc comment to `MutationDiff::absorb` (structural / total / base-free / sequential-coalesce-only, per the plan's `## Absorb` section) — no behavior change, doc only.

Confirmed **zero** `impl DiffAlgebra` anywhere in the repo (grep), as required ("do NOT implement DiffAlgebra for any concrete type in this wave").

`cargo check -p semio-framework-os-kernel` — clean, standalone.

## Step B — S-2 then S-3, flip-then-sweep-then-flip-then-sweep

### S-2: `ArtifactBuilder::mutate(self, mutation) -> (Self, Self::Diff)`

File: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (~line 428). Flipped the signature and added the `type Mutation: protocol::Mutation<Self::Snapshot, Diff = Self::Diff>` bound the plan specifies.

**Mechanical sweep — 252 repo-wide `impl ArtifactBuilder for` sites** (full list: `s1-artifacts/s1-artifactbuilder-252-files.txt` in this folder). Found and fixed 5 distinct body shapes, all behavior-preserving (mutation still applies exactly as before; only the return value changes):

1. **31 stdio leaf builders** (the `🪆️subsets/✳️any/🏗️builder` level, one per standard) — body was `apply_x_mutation(&mut self.snapshot, &mutation); self`. Changed to `let diff = apply_x_mutation(&mut self.snapshot, &mutation); (self, diff)`, which requires `apply_x_mutation` itself to return the diff (see below).
2. **21 non-stdio leaf builders** using the same free-function-mutates-in-place shape — since the plan only mandates flipping stdio's `apply_*_mutation` signatures, these were fixed without touching the (untouched, still `()`-returning) free function: compute the diff first via the already-existing generic `<Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot)`, then call the free function, then return `(self, diff)`.
3. **27 files** already computing a diff via `Mutation::diff` then applying it (`let d = <XMutation as protocol::Mutation<XSnapshot>>::diff(...); self.snapshot = ...::apply(&d, ...); self`) — trivial: return `(self, d)` instead of `self`.
4. **165 files** — the pure delegator shape `Self(self.0.mutate(mutation))` (root→standard and standard→subset facades). Changed to `let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff)`.
5. **8 remaining outliers** (2 stdio `binary` root/standard delegators using a multi-line delegate form, 1 `cad` file using variable name `diff` instead of `d`, 5 "functional-leaf" files — `forms`/`playbook`/`draw`/`raster`/`note` — whose free function takes `&self.snapshot` and *returns* a new snapshot rather than mutating in place) — each fixed by hand to the same pattern family as 2–4 above.

**apply_*_mutation flip (the 31 stdio standards' own mechanical requirement)**: each standard's `pub fn apply_x_mutation(snapshot: &mut XSnapshot, mutation: &XMutation)` was changed to return `XDiff`, computed as `let diff = <XMutation as protocol::Mutation<XSnapshot>>::diff(mutation, snapshot); <original unchanged body>; diff` — i.e. the diff is captured via the mutation's own already-correct `.diff()` method *before* the field-level mutation runs, and the original mutation logic is left byte-for-byte unchanged. This is uniform across all 27–28 generic-template standards and the 3 partial ones (gif 89a, pdf 1.7, svg).

**Real bug caught and fixed**: svg's `Mutation::diff` is apply-and-capture — for any non-trivial variant it clones the base snapshot and calls `apply_svg_mutation` on the clone to compute the diff (this is svg's own known architectural defect, flagged by W0, deferred to F-wave). Wrapping `apply_svg_mutation` to call `.diff()` at its own top (the pattern used for the other 30 standards) therefore created **infinite mutual recursion** (`apply_svg_mutation` → `.diff()` → `apply_svg_mutation` → …), confirmed as a real stack-overflow test crash (`insert_then_remove_element_apply_and_inverse`), not a false alarm. Fixed by deriving svg's diff *after* mutating (mirroring exactly what `.diff()`'s apply-and-capture arm already produces, without calling back into `.diff()`): mutate in place unchanged, then `match mutation { NoMutation => default, SetSnapshot{next} => diff_set_snapshot(next), _ => diff_set_snapshot(snapshot) }`. Verified no other of the 31 standards has `.diff()` calling its own `apply_*_mutation` (grepped every standard's `fn diff` body).

**Gate**: `cargo check -p semio-s-plugin-stdio` — 0 errors. `cargo check -p semio-framework-os-kernel` — 0 errors. `cargo test -p semio-s-plugin-stdio --lib` — 332 passed, 0 failed (see Verification section for the count discrepancy explanation).

### S-3: delete dead `ArtifactEngine` trait

File: `🧰️framework/🛍️products/💻️os/🔨️modules/⚙️engine/🦀️component.rs` (~line 81, `//#region 🔖️ArtifactEngine`). Deleted the trait definition entirely (kept `EngineFault`/`Engine`/`EngineCache` — the unrelated byte-cache-kernel machinery — untouched; confirmed `EngineFault` has many other real users so it stays).

**85 repo-wide `impl (protocol::)?ArtifactEngine for X { ... }` blocks removed** (full list: `s1-artifacts/s1-artifactengine-85-files.txt`), via a brace-matched deletion that removes only the trait-impl block — the struct definition and its inherent `impl X { pub fn new(...) }` (and any other inherent methods) are left in place in every file, confirming the plan's claim that per-artifact `⚙️engine` files stay as codec homes. Re-confirmed zero real construction sites (`dyn ArtifactEngine`, `<E: ArtifactEngine>`, `ArtifactEngine::new`) before deleting — the one `ArtifactEngine::new`-shaped hit found is an unrelated concrete struct literally named `PlaygroundArtifactEngine`, not the trait.

Two files initially flagged by a naive brace-counting sanity script as "unbalanced" turned out to be false positives (test fixtures containing string literals with a lone unmatched `{`, e.g. `parse_bindings_json("{not json")`) — both files' crates (`semio-s-plugin-trinity`, `semio-s-plugin-puzzle`) compile clean, confirmed directly.

**Gate**: `cargo check -p semio-framework-os-kernel` 0 errors; `cargo test -p semio-s-plugin-stdio --lib` still 332/0 after S-3 (unchanged from post-S-2, since stdio never used `ArtifactEngine`).

## Step C — S-6, extended to pdf per W0's finding

**gif** (`✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs`, gif module's shim block): `schema`/`engine`/`io` shims flipped from `standards::v87a::…` to `standards::v89a::…`; `engine` shim gained a hand-written `register()` that calls both `v87a::engine::register()` and `v89a::engine::register()` (a pure glob re-export can't do this — two `register` fns of the same name would collide; a local fn definition shadows the glob-imported one, which is what makes this work). Root-level `🏗️builder/component.rs` and `🧐️analyzer/component.rs` (the artifact-level facades) repointed from `standards::v87a::{builder,analyzer}` to `standards::v89a::{builder,analyzer}` (required — these delegate to a raw builder/analyzer whose associated `Snapshot`/`Mutation`/`Diff` types must match the now-89a-shaped root re-exports, or the crate doesn't type-check). Root composer already unioned both standards' entries (no change needed).

**pdf** (twin fix, same file): `schema`/`engine`/`io` shims flipped from `standards::v1_4::…` to `standards::v1_7::…`; same dual-register pattern for `engine::register()`. Root `🏗️builder`/`🧐️analyzer` repointed from `v1_4` to `v1_7` (including the `Dialect`'s `standard` tag, `"1.4"` → `"1.7"`). Root composer already unioned both (no change needed).

**Secondary defect surfaced and fixed**: 87a's and 1.4's *own* internal code (engine/schema/mutations/io-leaf files inside `standards::v87a::**` and `standards::v1_4::**`) had been written importing types through the shared root alias (`crate::artifacts::gif::GifSnapshot`, `crate::artifacts::gif::schema::…`, etc.) on the assumption that root always pointed at themselves — true before this fix, false after. This affected ~30 files across the two standards' subtrees (engine, schema/component.rs, mutations, the `set-snapshot` mutation triad's `mutation`/`inverse` leaves, io import/export deserializer/serializer leaves, the standard-level builder facade, and gif 89a's `migrate_87a_to_89a` function which explicitly needs *both* standards' own types by name). All redirected to their own standard-local paths (`standards::v87a::subsets::any::schema::…` / `standards::v1_4::subsets::any::schema::…`); a couple of round-trip fixes were needed after the first pass (unqualified `schema::{GifDiff, GifMutation, GifSnapshot}` needed the `diff::`/`mutations::`/`snapshot::` submodule prefixes since those types live one level deeper than `GifArtifact`, which is the only type schema's own `component.rs` re-exports directly).

**Gate**: `cargo test -p semio-s-plugin-stdio --lib "artifacts::gif"` → 26 passed, 0 failed. `cargo test -p semio-s-plugin-stdio --lib "artifacts::pdf"` → 38 passed, 0 failed. Full suite still 332/0 afterward.

## Step D — S-4: NOT attempted

Explicitly optional per the brief. Given the size already covered in this wave (S-2's 252-site sweep + S-3's 85-site deletion + S-6's cascading ~30-file secondary fix), S-4 (`ArtifactSchemaDescriptor` gains a 4th `mutations: FacetLeaves` field, 391 `include_str!` call sites) was left entirely untouched rather than attempted partially, per the brief's explicit instruction ("If you do NOT have capacity, do not attempt a partial/half-done version — leave it entirely for S2"). Confirmed via W0 recon (not re-verified independently this session) that the struct is still exactly 3 fields.

## Step E — S-7: confirmed, no code change

`grep -rln "CollectionDiff\|CollectionMutation\|Patchable\|Identified" ✏️s/🔌️plugins/🗄️stdio` → zero matches. No stdio artifact references vcs collection machinery; nothing to fix. (The policy-ban script.ts rule enforcing this is S2 scope, per the plan.)

## Hard exit gate — final state

```
cargo check -p semio-framework-os-kernel   → 0 errors
cargo check -p semio-s-plugin-stdio        → 0 errors
cargo test -p semio-s-plugin-stdio --lib   → 332 passed, 0 failed, 0 ignored
```

**332 vs. W0's 318 baseline**: the +14 tests are the `pdf/standards/v1_7/subsets/a2b/*` suite (analyzer/builder/composer tests for the PDF/A-2b subset), which did not exist in this crate at session start and landed mid-session from a concurrent session's work (confirmed: this subset's builder file already contained the post-S-2 tuple-return `mutate` signature *and* a docstring explicitly citing "`ArtifactBuilder::mutate`'s `-> (Self, Self::Diff)` signature" — i.e. another session adapted its own new code to my in-progress signature flip in real time). 0 failures is the invariant the gate cares about, and it holds throughout.

`cargo check --workspace --keep-going`: zero errors anywhere in the workspace are attributable to this wave's changes (grepped the full log for `mutate`/`Self::Diff`/`ArtifactBuilder`/`ArtifactEngine` — no hits beyond incidental `use` lines in unrelated error contexts). All remaining errors (paths in `s1-check-workspace-final.txt`, this folder) are pre-existing/concurrent churn from other sessions, unrelated to any file this wave touched:

- `semio-framework-os-kernel-db`, and 10 plugin crates (`vcs`, `sourcing`, `sequence`, `reasoning-mindmap`, `norm`, `mathematical`, `imperative`, `forms`, `flow`, `dag`, `block`, `architect`) — a repo-wide missing `📄️document/component.rs` panel-module file (hard I/O error, `#[path=...] mod` pointing at a file that doesn't exist on disk — some other session is mid-rename/mid-delete of a "document" panel module).
- `semio-framework-os` — `AppDefinition`/`OsAppRegistration` gaining/losing a `label`/`document` field mid-refactor (duplicate-field and missing-field errors), plus an unrelated `OsMediaExportResult::from_format_kind_bytes` rename.
- `semio-s-plugin-fem`, `semio-s-plugin-energy` — `CsvSnapshot` missing a `has_header` field at call sites outside csv's own tree (W0 explicitly flagged csv as mid-edit at recon time; this is that same in-flight work, not yet propagated to its own downstream serializer call sites).
- `semio-compose-rs` — 22 unrelated errors (lifetime-elision style errors, `label`/`document` field errors matching the host refactor above) in a completely separate crate tree (`./compose/**`), transitively broken by the `semio-framework-os-kernel-db` failure.

**Packages verified directly** (via `-p`, not just swept up in `--workspace`): `semio-framework-os-kernel`, `semio-s-plugin-stdio`, and — via a batched `cargo check --keep-going` across all 32 crates that had an `ArtifactEngine` impl removed — `semio-s-plugin-{animate,architect,block,cad,dag,demonstrator,draw,energy,fem,flow,forms,gis,imperative,layout,lowpoly,mathematical,norm,note,playbook,procedural,process,puzzle,raster,reasoning-mindmap,remodel,sequence,shooting,sourcing,space,trinity,vcs,writer}`. Of these, `architect,block,dag,energy,fem,flow,forms,imperative,mathematical,norm,reasoning-mindmap,sequence,sourcing,vcs` (14) could not be fully verified end-to-end because they fail before reaching my own edits, for the pre-existing/unrelated reasons listed above — I did not attempt to fix those (out of scope, another session's in-progress work per the repo rules). The other 18 compile clean.

**Not independently re-verified this session** (relied on W0's own direct reads, unchanged by anything in this wave): S-5's target (`register_document_codec`) and S-4's target (`ArtifactSchemaDescriptor`) field count.

## Handoff to S2

- **S-4 not started at all** — no partial state, do it first as the brief anticipated. 391 call sites, all still the 3-field pattern.
- **glue.rs**: touched in this wave only for the gif+pdf S-6 shim/register blocks (explicitly permitted for S1 as the spine agent). From S2 onward it reverts to closer-only ownership per the plan.
- **S-8 policies and mutation-triad pre-mounts**: untouched, as planned, S2 scope.
- If S2 (or any later wave) needs to add a *new* file anywhere under `standards::v87a::**` or `standards::v1_4::**` for gif/pdf, remember the S-6 lesson: don't reach for the shared root `crate::artifacts::{gif,pdf}::` alias from inside a non-canonical standard's own subtree — it now resolves to the *other* (canonical) standard, not itself.
- Full touched-file audit trail: `s1-artifacts/s1-artifactbuilder-252-files.txt` (S-2 sweep) and `s1-artifacts/s1-artifactengine-85-files.txt` (S-3 deletions) in this folder; raw `cargo check` logs also in this folder (`s1-check-1.txt`, `s1-check-workspace-1.txt`, `s1-check-workspace-2.txt`, `s1-check-workspace-keepgoing.txt`, `s1-check-engine-crates.txt`, `s1-check-workspace-final.txt`).
