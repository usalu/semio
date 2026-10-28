#!/usr/bin/env python3
"""🍰️ Staged batch `memoslice` (lane hotpath): the graph memo of a request classifies the facts the planner verifies for its slice, not the whole generation. `python3 -I stage.py check|apply|revert [ROOT]`."""
import os, subprocess, sys
here = os.path.dirname(os.path.abspath(__file__))
sys.exit(subprocess.call([sys.executable, "-I", os.path.join(here, "..", "..", "stage.py"), here, *sys.argv[1:]]))
