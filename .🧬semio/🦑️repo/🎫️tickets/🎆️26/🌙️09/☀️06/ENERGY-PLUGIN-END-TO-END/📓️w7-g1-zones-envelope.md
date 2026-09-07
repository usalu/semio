# 📓️ G1 — zones & spaces (100–199) + geometry & envelope (200–299)

Lane G1. Ranges `100`–`199` (§4.2) and `200`–`299` (§4.3). Predecessor was killed at start, so nothing
in this file is inherited — every claim below was produced in this shift and is marked with the
evidence that produced it.

**Landed: 43 new kinds, 86 fixture cases.** Generator says `121 kinds, 242 fixture cases emitted`
(shared total across all groups; G1 owns 43 of the 121 and 86 of the 242). Numbers `105`–`111` and
`200`–`235`; `100`–`104` were already W-D0's.

---

## 1. Pattern vocabulary this lane added to the generator (for W-D0, the custodian)

All of it lives inside `#region 🔖️G1ZonesEnvelope` in `🐍️generate-mutation-leaves.py`, is free of
anything zone- or surface-specific, and is callable by any group.

| helper | what it emits |
|---|---|
| `g1_emoji(text)` | exactly one trailing U+FE0F, whether or not the source emoji was pasted with one. `mutationDirectoryPattern` requires the variation selector and most emoji render identically with and without it, so this removes a whole class of invisible failure |
| `g1_scalar(...)` | a one-scalar verb over an id-keyed `Model` collection: Rust `diff` (target-missing → guard ladder → no-op → write one field), Rust `inverse`, the Python second implementation and the Python inverse, from one row |
| `g1_create(...)` | `create-<entity>`: caller-supplied id, `mutation.duplicate-id` on a taken id, the guard ladder, insertion at the **ascending-id position** (see §2), plus `prelude`/`py_prelude` for a kind that must bind a local before writing and `extra_refusal` for a refusal the prelude raises |
| `g1_delete(...)` | `delete-<entity>` with no cascade: target-missing, a RESTRICT ladder, drop the row; undo is the matching `create` |
| `g1_rs_positive` / `g1_py_positive`, `…_non_negative`, `…_fraction` | the three numeric admissibility predicates, spelled once for Rust and once for Python so the two implementations cannot drift apart by accident |
| `g1_py_schedule_known(expr)` | "the model's own `ScheduleSet` defines this id", over the five schedule families the snapshot document carries |
| `g1_window_scalar(...)` | a `g1_scalar` over `fenestrations` whose two fixture scenarios are the same ANSI/ASHRAE 140 §5.2 south window, so the ten window kinds' committed vectors stay comparable |

Shared-surface extensions (small, appendable, marked):

- four rows added to each of `JSON_TYPES` / `PROTO_TYPES` / `TS_TYPES` / `GRAPHQL_TYPES`:
  `crate::model::SurfaceClass`, `crate::model::OutsideBoundaryKind`, `Vec<[f64; 3]>`,
  `Option<crate::model::EntityId>`;
- `aggregate_proto()` now emits one shared `message Vertex3 { double x = 1; … }` — protobuf has no
  nested repeated scalar, so every `Vec<[f64; N]>` payload field repeats that message;
- `FIXTURES_RS` gained `//#region 🧰️G1Constructors` with `space`, `material`, `construction`,
  `surface`, `window` and `shading`. They are reached as `fixtures::surface(…)`, so
  `case_component`'s fixed `use …::{self, link, snapshot, zone, Case}` line did **not** have to change.

## 2. The one design rule the create/delete pairs rest on

**Every id-keyed `Model` collection is kept in ascending-id order, and `adjacency_pairs` in ascending
`(surface_a_id, surface_b_id)` order. `create` inserts at the position that keeps that order rather
than appending.**

This is not decoration. `assert_inverse` and `assert_mutation_inverse_law` compare whole documents, so
`delete(x)` followed by its own `create(x)` undo has to put the row back where it was; an append would
move a re-created row to the end and fail the law for every base whose deleted row was not last. The
position is derived from the id, never carried in the payload, so this is not the index addressing
`📓️taxonomy.md:108-114` forbids.

## 3. Model-schema changes other lanes must know about

| change | file | why |
|---|---|---|
| `Fenestration.glazing_construction_id: Option<EntityId>` | `🔨️modules/⚡️simulation/⚙️engine/🔋️model/🦀️.rs` | the layered-glazing escape from the U/SHGC/VLT simplification. W-B measured what that simplification costs: a semio→honeybee→EnergyPlus translation can only emit `WindowMaterial:SimpleGlazingSystem`, worth +5.7…+8.1 % of annual cooling on ANSI/ASHRAE 140 cases 600/900. When the slot names a `Construction`, that stack IS the glazing and the three scalars are the fallback — stated in the `Fenestration` docstring so both sides follow one convention |
| `Model::validate` checks that reference | same | a glazing id that names nothing is now a `severe` diagnostic beside the existing surface/construction checks |
| NEW `OutsideBoundaryKind` unit enum + `OutsideBoundary::{kind, interzone_partner, from_parts}` | same | `dsl::DslScalar` supports unit variants only (`🗣️dsl/✨️derive/🦀️.rs:1626`), so a mutation payload names the boundary through this discriminator and carries `Interzone`'s partner in its own `Option<EntityId>` — the parallel-field shape W-D0's `replace-airflow-network` already established. `from_parts` refuses the two disagreeing combinations, so the union can never be assembled half-formed |
| `SurfaceClass` gained `dsl::DslScalar` | same | it is a payload field type now |
| NEW `ScheduleSet::contains(ScheduleId)` | `🔨️modules/⚡️simulation/⚙️engine/🗓️schedule/🦀️.rs` | the referential-integrity question every `ScheduleId`-carrying mutation asks; the same five-family set `Model::validate` builds. G3/G4 will want it |

Five `Fenestration` literal sites were updated in the same pass — `🏛️bestest/🦀️.rs` (W-C's
`window()`), `🔋️model/🦀️.rs`'s own test, `🧠️precompute/🦀️.rs`'s test, the epJSON importer (W-F) and
the simulation-session projection.

**FOR W-F:** the epJSON importer already carries a dead `let _ = glazing_construction_name(name);`
right after its `Fenestration` literal. That literal now sets `glazing_construction_id: None`; the
hook is yours to wire once the importer materializes the glazing stack as a `Construction`.

## 4. Kinds

### 4.1 Zones & spaces (105–111)

| # | kind | shape | refusals |
|---|---|---|---|
| 105 | `create-zone` 🏘️ | `g1_create` | taken id, blank name, duplicate name, non-positive volume, zero multiplier |
| 106 | `delete-zone` 🏚️ | `g1_delete` | target-missing; `mutation.invariant` while **any** of the 14 `zone_id`-bearing collections, `thermal_enclosures.zone_ids`, `air_loops.terminal_zone_ids` or the airflow network's zone nodes still names it |
| 107 | `create-space` 🪑️ | `g1_create` | taken id, blank name, absent zone, negative floor area |
| 108 | `delete-space` 🧹️ | `g1_delete` | target-missing, still a space-list member |
| 109 | `rename-space` 🔤️ | `g1_scalar` | target-missing, blank, duplicate |
| 110 | `change-space-floor-area` 🧮️ | `g1_scalar` | target-missing, negative |
| 111 | `change-space-zone` 🚚️ | `g1_scalar` | target-missing, absent destination zone |

`delete-zone` RESTRICTs rather than cascades, as the ledger §6 requires — a cascade would have to
delete surfaces, which cascade again to fenestrations and adjacency pairs. The check is wider than the
ledger's "space or surface", because leaving a dangling `thermostat.zone_id` is a `Model::validate`
error too; the Python second implementation drives the same list.

### 4.2 Surfaces (200–210)

| # | kind | notes |
|---|---|---|
| 200 | `create-surface` 🟫️ | full payload including the boundary's two halves; refuses a polygon under three vertices, an absent zone or construction, a self/absent interzone partner, a zero multiplier, and — in the prelude — two halves that disagree |
| 201 | `delete-surface` 🪚️ | **the one cascading delete.** Strips the fenestrations hosted on it and the adjacency pairs naming it, reports `mutation.cascade` at **info** level (`outcomeClasses: ["applied", "info", "error"]`), and its inverse is a multi-step `create-surface` → `create-fenestration`* → `connect-surfaces`*. It still RESTRICTs (`mutation.invariant`) on the one reference a cascade cannot answer for: another surface naming this one as its interzone partner — silently rewriting that surface's boundary condition is a physics decision no delete may take |
| 202 | `rename-surface` 🏳️ | blank, duplicate |
| 203 | `change-surface-zone` 🗜️ | absent destination zone |
| 204 | `change-surface-class` 🧩️ | `SurfaceClass` scalar |
| 205 | `replace-surface-vertices` 🔺️ | `replace` verb, `Vec<[f64; 3]>`, ≥3 vertices |
| 206 | `change-surface-construction` 🧰️ | absent construction |
| 207 | `change-surface-boundary-condition` 🚧️ | hand-written: the union's two halves must agree (`mutation.invalid-payload`) and an `Interzone` partner must be another existing surface |
| 208 | `change-surface-sun-exposed` 🌅️ | — |
| 209 | `change-surface-wind-exposed` 🍃️ | — |
| 210 | `change-surface-multiplier` 🔁️ | zero instances |

### 4.3 Fenestration (211–220, 228–235)

`create-fenestration` 🪟️ (16-field payload, nine guards) / `delete-fenestration` 🚪️ /
`rename-fenestration` 🏁️ / `change-fenestration-surface` 🧲️ / `-u-value` 🌐️ / `-shgc` 🌇️ / `-vlt` 🌈️ /
`-area` 🟥️ / `-frame-conductance` 🖼️ / `-divider-conductance` 🧷️, plus this lane's additions:
`bind-fenestration-glazing-construction` 🧊️ / `clear-fenestration-glazing-construction` 🫗️ and the six
window-geometry kinds W-C's ANSI/ASHRAE 140 fields created — `-height` ⬆️, `-sill-height` ⬇️,
`-overhang-depth` 🧢️ (cases 610/910), `-overhang-offset` 🎩️, `-fin-depth` 🐬️ (cases 630/930),
`-fin-offset` 🐋️.

**Verb note.** The brief asked for "assign/clear" glazing kinds. `assign` is **not** in
`protocol::APPROVED_VERBS` (`📡️spr/🎮️command/🦀️.rs:112-154`) and `#[derive(Mutations)]` asserts
membership at compile time, so the pair is `bind`/`clear` — both approved, and `bind`/`unbind` is
already this vocabulary's own idiom for filling a slot (`bind-weather-file`). `clear` rather than
`unbind` because the partner is a value slot on an entity, not a document-root link.

### 4.4 Shading & adjacency (221–227)

`create-shading-surface` 🌳️ / `delete-shading-surface` 🪵️ / `rename-shading-surface` 🏕️ /
`replace-shading-surface-vertices` 🗺️ / `change-shading-surface-transmittance-schedule` ⛱️ (refuses a
`ScheduleId` the model does not define), and the edge pair `connect-surfaces` 🤝️ /
`disconnect-surfaces` 💔️.

`AdjacencyPair` carries no id (vocabulary §5.5), so the **unordered pair is the address**: connect
refuses a self-pair (`mutation.invariant`), an absent surface (`mutation.target-missing`) and an
already-existing pair in either order (`mutation.duplicate`); disconnect refuses a pair that does not
exist. Both address the outcome at `[surfaceAId, surfaceBId]` rather than a single id.

## 5. Evidence

### 5.1 Generator

```
$ python3 .🧬semio/…/ENERGY-PLUGIN-END-TO-END/🐍️generate-mutation-leaves.py
121 kinds, 242 fixture cases emitted
```

`audit()` passes with all 121 kinds loaded, i.e. no ledger-number, emoji or slug collision with G2's
concurrently-landed 300–444 (an earlier draft of this lane's emoji did collide with G2 on ☀️, 🏗️, 📏️,
🧭️, 🧱️, 🕳️, 🪪️, 🪧️, 🔆️ and 🔅️ — all reassigned before landing, and the final 43 are recorded in
`📓️mutation-tag-ledger.md` and `📓️status.md` so the remaining lanes can avoid them).

### 5.2 Third-party JSON Schema validation

`jsonschema` 4.26.0 out of the repo's own `.venv` (an independent implementation, not ours):

```
$ .venv/bin/python … Draft7Validator.check_schema(…) for every G1 leaf
checked 43 G1 payload schemas + descriptors: all valid draft-07, titles == aggregateVariant
```

Validation of the committed **payload documents** against those schemas is pending the fixture-write
pass (§5.9) — the five JSON files per case are still the generator's `{}` seeds until the Rust
`SEMIO_ENERGY_WRITE_FIXTURES=1` run materializes them.

### 5.3 Frozen message codes — a peer's audit caught this lane, and the disk is already migrated

While this lane was writing, W-D0 added `FROZEN_MESSAGE_CODES` to the generator's `audit()` —
`📋️contract-freeze.md` §C2's seven codes, enforced repo-wide by `bun nx run mutation-outcome-law`.
This lane's first draft used three codes that are NOT in that set (`mutation.duplicate`,
`mutation.target-in-use`, `mutation.invalid-payload`). W-D0 migrated the G1 rows in place and re-ran
the generator, so the disk now carries `mutation.duplicate-id` (9×), `mutation.invariant` (48×),
`mutation.target-missing` (33×), `mutation.cascade` (3×) and `mutation.no-op` (6×) across the region —
Rust and Python both, because the substitution ran over the whole region. Verified after the fact,
and the semantics still read correctly: a RESTRICT refusal (`delete-zone` while a space still names
the zone, `delete-surface` while another surface names it as its interzone partner) is a
`mutation.invariant`, and so is a disagreeing pair of tagged-union halves.

**The generator cannot currently be re-run**: `audit()` now fails on G4's `create-constant-schedule`,
which still reports `mutation.duplicate`. That is G4's row to fix, not this lane's — recorded in
`📓️status.md`. Everything G1 owns is already materialized on disk from the run that preceded G4's
rows, and §5.5's cross-check confirms it.

### 5.4 Parse gate — 1382 generated Rust files, zero syntax errors

With the compiler unavailable (§5.6), the repo's own nightly rustc was used as a pure parser, which
is cheap and needs neither the lock nor the dependency graph:

```
$ for f in <every 🦀️.rs under 🧬️mutations/>; do rustc -Zparse-crate-root-only --edition 2024 "$f"; done
parsed=1382 failed=0
```

plus the mutations aggregate, the crate entry, `🔋️model/🦀️.rs`, `🗓️schedule/🦀️.rs` and
`🏛️bestest/🦀️.rs` — all OK — and `ast.parse` on the Python oracle. This proves syntax, not types.

### 5.5 Wiring cross-check — all 43 kinds, all six registration points

A static check over the emitted files (not over the spec table, so it would catch a generator that
silently skipped a row): for each of the 43 G1 kinds, its aggregate re-export, its enum variant, its
`DIRECTORIES` pair, its `#[path]` crate mount, its `🔮️oracle/🔣️.json` registration, its
`VOCABULARY` row and both `VECTOR_ROOTS` entries in the Python oracle, both `🥒️.feature` rows, every
`vocabulary::…` builder its diff/inverse calls, and all six files of each of its 86 fixture case
directories.

```
G1 kinds: 43 | problems: 0
```

### 5.6 Symbol cross-check against `🔋️model/🦀️.rs` — the class of error a parser cannot see

Field names and call arities were checked against the model's own struct definitions, parsed out of
`🔋️model/🦀️.rs` rather than assumed: every `payload.<field>` in a G1 diff/inverse is a field of that
kind's own payload struct; every `base.model.<name>` is a real `Model` field; every
`existing/item/window/pair.<field>` is a field of the collection's element struct (`terminal_zone_ids`
on `ModelAirLoop`, `zone_ids` on `ThermalEnclosure` and `space_ids` on `SpaceList` are the three the
first pass flagged, and all three exist — `🔋️model/🦀️.rs:589/635/643`); every `vocabulary::<builder>(…)`
call passes exactly as many arguments as that kind declares fields (this is what would break a
`create-fenestration` call after someone adds a sixteenth `Fenestration` field); and every
`crate::model::<Struct> { … }` literal names **all** of that struct's fields. **0 problems across the
43 kinds.** This is not a substitute for `cargo check` — it says nothing about types, traits or
borrowing — but it is the check that would have caught the `glazing_construction_id` fallout in the
five literal sites of §3.

### 5.7 Python second implementation — driven, and it is not a stub

`🐍️g1-drive-python-oracle.py` (kept in this ticket folder) stubs `semio_repo_test`, imports the
generated `🐍️.py`, and drives every G1 kind against a synthetic snapshot document — independent of
the Rust, which is what a second implementation is for:

```
$ .venv/bin/python …/🐍️g1-drive-python-oracle.py
G1 python second implementation: 43/43 kinds move the document, undo restores it exactly,
and every refusal reports a frozen code and leaves the document untouched
```

Per kind it asserts: the happy payload is `applied` and the document actually changes (a stub would
fail here); the handler does not mutate its input; `invert` restores the document **exactly**, which
is what proves §2's ascending-id insertion — `delete-surface`'s cascade undo re-creates a surface, its
window and its adjacency pairs and still lands on the identical document; the refusal payload is
`rejected` with the exact expected frozen code; and a refused step owes no undo steps.

### 5.8 The three repo gates — run, and clean for energy

All three were run in full (they are bun/TypeScript, so the cargo lock does not block them):

```
$ bun nx run mutation-outcome-law
[verify mutation-outcome-law] 44 breach(es)          # ZERO of them in 🔋️energy
$ grep -c "🔋️energy" <the 47 breach lines>
0
```

The 44 are all pre-existing and all somebody else's: `📸️remodel`'s eight private
`mutation.invalid-reconstruction-*` / `mutation.invalid-asset-*` codes, `🌍️gis`'s
`mutation.invalid-number`, `📋️forms`' four `forms.try-value.*`, `🗄️stdio`'s per-subset
`stdio.*.mutation-rejected` family, two `mutation.duplicate` and one `mutation.conflict` in
`🏪️store/🦀️.rs`, a `MergePolicy` parity gap in `📡️spr/🧾️wire`, and a byte-drift between the two
copies of the dsl derive. **The energy vocabulary — all 121 kinds, G1's 43 included — passes the
frozen-code law.**

```
$ bun nx run verify-taxonomy-report      → exit 1
$ bun nx run verify-taxonomy-enforce     → exit 1
error: Normalization requires an explicit repository-boundary decision before authored
       classification: ♻️mit-bestand/🔎️recherche
```

Both taxonomy gates abort on the same unrelated repository-boundary decision, before they reach any
plugin: `grep -ic energy` over either transcript is `0`. So they are red for the whole repository,
not for this lane, and nobody can get a taxonomy verdict on the energy mutation directories until
that boundary decision is made. **Recorded for the coordinator; G1 did not touch it.**

### 5.9 Cargo — NOT YET GREEN, and the reason is the host, not the code

`cargo check -p semio-s-plugin-energy --lib --tests` has produced **no output at all** in this shift.
Measured at 21:27, not assumed:

```
$ ps -eo pid,ppid,%cpu,etime,command | grep "[c]argo check -p semio-s-plugin-energy" | wc -l
14          # 7 cargo invocations, every one of them at 0.0 % CPU
$ uptime
load averages: 54.62 57.51 64.86
$ sysctl vm.swapusage
vm.swapusage: total = 59392.00M  used = 58457.81M  free = 934.19M
```

Every lane is queued on the one `target-energy-e2e` lock on a host with 934 MB of swap left. W-A has
claimed the native check for the whole fleet (`📓️w1-compile.md` §5.2) and reports a pre-existing
191-error wall in `🧪️sim/🦀️.rs` caused by `EnergyJob`'s `Deref` newtype — i.e. the crate did not
compile before this lane touched it either, so a G1 error list cannot be separated from that wall
until W-A lands its fix.

**Therefore, explicitly: the Rust side of this lane is written and self-consistent but NOT yet
compiled, the 86 fixture quintets are NOT yet materialized, the law tests have NOT been run, and the
Python second implementation has NOT been run against committed bytes.** Those four are the
outstanding work; §6 is the exact recipe.

## 6. What the next shift runs, in order

```
RUSTC_WRAPPER= CARGO_TARGET_DIR=/Users/ueli/Documents/semio/target-energy-e2e CARGO_BUILD_JOBS=4 \
  cargo check -p semio-s-plugin-energy --lib --tests --message-format=short
SEMIO_ENERGY_WRITE_FIXTURES=1 RUSTC_WRAPPER= CARGO_TARGET_DIR=… cargo test -p semio-s-plugin-energy
RUSTC_WRAPPER= CARGO_TARGET_DIR=… cargo test -p semio-s-plugin-energy
bun nx run verify-taxonomy-report && bun nx run verify-taxonomy-enforce && bun nx run mutation-outcome-law
```

Two things to expect on the first materialization pass, because they are the only places where the
Rust and the Python second implementation could legitimately disagree, and both are decided by the
committed bytes rather than by either implementation:

1. **`OutsideBoundary`'s wire form.** The Python assumes external tagging — `"OutdoorAir"` for a unit
   arm and `{"Interzone": 3}` for the data arm (W-B's reading, `📓️status.md`). If the `ToValue`
   derive spells it differently, the four places the Python touches
   `row["outside_boundary_condition"]` (`change-surface-boundary-condition`, its inverse,
   `delete-surface`'s RESTRICT check and `create-surface`'s prelude) are the only edits needed.
2. **`Option<T>` in the snapshot document.** The Python assumes a present key with a JSON `null`,
   which is what `airflow_network` and `weatherLink` already do in W-D0's landed vectors.
