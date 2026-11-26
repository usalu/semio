"""🪑️ The `--with-500` ops of `used-bentwood-dining-chairs` of lane MU written into THIS folder (`ops/used-bentwood-dining-chairs.json`) instead of `../MU/ops`, so that the trial of round MV-L3 leaves the chain of MU as it is until the light proof passed: `python3 mu-ops.py` runs `../MU/make/used-bentwood-dining-chairs.py --with-500` with the output folder of `../MU/lib.py` set here."""
import os, runpy, sys
HERE = os.path.dirname(os.path.abspath(__file__))
MU = os.path.join(os.path.dirname(HERE), "MU")
sys.path.insert(0, MU)
import lib
lib.HERE = HERE
sys.argv = [os.path.join(MU, "make", "used-bentwood-dining-chairs.py"), "--with-500"]
runpy.run_path(sys.argv[0], run_name="__main__")
