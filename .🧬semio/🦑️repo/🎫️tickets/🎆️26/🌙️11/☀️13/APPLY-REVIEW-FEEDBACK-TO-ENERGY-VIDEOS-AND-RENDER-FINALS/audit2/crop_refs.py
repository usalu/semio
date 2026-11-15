from PIL import Image
src = r"C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS/🗑️generated/physics-out/fr/Beat8_Zusammenfassung.png"
im = Image.open(src)
print(im.size)
box = (1340, 860, 1900, 940)
im.crop(box).resize(((box[2]-box[0])*3, (box[3]-box[1])*3), Image.LANCZOS).save(r"C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS/audit2/crop-refs.png")
