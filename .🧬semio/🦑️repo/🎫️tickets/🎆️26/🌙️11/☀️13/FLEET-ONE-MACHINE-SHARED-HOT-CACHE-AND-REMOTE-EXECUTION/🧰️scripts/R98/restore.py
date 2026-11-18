import os, shutil
LIB = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/"
BK = "/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R98/backup/"
for k in ("📥️take/🟨️.mjs", "📥️take/🟨️.d.mts", "📥️take/🧬️schema/🔣️.json", "🌐️owner-pool/🧬️schema/🔣️.json", "🟨️.mjs"):
    shutil.copy(BK + k, LIB + k + ".r98tmp"); os.replace(LIB + k + ".r98tmp", LIB + k)
for d in ("📥️take/🧪️tests/🚀️release", "📥️take/🧪️tests/🤝️two-machines"):
    p = LIB + d
    if os.path.isdir(p) and os.path.realpath(p).startswith(LIB): shutil.rmtree(p)
print("restored")
