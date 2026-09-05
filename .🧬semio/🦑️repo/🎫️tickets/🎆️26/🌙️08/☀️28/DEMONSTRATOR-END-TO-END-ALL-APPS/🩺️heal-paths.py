#!/usr/bin/env python3
"""🩺️ Repoints broken `#[path]` attributes at directories a peer renamed.

A rename in this repo only ever changes a leading emoji, never the ASCII tail
(`✳️mesh` -> `🔺️mesh`, `🗂add-layer` -> `🗃️add-layer`). So a broken segment is healed ONLY when
exactly one sibling on disk shares its ASCII tail. Ambiguous or unmatched segments are reported and
left alone -- never guessed, so this cannot invent a target the way a blind emoji-strip would.
"""
import os, io, re, sys, glob

ASCII = re.compile(r"[A-Za-z0-9][A-Za-z0-9._-]*$")

def tail(name):
    m = ASCII.search(name)
    return m.group(0) if m else None

def heal(rel, base):
    """Walk `rel` component by component, swapping any missing component for its unique tail-match."""
    parts = os.path.normpath(os.path.join(base, rel)).split(os.sep)
    cur, changed = os.sep if os.path.isabs(os.path.join(base, rel)) else "", False
    for part in parts:
        nxt = os.path.join(cur, part) if cur else part
        if os.path.exists(nxt):
            cur = nxt
            continue
        parent = cur if cur else "."
        if not os.path.isdir(parent):
            return None, False
        want = tail(part)
        cands = [e for e in os.listdir(parent) if want and tail(e) == want and e != part]
        if len(cands) != 1:
            return None, False
        cur, changed = os.path.join(parent, cands[0]), True
    return cur, changed

def main(paths):
    total_fixed = 0
    for root in paths:
        base = os.path.dirname(root)
        src = io.open(root, encoding="utf-8").read()
        out, fixed, unresolved = src, 0, []
        for rel in set(re.findall(r'#\[path\s*=\s*"([^"]+)"\]', src)):
            if os.path.exists(os.path.normpath(os.path.join(base, rel))):
                continue
            healed, changed = heal(rel, base)
            if not changed or healed is None:
                unresolved.append(rel)
                continue
            new_rel = os.path.relpath(healed, base)
            if rel.startswith("./") and not new_rel.startswith("."):
                new_rel = "./" + new_rel
            out = out.replace(f'#[path = "{rel}"]', f'#[path = "{new_rel}"]')
            fixed += 1
        if fixed:
            io.open(root, "w", encoding="utf-8").write(out)
            print(f"  healed {fixed:3} in {root.split('/')[2]}")
            total_fixed += fixed
        for u in unresolved:
            print(f"  UNRESOLVED {root.split('/')[2]}: {u[-90:]}")
    print(f"total healed: {total_fixed}")
    return total_fixed

def discover(roots):
    """🔎️ Every `🦀️.rs` beneath `roots`, skipping build output trees."""
    found = []
    for root in roots:
        if os.path.isfile(root):
            found.append(root)
            continue
        for dp, dn, fn in os.walk(root):
            dn[:] = [d for d in dn if d not in ("target", "node_modules", "dist", "🕸️bindings", "pkg")]
            found.extend(os.path.join(dp, f) for f in fn if f == "🦀️.rs")
    return sorted(set(found))

if __name__ == "__main__":
    args = sys.argv[1:]
    if args and args[0] == "--scan":
        main(discover(args[1:]))
    else:
        main(args or sorted(glob.glob("✏️s/🔌️plugins/*/📦️packages/🦀️rust/🦀️.rs")))
