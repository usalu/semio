# History Held Constant-Storage Retirement Audit

Read-only source audit, no execution. Candidate is `📥️inputs/📜️history-output/📋️projection-retirement-pair.json`. Active History native:79 still calls generic FromValue::retire_decoded; no candidate runtime success is established.

## Ownership Invariant

The candidate has two live intrinsic owner roots, current and parent. Popping a child from a nonempty array/object leaves one guaranteed reusable capacity slot. Pushing the former parent into that slot cannot grow the Vec. Object links use String::new with zero owned backing; the original popped key is dropped once. After a leaf dies, backtracking pops the parent link before revisiting remaining children. Nonempty containers always move into the parent chain; only emptied containers or scalar/string/byte leaves reach ordinary drop. Ordered occurrences, duplicate names and NaN words require no comparisons or normalization during destruction. Empty containers with retained capacity still release their buffers.

Every granted transition consumes one loop unit; zero-item close_step changes nothing. Mandatory Drop drains remaining ownership in256-transition batches. It does not consult canceled provider control, which avoids refusing destruction or leaking after cancellation. This is bounded work per close_step, not an asynchronous/interruption guarantee for the synchronous draining Drop. Private state makes the expect/unreachable parent invariants unreachable through ordinary callers; this source review supplies no unwinding runtime proof.

## Actual Fourteenth Law

History native:281–340 mounts the genuine temporary-retirement law. It retains all nine individual kinds, UInt maximum/Int minimum, raw NaN word, NUL/unicode String, raw bytes, empty arrays/objects, full existing neutral intrinsic corpus, depth256 array/object owners, and long ordered duplicate branches. It creates a canceled NativeEncodeControl before retiring the owner. Cleanup itself has no cancellation callback dependency.

The borrowed capacity census:321–333 occurs outside observation and includes String/Bytes capacity, array slots, object pair slots and every object key capacity. Source census work is dropped before observing. Actual observe_backing:335 measures requests and released Layout bytes, then assertions:337–338 require zero requests and exact complete release. A leak/forget cannot satisfy the second assertion. Deep specimen construction is iterative; neutral literal parsing is outside the measured interval. This law does not exercise zero/one-item cursor interruption and mandatory Drop halfway through a chain; meaningful follow-up is to observe complete release after each partial close_step cutoff, including zero, rather than weakening the full-law assertions.

Actual shared observer `🧰️framework/🔨️modules/⏱️trace/🧮️memory/🧪️testing/📥️requests/🦀️.rs`:47 records realloc requested size and releases old Layout only on successful replacement;48 records dealloc Layout;52–54 retains generic observe's original (result,requested) API;58–66 supplies observe_backing over the same inclusive scope stack. No second allocator or guessed representation estimate is used.

## Concrete Integration Qualification

The capsule adds retire_record, but current actual Pack producer native:238 still calls drop(record) on EncodedRecord. Replacing retire_projection alone can establish its direct cleanup law while leaving generic EncodedRecord guard destruction and the measured Pack output request tail intact. The real owning guard must transfer its record through its declared take/ownership API to the private retirement routine; merely adding an unused helper is insufficient. All other early error/cancellation exits need the same guard-authority qualification. This is a current source integration gap, not a failure of the parent-link algorithm.

Shared schema fixed-capacity HashMap estimates remain separate from exact release proof. Hasher/map request ledger repair and candidate cleanup cannot be certified by subtracting prior tail bytes or by zero-request destruction alone. Root's authentic unchanged fourteen-law run must establish the baseline and later actual producer result; current graph failure did not run assertions.
