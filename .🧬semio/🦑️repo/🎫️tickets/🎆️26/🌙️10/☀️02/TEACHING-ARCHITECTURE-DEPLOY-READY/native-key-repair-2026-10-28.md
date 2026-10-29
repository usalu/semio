# Native Key and Child Receiving Repair — 2026-10-28

The actual proctor build failed in semio-framework-value-native-owned with 228 Rust diagnostics. This work addresses dynamic map keys, fixed record keys and borrowed heterogeneous child projections; the backend agent owns the shared tree/list receiver generic and storage changes.

NativeRecordKey remains Name(&str) or Ordinal(u16). Dynamic string maps call record_name and refuse ordinal keys. Fixed field membership matches the declared field identity, retaining declaration ordinals independently of source iteration order and wire-name casing. Comparison demand quotes UTF-8 name bytes or the actual u16 width. Flattened fields pass the same key to their declared children and refuse ambiguous memberships; no ordinal offset or string conversion is invented.

Map and derived record/enum child slots receive dyn NativeValueSource borrowed projections from the actual original lease. Input I may be unsized, with a nonowning marker. Enum direct variant_tag/variant_payload handling selects the original payload carrier independently of canonical JSON tag fields, so typed text payload ordinals start at their own field zero. Canonical internally tagged carriers retain their actual tag field declaration.

All Rust NativeRecordFields implementations and callers were searched across the repository. Besides the owned shared delegate and both derive emitters, the OS store ArtifactChild receiver delegates the new key contract and admits an unsized input source.

The portable record fixture now includes named/ordinal identity, a numeric string that cannot alias ordinal zero, unknown ordinals and UTF-8 comparison cost. The map fixture distinguishes genuine names from ordinals. Rust regressions exercise the actual key matcher/cost, Name-only map key preview and ambiguous flattened ordinal zero. Existing prefix cancellation, exact physical grant, source identity and serde enum laws remain in place.

The independent native-record-oracle.mjs uses Ajv 2020 and Lodash plus the JavaScript Map implementation. It passed six field identities, four dynamic key cases and two ordered replacement cases. Source review by the independent contract auditor found no specific funding, borrowed-projection or enum slot alignment defect. Runtime verification remains pending the corrected single coordinated native-owned release --lib rerun, owned by the backend agent; no parallel Cargo job was started here.

No legacy string-key overload, proxy owner or ordinal stringification was added.

## Verification Scope

The native-owned --lib suite mounts the new record-key/cost and dynamic-map-name checks, plus flattened ordinal membership ambiguity and existing canonical enum payload tests. The direct text variant facet with ordinal payload law lives in the separate DSL schema receive/tests.rs module and is not compiled by this suite; it needs subsequent owning-crate verification. The portable independent oracle is the retained ticket input native-record-oracle.mjs, executed separately through managed Bun: `[DEBUG] {"fieldKeys":6,"dynamicKeys":4,"maps":2}`. It uses Ajv2020, Lodash and Buffer/Map and is not implicitly run by Cargo.

The first managed suite 18416 terminated with compilation status 101: one syntax error in the new map test used a comma between local let declarations. Corrected to a semicolon and let. No other compiler errors were reported; runtime tests did not execute. Backend owns the next coordinated rerun.

## Actual Native-Owned Result

The sole coordinated managed rerun, session 13230, terminated with status 0: 31 tests passed, 0 failed, 0 filtered. It includes actual map-key/record-key/canonical payload tests, new funded erased-source drivers for five receiver types and the mounted closure laws. This result follows correction of the map test syntax. Backend then launched one canonical proctor release build, session 12123; no concurrent DSL or Cargo job was launched here. The separate DSL direct-text ordinal payload law remains pending after the proctor result and parent priority.
