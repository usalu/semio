"""🧱️ The cross-crate surface of every WP5 seam on the staged tree: the top-level names a definer file set defines (with their current visibility) that a consumer file set uses, so the flip knows exactly which items must become `pub` (`#[doc(hidden)]` where the facade must not re-export). usage: python3 surface5.py [--json out]"""
import json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
import lib5 as L, plan_tools as pt
from rsitems import split_items
P = L.PLUG + '/'
def files(*rels):
    out = []
    for r in rels:
        out.append(P + r)
    return out
SEAMS = {
 'time-travel-panel -> time-travel (root + turns)': (['⏪️time-travel/🪟️panel/🔢️pointer/🦀️.rs', '⏪️time-travel/🪟️panel/🖼️sections/🦀️.rs', '⏪️time-travel/🪟️panel/🛬️lifted/🎮️command/🦀️.rs'], ['⏪️time-travel/🦀️.rs', '⏪️time-travel/🧵️instance/🦀️.rs', '⏪️time-travel/♻️retirement/🦀️.rs', '⏪️time-travel/🎮️decision/🗑️discard/🦀️.rs']),
 'command lift -> panel + time-travel (root + turns)': (['🎮️command/🛬️lifted/⏪️time-travel/🦀️.rs'], ['⏪️time-travel/🪟️panel/🔢️pointer/🦀️.rs', '⏪️time-travel/🪟️panel/🖼️sections/🦀️.rs', '⏪️time-travel/🪟️panel/🛬️lifted/🎮️command/🦀️.rs', '⏪️time-travel/🦀️.rs', '⏪️time-travel/🧵️instance/🦀️.rs']),
 'command lift -> command root and instance family': (['🎮️command/🛬️lifted/⏪️time-travel/🦀️.rs'], ['🎮️command/🦀️.rs', '🧵️instance/📨️dispatch/🦀️.rs', '🧵️instance/🦀️.rs', '🧵️instance/🚪️ingress/🦀️.rs', '🪜️maintenance-ladder/🦀️.rs', '👷️job/🦀️.rs', '🦀️.rs']),
 'time-travel root -> job facet, ladders, instance family': (['⏪️time-travel/🦀️.rs'], ['👷️job/🦀️.rs', '🧵️instance/🚪️ingress/🦀️.rs', '🪜️maintenance-ladder/🦀️.rs', '🧵️instance/🦀️.rs', '🧵️instance/📨️dispatch/🦀️.rs', '🦀️.rs']),
 'tool-run -> time-travel root': (['⏯️tool-run/🦀️.rs'], ['⏪️time-travel/🦀️.rs', '⏪️time-travel/🧵️instance/🦀️.rs']),
 'job lift -> job root and runtime-ops': (['👷️job/🛬️lifted/⏯️tool-run/🦀️.rs'], ['👷️job/🦀️.rs', '⚛️reactor/🏃️runtime/🧰️ops/🛡️wire-limits/🦀️.rs', '⚛️reactor/🏃️runtime/🧰️ops/🧾️typed-operations/🦀️.rs', '⚛️reactor/🏃️runtime/🧰️ops/📥️ingress/🦀️.rs']),
}
DEF = re.compile(r'^((?:pub(?:\([^)]*\))?\s+)?)(?:(?:const|unsafe|async)\s+)*(fn|struct|enum|const|static|type|trait|union)\s+([A-Za-z_]\w*)')
def defined(path):
    out = {}
    if not os.path.exists(path): return out
    for it in split_items(L.rd(path)):
        if it.kind in ('marker', 'innerdoc', 'use', 'other', 'impl'): continue
        for l in it.text.split('\n'):
            s = l.strip()
            if not s or s.startswith(('//', '#[', '/*', '*')): continue
            m = DEF.match(s)
            if m: out[m.group(3)] = (m.group(1).strip() or 'private', m.group(2))
            break
    return out
def main():
    res = {}
    for seam, (defs, cons) in SEAMS.items():
        d = {}
        for f in defs: d.update(defined(P + f))
        ids = set()
        for f in cons:
            q = P + f
            if os.path.exists(q): ids |= set(re.findall(r'[A-Za-z_]\w*', pt.strip(L.rd(q))))
        used = {n: v for n, v in d.items() if n in ids and n not in pt.GENERIC and len(n) > 3}
        need = {n: v for n, v in used.items() if v[0] != 'pub'}
        res[seam] = dict(defined=len(d), used=len(used), widen_to_pub=sorted(need), already_pub=sorted(n for n, v in used.items() if v[0] == 'pub'))
        print(f'{seam}: defines {len(d)}, named across the seam {len(used)}, not yet pub {len(need)}')
        for n in sorted(need): print('     ', need[n][0], need[n][1], n)
    if '--json' in sys.argv: json.dump(res, open(sys.argv[sys.argv.index('--json') + 1], 'w'), ensure_ascii=False, indent=1)
main()
