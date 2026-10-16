import sys; sys.path.insert(0,'.')
import wrun4
from wapply import *
crates, T, files, P, libtexts, testtexts = wrun4.main4()
OUT = wrun4.wrun3.OUT
for d in TEST_DIRS_OLD: pass
for c in crates:
    shutil.rmtree(f"{LIVE}/🌍️world/{c.folder}", ignore_errors=True)
    shutil.copytree(f"{OUT}/🌍️world/{c.folder}", f"{LIVE}/🌍️world/{c.folder}", symlinks=True)
shutil.copyfile(f"{OUT}/🌍️world/🦀️.rs", f"{LIVE}/🌍️world/🦀️.rs")
print('recopied')
