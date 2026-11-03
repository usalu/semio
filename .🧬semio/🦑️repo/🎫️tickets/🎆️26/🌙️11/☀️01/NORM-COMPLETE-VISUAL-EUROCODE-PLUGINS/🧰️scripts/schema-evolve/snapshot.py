#!/usr/bin/env python3
"""📸 Regression snapshot of a dry run: writes {path: sha1} of every planned file so that generator changes can be diffed (python3 -I snapshot.py <artifact> <command> <spec> [entity] <out.json>)."""
import hashlib, json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import evolve
from ctx import Ctx
from spec import Spec


def plan_of(short, command, spec_path, entity=None):
    ctx = Ctx(short, Spec(spec_path))
    ctx.prefix = None
    if command == "add-collection":
        layers = evolve.collection_layers()
    else:
        import l_leaf
        ctx.entity_path = entity
        layers = l_leaf.layers()
    for name, fn in layers:
        ctx.guard(fn, name)
    return ctx


if __name__ == "__main__":
    short, command, spec = sys.argv[1:4]
    entity = sys.argv[4] if command == "add-leaf" else None
    ctx = plan_of(short, command, spec, entity)
    out = {p[len(evolve.SEMIO) + 1:]: [hashlib.sha1(n.encode()).hexdigest(), l] for p, (o, n, l) in ctx.plan.files.items()}
    json.dump({"files": out, "unsupported": ctx.unsupported}, open(sys.argv[-1], "w"), indent=1, ensure_ascii=False)
    print(len(out), "files", len(ctx.unsupported), "unsupported")
