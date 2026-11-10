"""Rebuilds R/node_modules as a symlink farm over the live dependency store whose first-party workspace links point into the private root."""
import os, sys, shutil
LIVE, R = "/home/deck/semio", sys.argv[1]
live = os.path.realpath(os.path.join(LIVE, "node_modules"))
target = os.path.join(R, "node_modules")
if os.path.islink(target) or os.path.isfile(target): os.remove(target)
if os.path.isdir(target): shutil.rmtree(target)
os.makedirs(target)
def translate(path):
    real = os.readlink(path)
    return real.replace(LIVE + "/", R + "/", 1) if real.startswith(LIVE + "/") else real
for name in os.listdir(live):
    source = os.path.join(live, name)
    if name == "@semio-tech":
        os.makedirs(os.path.join(target, name))
        for child in os.listdir(source):
            os.symlink(translate(os.path.join(source, child)), os.path.join(target, name, child))
    else:
        os.symlink(source, os.path.join(target, name))
print(len(os.listdir(target)))
