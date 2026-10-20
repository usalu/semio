"""↩️ Restores a file from its snapshot and removes the child folders a split created. usage: revert.py <snapshot> <file> <childDir>..."""
import shutil, sys, os
snap, target, *children = sys.argv[1:]
shutil.copyfile(snap, target)
for c in children:
    shutil.rmtree(os.path.join(os.path.dirname(target), c), ignore_errors=True)
