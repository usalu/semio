"""🌱️ Seeded regression for the slice D oracle: a privatised `set_naming` must surface as a new E0624."""
import shutil, sys
from importlib import util
spec = util.spec_from_file_location("oracle", __file__.rsplit("/", 1)[0] + "/oracle.py")
oracle = util.module_from_spec(spec)
sys.argv = [sys.argv[0], sys.argv[1]]
spec.loader.exec_module(oracle)
SCR = sys.argv[1]
seed = SCR + "/S2n"
shutil.rmtree(seed, ignore_errors=True)
shutil.copytree(SCR + "/S1n", seed, symlinks=True)
style = seed + "/" + oracle.TREES[0] + "/🎲️board/🔌️ports/➡️directed/➕️normal/🎨️style/🦀️.rs"
text = open(style, encoding="utf-8").read()
assert text.count("    pub fn set_naming(") == 1
open(style, "w", encoding="utf-8").write(text.replace("    pub fn set_naming(", "    fn set_naming("))
a = oracle.run(SCR + "/S1n", "infinite", ["--test"], "seed-clean")
b = oracle.run(seed, "infinite", ["--test"], "seed-mutant")
for (kind, msg, loc), c in sorted((b - a).items()):
    print(f"  +seed {c}x {kind}: {msg[:160]} @ {loc[-90:]}")
