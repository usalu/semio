# Retained Clone Cursor Combined Close Contract Inventory

21 October 2026. Read-only source inventory; this report adds no Native runtime evidence and changes no cursor implementation or policy.

## Result

There are 26 implementation/template loci in the active Root and framework workspace source graph. Eighteen declare a literal trait `close_demands` method; two derive templates inject the explicit method produced by `close_demand_methods`. Six implementations still inherit the trait default. Generated struct/enum cursors are not missing the method: the shared generator at `🧰️framework/🔨️modules/🌱️value/✨️derive/🧬️retained-clone/🦀️.rs:199` emits it into both template bodies (lines 318 and 533).

## Remaining Default Consumers

- `impl<S: Send + Sync + 'static> RetainedCloneCursor<ArtifactChild<S>> for ArtifactChildCloneCursor<S>` — `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️retained-clone/🦀️.rs:136`.
- `impl<K: RetainedClone, V: RetainedClone> RetainedCloneCursor<RetainedOrderedMap<K, V>> for RetainedOrderedMapCloneCursor<K, V>` — `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:64`.
- `impl RetainedCloneCursor<DslValue> for DslValueRetainedCloneCursor` — `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🌱️dynamic/🦀️.rs:121`.
- `impl<M: RetainedCloneEntries> RetainedCloneCursor<M> for RetainedCloneEntriesCursor<M>` — `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧺️std-collections/🦀️.rs:131`.
- `impl RetainedCloneCursor<DropProbe> for DropProbeCursor` — `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📋️paged-list/🧪️tests/🔬️unit/🦀️.rs:85`.
- `impl RetainedCloneCursor<ReleaseProbe> for ReleaseProbeCursor` — `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📋️paged-list/🧪️tests/🔬️unit/🦀️.rs:287`.

The four production cursor owners already define an inherent combined selector: ArtifactChild at line 126, retained ordered map at line 53, dynamic value at line 52, and standard collection entries at line 122. Their concrete `self.close_demands(...)` calls can select that inherent method, while generic `RetainedCloneCursor::close_demands` dispatch still uses the trait default. The independent copy/release/depth observers call the inherent selector with body zero; the capacity observer uses the caller body. Therefore the trait default can combine axes selected for different bodies. This is a contract risk established by source, not a claim that each of these four owners currently has a failing body.

DropProbeCursor reconstructs owner/binding axes independently. ReleaseProbeCursor omits the copy observer as well, so its inherited combined quote can return the existing unsupported-copy error; its direct close step simply completes. These fixture semantics need explicit owner-defined answers if the combined method becomes required. The audit does not change either fixture.

## Recommended Clean Contract

Make `RetainedCloneCursor::close_demands(body)` required, like the existing required selected retirement demand. Each owner must quote a single actual next event and preserve the body semantics used by its corresponding close step. Forward quotes, no-progress policies, native currency limits, and the old independent observers remain separate concerns. Removing the default prevents a new cursor from silently recreating the already observed independent-axis defect.

For the four production owners, the bounded implementation would expose their existing inherent selector through the required trait method, using an unambiguous concrete/inherent call or a semantically named private helper. It must not recurse through the trait. For the two fixtures, author the explicit intended close event, preserving deliberate refusal behavior. Keep both derive template injections intact. Do not pad grants, choose independent maxima, or increase body/capacity limits.

Before such a source change, preserve the currently frozen affected-seven runtime request. After its actual diagnostics, any mandatory-method change needs schema-first neutral rows contrasting a correlated body-selected event with independently reconstructed axes, a third-party SQL oracle, actual generic trait dispatch tests on the affected owners, and a compiler check that no impl/template omits the required method. This is a recommendation, not an unreviewed source expansion.

## Coverage and Limits

The inventory began with 973 unique physical Cargo manifests explicitly listed by Root/framework workspace member arrays and resolved 1,146 declared/default Rust entry points. It followed literal Rust module paths, inline path-directory overrides, literal includes, and both manifest-relative concat includes. The bounded read covered 8075 physical Rust files / 89997559 bytes; no literal module edge remained unresolved. No RetainedCloneCursor import alias was found. Character/string/comment masking and balanced implementation bodies excluded `impl Trait` arguments from the roster.

All cfg branches were inspected syntactically; this does not establish which branches compile for a particular host, nor expand arbitrary macros or inventory detached historical source outside the active workspace graph. The two known derive templates and the tuple macro template were read explicitly. Limits were 25,000 files, 128 MiB aggregate, 2 MiB per file; no cap was exceeded. No compiler or tests were run for this audit. The native `repo query` attempt was silent and ended by its exact owned TTY Ctrl-C (whole 1); it supplied no search result and is excluded from evidence.

## Full Implementation and Template Roster

| # | Implementation Header | Combined Selector | Physical Source |
| --- | --- | --- | --- |
| 1 | `impl<S: Send + Sync + 'static> RetainedCloneCursor<ArtifactChild<S>> for ArtifactChildCloneCursor<S>` | inherited default | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️retained-clone/🦀️.rs:136` |
| 2 | `impl RetainedCloneCursor<StrictAtom> for StrictCursor` | explicit trait method | `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧮️scalar/🧪️tests/🌐️lifecycle/🦀️.rs:23` |
| 3 | `impl #impl_generics #value_crate::retained_clone::RetainedCloneCursor<#name #ty_generics> for #cursor_name #ty_generics #where_clause` | generated explicit method via close_demand_methods | `🧰️framework/🔨️modules/🌱️value/✨️derive/🧬️retained-clone/🦀️.rs:318` |
| 4 | `impl #impl_generics #value_crate::retained_clone::RetainedCloneCursor<#name #ty_generics> for #cursor_name #ty_generics #where_clause` | generated explicit method via close_demand_methods | `🧰️framework/🔨️modules/🌱️value/✨️derive/🧬️retained-clone/🦀️.rs:533` |
| 5 | `impl<S:RetainedClone+ImmutableRootProjection> RetainedCloneCursor<S> for ImmutableRootCloneCursor<S>` | explicit trait method | `🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🧬️clone/🦀️.rs:20` |
| 6 | `impl<const N: usize> RetainedCloneCursor<PagedBytes<N>> for PagedBytesCursor<N>` | explicit trait method | `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📦️paged/🦀️.rs:59` |
| 7 | `impl<const N: usize> RetainedCloneCursor<PagedUtf8<N>> for PagedUtf8Cursor<N>` | explicit trait method | `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📦️paged/🦀️.rs:195` |
| 8 | `impl<V: RetainedClone, const N: usize> RetainedCloneCursor<PagedMap<V, N>> for PagedMapCursor<V, N>` | explicit trait method | `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📦️paged/🦀️.rs:423` |
| 9 | `impl<K: RetainedClone, V: RetainedClone> RetainedCloneCursor<RetainedOrderedMap<K, V>> for RetainedOrderedMapCloneCursor<K, V>` | inherited default | `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🔁️clone-cursor/🦀️.rs:64` |
| 10 | `impl RetainedCloneCursor<DslValue> for DslValueRetainedCloneCursor` | inherited default | `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🌱️dynamic/🦀️.rs:121` |
| 11 | `impl<T: RetainedClone, const N: usize> RetainedCloneCursor<PagedList<T, N>> for PagedListCursor<T, N>` | explicit trait method | `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📋️paged-list/🦀️.rs:72` |
| 12 | `impl<M: RetainedCloneEntries> RetainedCloneCursor<M> for RetainedCloneEntriesCursor<M>` | inherited default | `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧺️std-collections/🦀️.rs:131` |
| 13 | `impl<T:Send+Sync+'static> RetainedCloneCursor<SealedShared<T>> for SharedCloneCursor<T>` | explicit trait method | `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️issuance-clone/🦀️.rs:10` |
| 14 | `impl<T: RetainedClone> RetainedCloneCursor<T> for RetainedFieldCursor<T>` | explicit trait method | `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📋️field/🦀️.rs:16` |
| 15 | `impl<T> RetainedCloneCursor<T> for ScalarCursor<T> where T: RetainedClone + Copy,` | explicit trait method | `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🔢️scalar-cursor/🦀️.rs:19` |
| 16 | `impl<T: RetainedClone, const N: usize> RetainedCloneCursor<[T; N]> for ArrayCursor<T, N>` | explicit trait method | `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🧮️array-cursor/🦀️.rs:25` |
| 17 | `impl RetainedCloneCursor<String> for StringCursor` | explicit trait method | `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🔤️string-cursor/🦀️.rs:14` |
| 18 | `impl<T: RetainedClone> RetainedCloneCursor<Vec<T>> for VecCursor<T>` | explicit trait method | `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/📚️vec-cursor/🦀️.rs:48` |
| 19 | `impl<T: RetainedClone> RetainedCloneCursor<Option<T>> for OptionCursor<T>` | explicit trait method | `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🎁️option-cursor/🦀️.rs:22` |
| 20 | `impl<T: RetainedClone> RetainedCloneCursor<Box<T>> for BoxCursor<T>` | explicit trait method | `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🗃️box-cursor/🦀️.rs:22` |
| 21 | `impl<$($type: RetainedClone),+> RetainedCloneCursor<($($type,)+)> for $cursor<$($type),+>` | explicit trait method | `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🧱️tuple-cursors/🦀️.rs:35` |
| 22 | `impl RetainedCloneCursor<DropProbe> for DropProbeCursor` | inherited default | `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📋️paged-list/🧪️tests/🔬️unit/🦀️.rs:85` |
| 23 | `impl RetainedCloneCursor<NonconformingChild> for NonconformingChildCursor` | explicit trait method | `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📋️paged-list/🧪️tests/🔬️unit/🦀️.rs:154` |
| 24 | `impl RetainedCloneCursor<NonconformingRetirementChild> for NonconformingRetirementChildCursor` | explicit trait method | `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📋️paged-list/🧪️tests/🔬️unit/🦀️.rs:212` |
| 25 | `impl RetainedCloneCursor<ReleaseProbe> for ReleaseProbeCursor` | inherited default | `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📋️paged-list/🧪️tests/🔬️unit/🦀️.rs:287` |
| 26 | `impl RetainedCloneCursor<InsufficientScaffoldChild> for InsufficientScaffoldChildCursor` | explicit trait method | `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📋️paged-list/🧪️tests/🔬️unit/🦀️.rs:389` |

## Pending Execution

The authored input plan is retained at ticketroot `e2e-value-stable-original-plan.json`, outside generated output. It requires affected seven complete libraries, actual positive/custom-Drop compile-fail façade doctests, then the full fourteen-library matrix. The 69 advertised triples and all supplemental bodies remain independently counted. Parent filesystem Native request 75018 owns the slot at this report boundary; no Native request was started here.

## Accepted Follow-Up Decision

Parent approved the required-method contract correction after the affected-seven actual diagnostics. Remove the default entirely, forward all four existing production inherent selectors through explicit trait methods, and define both fixture selectors deliberately; no compatibility implementation. The generated methods remain explicit. Targeted neutral/third-party parity and actual generic-dispatch laws must precede the production correction. Shared Source TS/schema changes wait for Root request 57784 to settle; no source change is made by this audit.

The retained execution plan now records two additional physical owners outside the original full-fourteen roster: `semio-framework-value-retained-map` and `semio-framework-os-store-child`, each in the framework workspace. Their actual compilation/dispatch acceptance must be explicit after the contract correction. They must not be inferred passed from the fourteen-package matrix. The advertised 69-triple inventory remains unchanged.

## Targeted Regression Design

Reviewer traced each genuine owner selector: ArtifactChild orders child-id cursor, target cursor, pending child-id/target/local alias, then whole output/binding; ordered map orders key cursor, value cursor, pending key/value, current page, pages, then map/binding; standard collection entries uses its corresponding key/value/output order; dynamic value orders child, active close, bytes, key, then target/binding. Tests must create states through genuine Source admission and public advance, then begin close. Do not inject private fields or invent a body-dependent measured event.

For each owner, assert generic UFCS `RetainedCloneCursor::close_demands(&cursor, B)` equals the complete concrete inherent quote at the same state, with exact denied-axis/heap/source identity and finite paid cleanup retained. If a reachable real state does not yield different quotes at zero and B, an allocation-free cfg(test) observation at the genuine inherent selector can prove dispatch selects exactly once with B. The old default calls the inherent selector through observers with body transcript `[0, B, 0, 0]` (copy, capacity, release, depth); the required explicit method must have transcript `[B]`. Observation must be reset/read outside heap measurements and confined to test compilation. This tests actual generic dispatch even when both resulting tuples happen to equal.

The language-neutral counterexample is separate: a zero-body child-birth event and a sufficient-body payload event have correlated copy/capacity/depth tuples. Independent axis reconstruction yields a tuple that belongs to neither selected event. Closed JSON rows plus a first-party model and SQLite query should verify tuple selection and the exact observer transcript, without labeling those neutral byte values as an ArtifactChild or map measurement. The existing contract schema owns first-party demand/grant/receipt DTOs; extend its semantic close-test subtree rather than adding a permanent executable script or external runtime type.

Actual package gates must include both additional owners, preserve the existing full-fourteen/69 gate, and prove generated struct/enum methods remain present after making the trait method mandatory. A missing-method compiler-negative fixture should call the true trait in an admitted doctest/test owner, with a matching positive implementation; source text searches alone are not acceptance.

### Remaining Reference Check

The other source files containing the unaliased RetainedCloneCursor token were inspected for hidden implementation generators: they contain imports, bounds, helper calls, handoff wrappers, tests, the trait itself, and the separate demand extension trait. No additional named cursor implementation or generic-trait macro emitter was found there. The roster remains 26 loci / six inherited consumers.

## Authored Required Selector Correction

Root released the quiet boundary after capacity-only cancellation of request75018; there was no admitted Source/compiler/body red. The schema/neutral cases, SQL oracle, four actual owner regressions and actual missing-selector positive/compile-fail doctests were authored first, followed by removing the trait default and defining all six formerly inherited implementations. Four production trait methods forward directly to their existing concrete selector. DropProbe uses actual selected owner/binding closure; ReleaseProbe explicitly quotes its intentional zero-effect completion. Derive generated methods are unchanged.

Each production inherent selector has cfg(test)-only, allocation-free per-cursor/body observation. Four tests use genuine admitted Source/public advance cuts0/1/3/7, check concrete versus generic full tuple, one root-body observation, query/zero-item denial zeroheap, original pointer identity, exact funded quote re-selection and finite1024 cursor/Source closure. The observation helper is one physical source file shared by all four owners. Neutral tuples are protocol DATA rather than claimed measured owner extents. OS child adds one declared first-party allocator dev-only edge; no lock command has run at this boundary.

Native plan is now affected9, façade4doctests, complete16, preserving all original14/69 plus two additional physical owners. Independent source review and all actual runtime gates are pending. Curated permanent attribution now306paths (304present/2removed), maxabsoluteUTF16=180.

### Corrected Actual Workspace Boundary

Fresh declared workspace resolution corrects the earlier claim that both additional owners are framework members: `semio-framework-value-retained-map` belongs to the framework workspace, while `semio-framework-os-store-child` belongs to Root (`package.workspace = "../../../../../../../.."`). The OS child exists only in Root Cargo.lock and the new declared first-party allocator dev edge is absent there. Framework lock has no OS-child row; it needs no refresh for this edge. The native plan now separates affected framework8, Root OS child1, façade docs4, then complete framework15; the complete16 conceptual matrix includes the Root child gate separately. Original14/69 remain retained. No invalid framework package selection, lock edit, or runtime command has been performed.

### Source Ownership and Stronger Physical Harness

The shared harness now denies zero items and every feasible positive copy/capacity/release/depth axis one below its selected demand, requiring unchanged full quote/source identity and zero heap/default progress or typed refusal. Funded cursor and Source close steps are measured against their exact physical receipt. The standard-entry dispatch fixture is an actually admitted empty BTreeMap: nonempty standard backing is explicitly unpriced by the existing owner and is not falsely claimed covered by this new physical equality test. Existing nonempty collection laws remain unchanged.

A bounded literal-module observation found260,514B of existing retained-clone Rust source before new4,496B shared helper, already exceeding262,144B when combined. Four public standard-collection integration laws and unchanged public helpers therefore move to the existing public integration package. Their original private String prefix neighbor stays at its current owner with every assertion preserved, using the existing canonical FixtureSourceBirth and same finite close helper. Actual graph ownership/fixturepath review is pending; reduction7,419B is not a claim of admitted Source. No cap changed; no69 triple refers to those four moved laws.

Current permanent ownership 307 paths; all Native, doctest, SQL and Source execution pending.

### Independent Review and Owned Lock Attempt

Independent reviewer accepted required-trait/fixture/doctest source, strengthened actual harness, and public standard-collection carve/mounts. All runtime acceptance is still pending. Sole owned lock refresh TTY94746 runs `repo cargo --store=shared --configuration=prototype -- update --manifest-path Cargo.toml --workspace --offline`, artifact directory `🗑️generated/slr`, log `🗑️generated/e2e-selector-root-lock-refresh.log`. Before snapshots independently parsed through @iarna/toml: Root1562packages/706registry, framework1066/612. At this record it is host-capacity queued with no Cargo body; no lock success or delta inferred. Before projections: `e2e-selector-root-lock-before.json`, `e2e-selector-framework-lock-before.json` under generated.

### Capacity Identifier Sanity

A bounded read of this owned request found no capacity grant/request/PID identifier in its retained log, and its declared `slr` artifact directory remains absent before phase admission. Tool session94746 establishes exact TTY custody, not the identity of any capacity holder. The observed one-running/zero-ahead counters cannot establish whether the holder is a parent/nested request or external work. No process query, hoststore mutation or holder signal was made.

## Quiet Boundary: Owned Lock Request 94746

The exact owned PTY received Ctrl-C at the parent-requested quiet boundary and settled with whole exit code 1. The request remained in host-capacity waiting; no Cargo body or lock refresh ran. Independent TOML projections after settlement exactly match both saved before projections: Root 1,562 packages / 706 registry entries; framework 1,066 packages / 612 registry entries. This is cancellation evidence, not a successful refresh. The new OS-child dev allocator edge still requires a fresh genuine offline Root refresh after the canonical OS MCP probe registration and review.

The neutral selector correlation assertion now compares all four numeric axes directly, avoiding property-order dependence in JSON serialization. The Ajv and independent SQLite complete-event oracle remain unchanged. This test-only correction landed after request 94746 settled; runtime remains pending.
