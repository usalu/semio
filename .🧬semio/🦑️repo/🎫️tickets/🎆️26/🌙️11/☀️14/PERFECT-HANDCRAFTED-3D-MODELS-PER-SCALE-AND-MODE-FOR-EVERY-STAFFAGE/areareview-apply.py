"""One review pass: re-runs the area audit, applies the decisions of areareview-decisions.py surgically (areaM2 number token, areaBasis line), records them in
🗑️generated/areareview/state.json and lists the rows that have no decision yet. Usage: python3 areareview-apply.py [--dry]"""
import glob, importlib.util, json, os, re, subprocess, sys, tempfile, time

T = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(T, *[".."] * 7))
ROOT = os.path.join(REPO, "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
GEN = os.path.join(T, "🗑️generated", "areareview")
STATE = os.path.join(GEN, "state.json")
DRY = "--dry" in sys.argv

spec = importlib.util.spec_from_file_location("dec", os.path.join(T, "areareview-decisions.py"))
dec = importlib.util.module_from_spec(spec)
spec.loader.exec_module(dec)
D = dec.D

def run_audit():
    subprocess.run(["python3", "areaaudit.py"], cwd=T, capture_output=True)
    return json.load(open(os.path.join(T, "area-audit.json")))

def path_of(slug):
    return glob.glob(f"{ROOT}/*/{slug}/🔣️.json")[0]

def committed_regions(path):
    rel = os.path.relpath(path, REPO)
    try:
        doc = json.loads(subprocess.check_output(["git", "-C", REPO, "show", "HEAD:" + rel]))
    except Exception:
        return {}
    return {r["id"]: r for r in doc["asset"]["thermal"]["regions"]}

def edit(text, region, value, basis):
    mt = min(re.finditer(r"\n( +)\"thermal\": \{\n( +)\"\w+\"", text), key=lambda m: len(m.group(1)))
    i1, i2 = len(mt.group(1)), len(mt.group(2))
    k = " " * (i2 + 2 * (i2 - i1))
    kc = " " * (i2 + (i2 - i1))
    r0 = text.index("\n" + " " * i2 + "\"regions\": [", mt.start())
    r1 = text.index("\n" + " " * i2 + "]", r0)
    marker = f"\n{k}\"id\": \"{region}\","
    at = text.index(marker, r0)
    if at > r1 or text.count(marker, r0, r1) != 1:
        raise ValueError(f"region {region} not unique")
    end = text.find("\n" + kc + "}", at)
    m = re.compile(r"(\n" + k + r"\"areaM2\": )([-0-9.eE+]+)(,)").search(text, at, end)
    if m is None:
        raise ValueError(f"no areaM2 for {region}")
    tok = value if value is not None else m.group(2)
    out = text[: m.start(2)] + tok + text[m.end(2):]
    if basis:
        seg = out[at: out.find("\n" + kc + "}", at)]
        if "\"areaBasis\"" not in seg:
            c = out.index(",", m.start(2) + len(tok))
            out = out[: c + 1] + "\n" + k + "\"areaBasis\": \"exchange\"," + out[c + 1:]
    return out

def apply(path, region, value, basis):
    for attempt in range(8):
        text = open(path, encoding="utf8").read()
        before = json.loads(text)
        tok = None if value is None else json.dumps(value)
        new = edit(text, region, tok, basis)
        after = json.loads(new)
        exp = json.loads(text)
        for r in exp["asset"]["thermal"]["regions"]:
            if r["id"] == region:
                if value is not None:
                    r["areaM2"] = value
                if basis:
                    r["areaBasis"] = "exchange"
        assert after == exp, f"{path}: more than the target changed"
        if new == text:
            return False
        if DRY:
            return True
        if open(path, encoding="utf8").read() != text:
            time.sleep(0.2)
            continue
        fd, tmp = tempfile.mkstemp(dir=os.path.dirname(path))
        with os.fdopen(fd, "w", encoding="utf8") as f:
            f.write(new)
        os.replace(tmp, path)
        return True
    raise RuntimeError(f"{path}: file kept changing")

def main():
    audit = run_audit()
    state = json.load(open(STATE)) if os.path.exists(STATE) else {}
    stamp = time.strftime("%Y-%m-%d %H:%M")
    rows = {(a["slug"], a["region"]): a for a in audit["areas"]}
    keys = set(rows) | {k for k in D if k not in rows}
    new_undecided, rereview, applied, skipped = [], [], [], []
    for key in sorted(keys):
        slug, region = key
        a = rows.get(key)
        path = path_of(slug)
        doc = json.load(open(path))["asset"]
        reg = next((r for r in doc["thermal"]["regions"] if r["id"] == region), None)
        com = committed_regions(path).get(region)
        if reg is None or com is None:
            continue
        old, cur = com["areaM2"], reg["areaM2"]
        if key not in D:
            if a is not None:
                new_undecided.append((slug, region, old, cur))
            continue
        act, val, why, finding = D[key]
        prev = state.get(f"{slug}/{region}")
        if prev and prev.get("seen") != cur and prev.get("final") != cur and cur != old and act != "DEFER" and not (act == "A" and val == cur):
            rereview.append((slug, region, old, prev.get("seen"), cur))
            continue
        basis = act in ("RX", "SX", "X")
        if act == "A":
            final = val if val is not None else cur
        elif act in ("RX", "RF", "RB"):
            final = old
        elif act in ("S", "SX"):
            final = val
        elif act == "X":
            final = cur
        else:
            final = cur
        changed = False
        if act != "DEFER":
            tgt = None if final == cur else final
            if tgt is not None or (basis and reg.get("areaBasis") != "exchange"):
                changed = apply(path, region, tgt, basis and reg.get("areaBasis") != "exchange")
        if prev is None or changed or prev.get("act") != act or prev.get("final") != final:
            state[f"{slug}/{region}"] = dict(slug=slug, region=region, cat=os.path.basename(os.path.dirname(os.path.dirname(path))), kind=reg["kind"], model=reg["model"], committed=old,
                                             author=prev["author"] if prev else (cur if a is not None else (val if val is not None else cur)), seen=cur if not prev else prev.get("seen"), final=final,
                                             act=act, reason=why, finding=finding, basis=basis, heatW=(reg.get("operating") or {}).get("heatW"), first=(prev or {}).get("first", stamp), last=stamp)
            if changed:
                applied.append((slug, region, act, cur, final))
        else:
            state[f"{slug}/{region}"]["last"] = stamp
    state["_pending"] = dict(undecided=[list(x) for x in new_undecided], rereview=[list(x) for x in rereview])
    state.setdefault("_passes", []).append(dict(time=stamp, rows=len(rows), applied=len(applied), undecided=len(new_undecided), rereview=len(rereview)))
    if not DRY:
        os.makedirs(GEN, exist_ok=True)
        json.dump(state, open(STATE, "w"), ensure_ascii=False, indent=1)
    print(f"{stamp}: audit rows {len(rows)}, decisions {len(D)}, applied {len(applied)}, undecided {len(new_undecided)}, re-review {len(rereview)}")
    for x in applied:
        print("  applied", x)
    for x in new_undecided:
        print("  UNDECIDED", x)
    for x in rereview:
        print("  RE-REVIEW", x)

main()
