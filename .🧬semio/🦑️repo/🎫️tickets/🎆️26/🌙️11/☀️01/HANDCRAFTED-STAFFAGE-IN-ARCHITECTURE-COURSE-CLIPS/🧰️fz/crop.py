import sys
from PIL import Image

im = Image.open(sys.argv[1])
im.crop((0, 130, 380, 250)).resize((1520, 480), Image.LANCZOS).save(sys.argv[2])
