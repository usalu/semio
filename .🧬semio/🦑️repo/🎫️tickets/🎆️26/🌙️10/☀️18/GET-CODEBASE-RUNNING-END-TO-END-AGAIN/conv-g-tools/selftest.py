import os, shutil, subprocess, sys, tempfile, filecmp
HERE = os.path.dirname(os.path.abspath(__file__)); C = os.path.dirname(HERE); S = HERE + "/staged"; O2 = C + "/oracle2-g-tools/staged"
LIVE = "/Users/ueli/Documents/semio"
work = tempfile.mkdtemp(prefix="cg-selftest-"); root = work + "/root"; env = dict(os.environ, CG_ROOT=root, CG_BACKUP=work + "/backup")
def run(mode):
    r = subprocess.run([sys.executable, "-I", HERE + "/stage.py", mode], env=env, capture_output=True, text=True); return r.returncode, r.stdout + r.stderr
for base, _, names in os.walk(S + "/base"):
    for n in names:
        f = os.path.relpath(os.path.join(base, n), S + "/base"); dst = root + "/" + f; os.makedirs(os.path.dirname(dst), exist_ok=True)
        shutil.copy2(LIVE + "/" + f if os.path.exists(LIVE + "/" + f) else S + "/base/" + f, dst)
code, out = run("check"); assert code == 1 and "PREREQ-ORACLE2" in out, ("prereq must refuse before oracle2-g", out[:400])
code, out = run("apply"); assert code == 1 and "REFUSED" in out, out[:300]
for base, _, names in os.walk(O2 + "/files"):
    for n in names:
        f = os.path.relpath(os.path.join(base, n), O2 + "/files")
        if os.path.exists(S + "/base/" + f) and os.path.exists(root + "/" + f): shutil.copy2(O2 + "/files/" + f, root + "/" + f)
code, out = run("check"); assert code == 0, out
code, out = run("apply"); assert code == 0 and out.strip().endswith("files"), out[-300:]
for base, _, names in os.walk(S + "/files"):
    for n in names:
        f = os.path.relpath(os.path.join(base, n), S + "/files"); assert filecmp.cmp(S + "/files/" + f, root + "/" + f, shallow=False), f
code, out = run("apply"); assert code == 0 and "applied 0 files" in out, out[-200:]
code, out = run("revert"); assert code == 0, out[-300:]
for base, _, names in os.walk(S + "/base"):
    f0 = None
    for n in names:
        f = os.path.relpath(os.path.join(base, n), S + "/base"); assert filecmp.cmp(S + "/base/" + f, root + "/" + f, shallow=False) or os.path.exists(O2 + "/files/" + f), f
shutil.rmtree(work); print("selftest ok")
