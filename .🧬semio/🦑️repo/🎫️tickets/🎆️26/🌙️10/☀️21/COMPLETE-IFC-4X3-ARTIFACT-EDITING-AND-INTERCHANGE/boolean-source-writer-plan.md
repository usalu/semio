# Original Source Mesh Writer Plan

Workspace69 is independently closed at928actual laws/strict14; all generic Boolean14owned files remain unchanged. Root owns the public Boolean AST, policy control and geometry dispatcher. Low's rounded75 readers are terminal, so the six source generators are released for this next refactor.

## One Sink and One Source Algorithm

New `geometry/mesh-writer.ts` exports first-party `IfcMeshWriter<T>` with `point(index,x,y,z)`, `triangle(index,a,b,c)`, `finish():T`, `close()`, and `IfcMeshWriterFactory<T>=(vertexCount,triangleCount)=>IfcMeshWriter<T>`. `ifcArrayMeshWriterFactory(maxValues)` validates exact total capacity before its arrays are born; it stores only the actual endpoint's positions/flat indices. Root's separate factory creates the existing prepaid session builder and moves its opaque result. A writer is a closed buffer owner, not an IFC/schema/topology certificate.

Generic source methods are `ifcCsgPrimitiveWriteSteps(graph,primitive,world,scale,policy,make)`, `ifcRectangleExtrusionWriteSteps(graph,solid,world,scale,make)`, `ifcCircleExtrusionWriteSteps(graph,solid,world,scale,policy,make)`, `ifcRoundedRectangleExtrusionWriteSteps` with the same circle signature, `ifcCircularSolidWriteSteps(source,policy,make)` and `ifcConvexSolidWriteSteps(source,make)`. All are generic in the terminal sink result. Current raw endpoints delegate to these methods with the array factory; no second source algorithm or raw-mesh-to-builder copy is permitted.

The exact rectangular raw endpoint gains one required trailing `maxValues:number`; root will pass remainingValues at its sole geometry.ts dispatcher call. No optional compatibility default is added. Other existing raw endpoints retain their actual maxValues parameter. Source owner/type/dimension/precision checks precede the same writer creation boundary as before. Factory creation itself captures a primitive grant; its actual invocation performs capacity admission after source shape/counts are known.

## Exact Count and Precision Frontier

Block/rectangle have8vertices12triangles60numeric values. An n-boundary cylinder has2nvertices/(4n-4)triangles/(18n-12)values; cone has(n+1)/(2n-2)/(9n-3). At72chords this is645 for cone and1284 for cylinder. Rounded boundary n is the exact existing convexProfileSize including nonzero straight joins; no cap/default/sample change.

The convex point formula becomes one indexed function using the original profile transform, displaced layer in solid coordinates, represented frame and SI scale in the same arithmetic order. Emission computes one fixed scratch point and writes it immediately. Triangle precision checks recompute only its three fixed scratch points from that same function; no second full positions backing is retained. Existing triangle ordering and negative extrusion reversal are preserved. These are pre-reflection represented indices: raw publication and root's owned factory each apply the actual world orientation once at their existing owner boundary, never inside this source writer.

Writer construction preflights before point/triangle births. Every generator closes its entered writer on completion, return or failure; a failed original operation preserves its first thrown value, including undefined, if cleanup also refuses. Finish moves the actual endpoint backing; subsequent close is idempotent and does not release a returned owned handle. Root advances original IO and session checkpoints for each source yield. The sink does not receive an arbitrary generator, source substitution, fresh policy wallet or free source certificate.

## Test Order and Evidence

Add closed language-neutral writer/count programs before production, including Block60, cone645, cylinder1284, rounded full/equality join counts, one-less refusal before emitter birth, duplicates/holes, exact indexed stream order, return/undefined failure close, and genuine owned session retention. Use the existing SQLite capacity oracle pattern for count conservation, Three/Manifold for complete geometric outputs and actual raw-vs-owned word equality. New public WriteSteps red must precede source changes. Existing primitive/rounded/circle/rectangle consumers and strict imports run after all six source modules are coherent; root's changing dispatcher/control are coordinated references, not independently owned source.

Do not touch generic Boolean production, public AST/control/dispatcher, native implementation, registry or capability flags. New registration remains root-owned after independent closure. Native compiler/runtime/funding proof remains false.
