@capability-diagrams2d-window
@oracle-shapely-numpy
@comparison-floating-point-v1
Feature: 2D diagrams window
  A Diagram2d is lowered to Canvas2d layers with axes, ticks, grid, series, markers, legend and units per locale,
  hit-tested into selection events and mirrored by an accessible table twin.

  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:
  - Pointer distances equal shapely: Given pick shapes of kind path, area, point and rectangle When the distance to probe points is computed Then it equals the shapely distance within 1e-9
  - Logarithmic ticks equal numpy logspace: Given a log axis from 0.1 to 10000 When the decade ticks are computed Then they equal numpy.logspace
  - Lowering is bilingual: Given the interaction diagram When it is lowered for locale "<locale>" Then the layers contain the title "<title>" and the marker label "<marker>" Examples (2 rows: en-interaction-load, de-interaktion-last)
  - Marker selection carries its check: Given a lowered diagram with a marker bound to check "c1" When the pointer is near the marker Then the selection event names the marker, check "c1" and the diagram focus
  - Table twin and description: Given the interaction diagram When the table twin and the accessible description are built in German Then numbers use decimal commas and the description names axes, series and markers
  - Golden layers are stable: Given the committed golden layers When the diagram is lowered Then the bytes equal the golden
  - A marker row of the table twin is an editable cell: Given a diagram with a marker bound to the field "placements[id=p1].x" shown in metres And a marker without a binding When the diagrams window is built in English or German Then the bound marker row holds a text cell named "Placement P1, edit value" or "Platzierung P1, Wert bearbeiten" showing "2.5 m" And the unbound marker row stays read-only
  - A typed cell value commits like a drag: Given the cell of the bound marker shows "2.5 m" When "350 cm" is committed Then exactly one write of "placements[id=p1].x" with 3.5 is committed And committing "2.5 m" commits nothing And a value above the axis limit is clamped And a unit of another dimension is refused
