# A1 — framework-core (io ArtifactRef + kernel grouping) report

Scope: exactly two files, per assignment —
`🧰️framework/🔨️modules/🚪️io/🦀️component.rs` and
`🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs`. No other file was edited.

## What changed

### TASK 1 — `🔖️ArtifactRef` region in `🚪️io/🦀️component.rs`

New region inserted immediately after `//#endregion 🔖️Dialect`, before `//#region 🔖️ComposeTypes`:

- `🧰️framework/🔨️modules/🚪️io/🦀️component.rs:85` — `//#region 🔖️ArtifactRef` opens.
- `🧰️framework/🔨️modules/🚪️io/🦀️component.rs:95` — `pub struct ArtifactKindId(String)` — newtype,
  canonical grammar `s.<plugin>.<artifact>` (exactly 3 dot-separated ASCII segments, first
  literally `s`, remaining two lowercase-ASCII kebab `[a-z0-9-]`, no leading/trailing/doubled
  hyphen). Derives `Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize` — matches
  `StandardId`/`SubsetId`'s derive set minus `Copy` (String isn't `Copy`; `StandardId`/`SubsetId`
  wrap `&'static str` so they can be). No serde rename attribute needed — tuple-struct newtypes
  serialize transparently by default, same as `StandardId`/`SubsetId`.
  - `ArtifactKindId::parse(&str) -> Result<Self, String>` — precise rejection message naming the
    grammar rule.
  - `as_str()`, `plugin()` (2nd segment), `artifact()` (3rd segment), `Display`.
- `🧰️framework/🔨️modules/🚪️io/🦀️component.rs:132` — free `pub fn is_canonical_artifact_kind(kind: &str) -> bool`
  — the predicate `parse` is built on, exposed standalone for policy/breach-scan callers.
- `🧰️framework/🔨️modules/🚪️io/🦀️component.rs:~136` — private `fn is_kebab_segment(segment: &str) -> bool` helper.
- `🧰️framework/🔨️modules/🚪️io/🦀️component.rs:162` — `pub struct ArtifactRef { pub artifact_id: String, pub dialect: ArtifactDialect }`.
  Derives `Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize` +
  `#[serde(rename_all = "camelCase")]` — matches `ArtifactDialect`'s exact derive/attr set (the
  neighbouring owned/wire value type in the same region).
  - `to_uri() -> String` — `"<artifact_id>!<kind>@<standard>/<subset>"`, built by reusing
    `ArtifactDialect::to_coordinate()` for everything after `!` (no re-derivation of the
    dialect-coordinate codec).
  - `parse_uri(&str) -> Result<Self, String>` — exact inverse; splits on the FIRST `!`, then
    delegates to `ArtifactDialect::parse_coordinate` for the remainder, so artifact ids containing
    dots/dashes round-trip (only `!` is special to `ArtifactRef` itself).
- `🧰️framework/🔨️modules/🚪️io/🦀️component.rs:185` — `//#endregion 🔖️ArtifactRef`.

No existing artifact id anywhere in the repo was renamed — this wave lands only the type +
validator, as instructed.

Unit tests added to the file's existing `#[cfg(test)] mod tests` (line ~638), matching its style
(plain `#[test] fn` names, `assert!`/`assert_eq!`, no test framework macros):

- `artifact_kind_id_accepts_canonical_grammar` — accepts `s.stdio.stl`, `s.stdio.semio`.
- `artifact_kind_id_rejects_non_canonical_grammar` — rejects `stdio.stl`, `3d.cad`,
  `data.🏛️program`, `s.Stdio.stl`, `s.stdio`, `s.stdio.stl.extra`, `s..stl`, `s.stdio.-stl` (the
  exact reject table from the task).
- `artifact_ref_uri_round_trips` — round-trips two cases, including an artifact id containing
  dots and dashes (`"doc.v2-final.draft"`) paired with a dialect whose kind also contains a dash
  (`"s.norm.en-1994-1"`).
- `artifact_ref_to_uri_matches_expected_shape` — pins the exact wire string
  `"abc123!s.stdio.gif@87a/*"`.
- `artifact_ref_parse_uri_rejects_malformed_input` — missing `!`, empty artifact id.

### TASK 2 — grouping types in `🎠️kernel/🦀️component.rs`

- `InvocationId` already existed (`🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs:46`,
  `pub struct InvocationId(pub String)`, `#[serde(transparent)]`) and `UndoGroup.invocation_id`
  already used it — **no new type introduced**, reused as-is per the task's own instruction to
  check first. See "Design decisions" below.
- `🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs:451` — new
  `pub struct EditRef { pub document: ArtifactHandle, pub edit_id: String }`, using the real
  `ArtifactHandle` newtype already defined in this file (line 12). Derives
  `Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize` + `#[serde(rename_all = "camelCase")]`
  — same set as the file's other small value structs (e.g. `IconRenderExportItem`,
  `PhysicalSize`); `Eq`/`Hash` are sound here (`ArtifactHandle` and `String` both support them)
  and consistent with `ArtifactHandle`'s own derive set.
- `🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs:465` — `UndoGroup` gains
  `#[serde(default, skip_serializing_if = "Vec::is_empty")] pub member_edits: Vec<EditRef>`,
  additive, attributed exactly like `KernelMutation.dependencies` / `InverseMutation.dependencies`
  in the same file (`#[serde(default, skip_serializing_if = "Vec::is_empty")]` on a `Vec` field).

Construction-site sweep: `UndoGroup { .. }` is **not constructed anywhere inside either of my two
files** — it is only defined in `🎠️kernel/🦀️component.rs`, never literal-constructed there or in
`🚪️io/🦀️component.rs`. So there was nothing to update in-boundary. A repo-wide grep (read-only,
for the sweep only) found it constructed in exactly two other files, both outside this crate — see
`## sharedFileRequests`.

### TypeScript mirror

- `🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts:243` — new `export type EditRef = { readonly document: number; readonly editId: string }`
  (`document: number` mirrors how `KernelMutation.artifact: ArtifactHandle` is already mirrored as
  `artifact: number` a few lines above it in the same file — the file's established
  `ArtifactHandle → number` convention).
- `🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts:253` — `UndoGroup` gains
  `readonly memberEdits?: readonly EditRef[]` (optional, matching the Rust field's
  `#[serde(default, skip_serializing_if)]` additive/optional semantics).

## Verification (actually run)

```
CARGO_TARGET_DIR=".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM/🎯️target" cargo check -p semio-framework
```
Result: **clean.** Full output saved to `scratch-w1a-check-1.txt` in this ticket folder.
`grep -c "^error"` on that file → `0`. Only pre-existing warnings from unrelated files
(`glue.rs` ambiguous glob re-exports, unused `len` in `🔍️lexer`, dead `print_edge_label` in
`🖋️notation`, unused `set_envelope` in `🏪️store`) — none in `🚪️io/🦀️component.rs` or
`🎠️kernel/🦀️component.rs`, and none newly introduced (grep for `ArtifactKindId`/`ArtifactRef`/
`EditRef`/`member_edits` in the log: zero hits, i.e. zero diagnostics against the new code).
`semio-framework-os-kernel` (a dependency crate compiled along the way) reported its baseline 49
warnings, matching the stated pre-change baseline — no regression there either. Final line:
`Finished \`dev\` profile [unoptimized] target(s) in 30.91s`.

```
CARGO_TARGET_DIR=".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM/🎯️target" cargo test -p semio-framework --lib
```
Result: **125 passed; 0 failed; 0 ignored.** Full output saved to `scratch-w1a-check-2.txt`.
Confirmed by direct grep of the log that all 6 new/kept `io::tests` ran and passed:
```
test io::tests::artifact_kind_id_accepts_canonical_grammar ... ok
test io::tests::artifact_kind_id_rejects_non_canonical_grammar ... ok
test io::tests::artifact_ref_uri_round_trips ... ok
test io::tests::artifact_ref_to_uri_matches_expected_shape ... ok
test io::tests::artifact_ref_parse_uri_rejects_malformed_input ... ok
test io::tests::io_compose_via_chains_two_registered_hops ... ok
test io::tests::io_compose_via_surfaces_hub_resolve_failure ... ok
```

## Design decisions

- **`InvocationId`**: kept the existing `InvocationId(pub String)` newtype
  (`🎠️kernel/🦀️component.rs:46`) exactly as-is. `UndoGroup.invocation_id` already used it before
  this wave, so there was no parallel/bare-`String` representation to reconcile — the task's
  "introduce a newtype only if... check first" branch resolved to "already done, reuse."
- **`EditRef` derive set**: added `Eq, Hash` beyond the minimal `Clone, Debug, PartialEq, Serialize,
  Deserialize` most structs in this file's `🔖️Invocation` region carry, because `EditRef`'s two
  fields (`ArtifactHandle`, `String`) both support `Eq`/`Hash` and `ArtifactHandle` itself derives
  them (line 10-12) — consistent with treating `EditRef` as an id-shaped comparison key (e.g. for
  future dedup/set membership when the composition coordinator collects member edits), not
  over-deriving relative to what its fields support.
- **`ArtifactRef.dialect` stays `ArtifactDialect`**, not `ArtifactKindId` + separate standard/subset
  fields — this matches the task's literal struct shape and keeps `ArtifactRef` decoupled from the
  as-yet-unadopted `ArtifactKindId` grammar; `Dialect.artifact_kind`/schema ids/catalog keys
  migrating to `ArtifactKindId` is explicitly a later wave per the design doc ("Kind becomes
  newtype `ArtifactKindId`... " under section 1, not scoped to this ticket slice).
- **`is_kebab_segment` kept private** (`fn`, not `pub fn`) — only `is_canonical_artifact_kind` (the
  task's requested free function) and `ArtifactKindId::parse` need it; no external caller was
  specified.

## sharedFileRequests

`UndoGroup { .. }` literal construction sites outside my two files, both in crate
`semio-framework-os-kernel` (a downstream crate — NOT covered by `cargo check -p semio-framework`,
so my verification above cannot observe their state; not touched, per instructions):

1. **`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:5415` and `:5476`** —
   `UndoGroup { invocation_id, mutations: ..., inverse_mutations: ... }`, field names match the
   current struct exactly. These two need `member_edits: Vec::new()` added once this crate is
   rebuilt against the updated `UndoGroup` shape, or they will fail to compile (Rust struct
   literals require every field; there is no way to make an addition here "free" for full
   struct-literal call sites without either editing them or giving `UndoGroup` a `Default` impl
   *and* switching these call sites to `..Default::default()` — both are edits to a file this
   ticket's ownership table assigns to "W1 mechanism agent" as a *different* file from mine, so I
   left it alone and am filing this request instead).
2. **`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ProgramBridge/🧊️component.rs:143` and `:445`** —
   also construct `UndoGroup { .. }`, but **already use stale field names** (`operations`,
   `inverse_operations`) that don't match the current struct (`mutations`, `inverse_mutations`) —
   this predates my change and is unrelated to it (see Concurrent-churn observations below). Once
   whoever owns that file reconciles the field-name drift, it will also need `member_edits`.

Region: `🔖️Invocation` in both target files (wherever `InvocationResult`/`UndoGroup` literals are
built). No patch file was written (out of my file scope to touch `🔧️patches/`, and the exact fix
is a one-line-per-site addition, spelled out above).

## Concurrent-churn observations

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ProgramBridge/🧊️component.rs`
  currently constructs `InvocationResult`/`UndoGroup` with field names (`operations`,
  `inverse_operations`) that do not exist on the current `kernel::InvocationResult`/`UndoGroup`
  definitions (`mutations`/`inverse_mutations`). This mismatch predates my edits (I never touched
  this file, and my own two files' `cargo check -p semio-framework` run is clean — the mismatch
  cannot be mine since `ArtifactHandle`/`UndoGroup`'s field names are unchanged by my diff, only a
  new field was appended). This file lives in `semio-framework-os-kernel`, a crate my scoped
  `-p semio-framework` check does not build, so I cannot confirm from my own run whether this is a
  live compile error there right now or an already-red spot in that crate; either way it is
  outside my two-file boundary and I did not touch it. Flagging for whoever owns that file /
  crate's next full-crate check.
- `git status --porcelain` at report time also shows `🧰️framework/🔨️modules/🧬️schema/✨️derive/🦀️component.rs`,
  `🧬️schema/🦀️component.rs`, and `🧬️schema/🟦️component.ts` as modified — **not by me** (I never
  opened those files). Per the hot-file table these are also W1-mechanism-agent territory but a
  different file than mine, so this is a sibling W1 sub-task's concurrent edit, not SMO. Since
  those files live in the same `semio-framework` crate, my `cargo check -p semio-framework` run
  above necessarily compiled them too — it passed with 0 errors, so as of my check their
  in-progress state was compatible with my `🚪️io`/`🎠️kernel` additions. Not flagged as
  `blocked-mechanism` since nothing failed.
- Outside my crate entirely, `git status` shows heavy concurrent SMO fan-out churn (dozens of
  `✏️s/🔌️plugins/**/🧬️mutations/**` files across norm/energy/space/puzzle/animate/gis/flow/etc.,
  plus several `📓️wave*-reports` under the sibling `SEMANTIC-MUTATIONS-OVERHAUL` ticket). Expected
  per this ticket's `📌️important.md` §"Coordination with SEMANTIC-MUTATIONS-OVERHAUL" — none of it
  touches my two files or `semio-framework`, so no action needed from me.

## Files touched

- `🧰️framework/🔨️modules/🚪️io/🦀️component.rs` (new `🔖️ArtifactRef` region + tests)
- `🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs` (`EditRef` + `UndoGroup.member_edits`)
- `🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts` (TS mirror: `EditRef` + `UndoGroup.memberEdits`)
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM/scratch-w1a-check-1.txt` (cargo check output)
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM/scratch-w1a-check-2.txt` (cargo test output)
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM/📓️wave1-reports/a1-framework-core-report.md` (this report)

No other file was created, edited, or removed. Ticket left open (not closed); `📓️status.md` not touched.
