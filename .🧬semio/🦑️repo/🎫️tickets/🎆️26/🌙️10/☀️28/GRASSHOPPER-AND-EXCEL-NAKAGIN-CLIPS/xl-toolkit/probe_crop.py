import sys
from PIL import Image
im = Image.open(sys.argv[1]).crop((30, 125, 660, 200)); im = im.resize((im.width * 4, im.height * 4), Image.LANCZOS); im.save(sys.argv[2])
