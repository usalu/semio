#!/usr/bin/env python3
"""🏠️ LB2 p15 (window 3, T6 row 3 with p9 — REQUIRES p9 written first): a hosting package is a first-class trusted-catalog member.

publish-all (W4 chain, 2026-09-29 11:41) refused the nine stdio family packages: "component codec probe requires at least one
declared artifact kind" — p9 made each family self-sufficient in its own guest (`host_artifact`), but its descriptor still
declared nothing it opens, and the trusted catalog knew only OWN kinds and OWN codec rows. End to end:

- descriptor (schema-first, framework manifest + its TS projection): `PluginManifest.hostedArtifactKinds` — one
  `HostedArtifactKind { id, schema }` row per hosted kind × document schema its owner's codecs decode (`host_artifact`
  declarations, `ArtifactDeclaration::hosted_kinds`), distinct from owned `artifactKinds`, skipped when empty (every other
  descriptor stays byte-identical).
- hub (`🔏️trusted-catalog`): the one pairing rule (`app_opens_kind`, `descriptor_open_targets`) opens a hosted kind in the host
  surfaces whose dialect names it (the plugin-level rule); a document-open target binds its own package's native codec or —
  for a hosted kind — the codec of the kind's OWNER (`open_target_codec_package`): the owner must be in the same catalog, a
  declared dependency of the host, and declare that exact codec, else a named refusal (bundle validation and generation load
  alike); codec rows never move (a host carries none for what it hosts); the decoded manifest's hosted rows must name a
  declared dependency that is present; creation resolves a hosted target against its owner's codec.
- publication (`📜️script.ts materializeTrustedCatalogBundle`): a package with hosted rows and no owned kind skips the codec
  probe; its open targets bind the owner's verified rows (the owner is published earlier in the same catalog, else refused);
  a hosted row whose owner publishes no codec opens nothing — the owner's own rule for its unlinked kinds.
- laws: `shipped_fleet` (describe side: families own no kind, host every kind they open, every hosted row is a document codec of
  its owner's runtime declaration, stdio hosts nothing); hub unit (`descriptor_open_targets` hosted case through the shared
  fixture; `open_target_codec_package` accepts the owner's codec and refuses an absent owner, a non-dependency, a missing codec).

usage: python3 lb2-p15-hosted-kinds-hub.py --dry-run | --write | --revert [--root <tree>]
Backups (byte-exact, per root) under `.🧬semio/🌐hub/s14-lb2-backup/p15/<root-hash>/`.
"""
import hashlib, json, os, re, shutil, subprocess, sys

TREE = sys.argv[sys.argv.index("--root") + 1] if "--root" in sys.argv else "/Users/ueli/Documents/semio"
BACKUP = f"/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-lb2-backup/p15/{hashlib.sha256(TREE.encode()).hexdigest()[:12]}"
MANIFEST = "🧰️framework/🔨️modules/🛂️manifest/🦀️.rs"
PROJECTION = "🧰️framework/🔨️modules/🧬️schema/📽️projection/🦀️.rs"
GENERATED = "🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts"
TYPEGEN_LAW = "🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🔬️app-label/🦀️.rs"
SDK = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
BUILDER = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️.rs"
HUB = "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs"
HUB_TESTS = "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🔬️unit/🦀️.rs"
HUB_FIXTURE = "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🎯️descriptor-open-targets/🔣️.json"
HUB_SCRIPT = "🌎️hub/📦️packages/🦀️rust/📜️script.ts"
LAW = "✏️s/🔌️plugins/🗄️stdio/🧪️tests/🚢️shipped-fleet/🦀️.rs"
LITERAL_ROOTS = ["🧰️framework", "✏️s", "🌎️hub"]

problems = []


def once(text, old, new, label):
    count = text.count(old)
    if count != 1:
        problems.append(f"{label}: expected 1 anchor, found {count}")
        return text
    return text.replace(old, new)


#region Descriptor
HOSTED_DOC = (
    "    /// 🏠️ Artifact kinds another package owns whose documents this plugin's apps open — one row per kind and document\n"
    "    /// schema its owner's codecs decode (`PluginBuilder::host_artifact`). Distinct from `artifact_kinds`: the codec rows stay\n"
    "    /// the owner's, and a trusted catalog binds these kinds to the owner's codecs.\n"
)


def manifest(text):
    text = once(
        text,
        "    /// 🗂️ Plugin-level artifact kinds (library plugins with zero apps declare kinds here).\n    #[serde(default)]\n    #[value(default)]\n    pub artifact_kinds: Vec<ArtifactKindSpec>,\n",
        "    /// 🗂️ Plugin-level artifact kinds (library plugins with zero apps declare kinds here).\n    #[serde(default)]\n    #[value(default)]\n    pub artifact_kinds: Vec<ArtifactKindSpec>,\n"
        + HOSTED_DOC
        + '    #[serde(default, skip_serializing_if = "Vec::is_empty")]\n    #[value(default, skip_serializing_if = "Vec::is_empty")]\n    pub hosted_artifact_kinds: Vec<HostedArtifactKind>,\n',
        "manifest: field",
    )
    return once(
        text,
        "//#endregion ArtifactKind\n",
        "\n"
        "/// 🏠️ One artifact kind a package hosts — its owner's kind id and one document schema of the owner's codecs (see\n"
        "/// `PluginManifest::hosted_artifact_kinds`). The owner is the kind's plugin (`s.<owner>.<artifact>`).\n"
        "#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]\n"
        '#[serde(rename_all = "camelCase", deny_unknown_fields)]\n'
        '#[value(rename_all = "camelCase", deny_unknown_fields)]\n'
        "pub struct HostedArtifactKind {\n"
        "    pub id: String,\n"
        "    pub schema: String,\n"
        "}\n"
        "//#endregion ArtifactKind\n",
        "manifest: HostedArtifactKind",
    )


TS_FIELD_OLD = "artifactKinds: Array<ArtifactKindSpec>,\n/**\n * 🔗️ Direct plugin dependencies"
TS_FIELD_NEW = (
    "artifactKinds: Array<ArtifactKindSpec>,\n"
    "/**\n"
    " * 🏠️ Artifact kinds another package owns whose documents this plugin's apps open — one row per kind and document\n"
    " * schema its owner's codecs decode (`PluginBuilder::host_artifact`). Distinct from `artifact_kinds`: the codec rows stay\n"
    " * the owner's, and a trusted catalog binds these kinds to the owner's codecs.\n"
    " */\n"
    "hostedArtifactKinds: Array<HostedArtifactKind>,\n"
    "/**\n"
    " * 🔗️ Direct plugin dependencies"
)
TS_TYPE = (
    "/**\n"
    " * 🏠️ One artifact kind a package hosts — its owner's kind id and one document schema of the owner's codecs (see\n"
    " * `PluginManifest::hosted_artifact_kinds`). The owner is the kind's plugin (`s.<owner>.<artifact>`).\n"
    " */\n"
    "export type HostedArtifactKind = { id: string, schema: string, };"
)


def projection(text):
    text = once(text, TS_FIELD_OLD, TS_FIELD_NEW, "projection: PluginManifest field")
    return once(
        text,
        '        SchemaMetadata {\n            name: "IconName",\n',
        '        SchemaMetadata {\n            name: "HostedArtifactKind",\n            version: 1,\n            typescript: r####"' + TS_TYPE + '"####,\n        },\n        SchemaMetadata {\n            name: "IconName",\n',
        "projection: HostedArtifactKind",
    )


def generated(text):
    text = once(text, TS_FIELD_OLD, TS_FIELD_NEW, "generated: PluginManifest field")
    icon = re.search(r"\n\n(/\*\*\n(?: \*[^\n]*\n)* \*/\nexport type IconName\b|export type IconName\b)", text)
    if not icon:
        problems.append("generated: IconName anchor")
        return text
    return text[: icon.start()] + "\n\n" + TS_TYPE + text[icon.start() :]


def typegen_law(text):
    """🧬️ The projection law pins no magic count (it went stale at 197 while the registry held 201): the generated TypeScript
    equals the generator's rendering of the current registry, which `validate()` already proves well-formed and unique."""
    found = re.findall(r"\n    assert_eq!\(crate::schema_metadata::TYPES\.len\(\), \d+\);", text)
    if len(found) != 1:
        problems.append("typegen law: type count pin")
        return text
    return text.replace(found[0], "")


def literal_end(text, start):
    depth = 0
    for index in range(start, len(text)):
        if text[index] == "{":
            depth += 1
        elif text[index] == "}":
            depth -= 1
            if depth == 0:
                return index
    return None


def field_value_end(body, start):
    depth = 0
    for index in range(start, len(body)):
        character = body[index]
        if character in "([{":
            depth += 1
        elif character in ")]}":
            depth -= 1
        elif character == "," and depth == 0:
            newline = body.find("\n", index)
            return None if newline < 0 else newline + 1
    return None


def manifest_literals(text, label):
    out, cursor, found = [], 0, 0
    for match in real_literals(text):
        brace = match.end() - 1
        end = literal_end(text, brace)
        if end is None:
            problems.append(f"{label}: unbalanced PluginManifest literal")
            return text
        body = text[brace:end]
        if "hosted_artifact_kinds" in body or re.search(r"\.\.\s*[A-Za-z_(]", body):
            continue
        field = re.search(r"\n(\s*)artifact_kinds[:,]", body)
        inline = re.search(r"(artifact_kinds: [^,{}]*(?:\{[^{}]*\})?[^,{}]*, )", body) if not field else None
        if field:
            end_of_value = field_value_end(body, field.end() - 1)
            if end_of_value is None:
                problems.append(f"{label}: unterminated artifact_kinds value")
                continue
            insert = brace + end_of_value
            out.append(text[cursor:insert] + f"{field.group(1)}hosted_artifact_kinds: Vec::new(),\n")
        elif inline:
            insert = brace + inline.end()
            out.append(text[cursor:insert] + "hosted_artifact_kinds: Vec::new(), ")
        else:
            problems.append(f"{label}: PluginManifest literal without artifact_kinds at {text.count(chr(10), 0, match.start()) + 1}")
            continue
        cursor = insert
        found += 1
    out.append(text[cursor:])
    if not found:
        problems.append(f"{label}: no literal changed")
    return "".join(out)


def real_literals(text):
    return [
        match
        for match in re.finditer(r"(?<![A-Za-z_])PluginManifest \{", text)
        if not re.search(r"(pub struct |for |-> (?:[A-Za-z_]+::)*)$", text[max(0, match.start() - 64) : match.start()])
    ]


def literal_files():
    found = subprocess.run(["/usr/bin/grep", "-rl", "--include=🦀️.rs", "--exclude-dir=node_modules", "--exclude-dir=dist", "PluginManifest {", *LITERAL_ROOTS], cwd=TREE, capture_output=True, text=True).stdout.split("\n")
    return sorted(path for path in found if path and real_literals(open(os.path.join(TREE, path), encoding="utf-8").read()))


#endregion Descriptor


#region Sdk
def sdk(text):
    return once(
        text,
        "        /// 📚️ Returns this declaration's one authoritative artifact definition.\n        pub fn definition(&self) -> &ArtifactDefinition {\n            &self.definition\n        }\n",
        "        /// 📚️ Returns this declaration's one authoritative artifact definition.\n        pub fn definition(&self) -> &ArtifactDefinition {\n            &self.definition\n        }\n"
        "\n"
        "        /// 🏠️ The descriptor rows a package hosting this declaration opens: its kind with each document schema its codecs\n"
        "        /// decode — see [`semio_framework::HostedArtifactKind`].\n"
        "        pub fn hosted_kinds(&self) -> Vec<semio_framework::HostedArtifactKind> {\n"
        "            self.document_codecs.iter().map(|codec| semio_framework::HostedArtifactKind { id: self.definition.identity().as_str().to_string(), schema: codec.schema.clone() }).collect()\n"
        "        }\n",
        "sdk: hosted_kinds",
    )


def builder(text):
    return once(
        text,
        "        plugin.manifest.topic_contributions = topic_contributions;\n        for declaration in artifacts.into_iter().chain(hosted_artifacts) {\n",
        "        plugin.manifest.topic_contributions = topic_contributions;\n        plugin.manifest.hosted_artifact_kinds = hosted_artifacts.iter().flat_map(ArtifactDeclaration::hosted_kinds).collect();\n        for declaration in artifacts.into_iter().chain(hosted_artifacts) {\n",
        "builder: hosted kinds (requires p9)",
    )
#endregion Sdk


#region Hub
def hub(text):
    text = once(
        text,
        "/// and by the viewers of that editor's dialect, the read-only surface of the same documents.\nfn app_opens_kind(",
        "/// and by the viewers of that editor's dialect, the read-only surface of the same documents. A HOSTED kind\n"
        "/// (`PluginManifest::hosted_artifact_kinds`, `PluginBuilder::host_artifact`) follows the plugin-level rule: another\n"
        "/// package's documents open in the host surfaces whose dialect names the kind.\n"
        "fn app_opens_kind(",
        "hub: pairing doc",
    )
    text = once(
        text,
        "    if declares(&descriptor.manifest.artifact_kinds) {\n        return app.dialect.artifact_kind == artifact_kind;\n    }\n",
        "    let hosts = descriptor.manifest.hosted_artifact_kinds.iter().any(|kind| kind.id == artifact_kind && kind.schema == artifact_schema);\n"
        "    if declares(&descriptor.manifest.artifact_kinds) || hosts {\n        return app.dialect.artifact_kind == artifact_kind;\n    }\n",
        "hub: app_opens_kind",
    )
    text = once(
        text,
        "        for kind in descriptor.manifest.artifact_kinds.iter().chain(app.artifact_kinds.iter()).chain(editors.flat_map(|editor| editor.artifact_kinds.iter())) {\n"
        "            if !seen.insert((kind.id.as_str(), kind.schema.as_str())) || !app_opens_kind(descriptor, app, &kind.id, &kind.schema) {\n"
        "                continue;\n"
        "            }\n"
        "            targets.push(schema::TrustedDescriptorOpenTargetV1 {\n"
        "                artifact_kind: kind.id.clone(),\n"
        "                artifact_schema: kind.schema.clone(),\n",
        "        let owned = descriptor.manifest.artifact_kinds.iter().chain(app.artifact_kinds.iter()).chain(editors.flat_map(|editor| editor.artifact_kinds.iter())).map(|kind| (kind.id.as_str(), kind.schema.as_str()));\n"
        "        let hosted = descriptor.manifest.hosted_artifact_kinds.iter().map(|kind| (kind.id.as_str(), kind.schema.as_str()));\n"
        "        for (kind_id, kind_schema) in owned.chain(hosted) {\n"
        "            if !seen.insert((kind_id, kind_schema)) || !app_opens_kind(descriptor, app, kind_id, kind_schema) {\n"
        "                continue;\n"
        "            }\n"
        "            targets.push(schema::TrustedDescriptorOpenTargetV1 {\n"
        "                artifact_kind: kind_id.to_string(),\n"
        "                artifact_schema: kind_schema.to_string(),\n",
        "hub: descriptor_open_targets",
    )
    text = once(
        text,
        "/// 📤️ Answers `os-hub trusted-catalog open-targets`:",
        "/// 🏷️ The owner plugin of an artifact kind id (`s.<owner>.<artifact>`; a plugin id carries no `.`).\n"
        "fn hosted_kind_owner(kind: &str) -> Option<&str> {\n"
        "    kind.strip_prefix(\"s.\")?.split_once('.').map(|(owner, _)| owner).filter(|owner| !owner.is_empty())\n"
        "}\n"
        "\n"
        "/// 🏠️ The package whose native codec one document-open target of `host` binds: `host` itself when it declares that exact\n"
        "/// codec, else — for a kind `host` hosts — the kind's owner package, which must be in this catalog, a declared dependency\n"
        "/// of `host`, and declare the exact codec. Codec rows never move: a host carries none for the kinds it hosts.\n"
        "fn open_target_codec_package<'a>(packages: &'a [TrustedBundlePackageV1], host: &'a TrustedBundlePackageV1, target: &TrustedBundleOpenTargetV1) -> Result<&'a TrustedBundlePackageV1, AuthorityError> {\n"
        "    let binds = |package: &TrustedBundlePackageV1| package.native_codecs.iter().any(|codec| codec.artifact_kind == target.artifact_kind && codec.artifact_schema == target.artifact_schema && codec.pack_schema_hash == target.pack_schema_hash);\n"
        "    if binds(host) {\n"
        "        return Ok(host);\n"
        "    }\n"
        "    let owner = hosted_kind_owner(&target.artifact_kind).filter(|owner| *owner != host.plugin_id).ok_or_else(|| catalog(\"trusted document-open target is bound to no native codec of its own package\"))?;\n"
        "    let owner = packages.iter().find(|package| package.plugin_id == owner).ok_or_else(|| catalog(\"trusted document-open target hosts a kind whose owner package is absent from the catalog\"))?;\n"
        "    if !host.dependencies.iter().any(|dependency| dependency.plugin_id == owner.plugin_id) {\n"
        "        return Err(catalog(\"trusted document-open target hosts a kind whose owner is not a declared dependency of its package\"));\n"
        "    }\n"
        "    if !binds(owner) {\n"
        "        return Err(catalog(\"trusted document-open target hosts a kind its owner package declares no exact native codec for\"));\n"
        "    }\n"
        "    Ok(owner)\n"
        "}\n"
        "\n"
        "/// 📤️ Answers `os-hub trusted-catalog open-targets`:",
        "hub: open_target_codec_package",
    )
    text = once(
        text,
        "            if !package.native_codecs.iter().any(|codec| codec.artifact_kind == target.artifact_kind && codec.artifact_schema == target.artifact_schema && codec.pack_schema_hash == target.pack_schema_hash) {\n"
        "                return Err(catalog(\"trusted document-open target is bound to no native codec of its own package\"));\n"
        "            }\n",
        "            open_target_codec_package(&bundle.packages, package, target)?;\n",
        "hub: validate_bundle binding",
    )
    text = once(
        text,
        "                let declared = record.native_codecs.iter().any(|codec| codec.artifact_kind == target.artifact_kind && codec.artifact_schema == target.artifact_schema && codec.pack_schema_hash == target.pack_schema_hash);\n"
        "                if !declared {\n"
        "                    return Err(catalog(\"document-open target has no exact verified native codec\"));\n"
        "                }\n",
        "                open_target_codec_package(&bundle.packages, record, target)?;\n",
        "hub: load binding",
    )
    text = once(
        text,
        "        if !record.native_codecs.iter().any(|codec| codec.artifact_kind == kind.id && codec.artifact_schema == kind.schema) {\n"
        "            return Err(catalog(\"decoded manifest artifact kind is absent from the trust record\"));\n"
        "        }\n"
        "    }\n",
        "        if !record.native_codecs.iter().any(|codec| codec.artifact_kind == kind.id && codec.artifact_schema == kind.schema) {\n"
        "            return Err(catalog(\"decoded manifest artifact kind is absent from the trust record\"));\n"
        "        }\n"
        "    }\n"
        "    let mut hosted_kinds = BTreeSet::new();\n"
        "    for kind in &descriptor.manifest.hosted_artifact_kinds {\n"
        "        if !hosted_kinds.insert((kind.id.as_str(), kind.schema.as_str())) || manifest_kinds.contains(kind.id.as_str()) {\n"
        "            return Err(catalog(\"decoded manifest hosted kind is duplicated or also owned\"));\n"
        "        }\n"
        "        let owner = hosted_kind_owner(&kind.id).filter(|owner| *owner != record.plugin_id).ok_or_else(|| catalog(\"decoded manifest hosted kind names no other owner package\"))?;\n"
        "        if !manifest_dependencies.contains(owner) || !records.contains_key(owner) {\n"
        "            return Err(catalog(\"decoded manifest hosted kind's owner is not a declared dependency present in the catalog\"));\n"
        "        }\n"
        "    }\n",
        "hub: manifest hosted kinds",
    )
    return once(
        text,
        "                && self.codecs.iter().any(|codec| {\n"
        "                    codec.identity.plugin_id == selection.package.plugin_id\n"
        "                        && codec.identity.package_id == selection.package.package_id\n"
        "                        && codec.identity.version == selection.package.version\n"
        "                        && codec.identity.package_hash == selection.package.component_sha256\n"
        "                        && codec.identity.artifact_kind == selection.artifact.kind\n",
        "                && self.codecs.iter().any(|codec| {\n"
        "                    let own = codec.identity.plugin_id == selection.package.plugin_id\n"
        "                        && codec.identity.package_id == selection.package.package_id\n"
        "                        && codec.identity.version == selection.package.version\n"
        "                        && codec.identity.package_hash == selection.package.component_sha256;\n"
        "                    let hosted = hosted_kind_owner(&selection.artifact.kind) == Some(codec.identity.plugin_id.as_str())\n"
        "                        && self.packages.iter().any(|package| package.plugin_id == selection.package.plugin_id && package.dependencies.contains(&codec.identity.plugin_id));\n"
        "                    (own || hosted)\n"
        "                        && codec.identity.artifact_kind == selection.artifact.kind\n",
        "hub: creation selection",
    )


def hub_fixture(text):
    document = json.loads(text)
    cases = document["cases"]
    plugin = next((case for case in cases if case["declaredOn"] == "plugin" and case["targets"]), None)
    if plugin is None or any(case["declaredOn"] == "hosted" for case in cases):
        problems.append("hub fixture: plugin case / hosted case")
        return text
    cases.append({**plugin, "name": "hosted-kind-named-by-the-host-dialect", "declaredOn": "hosted"})
    return json.dumps(document, indent=2, ensure_ascii=False) + "\n"


def hub_tests(text):
    text = once(
        text,
        "        if case[\"declaredOn\"] == \"plugin-and-editor\" {\n",
        "        if case[\"declaredOn\"] == \"hosted\" {\n"
        "            let kind = descriptor.manifest.artifact_kinds.remove(0);\n"
        "            descriptor.manifest.hosted_artifact_kinds.push(semio_framework::HostedArtifactKind { id: kind.id, schema: kind.schema });\n"
        "        }\n"
        "        if case[\"declaredOn\"] == \"plugin-and-editor\" {\n",
        "hub tests: hosted case",
    )
    return text + (
        "\n"
        "/// 🏠️ A hosted kind's document-open target binds its OWNER's native codec — never a copied row: accepted when the owner is\n"
        "/// in the catalog, a declared dependency and declares the exact codec; each missing condition is refused by name.\n"
        "#[test]\n"
        "fn hosted_open_targets_bind_the_owner_codec_or_are_refused_by_name() {\n"
        "    let fixture = fixture_json();\n"
        "    let bundle: TrustedBundleV1 = serde_json::from_value(fixture[\"bundle\"].clone()).expect(\"bundle\");\n"
        "    let mut host = bundle.packages[0].clone();\n"
        "    let mut owner = bundle.packages[1].clone();\n"
        "    let target = host.open_targets[0].clone();\n"
        "    owner.plugin_id = hosted_kind_owner(&target.artifact_kind).expect(\"kind owner\").to_string();\n"
        "    owner.native_codecs = std::mem::take(&mut host.native_codecs);\n"
        "    host.dependencies[0].plugin_id = owner.plugin_id.clone();\n"
        "    let packages = vec![host.clone(), owner.clone()];\n"
        "    assert_eq!(open_target_codec_package(&packages, &packages[0], &target).expect(\"hosted target binds the owner codec\").plugin_id, owner.plugin_id);\n"
        "    let refusal = |packages: Vec<TrustedBundlePackageV1>| open_target_codec_package(&packages, &packages[0], &target).expect_err(\"refused\").to_string();\n"
        "    assert!(refusal(vec![host.clone()]).contains(\"owner package is absent\"));\n"
        "    let mut stranger = host.clone();\n"
        "    stranger.dependencies.clear();\n"
        "    assert!(refusal(vec![stranger, owner.clone()]).contains(\"not a declared dependency\"));\n"
        "    let mut bare = owner.clone();\n"
        "    bare.native_codecs.clear();\n"
        "    assert!(refusal(vec![host.clone(), bare]).contains(\"declares no exact native codec\"));\n"
        "    let mut own = host.clone();\n"
        "    own.native_codecs = owner.native_codecs.clone();\n"
        "    assert_eq!(open_target_codec_package(&[own.clone()], &own, &target).expect(\"own codec\").plugin_id, own.plugin_id);\n"
        "}\n"
    )
#endregion Hub


#region Publication
def hub_script(text):
    text = once(
        text,
        "        const manifestKinds = trustedBootstrapDescriptorKindsV1(descriptorJson.get(request.pluginId)!).map((kind) => [String(kind.id), String(kind.schema)] as const);\n",
        "        const manifestKinds = trustedBootstrapDescriptorKindsV1(descriptorJson.get(request.pluginId)!).map((kind) => [String(kind.id), String(kind.schema)] as const);\n"
        "        const hostedKinds = trustedBootstrapHostedKindsV1(request.pluginId, descriptorJson.get(request.pluginId)!, codecs);\n",
        "script: hosted kinds",
    )
    text = once(
        text,
        "        } else {\n          const emitter = join(target, \"debug\", process.platform === \"win32\" ? \"semio-framework-plugin-describe.exe\" : \"semio-framework-plugin-describe\");\n",
        "        } else if (manifestKinds.length === 0 && hostedKinds.pairs.length > 0) {\n"
        "          codecs[request.pluginId] = [];\n"
        "          unownedKinds.set(request.pluginId, hostedKinds.unowned);\n"
        "        } else {\n          const emitter = join(target, \"debug\", process.platform === \"win32\" ? \"semio-framework-plugin-describe.exe\" : \"semio-framework-plugin-describe\");\n",
        "script: hosting package skips the probe",
    )
    text = once(
        text,
        "        for (const declared of declaredOpenTargets.get(request.pluginId)!(codecs[request.pluginId]!, unownedKinds.get(request.pluginId) ?? new Set())) openTargets.push(declared);\n",
        "        for (const declared of declaredOpenTargets.get(request.pluginId)!([...codecs[request.pluginId]!, ...hostedKinds.rows], unownedKinds.get(request.pluginId) ?? new Set(), hostedKinds.unbound)) openTargets.push(declared);\n",
        "script: hosted targets bind owner rows",
    )
    text = once(
        text,
        "    const declaredOpenTargets = new Map<string, (codecs: readonly TrustedBootstrapCodec[], unowned: ReadonlySet<string>) => readonly TrustedBootstrapOpenTargetV1[]>();\n",
        "    const declaredOpenTargets = new Map<string, (codecs: readonly TrustedBootstrapCodec[], unowned: ReadonlySet<string>, unbound: ReadonlySet<string>) => readonly TrustedBootstrapOpenTargetV1[]>();\n",
        "script: declared targets map type",
    )
    text = once(
        text,
        "          declaredOpenTargets.set(request.pluginId, (rows, unowned) => {\n            try {\n              return trustedBootstrapHubOpenTargetsV1(hubBinary, request.pluginId, descriptorCopy, rows, unowned);\n",
        "          declaredOpenTargets.set(request.pluginId, (rows, unowned, unbound) => {\n            try {\n              return trustedBootstrapHubOpenTargetsV1(hubBinary, request.pluginId, descriptorCopy, rows, unowned, unbound);\n",
        "script: declared targets pass unbound hosted pairs",
    )
    text = once(
        text,
        "function trustedBootstrapHubOpenTargetsV1(hubBinary: string, pluginId: string, descriptor: Uint8Array, codecs: readonly TrustedBootstrapCodec[], unowned: ReadonlySet<string>): readonly TrustedBootstrapOpenTargetV1[] {\n",
        "function trustedBootstrapHubOpenTargetsV1(hubBinary: string, pluginId: string, descriptor: Uint8Array, codecs: readonly TrustedBootstrapCodec[], unowned: ReadonlySet<string>, unbound: ReadonlySet<string> = new Set()): readonly TrustedBootstrapOpenTargetV1[] {\n",
        "script: open targets signature",
    )
    text = once(
        text,
        "    document.targets.filter((value: any) => !unowned.has(String(value?.artifactKind))).map((value: unknown) => {\n",
        "    document.targets.filter((value: any) => !unowned.has(String(value?.artifactKind)) && !unbound.has(`${String(value?.artifactKind)} ${String(value?.artifactSchema)}`)).map((value: unknown) => {\n",
        "script: open targets skip unbound hosted pairs",
    )
    return once(
        text,
        "function trustedBootstrapDescriptorKindsV1(descriptor: Record<string, any>): readonly Record<string, any>[] {\n",
        "/** 🏠️ The kinds one package HOSTS (`manifest.hostedArtifactKinds`, `PluginBuilder::host_artifact`): each owner (the kind's\n"
        " * plugin, `s.<owner>.<artifact>`) must be a declared dependency already published in this catalog; a hosted row binds the\n"
        " * owner's verified codec row (codec rows never move), and a row the owner publishes no codec for opens nothing — the owner's\n"
        " * own rule for its unlinked kinds (the owner publishes ONE codec row per kind, so a kind's other document schemas stay\n"
        " * unbound). */\n"
        "function trustedBootstrapHostedKindsV1(pluginId: string, descriptor: Record<string, any>, codecs: Readonly<Record<string, readonly TrustedBootstrapCodec[]>>): Readonly<{ pairs: readonly (readonly [string, string])[]; rows: readonly TrustedBootstrapCodec[]; unowned: ReadonlySet<string>; unbound: ReadonlySet<string> }> {\n"
        "  const manifest = descriptor.manifest as Record<string, any>;\n"
        "  const dependencies = new Set(((manifest.dependencies ?? []) as Record<string, any>[]).map((dependency) => String(dependency.pluginId)));\n"
        "  const pairs = ((manifest.hostedArtifactKinds ?? []) as Record<string, any>[]).map((kind) => [String(kind.id), String(kind.schema)] as const);\n"
        "  const rows: TrustedBootstrapCodec[] = [];\n"
        "  const unowned = new Set<string>();\n"
        "  const unbound = new Set<string>();\n"
        "  for (const [id, schema] of pairs) {\n"
        "    const owner = /^s\\.([^.]+)\\./.exec(id)?.[1];\n"
        "    if (!owner || owner === pluginId || !dependencies.has(owner)) throw new Error(`trusted catalog package ${pluginId} hosts ${id}, whose owner is not one of its declared dependencies`);\n"
        "    const published = codecs[owner];\n"
        "    if (!published) throw new Error(`trusted catalog package ${pluginId} hosts ${id}, whose owner ${owner} is not published before it in this catalog`);\n"
        "    const row = published.find((codec) => codec.artifactKind === id && codec.artifactSchema === schema);\n"
        "    if (row) rows.push(row);\n"
        "    else {\n"
        "      unowned.add(id);\n"
        "      unbound.add(`${id} ${schema}`);\n"
        "    }\n"
        "  }\n"
        "  for (const row of rows) unowned.delete(row.artifactKind);\n"
        "  return Object.freeze({ pairs: Object.freeze(pairs), rows: Object.freeze(rows), unowned, unbound });\n"
        "}\n"
        "\n"
        "function trustedBootstrapDescriptorKindsV1(descriptor: Record<string, any>): readonly Record<string, any>[] {\n",
        "script: trustedBootstrapHostedKindsV1",
    )
PREFLIGHT_FIXTURE = "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🛫️catalog-selection-preflight/🔣️.json"
PREFLIGHT_LAW = "🌎️hub/🧪️tests/🛫️catalog-selection-preflight/🟦️.ts"


def selection_rule(text):
    text = once(
        text,
        " * trusted identity; a package without linked codecs declares at least one artifact kind — its component can only answer codec\n"
        " * rows for kinds it declares, and the catalog has no hosted-package model — (which of them its apps OWN is the `describe` gate's\n"
        " * `first_owned_codec` law); a linked package's codec registry exists;",
        " * trusted identity; a package without linked codecs declares an artifact kind of its own (its component answers the codec rows)\n"
        " * or HOSTS kinds (`manifest.hostedArtifactKinds`, p15) — each hosted kind's owner (`s.<owner>.<artifact>`) a declared dependency\n"
        " * selected in this catalog that declares the kind (its codec row is the owner's); which kinds its apps OWN is the `describe`\n"
        " * gate's `first_owned_codec` law; a linked package's codec registry exists;",
        "script: selection rule doc",
    )
    return once(
        text,
        "    } else if (kinds.length === 0) findings.push(`${row.pluginId}: declares no artifact kind — its apps host kinds another package owns, and a trusted-catalog package must answer at least one codec row of its own (no hosted-package model)`);\n"
        "  }\n",
        "    }\n"
        "    const manifest = (row.descriptor.manifest ?? {}) as Record<string, any>;\n"
        "    const hosted = (manifest.hostedArtifactKinds ?? []) as Record<string, any>[];\n"
        "    if (row.linkedCodecRegistry === null && kinds.length === 0 && hosted.length === 0) findings.push(`${row.pluginId}: declares no artifact kind, owned or hosted — a trusted-catalog package answers a codec row of its own or hosts kinds a selected owner declares`);\n"
        "    const dependencies = new Set(((manifest.dependencies ?? []) as Record<string, any>[]).map((dependency) => String(dependency.pluginId)));\n"
        "    for (const kind of hosted) {\n"
        "      const owner = /^s\\.([^.]+)\\./u.exec(String(kind.id))?.[1];\n"
        "      const ownerRow = packages.find((candidate) => candidate.pluginId === owner);\n"
        "      if (!bounded(kind.id) || !bounded(kind.schema)) findings.push(`${row.pluginId}: hosted kind ${JSON.stringify(String(kind.id).slice(0, 96))} has an unbounded id or schema`);\n"
        "      else if (!owner || owner === row.pluginId || !dependencies.has(owner) || !ownerRow || !trustedBootstrapDescriptorKindsV1(ownerRow.descriptor).some((declared) => declared.id === kind.id)) findings.push(`${row.pluginId}: hosts ${kind.id}, whose owner ${owner ?? \"(none)\"} is not a declared dependency selected in this catalog that declares it`);\n"
        "    }\n"
        "  }\n",
        "script: selection rule",
    )


def preflight_law(text):
    return once(
        text,
        " * a package identity or declared kind is unbounded, a package without linked codecs declares no artifact kind (hosted-only), or a\n",
        " * a package identity or declared kind is unbounded, a package without linked codecs declares no kind of its own nor hosts one, a\n * hosted kind's owner is not a selected declared dependency declaring it, or a\n",
        "preflight law: doc",
    )


def preflight_fixture(text):
    document = json.loads(text)
    cases = {case["id"]: case for case in document["cases"]}
    if "hosted-package-refused" not in cases or any(case_id.startswith("hosted-kind") for case_id in cases):
        problems.append("preflight fixture: anchor cases")
        return text
    stdio = json.loads(json.dumps(cases["hosted-package-refused"]["packages"][0]))
    stdio["descriptor"]["manifest"]["artifactKinds"] = [{"id": "s.stdio.pdf", "schema": "stdio.pdf"}]
    family = json.loads(json.dumps(cases["hosted-package-refused"]["packages"][1]))
    family["descriptor"]["manifest"]["dependencies"] = [{"pluginId": "stdio", "version": "=0.1.0"}]
    family["descriptor"]["manifest"]["hostedArtifactKinds"] = [{"id": "s.stdio.pdf", "schema": "stdio.pdf"}]
    stranger = json.loads(json.dumps(family))
    stranger["descriptor"]["manifest"]["dependencies"] = []
    undeclared = json.loads(json.dumps(stdio))
    undeclared["descriptor"]["manifest"]["artifactKinds"] = []
    profile = cases["hosted-package-refused"]["profileId"]
    document["cases"].extend(
        [
            {"id": "hosted-kind-of-a-selected-owner-accepted", "profileId": profile, "packages": [stdio, family], "findings": []},
            {"id": "hosted-kind-owner-outside-the-selection-refused", "profileId": "local-stdio-pdf-open-v1", "packages": [family], "findings": ["stdio-pdf: hosts s.stdio.pdf"]},
            {"id": "hosted-kind-owner-not-a-dependency-refused", "profileId": profile, "packages": [stdio, stranger], "findings": ["stdio-pdf: hosts s.stdio.pdf"]},
            {"id": "hosted-kind-the-owner-does-not-declare-refused", "profileId": profile, "packages": [undeclared, family], "findings": ["stdio-pdf: hosts s.stdio.pdf"]},
        ]
    )
    return json.dumps(document, indent=2, ensure_ascii=False) + "\n"


#endregion Publication


#region Law
def law(text):
    return text + (
        "\n"
        "/// 🏠️ LAW (describe side of hosting, p15): every stdio FAMILY package owns no artifact kind and its descriptor hosts every\n"
        "/// kind it opens, each hosted row a document codec its owner's runtime declaration declares (kind × codec schema), so a trusted\n"
        "/// catalog routes the owner's documents to the family's editors and binds the owner's codec; the owner `stdio` hosts nothing.\n"
        "#[test]\n"
        "fn every_family_descriptor_hosts_exactly_its_owners_codecs_for_the_kinds_it_opens() {\n"
        '    let assemblies = semio_s_plugin_stdio::registry::artifact_assemblies().expect("the stdio artifact assemblies");\n'
        "    let owner_codecs = assemblies\n"
        "        .iter()\n"
        "        .filter_map(|assembly| match assembly {\n"
        "            ArtifactAssembly::Runtime(declaration) => Some(declaration),\n"
        "            _ => None,\n"
        "        })\n"
        "        .flat_map(|declaration| declaration.hosted_kinds())\n"
        "        .map(|kind| (kind.id, kind.schema))\n"
        "        .collect::<BTreeSet<_>>();\n"
        "    for package in packages() {\n"
        "        let hosted = package.descriptor.manifest.hosted_artifact_kinds.iter().map(|kind| (kind.id.clone(), kind.schema.clone())).collect::<BTreeSet<_>>();\n"
        '        if package.id == "stdio" {\n'
        '            assert!(hosted.is_empty(), "stdio owns its kinds and hosts none: {hosted:?}");\n'
        "            continue;\n"
        "        }\n"
        '        assert!(package.descriptor.manifest.artifact_kinds.is_empty(), "{} owns no artifact kind", package.id);\n'
        '        assert!(hosted.is_subset(&owner_codecs), "{} hosts rows its owner declares no codec for: {:?}", package.id, hosted.difference(&owner_codecs).collect::<Vec<_>>());\n'
        "        let hosted_ids = hosted.iter().map(|(id, _)| id.as_str()).collect::<BTreeSet<_>>();\n"
        "        for kind in activated_kinds(&package.descriptor) {\n"
        '            assert!(hosted_ids.contains(kind.as_str()), "{} opens {kind} without hosting it", package.id);\n'
        "        }\n"
        "    }\n"
        "}\n"
    )
#endregion Law


def plan():
    edits = {MANIFEST: manifest, PROJECTION: projection, GENERATED: generated, TYPEGEN_LAW: typegen_law, SDK: sdk, BUILDER: builder, HUB: hub, HUB_TESTS: hub_tests, HUB_FIXTURE: hub_fixture, HUB_SCRIPT: lambda text: selection_rule(hub_script(text)), LAW: law, PREFLIGHT_FIXTURE: preflight_fixture, PREFLIGHT_LAW: preflight_law}
    for path in literal_files():
        previous = edits.get(path)
        edits[path] = (lambda text, path=path, previous=previous: manifest_literals(previous(text) if previous else text, path))
    return edits


def main():
    mode = next((flag for flag in ("--dry-run", "--write", "--revert") if flag in sys.argv), None)
    if mode is None:
        print(__doc__)
        sys.exit(2)
    edits = plan()
    if mode == "--revert":
        for path in edits:
            source = os.path.join(BACKUP, path)
            if os.path.isfile(source):
                shutil.copyfile(source, os.path.join(TREE, path))
                print("restored", path)
        return
    staged = {}
    for path, edit in edits.items():
        before = open(os.path.join(TREE, path), encoding="utf-8").read()
        after = edit(before)
        if after == before:
            problems.append(f"{path}: unchanged")
        staged[path] = (before, after)
    for problem in problems:
        print("PROBLEM", problem)
    print(f"{len(staged)} files, {len(problems)} problems")
    if mode == "--write" and not problems:
        for path, (before, after) in staged.items():
            backup = os.path.join(BACKUP, path)
            os.makedirs(os.path.dirname(backup), exist_ok=True)
            if not os.path.exists(backup):
                open(backup, "w", encoding="utf-8").write(before)
            open(os.path.join(TREE, path), "w", encoding="utf-8").write(after)
        print("written; backups under", BACKUP)
    sys.exit(1 if problems else 0)


if __name__ == "__main__":
    main()
