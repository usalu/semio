# Owned Auxiliary Source Transport

Status: source accepted by the independent bounded review; runtime unrun. Controls has no producer or live child. High owns actual BuildCLI admission and launch conversion.

## Contract and implementation

`runOwnedCommand` accepts closed primitive `auxiliaryInput: { version: 1, text: string }` under its actual issued control and the same cancellation signal. The UTF-8 byte ceiling is 25,165,824, matching the unchanged Source snapshot policy. It publishes DATA on owned descriptor 3, preserves ordinary stdin (inherited or explicitly supplied), streams stdout/stderr through the existing backpressured ports, and awaits auxiliary publication and actual child close. No source file, temporary/store snapshot, environment blob, renewed Original, frontend fallback, or receipt change is introduced.

The first-party structural pipe interface retains callback-first errors, event errors, premature close, and explicit synchronous/event `undefined` failures. Callback null/undefined means successful completion. A separate failed flag preserves the first actual failure value; cancellation and output-consumer primary identity remain observable after terminal settlement.

`buildSourceClosureStreamLoaderV1` consumes only descriptor 3; there is no stdin fallback. One shared immutable loader body validates both the retained file transport and the auxiliary transport. The latter binds the actual single `--eval` bytes from `process.execArgv`, checks exact snapshot SHA-256 and argv, injects the captured source map, retains all module/import/schema and physical-image fences, and passes `{ version: 1, snapshot: null, sha256, loaderSha256, root, recheck }`. The caller keeps its cwd. Root is the validated snapshot root. Fixed 64 KiB accumulation retains at most 384 blocks, including fragmented one-byte reads, and preserves the cap-plus-one EOF probe.

## Authored checks

Closed neutral fixtures cover six auxiliary UTF-8 boundary cases and nine snapshot/digest/eval/fragment cases. Ajv validates the closed cases; SQLite independently computes UTF-8 BLOB byte lengths and policy outcomes. The same production publication port is exercised with callback-before-event, event-before-callback, close, null/undefined success callbacks, late errors, and explicit undefined failures.

Eight real owned modes cover inherited stdin through a nested owned child, a payload exceeding two MiB, early fd3 close with a live child, cancellation, deadline expiry, exact output-consumer primary, actual status 7, and thirty MiB of live output without collecting it. The real evaluated-loader law checks argv including Unicode/empty arguments, source map/root/proof identities, ordinary stdin, changed snapshot digest, changed physical source, and post-entry physical recheck. These bodies are authored and unrun.

## Mounts and boundaries

Source0 imports only the focused auxiliary stream leaf; it statically imports the auxiliary suite. The existing process owned-execution suite also imports that suite. Both existing declarations explicitly include the genuine child script runtime entry. Source0 retains uncached/live classification and its existing freshness/dependency/output behavior; no roster owner is added. All twelve exact names are retained in `e2e-source-single-plan.json` and the authored transport plan.

Static parsing and literal relative-import/path checks found zero errors; maximum absolute source path was 140 UTF-16 units. Generated loader-body syntax was checked without launching it. This does not establish Bun extra-pipe behavior on Windows or any actual runtime body. Actual eval argv/pipe behavior, Original guard/current-source admission, inherited stdin, terminal/error handling, and the twelve named bodies remain required. The actual BuildCLI admission/caller transition is separately owned by High and must be jointly source-frozen before sync or cold dispatch.

## Exact source observation

Shared root declaration/main hashes are observed bytes; Root may append its separately authorized dependency law/inputs afterward. No exclusive peer attribution is claimed.

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟦️.ts` — 27258 bytes; SHA-256 `c796b9210f10e9084fd6098c766dd498f3949bd84a73c55f04fca53fc4b7dd85`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🧬️schema/🔣️.json` — 6150 bytes; SHA-256 `8f3141139c85b7a5631e7dbab2f20ccbcea44cf8f877adfe5716017aeffc0112`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🧪️tests/📥️stream/🔣️.json` — 1357 bytes; SHA-256 `1af345ac05363aed23dc9e75037ad986b08494ef2ae904ed7af8d3161a187f19`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🧪️tests/📥️stream/🟦️.ts` — 6582 bytes; SHA-256 `e252e5d1b62992ea6b55c516cbc205f9acf7cd1ec76603e3ebe4b04666cbcf26`.
- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️auxiliary/🟦️.ts` — 2449 bytes; SHA-256 `8006bf42dc10c033aaa6a3bf2fa32e06a8e9d4dbe43f7978ebc4be0fa9231329`.
- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️auxiliary/🧬️schema/🔣️.json` — 1082 bytes; SHA-256 `1bab4709501f27bb272273c5e3c1fa6965f12058dda0f3292193e2fdac78bd97`.
- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️auxiliary/🧪️tests/🔣️.json` — 714 bytes; SHA-256 `1c780752e235db3cb5b47d3dc8e35246f51bc7d9887b2085ab35023ddd13ceff`.
- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️auxiliary/🧪️tests/🟦️.ts` — 5256 bytes; SHA-256 `bc887d9ed0eba4a11cd00de221dad70e8b68ee2fed674323c5accb82fa4268f0`.
- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️auxiliary/🧪️tests/📜️script.ts` — 3208 bytes; SHA-256 `9b401518aebca051d3f544c0f1c5ca447c19a266ced2d18d41fa4a21b793eb78`.
- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts` — 16377 bytes; SHA-256 `0455563d375770f72fc07a81ccda7c4d2df8f7543e04f629404aed68f6dc4833`.
- `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts` — 1981 bytes; SHA-256 `c96b98b763e4c440bfd6d85868734011650d0d0547c647335ed6f2aff35ebcde`.
- `🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🧪️tests/🟦️.ts` — 12329 bytes; SHA-256 `600a21163fb7230749f27816eb7f1d93b22df42564d8e0819854cd23639edb9d`.
- `🧰️framework/🔨️modules/🏃️process/📋️project.json` — 34794 bytes; SHA-256 `52f0acd2e9a071249fde922de9ccaad3b04ee2577b4f7548582e21621f0560d2`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🟦️.ts` — 6480 bytes; SHA-256 `fbe66b19ca9248986a0e5ca68667cb94f3c6a949e6b28f667d8fd2fd046642d2`.
- `📋️project.json` — 166119 bytes; SHA-256 `d2f516884b48d0f6901270d9b6648abe2f1fa65e67ae4091d6c9ab866ab3a162`.

## Exact required bodies

- `owned auxiliary DATA UTF-8 boundaries match the independent SQLite contract`
- `actual owned auxiliary pipe: inherit`
- `actual owned auxiliary pipe: large`
- `actual owned auxiliary pipe: early-close`
- `actual owned auxiliary pipe: cancel`
- `actual owned auxiliary pipe: deadline`
- `actual owned auxiliary pipe: consumer-primary`
- `actual owned auxiliary pipe: status`
- `actual owned auxiliary pipe: large-stream`
- `auxiliary publication preserves callback event and undefined primary failures`
- `immutable auxiliary snapshot boundaries match the independent SQLite contract`
- `actual auxiliary immutable loader binds eval bytes argv and physical Source`
