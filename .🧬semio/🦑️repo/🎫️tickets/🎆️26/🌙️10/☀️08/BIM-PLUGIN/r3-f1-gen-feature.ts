#!/usr/bin/env bun
/**
 * 🥒️ Writes the language-agnostic mutation case `🧪️tests/🏙️mutate-model-1-any/{🥒️.feature,🦀️.rs}` from the leaf specs: one `mutate` and one
 * `inverse` scenario row per leaf, named after the first applied case of the leaf. Reads the leaf directories and their committed fixtures, so Wave M re-runs it after adding leaves.
 */
import { mkdirSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { em, fixtures as fixturesRoot, JSONF, mutations, RS, subset } from "./r3-f1-paths.ts";

const fixtures = join(fixturesRoot, em(0x1f9ec) + "mutations");
const dir = join(subset, em(0x1f9ea) + "tests", em(0x1f3d9) + "mutate-model-1-any");
mkdirSync(dir, { recursive: true });
const aggregate = readFileSync(join(mutations, RS), "utf8");
const kindNames = [...aggregate.slice(aggregate.indexOf("pub const KINDS")).matchAll(/"([a-z-]+)",/g)].map((m) => m[1]);
const rows = kindNames.map((kind) => {
  const leafDir = readdirSync(mutations).find((name) => name.endsWith(kind) && name.length > kind.length && !name.includes("."))!;
  const fixture = readdirSync(join(fixtures, leafDir)).sort().find((name) => JSON.parse(readFileSync(join(fixtures, leafDir, name, em(0x1f3af) + "outcome", JSONF), "utf8")).status === "applied")!;
  const params = JSON.stringify(JSON.parse(readFileSync(join(fixtures, leafDir, fixture, em(0x1f9a0) + "mutation", JSONF), "utf8")));
  return { id: kind, params, dir: leafDir, fixture };
});
const width = (key: "id" | "params" | "dir" | "fixture") => Math.max(...rows.map((r) => r[key].length), key.length);
const line = (r: { id: string; params: string; dir: string; fixture: string }) => `      | ${r.id.padEnd(width("id"))} | ${r.params.padEnd(width("params"))} | ${r.dir.padEnd(width("dir"))} | ${r.fixture.padEnd(width("fixture"))} |`;
const table = [line({ id: "id", params: "params", dir: "dir", fixture: "fixture" }), ...rows.map(line)].join("\n");
const before = `shared://${em(0x1f9ec)}mutations/<dir>/<fixture>/${em(0x1f4f8)}snapshot/${em(0x2b05)}before/${JSONF}`;
const mutation = `shared://${em(0x1f9ec)}mutations/<dir>/<fixture>/${em(0x1f9a0)}mutation/${JSONF}`;
const after = `shared://${em(0x1f9ec)}mutations/<dir>/<fixture>/${em(0x1f4f8)}snapshot/${em(0x27a1)}after/${JSONF}`;
const doc = `      """
      {"kind": "<id>", "params": <params>, "before": "${before}"}
      """`;
const feature = `@capability-bim-1-mutate
@oracle-bim-1-jsonpatch-diff
@comparison-ordered-json-v1
@mutations-bim-1-any
Feature: Apply every typed BIM mutation to its own committed model and undo it with its own concrete inverse
  \`s.bim.model@1\` stores authored parameters only; every mutation is a sparse typed diff with a concrete inverse, applied only by the
  central applier. This case walks every kind of \`ModelMutation\` in declaration order: sites and buildings, the storey
  family (create, rename, height, level, cascading delete), and the first wall kinds. Each row names the committed leaf fixture it
  replays: the before snapshot, the mutation payload and the expected after snapshot live in the leaf's own \`🧫️fixtures\`, written by the
  same code the unit tests exercise and reviewed by hand. Rejected cases (missing reference, duplicate id, still-referenced target, broken
  invariant) are asserted by the leaf unit tests; this case proves the differential law over the language boundary: the projection moves
  on apply and returns to the base on undo. The projection is the canonical JSON of the whole snapshot. Shape: externally tagged enums,
  snake_case fields, ids as map keys.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed model and observe it move
    Given the committed before-snapshot ${before}
    And the committed mutation payload ${mutation}
    And the committed after-document ${after}
    When the <id> mutation is applied through apply_model_mutation
${doc}
    Then the resulting projection differs from the base projection
    Examples:
${table}

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores its committed model exactly
    Given the committed before-snapshot ${before}
    And the committed mutation payload ${mutation}
    And the committed after-document ${after}
    When the <id> mutation is applied through apply_model_mutation
${doc}
    And every step of its own computed inverse is applied through apply_model_mutation
    Then the projection equals the base projection again
    Examples:
${table}

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real committed demo DSL artifact
    Given the plugin's own committed DSL artifact asset://${em(0x1f3ac)}demo/${em(0x1f5e3)}.dsl.semio
    And the committed JSON document of the same demo asset://${em(0x1f3ac)}demo/${em(0x1f4f8)}snapshot.json
    When it is parsed with parse_dsl and printed back with print_dsl
    Then the printed bytes are identical to the committed bytes and reparsing preserves the projection
      """
      {"kind": "identity-round-trip", "params": {"carrier": "byte-exact"}}
      """
`;
writeFileSync(join(dir, em(0x1f952) + ".feature"), feature);

const kinds = rows.map((row) => `    "${row.id}",`).join("\n");
const adapter = `//! ${em(0x1f980)} BIM model exhaustive mutation case, Rust adapter. The subject role asserts its law through the shared
//! \`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/⚖️law/🦀️.rs\` module before it returns. The subject half is \`sut\`-gated because the generated host
//! links this repository's crate only for the subject role. No oracle handler is registered: a third-party reproduction of the
//! mutation semantics is registered by the oracle wave, never by an adapter that re-reads what the subject just produced.

use semio_repo_test_host::Adapter;

//#region 🔖️Vocabulary
/// 🏷️ Mirrors \`KINDS\` of \`ModelMutation\` (\`../../🧬️schema/🧬️mutations/🦀️.rs\`) so the oracle-only build never links the subject crate.
const KINDS: &[&str] = &[
${kinds}
];

/// 📄️ The plugin's own committed real DSL artifact: the only input that can carry evidence about the text grammar.
const DSL_ASSET: &str = "asset://${em(0x1f3ac)}demo/${em(0x1f5e3)}.dsl.semio";
//#endregion 🔖️Vocabulary

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::DSL_ASSET;
    use semio_repo_test_host::law::{carrier_is_exact, inverse_restores, mutation_is_observable, round_trip_preserves};
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_artifact_bim_model::mutations::{apply_model_mutation, inverse_model_mutation, ModelMutation};
    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::text::mutations::decode_model_mutation_json;
    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::text::snapshot::{decode_model_snapshot_json, encode_model_projection_json, parse_dsl, print_dsl};
    use semio_s_artifact_bim_model::ModelSnapshot;

    fn base(ctx: &Context, spec: &Json) -> Result<ModelSnapshot, String> {
        let bytes = ctx.input_bytes(&spec.str("before"))?;
        decode_model_snapshot_json(&String::from_utf8(bytes).map_err(|error| format!("the committed before-snapshot is not UTF-8: {error}"))?)
    }

    fn mutation(spec: &Json) -> Result<ModelMutation, String> {
        let params = spec.get("params").ok_or_else(|| "the scenario doc string carries no params member".to_string())?;
        decode_model_mutation_json(&params.to_string())
    }

    fn projection(snapshot: &ModelSnapshot) -> Result<Json, String> {
        parse_json(&encode_model_projection_json(snapshot))
    }

    //#region 🔖️Handlers
    /// 👁️ Every declared kind must MOVE the surface the scenario is compared through; the exemption list is empty.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let kind = spec.str("kind");
        let base = base(ctx, &spec)?;
        let mutated = apply_model_mutation(&base, &mutation(&spec)?).map_err(|error| format!("mutate-{kind}: the mutation did not apply: {error}"))?;
        let after = projection(&mutated)?;
        mutation_is_observable(&kind, &after, &projection(&base)?, &[])?;
        Ok(Outcome::with_raw(after.to_string().into_bytes(), after))
    }

    /// ↩️ Applying the kind and then its OWN computed inverse (replayed storage-reversed, as the store does) lands back on the base.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let kind = spec.str("kind");
        let base = base(ctx, &spec)?;
        let mutation = mutation(&spec)?;
        let mut current = apply_model_mutation(&base, &mutation).map_err(|error| format!("inverse-{kind}: the forward mutation did not apply: {error}"))?;
        for step in inverse_model_mutation(&base, &mutation).map_err(|error| format!("inverse-{kind}: {error}"))?.iter().rev() {
            current = apply_model_mutation(&current, step).map_err(|error| format!("inverse-{kind}: an inverse step did not apply: {error}"))?;
        }
        let restored = projection(&current)?;
        inverse_restores(&kind, &restored, &projection(&base)?)?;
        Ok(Outcome::with_raw(restored.to_string().into_bytes(), restored))
    }

    /// 🔁️ The identity law on the real committed DSL bytes; the carrier is this codec's own output, so it must be reproduced exactly.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = ctx.input_bytes(DSL_ASSET)?;
        let committed = String::from_utf8(input.clone()).map_err(|error| format!("the committed BIM artifact is not UTF-8: {error}"))?;
        let decoded = parse_dsl(&committed).map_err(|error| format!("identity-round-trip: the committed BIM artifact does not parse: {error:?}"))?;
        let printed = print_dsl(&decoded);
        carrier_is_exact(printed.as_bytes(), &input)?;
        let reparsed = parse_dsl(&printed).map_err(|error| format!("identity-round-trip: this codec's own output does not parse back: {error:?}"))?;
        let after = projection(&reparsed)?;
        round_trip_preserves(&after, &projection(&decoded)?)?;
        Ok(Outcome::with_raw(printed.into_bytes(), after))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario id, mirroring the \`Examples\` tables.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        let _ = kind;
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate).subject(&format!("inverse-{kind}"), subject::inverse);
        }
    }
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
`;
writeFileSync(join(dir, RS), adapter);
console.log(`feature + adapter written for ${rows.length} kinds`);
