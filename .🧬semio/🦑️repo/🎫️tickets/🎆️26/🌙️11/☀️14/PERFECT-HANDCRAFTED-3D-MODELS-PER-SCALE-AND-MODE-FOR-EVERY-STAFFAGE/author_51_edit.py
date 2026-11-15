import json, re, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import author_15_asset as A
def edit_member(slug, key, fn):
    path = A.ROOT % slug
    text = open(path, encoding="utf-8").read()
    indent = re.search(r'\n( +)"id"\s*:', text).group(1)
    f = re.search(r'\n' + indent + r'"' + key + r'"\s*:\s*', text)
    e = A.member_end(text, f.end())
    old = json.loads(text[f.end():e])
    new = fn(json.loads(json.dumps(old)))
    result = text[:f.end()] + A.dump(new, len(indent) // 2) + text[e:]
    json.loads(result)
    open(path, "w", encoding="utf-8").write(result)
    return old, new
