import json,sys
def sender(case):
    ended=case["ended"]
    if ended is None:return "none"
    near=lambda at:abs(case["endedMs"]-at)<=60000
    for row in case.get("sent",[]):
        if row["reason"]=="deadline" and near(row["atMs"]):return "deadline"
    by=case.get("stopBy")
    if by and near(by["atMs"]):return "supervisor"
    warden=case.get("warden")
    if warden and near(warden["atMs"]):return "warden"
    for row in case.get("kills",[]):
        if row["pid"]==case["stagePid"] and near(row["atMs"]):return "runner"
    return "outside-wrapper" if case.get("received") else "outside-child"
def clock(case):return max(case["startedMs"],case["observedMs"])
if __name__=="__main__":
    mode=sys.argv[1];data=json.loads(sys.argv[2])
    print(json.dumps([sender(row) if mode=="sender" else clock(row) for row in data]))
