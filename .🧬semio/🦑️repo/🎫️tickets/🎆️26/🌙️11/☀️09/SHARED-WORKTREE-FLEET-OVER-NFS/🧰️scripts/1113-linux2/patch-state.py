import sys
root=sys.argv[1]
p=root+'/🔁️replica/💾️state/🧬️schema/🔣️.json'
s=open(p,encoding='utf-8').read()
old='"items": { "type": "object", "required": ["path", "hlc", "operation"] }'
new='"items": { "type": "object", "required": ["path", "hlc", "operation"], "properties": { "path": { "type": "string", "minLength": 1 }, "hlc": { "$ref": "#/definitions/Hlc" }, "operation": { "enum": ["put", "delete", "link"] } } }'
assert s.count(old)==2
open(p,'w',encoding='utf-8').write(s.replace(old,new))
