import lib
from lib import S
UNIT = "🎛commands/📝commit/📊metrics/🔢uloc"
keys = [("", 0), ("   ", 0), ("{}", 0), ('{"a":1,"b":{"c":2}}', 3), ('[{"a":1},{"b":2,"c":3}]', 3), ("not json", 0), ('  {"a":1}  ', 1), ("42", 0), ('{"a":[{"b":1}]}', 2), ("[]", 0), ('{"a":null,"b":[1,2,[{"c":0}]]}', 3), ('{"x":{"y":{"z":{}}}}', 3)]
diffs = [(None, '{"a":1}', {"added": 1, "removed": 0, "edited": 0}), ('{"a":1}', None, {"added": 0, "removed": 1, "edited": 0}), ('{"a":1}', '{"a":1}', {"added": 0, "removed": 0, "edited": 0}), ('{"a":1}', '{"a":2}', {"added": 0, "removed": 0, "edited": 1}),
 ('{"a":{"b":1}}', '{"a":{"b":1,"c":2}}', {"added": 1, "removed": 0, "edited": 1}), ('{"a":1,"b":2}', '{"b":2}', {"added": 0, "removed": 1, "edited": 0}), ('[{"a":1}]', '[{"a":1},{"a":2}]', {"added": 1, "removed": 0, "edited": 0}),
 ('{"a":[1,2]}', '{"a":[1,3]}', {"added": 0, "removed": 0, "edited": 1}), ('{"a":{"b":1}}', '{"a":5}', {"added": 0, "removed": 1, "edited": 1}), ('{"a":1}', "not json", None), ("[]", '{"a":1}', None), (None, "{}", None), ('{"a":1}', '{"b":1}', {"added": 1, "removed": 1, "edited": 0}), (None, None, {"added": 0, "removed": 0, "edited": 0})]
locs = [("a.ts", "a\nb", 2), ("a.ts", "", 0), ("a.ts", "a\r\nb\r\nc", 3), ("a.json", '{"a":1,"b":2}', 2), ("A.JSON", '{"a":1}', 1), ("a.jsonc", "{}", 1), ("a.json", "", 0), ("a.json", "   \n", 0), ("a.json", "{\n}", 2), ("a.json", "not json\nline", 2), ("Makefile", "x\ny", 2), ("dir.d/file", "x", 1)]
locdefects = [("a.ts", "a\nb\n", 2), ("a.ts", "\n", 1), ("a.json", "{}\n", 1)]
corpus = {"version": 1, "keys": [{"text": t, "expected": n} for t, n in keys], "diff": [{"before": b, "after": a, "expected": e} for b, a, e in diffs], "loc": [{"path": p, "data": d, "expected": n} for p, d, n in locs],
 "locDefects": [{"path": p, "data": d, "expected": n, "defect": "📊️metrics/🔢️uloc/🟦️.ts:physicalLineCount counts the empty segment after a final newline, so every newline-terminated file is one line larger than `wc -l` and git numstat"} for p, d, n in locdefects]}
dl = S.obj({"added": S.int, "removed": S.int, "edited": S.int})
schema = S.obj({"version": S.const(1), "keys": S.arr(S.obj({"text": S.str, "expected": S.int}), 1), "diff": S.arr(S.obj({"before": S.one(S.str, S.null), "after": S.one(S.str, S.null), "expected": S.one(dl, S.null)}), 1),
 "loc": S.arr(S.obj({"path": S.str, "data": S.str, "expected": S.int}), 1), "locDefects": S.arr(S.obj({"path": S.str, "data": S.str, "expected": S.int, "defect": S.str}), 1)})
bodyts = '''for(const row of corpus.keys)test("json keys "+JSON.stringify(row.text),()=>{expect(countJsonKeys(row.text)).toBe(row.expected);});
for(const[index,row]of corpus.diff.entries())test("json diff "+index+": "+JSON.stringify(row.before)+" -> "+JSON.stringify(row.after),()=>{expect(diffJsonUloc(row.before,row.after)).toEqual(row.expected);});
for(const row of corpus.loc)test("loc "+row.path+" "+JSON.stringify(row.data),()=>{expect(countUnifiedLocForFile(row.path,row.data)).toBe(row.expected);});
for(const row of corpus.locDefects)test.failing("loc defect "+row.path+" "+JSON.stringify(row.data),()=>{expect(countUnifiedLocForFile(row.path,row.data)).toBe(row.expected);});'''
ts = lib.ts_wrap(UNIT, "🔢️ Contract: unified lines of code count JSON object keys, diff JSON by key units and count physical lines for everything else; the corpus is language-agnostic and `@@O@@` recounts with Python's `json` module.",
 'import {countJsonKeys,countUnifiedLocForFile,diffJsonUloc} from "@@' + UNIT + '/🟦.ts@@";', bodyts)
ORACLE = lib.PY_HEAD.format(doc="🐍️ Independent model of the unified line counts: JSON keys and key-unit diffs with the stdlib `json` module.") + r'''

def keys(v):
    if isinstance(v, dict):
        return len(v) + sum(keys(x) for x in v.values())
    if isinstance(v, list):
        return sum(keys(x) for x in v)
    return 0


def count_keys(text):
    t = text.strip()
    if not t:
        return 0
    try:
        return keys(json.loads(t))
    except ValueError:
        return 0


def fp(v):
    if isinstance(v, dict):
        return "object:" + "\0".join(json.dumps(k, ensure_ascii=False) for k in v)
    if isinstance(v, list):
        return "array:" + "\0".join(("[]" if isinstance(i, list) else "{}") if isinstance(i, (dict, list)) else json.dumps(i, ensure_ascii=False) for i in v)
    return json.dumps(v, ensure_ascii=False)


def units(text):
    try:
        v = json.loads(text)
    except ValueError:
        return None
    out = {}

    def walk(x, path):
        if isinstance(x, list):
            for i, item in enumerate(x):
                walk(item, "%s/%d" % (path, i))
        elif isinstance(x, dict):
            for k, item in x.items():
                p = path + "/" + json.dumps(k, ensure_ascii=False)
                out[p] = fp(item)
                walk(item, p)
    walk(v, "")
    return out or None


def diff(before, after):
    b = {} if before is None else units(before)
    a = {} if after is None else units(after)
    if b is None or a is None:
        return None
    return {"added": sum(1 for p in a if p not in b), "removed": sum(1 for p in b if p not in a), "edited": sum(1 for p in a if p in b and a[p] != b[p])}


def lines(text):
    if text == "":
        return 0
    body = text[:-2] if text.endswith("\r\n") else text[:-1] if text.endswith("\n") else text
    return len(re.split(r"\r?\n", body))


def loc(path, data):
    ext = path[path.rfind("."):].lower()
    if ext in (".json", ".jsonc"):
        k = count_keys(data)
        if k > 0:
            return k
        if not data.strip():
            return 0
    return lines(data)


for c in corpus["keys"]:
    if count_keys(c["text"]) != c["expected"]:
        failures.append("keys %r" % c["text"])
for i, c in enumerate(corpus["diff"]):
    if diff(c["before"], c["after"]) != c["expected"]:
        failures.append("diff %d" % i)
for c in corpus["loc"] + corpus["locDefects"]:
    if loc(c["path"], c["data"]) != c["expected"]:
        failures.append("loc %s %r" % (c["path"], c["data"]))
''' + lib.PY_TAIL
lib.write_unit(UNIT, corpus, schema, ts, ORACLE)
