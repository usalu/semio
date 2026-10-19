import json, os, re, subprocess, sys, shutil
ROOT = "/Users/ueli/Documents/semio"
C = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
def run(f):
    p = subprocess.run([C + "/tst3b-tools/t.sh", "./" + f], capture_output=True, cwd=ROOT)
    t = p.stdout.decode("utf-8", "replace")
    num = lambda k: int((re.findall(r"^\s*(\d+) " + k + r"\b", t, re.M) or ["0"])[-1])
    return num("pass"), num("fail"), t
def main():
    mode = sys.argv[1]
    for f in sys.argv[2:]:
        full = os.path.join(ROOT, f)
        bak = full + ".tst3b.bak"
        shutil.copy(full, bak)
        b = run(f)
        subprocess.run(["python3", C + "/tst2-tools/treeify.py", "apply", f], cwd=ROOT, capture_output=True)
        a = run(f)
        if a[1] == 0 and a[0] > 0:
            print("KEEP green", f[-80:], b[:2], "->", a[:2]); os.remove(bak)
        else:
            shutil.move(bak, full); print("REVERT", f[-80:], b[:2], "->", a[:2])
main()
