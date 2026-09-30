"""🏅️ W2-W-office: converts the xlsx/pptx strict + transitional conformance-class cases to wire witnesses (design §11, F10).

Adapters: every `{kind, params}` row decodes through `Mutation::from_payload_value` (crate-root re-export) and must re-emit
exactly (`law::params_are_wire`); the subject undoes rows with `Mutation::inverse`; the `no-mutation` baselines are gone
(the identity round trip is the baseline); the whole-package class stamp — a `set-snapshot` whose payload is the entire
stamped package, so no table cell — is the plain `mutate-set-snapshot`/`inverse-set-snapshot` scenario pair: the subject
records `stamp_conformance_class_mutation`, the reference stamps with the shared engine (`oracle_stamp`).
Oracle modules: `KINDS` drops `no-mutation`/`set-snapshot`, gains `oracle_stamp`; unit tests mirror the rows.
Production: `stamp_conformance_class_mutation` per aggregate. Features: rows → wire, sentinels removed, stamp pair added.

  python3 🧪️w2-w-office-conformance.py
"""
import pathlib

STDIO = pathlib.Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts")
VML = '<xml xmlns:v="urn:schemas-microsoft-com:vml"><v:shape id="legacyShape" type="#_x0000_t202"/></xml>'
SUBSETS = [
    # artifact, subset dir, module, crate, aggregate, snapshot, apply, codec module, encode, decode, noun, stamped noun
    ("📕️xlsx", "🔒️strict", "strict", "xlsx", "XlsxStrictMutation", "XlsxSnapshot", "apply_xlsx_strict_mutation", "encode_xlsx", "decode_xlsx", "workbook package"),
    ("📕️xlsx", "🌉️transitional", "transitional", "xlsx", "XlsxTransitionalMutation", "XlsxSnapshot", "apply_xlsx_transitional_mutation", "encode_xlsx", "decode_xlsx", "workbook package"),
    ("📽️pptx", "🔒️strict", "strict", "pptx", "PptxStrictMutation", "PptxSnapshot", "apply_pptx_strict_mutation", "encode_pptx", "decode_pptx", "presentation package"),
    ("📽️pptx", "🌉️transitional", "transitional", "pptx", "PptxTransitionalMutation", "PptxSnapshot", "apply_pptx_transitional_mutation", "encode_pptx", "decode_pptx", "presentation package"),
]

ORACLE_STAMP = '''
/// 🏅️ The reference half of the whole-package class stamp (`set-snapshot` of the package's own strict stamp): the
/// independent engine stamps the real package strict, and the conformance-class projection has to move.
fn stamp_oracle(ctx: &Context) -> Result<Outcome, String> {{
    let input = mutable_input(ctx)?;
    let output = oracle_stamp(&input, true)?;
    let projection = project_package(&output)?;
    mutation_is_observable("set-snapshot", &projection, &project_package(&input)?, &[])?;
    Ok(Outcome::with_raw(output, projection))
}}

/// ↩️ The class stamp undone by the reference's own stamp back, which must restore the real package's projection exactly.
fn stamp_inverse_oracle(ctx: &Context) -> Result<Outcome, String> {{
    let input = mutable_input(ctx)?;
    let restored = oracle_stamp(&oracle_stamp(&input, true)?, false)?;
    let projection = project_package(&restored)?;
    inverse_restores("set-snapshot", &projection, &project_package(&input)?)?;
    Ok(Outcome::with_raw(restored, projection))
}}
//#endregion 🔖️Oracle
'''

SUBJECT = '''mod subject {{
    use super::{{arranged_input, mutable_input}};
    use semio_repo_test_host::{{Context, Json, Outcome}};
    use semio_s_artifact_stdio_{crate}::standards::v_ecma_376::subsets::base::io::export::serializers::{encode};
    use semio_s_artifact_stdio_{crate}::standards::v_ecma_376::subsets::base::io::import::deserializers::{decode};
    use semio_s_artifact_stdio_{crate}::standards::v_ecma_376::subsets::{module}::schema::mutations::{{{apply}, stamp_conformance_class_mutation, {agg}}};
    use semio_s_artifact_stdio_{crate}::{{from_json_str, to_json_string, DslValue, Mutation, {snap}}};
    use semio_s_plugin_stdio_test_oracle::artifacts::{crate}::standards::v_ecma_376::subsets::{module}::project_package;
    use semio_s_plugin_stdio_test_oracle::law::params_are_wire;

    fn decode(bytes: &[u8]) -> Result<{snap}, String> {{
        {decode}(bytes).map_err(|error| error.to_string())
    }}

    fn encode(snapshot: &{snap}) -> Result<Vec<u8>, String> {{
        {encode}(snapshot).map_err(|error| error.to_string())
    }}

    fn outcome_of(snapshot: &{snap}) -> Result<Outcome, String> {{
        let output = encode(snapshot)?;
        let projection = project_package(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }}

    /// 🦠️ The scenario's `{{kind, params}}` witness decoded generically: `params` IS the leaf's wire payload, so the
    /// derive-generated `from_payload_value` is the only decoder, and re-emitting the decoded payload must give back
    /// exactly `params`.
    fn mutation_from_spec(spec: &Json) -> Result<{agg}, String> {{
        let kind = spec.str("kind");
        let params = spec.get("params").cloned().unwrap_or(Json::Null);
        let payload: DslValue = from_json_str(&params.to_string()).map_err(|error| error.to_string())?;
        let mutation = <{agg} as Mutation<{snap}>>::from_payload_value(&kind, payload).map_err(|error| error.to_string())?;
        params_are_wire(&kind, &params, &to_json_string(&<{agg} as Mutation<{snap}>>::payload_value(&mutation)))?;
        Ok(mutation)
    }}

    /// ↩️ Applies `mutation` to `base` and then `{agg}::inverse` of it — the vocabulary's own algebra is the law
    /// under test, never a transcription of it.
    fn applied_and_undone(base: {snap}, mutation: &{agg}) -> {snap} {{
        let undo = <{agg} as Mutation<{snap}>>::inverse(mutation, &base);
        let mut snapshot = base;
        {apply}(&mut snapshot, mutation);
        for step in &undo {{
            {apply}(&mut snapshot, step);
        }}
        snapshot
    }}

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {{
        let spec = ctx.doc_json()?;
        let mut snapshot = decode(&arranged_input(ctx, &spec)?)?;
        {apply}(&mut snapshot, &mutation_from_spec(&spec)?);
        outcome_of(&snapshot)
    }}

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {{
        let spec = ctx.doc_json()?;
        let base = decode(&arranged_input(ctx, &spec)?)?;
        outcome_of(&applied_and_undone(base, &mutation_from_spec(&spec)?))
    }}

    /// 🏅️ The whole-package class stamp recorded as one `set-snapshot` of this repository's own stamp.
    pub fn stamp(ctx: &Context) -> Result<Outcome, String> {{
        let mut snapshot = decode(&mutable_input(ctx)?)?;
        let stamp = stamp_conformance_class_mutation(&snapshot, true);
        {apply}(&mut snapshot, &stamp);
        outcome_of(&snapshot)
    }}

    /// ↩️ The class stamp undone by `set-snapshot`'s own inverse.
    pub fn stamp_inverse(ctx: &Context) -> Result<Outcome, String> {{
        let base = decode(&mutable_input(ctx)?)?;
        let stamp = stamp_conformance_class_mutation(&base, true);
        outcome_of(&applied_and_undone(base, &stamp))
    }}

    /// 🔁️ Full semantic parse, re-serialized from the model alone — copying, splicing or patching
    /// source bytes is cheating, and this tripwire catches it.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {{
        let input = mutable_input(ctx)?;
        let output = encode(&decode(&input)?)?;
        if output == input {{
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }}
        let projection = project_package(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }}
}}
'''

REGISTRATION = '''//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. The class stamp is the `set-snapshot` kind's own plain scenario
/// pair, registered under its exact ids next to the outline bases.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    built = built.oracle("mutate", mutate_oracle).oracle("inverse", inverse_oracle).oracle("mutate-set-snapshot", stamp_oracle).oracle("inverse-set-snapshot", stamp_inverse_oracle).oracle("identity-round-trip", round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built
            .subject("mutate", subject::mutate)
            .subject("inverse", subject::inverse)
            .subject("mutate-set-snapshot", subject::stamp)
            .subject("inverse-set-snapshot", subject::stamp_inverse)
            .subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
'''

ORACLE_STAMP_REGION = '''
//#region 🔖️Stamp
/// 🏅️ Stamps the whole package into (`strict`) or out of the strict conformance class — the reference half of the
/// `mutate-set-snapshot`/`inverse-set-snapshot` scenarios, whose subject replaces its whole snapshot with its own stamp.
#[cfg(feature = "oracles")]
pub fn oracle_stamp(input: &[u8], strict: bool) -> Result<Vec<u8>, String> {
    let mut parts = crate::document::ooxml::read_parts(input)?;
    crate::document::ooxml::stamp_conformance_class(&mut parts, &PROFILE, strict)?;
    crate::document::ooxml::write_parts(&parts)
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_stamp(_input: &[u8], _strict: bool) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Stamp
'''

STAMP_PRODUCTION = '''
/// 🏅️ The one whole-package operation that moves `base` into (`strict`) or out of the strict conformance class: a
/// `set-snapshot` of [`stamp_conformance_class`]'s stamp — what a class conversion records as one edit.
pub fn stamp_conformance_class_mutation(base: &{snap}, strict: bool) -> {agg} {{
    {agg}::SetSnapshot(set_snapshot::SetSnapshot {{ snapshot: stamp_conformance_class(base.clone(), strict) }})
}}
'''

SCENARIOS = '''  @id-mutate-set-snapshot
  @level-exhaustive
  @mode-differential
  Scenario: Replace the real {noun} with its own strict-class stamp
    Given the real input package {uri}
    When the whole package is replaced by its own stamp into the strict conformance class
    Then the oracle and the subject agree on the conformance-class projection

  @id-inverse-set-snapshot
  @level-exhaustive
  @mode-property
  Scenario: Undoing the strict-class stamp restores the real {noun}
    Given the real input package {uri}
    When the whole package is replaced by its own strict-class stamp and that replacement is undone
    Then the oracle and the subject agree on the conformance-class projection of the original package

'''


def replace_once(text, old, new, label):
    assert text.count(old) == 1, f"{label}: expected one {old[:70]!r}, found {text.count(old)}"
    return text.replace(old, new)


def convert_adapter(path, cfg):
    artifact, _, module, crate, agg, snap, apply, encode, decode, _ = cfg
    text = path.read_text()
    oracle_import = f"subsets::{module}::{{oracle_apply_mutation, oracle_arrange, oracle_inverse_spec, oracle_round_trip, project_package}};"
    text = replace_once(text, oracle_import, f"subsets::{module}::{{oracle_apply_mutation, oracle_arrange, oracle_inverse_spec, oracle_round_trip, oracle_stamp, project_package}};", path.name)
    text = replace_once(text, "//#endregion 🔖️Oracle\n", ORACLE_STAMP.format().lstrip("\n"), path.name)
    start, end = text.index("mod subject {"), text.index("//#endregion 🔖️Subject")
    text = text[:start] + SUBJECT.format(crate=crate, module=module, agg=agg, snap=snap, apply=apply, encode=encode, decode=decode) + text[end:]
    start = text.index("//#region 🔖️Registration")
    end = text.index("//#endregion 🔖️Registration") + len("//#endregion 🔖️Registration\n")
    path.write_text(text[:start] + REGISTRATION + text[end:])


def convert_oracle(path, cfg):
    text = path.read_text()
    start = text.index("pub const KINDS: &[&str] = &[")
    end = text.index("];", start) + 2
    kinds = [kind.strip().strip('"') for kind in text[start + len("pub const KINDS: &[&str] = &["):end - 2].split(",")]
    kinds = [kind for kind in kinds if kind not in ("no-mutation", "set-snapshot")]
    text = text[:start] + "pub const KINDS: &[&str] = &[" + ", ".join(f'"{kind}"' for kind in kinds) + "];" + text[end:]
    text = replace_once(text, "\n//#region 🔖️Bridge\n", ORACLE_STAMP_REGION + "\n//#region 🔖️Bridge\n", path.name)
    path.write_text(text)


def convert_oracle_tests(path):
    text = path.read_text()
    text = text.replace('        "no-mutation" => json_object(vec![]),\n', "")
    text = text.replace('        "set-snapshot" => json_object(vec![("conformanceClass", Json::String("strict".to_string()))]),\n', "")
    text = text.replace('("contentType", Json::String("application/xml".to_string()))', '("content_type", Json::String("application/xml".to_string()))')
    for vml in ('"xl/drawings/vmlDrawing1.vml"', '"ppt/drawings/vmlDrawing1.vml"'):
        text = text.replace(f'        "insert-vml-part" => json_object(vec![("path", Json::String({vml}.to_string()))]),', f'        "insert-vml-part" => json_object(vec![("path", Json::String({vml}.to_string())), ("markup", Json::String(crate::document::ooxml::VML_MARKUP.to_string()))]),')
    text = text.replace('''        if *kind != "no-mutation" {
            assert_ne!(mutated_projection, base_projection, "{kind} must be observable in the conformance-class projection");
        }''', '''        assert_ne!(mutated_projection, base_projection, "{kind} must be observable in the conformance-class projection");''')
    start = text.index("#[test]\nfn no_mutation_is_a_true_byte_identity()")
    end = text.index("#[test]", start + 1)
    text = text[:start] + '''/// 🏅️ The whole-package class stamp moves the conformance-class projection, and stamping back restores it exactly.
#[test]
fn the_class_stamp_is_observable_and_stamping_back_restores_the_package() {
    let base = project_package(FIXTURE).unwrap();
    let stamped = oracle_stamp(FIXTURE, true).unwrap();
    assert_ne!(project_package(&stamped).unwrap(), base, "the strict stamp must be observable");
    assert_eq!(project_package(&oracle_stamp(&stamped, false).unwrap()).unwrap(), base, "stamping back must restore the package");
}

''' + text[end:]
    assert "no-mutation" not in text and "conformanceClass" not in text, path
    path.write_text(text)


def convert_production(path, cfg):
    _, _, _, _, agg, snap, *_ = cfg
    text = path.read_text()
    if "pub fn stamp_conformance_class_mutation" in text:
        return
    anchor = text.index("pub fn stamp_conformance_class(")
    head = text.index("\n}\n", anchor) + 3
    path.write_text(text[:head] + STAMP_PRODUCTION.format(agg=agg, snap=snap) + text[head:])


def convert_feature(path, cfg):
    noun = cfg[9]
    text = path.read_text()
    uri = next(line.split()[-1] for line in text.splitlines() if line.strip().startswith("Given the real input package"))
    out = []
    for line in text.split("\n"):
        stripped = line.strip()
        if stripped.startswith("| set-snapshot "):
            continue
        if stripped.startswith("| insert-vml-part "):
            cells = [cell.strip() for cell in stripped[1:-1].split("|")]
            path_value = cells[1].split('"path": "')[1].split('"')[0]
            vml = VML.replace('"', '\\"')
            line = f'      | {cells[0]} | {{"path": "{path_value}", "markup": "{vml}"}} |'
        if stripped.startswith("| set-worksheet-content-type "):
            line = line.replace('"contentType"', '"content_type"')
        out.append(line)
    text = "\n".join(out)
    blocks = [block for block in text.split("\n\n")]
    kept = [block for block in blocks if "@id-no-mutation-baseline" not in block]
    text = "\n\n".join(kept)
    anchor = "  @id-identity-round-trip\n"
    text = replace_once(text, anchor, SCENARIOS.format(noun=noun, uri=uri) + anchor, path.name)
    assert "no-mutation" not in text and "conformanceClass" not in text, path
    path.write_text(text)


for cfg in SUBSETS:
    artifact, subset = cfg[0], cfg[1]
    root = STDIO / artifact / "🏅️standards/🔖️ecma-376/🪆️subsets" / subset
    convert_adapter(next((root / "🧪️tests").glob("*/🦀️.rs")), cfg)
    convert_feature(next((root / "🧪️tests").glob("*/🥒️.feature")), cfg)
    convert_oracle(root / "🔮️oracles/🦀️.rs", cfg)
    convert_oracle_tests(root / "🔮️oracles/🧪️tests/🔬️unit/🦀️.rs")
    convert_production(root / "🧬️schema/🧬️mutations/🦀️.rs", cfg)
    print("converted", artifact, subset)
