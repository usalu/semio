# Missing Family Data Source Default Audit

Explicit source mapping: 39 families = eleven network/flow, twenty-seven diagram/interaction/infographic and one axis. Authored input contains exact source hashes and owner bindings. Metadata was not used to infer defaults. All are string table identifiers consumed by layout/table readers; no inert selector was found in this bounded set. Axis derives scales from its selected table and reads its rows, so it is effective.

| Family | Native Reset Default | Source | Consumer |
| --- | --- | --- | --- |
| adjacency-matrix | demo-graph-community | semio-viz-network-matrix.sty | semio_viz_layout_structure with selected data; shared network binding |
| node-link-matrix | demo-graph-community | semio-viz-network-matrix.sty | semio_viz_layout_structure with selected data; shared network binding |
| biofabric | demo-graph-community | semio-viz-network-matrix.sty | semio_viz_layout_structure with selected data; shared network binding |
| arc-diagram | demo-graph-community | semio-viz-network-arc.sty | semio_viz_layout_structure with selected data; shared network binding |
| hive-plot | demo-graph-community | semio-viz-network-arc.sty | semio_viz_layout_structure with selected data; shared network binding |
| chord | demo-graph-community | semio-viz-network-chord.sty | semio_viz_layout_structure with selected data; shared network binding |
| dependency-wheel | demo-graph-community | semio-viz-network-chord.sty | semio_viz_layout_structure with selected data; shared network binding |
| edge-bundled | demo-graph-community | semio-viz-network-bundling.sty | semio_viz_layout_structure with selected data; shared network binding |
| sankey | demo-flow | semio-viz-flow-sankey.sty | semio_viz_layout_structure with selected data; shared network binding |
| alluvial | demo-flow | semio-viz-flow-sankey.sty | semio_viz_layout_structure with selected data; shared network binding |
| parallel-sets | demo-flow | semio-viz-flow-sankey.sty | semio_viz_layout_structure with selected data; shared network binding |
| cycle | demo-diagram-cycle | semio-viz-diagram-flowchart.sty | selected table rows/cells in family renderer; diagram layout where applicable |
| decision-table | demo-diagram-rules | semio-viz-diagram-flowchart.sty | selected table rows/cells in family renderer; diagram layout where applicable |
| fishbone | demo-diagram-cause | semio-viz-diagram-flowchart.sty | selected table rows/cells in family renderer; diagram layout where applicable |
| flow | demo-diagram-flow | semio-viz-diagram-flowchart.sty | selected table rows/cells in family renderer; diagram layout where applicable |
| infographic-icon | demo-infographic | semio-viz-infographic.sty | selected table rows/cells in family renderer; diagram layout where applicable |
| infographic-illustration | demo-illustration | semio-viz-infographic.sty | selected table rows/cells in family renderer; diagram layout where applicable |
| infographic-list | demo-infographic | semio-viz-infographic.sty | selected table rows/cells in family renderer; diagram layout where applicable |
| infographic-number | demo-infographic | semio-viz-infographic.sty | selected table rows/cells in family renderer; diagram layout where applicable |
| logic-tree | demo-diagram-tree | semio-viz-diagram-flowchart.sty | selected table rows/cells in family renderer; diagram layout where applicable |
| matrix-grid | demo-matrix-grid | semio-viz-diagram-process.sty | selected table rows/cells in family renderer; diagram layout where applicable |
| notation-board | demo-board | semio-viz-diagram-process.sty | selected table rows/cells in family renderer; diagram layout where applicable |
| notation-railroad | demo-railroad | semio-viz-diagram-process.sty | selected table rows/cells in family renderer; diagram layout where applicable |
| pm-board | demo-pm-board | semio-viz-diagram-process.sty | selected table rows/cells in family renderer; diagram layout where applicable |
| pm-gantt | demo-pm-gantt | semio-viz-diagram-process.sty | selected table rows/cells in family renderer; diagram layout where applicable |
| pm-network | demo-pm-gantt | semio-viz-diagram-process.sty | selected table rows/cells in family renderer; diagram layout where applicable |
| schedule-calendar | demo-schedule | semio-viz-diagram-process.sty | selected table rows/cells in family renderer; diagram layout where applicable |
| schedule-timedistance | demo-timedistance | semio-viz-diagram-process.sty | selected table rows/cells in family renderer; diagram layout where applicable |
| seating | demo-seating | semio-viz-diagram-process.sty | selected table rows/cells in family renderer; diagram layout where applicable |
| stage | demo-diagram-cycle | semio-viz-diagram-flowchart.sty | selected table rows/cells in family renderer; diagram layout where applicable |
| state-overview | demo-state-series | semio-viz-interactionstate.sty | selected table rows/cells in family renderer; diagram layout where applicable |
| state-selection | demo-diagram-flow | semio-viz-interactionstate.sty | selected table rows/cells in family renderer; diagram layout where applicable |
| state-sequence | demo-state-frames | semio-viz-interactionstate.sty | selected table rows/cells in family renderer; diagram layout where applicable |
| swimlane | demo-diagram-flow | semio-viz-diagram-flowchart.sty | selected table rows/cells in family renderer; diagram layout where applicable |
| uml-class | demo-uml-class | semio-viz-diagram-uml.sty | selected table rows/cells in family renderer; diagram layout where applicable |
| uml-sequence | demo-uml-sequence | semio-viz-diagram-uml.sty | selected table rows/cells in family renderer; diagram layout where applicable |
| uml-timing | demo-uml-timing | semio-viz-diagram-uml.sty | selected table rows/cells in family renderer; diagram layout where applicable |
| uml-usecase | demo-uml-usecase | semio-viz-diagram-uml.sty | selected table rows/cells in family renderer; diagram layout where applicable |
| axis | demo | semio-viz-guide.sty | 1280–1286 scale-from-table bindings;1316 table cells |

Network inheritance: network-graph432 installs data setter; reset477 sets demo-graph-community. Matrix/arc/chord/bundling reset calls that shared reset without a data override. Sankey reset60 overrides demo-flow; alluvial/parallel-sets dispatch the same reset/render and selected table enters layout351. Diagram flow reset196 sets demo-diagram-flow; swimlane and state-selection invoke it. Remaining defaults are explicit literals in their selected family entrypoint before caller keys. These are source/default proofs, not fresh runtime customization claims.
