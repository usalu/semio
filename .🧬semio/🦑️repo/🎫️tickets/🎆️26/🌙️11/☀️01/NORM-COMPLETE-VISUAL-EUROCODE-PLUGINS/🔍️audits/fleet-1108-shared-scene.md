# Fleet 1108 report: lane shared-scene (structural-scene)

## #68 StructuralProjection rollout
Stale: 13 of 15 norm artifacts already declare `live_scene!(pub mod live = Kit<XKit>)` (en1990-1999, din4108, din18599, iso16757), through `SceneKit` + `Kit<K>` (not the mock). The trait is complete: `entities`, `fingerprints`, `entity_dependencies`, `project_entity`, `project_global`, `finish`, `basis`, `diagrams`, `handles`; `SceneKit` supplies defaults for layout stamps, scalar owner and inert references.
Open: `din16798` and `vdi3805` scene crates have no root `🦀️.rs` (their `[lib] path` is dangling) and no `SceneKit` impl or `live_scene!`; request filed in `requests.md` under `## fleet-1108 → din16798, vdi3805`.

## Golden bless (#117 #130 #186 #267)
Root cause: each artifact hand-rolls compare/bless; an empty golden fails, and the bless needs `SEMIO_UPDATE_GOLDEN` to reach the test process, which the repo gives no guarantee for.
Fix: `projection_kit::test_support::assert_golden(path, actual)` in `🧰️projection-kit/🧪️test-support/🦀️.rs`. Writes the golden when `SEMIO_UPDATE_GOLDEN` is set or when it is missing/empty (first run fails once to force review, the next passes); otherwise compares ignoring trailing whitespace. The structural-scene mock golden test uses it. Unit test `an_empty_golden_is_blessed_once_and_then_guards_the_bytes` plus a feature line in `🧪️tests/🧰️projection-kit/🥒️.feature`.
Bless requests filed per artifact (en1990-en1999) in `requests.md`. Goldens exist for 10 artifacts + the structural-scene mock (11 files, all 0 bytes); iso16757 has a fixtures json elsewhere.

## Vocabulary vs visual-editor-architecture §3
Compared geometry (Member, Plate, Shell, Solid, Fastener, Support, Load, Surface, Group), solids (Prism, Box, Cylinder, Wedge, Mesh, StrataColumn, FootingBody, WallAssembly, Laminate), fasteners (Bolt..Tendon plus WeldRun), patterns, loads (all 9 glyphs), supports (6), annotations (5), failure modes, deformation overlay (deformed + mode, surface samples), utilisation binding with stations, member fields (taper, offsets, releases, stations), tessellation modules (section outline, extrude, fasteners, solids, glyphs, shells). Everything in §3 exists, several beyond (Laminate, WeldRun, Points pattern, motion tracks). Nothing missing; no schema change made.

## Gates
Contract build queued through build-slot.sh: log `🗑️generated/shared-scene/1.txt` (see final status in the hand-back). Not verified green unless stated there.

## QuantityKind additions (coordinator request, precompile pass 2)
Added `VentilationRate` (SI m³/s, unit family VolumeFlow), `HeatTransferCoefficient` (W/(m²·K), HeatTransfer) and `Power` (W, Power) to `QuantityKind` in Rust, GraphQL, proto (18-20) and TypeScript; also `Velocity`, which TS and proto were missing (proto 17). `kind_fits_unit` in test-support accepts them for their unit families. Colour: `ResultField::Airflow` (cool ramp) and `HeatFlow` (warm ramp) plus `ResultField::of(QuantityKind)`. Tests: twin-parity/wire-name test in `🧪️tests/🔬️unit/🦀️.rs`, colour-map assertions in `🎨️colour-maps/🧪️tests`. Uncompiled.

Quantity-kind feature row, fixture `🧪️tests/🧫️fixtures/📏️quantity-kinds/🔣️.json`, Python oracle `🔮️oracles/📏️quantity-kinds.py` (run with `.venv/bin/python -I`): 0 failures, 4 rows (observed). Rust twin test reads the same fixture (uncompiled). The gated build 1.txt was cancelled by the coordinator; shared/2.txt compiles these crates.
