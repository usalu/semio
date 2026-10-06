# Three Mode Families Native Default Vector Contract — 2026-10-05

Read-only source adjudication. These are proposed authored examples and neutral contracts, not evidence that production dispatch works. Existing tuple painters remain the owner. Source: biology lines 608–700; engineering lines 26–240; mathematics lines 1003–1114; catalogue stock options listed in the companion three-mode audit. The public vocabularies have exactly 4, 6, and 8 modes.

## Override And Relation Contract

Select defaults after mode is parsed and before node/element consumption. An internal sentinel must distinguish omitted lists from explicitly empty lists; do not use clist emptiness as omission. Each nodes/edges/elements override is independently preserved. Explicit nodeShape, unit, coordinates, relation/style tokens and labels also win over default selection. Mode supplies a semantic default example and default relation vocabulary, not a permission to reinterpret explicitly typed relations. Unknown modes need schema/guard rejection, not generic fallback. Empty nodes with default edges requires a documented guard or endpoint rejection, never invisible fabricated nodes.

## Pathway Defaults

Tuple forms nodes=id/x/y/label; edges=from/to/kind/label. Coordinates millimetres at unit=1.

| Mode | Nodes | Edges | Meaning |
|---|---|---|---|
| pathway | a/0/28/ligand,b/0/14/receptor,c/22/14/kinase,d/44/14/TF,e/44/0/gene,f/22/28/inhibitor | a/b/activate/,b/c/activate/,c/d/activate/P,d/e/activate/,f/c/inhibit/ | Preserve existing activation cascade and inhibition bar. |
| food-web | p/0/0/producer,h/22/14/herbivore,o/22/-14/omnivore,c/44/0/carnivore | p/h/flux/,p/o/flux/,h/c/flux/,o/c/flux/,h/o/flux/ | Arrows point from consumed resource to consumer: energy transfer, not predator to prey. |
| metabolic | g/0/0/glucose,p/22/0/pyruvate,a/44/0/acetyl-CoA,l/22/-18/lactate,t/44/18/TCA | g/p/flux/glycolysis,p/a/flux/,p/l/flux/,a/t/flux/ | Directed substrate/product conversion, branching pathway; no activation/inhibition semantics on conversion edges. |
| interaction | a/0/0/P1,b/22/14/P2,c/44/0/P3,d/22/-14/P4 | a/b/plain/,b/c/plain/,c/d/plain/,d/a/plain/,b/d/plain/ | Undirected protein interaction, not activation cascade. |

Stock phylogenetic-network needs its own explicitly authored rooted ancestral tree/network nodes and plain ancestry edges, not protein interaction defaults. Food-chain needs an explicit single producer→herbivore→carnivore path; food-web keeps branching. Ecological-network may use food-web energy convention. Metabolic-network-diagram must inherit the metabolic conversion example. These semantic distinctions require handcrafted catalogue tuples; captions and mode alone cannot distinguish every stock kind.

## Circuit Defaults

Tuple elements=type/x1/y1/x2/y2/label, millimetres. Existing schematic vector source lines 56–59 is retained unchanged for schematic. Other five modes can use existing stock vectors from catalogue, with the following corrections.

| Mode | Proposed exact vector | Required semantics |
|---|---|---|
| logic | and/0/0/12/0/A,wire/12/0/20/0/,not/20/0/32/0/B,wire/32/0/40/0/,or/40/0/52/0/C | Connected signal chain, gate symbols retain existing symbol painter. |
| ladder | wire/0/0/0/30/,wire/40/0/40/30/,contact/0/24/16/24/K1,wire/16/24/24/24/,coil/24/24/40/24/Q1,contact/0/12/16/12/K2,wire/16/12/24/12/,coil/24/12/40/12/Q2 | Parallel rungs between supply rails; resistor is not a relay contact. Existing painter lacks contact/coil types: meaningful ladder semantics requires adding these symbols in this owner/schema, not renaming resistors. |
| wiring | node/0/0/0/0/,wire/0/0/24/0/L1,node/24/0/24/0/,wire/24/0/24/18/L2,node/24/18/24/18/ | Conductor route and junctions, preserve orthogonal terminal positions. |
| block | block/0/0/16/0/in,signal/16/0/24/0/,block/24/0/44/0/G,signal/44/0/52/0/,block/52/0/68/0/out | Directed input→transfer→output connectors. Existing wire is undirected; add a directed signal element or mode-conditioned default connector only. |
| signal-flow | node/0/0/0/0/x,signal/0/0/20/0/a,node/20/0/20/0/y,signal/20/0/40/0/b,node/40/0/40/0/z | Directed weighted graph; plain wire currently loses direction. Same existing-owner signal extension. |

Preserve explicitly authored wire as a wire in every mode. Stock ladder and signal-flow/block vectors also need the semantic symbol/connector corrections; selecting a new default does not repair explicit stock vectors.

## Abstract Defaults

Tuple nodes=id/x/y/label, edges=from/to/style/label; unit=18. Suggested exact defaults:

| Mode | Nodes | Edges |
|---|---|---|
| commutative | a/0/1/A,b/1/1/B,c/0/0/C,d/1/0/D | a/b/arrow/f,a/c/arrow/g,b/d/arrow/h,c/d/arrow/k |
| string | a/0/1/f,b/1/1/g,c/0/0/,d/1/0/ | c/a/wire/,d/b/wire/,a/b/wire/ |
| tensor | a/0/1/A,b/1/1/B,c/0/0/C,d/1/0/D | a/b/leg/,a/c/leg/,b/d/leg/,c/d/leg/ |
| hasse | a/0/0/0,b/-0.7/1/a,c/0.7/1/b,d/0/2/1 | a/b/order/,a/c/order/,b/d/order/,c/d/order/ |
| cayley | a/0/0/e,b/1/0/r,c/1/1/r2,d/0/1/r3 | a/b/arrow/r,b/c/arrow/r,c/d/arrow/r,d/a/arrow/r |
| dynkin | a/0/0/,b/1/0/,c/2/0/,d/3/0/ | a/b/order/,b/c/dynkin-double/,c/d/order/ |
| proof | a/-0.7/1/P,b/0.7/1/Q,c/0/0/R | a/c/order/,b/c/order/ |
| rewrite | a/0/1/s,b/1/1/t,c/0/0/u,d/1/0/v | a/b/double/r,a/c/dashed/,b/d/double/r,c/d/double/r |

Default nodeShape: commutative/string/proof/rewrite none; tensor/cayley/dynkin/hasse circle, unless explicitly authored. For a proof-tree default using standard inference-rule notation, add a horizontal rule joining premise/conclusion layout in this owner and validate rule span/vertical separation. A declared derivation-tree graph convention can instead use inference nodes and explicit rooted incidence; source prose alone does not mandate rule bars. The selected convention must distinguish a derivation from the old commutative-square defaults. Existing double style is double with an arrow head, unsuitable for an undirected Dynkin double bond; add separate dynkin-double undirected double stroke. Coxeter labelled order edges can remain undirected; Dynkin and Coxeter stock vectors must have appropriate multiplicity/order geometry. Existing hook style is a dashed arrow rather than a hook head: independent accepted-style defect, not fixed by mode vectors.

## Neutral TDD Scope

Each of 18 omitted-list modes needs actual body path/glyph coordinates, directed-head or undirected absence, relation multiplicity and graph incidence checked against authored tuple geometry using an independent graph library for node/edge incidence plus existing D3 point/window transforms. Test explicit nodes-only, edges-only, all-list, empty-list and shape overrides per owner. Stock phylogenetic/food-chain/metabolic/ladder/signal-flow/Dynkin/proof cases need dedicated semantic assertions. Different paint hashes alone prove difference, not any of these meanings. Preserve both explicit locales/themes and existing shared physical frame behavior.

## Additional Circuit Terminal Geometry Audit

Current generic gate painter engineering lines 219–245 draws AND/OR/XOR/NAND half-stadium ending at `(length/2+3.4,0)`; NAND bubble extends to `length/2+4.6`. It does not call the existing leads helper. For length 12, the AND body ends at x=9.4 and NAND bubble at x=10.6, while the declared output terminal and following wire start at x=12. A connected logic default vector therefore exposes a real terminal gap, not a mode defect by itself. Independent circuit neutral assertions must check terminal connectivity against actual path endpoints, allowing the inversion bubble as a component, rather than validating only the reported tuple terminals. Unknown element types currently fall into generic gate painting; schema/guard must reject unsupported literal types instead of drawing an AND body.

Dynkin adjudication qualification: a directed double bond is legitimate for a non-simply-laced Dynkin diagram when the arrow identifies root-length direction. Existing arrowed double is therefore not universally an incorrect Dynkin symbol. The separate undirected double style is needed only for an authored/default undirected multiplicity contract; source/tests must state the chosen root-length convention before rejecting an existing directed stock bond. Do not convert every explicit double edge to an undirected one. Likewise string/tensor graphs can be valid drawings using the existing line primitives; mode defaults supply the appropriate incidence example rather than requiring arbitrary extra decoration.

