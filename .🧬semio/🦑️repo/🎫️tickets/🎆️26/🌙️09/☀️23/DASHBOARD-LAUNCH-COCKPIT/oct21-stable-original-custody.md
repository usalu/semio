# Stable Original Custody

The diagnostic gate 85883 completed with exit 101: 97 passed and 12 failed. The wide record failed while closing its original Source, where the old final shared unwrap quoted 5128 copied bytes and 2576 released bytes. The 4096 copy limit remains unchanged. The projection candidate already used stable storage; this is a separate original-allocation defect.

## Current Design

The first-party original owner stores initialized T in an opaque counted allocation containing an atomic logical alias count and UnsafeCell<MaybeUninit<T>>. Its safe constructor admits the exact Layout-derived allocation before effects. Live handles expose immutable references only. Consuming retirement atomically elects exactly one original owner; other aliases release their physical references. The elected owner retains the same allocation, and final physical release waits until delayed peer references have dropped. Return slots transfer the same counted handle, including its original issuance. No public Arc, Weak, mutable payload, or safe initialization claim is exposed.

Derived native retirement optionally consumes that same allocation field by field. Each event admits twice the field movement width and the exact native child birth, transfers the field once, advances the ordinal before invoking user code, and hands the native child to the existing caller frontier. The allocation is freed only after all fields have transferred. Packed layouts and custom Drop implementations are refused. A safe external hook cannot fabricate initialized storage; destructive field operations carry explicit unsafe layout contracts. Unsupported native fields refuse before extraction.

The issuer still contains three function pointers, so shared handles and wide records do not grow from extra callback fields. The allocation itself adds its counted metadata; all constructor and release quotes use its actual Layout including alignment. Small owners without the optional capability retain their honest by-value native path.

A quote-to-election interleaving may elect the last owner after an alias-only quote. The actual internal transition revalidates the elected owner's demand before effects; an inadequate grant pays only the structural election event. The next turn receives the new quote.

## Authored Verification

Closed neutral schemas and TypeScript/SQLite oracles cover alias events, delayed physical release, native admission, field-constructor panic, and obsolete alias quotes. Actual Rust laws cover concurrent two-alias election, delayed physical references, partial field drain, denied release, real constructor Err/unwind slot retention, the actual Cursor election window, panicking field constructors and unsupported known-birth fields. Facade doctests include positive derivation and a custom-Drop compile refusal.

These new runtime laws have not executed. The filesystem suite owns the current native slot. Full acceptance must add the generic retirement library to the existing thirteen-library matrix, explicitly run the new owner laws, retain all 69 advertised exact triples and supplemental cases, and run the positive/compile-fail facade doctests through the managed repo Cargo route.

## Other Narrow Corrections

The Box depth refusal now uses a borrowed static error instead of allocating its 50-byte message. The strict public child test uses the existing checked one-row Work bound rather than counting eight paid steps as a stall. Paged parents retain their own child-start phase before querying child closure. Adversarial fixture cleanup and native close consumers select one complete event instead of reconstructing independent axes. Collection and PlainArray combined queries now use the same body-selected spill as their actual native step; the authored zero-copy spill token is preserved.

Exact attribution is retained in oct19-e2e-controls.md and generated/e2e-controls-owned-paths.json (293 paths; maximum absolute UTF-16 length 180).

## Finite Execution Plan

The retained plan is `e2e-value-stable-original-plan.json`. Each package is resolved through an actual framework workspace member and its physical Cargo manifest/lib path; all fourteen source entries exist. Stage one runs seven affected complete libraries with `--locked --no-fail-fast --lib`, without a name filter. Stage two runs all facade doctests, including the new positive derivation and custom-Drop compile refusal. Stage three runs fourteen complete libraries, preserving the exact 69 advertised package/lib/full-law triples and all supplemental bodies. Every stage uses the prototype managed store, authored 128 MiB test stack and one test thread under the normal repo Cargo ingress.

The seven new native laws are physically mounted through the generic retirement original module (three), sealed frontier tests (two), and projection record tests (two). Existing four public integration laws remain mounted under their dedicated actual integration package. The new facade examples are inside the public value module, not an unmounted research file. No zero-test, ignored, filtered, compile-only or AST-only result can satisfy these body gates. Whole exit and per-package result counts will be retained separately from advertised/supplemental name projections.

### Supplemental Mount Audit Before Native Release

The bounded literal module graph reaches all seven new stable-original laws from their exact planned package library roots. The four retained public integration laws are each present once and reach the dedicated integration package root. All eleven are declared tests with no ignored annotation found in their declarations. Generated projection: `🗑️generated/e2e-value-new-body-mounts.json`. This establishes source/mount coherence only; real Cargo body output and the two façade doctests remain mandatory.

The authored execution plan is retained at ticketroot `e2e-value-stable-original-plan.json`; generated output contains no copy. Required combined-close contract follow-up and its two additional physical production owners are recorded there. The cursor audit is `oct21-retained-clone-cursor-contract-audit.md`.

### Corrected Actual Workspace Boundary

Fresh declared workspace resolution corrects the earlier claim that both additional owners are framework members: `semio-framework-value-retained-map` belongs to the framework workspace, while `semio-framework-os-store-child` belongs to Root (`package.workspace = "../../../../../../../.."`). The OS child exists only in Root Cargo.lock and the new declared first-party allocator dev edge is absent there. Framework lock has no OS-child row; it needs no refresh for this edge. The native plan now separates affected framework8, Root OS child1, façade docs4, then complete framework15; the complete16 conceptual matrix includes the Root child gate separately. Original14/69 remain retained. No invalid framework package selection, lock edit, or runtime command has been performed.
