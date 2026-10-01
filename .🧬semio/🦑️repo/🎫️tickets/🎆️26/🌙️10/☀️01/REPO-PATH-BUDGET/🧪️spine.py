"""Print byte cost of the editor spine and how many bytes a mode-segment drop saves."""

import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("apply", os.path.join(HERE, [name for name in os.listdir(HERE) if name.endswith("apply.py")][0]))
apply = importlib.util.module_from_spec(spec)
spec.loader.exec_module(apply)

files = [path for path in apply.repo_files() if not apply.skipped_tree(path.split("/")) and apply.nbytes(path) > apply.BUDGET]
print("over", len(files))

def drop_modes(path):
    parts = path.split("/")
    out = []
    skip = False
    for part in parts:
        if skip:
            skip = False
            continue
        if apply.ascii_tail(part) == "modes":
            skip = True
            continue
        out.append(part)
    return "/".join(out)

cleared = sum(apply.nbytes(drop_modes(path)) <= apply.BUDGET for path in files)
print("drop modes+name clears", cleared, "of", len(files), "still", len(files) - cleared)

# also drop windows+name
def drop_windows(path):
    parts = drop_modes(path).split("/")
    out = []
    skip = False
    for part in parts:
        if skip:
            skip = False
            continue
        if apply.ascii_tail(part) == "windows":
            skip = True
            continue
        out.append(part)
    return "/".join(out)

cleared_w = sum(apply.nbytes(drop_windows(path)) <= apply.BUDGET for path in files)
print("drop modes and windows clears", cleared_w, "still", len(files) - cleared_w)

# segment sizes on one editor path
sample = next(path for path in files if "/".join(path.split("/")).count("editor") or apply.ascii_tail(path.split("/")[9] if len(path.split("/"))>9 else "") == "editor")
longest = max(files, key=apply.nbytes)
print("longest", apply.nbytes(longest))
for index, part in enumerate(longest.split("/")):
    print(f"{index:2} {apply.nbytes(part):3} {apply.ascii_tail(part)}")
print("--- sample", apply.nbytes(sample))
for index, part in enumerate(sample.split("/")):
    print(f"{index:2} {apply.nbytes(part):3} {apply.ascii_tail(part)}")
