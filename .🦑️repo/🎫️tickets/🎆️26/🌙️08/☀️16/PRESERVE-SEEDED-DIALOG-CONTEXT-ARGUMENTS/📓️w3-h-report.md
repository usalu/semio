# W3-H report — clear the `semio-s-plugin-puzzle` blocker for the wgpu shell lanes

Lane 3-H. Scope per brief: fix the named `semio-s-plugin-puzzle` compile blocker, regression-guard a
spread of other plugins, then confirm `semio-framework-os-renderer-wgpu` compiles/tests. Confirmed
before touching anything: `MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS/🎫️ticket.json`
is `"status": "closed"` and its `📌️important.md` is gone (deleted); `FULL-STDIO-…/🎫️ticket.json` is
`"status": "open"` (`🗄️stdio/**` therefore stayed untouched, confirmed by `git status --porcelain`).

## 1. Diagnosis (task 1)

`cargo check -p semio-s-plugin-puzzle` baseline (`🧪️3-h-puzzle-check-baseline.txt`) — **3 real errors**
(the brief's "4 errors total" is `grep -c "^error"` counting the trailing `error: could not compile …
due to 3 previous errors` summary line as a 4th match; there is no undiscovered fourth compile error):

```
error[E0277]: the trait bound `Puzzle2dMutation: SemanticMutation<Puzzle2dPlaySnapshot>` is not satisfied
error[E0277]: the trait bound `Puzzle3dMutation: SemanticMutation<Puzzle3dPlaySnapshot>` is not satisfied
error[E0277]: the trait bound `Puzzle5dMutation: SemanticMutation<Puzzle5dPlaySnapshot>` is not satisfied
```

All three come from `✏️s/🔌️plugins/🧩️puzzle/🦀️component.rs:44,48,52` —
`.editor_mutation_roster::<crate::editor::puzzle{2,3,5}d::Puzzle{2,3,5}dPlayApp>()`.

**Which option, and why:** option **(a)**, but not the naive version. The plugin already opts in
explicitly (it calls `.editor_mutation_roster()` right after each `.editor()` — this is NOT the
missing-split shape lane 2-0 fixed; that split (`document_app`/`document_app_mutation_roster`) is
already in place and already working correctly here). The real gap: each `Puzzle{N}dPlayApp::Snapshot`
is `Puzzle{N}dPlaySnapshot` (a `serde_json::Value`-backed newtype — the play app predates the typed
`Puzzle{N}dSnapshot` and stays on ad-hoc JSON, per each mutations file's own `🔖️PlaySnapshot` region
doc), but `#[derive(dsl::Mutations)]` only ever generates
`impl SemanticMutation<Puzzle{N}dSnapshot>` (the typed snapshot named in `#[mutations(snapshot = …)]`)
— never for `Puzzle{N}dPlaySnapshot`. Each mutations file already hand-bridges `Mutation<Value>` and
`Mutation<Puzzle{N}dPlaySnapshot>` onto the SAME derive-generated logic (the `🔖️ValueBridge`/
`🔖️PlaySnapshot` regions), exactly because of this same predates-the-typed-struct situation — so the
missing piece was purely the `SemanticMutation` twin of that already-established bridge, not a bound
that needs relaxing anywhere.

Ruled out **(b)** (these are not "not semantic" — `CreateNode`/`DeleteNode`/`MoveNode`/… are real,
meaningful, `APPROVED_VERBS`-checked semantic vocabulary; dropping `.editor_mutation_roster()` would
silently lose real roster/introspection coverage for three real editor apps, which the ticket's
"you must not trade one broken plugin for another" instruction and CLAUDE.md's "get everything
working" both argue against). Ruled out **(c)** — no missing split anywhere; `🏗️builder/🦀️component.rs`
was not touched (see "Lease" below, confirmed unnecessary).

**Why a hand-written impl is legitimate here** (the `SemanticMutation` trait doc says "Implemented only
by `#[derive(Mutations)]`, never by hand"): none of its four methods (`kinds`/`semantics`/`label`/
`target`) actually read the projection type `P` — `kinds()` takes no `self`/`P` argument at all, and the
other three only read `self`. The generated `impl SemanticMutation<Puzzle{N}dSnapshot>`'s bodies are
projection-independent by construction. Forwarding through `<Self as
SemanticMutation<Puzzle{N}dSnapshot>>::kinds()` etc. is therefore not new logic, just a second type
parameter for the identical vocabulary — the same shape of bridge each file already applies to
`Mutation`/`MutationDiff`, not a workaround around the derive.

## 2. Fix (task 2)

Added one hand-written forwarding `impl protocol::SemanticMutation<Puzzle{N}dPlaySnapshot> for
Puzzle{N}dMutation` per dimension, placed immediately after the existing `impl
Mutation<Puzzle{N}dPlaySnapshot>` block inside the `//#region 🔖️PlaySnapshot` region (same region, same
pattern, doc-commented with the emoji-first convention):

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`

Each addition is 21 lines, insertion-only (`git diff --stat` confirms), e.g. (puzzle2d):

```rust
impl protocol::SemanticMutation<Puzzle2dPlaySnapshot> for Puzzle2dMutation {
    fn kinds() -> &'static [protocol::SemanticDescriptor] {
        <Self as protocol::SemanticMutation<Puzzle2dSnapshot>>::kinds()
    }
    fn semantics(&self) -> &'static protocol::SemanticDescriptor {
        <Self as protocol::SemanticMutation<Puzzle2dSnapshot>>::semantics(self)
    }
    fn label(&self) -> String {
        <Self as protocol::SemanticMutation<Puzzle2dSnapshot>>::label(self)
    }
    fn target(&self) -> Vec<String> {
        <Self as protocol::SemanticMutation<Puzzle2dSnapshot>>::target(self)
    }
}
```

`cargo check -p semio-s-plugin-puzzle` (`🧪️3-h-puzzle-check-fixed.txt`): **0 errors** (282 pre-existing
warnings, unchanged in kind from baseline). `🏗️builder/🦀️component.rs` was never touched — the
document_app/document_app_mutation_roster split lane 2-0 already built is sufficient as-is; nothing in
the puzzle plugin needed a further split there.

## 3. Regression guard (task 3)

All green, 0 errors each (full tails in the named logs):

- `cargo check -p semio-s-plugin-space` → 0 errors, 55 warnings (`🧪️3-h-space-check.txt`)
- `cargo check -p semio-s-plugin-dag` → 0 errors, 42 warnings (`🧪️3-h-dag-check.txt`)
- `cargo check -p semio-s-plugin-norm` → 0 errors, 240 warnings (`🧪️3-h-norm-check.txt`)
- `cargo check -p semio-framework-plugin` → 0 errors, 86 warnings (`🧪️3-h-framework-plugin-check.txt`)

No plugin was traded for another.

## 4. The real goal — `semio-framework-os-renderer-wgpu` (task 4)

`cargo check -p semio-framework-os-renderer-wgpu` (`🧪️3-h-wgpu-check.txt`): the puzzle blocker is
gone — **zero errors mention `semio-s-plugin-puzzle`** — but the crate still does **not** compile:
**4 errors, all inside this crate's own pre-existing code, none related to identity/directory/
presence/check-in** (the task-4 "ours" list: identity actor shape, default bindings, directory fold,
`os.open-artifact`, presence filtering, status pill, check-in guard). All 4 are **outside this lane's
lease** (`✏️s/🔌️plugins/🧩️puzzle/**`, conditionally `🏗️builder/🦀️component.rs` — neither file below is
either):

1. **`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ProgramBridge/🧊️component.rs:129,221,250`**
   — three `AppFrame::Error { in_reply_to, fault }` patterns missing the `report` field.
   Root cause: `AppFrame::Error` gained a third field, `report: Vec<u8>`
   (`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️component.rs:270`) — `📡️spr/**` is
   explicitly **forbidden** to me per `📋️ownership-and-handoffs.md` ("MUTATION-OUTCOMES — forbidden
   (consume as-is)"). `git log --date=iso` on the channel file's last touch: `5a1367dfcc`,
   `2026-08-16 14:18:35` (commit `🚩️524`). `ProgramBridge/🧊️component.rs`'s own last touch predates
   that: `6223c64fbb`, `2026-08-15 21:47:03` (`🚩️517`) — it genuinely predates the schema change and was
   never updated to match. Per `📋️master-plan.md:116`, `ProgramBridge/🧊️component.rs` is lane 2-D's
   scope, not mine; 2-D's own report (`📓️w2-d-report.md`) never touched this file (it only discusses
   the wasm32-only `"actor": "local"` region there, a different part of the file). Mechanical fix (not
   applied): add `, report: _` (or `, ..`) to each of the three patterns.
2. **`Dock/🧊️component.rs:1410`** (a `command_registry_tests`-style test fixture — verified against
   the analogous, definitely-pre-existing `Shell/🧊️component.rs:6411 command_registry_tests::test_app`
   fixture below) and **`Shell/🧊️component.rs:6411`** (`command_registry_tests::test_app`, a
   pre-existing test helper, NOT lane 2-D/3-A's new `identity_directory_presence_tests` module) — both
   `AppDefinition { … }` struct literals are missing the `role`/`dialect` fields. Root cause:
   `AppDefinition` gained non-defaulted `pub role: AppRole` / `pub dialect: ArtifactDialect` fields in
   `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs:2694-2700`, part of the canonical
   `<dialect>/<subset>@<v>/*#<role>` surface-id convention lane 2-0's own report already identified
   (`git log --date=iso` on that convention's introducing commit: `07873f842a`,
   `2026-08-16 11:00:35`) as belonging to `26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET` — checked
   just now, **still open**, not mine to fix or attribute to the closed MUTATION-OUTCOMES ticket.
3. **`Dock/🧊️component.rs:1087`** — `paint_dock_tab_icon(ctx, icon_id, …, tab_rect, tint)` passes
   `tab_rect: &Rect` (bound by-ref from `for (window_id, label, icon_id, tab_rect) in &tabs` at line
   1074) where `fn paint_dock_tab_icon(…, tab_rect: Rect, …)` (line 1012) wants an owned `Rect`.
   Unrelated to either schema change above — a plain local type mismatch, first ever seen by `rustc`
   here because the crate could never compile far enough before today. Mechanical fix (not applied):
   `*tab_rect` at the call site.

None of these three files are in my lease or in the task-4 "ours" whitelist, so — mirroring task 5's
instruction for the puzzle case exactly — I stopped and report with `file:line` +
`git log --date=iso` attribution rather than forcing a foreign-file edit un-authorized by the
coordinator.

`cargo test -p semio-framework-os-renderer-wgpu --lib` (`🧪️3-h-wgpu-test.txt`): **6 errors** (the same
4 above, plus the `Shell/🧊️component.rs:6411` and `Dock/🧊️component.rs:1410` `AppDefinition` literals
above, which only compile under `#[cfg(test)]`) — **could not run, 0/0 real test numbers**. This means
**lane 2-D's and 3-A's own new tests (`identity_directory_presence_tests`, 9+1 tests) still have never
once executed** — the puzzle blocker was necessary but not sufficient; three more pre-existing,
out-of-lease compile errors stand between here and that first run.

## Bonus finding (not fixed, not in the named task, flagged for visibility)

`cargo test -p semio-s-plugin-puzzle --lib` (`🧪️3-h-puzzle-test.txt`, run as an extra check beyond the
brief's scope): **151 errors**, entirely separate from anything above — `MutationOutcome<D>` has no
`.apply()`, `Result<ArtifactStore<…>, VcsError>` has no `.envelope()`/`.dispatch()`, and
`Puzzle3dMutation: Mutation<Result<Puzzle3dSnapshot, MutationApplyError>>` is expected but not
satisfied. Confirmed pre-existing and unrelated to my two-error-class fix above (my only edits added
zero `#[cfg(test)]` code and zero `diff`/`dispatch`/`envelope` calls): `MutationOutcome`
(`📡️spr/🎮️command/🦀️component.rs`) and the `protocol::testkit` helpers
(`📡️spr/🧪️testkit/🦀️component.rs`, imported by every puzzle mutations file's own test module) are both
committed clean (`git status --porcelain` = empty) as of the standing HEAD sweep
(`c8a29e41c5`, `2026-08-16 20:26:15`) — not a live in-progress edit, a real standing gap. Both files
are under `📡️spr/**`, **forbidden** to me per the ownership table. `cargo check` (non-test) never
exercises this path, which is why it was invisible until I ran `cargo test` as a bonus check. This
affects only the puzzle plugin's *own* test suite, not `semio-framework-os-renderer-wgpu` (which only
depends on puzzle's library code, not its `#[cfg(test)]` module) — so it does not block task 4's goal,
but it does mean `cargo test -p semio-s-plugin-puzzle --lib` cannot report real pass/fail numbers
either. Flagging for the coordinator; explicitly out of scope for this lane (way beyond "the blocker,"
touches forbidden `📡️spr/**`).

## Changed files

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  — added `impl protocol::SemanticMutation<Puzzle2dPlaySnapshot> for Puzzle2dMutation`.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  — added `impl protocol::SemanticMutation<Puzzle3dPlaySnapshot> for Puzzle3dMutation`.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  — added `impl protocol::SemanticMutation<Puzzle5dPlaySnapshot> for Puzzle5dMutation`.

(Confirmed via `git diff --stat` that each of the three shows only the 21-line insertion; no other
puzzle-tree file was written by me — the many other `M` files under
`✏️s/🔌️plugins/🧩️puzzle/**` at session start/end are pre-existing, unrelated, uncommitted churn from
before this lane started, e.g. `➕add-node-handle/🔺️diff/🦀️component.rs` last touched
`2026-08-16 23:11:36`, hours before this session.)

## Commands run (real tails, all logged to `$T/🧪️3-h-*.txt`)

- `cargo check -p semio-s-plugin-puzzle` — baseline 3 errors → fixed 0 errors.
- `cargo check -p semio-s-plugin-space` — 0 errors.
- `cargo check -p semio-s-plugin-dag` — 0 errors.
- `cargo check -p semio-s-plugin-norm` — 0 errors.
- `cargo check -p semio-framework-plugin` — 0 errors.
- `cargo check -p semio-framework-os-renderer-wgpu` — 4 errors, all outside lease (see above).
- `cargo test -p semio-framework-os-renderer-wgpu --lib` — 6 errors, could not run, 0/0 real numbers.
- `cargo test -p semio-s-plugin-puzzle --lib` — 151 errors, pre-existing, unrelated, forbidden territory
  (bonus check, not required by the brief).

## sharedFileRequests

For whoever owns these three files (2-D per `📋️master-plan.md:116` for `ProgramBridge`; unclear/unowned
for `Dock`; coordinator to route):

1. `ProgramBridge/🧊️component.rs:129,221,250` — add `report: _` (or `..`) to the three
   `AppFrame::Error{in_reply_to, fault}` match patterns.
2. `Dock/🧊️component.rs:1410` and `Shell/🧊️component.rs:6411` — add `role: …` / `dialect: …` to the
   two `AppDefinition{…}` test-fixture literals (needs a real `AppRole`/`ArtifactDialect` value, a
   small design choice for whoever owns the fixture — I did not pick one).
3. `Dock/🧊️component.rs:1087` — `*tab_rect` (dereference; `tab_rect` is `&Rect` from `for … in &tabs`).

For whoever owns `📡️spr/🎮️command`/`📡️spr/🧪️testkit` (forbidden to me): `cargo test -p
semio-s-plugin-puzzle --lib` has 151 pre-existing errors — `MutationOutcome<D>` has no `.apply()`,
`Result<ArtifactStore<…>, VcsError>` has no `.envelope()`/`.dispatch()`, and
`Puzzle{2,3,5}dMutation: Mutation<Result<…Snapshot, MutationApplyError>>` is expected but unsatisfied —
`protocol::testkit`'s helpers appear to target a different/newer `Mutation`/`MutationOutcome` API shape
than what's actually implemented. Not investigated further (way outside this lane's named task and
lease).

## What is NOT done

- `semio-framework-os-renderer-wgpu` still does not compile — 4 errors, all outside my lease (see task
  4 above). Lane 2-D's and 3-A's `identity_directory_presence_tests` (9+1 tests) have STILL never
  executed once. Re-run `cargo check -p semio-framework-os-renderer-wgpu` and
  `cargo test -p semio-framework-os-renderer-wgpu --lib` the moment the three foreign fixes above land.
- `cargo test -p semio-s-plugin-puzzle --lib`'s 151 pre-existing errors (forbidden `📡️spr/**`
  territory) — reported, not fixed, not in scope.
- Ticket not closed (coordinator owns that), never called `ticket_close`/`ticket_reopen`.
