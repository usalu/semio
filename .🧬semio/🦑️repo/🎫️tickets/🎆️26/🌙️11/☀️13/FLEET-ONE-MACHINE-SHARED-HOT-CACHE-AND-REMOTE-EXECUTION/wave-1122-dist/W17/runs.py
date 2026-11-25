import json,sys,datetime
for o in sys.argv[1:]:
    d=json.load(open(f"/Users/Shared/semio-store/v1/origins/{o}/ready-timings.json"))
    print("==",o)
    for r in d["runs"][-int(__import__('os').environ.get('N','14')):]:
        t=lambda m: datetime.datetime.fromtimestamp(m/1000).strftime('%m-%d %H:%M') if m else '-'
        print(t(r.get("startedMs")),t(r.get("endedMs")),r["stage"],r.get("seconds"),r.get("outcome"),"exit",r.get("exit"),r.get("signal"),"peak",r.get("peakMegabytes"),"growth",r.get("growthMegabytes"),r.get("digest"),r.get("endEvidence"))
