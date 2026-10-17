"""🧵 rw2t step D1b: the crates whose laws read scene state build scene-state with the testkit cell in their test units."""
from mf import *
line = FP + 'scene-state = { path = "../../../🎭️scene-state/📦️packages/🦀️rust", features = ["testkit"] }'
for f in ["🔄️canvas2d-host", "🎨️scene-canvas2d", "✒️scene-ink", "📰️scene-feeds", "🧮️scene-tables", "📁️scene-text-vfs", "💠️icon-host", "🪞️scene-render"]:
    add_lines(mpath(f), "dev-dependencies", [line])
