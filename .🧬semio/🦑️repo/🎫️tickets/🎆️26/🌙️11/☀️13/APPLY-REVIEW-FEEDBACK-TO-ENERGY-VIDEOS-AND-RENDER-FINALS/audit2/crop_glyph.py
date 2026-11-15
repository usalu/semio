from PIL import Image
base = r"C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS"
im = Image.open(base + "/🗑️generated/physics-out/fr/Beat8_Zusammenfassung.png")
out = Image.new("RGB", (1200, 420), (0, 0, 0))
a = im.crop((1655, 790, 1745, 815)).resize((900, 250), Image.NEAREST)
out.paste(a, (0, 0))
b = im.crop((1640, 880, 1800, 905)).resize((960, 150), Image.NEAREST)
out.paste(b, (0, 260))
out.save(base + "/audit2/crop-glyphs.png")
