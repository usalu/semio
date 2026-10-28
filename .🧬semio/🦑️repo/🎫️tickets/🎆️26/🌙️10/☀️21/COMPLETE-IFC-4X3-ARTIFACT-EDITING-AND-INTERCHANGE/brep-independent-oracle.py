"""🧊 Corroborate selected faceted Brep metadata and unchanged original STEP observations."""
import hashlib,importlib.util,json,sys
from pathlib import Path
root=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('surface_oracle',root/'body-surface-model-oracle.py');module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
class BrepOracle(module.IfcOpenShellSurfaceModelOracle):
    def observe(self,path,identity):
        model=self._library.open(str(path));rep=model.by_id(int(identity));families=[self.families(item.is_a())for item in rep.Items]
        logger=self._validation.json_logger();self._validation.validate(model,logger,express_rules=True)
        return dict(headerSchema=list(model.header.file_schema.schema_identifiers),representation=str(rep.id()),identifier=rep.RepresentationIdentifier,type=rep.RepresentationType,items=[dict(id=str(item.id()),type=item.is_a(),families=names)for item,names in zip(rep.Items,families)],selectedConceptStatus='pass'if rep.RepresentationIdentifier=='Body'and rep.RepresentationType=='Brep'and all('IFCFACETEDBREP'in names for names in families)else'fail',actualTypeFunction=bool(self._rules.IfcShapeRepresentationTypes(rep.RepresentationType,rep.Items)),brepTypeFunction=bool(self._rules.IfcShapeRepresentationTypes('Brep',rep.Items)),diagnostics=logger.statements)
base=Path(sys.argv[1]);laws=json.loads((base/'🧬️schema/👁️mvd/🧩️concepts/🧊️brep/cases.json').read_text(encoding='utf-8'));oracle=BrepOracle();neutral=[]
for row in laws['cases']:
    names=[oracle.families(name)for name in row['items']];known=all(item is not None for item in names);status='not-applicable'if row['applicable']is False else'incomplete'if row['applicable']is None or not known else'pass'if row['identifier']=='Body'and row['type']=='Brep'and all('IFCFACETEDBREP'in item for item in names)else'fail'
    assert status==row['expected'],row
    neutral.append(dict(id=row['id'],status=status,families=names))
originals=[]
for file,identity in [('brep-model.ifc','1020'),('basin-faceted-brep.ifc','1155'),('cube-advanced-brep.ifc','183')]:
    path=base/'🧪️fixtures/official'/file;originals.append(dict(file=file,sha256=hashlib.sha256(path.read_bytes()).hexdigest(),**oracle.observe(path,identity)))
output=dict(oracle='ifcopenshell',version=oracle.version,nativeExecution=False,neutral=neutral,originals=originals)
(root/'🗑️generated/brep-independent-oracle.json').write_text(json.dumps(output,ensure_ascii=False,indent=2,default=str)+'\n',encoding='utf-8')
print('[DEBUG] '+json.dumps(dict(neutral=len(neutral),originals=[dict(file=r['file'],selected=r['selectedConceptStatus'],actualFunction=r['actualTypeFunction'],brepFunction=r['brepTypeFunction'],errors=len(r['diagnostics']))for r in originals])))
