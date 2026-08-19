# terra / descriptor-prep — 7 missing plugin descriptors

Packet: `descriptor-prep`, executor `terra`, ticket `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`.

## Scope recap

Missing descriptors: **demonstrator, fem, playbook, trinity, stdio, puzzle, block**. Owned/writable:
`✏️s/🔌️plugins/🔱️trinity/**`, `✏️s/🔌️plugins/🧱️block/**`. Everything else read-only. Emitting the
descriptors themselves needs a `wasm32-wasip2` build gated behind the SDK — out of reach this packet;
this is the pre-wasm half only.

## 1. `🔱️trinity` — re-verified intact

Read every byte of `✏️s/🔌️plugins/🔱️trinity/🦀️component.rs` and
`✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🦀️component.rs` fresh from disk (not from the prior
report — reports can drift, disk is ground truth). Both match `📓️terra-fleet-trinity-recipe-report.md`
exactly: `.declare_artifact(crate::artifacts::jack::artifact())` /
`.declare_artifact(crate::artifacts::rewrite::artifact())`, `pub async fn artifact()` on each artifact
root, standard-root + subset-root files present, `io_declaration()` deviation with `entries: &[]`
documented. Untouched this packet. **One correction to my own prior report**: `artifact()` and
`standard()`/`subset()` are `pub async fn`, not the plain `pub fn` my own earlier report text claimed —
disk is authoritative; I built block's migration off the disk shape, not the report's prose.

A full `cargo check -p semio-s-plugin-trinity --lib` re-run was not attempted separately from block's —
see §3, the whole shared dependency graph is unstable right now and would hit the identical wall.

## 2. `🧱️block` — migrated to `.declare_artifact(...)`

**Block has 3 owned artifacts** (block2d, block3d, block5d) — `📓️luna-claims-audit.md` recorded it as
"1 (block)", unverified per its own §4 admission. Corrected by direct inspection.

Applied the exact 9-step `fleet-trinity-recipe` per artifact, with one deliberate difference from
trinity: **`🚪️io/` is NOT excluded from this packet's ownership** (trinity's exclusion was a live-peer
boundary that does not apply to block), so I read block's own `🚪️io/🦀️component.rs` files in full. They
are on the same OLD `ArtifactComposition`/`ComposerEntry` channel trinity's were — converting them to
the new typed `Serializer<S>`/`Deserializer<S>` channel is real, non-trivial per-format work (12 typed
impls per artifact × 3 artifacts = 36), not a byproduct of the capability-claim migration this packet
targets. I made the same documented-gap choice trinity's packet made (`entries: &[]`, real native codec)
rather than attempt 36 untested hand-authored format conversions with no way to `cargo test` them this
session (single shared build lock, see §3) — recommended as dedicated follow-up, not a lease-request
(nothing blocks it; it's a scope decision, not a boundary conflict).

### Files changed

- `✏️s/🔌️plugins/🧱️block/🦀️component.rs` — plugin root: 3× `.artifact(x::declaration())` +
  3× `.editor::<>()` + 3× `.viewer::<>()` → 3× `.declare_artifact(x::artifact())`, keeping
  `.editor_mutation_roster()`/`.viewer_mutation_roster()`/`.activation()`/`.execution()`/`.requests()`
  untouched.
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/{◻2d,🧊️3d,🖐️5d}/🦀️component.rs` — each: `declaration()` deleted,
  `pub async fn artifact()` added (kind `s.block.block2d`/`s.block.block3d`/`s.block.block5d`),
  `pilot_languages()` made `pub` (needed by the new subset-root file).
- NEW `✏️s/🔌️plugins/🧱️block/🗿️artifacts/{◻2d,🧊️3d,🖐️5d}/🏅️standards/🔖️1/🦀️component.rs` — standard
  roots. `extensions` real (carried from each artifact's own codec-capability row: `block2d`/`block3d`/
  `block5d`); `mimes` synthesized `application/vnd.semio.block<n>d+json`, documented as such (no real
  mime claim existed pre-migration, matching note/draw/trinity's own precedent).
- NEW `✏️s/🔌️plugins/🧱️block/🗿️artifacts/{◻2d,🧊️3d,🖐️5d}/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️component.rs`
  — subset roots. Real schema/inference descriptors, real editor/viewer surfaces, real native codec
  (reusing each artifact's own `pilot_languages()`), `entries: &[]` (documented gap, see above). Examples:
  block2d's two (`hexagonal-cut-concrete-forest-{left,right}`) and block3d/5d's two each
  (`hexagonal-cut-concrete-forest-left`, `nakagin-capsule`) are mounted at the **plugin-root**
  `crate::examples::art_<dim>_*` module (block's `📦️glue.rs` mounts examples flat at the crate root, NOT
  per-artifact like trinity's `crate::artifacts::jack::examples::demo` shim — verified from `📦️glue.rs`
  directly, not assumed from the trinity template).
- `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/📦️glue.rs` — mounted both new file types × 3 artifacts
  (first line inside each `pub mod v1 { }` and `pub mod any { }`, exact `#[path="../../🗿️artifacts/...`
  prefix trinity's recipe documents).
- Stale doc-comment fixes (2 files) that named the deleted `block2d::declaration()`, updated to name
  `block2d::artifact()`.

### What did NOT generalize from trinity (found by reading, not assumed)

- **3 artifacts, not 1.** Luna's audit under-counted block; verify-don't-inherit paid off immediately.
- **Examples live at the plugin-root `crate::examples::` module**, not a per-artifact
  `crate::artifacts::<x>::examples` shim — block's own `📦️glue.rs` mounts them flatly (2 artifact-level
  examples per artifact, plus a separate `app_<dim>_demo_session` editor-level example not part of
  `SubsetDeclaration.examples`, mirroring how note's own `app_note_demo_session` is wired elsewhere, not
  in `subset()`).
- **block2d's schema-descriptor call site used the deep path** (`standards::v1::subsets::any::schema::…`)
  while block3d/5d's used the shim (`schema::…`) — both resolve to the same thing (the shim re-exports
  the deep path), confirmed by reading `📦️glue.rs`'s shim block for all three, not just copying one.

## 3. Build verification — BLOCKED by concurrent unrelated churn, not by this packet's code

`CARGO_TARGET_DIR=<scratchpad>/target-block cargo check -p semio-s-plugin-block --lib`, run **7 times**
across this session. Every single failure was in a shared framework crate **outside my owned paths**,
confirmed live (not stale) via `git diff HEAD --stat` / `git log --date=iso`:

| run | crate that failed | file | errors | evidence it's someone else's live edit |
|---|---|---|---|---|
| 1 | `semio-framework-os-kernel` | `🏪️store/🦀️component.rs` | 3 | `git diff HEAD --stat`: 998 lines changed uncommitted; re-read the exact broken line seconds later and it had already been fixed by another session |
| 2 | `semio-framework` | `🛂️manifest/🦀️component.rs` | 204 | `#[serde(default = "async_fn")]` — an E1 violation (external `Deserialize` trait fed an async fn) from a codemod mid-sweep elsewhere |
| 3 | `semio-framework-number` | `🔢️number/🦀️component.rs` | 620 | `Add`/`Neg`/`Mul`/`Iterator` external-trait impls made async — same E1 shape, no diff vs HEAD (landed-but-broken, or a live session about to fix it) |
| 4 | `semio-framework-graph` | `🕸️graph`→`🛂️manifest/🦀️component.rs` | 578 | same manifest.rs family |
| 5 | `semio-framework` | `🛂️manifest/🦀️component.rs` | 202 (was 204) | count dropped 2 between runs — a sibling session incrementally fixing it live |
| 6 | `semio-framework-number` | `🔢️number/🦀️component.rs` | 620 (same as run 3) | oscillating back |
| 7 | `semio-framework-number` | `🔢️number/🦀️component.rs` | 620 (identical to run 6) | no `git diff HEAD` on this file either run — either a landed-but-broken commit or a live session that has not touched it between runs 6 and 7 |

**None of my crate's own files (`✏️s/🔌️plugins/🧱️block/**`, `🔱️trinity/**`) ever appeared as an error
source in any of the 7 runs** — `semio-s-plugin-block` never got far enough to be type-checked at all,
because its transitive dependencies (`semio_framework_plugin` → `semio-framework`/`-graph`/`-number`/
`-os-kernel`) failed first, every time, in a different crate. This matches the program state I was
briefed with almost exactly: "Seven sibling agents are clearing the last two crates gating the guest
SDK" — the whole framework dependency graph is being asyncified live, several crates at once, right now.

Per the ticket's own "Concurrent Cargo Workspace Churn" rule (check shared files before assuming it's
your bug; poll rather than chase) I stopped after 7 attempts rather than loop indefinitely. **I am not
claiming block compiles** — only that: (a) it never once caused a compile error itself across 7 attempts
that each got substantially further into the dependency graph before failing elsewhere, (b) every new/
edited file balances braces and parens (`python3` brace/paren count, not a name-keyed tool — R10), and
(c) every identifier I referenced (`pub async fn artifact()`, `pilot_languages()`, schema/inference
descriptor fns, editor/viewer create fns, `Block<n>dSnapshot`/`Block<n>dMutation`/`BLOCK<n>D_DIALECT`/
`BLOCK_<n>D_SCHEMA`) was `grep`-confirmed present with the exact name and signature I used, not assumed
from the trinity template. This is real diligence, not a compile — a fresh `cargo check` once the shared
crates stabilize is still owed before this can be called done.

## 4. The other five — read-only migration plans

Verified rather than inherited: luna's audit (§4) explicitly flagged fem/layout/playbook/trinity/block
as "not traced in detail, assumed same root cause." I traced fem/playbook/puzzle/demonstrator/stdio
myself this pass; findings below **correct or extend** luna's table where they differ.

| plugin | owned artifacts | shape | cause (verified) | exact change needed | data or mechanism |
|---|---|---|---|---|---|
| **fem** | 2 (fem2d, fem3d) | identical to block/trinity: `.artifact(declaration())` + `.editor::<>()`/`.viewer::<>()` per artifact, no standard-root/subset-root files yet | capability-claim exact-equality check (same as block/trinity) | apply the 9-step recipe verbatim, twice (fem2d, fem3d) — no artifact-specific deviation found in the root file read | **mechanism-shaped but purely mechanical** — same recipe as block, ~2x the file count of trinity |
| **playbook** | 1 (playbook) | identical shape, single artifact, `.editor()`/`.viewer()` for one dialect | same | apply the recipe once | **mechanical**, smallest of the 7 |
| **puzzle** | 3 (puzzle2d, puzzle3d, puzzle5d) | identical shape to block (3 artifacts) | same | apply the recipe 3×. **Trap found**: the plugin root's own doc comment claims `.setup()` "still survives for the OS media-host bridges" (`register_media_io`/`register_mesh_io`), but the actual `Plugin::builder(...)` chain in the file has **no `.setup(...)` call at all** — and puzzle5d's own editor file (`🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:2057-2059`) says outright those registrars were "never rewired to a real `.setup()` caller... and was deleted as dead code." The doc comments on puzzle2d's/puzzle3d's artifact roots are **stale** — verify whether `register_media_io`/`register_mesh_io` are called from anywhere at all (a `#[ctor]`, a test-only path, or genuinely dead) before assuming the migration needs to preserve a call site that may not exist | **mechanical + one doc/dead-code hazard to resolve first** |
| **demonstrator** | 1 owned (`playground`) + 6 **foreign** plugin surfaces (cad/gis/procedural/process/puzzle3d/sourcing, registered via bare `.editor()`/`.viewer()`, not `.artifact()`) | plugin root lives at `🛂️manifest/🎪️demonstrator/🦀️component.rs`, not the usual `🦀️component.rs` at the plugin's own top level | `playground`'s `.artifact(declaration())` is the only capability-claim-checked call; the 6 foreign `.editor()`/`.viewer()` calls never touch `.artifact()` at all and are unrelated to this bug — **must stay untouched** | apply the recipe ONCE, to `playground` only. **Open question, not yet resolved**: playground's own `declaration()` doc comment says "Playground owns no `ArtifactApp`... so there is no `.document_codec()` call" — every other plugin's `subset()`/`io_declaration()` template assumes a real `store::ArtifactCodec::of::<Snapshot,Mutation>(...)`; whoever migrates playground must first determine what (if anything) `NativeCodecs.codec` should hold when there was never a document codec registered on the old channel — read how `crate::editor::playground::PlaygroundEditor`'s own document type is wired before assuming the same codec pattern applies | **mechanical for 5/6 of the file; one genuine open design question for the codec field** |
| **stdio** | 36 (one per file format) | **NOT the same shape as the other 6** — confirmed by direct read, not luna's "unknown/unverified" guess. A JSON-schema-driven registry (`📇️registry/🦀️component.rs`): 36 `artifact-definition.json` sources, a `Source`/`build()` pipeline, and an `ArtifactAssembly` enum with only `Definition`/`Runtime` variants (no slot for the new declaration channel). The plugin root loops `for assembly in crate::registry::artifact_assemblies()? { builder = match assembly { Definition(d) => builder.artifact_definition(d), Runtime(d) => builder.artifact(d) } }`, THEN separately makes ~90 explicit `.editor::<>()`/`.viewer::<>()` calls in the plugin root itself (not derived from any per-format `subset()`) | same claim-check bug, but the fix shape is structural, not per-artifact boilerplate | (1) add a third `ArtifactAssembly` variant (or a parallel path) carrying `crate::app::declarations::ArtifactDeclaration`; (2) migrate 36 artifacts' `assembly()` factory fns to the new `artifact()` shape — some own MULTIPLE standards/subsets (docx/pptx/xlsx each have 3 subsets: any/strict/transitional; step/ifc likely similarly multi-subset — not yet enumerated per-format); (3) fold the ~90 plugin-root `.editor()`/`.viewer()` calls into each format's own `subset()`, the same way block/trinity did. **Concrete head start found**: `txt` and `binary` (2 of 36) **already have** `pub async fn artifact()` + full standard-root/subset-root trees on disk (`🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/…`, `🗿️artifacts/💾️binary/🏅️standards/🔖️raw/…`) — a prior, unfinished attempt — but neither is wired into `artifact_factories()`/`plugin()` yet (zero `declare_artifact` calls anywhere in the `🗄️stdio` tree, confirmed by grep). Whoever picks this up should finish wiring those 2 first as the proof-of-concept before scaling to the other 34 | **mechanism (new enum variant, registry-loop change) + large-volume data (36 artifacts, ~90 surface registrations)** — the largest and most architecturally distinct of the 7, exactly matching luna's own "requires separate investigation" flag, now with a concrete path traced |

## 5. Lease-request

None. Both trinity and block's `🚪️io/` are inside this packet's own ownership; no shared/registrar file
was touched. The recommended stdio/demonstrator/puzzle follow-up work above is scoped to plugins outside
this packet's ownership, so it is future-packet work, not a lease.

## Files touched (for `ticket_close`, not run by this packet)

- Modified: `✏️s/🔌️plugins/🧱️block/🦀️component.rs`
- Modified: `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻2d/🦀️component.rs`
- Modified: `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🦀️component.rs`
- Modified: `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🦀️component.rs`
- Modified: `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/📦️glue.rs`
- Modified: `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs` (doc comment only)
- Modified: `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs` (doc comment only)
- Created: `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻2d/🏅️standards/🔖️1/🦀️component.rs`
- Created: `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️component.rs`
- Created: `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🦀️component.rs`
- Created: `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️component.rs`
- Created: `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🦀️component.rs`
- Created: `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️component.rs`
- NOT touched: `✏️s/🔌️plugins/🔱️trinity/**` (read-only re-verification), `demonstrator`/`fem`/`playbook`/
  `puzzle`/`stdio` (read-only per scope).
- Scratch: `block-check{1..7}.txt` and the `target-block` cargo target dir were written to the session
  scratchpad (`/private/tmp/claude-501/.../scratchpad/`), not the ticket folder, per the cargo-target-dir
  rule — nothing left in the ticket folder itself.
