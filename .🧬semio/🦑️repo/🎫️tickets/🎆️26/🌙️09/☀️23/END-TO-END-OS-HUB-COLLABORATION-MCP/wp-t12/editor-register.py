"""🎚️ Registers the editor/viewer state-lane mutation vocabularies (T11 §1) schema-first, from the reviewed authoring
spec `wp-t12/editor-vectors.json` (hand-authored before/mutation/after per scenario, plus the harness-measured `diff`
and `messages` recorded after review):

- the committed vectors: `<surface>/🧫️fixtures/<leaf>/<scenario>/{📸️snapshot/⬅️before,📸️snapshot/➡️after,🦠️mutation,🔺️diff,🎯️outcome}/🔣️.json`;
- `<surface>/🔮️oracles/🔣️.json`: the catalog, the state-lane manifest (`surface`), the surveyed no-oracle decision and
  one `fixture/v2` manifest per vector;
- the exhaustive case `<subset>/🧪️tests/<lane-emoji>mutate-<catalog>/{🥒️.feature,🦀️.rs}`;
- the production report bridge `<aggregate>_report_json` beside each aggregate (one line over
  `store::os_store::test_support::mutation_report_json`);
- the Rust test-oracle host package on the hosting subset's contribution when no ancestor supplies it.

`--dry` reports without writing; positional args restrict to survey indices."""
import hashlib, json, os, re, sys

root = "/Users/ueli/Documents/semio/"
dry = "--dry" in sys.argv
only = {int(a) for a in sys.argv[1:] if a.isdigit()}
survey = json.load(open(root + ".tmp-ticket/wp-t12/generated/editor-survey.json", encoding="utf-8"))
spec = json.load(open(root + ".tmp-ticket/wp-t12/editor-vectors.json", encoding="utf-8"))
decisions = json.load(open(root + ".tmp-ticket/wp-t12/editor-decisions.json", encoding="utf-8"))
HOST_PACKAGE = {"implementation": "rust", "package": "semio-s-plugin-stdio-test-oracle", "path": "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📦️packages/🦀️rust"}
LANE = {"🎚️config": ("🎚️", "config"), "👥️presence": ("👥️", "presence"), "🫧️transient": ("🫧️", "transient")}
STATUS_EMOJI = {"applied": "✅️", "no-op": "🟰️", "rejected": "⛔️"}
written, problems = [], []


def plain(segment):
    return re.sub(r"^[^a-z0-9]+", "", segment)


def write(path, text):
    written.append(path[len(root):])
    if dry: return
    os.makedirs(os.path.dirname(path), exist_ok=True)
    open(path, "w", encoding="utf-8").write(text)


def dump(value):
    return json.dumps(value, ensure_ascii=False, indent=2) + "\n"


def snake(name):
    return re.sub(r"(?<!^)(?=[A-Z])", "_", name).lower()


def identity(v):
    artifact = v["artifact"]
    slug = re.sub(r"^s\.", "", artifact).replace(".", "-")
    words = [plain(s) for s in v["surface"].split("/") if plain(s) not in ("modes", "windows")]
    catalog = f"{slug}-{v['standard']}-{v['subset']}-{'-'.join(words)}"
    return catalog, f"{catalog}-mutate", f"{catalog}-state-lane-semantics"


def ancestors_supply_host(subset_dir):
    parts = subset_dir.split("/")
    for n in range(len(parts), 0, -1):
        f = root + "/".join(parts[:n]) + "/🔮️oracles/🔣️.json"
        if os.path.exists(f) and any(p.get("package") == HOST_PACKAGE["package"] and p.get("implementation") == "rust" for p in json.load(open(f, encoding="utf-8")).get("oracleHostPackages", [])):
            return True
    return False


for index, v in enumerate(survey):
    if only and index not in only: continue
    entry = spec.get(str(index))
    if entry is None or not entry.get("scenarios"):
        problems.append(f"[{index}] {v['aggregate']}: no authored vectors")
        continue
    if any("diff" not in s or "messages" not in s for s in entry["scenarios"]):
        problems.append(f"[{index}] {v['aggregate']}: vectors not yet measured by the harness")
        continue
    owner, subset_dir, surface = v["owner"], v["subsetDir"], v["surface"]
    lane_dir = surface.split("/")[-1]
    lane_emoji, lane = LANE[lane_dir]
    catalog, capability, decision_id = identity(v)
    leaves = {l["kind"]: l for l in v["leaves"]}
    kinds = [l["kind"] for l in v["leaves"]]
    covered = {s["kind"] for s in entry["scenarios"] if s["status"] == "applied"}
    if set(kinds) - covered:
        problems.append(f"[{index}] {v['aggregate']}: kinds without an applied vector {sorted(set(kinds) - covered)}")
        continue
    std_dir, sub_dir = re.search(r"/🏅️standards/([^/]+)/🪆️subsets/([^/]+)$", subset_dir).groups()
    report_fn = f"{snake(v['aggregate']).removesuffix('_mutation')}_mutation_report_json"
    module = v["aggregate_path"].rsplit("::", 1)[0]
    crate_ident = v["crate_ident"]

    fixture_manifests, rows_by_status = [], {}
    for s in entry["scenarios"]:
        leaf = leaves[s["kind"]]
        scenario = f"{STATUS_EMOJI[s['status']]}{s['kind']}-{s['status']}"
        s["dir"] = f"{leaf['dir']}/{scenario}"
        bundle = f"{root}{owner}/🧫️fixtures/{leaf['dir']}/{scenario}/"
        outcome = {"status": s["status"], "messages": [{"level": m["level"], "code": m["code"]} for m in s["messages"]]}
        files = [("expected-before", "📸️snapshot/⬅️before/🔣️.json", s["before"]), ("mutation", "🦠️mutation/🔣️.json", s["mutation"]), ("expected-after", "📸️snapshot/➡️after/🔣️.json", s["after"]), ("expected-diff", "🔺️diff/🔣️.json", s["diff"]), ("declared-outcome", "🎯️outcome/🔣️.json", outcome)]
        entries = []
        for role, rel, value in files:
            text = dump(value)
            write(bundle + rel, text)
            data = text.encode("utf-8")
            entries.append({"role": role, "path": f"../🧫️fixtures/{leaf['dir']}/{scenario}/{rel}", "mediaType": "application/json", "sha256": "sha256:" + hashlib.sha256(data).hexdigest(), "bytes": len(data)})
        fixture_manifests.append({
            "schema": "semio.repository-test.fixture/v2",
            "id": f"{s['kind']}-{s['status']}",
            "class": "handcrafted",
            "target": {"artifact": v["artifact"], "standard": v["standard"], "subset": v["subset"]},
            "mutation": s["kind"],
            "outcome": s["status"],
            "units": {"length": "unitless", "angle": "unitless"},
            "files": entries,
            "provenance": {"source": "authored", "license": "public-domain (handcrafted by this repository)", "attribution": s.get("attribution", "Handcrafted before/mutation/after vector authored against this state lane's own schema; the diff and diagnostics are the production dispatch's, reviewed against the after-snapshot."), "security": "scanned-clean", "privacy": "no-personal-data"},
            "comparisonProfile": "ordered-json-v1",
            "reproducible": True,
            "family": f"{lane}-state-lane",
            "notes": s["story"],
        })
        rows_by_status.setdefault(s["status"], []).append(s)

    d = decisions[str(index)]
    contribution = {
        "$schema": os.path.relpath(root + "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json", root + owner + "/🔮️oracles"),
        "schemaVersion": 2,
        "_comment": f"🎚️ The {lane} state lane of `{v['artifact']}`'s {plain(surface.split('/')[0])} ({surface}): its mutation vocabulary `{v['aggregate']}` over `{v['snapshot']}`, measured by the plugin bridge on the surface coordinate, claimed by `../../🧪️tests/{lane_emoji}mutate-{catalog}` and backed by one committed vector per declared outcome class.",
        "oracles": [],
        "noOracleDecisions": [{
            "id": decision_id,
            "capabilities": [capability],
            "rationale": d["rationale"],
            "substitutes": ["specification-vectors", "metamorphic-laws"],
            "coversMutations": True,
            "referenceSurvey": {"ecosystemsSearched": d["ecosystems"], "candidatesConsidered": d["candidates"], "whyNoneQualifies": d["whyNone"]},
        }],
        "mutationCatalogs": [{"id": catalog, "capability": capability, "standardDirectoryName": std_dir, "subsetDirectoryName": sub_dir, "kinds": kinds, "vectors": []}],
        "mutationManifests": [{
            "schema": "semio.repository-test.mutation-manifest/v2",
            "artifact": v["artifact"], "standard": v["standard"], "subset": v["subset"], "surface": surface,
            "mutations": [{"id": k, "capability": capability, "payloadSchema": "🧬️schema/🔣️.json", "outcomes": leaves[k]["outcomes"], "productionDispatch": {"operation": k, "bridgeVersion": 1, "variant": leaves[k]["variant"]}, "oracleRequirements": [{"capability": capability, "qualifyingKind": "third-party-library"}]} for k in kinds],
        }],
        "fixtureManifests": fixture_manifests,
    }
    write(root + owner + "/🔮️oracles/🔣️.json", dump(contribution))

    case_name = f"{lane_emoji}mutate-{catalog}"
    case_dir = f"{root}{subset_dir}/🧪️tests/{case_name}/"
    rel_owner = os.path.relpath(root + owner, case_dir)
    def outline(base, title, rows, then):
        table = "\n".join(f"      | {r['kind']} |" for r in rows)
        return f"""  @id-{base}
  @level-exhaustive
  @mode-{'property' if base == 'inverse' else 'conformance'}
  Scenario Outline: {title}
    Given the committed <id> vector under {surface}/🧫️fixtures
    When <id> is applied to that vector's before-snapshot through {report_fn}
    Then {then}
    Examples:
      | id |
{table}
"""
    applied = rows_by_status["applied"]
    feature = f"""@capability-{capability}
@no-oracle-{decision_id}
@comparison-ordered-json-v1
@mutations-{catalog}
Feature: Apply every {lane} state-lane mutation of {v['artifact']}'s {surface} to its committed vector
  `{v['aggregate']}` is the {lane} lane of this subset's {plain(surface.split('/')[0])}: {d['summary']} It is this
  repository's own state record, so the case records the no-oracle decision `{decision_id}` in
  `{surface}/🔮️oracles/🔣️.json` and asserts every law inside the subject handlers through the shared
  `semio_s_plugin_stdio_test_oracle::law::vector` module: the applied snapshot is the committed after-snapshot, the
  produced delta is the committed `🔺️diff`, the diagnostics are the ones the committed `🎯️outcome` declares, an
  applied vector really moves the snapshot, and the mutation's own computed inverse restores the committed
  before-snapshot. Production dispatch is reached through `{report_fn}`, which runs
  `Mutation::diff(..).apply_to` exactly as the store does.

{outline('mutate', 'Apply <id> and land on the committed after-snapshot, diff and outcome', applied, 'the applied snapshot, the produced diff and the diagnostics are exactly what the vector commits, and the snapshot moved')}
{outline('inverse', 'Undoing <id> restores the committed before-snapshot', applied, "the mutation's own inverse steps apply without refusal and restore the before-snapshot exactly")}"""
    if "no-op" in rows_by_status:
        feature += "\n" + outline("keep", "Re-applying <id> to a snapshot that already holds its value is a warned no-op", rows_by_status["no-op"], "the snapshot is unchanged and the only diagnostic is mutation.no-op")
    write(case_dir + "🥒️.feature", feature)

    def vector_arm(s):
        base = f"{rel_owner}/🧫️fixtures/{s['dir']}"
        return f"""        "{s['kind']}" => Vector {{
            before: include_str!("{base}/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("{base}/🦠️mutation/🔣️.json"),
            after: include_str!("{base}/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("{base}/🔺️diff/🔣️.json"),
            outcome: include_str!("{base}/🎯️outcome/🔣️.json"),
            observable: {'false' if s['before'] == s['after'] else 'true'},
        }},"""
    keep = rows_by_status.get("no-op", [])
    keep_fn = f"""
/// 🟰️ The committed no-op vector of one kind: its before-snapshot already holds the value the mutation sets.
fn kept(kind: &str) -> Result<Vector, String> {{
    Ok(match kind {{
{chr(10).join(vector_arm(s) for s in keep)}
        other => return Err(format!("no committed no-op vector for {{other:?}}")),
    }})
}}
""" if keep else ""
    adapter = f"""//! {lane_emoji} `{v['artifact']}` {surface} state-lane mutation case — Rust adapter.
//!
//! Recorded no-oracle decision `{decision_id}`: the runner dispatches no oracle role, so every law is asserted inside
//! the subject handlers through `semio_s_plugin_stdio_test_oracle::law::vector` over the report of this crate's
//! production bridge `{report_fn}`. The oracle handlers answer with the committed after- and before-snapshots read
//! literally, so the reference side exists the moment a second producer does. Handlers are registered by Scenario
//! Outline base id and read their kind from the row.

use semio_repo_test_host::{{parse_json, Adapter, Context, Outcome}};
use semio_s_plugin_stdio_test_oracle::law::vector::Vector;

//#region 🔖️Vectors
/// 🧫️ The committed applied vector of one kind, read literally from `{surface}/🧫️fixtures`.
fn vector(kind: &str) -> Result<Vector, String> {{
    Ok(match kind {{
{chr(10).join(vector_arm(s) for s in applied)}
        other => return Err(format!("no committed vector for {{other:?}}")),
    }})
}}
{keep_fn}//#endregion 🔖️Vectors

//#region 🔖️Oracle
fn literal(text: &str) -> Result<Outcome, String> {{
    Ok(Outcome::with_raw(text.as_bytes().to_vec(), parse_json(text)?))
}}

fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {{
    literal(vector(ctx.row()?)?.after)
}}

fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {{
    literal(vector(ctx.row()?)?.before)
}}
{"" if not keep else '''
fn keep_oracle(ctx: &Context) -> Result<Outcome, String> {
    literal(kept(ctx.row()?)?.after)
}
'''}//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {{
    use super::*;
    use semio_s_plugin_stdio_test_oracle::law::vector;
    use {crate_ident}::{module.split('::', 1)[1]}::{report_fn};

    fn report(committed: &Vector) -> Result<String, String> {{
        {report_fn}(committed.before, committed.mutation, committed.after)
    }}

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {{
        let kind = ctx.row()?;
        let committed = vector(kind)?;
        let applied = vector::mutate(kind, &report(&committed)?, &committed)?;
        Ok(Outcome::with_raw(applied.to_string().into_bytes(), applied))
    }}

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {{
        let kind = ctx.row()?;
        let restored = vector::inverse(kind, &report(&vector(kind)?)?)?;
        Ok(Outcome::with_raw(restored.to_string().into_bytes(), restored))
    }}
{"" if not keep else '''
    pub fn keep(ctx: &Context) -> Result<Outcome, String> {
        let kind = ctx.row()?;
        let committed = kept(kind)?;
        let unchanged = vector::mutate(kind, &report(&committed)?, &committed)?;
        Ok(Outcome::with_raw(unchanged.to_string().into_bytes(), unchanged))
    }
'''}}}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {{
    let mut built = Adapter::new("rust").oracle("mutate", mutate_oracle).oracle("inverse", inverse_oracle){'.oracle("keep", keep_oracle)' if keep else ''};
    #[cfg(feature = "sut")]
    {{
        built = built.subject("mutate", subject::mutate).subject("inverse", subject::inverse){'.subject("keep", subject::keep)' if keep else ''};
    }}
    built
}}
//#endregion 🔖️Registration
"""
    write(case_dir + "🦀️.rs", adapter)

    agg_file = f"{root}{owner}/🧬️schema/🧬️mutations/🦀️.rs"
    source = open(agg_file, encoding="utf-8").read()
    if f"fn {report_fn}(" not in source:
        bridge = f"""
//#region 🌉️TestBridge
/// 🌉️ The committed-vector report of this state lane for the language-neutral case adapter, which links only this
/// crate: production dispatch (`Mutation::diff(..).apply_to`) and the mutation's own inverse over `{v['snapshot']}`.
///
/// @see store::os_store::test_support::mutation_report_json
pub fn {report_fn}(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {{
    store::os_store::test_support::mutation_report_json::<{v['snapshot']}, {v['aggregate']}>(base_json, mutation_json, after_json)
}}
//#endregion 🌉️TestBridge
"""
        write(agg_file, source.rstrip("\n") + "\n" + bridge)

    if not ancestors_supply_host(subset_dir):
        sub_file = f"{root}{subset_dir}/🔮️oracles/🔣️.json"
        doc = json.load(open(sub_file, encoding="utf-8"))
        doc["oracleHostPackages"] = doc.get("oracleHostPackages", []) + [HOST_PACKAGE]
        text = open(sub_file, encoding="utf-8").read()
        write(sub_file, json.dumps(doc, ensure_ascii=False, indent=1 if text.startswith('{\n "') else 2) + "\n")

    entry["catalog"] = catalog
print(f"{'would write' if dry else 'wrote'} {len(written)} file(s); {len(problems)} problem(s)")
for p in problems: print("PROBLEM", p)
json.dump(written, open(root + ".tmp-ticket/wp-t12/generated/editor-register-written.json", "w"), ensure_ascii=False, indent=1)
