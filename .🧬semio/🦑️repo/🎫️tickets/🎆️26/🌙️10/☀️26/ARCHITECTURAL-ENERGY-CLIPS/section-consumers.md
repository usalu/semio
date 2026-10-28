# Scene Section Consumers

## Scope and Result

Architectural cut material in local house, room and plant consumers now uses the shared `section_material` interface. Construction uses the theme's uniform white, full fill opacity and no visible stroke. Masonry joints, plaster boundaries and cut hatching were removed from the heating room exterior and solar window cut. Natural ventilation's lintel now shares the material color. Ground datum hatching, glazing, heat arrows and separate analysis overlays retain their semantic styling.

Physics heat animations now color independent ceiling, floor and wall copies rather than changing the base construction's color or transparency. Conduction envelope analysis uses separate colored copies. Ventilation's focus transition dims labels and people while retaining opaque room material.

Touching local cuts now use live `section_assembly` groups for plant walls and roof, heating stage construction, ventilation volume house and natural room, demand practice room rows and the solar facade cut. The assembly retains the source pieces and follows their geometry/style. The parent verifies raster seams and renders bounded previews rather than a full curriculum render.

## Source Inventory

All source paths below are relative to `🎓️teaching/🏛️architecture/⚡️energy/`:

- `🔥️heating/🎬️clip/🧮️calculation/🐍️.py`: plant slab, roof and chimney material; window wall pieces; floor color override removed; room material uses shared interface.
- `🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py`: uniform room slabs and single exterior cut wall; obsolete decorative keys removed; thermal tint remains separate. Magnifier solid and particle bonds use semantically named helper-local colors.
- `🔥️heating/🎬️clip/🧱️conduction/🐍️.py`: opaque wall around door and separate envelope analysis copies.
- `🔥️heating/🎬️clip/☀️solar-gains/🐍️.py`: opaque facade pieces, room slabs and overhang; cut hatch removed.
- `🌬️ventilation/🎬️clip/❓️need/🐍️.py`: room material interface.
- `🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py`: room material interface.
- `🌬️ventilation/🎬️clip/📦️volume/🐍️.py`: opaque house polygons and rooms; room excluded from context opacity reduction.
- `🌬️ventilation/🎬️clip/🪟️natural/🐍️.py`: room material interface and uniform local floor, ceiling, back wall, parapet and lintel.
- `📊️demand/🎬️clip/♨️system-losses/🐍️.py`: opaque cellar wall material.
- `📊️demand/🎬️clip/⚖️regulations/🐍️.py`: opaque cut wall pieces around north-facing aperture.
- `📊️demand/🎬️clip/🧪️practice/🐍️.py`: opaque shared room-row ceiling, floor and partitions.
- `🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py`: opaque added floor slab and exterior window wall; independent heat overlays preserve solid base geometry.
- `❄️cooling/🎬️clip/💧️transmission-humidity/🐍️.py`: envelope heat colors an independent copy while base construction remains uniform and opaque.

## Test Inventory

- `🏗️authoring/🎨️visuals/🧪️tests/🔣️section_consumers.json`: portable opacity/stroke material contract, scales, module cases, construction keys and AST setup cases.
- `🏗️authoring/🎨️visuals/🧪️tests/🐍️section_consumers.py`: nine unittest methods using real Manim constructors, geometry helpers and AST-executed scene setup. A third-party Manim `Rectangle` generates the expected style output from the portable contract. Cooling verification mutates the heat overlay and checks the base material remains unchanged. The conduction regression constructs the actual `_stage` → `_lens` → `_lattice` chain and compares magnifier bounds with Manim `Circle` and bond styling with Manim `Line`.
- Parent registers this module in the canonical test entry.
- Ticket input `edit_section_consumers.py` records the bounded source transformations used during implementation.

## Verification

1. Before implementation, six test methods reproduced seventeen concrete style failures: translucent plant cuts, masonry color/joints, colored room construction, stroked volume polygons, demand walls and partitions. An ambiguous physics AST selector was then restricted to its exact scene method.
2. After implementation, all six methods passed in 8.597 seconds.
3. The final extended suite passed all seven methods in 13.900 seconds, including natural room material and the AST guard against direct base-construction style mutations.
4. After the one-line volume focus adjustment, live assembly integration and cooling overlay correction, the extended eight-method suite passed in 11.379 seconds through the managed capacity scheduler.

`_plant_house` now returns `material = section_assembly(parts["walls"], parts["roof"])`. Its production scene fades in this group. Bounded previews use `material` instead of drawing the walls and roof separately. Chimney material remains separate because production reveals it later with the plant equipment. The heating stage similarly exposes `material` and `construction` and draws `construction` in `_show_stage`.

Tests ran through native `repo tools exec uv -- run --project <authoring-python-project> --python 3.12 --group test python -X utf8 -m unittest 🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️section_consumers`, with managed environment `.🧬semio/🎓️teaching/⚡️cache/python`, `PYTHONDONTWRITEBYTECODE=1` and ticket-local `SEMIO_TEACHING_OUTPUT_ROOT` under `🗑️generated/sections-consumers`.

No Git mutations, worktrees, IDE usage, AGENTS changes or full curriculum render were performed.

## Late Conduction Magnifier Regression

Root review found that removed decoration globals `BRICK_FILL` and `MORTAR` still served the explanatory magnifier and molecular bonds. A portable `magnifier` case and real helper-chain test were added before the fix. The focused test failed with `NameError: BRICK_FILL` in `_lens` in 3.712 seconds. The two helpers now choose `solid_color` and `bond_color` locally from the same theme tokens, preserving the scientific illustration independently of the architectural base material.

The obsolete `joints`, `inner`, `outer` and `outline` dictionary keys were removed. The only remaining decorative-key caller used `outline` for label centering and now uses the actual `core` geometry. The material contract checks the exact remaining keys `core`, `tint` and `group`. The unused `PLASTER` global was removed. An AST scan of all energy Python scene consumers found only the two reported removed-global loads before the fix; no remaining `SHELL_STYLE` or `_SLAB` consumers were found.

The fresh late-regression suite passed all nine tests in 12.734 seconds in a new Python process; it is separate from the root aggregate run that had already imported the prior module.
