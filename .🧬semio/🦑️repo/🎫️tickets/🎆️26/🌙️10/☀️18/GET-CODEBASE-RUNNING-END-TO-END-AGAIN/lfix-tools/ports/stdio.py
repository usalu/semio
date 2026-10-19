import sys, re, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from wrapper import *
ROOT = '/Users/ueli/Documents/semio/'
SCR = '/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad/'
ROOTS = ('app', 'reopened', 'source', 'target', 'fresh', 'other', 'peer')
ANCHOR = r'let mut (?:app|reopened|source|target|fresh|other|peer)\b'
CXT = "semio_framework_plugin::StepContext<'_>"

def policy_of(s):
    m = re.search(r'close_registered_fixture_app\(&mut \w+,\s*((?:[^()]|\([^()]*\))+?)\)\s*;', s)
    return m.group(1).strip() if m else POLICY

def helper_names(s):
    return sorted(set(re.findall(r'fn (\w+)\s*(?:<[^>]*>)?\s*\(\s*app:\s*&mut\s', s)))

def media_rewrite(s, pol):
    close = f'{pol}.close'
    s = re.sub(r'(\w+)\.submit_media_export\(((?:[^()]|\([^()]*\))+?)\)\.await', lambda m: f'{LAWS}::submit_fixture_media_export(&mut {m.group(1)}, {m.group(2)}, scope.cx(), {close}).await', s)
    s = re.sub(r'(\w+)\.poll_media_export\(&(\w+)\)\.await', lambda m: f'{LAWS}::poll_fixture_media_export(&mut {m.group(1)}, &{m.group(2)}, scope.cx(), {close}).await', s)
    s = re.sub(r'(\w+)\.retain_media_export_result\(&(\w+),\s*(\w+)\)\.await', lambda m: f'{LAWS}::retain_fixture_media_export_result(&mut {m.group(1)}, &{m.group(2)}, {m.group(3)}, scope.cx(), {close}).await', s)
    s = re.sub(r'(\w+)\.take_media_export_chunk\(&(\w+)\)\.await', lambda m: f'{LAWS}::take_fixture_media_export_chunk(&mut {m.group(1)}, &{m.group(2)}, &mut download_terminal, scope.cx(), {close}).await', s)
    lines = s.split('\n'); out = []
    for ln in lines:
        if 'take_fixture_media_export_chunk' in ln and 'while let' in ln:
            ind = re.match(r'[ \t]*', ln).group(0)
            out.append(f'{ind}let mut download_terminal = false;')
        out.append(ln)
    s = '\n'.join(out)
    s = re.sub(r'async \|(\w+)\| (\w+)\.produce_media\(((?:[^()]|\([^()]*\))+?),\s*\1\)\.await', lambda m: f'async |{m.group(1)}| {LAWS}::produce_fixture_media(&mut {m.group(2)}, {m.group(3)}, {m.group(1)}, scope.cx(), {close}).await', s)
    return s

def process(path):
    s = read(path)
    pol = policy_of(s)
    helpers = helper_names(s)
    raw_scoped_port(path, helpers, top_roots=ROOTS, anchor=ANCHOR, policy=pol, cx_type=CXT)
    s = read(path)
    s = media_rewrite(s, pol)
    prof = Profile(lambda r: r, lambda r: 'scope.cx()', policy=pol, roots=ROOTS)
    s, a = method_rewrite(s, prof, 'close_step', 1, lambda a, r: None if 'cx' in a[0] else [prof.cx(r), a[0]])
    s, b = method_rewrite(s, prof, 'consume_media', 2, lambda a, r: a + [prof.cx(r)])
    for m in list(re.finditer(r'fn \w+\s*(?:<[^>]*>)?\s*\(', s))[::-1]:
        o = m.end() - 1; c = match_paren(s, o)
        if c < 0 or 'cx: &mut ' + CXT not in s[o:c]:
            continue
        bb = s.find('{', c); e = match_paren(s, bb)
        if bb < 0 or e < 0:
            continue
        s = s[:bb] + s[bb:e + 1].replace('scope.cx()', 'cx') + s[e + 1:]
    write(path, s)
    return pol, helpers, a, b

if __name__ == '__main__':
    for p in sys.argv[1:]:
        print(p[-70:], process(p))
