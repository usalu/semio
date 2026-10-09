import os

chrome = open(os.path.join(os.environ["TEMP"], "semio-chrome-path.txt"), encoding="utf-8").read().strip()
chrome_file = os.path.join(chrome, "\U0001f7e6\ufe0f.ts")
text = open(chrome_file, encoding="utf-8").read()
old = 'export { ResponsiveLabel, shellChromeTitleClassName } from "../\U0001f7e6\ufe0f.tsx";'
new = 'export { ResponsiveLabel, shellChromeTitleClassName } from "./responsive-label/\U0001f7e6\ufe0f.tsx";'
if old not in text:
    raise SystemExit("chrome export not found")
open(chrome_file, "w", encoding="utf-8", newline="\n").write(text.replace(old, new, 1))

react = os.path.dirname(chrome)
barrel = os.path.join(react, "\U0001f7e6\ufe0f.tsx")
barrel_text = open(barrel, encoding="utf-8").read()
start = barrel_text.find("/** @emoji \U0001f527 Static shell title")
end = barrel_text.find("/** @emoji \U0001f527 Uppercase shell section title")
if start < 0 or end < 0:
    raise SystemExit(f"barrel markers missing {start} {end}")
replacement = 'export { ResponsiveLabel, shellChromeTitleClassName } from "./\U0001fa9f\ufe0fchrome/responsive-label/\U0001f7e6\ufe0f.tsx";\n\n'
open(barrel, "w", encoding="utf-8", newline="\n").write(barrel_text[:start] + replacement + barrel_text[end:])
print("updated")
