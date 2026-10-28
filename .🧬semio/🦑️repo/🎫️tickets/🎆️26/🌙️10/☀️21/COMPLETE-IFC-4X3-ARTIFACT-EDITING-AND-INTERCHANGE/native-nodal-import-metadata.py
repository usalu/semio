"""🔎️ Independent metadata rosters name the exact declared native nodal import owners."""
import json,sys
class NodalMetadataOracle:
    def fields(self,entity,names):
        raise NotImplementedError
class IfcOpenShellMetadataOracle(NodalMetadataOracle):
    def __init__(self):
        import ifcopenshell,ifcopenshell.ifcopenshell_wrapper as wrapper
        self.version=ifcopenshell.version
        self.schema=wrapper.schema_by_name("IFC4X3_ADD2")
    def fields(self,entity,names):
        declaration=self.schema.declaration_by_name(entity)
        out=[]
        for name in names:
            fields=[(i,a) for i,a in enumerate(declaration.all_attributes()) if a.name()==name]
            assert len(fields)==1,(entity,name)
            index,attribute=fields[0]
            owner=declaration
            while owner is not None and not any(a.name()==name for a in owner.attributes()):
                owner=owner.supertype()
            assert owner is not None
            out.append({"entity":entity.upper(),"name":name,"owner":owner.name().upper(),"index":index,"optional":attribute.optional()})
        return out
oracle=IfcOpenShellMetadataOracle()
requests=[
("IfcStructuralAnalysisModel",["LoadedBy"]),
("IfcStructuralLoadCase",["Name","PredefinedType","Coefficient","SelfWeightCoefficients"]),
("IfcRelAssignsToGroup",["RelatedObjects","RelatingGroup"]),
("IfcRelConnectsStructuralActivity",["RelatingElement","RelatedStructuralActivity"]),
("IfcStructuralPointAction",["ObjectPlacement","Representation","AppliedLoad","GlobalOrLocal","DestabilizingLoad"]),
("IfcStructuralLoadSingleForce",["ForceX","ForceY","ForceZ","MomentX","MomentY","MomentZ"])
]
rows=[field for entity,names in requests for field in oracle.fields(entity,names)]
print(json.dumps({"ifcopenshell":oracle.version,"python":sys.version,"fields":rows,"nativeExecuted":False,"physicalValidityClaimed":False},indent=2))
