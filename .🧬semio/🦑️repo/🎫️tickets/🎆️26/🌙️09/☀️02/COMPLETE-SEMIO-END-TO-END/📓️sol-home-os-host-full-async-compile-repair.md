# Home OS Host Full Async Compile Repair

## Scope

The registered Home identity/native packet reaches `semio-framework-os --lib --features os-host-full`. Receipt `exact-cargo-laws-C22uJD` built and ran the plugin-host inference effect law (`GREEN1`, executable SHA-256 `dc4dd506699e6bd6720be7b58b7546c018c104707e546986ea847d6635fc9cb8`), then stopped while compiling the OS host test binary. No Home identity law ran.

## Recovered diagnostic

`01/build.json` records Cargo status `101`. Its JSON diagnostics contain 149 errors:

- 28 missing `semio_framework_async_macros` resolutions from the included workflow test module.
- 55 type mismatches, 27 future-field accesses, 22 future trait-bound cascades, 12 missing result/vector methods on futures, three obsolete `dyn Backbone` uses, and two comparisons against unresolved futures.
- The concrete owned boundary was the OS host's synchronous facade and its tests after kernel APIs became async: `ConfigSpec`, `CommandGrammar`, `AppIo`, workflow constructors/planning, Store constructors/dispatch/parsers, backbone constructors/storage, and format registration.
- Two extension fixtures still constructed `serde_json::Value` after the extension manifest switched to the first-party JSON value.

## Repair

- Added the existing first-party async test macro crate as an OS-host dev dependency.
- Kept the already-published synchronous OS-host facade on its documented immediate-future boundary, including enum-dispatched `store::Backbones`; no compatibility wrapper or runtime API redesign was introduced.
- Resolved the affected pure/in-memory async constructors and calls at the existing immediate boundary in native host tests and synchronous storage ports.
- Updated extension fixtures to the first-party `semio_framework_os_kernel::json::Value`.
- The first warm retry, `exact-cargo-laws-Tj3Gs6`, kept the plugin-host group GREEN and reduced the OS-host compile frontier from 149 errors to 11 diagnostics at seven exact call sites. Those were repaired at their typed boundary: async `AppIo::with_ports`, fixed edit-message ledger comparison, borrowed cursor comparison, synchronous workflow validation/event-log construction/I/O registration.

## Qualification

Registered warm retry `exact-cargo-laws-60QtKH` stopped in group 00 before any Home law: the concurrently evolved durable-group composition omitted newly required `OpBinary + OpText` mutation bounds at its three `ArtifactStore::begin_apply_one` calls. Its owner repaired and source-qualified those exact bounds.

Immediate retry `exact-cargo-laws-v4J1FZ` produced two positive receipts: current plugin-host `GREEN1`, then `semio-framework-os --features os-host-full` compiled and its exact workflow law ran `GREEN1`. Group 02 completed its fresh Space dependency build but failed before test listing because the crate root mounted a removed logical fixture path. The actual bounded physical fixture is `📇️bumps-the-36f82f`; its stale `📇️bumps-the-catalog-generation-to-7` mount was repaired. A TDD source assertion first reproduced the stale mount and now proves both the physical fixture and exact mount; the language-agnostic/source packet is GREEN (`checks=14`).

The public-member cache was handed back for one coordinated warm retry, active as `exact-cargo-laws-52dKES`. The `C22uJD`, `Tj3Gs6`, `60QtKH`, and `v4J1FZ` receipts remain build failures, not complete Home runtime verdicts; `v4J1FZ` nevertheless provides valid scoped GREEN receipts for groups 00 and 01.
