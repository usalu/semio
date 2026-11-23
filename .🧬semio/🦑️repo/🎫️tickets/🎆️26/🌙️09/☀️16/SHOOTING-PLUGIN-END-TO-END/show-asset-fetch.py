"""🔎 Prints the asset fetch loop inside the frame worker."""
from pathlib import Path
path = Path("/home/deck/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/📦️assets-and-host-io/🟦️.ts")
lines = path.read_text().splitlines()
for i, line in enumerate(lines[:140], 1):
    print(f"{i}:{line}")
