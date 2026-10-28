"""📏️ Actual IfcOpenShell unit instances corroborate dimensional words behind an owned test interface."""
import json, sys
from pathlib import Path
class UnitOracle:
    def check(self, row):
        raise NotImplementedError
class IfcOpenShellUnitOracle(UnitOracle):
    def __init__(self):
        import ifcopenshell, ifcopenshell.util.unit
        import ifcopenshell.express.rules.IFC4X3_ADD2 as rules
        self.ifc=ifcopenshell
        self.util=ifcopenshell.util.unit
        self.rules=rules
    def check(self,row):
        model=self.ifc.file(schema="IFC4X3_ADD2")
        unit=model.create_entity("IfcSIUnit",UnitType=row["tag"] if row["form"]=="si" else "FORCEUNIT",Prefix=row["prefix"],Name=row["name"])
        exponents=["LengthExponent","MassExponent","TimeExponent","ElectricCurrentExponent","ThermodynamicTemperatureExponent","AmountOfSubstanceExponent","LuminousIntensityExponent"]
        scale=self.util.get_prefix_multiplier(row["prefix"])
        dimensions=unit.Dimensions
        if row["form"]=="derived":
            metre=model.create_entity("IfcSIUnit",UnitType="LENGTHUNIT",Name="METRE")
            elements=[model.create_entity("IfcDerivedUnitElement",Unit=unit,Exponent=1),model.create_entity("IfcDerivedUnitElement",Unit=metre,Exponent=row["lengthExponent"])]
            if row.get("extraDerivedZero"):
                mass=model.create_entity("IfcSIUnit",UnitType="MASSUNIT",Prefix="KILO",Name="GRAM")
                density=model.create_entity("IfcDerivedUnit",Elements=[model.create_entity("IfcDerivedUnitElement",Unit=mass,Exponent=1),model.create_entity("IfcDerivedUnitElement",Unit=metre,Exponent=-3)],UnitType="MASSDENSITYUNIT")
                elements.append(model.create_entity("IfcDerivedUnitElement",Unit=density,Exponent=0))
            dimensions=self.rules.IfcDeriveDimensionalExponents(elements)
            unit=model.create_entity("IfcDerivedUnit",Elements=elements,UnitType=row["tag"])
        dims=[getattr(dimensions,key) for key in exponents]
        tag="FORCEUNIT" if row["role"]=="force" else "TORQUEUNIT"
        expected=[1,1,-2,0,0,0,0] if row["role"]=="force" else [2,1,-2,0,0,0,0]
        si_valid=row["form"]!="si" or bool(self.rules.IfcCorrectDimensions(unit.UnitType,unit.Dimensions))
        named_children=row["form"]=="si" or all(element.Unit.is_a("IfcNamedUnit") for element in unit.Elements)
        accepted=row["assignments"]==1 and unit.UnitType==tag and dims==expected and scale==1 and si_valid and named_children
        result={"id":row["id"],"expected":row["valid"],"actual":accepted,"dimensions":dims,"scale":scale,"siTagMatches":si_valid,"namedChildren":named_children}
        if row.get("extraDerivedZero"):
            import ifcopenshell.validate
            logger=ifcopenshell.validate.json_logger()
            ifcopenshell.validate.validate(model,logger,express_rules=True)
            result["rawDiagnostics"]=logger.statements
        return result
data=json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
oracle=IfcOpenShellUnitOracle()
rows=[]
for row in data["cases"]:
    try:
        rows.append(oracle.check(row))
    except RuntimeError as error:
        rows.append({"id":row["id"],"expected":row["valid"],"actual":False,"externalSchemaRefusal":str(error)})
assert all(row["expected"]==row["actual"] for row in rows),rows
print(json.dumps({"ifcopenshell":oracle.ifc.version,"python":sys.version,"cases":rows,"nativeExecuted":False,"fullIfcValidityClaimed":False,"fundingCertified":False},indent=2,default=str))
