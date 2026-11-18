import glob
B = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
BK = "/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R101/backup1/"
for n in glob.glob(BK + "*.name"):
    k = open(n, encoding="utf8").read(); open(B + k, "w", encoding="utf8").write(open(n[:-5], encoding="utf8").read())
print("restored")
