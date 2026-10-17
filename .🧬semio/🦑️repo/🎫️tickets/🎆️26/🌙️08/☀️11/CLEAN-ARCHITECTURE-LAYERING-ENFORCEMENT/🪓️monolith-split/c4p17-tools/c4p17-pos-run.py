"""▶️ c4p17 launcher of the POS typeck oracle under its own process name (a peer's `pkill -f pos.py` killed the label b2 run with exit 143)."""
import os, runpy, sys
POS = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/pos-tools"
sys.path.insert(0, POS)
os.chdir(POS)
sys.argv = [os.path.join(POS, "pos.py")] + sys.argv[1:]
runpy.run_path(os.path.join(POS, "pos.py"), run_name="__main__")
