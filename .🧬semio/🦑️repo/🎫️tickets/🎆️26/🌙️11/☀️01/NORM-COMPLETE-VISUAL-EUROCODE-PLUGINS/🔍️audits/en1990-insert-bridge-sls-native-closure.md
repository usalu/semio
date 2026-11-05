# EN1990 InsertBridgeSls Native Type Closure

Read-only source audit; no derive edits, compilation or native pass claimed.

| Actual Type | Definition | Fields / Existing Capability | Native Work Needed |
| --- | --- | --- | --- |
| InsertBridgeSls | EN1990 subset 🧬️schema/🧬️mutations/🌉️insert-bridge-sls/🦀️.rs:5-14 | index:Option<usize>, item:BridgeSls. Derives MutationLeaf, ToValue, FromValue, RetireOwned, CanonicalJsonTree, RetainedClone; value camelCase. | Real NativeReceive derive/implementation at this leaf owner. |
| BridgeSls | EN1990 subset 🧬️schema/📸️snapshot/🦀️.rs:415 | id:String, member_id:String and14 plain f64 quantities. Same FromValue/RetireOwned/RetainedClone plus DslRecord; value camelCase. | Real NativeReceive derive/implementation at model owner. |
| String, f64, usize, Option<usize> | Framework 🌱️value/🛬️decode/🧳️owned/🌳️value/🦀️.rs | Existing scalar/string/Option native receivers. | Reuse existing receiver lifecycle; no domain wrapper implementation needed. |

Full EN1990 subset prefix: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/`.

BridgeSls numeric fields are deck_acceleration, deck_acceleration_limit, deck_twist, deck_twist_limit, bridge_deflection, bridge_deflection_limit, track_gauge, line_speed, ballast_depth, deck_mass, natural_frequency, end_rotation, basic_wind_speed, damping_ratio. These physical quantities are plain framework-language f64 fields, not external unit-library wrappers. Neither InsertBridgeSls nor BridgeSls contains any enum, map, vector, flattened record, custom codec or generic type parameter. No snapshot type is needed to CONSTRUCT this payload; actual domain semantic admission later still needs the artifact snapshot.

Nearby common model enum definitions in the SAME snapshot source include StructureKind:24, SupervisionLevel:37, InspectionLevel:49, PermanentKind:233, LoadGroup:282, ImportanceClass:325, DistributionKind:466 and TestDistribution:500. They use FromValue/RetireOwned and value camelCase, but they are not in InsertBridgeSls construction closure. Requiring all those enums for this first genuine leaf constructor would expand its actual type closure unnecessarily. En1990Mutation aggregate wrapping is a separate dispatch layer; a leaf receiver plus exact funded map/wrap can avoid requiring every other aggregate variant receiver initially.

## NativeReceive Attribute Support

Implementation: `🧰️framework/🔨️modules/🌱️value/✨️derive/🛬️receive/🦀️.rs`.

- Lines7-9 reject generic declarations and unions require explicit supported owners. Concrete named records and tuple records are generated; tuple flatten is rejected.
- Field names use parsed value attributes and field_wire_name with container rename_all (line23); the actual InsertBridgeSls/BridgeSls camelCase contract fits this port.
- Unit enum variants use parsed variant rename/rename_all; a declared #[default] unit variant can generate NativeDefault. Tagged/data enums dispatch to the sibling enum expansion at line16.
- Missing Option fields, bare field defaults and container default use phased NativeDefault slots (lines23-27), not eager default construction. A custom default path is treated as required in the native required-field calculation; arbitrary default callbacks must not be assumed supported.
- Named flatten is supported using explicit NativeRecordFields/schema membership and flattened child receiving. Skip, deserialize_with and tuple flatten explicitly refuse at line23; these require a genuine explicit receiving port.
- The native struct expansion has no attrs.transparent branch. Although a tuple/newtype structure can be generated, this does not prove serde/value transparent wire semantics are honored. A transparent scalar wrapper needs explicit native support/receiver or proof from an existing wrapper implementation; do not equate one tuple field with transparent input automatically.
- Serde attrs are conditional test support on these real domain types; their production wire naming is explicitly declared by value(rename_all=camelCase). Native receiving parses the value contract, not a promise of every serde feature.

For the first leaf, the shortest true constructor closure is two concrete NativeReceive owners (BridgeSls and InsertBridgeSls) plus already supported primitives/options. Feed their receiver genuine retained original DslValue/NativeValueSource under actual NativeDecodeControl/grants, then transfer the typed leaf/wrapped aggregate under measured lifecycle. Do not substitute synchronous FromValue or from_payload_value behind a fixed guessed quote.
