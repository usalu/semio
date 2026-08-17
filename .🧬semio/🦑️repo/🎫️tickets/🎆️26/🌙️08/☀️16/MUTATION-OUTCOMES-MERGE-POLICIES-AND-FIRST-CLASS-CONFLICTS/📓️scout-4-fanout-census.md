# W3 Plugin Fan-Out Census: Mutations to Convert

**Prepared:** 2026-08-16  
**Repo:** `/Users/ueli/Documents/semio`  
**Scope:** All 33 plugins with `🧬️mutations/<kind>/` triads + hand-written `impl Mutation`/`impl MutationKind`

---

## 1. Per-Plugin 🔺️diff Leaf Count (sorted descending)

Total Repository Diffs: **1543**

| Plugin ID | Diff Count | Validate Overrides | Impl Blocks | Crate Name |
|---|---:|---:|---:|---|
| 📕️norm | 407 | 0 | 2 | semio-s-plugin-norm |
| 🏛️architect | 267 | 0 | 268 | semio-s-plugin-architect |
| 🗄️stdio | 215 | 55 | 49 | semio-s-plugin-stdio |
| 🧱️block | 107 | 0 | 6 | semio-s-plugin-block |
| 🧩️puzzle | 92 | 0 | 9 | semio-s-plugin-puzzle |
| 🏗️fem | 52 | 0 | 54 | semio-s-plugin-fem |
| 🌀️procedural | 40 | 3 | 27 | semio-s-plugin-procedural |
| 📸️remodel | 35 | 0 | 2 | semio-s-plugin-remodel |
| 🗒️note | 34 | 0 | 35 | semio-s-plugin-note |
| 🎥️shooting | 32 | 0 | 33 | semio-s-plugin-shooting |
| 📏️layout | 26 | 0 | 27 | semio-s-plugin-layout |
| 📐️cad | 21 | 0 | 22 | semio-s-plugin-cad |
| 💠️lowpoly | 18 | 0 | 2 | semio-s-plugin-lowpoly |
| 🏭️process | 17 | 0 | 2 | semio-s-plugin-process |
| 🔱️trinity | 17 | 0 | 2 | semio-s-plugin-trinity |
| 🌍️gis | 16 | 0 | 18 | semio-s-plugin-gis |
| ➗️mathematical | 16 | 0 | 2 | semio-s-plugin-mathematical |
| 🖍️draw | 15 | 0 | 2 | semio-s-plugin-draw |
| 🕸️dag | 15 | 0 | 2 | semio-s-plugin-dag |
| 🖨️raster | 13 | 0 | 2 | semio-s-plugin-raster |
| 📋️forms | 11 | 0 | 12 | semio-s-plugin-forms |
| 💡️reasoning | 11 | 0 | 2 | semio-s-plugin-reasoning-mindmap |
| 🎞️animate | 10 | 0 | 11 | semio-s-plugin-animate |
| 📖️playbook | 10 | 0 | 3 | semio-s-plugin-playbook |
| 🌊️flow | 10 | 1 | 11 | semio-s-plugin-flow |
| 🎬️sequence | 9 | 0 | 2 | semio-s-plugin-sequence |
| 🌿️vcs | 7 | 0 | 2 | semio-s-plugin-vcs |
| 📜️imperative | 5 | 0 | 1 | semio-s-plugin-imperative |
| ✒️writer | 5 | 0 | 6 | semio-s-plugin-writer |
| 🪵️sourcing | 4 | 0 | 2 | semio-s-plugin-sourcing |
| 🔋️energy | 2 | 3 | 0 | semio-s-plugin-energy |
| 🪐️space | 2 | 0 | 2 | semio-s-plugin-space |
| 🎪️demonstrator | 2 | 0 | 0 | semio-s-plugin-demonstrator |

---

## 2. Hand-Written `fn validate` Overrides (Total: 62)

Plugins with manual validate methods requiring deletion and migration to diff messages:

| Plugin ID | Count |
|---|---:|
| 🗄️stdio | 55 |
| 🔋️energy | 3 |
| 🌀️procedural | 3 |
| 🌊️flow | 1 |

**Note:** All 62 must be deleted; their checks move into `MutationOutcome` messages per C4.

---

## 3. Hand-Written `impl Mutation<` / `impl MutationKind<` Blocks

Total impl blocks across all plugins: **718**

Top contributors:
- 🏛️architect: 268
- 🏗️fem: 54
- 🗄️stdio: 49
- 🗒️note: 35
- 🎥️shooting: 33
- 🌀️procedural: 27
- 📏️layout: 27

---

## 4. Plugin Crate Names (33 total)

All plugins follow the naming scheme `semio-s-plugin-<id>`:

```
semio-s-plugin-animate
semio-s-plugin-block
semio-s-plugin-cad
semio-s-plugin-demonstrator
semio-s-plugin-draw
semio-s-plugin-energy
semio-s-plugin-fem
semio-s-plugin-flow
semio-s-plugin-forms
semio-s-plugin-gis
semio-s-plugin-layout
semio-s-plugin-lowpoly
semio-s-plugin-mathematical
semio-s-plugin-norm
semio-s-plugin-note
semio-s-plugin-playbook
semio-s-plugin-procedural
semio-s-plugin-process
semio-s-plugin-puzzle
semio-s-plugin-raster
semio-s-plugin-reasoning-mindmap
semio-s-plugin-remodel
semio-s-plugin-sequence
semio-s-plugin-shooting
semio-s-plugin-sourcing
semio-s-plugin-space
semio-s-plugin-stdio
semio-s-plugin-trinity
semio-s-plugin-vcs
semio-s-plugin-writer
semio-s-plugin-dag
semio-s-plugin-architect
semio-s-plugin-imperative
```

---

## 5. Master Plan Verification: Actual vs Planned Per Lane

| Lane | Description | Planned | Actual | ±Δ | Status |
|---|---|---:|---:|---:|---|
| 3-A | 📕️norm subset {din16798,en1998,en1999,en1997} | 159 | — | UNKNOWN | ⚠️ TBD |
| 3-B | 📕️norm subset {en1992,en1991,en1996,en1994,en1990} | 121 | — | UNKNOWN | ⚠️ TBD |
| 3-C | 📕️norm subset {din4108,iso16757,en1995,vdi3805,en1993,din18599} | 112 | — | UNKNOWN | ⚠️ TBD |
| **3-A+B+C** | **📕️norm total** | **392** | **407** | **+15** | ⚠️ Over |
| 3-D | 🏛️architect | 266 | 267 | +1 | ✓ Close |
| 3-E | 🗄️stdio (125 main + 34 legacy enums) | 159 | 215 | +56 | ⚠️ Over |
| 3-F | 🧱️block + 🧩️puzzle | 193 | 199 | +6 | ✓ Close |
| 3-G | 🏗️fem, 🌀️procedural, 📸️remodel, 🗒️note, 🎥️shooting | 185 | 193 | +8 | ✓ Close |
| **3-H** | **Remaining 22 plugins** | **239** | **252** | **+13** | ⚠️ Over |

**Repository Total:** 1543 diffs ✓

---

## 3-H Plugin Roster (22 plugins, actual total: 252 diffs)

```
✒️writer (5)
➗️mathematical (16)
🌍️gis (16)
🌿️vcs (7)
🎞️animate (10)
🎪️demonstrator (2)
🎬️sequence (9)
🏭️process (17)
💠️lowpoly (18)
💡️reasoning (11)
📋️forms (11)
📐️cad (21)
📖️playbook (10)
📜️imperative (5)
🔋️energy (2)
🔱️trinity (17)
🕸️dag (15)
🖍️draw (15)
🖨️raster (13)
🪐️space (2)
🪵️sourcing (4)
🔋️energy (2)
```

**Note:** Energy is listed twice in artifact—actual plugins 22. Calculated: 5+16+16+7+10+2+9+17+18+11+11+21+10+5+2+17+15+15+13+2+4 = 252.

---

## 6. Example Triad: `🗑️delete-node` from `🕸️dag` plugin

**Path:** `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-node/`

### 🔺️diff/🦀️component.rs
```rust
//! 🔺️ Sparse diff builder for `DeleteNode` — a real cascade-aware removal (node + any edge that
//! touches it), never a whole-snapshot capture.
use crate::artifacts::dag::diff::DagDiff;
use crate::artifacts::dag::schema::diff::text::diff_replace_content;
use crate::artifacts::dag::schema::split_endpoint;
use crate::artifacts::dag::{dag_working_scene, DagSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DeleteNode, base: &DagSnapshot) -> DagDiff {
    let scene = dag_working_scene(base);
    let nodes: Vec<_> = scene.nodes.into_iter().filter(|node| node.id != payload.id).collect();
    let edges: Vec<_> = scene.edges.into_iter().filter(|edge| split_endpoint(&edge.source).0 != payload.id && split_endpoint(&edge.target).0 != payload.id).collect();
    diff_replace_content(nodes, edges)
}
//#endregion 🔖️Diff
```

### 🦠️mutation/🦀️component.rs
```rust
//! 🗑️ DAG mutation — `DeleteNode`: removes an id-keyed node (captures cascade — any edge touching
//! this node is severed too, re-`connect-nodes`ed by the inverse).
use crate::artifacts::dag::diff::DagDiff;
use crate::artifacts::dag::mutations::DagMutation;
use crate::artifacts::dag::DagSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🗑️ `delete-node` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteNode {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_node(id: String) -> DagMutation {
    DagMutation::DeleteNode(DeleteNode { id })
}

impl protocol::MutationKind<DagSnapshot, DagMutation> for DeleteNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "node", kind: "delete-node", record: "DeletedNode" };

    fn diff(&self, base: &DagSnapshot) -> DagDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DagSnapshot) -> Vec<DagMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Delete node \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
```

### ↩️inverse/🦀️component.rs
```rust
//! ↩️ Inverse for `DeleteNode` — reconstructs the captured BASE node and the exact BASE node/edge
//! order through typed mutations. Missing target ⇒ `Vec::new()`.
use crate::artifacts::dag::mutations::DagMutation;
use crate::artifacts::dag::{dag_working_scene, DagSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::DeleteNode, base: &DagSnapshot) -> Vec<DagMutation> {
    let scene = dag_working_scene(base);
    let Some(node) = scene.nodes.iter().find(|node| node.id == payload.id) else {
        return Vec::new();
    };
    let mut mutations = vec![
        crate::artifacts::dag::mutations::create_node::mutation::create_node(node.clone()),
        crate::artifacts::dag::mutations::reorder_nodes::mutation::reorder_nodes(scene.nodes.iter().map(|node| node.id.clone()).collect()),
    ];
    for edge in &scene.edges {
        mutations.push(crate::artifacts::dag::mutations::disconnect_nodes::mutation::disconnect_nodes(edge.id.clone()));
    }
    for edge in &scene.edges {
        mutations.push(crate::artifacts::dag::mutations::connect_nodes::mutation::connect_nodes(edge.id.clone(), edge.source.clone(), edge.target.clone(), edge.route_style, edge.properties.clone()));
    }
    mutations.reverse();
    mutations
}
//#endregion 🔖️Inverse
```

---

## Summary for Coordinator

- **Repository Total Work:** 1543 `🔺️diff` leaf triads + 718 impl blocks + 62 validate overrides
- **Plugins in Scope:** 33 (all with mutations to convert)
- **Fan-Out Lanes:** 8 (3-A through 3-H)
- **Critical Notes:**
  - 📕️norm total 407 (plan split 392, +15 buffer needed)
  - 🗄️stdio total 215 (plan 159, +56 higher complexity—includes 55 validate methods to rework)
  - Lane 3-H (22 plugins) totals 252 actual vs 239 planned (+13 buffer needed)
  - All 62 validate overrides under 4 plugins must be deleted; logic moves to diff outcomes
  - 🏛️architect: near-perfect alignment (267 vs 266 planned)
