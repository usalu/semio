# Catalog-Aware IFC Details

## Implemented Surface

IFC4 full snapshot Details now consumes its own normative IFC4 ADD2 TC1 catalog for every entity and explicit serialized attribute, including inherited slots. The selected entity semantic inspector remains available separately. The provider exposes declaration labels and types, optional unset/create variants, derived read-only state, scalar/enum controls, named SELECT leaves, nested typed values, aggregate bounds/order/uniqueness controls, exact decimal references, collection reference insertion, complex constituents, and entity/constituent type variants. Header/source details retain their existing common route.

The native shared provider and declaration-aware edit planner live in IFC-owned `🔎️details/🦀️.rs`. Version wrappers are IFC4 `🚪️io/🪟️details/🦀️.rs` and IFC4x3 `✏️editor/details.rs`. Both consume the already shared lossless IFC graph and mutation package. Each source mount imports its own parent catalog aliases, avoiding nominal Rust catalog mismatches and preventing IFC4x3 declarations from being substituted for IFC4. The IFC4x3 profile preserves its existing public provider name and MVD presentation.

The shared TypeScript twin `🔎️details/🟦️.ts` exposes catalog-parameterized controls, exact reference mutation planning (including complex constituent arguments), and entity type-switch batches. All published edits use existing stable-identity mutations and central diff/inverse APIs. No new publisher or external runtime dependency was introduced.

## Retained Context Integration

The current common factory/profile work hooks and retained copy work were preserved as authoritative concurrent changes. Final custom retained planning now passes the original StepContext into reference target scans, aggregate duplicate checks, argument walks, type/complex planning loops and identity generation; final fuel/cancellation and per-item publication checks run before Complete. Pending mutations remain under the work's retained owner for close rather than being dropped on final refusal. The final guard compares the batch’s declared forward/inverse row extent to profile capacity before emitting; the Store still derives and admits its actual gesture footprint independently. This guard does not replace sequential Store inverse authority. A malformed over-arity tuple is explicitly refused when its type switch exceeds the profile’s finite capacity.

The synchronous special planner is a bounded one-shot alternative and refuses exhausted callbacks; it is not described as yield/resume. The current retained work performs resumable captured argument/entity copying; final planning still uses bounded one-shot work. Generic EditRules.resolve and source replacement retain framework-owned whole-snapshot materialization; no claim of fully incremental generic source editing is made.

## Validation Performed

- Shared language-neutral fixture `🔎️details/🧪️tests/🔣️.json` covers inherited identity, enum members, coordinate collection bounds, inherited derived slots, optional reference domains, full positive u64 IDs, select leaf names, complex references and type variants. Rust and TypeScript consume these declarations.
- The actual TypeScript source suite executes nine groups with zero strict TypeScript diagnostics. It visits every serialized attribute of all 776 IFC4 entities; checks exact IFC4/IFC4x3 enum isolation; applies actual reference/complex mutations and inverses; checks each cancellation callback; and verifies ordered multi-primitive IFCWALL-to-IFCDOOR changes restore the original graph through central inverses.
- Independent existing AJV validates closed enumeration acceptance and declared aggregate cardinality against the same neutral schema contracts. This independently validates those control domains; it is not an EXPRESS rule evaluator. The separate normative catalog report records the broader independent third-party declaration comparison.
- Existing semantic source suite still passes all 13 groups with zero strict diagnostics. The full public IFC facade strict check reaches one unresolved existing @semio-tech/framework-interaction import in framework replication/wire/local-interaction; the targeted details suite has no diagnostics.
- Changed native source groups, including retained work and branch tests, parse with the independent installed Rust tree-sitter grammar, and native neutral fixture include resolution is verified. Native tests additionally author real provider controls, exact references, multi-primitive ordered diff/inverse restoration, every planner callback stop, original cancellation-token identity and exact-final-fuel behavior. The shared work_tests.rs drives actual captured-source/copy Work branches: valid multi-primitive changes, prepared zero/one-fuel and cancellation refusal, malformed 1000-argument capacity refusal, pending argument mutation resume after value transfer, and controlled close draining retained owners/Arc aliases. These assertions are authored and syntax-checked, not executed.

## Execution Limits

Native compilation/test execution remains blocked by the previously reproduced Windows native build-cache case-sensitivity access-denied gate. Rust source parsing is not Rust type checking. Native UI rendering, actual retained-host publication/undo and authored native assertions have not been executed. IfcOpenShell is not available in this local runtime; existing authored IfcOpenShell reference tests must not be described as executed.

Typed declaration controls and reference admission do not implement all EXPRESS WHERE/UNIQUE/inverse/global rules, contextual bounds or complete complex-instance conformance. Required templates can remain unset when no admitted existing reference exists; the provider does not silently invent graph targets. Reference suggestions inspect the first 256 entities, while exact decimal entry reaches every admitted existing ID.

Generated verification output directories from this slice were removed after recording the results. Source tests, neutral fixtures, input harness and this report remain.
