# 🔢️ `s.energy.model` mutation number & emoji ledger

The single allocation authority for the `EnergyModelMutation` fan-out. Every group worker takes its
numbers and its emoji from here and **appends nothing to any other worker's range**, so parallel
groups never collide in the aggregate files.

## 1. What a number is

With the aggregate on `#[derive(dsl::DslEnum)]` (`🧬️mutations/🦀️.rs`) there is **no per-kind
`TEXT_OPCODE` constant, no per-kind `BINARY_TAG` and no `TEXT_OPCODE_REGISTRY`/`BINARY_TAG_REGISTRY`
array** — the text opcode IS the kebab slug and the binary tag IS the variant's ordinal in the enum,
both emitted by the derive. A ledger number is therefore exactly two things:

1. the **protobuf `oneof` field number** in `🧬️mutations/🛰️.proto`'s `message EnergyModelMutation`,
   which must be stable forever and unique across the whole vocabulary;
2. the **sort key** that decides where a kind's lines go in every aggregate file (§4), so two workers
   appending at the same time touch different line ranges.

Each leaf's `🔣️.json` carries `"binaryTag": null` (remodel's own shape) — do not invent a tag.

## 2. Ranges

| Range | Group | Vocabulary section |
|---|---|---|
| `1`–`99` | Model root | §4.1 + §4.12 + §4.10 |
| `100`–`199` | Zones & spaces | §4.2 |
| `200`–`299` | Geometry & envelope | §4.3 |
| `300`–`399` | Constructions & materials | §4.4 |
| `400`–`499` | Internal gains | §4.5 |
| `500`–`599` | HVAC zone-level | §4.6 |
| `600`–`699` | HVAC loop-level & plant | §4.7 |
| `700`–`799` | Electrical, renewables & other systems | §4.8 |
| `800`–`899` | Grouping collections | §4.9 |
| `900`–`999` | Schedules | §5.1 |

A group that needs more than its 100 numbers takes the next free hundred and records it here in the
same commit; it never borrows from a neighbour.

## 3. Emoji rules (load-bearing — read before picking one)

- `taxonomy.json`'s `mutationDirectoryPattern` is `^.+️[a-z][a-z0-9]*(?:-[a-z0-9]+)+$`. The
  **U+FE0F variation selector is mandatory** and many emoji render identically with and without it:
  `🏷️rename-zone` matches, `🏷rename-zone` does not, and a directory that does not match is invisible
  to every taxonomy discovery tool. Always paste the emoji and then append U+FE0F explicitly.
- The name must be NFC-normalized (`🔍️discovery/🟦️.ts` checks `name === name.normalize("NFC")`).
- `pathEmojiPolicy.siblingNamespace` is `files-and-directories`: **a kind's emoji must be unique among
  every sibling inside `🧬️mutations/`**, files included. Already taken by non-kind siblings:
  `💾️` `📖️` `📝️` `🔗️` `🔣️` `🛰️` `🟦️` `🦀️`. Already taken by landed kinds: see §5.
- Inside a kind's own `🧪️tests/`, the case directories share one namespace: this ticket uses `✅️` for
  the vector that moves the document and `⛔️` for the vector that is refused. Reuse those two —
  different kinds have different parents, so they do not collide.
- Keep the case slug short. A repo-wide sweep (`26/04/08/ENFORCE-UNIQUE-SEMANTIC-EMOJIS…`'s
  `🪟️shorten-long-paths.ts`) renames any `🧪️tests/<case>/…` path longer than 190 UTF-16 units to
  `<truncated>-<6 hex>` and does NOT fix the `#[path]`/`include_str!`/`asset://` literals that name
  it. The prefix up to `🧬️mutations/` is already 105 units, so budget accordingly.

## 4. How to insert your lines without conflicting

Every aggregate file is ordered by ledger number, ascending, and grouped by range. Insert your
group's block **between the neighbouring ranges**, never at the end of the file:

| File | One line per kind | Ordering |
|---|---|---|
| `🧬️mutations/🦀️.rs` | `pub use super::<module>::{<builder>, <Variant>};` | alphabetical inside `//#region 🔖️Reexports` (Rust does not care; the region is sorted so diffs stay local) |
| `🧬️mutations/🦀️.rs` | `    <Variant>(<Variant>),` | **ledger number** — this order IS the binary ordinal |
| `🧬️mutations/🦀️.rs` | `    "<slug>",` in `KINDS` | ledger number (must equal the enum order) |
| `🧬️mutations/🦀️.rs` | `    ("<slug>", "<emoji><slug>"),` in `DIRECTORIES` | ledger number |
| `🧬️mutations/🦀️.rs` | one `wire_probes()` literal | ledger number |
| `🧬️mutations/🔣️.json` | one `{"$ref": "#/$defs/<Variant>"}` + one `$defs` entry | ledger number |
| `🧬️mutations/🟦️.ts` | one `export interface <Variant>` + one union member | ledger number |
| `🧬️mutations/🔗️.graphql` | one `type <Variant>` + one union member | ledger number |
| `🧬️mutations/🛰️.proto` | one `message <Variant>` + one `oneof kind` entry `= <number>;` | ledger number |
| `🧬️mutations/📖️.grammar.semio` | one `op = … \| <slug>` alternative + one production | ledger number |
| `📦️packages/🦀️rust/🦀️.rs` | one `pub mod <module> { … }` mount block | ledger number |
| `✳️any/🔮️oracle/🔣️.json` | one `mutationCatalogs[0].vectors[]` entry, one `kinds[]` entry, one `mutationManifests[0].mutations[]` entry | ledger number |
| `✳️any/🧪️tests/🏛️mutate-energy-model-1/🥒️.feature` | two `Examples` rows per fixture case | ledger number |
| `✳️any/🧪️tests/🏛️mutate-energy-model-1/🦀️.rs` | one `Vector { … }` per fixture case | ledger number |
| `✳️any/🧪️tests/🏛️mutate-energy-model-1/🐍️.py` | one `VECTOR_ROOTS` entry per case, one vocabulary fn, one `VOCABULARY` row, one `invert` arm | ledger number |

The mechanical way to do all of it is `🐍️generate-mutation-leaves.py` in this ticket folder: add your
group's `kind(...)` entries to its spec table (keeping the `number=` from this ledger) and re-run it
from the repository root. It rewrites every file in the table above from the one spec, so two workers
who both re-run it after adding only their own entries produce the same file, not a conflict.

## 5. Allocation

### Model root — `1`–`100` (§4.1 + §4.12 + §4.10)

| # | kind | emoji | status |
|---|---|---|---|
| | **Model / document root** | | |
| 1 | `rename-model` | 🏷️ | LANDED (W-D0) |
| 2 | `change-model-version` | 🔢️ | LANDED (W-D0) |
| 3 | `update-site` | 🌍️ | LANDED (W-D0) |
| 4 | `update-ground-temperature` | 🌡️ | LANDED (W-D0) |
| 5 | `update-run-period` | 📅️ | LANDED (W-D0) |
| 6 | `replace-airflow-network` | 🫧️ | LANDED (W-D0) |
| 7 | `add-output-variable` | 📊️ | LANDED (W-D0) |
| 8 | `remove-output-variable` | 📉️ | LANDED (W-D0) |
| 9 | `bind-weather-file` | 🌦️ | LANDED (W-D0) |
| 10 | `unbind-weather-file` | 🌤️ | LANDED (W-D0) |
| 11 | `connect-referenced-model` | 🪢️ | LANDED (W-D0) |
| 12 | `disconnect-referenced-model` | ✂️ | LANDED (W-D0) |

### Zones & spaces — `100`–`199` (§4.2)

| # | kind | emoji | status |
|---|---|---|---|
| | **Zone** | | |
| 100 | `rename-zone` | 🏠️ | LANDED (W-D0) |
| 101 | `change-zone-volume` | 📦️ | LANDED (W-D0) |
| 102 | `change-zone-multiplier` | ✖️ | LANDED (W-D0) |
| 103 | `change-zone-conditioned` | 🌬️ | LANDED (W-D0) |
| 104 | `change-zone-floor-area-participation` | 📐️ | LANDED (W-D0) |
| 105 | `create-zone` | — | reserved |
| 106 | `delete-zone` | — | reserved |
| | **Space** | | |
| 107 | `create-space` | — | reserved |
| 108 | `delete-space` | — | reserved |
| 109 | `rename-space` | — | reserved |
| 110 | `change-space-floor-area` | — | reserved |
| 111 | `change-space-zone` | — | reserved |

### Geometry & envelope — `200`–`299` (§4.3)

| # | kind | emoji | status |
|---|---|---|---|
| | **Surface** | | |
| 200 | `create-surface` | — | reserved |
| 201 | `delete-surface` | — | reserved |
| 202 | `rename-surface` | — | reserved |
| 203 | `change-surface-zone` | — | reserved |
| 204 | `change-surface-class` | — | reserved |
| 205 | `replace-surface-vertices` | — | reserved |
| 206 | `change-surface-construction` | — | reserved |
| 207 | `change-surface-boundary-condition` | — | reserved |
| 208 | `change-surface-sun-exposed` | — | reserved |
| 209 | `change-surface-wind-exposed` | — | reserved |
| 210 | `change-surface-multiplier` | — | reserved |
| | **Fenestration** | | |
| 211 | `create-fenestration` | — | reserved |
| 212 | `delete-fenestration` | — | reserved |
| 213 | `rename-fenestration` | — | reserved |
| 214 | `change-fenestration-surface` | — | reserved |
| 215 | `change-fenestration-u-value` | — | reserved |
| 216 | `change-fenestration-shgc` | — | reserved |
| 217 | `change-fenestration-vlt` | — | reserved |
| 218 | `change-fenestration-area` | — | reserved |
| 219 | `change-fenestration-frame-conductance` | — | reserved |
| 220 | `change-fenestration-divider-conductance` | — | reserved |
| | **ShadingSurface** | | |
| 221 | `create-shading-surface` | — | reserved |
| 222 | `delete-shading-surface` | — | reserved |
| 223 | `rename-shading-surface` | — | reserved |
| 224 | `replace-shading-surface-vertices` | — | reserved |
| 225 | `change-shading-surface-transmittance-schedule` | — | reserved |
| | **AdjacencyPair** | | |
| 226 | `connect-surfaces` | — | reserved |
| 227 | `disconnect-surfaces` | — | reserved |

### Constructions & materials — `300`–`399` (§4.4)

| # | kind | emoji | status |
|---|---|---|---|
| | **Material** | | |
| 300 | `create-material` | — | reserved |
| 301 | `delete-material` | — | reserved |
| 302 | `rename-material` | — | reserved |
| 303 | `change-material-thickness` | — | reserved |
| 304 | `change-material-conductivity` | — | reserved |
| 305 | `change-material-density` | — | reserved |
| 306 | `change-material-specific-heat` | — | reserved |
| 307 | `change-material-thermal-absorptance` | — | reserved |
| 308 | `change-material-solar-absorptance` | — | reserved |
| 309 | `change-material-visible-absorptance` | — | reserved |
| | **Construction** | | |
| 310 | `create-construction` | — | reserved |
| 311 | `delete-construction` | — | reserved |
| 312 | `rename-construction` | — | reserved |
| 313 | `add-construction-layer` | — | reserved |
| 314 | `remove-construction-layer` | — | reserved |
| 315 | `reorder-construction-layers` | — | reserved |

### Internal gains — `400`–`499` (§4.5)

| # | kind | emoji | status |
|---|---|---|---|
| | **PeopleGain** | | |
| 400 | `create-people-gain` | — | reserved |
| 401 | `delete-people-gain` | — | reserved |
| 402 | `change-people-gain-zone` | — | reserved |
| 403 | `change-people-gain-schedule` | — | reserved |
| 404 | `change-people-gain-activity-schedule` | — | reserved |
| 405 | `change-people-gain-people-per-area` | — | reserved |
| 406 | `change-people-gain-sensible-fraction` | — | reserved |
| 407 | `change-people-gain-latent-fraction` | — | reserved |
| 408 | `change-people-gain-radiant-fraction` | — | reserved |
| | **LightingGain** | | |
| 409 | `create-lighting-gain` | — | reserved |
| 410 | `delete-lighting-gain` | — | reserved |
| 411 | `change-lighting-gain-zone` | — | reserved |
| 412 | `change-lighting-gain-schedule` | — | reserved |
| 413 | `change-lighting-gain-watts-per-area` | — | reserved |
| 414 | `change-lighting-gain-radiant-fraction` | — | reserved |
| 415 | `change-lighting-gain-visible-fraction` | — | reserved |
| 416 | `change-lighting-gain-return-air-fraction` | — | reserved |
| | **EquipmentGain** | | |
| 417 | `create-equipment-gain` | — | reserved |
| 418 | `delete-equipment-gain` | — | reserved |
| 419 | `change-equipment-gain-zone` | — | reserved |
| 420 | `change-equipment-gain-schedule` | — | reserved |
| 421 | `change-equipment-gain-watts-per-area` | — | reserved |
| 422 | `change-equipment-gain-radiant-fraction` | — | reserved |
| 423 | `change-equipment-gain-latent-fraction` | — | reserved |
| | **Infiltration** | | |
| 424 | `create-infiltration` | — | reserved |
| 425 | `delete-infiltration` | — | reserved |
| 426 | `change-infiltration-zone` | — | reserved |
| 427 | `change-infiltration-schedule` | — | reserved |
| 428 | `change-infiltration-flow-per-exterior-area` | — | reserved |
| 429 | `change-infiltration-constant-term-coefficient` | — | reserved |
| 430 | `change-infiltration-temperature-term-coefficient` | — | reserved |
| 431 | `change-infiltration-velocity-term-coefficient` | — | reserved |
| 432 | `change-infiltration-velocity-squared-term-coefficient` | — | reserved |
| | **MechanicalVentilation** | | |
| 433 | `create-mechanical-ventilation` | — | reserved |
| 434 | `delete-mechanical-ventilation` | — | reserved |
| 435 | `change-mechanical-ventilation-zone` | — | reserved |
| 436 | `change-mechanical-ventilation-schedule` | — | reserved |
| 437 | `change-mechanical-ventilation-design-flow` | — | reserved |
| 438 | `change-mechanical-ventilation-fan-total-efficiency` | — | reserved |
| 439 | `change-mechanical-ventilation-fan-delta-pressure` | — | reserved |

### HVAC zone-level — `500`–`599` (§4.6)

| # | kind | emoji | status |
|---|---|---|---|
| | **Thermostat** | | |
| 500 | `create-thermostat` | — | reserved |
| 501 | `delete-thermostat` | — | reserved |
| 502 | `change-thermostat-zone` | — | reserved |
| 503 | `change-thermostat-heating-setpoint-schedule` | — | reserved |
| 504 | `change-thermostat-cooling-setpoint-schedule` | — | reserved |
| 505 | `change-thermostat-heating-throttle-range` | — | reserved |
| 506 | `change-thermostat-cooling-throttle-range` | — | reserved |
| | **Humidistat** | | |
| 507 | `create-humidistat` | — | reserved |
| 508 | `delete-humidistat` | — | reserved |
| 509 | `change-humidistat-zone` | — | reserved |
| 510 | `change-humidistat-humidifying-setpoint-schedule` | — | reserved |
| 511 | `change-humidistat-dehumidifying-setpoint-schedule` | — | reserved |
| 512 | `change-humidistat-humidifying-throttle-range` | — | reserved |
| 513 | `change-humidistat-dehumidifying-throttle-range` | — | reserved |
| | **IdealLoadsSystem** | | |
| 514 | `create-ideal-loads-system` | — | reserved |
| 515 | `delete-ideal-loads-system` | — | reserved |
| 516 | `change-ideal-loads-system-zone` | — | reserved |
| 517 | `change-ideal-loads-system-max-heating-supply-air-temp` | — | reserved |
| 518 | `change-ideal-loads-system-min-cooling-supply-air-temp` | — | reserved |
| 519 | `change-ideal-loads-system-max-heating-capacity` | — | reserved |
| 520 | `change-ideal-loads-system-max-cooling-capacity` | — | reserved |
| 521 | `change-ideal-loads-system-outdoor-air-per-person` | — | reserved |
| 522 | `change-ideal-loads-system-outdoor-air-per-area` | — | reserved |
| | **ZoneEquipmentAssignment** | | |
| 523 | `create-zone-equipment` | — | reserved |
| 524 | `delete-zone-equipment` | — | reserved |
| 525 | `change-zone-equipment-zone` | — | reserved |
| 526 | `change-zone-equipment-type` | — | reserved |
| 527 | `change-zone-equipment-priority` | — | reserved |
| 528 | `change-zone-equipment-heating-capacity` | — | reserved |
| 529 | `change-zone-equipment-cooling-capacity` | — | reserved |
| | **DaylightZoneConfig** | | |
| 530 | `create-daylight-zone` | — | reserved |
| 531 | `delete-daylight-zone` | — | reserved |
| 532 | `change-daylight-zone-zone` | — | reserved |
| 533 | `change-daylight-zone-illuminance-target` | — | reserved |
| 534 | `change-daylight-zone-glare-limit` | — | reserved |
| 535 | `change-daylight-zone-window-transmittance` | — | reserved |
| | **SizingObject** | | |
| 536 | `create-sizing-object` | — | reserved |
| 537 | `delete-sizing-object` | — | reserved |
| 538 | `change-sizing-object-zone` | — | reserved |
| 539 | `change-sizing-object-sizing-type` | — | reserved |
| 540 | `change-sizing-object-design-day-type` | — | reserved |
| | **RoomAirModelAssignment** | | |
| 541 | `create-room-air-model-assignment` | — | reserved |
| 542 | `delete-room-air-model-assignment` | — | reserved |
| 543 | `change-room-air-model` | — | reserved |

### HVAC loop-level & plant — `600`–`699` (§4.7)

| # | kind | emoji | status |
|---|---|---|---|
| | **SetpointManager** | | |
| 600 | `create-setpoint-manager` | — | reserved |
| 601 | `delete-setpoint-manager` | — | reserved |
| 602 | `rename-setpoint-manager` | — | reserved |
| 603 | `replace-setpoint-manager-kind` | — | reserved |
| 604 | `change-setpoint-manager-schedule` | — | reserved |
| | **ModelAirLoop** | | |
| 605 | `create-air-loop` | — | reserved |
| 606 | `delete-air-loop` | — | reserved |
| 607 | `rename-air-loop` | — | reserved |
| 608 | `change-air-loop-supply-node` | — | reserved |
| 609 | `change-air-loop-return-node` | — | reserved |
| 610 | `change-air-loop-design-supply-air-flow` | — | reserved |
| 611 | `add-air-loop-terminal-zone` | — | reserved |
| 612 | `remove-air-loop-terminal-zone` | — | reserved |
| | **PlantLoopConfig** | | |
| 613 | `create-plant-loop` | — | reserved |
| 614 | `delete-plant-loop` | — | reserved |
| 615 | `rename-plant-loop` | — | reserved |
| 616 | `change-plant-loop-type` | — | reserved |
| 617 | `change-plant-loop-supply-temperature` | — | reserved |
| 618 | `change-plant-loop-return-temperature` | — | reserved |
| 619 | `change-plant-loop-design-flow` | — | reserved |
| 620 | `add-plant-loop-equipment` | — | reserved |
| 621 | `remove-plant-loop-equipment` | — | reserved |
| | **OutdoorAirSystem** | | |
| 622 | `create-outdoor-air-system` | — | reserved |
| 623 | `delete-outdoor-air-system` | — | reserved |
| 624 | `change-outdoor-air-system-air-loop` | — | reserved |
| 625 | `change-outdoor-air-system-min-oa-flow` | — | reserved |
| 626 | `change-outdoor-air-system-economizer-enabled` | — | reserved |

### Electrical, renewables & other systems — `700`–`799` (§4.8)

| # | kind | emoji | status |
|---|---|---|---|
| | **ElectricalLoadCenter** | | |
| 700 | `create-electrical-load-center` | — | reserved |
| 701 | `delete-electrical-load-center` | — | reserved |
| 702 | `rename-electrical-load-center` | — | reserved |
| 703 | `add-electrical-load-center-generator` | — | reserved |
| 704 | `remove-electrical-load-center-generator` | — | reserved |
| 705 | `add-electrical-load-center-pv` | — | reserved |
| 706 | `remove-electrical-load-center-pv` | — | reserved |
| 707 | `add-electrical-load-center-battery` | — | reserved |
| 708 | `remove-electrical-load-center-battery` | — | reserved |
| | **PvSystemAssignment** | | |
| 709 | `create-pv-system` | — | reserved |
| 710 | `delete-pv-system` | — | reserved |
| 711 | `change-pv-system-dc-capacity` | — | reserved |
| 712 | `change-pv-system-area` | — | reserved |
| 713 | `change-pv-system-tilt` | — | reserved |
| 714 | `change-pv-system-azimuth` | — | reserved |
| 715 | `change-pv-system-module-efficiency` | — | reserved |
| 716 | `change-pv-system-inverter-efficiency` | — | reserved |
| | **BatteryAssignment** | | |
| 717 | `create-battery` | — | reserved |
| 718 | `delete-battery` | — | reserved |
| 719 | `change-battery-capacity` | — | reserved |
| 720 | `change-battery-max-charge` | — | reserved |
| 721 | `change-battery-max-discharge` | — | reserved |
| 722 | `change-battery-round-trip-efficiency` | — | reserved |
| | **ShwSystemConfig** | | |
| 723 | `create-shw-system` | — | reserved |
| 724 | `delete-shw-system` | — | reserved |
| 725 | `change-shw-system-heater-capacity` | — | reserved |
| 726 | `change-shw-system-storage-volume` | — | reserved |
| 727 | `change-shw-system-setpoint` | — | reserved |
| 728 | `change-shw-system-schedule` | — | reserved |
| | **SolarThermalConfig** | | |
| 729 | `create-solar-thermal-system` | — | reserved |
| 730 | `delete-solar-thermal-system` | — | reserved |
| 731 | `change-solar-thermal-system-collector-area` | — | reserved |
| 732 | `change-solar-thermal-system-efficiency` | — | reserved |
| 733 | `change-solar-thermal-system-storage-volume` | — | reserved |
| 734 | `change-solar-thermal-system-tilt` | — | reserved |
| 735 | `change-solar-thermal-system-azimuth` | — | reserved |
| | **RefrigerationConfig** | | |
| 736 | `create-refrigeration-system` | — | reserved |
| 737 | `delete-refrigeration-system` | — | reserved |
| 738 | `change-refrigeration-system-case-count` | — | reserved |
| 739 | `change-refrigeration-system-design-load` | — | reserved |
| 740 | `change-refrigeration-system-defrost-schedule` | — | reserved |
| | **WaterSystemConfig** | | |
| 741 | `create-water-system` | — | reserved |
| 742 | `delete-water-system` | — | reserved |
| 743 | `change-water-system-fixture-count` | — | reserved |
| 744 | `change-water-system-peak-flow` | — | reserved |
| 745 | `change-water-system-schedule` | — | reserved |
| | **FaultDefinition** | | |
| 746 | `create-fault` | — | reserved |
| 747 | `delete-fault` | — | reserved |
| 748 | `change-fault-target-equipment` | — | reserved |
| 749 | `change-fault-type` | — | reserved |
| 750 | `change-fault-severity` | — | reserved |
| 751 | `change-fault-start-schedule` | — | reserved |

### Grouping collections — `800`–`899` (§4.9)

| # | kind | emoji | status |
|---|---|---|---|
| | **SpaceList** | | |
| 800 | `create-space-list` | — | reserved |
| 801 | `delete-space-list` | — | reserved |
| 802 | `rename-space-list` | — | reserved |
| 803 | `add-space-list-member` | — | reserved |
| 804 | `remove-space-list-member` | — | reserved |
| | **ThermalEnclosure** | | |
| 805 | `create-thermal-enclosure` | — | reserved |
| 806 | `delete-thermal-enclosure` | — | reserved |
| 807 | `rename-thermal-enclosure` | — | reserved |
| 808 | `add-thermal-enclosure-zone` | — | reserved |
| 809 | `remove-thermal-enclosure-zone` | — | reserved |

### Schedules — `900`–`999` (§5.1 (new — schedules are now persisted `Model` data))

| # | kind | emoji | status |
|---|---|---|---|
| | **ConstantSchedule** | | |
| 900 | `create-constant-schedule` | — | reserved |
| 901 | `delete-constant-schedule` | — | reserved |
| 902 | `change-constant-schedule-value` | — | reserved |
| | **DailySchedule** | | |
| 903 | `create-daily-schedule` | — | reserved |
| 904 | `delete-daily-schedule` | — | reserved |
| 905 | `replace-daily-schedule-hourly-values` | — | reserved |
| 906 | `change-daily-schedule-interpolation` | — | reserved |
| 907 | `change-daily-schedule-limits` | — | reserved |
| | **WeeklySchedule** | | |
| 908 | `create-weekly-schedule` | — | reserved |
| 909 | `delete-weekly-schedule` | — | reserved |
| 910 | `change-weekly-schedule-day` | — | reserved |
| | **AnnualSchedule** | | |
| 911 | `create-annual-schedule` | — | reserved |
| 912 | `delete-annual-schedule` | — | reserved |
| 913 | `add-annual-schedule-rule` | — | reserved |
| 914 | `remove-annual-schedule-rule` | — | reserved |
| 915 | `reorder-annual-schedule-rules` | — | reserved |
| 916 | `change-annual-schedule-default-daily-schedule` | — | reserved |
| 917 | `change-annual-schedule-holiday-daily-schedule` | — | reserved |
| 918 | `add-annual-schedule-holiday` | — | reserved |
| 919 | `remove-annual-schedule-holiday` | — | reserved |
| | **TimeSeriesSchedule** | | |
| 920 | `create-time-series-schedule` | — | reserved |
| 921 | `delete-time-series-schedule` | — | reserved |
| 922 | `replace-time-series-schedule-values` | — | reserved |
| 923 | `change-time-series-schedule-timestep` | — | reserved |

<!-- total 265 -->

## 6. Notes on the reserved rows

- **`replace-model` is not in this ledger and never will be.** `📓️derivation-rules.md` rule 6 bans a
  document-level whole-content replace inside the mutation enum; whole-model load (file open, import,
  load-example) goes through `store::ArtifactStore::reset`.
- **Counts.** The vocabulary report's §4.13 total of 238 double-counts: its "Model root 16" adds
  §4.12's five kinds to a §4.1 it counts as ten while §4.1 lists five. The enumeration above is the
  corrected one — 265 rows, including the schedule group §4.13 never counted at all (schedules only
  became persistable in this ticket) and the two §4.10 output kinds folded into the model root.
- **`replace-airflow-network` carries parallel `zoneIds`/`nodeIds` lists** because
  `AirflowNetworkDefinition::zone_node_ids` is a `Vec<(EntityId, u32)>` and a tuple has no
  `dsl::DslField`. Follow-up for whoever owns the engine model next: turn that field into a named
  record (`AirflowNetworkZoneNode { zone_id, node_id }`) and collapse the payload to one list.
- **`add-plant-loop-equipment` / `remove-plant-loop-equipment` (`607`, `608`) and
  `add-/remove-electrical-load-center-generator` (`703`, `704`) are speculative.**
  `PlantLoopConfig::equipment_ids` and `ElectricalLoadCenter::generator_ids` reference collections
  that do not exist in `Model` at all (vocabulary §5.4). Do not author them until a backing collection
  lands, or author them with an explicit `mutation.target-missing` refusal and say so in the leaf.
- **`change-fault-target-equipment` (`757`)** addresses an untyped `EntityId` with no discriminator;
  it cannot be given a real referential-integrity check until `FaultDefinition::target_equipment_id`
  becomes a tagged reference.
- **`delete-zone` (`105`) must REFUSE while any space or surface still references the zone**
  (`mutation.target-in-use`), rather than cascading — the cascade would have to delete surfaces,
  which cascade to fenestrations and adjacency pairs. `delete-surface` (`201`) DOES cascade, stripping
  the fenestrations and adjacency pairs that address it and emitting `mutation.cascade` at info level
  (`📸️remodel`'s `🪓delete-stream` is the worked reference).
