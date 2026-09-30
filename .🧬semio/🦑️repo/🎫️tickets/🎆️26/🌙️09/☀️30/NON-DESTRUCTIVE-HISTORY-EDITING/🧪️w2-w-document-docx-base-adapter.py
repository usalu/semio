"""📨️ W2-W-document: the `📜️mutate-docx-ecma-376` Rust adapter decodes every row through `decode_docx_mutation_payload`
(design §11, F10); the hand-mapped block/style/path/run codec is deleted; the `no-mutation` baselines are gone (the
identity round trip is the baseline, now the reference's own `oracle_round_trip`); `set-snapshot` runs as its own
scenario pair whose payload is the committed after-document's decoded snapshot."""
import pathlib

PATH = pathlib.Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧪️tests/📜️mutate-docx-ecma-376/🦀️.rs")

SUBJECT = '''mod subject {
    use super::{mutable_input, set_snapshot_document};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_docx::standards::v_ecma_376::subsets::any::io::export::serializers::encode_docx;
    use semio_s_artifact_stdio_docx::standards::v_ecma_376::subsets::any::io::import::deserializers::decode_docx;
    use semio_s_artifact_stdio_docx::standards::v_ecma_376::subsets::any::schema::mutations::{apply_docx_mutation, decode_docx_mutation_payload, set_snapshot};
    use semio_s_artifact_stdio_docx::{DocxMutation, DocxSnapshot};
    use semio_s_plugin_stdio_test_oracle::artifacts::docx::standards::v_ecma_376::subsets::base::project_docx_ecma_376;

    fn decode(bytes: &[u8]) -> Result<DocxSnapshot, String> {
        decode_docx(bytes).map_err(|error| format!("decode_docx failed: {error}"))
    }

    fn encode(snapshot: &DocxSnapshot) -> Result<Vec<u8>, String> {
        encode_docx(snapshot).map_err(|error| format!("encode_docx failed: {error}"))
    }

    /// 📨️ The scenario's `{kind, params}` row decoded generically: `params` is the leaf wire payload, the only channel
    /// between the feature and the subject's typed `DocxMutation`.
    fn mutation_from_spec(spec: &Json) -> Result<DocxMutation, String> {
        decode_docx_mutation_payload(&spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null).to_string())
    }

    /// ↩️ The whole-document undo every inverse scenario lands on: `set-snapshot` back to the untouched base.
    fn restore(base: &DocxSnapshot) -> DocxMutation {
        DocxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })
    }

    /// 📦️ The produced package as the `actual-docx` artifact the `docx-ecma-376-jszip-compare-v1` pipeline reads.
    fn actual(ctx: &Context, bytes: Vec<u8>) -> Result<Outcome, String> {
        let projection = project_docx_ecma_376(&bytes)?;
        let path = ctx.artifact("actual-docx", "actual.docx")?;
        std::fs::write(&path, &bytes).map_err(|error| error.to_string())?;
        Ok(Outcome::with_raw(bytes, projection).artifact("actual-docx", &path, "application/vnd.openxmlformats-officedocument.wordprocessingml.document"))
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let mut snapshot = decode(&mutable_input(ctx)?)?;
        apply_docx_mutation(&mut snapshot, &mutation_from_spec(&ctx.doc_json()?)?);
        actual(ctx, encode(&snapshot)?)
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let base = decode(&mutable_input(ctx)?)?;
        let mut snapshot = base.clone();
        apply_docx_mutation(&mut snapshot, &mutation_from_spec(&ctx.doc_json()?)?);
        apply_docx_mutation(&mut snapshot, &restore(&base));
        actual(ctx, encode(&snapshot)?)
    }

    /// 📸️ The whole document replaced by the snapshot this repository's own codec decodes from the committed
    /// after-document — a `set-snapshot` payload is an entire package, not a table cell.
    pub fn set_snapshot(ctx: &Context) -> Result<Outcome, String> {
        let mut snapshot = decode(&mutable_input(ctx)?)?;
        apply_docx_mutation(&mut snapshot, &DocxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: decode(&set_snapshot_document(ctx)?)? }));
        actual(ctx, encode(&snapshot)?)
    }

    pub fn set_snapshot_inverse(ctx: &Context) -> Result<Outcome, String> {
        let base = decode(&mutable_input(ctx)?)?;
        let mut snapshot = base.clone();
        apply_docx_mutation(&mut snapshot, &DocxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: decode(&set_snapshot_document(ctx)?)? }));
        apply_docx_mutation(&mut snapshot, &restore(&base));
        actual(ctx, encode(&snapshot)?)
    }

    /// 🔒️ The no-byte-pass-through rule: the subject must fully parse the real artifact into its
    /// typed snapshot and re-serialize from the model alone -- `decode_docx`/`encode_docx` are this
    /// subset's ONLY channel from input to output.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let output = encode(&decode(&input)?)?;
        if output == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        actual(ctx, output)
    }
}
'''

REGISTRATION = '''//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Handlers are registered under the Scenario Outline
/// base ids, which the host resolves for every Examples row, and plain scenarios under their own ids.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    built = built.oracle("mutate", mutate_oracle).oracle("inverse", inverse_oracle).oracle("set-snapshot", set_snapshot_oracle).oracle("set-snapshot-inverse", set_snapshot_inverse_oracle);
    built = built.oracle("identity-round-trip", identity_round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("mutate", subject::mutate).subject("inverse", subject::inverse).subject("set-snapshot", subject::set_snapshot).subject("set-snapshot-inverse", subject::set_snapshot_inverse);
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
'''

INPUT_OLD = '''/// 🈳️ The `no-mutation` spec, which is how the identity round trip asks the reference composition to
/// unzip, parse, re-serialize and rezip the real package without changing anything.
fn no_mutation() -> Json {
    Json::Object(vec![("kind".to_string(), Json::String("no-mutation".to_string())), ("params".to_string(), Json::Object(vec![]))])
}
'''
INPUT_NEW = '''/// 📸️ The committed after-document whose decoded snapshot the `set-snapshot` scenarios replace the README with.
fn set_snapshot_document(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture("shared://🧾️readme-afters/📸️set-snapshot/➡️after.docx", Some("set-snapshot-after.docx"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
'''

ORACLE_OLD = '''    let input = mutable_input(ctx)?;
    let bytes = oracle_apply_mutation(&input, &no_mutation())?;'''
ORACLE_NEW = '''    let input = mutable_input(ctx)?;
    let bytes = oracle_round_trip(&input)?;'''

SNAPSHOT_ORACLES = '''
/// 📸️ The reference side of the whole-document replacement: the committed after-document read and re-written by the
/// `zip`+`quick-xml` composition in place of the README, which has to move the compared projection.
fn set_snapshot_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let bytes = oracle_replace_package(&input, &set_snapshot_document(ctx)?)?;
    let projection = project_docx_ecma_376(&bytes)?;
    mutation_is_observable("set-snapshot", &projection, &project_docx_ecma_376(&input)?, &[])?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ The replacement undone by replacing back, which must land on the untouched README's projection.
fn set_snapshot_inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let bytes = oracle_replace_package(&oracle_replace_package(&input, &set_snapshot_document(ctx)?)?, &input)?;
    let projection = project_docx_ecma_376(&bytes)?;
    inverse_restores("set-snapshot", &projection, &project_docx_ecma_376(&input)?)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle
'''

text = PATH.read_text()
if "set_snapshot_document" in text:
    raise SystemExit("already converted")
for old, new in [
    ("use semio_s_plugin_stdio_test_oracle::artifacts::docx::standards::v_ecma_376::subsets::base::{oracle_apply_mutation, oracle_apply_mutation_inverse, project_docx_ecma_376};", "use semio_s_plugin_stdio_test_oracle::artifacts::docx::standards::v_ecma_376::subsets::base::{oracle_apply_mutation, oracle_apply_mutation_inverse, oracle_replace_package, oracle_round_trip, project_docx_ecma_376};"),
    (INPUT_OLD, INPUT_NEW),
    (ORACLE_OLD, ORACLE_NEW),
    ("//#endregion 🔖️Oracle\n", SNAPSHOT_ORACLES.lstrip("\n")),
]:
    assert text.count(old) == 1, old[:80]
    text = text.replace(old, new)
start = text.index("mod subject {")
end = text.index("//#endregion 🔖️Subject")
text = text[:start] + SUBJECT + text[end:]
start = text.index("//#region 🔖️Registration")
end = text.index("//#endregion 🔖️Registration") + len("//#endregion 🔖️Registration\n")
text = text[:start] + REGISTRATION + text[end:]
PATH.write_text(text)
print("converted")
