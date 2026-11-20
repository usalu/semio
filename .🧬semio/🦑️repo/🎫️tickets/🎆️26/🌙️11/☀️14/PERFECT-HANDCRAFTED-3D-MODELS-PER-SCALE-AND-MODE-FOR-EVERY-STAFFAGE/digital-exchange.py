"""🌡️ Physical exchange faces and supports at native and digital scales."""
from pathlib import Path
from importlib.util import spec_from_file_location,module_from_spec
import json
import re
import sys
spec=spec_from_file_location("models",Path(__file__).with_name("digital-models.py"))
models=module_from_spec(spec)
spec.loader.exec_module(models)
from copy import deepcopy
C=models.CATALOG
values={
    "ceiling-diffuser":{"plate":1.64,"collar":.088},
    "gas-meter":{"shell":.143},
    "kitchen-extractor-hood":{"canopy":1.14,"chimney":.96},
    "pendant-lamp":{"lampholder":.013,"shade":.22,"rose":.022},
    "thermostatic-radiator-valve":{"head":.0187},
    "tower-server":{"feet":.011},
    "toaster":{"top-plate":.077,"control":.0053,"feet":.020},
    "street-lamp":{"lamp-housing":.36},
    "drinking-fountain":{"basin-steel":.40},
    "height-adjustable-desk":{"steel-desk":2.44,"monitor-housing":.56},
    "shower-head":{"hose":.052},
    "shopping-trolley":{"frame-tube":.48,"wheels":.14},
    "playground-swing":{"chain-steel":.34,"seat-rubber":.173},
    "tealight":{"wax":.0011}
}

def replace_area(p,values):
    raw=p.read_text()
    pos=raw.index('"regions"',raw.index('"thermal": {',raw.index('"clips":')))
    pos=raw.index('[',pos)
    regions,end=json.JSONDecoder().raw_decode(raw[pos:])
    if not isinstance(regions,list) or not all('areaM2' in r for r in regions):
        raise ValueError((p,"wrong thermal section"))
    text=raw[pos:pos+end]
    for region,value in values.items():
        i=text.index('"id": "'+region+'"');m=re.search(r'"areaM2": ([-+0-9Ee.]+)',text[i:])
        text=text[:i+m.start(1)]+str(value)+text[i+m.end(1):]
    new=raw[:pos]+text+raw[pos+end:]
    expected=json.loads(raw)
    for r in expected['asset']['thermal']['regions']:
        if r['id'] in values:r['areaM2']=values[r['id']]
    assert expected==json.loads(new) and raw==p.read_text()
    p.write_text(new)

def derived(src,shape):
    return {**{k:v for k,v in src.items() if k in models.IDENTITY},**shape}

def set_geometry(a,v,list_name,id,shape):
    items=v[list_name]
    i=next(i for i,s in enumerate(items) if s['id']==id)
    s=items[i]
    if 'of'in s:s={**next(b for b in v['body'] if b['id']==s['of']),**s}
    items[i]=derived(s,shape)
    return items[i]

for p in sorted(C.glob('*/*/🔣️.json')):
    a=json.loads(p.read_bytes())['asset'];id=a['id']
    if sys.argv[1:] and id not in sys.argv[1:]:continue
    if id in values:replace_area(p,values[id])
    if id=="chalkboard":
        raw=p.read_text()
        rep=a["representations"]["500"]
        tray=next(s for s in rep["geometry"]["elevation"] if s["id"]=="tray")
        if tray["points"]==[[-2,0],[2,0]]:
            start=raw.index('"500"',raw.index('"representations"'))
            pos=raw.index('"points"',raw.index('"id": "tray"',start))
            pos=raw.index('[',pos)
            old,length=json.JSONDecoder().raw_decode(raw[pos:])
            assert old==tray["points"] and raw==p.read_text()
            p.write_text(raw[:pos]+json.dumps([[-1.955,0],[1.955,0]])+raw[pos+length:])
    if id=="gas-hob":
        raw=p.read_text()
        anchor=raw.index('"id": "flame"',raw.index('"regions": [',raw.index('"thermal": {',raw.index('"clips":'))))
        pos=raw.index('"areaM2":',anchor); end=raw.index(',',pos)
        if 'areaBasis' not in next(r for r in a['thermal']['regions'] if r['id']=='flame'):
            assert raw==p.read_text()
            p.write_text(raw[:end+1]+'\n          "areaBasis": "exchange",'+raw[end+1:])
    for key in a['volumes']:
        v=deepcopy(a['volumes'][key]);d=int(key)
        if id=="duct-fan" and d==100:
            strut=next(s for s in v["body"] if s["id"]=="strut")
            strut["size"][0]=.078
            strut["transform"]["translation"][0]=.0585
        elif id=="circulation-pump" and d==500:
            set_geometry(a,v,"body","motor-head",models.prism("xy",-67,8,models.disc(42,8)))
            set_geometry(a,v,"body","pipe",models.prism("yz",-6,214,[(y,35+z) for y,z in models.disc(20,6)],[[(y,35+z) for y,z in models.disc(13,6)]]))
            set_geometry(a,v,"body","casing",models.prism("xy",8,67,[(105.5+x,-3+y) for x,y in models.disc(48,8)]))
        elif id=="electric-fan-heater" and d==500:
            out=next(s for s in v["thermal"] if s["id"]=="th-casing")
            out["profile"]=[models.mm(p) for p in [(232,-114),(38,-114),(16,-92),(16,92),(38,114),(232,114)]]
        elif id=="gas-boiler" and d==500:
            set_geometry(a,v,"structure","mass-casing",models.prism("zx",385,455,models.disc(35,4)))
        elif id=="ceiling-diffuser" and d>=100:
            if d==100:
                plate=models.prism("zx",0,.8,models.rect(-287,-287,287,287),[models.disc(124.5,16)])
            else:
                plate=models.prism("zx",0,.8,models.rect(-287,-287,287,287),[models.disc(124.5,8)])
            thermal=v['thermal'];thermal[:]=[s for s in thermal if s['id']!='th-plenum-back']
            guide=next(s for s in v['body'] if s['id']=='flange')
            thermal.append({**derived(guide,plate),"id":"th-plenum-back","region":"plate","layer":"sheet"})
            thermal[-1].pop("sectionFaces",None)
            middle=models.prism("zx",-11,-10.2,models.rect(-235,-235,235,235),[models.rect(-135,-135,135,135)])
            if d==100: middle["profile"]=[models.mm(p) for p in models.octagon(235,235,4)]
            thermal[:]=[s for s in thermal if s["id"]!="th-discharge-baffle"]
            thermal.append({**derived(guide,middle),"id":"th-discharge-baffle","region":"plate","layer":"sheet"})
            thermal[-1].pop("sectionFaces",None)
            if d!=500:
                frame=models.prism("zx",-12,0,models.rect(-297.5,-297.5,297.5,297.5),[models.rect(-262,-262,262,262)])
                set_geometry(a,v,"thermal","th-flange",frame)
            if d==500:
                next(s for s in thermal if s["id"]=="th-cone").pop("sectionFaces",None)
            if d==500:
                set_geometry(a,v,'thermal','th-cone',models.prism("zx",-70,-13,models.rect(-252,-252,252,252))).update(taperTo=[.54,1])
                next(s for s in thermal if s["id"]=="th-cone").pop("sectionFaces",None)
                set_geometry(a,v,'thermal','th-collar',models.prism('zx',0,50,models.disc(124.5,10),[models.disc(123,10)]))
        elif id=="pendant-lamp" and d==500:
            set_geometry(a,v,'thermal','th-shade',models.turn(models.X,(0,0,0),[(-30,0),(-30,19),(4,41),(45,92),(139,133),(139,131),(46,90),(7,39),(0,0)]))
        elif id=="tealight" and d==50:
            next(s for s in v["thermal"] if s["id"]=="th-wax")["sectionFaces"]=["+u","-u","+w","-w","-v"]
        elif id=="tealight" and d in (100,200,500):
            plume=next(s for s in v['thermal'] if s['id']=='th-flame-plume')
            plume['profile']=[models.mm(p) for p in models.disc(10,{100:16,200:12,500:8}[d])]
        elif id=="shower-head" and d in (50,100,200):
            plume=next(s for s in v['thermal'] if s['id']=='th-spray')
            start=plume['sections'][0]['at']
            film=derived(plume,models.prism("xy",-0.4,0.4,models.disc(48,{50:20,100:14,200:10}[d])))
            film.update(id="spray-film",transform={"rotation":[0.4003,0.6165,0,0.6781],"translation":start})
            q=film['transform']['rotation'];film['transform']['rotation']=[x/sum(t*t for t in q)**.5 for x in q]
            film.pop('tags',None)
            v['thermal']=[s for s in v['thermal'] if s['id']!="spray-film"]+[film]
        elif id=="toaster" and d==100:
            next(s for s in v["thermal"] if s["id"]=="th-pan")["sectionFaces"]=["+v"]
            next(s for s in v["thermal"] if s["id"]=="th-top-plate").pop("sectionFaces",None)
        elif id=="tower-server" and d==100:
            for s in list(v["thermal"]):
                if s["region"]!="feet":continue
                foot=next(b for b in v["body"] if b["id"]==s["id"].removeprefix("th-"))
                set_geometry(a,v,"thermal",s["id"],{**models.box([40,12,30],[0,0,0]),"cornerRadius":.003,"transform":foot["transform"]})
        elif id=="drinking-fountain" and d>=100:
            jet=next(s for s in v["body"] if s["id"]=="jet")
            sections=deepcopy(jet["sections"])
            for i,sec in enumerate(sections):
                sec["rx"]=sec["ry"]=.005+.001*i/(len(sections)-1)
            set_geometry(a,v,"thermal","th-jet",dict(kind="loft",sections=sections,capStart=True,capEnd=True))
        elif id=="stepladder" and d==500:
            for s in list(v["thermal"]):
                if "rail-" in s["id"]:
                    rail=next(b for b in v["body"] if b["id"]==s["id"].removeprefix("th-"))
                    w=30 if "front" in s["id"] else 25
                    shape={k:rail[k] for k in ("kind","plane","from","to","transform")}
                    set_geometry(a,v,"thermal",s["id"],{**shape,"profile":[models.mm(pt) for pt in models.rect(-w,-22.5,w,22.5)]}).update(layer="wall")
                elif s["id"].startswith("th-spreader"):
                    s["profile"]=[models.mm(pt) for pt in models.disc(12,6)]
                    s["layer"]="strap"
        elif id=="playground-swing" and d in (50,100):
            for member in v["structure"]:
                if member["id"].startswith(("member-leg-","member-brace-","member-beam")) and member.get("kind")=="revolve":
                    profile=member["profile"]
                    if profile[0]!=profile[-1]:profile.append(profile[0])
            if d==50:
                for dest,asset in [(models.TICKET/"volumes"/a["category"]/(id+".json"),False),(p,True)]:
                    models.replace_scale(dest,d,v,asset)
                continue
            for side,z in (("left",200),("right",-200)):
                name="th-chain-"+side
                profile=[(-21,-5),(-17,-12),(17,-12),(21,-5),(21,5),(17,12),(-17,12),(-21,5)]
                ring=models.prism("xy",z-4,z+4,profile,[models.rect(-13,-4,13,4)])
                solid=set_geometry(a,v,"thermal",name,ring)
                solid.update(transform={"translation":[.022,0,0]},repeat={"count":39,"translation":[.0494,0,0]},layer="link")
                cross={**deepcopy(solid),**models.prism("zx",-4,4,[(z+y,x) for x,y in profile],[[(z+y,x) for x,y in models.rect(-13,-4,13,4)]])}
                cross.update(id=name+"-cross",transform={"translation":[.0467,0,0]},repeat={"count":38,"translation":[.0494,0,0]})
                v["thermal"]=[item for item in v["thermal"] if item["id"]!=cross["id"]]+[cross]
        elif id=="height-adjustable-desk" and d!=50:
            v["thermal"]=[{"id":s["id"],"of":"top-board","region":"top-board"} if s["id"]=="th-top-board" else s for s in v["thermal"]]
        elif id=="height-adjustable-desk":
            for solid in v["thermal"]:
                if solid["region"]=="top-board" and "repeat" in solid:
                    solid.pop("repeat")
                    solid["size"]=[1.6,.025,.8]
                    solid["transform"]={"translation":[.55,.0125,0]}
        elif id=="operating-table":
            lens=next(s for s in v["thermal"] if s["region"]=="light-lens")
            lens["sectionFaces"]=["+v"]
            if d in (200,500):
                for name in ("mass-top","mass-base","mass-column"):
                    m=next(s for s in v["structure"] if s["id"]==name)
                    if m["kind"]=="ellipsoid":
                        r=m["radii"];o=m["transform"]["translation"]
                        outline=[(o[2]*1000+z*r[2]*1000,o[0]*1000+x*r[0]*1000) for z,x in models.disc(1,8 if d==200 else 6)]
                        set_geometry(a,v,"structure",name,models.prism("zx",(o[1]-r[1])*1000,(o[1]+r[1])*1000,outline))
        elif id=="bookshelf":
            next(s for s in v["thermal"] if s["region"]=="plinth")["sectionFaces"]=["+v","-v"]
        elif id=="cooking-pot":
            water=next(s for s in v["thermal"] if s.get("of")=="water")
            set_geometry(a,v,"thermal",water["id"],models.prism("zx",49.75,50,models.disc(80,{50:24,100:20,200:12,500:8}[d]))).update(sectionFaces=["-v","+u","-u","+w","-w"])
            if d==500:
                set_geometry(a,v,"body","wall",models.turn(models.Y,(0,0,0),[(0,0),(0,85),(100,86),(100,85.2),(5,84.2),(5,0)]))
        else:continue
        for dest,asset in [(models.TICKET/'volumes'/a['category']/(id+'.json'),False),(p,True)]:
            models.replace_scale(dest,d,v,asset)
