"""🔬 Independently verify representable IFC source with ASCII and escaped Unicode headers."""
import json
import pathlib
import sys

TICKET = pathlib.Path(__file__).resolve().parent
sys.path.insert(0, str(TICKET / '🗑️generated/ifc083'))
import ifcopenshell

text = "ISO-10303-21;HEADER;FILE_DESCRIPTION(('test'),'2;1');FILE_NAME('','',(''),(''),'','','');FILE_SCHEMA(('IFC4X3_ADD2'));ENDSEC;DATA;#1=IFCCARTESIANPOINT((0.,1.));ENDSEC;END-ISO-10303-21;"
rows = []
for name, value in [('ascii', 'a' * 200), ('unicode', 'ä' * 30), ('short', 'short')]:
    model = ifcopenshell.file.from_string(text)
    model.header.file_name.name = value
    source = model.to_string()
    restored = ifcopenshell.file.from_string(source)
    assert restored.header.file_name.name == value
    assert restored.schema_identifier == 'IFC4X3_ADD2'
    assert restored.by_id(1).Coordinates == (0., 1.)
    rows.append(dict(case=name, sourceBytes=len(source.encode('utf-8')), headerCharacters=len(value), exactRoundtrip=True))
print(json.dumps(dict(oracle='IfcOpenShell', version=ifcopenshell.version, cases=rows)))
