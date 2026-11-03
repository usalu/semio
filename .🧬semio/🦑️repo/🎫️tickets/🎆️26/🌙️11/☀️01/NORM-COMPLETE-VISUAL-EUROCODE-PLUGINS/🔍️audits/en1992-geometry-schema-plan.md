# EN 1992 snapshot geometry fields: plan and touch list

Status: not applied. Reason: a scalar field in `RcMember` touches 14 hand-written, byte-order-checked twins (including the minified SQLite codecs with `ieee754_bits`/`_class` columns and column-index arithmetic), a collection needs new SQLite child tables in Rust and TypeScript, and the build is on hold, so a blind change would break the wire-twin, SQLite round-trip and 166 fixture tests at once. Apply together with the first green build, then validate in one run.

## Fields (all additive, defaults keep every current document valid)

- `RcMember.placement: Placement { x, y, z, yaw }` (m, rad), `RcMember.relativeHumidity` (0..1, default 0.5; feeds A-01/A-02 and SL-08).
- `RcMember.section: Vec<SectionPoint { y, z }>` (outline polygon in m, empty = rectangle b×h), `RcMember.bars: Vec<BarPoint { id, layerId, y, z, diameter }>` (empty = distribute `longitudinal` layers by rule), `RcMember.tendonPoints: Vec<TendonPoint { x, y, z }>`.
- `PunchingSpec.studRings: Vec<StudRing { radius, count, diameter }>`.
- `Anchor.placement`, `Anchor.headDiameter` (replaces the 2d assumption in `part_4::pullout_resistance_n`).

## Touch list per scalar (found with grep on `bridge_delta_sigma_s`)

`🧬️schema/{🟦️.ts,🛰️.proto,🔣️.json,🔗️.graphql}`, `🧬️schema/📸️snapshot/{🦀️.rs (struct + 6 constructor literals), 🟦️.ts, 🛰️.proto, 🔣️.json, 🔗️.graphql}`, `🚪️io/🪶️sqlite/📸️snapshot/{🗄️.sql (REAL column + bits/class columns), 🦀️.rs (insert and read, column indices from 29), 🟦️.ts (type, `fields([...])`, write, read, MEMBER descriptor), 🧪️tests/🦀️.rs}`, `✏️editor/🏷️field-meta/🦀️.rs`, 68 fixture snapshots under `🧪️tests/🧫️fixtures/🧬️mutations/**/📸️snapshot/**/🔣️.json` (insert the new keys after `bridgeDeltaSigmaS`, canonical order). Collections additionally: child tables `en1992_section_point`, `en1992_bar_point`, `en1992_tendon_point`, `en1992_stud_ring` with ordinal columns (pattern of `en1992_bar_layer`), `#[dsl(table)]` fields, text/binary grammars if they enumerate fields.

## Events

Mutation leaves (one per field, pattern of `change-member-cover`): `move-bar`, `move-section-vertex`, `move-tendon-point`, `change-stud-ring`, `change-member-placement`, `change-member-relative-humidity`, `change-anchor-head-diameter`; each needs the leaf crate files, `binaryTag`, edit-rules rows, oracle manifest rows, `KINDS` entry and fixtures `✅apply/⛔`.

## Generator

`🧰️scripts/en1992-geometry-fixtures.py` (to write with the application): walks the 68 fixture JSON files and inserts the defaults after `bridgeDeltaSigmaS` of every member and after `aS` of every anchor; idempotent (skips present keys); `DRY=1` prints the diff count.
