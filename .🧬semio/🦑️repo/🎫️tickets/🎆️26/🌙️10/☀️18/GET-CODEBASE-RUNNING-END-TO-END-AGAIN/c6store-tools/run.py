"""▶️ C6STORE one-shot module-split check. usage: run.py stage|live <tag>... ; stage: stage + lossless + vmod + defscan + ambig (brief output); live: the same checks on the installed tree."""
import subprocess, sys, os
HERE = os.path.dirname(os.path.abspath(__file__))
mode = sys.argv[1]
def sh(*a):
    p = subprocess.run(["python3"] + list(a), capture_output=True, text=True, cwd=HERE)
    return p.returncode, p.stdout + p.stderr
for tag in sys.argv[2:]:
    steps = []
    if mode == "stage":
        rc, out = sh("modsplit.py", "stage", tag)
        print(out.strip()[-1500:])
        if rc: continue
    ft = dict(l.rstrip("\n").split("\t") for l in open(HERE + "/files.tsv", encoding="utf-8"))
    stagedir = "/Users/ueli/.cache/c6store/stage/" + tag if mode == "stage" else os.path.dirname("/Users/ueli/Documents/semio/" + ft[tag])
    rc, out = sh("modsplit.py", "lossless", tag, *( [] if mode == "stage" else [stagedir] ))
    lines = out.split("\n")
    print("lossless:", [l for l in lines if l.startswith(("missing", "extra", "LOSSLESS", "regions"))])
    for l in lines:
        if "UNEXPLAINED" in l: print("   ", l[:200])
    rc, out = sh("vmod.py", mode, tag)
    print(out.strip()[-1200:])
    rc, out = sh("defscan.py", tag, *( [] if mode == "stage" else [] ))
    print(out.strip()[-800:])
    rc, out = sh("ambig.py", mode, tag)
    print(out.strip()[-600:])
