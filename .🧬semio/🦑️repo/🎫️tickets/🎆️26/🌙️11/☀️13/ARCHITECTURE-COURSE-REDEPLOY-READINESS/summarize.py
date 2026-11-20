import json,re,sys
for f in sys.argv[1:]:
    s=open(f).read()
    for part in re.split(r'(?<=\n)(?=\{\n "label")',s):
        try: d=json.loads(part)
        except Exception: continue
        procs=[int(m.group(1)) for e in d["events"] for m in [re.search(r"^(pointerdown|click|keydown|pointerup|input|change) delay \d+ proc (\d+)",e)] if m for m in [re.match(r".* proc (\d+)",e)]]
        print(f.split("/")[-1][:22],d["label"],"| events proc ms:",procs,"| long tasks:",d["long"])
