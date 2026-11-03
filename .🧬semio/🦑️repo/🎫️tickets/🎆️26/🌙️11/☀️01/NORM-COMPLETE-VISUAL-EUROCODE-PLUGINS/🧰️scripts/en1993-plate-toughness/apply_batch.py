"""🧱 Applies several en1993 leaves with ONE twin generation: the file layers of every leaf are committed in turn (idempotent), then `schema twins generate`, the build-owner sync and the gates run once (evolve.py runs the generation twice per leaf, each a queue wait in heavy.sh).
Usage: /home/deck/semio/.venv/bin/python -I apply_batch.py ENTITY:SPEC [ENTITY:SPEC ...]
"""
import os, sys
TICKET = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", ".."))
sys.path.insert(0, TICKET + "/🧰️scripts/schema-evolve")
import evolve, post

ctx = None
for item in sys.argv[1:]:
    entity, spec = item.split(":")
    ctx = evolve.build("add-leaf", "en1993", [entity, f"{TICKET}/🧰️scripts/schema-evolve/specs/{spec}.json"], True, None)
    for line in ctx.plan.report():
        print(line)
    print(f"[batch] {entity} {spec}: {len(ctx.plan.files)} files, {len(ctx.unsupported)} unsupported")
    if ctx.unsupported or evolve.scan_plan(ctx):
        sys.exit(f"[batch] refusing {spec}")
    ctx.plan.commit()
post.generate(ctx)
post.owners_sync()
sys.exit(post.gates(ctx))
