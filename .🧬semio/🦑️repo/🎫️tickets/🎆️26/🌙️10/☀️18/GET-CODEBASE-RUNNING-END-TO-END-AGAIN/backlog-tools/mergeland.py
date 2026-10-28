"""🧬 [backlog] 3-way landing of WHOLE-FILE staged batches (route2, xplat3, xplat4 x3, obs-g fleet, nxfix core/nx-patch, nxlock, dashlock): `mine` = the staged full file, `base` = the bytes it was cut from (kept flat in backlog-tools/merge/base0/<id>/), `theirs` = the file now. A peer edit since staging no longer refuses the whole batch: `git merge-file` combines it, only a real conflict refuses (nothing written). `revert` restores the byte backup taken by `apply` when the file still equals what apply wrote, else merges the reverse change. stdlib + git. usage: mergeland.py <id> check|apply|revert [--root R] | mergeland.py register <id> [--base-mirror blbase] | mergeland.py rebase <id> <rel-suffix> --from-mirror <mirror> (base := the file now, after a hand resolution of the staged copy)"""
import hashlib, json, os, re, shutil, subprocess, sys, tempfile
C = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
LIVE = os.path.abspath(os.path.join(C, *[".."] * 7))
M = C + "/backlog-tools/merge"
LIBP = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
def sha(b): return hashlib.sha256(b).hexdigest()
def rd(p):
    with open(p, "rb") as h: return h.read()
def wr(p, b):
    os.makedirs(os.path.dirname(p), exist_ok=True); t = p + ".bl-tmp"
    with open(t, "wb") as h: h.write(b)
    if os.path.exists(p): os.chmod(t, os.stat(p).st_mode & 0o7777)
    os.replace(t, p)
def lines_of(text): return text.split("\n")
def parses(rel, data):
    """parse gate: a merged output must be accepted by Bun's scanner (`Bun.Transpiler.scan`: syntax errors AND duplicate declarations, which `bun build --no-bundle` lets through) or by the JSON parser before it is written; -> error text or None"""
    ext = os.path.splitext(rel)[1]
    if ext == ".json":
        try: json.loads(data.decode("utf8")); return None
        except Exception as error: return str(error)[:200]
    if ext not in (".ts", ".mts", ".mjs", ".js", ".cts", ".cjs") or rel.endswith((".d.ts", ".d.mts")): return None
    with tempfile.TemporaryDirectory() as d:
        f = os.path.join(d, "p" + ext); open(f, "wb").write(data)
        script = 'import {readFileSync,writeFileSync} from "node:fs";const rel=process.argv[1];try{const loader=/\\.[cm]?tsx?$/u.test(rel)?"ts":"js",source=readFileSync(process.argv[2],"utf8").replace(/^#![^\\r\\n]*/u,"");new Bun.Transpiler({loader}).scan(source);writeFileSync(process.argv[3],new Bun.Transpiler({loader,trimUnusedImports:false}).transformSync(source));}catch(error){console.error(String(error.message));process.exit(1);}'
        out = os.path.join(d, "out.mjs")
        r = subprocess.run([os.environ.get("BUN", "bun"), "-e", script, rel, f, out], capture_output=True, text=True)
        if r.returncode == 0:
            node = subprocess.run(["node", "--check", out], capture_output=True, text=True)
            line = next((l for l in node.stderr.split("\n") if "Error" in l), "")
            if node.returncode != 0 and not re.search(r"await is only valid|Cannot use import statement", line): return line[:200]
        return None if r.returncode == 0 else (r.stderr or r.stdout)[:300]
def char_merge(b, m, t):
    """one-line hunk: re-apply the substring conv change base->mine onto the changed line `t` (needs a unique 48-char context on both sides)"""
    i = 0
    while i < min(len(b), len(m)) and b[i] == m[i]: i += 1
    j = 0
    while j < min(len(b), len(m)) - i and b[-1 - j] == m[-1 - j]: j += 1
    old, new = b[i:len(b) - j], m[i:len(m) - j]; before, after = b[max(0, i - 48):i], b[i + len(old):i + len(old) + 48]
    return t.replace(before + old + after, before + new + after) if t.count(before + old + after) == 1 else None
def both_inserts(b, m, t):
    """both sides inserted at the very same spot of one line (a list element on each side): keep both, now first"""
    i = 0
    while i < min(len(b), len(m)) and b[i] == m[i]: i += 1
    j = 0
    while j < min(len(b), len(m)) - i and b[-1 - j] == m[-1 - j]: j += 1
    k = 0
    while k < min(len(b), len(t)) and b[k] == t[k]: k += 1
    l = 0
    while l < min(len(b), len(t)) - k and b[-1 - l] == t[-1 - l]: l += 1
    if i + j == len(b) and k + l == len(b) and len(m) > len(b) and len(t) > len(b) and abs(i - k) <= 12:
        return b[:k] + t[k:len(t) - l] + m[i:len(m) - j] + b[k:]
    return None
def block_merge(m, b, t):
    """one side only inserted lines around the unchanged base block, the other side changed it: keep the changed lines inside the insertion"""
    for x, y, first in ((m, t, False), (t, m, True)):
        if len(x) > len(b) and any(x[i:i + len(b)] == b for i in range(len(x) - len(b) + 1)):
            i = next(i for i in range(len(x) - len(b) + 1) if x[i:i + len(b)] == b); return x[:i] + y + x[i + len(b):]
    return None
def per_line(b, x):
    """-> (replacement list per base line index, insertions per gap 0..len(b)) of the line diff base -> x"""
    import difflib
    rep = {i: [b[i]] for i in range(len(b))}; gap = {i: [] for i in range(len(b) + 1)}
    for op, a0, a1, b0, b1 in difflib.SequenceMatcher(None, b, x, autojunk=False).get_opcodes():
        if op == "replace":
            for k in range(a0, a1): rep[k] = []
            rep[a0] = x[b0:b1]
        elif op == "delete":
            for k in range(a0, a1): rep[k] = []
        elif op == "insert": gap[a0] += x[b0:b1]
    return rep, gap
def linewise(m, b, t):
    """small hunk: apply both sides' line changes; a line changed by both is char-merged, insertions at the same gap keep live first"""
    rm, gm = per_line(b, m); rt, gt = per_line(b, t); out = []
    for i in range(len(b) + 1):
        out += gt[i] + gm[i]
        if i < len(b):
            if rm[i] == [b[i]]: out += rt[i]
            elif rt[i] == [b[i]]: out += rm[i]
            elif rm[i] and rt[i] and (one := char_merge(b[i], rm[i][-1], rt[i][-1])) is not None: out += rt[i][:-1] + rm[i][:-1] + [one]
            else: return None
    return out
def merge(mine, base, theirs):
    with tempfile.TemporaryDirectory() as d:
        for n, b in (("m", mine), ("b", base), ("t", theirs)): open(f"{d}/{n}", "wb").write(b)
        r = subprocess.run(["git", "merge-file", "-p", f"{d}/t", f"{d}/b", f"{d}/m"], capture_output=True)
        if r.returncode == 0: return r.stdout, 0
        try:
            lines = subprocess.run(["git", "merge-file", "-p", "--diff3", "-L", "now", "-L", "base", "-L", "staged", f"{d}/t", f"{d}/b", f"{d}/m"], capture_output=True).stdout.decode("utf8").split("\n")
            res = []; i = 0
            while i < len(lines):
                if lines[i].startswith("<<<<<<< "):
                    j = lines.index("||||||| base", i); k = lines.index("=======", j); e = next(x for x in range(k, len(lines)) if lines[x].startswith(">>>>>>> "))
                    now, bas, stg = lines[i + 1:j], lines[j + 1:k], lines[k + 1:e]
                    if len(now) == len(bas) == len(stg) == 1 and (one := char_merge(bas[0], stg[0], now[0])) is not None: res.append(one)
                    elif len(now) == len(bas) == len(stg) == 1 and (two := both_inserts(bas[0], stg[0], now[0])) is not None: res.append(two)
                    elif len(bas) <= 8 and (four := linewise(stg, bas, now)) is not None: res += four
                    elif (three := block_merge(stg, bas, now)) is not None: res += three
                    else: return None, r.returncode
                    i = e + 1
                else: res.append(lines[i]); i += 1
            return "\n".join(res).encode("utf8"), 0
        except Exception: return None, r.returncode
def sources(bid):
    """-> list of (rel, mine path, base-bytes-from-recorded-sha or None)"""
    out = []
    if bid == "route2":
        T = C + "/route2-g-tools"; man = json.load(open(T + "/staged-base.json", encoding="utf8"))
        for sub, pre in (("staged", LIBP + "🏗️build/"), ("staged-lib", LIBP)):
            for dp, _, fn in os.walk(f"{T}/{sub}"):
                for f in fn:
                    p = os.path.join(dp, f); key = os.path.relpath(p, f"{T}/{sub}"); key = ("lib:" if sub == "staged-lib" else "") + key
                    if key in man: out.append((pre + os.path.relpath(p, f"{T}/{sub}"), p, man[key]))
    elif bid == "xplat3" or bid.startswith("xplat4-"):
        T = C + ("/xplat3-g-tools" if bid == "xplat3" else f"/xplat4-g-tools/staged/{bid[7:]}")
        S = T + ("/staged/files" if bid == "xplat3" else "/files")
        for line in open(T + "/bases.tsv", encoding="utf8"):
            if line.strip():
                b, rel = line.rstrip("\n").split("\t", 1); out.append((rel, f"{S}/{rel}", None if b == "NEW" else b))
    elif bid == "conv-g":
        S = C + "/conv-g-tools/staged"
        for dp, _, fn in os.walk(S + "/files"):
            for f in fn:
                mine = os.path.join(dp, f); rel = os.path.relpath(mine, S + "/files"); base = f"{S}/base/{rel}"
                out.append((rel, mine, ("file:" + base) if os.path.exists(base) else None))
    elif bid in ("obs-g-fleet", "nxfix-core", "nxfix-nx-patch", "nxlock", "dashlock"):
        T = {"obs-g-fleet": "obs-g-tools/staged/fleet", "nxfix-core": "nxfix-g-tools/staged/core", "nxfix-nx-patch": "nxfix-g-tools/staged/nx-patch", "nxlock": "nxlock-g-tools/staged/nxlock", "dashlock": "dashlock-g-tools/staged/dashlock"}[bid]
        man = json.load(open(f"{C}/{T}/manifest.json", encoding="utf8"))
        for rel, v in man.items():
            if rel.endswith("📓️build-guide.md") and bid == "nxfix-core": continue
            out.append((rel, f"{C}/{T}/files/{v['file']}", v.get("base")))
    return out
def register(bid, basemir):
    home = os.path.join(os.environ.get("TMPDIR", "/tmp"), "sm"); rows = []; d = f"{M}/base0/{bid}"; os.makedirs(d, exist_ok=True); n = 0
    core = {}
    if bid == "nxlock":
        cm = json.load(open(C + "/nxfix-g-tools/staged/core/manifest.json", encoding="utf8")); core = {r: C + "/nxfix-g-tools/staged/core/files/" + v["file"] for r, v in cm.items()}
    for rel, mine, bsha in sources(bid):
        bf = None
        if bsha is not None and bsha.startswith("file:"):
            bf = f"{d}/f{n:03d}"; shutil.copyfile(bsha[5:], bf); n += 1
        elif bsha is not None:
            cands = ([core[rel]] if rel in core else []) + [f"{home}/{basemir}/{rel}"] + ([f"{C}/xplat3-g-tools/staged/files/{rel}"] if bid.startswith("xplat4-") else [])
            src = next((c for c in cands if os.path.exists(c) and sha(rd(c)) == bsha), None)
            if src is None: print("BASE-MISSING", bid, rel[-70:], "(recorded base not found byte-equal: already live or cut from an unlanded batch)"); continue
            bf = f"{d}/f{n:03d}"; shutil.copyfile(src, bf); n += 1
        rows.append({"rel": rel, "mine": mine, "base": bf})
    json.dump({"id": bid, "files": rows}, open(f"{M}/{bid}.json", "w", encoding="utf8"), ensure_ascii=False, indent=1); print("registered", bid, len(rows), "files")
def state(root, row):
    p = f"{root}/{row['rel']}"; mine = rd(row["mine"]); now = rd(p) if os.path.exists(p) else None
    if now is None: return "NEW", mine
    if now == mine: return "APPLIED", mine
    if row["base"] is None: return "EXISTS-OTHER", None
    base = rd(row["base"])
    if now == base: return "UNCHANGED-BASE", mine
    out, rc = merge(mine, base, now)
    return ("MERGE-CLEAN", out) if rc == 0 else (f"MERGE-CONFLICT({rc})", None)
def main(a):
    if a[0] == "rebase":
        bid, suffix, mirror = a[1], a[2], a[a.index("--from-mirror") + 1]; home = os.path.join(os.environ.get("TMPDIR", "/tmp"), "sm"); doc = json.load(open(f"{M}/{bid}.json", encoding="utf8"))
        for r in doc["files"]:
            if r["rel"].endswith(suffix) and r["base"]:
                keep = r["base"] + ".pre-rebase"
                if not os.path.exists(keep): shutil.copyfile(r["base"], keep)
                shutil.copyfile(f"{home}/{mirror}/{r['rel']}", r["base"]); print("base := current", mirror, r["rel"][-60:])
        return 0
    if a[0] == "register": return register(a[1], a[a.index("--base-mirror") + 1] if "--base-mirror" in a else "blbase")
    bid, cmd = a[0], a[1]; root = os.path.abspath(a[a.index("--root") + 1]) if "--root" in a else LIVE
    rows = json.load(open(f"{M}/{bid}.json", encoding="utf8"))["files"]; bk = f"{M}/backup/{bid}@{'live' if root == LIVE else os.path.basename(root)}"
    if cmd == "check":
        sts = [(r, *state(root, r)) for r in rows]
        for r, s, out in sts:
            if s == "MERGE-CLEAN" and (err := parses(r["rel"], out)): print("PARSE-ERROR", r["rel"][-80:], err.replace("\n", " ")[:160]); sts = [(x, ("MERGE-CONFLICT(parse)" if x is r else y), z) for x, y, z in sts]
        for r, s, _ in sts: print("%-16s %s" % (s, r["rel"]))
        bad = [s for _, s, _ in sts if s.startswith(("MERGE-CONFLICT", "EXISTS"))]
        print("conflicts:", len(bad)); return 1 if bad else 0
    if cmd == "apply":
        sts = [(r, *state(root, r)) for r in rows]; bad = [(r, s) for r, s, _ in sts if s.startswith(("MERGE-CONFLICT", "EXISTS"))]
        for r, s, out in sts:
            if out is not None and s != "APPLIED" and (err := parses(r["rel"], out)): bad.append((r, "PARSE-ERROR " + err.replace("\n", " ")[:200]))
        if bad:
            for r, s in bad: print("REFUSED", s, r["rel"])
            return 1
        os.makedirs(bk, exist_ok=True); man = {}
        for i, (r, s, out) in enumerate(sts):
            p = f"{root}/{r['rel']}"
            if s == "APPLIED": continue
            man[r["rel"]] = {"file": None, "after": sha(out)}
            if os.path.exists(p): shutil.copyfile(p, f"{bk}/f{i:03d}"); man[r["rel"]]["file"] = f"f{i:03d}"
            wr(p, out)
        json.dump(man, open(bk + "/manifest.json", "w"), ensure_ascii=False, indent=1); print("applied", len(man), "files (backup", bk + ")"); return 0
    if cmd == "revert":
        man = json.load(open(bk + "/manifest.json", encoding="utf8")); byrel = {r["rel"]: r for r in rows}; n = 0
        for rel, m in man.items():
            p = f"{root}/{rel}"; now = rd(p) if os.path.exists(p) else None
            if now is not None and sha(now) == m["after"]:
                if m["file"] is None: os.remove(p)
                else: wr(p, rd(f"{bk}/{m['file']}"))
                n += 1
            elif now is not None and byrel[rel]["base"] is not None:
                out, rc = merge(rd(byrel[rel]["base"]), rd(byrel[rel]["mine"]), now)
                if rc: print("REVERT-CONFLICT", rel); return 1
                wr(p, out); n += 1
        print("reverted", n, "of", len(man)); return 0
sys.exit(main(sys.argv[1:]))
