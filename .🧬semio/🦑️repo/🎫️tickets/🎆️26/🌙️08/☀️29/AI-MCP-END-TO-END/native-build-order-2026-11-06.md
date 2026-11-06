# Native Cargo Publication Order

## Actual Terminal Evidence

Source request 86764 reached an actual operation exit 1 before compilation after 959651 ms. The complete result is generated/mcp-43892.json. Process 27880 held isolated namespace dfb029aa36f2 exclusively while awaiting cargo:configuration-publication exclusively for 15871 ms; process 52212 held the latter. The independent parent request14450 has the same shape (generated/mcp-71172.json, namespace40d145001c8a, process52588). These are genuine lease-guard refusals, not native Rust law failures.

## Owner And Correction

The isolated store owner is library/process/cargo/launcher.withRepositoryCargoIsolatedStoreV1. Its raw12hex resource is the unique temporary namespace, not the storage mapping. The old owner held EX through configuration capture, registry acquisition, capacity admission, compilation and consumption. That made every publication wait an exclusive hold-and-wait.

The closed launcher schema now distinguishes namespace liveness SH and maintenance sweep EX. The live SH excludes the sweep's immediate EX probe throughout the body and creator cleanup. Atomic nonrecursive directory birth admits exactly one actual creator; a nonce collision releases only the new reader and preserves the existing directory. Payload writes still require their existing genuine storage reservation and Cargo build/profile custody. The lease-order guard, timeouts, caps and maintenance exclusion are unchanged. The creator owns its private directory by actual successful birth; SH is a liveness guard, not writable storage authority.

## Tests

The neutral lifetime schema/corpus is next to launcher tests. The actual owner function is extracted unchanged with TypeScript for its namespace unit; only storage authentication/path ports are substituted, so this test does not claim storage admission qualification. It uses a genuinely declared first-party test Source, real ResourceLease acquisition and Source control, actual queued configuration contention, and independent SQLite transaction exclusion. RED reproduced the exact unclassified12hex EX to configuration-publication violation before the implementation change. GREEN:2 tests,15 assertions, including an actual existing-owner nonce collision and marker preservation. Output is generated/isolated-order-{red,green}.txt. Existing full Native image/forged-reservation laws remain intact and are not claimed as run here.

## Testing Owner Discovery

The pure Wasm component generator was canonically relocated from interpreter/🧫️fixtures to interpreter/🧪️components because the former is a reserved data collection excluded by the workspace discovery contract. Package identity semio-plugin-interpreter-fixtures and every function remain the same. Root member, project root/sourceRoot, and both Runtime optional/dev paths were handcrafted together. The previous source location is absent; no discovery exception or compatibility path was introduced. Launch owns independent closure validation.

## Retry Gate

Old Source process27880, MCP73524 and wrapper43892 are absent (ESRCH). The owned leases_status --all snapshot generated/mcp-69812.json covers4directories and contains no matching oldPID, agent or session. No replacement native request has been started after this fix yet. Parent process12035 is left untouched. Actual Rust/native integration qualification remains pending.

## Shared Mapping Order

Configuration publication also performed storage rechecks while holding its global writer. The new withNativeStorageReservationOperationV1 requires the actual private-factory reservation and genuine descendant control, then delegates the existing Source operation/physical backend port. Pending operations and independent failures keep their existing owner. The actual mapping read therefore encloses the narrower publication writer; rechecks reuse that live owned frame. No cached check or raw backend port escapes.

Shared regression: observed RED (actual parent mapping transaction missing at publication) to GREEN1test10assertions, using real private-factory reservation and Source-issued parent/descendant controls, real mapping/publication leases, and an independent SQLite order oracle. The test substitutes the physical hard-cap backend only, so it does not claim Native fixed-image qualification. Copied and retired reservation objects refuse before the publication body. Both shared and isolated regressions are registered in their owning suite entry files. Combined GREEN3tests25assertions. Strict TypeScript checking of the5edited owner/test roots and their dependency closure with actual repository compiler options reports zero diagnostics.

The preserved full Native-image lifecycle law was attempted directly and refused before its body because the bare fixture has no captured runtime entry provider (generated/isolated-native-lifecycle.txt). This remains an integration limitation, not a law pass; configured native execution must supply that real provider.

Launch completed the actual MCP closure census after the component move:464packages,3workspace scopes,0issues. See mcp-membership-census-2026-11-06.md. Source edits are coherent for the next owned native qualification.
