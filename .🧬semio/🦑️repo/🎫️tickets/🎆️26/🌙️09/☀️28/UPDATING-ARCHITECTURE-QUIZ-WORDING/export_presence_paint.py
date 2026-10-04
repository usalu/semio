"""🖌️ Exports `presencePaint` of 👥️presence-presentation from the ui-react barrel and the `/chrome` subpath: one anchored
line after the barrel's PresenceBar re-export, one block at the end of the subpath."""

import pathlib
import sys
import time

ROOT = pathlib.Path("C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react")
EDITS = [
    (ROOT / "🟦️.tsx",
     "export { PresenceBar, presenceColor, presenceCssVar, PRESENCE_BAR_DEFAULT_MAX, type PresenceAppearance, type PresenceBarProps, type PresenceHsl, type PresencePeer, type PresenceRole };\n",
     "export { presencePaint } from \"../../🔨️modules/👥️presence-presentation/🟦️.ts\";\n"),
    (ROOT / "🪟️chrome" / "🟦️.ts",
     "  type ElementsSurfaceChromeInput,\n} from \"../🌓️appearance/🟦️.ts\";\n",
     "export { presenceColor, presenceCssVar, presencePaint, type PresenceAppearance, type PresenceHsl } from \"../../../🔨️modules/👥️presence-presentation/🟦️.ts\";\n"),
]
for path, anchor, line in EDITS:
    text = path.read_text(encoding="utf-8")
    if line in text:
        continue
    if text.count(anchor) != 1:
        sys.exit(f"{path.name}: anchor found {text.count(anchor)} times")
    text = text.replace(anchor, anchor + line)
    for attempt in range(20):
        try:
            path.write_text(text, encoding="utf-8", newline="")
            break
        except OSError:
            time.sleep(0.25 * (attempt + 1))
print("[export presence paint] done")
