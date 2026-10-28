from pathlib import Path
T=Path(__file__).parent
p=T/'type-value-select-metadata-oracle.py'
s=p.read_text(encoding='utf-8-sig')
s=s.replace("return {'oracle':'IfcOpenShell','version':ifcopenshell.version,'schema':'IFC4X3_ADD2','ancestry':output}","aggregates={}\n  for name in ['IfcComplexNumber','IfcCompoundPlaneAngleMeasure']:\n   declaration=schema.declaration_by_name(name).as_type_declaration().declared_type().as_aggregation_type()\n   aggregates[name.upper()]={'kind':declaration.type_of_aggregation_string(),'lower':declaration.bound1(),'upper':declaration.bound2()}\n  return {'oracle':'IfcOpenShell','version':ifcopenshell.version,'schema':'IFC4X3_ADD2','ancestry':output,'aggregates':aggregates}")
p.write_text(s,encoding='utf-8',newline='\n')
