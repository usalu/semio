path = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts"
text = open(path, encoding="utf-8").read()
key = "export const parseEn1993Diff"
i = text.find(key)
j = text.find(";", i)
chunk = text[i:j]
for name in ["placements", "jointPlates", "foundationBlocks", "frameActions", "frameMemberBases", "reliabilityClass"]:
    k = chunk.find(name)
    print(name, chunk[k:k+180] if k >= 0 else "MISSING")
    print("---")
