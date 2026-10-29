# Consumer Runtime-Contract Audit R6

Read-only re-audit of all six chapter source files after R5 repairs. Source and canonical JSON inspected through direct Node filesystem API. No source changed, no Manim runtime available. Prior resolved R5 findings intentionally excluded.

## Fatal Chapter Failures

1. Information _wall_detail line462 still reads curtain_wall.path after provider changed to disconnected paths. This causes KeyError. Exchange _bay line291 repeats the obsolete lookup and must iterate disconnected paths without reconnecting entrance gap.

2. Exchange _beat_slab lines799-800 defines three tones and indexes by every first-floor room. Frozen canonical model has eleven first-floor spaces. tones[i] raises IndexError on fourth room, also repeated in labels line810 and wires line814. Use a deliberate limited room selection or scalable color mapping and fitting layout. Raw outlines filtering at797 is also unnecessary when resolved geometry outlines are available.

## Misleading Geometry and Quantities

3. Information _wall_detail lines454-485 still asserts and draws glass radius derived from car arrival circle, including Rglass=Rturn-bcar-c. Arrival is now a separate forecourt path. The inset uses arrival curve_centre but glass geometry and fixed view centre (6,14.3), manually forces first point y=11.7, and samples angles90..180 for a route now turning below the building. This can place geometry outside inset and claims a false spatial derivation. Rewrite the actual inset around independent arrival and glass geometry, or remove this obsolete derivation. The separate circle at664 still centers turning circle on curtain wall centre instead of arrival curve_centre; drive legend726 still calls curved glass derived from turning circle despite updated narration.

4. Both models _slab lines629-630 and exchange _beat_slab lines786-793 draw/highlight only first-floor voids[0] (V-RAMP), but numeric slab void_area subtracts V-RAMP plus V-STAIR-1. Their gross-minus-opening equations, labels and hatch are inconsistent with the shown geometry. Draw all voids with source tags and describe plural openings; synchronize subtraction highlight with total area. Models first-bay dimension at656 still starts from hardcoded x1.25 instead of actual grid x positions.

5. Objects _south lines854-900 remains a bespoke box approximation instead of shared source elevation. It draws every distinct column x from all storeys from ground to first soffit (870-872), fills one rectangular ground enclosure from aggregate space x extents (863-868), and draws roof objects solely from g.screens (885-889). SC-FRAME is now STANDARD, so that landscape frame/opening is omitted entirely; other actual standard roof walls are absent too. It is called for main elevation, thumbnail and recap, so misleading views occur repeatedly. Replace geometry construction with shared villa_elevation and map selected wall/opening/window parts for staged highlights.

## Coverage Notes

Information, objects, representations, models, exchange and consistency chapter source were all searched for old connected curtain paths, hardcoded geometry, room topology and selected support assumptions. Above findings are source-level concrete defects. Models room chapter itself derives counts and areas across all three storeys; no fixed-room-count failure found there. Existing representations scoped W-S ribbon assumption remains valid. No new concrete failure isolated in consistency during this bounded re-audit. Runtime suite and six render probes are still necessary.
