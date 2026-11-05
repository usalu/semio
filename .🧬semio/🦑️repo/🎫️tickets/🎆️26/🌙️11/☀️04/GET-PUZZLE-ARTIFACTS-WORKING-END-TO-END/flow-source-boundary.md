# Flow Session Source Boundary

The reported `maximumBytes: 4800` is the remaining byte allowance for the crate closure, not an independent per-file cap.

## Governing Contract

`🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/🟦️.ts` lines 27–28 takes the minimum of global policy and owning unit metadata for byte, file-count and active-line limits. For each unique physical source input it computes `used = sum(seen.values())`, then passes `maximumBytes = limits.maxSourceBytes - used + (previous ?? 0)` and `remainingFiles = limits.maxSourceFiles - seen.size`. After reading it sums bytes again and rejects aggregate overages. The final closure also checks total source files and active lines.

`📖️read/🟦️.ts` checks the actual file extent against that remaining allowance before allocating compiler-input bytes. Moving the same body into another ordinary Rust module does not reset the allowance. Every reached module remains in the same closure; its bytes and additional module declarations still count.

## Owning Unit

`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🎞️session/📦️packages/🦀️rust/Cargo.toml` owns `semio-framework-os-flow-host-session` with maxSourceBytes 262144, maxSourceFiles 32 and maxActiveLines 4000. Its source entry explicitly includes `../📤️publication/🎟️session/🧵️work/🦀️.rs`. The refusal observed 6506 bytes for that file with only 4800 bytes remaining and 9 file slots remaining. The overage is 1706 bytes at this read frontier.

## Extraction Assessment

A plain colocated split of baseline source methods or testkit methods cannot reduce the aggregate compiled closure, so it cannot honestly resolve this refusal. The existing cfg-gated testkit method block is only 793 UTF-8 bytes; even excluding it from a configuration that does not compile testkit would not cover the observed 1706-byte deficit, before counting the new module declaration/import overhead. Active testkit units must still admit those bytes.

A valid fix requires reducing the owning crate’s actual source extent or transferring a coherent implementation responsibility to a genuinely separate dependency crate with its own owned boundary and APIs. That is a crate-ownership change, not a nonsemantic file split; it must preserve the original Flow ownership/grant semantics and be coordinated with the current Flow work. No source was edited and no boundary, file observation, admission or receipt was weakened. No Cargo command was started while integration owned WASM compilation.

## Genuine Responsibility Transfer Assessment

Read-only inspection identifies a smaller coherent transfer than the session baseline target methods: `SessionLivePublication`, currently the complete 2913-byte `📤️publication/🎟️session/🦀️.rs` leaf. This cursor has no `FlowEvalSession`, session state, geometry, registry or peer dependency. It owns a source pointer/length identity, funded UTF-8 copy frontier, String ownership and a paid shared-header birth. Its inputs and outputs are strings, grants and Arc<String>. That is the same source/publication responsibility already owned by the source crate’s FlowWorkPublication, display and paint cursors.

### Minimal Concrete Plan if the Actual Producer Still Refuses

1. Transfer the whole cursor body into a colocated publication live-text module reached only by `semio-framework-os-flow-host-source`. Export the owned cursor and the same operational methods from that source owner. Keep all fields private. The source publication module already owns an identical controlled `FlowPublicationShared<T>` wrapper, so the moved cursor can use that existing local wrapper without depending back on host-core or session. Both wrappers use the same SharedControlledRetirement lease, same shared-retirement birth accounting and same RetireOwned support requirement.
2. Replace the session crate’s physical module mount with an import of the cursor from its already declared source dependency. Session state retains exactly one pending cursor and the existing LivePublication retirement variant. Existing tick intake/hand-off logic continues to call the same constructor, demand, step, published/ready, take, restore and mark methods. Do not alter any baseline-source method, registry generation check, identity comparison, retirement sequence, peer logic or FlowPublicationTarget implementation.
3. Source gains one real admitted module (~3 KB); session loses the 2913-byte module with only a short imported owned type in its entry. Expected honest net session reduction is approximately 2.8 KB, exceeding the old 1706-byte deficit. No Cargo manifest dependency change is needed: session already depends on source. No macro/string expansion, cfg omission or admission changes are needed. The source compiler owns the entire moved implementation and its trait derivation.
4. Preserve the neutral funded-cursor law: source swap refusal, exact UTF-8 bytes, zero grants, admitted shared-header birth, take/restore identity and controlled retirement. Verify existing session tick publication laws plus a focused cursor fixture law and the actual source-admission counters. Native/WASM compilation must demonstrate both crate closures remain below their existing byte/file/active-line limits.

This is a real ownership transfer of a generic funded text publication cursor into the existing source/publication authority, not a split inside the same admitted crate. Moving baseline methods alone would instead require a new narrow mutable-state port and careful retirement-queue ownership; it is larger and risks changing the session’s private authority boundary.

### Current Live Closure Measurements

The repository’s own readRustSourceClosureV1 was invoked read-only through registered repo/Bun ingress, using actual physical file reads, the exact crate entry and manifest owner. No compiler or Cargo process was started. Current counts are:

| Owner | Cfg | Inputs | UTF-8 Bytes | Active Lines |
|---|---|---:|---:|---:|
| evaluation-source | ordinary | 10 | 120007 | 788 |
| evaluation-source | feature=testkit | 10 | 120007 | 795 |
| evaluation-source | test | 17 | 147736 | 875 |
| session | ordinary | 23 | 255260 | 2883 |
| session | feature=testkit | 23 | 255260 | 2892 |
| session | test | 23 | 255260 | 2891 |

The source target has substantial headroom for this transfer under its existing 262144-byte/32-file/4000-line limits. Crucially, the concurrently edited session closure now measures 255260 bytes, which is already below the aggregate limit by 6884 bytes. Thus the earlier 6506-versus-4800 refusal is stale relative to the inspected live closure. No implementation edit is warranted until the actual WASM producer confirms whether its exact current cfg/feature closure still refuses. These read-only measurements guide the plan; actual owned native/WASM admission remains authoritative.

## Failed Caller Configuration Cross-Check

The failed caller was the `semio-hub-puzzle` native library-test command with its explicit hub manifest and prototype profile. `semio-framework-os-flow-host-session` is a dependency library in that command, not the library-test root; its own `cfg(test)` is normally disabled. Its owning Cargo manifest declares only the `testkit` feature and no default features. The exact historical compiler-cfg snapshot was not retained, so this report does not invent one. Instead the current physical closure was checked with every cfg condition that can affect this owner.

All cfg/cfg_attr expressions found in its complete currently reached physical source closure are exactly `#[cfg(any(test,feature="testkit"))]` and `#[cfg(feature="testkit")]`. There are no platform-selected source branches in these inputs. The repository walker was rerun using native macOS/aarch64/unix cfgs, with and without testkit; wasm32/WASI-p2 cfgs, with and without testkit; native test cfg; and native test plus testkit cfg. Every case reads the same 23 physical inputs totaling255260 bytes. Active lines are2883 ordinary,2892 testkit,2891 test, and2892 test+testkit. All satisfy the owner’s unchanged262144-byte/32-file/4000-line caps.

Therefore a fresh normal session dependency closure is now within its existing cap (6884 byte headroom), including the feature combination that the failed caller could have enabled. The clean responsibility-transfer plan stays evidence only; no implementation change is needed based on the current closure. The actual producer’s native/WASM admission remains the authority and may reveal subsequent concurrent edits or a different failing owner. No Cargo/compiler operation was run during this cross-check.
