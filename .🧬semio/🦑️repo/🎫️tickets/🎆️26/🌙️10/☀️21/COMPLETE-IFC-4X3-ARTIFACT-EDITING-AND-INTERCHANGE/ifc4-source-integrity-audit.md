# IFC4 Source Integrity Audit

Actual file read 2026-10-21; source untouched.

File: C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📐️geometry/🦀️.rs

Confirmed actual disk corruption, not tool escaping. At audit snapshot: 57,420 JS UTF-16 characters, 995 lines. Line 172 ends `matches!(value, b'_' | b''a, 'b> {`. Line 173 begins Projector fields without its struct declaration. Lines 173–580 are an injected copy of the old Projector body/helper tail, ending the first `mod tests;`. Line 581 resumes the interrupted product_hint predicate (`)) { return false; }`), lines 582–584 finish product_hint, and line 586 starts a second intact `struct Projector`. A second `mod tests;` occurs at 994. This is consistent with a JavaScript string replacement treating a literal dollar-apostrophe sequence as the special replacement suffix token, but that mechanism is inferred, not observed. Use a function replacement callback or literal slicing when restoring; preserve concurrent changes.

The intended source is the header through the GUID predicate followed by its proper closure and then the single current Projector block. Do not blindly delete the final tail; the first inserted copy is the duplicate. Native compilation cannot succeed while this source is present. Main was notified immediately.

## Matrix Ownership

Geometry imports `crate::standards::v4::subsets::any::io::spatial::Mat4` from IFC4 any/🚪️io/🏛️spatial/🦀️.rs. That own first-party type is a tuple row-major [[f64;4];4], with identity, mul (parent * child) and transform_point (translation in column 3). Geometry determinant and matrix construction use the same convention. An independent framework matrix implementation exists at C:/git/semio/🧰️framework/🔨️modules/🧊️3d/📐️brep/📸️representation/➡️vector/🔢️matrix/🦀️.rs; it uses typed rows and its kernel primarily accepts restricted Trsf. Do not casually swap IFC mapped transforms to Trsf: nonuniform mapping must remain representable. Three Matrix4 is an existing development-only independent oracle; supply row-major elements via .set() rather than .fromArray() (column-major).

## Framework Selection Integration

C:/git/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/🔍️lane-views/🦀️.rs defines InteractionView (line 62). `selection(domain)` returns protocol DomainSelection without unwrap (line 86); `hover(domain,channel)` returns matching transient channel; `active_granularity` and `active_mode` expose admitted selection options. `leftover_selected_ids()` is an aggregate fallback helper, not the preferred domain-specific inspector identity contract. IFC should declare a product domain and use its selection ids, retaining product `ifc-ID` interaction ids already emitted by geometry.

C:/git/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🌍️world3d/🦀️.rs exports world3d_selection_json at 473 and world3d_selection_json_with_granularity at 476: publishes method, mode replace, ids, hoveredId and optional granularity. Geometry currently passes empty ids. Pass admitted InteractionView selection into artifact scene publication; continue through framework interaction reducers rather than introducing persisted artifact selection mutations.

C:/git/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🖼️render/📥️staging/🧭️interaction/🦀️.rs resolves domain topology and revalidates selections on document changes (around 142–145). C:/git/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/🧭️topology/🦀️.rs contains interaction topology behavior. Use this existing document-change selection invalidation for deleted IFC product ids.
