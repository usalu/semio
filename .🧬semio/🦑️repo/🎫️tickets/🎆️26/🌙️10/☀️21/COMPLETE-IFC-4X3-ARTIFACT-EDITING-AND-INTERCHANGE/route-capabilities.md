# General Composition Route Capabilities

The native IFC4x3 source task exposed a general registration defect: the declaration builder accepted foreign-write composers, but capability requirements claimed the target's whole dialect. An IFC exporter targeting text would collide with the text artifact's legitimate native composer authority in the global ArtifactDefinitionRegistry. Parent explicitly authorized the smallest general framework correction required for the real export route.

## Contract

`composer_identity_claims(owner, writes, reads)` is the single typed authority derivation API used by both declaration-builder `.composers()` admission and final runtime capability requirement collection. Same-owner writes preserve the existing `dialect` claim exactly. Foreign writes require source dialects owned by the declaring artifact and claim their directed routes in the new `composition-route` namespace. A foreign exporter cannot silently register unrelated source routes.

The serialized claim uses unambiguous UTF-8 byte length framing: `<source bytes>:<source coordinate><target bytes>:<target coordinate>`. The IFC4x3 example is `17:s.stdio.ifc@4x3/*19:s.stdio.txt@utf-8/*`. `ArtifactIdentityClaim::composition_route` constructs it and the ordinary central `new` constructor validates it. Validation requires two exact canonical dialect coordinates, nonzero canonical decimal lengths, correct UTF-8 boundaries and no trailing content. ArtifactDefinition validation additionally requires the source artifact to equal the declaring artifact identity. Existing global conflict rejection/idempotence remains unchanged.

No IFC-specific exception, renamed target dialect, hidden export registration or alternate capability registry was introduced. The IFC artifact definition binds the real foreign exporter route normally, alongside native IFC4x3 write authority and its separate host document codec capability.

The global composer registry and cross-plugin IoRouter were inspected: they already key each route by complete source/target dialect and direction. Discovery's `list_composer_entries` previously retained only one arbitrary entry per written dialect, dropping other foreign readers. Its pure aggregation now unions all reader dialects for each target in deterministic order. The IFC convenience dispatcher also filters both target and supplied source dialects.

## Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪪️identity/🦀️.rs`: namespace, central framed claim construction/validation, owner enforcement and typed composer claim derivation.
- Identity `🧪️tests/composition-route.{feature,json,schema.json,rs}`: language-neutral cases and focused Rust tests. Identity Cargo manifest adds only the already-used serde_json test dependency.
- Plugin declaration `🧮️codec-spec-and-capability-sources/🦀️.rs` and `🏗️ready-declaration-builder/🦀️.rs`: same central claim derivation at both admission boundaries.
- Framework IO `📇️composer-entries/🦀️.rs` and `🧪️tests/route-discovery.rs`: complete source discovery and exact-key tests.
- Stdio contract assembly validator: recognizes the new first-party namespace; actual string validation still runs through the central typed claim constructor.
- IFC root declaration/dispatcher and artifact definition: normal exporter contribution, exact framed route metadata and distinct host codec identity capability.
- Ticket input `route-fixtures.ts`: existing Ajv independent JSON Schema fixture oracle, plus UTF-8 TextEncoder framing checks.

## Verification

The feature, JSON vectors, fixture schema and Rust tests were added before implementation. Two identity tests cover unchanged native claims, independent alpha/beta exporters coexisting with a native text artifact, duplicate/unowned rejection without registry mutation, exact canonical framing and malformed vectors. The discovery test covers native text plus two independent foreign writers, complete discovery union, duplicate listing elimination and exact source/target route key distinction. Five native IFC tests additionally invoke actual erased import/export entry functions on packed native artifacts, check FILE_SCHEMA retention and refuse foreign IFC4 graph packs.

**Passed:** existing Ajv validates the language-neutral fixture against its JSON Schema, with three valid and six malformed vectors; independent TextEncoder checks all valid framing byte counts. This is fixture/schema evidence and does not substitute for executing the Rust parser. **Passed:** rustfmt parsing/format checks for all new/modified general Rust modules and tests; IFC Cargo metadata resolves the package/dependencies. No full Rust type-check or unit-test execution is claimed: authentic compiler ingress remains unavailable, as recorded in `native-io-implementation.md`. No compiler enforcement bypass or build-system change was made.

The change is intentionally limited to correct route ownership and discovery. It does not redesign all existing conversion contributions, migrate legacy registrations, alter generic IO scheduling or assert that current synchronous composition supports retained cancellation. These latter limits remain explicit in the native IO report.
