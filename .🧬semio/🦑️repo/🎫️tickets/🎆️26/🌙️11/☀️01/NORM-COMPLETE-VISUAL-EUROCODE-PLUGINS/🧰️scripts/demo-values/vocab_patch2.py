"""EN 1996 vocabulary: the exposure choices of M-60 are the EN 1996-2 classes MX1..MX5 the kernel reads (the vocabulary offered xo/xf/xs, which the kernel rejects). Edits `📚️inputs/🔣️.json` and the generated `📚️inputs/🦀️.rs` in step. Idempotent. Run with python3 -I vocab_patch2.py."""
import json, sys
sys.path.insert(0, "."); from dv_lib import *
CHOICES = [("mx1", "MX1 dry", "MX1 trocken"), ("mx2", "MX2 wet or exposed to humidity", "MX2 feucht oder Feuchtigkeit ausgesetzt"), ("mx3", "MX3 wet and freeze/thaw", "MX3 nass und Frost-Tau"), ("mx4", "MX4 saturated with salt or de-icing", "MX4 salzbeansprucht oder Tausalz"), ("mx5", "MX5 aggressive chemical environment", "MX5 chemisch aggressive Umgebung")]
vp = vocab_path("en1996"); rp = vp.parent / "🦀️.rs"
v = json.loads(vp.read_text(encoding="utf-8")); rs = rp.read_text(encoding="utf-8")
for r in v["inputs"]:
    if r["key"] == "exposure":
        r["choices"] = [{"value": a, "en": b, "de": c} for a, b, c in CHOICES]; r["example"] = "mx2"
vp.write_text(json.dumps(v, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
old = 'InputSpec::choice("exposure", "Exposure class", "Expositionsklasse", &[InputChoice::new("xo", "XO", "XO"), InputChoice::new("xf", "XF frost", "XF Frost"), InputChoice::new("xs", "XS salt", "XS Salz")]).example_text("xo")'
new = 'InputSpec::choice("exposure", "Exposure class", "Expositionsklasse", &[' + ", ".join('InputChoice::new("%s", "%s", "%s")' % c for c in CHOICES) + ']).example_text("mx2")'
if old in rs: rp.write_text(rs.replace(old, new), encoding="utf-8"); print("patched rs")
