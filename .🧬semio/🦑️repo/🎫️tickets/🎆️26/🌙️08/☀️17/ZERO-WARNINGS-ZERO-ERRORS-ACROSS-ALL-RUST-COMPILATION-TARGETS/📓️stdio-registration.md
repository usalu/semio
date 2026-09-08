# Stdio Registration Audit

Pass 345: strict WASI324 reports 143 discarded registration futures. These must execute registration instead of merely dropping a future. The current source contains 104 registration bodies in the diagnosed files.

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🦀️.rs:81

```rust
pub fn register() {
        let _ = register_composer_entries(v_raw::entries());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🚪️io/🦀️.rs:76

```rust
pub fn register() {
    let _ = semio_framework_plugin::register_composer_entries(io_registry::entries());
    register_artifact_schema();
    register_artifact_inferences();
    register_pilot_languages();
    register_schema_specs();
    let _ = store::register_document_codec(store::ArtifactCodec::of::<
        crate::artifacts::binary::standards::v_raw::subsets::any::schema::snapshot::BinarySnapshot,
        crate::artifacts::binary::standards::v_raw::subsets::any::schema::mutations::BinaryMutation,
    >(crate::artifacts::binary::STDIO_BINARY_DOCUMENT_SCHEMA));
}
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🦀️.rs:75

```rust
pub fn register() {
        let _ = register_composer_entries(v_utf_8::entries());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/🦀️.rs:57

```rust
pub fn register() {
    crate::artifacts::txt::io_registry::register();
    register_artifact_schema();
    register_artifact_inferences();
    register_pilot_languages();
    register_schema_specs();
    let _ = store::register_document_codec(store::ArtifactCodec::of::<TxtSnapshot, TxtMutation>(STDIO_TXT_DOCUMENT_SCHEMA));
}
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🦀️.rs:168

```rust
pub fn register() {
        let _ = register_composer_entries(v_rfc8259::entries());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/🚪️io/🦀️.rs:84

```rust
pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🦀️.rs:174

```rust
pub fn register() {
        let _ = register_composer_entries(v1_0::entries());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✅️valid/🚪️io/🦀️.rs:84

```rust
pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🦀️.rs:155

```rust
pub fn register() {
        let _ = register_composer_entries(v_rfc4180::entries());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🦀️.rs:169

```rust
pub fn register() {
        let _ = register_composer_entries(v_commonmark::entries());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🦀️.rs:164

```rust
pub fn register() {
        let _ = register_composer_entries(v_rfc1950::entries());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🦀️.rs:172

```rust
pub fn register() {
        let _ = register_composer_entries(v2_0::entries());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/🚪️io/🦀️.rs:121

```rust
pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🦀️.rs:188

```rust
pub fn register() {
        let _ = register_composer_entries(v_ap214::entries());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/1️⃣cc1/🚪️io/🦀️.rs:89

```rust
pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/2️⃣cc2/🚪️io/🦀️.rs:89

```rust
pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/3️⃣cc3/🚪️io/🦀️.rs:89

```rust
pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/4️⃣cc4/🚪️io/🦀️.rs:89

```rust
pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/5️⃣cc5/🚪️io/🦀️.rs:89

```rust
pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/6️⃣cc6/🚪️io/🦀️.rs:89

```rust
pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🦀️.rs:89

```rust
pub fn register() {
        let _ = register_composer_entries(v4::entries());
        let _ = register_composer_entries(v2x3::entries());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🦀️.rs:274

```rust
pub fn register() {
    crate::artifacts::ifc::io_registry::register();
    register_artifact_schema();
    register_artifact_inferences();
    register_pilot_languages();
    let _ = store::register_document_codec(store::ArtifactCodec::of::<IfcSnapshot, IfcMutation>(STDIO_IFC_DOCUMENT_SCHEMA));
}
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/🦀️.rs:329

```rust
pub fn register() {
    ::schema::register_artifact_schema_descriptor(ifc2x3_artifact_schema_descriptor());
    register_artifact_inferences();
    register_pilot_languages();
    let _ = store::register_document_codec(store::ArtifactCodec::of::<Ifc2x3Snapshot, crate::artifacts::ifc::standards::v2x3::subsets::base::schema::mutations::Ifc2x3Mutation>(
        crate::artifacts::ifc::standards::v2x3::subsets::base::schema::snapshot::STDIO_IFC2X3_DOCUMENT_SCHEMA,
    ));
    // 🛡️ D5's generic validate-on-build hook: registers each real subset's `SubsetValidator` so
    // `io_dispatch`/`wire_artifact_compose` re-check them for free. Each subset's `ComposerEntry`
    // is registered separately via this standard's own `composer::entries()` aggregation.
    crate::artifacts::ifc::standards::v2x3::subsets::cv20::io::register();
    crate::artifacts::ifc::standards::v2x3::subsets::sav::io::register();
    crate::artifacts::ifc::standards::v2x3::subsets::cobie::io::register();
}
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🤝️cv20/🚪️io/🦀️.rs:81

```rust
pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧮️sav/🚪️io/🦀️.rs:76

```rust
pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🏢️cobie/🚪️io/🦀️.rs:76

```rust
pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🦀️.rs:161

```rust
pub fn register() {
        let _ = register_composer_entries(v1_0::entries());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🦀️.rs:627

```rust
pub fn register() {
        let _ = register_composer_entries(v2_0::entries());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🦀️.rs:161

```rust
pub fn register() {
        let _ = register_composer_entries(v3_0::entries());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🦀️.rs:155

```rust
pub fn register() {
        let _ = register_composer_entries(v1_0::entries());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🦀️.rs:156

```rust
pub fn register() {
        let _ = register_composer_entries(v_r12::entries());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/🦀️.rs:155

```rust
pub fn register() {
        let _ = register_composer_entries(v_ascii::entries());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🦀️.rs:186

```rust
pub fn register() {
        let _ = register_composer_entries(v1_1::entries());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔬️tiny/🚪️io/🦀️.rs:91

```rust
pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔰️basic/🚪️io/🦀️.rs:90

```rust
pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🦀️.rs:70

```rust
pub fn register() {
        let _ = register_composer_entries(v_v3::entries());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🚪️io/🦀️.rs:528

```rust
pub fn register() {
    crate::artifacts::bmp::io_registry::register();
    register_artifact_schema();
    register_artifact_inferences();
    register_pilot_languages();
    register_schema_specs();
    let _ = store::register_document_codec(store::ArtifactCodec::of::<BmpSnapshot, BmpMutation>(STDIO_BMP_DOCUMENT_SCHEMA));
}
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🦀️.rs:201

```rust
pub fn register() {
        let _ = register_composer_entries(v_ac1018::entries());
        let _ = register_composer_entries(v_ac1024::entries());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🦀️.rs:152

```rust
pub fn register() {
    io_registry::register();
    ::schema::register_artifact_schema_descriptor(crate::artifacts::png::standards::v1_2::subsets::any::schema::png_artifact_schema_descriptor());
    ::schema::register_artifact_inference_descriptor(crate::artifacts::png::standards::v1_2::subsets::any::schema::inferences::png_artifact_inference_descriptor());
    for lang in pilot_languages() {
        dsl::register_language(*lang);
    }
    let _ = store::register_document_codec(store::ArtifactCodec::of::<PngSnapshot, PngMutation>(STDIO_PNG_DOCUMENT_SCHEMA));
}
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🦀️.rs:183

```rust
pub fn register() {
        let _ = register_composer_entries(v1_2::entries());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🦀️.rs:250

```rust
pub fn register() {
        let _ = register_composer_entries(v1_4::entries());
        let _ = register_composer_entries(v1_7::entries());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🚪️io/🦀️.rs:76

```rust
pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🚪️io/🦀️.rs:76

```rust
pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🚪️io/🦀️.rs:90

```rust
pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🚪️io/🦀️.rs:83

```rust
pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🚪️io/🦀️.rs:80

```rust
pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🚪️io/🦀️.rs:80

```rust
pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🚪️io/🦀️.rs:83

```rust
pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🚪️io/🦀️.rs:76

```rust
pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🦀️.rs:182

```rust
pub fn register() {
        let _ = register_composer_entries(v_jfif_1_01::entries());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧱️baseline/🚪️io/🦀️.rs:86

```rust
pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🦀️.rs:77

```rust
pub fn register() {
        let _ = register_composer_entries(v87a::entries());
        let _ = register_composer_entries(v89a::entries());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🚪️io/🦀️.rs:670

```rust
pub fn register() {
    crate::artifacts::gif::io_registry::register();
    ::schema::register_artifact_schema_descriptor(crate::artifacts::gif::standards::v87a::subsets::any::schema::gif_artifact_schema_descriptor());
    register_artifact_inferences();
    register_pilot_languages();
    register_schema_specs();
    let _ = store::register_document_codec(store::ArtifactCodec::of::<GifSnapshot, GifMutation>(STDIO_GIF_DOCUMENT_SCHEMA));
}
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🧬️migrations/🦀️.rs:89

```rust
pub fn register() {
    let _ = store::register_dialect_migration(store::DialectMigration {
        from: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.gif".into(), standard: "87a".into(), subset: "*".into() },
        to: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.gif".into(), standard: "89a".into(), subset: "*".into() },
        lossless: true,
        migrate_pack: migrate_87a_to_89a_pack,
    });
}
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🚪️io/🦀️.rs:411

```rust
pub fn register() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::gif::standards::v89a::subsets::any::schema::gif_artifact_schema_descriptor());
    register_artifact_inferences();
    register_pilot_languages();
    register_schema_specs();
    let _ = store::register_document_codec(store::ArtifactCodec::of::<GifSnapshot, GifMutation>(STDIO_GIF89A_DOCUMENT_SCHEMA));
}
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🦀️.rs:178

```rust
pub fn register() {
        let _ = register_composer_entries(v6_0::entries());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline/🚪️io/🦀️.rs:75

```rust
pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🦀️.rs:176

```rust
pub fn register() {
        let _ = register_composer_entries(v_ecma_376::entries());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/📏️strict/🚪️io/🦀️.rs:85

```rust
pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🔄️transitional/🚪️io/🦀️.rs:85

```rust
pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🦀️.rs:176

```rust
pub fn register() {
        let _ = register_composer_entries(v_ecma_376::entries());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/🚪️io/🦀️.rs:85

```rust
pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional/🚪️io/🦀️.rs:85

```rust
pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🦀️.rs:176

```rust
pub fn register() {
        let _ = register_composer_entries(v_ecma_376::entries());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/🚪️io/🦀️.rs:89

```rust
pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional/🚪️io/🦀️.rs:84

```rust
pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🦀️.rs:158

```rust
pub fn register() {
        let _ = register_composer_entries(v2_1::entries());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs:52

```rust
pub fn register() {
    subsets::brep::io::register();
    subsets::mesh::io::register();
    subsets::model::io::register();
    subsets::value::io::register();
    subsets::document::io::register();
    subsets::cad::io::register();
    subsets::drawing::io::register();
    subsets::image::io::register();
    subsets::video::io::register();
    subsets::audio::io::register();
    subsets::animation::io::register();
    subsets::presentation::io::register();
    subsets::flow::io::register();
    subsets::text::io::register();
    subsets::table::io::register();
    subsets::graph::io::register();
    subsets::object::io::register();
    subsets::kit::io::register();
    subsets::base::io::register();
}
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs:1186

```rust
pub fn register() {
        let _ = register_composer_entries(v1::entries());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🚪️io/🦀️.rs:120

```rust
pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::animation::schema::semio_animation_artifact_schema_descriptor());
        let _ = store::register_document_codec(store::ArtifactCodec::of::<SemioAnimationSnapshot, crate::artifacts::semio::standards::v1::subsets::animation::schema::mutations::SemioAnimationMutation>(
            crate::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA,
        ));
        let _ = register_subset_validator(validator_entry());
        let _ = register_composer_entries(bridge_entries());
        register_artifact_inferences();
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🚪️io/🦀️.rs:137

```rust
pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::base::schema::semio_artifact_schema_descriptor());
        let _ = store::register_document_codec(store::ArtifactCodec::of::<SemioSnapshot, crate::artifacts::semio::standards::v1::subsets::base::schema::mutations::SemioMutation>(
            crate::artifacts::semio::standards::v1::subsets::base::schema::snapshot::STDIO_SEMIO_DOCUMENT_SCHEMA,
        ));
        let _ = register_subset_validator(validator_entry());
        register_artifact_inferences();
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🚪️io/🦀️.rs:134

```rust
pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::audio::schema::semio_audio_artifact_schema_descriptor());
        let _ = store::register_document_codec(store::ArtifactCodec::of::<SemioAudioSnapshot, crate::artifacts::semio::standards::v1::subsets::audio::schema::mutations::SemioAudioMutation>(
            crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA,
        ));
        let _ = register_subset_validator(validator_entry());
        let _ = register_composer_entries(bridge_entries());
        register_artifact_inferences();
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🚪️io/🦀️.rs:139

```rust
pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::brep::schema::semio_brep_artifact_schema_descriptor());
        let _ = store::register_document_codec(store::ArtifactCodec::of::<SemioBrepSnapshot, crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::SemioBrepMutation>(
            crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::STDIO_SEMIOBREP_DOCUMENT_SCHEMA,
        ));
        let _ = register_subset_validator(validator_entry());
        let _ = register_composer_entries(io_bridge_entries());
        register_artifact_inferences();
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🚪️io/🦀️.rs:138

```rust
pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::cad::schema::semio_cad_artifact_schema_descriptor());
        let _ = store::register_document_codec(store::ArtifactCodec::of::<SemioCadSnapshot, crate::artifacts::semio::standards::v1::subsets::cad::schema::mutations::SemioCadMutation>(
            crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::STDIO_SEMIOCAD_DOCUMENT_SCHEMA,
        ));
        let _ = register_subset_validator(validator_entry());
        let _ = register_composer_entries(io_entries());
        register_artifact_inferences();
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🚪️io/🦀️.rs:176

```rust
pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::document::schema::semio_document_artifact_schema_descriptor());
        let _ = store::register_document_codec(store::ArtifactCodec::of::<SemioDocumentSnapshot, crate::artifacts::semio::standards::v1::subsets::document::schema::mutations::SemioDocumentMutation>(
            crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA,
        ));
        let _ = register_subset_validator(validator_entry());
        let _ = register_composer_entries(io_entries());
        register_artifact_inferences();
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🚪️io/🦀️.rs:145

```rust
pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::drawing::schema::semio_drawing_artifact_schema_descriptor());
        let _ = store::register_document_codec(store::ArtifactCodec::of::<SemioDrawingSnapshot, crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation>(
            crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::STDIO_SEMIODRAWING_DOCUMENT_SCHEMA,
        ));
        let _ = register_subset_validator(validator_entry());
        let _ = register_composer_entries(io_entries());
        register_artifact_inferences();
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🚪️io/🦀️.rs:115

```rust
pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::image::schema::semio_image_artifact_schema_descriptor());
        let _ = store::register_document_codec(store::ArtifactCodec::of::<SemioImageSnapshot, crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::SemioImageMutation>(
            crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA,
        ));
        let _ = register_subset_validator(validator_entry());
        let _ = register_composer_entries(io_entries());
        register_artifact_inferences();
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🚪️io/🦀️.rs:137

```rust
pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::mesh::schema::semio_mesh_artifact_schema_descriptor());
        let _ = store::register_document_codec(store::ArtifactCodec::of::<SemioMeshSnapshot, crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation>(
            crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::STDIO_SEMIOMESH_DOCUMENT_SCHEMA,
        ));
        let _ = register_subset_validator(validator_entry());
        let _ = register_composer_entries(io_bridge_entries());
        register_artifact_inferences();
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🚪️io/🦀️.rs:121

```rust
pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::model::schema::semio_model_artifact_schema_descriptor());
        let _ = store::register_document_codec(store::ArtifactCodec::of::<SemioModelSnapshot, crate::artifacts::semio::standards::v1::subsets::model::schema::mutations::SemioModelMutation>(
            crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::STDIO_SEMIOMODEL_DOCUMENT_SCHEMA,
        ));
        let _ = register_subset_validator(validator_entry());
        let _ = register_composer_entries(io_bridge_entries());
        register_artifact_inferences();
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🚪️io/🦀️.rs:119

```rust
pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::value::schema::semio_value_artifact_schema_descriptor());
        let _ = store::register_document_codec(store::ArtifactCodec::of::<SemioValueSnapshot, crate::artifacts::semio::standards::v1::subsets::value::schema::mutations::SemioValueMutation>(
            crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::STDIO_SEMIOVALUE_DOCUMENT_SCHEMA,
        ));
        let _ = register_subset_validator(validator_entry());
        let _ = register_composer_entries(io_bridge_entries());
        register_artifact_inferences();
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🚪️io/🦀️.rs:122

```rust
pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::presentation::schema::semio_presentation_artifact_schema_descriptor());
        let _ = store::register_document_codec(store::ArtifactCodec::of::<SemioPresentationSnapshot, crate::artifacts::semio::standards::v1::subsets::presentation::schema::mutations::SemioPresentationMutation>(
            crate::artifacts::semio::standards::v1::subsets::presentation::schema::snapshot::STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA,
        ));
        let _ = register_subset_validator(validator_entry());
        let _ = register_composer_entries(io_entries());
        register_artifact_inferences();
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/🚪️io/🦀️.rs:128

```rust
pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::video::schema::semio_video_artifact_schema_descriptor());
        let _ = store::register_document_codec(store::ArtifactCodec::of::<SemioVideoSnapshot, crate::artifacts::semio::standards::v1::subsets::video::schema::mutations::SemioVideoMutation>(
            crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA,
        ));
        let _ = register_subset_validator(validator_entry());
        let _ = register_composer_entries(bridge_entries());
        register_artifact_inferences();
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🚪️io/🦀️.rs:117

```rust
pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::flow::schema::semio_flow_artifact_schema_descriptor());
        let _ = store::register_document_codec(store::ArtifactCodec::of::<SemioFlowSnapshot, crate::artifacts::semio::standards::v1::subsets::flow::schema::mutations::SemioFlowMutation>(
            crate::artifacts::semio::standards::v1::subsets::flow::schema::snapshot::STDIO_SEMIOFLOW_DOCUMENT_SCHEMA,
        ));
        let _ = register_subset_validator(validator_entry());
        let _ = register_composer_entries(io_entries());
        register_artifact_inferences();
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔤️text/🚪️io/🦀️.rs:88

```rust
pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::text::schema::semio_text_artifact_schema_descriptor());
        let _ = store::register_document_codec(store::ArtifactCodec::of::<SemioTextSnapshot, crate::artifacts::semio::standards::v1::subsets::text::schema::mutations::SemioTextMutation>(
            crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::STDIO_SEMIOTEXT_DOCUMENT_SCHEMA,
        ));
        let _ = register_subset_validator(validator_entry());
        let _ = register_composer_entries(io_entries());
        register_artifact_inferences();
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/🚪️io/🦀️.rs:89

```rust
pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::table::schema::semio_table_artifact_schema_descriptor());
        let _ = store::register_document_codec(store::ArtifactCodec::of::<SemioTableSnapshot, crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::SemioTableMutation>(
            crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::STDIO_SEMIOTABLE_DOCUMENT_SCHEMA,
        ));
        let _ = register_subset_validator(validator_entry());
        let _ = register_composer_entries(io_entries());
        register_artifact_inferences();
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🚪️io/🦀️.rs:87

```rust
pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::graph::schema::semio_graph_artifact_schema_descriptor());
        let _ = store::register_document_codec(store::ArtifactCodec::of::<SemioGraphSnapshot, crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation>(
            crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::STDIO_SEMIOGRAPH_DOCUMENT_SCHEMA,
        ));
        let _ = register_subset_validator(validator_entry());
        let _ = register_composer_entries(io_entries());
        register_artifact_inferences();
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/🚪️io/🦀️.rs:111

```rust
pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::object::schema::semio_object_artifact_schema_descriptor());
        let _ = store::register_document_codec(store::ArtifactCodec::of::<SemioObjectSnapshot, crate::artifacts::semio::standards::v1::subsets::object::schema::mutations::SemioObjectMutation>(
            crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA,
        ));
        let _ = register_subset_validator(validator_entry());
        let _ = register_composer_entries(io_entries());
        register_artifact_inferences();
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🚪️io/🦀️.rs:113

```rust
pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::kit::schema::semio_kit_artifact_schema_descriptor());
        let _ = store::register_document_codec(store::ArtifactCodec::of::<SemioKitSnapshot, crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::SemioKitMutation>(
            crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::STDIO_SEMIOKIT_DOCUMENT_SCHEMA,
        ));
        let _ = register_subset_validator(validator_entry());
        let _ = register_composer_entries(io_entries());
        register_artifact_inferences();
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🦀️.rs:94

```rust
pub fn register() {
        let _ = register_composer_entries(std_composer::entries());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🚪️io/🦀️.rs:46

```rust
pub async fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::mp4::standards::isobmff::subsets::any::schema::mp4_artifact_schema_descriptor());
        register_artifact_inferences().await;
        let _ = store::register_document_codec(store::ArtifactCodec::of::<Mp4Snapshot, crate::artifacts::mp4::standards::isobmff::subsets::any::schema::mutations::Mp4Mutation>(
            crate::artifacts::mp4::standards::isobmff::subsets::any::schema::snapshot::STDIO_MP4_DOCUMENT_SCHEMA,
        ));
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🦀️.rs:94

```rust
pub fn register() {
        let _ = register_composer_entries(std_composer::entries());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🚪️io/🦀️.rs:47

```rust
pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::avi::standards::v1_0::subsets::any::schema::avi_artifact_schema_descriptor());
        register_artifact_inferences();
        let _ = store::register_document_codec(store::ArtifactCodec::of::<AviSnapshot, crate::artifacts::avi::standards::v1_0::subsets::any::schema::mutations::AviMutation>(
            crate::artifacts::avi::standards::v1_0::subsets::any::schema::snapshot::STDIO_AVI_DOCUMENT_SCHEMA,
        ));
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🦀️.rs:95

```rust
pub fn register() {
        let _ = register_composer_entries(std_composer::entries());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🚪️io/🦀️.rs:47

```rust
pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::mp3_artifact_schema_descriptor());
        let _ = store::register_document_codec(store::ArtifactCodec::of::<Mp3Snapshot, crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::mutations::Mp3Mutation>(
            crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::snapshot::STDIO_MP3_DOCUMENT_SCHEMA,
        ));
        register_artifact_inferences();
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🦀️.rs:70

```rust
pub fn register() {
        let _ = register_composer_entries(std_composer::entries());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🚪️io/🦀️.rs:47

```rust
pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::wav_artifact_schema_descriptor());
        register_artifact_inferences();
        let _ = store::register_document_codec(store::ArtifactCodec::of::<WavSnapshot, crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::mutations::WavMutation>(
            crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::snapshot::STDIO_WAV_DOCUMENT_SCHEMA,
        ));
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🦀️.rs:49

```rust
pub fn register() {
    crate::artifacts::epw::standards::energyplus::subsets::any::io::register();
    register_pilot_languages();
}
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🦀️.rs:91

```rust
pub fn register() {
        let _ = register_composer_entries(std_composer::entries());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🚪️io/🦀️.rs:50

```rust
pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::epw::standards::energyplus::subsets::any::schema::epw_artifact_schema_descriptor());
        register_artifact_inferences();
        let _ = store::register_document_codec(store::ArtifactCodec::of::<EpwSnapshot, crate::artifacts::epw::standards::energyplus::subsets::any::schema::mutations::EpwMutation>(
            crate::artifacts::epw::standards::energyplus::subsets::any::schema::snapshot::STDIO_EPW_DOCUMENT_SCHEMA,
        ));
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🦀️.rs:49

```rust
pub fn register() {
    crate::artifacts::tsv::standards::iana::subsets::any::io::register();
    register_pilot_languages();
}
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🦀️.rs:91

```rust
pub fn register() {
        let _ = register_composer_entries(std_composer::entries());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🚪️io/🦀️.rs:47

```rust
pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::tsv::standards::iana::subsets::any::schema::tsv_artifact_schema_descriptor());
        register_artifact_inferences();
        let _ = store::register_document_codec(store::ArtifactCodec::of::<TsvSnapshot, crate::artifacts::tsv::standards::iana::subsets::any::schema::mutations::TsvMutation>(
            crate::artifacts::tsv::standards::iana::subsets::any::schema::snapshot::STDIO_TSV_DOCUMENT_SCHEMA,
        ));
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🦀️.rs:70

```rust
pub fn register() {
        let _ = register_composer_entries(std_composer::entries());
    }
```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🚪️io/🦀️.rs:47

```rust
pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::html::standards::v5::subsets::any::schema::html_artifact_schema_descriptor());
        register_artifact_inferences();
        let _ = store::register_document_codec(store::ArtifactCodec::of::<HtmlSnapshot, crate::artifacts::html::standards::v5::subsets::any::schema::mutations::HtmlMutation>(
            crate::artifacts::html::standards::v5::subsets::any::schema::snapshot::STDIO_HTML_DOCUMENT_SCHEMA,
        ));
    }
```

## Pass 351 — Caller Inventory

203 no-argument registration definitions, 36 executable calls. Comments and strings are excluded from executable-call scanning; raw source is retained in the inventory.




## Pass 353 — Synchronous Publication Boundary

The underlying registration implementations perform finite validation and publication under the existing synchronous assembly barrier and registry locks. Their only awaits forward to each other; document registration already has duplicate synchronous _now implementations. The planned fix makes these atomic publication primitives synchronous, preserves their typed Result/error and all-or-nothing validation, removes duplicate _now entry points, and updates existing awaits/ready wrappers. This executes Stdio registration directly without introducing a polling helper or async runtime.

{
  "register_composer_entry_refs:awaited": 1,
  "register_composer_entry_refs_in_assembly:awaited": 1,
  "register_subset_validators:awaited": 1,
  "register_subset_validators_in_assembly:awaited": 1,
  "register_composer_entries:awaited": 3,
  "register_document_codec:awaited": 7,
  "register_composer_entries:direct": 61,
  "register_document_codec_now:direct": 2,
  "register_document_codec:direct": 34,
  "register_document_codecs_now:direct": 2,
  "register_dialect_migrations:awaited": 2,
  "register_dialect_migrations_in_assembly:awaited": 1,
  "register_subset_validator:direct": 51,
  "register_dialect_migration:direct": 1,
  "register_dialect_migration:awaited": 1
}


## Pass 355 — Atomic Publication Primitives

{
  "definitions": 11,
  "awaits": 18,
  "wrappers": 4,
  "renames": 1,
  "stdioResults": 143,
  "files": 107
}

Composer/subset/document/migration registration now executes synchronously under the existing assembly transaction and registry locks. Validation, insertion order, ownership and typed errors are unchanged. Redundant document _now wrappers were removed and all Rust callers were inventoried. Four macro ready wrappers were removed. Stdio static setup now checks all 143 registration results and reports unavailability or conflicting declarations. No executor, compatibility layer or lint allowance was added.

Targeted registration/codec laws and fresh native/WASI checks remain required.

- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs
- 🧰️framework/🔨️modules/🚪️io/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🧬️migrations/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧱️baseline/🚪️io/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🔀️migrate/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔬️tiny/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔰️basic/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✅️valid/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🤝️cv20/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/2️⃣cc2/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧮️sav/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/6️⃣cc6/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🏢️cobie/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/3️⃣cc3/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/4️⃣cc4/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/5️⃣cc5/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/1️⃣cc1/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🔄️transitional/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔤️text/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/📏️strict/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🦀️.rs


## Pass 362 — Exact Baseline Registration Law

Pass348 built successfully but used a nonexistent test path (missing component). The original executable was SHA-256 verified and its exact registered test name was then run directly via Bun/Nx. Exit code 101.

```text

running 1 test
test artifacts::binary::component::io_registry::tests::register_then_resolve_through_the_typed_registry_finds_this_composer ... FAILED

failures:

failures:
    artifacts::binary::component::io_registry::tests::register_then_resolve_through_the_typed_registry_finds_this_composer

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 6450 filtered out; finished in 0.37s


thread 'artifacts::binary::component::io_registry::tests::register_then_resolve_through_the_typed_registry_finds_this_composer' (7504997) panicked at ✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/././../../🗿️artifacts/💾️binary/🦀️.rs:107:48:
resolve: IoResolveError { message: "no composer registered for s.stdio.binary/raw/* Import s.stdio.binary/raw/*", candidates: [], unavailable: None }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```


## Pass 367 — Codec Fixture Drift

The post-change framework composition and conflicting-registration laws passed in pass358. The store codec round-trip law stopped before registration because its fixture asserted extension demo.doc, while the declared schema explicitly uses id demo.doc and extension demo. Updated the literal extension assertion to demo, and detached the test’s fresh envelope owners immediately after serialization. No codec runtime implementation changed in this pass. The store laws must be rerun; the first failure prevented the later two registration laws from running in pass358.
