# Mockup Exploration

Read root AGENTS.md. No additional AGENTS.md files were found inside the teaching tree. Read-only exploration used repo tools exec python; rg is not registered by repo.

## Existing Renderer

Architecture site entry: 🎓️teaching/🏛️architecture/📚️course/🟦️.ts. It mounts @semio-tech/course-react/app with lazy local course material. Material loader is 📚️course/📚️course/🟦️.ts.

Framework domain root: 🧰️framework/🛍️products/📚️course.
React renderer: 🎯️targets/⚛️react/🔨️modules.
- ▶️run/🟦️.tsx lines 51–73 dispatches ClassificationTaskView, SortingTaskView, MatchingTaskView.
- ▶️run/🟦️.tsx around 347 renders the localized task prompt immediately before SteadyTaskView: natural insertion point for task context mockups.
- 🗂️classification/🟦️.tsx lines 111–116 renders item chips, label, keyboard select and pointer drag.
- ↕️sorting/🟦️.tsx around 160–177 renders ordered item rows.
- 🃏️matching/🟦️.tsx around 137–141 renders item slots.
- 🪟️chrome/🟦️.tsx IconLabel (213) renders an inline emoji then children; TaskGlyph (170) renders a small title icon. Icons have hover/focus microanimations and reduced-motion behavior.
- 🧩️task/🟦️.tsx provides shared task props, keyboard focus helpers, locked state, accessible numeric entry, polite announcements and visible hint text.

No existing Excel/Grasshopper mockup implementation was found in these modules. Existing display is compact emoji labels and native accessible task controls.

## Recommended Contract Boundary

Use an explicit optional mockup property on tasks/items, not an Icon extension. Icon is narrowly defined by the schema as one emoji grapheme plus motion, used in small inline labels and title chips. A realistic spreadsheet or node graph has semantic structure, substantial layout and sometimes native controls; placing it in Icon would conflate decorative marks and task content, repeat previews on value cards, and risk invalid nested interactive controls.

Define a language-neutral discriminated Mockup schema first (for example spreadsheet and node-graph variants), with localized caption/labels and data describing rows/cells or nodes/ports/connections. Keep UI appearance data independent of grading truth: hard/expert sheets must not disclose the values they ask learners to guess.

Domain contract: 🧬️schema/🔣️.json (the large root contract; confirm exact basename). Authored task definitions and SheetClassificationTask/SheetSortingTask/SheetMatchingTask must carry task mockups; item definitions and SheetItem must carry item mockups. Current SheetItem properties: id, label, short, icon. Current Sheet tasks already carry prompt and icon. TS/Rust parity points:
- root 🟦️.ts and 🦀️.rs domain types;
- 🔨️modules/✅️validation/🟦️.ts and 🦀️.rs;
- 🔨️modules/🃏️sheet/🟦️.ts and 🦀️.rs (presentation mapping);
- schema wire version/fingerprint is pinned and structural edits require following its contract checks.

## Renderer Boundary

Implement reusable native React mockups in a dedicated course React module (domain-neutral software/workspace previews). Render task previews after prompt; render item previews as siblings of label/control rows, never inside buttons, labels or selects. Excel preview should use native table semantics with row/column headers, formula/name bars and selected-cell styling. Grasshopper preview can use SVG for wires and native HTML labelled nodes/ports; provide a text summary or semantic list that conveys every meaningful relationship without relying on wire color or geometry.

Use local responsive CSS and existing design tokens, visible focus, accessible names, keyboard alternatives and reduced motion. Keep existing answer controls authoritative; if preview interaction alters a graded answer, route through onAnswer and existing CQRS state rather than independent hidden state.

Add language-agnostic fixtures before implementation that verify presentation survival and absence of solution leakage across challenge levels, with TS/Rust and third-party JSON-schema validation. React tests should verify table/graph accessible names and relation text, keyboard behavior, locked behavior and narrow-screen overflow.
