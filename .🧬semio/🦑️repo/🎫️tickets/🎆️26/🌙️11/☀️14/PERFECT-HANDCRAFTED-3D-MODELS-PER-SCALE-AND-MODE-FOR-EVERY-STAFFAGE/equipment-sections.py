"""📐 Reconcile measured equipment surfaces and the bicycle frame's lumped mass with the authored geometry."""
import json
import re
from pathlib import Path

ticket=Path(__file__).resolve().parent
catalog=next((ticket.parents[6]/"🎓️teaching").glob("*architecture/*staffage/*catalog"))
decoder=json.JSONDecoder()


def locate(text, at, path):
    if not path:
        return at, at+decoder.raw_decode(text[at:])[1]
    is_object=text[at]=="{"
    position=at+1
    index=0
    while True:
        position=re.compile(r"\s*").match(text,position).end()
        if is_object:
            key, length=decoder.raw_decode(text[position:])
            position=re.compile(r"\s*:\s*").match(text,position+length).end()
        else:
            key=index
        if key==path[0]:
            return locate(text,position,path[1:])
        position+=decoder.raw_decode(text[position:])[1]
        separator=re.compile(r"\s*,\s*").match(text,position)
        if separator is None:
            raise KeyError(path)
        position=separator.end()
        index+=1


def write(path, edits):
    text=path.read_text()
    spans=[]
    for address,value in edits:
        a,b=locate(text,0,address)
        old=text[a:b]
        if json.loads(old)==value:
            continue
        if "\n" in old:
            indentation=len(old[old.rfind("\n")+1:])-1
            indent=len(re.search(r"\n( +)",old)[1])-indentation
            new=("\n"+" "*indentation).join(json.dumps(value,ensure_ascii=False,indent=indent).splitlines())
        else:
            new=json.dumps(value,ensure_ascii=False)
        spans.append((a,b,new))
    for a,b,new in sorted(spans,reverse=True):
        text=text[:a]+new+text[b:]
    if spans:
        path.write_text(text)


def document(name):
    file=next(catalog.glob("*/"+name+"/*.json"))
    return file,json.loads(file.read_text())["asset"]


if __name__ == "__main__":
    path,asset=document("expansion-valve")
    eds=[]
    for i,r in enumerate(asset["thermal"]["regions"]):
        changes={"body":(.0055,-.66),"head":(.0034,-.39),"capillary":(.0074,None)}
        if r["id"] in changes:
            area,power=changes[r["id"]]
            at=["asset","thermal","regions",i]
            eds.append((at+["areaM2"],area))
            if power is not None:
                eds.append((at+["operating","heatW"],power))
    write(path,eds)

    path,asset=document("panel-radiator")
    eds=[]
    areas={"panel-rear":1.41,"panel-front-lower":.44,"panel-front-mid":.44,"panel-front-upper":.44}
    for i,r in enumerate(asset["thermal"]["regions"]):
        if r["id"] in areas:
            eds.append((["asset","thermal","regions",i,"areaM2"],areas[r["id"]]))
    for i,pose in enumerate(asset["poses"]):
        for name,area in areas.items():
            projected=.6 if name=="panel-rear" else .2
            eds.append((["asset","poses",i,"thermal","regions",name,"exposedFraction"],round(projected/area,6)))
    notes=asset["thermal"]["assumptions"]
    extra={"en":"The pressed plates include both corrugated faces in their surface area. The rear plate also includes the two side covers. The room exposure represents the outward face of each water panel. The inward faces exchange heat with the convector channels.", "de":"Die Oberfläche der Pressplatten umfasst beide profilierte Seiten. Zur Rückplatte gehören auch die beiden Seitenbleche. Der Raumanteil entspricht jeweils der nach außen gewandten Wasserplatte. Die inneren Flächen stehen mit den Konvektorkanälen im Wärmeaustausch."}
    if extra not in notes:
        eds.append((["asset","thermal","assumptions"],notes+[extra]))
    write(path,eds)

    path,asset=document("city-bicycle")
    for file,prefix in [(path,["asset","volumes"]),(ticket/"volumes/vehicles/city-bicycle.json",["volumes"])]:
        eds=[]
        for k in ("50","100","200"):
            volume=asset["volumes"][k]
            index=next(i for i,s in enumerate(volume["structure"]) if s["id"]=="mass-frame")
            remaining=[s.copy() for s in volume["structure"] if s["id"] != "mass-frame-stays"]
            centre=remaining[index].copy()
            projection_radius={"50":.005,"100":.0075,"200":.015}[k]
            centre.update(radii=[2.2*projection_radius]*3,transform={"translation":[-.21838,.593,0]})
            remaining[index]=centre
            stays={**centre,"id":"mass-frame-stays","radii":[projection_radius]*3,
                   "transform":{"translation":[-.42585,.593,.04127]},
                   "repeat":{"count":2,"translation":[0,0,-.08254]}}
            remaining.append(stays)
            eds.append((prefix+[k,"structure"],remaining))
        write(file,eds)

    path,asset=document("heat-pump-outdoor-unit")
    for file,prefix in [(path,["asset","volumes"]),(ticket/"volumes/machinery/heat-pump-outdoor-unit.json",["volumes"])]:
        eds=[]
        for k in ("50","100"):
            volume=asset["volumes"][k]
            shell={"id":"th-compressor-envelope","kind":"box","bone":"unit","material":"powder-coat","role":"body","tone":.5,
                   "size":[.43,1.275,.3075],"transform":{"translation":[0,.7875,.39625]},"region":"compressor-casing",
                   "sectionFaces":["-v","+v","-w"]}
            if k=="50":
                shell.pop("size")
                shell.pop("transform")
                shell.update(kind="extrude",plane="zx",profile=[[.2425,-.215],[.548,-.215],[.55,-.213],[.55,.213],[.548,.215],[.2425,.215]],**{"from":.15,"to":1.425})
            remaining=[s for s in volume["thermal"] if s["id"] not in ("th-column-panel","th-column-back","th-service-panel","th-compressor-envelope")]
            remaining.append(shell)
            eds.append((prefix+[k,"thermal"],remaining))
            notes=[d for d in volume["decisions"] if not d["en"].startswith("The compressor compartment")]
            notes.append({"en":"The compressor compartment exchanges heat through its front, back and service cover. The envelope meets the base tray, roof and central partition on the three remaining faces.", "de":"Das Verdichterabteil gibt Wärme über Front, Rückseite und Wartungsdeckel ab. An den drei übrigen Flächen schließen Bodenwanne, Dach und Mitteltrennwand an."})
            eds.append((prefix+[k,"decisions"],notes))
        write(file,eds)

    path,asset=document("cargo-bike")
    for file,prefix in [(path,["asset","volumes"]),(ticket/"volumes/vehicles/cargo-bike.json",["volumes"])]:
        eds=[]
        for k in ("50","100","200","500"):
            remaining=[s.copy() for s in asset["volumes"][k]["structure"] if s["id"]!="mass-cargo-beam"]
            marker=next(s for s in remaining if s["id"]=="mass-frame")
            projection_radius={"50":.01165,"100":.01165,"200":.015,"500":.0375}[k]
            marker.update(radii=[1.5*projection_radius]*3,transform={"translation":[.63801,.51811,0]})
            remaining.append({**marker,"id":"mass-cargo-beam","radii":[projection_radius]*3,"transform":{"translation":[.9707,.255,0]}})
            eds.append((prefix+[k,"structure"],remaining))
        write(file,eds)

