# Resumable Declaration-Domain Cursor Design

Status: implemented source, with actual TypeScript continuation checks passing. The shared generator core is the single source for synchronous drains and retained TS domain/equality/repair cursors. Native cursor.rs is mounted in shared4/43 and own2 retained work; original fuel exhaustion preserves positions rather than replaying domain scans. Native execution remains unverified behind the recorded build gate.

## Owned Work Seam

Keep the current retained command factory, captured snapshot Arc/source capability, pending mutation vector, receiver/copy work, publication codec and Store authority. Add a declaration cursor as another charged work-frame member before publication. It owns only bounded traversal state; it borrows values afresh each step from those existing owners. A cursor must never retain Rust references into its enclosing work frame across calls or clone a second graph.

Represent a value location as an owner handle (original argument, prepared mutation argument, own Part21 constituent argument) plus a bounded native child path. A schema handle identifies catalog, entity declaration, explicit/inherited attribute and a finite alias/item/select route. Catalogs remain schema-specific static declarations. Both native owned4 and exact Part21 values implement the same borrowed DomainValue interface, so a domain cursor uses their actual representation without conversion.

## Continuations

Phases: attribute admission; alias resolution; constructor head; marker/optional check; scalar lexical/width check; aggregate cardinality; changed item traversal; reference lookup; uniqueness comparison; equality continuation; complete/refused. Each step performs one bounded node, codepoint, decimal digit, entity/constituent candidate or equality pair and debits the original StepContext. should_yield returns Pending with indices preserved; cancellation returns an original-turn failure and leaves every source/pending owner for close.

STRING/BINARY state retains byte/codepoint offset and scalar count. Exact decimal state retains coefficient scan offsets, leading/trailing zero boundaries, effective base10 exponent and sign; no coefficient copy or binary64 cast. Reference state retains row/constituent index, matched-identity count and subtype status, so duplicate IDs cannot hide after an early successful target.

Aggregate state retains child index and bound deficit/excess. Repair uniqueness state retains candidate group, prior/next member positions and group multiplicities, refusing newly increased duplicate groups. Equality state must include nested ordered comparison and unordered group-count continuations; recursively calling today's one-shot equal function inside a resumable outer loop would leave the actual expensive operation unresumable.

Use an explicitly bounded traversal frame stack and nested equality frontier. Fixed storage must be included in the work-frame size; dynamically retained storage must declare and consume original retained capacity/copy/release grants before allocation or push. No uncharged Vec matching frontier or temporary snapshot conversion is admitted. Domain depth remains128; source path/cursor frame caps refuse predictably.

## Publication and Close

Only a completed cursor sets domain_validated. Existing actual mutation row/byte validation and borrowed publication codec run afterward, with final original cancellation/fuel checks. Pending at any point preserves mutation custody. Close first drains an active cursor's charged state, then the existing receivers/copy cursors, displaced values, pending mutations and original source in the existing authority path. No parallel publisher or alternate undo path is introduced.

## Required Executable Regressions

Run every neutral domain and repair vector at grant/fuel1 through completion, compare with a single-turn run, and apply/undo the same real primitive batch. Pause midway through long Unicode/BINARY/decimal scans, target lookup after the first matching duplicate ID, nested BAG equality and SET repair multiplicity scans. Cancel the same OriginalWorkFixture token at each retained phase and prove no emission, graph unchanged and owned close empty. Exhaust exact final fuel before completion and require Pending, then resume with the same source identity. Test both owned4/43 nominal catalogs and own2x3 Part21 values, including arbitrary coefficients/exponents and u64 identities.

Native compilation/host execution is still gated by the separately recorded repo cache AccessDenied issue. Tree-sitter checks establish native grammar only. Actual TS continuation tests establish borrowed generator logical progress/cancel/yield behavior; owner custody and mounted physical accounting still belong to the embedding retained owner. See ifc2x3-retained-preparation.md for implemented native preparation and its remaining one-shot boundaries.
