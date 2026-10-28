"""🏋️ Observe complete actual STEP cases through a tool-independent physical interface."""
import hashlib
import json
from pathlib import Path
import sys

class SelfWeightOracle:
    def inspect(self,path,expected):
        raise NotImplementedError

class IfcOpenShellOracle(SelfWeightOracle):
    def inspect(self,path,expected):
        import ifcopenshell
        import ifcopenshell.validate
        model=ifcopenshell.open(str(path))
        cases=model.by_type('IfcStructuralLoadCase')
        raw_vectors=[list(row.SelfWeightCoefficients) if row.SelfWeightCoefficients is not None else None for row in cases]
        vectors=[[getattr(value,'wrappedValue',value) for value in vector] if vector is not None else None for vector in raw_vectors]
        actual=[vector==[0,0,-1] for vector in vectors]
        assert actual==expected['selfWeights'],(expected,actual)
        assert len(model.by_type('IfcStructuralAction'))==expected['actionCount']
        assert cases[0].Coefficient==expected['coefficient']
        assert all(row.is_a()=='IfcStructuralLoadCase' for row in cases)
        material=model.by_type('IfcMaterialProperties')[0]
        rho=next(row for row in material.Properties if row.Name=='MassDensity')
        assert rho.NominalValue.is_a()=='IfcMassDensityMeasure' and rho.NominalValue.wrappedValue==7850
        assert rho.Unit.UnitType=='MASSDENSITYUNIT'
        units={part.Unit.Name:(part.Unit.Prefix,part.Exponent) for part in rho.Unit.Elements}
        assert units=={'GRAM':('KILO',1),'METRE':(None,-3)},units
        area=next(row for row in model.by_type('IfcPropertySingleValue') if row.Name=='CrossSectionArea')
        assert area.NominalValue.is_a()=='IfcAreaMeasure' and area.Unit.Name=='SQUARE_METRE'
        logger=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(model,logger,express_rules=True)
        return dict(id=expected['id'],version=ifcopenshell.version,schema=model.schema_identifier,sha256=hashlib.sha256(path.read_bytes()).hexdigest(),vectors=vectors,rawVectors=raw_vectors,coefficient=cases[0].Coefficient,actions=expected['actionCount'],density=rho.NominalValue.wrappedValue,densityUnits=units,area=area.NominalValue.wrappedValue,diagnostics=logger.statements)

def main():
    root=Path(__file__).parent/'🗑️generated/fem-self-weight'
    inputs=json.loads((root/'input.json').read_text(encoding='utf-8'))
    oracle=IfcOpenShellOracle();rows=[]
    for expected in inputs:
        row=oracle.inspect(root/(expected['id']+'.ifc'),expected);rows.append(row)
        (root/'oracle.json').write_text(json.dumps(dict(python=sys.executable,observations=rows),indent=2,default=str)+'\n',encoding='utf-8')
        assert row['schema']=='IFC4X3_ADD2',row
        if expected['id']=='typed':
            assert len(row['diagnostics'])==1 and row['diagnostics'][0]['attribute']=='SelfWeightCoefficients',row
            assert expected['projected'] is None
        else:assert not row['diagnostics'],row
        print('[DEBUG] physical '+expected['id']+' EXPRESS'+str(len(row['diagnostics'])),flush=True)
    (root/'oracle.json').write_text(json.dumps(dict(python=sys.executable,observations=rows),indent=2,default=str)+'\n',encoding='utf-8')
    import ifcopenshell
    import ifcopenshell.validate
    repo=next(p for p in Path(__file__).parents if (p/'AGENTS.md').exists())
    fixture=repo/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests/structural.ifc'
    source=ifcopenshell.open(str(fixture));logger=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(source,logger,express_rules=True)
    official=dict(sha256=hashlib.sha256(fixture.read_bytes()).hexdigest(),schema=source.schema_identifier,version=ifcopenshell.version,vectors=[row.SelfWeightCoefficients for row in source.by_type('IfcStructuralLoadCase')],sharedPlacements=[row.SharedPlacement.id() if row.SharedPlacement else None for row in source.by_type('IfcStructuralAnalysisModel')],diagnostics=logger.statements,projection='existing public fixture test refuses fem.shared-placement; no whole-model positive claim')
    (root/'official.json').write_text(json.dumps(official,indent=2,default=str)+'\n',encoding='utf-8')
    print('[DEBUG] unchanged official coefficients '+str(official['vectors'])+' EXPRESS'+str(len(logger.statements)),flush=True)
    print('[DEBUG] '+json.dumps(dict(files=len(rows),version=rows[0]['version'],diagnostics=sum(len(row['diagnostics']) for row in rows),expectedInvalidTyped=1)))

if __name__=='__main__':main()
