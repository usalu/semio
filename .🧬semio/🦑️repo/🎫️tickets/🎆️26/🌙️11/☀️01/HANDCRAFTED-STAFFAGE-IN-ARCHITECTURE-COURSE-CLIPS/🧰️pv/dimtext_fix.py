import json
S="🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/"
def edit(f,pairs):
    s=open(S+f,encoding="utf-8",newline="").read()
    for o,n in pairs:
        assert s.count(o)==1,(f,o[:50],s.count(o)); s=s.replace(o,n)
    json.loads(s); open(S+f,"w",encoding="utf-8",newline="").write(s)

edit("🧱️components/tilt-window/🔣️.json",[
 ("drawn in vertical section with its blend-frame sill; conditioned","drawn in vertical section with its blend-frame sill (the 1.25 m section height is the 1.2 m sash plus the sill); conditioned"),
 ("im Vertikalschnitt mit dem Blendrahmen-Sohlstück gezeichnet; konditioniertes","im Vertikalschnitt mit dem Blendrahmen-Sohlstück gezeichnet (die Schnitthöhe von 1,25 m ist der 1,2-m-Flügel plus das Sohlstück); konditioniertes")])
edit("🪑️objects/tealight/🔣️.json",[
 ("and a flame of 22 mm; burning","and a flame of 22 mm (40.5 mm overall height with the flame); burning"),
 ("einer 22 mm hohen Flamme; brennend","einer 22 mm hohen Flamme (40,5 mm Gesamthöhe mit Flamme); brennend")])
print("ok")
