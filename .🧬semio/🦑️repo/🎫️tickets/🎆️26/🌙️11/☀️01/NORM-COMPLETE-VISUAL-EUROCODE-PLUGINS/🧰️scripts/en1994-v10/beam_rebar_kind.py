"""🪚 EN 1994 v10: add the mutation kind `change-beam-rebar-product` by cloning `change-beam-support` through every registration site (the stale index-based generator is not used). Idempotent."""
import copy, json, os, re, shutil

ANY = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any"
MUT = ANY + "/🧬️schema/🧬️mutations"
FIX = ANY + "/🧪️tests/🧫️fixtures/🧬️mutations"
HAR = ANY + "/🧪️tests/🧩️mutate-en1994-1"
TAX = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🧪️tests/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json"
PROTO_SEMIO = ANY + "/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"


def rd(p):
    return open(p, encoding="utf-8").read()


def wr(p, t):
    os.makedirs(os.path.dirname(p), exist_ok=True)
    open(p, "w", encoding="utf-8").write(t)


def dumps(o):
    return json.dumps(o, indent=2, ensure_ascii=False) + "\n"


KINDS = [
    dict(
        src="change-beam-support", new="change-beam-rebar-product", old_emoji="🪢️", emoji="🧵️", crate="beam-fields",
        rep=[("ChangedBeamSupport", "ChangedBeamRebarProduct"), ("change_beam_support", "change_beam_rebar_product"), ("ChangeBeamSupport", "ChangeBeamRebarProduct"),
             ("beam-support", "beam-rebar-product"), ("beams[].support", "beams[].reinforcementProduct"), ("new_support", "new_reinforcement_product"), ("newSupport", "newReinforcementProduct"),
             ("beam.support", "beam.reinforcement_product"), ("support: Some", "reinforcement_product: Some"),
             ("Change support condition of the beam", "Change reinforcement product of the beam"), ("Lagerungsart des Trägers ändern", "Bewehrungsprodukt des Trägers ändern"),
             ("Support condition of the beam", "Reinforcement product of the beam"), ("Lagerungsart des Trägers", "Bewehrungsprodukt des Trägers")],
        testkit=None,
    ),
]


def mapper(k):
    def f(text):
        text = text.replace(k["old_emoji"] + k["src"], k["emoji"] + k["new"])
        text = text.replace(k["src"], k["new"])
        for a, b in k["rep"]:
            text = text.replace(a, b)
        return text
    return f


def dup_lines(text, k, pattern, f):
    done = False

    def go(m):
        nonlocal done
        block = m.group(0)
        new = f(block)
        if new in text:
            done = True
            return block
        return block + new
    out = re.sub(pattern, go, text, flags=re.M)
    return out


def clone_dir(k, f):
    src = MUT + "/" + k["old_emoji"] + k["src"]
    dst = MUT + "/" + k["emoji"] + k["new"]
    if os.path.exists(dst):
        return
    for root, _, files in os.walk(src):
        for name in files:
            sp = os.path.join(root, name)
            dp = os.path.join(dst, os.path.relpath(sp, src))
            wr(dp, f(rd(sp)))


def clone_fixture(k, f, tag):
    src = FIX + "/" + k["old_emoji"] + k["src"] + "/✅apply"
    dst = FIX + "/" + k["emoji"] + k["new"] + "/✅apply"
    if os.path.exists(dst):
        return
    for root, _, files in os.walk(src):
        for name in files:
            sp = os.path.join(root, name)
            wr(os.path.join(dst, os.path.relpath(sp, src)), f(rd(sp)))
    p = lambda *x: dst + "/" + "/".join(x)
    before = json.loads(rd(src + "/🔣️.before.json"))
    after = copy.deepcopy(before)
    next(b for b in after["beams"] if b["id"] == "beam-B1")["reinforcementProduct"] = "cold_worked"
    wr(p("📸️snapshot", "⬅️before", "🔣️.json"), dumps(before))
    wr(p("📸️snapshot", "➡️after", "🔣️.json"), dumps(after))
    wr(p("🦠️mutation", "🔣️.json"), dumps({"ChangeBeamRebarProduct": {"id": "beam-B1", "newReinforcementProduct": "cold_worked"}}))
    diff = json.loads(rd(p("🔺️diff", "🔣️.json")))
    patch = diff["beams"]["modified"][0]["patch"]
    patch["support"] = None
    patch["reinforcementProduct"] = "cold_worked"
    wr(p("🔺️diff", "🔣️.json"), dumps(diff))


def json_clone(node, k, f):
    src = k["src"]
    if isinstance(node, list):
        out = []
        for item in node:
            out.append(json_clone(item, k, f))
            if (isinstance(item, (str, dict))) and src in json.dumps(item, ensure_ascii=False) and len(json.dumps(item, ensure_ascii=False)) < 3000:
                clone = json.loads(f(json.dumps(item, ensure_ascii=False)))
                if clone not in node:
                    out.append(clone)
        return out
    if isinstance(node, dict):
        return {key: json_clone(v, k, f) for key, v in node.items()}
    return node


def main():
    tags = [int(x) for x in re.findall(r"tag=(\d+)", rd(PROTO_SEMIO))]
    proto_p = MUT + "/🛰️.proto"
    fields = [int(x) for x in re.findall(r"= (\d+);", rd(proto_p))]
    tag, number = max(tags) + 1, max(fields) + 1
    for k in KINDS:
        f = mapper(k)
        record = "".join(w.capitalize() for w in k["new"].split("-"))
        record = f("".join(w[:1].upper() + w[1:] for w in k["src"].split("-")))
        module = k["new"].replace("-", "_")
        if (MUT + "/" + k["emoji"] + k["new"]) in [x for x in [MUT + "/" + k["emoji"] + k["new"]] if os.path.exists(x)]:
            print("exists", k["new"])
            continue
        clone_dir(k, f)
        dj = MUT + "/" + k["emoji"] + k["new"] + "/🔣️.json"
        meta = json.loads(rd(dj))
        meta["binaryTag"] = tag
        wr(dj, dumps(meta))
        sj = MUT + "/" + k["emoji"] + k["new"] + "/🧬️schema/🔣️.json"
        schema = json.loads(rd(sj))
        value = [v for key, v in schema["properties"].items() if key != "id"][0]
        value["x-semio-ui"]["description"] = {"en": "hot_rolled or cold_worked hogging reinforcement (EN 1992-1-2 Table 3.2a).", "de": "hot_rolled (warmgewalzt) oder cold_worked (kaltverformt) nach EN 1992-1-2 Tabelle 3.2a."}
        wr(sj, dumps(schema))
        clone_fixture(k, f, tag)
        src_rec = "".join(w[:1].upper() + w[1:] for w in k["src"].split("-"))
        sm = k["src"].replace("-", "_")

        def edit(path, pattern, flags=re.M):
            t = rd(path)
            n = dup_lines(t, k, pattern, f)
            if n != t:
                wr(path, n)

        for pth in (HAR + "/🐍️.py", HAR + "/🥒️.feature"):
            edit(pth, r'^.*%s.*\n' % re.escape(k["src"]))
        edit(HAR + "/🦀️.rs", r'^    "%s",\n' % re.escape(k["src"]))
        t = rd(HAR + "/🦀️.rs")
        block = re.search(r'        "%s" => \(\n(?:            include_str!.*\n)+        \),\n' % re.escape(k["src"]), t)
        if block and f(block.group(0)) not in t:
            wr(HAR + "/🦀️.rs", t.replace(block.group(0), block.group(0) + f(block.group(0)), 1))
        edit(MUT + "/🟦️.ts", r'^.*%s.*\n' % re.escape(src_rec))
        edit(MUT + "/🦀️.rs", r'^.*(?:%s|%s).*\n' % (re.escape(sm), re.escape(src_rec)))
        edit(MUT + "/🦀️.rs", r'^    "%s",\n' % re.escape(k["src"]))
        edit(MUT + "/%s/🦀️.rs" % {"beam-fields": "🌉️beam-fields"}[k["crate"]], r'^#\[path = "\.\./%s%s/🦀️\.rs"\]\npub mod %s;\n' % (re.escape(k["old_emoji"]), re.escape(k["src"]), sm))
        edit(ANY + "/🚪️io/📝️text/🧬️mutations/🦀️.rs", r'^.*%s;\n' % re.escape(sm))
        edit(MUT + "/🧭️edit-rules/🦀️.rs", r'^.*"%s".*\n' % re.escape(k["src"]))
        t = rd(MUT + "/🧰️testkit/🦀️.rs")
        line = re.search(r'^.*%s.*\n' % re.escape(src_rec), t, re.M).group(0)
        nl = f(line)
        nl = nl.replace('new_reinforcement_product: "continuous_2_span".into()', 'new_reinforcement_product: "cold_worked".into()')
        if nl not in t:
            wr(MUT + "/🧰️testkit/🦀️.rs", t.replace(line, line + nl, 1))
        g = MUT + "/🔗️.graphql"
        t = rd(g)
        if (" | " + f(src_rec)) not in t:
            wr(g, t.replace(" | " + src_rec, " | " + src_rec + " | " + f(src_rec), 1))
        t = rd(proto_p)
        line = re.search(r'^.*\b%s\b.*\n' % re.escape(src_rec), t, re.M).group(0)
        nl = re.sub(r"= \d+;", "= %d;" % number, f(line))
        if nl not in t:
            wr(proto_p, t.replace(line, line + nl, 1))
        t = rd(PROTO_SEMIO)
        add = "record %s tag=%d\nfield payload bytes\n" % (k["new"], tag)
        if add not in t:
            wr(PROTO_SEMIO, t.rstrip("\n") + "\n" + add)
        for pth in (MUT + "/🔣️.json", ANY + "/🔮️oracles/🔣️.json"):
            d = json.loads(rd(pth))
            wr(pth, dumps(json_clone(d, k, f)))
        d = json.loads(rd(TAX))
        d = json_clone(d, k, f)
        idx = [i for i, r in enumerate(d["rows"]) if r["artifact"] == "🧩️en1994"]
        d["rows"][idx[0]: idx[-1] + 1] = sorted(d["rows"][idx[0]: idx[-1] + 1], key=lambda r: r["module"])
        wr(TAX, dumps(d))
        tag += 1
        number += 1
        print("cloned", k["new"])


main()
