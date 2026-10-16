import sys, re, os, json
sys.path.insert(0, os.path.dirname(__file__))
from lex import mask, segments, classify
FW = "/Users/ueli/Documents/semio/🧰️framework/"
M = FW + "🔨️modules/"
FILES = {
 "root": FW + "📦️packages/🦀️rust/🦀️.rs",
 "abi": M + "🌉️abi/🦀️.rs",
 "action_bus": M + "🎯️action-bus/🦀️.rs",
 "source_projection": M + "📁️filesystem/📷️snapshot/🦀️.rs",
 "platform": M + "🖥️platform/🦀️.rs",
 "interaction": M + "🕹️interaction/🦀️.rs",
 "interaction_gesture": M + "🕹️interaction/👆️gesture/🦀️.rs",
 "interaction_schema": M + "🕹️interaction/🧬️schema/🦀️.rs",
 "kernel": M + "🎠️kernel/🦀️.rs",
 "k_clipboard": M + "🎠️kernel/📋️clipboard/🦀️.rs",
 "k_content": M + "🎠️kernel/📤️return/📦️content/🦀️.rs",
 "k_message": M + "🎠️kernel/📤️return/📦️content/💌️message/🦀️.rs",
 "k_cold": M + "🎠️kernel/📥️cold-pair/🦀️.rs",
 "manifest": M + "🛂️manifest/🦀️.rs",
 "m_canonical": M + "🛂️manifest/🌲️canonical/🦀️.rs",
 "m_media": M + "🛂️manifest/🎞️media/🦀️.rs",
 "m_space": M + "🛂️manifest/📇️space-index/🦀️.rs",
 "m_utilinit": M + "🛂️manifest/🪛️utilities/🌅️initial/🦀️.rs",
 "m_argschema": M + "🛂️manifest/🧮️arg-schema/🦀️.rs",
 "m_actions": M + "🛂️manifest/🕹️actions/🦀️.rs",
 "m_vocab": M + "🛂️manifest/🧳️product-vocabulary/🦀️.rs",
 "m_surface": M + "🛂️manifest/🧱️app-surface/🦀️.rs",
 "m_plugin": M + "🛂️manifest/🧾️plugin-manifest/🦀️.rs",
 "m_view": M + "🛂️manifest/🖼️view-model/🦀️.rs",
}
TOK = re.compile(r'[A-Za-z_][A-Za-z0-9_]*')
def load():
    data = {}
    for k, p in FILES.items():
        text = open(p, encoding='utf-8').read()
        data[k] = text
    return data
def defs(text):
    out = {}
    segs = segments(text)
    for a, b in segs:
        kind, name, vis, cl = classify(text, (a, b))
        if name and kind in ('fn','struct','enum','type','trait','static','mod','union','macro_rules!') :
            out.setdefault(name, []).append((kind, a, b))
        elif kind == 'const' or (kind=='impl'):
            pass
    # const items: classify may fold 'const' into fn path; handle regex for const/static
    return out
CONST = re.compile(r'^(?:pub(?:\([^)]*\))?\s+)?const\s+([A-Z_][A-Z0-9_]*)\s*[:=]', re.M)
STATIC = re.compile(r'^(?:pub(?:\([^)]*\))?\s+)?static\s+([A-Z_][A-Z0-9_]*)\s*[:=]', re.M)
if __name__ == '__main__':
    data = load()
    alldefs = {}
    fdefs = {}
    for k, t in data.items():
        m = mask(t)
        d = defs(t)
        names = set(d)
        for r in (CONST, STATIC):
            names |= set(r.findall(m))
        fdefs[k] = names
        for n in names:
            alldefs.setdefault(n, set()).add(k)
    graph = {}
    for k, t in data.items():
        m = mask(t)
        toks = set(TOK.findall(m))
        deps = {}
        for n in toks:
            if n in alldefs and k not in alldefs[n]:
                for o in alldefs[n]:
                    deps.setdefault(o, set()).add(n)
        graph[k] = deps
    json.dump({k: {o: sorted(v) for o, v in d.items()} for k, d in graph.items()}, open('graph.json', 'w'), indent=1, ensure_ascii=False)
    json.dump({k: sorted(v) for k, v in fdefs.items()}, open('defs.json', 'w'), indent=1, ensure_ascii=False)
    for k in data:
        print(k, len(fdefs[k]), '->', ', '.join(f"{o}({len(v)})" for o, v in sorted(graph[k].items())))
