# Lowpoly End-to-End: Examples, Rendering, and ArtifactChild Resolution

## Question 1: Example Enumeration and Defaults

**Example IDs in repo:**
- Only ONE example exists: `demo` (ID: `demo`)
  - Location: `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/`
  - Source: `🖼️assets/🗣️.dsl.semio` — line 1

**Default on boot:**
- Viewer (LowpolyViewer): calls `initial_snapshot()` → `default_snapshot()` (line 53-54 in `👁️viewer/🦀️.rs`)
- Editor: same — `default_snapshot()` is called for both app manifests
- Location of default choice: `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:202-213`

**Example payload analysis:**
- The demo example file at `🖼️assets/🗣️.dsl.semio` contains:
  ```
  semio lowpoly.lowpoly.dsl v1
  schema=6c6f77706f6c792e646f63756d656e74
  objects=[[6f626a2d31,556e697420426f78,[0,0,0,0,0,0,1,1,1],false,[],[]]]
  ```
  - Decoded: `obj-1`, `Unit Box`, transform [0,0,0,0,0,0,1,1,1], smooth_shading=false, **empty mesh handle `[]`**, **empty paint layers `[]`**
  - Critical: NO mesh geometry in payload, NO paint layers (line 3)

**Default document source:**
- `default_owned_document()` line 202-208 in `🧬️schema/🦀️.rs` builds the REAL default:
  ```rust
  let mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).expect("box prim");
  let mesh_json = mesh.to_json().expect("mesh json");
  let snapshot = crate::artifacts::lowpoly::snapshot_from_mesh_json(&mesh_json, "obj-1", "Unit Box");
  let mesh_workspace = std::collections::HashMap::from([("obj-1".to_string(), mesh_json)]);
  ```
  - Default IS a real unit box mesh (created at runtime, not persisted)
  - Stored in **session-local `mesh_workspace` cache**, NOT in the persisted snapshot

---

## Question 2: Example Switching / setActiveExample Command

**Result:** NO SUCH COMMAND EXISTS.

**Mutation catalog** (`🧬️schema/🧬️mutations/🦀️.rs:71-89`):
- Lists all 17 mutations: `create-object`, `delete-object`, `rename-object`, `move-object`, `rotate-object`, `scale-object`, `create-mesh`, `delete-mesh`, `insert-paint-layer`, `remove-paint-layer`, `rename-paint-layer`, `change-paint-layer-visible`, `change-paint-layer-opacity`, `change-paint-layer-blend-mode`, `edit-paint-layer`
- **NO `set-active-example` or `load-example` variant**

**Grep result:** Zero matches for `setActiveExample`, `load.*example`, or `select.*example` in the lowpoly artifact tree.

**Implication:** Examples cannot be switched at runtime. The demo example exists but is inaccessible to the app—it serves no purpose in the current implementation.

---

## Question 3: Windows/Surfaces and Empty-State Markers

**Declared window surfaces:**

| Surface ID | SurfaceKind | Empty-State Marker | Condition |
|---|---|---|---|
| `framework.window.mesh` (viewer, model, UV) | `World3d` | `semio-world-3d-empty` | `!scene` (null/undefined) |
| `lowpoly.play.main` (editor model) | `World3d` | `semio-world-3d-empty` | `!scene` |

**React surface host:** `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx`

**Empty-state check (line 5055):**
```typescript
if (!scene) return <div className="semio-world-3d-empty">{emptySceneLabel}</div>;
```

**What must be non-empty for rendering:**
- `node.world3d` (the scene object passed to the host) must be non-null
- Scene requires `instancesJson` to be populated (parsed at line 3892)
- Editor additionally requires `meshesJson` with real tessellation data (lines 111-126 in model render)

**For lowpoly specifically:**
- Viewer: `node.world3d` contains instances with hardcoded `meshId: "box"` (placeholder)
- Editor: `node.world3d` contains instances with `meshId: object.id` (real mesh id reference)

---

## Question 4: VERIFY / REFUTE prior session's ArtifactChild claim

**PRIOR CLAIM:**
> "the viewer's placeholder-mesh fallback" and "composed-child mesh resolution is structurally unreachable — LowpolySnapshot never implements ArtifactRefs and nothing calls register_child, so doc.children is provably always empty"

**VERDICT: CONFIRMED — the claim is CORRECT.**

**Evidence:**

1. **LowpolySnapshot structure** (`🧬️schema/📸️snapshot/🦀️.rs:19-30`):
   - Only two fields: `schema: String` and `objects: Vec<LowpolyObject>`
   - `objects` is id-keyed list of objects, NOT an ArtifactRefs collection
   - `LowpolySnapshot::default()` creates `Vec::new()` (line 28)

2. **LowpolyObject.mesh field** (`🧬️schema/📸️snapshot/🦀️.rs:121`):
   - Each object has `mesh: Option<store::ArtifactChild<SemioMeshSnapshot>>`
   - This is a **handle ONLY** — the child_id string + target URI, never the content
   - Decoded at line 133 via `dec_child_opt()` but never resolved

3. **Mesh content location** (`🧬️schema/📸️snapshot/🦀️.rs:116-119`):
   ```
   /// One object: `[id,name,transform,smooth-shading,mesh-handle,paint-layers]`. 
   /// The live half-edge mesh JSON content is DELIBERATELY absent — 
   /// it is not a field of `LowpolyObject` at all (moved to `✏️editor/🖌️session::LowpolyScratch`'s 
   /// session-local `mesh_workspace` cache
   ```
   - No `register_child` call anywhere (grep: zero matches)
   - No `ArtifactRefs` impl on LowpolySnapshot
   - No async resolution path in any render signature

4. **Viewer fallback** (`👁️viewer/🎭️modes/👁️view/🪟️windows/🌐️model/🦀️.rs:6-9, 59`):
   - Lines 6-9 explicitly state: "Object geometry renders the same fallback-box placeholder the editor's own `world_meshes_json` falls back to while composed-child mesh resolution is **unimplemented** (ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave-3 gap, pre-existing, not introduced here)"
   - Line 25: `const LOWPOLY_VIEW_FALLBACK_MESH_KIND: &str = "box";`
   - Line 59: `"meshId": LOWPOLY_VIEW_FALLBACK_MESH_KIND` — hardcoded "box"

5. **Export serializer attestation** (`🚪️io/📤️export/🧵️serializers/🗿️artifacts/🗿️obj/🔖️3.0/✳️any/🦀️.rs:7-19`):
   ```
   Root cause: `LowpolyObject.mesh` is only a content-addressed `store::ArtifactChild<SemioMeshSnapshot>` 
   HANDLE (see that field's doc comment) -- the live half-edge mesh geometry is not a field of 
   `LowpolySnapshot` at all, so no synchronous function of `&LowpolySnapshot` alone can ever 
   produce real OBJ vertices/faces.
   
   Fix: ... Geometry-empty is honest, not silent: real mesh export needs an out-of-scope 
   architecture change (resolving the mesh child artifact through a store/session handle 
   no `serialize`/`serialize_bytes` signature here receives).
   ```

**Conclusion:** The ArtifactChild mesh handle is declared but structurally **dead code**. It cannot be resolved because:
- No async resolution infrastructure exists in any render path
- `serialize`/`render` signatures lack access to store/session handles
- The mesh lives permanently in a session-local cache, never persisted/synchronized
- The framework has no mechanism to resolve an `ArtifactChild<SemioMeshSnapshot>` at render time

---

## Question 5: Mesh Source for Rendering

**Where does the mesh come from?**

**Viewer rendering path:**
- Input: `LowpolySnapshot` with empty mesh handles (or handles that are never read)
- Render function: `👁️viewer/🎭️modes/👁️view/🪟️windows/🌐️model/🦀️.rs:76-85`
- Line 77: `let meshes_json = serde_json::to_string(&[serde_json::json!({ "id": LOWPOLY_VIEW_FALLBACK_MESH_KIND, "data": Into::<serde_json::Value>::into(dsl::ToValue::to_value(&mesh_from_kind(LOWPOLY_VIEW_FALLBACK_MESH_KIND))) })])`
- **Result:** Calls `mesh_from_kind("box")` to generate a placeholder at render time
- **No mesh data from the snapshot is used**

**Editor rendering path:**
- Compute session: `✏️editor/⚙️engine/🦀️.rs:346-370`
- `tessellate_all_json()` reads from `self.meshes` (loaded from session-local `mesh_workspace` cache)
- `reload_meshes()` (line 150-165): loads each object's `HalfedgeMesh` from `mesh_workspace.get(&object.id)`
- **Never calls** `object.mesh` handle's resolution — just checks it's in sync via `mesh_child_handle()` (line 157)
- Window render: `✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️model/🦀️.rs:111-126`
- `world_meshes_json()` (line 111): reads from `doc.tessellate_all_json()` output
- **Mesh content comes 100% from the session-local cache, never from ArtifactChild resolution**

**Scene-building function:** None exists that converts LowpolySnapshot → renderable geometry.
- The app builds `LowpolyDocument` from snapshot + session cache (line 92-98 in engine)
- Tessellation is live kernel output, not a derivation of the handle

**Conclusion:**
- Viewer: **placeholder-box fallback** (hardcoded, generated at render time)
- Editor: **session-local mesh_workspace cache** (loaded at startup, persisted via mutations)
- **Both: zero reliance on ArtifactChild.target resolution** — it is never attempted, never needed, provably unreachable

---

## Summary: Would Fresh Boot Show Real Geometry?

### VIEWER:
- **Will render?** YES — a unit box placeholder
- **Is it real?** NO — hardcoded fallback mesh ("box")
- **Blocking link:** The mesh lives in ArtifactChild handle, but the viewer never reads handles. It always renders "box" regardless of what's in the snapshot.

### EDITOR:
- **Will render?** YES — a unit box (the default)
- **Is it real?** YES, but only because the default document was built into memory at app init
- **Blocking link:** If you mutate and close without persisting mesh to artifact, the next load will fail with `StaleMeshWorkspace` (line 29, 41 in engine errors) because the handle won't match the cache

### EXAMPLES:
- **Switchable?** NO — there is no `set-active-example` mutation
- **Work if they could be loaded?** NO — the demo example has an empty mesh handle and empty paint layers; it would fail to render in the editor (StaleMeshWorkspace) and show as empty in the viewer (it's already a placeholder)

### ROOT CAUSE FOR MISSING GEOMETRY:
**The mesh lives in the session-local `LowpolyScratch::mesh_workspace` HashMap, keyed by object id. The persisted `LowpolySnapshot` stores only a content-addressed `ArtifactChild` handle that is never resolved. No rendering path reads real mesh data from the snapshot itself — both viewer and editor read from the transient session cache or fallback placeholders.**
