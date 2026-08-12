# Status

Finishing plan: `/Users/ueli/.claude/plans/finish-semantic-mutations-overhaul-melodic-horizon.md`
Census + evidence: `📓️remaining-work-map.md`. Agent recipe: `📓️fanout-brief.md`,
`📓️stdio-lane-brief.md`.

## Waves 0–2 (earlier sessions): DONE / PARTIAL

- Wave 0 mechanism + policy: DONE (`📓️wave0-mechanism-report.md`, `📓️wave0-policy-rules-report.md`).
- Wave 1 exemplars: DONE (`📓️wave1-reports/`).
- Wave 2 fan-out: PARTIAL — 25 facet reports in `📓️wave2-reports/`, 32/107 facets migrated,
  but agents were denied `📦️glue.rs` edits, leaving 4 crates compile-broken, legacy dirs pinned,
  and app call sites dangling.

## Wave R (repair + policy trueing) — IN PROGRESS

| step | owner | state |
|---|---|---|
| R1 glue repair: writer, vcs, flow, sequence + flow's 8 leaves | agent | running |
| R2a gis 12 leaves + shooting 1 | agent | **DONE** — gis clean, 170/170 lib tests pass; `features_delta_from_collection_mutation` deleted |
| R2b animate 6 + layout 4 + process 2 leaves | agent | running |
| R2c architect 4 leaves | agent | **DONE** — leaves were already structurally correct; only banned tokens in doc-comments, now reworded |
| R3 `📜️script.ts` policy trueing | coordinator | HELD until the fan-out settles, so the re-seeded allowlist census is not stale |

### Coordinator fix: architect unblocked

`✏️s/🔌️plugins/🏛️architect/📦️packages/🦀️rust/📦️glue.rs:938` re-exported
`…::any::io::registers::*`, but the module actually lives at `🧬️schema/🗄️registers` (mounted at
glue.rs:74–75). One-token fix to `…::any::schema::registers::*` cleared **149 of 254** errors.
The remaining 105 are all this ticket's own funnel debt (app commands constructing
`SetAdjacency`/`ClearAdjacency`/`SetSnapshot`/`Reports(CollectionMutation…)`), now owned by the
Wave-C architect lane.

## Wave C (cleanup + funnel) — IN PROGRESS

| lane | plugins | state |
|---|---|---|
| architect | `🏛️architect` (105 errors → restructure 72 noun dirs into ~266 triad dirs + glue rewire) | running |
| shooting/demonstrator/lowpoly | those 3 plugins | running |
| remaining | animate, process, reasoning, layout, gis, mathematical, writer, vcs, flow app+config debt | not yet launched |

## Wave M (mass fan-out) — IN PROGRESS

| lane | facets | state |
|---|---|---|
| space + trinity | space-home, trinity-jack, trinity-rewrite | running |
| note | note | running |
| singles | energy-model, sourcing-curate, raster, dag | running |
| block | block ◻2d/🧊️3d/🖐️5d | running |
| puzzle | puzzle ◻2d/🖐️5d/🧊️3d | running |
| norm | all 15 norm facets (5 new + 10 dir/glue trueing) | running |
| stdio | 53 facets, 10 family sub-lanes + funnel agent | not yet launched (brief ready) |
| odd remainder | remodel, imperative, playbook | not yet launched |

### playbook design decision: RESOLVED → move the vocabulary into the plugin

An evidence-gathering pass settled this. Correction to an earlier note here: the blocker is NOT
the orphan rule (a plugin-local payload struct may legally
`impl MutationKind<PlaybookSnapshot, PlaybookMutation>` even when both type parameters are
foreign). The real constraint is **crate dependency direction** — if the dispatch enum stays in
the framework, its variants cannot wrap plugin-defined payload structs, which would force all 9
plugin triad leaves to be permanent re-export shims and violate this overhaul's rule that
`🦠️mutation` leaves contain a real `impl MutationKind<`.

Findings that make the move safe:
- `PlaybookMutation` / `apply_playbook_edit_mutation` have **zero** references anywhere in
  `🧰️framework` outside `🔨️modules/📖️playbook/🦀️component.rs` itself (82 refs, all local).
- The module is mounted in exactly one place (flow's `📦️glue.rs:43-46`) and is not re-exported
  through `protocol::…`. `PlaybookStore`/`PlaybookEnvelope` and the
  `playbook-document-wasm` `wasm_bridge` are dead (no Cargo feature enables it).
- `🌊️flow`'s `forms_bridge` and the `📋️forms` plugin consume only the DOMAIN types
  (`PlaybookSpec`/`PlaybookStep`/`PlaybookBlock`) and `builder_kit`'s rendering half — never the
  mutation vocabulary.
- The plugin already owns a richer sparse `PlaybookDiff` plus a real 200-line
  `playbook_diff_from_mutation` translator, so the per-kind `🔺️diff` leaves have real logic to
  receive. (Two same-named `PlaybookDiff` types exist today — framework tag-enum vs plugin sparse
  struct — a pre-existing duplication this move removes rather than worsens.)
- `git log` shows the framework playbook file last touched 2026-08-11 and clean in
  `git status` — no other session is in it.

Work: define `PlaybookMutation` in the plugin facet with 9 single-tuple variants; give each of
the 9 existing triad dirs a real payload struct + `MutationKind` impl with diff/inverse
distributed from `playbook_diff_from_mutation`; delete ~452 framework lines (the `🔖️Mutations`
region 280-534, the `OpText`/`OpBinary` impls 602-633, the dead aliases/wasm bridge, the 7
`builder_kit` op-builders, and the orphaned tests) while KEEPING all domain types, validation,
`generation_forms`, and `builder_kit`'s rendering half; update the app's 7 op-builder call sites;
re-verify flow and forms still compile.

## Cross-ticket coordination with `UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (peer session, issue 2548)

A concurrent session owns ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` and has taken an
explicit claim on `✏️s/🔌️plugins/🗄️stdio/**`. Negotiated outcome:

**stdio ordering: THEIR restructure first, this ticket's 53 stdio mutation facets after.**
Their change alters which subsets exist (`🧿️semio` roster 13→18: `✳️workflow`→`✳️flow`,
`✳️object`→`✳️value`, plus new `text`, `table`, spatial `object`, `graph`, `kit`); this ticket
rewrites the mutation vocabulary inside each. Going first would mean doing `✳️workflow` and
`✳️object` twice and covering 13 subsets when there will be 18. The stdio lane brief
(`📓️stdio-lane-brief.md`) is written and deliberately **unlaunched**; nothing of this ticket sits
under `🗄️stdio/**`. Resume that lane on their green signal, then re-derive the sub-lane table
from the settled roster.

Consequences for this ticket:
- Facet count grows **107 → 112** once the 5 new subsets land.
- `🧿️semio ✳️any` (the union dispatch, always migrated last) may become an 18-way union — its
  final variant list must be known before it is touched. Question posted to the peer.
- The peer was asked NOT to scaffold the 5 new subsets with `SetSnapshot`/`NoMutation` facets
  (fresh debt), and pointed at `📓️taxonomy.md` + `📓️fanout-brief.md`; offered fallback is an
  empty mutation enum with no triad dirs, which is cheap to populate later.
- Their in-flight subset renames currently leave `semio-s-plugin-stdio` red (6× `E0433`), and
  every plugin depends on stdio, so **all lane gates are blocked**. Lanes record `blocked-churn`
  and continue on unverified work; Wave V re-verifies everything.

### Resolved: `MutationMeta.group_id` (had blocked every crate)

The peer added `group_id: Option<String>` to `MutationMeta` (`📡️spr/🎮️command/🦀️component.rs:424`)
and has now filled in all six explicit struct literals in `🏪️store/🦀️component.rs`
(:1176, :1192, :1335, :1481, :2684, :3018 — this session caught it mid-edit at three). Deliberately
`Option<String>` rather than a newtype, to avoid inverting the os-kernel→semio-framework
dependency direction. Fixed by them, not by this ticket.

`MutationMeta` is shared: Wave 0 added `semantic_kind`/`label` (both still constructed `None`),
the peer added `group_id` (group-stamped edits are how composite gestures collapse into a single
undo step). The peer has no further changes planned for it. **Re-read this struct before the
final ratchet**, and notify the peer before touching it.

## Concurrency note

Agents share one cargo target lock, so per-crate `cargo check` serializes across lanes. Lanes are
capped at ~10 concurrent for this reason, and stdio sub-lane agents are instructed to run the
crate gate once at the end rather than per facet.
