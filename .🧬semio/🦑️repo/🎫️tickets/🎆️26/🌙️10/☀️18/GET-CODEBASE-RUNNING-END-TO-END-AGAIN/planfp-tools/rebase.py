"""🔁 Rebases batch planfp's script.ts onto the live file when peers changed it: copies live 📜️script.ts to staged/base, regenerates staged/files/script.ts with the anchored edit_script.py (exit 1 on a missing anchor) and rewrites manifest.json. Other files: re-stage by hand.
usage: python3 -I rebase.py [--root DIR]"""
import os, shutil, subprocess, sys
here = os.path.dirname(os.path.abspath(__file__))
root = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else "/Users/ueli/Documents/semio"
live = os.path.join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts")
shutil.copyfile(live, os.path.join(here, "staged/base/script.ts")); shutil.copyfile(live, os.path.join(here, "staged/files/script.ts"))
subprocess.run([sys.executable, "-I", os.path.join(here, "edit_script.py"), os.path.join(here, "staged/files/script.ts")], check=True)
subprocess.run([sys.executable, "-I", os.path.join(here, "manifest.py")], check=True)
