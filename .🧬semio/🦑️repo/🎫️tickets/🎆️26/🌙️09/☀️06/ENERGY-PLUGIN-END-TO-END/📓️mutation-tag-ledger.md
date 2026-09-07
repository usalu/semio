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
- Keep the case slug short, but the exact number was wrong here and is now MEASURED (W-D0). The
  sweep this rule cited (`26/04/08/ENFORCE-UNIQUE-SEMANTIC-EMOJIS…`'s `🪟️shorten-long-paths.ts`)
  targets 190 UTF-16 units and does NOT fix the `#[path]`/`include_str!`/`asset://` literals it
  renames — but it is a **one-off script in that ticket's folder, not a live gate**: 1616 tracked
  paths already exceed 190 and the repository maximum is 227 (`🧩️puzzle`'s own mutation fixtures
  reach 225). 190 is also unreachable here: the prefix up to `🧬️mutations/` is 105 units and
  `📐️change-zone-floor-area-participation` alone spends 39 more, leaving 5 for a case name.
  The generator therefore enforces **227** — "never become the repository's new worst path" — and
  fails loudly above it. Energy's longest case path today is 208.

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
| 105 | `create-zone` | 🏘️ | LANDED (G1) |
| 106 | `delete-zone` | 🏚️ | LANDED (G1) |
| | **Space** | | |
| 107 | `create-space` | 🪑️ | LANDED (G1) |
| 108 | `delete-space` | 🧹️ | LANDED (G1) |
| 109 | `rename-space` | 🔤️ | LANDED (G1) |
| 110 | `change-space-floor-area` | 🧮️ | LANDED (G1) |
| 111 | `change-space-zone` | 🚚️ | LANDED (G1) |

### Geometry & envelope — `200`–`299` (§4.3)

| # | kind | emoji | status |
|---|---|---|---|
| | **Surface** | | |
| 200 | `create-surface` | 🟫️ | LANDED (G1) |
| 201 | `delete-surface` | 🪚️ | LANDED (G1) |
| 202 | `rename-surface` | 🏳️ | LANDED (G1) |
| 203 | `change-surface-zone` | 🗜️ | LANDED (G1) |
| 204 | `change-surface-class` | 🧩️ | LANDED (G1) |
| 205 | `replace-surface-vertices` | 🔺️ | LANDED (G1) |
| 206 | `change-surface-construction` | 🧰️ | LANDED (G1) |
| 207 | `change-surface-boundary-condition` | 🚧️ | LANDED (G1) |
| 208 | `change-surface-sun-exposed` | 🌅️ | LANDED (G1) |
| 209 | `change-surface-wind-exposed` | 🍃️ | LANDED (G1) |
| 210 | `change-surface-multiplier` | 🔁️ | LANDED (G1) |
| | **Fenestration** | | |
| 211 | `create-fenestration` | 🪟️ | LANDED (G1) |
| 212 | `delete-fenestration` | 🚪️ | LANDED (G1) |
| 213 | `rename-fenestration` | 🏁️ | LANDED (G1) |
| 214 | `change-fenestration-surface` | 🧲️ | LANDED (G1) |
| 215 | `change-fenestration-u-value` | 🌐️ | LANDED (G1) |
| 216 | `change-fenestration-shgc` | 🌇️ | LANDED (G1) |
| 217 | `change-fenestration-vlt` | 🌈️ | LANDED (G1) |
| 218 | `change-fenestration-area` | 🟥️ | LANDED (G1) |
| 219 | `change-fenestration-frame-conductance` | 🖼️ | LANDED (G1) |
| 220 | `change-fenestration-divider-conductance` | 🧷️ | LANDED (G1) |
| | **ShadingSurface** | | |
| 221 | `create-shading-surface` | 🌳️ | LANDED (G1) |
| 222 | `delete-shading-surface` | 🪵️ | LANDED (G1) |
| 223 | `rename-shading-surface` | 🏕️ | LANDED (G1) |
| 224 | `replace-shading-surface-vertices` | 🗺️ | LANDED (G1) |
| 225 | `change-shading-surface-transmittance-schedule` | ⛱️ | LANDED (G1) |
| | **AdjacencyPair** | | |
| 226 | `connect-surfaces` | 🤝️ | LANDED (G1) |
| 227 | `disconnect-surfaces` | 💔️ | LANDED (G1) |
| | **Fenestration glazing & attached shading (added by G1)** | | |
| 228 | `bind-fenestration-glazing-construction` | 🧊️ | LANDED (G1) |
| 229 | `clear-fenestration-glazing-construction` | 🫗️ | LANDED (G1) |
| 230 | `change-fenestration-height` | ⬆️ | LANDED (G1) |
| 231 | `change-fenestration-sill-height` | ⬇️ | LANDED (G1) |
| 232 | `change-fenestration-overhang-depth` | 🧢️ | LANDED (G1) |
| 233 | `change-fenestration-overhang-offset` | 🎩️ | LANDED (G1) |
| 234 | `change-fenestration-fin-depth` | 🐬️ | LANDED (G1) |
| 235 | `change-fenestration-fin-offset` | 🐋️ | LANDED (G1) |

### Constructions & materials — `300`–`399` (§4.4)

| # | kind | emoji | status |
|---|---|---|---|
| | **Material** | | |
| 300 | `create-material` | 🧱️ | LANDED (G2) |
| 301 | `delete-material` | 🪨️ | LANDED (G2) |
| 302 | `rename-material` | 🪧️ | LANDED (G2) |
| 303 | `change-material-thickness` | 📏️ | LANDED (G2) |
| 304 | `change-material-conductivity` | 🔥️ | LANDED (G2) |
| 305 | `change-material-density` | ⚖️ | LANDED (G2) |
| 306 | `change-material-specific-heat` | ♨️ | LANDED (G2) |
| 307 | `change-material-thermal-absorptance` | 🔆️ | LANDED (G2) |
| 308 | `change-material-solar-absorptance` | ☀️ | LANDED (G2) |
| 309 | `change-material-visible-absorptance` | 👁️ | LANDED (G2) |
| | **Construction** | | |
| 310 | `create-construction` | 🏗️ | LANDED (G2) |
| 311 | `delete-construction` | 🧨️ | LANDED (G2) |
| 312 | `rename-construction` | 🪪️ | LANDED (G2) |
| 313 | `add-construction-layer` | ➕️ | LANDED (G2) |
| 314 | `remove-construction-layer` | ➖️ | LANDED (G2) |
| 315 | `reorder-construction-layers` | 🔀️ | LANDED (G2) |

### Internal gains — `400`–`499` (§4.5)

| # | kind | emoji | status |
|---|---|---|---|
| | **PeopleGain** | | |
| 400 | `create-people-gain` | 👤️ | LANDED (G2) |
| 401 | `delete-people-gain` | 🚷️ | LANDED (G2) |
| 402 | `change-people-gain-zone` | 🚶️ | LANDED (G2) |
| 403 | `change-people-gain-schedule` | ⏰️ | LANDED (G2) |
| 404 | `change-people-gain-activity-schedule` | 🏃️ | LANDED (G2) |
| 405 | `change-people-gain-people-per-area` | 👥️ | LANDED (G2) |
| 406 | `change-people-gain-sensible-fraction` | 🌞️ | LANDED (G2) |
| 407 | `change-people-gain-latent-fraction` | 💧️ | LANDED (G2) |
| 408 | `change-people-gain-radiant-fraction` | 📡️ | LANDED (G2) |
| | **LightingGain** | | |
| 409 | `create-lighting-gain` | 💡️ | LANDED (G2) |
| 410 | `delete-lighting-gain` | 🕯️ | LANDED (G2) |
| 411 | `change-lighting-gain-zone` | 🔦️ | LANDED (G2) |
| 412 | `change-lighting-gain-schedule` | ⏱️ | LANDED (G2) |
| 413 | `change-lighting-gain-watts-per-area` | 🔌️ | LANDED (G2) |
| 414 | `change-lighting-gain-radiant-fraction` | 🌟️ | LANDED (G2) |
| 415 | `change-lighting-gain-visible-fraction` | 🔅️ | LANDED (G2) |
| 416 | `change-lighting-gain-return-air-fraction` | 🎐️ | LANDED (G2) |
| | **EquipmentGain** | | |
| 417 | `create-equipment-gain` | 🖥️ | LANDED (G2) |
| 418 | `delete-equipment-gain` | 🧯️ | LANDED (G2) |
| 419 | `change-equipment-gain-zone` | 🖨️ | LANDED (G2) |
| 420 | `change-equipment-gain-schedule` | ⌛️ | LANDED (G2) |
| 421 | `change-equipment-gain-watts-per-area` | ⚡️ | LANDED (G2) |
| 422 | `change-equipment-gain-radiant-fraction` | 🌠️ | LANDED (G2) |
| 423 | `change-equipment-gain-latent-fraction` | 💦️ | LANDED (G2) |
| | **Infiltration** | | |
| 424 | `create-infiltration` | 💨️ | LANDED (G2) |
| 425 | `delete-infiltration` | 🧽️ | LANDED (G2) |
| 426 | `change-infiltration-zone` | 🌀️ | LANDED (G2) |
| 427 | `change-infiltration-schedule` | ⏳️ | LANDED (G2) |
| 428 | `change-infiltration-flow-per-exterior-area` | 🌫️ | LANDED (G2) |
| 429 | `change-infiltration-constant-term-coefficient` | 🅰️ | LANDED (G2) |
| 430 | `change-infiltration-temperature-term-coefficient` | 🅱️ | LANDED (G2) |
| 431 | `change-infiltration-velocity-term-coefficient` | 🆎️ | LANDED (G2) |
| 432 | `change-infiltration-velocity-squared-term-coefficient` | 🆑️ | LANDED (G2) |
| | **Infiltration — added by G2 after W-C's schema reshape** | | |
| 440 | `change-infiltration-method` | 🔬️ | LANDED (G2) |
| 441 | `change-infiltration-design-flow-ach` | 🔄️ | LANDED (G2) |
| 442 | `change-infiltration-effective-leakage-area` | 🕳️ | LANDED (G2) |
| 443 | `change-infiltration-discharge-coefficient` | 🚰️ | LANDED (G2) |
| 444 | `change-infiltration-stack-height` | 🏭️ | LANDED (G2) |
| | **MechanicalVentilation** | | |
| 433 | `create-mechanical-ventilation` | 🌪️ | LANDED (G2) |
| 434 | `delete-mechanical-ventilation` | 🚫️ | LANDED (G2) |
| 435 | `change-mechanical-ventilation-zone` | 🧭️ | LANDED (G2) |
| 436 | `change-mechanical-ventilation-schedule` | 📆️ | LANDED (G2) |
| 437 | `change-mechanical-ventilation-design-flow` | 🚿️ | LANDED (G2) |
| 438 | `change-mechanical-ventilation-fan-total-efficiency` | 💠️ | LANDED (G2) |
| 439 | `change-mechanical-ventilation-fan-delta-pressure` | 🎈️ | LANDED (G2) |

### HVAC zone-level — `500`–`599` (§4.6)

| # | kind | emoji | status |
|---|---|---|---|
| | **Thermostat** | | |
| 500 | `create-thermostat` | 🩺️ | LANDED (G3) |
| 501 | `delete-thermostat` | 🛑️ | LANDED (G3) |
| 502 | `change-thermostat-zone` | 🛖️ | LANDED (G3) |
| 503 | `change-thermostat-heating-setpoint-schedule` | 🥵️ | LANDED (G3) |
| 504 | `change-thermostat-cooling-setpoint-schedule` | 🐧️ | LANDED (G3) |
| 505 | `change-thermostat-heating-throttle-range` | 🎚️ | LANDED (G3) |
| 506 | `change-thermostat-cooling-throttle-range` | 🎛️ | LANDED (G3) |
| | **Humidistat** | | |
| 507 | `create-humidistat` | 🌂️ | LANDED (G3) |
| 508 | `delete-humidistat` | 🏜️ | LANDED (G3) |
| 509 | `change-humidistat-zone` | 🏙️ | LANDED (G3) |
| 510 | `change-humidistat-humidifying-setpoint-schedule` | ☔️ | LANDED (G3) |
| 511 | `change-humidistat-dehumidifying-setpoint-schedule` | 🏝️ | LANDED (G3) |
| 512 | `change-humidistat-humidifying-throttle-range` | 🌧️ | LANDED (G3) |
| 513 | `change-humidistat-dehumidifying-throttle-range` | 🧻️ | LANDED (G3) |
| | **IdealLoadsSystem** | | |
| 514 | `create-ideal-loads-system` | 🫁️ | LANDED (G3) |
| 515 | `delete-ideal-loads-system` | 🫥️ | LANDED (G3) |
| 516 | `change-ideal-loads-system-zone` | 🏢️ | LANDED (G3) |
| 517 | `change-ideal-loads-system-max-heating-supply-air-temp` | 🔴️ | LANDED (G3) |
| 518 | `change-ideal-loads-system-min-cooling-supply-air-temp` | 🔵️ | LANDED (G3) |
| 519 | `change-ideal-loads-system-max-heating-capacity` | ⛽️ | LANDED (G3) |
| 520 | `change-ideal-loads-system-max-cooling-capacity` | 🟧️ | LANDED (G3) |
| 521 | `change-ideal-loads-system-outdoor-air-per-person` | 🧍️ | LANDED (G3) |
| 522 | `change-ideal-loads-system-outdoor-air-per-area` | 🔳️ | LANDED (G3) |
| | **ZoneEquipmentAssignment** | | |
| 523 | `create-zone-equipment` | 🛠️ | LANDED (G3) |
| 524 | `delete-zone-equipment` | 🗑️ | LANDED (G3) |
| 525 | `change-zone-equipment-zone` | 🏬️ | LANDED (G3) |
| 526 | `change-zone-equipment-type` | 🔧️ | LANDED (G3) |
| 527 | `change-zone-equipment-priority` | 🎗️ | LANDED (G3) |
| 528 | `change-zone-equipment-heating-capacity` | 🧇️ | LANDED (G3) |
| 529 | `change-zone-equipment-cooling-capacity` | 🍧️ | LANDED (G3) |
| | **DaylightZoneConfig** | | |
| 530 | `create-daylight-zone` | 🔭️ | LANDED (G3) |
| 531 | `delete-daylight-zone` | 🌗️ | LANDED (G3) |
| 532 | `change-daylight-zone-zone` | 🏫️ | LANDED (G3) |
| 533 | `change-daylight-zone-illuminance-target` | 🪔️ | LANDED (G3) |
| 534 | `change-daylight-zone-glare-limit` | 🕶️ | LANDED (G3) |
| 535 | `change-daylight-zone-window-transmittance` | 🥃️ | LANDED (G3) |
| | **SizingObject** | | |
| 536 | `create-sizing-object` | 📶️ | LANDED (G3) |
| 537 | `delete-sizing-object` | 🪒️ | LANDED (G3) |
| 538 | `change-sizing-object-zone` | 🏨️ | LANDED (G3) |
| 539 | `change-sizing-object-sizing-type` | 🧾️ | LANDED (G3) |
| 540 | `change-sizing-object-design-day-type` | 🌥️ | LANDED (G3) |
| | **RoomAirModelAssignment** | | |
| 541 | `create-room-air-model-assignment` | 🛏️ | LANDED (G3) |
| 542 | `delete-room-air-model-assignment` | 🧺️ | LANDED (G3) |
| 543 | `change-room-air-model` | 🪭️ | LANDED (G3) |

### HVAC loop-level & plant — `600`–`699` (§4.7)

| # | kind | emoji | status |
|---|---|---|---|
| | **SetpointManager** | | |
| 600 | `create-setpoint-manager` | 📌️ | LANDED (G3) |
| 601 | `delete-setpoint-manager` | 🍄️ | LANDED (G3) |
| 602 | `rename-setpoint-manager` | 🖇️ | LANDED (G3) |
| 603 | `replace-setpoint-manager-kind` | 🔃️ | LANDED (G3) |
| 604 | `change-setpoint-manager-schedule` | 🎼️ | LANDED (G3) |
| | **ModelAirLoop** | | |
| 605 | `create-air-loop` | 🛞️ | LANDED (G3) |
| 606 | `delete-air-loop` | 🥀️ | LANDED (G3) |
| 607 | `rename-air-loop` | 📇️ | LANDED (G3) |
| 608 | `change-air-loop-supply-node` | ↗️ | LANDED (G3) |
| 609 | `change-air-loop-return-node` | ↘️ | LANDED (G3) |
| 610 | `change-air-loop-design-supply-air-flow` | 🍥️ | LANDED (G3) |
| 611 | `add-air-loop-terminal-zone` | 🪺️ | LANDED (G3) |
| 612 | `remove-air-loop-terminal-zone` | 🪹️ | LANDED (G3) |
| | **PlantLoopConfig** | | |
| 613 | `create-plant-loop` | ⚗️ | LANDED (G3) |
| 614 | `delete-plant-loop` | 💣️ | LANDED (G3) |
| 615 | `rename-plant-loop` | 📛️ | LANDED (G3) |
| 616 | `change-plant-loop-type` | ♻️ | LANDED (G3) |
| 617 | `change-plant-loop-supply-temperature` | ☕️ | LANDED (G3) |
| 618 | `change-plant-loop-return-temperature` | 🧫️ | LANDED (G3) |
| 619 | `change-plant-loop-design-flow` | 🚤️ | LANDED (G3) |
| 620 | `add-plant-loop-equipment` | 🔩️ | LANDED (G3) |
| 621 | `remove-plant-loop-equipment` | ⚙️ | LANDED (G3) |
| | **OutdoorAirSystem** | | |
| 622 | `create-outdoor-air-system` | 🌲️ | LANDED (G3) |
| 623 | `delete-outdoor-air-system` | 🍂️ | LANDED (G3) |
| 624 | `change-outdoor-air-system-air-loop` | ⛓️ | LANDED (G3) |
| 625 | `change-outdoor-air-system-min-oa-flow` | 🦋️ | LANDED (G3) |
| 626 | `change-outdoor-air-system-economizer-enabled` | 💰️ | LANDED (G3) |

### Electrical, renewables & other systems — `700`–`799` (§4.8)

| # | kind | emoji | status |
|---|---|---|---|
| | **ElectricalLoadCenter** | | |
| 700 | `create-electrical-load-center` | 🏦️ | LANDED (G4) |
| 701 | `delete-electrical-load-center` | 🔻️ | LANDED (G4) |
| 702 | `rename-electrical-load-center` | 🖊️ | LANDED (G4) |
| 703 | — | — | NOT AUTHORED (see §6) |
| 704 | — | — | NOT AUTHORED (see §6) |
| 705 | `add-electrical-load-center-pv` | ☄️ | LANDED (G4) |
| 706 | `remove-electrical-load-center-pv` | 🌘️ | LANDED (G4) |
| 707 | `add-electrical-load-center-battery` | 🔋️ | LANDED (G4) |
| 708 | `remove-electrical-load-center-battery` | 🪝️ | LANDED (G4) |
| | **PvSystemAssignment** | | |
| 709 | `create-pv-system` | ✨️ | LANDED (G4) |
| 710 | `delete-pv-system` | 🌒️ | LANDED (G4) |
| 711 | `change-pv-system-dc-capacity` | ⚛️ | LANDED (G4) |
| 712 | `change-pv-system-area` | 🟨️ | LANDED (G4) |
| 713 | `change-pv-system-tilt` | 📈️ | LANDED (G4) |
| 714 | `change-pv-system-azimuth` | 🧿️ | LANDED (G4) |
| 715 | `change-pv-system-module-efficiency` | 🎖️ | LANDED (G4) |
| 716 | `change-pv-system-inverter-efficiency` | ♌️ | LANDED (G4) |
| | **BatteryAssignment** | | |
| 717 | `create-battery` | 🪙️ | LANDED (G4) |
| 718 | `delete-battery` | ♒️ | LANDED (G4) |
| 719 | `change-battery-capacity` | 🥫️ | LANDED (G4) |
| 720 | `change-battery-max-charge` | ⏫️ | LANDED (G4) |
| 721 | `change-battery-max-discharge` | ⏬️ | LANDED (G4) |
| 722 | `change-battery-round-trip-efficiency` | 🥉️ | LANDED (G4) |
| | **ShwSystemConfig** | | |
| 723 | `create-shw-system` | 🛀️ | LANDED (G4) |
| 724 | `delete-shw-system` | 🚱️ | LANDED (G4) |
| 725 | `change-shw-system-heater-capacity` | 🍵️ | LANDED (G4) |
| 726 | `change-shw-system-storage-volume` | 🛢️ | LANDED (G4) |
| 727 | `change-shw-system-setpoint` | 🏹️ | LANDED (G4) |
| 728 | `change-shw-system-schedule` | 🕐️ | LANDED (G4) |
| | **SolarThermalConfig** | | |
| 729 | `create-solar-thermal-system` | 🌄️ | LANDED (G4) |
| 730 | `delete-solar-thermal-system` | 🌆️ | LANDED (G4) |
| 731 | `change-solar-thermal-system-collector-area` | 🟩️ | LANDED (G4) |
| 732 | `change-solar-thermal-system-efficiency` | 🏅️ | LANDED (G4) |
| 733 | `change-solar-thermal-system-storage-volume` | 🧃️ | LANDED (G4) |
| 734 | `change-solar-thermal-system-tilt` | 🔼️ | LANDED (G4) |
| 735 | `change-solar-thermal-system-azimuth` | ⛵️ | LANDED (G4) |
| | **RefrigerationConfig** | | |
| 736 | `create-refrigeration-system` | ❄️ | LANDED (G4) |
| 737 | `delete-refrigeration-system` | 🫠️ | LANDED (G4) |
| 738 | `change-refrigeration-system-case-count` | 🗄️ | LANDED (G4) |
| 739 | `change-refrigeration-system-design-load` | 🏋️ | LANDED (G4) |
| 740 | `change-refrigeration-system-defrost-schedule` | 🕑️ | LANDED (G4) |
| | **WaterSystemConfig** | | |
| 741 | `create-water-system` | 🚽️ | LANDED (G4) |
| 742 | `delete-water-system` | 🧼️ | LANDED (G4) |
| 743 | `change-water-system-fixture-count` | 🪣️ | LANDED (G4) |
| 744 | `change-water-system-peak-flow` | 🚾️ | LANDED (G4) |
| 745 | `change-water-system-schedule` | 🕒️ | LANDED (G4) |
| | **FaultDefinition** | | |
| 746 | `create-fault` | ⚠️ | LANDED (G4) |
| 747 | `delete-fault` | 🩹️ | LANDED (G4) |
| 748 | `change-fault-target-equipment` | 🎣️ | LANDED (G4) |
| 749 | `change-fault-type` | 🐛️ | LANDED (G4) |
| 750 | `change-fault-severity` | 🌶️ | LANDED (G4) |
| 751 | `change-fault-start-schedule` | 🕓️ | LANDED (G4) |

### Grouping collections — `800`–`899` (§4.9)

| # | kind | emoji | status |
|---|---|---|---|
| | **SpaceList** | | |
| 800 | `create-space-list` | 📋️ | LANDED (G4) |
| 801 | `delete-space-list` | 🗒️ | LANDED (G4) |
| 802 | `rename-space-list` | 🪶️ | LANDED (G4) |
| 803 | `add-space-list-member` | ➡️ | LANDED (G4) |
| 804 | `remove-space-list-member` | 🪤️ | LANDED (G4) |
| | **ThermalEnclosure** | | |
| 805 | `create-thermal-enclosure` | 🏟️ | LANDED (G4) |
| 806 | `delete-thermal-enclosure` | 🏯️ | LANDED (G4) |
| 807 | `rename-thermal-enclosure` | 🖋️ | LANDED (G4) |
| 808 | `add-thermal-enclosure-zone` | 🔒️ | LANDED (G4) |
| 809 | `remove-thermal-enclosure-zone` | 🔓️ | LANDED (G4) |

### Schedules — `900`–`999` (§5.1 — schedules are persisted `Model` data as of this ticket)

| # | kind | emoji | status |
|---|---|---|---|
| | **ConstantSchedule** | | |
| 900 | `create-constant-schedule` | 🕜️ | LANDED (G4) |
| 901 | `delete-constant-schedule` | 📍️ | LANDED (G4) |
| 902 | `change-constant-schedule-value` | 🕝️ | LANDED (G4) |
| | **DailySchedule** | | |
| 903 | `create-daily-schedule` | 🕞️ | LANDED (G4) |
| 904 | `delete-daily-schedule` | 🌓️ | LANDED (G4) |
| 905 | `replace-daily-schedule-hourly-values` | 🕔️ | LANDED (G4) |
| 906 | `change-daily-schedule-interpolation` | 🕕️ | LANDED (G4) |
| 907 | `change-daily-schedule-limits` | 🕟️ | LANDED (G4) |
| | **WeeklySchedule** | | |
| 908 | `create-weekly-schedule` | 🗓️ | LANDED (G4) |
| 909 | `delete-weekly-schedule` | 🕖️ | LANDED (G4) |
| 910 | `change-weekly-schedule-day` | 🕗️ | LANDED (G4) |
| | **AnnualSchedule** | | |
| 911 | `create-annual-schedule` | 📚️ | LANDED (G4) |
| 912 | `delete-annual-schedule` | 📕️ | LANDED (G4) |
| 913 | `insert-annual-schedule-rule` | 📗️ | LANDED (G4) |
| 914 | `remove-annual-schedule-rule` | 📙️ | LANDED (G4) |
| 915 | `reorder-annual-schedule-rules` | 🗂️ | LANDED (G4) |
| 916 | `change-annual-schedule-default-daily-schedule` | 🎌️ | LANDED (G4) |
| 917 | `change-annual-schedule-holiday-daily-schedule` | 🎄️ | LANDED (G4) |
| 918 | `add-annual-schedule-holiday` | 🎉️ | LANDED (G4) |
| 919 | `remove-annual-schedule-holiday` | 🎊️ | LANDED (G4) |
| | **TimeSeriesSchedule** | | |
| 920 | `create-time-series-schedule` | 🪗️ | LANDED (G4) |
| 921 | `delete-time-series-schedule` | 🎞️ | LANDED (G4) |
| 922 | `replace-time-series-schedule-values` | 🕘️ | LANDED (G4) |
| 923 | `change-time-series-schedule-timestep` | 🕙️ | LANDED (G4) |

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
- **G1 landed 228–235 beyond the reserved rows.** `Fenestration` gained `glazing_construction_id:
  Option<EntityId>` (the layered-glazing escape from the U/SHGC/VLT simplification the ticket's oracle
  comparison measured at +5.7…+8.1 % annual cooling), and W-C's ANSI/ASHRAE 140 work had already added
  `height_m`/`sill_height_m` and the four overhang/fin fields — so the geometry range carries a
  `bind`/`clear` pair for the glazing slot and six `change` kinds for the window geometry §4.3 never
  listed. `assign` is NOT in `protocol::APPROVED_VERBS`, so the glazing pair uses `bind`/`clear`.
- **`delete-zone` (`105`) must REFUSE while any space or surface still references the zone**
  (`mutation.target-in-use`), rather than cascading — the cascade would have to delete surfaces,
  which cascade to fenestrations and adjacency pairs. `delete-surface` (`201`) DOES cascade, stripping
  the fenestrations and adjacency pairs that address it and emitting `mutation.cascade` at info level
  (`📸️remodel`'s `🪓delete-stream` is the worked reference).

## 7. G4 amendments (electrical 700–799, grouping 800–899, schedules 900–999)

- **`703`/`704` `add-/remove-electrical-load-center-generator` are NOT AUTHORED, and the numbers stay
  reserved.** `ElectricalLoadCenter::generator_ids` references a collection `Model` does not have
  (vocabulary §5.4); `⚡️electrical`'s `Generator` type exists in the engine but is not reachable from
  the document. A kind that can only ever refuse cannot carry the `✅️` happy vector the generator's
  own `audit()` now requires, and adding `generators: Vec<…>` to `Model` mid-ticket would invalidate
  every committed `🔋️model.json` and every oracle-emitted document. `generator_ids` IS carried
  verbatim by `create-electrical-load-center` (unchecked, documented in the leaf), so `delete`'s
  inverse restores a load centre exactly. Author 703/704 the moment a backing collection lands.
- **`913` is `insert-annual-schedule-rule`, not `add-annual-schedule-rule`.** `📓️taxonomy.md`
  reserves `add` for set-like membership and `insert` for an ordered, index-addressed list, and
  `AnnualSchedule::rules` is measurably ordered: `ScheduleSet::annual_value`
  (`⚙️engine/🗓️schedule/🦀️.rs`) returns the FIRST rule whose date range contains the day, which is
  also why `915 reorder-annual-schedule-rules` exists at all. `914 remove-annual-schedule-rule` keeps
  its name — `remove` is `insert`'s partner in the same table.
- **`918`/`919` stay `add`/`remove`.** `AnnualSchedule::holiday_dates` is consumed by a `contains`
  test, so it is a set; the FINAL-state `index` on `add` exists only because a JSON array compares
  positionally, and it is what lets `remove`'s inverse put the date back where it was.
- **`748 change-fault-target-equipment` now HAS a real referential check.** The ledger's earlier note
  said it could not, for want of a discriminator. The engine supplies one: the kernel's
  `SystemSubstepStage::Fault` matches `fault.target_equipment_id == ideal.id` and
  `🧠️precompute`'s `PrecomputeStage::Faults` keys the severity map by the same id, so `ideal_loads`
  is the collection it actually addresses, and both `746` and `748` check it there.
- **Schedule ids are one namespace across all five groups.** `ScheduleSet::lookup` searches
  constants → annual → weekly → daily → time-series in turn, so every `create-*-schedule` refuses an
  id ANY group already holds, and every `delete-*-schedule` refuses while any of the fourteen
  `ScheduleId`-typed slots in the document — schedule-to-schedule references included — still
  resolves it.
