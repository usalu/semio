path = r"c:\git\semio\semio\py\main.py"
old = """            if heal and h_add is not None:
                first = True
                na = []
                for x in h_add:
                    if x.get("guid") == ag:
                        if first:
                            na.append(x)
                            first = False
                        else:
                        na.append(x)
                h_add = na
"""
new = """            if heal and h_add is not None:
                na = []
                first_kept = False
                for x in h_add:
                    if x.get("guid") == ag:
                        if not first_kept:
                            na.append(x)
                            first_kept = True
                        continue
                    na.append(x)
                h_add = na
"""
text = open(path, encoding="utf-8").read()
if old not in text:
    raise SystemExit("old block not found")
open(path, "w", encoding="utf-8", newline="\n").write(text.replace(old, new, 1))
print("patched")
