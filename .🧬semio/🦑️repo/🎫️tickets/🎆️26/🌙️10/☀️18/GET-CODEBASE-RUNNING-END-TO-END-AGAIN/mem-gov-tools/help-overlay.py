import os, shutil, sys
from paths import *
import patches

OVERLAY = sys.argv[1] if len(sys.argv) > 1 else "/private/tmp/mem-gov/h"
O = J(B, OPERATIONS)
shutil.rmtree(OVERLAY, ignore_errors=True)
for rel in (J(HELPDIR, TS), J(SCHEMA, JSON_), J(TESTS, FIXTURES, HELPDIR, JSON_)):
    destination = J(OVERLAY, O, rel)
    os.makedirs(os.path.dirname(destination), exist_ok=True)
    shutil.copyfile(J(ROOT, O, rel), destination)
for key, rel in (("catalog", CATALOG), ("operations-schema", J(SCHEMA, JSON_))):
    with open(J(ROOT, TARGET[key]), encoding="utf-8") as handle:
        patched = patches.EDITORS[key](handle.read())
    destination = J(OVERLAY, TARGET[key])
    os.makedirs(os.path.dirname(destination), exist_ok=True)
    with open(destination, "w", encoding="utf-8") as handle:
        handle.write(patched)
print("help overlay", OVERLAY)
