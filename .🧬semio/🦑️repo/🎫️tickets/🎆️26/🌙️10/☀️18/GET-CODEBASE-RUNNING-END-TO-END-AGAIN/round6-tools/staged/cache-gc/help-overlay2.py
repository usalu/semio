import os, shutil, sys
from paths import *
import patches

OVERLAY = sys.argv[1] if len(sys.argv) > 1 else "/private/tmp/cache-gc/h2"
O = J(B, OPERATIONS)
shutil.rmtree(OVERLAY, ignore_errors=True)
for rel in (J(E(0x1F4D6) + "help", TS), J(SCHEMA, JSON_), J(TESTS, FIXTURES, HELPDIR, JSON_)):
    destination = J(OVERLAY, O, rel)
    os.makedirs(os.path.dirname(destination), exist_ok=True)
    shutil.copyfile(J(ROOT, O, rel), destination)
with open(J(ROOT, TARGET["catalog"]), encoding="utf-8") as handle:
    patched = patches.EDITORS2["catalog"](handle.read())
destination = J(OVERLAY, TARGET["catalog"])
os.makedirs(os.path.dirname(destination), exist_ok=True)
with open(destination, "w", encoding="utf-8") as handle:
    handle.write(patched)
print("help overlay", OVERLAY)
