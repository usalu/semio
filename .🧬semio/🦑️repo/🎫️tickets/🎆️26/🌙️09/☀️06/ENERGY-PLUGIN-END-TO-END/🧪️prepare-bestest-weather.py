"""🌦️ Decode the official EPW into explicit native validation input without the UI store dependency."""
from pathlib import Path
import csv
import json
root = Path.cwd()
ticket = Path(__file__).resolve().parent
source = root / "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🌦️denver-tmy/🌦️.epw"
rows = list(csv.reader(source.open()))
location = rows[0]
fields = {"dry_bulb_c": 6, "dew_point_c": 7, "atmospheric_pressure_pa": 9, "wind_speed_m_s": 21, "wind_direction_deg": 20, "direct_normal_irradiance_w_m2": 14, "diffuse_horizontal_irradiance_w_m2": 15, "horizontal_infrared_w_m2": 12, "precipitation_mm": 33}
records = []
for row in rows[8:]:
    record = {key: float(row[index]) for key, index in fields.items()}
    record.update(year=int(row[0]), month=int(row[1]), day=int(row[2]), hour=int(row[3])-1, minute=int(row[4]), relative_humidity=float(row[8])/100, snow_depth_mm=float(row[30])*10)
    records.append(record)
weather = {"calendar": "TypicalYear", "location": location[1], "latitude_deg": float(location[6]), "longitude_deg": float(location[7]), "elevation_m": float(location[9]), "time_zone_hours": float(location[8]), "records": records}
(ticket / "🗑️generated/native-weather.json").write_text(json.dumps(weather))
print(f"[DEBUG] Prepared {len(records)} official native weather records")
