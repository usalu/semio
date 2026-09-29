#!/usr/bin/env python3
"""🧰️ One-off: builds h14-creation-rule.json from the post-edit copies (backup dir) and the pre-edit anchors below."""
import json, pathlib
ROOT = pathlib.Path('/Users/ueli/Documents/semio')
BK = ROOT / '.🧬semio/🌐hub/s14-h14-backup/creation-rule'
CAT = '🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs'
TST = '🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🔬️unit/🦀️.rs'
INF = '🌎️hub/💡️inference/📇️catalog/🦀️.rs'
FIX = '🧰️framework/🔨️modules/🚪️io/🧫️fixtures/🌳️most-general-dialect/🔣️.json'
post = {CAT: (BK / 'catalog.post.rs').read_text(), TST: (BK / 'tests.post.rs').read_text(), INF: (BK / 'inference.post.rs').read_text()}


def region(file, start, end):
    text = post[file]
    assert text.count(start) == 1, ('start', start[:70])
    i = text.index(start)
    return text[i:text.index(end, i) + len(end)]


H = []


def hunk(file, old, new):
    assert post[file].count(new) == 1, ('new', new[:70])
    H.append({'file': file, 'old': old, 'new': new})


hunk(CAT, r'''    /// 🌱️ Creation resolves one unambiguous writable target in this exact admitted generation, OWNER-PREFERRED
    /// ([`owner_preferred_creation`]): the editors whose own package declares the kind's exact codec decide; only a kind no
    /// owner's editor creates falls to the editors of hosts whose declared dependency declares it. Hosting opens documents,
    /// it never re-owns their creation (demonstrator's embedded cad/gis/procedural/process editors made every creation of
    /// those kinds ambiguous, hub 7800 p34, 2026-09-29).
    pub(crate) fn artifact_creation_selection(&self, kind_id: &str) -> Option<&VerifiedDocumentOpenSelectionV1> {
        let binds = |selection: &VerifiedDocumentOpenSelectionV1, owned: bool| {
            self.codecs.iter().any(|codec| {
                let own = codec.identity.plugin_id == selection.package.plugin_id
                    && codec.identity.package_id == selection.package.package_id
                    && codec.identity.version == selection.package.version
                    && codec.identity.package_hash == selection.package.component_sha256;
                let hosted = codec.identity.plugin_id != selection.package.plugin_id
                    && self.packages.iter().any(|package| package.plugin_id == selection.package.plugin_id && package.dependencies.contains(&codec.identity.plugin_id));
                (if owned { own } else { hosted })
                    && codec.identity.artifact_kind == selection.artifact.kind
                    && codec.identity.artifact_schema == selection.artifact.schema
                    && codec.identity.pack_schema_hash == selection.artifact.pack_schema_hash
            })
        };
        let editors = || self.open_targets.iter().filter(|selection| selection.artifact.kind == kind_id && selection.surface.role == DocumentOpenSurfaceRoleV1::Editor);
        owner_preferred_creation(editors().filter(|selection| binds(selection, true)), editors().filter(|selection| binds(selection, false)))
    }

    /// 🗣️ Projects only unambiguous factory-backed choices from retained compiled descriptors.
    pub(crate) fn artifact_creation_catalog(&self, space_id: &str) -> Option<directory::os_directory::schema::space_artifact_creation::SpaceArtifactCreationCatalogV1> {
        use directory::os_directory::schema::space_artifact_creation::{
            SpaceArtifactCreationCatalogV1, SpaceArtifactCreationDialectV1, SpaceArtifactCreationKindV1, SpaceArtifactCreationLabelV1,
        };
        let kind_ids = self.open_targets.iter().map(|selection| selection.artifact.kind.as_str()).collect::<BTreeSet<_>>();
        let mut kinds = Vec::new();
        for kind_id in kind_ids {
            let Some(selection) = self.artifact_creation_selection(kind_id) else { continue };
            let retained = self.packages.iter().find(|package| {
                package.plugin_id == selection.package.plugin_id
                    && package.package.package.0 == selection.package.package_id
                    && package.version == selection.package.version
                    && hex_lower(&package.component_sha256) == selection.package.component_sha256
            })?;
            let app = retained.descriptor.manifest.apps.iter().find(|app| app.id == selection.surface.app_id && app.dialect == selection.parent_dialect)?;
            let kind = app.artifact_kinds.iter().chain(retained.descriptor.manifest.artifact_kinds.iter()).find(|kind| kind.id == selection.artifact.kind && kind.schema == selection.artifact.schema)?;''',
     region(CAT, "    /// 🌱️ Creation resolves one writable target in this exact admitted generation, OWNER-PREFERRED", "            let Some(kind) = declared_kind(app.artifact_kinds.iter().chain(retained.descriptor.manifest.artifact_kinds.iter()), selection).or_else(owner) else { continue };"))

hunk(CAT, r'''    /// 🎯 Resolves one exact descriptor, subject role, and optional surface preference without fallback.
    pub fn resolve_document_open(&self, descriptor: &DocumentDescriptor, requested_surface_id: Option<&str>, writable: bool) -> Option<VerifiedDocumentOpenSelectionV1> {
        let role = if writable { DocumentOpenSurfaceRoleV1::Editor } else { DocumentOpenSurfaceRoleV1::Viewer };
        let mut matches = self.open_targets.iter().filter(|selection| {''',
     region(CAT, "    /// 🎯 Resolves one exact descriptor, subject role, and optional surface preference without fallback. Without a preference", "        let matches = self.open_targets.iter().filter(|selection| {"))

hunk(CAT, r'''                && requested_surface_id.is_none_or(|requested| selection.surface.surface_id == requested)
        });
        let selected = matches.next()?.clone();
        matches.next().is_none().then_some(selected)
    }
}''', r'''                && requested_surface_id.is_none_or(|requested| selection.surface.surface_id == requested)
        });
        most_general_dialect(matches.collect(), |selection| &selection.parent_dialect).one().cloned()
    }
}''')

hunk(CAT, r'''/// 🧪️ One immutable authority identity bound to its executable. `codec` is `Some` only for a package
/// this binary links a Rust codec for (stdio import/export and GIS inference still run natively, TC2
/// §9); every other package validates and applies through its own component. `guest` is never
/// optional: it is the only creation authority there is.''',
     region(CAT, "/// 🧪️ One immutable authority identity bound to its executable. `codec` is `Some` only for a kind", "/// optional: it is the only creation authority there is, always the identity's own package."))

hunk(CAT, r'''        let mut previews = Vec::with_capacity(staged.len());
        let mut pending_rows = BTreeMap::new();
        for stage in &staged {
            context.checkpoint()?;
            let native_bindings = providers.preview(NativeCodecProviderPackageV1 { plugin_id: &stage.record.plugin_id, package_id: &stage.record.package_id, version: &stage.record.version }, &stage.descriptor, context)?;
            context.checkpoint()?;
            let bound = validate_native_bindings(&native_bindings)?.into_keys().collect::<BTreeSet<_>>();
            let mut rows = Vec::new();
            for expected in &stage.record.native_codecs {
                if bound.contains(&CodecKey::from_parts(&stage.record.plugin_id, &stage.record.package_id, &expected.artifact_kind, &expected.artifact_schema)) {
                    continue;
                }
                let expected_hash = decode_digest(&expected.pack_schema_hash, "pack schema hash")?;
                if expected_hash == [0; 32] {
                    return Err(catalog("artifact codec schema hash is zero"));
                }
                if verifications.recall(&stage.component_sha256, &expected.artifact_schema).await == Some(expected_hash) {
                    continue;
                }
                rows.push((expected.artifact_schema.clone(), expected_hash));
            }
            let pinned = u64::try_from(stage.record.native_codecs.len() - rows.len()).unwrap_or(u64::MAX);
            progress.package(stage.position, |package| {
                package.rows_pinned = pinned;''',
     region(CAT, "        let selected = |record: &TrustedBundlePackageV1, target: &TrustedBundleOpenTargetV1| {", "                package.rows_pinned = pinned;"))

hunk(CAT, r'''                let parent_dialect = validate_descriptor_open_target(&descriptor, target)?;
                if !profile.open_targets.iter().any(|selected| {
                    selected.package.plugin_id == record.plugin_id && selected.package.package_id == record.package_id && selected.package.version == record.version && selected.target == *target
                }) {
                    continue;
                }''', r'''                let parent_dialect = validate_descriptor_open_target(&descriptor, target)?;
                if !selected(record, target) {
                    continue;
                }''')

hunk(CAT, r'''                open_targets.push(selection);
            }
            report_package_progress(context, position, 4, total_units)?;''',
     region(CAT, "                open_targets.push(selection);\n            }\n            for (owner, target) in hosted_codec_targets", "            report_package_progress(context, position, 4, total_units)?;"))

hunk(CAT, r'''        return Err(catalog("trusted document-open target binds an exact native codec more than one declared dependency declares"));
    }
    Ok(owner)
}
''', region(CAT, '        return Err(catalog("trusted document-open target binds an exact native codec more than one declared dependency declares"));', "    Ok(hosted)\n}\n"))

hunk(CAT, r'''/// 🌱️ The owner-preferred creation rule over two candidate tiers — the owners' editors, then the hosts' editors: the first
/// non-empty tier decides and must hold exactly one candidate, so a host never makes an owner's kind ambiguous and two owners
/// never pick one.
fn owner_preferred_creation<T>(owned: impl Iterator<Item = T>, hosted: impl Iterator<Item = T>) -> Option<T> {
    fn unique<T>(mut candidates: impl Iterator<Item = T>) -> Option<T> {
        let first = candidates.next()?;
        candidates.next().is_none().then_some(first)
    }
    let mut owned = owned.peekable();
    if owned.peek().is_some() { unique(owned) } else { unique(hosted) }
}''', region(CAT, "/// 🌱️ The owner-preferred creation rule over two candidate tiers — the owners' editors, then the hosts' editors: the first\n/// non-empty tier decides by", "    kinds.find(|kind| kind.id == selection.artifact.kind && kind.schema == selection.artifact.schema)\n}"))

hunk(TST, r'''    fn rewrite_descriptor(&mut self, index: usize, schema: Option<&str>, dependency: Option<(&str, &str)>) {
        let record = &mut self.bundle["packages"][index];
        let bytes = descriptor_bytes(
            record["pluginId"].as_str().expect("plugin"),
            record["packageId"].as_str().expect("package"),
            record["version"].as_str().expect("version"),
            record["component"]["sha256"].as_str().expect("component hash"),
            schema,
            dependency,
        );
        record["descriptor"]["byteLength"] = bytes.len().into();''',
     region(TST, "    fn rewrite_descriptor(&mut self, index: usize, schema: Option<&str>, dependency: Option<(&str, &str)>) {", "        let record = &mut self.bundle[\"packages\"][index];\n        record[\"descriptor\"][\"byteLength\"] = bytes.len().into();"))

hunk(TST, r'''fn descriptor_bytes(plugin_id: &str, package_id: &str, version: &str, component_sha256: &str, schema: Option<&str>, dependency: Option<(&str, &str)>) -> Vec<u8> {
    let artifact_kinds = schema.map_or_else(Vec::new, |schema| {''',
     region(TST, "fn descriptor_bytes(plugin_id: &str, package_id: &str, version: &str, component_sha256: &str, schema: Option<&str>, dependency: Option<(&str, &str)>) -> Vec<u8> {", "-> serde_json::Value {\n    let artifact_kinds = schema.map_or_else(Vec::new, |schema| {"))

hunk(TST, r'''    let json = serde_json::json!({
        "descriptorVersion": 1,
        "packageId": package_id,
        "role": "plugin",
        "manifest": {
            "pluginId": plugin_id,
            "label": plugin_id,
            "version": version,
            "apps": apps,
            "examples": [],
            "artifactKinds": artifact_kinds,
            "dependencies": dependencies
        },
        "execution": "isolated",
        "executionProtocol": { "appChannelVersion": directory::os_spr::CHANNEL_VERSION },
        "hashes": {
            "wasmSha256": component_sha256,
            "coreWasmSha256": "22".repeat(32),
            "descriptorSha256": "33".repeat(32)
        }
    });
    let descriptor: PackageDescriptor = serde_json::from_value(json).expect("package descriptor");
    os_store::pack_rt::encode_wire_value(&to_dsl_value(&descriptor).expect("project descriptor"))
}''', r'''    serde_json::json!({
        "descriptorVersion": 1,
        "packageId": package_id,
        "role": "plugin",
        "manifest": {
            "pluginId": plugin_id,
            "label": plugin_id,
            "version": version,
            "apps": apps,
            "examples": [],
            "artifactKinds": artifact_kinds,
            "dependencies": dependencies
        },
        "execution": "isolated",
        "executionProtocol": { "appChannelVersion": directory::os_spr::CHANNEL_VERSION },
        "hashes": {
            "wasmSha256": component_sha256,
            "coreWasmSha256": "22".repeat(32),
            "descriptorSha256": "33".repeat(32)
        }
    })
}''')

hunk(TST, r'''/// 🌱️ LAW: creation is owner-preferred — the owners' editors decide and a host's editor of the same kind never makes it
/// ambiguous (cad + demonstrator's embedded cad editor → cad); a kind only hosts create falls to its one host (a stdio
/// family); two owners or two hosts alone stay ambiguous; nothing creates nothing.
#[test]
fn creation_prefers_the_owners_editor_over_a_hosts() {
    assert_eq!(owner_preferred_creation(["cad"].into_iter(), ["demonstrator"].into_iter()), Some("cad"));
    assert_eq!(owner_preferred_creation(std::iter::empty(), ["stdio-pdf"].into_iter()), Some("stdio-pdf"));
    assert_eq!(owner_preferred_creation(["cad", "cad-twin"].into_iter(), ["demonstrator"].into_iter()), None);
    assert_eq!(owner_preferred_creation(std::iter::empty(), ["demonstrator", "bundle"].into_iter()), None);
    assert_eq!(owner_preferred_creation(std::iter::empty::<&str>(), std::iter::empty()), None);
}''', region(TST, "/// 🌱️ LAW: creation is owner-preferred — the owners' editors decide and a host's editor of the same kind never makes it\n/// ambiguous (cad + demonstrator's embedded cad editor → cad, even", "(1, u64::from(linked)), \"the hosted identity is a row of the host, pinned only when linked\");\n    }\n}"))

hunk(INF, r'''/// `artifact_creation_selection` is the same unambiguity rule scoped to one kind: exactly one
/// writable editor target whose codec identity this generation verified.''', r'''/// `artifact_creation_selection` is the creation rule scoped to one kind: the owners' ONE most
/// general writable editor target whose codec identity this generation verified.''')

H.append({'file': FIX, 'create': True, 'new': (ROOT / FIX).read_text()})
out = pathlib.Path(__file__).with_name('h14-creation-rule.json')
out.write_text(json.dumps(H, ensure_ascii=False, indent=1) + '\n')
print(len(H), 'hunks ->', out)
