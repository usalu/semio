"""rustfmt --check hunk counts, old snapshot versus live file, per changed hub Rust file (skip_children so each file stands alone)."""
import os, subprocess, sys, re, shutil
sys.path.insert(0, os.path.dirname(__file__))
from hdclib import *
CFG = ROOT + "/rustfmt.toml"
TMP = SC + "/hdc-fmt"
shutil.rmtree(TMP, ignore_errors=True)
os.makedirs(TMP)


def hunks(path):
    r = subprocess.run(["rustfmt", "+nightly-2026-07-20", "--edition", "2021", "--config-path", CFG, "--config", "skip_children=true,unstable_features=true", "--check", path], capture_output=True, text=True)
    err = r.stderr
    if "error:" in err and "Diff in" not in err:
        return -1, err[:200]
    return len(re.findall(r"^Diff in ", r.stdout, re.M)), ""


tot_old = tot_new = 0
worse = []
parse_fail = []
i = 0
for f in hub_files():
    new = os.path.join(ROOT, f)
    o = SC + "/hdc-orig/" + f
    oldtext = read(o) if os.path.exists(o) else None
    if oldtext is not None and oldtext == read(new):
        continue
    i += 1
    hn, en = hunks(new)
    if hn < 0:
        parse_fail.append((f, en)); continue
    if oldtext is None:
        print("new   %3d  %s" % (hn, f)); tot_new += hn; continue
    tmp = os.path.join(TMP, "%d.rs" % i)
    write(tmp, oldtext)
    ho, eo = hunks(tmp)
    tot_old += max(ho, 0); tot_new += hn
    if hn > ho:
        worse.append((f, ho, hn))
print("files", i, "hunks old(changed files) %d new(all) %d" % (tot_old, tot_new))
for f, a, b in worse:
    print("WORSE", a, "->", b, f)
for f, e in parse_fail:
    print("PARSEFAIL", f, e)
