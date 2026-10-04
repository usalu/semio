"""🎨️ Moves the presence palette out of 👥️PresenceBar into the ui module 👥️presence-presentation (created beside it):
the region becomes a re-export, `presenceStyleColor` becomes the module's public `presencePaint`, and the now unused
`STYLING_PRESENCE_PALETTES` import leaves the element. Exact markers; the write only happens when every one matched."""

import pathlib
import sys
import time

path = pathlib.Path("C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/👥️PresenceBar/🟦️.tsx")
text = path.read_text(encoding="utf-8")
start = text.index("//#region 🔖️Palette\n")
end = text.index("//#endregion 🔖️Palette\n") + len("//#endregion 🔖️Palette\n")
region = (
    "//#region 🔖️Palette\n"
    "export { presenceColor, presenceCssVar, type PresenceAppearance, type PresenceHsl } from \"../../🔨️modules/👥️presence-presentation/🟦️.ts\";\n"
    "//#endregion 🔖️Palette\n"
)
replacements = [
    ('import { currentStylingAppearanceName, STYLING_PRESENCE_PALETTES } from "@semio-tech/ui-styling";\n',
     'import { currentStylingAppearanceName } from "@semio-tech/ui-styling";\n'
     'import { presencePaint } from "../../🔨️modules/👥️presence-presentation/🟦️.ts";\n'),
]
text = text[:start] + region + text[end:]
for old, new in replacements:
    if text.count(old) != 1:
        sys.exit(f"anchor count {text.count(old)}: {old!r}")
    text = text.replace(old, new)
if text.count("presenceStyleColor(") != 2:
    sys.exit(f"presenceStyleColor uses: {text.count('presenceStyleColor(')}")
text = text.replace("presenceStyleColor(", "presencePaint(")
for attempt in range(20):
    try:
        path.write_text(text, encoding="utf-8", newline="")
        break
    except OSError:
        time.sleep(0.25 * (attempt + 1))
print("[extract presence palette] done")
