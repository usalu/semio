"""EN 1998 field-adder, root entities (site, building): appends one plain f64 field through every layer, idempotently: the schema (snapshot, artifact and diff JSON) first, then the generator for every twin and struct, then the hand-written literals, diff macro, SQLite and field meta. Driven by `en1998_profile.py`."""
import json, os, pathlib, re, sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
import schema_twins

ANY = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/"
DRY = False


def rd(p): return open(ANY + p, encoding="utf-8").read()
def wr(p, s):
    if not DRY: open(ANY + p, "w", encoding="utf-8").write(s)
def camel(s):
    a = s.split("_"); return a[0] + "".join(x[:1].upper() + x[1:] for x in a[1:])
def kebab(s): return s.replace("_", "-")
def lit(v): return repr(float(v))
def jval(v): return json.dumps(float(v))

def sub1(s, pat, repl, flags=0):
    n = len(re.findall(pat, s, flags)); assert n == 1, (pat[:100], n)
    return re.sub(pat, repl, s, flags=flags)

def edit(p, fn):
    s = rd(p); t = fn(s)
    if t != s: wr(p, t); print("  edited", p[-60:])

ENT = dict(
    site=dict(model="En1998Site", table="en1998_site", const="SITE", var="s", ts_var="site", gql="En1998Site", proto="En1998Site", meta=None, dsl="site"),
    building=dict(model="En1998Building", table="en1998_building", const="BUILDING", var="b", ts_var="b", gql="En1998Building", proto="En1998Building", meta="buildings[]", dsl="buildings"),
)

def last_field(model_rs, model):
    body = re.search(r"pub struct %s \{(.*?)\n\}" % model, model_rs, re.S).group(1)
    return re.findall(r"    pub (\w+): ", body)[-1]

SCHEMA_SCOPE = "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998"


def run(ent, name, default, en, de):
    e = ENT[ent]; cam = camel(name); model = e["model"]
    ms = rd("🧬️schema/📸️snapshot/🦀️.rs")
    if re.search(r"Cell::Real\(\w+\.%s\)" % name, rd("🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs")): return
    after = last_field(ms, model); acam = camel(after)
    print(ent, name, "after", after)
    ui = {"label": {"en": en, "de": de}, "widget": "stepper", "precision": 3, "step": 0.001}
    for p in ("🧬️schema/📸️snapshot/🔣️.json", "🧬️schema/🔣️.json"):
        if not DRY and schema_twins.add_field(ANY + p, cam, "f64", after=acam, container=model, ui=ui):
            print("  edited", p[-40:])
    if ent == "building":
        def patch_json(data):
            properties = data["$defs"]["En1998BuildingPatch"]["properties"]
            if cam in properties: return False
            properties[cam] = {"anyOf": [{"type": "number"}, {"type": "null"}]}
        if not DRY and schema_twins.edit_schema(ANY + "🧬️schema/🔺️diff/🔣️.json", patch_json): print("  edited diff json")
    if not DRY: schema_twins.regenerate(SCHEMA_SCOPE)
    def m(s):
        if ent == "site":
            return sub1(s, r"(\n( +)importance_class: \"II\"\.into\(\),\n)", lambda x: x.group(1) + "%s%s: %s,\n" % (x.group(2), name, lit(default)))
        return sub1(s, r"(\n( +)accidental_eccentricity_ratio: [0-9.]+,\n)" if after == "accidental_eccentricity_ratio" else r"(\n( +)%s: [0-9.]+,\n)" % after, lambda x: x.group(1) + "%s%s: %s,\n" % (x.group(2), name, lit(default)))
    edit("🧬️schema/📸️snapshot/🦀️.rs", m)
    if ent == "building":
        edit("🧬️schema/🔺️diff/🦀️.rs", lambda s: sub1(s, r"(%s: f64) \} nest \{ systems: En1998SystemDelta, storeys: En1998StoreyDelta, members: En1998MemberDelta \} \}" % after, lambda x: x.group(1) + ", %s: f64 } nest { systems: En1998SystemDelta, storeys: En1998StoreyDelta, members: En1998MemberDelta } }" % name))
    sq = "🚪️io/🪶️sqlite/📸️snapshot/"
    rs = rd(sq + "🦀️.rs"); table, const, var = e["table"], e["const"], e["var"]
    ncols = int(re.search(r'entities\(database,"%s",(\d+),' % table, rs).group(1))
    idx = ncols
    def sql(s):
        line = re.search(r"^CREATE TABLE %s \(.*$" % table, s, re.M).group(0)
        first = re.search(r",[a-z0-9_]+_ieee754_bits INTEGER NOT NULL", line).start()
        new = line[:first] + ",%s REAL" % name + line[first:]
        new = new[:-2] + ",%s_ieee754_bits INTEGER NOT NULL,%s_ieee754_class TEXT NOT NULL);" % (name, name)
        return s.replace(line, new)
    edit(sq + "🗄️.sql", sql)
    def rsf(s):
        cell = "Cell::Real(%s.%s)" % (var, name)
        if ent == "site":
            s = sub1(s, r"(Cell::Real\(s\.%s\))(\],SITE\))" % after, lambda x: x.group(1) + "," + cell + x.group(2))
            s = sub1(s, r"(const SITE:&\[FloatColumn\]=&\[[^\]]*)\];", lambda x: x.group(1) + ",FloatColumn::Binary64(%d)];" % idx)
            s = sub1(s, r"(importance_class:reconstruct_text\(control,s\.text\(7\)\?\)\?)", lambda x: x.group(1) + ",%s:s.real(%d)?" % (name, idx))
        else:
            s = sub1(s, r"(Cell::Real\(b\.%s\))(\],BUILDING\))" % after, lambda x: x.group(1) + "," + cell + x.group(2))
            s = sub1(s, r"(const BUILDING:&\[FloatColumn\]=&\[[^\]]*)\];", lambda x: x.group(1) + ",FloatColumn::Binary64(%d)];" % idx)
            s = sub1(s, r"(%s:b\.real\(\d+\)\?,)" % after, lambda x: x.group(1) + "%s:b.real(%d)?," % (name, idx))
        return sub1(s, r'(entities\(database,"%s",)%d,' % (table, ncols), lambda x: x.group(1) + "%d," % (ncols + 1))
    edit(sq + "🦀️.rs", rsf)
    def tsf(s):
        if ent == "site":
            s = sub1(s, r"(site\.%s)(\],SITE,options)" % acam, lambda x: x.group(1) + ",site.%s" % cam + x.group(2))
            s = sub1(s, r"(const SITE:readonly Ieee754Column\[\]=\[[0-9,]*?)\]\.map", lambda x: x.group(1) + ",%d].map" % idx)
            s = sub1(s, r"(importanceClass:artifactSqliteText\(s,7\))", lambda x: x.group(1) + ",%s:readBinary64(s,%d,SITE)" % (cam, idx))
        else:
            s = sub1(s, r"(b\.%s)(\],BUILDING,options)" % acam, lambda x: x.group(1) + ",b.%s" % cam + x.group(2))
            s = sub1(s, r"(const BUILDING:readonly Ieee754Column\[\]=\[[0-9,]*?)\]\.map", lambda x: x.group(1) + ",%d].map" % idx)
            s = sub1(s, r"(%s:readBinary64\(b,\d+,BUILDING\),)" % acam, lambda x: x.group(1) + "%s:readBinary64(b,%d,BUILDING)," % (cam, idx))
        return sub1(s, r"(entities\(tables\[\d+\]!,)%d(,%s,options)" % (ncols, const), lambda x: x.group(1) + "%d" % (ncols + 1) + x.group(2))
    edit(sq + "🟦️.ts", tsf)
    def trs(s):
        if ent == "site": return sub1(s, r"(s\.site\.a_gr=v;)", lambda x: x.group(1) + "s.site.%s=v;" % name)
        return sub1(s, r"(b\.%s=v;)" % after, lambda x: x.group(1) + "b.%s=v;" % name)
    edit(sq + "🧪️tests/🦀️.rs", trs)
    def tts(s):
        if ent == "site": return sub1(s, r'(floats\(fixture\.site,\[[^\]]*)\]\)', lambda x: x.group(1) + ',"%s"])' % cam)
        return sub1(s, r'("%s")(\]\),systems:b\.systems)' % acam, lambda x: x.group(1) + ',"%s"' % cam + x.group(2))
    edit(sq + "🧪️tests/🟦️.ts", tts)
    p = sq + "🧪️tests/🧫️fixtures/🔣️.json"; s = rd(p)
    if '"%s"' % cam not in s:
        key = "importanceClass" if ent == "site" else acam
        s2 = re.sub(r'("%s":(?:"(?:[^"\\]|\\.)*"|[^,}\]]*))' % key, lambda x: x.group(1) + ',"%s":%s' % (cam, jval(default)), s)
        assert s2 != s; wr(p, s2); print("  edited sqlite fixture")
    if ent == "building":
        edit("🏷️field-meta/🦀️.rs", lambda s: sub1(s, r'(    \("buildings\[\]\.%s", [^\n]*\),\n)' % acam, lambda x: x.group(1) + '    ("buildings[].%s", m!("%s", "%s")),\n' % (cam, en, de)))
    else:
        edit("🏷️field-meta/🦀️.rs", lambda s: sub1(s, r'(    \("site\.aGr", [^\n]*\),\n)', lambda x: x.group(1) + '    ("site.%s", m!("%s", "%s")),\n' % (cam, en, de)))
    kd = kebab(name)
    for d in os.listdir(ANY + "🖼️assets"):
        for f in os.listdir(ANY + "🖼️assets/" + d):
            p = "🖼️assets/%s/%s/🗣️.dsl.semio" % (d, f)
            if not os.path.exists(ANY + p): continue
            s = rd(p)
            if ent == "site":
                if (" %s=" % kd) in s: continue
                s = sub1(s, r"(\nannex=\w+ site=[^\n]*? importance-class=\w+)", lambda x: x.group(1) + " %s=%s" % (kd, ("%g" % default)))
            else:
                if (" %s:NUM" % kd) in s: continue
                s = sub1(s, r"(accidental-eccentricity-ratio:NUM|%s:NUM)(\])" % kebab(after), lambda x: x.group(1) + " %s:NUM" % kd + x.group(2)) if after != "accidental_eccentricity_ratio" else sub1(s, r"(accidental-eccentricity-ratio:NUM)(\])", lambda x: x.group(1) + " %s:NUM" % kd + x.group(2))
                s = re.sub(r"(\] (?:true|false) (?:true|false) [^\n\]]*)(\n)", lambda x: x.group(1) + " %g" % default + x.group(2), s)
            wr(p, s); print("  edited", p[-70:])
    fixtures(ent, name, cam, default)

def fixtures(ent, name, cam, default):
    after_key = "importanceClass" if ent == "site" else "accidentalEccentricityRatio"
    anchor = None
    n = 0
    def walk(o, anchor_key):
        nonlocal n
        ch = False
        if isinstance(o, dict):
            marker = "importanceClass" if ent == "site" else None
            keys = list(o.keys())
            target = (ent == "site" and "importanceClass" in o and "deGroundCombo" in o) or (ent == "building" and "planWidthM" in o and "driftLimitClass" in o)
            if target and cam not in o:
                is_row = ent == "site" or "id" in o
                val = default if is_row else None
                out = {}
                last = keys[-1]
                for k in keys:
                    out[k] = o[k]
                    if k == last: out[cam] = float(val) if val is not None else None
                o.clear(); o.update(out); ch = True
            for v in list(o.values()):
                ch = walk(v, anchor_key) or ch
        elif isinstance(o, list):
            for v in o: ch = walk(v, anchor_key) or ch
        return ch
    base = ANY + "🧪️tests/🧫️fixtures/🧬️mutations"
    for dp, dn, fn in os.walk(base):
        for f in fn:
            if not f.endswith(".json"): continue
            p = os.path.join(dp, f); s = open(p, encoding="utf-8").read(); d = json.loads(s)
            if walk(d, None):
                if not DRY: open(p, "w", encoding="utf-8").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")
                n += 1
    print("  fixtures", n)

