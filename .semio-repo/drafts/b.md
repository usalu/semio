Fix D3 Force Diagram Node-Edge Alignment

Problem: In the D3 Force diagram, node avatars appear smaller than their edge connection circles, creating a visual gap. Edges connect to an invisible larger circle while the rendered node is smaller.

What needs to happen:

Node avatar radius must match the edge endpoint calculation radius
Verify coordinate system consistency (D3 simulation positions vs rendered elements)
Ensure node dimensions are uniform across simulation, rendering, and interaction handling
Success criteria:

Nodes and edges align cleanly—no gaps or misalignment


Constraints:

No questions or approvals between steps
Don't stop until alignment works everywhere
Open ticket with plan.md before starting
Close ticket with summary and changes

Analyze:

Node radius in D3 simulation vs visual rendering
Edge connection point calculations
Avatar element dimensions (padding, border, content)
Coordinate transformation pipeline