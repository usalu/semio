"""📨️ W2-W-document: converts the DOCX strict/transitional conformance-class case adapters to the generic wire decoder
(design §11, F10). Rows decode through `decode_docx_<class>_mutation_payload`, the oracle's computed undo spec too; the
`no-mutation` baseline scenarios are gone (the identity round trip is the baseline); the whole-package class stamp, whose
`set-snapshot` payload is the entire stamped package and so no table cell, becomes the `stamp-conformance-class`
scenario pair: the subject records `stamp_conformance_class_mutation`, the reference stamps with its own engine."""
import pathlib, re

ROOT = pathlib.Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets")
SUBSETS = {"📏️strict": ("strict", "DocxStrictMutation", "apply_docx_strict_mutation", "decode_docx_strict_mutation_payload"), "🔄️transitional": ("transitional", "DocxTransitionalMutation", "apply_docx_transitional_mutation", "decode_docx_transitional_mutation_payload")}

ORACLE_STAMP = '''
/// 🏅️ The reference half of the whole-package class stamp: the independent engine stamps the real package strict, and
/// the conformance-class projection has to move.
fn stamp_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let output = oracle_stamp(&input, true)?;
    let projection = project_package(&output)?;
    mutation_is_observable("stamp-conformance-class", &projection, &project_package(&input)?, &[])?;
    Ok(Outcome::with_raw(output, projection))
}

/// ↩️ The class stamp undone by stamping back, which must restore the real package's projection exactly.
fn stamp_inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let restored = oracle_stamp(&oracle_stamp(&input, true)?, false)?;
    let projection = project_package(&restored)?;
    inverse_restores("stamp-conformance-class", &projection, &project_package(&input)?)?;
    Ok(Outcome::with_raw(restored, projection))
}
//#endregion 🔖️Oracle
'''

SUBJECT = '''mod subject {{
    use super::{{arranged_input, mutable_input}};
    use semio_repo_test_host::{{Context, Json, Outcome}};
    use semio_s_artifact_stdio_docx::standards::v_ecma_376::subsets::any::io::export::serializers::encode_docx;
    use semio_s_artifact_stdio_docx::standards::v_ecma_376::subsets::any::io::import::deserializers::decode_docx;
    use semio_s_artifact_stdio_docx::standards::v_ecma_376::subsets::{cls}::schema::mutations::{{{apply}, {decode}, stamp_conformance_class_mutation, {ty}}};
    use semio_s_artifact_stdio_docx::DocxSnapshot;
    use semio_s_plugin_stdio_test_oracle::artifacts::docx::standards::v_ecma_376::subsets::{cls}::{{oracle_inverse_spec, project_package}};

    fn decode(bytes: &[u8]) -> Result<DocxSnapshot, String> {{
        decode_docx(bytes).map_err(|error| error.to_string())
    }}

    fn encode(snapshot: &DocxSnapshot) -> Result<Vec<u8>, String> {{
        encode_docx(snapshot).map_err(|error| error.to_string())
    }}

    /// 📨️ The scenario's `{{kind, params}}` row — or the oracle's computed undo spec — decoded generically: `params` is the
    /// leaf wire payload, the only channel between the feature and the subject's typed `{ty}`.
    fn mutation_from_spec(spec: &Json) -> Result<{ty}, String> {{
        {decode}(&spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null).to_string())
    }}

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {{
        let spec = ctx.doc_json()?;
        let mut snapshot = decode(&arranged_input(ctx, &spec)?)?;
        {apply}(&mut snapshot, &mutation_from_spec(&spec)?);
        let output = encode(&snapshot)?;
        let projection = project_package(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }}

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {{
        let spec = ctx.doc_json()?;
        let base = arranged_input(ctx, &spec)?;
        let mut snapshot = decode(&base)?;
        {apply}(&mut snapshot, &mutation_from_spec(&spec)?);
        let undo = oracle_inverse_spec(&base, &spec)?;
        {apply}(&mut snapshot, &mutation_from_spec(&undo)?);
        let output = encode(&snapshot)?;
        let projection = project_package(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }}

    /// 🏅️ The whole-package class stamp recorded as one `set-snapshot` of this repository's own stamp.
    pub fn stamp(ctx: &Context) -> Result<Outcome, String> {{
        let mut snapshot = decode(&mutable_input(ctx)?)?;
        let stamp = stamp_conformance_class_mutation(&snapshot, true);
        {apply}(&mut snapshot, &stamp);
        let output = encode(&snapshot)?;
        let projection = project_package(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }}

    /// ↩️ The class stamp undone by the stamp back, recorded the same way.
    pub fn stamp_inverse(ctx: &Context) -> Result<Outcome, String> {{
        let mut snapshot = decode(&mutable_input(ctx)?)?;
        let stamp = stamp_conformance_class_mutation(&snapshot, true);
        {apply}(&mut snapshot, &stamp);
        let back = stamp_conformance_class_mutation(&snapshot, false);
        {apply}(&mut snapshot, &back);
        let output = encode(&snapshot)?;
        let projection = project_package(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }}

    /// 🔁️ Full semantic parse, re-serialized from the model alone — copying, splicing or patching
    /// source bytes is cheating, and this tripwire catches it.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {{
        let input = mutable_input(ctx)?;
        let snapshot = decode(&input)?;
        let output = encode(&snapshot)?;
        if output == input {{
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }}
        let projection = project_package(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }}
}}
'''

REGISTRATION = '''//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    built = built.oracle("mutate", mutate_oracle).oracle("inverse", inverse_oracle).oracle("stamp-conformance-class", stamp_oracle).oracle("stamp-conformance-class-inverse", stamp_inverse_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("mutate", subject::mutate).subject("inverse", subject::inverse).subject("stamp-conformance-class", subject::stamp).subject("stamp-conformance-class-inverse", subject::stamp_inverse);
    }
    built = built.oracle("identity-round-trip", round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
'''

for folder, (cls, ty, apply, decode) in SUBSETS.items():
    path = next((ROOT / folder / "🧪️tests").glob("*/🦀️.rs"))
    text = path.read_text()
    if "stamp_conformance_class_mutation" in text:
        print("unchanged", folder)
        continue
    text = text.replace(f"subsets::{cls}::{{oracle_apply_mutation, oracle_arrange, oracle_inverse_spec, oracle_round_trip, project_package}};", f"subsets::{cls}::{{oracle_apply_mutation, oracle_arrange, oracle_inverse_spec, oracle_round_trip, oracle_stamp, project_package}};", 1)
    assert "oracle_stamp" in text, folder
    text = text.replace("//#endregion 🔖️Oracle\n", ORACLE_STAMP.lstrip("\n"), 1)
    start = text.index("mod subject {")
    end = text.index("//#endregion 🔖️Subject")
    text = text[:start] + SUBJECT.format(cls=cls, ty=ty, apply=apply, decode=decode) + text[end:]
    start = text.index("//#region 🔖️Registration")
    end = text.index("//#endregion 🔖️Registration") + len("//#endregion 🔖️Registration\n")
    text = text[:start] + REGISTRATION + text[end:]
    path.write_text(text)
    print("converted", folder)
