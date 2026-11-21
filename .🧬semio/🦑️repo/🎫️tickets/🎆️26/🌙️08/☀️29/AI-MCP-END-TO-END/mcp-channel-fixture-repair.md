# Owned Channel Fixture Repair

The synthetic component now emits the signatures declared by `OwnedSemioExport::core_type` for all 36 exports, including mixed i32/i64 actor input and the distinct job and failure arities. The exhaustive match makes future export additions visible to the compiler. Channel result, inert allocation and deallocation, and traps on unrelated calls remain unchanged.

`mcp-channel-fixture-law.ts` compiled the actual fixture against the pinned previously built first-party Core interpreter, parsed and instantiated the full component, checked channel result `42` and the unrelated export trap, then independently instantiated its Core bytes in Bun native WebAssembly and checked the same observations. Passed.

`mcp-channel-admission-law.ts` linked the pinned previously built current owned runtime and the actual fixture, then checked admission for the host channel and the stable mismatch fault code and host/guest parameters for adjacent channels. Passed. Both checks used existing exact dependency metadata and the pinned Rust toolchain without Cargo. The existing runtime codec-path test was not independently executed.
