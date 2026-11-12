import sys
import numpy as np
from PIL import Image
a,b=(np.asarray(Image.open(p).convert("RGB")).astype(int) for p in sys.argv[1:3])
print(sys.argv[1].split("/")[-1], int((np.abs(a-b).sum(axis=2)>0).sum()))
