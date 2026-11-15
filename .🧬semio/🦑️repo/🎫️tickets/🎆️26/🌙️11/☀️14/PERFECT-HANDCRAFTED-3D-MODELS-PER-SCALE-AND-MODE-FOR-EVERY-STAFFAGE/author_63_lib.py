import json, os, subprocess, re, glob
T = os.path.dirname(os.path.abspath(__file__))
BUN = sorted(glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]
GEN = os.path.join(T, "🗑️generated", "author-63")
os.makedirs(GEN, exist_ok=True)
CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans"

def src(slug): return os.path.join(T, "volumes", "humans", slug + ".json")
def load(slug): return json.load(open(src(slug)))
def save(slug, scale, **lists):
    d = load(slug); v = d["volumes"][str(scale)]
    for k, l in lists.items(): v[k] = l
    p = src(slug); tmp = p + ".a63.tmp"
    json.dump(d, open(tmp, "w"), ensure_ascii=False, separators=(",", ":")); os.replace(tmp, p)
def run(script, *a):
    r = subprocess.run([BUN, script, *a], cwd=T, capture_output=True, text=True); return r.stdout + r.stderr
def check(slug, scale=None, full=False):
    a = ["volumes-check.ts", f"humans/{slug}", "--product"] + (["--scale", str(scale)] if scale else [])
    out = run(*a)
    keep = []
    for l in out.splitlines():
        if l.startswith("[check]"): continue
        if l.startswith("FAIL") or l.startswith("ok") or l.startswith("humans/"):
            if full or l.startswith("FAIL") or l.startswith("humans/"): keep.append(l)
    # compress FAIL lines to the failing measurements
    res = []
    for l in keep:
        if l.startswith("FAIL") and "[p]" in l:
            head = l.split("[p]")[0].strip()
            bad = re.findall(r"!\S+", l)
            res.append(head + "  " + " ".join(b[:60] for b in bad))
        else: res.append(l[:300])
    return "\n".join(res)
def asset_path(slug): return os.path.join(CAT, slug, "🔣️.json")
def asset(slug): return json.load(open(asset_path(slug)))

def _norm(o):
    if isinstance(o, float) and o == int(o) and abs(o) < 1e15: return int(o)
    if isinstance(o, list): return [_norm(x) for x in o]
    if isinstance(o, dict): return {k: _norm(v) for k, v in o.items()}
    return o

def jsdump(o):
    from decimal import Decimal
    t = json.dumps(_norm(o), indent=2, ensure_ascii=False)
    return re.sub(r"-?\d+(?:\.\d+)?e[-+]\d+", lambda m: format(Decimal(m.group()), "f"), t)

def patch_section(slug, key, fn):
    """Apply fn(value)->value to asset[key] and rewrite only that section's text (indent 2), leaving the rest verbatim."""
    p = asset_path(slug); raw = open(p).read(); lines = raw.split("\n")
    start = next(i for i, l in enumerate(lines) if l.startswith(f'    "{key}": '))
    end = start
    first = lines[start]
    if first.rstrip().endswith("{") or first.rstrip().endswith("["):
        close = "    }" if first.rstrip().endswith("{") else "    ]"
        end = next(i for i in range(start + 1, len(lines)) if lines[i].rstrip(",") == close)
    body = "\n".join(lines[start:end + 1]); trailing = "," if lines[end].endswith(",") else ""
    val = json.loads("{" + body.rstrip(",") + "}")[key]
    new = fn(val)
    txt = jsdump({key: new}).split("\n")[1:-1]
    txt = ["  " + l for l in txt]
    txt[-1] += trailing
    if "\n".join(txt) == "\n".join(lines[start:end + 1]) and new == val: return False
    out = "\n".join(lines[:start] + txt + lines[end + 1:])
    tmp = p + ".a63.tmp"; open(tmp, "w").write(out); os.replace(tmp, p)
    return True
