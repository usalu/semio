# Original Input and Native Custody Review

Read-only source review of the current original-input/original-worker guards, their neutral hostile fixtures, publication/lifecycle integration, and the selected Layout typing metadata law. No tests, compiler, build, or peer processes were run. Source was read after the input guard migration and while the separate BEFORE/CONTROL typing policy work remained explicitly in progress.

## Concrete Gap

`original-worker/🟦️.ts`, `singleTurn`: the no-loop expression rejects `while` and `loop` only. A Rust `for _ in 0..2 { ... }` around the existing pump retains one textual `session.pump_one_for_interactive_turn(same)` occurrence and one `pump_one` occurrence, so every existing singleTurn condition still accepts it despite two dynamic worker turns. The hostile fixture exercises only `while true{}`. Include `for` in the prohibited loop syntax and add an owner-scoped hostile for-loop witness around the actual pump or its funded callback. This is a source-law false positive, not evidence that production currently contains such a loop.

## Supported Guard Evidence

The source reader now includes all six families, including `🎮️command`. Lifecycle delegates directly to the new mandatory input facts; retained dispatch setup first requires the new worker facts. No obsolete alternate acceptance branch was found in these integration points.

Both guard helpers refuse missing or duplicate named owners. Input producer/validator/wire facts are scoped to the unique OriginalTypedCommandSource implementation; action, manifest, host, intent, worker setup, producer and worker turn facts are scoped to their named function bodies. Hostile tests require the selected owner and mutation needle to exist, mutate within that body, and require the named fact and complete strict schema to reject. Ajv and JSON Patch provide an independent evidence-table/schema oracle, not an independent Rust semantic interpreter. The reported 51 tests/341 assertions were not rerun here.

Current positive constraints retain the registry/classifier-before-producer order, manifest ownership/mode checks, genuine original parent loan, source binding and decoder/caller validation, qualified catalog and command admission, reserved queue/root capture, original wire/request/metadata transfer, retained candidate handoff, funded original work loan and one textual pump. Text recognition alone does not establish arbitrary control-flow semantics; the for-loop gap above is the concrete counterexample identified in this review.

## Actual Typing Metadata Law

Selected transform companion `original_typing_dispatch_retains_metadata_until_atomic_finish_or_cancel`, lines589–610, constructs the actual EditorApp/LayoutPlayApp dispatcher and a genuine OriginalFixtureScope, with a real empty TypingLedger. It invokes admit, capture_clock, admit_parent, complete_empty_ledger and finish_into rather than setting phases. It checks zero-grant admission/clock/finish invariance, absence of externally available metadata while pending, identical actor allocation pointer internally and after normal handoff, replacement-output refusal without progress or custody loss, and paid cancellation retirement under the genuine cancelled parent. Normal completion restores metadata together with ready/suppression; cancellation leaves metadata absent and drains terminal custody. Delivered metadata and replacement are themselves controlled-retired before fixture retirement.

The implementation pins ledger address, OriginalSourceBinding and native receiving identity. Final metadata/ready/suppression writes occur together inside the admitted final close receipt after retained fields and caller loan have closed. The test exercises empty-ledger metadata custody, not nonempty typing-row publication, decoder/foreign-ledger refusal, reactor integration, or complete BEFORE/CONTROL routing policy. Policy work was confirmed in progress by its owner, so intermediate policy differences are not reported as defects. The native law has only been source-reviewed here; actual native execution and actual-record portable comparison remain the required runtime evidence.

## Follow-up: Loop Fix and Iterator Wrappers

Fresh source read confirms the defining worker guard now rejects `for|while|loop`. Root reports reproducing the former for-loop false pass with a neutral mutation before applying this correction; the rerun result is not independently observed here.

The correction does not close callback repetition. The actual production pump in retained-command/full-operation is the direct expression body of `|same|` passed to `cx.with_original_work_loan`; that loan is the scrutinee of a match inside a once-invoked local closure inside the original_lifecycle_step callback. Wrapping that match in `(0..2).map(|_| { MATCH }).last().unwrap()` introduces no forbidden loop keyword, preserves every single counted call spelling and both required funding substrings, and repeats the loan/pump. A for_each wrapper with an output Option has the same weakness. Production has no such wrapper in the inspected body.

For a structural current-call-path fence, locate the unique pump invocation in Rust tokens, require it to be the direct expression body of the loan callback, require that unique loan to be a direct match scrutinee, and constrain its enclosing callable scopes to the original_lifecycle_step callback and the existing immediately invoked `let step=(|| ... )()?` shell. Additional enclosing callback scopes must refuse. Checking only textual occurrence counts or a finite iterator method denylist cannot establish one dynamic invocation: custom repeat(callback) has the same issue. If a lexical restriction is retained, add explicit owner-scoped for_each and map/last hostile fixtures and state that its guarantee is syntactic.

No source edits or test executions were performed for this follow-up. The empty-ledger typing custody law assessment and its unexecuted runtime limits above remain unchanged; separate policy refinement is still owned by source_lifetime.

## Structural Fence Review

Fresh read of original-worker/scope confirms a defining delimiter-ancestry fence rather than a dispatcher denylist. It requires the exact direct loan callback pump and match, original lifecycle callback and immediately invoked result closure, with the actual closing invocation and return tail. Added for_each, map/last and custom-repeat fixtures wrap the actual complete pump match extracted with toolJobRustBlock; missing target/body refuses before mutation assertions. These fixtures therefore exercise the real owner and preserve its single textual pump while testing dynamic repetition. Root reports 21 tests/147 assertions passing; this reviewer did not execute them.

One remaining concrete recursion false positive exists in scope helper line10: only `self.drive_worker_step(` is rejected. The valid Rust UFCS spelling `Self::drive_worker_step(self,cx)?;` can be inserted before the existing session mutable borrow (before `let grant=cx.retained_grant()`), leaving the exact pump ancestry, tail and all counted spellings unchanged. It recursively re-enters the owner but is not rejected by the current check. Add owner-local invocation recognition covering method and UFCS forms plus a hostile UFCS witness. No such recursion is present in the inspected production body.

## Final Fixed Status

Fresh final source read confirms the scope helper now rejects any `drive_worker_step` identifier in the string-masked actual function body, replacing the narrower method-call check. Both owner-scoped neutral cases are present: worker-recursive-ufcs inserts the direct Self call before grant capture, and worker-recursive-reference first captures Self::drive_worker_step into an alias and calls that alias. Their target exists before the session borrow. Both are rejected by the defining identifier check while the existing structural ancestry fence remains intact.

The reported RED counterexamples for loop, iterator/custom callback and recursion have corresponding defining fixes and hostile witnesses in current source. No further concrete hole was identified in this final bounded review. This is a syntactic current-source contract, not a general Rust execution proof or an interprocedural semantic interpreter. The combined rerun was active when this final read occurred; no result is claimed here. Actual native typing/metadata execution remains unverified by this reviewer. No production changes or tests were made by this review.
