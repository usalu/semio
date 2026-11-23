import os, sys, importlib.util

here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("rh", os.path.join(here, "rescue-hunks.py"))
rh = importlib.util.module_from_spec(spec)
spec.loader.exec_module(rh)

NEW = r"C:\semio"
apply = sys.argv[1] == "apply"
log = open(os.path.join(here, "git-semio-rescue-overridden-hunks.txt"), "w", encoding="utf-8")
report = []

for i, (d, p, hs) in sorted(rh.load().items()):
    f = "merged.lf" if i in ("02", "08") else "merged"
    txt = open(os.path.join(d, f), encoding="utf-8", newline="").read().replace("\r\n", "\n")
    out = []
    state = None
    k = 0
    cur = None
    for l in txt.split("\n"):
        if l.startswith("<<<<<<< "):
            cur = {"new": [], "base": [], "old": []}
            state = "new"
            continue
        if cur is not None:
            if l.startswith("||||||| "):
                state = "base"
                continue
            if l.startswith("======="):
                state = "old"
                continue
            if l.startswith(">>>>>>> "):
                kd = rh.kind(cur)
                chosen = cur["new"] if kd == "ws" else cur["old"]
                if kd != "ws" and cur["new"] != cur["old"]:
                    log.write(f"##### {p} hunk {k} kind={kd}: C:\\semio side replaced by old side; C:\\semio text was:\n")
                    log.write("\n".join(cur["new"]) + "\n\n")
                out.extend(chosen)
                k += 1
                cur = None
                continue
            cur[state].append(l)
            continue
        out.append(l)
    res = "\n".join(out)
    assert "<<<<<<< " not in res and ">>>>>>> " not in res
    fn = os.path.join(NEW, p)
    original = open(d + r"\new", "rb").read()
    eol_crlf = original.count(b"\r\n") > original.count(b"\n") / 2
    data = res.replace("\n", "\r\n") if eol_crlf else res
    data = data.encode("utf-8")
    cur_bytes = open(fn, "rb").read()
    if cur_bytes != original:
        report.append(("CHANGED-UNDER-US", p))
        continue
    if apply:
        open(fn, "wb").write(data)
    report.append(("written" if apply else "dry", p, k, "crlf" if eol_crlf else "lf"))

for r in report:
    print(*[str(x).encode("ascii", "replace").decode() if isinstance(x, str) else x for x in r][:4])
