# EN1990 Insertion Native Expansion Inventory

Read-only followup while exact first-bridge red50937 compiles. All14 actual EDIT_RULES insert_item rows retained. No new gate or source edits for this inventory.

| Actual Kind | Leaf Definition Relative to mutations | Immediate Model Type | Model Definition | Additional Child Receiver Closure |
| --- | --- | --- | --- | --- |
| insert-permanent | ➕insert-permanent/🦀️.rs | PermanentAction | snapshot/🦀️.rs:245 | PermanentKind |
| insert-seismic | 🌋insert-seismic/🦀️.rs | SeismicAction | snapshot/🦀️.rs:354 | ImportanceClass |
| insert-accidental | 💣insert-accidental/🦀️.rs | AccidentalAction | snapshot/🦀️.rs:270 | String/f64 primitives only |
| insert-effect | 📎insert-effect/🦀️.rs | MemberEffect | snapshot/🦀️.rs:440 | String/f64 primitives only |
| insert-variable | 📥insert-variable/🦀️.rs | VariableAction | snapshot/🦀️.rs:257 | String/f64 primitives only |
| insert-member | 🔩insert-member/🦀️.rs | Member | snapshot/🦀️.rs:373 | String/f64 primitives only |
| insert-bridge-sls | 🌉️insert-bridge-sls/🦀️.rs | BridgeSls | snapshot/🦀️.rs:415 | String/f64 primitives only |
| insert-fatigue-spectrum | 🔄️insert-fatigue-spectrum/🦀️.rs | FatigueSpectrum | snapshot/🦀️.rs:452 | String/f64 primitives only |
| insert-reliability-analysis | 🎲️insert-reliability-analysis/🦀️.rs | ReliabilityAnalysis | snapshot/🦀️.rs:479 | DistributionKind (3 fields) |
| insert-test-series | 🧪️insert-test-series/🦀️.rs | TestSeries | snapshot/🦀️.rs:511 | TestDistribution |
| insert-placement | 📍️insert-placement/🦀️.rs | Placement | snapshot/🦀️.rs:534 | String/f64 primitives only |
| insert-case | ➕️insert-case/🦀️.rs | CaseRow | norm/⚖️compliance/🧰️design-case/📇️rows/🦀️.rs | Vec<CaseNumber>, Vec<CaseText>; both shared row types |
| insert-factor-override | 🎚️insert-factor-override/🦀️.rs | FactorOverride | snapshot/🦀️.rs:552 | String/f64 primitives only |
| insert-traffic-group | 🚦️insert-traffic-group/🦀️.rs | TrafficGroup | snapshot/🦀️.rs:570 | LoadGroup |

All14 insertion leaves have index:Option<usize> except InsertCase index:usize; generic item key is item except InsertCase case. Each actual payload derives FromValue/RetireOwned/RetainedClone, value camelCase, with no current NativeReceive. All13 model records defined in EN1990 snapshot source likewise use value camelCase and FromValue/RetireOwned/RetainedClone. Shared CaseRow/CaseNumber/CaseText use same value contract and lack NativeReceive in their current definitions.

No skip, custom deserialize_with, flatten, custom default path or transparent attribute was found on these14 actual insertion payloads or their immediate record definitions. Standard value camelCase renaming is present; serde attributes are conditional test-support/compliance-testing only. No external physical-unit wrapper occurs in this closure: quantities are ordinary f64, sample_size in TestSeries is u32, and identity/text fields are String. Existing native primitive/Option/Vec support can be reused, but each domain record/enum and shared Case row child still needs its own genuine receiving implementation before claiming that insertion is supported.

Simple record-only expansion after BridgeSls: AccidentalAction, VariableAction, MemberEffect, Member, FatigueSpectrum, Placement, FactorOverride. Enum-bearing expansion: PermanentAction/PermanentKind, SeismicAction/ImportanceClass, TrafficGroup/LoadGroup, ReliabilityAnalysis/DistributionKind, TestSeries/TestDistribution. Shared nested case expansion must be owned in compliance CaseRow/CaseNumber/CaseText and then InsertCase; do not duplicate those definitions under EN1990. This is preparation, not runtime acceptance of14 or all15 families.
