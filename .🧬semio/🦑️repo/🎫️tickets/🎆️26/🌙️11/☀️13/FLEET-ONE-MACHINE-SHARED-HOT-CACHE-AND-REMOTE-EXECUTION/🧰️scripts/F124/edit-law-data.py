import os
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/👓️image-lease-readers/"
def patch(path,pairs):
    s=open(path,encoding="utf8").read()
    for old,new in pairs:
        assert s.count(old)==1,(path,old[:60],s.count(old));s=s.replace(old,new)
    tmp=path+".f124tmp";open(tmp,"w",encoding="utf8").write(s);os.replace(tmp,path)
patch(L+"🧬️schema/🔣️.json",[
 ('    "mountPatterns",\n    "openers",\n    "leaseCallees",\n    "leasedParameters",\n','    "mountPatterns",\n    "rootPatterns",\n    "rootTypes",\n    "openers",\n    "leaseCallees",\n    "leasedParameters",\n    "leasedArguments",\n'),
 ('    "leasedParameters": {\n      "type": "array",\n      "minItems": 1,\n      "items": {\n        "type": "string",\n        "minLength": 1\n      }\n    }\n',
  '    "leasedParameters": {\n      "type": "array",\n      "minItems": 1,\n      "items": {\n        "type": "string",\n        "minLength": 1\n      }\n    },\n    "rootPatterns": {\n      "type": "array",\n      "minItems": 1,\n      "description": "Regular expressions over the text of an expression: a match means the expression is a path below the image of a cache partition (a call of a helper that resolves an area directory, the declared partition table). Names declared from such an expression carry the path on, through variables, parameters, assignments, loop bindings and calls of functions of the same file, whatever the shape of the expression.",\n      "items": {\n        "type": "string",\n        "minLength": 1\n      }\n    },\n    "rootTypes": {\n      "type": "array",\n      "minItems": 1,\n      "description": "Regular expressions over a type annotation: a name declared with a matching type (a partition table, a request that carries the directory to walk) carries a path below the image of a cache partition.",\n      "items": {\n        "type": "string",\n        "minLength": 1\n      }\n    },\n    "leasedArguments": {\n      "type": "array",\n      "minItems": 1,\n      "description": "Regular expressions over the argument text of an opener call: a match means the call hands a reader of the image to its callee, which runs every read inside the reader (the callee file is judged by the same law).",\n      "items": {\n        "type": "string",\n        "minLength": 1\n      }\n    }\n'),
])
patch(L+"📒️allowlist/🔣️.json",[
 (' "mountPatterns": [','__SKIP__') if False else (' "openers": [\n',' "rootPatterns": [\n  "\\\\b(cacheAreaDirectoryV1|cacheAreaHomeV1)\\\\s*\\\\(",\n  "\\\\bbudgetPolicy\\\\.partitions\\\\b",\n  "\\\\brepositoryCacheBudgetV1\\\\s*\\\\("\n ],\n "rootTypes": [\n  "\\\\b(CachePartitionBudgetsV1|RepositoryCacheBudgetV1|RepositoryCachePartitionV1|DedupeRequestV1)\\\\b"\n ],\n "openers": [\n'),
 ('  "^(censusNxToolingStoreV1|collectUnpinnedDependencyGenerationsV1)$"','  "^(censusNxToolingStoreV1|collectUnpinnedDependencyGenerationsV1|dedupeTreeV1)$"'),
 (' "classes": [',' "leasedArguments": [\n  "\\\\breader\\\\b"\n ],\n "classes": ['),
])
print("law data ok")
