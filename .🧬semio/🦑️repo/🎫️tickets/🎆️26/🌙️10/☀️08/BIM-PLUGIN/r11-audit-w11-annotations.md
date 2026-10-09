# 🔎️ R11 Audit — w11-annotations (WP-11)

Status: **PARTIAL**. Mutation leaves, editor, SVG layer and inference wiring are on disk. The verification layer is missing.

## Present
- 15 leaves: create/set/delete for dimension (`↔️📌️📎️`), tag (`🔖️🏴️🎫️`), text-note (`🗒️📝️📃️`), leader (`↗️⤴️↪️`), and
  annotation-style (`🎨️🖊️🧺️`). Enum `🧬️mutations/🦀️.rs:115-129`, KINDS 256-270, cascade `🌊️cascade/🦀️.rs:110`, entity
  `🧩️entities/🪧️notations`. Fixtures (applied + rejected) and mutation feature rows (`🥒️.feature:112-126, 264-278`).
- Inference `🪧️annotation-layout` (anchors, dimensions, texts) in the model graph (`ModelNode::Annotation`); 15 unit tests,
  including anchor-move and style-edit recompute; plan-linework notation drawing.
- Editor gesture `📏️annotate`, utilities (shift+d/t/n/l, plan), arm-utility rows, en+de labels; SVG `annotations` layer
  (`🎨️svg/🖍️drawing/🦀️.rs:99`); text snapshot metrics.
- Shapely oracle file `🧪️tests/📏️infer-bim-1-annotations/🐍️.py` and one inference fixture `🪧️annotation-layout/🏠️room`.

## Missing
1. `🥒️.feature` for `📏️infer-bim-1-annotations`.
2. Oracle registration in `🔮️oracles/🔣️.json`.
3. Graph-level gating + cache-transparency test in `🕸️model-graph/🧪️tests/📈️incremental`.
4. SVG annotation layer scenario/fixture in `🎨️export-bim-1-svg`.
5. Annotations in the house/office examples (via generators).
6. IFC `IfcAnnotation` export (optional, but do it: full BIM).
7. Sum-law test confirmation in the five delete leaves.

Last log check6 (23:20): 8 errors, all in peers' areas (railings BALUSTER/INFILL w09; schedule window w13; chain `Ramp` w09;
model-graph compute railing arity w09).
