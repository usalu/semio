"""Layer 1 of the factor-override cascade: model, root schema struct, snapshot and root twins (json, ts, graphql, proto), parse test, examples."""
import glob
from fo_lib import *

SNAP_URL = "https://json.schemas.assets.semio-tech.com/s/norm/en1990/snapshot.json#/$defs/"
OPTIONS = {
    "gamma": ("γ partial factor", "γ Teilsicherheitsbeiwert"),
    "psi0": ("ψ₀ combination value", "ψ₀ Kombinationsbeiwert"),
    "psi1": ("ψ₁ frequent value", "ψ₁ Beiwert für häufige Werte"),
    "psi2": ("ψ₂ quasi-permanent value", "ψ₂ Beiwert für quasi-ständige Werte"),
    "xi": ("ξ reduction factor", "ξ Abminderungsbeiwert"),
    "k_fi": ("K_FI consequence factor", "K_FI Zuverlässigkeitsfaktor"),
}


def ui(en, de, **extra):
    return {"label": {"en": en, "de": de}, **extra}


def props(root):
    return {
        "id": {"type": "string"},
        "factor": {"type": "string", "x-semio-ui": ui("Factor kind", "Faktorart", widget="select", options={k: {"en": v[0], "de": v[1]} for k, v in OPTIONS.items()}, description={"en": "gamma, psi0, psi1, psi2, xi or k_fi: the annex factor this row replaces for the project.", "de": "gamma, psi0, psi1, psi2, xi oder k_fi: der Anhangfaktor, den diese Zeile projektbezogen ersetzt."})},
        "target": {"type": "string", "x-semio-ui": ui("Target action or group", "Zielwirkung oder -gruppe", widget="text", description={"en": "Action id, exclusive group id or ψ category the factor applies to; empty = every action (xi, k_fi are always global).", "de": "Einwirkungs-Id, Ausschlussgruppe oder ψ-Kategorie, auf die der Faktor wirkt; leer = alle Einwirkungen (xi, k_fi gelten immer global)."})},
        "value": {"type": "number", "x-semio-ui": ui("Override value", "Überschreibungswert", widget="stepper", precision=3, step=0.01)},
        "justification": {"type": "string", "x-semio-ui": ui("Justification", "Begründung", widget="text", description={"en": "Why the project deviates from the National Annex (required for every override).", "de": "Warum das Projekt vom Nationalen Anhang abweicht (für jede Überschreibung erforderlich)."})},
    }


def definition(root):
    p = props(root)
    d = {} if root else {"title": ENTITY}
    d.update({"type": "object", "additionalProperties": False, "required": list(p), "properties": p})
    if not root:
        d["x-semio-formats"] = ["🔣️jsonschema", "🔗️graphql", "🛰️protobuf"]
    return d


def json_twin(path, key):
    t = rd(path)
    d = json.loads(t)
    if CAMEL in d["properties"]:
        return
    root = key == "definitions"
    prefix = "#/definitions/" if root else "#/$defs/"
    out = {}
    for k, v in d[key].items():
        out[k] = v
        if k == "Placement":
            out[ENTITY] = definition(root)
    assert ENTITY in out
    d[key] = out
    pr = {}
    for k, v in d["properties"].items():
        pr[k] = v
        if k == "cases":
            pr[CAMEL] = {"type": "array", "items": {"$ref": prefix + ENTITY}, "x-semio-state": "artifact"}
    d["properties"] = pr
    d["required"].append(CAMEL)
    wr(path, json.dumps(d, indent=2, ensure_ascii=False) + "\n")
    print("  edited", path.split("/✳️any/")[-1])


def ts_twin(path, prefix, root):
    def go(t):
        if "FactorOverride" in t:
            return t
        name = prefix + ENTITY
        t = put(t, "  cases: CaseRow[];\n}", "  cases: CaseRow[];\n  /** 🎚️ @state artifact */\n  %s: %s[];\n}" % (CAMEL, name))
        iface = "export interface %s {\n  id: string;\n  factor: string;\n  target: string;\n  value: number;\n  justification: string;\n}\n\n" % name
        m = re.search(r"export interface %sPlacement \{.*?\n\}\n\n" % prefix, t, re.S)
        t = t[:m.end()] + iface + t[m.end():]
        parser = "export const parse%s: NormWireReader<%s> = normWireObject<%s>({ id: normWireRequired(normWireString), factor: normWireRequired(normWireString), target: normWireRequired(normWireString), value: normWireRequired(normWireNumber), justification: normWireRequired(normWireString) });\n" % (name, name, name)
        m = re.search(r"^export const parse%sPlacement: .*\n" % prefix, t, re.M)
        t = t[:m.end()] + parser + t[m.end():]
        root_parser = "parseEn1990Artifact" if root else "parseEn1990Snapshot"
        m = re.search(r"^export const %s: .*$" % root_parser, t, re.M)
        line = m.group(0)
        tail = "cases: normWireRequired(normWireArray(normWireRef(() => parseCaseRow))) });"
        assert line.endswith(tail), line[-120:]
        line = line[:-4] + ", %s: normWireRequired(normWireArray(normWireRef(() => parse%s))) });" % (CAMEL, name)
        return t[:m.start()] + line + t[m.end():]
    edit(path, go)


def graphql_twin(path):
    def go(t):
        if "type FactorOverride" in t:
            return t
        block = "type FactorOverride {\n  id: String!\n  factor: String!\n  target: String!\n  value: Float!\n  justification: String!\n}\n\n"
        m = re.search(r"type Placement \{.*?\n\}\n\n", t, re.S)
        t = t[:m.end()] + block + t[m.end():]
        return put(t, "  cases: [CaseRow!]!\n}", "  cases: [CaseRow!]!\n  %s: [FactorOverride!]!\n}" % CAMEL)
    edit(path, go)


def proto_twin(path):
    def go(t):
        if "message FactorOverride" in t:
            return t
        broken = "double conversimessage Placement { string id = 1; double x = 2; double y = 3; double z = 4; double yaw = 5; }\non_factor = 8; double required = 9; }\n"
        if broken in t:
            t = t.replace(broken, "double conversion_factor = 8; double required = 9; }\nmessage Placement { string id = 1; double x = 2; double y = 3; double z = 4; double yaw = 5; }\n")
        m = re.search(r"^message Placement \{[^\n]*\}\n", t, re.M)
        t = t[:m.end()] + "message FactorOverride { string id = 1; string factor = 2; string target = 3; double value = 4; string justification = 5; }\n" + t[m.end():]
        m = re.search(r"message En1990(?:Snapshot|Artifact) \{.*?\n\}", t, re.S)
        body = m.group(0)
        number = max(int(n) for n in re.findall(r"= (\d+);", body)) + 1
        number = max(number, 101) if "En1990Artifact" in body else number
        return t.replace(body, body[:-1] + "  repeated FactorOverride %s = %d;\n}" % (COLL, number))
    edit(path, go)


def model():
    p = SNAP + "/🦀️.rs"

    def go(t):
        if "pub struct FactorOverride" in t:
            return t
        t = put(t, "    pub cases: Vec<CaseRow>,\n}", "    pub cases: Vec<CaseRow>,\n    #[dsl(table)]\n    #[state(artifact)]\n    pub %s: Vec<%s>,\n}" % (COLL, ENTITY))
        t = put(t, "                texts: vec![],\n            }],\n        }\n    }\n}", "                texts: vec![],\n            }],\n            %s: vec![],\n        }\n    }\n}" % COLL)
        m = re.search(r"((?:^#\[[^\n]*\n)+)pub struct Placement \{.*?\n\}\n", t, re.M | re.S)
        attrs = m.group(1)
        struct = "/// 🎚️ Project override of one annex factor (γ, ψ₀/ψ₁/ψ₂, ξ or K_FI): applied by the combination generator only where the annex allows it; every row deviates from the National Annex and carries its justification.\n" + attrs + "pub struct FactorOverride {\n    pub id: String,\n    /// 🏷️ `gamma`, `psi0`, `psi1`, `psi2`, `xi` or `k_fi`.\n    pub factor: String,\n    /// 🎯️ Action id, exclusive group id or ψ category; empty = every action.\n    pub target: String,\n    /// 🔢️ The replacement value.\n    pub value: f64,\n    /// 📝️ Why the project deviates from the National Annex.\n    pub justification: String,\n}\n\n"
        return t[:m.end()] + "\n" + struct.rstrip("\n") + "\n" + t[m.end():]
    edit(p, go)

    def root(t):
        if COLL in t:
            return t
        t = put(t, "    pub cases: Vec<semio_s_artifact_norm_en1990_model::CaseRow>,\n}", "    pub cases: Vec<semio_s_artifact_norm_en1990_model::CaseRow>,\n    #[state(artifact)]\n    pub %s: Vec<semio_s_artifact_norm_en1990_model::%s>,\n}" % (COLL, ENTITY))
        t = put(t, "            cases: self.cases.clone(),\n", "            cases: self.cases.clone(),\n            %s: self.%s.clone(),\n" % (COLL, COLL))
        return put(t, "            cases: snapshot.cases,\n", "            cases: snapshot.cases,\n            %s: snapshot.%s,\n" % (COLL, COLL))
    edit(SCH + "/🦀️.rs", root)


def examples():
    for p in glob.glob(SUB + "/📚️examples/*/🦀️.rs"):
        edit(p, lambda t: t if COLL in t else put(t, "        cases: vec![],\n", "        cases: vec![],\n        %s: vec![],\n" % COLL))
    edit(SCH + "/🧪parse-en1990-artifact.bun.ts", lambda t: t if CAMEL in t else put(t, "  placements: [],\n", "  placements: [],\n  cases: [],\n  %s: [],\n" % CAMEL))


def main():
    model()
    json_twin(SNAP + "/🔣️.json", "$defs")
    json_twin(SCH + "/🔣️.json", "definitions")
    ts_twin(SNAP + "/🟦️.ts", "En1990", False)
    ts_twin(SCH + "/🟦️.ts", "", True)
    for p in (SNAP + "/🔗️.graphql", SCH + "/🔗️.graphql"):
        graphql_twin(p)
    for p in (SNAP + "/🛰️.proto", SCH + "/🛰️.proto"):
        proto_twin(p)
    examples()


if __name__ == "__main__":
    main()
