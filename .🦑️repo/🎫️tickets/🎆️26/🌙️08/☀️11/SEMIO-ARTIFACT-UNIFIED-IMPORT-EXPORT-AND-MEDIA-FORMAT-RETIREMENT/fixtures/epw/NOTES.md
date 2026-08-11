# epw/example.epw — handcrafted EnergyPlus Weather file fixture

Generator: `../../generators/w0-fixtures/make_epw.py`.
Verifier: `../../generators/w0-fixtures/verify_epw.py`.

Line ending: CRLF (`\r\n`), matching real EPW files. Total 32 lines (8 header + 24 data), 6124 bytes.

## Header (8 lines, exact leading keywords)

1. `LOCATION,Hannover,Niedersachsen,DEU,semio-fixture,10238,52.37,9.74,1.0,55.0` — **10 fields**: city, state/region, country, data source, WMO station number, latitude, longitude, time zone, elevation (+ the `LOCATION` keyword itself makes 10 comma-separated tokens).
2. `DESIGN CONDITIONS,...` — 1997/2009-ASHRAE-HOF-style heating/cooling/extremes design-day summary, well-formed comma list.
3. `TYPICAL/EXTREME PERIODS,6,...` — 6 named periods (summer max/avg, winter min/avg, autumn avg, spring avg) with date ranges.
4. `GROUND TEMPERATURES,3,...` — 3 depths (0.5m, 2.0m, 4.0m), each with conductivity/density/specific-heat placeholders + 12 monthly temperatures.
5. `HOLIDAYS/DAYLIGHT SAVINGS,No,0,0,0` — no DST, no holidays.
6. `COMMENTS 1,...` — states this is a handcrafted fixture, not a real station record.
7. `COMMENTS 2,...` — points back at the generating ticket.
8. `DATA PERIODS,1,1,Data,Sunday, 1/ 1, 1/ 1` — 1 period, 1 record/hour, starts Sunday Jan 1, single day.

## Data records (24 hourly records, all 35 columns populated)

Column order (spec order, EnergyPlus Auxiliary Programs documentation):
`year,month,day,hour,minute,data_source_uncertainty,dry_bulb_temp,dew_point_temp,relative_humidity,atmospheric_pressure,extraterrestrial_horizontal_radiation,extraterrestrial_direct_normal_radiation,horizontal_infrared_radiation,global_horizontal_radiation,direct_normal_radiation,diffuse_horizontal_radiation,global_horizontal_illuminance,direct_normal_illuminance,diffuse_horizontal_illuminance,zenith_luminance,wind_direction,wind_speed,total_sky_cover,opaque_sky_cover,visibility,ceiling_height,present_weather_observation,present_weather_codes,precipitable_water,aerosol_optical_depth,snow_depth,days_since_last_snowfall,albedo,liquid_precip_depth,liquid_precip_quantity`

- `year=2026, month=1, day=15` fixed across all 24 records; `hour` runs `1..24` (EPW convention, hour-ending, not 0-indexed); `minute=0`.
- `data_source_uncertainty`: a real-shaped EPW field-source string (`?9?9...E0?9...`), same convention used in real TMY3-derived EPWs.
- `dry_bulb_temp`: a diurnal sine centered at −2.0 °C (plausible mid-January Hannover value), range roughly −8 °C to +4 °C across the day.
- `dew_point_temp = dry_bulb_temp − 4.5` (plausible offset).
- `relative_humidity`: 20–100 %, inversely tracking the diurnal swing.
- `atmospheric_pressure = 101100` Pa (constant, plausible sea-level-ish value).
- Solar fields (`extraterrestrial_*`, `global/direct/diffuse_horizontal_radiation`, `*_illuminance`, `zenith_luminance`): all driven by a shared `solar_shape(hour)` bell curve that is 0 before 06:00 and after 18:00, peaking at noon — so nighttime hours correctly show 0 W/m² radiation and daytime hours show plausible non-zero values (e.g. hour 12 ≈ peak).
- `wind_direction`: cycles `(200 + hour*5) % 360`; `wind_speed`: `2.5 + 1.5·sin(...)`, always positive.
- `total_sky_cover`/`opaque_sky_cover`: values in `[0,10]` (EPW oktas-like scale).
- `visibility=20.0` km, `ceiling_height=22000` m (both "unlimited"-style plausible defaults).
- `present_weather_observation=0` (observed), `present_weather_codes="999999999"` (missing/no-weather code, valid per spec).
- `precipitable_water=14` mm, `aerosol_optical_depth=0.081`, `snow_depth=0`, `days_since_last_snowfall=88`, `albedo=0.2`, `liquid_precip_depth=0`, `liquid_precip_quantity=0`.

Example record (hour 1): `2026,1,15,1,0,?9?9?9?9E0?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9,-7.8,-12.3,92,101100,0,0,280,0,0,0,0,0,0,0,205,2.9,3,2,20.0,22000,0,999999999,14,0.081,0,88,0.2,0,0`

## Verification performed

`verify_epw.py`:
1. Confirms exactly 32 lines total (8 + 24), splits on the real `\r\n` line ending used.
2. Confirms each of the 8 header lines starts with its exact expected keyword (`LOCATION`, `DESIGN CONDITIONS`, `TYPICAL/EXTREME PERIODS`, `GROUND TEMPERATURES`, `HOLIDAYS/DAYLIGHT SAVINGS`, `COMMENTS 1`, `COMMENTS 2`, `DATA PERIODS`).
3. Confirms `LOCATION` has exactly **10** comma-separated fields.
4. Confirms exactly **24** data records, each with exactly **35** comma-separated columns (not 15 like the honest-boundary energy-plugin seed — full fidelity per the plan).
5. Confirms the `hour` column runs `1,2,...,24` in order (one record per record index).
6. Range-checks `dry_bulb_temp` (−50..60 °C), `relative_humidity` (0..100 %), and non-negativity of `global_horizontal_radiation`.

→ **all assertions passed** (script output: "All 24 records have exactly 35 columns; hour sequence 1..24 confirmed; dry-bulb/RH/radiation ranges plausible.").

## Known honest limitation

This is a fabricated-but-plausible single day, not a real meteorological station record (no real EnergyPlus/TMY3 source file was available to crib from in-session) — every field is dimensionally and range-plausible and the column *order/count* is exactly per spec, which is what the stdio epw codec needs to round-trip losslessly.
