# Scene-Owned Drawing Template Cache Design

## Assessment And Scope

Replace the process-global rich drawing dictionary with an explicitly owned first-party cache context, bounded by entry count and two geometry-resource budgets. Preserve all physical geometry, returned drawing semantics, pure model/mesh caches, geometry-only ordering recipes, and font/text reuse. This is a production lifecycle/control proposal, not a ticket-side private cache clear, raster backend replacement, or permission to edit current running dependencies. No production or active recorder input was changed in this assessment.

The proposed default policy is `max_entries=8`, `max_members=1000000`, `max_points=10000000`, with public configuration and stats. Eight small entries remain possible; two currently measured complete building templates cannot both fit the member budget because each reported family contains 759230 Mobjects. The points count has not yet been measured for that template, so the proposed point threshold is a starting policy to validate, not a measured fit claim. A template exceeding either budget must bypass retention and cloning. These budgets describe geometry resources, not OS bytes, peak RSS, admission memory, or a promise that the entire Scene will fit a particular machine.

The earlier suggestion of a fixed single-entry cache also bounds rich retention but causes avoidable misses for small drawings. The dual-resource policy gives better control while preserving the desired one-full-template consequence of the current measured member count. Entry count remains useful for overhead and very small entries. Hosts can explicitly choose a different first-party policy; there must be no hidden global enlargement or environment-specific default.

## Actual Causal Evidence

The current renderer is `🎓️teaching/🏛️architecture/🏢️nakagin/🎨️visuals/🐍️.py`, raw SHA256 `4d1d855d72e36263b8fa79402de86abb437f869fa61bf93f65b8cca42bc7edc2` in the accepted freeze. Its relevant source is:

- Line 44 declares `_TOWERS: dict = {}` at module scope. It is a strong root independent of any completed Scene.
- Lines 1682–1704 implement `tower_axon`. The key includes view, lit selection, tint colours, inclusion selection, types and assembly flags, towers/storeys/segments/base. Position/scale are intentionally outside the key; coverage padding can require replacing an existing key's template.
- Lines 1694–1698 build on a miss or insufficient coverage. Eviction occurs only when the dictionary already has eight entries, by `pop(next(iter(_TOWERS)))`. Hits do not refresh recency: the current policy is insertion-order eviction, not LRU.
- The right-hand `_tower_drawing(...)` completes before assigning `_TOWERS[key]`. During a coverage upgrade, the old value for that same key remains strongly reachable throughout the replacement build. Other cached templates can remain throughout any new build as well.
- Lines 1699–1703 then call `template.copy()` and transform the clone. The cached canonical template remains reachable even if the returned clone/Scene dies. A rich template and its clone can therefore coexist with previous cache entries and already live animation drawings.
- Lines 1713–1722 build a full `NakaginDrawing3D`, populate capsules/cores/capitals/podium/bridges, and retain semantic dictionaries and painted subobjects. This is a rich object graph, not a compact mesh recipe.

A repository source scan through `repo tools exec --light node` found exactly six `_TOWERS` occurrences, all at the above production declaration/access lines. It scanned ordinary `.py/.cjs/.js/.mjs/.ts` source under the repository, skipped installed dependencies, Git internals, generated outputs, build outputs, interpreter caches and symlinks, and limited individual input files to 4 MB. No runtime consumer or test directly accesses `_TOWERS` outside this implementation in that scan. `repo tools exec --light rg` was attempted first but the registry rejects `rg` as an unregistered tool. Existing ticket Markdown discussion is not a runtime/test reference.

Concrete callers demonstrate multiple retained variants:

- Excel anatomy `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🦴️anatomy/🐍️.py`, lines 332, 338 and 525, requests an initial highlighted full tower and later sidebar-scale variants. The changed scale can require larger coverage padding despite sharing the semantic key. The caller also retains animation/silhouette references.
- Grasshopper anatomy `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🦴️anatomy/🐍️.py`, lines 303–304 and 436, requests live variants with towers/podium/lit changes and an initial highlighted full building. Lines 211–215 have an additional **Scene-owned** `_towers` lookup for static coordinate references. That is independent returned-drawing retention and must not be mistaken for the process-global `_TOWERS` cache.
- Existing visual contract `🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🎨️visuals/🐍️.py`, lines 267–285, verifies clone isolation, position/scale reuse, and unchanged source drawing behaviour. These are mandatory preserved semantics.

The full current profile measured 140 capsules, 17920 source capsule parts, 430509 painted faces and 759230 family members for both cold/warm drawings; see `final-profile-contact-proof.md`. During concurrent native execution the Grasshopper process tree exceeded 26 GB observed. Excel rose above 11 GB while capture and other native work remained live; official available memory reached 443 MB against the 4096 MB floor. Root authorized exact-owned Excel cancellation and memory recovered to 19980 MB; see `excel-native-critical-cancellation.md`.

This proves the global retention mechanism and a concrete aggregate resource failure. It does **not** prove which fraction of RSS was `_TOWERS`, report actual cache occupancy at the failure, or exclude temporary construction/animation copies and other data. No profiler or heavy import was launched for this assessment. A cache repair alone cannot promise to bound live caller-owned clones or concurrent independent jobs.

## Ownership And Public Contract

The neutral cache must depend only on the standard library and expose first-party policy/cost/stats types. Its keys and values are opaque to the cache. It must not import Manim, NumPy, the Nakagin model, or external runtime packages. The renderer supplies the geometry-resource counter and clone/materialization operations behind this interface. External oracle types remain inside tests.

Each `NakaginDrawing` instance lazily owns its own cache/context; remove the module-global rich root. Standalone drawing clients and real Scenes both work without a mandatory Manim constructor override. Never put Scene objects in a global owner registry. A caller may explicitly share a first-party context when it intentionally owns its lifetime; sharing must not silently happen across scenes or languages.

Proposed public renderer methods are `configure_drawing_cache(...)`, `drawing_cache_stats()`, `release_drawing_cache()`, and `drawing_cache_context(...)`. They configure, observe and release **only retained rich templates**. Release is idempotent and leaves caller-owned returned drawings and pure caches intact. Configuration shrink evicts immediately to satisfy every resource bound. Invalid policy fails before mutation. Stats distinguish entries, retained member/point totals, hits/misses, evictions, bypasses, factory calls, and clone calls. Releasing cached entries must drop references; it must not mutate or destroy a template that an external client can still observe through a returned drawing.

Use an explicit context/finally scope around chapter presentation in the two existing clip scene bases, so `scene.construct()` tests receive cleanup even when they bypass Manim's render/tear_down lifecycle. A tear_down-only solution is insufficient for the current native harness. The scope should release on success, exception and cancellation, and restore an outer explicit context correctly. Standalone clients can hold the context across cold/warm calls or release explicitly; they must not rely on private dictionary access or global garbage collection. No returned physical element is filtered for cache reasons.

## Factory Ordering And Oversized Bypass

Weight is unknown before constructing a never-seen template. A simple `factory(); weigh(); evict(); clone()` only bounds steady retention and still allows the problematic old-cache-plus-new-build peak. An honest lazy API must accept an explicit conservative reservation, or treat an unknown-cost factory as reserving the full configured resource budget. Evict enough entries **before calling the factory**, and retain no extra local references to evicted values while that factory runs. Check the count constraint at the same checkpoint. Coverage replacement removes its old entry before construction too.

For the minimal correct implementation, use full-budget reservation for unknown weight. Optionally retain a separately bounded, lightweight key-to-observed-cost history for **exact unchanged keys** to preserve several known small templates. Such metadata contains no values, Scene objects or callable closures and needs its own count cap. A previous cost is not a guaranteed upper bound for a changed source/configuration or coverage upgrade; those become unknown again. Do not silently advertise arbitrary small-variant cache reuse before a safe reservation exists. Allowing a new unknown build alongside all retained templates would require explicitly weakening the peak contract and is not recommended after this failure.

After construction, count resource weights once without materializing a second rich drawing. Count unique family objects by identity and unique retained point arrays by identity; define in the schema whether point rows include only render control points or also world-vertex arrays. Prefer covering both known coordinate fields while documenting that semantic Python objects and native allocations remain outside this resource metric. Avoid allocating a giant flattened point array to count it. Cache the computed cost with the template, and do not rescan on every hit.

If the actual template exceeds either budget (or retention is disabled), return the constructed drawing directly and transform it for this request. Do not insert it and then call `copy()`. It is now caller-owned and will never be reused as a canonical template. If it fits, commit it to the cache and clone only after admission. A hit must check coverage validity before cloning. Clones remain independent; mutations and semantic owner references must continue to resolve to that returned drawing's own objects.

If a reserved estimate is exceeded, evict further retained entries before cloning; if it exceeds the total budget, bypass without clone. An underestimated reservation can still permit a temporary construction peak; stats must expose this and tests must not call it a hard OS-memory bound. A factory that raises must leave the cache accounting valid and propagate the original error, without retaining the abandoned template.

## Schema-First Portable Tests And Independent Oracle

Create one language-neutral JSON schema and trace fixture before implementation. Policy contains nonnegative integer entry/member/point limits. Values are symbolic IDs with explicit resource costs; operations include request/hit, reserve+factory, configure, release and factory failure. Expected outputs include retained keys in a defined LRU order, both totals, event order, returned value identity, factory/clone counts and bypass status. Reject negative/fractional weights, unknown operation fields and malformed policy. Define zero-cost values and a disabled zero limit explicitly.

Required traces:

1. Two owners request the same key and remain isolated; releasing one leaves the other's cache and all previously returned drawings unchanged.
2. Eight small entries fit; the ninth evicts least recently used; a hit refreshes recency. This deliberately improves the old insertion-order policy and needs explicit expectation.
3. Members and points independently force eviction. Include mixed resource vectors so one scalar combined weight cannot accidentally pass.
4. Two 759230-member symbolic full towers under a 1000000-member budget never coexist retained; a small entry can coexist if both totals fit.
5. Event ordering requires disposal before a new unknown factory starts, and before any clone. Coverage replacement has no old same-key template live in the factory's cache references.
6. Oversized construction returns its original result, retains nothing, and invokes clone zero times. Exact-limit values fit; one above either limit bypasses.
7. Policy shrink/release updates accounting immediately; release is idempotent; invalid configuration leaves the previous cache intact.
8. Factory failure, explicit cancellation and nested context exit release the correct owner without touching pure geometry/order/font caches.
9. Returned drawing mutations never affect retained canonical data or a later clone. Existing real tower transform/semantic-part contracts remain required.

An installed third-party oracle is available without installation. Top-level `require.resolve('lru-cache')` fails because it is not exported at the root, but explicit installed resolution succeeds:

`C:/git/semio/node_modules/.bun/lru-cache@11.5.3/node_modules/lru-cache/dist/commonjs/index.min.js`

Version 10.4.3 also resolves. A light Node smoke using the 11.5.3 `LRUCache` class, `max=1`, `maxSize=5`, and resource size calculation retained only key `b` with calculated size 4 after costs 3 then 4. No package was installed and no third-party runtime dependency is proposed.

For independent two-budget validation, use two synchronized installed `LRUCache` instances, one sized by member count and one by point count, both using the same entry cap and access sequence. After an operation, intersect surviving keys and delete keys missing in the other instance; both have the same recency order, so this preserves the stricter prefix of evictions. Check zero-size/zero-limit semantics explicitly because the third-party package may require positive sizes/limits; disabled retention should be handled as a separate declared trace policy rather than inventing an oracle equality. Use the oracle for actual LRU/count/weight results. Validate lifecycle event ordering and clone/factory identity independently with symbolic callbacks and weak-reference collection tests, rather than claiming lru-cache implements scene ownership. Do not replace two budgets with `max(normalized_member,normalized_point)` or a sum: those are different policies for mixed vectors.

## Proposed Affected Paths

No listed production path has been edited. The smallest clean long-term implementation set is:

- New neutral module root `🧰️framework/🔨️modules/🗃️cache/`: first-party Python implementation, first-party TypeScript implementation for multi-implementation consistency, README, JSON policy/trace schema, shared trace cases, and Python/Node tests kept together. Names and integration metadata should follow existing framework project conventions. Every new full path must be checked below 256 characters.
- `🎓️teaching/🏛️architecture/🏢️nakagin/🎨️visuals/🐍️.py`: remove `_TOWERS`, integrate owner-local context, public configure/stats/release, cost counting, lazy resolution and oversized no-copy return. Keep model/assembly/paint/fragment semantics and all existing physical detail.
- `🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/🎨️visuals/🔣️.json` and `🧪️tests/🎨️visuals/🐍️.py`: portable integration cases and focused ownership/clone/release tests; retain existing complete geometry tests. New neutral cases belong with the cache module, not only here.
- Both `🎓️teaching/🏛️architecture/{📊️excel,🦗️grasshopper}/🎬️clip/🎨️visuals/🐍️.py`: explicit finally-scoped public drawing cache context around presentation, because tests construct scenes directly.
- Their presentation tests/cases: minimal two-language lifecycle assertions that exercise real scene cleanup on normal/exception paths without mirroring cache internals. Documentation should explain public resource metrics/lifetime and their limits.
- Extend renderer `source_files()` to include neutral implementation and contract sources so every subsequent proof baseline consumes them. Recompute the source freeze and rerun all affected accepted gates after any authorized edit; old current-source proofs cannot remain accepted by omission.

## Other Global Caches To Preserve

`_OCCLUSION_ORDERS` is a 24-entry `OrderedDict`. Current lines 1100–1102 restore cached recipes against current `items`; lines 1224–1228 retain the coalesced recipe tuple. `_FragmentRecipe` contains source index, polygon index, tuples of fragment coordinates and normal coordinates; it does **not** retain a rich Manim drawing or current owner/draw closure. `_restore_order` binds current style/owner afresh. The older review's integer-only description is incomplete for the current implementation: fragment coordinate tuples can be substantial even though they are geometry-only. Leave this reuse intact in this scoped change and report separate metrics if later measurements justify another budget.

`_OPAQUE_VISIBILITY` is a 64-entry `OrderedDict` of retained integer indices (lines 841–843, 915–919); it resolves those indices against current items. Keep it. Geometry caches `capsule_mesh(512)`, `_capsule_parts(256)`, `_core_parts(64)`, `_capital_parts(32)`, `_podium_parts(1)` and `_bridge_parts(1)` retain model/mesh part data, not painted Scene templates. `_blend(8192)` retains colour values. `_TEXTS` retains copied text templates and has no current budget; it is a different category, preserved here rather than silently cleared. The existing font/disk configuration and pure model caches remain untouched.

## Required Validation And Remaining Limits

Start with the schema, portable traces and installed-oracle comparison. Then test the neutral implementations, renderer cost counting without giant temporary arrays, exact ownership release, pre-factory eviction and oversized no-copy paths. Finally run focused real geometry/clone checks before rebuilding the complete visual, profile and native evidence on a new freeze. No active root job's declared input may change until root finishes or safely cancels it.

Public cache budgets control retained templates only. Scene-authored clones, Grasshopper's local `_towers`, animation target copies, geometry recipes, renderer temporaries and simultaneous processes can still consume substantial memory. The next native admission plan must therefore serialize rich workers and measure observed peaks. Do not claim the proposed cache alone fixes every memory source, restores an incomplete native pass, or permits unlimited concurrency. All eighteen native pairs, both contract suites, current source/runtime endpoints, gallery review and final ledger remain required.
