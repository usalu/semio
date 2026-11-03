"""Part 3: clones the assessment leaf `change-assessment-axial-force-n` into `change-assessment-shear-force-demand-n` (field `shear_force_demand_n`, added by `field-adder/specs/en1998-part3.json`) and mounts it into the `mut-assess-section` crate and every table of the leaf (root enum, edit rules, demo cases, text codec, binary tag, harness, feature, oracle vectors, taxonomy). Idempotent."""
import json, os, re

ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm"
ART = ROOT + "/🗿️artifacts/🫨️en1998"
ANY = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any"
MUT = ANY + "/🧬️schema/🧬️mutations"
FIX = ANY + "/🧪️tests/🧫️fixtures/🧬️mutations"
SRC = "change-assessment-axial-force-n"
OLD = dict(snake="axial_force_n", kebab="axial-force-n", camel="axialForceN", pascal="AxialForceN", en="Axial force N (compression positive)", de="Normalkraft N (Druck positiv)")
NEW = dict(snake="shear_force_demand_n", kebab="shear-force-demand-n", camel="shearForceDemandN", pascal="ShearForceDemandN", en="Shear force demand V_Ed of the brittle shear mechanism (0 = not given)", de="Querkraftbeanspruchung V_Ed des spröden Schubmechanismus (0 = nicht angegeben)")
TOK = ("assessment-axial-force-n", "AssessmentAxialForceN", "assessment_axial_force_n", "assessments[].axialForceN")
GROUP = "🧱️assess-section"
CRATE = "semio_s_artifact_norm_en1998_mut_assess_section"
AGG = "🧱️members"
KIND = "change-assessment-" + NEW["kebab"]


def rd(p):
    return open(p, encoding="utf-8").read()


def wr(p, s):
    assert len(p[len("/home/deck/semio/"):].encode()) < 256, p
    os.makedirs(os.path.dirname(p), exist_ok=True)
    open(p, "w", encoding="utf-8").write(s)


def jw(p, o):
    wr(p, json.dumps(o, indent=2, ensure_ascii=False) + "\n")


def tr(s):
    for k in ("snake", "kebab", "camel", "pascal"):
        s = s.replace(OLD[k], NEW[k])
    return s.replace("Change " + OLD["en"], "Change " + NEW["en"]).replace(OLD["de"] + " ändern", NEW["de"] + " ändern").replace(OLD["en"], NEW["en"]).replace(OLD["de"], NEW["de"])


def has(l):
    return any(t in l for t in TOK)


def clone_lines(path, block=None, skip=lambda l: False):
    text = rd(path)
    if "assessment-" + NEW["kebab"] in text or "Assessment" + NEW["pascal"] in text or "assessment_" + NEW["snake"] in text:
        return
    lines, out, i = text.split("\n"), [], 0
    while i < len(lines):
        l = lines[i]
        if has(l) and not skip(l):
            n = block(lines, i) if block else 1
            chunk = lines[i:i + n]
            out += chunk + [tr(x) for x in chunk]
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


def fixtures():
    fsrc, fdst = FIX + "/✏️" + SRC + "/✅apply", FIX + "/✏️" + KIND + "/✅apply"
    if os.path.exists(fdst):
        return
    before = json.loads(rd(fsrc + "/🔣️.before.json"))
    after = json.loads(json.dumps(before))
    next(a for a in after["assessments"] if a["id"] == "as-1")[NEW["camel"]] = 1.5
    jw(fdst + "/🔣️.before.json", before)
    jw(fdst + "/🔣️.after.json", after)
    jw(fdst + "/🔣️.mutation.json", {"mutation": "change" + "Assessment" + NEW["pascal"], "id": "as-1", "new" + NEW["pascal"]: 1.5})
    diff = json.loads(rd(fsrc + "/🔣️.diff.json"))
    for row in diff["assessments"]["modified"]:
        row["patch"][OLD["camel"]] = None
        row["patch"][NEW["camel"]] = 1.5
    jw(fdst + "/🔣️.diff.json", diff)
    wr(fdst + "/🔣️.outcome.json", rd(fsrc + "/🔣️.outcome.json"))


def main():
    src, dst = MUT + "/✏️" + SRC, MUT + "/✏️" + KIND
    if os.path.exists(dst):
        return
    tag = max(int(x) for x in re.findall(r"tag=(\d+)", rd(ANY + "/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"))) + 1
    for dp, _, fs in os.walk(src):
        for f in fs:
            p = os.path.join(dp, f)
            s = tr(rd(p).replace(SRC, KIND))
            if p.endswith("/🔣️.json") and "binaryTag" in s:
                s = re.sub(r'"binaryTag": \d+', '"binaryTag": %d' % tag, s)
            wr(dst + p[len(src):], s)
    fixtures()
    group = MUT + "/" + GROUP + "/🦀️.rs"
    mark = "//#endregion 🪢️Mounts"
    assert mark in rd(group)
    wr(group, rd(group).replace(mark, '#[path = "../✏️%s/🦀️.rs"]\npub mod %s;\n%s' % (KIND, KIND.replace("-", "_"), mark)))
    bp = MUT + "/" + AGG + "/🦀️.rs"
    old_use = "pub use %s::{" % CRATE
    t = rd(bp)
    assert old_use in t
    wr(bp, t.replace(old_use, old_use + KIND.replace("-", "_") + ", ", 1))
    clone_lines(MUT + "/🦀️.rs")
    clone_lines(MUT + "/🧭️edit-rules/🦀️.rs")
    clone_lines(MUT + "/🎲️demo-cases/🦀️.rs")
    clone_lines(ANY + "/🚪️io/📝️text/🧬️mutations/🦀️.rs")
    clone_lines(ANY + "/🧪️tests/🫨️mutate-en1998-1/🥒️.feature")
    clone_lines(ANY + "/🧪️tests/🫨️mutate-en1998-1/🐍️.py")
    clone_lines(ANY + "/🧪️tests/🫨️mutate-en1998-1/🦀️.rs", lambda ls, i: arm(ls, i) if ls[i].rstrip().endswith("=> (") else 1, lambda l: "include_str!" in l)
    p = MUT + "/🔣️.json"
    j = json.loads(rd(p))
    refs = [x.get("$ref", "") for x in j["oneOf"]]
    k = next(i for i, r in enumerate(refs) if SRC + "/" in r)
    j["oneOf"].insert(k + 1, {"$ref": refs[k].replace(SRC, KIND)})
    jw(p, j)
    p = ANY + "/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"
    wr(p, rd(p).rstrip("\n") + "\nrecord %s tag=%d\nfield payload bytes\n" % (KIND, tag))
    p = ANY + "/🔮️oracles/🔣️.json"
    o = json.loads(rd(p))
    cat, man = o["mutationCatalogs"][0], o["mutationManifests"][0]
    v = next(x for x in cat["vectors"] if x["mutationId"] == SRC)
    m = next(x for x in man["mutations"] if x["id"] == SRC)
    iv, im, ik = cat["vectors"].index(v), man["mutations"].index(m), cat["kinds"].index(SRC)
    cat["vectors"].insert(iv + 1, json.loads(tr(json.dumps(v, ensure_ascii=False).replace(SRC, KIND))))
    man["mutations"].insert(im + 1, json.loads(tr(json.dumps(m, ensure_ascii=False).replace(SRC, KIND))))
    cat["kinds"].insert(ik + 1, KIND)
    jw(p, o)
    p = ROOT + "/🧪️tests/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json"
    t = json.loads(rd(p))
    i = next(i for i, x in enumerate(t["rows"]) if x["kind"] == SRC and x["artifact"] == "🫨️en1998")
    t["rows"].insert(i + 1, json.loads(tr(json.dumps(t["rows"][i], ensure_ascii=False).replace(SRC, KIND))))
    jw(p, t)


main()
