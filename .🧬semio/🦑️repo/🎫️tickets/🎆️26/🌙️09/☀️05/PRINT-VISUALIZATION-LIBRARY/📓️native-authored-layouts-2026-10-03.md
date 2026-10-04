# Native Authored Layout Geometry

Canonical native chart inference emits `SemioVizLayout{algorithm}{prepared-table}{prepared-table}[authored options]`. Previously these calls reached low-level algorithms that expected different structures, bindings and option spellings. Pie and DAG were absent from the layout registry. Hierarchy, spatial and projection routines published hierarchy/point/grid/geometry structures instead of plot tables.

## Native Ownership

| Owner | Authored algorithms and behavior |
| --- | --- |
| `semio-viz-layout.sty` | Detect authored tables, validate the schema option vocabulary, snapshot rows before in-place publication, preserve columns, serialize complete inferred tables, and delegate bin/stack to canonical authored transforms. Structure dispatch remains the native structure entry point. |
| `semio-viz-transform.sty` | Interpret authored scalar integer histogram thresholds as bin counts before invoking the existing histogram solver. Explicit threshold lists retain their boundary semantics. |
| `semio-viz-shape.sty` | Pie delegates to the existing D3-equivalent angular solver and emits original cells, start/end angles, padding, center, radii and plot angle/radius columns. |
| `semio-viz-hierarchy.sty` | Tree, cluster, treemap, partition and pack construct topology from authored bindings, resolve native geometry and publish original cells plus value, depth, coordinates, bounds, radii, area and parent coordinates. Bundling emits hierarchy link endpoints and serialized paths in cluster coordinates. |
| `semio-viz-network.sty` | Arc, DAG, force and chord translate schema controls into owned graph solvers. Arc spacing uses node count; vertical origins use the chosen axis. Force honors initial node positions, seed, authored force order, iteration controls and collision radii. Chord transposes its matrix while retaining original node order. Nodes/groups/links publish the selected output schema. |
| `semio-viz-flow.sty` | Sankey emits selected node bounds or weighted link endpoints. Alluvial constructs stages with stable identities and aggregates duplicate stage pairs before the same flow solver. |
| `semio-viz-spatial.sty` | Hexbin emits centers, counts and radii. Jitter uses D3's seeded 32-bit LCG for both coordinates. Beeswarm resolves exact collision candidates on either axis. Voronoi, Delaunay and hull emit ordered vertex rows with detail, value and serialized polygons. Contour and density publish padded complete contours in plot coordinates, sharing threshold generation, ring extraction and output serialization. Density supports the four declared product kernels and raster controls. |
| `semio-viz-geo.sty` | Projection accepts authored coordinate selectors, projection, scale, translation, rotation, center and reflection, retaining original row cells and replacing x/y with projected geometry. |
| `semio-viz-showcase.sty` | The two internal long-form stack/histogram showcases explicitly call structure dispatch; their source tables and primitive grammar are already native structure inputs. |

All mathematics remains in the existing native domain owners. The layout registry contains dispatch, validation and table publication only. No standalone TypeScript numerical runtime was added. Bin and stack share `semio_viz_transform:nnn` rather than implementing a second transform engine. Progress and cancellation remain owned by the canonical native inference service and its cancellable compilation process.

## Neutral Verification

`🧪️tests/🧬️chart-layout-grammar/🔣️.json` defines all 24 authored layout inputs; `⚙️.json` adds 18 customization inputs. The adjacent TypeScript test adapter exports `nativeLayoutGrammarChecks(workDir)`. It compiles the actual in-place TeX calls once and reads their published numeric table columns. Independent D3 packages generate the reference results; the DAG reference uses existing dagre. D3 random and polygon are dev-only test dependencies added in the existing print TypeScript package.

The 18 controls exercise pie sorting, angular spans, center and radii; vertical arc origin and spacing; chord transposition; force initial positions, alpha, decay, charge and collisions; tree/cluster separation and leaves; treemap tiling, rounding and edge padding; partition/pack padding; multi-stage alluvial aggregation; Sankey dimensions, iterations and alignment; alternate coordinate/value selectors; jitter seed and amount; beeswarm axis; Voronoi bounds; contour raster/threshold controls; density extent, cell size and uniform kernel; geographic translation, rotation, center and both reflections.

The adapter checks 129 numeric columns and eight coupled polygon vertex projections (137 checks). Coupled projections retain x/y/value relationships while canonicalizing only vertex order. Native diagnostics reject unknown authored controls, absent required columns, nonpositive geometric controls and incomplete contour rasters.

## Recorded Execution

The initial compiled all-24 document produced 67 matching columns out of 72. Five bin/stack columns were empty because primitive options had consumed authored inputs incorrectly. Delegating to the canonical authored transform fixed stack and made bin output nonempty. The next actual compile of all 42 inputs matched 126 of 129 columns. The three remaining comparisons diagnosed an authored integer threshold interpreted as an explicit boundary rather than a bin count; the canonical transform lane owns that correction. A chord customization failure exposed reversed first-appearance node order during transposition; transposing the owned matrix fixed the four angular comparisons.

The canonical bin threshold correction is now applied in the transform owner. The final actual pinned native compile succeeded for all 42 inputs, and its independent comparisons finished with `[DEBUG] checks 137 failures 0`, exit code 0. This includes 129 numeric columns and eight coupled polygon checks. The coupled checks round only the sorting keys to prevent equivalent floating-point vertex values from changing their order; comparison still uses the original values with the declared tolerance.

Execution invoked `nativeLayoutGrammarChecks` through Bun, compiled the real `SemioVizLayout` calls in place, and compared every returned subject/oracle vector. The command log is `🗑️generated/native-layout-check.log`; generated TeX, PDF, compiler logs and probe JSONL are under `🗑️generated/native-layout-check`. These are tool outputs for cleanup. This report retains the observed result independently. The coordinating agent's canonical Nx native grammar and full visualization catalogue runs are separate gates; this report does not claim their outcomes.

## Exact Inventory

Updated native sources: `semio-viz-layout.sty`, `semio-viz-shape.sty`, `semio-viz-hierarchy.sty`, `semio-viz-network.sty`, `semio-viz-flow.sty`, `semio-viz-spatial.sty`, `semio-viz-geo.sty`, `semio-viz-transform.sty` (one authored bin-count conversion), and `semio-viz-showcase.sty` under the print product's `🖋️latex` directory.

Created neutral case assets: `🧪️tests/🧬️chart-layout-grammar/🔣️.json`, `⚙️.json`, `🥒️.feature`, and `🟦️.ts`.

Registered the layout helper in `🧪️tests/🧬️native-chart-grammar/🟦️.ts`, after its transform checks. The existing native-grammar executable task invokes this path; no new task or launch configuration is needed.

Updated dev-only package metadata: `📦️packages/🟦️typescript/package.json` and root `bun.lock`. Bun preserved the inference package's relocated workspace path. Its regenerated lock removed two registrations for absent repo-library cache fixtures; this was reported to the coordinating agent for shared-lock review.

Retained evidence: this Markdown report. No Git mutation, worktree, AGENTS edit, runtime library dependency, additional executable command or standalone script was introduced.
