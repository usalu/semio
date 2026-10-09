# Exact Artifact Identity And Custom Facet Input Manifest

Read-only independent Rust AST census: 9019 semantic artifact paths, 1499 parsed candidates, 35 exact items. No build or whole-repo closure.

## identity: ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🆔digest/🦀️.rs:15
- AST item function_item; attached and ancestor cfg: none.

```rust
pub fn compute_content_digest(snapshot: &SHomeSnapshot) -> String {
    let mut hasher = DefaultHasher::new();
    snapshot.schema.hash(&mut hasher);
    snapshot.catalog_generation.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}
```

## identity: ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:18
- AST item function_item; attached and ancestor cfg: none.

```rust
pub fn create_drawing_id(prefix: &str, material: &[u8]) -> String {
    format!("{prefix}-{}", drawing_id_hex(material))
}
```

## identity: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🔨️geometry-core/🦀️.rs:135
- AST item function_item; attached and ancestor cfg: none.

```rust
pub(crate) fn fingerprint(points: &[V3], triangles: &[[usize; 3]]) -> String {
    let mut h = 1469598103934665603u64;
    for p in points {
        for x in p {
            h ^= x.to_bits();
            h = h.wrapping_mul(1099511628211);
        }
    }
    for f in triangles {
        for x in f {
            h ^= *x as u64;
            h = h.wrapping_mul(1099511628211);
        }
    }
    format!("{h:016x}")
}
```

## identity: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🔨️geometry-core/🦀️.rs:151
- AST item function_item; attached and ancestor cfg: none.

```rust
pub(crate) fn byte_fingerprint(bytes: &[u8]) -> String {
    let mut hash = 1469598103934665603u64;
    for byte in bytes {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(1099511628211);
    }
    format!("{hash:016x}")
}
```

## identity: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs:26
- AST item function_item; attached and ancestor cfg: none.

```rust
pub fn png_revision(snapshot: &PngSnapshot) -> String {
    let mut hash=PngRevisionHash(0xcbf29ce484222325);let image=&snapshot.image;
    hash.text(&snapshot.schema);for n in [u64::from(image.width),u64::from(image.height),u64::from(image.bit_depth),u64::from(image.color_type.to_u8()),u64::from(image.interlace)] { hash.number(n); }
    hash.number(image.samples.len() as u64);for sample in &image.samples { hash.number(u64::from(*sample)); }
    hash_image_metadata(&mut hash,image);
    hash.number(image.text_chunks.len() as u64);for text in &image.text_chunks {hash.text(&text.keyword);hash.text(&text.value);hash.number(u64::from(text.compressed));hash.number(match text.kind {crate::schema::snapshot::PngTextKind::Text=>0,crate::schema::snapshot::PngTextKind::ZText=>1,crate::schema::snapshot::PngTextKind::IText=>2});hash.text(&text.language_tag);hash.text(&text.translated_keyword);}
    hash.number(image.ancillary_chunks.len() as u64);for chunk in &image.ancillary_chunks {hash.bytes(&chunk.kind);hash.number(chunk.data.len() as u64);hash.bytes(&chunk.data);hash.number(u64::from(chunk.after_raster));}
    format!("{:016x}",hash.0)
}
```

## identity: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/⚙️operations/🪪️validation/🦀️.rs:10
- AST item function_item; attached and ancestor cfg: none.

```rust
pub fn revision(&self)->Option<String> { if self.stage>=4 { self.hash.as_ref().map(|hash|format!("{:016x}",hash.0)) } else {None} }
```

## identity: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/💡️inferences/📏extent/🦀️.rs:25
- AST item function_item; attached and ancestor cfg: none.

```rust
pub fn compute_binary_extent(snapshot: &BinarySnapshot) -> BinaryExtent {
    let mut hasher = DefaultHasher::new();
    snapshot.bytes.hash(&mut hasher);
    BinaryExtent { byte_length: snapshot.bytes.len() as u64, is_empty: snapshot.bytes.is_empty(), content_digest: format!("{:016x}", hasher.finish()) }
}
```

## identity: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/💡️inferences/🪟window/🦀️.rs:41
- AST item function_item; attached and ancestor cfg: none.

```rust
pub fn compute_deflate_window(snapshot: &DeflateSnapshot) -> DeflateWindow {
    let window_size = if snapshot.window_bits <= 7 { 1u32 << (snapshot.window_bits as u32 + 8) } else { 0 };
    let mut hasher = DefaultHasher::new();
    snapshot.payload.hash(&mut hasher);
    DeflateWindow {
        window_size,
        compression_level_hint: format!("{:?}", snapshot.compression_level_hint),
        has_preset_dictionary: snapshot.dict_id.is_some(),
        payload_size: snapshot.payload.len() as u64,
        content_digest: format!("{:016x}", hasher.finish()),
    }
}
```

## identity: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/🎨️paint-region/🖌️samples/🦀️.rs:9
- AST item function_item; attached and ancestor cfg: none.

```rust
pub fn tiff_revision(snapshot:&TiffSnapshot)->String {
 struct Fingerprint(u64);
 impl Fingerprint {fn bytes(&mut self,bytes:&[u8]){for byte in bytes{self.0=(self.0^u64::from(*byte)).wrapping_mul(0x100000001b3);}}fn count(&mut self,count:usize){self.bytes(&(count as u64).to_le_bytes());}}
 let mut hash=Fingerprint(0xcbf29ce484222325);hash.count(snapshot.schema.len());hash.bytes(snapshot.schema.as_bytes());hash.count(snapshot.ifds.len());
 for ifd in &snapshot.ifds {
  hash.count(ifd.entries.len());
  for tag in &ifd.entries {
   hash.bytes(&tag.tag.to_le_bytes());hash.bytes(&[tag.values.kind()as u8]);hash.count(tag.values.count()as usize);
   macro_rules! words {($values:expr)=>{for value in $values{hash.bytes(&value.to_le_bytes());}};}
   match &tag.values {
    TiffValues::Byte(v)|TiffValues::Undefined(v)=>hash.bytes(v),
    TiffValues::Ascii(texts)=>for text in texts{hash.count(text.len());hash.bytes(text.as_bytes());},
    TiffValues::Short(v)=>words!(v),TiffValues::Long(v)=>words!(v),TiffValues::SByte(v)=>for value in v{hash.bytes(&[*value as u8]);},
    TiffValues::SShort(v)=>words!(v),TiffValues::SLong(v)=>words!(v),
    TiffValues::Rational(v)=>for(a,b)in v{hash.bytes(&a.to_le_bytes());hash.bytes(&b.to_le_bytes());},
    TiffValues::SRational(v)=>for(a,b)in v{hash.bytes(&a.to_le_bytes());hash.bytes(&b.to_le_bytes());},
    TiffValues::Float(v)=>for value in v{hash.bytes(&value.bits.to_le_bytes());},
    TiffValues::Double(v)=>for value in v{hash.bytes(&value.lo.to_le_bytes());hash.bytes(&value.hi.to_le_bytes());}
   }
  }
  hash.count(ifd.blocks.len());for block in &ifd.blocks{for value in [block.x,block.y,block.width,block.height,u32::from(block.channels)]{hash.bytes(&value.to_le_bytes());}hash.count(block.samples.len());for sample in &block.samples{hash.bytes(&sample.lo.to_le_bytes());hash.bytes(&sample.hi.to_le_bytes());}}
 }
 format!("{:016x}",hash.0)
}
```

## identity: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs:25
- AST item function_item; attached and ancestor cfg: none.

```rust
pub fn bmp_revision(snapshot: &BmpSnapshot) -> String {
    let mut hash = 0xcbf29ce484222325;
    feed_counted(&mut hash, snapshot.schema.as_bytes());
    let image = &snapshot.image;
    feed_counted(&mut hash, image.profile.id().as_bytes());
    feed(&mut hash, &[u8::from(image.row_order == crate::schema::snapshot::BmpRowOrder::TopDown)]);
    for scalar in [image.width, image.height, image.colors_used, image.colors_important, image.reserved_1.into(), image.reserved_2.into()] { feed(&mut hash, &scalar.to_le_bytes()); }
    feed(&mut hash, &image.x_pixels_per_meter.to_le_bytes()); feed(&mut hash, &image.y_pixels_per_meter.to_le_bytes());
    for mask in image.masks { feed(&mut hash, &mask.to_le_bytes()); }
    feed(&mut hash, &(image.palette.len() as u64).to_le_bytes());
    for entry in &image.palette { feed(&mut hash, &[entry.r, entry.g, entry.b, entry.reserved]); }
    match &image.pixels {
        BmpPixels::Indexed { indices } => { feed(&mut hash, &[0]); feed_counted(&mut hash, indices); }
        BmpPixels::Direct { samples } => { feed(&mut hash, &[1]); feed(&mut hash, &(samples.len() as u64).to_le_bytes()); for sample in samples { for scalar in [sample.red, sample.green, sample.blue, sample.alpha, sample.reserved] { feed(&mut hash, &scalar.to_le_bytes()); } } }
    }
    feed_counted(&mut hash, &image.opaque_gap); feed_counted(&mut hash, &image.opaque_trailer);
    format!("{hash:016x}")
}
```

## identity: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs:49
- AST item function_item; attached and ancestor cfg: none.

```rust
fn require_revision(snapshot: &BmpSnapshot, revision: &str) -> Result<(), String> { let expected = bmp_revision(snapshot); if revision != expected { return Err(format!("bmp: stale owned revision {revision}; expected {expected}")); } Ok(()) }
```

## identity: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/⚙️operations/🪪️validation/🦀️.rs:7
- AST item function_item; attached and ancestor cfg: none.

```rust
pub fn revision(&self)->Option<String> {if self.stage>=5 {Some(format!("{:016x}",self.hash))}else{None}}
```

## identity: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs:82
- AST item function_item; attached and ancestor cfg: none.

```rust
fn hash_bytes(hash: &mut u64, bytes: &[u8]) {
    for byte in bytes {
        *hash ^= u64::from(*byte);
        *hash = hash.wrapping_mul(0x100_0000_01b3);
    }
}
```

## identity: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs:89
- AST item function_item; attached and ancestor cfg: none.

```rust
fn hash_node(hash: &mut u64, node: &XmlNode) {
    match node {
        XmlNode::Element { name, attrs, children } => {
            hash_bytes(hash, b"E");
            hash_bytes(hash, name.as_bytes());
            for attr in attrs {
                hash_bytes(hash, b"A");
                hash_bytes(hash, attr.name.as_bytes());
                hash_bytes(hash, b"=");
                hash_bytes(hash, attr.value.as_bytes());
            }
            for child in children {
                hash_node(hash, child);
            }
            hash_bytes(hash, b"/E");
        }
        XmlNode::Text { text } => {
            hash_bytes(hash, b"T");
            hash_bytes(hash, text.as_bytes());
        }
        XmlNode::CData { text } => {
            hash_bytes(hash, b"C");
            hash_bytes(hash, text.as_bytes());
        }
        XmlNode::Comment { text } => {
            hash_bytes(hash, b"M");
            hash_bytes(hash, text.as_bytes());
        }
        XmlNode::ProcessingInstruction { target, data } => {
            hash_bytes(hash, b"P");
            hash_bytes(hash, target.as_bytes());
            hash_bytes(hash, data.as_bytes());
        }
    }
}
```

## identity: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs:125
- AST item function_item; attached and ancestor cfg: none.

```rust
pub fn pptx_xml_subtree_revision(node: &XmlNode) -> String {
    let mut hash = 0xcbf2_9ce4_8422_2325;
    hash_node(&mut hash, node);
    format!("{hash:016x}")
}
```

## identity: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs:155
- AST item function_item; attached and ancestor cfg: none.

```rust
fn hash_bytes(hash: &mut u64, bytes: &[u8]) {
    for byte in bytes {
        *hash ^= u64::from(*byte);
        *hash = hash.wrapping_mul(0x100000001b3);
    }
}
```

## identity: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs:162
- AST item function_item; attached and ancestor cfg: none.

```rust
fn hash_field(hash: &mut u64, value: &str) {
    hash_bytes(hash, &(value.len() as u64).to_le_bytes());
    hash_bytes(hash, value.as_bytes());
}
```

## identity: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs:167
- AST item function_item; attached and ancestor cfg: none.

```rust
fn hash_node(hash: &mut u64, node: &XmlNode) {
    match node {
        XmlNode::Element { name, attrs, children } => {
            hash_bytes(hash, &[0]);
            hash_field(hash, name);
            hash_bytes(hash, &(attrs.len() as u64).to_le_bytes());
            for attr in attrs {
                hash_field(hash, &attr.name);
                hash_field(hash, &attr.value);
            }
            hash_bytes(hash, &(children.len() as u64).to_le_bytes());
            for child in children {
                hash_node(hash, child);
            }
        }
        XmlNode::Text { text } => {
            hash_bytes(hash, &[1]);
            hash_field(hash, text);
        }
        XmlNode::CData { text } => {
            hash_bytes(hash, &[2]);
            hash_field(hash, text);
        }
        XmlNode::Comment { text } => {
            hash_bytes(hash, &[3]);
            hash_field(hash, text);
        }
        XmlNode::ProcessingInstruction { target, data } => {
            hash_bytes(hash, &[4]);
            hash_field(hash, target);
            hash_field(hash, data);
        }
    }
}
```

## identity: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs:202
- AST item function_item; attached and ancestor cfg: none.

```rust
fn hash_shallow_node(hash: &mut u64, node: &XmlNode) {
    match node {
        XmlNode::Element { name, attrs, children } => {
            hash_bytes(hash, &[0]);
            hash_field(hash, name);
            hash_bytes(hash, &(attrs.len() as u64).to_le_bytes());
            for attr in attrs {
                hash_field(hash, &attr.name);
                hash_field(hash, &attr.value);
            }
            hash_bytes(hash, &(children.len() as u64).to_le_bytes());
        }
        XmlNode::Text { text } => {
            hash_bytes(hash, &[1]);
            hash_field(hash, text);
        }
        XmlNode::CData { text } => {
            hash_bytes(hash, &[2]);
            hash_field(hash, text);
        }
        XmlNode::Comment { text } => {
            hash_bytes(hash, &[3]);
            hash_field(hash, text);
        }
        XmlNode::ProcessingInstruction { target, data } => {
            hash_bytes(hash, &[4]);
            hash_field(hash, target);
            hash_field(hash, data);
        }
    }
}
```

## identity: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs:234
- AST item function_item; attached and ancestor cfg: none.

```rust
fn address_revision(root: &XmlNode, path: &[usize], replacement: Option<&XmlNode>) -> Result<String, ValueError> {
    let mut hash = 0xcbf29ce484222325;
    let mut node = root;
    for (depth, &index) in path.iter().enumerate() {
        let XmlNode::Element { children, .. } = node else { return Err(ValueError::new(ValueRefusalKind::InvalidValue,format!("node path descends through non-element at child {index}"))) };
        hash_shallow_node(&mut hash, node);
        hash_bytes(&mut hash, &(index as u64).to_le_bytes());
        for (child_index, child) in children.iter().enumerate() {
            let child = if depth + 1 == path.len() && child_index == index { replacement.unwrap_or(child) } else { child };
            hash_shallow_node(&mut hash, child);
        }
        node = children.get(index).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,format!("node path child {index} is outside {} children", children.len())))?;
    }
    hash_node(&mut hash, replacement.unwrap_or(node));
    Ok(format!("{hash:016x}"))
}
```

## identity: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs:252
- AST item function_item; attached and ancestor cfg: none.

```rust
pub fn docx_xml_subtree_revision(node: &XmlNode) -> String {
    let mut hash = 0xcbf29ce484222325;
    hash_node(&mut hash, node);
    format!("{hash:016x}")
}
```

## identity: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs:301
- AST item function_item; attached and ancestor cfg: none.

```rust
pub(super) fn revision_after_replacement(snapshot: &DocxSnapshot, address: &DocxXmlAddress, replacement: &XmlNode) -> Result<String, ValueError> {
    let document = materialized_document(snapshot, &address.part_path)?;
    let root = document.root.as_ref().ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,format!("DOCX XML part {} has no root", address.part_path)))?;
    address_revision(root, &address.node_path, Some(replacement))
}
```

## identity: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/💡️inferences/🗃️entries/🦀️.rs:28
- AST item function_item; attached and ancestor cfg: none.

```rust
pub fn compute_zip_entries(snapshot: &ZipSnapshot) -> ZipEntries {
    let mut hasher = DefaultHasher::new();
    let mut total_uncompressed_size: u64 = 0;
    for entry in &snapshot.entries {
        entry.name.hash(&mut hasher);
        entry.data.hash(&mut hasher);
        total_uncompressed_size += entry.data.len() as u64;
    }
    ZipEntries { entry_count: snapshot.entries.len() as u32, total_uncompressed_size, content_digest: format!("{:016x}", hasher.finish()) }
}
```

## identity: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️cell-address/🦀️.rs:76
- AST item function_item; attached and ancestor cfg: none.

```rust
fn validate_revision(revision: &str, kind: &str) -> Result<(), String> {
    if revision.len() != 16 || !revision.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)) {
        return Err(format!("XLSX {kind} address revision is invalid"));
    }
    Ok(())
}
```

## identity: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️cell-address/🦀️.rs:100
- AST item function_item; attached and ancestor cfg: none.

```rust
fn hash_bytes(hash: &mut u64, bytes: &[u8]) {
    for byte in bytes {
        *hash ^= u64::from(*byte);
        *hash = hash.wrapping_mul(0x100000001b3);
    }
}
```

## identity: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️cell-address/🦀️.rs:107
- AST item function_item; attached and ancestor cfg: none.

```rust
fn hash_field(hash: &mut u64, value: &str) {
    hash_bytes(hash, &(value.len() as u64).to_le_bytes());
    hash_bytes(hash, value.as_bytes());
}
```

## identity: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️cell-address/🦀️.rs:112
- AST item function_item; attached and ancestor cfg: none.

```rust
fn hash_node(hash: &mut u64, node: &XmlNode, recursive: bool) {
    match node {
        XmlNode::Element { name, attrs, children } => {
            hash_bytes(hash, &[0]);
            hash_field(hash, name);
            hash_bytes(hash, &(attrs.len() as u64).to_le_bytes());
            for attr in attrs {
                hash_field(hash, &attr.name);
                hash_field(hash, &attr.value);
            }
            hash_bytes(hash, &(children.len() as u64).to_le_bytes());
            if recursive {
                for child in children {
                    hash_node(hash, child, true);
                }
            }
        }
        XmlNode::Text { text } | XmlNode::CData { text } | XmlNode::Comment { text } => hash_field(hash, text),
        XmlNode::ProcessingInstruction { target, data } => {
            hash_field(hash, target);
            hash_field(hash, data);
        }
    }
}
```

## identity: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️cell-address/🦀️.rs:171
- AST item function_item; attached and ancestor cfg: none.

```rust
fn address_revision(snapshot: &XlsxSnapshot, root: &XmlNode, path: &[usize]) -> Result<String, String> {
    let mut hash = 0xcbf29ce484222325;
    let mut node = root;
    for &index in path {
        let XmlNode::Element { children, .. } = node else { return Err(format!("node path descends through non-element at child {index}")) };
        hash_node(&mut hash, node, false);
        hash_bytes(&mut hash, &(index as u64).to_le_bytes());
        for child in children {
            hash_node(&mut hash, child, false);
        }
        node = children.get(index).ok_or_else(|| format!("node path child {index} is outside {} children", children.len()))?;
    }
    hash_node(&mut hash, node, true);
    if let Some(entry) = referenced_shared_string(snapshot, root, path)? {
        hash_node(&mut hash, entry, true);
    }
    Ok(format!("{hash:016x}"))
}
```

## identity: ✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:432
- AST item function_item; attached and ancestor cfg: none.

```rust
fn hash_intrinsic(value: &semio_framework_value::DslValue, hasher: &mut impl Hasher) {
    use semio_framework_value::{DslValue, Number};
    std::mem::discriminant(value).hash(hasher);
    match value {
        DslValue::Null => {},
        DslValue::Bool(value) => value.hash(hasher),
        DslValue::Number(value) => {
            std::mem::discriminant(value).hash(hasher);
            match value { Number::UInt(value) => value.hash(hasher), Number::Int(value) => value.hash(hasher), Number::Float(value) => value.to_bits().hash(hasher) }
        },
        DslValue::String(value) => value.hash(hasher),
        DslValue::Bytes(value) => value.hash(hasher),
        DslValue::Array(values) => { values.len().hash(hasher); for value in values { hash_intrinsic(value, hasher); } },
        DslValue::Object(values) => { values.len().hash(hasher); for (key, value) in values { key.hash(hasher); hash_intrinsic(value, hasher); } },
    }
}
```

## identity: ✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:449
- AST item function_item; attached and ancestor cfg: none.

```rust
fn hash_value<T: ToValue>(value: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    let value = semio_framework_value::DecodedValue::new(value.to_value(), <semio_framework_value::DslValue as FromValue>::retire_decoded);
    hash_intrinsic(value.get(), &mut hasher);
    hasher.finish()
}
```

## identity: ✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:456
- AST item function_item; attached and ancestor cfg: none.

```rust
fn prefix_signature(stock_signature: u64, steps: &[&ProcessStep]) -> u64 {
    let mut hasher = DefaultHasher::new();
    stock_signature.hash(&mut hasher);
    steps.len().hash(&mut hasher);
    for step in steps {
        let value = semio_framework_value::DecodedValue::new(step.to_value(), <semio_framework_value::DslValue as FromValue>::retire_decoded);
        hash_intrinsic(value.get(), &mut hasher);
    }
    hasher.finish()
}
```

## custom-facet: ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾change-data-fields/🦀️.rs:18
- AST item impl_item; attached and ancestor cfg: #[cfg(test)].

```rust
impl serde::Serialize for ChangeDataFields {fn serialize<S:serde::Serializer>(&self,serializer:S)->Result<S::Ok,S::Error>{serde::Serialize::serialize(&serde_json::Value::from(semio_framework_value::ToValue::to_value(self)),serializer)}}
```

## custom-facet: ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾change-data-fields/🦀️.rs:20
- AST item impl_item; attached and ancestor cfg: #[cfg(test)].

```rust
impl<'de>serde::Deserialize<'de>for ChangeDataFields{fn deserialize<D:serde::Deserializer<'de>>(deserializer:D)->Result<Self,D::Error>{let value=<serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;<Self as semio_framework_value::FromValue>::from_value(semio_framework_value::DslValue::from(&value)).map_err(serde::de::Error::custom)}}
```

## custom-facet: ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧾️dictionary/🦀️.rs:44
- AST item impl_item; attached and ancestor cfg: #[cfg(test)].

```rust
impl serde::Serialize for FormDictionary{fn serialize<S:serde::Serializer>(&self,serializer:S)->Result<S::Ok,S::Error>{serde::Serialize::serialize(&serde_json::Value::from(self.to_value()),serializer)}}
```

## custom-facet: ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧾️dictionary/🦀️.rs:46
- AST item impl_item; attached and ancestor cfg: #[cfg(test)].

```rust
impl<'de>serde::Deserialize<'de>for FormDictionary{fn deserialize<D:serde::Deserializer<'de>>(deserializer:D)->Result<Self,D::Error>{let value=<serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;Self::from_value(DslValue::from(&value)).map_err(serde::de::Error::custom)}}
```


## Direct Identity Call Evidence

Exact distinctive symbols; definitions and inline cfg(test) calls remain visible as source evidence, not production counts.

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🔨️geometry-core/🦀️.rs:151:pub(crate) fn byte_fingerprint(bytes: &[u8]) -> String {`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️.rs:1810:        hash_value(hasher, value);`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➕️add-layer/🦀️.rs:53:        let id = crate::standards::v1::subsets::any::schema::create_drawing_id("layer", &candidate);`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎛️edit-selection/🦀️.rs:65:            layer_base_mut(&mut group).id = crate::standards::v1::subsets::any::schema::create_drawing_id("group", material.as_bytes()).into();`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:11:pub(crate) fn drawing_id_hex(material: &[u8]) -> String {`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:18:pub fn create_drawing_id(prefix: &str, material: &[u8]) -> String {`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:19:    format!("{prefix}-{}", drawing_id_hex(material))`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:26:            id: create_drawing_id("path", name.as_bytes()).into(),`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:240:    DrawingLayerBase { id: create_drawing_id("layer", name.as_bytes()).into(), name: name.into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: default_drawing_transform(), attributes: DrawingAttributes::default() }`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:249:            id: create_drawing_id("group", name.as_bytes()).into(),`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:265:            id: create_drawing_id("boolean", name.as_bytes()).into(),`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:282:            id: create_drawing_id("trace", name.as_bytes()).into(),`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:299:            id: create_drawing_id("shape", name.as_bytes()).into(),`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:320:            id: create_drawing_id("text", name.as_bytes()).into(),`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:339:            id: create_drawing_id("image", name.as_bytes()).into(),`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:673:        base.id = create_drawing_id("layer", format!("{old}{suffix}").as_bytes()).into();`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/💾️binary/📸️snapshot/🧮️decoded-accessors/🦀️.rs:16:        buffer_fingerprints.push(format!("buffer:{index}:{}",crate::standards::v2_0::subsets::any::schema::inferences::geometry_core::byte_fingerprint(bytes)));`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs:21:        let mut job=document::SvgImportJob::new(source,&crate::standards::v1::subsets::any::schema::create_drawing_id("svg",source.as_bytes())).map_err(error)?;`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🦀️.rs:241:    crate::standards::v1::subsets::any::schema::create_drawing_id("shape", &identity)`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:199:    let stock_signature = hash_value(&scene.stock);`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:209:        let signature = prefix_signature(stock_signature, &enabled_steps[..start]);`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:221:        session.tables.memo.insert(prefix_signature(stock_signature, &[]), stock.clone());`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:236:        session.tables.memo.insert(prefix_signature(stock_signature, &enabled_steps[..=index]), handle.clone());`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:456:fn prefix_signature(stock_signature: u64, steps: &[&ProcessStep]) -> u64 {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:30:        Self { extent: compute_binary_extent(snapshot) }`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:42:        Self { extent: compute_binary_extent(snapshot) }`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/💡️inferences/📏extent/🦀️.rs:25:pub fn compute_binary_extent(snapshot: &BinarySnapshot) -> BinaryExtent {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/💡️inferences/📏extent/🦀️.rs:33:        compute_binary_extent(&BinarySnapshot::default())`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/✏️editor/🎭️modes/✏️edit/🎮️commands/🎨️paint-region/🦀️.rs:101:    Ok(tiff_revision(snapshot))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/🎨️paint-region/🦀️.rs:64:    TiffMutation::PaintRegion(PaintRegionMutation { revision: crate::standards::v6_0::subsets::document::schema::mutations::paint_region::samples::tiff_revision(&base), ifd_index: 0, x: 0, y: 0, width: 1, height: 1, red: 0, green: 0, blue: 0, alpha: 255 })`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/🎨️paint-region/🖌️samples/🦀️.rs:9:pub fn tiff_revision(snapshot:&TiffSnapshot)->String {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/🎨️paint-region/🖌️samples/🦀️.rs:62: if revision!=tiff_revision(snapshot){return Err("tiff: stale owned revision".into())}validate_tiff_region_paint(snapshot,ifd_index,region,color)?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs:125:pub fn pptx_xml_subtree_revision(node: &XmlNode) -> String {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs:166:    Ok(PptxXmlAddress { part_path: part_path.into(), node_path, namespace_uri, local_name, revision: pptx_xml_subtree_revision(node) })`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs:170:pub fn pptx_xml_address(snapshot: &PptxSnapshot, part_path: &str, node_path: Vec<usize>) -> Result<PptxXmlAddress, String> {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs:181:    if namespace_uri != address.namespace_uri || local_name != address.local_name || pptx_xml_subtree_revision(node) != address.revision {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs:188:    resolve_pptx_xml_address(snapshot, address)?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs:373:    let node = resolve_pptx_xml_address(snapshot, &address.node)?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs:505:    Ok(PptxXmlAddress { part_path: part_path.into(), node_path, namespace_uri, local_name, revision: pptx_xml_subtree_revision(node) })`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs:511:    resolve_pptx_xml_address(snapshot, container)?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs:646:    let previous = resolve_pptx_xml_address(snapshot, address)?.clone();`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs:666:    let node = resolve_pptx_xml_address(snapshot, &address.node)?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs:688:    let node = resolve_pptx_xml_address(snapshot, &address.node)?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:129:            leaves.push(PptxMutation::ReplaceXmlNode(replace_xml_node::ReplaceXmlNode { address: xml_address::pptx_xml_address(base, &part.path, Vec::new()).ok()?, node: part.document.root.clone()? }));`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:281:    let slide_entry = xml_address::resolve_pptx_xml_address(&fixture, &slides[0].address.entry).expect("canonical slide entry").clone();`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:282:    let shape_node = xml_address::resolve_pptx_xml_address(&fixture, &first_shape.node).expect("canonical shape node").clone();`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/💡️inferences/🗃️entries/🦀️.rs:28:pub fn compute_zip_entries(snapshot: &ZipSnapshot) -> ZipEntries {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/💡️inferences/🗃️entries/🦀️.rs:41:        compute_zip_entries(&ZipSnapshot::default())`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/💡️inferences/🦀️.rs:27:        Self { entries: compute_zip_entries(snapshot) }`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/💡️inferences/🦀️.rs:39:        Self { entries: compute_zip_entries(snapshot) }`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:29:        Self { window: compute_deflate_window(snapshot) }`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:42:        Self { window: compute_deflate_window(snapshot) }`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/💡️inferences/🪟window/🦀️.rs:31:        compute_deflate_window(&DeflateSnapshot::default())`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/💡️inferences/🪟window/🦀️.rs:41:pub fn compute_deflate_window(snapshot: &DeflateSnapshot) -> DeflateWindow {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs:74:    let resolved = resolve_docx_xml_address(snapshot, address)?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs:252:pub fn docx_xml_subtree_revision(node: &XmlNode) -> String {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs:259:pub fn docx_xml_address(snapshot: &DocxSnapshot, part_path: impl AsRef<str>, node_path: Vec<usize>) -> Result<DocxXmlAddress, ValueError> {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs:278:pub fn resolve_docx_xml_address(snapshot: &DocxSnapshot, address: &DocxXmlAddress) -> Result<ResolvedDocxXmlAddress, ValueError> {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs:382:    docx_xml_address(snapshot, part_path, block_path)`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs:419:    let address = docx_xml_address(snapshot, part_path, run_path)?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs:420:    let resolved = resolve_docx_xml_address(snapshot, &address)?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs:468:    let resolved = resolve_docx_xml_address(snapshot, address)?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs:566:    docx_xml_address(snapshot, part_path, run_path)`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs:823:    let resolved = resolve_docx_xml_address(snapshot, address)?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:234:    let resolved = resolve_docx_xml_address(snapshot, address)?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:245:    let resolved = resolve_docx_xml_address(snapshot, parent)?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:251:    let revision = docx_xml_subtree_revision(&node);`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:261:    let resolved = resolve_docx_xml_address(snapshot, parent)?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:268:    let actual_revision = docx_xml_subtree_revision(&previous);`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:284:            let resolved = resolve_docx_xml_address(snapshot, address)?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:289:            let resolved = resolve_docx_xml_address(snapshot, address)?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:293:            let resolved = resolve_docx_xml_address(snapshot, address)?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:297:            let resolved = resolve_docx_xml_address(snapshot, address)?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:302:            let resolved = resolve_docx_xml_address(snapshot, address)?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:305:            child_remove_plan(snapshot, address, physical, &expected_name, &docx_xml_subtree_revision(&row))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:369:    let resolved = resolve_docx_xml_address(base, &container)?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:373:    child_remove_plan(base, &container, slot, &expected_name, &docx_xml_subtree_revision(node))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:382:    let address = docx_xml_address(base, &container.part_path, node_path)?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:409:    let resolved = resolve_docx_xml_address(base, &root)?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:413:    child_remove_plan(base, &root, at, &expected_name, &docx_xml_subtree_revision(node))`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:424:    let style = docx_xml_address(base, &root.part_path, style_path.clone())?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:425:    let resolved = resolve_docx_xml_address(base, &style)?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:433:            replacement_plan(base, &docx_xml_address(base, &root.part_path, path)?, element)?`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:435:        (Some(index), None) => child_remove_plan(base, &style, index, property, &docx_xml_subtree_revision(&children[index]))?,`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:823:    let mut node = resolve_docx_xml_address(&base, &address).expect("demo run").node.clone();`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:829:    let resolved_run = resolve_docx_xml_address(&base, &address).expect("demo run");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:834:    let removed_revision = docx_xml_subtree_revision(&removed);`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs:87:                        let address = xlsx_cell_address(document, &sheet.name, row, column).map_err(|error| PluginAssemblyError::new("xlsx.cell-address", error))?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/🦀️.rs:302:        let address = xlsx_cell_address(snapshot, sheet_name, row, column).map_err(|message| fault("cell-stale", message))?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:174:                    let leaf = cell_address::xlsx_cell_address(&state, &new_sheet.name, cell.row, cell.col).ok().map(|address| XlsxMutation::SetCell(set_cell::SetCell { address, value: cell.value.clone(), node: None }));`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:185:                let leaf = cell_address::xlsx_cell_address(&state, &new_sheet.name, cell.row, cell.col).ok().map(|address| XlsxMutation::RemoveCell(remove_cell::RemoveCell { address }));`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:414:    let address = cell_address::xlsx_cell_address(&base, "Sheet1", 1, 0).expect("fixture cell address");`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs:422:        XlsxMutation::RemoveCell(remove_cell::RemoveCell { address: cell_address::xlsx_cell_address(&base, "Sheet1", 1, 0).expect("fixture cell address") }),`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️cell-address/🦀️.rs:304:    match xlsx_cell_address(snapshot, sheet_name, row, column) {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️cell-address/🦀️.rs:311:pub fn xlsx_cell_address(snapshot: &XlsxSnapshot, sheet_name: &str, row: u32, column: u32) -> Result<XlsxCellAddress, String> {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️cell-address/🦀️.rs:423:    resolve_xlsx_cell_address(snapshot, address)?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️cell-address/🦀️.rs:430:    let part_index = resolve_xlsx_cell_address(snapshot, address)?.part_index;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️canonical-edit/🦀️.rs:247:    let resolved = cell_address::resolve_xlsx_cell_address(snapshot, address)?;`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️canonical-edit/🦀️.rs:352:    let resolved = cell_address::resolve_xlsx_cell_address(snapshot, address)?;`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🆔digest/🦀️.rs:15:pub fn compute_content_digest(snapshot: &SHomeSnapshot) -> String {`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:33:        Self { content_digest: compute_content_digest(snapshot) }`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:47:        Self { content_digest: compute_content_digest(snapshot) }`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs:25:pub fn bmp_revision(snapshot: &BmpSnapshot) -> String {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs:49:fn require_revision(snapshot: &BmpSnapshot, revision: &str) -> Result<(), String> { let expected = bmp_revision(snapshot); if revision != expected { return Err(format!("bmp: stale owned revision {revision}; expected {expected}")); } Ok(()) }`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🎮️commands/🩹️patch-pixel-region/🦀️.rs:161:            self.revision = Some(png_revision(input.snapshot));`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs:26:pub fn png_revision(snapshot: &PngSnapshot) -> String {`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs:47:pub fn require_revision(snapshot: &PngSnapshot, revision: &str) -> Result<(), String> { if png_revision(snapshot) != revision { return Err("png: stale source revision".into()); } Ok(()) }`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️paint-native-samples/🦀️.rs:45:    let revision = crate::schema::operations::png_revision(&base);`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌗️change-gamma/🦀️.rs:42:    PngMutation::ChangeGamma(ChangeGammaMutation { revision: crate::schema::operations::png_revision(&base), gama: Some(45_455) })`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹️patch-pixels/🦀️.rs:56:    PngMutation::PatchPixels(PatchPixelsMutation { revision: crate::schema::operations::png_revision(&base), x: 0, y: 0, width: 1, height: 1, red: 0, green: 0, blue: 0, alpha: 255 })`

### Direct Package Imports: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️cell-address/🦀️.rs

- `use crate::standards::v_ecma_376::subsets::base::schema::vocabulary::{attribute_value, column_letter, element_matches, expanded_element_name, namespace_scope, REL_TYPE_OFFICE_DOCUMENT_STRICT, R_NS, R_NS_STRICT, SML_NS, SML_NS_STRICT};`
- `use crate::XlsxSnapshot;`
- `use semio_s_artifact_stdio_xml::schema::snapshot::XmlNode;`
- `use semio_s_artifact_stdio_zip::opc::{resolve_relationship_target, REL_TYPE_OFFICE_DOCUMENT};`

### Direct Package Imports: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs

- `use semio_framework_value::{ValueError,ValueRefusalKind};`
- `use super::*;`
- `use crate::standards::v_ecma_376::subsets::base::schema::namespaces::{apply_bindings, expanded_name, is_word_name, qualified_word_prefix, set_word_attr, word_attr, XML_NAMESPACE};`

### Direct Package Imports: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs

- `use super::{move_slide, remove_slide, remove_shape, insert_slide, insert_shape, replace_xml_node, PptxMutation};`
- `use crate::schema::diff::{NamedModified, NamedTripleDiff, PptxDiff, PptxXmlPartDiff};`
- `use crate::schema::snapshot::{PptxTransform, PptxXmlPart};`
- `use semio_s_artifact_stdio_xml::schema::diff::{diff_at_path, XmlChildAdded, XmlChildModified, XmlChildrenDiff, XmlElementDiff, XmlNodeDiff};`
- `use crate::standards::v_ecma_376::subsets::base::{schema::{vocabulary::{attribute_value,element_matches,expanded_element_name,namespace_scope,resolve_office_document_relationship,DRAWINGML_NAMESPACES,OFFICE_RELATIONSHIP_NAMESPACES,PRESENTATIONML_NAMESPACES}}};`
- `use crate::PptxSnapshot;`
- `use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlNode};`
- `use semio_s_artifact_stdio_zip::opc::resolve_relationship_target;`

## Supplementary Exact Source Inputs

### `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:11` through20

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

### `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:844` through855

```rust
pub fn hex_to_rgba(hex: &str, alpha: f64) -> [f64; 4] {
    let normalized = hex.trim_start_matches('#');
    let value = if normalized.len() == 3 { normalized.chars().map(|c| format!("{c}{c}")).collect::<String>() } else { normalized.to_string() };
    let parse = |start: usize| u8::from_str_radix(&value[start..start + 2], 16).unwrap_or(0) as f64 / 255.0;
    [parse(0), parse(2), parse(4), alpha]
}

pub fn rgba_to_hex(color: [f64; 4]) -> String {
    let channel = |value: f64| format!("{:02x}", (value.clamp(0.0, 1.0) * 255.0).round() as u8);
    format!("#{}{}{}", channel(color[0]), channel(color[1]), channel(color[2]))
}
//#endregion 🔖️Tree
```

### `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🌿️cas-internals/🦀️.rs:310` through402

```rust
    /// 🔢️ FNV-1a, computed bottom-up once at construction and cached — equality checks hash first (cheap
    /// reject), then `Rc::ptr_eq` (cheap accept, common since subtrees are shared), then a full structural
    /// compare only in the rare remaining case.
    fn fnv1a_mix(mut hash: u64, bytes: &[u8]) -> u64 {
        for &b in bytes {
            hash ^= b as u64;
            hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
        }
        hash
    }

    const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;

    fn hash_kind(kind: &Kind) -> u64 {
        let mut h = FNV_OFFSET;
        match kind {
            Kind::Integer(n) => {
                h = fnv1a_mix(h, b"int");
                h = fnv1a_mix(h, n.to_decimal().as_bytes());
            }
            Kind::Rational(r) => {
                h = fnv1a_mix(h, b"rat");
                h = fnv1a_mix(h, r.numer().to_decimal().as_bytes());
                h = fnv1a_mix(h, r.denom().to_decimal().as_bytes());
            }
            Kind::Symbol(s) => {
                h = fnv1a_mix(h, b"sym");
                h = fnv1a_mix(h, s.name.as_bytes());
                h = fnv1a_mix(h, &s.assumptions.bits().to_le_bytes());
            }
            Kind::Constant(c) => {
                h = fnv1a_mix(h, b"const");
                h = fnv1a_mix(h, c.name().as_bytes());
            }
            Kind::Bool(b) => {
                h = fnv1a_mix(h, b"bool");
                h = fnv1a_mix(h, &[*b as u8]);
            }
            Kind::Add(terms) => {
                h = fnv1a_mix(h, b"add");
                for t in terms {
                    h = fnv1a_mix(h, &t.hash().to_le_bytes());
                }
            }
            Kind::Mul(factors) => {
                h = fnv1a_mix(h, b"mul");
                for f in factors {
                    h = fnv1a_mix(h, &f.hash().to_le_bytes());
                }
            }
            Kind::Pow(base, exp) => {
                h = fnv1a_mix(h, b"pow");
                h = fnv1a_mix(h, &base.hash().to_le_bytes());
                h = fnv1a_mix(h, &exp.hash().to_le_bytes());
            }
            Kind::Fn(kind, args) => {
                h = fnv1a_mix(h, b"fn");
                h = fnv1a_mix(h, kind.name().as_bytes());
                for a in args {
                    h = fnv1a_mix(h, &a.hash().to_le_bytes());
                }
            }
            Kind::RootOf { coeffs, index } => {
                h = fnv1a_mix(h, b"rootof");
                for c in coeffs {
                    h = fnv1a_mix(h, c.numer().to_decimal().as_bytes());
                    h = fnv1a_mix(h, c.denom().to_decimal().as_bytes());
                }
                h = fnv1a_mix(h, &index.to_le_bytes());
            }
            Kind::Piecewise(cases) => {
                h = fnv1a_mix(h, b"piecewise");
                for (v, c) in cases {
                    h = fnv1a_mix(h, &v.hash().to_le_bytes());
                    h = fnv1a_mix(h, &c.hash().to_le_bytes());
                }
            }
            Kind::Rel(operation, a, b) => {
                h = fnv1a_mix(h, b"rel");
                h = fnv1a_mix(h, &[*operation as u8]);
                h = fnv1a_mix(h, &a.hash().to_le_bytes());
                h = fnv1a_mix(h, &b.hash().to_le_bytes());
            }
            Kind::Wild(id, _) => {
                h = fnv1a_mix(h, b"wild");
                h = fnv1a_mix(h, &id.to_le_bytes());
            }
        }
        h
    }
    // #endregion 🔖️Node

    // #region 🔖️Expr
```
