## 1. Nakagin Teaching Model — `🏢️nakagin/🐍️.py` (owner NAKAGIN-MODEL) · Status: ready

Data `🏢️nakagin/🔣️.json` + `🧬️schema/🔣️.schema.json`. Every leaf is `{"value","unit","basis":"sourced|assumed|derived","source"?,"note"?}`; sources once under `sources`. Names bilingual `{"en","de"}`. Load the module with `importlib.util.spec_from_file_location` (folder name has no importable form beyond the literal `importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")`). All geometry in metres, angles in degrees, X east, Y north, Z up, world origin at the centre of Tower A at ground.

```python
load() -> dict ; sources() -> dict ; value(path) -> Value(value, unit, basis, source)   # "capsule.width", "types.A.window_diameter", "energy.u_window"
Plane(origin, x, y, z) ; Circle(center, normal, radius) ; Solid(vertices, faces, edges, window, bolts, meta)  # faces: index loops, outward winding
CapsuleType(key, name, use, width, length, height, corner_radius, corner_segments, window_diameter, window_u, window_v, rotation,
            occupants, fittings[{key,name,width,length,count}], interior_width/length/height, count)   # count derived from slots()
capsule_types() -> tuple[CapsuleType]            # A Residence, B Residence with desk, C Studio (window 1.6), D Splayed residence (window u=-0.45, rotation 5°)
capsule_type(key) ; capsule_solid(t: CapsuleType | dict of overrides + "key") -> Solid
    # local frame: x width centred, y length, z up from 0; back face (bolted) y=0, front face y=length; window Circle on front; bolts 4 points on back; meta profile/bolt_radius
Tower(key, name, index, center(x,y), storeys, capsule_storeys, first_capsule_storey, core_side, shaft_side, storey_height, base_elevation, height,
      capital_height, capital_overhang, capital_taper) ; towers() ; facade_storeys(tower) -> per capsule storey the tuple of facade indices
core_solid(tower) ; capital_solid(tower) ; podium_solid() ; podium_height()         # tower = Tower | index | key
Slot(building, tower, storey, facade, item, type, plane, facade_plane, rotation) ; .path = (b,t,s,f)
slots() -> 140 Slots in tree order ; tree() -> {(b,t,s,f): (Slot item0, Slot item1)} (70 branches)
place(solid, plane) -> Solid ; slot_solid(slot) -> world-placed capsule Solid
program(t) -> {key, net_floor_area, volume, occupants, fittings[{key,name,count,area}], fitting_area, free_area, area_per_person}
envelope(t) -> {key, surfaces[{key,name,area,u_value,exposed,factor,ua}] (roof, floor, wall_left, wall_right, front, window, back), window_area,
                exposed_area, attached_area, air_change, volume, h_t, h_v, h}
climate() -> {station, indoor_temperature, design_temperature, heating_base, months[{key,name,mean,days,hdd}], hdd, sources}
heating(t) -> {key, h_t, h_v, h, load (W), hdd, annual (kWh/a), annual_per_area (kWh/(m²·a)), net_floor_area, count} ; totals() -> {count, area, annual, annual_per_area, per_type, per_tower}
```
Facts: slot plane = facade plane (origin on the core facade, x along facade = outward normal × Z, y outward, z up; item 0 left at -1.375 m, item 1 right at +1.375 m and 0.917 m higher) yawed by the type rotation (+ for item 0, − for item 1). Facades 0..3 = N,E,S,W. Tower A (centre 0,0): facades 0,1,2 only (west, towards B and the bridges, has none) × 9 storeys; Tower B (centre -11.1,0): 4 facades × 10 storeys + 3 facades (no W) on storey 10 → 27+43 = 70 branches × 2 = 140. Storey index counts from the first capsule storey (elevation 5.5 m = 2 podium core storeys × 2.75 m). Type per slot = order[(tower + storey + 2·facade + item) mod 4]. Counts A 36, B 35, C 34, D 35.
