"""Clones the `change-annex` leaf as `change-reliability-class` through its whole registration footprint of one norm artifact."""
import copy, json, os, re
import rc_lib
from rc_lib import rd, wr, clone_lines, sub_once, ART, SUBSET, NORM

NEW_DIR = "🎯change-reliability-class"
TAXONOMY = NORM + "/🧪️tests/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json"


def lc(s):
    s = re.sub(r"[^\x00-\x7f]+change-annex", NEW_DIR, s)
    for a, b in [("change-annex", "change-reliability-class"), ("change_annex", "change_reliability_class"), ("ChangedAnnex", "ChangedReliabilityClass"), ("ChangeAnnex", "ChangeReliabilityClass"),
                 ("new_annex", "new_reliability_class"), ("newAnnex", "newReliabilityClass"), ("new-annex", "new-reliability-class"),
                 ('path: "annex"', 'path: "reliabilityClass"'), ("semio_s_plugin_norm_compliance::AnnexChoice", "semio_s_plugin_norm_compliance::ReliabilityClass"),
                 ("AnnexChoice::En", "ReliabilityClass::RC1"), ("AnnexChoice::De", "ReliabilityClass::RC3")]:
        s = s.replace(a, b)
    return re.sub(r"\bAnnexChoice\b", "ReliabilityClass", s)


def find_annex_dir(M):
    hits = [d for d in os.listdir(M) if d.endswith("change-annex") and os.path.isdir(f"{M}/{d}")]
    assert len(hits) == 1, hits
    return hits[0]


def leaf_sources(R, art, Art, M, annex_dir):
    A = f"{M}/{annex_dir}"
    D = f"{M}/{NEW_DIR}"
    for rel in ["🦀️.rs", "🔺️diff/🦀️.rs", "↩️inverse/🦀️.rs"]:
        p = f"{A}/{rel}"
        if not os.path.exists(p):
            continue
        t = lc(rd(p))
        t = t.replace("Change national annex", "Change reliability class").replace("Nationalen Anhang ändern", "Zuverlässigkeitsklasse ändern")
        t = t.replace('entity: "annex"', 'entity: "reliability-class"').replace("The annex already has this value.", "reliability_class already has this value.")
        t = t.replace("🌍️ ", "🎯 ").replace("//! 🌍️", "//! 🎯").replace("sets the document's annex", "sets the document's reliability class")
        t = re.sub(r"\bbase\.annex\b", "base.reliability_class", t)
        t = re.sub(r"\bannex: Some\(", "reliability_class: Some(", t)
        t = re.sub(r"\bannex: base\.", "reliability_class: base.", t)
        t = t.replace("self.annex", "self.reliability_class")
        t = re.sub(r"\bAnnexChoice\b", "ReliabilityClass", t)
        for a, b in [("national annex selection", "reliability class selection"), ("Change national annex to", "Change reliability class to"), ("Nationalen Anhang auf", "Zuverlässigkeitsklasse auf"), ("Annex already has this value.", "Reliability class already has this value."), ("annex selection", "reliability class selection"), ("annex unchanged", "reliability class unchanged"), ("National annex already set.", "Reliability class already set."), ("sets the national annex; an annex the document already holds", "sets the reliability class; a class the document already holds"), ("the annex scalar", "the reliability class"), ("BASE's annex", "BASE's reliability class")]:
            t = t.replace(a, b)
        t = t.replace(" (national annex)", "").replace("`annex`", "`reliability_class`").replace(".annex`", ".reliability_class`")
        left = re.findall(r"[^\n]*[Aa]nnex[^\n]*", t)
        if left:
            print("[leaf] annex left in", rel, [x.strip()[:100] for x in left])
        wr(f"{D}/{rel}", t)
    j = json.loads(rd(f"{A}/🔣️.json"))
    tags = []
    for d in os.listdir(M):
        f = f"{M}/{d}/🔣️.json"
        if os.path.exists(f):
            try:
                tags.append(json.loads(rd(f)).get("binaryTag") or 0)
            except ValueError:
                pass
    j.update(owner=j["owner"].replace(annex_dir, NEW_DIR), semanticKind="change-reliability-class", displayName="Change Reliability Class", emoji="🎯", aggregateVariant="ChangeReliabilityClass", binaryTag=max(tags) + 1)
    wr(f"{D}/🔣️.json", json.dumps(j, indent=2, ensure_ascii=False) + "\n")
    sj = {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "$id": f"https://json.schemas.assets.semio-tech.com/s/norm/{art}/mutation/change-reliability-class/schema.json",
        "title": "ChangeReliabilityClass", "type": "object", "additionalProperties": False, "required": ["newReliabilityClass"],
        "properties": {"newReliabilityClass": {"type": "string", "enum": list(rc_lib.RC), "x-semio-ui": {"role": "value", "label": {"en": "Reliability class RC", "de": "Zuverlässigkeitsklasse RC"},
                       "description": {"en": "RC1, RC2 or RC3 (EN 1990 Annex B); K_FI of the class scales the partial factors of the governing combination.", "de": "RC1, RC2 oder RC3 (EN 1990 Anhang B); K_FI der Klasse skaliert die Teilsicherheitsbeiwerte der maßgebenden Kombination."},
                       "widget": "select", "group": "value", "order": 20}, "x-semio-rust": {"type": "semio_s_plugin_norm_compliance::ReliabilityClass"}}}}
    wr(f"{D}/🧬️schema/🔣️.json", json.dumps(sj, indent=2, ensure_ascii=False) + "\n")
    ts = ('/** 🎯 `change-reliability-class` wire twin: the leaf payload `ChangeReliabilityClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`\n'
          ' * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema\'s bounds stay Ajv\'s.\n * @see ./🔣️.json */\n'
          'import { normWireLiteral, normWireObject, type NormWireReader, normWireRequired } from "@semio-tech/norm-js/registry/contract";\n\n'
          'export interface ChangeReliabilityClass {\n  newReliabilityClass: "RC1" | "RC2" | "RC3";\n}\n\n'
          'export const parseChangeReliabilityClass: NormWireReader<ChangeReliabilityClass> = normWireObject<ChangeReliabilityClass>({ newReliabilityClass: normWireRequired(normWireLiteral("RC1", "RC2", "RC3")) });\n')
    wr(f"{D}/🧬️schema/🟦️.ts", ts)
    return max(tags) + 1


def fixtures(F, annex_dir):
    A = f"{F}/{annex_dir}"
    D = f"{F}/{NEW_DIR}"
    if not os.path.isdir(A):
        return
    for scen, rc in [("✅apply", "RC3"), ("🟰noop", "RC2")]:
        if not os.path.isdir(f"{A}/{scen}"):
            continue
        if os.path.exists(f"{A}/{scen}/🔣️.mutation.json"):
            flat_fixtures(A, D, scen, rc)
            continue
        for dirpath, _, names in os.walk(f"{A}/{scen}"):
            for n in sorted(names):
                src = f"{dirpath}/{n}"
                rel = src[len(A) + 1:]
                t = rd(src)
                if "🦠️mutation" in rel:
                    t = json.dumps({"ChangeReliabilityClass": {"newReliabilityClass": rc}}, indent=2) + "\n"
                elif "🔺️diff" in rel:
                    t = re.sub(r'"annex": "[A-Za-z]+"', '"annex": null', t)
                    t = t.replace('"reliabilityClass": null', f'"reliabilityClass": "{rc}"') if scen == "✅apply" else t
                elif "➡️after" in rel:
                    t = None
                if t is not None:
                    wr(f"{D}/{rel}", t)
        before = rd(f"{D}/{scen}/🔣️.before.json")
        wr(f"{D}/{scen}/🔣️.after.json", before.replace('"reliabilityClass": "RC2"', f'"reliabilityClass": "{rc}"') if scen == "✅apply" else before)


def flat_fixtures(A, D, scen, rc):
    """Flat fixture layout: `🔣️.{mutation,before,diff,after,outcome}.json` next to each other."""
    for kind in ["mutation", "before", "diff", "after", "outcome"]:
        src = f"{A}/{scen}/🔣️.{kind}.json"
        if not os.path.exists(src):
            continue
        t = rd(src)
        if kind == "mutation":
            t = json.dumps({"mutation": "changeReliabilityClass", "newReliabilityClass": rc}, indent=2) + "\n"
        elif kind == "diff":
            t = re.sub(r'"annex": "[A-Za-z]+"', '"annex": null', t)
            t = re.sub(r'"reliabilityClass": null', f'"reliabilityClass": "{rc}"', t) if scen == "✅apply" else t
        elif kind == "after" and scen == "✅apply":
            t = re.sub(r'("reliabilityClass":\s*)"RC2"', rf'\1"{rc}"', t)
        wr(f"{D}/{scen}/🔣️.{kind}.json", t)


def line_register(files, conv):
    for p in files:
        text = rd(p)
        if "reliability-class" in text.replace("change-reliability-class", "x") and "ChangeReliabilityClass" in text:
            continue
        if re.search(r"change-reliability-class|ChangeReliabilityClass|change_reliability_class", text):
            continue
        pattern = r"^[ \t]*\|[^\n]*(?:change-annex|change_annex|ChangeAnnex)[^\n]*\n" if p.endswith(".feature") else r"^[^\n]*(?:change-annex|change_annex|ChangeAnnex)[^\n]*\n"
        clone_lines(p, pattern, conv)


def json_lists(obj, visit):
    if isinstance(obj, list):
        out = []
        for x in obj:
            out.append(x)
            new = visit(x)
            if new is not None:
                out.append(new)
        obj[:] = out
        for x in obj:
            json_lists(x, visit)
    elif isinstance(obj, dict):
        for v in obj.values():
            json_lists(v, visit)


def conv_json(x):
    return json.loads(lc(json.dumps(x, ensure_ascii=False)))


def oracles_register(path):
    d = json.loads(rd(path))
    if "change-reliability-class" in rd(path):
        return
    def visit(x):
        if x == "change-annex":
            return "change-reliability-class"
        if isinstance(x, dict) and x.get("mutationId") == "change-annex":
            n = conv_json(x)
            return n
        if isinstance(x, dict) and x.get("id") == "change-annex" and "productionDispatch" in x:
            return conv_json(x)
        return None
    json_lists(d, visit)
    wr(path, json.dumps(d, indent=2, ensure_ascii=False) + "\n")


def mutations_json_register(path):
    if "ChangeReliabilityClass" in rd(path):
        return
    d = json.loads(rd(path))
    def visit(x):
        if isinstance(x, dict) and x.get("required") == ["ChangeAnnex"]:
            return conv_json(x)
        return None
    json_lists(d, visit)
    wr(path, json.dumps(d, indent=2, ensure_ascii=False) + "\n")


def taxonomy_register(art_dir_name, annex_dir):
    t = rd(TAXONOMY)
    d = json.loads(t)
    if any(r["artifact"] == art_dir_name and r["kind"] == "change-reliability-class" for r in d["rows"]):
        return False
    rows = []
    n = 0
    for r in d["rows"]:
        rows.append(r)
        if r["artifact"] == art_dir_name and r["kind"] == "change-annex":
            c = conv_json(r)
            c["entity"] = "reliability-class"
            if "edit" in c:
                c["edit"]["path"] = "reliabilityClass"
            rows.append(c)
            n += 1
    assert n == 1, (art_dir_name, n)
    d["rows"] = rows
    wr(TAXONOMY, json.dumps(d, indent=2, ensure_ascii=False) + "\n")
    return True


def proto_register(path, annex_msg="ChangeAnnex"):
    t = rd(path)
    if "ChangeReliabilityClass" in t:
        return
    m = re.search(r"^    ChangeAnnex change_annex = (\d+);\n", t, re.M)
    numbers = [int(x) for x in re.findall(r"^    \w+ \w+ = (\d+);", t, re.M)]
    t = t[:m.end()] + f"    ChangeReliabilityClass change_reliability_class = {max(numbers) + 1};\n" + t[m.end():]
    mm = re.search(r"^message ChangeAnnex \{ [^\n]*\}\n", t, re.M)
    if mm:
        t = t[:mm.end()] + "message ChangeReliabilityClass { string new_reliability_class = 1; }\n" + t[mm.end():]
    else:
        mm = re.search(r"^message ChangeAnnex \{\n(?:[^\n]*\n)*?\}\n", t, re.M)
        t = t[:mm.end()] + "message ChangeReliabilityClass {\n  string new_reliability_class = 1;\n}\n" + t[mm.end():]
    wr(path, t)


def graphql_register(path):
    t = rd(path)
    if "ChangeReliabilityClass" in t:
        return
    if re.search(r"union \w+ = ChangeAnnex \|", t):
        t = re.sub(r"(union \w+ = ChangeAnnex)( \|)", r"\1 | ChangeReliabilityClass\2", t, count=1)
    else:
        m = re.search(r"^  ChangeAnnex\n", t, re.M)
        assert m, path[-60:]
        t = t[:m.end()] + "  | ChangeReliabilityClass\n" + t[m.end():]
    m = re.search(r"^type ChangeAnnex \{ ([^\n]*)\}\n", t, re.M)
    if m:
        body = m.group(1).replace("newAnnex: AnnexChoice!", "newReliabilityClass: String!")
        t = t[:m.end()] + "type ChangeReliabilityClass { " + body + "}\n" + t[m.end():]
    else:
        m = re.search(r"^type ChangeAnnex \{\n(?:[^\n]*\n)*?\}\n\n?", t, re.M)
        block = m.group(0).replace("ChangeAnnex", "ChangeReliabilityClass").replace("newAnnex: AnnexChoice", "newReliabilityClass: String").replace("newAnnex", "newReliabilityClass")
        t = t[:m.end()] + block + t[m.end():]
    wr(path, t)


def protocol_semio_register(path, tag):
    t = rd(path)
    if "record change-reliability-class " in t:
        return
    wr(path, t.rstrip("\n") + f"\nrecord change-reliability-class tag={tag}\nfield payload bytes\n")


def footprint(R, annex_dir):
    out = {"line": [], "json": [], "proto": [], "graphql": [], "semio": [], "mount": []}
    for dirpath, _, names in os.walk(R):
        if "/fixtures/" in dirpath + "/" or annex_dir in dirpath or NEW_DIR in dirpath:
            continue
        for n in names:
            p = f"{dirpath}/{n}"
            ext = n.rsplit(".", 1)[-1]
            if ext not in ("rs", "ts", "py", "feature", "json", "proto", "graphql", "semio"):
                continue
            if "🔬️unit" in p or "🔬️kinds-catalog" in p or "🧪️tests/🔬️" in p or "🧪️tests/🧪️" in p:
                continue
            try:
                t = open(p, encoding="utf-8").read()
            except UnicodeDecodeError:
                continue
            if not re.search(r"change-annex|change_annex|ChangeAnnex", t):
                continue
            if ext == "json":
                out["json"].append(p)
            elif ext == "proto":
                out["proto"].append(p)
            elif ext == "graphql":
                out["graphql"].append(p)
            elif ext == "semio":
                out["semio"].append(p)
            elif re.search(r'#\[path = "\.\./[^"]*change-annex[^"]*"\]', t):
                out["mount"].append(p)
            else:
                out["line"].append(p)
    return out


def mount_register(path):
    t = rd(path)
    if NEW_DIR in t:
        return
    m = re.search(r'#\[path = "\.\./[^"]*change-annex/🦀️\.rs"\]\npub mod change_annex;\n', t)
    assert m, path[-60:]
    wr(path, t[:m.end()] + f'#[path = "../{NEW_DIR}/🦀️.rs"]\npub mod change_reliability_class;\n' + t[m.end():])
