# 🔎 Lists every `crate::` / `$crate::` path of the WP1 file sets by head and tail (token level, comments and strings excluded).
import os, sys, collections, re
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from okrs import tokenize
M = '/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/'
SETS = {
 'list-delta': ['🪡️list-delta/🦀️.rs'],
 'semio': ['🧬️semio/🦀️.rs'],
 'space-sqlite-cells': ['🪐️space/🪶️sqlite/🦀️.rs'],
 'plugin-registry-schema': ['🔌️plugin/📇️registry/📦️deployment/🧬️schema/🦀️.rs', '🔌️plugin/📇️registry/📥️runtime/🦀️.rs'],
 'extension': ['🧩️extension/🦀️.rs'],
 'pack-tools': ['🎒️pack/⌨️cli/🦀️.rs', '🎒️pack/🚦️control/🦀️.rs'],
 'spr': ['📡️spr/🦀️.rs', '📡️spr/🔌️io/🦀️.rs', '📡️spr/💎️materialize/🦀️.rs'],
 'spr-cli': ['📡️spr/⌨️cli/🦀️.rs'],
 'dsl': ['🗣️dsl/🦀️.rs', '🗣️dsl/🫴️receiving/🦀️.rs', '🗣️dsl/📖️grammar/🦀️.rs', '🗣️dsl/🧠️lsp/🦀️.rs', '🗣️dsl/📇️registry/🦀️.rs'] + ['🗣️dsl/👪️family/' + f + '/🦀️.rs' for f in ('🗂️catalog', '📎️embed', '🌍️geo', '🕸️graph', '🧑‍🍳recipe', '🎬️scene', '📊️sheet')],
}
which = sys.argv[1:] or list(SETS)
for crate in which:
    heads = collections.defaultdict(collections.Counter)
    for rel in SETS[crate]:
        src = open(M + rel, encoding='utf8').read()
        toks = [t for t in tokenize(src) if t[0] not in ('ws', 'comment')]
        i = 0
        while i < len(toks):
            if toks[i][0] == 'id' and toks[i][1] == 'crate' and i + 2 < len(toks) and toks[i + 1][1] == ':' and toks[i + 2][1] == ':':
                j = i + 3; path = []
                if j < len(toks) and toks[j][1] == '{':
                    path = ['{']
                else:
                    while j < len(toks) and toks[j][0] == 'id':
                        path.append(toks[j][1]); j += 1
                        if j + 1 < len(toks) and toks[j][1] == ':' and toks[j + 1][1] == ':': j += 2
                        else: break
                heads[path[0] if path else ''][tuple(path[1:3])] += 1
            i += 1
    print('#####', crate)
    for h, c in sorted(heads.items()):
        print(' ', h, sum(c.values()), ' '.join(f"{'::'.join(k)}x{v}" for k, v in sorted(c.items())[:25]))
