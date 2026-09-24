"""Normalizes the five wfc artifacts' test contributions onto one schema-conforming shape (bitmap's)."""
import json, os, re, sys
ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🀄️wfc/🗿️artifacts"
KEBAB = re.compile(r"[a-z][a-z0-9]*(?:-[a-z0-9]+)+$")
ART = {
    "◻️2d": dict(artifact="s.wfc.wfc2d", cap="wfc2d-1-mutate", catalog="wfc2d-1-any", oracle="wfc2d-python-independent", case="🧩️mutate-wfc2d-1", noun="wfc2d"),
    "🔲️grid2d": dict(artifact="s.wfc.grid2d", cap="wfc-grid2d-1-mutate", catalog="wfc-grid2d-1-any", oracle="wfc-grid2d-python-independent", case="🔲️mutate-grid2d-1", noun="grid2d"),
    "🧱️grid3d": dict(artifact="s.wfc.grid3d", cap="wfc-grid3d-1-mutate", catalog="wfc-grid3d-1-any", oracle="wfc-grid3d-python-independent", case="🧩️mutate-wfc-grid3d-1", noun="grid3d"),
    "🖼️bitmap": dict(artifact="s.wfc.bitmap", cap="wfc-bitmap-1-mutate", catalog="wfc-bitmap-1-any", oracle="wfc-bitmap-python-independent", case="🧩️mutate-bitmap-1", noun="bitmap"),
    "🧊️3d": dict(artifact="s.wfc.wfc3d", cap="wfc3d-1-mutate", catalog="wfc3d-1-any", oracle="wfc3d-python-independent", case="🧩️mutate-wfc3d-1", noun="wfc3d"),
}

def kebab(pascal): return re.sub(r"(?<!^)([A-Z])", r"-\1", pascal).lower()

def enum_variants(text, pattern):
    m = re.search(pattern + r"\s*\{(.*?)\n\}", text, re.S)
    body, depth, names, i = m.group(1), 0, [], 0
    for line in body.split("\n"):
        s = line.strip()
        if depth == 0:
            v = re.match(r"([A-Z][A-Za-z0-9]*)\s*[\({,]", s) or re.match(r"([A-Z][A-Za-z0-9]*)\s*$", s)
            if v: names.append(v.group(1))
        depth += s.count("{") - s.count("}")
    return names

def facts(art):
    sub = f"{ROOT}/{art}/🏅️standards/🔖️1/🪆️subsets/✳️any"
    rs = open(f"{sub}/🧬️schema/🧬️mutations/🦀️.rs", encoding="utf-8").read()
    kinds = re.findall(r'"([a-z0-9-]+)"', re.search(r"pub const KINDS: &\[&str\] = &\[(.*?)\];", rs, re.S).group(1))
    variants = enum_variants(rs, r"pub enum [A-Za-z0-9]+Mutation")
    assert [kebab(v) for v in variants] == kinds or sorted(kebab(v) for v in variants) == sorted(kinds), (art, variants, kinds)
    dsl_path = f"{sub}/🧬️schema/🧬️mutations/📝️text/🦀️.rs"
    tags = None
    if os.path.exists(dsl_path):
        dsl = [kebab(v) for v in enum_variants(open(dsl_path, encoding="utf-8").read(), r"pub enum [A-Za-z0-9]+OperationDsl")]
        assert sorted(dsl) == sorted(kinds), (art, dsl, kinds)
        tags = dsl
    fixtures = {}
    fx = f"{sub}/🧫️fixtures/🧬️mutations"
    for kd in sorted(os.listdir(fx)):
        if not os.path.isdir(f"{fx}/{kd}"): continue
        fixtures[KEBAB.search(kd).group(0)] = (kd, sorted(d for d in os.listdir(f"{fx}/{kd}") if os.path.isdir(f"{fx}/{kd}/{d}")))
    assert sorted(fixtures) == sorted(kinds), (art, sorted(fixtures), kinds)
    return sub, kinds, {kebab(v): v for v in variants}, tags, fixtures

def catalog(meta, kinds, fixtures):
    return {"id": meta["catalog"], "capability": meta["cap"], "standardDirectoryName": "🔖️1", "subsetDirectoryName": "✳️any", "kinds": kinds,
            "vectors": [{"mutationId": k, "sourceMutationDirectoryName": fixtures[k][0], "mutationDirectoryName": fixtures[k][0],
                         "scenarios": [{"id": KEBAB.search(d).group(0), "directoryName": d} for d in fixtures[k][1]]} for k in kinds]}

def manifest(meta, kinds, variant, outcomes):
    return {"schema": "semio.repository-test.mutation-manifest/v2", "artifact": meta["artifact"], "standard": "1", "subset": "any", "standardDirectoryName": "🔖️1", "subsetDirectoryName": "✳️any",
            "mutations": [{"id": k, "capability": meta["cap"], "payloadSchema": "🧬️.schema.json", "outcomes": outcomes(k),
                           "productionDispatch": {"operation": k, "bridgeVersion": 1, "variant": variant[k]},
                           "oracleRequirements": [{"capability": meta["cap"], "qualifyingKind": "verified-native-second-implementation"}]} for k in kinds]}

def protocol(meta, tags):
    lines = ["dialect protocol", f"protocol wfc{meta['noun'].replace('wfc', '')}.mutations" if meta["noun"] not in ("wfc2d", "wfc3d") else f"protocol {meta['noun']}.mutations", "version 1", f"schema wfc.{meta['noun']}.mutations", "start record", "framing record", "",
             f"# Real binary op frame of `{meta['artifact']}@1` mutations, as `dsl::variants_binary::encode_op` writes it:",
             "# `format u8` (always 1) + `tag` (the kind's ordinal in the text facet's `*OperationDsl` variant table, a",
             "# LEB128 varint that is one byte for every tag below 128) are the fixed, protocol-walkable header; each",
             "# record below is one kind at its own tag. Past the header, `body` is the kind's `encode_record_body` bytes",
             "# under that variant's own record spec.",
             "header fixed 2", "field format u8", "field tag u8"]
    for index, kind in enumerate(tags):
        lines += [f"record {kind} tag={index}", "field body bytes"]
    return "\n".join(lines) + "\n"

if __name__ == "__main__":
    for art, meta in ART.items():
        sub, kinds, variant, tags, fixtures = facts(art)
        print(art, len(kinds), "kinds", "tags" if tags else "no-binary-facet", sum(len(v[1]) for v in fixtures.values()), "vectors")

DESCRIPTION = {
    "◻️2d": None,
    "🔲️grid2d": """  This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️.py` in this directory: a second
  implementation of the `s.wfc.grid2d` document and all fourteen typed mutations' diff, apply and
  inverse, written in Python from `../../🧬️schema/📸️snapshot/🔣️.json` and the per-kind payload schemas
  under `../../🧬️schema/🧬️mutations/<kind>/🧬️schema/🔣️.json`. It imports nothing from this repository
  and replays every committed quintet on its own (`python3 🐍️.py`).

  Why a second implementation rather than a third-party library. The published WaveFunctionCollapse
  libraries solve a tiled model; none of them models an editable problem document with an invertible,
  canonically positioned mutation log, and none reads this carrier.

  The persisted document is the PROBLEM only. Every mutation carries its committed before-snapshot to
  its committed after-snapshot, and its inverse carries it back — value AND position, because every
  collection insert lands at its canonical sorted index.
""",
    "🧱️grid3d": """  This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️.py` in this directory: a second
  implementation of the `s.wfc.grid3d` document and all fourteen typed mutations, written in Python
  against the normative JSON Schema rather than ported from the Rust. It re-derives every sparse diff
  and re-applies every committed one, and replays every committed quintet on its own
  (`python3 🐍️.py`).

  Why a second implementation rather than a third-party library. The surveyed WaveFunctionCollapse
  implementations solve a tiled model; none carries a persisted 3D problem document with per-axis cell
  sizes, an authored allow-list and a mutation vocabulary, so none can adjudicate an edit to it.
""",
}

def feature(meta, kinds, fixtures, description):
    rows = [(k, f"{fixtures[k][0]}/{fixtures[k][1][0]}") for k in kinds]
    width = max(len(k) for k, _ in rows)
    vwidth = max(len(v) for _, v in rows)
    table = [f"      | {'id'.ljust(width)} | {'vector'.ljust(vwidth)} |"] + [f"      | {k.ljust(width)} | {v.ljust(vwidth)} |" for k, v in rows]
    vector = """      \"\"\"
      {
        "kind": "<id>",
        "before": "shared://🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json",
        "mutation": "shared://🧬️mutations/<vector>/🦠️mutation/🔣️.json",
        "diff": "shared://🧬️mutations/<vector>/🔺️diff/🔣️.json",
        "outcome": "shared://🧬️mutations/<vector>/🎯️outcome/🔣️.json",
        "after": "shared://🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json"
      }
      \"\"\""""
    head = [f"@capability-{meta['cap']}", f"@oracle-{meta['oracle']}", "@comparison-ordered-json-v1", f"@mutations-{meta['catalog']}",
            f"Feature: Apply every typed {meta['noun']} mutation twice — once in Rust, once in Python — and require the same answer", description.rstrip("\n"), ""]
    body = ["  @id-mutate", "  @level-exhaustive", "  @mode-differential", "  Scenario Outline: The committed <id> vector declares its own kind and moves the document",
            "    Given the committed specification vector for the <id> kind", vector,
            "    Then the committed mutation payload declares the <id> kind",
            "    And the after-snapshot differs from the before-snapshot, or the committed outcome declares the vector a no-op", "    Examples:", *table, "",
            "  @id-inverse", "  @level-exhaustive", "  @mode-differential", "  Scenario Outline: The committed <id> vector changes only what its diff declares, and inverts exactly",
            "    Given the committed specification vector for the <id> kind", vector,
            "    Then every field where the after-snapshot differs from the before-snapshot is declared by the committed diff",
            "    And every field the committed diff declares actually differs",
            "    And the reference's own inverse of the committed mutation restores the before-snapshot exactly", "    Examples:", *table]
    return "\n".join(head + body) + "\n"

def native_oracle(meta, old, vectors):
    nsi = old.get("nativeSecondImplementation", {})
    survey = nsi.get("noThirdPartySurvey", {})
    def candidate(entry):
        if isinstance(entry, dict): return entry
        name, _, reason = entry.partition(" — ")
        return {"package": name.strip(), "reason": (reason or "surveyed; it solves a tiled model and carries no problem document or mutation vocabulary, so it can neither read this carrier nor adjudicate an edit to it.").strip()}
    return {"id": meta["oracle"], "ecosystem": "python", "package": "", "version": "", "capabilities": [meta["cap"]], "comparisonProfiles": ["ordered-json-v1"],
            "license": "AGPL-3.0-only", "testOnly": True,
            "rationale": old["rationale"].replace("`🐍️grid2d-oracle.py`", f"`../../../../../🧪️tests/{meta['case']}/🐍️.py`").replace("🧪️tests/🧩️mutate-wfc-grid3d-1/🐍️.py", f"../../../../../🧪️tests/{meta['case']}/🐍️.py"),
            "kind": "verified-native-second-implementation", "engine": {"family": "none", "implementation": "in-repository second implementation", "version": "0"},
            "productionReachable": False, "networkDuringExecution": False,
            "nativeSecondImplementation": {"format": meta["artifact"], "noThirdPartySurvey": {"ecosystemsSearched": survey.get("ecosystemsSearched", []), "candidatesConsidered": [candidate(c) for c in survey.get("candidatesConsidered", [])]},
                                           "subjectImplementationLanguage": "rust", "secondImplementationLanguage": "python",
                                           "specificationSource": "../🧬️schema/📸️snapshot/🔣️.json; ../🧬️schema/🧬️mutations/🔣️.json; ../🧬️schema/🧬️mutations/<kind>/🧬️schema/🔣️.json; ../../../../../🧫️fixtures/🧬️mutations",
                                           "fixtureCoverage": {"vectors": vectors, "capabilitiesCovered": [meta["cap"]]}}}

def apply():
    for art, meta in ART.items():
        sub, kinds, variant, tags, fixtures = facts(art)
        path = f"{sub}/🔮️oracles/🔣️.json"
        doc = json.load(open(path, encoding="utf-8"))
        vectors = sum(len(v[1]) for v in fixtures.values())
        if art in ("◻️2d", "🔲️grid2d", "🧱️grid3d"):
            doc["oracles"] = [native_oracle(meta, doc["oracles"][0], vectors)]
            doc["noOracleDecisions"] = []
            doc["mutationManifests"] = [manifest(meta, kinds, variant, lambda k: ["applied", "rejected"])]
            feature_path = f"{sub}/🧪️tests/{meta['case']}/🥒️.feature"
            rendered = feature(meta, kinds, fixtures, DESCRIPTION[art] or open(feature_path, encoding="utf-8").read().split("\n  Scenario Outline")[0].split("\n", 5)[5])
            open(feature_path, "w", encoding="utf-8").write(rendered)
        doc["mutationCatalogs"] = [catalog(meta, kinds, fixtures)]
        ordered = {k: doc[k] for k in ["$schema", "schemaVersion", "_comment", "oracles", "noOracleDecisions", "mutationCatalogs", "mutationManifests", "probes", "comparisonProfiles", "comparisonPipelines", "fixtureManifests"] if k in doc}
        ordered.update({k: v for k, v in doc.items() if k not in ordered})
        open(path, "w", encoding="utf-8").write(json.dumps(ordered, ensure_ascii=False, indent=2) + "\n")
        if tags:
            pp = f"{sub}/🧬️schema/🧬️mutations/💾️binary/📡️.protocol.semio"
            head = open(pp, encoding="utf-8").read().split("\n")[:4]
            text = protocol(meta, tags).split("\n")
            open(pp, "w", encoding="utf-8").write("\n".join(head + text[4:]))
        print("normalized", art)

if __name__ == "__main__" and len(sys.argv) > 1 and sys.argv[1] == "apply":
    apply()
