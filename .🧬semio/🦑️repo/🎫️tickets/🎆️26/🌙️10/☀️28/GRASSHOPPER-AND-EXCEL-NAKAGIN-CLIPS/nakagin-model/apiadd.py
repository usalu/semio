from pathlib import Path
here = Path(__file__).parent
api = here.parent / "api.md"
t = api.read_text(encoding="utf-8")
a = "climate() -> {station, indoor_temperature, design_temperature, heating_base, months[{key,name,mean,days,hdd}], hdd, sources}"
assert a in t
t = t.replace(a, "climate() -> {station, indoor_temperature, design_temperature, heating_base, months[{key,name,mean,days,hdd}], hdd, source (key), period{en,de}, method{en,de}, hdd_value (Value, 1585.7 K·d derived), sources}\nvariants() -> keys ; variant(name) -> {key, name{en,de}, description{en,de}, overrides{energy key: Value}}   # window_upgrade (u_window 1.1), back_wall_exposed (attached_factor 1.0), shared_roof_floor (u_roof = u_floor = 0)")
t = t.replace("envelope(t) -> {key,", "envelope(t, variant=None) -> {key, variant,", 1)
t = t.replace("heating(t) -> {key,", "heating(t, variant=None) -> {key, variant,", 1)
t = t.replace("Facts: slot plane", "Variants: `variant` is a name or a tuple of names applied in order (e.g. `('window_upgrade','shared_roof_floor')`); the data stay untouched. Type A annual demand: base 1295.5, window_upgrade 1233.4, back_wall_exposed 1366.9, shared_roof_floor 876.9 kWh/a.\nFacts: slot plane", 1)
api.write_text(t, encoding="utf-8")
r = here.parent / "nakagin-model.md"
x = r.read_text(encoding="utf-8")
x = x.replace("Ran 24 tests, OK, including zero core penetration", "Ran 29 tests, OK, including zero core penetration and variants")
x = x.replace("`node 🧪️tests/🟨️.cjs`: schema accepted; 17 invalid models rejected;", "`node 🧪️tests/🟨️.cjs`: schema accepted; 24 invalid models rejected; variants and HDD re-derived in JS;")
x += "\n## Variants and climate source (added)\n\n- `variants` in the JSON: `window_upgrade` (u_window 1.1, assumed), `back_wall_exposed` (attached_factor 1.0), `shared_roof_floor` (u_roof = u_floor = 0); bilingual name and description; overrides are Values. `variant(name)`, `variants()`, `envelope(t, variant)`, `heating(t, variant)` (name or tuple of names). Type A demand 1295.5 / 1233.4 / 1366.9 / 876.9 kWh/a; window plus shared roof/floor 814.8.\n- Climate carries `source`, `period` (JMA normals 1991-2020 via the Wikipedia Tokyo climate table), `method` and `heating_degree_days` (1585.7 K·d, derived; test equals the monthly sum). No source for the 1.1 W/(m2 K) value was searched; basis assumed.\n"
r.write_text(x, encoding="utf-8")
