# 1107 Audit: Frame Size of NativeReceivingOriginal<DslValue, NamedPersistentRoot>

Lane: frame (read-only audit, no build, no cargo, no edits except this report).

## Question and short answer

Why is the owner about 19 KB, where does it live, and what should shrink it.

Short answer: the owner is inline data, not heap. Its big inline members are the child action state (`NativeReceivingChild`, with its effect and failure records), two `RetainedCloneSource`-class values (one for the original input, one inside the staged `NamedPersistentRoot` output), and a set of nested allocation and retirement records. Static reading accounts for roughly 8 to 10 KB; the remaining roughly 9 to 11 KB sits in types I could not fully expand without a size probe. Recommendation: box the child action state (`NativeReceivingChild`), which saves an estimated 5 to 7 KB of frame and is the only option that changes receipts in a bounded, predictable way.

## Size basis (inferred, not measured)

- Close quote 38,848 B (C/1107-result-valuetests.md, item for `NativeReceivingOriginal`). The valuetests file says the close quote is a copy-in plus copy-out of the cursor, so the owner is about 38,848 / 2 = 19,424 B. I did not confirm this by `size_of`, because a probe needs a build, which this lane forbids.
- The owner is held inline by `NativeReceivingOriginalSeed` (`🌱️value/🧳️owned/🧳️original/🦀️.rs:81`). The seed is the actual retirement cursor, so the heap block for each close is about 19.4 KB as well (`retirement_birth_bytes` for the seed is `size_of::<Seed>()`, see `🌱️value/♻️retirement/🦀️.rs:73` where `pointer.write(value.retirement_seed())` places it).

## Inline fields of the owner

Source: `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧳️original/🦀️.rs` line 13.

| Field | Type | Inline size (estimate) | Note |
|---|---|---|---|
| `original` | `ManuallyDrop<Option<OriginalInput<I,A>>>` | about 0.6 KB | `Source(RetainedCloneSource)`; the source holds `owner_close: RetainedOwnerGate<ControlledRetirement<..>>` inline (`🌱️value/🧬️retained-clone/🔗️source/🦀️.rs:51`) |
| `receiver` | `NativeOwnedStorage<T::Receiver>` | 16 B | pointer only; receiver lives on the heap (`🛬️decode/🧳️owned/📦️storage/🦀️.rs:4`) |
| `retired_header` | `Option<Box<T::Receiver>>` | 8 B | heap |
| `output` | `ManuallyDrop<Option<T>>` (`NamedPersistentRoot`) | about 0.7 KB | `source` field again carries a `RetainedCloneSource` inline (`📋️list/🔺️edit/🔑️named/📇️index/🌳️root/🦀️.rs:7`) |
| `retirement` | `RetainedCloneClose` | 16 B | `Box<dyn>` |
| `action` | `NativeReceivingChild<OriginalReceivingPrimary>` | about 5 to 7 KB | largest identifiable member, see below |
| `authority` | `ManuallyDrop<Option<OriginalEffectAuthority<NativeSlotProducer>>>` | under 0.1 KB | identity only |
| `closing` | `bool` | 1 B | |

Inside `action` (`🛬️decode/🧬️protocol/📦️packages/🦀️rust/🦀️.rs:13`):

- `effect: NativeReceivingEffect<ValueError>` (`🛬️decode/🏭️construction/📦️effect/🦀️.rs:21`): about 2.5 to 3.5 KB. It has roughly 25 inline fields, including about 8 `ManuallyDrop<Option<ValueError>>`-class refusals (each about 64 B, since `ValueError` is kind + `Cow<str>` + `RetainedCloneProgress`), `allocation: Option<NativeDecodeAllocationProgress>`, `outcome: Option<NativeDecodeAllocationOutcome>` (about 350 B), `performed`, `partial_work`, `work_claim`, `producer`, `producer_reply`, `child`.
- `failure: ManuallyDrop<Option<NativeReceivingEffectFailure>>` (`🛬️decode/📬️outcome/🛬️receiving/📦️effect/🦀️.rs:10`): about 2 KB. It embeds a full `OriginalReceivingPrimary` (`🛬️decode/📬️outcome/🛬️receiving/🦀️.rs:7`), which in turn holds `Option<NativeAllocationFailure>` (about 8 refusal fields, about 0.6 KB), `Option<OriginalRetirementFailure>` (about 0.4 KB), `Option<NativeDecodeAllocationOutcome>` (about 0.35 KB), and about 6 `Option<ValueError>`.
- Scalars (`demand`, `grant`, `work`, `ceiling`, `request`, `tag`): under 0.2 KB.

Unexplained remainder: the visible members sum to about 8 to 10 KB against about 19.4 KB. The gap is most likely in `NativeAllocationFailure`, `NativeAllocationTransaction` (`🎟️control/🧬️contract/🛂️allocation/🦀️.rs:44`), `OriginalWorkActionReservation` (inline `partial` and `quote_reply`), and the derive-generated `NativeReceive` outcome types. I did not expand these. The single fastest way to close the gap is a one-off `size_of` probe of each field in the owning crate (a test that prints `size_of::<FieldType>()` for each row above). That needs a build, so it belongs to the next lane that may build.

## Production stack exposure

Where a full owner value is moved on the stack:

1. `NativeReceivingOriginal::new` and `from_projection` return the owner by value (`🧳️original/🦀️.rs:20`, `🪞️projection/🦀️.rs:7`).
2. `retirement_seed(self)` takes it by value and wraps it in `ManuallyDrop::new(Some(self))` (`🧳️original/🦀️.rs:82`).
3. `admit_retirement_cursor` takes it out of `Option<T>` (`original.take()`) and then `pointer.write(value.retirement_seed())` (`♻️retirement/🦀️.rs:73`). That is at least two full 19 KB copies in release, more in debug.

Consequences by runtime:

- Wasm plugin guests: 8 MiB shadow stack (`.cargo/config.toml`, `[target.wasm32-wasip2]`). A few 19 KB copies are fine, and the owner is not nested on the stack across heap receivers.
- Browser wasm: 16 MiB (same file). Fine.
- Native pool workers: 64 MiB (`⏳️async/🖥️native-pool/🦀️.rs:244`). Fine.
- Default 2 MiB threads (cargo test, debug): this is where it overflows. Debug builds multiply each 19 KB move, and the ui-locale test needs 256 MB as a result (see C/1107-result-pixels.md item "ui-locale"). The 256 MB thread is masking the frame, not fixing it.

Production risk: low on the targets above, but the frame is a real hazard for any path that runs a debug build on a 2 MiB thread, and for any future owner nested in a call chain.

## Options

Estimates are for the owner after the change. Baseline is 19.4 KB, close quote 38.8 KB.

| Option | Owner inline after (est.) | Close quote after (est.) | Heap effect | Receipt and quote changes | Effort |
|---|---|---|---|---|---|
| 0. Keep | 19.4 KB | 38.8 KB | seed block 19.4 KB | none | none |
| 1. Box `action` (`Box<NativeReceivingChild>`) | 12 to 14 KB | 24 to 28 KB | +1 box of about 5 to 7 KB, one extra allocation | `demands`, `close_demands`, `close_work_demand`, `close_step`, `advance` all touch `self.action`; add an alloc phase at birth, a release phase at close, one allocation work unit each; heap total unchanged, allocation count +1 | medium, about 15 call sites in this file |
| 2. Box `NativeReceivingEffect` only | 16 to 17 KB | 32 to 34 KB | +1 box of about 2.5 to 3.5 KB | same kinds of change but smaller; leaves the failure record inline | small to medium |
| 3. Box the two `RetainedCloneSource` values | 17 to 18 KB | 35 to 36 KB | +2 boxes of about 0.6 KB each | pure heap churn for under 2 KB saved; not worth it | small |
| 4. Split state into phase cursors | a few KB (largest phase) | a few KB | one birth per phase | every phase gets its own birth quote and receipt row; the close chain changes shape; large rewrite across the seed and retirement protocol | large |
| 5. Iterative cursor | 19.4 KB (unchanged) | 38.8 KB (unchanged) | none | removes nested `close_step` frames; does not reduce the data frame, so it does not fix the 19 KB | medium, complementary only |

Effect on the cross-language vectors: the fixture funds a 4096 B body (`LAW_BODY`). Options 1 and 2 still exceed it (close quote 24 to 34 KB), so the 1<<16 body used by the runtime stays necessary. Only option 4, or a much smaller owner, brings the close under 4096 B.

## Recommendation

Box the child action state (`NativeReceivingChild<OriginalReceivingPrimary>`) as one fixed allocation created at `new` and `from_projection`, and released in the close chain. Reasons:

1. It is the largest identifiable inline member and it is isolated behind one field (`action`), so the change is local to this file.
2. It keeps heap-exact receipts simple: one extra fixed-size block, quoted as capacity at birth and as release at close, following the same pattern `NativeOwnedStorage` already uses for the receiver. Heap total is unchanged (the bytes move, they are not removed); the only receipt change is one more allocation work unit and one more release.
3. It shrinks every copy of the owner (birth, seed move, `retirement_seed`), which is where the stack and the 38.8 KB quote come from, by an estimated 35 to 40 percent.
4. It does not touch the receiver protocol or the retirement cursor contract.

Caveats: the estimate is not measured; the remaining 9 to 11 KB gap must be closed with a `size_of` probe before the new numbers are written into any quote. After the box, the 256 MB test thread can probably drop to a smaller bound, but I would not remove it until the probe confirms the new frame. Option 4 remains the only route to a frame under the 4096 B cross-language body, and it is a separate decision.

## Files read

- /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1107-result-valuetests.md
- /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1107-result-pixels.md
- /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1107-HANDOVER.md (lines 73, 76, 89, the "38 KB frame" debt)
- /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧳️original/🦀️.rs
- /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧬️protocol/📦️packages/🦀️rust/🦀️.rs
- /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/📦️effect/🦀️.rs
- /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🛬️decode/📬️outcome/🛬️receiving/🦀️.rs and its `📦️effect/🦀️.rs`
- /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/🦀️.rs
- /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs and `🎮️controlled/🦀️.rs`
- /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/📋️list/🔺️edit/🔑️named/📇️index/🌳️root/🦀️.rs
- /Users/ueli/Documents/semio/.cargo/config.toml (wasm stack sizes)
- /Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🖥️native-pool/🦀️.rs (worker stack)
