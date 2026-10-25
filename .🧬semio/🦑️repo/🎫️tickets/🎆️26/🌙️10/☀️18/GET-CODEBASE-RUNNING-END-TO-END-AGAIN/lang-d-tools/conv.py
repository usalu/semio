import json
def edit(path, pairs):
    s = open(path, encoding="utf8").read()
    for old, new, count in pairs:
        assert s.count(old) == count, (path[-40:], old[:80], s.count(old))
        s = s.replace(old, new)
    open(path, "w", encoding="utf8").write(s)
def insert_after(path, anchor, text):
    s = open(path, encoding="utf8").read()
    assert s.count(anchor) == 1, (path[-40:], anchor[:80], s.count(anchor))
    s = s.replace(anchor, anchor + text)
    open(path, "w", encoding="utf8").write(s)
def catalog(path, messages, new_id=None):
    try:
        d = json.load(open(path, encoding="utf8"))
    except FileNotFoundError:
        d = {"version": 1, "id": new_id, "languages": ["en", "de"], "messages": {}}
    for key, (en, de) in messages.items():
        assert key not in d["messages"] or d["messages"][key] == {"en": en, "de": de}, key
        d["messages"][key] = {"en": en, "de": de}
    open(path, "w", encoding="utf8").write(json.dumps(d, ensure_ascii=False, indent=2) + "\n")
