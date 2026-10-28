import sys,json
from pathlib import Path
import pdfplumber
from importlib.util import spec_from_file_location,module_from_spec
spec=spec_from_file_location("mixed",Path(__file__).parent/"verify-mixed-pdf.py")
m=module_from_spec(spec)
spec.loader.exec_module(m)
with pdfplumber.open(sys.argv[1]) as pdf:
    page=pdf.pages[0]
    xs,ys=m.matrix(page)
    art=m.primitives(page,(xs[3],ys[0],xs[4],ys[1]))
    print(json.dumps([{k:obj.get(k) for k in ["object_type","dash","linewidth","stroke","fill","x0","x1","top","bottom","path"]} for obj in art],ensure_ascii=False))
