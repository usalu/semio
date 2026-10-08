# Drawing Identity Current Design And Caller Frontier

Color/dash receiving closure is the current execution scope. Identity remains unfinished; no identity source edit or runtime identity admission claim is made here. This design records the concrete frontier before a separate controlled identity change.

Current schema body at `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs`:

```rust
pub(crate) fn drawing_id_hex(material: &[u8]) -> String {
    let mut hasher = DefaultHasher::new();
    material.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

/// 🪪️ Derives a content-addressed document identity without a process-wide counter.
pub fn create_drawing_id(prefix: &str, material: &[u8]) -> String {
    format!("{prefix}-{}", drawing_id_hex(material))
}
```

Direct distinctive caller rows:

```text
✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🦀️.rs:241:    crate::standards::v1::subsets::any::schema::create_drawing_id("shape", &identity)
✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/📐️footprint/🧪️tests/🔬️unit/🦀️.rs:103:        assert_eq!(expected, crate::schema::create_drawing_id("layer", material.as_bytes()));
✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➕️add-layer/🦀️.rs:53:        let id = crate::standards::v1::subsets::any::schema::create_drawing_id("layer", &candidate);
✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️edit-selection/🦀️.rs:65:            layer_base_mut(&mut group).id = crate::standards::v1::subsets::any::schema::create_drawing_id("group", material.as_bytes()).into();
✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs:21:        let mut job=document::SvgImportJob::new(source,&crate::standards::v1::subsets::any::schema::create_drawing_id("svg",source.as_bytes())).map_err(error)?;
✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:181:    let renamed = crate::schema::find_drawing_layer(&projection, &crate::standards::v1::subsets::any::schema::create_drawing_id("path", b"Path")).expect("retained Drawing target");
✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:11:pub(crate) fn drawing_id_hex(material: &[u8]) -> String {
✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:18:pub fn create_drawing_id(prefix: &str, material: &[u8]) -> String {
✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:19:    format!("{prefix}-{}", drawing_id_hex(material))
✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:26:            id: create_drawing_id("path", name.as_bytes()).into(),
✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:240:    DrawingLayerBase { id: create_drawing_id("layer", name.as_bytes()).into(), name: name.into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: default_drawing_transform(), attributes: DrawingAttributes::default() }
✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:249:            id: create_drawing_id("group", name.as_bytes()).into(),
✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:265:            id: create_drawing_id("boolean", name.as_bytes()).into(),
✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:282:            id: create_drawing_id("trace", name.as_bytes()).into(),
✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:299:            id: create_drawing_id("shape", name.as_bytes()).into(),
✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:320:            id: create_drawing_id("text", name.as_bytes()).into(),
✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:339:            id: create_drawing_id("image", name.as_bytes()).into(),
✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:673:        base.id = create_drawing_id("layer", format!("{old}{suffix}").as_bytes()).into();
✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧬️schema/🧬️mutations/📋️duplicate-layer/🧪️tests/🚫️rejects/🦀️.rs:9://! duplicate's id is content-addressed through `create_drawing_id`/`DefaultHasher`
✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧬️schema/🧬️mutations/📋️duplicate-layer/🧪️tests/🚫️rejects/🦀️.rs:10://! (`🧬️schema/🦀️component.rs`, `drawing_id_hex`). A hand-authored `➡️after` would have to embed that

```

The direct free-function frontier is small, but ID creation also occurs inside layer/default constructors and recursive clone-with-new-IDs. Those constructors feed many mutation/editor/fixture callers. Simply moving hash/hex helpers and having these constructors call them would preserve hidden IO in the semantic owner. The necessary canonical semantic contract is a DrawingIdentity containing intrinsic identity octets plus an admitted construction fact bound to its domain kind and input facts; constructors take admitted identity values explicitly. Controlled binary identity IO owns the unambiguous preimage (kind tag, counted UTF-8 naming facts, counted source facts where applicable), first-party commitment algorithm and progress/cancellation. Controlled text identity IO owns hexadecimal spelling and prefixed publication. The current DefaultHasher implementation is not a stable interoperable commitment specification; adopting an explicit algorithm requires neutral preimage/digest witnesses and independent third-party computation. Collision/stale-target/refusal laws must not be silently lost.

Before implementing, resolve the constructor/clone frontier using actual parameter types and the current General retained allocation APIs. No legacy create_drawing_id alias, semantic-root IO forwarding or metadata-dependent default hashing should survive that change. Existing authored IDs are preserved until authentic IO identity witnesses justify canonical replacements. No broad camera/string substitutions are authorized by this design.
