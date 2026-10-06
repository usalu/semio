# Hierarchy Shared Table Type Classification — 2026-10-05

Read-only actual source versus current family schema audit. semio-viz-hierarchy.sty render path semio/viz/hierarchy/render lines1339–1356 declares height/nodeRadius/nodeWidth/nodeHeight/inset/innerRadius/opacity as fp, labels/leafLabels as bool. Reset lines1361–1379 uses frame height or45mm, node radius1.4mm, width16mm,height6mm,inset4mm, innerRadius6mm, opacity1, labels true,leafLabels false. Thus schema string nodeRadius/nodeWidth/nodeHeight/labels is a real typed mismatch, not an auto-valued hybrid.

Layout path lines650–678 declares separationOther fp,separationDepth bool, round bool,relaxation int, padding code populating fp values. Reset lines690–703 uses2,false,false,24,zero respectively. radius alone is tl with auto default and later numeric use, so its auto/expression string is source-backed. Token-list storage cannot by itself imply plain text: strokeWidth is TeX dimension, radius hybrid, and identifiers/choice fields have distinct grammar. This audit corrects an initial message's guessed relaxation10 to actual24 immediately after reading reset.

No product/schema/table mutation performed. Actual schema corrections belong Catalogue; runtime controls and complete syntax taxonomy remain pending. Names-only shared coverage cannot override these exact property/default semantics.
