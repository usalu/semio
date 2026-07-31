# CW3 — Kernel cut-over: framework/core extraction + vcs slimming

Single serial agent, critical section. You are the sole editor this wave of: `vcs/rs/lib.rs`,
`vcs/rs/Cargo.toml`, `framework/core/rs/lib.rs`, `framework/core/rs/Cargo.toml`,
`dsl/derive/rs/lib.rs`, `framework/plugin/rs/lib.rs`, `framework/hlc/` (deletion). Re-read every
one of these fully and fresh before editing — other sessions may have touched them since this was
written. All 12 `protocol` crates are complete, built, and tested (CW2 done): read their actual
`lib.rs` files as ground truth for exact type/trait names and signatures, do not guess from this
document alone.

Read first: `/Users/ueli/.claude/plans/introduce-a-new-technology-cuddly-rabbit.md` in full (only
148 lines) for complete context — Part 1 "vcs slimming" and Part 3 "framework/core cleanup" are
your primary spec; this file restates and sharpens them into an execution checklist.

**Goal for end of wave: `cargo build --workspace` succeeds.** This is the hard bar — CW4 (db
family) and everything after depends on a green tree.

## 1. `framework/core/rs/lib.rs` — extraction

Move OUT (delete from framework/core, now live in `protocol_core`/`protocol_causal` — read
`/Users/ueli/Documents/semio/protocol/core/rs/lib.rs` and
`/Users/ueli/Documents/semio/protocol/causal/rs/lib.rs` to confirm exact names before deleting
anything):
- `HybridLogicalTimestamp` (region `🔖️HybridLogicalTimestamp`) — now `protocol_core::HybridLogicalTimestamp`.
- `PayloadHash`, `OperationEnvelope`, `OpDag` (+ its tests) (region `🔖️Sync`) — now
  `protocol_causal::{OperationEnvelope, OpDag}` and `protocol_core::PayloadHash`.
- `UndoPolicy`, `MergeStrategyKind` (region `🔖️Policies`/near `DocumentKind`) — now
  `protocol_core::{UndoPolicy, MergeStrategyKind}`.
- The operation-flavored id newtypes (`OperationId`, `ActorId`, `DocumentId`, `DocumentVersion`,
  `SchemaId`, and any others in the `🔖️Identifiers` region that moved — cross-check against
  `protocol_core`'s actual `Identifiers` region for the exact list; leave any id type that did NOT
  move, e.g. pure UI/window/plugin ids, untouched in framework/core).
- The `🔖️HubProtocol` region entirely (`HubClientFrame`, `HubServerFrame`, `PresencePeer`, and
  presence types) — superseded by `protocol_wire` (read
  `/Users/ueli/Documents/semio/protocol/wire/rs/lib.rs`); this region is simply deleted, nothing
  in framework/core replaces it (wire consumers land in CW5, not this wave — if deleting this
  region breaks a framework/core-internal caller, that caller needs a narrow `protocol_wire` dep
  added now rather than leaving dead code, but do not implement CW5's full wire integration here).

framework/core gains a narrow `protocol` (or `protocol_core`/`protocol_causal` directly, whichever
is more precise per what it actually still needs) dependency in `framework/core/rs/Cargo.toml` for
the dual-use ids it retains references to (e.g. if `ActionInvocation`/`CommandInvocation` or
similar kernel types still reference `OperationId`, they now reference
`protocol_core::OperationId`).

**Delete `framework/hlc/rs` outright** (crate + its `Cargo.toml` entry in root workspace
`members`) — the plan states zero dependents, but re-verify with a grep for
`semio-framework-hlc`/`framework_hlc`/`framework/hlc` across the whole repo before deleting; if you
find a live dependent, stop and report rather than deleting.

## 2. `vcs/rs/lib.rs` — slimming

**Stays in vcs** (do not move, do not touch beyond what's needed to compile against the new
protocol deps): `DocumentVcs*`, `Change`/`Checkpoint`/`Alternative`, `Author`, materialize-by-
replay (`materialize_document_projection`), history columns (`build_history_columns`),
`DocumentVcsStore`, `Backbone`/`BackboneMessage`/`PortBackbone`/etc., `BlobStore`, the Studio layer
(`StudioMember`, `StudioVcsHost`, etc.), `CodecRegistry`, `DocumentDsl`/`DocumentPack`/`pack_rt`,
`FolderTextStorage`/`FolderSqliteStorage`, `test_support`.

**Moves out of vcs** (delete the original definitions from vcs, now live in `protocol_command`/
`protocol_causal`/`protocol_crdt` — confirm exact names against those crates' actual `lib.rs`):
`Operation<P>`/`OperationDiff<P>` traits, `OpText`, `OperationMeta`, `Edit<Operation>` (careful:
`vcs::Edit` is used pervasively by `DocumentVcs`/`Change`/etc. that STAY in vcs — check whether
`protocol_command::Edit` is meant to fully replace `vcs::Edit` as a type alias/re-export, or
whether vcs keeps its own `Edit` struct that now embeds/references protocol's `Operation`/`OpText`
bound differently; read `protocol_command`'s actual `Edit` definition and the vcs slimming table's
exact wording — "Edit" is listed under "moves to protocol", so the type itself relocates and vcs's
`DocumentVcs<P, Operation>`/`Edit` usage becomes `protocol::Edit<Operation>` via re-export), the
`Identified`/`Patchable`/`CollectionOperation` collection kit, `merge_concurrent_diffs`,
`operation_envelope_from_edit`.

**Add temporary compatibility shims** so the rest of the tree (not yet updated — that's CW7) keeps
compiling: `pub use protocol::{Operation, OperationDiff, OpText, OperationMeta, Edit, /* collection
kit names */, merge_concurrent_diffs, operation_envelope_from_edit, ...};` near the top of
`vcs/rs/lib.rs`, clearly marked with a comment `// 🚧️ TEMPORARY shim for the CW3 kernel cut-over —
deleted at CW8 once every ~40 dependent crate imports protocol:: directly.` Any code inside vcs
itself that used the bare (unqualified, since-it-was-local) names continues to work unchanged
through the shim's local scope; anything referencing `vcs::Operation` etc. from OUTSIDE vcs also
keeps working through the re-export.

`.ops` text grammar: `vcs`'s `OpsHeaderLine`/`print_ops_log`/`parse_document_text`'s structural-
line handling should now delegate to `protocol_history`'s equivalent grammar functions rather than
maintaining a second copy — read `protocol_history`'s `parse_ops_text`/`print_ops_text` (or
whatever the closest equivalent is) and wire vcs's existing `print_document_text`/
`parse_document_text` to call through, keeping vcs's public API (`DocumentTextFiles`, the function
names) unchanged so no downstream crate needs to change. If a clean delegation isn't achievable in
this wave without a larger refactor, it's acceptable to leave vcs's own grammar code as-is for now
and note this as a follow-up — don't let this specific item block a green tree; prioritize the
Operation/OpText/OpDag moves and the dsl_derive flip below, which are load-bearing for everything
downstream.

**`DocumentVcsStore`**: its `dag` field re-types from whatever local `OpDag`-like type it used to
`protocol::OpDag` (or `protocol_causal::OpDag` directly). `vcs` drops its `semio-framework-core`
Cargo dependency entirely once nothing in vcs references framework-core types anymore (verify with
a grep after your edits, then remove the dep line).

**Fixes** (small, scoped): content-addressed checkpoint ids — `Checkpoint.id` generation changes
from the counter-string scheme to `format!("ck-{}", hex16(blake3(parent_id || ordered_change_
content_hashes || message || authors || timestamp)))` using `pack_core::ContentHash`/blake3 (vcs
already depends on `pack`, which re-exports `ContentHash`). Add `merge_base(envelope, a, b)` +
supporting ancestor-traversal helpers beside `build_history_columns`. These are genuinely new
logic, not a move — implement them cleanly with inline tests.

## 3. `dsl/derive/rs/lib.rs` — the OpText flip

Find the `DslOps` derive macro's codegen (region `🔖️DslOps`, per the repo's dsl_derive
conventions) and change the generated `impl ::vcs::OpText for ...` to `impl ::protocol::OpText for
...` (verify the exact facade re-export name — it's `protocol::OpText`, re-exported from
`protocol_command` by the `protocol` facade; confirm by reading `protocol/rs/lib.rs`). This is a
ONE-LINE-ISH change in the macro's `quote!{}` output, but it changes what every one of the ~40
`#[derive(dsl::DslOps)]` crates' generated code implements. Because of vcs's temporary shim
(`pub use protocol::{OpText, ...}`), code that still says `use vcs::OpText` or references
`vcs::OpText` as a bound continues to resolve correctly — the flip should NOT itself break any
downstream crate's compilation this wave, since both `vcs::OpText` (via shim re-export) and
`protocol::OpText` now name the identical trait. Verify this reasoning empirically: after the flip,
build a handful of representative `DslOps`-deriving crates (e.g. `puzzle_2d`, `draw`, `cad`) and
confirm they still compile without any changes to their own source.

Every crate with `#[derive(dsl::DslOps)]` also needs `protocol` (or at minimum `protocol_command`)
reachable as a transitive/direct dependency for the generated `impl ::protocol::OpText` to resolve
— since `vcs` now re-exports `protocol::OpText` and virtually every `DslOps`-deriving crate already
depends on `vcs`, this likely resolves for free through the shim without adding new Cargo.toml
deps anywhere. Confirm this is actually true by building; if some crate depends on `dsl` for the
derive but NOT on `vcs`, it will need a new dep — if you find such crates, add the minimal
`protocol` path dep to their `Cargo.toml` (this counts as part of "the ~40-crate dep/import sweep"
mentioned in the wave plan, scoped to only what's needed for a green build, not a full migration —
full per-file import-path surgery to `protocol::` is CW7's job, not yours).

## 4. `framework/plugin/rs/lib.rs` — trait bound update

`DocumentApp::Operation`'s trait bound changes from (whatever it currently references from vcs,
e.g. `vcs::Operation<Self::Projection> + vcs::OpText`) to `protocol::Operation<Self::Projection> +
protocol::OpText` (or continues to type-check unchanged through the vcs shim — check both; prefer
updating the bound to reference `protocol::` directly here since `framework/plugin` is a small,
contained, single file you fully control this wave, unlike the ~40 app crates). `VcsDocumentApp`
keeps its name and behavior — only the trait bound's source module changes.

## 5. Verify tree-green

After every edit above: `cd /Users/ueli/Documents/semio && cargo build --workspace 2>&1 | tail
-300`. Fix compilation errors as they surface, prioritizing in this order: vcs itself → framework/
core → framework/plugin → dsl/derive → then whatever ripples out from those four. If a downstream
app crate fails to compile because of a genuinely missing shim (a moved type the vcs re-export list
doesn't cover), add it to the shim list in vcs rather than editing the app crate — app-crate edits
are explicitly out of scope for this wave (CW7). Keep going until `cargo build --workspace`
succeeds with zero errors (warnings are fine, note significant ones).

Run `cargo test -p vcs -p semio-framework-core -p semio-framework-plugin` and confirm existing
tests still pass (adjust test code inside those three crates only if the move broke a test's
import path — that's in-scope, it's the same file you're already editing).

## Report back

Write `.repo/🎫️/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/cw3-report.txt`
covering: exact diff summary per file, the full shim list left in vcs (needed by CW8 to remove
them later), confirmation `cargo build --workspace` is green with the exact command output tail,
test results for vcs/framework-core/framework-plugin, and anything you deferred (e.g. the `.ops`
grammar delegation, if you had to skip it) with a clear note for whoever picks it up next. If you
hit a genuine blocker you can't resolve within this wave's scope, report it clearly rather than
forcing a broken workaround — a slightly incomplete but honestly-reported wave is better than a
green build achieved by silently leaving something broken.
