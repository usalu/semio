import json
S="🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/"
def edit(f,pairs):
    s=open(S+f+"/🔣️.json",encoding="utf-8",newline="").read()
    for o,n in pairs:
        assert s.count(o)==1,(f,o,s.count(o)); s=s.replace(o,n)
    json.loads(s); open(S+f+"/🔣️.json","w",encoding="utf-8",newline="").write(s)
for f in ("🔌️appliances/circulation-pump","🔌️appliances/gas-boiler","🔌️appliances/hot-water-cylinder"):
    edit(f,[('"ground": 0.2\n','"ground": 0.25\n')])
edit("🏗️machinery/oil-pumpjack",[('"body": 0.22,','"body": 0.25,'),("body pen 0.22 mm","body pen 0.25 mm"),("Körperstift 0,22 mm","Körperstift 0,25 mm")])
print("ok")
