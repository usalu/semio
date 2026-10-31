"""🔬 Recheck every assigned structural diagnostic pair on current explicitly owned native geometry."""
import hashlib,importlib,importlib.util,json,math,sys,time,traceback,threading,_thread
from pathlib import Path
sys.path.insert(0,str(Path.cwd()))
A=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
T=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🏗️structure.🔗️joints.🧪️tests.🐍️")
B=Path(__file__).parent;O=B/"🗑️generated/native-r92/structural-ledger";O.mkdir(parents=True,exist_ok=True)
START=time.monotonic();LIMIT=1500
spec=importlib.util.spec_from_file_location("structural_source",B/"structural-joint-source-r95.py");S=importlib.util.module_from_spec(spec);spec.loader.exec_module(S)

def guard():
    """🛑 Bound only this assigned proof and honour a local cooperative cancellation file."""
    if (O/"cancel").exists():raise InterruptedError("Structural ledger recheck cancelled")
    if time.monotonic()-START>LIMIT:raise TimeoutError("Structural ledger 1500 second bound")


def observe(stop):
    """🧵 Observe only this proof with safe GIL-held snapshots and cooperative Python interruption."""
    main=threading.main_thread().ident;sample=time.monotonic()
    while not stop.wait(1):
        if time.monotonic()-sample>60:
            frame=sys._current_frames().get(main)
            if frame:
                with (O/"stack.log").open("a",encoding="utf-8")as out:out.write("[DEBUG] safe structural ledger snapshot"+chr(10));out.writelines(traceback.format_stack(frame))
            sample=time.monotonic()
        try:guard()
        except BaseException:
            (O/"bound.txt").write_text(traceback.format_exc(),encoding="utf-8");_thread.interrupt_main();return


def digest(value):
    """🪪 Bind outputs to the actual source and physical producer identities."""
    return hashlib.sha256(value).hexdigest()


def parts(model,tag):
    """🧱 Resolve actual published native material, never a structural envelope or borrowed reference."""
    obj=A._find(model,tag)
    if obj["class"]=="IfcStair":raw=A._stair_geometry(model,obj)["parts"]
    else:raw=A._interface_owner_parts(model,tag)
    result=[]
    for index,part in enumerate(raw):
        guard()
        for cell_index,cell in enumerate(A._native_material_cells(part)):
            published={**A._native_public_value(cell),"material":part.get("material",A._material(model,obj)["tag"]),"role":part.get("role","owned_material")};native=A._polyhedron_part(A.part_vertices(published),A.part_faces(published));volume=A._polyhedron_volume(native)
            if volume<=0:raise ValueError("Actual owned native material requires positive closed cells")
            material=part.get("material",A._material(model,obj)["tag"])
            result.append({"source_part":index,"source_cell":cell_index,"material":material,"geometry":published,"bounds":A._material_bounds(published)})
    return result


def main():
    """📋 Visit all 48 rows, retaining failures and every strictly positive common-material witness."""
    raw=A._source();m=json.loads(raw);source=digest(raw.encode());producer=str(A._geometry_producers());case=json.loads((B/"structural-joint-source-cases-r95.json").read_text(encoding="utf-8"))
    candidate="--candidate"in sys.argv
    if candidate:S.apply(m,ramps=True)
    rows=case["complete_ledger"];assert len(rows)==48;cache={};errors={};results=[]
    for row in rows:
        guard();tags=[row["source"],row["target"]]
        for tag in tags:
            if tag not in cache and tag not in errors:
                print("[DEBUG] "+json.dumps({"phase":"physical-material","tag":tag,"seconds":time.monotonic()-START}),flush=True)
                try:cache[tag]=parts(m,tag)
                except Exception:errors[tag]=traceback.format_exc()
        result={"source":tags[0],"target":tags[1],"diagnostic_volume":row["diagnostic_volume"],"positive":[],"unresolved":[]}
        if any(tag in errors for tag in tags):result["unresolved"].append({"geometry_errors":{tag:errors[tag]for tag in tags if tag in errors}})
        else:
            for left in cache[tags[0]]:
                for right in cache[tags[1]]:
                    guard()
                    if not all(a[0]<b[1]and b[0]<a[1]for a,b in zip(left["bounds"],right["bounds"])):continue
                    try:
                        common=A._reference_material_parts(left["geometry"],[],bounds=[right["geometry"]],cancel=lambda:(guard()or False))
                        if not common:continue
                        volumes=[A._polyhedron_volume(p)for p in common];total=math.fsum(volumes)
                        if total<=0:raise ValueError("Nonempty exact material intersection lost positive volume")
                        independent=T.volume(common)
                        witness={"source_part":left["source_part"],"source_cell":left["source_cell"],"target_part":right["source_part"],"target_cell":right["source_cell"],"source_material":left["material"],"target_material":right["material"],"volume":total,"independent_geos_volume":independent,"independently_confirmed":math.isclose(total,independent,rel_tol=1e-7,abs_tol=0)}
                        result["positive"].append(witness)
                        if not witness["independently_confirmed"]:result["unresolved"].append({"reason":"Independent actual section integration does not resolve the exact positive witness","witness":witness})
                    except Exception:result["unresolved"].append({"source_part":left["source_part"],"target_part":right["source_part"],"error":traceback.format_exc()})
        result["clear"]=not result["positive"]and not result["unresolved"];results.append(result)
        output={"source":source,"candidate":candidate,"scope":"All 48 assigned current physical product pairs; no small-volume waiver, no publication certificate","complete":len(results)==48,"rows":results,"geometry_errors":errors,"products":len(cache),"seconds":time.monotonic()-START,"stable_source":source==digest(A._source().encode()),"stable_producers":producer==str(A._geometry_producers())}
        (O/"result.json").write_text(json.dumps(output,indent=2),encoding="utf-8");print("[DEBUG] "+json.dumps({"source":tags[0],"target":tags[1],"clear":result["clear"],"positive":len(result["positive"]),"unresolved":len(result["unresolved"]),"seconds":time.monotonic()-START}),flush=True)
    (O/"parts.json").write_text(json.dumps({"source":source,"candidate":candidate,"products":cache}),encoding="utf-8")
    return all(r["clear"]for r in results)and output["stable_source"]and output["stable_producers"]

STOP=threading.Event();watcher=threading.Thread(target=observe,args=(STOP,),daemon=True);watcher.start()
try:raise SystemExit(not main())
except BaseException:
    (O/"exit.txt").write_text(traceback.format_exc(),encoding="utf-8");raise
finally:STOP.set();watcher.join(timeout=2)
