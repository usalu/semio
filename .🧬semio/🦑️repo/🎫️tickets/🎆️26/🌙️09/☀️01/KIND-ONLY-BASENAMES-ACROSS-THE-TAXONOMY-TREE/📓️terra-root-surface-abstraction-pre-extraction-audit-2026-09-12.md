# Root Surface and Abstraction Law — Pre-Extraction Audit

**Scope:** read-only preparation for the surface-schema and abstraction-ownership slice. It covers surface config/presence discovery, six app-schema laws, the abstraction schema/evaluator/proof, and their current command and source-data consumers. Artifact schema discovery remains a lower-level direct dependency owned by the artifact slice; it is not copied into a surface module. No broad Nx or artifact route was run.

## Semantic Ownership Boundary

The current root functions separate into three related owners:

| Concern | Current root entry points | Required extracted role |
| --- | --- | --- |
| Surface owner discovery and facet loading | `policyDiscoverAppSchemaOwners` (21768), `policyLoadAppSchemaFacetLeaves` (21820) | Discover config/presence owners from real surface `type Config` bindings; project configured schema formats through the accepted field-discovery owners |
| Surface schema laws | facet completeness (21874), field parity (21936), config fidelity (22007), state purity (22063), type-name parity (22105), config relocation (22149) | Surface configuration and presence semantics |
| Abstraction ownership | schema/evaluator/source extractors (22185–22227), `policyAbstractionOwnershipBreaches` (22301) | OS, surface, and artifact ownership contract plus direct source evaluation |

[`policyAppSchemaBreaches`](../../../../../../../📜️script.ts:22359) composes the abstraction evaluator with the six surface laws. Its final owner should be a thin composition owner; neither the root nor an artifact compatibility façade should remain.

Surface discovery derives the config type from an authored surface Rust `type Config = XConfig;` binding. It discovers the canonical `🎚️config` owner and its sibling `👥️presence` owner, deriving `XPresence` from `XConfig`. The five configured schema formats are read with the accepted Rust, TypeScript module-resolution, GraphQL, JSON Schema, and Protobuf owners. A relocation finding for `🧮️config` or `🕸️wasm` is diagnostic evidence for forbidden source; it must not become a compatibility fallback that allows the forbidden layout to pass.

The abstraction evaluator owns a different law: it rejects OS fields/commands in a surface, and it rejects UI/host fields from an artifact. Its artifact portion must directly consume the shared artifact-root inventory provided by the artifact slice, then inspect only document and diff normative JSON leaves. That shared inventory is a dependency, not a reason to move artifact completeness, parity, or other artifact laws into this slice.

## Direct and Source-Data Consumers

1. The normative abstraction schema is [`library/📏️ownership/🧬️schema/🔣️.json`](../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📏️ownership/🧬️schema/🔣️.json). The evaluator must read it as source data.
2. The portable fixture [`abstraction-ownership/🔣️.json`](../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📏️ownership/🧫️fixtures/🧪️abstraction-ownership/🔣️.json) supplies ownership vectors, nested schema vectors, command-source vectors, and the artifact-schema census. Keep it a fixture rather than moving it into surface production code.
3. Root `verify abstraction-ownership` test/report/enforce dispatch at [`📜️script.ts:7686`](../../../../../../../📜️script.ts:7686) must directly call the extracted proof/evaluator. The separate `artifact-contract-ownership` route at [`📜️script.ts:7137`](../../../../../../../📜️script.ts:7137) also calls the proof and consumes the fixture’s artifact paths.
4. Full verification calls the app aggregate at [`📜️script.ts:8242`](../../../../../../../📜️script.ts:8242), and the lint aggregate calls it at [`📜️script.ts:26566`](../../../../../../../📜️script.ts:26566).
5. Registered workspace targets remain [`📋️project.json:1291`](../../../../../../../📋️project.json:1291) and [`📋️project.json:1428`](../../../../../../../📋️project.json:1428), with corresponding seed and generated launch entries. A direct function test alone cannot replace those routes.
6. The artifact field-parity owner independently consumes the same fixture for representation parity. Retain that explicit source-data edge, but do not fold its field law into the abstraction evaluator.

Each newly extracted owner must import shared field discovery, surface-root discovery, and artifact-root discovery directly from their semantic owners. No extracted owner may import root `📜️script.ts`.

## Existing Independent Acceptance Baseline

The packet records a prior direct `verify abstraction-ownership test` result: **11 ownership vectors, four nested-schema vectors, and three Rust command-source vectors** agreed with Ajv in 2.967 seconds. It also reported **104 artifact contracts across five schema formats**. Those are current prerequisite evidence, not tests run by this audit.

The retained portable proof has useful independent layers:

- Ajv validates the ownership document while the owned evaluator returns the same violation disposition.
- Nested JSON Schema traversal covers `oneOf`, `$defs`, artifact-state exclusion, and an allowed domain option.
- Rust command extraction uses the first-party `inspectRustStructure` implementation (built from `rustTokens` and `RustStructureParser`) and distinguishes an authored enum declaration from a comment or host reference. Ajv validates only the projected ownership disposition; it does not independently parse the Rust source.
- Artifact representations use the accepted language field extractors; TypeScript sources are scanned by Bun’s TypeScript parser.

Keep this exact baseline after rebinding. The existing native [`rust-physical-reference-context` oracle](../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔮️oracles/🧲️rust-physical-reference-context/🦀️.rs:581) uses `syn::parse_file` and is registered as `@semio-tech/repo-lib:test-rust-physical-reference-context`, but it checks physical-reference contexts rather than command enum ownership. The abstraction route does not execute it. A future native continuation needs a scoped `syn` command-enum assertion before it can independently validate the first-party extraction; do not present the current portable route as having done so.

## Focused Controls Needed for the Split

Add or preserve language-independent controls for:

- canonical config and presence discovery from a real `type Config` binding; a missing config owner; a missing presence sibling; and a legacy `🧮️config`/ `🕸️wasm` directory that is reported rather than accepted;
- missing configured facet leaves, missing normative leaf, malformed normative JSON, wrong type name, and actual TypeScript alias/import resolution from the source directory;
- field missing/extra and optionality/cardinality negatives, retaining only the admitted Protobuf-map optionality and fixed-list exceptions;
- config fidelity against the real Rust config struct and state-purity reversal between config and presence;
- nested schema cases through `allOf`, `anyOf`, `additionalProperties`, false properties, `dependentSchemas`, and artifact-state annotations, so an extraction cannot quietly narrow the existing recursive evaluator;
- Rust source command negatives for an enum ending in neither `Command` nor `Mutation`, a comment/string that resembles an enum, a forbidden OS command in an authored enum, and an allowed domain command;
- source admission failures: unreadable surface/config/presence paths and symlinks must report a controlled result. Current directory readers and the relocation walker suppress errors, so an unreadable owner must never become an absent, clean owner.

Use structured fixture/schema values and parser/Ajv outcomes. Do not create source-body hash snapshots or broaden the OS-field vocabulary to hide current violations.

## Limits and Status

The packet’s current app aggregate observation—278 discovered owners and 601 findings—is diagnostic debt, not an acceptance count. The surface extraction must preserve its findings and severity rather than make them disappear through taxonomy movement.

**Status:** pre-extraction audit complete. The extraction needs a direct shared artifact-inventory import, complete fixture/source-data registration, and the focused hostile controls above before acceptance. No product file changed.
