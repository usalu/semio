How would you design a spatial query language for this inspired by Cypher?

# Geometry

Topology: A Topology is an abstract superclass.

## Editable

Vertex: A Vertex is a zero-dimensional entity equivalent to a geometry point.

Edge: An Edge is a one-dimensional entity defined by two vertices. It is important to note that while a topologic edge is made of two vertices, its geometry can be a curve with multiple control vertices.

Wire: A Wire is a contiguous collection of Edges where adjacent Edges are connected by shared Vertices. It may be open or closed and may be manifold or non-manifold.

Face: A Face is a two-dimensional region defined by a collection of closed Wires. The geometry of a face can be flat or undulating.

Shell: A Shell is a contiguous collection of Faces, where adjacent Faces are connected by shared Edges. It may be open or closed and may be manifold or non-manifold.

Cell: A Cell is a three-dimensional region defined by a collection of closed Shells. It may be manifold or non- manifold.

CellComplex: A CellComplex is a contiguous collection of Cells where adjacent Cells are connected by shared Faces. It is non-manifold.

Cluster: A Cluster is a collection of any topologic entities. It may be contiguous or not and may be manifold or non-manifold. Clusters can be nested within other Clusters.

## Non-editable

Surfaces are derived faces that are a combination of Exposure (External or Internal) and Stance (Horizontal or Vertical).

e.g. two coplanar faces are merged into a single surface

e.g. when two cells intersect the surface will not be the complete face but it is split into external and internal faces.

Surfaces are just a different way of "splitting the faces semantically". e.g. the total area or shape doesnt change

Parts are derived cells that are a combination of Overlap (None, Difference, Intersection)

Parts are just a different way of "splitting the cells semantically". e.g. the total volume or shape doesnt change

Designing a Cypher-inspired query language for this topological model is a brilliant approach. Boundary Representation (B-rep) and cell complexes are fundamentally graph structures—entities are nodes, and their connections (boundaries, incidences, adjacencies) are edges.

Because Cypher is inherently designed for pattern matching on graphs, we can map your geometric definitions directly to nodes, relationships, and properties. Let's call this conceptual language TopoCypher.

Here is how I would architect the schema and syntax for TopoCypher.

1. The Graph Schema

To make this work in a Cypher-like environment, we need to strictly define how your entities map to Graph Nodes and Graph Relationships.
Nodes (Labels)

Every entity in your model becomes a Node Label.

    Editable: (:Vertex), (:Edge), (:Wire), (:Face), (:Shell), (:Cell), (:CellComplex), (:Cluster)

    Non-editable (Semantic): (:Surface), (:Part)

Relationships (Directed Edges)

We need a standardized set of relationships to navigate the hierarchy and derivations.
Relationship Description Example Pattern
[:BOUNDED_BY] Moves down the dimensional hierarchy. (f:Face)-[:BOUNDED_BY]->(w:Wire)
[:CONTAINS] Captures collections and clusters. (c:Cluster)-[:CONTAINS]->(cel:Cell)
[:SHARES] Expresses lateral adjacency. (w1:Wire)-[:SHARES]->(v:Vertex)<-[:SHARES]-(w2:Wire)
[:DERIVES] Connects editable geometries to their semantic outputs. (f:Face)-[:DERIVES]->(s:Surface)
[:MERGED_FROM] Used when multiple coplanar faces form one surface. (s:Surface)-[:MERGED_FROM]->(f:Face)
Properties

Properties will store the geometric and semantic data.

    Topological Booleans: isClosed, isManifold (e.g., on Wires, Shells, Cells).

    Semantic Enums: exposure (External/Internal), stance (Horizontal/Vertical), overlap (None, Difference, Intersection).

    Metrics: area, volume, length.

2. Querying the Editable Topology

Here is how TopoCypher handles complex structural queries using standard Cypher pattern matching.

Example A: Finding non-manifold edges.
A non-manifold edge is an edge shared by more than two faces. You can query this by counting the inbound relationships.
Cypher

MATCH (f:Face)-[:BOUNDED_BY]->(:Wire)-[:CONTAINS]->(e:Edge)
WITH e, count(f) as connectedFaces
WHERE connectedFaces > 2
RETURN e, connectedFaces

Example B: Traversing a Cell Complex.
Find all contiguous Cells in a CellComplex that share a specific Face.
Cypher

MATCH (c1:Cell)-[:BOUNDED_BY]->(:Shell)-[:CONTAINS]->(f:Face)
<-[:CONTAINS]-(:Shell)<-[:BOUNDED_BY]-(c2:Cell)
WHERE c1 <> c2
RETURN c1, c2, f

3. Querying the Non-editable (Semantic) Topology

Your derived entities (Surfaces and Parts) are where a graph query language shines, because it allows you to abstract away the messy triangulation or underlying faces and ask purely semantic questions.

Example C: Calculating the area of the building envelope.
You want the total area of all external surfaces (e.g., ignoring internal partitions).
Cypher

MATCH (s:Surface)
WHERE s.exposure = 'External'
RETURN sum(s.area) AS EnvelopeArea

Example D: Finding flat roofs vs. walls.
Coplanar faces are already merged into Surfaces in your model. You can easily query specific stances.
Cypher

MATCH (c:Cell)-[:DERIVES]->(s:Surface)
WHERE s.exposure = 'External' AND s.stance = 'Horizontal'
RETURN c, s AS FlatRoof

Example E: Analyzing Intersections (Clash Detection).
You defined Parts as derived cells based on overlap. If two cells intersect, we can query that specific volumetric overlap directly.
Cypher

MATCH (c1:Cell)-[:DERIVES]->(p:Part)<-[:DERIVES]-(c2:Cell)
WHERE p.overlap = 'Intersection' AND p.volume > 0.5
RETURN c1, c2, p.volume AS ClashVolume

4. Syntactic Sugar (Language Extensions)

If you are building a custom parser for this, writing out the full topological hierarchy (Cell -> Shell -> Face -> Wire -> Edge) every time can become tedious.

I recommend introducing transitive spatial operators into your Cypher implementation. Under the hood, they expand into the full graph traversal, but they make the language much more readable for spatial engineers.

    --[:ADJACENT_TO]-- (Abstracts finding shared boundaries).

    --[:HAS_VERTEX]-- (Bypasses intermediate Wires/Faces to jump straight to points).

Using the abstraction:
Cypher

// Instead of traversing Shells and Wires:
MATCH (c1:Cell)-[:ADJACENT_TO]-(c2:Cell)
RETURN c1, c2

// Bypassing the hierarchy to find all points of a Cell:
MATCH (c:Cell {id: 'Room_101'})-[:HAS_VERTEX*1..4]->(v:Vertex)
RETURN DISTINCT v

Designing the query language this way keeps the syntax familiar to data scientists and graph engineers while respecting the strict mathematical rules of non-manifold topology.

Are you planning to use this query language strictly for analytical queries (read-only), or do you want to extend it to mutate the graph—like using Cypher's CREATE or MERGE to actually construct and Boolean new geometries on the fly?

It should work both for read and write. But direct modifcation of the graph is not possible, only over a safe command layer.

That is an excellent architectural constraint. Directly modifying a topological B-rep graph using raw Cypher CREATE or DELETE clauses is incredibly risky. If a user deletes an edge without deleting the dependent face, or creates a face without a fully closed wire, the strict geometric validity of the model collapses entirely.

By routing modifications through a safe command layer, you are effectively applying the CQRS (Command Query Responsibility Segregation) pattern to your graph. TopoCypher handles the "Query" side, while a strictly validated API handles the "Command" side.

Here is how you can design the write-operations in TopoCypher to interface cleanly with that command layer.

1. The Syntax: Stored Procedures as Commands

In a Cypher environment (like Neo4j or Memgraph), the standard way to interface with an external command layer or bounded context is through Custom Procedures (CALL ... YIELD ...).

Instead of allowing users to write CREATE (f:Face {id: 123}), you force them to call a procedure that encapsulates your complex topological logic.

Example A: Creating a Face from existing Vertices
The user passes the points. The command layer verifies they are coplanar, creates the intermediate Edges and the Wire, ensures it is closed, and then generates the Face.
Cypher

MATCH (v1:Vertex {id: 'v1'}), (v2:Vertex {id: 'v2'}),
(v3:Vertex {id: 'v3'}), (v4:Vertex {id: 'v4'})
CALL topo.constructFace([v1, v2, v3, v4]) YIELD face, wire, edges
RETURN face.id, wire.isClosed

Example B: Boolean Operations (The "Write" for Semantic Parts)
To create an intersection, the user doesn't manually map the [:DERIVES] relationships. They call a command that calculates the intersection, writes the new Part node, and maps the graph correctly.
Cypher

MATCH (c1:Cell {name: 'Duct'}), (c2:Cell {name: 'Wall'})
CALL topo.boolean.intersect(c1, c2) YIELD part AS ClashPart
RETURN ClashPart.volume

Example C: Destructive Commands (Safe Deletion)
If a user wants to delete a wall (Cell), the command layer must recursively cascade the deletion down to Shells, Faces, and Edges—unless those lower-dimensional entities are shared by another Cell (e.g., a shared floor).
Cypher

MATCH (c:Cell {id: 'Wall_A'})
CALL topo.removeCell(c) YIELD deletedNodeCount, orphanedEntitiesCleaned
RETURN deletedNodeCount

2. Responsibilities of the Safe Command Layer

For this architecture to work, the backend executing these CALL procedures must act as a strict gatekeeper.

    Topological Validation: Before committing any transaction, the command layer must run sanity checks (e.g., Euler-Poincaré characteristic formulas) to ensure the resulting graph is mathematically sound.

    Automatic Derivation Updates: If a command modifies an editable Face, the command layer must automatically update, split, or recalculate the downstream semantic Surfaces attached to it. The user should never manually update a Surface.

    Atomic Transactions: If a command fails at step 4 of a 5-step geometric extrusion, the entire graph transaction rolls back so you never end up with phantom vertices.

3. Piping Queries into Commands

The true power of this design is that users can use the read-heavy TopoCypher to dynamically find target geometries, and pipe them directly into your write-heavy command layer in a single script.

Example: Finding all non-manifold flat roofs and automatically lifting them.
Cypher

// 1. QUERY: Find the targets
MATCH (c:Cell)-[:DERIVES]->(s:Surface)
WHERE s.exposure = 'External' AND s.stance = 'Horizontal'

// 2. COMMAND: Pass them to the safe mutation layer
CALL topo.extrudeSurface(s, 0.5) YIELD newCell

// 3. RETURN: Output the results
RETURN newCell.id AS ParapetAdded

Are you planning to implement this command layer using event sourcing (where you store an immutable log of every command to allow for easy undo/redo of the geometry), or will it just be a stateless transactional gateway that mutates the graph directly?

The commands are called actions. Make an extensive markdown file I can download with a plan. 5. Implementation Roadmap
Phase 1: Foundation

    Goal: Set up the database environment and basic primitives.

    Tasks:

        Define the strict Node Labels and Relationship Types in the Graph DB.

        Implement the basic Constructor Actions (createVertex through createFace).

        Implement standard topological checks (e.g., checking if a Wire is closed before allowing a Face to be created).

Phase 2: The Macro & Semantic Engine

    Goal: Enable 3D generation and semantic mapping.

    Tasks:

        Implement complex operations (extrude, boolean).

        Build the trigger system: When a Face is finalized, automatically fire deriveSurface.

        Create the graph rollback mechanisms for failed complex operations.

Phase 3: The Cypher Interface

    Goal: Connect the Action API to the query language.

    Tasks:

        Wrap the Action API endpoints as Custom Procedures (CALL action...) within the graph database (e.g., using Neo4j's Java API or Memgraph's Python/C API).

        Optimize query-to-action piping to handle bulk operations (e.g., mutating 10,000 faces at once).

Phase 4: Event Sourcing & Validation Polish

    Goal: Production readiness and Undo/Redo.

    Tasks:

        Implement the Action Log (storing the exact parameters of every CALL before execution).

        Build the action.undo() command which reads the log and executes the inverse topological operations.
        """

with open("TopoCypher_Action_Layer_Plan.md", "w") as f:
f.write(markdown_content)

## print("TopoCypher_Action_Layer_Plan.md generated successfully.")

Writing a query engine that executes directly on the Boundary Representation (B-rep) kernel is a high-performance approach. By bypassing an intermediate graph database (like Neo4j or Graphology), you eliminate synchronization overhead, reduce memory duplication, and gain the ability to query real-time geometric properties (like bounding boxes, normals, or dynamically calculated areas) during the traversal.However, a B-rep is usually built on a Half-Edge, Winged-Edge, or Radial-Edge data structure. These structures are optimized for localized topological queries (e.g., "what edges bound this face?"), not global graph searches (e.g., "find all non-manifold edges in the model").To make your declarative construct language execute efficiently on brepjs, you must build a Query Planner and a set of Topological Iterators.Here is the architectural blueprint for building a direct-to-kernel query engine in TypeScript/JavaScript.1. The Engine Architecture PipelineYour new pipeline looks like a miniature database engine:Parser: Text (MATCH...) $\rightarrow$ Abstract Syntax Tree (AST).Query Planner (Optimizer): AST $\rightarrow$ Execution Plan. (Decides where to start the search to minimize iterations).Execution Engine (The Traverser): Execution Plan $\rightarrow$ brepjs Iterators $\rightarrow$ Yields raw brepjs objects.2. Step 1: Lightweight Kernel IndexingIf your query is MATCH (f:Face {exposure: 'External'}), you do not want to iterate over 100,000 faces in brepjs. Even though you are querying the kernel directly, the kernel must maintain lightweight HashMaps for entry points.Add these to your brepjs environment:ID Index: Map<string, TopoEntity> (O(1) lookup by ID).Type Index: Map<EntityType, Set<TopoEntity>> (O(1) lookup to get all Cells, all Faces, etc.).3. Step 2: The Topological Iterators (The Core Engine)The hardest part of this architecture is translating a Cypher-like relationship (--[:SHARES]--) into B-rep pointer hopping. You need to write a library of Generator Functions (using JS yield) that abstract the half-edge traversals.Here is what the execution layer looks like in TypeScript:TypeScriptimport { TopoFace, TopoEdge, TopoVertex } from 'brepjs';

class TopologyTraverser {

    // Traverses down the hierarchy: Face -> Edges
    static *getEdgesOfFace(face: TopoFace): IterableIterator<TopoEdge> {
        let startHalfEdge = face.outerLoop.halfEdge;
        let current = startHalfEdge;
        do {
            yield current.edge;
            current = current.next;
        } while (current !== startHalfEdge);
    }

    // Traverses laterally: Face -> Shared Edge -> Adjacent Face
    static *getAdjacentFaces(face: TopoFace): IterableIterator<TopoFace> {
        let startHalfEdge = face.outerLoop.halfEdge;
        let current = startHalfEdge;
        do {
            if (current.twin && current.twin.face) {
                yield current.twin.face;
            }
            current = current.next;
        } while (current !== startHalfEdge);
    }

    // Traverses up: Vertex -> Faces (requires Radial or Winged Edge structure)
    static *getFacesSharingVertex(vertex: TopoVertex): IterableIterator<TopoFace> {
        // Implementation depends heavily on your specific brepjs pointer structure
    }

}
By using JS Generators (yield), your engine is lazy. It only computes the next step in the B-rep if the query engine asks for it, saving massive amounts of memory.4. Step 3: The Query Execution PlanWhen the AST is generated, your Query Planner must convert the declarative construct statement into a chain of these iterators.Example Query:MATCH (f1:Face)-[:ADJACENT_TO]->(f2:Face) WHERE f1.id = 'Roof' AND f2.stance = 'Vertical'Bad Execution (O(N²)): Loop over all faces, loop over all faces again, check if they are adjacent, check properties.Good Execution (The Planner):Index Lookup: Find f1 in the ID Index (O(1)).Traverse: Use TopologyTraverser.getAdjacentFaces(f1) (O(E) where E is edges of f1).Filter: Check if f2.stance === 'Vertical'.The Executor Code:TypeScriptclass ConstructExecutor {
constructor(private kernel: BrepKernel) {}

    executeMatchAdjacentFaces(f1Id: string, filterCondition: (f: TopoFace) => boolean) {
        const results = [];
        // 1. Look up the starting node from the kernel's hash map
        const f1 = this.kernel.getById(f1Id);

        if (!f1 || f1.type !== 'Face') return results;

        // 2. Use the specialized B-rep iterator
        const adjacentFaces = TopologyTraverser.getAdjacentFaces(f1 as TopoFace);

        // 3. Filter on the fly
        for (const f2 of adjacentFaces) {
            if (filterCondition(f2)) {
                results.push({ f1, f2 });
            }
        }
        return results;
    }

} 5. Handling "Semantic" Queries (Surfaces and Parts)In your original spec, Surfaces and Parts are non-editable derivations. If you execute directly on the kernel, you have two choices for how to handle these:Option A: Virtual Graph (Computed on the fly)When a user queries a Surface, the engine dynamically groups coplanar faces at runtime.Pros: No state synchronization needed.Cons: Highly computationally expensive for large models.Option B: Hybrid Graph (Recommended)brepjs handles the raw Editable geometry (Vertices, Edges, Faces, Cells). When your Stately machine finishes an Action, it attaches Semantic Metadata Labels directly onto the brepjs objects.e.g., face.metadata.surfaceId = 'Surf_01'e.g., cell.metadata.partOverlap = 'Intersection'When the query engine encounters a semantic term, it just acts as a filter on the underlying B-rep entities, returning a grouped cluster of faces or cells.SummaryBuilding the engine directly on brepjs means you are essentially building a Graph-to-B-Rep Translation Compiler. You must map graph relationship concepts ([:CONTAINS], [:BOUNDED_BY], [:SHARES]) directly to Half-Edge pointer chasing algorithms. If done correctly, it will be exceptionally fast and capable of deep, mathematically rigorous spatial queries.
