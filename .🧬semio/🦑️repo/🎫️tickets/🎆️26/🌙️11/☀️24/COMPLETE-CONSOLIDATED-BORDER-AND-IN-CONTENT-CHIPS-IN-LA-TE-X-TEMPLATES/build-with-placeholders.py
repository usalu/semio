"""🖼️ Builds a document whose generated image resources are missing locally: every resource the compiler
reports as unloadable gets a neutral placeholder in a ticket-local copy of the sources, then the build repeats.

usage: python build-with-placeholders.py <source dir relative to repo> <main tex> <name> [dpi]
"""
import os, re, shutil, subprocess, sys
from PIL import Image

here = os.path.dirname(os.path.abspath(__file__))
source, main, name = sys.argv[1], sys.argv[2], sys.argv[3]
dpi = sys.argv[4] if len(sys.argv) > 4 else "70"
copy = os.path.join(os.environ["SEMIO_PRINT_SCRATCH"], "sources", name)
shutil.rmtree(copy, ignore_errors=True)
shutil.copytree(os.path.join("C:/semio", source), copy)
missing = re.compile(r"Unable to load picture or PDF file '([^']+)'")
for _ in range(80):
    output = subprocess.run(["bun", os.path.join(here, "build.ts"), copy, main, name, dpi], capture_output=True, text=True, encoding="utf-8", env={**os.environ, "SEMIO_PRINT_PASSES": "1"}).stdout
    found = missing.search(output)
    if not found:
        break
    target = os.path.join(copy, found.group(1))
    os.makedirs(os.path.dirname(target), exist_ok=True)
    Image.new("RGB", (240, 160), (200, 205, 200)).save(target)
print(subprocess.run(["bun", os.path.join(here, "build.ts"), copy, main, name, dpi], capture_output=True, text=True, encoding="utf-8").stdout)
