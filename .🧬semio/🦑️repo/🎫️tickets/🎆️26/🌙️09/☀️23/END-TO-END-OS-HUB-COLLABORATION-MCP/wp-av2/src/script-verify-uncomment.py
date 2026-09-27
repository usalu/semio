# -*- coding: utf-8 -*-
"""🧰️ AV2 one-off: drops the in-body comment of the `video-render-export` verify lane (AGENTS.md: no comments inside definitions)."""
p = "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-av1-overlay/📜️script.ts"
t = open(p, encoding="utf-8").read()
old = '''      // 🧬️ `tsc --strict` over the raster video twin alone: it is self-contained by construction, so this lane's verdict
      // never inherits the renderer graph's unrelated diagnostics. FFmpeg's reading of the same streams is the repository
      // test platform's `🖌️raster/🎥️video/🧪️tests/🎞️ffmpeg-decode` case (`bun ./📜️script.ts test parity`).
'''
assert t.count(old) == 1
t = t.replace(old, "")
open(p, "w", encoding="utf-8").write(t)
print("ok")
