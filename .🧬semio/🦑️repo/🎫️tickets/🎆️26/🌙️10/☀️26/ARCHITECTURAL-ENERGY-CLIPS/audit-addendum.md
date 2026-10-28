# Independent Audit Addendum

## P1: Old Seated Limb Indexing Prevents Three Chapters From Rendering

The new shared seated_person_glyph returns igure as one closed VMobject rather than the former group of limb primitives (🏗️authoring/🎨️visuals/🐍️.py:1196–1209). Three consumers still access igure[7]: cooling internal gains 🐍️.py:278–279 in Beat2_HumanFactor, cooling internal gains 🐍️.py:968 in Beat5_Mitigation, and heating internal gains 🐍️.py:466 in Beat3_GeraetePhiE. VMobject has no eighth child, so these constructs raise IndexError before creating their desks. Replace indexed geometry dependencies with an explicit semantic desk/elbow anchor. Add a regression instantiating the three desk setups against the new shared figure.

All paths above are below 🎓️teaching/🏛️architecture/⚡️energy.

## P2: Standing Head And Mouth Still Resolve To Whole-Body Bounds

Physics fundamentals 🐍️.py:3150–3153 derives the head from person[0].get_center() and mouth from person[0].get_right(). The shared person now contains a single body silhouette, so these points refer to its torso-level center and rightmost arm/leg bounds. The latent-heat demonstration exhales from the body side rather than the face, and the convection path starts inside the upper body. Use explicit standing head/mouth anchors normalized to silhouette height and transformed with the figure. Verify emission origins against the head contour.

## P2: Revised Cache Dependency Discovery Drops Framework Style Inputs

The concurrent cache fix correctly begins including the shared visuals JSON. However, inspected runtime segment_state at 🏗️authoring/🎬️clip/⚙️runtime/🐍️.py:328–333 replaces the previous dependency inventory without retaining 	heme.source_files(). Literal teaching-module discovery cannot reach the framework JSON and fonts that theme loads from external paths. 🏗️authoring/🎨️theme/🐍️.py:10,17,45,57–59 explicitly enumerates the styling tokens, font catalog, and checked-in font files. Token-only or font-only edits can now reuse stale clips. Preserve this explicit dependency inventory alongside the new source discovery.

## Additional Coverage

Inspected the five lesson composition wrappers and the three affected chapter composition entrypoints. They delegate to the curriculum manifest; removed thermal-mass duplicate classes are absent from the heating solar and cooling transmission chapter modules. The repeated system-loss chapter takes its canonical source from earlier heating. No additional reversed source chronology or duplicate explanation in these supported composition entrypoints was identified. Remaining ordinary section builders still use their authored beat lists; no additional replaced explanation classes appeared in those lists.

The parent owns smoke renders and scale fixes. This audit did not duplicate rendering, modify application code, or broaden traversal outside the energy teaching subtree. Audio-source integration remained unfinished at the last source inspection; final recheck is pending.

## Source Recheck After Corrections

All three old seated limb indices are removed: heating internal gains now uses lap plus height, and cooling internal gains uses the semantic lap anchor through a room-scaled helper. Physics face emissions now use top-relative coordinates normalized by person height. Physics canonical summer/winter now creates a shared window in its aperture, hides the old rectangle and uses projected open/close animations. These findings are source-resolved.

The ready runtime at lines 324–335 includes the shared geometry contract through adjacent asset discovery and restores theme.source_files, covering framework tokens and fonts. Source narration at lines 338–356 recognizes existing zero-based clause recordings, preserves silence when all are absent, and rejects partial recordings. Lines 359–377 align available recordings using measured marks and publish the muxed source atomically; lines 420–425 attach audio before provenance publication and recheck authored dependencies. Clause numbering matches the existing synthesizer. The mixer explicitly produces mono audio, so the decoded sample windows in the integration test follow one sample clock. No remaining source-level cache/audio issue was identified in this recheck. Independent real-render integration test results and final representative visual review remain to be supplied by the owning agents.

## Available-Audio Integration Verification

The reuse owner reports the final managed clip suite passed 30 tests in 30.4 seconds with one existing symlink skip. The real source fixture declares NARRATION and supplies three independent clause WAV recordings; decoded excerpts preserve the expected reordered tone frequencies across 15 real rendered frames. Cache regressions for framework style/font inputs, shared profile, imported scene sources and audio assets passed. Actual authored corpus contains zero audio files, so authored silence remains deliberate. This source audit independently checked the relevant code; the executed test result is attributed to the reuse owner.

## Final Primitive And Ventilation Visual Review

Independently viewed architecture-primitives.png and ventilation-preview/images/VentilationPreview_ManimCE_v0.22.0.png from the ticket generated directory. Shared house roof/wall joints are closed, structural bands remain legible, standing and seated people form connected recognizable silhouettes, and the panel radiator is a coherent filled object. The room-scale ventilation occupant stands on the slab and fits beneath the ceiling. Both projected opening types remain distinguishable: broad casement and narrow bottom-hinged sash. Recessed apertures and opposed window openings read coherently in the section views. No new actionable visual defect was identified in these final requested contact sheets. The overview image currently present was also viewed, but coordinator indicates a refreshed overview is pending; final overview acceptance waits for that refresh.

## Overview Review

Independently re-viewed ArchitecturalOverview_ManimCE_v0.22.0.png after render 27529 completed. Confirmed coordinator-known pending corrections: displaced dynamic tank fill, faint plant section material, open flue outline and oversized plant equipment. Beyond those assigned corrections, no additional actionable defect was identified. People are floor anchored; emitter/person/storey ratios read plausibly; recessed windows occupy wall apertures; emitter pipes connect; the shared right-hand house roof/wall joints and intermediate slab remain coherent. This review does not mark the coordinator-known corrections complete before their refreshed output is inspected.

## Refreshed Overview 72464: Plant Accepted, Shared Comparison House Missing

The refreshed overview independently confirms the assigned plant corrections: dynamic yellow fill remains inside storage frame, solid walls/slabs, closed continuous flue, smaller equipment and connected pipe endpoints. However, the right-hand comparison house loses its complete roof, walls, slab, floor and windows in the final frame, leaving its ground line and independent person/radiator. This is a new actionable visual regression. overview-preview.py:27–36 still creates and adds the complete shared house then animates its window sashes, while 🏗️authoring/🎨️visuals/🐍️.py:1081 still returns all house material in the group. Investigate final-frame scene/static-family handling and verify the complete shared section remains visible throughout window opening before accepting this overview. Reported to coordinator immediately; root cause and correction pending.

## Snapshot-Anchor Consumer Audit

Bounded energy-source scan found no additional concrete house-group snapshot-anchor defect. Physics Beat1 transforms its house group at fundamentals line 764, then uses only the live wall_right center at line 765; no later numeric house/window metadata is consumed. Cooling internal gains transforms only the extracted facade group (line 205), then replaces it with a separately constructed room rather than using facade snapshots. Primary-energy line 1329 shifts the house together with its interior and envelope; subsequent use is direct object drawing/highlighting, with no stale numeric section anchors. Ventilation volume line 863 transforms the complete house presentation, then morphs it to a scale dot; its later airflow paths belong to distinct glyphs. Heating fundamentals stage scaling exposes explicit p/fit coordinate-transform helpers at lines 197–207. Adjacent volume clock updater uses live face center/radius at lines 325–331, so its group relocation is also coherent.

## Final Overview Acceptance And Transient Display Resolution

Final visual acceptance passed. Repeated same-path image viewing continued to display the earlier missing-house image even after coordinator and reuse-owner controls confirmed complete rendering. To disambiguate file contents from display caching, this auditor copied the exact refreshed PNG to the new generated filename overview/images/audit-final-overview.png through the repo command interface. Its SHA-256 was 03d6d96b167a5773ad62e74b669bd7ce5588bfe3b31b6365ea10dd0acf17fc5f and size 44,578 bytes. Viewing that unique path independently showed the complete right-hand house including walls, roof, floor, intermediate slab, recessed windows, occupant and radiator. The corrected plant shows enclosed tank fill, solid section material, a continuous closed flue, coherent equipment dimensions and touching pipe endpoints. No remaining actionable visual issue was identified.

The earlier disappearance observation is withdrawn as an unresolved code defect: it was a transient/stale artifact or image-display observation, not a reproducible final source defect. Reuse-owner cached/frozen-fill/uncached controls retained 42 of 42 house family members, and fresh/live raster controls retained the house material. No speculative shared implementation change was made. A unique-path image was necessary to independently verify the final bytes because the same-path image tool result remained stale in this audit session.

All audit-origin source findings are corrected or explicitly scope-resolved. Final independent architecture, ventilation and overview visual checks pass. Full five-lesson real rendering remains the coordinator-owned final integration check.
