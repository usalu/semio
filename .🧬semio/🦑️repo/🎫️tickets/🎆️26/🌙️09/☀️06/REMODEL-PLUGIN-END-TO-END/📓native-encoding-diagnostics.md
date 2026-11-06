# Native encoding retirement compiler diagnostics

Forwarded control lifetime note: its constructor accepts `&'a mut NativeEncodeAllocationLoan<'a>`. The new pending test explicitly drops the control and then the loan; an existing unwind test drops the control without explicitly moving the loan. A borrow-check failure is plausible but not established without the live compiler result.

Tracked non-test-path legacy close-call candidates include Energy snapshot bridge, stdio native print, OS MCP workspace JSON, store space-history SQLite validation and composition opening, SPR history, plugin snapshot authority and interpreter input. Some paths contain decode calls or inline tests, so each call's actual control type must be resolved before migration; this spelling census does not claim all are encode callers. Testing support paths and ticket files are excluded.

The captured paged-selection-header retry log reports 31 native-encoding errors before the page test body. These are historical compiler locations, not proof the current tree is unchanged. Paths below are under `🧰️framework/🔨️modules/🌱️value/🛫️encode`.

- Root `🦀️.rs`: lines 39–40 `NativeEncodeOperationFailure` fixed error-field sequence; 61–62 scoped continuation scalar leaf; 69 continuation cursor missing actual-marker parameter; 76 continuation associated retirement; 114 close call missing actual marker.
- `🛂️allocation/🦀️.rs`: line 52 allocation outcome fixed six-error sequence; 76–78 pending allocation progress is a conditional unresolved owner and its cursor needs actual-marker signature; 124–126 forwarded receipt wraps continuation.
- `🎟️effect/🦀️.rs`: lines 38–40 effect state fixed allocation/outcome/settlement fields, and 52–53 effect failure fixed error fields.
- `🎟️effect/🧭️settlement/🦀️.rs`: lines 23–24 and 28–29 receipt/cancellation scalar leaves; 64–71 pending settlement conditionally retains original callbacks and needs typed conditional ownership plus actual-marker close.
- `🫴️recipient/🦀️.rs`: lines 21–22 erased retirement close signature and underlying call lack the real marker.

Compiler classes were 11 missing associated `Retirement` items, four obsolete close signatures, two missing call arguments, and 14 typed-versus-boxed mismatches. Fixed-field owners can use actual associated typed cursors; pending allocation/settlement must retain original callback custody and refuse until recovered. No box-based compatibility rewrite or native-pass claim follows from these diagnostics.

Separately, the current PagedList has a three-word Vec and five usize fields. Both actual-height and exact-final-extent fixtures and native assertions were updated to eight owner words; no native test was run for that change.

A nine-row ticket neutral fixture/schema and lodash/Ajv model oracle passed under managed Bun. It models field preservation and pending-versus-settled expectations, but cannot observe real cursor heap, marker or callback custody; Rust laws must establish those effects.

Corrected neutral model: actual continuation has seven scalar fields, pending ports cause pause refusal rather than an invented pending continuation, and allocation outcome retains its complete nine-field projection. Managed Bun model passed nine cases; no native proof.

Typed wrapper audit: Ready fails to forward several optional quote/capability methods, and Pending exposes only a mutable borrow with no consuming extraction or settled transition, so recovery cannot lead to paid closure through the wrapper. Existing allocation unwind tests show a real public issuer/forwarded-control pending callback path. Owner notified; source-only.

Typed-law review: pending scenario uses original issuer and callback result, but creates fresh stock for close and a separate return grant, so it does not prove a single finite end-to-end bank; its refusal match also drops the owned refusal. Encoding recipient close still precharges quoted capacity. Correct migration needs a caller-owned persistent effect frame and the actual child marker, preserving typed OriginalRetirementFailure and callback settlement through interruption.

Recipient delegation: quote work 1 for reserved slot or terminal boxed ticket, 0 for no owner, otherwise forward selected child. Requires-original applies to nonterminal selected child. Controlled and original closure should use the matching close_factory_ticket helper with the same control and actual marker; helper precharge remains a separate known issue. Existing encoding allocation test installs genuine ControlledRetirement<String> via with_retirement_owner; action test owns a real pending settlement and issuer pair for original-control behavior.

Typed-law coverage snapshot: four fixture IDs are directly read; allocation settled is exercised but its row is not read, and both settlement pending rows, settled settlement and refused pause lack direct row-driven laws. Existing action settlement and visitor tests expose genuine issuer/recovery methods; their synthetic Some(Ok(())) is not proof of an actual captured callback result.

Native typed-retirement retry log reports 4 passed, 0 failed, 0 ignored, 0 filtered: continuation, failed cursor birth, pending allocation recovery, and failure/allocation cursor closure. Current pending test source debits refusal retirement, funded return, and settled output closure from one bank/fuel. This evidence covers those four tests, not all nine fixture rows or whole encoding close-frame behavior; exact compiler source capture was not independently established here.

Proposed recipient law: install a genuine occupied ControlledRetirement owner via existing allocation test with_retirement_owner; assert selected child work quote and zero-grant marker preservation, then use real original-control test fixture with the same construction/work issuers to prove foreign-recipient refusal retains the actual response and matching loan forwards it. Assert terminal ticket release work separately. Preserve original child refusal and actual allocation marker.

New settlement laws use same real issuer loans and stored callback errors, but the original work/physical issuers issue fresh fixed allowances and are not debited from the cleanup bank; declared one-byte progress is not tied to an observed physical mutation. They currently establish callback custody, not end-to-end finite-stock or actual physical effect proof. Native run pending.

Corrected settlement law now writes an actual inline byte and issuer allowance is finite, with its posted work and submitted receipt debited from cleanup bank after loans. Production tracked call chain is io snapshot settlement wrapper (begin/record) and OS space-history SQLite native output handoff/writer; the latter uses writer.last_turn_work, which can differ from one. A truthful settlement API must retain selected quote before effect and actual work plus progress through interrupted callbacks, posting actual once.

Explicit work API call sites: io control snapshot settlement wrapper and OS store space-history SQLite native writer (handoff and writer turns). Tracked tests additionally occur in encoding action/effect, pack JSON borrowed-source laws, and io control receipt tests; new untracked encoding typed-retirement laws also call the API.

JsonBorrowedWriteCursor exposes physical normal_step_demands_with_copy and post-turn last_turn_work, but no pre-effect selected-work quote. Its max_units=1 increments an iteration count, while demand traverses source/path and measures text. A source-free funded selection stage or explicit truthful bounded quote is required before settlement admission; fixed output handoff work is a separate event.

Explicit-work API census: encoding action tests lines 64,72,83,92,101,117; effect test 45; pack JSON borrowed-source law 328; IO control receipt 175; untracked typed-retirement law also calls both. New settlement stores work quote and actual and commits recorded actual across interrupted phases. Source-only review found no additional concrete defect; writer quote remains unresolved.

Current explicit-work ports forward selected and actual values; typed law reads fixture selected/actual and checks the issuer quote, actual post and retained value across interruption. No new concrete arity defect found in bounded source review. SQLite writer selected one remains an iteration bound without semantic source-work proof.

Snapshot settlement typed-retirement audit: original, local_failure and operation_unwind are owned fields; pending owner must retain the latter or unresolved original. Ready branch can take first two then drop state under current guard. Existing IO receipt law covers unwind/recovery but not typed cursor birth/close. No native proof.

Narrow IO settlement integration owner: io/control/📦packages/🦀rust/Cargo.toml already has an integration target and trace-memory-testing, value and job testkit dependencies; a separate [[test]] avoids legacy receipt corpus. Public NativeSnapshotEncodeOwner/Settlement plus NativeEncode issuer/typed retirement APIs support same-original recovery and observed close. Existing receipt test 175 is workflow precedent only; its legacy close helper needs no copying.

Dedicated IO integration fixture paths resolve. It models quote 7/actual 3 and retains a real unwind and original post result, but the source explicitly mutates only one output byte while recording three work units; this is numeric propagation, not actual producer-work proof. Its issuer and cleanup bank are reconciled after loans, not a single pre-admitted shared wallet under tight budgets. Native run pending.

Correction after current-source reread: encoding and IO laws now reserve physical item1/copy3 and work7 before issuer loans, perform three byte writes, debit pending cleanup from remaining bank, and refund only unspent issuer balances after loans. Prior one-byte/shared-wallet limitation no longer describes current source. Native validation remains pending.
