# Exact Slab Cache Prototype

Prototype only: slab-cache.py, slab-cache.schema.json, slab-cache-cases.json, and slab-cache-test.py in this ticket. No production source or BIM model is changed. Eleven prototype tests pass; Shapely and jsonschema are test-only oracles.

The key hashes codec revision, typed canonical full model, typed canonical full slab value, and SHA-256 content digests of explicitly declared producer files, named with repo-relative POSIX paths. A caller must pass the entire dependency closure; the prototype does not yet discover or verify completeness. Before integration, construct a reviewed deterministic manifest containing the BIM model module, source files of every function in geometry_producers(), transitive imported numerical/geometry backends, schema and any consulted model or asset files. Function identity, module mtime and a hand-picked slab ID are insufficient. Include the full slab definition even if it is also present in the model. Reject unknown or outside-workspace dependencies. Verify the closure with mutation tests for every declared class of dependency before enabling persistence.

Values use explicitly tagged null, bool and string scalars, arbitrary integers via canonical decimal strings, finite float via hex representation, Fraction numerator/denominator, lists, tuples and dictionaries including tuple/Fraction keys. Other types and malformed records are rejected; this must be compared with the complete actual exact _slab_geometry_entry output before integration. An envelope stores a SHA-256 of the encoded value and key and rejects altered or malformed records and rebuilds them under the lock; this detects corruption but is not an authenticated signature against a party able to rewrite both value and digest.

A per-key OS file lock uses msvcrt on Windows and flock on POSIX and is released by process exit. A waiting operation polls cancellation at 50 ms, emits one waiting progress event, then rechecks the full identity inside the lock before use, after cache-hit decoding, and immediately before atomic publication. The builder receives the cancellation callback and must check it within its existing expensive loops; prototype checks again after the builder and immediately before atomic replacement. It cannot interrupt a noncooperative builder. Values publish by same-directory temporary file plus atomic replace only after full encoding and unchanged-key validation. Concurrent builders of the same key serialize and the later one reuses the complete record. No incomplete record is reused.

Integration sequence for review: (1) certify the complete producer/data dependency closure and prove exact native outputs are fully serializable, (2) install this code under the domain-owned interface and wrap _slab_geometry_entry before its expensive compute while preserving in-process memoization and callback cancellation, (3) run cold/warm real-model exact tuple/Fraction equality, Shapely/SciPy geometry comparisons, concurrent process, cancellation and killed-writer cases, (4) compare performance and disk volume, (5) invalidate all dependent render/source fingerprints honestly and rerender; do not relabel old results. Because the BIM model and/or its producer dependencies would change, existing BIM source clips and presentation outputs must be re-evaluated by the canonical fingerprint system. Active producer processes cannot use or populate the prototype. The cache key itself is not a render freshness certificate.

## Reviewed Dependency Inventory

The live geometry_producers() enumeration resolves 170 callable slots to 9 source files; runtime.source_dependencies extends these to the following 16 repository files. No direct producer callable remains unresolved after inspect.unwrap. The only nonliteral import_module expression in these files is in model/🧮️analysis/🐍️.py: __package__.rpartition(".")[0] + ".🐍️", resolving back to the already listed model/🐍️.py. No repository imports written as regular import/from statements, or __import__/eval/exec calls, were found in this closure. The model source JSON is read by source_text and its complete loaded model is already included in the key; production should additionally include its schema and the final cache-interface/codec source and schema in the revision key. The exchange helper scans its parent Python sources; it is a conservative scanner-discovered dependency, not shown to participate in the tested slab path. This is a reviewed static inventory plus one native variant, not proof that every future model path or external data dependency is covered.

- 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🎨️finishes/🐍️.py
- 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🎨️finishes/🛝️ramps.py
- 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🎨️finishes/🧱️layers/🐍️.py
- 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🏗️facade/🐍️.py
- 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🏗️facade/🔗️junctions.py
- 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🏗️structure/🔗️joints/🐍️.py
- 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🐍️.py
- 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔌️exchange/➖️difference/🐍️.py
- 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔌️exchange/🐍️.py
- 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔌️exchange/📐️frames/🐍️.py
- 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔧️services/🐍️.py
- 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧊️profiles/🐍️.py
- 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧊️solids/➖️difference/🐍️.py
- 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧮️analysis/🐍️.py
- 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧮️analysis/📏️distance/🐍️.py
- 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🪟️curtain/🐍️.py

Callable safety requires a committed reviewed ordered slot manifest, producer source hashes captured by a trusted loader at module load, and rejection of changed bindings, code objects, source bytes, local functions, or closures. No trusted import-time snapshot currently exists in the production BIM API; registering hashes after a file already changed cannot prove which bytes were loaded. Until such provenance is available, persistent eligibility must conservatively fail and perform cold computation. Current native test captures files after import for empirical codec verification only and does not certify that provenance.

Canonical scanner invalidation measurement: applying a change inside the 16-file closure is detected for BIM videos bim_information s114, bim_objects s115, bim_representations s116, bim_models s117, bim_exchange s118, bim_consistency s119. The current literal scanner reported no GH or energy segment intersection; this is scanner behavior and should be checked against any dynamically accessed source before integration.

Proposed integration API: opt-in repository-configured cache directory under the local generated cache; pass explicit cancellation/progress and a reviewed dependency/slot manifest to a domain-owned adapter around _slab_geometry_entry. Default disabled until certification; cache contents are never a render certificate. A full-model cold/warm evidence command must execute the domain's exact native geometry/ownership tests twice with an empty dedicated cache on the first pass, then compare typed results and independent Shapely/SciPy geometry on the second. A production command cannot yet be specified because there is no production cache configuration or adapter; adding one before those proofs would be premature. Existing rendering inputs must be honestly re-fingerprinted and the six BIM sources rebuilt after any production integration.

## Latest Prototype Verification

The codec uses tagged canonical decimal strings for arbitrary integers, including values beyond 2^53; malformed and noncanonical strings are rejected. The actual small native slab test passed after this change, preserving exact cold/warm structural polygons, finish meshes and shared reservation meshes, with independent SciPy and Shapely comparisons. Two concurrent prototype runs both reported success; an isolated rerun is warranted when memory headroom returns to eliminate shared-test-cache interference. This empirical test does not provide trusted import-time source provenance.

## Loaded Module Code Identity Experiment

The ticket-only module-identity-test.py passed its global-constant replacement case: normalized top-level code objects differ when a module constant changes despite identical geometry function bytecode, while repository path differences are normalized recursively. Its test also demonstrates that post-import global mutation changes results without changing that digest. The experiment uses Python marshal and includes the Python version, so it is not a language-neutral persisted encoding and its stability across interpreter implementations is unproven. A safe production design additionally needs a reviewed inventory of mutable globals and other dynamic inputs, immutable snapshots or conservative eligibility checks, and a committed slot/source manifest. This experiment does not justify enabling persistence yet.

## Reviewed Computational Globals Proposal

A committed manifest can enumerate each reachable owned callable and its semantic global reads, including qualified attributes read from owned modules. Resolve and encode the current values in the same typed key as the full model, loaded module code, Python implementation/version and file hashes; re-resolve and compare before reuse and publication. Explicitly review exclusions for memo tables, synchronization objects and progress-only counters. A reassigned integer constant then changes the key; in-place mutable changes must also change its encoded value. Callable replacement and module attribute rebinding must fail eligibility or change a separately certified identity.

This is fail-closed only if its closure is complete: recursively examine global bytecode reads of all reachable owned functions, defaults, keyword defaults and relevant class descriptors, and reject unreviewed dynamic getattr, computed imports, reflection, mutation, non-owned callbacks, nonserializable values and unknown calls unless a separately reviewed adapter declares their complete dependencies. Include builtins and library implementation/version where semantic. Static bytecode alone cannot prove indirect or native-call behavior; enforce a conservative whitelist rather than assume purity. Rechecking before publication detects ordinary mutation but cannot eliminate concurrent mutation and restoration (ABA), or a mutation while computation reads a value. True correctness under concurrent mutation requires owned synchronization or immutable snapshots actually used by the producer. A manifest and fresh-value key alone therefore cannot certify arbitrary Python execution. No production integration is authorized by this proposal.

## Proposed Supported BIM API Contract

Default-enable the domain cache for canonical owned producers only. At the public BIM query boundary privately snapshot the complete model and use that same snapshot in both key and computation. Register each reviewed module top-level code identity at execution; identify Python implementation/version and reviewed repo-relative producer/source/schema/codec digests, and encode the explicitly reviewed computational-global values. Unknown or replaced producer bindings, changed code objects, closures, unsupported globals or unrecognized producer sets take the ordinary cold path. Preserve existing progress and cancellation callbacks. Put cache records under the repo-owned local generated cache, with per-key cross-process OS locking, cancellation-aware wait and atomic completed writes.

Inside the lock recheck source digests, module/producer identities and computational globals before a hit, after decode, and after computation immediately before atomic replacement; normal external file edits invalidate the operation rather than publish a mismatched result. Concurrent supported model queries work on their own private snapshots and serialize identical keys. Private module-global mutation during a running call is outside the supported BIM API contract; single-thread replacement before a call must change the key or disable persistence. Before enabling this, enumerate the actual semantic global closure, add a trusted top-level registration to owned modules, port the tested interface into the domain, and prove small plus full-model cold/warm equality, model mutation isolation, independent geometry oracles, two-process coordination, cancellation, source-edit invalidation and noncanonical producer fallback. Changes to owned producer files honestly invalidate the six measured BIM source renders.

## Implemented Domain Integration

The domain-owned ♻️cache/🐍️.py, 🔣️.schema.json and 🔣️.manifest.json implement the tested exact codec and per-key process lock. The manifest declares the 16 module files and 170 canonical producer slots. Each source module captures its top-level executed code object; supported producer functions must match both their declared binding and that code. The cache key incorporates Python implementation and full version, OS/release/machine, normalized executed module code, current computational globals, full private model and slab, and source/schema/manifest hashes. Unknown producer/module/global cases fall back to ordinary cold geometry. Existing progress and cancellation flow through the cache and geometry builder. The cache directory can be selected with SEMIO_BIM_SLAB_CACHE; otherwise the local generated repository cache is used.

A real small native slab cold/warm equality test passed on render Python 3.12 (81 seconds), with independent Shapely structural volume and SciPy shared-reservation volume. The 11 production codec/lock tests passed, including two-process contention, cancellation, corruption and input mutation; producer/global change and noncanonical fallback tests passed 2/2. The canonical schema fixture and Shapely test were registered in the authoring test list. Canonical segment dependency discovery includes the implementation and its JSON schema/manifest. The 16-source class inventory found only a geometry exception, a dict subclass with slot names, and two IFC writer classes; none declares mutable computational class data.

Full-model validation is pending. full-slab-validation.py enumerates all 10 actual slabs, requires an empty dedicated ticket-generated cache, and checks warm cache hits plus typed cold/warm equality and hashes. It uses the same 3.12 interpreter as rendering. Subsequent rendering must inherit SEMIO_BIM_SLAB_CACHE pointing to that directory to reuse the validated artifacts. No full-model result or refreshed BIM video has yet been claimed.

Changed files: 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🐍️.py and all 15 other source files in the reviewed inventory above (module identity registration); added 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/♻️cache/🐍️.py, 🔣️.schema.json, 🔣️.manifest.json, 🧪️tests/🐍️.py and 🧪️tests/🔣️.cases.json; updated 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts. Ticket inputs added: bim-globals-inventory.py, bim-globals-values.py, bim-classes-inventory.py, module-identity-test.py, production-cache-test.py, production-context-test.py, production-slab-test.py, cache-context-check.py, cache-source-audit.py and full-slab-validation.py.

Freeze v2 review corrected the global key to preserve list versus tuple and set versus frozenset distinctions. The render-Python native cold/warm oracle test passed again in 95 seconds, and context/type/fallback tests passed 3/3. All 19 source/schema/manifest hashes matched the pretest snapshot. The earlier full-model run published SL-1 but rejected its warm read when inputs changed; no result from that run is accepted. The next run uses the fresh ticket-generated full-slab-v2 folder and must cover all 10 slabs.

## BIM Restart After Full Validation

The cache path passed a registered repo→uv→render ToolSession→Python child probe and a separate repo→bun probe; the retained input is cache-env-probe.py. After all ten full-model validations succeed and the coordinator authorizes restart, the PowerShell invocation from C:\git\semio is:

```powershell
$env:SEMIO_BIM_SLAB_CACHE = Join-Path (Get-Location) '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️03/FINISH-ARCHITECTURE-COURSE-VIDEOS/🗑️generated/full-slab-v2'
repo tools exec bun -- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --video bim_information --video bim_objects --video bim_representations --video bim_models --video bim_exchange --video bim_consistency --language en --audience speaker --quality l --fps 15 --jobs 2 --no-play --log-output
```

No BIM producer was started by this probe. The full validation owns the cache directory until it exits; source fingerprints remain authoritative.

The coordinator authorized cancellation of the stale pre-change authoring suite: only PID 33432 was stopped after verifying its unittest command, parent PID 68548 and bun authoring test ancestor PID 39552. Its partial output is not a passing current suite. A fresh canonical suite remains required after the complete full-model cache validation.

At 08:30 UTC, a read-only SL-R validation stack sample showed reservation owner index 860 (CHIMNEY-SALON-ROOF), cutter index 36 and region index 78 (R-PRIMARY-SL-R-79). This is an instantaneous observation, not an estimated completion percentage.

## Full Native Roof Validation

Full-model SL-R passed typed cold/warm equality and an actual warm hit on the frozen v2 sources. SHA-256 of typed encoded geometry: 6ff2fe59a66135c9ac39ea943291b4e32862505508f7a48fca14bd6673d976c4. Cold calculation:9181.953seconds; warm calculation:45.828seconds. SL-1 had already passed; validator now computes SL-KIOSK. These two successes do not replace the remaining eight-slab gate.

SL-KIOSK full-model validation passed: typed cold/warm equality and warm hit1, geometry hash6160e1dbb2fe99375e3fd3815e79b86a92f48d29c05a0bd25ce2e3dba81ebb65. Cold852.547seconds;warm29.922seconds. Three of ten slabs have passed; no all-model claim yet.

The admittedBIM80246native114/115workers4672/63528were inspectedread-only: both inherit the exact validated SEMIO_BIM_SLAB_CACHE path and are CPU-active near1.25GiBworking set. Environment output was restricted to the cache variable. This confirms propagation in actual production, not completion or cache-hit proof by itself.

After all ten cold/warm tests passed, read-only samples of s114 and s115 producers showed curtain filling work and canonical slab context/model hashing respectively, not SL-R finish reservation computation. The samples do not independently prove a cache hit because the building query supplies no progress callback.
