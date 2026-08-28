# Paired Node And Scene Ownership

`retained/scene/binding` now owns the exact wire node and its prepared scene in one private reference-counted root. Capturing an issued pair increments only that root, avoiding sequential node/scene capture failure. Construction moves already-owned handles from its retained preparation cursor. Callers cannot mutate or close the prepared scene owner: they receive immutable node/diagnostic reads or independently owned record/text/value readers.

`OwnedUiNode.captureComponent()` is a narrow exact direct-field capture. It does not normalize, clone or reconstruct the component. Preparation owns that handle before creating the raw parser, owns the raw scene before creating typed projection, and explicitly closes each intermediate owner. Malformed packets, unsupported versions and invalid fields preserve the original node with a fixed diagnostic code. No successful empty substitute is created for a supported malformed scene.

Previous projection reuse compares the exact immutable component object identity while retaining the previous paired owner. A style-only replacement shares the prepared scene, then retires the temporary previous pair before readiness. A component replacement always prepares a new scene. The result does not retain a previous node/binding ancestor. Cancellation stores every potentially unbounded owner in cursor fields, not generator locals.

## Executed Verification

Canonical target: `bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --args='--run -t SceneBinding'`.

- R1: missing-module source RED,4failed suites,0executed tests.
- R2:1failed test due a missing closing brace in the newly authored fixture schema, not owner behavior. The schema was corrected; no semantic expectation changed.
- R3:1PASS/575skipped/576total,37.10s,start17:13:02; four neutral valid/diagnostic cases, strictAjv, safe53-bit node identity, paired aliases and prepared reader survival after both aliases close.
- R4:2PASS/575skipped/577total,14.72s,start17:13:43. Adds cancellation at every observed top-level phase and ready boundary, exact component-identity reuse without packet traversal, stale projection rejection after component replacement, and old prepared-reader survival after all source nodes/bindings close.

These are targeted tests; no full577-suite or mounted-renderer result is claimed. Native typed15valid/6hostile serde parity independently passed1test/94skipped in child R3. Additional numeric/domain edge parity is being prepared following coordinator review.

## Fixed Metadata Accounting

Each typed prepared field owns one object with four slots (`name`, `type`, `source`, `literal`). Name/type/default string and array literals are references into the permanently frozen static catalog, not newly allocated per-record strings. With32 fields, four8-byte slots plus a conservative32-byte object header per field =2048 bytes; a32-entry8-byte field array plus64-byte header =320; one prepared-record object and task/list headers are below256. The retained3072-byte cleanup charge therefore covers a maximal fixed record/task even with conservative metadata allowance. This is a logical owned-metadata envelope, not a JS allocator or8ms timing certificate. Native input strings stay in original pages and are never included as unaccounted final drops.

Binding cleanup passes through actual child byte counts unchanged. Its linked retirement queue detaches an exhausted queue node on a separate32-byte turn rather than adding overhead to an arbitrary child grant. A paired root release itself touches fixed metadata only; final scene and node payloads are closed separately.

## Remaining Mounting Work

Parallel prepared binding index, atomic node+binding publication, read-lease integration, incremental nested host JSON/pack projections and exact per-instance aggregate close remain unmounted. Interpreter and wgpu retain their existing synchronous scene path until the complete prepared host boundary is ready. No files were cleaned, deleted, relocated or committed.
