"""v10: clones the member kind into one kind per wall plan field (wall_start_x_m, wall_start_y_m) and mounts them into the existing `mut-members-wall-rows` crate. Original: clones the member leaf `change-member-wall-thickness-m` into one kind per masonry wall row field (idempotent, run with python -I): crate `mut-members-wall-rows`, leaf dirs, fixtures, mutations root (rs, ts, json, proto, graphql), text codec, binary protocol, edit rules, demo cases, mutate harness, oracle manifest, taxonomy."""
import json, os, re, shutil

ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm"
ART = ROOT + "/🗿️artifacts/🫨️en1998"
ANY = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any"
MUT = ANY + "/🧬️schema/🧬️mutations"
FIX = ANY + "/🧪️tests/🧫️fixtures/🧬️mutations"
SRC = "change-member-wall-thickness-m"
OLD = dict(snake="wall_thickness_m", kebab="wall-thickness-m", camel="wallThicknessM", pascal="WallThicknessM", en="Masonry wall thickness t_ef (0 = not a masonry wall)", de="Mauerwerksdicke t_ef (0 = keine Mauerwerkswand)")
SPEC = json.load(open("/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️scripts/field-adder/specs/en1998-wall-plan.json"))


def words(s):
    return s.split("_")


NEW = []
for f in SPEC["fields"]:
    w = words(f["field"])
    NEW.append(dict(value="1.5", snake=f["field"], kebab=f["field"].replace("_", "-"), camel=w[0] + "".join(x[:1].upper() + x[1:] for x in w[1:]), pascal="".join(x[:1].upper() + x[1:] for x in w), en=f["en"], de=f["de"]))
TOK = ("member-wall-thickness-m", "MemberWallThicknessM", "member_wall_thickness_m", "members[].wallThicknessM")
GROUP = "🧱️members-wall-rows"
CRATE = "semio_s_artifact_norm_en1998_mut_members_wall_rows"


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
    if "change-member-" + NEW[0]["kebab"] in text or "ChangeMember" + NEW[0]["pascal"] in text or "change_member_" + NEW[0]["snake"] in text:
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
        kind = "change-member-" + n["kebab"]
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
                    s = re.sub(r'("%s": )[0-9.]+' % n["camel"], r"\g<1>%s" % n["value"], s.replace('"wallThicknessM": 1.5', '"wallThicknessM": 0.0', 1), count=1)
                elif "🔺️diff" in p:
                    s = re.sub(r'("%s": )null' % n["camel"], r"\g<1>%s" % n["value"], s.replace('"wallThicknessM": 1.5', '"wallThicknessM": null', 1), count=1)
                if "🦠️mutation" in p or "🎯️outcome" in p:
                    s = tr(s, n)
                    s = s.replace("1.5", n["value"])
                wr(fdst + p[len(fsrc):], s)
    group = MUT + "/" + GROUP
    mounts_text = rd(group + "/🦀️.rs")
    if mounts[0].replace("-", "_") not in mounts_text:
        end = "//#endregion 🪢️Mounts"
        wr(group + "/🦀️.rs", mounts_text.replace(end, "".join('#[path = "../✏️%s/🦀️.rs"]\npub mod %s;\n' % (k, k.replace("-", "_")) for k in mounts) + end))
        bp = MUT + "/🧱️members/🦀️.rs"
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
    clone_lines(MUT + "/🛰️.proto", lambda ls, i: 2 if ls[i].startswith("//") else 1, lambda l: " = 123;" in l and "oneof" not in l and l.startswith("    "))
    p = MUT + "/🛰️.proto"
    s = rd(p)
    if "change_member_" + NEW[0]["snake"] + " =" not in s:
        close = s.rindex("\n  }\n}")
        top = max(int(x) for x in re.findall(r"= (\d+);", s[s.index("oneof"):]))
        wr(p, s[:close] + "\n" + "\n".join("    ChangeMember%s change_member_%s = %d;" % (d["pascal"], d["snake"], top + 1 + k) for k, d in enumerate(NEW)) + s[close:])
    clone_lines(ANY + "/🚪️io/📝️text/🧬️mutations/🦀️.rs")
    clone_lines(ANY + "/🧪️tests/🫨️mutate-en1998-1/🥒️.feature")
    clone_lines(ANY + "/🧪️tests/🫨️mutate-en1998-1/🐍️.py")
    clone_lines(ANY + "/🧪️tests/🫨️mutate-en1998-1/🦀️.rs", lambda ls, i: arm(ls, i) if ls[i].rstrip().endswith("=> (") else 1, lambda l: "include_str!" in l)
    p = MUT + "/🔣️.json"
    j = json.loads(rd(p))
    refs = [x.get("$ref", "") for x in j["oneOf"]]
    k = next(i for i, r in enumerate(refs) if SRC + "/" in r)
    if not any("member-" + NEW[0]["kebab"] + "/" in r for r in refs):
        for d in reversed(NEW):
            j["oneOf"].insert(k + 1, {"$ref": refs[k].replace(SRC, "change-member-" + d["kebab"])})
        wr(p, json.dumps(j, indent=2, ensure_ascii=False) + "\n")
    p = ANY + "/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"
    s = rd(p)
    if "change-member-" + NEW[0]["kebab"] not in s:
        wr(p, s.rstrip("\n") + "\n" + "".join("record change-member-%s tag=%d\nfield payload bytes\n" % (d["kebab"], tag + k) for k, d in enumerate(NEW)))
    p = ANY + "/🔮️oracles/🔣️.json"
    o = json.loads(rd(p))
    if "change-member-" + NEW[0]["kebab"] not in rd(p):
        cat, man = o["mutationCatalogs"][0], o["mutationManifests"][0]
        v = next(x for x in cat["vectors"] if x["mutationId"] == SRC)
        m = next(x for x in man["mutations"] if x["id"] == SRC)
        iv, im, ik = cat["vectors"].index(v), man["mutations"].index(m), cat["kinds"].index(SRC)
        for d in reversed(NEW):
            kind = "change-member-" + d["kebab"]
            cat["vectors"].insert(iv + 1, json.loads(tr(json.dumps(v, ensure_ascii=False).replace(SRC, kind), d)))
            man["mutations"].insert(im + 1, json.loads(tr(json.dumps(m, ensure_ascii=False).replace(SRC, kind), d)))
            cat["kinds"].insert(ik + 1, kind)
        wr(p, json.dumps(o, indent=2, ensure_ascii=False) + "\n")
    p = ROOT + "/🧪️tests/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json"
    t = json.loads(rd(p))
    if "change-member-" + NEW[0]["kebab"] not in rd(p):
        i = next(i for i, x in enumerate(t["rows"]) if x["kind"] == SRC and x["artifact"] == "🫨️en1998")
        for d in reversed(NEW):
            t["rows"].insert(i + 1, json.loads(tr(json.dumps(t["rows"][i], ensure_ascii=False).replace(SRC, "change-member-" + d["kebab"]), d)))
        wr(p, json.dumps(t, indent=2, ensure_ascii=False) + "\n")


main()
