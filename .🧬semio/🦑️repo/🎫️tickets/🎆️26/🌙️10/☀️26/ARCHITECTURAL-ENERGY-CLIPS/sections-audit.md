# Architectural Section Audit

## Scope

Read-only review of unique script references from the energy video manifest plus shared visuals. Requirement supersedes earlier approval of translucent outlined sections: base cut material must be opaque, one color, without stroke seams or hatching. Scene consumers and shared material are being repaired by their assigned owners.

## Initial Findings Sent to Owners

- Shared visuals house_section lines 1060, 1062-1063, 1068: explicit white polygon material retained opacity 0.7 and stroke 1.3.
- Heating conduction _envelope_house lines 289-291: replacement right-wall polygon retained opacity 0.7 and stroke 1.3. Lines 297-299 recolored roof, walls and floor independently.
- Heating solar-gains lines 865-920: lintel/sill masonry hatch.
- Heating calculation line 217: floor recolored with teal ground.
- Physics fundamentals lines 993-994: supplementary intermediate slab opacity 0.5 with stroke 2.
- Physics fundamentals lines 1556/1570, 2282-2284/2380-2382, 2631-2632, 2708-2709, 2796/2799: direct structural material opacity changes and resets. Heat tint is semantically valid but must retain opaque structural material or remain a distinct overlay.
- Physics fundamentals line 3152: cold-surface exterior wall material opacity 0.16 and stroke 2.
- Heating fundamentals _brick_wall lines 113 onward: actual room exterior cut with mortar and layer seams; consumer owner confirms coverage.

## Exclusions

Glazing, pane/sash outlines, air temperature tint, thermal paths, analysis highlights, furniture and equipment remain distinct from base cut material. Soil datum hatches are exempt: shared house_section lines 1044-1048, ventilation volume lines 256-258, demand system-losses lines 507-509, primary-energy _ground, heating fundamentals _ground. Independent explanatory material diagrams remain outside base building simplification: physics concrete cube around line 2490, conduction balcony bridge around line 1364, and heating fundamentals resistance wall-layer diagram around line 1389.

## Verification Status

Initial bounded source audit complete. Findings delivered directly to both owners. Post-edit source checks and fresh rendered preview audit pending.

## Additional Confirmed Consumer Findings

- Demand regulations _north_window line 358: replacement wall style stroke 1.3 and opacity 0.7.
- Demand system-losses _plant line 499: cellar wall style stroke 2 and opacity 0.14.
- Demand practice _room_row line 633: room row slabs and walls stroke 2 and opacity 0.14.
- Ventilation natural _vent_room lines 223 and 229: opaque structural fill but stroke 2; lintel additionally orange rather than common base color.

All additional findings delivered to consumer owner.

## Post-Edit Source Disposition

Rechecked all unique manifest source references after consumer patches using bounded structural construction and mutation searches. Previously reported local cut construction now delegates to section_material. Solar masonry hatching removed; heating brick stage now one uniform core with distinct thermal tint. Physics direct ceiling/floor/wall opacity mutations now affect separate copied heat overlays; base material persists. Conduction envelope analysis colors now belong to copied overlays, and calculation floor no longer inherits soil color. Confirmed demand cellar, room-row, north-window replacements, natural room lintel and volume cut polygons use common material. Remaining hatch references are soil datum; remaining translucent structural copies are explicit thermal analysis overlays.

Cooling transmission-humidity direct envelope recolor was additionally flagged during this pass and sent to the consumer owner; the current helper still references original wall and roof objects, so this finding remains open pending owner repair. No other confirmed scene-local base style defects found. Explanatory material diagrams remain exempt as documented.

Source review does not accept raster joins. Shared section_assembly work and fresh image audit remain pending.

## Fresh Preview Acceptance

Inspected all six unique 1600 by 900 production-source frames under generated/sections-preview/291cef86846f. PASS: plant-house and shared house cut material is uniformly opaque cream; roof, wall, floor and chimney joins show no interior strokes or raster seams. Both mirrored room shells are uninterrupted solid cream around declared apertures, with no outline or corner seam. Closed, open and closed-again frames preserve material geometry. Casement and tilt panes move through apertures and return; cyan glazing outlines and equipment details remain intentionally distinct. No concrete preview defect found. Acceptance applies to these six frames rather than every video timeline.

SHA-256 fingerprints:

- UniformSectionOverview-closed.png | 7fcd81ef254cc2f7a1916500a82d48ce3aef072e8fea22be394a94ebcc27d779
- UniformSectionOverview-open.png | 41b8a4cb086221767bc7711e80f0c83ac6626537f05cd864f3c0377e3be37207
- UniformSectionOverview-closed-again.png | 7fcd81ef254cc2f7a1916500a82d48ce3aef072e8fea22be394a94ebcc27d779
- UniformRoomOpenings-closed.png | b96cd5d7a44cbfadd0f7c51de4f2fed550299a87d6e63cd4e5cdf780a10077fd
- UniformRoomOpenings-open.png | ea81b4f2d2e784320fff829624ef38df4d98d508a7635490cb42a1a2ee0f0ac4
- UniformRoomOpenings-closed-again.png | b96cd5d7a44cbfadd0f7c51de4f2fed550299a87d6e63cd4e5cdf780a10077fd

Cooling alias recheck PASS: transmission-humidity line 133 constructs a copied envelope heat overlay, adds it separately at 134, and lines 281/456 animate only its translucent heat fill. Original cut material is preserved. All previously open source findings are resolved. Matching SHA-256 values confirm both closed-again frames exactly restore their respective closed frame.

## Actual-Movie Intermediate Acceptance

The initial six PNG acceptance established endpoint appearance only. Independently extracted and inspected actual movie frames at 0.3, 0.5, 0.7, 1.3, 1.5 and 1.7 seconds from each current 19d5d284f42a preview, using retained sections-motion-audit.py. Both generated motion contact sheets are under that preview root. PASS across all twelve intermediate frames: uniform cream base material stays present, no internal join seam or outline reappears, apertures remain open, shared house casements visibly progress through opening and closing, and mirrored room casement/tilt leaves progressively move and return while shells stay intact. Equipment, glazing and people remain correctly distinct. No concrete motion defect found.

Movie identities are the exact SHA-256 entries retained in sections-media.md: UniformRoomOpenings.mp4 3246c83bfd6b7ba944b12cf04346597a65edf85cc936b6dc9a270f2feb351a79; UniformSectionOverview.mp4 164ba34eed61156e4a562cbccc26add232d6f5c9e34e4e6547d23bb8690367fa. This extends the endpoint acceptance to sampled actual movement in both directions; canonical suite status is tracked separately.

## Final Consumer Assembly/Index Audit

Boundedly searched every unique manifest source for positional group/shell/material indexing and direct architectural constituent drawing. No house group or room shell positional index assumes the former assembly child layout. Remaining group indices belong to thermometers, dial ticks and gauges. Named roof/wall references and local plant walls index references remain intentional. Heating stage draws construction (line 198), which includes material and thermal tint (176-177); raw shell is retained as an unused geometry handle. Plant reveal draws material (1418), then chimney and equipment (1428); its separately revealed chimney was already accepted in endpoint and actual-motion previews. Solar masonry/room reveal draws the complete section assembly (926-927), natural room group includes its assembly (241-247), practice room-row shell is the assembly (640), and volume material is assembled before grouping (267). No confirmed assembly omission or stale child-order dependency found.

## Final Current Profile Acceptance

Fresh current-source identity 62efe597fff3 supersedes the prior profile previews. Independently viewed all six current endpoint PNGs and extracted/viewed six intermediate frames from each current actual movie at 0.3, 0.5, 0.7, 1.3, 1.5 and 1.7 seconds. PASS: solid opaque one-color cream material, no outline or internal join seam, preserved apertures and progressive casement/tilt motion in both directions. The refined silhouettes remain separate from material. No concrete visual defect found in these eighteen reviewed current-source images. Current contact sheets remain under the same generated identity; sections-motion-audit.py now targets this current identity.

Current movie SHA-256 values (also retained in sections-media.md):

- UniformRoomOpenings.mp4: b5e7b9c88b25edfd5792816cbe03fb57851f58a289adcfb1aac11517788fbe2e
- UniformSectionOverview.mp4: 94622e1414b1abfae569a83c90346ec8020f1ed00dbef8c728f892f1361972ec
