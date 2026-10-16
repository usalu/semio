"""Names that code outside the infinite crate imports from the board family (any alias, any path)."""
import re, os, collections, sys
R = "/Users/ueli/Documents/semio/"
SC = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/"
CRATE = 'semio_framework_os_infinite'

def users():
    fs = [l.strip() for l in open(SC + 'inf-users.txt', encoding='utf-8', errors='replace')]
    return [f for f in fs if '♾️infinite/' not in f and '🎫️tickets' not in f]

BASE_ALIASES = {CRATE, 'infinite_canvas', 'semio_framework_os_kernel_infinite_board_port_directed_dag', 'semio_framework_os_kernel_infinite_canvas', 'semio_framework_kernel_infinite_world', 'infinite_board_port_directed', 'infinite_board_port_directed_normal'}
def aliases(t):
    al = set(BASE_ALIASES)
    for b in list(BASE_ALIASES):
        for m in re.finditer(r'extern\s+crate\s+' + b + r'\s+as\s+(\w+)\s*;', t): al.add(m.group(1))
        for m in re.finditer(r'\buse\s+' + b + r'\s+as\s+(\w+)\s*;', t): al.add(m.group(1))
    return al

def tree_names(body):
    return re.findall(r'[A-Za-z_][A-Za-z_0-9]*', body)

def scan():
    out = collections.defaultdict(lambda: collections.defaultdict(set))   # path-prefix -> name -> files
    globs = collections.defaultdict(set)
    for f in users():
        try: t = open(R + f, encoding='utf-8').read()
        except Exception: continue
        for al in aliases(t):
            for m in re.finditer(r'(?<![A-Za-z0-9_:])' + re.escape(al) + r'::((?:[a-z_][a-z0-9_]*::)*)(\{(?:[^{}]|\{[^{}]*\})*\}|\*|[A-Za-z_][A-Za-z_0-9]*)', t, re.S):
                prefix = m.group(1).rstrip(':')
                body = m.group(2)
                if body == '*': globs[prefix].add(f); continue
                if body.startswith('{'):
                    for n in tree_names(body): out[prefix][n].add(f)
                else: out[prefix][body].add(f)
    return out, globs

if __name__ == '__main__':
    out, globs = scan()
    for pre in sorted(out):
        print(repr(pre), len(out[pre]), sorted(out[pre])[:50])
    print('globs', {k: len(v) for k, v in globs.items()})
