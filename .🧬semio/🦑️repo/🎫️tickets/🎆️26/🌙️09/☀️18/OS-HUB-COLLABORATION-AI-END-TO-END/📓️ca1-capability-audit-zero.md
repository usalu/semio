# CA1 — `capability-audit-check` to zero findings

Slice CA1 of ticket 26/09/18, session 8 (2026-09-22 11:05 →). Outcome 4.
Inherited: `📓️ce2-mcp-gates-green.md` §2/§5, `📓️a3b-descriptor-sweep.md` §5,
`📓️ce1-client-e2e-pinning-and-puzzle-bound.md` §6, `📓️status.md` session 7/8 tail.

## 0. Headline

| | before (11:05) | after |
| --- | ---: | ---: |
| `semio-os-mcp audit` findings | **29** over 59 descriptors | **1** over **60** (§5) |
| catalog diagnostics | 1 (`puzzle` 🔣️.json does not decode) | **0** — `🧩️puzzle` decodes for the first time in this ticket |
| plugin source declarations added | — | **27 lines over 19 files** (§2, §2c) |
| plugins re-described | — | 8 by this slice's batch, 5 more by peers after its edits |

Instrument: the staged gateway `🌉️mcp/📦️packages/🦀️rust/dist/build/semio-os-mcp audit --folder
<repo>` (11:04 on 09-21, the same binary CE2/A3b/CE1 measured with — `capability-audit-check` runs
exactly this). Captures `🗑️generated/ca1-audit-before.txt`, `…/ca1-audit-after.txt`.

## 1. The 29 findings, classified

All 29 are `UnmarkedDestructiveVerb` (25) or `UndeclaredGestureRoute` (4), the two variants
`🌉️mcp/🗂️catalog/🦀️.rs` defines. Both are **declaration** defects: the lexicons
(`DESTRUCTIVE_VERB_WORDS`, `GESTURE_ROUTE_WORDS`, `🗂️catalog/🦀️.rs:958/1007`) only ask the
question; `ActionDefinition::destructive()` / `AppBuilder::action_destructive(id)` /
`action_audience(id, Input)` answer it. Each reaches the gate only through a committed descriptor,
so every fix is **source edit + re-describe** — no `🔣️.json` was hand-edited.

Three of the 29 needed no source edit at all, only a re-describe: their source already declares the
fact and the committed descriptor predates the declaration.

| # | finding | owner source | fix |
| --- | --- | --- | --- |
| 1 | `architect…program#editor.setActiveExample` | `🏛️architect/🗿️artifacts/🏛️program/…/✏️editor/🦀️.rs:1606` | `.action_destructive` |
| 2 | `demonstrator…gis.gismap#editor.deleteFeature` | gis' gismap editor (demonstrator reuses the artifact crate) | `.action_destructive` + re-describe demonstrator |
| 3–6 | `demonstrator…puzzle.puzzle3d#editor.{engagementAbort,engagementInput,engagementSubmit,worldPointerDown}` | `🧩️puzzle/🗿️artifacts/🧊️3d/…/✏️editor/🦀️.rs:8488/8526/8527/8546` | `.action_audience(…, Input)` ×4 + re-describe demonstrator |
| 7–9 | `energy…model#editor.{delete-surface,delete-zone,setActiveExample}` | `🔋️energy/🗿️artifacts/🔋️model/…/✏️editor/🦀️.rs` | new `ENERGY_MODEL_DESTRUCTIVE_ACTION_IDS` roster |
| 10 | `gis…gismap#editor.setActiveExample` | already `.action_destructive` at `:1229` | **re-describe only** (descriptor 09-21 14:56) |
| 11 | `layout…layout#editor.deleteSelection` | `📏️layout/…/✏️editor/🦀️.rs:1406` | `.action_destructive` |
| 12 | `lowpoly…lowpoly#editor.deleteSelection` | `💠️lowpoly/…/✏️editor/🦀️.rs:2263` | `.action_destructive` |
| 13 | `mathematical…equation#editor.setActiveExample` | `➗️mathematical/…/✏️editor/🦀️.rs:1503` | `.action_destructive` |
| 14 | `norm…din4108#editor.setSnapshot` | `📕️norm/🗿️artifacts/🧱️din4108/…/✏️editor/🦀️.rs:249` | `.action_destructive` (the other 13 norm artifacts already carry it at `:224`) |
| 15–16 | `playbook…playbook#editor.{removeBlock,removeStep}` | `📖️playbook/…/✏️editor/🦀️.rs:689/692` | `.action_destructive` ×2 |
| 17 | `sequence…sequence#editor.setActiveExample` | `🎬️sequence/…/✏️editor/🦀️.rs:3676` | `.action_destructive` |
| 18 | `trinity…rewriting#editor.setActiveExample` | `🔱️trinity/🗿️artifacts/♻️rewriting/…/✏️editor/🦀️.rs:1067` | `.action_destructive` |
| 19 | `vcs…vcs#editor.setActiveExample` | `🌿️vcs/…/✏️editor/🦀️.rs:1091` | `.action_destructive` |
| 20 | `wfc…bitmap#editor.remove-palette-color` | `🀄️wfc/🗿️artifacts/🖼️bitmap/…/✏️editor/🦀️.rs:983` | `.action_destructive` (the id is a WINDOW action, `🪟️windows/🖼️input/🦀️.rs:84`) |
| 21–23 | `wfc…grid2d#editor.{delete-rule,delete-tile,setActiveExample}` | `🀄️wfc/🗿️artifacts/🔲️grid2d/…/✏️editor/🦀️.rs:834` | 3 × `.action_destructive` before `build_definition()` |
| 24–26 | `wfc…grid3d#editor.{deleteRule,deleteTile,setActiveExample}` | `🀄️wfc/🗿️artifacts/🧱️grid3d/…/✏️editor/🦀️.rs:747` | 3 × `.action_destructive` |
| 27–28 | `wfc…wfc2d#editor.{delete-slot,setActiveExample}` | `🀄️wfc/🗿️artifacts/◻️2d/…/✏️editor/🦀️.rs:1099` | 2 × `.action_destructive` |
| 29 | `wfc…wfc3d#editor.delete-slot` | `🀄️wfc/🗿️artifacts/🧊️3d/…/✏️editor/🦀️.rs:1045` | `.action_destructive` |

Two structural facts that decide where a fix goes, both verified in
`🔌️plugin/🦀️.rs:5144/5172` rather than assumed:

1. **`action_destructive` / `action_audience` reach every surface an id can be declared on** — bare
   actions, **window-kind actions**, bare commands and mode commands. That is why the wfc grid/slot
   verbs, which are declared inside `🎭️modes/✏️edit/🪟️windows/**`, are fixable from the editor's
   own builder chain, and why the call must sit AFTER the `window_kind_def` that carries them.
2. **`demonstrator` declares none of its flagged verbs.** Its `Cargo.toml` composes the real artifact
   crates (`semio-s-artifact-gis-gismap`, `…-puzzle-3d`, `app = "s.puzzle.puzzle3d@1/*#editor"`), so
   its descriptor is a UNION and the two owners' sources are the root. `demonstrator`'s descriptor
   (09-21 18:19) is older than gis' and puzzle's source, which is exactly why the SAME kind shows a
   different finding in the two descriptors (`deleteFeature` in demonstrator's copy of gismap,
   `setActiveExample` in gis' own).

## 2. Source fixes

`🐍️ca1-declare.py` (ticket folder, new) — an anchored insertion codemod, refusing to write unless
every anchor line carries its expected fragment, skipping an id already declared, applying each
file's insertions in descending line order. **19 insertions over 14 files**, plus 6 hand-written
lines in 3 files the codemod could not express:

| file | lines added |
| --- | --- |
| `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/…/✏️editor/🦀️.rs` | `ENERGY_MODEL_DESTRUCTIVE_ACTION_IDS` (`:118`) + the loop that applies it (`:2585`) — the two inspector deletes only join the builder inside `app_level_action_definitions()`'s loop, so the declaration has to run after it |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/…/✏️editor/🦀️.rs` | 3 lines before `build_definition()` |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/…/✏️editor/🦀️.rs` | 3 lines before `build_definition()` |

Not touched, deliberately: `✏️s/🔌️plugins/🌊️flow/**` (a peer session's topic — it carries **no**
finding in this run either, so nothing is owed there).

### 2b. Will a re-describe surface NEW findings?

The twelve plugins queued for a re-describe have descriptors between 09-20 07:21 (`📕️norm`, 409
newer `🦀️.rs` files) and 09-22 11:01 (`🏛️architect`), so a fresh descriptor can expose declaration
debt that the stale one hid. `🐍️ca1-source-lexicon-scan.py` (ticket folder, new) re-implements the
gateway's own `verb_id_words`/`matching_lexicon_word` over the ids the source declares and reports
the unmarked lexicon matches: capture `🗑️generated/ca1-source-lexicon-scan.txt`, **one** hit —
`playbook.setActiveExample`, declared `ActionKind::View` (`📖️playbook/…/✏️editor/🦀️.rs:709`), and
`audit_declaration` only flags `ActionKind::Mutation`, so it is not a finding.
**The scan is a weak predictor and is reported as one** (§7 gap 2): it sees only literal
`.mutation("…")` / `ActionDefinition::{new,new_catalog,bounded_catalog}("…")` /
`.view_action`/`.shell_action` forms, so it counts 0 ids for `🔋️energy` and 1 for `🎪️demonstrator`,
both of which declare through helpers and consts.

### 2c. The latent `🧩️puzzle` set — 8 more declarations, no finding today

Running the same scan PER ARTIFACT rather than per plugin un-masks an id a SIBLING artifact of the
same plugin already declares, and that turned up eight more undeclared routes in `🧩️puzzle`'s other
two artifacts. They carry **no finding in `ca1-audit-before.txt`** only because `🧩️puzzle`'s
`🔣️.json` does not decode at all — they become findings the moment CE3's describe of it succeeds,
so they are declared now (`🐍️ca1-declare.py latent`, capture
`🗑️generated/ca1-source-lexicon-per-artifact.txt`):

| artifact | id | declaration |
| --- | --- | --- |
| `◻️2d` | `deleteEdge` (`:5340`), `deleteTargetRegion` (`:5376`) | `.action_destructive` |
| `◻️2d` | `engagementInput` (`:5356`), `engagementSubmit` (`:5362`), `engagementAbort` (`:5363`) | `.action_audience(…, Input)` |
| `🖐️5d` | `engagementSubmit` (`:9839`), `engagementInput` (`:9869`), `engagementAbort` (`:9870`) | `.action_audience(…, Input)` |

`🖐️5d`'s `worldPointerDown` was already declared; `◻️2d` has no such route. `🀄️wfc` scanned clean
per artifact after §2's edits.

**Why only some of these would ever have been findings**, checked against
`🛂️manifest/🦀️.rs:780` rather than guessed: `derive_audience` answers `Chrome` for a `View` action
that is out of the palette and `Agent` for everything else, and `audit_declaration` returns
immediately for a non-`Agent` audience. So `◻️2d`'s `engagementInput`/`engagementAbort`
(`in_palette: false`, `View`) were already `Chrome` and silent — declaring them `Input` only replaces
a derived "window chrome" with the truth. The other six (`Mutation`, or `View` still in the palette)
derive `Agent` and are real latent findings. The same rule is why
`🏭️process`'s `process3d.engagementInput`/`engagementAbort` — the only other unmarked routes under
any artifact `🎪️demonstrator` composes (`📐️cad`, `🌀️procedural`, `🪵️sourcing` scan clean;
capture `🗑️generated/ca1-source-lexicon-demonstrator-children.txt`) — carry no finding and are
**not** touched by this slice: both are `View` with `in_palette: false`, and the committed
demonstrator descriptor confirms it (`audience=None`, unflagged, while `puzzle3d`'s palette-visible
twins are flagged).


## 3. Native type-checks

ONE cargo, all seventeen edited artifact crates selected by `-p`, never workspace-wide, in a private
`CARGO_TARGET_DIR` over the SHARED build dir, `CARGO_INCREMENTAL=0` (preamble rules 7/25/30):

```sh
CARGO_INCREMENTAL=0 CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/⚡️cache/cargo/target-ca1" \
  cargo check -p semio-s-artifact-architect-program … -p semio-s-artifact-puzzle-3d \
  --features "semio-s-artifact-gis-gismap/component-app-assembly …" -j 4
```

Nine of the seventeen have **no** `component-app-assembly` feature (architect-program,
energy-model, layout-layout, lowpoly-lowpoly, mathematical-equation, norm-din4108,
playbook-playbook, sequence-sequence, vcs-vcs) — their editor compiles by default — so the feature
is passed per package for the eight that do, which is why a flat `--features component-app-assembly`
is refused here.

**Result: `Finished dev profile [unoptimized] target(s) in 15m 06s`, rc 0, 0 errors** at load ≈ 110
(capture `🗑️generated/ca1-check.txt`). Every one of the seventeen appears in the run's own
`Checking semio-s-artifact-…` lines, and warnings are the proof the crates really type-checked
(memory law "Require Warnings As Proof Of Type-Check"):

| crate | warnings |
| --- | ---: |
| `…-puzzle-3d` | 103 |
| `…-wfc-3d` | 18 |
| `…-sequence-sequence` | 5 |
| `…-architect-program` | 4 |
| `…-wfc-2d` | 4 |
| `…-mathematical-equation` | 3 |
| `…-gis-gismap` | 2 |
| `…-energy-model`, `…-layout-layout`, `…-trinity-rewriting`, `…-wfc-grid2d`, `…-wfc-grid3d` | 1 each |
| `…-lowpoly-lowpoly`, `…-playbook-playbook`, `…-vcs-vcs`, `…-norm-din4108`, `…-wfc-bitmap` | 0 |

A first attempt at 11:22 died in a PEER's file — `semio-framework-plugin` refused
`E0027: pattern does not mention field tasks` at `🔌️plugin/🦀️.rs:25311`, FP10's async-task lane
landing mid-flight. Nothing of this slice's; the peer's edit completed and the rerun is the row
above. That is the whole of the interference this slice saw.

§2c's two extra crates were checked separately once they were edited —
`cargo check -p semio-s-artifact-puzzle-2d -p semio-s-artifact-puzzle-5d --features "…/component-app-assembly"`
→ **rc 0, 0 errors, 13.30 s** warm (5 and 3 warnings respectively),
capture `🗑️generated/ca1-check-puzzle.txt`.

## 4. Re-describe batch

A source declaration reaches `semio-os-mcp audit` only through the committed descriptor its own
producer writes, so the twelve plugins §2 edited (plus `🌍️gis`, whose source was already right and
whose descriptor was merely stale) are re-described in ONE fleet-mutex hold:

```sh
zsh "📜️mutex-ordered.sh" 20260922110400 ca1 -- zsh "📜️ca1-describe-batch.sh"
```

`📜️ca1-describe-batch.sh` (ticket folder, new; `📜️jb1-describe-batch.sh`'s shape) exports
`CARGO_PROFILE_WASM_DEV_DEBUG=false`, `CARGO_INCREMENTAL=0`, `NX_DAEMON=false`, runs each owner's own
`bun ./📜️script.ts describe` **serially**, and appends a row per owner to
`🗑️generated/ca1-describe-ledger.txt` (`rc=… <s> json <old> -> <new> pack <old> -> <new>`), with a
per-owner capture `🗑️generated/ca1-describe-<owner>.txt`. Owners are ordered cheapest-descriptor
first so a cut leaves the most rows landed:

`➗️mathematical → 📖️playbook → 📏️layout → 🎬️sequence → 💠️lowpoly → 🏛️architect → 🔱️trinity →
🔋️energy → 🌿️vcs → 🌍️gis → 📕️norm → 🎪️demonstrator`.

`🀄️wfc` and `🧩️puzzle` are **deliberately not in the batch** — CE3 owns those two describes this
session (queue stamp `…110300`, ahead of this slice's `…110400`), and both their sources carry this
slice's declarations already, so whichever of the two runs picks them up.

**Queue position, measured:** queued at 12:00:51 behind `c8` (holding since 11:32:57, a live
`trusted-catalog-bootstrap --packages stdio,gis`), `s11`, `tc3e` and the peer `play` — fifth.

**What actually ran.** The account session limit cut the fleet at ~13:15 (reset 15:50); the wrapper
survived, took the mutex at **15:47:28** and described **7 of 12** before a second event ended it:

| owner | rc | wall | `🔣️.json` before → after |
| --- | --- | ---: | ---: |
| `➗️mathematical` | 0 | 27 s | 89 199 → 89 199 |
| `📖️playbook` | 0 | 41 s | 115 388 → 115 386 |
| `📏️layout` | 0 | 28 s | 164 007 → 164 007 |
| `🎬️sequence` | 0 | 33 s | 169 650 → 169 650 |
| `💠️lowpoly` | 0 | 38 s | 303 211 → 303 211 |
| `🏛️architect` | 0 | 72 s | 394 681 → 394 680 |
| `🔱️trinity` | 0 | 64 s | 475 697 → 475 697 |
| `🔋️energy` | 0 | 283 s | 478 140 → 478 140 |

(the byte deltas are ≈ 0 because `"destructive":false` → `"destructive":true` is one byte shorter and
`"approval":"never"` → `"whenDestructive"` is longer — the descriptor CONTENT is what moved, checked
field by field: e.g. `💠️lowpoly`'s `deleteSelection` lives at
`/manifest/apps[0]/windowKinds[0]/actions[30]` and now reads `destructive=true`,
`approval=whenDestructive`.)

`🌿️vcs` was next and ran for **24 minutes** at ≈ 89 k fuel/s without finishing — past the old
1 800 s wall, which is CE2's `OwnedDeadline::NoFuelProgress` (`🔌️plugin/🖥️host/🦀️.rs:1493/1510`)
working: a starved-but-progressing describe is no longer killed by the clock. At **~16:30** a
machine-wide disk prune (11 → 114 GiB free) emptied `🗑️generated` and the batch process died
(exit 1), so **this slice's own ledger and per-owner captures for that run are gone** — the
descriptors on disk and their mtimes are the surviving evidence, and they are what §5 measures.

`🌿️vcs`, `🌍️gis` and `🎪️demonstrator` did not need the batch after all: **peers re-described them
at 15:03, 14:40 and 15:10** — after this slice's 11:25 source edits — so they carry the fixes
already, verified field by field (`gis.deleteFeature` `destructive=true`,
`demonstrator`'s embedded `s.puzzle.puzzle3d@1/*#editor` routes `audience=input`). `🀄️wfc` and
`🧩️puzzle` likewise landed through CE3's own describes, and `🧩️puzzle`'s descriptor **decodes for
the first time**, which is what takes the catalog diagnostic to 0 and the descriptor count 59 → 60.

Only `📕️norm` (descriptor still 09-20 07:21) remained, re-queued alone:
`zsh 📜️mutex-ordered.sh 20260922163600 ca1 -- zsh 📜️ca1-describe-batch.sh 📕️norm`
(the script now takes an owner list; capture `🗑️generated/ca1-describe-norm.txt`).

## 5. After — **29 findings → 1**, and the catalog diagnostic is gone

`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/dist/build/semio-os-mcp audit
--folder <repo>`, 17:00, capture `🗑️generated/ca1-audit-after.txt`:

```
norm.s.norm.din4108@1/*#editor.setSnapshot is a `setsnapshot`-class mutation published to agents
  with effects.destructive = false — WhenDestructive never fires
semio-os-mcp audit: 1 finding(s) over 60 descriptor(s) under /Users/ueli/Documents/semio
```

| | before (11:05) | after (17:00) |
| --- | ---: | ---: |
| findings | **29** | **1** |
| descriptors inspected | 59 | **60** |
| `[mcp registry] skipping plugin …` diagnostics | 1 (`puzzle`) | **0** |
| `UnmarkedDestructiveVerb` | 25 | 1 |
| `UndeclaredGestureRoute` | 4 | **0** |

Spot-checked field by field rather than trusted to the tally — every one of the 28 cleared rows now
reads `destructive=true, approval=whenDestructive` (or `audience=input`) in its committed
descriptor, including the ones that live under `windowKinds` rather than `actions`
(`💠️lowpoly` `/manifest/apps[0]/windowKinds[0]/actions[30]`, `🀄️wfc`'s two `delete-slot` window
rows) and the composed ones (`🎪️demonstrator`'s `s.puzzle.puzzle3d@1/*#editor` routes are
`audience=input`; its `s.gis.gismap@1/*#editor.deleteFeature` is `destructive=true`).

### 5.1 The one survivor: `📕️norm` cannot be described — its dev component is 2 % over the bound

Not a declaration defect. `📕️norm`'s source fix is landed and type-checked (§2, §3); its describe
fails **before** the guest ever runs:

```
Finished `wasm-dev` profile [unoptimized] target(s) in 5m 25s
describe semio-s-plugin-norm failed: raw component …/wasm32-wasip2/wasm-dev/semio_s_plugin_norm.wasm
  must be a regular non-symlink file of 1..268435456 bytes
```

Measured: the component is **273 934 765 B**, i.e. **5.2 MiB (2.0 %) over**
`FRESH_COMPONENT_MAX_BYTES = 256 * 1024 * 1024`
(`🔌️plugin/🖨️describe/🏗️component-build/🟦️.ts:14`), whose own docstring tracks the previous
high-water marks — *"The GIS plugin's dev component is ~193 MiB; procedural ~80 MiB."* `📕️norm`'s
16 standards artifacts have passed both. Ledger row: `📕️norm rc=1 334s json 1213835 -> 1213835`.

**It is not debug info.** `CARGO_PROFILE_WASM_DEV_DEBUG=false` was exported and did apply — a scan
of the artifact finds **no `.debug_*` section name at all**. The 261 MiB is unoptimized code.

**This slice deliberately did not raise the constant.** It is a build-time admission bound in
`🔌️plugin` (FP9/FP10's module this session), it is doing exactly its job — noticing that a plugin
crossed a line it did not cross on 09-20, when `📕️norm` last described successfully — and raising a
bound to make a gate green is the move CE2's §5 report argues against. Turning `📕️norm` green needs
its dev component under 256 MiB or the bound's owner moving it **with this measurement in hand**;
either way the declaration is already in the source, so one describe closes it.

## 6. Files changed

| file | change |
| --- | --- |
| `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/…/✏️editor/🦀️.rs` | `.action_destructive("setActiveExample")` |
| `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/…/✏️editor/🦀️.rs` | `.action_destructive("deleteFeature")` |
| `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/…/✏️editor/🦀️.rs` | `.action_destructive("deleteSelection")` |
| `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/…/✏️editor/🦀️.rs` | `.action_destructive("deleteSelection")` |
| `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/…/✏️editor/🦀️.rs` | `.action_destructive("removeStep"/"removeBlock")` |
| `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/…/✏️editor/🦀️.rs` | `.action_destructive("setActiveExample")` |
| `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/…/✏️editor/🦀️.rs` | `.action_destructive("setActiveExample")` |
| `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/…/✏️editor/🦀️.rs` | `.action_destructive("setActiveExample")` |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/…/✏️editor/🦀️.rs` | `.action_destructive("setActiveExample")` |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/…/✏️editor/🦀️.rs` | `.action_destructive("setSnapshot")` — the 13 sibling norms already had it |
| `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/…/✏️editor/🦀️.rs` | new `ENERGY_MODEL_DESTRUCTIVE_ACTION_IDS` roster + the loop that applies it after `app_level_action_definitions()` |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/…/✏️editor/🦀️.rs` | `.action_destructive("delete-slot")`, `.action_destructive(WFC_2D_SET_ACTIVE_EXAMPLE)` |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/…/✏️editor/🦀️.rs` | `.action_destructive("delete-slot")` |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/…/✏️editor/🦀️.rs` | `.action_destructive("remove-palette-color")` |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/…/✏️editor/🦀️.rs` | 3 × `.action_destructive` before `build_definition()` |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/…/✏️editor/🦀️.rs` | 3 × `.action_destructive` before `build_definition()` |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/…/✏️editor/🦀️.rs` | 4 × `.action_audience(…, Input)` |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/…/✏️editor/🦀️.rs` | 3 × `.action_audience(…, Input)`, 2 × `.action_destructive` (§2c) |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/…/✏️editor/🦀️.rs` | 3 × `.action_audience(…, Input)` (§2c) |

Ticket files (new): `🐍️ca1-declare.py`, `🐍️ca1-source-lexicon-scan.py`, `📜️ca1-describe-batch.sh`
(takes an optional owner list), this report. Captures that survive the 16:30 prune (§7 gap 3): `🗑️generated/ca1-{audit-before (re-materialised,
says so in line 1), audit-mid, audit-after, describe-ledger, describe-norm, describe-📕️norm,
describe-norm-pid}.txt`. The prune also deleted this slice's `ca1-{check,check-puzzle,
source-lexicon-scan,source-lexicon-per-artifact,source-lexicon-demonstrator-children,
describe-batch,describe-<owner>}.txt`; §3's numbers and §2b/§2c's rows were read off them while they
existed and are quoted verbatim here, and every source edit and descriptor they justify is on disk.

**No committed `🔣️.json` or `🛂️.descriptor.semio` was hand-edited** — every descriptor change in
this slice is producer output from `bun ./📜️script.ts describe`.

Nothing under `✏️s/🔌️plugins/🌊️flow/**` was read or written (a peer session's topic); it carries no
finding in `ca1-audit-before.txt` either, so nothing is owed there.

## 7. Honest gaps

1. **The goal was 0 findings; the tree is at 1.** `📕️norm.din4108.setSnapshot` (§5.1). The
   declaration is in the source and type-checked; the describe cannot run because norm's `wasm-dev`
   component is 273 934 765 B against a 268 435 456 B admission bound. Located exactly, not fixed —
   and deliberately not fixed by raising the constant.
2. **The source lexicon scan is a weak predictor** (§2b). It matches only literal
   `.mutation("…")` / `ActionDefinition::{new,new_catalog,bounded_catalog}("…")` /
   `.view_action` / `.shell_action` declarations, so it sees 0 ids for `🔋️energy` and 1 for
   `🎪️demonstrator`, both of which declare through helpers and consts — it missed `◻️2d`'s
   `engagementSubmit` (declared through `puzzle2d_internal_action`), which was added by hand after
   reading the chain. The gate remains `bun ./📜️script.ts capability-audit-check`.
3. **This slice's own describe captures for the 15:47 batch are gone.** A machine-wide disk prune at
   ~16:30 (11 → 114 GiB free) emptied `🗑️generated` and killed the batch process; the eight ledger
   rows in §4 were read off the ledger before it vanished and are reproduced there verbatim, and the
   descriptors + their mtimes on disk are the surviving primary evidence.
   `🗑️generated/ca1-audit-before.txt` was re-materialised from the session transcript and says so in
   its own first line; `ca1-audit-after.txt` is a live capture.
4. **Five of the thirteen plugins were re-described by PEERS, not by this batch** (`🌿️vcs`,
   `🌍️gis`, `🎪️demonstrator` at 15:03/14:40/15:10, `🀄️wfc` and `🧩️puzzle` through CE3). Their
   descriptors carry this slice's declarations because those runs came after the 11:25 source edits,
   which was verified per capability rather than assumed — but this slice did not run them.
5. **Nothing here was measured against a live shell or a live MCP client.** The instrument is the
   staged `semio-os-mcp audit` binary (11:04 on 09-21) over committed descriptors, the same one
   `capability-audit-check` invokes. `client-e2e`, `live-agent-loop-check` and
   `hub-agent-participant-check` were not re-run by this slice.
6. **`🌿️vcs` describes very slowly.** The batch's run reached 24 minutes at ≈ 89 k fuel/s without
   finishing (it was superseded by the peer's 15:03 descriptor, so no work was lost). That is the
   starvation CE2 §5 measured, now bounded by no-fuel-progress rather than the wall clock — worth
   knowing before anyone budgets a `🌿️vcs` describe under fleet load.
7. **Two mutex wrappers of this slice died before running** — the first survived four and a half
   hours and the session-limit cut, then died with the disk prune; the second was killed ~13 minutes
   in. The third (`nohup … & disown`, pid recorded in `🗑️generated/ca1-describe-norm-pid.txt`,
   preamble rule 8's form) ran to completion and released the mutex cleanly. No stale
   `/tmp/semio-wasm-build.lock` or queue ticket of this slice remains.
