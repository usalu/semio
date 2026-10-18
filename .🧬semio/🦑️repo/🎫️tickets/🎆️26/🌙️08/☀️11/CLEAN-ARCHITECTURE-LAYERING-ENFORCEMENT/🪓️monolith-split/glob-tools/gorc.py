#!/usr/bin/env python3
"""🔬 glob-removal scoped differential: live tree against live tree with the pre-edit backups (manifest.json) restored. usage: gorc.py <c5b|c5c9> <group> [cfg,...]"""
import importlib.util, json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
F = os.path.dirname(HERE)
tools, group = sys.argv[1], sys.argv[2]
spec = importlib.util.spec_from_file_location("o5", f"{F}/{tools}-tools/oracle5.py")
o5 = importlib.util.module_from_spec(spec)
sys.modules["o5"] = o5
sys.path.insert(0, f"{F}/{tools}-tools")
spec.loader.exec_module(o5)
man = json.load(open(HERE + "/manifest.json", encoding="utf-8"))
def baseline_into(g, dst):
    n = 0
    for rel, bk in man.items():
        p = os.path.join(dst, rel)
        if os.path.exists(p):
            o5.under(os.path.dirname(p))
            shutil.copyfile(os.path.join(HERE, bk), p); n += 1
    print("restored", n, "pre-edit files into baseline")
o5.baseline_into = baseline_into
_copy = o5.copy_tree
def copy_tree(g, dst):
    _copy(g, dst)
    sab = os.environ.get("GORC_SABOTAGE")
    if sab and dst.endswith("/new"):
        rel, old, new = sab.split("=>", 2) if sab.count("=>") == 2 else (None, None, None)
        p = os.path.join(dst, rel)
        t = open(p, encoding="utf-8").read()
        assert old in t, "sabotage target missing"
        open(p, "w", encoding="utf-8").write(t.replace(old, new, 1))
        print("SABOTAGED", rel[-40:])
o5.copy_tree = copy_tree
o5.CONFIGS["testkit"] = ["--cfg", 'feature="testkit"']
o5.CONFIGS["testkit_test"] = ["--test", "--cfg", 'feature="testkit"']
o5.CONFIGS["native"] = ["--cfg", 'feature="ureq"', "--cfg", 'feature="sync"']
o5.CONFIGS["native_test"] = ["--test", "--cfg", 'feature="ureq"', "--cfg", 'feature="sync"', "--cfg", 'feature="testkit"']
OD = "🧰️framework/🛍️products/💻️os/🔨️modules"
o5.GROUPS["dgdx"] = dict(crate="semio_framework_os_kernel_dsl_derive_expand", copy=[OD + "/🗣️dsl/✨️derive"], root=OD + "/🗣️dsl/✨️derive/🧩️expand/🦀️.rs", tags=[], cfgs=["lib", "test"])
o5.CONFIGS["aat"] = ["--cfg", 'feature="artifact-app-testing"']
o5.CONFIGS["aat_test"] = ["--test", "--cfg", 'feature="artifact-app-testing"']
if len(sys.argv) < 4:
    sys.argv.append("lib,test,testkit,testkit_test,aat,aat_test")
sys.argv = [sys.argv[0]] + sys.argv[2:]
o5.main()
