#!/usr/bin/env python3
"""🪪️ W4 (14c → window 3, landed by L1 in T3): ONE schema identity per document kind.

The trusted-catalog publish refused 9 packages ("trusted <plugin> closure carries no artifact codec"): their
`ArtifactKindSpec.schema` named a "media schema" that no app's DOCUMENT_SCHEMA (nor dialect kind) equals, so the guest
answered every declared kind as unowned. Every platform reader keys a document kind on ONE schema — hub codec rows,
document-open targets and genesis (`🌎️hub/🗿️artifact-authority`), the MCP workspace store (`storage.write(id,
&kind.schema, …)`), host-media contributions (`artifact_kind.schema == artifact_schema`) — and each app's own `io` already
declares that identity (`ArtifactPresentation.id` + `artifact_schema` = DOCUMENT_SCHEMA). So each document kind spec now
carries its app's DOCUMENT_SCHEMA constant; the former media string stays declared as `source_format` (nothing lost);
the four laws that pinned the split now pin equality; fem gains its document kinds (`2d.fem`/`3d.fem`, the ids its io
already presents) beside the unchanged results kinds; shooting's editor io joins its viewer on SHOOTING_DOCUMENT_SCHEMA;
forms' editor drops its inline duplicate of `crate::artifact_kind()`; trinity's rewriting io presents its declared kind
(`crate::artifact_kind().id` = `text.rewriting`, was an undeclared `trinity.rewriting`).

usage: python3 w4-kind-spec-identity.py --dry-run | --write | --revert
  --dry-run  every edit reported as pending / applied / MISSING (exit 1 on MISSING or ambiguity)
  --write    applies every pending edit (refuses when any edit is MISSING or ambiguous; idempotent)
  --revert   restores every applied edit
"""
import os, re, sys

ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins"
LAW_WHY = ("a document kind has ONE schema identity — the hub's codec rows, document-open targets and genesis, the MCP workspace\n"
           "/// store and host-media contributions all key on it (ticket 26/09/23 W4: a distinct \"media schema\" left the package without a\n"
           "/// codec owner, so the trusted catalog refused it).")


def law(test_name, kind_expr, const, source, kept="The former media string stays declared as `source_format`."):
    return (f"/// 🪪️ `artifact_kind().schema` IS `{const}`: {LAW_WHY} {kept}\n"
            f"#[semio_framework_async_macros::async_test]\n"
            f"async fn {test_name}() {{\n"
            f"    assert_eq!({kind_expr}.schema, {const});\n"
            f"    assert_eq!({kind_expr}.source_format, \"{source}\");\n"
            f"}}\n")


def fem_document_kind(dim, schema_const, media, component, stdio_export, stdio_import):
    return (f"/// 🪪️ The fem {dim.upper()} model document kind — the id `fem{dim}_io`'s `ArtifactPresentation` already names (`{dim}.fem`),\n"
            f"/// with the ONE schema the hub's codec rows, document-open targets and genesis key on (`{schema_const}`). Declared\n"
            f"/// beside [`computation_artifact_kind`], which stays the results kind `results:out` pins itself to.\n"
            f"pub fn document_artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {{\n"
            f"    semio_framework_plugin::ArtifactKindSpec {{\n"
            f"        id: \"{dim}.fem\".into(),\n"
            f"        name: \"FEM {dim.upper()}\".into(),\n"
            f"        source_format: {schema_const}.into(),\n"
            f"        component_kind: \"{component}\".into(),\n"
            f"        dimension: \"{dim}\".into(),\n"
            f"        media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,\n"
            f"        media_type: semio_framework_plugin::MediaType {{ class: semio_framework_plugin::MediaClass::{media[0]}, form: semio_framework_plugin::MediaForm::{media[1]} }},\n"
            f"        schema: {schema_const}.into(),\n"
            f"        export_formats: vec![],\n"
            f"        import_formats: vec![],\n"
            f"        export_stdio_kinds: vec![{stdio_export}],\n"
            f"        import_stdio_kinds: vec![{stdio_import}],\n"
            f"    }}\n"
            f"}}\n\n")


FEM_EXPORT = '"stdio.csv".into(), "stdio.json".into(), "stdio.obj".into(), "stdio.stl".into(), "stdio.txt".into()'
FEM_IMPORT = '"stdio.json".into(), "stdio.txt".into()'
FORMS_INLINE = ('            .artifact_kind(ArtifactKindSpec {\n                id: "form.dictionary".into(),\n                name: "Form Dictionary".into(),\n'
                '                source_format: "form.dictionary".into(),\n                component_kind: "forms".into(),\n                dimension: "data".into(),\n'
                '                media_capability: OsMediaCapability::MeshOnly,\n                media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },\n'
                '                schema: "form.dictionary".into(),\n                export_formats: vec![],\n                import_formats: vec![],\n'
                '                    export_stdio_kinds: vec![],\n        import_stdio_kinds: vec![],\n    })\n')
FORMS_EDITOR = "📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"
SHOOTING_EDITOR = "🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
FEM2 = "🏗️fem/🗿️artifacts/◻️2d"
FEM3 = "🏗️fem/🗿️artifacts/🧊️3d"
FEM2_EDITOR = f"{FEM2}/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs"
FEM3_EDITOR = f"{FEM3}/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs"

EDITS = [
    ("🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs", '        schema: "flow.artifact".into(),\n', '        schema: FLOW_DOCUMENT_SCHEMA.into(),\n'),
    ("🌊️flow/🗿️artifacts/🌊️flow/🧪️tests/🔬️unit/🦀️.rs",
     '/// 🗂️ The manifest-facing `ArtifactKindSpec.schema` ("flow.artifact") is deliberately NOT\n/// `FLOW_DOCUMENT_SCHEMA` ("flow.host_snapshot") — the former names the artifact kind in the OS media\n'
     '/// catalogue, the latter keys the store envelope. Pinned so a future edit can\'t silently merge them.\n#[semio_framework_async_macros::async_test]\n'
     'async fn artifact_kind_keeps_the_media_schema_distinct_from_the_store_schema() {\n    assert_eq!(artifact_kind().schema, "flow.artifact");\n    assert_eq!(FLOW_DOCUMENT_SCHEMA, "flow.host_snapshot");\n}\n',
     law("artifact_kind_names_the_store_schema", "artifact_kind()", "FLOW_DOCUMENT_SCHEMA", "flow.artifact")),
    ("➗️mathematical/🗿️artifacts/➗️equation/🦀️.rs", '        schema: "computation.equation".into(),\n', '        schema: MATH_DOCUMENT_SCHEMA.into(),\n'),
    ("➗️mathematical/🗿️artifacts/➗️equation/🧪️tests/🔬️unit/🦀️.rs",
     '#[semio_framework_async_macros::async_test]\nasync fn artifact_kind_keeps_the_media_schema_distinct_from_the_store_schema() {\n    assert_eq!(artifact_kind().schema, "computation.equation");\n    assert_eq!(MATH_DOCUMENT_SCHEMA, "semio.equation/v1");\n}\n',
     law("artifact_kind_names_the_store_schema", "artifact_kind()", "MATH_DOCUMENT_SCHEMA", "semio.equation/v1", "The former media string was the kind id\n/// itself (`computation.equation`), which stays.")),
    ("🎥️shooting/🗿️artifacts/🎥️shooting/🦀️.rs", '        schema: "shooting.scene".into(),\n', '        schema: SHOOTING_DOCUMENT_SCHEMA.into(),\n'),
    ("🎥️shooting/🗿️artifacts/🎥️shooting/🧪️tests/🔬️unit/🦀️.rs",
     '/// 🗂️ The manifest-facing `ArtifactKindSpec.schema` ("shooting.scene") is deliberately NOT\n/// `SHOOTING_DOCUMENT_SCHEMA` ("shooting.shooting") — the former names the artifact kind in the OS\n'
     '/// media catalogue, the latter keys the store envelope. Pinned so a future edit can\'t silently\n/// merge them.\n#[semio_framework_async_macros::async_test]\n'
     'async fn artifact_kind_keeps_the_media_schema_distinct_from_the_store_schema() {\n    assert_eq!(artifact_kind().schema, "shooting.scene");\n    assert_eq!(SHOOTING_DOCUMENT_SCHEMA, "shooting.shooting");\n}\n',
     law("artifact_kind_names_the_store_schema", "artifact_kind()", "SHOOTING_DOCUMENT_SCHEMA", "shooting.scene")),
    (f"{SHOOTING_EDITOR}/🦀️.rs", '        artifact_schema: "shooting.scene".into(),\n', '        artifact_schema: crate::SHOOTING_DOCUMENT_SCHEMA.into(),\n'),
    (f"{SHOOTING_EDITOR}/🧪️tests/🔬️unit/🦀️.rs", '    assert_eq!(io.artifact_schema, "shooting.scene");\n', '    assert_eq!(io.artifact_schema, crate::SHOOTING_DOCUMENT_SCHEMA);\n'),
    ("💠️lowpoly/🗿️artifacts/💠️lowpoly/🦀️.rs", '        schema: "lowpoly.fixture".into(),\n', '        schema: LOWPOLY_DOCUMENT_SCHEMA.into(),\n'),
    ("📋️forms/🗿️artifacts/📋️forms/🦀️.rs", '        schema: "form.dictionary".into(),\n', '        schema: FORMS_DOCUMENT_SCHEMA.into(),\n'),
    ("📋️forms/🗿️artifacts/📋️forms/🧪️tests/🔬️unit/🦀️.rs",
     '#[semio_framework_async_macros::async_test]\nasync fn artifact_kind_uses_the_dictionary_media_kind_as_both_id_and_schema() {\n    assert_eq!(artifact_kind().id, "form.dictionary");\n    assert_eq!(artifact_kind().schema, "form.dictionary");\n    assert_eq!(FORMS_DOCUMENT_SCHEMA, "forms.form");\n}\n',
     law("artifact_kind_names_the_store_schema", "artifact_kind()", "FORMS_DOCUMENT_SCHEMA", "form.dictionary")),
    (FORMS_EDITOR, FORMS_INLINE, '            .artifact_kind(crate::artifact_kind())\n'),
    (FORMS_EDITOR, "ArtifactEditor, ArtifactKindSpec, ArtifactOwnedToolJobRequest", "ArtifactEditor, ArtifactOwnedToolJobRequest"),
    (FORMS_EDITOR, "NoDraftMutation, OsMediaCapability, SelectionMethod", "NoDraftMutation, SelectionMethod"),
    ("📜️imperative/🗿️artifacts/📜️procedure/🦀️.rs", '        schema: "procedure.document".into(),\n', '        schema: PROCEDURE_DOCUMENT_SCHEMA.into(),\n'),
    ("📜️imperative/🗿️artifacts/📜️procedure/🧪️tests/🔬️unit/🦀️.rs",
     '/// 🗂️ The manifest-facing `ArtifactKindSpec.schema` ("procedure.document") is deliberately NOT\n/// `PROCEDURE_DOCUMENT_SCHEMA` ("procedure.document/v1") — the former names the artifact kind in\n'
     '/// the OS media catalogue, the latter keys the store envelope. Pinned so a future edit can\'t silently\n/// merge them.\n#[semio_framework_async_macros::async_test]\n'
     'async fn artifact_kind_keeps_the_media_schema_distinct_from_the_store_schema() {\n    assert_eq!(artifact_kind().schema, "procedure.document");\n    assert_eq!(PROCEDURE_DOCUMENT_SCHEMA, "procedure.document/v1");\n}\n',
     law("artifact_kind_names_the_store_schema", "artifact_kind()", "PROCEDURE_DOCUMENT_SCHEMA", "procedure.document")),
    ("🪵️sourcing/🗿️artifacts/🗂️curation/🦀️.rs", '        schema: "sourcing.curation".into(),\n', '        schema: SOURCING_CURATION_SCHEMA.into(),\n'),
    ("🪵️sourcing/🗿️artifacts/🗂️curation/🧪️tests/🔬️unit/🦀️.rs",
     '/// 🗂️ The manifest-facing `ArtifactKindSpec.schema` ("sourcing.curation") is deliberately NOT\n/// `SOURCING_CURATION_SCHEMA` ("sourcing.curation/v1") — the former names the artifact kind in the OS\n'
     '/// media catalogue, the latter keys the store envelope. Pinned so a future edit can\'t silently\n/// merge them (mirrors `flow`\'s identical `artifact_kind` split-schema pin).\n#[semio_framework_async_macros::async_test]\n'
     'async fn artifact_kind_keeps_the_media_schema_distinct_from_the_store_schema() {\n    assert_eq!(artifact_kind().schema, "sourcing.curation");\n    assert_eq!(SOURCING_CURATION_SCHEMA, "sourcing.curation/v1");\n}\n',
     law("artifact_kind_names_the_store_schema", "artifact_kind()", "SOURCING_CURATION_SCHEMA", "sourcing.curation")),
    ("📕️norm/🖥️app-surface/🦀️.rs",
     '/// 🗿️ A norm artifact kind — Data × Value document per owner-table (IO coverage lattice).\npub fn artifact_kind_spec(variant: &str, label: &str) -> ArtifactKindSpec {\n',
     '/// 🗿️ A norm artifact kind — Data × Value document per owner-table (IO coverage lattice). `artifact_schema` is the\n'
     '/// family\'s DOCUMENT_SCHEMA — the ONE schema identity the hub\'s codec rows, open targets and genesis key on (the same\n'
     '/// value [`norm_io`] declares); the former `norm.<variant>.document` media string stays declared as `source_format`.\n'
     'pub fn artifact_kind_spec(variant: &str, label: &str, artifact_schema: &str) -> ArtifactKindSpec {\n'),
    ("📕️norm/🖥️app-surface/🦀️.rs", '        schema: format!("norm.{variant}.document"),\n', '        schema: artifact_schema.into(),\n'),
    ("📕️norm/🖥️app-surface/🧪️tests/🔬️unit/🦀️.rs",
     '    let spec = artifact_kind_spec("en1990", "EN 1990");\n    assert_eq!(spec.id, "computation.norm.en1990");\n    assert_eq!(spec.source_format, "norm.en1990.document");\n',
     '    let spec = artifact_kind_spec("en1990", "EN 1990", "semio.norm.en1990/v1");\n    assert_eq!(spec.id, "computation.norm.en1990");\n    assert_eq!(spec.schema, "semio.norm.en1990/v1");\n    assert_eq!(spec.source_format, "norm.en1990.document");\n'),
    (f"{FEM2}/🦀️.rs", "// #region 🔖️ArtifactKind\n/// 🔌️ The computed-results output artifact kind",
     "// #region 🔖️ArtifactKind\n" + fem_document_kind("2d", "FEM_2D_SCHEMA", ("TwoD", "Vector"), "fem2d", FEM_EXPORT, FEM_IMPORT) + "/// 🔌️ The computed-results output artifact kind"),
    (f"{FEM3}/🦀️.rs", "// #region 🔖️ArtifactKind\n/// 🏷️ The `computation.fem3d` artifact kind",
     "// #region 🔖️ArtifactKind\n" + fem_document_kind("3d", "FEM_3D_SCHEMA", ("ThreeD", "Any"), "fem3d", FEM_EXPORT, FEM_IMPORT) + "/// 🏷️ The `computation.fem3d` artifact kind"),
    (FEM2_EDITOR, "            .artifact_kind(crate::computation_artifact_kind())\n", "            .artifact_kind(crate::document_artifact_kind())\n            .artifact_kind(crate::computation_artifact_kind())\n"),
    (FEM3_EDITOR, "            .artifact_kind(crate::computation_artifact_kind())\n", "            .artifact_kind(crate::document_artifact_kind())\n            .artifact_kind(crate::computation_artifact_kind())\n"),
    ("🏗️fem/🦀️.rs",
     "        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_fem_3d::computation_artifact_kind().id })\n",
     "        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_fem_3d::computation_artifact_kind().id })\n"
     "        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_fem_2d::document_artifact_kind().id })\n"
     "        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_fem_3d::document_artifact_kind().id })\n"),    ("🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
     'artifact: semio_framework_plugin::ArtifactPresentation { id: "trinity.rewriting".into(),',
     'artifact: semio_framework_plugin::ArtifactPresentation { id: crate::artifact_kind().id,'),
]

NORM_CALL = re.compile(r'app_surface::artifact_kind_spec\("([a-z0-9]+)", "([^"]+)"(?:, [A-Z0-9_]+)?\)')
norm_artifacts = os.path.join(ROOT, "📕️norm", "🗿️artifacts")
for name in sorted(os.listdir(norm_artifacts)):
    rel = f"📕️norm/🗿️artifacts/{name}/🦀️.rs"
    path = os.path.join(ROOT, rel)
    if not os.path.isfile(path):
        continue
    text = open(path, encoding="utf-8").read()
    for variant, label in NORM_CALL.findall(text):
        const = f"{variant.upper()}_DOCUMENT_SCHEMA"
        if f"pub const {const}: &str" not in text:
            sys.exit(f"MISSING {const} in {rel}")
        EDITS.append((rel, f'app_surface::artifact_kind_spec("{variant}", "{label}")', f'app_surface::artifact_kind_spec("{variant}", "{label}", {const})'))


def state(text, old, new):
    if old in new and new in text:
        return "applied" if text.count(new) == 1 else "AMBIGUOUS"
    if new in text and old not in text:
        return "applied"
    count = text.count(old)
    return "pending" if count == 1 else ("MISSING" if count == 0 else "AMBIGUOUS")


def main():
    mode = sys.argv[1] if len(sys.argv) > 1 else "--dry-run"
    if mode not in ("--dry-run", "--write", "--revert"):
        sys.exit(__doc__)
    texts, bad = {}, 0
    for rel, old, new in EDITS:
        path = os.path.join(ROOT, rel)
        texts.setdefault(rel, open(path, encoding="utf-8").read())
    plan = []
    for rel, old, new in EDITS:
        s = state(texts[rel], old, new)
        plan.append((rel, old, new, s))
        bad += s in ("MISSING", "AMBIGUOUS")
        print(f"{s:9} {rel} :: {old.strip().splitlines()[0][:90]}")
    if mode == "--dry-run" or bad:
        print(f"{len(EDITS)} edits, {sum(p[3] == 'pending' for p in plan)} pending, {sum(p[3] == 'applied' for p in plan)} applied, {bad} bad")
        sys.exit(1 if bad else 0)
    for rel, old, new, s in plan:
        if mode == "--write" and s == "pending":
            texts[rel] = texts[rel].replace(old, new, 1)
        elif mode == "--revert" and s == "applied":
            texts[rel] = texts[rel].replace(new, old, 1)
    for rel, text in texts.items():
        path = os.path.join(ROOT, rel)
        if open(path, encoding="utf-8").read() != text:
            open(path, "w", encoding="utf-8").write(text)
            print(f"wrote {rel}")


if __name__ == "__main__":
    main()
