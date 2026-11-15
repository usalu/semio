import sys,glob,json
p=glob.glob(sys.argv[1]+"/*attest/*schema/\U0001F523️.json")[0]
s=open(p,encoding="utf8").read()
old='''     "properties": {"incoming": {"$ref": "#/definitions/Count"}}
    }
   }
  },
  "Retirement": {'''
new='''     "properties": {"incoming": {"$ref": "#/definitions/Count"}}
    },
    "pending": {
     "type": "object",
     "additionalProperties": false,
     "required": ["code", "ok", "reason"],
     "properties": {"code": {"$ref": "#/definitions/Sha256"}, "ok": {"const": false}, "reason": {"type": "string", "maxLength": 512}}
    },
    "failures": {
     "type": "array",
     "maxItems": 8,
     "items": {
      "type": "object",
      "additionalProperties": false,
      "required": ["count", "kind", "lastMs"],
      "properties": {"kind": {"type": "string", "pattern": "^[a-z][a-z0-9-]{0,39}$"}, "count": {"$ref": "#/definitions/Count"}, "lastMs": {"$ref": "#/definitions/Count"}}
     }
    }
   }
  },
  "Retirement": {'''
assert s.count(old)==1,"attest schema anchor"
s=s.replace(old,new)
o2='"Field": {"enum": ["manifest", "head.commit", "indexTree", "view.digest"]},'
assert o2 in s
s=s.replace(o2,'"Field": {"enum": ["code", "manifest", "head.commit", "indexTree", "view.digest"]},')
json.loads(s)
open(p,"w",encoding="utf8").write(s)
