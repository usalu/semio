# Stdio Component Link Boundary

Investigation and repair packet for the final-source `wasm-component-ld` failure recorded by `📓️sol-stdio-catalog-root-completion.md` on 2026-09-03.

## Invariants

- Do not raise or patch wasmparser/component-model safety limits.
- Use only fresh, ticket-owned Cargo targets as evidence.
- Preserve `semio:stdio`, strict raw/core/descriptor identity, retained verified bytes, marker-last publication, owner-derivative independence, and empty native-codec admission.
- Do not hand-edit generated registry artifacts or broaden into residual descriptor owners.

## Initial evidence

- The valid clean-root build invoked `cargo rustc -p semio-s-plugin-stdio --lib --crate-type cdylib --target wasm32-wasip2` with `-C codegen-units=1`.
- `wasm-component-ld` 0.5.25 first invoked `wasm-ld`, then failed while decoding that linked core for componentization: `functions count exceeds limit of 1000000 (at offset 0xdc7)`.
- The offset is in the linked core's early function section, so the next diagnostic produces the exact linked core through the linker's supported `--skip-wit-component` mode and parses its section counts independently. This distinguishes a genuine linked function count from component-wrapper corruption or script path reuse.

## Measurements

- A fresh debug-profile link using `wasm-component-ld --skip-wit-component` did not produce its linked core within the enforced 20-minute aggregate diagnostic bound. It was cancelled and its target removed. It is timing evidence only, not artifact/count evidence; it inherited the repository compiler wrapper and is excluded from final admission evidence.
- The language-neutral fixture now contains empty, one-function, and declared-1,000,001-function core modules. The bounded first-party section reader agrees with `WebAssembly.validate`; Binaryen 130 independently parses and rewrites both valid vectors. The over-limit vector is rejected without changing the parser ceiling.
- The final catalog operation explicitly disables `RUSTC_WRAPPER`, sccache, and Cargo incremental compilation and selects the repository's existing deterministic `wasm-release` component profile. Its aggregate deadline remains bounded at 20 minutes.
- A fresh `wasm-release` core-only build with sccache disabled reached the single stdio `rustc` optimized codegen/LTO/link process but did not emit a core before the 20-minute aggregate wall. It was cancelled before the wall and its isolated target was deleted. There is therefore no admissible optimized count/size yet, and merely changing the catalog profile is not claimed sufficient.
- A fresh expansion-only probe streamed compiler stdout through a 64 MiB ceiling and retained no expanded source. It reached 67,417,736 bytes in about 21 seconds before the consumer deliberately closed the pipe. That prefix contained 5,155 `fn` tokens, 16 `export_name` attributes, and 12 `no_mangle` attributes. The result proves substantial source/proc-macro expansion, but it is only a prefix and `fn` tokens are not linked-Wasm function definitions; it does not explain or validate the million-function claim by itself.
- A second streaming probe used a 128 MiB/60-CPU-second ceiling and retained only a 1.7 KiB histogram. It stopped at 134,704,382 bytes with 10,594 `fn` tokens, 5,902 `impl` tokens, 1,237 `struct` tokens, and 437 `enum` tokens. Both the first and last `StdioApps` variants appeared in 78 generated arms, proving that the closed dispatch was wholly present in that prefix.
- An exact-delimiter probe then consumed only the expanded `StdioApps` region. The complete 176-variant/77-method closure is 1,702,133 expanded bytes, contains 254 `fn` tokens, 176 `From` implementations, 13,728 match arms, and 528 `VcsArtifactApp` mentions. It is linear and compact compared with the crate's greater-than-128-MiB expansion. The dispatch proc macro is therefore not a pathological million-function source emitter.
- The package root has 4,688 `#[path]` attributes, 2,437 of which are the inline taxonomy-directory marker `#[path = "."]`. Exact path-string census finds only three duplicated physical source modules: the CSV, XML, and TIFF demo example compatibility shims. Those three small duplicates cannot explain the linker count. No broad duplicated source mounting was observed.
- A ticket-local representative workspace defined structurally identical `plugin_exports!` component roots at N=0, N=2, and N=8 app variants. N2/N8 use real stdio PNG/JPEG/BMP nested `VcsArtifactApp<EditorApp<_>>` and `VcsArtifactApp<ViewerApp<_>>` members; all points carry the same component-guest/plugin features and the same no-default-feature stdio dependency. The planned stop rule was five minutes and 64 MiB per core, with first-party function/code-section agreement plus an independent Wasm parser before extrapolating to N=176.

## Repair and verification

- The permanent stdio operation now selects the existing `wasm-release` profile explicitly and disables the repository compiler wrapper, sccache, and incremental compilation through both environment and Cargo's command-line config. This is deterministic build hygiene, not claimed as the linker repair: the fresh optimized core-only build did not terminate within the bound.
- The language-neutral catalog-root fixture has schema-v2 empty, one-function, and declared-1,000,001-function core vectors. The first-party bounded section reader rejects malformed/over-limit/function-code-mismatch modules without raising the parser ceiling; JavaScript's independent `WebAssembly.validate` and Binaryen 130 agree on the valid vectors.
- `NX_SKIP_NX_CACHE=true bun nx run @semio-tech/stdio-plugin:test-quick -- catalog-root-contract` is final-source green in 3.4 seconds for the structural fixture/oracle laws. Binaryen prints its expected two “no passes specified” notices and reports no failure. No runtime component or catalog row is claimed by this gate.

## Current blocker and nonclaims

- The current shared S3 atomic cutover prevents the N=0 representative from reaching the diagnostic crate. Two fresh isolated attempts stopped before any core was emitted with exactly three `E0599` diagnostics in `semio-framework-os-kernel`: `DirectoryClient::set_token` is absent at `📇️directory/🪪️identity/🦀️.rs:176` and `:192`; `DirectoryClient::mint_session` is absent at `:189`. The N=0 dependency and feature set was made identical to N2/N8 before the second attempt, so this is not a representative-fixture feature difference. It belongs to the concurrent S3 owner and was not patched here.
- Consequently N=0/2/8 core sizes, function counts, code counts, RSS, and growth curve remain unmeasured. There is no independent linked core with which to decide whether the earlier component-linker diagnostic is a genuine million-definition module or a malformed handoff at that exact boundary.
- The optimized `wasm-release` profile is unproven. No fresh catalog-root rerun can be final-source evidence while the shared dependency does not compile, and no shard schema was introduced without the required measured growth curve.
- The original fail-closed result remains authoritative: no raw component, extracted core, descriptor triplet, catalog row, completion marker, or native-codec receipt was published. Package identity, marker-last publication, retained-byte verification, and empty native-codec admission were not weakened.
- All ticket-local expansion logs, temporary representative sources, and their isolated Cargo targets were deleted after these conclusions were recorded.
