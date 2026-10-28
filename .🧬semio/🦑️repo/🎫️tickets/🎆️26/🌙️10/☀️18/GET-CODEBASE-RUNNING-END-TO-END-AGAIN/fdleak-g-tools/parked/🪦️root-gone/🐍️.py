"""🐍️ Independent model of when a change journal service must stop (stdlib only). usage: python3 -I 🐍️.py <idleMs> <maxBytes> then JSON rows on stdin; prints one reason (or null) per row. Order: workspace or journal directory gone, owner gone, idle, memory."""
import json,sys
idle,maximum=int(sys.argv[1]),int(sys.argv[2])
def reason(f):
    if not f["rootExists"] or not f["directoryExists"]: return "root-gone"
    if f.get("ownerAlive") is False: return "owner-gone"
    if f["idleMilliseconds"]>idle: return "idle"
    if f["residentBytes"]>maximum: return "memory"
    return None
print(json.dumps([reason(row) for row in json.load(sys.stdin)]))
