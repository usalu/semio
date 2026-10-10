import json, sys, jsonschema
s = json.load(open(sys.argv[1], encoding="utf-8"))
jsonschema.Draft7Validator.check_schema(s)
d = json.load(open(sys.argv[2], encoding="utf-8"))
errs = list(jsonschema.Draft7Validator(s).iter_errors(d))
print(len(errs))
for e in errs[:8]:
    print(list(e.path), e.message[:120])
