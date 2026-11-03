"""v11: clones the bearing row-kind family (change-bearing-*, insert-bearing, remove-bearing) into the joint family (change-joint-*, insert-joint, remove-joint) in the new crate `mut-members-joints`, with fixtures built from the committed bearing fixtures.
Usage: /home/deck/semio/.venv/bin/python -I 2-joint-kinds.py [--dry]"""
import copy, glob, json, os, re, sys

ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm"
ART = ROOT + "/🗿️artifacts/🫨️en1998"
ANY = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any"
MUT = ANY + "/🧬️schema/🧬️mutations"
FIX = ANY + "/🧪️tests/🧫️fixtures/🧬️mutations"
GEN = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️scripts/en1998-gen-field-mutations.py"
DRY = "--dry" in sys.argv
BEFORE = "/✅apply/🔣️.before.json"
AFTER = "/✅apply/🔣️.after.json"

J1 = dict(id="joint-1", buildingId="bldg-office", storeyId="s1", columnId="col-c1", beamId="beam-b1", xM=4.0, yM=4.0, columnMomentRdNm=9000000.0, beamMomentRdNm=4000000.0)
J2 = dict(J1, id="joint-2", xM=8.0)
FIELDS = [
    ("building_id", "change-bearing-bridge-id", "bridge", "building", "building", "x"),
    ("storey_id", "change-bearing-bridge-id", "bridge", "storey", "storey", "s2"),
    ("column_id", "change-bearing-bridge-id", "bridge", "column", "member", "col-c2"),
    ("beam_id", "change-bearing-bridge-id", "bridge", "beam", "member", "beam-b2"),
    ("x_m", "change-bearing-x-m", None, None, None, 6.5),
    ("y_m", "change-bearing-x-m", None, None, None, 2.5),
    ("column_moment_rd_nm", "change-bearing-seat-length-m", None, None, None, 12000000.0),
    ("beam_moment_rd_nm", "change-bearing-seat-length-m", None, None, None, 5000000.0),
]


def rd(p):
    return open(p, encoding="utf-8").read()


def wr(p, s):
    assert len(p[len("/home/deck/semio/"):].encode()) < 256, p
    if DRY:
        return
    os.makedirs(os.path.dirname(p), exist_ok=True)
    open(p, "w", encoding="utf-8").write(s)


def words(snake):
    return snake.split("_")


def kebab(s):
    return s.replace("_", "-")


def camel(s):
    w = words(s)
    return w[0] + "".join(x[:1].upper() + x[1:] for x in w[1:])


def pascal(s):
    return "".join(x[:1].upper() + x[1:] for x in words(s))


def src_field(kind):
    return {"change-bearing-bridge-id": "bridge_id", "change-bearing-x-m": "x_m", "change-bearing-seat-length-m": "seat_length_m"}[kind]


fm = rd(ANY + "/🏷️field-meta/🦀️.rs")
LABEL = {m.group(1): (m.group(2), m.group(3)) for m in re.finditer(r'\("([^"]+)",\s*m!\("([^"]*)",\s*"([^"]*)"', fm)}

TARGETS = []
for snake, src, _, _, ref, new in FIELDS:
    sf = src_field(src)
    TARGETS.append(dict(field=snake, src=src, ref=ref, new=new, kind="change-joint-" + kebab(snake), sf=sf, labels=[(LABEL["bearings[]." + camel(sf)][0], LABEL["joints[]." + camel(snake)][0]), (LABEL["bearings[]." + camel(sf)][1], LABEL["joints[]." + camel(snake)][1])]))
TARGETS.append(dict(field=None, src="insert-bearing", kind="insert-joint", labels=[]))
TARGETS.append(dict(field=None, src="remove-bearing", kind="remove-joint", labels=[]))


def variants(kind):
    return (kind, "".join(x[:1].upper() + x[1:] for x in kind.split("-")), kind.replace("-", "_"))


def pascal_of_kind(kind):
    return "".join(x[:1].upper() + x[1:] for x in kind.split("-"))


def tr(s, t):
    for a, b in t["labels"]:
        s = s.replace(a, b)
    if t["field"]:
        sf, tf = t["sf"], t["field"]
        for a, b in ((sf, tf), (kebab(sf), kebab(tf)), (pascal(sf), pascal(tf)), (camel(sf), camel(tf))):
            s = s.replace(a, b)
    s = s.replace("mut_bridges_towers", "mut_members").replace("mut-bridges-towers", "mut-members")
    s = s.replace("bearings", "joints").replace("Bearings", "Joints").replace("bearing", "joint").replace("Bearing", "Joint").replace("Lager", "Knoten")
    return s


def src_pascal(src):
    return {"change-bearing-bridge-id": "ChangeBearingBridgeId", "change-bearing-x-m": "ChangeBearingXM", "change-bearing-seat-length-m": "ChangeBearingSeatLengthM", "insert-bearing": "InsertBearing", "remove-bearing": "RemoveBearing"}[src]


def tpascal(t):
    return tr(src_pascal(t["src"]), t)


def matches(line, src):
    return src in line or src.replace("-", "_") in line or src_pascal(src) in line


def clone_lines(path, block=None, skip=lambda l: False, transform=None):
    text = rd(path)
    if any(t["kind"] in text or t["kind"].replace("-", "_") in text for t in TARGETS):
        return
    lines, out, i = text.split("\n"), [], 0
    while i < len(lines):
        l = lines[i]
        srcs = [t for t in TARGETS if matches(l, t["src"])]
        if srcs and not skip(l):
            n = block(lines, i) if block else 1
            chunk = lines[i:i + n]
            out += chunk
            seen = {t["src"] for t in srcs}
            for t in TARGETS:
                if t["src"] in seen:
                    out += [(transform or (lambda x, _t: x))(tr(x, t), t) for x in chunk]
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


def build_fixture(t):
    base = json.loads(rd(FIX + "/✏️change-bearing-x-m" + BEFORE))
    before = copy.deepcopy(base)
    before["joints"] = [copy.deepcopy(J1)]
    empty = {"removed": [], "inserted": [], "moved": [], "modified": []}
    diff = json.loads(rd(FIX + "/✏️change-bearing-x-m/✅apply/🔣️.diff.json"))
    for k, v in diff.items():
        if isinstance(v, dict) and "modified" in v:
            diff[k] = copy.deepcopy(empty)
    kind = t["kind"]
    if t["field"]:
        c = camel(t["field"])
        after = copy.deepcopy(before)
        after["joints"][0][c] = t["new"]
        keys = ["buildingId", "storeyId", "columnId", "beamId", "xM", "yM", "columnMomentRdNm", "beamMomentRdNm"]
        diff["joints"]["modified"] = [{"id": "joint-1", "patch": {k: (t["new"] if k == c else None) for k in keys}}]
        mutation = {"mutation": "c" + tpascal(t)[1:], "id": "joint-1", "new" + pascal(t["field"]): t["new"]}
    elif kind == "insert-joint":
        after = copy.deepcopy(before)
        after["joints"].append(copy.deepcopy(J2))
        diff["joints"]["inserted"] = [{"index": 1, "row": copy.deepcopy(J2)}]
        mutation = {"mutation": "insertJoint", "index": 1, "joint": copy.deepcopy(J2)}
    else:
        after = copy.deepcopy(before)
        after["joints"] = []
        diff["joints"]["removed"] = [{"id": "joint-1", "index": 0}]
        mutation = {"mutation": "removeJoint", "id": "joint-1"}
    return before, mutation, after, diff


def dumps(o):
    return json.dumps(o, indent=2, ensure_ascii=False) + "\n"


def emoji_of(kind):
    return {"change": "✏️", "insert": "➕️", "remove": "➖️"}[kind.split("-")[0]]


def leaf_dirs(tag):
    for k, t in enumerate(TARGETS):
        kind = t["kind"]
        src = MUT + "/" + emoji_of(t["src"]) + t["src"]
        dst = MUT + "/" + emoji_of(kind) + kind
        for dp, _, fs in os.walk(src):
            for f in fs:
                p = os.path.join(dp, f)
                s = tr(rd(p), t)
                if p.endswith("/🔣️.json") and "binaryTag" in s:
                    s = re.sub(r'"binaryTag": \d+', '"binaryTag": %d' % (tag + k), s)
                if p.endswith("/🧬️schema/🔣️.json") and t.get("ref"):
                    d = json.loads(s)
                    ui = d["properties"]["new" + pascal(t["field"])]["x-semio-ui"]
                    ui["widget"] = "reference"
                    ui["ref"] = {"kind": t["ref"]}
                    s = dumps(d)
                wr(dst + p[len(src):], s)
        before, mutation, after, diff = build_fixture(t)
        fd = FIX + "/" + emoji_of(kind) + kind + "/✅apply"
        wr(fd + "/🔣️.before.json", dumps(before))
        wr(fd + "/🔣️.after.json", dumps(after))
        wr(fd + "/🔣️.mutation.json", dumps(mutation))
        wr(fd + "/🔣️.diff.json", dumps(diff))
        wr(fd + "/🔣️.outcome.json", dumps({"status": "applied"}))


def main():
    tag = next_tag(rd(ANY + "/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"))
    if os.path.exists(MUT + "/✏️change-joint-x-m"):
        print("leaves exist; only aggregates")
    else:
        leaf_dirs(tag)
    ns = {"__name__": "gen", "__file__": GEN}
    src = rd(GEN)
    exec(compile(src, GEN, "exec"), ns)
    todo = [dict(ent="joint", snake=(t["field"] or t["kind"].split("-")[0]), dirname=emoji_of(t["kind"]) + t["kind"], module=t["kind"].replace("-", "_")) for t in TARGETS]
    if not DRY:
        ns["new_crates"](todo)
    def tf(line, t):
        if "En1998Joint {" in line and t["kind"] == "insert-joint":
            line = re.sub(r"semio_s_artifact_norm_en1998_model::En1998Joint \{[^}]*\}", 'semio_s_artifact_norm_en1998_model::En1998Joint { id: String::from("joint-2"), building_id: String::from("bldg-office"), storey_id: String::from("s1"), column_id: String::from("col-c1"), beam_id: String::from("beam-b1"), x_m: 8.0, y_m: 4.0, column_moment_rd_nm: 9000000.0, beam_moment_rd_nm: 4000000.0 }', line)
        return line

    for p in (MUT + "/🦀️.rs", MUT + "/🟦️.ts", MUT + "/🧭️edit-rules/🦀️.rs", MUT + "/🎲️demo-cases/🦀️.rs", ANY + "/🚪️io/📝️text/🧬️mutations/🦀️.rs", ANY + "/🧪️tests/🫨️mutate-en1998-1/🥒️.feature", ANY + "/🧪️tests/🫨️mutate-en1998-1/🐍️.py"):
        clone_lines(p, transform=tf)
    clone_lines(ANY + "/🧪️tests/🫨️mutate-en1998-1/🦀️.rs", lambda ls, i: arm(ls, i) if ls[i].rstrip().endswith("=> (") else 1, lambda l: "include_str!" in l, transform=tf)
    p = MUT + "/🔣️.json"
    j = json.loads(rd(p))
    refs = [x.get("$ref", "") for x in j["oneOf"]]
    if not any("change-joint-x-m/" in r for r in refs):
        for t in reversed(TARGETS):
            k = max(i for i, r in enumerate(refs) if "/" + t["src"] + "/" in r)
            j["oneOf"].insert(k + 1, {"$ref": refs[k].replace(t["src"], t["kind"])})
            refs = [x.get("$ref", "") for x in j["oneOf"]]
        wr(p, dumps(j))
    p = ANY + "/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"
    s = rd(p)
    if "change-joint-x-m" not in s:
        wr(p, s.rstrip("\n") + "\n" + "".join("record %s tag=%d\nfield payload bytes\n" % (t["kind"], tag + k) for k, t in enumerate(TARGETS)))
    p = ANY + "/🔮️oracles/🔣️.json"
    o = json.loads(rd(p))
    if "change-joint-x-m" not in rd(p):
        cat, man = o["mutationCatalogs"][0], o["mutationManifests"][0]
        for t in TARGETS:
            v = next(x for x in cat["vectors"] if x["mutationId"] == t["src"])
            m = next(x for x in man["mutations"] if x["id"] == t["src"])
            kind = t["kind"]
            vs = tr(json.dumps(v, ensure_ascii=False), t)
            ms = tr(json.dumps(m, ensure_ascii=False), t)
            iv, im, ik = max(i for i, x in enumerate(cat["vectors"]) if x["mutationId"].startswith(("change-joint", "insert-joint", "remove-joint")) or x["mutationId"] == t["src"]), max(i for i, x in enumerate(man["mutations"]) if x["id"].startswith(("change-joint", "insert-joint", "remove-joint")) or x["id"] == t["src"]), cat["kinds"].index(t["src"])
            cat["vectors"].insert(iv + 1, json.loads(vs))
            man["mutations"].insert(im + 1, json.loads(ms))
            cat["kinds"].insert(ik + 1 + sum(1 for x in cat["kinds"] if x.startswith(("change-joint", "insert-joint", "remove-joint"))) * 0, kind)
        wr(p, dumps(o))
    p = ROOT + "/🧪️tests/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json"
    tj = json.loads(rd(p))
    if "change-joint-x-m" not in rd(p):
        for t in TARGETS:
            i = max(i for i, x in enumerate(tj["rows"]) if x["artifact"] == "🫨️en1998" and (x["kind"] == t["src"] or x["kind"].startswith(("change-joint", "insert-joint", "remove-joint"))))
            row = next(x for x in tj["rows"] if x["kind"] == t["src"] and x["artifact"] == "🫨️en1998")
            rs = tr(json.dumps(row, ensure_ascii=False), t)
            tj["rows"].insert(i + 1, json.loads(rs))
        wr(p, dumps(tj))


main()
