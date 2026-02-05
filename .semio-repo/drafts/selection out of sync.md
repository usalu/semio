Synchronize Kit App Diagram Node Visibility with Table Artifact Filtering
Problem Statement
The Kit app's diagram window fails to display nodes that correspond to visible artifacts in the table view. This creates a disconnect between the two visualization modes, reducing their utility as complementary views of the kit structure.

Current Behavior:

Table displays artifacts based on active filters (types, designs, qualities, ports, tags, concepts, files, folders, authors)
Diagram shows a static set of nodes that don't update when table filtering changes
Unfolded (collapsed) artifacts don't appear in the diagram until they're explicitly expanded in the table
Some Ancestor nodes remain visible in the diagram even when all their children are filtered out
Expected Behavior:

Diagram nodes reflect the exact set of artifacts currently visible in the table
Expanding/collapsing artifact rows updates diagram visibility in real-time
Toggling filter categories (files, folders, types, designs, qualities, ports, tags, concepts, authors) immediately updates which nodes appear in the diagram
Ancestor nodes show/hide based on whether they have visible descendants
Diagram maintains simulation state and selection during filter transitions
Scope

Affected Components:

Kit.tsx - KitAppState, filter logic, diagram renderer
elements.tsx - Diagram component, node/edge rendering
Kit app diagram relationships (part-of, reference edges)
Artifact Kinds to Support:

Types, Designs, Qualities, Ports, Tags, Concepts, Files, Folders, Authors
Acceptance Criteria
✅ Diagram node visibility matches table visibility for all artifact kinds
✅ Expanding/collapsing rows in the table updates diagram within one render cycle
✅ Toggling filter toggles in the toolbar updates diagram nodes synchronously
✅ Ancestor nodes (parent types, parent designs) only appear if they have visible descendants
✅ Diagram selection state persists across filter changes
✅ Diagram hover state clears when hovered node becomes hidden
✅ D3 simulation continues running smoothly during visibility transitions
✅ Edge rendering excludes connections between hidden nodes
✅ All existing diagram relationships (part-of, reference) remain intact
✅ No console errors or performance degradation with large kits


Technical Approach
Calculate visible artifact set from current table state (filters + expanded rows)
Pass visible set to diagram as derived/memoized state
Filter React Flow nodes and edges to only include visible artifacts
Handle ancestor visibility by checking if any descendants are visible
Ensure diagram relayout on visibility changes via simulation reheat
Update hover state predicates to check visibility before rendering highlights

--------------