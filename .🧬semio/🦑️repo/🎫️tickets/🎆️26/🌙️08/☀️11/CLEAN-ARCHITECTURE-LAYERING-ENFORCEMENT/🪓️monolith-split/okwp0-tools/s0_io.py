# ✂️ S0, lossless staging split of the only non-store multi-owner file: M/🚪️io/🦀️.rs (os-io 1639 item lines / os-store-codec-registry 531).
# The runs of Appendix C (line numbers stale by +1) are re-derived by item names. The codec-registry items move verbatim into the staging file
# M/🏪️store/📦️codec/📥️lifted/🚪️io/🦀️.rs that the original `include!`s (one module, so no visibility changes); `//#region` marker lines stay in the original and are repeated, balanced, around the moved blocks.
# usage: s0_io.py dry|apply
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from okrs import items
import okedit
R = '/Users/ueli/Documents/semio'
M = R + '/🧰️framework/🛍️products/💻️os/🔨️modules/'
SRC = M + '🚪️io/🦀️.rs'
STAGE = M + '🏪️store/📦️codec/📥️lifted/🚪️io/🦀️.rs'
INCLUDE = 'include!("../🏪️store/📦️codec/📥️lifted/🚪️io/🦀️.rs");'
# (crate, first item name) in file order, from Appendix C
RUNS = [('io', 'AnalyzeSource'), ('codec', 'Confidence'), ('io', 'Analysis'), ('codec', 'ComposeError'), ('io', 'compose_thunk'), ('codec', 'ComposerEntry'), ('io', 'preflight_composer_entries'),
        ('codec', 'SubsetValidator'), ('io', 'SnapshotSubsetValidator'), ('codec', 'SnapshotSubsetValidation'), ('io', 'validate_payload_thunk'), ('codec', 'SUBSET_VALIDATOR_REGISTRY'),
        ('io', 'SubsetValidationError'), ('codec', 'same_subset_validator_entry'), ('io', 'register_subset_validator'), ('codec', 'validate_subset_validators'), ('io', 'preflight_subset_validators'),
        ('codec', 'FormatDescriptor'), ('io', 'register_format_descriptors'), ('codec', 'FormatDescriptorIndexes'), ('io', 'format_descriptor'), ('codec', 'ArtifactAssemblyRegistryPlan'), ('io', 'io_mechanism')]
MARK = re.compile(r'^\s*//#(end)?region\b')

def plan(src):
    lines = src.split('\n')
    its = items(src)
    def ln(off): return src.count('\n', 0, off) + 1
    spans = [(ln(it['start']), ln(it['end']), it['name'], it['kind']) for it in its]
    firsts = []
    for crate, name in RUNS:
        idx = next(i for i, s in enumerate(spans) if s[2] == name and (not firsts or i > firsts[-1][1]))
        firsts.append((crate, idx))
    group = ['io'] * len(spans)
    for k, (crate, idx) in enumerate(firsts):
        end = firsts[k + 1][1] if k + 1 < len(firsts) else len(spans)
        for i in range(idx, end): group[i] = crate
    for i, s in enumerate(spans):
        if s[3] == 'mod' and s[2] == 'bindings': group[i] = 'codec'
    return lines, spans, group

LIT = re.compile(r'(#\[\s*path\s*=\s*\s*"|include(?:_str|_bytes)?!\(\s*")([^"]+)"')
def rebase(text):
    def sub(m):
        target = os.path.normpath(os.path.join(os.path.dirname(SRC), m.group(2)))
        return m.group(1) + os.path.relpath(target, os.path.dirname(STAGE)) + '"'
    return LIT.sub(sub, text)

def build(src):
    lines, spans, group = plan(src)
    moved = set(); blocks = []
    for (a, b, name, kind), g in zip(spans, group):
        if g != 'codec': continue
        rng = [l for l in range(a, b + 1) if not MARK.match(lines[l - 1])]
        blocks.append((a, b, rng))
        moved.update(rng)
    # open-region stack before a line
    def stack_at(line):
        st = []
        for l in range(1, line):
            m = MARK.match(lines[l - 1])
            if m:
                name = lines[l - 1].strip()[len('//#region '):] if not m.group(1) else None
                if m.group(1): st.pop()
                else: st.append(name)
        return st
    out_blocks = []
    cur = None
    for a, b, rng in blocks:
        if not rng: continue
        first = rng[0]
        st = stack_at(first)
        text = rebase('\n'.join(lines[l - 1] for l in rng))
        if cur and cur['stack'] == st and first - cur['last'] <= 4: cur['texts'].append(text); cur['last'] = rng[-1]
        else:
            if cur: out_blocks.append(cur)
            cur = {'stack': st, 'texts': [text], 'last': rng[-1]}
    if cur: out_blocks.append(cur)
    stage = ['// 📥️ Lifted from `🚪️io/🦀️.rs` by S0: the composer registry, `IoKey`, subset validators, format catalog and the native-snapshot registration plan that belong to `os-store-codec-registry`.',
             '// The original `include!`s this file, so every item stays in the `io` module until WP4 turns the file into the crate.', '']
    for blk in out_blocks:
        for name in blk['stack']: stage.append('//#region ' + name)
        stage.append('\n\n'.join(blk['texts']))
        for name in reversed(blk['stack']): stage.append('//#endregion ' + name)
        stage.append('')
    first_codec_after_macros = next(a for (a, b, n, k), g in zip(spans, group) if g == 'codec' and n == 'ComposerEntry')
    keep = []
    placed = False
    for l in range(1, len(lines) + 1):
        if l in moved:
            if not placed and l >= first_codec_after_macros:
                keep.append('// 📥️ S0: the registry half of this file lives in the lifted file below.')
                keep.append(INCLUDE); placed = True
            continue
        keep.append(lines[l - 1])
    assert placed
    return '\n'.join(keep), '\n'.join(stage).rstrip('\n') + '\n', group, spans

def main():
    mode = sys.argv[1]
    src = open(SRC, encoding='utf-8').read()
    if INCLUDE in src: print('already split'); return
    new_src, stage, group, spans = build(src)
    from collections import Counter
    def lines_of(t): return Counter(LIT.sub(lambda m: m.group(1) + '<lit>"', l) for l in t.split('\n') if l.strip())
    orig = lines_of(src)
    got = lines_of(new_src) + lines_of(stage)
    for l in ('// 📥️ S0: the registry half of this file lives in the lifted file below.', LIT.sub(lambda m: m.group(1) + '<lit>"', INCLUDE)): got[l] -= 1
    for l in [x for x in stage.split('\n') if x.startswith('// ')]: got[l] -= 1
    # region markers are repeated around moved blocks
    extra = got - orig; lost = orig - got
    extra = Counter({k: v for k, v in extra.items() if not MARK.match(k)})
    print('items', len(spans), 'codec items', sum(1 for g in group if g == 'codec'), 'orig lines', len(src.split('\n')), '->', len(new_src.split('\n')), '+ stage', len(stage.split('\n')), 'lost', sum(lost.values()), 'extra(non-marker)', sum(extra.values()))
    for k, v in list(lost.items())[:8]: print('  lost', v, k[:100])
    for k, v in list(extra.items())[:8]: print('  extra', v, k[:100])
    def balance(t):
        d = 0
        for l in t.split('\n'):
            m = MARK.match(l)
            if m: d += -1 if m.group(1) else 1
            assert d >= 0
        return d
    print('balance original', balance(new_src), 'stage', balance(stage))
    if mode == 'apply':
        assert not lost and not extra and balance(new_src) == 0 and balance(stage) == 0
        assert open(SRC, encoding='utf-8').read() == src
        assert not os.path.exists(STAGE)
        okedit.write(SRC, new_src); okedit.write(STAGE, stage)
        print('applied')
main()
