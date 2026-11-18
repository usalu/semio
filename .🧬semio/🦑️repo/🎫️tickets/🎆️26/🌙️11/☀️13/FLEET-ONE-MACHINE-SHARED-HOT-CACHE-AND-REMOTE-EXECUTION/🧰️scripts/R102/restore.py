import shutil, os
B = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
SC = "/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R102/backup/"
for name in os.listdir(SC):
    shutil.copy(SC + name, B + name.replace("__", "/"))
    print("restored", name)
