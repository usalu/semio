# Inspection Performance Audit

Bounded read-only source assessment of slow ventilation sources12/16. No production code, render settings, caches or processes changed. Reported observed8–20seconds/frame comes from root; this audit establishes call paths rather than CPU attribution.

## Concrete Per-Frame Work

Shared authoring visuals Python: math_readout1981–1989 uses Manim always_redraw with a lambda calling math_label on every updater execution. math_text1926–1931 reparses the formula into custom math nodes. _math_render1810–1923 recursively builds groups, markers and Text probes; each character/run uses body_text at1829/1843, with fresh recursive script/fraction layout. There is no unchanged-string guard at this interface.

The volume scene Beat1_Innenvolumen starts470. It adds livea_read551, ve_read563, a_net_read593, di_read604andv_read642. Onlyve_read explicitly clears updaters569. Therefore previously settled readouts remain active during later animations, including the warm-air hold661. a_read and a_net_read include polygon clipping/shoelace computations; v_read computes net_area twice for its formatted expression. room_fill638 also rebuilds prism geometry every frame. These are concrete repeated costs even when trackers have stopped changing.

Heat-flow Beat2_SensibleWaerme starts432. It adds temperature readouts505, delta514, volume538, air-capacity549andheat562. Their nested units, indices and exponents invoke the same recursive typesetter. Multiple coexisting always_redraw objects multiply Text construction per rendered frame.

The named _Flow animation is at visuals600, wrapping updater585–598. Per particle it queries path position three times, moves/rotates the ellipse, interpolates color and sets opacity. It performs no font registration, compiler invocation or external tool call itself. A progress label naming _Flow therefore does not identify the dominant cost because all scene updaters execute during that animation.

## Font and Cache Evidence

body_text at fonts275 constructs Manim Text eachcall. The typography wrapper192–223 registers requested faces through its existing registration guard, checks fallback coverage, chooses a minimum60point rendering size, temporarily uses at least854 ×480 text canvas, invokes originalText construction, then rescales. Even a480 ×270 review preserves that text work. _font_digest88 is cached once; _font_cmaps100and_font_supports138 are cached. Pango attachment reuse27–41already suppresses redundant Windows attachments. No repeated external compiler command appears in these inspected math/font paths; math rendering is the project customtypesetter, not a LaTeX subprocess. Existing Manim SVG caching may reduce native shaping, but does not remove Python object/SVG construction and custom math layout on each call.

Runtime render command513 always passes --disable_caching for scene partial movies. This is independent from text SVG caches. Runtime343–346 handles localtextcache SVGs; output config12 names text_dir as{media_dir}/t. Final encoding uses -threads1at585; no OMP/OPENBLAS/NUMEXPR controls were found in the inspected runtime/output/inspection launcher text.

## Safe Operational Decision

Keep the current render running and preserve validated source sidecars; restarts lose work and disabling motion/updaters changes output. Lower pixel resolution cannot eliminate custom math reconstruction or the minimum textcanvas. No evidence-based cache flag or threadlimit was found that safely removes this measured updater cost without changing production code. Setting BLAS threadcounts now would only affect newprocesses, and these mostly small geometry calculations do not establish a BLAS oversubscription bottleneck. Do not claim such an environment change as a fix without timing evidence. Bound new auxiliary jobs through repo capacity and avoid duplicating active sources.

A future implementation investigation should measure math_readout and Textconstructor elapsedtime separately from _Flow, then validate unchanged-value reuse against exact visual output; that is outside this render-only task and no speculative rewrite was applied.

## Ticket Template Cache Correctness Review

Read inspection-math-bench.py and shared math_label1975/math_readout1981. The prototype caches unplaced math_text by source, resolved font size and string color, then returns Manim.copy before placement. For the benchmark fixed string-color/font/config environment, the key covers explicit math_text inputs and excludes placement correctly. math_readout still calls callable at every updater, and place_math applies alignment after copying, so unchanged text can move. Font-size resolution matches existing font_size-or-BODY_FONT_SIZE behavior. BoundedLRU capacity limits templatecount.

The benchmark explicitly verifies deep point-memory separation, baseline/bounds, a mutatedcopy followed by a pristine laterrequest, moving callableposition, and Cairo pixelidentity. No concrete correctness defect was found within this intentionally fixed-environment benchmark scope.

This key is insufficient as a general productioncache if font/style/render environment can vary during the process: selected BODY_FONT/registered outlines, kerning behavior and minimum textcanvas/config can change construction without source/size/color changing. Production acceptance must either prove that environment immutable for lifetime or include a complete generationidentity; a fontsize+source cache alone is not evidence of safe runtime invalidation. Current _font_digest is itself processcached, so it is not a dynamic filewatcher. The benchmark str(color) normalization is tested forstring colors only; anywider acceptedManimColor/tuplecontract needs explicit canonicalizationtests before expansion.

Additional validation useful before production adoption: compare everyfamily stroke/fillopacity andwidth, directly verify baseline marker references are notshared with template, and exercise source/size/color changes plus eviction. Existing rasterequality and mutationtests are strong evidence for cases actuallymeasured but do not prove arbitrary environmentvariation. This review was shared with benchmarkowner. No productionedit made.

## Actual Production Cache Review

Read the implemented _math_readout_template1982/_math_readout_text1990/math_readout1999 and fonts.text_context252. No blocking correctness issue found. The32entrycache keys source, resolved size, normalizedRGBA and typographycontext. text_context includes processcapturedassetdigest, activeface/bodydefaults/fallbacks, kerning/minimumcanvas, actualpixel/frame dimensions, renderer and Text constructor/hashidentities. Eachrequest returns a freshcopy before place_math, preservingposition/alignmentisolation. Callablepositions remain reevaluated everyupdate. Ordinarymath_text/math_label are untouched.

The processcaptureddigest is appropriate only with the existing fresh-render-process and source-provenance publicationguards; it is not a hotassetwatcher. Preexisting _MATH_METRICS at1657 remains keyedbysizeonly, so no general hotfontswapguarantee should be inferred. apply_body_font uses Text.set_default and maychange constructoridentity; resultingcachemisses are safe rather than stalehits. Activeproductioninputs used by the current render appear covered. Reviewsharedwithrootandverifier.

## Publisher Stale-Owner Adaptation Review

Read the saved ticket-only inspection-publish.py adaptation after newly stale BIM114 was detected. No blocking stale-certification or duplicate-writer issue found. stale_owners calls production render_segment with concat_only plus cursor validation for unique canonical sources, maps stale ordinals to every owning lesson, and combines those owners with the six optimization lessons. Those native owners invoke ordinary compose_videos; unaffected owners retain concat-only guards. No source metadata is rewritten or stale state certified.

A stale shared clause source explicitly aborts before concurrent composition, while the eight shared excerpts are prepared serially. The parallel selection contains distinct lesson IDs once, and each retry remains sequential within its own worker. Physical seeding precedes the distinct remaining lesson pool. Final full source/chapter/output gates remain mandatory before publish-complete. A newly changing non-native owner after inventory fails closed under concat-only and may need another recovery; it is not silently accepted as current. Review was sent to root and executor before the approved launch. No production edit or duplicate publisher was started by this audit.

## Final Ordinary-Composition Supervisor Review

Reviewed the final saved publisher form: all seventeen IDs now use ordinary no-force compose, letting production guards reuse current sources and refresh stale ones without relying on an owner snapshot to select concat-only. The stale inventory is diagnostic only. Shared clause sources are rendered/trace-validated and excerpts prepared serially before the distinct-video pool. The final117-source/38-chapter/output gate remains required. No blocking stale acceptance or duplicate-writer issue found for the single approved publisher.

Operational note: --check-only returns after normal shared-clause preparation, so it can render and update caches. It must not be treated as a read-only status probe or run beside the active publisher. Root was notified. This audit started no duplicate publisher and made no production edits.

## Browser Dependency Scan Scale

Bounded read-only filesystem counts found the explicit framework UI candidate root has 2,403 directories, 2,435 files and 649 TS/TSX candidates totaling5,530,746bytes; the browser owner has6directories,14files and1TSX candidate14,723bytes. Both scans excluded node_modules, .git and symlinks. These are direct candidate-root counts, not a complete compiler closure: imported CSS/compiler.sources can add roots. Historical optimization delivery sidecar contains 2545 presentation source paths; this is scale evidence only and is not accepted as a current closure. Successful browser attempt directories are deleted by _call, so no current successful source-only response remained in the inspected ui root.

Cold and repeated costs are distinguishable in code, but not quantitatively separated by the available logs. Cold source_files invokes the real compiler/scanner through _call(sources_only=True). Warm source_files still walks membership roots and hashes known source bytes to validate its cache. _handoff_identity then resolves every returned path to exclude the catalog and hashes those files again. render_batch calls _handoff_identity before and after a canonical handoff batch. Therefore repeated validation/stat/hash work exists even after discovery; the observed Path.resolve/stat stack alone does not prove first-cold discovery. No browser/compiler/provisioning was duplicated for this audit.

The latest inspected publish-stacks.log ended in Windows fatal access-violation lines after repeated Path.resolve/stat samples. Root was immediately notified to confirm actual publisher process state; this diagnostic must not be mistaken for successful publication. No production change was proposed or applied from these counts alone.
