"""rv_sheet.py <framedir> <scene> <outpng>: 2-column contact sheet of every frame PNG of a scene."""
import glob
import sys

from PIL import Image

frames = sorted(glob.glob(f"{sys.argv[1]}/{sys.argv[2]}_*.png"))
images = [Image.open(p) for p in frames]
w, h = images[0].size
cols = 2
rows = (len(images) + cols - 1) // cols
sheet = Image.new("RGB", (cols * w, rows * h))
for i, im in enumerate(images):
    sheet.paste(im, ((i % cols) * w, (i // cols) * h))
sheet.save(sys.argv[3])
print(sheet.size)
