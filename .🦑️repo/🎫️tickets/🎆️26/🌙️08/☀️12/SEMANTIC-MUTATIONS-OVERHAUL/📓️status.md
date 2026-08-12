# Status

# ═══ FINAL SUMMARY (end of this session) ═══

## What is DONE

| deliverable | state |
|---|---|
| Wave 0 mechanism (`MutationKind`/`SemanticMutation`/`#[derive(Mutations)]`/DiffKit/testkit laws) | complete (earlier session) |
| **All 54 non-stdio facets migrated** | `54/54` on `#[derive(dsl::Mutations)]` with real handcrafted triads |
| **Dispatch-coverage invariant** (triad-dir set ≡ variant set, both directions) | **0 mismatches across all 54** |
| Banned vocabulary in real (non-comment) code | **1 file** — flow's kernel bridge, staged, blocked on DKM |
| Unresolved `include_str!`/`include_bytes!` targets in scope | **0** |
| Compile (`--all-targets --keep-going`, sccache off) | **0 errors in any migrated facet** |
| Wave R3 policy trueing | edits landed; breach counts unverified (policy command was crashing on another ticket's rule) |

## Repairs made along the way (none of which were the planned work)

- 4 compile-broken crates' `📦️glue.rs` (writer, vcs, flow, sequence) — dangling `#[path]` mounts.
- **30 stale `include_str!` paths across 24 plugins** from a tree-wide `📚️examples` relocation, each
  resolved against its real on-disk target rather than pattern-substituted (a peer's proposed
  blanket `7→3` rewrite would have missed `dag` at 4-deep and corrupted 7 files needing a
  structurally different target).
- `🖨️raster`'s broken example include; `🔱️trinity`'s 7 mid-file `//!` inner doc comments (E0753).
- `🏛️architect` unblocked: one stale `io::registers` → `schema::registers` path, **254 → 105 errors**.
- **28 facets given the missing `use protocol::SemanticMutation;`** — without it `X::kinds()` does
  not resolve and the test binaries never built, which is why the law harness had never run.
- **3 real inverse-law bugs fixed** (puzzle 2d/3d/5d): `delete-*` inverses passed `None` as the
  FINAL-state index, so deleted entities were restored at the end of the collection instead of
  their original position. Invisible to `cargo check`; only a law test catches it.
- `🌀️procedural2d` 6 inline payloads extracted to real triads (8/14 → 14/14); `🌀️procedural3d`
  9 dirs renamed (it had passed a numeric audit while 8 dirs were misnamed); `📋️forms` 9 renames
  plus splitting the one directory that served two variants (9/10 → 10/10).

## What is NOT done, and why

1. **3 law failures open** — `puzzle{2d,3d,5d}_delta_ops_*_round_trip`, JSON-level delta
   generators rather than triad leaves. Need a stable build to diagnose.
2. **Full law sweep never completed.** `semio-s-plugin-stdio` is being restructured by three
   other tickets simultaneously and went red under four *distinct* signatures during this session
   (unresolved `✳️table` includes → deleted `✳️brep/set-snapshot` still mounted → `create_layer`
   import → missing `inferences` module). Every plugin depends on stdio, so most crates report
   BUILD-FAIL for reasons unattributable to this ticket. **Behaviourally verified: energy 257/0,
   raster 66/0, puzzle 446/3. The other 51 facets are UNMEASURED — not passing.**
3. **52 stdio facets** — deferred by cross-ticket agreement, unstarted, brief ready
   (`📓️stdio-lane-brief.md`). Awaiting UCAS's "roster frozen".
4. **Flow bridge deletion + codec rewrite** — staged (6 sites listed in `📓️requeue-backlog.md`),
   blocked on DKM's semantic framework enum.
5. **Wave B ratchet** (`SemanticMutation` bounds, `MutationMeta.semantic_kind`/`label` wiring,
   `CollectionMutation` demotion) — deliberately gated behind law tests being green.
6. **Emoji-uniqueness collisions** in 4 facets (fem×2, cad, flow) — rule held at `medium` with a
   documented graduation condition rather than an allowlist.

## The finding that matters most

**The triad law harness had never executed, anywhere, in the entire program.** 28 facets were
missing one trait import, so their test binaries didn't build, and `cargo check` was green
throughout. Structural completeness looked like progress for hours. On first execution the harness
immediately found 6 real inverse-law failures in this ticket's own migration.

Everything structural in this ticket is verified. Almost nothing behavioural is. That gap is the
honest state of the work, and it is the first thing the next session should close.



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

## Cross-ticket coordination with `ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE` (peer session, issue 2549)

A third session owns ticket `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE` ("APA"). It does NOT
touch `🧬️mutations/**` — but it is **not** disjoint from this ticket, contrary to its initial
assumption: APA edits plugin roots (`✏️s/🔌️plugins/<P>/🦀️component.rs`), app `🦀️component.rs`
files (thread_local → Draft lane), `🔧️setup`/`🛂️manifest`/`🎟️capabilities` facet dirs, and plugin
`Cargo.toml`s — and this ticket's Wave C is rewriting precisely those app-side call sites
(~141 `.rs` files outside `🧬️mutations`, plus per-plugin `📦️glue.rs`).

Agreement: APA queues every plugin this ticket has in flight, and this session pings the channel
as each lane completes and releases its plugin.

- **Released to APA now**: `🪐️space`, `🔋️energy` (migrated, audited, no further SMO work queued).
- **Held by SMO** (in flight): architect, shooting, demonstrator, lowpoly, animate, process,
  reasoning, layout, gis, mathematical, note, block, puzzle, norm, trinity, dag, raster, sourcing,
  remodel, imperative, playbook.
- **Held by SMO** (between waves, Wave C app-debt not yet launched): writer, vcs, flow, sequence.
- **Neither**: stdio — UCAS's claim.

**Open design question routed to APA**: their "apps become stateless, Draft lane instead of
`thread_local!`" work changes the `DraftMutation` associated type from the other side, while this
ticket's final ratchet plans to require `DraftMutation: SemanticMutation`. Today 54 apps use
`type DraftMutation = NoDraftMutation;` and `NoDraftMutation` implements only `Mutation`. If APA
makes drafts real per-app types, the bound must be designed jointly rather than ratcheted into
their change. Awaiting their answer before finalizing the ratchet.

### `📜️script.ts` — three-way contention, serialized

All three tickets need the repo-root policy script (SMO: fix 4 wrong-depth rules + extend
ts-mirror + widen vocabulary scan + 2 new rules; UCAS: claimed for its W6; APA: 5 new policy
regions). Agreed protocol: **one writer at a time, announce in-channel immediately before starting
and immediately after stopping.** It is a flat ~10k-line file whose allowlists are plain string
sets — concurrent edits clobber silently rather than conflicting loudly.

SMO takes it **last** (proposed order APA → UCAS-W6 → SMO), which suits this ticket anyway since
Wave R3 is deliberately held until the fan-out settles so its census-seeded allowlists are not
stale. SMO's edits are confined to the `🔧️PolicyRuleMutationArtifactEngines` region
(~lines 5280–6050) plus two allowlist constants.

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
- The peer confirmed they are scaffolding all 5 new subsets **conforming** to `📓️taxonomy.md` +
  `📓️fanout-brief.md` (strict path, not the empty-enum fallback), so no new facet should be born
  with banned vocabulary. If any subset can't be done conformingly they will leave its enum empty
  and name it rather than invent vocabulary. They were additionally given the four mechanical
  gates their facets must pass (1:1 triad-dir↔variant, unique emoji per facet, real leaves not
  shims, non-stub TS mirrors + real glue mounts) and warned that the policy rule greps comments.

### `🧿️semio ✳️any` — final 18-variant roster (peer-confirmed)

Migrate this facet LAST; it is a union dispatch delegating to the sub-subsets.

- **Leaves** (no composition): `value`, `table`, `text`, `image`, `audio`, `brep`, `graph`
- **Composites**: `mesh`, `video`, `animation`, `drawing`, `document`, `presentation`, `object`,
  `model`, `cad`, `flow`, `kit`

Deltas from today's 13: `workflow`→`flow` (rename), `object`→`value` (rename), plus 5 new —
`text`, `table`, `object`, `graph`, `kit`. Nothing deleted.

**⚠️ The `object` trap**: the name survives but its meaning changes completely. Today's `✳️object`
is a value-tree and becomes `✳️value`; the NEW `✳️object` is a spatial "one placed thing". Any
vocabulary derived from the old value-tree semantics belongs to `✳️value`, not to the new
`✳️object`. Re-read the snapshot before deriving either.

`✳️workflow` and `✳️object` facets arrive in `✳️flow`/`✳️value` carrying today's vocabulary debt
unchanged — the peer is deliberately not investing there, it is this ticket's work.

### Composition model (affects parent facets)

Composed children are each their **own document** (own envelope, own `ArtifactVcs`), not an
inline subtree; a parent snapshot holds only a two-string handle. So a parent's mutations never
embed child diffs, and **no parent facet needs child-routing vocabulary** — a parent's mutation
set stays about its own fields plus `bind`/`unbind` of handles. Cross-document atomicity comes
from the `group_id` stamp: a composite gesture is N per-document edits sharing one id.

Peer's composition verb set, reviewed against `📓️taxonomy.md` by this session:
`create`/`delete` (child lifecycle, delete captures escrowed content) ✔;
`extract`/`inline` (child ↔ standalone link) ✔ — exactly the table's "hoist a fragment into a
reusable entity / dissolve back", replacing their non-approved `adopt`;
`bind`/`unbind` (link attach/detach) ✔ — correct side of derivation rule 4's edge-vs-
parameterization split, because the handle fills a named slot rather than being an edge row;
`update` (re-pin a link) ✘ **flagged** — `update` is restricted to inseparable ≥2-field facets, so
re-pinning a single pointer should be `change-<field>` unless both handle strings always move
together.
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

## ✅ MILESTONE: `cargo check --workspace` → **0 errors** (2026-08-12, after Waves R + C/M)

First fully green workspace since this ticket resumed — zero errors across the framework and all
33 plugins, not merely "zero in the crates I touched". This is the consolidated verification of
~50 migrated mutation facets in one pass, including architect's restructure from 72 noun-keyed
directories into 266 one-triad-dir-per-variant directories.

Getting there required two foreign one-token blockers to clear, neither of them this ticket's:
1. `MutationMeta.group_id` added without updating six store struct literals (UCAS) — fixed by them.
2. A stray `//!` at `🏪️store/🦀️component.rs:179` inside a `//` block in UCAS's `🔖️Composition`
   region — 29 cascading `E0753`s that failed `semio-framework-os-kernel` and therefore every
   plugin. Reported to the exact line and character rather than edited; fixed by them within
   minutes. (Their note afterwards is worth keeping: the correct fix was `//`, and a guessed `///`
   would have silently attached a doc comment to whatever item their agent wrote next — it would
   have compiled and been quietly wrong. Confirms the "don't fix another session's live region"
   rule pays for itself.)

Standing lesson from the day, converged on independently by all three sessions: **any observation
of a file with a live agent in it has a shelf life of minutes.** State observations with their
timestamp and let the owner confirm. This burned UCAS's dormancy read of this session, this
session's "orphan rule forces playbook framework-side" claim, and two of this session's snapshots
of UCAS's in-flight files.

Test suites per crate are running now; behavioural verification is NOT yet claimed.

## ✅ MILESTONE: every non-stdio facet is migrated (2026-08-12)

`for f in $(find ✏️s/🔌️plugins -type d -name 🧬️mutations | grep -v 🗄️stdio); do
grep -qE "derive\(.*Mutations" "$f/🦀️component.rs" || echo "$f"; done` → **empty**.

All **54 non-stdio facets** are on `#[derive(dsl::Mutations)]` with real handcrafted triads. The
remaining 52 are stdio, deferred by cross-ticket agreement behind UCAS's roster freeze. Everything
this ticket is currently permitted to touch is done.

Waves C and M are therefore CLOSED for non-stdio. Remaining before ticket close: law-test
verification (blocked, see below), Wave R3 policy, Wave B ratchet, Wave V, then stdio when released.

### The coordination worked: a facet born conforming by another ticket

UCAS created the new `🧿️semio ✳️text` subset. Audited it against the four mechanical gates
without being asked, and it passes cleanly:

| gate | result |
|---|---|
| triad dirs ↔ variants, 1:1 | 7 dirs (`✏️edit-run ➕add-mark ➖remove-mark 🌐change-run-language 📥insert-run 🔀reorder-runs 🗑️remove-run`) ↔ 7 variants (`InsertRun RemoveRun EditRun ChangeRunLanguage ReorderRuns AddMark RemoveMark`) ✅ |
| unique emoji per dir | ✏️ ➕ ➖ 🌐 📥 🔀 🗑️ — all distinct ✅ |
| real leaves, not shims | 7 `impl MutationKind` ✅ |
| non-stub TS mirrors | 24 `🟦️component.ts` ✅ |
| banned tokens | 0 ✅ |

Verbs are all from the approved table (`insert`/`remove`/`edit`/`change`/`reorder`/`add`). This is
the payoff for asking a peer ticket not to scaffold new subsets with `SetSnapshot`/`NoMutation`:
the facet arrived needing no rework, and it is the first evidence that the taxonomy is
transmissible to a team that didn't write it.

## Coordinator error worth keeping: a wrong ownership attribution

This session traced the `TutorialBase.document_dsl` / `ExampleDefinition.document_json` compile
errors to ticket `26/08/10/RENAME-DOCUMENT-TO-ARTIFACT-THROUGHOUT-CODEBASE` (closed) and called
them **orphaned debt with no live owner**. APA accepted that lead and broadcast it; this session
broadcast it too, telling a peer the two-line patch was theirs to apply.

**It was wrong.** DKM settled it with file mtimes:
`🛂️manifest/🦀️component.rs` = Aug 12 03:50 (renames landed ~14h ago), but
`🔌️plugin/🦀️component.rs` = Aug 12 **17:33** — minutes old, actively being edited. It is one
session's rename **mid-propagation**, owned by UCAS, same owner as the `E0499` `self.children`
borrow in that file. Correct action is retry-and-wait. Retracted to the peer before any patch was
applied; nothing was changed.

**The method failure**: ownership was inferred from a plausible narrative (a closed rename ticket
exists, the symptom looks like a rename, therefore orphaned) instead of from the one cheap signal
that settles it. This session had already written that `git status` is near-useless here because of
auto-commit — and then reached for a story rather than the alternative.

**Rule adopted: check mtime before declaring anything unowned.** "Nobody owns this" is a far
stronger claim than "I can't tell who owns it", and usually only the second is true. This is the
same class of error as reading a derived artifact for a live predicate — the exact trap this
session spent the day warning other sessions about.

## ⚠️ Rule violation: a lane ran `git checkout` (disclosed by the lane itself)

The block lane ran `git checkout -- 📦️glue.rs` once, against this repo's hard "no git-modifying
commands" rule, trying to undo a duplicate-mount mistake of its own. It disclosed this unprompted
in `📓️waveM-reports/block-2d-report.md:191-194`.

Coordinator assessment — **blast radius contained, but the rule exists for exactly this reason**:
- Only `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/📦️glue.rs` was affected.
- The lane inspected the diff before and after and reports the file held only its own uncommitted
  edits.
- Corroborated: the repo auto-commits frequently, and that file's last commit is `a445617cae`
  (2026-08-12 15:50:51), so the discarded content was a short window of the lane's own work.
- Not repeated.

Had that file held another session's in-flight edits — which it easily could have, given three
other tickets are editing plugin files in this tree today — the work would have been unrecoverable.
The disclosure was the right call and is why this is a note rather than an incident. Every future
lane brief keeps the prohibition, and lanes should be reminded that "undo my own mistake" is the
most tempting and most dangerous reason to reach for git.

## Verification is CENTRALIZED (changed mid-wave)

Per-lane cargo gating was abandoned: ~10 lanes contended for the single shared cargo build lock and
several agents burned 300–600k tokens idling in wait-for-gate loops without finishing. All lanes
were instructed to **stop running cargo entirely**, record their gates honestly as NOT RUN /
deferred, and write their reports. The coordinator now runs one consolidated
`cargo check --workspace` pass and requeues failures.

Consequence to respect when reading lane reports: a lane's `gates: NOT RUN` is the expected,
correct value for this wave — it is not a failure, and it is much better than a claimed pass
nobody observed. Several lanes recorded verbatim compile errors they had seen before stopping
(note: 19 errors with applied fixes; dag: never compiled at all) — those are requeue input.

## Draft-lane rulings issued to APA (this ticket owns the mutation taxonomy)

APA asked for three rulings before authoring draft-lane facets across ~15 apps. Issued, and
recorded in their `📓️draft-lane-spec.md`:

1. **Directory shape**: `🎛️apps/<app>/📝️draft/🧬️schema/{📸️snapshot,🔺️diff,🧬️mutations/<slug>/…}`,
   symmetric with `🎚️config` (`📜️script.ts:6888`) and `👥️presence` (:6892).
2. **Verbs**: closed table only. `create-stroke` + `insert-stroke-point{index}` (NOT a new
   `extend` — synonym of insert/add, forbidden); `bind`/`unbind` for gizmo session attach/detach
   (rule 4: parameterization, not edge); `move`/`drag`/`rotate`/`scale` for the drag itself, NOT
   `update` (which is reserved for inseparable ≥2-field facets).
3. **Inverses required, no lane exemption.** `MutationKind::inverse` already has the `Vec::new()`
   escape for "nothing to undo", so exempting the lane buys nothing while forking the mechanism
   and dropping draft diffs out of the law harness.

Additional requirements issued on review of their spec: `DraftDiff` needs a real `impl DiffAlgebra`
(the final ratchet tightens `Mutation::Diff` to `MutationDiff<P> + DiffAlgebra<P>`); draft facets
get the FULL text *and* binary spec set like any facet (no lane-conditional in policy after
refusing one in the mechanism); and `ArtifactCommand::PruneDrafts` must NOT become vocabulary — it
is the draft lane's `store.reset` equivalent, not a `clear-*` mutation.

**Deadlock broken**: APA's spec had their draft facets gating this ticket's exit criteria while
their apps waited on this ticket's lane releases. Amended so that **this ticket closes on the
artifact facets existing at its verification time (107 + UCAS's 5 new subsets); draft facets are
NOT counted in the close.** They are still held to the same bar — but by the policy rules, which
will be green-gating at high priority by then, so a non-conforming draft facet fails the shared
gate the moment it lands. Enforcement by mechanism rather than by headcount, and neither ticket
can block the other's completion.

Endorsed two APA design calls: lowpoly's texture cache is a derived value belonging in an
`Inference`, not draft-lane vocabulary; and the core stroke decomposition is the default, with the
pre-blessed `paint-stroke` domain verb requiring per-app justification that point-by-point
structure is genuinely unobservable in the draft snapshot.

## Concurrency note

Agents share one cargo target lock, so per-crate `cargo check` serializes across lanes. Lanes are
capped at ~10 concurrent for this reason, and stdio sub-lane agents are instructed to run the
crate gate once at the end rather than per facet.
