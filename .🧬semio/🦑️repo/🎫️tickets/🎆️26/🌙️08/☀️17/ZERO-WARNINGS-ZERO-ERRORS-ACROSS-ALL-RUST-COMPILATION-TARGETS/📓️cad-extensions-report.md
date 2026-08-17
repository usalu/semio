# CAD AEC-Domain Extension Crates — Report

## Scope
Four workspace members, all under `✏️s/🔌️plugins/📐️cad/🧩️extensions/*/📦️packages/🦀️rust`:

| Crate dir | Crate name |
|---|---|
| `📐️spatial-shape` | `semio-s-plugin-cad-spatial-shape` |
| `🏢️aec-building` | `semio-s-plugin-cad-aec-building` |
| `🔥️aec-building-energy` | `semio-s-plugin-cad-aec-building-energy` |
| `🏛️aec-building-structure` | `semio-s-plugin-cad-aec-building-structure` |

## Result: all four already at 0 warnings / 0 errors — no code changes needed

Ran `cargo check -p <name> --message-format=short` for each, one at a time (foreground,
synchronous — some runs exceeded the harness's 300s soft-timeout and were polled to completion
via `pgrep`-based wait loops rather than relying on any background-task notification, per the
hazard noted for this ticket). All four completed with exit code 0 and **zero** `error`/`warning`
lines attributed to the target crate itself in any run's output.

- **`semio-s-plugin-cad-spatial-shape`**: 0 → 0. Clean build, ~9m (cold, first crate checked).
- **`semio-s-plugin-cad-aec-building`**: 0 → 0. Clean build, ~6s (warm cache).
- **`semio-s-plugin-cad-aec-building-energy`**: 0 → 0. Clean build, ~49s.
- **`semio-s-plugin-cad-aec-building-structure`**: 0 → 0. Clean build, ~8m14s.

Each run did print warnings, but every single one was attributed to an upstream dependency crate
compiled along the way, never to the target crate's own `(lib)`:
- `semio-framework-plugin` (lib): 2 recurring `never read` field warnings
  (`child_slots`/`link_slots` at `🦀️component.rs:2812`, `schemas`/`inferences`/`languages`/
  `app_schemas` at `🦀️component.rs:3822`) — appeared in every run. In the
  `aec-building-structure` run specifically it showed as 12 warnings (10 more
  `unnecessary_qualifications` at various lines, auto-fixable via `cargo fix`), suggesting that
  particular `semio-framework-plugin` compilation unit picked up feature/cfg flags the other runs
  didn't. Not this ticket's crate — out of scope, not touched.
- `semio-s-plugin-stdio` (lib): the 4 known-remaining warnings documented in
  `📓️stdio-plugin-report.md` (`parse_ut_mtime`, `hard`, `soft`, `StlFormat::Ascii`) — showed up
  once, as a transitive dependency of `aec-building`.
- `semio-s-plugin-cad` (lib, the parent plugin these four extend): 1 warning (`fn plugin` never
  used) — showed up once, also as a transitive dependency of `aec-building`. Per
  `📓️progress.md`, `semio-s-plugin-cad` itself was already taken to 0 warnings earlier this
  session (`fem-gis-cad-surface` wave) — this 1-warning readout is presumably a stale/differently
  -configured compilation unit from a slightly different feature set than that wave checked, not
  a regression this task should chase; it is not one of this task's 4 assigned crates regardless.

Verified with an explicit grep pass across all four raw logs for `error` (workspace-wide zero
hits) and for each crate's own `generated ... warning` summary line (zero hits, confirming no
warnings self-attributed to any of the four target crates).

## Files touched
None. No source edits were necessary — all four crates were already clean on their `(lib)`
target before this task began (very likely a side effect of the machine-wide
`cargo fix --workspace --all-targets --allow-dirty --allow-staged` pass documented earlier in
`📓️progress.md`, since these are small extension crates with no distinctive lint-prone code of
their own).

## Not checked / left alone
- `(lib test)` targets were not run for these four crates — out of scope per the ticket brief
  (only `(lib)` target warning-zeroing was requested; test-target errors from the cross-cutting
  `Mutation::apply`/`::diff` migration are explicitly out of scope repo-wide).
- The `semio-framework-plugin` warnings and the stray `semio-s-plugin-cad` 1-warning readout
  above are dependency-crate warnings, not this task's 4 crates — left untouched, flagged here
  for whoever owns those crates' cleanup.
