# Nakagin Model Report

Owner NAKAGIN-MODEL. Folder `🎓️teaching/🏛️architecture/🏢️nakagin`: `🔣️.json`, `🧬️schema/🔣️.schema.json`, `🐍️.py`, `🧪️cases/🔣️.json`, `🧪️tests/🐍️.py`, `🧪️tests/🟨️.cjs`, `README.md` (en + de). API: `api.md` section 1, Status ready (signatures final).

## Decisions

- Capsule 2.5 × 4.0 × 2.5 m, rounded profile in the XZ plane (corner radius 0.25 m, 4 segments per corner, 20 profile points, 40 vertices, 22 faces, 60 edges), window on the front face, 4 bolts on the back face. Interior 2.3 × 3.8 × 2.1 m sourced.
- Types: A Residence, B Residence with desk, C Studio (window 1.6 m), D Splayed residence (window u -0.45 m, yaw 5 degrees, fanned outward: + for item 0, - for item 1). Pattern `order[(tower + storey + 2 facade + item) mod 4]`; counts A 36, B 35, C 34, D 35.
- Towers: A centre (0,0), B centre (-11.1,0). Capsule storey 0 at 5.5 m (2 podium core storeys of 2.75 m). Tower A: facades N,E,S, none on W (faces Tower B and the bridges), 9 storeys = 27 branches. Tower B: 10 storeys x 4 facades + top storey 10 with N,E,S (no W) = 43 branches. 70 branches x 2 = 140.
- Slot plane: origin on the core facade, X = outward normal x Z, Y outward, Z up; item 0 at -1.375 m, item 1 at +1.375 m and 0.917 m higher. Capsule yaw about the plane origin; the slot plane origin is pushed outward along the facade normal by (width/2)·sin|yaw| (derived, 0.109 m for 5 degrees), so the back face just touches the core facade and no capsule penetrates any core (test: NumPy vertex-in-box and rhino3dm BoundingBox.Contains, 140 capsules, minimum depth 0).
- Capital: 2.75 m high, overhang 0.25 m, taper 0.5 m (frustum 6.0 m to 5.0 m). Podium 31.4 x 20.7 x 5.5 m centred between the towers.
- Energy: roof 0.5, floor 0.6, walls 0.6 (assumed), window 2.33 W/(m2 K) sourced (Tokyo 2019), back wall to core factor 0.5, n 0.5 1/h, 0.34 Wh/(m3 K), theta_i 20 C, design 0 C (assumed), base 18 C.
- Climate: monthly means Tokyo 1991-2020 normals (JMA, as tabulated in Wikipedia "Tokyo", Kitanomaru Park) give 1585.7 K d at base 18 C (monthly-mean method, slightly below a daily-data HDD). The ASHRAE design temperature and a Tokyo HDD figure could not be retrieved (NOAA PDF unreadable), so the design temperature is marked assumed.
- Results: A, B, D: H 34.04 W/K, 1295.5 kWh/a, 148.2 kWh/(m2 a); C: 35.22 W/K, 1340.5 kWh/a, 153.4 kWh/(m2 a); building 182,901 kWh/a.

## Test results

- `node 🧪️tests/🟨️.cjs`: schema accepted; 24 invalid models rejected; variants and HDD re-derived in JS; 137 leaves with known sources; 70 branches, 140 capsules, types {"A":36,"B":35,"C":34,"D":35}; geometry rules hold.
- `python 🧪️tests/🐍️.py`: Ran 29 tests, OK, including zero core penetration and variants (NumPy volumes/areas/orthonormality/degree days/envelope, rhino3dm PlaneToPlane for capsule placement, independent count oracle).

Temporary scripts: `nakagin-model/` (smoke, probe, splice, api1.md).

## Variants and climate source (added)

- `variants` in the JSON: `window_upgrade` (u_window 1.1, assumed), `back_wall_exposed` (attached_factor 1.0), `shared_roof_floor` (u_roof = u_floor = 0); bilingual name and description; overrides are Values. `variant(name)`, `variants()`, `envelope(t, variant)`, `heating(t, variant)` (name or tuple of names). Type A demand 1295.5 / 1233.4 / 1366.9 / 876.9 kWh/a; window plus shared roof/floor 814.8.
- Climate carries `source`, `period` (JMA normals 1991-2020 via the Wikipedia Tokyo climate table), `method` and `heating_degree_days` (1585.7 K·d, derived; test equals the monthly sum). No source for the 1.1 W/(m2 K) value was searched; basis assumed.
