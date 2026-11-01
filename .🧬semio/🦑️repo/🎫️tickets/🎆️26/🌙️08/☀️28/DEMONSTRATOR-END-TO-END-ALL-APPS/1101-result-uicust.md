STATUS: GREEN native `native_resource_custody` 5/5 (default parallel threads, rc 0, log `🗑️generated/1101/uicust-3.txt`). Wasm/other crates not re-checked.

## Root causes (two, independent)
1. REAL FAULT, `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🎭️present/📦️resources/📤️transport/📨️message/🦀️.rs` `UiNativeMeshReceiving::receive_step`, `ResourceComplete` arm: the `surface/mesh_key is_none() -> Incomplete` guard ran on every call. The first call moves both names into `self.ready` (`take_ready`, returns `Ok(false)`); the second call (needed to set `complete`) then saw `None` and failed with "original mesh packet failed its exact receiving frontier". Production path would fail every resource receive at its last packet. Fix: guard only while `self.ready.is_none()`. Found with a temporary `[DEBUG]` backtrace in `mesh_receiving_fault` (the helper discards the `Mesh3dFault`, which hid the cause); removed again, `grep DEBUG` = 0.
2. TEST HARNESS, `📦️resources/🧪️tests/🦀️.rs`: the three `Busy` / `describe_mesh_on_original -> None` failures come from the process-global mesh authority (`mesh3d_authority().try_lock()` -> `Mesh3dFault::Busy`, by design a retry signal) while the 5 tests ran on parallel threads. Repro: `--test-threads=1` made 4/5 pass (`uicust-1.txt`). Fix: test-local `MESH_AUTHORITY_SERIAL` mutex guard (`serial()`) at the top of each test.

## Failure path (`OriginalDomainReservation` / `UiNativeFailurePlace`)
No failure-place fault surfaced; none of the 4 failures touched `admit_native_resource_failure`. Its behaviour is still not covered by a dedicated test (only compile-proven on wasm by p2d). Not added in this run.

## Files
- `📤️transport/📨️message/🦀️.rs` (guard fix), `🧪️tests/🦀️.rs` (serial guard), tool `1101-tools/uicust/t.sh`.
