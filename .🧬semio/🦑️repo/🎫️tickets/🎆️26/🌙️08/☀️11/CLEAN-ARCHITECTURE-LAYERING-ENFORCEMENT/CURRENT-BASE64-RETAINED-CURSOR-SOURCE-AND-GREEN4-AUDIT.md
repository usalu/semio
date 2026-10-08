# Retained Base64 Cursor Source and Green4 Audit

Read-only audit of the actual defining implementation and closed receipts. No production changes or reruns.

## Source findings

Authority: `🧰️framework/🔨️modules/🚪️io/🔤️base64/🎮️cursor/🦀️.rs`, canonical collector in Base64 root `🦀️.rs`, and mounted `🧪️tests/🔬️unit/🦀️.rs`.

The constructor examines length and terminal padding only, allocates no payload storage, and moves OwnedText/OwnedBytes or retains the original Borrowed pointer. Private source custody prevents mutation between strict validation and reconstruction. InvalidLength is recorded immediately; a zero grant returns that already frozen refusal without additional work. That differs from a fresh valid cursor, where zero/undersized grants return Pending without callback, progress change or allocation.

Each advancing byte operation covers at most 4096 source bytes. Nonfinal Validate charges its padding precheck plus quartet scan, so maximum demand is 8192. Phase starts and transitions cost one unit. The quartet stack value avoids a temporary decoded chunk. Decode phase admission/checkpoint precedes the sole exact output reservation. No recursive materializer or unbounded source scan is hidden in a one-unit transition. Allocation itself is not a byte-work or wall-clock bound: a caller can admit a large output allowance, and the Decode-start transition reserves that allowance in one operation.

Validate and Decode progress callbacks retain phase-zero and end-of-chunk ordering through the canonical synchronous collector. The extra phase transitions do not introduce callbacks. Absolute quartet offsets remain passed into the codec. Nonfinal padding scan still precedes individual quartet errors. Empty input takes finite phase transitions, produces an empty vector and no backing allocation. Terminal output can be moved once; cancellation or other refusal leaves it inaccessible until into_parts transfers the partial allocation. Refusal is frozen before later changed limits/callbacks. into_parts moves original input, partial output, refusal and completion state without cleanup or clone claims. The synchronous collector intentionally drops retained partial custody on its error return.

No semantic regression found in these inspected branches. Existing control admission is checked more frequently by the cursor, including pure transitions; if the caller mutates its output maximum between turns, a later smaller allowance can refuse before a phase callback. The public control is caller-owned, so this is visible behavior worth keeping explicit.

## Mounted evidence and limits

The exact-demand law now checks callback Cell inertia as well as unchanged progress and zero requested bytes for every undersized neutral grant. It checks OwnedText original pointer, terminal-only/single take, third-party base64 bytes and a Borrowed original pointer. The custody law checks OwnedBytes pointer, Decode4096 cancellation, requested output8193, released0, retained prefix3072, capacity8193, frozen Cancelled after maximum becomes0, and allocation/release-free into_parts.

The laws do not yet directly exercise public cancel() before/after completion, malformed retained into_parts, or a mutable-limit refusal after partial reconstruction. Existing synchronous controlled laws cover malformed/limit/callback cases, but those are not direct retained custody witnesses. These are finite followup vectors, not evidence of a discovered implementation failure. TS has no retained cursor twin; source green proves existing TS controlled behavior only.

## Actual receipt join

`🗑️generated/general-base64-native/native-4/terminal.json`: code0, actual Rust14 passed, zero failed/ignored/filtered. This comprises original7, controlled3, retained2 and observer2. Native DEBUG output confirms retained input, terminal output and partial-custody assertions ran. Observer intentional caught panic output accompanies passing observer laws.

`source-4/terminal.json`: code0, Bun5 passed,703 expectations. `strict-4/terminal.json`: code0 and diagnostics[]. All three receipts contain21 selected bodies plus producer. Independently recomputed all22 UTF-8 SHA256 hashes and compared every body against current physical source: exact, no mismatch. Receipt exactSelectedSources and exactProducer are true.

Native3 remains the recorded pretest Cargo101 missing-interface red; its body is historical and differs from current cursor/root/tests/producer. It is not a law-execution failure. No workspace-wide or cross-language retained-cursor acceptance is inferred.
