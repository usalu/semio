"""🔬 Resolution differential for the register-operators split (reuses the c6fin2 oracle5 machinery): sandbox A has the original register-operators file and no new child folders, sandbox B is the live tree; same patched rustc --emit=metadata, error and warning multisets compared."""
import json, os, shutil, sys
T = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/c6fin2-tools"
sys.path.insert(0, T)
import oracle5 as O
HERE = os.path.dirname(os.path.abspath(__file__))
spec = json.load(open(HERE + "/rs.json", encoding="utf8"))
BREP = "✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep"
REG = BREP + "/🧾️register-operators/🦀️.rs"
O.SBX = "/Users/ueli/.cache/fnsplit/sbx"
O.OUT = "/Users/ueli/.cache/fnsplit/orc"
O.GROUPS["fb2"] = dict(crate="semio_s_plugin_flow_brep", copy=[BREP], root=BREP + "/🦀️.rs", tags=["fb2"], cfgs=["lib", "test"])
def baseline_into(g, dst):
    p = os.path.join(dst, REG)
    O.under(os.path.dirname(p))
    shutil.copyfile(HERE + "/orig/register-operators.rs", p)
    for c in spec["children"]:
        d = os.path.join(os.path.dirname(p), c["dir"])
        O.under(d)
        shutil.rmtree(d, ignore_errors=True)
O.baseline_into = baseline_into
sys.argv = ["rs-oracle.py", "fb2"] + sys.argv[1:]
O.main()
