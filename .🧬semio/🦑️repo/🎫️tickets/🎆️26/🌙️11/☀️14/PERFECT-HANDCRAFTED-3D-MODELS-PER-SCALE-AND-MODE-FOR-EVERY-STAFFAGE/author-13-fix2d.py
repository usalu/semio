import json
_src = open("volumes-insert.py").read()
_src = _src[_src.index("import json"):_src.index("def member_end")]
_ns = {}
exec(_src, _ns)
dump = _ns["dump"]
def edit(path, fn):
    text = open(path).read()
    doc = json.loads(text)
    same = dump(doc, 0) == text or dump(doc, 0) + "\n" == text
    assert same, "roundtrip differs"
    fn(doc["asset"])
    open(path, "w").write(dump(doc, 0) + ("\n" if text.endswith("\n") else ""))
