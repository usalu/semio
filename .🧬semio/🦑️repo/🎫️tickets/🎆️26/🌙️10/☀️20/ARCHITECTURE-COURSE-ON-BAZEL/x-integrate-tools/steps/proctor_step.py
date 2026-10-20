"""x-proctor in process: the manifest rewrites of its staged apply.py on an overlay root; the acquisition flip only while the switch still exists (the cleanup step deletes it)."""
import json, os, re, subprocess, sys, time
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

STAGED = os.path.join(TICKET, "x-proctor-tools", "staged")


def run(root):
    module = load_script(os.path.join(STAGED, "apply.py"))
    for attempt in range(3):
        generated = subprocess.run(["bun", os.path.join(STAGED, "inputs.ts"), LIVE], capture_output=True, text=True, cwd=LIVE)
        if not generated.returncode:
            break
        time.sleep(3)
    if generated.returncode:
        refuse("x-proctor inputs.ts failed three times (the Cargo closure is read from the checkout, which peers edit): " + generated.stderr.strip()[-400:])
    globs = json.loads(generated.stdout)["globs"]
    old_proctor, old_course, old_product = read(root, PROCTOR), read(root, COURSE), read(root, PRODUCT)
    new_proctor, new_course = module["proctor_manifest"](old_proctor, globs), module["course_manifest"](old_course)
    module["post_conditions"](new_proctor, new_course)
    product = old_product
    if "PROCTOR_ACQUISITION_DEFAULT" in old_product:
        product = re.sub(r'(PROCTOR_ACQUISITION_DEFAULT: "product" \| "build" = )"(?:build|product)"', r'\1"product"', old_product)
    write_changed = [(PROCTOR, old_proctor, new_proctor), (COURSE, old_course, new_course), (PRODUCT, old_product, product)]
    for rel, old, new in write_changed:
        if old != new:
            write(root, rel, new)
    return [rel for rel, old, new in write_changed if old != new]
