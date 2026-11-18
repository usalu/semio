import os, shutil
R = "/Users/ueli/Documents/semio/"
BK = "/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R103/backup/"
for dp, _, fs in os.walk(BK):
    for f in fs:
        src = os.path.join(dp, f); dest = R + src[len(BK):]
        shutil.copy(src, dest + ".r103tmp"); os.replace(dest + ".r103tmp", dest); print("restored", dest[-50:])
N = R + "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🪂️set-aside"
shutil.rmtree(N, ignore_errors=True)

shutil.rmtree(R + "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/📦️project-records/🧳️portable/🧪️tests/🪶️rest-walk", ignore_errors=True)
