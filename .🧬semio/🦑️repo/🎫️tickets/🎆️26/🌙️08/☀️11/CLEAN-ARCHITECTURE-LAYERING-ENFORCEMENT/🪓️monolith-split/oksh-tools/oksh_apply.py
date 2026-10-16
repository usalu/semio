# 🚀 apply.py under the OS-KERNEL-SQLITE-HOST overrides (imports oksh first so assign/model/plan/carve are patched).
# usage: oksh_apply.py <batch-name> dry|write
import os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import oksh
import apply
apply.main()
