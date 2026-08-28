# Dag Actual Schema Review

The staged Dag schemas were reported with a passing descriptor-proposal and fixture-container check. That controller did not read or compile the actual new schema files. This independent packet tests those files with Ajv2020, retaining all resolved schema hashes and before/after stability. It never uses real `compose/**` or copied mutation implementations.

Six fixed vectors cover a positive leaf control, two actual internally tagged aggregate envelopes, missing intrinsic node fields, a safely representable index above u64, and a missing intrinsic kind tag. The index vector is not a claim about exact JavaScript representation of u64 endpoints; later Rust and decimal BigInt checks remain necessary.

This is schema validation only. The staged Rust aggregate is unmounted, and intrinsic serde/DSL mismatches remain separate source work. Results are recorded after actual execution below.

## Actual Red

The scoped Bun/Nx invocation exited1 in `🧪️dag-schema-independent-36/🧫️run-wnFYfr`, with all schema/vector/controller hashes stable. One positive leaf control passed; both genuine aggregate positives were rejected, while all three malformed payloads were accepted. This proves five actual schema defects, not merely missing test coverage. The correction lane now owns the schema files and their real Ajv validation; the independent expectations remain unchanged.

## Actual Schema Green

After the actual schema correction, root reran the same six fixed vectors in `🧪️dag-schema-independent-36/🧫️run-WopQaT`:6/6passed, exit0, all inputs stable. The author separately retained925schema assertions in `🧪️dag-schema-repair-37`. This accepts only the bounded schema defect repair. Dag's Rust aggregate mounting, intrinsic type/DSL changes, all14leaf codecs/inverses, ordered diff composition and consumer adoption remain active source work, not compiler/runtime acceptance.
