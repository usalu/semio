"""usage: python sheet.py cast_dir out.png height_px name1 name2 ...  (cast ids; first frame of each)"""
import os
import subprocess
import sys

cast, out, height, *names = sys.argv[1:]
here = os.path.dirname(os.path.abspath(__file__))
raster = os.path.join(here, "..", "🧰️a10", "raster.py")
files = [os.path.join(cast, n, "0000.svg") for n in names]
subprocess.run([sys.executable, raster, height, out, *files], check=True)
