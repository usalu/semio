import json,os
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/60403654-b0e9-4c0d-a7d3-fba5340a3de9/scratchpad/R226"
p="🧰️framework/🔨️modules/🏃️process/🔒️leases/🧭️order/🧪️tests/🧫️fixtures/🔣️.json"
d=json.loads(open(f"{S}/base/{p}",encoding="utf8").read()) if os.path.exists(f"{S}/base/{p}") else json.loads(open("/Users/ueli/Documents/semio/"+p,encoding="utf8").read())
wait={"boundMilliseconds":60000,"maximumMilliseconds":600000,"heartbeatMilliseconds":10000,"silentMilliseconds":300000,"hint":{"en":"Run the command again once the holder has finished.","de":"Den Befehl erneut ausführen, sobald der Halter fertig ist."}}
for c in d["policy"]["classes"]: c["wait"]=wait
d["policy"]["unclassifiedWait"]=wait
os.makedirs(os.path.dirname(f"{S}/new/{p}"),exist_ok=True)
open(f"{S}/new/{p}","w",encoding="utf8").write(json.dumps(d,indent=1,ensure_ascii=False)+"\n")
