# Plugin release status — SMO (`26/08/12/SEMANTIC-MUTATIONS-OVERHAUL`, #2545)

**This file is the live predicate.** Other tickets (APA #2549, UCAS #2548) need to know "is plugin
P free for me to edit". Read it here rather than inferring it from report files, directory
contents, or agent activity — all three of those are derived artifacts that have already misled
each of the three sessions at least once today.

Updated: 2026-08-12, after `cargo check --workspace` → 0 errors.

**RELEASED** = this ticket has finished with the plugin: its mutation facet(s) are migrated, its
in-plugin mutation call sites are rewritten, and the workspace compiles. Take it freely.
**HELD** = a lane is mid-flight or not yet launched; editing it will collide.

> ⚠️ **ABSENCE FROM THIS FILE MEANS FREE, NOT HELD.** This ledger lists only plugins SMO has
> actually had a lane on. A plugin named in neither list was never claimed by this ticket and needs
> no clearance from it — proceed. Only an explicit entry under **HELD** blocks you.
>
> This wording exists because the omission had a real cost: five APA agents read "not in RELEASED"
> as "held" and skipped `📐️cad`, `🏗️fem`, `🖍️draw`, `🌀️procedural` and `📋️forms`, none of which
> were ever anyone's to hold. A ledger that is silent about its own default is a derived artifact
> pretending to be a predicate — the same trap that has caught every session in this tree.

## RELEASED

| plugin | facets | evidence |
|---|---|---|
| `🪐️space` | `🏠️home` | `🔢️change-catalog-generation` triad; banned variants gone; leaf audited by hand (inverse reconstructs from `base`) |
| `🔋️energy` | `🔋️model` | `♻️replace-model` triad; leaf audited by hand (sparse diff, siblings untouched) |
| `🖨️raster` | `🖨️raster` | 12 triads; `PatchLayer` option-bag split; **`cargo test` 66 passed / 0 failed** |
| `🕸️dag` | `🕸️dag` | 14 triads; generic collection wraps + whole-collection setters decomposed |
| `🪵️sourcing` | `🗂️curate` | 3 triads on `curated`; `stock` deliberately has no vocabulary (bulk-seeded catalogue, no per-item editor) |
| `🗒️note` | `🗒️note` | migrated; 19 compile errors were fixed blind and are now confirmed by the green workspace |
| `🧩️puzzle` | `◻2d`, `🖐️5d`, `🧊️3d` | 26 / 28 / 35 mutations; 11 kebab-slug mismatches and 3 emoji collisions found and fixed |

## RELEASED — lane finished, compiles in the workspace check

| plugin | facets | evidence |
|---|---|---|
| `🔱️trinity` | `🔌️jack`, `♻️rewrite` | jack: 10 triads, `SetFixture` gone. rewrite: `SetState` (a whole-doc replace in disguise) deleted, 7 field-level mutations |
| `🧱️block` | `◻2d`, `🧊️3d`, `🖐️5d` | 26 / 37 / 41 mutations; triad-dir↔variant counts verified 1:1 on disk |
| `📜️imperative` | `📜️imperative` | struct-with-`CollectionMutation` replaced by a 4-variant enum; 8 app handlers rewired |
| `📖️playbook` | `📖️playbook` | vocabulary MOVED from the framework kernel into the plugin; ~470 framework lines deleted |

Compile evidence for all four: `cargo check --workspace` reports **zero errors in any
`✏️s/🔌️plugins` crate**. The only failing crates in that run are `semio-framework-os-kernel-db`
(57) and `semio-compose-rs` (22), both foreign refactors belonging to other sessions.

⚠️ **Test targets are NOT yet verified for these** — see the caveat below.

## RELEASED — Wave C / late Wave M lanes complete

| plugin | facets | evidence |
|---|---|---|
| `🎥️shooting` | `🎥️shooting` | 31 mutations, 1:1 triad dirs, glue rewired, `cargo check` 0 errors, **`cargo test` 104/104** |
| `🌍️gis` | `🗺️gismap`, `🏔️gisterrain` | **`cargo test` 171/0**; 8× emoji collision fixed; both config mutations semanticized; 42 TS mirrors |
| `📏️layout` | `📏️layout` | 25 triads rewired; triple emoji collision fixed; 75 TS mirrors; found a real missing-`SemanticMutation`-import bug that plain `cargo check` never exercised |
| `➗️mathematical` | `➗️mathematical` | `dsl_derive::Mutations` → `dsl::Mutations` bug fixed; 3 orphan triads deleted; 6 funnel call sites |
| `💠️lowpoly` | `💠️lowpoly` | 16 mutations, 1:1 triad dirs, glue rewired, `cargo check` 0 self-owned errors |
| `🎪️demonstrator` | `🎪️playground` | 1 mutation, orphans removed, glue rewired, `cargo check` 0 errors |
| `📕️norm` | all 15 | **392 triads** — 5 facets migrated from scratch + 10 finished; every dir glue-mounted (no self-wiring left), real OpText/OpBinary, real TS mirrors, `from_snapshot` decomposition replacing whole-document replace |
| `📸️remodel` | `📸️remodel` | 34 mutations replace all 20 `Set*`; no whole-collection setter survives; `cargo check` 0 errors |

Banned-token census: **43 non-stdio `.rs` files remain**, and the known composition of that number
is (a) app *command* names, ruled out of scope — see `📓️requeue-backlog.md` §A0 — and (b) the
flow plugin's `from_framework_mutation`/`to_framework_mutation` kernel bridge, which cannot be
removed from the plugin side and is being resolved by DKM (#2550) taking the framework module.

## HELD — lane in flight

`🏛️architect` (restructure landed, report written, awaiting confirmation), `🎞️animate`,
`🏭️process`, `💡️reasoning`.

## ⚠️ `cargo check` is not sufficient evidence — test targets matter

`cargo check` does not compile `#[cfg(test)]` code, so it cannot see the triad law tests
(`assert_mutation_inverse_law`, `assert_mutation_diff_absorb_law`, the `DiffAlgebra` laws) that are
this mechanism's entire correctness argument. Proof this is not theoretical: **`🕸️dag` passes the
workspace check and still fails to build its tests.**

Therefore a facet counts as verified only under `cargo check --workspace --all-targets` plus a
passing `cargo test`. `🖨️raster` is the only plugin currently meeting that bar
(66 passed / 0 failed). The `--all-targets` sweep is running now, and anything it turns up gets
requeued before release is confirmed.

## HELD — between waves (Wave R done, Wave C app-debt not yet launched)

`✒️writer`, `🌿️vcs`, `🌊️flow`, `🎬️sequence`.

## NOT SMO'S TO RELEASE

`🗄️stdio` — claimed by UCAS (#2548) for the `🧿️semio` subset roster restructure. SMO's 53 stdio
mutation facets are deferred behind them and will not start until they signal "roster frozen".

## Notes for consumers

- `🎪️demonstrator`: APA's census found it registers IO handlers for four artifact kinds owned by
  other plugins; for `3d.process` and `3d.procedural` both register the same kind into one
  process-global map, so load order silently decides the winner. That is a real bug and it is
  APA's, not SMO's — this ticket's demonstrator lane touches only the mutation facet and mutation
  call sites, and will neither fix nor disturb it.
- Releasing a plugin does **not** mean SMO will never touch it again. The final ratchet
  (`SemanticMutation` bounds, populating `MutationMeta.semantic_kind`/`label`) is repo-wide and
  lands last; it touches framework trait definitions rather than plugin internals, but it can
  force mechanical follow-ups. I will announce before that ratchet starts.
