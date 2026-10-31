"""🏠 Author raised exclusive structural joints within complete roof finish reserves."""
import importlib,importlib.util,json,sys,copy
from pathlib import Path
from shapely.geometry import Polygon
sys.path.insert(0,str(Path.cwd()));a=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️");root=Path(__file__).parent;spec=importlib.util.spec_from_file_location("raised_floor_author",root/"floor-raised-beam-authoring-r92.py");author=importlib.util.module_from_spec(spec);spec.loader.exec_module(author);m=a.load_model();c=json.loads((Path(a.__file__).parent/"🧪️tests/🏠roof-floors.json").read_text());rows=[{"tag":r["tag"],"parts":a._find(m,r["tag"])["geometry"]["parts"]} for r in c["floors"]];trimmed=author.trim_floor_layers(m,rows,level=6.6,beam_tags=c["raised_beams"],bedding_material="MAT-ROOF-FINISH",bedding_role="weathering_reserve");slabs={t:a._slab_geometry_entry(m,a._find(m,t)) for t in ("SL-R","SL-KIOSK")};before=sum(a._partition_volume(g["finish_parts"])+a._partition_volume(g["shared_finish_parts"]) for g in slabs.values());beams={tag:a._solid_geometry(m,a._find(m,tag),resolve_embedded=False)["parts"] for tag in c["raised_beams"]};jointmap={};n=copy.deepcopy(m)
for row in trimmed:a._find(n,row["tag"])["geometry"]["parts"]=row["parts"]
for tag,slab in slabs.items():
    jointmap[tag]={}
    for reserve in a._find(n,tag)["geometry"].get("finish_voids",[]):
        if reserve["owner"] not in {r["tag"] for r in rows}:continue
        ring=Polygon(reserve["outline"]);owners=[]
        for owner,parts in beams.items():
            occupied=sum(ring.intersection(Polygon(p["outline"])).intersection(Polygon(h["outline"])).area*max(0,min(p["z1"],h["z1"])-max(p["z0"],h["z0"])) for p in parts for h in slab["full_finish_parts"] if p["z0"]<h["z1"] and p["z1"]>h["z0"] and Polygon(p["outline"]).intersects(ring) and Polygon(h["outline"]).intersects(ring))
            if occupied>1e-12:owners.append(owner)
        if owners:reserve["joint_owners"]=owners;jointmap[tag][reserve["tag"]]=owners
after_slabs={t:a._slab_geometry_entry(n,a._find(n,t)) for t in slabs};after=sum(a._partition_volume(g["finish_parts"])+a._partition_volume(g["shared_finish_parts"]) for g in after_slabs.values());assert abs(before-after)<1e-7,(before,after);joint_volume=sum(a._partition_volume([p for p in g["shared_finish_parts"] if p["owner"] in beams]) for g in after_slabs.values());result={"floors":trimmed,"reserve_joints":jointmap,"before_reference":before,"after_reference":after,"structural_joint_volume":joint_volume};(root/"🗑️generated/roof-beam-joints-r92.json").write_text(json.dumps(result),encoding="utf8");print("[DEBUG] "+json.dumps({"floors":[{k:r[k] for k in ("tag","removed")} for r in trimmed],"joint_cells":{t:len(v) for t,v in jointmap.items()},"before":before,"after":after,"joint_volume":joint_volume}),flush=True)
