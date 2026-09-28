# -*- coding: utf-8 -*-
"""🧬️ AV2: writes a tsc program over every TS file the video-render slice touches. Usage: python3 make-tsconfig.py [<root> <out>]
(default: the overlay → tsconfig-av2.json; live: /Users/ueli/Documents/semio tsconfig-live.json)."""
import json, sys
o = sys.argv[1] if len(sys.argv) > 2 else "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-av1-overlay"
out = sys.argv[2] if len(sys.argv) > 2 else "tsconfig-av2.json"
files = ["🧰️framework/🔨️modules/🎠️kernel/🟦️.ts", "🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🎞️video-render-program/🟦️.ts", "🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🧵️video-render-job/🟦️.ts", "🧰️framework/🔨️modules/🖌️raster/🎥️video/🟦️.ts", "🧰️framework/🔨️modules/🖌️raster/🎥️video/🧪️tests/🔬️unit/🟦️.ts", "🧰️framework/🔨️modules/🖌️raster/🎥️video/🧪️tests/🎞️ffmpeg-decode/🟦️.ts", "🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts", "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎥️VideoRenderHost/🟦️.ts", "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎥️VideoRenderHost/🧪️tests/🔬️unit/🟦️.ts", "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🧵️TaskManager/🟦️.tsx", "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx", "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx"]
cfg = {"extends": o + "/🧰️framework/🛍️products/💻️os/tsconfig.json", "compilerOptions": {"noEmit": True, "incremental": False}, "include": [], "files": [o + "/" + f for f in files]}
json.dump(cfg, open(out, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
