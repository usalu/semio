# Controlled Value Encoding

The canonical `ToValue::to_value_controlled` and `to_object_key_controlled` interfaces initially refuse missing owner implementations. The shared encoding control belongs to the I/O worker. This worker owns scalar/container/intrinsic implementations and field-by-field `ToValue` derive generation, including explicit `serialize_controlled_with` custom conversions.

The schema-first neutral fixture lives beside the codec under `🧫️fixtures/🛫️controlled/🔣️.json`, with a strict JSON schema and independent serde and Bun SQLite/Ajv oracles. Initial native laws require derived ordered fields, intrinsic octets, scalar output, allocation admission and real collection cancellation.

The executable owner route is `@semio-tech/value-rs:test-controlled-encoding`, registered in its existing script/project and both launch files. The existing controlled-construction source route includes the additional independent output oracle.

## Verification

The actual registered native RED ran five selected laws: zero passed, all five failed in 0.142 seconds. Derived/scalar/intrinsic calls reached the strict missing-owner error; collection and nullable-frontier cancellation laws confirmed no interior work. The 95 unrelated Value laws were intentionally filtered out for this genuine feature RED.

The source oracle initially retained seven existing green laws and rejected the new schema because Ajv strict mode disallows an object/array/string `type` union. The handwritten schema now uses explicit `anyOf` alternatives. Its actual final uncached registered retry passes nine laws with 138 assertions in 5.73 seconds.

The implementation now covers primitive words, bounded stack formatting of native numeric keys/Binary32 decimals, strings, lossy platform paths, optional/boxed values, typed sequences/sets/maps/tuples, intrinsic iterative cloning and explicit custom byte helpers. Derived objects and enum representations admit slots and keys and guard every completed output before further allocation. A recursive derived projection is limited to 64 typed construction levels; intrinsic output uses a controlled iterative frontier and iterative retirement. No ordinary custom serialization fallback is present. Thirteen strengthened native laws passed the actual registered uncached route in 0.492 seconds of assertions (Nextest `8ae785ba-b55f-44ba-9c32-098860f49617`, 3m52s total).

The initial actual implementation retry stopped before compilation on graph admission; this is distinct from the genuine five-law feature RED and the subsequent native/source GREEN.

## Current verification prerequisites and strengthened ownership laws

The initial implementation retry stopped before compilation: emoji node creation reached its ten-minute cap and the resulting incomplete graph lacked the Writer test-host and renderer frame-worker producer nodes. The parent independently confirmed both actual source declarations and all exact nodes in an admitted current graph; no aliases, producer duplication or timeout bypass were applied. One fresh current native/source verification completed successfully with the same warm Cargo directories. Full Value regression is now running separately; physical Text/Pack producers and erased output dispatch belong to the I/O worker and are not claimed complete here.

The suite now retains thirteen native laws, including concurrently authored field-binding and caller-namespace laws. Produced enum fields use explicit generated aliases, so fields named `control`, `__output`, `__wrapper`, `__payload` and similar identifiers cannot shadow the controller or builder. The caller-local `Result` type is handled through fully qualified core result signatures. Entirely skipped records allocate no output slots, conditional predicates run once, and ordinary serializers remain uncalled under controlled custom conversion. Independent JSON meaning is compared through serde values, while declaration field order is checked separately against the neutral `fieldOrder` vector.

The first full-regression invocation accidentally placed the Nx cache flag after `--`, forwarding it to Nextest; Nextest rejected the argument before any assertions. The corrected registered invocation places `--skip-nx-cache` before the separator and is pending. This command error does not change the focused thirteen-law or nine-law source evidence.

The corrected actual registered uncached full Value regression is GREEN: 108 tests passed, zero skipped, 5.073 seconds assertions, Nextest `45c4678c-92db-49ba-8154-f876f7bb7cca` (6m29s Nx). This includes the thirteen output laws and all ninety-five prior input, lifecycle and value laws. Seven nonfatal compile warnings were emitted, six unnecessary qualifications in older unit/retained-map tests and the deliberately skipped output fixture field. No global warning suppression or deadline change was introduced. Physical output encoders and erased output activation remain separately owned and pending their own proof.
