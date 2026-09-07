# 🌡️ G3 — HVAC zone-level (500–543) and HVAC loop-level & plant (600–626)

Lane G3 of `26/09/06/ENERGY-PLUGIN-END-TO-END`. 71 mutation kinds, 142 committed fixture cases, all
emitted from one `#region 🔖️G3` block in `🐍️generate-mutation-leaves.py`.

## 0. State the predecessor left (verified on disk, not taken on trust)

The generator was committed at `2d2b39eb7f` (06:04, auto-commit 12:08) with **no G3 region at all** —
`grep "^# region"` found only `🔖️ModelRoot` and `🔖️Zones`. The only surviving trace of the killed
predecessor is in the four type tables at the top of the file (`JSON_TYPES`, `PROTO_TYPES`,
`TS_TYPES`, `GRAPHQL_TYPES`), which already carried exactly this group's needs:
`crate::model::{ZoneEquipmentType, PlantLoopType, SizingType, DesignDayType, RoomAirModelType}`,
`crate::model::ScheduleId`, `Option<crate::model::ScheduleId>`, `Option<f64>` and
`Vec<crate::model::EntityId>`. Those rows were kept; **two of them turned out to be unusable** (§2.1)
and are left in place only because they sit outside every group's region.

Everything else in this report was written from scratch.

## 1. The pattern vocabulary this lane added (for W-D0, the generator custodian)

The spec table's original shape is one `kind(...)` call carrying hand-written Rust for `diff`, Rust
for `inverse`, Python for the second implementation and two typed fixture scenarios. At 71 kinds that
is ~4 500 lines of near-identical prose in which a refusal would have to be restated four times and
could silently drift. The region therefore adds a **general** layer, usable by every group:

### 1.1 `G3Check` — a refusal stated once

```python
G3Check(code, cond, message, path, py_cond, py_path)
```

One object, four renderings: `.rust()` (the diff's early return), `.cond` (folded by `g3_refused()`
into the inverse's "a refused or no-op forward step owes no undo" guard), `.python()` (the second
implementation's branch) and `.final_rust()`/`.final_python()` (the WHOLE body of a kind that can only
ever refuse). Because the Rust guard and the Python branch are generated from the same object, the
`assert_inverse` law and the differential comparison cannot disagree about which payloads are refused.

Twelve builders sit on top of it, each stating one repeated rule in the physics, not in the syntax:

| builder | rule |
|---|---|
| `g3_chk_duplicate(entity)` | a create refuses an id the collection already holds |
| `g3_chk_zone(field)` | a zone-valued foreign key names a zone the model has |
| `g3_chk_reference(coll, display, field)` | any other id-valued foreign key resolves |
| `g3_chk_schedule(field)` | a `ScheduleId` resolves in **any** of the five `ScheduleSet` families |
| `g3_chk_positive` / `g3_chk_non_negative` / `g3_chk_finite` | sign and finiteness of a physical quantity |
| `g3_chk_range(field, key, low, high, what)` | a stated representable interval |
| `g3_chk_fraction` | the `0..=1` special case |
| `g3_chk_at_least_one` | a count or a rank starts at one |
| `g3_chk_blank` / `g3_chk_duplicate_name` | identity fields every report keys on |
| `g3_chk_absent_zero` / `g3_chk_absent_zero_id` | the flattened-`Option` rule (§2.1) |
| `g3_chk_when(flag, check)` | narrows a rule to the payloads a `present` flag says carry a value |
| `g3_chk_id_list` / `g3_chk_zone_list` | membership lists (§2.3) |

### 1.2 `G3Entity` — one collection declared once

```python
G3Entity(coll=…, display=…, struct=…, noun=…, key="id", payload=[(name, rust_ty), …],
         seed=…, bare=…, probe_args=…, ctor=…, recreate=…, py_new=…, py_recreate=…, pre=…)
```

`ctor` (payload → row), `recreate` (row → create arguments), `py_new` and `py_recreate` are **derived**
when the payload field names equal the struct field names, and overridden only by the two entities
whose payload is flattened (`IdealLoadsSystem`, `SetpointManager`). `seed` is the fixture model with
the row present, `bare` the same model without it; `key` is `"id"` everywhere except
`RoomAirModelAssignment`, whose `zone_id` **is** the key (vocabulary §5.5) and which the machinery
handles without a special case.

### 1.3 The seven emitters

`g3_create`, `g3_delete`, `g3_change`, `g3_change_optional`, `g3_rename`, `g3_member_add`,
`g3_member_remove`. Each takes an entity, a ledger number, an emoji, a doc sentence, its checks and
two fixture vectors, and produces the whole `kind(...)` row — Rust diff, Rust inverse, label, target,
outcome classes, wire probe, Python vocabulary function and Python invert arm. `g3_register` derives
the verb, the `record` name (`create → Created…`) and the display name from the ledger slug, so those
three can never disagree with it.

Result: 71 kinds in ~2 200 lines of spec table, of which the per-kind rows are ~1 500 — the physics
(seeds, limits, which reference points where, which value the happy vector moves to) and nothing else.

## 2. Findings every other lane needs

### 2.1 `Option<T>` has no `dsl::DslField` — an optional slot must be flattened

`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️.rs:167` implements `DslField` for `Vec<T>` and for
the primitives; **there is no `impl<T: DslField> DslField for Option<T>`**. A `#[derive(dsl::DslRecord)]`
payload therefore cannot carry `Option<f64>` or `Option<ScheduleId>`, and the generator's
`JSON_TYPES["Option<f64>"]` / `["Option<crate::model::ScheduleId>"]` rows are dead entries.

The repo already has the answer: `replace-airflow-network` (kind 6, W-D0) carries `present: bool`
beside the payload of its optional singleton. This group follows it exactly —
`IdealLoadsSystem::max_heating_capacity_w`, `max_cooling_capacity_w` and
`SetpointManager::schedule_id` travel as a `…_present: bool` flag next to their value, and
`g3_chk_absent_zero` refuses `present == false` with a non-zero value (`mutation.invalid-payload`), so
one payload can never describe two different documents.

### 2.2 A tagged union flattens the same way

`SetpointManagerKind` is `Scheduled | OutdoorAirReset { low_outdoor_c, high_outdoor_c, low_setpoint_c,
high_setpoint_c } | WarmestZone | ColdestZone`. It has no `DslField` either, so
`create-setpoint-manager` and `replace-setpoint-manager-kind` carry the variant's **wire name** as a
`String` beside the four reset limits, and refuse (a) a name that is not one of the four, (b) reset
limits on any variant but `OutdoorAirReset`, (c) a reset whose high outdoor temperature is not above
its low one. The verb is `replace`, not `change`, because the payload shape genuinely differs per
variant.

The union's JSON shape was read off the value derive's own docstring
(`🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️.rs:21-27`), not guessed: a `tag`-less enum with at
least one data-carrying variant is **externally tagged**, so `Scheduled` is the bare string
`"Scheduled"` and the reset variant is `{"OutdoorAirReset": {"low_outdoor_c": …, …}}` with its own
fields in snake_case. The Python implementation constructs exactly that.

### 2.3 A membership list must be kept ascending, or `assert_inverse` fails

`ModelAirLoop::terminal_zone_ids` and `PlantLoopConfig::equipment_ids` are `Vec<EntityId>` with an
`add`/`remove` pair. A naive `remove` (retain) followed by its inverse `add` (push) restores the SET
but not the ORDER, and `fixtures::assert_inverse` compares snapshots for equality. `g3_member_add`
therefore inserts at the ascending position rather than pushing, and `g3_chk_id_list` refuses a create
whose list is not ascending and duplicate-free. The invariant is stated in both leaf docstrings.

### 2.4 The one reference this group cannot check — and what was done about it

`PlantLoopConfig::equipment_ids` (vocabulary §5.4) points at a collection `Model` does not have:
there is no `Chiller`, `Boiler` or `Pump` type anywhere in the schema. `📓️mutation-tag-ledger.md` §6
says to "author them with an explicit `mutation.target-missing` refusal", and that is what this lane
first built — until W-D0's `audit()` grew the rule *every kind needs at least one `✅️` happy and one
`⛔️` refusal fixture*, which a kind that can only ever refuse cannot satisfy.

Settled the other way, and stated rather than hidden: `equipment_ids` is an **opaque** id list.
`add-plant-loop-equipment` (620) and `remove-plant-loop-equipment` (621) enforce only the list's own
shape — ascending, duplicate-free, never the unset id `0` — and `create-plant-loop` does the same. The
missing referential check is written into both leaf docstrings, into `📓️status.md` and here. It is the
**only** unchecked reference in 500–699: zone ids, air-loop ids and schedule ids are all validated,
in Rust and in Python, against the real collections. When a plant-equipment collection lands, replacing
the shape check with a `g3_chk_reference` over it is a one-line change in the spec row.

`remove-plant-loop-equipment` is not decorative even so: it is the repair a document that arrived with
a dangling `equipment_ids` entry needs.

### 2.5 Referential integrity actually enforced here (both implementations)

- every `zone_id` (thermostat, humidistat, ideal loads, zone equipment, daylight zone, sizing object,
  room air model, air-loop terminal list) → `model.zones`
- every `ScheduleId` (thermostat ×2, humidistat ×2, setpoint manager) → all five `ScheduleSet`
  families, the same rule `Model::validate` applies after the fact and the same one
  `✏️editor/🦀️.rs:600`'s `schedule_exists` uses
- `OutdoorAirSystem::air_loop_id` → `model.air_loops`
- `delete-air-loop` refuses while an outdoor air system names the loop, rather than cascading — the
  same call `delete-zone` (105) makes.

### 2.6 Emoji allocation cost three remap rounds

`pathEmojiPolicy.siblingNamespace` is `files-and-directories` inside `🧬️mutations/`, and four lanes
were picking emoji into it simultaneously. 34 + 25 + 8 + 3 collisions had to be resolved as G1's, G2's
and G4's rows landed. All 71 G3 emoji are now written into `📓️mutation-tag-ledger.md` §5 as
`LANDED (G3)`, which is the claim mechanism. Collisions **between the other three lanes** were still
open when this was written and are listed in `📓️status.md` for their owners — `audit()` fails on the
first of them, so the generator cannot be re-run until they settle.

## 3. What was emitted

| range | entity | kinds |
|---|---|---|
| 500–506 | `Thermostat` | create, delete, change-{zone, heating-setpoint-schedule, cooling-setpoint-schedule, heating-throttle-range, cooling-throttle-range} |
| 507–513 | `Humidistat` | create, delete, change-{zone, humidifying-setpoint-schedule, dehumidifying-setpoint-schedule, humidifying-throttle-range, dehumidifying-throttle-range} |
| 514–522 | `IdealLoadsSystem` | create, delete, change-{zone, max-heating-supply-air-temp, min-cooling-supply-air-temp, max-heating-capacity, max-cooling-capacity, outdoor-air-per-person, outdoor-air-per-area} |
| 523–529 | `ZoneEquipmentAssignment` | create, delete, change-{zone, type, priority, heating-capacity, cooling-capacity} |
| 530–535 | `DaylightZoneConfig` | create, delete, change-{zone, illuminance-target, glare-limit, window-transmittance} |
| 536–540 | `SizingObject` | create, delete, change-{zone, sizing-type, design-day-type} |
| 541–543 | `RoomAirModelAssignment` | create, delete, change-room-air-model |
| 600–604 | `SetpointManager` | create, delete, rename, replace-kind, change-schedule |
| 605–612 | `ModelAirLoop` | create, delete, rename, change-{supply-node, return-node, design-supply-air-flow}, add/remove-terminal-zone |
| 613–621 | `PlantLoopConfig` | create, delete, rename, change-{type, supply-temperature, return-temperature, design-flow}, add/remove-equipment |
| 622–626 | `OutdoorAirSystem` | create, delete, change-{air-loop, min-oa-flow, economizer-enabled} |

Every kind ships: the leaf (`🔣️.json`, `🦀️.rs`, `🧬️.schema.json`, `🔺️diff/🦀️.rs`, `↩️inverse/🦀️.rs`),
two fixture case directories (one `✅️` that moves the document, one `⛔️` that is really refused),
a crate `#[path]` mount, a row in all six aggregate surfaces, its oracle-catalog vector, its two
`🥒️.feature` `Examples` rows, its `Vector { … }` in the Rust subject adapter and its function, its
`VOCABULARY` row and its `EXTRA_INVERT` arm in the Python second implementation. The generator run
reports **252 kinds, 504 fixture cases** across all four groups.

## 4. Verification

This crate had **never compiled** when G3 started — the model-root group's own committed fixture files
(`📊️add-output-variable/🧪️tests/…/🔣️.json`) were still the `{}` stubs the generator seeds, because
`SEMIO_ENERGY_WRITE_FIXTURES=1` has never been able to run. Every group's fixtures materialize
together the first time `cargo test` succeeds, and at the time of writing seven concurrent
`cargo check -p semio-s-plugin-energy` processes from the other lanes were queued on the single
`target-energy-e2e` lock.

Two gates that do NOT need the crate to compile were therefore built and run, and both are green.

### 4.1 Payload schemas against a third-party validator — `🐍️verify-g3-payload-schemas.py`

```
$ uv run --with jsonschema python …/🐍️verify-g3-payload-schemas.py
jsonschema 4.26.0 | G3 kinds 71
result: 71 schemas meta-validated, 0 materialized fixture payloads validated, 0 problems
```

All 71 `🧬️.schema.json` are valid draft-07 schemas by `jsonschema` 4.26.0's own `check_schema`, and
every one of the 71 variants is present in the aggregate `🧬️mutations/🔣️.json`'s `$defs`. The script
also validates each materialized fixture payload against its own schema — it reports `0` today only
because the quintets are still stubs; re-run it the moment the fixture pass lands and it becomes the
brief's "schema validated with a third-party JSON Schema validator against the fixture payloads".

### 4.2 Every surface actually reached — `🐍️verify-g3-surfaces.py`

```
$ python3 …/🐍️verify-g3-surfaces.py
71 G3 kinds across 12 surfaces, 142 vectors across 9 artefacts
PASS
```

Reads the EMITTED files back and checks the whole fan-out against the spec table: the crate `#[path]`
mount, the aggregate's variant / `KINDS` / `DIRECTORIES` rows, the protobuf `message` and its `oneof`
field number, the TypeScript interface, the GraphQL type, the grammar production, the Python
`VOCABULARY` row, the oracle catalog and manifest entries, and per vector the `🥒️.feature` `Examples`
row, the Rust subject-adapter `Vector` and all six case files. A kind that is declared in the spec
table but silently missing from one surface is otherwise invisible until a full crate build.

### 4.3 The emitted Rust parses — `rustfmt`

`cargo` cannot be reached (the single `target-energy-e2e` lock has had up to nine peer clients queued
on it all evening), but `rustfmt` parses a file on its own and needs no dependency graph, so it is a
real syntax gate that the contention cannot block:

```
$ while read f; do rustfmt --edition 2021 --emit stdout "$f" >/dev/null || echo "PARSE FAIL $f"; done < g3-rs-files.txt
parse failures: 0        # 355 files: 71 × (🦀️.rs, 🔺️diff/🦀️.rs, ↩️inverse/🦀️.rs) + 142 fixture cases
$ rustfmt --edition 2021 --emit stdout 🧬️mutations/{🦀️.rs,📝️text/🦀️.rs,💾️binary/🦀️.rs} …/🏛️mutate-energy-model-1/🦀️.rs …/🦀️rust/🦀️.rs
OK OK OK OK OK
```

This proves syntax and nothing more — it says nothing about types, trait bounds or whether
`dsl::DslRecord` accepts every payload field. §5 records what is still owed.

### 4.4 The Python second implementation on every G3 vector — `🐍️verify-g3-python-vectors.py`

The differential comparison the ticket is built around only runs after a full crate build. To catch
the wire-name, camelCase and key-shape mistakes that would otherwise hide until then, this harness
rebuilds each vector's `before` document and payload by parsing the SAME typed scenario the generator
emits into the Rust fixture case, converts the Rust literals to their JSON forms, and runs `🐍️.py`'s
own `VOCABULARY` and `invert` over them.

```
$ python3 …/🐍️verify-g3-python-vectors.py
71 G3 kinds, 71 happy vectors, 71 refusal vectors
PASS
```

For every one of the 71 happy vectors the second implementation reaches an `applied` outcome, really
moves the document, owes at least one undo step, and applying that undo lands **exactly** back on
`before`. For every one of the 71 refusal vectors it reaches `rejected` and leaves the document
untouched. It judges only the Python side — the Rust side is `cargo test`'s job — but it is the same
`VOCABULARY`/`invert` the differential test dispatches, over the same scenarios.

It found one real defect: `replace-setpoint-manager-kind`'s Python arm was reading `payload["new_kind"]`
and `payload["new_lowOutdoorC"]` because the flattening helper concatenated the Rust prefix `new_` onto
an already-camelCased tail. Fixed at the source (`g3_py_spm_kind` now runs the whole name through
`field_camel`), regenerated, re-run green.

## 5. Log

- 21:20 — region authored, `audit()` green, generator run: 252 kinds / 504 cases emitted.
- 21:20 — `📓️mutation-tag-ledger.md` §5 rows 500–543 and 600–626 marked `LANDED (G3)` with their emoji.
- 21:21 — `cargo check -p semio-s-plugin-energy --lib --tests` started; blocked on the shared
  `target-energy-e2e` lock behind four peer cargo processes (nine by 22:00, still blocked).
- 21:30 — both crate-independent gates green (§4).
- 21:40 — `bun nx run verify-taxonomy-report` / `verify-taxonomy-enforce` cannot run at all: they abort
  before reaching any mutation directory with `Normalization requires an explicit repository-boundary
  decision before authored classification: ♻️mit-bestand/🔎️recherche`. That path is a **git submodule**
  (`git submodule status` → `92036c7ca0 ♻️mit-bestand/🔎️recherche`), entirely unrelated to this plugin;
  the taxonomy gates are therefore red repo-wide for every lane, not for anything energy did.
- 21:45 — `bun nx run mutation-outcome-law` is red with **44 breaches, none of them in `🔋️energy`**
  (`grep -c energy` over the full run = 0). The breaches are pre-existing and belong to `📸️remodel`,
  `🌍️gis`, `📋️forms`, `🗄️stdio`, `🏪️store`, `📡️spr/🧾️wire` and the `🗣️dsl` derive mirror.
- 21:50 — **the seven frozen message codes.** `📜️script.ts:29117` freezes the whole repo's mutation
  message vocabulary at `mutation.{target-missing, no-op, partial, clamped, duplicate-id, invariant,
  cascade}` — "no per-plugin codes, ever". This region was authored against the model-root group's
  spelling (`mutation.duplicate`, `mutation.invalid-payload`, `mutation.target-in-use`), and a peer's
  repo-wide migration rewrote all three inside the G3 region while this lane was running:
  `duplicate → duplicate-id`, and `invalid-payload`/`target-in-use` → `invariant`. Because every G3
  refusal states its code exactly once in a `G3Check` used by BOTH the Rust and the Python side (§1.1),
  that rewrite could not desynchronize them — verified by regenerating and re-running both gates green.
  The emitted leaves now contain only frozen codes: 328 `target-missing`, 244 `invariant`, 59
  `duplicate-id`, 180 `no-op`.
- 22:00 — generator re-run after peer rows landed: 276 kinds, 552 fixture cases; all three G3 gates
  still green; all 355 emitted G3 Rust files plus the five aggregate/adapter files parse under
  `rustfmt --edition 2021`.

## 6. Still owed, and why it could not be closed here

`cargo check -p semio-s-plugin-energy --lib --tests` was started at 21:21 and was STILL printing only
`Blocking waiting for file lock on build directory` at 22:03 — up to nine concurrent
`cargo check -p semio-s-plugin-energy` clients from the other lanes were queued on the one
`target-energy-e2e` lock, and the process holding it had been in `rustc semio_s_plugin_stdio` for 47
minutes. Nothing in this report claims the crate compiles, and nothing claims a Rust test passed.

What that leaves open, in the order it should be closed once the lock frees:

1. `cargo check -p semio-s-plugin-energy --lib --tests` — types, trait bounds, and in particular
   whether `dsl::DslRecord` accepts every payload field type this group introduced
   (`crate::model::{ZoneEquipmentType, PlantLoopType, SizingType, DesignDayType, RoomAirModelType}`,
   which all carry `dsl::DslScalar`, and `Vec<crate::model::EntityId>`, which rides
   `impl<T: DslField> DslField for Vec<T>`).
2. `SEMIO_ENERGY_WRITE_FIXTURES=1 cargo test -p semio-s-plugin-energy` — materializes all 552 committed
   quintets repo-wide (still `{}` stubs today, W-D0's included).
3. `cargo test -p semio-s-plugin-energy` filtered to 500–699 — the ten laws per fixture case.
4. Re-run `🐍️verify-g3-payload-schemas.py`; with the quintets materialized its "0 materialized fixture
   payloads validated" becomes 142, which is the brief's third-party-JSON-Schema requirement.
5. The Python oracle run and the taxonomy gates — both blocked on things outside this lane (§5's
   21:40/21:45 entries).
