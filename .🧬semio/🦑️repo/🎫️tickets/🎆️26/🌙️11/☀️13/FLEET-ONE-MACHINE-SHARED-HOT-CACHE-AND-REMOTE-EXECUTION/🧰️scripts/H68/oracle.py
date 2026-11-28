import re
def identity(text):
    def strip(v):
        if isinstance(v,list): return [strip(x) for x in v]
        if isinstance(v,dict): return {k:strip(x) for k,x in v.items() if k!="rendered"}
        return v
    rows=[]
    for line in text.split("\n"):
        if line=="": continue
        try: row=json.dumps(strip(json.loads(line)),ensure_ascii=False,separators=(",",":"))
        except ValueError: row=re.sub("\x1b\\[[0-9;]*m","",line)
        rows.append(re.sub(r"long-type-[0-9]+\.txt","long-type-0.txt",row))
    return "\n".join(sorted(rows))
for case in data.get("streams",[]):
    if (identity(case["kept"])==identity(case["refused"]))!=case["equal"]:
        print("stream mismatch:",case["name"]);sys.exit(1)
print("ok streams",len(data.get("streams",[])))
