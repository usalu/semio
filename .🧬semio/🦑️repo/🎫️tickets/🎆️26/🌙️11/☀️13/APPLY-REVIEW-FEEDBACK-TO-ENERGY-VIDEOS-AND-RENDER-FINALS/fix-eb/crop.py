import sys
from PIL import Image
src, dst, box = sys.argv[1], sys.argv[2], tuple(int(v) for v in sys.argv[3].split(","))
Image.open(src).crop(box).save(dst)
