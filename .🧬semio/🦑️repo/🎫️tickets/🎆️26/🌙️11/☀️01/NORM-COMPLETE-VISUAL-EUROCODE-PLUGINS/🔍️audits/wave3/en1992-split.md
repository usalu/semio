# W3-EN1992-SPLIT

## 1. en1992 scene crate split

`semio-s-artifact-norm-en1992-scene` was 300075 B, 4533 lines, 30 files (114 %, lines 113 % of the 4000 limit). Script: `🧰️scripts/crate-budget/w3_en1992_scene_split.py` (run once, refuses to run again).

| Crate | Folder (under `🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene`) | Content | Bytes | Files | Lines | Max ratio |
|---|---|---|---|---|---|---|
| `...-scene-foundation` (new) | `🧱️foundation` | `kit`, `layout`, `mechanics`, `reinforcement`, `tracks`, and `SceneInput` | 49643 | 6 | 1027 | 32 % |
| `...-scene-diagrams` (new) | `📈️diagrams` (root = the old diagram module) | `bond`, `ties`, `integrity`, `ledger`, `prestress_limit`, `material_laws`, `crack_width_moment`, `creep_curve`, `fatigue_range`, `fire_capacity`, `liquid_pressure`, `second_order`, `torsion_interaction`, `strut_and_tie`, `shear`, `grades`, `crack_pattern`, `prestress_loss` | 167775 | 19 | 2242 | 64 % |
| `...-scene` (kept) | `.` | root, `members`, `punching`, `anchors`, `cases`, `handles` | 84011 | 6 | 1304 | 33 % |

All three are far below 85 %. Layering: scene -> diagrams -> foundation -> (checks, model, compliance, structural-scene). The module folders were moved (plain `mv`, no git) into the crate folder that owns them, so each crate's closure is its own tree.

Seams and edits:
- Foundation holds everything the projections and diagrams both stand on. The back-edges of `kit` were cut: `kit::entities_of` became `kit::entities_where(document, declared)` and `kit::finish` no longer calls `tracks::finish` and `prestress_loss::finish`; the scene root defines `entities_of` (passes `cases::declared`) and `finish` (kit finish, tracks, prestress loss).
- `SceneInput` moved to the foundation root; scene root re-exports it.
- All `pub(crate)` and `pub(super)` in the moved modules became `pub`; `super::diagram_set::x` became `super::x` in the diagram modules (the diagram root now is the old `diagram_set`); the diagram root has extern aliases and `use foundation::{kit, layout, ...}`, so the `super::kit::*` style of every module is unchanged.
- Existing paths resolve: the scene root re-exports `kit` items, `layout`, `mechanics`, `reinforcement`, `tracks`, `SceneInput`, `Layout`, `Placement`, and `bond`, `crack_width_moment`, `grades`, `integrity`, `ledger`, `material_laws`, `prestress_limit`, `second_order`, `shear`, `ties`, `torsion_interaction`; the private ones (`crack_pattern`, `creep_curve`, `fatigue_range`, `fire_capacity`, `liquid_pressure`, `prestress_loss`, `strut_and_tie`, `diagram_set`) are private `use`s, so the tests in the scene crate (`use super::*`) compile unchanged. Tests stay in the scene crate; `shear` carries its own test module into the diagrams crate (dev-dependency `compliance-testing` feature, as in scene).
- Wiring: artifact `Cargo.toml` members and `workspace.dependencies` (2 each), `Cargo.lock` (2 new blocks, scene gains both dependencies; tomllib parse ok, 551 packages, sorted, no duplicate names), `📦️packages/🦀️rust/📜️script.ts` package list, scene manifest dependencies, `📋️build-owners.json` (2 entries, JSON parses). `project.json` needs no change (globs `🏅️standards/**`).

Checks run (no cargo, no rustc):
- source-budget: no en1992 crate in the warn band; the three crates above. Law summary 3 crates over the boundary, all en1993 mutation crates (`fastened-joints` 191 %, `fatigue-stresses` 106 %, `buckling-lengths` 106 %), not touched here.
- duplicate package names: none among repo-owned manifests (`[package] name` check, new names unique before and after).
- `check-cargo-authority.py`: checked 31 crates, 31 members, 0 errors (it skips `🗿️artifacts`, so the artifact members were checked by the workspace member and lock validation above).
- path-budget: my tree longest 186 bytes; the law still reports 241-byte en1991 mutation fixtures (not mine).
- Python mirror of the reasoning is in section 2; the Rust was reviewed by hand only.

Risks for the rebuild (not compiled): visibility of names reached through `use super::*` in kit (`kit` keeps `use super::*`, now the foundation root provides `En1992Snapshot` indirectly through `SceneInput`'s imports), `use kit::*` in the diagram root (kit imported from foundation), possible `unused_imports` warnings for `reinforcement`/`tracks` in the diagram root (used by child modules). Appended to `🗑️generated/build-en1992/latest.md`: `- ready for rebuild: en1992 split`.

## 2. Mechanics `M(x)` moment-line API

New module `✏️s/🔌️plugins/📕️norm/🧮️mechanics/📈️moment-line/🦀️.rs` (`mechanics::moment_line`, re-exported `BeamSupports`, `MomentLine`, `PointLoad`, `SpanLoad`), next to `continuous_beam` (EN 1990 6.4.3.2 envelope), no FE solve:
- `MomentLine::single(l, load)`, `::cantilever(l, load)`, `::continuous(&spans, &loads, clamped_start)`, `::solve(spans, loads, BeamSupports::{Pinned, ClampedStart, Cantilever})`, all `Result<_, String>` (refuse non-positive/NaN input, wrong list length, point loads on a support or outside the span, cantilever with more than one span).
- Load: uniform line load per span plus any number of point loads per span (downward positive). Sagging moment positive, shear upward on the left face positive.
- Queries: `moment_at(x)`, `shear_at(x)`, `support_moment(i)`, `reactions()`, `extremes(span)` (exact: span ends, point loads, zeros of the shear), `diagram(per_span)`, `spans()`, `length()`.
- Method: three-moment equation `M(i-1) L(i) + 2 M(i)(L(i)+L(i+1)) + M(i+1) L(i+1) = -6(S_R + S_L)` on the free-span moment areas (UDL `wL^3/24`; point load `P a b (L+a)/(6L)` and `P a b (L+b)/(6L)`), clamped start via the extra row `2 L M0 + L M1 = -6 S_L`, cantilever by statics. Gauss elimination with pivoting on the (n+1) system.
- EN 1995 T-30 usage: `MomentLine::single(l, SpanLoad {..})` for the simply supported and `cantilever` cases, `continuous(&[l1, l2], ...)` for two spans, then `moment_at(x)`.
- Tests (file `🧪️tests/📈️moment-line/🦀️.rs`, mounted in the mechanics root): closed forms (wL^2/8, PL/4, wL^2/2, -wL^2/8 over the middle support, 9wL^2/128 in a span, propped cantilever 5wL/8 and -wL^2/8, reactions 3/8, 10/8), refusals, and the property test: 240 seeded random beams (1..5 spans, UDL and 0..3 point loads per span, pinned/clamped/cantilever) compared at 13 abscissae, every support moment and every reaction with the numpy oracle `🔮️oracles/📈️moment-line/🐍️.py` (independent slope-deflection stiffness solve with `np.linalg.solve`), tolerance 1e-9 relative, plus vertical equilibrium and `extremes` vs a 2000-step sampled line.
- Verified here (Python only): oracle hand values (two equal spans -2000/1500/5000 for w = 1000, L = 4; propped cantilever -3125/3125; cantilever -5000) and a line-by-line Python mirror of the Rust algorithm vs the oracle on 300 random beams: worst relative deviation 4.2e-15. The Rust itself was not compiled or run.
- Feature bullet appended to `🧪️tests/🧮️mechanics/🥒️.feature`. Source-budget: mechanics crate is now 93 % (3714 of 4000 lines, 149429 B), warn band, not over; the next addition there should split a module.
- Appended to `🗑️generated/build-mechanics/latest.md`: `- ready for rebuild: mechanics M(x)`.
