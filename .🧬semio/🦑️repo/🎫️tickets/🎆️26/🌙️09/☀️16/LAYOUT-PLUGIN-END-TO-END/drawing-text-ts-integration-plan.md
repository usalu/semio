# Drawing TypeScript Funded Integration Plan

Current Source map only; production is held pending Root review and the focused behavioral RED. Rust owns native production. dependency_layout owns shared text schema/tests and, once released, the matching TypeScript integration. No permissive numeric-only font API or minted unlimited ledger is proposed.

## Existing Signatures and Required Replacement

The first-party TypeScript FontWorkLedger already accepts FontRetainedGrant with items/copy/capacity/release/depth, exposes receipt/remaining/nextDemand/blocked, and owns FontBuffer reservations. Font Unicode/catalog/run jobs already require advance(items, SAME ledger), and their teardown returns FontRetirementCursor requiring the same funded ledger. Drawing text still calls them item-only and archives them as numeric WorkRetirement. It also copies faces with slice(), archives owned children/content in uncharged arrays and publishes a spread content header without original work accounting.

Replace Drawing DocumentTextJob.advance(items, ledger), add nextWorkDemand() and lastTurnWork(), and use a first-party funded FontRetirementCursor for its original child/output/content/geometry custody. Stable empty spans, auto direction and explicit outlines:true preserve uniform authored text semantics. Retain original catalog font arrays instead of the uncharged faces.slice(). Use original funded buffers for archives and geometry/header moves; no per-child ledger renewal. Capture receipts/demand even on child error and distinguish paid preparatory allocation from a zero-effect blocked return.

## Exact Current Production Caller Files

All paths are beneath Drawing any:

- schema/scene/prepare/TypeScript: DocumentVectorJob.step text branch line45, advance line48; DocumentVectorPreparationJob.advance(source,items) line69; DocumentRasterJob vector branch line100 and advance line104. These must forward the same ledger and actual current-phase demand/CPU count, including close and handoff states, not only active text.
- schema/scene/placement/TypeScript: DrawingScenePlacementJob vector branch line21, public advance(source,items) line37, vectorClose/retirement. Forward the actual caller ledger through the vector child and funded close; do not manufacture byte grants from items.
- editor/commands/export-document/TypeScript: DrawingPngExportJob raster branch line29, advance line33, retirement and exportDrawingPng loop line44. The actual grant/receipt boundary must encompass the raster subtree and close, with separate CPU quantum and one cumulative receipt fold.
- schema/scene/prepare/TypeScript completeSceneProducer line77–79 drives prepareDocumentVector and rasterizeDocument. It currently has only optional workBudget number, signal, fontProvider and progress callback. Its numeric close loop is insufficient for funded font descendants.

The bounded Draw+Layout production search found no further direct references to these producer names outside those files and text itself. It is a direct-call map, not a whole-runtime dynamic call proof. Existing serialization/picking/export tests instantiate numeric vector/raster producers and must be handcrafted to the replacement contract; no old signature should remain supported for convenience.

## Unresolved Actual TypeScript Grant Owner

Current high-level DocumentVectorOptions, DocumentRasterOptions and exportDrawingPng control expose numeric workBudget only, with default empty options. That is semantic scheduling authority, not five-axis retained authority. A faithful integration requires the actual caller to provide first-party finite work-grant/receipt ownership, e.g. a closed own interface that obtains each fresh remaining grant and consumes its actual receipt, including close and error paths. It must be wired to the genuine defining scheduler/controller policy. Merely filling byte/depth fields from nextDemand, a fixed huge value or the integer workBudget inside production would fabricate capacity/release authority.

Root must approve this precise high-level boundary before TypeScript production edits. Tests may issue exact finite fixture grants based on actual demand as test policy, but that is not a production authority source. The current funded LayoutParagraphState/DrawingCompositionState already demonstrates receipt forwarding from a genuine five-axis caller and remains outside this repair.

## Test-First Evidence Plan

The new focused zero-work.test.ts does not call any proposed API before its assertion. It supplies the actual existing first-party ledger with all five axes zero to current advance(1, ledger), and checks public work0/nodes0/phase nodes/no completed output/same content owner. Current numeric-only advance ignores the extra runtime argument and should expose work1/fontSources. This is an intended behavioral counterexample, not an assumed result; import/bootstrap failure must be classified separately.

The new shared work schema/cases and direct Rust/TS regression laws preserve the existing fourteen glyph rows and external independent geometry checks. Actual StepContext composition CPU/receipt-before-error tests belong to Rust's production caller packet. No semantic execution or current pass is claimed yet.
