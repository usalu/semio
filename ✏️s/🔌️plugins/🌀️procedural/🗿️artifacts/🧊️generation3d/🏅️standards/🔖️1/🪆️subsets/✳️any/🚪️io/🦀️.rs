//! 🚪️ IO s.generation3d (1/✳️any) — registration now flows through 🎹️composer::register
//! (called once from ⚙️engine::register), not per-leaf register().
#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
pub fn import_stdio_kinds() -> &'static [&'static str] {
    &["stdio.dwg", "stdio.gltf", "stdio.json", "stdio.obj", "stdio.ply", "stdio.stl", "stdio.txt"]
}
// 🖼️ No "stdio.png"/"stdio.json" here (export-only list): generation2d owns those EXPORT claims, see
// `🚪️IoRegistry` region below. Import is unaffected — both stay in `import_stdio_kinds` above.
pub fn export_stdio_kinds() -> &'static [&'static str] {
    &["stdio.dwg", "stdio.gltf", "stdio.las", "stdio.obj", "stdio.ply", "stdio.stl", "stdio.txt"]
}
//#region 🔺️MeshBridge

//#endregion 🔺️MeshBridge

//#region 📄️DocumentIo
/// 📄️ The USER-facing half of this artifact's IO — the roster the editor's and the viewer's
/// `exportDocument`/`importDocument` commands pick from, the filename and MIME every download
/// carries, and the two byte-level entry points those commands call.
///
/// 🐛️ Until ticket 26/09/09/PROCEDURAL-3D-END-TO-END's io-surface lane the nine leaves below
/// `📤️export`/`📥️import` were round-trip tested and **unreachable**: no command, menu item, button
/// or keybinding in `✏️editor` or `👁️viewer` named them, so a user could not import or export
/// anything (`📓️audit-user-journey-gaps-2026-09-13.md` §6, P0 #1). This module is the composition
/// point that closed that gap; it re-implements no leaf and spells no file grammar.
///
/// 🏷️ A row's `id` is what the user picks (the `format` `ActionArgDef::select` option); its
/// `kind_id` is the owning `s.stdio.<format>` artifact's OWN representation id, and the extension,
/// MIME and binary-ness come from THAT artifact's `formats()` — this module never restates a
/// format's file facts, so a format that renames its extension cannot drift from the download.
///
/// @see ../../../../../../🗄️stdio/🗿️artifacts — every row's owning artifact and its `formats()`.
/// @see ../✏️editor/🎮️commands/📤️export-document, ../✏️editor/🎮️commands/📥️import-document.
pub mod document_io {
    use semio_s_artifact_procedural_3d_mesh_bridge::io_error;
    use semio_s_artifact_procedural_3d_io_export::serializers::artifacts as export_leaves;
    use semio_s_artifact_procedural_3d_io_import::deserializers::artifacts as import_leaves;
    use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
    use semio_framework_plugin::io::FormatDescriptor;
    use semio_s_artifact_stdio_semio_mesh::schema::snapshot::SemioMeshSnapshot;

    /// 🏷️ One row of the user-facing format roster: the id the user picks, the two labels the picker
    /// shows, and the owning stdio artifact's representation this artifact's leaf writes or reads.
    ///
    /// 🗣️ English first, German second, with NO default: `LocalizedLabel::native` takes both and a
    /// locale resolves one, so a row cannot exist in one language only.
    #[derive(Debug)]
    pub struct Generation3dFormatRow {
        pub id: &'static str,
        pub label_en: &'static str,
        pub label_de: &'static str,
        pub kind_id: &'static str,
        pub descriptors: fn() -> Result<Vec<FormatDescriptor>, semio_framework_plugin::ArtifactDefinitionError>,
    }

    /// 📤️ Every format `exportDocument` offers, in menu order. Exactly [`super::export_stdio_kinds`]
    /// — `json`/`png` are absent because generation2d owns those EXPORT claims (see `🚪️IoRegistry`),
    /// and offering a user a format this artifact does not claim would be a lie in the picker.
    pub const EXPORT_FORMATS: [Generation3dFormatRow; 7] = [
        Generation3dFormatRow { id: "stl", label_en: "STL Mesh", label_de: "STL-Netz", kind_id: "s.stdio.stl.standard.ascii.representation.document", descriptors: semio_s_artifact_stdio_stl::formats },
        Generation3dFormatRow { id: "obj", label_en: "OBJ Mesh", label_de: "OBJ-Netz", kind_id: "s.stdio.obj.standard.3-0.representation.document", descriptors: semio_s_artifact_stdio_obj::formats },
        Generation3dFormatRow { id: "ply", label_en: "PLY Mesh", label_de: "PLY-Netz", kind_id: "s.stdio.ply.standard.1-0.representation.document", descriptors: semio_s_artifact_stdio_ply::formats },
        Generation3dFormatRow { id: "gltf", label_en: "glTF Mesh", label_de: "glTF-Netz", kind_id: "s.stdio.gltf.standard.2-0.representation.document", descriptors: semio_s_artifact_stdio_gltf::formats },
        Generation3dFormatRow { id: "las", label_en: "LAS Point Cloud", label_de: "LAS-Punktwolke", kind_id: "s.stdio.las.standard.1-0.representation.document", descriptors: semio_s_artifact_stdio_las::formats },
        Generation3dFormatRow { id: "dwg", label_en: "DWG Drawing", label_de: "DWG-Zeichnung", kind_id: "s.stdio.dwg.standard.ac1018.representation.document", descriptors: semio_s_artifact_stdio_dwg::formats },
        Generation3dFormatRow { id: "txt", label_en: "Semio Text (whole artifact)", label_de: "Semio-Text (ganzes Artefakt)", kind_id: "s.stdio.txt.standard.utf-8.representation.document", descriptors: semio_s_artifact_stdio_txt::formats },
    ];

    /// 📥️ Every format `importDocument` accepts, in picker order — the SEVEN of
    /// [`super::import_stdio_kinds`]'s nine whose deserializer really reconstructs a document.
    /// `las` and `png` are deliberately absent: their import leaves are honest `Err`s (LAS is
    /// export-only, PNG carries no recoverable graph), and an `accept` filter that offered them
    /// would put files in the picker that can only fail. `no_import_formats_can_be_deserialized`
    /// pins that, so a leaf that later gains a real decoder cannot stay silently out of the picker.
    pub const IMPORT_FORMATS: [Generation3dFormatRow; 7] = [
        Generation3dFormatRow { id: "stl", label_en: "STL Mesh", label_de: "STL-Netz", kind_id: "s.stdio.stl.standard.ascii.representation.document", descriptors: semio_s_artifact_stdio_stl::formats },
        Generation3dFormatRow { id: "obj", label_en: "OBJ Mesh", label_de: "OBJ-Netz", kind_id: "s.stdio.obj.standard.3-0.representation.document", descriptors: semio_s_artifact_stdio_obj::formats },
        Generation3dFormatRow { id: "ply", label_en: "PLY Mesh", label_de: "PLY-Netz", kind_id: "s.stdio.ply.standard.1-0.representation.document", descriptors: semio_s_artifact_stdio_ply::formats },
        Generation3dFormatRow { id: "gltf", label_en: "glTF Mesh", label_de: "glTF-Netz", kind_id: "s.stdio.gltf.standard.2-0.representation.document", descriptors: semio_s_artifact_stdio_gltf::formats },
        Generation3dFormatRow { id: "dwg", label_en: "DWG Drawing", label_de: "DWG-Zeichnung", kind_id: "s.stdio.dwg.standard.ac1018.representation.document", descriptors: semio_s_artifact_stdio_dwg::formats },
        Generation3dFormatRow { id: "json", label_en: "Generation JSON", label_de: "Generation-JSON", kind_id: "s.stdio.json.standard.rfc8259.representation.document", descriptors: semio_s_artifact_stdio_json::formats },
        Generation3dFormatRow { id: "txt", label_en: "Semio Text (whole artifact)", label_de: "Semio-Text (ganzes Artefakt)", kind_id: "s.stdio.txt.standard.utf-8.representation.document", descriptors: semio_s_artifact_stdio_txt::formats },
    ];

    /// 🏷️ The owning artifact's own descriptor for one row — the single source of extension, MIME
    /// and binary-ness.
    pub fn descriptor_of(row: &Generation3dFormatRow) -> Result<FormatDescriptor, semio_framework_diagnostic::TextError> {
        let descriptors = (row.descriptors)().map_err(|error| io_error(format!("generation3d io: format `{}` has no readable descriptor set ({error})", row.id)))?;
        descriptors
            .into_iter()
            .find(|descriptor| descriptor.kind_id == row.kind_id)
            .ok_or_else(|| io_error(format!("generation3d io: format `{}` declares representation `{}`, which its owning artifact does not publish", row.id, row.kind_id)))
    }

    fn row_of(rows: &'static [Generation3dFormatRow], id: &str) -> Result<&'static Generation3dFormatRow, semio_framework_diagnostic::TextError> {
        rows.iter()
            .find(|row| row.id == id)
            .ok_or_else(|| io_error(format!("generation3d io: `{id}` is not one of this artifact's formats ({})", rows.iter().map(|row| row.id).collect::<Vec<_>>().join(", "))))
    }

    /// 🗣️ One `ActionArgOption` per [`EXPORT_FORMATS`] row, in table order — the editor's picker and
    /// the viewer's are THE SAME list, built once here rather than spelled twice, so a format this
    /// artifact stops claiming disappears from both surfaces at once.
    pub fn export_format_options() -> Vec<semio_framework_plugin::ActionArgOption> {
        EXPORT_FORMATS.iter().map(|row| semio_framework_plugin::ActionArgOption::new(row.id, semio_framework_ui_locale::LocalizedLabel::native(row.label_en, row.label_de))).collect()
    }

    /// 🗂️ The file picker's `accept` filter — every importable extension, comma-joined, exactly the
    /// shape `Effect::RequestFileOpen` carries.
    pub fn import_accept_filter() -> Result<String, semio_framework_diagnostic::TextError> {
        let mut extensions = Vec::new();
        for row in IMPORT_FORMATS.iter() {
            extensions.extend(descriptor_of(row)?.extensions);
        }
        Ok(extensions.join(","))
    }

    /// 📤️ One finished export, ready to become an `Effect::DownloadMediaExport`.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Generation3dDocumentExport {
        pub filename: String,
        pub data: String,
        pub mime_type: String,
        pub encoding: Option<String>,
    }
    semio_framework_value::artifact_retire_struct!(Generation3dDocumentExport { filename, data, mime_type, encoding });

    /// 🗣️ Reports a real format limitation through the app's localized inspector.
    pub struct Generation3dIoDiagnostic {
        pub code: &'static str,
        pub label: semio_framework_ui_locale::LocalizedLabel,
    }

    /// 📤️ Describes authored surface data the selected interchange codec cannot retain.
    pub fn format_diagnostics(meshes: &[semio_framework_plugin::MeshData], format: &str) -> Vec<Generation3dIoDiagnostic> {
        use semio_framework_plugin::MeshAttributeSemantic;
        if format == "txt" { return Vec::new(); }
        let supported: &[&str] = match format {
            "gltf" => &["normal", "uv", "color", "material", "texture", "custom"],
            "obj" => &["normal", "uv"],
            "ply" => &["normal", "uv", "color"],
            "las" => &["color"],
            _ => &[],
        };
        let present = [
            ("normal", "Authored normals", "Erstellte Normalen", meshes.iter().any(|mesh| !mesh.normals.is_empty() || mesh.attributes.values().any(|attribute| attribute.semantic == MeshAttributeSemantic::Normal))),
            ("uv", "UV coordinates", "UV-Koordinaten", meshes.iter().any(|mesh| !mesh.uvs.is_empty() || mesh.attributes.values().any(|attribute| attribute.semantic == MeshAttributeSemantic::Uv))),
            ("color", "Vertex colors", "Knotenfarben", meshes.iter().any(|mesh| !mesh.colors.is_empty() || mesh.attributes.values().any(|attribute| attribute.semantic == MeshAttributeSemantic::Color))),
            ("material", "Material assignments and properties", "Materialzuweisungen und Eigenschaften", meshes.iter().any(|mesh| !mesh.materials.is_empty() || mesh.attributes.values().any(|attribute| attribute.semantic == MeshAttributeSemantic::Material))),
            ("texture", "Texture images and bindings", "Texturbilder und Verknüpfungen", meshes.iter().any(|mesh| !mesh.textures.is_empty())),
            ("custom", "Custom attributes", "Benutzerdefinierte Attribute", meshes.iter().any(|mesh| mesh.attributes.values().any(|attribute| attribute.semantic == MeshAttributeSemantic::Custom))),
        ];
        let mut diagnostics: Vec<_> = present.into_iter().filter(|(code, _, _, present)| *present && !supported.contains(code)).map(|(code, en, de, _)| Generation3dIoDiagnostic {
            code,
            label: semio_framework_ui_locale::LocalizedLabel::native(
                &format!("{en} will not be retained by {}. The Semio Text document retains the editable data.", format.to_uppercase()),
                &format!("{de} bleiben in {} nicht erhalten. Das Semio-Text-Dokument erhält die bearbeitbaren Daten.", format.to_uppercase()),
            ),
        }).collect();
        let mut add = |code: &'static str, en: String, de: String| diagnostics.push(Generation3dIoDiagnostic { code, label: semio_framework_ui_locale::LocalizedLabel::native(&en, &de) });
        for (name, semantic, code) in [("normal",MeshAttributeSemantic::Normal,"multiple-normal"),("uv",MeshAttributeSemantic::Uv,"multiple-uv"),("color",MeshAttributeSemantic::Color,"multiple-color"),("material",MeshAttributeSemantic::Material,"multiple-material")] {
            if format != "gltf" && supported.contains(&name) && meshes.iter().any(|mesh| mesh.attributes.values().filter(|attribute| attribute.semantic == semantic).count() > 1) { add(code, format!("{} retains one {name} channel. Additional sets remain in the Semio Text document.",format.to_uppercase()), format!("{} erhält einen {name}-Kanal. Zusätzliche Sätze bleiben im Semio-Text-Dokument.",format.to_uppercase())); }
        }
        if matches!(format,"stl"|"ply"|"las"|"dwg") && meshes.len() > 1 { add("part-grouping","This format combines surface parts and their identities. The editable graph retains each part.".into(),"Dieses Format verbindet Oberflächenteile und ihre Kennungen. Der bearbeitbare Graph erhält jedes Teil.".into()); }
        if format == "las" && meshes.iter().any(|mesh| !mesh.indices.is_empty()) { add("topology","LAS exports points and cannot retain face connectivity.".into(),"LAS exportiert Punkte und kann die Flächenverbindungen nicht erhalten.".into()); }
        let levels = match format { "ply" => 255.0, "las" => 65535.0, _ => 0.0 };
        let mut color_precision = false;
        let mut color_alpha = false;
        for mesh in meshes {
            let mut check = |values: &[f64]| {
                color_precision |= levels > 0.0 && values.iter().take(if format == "las" {3}else{4}).any(|value| (value-(value*levels).round()/levels).abs() > 1e-8);
                color_alpha |= values.len() == 4 && values[3] != 1.0;
            };
            if let Some(attribute) = mesh.attributes.values().find(|attribute| attribute.semantic == MeshAttributeSemantic::Color) {
                for value in &attribute.values { if let Some(values) = value.as_array() { check(&values.iter().filter_map(semio_framework_value::DslValue::as_f64).collect::<Vec<_>>()); } }
            } else {
                let width = if mesh.colors.len() == mesh.positions.len()/3*4 {4}else{3};
                for color in mesh.colors.chunks(width) { check(&color.iter().map(|value| *value as f64).collect::<Vec<_>>()); }
            }
        }
        if color_precision { let bits = if format == "ply" {8}else{16}; add("color-precision",format!("{} rounds color channels to {bits} bits.",format.to_uppercase()),format!("{} rundet Farbkanäle auf {bits} Bit.",format.to_uppercase())); }
        if format == "las" && color_alpha { add("color-alpha","LAS cannot retain color transparency.".into(),"LAS kann die Farbtransparenz nicht erhalten.".into()); }
        if format == "las" && meshes.iter().any(|mesh| mesh.positions.iter().any(|value| {let value=*value as f64;(value-(value/0.0001).round()*0.0001).abs()>1e-8})) { add("coordinate-precision","LAS rounds positions to the codec's 0.0001-unit grid.".into(),"LAS rundet Positionen auf das Raster des Codecs mit 0.0001 Einheiten.".into()); }
        diagnostics
    }

    /// 📤️ The MESH half of every geometry export, isolated exactly as each leaf's own
    /// `serialize_mesh_bytes` is: it needs no flow evaluator, so the format table, the filename and
    /// the wire envelope are all provable natively against the committed unit-cube host_snapshot.
    pub fn export_mesh_bytes(mesh: &SemioMeshSnapshot, format: &str) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
        match format {
            "stl" => export_leaves::stl::v_ascii::any::serialize_mesh_bytes(mesh),
            "obj" => export_leaves::obj::v3_0::any::serialize_mesh_bytes(mesh),
            "ply" => export_leaves::ply::v1_0::any::serialize_mesh_bytes(mesh),
            "gltf" => export_leaves::gltf::v2_0::any::serialize_mesh_bytes(mesh),
            "las" => export_leaves::las::v1_0::any::serialize_mesh_bytes(mesh),
            "dwg" => export_leaves::dwg::v_ac1018::any::serialize_mesh_bytes(mesh),
            "txt" => Err(io_error("generation3d export: `txt` is the whole document's own text, not a mesh — it has no mesh-only half")),
            other => Err(io_error(format!("generation3d export: `{other}` is not one of this artifact's formats"))),
        }
    }

    /// 📄️ Encodes the reversible graph document without geometry evaluation.
    pub fn export_document_bytes(snapshot: &Generation3dSnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
        export_leaves::txt::v_utf_8::any::serialize_bytes(snapshot)
    }

    /// 🧵️ Retains the physical output until its exact textual or binary envelope can publish.
    pub struct Generation3dDocumentEnvelope {
        row: &'static Generation3dFormatRow, bytes: Option<Vec<u8>>, position: usize, binary: Option<bool>,
        filename: Option<String>, mime_type: Option<String>, data: Option<String>, complete: bool,
    }

    impl Generation3dDocumentEnvelope {
        /// 🌱️ Takes existing physical output without copying or interpreting its bytes.
        pub fn new(row: &'static Generation3dFormatRow, bytes: Vec<u8>) -> Self { Self { row, bytes: Some(bytes), position: 0, binary: None, filename: None, mime_type: None, data: None, complete: false } }
        /// 📍️ Returns the processed physical byte count.
        pub fn position(&self) -> usize { self.position }
        /// ⏱️ Encodes one base64 triple or validates one UTF-8 scalar per supplied unit.
        pub fn step(&mut self, maximum_units: usize, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<Option<Generation3dDocumentExport>, semio_framework_diagnostic::TextError> {
            let controlled = |error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1));
            for _ in 0..maximum_units {
                control.checkpoint().map_err(controlled)?;
                if self.complete { return Ok(None); }
                if self.binary.is_none() {
                    let descriptor = descriptor_of(self.row)?;
                    let extension = descriptor.extensions.first().ok_or_else(|| io_error("generation3d export format claims no extension"))?;
                    let mime = descriptor.mimes.first().ok_or_else(|| io_error("generation3d export format claims no MIME type"))?;
                    self.filename = Some(control.copy_text(&format!("generation3d{extension}")).map_err(controlled)?);
                    self.mime_type = Some(control.copy_text(mime).map_err(controlled)?);
                    if descriptor.is_binary {
                        let length = self.bytes.as_ref().unwrap().len().div_ceil(3).checked_mul(4).ok_or_else(|| io_error("generation3d export envelope size overflow"))?;
                        control.charge(length).map_err(controlled)?;
                        let mut output = String::new(); output.try_reserve_exact(length).map_err(|_| io_error("generation3d export envelope allocation failed"))?;
                        self.data = Some(output);
                    }
                    self.binary = Some(descriptor.is_binary);
                } else {
                    let bytes = self.bytes.as_ref().expect("physical source is retained");
                    if self.position == bytes.len() {
                        control.checkpoint().map_err(controlled)?;
                        if self.binary == Some(false) { self.data = Some(unsafe { String::from_utf8_unchecked(self.bytes.take().unwrap()) }); }
                        let encoding = if self.binary == Some(true) { Some(control.copy_text("base64").map_err(controlled)?) } else { None };
                        self.complete = true;
                        return Ok(Some(Generation3dDocumentExport { filename: self.filename.take().unwrap(), mime_type: self.mime_type.take().unwrap(), data: self.data.take().unwrap(), encoding }));
                    }
                    if self.binary == Some(true) {
                        let end = (self.position + 3).min(bytes.len());
                        control.charge(4).map_err(controlled)?;
                        self.data.as_mut().unwrap().push_str(&semio_s_artifact_procedural_3d_mesh_bridge::base64_encode(&bytes[self.position..end]));
                        control.advance(end - self.position).map_err(controlled)?; self.position = end;
                    } else {
                        let first = bytes[self.position];
                        let width = if first < 0x80 { 1 } else if first < 0xe0 { 2 } else if first < 0xf0 { 3 } else { 4 };
                        let end = (self.position + width).min(bytes.len());
                        std::str::from_utf8(&bytes[self.position..end]).map_err(|error| io_error(format!("generation3d textual export has invalid UTF-8 ({error})")))?;
                        control.advance(end - self.position).map_err(controlled)?; self.position = end;
                    }
                }
                control.step().map_err(controlled)?;
            }
            Ok(None)
        }
    }

    impl semio_framework_value::retirement::RetireOwned for Generation3dDocumentEnvelope {
        fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> { use semio_framework_value::retirement::{sequence,deferred};sequence(vec![deferred(self.bytes),deferred(self.filename),deferred(self.mime_type),deferred(self.data)]) }
        fn retirement_birth_bytes(&self) -> Option<usize> { use semio_framework_value::retirement::{sequence_birth_bytes,deferred_birth_bytes_for};sequence_birth_bytes(&[deferred_birth_bytes_for(&self.bytes),deferred_birth_bytes_for(&self.filename),deferred_birth_bytes_for(&self.mime_type),deferred_birth_bytes_for(&self.data)]) }
        fn controlled_retirement_supported() -> bool { true }
    }

    /// 📦️ Wraps raw export bytes in the wire envelope `Effect::DownloadMediaExport` expects: the
    /// owning artifact's extension and MIME, and base64 exactly when that artifact calls itself
    /// binary. A row that claims to be text and hands back bytes that are not UTF-8 is a typed
    /// error, never a lossy `from_utf8_lossy` download.
    pub fn document_export_envelope(row: &Generation3dFormatRow, bytes: Vec<u8>) -> Result<Generation3dDocumentExport, semio_framework_diagnostic::TextError> {
        let descriptor = descriptor_of(row)?;
        let extension = descriptor.extensions.first().cloned().ok_or_else(|| io_error(format!("generation3d export: format `{}` claims no extension", row.id)))?;
        let mime_type = descriptor.mimes.first().cloned().ok_or_else(|| io_error(format!("generation3d export: format `{}` claims no MIME type", row.id)))?;
        let filename = format!("generation3d{extension}");
        if descriptor.is_binary {
            return Ok(Generation3dDocumentExport { filename, data: semio_s_artifact_procedural_3d_mesh_bridge::base64_encode(&bytes), mime_type, encoding: Some("base64".into()) });
        }
        let data = String::from_utf8(bytes).map_err(|error| io_error(format!("generation3d export: format `{}` is declared textual but its bytes are not UTF-8 ({error})", row.id)))?;
        Ok(Generation3dDocumentExport { filename, data, mime_type, encoding: None })
    }

    /// 📤️ Encodes caller-prepared geometry in its declared format.
    pub fn export_geometry(mesh: &SemioMeshSnapshot, format: &str) -> Result<Generation3dDocumentExport, semio_framework_diagnostic::TextError> {
        let row = row_of(&EXPORT_FORMATS, format)?;
        document_export_envelope(row, export_mesh_bytes(mesh, row.id)?)
    }

    /// 🎨️ Keeps rich prepared surfaces until their declared serializer has emitted the interchange.
    pub fn export_prepared_geometry(meshes: &[semio_framework_plugin::MeshData], format: &str) -> Result<Generation3dDocumentExport, semio_framework_diagnostic::TextError> {
        let row = row_of(&EXPORT_FORMATS, format)?;
        let bytes = if format == "gltf" { export_leaves::gltf::v2_0::any::serialize_prepared_meshes_bytes(meshes)? } else { export_mesh_bytes(&semio_s_artifact_procedural_3d_mesh_bridge::semio_mesh_from_meshes(meshes)?, format)? };
        document_export_envelope(row, bytes)
    }

    /// 📄️ Exports the reversible graph document as its own text.
    pub fn export_document(snapshot: &Generation3dSnapshot) -> Result<Generation3dDocumentExport, semio_framework_diagnostic::TextError> {
        let row = row_of(&EXPORT_FORMATS, "txt")?;
        document_export_envelope(row, export_document_bytes(snapshot)?)
    }

    /// 📦️ Decodes what `Effect::RequestFileOpen { read_as: Some("dataUrl") }` hands back. A shell
    /// that answers with the raw text instead (the `read_as: None` case, and every test harness) is
    /// read as its own UTF-8 bytes.
    pub fn import_payload_bytes(payload: &str) -> Result<std::borrow::Cow<'_, [u8]>, semio_framework_diagnostic::TextError> {
        if let Some((header, encoded)) = payload.split_once(',') {
            if header.starts_with("data:") {
                return if header.ends_with(";base64") { semio_s_artifact_procedural_3d_mesh_bridge::base64_decode(encoded).map(std::borrow::Cow::Owned) } else { Ok(std::borrow::Cow::Borrowed(encoded.as_bytes())) };
            }
        }
        Ok(std::borrow::Cow::Borrowed(payload.as_bytes()))
    }

    /// 🏷️ The roster row a picked file's name resolves to, by the owning artifact's OWN extension
    /// claim — never a hand-written extension table.
    pub fn import_row_for_name(name: &str) -> Result<&'static Generation3dFormatRow, semio_framework_diagnostic::TextError> {
        let lowered = name.to_ascii_lowercase();
        for row in IMPORT_FORMATS.iter() {
            for extension in descriptor_of(row)?.extensions {
                if lowered.ends_with(&extension.to_ascii_lowercase()) {
                    return Ok(row);
                }
            }
        }
        Err(io_error(format!("generation3d import: `{name}` is not one of this artifact's importable formats ({})", IMPORT_FORMATS.iter().map(|row| row.id).collect::<Vec<_>>().join(", "))))
    }

    /// 📥️ The bytes of one picked file as a real document, through that format's own leaf.
    pub fn import_document_bytes(format: &str, bytes: &[u8]) -> Result<Generation3dSnapshot, semio_framework_diagnostic::TextError> {
        match format {
            "stl" => import_leaves::stl::v_ascii::any::deserialize_bytes(bytes),
            "obj" => import_leaves::obj::v3_0::any::deserialize_bytes(bytes),
            "ply" => import_leaves::ply::v1_0::any::deserialize_bytes(bytes),
            "gltf" => import_leaves::gltf::v2_0::any::deserialize_bytes(bytes),
            "dwg" => import_leaves::dwg::v_ac1018::any::deserialize_bytes(bytes),
            "json" => import_leaves::json::v_rfc8259::any::deserialize_bytes(bytes),
            "txt" => import_leaves::txt::v_utf_8::any::deserialize_bytes(bytes),
            other => Err(io_error(format!("generation3d import: `{other}` is not one of this artifact's importable formats"))),
        }
    }

    /// 📥️ The whole import, from a picked file to a document.
    pub fn import_document(name: &str, payload: &str) -> Result<Generation3dSnapshot, semio_framework_diagnostic::TextError> {
        let row = import_row_for_name(name)?;
        import_document_bytes(row.id, &import_payload_bytes(payload)?)
    }
}
pub use document_io::{export_document, import_document as import_picked_document, Generation3dDocumentExport};
//#endregion 📄️DocumentIo

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::Generation3dAnalyzer;
    use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
    use {semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.procedural.generation3d", standard: StandardId("1"), subset: SubsetId("*") };
    const DEP_DWG: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };
    const DEP_GLTF: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId("*") };
    const DEP_JSON: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    const DEP_OBJ: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId("*") };
    const DEP_PLY: Dialect = Dialect { artifact_kind: "s.stdio.ply", standard: StandardId("1.0"), subset: SubsetId("*") };
    const DEP_STL: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    pub struct Generation3dComposerComposition;

    impl ArtifactComposition for Generation3dComposerComposition {
        type Snapshot = Generation3dSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_DWG, DEP_GLTF, DEP_JSON, DEP_OBJ, DEP_PLY, DEP_STL, DEP_TXT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            for source in sources {
                if source.dialect == DIALECT {
                    let native = match &source.payload {
                        AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                        AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                    };
                    let analysis = Generation3dAnalyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
                if source.dialect == DEP_DWG {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = semio_s_artifact_procedural_3d_io_import::deserializers::artifacts::dwg::v_ac1018::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::io::Confidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_GLTF {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = semio_s_artifact_procedural_3d_io_import::deserializers::artifacts::gltf::v2_0::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::io::Confidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_JSON {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = semio_s_artifact_procedural_3d_io_import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::io::Confidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_OBJ {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = semio_s_artifact_procedural_3d_io_import::deserializers::artifacts::obj::v3_0::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::io::Confidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_PLY {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = semio_s_artifact_procedural_3d_io_import::deserializers::artifacts::ply::v1_0::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::io::Confidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_STL {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = semio_s_artifact_procedural_3d_io_import::deserializers::artifacts::stl::v_ascii::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::io::Confidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_TXT {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = semio_s_artifact_procedural_3d_io_import::deserializers::artifacts::txt::v_utf_8::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::io::Confidence::Medium, diagnostics: Vec::new() });
                    }
                }
            }
            Err(ComposeError { message: "Generation3dComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️IoRegistry
/// 🚪️ Rehomed from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// the composer/export-entry registry lives with the rest of `🚪️io`, not behind an engine facade.
pub mod io_registry {
    use crate::Generation3dBuilder as Generation3dAnyBuilder;
    use crate::Generation3dComposer as Generation3dAnyComposer;
    use {semio_framework_plugin::composer_entry_of,semio_framework_plugin::ArtifactBuilder,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposedArtifact,semio_framework_plugin::io::ComposerEntry,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::ErasedComposeSource,semio_framework_plugin::io::IoPayload,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    //#region 🔖️ExportEntries
    /// 🗄️ Graph compositions preserve document text; prepared geometry uses the owning mesh codecs.
    const GENERATION3D_DIALECT: Dialect = Dialect { artifact_kind: "s.procedural.generation3d", standard: StandardId("1"), subset: SubsetId("*") };
    const GENERATION3D_JSON_BRIDGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    async fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot, ComposeError> {
        if let Some(source) = sources.iter().find(|s| s.dialect == GENERATION3D_DIALECT) {
            let builder = match &source.payload {
                IoPayload::Text(t) => Generation3dAnyBuilder::from_text(t).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
                IoPayload::Binary(b) => Generation3dAnyBuilder::from_binary(b).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
            };
            return builder.build().map_err(|diagnostics| ComposeError { message: "Generation3dComposer export: build() failed".into(), diagnostics });
        }
        if let Some(source) = sources.iter().find(|s| s.dialect == GENERATION3D_JSON_BRIDGE_DIALECT) {
            // 🌉 The OS dispatch layer (export_os_app_instance_media_kind) deals in already-
            // deserialized `serde_json::Value`, not this artifact's own wire text/binary -- json
            // is the universal bridge dialect every domain artifact already imports from.
            let bytes: Vec<u8> = match &source.payload {
                IoPayload::Text(t) => t.as_bytes().to_vec(),
                IoPayload::Binary(b) => b.clone(),
            };
            return semio_s_artifact_procedural_3d_io_import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() });
        }
        Err(ComposeError { message: "Generation3dComposer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() })
    }

    const EXPORT_TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };
    fn compose_export_txt(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources).await?;
            let encoded = semio_s_artifact_procedural_3d_io_export::serializers::artifacts::txt::v_utf_8::any::serialize_bytes(&snapshot);
            snapshot.retire_cold();
            let bytes = encoded.map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_TXT_DIALECT, payload: IoPayload::Text(String::from_utf8(bytes).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?), diagnostics: Vec::new(), confidence: semio_framework_plugin::io::Confidence::High })
        })
    }
    //#endregion 🔖️ExportEntries

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES
            .get_or_init(|| {
                vec![
                    composer_entry_of::<Generation3dAnyComposer>(),
                    ComposerEntry { writes: EXPORT_TXT_DIALECT, reads: &[GENERATION3D_DIALECT], compose: compose_export_txt },
                ]
            })
            .as_slice()
    }
}
//#endregion 🚪️IoRegistry




pub mod derived_construction {
    use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;
    use semio_s_artifact_procedural_3d_mutations::Generation3dMutation;
    use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct Generation3dBuilderConstruction {
        snapshot: Generation3dSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for Generation3dBuilderConstruction {
        type Snapshot = Generation3dSnapshot;
        type Mutation = Generation3dMutation;
        type Diff = Generation3dDiff;
        fn empty() -> Self {
            Self { snapshot: Generation3dSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<Generation3dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Generation3dSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            match protocol::apply_diff(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(semio_framework_diagnostic::Diagnostic::error("build.apply", semio_framework_diagnostic::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = protocol::apply_diff(&diff, &self.snapshot)?;
            self.snapshot = snapshot;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
}
pub use derived_construction::*;

pub mod derived_analysis {
    use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct Generation3dParts {
        pub snapshot: Option<Generation3dSnapshot>,
    }

    pub struct Generation3dAnalyzerAnalysis;

    impl ArtifactAnalysis for Generation3dAnalyzerAnalysis {
        type Parts = Generation3dParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.procedural.generation3d", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            semio_framework_plugin::io::Confidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Generation3dParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <Generation3dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Generation3dSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("analyze.binary", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec Generation3dBuilderFacets {
        construction: Generation3dBuilderConstruction,
        analysis: Generation3dAnalyzerAnalysis,
        composition: Generation3dComposerComposition,
    }
    builder: Generation3dBuilder,
    analyzer: Generation3dAnalyzer,
    composer: Generation3dComposer,
);
