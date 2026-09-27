# -*- coding: utf-8 -*-
"""🧰️ AV2 one-off: imports the kernel's `VideoRenderJobRow` into the overlay ShellHost through `@semio-tech/framework`."""
p = "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-av1-overlay/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx"
t = open(p, encoding="utf-8").read()
old = "  type UtilityNode,\n  waitForEvent,\n"
assert t.count(old) == 1
t = t.replace(old, "  type UtilityNode,\n  type VideoRenderJobRow,\n  waitForEvent,\n")
open(p, "w", encoding="utf-8").write(t)
print("ok")
