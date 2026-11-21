# BREP Engine Source Boundary Audit

Direct engine module sources total approximately 287,683 bytes plus 7,793-byte root (295,476), excluding test sources and already separate shape-value, kernel-contract and mesh-io crates. This is an estimate, not the authoritative cfg-selected scanner count; against 262,144 it suggests at least 33,332 bytes must leave. Shape-value (11,778 bytes) and kernel-contract (17,112 bytes) already have separate manifests and the engine imports/reexports them, so cutting them again does not reduce the engine closure.

Operation-jobs is 40,104 bytes, but not a leaf: its private Extraction uses engine Entity and Brep state, planner resolves Brep handles and body, and commit absorbs scratch and registers handles. Rust forbids moving inherent impl Brep to an independent crate. A safe refactor would separate pure job descriptor/types and scratch planning through owned input/output types while leaving handle resolution and commit in the engine; rough prefix 11,448 bytes plus the planner 15,607 bytes gives an upper bound of ~27 KB before unavoidable adapter code, insufficient alone. Do not point another manifest at the same implementation or create a cyclic dependency.

Solid publication has ~19 KB before its first impl Brep but even that prefix checks Brep identity and refers to private certified boolean helpers; polygon wire (~8 KB), conic (~6 KB), polygon face (~6 KB), and prism (~4 KB) prefixes similarly need interface work. Extracting native session/owner state with a clean dependency direction can free enough space, provided the engine facade retains inherent methods and existing publication receipts and cancellation semantics. Source byte movement is only an upper bound: actual admitted closure and feature/config selection must be remeasured with the repository scanner after the change. Concurrent peers are editing this area; re-read source before each edit. No edits, builds or tests were performed in this audit.

## Applied crate split

The new native-registry crate owns the live index and arenas, handle conversions, provenance, and budgeted analytic, polygon, conic, prism, solid and certified Boolean publication. The engine keeps session entry points and delegates bounded publication to that same registry. Its retirement implementation returns the registry's existing field cursor directly, preserving the original cursor shape and grant depth. The workspace and all 23 dependent Cargo lockfiles record the new first-party crate.

A recursive count of production Rust module declarations measured 187,953 bytes across 17 files for the engine and 167,011 bytes across 13 files for the native registry. The repository Cargo ingress admitted sources and moved on to host compiler capacity. A scoped pinned-nightly metadata build separately rebuilt the changed B-Rep dependency graph and accepted the new registry and engine. The metadata build exposed a concurrent bore-sampler type-inference failure, resolved by annotating the two boundary-coordinate inputs as f64. The ordinary arc path shares the same exact recipe as its retained counterpart, and wire length now invokes the existing B-Rep analysis crate directly.

Subsequent live native checks exposed ambiguity in the new wire-preview vertex helper, unsupported scalar division in circular revolution, and missing fixed-size fit traits and index type in the grid-fit publication. These compilation corrections are recorded in the changed-file list. Three nested engine unit modules now address their test-only arena and registry state through the new session field.

## Native test results

The locked, offline B-Rep engine unit run on the pinned nightly compiled both crates and reported 11 passed, 2 failed and 1 already ignored. The focused rerun of the same executable reported 11 passed, 0 failed, 1 ignored and 2 filtered. It exercised working-copy extraction and absorption, retention admission, handle disposal, shape exchange, scoped operations and shape validation.

The full run initially failed in two independent paths. The retained-row fixture called the job from an authoring wallet without the original producer invocation. The same fixture construction and invocation guard exist at HEAD. The fixture now uses the existing host retirement pump to lend a finite original invocation and preserve its exact retained grants. The open-wire extrusion fixture expected InvalidInput, while the existing conversion from kernel to session errors yields Operation. The same open-wire operation refusal and conversion exist at HEAD. The test now asserts the existing typed refusal and its closed-boundary reason. The comparison used read-only source inspection.

After the new circular revolution and grid-fit modules landed, a fresh production-source traversal measured 189,058 bytes across 17 engine files and 186,055 bytes across 15 native-registry files. Procedural passed the native browser source admission stage with the split.

The repaired scoped suite finished with 13 passed and 0 failed. One mesh-import test retains its existing ignore because the cleanup cursor exceeds that test's 4,096-step limit. Both repaired fixtures passed. Concurrent planar-thickening and Coons grid additions also required a corrected engine re-export and the fixed-size Coons retirement leaf traits. The final measured production closures at task handoff were 191,088 bytes across 17 engine files and 202,340 bytes across 17 native-registry files.

## Spline constructor boundary

Further native analytic features reached the registry's 4,000-active-line source limit during Demonstrator and CAD admission. The registry now delegates bounded arc construction, curve interpolation, tensor-product grid fitting and Coons control construction to the separate spline-construction crate. Handle labels, ownership checks, funded publication and the field-wise retirement entry point stay with the owning registry. The extracted arc retains the same staged knot, control and weight owners and transfers them to the completed curve at the final funded turn. All 23 dependent locks include the new first-party package. The Generator lane had already supplied the analysis dependency and fixed the copyable curve-fit state during the same live edits.

The repository Rust closure scanner reported 3,705 active lines, 19 files and 196,361 bytes for the registry and 796 active lines, five files and 37,446 bytes for spline construction. Both units passed actual guarded Cargo source admission under the unchanged limits of 4,000 active lines, 32 files and 262,144 bytes per package. The registry has 295 active lines of headroom at this source generation. No module build was started from this lane.

The locked pinned-nightly native engine suite passed 14, failed zero and retained one existing mesh-import ignore. A language-neutral rational arc fixture drove the extracted construction under retained grants, checked visibility before publication and compared the sampled curve to independent Parry distance calculations.


## Native shape exchange boundary

The 2026-11-20 real wasm32-wasip2 release graph measured the engine at 4,008 active lines, 23 files, and 239,042 source bytes. The standalone shape value crate now owns the existing registry exchange routines. Shape capture keeps the same exact root closure, dense arena mapping and persistent label high water. Shape registration keeps the existing validation-before-merge, label shift and per-kind minting order. The three engine entry points delegate through the current native registry and keep their names and signatures. The exchange moved as ordinary functions. All 23 affected lockfiles already named the native registry and now record the shape value dependency edge.

The same repository source-closure scanner and actual retained Demonstrator release configuration measured 3,934 active lines across 23 files and 233,261 source bytes in the engine. It measured 339 active lines across three files and 21,585 bytes in shape value. Both fit the unchanged 4,000 line, 32 file, and 262,144 byte limits. Locked offline Cargo metadata resolved the existing native registry dependency in the real wasm32-wasip2 hub graph. No module build was started for this extraction.

The pinned, locked native engine test compiled the new boundary and passed the shape exchange law for malformed roots, missing curves, and stale labels without mutating the target. The full linked native engine suite then passed eight tests with one existing mesh-import cleanup-limit ignore. The source scanner evidence is in the ticket’s generated brep-shape-source-count log, package resolution in brep-shape-exchange-locked-resolution, and native results in brep-shape-exchange-native and brep-shape-exchange-native-laws.


## Cuboid and section spline storage boundary

Demonstrator wt0aAnK refused the native solid-transaction closure at 4,028 active lines. The existing spline-construction crate now owns the cuboid degree-one spline recipe and bounded quadratic section-arc storage previously declared inside solid-transaction. Solid topology and conic face publication still decide when to allocate, publish, restore, and retire each exact spline payload. The public section-arc type keeps its previous entry point. The change adds no new package and records the two existing-package edges in affected Cargo manifests and 23 lockfiles.

The real wt0aAnK wasm32-wasip2 source configuration measured solid transaction at 3,737 active lines across 30 files and 193,195 bytes, with 263 active lines of margin. Spline construction measured 1,157 lines in seven files and 53,298 bytes. Offline locked Cargo metadata resolved both edges in the real hub release graph. The neutral cuboid-edge and rational section-arc coordinate fixture also agreed with SQLite affine and weighted Bernstein calculations.

The focused pinned native spline test passed both funded storage laws. It checked receipt bytes as well as the degree-one and rational geometry samples, and checked that round-tripping the arc restores the allocation owner and releases each array. The combined pinned native engine, solid-transaction, and spline-construction command subsequently passed ten tests, with the existing engine mesh-close ignore. The initial combined build found a separate live MeshIntersectionKernel::create Sized-bound error, fixed by the mesh owner before the successful rerun. The evidence is under the generated brep-solid-spline source count, locked resolution, native green, focused test, and oracle logs.


## Annular revolution compile gate

The wsWhso3 native build’s WASI compiler rejected the annular closed-end distance closure because the subtract-and-hypot expressions did not constrain its parameter before method resolution. The parameter now explicitly uses f64, matching the frame coordinates, radial distance and both call sites. The measured wsWhso3 release source closure passes at 3,838 active lines in 26 files and 185,301 bytes. No geometry or limit changed.

The offline locked pinned-nightly check compiled the actual representation crate under wasm32-wasip2 and exited successfully. The repository capacity gate admitted the check after its swap wait. The generated source scan and compile logs are brep-annular-wasip2-source and brep-annular-wasip2.


## Completed native scratch retirement

At 17:57:44 the completed ticket-local B-Rep Cargo scratch under generated b-shape held 804,966,400 allocated bytes. Its only children were Cargo’s generated target and build directories. Kernel handle enumeration, process commands, and authored ticket watcher inputs found no consumer. The authorized removal touched only that scratch directory, preserving tests, raw evidence logs, live images, and maintenance inputs. Host available bytes sampled immediately before and after were 13,585,592,320 and 14,390,480,896. Their observed difference was 804,888,576 bytes while unrelated processes could write concurrently. The measured record is in the generated brep-test-scratch-retirement log.


## Box section retry compile check

The wsWhso3 retry caught an immutable plane capture lasting across the mutable edge draft record. The live shared box-section source already copied the plane origin and normal before constructing the signed-distance closure. The actual current representation crate then passed pinned wasm32-wasip2 metadata compilation against the exact target and feature-matched externs retained by that retry, producing 1,199,219 metadata bytes. The compiler log is in generated brep-box-section-wasip2. The source was verified in place without a duplicate edit.
