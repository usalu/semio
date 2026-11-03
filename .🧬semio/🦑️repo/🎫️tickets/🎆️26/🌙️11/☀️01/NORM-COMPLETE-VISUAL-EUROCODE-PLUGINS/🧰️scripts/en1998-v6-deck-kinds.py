"""Clones the id-addressed kind `change-bridge-pier-shear-rd-n` into `change-bridge-deck-length-m` and `change-bridge-deck-ei-nm2` through every registry (idempotent). Run with python -I."""
import json, os, re, shutil

ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm"
ANY = ROOT + "/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any"
MUT = ANY + "/🧬️schema/🧬️mutations"
FIX = ANY + "/🧪️tests/🧫️fixtures/🧬️mutations"
SRC = "change-bridge-pier-shear-rd-n"
OLD = dict(snake="pier_shear_rd_n", kebab="pier-shear-rd-n", camel="pierShearRdN", pascal="PierShearRdN", en="Pier shear resistance V_Rd", de="Pfeilerquerkrafttragfähigkeit V_Rd")
NEW = [
    dict(snake="deck_length_m", kebab="deck-length-m", camel="deckLengthM", pascal="DeckLengthM", en="Deck length for the lumped mass model (0 = span of the supports)", de="Überbaulänge für das Punktmassenmodell (0 = Spannweite der Auflager)"),
    dict(snake="deck_ei_nm2", kebab="deck-ei-nm2", camel="deckEiNm2", pascal="DeckEiNm2", en="Transverse bending stiffness EI of the deck in N·m² (0 = rigid deck)", de="Biegesteifigkeit EI des Überbaus quer in N·m² (0 = starrer Überbau)"),
]
TOK = ("bridge-pier-shear-rd-n", "BridgePierShearRdN", "bridge_pier_shear_rd_n")
ORDER = ["en", "de"]


def rd(p):
    return open(p, encoding="utf-8").read()


def wr(p, s):
    assert len(p[len("/home/deck/semio/"):].encode()) < 256, p
    os.makedirs(os.path.dirname(p), exist_ok=True)
    open(p, "w", encoding="utf-8").write(s)


def tr(s, n):
    for k in ("snake", "kebab", "camel", "pascal"):
        s = s.replace(OLD[k], n[k])
    s = s.replace("Change " + OLD["en"], "Change " + n["en"]).replace(OLD["de"] + " ändern", n["de"] + " ändern")
    return s.replace(OLD["en"], n["en"]).replace(OLD["de"], n["de"])


def has(l):
    return any(t in l for t in TOK)


def clone_lines(path, block=None, skip=lambda l: False):
    lines = rd(path).split("\n")
    out, i = [], 0
    probe = "change-bridge-" + NEW[0]["kebab"]
    if probe in "\n".join(lines) or ("Change" + "Bridge" + NEW[0]["pascal"]) in "\n".join(lines) or ("change_bridge_" + NEW[0]["snake"]) in "\n".join(lines):
        return False
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
    return True


def pair(lines, i):
    return 2


def ref_block(lines, i):
    return 1


def arm(lines, i):
    j = i
    while lines[j].strip() != "),":
        j += 1
    return j - i + 1


def main():
    src = MUT + "/✏️" + SRC
    for n in NEW:
        kind = "change-bridge-" + n["kebab"]
        dst = MUT + "/✏️" + kind
        if os.path.exists(dst):
            continue
        for dp, _, fs in os.walk(src):
            for f in fs:
                p = os.path.join(dp, f)
                wr(dst + p[len(src):], tr(rd(p).replace(SRC, kind), n))
        fsrc, fdst = FIX + "/✏️" + SRC, FIX + "/✏️" + kind
        for dp, _, fs in os.walk(fsrc):
            for f in fs:
                p = os.path.join(dp, f)
                s = rd(p)
                if "➡️after" in p:
                    s = s.replace('"pierShearRdN": 1.5', '"pierShearRdN": 0.0').replace('"%s": 0.0' % n["camel"], '"%s": 1.5' % n["camel"])
                elif "🔺️diff" in p:
                    s = s.replace('"pierShearRdN": 1.5', '"pierShearRdN": null').replace('"%s": null' % n["camel"], '"%s": 1.5' % n["camel"])
                elif "🦠️mutation" in p:
                    s = s.replace("newPierShearRdN", "new" + n["pascal"]).replace("changeBridgePierShearRdN", "changeBridge" + n["pascal"])
                wr(fdst + p[len(fsrc):], s)
    clone_lines(MUT + "/🦀️.rs")
    clone_lines(MUT + "/🟦️.ts")
    clone_lines(MUT + "/🧭️edit-rules/🦀️.rs")
    clone_lines(MUT + "/🎲️demo-cases/🦀️.rs")
    clone_lines(MUT + "/🗼️bridges-pier/🦀️.rs", pair)
    clone_lines(MUT + "/🔗️.graphql", lambda ls, i: 2 if ls[i].startswith("#") else 1, lambda l: l.startswith("# KINDS"))
    clone_lines(MUT + "/🛰️.proto", lambda ls, i: 2 if ls[i].startswith("//") else 1, lambda l: "= 127;" in l)
    p = MUT + "/🛰️.proto"
    s = rd(p)
    if "ChangeBridgeDeckLengthM change_bridge_deck_length_m" not in s:
        add = "".join("    Change%s%s change_bridge_%s = %d;\n" % ("Bridge", d["pascal"], d["snake"], 223 + k) for k, d in enumerate(NEW))
        a = "    RemoveAccelerogram remove_accelerogram = 222;\n"
        assert a in s
        wr(p, s.replace(a, a + add, 1))
    p = MUT + "/🔗️.graphql"
    s = rd(p)
    s = s.replace("{ index: Int! @state(class: ARTIFACT) newDeck", "{ id: String! @state(class: ARTIFACT) newDeck")
    wr(p, s)
    clone_lines(ANY + "/🚪️io/📝️text/🧬️mutations/🦀️.rs")
    clone_lines(ANY + "/🧪️tests/🫨️mutate-en1998-1/🥒️.feature")
    clone_lines(ANY + "/🧪️tests/🫨️mutate-en1998-1/🐍️.py")
    clone_lines(ANY + "/🧪️tests/🫨️mutate-en1998-1/🦀️.rs", lambda ls, i: arm(ls, i) if ls[i].rstrip().endswith("=> (") else 1, lambda l: "include_str!" in l)
    p = MUT + "/🔣️.json"
    j = json.loads(rd(p))
    refs = [x.get("$ref", "") for x in j["oneOf"]]
    k = next(i for i, r in enumerate(refs) if SRC + "/" in r)
    if not any("deck-length" in r for r in refs):
        for d in reversed(NEW):
            j["oneOf"].insert(k + 1, {"$ref": refs[k].replace(SRC, "change-bridge-" + d["kebab"])})
        wr(p, json.dumps(j, indent=2, ensure_ascii=False) + "\n")
    p = ANY + "/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"
    s = rd(p)
    if "change-bridge-deck-length-m" not in s:
        tag = max(int(x) for x in re.findall(r"tag=(\d+)", s)) + 1
        s = s.rstrip("\n") + "\n" + "".join("record change-bridge-%s tag=%d\nfield payload bytes\n" % (d["kebab"], tag + k) for k, d in enumerate(NEW))
        wr(p, s)
    p = ANY + "/🔮️oracles/🔣️.json"
    o = json.loads(rd(p))
    if "change-bridge-deck-length-m" not in rd(p):
        cat, man = o["mutationCatalogs"][0], o["mutationManifests"][0]
        v = next(x for x in cat["vectors"] if x["mutationId"] == SRC)
        m = next(x for x in man["mutations"] if x["id"] == SRC)
        iv, im, ik = cat["vectors"].index(v), man["mutations"].index(m), cat["kinds"].index(SRC)
        for d in reversed(NEW):
            kind = "change-bridge-" + d["kebab"]
            nv = json.loads(json.dumps(v).replace(SRC, kind))
            nm = json.loads(json.dumps(m).replace(SRC, kind).replace("ChangeBridgePierShearRdN", "ChangeBridge" + d["pascal"]))
            cat["vectors"].insert(iv + 1, nv)
            man["mutations"].insert(im + 1, nm)
            cat["kinds"].insert(ik + 1, kind)
        wr(p, json.dumps(o, indent=2, ensure_ascii=False) + "\n")
    p = ROOT + "/🧪️tests/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json"
    t = json.loads(rd(p))
    if "change-bridge-deck-length-m" not in rd(p):
        i = next(i for i, x in enumerate(t["rows"]) if x["kind"] == SRC and x["artifact"] == "🫨️en1998")
        row = t["rows"][i]
        for d in reversed(NEW):
            t["rows"].insert(i + 1, json.loads(json.dumps(row).replace(SRC, "change-bridge-" + d["kebab"]).replace("ChangeBridgePierShearRdN", "ChangeBridge" + d["pascal"])))
        wr(p, json.dumps(t, indent=2, ensure_ascii=False) + "\n")


main()
