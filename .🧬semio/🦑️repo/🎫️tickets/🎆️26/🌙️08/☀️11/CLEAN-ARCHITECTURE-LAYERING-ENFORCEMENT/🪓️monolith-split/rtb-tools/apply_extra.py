"""Second passes of the shell bins: settle-pump joined chrome-6, the tool-run panel law (a former panel-records chunk mount) joins windows-1."""
import glob, sys
sys.argv = ["x", "apply"]
import mk_bins as M
from lib import *
f = glob.glob(M.TD + "*wgpu-tool-run-panel/🦀️.rs")[0]
host = SNAP + "/E/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🧾️panel-records/🦀️.rs"
real_inventory = M.INV.inventory
M.INV.inventory = lambda: dict(real_inventory(), **{f: [dict(kind="mod", name="tool_run_panel_tests", cfg="", host=host, text="")]})
M.run("shell-tests-windows-1", [f])
