# Checked DSL Integer Mutation Codecs

## Contract And Implementation

Every shared signed/unsigned integer `DslField` binding now uses checked conversion from the wire's `i64`/`u64` representation. An out-of-range value returns an error instead of truncating into a different mutation payload. Generic binary variant ordinals are checked before native-index lookup and before wire encoding. Floating-point conversion is unchanged and is not covered by this claim.

The permanent language-neutral decimal-string fixture contains51integer boundaries across all ten integer bindings, five u32 codec cases and four single-variant ordinal cases. Decimal strings preserve exact64-bit values in every implementation. Native tests compare the real DSL binding with independent typed `serde_json` decoding, exercise derived text/binary records, nested collection propagation and malformed ordinal input.

## Actual Red And Green

The pre-fix neutral/source gate was352of355: only the three checked-conversion source requirements failed. [Its retained result](./🧪️dsl-checked-integers-41/🧫️run-y8S5zB/🔣️.json) is unchanged.

The real OS-kernel native gate then compiled in61.744s. It selected six tests: one passed, one failed and four were unexecuted after the first failure. The failure proved that `set-index index=4294967296` was accepted by the actual derived u32 text decoder. [The native red receipt](./🧪️os-kernel-fixtures-41/🧫️run-g0eX07/🔣️.json) and its test001log retain this exact counterexample; no compiler-only failure is represented as behavioral red.

After the checked-conversion change, the unchanged neutral expectations passed355of355 in [the source green run](./🧪️dsl-checked-integers-41/🧫️run-O4G1Yf/🔣️.json). The native suite compiled in29.834s and passed all six tests in [the native green run](./🧪️os-kernel-fixtures-41/🧫️run-1e2kG5/🔣️.json). Runtime output confirms51typed vectors and a64-bit native host. The32-bit cases were checked by the neutral exact-integer model; a32-bit Rust target was not executed.

Captured source inputs were stable in both native runs. The source capture is the explicitly recorded OS-kernel fixture/dependency subset, not proof of complete transitive build-input closure. No Plugin, Flow or GIS native test ran. The subsequent101of101Store/SPR native fixture pass is recorded separately in [the Store/SPR report](./📓️os-kernel-fixtures-native-41.md).

## Frozen Source Boundary

- DSL facade: `5e02c46094f85d87195816406a826b7f93e0f14a7f3562556331b1262c6e2c05`.
- Native tests: `1cf2849c7485fd4535ff0e18a463b65c752b6ef5ebc736a3a294db9d30375b4f`.
- Neutral vectors: `dfa9cf2a166330d75bc0f8b7957d96538247d50c5cccf90bfe272ef877bc08ad`.
- Neutral controller: `e47e2b48b4bd946baaf54f340084b956aee06c1524fec4fb6ebb10c8570a82a8`.

All logs, failed runs and compiler caches remain retained. The borrowed compiler slot was explicitly released after the final Store/SPR gate; exact-name Cargo/rustc PID checks found none. Launch source/native rows are coordinated with taxonomy at orders410.401 and410.41; the unrelated existing410.4 row remains preserved.
