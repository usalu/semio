"""One-shot, idempotent: make the archived Zukunft Bau page self-contained (drop remote preloads/icon, drop @font-face sources for fonts that were never archived)."""
import re, sys
root = "♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🌐️public"
html = f"{root}/🌐️zukunft-bau-entwerfen-mit-bestand.html"
css = f"{root}/🧰️zukunft-bau-entwerfen-mit-bestand_files/📐️layout.css"
s = open(html, encoding="utf8").read()
n = s
n = re.sub(r'[ \t]*<link rel="preload" href="https?://[^"]*" as="font"[^>]*/>\n', "", n)
n = re.sub(r'[ \t]*<link rel="icon" href="https?://[^"]*"[^>]*/>\n', "", n)
open(html, "w", encoding="utf8").write(n)
c = open(css, encoding="utf8").read()
def block(m):
    b = m.group(0)
    def sources(sm):
        kept = [x for x in re.split(r",\s*(?![^(]*\))", sm.group(2).strip()) if x and "../Fonts/" not in x]
        return "" if not kept else sm.group(1) + ",\n    ".join(kept) + ";"
    b = re.sub(r"(src:\s*)((?:[^;}]|\([^)]*\))*);", lambda sm: sources(sm), b)
    return "" if "src:" not in b else b
c2 = re.sub(r"@font-face\s*\{[^}]*\}\n?", block, c)
open(css, "w", encoding="utf8").write(c2)
print("html bytes", len(s), "->", len(n), "css bytes", len(c), "->", len(c2), "font-face", len(re.findall("@font-face", c)), "->", len(re.findall("@font-face", c2)))
