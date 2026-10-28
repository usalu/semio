"""🔢️ Observe physical REAL/NUMBER/BINARY boundaries without exporting external types."""
from pathlib import Path
import ctypes
import json
import struct
import sys

ticket = Path(__file__).resolve().parent
dependency = str(ticket / "🗑️generated/oracle")
if sys.platform == "win32":
    buffer = ctypes.create_unicode_buffer(32768)
    ctypes.windll.kernel32.GetShortPathNameW(dependency, buffer, len(buffer))
    dependency = buffer.value or dependency
sys.path.insert(0, dependency)

class ScalarOracle:
    """🧭️ Ordinary observations isolate physical parser and validator behavior."""
    def observe(self, kind, payload, full):
        raise NotImplementedError

class IfcOpenShellScalar(ScalarOracle):
    """🧊️ Real word, numeric transport and binary text remain independent observations."""
    def __init__(self):
        import ifcopenshell
        import ifcopenshell.validate
        self._library = ifcopenshell
        self._validation = ifcopenshell.validate
        self.version = ifcopenshell.version

    def observe(self, kind, payload, full):
        header = "ISO-10303-21;HEADER;FILE_DESCRIPTION(('Audit'),'2;1');FILE_NAME('audit.ifc','2026-10-21T00:00:00',('Audit'),('Audit'),'Audit','Audit','');FILE_SCHEMA(('IFC4X3_ADD2'));ENDSEC;DATA;"
        if kind == "real":
            body = "#9=IFCCARTESIANPOINT((0.,0.));#10=IFCCARTESIANPOINT((1.,0.));#7=IFCPOLYLINE((#9,#10));#1=IFCPOINTONCURVE(#7,"+payload+");"
            attribute = "PointParameter"
        elif kind == "number":
            body = "#1=IFCQUANTITYNUMBER('Audit',$,$,"+payload+",$);"
            attribute = "NumberValue"
        else:
            body = '#1=IFCBLOBTEXTURE(.T.,.F.,$,$,$,\'PNG\',"'+payload+'");'
            attribute = "RasterCode"
        text = header + body + "ENDSEC;END-ISO-10303-21;"
        try:
            model = self._library.file.from_string(text)
            entity = model.by_id(1)
            value = getattr(entity, attribute)
            result = {"kind": kind, "input": payload, "parsed": True, "pythonType": type(value).__name__, "value": str(value), "stepEntity": str(entity)}
            if isinstance(value, float):
                result["bits"] = str(struct.unpack('>Q', struct.pack('>d', value))[0])
            if full:
                logger = self._validation.json_logger()
                self._validation.validate(model, logger, express_rules=True)
                result["fullExpressDiagnostics"] = [{"attribute": str(row.get("attribute", "")), "message": str(row.get("message", ""))} for row in logger.statements]
            return result
        except Exception as error:
            return {"kind": kind, "input": payload, "parsed": False, "error": str(error)}

oracle = IfcOpenShellScalar()
rows = [oracle.observe("real", value, value == "1.5") for value in ["0.", "-0.", "0.5", "1.5", "5.E-324", "1.7976931348623157E308"]]
rows += [oracle.observe("number", value, True) for value in ["2", "1.5", "9007199254740993"]]
rows += [oracle.observe("binary", value, value in ["0A5FF", "0"]) for value in ["0A5FF", "31", "23", "12A", "00", "0", "", "0a5", "4F", "3A"]]
result = {"library": "IfcOpenShell", "version": oracle.version, "rows": rows, "nativeExecuted": False, "internalWireAdmissionProved": False}
with (ticket / "🗑️generated/type-real-number-binary-oracle.json").open("x", encoding="utf-8") as output:
    json.dump(result, output, indent=2)
print("[DEBUG] " + json.dumps(rows))
