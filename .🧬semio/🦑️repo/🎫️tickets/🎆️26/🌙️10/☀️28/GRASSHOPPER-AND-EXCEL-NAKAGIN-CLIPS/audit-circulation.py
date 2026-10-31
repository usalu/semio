"""🧾 Snapshot the physical support graph and report exact broken circulation contacts."""
import importlib.util
import ast
import hashlib
import json
from pathlib import Path
import sys
sys.stdout.reconfigure(errors='backslashreplace')
root=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
spec=importlib.util.spec_from_file_location('circulation_audit',root/'🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🚶️circulation/🐍️.py')
fingerprint_files=[root/'🎓️teaching/🏛️architecture/🏢️nakagin'/name for name in ('🐍️.py','🔣️.json','🧪️tests/🚶️circulation/🐍️.py','🧪️cases/🚶️circulation/🔣️.json')]
def fingerprints():
    result={}
    for path in fingerprint_files:
        text=path.read_text(encoding='utf8')
        value=ast.dump(ast.parse(text),include_attributes=False) if path.suffix=='.py' else json.dumps(json.loads(text),sort_keys=True,ensure_ascii=False,separators=(',',':'))
        result[str(path.relative_to(root))]=hashlib.sha256(value.encode('utf8')).hexdigest()
    return result
source_fingerprints=fingerprints()
module=importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
parts=module.MODEL.structure_assembly("full").parts
nodes,graph,seeds=module.snapshot(parts)
headroom=module.headroom_conflicts(parts,nodes)
reached=module.reachable(graph,seeds)
targets=sorted(k for k in nodes if '/capsule-landing-' in k)
missing=[k for k in targets if k not in reached]
def contact(a,b):
    aa,bb=nodes[a],nodes[b]
    ax0,ax1,ay0,ay1=aa['rect']; bx0,bx1,by0,by1=bb['rect']
    return {'a':a,'b':b,'dx':min(ax1,bx1)-max(ax0,bx0),'dy':min(ay1,by1)-max(ay0,by0),'dz':abs(aa['z']-bb['z']),'a_node':aa,'b_node':bb}
turns=[]
for tower in 'AB':
    previous=tower+'/turning-landing-0'
    for key in nodes:
        if previous in nodes and key.startswith(tower+'/stair-1-'):
            turns.append(contact(previous,key))
near=[]
diagnostic_targets=missing+[k for k,node in nodes.items() if node['role'] in ('lift_approach','bridge') and k not in reached]
for key in diagnostic_targets:
    candidates=[]
    for other,node in nodes.items():
        if other==key or (key[0] in 'AB' and not other.startswith(key[0])) or '/capsule-landing-' in other or abs(node['z']-nodes[key]['z'])>module.CASES['max_riser']:
            continue
        data=contact(key,other)
        data['gap']=max(0,-data['dx'],-data['dy'])
        candidates.append(data)
    near.extend(sorted(candidates,key=lambda d:(d['gap'],-max(d['dx'],d['dy'])))[:2])
bridge_contacts=[contact(key,other) for key,node in nodes.items() if node['role']=='bridge' for other,second in nodes.items() if second['role']=='bridge_approach' and abs(node['z']-second['z'])<.2]
terminal_counts={role:{'total':sum(n['role']==role for n in nodes.values()),'reached':sum(nodes[k]['role']==role for k in reached)} for role in ('landing','bridge','bridge_approach','lift_approach','ground_threshold','office_threshold','bridge_return')}
footprint_coverage={'paths':0,'unsupported':[]}
for key,node in nodes.items():
    if not node.get('walking_route'):
        continue
    supports=[module._polygon(other) for other in nodes.values() if abs(other['z']-node['z'])<module.EPS]
    for prism in module.walking_strips(node):
        footprint=module.strip_footprint(prism)
        missing_area=module.unsupported_area(footprint,supports)
        oracle_area=module.numpy_supported_area(footprint,supports)
        area=module.polygon_area(footprint)
        footprint_coverage['paths']+=1
        if missing_area>=1e-7 or abs(oracle_area-area)>=1e-7:
            footprint_coverage['unsupported'].append({'key':key,'missing_area':missing_area,'oracle_supported':oracle_area,'footprint_area':area})
roof_returns=[key for key in nodes if '/roof-return-landing-' in key]
if fingerprints()!=source_fingerprints:
    raise RuntimeError('Authoritative circulation inputs changed during the audit')
role_counts={role:sum(n['role']==role for n in nodes.values()) for role in sorted({n['role'] for n in nodes.values()})}
result={'footprint_coverage':footprint_coverage,'roof_returns':{'total':len(roof_returns),'reached':sum(key in reached for key in roof_returns)},'source_fingerprints':source_fingerprints,'role_counts':role_counts,'bridge_contacts':bridge_contacts,'terminal_counts':terminal_counts,'support_count':len(nodes),'reachable_count':len(reached),'capsule_target_count':len(targets),'missing':missing,'turns':turns,'nearest_missing_supports':near,'headroom_conflicts':headroom,'seeds':sorted(seeds)}
folder=Path(__file__).parent
(folder/'🗑️generated'/'circulation.json').write_text(json.dumps(result,ensure_ascii=False,indent=2),encoding='utf8')
lines=['# Physical Circulation Audit','',f'Actual convex support faces: {len(nodes)}; reachable from ground: {len(reached)}; missing capsule landings: {len(missing)}/{len(targets)}.','', 'A walking edge requires actual upward convex faces, maximum riser 0.170001 m, no XY gap and at least 0.6 m shared support width. Corner contacts and narrower edges are rejected. The stdlib convex support graph is checked against the independent NumPy clipping/NetworkX oracle.','', '## Required Targets','', '| Target | Required | Reached |','|---|---:|---:|',f'| Capsule landings | 140 | {len(targets)-len(missing)} |',f'| Lift approaches, including B1 | 26 | {terminal_counts["lift_approach"]["reached"]} |',f'| Bridge approaches | 6 | {terminal_counts["bridge_approach"]["reached"]} |',f'| Bridge decks | 3 | {terminal_counts["bridge"]["reached"]} |',f'| Exterior threshold seeds | 2 | {len(seeds)} |',f'| Office thresholds | 2 | {terminal_counts["office_threshold"]["reached"]} |',f'| Native L-return decks | 3 | {terminal_counts["bridge_return"]["reached"]} |',f'| Roof return landings | 2 | {sum(key in reached for key in roof_returns)} |','', '## Historical Failure Record','', 'Before phase-route reconstruction, the graph had 819 supports, only 72 reachable and none of 140 capsule landings reachable. The first-turn shared width was 0.55 m, below the 0.6 m contract, and the first A west pair had a 0.84 m gap. The later graph reached all targets but mesh headroom exposed two low roof arms and then one B water-tank obstruction; these were corrected in geometry, not suppressed by the tests.','', '## First Turning Contacts','', '~~~json',json.dumps(turns,ensure_ascii=False,indent=2),'~~~','','## Capsule Landing Contacts','', '~~~json',json.dumps(near[:12],ensure_ascii=False,indent=2),'~~~','','## Authoritative Semantic Fingerprints','','~~~json',json.dumps(source_fingerprints,ensure_ascii=False,indent=2),'~~~','','## Body Footprint Coverage','',f'Checked oriented paths: {footprint_coverage["paths"]}; unsupported or oracle-disagreeing paths: {len(footprint_coverage["unsupported"])}.','', '## Walking Clearance','',f'All-part mesh clearance conflicts: {len(headroom)}. Actual stair-tread centre paths checked: {role_counts.get("stair_tread",0)}.','~~~json',json.dumps(headroom[:16],ensure_ascii=False,indent=2),'~~~','','## Coverage Limits','','The graph starts at the two explicit exterior ground thresholds. Passage width, normal-riser limits and clear heights are authored assumptions, not surveyed proof of the historical building. Flat supports use continuous 0.6 m body strips, including oriented polygonal turns; complete footprints are independently proven covered by the union of actual supports. Every actual stair tread is checked along its complete run with a continuous 2 mm centreline tube at 2.1 m headroom. AABBs provide only a broad phase. Actual closed-mesh faces are ear-clipped and checked by triangle/prism SAT plus ray containment; independent NumPy clipping/solid-angle fixtures validate the narrow phase. Every physical part and conservative nominal capsule envelope is considered. Door landings use the authored 1.95 m height, interior connectors 2.1 m; a 0.0001 m surface tolerance handles rounded authored elevations, and only geometrically adjacent normal-riser transitions are exempted. The separate full doorway-volume and structural-bearing regressions complement this route proof. This does not establish clearance along every possible path over a support, between the tested centre strips, through operable lift doors, or for a surveyed/code-compliant reconstruction.']
(folder/'circulation.md').write_text('\n'.join(lines)+'\n',encoding='utf8')
print('[DEBUG] '+json.dumps({'support_count':len(nodes),'reachable_count':len(reached),'missing':len(missing),'terminal_counts':terminal_counts,'headroom_count':len(headroom),'headroom_first':headroom[:8],'bridge_contacts':bridge_contacts},ensure_ascii=False),flush=True)
