import os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
EVOLVE = os.path.join(HERE, "..", "schema-evolve")
sys.path.insert(0, EVOLVE)
os.environ.setdefault("EVOLVE_PYTHON", sys.executable)
import evolve, post
from core import Layout

JOBS = [
    ("add-collection", ["specs/en1993-fastened-joints.json"]),
]

def main():
    os.chdir(EVOLVE)
    last = None
    for command, rest in JOBS:
        rest = rest[:-1] + [os.path.join(EVOLVE, rest[-1])]
        ctx = evolve.build(command, "en1993", rest, True, None)
        print(f"[batch] {command} {rest[-1]}: {len(ctx.plan.files)} files, {len(ctx.unsupported)} unsupported", flush=True)
        if ctx.unsupported or evolve.scan_plan(ctx):
            print("[batch] refusing:", *ctx.unsupported, flush=True)
            return 2
        ctx.plan.commit()
        last = ctx
    print("[batch] all plans committed; twins, owners sync, gates once", flush=True)
    post.generate(last)
    import l_mutations
    l_mutations.apply(last)
    post.generate(last)
    post.owners_sync()
    return post.gates(last)

if __name__ == "__main__":
    sys.exit(main())
