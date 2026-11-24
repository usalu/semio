"""🗺️ Turns the proposals of `../MR/suggest.ts` into the ops of an asset: `python3 plan.py <asset.json> <suggest.json> <bone-regex> [--scales 50,100] [--skip <id-regex>] [--trunk <bone-regex>] [--front-only <bone-regex>]` keeps the proposals of the bones the gate names and decides the ones marked for review by the kind of part: a shoe (bones `foot-*`, `toe-*`) is a prism, so its front and plan become the elevation outline swept laterally (`extrude`); a part that stands along its bone and is looked at along it in the plan (bones of `--trunk`, default neck, head, torso, lumbar, pelvis) keeps its plan outline as the section of a prism along the bone (`plan-sweep-u` of `fit.ts` in this folder); a limb is revolved; the plan proposals of the bones of `--front-only` are dropped. Prints the ops as JSON."""
import json, re, sys

args = sys.argv[1:]
def flag(name, default=None):
    return args[args.index(name) + 1] if name in args else default
asset = json.load(open(args[0]))["asset"]
proposals = json.load(open(args[1]))
keep = re.compile(args[2])
scales = flag("--scales")
skip = re.compile(flag("--skip", "^$"))
trunk = re.compile(flag("--trunk", r"^(neck|head|torso|lumbar|pelvis)$"))
shoe = re.compile(r"^(foot|toe)-")
front_only = re.compile(flag("--front-only", "^$"))
ops = []
for proposal in proposals:
    scale, ident = proposal["scale"], proposal["id"]
    if scales and scale not in scales.split(","):
        continue
    bone = proposal.get("bone") or next(shape["bone"] for shape in asset["representations"][scale]["geometry"]["elevation"] if shape["id"] == ident)
    if not keep.search(bone) or skip.search(ident):
        continue
    if front_only.search(bone) and (proposal["op"] == "plan-revolve" or proposal.get("view") == "plan"):
        continue
    op = {key: value for key, value in proposal.items() if key not in ("why", "review")}
    if shoe.search(bone):
        if op["op"] == "front-sweep":
            op = {"op": "extrude", "scale": scale, "id": ident, **({"bone": op["bone"]} if "bone" in op else {}), "view": "front"}
        elif op["op"] == "plan-revolve":
            op = {"op": "extrude", "scale": scale, "id": ident, **({"bone": op["bone"]} if "bone" in op else {}), "view": "plan"}
    elif op["op"] == "plan-revolve" and trunk.search(bone):
        op["op"] = "plan-sweep-u"
    ops.append(op)
print("[\n" + ",\n".join("  " + json.dumps(op) for op in ops) + "\n]")
