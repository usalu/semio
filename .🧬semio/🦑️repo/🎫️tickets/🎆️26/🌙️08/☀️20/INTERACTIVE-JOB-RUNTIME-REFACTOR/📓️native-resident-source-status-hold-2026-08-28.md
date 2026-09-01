# Native Resident Source Status And WGPU Capture Hold

## Exact Status

This is a read-only status inspection after the parent's follow-up. All files are preserved. No Rust/Cargo/Nx native command ran in this lane. Production editing is paused for the requested bounded capture.

The native resident private-consumer repair is **in flight and not compile-ready**. Its sole Rust authority now contains an uncompiled candidate typed consumer page, explicit Layout allocation, non-owning private consumer capability, phase-checked install/read/close, and erased admission alias handback. Existing test callers still construct `Arc<Mutex<Option<C>>>`; those no longer match the candidate `ResidentConsumer` parameters. No test migration, fake successful result or compatibility API has been mounted. The added `consumer_page_bytes` layout field also still needs the canonical native layout fixture/caller join. The R6 foreign-refill test still describes its old Arc caller and must be coherently migrated to the new private capability while preserving attempted-source custody and the original zero-destructor assertion.

The candidate has not had even a library compile, so type/borrow errors remain possible. The admission and record backing paths still use their earlier Vec-based representation; the new explicit Layout work is currently only the consumer-page slice. The registered actual-parent receiver is not implemented: current C/S handoff methods still take structurally retained Option destinations and must not be advertised as funded authority. Original-root loss/poison, actual Opening construction and completed callback-tail ownership remain open.

## Owned Source And Current Hashes

All paths below are rooted at `/Users/ueli/Documents/semio`.

| Path | Current SHA256 | Status |
| --- | --- | --- |
| `🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs` | `ad1674b38b6a648afb9ea0657a9ba2ce5f2ba572e2a0407a471c43165a6150d4` | Sole in-flight Rust repair |
| `🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️tests/🦀️.rs` | `a3f0b73394fb74ca2ea3302370cfbb0f8631fba0cdf045379e59ee8b084ff5ba` | Unchanged since actual R6; old consumer API |
| `🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧬️contract.json` | `58b478142b8fa7289054f752f2629e4c03072258e60b1631cfa7688fb38c6182` | Shared native declaration |
| `🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧬️schema.json` | `287402b49217843a4008c312a15cd69328c9724e81ea3afe21f92e59601322ae` | Shared declaration schema |
| `🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧪️fixture.json` | `300e03eef7dca81430ab65f84eb38c442bde0abce1f52e5b256d5326abd9e60a` | Actual R6 foreign-refill expectation |
| `🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧪️schema.json` | `214b5690beaf841cc53afc1893df25e82409ca5c4b158fb4726bde2b7f3975cd` | Same fixture schema |
| `🧰️framework/🔨️modules/🌱️value/💾️resident/📜️script.ts` | `db33d720554121d98107ac6ad3a4b57fbdfa11243778b6cc6af407d7b0537ac9` | Actual passing neutral gate; not native |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` | `2ad816977def25ded3175c87c0f7d03344f1bae57549689b17388adf871736ca` | No new RuntimeAppCell/Opening edits in this packet |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️lifetime/🦀️component.rs` | `5cc2eba5bc406eb3d6d232fc7e948f9f21be316e7099dc98c23eca959ff37046` | Read only in this packet |

Resident package metadata remains taxonomy-owned and unchanged by this lane: Cargo `c73b7b90a2efe859270f797c4ecfebd3457472e06462e991bdadb11fb0d750de`, project `9669b870a0f0e95a8466955cc76f1fec629bacf98928fa8430831cd5137ddbd8`, script `4b977ef3f6fbe7d04fbdf90bcd186346b79f81788b0d529ea5501fc06b95bbb9`.

## Last Executed Evidence

- Resident R4: actual12PASS/0FAIL/0skip,.027s,Nx0, on the earlier access-repair source—not this private-slot candidate. Passing stdout was not captured.
- Resident R5: actual wasm32-wasip2 .84s and wasm32-unknown-unknown .97s compile checks, Nx0, on that same earlier source; no Wasm runtime claim.
- Resident R6: actual13run/12PASS/1FAIL/0skip,.050s,Nx1. Exact debug `accepted=true consumerDropsDuringRelease=1 originalRootTerminal=true`; unchanged expected0 assertion failed after cleanup, without secondary abort. The complete report `📓️resident-foreign-consumer-r6-native-red-2026-08-28.md` was read. Its source was `6a24e42d16e99d66279d255817b4e148a62da9c5fa59eb718e799dd3b864efdc`, not current `ad1674b3…`.
- Neutral TS: actual `bun x nx run @semio-tech/value-resident:test` exit0. Strict Ajv and Immer sealed-replacement oracle output is preserved in `📓️native-resident-foreign-consumer-neutral-r1-2026-08-28.md`; this does not execute native scheduling or Drop.
- Last changed Plugin checkpoint boundary remains the earlier R6 full tail1PASS/523unselected/.057s with actual `Ok((1,1,1,1,7)),closed=true,close_fault=None`, and R7 changed roster1PASS/523unselected/.014s. No new Plugin or Opening result is claimed.

## WGPU Dependency And Hold

A read-only `rg` census of Cargo manifests and Rust sources found the resident package name only in the workspace path-dependency declaration and the resident package's own Cargo name/lib name. It found no Plugin, WGPU, UI or common-Kernel import/dependency on native resident. The workspace membership/path declaration alone does not establish a compiled dependency edge; the sole executor must use the actual selected WGPU target/dependency capture. Retained independently reports no resident funding adoption in its input primitives.

I will hold resident source at the above incomplete boundary and all existing Plugin/host/common-Kernel/Actor/lifetime source in my ownership through the requested short WGPU capture/compile. If the selected graph genuinely compiles native resident, do not call this a coherent green candidate: its test/API join is incomplete and its library is uncompiled. If resident is not selected, this lane has no in-flight Plugin/Opening change that needs to delay that scoped WGPU test. No rollback, deletion, rebuild, source substitute or passing-result inference is authorized by this hold.

