"""📜 Validate a fresh manually declared wall source candidate against the complete schema and ledger."""
import importlib,importlib.util,json,sys,unittest,copy,hashlib,time
from pathlib import Path
from collections import Counter
R=Path.cwd();p=Path(__file__).parent;C=R/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip'
INPUTS={file for file in C.rglob('*') if file.is_file() and file.suffix in ('.py','.json') and not any(part in ('🗑️generated','__pycache__','node_modules') for part in file.relative_to(C).parts)}|{Path(__file__),p/'wall-junction-source-r95.py',p/'complete-conflict-ledger-r95.md'}

def hashes():
    """🔐 Bind fresh imports and semantic assertions to complete source, producer and fixture bytes."""
    return {str(path.relative_to(R)):hashlib.sha256(path.read_bytes()).hexdigest() for path in INPUTS}

started=time.monotonic();inputs_before=hashes();sys.path.insert(0,str(R));a=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');spec=importlib.util.spec_from_file_location('wall_junction_source',p/'wall-junction-source-r95.py');h=importlib.util.module_from_spec(spec);spec.loader.exec_module(h)
source_path=Path(a.__file__).with_name('🔣️.json');source_bytes=source_path.read_bytes();source=json.loads(source_bytes);assert hashlib.sha256(source_bytes).hexdigest()==inputs_before[str(source_path.relative_to(R))]
m,rels=h.apply(a,source);ledger=[tuple(c.strip() for c in line.split('|')[2:5:2]) for line in (p/'complete-conflict-ledger-r95.md').read_text(encoding='utf8').splitlines() if line.startswith('| Native |')];pairs={frozenset((r['relating'],r['related'][0])) for r in rels};missing=[pair for pair in ledger if frozenset(pair) not in pairs];print('[DEBUG] Declared',len(rels),'relationships',len(h.NODES),'nodes; ledger uncovered',missing);assert not missing
from jsonschema import Draft202012Validator
schema=json.loads((Path(a.__file__).parent/'🧬️schema/🔣️.schema.json').read_text(encoding='utf8'));errors=list(Draft202012Validator(schema).iter_errors(m));print('[DEBUG] Candidate schema errors',len(errors));print('\n'.join(str(e)[:800] for e in errors[:3]));assert not errors
out=p/'🗑️generated/wall-junctions-r95';out.mkdir(parents=True,exist_ok=True);(out/'source-candidate.json').write_text(json.dumps(m,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
T=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧪️tests.🐍️')
controls={
'BimGlobalIdContract':['test_every_stored_guid_is_derived_from_its_tag_and_unique'],
'BimModelStructureContract':['test_every_element_is_complete_and_references_exist','test_class_registry_reaches_ifc_root_for_every_used_class'],
'BimRelationContract':['test_materials_are_associated_and_relation_ends_exist','test_relations_of_reports_both_sides_with_guids'],
'BimClassChainContract':['test_every_authored_element_class_has_a_chain'],
'BimIdsContract':['test_default_requirement_passes_for_all_pilotis_in_ifc_and_finds_none_in_the_mesh','test_counts_equal_an_independent_recount_of_the_json'],
'BimPartialModelContract':['test_models_share_schema_and_differ_in_content','test_counts_by_class_are_recomputed_from_the_guids'],
}
results=[];before=copy.deepcopy(m)
for class_name,names in controls.items():
    cls=getattr(T,class_name)
    for name in names:
        instance=cls(methodName=name);instance.api=a;instance.model=m;instance.snapshot=copy.deepcopy(m);instance.items=T.registry(m)
        getattr(instance,name)();results.append(class_name+'.'+name)
for relation in rels:
    chain=a.class_chain(relation['class'],m);attributes=[field for level in chain for field in level['adds']]
    assert chain[0]['class']=='IfcRoot' and len(attributes)==len(set(attributes))
    assert len(attributes)==(11 if relation['class']=='IfcRelConnectsPathElements' else 7)
    for tag in (relation['relating'],relation['related'][0]):
        found=[row for row in a.relations_of(tag,m) if row['tag']==relation['tag']];assert len(found)==1 and found[0]['guid']==relation['guid']
assert m==before
assert all(m[key]==source[key] for key in source if key not in ('relations','classes'));assert m['relations'][:-len(rels)]==source['relations'];assert all(m['classes'][key]==value for key,value in source['classes'].items())
original_counts=Counter(row['class'] for row in source['relations']);counts=Counter(row['class'] for row in m['relations']);added_counts=Counter(row['class'] for row in rels);assert counts==original_counts+added_counts
inputs_after=hashes();report={'success':True,'stable':inputs_before==inputs_after,'before':inputs_before,'after':inputs_after,'product_count':len(m['elements']),'junction_groups':len(h.NODES),'relations_added':len(rels),'ledger_pairs_covered':len(ledger),'class_chains':{name:a.class_chain(name,m) for name in h.RELATION_CLASSES},'relationship_roster':dict(counts),'original_relationship_roster':dict(original_counts),'added_relationship_roster':dict(added_counts),'source_controls':results,'source_unchanged':inputs_before[str(source_path.relative_to(R))]==inputs_after[str(source_path.relative_to(R))],'candidate_preserves_other_fields':True,'seconds':time.monotonic()-started,'source_sha256':hashlib.sha256(source_bytes).hexdigest()}
(out/'candidate-semantic.json').write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf8');print('[DEBUG] Candidate semantic/core/IDS controls',len(results),'GREEN; stable',report['stable']);assert report['stable']
