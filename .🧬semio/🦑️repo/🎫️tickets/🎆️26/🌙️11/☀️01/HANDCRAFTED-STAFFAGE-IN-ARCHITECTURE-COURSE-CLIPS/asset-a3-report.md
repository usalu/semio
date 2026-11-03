# Asset agent A3 report

Category plumbing: `appliances` (en "Appliances", de "Geräte", folder `🔌️appliances`, emoji 🔌️) added to the catalog manifest, schema JSON `CategoryId`, schema TS `CATEGORIES`, the schema-twin test and the React overview icon map (`plug`). The asset-validation fixture lists synthetic categories and was left alone (tests pass).

Assets (S/🗂️catalog/🔌️appliances/): `laptop`, `desktop-monitor`, `pendant-lamp`, all with manifest rows. Generator and previews tooling: `T/🧰️a3/` (build.mjs, laptop/monitor/lamp.mjs, casts.mjs, raster.py); the shapes are handcrafted per scale, the scripts only emit JSON.

Thermal: no solver change needed. Electrical heat is an `operating` region with `heatW` at level 1 (as in patio-heater and street-lamp); operating key `power` with named levels.
- laptop: 45 W (deck 17, underside 24, lid 4), idle 11 W, sleep 1.4 W. Poses open, open-idle, closed (lid 105 deg or 0). Clips open-lid, work-and-sleep.
- desktop-monitor: 25 W (panel front 9, rear housing 16), standby 0.25 W. Poses on, standby, off. Clips power-on-warm-up, standby-cool-down.
- pendant-lamp: 60 W (glass 36, shade 18, cap and collar 6). Poses on, dimmed (24 W), off. Clips switch-on-warm-up, switch-off-cool-down. Origin = lowest point of the bulb, ceiling rose 0.8 m above (place the anchor 0.8 m under the ceiling); ground "authored", support pin at the hook.

Scales: 1:50, 1:100, 1:200 authored individually; 1:500 unavailable with reasons (alternatives 200/100). Dimensions are rounded product-class estimates (14" laptop, 24" 16:9 monitor, A60/E27 bulb per IEC 60064/60061-1), power from Wilkins and Hosni 2011, Hosni et al. 1999, VDI 2078, ENERGY STAR; splits tagged authored-estimate.

Verification: catalog check `--category appliances` all pass (office-laser suitable at all authored scales); schema-twin, schema-conformance and catalog vitest files pass (393). Previews (dark-line, dark-solid, thermal, forces, moments; 1:50/100/200) rendered through the cast CLI and rasterised with PyMuPDF (no rsvg-convert installed) and inspected; the generated folder is deleted.

Known limits: the laptop is 4 mm on paper at 1:50, so in the thermal view the region colour is mostly covered by ink; use it at a larger clip scale (e.g. 1:10 to 1:20, derived use) for visible warm surfaces. The lamp, monitor and laptop have no `plan` geometry (elevation only).
