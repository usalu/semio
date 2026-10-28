# Energy Clip Style Implementation

The shared energy visual helpers currently use neon and pastel palettes, rounded annotation frames, and independent spacing values. The semio styling tokens define Anta, Kelly Slab, Share Tech Mono, semantic colors, square chrome, and compact spacing. The canonical WindowSilhouette geometry creates a single rectilinear outline around body and cap chips, without drawing an internal cap/body seam.

Implementation scope: shared visual helpers, semantic scene palette constants, and scene annotation corner radii. Preserve physical device and human glyph geometry and the energy certificate grading colors, whose green-to-red progression is domain data. Use the shared theme module created by the parent task for all general styling values. The language-neutral geometry fixtures and third-party path union validation are owned by the test agent. Their missing-silhouette assertion failed before implementation began.

Implemented API: window_silhouette_outline(width, height, *, top_depth=0, bottom_depth=0, top_chips=(), bottom_chips=(), inset=1) returns canonical pixel-coordinate vertices. window_frame(width, height, *, title_width=None, title_depth=0, color=P_TEAL, fill_opacity=...) returns a centered Manim Polygon, with total height including its fused top-left title chip. All general annotations use square corners and token-based spacing.

## Completed Changes

Shared visuals now resolve palette roles through the canonical theme, convert canonical pixel outline geometry to scene units, and use one fused polygon for window frames. Chips share the same filled square surface. Quantity cards and everyday-watt cards fuse their header into the frame, while formula and caption panels use square chrome and token padding. Formula backgrounds stay behind formula text. Local backlink and mathematical handover chip helpers use the shared chip implementation. Local annotation frames use canonical border and spacing values; physical devices and people retain their geometry.

General scene colors now use named semio tokens. The institute intro uses shared type sizes, a single outer silhouette, and a fused series title cap. The heating calculation chapter headings use the shared scene-title helper. Primary-energy mathematical cards and regulatory standard cards use the shared fused quantity-card layout; internal-gains usage and source cards now fuse their name into the top cap.

## Validation

A repo-wrapped Python AST pass parsed 95 energy Python sources successfully, excluding provisioned dependency environments. The dedicated UI contract has validated six portable outline cases against independent Skia PathOps polygon unions, including exact empty XOR area, and verified fused frame vertices plus square chip/card content containment. The root reports the latest 19 tests passing after the final scene edits.

## Retained Domain Colors

The following energy certificate rating colors remain exact domain data, shared conceptually between certificate and practice chapters. They represent the familiar A+ through H scale rather than general interface styling.

- 🎓️teaching\🏛️architecture\⚡️energy\📊️demand\🎬️clip\🏷️certificate\🐍️.py:76 ("A+", 30, "#1D7A3E"), ("A", 50, "#3FA34D"), ("B", 75, "#8DC63F"),
- 🎓️teaching\🏛️architecture\⚡️energy\📊️demand\🎬️clip\🏷️certificate\🐍️.py:77 ("C", 100, "#C7D93A"), ("D", 130, "#F9E547"), ("E", 160, "#F5A623"),
- 🎓️teaching\🏛️architecture\⚡️energy\📊️demand\🎬️clip\🏷️certificate\🐍️.py:78 ("F", 200, "#E8622C"), ("G", 250, "#D0342C"), ("H", None, "#A61C1C"),
- 🎓️teaching\🏛️architecture\⚡️energy\📊️demand\🎬️clip\🧪️practice\🐍️.py:109 ("A+", 30, "#1D7A3E"), ("A", 50, "#3FA34D"), ("B", 75, "#8DC63F"),
- 🎓️teaching\🏛️architecture\⚡️energy\📊️demand\🎬️clip\🧪️practice\🐍️.py:110 ("C", 100, "#C7D93A"), ("D", 130, "#F9E547"), ("E", 160, "#F5A623"),
- 🎓️teaching\🏛️architecture\⚡️energy\📊️demand\🎬️clip\🧪️practice\🐍️.py:111 ("F", 200, "#E8622C"), ("G", 250, "#D0342C"), ("H", None, "#A61C1C"),

## Files Changed by the Style Implementation

- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/☀️solar-radiation/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/👥️internal-gains/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/💧️transmission-humidity/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/📐️system-sizing/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/❓️need/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/📦️volume/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/🤝️handoff/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/🪟️natural/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/🎬️clip/👋️intro/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/♨️system-losses/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/⚖️regulations/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/⚡️demand-power/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/🏭️primary-energy/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/🏷️certificate/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/🔁️recap/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/🧪️practice/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/☀️solar-gains/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/🌬️convection/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/🧮️calculation/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/🧱️conduction/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py
- 🎓️teaching/🏛️architecture/⚡️energy/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py

## Screenshot Review Follow-Up

The physical-fundamentals electricity meter now uses a single square WindowSilhouette with its Zähler caption fused into a top-left cap. Its energy readout and wire endpoints retain their original physical anchors. The same scene no longer overrides the canonical label/body font sizes. A source scan found no remaining scene-level numeric TITLE/LABEL/BODY/SUBTITLE/FORMULA/CAPTION_FONT_SIZE overrides.

Two local equation highlights in heating calculation and conduction now use explicit canonical spacing and border width; shared math-panel spacing is two UI units.

## Final Visual QA

Reviewed the native 1280×720 screenshots for the Lüftung institute intro, physical_fundamentals clause 1, and ventilation clause 1. No implementation defect was identified, so no additional source change or test run was needed.

- Intro: the Gebäudeenergie · Lüftung label is fully inside its fused cap, with visible horizontal and vertical padding. The cap and body share one continuous square outline. The main Lüftung title and wrapped explanation remain inside the body with clear margins; the two-line explanation clears the bottom border. The university identity, institute hierarchy, author line, and watermark remain legible with the common Anta type and semio palette.
- Physics: the Zähler label stays inside its fused top-left cap, the 4000 kWh readout remains centered in the body, and both wires keep their horizontal physical anchors. The title, subtitle, meter, and caption use consistent semio typography and square UI framing. The thought bubble retains its intentional rounded physical/comic shape. Umlauts, quotation marks, and colon now render in the inspected frame without tofu.
- Ventilation: title, subtitle, standards reference, CO₂ label, gauge labels, formula readout, and subtitle are legible and separated. The caption has clear padding and remains inside the safe frame. UI typography and colors agree with the intro and physics frame; the physical section/gauge remain distinct from chrome.

Screenshots reviewed (generated review artifacts, removed at ticket completion):

- 🗑️generated/o/intro/i/Demo_Intro_Lueftung_ManimCE_v0.22.0.png
- 🗑️generated/o/qa/physical_fundamentals-1.png
- 🗑️generated/o/qa/ventilation-1.png

## Isolated Cooling Visual Review

Ran only the cooling preview through repo capacity run with a 6144 MB reservation and one CPU, using the existing ticket Python environment and isolated per-video text cache. The process captured two settled clause frames and exited successfully (exit 0). No full-video render, additional process, source edit, or test run was performed.

Reviewed 🗑️generated/o/qa/cooling-0.png and cooling-1.png at native 1280×720 resolution. The first frame has a legible common Anta title, teal subtitle, standards reference, and a two-line white caption within a square frame. Umlauts and punctuation are resolved, and the caption clears the house drawing. The second capture shows the solar/internal-gains content state: the Person and Gerät watt-reference cards both have intact fused caps, labels inside their cap bounds, centered values, distinct icons, and comparison hints inside the body. Both cards use the same square chrome and semantic palette as the previously inspected intro/physics/ventilation frames. No clipping, overlapping card content, malformed glyph, or implementation defect was found in these two captures.

## Final Call-Site Normalization

Heating calculation now uses FORMULA_FONT_SIZE for eight main equation rows and BODY_FONT_SIZE for two supporting numeric/quantity rows. Their former independent 0.08–0.18 gaps are one or two UI_SPACING units. The final main-equation chapter heading, “Die Hauptgleichung des Heizwärmebedarfs (DIN V 18599)”, uses scene_title with the shared frame and type scale. The ventilation heat-flow moisture-content chart columns are square Rectangles with the canonical chrome border. These changes affect only two paths already listed among the same 26 source files.

The large question mark in the regulations diagram retains its intentional font_size=72 as a diagrammatic icon, rather than a text hierarchy role. The scoped test-agent check passes: both affected modules parse and import, and the complete main-equation heading fits within the viewport and inside its shared square frame. The targeted run completed in 4.221 seconds and its native title screenshot was visually verified. The full 19-test font/style suite was not repeated for these simple call-site changes.
