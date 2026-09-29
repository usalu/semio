#!/usr/bin/env python3
"""🏠️ LB2 p9 (window 3, SDK + stdio): a family package is self-sufficient for the kinds it hosts.

(a) SDK `PluginBuilder::host_artifact(declaration)`: an artifact kind another package owns commits its document schemas,
    inference descriptors, document codecs, composers, formats, subset validators and dialect migrations into THIS guest's
    registries — preflighted (kind owned by a direct dependency, never by the host; channels inside the kind; no kind twice),
    identical rows tolerated, conflicting rows fatal. Hosting is not owning: the definition stays the owner's (never
    registered by the host), inference services stay routed to the owner, `describe` lists composers only for kinds the
    plugin id owns and codec rows only for kinds its apps open — so no descriptor row, W4 codec gate or hub catalog row moves.
(b) the nine family packages host every kind they activate on, plus the kinds whose schema documents those kinds' snapshot
    contracts compose (`$ref`, transitively; measured: office hosts xml — docx/xlsx parts are xml documents).
(c) bmp, wav, epw, binary, ifc, gif and semio become runtime declarations (schema-first `runtime_capabilities` in their
    definitions; `declaration()` next to `definition()`; semio's 19 subsets and ifc's 3 MVD subsets gain `declare()`, the
    declarative twin of their `register()`).
(d) LAW `every_package_hosts_the_runtime_of_every_kind_it_opens_in_its_own_process` (shipped-fleet): each package assembled
    ALONE in a child process; every schema/inference/codec/composer/format/subset-validator requirement of every kind it
    opens is live.
(e) every registered schema document is resolvable by `$id` in the guest that registered its referrer: an artifact declares
    its SHARED schema documents (`ArtifactDeclarationBuilder::schema_documents`, named exports of its own scope, one `schema`
    capability row each claiming `schema-export` `<scope>#<export>` — its own namespace: `s.stdio.semio.child` would read as a
    subset schema id; semio: `base/geometry.json`, `base/child.json`); an inference descriptor registers
    its document as the `inference` export of its scope; the store and io vocabulary register their own documents
    (`os/store/{child,link,blob}.json`, `framework/io/schema.json`) — committed with the declared catalogs of every assembly.
    LAW `every_registered_snapshot_contract_resolves_in_each_package_process`: per package, in its own child process, every
    registered artifact's snapshot contract compiles (every `$ref` resolves).

usage: python3 lb2-p9-hosted-artifacts.py --dry-run | --write | --revert [--root <tree>] [--skip-json]
Backups (byte-exact, per root) under `.🧬semio/🌐hub/s14-lb2-backup/p9/<root-hash>/`.
"""
import glob, hashlib, json, os, re, shutil, sys

TREE = sys.argv[sys.argv.index("--root") + 1] if "--root" in sys.argv else "/Users/ueli/Documents/semio"
HERE = os.path.dirname(os.path.abspath(__file__))
BACKUP = f"/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-lb2-backup/p9/{hashlib.sha256(TREE.encode()).hexdigest()[:12]}"
SKIP_JSON = "--skip-json" in sys.argv
ROWS = os.path.join(HERE, "payload", "p9", "runtime-capabilities.json")

SDK = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
BUILDER = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️.rs"
STDIO = "✏️s/🔌️plugins/🗄️stdio"
ART = f"{STDIO}/🗿️artifacts"
LAW = f"{STDIO}/🧪️tests/🚢️shipped-fleet/🦀️.rs"
STDIO_MANIFEST = f"{STDIO}/📦️packages/🦀️rust/Cargo.toml"
SCHEMA_COMPONENT = "🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs"
KERNEL_MANIFEST = "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml"
IO_SCHEMA = "🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️.rs"
STORE = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"
SEMIO_BASE_SCHEMA = f"{STDIO}/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🦀️.rs"
REGISTRY = "semio_framework_schema_registry"
LOCK = "Cargo.lock"
CONTRACT = f"{STDIO}/📇️registry/🧬️contract/🦀️.rs"
FAMILIES = ["🖼️image", "🎵️media", "🛠️cad", "🏠️bim", "🔺️mesh", "📘️pdf", "💼️office", "🧿️semio", "🔢️binary"]
ROOTS = ["🪟️bmp", "🔊️wav", "🌦️epw", "💾️binary", "🏗️ifc", "🎞️gif", "🧿️semio"]
SEMIO_SUBSETS = f"{ART}/🧿️semio/🏅️standards/🔖️v1/🪆️subsets"
IFC_MVDS = [f"{ART}/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/{s}/🚪️io/🦀️.rs" for s in ["🤝️cv20", "🧮️sav", "🏢️cobie"]]
BUILDER_TYPE = "semio_framework_plugin::app::ArtifactDeclarationBuilder<semio_framework_plugin::app::DeclarationReady>"
R9 = "// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9\n"
DECLARATION_SIGNATURE = "pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {\n"
IMPORTED_DECLARATION_SIGNATURE = "pub fn declaration(definition: ArtifactDefinition) -> Result<ArtifactDeclaration, ArtifactDefinitionError> {\n"

problems = []


def once(text, old, new, label):
    count = text.count(old)
    if count != 1:
        problems.append(f"{label}: expected 1 anchor, found {count}")
        return text
    return text.replace(old, new)


def times(text, old, new, n, label):
    count = text.count(old)
    if count != n:
        problems.append(f"{label}: expected {n} anchors, found {count}")
        return text
    return text.replace(old, new)


#region SDK
def sdk(text):
    old_preflight = re.search(
        r"(        pub\(crate\) fn preflight\(&self, plugin_id: &str, definitions: &mut ArtifactDefinitionRegistry\) -> Result<\(\), PluginAssemblyError> \{\n"
        r"            preflight_artifact_identity\(plugin_id, &self.kind\)\?;\n"
        r"            self.register_definitions\(definitions\).map_err\(PluginAssemblyError::definition\)\?;\n)"
        r"(            for entry in &self.composers \{\n.*?\n            \}\n            for entry in &self.subset_validators \{\n.*?\n            \}\n            for migration in &self.migrations \{\n.*?\n            \}\n)"
        r"(            for service in &self.inference_services \{\n.*?\n            \}\n            Ok\(\(\)\)\n        \}\n)",
        text,
        re.S,
    )
    if not old_preflight:
        problems.append("sdk: preflight anchor")
        return text
    head, channels, services = old_preflight.groups()
    new = (
        head
        + "            self.preflight_channels()?;\n"
        + services
        + "\n"
        + "        /// 🏠️ Validates a declaration another package owns for hosting in `plugin_id`'s guest, without side effects: the\n"
        + "        /// kind is owned by a plugin `plugin_id` directly depends on — never by `plugin_id` itself, which declares its own\n"
        + "        /// kinds — and every channel stays inside it. The definition is not registered: it stays its owner's.\n"
        + "        pub(crate) fn preflight_hosted(&self, plugin_id: &str, dependencies: &[semio_framework::PluginDependency]) -> Result<(), PluginAssemblyError> {\n"
        + "            let kind = ArtifactKindId::parse(self.kind.as_str()).map_err(|_| PluginAssemblyError::new(\"plugin-assembly.artifact-kind\", \"artifact identity must use canonical s.<plugin>.<artifact> grammar\"))?;\n"
        + "            if kind.plugin() == plugin_id {\n"
        + "                return Err(PluginAssemblyError::new(\"plugin-assembly.hosted-artifact-owner\", format!(\"plugin {plugin_id:?} hosts {:?}, a kind it owns and declares\", self.kind)));\n"
        + "            }\n"
        + "            if !dependencies.iter().any(|dependency| dependency.plugin_id == kind.plugin()) {\n"
        + "                return Err(PluginAssemblyError::new(\"plugin-assembly.hosted-artifact-dependency\", format!(\"plugin {plugin_id:?} hosts {:?} without depending on its owner {:?}\", self.kind, kind.plugin())));\n"
        + "            }\n"
        + "            self.preflight_channels()\n"
        + "        }\n"
        + "\n"
        + "        /// 🧭️ Every composer, subset validator, dialect migration and shared schema document of this declaration stays inside its own kind.\n"
        + "        fn preflight_channels(&self) -> Result<(), PluginAssemblyError> {\n"
        + channels
        + "            for documents in &self.schema_documents {\n"
        + "                if documents.scope != self.kind {\n"
        + "                    return Err(PluginAssemblyError::new(\"plugin-assembly.schema-documents-owner\", format!(\"schema documents of scope {:?} declared by artifact {:?}\", documents.scope, self.kind)));\n"
        + "                }\n"
        + "            }\n"
        + "            Ok(())\n"
        + "        }\n"
    )
    text = text[: old_preflight.start()] + new + text[old_preflight.end() :]
    text = once(
        text,
        "        pub(crate) fn from_declarations(\n            declarations: &[ArtifactDeclaration],\n",
        "        pub(crate) fn from_declarations(\n            declarations: &[ArtifactDeclaration],\n            hosted: &[ArtifactDeclaration],\n",
        "sdk: from_declarations signature",
    )
    text = once(
        text,
        "            for declaration in declarations {\n                plan.schemas.extend(declaration.schemas.iter().copied());\n                plan.inferences.extend(declaration.inferences.iter().copied());\n                plan.inference_services.extend(declaration.inference_services.iter().copied());\n",
        "            for declaration in declarations {\n                plan.inference_services.extend(declaration.inference_services.iter().copied());\n            }\n            for declaration in declarations.iter().chain(hosted) {\n                plan.schemas.extend(declaration.schemas.iter().copied());\n                plan.inferences.extend(declaration.inferences.iter().copied());\n",
        "sdk: from_declarations body",
    )
    text = once(
        text,
        "    impl ArtifactRegistrationPlan {\n        pub(crate) fn from_declarations(\n",
        "    impl ArtifactRegistrationPlan {\n        /// 🧾️ Collects every channel of the owned `declarations` and of the `hosted` ones, except the hosted inference\n        /// services: those stay listed and executed by their owner's assembly alone.\n        pub(crate) fn from_declarations(\n",
        "sdk: from_declarations doc",
    )
    return schema_documents_channel(text)


def schema_documents_channel(text):
    text = once(
        text,
        "        /// 📖️ Grammar identity namespace.\n        pub fn grammar() -> Self {\n            Self(\"grammar\".into())\n        }\n",
        "        /// 📖️ Grammar identity namespace.\n        pub fn grammar() -> Self {\n            Self(\"grammar\".into())\n        }\n"
        "\n"
        "        /// 📑️ Shared schema document identity namespace: one named export of the OS-wide schema export registry.\n"
        "        pub fn schema_export() -> Self {\n"
        "            Self(\"schema-export\".into())\n"
        "        }\n",
        "sdk: schema-export namespace",
    )
    text = once(
        text,
        "            Self::new(ArtifactIdentityNamespace::parse(\"codec-extension\")?, format!(\"{}:{schema}:{extension}\", schema.len()))\n        }\n",
        "            Self::new(ArtifactIdentityNamespace::parse(\"codec-extension\")?, format!(\"{}:{schema}:{extension}\", schema.len()))\n        }\n"
        "\n"
        "        /// 🧷️ Claims the named export `export` of `scope` in the OS-wide schema export registry as `<scope>#<export>` — its own\n"
        "        /// namespace, so a shared schema document never reads as (nor collides with) an artifact schema id such as a subset's.\n"
        "        pub fn schema_export(scope: &str, export: &str) -> Result<Self, ArtifactDefinitionError> {\n"
        "            if scope.trim().is_empty() || export.trim().is_empty() || scope.contains('#') || export.contains('#') {\n"
        "                return Err(ArtifactDefinitionError::new(\"artifact-definition.claim\", \"schema export authority requires a scope and an export id without `#`\"));\n"
        "            }\n"
        "            Self::new(ArtifactIdentityNamespace::schema_export(), format!(\"{scope}#{export}\"))\n"
        "        }\n",
        "sdk: schema_export claim",
    )
    field = "        schema_documents: Vec<::semio_framework_schema::ScopeSchemaExports>,\n"
    text = once(text, "        document_codecs: Vec<DocumentCodecSpec>,\n        migrations: Vec<store::DialectMigration>,\n        /// 🧒️🔗️ Pulled", "        document_codecs: Vec<DocumentCodecSpec>,\n        migrations: Vec<store::DialectMigration>,\n" + field + "        /// 🧒️🔗️ Pulled", "sdk: declaration field")
    text = once(text, "        document_codecs: Vec<DocumentCodecSpec>,\n        migrations: Vec<store::DialectMigration>,\n        child_slots: &'static", "        document_codecs: Vec<DocumentCodecSpec>,\n        migrations: Vec<store::DialectMigration>,\n" + field + "        child_slots: &'static", "sdk: builder field")
    text = once(text, "                document_codecs: Vec::new(),\n                migrations: Vec::new(),\n                child_slots: &[],\n", "                document_codecs: Vec::new(),\n                migrations: Vec::new(),\n                schema_documents: Vec::new(),\n                child_slots: &[],\n", "sdk: builder init")
    text = times(text, "                document_codecs: self.document_codecs,\n                migrations: self.migrations,\n                child_slots: self.child_slots,\n", "                document_codecs: self.document_codecs,\n                migrations: self.migrations,\n                schema_documents: self.schema_documents,\n                child_slots: self.child_slots,\n", 2, "sdk: builder moves")
    text = once(text, "        document_codecs: &'a [DocumentCodecSpec],\n    }\n\n    impl ArtifactCapabilitySources<'_> {\n", "        document_codecs: &'a [DocumentCodecSpec],\n        schema_documents: &'a [::semio_framework_schema::ScopeSchemaExports],\n    }\n\n    impl ArtifactCapabilitySources<'_> {\n", "sdk: sources field")
    text = once(text, "            let Self { schemas, inferences, inference_services, composers, formats, subset_validators, languages, document_codecs } = self;\n", "            let Self { schemas, inferences, inference_services, composers, formats, subset_validators, languages, document_codecs, schema_documents } = self;\n", "sdk: sources destructure")
    text = once(
        text,
        "            Ok(rows)\n        }\n    }\n\n    impl ArtifactDeclaration {\n",
        "            for documents in schema_documents {\n                for export in documents.exports {\n                    rows.push(ArtifactRuntimeCapabilityRequirement::new(ArtifactCapabilityKind::schema(), vec![ArtifactIdentityClaim::schema_export(documents.scope, export.id)?]));\n                }\n            }\n            Ok(rows)\n        }\n    }\n\n    impl ArtifactDeclaration {\n",
        "sdk: requirement rows",
    )
    text = times(text, "                document_codecs: &self.document_codecs,\n            }\n            .requirements()\n", "                document_codecs: &self.document_codecs,\n                schema_documents: &self.schema_documents,\n            }\n            .requirements()\n", 2, "sdk: sources construction")
    text = once(
        text,
        "        /// 🧭️ Appends dialect migrations for transactional registration.\n",
        "        /// 📚️ Appends shared schema documents of this artifact — named exports of its own scope that its facets `$ref`\n"
        "        /// and the four fixed facets cannot carry — committed into the OS-wide export registry with the other catalogs, so\n"
        "        /// every contract referencing them resolves. One `schema` capability row per export, claiming it as\n"
        "        /// [`ArtifactIdentityClaim::schema_export`].\n"
        "        pub fn schema_documents(self, documents: ::semio_framework_schema::ScopeSchemaExports) -> Self {\n"
        "            resolve_ready(self.schema_documents_async(documents))\n"
        "        }\n"
        "\n"
        "        async fn schema_documents_async(mut self, documents: ::semio_framework_schema::ScopeSchemaExports) -> Self {\n"
        "            for export in documents.exports {\n"
        "                require_declared_capability_or_record(&self.definition, &mut self.definition_error, &ArtifactCapabilityKind::schema(), ArtifactIdentityClaim::schema_export(documents.scope, export.id).map(|claim| vec![claim]));\n"
        "            }\n"
        "            self.schema_documents.push(documents);\n"
        "            self\n"
        "        }\n"
        "\n"
        "        /// 🧭️ Appends dialect migrations for transactional registration.\n",
        "sdk: schema_documents method",
    )
    text = once(text, "        migrations: Vec<store::DialectMigration>,\n        app_schemas: Vec<::semio_framework_schema::AppSchemaDescriptor>,\n        owner: String,\n", "        migrations: Vec<store::DialectMigration>,\n        schema_documents: Vec<::semio_framework_schema::ScopeSchemaExports>,\n        app_schemas: Vec<::semio_framework_schema::AppSchemaDescriptor>,\n        owner: String,\n", "sdk: plan field")
    text = once(text, "                migrations: Vec::new(),\n                app_schemas,\n", "                migrations: Vec::new(),\n                schema_documents: Vec::new(),\n                app_schemas,\n", "sdk: plan init")
    text = once(text, "                plan.migrations.extend(declaration.migrations.iter().cloned());\n", "                plan.migrations.extend(declaration.migrations.iter().cloned());\n                plan.schema_documents.extend(declaration.schema_documents.iter().copied());\n", "sdk: plan collect")
    text = once(text, "subset_validators, composers, languages, document_codecs, migrations, app_schemas, owner, host_media_handlers, flow_extensions } =\n", "subset_validators, composers, languages, document_codecs, migrations, schema_documents, app_schemas, owner, host_media_handlers, flow_extensions } =\n", "sdk: into_runtime destructure")
    text = once(text, "                PluginRuntimeRegistry { definitions, schemas, inferences, inference_services, routed_inferences, languages, app_schemas,", "                PluginRuntimeRegistry { definitions, schemas, inferences, schema_documents, inference_services, routed_inferences, languages, app_schemas,", "sdk: runtime construction")
    text = once(text, "        schemas: Vec<::semio_framework_schema::ArtifactSchemaDescriptor>,\n        inferences: Vec<::semio_framework_schema::ArtifactInferenceDescriptor>,\n        inference_services: ArtifactInferenceServiceRegistry,\n", "        schemas: Vec<::semio_framework_schema::ArtifactSchemaDescriptor>,\n        inferences: Vec<::semio_framework_schema::ArtifactInferenceDescriptor>,\n        schema_documents: Vec<::semio_framework_schema::ScopeSchemaExports>,\n        inference_services: ArtifactInferenceServiceRegistry,\n", "sdk: runtime field")
    text = once(text, "                schemas: Vec::new(),\n                inferences: Vec::new(),\n                inference_services: ArtifactInferenceServiceRegistry::new(),\n", "                schemas: Vec::new(),\n                inferences: Vec::new(),\n                schema_documents: Vec::new(),\n                inference_services: ArtifactInferenceServiceRegistry::new(),\n", "sdk: runtime empty")
    text = once(
        text,
        "        pub(crate) fn publish_declared_catalogs(&self) -> Result<(), PluginAssemblyError> {\n",
        "        pub(crate) fn publish_declared_catalogs(&self) -> Result<(), PluginAssemblyError> {\n"
        "            let documents_error = |error: ::semio_framework_schema::SchemaExportRegistryError| PluginAssemblyError::new(\"plugin-assembly.declaration-schema-documents\", error.to_string());\n"
        "            store::register_store_schema_exports().map_err(documents_error)?;\n"
        "            store::io_schema::register_io_schema_exports().map_err(documents_error)?;\n"
        "            for documents in &self.schema_documents {\n"
        "                ::semio_framework_schema::register_scope_schema_exports(*documents).map_err(documents_error)?;\n"
        "            }\n",
        "sdk: publish documents",
    )
    text = once(
        text,
        "        /// 📌️ Publishes the schema and inference descriptors the `.artifact(…)` declarations contributed into the OS-wide\n",
        "        /// 📌️ Publishes the store's and io vocabulary's own schema documents, the declared shared schema documents, and the\n"
        "        /// schema and inference descriptors the `.artifact(…)` declarations contributed into the OS-wide\n",
        "sdk: publish doc",
    )
    return text


def builder(text):
    text = once(
        text,
        "    artifacts: Vec<ArtifactDeclaration>,\n    artifact_definitions: Vec<crate::app::ArtifactDefinition>,\n",
        "    artifacts: Vec<ArtifactDeclaration>,\n    /// 🏠️ Declarations of kinds another package owns, hosted in this plugin's guest — see [`Self::host_artifact`].\n    hosted_artifacts: Vec<ArtifactDeclaration>,\n    artifact_definitions: Vec<crate::app::ArtifactDefinition>,\n",
        "builder: field",
    )
    text = once(text, "            artifacts: Vec::new(),\n", "            artifacts: Vec::new(),\n            hosted_artifacts: Vec::new(),\n", "builder: new")
    text = times(text, "            artifacts: self.artifacts,\n", "            artifacts: self.artifacts,\n            hosted_artifacts: self.hosted_artifacts,\n", 2, "builder: typestate moves")
    text = once(
        text,
        "    /// 🧾️ Registers one definition-only artifact through the same typed preflight registry.\n",
        "    /// 🏠️ Hosts the runtime of one artifact kind another package owns, so this package's own guest opens, edits,\n"
        "    /// imports and exports it: the owner's document schemas, inference descriptors, document codecs, composers, formats,\n"
        "    /// subset validators and dialect migrations commit into this guest's registries exactly as the owner's assembly\n"
        "    /// commits them (identical rows are tolerated, a conflicting row is fatal). Hosting is not owning: the kind's owner is a\n"
        "    /// direct dependency, its definition is never registered here, its inference services stay listed by the owner alone, and\n"
        "    /// `describe` lists composers only for kinds this plugin's id owns and codec rows only for kinds this plugin's apps open\n"
        "    /// — no descriptor or hub catalog row moves. Repeatable, once per kind. See ticket\n"
        "    /// `26/09/23/END-TO-END-OS-HUB-COLLABORATION-MCP` `📓️wp-lb2.md` (p9).\n"
        "    pub fn host_artifact(mut self, declaration: ArtifactDeclaration) -> Self {\n"
        "        self.hosted_artifacts.push(declaration);\n"
        "        self\n"
        "    }\n"
        "\n"
        "    /// 🧾️ Registers one definition-only artifact through the same typed preflight registry.\n",
        "builder: host_artifact",
    )
    text = once(text, "            artifacts,\n            artifact_definitions,\n            mut capabilities,\n", "            artifacts,\n            hosted_artifacts,\n            artifact_definitions,\n            mut capabilities,\n", "builder: destructure")
    text = once(
        text,
        "        for declaration in &artifacts {\n            declaration.preflight(&plugin_id, &mut definitions)?;\n        }\n",
        "        for declaration in &artifacts {\n            declaration.preflight(&plugin_id, &mut definitions)?;\n        }\n"
        "        let mut hosted_kinds = BTreeSet::new();\n"
        "        for declaration in &hosted_artifacts {\n"
        "            declaration.preflight_hosted(&plugin_id, &dependencies)?;\n"
        "            if !hosted_kinds.insert(declaration.definition().identity().as_str()) {\n"
        "                return Err(PluginAssemblyError::new(\"plugin-assembly.hosted-artifact-repeated\", format!(\"plugin {plugin_id:?} hosts {:?} twice\", declaration.definition().identity().as_str())));\n"
        "            }\n"
        "        }\n",
        "builder: preflight hosted",
    )
    text = once(text, "ArtifactRegistrationPlan::from_declarations(&artifacts, app_schemas,", "ArtifactRegistrationPlan::from_declarations(&artifacts, &hosted_artifacts, app_schemas,", "builder: plan")
    text = once(text, "        for declaration in artifacts {\n            plugin = declaration.apply_to(plugin);\n        }\n", "        for declaration in artifacts.into_iter().chain(hosted_artifacts) {\n            plugin = declaration.apply_to(plugin);\n        }\n", "builder: apply")
    return text
#endregion SDK


#region SchemaDocuments
def schema_component(text):
    return once(
        text,
        "/// 📎 Registers one artifact's handcrafted inference descriptor into the OS-wide catalog. `id` on\n"
        "/// the descriptor must be `\"{artifact_id}.inference\"`, matching its owning `ArtifactSchemaDescriptor`'s id.\n"
        "pub fn register_artifact_inference_descriptor(descriptor: ArtifactInferenceDescriptor) {\n"
        "    register_kernel_artifact_inference_descriptor(inference_descriptor_to_kernel(&descriptor));\n"
        "}\n",
        "/// 📎 Registers one artifact's handcrafted inference descriptor into the OS-wide catalog, and its document as the\n"
        "/// `inference` export of its own scope so a contract that `$ref`s the inference document resolves it. `id` on the\n"
        "/// descriptor must be `\"{artifact_id}.inference\"`, matching its owning `ArtifactSchemaDescriptor`'s id.\n"
        "pub fn register_artifact_inference_descriptor(descriptor: ArtifactInferenceDescriptor) {\n"
        "    register_kernel_artifact_inference_descriptor(inference_descriptor_to_kernel(&descriptor));\n"
        "    let exports: &'static [SchemaExport] = Box::leak(Box::new([SchemaExport { id: \"inference\", leaves: descriptor.inference }]));\n"
        "    let _ = register_scope_schema_exports(ScopeSchemaExports { scope: descriptor.id, exports });\n"
        "}\n",
        "schema: inference export",
    )


def kernel_manifest(text):
    return once(text, "semio-framework-replication = { workspace = true }\n", "semio-framework-replication = { workspace = true }\nsemio-framework-schema-registry = { workspace = true }\n", "kernel: registry dependency")


def leaves(prefix, formats):
    names = {"rust": "🦀️.rs", "typescript": "🟦️.ts", "graphql": "🔗️.graphql", "json_schema": "🔣️.json", "proto": "🛰️.proto"}
    return f"{REGISTRY}::FacetLeaves {{ " + ", ".join(f'{field}: include_str!("{prefix}{names[field]}")' if field in formats else f'{field}: ""' for field in names) + " }"


ALL = ("rust", "typescript", "graphql", "json_schema", "proto")


def io_schema(text):
    region = (
        "\n//#region 🔖️SchemaExports\n"
        f"const IO_SCHEMA_EXPORTS: [{REGISTRY}::SchemaExport; 1] = [{REGISTRY}::SchemaExport {{ id: \"schema\", leaves: {leaves('', ALL)} }}];\n"
        "\n"
        "/// 📌️ Registers the io vocabulary's own schema document (`framework/io/schema.json`: `ArtifactRef`, dialects, the io wire\n"
        "/// types) as the `schema` export of the `framework.io` scope, so every contract that `$ref`s it resolves it.\n"
        "// 🚫️async: E1 pure registration helper (no I/O) — see R9\n"
        f"pub fn register_io_schema_exports() -> Result<(), {REGISTRY}::SchemaExportRegistryError> {{\n"
        f"    {REGISTRY}::register_scope_schema_exports({REGISTRY}::ScopeSchemaExports {{ scope: \"framework.io\", exports: &IO_SCHEMA_EXPORTS }})\n"
        "}\n"
        "//#endregion 🔖️SchemaExports\n"
    )
    return once(text, "//#endregion 🔖️Route\n// #endregion io-schema\n", "//#endregion 🔖️Route\n" + region + "// #endregion io-schema\n", "io_schema: exports")


def store(text):
    exports = [("child", "🪆️child/🧬️schema/", ("typescript", "graphql", "json_schema", "proto")), ("link", "🔗️link/🧬️schema/", ALL), ("blob", "📦️blob/🧬️schema/", ALL)]
    rows = "".join(f"    {REGISTRY}::SchemaExport {{ id: \"{name}\", leaves: {leaves(prefix, formats)} }},\n" for name, prefix, formats in exports)
    region = (
        "\n//#region 🔖️SchemaExports\n"
        f"const STORE_SCHEMA_EXPORTS: [{REGISTRY}::SchemaExport; {len(exports)}] = [\n{rows}];\n"
        "\n"
        "/// 📌️ Registers the store's own document-model schema documents (`os/store/child.json`, `link.json`, `blob.json`) as\n"
        "/// exports of the `os.store` scope, so every artifact contract that `$ref`s a composed child, a link or a blob resolves them.\n"
        "// 🚫️async: E1 pure registration helper (no I/O) — see R9\n"
        f"pub fn register_store_schema_exports() -> Result<(), {REGISTRY}::SchemaExportRegistryError> {{\n"
        f"    {REGISTRY}::register_scope_schema_exports({REGISTRY}::ScopeSchemaExports {{ scope: \"os.store\", exports: &STORE_SCHEMA_EXPORTS }})\n"
        "}\n"
        "//#endregion 🔖️SchemaExports\n"
    )
    return once(text, "//#endregion 🔖️InteractionStatePack\n", "//#endregion 🔖️InteractionStatePack\n" + region, "store: exports")


def semio_base_schema(text):
    child = "framework_schema::FacetLeaves { rust: include_str!(\"🪆️child/🦀️.rs\"), typescript: include_str!(\"🪆️child/🟦️.ts\"), graphql: \"\", json_schema: include_str!(\"🪆️child/🔣️.json\"), proto: \"\" }"
    geometry = "framework_schema::FacetLeaves { rust: include_str!(\"🧮️geometry/🦀️.rs\"), typescript: include_str!(\"🧮️geometry/🟦️.ts\"), graphql: include_str!(\"🧮️geometry/🔗️.graphql\"), json_schema: include_str!(\"🧮️geometry/🔣️.json\"), proto: include_str!(\"🧮️geometry/🛰️.proto\") }"
    block = (
        "/// 📚️ semio's shared schema documents — named exports of the `s.stdio.semio` scope that the subsets' facets `$ref`\n"
        "/// (`base/geometry.json`, `base/child.json`), declared with the artifact (`ArtifactDeclarationBuilder::schema_documents`).\n"
        "pub const SEMIO_SHARED_SCHEMA_DOCUMENTS: framework_schema::ScopeSchemaExports = framework_schema::ScopeSchemaExports {\n"
        "    scope: \"s.stdio.semio\",\n"
        f"    exports: &[framework_schema::SchemaExport {{ id: \"geometry\", leaves: {geometry} }}, framework_schema::SchemaExport {{ id: \"child\", leaves: {child} }}],\n"
        "};\n"
        "\n"
    )
    return once(text, R9 + "pub fn semio_artifact_schema_descriptor() -> framework_schema::ArtifactSchemaDescriptor {\n", block + R9 + "pub fn semio_artifact_schema_descriptor() -> framework_schema::ArtifactSchemaDescriptor {\n", "semio: shared documents")


def stdio_manifest(text):
    return once(text, "[dev-dependencies]\nsemio-framework-async-macros = { workspace = true }\n", "[dev-dependencies]\nsemio-framework-async-macros = { workspace = true }\nsemio-framework-schema = { workspace = true }\n", "stdio: schema dev-dependency")
def lock_dependency(text, package, after, dependency):
    block = re.search(r'\[\[package\]\]\nname = "' + re.escape(package) + r'"\nversion = "[^"]+"\n(?:source = "[^"]+"\n)?dependencies = \[\n(.*?)\n\]', text, re.S)
    if not block or f' "{after}",' not in block.group(1).split("\n") and f' "{after}"' not in block.group(1).split("\n"):
        problems.append(f"lock: {package} has no dependency {after}")
        return text
    lines = block.group(1).split("\n")
    index = next(i for i, line in enumerate(lines) if line.strip().strip(",") == f'"{after}"')
    lines.insert(index + 1, f' "{dependency}",')
    lines = [line if line.endswith(",") else line + "," for line in lines[:-1]] + [lines[-1]]
    body = "\n".join(lines)
    return text[: block.start(1)] + body + text[block.end(1) :]


def lock_sorted_dependency(text, package, dependency):
    block = re.search(r'\[\[package\]\]\nname = "' + re.escape(package) + r'"\nversion = "[^"]+"\n(?:source = "[^"]+"\n)?dependencies = \[\n(.*?)\n\]', text, re.S)
    if not block:
        problems.append(f"lock: no package {package}")
        return text
    names = [line.strip().strip(",").strip('"') for line in block.group(1).split("\n")]
    if dependency in names:
        problems.append(f"lock: {package} already depends on {dependency}")
        return text
    previous = max((name for name in names if name < dependency), default=None)
    if previous is None:
        problems.append(f"lock: {package} has no dependency before {dependency}")
        return text
    return lock_dependency(text, package, previous, dependency)


def lock(text, extras):
    text = lock_dependency(text, "semio-framework-os-kernel", "semio-framework-replication", "semio-framework-schema-registry")
    text = lock_dependency(text, "semio-s-plugin-stdio", "semio-framework-plugin", "semio-framework-schema")
    for fam, crates in extras.items():
        package = re.search(r'^name = "([^"]+)"', open(os.path.join(TREE, STDIO, "🧩️extensions", fam, "📦️packages", "🦀️rust", "Cargo.toml"), encoding="utf-8").read(), re.M).group(1)
        for crate in crates:
            text = lock_sorted_dependency(text, package, crate.replace("_", "-"))
    return text
#endregion SchemaDocuments


#region Contract
def contract(text):
    return once(
        text,
        '"schema" | "codec" | "codec-extension" | "extension" | "mime" | "dialect" | "validated-dialect" | "grammar")',
        '"schema" | "schema-export" | "codec" | "codec-extension" | "extension" | "mime" | "dialect" | "validated-dialect" | "grammar")',
        "contract: schema-export namespace",
    )
#endregion Contract


#region Families
SCHEMA_HOST = "https://json.schemas.assets.semio-tech.com/s/stdio/"


def activated_crates(text):
    return re.findall(r"\.activation\(ActivationEvent::OnArtifactKind \{ kind: (semio_s_artifact_stdio_[a-z0-9_]+)::artifact_kind\(\)\.id \}\)", text)


def artifact_documents():
    documents = {}
    for path in glob.glob(os.path.join(TREE, ART, "*", "🏅️standards", "**", "🔣️.json"), recursive=True):
        if "🧫️fixtures" in path or "🧪️tests" in path:
            continue
        try:
            document = json.load(open(path, encoding="utf-8"))
        except (OSError, ValueError):
            continue
        if isinstance(document, dict) and isinstance(document.get("$id"), str) and document["$id"].startswith(SCHEMA_HOST):
            documents[document["$id"]] = (path, document)
    return documents


def references(node):
    if isinstance(node, dict):
        reference = node.get("$ref")
        found = [reference.split("#")[0]] if isinstance(reference, str) and reference.startswith(SCHEMA_HOST) else []
        return found + [item for value in node.values() for item in references(value)]
    if isinstance(node, list):
        return [item for value in node for item in references(value)]
    return []


def schema_closure(crate, documents):
    """🔗️ The other stdio kinds whose schema documents `crate`'s snapshot contracts reference, transitively (law e)."""
    kind = crate.removeprefix("semio_s_artifact_stdio_")
    pending = [identity for identity, (path, _) in documents.items() if identity.startswith(f"{SCHEMA_HOST}{kind}/") and "/📸️snapshot/" in path]
    seen = set(pending)
    while pending:
        for reference in references(documents[pending.pop()][1]):
            if reference in documents and reference not in seen:
                seen.add(reference)
                pending.append(reference)
    return sorted({f"semio_s_artifact_stdio_{identity.removeprefix(SCHEMA_HOST).split('/')[0]}" for identity in seen} - {crate})


def hosted_extras():
    documents = artifact_documents()
    extras = {}
    for fam in FAMILIES:
        activated = activated_crates(open(os.path.join(TREE, STDIO, "🧩️extensions", fam, "🦀️.rs"), encoding="utf-8").read())
        extras[fam] = sorted({extra for crate in activated for extra in schema_closure(crate, documents)} - set(activated))
    return extras


def family(text, label, extras):
    kinds = activated_crates(text)
    if not kinds:
        problems.append(f"{label}: no activation rows")
        return text
    hosts = "".join(f"        .host_artifact({crate}::declaration({crate}::definition()?).map_err(PluginAssemblyError::definition)?)\n" for crate in kinds + extras)
    text = once(text, "        .depends_on(\"stdio\", semio_framework::tree_pin!())\n", "        .depends_on(\"stdio\", semio_framework::tree_pin!())\n" + hosts, f"{label}: depends_on")
    composed = "" if not extras else " — plus " + ", ".join(f"`{extra.removeprefix('semio_s_artifact_stdio_')}`" for extra in extras) + ", whose schema documents\n/// their snapshot contracts compose (`$ref`)"
    text = once(text, "`artifact_kind().id`, and the exact-pin runtime dependency on `stdio`, which owns those kinds and their codecs.\n", "`artifact_kind().id`, the exact-pin runtime dependency on `stdio`, which owns those kinds and their codecs, and the hosted runtime of\n/// each of them (`host_artifact`: schemas, inferences, document codecs, composers, formats, subset validators) in this component" + composed + ".\n", f"{label}: plugin doc")
    return text


def family_manifest(text, label, extras):
    for extra in extras:
        name = extra.replace("_", "-")
        rows = re.findall(r"^semio-s-artifact-stdio-[a-z0-9-]+ = \{ workspace = true[^\n]*\}\n", text, re.M)
        if not rows or f"{name} = " in text:
            problems.append(f"{label}: manifest anchor for {name}")
            continue
        after = max((row for row in rows if row.split(" = ")[0] < name), default=None, key=lambda row: row.split(" = ")[0])
        row = f"{name} = {{ workspace = true }}\n"
        text = text.replace(after, after + row, 1) if after else text.replace(rows[0], row + rows[0], 1)
    return text
#endregion Families


#region Roots
def runtime_root(text, artifact, body, label):
    text = once(text, f"    semio_s_artifact_stdio_contract::definition_only_assembly(\"{artifact}\", definition()?)\n}}\n", f"    semio_s_artifact_stdio_contract::runtime_assembly(\"{artifact}\", definition()?, declaration)\n}}\n\n{body}", f"{label}: assembly")
    if artifact == "binary":
        text = once(text, "use semio_framework_plugin::{ArtifactDefinition, ArtifactDefinitionError, PluginAssemblyError};\n", "use semio_framework_plugin::{ArtifactDeclaration, ArtifactDefinition, ArtifactDefinitionError, PluginAssemblyError};\n", f"{label}: declaration import")
    if artifact == "gif":
        text = once(text, "pub use schema::snapshot::GifSnapshot;\n", "pub use schema::snapshot::GifSnapshot;\npub use schema::snapshot::STDIO_GIF89A_DOCUMENT_SCHEMA;\n", f"{label}: 89a document schema export")
    return text


def chain(doc, links, imported=False):
    signature, builder = (IMPORTED_DECLARATION_SIGNATURE, "ArtifactDeclaration") if imported else (DECLARATION_SIGNATURE, "semio_framework_plugin::ArtifactDeclaration")
    return f"/// {doc}\n{R9}{signature}    let formats = formats()?;\n    {builder}::builder(definition)\n" + "".join(f"        .{link}\n" for link in links) + "        .try_build()\n}\n"


ROOT_BODIES = {
    "🪟️bmp": chain(
        "🧾️ The runtime `bmp` declares: its schema, format, inference descriptor, composers and document codec.",
        [
            "schema(schema::bmp_artifact_schema_descriptor())",
            "formats(formats)",
            "inferences([standards::v_v3::subsets::any::schema::inferences::bmp_artifact_inference_descriptor()])",
            "composers(standards::v_v3::subsets::any::io::io_registry::entries())",
            "document_codec_bare::<BmpSnapshot, BmpMutation>(STDIO_BMP_DOCUMENT_SCHEMA)",
        ],
    ),
    "🔊️wav": chain(
        "🧾️ The runtime `wav` declares: its schema, format, inference descriptor, composers and document codec.",
        [
            "schema(standards::riff_pcm::subsets::any::schema::wav_artifact_schema_descriptor())",
            "formats(formats)",
            "inferences([standards::riff_pcm::subsets::any::schema::inferences::wav_artifact_inference_descriptor()])",
            "composers(standards::riff_pcm::subsets::any::io::io_registry::entries())",
            "document_codec_bare::<WavSnapshot, WavMutation>(STDIO_WAV_DOCUMENT_SCHEMA)",
        ],
    ),
    "🌦️epw": chain(
        "🧾️ The runtime `epw` declares: its schema, format, inference descriptor, composers and document codec.",
        [
            "schema(standards::energyplus::subsets::any::schema::epw_artifact_schema_descriptor())",
            "formats(formats)",
            "inferences([standards::energyplus::subsets::any::schema::inferences::epw_artifact_inference_descriptor()])",
            "composers(standards::energyplus::subsets::any::io::io_registry::entries())",
            "document_codec_bare::<EpwSnapshot, EpwMutation>(STDIO_EPW_DOCUMENT_SCHEMA)",
        ],
    ),
    "💾️binary": chain(
        "🧾️ The runtime `binary` declares: its schema, format, inference descriptor, composers and document codec.",
        [
            "schema(standards::v_raw::subsets::any::schema::binary_artifact_schema_descriptor())",
            "formats(formats)",
            "inferences([standards::v_raw::subsets::any::schema::inferences::binary_artifact_inference_descriptor()])",
            "composers(standards::v_raw::subsets::any::io::io_registry::entries())",
            "document_codec_bare::<BinarySnapshot, BinaryMutation>(STDIO_BINARY_DOCUMENT_SCHEMA)",
        ],
        imported=True,
    ),
    "🎞️gif": chain(
        "🧾️ The runtime `gif` declares for both standards (`87a`, `89a`, independently versioned ids): schemas, format, inference\n/// descriptors, composers and document codecs.",
        [
            "schema(standards::v87a::subsets::any::schema::gif_artifact_schema_descriptor())",
            "schemas([standards::v89a::subsets::any::schema::gif_artifact_schema_descriptor()])",
            "formats(formats)",
            "inferences([standards::v87a::subsets::any::schema::inferences::gif_artifact_inference_descriptor(), standards::v89a::subsets::any::schema::inferences::gif89a_artifact_inference_descriptor()])",
            "composers(standards::v87a::engine::io_registry::entries())",
            "composers(standards::v89a::engine::io_registry::entries())",
            "document_codec_bare::<standards::v87a::subsets::any::schema::snapshot::GifSnapshot, standards::v87a::subsets::any::schema::mutations::GifMutation>(STDIO_GIF_DOCUMENT_SCHEMA)",
            "document_codec_bare::<GifSnapshot, GifMutation>(STDIO_GIF89A_DOCUMENT_SCHEMA)",
        ],
    ),
}

IFC_BODY = (
    "/// 🧾️ The runtime `ifc` declares for both standards (`4`, `2x3`, independently versioned ids): schemas, format, inference\n"
    "/// descriptors, composers, document codecs, and the `2x3` model-view-definition validators (`cv20`, `sav`, `cobie`).\n"
    + R9
    + DECLARATION_SIGNATURE
    + "    let builder = semio_framework_plugin::ArtifactDeclaration::builder(definition)\n"
    "        .schema(standards::v4::subsets::any::schema::ifc_artifact_schema_descriptor())\n"
    "        .schemas([standards::v2x3::subsets::base::schema::ifc2x3_artifact_schema_descriptor()])\n"
    "        .formats(formats()?)\n"
    "        .inferences([standards::v4::subsets::any::schema::inferences::ifc_artifact_inference_descriptor(), standards::v2x3::subsets::base::schema::inferences::ifc2x3_artifact_inference_descriptor()])\n"
    "        .composers(standards::v4::engine::io_registry::entries())\n"
    "        .composers(standards::v2x3::engine::io_registry::entries())\n"
    "        .document_codec_bare::<IfcSnapshot, IfcMutation>(STDIO_IFC_DOCUMENT_SCHEMA)\n"
    "        .document_codec_bare::<standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot, standards::v2x3::subsets::base::schema::mutations::Ifc2x3Mutation>(standards::v2x3::subsets::base::schema::snapshot::STDIO_IFC2X3_DOCUMENT_SCHEMA);\n"
    "    let builder = standards::v2x3::subsets::cv20::io::declare(builder);\n"
    "    let builder = standards::v2x3::subsets::sav::io::declare(builder);\n"
    "    standards::v2x3::subsets::cobie::io::declare(builder).try_build()\n"
    "}\n"
)

SEMIO_ORDER = ["animation", "audio", "brep", "cad", "document", "drawing", "flow", "graph", "image", "kit", "mesh", "model", "object", "presentation", "table", "text", "value", "video"]
SEMIO_BODY = (
    "/// 🧾️ The runtime `semio` declares: the `✉️base` envelope schema opens the declaration and every subset appends its own\n"
    "/// rows (`subsets::*::io::declare`, the declarative twin of each subset's `register`).\n"
    + R9
    + DECLARATION_SIGNATURE
    + "    let builder = semio_framework_plugin::ArtifactDeclaration::builder(definition).schema(subsets::base::schema::semio_artifact_schema_descriptor()).formats(formats()?).schema_documents(subsets::base::schema::SEMIO_SHARED_SCHEMA_DOCUMENTS);\n"
    "    let builder = subsets::base::io::declare(builder);\n"
    + "".join(f"    let builder = subsets::{name}::io::declare(builder);\n" for name in SEMIO_ORDER[:-1])
    + f"    subsets::{SEMIO_ORDER[-1]}::io::declare(builder).try_build()\n"
    "}\n"
    "\n"
    "/// 🎹️ The rows of `table` that write a `s.stdio.semio` dialect, copied once into `cache`. A composer's capability claims the\n"
    "/// dialect it writes and a dialect belongs to exactly one kind, so only these rows are this artifact's to declare; the bridge\n"
    "/// serializers that write another kind (semio → step, png, …) are that kind's, and stay reachable only through `register()`.\n"
    + R9
    + "pub(crate) fn semio_written(table: &'static [semio_framework_plugin::ComposerEntry], cache: &'static std::sync::OnceLock<Vec<semio_framework_plugin::ComposerEntry>>) -> &'static [semio_framework_plugin::ComposerEntry] {\n"
    "    cache.get_or_init(|| table.iter().filter(|entry| entry.writes.artifact_kind == SEMIO_ARTIFACT_SCHEMA_ID).map(|entry| semio_framework_plugin::ComposerEntry { writes: entry.writes, reads: entry.reads, compose: entry.compose }).collect())\n"
    "}\n"
)


def root_docs(text, root):
    if root == "🏗️ifc":
        found = re.search(r"/// ⚠️ \*\*Deliberately left imperative\*\*.*?\n(?=pub fn definition\(\))", text, re.S)
        if not found:
            problems.append("ifc: stale imperative doc")
            return text
        return text[: found.start()] + "/// 📜 The schema-owned definition: one kind, two independently versioned standards (`4`, `2x3`) — see [`declaration`].\n" + text[found.end() :]
    if root == "🧿️semio":
        found = re.search(r"/// 🗂️ Registers all 19 of `v1`'s subsets' IO composers.*?\n(?=// 🚫️async: E1 pure codec/computation helper[^\n]*\npub fn register\(\))", text, re.S)
        if not found:
            problems.append("semio: register doc")
            return text
        return text[: found.start()] + "/// 🗂️ Registers all 19 subsets imperatively, outside any plugin assembly — the twin of [`declaration`] for native callers\n/// that open semio documents without assembling `stdio`.\n" + text[found.end() :]
    if root == "🌦️epw":
        found = re.search(r"/// 🗂️ Registers this artifact's IO composer \+ the handcrafted grammar/protocol `LanguageSpec`.*?\n(?=// 🚫️async: E1 pure codec/computation helper[^\n]*\npub fn register\(\))", text, re.S)
        if not found:
            problems.append("epw: register doc")
            return text
        return text[: found.start()] + "/// 🗂️ Registers this artifact's IO and its handcrafted grammar/protocol `LanguageSpec` imperatively, outside any plugin\n/// assembly — the twin of [`declaration`].\n" + text[found.end() :]
    return text


def binary_io_doc(text):
    found = re.search(r"/// 🗂️ Registers codecs, the artifact schema descriptor, and every composer entry — dissolved out\n.*?\n(?=// 🚫️async: E1 pure codec/computation helper[^\n]*\npub fn register\(\))", text, re.S)
    if not found:
        problems.append("binary io: register doc")
        return text
    return text[: found.start()] + "/// 🗂️ Registers codecs, the artifact schema descriptor and every composer entry imperatively, outside any plugin assembly —\n/// the twin of [`crate::declaration`].\n" + text[found.end() :]
#endregion Roots


#region Subsets
def subset_declare(text, name, label):
    register = re.search(r"\n    pub fn register\(\) \{\n(.*?)\n    \}\n", text, re.S)
    if not register:
        problems.append(f"{label}: register")
        return text
    body = register.group(1)
    codec = re.search(r"ArtifactCodec::of::<([^,<>]+),\s*([^<>]+?)>\(\s*([^()]+?),?\s*\)\)", body, re.S)
    schema = re.search(r"register_artifact_schema_descriptor\(([^()]+)\(\)\)", body)
    inference = re.search(r"register_artifact_inference_descriptor\(([^()]+)\(\)\)", text)
    composers = re.search(r"(?:#\[cfg\(feature = \"([a-z-]+)\"\)\]\s*)?register_composer_entries\(([a-z_]+)\(\)\)", body)
    if not (codec and inference and "register_subset_validator(validator_entry())" in body and (schema or name == "base")):
        problems.append(f"{label}: register shape")
        return text
    links = []
    if name != "base":
        links.append(f".schemas([{schema.group(1)}()])")
    links.append(f".document_codec_bare::<{codec.group(1).strip()}, {codec.group(2).strip()}>({codec.group(3).strip()})")
    links.append(".subset_validators(std::slice::from_ref(validator_entry()))")
    links.append(f".inferences([{inference.group(1)}()])")
    gated = None
    entry = "ComposerEntry" if re.search(r"use semio_framework_plugin::\{[^}]*\bComposerEntry\b", text) else "semio_framework_plugin::ComposerEntry"
    cache = f"static COMPOSERS: std::sync::OnceLock<Vec<{entry}>> = std::sync::OnceLock::new();"
    if composers and composers.group(1):
        gated = (composers.group(1), composers.group(2))
    elif composers:
        links.append(f".composers(crate::semio_written({composers.group(2)}(), &COMPOSERS))")
    rows = "subset's schema, document codec, `SubsetValidator`, composers\n    /// (those writing semio, [`crate::semio_written`]) and inference descriptor" if name != "base" else "envelope's document codec, `SubsetValidator` and inference\n    /// descriptor (its schema opens the declaration)"
    declare = (
        f"\n    /// 🧾️ The declarative twin of [`register`]: this {rows} as rows of [`crate::declaration`].\n"
        "    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9\n"
        f"    pub fn declare(builder: {BUILDER_TYPE}) -> {BUILDER_TYPE} {{\n"
    )
    if gated:
        declare += "        let builder = builder\n" + "".join(f"            {link}\n" for link in links[:-1]) + f"            {links[-1]};\n" + f"        #[cfg(feature = \"{gated[0]}\")]\n        let builder = {{\n            {cache}\n            builder.composers(crate::semio_written({gated[1]}(), &COMPOSERS))\n        }};\n        builder\n    }}\n"
    elif composers:
        declare += f"        {cache}\n        builder\n" + "".join(f"            {link}\n" for link in links) + "    }\n"
    else:
        declare += "        builder\n" + "".join(f"            {link}\n" for link in links) + "    }\n"
    return text[: register.end()] + declare + text[register.end() :]


def mvd_declare(text, label):
    anchor = "        register_subset_validator(validator_entry()).expect(\"static Stdio registration must be available and conflict-free\");\n    }\n"
    declare = (
        "\n    /// 🧾️ The declarative twin of [`register`]: this subset's `SubsetValidator` as a row of the artifact's\n"
        "    /// [`crate::declaration`].\n"
        "    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9\n"
        f"    pub fn declare(builder: {BUILDER_TYPE}) -> {BUILDER_TYPE} {{\n"
        "        builder.subset_validators(std::slice::from_ref(validator_entry()))\n"
        "    }\n"
    )
    return once(text, anchor, anchor + declare, f"{label}: declare")
#endregion Subsets


#region Definitions
def definition_rows(text, root, rows):
    block = json.dumps(sorted(rows, key=lambda row: row["id"]), indent=2, ensure_ascii=False).replace("\n", "\n  ")
    return once(text, '  "runtime_capabilities": [],\n', f'  "runtime_capabilities": {block},\n', f"{root}: runtime_capabilities")
#endregion Definitions


#region Law
PACKAGES = [
    ("stdio", "../../📦️packages/🦀️rust/Cargo.toml", "semio_s_plugin_stdio"),
    ("stdio-image", "../../🧩️extensions/🖼️image/📦️packages/🦀️rust/Cargo.toml", "semio_s_plugin_stdio_image"),
    ("stdio-media", "../../🧩️extensions/🎵️media/📦️packages/🦀️rust/Cargo.toml", "semio_s_plugin_stdio_media"),
    ("stdio-cad", "../../🧩️extensions/🛠️cad/📦️packages/🦀️rust/Cargo.toml", "semio_s_plugin_stdio_cad"),
    ("stdio-bim", "../../🧩️extensions/🏠️bim/📦️packages/🦀️rust/Cargo.toml", "semio_s_plugin_stdio_bim"),
    ("stdio-mesh", "../../🧩️extensions/🔺️mesh/📦️packages/🦀️rust/Cargo.toml", "semio_s_plugin_stdio_mesh"),
    ("stdio-pdf", "../../🧩️extensions/📘️pdf/📦️packages/🦀️rust/Cargo.toml", "semio_s_plugin_stdio_pdf"),
    ("stdio-office", "../../🧩️extensions/💼️office/📦️packages/🦀️rust/Cargo.toml", "semio_s_plugin_stdio_office"),
    ("stdio-semio", "../../🧩️extensions/🧿️semio/📦️packages/🦀️rust/Cargo.toml", "semio_s_plugin_stdio_semio"),
    ("stdio-binary", "../../🧩️extensions/🔢️binary/📦️packages/🦀️rust/Cargo.toml", "semio_s_plugin_stdio_binary"),
]


def law(text):
    old = re.search(r"/// 📦️ The stdio component and its nine family components, each described once\.\nfn packages\(\) -> &'static \[ShippedPackage\] \{\n.*?\n\}\n", text, re.S)
    if not old:
        problems.append("law: packages()")
        return text
    ids = ", ".join(f'"{package}"' for package, _, _ in PACKAGES)
    arms = "".join(f'        "{package}" => (include_str!("{manifest}"), describe({crate}::plugin())),\n' for package, manifest, crate in PACKAGES)
    new = (
        "/// 📦️ The stdio component and its nine family components.\n"
        f"const PACKAGE_IDS: [&str; {len(PACKAGES)}] = [{ids}];\n"
        "\n"
        "/// 🧾️ Assembles and describes one stdio package by id.\n"
        "fn shipped(id: &'static str) -> ShippedPackage {\n"
        "    let (manifest, descriptor) = match id {\n"
        + arms
        + '        other => panic!("{other} is not a stdio package"),\n'
        "    };\n"
        "    ShippedPackage { id, manifest, descriptor }\n"
        "}\n"
        "\n"
        "/// 📦️ Every stdio package, each assembled and described once per process.\n"
        "fn packages() -> &'static [ShippedPackage] {\n"
        "    static PACKAGES: OnceLock<Vec<ShippedPackage>> = OnceLock::new();\n"
        "    PACKAGES.get_or_init(|| PACKAGE_IDS.into_iter().map(shipped).collect())\n"
        "}\n"
    )
    text = text[: old.start()] + new + text[old.end() :]
    text = once(text, "use semio_framework_plugin::{Plugin, PluginApp, PluginAssemblyError};\n", "use semio_framework_plugin::{ArtifactRuntimeCapabilityRequirement, Plugin, PluginApp, PluginAssemblyError};\n", "law: imports")
    text = once(text, "use semio_s_artifact_stdio_contract::editing::SNAPSHOT_EDIT_ACTION_IDS;\n", "use semio_s_artifact_stdio_contract::editing::SNAPSHOT_EDIT_ACTION_IDS;\nuse semio_s_plugin_stdio::registry::ArtifactAssembly;\n", "law: assembly import")
    text += (
        "\n"
        "/// 🧪️ The environment variable naming the one package [`package_runtime_probe`] assembles in its child process.\n"
        'const PROBE_PACKAGE: &str = "SEMIO_STDIO_RUNTIME_PROBE_PACKAGE";\n'
        "\n"
        "/// 🏠️ LAW (d): every stdio package, assembled ALONE in its own process as its wasm guest is, hosts the complete runtime of\n"
        "/// every artifact kind it opens — each schema, inference descriptor, document codec, composer, format and subset validator\n"
        "/// the kind's owner declares is live in that process. Measured before (LB2 native probe, 2026-09-29): `stdio-image`'s\n"
        "/// `plugin()` alone registered none of png/jpg/bmp/svg's schemas, and the bmp, wav, epw, binary, ifc, gif and semio roots\n"
        "/// declared no runtime at all — 28 shipped editors refused every snapshot edit `snapshot-edit.schema-unregistered`.\n"
        "#[test]\n"
        "fn every_package_hosts_the_runtime_of_every_kind_it_opens_in_its_own_process() {\n"
        '    let binary = std::env::current_exe().expect("the test binary");\n'
        "    let failures = PACKAGE_IDS\n"
        "        .into_iter()\n"
        '        .filter_map(|id| {\n'
        '            let run = std::process::Command::new(&binary).args(["package_runtime_probe", "--exact", "--ignored", "--nocapture", "--test-threads", "1"]).env(PROBE_PACKAGE, id).output().expect("the package probe runs");\n'
        '            (!run.status.success()).then(|| format!("{id} alone does not host every kind it opens:\\n{}\\n{}", String::from_utf8_lossy(&run.stdout), String::from_utf8_lossy(&run.stderr)))\n'
        "        })\n"
        "        .collect::<Vec<_>>();\n"
        '    assert!(failures.is_empty(), "{} of {} packages fail:\\n{}", failures.len(), PACKAGE_IDS.len(), failures.join("\\n"));\n'
        "}\n"
        "\n"
        "/// 🔬️ The child half of LAW (d): assembles only the package [`PROBE_PACKAGE`] names and checks every runtime requirement of\n"
        "/// every kind it activates on, exactly as the kind's owner declares them, against this process's live registries.\n"
        "#[test]\n"
        '#[ignore = "the child process of every_package_hosts_the_runtime_of_every_kind_it_opens_in_its_own_process"]\n'
        "fn package_runtime_probe() {\n"
        '    let id = std::env::var(PROBE_PACKAGE).expect("the parent law names one package");\n'
        '    let package = shipped(PACKAGE_IDS.into_iter().find(|candidate| *candidate == id).expect("a stdio package id"));\n'
        '    assert_eq!(package.descriptor.package_id, format!("semio:{id}"), "{id} assembles alone");\n'
        '    let assemblies = semio_s_plugin_stdio::registry::artifact_assemblies().expect("the stdio artifact assemblies");\n'
        "    let mut unmet = Vec::new();\n"
        "    for kind in activated_kinds(&package.descriptor) {\n"
        "        let declaration = assemblies.iter().find_map(|assembly| match assembly {\n"
        "            ArtifactAssembly::Runtime(declaration) if declaration.definition().identity().as_str() == kind => Some(declaration),\n"
        "            _ => None,\n"
        "        });\n"
        "        let Some(declaration) = declaration else {\n"
        '            unmet.push(format!("{kind}: its owner declares no runtime"));\n'
        "            continue;\n"
        "        };\n"
        '        for requirement in declaration.runtime_capability_requirements().expect("the declaration\'s runtime requirements") {\n'
        "            if !requirement_is_live(&requirement) {\n"
        '                unmet.push(format!("{kind}: {requirement:?}"));\n'
        "            }\n"
        "        }\n"
        "    }\n"
        '    assert!(unmet.is_empty(), "{id} alone leaves {} runtime requirements unmet: {unmet:#?}", unmet.len());\n'
        "}\n"
        "\n"
        "/// 🔎️ Whether one runtime requirement is live in this process: schemas and inference descriptors in the kernel catalogs,\n"
        "/// shared schema documents (`schema-export` claims) in the schema export registry,\n"
        "/// document codecs in the store, composers, formats and subset validators in the io registries. Grammar rows are captured by\n"
        "/// the plugin runtime and never published (`PluginRuntimeRegistry::languages`), so no process state answers them.\n"
        "fn requirement_is_live(requirement: &ArtifactRuntimeCapabilityRequirement) -> bool {\n"
        "    use semio_framework_plugin::resolve_ready;\n"
        "    let claims = resolve_ready(requirement.claims());\n"
        "    let values = |namespace: &str| claims.iter().filter(|claim| claim.namespace().as_str() == namespace).map(|claim| claim.value().to_string()).collect::<BTreeSet<_>>();\n"
        "    let value = |namespace: &str| values(namespace).into_iter().next().unwrap_or_default();\n"
        "    match resolve_ready(requirement.kind()).as_str() {\n"
        '        "schema" => match value("schema-export").split_once(\'#\') {\n'
        '            Some((scope, export)) => semio_framework_schema::resolve_schema_export(scope, export, semio_framework_schema::SchemaFormat::JsonSchema).is_ok(),\n'
        '            None => semio_framework_os_kernel::kernel_artifact_schema_descriptor_registered(&value("schema")),\n'
        '        },\n'
        '        "inference" => semio_framework_os_kernel::kernel_artifact_inference_descriptor_registered(&value("schema")),\n'
        '        "codec" => resolve_ready(semio_framework_os_kernel::document_codec(&value("codec"))).expect("the document codec registry").is_some(),\n'
        '        "composer" => resolve_ready(semio_framework::io::list_composer_entries()).expect("the composer registry").iter().any(|(writes, _)| writes.to_coordinate() == value("dialect")),\n'
        '        "subset-validator" => resolve_ready(semio_framework::io::list_registered_subset_validator_dialects()).expect("the subset validator registry").into_iter().any(|dialect| semio_framework::ArtifactDialect::from(dialect).to_coordinate() == value("validated-dialect")),\n'
        '        "representation" => values("extension").iter().filter_map(|extension| semio_framework::io::format_descriptor(extension.trim_start_matches(\'.\')).expect("the format catalog")).any(|format| format.mimes.iter().cloned().collect::<BTreeSet<_>>() == values("mime") && format.extensions.iter().cloned().collect::<BTreeSet<_>>() == values("extension")),\n'
        '        "grammar" => true,\n'
        '        other => panic!("unknown runtime capability category {other}"),\n'
        "    }\n"
        "}\n"
        "\n"
        "/// 🧪️ The environment variable naming the one package [`package_contract_probe`] assembles in its child process.\n"
        'const CONTRACT_PROBE_PACKAGE: &str = "SEMIO_STDIO_CONTRACT_PROBE_PACKAGE";\n'
        "\n"
        "/// 🔗️ LAW (e): in every stdio package's own process — assembled alone, as its wasm guest is — every registered artifact's\n"
        "/// snapshot contract compiles: each `$ref` it makes resolves against a schema document registered in that process. Measured\n"
        "/// before (LB2 scratch, 2026-09-29): las/dwg/ifc refs named absent `$defs`, and semio brep/object/kit `$ref` shared documents\n"
        "/// (`base/geometry.json`, `base/child.json` → `os/store/child.json`, `brep/inference.json`) that no guest registered — every\n"
        "/// snapshot edit on those kinds was refused `snapshot-edit.invalid-schema-contract`.\n"
        "#[test]\n"
        "fn every_registered_snapshot_contract_resolves_in_each_package_process() {\n"
        '    let binary = std::env::current_exe().expect("the test binary");\n'
        "    let failures = PACKAGE_IDS\n"
        "        .into_iter()\n"
        '        .filter_map(|id| {\n'
        '            let run = std::process::Command::new(&binary).args(["package_contract_probe", "--exact", "--ignored", "--nocapture", "--test-threads", "1"]).env(CONTRACT_PROBE_PACKAGE, id).output().expect("the contract probe runs");\n'
        '            (!run.status.success()).then(|| format!("{id} alone registers an unresolvable snapshot contract:\\n{}\\n{}", String::from_utf8_lossy(&run.stdout), String::from_utf8_lossy(&run.stderr)))\n'
        "        })\n"
        "        .collect::<Vec<_>>();\n"
        '    assert!(failures.is_empty(), "{} of {} packages fail:\\n{}", failures.len(), PACKAGE_IDS.len(), failures.join("\\n"));\n'
        "}\n"
        "\n"
        "/// 🔬️ The child half of LAW (e): assembles only the package [`CONTRACT_PROBE_PACKAGE`] names and compiles the snapshot\n"
        "/// contract of every artifact schema registered in this process — owned and hosted alike.\n"
        "#[test]\n"
        '#[ignore = "the child process of every_registered_snapshot_contract_resolves_in_each_package_process"]\n'
        "fn package_contract_probe() {\n"
        '    let id = std::env::var(CONTRACT_PROBE_PACKAGE).expect("the parent law names one package");\n'
        '    let package = shipped(PACKAGE_IDS.into_iter().find(|candidate| *candidate == id).expect("a stdio package id"));\n'
        '    assert_eq!(package.descriptor.package_id, format!("semio:{id}"), "{id} assembles alone");\n'
        "    let contracts = semio_framework_os_kernel::with_kernel_artifact_schema_catalog(|entries| entries.iter().map(|entry| entry.id).collect::<Vec<_>>());\n"
        '    assert!(!contracts.is_empty(), "{id} registers the snapshot contracts of the kinds it opens");\n'
        '    let unresolved = contracts.iter().filter_map(|contract| semio_framework_schema::structural_validator_for(contract, "snapshot").err().map(|error| format!("{contract}: {error}"))).collect::<Vec<_>>();\n'
        '    assert!(unresolved.is_empty(), "{id} alone registers {} unresolvable snapshot contracts: {unresolved:#?}", unresolved.len());\n'
        "}\n"
    )
    return text
#endregion Law


def plan():
    edits = {SDK: sdk, BUILDER: builder, LAW: law, SCHEMA_COMPONENT: schema_component, KERNEL_MANIFEST: kernel_manifest, IO_SCHEMA: io_schema, STORE: store, SEMIO_BASE_SCHEMA: semio_base_schema, STDIO_MANIFEST: stdio_manifest, CONTRACT: contract}
    extras = hosted_extras()
    edits[LOCK] = lambda text: lock(text, extras)
    for fam in FAMILIES:
        edits[f"{STDIO}/🧩️extensions/{fam}/🦀️.rs"] = lambda text, fam=fam: family(text, fam, extras[fam])
        if extras[fam]:
            edits[f"{STDIO}/🧩️extensions/{fam}/📦️packages/🦀️rust/Cargo.toml"] = lambda text, fam=fam: family_manifest(text, fam, extras[fam])
    for root in ROOTS:
        artifact = root.split("️", 1)[1] if "️" in root else root
        artifact = re.sub(r"^[^a-z]+", "", root)
        body = ROOT_BODIES.get(root, IFC_BODY if root == "🏗️ifc" else SEMIO_BODY)
        edits[f"{ART}/{root}/🦀️.rs"] = lambda text, root=root, artifact=artifact, body=body: root_docs(runtime_root(text, artifact, body, root), root)
    edits[f"{ART}/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🚪️io/🦀️.rs"] = binary_io_doc
    for entry in sorted(os.listdir(os.path.join(TREE, SEMIO_SUBSETS))):
        path = f"{SEMIO_SUBSETS}/{entry}/🚪️io/🦀️.rs"
        if os.path.isfile(os.path.join(TREE, path)):
            name = re.sub(r"^[^a-z]+", "", entry)
            edits[path] = lambda text, name=name, entry=entry: subset_declare(text, name, entry)
    for path in IFC_MVDS:
        edits[path] = lambda text, path=path: mvd_declare(text, path.split("/")[-3])
    if not SKIP_JSON:
        rows = json.load(open(ROWS, encoding="utf-8"))
        for root in ROOTS:
            edits[f"{ART}/{root}/📜️artifact-definition.json"] = lambda text, root=root: definition_rows(text, root, rows[root])
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
        full = os.path.join(TREE, path)
        if not os.path.isfile(full):
            problems.append(f"missing {path}")
            continue
        before = open(full, encoding="utf-8").read()
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
