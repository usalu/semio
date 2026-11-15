import sys,json,glob
base=sys.argv[1]
p=glob.glob(base+"/*replica/\U0001F9EC️schema/\U0001F523️.json")[0]
s=open(p,encoding="utf8").read()
def rep(old,new,count=1):
    global s
    assert s.count(old)==count,(old[:80],s.count(old))
    s=s.replace(old,new)
rep('''        "storeRoot": { "type": "string", "minLength": 1 }
      }
    },
    "Operation": {''','''        "storeRoot": { "type": "string", "minLength": 1 },
        "key": { "type": "string", "pattern": "^[0-9a-f]{32}$", "description": "The random identity of this machine for its origin id; the first publish records it in the origin's home.json, and a machine with another key is refused for that origin." }
      }
    },
    "Operation": {''')
rep('''        "group": { "$ref": "#/definitions/GroupId" }
      }''','''        "group": { "$ref": "#/definitions/GroupId" },
        "key": { "type": "string", "pattern": "^[0-9a-f]{32}$", "description": "The key of the machine that owns this origin id (see Machine)." }
      }''')
rep('''        "originId": { "$ref": "#/definitions/OriginId" }
      }
    },
    "SyncSwitch": {''','''        "originId": { "$ref": "#/definitions/OriginId" },
        "bootMs": { "$ref": "#/definitions/Count", "description": "Estimated boot time of the host (wall clock minus uptime) when the lock was taken: a lock from another boot is stale." },
        "hb": { "const": true, "description": "The holder refreshes the modification time of the lock every 5 s; a lock with this marker that has not been refreshed for 60 s is stale although its pid answers." }
      }
    },
    "SyncSwitch": {''')
rep('"restarting", "retired", "error"]','"restarting", "restart-refused", "journal-repaired", "retired", "error"]') if '"restarting", "retired", "error"]' in s else None
open(p,"w",encoding="utf8").write(s)
d=json.loads(s)["definitions"]
print(d["EventName"]); print(d["Status"]["required"])
