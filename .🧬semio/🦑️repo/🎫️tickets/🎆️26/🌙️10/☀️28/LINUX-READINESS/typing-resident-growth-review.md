# Typing Resident Growth Review

Current Source review confirms retained_read_pending is now in resident impl<M>, removing the earlier cold generic-bound mismatch. Both shared withdraw_original and install_resident_original now evaluate the supplied busy predicate before mutation; TypingLedger supplies original_write_pending, while other ledger predicates remain false. Macro arity/call-site types agree and no duplicate TypingLedger map_slots implementation remains.

## Concrete Visibility Concern

The new read::tests growth test directly accesses ledger.windows.capacity(). TypingLedger.windows is pub(crate), but its Vec element TypingWindow is pub(super) inside typing::resident, visible only within the typing module. The test is a sibling read module. This exposes a private element type at the capacity call and is a likely Rust private-type compile refusal. Align the intended internal visibility or place a test-only capacity observation inside the owner; no external type export or public compatibility API is necessary. The issue was sent to Root before actual test72636 settlement. No compiler result is claimed.

## Test Shape and Lifetime

The authored neutral fixture selects8197 u64 leaves,65 lexically earlier prefix windows and finite1048576turn limit. The test infers TypingLedger<u64> consistently from its explicit factory and leaf construction. It obtains actual issued retained-read ownership, retains the actual projection, grows/shifts resident storage through real cold sends and verifies unchanged resident key, net backing pointer, projected pointer and length. It separately proves backing capacity increased and lexicographic final reader window position. This covers both Vec allocation and prefix movement rather than only adding tail entries.

Projection borrow temporaries end within each assertion before the next mutable ledger operation. The shadowed mutable projection reference is closed through existing funded close_demand/close_step, then the actual retained-read owner is returned through the real ledger interface; factory Arc identity and final backing pointer are asserted. Existing paid helper compares actual heap allocations/releases with Original receipt values, using the genuine fixture caller. No new fake Original, uncontrolled read clone or Native boundary is introduced by this test.

This proof exercises cold prefix insertion while a retained reader exists; it does not prove the separate funded Original insertion path can progress for arbitrarily large resident sets under tiny grants. That existing capacity/copy protocol limitation remains distinct from the current type/lifetime review. Actual registered parent/edit tests still establish compilation and runtime behavior. No Source, tests, compiler, Cargo, Native, process or lease operation was changed or executed by this audit.
