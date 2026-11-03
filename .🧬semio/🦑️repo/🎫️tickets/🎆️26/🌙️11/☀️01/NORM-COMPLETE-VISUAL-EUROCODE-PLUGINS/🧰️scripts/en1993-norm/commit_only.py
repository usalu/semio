import os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
EVOLVE = os.path.join(HERE, "..", "schema-evolve")
sys.path.insert(0, EVOLVE)
import evolve
os.chdir(EVOLVE)
command, collection, spec = sys.argv[1:4]
rest = ([collection] if command == "add-leaf" else []) + [os.path.join(EVOLVE, spec)]
ctx = evolve.build(command, "en1993", rest, True, None)
print(len(ctx.plan.files), "files", len(ctx.unsupported), "unsupported")
if ctx.unsupported or evolve.scan_plan(ctx):
    sys.exit(2)
ctx.plan.commit()
