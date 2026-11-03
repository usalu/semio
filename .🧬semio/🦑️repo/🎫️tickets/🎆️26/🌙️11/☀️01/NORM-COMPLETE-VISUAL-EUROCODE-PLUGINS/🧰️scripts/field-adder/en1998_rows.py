"""EN 1998 field-adder, row entities (bridge, assessment, foundation, tower, member): appends one plain f64 field through every layer, idempotently: the schema (snapshot, artifact and diff JSON) first, then the generator for every twin and struct, then the hand-written literals, diff macro, SQLite and field meta. Driven by `en1998_profile.py`."""
import json, os, pathlib, re, sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
import schema_twins

ANY = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/"

DRY = False

ENT = {
    "bridge": dict(model="En1998Bridge", table="en1998_bridge", const="BRIDGE", wvar="b", coll="bridges", meta="bridges[]", detect=("periodRatio", "bearingDRdM"), dsl="bridges", fx_key="correlatedOccupancy", fx_re=r'("bearingDRdM":[^,]*,"permanentGkN":[^,]*,"correlatedOccupancy":(?:true|false))', rs_loop="s.bridges", ts_anchor=r'fixture\.bridges\.map\(x=>\(\{\.\.\.floats\(x,\['),
    "assessment": dict(model="En1998Assessment", table="en1998_assessment", const="ASSESSMENT", wvar="a", coll="assessments", meta="assessments[]", detect=("knowledgeLevel", "gammaEl"), dsl="assessments", fx_re=r'("gammaEl":[^,}]*)', rs_loop="s.assessments", ts_anchor=r'fixture\.assessments\.map\(x=>floats\(x,\['),
    "foundation": dict(model="En1998Foundation", table="en1998_foundation", const="FOUNDATION", wvar="a", coll="foundations", meta="foundations[]", detect=("kFoundation", "kSoil"), dsl="foundations", fx_re=r'("kSoil":[^,}]*)', rs_loop="s.foundations", ts_anchor=r'fixture\.foundations\.map\(x=>floats\(x,\['),
    "tower": dict(model="En1998Tower", table="en1998_tower", const="TOWER", wvar="a", coll="towers", meta="towers[]", detect=("mRdNm", "isChimney"), dsl="towers", fx_re=r'("isChimney":[^,]*,"qNominal":[^,]*,"permanentGkN":[^,]*,"correlatedOccupancy":(?:true|false))', rs_loop="s.towers", ts_anchor=r'fixture\.towers\.map\(x=>\(\{\.\.\.floats\(x,\['),
    "tank": dict(model="En1998Tank", table="en1998_tank", const="TANK", wvar="a", coll="tanks", meta="tanks[]", detect=("vRdN", "fillingRatio"), dsl="tanks", fx_re=r'("fillingRatio":[^,}]*,"vRdN":[^,}]*)', rs_loop="s.tanks", ts_anchor=r'fixture\.tanks\.map\(x=>floats\(x,\['),
    "retaining_wall": dict(model="En1998RetainingWall", table="en1998_retaining_wall", const="WALL", wvar="a", coll="retainingWalls", meta="retainingWalls[]", detect=("hRdNPerM", "soilGamma"), dsl="retaining-walls", fx_re=r'("hRdNPerM":[^,}]*)', rs_loop="s.retaining_walls", ts_anchor=r'fixture\.retainingWalls\.map\(x=>floats\(x,\['),
    "member": dict(model="En1998Member", table="en1998_member", const="MEMBER", wvar="m", coll="members", meta="buildings[].members[]", detect=("detailingCompatibleWithQ", "omegaWd"), dsl=None, fx_re=r'("steelSectionClass":[^,}]*)', rs_loop="b.members", ts_anchor=r'members:b\.members\.map\(x=>floats\(x,\['),
}

def rd(p): return open(ANY + p, encoding="utf-8").read()
def wr(p, s):
    if not DRY: open(ANY + p, "w", encoding="utf-8").write(s)
def camel(s):
    a = s.split("_"); return a[0] + "".join(x[:1].upper() + x[1:] for x in a[1:])
def kebab(s): return s.replace("_", "-")
def lit(v): return repr(float(v))

def sub1(s, pat, repl, flags=0):
    n = len(re.findall(pat, s, flags)); assert n == 1, (pat[:110], n)
    return re.sub(pat, repl, s, flags=flags)

def edit(p, fn):
    s = rd(p); t = fn(s)
    if t != s: wr(p, t); print("  edited", p[-58:])

def block_end(s, start):
    depth = 0
    for i in range(start, len(s)):
        if s[i] == "{": depth += 1
        elif s[i] == "}":
            depth -= 1
            if depth == 0: return i
    raise AssertionError("unbalanced")

def insert_in_literal(s, open_pat, text, before_variables):
    m = re.search(open_pat, s); assert m, open_pat
    start = m.end() - 1
    end = block_end(s, start)
    body = s[start:end]
    if before_variables and re.search(r"\bvariables:", body):
        k = start + body.index("variables:")
        return s[:k] + text + "," + s[k:]
    return s[:end] + "," + text + s[end:]

SCHEMA_SCOPE = "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998"


def run(ent, name, default, en, de, unit=None, ui_extra=None):
    e = ENT[ent]; cam = camel(name); model = e["model"]; var = e["wvar"]; const = e["const"]; table = e["table"]
    ms = rd("🧬️schema/📸️snapshot/🦀️.rs")
    if re.search(r"Cell::Real\(\w+\.%s\)" % name, rd("🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs")): return
    print(ent, name)
    patch = model + "Patch"
    ui = {"label": {"en": en, "de": de}, "widget": "stepper", "precision": 3, "step": 0.001}
    ui.update(ui_extra or {})
    for p in ("🧬️schema/📸️snapshot/🔣️.json", "🧬️schema/🔣️.json"):
        if not DRY and schema_twins.add_field(ANY + p, cam, "f64", container=model, ui=ui): print("  edited", p[-40:])

    def patch_json(data):
        properties = data["$defs"][patch]["properties"]
        if cam in properties: return False
        properties[cam] = {"anyOf": [{"type": "number"}, {"type": "null"}]}
    if not DRY and schema_twins.edit_schema(ANY + "🧬️schema/🔺️diff/🔣️.json", patch_json): print("  edited diff json")
    if not DRY: schema_twins.regenerate(SCHEMA_SCOPE)
    if ent == "member":
        edit("🧬️schema/📸️snapshot/🦀️.rs", lambda s: re.sub(r"(En1998Member \{ id: [^\n]*?)( \})", lambda x: x.group(1) + ", %s: %s" % (name, lit(default)) + x.group(2), s))
    edit("🧬️schema/🔺️diff/🦀️.rs", lambda s: sub1(s, r"(%s of [^{]*\{ set \{[^}]*?)( \})" % patch, lambda x: x.group(1) + ", %s: f64" % name + x.group(2)))
    sq = "🚪️io/🪶️sqlite/📸️snapshot/"
    ncols = int(re.search(r'entities\(database,"%s",(\d+),' % table, rd(sq + "🦀️.rs")).group(1)); idx = ncols
    def sql(s):
        line = re.search(r"^CREATE TABLE %s \(.*$" % table, s, re.M).group(0)
        first = re.search(r",[a-z0-9_]+_ieee754_bits INTEGER NOT NULL", line).start()
        new = line[:first] + ",%s REAL" % name + line[first:]
        new = new[:-2] + ",%s_ieee754_bits INTEGER NOT NULL,%s_ieee754_class TEXT NOT NULL);" % (name, name)
        return s.replace(line, new)
    edit(sq + "🗄️.sql", sql)
    def rsf(s):
        s = sub1(s, r'(insert_float\("%s",&\[[^\]]*?)(\],%s\))' % (table, const), lambda x: x.group(1) + ",Cell::Real(%s.%s)" % (var, name) + x.group(2))
        s = sub1(s, r"(const %s:&\[FloatColumn\]=&\[[^\]]*)\];" % const, lambda x: x.group(1) + ",FloatColumn::Binary64(%d)];" % idx)
        rvar = "x" if ent == "member" else ("b" if ent == "bridge" else "a")
        s = insert_in_literal(s, r"push\(%s\{" % model, "%s:%s.real(%d)?" % (name, rvar, idx), True)
        return sub1(s, r'(entities\(database,"%s",)%d,' % (table, ncols), lambda x: x.group(1) + "%d," % (ncols + 1))
    edit(sq + "🦀️.rs", rsf)
    def tsf(s):
        s = sub1(s, r'(write\(out,"%s",\[[^\]]*?)(\],%s,options)' % (table, const), lambda x: x.group(1) + ",%s.%s" % (var, cam) + x.group(2))
        s = sub1(s, r"(const %s:readonly Ieee754Column\[\]=\[[0-9,]*?)\]\.map" % const, lambda x: x.group(1) + ",%d].map" % idx)
        rvar = "x" if ent == "member" else ("b" if ent == "bridge" else "a")
        s = insert_in_literal(s, r"%s\.push\(\{" % ("member" if ent == "member" else "snapshot." + e["coll"]), "%s:readBinary64(%s,%d,%s)" % (cam, rvar, idx, const), True)
        return sub1(s, r"(entities\(tables\[\d+\]!,)%d(,%s,options)" % (ncols, const), lambda x: x.group(1) + "%d" % (ncols + 1) + x.group(2))
    edit(sq + "🟦️.ts", tsf)
    edit(sq + "🧪️tests/🦀️.rs", lambda s: sub1(s, r"(for x in &mut %s\{)" % e["rs_loop"], lambda x: x.group(1) + "x.%s=v;" % name))
    edit(sq + "🧪️tests/🟦️.ts", lambda s: sub1(s, r"(%s)" % e["ts_anchor"], lambda x: x.group(1) + '"%s",' % cam))
    p = sq + "🧪️tests/🧫️fixtures/🔣️.json"; s = rd(p)
    if '"%s"' % cam not in s:
        s2 = re.sub(e["fx_re"], lambda x: x.group(1) + ',"%s":%s' % (cam, json.dumps(float(default))), s)
        assert s2 != s; wr(p, s2); print("  edited sqlite fixture")
    edit("🏷️field-meta/🦀️.rs", lambda s: once_meta(s, e["meta"], cam, en, de, unit))
    dsl(ent, e, name, default)
    fixtures(e, cam, default)

def once_meta(s, meta, cam, en, de, unit=None):
    rows = list(re.finditer(r'    \("%s\.[A-Za-z0-9]+", [^\n]*\),\n' % re.escape(meta), s))
    assert rows, meta
    last = rows[-1]
    return s[:last.end()] + '    ("%s.%s", m!("%s", "%s"%s)),\n' % (meta, cam, en, de, (", UnitId::" + unit) if unit else "") + s[last.end():]

def dsl(ent, e, name, default):
    kd = kebab(name)
    for d in os.listdir(ANY + "🖼️assets"):
        for f in os.listdir(ANY + "🖼️assets/" + d):
            p = "🖼️assets/%s/%s/🗣️.dsl.semio" % (d, f)
            if not os.path.exists(ANY + p): continue
            s = rd(p); t = s
            if ent == "member":
                if (" %s=" % kd) in t: continue
                t = re.sub(r"(\{id=[a-z0-9-]+ material=[^\n{}]*?steel-section-class=\d+)(\})", lambda x: x.group(1) + " %s=%g" % (kd, default) + x.group(2), t)
            else:
                if (" %s:NUM" % kd) in t: continue
                m = re.search(r"^%s \[[^\n]*\] \{$" % re.escape(e["dsl"]), t, re.M)
                assert m, e["dsl"]
                header = m.group(0)
                new_header = header[: header.rindex("] {")] + " %s:NUM] {" % kd
                a = m.start() + len(new_header)
                t = t[: m.start()] + new_header + t[m.end():]
                b = t.index("\n}", a)
                rows = t[a:b].split("\n")
                rows = [r + " %g" % default if r.strip() else r for r in rows]
                t = t[:a] + "\n".join(rows) + t[b:]
            if t != s: wr(p, t); print("  edited", p[-64:])

def fixtures(e, cam, default):
    n = 0
    def walk(o):
        ch = False
        if isinstance(o, dict):
            if all(k in o for k in e["detect"]) and cam not in o:
                o[cam] = float(default) if "id" in o else None; ch = True
            for v in list(o.values()): ch = walk(v) or ch
        elif isinstance(o, list):
            for v in o: ch = walk(v) or ch
        return ch
    for dp, dn, fn in os.walk(ANY + "🧪️tests/🧫️fixtures/🧬️mutations"):
        for f in fn:
            if not f.endswith(".json"): continue
            p = os.path.join(dp, f); d = json.loads(open(p, encoding="utf-8").read())
            if walk(d):
                n += 1
                if not DRY: open(p, "w", encoding="utf-8").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")
    print("  fixtures", n)

