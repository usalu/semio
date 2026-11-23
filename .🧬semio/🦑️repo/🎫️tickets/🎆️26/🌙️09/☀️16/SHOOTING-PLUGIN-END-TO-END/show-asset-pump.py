"""🔎 Prints the world-asset retry section of the frame worker."""
from pathlib import Path
path = Path("/home/deck/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/📦️assets-and-host-io/🟦️.ts")
fail = Path("/home/deck/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🧩️asset-failure/🟦️.ts")
print("pump", path.exists(), "fail", fail.exists())
text = path.read_text().splitlines()
for i, line in enumerate(text, 1):
    if 140 <= i <= 220:
        print(f"{i}:{line}")
print("--- failure ---")
print(fail.read_text())
