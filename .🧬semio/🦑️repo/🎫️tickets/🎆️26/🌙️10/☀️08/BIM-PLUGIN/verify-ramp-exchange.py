"""🛝️ Regenerate the ramp IFC4 witness using the freshly linked exchange driver."""
from pathlib import Path
import subprocess
import time

ticket = Path(__file__).resolve().parent
driver = ticket / "🗑️generated/exchange-final"
log = ticket / "🗑️generated/exchange-final.log"
minimum = time.time()
while not driver.exists() or driver.stat().st_mtime < minimum or not log.exists() or "bytes," not in log.read_text():
    time.sleep(3)
fixtures = Path("✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures")
source = fixtures / "💡️inferences/🛝️ramp-runs/🏞️ramps/📸️snapshot/🔣️.json"
target = fixtures / "🏢️ifc4/🛝️ramps/🛝️ramps.ifc"
subprocess.run([str(driver), "4", str(source), str(target)], check=True)
