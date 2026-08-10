# A7 — Full Gate Report

## Headline

App schema facets are complete for all **39 owners / 53 apps**: config + presence × five formats, kernel `DocumentApp::Presence` + `presence_pack`, framework `AppSchemaDescriptor` catalog with all 39 registered, taxonomy/discovery/registry twins, and root `policyAppSchemaBreaches` at **0**.

## Gates

| Gate | Result |
| --- | --- |
| `policyAppSchemaBreaches` | **0** |
| `policyArtifactSchemaBreaches` | **0** (unchanged) |
| `validateTaxonomy` (discovery) | **0** problems |
| `cargo test -p semio-framework-schema --lib` | **5 passed** (incl. 39-owner catalog) |
| `cargo check -p semio-framework-plugin` | **pass** |
| `cargo test -p semio-framework-os-kernel --lib presence_peer_binary` | **2 passed** |
| `bun nx run @semio-tech/plugin-registry:generate` | **pass** (58 plugins) after fixing duplicate `appsDir` in registry `AppFacetWalk` |
| Owner leaf completeness | **39/39** owners, **0** missing facet leaves |
| `DocumentApp` Presence bindings | **0** apps missing `type Presence` |

## A7 fix during gate

Registry generate failed on duplicate `const appsDir` in `📔️registry/📜️script.ts` (examples walk + AppFacetWalk). Removed the second declaration; reuse the existing binding in the same validator function.

## Out of scope / parked

- Artifact `catalog-integration` feature still empty (plugin deps cycle with `semio-framework-schema`); app catalog uses cycle-free central `include_str!` registration instead.
- Live hub `presence_pack` UI sync per plugin was not end-to-end exercised (types + wire + facets only).
- Unrelated workspace policy families beyond `app-schema/*` / `artifact-schema/*` were not part of this ticket's gate.
