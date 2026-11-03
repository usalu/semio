"""EN 1995 vocabulary and registry defects found by reading the kernel against the vocabulary (labels and units that named another quantity than the kernel reads) plus the input keys the evaluators read but the registry did not list. Edits `📚️inputs/{🔣️.json,🦀️.rs}`, the registry lines of the kernel, `t13`, the committed catalogue fixture and `values_en1995.py` in step. Idempotent. Run with python3 -I vocab_patch_en1995.py, then demo_gen.py en1995."""
import json, re, sys
sys.path.insert(0, "."); from dv_lib import *
vp = vocab_path("en1995"); rp = vp.parent / "🦀️.rs"
v = json.loads(vp.read_text(encoding="utf-8")); rs = rp.read_text(encoding="utf-8")
K = art("en1995") / "🧬️schema/⚖️kernel"

def row(key, scope=""):
    return next(r for r in v["inputs"] if r["key"] == key and r.get("scope", "") == scope)

def relabel(key, scope, en, de, unit, unit_id, rust_unit):
    r = row(key, scope)
    old_en = r["en"]
    r.update(en=en, de=de, unit=unit, unitId=unit_id)
    global rs
    pattern = re.compile(r'(InputSpec::number\("%s", )"%s", "[^"]*", UnitId::\w+\)' % (re.escape(key), re.escape(old_en)))
    rs, count = pattern.subn(lambda m: '%s"%s", "%s", UnitId::%s)' % (m.group(1), en, de, rust_unit), rs, count=1)
    assert count == 1 or old_en == en, (key, count)

relabel("f_c90", "", "Compressive force F_c,90,d perpendicular to the grain", "Querdruckkraft F_c,90,d", "N", "Newton", "Newton")
relabel("i_tot", "", "Radius of gyration i of the whole column", "Trägheitsradius i des gesamten Stabes", "mm", "Millimetre", "Millimetre")
relabel("load_g", "", "Permanent line load G", "Ständige Streckenlast G", "N/mm", "NewtonPerMillimetre", "NewtonPerMillimetre")
relabel("load_q", "", "Variable line load Q", "Veränderliche Streckenlast Q", "N/mm", "NewtonPerMillimetre", "NewtonPerMillimetre")
relabel("p_d", "", "Design line load p_d at the apex", "Bemessungsstreckenlast p_d am First", "N/mm", "NewtonPerMillimetre", "NewtonPerMillimetre")
r = row("i1", "T-62,T-90,T-93")
r["scope"] = "T-62,T-90"
rs = rs.replace('UnitId::MillimetreFourth).scope("T-62,T-90,T-93")', 'UnitId::MillimetreFourth).scope("T-62,T-90")')
if not any(x["key"] == "i1" and x.get("scope") == "T-93" for x in v["inputs"]):
    v["inputs"].append({"key": "i1", "scope": "T-93", "kind": "number", "en": "Radius of gyration i₁ of one part", "de": "Trägheitsradius i₁ eines Einzelquerschnitts", "unit": "mm", "unitId": "Millimetre"})
    rs = rs.replace('    InputSpec::number("i_tot"', '    InputSpec::number("i1", "Radius of gyration i₁ of one part", "Trägheitsradius i₁ eines Einzelquerschnitts", UnitId::Millimetre).scope("T-93"),\n    InputSpec::number("i_tot"', 1)
if not any(x["key"] == "n_members" for x in v["inputs"]):
    v["inputs"].append({"key": "n_members", "scope": "T-50", "kind": "number", "en": "Number of load-sharing members", "de": "Anzahl lastverteilender Bauteile", "unit": "1", "unitId": "One"})
    rs = rs.replace('    InputSpec::number("i_tot"', '    InputSpec::number("n_members", "Number of load-sharing members", "Anzahl lastverteilender Bauteile", UnitId::One).scope("T-50"),\n    InputSpec::number("i_tot"', 1)
v["inputs"] = [x for x in v["inputs"] if not (x["key"] in ("z1", "z3") and x.get("scope") == "T-62,T-90")]
rs = re.sub(r'    InputSpec::number\("z[13]", [^\n]*\.scope\("T-62,T-90"\),\n', "", rs)
v["inputs"].sort(key=lambda x: (x["key"], x.get("scope", "")))
vp.write_text(json.dumps(v, indent=2, ensure_ascii=False) + "\n", encoding="utf-8"); rp.write_text(rs, encoding="utf-8")

def sub(path, old, new):
    t = path.read_text(encoding="utf-8")
    if old in t: path.write_text(t.replace(old, new), encoding="utf-8")
sub(K / "🧱️cross-sections/🦀️.rs", 'c.n("f_c_90")?', 'c.n("f_c90")?')
sub(K / "🧱️cross-sections/🦀️.rs", 'c.n("members")?', 'c.n("n_members")?')
sub(K / "🧱️cross-sections/🦀️.rs", '"b", "h", "members", "spacing_m"', '"b", "h", "n_members", "spacing_m"')
sub(K / "🛋️serviceability/🦀️.rs", '"e1", "z1", "a2", "i2", "e2", "a3", "i3", "e3", "z3", "s1", "s3"', '"e1", "h1", "a2", "i2", "e2", "h2", "a3", "i3", "e3", "h3", "s1", "s3"')
for entry in ("T-76", "T-80", "T-81", "T-82"):
    for p in K.rglob("🦀️.rs"):
        t = p.read_text(encoding="utf-8")
        m = re.search(r'(Entry \{ id: "%s",[^\n]*?inputs: &\[)([^\]]*)(\])' % entry, t)
        if m and '"kmod"' not in m.group(2):
            p.write_text(t[: m.start(2)] + m.group(2) + ', "kmod"' + t[m.end(2):], encoding="utf-8")
fxp = art("en1995") / "🧪️tests/🧫️fixtures/📇️catalogue/🔣️.json"
fx = json.loads(fxp.read_text(encoding="utf-8"))
case = fx["cases"][13]
assert case["entry"] == "T-90"
case["case"]["num"].setdefault("m_d", 8000000.0)
fxp.write_text(json.dumps(fx, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
vals = TICKET / "🧰️scripts/demo-values/values_en1995.py"
t = vals.read_text(encoding="utf-8")
for old, new in (('"load_g": 1500.0, "load_q": 2000.0', '"load_g": 1.5, "load_q": 2.0'), ('"p_d": 15000.0', '"p_d": 15.0'), ('"z1@T-62": 60.0, "z3@T-62": 60.0', '"h1@T-62": 60.0, "h2@T-62": 160.0, "h3@T-62": 60.0, "n_members": 5.0')):
    t = t.replace(old, new)
vals.write_text(t, encoding="utf-8")
print("patched")
