"""🧾️ A one-paragraph summary of an ops file for the report: `python3 opsum.py ops/<id>.json…` prints per file the ops grouped by kind and view with their scales and shape ids."""
import json, sys, collections, os
for path in sys.argv[1:]:
    groups = collections.OrderedDict()
    for o in json.load(open(path)):
        kind = o["op"] + (" " + o["view"] if "view" in o else "") + (" → " + ",".join(o["to"]) if "to" in o else "") + (" (width of the front)" if o.get("radius") == "front" else " (rebuilt)" if o.get("rebuild") else "") + (" (new shape)" if o.get("create") else "")
        groups.setdefault(kind, collections.OrderedDict()).setdefault(o.get("as", o["id"]), []).append(o.get("scale", "-"))
    print("**%s** (%d ops): " % (os.path.basename(path)[:-5], sum(len(v) for g in groups.values() for v in g.values())) + "; ".join("`%s`: %s" % (k, ", ".join("%s (1:%s)" % (i, "/".join(s)) for i, s in v.items())) for k, v in groups.items()))
