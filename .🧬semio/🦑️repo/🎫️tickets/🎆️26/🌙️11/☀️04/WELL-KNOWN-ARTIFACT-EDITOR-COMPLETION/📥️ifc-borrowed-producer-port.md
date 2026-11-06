# IFC4 Borrowed Producer Port

IFC4 original Pack construction, index allocation, publication and retained receiving fixtures now lend their producer and cursor callbacks by mutable reference. Their captures borrow the authored source, parent retained index, wire and fixture observers; no owned source capture is consumed before admission. Source/header installation remains inside the admitted callback.

- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/write-pack.rs: 3 callbacks
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/pack-index.rs: 1 callbacks
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/owned.rs: 2 callbacks
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/owned-tests.rs: 3 callbacks

Rust AST parsing passes for the four files. This is not semantic compilation or native proof. The shared callback API is being updated concurrently, so a fresh compiler capture is required. IFC4x3 and the separate legacy Store callers remain outside this bounded port.

## Caller-Owned Output Revision
The original Pack receiving and writing public methods now accept caller output slots and return work outcomes for unit success. The flat projection index retains each vector in Option<Vec<T>> so child publication writes directly into the parent field. There is no local vector output that would drop on post refusal. Index and writer header construction use original encoder effect quotes, install the header before post, and propagate actual work even on post failure. Native receiving fixtures now retain their output slots and explicitly assert cancellation/invalid forest paths publish nothing. All four source files parse as Rust AST. Native proof remains pending.

The IFC4 model manifest now declares its direct first-party IO control dependency. Cold native SQL decode/encode and independent rich host-wrapper reconciliation remain open and were not disguised as migrated.

Source-reader estimate for IFC4 model: 32 files,259496 raw bytes,256647 active bytes,3813 active lines,zero generated references; the unchanged limits are32files/262144bytes/4000lines. This leaves2648rawbytes and no file-count headroom. Exact Cargo cfg/admission remains authoritative.
