"""🩹 Runs the repo remedy-law check `⚖️compliance/🧪️tests/🩹️remedy-law/🐍️.py` (the scanner lives in the repo test)."""
import pathlib, runpy, sys

LAW = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/⚖️compliance/🧪️tests/🩹️remedy-law/🐍️.py")
sys.argv[0] = str(LAW)
runpy.run_path(str(LAW), run_name="__main__")
