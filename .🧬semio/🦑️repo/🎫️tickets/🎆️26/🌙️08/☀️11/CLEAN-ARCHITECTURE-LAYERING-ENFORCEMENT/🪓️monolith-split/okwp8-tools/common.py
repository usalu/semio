"""🧰 Common data of the WP8 generator: bins, crate names, nodes, unit-level uses, include rebasing, region repair."""
import sys, os, re, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + '/../okwp0-tools')
import wp8lib as L, pathrw, vis, packing as P, part, unitgraph
from okrs import tokenize
FE = '️'
RSF = '🦀' + FE + '.rs'
FIX_CRATE = 'semio_framework_os_store_testing_fixtures'
M_ABS = L.R + '/' + L.M_REL
S_ABS = L.R + '/' + L.S_REL
UNIT_ABS = L.R + '/' + P.UNIT
def fe(s): return s.replace('@', FE)
# bin key -> (crate key, node relative to S_ABS)
BINS = collections.OrderedDict()
for i, emo in enumerate(['🏙', '🌆', '🌈', '🌌', '🌅', '🌴', '🌲', '🍃', '🍎', '🍯'], start=1):
    BINS['tests-%02d' % i] = ('os-store-tests-%02d' % i, '🧪' + FE + 'tests/' + emo + FE + 'bin-%02d' % i)
BINS['sync-1'] = ('os-store-sync-tests-1', '🔄' + FE + 'sync/🧪' + FE + 'tests/🍰' + FE + 'bin-1')
BINS['sync-2'] = ('os-store-sync-tests-2', '🔄' + FE + 'sync/🧪' + FE + 'tests/🥧' + FE + 'bin-2')
BINS['member-open'] = ('os-store-member-open-tests', '🧩' + FE + 'composition/🚪' + FE + 'open/🧪' + FE + 'tests/🍰' + FE + 'companion')
BINS['durable'] = ('os-store-durable-group-tests', '🧩' + FE + 'composition/🗄' + FE + 'durable-group/🧪' + FE + 'tests/🥧' + FE + 'companion')
def node_abs(bin_key): return S_ABS + '/' + BINS[bin_key][1]
def pkg_abs(bin_key): return node_abs(bin_key) + '/📦' + FE + 'packages/🦀' + FE + 'rust'
def load_plan():
    return json.load(open(L.GEN + '/okwp8-plan.json'))
INC_RX = re.compile(r'(include_str|include_bytes|include)!\(\s*(concat!\(\s*env!\("CARGO_MANIFEST_DIR"\)\s*,\s*)?"([^"]*)"(\s*\))?')
JOIN_RX = re.compile(r'(env!\("CARGO_MANIFEST_DIR"\)\)\s*\.join\(\s*")([^"]+)(")')
def rebase_includes(text, old_file, new_file, old_manifest, new_manifest):
    text = JOIN_RX.sub(lambda m: m.group(1) + os.path.relpath(os.path.normpath(old_manifest + '/' + m.group(2)), new_manifest) + m.group(3), text)
    """Rewrites include!/include_str!/include_bytes! literals (and the CARGO_MANIFEST_DIR concat form) of text moved from old_file to new_file."""
    def rel(a, b): return os.path.relpath(a, os.path.dirname(b))
    def sub(m):
        macro, cat, lit, close = m.group(1), m.group(2), m.group(3), m.group(4) or ''
        if cat:
            target = os.path.normpath(old_manifest + '/' + lit.lstrip('/'))
            new = os.path.relpath(target, new_manifest)
            return macro + '!(' + cat + '"/' + new + '"' + close
        target = os.path.normpath(os.path.join(os.path.dirname(old_file), lit))
        new = os.path.relpath(target, os.path.dirname(new_file))
        return macro + '!("' + new + '"' + close
    return INC_RX.sub(sub, text)
def repair_regions(text):
    """Makes //#region / //#endregion markers a well-nested sequence: an end closes the regions still open above its start, orphan ends are dropped, regions open at the end are closed there."""
    out = []; stack = []; closed_early = collections.Counter()
    for line in text.split('\n'):
        s = line.strip()
        if s.startswith('//#region '):
            stack.append(s[len('//#region '):]); out.append(line)
        elif s.startswith('//#endregion '):
            name = s[len('//#endregion '):]
            if name in stack:
                while stack[-1] != name:
                    y = stack.pop(); closed_early[y] += 1; out.append('//#endregion ' + y)
                stack.pop(); out.append(line)
            elif closed_early[name] > 0: closed_early[name] -= 1
            else: pass
        else: out.append(line)
    body = '\n'.join(out)
    for y in reversed(stack): body = body.rstrip('\n') + '\n//#endregion ' + y + '\n'
    return body
def docline(emoji, text): return '//! ' + emoji + FE + ' ' + text + '\n'
