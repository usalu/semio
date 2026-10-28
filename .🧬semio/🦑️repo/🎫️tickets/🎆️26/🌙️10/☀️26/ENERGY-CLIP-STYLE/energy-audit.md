# Energy Clip Exploration

Energy teaching lives at `🎓️teaching/🏛️architecture/⚡️energy`, outside the headless energy plugin. Root AGENTS.md applies; no additional AGENTS.md exists along the teaching ancestor chain.

The curriculum manifest declares 112 segments: physical fundamentals 8, ventilation 22, heating 35, cooling 21, and energy balance 26. Shared typography is in `🏗️authoring/🔤️fonts/🐍️.py`; palette, layout zones, chips, formula panels, captions, and standing/seated glyphs are in `🏗️authoring/🎨️visuals/🐍️.py`.

Initial style used OS serif candidates and dark pastel colors. Shared standing glyph begins around line 1026 and seated glyph around 1057; seated return keys are animation anchors and must remain meaningful. Physics fundamentals has a local duplicate person helper around line 605; cooling internal gains has custom activity poses around line 89.

`🏗️authoring/📜️script.ts` exposes render and test commands. Renderer supports progress, cancellation, bounded parallelism, short output paths, source fingerprints, and segment reuse. Existing JSON cases and Python suites cover clip/font contracts and validate discovery using Manim. Representative beat rendering should precede any complete 112-segment render.

Exploration made no source edits.
