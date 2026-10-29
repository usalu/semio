# Native Ground Curtain and Joinery R31

## Published Model

The coordinated source has445 products after removal of artificial opaque strips W-G-ENTRY, W-G-SERVICE-ENTRY and W-G-SERVICE-RETURN. The parent/R33 ground registration, substrate and source-corrected circular column15 are preserved. Actual primary boundaries replace the inherited car-derived3.5m circle. Seven explicit structural-axis/stair controls give a maximum registration residual0.115m; coordinate uncertainty is about±0.15m. See ground-registration-r31.md and its inspected three-panel diagnostic. CMN contemporary-plan and FLC19414 design phases remain distinguished. The eastern garage envelope is separate from the glazed segment; its homogeneous150mm opaque interpretation/material remains inferred. The garage portal remains on W-G-E, not on the visible diagonal boundary.

One PATH-HALL reference path has57 source-authored station records:28 visible photograph candidates,24 occluded continuations and5 endpoints. Camera and station positions are inferred; hidden counts are not an attested historical schedule. Occluded continuation uses the reconstructed visible bay range, not a uniform global pane grid. Actual panes are planar chords between the registered members. A real intermediate metal rail and jamb/head/sill/mullion material replace the earlier continuous curved glass band. The curtain has433 occupied physical parts. Textured translucent glass is a separate native material; exact surface pattern and optical values are unresolved.

OP-D-ENTRY and OP-D-SERVICE are true curtain-hosted IfcOpeningElement products. Source geometry has exactly one host domain: wall, slab or curtain_wall, with a scoped curtain_path. Native portal cuts remove actual glass/member material and preserve fanlights. Public opening_geometry is shared by physical fillings, takeoff and all seven representations. Main/service door GUIDs and tags remain stable.

The main entrance now has source-supported timber double leaves, actual hollow timber construction, insulated metal side wings and a recessed metal frame. Wood battens remain timber even when the outer frame is metal. The main rough width is1.86m and actual free width1.65m after inferred80mm side wings and25mm frames. Service rough width0.95m gives0.90m free width and2.03m free head. All wing/profile/recess dimensions and unidentified insulation density are explicitly inferred/assumed. D-SALON is corrected to metal-framed glass; its retained single-leaf count is inferred and its location is unchanged. Obsolete unused TY-GLAZED-DOOR is removed.

## Shared Runtime and Drawings

The new model/🪟️curtain provider uses standard-library data and geometry only. Its strict native path/station/transom schema is language-independent. building_geometry()["curtain_wall"] is the same physical filling object used by the source. paths are records containing tag, reference_path, actual member path, stations and length. The single-path/radius/centre aliases and car-to-curtain radius dependency are removed. The independently assumed car route is checked against actual geometry.

The shared catalogue and plan, section, elevation and axon consumers include the actual native curtain-hosted door parts; the old wall-only loops would omit them. All representations and material quantities consume the same occupied parts. Curtain takeoff sums the independently verified disjoint physical partition rather than repeatedly unioning the same parts. No runtime external dependency was added.

## Native IFC and Independent Validation

Native selected-product STEP retains the actual planar panes, metal members, timber skins/battens, insulation and sheet-metal wings with constituent material sets and shape aspects. Insulation is separate from framing; glass materials export their native category. Four new portable native curtain/wing contracts passed after targeted red failures. Full445-product Draft202012Validator passed.

Independent web-ifc0.0.78 accepted all12 EN/DE samples with native product/type identity, dimensions, localized names, complete material/aspect coverage, placements and reconstructed volume agreement. Native volumes: curtain0.301991922862m³ across433 solids; entrance0.098544998160m³ across29 solids; salon0.020886793297m³ across8 solids. The complete native six-method suite had one stale sash-category fixture; its corrected independent material coverage method passed. The final combined15-contract run is recorded below when complete.

The independent source rail-buffer oracle uses a1e-10m GEOS precision grid only for comparing nearly coincident polygons, retaining its1e-9m² tolerance. Runtime source coordinates/facets are unchanged. This avoids the pathological exact-overlay stall also encountered in the earlier beam oracle.

## Limits

This is a source-informed reconstruction, not a complete measured1931 joinery survey. CCA PH1985:0889 lists unknown photographer and1931 or later. Original full-size metal profile pixels, exact concealed station counts, hinges/locks, detailed glass texture, wing construction dimensions and several phase differences remain unresolved. Selected-product IFC fragments preserve physical aperture cuts but are not a complete relational IFC building export with all neighboring hosts/fillings. Native Manim rendering remains blocked by the repository launcher condition already recorded by the parent; source factories and geometry contracts are the available checks. The parent owns complete-model geometry, parking and six-chapter integration capture.

## Final Handoff Checks

The combined15-contract run completed in53.292s:14 passed and one exposed a pre-existing stale MAT-GLASS association to ROOFLIGHT-1, whose declared material is MAT-RC. Removing only that stale membership made the focused association check pass in0.151s. No geometry or product count changed. Four updated drawing modules parsed successfully.

A subsequent catalogue regression was tested red then green: building_geometry.fillings contains only actual aperture windows/doors, while the whole curtain assembly is separate. The433 curtain collision parts are emitted exactly once with kind curtain and unchanged tag/part IDs. The focused portable catalogue/collision test passed in0.225s; native CW STEP still has433 solids. Current independent web-ifc validation was rerun successfully for12 samples: six native product cases in both languages. Earlier handoff messages saying14 samples were an arithmetic error; the actual recorded result has12.

The parent separately fixes the obsolete wall_curve_centre field in the assumed car-route result and owns fresh route/parking integration. Final source SHA256 after the relation-only cleanup:a444e6aa6edea9ae5deb0460cc52f0e6ef88395b25e48eebfadc8ea74bfa74bf. Source/API/views/exchange handoff is frozen; no further work is claimed on the newly identified roof/stair reconstruction. All native test interpreters are disposed at handoff.
