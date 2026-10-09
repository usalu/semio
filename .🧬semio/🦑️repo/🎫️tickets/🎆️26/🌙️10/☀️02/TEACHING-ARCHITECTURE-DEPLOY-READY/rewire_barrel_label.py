import os

chrome = open(os.path.join(os.environ["TEMP"], "semio-chrome-path.txt"), encoding="utf-8").read().strip()
react = os.path.dirname(chrome)
barrel = os.path.join(react, "\U0001f7e6\ufe0f.tsx")
text = open(barrel, encoding="utf-8").read()
start = text.find("/** @emoji \U0001faa7\ufe0f Static shell title")
end = text.find("/** @emoji \U0001faa7\ufe0f Uppercase shell section title")
if start < 0 or end < 0:
    raise SystemExit(f"markers {start} {end}")
replacement = 'export { ResponsiveLabel, shellChromeTitleClassName } from "./\U0001faa7\ufe0fchrome/responsive-label/\U0001f7e6\ufe0f.tsx";\n\n'
# chrome folder emoji is the directory name
folder = os.path.basename(chrome)
replacement = f'export {{ ResponsiveLabel, shellChromeTitleClassName }} from "./{folder}/responsive-label/\U0001f7e6\ufe0f.tsx";\n\n'
open(barrel, "w", encoding="utf-8", newline="\n").write(text[:start] + replacement + text[end:])
chrome_file = os.path.join(chrome, "\U0001f7e6\ufe0f.ts")
line = [row for row in open(chrome_file, encoding="utf-8") if "ResponsiveLabel" in row][0]
print("chrome-repr", repr(line))
print("barrel-ok", folder)
