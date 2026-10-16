"""🧱 Builds base+stage overlay (no union rewriting) for census runs."""
import os, shutil, sys
from lib import *
def build(dest):
    shutil.rmtree(dest, ignore_errors=True)
    shutil.copytree(BASE, dest, symlinks=True)
    for dp, dn, fn in os.walk(STAGE):
        for f in fn:
            sp = os.path.join(dp, f); rel = os.path.relpath(sp, STAGE)
            if f.endswith(".deleted"):
                t = os.path.join(dest, rel[:-len(".deleted")])
                if os.path.exists(t): os.remove(t)
                continue
            tp = os.path.join(dest, rel)
            os.makedirs(os.path.dirname(tp), exist_ok=True)
            shutil.copy2(sp, tp)
if __name__ == "__main__":
    build(SP + "/S3"); print("overlay built")
