import sys
from PIL import Image
im = Image.open(sys.argv[1]); W, H = im.size
cx, cy = W / 2, H / 2; s = W / 14.2222
box = (int(cx - 5 * s), int(cy - 0.8 * s), int(cx + 5 * s), int(cy + 0.8 * s))
c = im.crop(box); c = c.resize((c.width * 2 // 1, c.height * 2 // 1)); c.save(sys.argv[2])
