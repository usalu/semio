"""✅ c4p16 checks over a tree: reachability, duplicate mounts, docstring emoji uniqueness in the crate closure, regions, test names, sizes.
usage: c4p16check.py live|stage   (live = repo tree of the installed split, stage = scratch stage dir)"""
import collections, importlib, json, os, re, sys
T = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, T)
import c4p16 as C

REPO = C.REPO
S = '✏️s/🔌️plugins/🗄️stdio'
A = S + '/🗿️artifacts'
ROOTS = {
 'part21': S + '/📇️registry/🧬️contract/📐️part21/🦀️.rs',
 'document': S + '/🔮️oracles/📃️document/🦀️.rs',
 'header_diff': [A + '/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🧬️schema/📸️snapshot/📦️packages/🦀️rust/🦀️.rs'],
 'zip_io': [A + '/🎒️zip/🦀️.rs'],
 'mp4_io': [A + '/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🔬️unit/📦️packages/🦀️rust/🦀️.rs'],
 'bcf_oracles': [A + '/💬️bcf/🔮️oracles/🦀️.rs'],
 'step_geometry': [A + '/📐️step/🦀️.rs'],
 'xlsx_oracles': [A + '/📕️xlsx/🔮️oracles/🦀️.rs'],
}
RX_TEST = re.compile(r"#\[\s*(?:[\w:]*::)?(?:test|async_test|tokio::test)\b[^\]]*\]\s*(?:#\[[^\]]*\]\s*|//[^\n]*\n\s*)*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+(\w+)")
EMOJI_FIRST = re.compile(r"^//[!/]? (\S+)")
PATHRX = re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]')
INCRX = re.compile(r'\binclude!\(\s*"([^"]+)"\s*\)')


def read(p):
    return open(p, encoding='utf-8').read()


def mounts(text, base):
    out = []
    for m in PATHRX.finditer(text):
        out.append(os.path.normpath(os.path.join(base, m.group(1))))
    for m in INCRX.finditer(text):
        out.append(os.path.normpath(os.path.join(base, m.group(1))))
    return out


def closure(lib):
    seen, order, cnt = set(), [], collections.Counter()

    def visit(p):
        p = os.path.normpath(p)
        cnt[p] += 1
        if p in seen or not os.path.isfile(p) or not p.endswith('.rs'):
            return
        seen.add(p)
        order.append(p)
        for t in mounts(read(p), os.path.dirname(p)):
            visit(t)
    visit(lib)
    return order, cnt


def regions(ls):
    o, c = collections.Counter(), collections.Counter()
    for l in ls:
        m = re.match(r'^\s*//\s?#region (.*)$', l)
        if m:
            o[m.group(1).strip()] += 1
        m = re.match(r'^\s*//\s?#endregion (.*)$', l)
        if m:
            c[m.group(1).strip()] += 1
    return o, c


def main(mode):
    ok = True
    for key, libs in ROOTS.items():
        if isinstance(libs, str):
            libs = [libs]
        sp = importlib.import_module('specs.' + key).SPEC
        rels = C.unit_files(sp)
        base = os.path.dirname(sp['file'])
        if mode == 'live':
            files = {r: os.path.join(REPO, base, r) for r in rels}
        else:
            files = {r: os.path.join(C.SCR, 'stage', key, r) for r in rels}
        orig = C.load_orig(sp)
        newfiles = [r for r in rels if r != '🦀️.rs']
        report = []
        for lib in libs:
            libabs = os.path.join(REPO, lib)
            cl, cnt = closure(libabs)
            if mode == 'stage':
                report.append('(stage mode: reachability not checked)')
                break
            cl_rel = [os.path.relpath(p, REPO) for p in cl]
            first = collections.defaultdict(list)
            for p in cl_rel:
                ls = read(os.path.join(REPO, p)).split('\n')
                m = EMOJI_FIRST.match(ls[0])
                if m and not m.group(1).isascii():
                    first[m.group(1)].append(p)
            bad = []
            for r in newfiles:
                rp = os.path.relpath(files[r], REPO)
                n = cnt.get(os.path.normpath(files[r]), 0)
                if n != 1:
                    bad.append(('reach', r[-60:], n))
                m = EMOJI_FIRST.match(read(files[r]).split('\n')[0])
                e = m.group(1) if m else None
                if not e or e.isascii():
                    bad.append(('no-emoji', r[-60:]))
                elif len(first[e]) > 1 and rp in first[e]:
                    bad.append(('emoji-repeats', e, [x[-70:] for x in first[e]]))
            missing = []
            for p in cl:
                for t in mounts(read(p), os.path.dirname(p)):
                    if not os.path.isfile(t) and not os.path.isdir(t):
                        missing.append((os.path.relpath(p, REPO)[-60:], os.path.relpath(t, REPO)[-60:]))
            report.append((lib[-60:], len(cl), bad, missing))
            if bad or missing:
                ok = False
        o_reg = regions(orig.split('\n'))
        tot_o = sum(sum(regions(read(files[r]).split('\n'))[0].values()) for r in rels)
        tot_c = sum(sum(regions(read(files[r]).split('\n'))[1].values()) for r in rels)
        a = sorted(m.group(1) for m in RX_TEST.finditer(orig))
        b = sorted(m.group(1) for r in rels for m in RX_TEST.finditer(read(files[r])))
        longest = max(len(os.path.join(base, r).encode('utf-8')) for r in rels)
        biggest = max((read(files[r]).count('\n'), r[-50:]) for r in rels)
        bal = (tot_o == tot_c)
        if a != b or not bal or longest > 240:
            ok = False
        print(f'[{key}] files={len(rels)} regions-open={tot_o} close={tot_c} (orig {sum(o_reg[0].values())}) tests {len(a)}=={len(b)} identical={a == b} longest-path-bytes={longest} biggest={biggest}')
        for r in report:
            print('   ', r)
    print('CHECKS', 'OK' if ok else 'FAILED')
    return ok


if __name__ == '__main__':
    sys.exit(0 if main(sys.argv[1] if len(sys.argv) > 1 else 'stage') else 1)
