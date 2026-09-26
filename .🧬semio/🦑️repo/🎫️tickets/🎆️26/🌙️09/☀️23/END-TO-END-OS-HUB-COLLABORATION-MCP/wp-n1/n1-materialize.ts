/**
 * 🧫️ N1 one-off: turns `.🧬semio/🌐hub/s13-n1-vectors/vectors-<family>.json` (production-derived specification vectors, see
 * `n1-vectors.py`) into each norm subset's test-platform layer, all at once and schema-first:
 *
 * - `🧫️fixtures/🧬️mutations/<leaf>/<scenario>/…` — the committed (before, mutation, after, outcome, diff) vectors,
 * - `🔮️oracles/🔣️.json` — the mutation manifest generated from the leaf descriptors (`manifestFromLeafDescriptors`),
 *   the catalog (kinds in `dsl::Mutations` declaration order + one vector per kind),
 * - `🧪️tests/<emoji>mutate-<family>-1/{🥒️.feature, 🦀️.rs, 🐍️.py}` — the differential case, subject + oracle adapters.
 *
 *   bun n1-materialize.ts [--write] [family …]
 */
import { existsSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { basename, dirname, join } from "node:path";
import { manifestFromLeafDescriptors } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts";
import { leadingEmojiIdentity, loadCatalogTaxonomy, pathEmojiStatuteFindings } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";

const ROOT = "/Users/ueli/Documents/semio";
const VECTORS = `${ROOT}/.🧬semio/🌐hub/s13-n1-vectors`;
const FAMILIES: Record<string, { dir: string; title: string; crate: string; type: string }> = {
  din16798: { dir: "🌬️din16798", title: "DIN 16798", crate: "din16798", type: "Din16798" },
  din18599: { dir: "⚡️din18599", title: "DIN V 18599", crate: "din18599", type: "Din18599" },
  din4108: { dir: "🧱️din4108", title: "DIN 4108", crate: "din4108", type: "Din4108" },
  en1990: { dir: "⚖️en1990", title: "EN 1990", crate: "en1990", type: "En1990" },
  en1991: { dir: "🏋️en1991", title: "EN 1991", crate: "en1991", type: "En1991" },
  en1992: { dir: "🏛️en1992", title: "EN 1992", crate: "en1992", type: "En1992" },
  en1993: { dir: "🔩️en1993", title: "EN 1993", crate: "en1993", type: "En1993" },
  en1994: { dir: "🧩️en1994", title: "EN 1994", crate: "en1994", type: "En1994" },
  en1995: { dir: "🪵️en1995", title: "EN 1995", crate: "en1995", type: "En1995" },
  en1996: { dir: "🪨️en1996", title: "EN 1996", crate: "en1996", type: "En1996" },
  en1997: { dir: "🌍️en1997", title: "EN 1997", crate: "en1997", type: "En1997" },
  en1998: { dir: "🫨️en1998", title: "EN 1998", crate: "en1998", type: "En1998" },
  en1999: { dir: "🪶️en1999", title: "EN 1999", crate: "en1999", type: "En1999" },
  iso16757: { dir: "📇️iso16757", title: "ISO 16757", crate: "iso16757", type: "Iso16757" },
  vdi3805: { dir: "🏭️vdi3805", title: "VDI 3805", crate: "vdi3805", type: "Vdi3805" },
};
const VERB_EMOJI: Record<string, string> = { sets: "✏️", removes: "➖️", appends: "➕️", inserts: "➕️", swaps: "🔀️", applies: "🎯️" };
/** 🪆️ Owning subset of a kind the prior manifest never declared, for the subsets that split one artifact (`🪆️subsets/🔣️.json`). */
const DEFAULT_SUBSET: Record<string, (kind: string) => string> = { en1990: () => "combination", din4108: (kind) => (/layer/u.test(kind) ? "layers" : "envelope") };
const write = process.argv.includes("--write");
const selected = process.argv.slice(2).filter((arg) => !arg.startsWith("--"));
const taxonomy = loadCatalogTaxonomy(ROOT) as { pathEmojiPolicy: { genericEmojiIdentities: string[] } };

type Vector = { kind: string; example: string; rule: string; scenario: string; before: unknown; mutation: unknown; after: unknown; diff: unknown; outcome: { status: string }; derivation: string };
type Descriptor = { semanticKind: string; aggregateVariant: string; owner: string; emoji: string; displayName: string };
type Record_ = { family: string; examples: string[]; vectors: Record<string, Vector>; missing: string[]; descriptors: Record<string, Descriptor> };

const canonical = (name: string): boolean => {
  const identity = leadingEmojiIdentity(name);
  return !!identity.first && /^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(identity.rest) && pathEmojiStatuteFindings([{ path: name, nodeKind: "directory" }], taxonomy.pathEmojiPolicy.genericEmojiIdentities).length === 0;
};
const withEmoji = (emoji: string, rest: string, fallbacks: string[]): string => {
  for (const candidate of [emoji, `${emoji.replace(/\uFE0F/gu, "")}\uFE0F`, ...fallbacks]) if (candidate && canonical(`${candidate}${rest}`)) return `${candidate}${rest}`;
  throw new Error(`no canonical directory name for ${rest}`);
};
const pretty = (value: unknown): string => `${JSON.stringify(value, null, 2)}\n`;
const subsetDir = (family: string): string => `${ROOT}/✏️s/🔌️plugins/📕️norm/🗿️artifacts/${FAMILIES[family]!.dir}/🏅️standards/🔖️1/🪆️subsets/✳️any`;
const owner = (family: string): string => subsetDir(family).slice(ROOT.length + 1);
const caseDirName = (family: string): string => `${leadingEmojiIdentity(FAMILIES[family]!.dir).first}mutate-${family}-1`;

function plan(family: string) {
  const record = JSON.parse(readFileSync(`${VECTORS}/vectors-${family}.json`, "utf8")) as Record_;
  const order = Object.keys(record.descriptors);
  const rows = order.filter((kind) => record.vectors[kind]).map((kind) => {
    const vector = record.vectors[kind]!;
    const descriptor = record.descriptors[kind]!;
    const leaf = basename(descriptor.owner);
    const mutationDirectoryName = withEmoji(descriptor.emoji || (leadingEmojiIdentity(leaf).first ?? ""), kind, ["🧬️", "✏️"]);
    const verb = vector.scenario.split("-")[0]!;
    const scenarioDirectoryName = withEmoji(VERB_EMOJI[verb] ?? "🎯️", vector.scenario, ["🎯️", "✏️"]);
    return { kind, vector, leaf, mutationDirectoryName, scenarioDirectoryName };
  });
  return { record, order, rows };
}

function oracleContribution(family: string, rows: ReturnType<typeof plan>["rows"], order: string[]): unknown {
  const path = `${subsetDir(family)}/🔮️oracles/🔣️.json`;
  const current = JSON.parse(readFileSync(path, "utf8")) as Record<string, unknown>;
  const baseline = current.$schema === undefined ? (JSON.parse(Bun.spawnSync(["git", "show", `f7791a96178:${owner(family)}/🔮️oracles/🔣️.json`], { cwd: ROOT }).stdout.toString()) as Record<string, unknown>) : current;
  const oracles = (baseline.oracles as { id: string; kind?: string; capabilities: string[] }[]) ?? [];
  const mutateOracle = oracles.find((oracle) => oracle.capabilities.includes(`${family}-1-mutate`));
  if (!mutateOracle) throw new Error(`${family}: no oracle supplies ${family}-1-mutate`);
  const manifest = manifestFromLeafDescriptors(ROOT, owner(family), `${family}-1-mutate`);
  if (!manifest) throw new Error(`${family}: leaf descriptors do not yield a manifest`);
  const prior = new Map(((current.mutationManifests as { mutations: { id: string; subset?: string }[] }[] | undefined) ?? []).flatMap((entry) => entry.mutations.map((mutation) => [mutation.id, mutation] as const)));
  const mutations = order.map((kind) => {
    const derived = manifest.mutations.find((mutation) => mutation.id === kind)!;
    const subset = prior.get(kind)?.subset ?? DEFAULT_SUBSET[family]?.(kind);
    return { ...derived, oracleRequirements: [{ capability: `${family}-1-mutate`, qualifyingKind: mutateOracle.kind }], ...(subset === undefined ? {} : { subset }) };
  });
  const catalog = {
    id: `${family}-1-any`,
    capability: `${family}-1-mutate`,
    standardDirectoryName: "🔖️1",
    subsetDirectoryName: "✳️any",
    vectors: rows.map((row) => ({ mutationId: row.kind, sourceMutationDirectoryName: row.leaf, mutationDirectoryName: row.mutationDirectoryName, scenarios: [{ id: row.vector.scenario, directoryName: row.scenarioDirectoryName }] })),
    kinds: order,
  };
  const { semanticKinds: _dropped, family: _family, mutations: _mutations, capabilities: _capabilities, mutationKinds: _kinds, ...kept } = baseline as Record<string, unknown>;
  return { ...kept, mutationCatalogs: [catalog], mutationManifests: [{ ...manifest, mutations }] };
}

function feature(family: string, rows: ReturnType<typeof plan>["rows"], dslAsset: string, record: Record_): string {
  const { title, type } = FAMILIES[family]!;
  const width = (pick: (row: (typeof rows)[number]) => string, header: string) => Math.max(header.length, ...rows.map((row) => pick(row).length));
  const [wi, wd, wf] = [width((row) => row.kind, "id"), width((row) => row.mutationDirectoryName, "dir"), width((row) => row.scenarioDirectoryName, "fixture")];
  const table = [`      | ${"id".padEnd(wi)} | ${"dir".padEnd(wd)} | ${"fixture".padEnd(wf)} |`, ...rows.map((row) => `      | ${row.kind.padEnd(wi)} | ${row.mutationDirectoryName.padEnd(wd)} | ${row.scenarioDirectoryName.padEnd(wf)} |`)].join("\n");
  const derived = rows.filter((row) => row.vector.derivation === "from_snapshot").length;
  return `@capability-${family}-1-mutate
@oracle-${family}-1-python-independent
@comparison-ordered-json-v1
@mutations-${family}-1-any
Feature: Apply every typed ${title} mutation against an independent Python implementation
  \`s.norm.${family}\` is a semio-native artifact: no third-party library reads or writes it, so the second producer
  this differential comparison needs is a second implementation — \`semio_norm_vocabulary\`, the one independent
  Python engine of the norm mutation vocabulary, imported by \`🐍️.py\` beside this file. It resolves every kind from
  the repository's written derivation rules and this subset's committed catalog, never from the Rust it judges.

  Every row below is one committed specification vector of \`${type}Mutation\`, ${rows.length} kinds in
  \`#[derive(dsl::Mutations)]\` declaration order. Each vector starts from a committed example document, and its
  mutation is what production's own editor derivation (\`from_snapshot\`) emits for one rule-based single edit of
  that document (${derived} kinds) or what the leaf's own payload schema addresses in it
  (${rows.length - derived} kinds); production dispatch applied it without a diagnostic and its own inverse
  restored the example. Both implementations read the SAME committed bytes through the \`shared://\` URIs below.

  Each side asserts the same laws in role: the applied document must BE the committed after-snapshot, an \`applied\`
  vector must move the document, and the mutation followed by its OWN computed inverse must restore the
  before-snapshot exactly. \`inverse-\` projects both the mutated and the restored document.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed specification vector
    Given the committed before-snapshot shared://🧬️mutations/<dir>/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload shared://🧬️mutations/<dir>/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot shared://🧬️mutations/<dir>/<fixture>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome shared://🧬️mutations/<dir>/<fixture>/🎯️outcome/🔣️.json
    When both implementations apply the committed mutation to the committed before-snapshot
    Then each reaches the committed after-snapshot under the committed outcome status and the two agree
    Examples:
${table}

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores its committed before-snapshot
    Given the committed before-snapshot shared://🧬️mutations/<dir>/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload shared://🧬️mutations/<dir>/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot shared://🧬️mutations/<dir>/<fixture>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome shared://🧬️mutations/<dir>/<fixture>/🎯️outcome/🔣️.json
    When each implementation applies the committed mutation and then its OWN computed inverse
    Then both restore the before-snapshot and agree on the mutated and the restored document
    Examples:
${table}

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed ${title} document from the parsed carrier
    Given the real committed text artifact ${dslAsset}
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
`;
}

function pythonAdapter(family: string, rows: ReturnType<typeof plan>["rows"], dslAsset: string, envelope: string): string {
  const { title } = FAMILIES[family]!;
  return `"""🐍️ ${title}'s contribution to the norm reference implementation — the four things that are genuinely
per-standard, and nothing else: the committed kind list, the committed specification vectors, the real committed
document and its envelope token. The engine is \`semio_norm_vocabulary\`, imported, never copied.
"""

from __future__ import annotations

# region 🔖️Imports
from importlib import import_module

_vocabulary = import_module("🐍️")
Subset = _vocabulary.Subset
build_adapter = _vocabulary.build_adapter

# endregion 🔖️Imports


# region 🔖️Vocabulary
#: 🏷️ Every kind this subset's committed catalog declares, in catalog order.
KINDS = [
${rows.map((row) => `    ${JSON.stringify(row.kind)},`).join("\n")}
]

#: 🧫️ The committed specification vector each kind publishes, as (triad directory, fixture name).
VECTORS = {
${rows.map((row) => `    ${JSON.stringify(row.kind)}: (${JSON.stringify(row.mutationDirectoryName)}, ${JSON.stringify(row.scenarioDirectoryName)}),`).join("\n")}
}

#: 🗣️ The real committed ${title} document, read where the domain already keeps it.
DSL_ASSET = ${JSON.stringify(dslAsset)}

#: ✉️ The envelope token that artifact's text preamble must carry.
ENVELOPE = ${JSON.stringify(envelope)}
# endregion 🔖️Vocabulary


# region 🔖️Registration
def adapter():
    """🧭️ Oracle role only: registering these handlers as subjects as well would make the reference its own subject."""
    return build_adapter(Subset(${JSON.stringify(title)}, KINDS, VECTORS, DSL_ASSET, ENVELOPE, vector_root="shared://🧬️mutations"))
# endregion 🔖️Registration
`.replace(/"([^"\\]*)"/gu, (match) => match.replace(/\\u([0-9a-f]{4})/giu, (_m, hex) => String.fromCharCode(parseInt(hex, 16))));
}

function rustAdapter(family: string, rows: ReturnType<typeof plan>["rows"], dslAsset: string): string {
  const { title, crate, type } = FAMILIES[family]!;
  const t = crate;
  const arms = rows.map((row) => {
    const base = `../../🧫️fixtures/🧬️mutations/${row.mutationDirectoryName}/${row.scenarioDirectoryName}`;
    return `        ${JSON.stringify(row.kind)} => (
            include_str!("${base}/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("${base}/🦠️mutation/🔣️.json"),
            include_str!("${base}/📸️snapshot/➡️after/🔣️.json"),
            include_str!("${base}/🎯️outcome/🔣️.json"),
        ),`;
  }).join("\n");
  return `//! 🦀️ ${title} exhaustive mutation case — the SUBJECT half: this repository's own production dispatch
//! (\`apply_${t}_mutation\`, \`inverse_${t}_mutation\`) over every committed specification vector, decoded through the
//! subset's production JSON bridges (\`decode_${t}_mutation_json\`, \`decode_${t}_snapshot_json\`). The oracle half is
//! \`🐍️.py\` beside this file. Every law is asserted in role through the shared \`semio_s_plugin_stdio_test_oracle::law\`
//! helpers, so a subject that merely returned \`Ok\` could not pass.

use semio_repo_test_host::{digest, parse_json, Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::law;

//#region 🔖️Kinds
/// 🏷️ Every kind of \`${type}Mutation\`, in \`#[derive(dsl::Mutations)]\` declaration order — the committed catalog's list.
#[cfg(feature = "sut")]
const KINDS: &[&str] = &[
${rows.map((row) => `    ${JSON.stringify(row.kind)},`).join("\n")}
];

/// 🗣️ The real committed ${title} document the identity round trip reads.
#[cfg(feature = "sut")]
const DSL_ASSET: &str = ${JSON.stringify(dslAsset)};
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ The committed \`(before, mutation, after, outcome)\` vector of one kind — the same bytes the Python oracle reads.
#[cfg(feature = "sut")]
fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str, &'static str) {
    match kind {
${arms}
        other => panic!("mutate-${family}-1: no committed fixture is registered for kind {other:?}"),
    }
}

/// 🧾️ Parses one committed fixture document.
#[cfg(feature = "sut")]
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("committed fixture JSON must parse: {error}"))
}

/// 🎯️ The committed outcome status of one kind's vector.
#[cfg(feature = "sut")]
fn committed_status(kind: &str) -> String {
    let (_before, _mutation, _after, outcome) = fixture_text(kind);
    canonical(outcome).str("status")
}

/// 🗣️ The carrier-level projection of a re-emitted DSL document.
#[cfg(feature = "sut")]
fn carrier_projection(text: &str) -> Json {
    let (preamble, body) = text.split_once('\\n').unwrap_or((text, ""));
    let body = body.strip_suffix('\\n').unwrap_or(body);
    let lines = if body.is_empty() { Vec::new() } else { body.split('\\n').map(|line| Json::String(line.to_string())).collect::<Vec<Json>>() };
    Json::Object(vec![
        ("preamble".to_string(), Json::String(preamble.to_string())),
        ("lines".to_string(), Json::Array(lines)),
        ("dslDigest".to_string(), Json::String(digest(text.as_bytes()))),
        ("dslLength".to_string(), Json::Number(text.as_bytes().len() as f64)),
    ])
}
//#endregion 🔖️Fixtures

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_artifact_norm_${t}::standards::v1::subsets::any::schema::mutations::{apply_${t}_mutation, decode_${t}_mutation_json, inverse_${t}_mutation, ${type}Mutation};
    use semio_s_artifact_norm_${t}::standards::v1::subsets::any::schema::snapshot::{decode_${t}_dsl, decode_${t}_pack, decode_${t}_snapshot_json, encode_${t}_dsl, encode_${t}_pack, encode_${t}_snapshot_json, ${type}Snapshot};
    use semio_s_plugin_stdio_test_oracle::law;

    /// 🧫️ Decodes a committed snapshot through the subset's production JSON bridge.
    fn snapshot_of(text: &str, label: &str, kind: &str) -> Result<${type}Snapshot, String> {
        decode_${t}_snapshot_json(text).map_err(|error| format!("mutate-${family}-1: the committed {label}-snapshot for {kind:?} must decode: {error}"))
    }

    /// 🦠️ Decodes a committed mutation payload through the subset's production JSON bridge.
    fn mutation_of(text: &str, kind: &str) -> Result<${type}Mutation, String> {
        decode_${t}_mutation_json(text).map_err(|error| format!("mutate-${family}-1: the committed mutation payload for {kind:?} must decode: {error}"))
    }

    /// 🔣️ The JSON projection both implementations are compared on.
    fn projection(snapshot: &${type}Snapshot) -> Result<Json, String> {
        parse_json(&encode_${t}_snapshot_json(snapshot))
    }

    /// 🚨️ A failure message naming what disagreed, in the fixtures' own JSON.
    fn disagreement(what: &str, got: &${type}Snapshot, expected: &${type}Snapshot) -> String {
        format!("{what}\\n     got: {}\\nexpected: {}", encode_${t}_snapshot_json(got), encode_${t}_snapshot_json(expected))
    }

    /// 🎯️ Applies the kind to the committed before-snapshot and asserts the result IS the committed after-snapshot
    /// under the committed outcome: \`applied\` must raise no diagnostic and move the projection, \`rejected\` must leave it.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let (before, mutation, after, _outcome) = super::fixture_text(kind);
            let base = snapshot_of(before, "before", kind)?;
            let expected = snapshot_of(after, "after", kind)?;
            let mutation = mutation_of(mutation, kind)?;
            let status = super::committed_status(kind);
            let current = match (status.as_str(), apply_${t}_mutation(&base, &mutation)) {
                ("applied", Ok((snapshot, messages))) if messages.is_empty() => snapshot,
                ("applied", Ok((_snapshot, messages))) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet it raised {messages:?}")),
                ("applied", Err(error)) => return Err(format!("mutate-{kind}: the committed vector declares this mutation applied, yet this implementation refused it: {error}")),
                ("rejected", Ok((snapshot, messages))) if messages.is_empty() => return Err(format!("mutate-{kind}: the committed vector declares this mutation rejected, yet it raised no diagnostic — the document came back as {}", encode_${t}_snapshot_json(&snapshot))),
                ("rejected", Ok((snapshot, _messages))) => snapshot,
                ("rejected", Err(_error)) => base.clone(),
                (other, _) => return Err(format!("mutate-{kind}: unknown committed outcome status {other:?}")),
            };
            if current != expected {
                return Err(disagreement(&format!("mutate-{kind}: the applied document does not match the committed after-snapshot"), &current, &expected));
            }
            let (base_projection, mutated) = (projection(&base)?, projection(&current)?);
            if status == "applied" {
                law::mutation_is_observable(kind, &mutated, &base_projection, &[])?;
            } else if law::divergence(&mutated, &base_projection).is_some() {
                return Err(disagreement(&format!("mutate-{kind}: a rejected mutation must leave the document untouched"), &current, &base));
            }
            Ok(Outcome::with_raw(mutated.to_string().into_bytes(), mutated))
        }
    }

    /// ↩️ Applies the kind and then its OWN computed inverse; the committed before-snapshot must come back exactly.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let (before, mutation, _after, _outcome) = super::fixture_text(kind);
            let base = snapshot_of(before, "before", kind)?;
            let mutation = mutation_of(mutation, kind)?;
            let original = projection(&base)?;
            let mut current = match apply_${t}_mutation(&base, &mutation) {
                Ok((snapshot, _messages)) => snapshot,
                Err(error) => return Err(format!("inverse-{kind}: the forward mutation could not be applied to its own committed before-snapshot: {error}")),
            };
            let mutated = projection(&current)?;
            let steps = inverse_${t}_mutation(&mutation, &base);
            if super::committed_status(kind) == "applied" && steps.is_empty() {
                return Err(format!("inverse-{kind}: this kind changes the document, so its computed inverse must not be empty"));
            }
            for step in &steps {
                current = apply_${t}_mutation(&current, step).map_err(|error| format!("inverse-{kind}: an inverse step was rejected: {error}"))?.0;
            }
            let restored = projection(&current)?;
            law::inverse_restores(kind, &restored, &original)?;
            if current != base {
                return Err(disagreement(&format!("inverse-{kind}: undoing the mutation did not restore the before-snapshot"), &current, &base));
            }
            let projection = Json::Object(vec![("mutated".to_string(), mutated), ("restored".to_string(), restored)]);
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// 🔁️ The real committed document through every encoding it has: DSL (byte-exact), pack and JSON must agree.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let text = String::from_utf8(ctx.fixture_bytes(super::DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed ${title} artifact is not UTF-8: {error}"))?;
        let parsed = decode_${t}_dsl(&text)?;
        let reprinted = encode_${t}_dsl(&parsed);
        law::carrier_is_exact(reprinted.as_bytes(), text.as_bytes())?;
        let reparsed = decode_${t}_dsl(&reprinted)?;
        if reparsed != parsed {
            return Err(disagreement("identity-round-trip: printing the document back to DSL and reparsing it lost content", &reparsed, &parsed));
        }
        let repacked = decode_${t}_pack(&encode_${t}_pack(&parsed))?;
        if repacked != parsed {
            return Err(disagreement("identity-round-trip: encoding the document to a pack and decoding it back lost content", &repacked, &parsed));
        }
        let rejson = decode_${t}_snapshot_json(&encode_${t}_snapshot_json(&parsed))?;
        if rejson != parsed {
            return Err(disagreement("identity-round-trip: encoding the document to JSON and decoding it back lost content", &rejson, &parsed));
        }
        law::round_trip_preserves(&projection(&repacked)?, &projection(&parsed)?)?;
        Ok(Outcome::with_raw(reprinted.as_bytes().to_vec(), super::carrier_projection(&reprinted)))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration by full expanded scenario id, mirroring the feature's \`Examples\` tables exactly.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        for kind in KINDS {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind)).subject(&format!("inverse-{kind}"), subject::inverse(kind));
        }
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
`;
}

function dslAssetOf(family: string, record: Record_): { uri: string; envelope: string } {
  const preferred = record.examples.find((asset) => asset.includes("demo")) ?? record.examples[record.examples.length - 1] ?? record.examples[0]!;
  const text = readFileSync(`${subsetDir(family)}/🖼️assets/${preferred}`, "utf8");
  const envelope = text.split("\n")[0]!.trim().split(/\s+/u).find((token) => token.includes(".dsl")) ?? `norm.${family}.dsl`;
  return { uri: `asset://${preferred}`, envelope };
}

for (const family of selected.length ? selected : Object.keys(FAMILIES)) {
  const { record, order, rows } = plan(family);
  const dsl = dslAssetOf(family, record);
  const dir = subsetDir(family);
  const caseDir = `${dir}/🧪️tests/${caseDirName(family)}`;
  const staleCases = existsSync(`${dir}/🧪️tests`) ? readdirSync(`${dir}/🧪️tests`).filter((name) => /mutate-/u.test(name) && name !== caseDirName(family)) : [];
  console.log(`${family}: ${rows.length}/${order.length} kinds vectored; missing ${JSON.stringify(record.missing)}; case ${basename(caseDir)}; stale cases ${JSON.stringify(staleCases)}; dsl ${dsl.uri} (${dsl.envelope})`);
  if (!write) continue;
  for (const obsolete of ["🧫️fixtures/🧬️mutations", "🎫️fixtures"]) rmSync(`${dir}/${obsolete}`, { recursive: true, force: true });
  for (const row of rows) {
    const base = `${dir}/🧫️fixtures/🧬️mutations/${row.mutationDirectoryName}/${row.scenarioDirectoryName}`;
    for (const [rel, value] of [["📸️snapshot/⬅️before", row.vector.before], ["🦠️mutation", row.vector.mutation], ["📸️snapshot/➡️after", row.vector.after], ["🎯️outcome", row.vector.outcome], ["🔺️diff", row.vector.diff]] as const) {
      mkdirSync(`${base}/${rel}`, { recursive: true });
      writeFileSync(`${base}/${rel}/🔣️.json`, pretty(value));
    }
  }
  writeFileSync(`${dir}/🔮️oracles/🔣️.json`, pretty(oracleContribution(family, rows, order)));
  for (const stale of staleCases) rmSync(`${dir}/🧪️tests/${stale}`, { recursive: true, force: true });
  mkdirSync(caseDir, { recursive: true });
  writeFileSync(`${caseDir}/🥒️.feature`, feature(family, rows, dsl.uri, record));
  writeFileSync(`${caseDir}/🐍️.py`, pythonAdapter(family, rows, dsl.uri, dsl.envelope));
  writeFileSync(`${caseDir}/🦀️.rs`, rustAdapter(family, rows, dsl.uri));
}
