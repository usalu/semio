# Energy Editor Completion Work — 2026-10-21

## Implemented

- Surface and window polygon editing through their existing property commands, emitting only native `replace-surface-vertices` / `replace-fenestration-vertices` intents. Full polygon input uses SI triples (`x,y,z; x,y,z; …`), supports initializing an aperture without explicit vertices, and clears aperture geometry back to its native area/height/sill representation when emptied.
- Paged x/y/z number controls for every explicit surface, window and shading vertex. Number controls use framework Change/scrub bindings and full polygon input uses Commit.
- Surface thermal-zone and window host-surface controls, with live reference admission and native granular reassignment mutations.
- Shading inspector now edits name, optional transmittance schedule, full polygon and coordinates. New `set-shading-property` command is registered across typed command schema, action bridge, action manifest, retained roster, publication lanes, proof roster and TypeScript declarations.
- People, lighting, equipment and infiltration inspectors now expose every native scalar plus zone/schedule references, people activity schedule and infiltration method. Ideal-loads inspector exposes zone, temperatures, outdoor-air parameters and optional capacities.
- Space inspector now edits name, thermal zone and floor area. Space selections previously had no render branch and fell back to summary. New `set-space-property` follows the same retained/native mutation infrastructure.
- No new runtime libraries. Existing sparse diff construction, central applier and native inverse implementations remain authoritative.
- JSON language-neutral geometry/space/shading law fixture and Rust reducer/UI projection regression tests. UI projections and law fixture are parsed through independent serde_json in tests.

- The complete native mutation catalog now contributes 306 individual palette actions with precise leaf-schema forms (776 typed inputs), using original aggregate variant IDs as collision-free retained tool identities. The bridge creates the native aggregate via `Mutation::from_payload_value` and publishes it to the central Store. Constant tool/publication rosters and owner/factory proofs derive from the same descriptor roster. No giant conditional payload form or whole-model replacement is used.
- Reference catalogues larger than the fixed select capacity now offer editable numeric IDs (nullable references use text to preserve clearing), with existing live target admission.
- Retained admission traverses the entire borrowed canonical model tree, charging every nested node and UTF8/key byte plus native payload bytes with depth64 and65,536 total work units. Nested schedules/large strings no longer evade the declared first-step extent.
- Four PV payload schemas incorrectly named an undeclared `number` widget; these now use canonical `stepper`, allowing the actual manifest form projection to succeed.

## Validation

Focused command (ticket-owned toolchain wrapper):

```sh
python .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/ENERGY-PLUGIN-END-TO-END/📜️energy-cargo.py test --manifest-path ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/Cargo.toml -p semio-s-artifact-energy-model-editor-commands -p semio-s-artifact-energy-model-editor-panels -p semio-s-artifact-energy-model-editor --lib
```

Standalone audit invokes the real manifest-mutation-inputs owner against all306 current production leaf schemas, resolves their reference document and independently checks required/optional properties using serde_json. It passes with776 typed fields after fixing the four PV schema defects. Source input harness is retained at `native-schema-audit/` and compiler outputs stay under the generated folder.

Build reached external framework crates but failed before energy editor compilation: `semio-framework-os-store-replay` has23 current upstream API drift errors. Earlier protocol-law `ColdMutationDiff` import and actor stale method calls were repaired by the coordinating agent. Earlier missing generated locale axes/styling token outputs were restored by the coordinating agent through owner generators. No passing full editor integration result is claimed. Rustfmt parses all authored Rust logic/tests without syntax errors.

## Remaining Scope From Current Sources

The full objective is not complete. `Model` has numerous persisted collections with no matching editable inspector branch. Existing renderer covers zones, spaces, surfaces, fenestrations, opaque/glazing/gas materials, constructions, thermostats, shading, internal gains, infiltration and ideal loads. Site fields are editable in summary; model name/version are editable through the structure window. The306 native palette forms provide typed authoring for all native mutation kinds, but focused selection-aware inspectors remain incomplete for mechanical ventilation, humidistats, setpoint managers, zone equipment, air/plant loops, outdoor air systems, space lists, thermal enclosures, adjacency pairs, airflow network, electrical/PV/battery, hot water/solar thermal/refrigeration/water systems, faults, output variables, sizing objects, daylight zones, room air models, ground configuration and schedule families. Most are already represented by native granular mutation groups, so their UI must route to those rather than replacing the model.

Additional known holes: creation/removal controls are limited to existing zone/surface actions; thermostat zone assignment remains read-only; interzone boundary display does not offer its current kind among ordinary boundary options; entity-id allocation for zone/surface creation is collection-local despite shared entity selection identity. These require further implementation/audit. The entire306-action registered runtime dispatch and rendered action forms still need live validation once unrelated Store replay compiles; schema projection success alone is not proof of end-to-end product completion. Native first-step admission still intentionally rejects models above its declared work extent; large-model resumable authoring remains a requirement to assess.

## Owned Files

All authored product changes are inside the energy model artifact editor: command declarations, retained publication, editor proof roster, manifest descriptions, reducer/property handlers, inspector panel, editor Cargo dependency list, TypeScript action declarations and reducer/panel tests plus JSON law fixture. This report is retained in the existing energy end-to-end ticket.
