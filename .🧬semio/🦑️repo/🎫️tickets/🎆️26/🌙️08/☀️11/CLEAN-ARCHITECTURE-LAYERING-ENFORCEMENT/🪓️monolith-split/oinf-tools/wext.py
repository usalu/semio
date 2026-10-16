"""Names that code outside the infinite crate imports from the world facade."""
import re, os, json, collections
R = "/Users/ueli/Documents/semio/"
GEN = R + ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
def external_world_names():
    files = [l.rstrip('\n') for l in open(GEN + '/oinf-ext-users.txt', encoding='utf-8')]
    names = collections.defaultdict(set); paths = set()
    pat = re.compile(r'(?:infinite_world|semio_framework_os_infinite|infinite)::world(::\{(?:[^{}]|\{[^{}]*\})*\}|(?:::[A-Za-z_0-9]+)+)', re.S)
    files = files + ['🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/♻️resources/🧪️tests/🦀️.rs']
    for f in files:
        try: t = open(R + f, encoding='utf-8').read()
        except Exception: continue
        for m in pat.finditer(t):
            body = m.group(1)
            for idn in re.findall(r'[A-Za-z_][A-Za-z_0-9]*', body):
                names[idn].add(f)
    return names
if __name__ == '__main__':
    n = external_world_names()
    print(len(n)); print(sorted(n))
