import json, os, sys
ROOT = "C:/git/semio/🧰️framework/🔨️modules/🖼️assets"
TOKENS = json.load(open("C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️.json", encoding="utf-8"))
sans = TOKENS["fontStacks"]["sans"].split(",")[0].strip('" ')
catalog = json.load(open(os.path.join(ROOT, "🔤️fonts/📇️catalog.json"), encoding="utf-8"))
sources = {}
for family in catalog["families"]:
    subset = next(part for part in family["subsets"] if part["sourceSubset"] in ("latin", None))
    sources[family["family"]] = os.path.join(ROOT, "🔤️fonts", family["directory"], subset["directory"], "📖️regular", catalog["encodings"]["ttf"])
path = sources[sans]
print("SANS", sans)
print("PATH", path, os.path.isfile(path))
try:
    from PIL import Image, ImageDraw, ImageFont
    font = ImageFont.truetype(path, 96)
    img = Image.new("RGB", (1500, 360), (10, 20, 30))
    d = ImageDraw.Draw(img)
    d.text((20, 20), "( ) [ ] (2024)", font=font, fill=(255, 220, 60))
    d.text((20, 200), "Willems (2024) [2024]", font=font, fill=(255, 255, 255))
    out = os.path.join(os.path.dirname(os.path.abspath(__file__)), "paren-glyph-check.png")
    img.save(out)
    print("SAVED", out)
except Exception as e:
    print("ERR", type(e).__name__, e)
