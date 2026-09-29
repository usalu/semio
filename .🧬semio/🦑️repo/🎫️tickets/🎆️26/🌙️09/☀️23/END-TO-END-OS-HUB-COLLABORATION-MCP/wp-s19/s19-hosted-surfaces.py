#!/usr/bin/env python3
"""🏠️ S19 T6 row 3b (REQUIRES LB2 row 3 — p9 + p15 — written first; anchors are p15's post-state): hosting ≠ owning, end to end.

Measured 2026-09-29 (W4 p34 publish; `s14-w4-catalog-p34-failed-c9-1254`): demonstrator's descriptor OWNED `3d.cad`,
`3d.generation`, `3d.process`, `catalogue.sourcing` and `s.gis.gismap` — the app-level kind specs of the cad/procedural/process/
sourcing/gis editors it embeds — so it published native codec rows beside the owners' and every hub creation of those kinds
was ambiguous ("creation kind has no unambiguous verified editor", 409). p15's hosting model could not host them: it derived a
hosted kind's owner from the `s.<owner>.` prefix (these kinds are `3d.cad` …), counted hosted editors in creation (still two
editors), and paired hosted kinds only by dialect (`s.cad.cad` ≠ `3d.cad`). This set:

- descriptor (schema-first): `HostedArtifactKind { id, schema, owner }` — the owner is explicit, never inferred from the id.
- SDK: `ArtifactDeclaration::hosted_kinds` names its owner (the declaration's canonical kind plugin); `host_foreign_surface_kinds`
  turns every surface of another plugin's artifact (dialect `s.<owner>.<artifact>`, owner a declared dependency — the existing
  surface-dependency gate) into hosted rows: its kind specs leave the app and become `hostedArtifactKinds` naming that owner;
  `describe`'s kind-identity law accepts a presented kind the package hosts.
- hub (Rust): manifest hosted rows name a declared dependency present in the catalog; a hosted open target binds the ONE declared
  dependency that declares its exact codec, and at load that package must be the row's explicit owner; a hosted kind opens in
  the surfaces whose dialect names it OR whose io presents it (and the viewers of a presenting editor's dialect); creation is
  owner-preferred (`owner_preferred_creation`: the owners' editors decide, hosts' editors only for a kind no owner creates).
- hub publication script + preflight: owner = the row's `owner`; the browser twin `surfaceOpensArtifactKindV1` and the store
  worker's verification read hosted rows and each surface's presented kind.
- laws + fixtures: shared `🎯️descriptor-open-targets` + `🗂️surface-opens-kind` hosted cases (Rust + TS replay), preflight cases
  (a demonstrator-shaped selection has 0 collisions; stdio rows carry `owner: stdio`), hub `open_target_codec_package` and
  creation-tier laws, stdio shipped-fleet rows carry owner `stdio`, demonstrator assembly law (owns only its playground, hosts
  exactly the 8 embedded kinds, each from its surface's owner).

usage: python3 s19-hosted-surfaces.py --dry-run | --write | --revert [--root <tree>]
Backups (byte-exact, per root) under `.🧬semio/🌐hub/s14-s19-backup/hosted-surfaces/<root-hash>/`.
"""
import hashlib, json, os, shutil, sys

TREE = sys.argv[sys.argv.index("--root") + 1] if "--root" in sys.argv else "/Users/ueli/Documents/semio"
BACKUP = f"/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s19-backup/hosted-surfaces/{hashlib.sha256(TREE.encode()).hexdigest()[:12]}"
MANIFEST = "🧰️framework/🔨️modules/🛂️manifest/🦀️.rs"
PROJECTION = "🧰️framework/🔨️modules/🧬️schema/📽️projection/🦀️.rs"
GENERATED = "🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts"
SDK = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
BUILDER = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️.rs"
DESCRIBE = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs"
HUB = "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs"
HUB_TESTS = "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🔬️unit/🦀️.rs"
OPEN_TARGETS = "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🎯️descriptor-open-targets/🔣️.json"
SURFACE_OPENS = "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🗂️surface-opens-kind/🔣️.json"
PREFLIGHT = "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🛫️catalog-selection-preflight/🔣️.json"
HUB_SCRIPT = "🌎️hub/📦️packages/🦀️rust/📜️script.ts"
TWIN = "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts"
WORKER = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts"
TWIN_LAW = "🧰️framework/🛍️products/💻️os/🧪️tests/🗂️surface-opens-kind/🟦️.ts"
FLEET = "✏️s/🔌️plugins/🗄️stdio/🧪️tests/🚢️shipped-fleet/🦀️.rs"
DEMONSTRATOR_LAW = "✏️s/🔌️plugins/🎪️demonstrator/🪪️manifest/🎪️demonstrator/🧪️tests/🔬️unit/🦀️.rs"

problems = []


def once(text, old, new, label):
    if new in text:
        return text
    count = text.count(old)
    if count != 1:
        problems.append(f"{label}: expected 1 anchor, found {count}")
        return text
    return text.replace(old, new)


def dumps(document):
    return json.dumps(document, indent=2, ensure_ascii=False) + "\n"


#region Descriptor
OWNER_DOC_OLD = "/// 🏠️ One artifact kind a package hosts — its owner's kind id and one document schema of the owner's codecs (see\n/// `PluginManifest::hosted_artifact_kinds`). The owner is the kind's plugin (`s.<owner>.<artifact>`).\n"
OWNER_DOC_NEW = (
    "/// 🏠️ One artifact kind a package hosts — the kind id, one document schema of its owner's codecs, and the OWNER: the plugin\n"
    "/// that owns the kind (declares it and its codec), a declared dependency of the host (see `PluginManifest::hosted_artifact_kinds`).\n"
    "/// The owner is explicit, never read off the kind id — an embedded surface's kind (`3d.cad`) names no plugin.\n"
)
HOSTED_FIELD_DOC_OLD = "schema its owner's codecs decode (`PluginBuilder::host_artifact`). Distinct from `artifact_kinds`: the codec rows stay\n"
HOSTED_FIELD_DOC_NEW = "schema its owner's codecs decode (`PluginBuilder::host_artifact`, or a surface of another plugin's artifact the plugin\n/// registers). Distinct from `artifact_kinds`: the codec rows stay\n"
TS_OWNER_DOC_OLD = " * 🏠️ One artifact kind a package hosts — its owner's kind id and one document schema of the owner's codecs (see\n * `PluginManifest::hosted_artifact_kinds`). The owner is the kind's plugin (`s.<owner>.<artifact>`).\n */\nexport type HostedArtifactKind = { id: string, schema: string, };"
TS_OWNER_DOC_NEW = (
    " * 🏠️ One artifact kind a package hosts — the kind id, one document schema of its owner's codecs, and the OWNER: the plugin\n"
    " * that owns the kind (declares it and its codec), a declared dependency of the host (see `PluginManifest::hosted_artifact_kinds`).\n"
    " * The owner is explicit, never read off the kind id — an embedded surface's kind (`3d.cad`) names no plugin.\n"
    " */\n"
    "export type HostedArtifactKind = { id: string, schema: string, owner: string, };"
)
TS_FIELD_DOC_OLD = " * schema its owner's codecs decode (`PluginBuilder::host_artifact`). Distinct from `artifact_kinds`: the codec rows stay\n"
TS_FIELD_DOC_NEW = " * schema its owner's codecs decode (`PluginBuilder::host_artifact`, or a surface of another plugin's artifact the plugin\n * registers). Distinct from `artifact_kinds`: the codec rows stay\n"


def manifest(text):
    text = once(text, OWNER_DOC_OLD, OWNER_DOC_NEW, "manifest: HostedArtifactKind doc")
    text = once(text, "pub struct HostedArtifactKind {\n    pub id: String,\n    pub schema: String,\n}\n", "pub struct HostedArtifactKind {\n    pub id: String,\n    pub schema: String,\n    pub owner: String,\n}\n", "manifest: owner field")
    return once(text, "    /// " + HOSTED_FIELD_DOC_OLD, "    /// " + HOSTED_FIELD_DOC_NEW.replace("\n/// registers", "\n    /// registers"), "manifest: field doc")


def typescript(text, label):
    text = once(text, TS_OWNER_DOC_OLD, TS_OWNER_DOC_NEW, f"{label}: HostedArtifactKind")
    return once(text, TS_FIELD_DOC_OLD, TS_FIELD_DOC_NEW, f"{label}: field doc")
#endregion Descriptor


#region Sdk
def sdk(text):
    text = once(
        text,
        "        /// 🏠️ The descriptor rows a package hosting this declaration opens: its kind with each document schema its codecs\n"
        "        /// decode — see [`semio_framework::HostedArtifactKind`].\n"
        "        pub fn hosted_kinds(&self) -> Vec<semio_framework::HostedArtifactKind> {\n"
        "            self.document_codecs.iter().map(|codec| semio_framework::HostedArtifactKind { id: self.definition.identity().as_str().to_string(), schema: codec.schema.clone() }).collect()\n"
        "        }\n",
        "        /// 🏠️ The descriptor rows a package hosting this declaration opens: its kind with each document schema its codecs\n"
        "        /// decode, owned by the plugin of its canonical kind (`s.<owner>.<artifact>`, the grammar every declaration carries) —\n"
        "        /// see [`semio_framework::HostedArtifactKind`].\n"
        "        pub fn hosted_kinds(&self) -> Result<Vec<semio_framework::HostedArtifactKind>, PluginAssemblyError> {\n"
        "            let kind = ArtifactKindId::parse(self.kind.as_str()).map_err(|_| PluginAssemblyError::new(\"plugin-assembly.artifact-kind\", \"artifact identity must use canonical s.<plugin>.<artifact> grammar\"))?;\n"
        "            Ok(self.document_codecs.iter().map(|codec| semio_framework::HostedArtifactKind { id: self.definition.identity().as_str().to_string(), schema: codec.schema.clone(), owner: kind.plugin().to_string() }).collect())\n"
        "        }\n",
        "sdk: hosted_kinds owner",
    )
    return once(
        text,
        "            breaches.push(format!(\"app `{}` contributes a surface for `{}` owned by `{owner}`, which `{}` does not declare as a dependency\", app.id, app.dialect.to_coordinate(), manifest.plugin_id));\n"
        "        }\n"
        "        breaches\n"
        "    }\n",
        "            breaches.push(format!(\"app `{}` contributes a surface for `{}` owned by `{owner}`, which `{}` does not declare as a dependency\", app.id, app.dialect.to_coordinate(), manifest.plugin_id));\n"
        "        }\n"
        "        breaches\n"
        "    }\n"
        "\n"
        "    /// 🏠️ A surface of an artifact another plugin owns (its dialect's `s.<owner>.<artifact>`, the grammar\n"
        "    /// [`surface_dependency_breaches`] reads) opens that owner's documents — HOSTING, never owning: the kind specs it carries\n"
        "    /// leave the app and become `hosted_artifact_kinds` rows naming that owner, so no codec row, creation claim or kind\n"
        "    /// ownership of the owner moves into the host (demonstrator embedding the cad/gis/procedural/process/sourcing editors\n"
        "    /// re-owned their kinds and made every hub creation of them ambiguous, 2026-09-29). Runs after the surface dependency\n"
        "    /// gate, so every owner named here is a declared dependency.\n"
        "    pub(crate) fn host_foreign_surface_kinds(manifest: &mut PluginManifest) {\n"
        "        let owned: BTreeSet<String> = manifest.artifact_kinds.iter().map(|spec| spec.id.clone()).collect();\n"
        "        for app in &mut manifest.apps {\n"
        "            if owned.contains(&app.dialect.artifact_kind) {\n"
        "                continue;\n"
        "            }\n"
        "            let Ok(kind) = ArtifactKindId::parse(&app.dialect.artifact_kind) else { continue };\n"
        "            if kind.plugin() == manifest.plugin_id {\n"
        "                continue;\n"
        "            }\n"
        "            for spec in std::mem::take(&mut app.artifact_kinds) {\n"
        "                if !manifest.hosted_artifact_kinds.iter().any(|row| row.id == spec.id && row.schema == spec.schema) {\n"
        "                    manifest.hosted_artifact_kinds.push(semio_framework::HostedArtifactKind { id: spec.id, schema: spec.schema, owner: kind.plugin().to_string() });\n"
        "                }\n"
        "            }\n"
        "        }\n"
        "    }\n",
        "sdk: host_foreign_surface_kinds",
    )


def builder(text):
    text = once(
        text,
        "        plugin.manifest.hosted_artifact_kinds = hosted_artifacts.iter().flat_map(ArtifactDeclaration::hosted_kinds).collect();\n",
        "        for declaration in &hosted_artifacts {\n"
        "            let rows = declaration.hosted_kinds()?;\n"
        "            plugin.manifest.hosted_artifact_kinds.extend(rows);\n"
        "        }\n",
        "builder: declaration rows",
    )
    return once(
        text,
        "        if let Some(breach) = crate::app::surface_dependency_breaches(&plugin.manifest).into_iter().next() {\n"
        "            return Err(PluginAssemblyError::new(\"plugin-assembly.surface-dependency-gate\", breach));\n"
        "        }\n",
        "        if let Some(breach) = crate::app::surface_dependency_breaches(&plugin.manifest).into_iter().next() {\n"
        "            return Err(PluginAssemblyError::new(\"plugin-assembly.surface-dependency-gate\", breach));\n"
        "        }\n"
        "        crate::app::host_foreign_surface_kinds(&mut plugin.manifest);\n",
        "builder: surface hosting",
    )


def describe(text):
    return once(
        text,
        "            declared: app.artifact_kinds.iter().chain(manifest.artifact_kinds.iter()).map(|kind| (kind.id.as_str(), kind.schema.as_str())).collect(),\n",
        "            declared: app.artifact_kinds.iter().chain(manifest.artifact_kinds.iter()).map(|kind| (kind.id.as_str(), kind.schema.as_str())).chain(manifest.hosted_artifact_kinds.iter().map(|kind| (kind.id.as_str(), kind.schema.as_str()))).collect(),\n",
        "describe: hosted kinds are declared",
    )
#endregion Sdk


#region Hub
def hub(text):
    text = once(
        text,
        "    /// 🌱️ Creation resolves one unambiguous writable target in this exact admitted generation.\n"
        "    pub(crate) fn artifact_creation_selection(&self, kind_id: &str) -> Option<&VerifiedDocumentOpenSelectionV1> {\n"
        "        let mut matches = self.open_targets.iter().filter(|selection| {\n"
        "            selection.artifact.kind == kind_id\n"
        "                && selection.surface.role == DocumentOpenSurfaceRoleV1::Editor\n"
        "                && self.codecs.iter().any(|codec| {\n"
        "                    let own = codec.identity.plugin_id == selection.package.plugin_id\n"
        "                        && codec.identity.package_id == selection.package.package_id\n"
        "                        && codec.identity.version == selection.package.version\n"
        "                        && codec.identity.package_hash == selection.package.component_sha256;\n"
        "                    let hosted = hosted_kind_owner(&selection.artifact.kind) == Some(codec.identity.plugin_id.as_str())\n"
        "                        && self.packages.iter().any(|package| package.plugin_id == selection.package.plugin_id && package.dependencies.contains(&codec.identity.plugin_id));\n"
        "                    (own || hosted)\n"
        "                        && codec.identity.artifact_kind == selection.artifact.kind\n"
        "                        && codec.identity.artifact_schema == selection.artifact.schema\n"
        "                        && codec.identity.pack_schema_hash == selection.artifact.pack_schema_hash\n"
        "                })\n"
        "        });\n"
        "        let selected = matches.next()?;\n"
        "        matches.next().is_none().then_some(selected)\n"
        "    }\n",
        "    /// 🌱️ Creation resolves one unambiguous writable target in this exact admitted generation, OWNER-PREFERRED\n"
        "    /// ([`owner_preferred_creation`]): the editors whose own package declares the kind's exact codec decide; only a kind no\n"
        "    /// owner's editor creates falls to the editors of hosts whose declared dependency declares it. Hosting opens documents,\n"
        "    /// it never re-owns their creation (demonstrator's embedded cad/gis/procedural/process editors made every creation of\n"
        "    /// those kinds ambiguous, hub 7800 p34, 2026-09-29).\n"
        "    pub(crate) fn artifact_creation_selection(&self, kind_id: &str) -> Option<&VerifiedDocumentOpenSelectionV1> {\n"
        "        let binds = |selection: &VerifiedDocumentOpenSelectionV1, owned: bool| {\n"
        "            self.codecs.iter().any(|codec| {\n"
        "                let own = codec.identity.plugin_id == selection.package.plugin_id\n"
        "                    && codec.identity.package_id == selection.package.package_id\n"
        "                    && codec.identity.version == selection.package.version\n"
        "                    && codec.identity.package_hash == selection.package.component_sha256;\n"
        "                let hosted = codec.identity.plugin_id != selection.package.plugin_id\n"
        "                    && self.packages.iter().any(|package| package.plugin_id == selection.package.plugin_id && package.dependencies.contains(&codec.identity.plugin_id));\n"
        "                (if owned { own } else { hosted })\n"
        "                    && codec.identity.artifact_kind == selection.artifact.kind\n"
        "                    && codec.identity.artifact_schema == selection.artifact.schema\n"
        "                    && codec.identity.pack_schema_hash == selection.artifact.pack_schema_hash\n"
        "            })\n"
        "        };\n"
        "        let editors = || self.open_targets.iter().filter(|selection| selection.artifact.kind == kind_id && selection.surface.role == DocumentOpenSurfaceRoleV1::Editor);\n"
        "        owner_preferred_creation(editors().filter(|selection| binds(selection, true)), editors().filter(|selection| binds(selection, false)))\n"
        "    }\n",
        "hub: creation selection",
    )
    text = once(
        text,
        "/// and by the viewers of that editor's dialect, the read-only surface of the same documents. A HOSTED kind\n"
        "/// (`PluginManifest::hosted_artifact_kinds`, `PluginBuilder::host_artifact`) follows the plugin-level rule: another\n"
        "/// package's documents open in the host surfaces whose dialect names the kind.\n"
        "fn app_opens_kind(descriptor: &PackageDescriptor, app: &semio_framework::AppDefinition, artifact_kind: &str, artifact_schema: &str) -> bool {\n"
        "    let declares = |kinds: &[semio_framework::ArtifactKindSpec]| kinds.iter().any(|kind| kind.id == artifact_kind && kind.schema == artifact_schema);\n"
        "    let hosts = descriptor.manifest.hosted_artifact_kinds.iter().any(|kind| kind.id == artifact_kind && kind.schema == artifact_schema);\n"
        "    if declares(&descriptor.manifest.artifact_kinds) || hosts {\n"
        "        return app.dialect.artifact_kind == artifact_kind;\n"
        "    }\n",
        "/// and by the viewers of that editor's dialect, the read-only surface of the same documents. A HOSTED kind\n"
        "/// (`PluginManifest::hosted_artifact_kinds`) opens in the host surfaces whose dialect names it (a family hosting its owner's\n"
        "/// kind, `PluginBuilder::host_artifact`) or whose io presents it — an embedded editor of another plugin's artifact\n"
        "/// (demonstrator's cad editor presents `3d.cad`) — and in the viewers of a presenting editor's dialect.\n"
        "fn app_opens_kind(descriptor: &PackageDescriptor, app: &semio_framework::AppDefinition, artifact_kind: &str, artifact_schema: &str) -> bool {\n"
        "    let declares = |kinds: &[semio_framework::ArtifactKindSpec]| kinds.iter().any(|kind| kind.id == artifact_kind && kind.schema == artifact_schema);\n"
        "    if declares(&descriptor.manifest.artifact_kinds) {\n"
        "        return app.dialect.artifact_kind == artifact_kind;\n"
        "    }\n"
        "    if descriptor.manifest.hosted_artifact_kinds.iter().any(|kind| kind.id == artifact_kind && kind.schema == artifact_schema) {\n"
        "        let presents = |candidate: &semio_framework::AppDefinition| candidate.io.artifact.id == artifact_kind && candidate.io.artifact_schema == artifact_schema;\n"
        "        return app.dialect.artifact_kind == artifact_kind\n"
        "            || presents(app)\n"
        "            || (app.role == semio_framework::AppRole::Viewer && descriptor.manifest.apps.iter().any(|editor| editor.role == semio_framework::AppRole::Editor && editor.dialect == app.dialect && presents(editor)));\n"
        "    }\n",
        "hub: app_opens_kind",
    )
    text = once(
        text,
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
        "}\n",
        "/// 🏠️ The package whose native codec one document-open target of `host` binds: `host` itself when it declares that exact\n"
        "/// codec, else — for a kind `host` hosts — the ONE declared dependency of `host` in this catalog that declares the exact\n"
        "/// codec (the load then holds it to the hosted row's explicit owner). Codec rows never move: a host carries none for the\n"
        "/// kinds it hosts, and no owner is ever read off a kind id.\n"
        "fn open_target_codec_package<'a>(packages: &'a [TrustedBundlePackageV1], host: &'a TrustedBundlePackageV1, target: &TrustedBundleOpenTargetV1) -> Result<&'a TrustedBundlePackageV1, AuthorityError> {\n"
        "    let binds = |package: &TrustedBundlePackageV1| package.native_codecs.iter().any(|codec| codec.artifact_kind == target.artifact_kind && codec.artifact_schema == target.artifact_schema && codec.pack_schema_hash == target.pack_schema_hash);\n"
        "    if binds(host) {\n"
        "        return Ok(host);\n"
        "    }\n"
        "    let mut owners = packages.iter().filter(|package| package.plugin_id != host.plugin_id && host.dependencies.iter().any(|dependency| dependency.plugin_id == package.plugin_id) && binds(package));\n"
        "    let owner = owners.next().ok_or_else(|| catalog(\"trusted document-open target binds no exact native codec of its own package or of a declared dependency in the catalog\"))?;\n"
        "    if owners.next().is_some() {\n"
        "        return Err(catalog(\"trusted document-open target binds an exact native codec more than one declared dependency declares\"));\n"
        "    }\n"
        "    Ok(owner)\n"
        "}\n"
        "\n"
        "/// 🌱️ The owner-preferred creation rule over two candidate tiers — the owners' editors, then the hosts' editors: the first\n"
        "/// non-empty tier decides and must hold exactly one candidate, so a host never makes an owner's kind ambiguous and two owners\n"
        "/// never pick one.\n"
        "fn owner_preferred_creation<T>(owned: impl Iterator<Item = T>, hosted: impl Iterator<Item = T>) -> Option<T> {\n"
        "    fn unique<T>(mut candidates: impl Iterator<Item = T>) -> Option<T> {\n"
        "        let first = candidates.next()?;\n"
        "        candidates.next().is_none().then_some(first)\n"
        "    }\n"
        "    let mut owned = owned.peekable();\n"
        "    if owned.peek().is_some() { unique(owned) } else { unique(hosted) }\n"
        "}\n",
        "hub: open_target_codec_package + owner_preferred_creation",
    )
    text = once(
        text,
        "                open_target_codec_package(&bundle.packages, record, target)?;\n                let role = match target.role {\n",
        "                let bound = open_target_codec_package(&bundle.packages, record, target)?;\n"
        "                if bound.plugin_id != record.plugin_id && !descriptor.manifest.hosted_artifact_kinds.iter().any(|kind| kind.id == target.artifact_kind && kind.schema == target.artifact_schema && kind.owner == bound.plugin_id) {\n"
        "                    return Err(catalog(\"document-open target binds a dependency's codec its descriptor does not host from that owner\"));\n"
        "                }\n"
        "                let role = match target.role {\n",
        "hub: load binding holds the explicit owner",
    )
    return once(
        text,
        "        let owner = hosted_kind_owner(&kind.id).filter(|owner| *owner != record.plugin_id).ok_or_else(|| catalog(\"decoded manifest hosted kind names no other owner package\"))?;\n"
        "        if !manifest_dependencies.contains(owner) || !records.contains_key(owner) {\n",
        "        if kind.owner.is_empty() || kind.owner == record.plugin_id {\n"
        "            return Err(catalog(\"decoded manifest hosted kind names no other owner package\"));\n"
        "        }\n"
        "        if !manifest_dependencies.contains(kind.owner.as_str()) || !records.contains_key(kind.owner.as_str()) {\n",
        "hub: manifest hosted owner",
    )


def hub_tests(text):
    text = once(
        text,
        "            descriptor.manifest.hosted_artifact_kinds.push(semio_framework::HostedArtifactKind { id: kind.id, schema: kind.schema });\n        }\n",
        "            descriptor.manifest.hosted_artifact_kinds.push(semio_framework::HostedArtifactKind { id: kind.id, schema: kind.schema, owner: \"fixture-owner\".into() });\n"
        "        }\n"
        "        if case[\"declaredOn\"] == \"hosted-presented\" {\n"
        "            let kind = descriptor.manifest.artifact_kinds.remove(0);\n"
        "            let editor = descriptor.manifest.apps.iter_mut().find(|app| app.role == semio_framework::AppRole::Editor).expect(\"editor app\");\n"
        "            editor.io.artifact.id = kind.id.clone();\n"
        "            editor.io.artifact_schema = kind.schema.clone();\n"
        "            descriptor.manifest.hosted_artifact_kinds.push(semio_framework::HostedArtifactKind { id: kind.id, schema: kind.schema, owner: \"fixture-owner\".into() });\n"
        "        }\n",
        "hub tests: hosted cases",
    )
    return once(
        text,
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
        "}\n",
        "/// 🏠️ A hosted kind's document-open target binds the ONE declared dependency in the catalog that declares its exact native\n"
        "/// codec — never a copied row, never an owner read off the kind id (demonstrator's `3d.cad` names no plugin): accepted for a\n"
        "/// present declared dependency declaring the exact codec; refused by name when none does, when it is absent or undeclared, and\n"
        "/// when two declared dependencies declare the same exact codec.\n"
        "#[test]\n"
        "fn hosted_open_targets_bind_the_owner_codec_or_are_refused_by_name() {\n"
        "    let fixture = fixture_json();\n"
        "    let bundle: TrustedBundleV1 = serde_json::from_value(fixture[\"bundle\"].clone()).expect(\"bundle\");\n"
        "    let mut host = bundle.packages[0].clone();\n"
        "    let mut owner = bundle.packages[1].clone();\n"
        "    let target = host.open_targets[0].clone();\n"
        "    owner.native_codecs = std::mem::take(&mut host.native_codecs);\n"
        "    host.dependencies[0].plugin_id = owner.plugin_id.clone();\n"
        "    let packages = vec![host.clone(), owner.clone()];\n"
        "    assert_eq!(open_target_codec_package(&packages, &packages[0], &target).expect(\"hosted target binds the owner codec\").plugin_id, owner.plugin_id);\n"
        "    let refusal = |packages: Vec<TrustedBundlePackageV1>| open_target_codec_package(&packages, &packages[0], &target).expect_err(\"refused\").to_string();\n"
        "    assert!(refusal(vec![host.clone()]).contains(\"binds no exact native codec\"));\n"
        "    let mut stranger = host.clone();\n"
        "    stranger.dependencies.clear();\n"
        "    assert!(refusal(vec![stranger, owner.clone()]).contains(\"binds no exact native codec\"));\n"
        "    let mut bare = owner.clone();\n"
        "    bare.native_codecs.clear();\n"
        "    assert!(refusal(vec![host.clone(), bare]).contains(\"binds no exact native codec\"));\n"
        "    let mut twin = owner.clone();\n"
        "    twin.plugin_id = format!(\"{}-twin\", owner.plugin_id);\n"
        "    let mut both = host.clone();\n"
        "    let mut twin_dependency = both.dependencies[0].clone();\n"
        "    twin_dependency.plugin_id = twin.plugin_id.clone();\n"
        "    both.dependencies.push(twin_dependency);\n"
        "    assert!(refusal(vec![both, owner.clone(), twin]).contains(\"more than one declared dependency\"));\n"
        "    let mut own = host.clone();\n"
        "    own.native_codecs = owner.native_codecs.clone();\n"
        "    assert_eq!(open_target_codec_package(&[own.clone()], &own, &target).expect(\"own codec\").plugin_id, own.plugin_id);\n"
        "}\n"
        "\n"
        "/// 🌱️ LAW: creation is owner-preferred — the owners' editors decide and a host's editor of the same kind never makes it\n"
        "/// ambiguous (cad + demonstrator's embedded cad editor → cad); a kind only hosts create falls to its one host (a stdio\n"
        "/// family); two owners or two hosts alone stay ambiguous; nothing creates nothing.\n"
        "#[test]\n"
        "fn creation_prefers_the_owners_editor_over_a_hosts() {\n"
        "    assert_eq!(owner_preferred_creation([\"cad\"].into_iter(), [\"demonstrator\"].into_iter()), Some(\"cad\"));\n"
        "    assert_eq!(owner_preferred_creation(std::iter::empty(), [\"stdio-pdf\"].into_iter()), Some(\"stdio-pdf\"));\n"
        "    assert_eq!(owner_preferred_creation([\"cad\", \"cad-twin\"].into_iter(), [\"demonstrator\"].into_iter()), None);\n"
        "    assert_eq!(owner_preferred_creation(std::iter::empty(), [\"demonstrator\", \"bundle\"].into_iter()), None);\n"
        "    assert_eq!(owner_preferred_creation(std::iter::empty::<&str>(), std::iter::empty()), None);\n"
        "}\n",
        "hub tests: hosted binding + creation laws",
    )


def open_targets_fixture(text):
    document = json.loads(text)
    names = {case["name"] for case in document["cases"]}
    template = next(case for case in document["cases"] if case["name"] == "hosted-kind-named-by-the-host-dialect")
    if "hosted-kind-presented-by-the-host-editor" not in names:
        document["cases"].append({**template, "name": "hosted-kind-presented-by-the-host-editor", "kindId": "3d.fixture", "declaredOn": "hosted-presented"})
    if "hosted-kind-neither-named-nor-presented" not in names:
        document["cases"].append({**template, "name": "hosted-kind-neither-named-nor-presented", "kindId": "3d.fixture", "declaredOn": "hosted", "targets": []})
    return dumps(document)


def surface_opens_fixture(text):
    document = json.loads(text)
    document["why"] = (
        "The one surface <-> artifact-kind pairing rule (hub `app_opens_kind`, browser `surfaceOpensArtifactKindV1`): a plugin-level kind is "
        "opened only by the surfaces whose own dialect names it, even where a sibling app lists it as an input; a HOSTED kind (the package's "
        "`hostedArtifactKinds`) by the surfaces whose dialect names it or whose io presents it, and by the viewers of a presenting editor's "
        "exact dialect; any other kind by the editor that declares it itself and by the viewers of that editor's exact dialect (kind, "
        "standard, subset); never a kind with another schema. `app` indexes `apps`, the descriptor's surface apps; `presents` is the kind an "
        "app's io presents (null when it presents none)."
    )
    rebuilt = []
    for case in document["cases"]:
        if "hostedArtifactKinds" not in case:
            ordered = {}
            for key, value in case.items():
                ordered[key] = value
                if key == "pluginArtifactKinds":
                    ordered["hostedArtifactKinds"] = []
            case = ordered
        case["apps"] = [{**app, "presents": app.get("presents")} for app in case["apps"]]
        rebuilt.append(case)
    names = {case["name"] for case in rebuilt}
    cad = {"artifactKind": "s.cad.cad", "standard": "1", "subset": "*"}
    pdf = {"artifactKind": "s.stdio.pdf", "standard": "1.4", "subset": "*"}
    playground = {"artifactKind": "s.demonstrator.playground", "standard": "1", "subset": "*"}
    hosted_cad = [{"id": "3d.cad", "schema": "cad.scene"}]
    for case in [
        {"name": "a hosted kind opens in the host surface whose dialect names it (a family hosting its owner's kind)", "pluginArtifactKinds": [], "hostedArtifactKinds": [{"id": "s.stdio.pdf", "schema": "stdio.pdf"}], "apps": [{"role": "editor", "dialect": pdf, "artifactKinds": [], "presents": None}], "app": 0, "artifact": {"kind": "s.stdio.pdf", "schema": "stdio.pdf"}, "opens": True},
        {"name": "a hosted kind opens in the embedded editor that presents it (demonstrator's cad editor)", "pluginArtifactKinds": [], "hostedArtifactKinds": hosted_cad, "apps": [{"role": "editor", "dialect": cad, "artifactKinds": [], "presents": {"id": "3d.cad", "schema": "cad.scene"}}], "app": 0, "artifact": {"kind": "3d.cad", "schema": "cad.scene"}, "opens": True},
        {"name": "a hosted kind opens in the viewer of the presenting editor's exact dialect", "pluginArtifactKinds": [], "hostedArtifactKinds": hosted_cad, "apps": [{"role": "editor", "dialect": cad, "artifactKinds": [], "presents": {"id": "3d.cad", "schema": "cad.scene"}}, {"role": "viewer", "dialect": cad, "artifactKinds": [], "presents": None}], "app": 1, "artifact": {"kind": "3d.cad", "schema": "cad.scene"}, "opens": True},
        {"name": "a hosted kind never opens in a sibling surface that neither names nor presents it", "pluginArtifactKinds": [], "hostedArtifactKinds": hosted_cad, "apps": [{"role": "editor", "dialect": cad, "artifactKinds": [], "presents": {"id": "3d.cad", "schema": "cad.scene"}}, {"role": "editor", "dialect": playground, "artifactKinds": [{"id": "playground.document", "schema": "playground.playground"}], "presents": {"id": "playground.document", "schema": "playground.playground"}}], "app": 1, "artifact": {"kind": "3d.cad", "schema": "cad.scene"}, "opens": False},
        {"name": "a presented kind the package does not host is never opened by presentation alone", "pluginArtifactKinds": [], "hostedArtifactKinds": [], "apps": [{"role": "editor", "dialect": cad, "artifactKinds": [], "presents": {"id": "3d.cad", "schema": "cad.scene"}}], "app": 0, "artifact": {"kind": "3d.cad", "schema": "cad.scene"}, "opens": False},
        {"name": "a hosted kind of another schema is never opened by presentation", "pluginArtifactKinds": [], "hostedArtifactKinds": hosted_cad, "apps": [{"role": "editor", "dialect": cad, "artifactKinds": [], "presents": {"id": "3d.cad", "schema": "cad.scene"}}], "app": 0, "artifact": {"kind": "3d.cad", "schema": "cad.other"}, "opens": False},
    ]:
        if case["name"] not in names:
            rebuilt.append(case)
    document["cases"] = rebuilt
    return dumps(document)


def preflight_fixture(text):
    document = json.loads(text)
    for case in document["cases"]:
        for package in case["packages"]:
            for row in package["descriptor"]["manifest"].get("hostedArtifactKinds", []):
                row.setdefault("owner", row["id"].split(".")[1])
    ids = {case["id"] for case in document["cases"]}
    cad = {
        "pluginId": "cad", "componentPackageId": "semio:cad", "outputName": "semio_s_plugin_cad.wasm", "linkedCodecRegistry": None, "linkedCodecRegistryPresent": False,
        "descriptor": {"manifest": {"pluginId": "cad", "apps": [
            {"id": "s.cad.cad@1/*#editor", "role": "editor", "dialect": {"artifactKind": "s.cad.cad"}, "io": {"artifactSchema": "cad.scene"}, "artifactKinds": [{"id": "3d.cad", "schema": "cad.scene"}]},
            {"id": "s.cad.cad@1/*#viewer", "role": "viewer", "dialect": {"artifactKind": "s.cad.cad"}, "io": {"artifactSchema": "cad.scene"}, "artifactKinds": []},
        ]}},
    }

    def demonstrator(owned_surface, owner):
        manifest = {"pluginId": "demonstrator", "apps": [
            {"id": "s.demonstrator.playground@1/*#editor", "role": "editor", "dialect": {"artifactKind": "s.demonstrator.playground"}, "io": {"artifactSchema": "playground.playground"}, "artifactKinds": [{"id": "playground.document", "schema": "playground.playground"}]},
            {"id": "s.cad.cad@1/*#editor", "role": "editor", "dialect": {"artifactKind": "s.cad.cad"}, "io": {"artifactSchema": "cad.scene"}, "artifactKinds": [{"id": "3d.cad", "schema": "cad.scene"}] if owned_surface else []},
        ], "dependencies": [{"pluginId": "cad", "version": "=0.1.0"}]}
        if not owned_surface:
            manifest["hostedArtifactKinds"] = [{"id": "3d.cad", "schema": "cad.scene", "owner": owner}]
        return {"pluginId": "demonstrator", "componentPackageId": "semio:demonstrator", "outputName": "semio_s_plugin_demonstrator.wasm", "linkedCodecRegistry": None, "linkedCodecRegistryPresent": False, "descriptor": {"manifest": manifest}}

    for case in [
        {"id": "hosted-surface-kind-of-a-selected-owner-accepted", "profileId": "local-cad-demonstrator-open-v1", "packages": [cad, demonstrator(False, "cad")], "findings": []},
        {"id": "embedded-surface-re-owning-its-owners-kind-refused", "profileId": "local-cad-demonstrator-open-v1", "packages": [cad, demonstrator(True, "cad")], "findings": ["artifact kind 3d.cad is owned by editors of cad and demonstrator"]},
        {"id": "hosted-surface-kind-naming-itself-owner-refused", "profileId": "local-cad-demonstrator-open-v1", "packages": [cad, demonstrator(False, "demonstrator")], "findings": ["demonstrator: hosts 3d.cad"]},
    ]:
        if case["id"] not in ids:
            document["cases"].append(case)
    return dumps(document)


def hub_script(text):
    text = once(
        text,
        " * or HOSTS kinds (`manifest.hostedArtifactKinds`, p15) — each hosted kind's owner (`s.<owner>.<artifact>`) a declared dependency\n",
        " * or HOSTS kinds (`manifest.hostedArtifactKinds`, p15) — each hosted row's explicit `owner` a declared dependency\n",
        "script: selection rule doc",
    )
    text = once(
        text,
        "      const owner = /^s\\.([^.]+)\\./u.exec(String(kind.id))?.[1];\n",
        "      const owner = typeof kind.owner === \"string\" && kind.owner !== \"\" ? kind.owner : undefined;\n",
        "script: preflight owner",
    )
    text = once(
        text,
        "/** 🏠️ The kinds one package HOSTS (`manifest.hostedArtifactKinds`, `PluginBuilder::host_artifact`): each owner (the kind's\n"
        " * plugin, `s.<owner>.<artifact>`) must be a declared dependency already published in this catalog; a hosted row binds the\n",
        "/** 🏠️ The kinds one package HOSTS (`manifest.hostedArtifactKinds`: `PluginBuilder::host_artifact` and embedded surfaces of\n"
        " * another plugin's artifact): each row's explicit `owner` must be a declared dependency already published in this catalog —\n"
        " * never read off the kind id (an embedded editor's `3d.cad` names no plugin); a hosted row binds the\n",
        "script: hosted kinds doc",
    )
    text = once(
        text,
        "  const pairs = ((manifest.hostedArtifactKinds ?? []) as Record<string, any>[]).map((kind) => [String(kind.id), String(kind.schema)] as const);\n",
        "  const hosted = (manifest.hostedArtifactKinds ?? []) as Record<string, any>[];\n"
        "  const pairs = hosted.map((kind) => [String(kind.id), String(kind.schema)] as const);\n",
        "script: hosted rows",
    )
    return once(
        text,
        "  for (const [id, schema] of pairs) {\n"
        "    const owner = /^s\\.([^.]+)\\./.exec(id)?.[1];\n"
        "    if (!owner || owner === pluginId || !dependencies.has(owner)) throw new Error(`trusted catalog package ${pluginId} hosts ${id}, whose owner is not one of its declared dependencies`);\n",
        "  for (const kind of hosted) {\n"
        "    const [id, schema] = [String(kind.id), String(kind.schema)];\n"
        "    const owner = typeof kind.owner === \"string\" ? kind.owner : \"\";\n"
        "    if (!owner || owner === pluginId || !dependencies.has(owner)) throw new Error(`trusted catalog package ${pluginId} hosts ${id}, whose owner ${JSON.stringify(owner)} is not one of its declared dependencies`);\n",
        "script: hosted owner",
    )
#endregion Hub


#region Browser
def twin(text):
    text = once(
        text,
        "/** 🎯️ One surface app of a verified descriptor as the pairing rule reads it: its role, its full dialect coordinate and the\n"
        " * artifact kinds it declares itself. */\n"
        "export type SurfaceKindAppV1 = {\n"
        "  readonly role: \"editor\" | \"viewer\";\n"
        "  readonly dialect: { readonly artifactKind: string; readonly standard: string; readonly subset: string };\n"
        "  readonly artifactKinds: readonly SurfaceArtifactKindV1[];\n"
        "};\n",
        "/** 🎯️ One surface app of a verified descriptor as the pairing rule reads it: its role, its full dialect coordinate, the\n"
        " * artifact kinds it declares itself and the kind its io presents (`null` when it presents none). */\n"
        "export type SurfaceKindAppV1 = {\n"
        "  readonly role: \"editor\" | \"viewer\";\n"
        "  readonly dialect: { readonly artifactKind: string; readonly standard: string; readonly subset: string };\n"
        "  readonly artifactKinds: readonly SurfaceArtifactKindV1[];\n"
        "  readonly presents: SurfaceArtifactKindV1 | null;\n"
        "};\n",
        "twin: app type",
    )
    text = once(
        text,
        " * lists it as an input; any other kind is opened by the editor that declares it itself (a plugin on the declaration tree\n",
        " * lists it as an input; a HOSTED kind (`hostedArtifactKinds`) by the surfaces whose dialect names it or whose io presents it\n"
        " * and by the viewers of a presenting editor's exact dialect; any other kind is opened by the editor that declares it itself\n"
        " * (a plugin on the declaration tree\n",
        "twin: doc",
    )
    return once(
        text,
        "export function surfaceOpensArtifactKindV1(\n"
        "  pluginArtifactKinds: readonly SurfaceArtifactKindV1[],\n"
        "  apps: readonly SurfaceKindAppV1[],\n"
        "  app: SurfaceKindAppV1,\n"
        "  artifact: { readonly kind: string; readonly schema: string },\n"
        "): boolean {\n"
        "  const declares = (kinds: readonly SurfaceArtifactKindV1[]): boolean => kinds.some((kind) => kind.id === artifact.kind && kind.schema === artifact.schema);\n"
        "  if (declares(pluginArtifactKinds)) return app.dialect.artifactKind === artifact.kind;\n",
        "export function surfaceOpensArtifactKindV1(\n"
        "  pluginArtifactKinds: readonly SurfaceArtifactKindV1[],\n"
        "  hostedArtifactKinds: readonly SurfaceArtifactKindV1[],\n"
        "  apps: readonly SurfaceKindAppV1[],\n"
        "  app: SurfaceKindAppV1,\n"
        "  artifact: { readonly kind: string; readonly schema: string },\n"
        "): boolean {\n"
        "  const declares = (kinds: readonly SurfaceArtifactKindV1[]): boolean => kinds.some((kind) => kind.id === artifact.kind && kind.schema === artifact.schema);\n"
        "  if (declares(pluginArtifactKinds)) return app.dialect.artifactKind === artifact.kind;\n"
        "  if (declares(hostedArtifactKinds)) {\n"
        "    const presents = (candidate: SurfaceKindAppV1): boolean => candidate.presents !== null && candidate.presents.id === artifact.kind && candidate.presents.schema === artifact.schema;\n"
        "    return (\n"
        "      app.dialect.artifactKind === artifact.kind ||\n"
        "      presents(app) ||\n"
        "      (app.role === \"viewer\" &&\n"
        "        apps.some(\n"
        "          (editor) =>\n"
        "            editor.role === \"editor\" &&\n"
        "            editor.dialect.artifactKind === app.dialect.artifactKind &&\n"
        "            editor.dialect.standard === app.dialect.standard &&\n"
        "            editor.dialect.subset === app.dialect.subset &&\n"
        "            presents(editor),\n"
        "        ))\n"
        "    );\n"
        "  }\n",
        "twin: hosted rule",
    )


def worker(text):
    text = once(
        text,
        "  const artifactKinds = Array.isArray(manifest.artifactKinds) ? (manifest.artifactKinds as readonly PackValue[]).map(record) : [];\n",
        "  const artifactKinds = Array.isArray(manifest.artifactKinds) ? (manifest.artifactKinds as readonly PackValue[]).map(record) : [];\n"
        "  const hostedArtifactKinds = Array.isArray(manifest.hostedArtifactKinds) ? (manifest.hostedArtifactKinds as readonly PackValue[]).map(record) : [];\n",
        "worker: hosted rows",
    )
    text = once(
        text,
        "    return { role: entry.role, dialect: { artifactKind, standard, subset }, artifactKinds: kindPairs(Array.isArray(entry.artifactKinds) ? (entry.artifactKinds as readonly PackValue[]).map(record) : []) };\n",
        "    const packRecord = (value: PackValue | undefined): Readonly<Record<string, PackValue>> | null => (value === undefined || value === null || typeof value !== \"object\" || Array.isArray(value) || value instanceof Uint8Array || isPackInteger(value) ? null : (value as Readonly<Record<string, PackValue>>));\n"
        "    const io = packRecord(entry.io);\n"
        "    const presentedId = packRecord(io?.artifact)?.id;\n"
        "    const presentedSchema = io?.artifactSchema;\n"
        "    const presents = typeof presentedId === \"string\" && presentedId !== \"\" && typeof presentedSchema === \"string\" ? { id: presentedId, schema: presentedSchema } : null;\n"
        "    return { role: entry.role, dialect: { artifactKind, standard, subset }, artifactKinds: kindPairs(Array.isArray(entry.artifactKinds) ? (entry.artifactKinds as readonly PackValue[]).map(record) : []), presents };\n",
        "worker: presents",
    )
    return once(
        text,
        "!surfaceOpensArtifactKindV1(kindPairs(artifactKinds), apps.flatMap((entry) => surfaceApp(entry) ?? []), openingApp, fields.artifact)",
        "!surfaceOpensArtifactKindV1(kindPairs(artifactKinds), kindPairs(hostedArtifactKinds), apps.flatMap((entry) => surfaceApp(entry) ?? []), openingApp, fields.artifact)",
        "worker: call",
    )


def twin_law(text):
    text = once(
        text,
        "  readonly pluginArtifactKinds: readonly SurfaceArtifactKindV1[];\n  readonly apps: readonly SurfaceKindAppV1[];\n",
        "  readonly pluginArtifactKinds: readonly SurfaceArtifactKindV1[];\n  readonly hostedArtifactKinds: readonly SurfaceArtifactKindV1[];\n  readonly apps: readonly SurfaceKindAppV1[];\n",
        "twin law: case type",
    )
    text = once(text, "  readonly declaredOn: \"plugin\" | \"editor\" | \"plugin-and-editor\";\n", "  readonly declaredOn: \"plugin\" | \"editor\" | \"plugin-and-editor\" | \"hosted\" | \"hosted-presented\";\n", "twin law: declaredOn")
    text = once(
        text,
        "  it(\"replays the declaration-tree, plugin-level, sibling-dialect and viewer cases\", () => {\n    expect(pairing.cases.length).toBeGreaterThanOrEqual(13);\n",
        "  it(\"replays the declaration-tree, plugin-level, sibling-dialect, viewer and hosted cases\", () => {\n    expect(pairing.cases.length).toBeGreaterThanOrEqual(19);\n    expect(pairing.cases.some((testCase) => testCase.hostedArtifactKinds.length > 0 && testCase.opens)).toBe(true);\n",
        "twin law: counts",
    )
    text = once(
        text,
        "      expect(surfaceOpensArtifactKindV1(testCase.pluginArtifactKinds, testCase.apps, testCase.apps[testCase.app]!, testCase.artifact)).toBe(testCase.opens);\n",
        "      expect(surfaceOpensArtifactKindV1(testCase.pluginArtifactKinds, testCase.hostedArtifactKinds, testCase.apps, testCase.apps[testCase.app]!, testCase.artifact)).toBe(testCase.opens);\n",
        "twin law: pairing call",
    )
    return once(
        text,
        "      const pluginArtifactKinds = testCase.declaredOn === \"editor\" ? [] : [kind];\n"
        "      const apps: SurfaceKindAppV1[] = [\n"
        "        { role: \"editor\", dialect, artifactKinds: testCase.declaredOn === \"plugin\" ? [] : [kind] },\n"
        "        { role: \"viewer\", dialect, artifactKinds: [] },\n"
        "      ];\n"
        "      const opened = testCase.execution !== \"isolated\" ? [] : apps.filter((app) => surfaceOpensArtifactKindV1(pluginArtifactKinds, apps, app, { kind: kind.id, schema })).map((app) => `${dialect.artifactKind}@${dialect.standard}/${dialect.subset}#${app.role}`);\n",
        "      const hosted = testCase.declaredOn === \"hosted\" || testCase.declaredOn === \"hosted-presented\";\n"
        "      const pluginArtifactKinds = testCase.declaredOn === \"plugin\" || testCase.declaredOn === \"plugin-and-editor\" ? [kind] : [];\n"
        "      const hostedArtifactKinds = hosted ? [kind] : [];\n"
        "      const apps: SurfaceKindAppV1[] = [\n"
        "        { role: \"editor\", dialect, artifactKinds: testCase.declaredOn === \"editor\" || testCase.declaredOn === \"plugin-and-editor\" ? [kind] : [], presents: testCase.declaredOn === \"hosted-presented\" ? kind : null },\n"
        "        { role: \"viewer\", dialect, artifactKinds: [], presents: null },\n"
        "      ];\n"
        "      const opened = testCase.execution !== \"isolated\" ? [] : apps.filter((app) => surfaceOpensArtifactKindV1(pluginArtifactKinds, hostedArtifactKinds, apps, app, { kind: kind.id, schema })).map((app) => `${dialect.artifactKind}@${dialect.standard}/${dialect.subset}#${app.role}`);\n",
        "twin law: descriptor model",
    )
#endregion Browser


#region Laws
def fleet(text):
    text = once(
        text,
        "        .flat_map(|declaration| declaration.hosted_kinds())\n        .map(|kind| (kind.id, kind.schema))\n        .collect::<BTreeSet<_>>();\n",
        "        .flat_map(|declaration| declaration.hosted_kinds().expect(\"a runtime declaration names its canonical owner\"))\n        .map(|kind| (kind.id, kind.schema, kind.owner))\n        .collect::<BTreeSet<_>>();\n",
        "fleet: owner codecs",
    )
    text = once(
        text,
        "        let hosted = package.descriptor.manifest.hosted_artifact_kinds.iter().map(|kind| (kind.id.clone(), kind.schema.clone())).collect::<BTreeSet<_>>();\n",
        "        let hosted = package.descriptor.manifest.hosted_artifact_kinds.iter().map(|kind| (kind.id.clone(), kind.schema.clone(), kind.owner.clone())).collect::<BTreeSet<_>>();\n",
        "fleet: hosted rows",
    )
    text = once(
        text,
        "        let hosted_ids = hosted.iter().map(|(id, _)| id.as_str()).collect::<BTreeSet<_>>();\n",
        "        assert!(hosted.iter().all(|(_, _, owner)| owner == \"stdio\"), \"{} hosts rows of an owner other than stdio: {hosted:?}\", package.id);\n"
        "        let hosted_ids = hosted.iter().map(|(id, _, _)| id.as_str()).collect::<BTreeSet<_>>();\n",
        "fleet: owner stdio",
    )
    return once(
        text,
        "/// catalog routes the owner's documents to the family's editors and binds the owner's codec; the owner `stdio` hosts nothing.\n",
        "/// catalog routes the owner's documents to the family's editors and binds the owner's codec; every row names its owner `stdio`\n/// explicitly; the owner `stdio` hosts nothing.\n",
        "fleet: doc",
    )


DEMONSTRATOR_LAW_TEXT = '''
/// 🏠️ LAW: demonstrator HOSTS the kinds of the six plugins whose surfaces it embeds — it owns only its playground, every
/// embedded surface's kind is a hosted row naming that surface's owner (a declared dependency), and every embedded editor
/// presents a kind it hosts — so a trusted catalog with demonstrator and the owners has one creating editor per kind (hub 7800
/// p34, 2026-09-29: `3d.cad`/`3d.generation`/`3d.process`/`s.gis.gismap` creation answered 409).
#[test]
fn demonstrator_hosts_the_kinds_of_every_embedded_surface_and_owns_only_its_playground() {
    let manifest = test_bundle().manifest;
    let owned: std::collections::BTreeSet<&str> = manifest.artifact_kinds.iter().chain(manifest.apps.iter().flat_map(|app| app.artifact_kinds.iter())).map(|kind| kind.id.as_str()).collect();
    assert_eq!(owned, std::collections::BTreeSet::from(["playground.document"]));
    let hosted: std::collections::BTreeSet<(&str, &str, &str)> = manifest.hosted_artifact_kinds.iter().map(|kind| (kind.id.as_str(), kind.schema.as_str(), kind.owner.as_str())).collect();
    assert_eq!(
        hosted,
        std::collections::BTreeSet::from([
            ("3d.generation", "generation.3d", "procedural"),
            ("3d.cad", "cad.scene", "cad"),
            ("3d.puzzle", "puzzle.3d", "puzzle"),
            ("catalogue.sourcing", "sourcing.curation/v1", "sourcing"),
            ("catalogue.kinds", "catalogue.kinds", "sourcing"),
            ("kit.catalog", "kit.catalog", "sourcing"),
            ("3d.process", "process.3d", "process"),
            ("s.gis.gismap", "gis.map", "gis"),
        ])
    );
    let dependencies: std::collections::BTreeSet<&str> = manifest.dependencies.iter().map(|dependency| dependency.plugin_id.as_str()).collect();
    assert!(hosted.iter().all(|(_, _, owner)| dependencies.contains(owner)), "every hosted row names a declared dependency as its owner");
    for app in manifest.apps.iter().filter(|app| app.role == semio_framework::AppRole::Editor && !app.dialect.artifact_kind.starts_with("s.demonstrator.")) {
        assert!(hosted.iter().any(|(id, schema, _)| *id == app.io.artifact.id && *schema == app.io.artifact_schema), "{} presents {} ({}), a kind demonstrator does not host", app.id, app.io.artifact.id, app.io.artifact_schema);
    }
}
'''


def demonstrator_law(text):
    if "fn demonstrator_hosts_the_kinds_of_every_embedded_surface_and_owns_only_its_playground" in text:
        return text
    anchor = "\n#[test]\nfn every_surface_declares_a_artifact_schema() {\n"
    if text.count(anchor) != 1:
        problems.append("demonstrator law: anchor")
        return text
    return text.replace(anchor, DEMONSTRATOR_LAW_TEXT + anchor)
#endregion Laws


EDITS = [
    (MANIFEST, manifest), (PROJECTION, lambda text: typescript(text, "projection")), (GENERATED, lambda text: typescript(text, "generated")),
    (SDK, sdk), (BUILDER, builder), (DESCRIBE, describe), (HUB, hub), (HUB_TESTS, hub_tests), (OPEN_TARGETS, open_targets_fixture),
    (SURFACE_OPENS, surface_opens_fixture), (PREFLIGHT, preflight_fixture), (HUB_SCRIPT, hub_script), (TWIN, twin), (WORKER, worker),
    (TWIN_LAW, twin_law), (FLEET, fleet), (DEMONSTRATOR_LAW, demonstrator_law),
]


def main():
    mode = next((flag for flag in ("--dry-run", "--write", "--revert") if flag in sys.argv), "--dry-run")
    if mode == "--revert":
        for rel, _ in EDITS:
            source = os.path.join(BACKUP, rel)
            if os.path.exists(source):
                shutil.copy2(source, os.path.join(TREE, rel))
                print(f"restored {rel}")
        return 0
    planned = []
    for rel, change in EDITS:
        path = os.path.join(TREE, rel)
        if not os.path.exists(path):
            problems.append(f"{rel}: missing")
            continue
        before = open(path, encoding="utf-8").read()
        after = change(before)
        if after != before:
            planned.append((rel, after))
    print(f"{len(planned)} files, {len(problems)} problems")
    for problem in problems:
        print(f"  - {problem}")
    if problems or mode == "--dry-run":
        return 1 if problems else 0
    for rel, after in planned:
        target = os.path.join(TREE, rel)
        backup = os.path.join(BACKUP, rel)
        if not os.path.exists(backup):
            os.makedirs(os.path.dirname(backup), exist_ok=True)
            shutil.copy2(target, backup)
        open(target, "w", encoding="utf-8").write(after)
    print(f"written; backups under {BACKUP}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
