import sys
import fitz
from PIL import Image
import io
src, out, zoom = sys.argv[1], sys.argv[2], float(sys.argv[3])
doc = fitz.open(src)
pix = doc[0].get_pixmap(matrix=fitz.Matrix(zoom, zoom), alpha=False)
Image.open(io.BytesIO(pix.tobytes("png"))).save(out)
