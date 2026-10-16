"""🌱️ Seeded sensitivity check for the follow-up definition scan: a privatised cache field and scene field must be reported."""
import os, shutil, subprocess, sys
from importlib import util
T = os.path.dirname(os.path.abspath(__file__))
spec = util.spec_from_file_location("defscan", os.path.join(T, "defscan.py"))
d = util.module_from_spec(spec)
sys.argv = [sys.argv[0], "/Users/ueli/Documents/semio"]
spec.loader.exec_module(d)
seed = sys.argv_seed if hasattr(sys, "argv_seed") else os.environ["SEED_ROOT"]
shutil.rmtree(seed, ignore_errors=True)
for rel in d.MODULES:
    os.makedirs(os.path.dirname(os.path.join(seed, rel)), exist_ok=True)
    shutil.copyfile(os.path.join("/Users/ueli/Documents/semio", rel), os.path.join(seed, rel))
for rel, old, new in [
    (d.W + "🎯️targets/🧊️wgpu/🗂️engine-surface-registry/🦀️.rs", "    pub(super) placement_naming: Option<String>,", "    placement_naming: Option<String>,"),
    (d.PLANE, "    pub placement_naming: Option<String>,", "    placement_naming: Option<String>,"),
]:
    path = os.path.join(seed, rel)
    text = open(path, encoding="utf-8").read()
    assert text.count(old) == 1, rel
    open(path, "w", encoding="utf-8").write(text.replace(old, new))
print(subprocess.run([sys.executable, "-I", os.path.join(T, "defscan.py"), seed], capture_output=True, text=True).stdout.splitlines()[-1])
