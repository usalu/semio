"""Layer 4: `"factorOverrides"` in every committed EN 1990 JSON fixture (text insertion after the `cases` value, or after `placements` when a diff document has no cases)."""
import sys
from fo_lib import *

sys.path.insert(0, "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️scripts/placements-adder")
from padd import value_end

EMPTY_DELTA = '{"removed": [], "inserted": [], "moved": [], "modified": []}'


def insert(t):
    out, pos, hits = [], 0, 0
    for m in re.finditer(r'"placements"\s*:\s*', t):
        if m.start() < pos:
            continue
        s = m.end()
        e = value_end(t, s)
        val = t[s:e].lstrip()
        tail = re.match(r'\s*,\s*"cases"\s*:\s*', t[e:])
        if tail:
            e = value_end(t, e + tail.end())
        if re.match(r'\s*,\s*"%s"' % CAMEL, t[e:e + 80]):
            continue
        new = "[]" if val.startswith("[") else (EMPTY_DELTA if val.startswith("{") and '"removed"' in val else None)
        if new is None:
            continue
        ls = t.rfind("\n", 0, m.start()) + 1
        indent = re.match(r"\s*", t[ls:m.start()]).group(0) if t[ls:m.start()].strip() == "" else None
        sep = (",\n" + indent) if indent is not None else ", "
        colon = ": " if ": " in t[m.start():s] else ":"
        out.append(t[pos:e])
        out.append('%s"%s"%s%s' % (sep, CAMEL, colon, new))
        pos = e
        hits += 1
    out.append(t[pos:])
    return "".join(out), hits


def main():
    count = 0
    for dp, dn, fn in os.walk(SUB):
        dn[:] = [d for d in dn if d not in ("target", "node_modules", "🧬️schema", "🎬️scene")]
        for f in fn:
            if not f.endswith(".json") or f == "🎛️semantic.json":
                continue
            p = os.path.join(dp, f)
            t = rd(p)
            if '"placements"' not in t:
                continue
            n, hits = insert(t)
            if hits:
                json.loads(n)
                wr(p, n)
                count += 1
    print("  fixtures updated:", count)


if __name__ == "__main__":
    main()
