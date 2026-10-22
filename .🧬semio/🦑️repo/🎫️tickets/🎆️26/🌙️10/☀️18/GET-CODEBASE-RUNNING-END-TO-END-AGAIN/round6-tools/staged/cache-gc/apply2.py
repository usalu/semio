#!/usr/bin/env python3
"""Second staged set (status without walking): same flags as apply.py; dry run is the default, --write applies."""
import os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import apply, patches  # noqa: E402
from paths import J, NEW2  # noqa: E402

patches.STAGED = J(HERE, "staged2")
patches.EDITORS = patches.EDITORS2
apply.NEW_FILES = NEW2
apply.BASES = J(HERE, "bases2.json")
apply.BACKUP = J(HERE, "backup2")
apply.MANIFEST = J(apply.BACKUP, "manifest.json")
raise SystemExit(apply.main(sys.argv[1:]))
