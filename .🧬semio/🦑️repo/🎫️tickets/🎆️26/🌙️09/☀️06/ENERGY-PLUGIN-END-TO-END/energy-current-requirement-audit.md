# Current Energy Requirement Audit — 2026-10-21

Read-only product audit; no product changes or runtime tests performed. Direct tools follow the user-authorized Repo/MCP fallback. Paths below use `A = ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any` and `E = ✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine`. Line references describe current source, not successful runtime behavior.

## Confirmed Gaps

| Priority | Evidence | Next Implementable Action |
| --- | --- | --- |
| Critical | `A/✏️editor/🖥️editor-impl/🦀️.rs:127` constructs the run from snapshot and config template only. `A/✏️editor/🎚️config/🦀️.rs:65` supplies timesteps/warmup and defaults; `A/🧵️simulation-session/🦀️.rs:306` folds in run period/schedules only. `E/🧪️sim/📮️packets/🦀️.rs:144` uses synthetic weather whenever config.weather is absent. The persisted weather link mutation exists, but the editor run chain never resolves it. | Add retained, cancellable EPW artifact resolution pinned to the captured link head, populate actual WeatherData before admission, expose missing/refused/unavailable dependency errors and weather provenance. Test two distinguishable weather files through actual host dispatch and assert output differs. |
| High | `A/✏️editor/🎭️modes/✏️edit/🪟️windows/⚡️simulation/🦀️.rs:144` displays only generic weather-link text, with no selected weather identity or picker. | Provide import/select/clear weather actions, current file/head/location/date coverage, and localized diagnostics; drive existing bind/unbind native mutations. |
| High | Simulation window render at `…/⚡️simulation/🦀️.rs:210` builds run/settings/keyboard tree only. `A/✏️editor/📊️results/🔎️indexed/🦀️.rs:19` reads ESF2 surface partitions and four aggregate energy fields. No hourly zone/load charts, numerical result table, annual summary or results download is rendered by these paths. | Add bounded paged result source/read interface and publish retained hourly series/summary outputs; render zone/time/quantity selection, units, charts/table and export. Engine series alone does not complete this UI requirement. |
| Medium | `A/✏️editor/📌️panels/🔍️inspection/🦀️.rs:847` selection match covers a subset; line891 falls back to model summary. Mechanical/plant/electrical/water/schedule and other persisted collections have no focused inspector. | Add selection-aware schema-backed inspectors for every represented collection using existing native mutation forms and typed references; ensure tree selection reaches each one. |
| Medium | Same inspection file line649 explicitly renders thermostat zone read-only. | Bind live zone select to native thermostat zone reassignment. |
| Medium | `A/✏️editor/🧮️reduce-and-helpers/🦀️.rs:158,191` allocate create-zone/create-surface IDs from their respective collections, while editor entity selection identity is shared. | Define canonical cross-collection identity/allocation policy and test create/select/edit where another collection already holds the next ID. |
| Medium | `A/✏️editor/🎛️retained-commands/🦀️.rs:26` admission charges whole canonical model/payload against a fixed extent rather than resumable preparation. | Preserve explicit refusal if this is an intentional supported limit; otherwise implement retained incremental preparation for large models and test progress/cancellation. |
| Medium | Simulation window `…/⚡️simulation/🦀️.rs:173` puts `aria-live=polite · role=status · busy=…` into a plain tree-node label. This does not itself assign DOM accessibility semantics. | Verify framework rendering, then express live-region/busy semantics through actual accessible node attributes; browser-check status announcements and keyboard run controls. |

## Corrected Earlier Concerns

- IDF declaration capability is present in root model definition. Current IO registry `A/🚪️io/🦀️.rs:174` advertises canonical JSON input specifically for IDF export; ZIP and JSON entries remain model-only, avoiding the previously identified global JSON self-composition collision. Actual host IDF dispatch still requires proof.
- Native action German labels now have authored verb/entity/field localization (`A/✏️editor/🎮️commands/🦀️.rs:652` onward); the previous claim that all labels are identical is stale.
- Native action creation still zips descriptor/schema rosters, but a catalog regression exists. Source shows actual payload decoding through aggregate mutation `from_payload_value` (`A/✏️editor/🧮️reduce-and-helpers/🦀️.rs:221`). This is stronger than a raw snapshot replacement route.
- Retained command admission now traverses nested canonical model values and payload bytes. The earlier six-collection-only census concern is stale.

## Unverified Completion Gates

1. Full editor/commands/panels artifact compilation and execution after shared framework repairs; authored tests are not assumed passing.
2. Global plugin declaration/registry preflight; real import IDF → edit native → export IDF via host selectors, correct .idf filename, official converter success.
3. All306 actions through real manifest registration, form submission, retained factory, central application and undo/redo; verify numeric/reference/null/array forms rather than only schema projection.
4. Browser run start/pause/step/cancel/finalize, error presentation and stale source/config invalidation, including weather dependency changes and reload.
5. Shared/local persistence scopes, short-disconnection behavior and multi-user conflicting edits. Source mechanism existence is not end-to-end behavioral proof.
6. Annual numerical comparison and hourly physics traces remain owned by the results lane. No claim of EnergyPlus-class equivalence is made by this audit.

This report intentionally does not infer missing engine physics from missing inspector branches. Native action palette support is substantial but is not equivalent to a complete selection-aware simulation experience.
