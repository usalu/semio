# 📐️ Conventions of the staffage views (owner: sonnet-contracts)

Derived from the code on 2026-11-02: `🧬️schema`, `🔨️modules/🔗️contracts`, `🎨️depiction` (`🔭️views`, `🧩️shapes`, `🧊️depth`, `✂️section`), `🧮️evaluation`, `🦴️rig`. The ruling D6 fixed the wording. The camera table is also code: `🔨️modules/🎨️depiction/🔭️views/🟦️.ts` (`CAMERAS`, `project`, `nearness`).

## 1. Frame (asset-local and scene frame are one)

- Metres. x ahead (the asset's facing, default pose faces +x), y up, z lateral with the asset's left at +z. Ground is y = 0; the origin sits on the ground at the asset's reference point.
- Handedness: left-handed. x ahead, y up, z left. A right-handed frame with x ahead and y up would have z to the right. The brief's first wording ("right-handed", "seen from +z") was wrong; D6 keeps the code. Evidence: README ("the near wing of a right-facing side view is negative z"), `🧊️depth` (smaller z is nearer in the side elevation), `projected()` in evaluation.
- Bone frames (`EvaluatedBone.frame`, `BoneFrame`): `u` along the bone, `v` its normal in the sagittal plane at zero spread, `w` the lateral axis. All `Vec3` in (x ahead, y up, z left). Angles are degrees, counter-clockwise positive seen in the side elevation.
- `root {x, y, z, angleDeg}` is the placement of the root bone in the scene; scene and asset frames coincide, so a root offset translates the whole asset.
- Moments: `mzNm` about the lateral axis is the sagittal moment, counter-clockwise positive in the side elevation (sagging positive for beams). `mxNm` about the forward axis and `myNm` about the vertical axis follow the right-hand rule of the axis directions as written (x ahead, y up, z left).

## 2. Cameras (orthographic; paper-up coordinates, y up; the depiction flips y to paper millimetres, y down)

| viewpoint | name (en / de) | seen from | looking along | paper x | paper up | nearer |
|---|---|---|---|---|---|---|
| `elevation` | Side elevation / Seitenansicht | the asset's right (-z) | +z | x (ahead is right) | y | smaller z |
| `front` | Front elevation / Vorderansicht | ahead (+x) | -x | z (the asset's left is paper right) | y | larger x |
| `plan` | Plan (top view) / Draufsicht | above (+y) | -y | x | z (the asset's left is paper up) | larger y |

- Paper space: millimetres, x right, y down. A model point sits at paper `(origin.xMm + x * k, origin.yMm - y * k)` with `k = 1000 / denominator`.
- Switching viewpoint changes only the table row. Pose, clip time, root motion, seed, environment, scale and attachments are one evaluated state (`resolveViews`).
- Extent mapping per viewpoint: elevation (x = `widthM`, y = `heightM`); front (z = `depthM`, y = `heightM`); plan (x = `widthM`, z = `depthM`).
- The elevation is the rig plane. Front and plan shapes are authored in the rest pose in their own plane (front: paper x = z, plan: paper x = x, paper up = z) and follow their bone through the full 3D carry; `solid` shapes (extruded, revolved) are built from the bone-local profile and are allowed in every viewpoint.

## 3. Layout and ground

- Ground-role shapes are drawn but never count toward the figure's extent: not for the overall dimension, not for the ground line span, in any viewpoint (`figureOf` in depiction). The ground line is drawn in the side and front elevation, never in the plan.
- The overall dimension measures the viewpoint's own drawn shapes (height in the front elevation, depth along paper up in the plan); it never borrows the side elevation's extent. Fit-to-frame (`Canvas.reach`) uses the shapes of the viewpoint, plus the rig bounds only in the side elevation.

## 4. Hidden lines and occlusion

- Paint order is the authored `z`, then authored position. In a pose or clip instant a pair of shapes on different bones changes paper order only when the physical depth along the view direction reverses against the rest pose by more than 2 mm (`DEPTH_TOLERANCE_M`); depth is `nearness` of the bone frame (or of the placed profile of a solid).
- Closed, filled shapes of role body, detail or texture occlude everything below them in z. Outlines are role-pen runs with the parts behind a higher shape removed (exact clipping); hatches are clipped to their shape and hidden like outlines; same-region filled shapes on the same or adjacent bones hide each other's inner outlines so they read as one silhouette. Glazing does not occlude.
- Every view is a true orthographic view with hidden-line removal (D40). Hidden edges are never drawn dashed, and a part that lies completely under an overhead part is not drawn at all (in the plan, the mast of a parasol under its canopy is hidden). A cut plan (Grundriss) is only a named section (`sections.plan`, "Plan section B-B" / "Grundriss-Schnitt B-B"), never the default plan, which is the top view (Draufsicht). Glazing never occludes and is never hatched: a glazing shape that names a hatch is a validator failure (`glazing-hatch`), and the hatch loop skips glazing.
- Alle Ansichten sind echte orthogonale Ansichten mit Entfernen verdeckter Linien (D40). Verdeckte Kanten werden nie gestrichelt gezeichnet, und ein Teil, das vollständig unter einem darüberliegenden Teil liegt, wird nicht gezeichnet (in der Draufsicht der Mast eines Sonnenschirms unter seinem Schirmdach). Ein Grundriss-Schnitt ist nur ein benannter Schnitt (`sections.plan`), nie die Standard-Draufsicht. Verglasung verdeckt nie und wird nie schraffiert.
- Every representation paints the same prepared rings; line views add outlines, solid views add tones, analytic views add legends.

## 5. Sections

- A representation may draw its front elevation or its plan as a section (`sections.front`, `sections.plan`: `plane`, `label`). The geometry authored for that viewpoint is the cut drawing; a section never changes the other views or the evaluated state.
- Front cut: vertical plane across the forward axis at `plane` = forward x, seen from ahead; caption "Section A-A" / "Schnitt A-A". Plan cut: horizontal plane at `plane` = height y, seen from above; caption "Plan section B-B" / "Grundriss-Schnitt B-B".
- The side elevation shows the cut-line marker at 1:100 and larger: a vertical line at x = `plane` with arrows toward -x (front cut) or a horizontal line at y = `plane` with arrows pointing down (plan cut). The validator requires the plane to lie inside the asset's elevation extent (`section-plane-outside`).

## 6. Time, units, ids

- Seconds; frame n of a clip at t = n / fps; looping clips wrap the pose, root advance and transport continue on the absolute timeline. SI throughout, temperatures authored in degC; paper quantities (pens, hatch spacing, print profile) in millimetres.
- Scale maps metres to paper millimetres only (`mm = m * 1000 / denominator`). Presets 500, 200, 100, 50; each authored band declares a `validRange` of custom denominators.

## 7. Dimensions envelope and tolerance

- `dimensions` is the physical envelope of everything that belongs to the asset in its rest pose: held items, attachments, plates, flanges, lids and footings. It excludes stroke width, ground shading, plumes, transport and annotations. Measurement uses geometry only (rings), never pens or roles other than excluding `ground`.
- A drawing agrees in every viewpoint when |measured - declared| <= max(2 % of declared, 15 mm).

## 8. Solids

- `revolved`: the axis passes through the point translated from the rest origin of `axisBone` by `axisX` along world x and `axisZ` along world z at rest (not along the bone's `u` and `w`), in the bone-local direction `axis` (`u`, `v` or `w`), and is then carried with the bone frame. `axisBone` is the shape's own bone or one of its ancestors (`solid-axis-not-ancestor`). `extruded`: spans `z0`..`z1` laterally (left is +).

## 9. Symbols and fidelity

- `SymbolRepresentation.semantics`: `true-extent` (model metres, real size), `true-position` (real position, paper millimetres), `nominal` (fixed paper sign). A symbol is authored for all three viewpoints.
- Fidelity (`DrawingScene.metadata.fidelity`, `ResolvedDrawing.fidelity`): `physical` for an authored scale representation or a `true-extent` symbol; `conventional-symbol` for `true-position` and `nominal`. UI and exports show it from this field.
- An unavailable scale carries `cause` (`below-printable-feature` or `no-physical-depiction`), is unavailable in all three viewpoints, and states a reason, the limiting feature and size, and at least one resolvable alternative (scale, symbol or an asset of the same category that is drawable at that scale).

## 10. Sheet arrangement (three views on one sheet)

The sheet (`🔨️modules/📤️export/📑️sheet`) composes the plan, the front elevation and the side elevation of ONE evaluated state at one paper scale as a plain `DrawingScene`.
- The side elevation is the main view. The plan shares its paper x (world x) with it; the front shares its paper y (the ground line) with it.
- Third-angle (ISO 5456-2 method 3, ANSI): plan above the side elevation, front elevation to its right. First-angle (method 1, DIN, SN): plan below, front to its left. The projection is always named on the sheet (`--projection first|third`), never defaulted.
- Because the front is seen from ahead, the asset's left is on its paper right, and in the plan the asset's left is up the page: the front's paper x and the plan's paper up both run along +z.
- Each view carries its own overall dimension measured on the drawn figure without ground, the caption of its viewpoint, the scale bar and the projection symbol.

## 11. Inconsistencies found

1. Brief versus code: handedness and elevation side (resolved by D6, code stays).
2. `🔗️contracts` header and schema header said "x right"; now x ahead.
3. The front-view overall dimension used the side elevation's extent (`dimensionExtent` returned the rig bounds for front); fixed to use the front shapes.
4. Ground-role shapes counted toward front and plan extents and the front ground line; fixed (`figureOf`).
5. `?? []` tolerance on front and plan geometry in evaluation, representation and copies silently produced blank scenes; replaced by `ViewMissingError` or typed unavailability.
6. Validator `extent-*` covered only the side elevation; front and plan extents are checked now.
