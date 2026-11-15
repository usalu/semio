import sys
exec(open("author-20-2d.py").read())
d = Doc("🪑️objects", "retractable-bollard")
for sc in ("50", "100", "200", "500"):
    for view in ("elevation", "front"):
        d.set(sc, view, "sleeve", closed=True)
d.save()
