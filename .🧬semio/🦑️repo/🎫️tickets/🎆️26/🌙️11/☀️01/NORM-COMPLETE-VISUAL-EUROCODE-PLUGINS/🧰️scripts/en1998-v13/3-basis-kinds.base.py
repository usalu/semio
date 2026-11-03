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


NEW = []
for f in [x for x in SPEC["fields"] if x["field"] == CFG["field"]]:
    w = words(f["field"])
    NEW.append(dict(value="0.5", snake=f["field"], kebab=f["field"].replace("_", "-"), camel=w[0] + "".join(x[:1].upper() + x[1:] for x in w[1:]), pascal="".join(x[:1].upper() + x[1:] for x in w), en=f["en"], de=f["de"]))
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


def main():
    src = MUT + "/✏️" + SRC
    if not os.path.exists(MUT + "/" + GROUP):
        pass
    tag = next_tag(rd(ANY + "/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"))
    mounts = []
    for k, n in enumerate(NEW):
        kind = "change-" + CFG["ent"] + "-" + n["kebab"]
        dst = MUT + "/✏️" + kind
        mounts.append(kind)
        SHORT = "change-" + CFG["ent"] + "-stiffness-ratio"
        if not os.path.exists(dst):
            for dp, _, fs in os.walk(src):
                for f in fs:
                    p = os.path.join(dp, f)
                    s = tr(rd(p).replace(SRC, kind), n)
                    if p.endswith("/🔣️.json") and "binaryTag" in s:
                        s = re.sub(r'"binaryTag": \d+', '"binaryTag": %d' % (tag + k), s)
                    wr(dst + p[len(src):], s)
        fsrc, fdst = FIX + "/✏️" + SRC, FIX + "/✏️" + SHORT
        if os.path.exists(fdst):
            continue
        for dp, _, fs in os.walk(fsrc):
            for f in fs:
                p = os.path.join(dp, f)
                s = rd(p)
                if "➡️after" in p:
                    s = re.sub(r'("%s": )[0-9.]+' % n["camel"], r"\g<1>%s" % n["value"], s.replace(CFG["base_old"], CFG["base_new"], 1), count=1)
                elif "🔺️diff" in p:
                    s = re.sub(r'("%s": )null' % n["camel"], r"\g<1>%s" % n["value"], s.replace(CFG["base_old"], CFG["base_old"].split(":")[0] + ": null", 1), count=1)
                if "🦠️mutation" in p or "🎯️outcome" in p:
                    s = tr(s, n)
                    s = s.replace("1.5", n["value"])
                wr(fdst + p[len(fsrc):], s)
    group = MUT + "/" + GROUP
    mounts_text = rd(group + "/🦀️.rs")
    if mounts[0].replace("-", "_") not in mounts_text:
        end = "//#endregion 🪢️Mounts"
        wr(group + "/🦀️.rs", mounts_text.replace(end, "".join('#[path = "../✏️%s/🦀️.rs"]\npub mod %s;\n' % (k, k.replace("-", "_")) for k in mounts) + end))
        bp = MUT + "/" + CFG["agg"] + "/🦀️.rs"
        t = rd(bp)
        old_use = "pub use %s::{" % CRATE
        assert old_use in t
        wr(bp, t.replace(old_use, old_use + ", ".join(k.replace("-", "_") for k in mounts) + ", ", 1))
    if False:
        env = MUT + "/🏢️buildings-envelope"
        head = rd(env + "/🦀️.rs").split("//#region")[0]
        head = re.sub(r"^//! .*$", "//! 🧱️ EN 1998 masonry wall row mutation leaves: wall length, direction and storey.", head, count=1, flags=re.M)
        body = "//#region 🪢️Mounts\n" + "".join('#[path = "../✏️%s/🦀️.rs"]\npub mod %s;\n' % (k, k.replace("-", "_")) for k in mounts) + "//#endregion 🪢️Mounts\n"
        wr(group + "/🦀️.rs", head + body)
        c = rd(env + "/📦️packages/🦀️rust/Cargo.toml").replace("mut-buildings-envelope", "mut-members-wall-rows").replace("buildings envelope mutation leaves", "masonry wall row mutation leaves")
        wr(group + "/📦️packages/🦀️rust/Cargo.toml", c)
        ct = ART + "/Cargo.toml"
        t = rd(ct)
        member = '    "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️buildings-envelope/📦️packages/🦀️rust",\n'
        assert member in t
        t = t.replace(member, member + member.replace("🏢️buildings-envelope", GROUP))
        dep = '[workspace.dependencies.semio-s-artifact-norm-en1998-mut-buildings-envelope]\npath = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️buildings-envelope/📦️packages/🦀️rust"\n'
        assert dep in t
        t = t.replace(dep, dep + "\n" + dep.replace("🏢️buildings-envelope", GROUP).replace("mut-buildings-envelope", "mut-members-wall-rows"))
        wr(ct, t)
        bc = MUT + "/🧱️members/📦️packages/🦀️rust/Cargo.toml"
        t = rd(bc)
        anchor = "semio-s-artifact-norm-en1998-mut-members-wall = { workspace = true }\n"
        assert anchor in t
        wr(bc, t.replace(anchor, anchor + "semio-s-artifact-norm-en1998-mut-members-wall-rows = { workspace = true }\n"))
        bp = MUT + "/🧱️members/🦀️.rs"
        t = rd(bp)
        anchor = "pub use semio_s_artifact_norm_en1998_mut_members_wall::"
        i = t.index(anchor)
        wr(bp, t[:i] + "pub use %s::{%s};\n\n" % (CRATE, ", ".join(k.replace("-", "_") for k in mounts)) + t[i:])
    clone_lines(MUT + "/🦀️.rs")
    clone_lines(MUT + "/🟦️.ts")
    clone_lines(MUT + "/🧭️edit-rules/🦀️.rs")
    clone_lines(MUT + "/🎲️demo-cases/🦀️.rs")
    clone_lines(MUT + "/🔗️.graphql", lambda ls, i: 2 if ls[i].startswith("#") else 1, lambda l: l.startswith("# KINDS"))
    clone_lines(MUT + "/🛰️.proto", lambda ls, i: 2 if ls[i].startswith("//") else 1, lambda l: bool(re.search(r" = \d+;", l)) and "oneof" not in l and l.startswith("    "))
    p = MUT + "/🛰️.proto"
    s = rd(p)
    if CFG["snake_prefix"] + NEW[0]["snake"] + " =" not in s:
        close = s.rindex("\n  }\n}")
        top = max(int(x) for x in re.findall(r"= (\d+);", s[s.index("oneof"):]))
        wr(p, s[:close] + "\n" + "\n".join("    %s%s %s%s = %d;" % (CFG["pascal_prefix"], d["pascal"], CFG["snake_prefix"], d["snake"], top + 1 + k) for k, d in enumerate(NEW)) + s[close:])
    clone_lines(ANY + "/🚪️io/📝️text/🧬️mutations/🦀️.rs")
    clone_lines(ANY + "/🧪️tests/🫨️mutate-en1998-1/🥒️.feature")
    clone_lines(ANY + "/🧪️tests/🫨️mutate-en1998-1/🐍️.py")
    clone_lines(ANY + "/🧪️tests/🫨️mutate-en1998-1/🦀️.rs", lambda ls, i: arm(ls, i) if ls[i].rstrip().endswith("=> (") else 1, lambda l: "include_str!" in l)
    p = MUT + "/🔣️.json"
    j = json.loads(rd(p))
    refs = [x.get("$ref", "") for x in j["oneOf"]]
    k = next(i for i, r in enumerate(refs) if SRC + "/" in r)
    if not any(CFG["ent"] + "-" + NEW[0]["kebab"] + "/" in r for r in refs):
        for d in reversed(NEW):
            j["oneOf"].insert(k + 1, {"$ref": refs[k].replace(SRC, "change-" + CFG["ent"] + "-" + d["kebab"])})
        wr(p, json.dumps(j, indent=2, ensure_ascii=False) + "\n")
    p = ANY + "/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"
    s = rd(p)
    if "change-" + CFG["ent"] + "-" + NEW[0]["kebab"] not in s:
        wr(p, s.rstrip("\n") + "\n" + "".join("record change-%s-%s tag=%d\nfield payload bytes\n" % (CFG["ent"], d["kebab"], tag + k) for k, d in enumerate(NEW)))
    p = ANY + "/🔮️oracles/🔣️.json"
    o = json.loads(rd(p))
    if "change-" + CFG["ent"] + "-" + NEW[0]["kebab"] not in rd(p):
        cat, man = o["mutationCatalogs"][0], o["mutationManifests"][0]
        v = next(x for x in cat["vectors"] if x["mutationId"] == SRC)
        m = next(x for x in man["mutations"] if x["id"] == SRC)
        iv, im, ik = cat["vectors"].index(v), man["mutations"].index(m), cat["kinds"].index(SRC)
        for d in reversed(NEW):
            kind = "change-" + CFG["ent"] + "-" + d["kebab"]
            cat["vectors"].insert(iv + 1, json.loads(tr(json.dumps(v, ensure_ascii=False).replace(SRC, kind), d)))
            man["mutations"].insert(im + 1, json.loads(tr(json.dumps(m, ensure_ascii=False).replace(SRC, kind), d)))
            cat["kinds"].insert(ik + 1, kind)
        wr(p, json.dumps(o, indent=2, ensure_ascii=False) + "\n")
    p = ROOT + "/🧪️tests/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json"
    t = json.loads(rd(p))
    if "change-" + CFG["ent"] + "-" + NEW[0]["kebab"] not in rd(p):
        i = next(i for i, x in enumerate(t["rows"]) if x["kind"] == SRC and x["artifact"] == "🫨️en1998")
        for d in reversed(NEW):
            t["rows"].insert(i + 1, json.loads(tr(json.dumps(t["rows"][i], ensure_ascii=False).replace(SRC, "change-" + CFG["ent"] + "-" + d["kebab"]), d)))
        wr(p, json.dumps(t, indent=2, ensure_ascii=False) + "\n")


def shorten():
    short, long_ = "✏️change-" + CFG["ent"] + "-stiffness-ratio", "✏️change-" + CFG["ent"] + "-" + NEW[0]["kebab"]
    for p in (ANY + "/🧪️tests/🫨️mutate-en1998-1/🐍️.py", ANY + "/🧪️tests/🫨️mutate-en1998-1/🥒️.feature", ANY + "/🧪️tests/🫨️mutate-en1998-1/🦀️.rs"):
        t = rd(p)
        if long_ in t:
            wr(p, t.replace(long_, short))
    p = ANY + "/🔮️oracles/🔣️.json"
    t = rd(p)
    old = '"mutationDirectoryName": "' + long_ + '"'
    if old in t:
        wr(p, t.replace(old, '"mutationDirectoryName": "' + short + '"'))


main()
shorten()
