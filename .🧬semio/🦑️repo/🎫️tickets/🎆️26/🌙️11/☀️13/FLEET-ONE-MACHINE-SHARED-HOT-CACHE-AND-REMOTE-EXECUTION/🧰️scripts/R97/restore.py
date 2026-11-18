import os, shutil
R = "/Users/ueli/Documents/semio/"
BK = "/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R97/backup/"
for root, _, files in os.walk(BK):
    for f in files:
        b = os.path.join(root, f); live = R + b[len(BK):]
        shutil.copy(b, live + ".r97tmp"); os.replace(live + ".r97tmp", live)
for d in ("🪪️graph-state/📦️project-records/🚚️entry-lane/🧪️tests/🌊️streamed-intake", "🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/🧮️admission"):
    p = R + "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/" + d
    if os.path.isdir(p) and os.path.realpath(p).startswith(R): shutil.rmtree(p)
print("restored")
