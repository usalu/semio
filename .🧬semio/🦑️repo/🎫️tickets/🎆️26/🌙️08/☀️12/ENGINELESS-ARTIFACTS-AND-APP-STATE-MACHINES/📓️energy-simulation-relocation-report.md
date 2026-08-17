# Energy Simulation Relocation Report

## What moved

**Source:** `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/`
**Destination:** `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/`

All 50 `snake_case` directories were moved verbatim (single `mv` per directory, no content edits) — each
directory contains exactly one `🦀️component.rs` file:

```
air_exchange, air_system, airflow_network, calendar, coils, comfort, controls, curves, daylight,
dispatch, economics, electrical, envelope, error, evaporative, fans, faults, fenestration, gains,
geometry, heat_recovery, humidity_eq, hvac_topo, iaq, ideal_hvac, kernel, material, meters, metrics,
model, num, output, plant, precompute, props, refrigeration, results, room_air, schedule, shw, sim,
site, sizing, solar, solar_thermal, terminal, units, water, zone_air, zone_hvac
```

Verified count: 50/50. Verified total LOC moved: **11,494** (matches ticket's stated figure exactly).

The following leaves were confirmed present and were **NOT** moved (still legitimate `💡️inferences`
family members): `💾️binary`, `📝️text`, `🗃entries`, `🔗️component.graphql`, `🔣️component.json`,
`🛰️component.proto`, `🟦️component.ts`, `🦀️component.rs`.

## Sibling convention mirrored

Read three sibling `🔨️modules/` layouts before choosing an approach:

1. `✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/{🐚️shell,🧠️lsp}` — each submodule has its **own**
   `📦️packages/{🦀️rust,🟦️typescript}` with its **own** `Cargo.toml` (separate workspace crate,
   `[[bin]]` or `[lib] crate-type = ["rlib","cdylib"]`). This is used because `jack/shell` is a
   standalone binary and `jack/lsp` is a wasm-bindgen cdylib — both need independent crate metadata.
2. `✏️s/🔌️plugins/🌊️flow/🔨️modules/🧮️compute` — a bare stub, single `🟦️component.ts`, no Rust crate
   at all.
3. **The decisive precedent**, found while investigating (not named in the task brief but directly
   on-point — same master ticket, same operation): `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/📦️glue.rs`
   mounts `✏️s/🔨️modules/🏗️fem/⚙️engine/<domain>/🦀️component.rs` via bare `#[path]` attributes
   directly into FEM's existing single crate — **no new crate, no `Cargo.toml`, no `project.json`**
   for the engine directory itself. FEM's own header comment cites this exact ticket
   (`26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES`) and the same rationale ("an artifact is a
   schema + io system, never an engine; a MODULE may still have one — `taxonomyLeafParentDirs`
   already lists `⚙️engine` globally").

Energy's 50 domain files are plain internal library modules (not a separate binary, not a
wasm-bindgen cdylib), so — per the ticket's explicit hard rule to prefer "a mount-only arrangement
inside the existing crate" and to **stop and report** rather than create a new crate/workspace
member — I mirrored the **FEM precedent**, not the jack/shell or jack/lsp crate-per-submodule shape.
No new `Cargo.toml`, `project.json`, or workspace member was created. The 50 directories mount into
the existing `semio-s-plugin-energy` crate (`✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust/📦️glue.rs`)
exactly as they did before the move — only the `#[path]` target changed.

(Note: FEM's shared destination is the **repo-root** `✏️s/🔨️modules/🏗️fem/⚙️engine/`, not a
plugin-scoped `✏️s/🔌️plugins/🏗️fem/🔨️modules/…`. The task brief gave an explicit, fully-specified
destination path — `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/<domain>/`, matching the
trinity/flow plugin-scoped `🔨️modules/` shape — so that given path was used as directed; only the
*internal mount mechanism* (no new crate) was decided by reading the sibling conventions.)

## `#[path]` mounts and `use` sites repointed

Only one file in the whole repo referenced the moved paths:
`✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust/📦️glue.rs`.

All 50 `#[path]` attributes at crate root (previously lines 33–132, region `💡️Inferences`) were
repointed from:

```
../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/<domain>/🦀️component.rs
```

to:

```
../../🔨️modules/⚡️simulation/⚙️engine/<domain>/🦀️component.rs
```

The `pub mod <domain>;` names were **not** changed (still `pub mod air_exchange;` etc. at crate root),
so every existing `crate::<domain>::X` call site and the flat `pub use <domain>::*;` re-export block
(region `🔖️FlatReExports`, unchanged) both keep working unchanged, fully qualified against the
existing crate — no bare path could rebind elsewhere.

The doc comment header (lines 1–24) and the region markers (`//#region 💡️Inferences` →
`//#region ⚡️SimulationEngine`, and matching `//#endregion`) were updated to state the corrected
rationale (not an inference family — fails `Inference<Snapshot>` totality/purity/determinism —
relocated to a module engine, no app engine because energy has no app), modeled on FEM's own header
comment for the identical operation. No other Rust `use`/path anywhere in the repo referenced the old
locations (repo-wide grep confirmed) — the only other file matching `energy.*💡️inferences` search text
was an unrelated false-positive in `stdio`'s glue.rs (`🔖️energyplus` EPW standard name, nothing to do
with the `🔋️energy` plugin).

The nested `pub mod inferences { … }` block inside the `🗿️artifacts` region (lines ~221–236) was left
untouched — it only ever mounted the family's own `🦀️component.rs`/`📝️text`/`💾️binary`/`🗃entries`
leaves, none of which moved.

## Verification

```
ls …/🧬️schema/💡️inferences | grep -cE '^[a-z][a-z0-9_]*$'                     → 0
ls …/🧬️schema/💡️inferences                                                    → 💾️binary, 📝️text, 🔗️component.graphql,
                                                                                  🔣️component.json, 🗃entries,
                                                                                  🛰️component.proto, 🟦️component.ts,
                                                                                  🦀️component.rs   (all 8 leaves present, verbatim)
find …/🔨️modules/⚡️simulation/⚙️engine -maxdepth 1 -type d | wc -l              → 51  (50 domains + itself)
LOC moved                                                                       → 11,494  (exact match)
grep -c "💡️inferences/[a-z]" …/📦️glue.rs                                     → 0
```

All five numbers match the ticket's target exactly.

## Compiler check

Command run exactly as mandated:

```
RUSTC_WRAPPER="" cargo check -p semio-s-plugin-energy --all-targets
```

Result: **compiled clean.** Ran twice for corroboration: first attempt (redirected straight to the
ticket file, foreground) hit the tool's own timeout mid-build (exit 144, a harness-level SIGALRM, not
a cargo error — `RUSTC_WRAPPER=""` disables sccache so this crate compiles from scratch against a
warm-but-shared `target/`, and ~18 concurrent agent sessions are hammering the same tree); re-run in
the background completed cleanly, producing a `Finished \`dev\` profile [unoptimized] target(s) in
1m 38s` line and `EXIT CODE: 0` — required as evidence per the ticket, since a plain exit-code check
alone is unreliable if `sccache` misbehaves. Zero `error` lines anywhere in the 7,748-line output
(`grep -n "^error"` → empty). `semio-s-plugin-stdio` — which every plugin including energy depends
on — **compiled successfully in this run** (not RED at the time this check executed, contrary to the
ticket's warning that it was live-RED at ticket-write time; ~18 concurrent agent sessions are
actively touching the repo, so its state is a moving target moment-to-moment — this run simply landed
on a green window). `semio-s-plugin-energy` itself produced only pre-existing warnings (unused
imports, elided lifetimes, unnecessary qualifications) — **zero errors**, and `grep -c
"🔨️modules/⚡️simulation" scratch-energy-relocation-compile.txt` → **0**, confirming none of the 9
warnings originate from the relocated `⚙️engine/<domain>` files (all 9 point at pre-existing
`🗿️artifacts/🔋️model/…` files untouched by this session).

Full verbatim compiler output saved at:
`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES/scratch-energy-relocation-compile.txt`

### Error attribution
- **Mine:** none (zero compiler errors on `semio-s-plugin-energy`).
- **Pre-existing:** the 9–10 warnings on `semio-s-plugin-energy` all point at files under
  `🗿️artifacts/🔋️model/…` (unused imports, elided lifetimes, unnecessary qualifications in
  `io`/`schema`/`mutations` components) — none touch the relocated `⚙️engine/<domain>` files, and
  `git log` shows these files were not touched by this session.
- **Upstream:** `semio-s-plugin-stdio` produced 694 pre-existing warnings (not errors) during its own
  build in this run — unrelated to this move, not modified by this session.

## Auto-commit note

This repo runs a background auto-commit process (per project workflow). `git status` after the move
showed the working tree already clean and the move (dir relocation + `📦️glue.rs` edit) already folded
into an automatic commit (`62152fabcc`) — confirmed via `git cat-file -e HEAD:<path>` for both the old
(gone) and new (present) locations, and via `git show HEAD:📦️glue.rs | grep …` for the mount edit. No
`git add`/`git commit`/any modifying git command was run by this session; this is the repo's existing
auto-commit behavior picking up the filesystem `mv` and file edit.

## Files touched

- **Moved** (50 directories, `git mv`-equivalent via plain `mv`, verbatim contents):
  `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/<domain>/`
  → `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/<domain>/` for each of the 50 domains
  listed above (see `scratch-energy-moved-domains-list.txt` in this ticket folder for the exact
  enumerated list read back from the destination).
- **Edited:** `✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust/📦️glue.rs` — 50 `#[path]` mounts repointed,
  header doc comment and region markers updated to reflect the corrected rationale.
- **Not touched:** `📜️script.ts`, `🔣️taxonomy.json`, `AGENTS.md`, anything under
  `✏️s/🔌️plugins/🗄️stdio`, and everything outside `✏️s/🔌️plugins/🔋️energy`.

## Coordinator cross-check (post-completion, re-verified fresh from disk)

Coordinator #2546 independently enumerated all 112 inference families repo-wide and got
`TOTAL NON-EMOJI STRAYS REPO-WIDE: 0`, agreeing with this session's own count by two independent
methods. Re-ran the mandatory checks fresh after that message, reading names off disk rather than
retyping them (per the coordinator's own caught-bug precedent tonight):

- `find $DEST -maxdepth 1 -type d ! -path $DEST -exec basename {} \; | sort` → 50 names, byte-identical
  to `scratch-energy-moved-domains-list.txt` (`diff` → no output, `IDENTICAL`).
- Dangling-mount check (coordinator's exact script) on `📦️glue.rs`: **dangling: 0**.
- `grep -c "💡️inferences/[a-z]" 📦️glue.rs` → **0**.
- `grep -c "🔨️modules/⚡️simulation/⚙️engine" 📦️glue.rs` → 51 = 50 real `#[path]` mounts (lines 40–138,
  one per domain, each domain appearing exactly once) + 1 prose mention in the header comment (line 14)
  explaining the move — not a duplicate mount.

**Compile status:** the coordinator asked this to be reported as `UNVERIFIED — build-lock contention,
not attempted` for sessions still stuck in the 35-way cargo pile-up. This session is not in that
state: `RUSTC_WRAPPER="" cargo check -p semio-s-plugin-energy --all-targets` **already completed**,
before the coordinator's message arrived, with a `Finished` line, exit code 0, and zero `error` lines
across the full 7,748-line verbatim output (saved at `scratch-energy-relocation-compile.txt`). Per
this ticket's own rule — "Never report green on a build that did not run" — the converse also holds:
this build did run and did finish, so it is reported as **compiled clean**, not relabeled unverified.

## Scratch files in this ticket folder

- `scratch-energy-relocation-compile.txt` — full verbatim `cargo check` output.
- `scratch-energy-moved-domains-list.txt` — the 50 moved domain names, read back from the destination.
