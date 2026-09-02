# 🧩️ `semio-s-plugin-puzzle` — serde derive/attribute → first-party `value` conversion

## Scope actually done

Every `#[derive(… Serialize, Deserialize …)]` / `#[serde(…)]` site under
`✏️s/🔌️plugins/🧩️puzzle` (crate `semio-s-plugin-puzzle`, entry
`📦️packages/🦀️rust/🦀️.rs`, all real source spliced in via `#[path]`) was converted. No
`🧪️oracle/`, `🧪️test/`, `🔬️probes/`, `🏭️generator/`, `🧫️fixtures/` dirs exist inside this
crate, so nothing was excluded on that basis.

**290 derive sites** across **129 files** converted (matches the grep-verified count of
`#[derive(...)]` lines mentioning `Serialize`/`Deserialize`). Added
`extern crate semio_framework_value_derive as value_derive;` to the crate root and the
`semio-framework-value-derive` path dependency to `Cargo.toml` (framework module, not a
Cargo.toml serde line, so allowed).

## Why a flat "convert everything" pass was unsafe here — and the classification actually used

Puzzle's `serde_json` surface is **not** confined to derive-driven convenience: each of the
three artifacts (puzzle2d/3d/5d) has a real, unconditional production JSON import/export bridge
(`🚪️io/📥️import/…/🔣️json/🔖️rfc8259/✳️any/🦀️.rs`, `🚪️io/📤️export/…`) that calls
`serde_json::from_value` / round-trips through `Puzzle{2,3,5}dSnapshot`, plus ~2000 more
`serde_json::` call sites throughout editor commands/windows/brush/camera code. Blindly
stripping `Serialize`/`Deserialize` from every derive would have broken those call sites; the
task's own rule ("convert derive/attr, don't rewrite call sites, note single exceptions") doesn't
scale to a plugin with this many genuine call sites. So each of the 290 sites was classified
into one of three buckets by a reachability analysis (script + reasoning below), not by hand:

1. **`prod` (102 sites)** — the struct/enum is reachable (by field/variant type, BFS) from a type
   actually consumed by a **production** (non-test) `serde_json::` call site — chiefly
   `Puzzle2dSnapshot`/`Puzzle3dSnapshot`/`Puzzle5dSnapshot` via the RFC8259 io bridges, plus a
   handful of standalone command-payload structs (`PreselectSync`, `BrushPlacePayload`,
   `SceneConfig`, …) decoded directly with `serde_json::from_value`/`from_str`.
   → **dual-derive**: kept the original `Serialize`/`Deserialize` (and its `#[serde(…)]`
   attrs) exactly as they were, unconditionally, and **added**
   `value_derive::ToValue, value_derive::FromValue` + a parallel `#[value(…)]` attribute next to
   each kept `#[serde(…)]`. Nothing here can regress; it's additive.
2. **`test` (107 sites)** — reachable only from the 9 fixture-oracle root types
   (`Puzzle{2,3,5}dSnapshot`/`Mutation` + a few nested config/play-snapshot types) exercised by
   the 102 `🧪️tests/**` fixture files (`serde_json::from_str::<T>` against committed JSON), and
   **not** reachable from any production call site.
   → the sanctioned end-state from the task brief: unconditional derive is now
   `value_derive::ToValue, value_derive::FromValue` + `#[value(…)]`; the original
   `Serialize`/`Deserialize` moved to `#[cfg_attr(test, derive(serde::Serialize,
   serde::Deserialize))]` with `#[cfg_attr(test, serde(…))]` mirroring the original args
   (fully-qualified `serde::` paths used so no `use serde::{…}` import is needed for these).
3. **`none` (81 sites)** — not reachable from any serde_json:: call site, production or test
   (editor runtime state, windows, brush engine, retained-command payloads that never round-trip
   through JSON in this crate). → full conversion: `Serialize`/`Deserialize` removed,
   `value_derive::ToValue, value_derive::FromValue` added, `#[serde(…)]` → `#[value(…)]`.

`ArtifactSchema`-derived types (18 sites: `Puzzle{2,3,5}d{Config,Presence,Inference,Diff,
Artifact}`) got the same 3-way serde treatment on their attributes, but **no** `ToValue`/
`FromValue` was added — matching the existing repo convention (`🗄️stdio`'s
`SemioObjectArtifact`) where `ArtifactSchema` is the complete first-party contract and any
`ToValue`/`FromValue`-equivalent behaviour for that type is hand-written elsewhere, not derived.
`Puzzle{2,3,5}dSnapshot` are themselves `ArtifactSchema` + `prod`, so they kept dual-derive
serde/value attrs without an added `ToValue`/`FromValue` derive either.

All argument keys actually used in this crate — `rename_all`, `rename`, `default`,
`skip_serializing_if`, and `tag` (+ `tag, rename_all` together, on the externally-tagged
`Puzzle{2,3,5}dMutation` dispatch enums) — are in `#[value(…)]`'s supported set, confirmed
against `🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️.rs` before the bulk transform. No `content`
(adjacent tagging) or `flatten`/`with`/`skip` sites exist in this crate (confirmed by grep) —
nothing was left as a single-site exception.

Reachability was computed by a script (kept at
`/private/tmp/.../scratchpad/{scan.py,phase2.py,transform.py,cleanup_imports.py}`, not committed
— per-session scratch, not a permanent script) that: (a) parses every struct/enum with a
brace-matched body, (b) builds a type-reference graph from capitalized identifiers inside each
body, (c) seeds two root sets — types touched by `serde_json::from_str::<T>`/`from_value`
turbofish, `let x: T = serde_json::…`, or a function whose return type is `T` and whose body
calls `serde_json::from_*` — split by whether the call site sits inside a `#[cfg(test)]` span
(whole `🧪️tests/**` files, or `#[cfg(test)] mod … { }` blocks found in 65 other files) or not,
then (d) BFS-expands each root set over the graph. Deliberately biased toward
over-approximation (a false "reachable" just means extra harmless dual-derive/cfg_attr, never a
compile break); the real check is the `cargo check` below, not the heuristic.

After the transform, a cleanup pass removed 110 now-fully-dead `use serde::{Serialize,
Deserialize};` imports (files where every derive in that file became `none`/`test` class, which
use fully-qualified `serde::` paths and no longer need the import); 14 were kept because a
`prod`-class site in the same file still uses the bare names.

### Post-transform integrity checks (all against the actual on-disk result, not a dry run)

- `rustfmt --check` on all 129 touched files: 110 already clean, 19 report pure re-wrap diffs
  (long pre-existing lines) with zero `error:` lines — every file parses as valid Rust.
- `cargo metadata --no-deps` on the crate's manifest: exit 0 — the new `Cargo.toml` line and its
  relative path resolve.
- `#[value(…)]` count (838) equals the original `#[serde(…)]` count exactly (838) — one mirror
  per site, no drift.
- `#[cfg_attr(test, derive(…))]` count (107) equals the `test`-class site count exactly (107).
- Every `#[cfg_attr(test`/`#[cfg_attr(test, derive`/`#[cfg_attr(test, serde` line matches the
  well-formed `cfg_attr(test, (derive|serde)(…))]` shape — zero malformed lines.
- A line-adjacency check (not offset-based — see caveat below) confirmed all 838
  `#[value(…)]`/`#[serde(…)]` sibling pairs have byte-identical arguments: 0 mismatches.
- No manual `impl ToValue for X` / `impl FromValue for X` exists anywhere in the crate that a
  newly-added derive could collide with; no pre-existing symbol named `value_derive`.
- Spot-checked one `git diff` sample from every bucket (`prod`, `test`, `none`,
  `ArtifactSchema`+`none`, `ArtifactSchema`+`prod`, qualified `serde::Deserialize` original form,
  and the `tag`+`rename_all` externally-tagged `Puzzle3dMutation` enum) by hand — all correct.
  (An earlier *offset-based* self-check script, re-slicing the file by each site's
  pre-transform byte offsets, threw ~56 false "missing serde" warnings on `prod`/`test` sites in
  multi-site files — a bug in that throwaway checker, not the transform: `transform.py` itself
  splices back-to-front per file, which is offset-safe; the standalone checker replayed stale
  absolute offsets against the fully-edited file, which isn't. Every one of the 56 was confirmed
  a false positive via direct `git diff` on the actual file.)

## What was deliberately NOT touched (out of this pass's scope)

- Manual `impl serde::Serialize for X` / `impl<'de> Deserialize<'de> for X` blocks (4 found:
  `Puzzle3dScale`, `Puzzle5dScale`, `Puzzle2dFillText`, `Puzzle3dPlaySnapshot`) — not derives, so
  outside "derive → ToValue/FromValue" as literally scoped.
- The ~2000 `serde_json::` call sites themselves (`from_value`/`from_str`/`to_string`/`to_value`
  scattered through editor commands, camera, brush, patch-inspector, the io import/export
  bridges). Trinity/stdio/imperative already show the intended replacement
  (`pack::to_json_string`/`pack::from_json_str`, generic over `ToValue`/`FromValue` instead of
  `serde_json::Value`), but rewiring ~2000 call sites is a call-site-rewrite wave of its own —
  comparable in size to (larger than) the `🏭️process`/`🌀️procedural` waves the ticket's own
  `📓️status.md` already deferred as separate work. Not attempted here to avoid a half-tested,
  high-blast-radius rewrite in one pass.
- `serde_json::Value` used directly as a **field type** (e.g. `Puzzle2dConfig.brush_candidates:
  Vec<serde_json::Value>`) — this is the same "export API typed on a third-party value"
  architectural item already flagged in `📓️store-production-serde-surface.md` for `🏪️store`;
  fixing it means retyping the field to `DslValue`, not a derive/attribute change.
- `Cargo.toml`'s `serde`/`serde_json` lines — left untouched per the rule; the crate still
  genuinely needs both (dual-derive `prod` sites + the untouched call sites above).

## Verification

`cargo check -p semio-s-plugin-puzzle` was run in the foreground (Bash tool auto-moved it to
background after its 120s cap — never requested `run_in_background`/Monitor myself; that's
documented harness behaviour, not a rule violation) both before touching Cargo.toml and again
after the full transform. **Neither run produced a real result in over 40 minutes** — both sat at
`Blocking waiting for file lock on build directory` the entire time. Root-caused, not just
assumed: `lsof +D target` shows PID 19466 — `cargo check --workspace
--message-format=json-diagnostic-rendered-ansi --keep-going --all-targets` (someone else's
whole-monorepo gate, 100+ crates, all targets including tests) — holding the exclusive write lock
on `target/debug/.cargo-lock` for 55+ minutes and counting, actively writing tens-of-MB
`.fingerprint` output files for an unrelated plugin (`semio-s-plugin-block`). Every `cargo
check -p X` from every concurrent agent, mine included, queues behind that one process; this is
the "Concurrent Cargo Workspace Churn" pattern the fleet has hit before, at a scale (60-70
concurrent rustc/cargo processes observed via `ps aux`) beyond what a single agent can wait out
in one session. I did not kill PID 19466 (not mine to kill, clearly legitimate) and did not
kill-and-retry my own check (banned) — left it running at
`/private/tmp/.../scratchpad/check_after.txt` (PID 75920) for whoever picks this up next to read
once the lock clears; a stale duplicate of my own (PID 52002, started before the edits landed)
was killed since it was pure noise, confirmed via its own log to have never gotten past the same
lock wait either.

**Because the real compiler result never arrived, I cannot claim 0 errors — only that no
alternative, offset-safe static check found a defect.** What was actually confirmed, all against
the real on-disk result:
- `rustfmt --edition 2021 --check` on **all 706 `.rs` files** in the crate (not just the 129
  touched): 0 parse errors.
- Every `+` line in the full crate diff with a leading `#[` has balanced `[]`/`()`: 0 imbalances.
- `cargo metadata --no-deps` on the manifest: exit 0 (new dependency + path resolve).
- Field-name sequence in the 3 largest touched files (111/143/117 fields) is byte-identical
  before/after (compared against the git-index blob, not `HEAD` — this repo's `HEAD` predates the
  file's existence entirely; the working index is the real "before").
- Every one of the 838 `#[value(…)]` lines has an argument string identical to its `#[serde(…)]`
  (or `#[cfg_attr(test, serde(…))]`) sibling: 0 mismatches.
- Manual `git diff` review of one real example from every derive-combination bucket actually
  present in this crate — plain, `dsl::DslRecord`, `dsl::DslOps`, `dsl::DslArtifact`,
  `dsl::DslEnum`+`dsl::Mutations` (incl. the externally-tagged `tag`+`rename_all` enum), and
  `ArtifactSchema` crossed with all three of `prod`/`test`/`none` — all correct.

None of this substitutes for the real `cargo check`. If it's still not landed by the time this
ticket is picked back up: re-run `cargo check -p semio-s-plugin-puzzle` in the foreground once
the workspace-wide gate (or whatever's holding the lock next) clears, and fix forward from
whatever it reports — the conversion here is mechanical and low-risk (additive for `prod`,
symmetric for `test`, and every `none` site was reachability-checked against real call sites), so
a first real error is far more likely to be an interaction with a genuinely different concurrent
change (per the ticket's own repeated experience) than with this transform itself.
