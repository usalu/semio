#!/usr/bin/env python3
"""📜️ Staged batch `oncecapture` (lane hotpath): the door serializes and hashes its ~19 MB source snapshot once instead of four times. `python3 -I stage.py check|apply|revert [ROOT]`."""
import os, subprocess, sys
here = os.path.dirname(os.path.abspath(__file__))
sys.exit(subprocess.call([sys.executable, "-I", os.path.join(here, "..", "..", "stage.py"), here, *sys.argv[1:]]))
