import sys
from PIL import Image
src,dst=sys.argv[1:3]
im=Image.open(src).convert("RGB")
im.quantize(96,method=Image.Quantize.MEDIANCUT,dither=Image.Dither.NONE).save(dst,optimize=True)
import os;print(dst,os.path.getsize(dst)//1024,"KB")
