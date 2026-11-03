"""Clones the site leaf `change-site-v-s30-m-s` into one kind per ground-classification site field (idempotent, run with python -I): crate `mut-site-ground`, leaf dirs, fixtures, mutations root (rs, ts, json, proto, graphql), text codec, binary protocol, edit rules, demo cases, mutate harness, oracle manifest, taxonomy."""
import json, os, re, shutil

ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm"
ART = ROOT + "/🗿️artifacts/🫨️en1998"
ANY = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any"
MUT = ANY + "/🧬️schema/🧬️mutations"
FIX = ANY + "/🧪️tests/🧫️fixtures/🧬️mutations"
SRC = "change-site-v-s30-m-s"
OLD = dict(snake="v_s30_m_s", kebab="v-s30-m-s", camel="vS30MS", pascal="VS30MS", en="Average shear wave velocity v_s,30 (0 = not given)", de="Mittlere Scherwellengeschwindigkeit v_s,30 (0 = nicht angegeben)")
SPEC = json.load(open("/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️scripts/field-adder/specs/en1998-site-ground.json"))


def words(s):
    return s.split("_")


NEW = []
for f in SPEC["fields"]:
    w = words(f["field"])
    NEW.append(dict(snake=f["field"], kebab=f["field"].replace("_", "-"), camel=w[0] + "".join(x[:1].upper() + x[1:] for x in w[1:]), pascal="".join(x[:1].upper() + x[1:] for x in w), en=f["en"], de=f["de"]))
TOK = ("site-v-s30-m-s", "SiteVS30MS", "site_v_s30_m_s", "site.vS30MS")
GROUP = "🌍️site-ground"
CRATE = "semio_s_artifact_norm_en1998_mut_site_ground"


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
    if "change-site-" + NEW[0]["kebab"] in text or "ChangeSite" + NEW[0]["pascal"] in text or "change_site_" + NEW[0]["snake"] in text:
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
        kind = "change-site-" + n["kebab"]
        dst = MUT + "/✏️" + kind
        mounts.append(kind)
        if not os.path.exists(dst):
            for dp, _, fs in os.walk(src):
                for f in fs:
                    p = os.path.join(dp, f)
                    s = tr(rd(p).replace(SRC, kind), n)
                    if p.endswith("/🔣️.json") and "binaryTag" in s:
                        s = re.sub(r'"binaryTag": \d+', '"binaryTag": %d' % (tag + k), s)
                    wr(dst + p[len(src):], s)
        fsrc, fdst = FIX + "/✏️" + SRC, FIX + "/✏️" + kind
        if os.path.exists(fdst):
            continue
        for dp, _, fs in os.walk(fsrc):
            for f in fs:
                p = os.path.join(dp, f)
                s = rd(p)
                if "➡️after" in p:
                    s = re.sub(r'("%s": )[0-9.]+' % n["camel"], r"\g<1>1.5", s.replace('"vS30MS": 1.5', '"vS30MS": 0.0'), count=1)
                elif "🔺️diff" in p:
                    s = re.sub(r'("%s": )[0-9.]+' % n["camel"], r"\g<1>1.5", s.replace('"vS30MS": 1.5', '"vS30MS": 0.0'), count=1)
                if "🦠️mutation" in p or "🎯️outcome" in p:
                    s = tr(s, n)
                wr(fdst + p[len(fsrc):], s)
    group = MUT + "/" + GROUP
    if not os.path.exists(group):
        head = rd(MUT + "/🏢️buildings-envelope/🦀️.rs").split("//#region")[0].replace("buildings envelope mutation leaves: v s30 m s, setback ratio, plan envelope gap ratio.", "site ground classification mutation leaves: N_SPT, c_u, soft clay, liquefiable and alluvium indicators.")
        body = "//#region 🪢️Mounts\n" + "".join('#[path = "../✏️%s/🦀️.rs"]\npub mod %s;\n' % (k, k.replace("-", "_")) for k in mounts) + "//#endregion 🪢️Mounts\n"
        wr(group + "/🦀️.rs", head.replace("🏢️ EN 1998", "🌍️ EN 1998") + body)
        c = rd(MUT + "/🏢️buildings-envelope/📦️packages/🦀️rust/Cargo.toml").replace("mut-buildings-envelope", "mut-site-ground").replace("buildings envelope mutation leaves", "site ground classification mutation leaves")
        wr(group + "/📦️packages/🦀️rust/Cargo.toml", c)
        ct = ART + "/Cargo.toml"
        s = rd(ct)
        member = '    "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️buildings-envelope/📦️packages/🦀️rust",\n'
        assert member in s
        s = s.replace(member, member + member.replace("🏢️buildings-envelope", GROUP))
        dep = '[workspace.dependencies.semio-s-artifact-norm-en1998-mut-buildings-envelope]\npath = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️buildings-envelope/📦️packages/🦀️rust"\n'
        assert dep in s
        s = s.replace(dep, dep + "\n" + dep.replace("buildings-envelope", "site-ground").replace("🏢️buildings-envelope", GROUP))
        wr(ct, s)
        bc = MUT + "/🏢️buildings/📦️packages/🦀️rust/Cargo.toml"
        s = rd(bc)
        a = "semio-s-artifact-norm-en1998-mut-buildings-envelope = { workspace = true }\n"
        wr(bc, s.replace(a, a + "semio-s-artifact-norm-en1998-mut-site-ground = { workspace = true }\n"))
        bp = MUT + "/🏢️buildings/🦀️.rs"
        s = rd(bp)
        a = "pub use semio_s_artifact_norm_en1998_mut_buildings_envelope::"
        i = s.index(a)
        s = s[:i] + "pub use %s::{%s};\n\n" % (CRATE, ", ".join(k.replace("-", "_") for k in mounts)) + s[i:]
        wr(bp, s)
    clone_lines(MUT + "/🦀️.rs")
    clone_lines(MUT + "/🟦️.ts")
    clone_lines(MUT + "/🧭️edit-rules/🦀️.rs")
    clone_lines(MUT + "/🎲️demo-cases/🦀️.rs")
    clone_lines(MUT + "/🔗️.graphql", lambda ls, i: 2 if ls[i].startswith("#") else 1, lambda l: l.startswith("# KINDS"))
    clone_lines(MUT + "/🛰️.proto", lambda ls, i: 2 if ls[i].startswith("//") else 1, lambda l: "= 110;" in l)
    p = MUT + "/🛰️.proto"
    s = rd(p)
    if "change_site_" + NEW[0]["snake"] + " =" not in s:
        top = max(int(x) for x in re.findall(r"= (\d+);", s[s.index("oneof"):])) if "oneof" in s else 230
        a = "    RemoveSystem remove_system = %d;\n" % top
        assert a in s, a
        wr(p, s.replace(a, a + "".join("    ChangeSite%s change_site_%s = %d;\n" % (d["pascal"], d["snake"], top + 1 + k) for k, d in enumerate(NEW)), 1))
    clone_lines(ANY + "/🚪️io/📝️text/🧬️mutations/🦀️.rs")
    clone_lines(ANY + "/🧪️tests/🫨️mutate-en1998-1/🥒️.feature")
    clone_lines(ANY + "/🧪️tests/🫨️mutate-en1998-1/🐍️.py")
    clone_lines(ANY + "/🧪️tests/🫨️mutate-en1998-1/🦀️.rs", lambda ls, i: arm(ls, i) if ls[i].rstrip().endswith("=> (") else 1, lambda l: "include_str!" in l)
    p = MUT + "/🔣️.json"
    j = json.loads(rd(p))
    refs = [x.get("$ref", "") for x in j["oneOf"]]
    k = next(i for i, r in enumerate(refs) if SRC + "/" in r)
    if not any("site-" + NEW[0]["kebab"] + "/" in r for r in refs):
        for d in reversed(NEW):
            j["oneOf"].insert(k + 1, {"$ref": refs[k].replace(SRC, "change-site-" + d["kebab"])})
        wr(p, json.dumps(j, indent=2, ensure_ascii=False) + "\n")
    p = ANY + "/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"
    s = rd(p)
    if "change-site-" + NEW[0]["kebab"] not in s:
        wr(p, s.rstrip("\n") + "\n" + "".join("record change-site-%s tag=%d\nfield payload bytes\n" % (d["kebab"], tag + k) for k, d in enumerate(NEW)))
    p = ANY + "/🔮️oracles/🔣️.json"
    o = json.loads(rd(p))
    if "change-site-" + NEW[0]["kebab"] not in rd(p):
        cat, man = o["mutationCatalogs"][0], o["mutationManifests"][0]
        v = next(x for x in cat["vectors"] if x["mutationId"] == SRC)
        m = next(x for x in man["mutations"] if x["id"] == SRC)
        iv, im, ik = cat["vectors"].index(v), man["mutations"].index(m), cat["kinds"].index(SRC)
        for d in reversed(NEW):
            kind = "change-site-" + d["kebab"]
            cat["vectors"].insert(iv + 1, json.loads(tr(json.dumps(v, ensure_ascii=False).replace(SRC, kind), d)))
            man["mutations"].insert(im + 1, json.loads(tr(json.dumps(m, ensure_ascii=False).replace(SRC, kind), d)))
            cat["kinds"].insert(ik + 1, kind)
        wr(p, json.dumps(o, indent=2, ensure_ascii=False) + "\n")
    p = ROOT + "/🧪️tests/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json"
    t = json.loads(rd(p))
    if "change-site-" + NEW[0]["kebab"] not in rd(p):
        i = next(i for i, x in enumerate(t["rows"]) if x["kind"] == SRC and x["artifact"] == "🫨️en1998")
        for d in reversed(NEW):
            t["rows"].insert(i + 1, json.loads(tr(json.dumps(t["rows"][i], ensure_ascii=False).replace(SRC, "change-site-" + d["kebab"]), d)))
        wr(p, json.dumps(t, indent=2, ensure_ascii=False) + "\n")


main()
