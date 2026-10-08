import sys
import ifcopenshell
import ifcopenshell.validate

model = ifcopenshell.open(sys.argv[1])
log = ifcopenshell.validate.json_logger()
ifcopenshell.validate.validate(model, log, express_rules=len(sys.argv) > 2)
print(model.schema, len(log.statements), "statements")
seen = {}
for entry in log.statements:
    key = (entry.get("level"), str(entry.get("message"))[:150])
    seen[key] = seen.get(key, 0) + 1
for key, count in list(seen.items())[:40]:
    print(count, key)
