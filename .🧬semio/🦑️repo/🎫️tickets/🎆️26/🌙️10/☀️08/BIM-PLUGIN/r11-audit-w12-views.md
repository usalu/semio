# 🔎️ R11 Audit — w12-views (WP-12)

Status: **PARTIAL**. The authored-view core is in place. Missing: SVG per view, example views, and an oracle and feature for view-linework.

## Present and wired
- Leaves `📽️create-view` (22 cases), `🔭️set-view` (26), `📺️delete-view` (3; cascade on building/storey removal
  `🌊️cascade/🦀️.rs:160`); enum, KINDS, proto, graphql, TS, mutation oracle, root mount, feature
  (`🏙️mutate-model-1-any/🥒️.feature:153-155,305-307`); grammar + binary protocol (tags 12000–12002).
- Snapshot `🖼️views` (`views: BTreeMap<String, View>`, `ViewKind` = Plan, CeilingPlan, Section, Elevation,
  Orthographic, Perspective); `view_cut_height` falls back to `storey.cut_height`.
- Commands `🔭️create-view`, `🪟️set-view`; generic delete; create-entity of a storey creates its plan view.
- Outliner view browser; plan + section windows bound to `view` id (section line no longer in config); chrome view select.
- Inference `🖼️view-linework` (clip, vertical, hidden-lines, frame, filters, plans, cut) in aggregate + model-graph;
  16 unit tests including gating (`editing_one_view_computes_that_view_only`).
- en+de labels.

## Missing
1. SVG per view: `🚪️io/📤️export/🎨️svg/🦀️.rs` `export_svg`/`plans_to_svg` still emit per storey from plan-linework.
2. Examples: no `views` in `📚️examples/{🏡️house,🏢️office,🎬️demo}`; `T/r10-w12-views-examples.ts` not applied; test
   `the_example_views_are_the_ones_the_command_makes` absent.
3. View inference oracle (`🔮️oracles/🔣️.json`) + `🧪️tests/<emoji>infer-bim-1-views/🥒️.feature`.
4. Unused label `section_line` (`🗣️terminology/🦀️.rs:294`); warning `🗺️plan/🦀️.rs:138`.

Last log check8 (23:35): one error in `🧵️gestures/🧱️chain/🦀️.rs:120` (not WP-12; line changed since).

## Remaining tasks
Gate re-run → examples via generators (+ example test) → SVG per view from view-linework (+ feature/unit test) → view
oracle (shapely) + feature → label cleanup → generators (`r3-f1-gen-mutation-facets.ts`, `r3-f1-gen-oracle.ts`,
`r3-f1-gen-feature.ts`, `r3-f1-check-names.ts`) → gate.

Coordinator ruling: ViewKind superset (Plan, CeilingPlan, Section, Elevation, Orthographic, Perspective) is accepted.
