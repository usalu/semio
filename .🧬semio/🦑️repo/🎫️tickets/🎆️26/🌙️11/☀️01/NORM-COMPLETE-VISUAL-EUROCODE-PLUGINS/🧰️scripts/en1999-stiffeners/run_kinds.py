"""EN 1999 transverse stiffener kinds: runs the leaf-kinds generator with a private stage (the shared stage belongs to other agents). Usage: python -I run_kinds.py sets|rows [--apply]"""
import os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
LK = os.path.join(os.path.dirname(HERE), "leaf-kinds")
sys.path.insert(0, LK)
import common
TICKET = common.TICKET
STAGE = TICKET + "/🗑️generated/en1999-stiffeners/stage-" + sys.argv[1]
shutil.rmtree(STAGE, ignore_errors=True)
if sys.argv[1] == "sets":
    import en1999 as M
    M.STAGE = STAGE
    M.main()
else:
    import en1999_entity_rows as M
    M.STAGE_DIR = STAGE
    M.main()
