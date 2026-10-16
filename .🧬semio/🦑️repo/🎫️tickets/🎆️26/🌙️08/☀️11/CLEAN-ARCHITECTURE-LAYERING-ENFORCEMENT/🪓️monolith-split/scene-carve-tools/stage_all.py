"""🏁 Stages the complete carved scene tree (sources, tests, manifests, project files, facade) under the scratchpad."""
import carve_all, emit_meta
from lib import *
files, reg_, owner, mo = carve_all.stage()
emit_meta.write_meta(NEW)
emit_meta.write_facade(NEW)
print("staged", NEW)
