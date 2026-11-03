"""v11 (copy of v10 for wall_poisson_ratio): clones the member kind into one kind per wall plan field (wall_start_x_m, wall_start_y_m) and mounts them into the existing `mut-members-wall-rows` crate. Original: clones the member leaf `change-member-wall-thickness-m` into one kind per masonry wall row field (idempotent, run with python -I): crate `mut-members-wall-rows`, leaf dirs, fixtures, mutations root (rs, ts, json, proto, graphql), text codec, binary protocol, edit rules, demo cases, mutate harness, oracle manifest, taxonomy."""
import json, os, re, shutil

ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm"
ART = ROOT + "/🗿️artifacts/🫨️en1998"
ANY = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any"
MUT = ANY + "/🧬️schema/🧬️mutations"
FIX = ANY + "/🧪️tests/🧫️fixtures/🧬️mutations"
import sys
CFG = {
    "member": dict(ent="member", src="change-member-effective-stiffness-ratio", old=dict(snake="effective_stiffness_ratio", kebab="effective-stiffness-ratio", camel="effectiveStiffnessRatio", pascal="EffectiveStiffnessRatio", en="Effective share of the gross wall stiffness (EN 1998-1 4.3.1(6)): 0 = simplified method with the cracked-stiffness factor of the registry, 0 < ratio <= 1 = effective stiffness declared from a more accurate analysis (1 = uncracked)", de="Wirksamer Anteil der Bruttosteifigkeit der Wand (EN 1998-1 4.3.1(6)): 0 = vereinfachtes Verfahren mit dem Faktor der gerissenen Steifigkeit aus dem Register, 0 < Anteil <= 1 = aus genauerer Berechnung angegebene wirksame Steifigkeit (1 = ungerissen)"), new=dict(snake="effective_stiffness_basis", kebab="effective-stiffness-basis", camel="effectiveStiffnessBasis", pascal="EffectiveStiffnessBasis", en="Stiffness basis of the gross wall stiffness (EN 1998-1 4.3.1(6))", de="Steifigkeitsbasis der Brutto-Wandsteifigkeit (EN 1998-1 4.3.1(6))"), field="effective_stiffness_ratio", tok=("member-effective-stiffness-ratio", "MemberEffectiveStiffnessRatio", "member_effective_stiffness_ratio", "members[].effectiveStiffnessRatio"), group="🧱️members-wall-rows", crate="semio_s_artifact_norm_en1998_mut_members_wall_rows", agg="🧱️members", pascal_prefix="ChangeMember", snake_prefix="change_member_", short=("change-member-stiffness-ratio", "change-member-stiffness-basis")),
    "building": dict(ent="building", src="change-building-storey-stiffness-ratio", old=dict(snake="storey_stiffness_ratio", kebab="storey-stiffness-ratio", camel="storeyStiffnessRatio", pascal="StoreyStiffnessRatio", en="Effective share of the entered gross storey stiffnesses (EN 1998-1 4.3.1(6)): 0 = simplified method with the cracked-stiffness factor of the registry, 0 < ratio <= 1 = effective stiffness declared from a more accurate analysis (1 = entered values are already effective)", de="Wirksamer Anteil der eingegebenen Brutto-Geschosssteifigkeiten (EN 1998-1 4.3.1(6)): 0 = vereinfachtes Verfahren mit dem Faktor der gerissenen Steifigkeit aus dem Register, 0 < Anteil <= 1 = aus genauerer Berechnung angegebene wirksame Steifigkeit (1 = eingegebene Werte sind bereits wirksam)"), new=dict(snake="storey_stiffness_basis", kebab="storey-stiffness-basis", camel="storeyStiffnessBasis", pascal="StoreyStiffnessBasis", en="Stiffness basis of the entered gross storey stiffnesses (EN 1998-1 4.3.1(6))", de="Steifigkeitsbasis der eingegebenen Brutto-Geschosssteifigkeiten (EN 1998-1 4.3.1(6))"), field="storey_stiffness_ratio", tok=("building-storey-stiffness-ratio", "BuildingStoreyStiffnessRatio", "building_storey_stiffness_ratio", "buildings[].storeyStiffnessRatio"), group="🏢️buildings-envelope", crate="semio_s_artifact_norm_en1998_mut_buildings_envelope", agg="🏢️buildings", pascal_prefix="ChangeBuilding", snake_prefix="change_building_", short=("change-building-stiffness-ratio", "change-building-stiffness-basis")),
}[sys.argv[1]]
SRC = CFG["src"]
OLD = CFG["old"]
SPEC = json.load(open("/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️scripts/field-adder/specs/en1998-effective-stiffness.json"))


def words(s):
    return s.split("_")


NEW = [dict(CFG["new"])]
TOK = CFG["tok"]
GROUP = CFG["group"]
CRATE = CFG["crate"]


def rd(p):
    return open(p, encoding="utf-8").read()


def wr(p, s):
    assert len(p[len("/home/deck/semio/"):].encode()) < 256, p
    os.makedirs(os.path.dirname(p), exist_ok=True)
    open(p, "w", encoding="utf-8").write(s)


def tr(s, n):
    for k in ("snake", "kebab", "camel", "pascal"):
        s = s.replace(OLD[k], n[k])
    s = s.replace(CFG["short"][0], CFG["short"][1])
    return s.replace("Change " + OLD["en"], "Change " + n["en"]).replace(OLD["de"] + " ändern", n["de"] + " ändern").replace(OLD["en"], n["en"]).replace(OLD["de"], n["de"])


def has(l):
    return any(t in l for t in TOK)


def clone_lines(path, block=None, skip=lambda l: False):
    text = rd(path)
    if "change-" + CFG["ent"] + "-" + NEW[0]["kebab"] in text or CFG["pascal_prefix"] + NEW[0]["pascal"] in text or CFG["snake_prefix"] + NEW[0]["snake"] in text:
        return
    lines, out, i = text.split("\n"), [], 0
    while i < len(lines):
        l = lines[i]
        if has(l) and not skip(l):
            n = block(lines, i) if block else 1
            chunk = lines[i:i + n]
            out += chunk
            for d in NEW:
                out += [tr(x, d) for x in chunk]
            i += n
        else:
            out.append(l)
            i += 1
    wr(path, "\n".join(out))


def arm(lines, i):
    j = i
    while lines[j].strip() != "),":
        j += 1
    return j - i + 1


def next_tag(s):
    return max(int(x) for x in re.findall(r"tag=(\d+)", s)) + 1


ENUM = "semio_s_artifact_norm_en1998_model::StiffnessBasis"


def retype(dst, n):
    for dp, _, fs in os.walk(dst):
        for f in fs:
            p = os.path.join(dp, f)
            s = rd(p)
            s = s.replace("pub new_%s: f64," % n["snake"], "pub new_%s: %s," % (n["snake"], ENUM))
            i = s.find("    if !payload.new_%s.is_finite() {" % n["snake"])
            if i >= 0:
                j = s.index("    }\n", i) + len("    }\n")
                s = s[:i] + s[j:]
            if f == "🔣️.json" and dp.endswith("🧬️schema"):
                j = json.loads(s)
                prop = j["properties"]["new" + n["pascal"]]
                ui = {k: v for k, v in prop["x-semio-ui"].items() if k not in ("step", "precision")}
                ui["widget"] = "select"
                j["properties"]["new" + n["pascal"]] = {"type": "string", "enum": ["simplified", "declared"], "x-semio-ui": ui, "x-semio-rust": {"type": ENUM}}
                s = json.dumps(j, indent=2, ensure_ascii=False) + "\n"
            wr(p, s)


def fixtures(n):
    fsrc, fdst = FIX + "/✏️" + CFG["short"][0], FIX + "/✏️" + CFG["short"][1]
    if os.path.exists(fdst):
        return
    key_b, key_r = n["camel"], OLD["camel"]
    for dp, _, fs in os.walk(fsrc):
        for f in fs:
            p = os.path.join(dp, f)
            s = rd(p)
            if f.endswith("after.json"):
                s = s.replace('"%s": "declared"' % key_b, '"%s": "simplified"' % key_b, 1)
                s = s.replace('"%s": 0.5' % key_r, '"%s": 1.0' % key_r, 1)
            elif f.endswith("diff.json"):
                s = s.replace('"%s": null' % key_b, '"%s": "simplified"' % key_b, 1)
                s = s.replace('"%s": 0.5' % key_r, '"%s": null' % key_r, 1)
            elif f.endswith("mutation.json"):
                s = tr(s, n).replace("0.5", '"simplified"')
            wr(fdst + p[len(fsrc):], s)


def main():
    src = MUT + "/✏️" + SRC
    tag = max(int(x) for x in re.findall(r"tag=(\d+)", rd(ANY + "/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"))) + 1
    n = NEW[0]
    kind = "change-" + CFG["ent"] + "-" + n["kebab"]
    dst = MUT + "/✏️" + kind
    if os.path.exists(dst):
        return
    for dp, _, fs in os.walk(src):
        for f in fs:
            p = os.path.join(dp, f)
            s = tr(rd(p).replace(SRC, kind), n)
            if p.endswith("/🔣️.json") and "binaryTag" in s:
                s = re.sub(r'"binaryTag": \d+', '"binaryTag": %d' % tag, s)
            wr(dst + p[len(src):], s)
    retype(dst, n)
    fixtures(n)
    group = MUT + "/" + GROUP
    wr(group + "/🦀️.rs", rd(group + "/🦀️.rs").replace("//#endregion 🪢️Mounts", '#[path = "../✏️%s/🦀️.rs"]\npub mod %s;\n//#endregion 🪢️Mounts' % (kind, kind.replace("-", "_"))))
    bp = MUT + "/" + CFG["agg"] + "/🦀️.rs"
    old_use = "pub use %s::{" % CRATE
    t = rd(bp)
    assert old_use in t
    wr(bp, t.replace(old_use, old_use + kind.replace("-", "_") + ", ", 1))
    clone_lines(MUT + "/🦀️.rs")
    clone_lines(MUT + "/🧭️edit-rules/🦀️.rs")
    clone_lines(MUT + "/🎲️demo-cases/🦀️.rs")
    p = MUT + "/🎲️demo-cases/🦀️.rs"
    s = rd(p)
    s = s.replace("new_%s: 1.5 }" % n["snake"], "new_%s: %s::Simplified }" % (n["snake"], ENUM))
    wr(p, s)
    clone_lines(ANY + "/🚪️io/📝️text/🧬️mutations/🦀️.rs")
    clone_lines(ANY + "/🧪️tests/🫨️mutate-en1998-1/🥒️.feature")
    clone_lines(ANY + "/🧪️tests/🫨️mutate-en1998-1/🐍️.py")
    clone_lines(ANY + "/🧪️tests/🫨️mutate-en1998-1/🦀️.rs", lambda ls, i: arm(ls, i) if ls[i].rstrip().endswith("=> (") else 1, lambda l: "include_str!" in l)
    p = MUT + "/🔣️.json"
    j = json.loads(rd(p))
    refs = [x.get("$ref", "") for x in j["oneOf"]]
    k = next(i for i, r in enumerate(refs) if SRC + "/" in r)
    j["oneOf"].insert(k + 1, {"$ref": refs[k].replace(SRC, kind)})
    wr(p, json.dumps(j, indent=2, ensure_ascii=False) + "\n")
    p = ANY + "/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"
    wr(p, rd(p).rstrip("\n") + "\nrecord %s tag=%d\nfield payload bytes\n" % (kind, tag))
    p = ANY + "/🔮️oracles/🔣️.json"
    o = json.loads(rd(p))
    cat, man = o["mutationCatalogs"][0], o["mutationManifests"][0]
    v = next(x for x in cat["vectors"] if x["mutationId"] == SRC)
    m = next(x for x in man["mutations"] if x["id"] == SRC)
    iv, im, ik = cat["vectors"].index(v), man["mutations"].index(m), cat["kinds"].index(SRC)
    cat["vectors"].insert(iv + 1, json.loads(tr(json.dumps(v, ensure_ascii=False).replace(SRC, kind), n)))
    man["mutations"].insert(im + 1, json.loads(tr(json.dumps(m, ensure_ascii=False).replace(SRC, kind), n)))
    cat["kinds"].insert(ik + 1, kind)
    wr(p, json.dumps(o, indent=2, ensure_ascii=False) + "\n")
    p = ROOT + "/🧪️tests/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json"
    t = json.loads(rd(p))
    i = next(i for i, x in enumerate(t["rows"]) if x["kind"] == SRC and x["artifact"] == "🫨️en1998")
    t["rows"].insert(i + 1, json.loads(tr(json.dumps(t["rows"][i], ensure_ascii=False).replace(SRC, kind), n)))
    wr(p, json.dumps(t, indent=2, ensure_ascii=False) + "\n")


main()
