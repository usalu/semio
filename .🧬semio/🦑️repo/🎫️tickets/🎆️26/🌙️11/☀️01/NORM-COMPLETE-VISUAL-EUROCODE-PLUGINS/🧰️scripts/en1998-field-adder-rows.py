#!/usr/bin/env python3
"""SUPERSEDED by `field-adder/field_adder.py field-adder/specs/en1998-wave2.json` (EN 1998 profile); do not run. EN 1998 FIELD-ADDER for the row entities bridge, assessment, foundation, tower and member: appends plain f64 fields through every layer, idempotently.
Layers: model (+ member literals), own and combined twins, diff (rs, ts, graphql, proto, json), SQLite (sql, rs, ts, tests, fixture), field-meta, DSL assets, committed JSON fixtures.
Usage: python3 -I en1998-field-adder-rows.py"""
import json, os, re, sys

ANY = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/"

SPECS = [
    ("tower", "ei_nm2", 0.0, "Flexural stiffness EI of the tower (0 = not given)", "Biegesteifigkeit EI des Turms (0 = nicht angegeben)"),
    ("assessment", "chord_rotation_demand_rad", 0.0, "Chord rotation demand θ_Ed", "Sehnendrehwinkel-Einwirkung θ_Ed"),
    ("assessment", "chord_rotation_yield_rad", 0.0, "Yield chord rotation θ_y", "Fließ-Sehnendrehwinkel θ_y"),
    ("assessment", "chord_rotation_ultimate_rad", 0.0, "Ultimate chord rotation θ_um", "Bruch-Sehnendrehwinkel θ_um"),
    ("foundation", "liq_n1_60", 0.0, "Corrected SPT blow count (N1)60 of the layer (0 = no liquefaction check)", "Korrigierte SPT-Schlagzahl (N1)60 der Schicht (0 = kein Verflüssigungsnachweis)"),
    ("foundation", "liq_fines_percent", 0.0, "Fines content of the layer", "Feinkornanteil der Schicht"),
    ("foundation", "liq_depth_m", 0.0, "Depth of the layer below the surface", "Tiefe der Schicht unter der Oberfläche"),
    ("foundation", "liq_water_depth_m", 0.0, "Groundwater depth below the surface", "Grundwassertiefe unter der Oberfläche"),
    ("foundation", "liq_unit_weight_n_m3", 0.0, "Total unit weight of the soil above the layer", "Wichte des Bodens über der Schicht"),
    ("foundation", "liq_magnitude", 0.0, "Surface-wave magnitude Ms", "Oberflächenwellenmagnitude Ms"),
    ("bridge", "pier_height_m", 0.0, "Pier height L_s (0 = no pier check)", "Pfeilerhöhe L_s (0 = kein Pfeilernachweis)"),
    ("bridge", "pier_moment_rd_nm", 0.0, "Pier plastic hinge moment resistance M_Rd", "Plastisches Gelenkmoment des Pfeilers M_Rd"),
    ("bridge", "pier_shear_rd_n", 0.0, "Pier shear resistance V_Rd", "Pfeilerquerkrafttragfähigkeit V_Rd"),
    ("member", "fastener_diameter_mm", 0.0, "Timber fastener diameter d (0 = not a timber connection)", "Verbindungsmitteldurchmesser d (0 = keine Holzverbindung)"),
    ("member", "timber_thickness_mm", 0.0, "Timber member thickness at the connection", "Holzdicke am Anschluss"),
    ("member", "wall_thickness_m", 0.0, "Masonry wall thickness t_ef (0 = not a masonry wall)", "Mauerwerksdicke t_ef (0 = keine Mauerwerkswand)"),
    ("member", "wall_effective_height_m", 0.0, "Masonry wall effective height h_ef", "Wirksame Mauerwerkshöhe h_ef"),
]

ENT = {
    "bridge": dict(model="En1998Bridge", table="en1998_bridge", const="BRIDGE", wvar="b", coll="bridges", meta="bridges[]", detect=("periodRatio", "bearingDRdM"), dsl="bridges", fx_key="correlatedOccupancy", fx_re=r'("bearingDRdM":[^,]*,"permanentGkN":[^,]*,"correlatedOccupancy":(?:true|false))', rs_loop="s.bridges", ts_anchor=r'fixture\.bridges\.map\(x=>\(\{\.\.\.floats\(x,\['),
    "assessment": dict(model="En1998Assessment", table="en1998_assessment", const="ASSESSMENT", wvar="a", coll="assessments", meta="assessments[]", detect=("knowledgeLevel", "gammaEl"), dsl="assessments", fx_re=r'("gammaEl":[^,}]*)', rs_loop="s.assessments", ts_anchor=r'fixture\.assessments\.map\(x=>floats\(x,\['),
    "foundation": dict(model="En1998Foundation", table="en1998_foundation", const="FOUNDATION", wvar="a", coll="foundations", meta="foundations[]", detect=("kFoundation", "kSoil"), dsl="foundations", fx_re=r'("kSoil":[^,}]*)', rs_loop="s.foundations", ts_anchor=r'fixture\.foundations\.map\(x=>floats\(x,\['),
    "tower": dict(model="En1998Tower", table="en1998_tower", const="TOWER", wvar="a", coll="towers", meta="towers[]", detect=("mRdNm", "isChimney"), dsl="towers", fx_re=r'("isChimney":[^,]*,"qNominal":[^,]*,"permanentGkN":[^,]*,"correlatedOccupancy":(?:true|false))', rs_loop="s.towers", ts_anchor=r'fixture\.towers\.map\(x=>\(\{\.\.\.floats\(x,\['),
    "member": dict(model="En1998Member", table="en1998_member", const="MEMBER", wvar="m", coll="members", meta="buildings[].members[]", detect=("detailingCompatibleWithQ", "omegaWd"), dsl=None, fx_re=r'("steelSectionClass":[^,}]*)', rs_loop="b.members", ts_anchor=r'members:b\.members\.map\(x=>floats\(x,\['),
}

def rd(p): return open(ANY + p, encoding="utf-8").read()
def wr(p, s): open(ANY + p, "w", encoding="utf-8").write(s)
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

def run(ent, name, default, en, de):
    e = ENT[ent]; cam = camel(name); model = e["model"]; var = e["wvar"]; const = e["const"]; table = e["table"]
    ms = rd("🧬️schema/📸️snapshot/🦀️.rs")
    if re.search(r"pub %s: f64" % name, ms): return
    print(ent, name)
    def m(s):
        body = re.search(r"pub struct %s \{(.*?)\n\}" % model, s, re.S)
        pos = body.end(1)
        s = s[:pos] + "\n    pub %s: f64," % name + s[pos:]
        if ent == "member":
            s = re.sub(r"(En1998Member \{ id: [^\n]*?)( \})", lambda x: x.group(1) + ", %s: %s" % (name, lit(default)) + x.group(2), s)
        return s
    edit("🧬️schema/📸️snapshot/🦀️.rs", m)
    for p in ("🧬️schema/📸️snapshot/🟦️.ts", "🧬️schema/🟦️.ts"):
        def t(s):
            s = sub1(s, r"(export interface %s \{.*?\n)(\})" % model, lambda x: x.group(1) + "  %s: number;\n" % cam + x.group(2), re.S)
            return sub1(s, r"(export const parse%s: [^\n]*?)( \}\);)" % model, lambda x: x.group(1) + ", %s: normWireRequired(normWireNumber)" % cam + x.group(2))
        edit(p, t)
    for p in ("🧬️schema/📸️snapshot/🔣️.json", "🧬️schema/🔣️.json"):
        s = rd(p); d = json.loads(s); assert json.dumps(d, indent=2, ensure_ascii=False) + "\n" == s
        c = d["$defs"][model]
        if cam in c["properties"]: continue
        c["properties"][cam] = {"type": "number", "x-semio-ui": {"label": {"en": en, "de": de}, "widget": "stepper", "precision": 3, "step": 0.001}}
        c["required"].append(cam)
        wr(p, json.dumps(d, indent=2, ensure_ascii=False) + "\n"); print("  edited", p[-40:])
    edit("🧬️schema/📸️snapshot/🔗️.graphql", lambda s: sub1(s, r"(type %s \{.*?\n)(\})" % model, lambda x: x.group(1) + "  %s: Float!\n" % cam + x.group(2), re.S))
    def pr(s):
        blk = re.search(r"message %s \{.*?\n\}" % model, s, re.S).group(0)
        n = max(int(x) for x in re.findall(r"= (\d+);", blk)) + 1
        return s.replace(blk, blk[:-1] + "  double %s = %d;\n}" % (name, n))
    edit("🧬️schema/📸️snapshot/🛰️.proto", pr)
    patch = model + "Patch"
    edit("🧬️schema/🔺️diff/🦀️.rs", lambda s: sub1(s, r"(%s of [^{]*\{ set \{[^}]*?)( \})" % patch, lambda x: x.group(1) + ", %s: f64" % name + x.group(2)))
    def dts(s):
        s = sub1(s, r"(export interface %s \{[^\n]*?; )(\})" % patch, lambda x: x.group(1) + "%s: (number) | null; " % cam + x.group(2))
        return sub1(s, r"(export const parse%s: [^\n]*?)( \}\);)" % patch, lambda x: x.group(1) + ", %s: normWireDefault(normWireNullable(normWireNumber), () => null)" % cam + x.group(2))
    edit("🧬️schema/🔺️diff/🟦️.ts", dts)
    edit("🧬️schema/🔺️diff/🔗️.graphql", lambda s: sub1(s, r"(type %s \{.*?\n)(\})" % patch, lambda x: x.group(1) + "  %s: Float\n" % cam + x.group(2), re.S))
    def dpr(s):
        blk = re.search(r"message %s \{.*?\n\}" % patch, s, re.S).group(0)
        n = max(int(x) for x in re.findall(r"= (\d+);", blk)) + 1
        return s.replace(blk, blk[:-1] + "  optional double %s = %d;\n}" % (name, n))
    edit("🧬️schema/🔺️diff/🛰️.proto", dpr)
    p = "🧬️schema/🔺️diff/🔣️.json"; s = rd(p); d = json.loads(s); assert json.dumps(d, indent=2, ensure_ascii=False) + "\n" == s
    pp = d["$defs"][patch]["properties"]
    if cam not in pp:
        pp[cam] = {"anyOf": [{"type": "number"}, {"type": "null"}]}; wr(p, json.dumps(d, indent=2, ensure_ascii=False) + "\n"); print("  edited diff json")
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
    edit("🏷️field-meta/🦀️.rs", lambda s: sub1(s, r'(    \("%s\.[A-Za-z0-9]+", [^\n]*\),\n)(?=    \("%s\.|    \("[a-zA-Z]+",)' % (re.escape(e["meta"]), re.escape(e["meta"])), lambda x: x.group(1) + '    ("%s.%s", m!("%s", "%s")),\n' % (e["meta"], cam, en, de)) if False else once_meta(s, e["meta"], cam, en, de))
    dsl(ent, e, name, default)
    fixtures(e, cam, default)

def once_meta(s, meta, cam, en, de):
    rows = list(re.finditer(r'    \("%s\.[A-Za-z0-9]+", [^\n]*\),\n' % re.escape(meta), s))
    assert rows, meta
    last = rows[-1]
    return s[:last.end()] + '    ("%s.%s", m!("%s", "%s")),\n' % (meta, cam, en, de) + s[last.end():]

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
            if walk(d): open(p, "w", encoding="utf-8").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n"); n += 1
    print("  fixtures", n)

if __name__ == "__main__":
    for spec in SPECS: run(*spec)
