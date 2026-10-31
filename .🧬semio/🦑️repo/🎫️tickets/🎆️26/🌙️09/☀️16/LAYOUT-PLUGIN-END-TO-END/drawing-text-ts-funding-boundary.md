# TypeScript Caller-Funded Scene Boundary

This is the final interface proposal for Root and Low review, not production implementation. The actual RED input/log/terminal receipt remain immutable. Shared controller.schema.json/controller.json now define a closed lifecycle and eight abstract neutral cases; they are separate from the eight captured RED inputs. No runtime or TypeScript production changed.

## Reused First-Party Accounting

Reuse job/budget/JavaScript remainingRetainedGrant and consumeRetainedProgress, with their own JobRetainedGrant/JobRetainedProgress declarations. The latter validates exact independent axes, records actual effects in the recipient before an overrun refusal, and carries the original actual progress on error. This is the required error-after-effects semantic. Existing RetainedWorkBudget.charge checks before recording and therefore is not substituted at this ingress. FontWorkLedger remains the actual same-ledger producer/child implementation; no external runtime dependency or duplicated budget algebra is needed.

No current TypeScript host StepContext was found. Exact tracked non-test references to the high-level Drawing functions identify only their definitions and internal vector/raster/placement/PNG calls. Their current outer callers are test interfaces. Do not claim a real existing browser scheduler already issues these grants. The replacement public contract makes finite caller funding mandatory; tests provide explicit finite fixture policy, while any future product caller must use its actual controller.

## Mandatory Own Interfaces

The own DrawingSceneWorkTurn contract has check(), remainingFuel(), retainedGrant(), consumeRetained(actual), consumeFuel(actual), and shouldYield(). Grant and receipt are existing first-party records. CPU fuel is independent of retained items and byte axes. The turn is borrowed only inside its owning controller callback and bound to that controller, operation and original grant. It cannot be stored/reentered/reused after callback exit.

The mandatory DrawingSceneWorkController owns withTurn(mode, demand, callback) and waitForTurn(mode, demand). Mode is advance or close. Demand is scheduling DATA, never authority or an instruction to increase a grant. The controller supplies only remaining credits from its independently configured original finite caller policy. It validates closed shape, nonnegative safe integers, current lifetime, controller/operation binding and no reentry before invoking the callback or taking mutable ownership. A blocked controller does not invoke the producer and retains its owner.

withTurn returns either ran with callback result, or blocked without effects. A blocked producer/controller turn must leave to the caller; an asynchronous high-level loop awaits waitForTurn before another grant. It does not spin, reset a budget, derive bytes from maxWork, or silently create a new controller. waitForTurn belongs to the caller scheduler, supports cancellation/progress and supplies no credits by itself.

A retained owner remains with the high-level operation while awaiting work. A permanently refused/expired close must return a first-party failure carrying the still-owned close cursor; it must not drop ownership or pretend close succeeded. No escaped completed output is published before controller recheck and successful owned close. Semantic cancellation suppresses advance/publication; only the caller's explicit close-mode authority can drain custody afterward.

## Exact Boundary Sequence

1. Validate source/options and the live caller/controller before ownership mutation or advancing a child.
2. Inside one admitted turn, borrow its freshly remaining retained grant once and construct one FontWorkLedger. Use remainingFuel for the separate item quantum.
3. Pass that SAME ledger through parent and all font/vector/raster descendants. The active phase quotes its own or nested pending demand, including held copies/depth.
4. Retain the producer result/error. Read the ledger cumulative receipt and lastTurnWork once at the outer boundary.
5. Fold actual retained effects exactly once using consumeRetainedProgress semantics, including on child error. Fold actual CPU fuel exactly once separately. If accounting itself refuses, preserve both actual effects and original child failure; do not retry the same receipt or replace custody.
6. Recheck the current turn/controller. Return pending on zero-effect blocking; distinguish paid reservation progress. Never replenish inside a child loop.
7. Leave the turn, then yield or request a new caller-owned turn. Close follows the same funding/accounting sequence and preserves real release-axis denial.

If one ledger serves multiple child calls, fold its cumulative receipt only once at that turn boundary. If a later call creates a new ledger, it must use the controller's newly remaining grant after prior consumption. The producer never uses the original full grant again.

## Replacement Signatures and Actual Caller Map

Low-level replacement signatures are DocumentTextJob.advance(items, work), DocumentVectorJob.advance(items, work), DocumentVectorPreparationJob.advance(source, items, work), DocumentRasterJob.advance(items, work), DrawingScenePlacementJob.advance(source, items, work), and DrawingPngExportJob.advance(items, work). Each exposes nextWorkDemand()/lastTurnWork(); each compound close retaining font custody implements the funded first-party FontRetirementCursor contract. Nonfont owners keep their own defining contracts; they are not a numeric font compatibility bridge.

The high-level signatures become prepareDocumentVector(document, limits, algorithms, control), rasterizeDocument(document, limits, viewport, algorithms, control), and exportDrawingPng(document, limits, algorithms, control, options?). The required control includes work:DrawingSceneWorkController plus existing optional signal/font provider/progress and semantic quantum settings. No default empty control, optional work provider, or workBudget-only overload remains.

All actual internal replacements are in scene/text/TypeScript; scene/prepare/TypeScript (text handoff, vector, borrowed preparation, raster, completeSceneProducer and close); scene/placement/TypeScript (vector and close); and editor/commands/export-document/TypeScript (raster, PNG and yielded export/close). The bounded exact symbol search found no other non-test direct caller in tracked TS/TSX/MJS. Existing standalone serializer/picking/export fixture callers must all pass explicit fixture controllers and use the removed numeric signatures coherently. The already funded Layout paragraph/composition path is distinct and remains unchanged.

## Shared Contract and Review Gate

controller.json records zero admission, paid pending, child error after effects, overrun recorded before refusal, real close-release denial/payment, expired turn and foreign-controller refusal. The existing work contract/direct tests cover real same-ledger physical/logical receipts, glyph identity, geometry, finite work, cancellation and controlled close. Rust owns the genuine StepContext composition law for native retained receipt and CPU folding. Low must review this final lifecycle and the current test-first Source before TypeScript implementation. The actual zero-ledger work1 RED establishes the current bug, not passage of this new contract. Full genuine owning Rust/TypeScript verification remains required after implementation.

## Concrete Test-First Class Surface

Root approved a first-party class for implementation after this test-first review. DrawingSceneWorkController takes mandatory operation and source object identities, finite total stock and CPU fuel, mandatory provideTurn(request) and wait(request), plus an optional semantic cancellation signal. provideTurn returns an explicit grant/fuel loan from the caller; the class validates it against independently remaining total stock/fuel before invoking the callback. No production default is created. Source/operation binding is checked before provider admission and after awaits; private callback turns expire immediately after callback exit and cannot be copied, reentered or substituted.

The exact proposed methods are withTurn(operation, source, mode, demand, callback), waitForTurn(operation, source, mode, demand), receipt(), remaining(), and fuelRemaining(). The shared synchronous runDrawingSceneWorkTurn(turn, producer) invokes the producer with one actual FontWorkLedger, posts its cumulative receipt once and CPU separately even if either child or retained fold throws. Both causes survive, along with recorded effects. The context primitive consumeRetainedProgress supplies record-before-refusal semantics.

The generic high-level driveDrawingSceneWork(controller, operation, source, producer) drives actual admitted turns, awaits the caller's wait policy on blocking, and retains semantic output privately until actual close settles. resumeDrawingSceneWorkClose resumes the actual retained funded cursor or opaque pending-owner capsule under the SAME operation/source/controller stock. A failure carries pending without exposing raw output; its owned fields/cursor remain private. No budget reset occurs on recovery. Semantic abort blocks publication; explicit close-mode funding must still be supplied by the caller, or the pending owner is returned for later recovery.

controller.test.ts imports this actual future module, not a fixture implementation of its validator/accounting. Its fixture policy provides finite original stock and explicit loans. The finite owned test producer uses the actual first-party FontBuffer/FontWorkLedger/FontRetirement APIs; it does not implement controller accounting. One final-error case truly reserves portable FontBuffer capacity in the failing turn and requires that final capacity8 and CPU1, independent of prior totals. A hostile descendant receipt makes retained accounting refuse; the actual controller must still record it, fold CPU and preserve both failures.

Additional concrete laws cover postcallback borrowed-turn reuse, forged original-grant receiver, foreign operation/source before provider, original-stock excess, reentry, provider rejection, provider resolution after cancellation, wait rejection/cancellation, and actual paid-owner retention/recovery after refused/partial close and provider/wait failure. Private pending output may not escape and resumed close may not renew original stock. All of these are authored Source, not executed proof. The actual behavioral RED input/log is unchanged; its extra captured aggregate TS test root was legitimately extended AFTER terminal to mount this new controller law.
