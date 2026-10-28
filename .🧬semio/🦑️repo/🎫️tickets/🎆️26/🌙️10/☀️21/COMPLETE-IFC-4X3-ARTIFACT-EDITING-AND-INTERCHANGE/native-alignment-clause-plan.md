# Native Borrowed Alignment Clause Integration Plan

Current source status: Direct and Terminal local clauses both consume the actual prepared-view interface, and their request/source preparation is folded into one canonical generic owner. `native-mvd-preparation-implementation.md` and its independent audit describe the current source checkpoint. The sections below retain the staged design and intermediate wrapper responsibilities that led to that fold; they introduce no combined public MVD request or full-Alignment verdict.

## Required before behavior

Direct currently owns source/request aliases, container scan, source index and request index, alongside report/diagnostic/member/query/numeric state. Terminal owns the same infrastructure plus one per-request nesting relationship roster. Running both whole Works prepares two source indices. A probe callback around an unused population cannot fix that boundary: actual source reads and ID lookups must come from the prepared population borrowed view.

The public endpoint requests/reports, original caller/source/epoch checks, request validation before source graph access, and all24Direct/38Terminal/29owner-query neutral cases remain authoritative. Empty/duplicate/invalid requests fail before source graph access. A cheap original-paid snapshot Arc alias is retained on the first accepting request turn to establish genuine source lifetime, then the same alias moves into population only after admission. Container-unavailable reports retain the existing global diagnostic and per-target status/copies; they cannot fabricate a prepared view or discard the original request.

## First implementation slice: Direct

Add adjacent direct-clause.rs containing only retained local report/diagnostic/member/query/numeric state and fixed continuation metadata. The existing direct.rs keeps public domain request/report definitions, original endpoint admission, request uniqueness index and wrapper scheduler. It owns the real IfcMvdPopulationWork<IfcAlignmentDirectRequest> plus the clause, rather than another complete Work.

Wrapper phases are closed: first accepting request turn retains one paid snapshot Arc header before fallible construction/posting, without graph reads; original request seal; request index; bounded request semantic validation; donate exact source/request owners to population; actual shared population preparation; local clause progression; original result take. The two donors move once after validation. The same source alias enters population; no second source clone, wrapper shape scan or wrapper source index remains. The request uniqueness index is distinct from the one source-ID index and does not reread or duplicate the graph. This closes pre-source pointer reuse while preserving request-before-model-access admission.

For prepared input, every clause turn runs inside population.with_view. The clause stores and checks the actual PopulationPin, including stable source-index identity, before any debit or mutation. Source reads borrow view.snapshot; reference resolution uses view.lookup; request fields borrow view.request. The original qualified attribute query, Membership and PrimitiveZero remain the only implementations of their respective semantics. Diagnostic allocation/copy and complete report donation keep their existing staged original receipts and partial-owner slots.

Container-unavailable input requires a separate private one-turn request-only accessor on the population owner: it lends the already-admitted original request and exact input/caller/epoch identity, without exposing source/index or granting preparation. Its higher-ranked Copy-result callback prevents a borrowed request escape. A closed clause unavailable route may copy target labels and build existing incomplete/not-applicable diagnostics under original funding. Any graph/query/numeric phase on that route must refuse, rather than use an empty source or missing lookup. This preserves current invalid-container reports without a second source owner or an alternate readiness boolean.

The wrapper's take must run through the same original population/request route and completed clause identity; output remains retained until exact empty-destination original donation. Wrapper derived retirement lists clause/report/query metadata and request-validation metadata before population/source/request owners. Failed post/child progress retains its born owner and original primary; no cumulative double-charge or forecast debit.

## Second slice: Terminal

Extract adjacent terminal-clause.rs with report/diagnostic/member/query/numeric state and the retained per-request relation roster. Its once-per-source relationship discovery reads the prepared view and retains all late unknown/ambiguous owner information. All six qualified owner fields, actual LIST ordering, all-member traversal and existing unknown-priority diagnostics remain unchanged.

Terminal discovery also uses the completed original request uniqueness index to map actual RelatingObject IDs into target ordinals. It must validate that index's actual completed identity against the original request/epoch before use. That index is wrapper-owned metadata, not a second source index. Its completed row buffer and original root identity remain stable across moving the wrapper; no self-reference is stored. If a later generic joint request needs two target projections, that requires a separately closed original projection/index contract; no temporary stack-adapter pointer or unvalidated caller-provided projection identity is sufficient.

Both standalone endpoints then consume the actual same population interface while retaining their current public contracts. The shared population owner can host multiple local cursors, but no synthetic combined request or inferred correspondence is introduced in these migrations.

## Schema-first laws and evidence

Add adjacent closed integration laws before each source extraction. Preserve the entire existing neutral corpora and original H/V/C identity cases. Add actual wrapper assertions: one source Arc retained, one stable source index born/prepared; clause pins match that exact index; no duplicate shape scan; moved wrapper keeps pins; changed source/request/epoch/original caller/index refuses before debit; unavailable source keeps exact existing diagnostics; no graph view on unavailable/pending input.

At every preparation/clause/report/take prefix, deny item/copy/capacity/release/depth/fuel independently and assert state/heap/receipt stationarity. Check actual accepted allocation/release against original receipts, every cancellation prefix, expired ordinary deadlines, all partial report owners and paid original close. A clause child Complete cannot close the wrapper while a retained terminal owner remains. Native test module mounts stay adjacent to the real endpoints and preserve actual selectors.

The existing SQLite neutral lookup oracle and IfcOpenShell declaration/original-input observations remain external reference evidence, separate from actual native results. Run the authentic Cargo selector and retain its exact enforcement outcome; no bypass. Until genuine compiler ingress admits typechecking and execution, all native type/runtime/heap/receipt/mount/capability and semantic publication flags remain false. Syntax parsing and independent source review do not establish those properties.

## Scope sequence

Start with request-only population access and Direct neutral/native wrapper laws, then extract Direct and freeze that bounded source scope for review. Repeat Terminal only after Direct is coherent. Keep TypeScript test sources frozen during root's expanded89 snapshot. Details owns guided BOOL/enum/profile work; the only separate controller change requested here is its adjacent enum native-test module declaration. No build, public transport, renderer or root validation source changes are part of this plan.
