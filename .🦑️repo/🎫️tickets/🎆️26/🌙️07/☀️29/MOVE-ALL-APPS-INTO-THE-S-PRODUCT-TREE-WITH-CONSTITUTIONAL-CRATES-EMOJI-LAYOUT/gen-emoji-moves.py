#!/usr/bin/env python3
"""Generate moves.json for the emoji-layout restructuring, rule-based over the live crate inventory."""
import json, re, os

crates = {}
for line in open('/tmp/all_crates_emoji.txt'):
    line = line.rstrip('\n')
    if not line:
        continue
    d, name = line.split('\t')
    crates[d] = name

LANG = {'rs': '🦀️', 'js': '🟦️', 'py': '🐍️', 'go': '🐹️', 'net': '🔷️', 'rb': '💎️'}

moves = {}
unmatched = []

def add(old, new):
    assert old in crates, f"not a known crate dir: {old}"
    if old in moves and moves[old] != new:
        raise AssertionError(f"conflict for {old}: {moves[old]} vs {new}")
    moves[old] = new

# Ordered list of (regex, handler) — handler(m) -> new_dir. First match wins; try longest/most-specific patterns first.
RULES = []

def rule(pattern):
    def deco(fn):
        RULES.append((re.compile(pattern), fn))
        return fn
    return deco

# ---- framework general ----
@rule(r'^framework/rs$')
def _(m):
    return '🧰️/⚡️/🦀️'

@rule(r'^framework/module/math/(?P<sub>.+)/rs$')
def _(m):
    return f"🧰️/🔨️/math/⚡️/🦀️/{m.group('sub')}"

@rule(r'^framework/module/surface/(?P<name>[^/]+)/rs$')
def _(m):
    return f"🧰️/🔨️/surface/⚡️/🦀️/{m.group('name')}"

@rule(r'^framework/module/ui/(?P<sub>styling|tui|wgpu)/rs$')
def _(m):
    return f"🧰️/🔨️/ui/⚡️/🦀️/{m.group('sub')}"

@rule(r'^framework/module/(?P<mod>editor|hash|schema)/rs$')
def _(m):
    return f"🧰️/🔨️/{m.group('mod')}/⚡️/🦀️"

# ---- framework/product/os ----
@rule(r'^framework/product/os/rs$')
def _(m):
    return '🧰️/🛍️/💻️/⚡️/🦀️'

@rule(r'^framework/product/os/module/renderer/wgpu/rs$')
def _(m):
    return '🧰️/🛍️/💻️/🔨️/renderer/⚡️/🦀️/🧑️‍🎨️/🧊️'

@rule(r'^framework/product/os/module/plugin/(?P<sub>host/)?rs$')
def _(m):
    sub = m.group('sub') or ''
    return f"🧰️/🛍️/💻️/🔨️/plugin/⚡️/🦀️/{sub}".rstrip('/')

@rule(r'^framework/product/os/module/(?P<mod>db|dsl|flow|infinite|neural|pack|protocol|run|store|vcs|workflow)/(?P<sub>.+)/rs$')
def _(m):
    return f"🧰️/🛍️/💻️/🔨️/{m.group('mod')}/⚡️/🦀️/{m.group('sub')}"

@rule(r'^framework/product/os/module/(?P<mod>db|dsl|flow|infinite|neural|pack|protocol|run|store|vcs|workflow)/rs$')
def _(m):
    return f"🧰️/🛍️/💻️/🔨️/{m.group('mod')}/⚡️/🦀️"

# ---- hub ----
@rule(r'^hub/rs$')
def _(m):
    return '🌎️/⚡️/🦀️'

@rule(r'^hub/directory/(?P<sub>[^/]+)/rs$')
def _(m):
    return f"🌎️/🔨️/directory/⚡️/🦀️/{m.group('sub')}"

@rule(r'^hub/directory/rs$')
def _(m):
    return '🌎️/🔨️/directory/⚡️/🦀️'

@rule(r'^s/plugin/energy/engine/rs$')
def _(m):
    return '✏️/🔌️/energy/🔨️/engine/⚡️/🦀️'

# ---- repo (moves under 🧰️/🛍️/🦑️) ----
@rule(r'^repo/cli/rs$')
def _(m):
    return '🦑️/🔨️/cli/⚡️/🦀️'

# ---- s/module ----
@rule(r'^s/module/2d/engine/rs$')
def _(m):
    return '✏️/🔨️/2d/🔨️/engine/⚡️/🦀️'

@rule(r'^s/module/2d/rs$')
def _(m):
    return '✏️/🔨️/2d/⚡️/🦀️'

@rule(r'^s/module/3d/(?P<sub>.+)/rs$')
def _(m):
    return f"✏️/🔨️/3d/⚡️/🦀️/{m.group('sub')}"

@rule(r'^s/module/(?P<mod>imperative|mindmap)/rs$')
def _(m):
    return f"✏️/🔨️/{m.group('mod')}/⚡️/🦀️"

# ---- s/plugin ----
@rule(r'^s/plugin/(?P<p>[^/]+)/manifest/artifact/rs$')
def _(m):
    return f"✏️/🔌️/{m.group('p')}/🛂️/🗿️/⚡️/🦀️"

@rule(r'^s/plugin/(?P<p>[^/]+)/extension/(?P<e>[^/]+)/rs$')
def _(m):
    return f"✏️/🔌️/{m.group('p')}/🧩️/{m.group('e')}/⚡️/🦀️"

@rule(r'^s/plugin/(?P<p>[^/]+)/module/(?P<sub>.+)/rs$')
def _(m):
    return f"✏️/🔌️/{m.group('p')}/🔨️/{m.group('sub')}/⚡️/🦀️"

@rule(r'^s/plugin/(?P<p>[^/]+)/app/(?P<a>[^/]+)/(?P<slot>engine|dsl|op|pack|protocol|ui)/rs$')
def _(m):
    return f"✏️/🔌️/{m.group('p')}/🎛️/{m.group('a')}/🔨️/{m.group('slot')}/⚡️/🦀️"

@rule(r'^s/plugin/(?P<p>[^/]+)/app/(?P<a>[^/]+)/rs$')
def _(m):
    return f"✏️/🔌️/{m.group('p')}/🎛️/{m.group('a')}/⚡️/🦀️"

for old in sorted(crates):
    matched = False
    for pattern, handler in RULES:
        m = pattern.match(old)
        if m:
            add(old, handler(m))
            matched = True
            break
    if not matched:
        unmatched.append(old)

print(f"total crates: {len(crates)}")
print(f"matched: {len(moves)}")
print(f"unmatched: {len(unmatched)}")
for u in unmatched:
    print(f"  UNMATCHED: {u}  ({crates[u]})")

newdirs = list(moves.values())
dups = set(x for x in newdirs if newdirs.count(x) > 1)
if dups:
    print("DUPLICATE TARGETS:", dups)

out = [{"oldDir": k, "newDir": v, "newPkg": crates[k]} for k, v in moves.items()]
with open('.repo/🎫️/26/07/29/MOVE-ALL-APPS-INTO-THE-S-PRODUCT-TREE-WITH-CONSTITUTIONAL-CRATES-EMOJI-LAYOUT/emoji-moves.json', 'w') as f:
    json.dump(out, f, indent=2, ensure_ascii=False)
