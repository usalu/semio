# Executable Raw Replay Authority Companion

2026-10-28. The earlier Report law was in a preexisting unmounted test source, so it was not executable. It has been replaced by a focused public raw replay companion. All preexisting Report tests and their input fixtures remain; only the newly added unmounted law body was removed. Its earlier authority.json input remains retained.

## Genuine Native Owner

Package: semio-framework-os-store-replay-authority-tests.
Manifest: 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🧪️tests/🔐️authority/📦️packages/🦀️rust/Cargo.toml.
Actual lib entry: ../../🦀️.rs, guarded by cfg(test), containing two real #[test] functions:
- original_missing_producer_preserves_original_state_and_frontier
- explicit_cold_executor_matches_independent_authority_witness

Rust registered both actual workspace memberships; root registered the exact package build owner. The companion depends only on public raw replay, mutation/wire, value, native decode and history domain libraries plus serde_json for tests. It requires no engine dependency or testkit feature and does not mount the oversized engine test closure.

The native law uses a handcrafted schema-defined counter mutation and typed sparse counter diff. Missing original producer must refuse UnsupportedOwner with unchanged snapshot pointer, cursor, operation, progress and output extents, zero native receipt and zero native decoded ownership. The explicit cold entry executes exactly one real mutation, checked by a thread-local call observer, and matches the same neutral output record. Both paths retire their actual raw replay through its public original cleanup owner.

## Independent Behavior Oracle

The companion's 🟦️.ts consumes the exact same 🔣️.json neutral fixture and executes fast-json-patch's RFC6902 operations. The reference document includes the explicit cold-authority capability and counter/callcount state. A standard test operation validates that capability before the independent replace operations may change the state. TEST_OPERATION_FAILED maps to the domain UnsupportedOwner refusal. It computes accepted/unchanged/error/state records through the independent library, then compares those records to the neutral witness. No semantic result is substituted for executing the library.

This is a behavioral oracle; serde_json parsing by itself is not claimed as behavioral evidence. No external runtime dependency was introduced.

## Actual Validation

Canonical source-only command: repo --service=off tools exec --light bun -- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END/authority-inventory.ts

Actual cfg(test) closure:3files/7136bytes/76active lines. This fits unchanged32/262144/4000 bounds. The retained script reads only this actual package entry and cfg(test) source closure.

Independent behavior command: repo --service=off tools exec --light bun -- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🧪️tests/🔐️authority/🟦️.ts

Actual result:2 cases passed. Original returned accepted=false,unchanged=true,error=UnsupportedOwner,state={counter:7,calls:0}; explicit cold returned accepted=true,unchanged=false,error=null,state={counter:11,calls:1}.

Rust executed the whole package with no filter in the genuine repository-root workspace. Both named cases executed and passed; the successful command and real count are recorded below.

The first native attempt with the framework manifest was refused before compilation because the package's declared Cargo workspace owns it at repository root. Rust is resuming through the explicit repository-root Cargo.toml package selection; no test pass or zero-case result is inferred from that refusal.

## Actual Native Execution

The genuine repository-root Cargo package run completed successfully: semio-framework-os-store-replay-authority-tests,2passed/0failed/0filtered. Both exact named tests executed. Native original output was accepted=false,unchanged=true,error=UnsupportedOwner,state={counter:7,calls:0}; native cold output was accepted=true,unchanged=false,error=null,state={counter:11,calls:1}. These equal the independently executed fast-json-patch records from the same neutral fixture. No fixture-only or zero-filter result was counted. Rust retained the full command output under 🗑️generated/replay-authority-native-2.log until ticket cleanup.

Successful native command (working directory /Users/ueli/Documents/semio):

repo --service=off cargo --store=shared --configuration=prototype -- test --manifest-path Cargo.toml -p semio-framework-os-store-replay-authority-tests --lib -- --nocapture

Actual exit status0; session68410.
