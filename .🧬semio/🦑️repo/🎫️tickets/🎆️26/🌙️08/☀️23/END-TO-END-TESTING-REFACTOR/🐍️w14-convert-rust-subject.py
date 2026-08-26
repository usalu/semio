#!/usr/bin/env python3
"""🔧️ Scratch surgery: converts a wave-8 `no-oracle` semio case adapter into a SUBJECT-only adapter
for the wave-14 cross-language conversion. Keeps the file's own JSON readers, fixture decoding and
projection verbatim; replaces the header, drops the oracle region, and rewrites the handlers and the
registration so the reference answer comes from the case's `🐍️component.py` instead."""

import io, re, sys

ART = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests"

HEADER = '''//! 🦀️ Semio {UPPER} exhaustive mutation case — Rust SUBJECT adapter. Ticket 26/08/23/END-TO-END-
//! TESTING-REFACTOR.
//!
//! This file registers the SUBJECT role only. The reference answer comes from `🐍️component.py`
//! beside it — an independent Python implementation of the same carrier and the same
//! {COUNTWORD} verbs, written from the committed grammars, registered as the oracle
//! `semio-{SUBSET}-python-independent` in
//! `../../🏅️standards/🔖️v1/🪆️subsets/✳️{SUBSET}/🧪️oracle/🔣️component.json`. Registering oracle
//! handlers here as well would put this repository's own answer on both sides of the comparison,
//! which is the one failure the platform exists to prevent, so the registrations this file used to
//! carry are gone rather than merely unused.
//!
//! Every scenario drives this repository's own production entry points — `{PARSE}`/`{PRINT}` for
//! the carrier and `{APPLY}`/`{INVERSE}` for the vocabulary — over the real committed {NOUN}
//! artifact `{ASSETDOC}`, and projects the resulting snapshot as structural JSON for
//! `ordered-json-v1` to compare against the Python side's.
//!
//! The mutation parameters and the specification-vector paths live in `component.feature`, so both
//! implementations read one physical copy of each and cannot drift apart. The laws are asserted
//! here IN ROLE as well as compared: `inverse-` requires the {NOUN} back, `spec-vector-` requires
//! the committed after-snapshot, and `identity-round-trip` requires byte-exact re-emission through
//! `law::carrier_is_exact`.
//!
//! The subject half is gated behind the generated host's `sut` feature so a non-subject build never
//! compiles the local implementation.

use semio_repo_test_host::Adapter;

//#region 🔖️Kinds
/// 🏷️ Mirrors `{MUTENUM}::KINDS` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️{SUBSET}/🧬️schema/
/// 🧬️mutations/🦀️component.rs`) — duplicated, not imported, because the registration loop runs in
/// builds where the subject crate is not linked. The contract's mutation-coverage gate keeps this
/// list honest against the catalog; `kinds_match_the_enum_and_the_catalog` in that production file
/// keeps it honest against the enum.
#[cfg_attr(not(feature = "sut"), allow(dead_code))]
const KINDS: &[&str] = &[{KINDS}];
//#endregion 🔖️Kinds

'''

HANDLERS = '''    //#region 🔖️Inputs
    const {CONST}: &str = "{ASSET}";

    /// {EMOJI} The real committed {NOUN} artifact, parsed by this repository's own DSL codec.
    fn artifact(ctx: &Context) -> Result<{SNAP}, String> {{
        let bytes = ctx.fixture_bytes({CONST})?;
        let source = String::from_utf8(bytes).map_err(|error| format!("the committed {NOUN} artifact must be UTF-8: {{error}}"))?;
        {PARSE}(&source)
    }}

    /// 🦠️ The verb the scenario declares, read from the feature's own doc string.
    fn declared(ctx: &Context) -> Result<{MUTENUM}, String> {{
        mutation_of(&ctx.doc_json()?)
    }}

    fn run(current: &mut {SNAP}, mutation: &{MUTENUM}, what: &str) -> Result<(), String> {{
        let applied = {APPLY}(current, mutation);
        let refusals = semio_mutation_refusals(&applied);
        if refusals.is_empty() {{
            return Ok(());
        }}
        Err(format!("{{what}}: mutation rejected: {{refusals:?}}"))
    }}
    //#endregion 🔖️Inputs

    //#region 🔖️Handlers
    /// 🎯️ One verb applied to the real committed {NOUN} through the production entry point.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {{
        let mut current = artifact(ctx)?;
        run(&mut current, &declared(ctx)?, ctx.scenario.id.as_str())?;
        Ok(outcome(snapshot_json(&current)))
    }}

    /// ↩️ The metamorphic inverse law on the real committed {NOUN}: applying the verb and then its
    /// OWN computed inverse must restore the artifact exactly. The MUTATED snapshot travels in the
    /// projection alongside the restored one, so the rows cannot all project the same restored
    /// value and compare vacuously.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {{
        let base = artifact(ctx)?;
        let mutation = declared(ctx)?;
        let mut current = base.clone();
        run(&mut current, &mutation, ctx.scenario.id.as_str())?;
        let mutated = snapshot_json(&current);
        for step in &{INVERSE}(&mutation, &base) {{
            run(&mut current, step, ctx.scenario.id.as_str())?;
        }}
        if current != base {{
            return Err(disagreement(&format!("{{}}: undoing the mutation did not restore the real committed {NOUN}", ctx.scenario.id), &current, &base));
        }}
        Ok(outcome(Json::Object(vec![("mutated".to_string(), mutated), ("restored".to_string(), snapshot_json(&current))])))
    }}

    /// 🧫️ The same verb on its committed `(before, mutation, after)` vector — a THIRD statement of
    /// what the verb means, independent of both implementations, kept from before this oracle
    /// existed rather than replaced by it.
    pub fn spec_vector(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {{
        move |ctx: &Context| {{
            let (mut current, mutation, expected) = vector_of(ctx, kind)?;
            run(&mut current, &mutation, ctx.scenario.id.as_str())?;
            if current != expected {{
                return Err(disagreement(&format!("{{}}: the applied snapshot does not match the committed after-snapshot", ctx.scenario.id), &current, &expected));
            }}
            Ok(outcome(snapshot_json(&current)))
        }}
    }}

    /// 🔁️ The real committed artifact, parsed into the typed snapshot, printed back to DSL text and
    /// parsed again — the only channel from input to output is the model, so nothing of the source
    /// bytes can be smuggled into the projection.
    /// 🔒️ **The byte half of the identity law — asserted as `carrier_is_exact`.** `.dsl.semio` is a
    /// fixed-layout record grammar and the committed artifact was produced by this very printer, so
    /// reproducing it BYTE FOR BYTE is the correct answer here and `law::reparsed_not_copied` would
    /// be exactly backwards. Nor is it a self-comparison: the Python oracle re-emits the same file
    /// from the grammar alone and the two digests are compared. This subset's committed
    /// `.pack.semio` twin is NOT read here — no pack bridge is exported for it — so the byte claim
    /// is about the text carrier alone.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {{
        let committed = ctx.fixture_bytes({CONST})?;
        let source = String::from_utf8(committed.clone()).map_err(|error| format!("the committed {NOUN} artifact must be UTF-8: {{error}}"))?;
        let once = {PARSE}(&source)?;
        let printed = {PRINT}(&once);
        carrier_is_exact(printed.as_bytes(), &committed)?;
        let twice = {PARSE}(&printed)?;
        if twice != once {{
            return Err(disagreement("identity-round-trip: re-parsing the printed DSL did not reproduce the parsed snapshot", &twice, &once));
        }}
        let (declared, _mutation, _after) = vector_of(ctx, "no-mutation")?;
        if once != declared {{
            return Err(disagreement("identity-round-trip: the real committed {NOUN} artifact does not decode to the before-snapshot every specification vector starts from", &once, &declared));
        }}
        Ok(outcome(Json::Object(vec![
            ("document".to_string(), snapshot_json(&twice)),
            ("dslDigest".to_string(), Json::String(digest(printed.as_bytes()))),
            ("dslLength".to_string(), Json::Number(printed.len() as f64)),
        ])))
    }}
    //#endregion 🔖️Handlers
}}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario
/// id, so the whole vocabulary is registered in one loop. Subject only — the oracle role belongs to
/// `🐍️component.py`.
pub fn adapter() -> Adapter {{
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {{
        for kind in KINDS {{
            built = built
                .subject(&format!("mutate-{{kind}}"), subject::mutate)
                .subject(&format!("inverse-{{kind}}"), subject::inverse)
                .subject(&format!("spec-vector-{{kind}}"), subject::spec_vector(kind));
        }}
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }}
    built
}}
//#endregion 🔖️Registration
'''

COUNTWORD = {9: "nine", 10: "ten", 13: "thirteen", 16: "sixteen", 18: "eighteen", 20: "twenty"}


def convert(case, subset, upper, noun, emoji, snap, mutenum, parse, printer, apply, inverse, asset):
    path = "%s/%s/🦀️component.rs" % (ART, case)
    src = io.open(path, encoding="utf-8").read()

    kinds_line = re.search(r"const KINDS: &\[&str\] = &\[(.*?)\];", src, re.S).group(1)
    kinds = [k.strip().strip('"') for k in kinds_line.split(",") if k.strip() != ""]

    head = HEADER.format(
        UPPER=upper, SUBSET=subset, COUNTWORD=COUNTWORD[len(kinds)], PARSE=parse, PRINT=printer,
        APPLY=apply, INVERSE=inverse, NOUN=noun, MUTENUM=mutenum,
        ASSETDOC=asset.replace("asset://", "../../"), KINDS=", ".join('"%s"' % k for k in kinds),
    )

    body = src[src.index("//#region 🔖️Subject") :]
    body = body[: body.index("    //#region 🔖️Handlers")]
    body = body.replace(
        "use semio_repo_test_host::{Context, Json, Outcome};",
        "use semio_repo_test_host::{digest, Context, Json, Outcome};",
    )
    tail = HANDLERS.format(
        CONST="%s_DSL" % noun.upper(), ASSET=asset, NOUN=noun, EMOJI=emoji, SNAP=snap,
        MUTENUM=mutenum, PARSE=parse, PRINT=printer, APPLY=apply, INVERSE=inverse,
    )
    io.open(path, "w", encoding="utf-8").write(head + body + tail)
    print("%s: %d kinds" % (case, len(kinds)))


if __name__ == "__main__":
    convert(
        "mutate-semio-animation", "animation", "ANIMATION", "walk", "🚶️",
        "SemioAnimationSnapshot", "SemioAnimationMutation", "parse_semio_animation_dsl", "print_semio_animation_dsl",
        "apply_semio_animation_mutation", "inverse_semio_animation_mutation",
        "asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🚶️walk/🖼️assets/🗣️example.dsl.semio",
    )
