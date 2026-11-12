# Authoring requests (furniture wave)

Precise notes on tool and data issues found while authoring `furniture`. Each was worked around locally; none edits shared code.

1. Catalog data: no `solid.sweep` on any extruded hint of the furniture assets (D47 migration not applied). `evaluate` treats a missing sweep like `v`, so every `extruded` shape is drawn wrong in front and plan (profile placed in the (u, w) plane): cafe-chair plan IoU 0.13 live against 0.97 with `sweep: "w"`. The furniture scripts therefore also check a scratch copy with `sweep: "w"` added (`authoring-furniture-patch.py`).
2. Loft semantics: the schema description (`VolumeSolid`, loft) says the section frame follows the tangent (first axis = bone-local `w` projected perpendicular, `rotation` turns the section), but `tessellateSolid` (`🧵️loft`) places every section in the x, y plane of its own `rotation` (identity: the u, v plane) and ignores the tangent. The sample figure's torso loft (no `rotation`, sections along u) is degenerate with the kernel as it is. Furniture lofts carry an explicit `rotation` that puts the section plane perpendicular to the loft axis; if the volume module later implements the schema's tangent frame, these rotations must be reviewed.
3. Capsule and revolve `lateralScale`: the kernel scales along the node X axis (local u) and falls back to Z (w) only when the axis is parallel to u; the schema says w. Furniture volumes use `transform.scale` (about the solid origin) or lofts instead.
4. `volume-profile-self-intersecting` rejects a revolve profile whose first point equals its last (a closed loop, the kernel's torus route) because repeated points are degenerate; a torus is therefore not expressible as a valid `revolve`. Rings are authored as one capsule chord with `repeat` (24 copies rotated about the ring axis).

## animals (loft frame, repeat of the appliances-b note)

- `authoring-animals-lib.ts` (`fit`, `tube`, `stripe`, `shell`, `slice`, `patch`) emits every loft for the kernel as implemented: path along u gets `rotation: [0.5, 0.5, 0.5, 0.5]` (`rx` on bone-local v, `ry` on w); tilted tubes get a quaternion built from the path tangent (`rx` on the sagittal normal, `ry` on w). If the kernel is changed to the schema rule (`rx` lateral, rotation turning the section), every animal loft must be re-checked with `volume-check` (all 14 animal volumes use it).
- `Bone.z` offsets (hedgehog coat bones, bird wing and pigeon humerus bones) are part of the bone-local origin: loft section centres must subtract the bone's lateral origin, otherwise the solid is displaced twice (found when the hedgehog skirts came out 0.04 m too wide); the schema text says so, `volume-draft.ts` handles it.

## authoring-components (re-appended after an overwrite)

- Loft section plane: kernel uses the plane of `rotation` (default u,v), schema text says perpendicular to the path tangent. The components category uses no lofts, so the coordinator's rotation amendment needs no change there.
- The staffage validator flagged closed revolve profiles (first point equals last) as `volume-profile-self-intersecting`; the components volumes avoid them (C-sections are extrudes, the garage door arc rail is ten straight segments with a rotating repeat, the coil is an open sawtooth profile).

## From authoring-vegetation: `volume-lib.ts` `meshes()` and lofts without `rotation` (superseded by the coordinator amendment)

Resolved by the amendment: every vegetation loft section carries an explicit `rotation` (absolute frame), so check and module agree. Also noted: `veg-validate.ts` (ticket-local) walks every pose and clip of an asset; `volume-lib.ts` has no pose walk.

## machinery author: scissor-lift bone ids violate Slug
`scissor-lift` rig bones `A1..A5`, `B1..B5` are not kebab-case slugs (Ajv: `/rig/bones/N/id must match pattern`, already failing before any volume). Volume solids must name those bones, so Ajv also reports `/volume/solids/N/bone must match pattern` (57 solids bound to them). `volumeIssues`/`validateAsset` report 0 volume findings. Fix needs a rig rename (a1..b5) across rig, poses, clips, representations by the asset owner; not done by the machinery author.

5. Resolved by coordinator amendments (loft rotation stays absolute, `lateralScale` kernel semantics, closed revolve profiles valid, sweep fixed catalog-wide later): items 2 to 4 above need no action from the furniture wave.

## vehicles author: volume-check limits (no tool bug blocked authoring)
1. `compareView` counts only closed body/detail/glazing shapes. Tube frames drawn as open polylines (city-bicycle, cargo-bike rear rack/fender, tram pantograph arms) and wheels drawn as filled discs (cargo-bike elevation `tyre-*`, `rim-*`) are physically hollow or thin in the volume, so IoU is capped (city-bicycle elevation 0.74, front 0.78; cargo-bike elevation 0.79). A fair gate would stroke open shapes with their `penMm` x denominator and treat spoked wheels as rings.
2. The bounding-box gate measures rest frames; delivery-van rest has both rear leaves swung open (x +17 %), the default pose `parked` is +0.7 %. Judge in `presentation.defaultPose`.
3. `authoring-components-validate.ts` evaluates "rest" with identity frames for all bones, so its rest component count is meaningless for assets with offset bone origins; the per-pose lines are correct.

## objects author: findings (workarounds are local; nothing edited)
1. Revolve always closes both ends of the profile to the axis (flat discs), and the validator rejects a closed loop, so an open-top vessel (bowl, pot, basin, dome, crate) cannot be a revolve. Objects use stacks of tapered ring extrusions (profile with a hole plus `taperTo`, hole walls face the cavity) with a solid dish or crown; a revolve that stays open at one end would remove ~10 solids per vessel.
2. `taperTo` scales about the plane origin, so a ring or rectangle ring must be centred on the origin of its plane (translation via `transform`); non-uniform tapers (storage-crate-stack 0.56 x 0.34 to 0.60 x 0.38) are not expressible.
3. `volume-check` and `validateAsset` judge only the rest pose. market-parasol rests with all eight ribs collinear (open umbrella only through pose offsets), so rest bbox z is -53 % and the rest silhouettes fail by construction; the default pose open-still is within 1.4 %. retractable-bollard rests lowered (y -33 %); the default pose raised is exact. A `--pose` option on `volume-check` (poses via `solveChain` plus `carry`, as `authoring-objects-report.ts` does) would make the gates meaningful.
4. Front and plan placement of shapes with a `solid` hint on a swinging bone is wrong in `evaluate` (playground-swing `seat`, `chain-*`: seat front at x -0.098..-0.04, chain at x 0; shopping-trolley `gate`), so these authored silhouettes cannot be matched by any physically right body.
5. Several catalog files fail Ajv outside `/volume` (`/clips/N/fps must be >= 1`; `representations/50/geometry/<view>/N` missing `x0,y0,x1,y1` on ellipse-with-capsule-keys shapes) in bicycle-stand, chalkboard, charcoal-grill, drinking-fountain, market-parasol, playground-swing, shopping-trolley, waste-bin; not caused by the volume and not touched.

## authoring-humans-a: draft solid positions of view-only parts, and 2D data issues (no tool bug blocks the work)

- `volume-draft.ts` places view-only parts (`knee-*`, `eye-*`, `face`, `shoulders`, `hair-nape`, far limbs) with a `transform` rotation and an estimated `from/to`; the resulting solids sit inside the head or behind the figure (for example the child scooter `face` extrudes behind the head). All were re-authored by hand for the humans-a assets.
- Draft id clash: front/plan shape ids that equal an unrelated elevation id are merged by id (courier-with-hand-truck `brace-0/1` crossbars of the front view versus the `brace-1/2` toe stays of the elevation).
- Bones that are children of a held object inherit the parent's `z` (parent-with-stroller: stroller, hood, wheels, child sit at z -0.1925); the draft's from/to are already local, hand-authored lateral numbers from the front view must add 0.1925.
- 2D data: person-cooking draws head, neck and hair as revolves about an axis 0.15 m right of the body (`axisZ: -0.15`); person-with-umbrella draws the canopy only as a thin sliver along each rib, so a dome's side silhouette cannot reach IoU 0.80 at rest (0.78).
