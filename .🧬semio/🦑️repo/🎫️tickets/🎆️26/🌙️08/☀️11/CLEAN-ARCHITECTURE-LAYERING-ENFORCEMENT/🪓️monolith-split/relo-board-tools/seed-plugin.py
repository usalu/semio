"""🌾️ Seeded regression for the plugin crate: a private `PUZZLE_2D_BOARD_NAMING` must surface as new E0603 at its cross-module readers."""
import shutil, sys
from importlib import util
spec = util.spec_from_file_location("oracle", __file__.rsplit("/", 1)[0] + "/oracle.py")
oracle = util.module_from_spec(spec)
sys.argv = [sys.argv[0], sys.argv[1]]
spec.loader.exec_module(oracle)
SCR = sys.argv[1]
seed = SCR + "/S3n"
shutil.rmtree(seed, ignore_errors=True)
shutil.copytree(SCR + "/S1n", seed, symlinks=True)
host = seed + "/" + oracle.TREES[1] + "/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🦀️.rs"
text = open(host, encoding="utf-8").read()
assert text.count("pub static PUZZLE_2D_BOARD_NAMING") == 1
open(host, "w", encoding="utf-8").write(text.replace("pub static PUZZLE_2D_BOARD_NAMING", "static PUZZLE_2D_BOARD_NAMING"))
a = oracle.parse(open(SCR + "/oracle-puzzle2d-lib-1.err").read(), SCR + "/S1n")
b = oracle.run(seed, "puzzle2d", [], "seed-plugin-mutant")
for (kind, msg, loc), c in sorted((b - a).items()):
    print(f"  +seed {c}x {kind}: {msg[:160]} @ {loc[-100:]}")
