import json,sys
d=json.load(open(f"volumes/animals/{sys.argv[1]}.json"))["volumes"]
for sc,v in d.items():
    if v.get("status")=="authored": print(sc, len(v["body"]), len(v["thermal"]), len(v["structure"]))
    else: print(sc,"unavailable")
