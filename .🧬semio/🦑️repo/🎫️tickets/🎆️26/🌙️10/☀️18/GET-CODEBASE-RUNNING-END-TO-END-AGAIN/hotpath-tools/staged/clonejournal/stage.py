#!/usr/bin/env python3
"""🧬 Staged batch `clonejournal` (lane hotpath): the change journal no longer reports the source of an APFS clone as modified. `python3 -I stage.py check|apply|revert [ROOT]`."""
import os, subprocess, sys
here = os.path.dirname(os.path.abspath(__file__))
sys.exit(subprocess.call([sys.executable, "-I", os.path.join(here, "..", "..", "stage.py"), here, *sys.argv[1:]]))
