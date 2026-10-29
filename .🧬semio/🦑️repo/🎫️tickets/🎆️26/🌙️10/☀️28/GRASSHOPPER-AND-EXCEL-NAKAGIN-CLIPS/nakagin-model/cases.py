import glob
import json
from pathlib import Path

n = Path(glob.glob("C:/git/semio/*/*/*nakagin")[0])
c = n / "🧪️cases/🔣️.json"
d = json.loads(c.read_text(encoding="utf-8"))
d["variants"] = {
    "keys": ["window_upgrade", "back_wall_exposed", "shared_roof_floor"],
    "overrides": {"window_upgrade": {"u_window": 1.1}, "back_wall_exposed": {"attached_factor": 1.0}, "shared_roof_floor": {"u_roof": 0.0, "u_floor": 0.0}},
    "heating_A": [
        {"variant": None, "h_t": 30.921268610325118, "h": 34.04144861032512, "annual": 1295.5086014734209},
        {"variant": ["window_upgrade"], "h_t": 29.288661448070844, "h": 32.408841448070845, "annual": 1233.3767972209423},
        {"variant": ["back_wall_exposed"], "h_t": 32.79626861032512, "h": 35.91644861032512, "annual": 1366.865101473421},
        {"variant": ["shared_roof_floor"], "h_t": 19.921268610325118, "h": 23.04144861032512, "annual": 876.8838014734208},
        {"variant": ["window_upgrade", "shared_roof_floor"], "h_t": 18.288661448070844, "h": 21.408841448070845, "annual": 814.7519972209425}
    ]
}
d["climate_source"] = {"source": "jma-tokyo-normals", "period_en": "JMA normals 1991-2020 (via the Wikipedia Tokyo climate table)", "hdd": 1585.7, "unit": "K·d"}
d["invalid_models"] += [
    {"id": "negative-variant-u", "path": ["variants", 0, "overrides", "u_window", "value"], "value": -0.1},
    {"id": "unknown-override", "path": ["variants", 0, "overrides", "colour"], "add": True, "value": {"value": 1, "unit": "1", "basis": "assumed"}},
    {"id": "no-overrides", "path": ["variants", 0, "overrides"], "value": {}},
    {"id": "variant-key-case", "path": ["variants", 0, "key"], "value": "WindowUpgrade"},
    {"id": "climate-without-source", "path": ["climate", "source"], "remove": True},
    {"id": "climate-without-period-de", "path": ["climate", "period", "de"], "remove": True},
    {"id": "no-variants", "path": ["variants"], "remove": True}
]
c.write_text(json.dumps(d, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
print("ok")
