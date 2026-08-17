# W2 Packet P8 — Report

Lane: W2 packet P8, ticket `26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET`. Lease: `🔱️trinity`
(2 apps: `🔌️jack`, `♻️rewrite`), `📜️imperative`, `📖️playbook`, `🕸️dag`, `💡️reasoning` (app `🔌️wires`),
`📋️forms`, `📏️layout` — 8 subsets total across 7 plugins. Followed the pilot's recipe
(`📓️w2-cad-report.md`), the frozen contract (`📋️contract-freeze.md`), and the closed SDK gaps
(`📓️w0-f-report.md`).

## Method

Dispatched one foreground sub-agent per plugin in parallel (disjoint file trees, no shared-file risk),
each given the pilot's recipe, the frozen contract, and plugin-specific ground truth (app paths, window
layout, `artifact_kind` strings for `DIALECT`, struct/fn names) pre-derived by the coordinator. Every
agent moved its retired `🎛️apps/<app>/` tree into `✏️editor` (overwriting the W1-E scaffolder's
placeholder leaves), authored an independent `👁️viewer`, rewired `📦️glue.rs`/plugin root/artifact root/
`Cargo.toml`/`tsconfig.json`, deleted `🎛️apps/`, and wrote its own notes file.

Per-plugin details, DIALECT derivation, viewer design rationale, and files touched are in each plugin's
own notes file — this report summarizes and records the coordinator's own verification pass and one
fix it made after landing.

- `📓️w2-p8-trinity-notes.md` — subsets `s.trinity.jack@1/*`, `s.trinity.rewrite@1/*`
- `📓️w2-p8-imperative-notes.md` — subset `s.imperative.imperative@1/*`
- `📓️w2-p8-playbook-notes.md` — subset `s.playbook.playbook@1/*`
- `📓️w2-p8-dag-notes.md` — subset `s.dag.dag@1/*`
- `📓️w2-p8-reasoning-notes.md` — subset `s.reasoning.wires@1/*`
- `📓️w2-p8-forms-notes.md` — subset `s.forms.forms@1/*`
- `📓️w2-p8-layout-notes.md` — subset `s.layout.layout@1/*`

## Coordinator fix: trinity's missing editor `🟦️component.ts` twins

The repo-wide `bun ./📜️script.ts policy` CLI run (`🧪️w2-p8-policy-full.txt`) reads a cached
`compose.json` snapshot that turned out stale for this session's timing (built before this packet's
edits landed) — it reported 0 breaches for all seven plugins across every surface policy, which the
coordinator did not take on faith. Calling `policySubsetSurfaceCompletenessBreaches`/
`policyViewerPurityBreaches` directly (bypassing the cache) found 4 real breaches, all in `🔱️trinity`:
both `🔌️jack`'s and `♻️rewrite`'s editor windows had only `🦀️component.rs` (no `🟦️component.ts` at
all), and both editor surface roots were still the scaffolder's literal `SCAFFOLD = true` placeholder —
the original trinity session had authored real twins for both viewers but never got to the editor side.

Fixed directly by the coordinator (no Rust files touched): 9 new window-level `🟦️component.ts` twins
(one typed ViewModel per window, derived from each window's own `render()` Rust signature — jack's
Results window became a discriminated union since it renders either a table or a graph depending on
the last query's kind; rewrite's five graph windows each got their own ViewModel despite sharing
`render_fixture_graph()`, since editability/camera-override differs per call site) plus 2 real
surface-root re-export files replacing both `SCAFFOLD` placeholders, naming kept consistent with
trinity's own already-correct viewer twins. Re-verified live (not via the cache): 0 breaches for
`🔱️trinity` across `surface-completeness`, `surface-scaffold-residue`, `viewer-purity` after the fix.
Detailed in `📓️w2-p8-trinity-notes.md`'s "Coordinator follow-up" section.

## Coordinator verification (all 7 plugins, all live/direct — not cache-trusting)

**Cargo**, one crate at a time, serial (`RUSTC_WRAPPER="" cargo check -p <crate> --all-targets
--keep-going`, then `cargo test -p <crate> --no-run` or `cargo test -p <crate>`), output in
`🧪️w2-p8-<slug>-cargo.txt` / `🧪️w2-p8-<slug>-test.txt`:

| plugin | crate | cargo check: errors anchored in own files | cargo test: errors anchored in own files |
|---|---|---:|---:|
| trinity | semio-s-plugin-trinity | 0 | 0 |
| imperative | semio-s-plugin-imperative | 0 | 0 |
| playbook | semio-s-plugin-playbook | 0 | 0 |
| dag | semio-s-plugin-dag | 0 | 0 |
| reasoning | semio-s-plugin-reasoning-mindmap | 0 | 0 |
| forms | semio-s-plugin-forms | 0 | 0 |
| layout | semio-s-plugin-layout | 0 | 0 |

Every crate is currently blocked from finishing a full build by unrelated, live, uncommitted peer work
in `semio-s-plugin-stdio` and/or `semio-framework-plugin`/`semio-framework-os-kernel` (the two
known-broken-by-live-peers crates named in this packet's brief) — confirmed per run via
`git status --porcelain` (uncommitted `M`/`D` entries) and `git log --date=iso` (most recent real
commit `0727b80a`, 2026-08-16 12:10:56 — today, tied to the concurrent
`MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` ticket), never by parsing commit message
text. The failing crate moved between `semio-s-plugin-stdio` and `semio-framework-os-kernel` run to run
for the same plugin, matching the pilot's and w0-f's documented pattern of in-flight upstream churn.
Zero errors were ever anchored inside any of the seven plugins' own files across every run. Two agents'
(trinity, reasoning) in-session `cargo` runs stalled on the shared workspace target-dir lock (7
concurrent sibling packets' cargo processes) and were re-run by the coordinator once the lock cleared.

**Policy** — called `policySubsetSurfaceCompletenessBreaches`, `policyViewerPurityBreaches`,
`policyContributedSurfaceTargetBreaches`, `policyOsConfigShapeBreaches` from `📜️script.ts` directly
against the live filesystem (not the CLI's cached `compose.json`), filtered to the seven plugins' scope
strings, after the trinity fix above:

- `taxonomy/surface-completeness`: **0**
- `taxonomy/surface-scaffold-residue`: **0**
- `taxonomy/viewer-purity`: **0**
- `plugin-dependency/contributed-surface-target`: **0**
- `taxonomy/os-config-shape`: **0**

Cross-checked with direct filesystem greps (independent of the policy code): `grep -rl "SCAFFOLD"` under
all seven plugins' `🗿️artifacts` trees returns nothing; `grep -rl "::editor::"` and
`grep -rl "\.mutation(\|Emit::mutations\|artifact_mutations"` restricted to every `👁️viewer` dir under
the seven plugins both return nothing.

**Structural**: all seven `✏️s/🔌️plugins/<plugin>/🎛️apps/` directories confirmed deleted. All seven
artifact-level `<PLUGIN>_DIALECT`/`WIRES_DIALECT`/`DAG_DIALECT`/`FORMS_DIALECT`/`LAYOUT_DIALECT`/
`PLAYBOOK_DIALECT`/`IMPERATIVE_DIALECT`/`TRINITY_JACK_DIALECT`/`TRINITY_REWRITE_DIALECT` constants
confirmed present with the correct `s.<plugin>.<artifact>` grammar (cross-checked against each
subset's own `#[artifact_schema(id = "…")]`). `#[path]` resolution for every `📦️glue.rs` was verified
by each packet's own agent with the recipe's path-resolution script AND independently corroborated
here: a broken `#[path]` produces a hard `cargo check` compile error citing the missing file inside the
failing plugin's own trace, and none of the seven plugins' `cargo check` runs ever anchored an error in
their own files — so every `#[path]` genuinely resolves.

## SDK gaps found (framework, outside this packet's lease — beyond what w0-f already closed)

1. **`TextWindowKit`/`TableWindowKit`/`TreeWindowKit`/`WindowKit` (contract §2.6, `//#region
   🔖️WindowKits`) are not in `semio_framework_plugin`'s curated crate-root re-export list** — the same
   gap category w0-f's Gap 1 closed for `ArtifactEditor`/`ArtifactViewer`/`Editor`/`Viewer`/
   `EditorApp`/`ViewerApp`/`ViewEmit`, but the WindowKits region wasn't included in that fix. Every
   packet reaching for a window kit hits `E0432`; workaround used throughout this packet:
   `use semio_framework_plugin::app::{TextWindowKit, TableWindowKit, TreeWindowKit, …};`.
2. **`.example(...)`/`.workflow(...)` still don't exist on `EditorBuilder`/`ViewerBuilder`** (contract
   §2.4's `App { definition, examples }` split) — same gap the pilot documented and w0-f left open;
   every migrated app's example/workflow registration was dropped, not ported, and noted inline at each
   `create_*_app()`'s doc comment per packet.
3. **`testkit::assert_declared_actions_bridge_to_commands`'s signature is unchanged** (still
   `fn(manifest: fn() -> App)`) — packets whose pre-existing tests called it wrapped `create_x_app()` in
   a small local `App { definition, examples: Vec::new() }` shim, per w0-f's Gap 3 note.

## Outside-lease referrers (report, not fixed)

- Zero real (non-doc-comment) Rust compile dependencies found repo-wide on any of the seven retired
  `apps::<name>::` module paths — each packet grepped the whole repo, not just its own plugin.
- Root `📜️script.ts`'s large static path-string array (`:8177`-ish) still lists several plugins'
  already-deleted `🎛️apps/<plugin>/…` paths (including cad's, from the pilot) — confirmed not kept in
  sync with `🎛️apps` deletions by any packet so far; cosmetic, not a compile or policy blocker, not this
  packet's job (`policyTaxonomyDirsBreaches` doesn't walk surface subtrees until W3 per contract §6).
- `📏️layout`'s `📦️packages/🟦️typescript/📦️index.ts` has three pre-existing, already-broken relative
  imports (missing the `🏅️standards/🔖️1/🪆️subsets/✳️any` path segment) — dated 2026-08-12, unrelated to
  this migration, reported in `📓️w2-p8-layout-notes.md`, not fixed (out of scope, pre-existing).
- `💡️reasoning`'s `📦️packages/🟦️typescript/📦️index.ts` has three analogous pre-existing broken
  imports — same nature, reported in `📓️w2-p8-reasoning-notes.md`, not fixed.

## Files touched

See each plugin's own notes file for the full created/edited/deleted list. Coordinator-level additions
beyond what the seven sub-sessions did:

- Created (coordinator): 11 `🟦️component.ts` files under `🔱️trinity`'s `🔌️jack`/`♻️rewrite` editor
  trees (9 window twins + 2 surface-root re-exports, replacing the 2 `SCAFFOLD` placeholders).
- Edited (coordinator): `📓️w2-p8-trinity-notes.md`, `📓️w2-p8-reasoning-notes.md` (verification
  sections filled in with the coordinator's re-run cargo output after the shared-lock stall).
- Re-ran (coordinator, output overwritten with real results): `🧪️w2-p8-trinity-cargo.txt`,
  `🧪️w2-p8-trinity-test.txt`, `🧪️w2-p8-reasoning-cargo.txt`, `🧪️w2-p8-reasoning-test.txt`,
  `🧪️w2-p8-layout-test.txt` (all four had stalled on the shared cargo lock in-session).
- Created (coordinator): `🧪️w2-p8-policy-full.txt` (repo-wide CLI run, noted stale/cached — see above),
  scratch policy-check scripts in the session scratchpad (not in the ticket folder — throwaway, not
  referenced by any deliverable).

## Status

All 8 subsets across the 7 leased plugins have real, independent editor + viewer surfaces with 0
surface-completeness, scaffold-residue, or viewer-purity breaches, 0 own-file cargo errors (blocked only
by confirmed-live unrelated peer work), and 0 remaining `🎛️apps` directories. Ready for W3.
