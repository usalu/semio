# W3 — Flow, Config, Compose — Report

Lane R6. Three lease areas, all converted per `📋️contract-freeze.md` fan-out recipe.

## 1. `✏️s/🔌️plugins/🌊️flow/**` — 9 leaves + facet tests

All 9 `🔺️diff` leaves under `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/`
converted to `pub fn diff(..) -> protocol::MutationOutcome<FlowDiff>`, with each sibling
`🦠️mutation/🦀️component.rs`'s `fn diff` return-type updated to delegate:

| Leaf | Detection added |
|---|---|
| `➕️create-widget` | Fatal `mutation.duplicate-id` on existing widget id |
| `🗑️delete-widget` | Error `mutation.target-missing` on absent id; Info `mutation.cascade` when severed synapses |
| `🔗️connect-widgets` | Fatal `mutation.duplicate-id` (dup synapse id); Error `mutation.target-missing` (missing `from`/`to` widget); Warning `mutation.no-op` (identical parallel synapse) |
| `✂️disconnect-widgets` | Error `mutation.target-missing` on absent synapse id |
| `📍️move-widgets` | Error `mutation.target-missing` (any entry targeting a missing widget); Fatal `mutation.invariant` (non-finite x/y); Warning `mutation.no-op` (all entries already match) |
| `🔁️replace-widget` | Error `mutation.target-missing`; Warning `mutation.no-op` (identical value) |
| `🔀️🪟️reorder-widgets` | Error `mutation.target-missing`; Warning `mutation.no-op` (already at that index) |
| `🔀️reorder-synapses` | Same shape as reorder-widgets, over synapses |
| `🔄️update-synapse-endpoints` | Error `mutation.target-missing` (synapse or either endpoint widget absent); Warning `mutation.no-op` (identical endpoints) |

No `validate` override existed on any of the 9 leaves (already stripped at the W0 barrier). One WAS
found and deleted: `👯️duplicate-widget/🦠️mutation/🦀️component.rs`'s `CompositeMutationKind::validate`
override — the trait no longer declares that method, so the override was a dead impl item (would not
compile). `plan.rs`'s own internal `precondition()` helper (called from `plan()` itself, unrelated to
the deleted trait method) was left untouched, matching the recipe's "composite kinds: nothing beyond
the derive."

Call sites fixed:
- `🧬️mutations/🦀️component.rs::apply_flow_mutation` — `.diff(snapshot).apply(snapshot)` →
  `.diff(snapshot).diff().apply(snapshot)?`.
- `👯️duplicate-widget/🧩️plan/🦀️component.rs` tests — 4 call sites (`fold_plan_diff(..).apply(..)` /
  `op.diff(..).apply(..)`) updated to insert `.diff()` before `.apply()`.

`✏️s/🔌️plugins/🌊️flow/🌿️vcs/🦀️component.rs` (the hand-written `FlowMutation` impl named in the
brief) **does not exist** — confirmed via directory listing (`✏️s/🔌️plugins/🌊️flow` has no `🌿️vcs`
subtree). Nothing to convert there.

Facet `🧪️Tests`: added a `🔖️OutcomeLaws` region to `🧬️mutations/🦀️component.rs`'s existing
`#[cfg(test)] mod tests` (previously empty) — 19 tests, one representative
`assert_missing_target_is_error`/`assert_fatal_never_applies`/no-op/cascade check per verb family
listed in the table above.

## 2. `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/**` — 2 leaves

Both `🔺️diff` leaves (`📌️set-default-app`, `🧹clear-default-app`) were previously only
**mechanically wrapped** at the `🦠️mutation/🦀️component.rs` level (`MutationOutcome::new(super::diff::diff(..))`,
diff.rs itself still returning a bare `OpeningPreferences`) — the "no `Error`/`Warning`/`Fatal`
messages added here yet" TODO comment in both `🦠️mutation` files. Moved the real detection into the
`🔺️diff` leaves themselves and simplified `🦠️mutation`'s `fn diff` back to pure delegation:

- `set-default-app`: Warning `mutation.no-op` (empty-diff-shaped as `base.clone()`, not
  `D::default()`) when the exact `(dialect, role, app)` triple is already pinned — this facet's
  `Diff` type IS the whole `OpeningPreferences` value (`impl MutationDiff<OpeningPreferences> for
  OpeningPreferences { fn apply(_, _) -> Ok(self.clone()) }`), so `D::default()` would wipe every
  other pin; no-op therefore chains `.warn(..)` onto `MutationOutcome::new(base.clone())` rather than
  using `::error`/`::fatal` (which force `D::default()`). No Error/Fatal case applies to `set`: it is
  an unconditional upsert with no external target that can be absent.
- `clear-default-app`: same shape — Warning `mutation.no-op` (`base.clone()`) when the coordinate has
  no pin to clear.

`OpeningConfigMutation`'s hand-written `impl Mutation<OpeningPreferences>` in `🧬️mutations/🦀️component.rs`
was **already converted** (dispatch already forwarded `MutationOutcome`) — nothing to do there beyond
2 new no-op regression tests.

**Caveat on verification**: `🎚️config/🧬️schema` (the whole `os.config.opening` facet, including these
two leaves) is **not wired into `semio-framework-os-kernel`'s module tree** — confirmed via
`grep -rl opening_config …` returning nothing outside the facet's own files, and the facet's own doc
comment: "NOT YET wired into any crate's `📦️glue.rs` (out of this lease's scope; see
`.../ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/📓️w0-c-report.md`)". `cargo check -p
semio-framework-os-kernel` therefore passes without ever compiling these files — it is not a real
verification of this lease's changes. Did a careful manual type/borrow review instead (traced every
field type back to its derive list); found no issues. Wiring the facet in is out of this lease's
scope per its own doc comment and was not attempted.

## 3. `compose/client/lib/rs/lib.rs` — exactly 2 errors, both fixed

- `lib.rs:7831` `impl OperationDiff<KitSnapshot> for ComposeKitDiff::apply` — return type
  `KitSnapshot` → `MutationApplyResult<KitSnapshot>`, body wrapped in `Ok(..)`.
- `lib.rs:7874` `impl Mutation<KitSnapshot> for ComposeWireOperation::diff` — return type
  `Self::Diff` (bare `ComposeKitDiff`) → `MutationOutcome<Self::Diff>`, body wrapped in
  `MutationOutcome::new(..)` (mechanical wrap — `kind`/`input` is an untyped wire operation resolved
  at runtime through `crate::operation::Operation`, which owns its own error channel (`ComposeError`
  via `to_diff`) predating the §C2 message vocabulary; no per-verb detail to surface without
  duplicating `Operation`'s own validation — documented inline).
- Import line updated: added `MutationApplyResult`, `MutationOutcome` to the existing
  `use semio_framework_os_kernel::os_spr::{..}` in `kit_vcs`.
- No other call site in the file touches `ComposeWireOperation`/`ComposeKitDiff` directly — the
  generic `store.dispatch`/`materialize_document_projection` infra (used by `KitSnapshotStore`)
  is unaffected since it's already generic over the `Mutation`/`MutationDiff` trait bounds.

**Found but NOT fixed (outside the named "exactly 2 errors" scope, flagging for the coordinator)**:
`cargo test -p semio-compose-rs --lib` (not `cargo check`, and not requested for this lease) fails
with 6 errors, all inside one disused test fixture, `tests::vcs_typed_ops_materialize_projection`
(`lib.rs:19240-19312`). It has its own local `KitDiff`/`KitOperation`/`KitProjection` mock types with
the *same* stale `Mutation`/`MutationDiff` signatures I fixed above (lines 19265, 19285) **plus** an
unrelated, pre-existing `ArtifactVcsStore::new(..)` fallibility drift (`store.dispatch(..)` at
line 19308 — `ArtifactVcsStore::new` returns `Result` now, the `.expect(..)` was never added, unlike
every other call site in the file). Mixing two unrelated API drifts in one disused test; left alone
per the brief's literal "exactly 2 errors" / `cargo check` framing.

## Verification (real counts)

- `cargo check -p semio-compose-rs` — **PASS**, 0 errors, 825 pre-existing warnings (unrelated to
  the 2-line fix; `cargo fix` suggestions for `crate::external_adapters::serde_json` qualification,
  pre-existing).
- `cargo check -p semio-framework-os-kernel` — **PASS**, 0 errors, 9 warnings. See caveat above: does
  not compile the config facet at all (unwired).
- `cargo check -p semio-s-plugin-flow` — **BLOCKED**, transitively, by its direct dependency
  `semio-s-plugin-stdio` (`Cargo.toml`: `semio-s-plugin-stdio = { path = "../../../🗄️stdio/…" }`),
  which fails with **197 errors** (`OpText`/`OpBinary` unsatisfied trait bounds across ~34 legacy
  artifact mutation enums — `PdfMutation`, `DwgMutation`, `StepMutation`, `SemioAnimationMutation`,
  etc.). Confirmed via `git status` (`✏️s/🔌️plugins/🗄️stdio/**` uncommitted, actively modified) and
  cross-referenced against `📋️master-plan.md` §Fan-out recipe: "stdio's legacy enums are FULL-STDIO's
  charter" (ticket `26/08/16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS`, visibly
  mid-flight in this same repo). **Zero of the 197 errors reference any `🌊️flow` path** (`grep -i
  "🌊️flow"` on the full error log: no hits) — confirmed my 9 leaves introduce no compile errors of
  their own. A separate, unrelated transient error (`🧰️framework/…/🌊️flow/🌿️vcs/🦀️component.rs:193`,
  `.id` used as a field instead of method call) appeared on the first run (a concurrent peer session's
  in-progress commit `c8a29e41c5`, same ticket, different lane) and was gone on rerun — not mine to
  fix, self-resolved.
- `cargo test -p semio-s-plugin-flow --lib` — **could not run**, same transitive `semio-s-plugin-stdio`
  blocker.

Given the flow crate could not be compiler-verified, did a full manual re-read of all 9 diff leaves,
all 9 mutation leaves, the dispatch file's `apply_flow_mutation` fix, the 19 new tests, and the
duplicate-widget `validate` deletion + 4 test call-site fixes — traced every method/field back to its
derive list or trait definition (`Identified`, `PartialEq` on `Widget`/`WidgetLayout`, `FaultCode(pub
String)`, `MutationOutcome`'s generic vs `Default`-bounded impl blocks, `protocol::Severity` re-export)
to confirm each compiles. No issues found, but this is manual review, not a compiler pass — flagged
honestly rather than claimed as verified.

## Files touched

**Flow** (`✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/`):
`🦀️component.rs` (dispatch + new tests), `➕️create-widget/{🔺️diff,🦠️mutation}/🦀️component.rs`,
`🗑️delete-widget/{🔺️diff,🦠️mutation}/🦀️component.rs`, `🔗️connect-widgets/{🔺️diff,🦠️mutation}/🦀️component.rs`,
`✂️disconnect-widgets/{🔺️diff,🦠️mutation}/🦀️component.rs`, `📍️move-widgets/{🔺️diff,🦠️mutation}/🦀️component.rs`,
`🔁️replace-widget/{🔺️diff,🦠️mutation}/🦀️component.rs`, `🔀️🪟️reorder-widgets/{🔺️diff,🦠️mutation}/🦀️component.rs`,
`🔀️reorder-synapses/{🔺️diff,🦠️mutation}/🦀️component.rs`,
`🔄️update-synapse-endpoints/{🔺️diff,🦠️mutation}/🦀️component.rs`,
`👯️duplicate-widget/{🦠️mutation,🧩️plan}/🦀️component.rs`.

**Config** (`🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/`): `🦀️component.rs` (new
no-op tests), `📌️set-default-app/{🔺️diff,🦠️mutation}/🦀️component.rs`,
`🧹clear-default-app/{🔺️diff,🦠️mutation}/🦀️component.rs`.

**Compose**: `compose/client/lib/rs/lib.rs`.

Logs: `🧪️w3-flow-config-compose-check-flow.txt`, `🧪️w3-flow-config-compose-check-config.txt`,
`🧪️w3-flow-config-compose-check-compose.txt`, `🧪️w3-flow-config-compose-test-config.txt`.
