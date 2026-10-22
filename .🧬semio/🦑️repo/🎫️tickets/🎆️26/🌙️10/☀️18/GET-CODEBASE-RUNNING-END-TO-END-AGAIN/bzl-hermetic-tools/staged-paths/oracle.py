"""Independent reference of the Bazel worst-case path law and output roots. Reads one JSON document on stdin: {schema, component, cases} and prints the expected rows.
Specification (all lengths in UTF-16 code units):
  limit        = min(256, win32 ? (long ? 32767 : 260) - 1 : inf)
  alias(R)     = "a/" + sha256(dirname(R))[:10] + "/" + basename(R)            (dirname of a top-level file is "")
  host-source  = len(checkout) + 1 + len(R)                                    kind host
  alias-source = len(base) + 1 + len("execroot/_main/") + len(alias(R))        needs base
  sandbox-alias-source = len(base) + 1 + len("sandbox/<longest sandbox>/<000000>/execroot/_main/") + len(alias(R))   needs base, sandboxes
  hermetic-root = len(root) + len(R)  kind hermetic; root = base_template + sep + "0"*workDigits + sep + workspace + sep;
                  a relative base is "<outputBase>/execroot/_main/" + base and needs base
  sandbox-stash = len(base) + 1 + len("sandbox/sandbox_stash/<stash mnemonic>/<000000>/" + snapshot)   kind hermetic, needs base, sandboxes
  snapshot     = len(base) + 1 + len("execroot/_main/bazel-out/<cfg>/bin/<t_0*16>.snapshot/files/<0*32>")   needs base
  tree         = len(base) + 1 + len("execroot/_main/bazel-out/<cfg>/bin/m_0*16/") + len(entry)   kind hermetic, needs base
  worst = the largest family; slack = limit - worst; fits = slack >= 0
  output root  = template with {drive} = drive of the checkout (win32) and {checkout} = sha256(lowercased backslash path)[:digits]
"""
import hashlib, json, re, sys

doc = json.loads(sys.stdin.read())
policy, budget = doc["schema"], doc["component"]
rules = policy["definitions"]["Policy"]["const"]
bud = budget["definitions"]["PathBudget"]["const"]


def u16(text):
    return len(text.encode("utf-16-le")) // 2


def alias(path):
    d = path.rsplit("/", 1)[0] if "/" in path else ""
    return "a/" + hashlib.sha256(d.encode()).hexdigest()[:10] + "/" + path.rsplit("/", 1)[-1]


def hermetic_root(platform, base):
    row = rules["hermeticRoot"][platform]
    b = row["base"].replace("{drive}", rules["fillers"]["drive"])
    relative = not re.match(r"^[A-Za-z]:|^[\\/]", b)
    if relative and base is None:
        return None
    text = ((base + "/" + rules["fillers"]["execroot"]) if relative else "") + b + row["separator"] + "0" * row["workDigits"] + row["separator"] + row["workspace"] + row["separator"]
    return u16(text)


def law(case):
    h, targets = case["host"], case["targets"]
    p = bud["platforms"][h["platform"]]
    limit = min(bud["repositoryUnits"], (((32767 if h["longPaths"] == "enabled" else 260) - 1) if h["platform"] == "win32" else 10 ** 9))
    cfg = p["configuration"][h["arch"]]
    base = h.get("outputBase")
    exe = rules["fillers"]["execroot"]
    sandbox = max(p["sandboxes"], key=len) if p["sandboxes"] else ""
    hroot = hermetic_root(h["platform"], base)
    snap = exe + "bazel-out/" + cfg + "/bin/" + bud["task"] + ".snapshot/files/" + bud["file"]
    tpre = exe + "bazel-out/" + cfg + "/bin/" + rules["fillers"]["tree"] + "/"
    rows = []
    for t in targets:
        fam = []
        longest = lambda vals, f: max([f(v) for v in vals] or [0])
        if t["kind"] == "host" and h.get("checkout") is not None and t["sources"]:
            fam.append(["host-source", u16(h["checkout"]) + 1 + longest(t["sources"], u16)])
        if base is not None and t["sources"]:
            fam.append(["alias-source", u16(base) + 1 + u16(exe) + longest(t["sources"], lambda s: u16(alias(s)))])
            if p["sandboxes"]:
                fam.append(["sandbox-alias-source", u16(base) + 1 + u16("sandbox/" + sandbox + "/" + "0" * bud["sandboxDigits"] + "/" + exe) + longest(t["sources"], lambda s: u16(alias(s)))])
        if t["kind"] == "hermetic" and base is not None and p["sandboxes"]:
            fam.append(["sandbox-stash", u16(base) + 1 + u16("sandbox/sandbox_stash/" + rules["fillers"]["stash"] + "/" + "0" * bud["sandboxDigits"] + "/" + snap)])
        if t["kind"] == "hermetic" and hroot is not None and t["sources"]:
            fam.append(["hermetic-root", hroot + longest(t["sources"], u16)])
        if base is not None:
            fam.append(["snapshot", u16(base) + 1 + u16(snap)])
        if t["kind"] == "hermetic" and base is not None and t.get("tree"):
            fam.append(["tree", u16(base) + 1 + u16(tpre) + longest(t["tree"], u16)])
        worst = None
        for name, units in fam:
            if worst is None or units > worst[1]:
                worst = [name, units]
        slack = limit if worst is None else limit - worst[1]
        rows.append({"id": t["id"], "families": [{"name": n, "units": v} for n, v in fam], "worst": None if worst is None else {"name": worst[0], "units": worst[1]}, "slack": slack, "fits": slack >= 0})
    return {"limit": limit, "rows": rows, "over": [r["id"] for r in rows if not r["fits"]], "fits": all(r["fits"] for r in rows)}


def output_root(case):
    platform, checkout = case["platform"], case["checkout"]
    row = rules["outputRoot"][platform]
    if row is None:
        return None
    win = platform == "win32"
    norm = checkout.replace("/", "\\").lower() if win else checkout
    m = re.match(r"^([A-Za-z]:)", checkout.replace("/", "\\")) if win else None
    drive = m.group(1).upper() if m else rules["fillers"]["drive"]
    return row["template"].replace("{drive}", drive).replace("{checkout}", hashlib.sha256(norm.encode()).hexdigest()[: row["checkoutDigits"]])


print(json.dumps({"law": [law(c) for c in doc["cases"]["law"]], "roots": [output_root(c) for c in doc["cases"]["roots"]]}))
