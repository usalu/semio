import os, sys, importlib.util
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("rh", os.path.join(here, "rescue-hunks.py")); rh = importlib.util.module_from_spec(spec); spec.loader.exec_module(rh)
apply = sys.argv[1] == "apply"
rows = []
for i, (d, p, hs) in sorted(rh.load().items()):
    f = "merged.lf" if i in ("02", "08") else "merged"
    txt = open(os.path.join(d, f), encoding="utf-8", newline="").read().replace("\r\n", "\n")
    original = open(d + r"\new", "rb").read()
    crlf = original.count(b"\r\n") > original.count(b"\n") / 2
    def build(policy):
        out = []; cur = None; state = None; k = 0; keep = []
        for l in txt.split("\n"):
            if l.startswith("<<<<<<< "): cur = {"new": [], "base": [], "old": []}; state = "new"; continue
            if cur is not None:
                if l.startswith("||||||| "): state = "base"; continue
                if l.startswith("======="): state = "old"; continue
                if l.startswith(">>>>>>> "):
                    kd = rh.kind(cur)
                    newonly = [x for x in cur["new"] if x.strip() and x not in cur["base"] and x not in cur["old"]]
                    if kd == "ws": ch = cur["new"]
                    elif policy == "old": ch = cur["old"]
                    elif kd == "R": ch = cur["old"]
                    elif newonly: ch = cur["new"]; keep.append((k, newonly))
                    else: ch = cur["old"]
                    out.extend(ch); k += 1; cur = None; continue
                cur[state].append(l); continue
            out.append(l)
        r = "\n".join(out)
        r = r.replace("\n", "\r\n") if crlf else r
        return r.encode("utf-8"), keep, k
    prev, _, _ = build("old")
    now, keep, k = build("safe")
    fn = os.path.join(r"C:\semio", p)
    cur_b = open(fn, "rb").read()
    status = "unchanged-since-my-write" if cur_b == prev else "CHANGED-BY-OTHERS"
    if apply and status.startswith("unchanged") and now != prev: open(fn, "wb").write(now)
    rows.append((i, status, k, len(keep), p))
    if keep:
        with open(os.path.join(here, "git-semio-rescue-open-hunks.txt"), "a", encoding="utf-8") as lg:
            for kk, lines in keep:
                lg.write(f"##### {p} hunk {kk}: kept C:\semio text; old side not applied. C:\semio-only lines:\n" + "\n".join(lines) + "\n\n")
for r in rows: print(r[0], r[1], "hunks", r[2], "restored-to-C:\semio", r[3], r[4].split("/")[-2].encode("ascii","replace").decode()[:24])
