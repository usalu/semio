# -*- coding: utf-8 -*-
"""🏗️ Generates the 📕️norm mutation-catalog closure: per-subset manifests, per-artifact cases and
the KINDS/bridge additions to the production mutation and snapshot facets. Every fact it writes is
read off the committed tree — kinds from each leaf's own SemanticDescriptor, fixture paths from the
committed directories, prose from meta.py."""
import json, os, re, sys

ROOT = "/Users/ueli/Documents/semio"
TICKET = os.path.join(ROOT, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/w12-norm")
sys.path.insert(0, TICKET)
from meta import META

ART = os.path.join(ROOT, "✏️s/🔌️plugins/📕️norm/🗿️artifacts")
SUB = "🏅️standards/🔖️1/🪆️subsets/✳️any"
SCHEMA = os.path.join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️component.json")
survey = json.load(open(os.path.join(TICKET, "survey.json"), encoding="utf-8"))


def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(text)


def wrap(text, width=98, indent="  "):
    out = []
    for para in text.split("\n"):
        stripped = para.strip()
        lead = indent + ("  " if para.startswith("  ") else "")
        words, line = stripped.split(" "), ""
        for word in words:
            if line and len(lead) + len(line) + 1 + len(word) > width:
                out.append(lead + line)
                line = word
            else:
                line = f"{line} {word}".strip()
        out.append(lead + line)
    return "\n".join(out)


def doc(text, width=96, prefix="//! "):
    out = []
    for para in text.split("\n"):
        stripped = para.strip()
        if stripped == "":
            out.append(prefix.rstrip())
            continue
        line = ""
        for word in stripped.split(" "):
            if line and len(prefix) + len(line) + 1 + len(word) > width:
                out.append(prefix + line)
                line = word
            else:
                line = f"{line} {word}".strip()
        out.append(prefix + line)
    return "\n".join(out)


for art, info in survey.items():
    m = META[art]
    slug, ty, mod = m["slug"], m["ty"], m["mod"]
    kinds = [k["kind"] for k in info["kinds"]]
    art_dir = os.path.join(ART, art)
    sub_dir = os.path.join(art_dir, SUB)
    case = f"mutate-{slug}-1"
    case_dir = os.path.join(art_dir, "🧪️tests", case)
    capability, catalog, decision = f"{slug}-1-mutate", f"{slug}-1-any", f"{slug}-1-mutation-semantics"
    rejected = [k["kind"] for k in info["kinds"] if json.load(open(os.path.join(sub_dir, "🧬️schema/🧬️mutations", k["leaf"]["dir"], "🧪️tests", k["leaf"]["cases"][0], "🎯️outcome/🔣️component.json"), encoding="utf-8")).get("status") != "applied"]

    # ── 1. the subset's own oracle manifest ──────────────────────────────────────────────────────
    oracle_dir = os.path.join(sub_dir, "🧪️oracle")
    rel_schema = os.path.relpath(SCHEMA, oracle_dir)
    rationale = (
        f"`s.norm.{slug}` is a semio-NATIVE artifact: the document is this repository's own "
        f"`.dsl.semio`/`.pack.semio` envelope over a {m['standard_line']} model, and no third-party "
        f"library — in any ecosystem — reads or writes it, let alone one that could be authoritative "
        f"over this subset's `{ty}Mutation` vocabulary. That vocabulary IS this subset's own "
        f"specification (derived from `{ty}Snapshot`'s shape by "
        f"`26/08/12/SEMANTIC-MUTATIONS-OVERHAUL`'s `📓️derivation-rules.md`), not a fact an external "
        f"implementation could confirm or refute; a crate that parsed {m['title']} data from some "
        f"other interchange format would be judging a different document. Registering a weak or "
        f"tangential crate here would be worse than registering none. Confidence comes instead from "
        f"the {len(kinds)} committed, independently handcrafted per-kind specification vectors under "
        f"`../🧬️schema/🧬️mutations/<kind>/🧪️tests/<fixture>/` — a `(before, mutation, after, diff, "
        f"outcome)` quintet per kind, already unit-tested inside the production crate itself — "
        f"re-exercised end to end here through `apply_{slug}_mutation`, plus the inverse law as a "
        f"metamorphic property: applying a mutation and then its own computed inverse must restore "
        f"the exact pre-mutation document."
    )
    write(os.path.join(oracle_dir, "🔣️component.json"), json.dumps({
        "$schema": rel_schema,
        "schemaVersion": 1,
        "_comment": (
            f"🧩️ This subset's own contribution. A mutation vocabulary belongs to ONE subset of ONE "
            f"standard of ONE artifact, so the (non-)oracle registration and the catalog live here "
            f"rather than in a shared manifest. `{catalog}` is generated from each triad leaf's own "
            f"`SemanticDescriptor.kind`, in `#[derive(dsl::Mutations)]`'s declaration order, and "
            f"`kinds_match_the_enum_and_the_catalog` in "
            f"`../🧬️schema/🧬️mutations/🦀️component.rs` fails the moment this list and the enum part "
            f"company."),
        "oracles": [],
        "noOracleDecisions": [{
            "id": decision,
            "capabilities": [capability],
            "rationale": rationale,
            "substitutes": ["specification-vectors", "metamorphic-laws"],
        }],
        "mutationCatalogs": [{"id": catalog, "capability": capability, "kinds": kinds}],
    }, ensure_ascii=False, indent=2) + "\n")

    # ── 2. the feature ───────────────────────────────────────────────────────────────────────────
    asset = f"asset://{SUB}/📚️examples/{m['example']}/🖼️assets/{m['dsl']}"
    pack_asset = f"asset://{SUB}/📚️examples/{m['example']}/🖼️assets/{m['pack']}" if m["pack"] else None
    rows = "\n".join(f"      | {k} |" for k in kinds)
    parts = [
        f"`s.norm.{slug}` is a semio-NATIVE artifact — no third party reads or writes its "
        f"`.dsl.semio`/`.pack.semio` envelope — so there is no reference implementation to register "
        f"as an oracle. That is recorded as the `{decision}` no-oracle decision in "
        f"`../../{SUB}/🧪️oracle/🔣️component.json`, and it means the runner executes NO oracle role "
        f"for this case: every assertion below lives inside the subject handler, which compares the "
        f"applied document against the committed after-snapshot and the undone document against the "
        f"committed before-snapshot, and fails with both documents printed. A handler that merely ran "
        f"the mutation and returned would report a pass having checked nothing.",
        m["shape"],
        m["distinguishing"],
    ]
    if m["deferred"]:
        parts.append(m["deferred"])
    parts.append(
        f"Each of the {len(kinds)} kinds carries its own independently handcrafted `(before, "
        f"mutation, after, diff, outcome)` quintet under "
        f"`../../{SUB}/🧬️schema/🧬️mutations/<kind>/🧪️tests/<fixture>/`, and this feature re-exercises "
        f"those SAME committed bytes end to end through `apply_{slug}_mutation` rather than calling "
        f"`Mutation::diff`/`inverse` directly the way the in-crate fixture tests do. The committed "
        f"`🎯️outcome` decides which contract a row is held to: `applied` demands the observability "
        f"law (the document must MOVE), `rejected` demands the opposite and stricter one — the "
        f"mutation must be refused and the document must come back bit-identical."
        + ("" if rejected else
           f" All {len(kinds)} committed vectors declare `applied`, so every row below is held to the "
           f"observability law: a kind that left the document bit-for-bit unchanged would fail rather "
           f"than pass silently.")
    )
    parts.append(
        f"The identity scenario reads the real committed {m['title']} document at "
        f"`📚️examples/{m['example']}`, not a fixture authored for this case. Its DSL carrier is "
        f"deliberately byte-preserving — the committed file IS this codec's own canonical printer "
        f"output, so reproducing it exactly is the correct answer and anything else is the defect — "
        f"which is why that half of the identity law is asserted as `carrier_is_exact` rather than as "
        f"the usual no-byte-pass-through inequality. The evidence that the document was genuinely "
        f"PARSED rather than copied comes from the other half: the same snapshot is round-tripped "
        f"through two further, independently written codecs — the binary `.pack.semio` protocol and "
        f"the JSON projection — and all three must agree on one document."
        + (f" The committed binary twin `{m['pack']}` is decoded and cross-checked against the text "
           f"artifact as well, so two separately committed files written by two separate codecs have "
           f"to describe the same {m['title']} document." if m["pack"] else
           f" This artifact commits only the DSL encoding of its example, so the binary leg is "
           f"encode-then-decode rather than a committed twin; that is stated here rather than "
           f"papered over.")
    )
    desc = "\n\n".join(wrap(p) for p in parts)
    feature = f"""@capability-{capability}
@no-oracle-{decision}
@comparison-ordered-json-v1
@mutations-{catalog}
Feature: Apply every typed {m['title']} mutation to its committed specification fixtures
{desc}

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to its committed before-snapshot fixture
    Given the committed before-snapshot, mutation and outcome fixture for the <id> kind
    When <id> is applied through apply_{slug}_mutation
    Then the resulting document matches the committed after-snapshot fixture for <id> and honours the committed outcome status
    Examples:
      | id |
{rows}

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot fixture
    Given the committed before-snapshot and mutation fixture for the <id> kind
    When <id> is applied through apply_{slug}_mutation
    And the mutation's own computed inverse is applied through apply_{slug}_mutation
    Then the document matches the committed before-snapshot fixture again
    Examples:
      | id |
{rows}

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode the real committed {m['title']} document through every encoding it has
    Given the real committed text artifact {asset}
"""
    if pack_asset:
        feature += f"    And its committed binary twin {pack_asset}\n"
    feature += (f"    When the text artifact is parsed, printed back to DSL and parsed again, and the same document is round-tripped through the binary pack protocol and the JSON projection\n"
                f"    Then the canonical DSL rendering is reproduced byte for byte and every decoding agrees on one {m['title']} document\n")
    write(os.path.join(case_dir, "component.feature"), feature)

    # ── 3. the adapter ───────────────────────────────────────────────────────────────────────────
    def leafpath(k, tail):
        leaf = next(x for x in info["kinds"] if x["kind"] == k)["leaf"]
        return f"../../{SUB}/🧬️schema/🧬️mutations/{leaf['dir']}/🧪️tests/{leaf['cases'][0]}/{tail}"

    arms = []
    for k in kinds:
        arms.append(
            f'        "{k}" => (\n'
            f'            include_str!("{leafpath(k, "📸️snapshot/⬅️before/🔣️component.json")}"),\n'
            f'            include_str!("{leafpath(k, "🦠️mutation/🔣️component.json")}"),\n'
            f'            include_str!("{leafpath(k, "📸️snapshot/➡️after/🔣️component.json")}"),\n'
            f'            include_str!("{leafpath(k, "🎯️outcome/🔣️component.json")}"),\n'
            f'        ),'
        )
    kind_list = "\n".join(f'    "{k}",' for k in kinds)
    header = doc(
        f"🦀️ {m['title']} exhaustive mutation case — Rust adapter. Ticket "
        f"26/08/23/END-TO-END-TESTING-REFACTOR, wave 12 (the unregistered-vocabulary sweep). "
        f"Recorded no-oracle decision `{decision}` "
        f"(`../../{SUB}/🧪️oracle/🔣️component.json`): `s.norm.{slug}` is a semio-native artifact with "
        f"no third-party reader or writer, so the `oracle` handlers here read the committed, "
        f"independently handcrafted per-kind specification vectors literally — no recomputation, no "
        f"reimplementation of mutation semantics — while `subject` drives this repository's own "
        f"`apply_{slug}_mutation` over the full {len(kinds)}-kind `{ty}Mutation` vocabulary.\n\n"
        + m["shape"] + "\n\n" +
        "⚖️ WHERE THE ASSERTIONS LIVE. A recorded no-oracle case runs NO oracle role — the runner "
        "resolves an oracle implementation from the feature's `@oracle-` tag and this feature has "
        "none — so the comparison profile never gets two sides to compare. Every law this case "
        "claims is therefore asserted IN ROLE inside the subject handlers, through the shared "
        "`✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law` module (`law::mutation_is_observable`, "
        "`law::inverse_restores`, `law::round_trip_preserves`, `law::carrier_is_exact`) that the "
        "stdio mutation cases use, reached through the `oracleHostPackages` entry this plugin "
        "declares in `✏️s/🔌️plugins/📕️norm/🧪️oracle/🔣️component.json`. The oracle handlers still "
        "assert what a committed vector can prove on its own: that an `applied` vector genuinely "
        "moves the document and a `rejected` one genuinely does not.\n\n"
        "🌉️ HOW THE FIXTURES REACH TYPED VALUES. The generated test host links only "
        "`semio-repo-test-host`, the stdio law crate and — behind `sut` — this plugin's own crate; "
        "`serde`, `serde_json` and this crate's `protocol`/`store`/`vcs` extern-crate aliases are "
        f"all unreachable from here. The subset's own production code therefore exports the bridges "
        f"(`decode_{slug}_snapshot_json`/`encode_{slug}_snapshot_json`, `decode_{slug}_dsl`/"
        f"`encode_{slug}_dsl`, `decode_{slug}_pack`/`encode_{slug}_pack` in "
        f"`../../{SUB}/🧬️schema/📸️snapshot/🦀️component.rs`; `decode_{slug}_mutation_json`, "
        f"`apply_{slug}_mutation`, `inverse_{slug}_mutation` in "
        f"`../../{SUB}/🧬️schema/🧬️mutations/🦀️component.rs`), whose signatures name only reachable "
        f"types. Both roles read the SAME committed bytes — the oracle role via `include_str!`, the "
        f"subject role by decoding that same text — so a fixture can never drift away from a Rust "
        f"literal transcribed beside it, because there is none.\n\n"
        "🚧️ The Rust SUBJECT phase cannot run at the time of writing: `semio-s-plugin-norm` does not "
        "compile (a concurrent session is mid-flight removing gratuitous `async fn` wrappers across "
        "the crate), and `semio-framework-os-kernel` is red for the same reason. The subject half is "
        "written against the SYNC trait surface the fixture tests in this crate already call "
        "(`Mutation::diff`, `MutationDiff::apply`, `Mutation::inverse`, `ArtifactDsl`, "
        "`ArtifactPack`) rather than against the plugin's async wrappers, and is `sut`-gated so the "
        "oracle-only run never links it."
    )
    pack_const = f'\n/// 🎒️ The same document in its binary envelope, written by a separate codec from the DSL text.\n#[cfg(feature = "sut")]\nconst PACK_ASSET: &str = "{pack_asset}";' if pack_asset else ""
    twin_leg = (
        "\n        let twin = decode_{slug}_pack(&ctx.fixture_bytes(super::PACK_ASSET)?)?;\n"
        "        if twin != parsed {{\n"
        "            return Err(disagreement(\"identity-round-trip: the committed binary twin decodes to a different document than the committed text artifact\", &twin, &parsed));\n"
        "        }}"
    ).format(slug=slug) if pack_asset else ""

    adapter = f'''{header}

use semio_repo_test_host::{{parse_json, Adapter, Context, Json, Outcome}};
use semio_s_plugin_stdio_test_oracle::law;

//#region 🔖️Kinds
/// 🏷️ Mirrors `{ty}Mutation::KINDS` (`../../{SUB}/🧬️schema/🧬️mutations/🦀️component.rs`) —
/// duplicated, not imported, because the oracle-only build must not link the subject crate. The
/// contract's mutation-coverage gate keeps this list honest against the catalog;
/// `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against the enum.
const KINDS: &[&str] = &[
{kind_list}
];

/// 🗣️ The real committed {m['title']} document, read where the domain already keeps it.
const DSL_ASSET: &str = "{asset}";{pack_const}
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ The committed `(before, mutation, after, outcome)` specification vector for one kind, read
/// literally via `include_str!` — this IS the independently handcrafted evidence the no-oracle
/// decision rests on, never recomputed. One `include_str!` per committed file: the oracle role
/// answers with `before`/`after`, the subject role decodes all four.
fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str, &'static str) {{
    match kind {{
{chr(10).join(arms)}
        other => panic!("{case}: no committed fixture is registered for kind {{other:?}}"),
    }}
}}

/// 🔎️ Parses one embedded fixture file into the framework's own dependency-free `Json`.
fn canonical(text: &str) -> Json {{
    parse_json(text).unwrap_or_else(|error| panic!("committed fixture JSON must parse: {{error}}"))
}}

/// 🎯️ The status the committed `🎯️outcome/🔣️component.json` declares for one kind — `applied` or
/// `rejected` — read out of the committed file rather than transcribed beside it, so the contract a
/// row is held to cannot drift away from the vector that states it.
fn committed_status(kind: &str) -> String {{
    let (_before, _mutation, _after, outcome) = fixture_text(kind);
    canonical(outcome).str("status")
}}
//#endregion 🔖️Fixtures

//#region 🔖️Oracle
/// 🔮️ The forward reference answer: the committed AFTER snapshot, read literally. The one law a
/// committed pair can carry on its own is asserted here in role — an `applied` vector must MOVE the
/// document and a `rejected` vector must leave it identical — so a placeholder fixture that changed
/// nothing could not sit in this table unnoticed.
fn mutate_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {{
    move |_ctx: &Context| {{
        let (before, _mutation, after, _outcome) = fixture_text(kind);
        let (base, projection) = (canonical(before), canonical(after));
        match committed_status(kind).as_str() {{
            "applied" => law::mutation_is_observable(kind, &projection, &base, &[])?,
            "rejected" if law::divergence(&projection, &base).is_some() => {{
                return Err(format!("mutate-{{kind}}: the committed outcome declares this vector rejected, so its after-snapshot must be identical to its before-snapshot"));
            }}
            "rejected" => {{}}
            other => return Err(format!("mutate-{{kind}}: unknown committed outcome status {{other:?}}")),
        }}
        Ok(Outcome::with_raw(after.as_bytes().to_vec(), projection))
    }}
}}

/// 🔮️ The inverse reference answer: the committed BEFORE snapshot — undoing any mutation must
/// return to exactly where the specification vector started. The inverse LAW itself cannot be
/// asserted from the committed vectors alone (nothing here computes an inverse), which is precisely
/// why it is asserted in the subject handler below instead.
fn inverse_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {{
    move |_ctx: &Context| {{
        let (before, _mutation, _after, _outcome) = fixture_text(kind);
        Ok(Outcome::with_raw(before.as_bytes().to_vec(), canonical(before)))
    }}
}}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {{
    use semio_repo_test_host::{{parse_json, Context, Json, Outcome}};
    use semio_s_plugin_norm::artifacts::{mod}::standards::v1::subsets::any::schema::mutations::{{apply_{slug}_mutation, decode_{slug}_mutation_json, inverse_{slug}_mutation, {ty}Mutation}};
    use semio_s_plugin_norm::artifacts::{mod}::standards::v1::subsets::any::schema::snapshot::{{decode_{slug}_dsl, decode_{slug}_pack, decode_{slug}_snapshot_json, encode_{slug}_dsl, encode_{slug}_pack, encode_{slug}_snapshot_json, {ty}Snapshot}};
    use semio_s_plugin_stdio_test_oracle::law;

    //#region 🔖️FixtureDecode
    /// 🧫️ Decodes the SAME committed fixture text `../🦀️component.rs::fixture_text` embeds, through
    /// this subset's own production JSON bridge — real deserialization of the committed bytes, never
    /// a Rust literal transcribed beside them.
    fn snapshot_of(text: &str, label: &str, kind: &str) -> Result<{ty}Snapshot, String> {{
        decode_{slug}_snapshot_json(text).map_err(|error| format!("{case}: the committed {{label}}-snapshot for {{kind:?}} must decode: {{error}}"))
    }}

    fn mutation_of(text: &str, kind: &str) -> Result<{ty}Mutation, String> {{
        decode_{slug}_mutation_json(text).map_err(|error| format!("{case}: the committed mutation payload for {{kind:?}} must decode: {{error}}"))
    }}

    fn projection(snapshot: &{ty}Snapshot) -> Result<Json, String> {{
        parse_json(&encode_{slug}_snapshot_json(snapshot))
    }}

    /// 🚨️ A failure message that names WHAT disagreed, in the same JSON the fixtures are written in,
    /// so a red scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &{ty}Snapshot, expected: &{ty}Snapshot) -> String {{
        format!("{{what}}\\n     got: {{}}\\nexpected: {{}}", encode_{slug}_snapshot_json(got), encode_{slug}_snapshot_json(expected))
    }}
    //#endregion 🔖️FixtureDecode

    //#region 🔖️Handlers
    /// 🎯️ Applies the kind to the committed before-snapshot and asserts the result IS the committed
    /// after-snapshot, under whichever contract the committed `🎯️outcome` declares: an `applied`
    /// vector must be accepted without a diagnostic and must move the projection (`law::
    /// mutation_is_observable`), a `rejected` one must raise a diagnostic and leave the document
    /// bit-identical. A handler that merely returned `Ok` would report a pass having checked nothing.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {{
        move |_ctx: &Context| {{
            let (before, mutation, after, _outcome) = super::fixture_text(kind);
            let base = snapshot_of(before, "before", kind)?;
            let expected = snapshot_of(after, "after", kind)?;
            let mutation = mutation_of(mutation, kind)?;
            let status = super::committed_status(kind);
            let applied = apply_{slug}_mutation(&base, &mutation);
            let current = match (status.as_str(), applied) {{
                ("applied", Ok((snapshot, messages))) if messages.is_empty() => snapshot,
                ("applied", Ok((_snapshot, messages))) => return Err(format!("mutate-{{kind}}: the committed vector declares this mutation applied, yet it raised {{messages:?}}")),
                ("applied", Err(error)) => return Err(format!("mutate-{{kind}}: the committed vector declares this mutation applied, yet this implementation refused it: {{error}}")),
                ("rejected", Ok((snapshot, messages))) if messages.is_empty() => return Err(format!("mutate-{{kind}}: the committed vector declares this mutation rejected, yet it raised no diagnostic at all — the document came back as {{}}", encode_{slug}_snapshot_json(&snapshot))),
                ("rejected", Ok((snapshot, _messages))) => snapshot,
                ("rejected", Err(_error)) => base.clone(),
                (other, _) => return Err(format!("mutate-{{kind}}: unknown committed outcome status {{other:?}}")),
            }};
            if current != expected {{
                return Err(disagreement(&format!("mutate-{{kind}}: the applied document does not match the committed after-snapshot"), &current, &expected));
            }}
            let (base_projection, mutated) = (projection(&base)?, projection(&current)?);
            if status == "applied" {{
                law::mutation_is_observable(kind, &mutated, &base_projection, &[])?;
            }} else if law::divergence(&mutated, &base_projection).is_some() {{
                return Err(disagreement(&format!("mutate-{{kind}}: a rejected mutation must leave the document untouched"), &current, &base));
            }}
            Ok(Outcome::with_raw(mutated.to_string().into_bytes(), mutated))
        }}
    }}

    /// ↩️ The metamorphic inverse law, asserted in role through `law::inverse_restores`: applying the
    /// kind and then its OWN computed inverse must restore the committed before-snapshot exactly —
    /// collection POSITION included, not merely membership. A kind the committed outcome declares
    /// `applied` must additionally produce a non-empty inverse, because a mutation that changes the
    /// document and reports nothing to undo silently breaks the event-sourced undo history.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {{
        move |_ctx: &Context| {{
            let (before, mutation, _after, _outcome) = super::fixture_text(kind);
            let base = snapshot_of(before, "before", kind)?;
            let mutation = mutation_of(mutation, kind)?;
            let original = projection(&base)?;
            let mut current = match apply_{slug}_mutation(&base, &mutation) {{
                Ok((snapshot, _messages)) => snapshot,
                Err(error) => return Err(format!("inverse-{{kind}}: the forward mutation could not be applied to its own committed before-snapshot: {{error}}")),
            }};
            let steps = inverse_{slug}_mutation(&mutation, &base);
            if super::committed_status(kind) == "applied" && steps.is_empty() {{
                return Err(format!("inverse-{{kind}}: this kind changes the document, so its computed inverse must not be empty"));
            }}
            for step in &steps {{
                current = apply_{slug}_mutation(&current, step).map_err(|error| format!("inverse-{{kind}}: an inverse step was rejected: {{error}}"))?.0;
            }}
            law::inverse_restores(kind, &projection(&current)?, &original)?;
            if current != base {{
                return Err(disagreement(&format!("inverse-{{kind}}: undoing the mutation did not restore the before-snapshot"), &current, &base));
            }}
            Ok(Outcome::with_raw(original.to_string().into_bytes(), original))
        }}
    }}

    /// 🔁️ The real committed document through every encoding it has. The DSL carrier is deliberately
    /// byte-preserving here — the committed file IS this printer's own canonical output — so
    /// `law::carrier_is_exact` is the correct half of the identity law and the usual
    /// no-byte-pass-through inequality would be the wrong claim. What proves the document was PARSED
    /// rather than copied is the agreement of three independently written codecs: the hand-written
    /// DSL grammar, the hand-written binary pack protocol, and the JSON projection. A shortcut that
    /// handed back its input bytes could not survive the pack leg.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {{
        let text = String::from_utf8(ctx.fixture_bytes(super::DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed {m['title']} artifact is not UTF-8: {{error}}"))?;
        let parsed = decode_{slug}_dsl(&text)?;
        let reprinted = encode_{slug}_dsl(&parsed);
        law::carrier_is_exact(reprinted.as_bytes(), text.as_bytes())?;
        let reparsed = decode_{slug}_dsl(&reprinted)?;
        if reparsed != parsed {{
            return Err(disagreement("identity-round-trip: printing the document back to DSL and reparsing it lost content", &reparsed, &parsed));
        }}
        let repacked = decode_{slug}_pack(&encode_{slug}_pack(&parsed))?;
        if repacked != parsed {{
            return Err(disagreement("identity-round-trip: encoding the document to a pack and decoding it back lost content", &repacked, &parsed));
        }}
        let rejson = decode_{slug}_snapshot_json(&encode_{slug}_snapshot_json(&parsed))?;
        if rejson != parsed {{
            return Err(disagreement("identity-round-trip: encoding the document to JSON and decoding it back lost content", &rejson, &parsed));
        }}{twin_leg}
        let projection = projection(&parsed)?;
        law::round_trip_preserves(&projection(&repacked)?, &projection)?;
        Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
    }}
    //#endregion 🔖️Handlers
}}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario
/// id, so the loop mirrors the feature's `Examples` tables exactly. `identity-round-trip` is
/// deliberately subject-only: the reference answer for every other scenario is a committed JSON
/// document the oracle role can read literally, but the real artifact is committed as DSL{" and pack" if pack_asset else ""} bytes
/// ONLY, and turning those into a document needs this subset's own codec — which the oracle-only
/// build must not link.
pub fn adapter() -> Adapter {{
    let mut built = Adapter::new("rust");
    for kind in KINDS {{
        built = built.oracle(&format!("mutate-{{kind}}"), mutate_oracle_for(kind)).oracle(&format!("inverse-{{kind}}"), inverse_oracle_for(kind));
        #[cfg(feature = "sut")]
        {{
            built = built.subject(&format!("mutate-{{kind}}"), subject::mutate(kind)).subject(&format!("inverse-{{kind}}"), subject::inverse(kind));
        }}
    }}
    #[cfg(feature = "sut")]
    {{
        built = built.subject("identity-round-trip", subject::round_trip);
    }}
    built
}}
//#endregion 🔖️Registration
'''
    write(os.path.join(case_dir, "🦀️component.rs"), adapter)
    print(f"{art}: {len(kinds)} kinds, {len(rejected)} rejected vectors")
