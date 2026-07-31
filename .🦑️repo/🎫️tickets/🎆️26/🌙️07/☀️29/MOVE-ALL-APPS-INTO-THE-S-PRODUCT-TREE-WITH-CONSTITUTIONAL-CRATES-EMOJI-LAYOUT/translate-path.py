#!/usr/bin/env python3
"""Structural word->emoji path translator, reusable as a library for fixer scripts."""
import re, os

RULES = []

def rule(pattern):
    def deco(fn):
        RULES.append((re.compile(pattern), fn))
        return fn
    return deco

# ---- compose (untouched, out of scope) ----
@rule(r'^compose/(.+)$')
def _(m):
    return f"compose/{m.group(1)}"

# ---- repo (special: entry-renamed lib too) ----
@rule(r'^repo/lib/js/index\.ts$')
def _(m):
    return '🧰️/🛍️/🦑️/🔨️/lib/⚡️/🟦️/📦️.ts'

@rule(r'^repo/lib/js/(.+)$')
def _(m):
    return f"🧰️/🛍️/🦑️/🔨️/lib/⚡️/🟦️/{m.group(1)}"

@rule(r'^repo/lib/go/(.+)$')
def _(m):
    return f"🧰️/🛍️/🦑️/🔨️/lib/⚡️/🐹️/{m.group(1)}"

@rule(r'^repo/client/mcp/go/(.+)$')
def _(m):
    return f"🧰️/🛍️/🦑️/🔨️/client/mcp/⚡️/🐹️/{m.group(1)}"

@rule(r'^repo/client/(cli|sqlite|vscode)/js/(.+)$')
def _(m):
    return f"🧰️/🛍️/🦑️/🔨️/client/{m.group(1)}/⚡️/🟦️/{m.group(2)}"

@rule(r'^repo/client/(cli|sqlite|vscode)/(.+)$')
def _(m):
    return f"🧰️/🛍️/🦑️/🔨️/client/{m.group(1)}/⚡️/🟦️/{m.group(2)}"

@rule(r'^repo/server/coordinator/js/(.+)$')
def _(m):
    return f"🧰️/🛍️/🦑️/🔨️/server/coordinator/⚡️/🟦️/{m.group(1)}"

@rule(r'^repo/server/coordinator/go/(.+)$')
def _(m):
    return f"🧰️/🛍️/🦑️/🔨️/server/coordinator/⚡️/🐹️/{m.group(1)}"

@rule(r'^repo/server/lib/(.+)$')
def _(m):
    return f"🧰️/🛍️/🦑️/🔨️/server/lib/⚡️/🟦️/{m.group(1)}"

@rule(r'^repo/(.+)$')
def _(m):
    return f"🧰️/🛍️/🦑️/{m.group(1)}"

# ---- framework/product/os/module/renderer ----
@rule(r'^framework/os/renderer/wgpu/(.+)$')
def _(m):
    return f"🧰️/🛍️/💻️/🔨️/renderer/⚡️/🦀️/🧑️‍🎨️/🧊️/{m.group(1)}"

@rule(r'^framework/os/renderer/js/react/(.+)$')
def _(m):
    return f"🧰️/🛍️/💻️/🔨️/renderer/⚡️/🟦️/🧑️‍🎨️/⚛️/{m.group(1)}"

@rule(r'^framework/product/os/module/renderer/wgpu/(.+)$')
def _(m):
    return f"🧰️/🛍️/💻️/🔨️/renderer/⚡️/🦀️/🧑️‍🎨️/🧊️/{m.group(1)}"

@rule(r'^framework/product/os/module/renderer/js/react/(.+)$')
def _(m):
    return f"🧰️/🛍️/💻️/🔨️/renderer/⚡️/🟦️/🧑️‍🎨️/⚛️/{m.group(1)}"

# ---- framework/product/os/module/dev ----
@rule(r'^framework/os/dev/(.+)$')
def _(m):
    return f"🧰️/🛍️/💻️/🔨️/dev/⚡️/🟦️/{m.group(1)}"

@rule(r'^framework/product/os/module/dev/js/(.+)$')
def _(m):
    return f"🧰️/🛍️/💻️/🔨️/dev/⚡️/🟦️/{m.group(1)}"

@rule(r'^framework/product/os/module/dev/(.+)$')
def _(m):
    return f"🧰️/🛍️/💻️/🔨️/dev/⚡️/🟦️/{m.group(1)}"

# ---- framework/product/os/module/plugin/registry ----
@rule(r'^framework/plugin/registry/(.+)$')
def _(m):
    return f"🧰️/🛍️/💻️/🔨️/plugin/⚡️/🟦️/registry/{m.group(1)}"

@rule(r'^framework/product/os/module/plugin/registry/(.+)$')
def _(m):
    return f"🧰️/🛍️/💻️/🔨️/plugin/⚡️/🟦️/registry/{m.group(1)}"

# ---- framework/product/os/module/infinite ----
@rule(r'^framework/os/kernel/infinite/canvas/react-renderer/(.+)$')
def _(m):
    return f"🧰️/🛍️/💻️/🔨️/infinite/⚡️/🟦️/canvas/react-renderer/{m.group(1)}"

@rule(r'^framework/os/kernel/infinite/world/r3f/(.+)$')
def _(m):
    return f"🧰️/🛍️/💻️/🔨️/infinite/⚡️/🟦️/world/r3f/{m.group(1)}"

@rule(r'^framework/product/os/module/infinite/canvas/react-renderer/(.+)$')
def _(m):
    return f"🧰️/🛍️/💻️/🔨️/infinite/⚡️/🟦️/canvas/react-renderer/{m.group(1)}"

@rule(r'^framework/product/os/module/infinite/world/r3f/(.+)$')
def _(m):
    return f"🧰️/🛍️/💻️/🔨️/infinite/⚡️/🟦️/world/r3f/{m.group(1)}"

# ---- framework/product/os/module/flow ----
@rule(r'^framework/os/kernel/flow/core/rs/pkg/(.+)$')
def _(m):
    return f"🧰️/🛍️/💻️/🔨️/flow/⚡️/🦀️/core/pkg/{m.group(1)}"

@rule(r'^framework/product/os/module/flow/core/rs/pkg/(.+)$')
def _(m):
    return f"🧰️/🛍️/💻️/🔨️/flow/⚡️/🦀️/core/pkg/{m.group(1)}"

@rule(r'^framework/product/os/module/flow/core/(.+)$')
def _(m):
    return f"🧰️/🛍️/💻️/🔨️/flow/⚡️/🟦️/core/{m.group(1)}"

@rule(r'^framework/product/os/module/flow/module/(.+)$')
def _(m):
    return f"🧰️/🛍️/💻️/🔨️/flow/⚡️/🦀️/module/{m.group(1)}"

# ---- framework/product/os/module/<other> ----
@rule(r'^framework/product/os/module/([a-z]+)/(.+)$')
def _(m):
    return f"🧰️/🛍️/💻️/🔨️/{m.group(1)}/⚡️/🦀️/{m.group(2)}"

@rule(r'^framework/product/os/(rs|js)/(.+)$')
def _(m):
    lang = '🦀️' if m.group(1) == 'rs' else '🟦️'
    return f"🧰️/🛍️/💻️/⚡️/{lang}/{m.group(2)}"

@rule(r'^framework/product/os$')
def _(m):
    return "🧰️/🛍️/💻️"

@rule(r'^framework/product/print/(.+)$')
def _(m):
    return f"🧰️/🛍️/📓️/⚡️/🟦️/{m.group(1)}"

# ---- framework/module ----
@rule(r'^framework/module/ui/js/react/(.+)$')
def _(m):
    return f"🧰️/🔨️/ui/⚡️/🟦️/react/{m.group(1)}"

@rule(r'^framework/ui/js/react/(.+)$')
def _(m):
    return f"🧰️/🔨️/ui/⚡️/🟦️/react/{m.group(1)}"

@rule(r'^framework/module/ui/asset/(.+)$')
def _(m):
    return f"🧰️/🔨️/ui/asset/⚡️/🟦️/{m.group(1)}"

@rule(r'^framework/ui/asset/(.+)$')
def _(m):
    return f"🧰️/🔨️/ui/asset/⚡️/🟦️/{m.group(1)}"

@rule(r'^framework/module/ui/styling/(.+)$')
def _(m):
    return f"🧰️/🔨️/ui/⚡️/🟦️/styling/{m.group(1)}"

@rule(r'^framework/ui/styling/(.+)$')
def _(m):
    return f"🧰️/🔨️/ui/⚡️/🟦️/styling/{m.group(1)}"

@rule(r'^framework/module/math/graph/dsl/core/(.+)$')
def _(m):
    return f"🧰️/🔨️/math/⚡️/🟦️/graph/dsl/core/{m.group(1)}"

@rule(r'^framework/module/math/(.+)$')
def _(m):
    return f"🧰️/🔨️/math/⚡️/🦀️/{m.group(1)}"

@rule(r'^framework/module/surface/(.+)$')
def _(m):
    return f"🧰️/🔨️/surface/⚡️/🦀️/{m.group(1)}"

@rule(r'^framework/module/asset/(.+)$')
def _(m):
    return f"🧰️/🔨️/asset/⚡️/🟦️/{m.group(1)}"

@rule(r'^framework/asset/(.+)$')
def _(m):
    return f"🧰️/🔨️/asset/⚡️/🟦️/{m.group(1)}"

@rule(r'^framework/module/([a-z]+)/(.+)$')
def _(m):
    return f"🧰️/🔨️/{m.group(1)}/⚡️/🦀️/{m.group(2)}"

@rule(r'^framework/(rs|js)/(.+)$')
def _(m):
    lang = '🦀️' if m.group(1) == 'rs' else '🟦️'
    return f"🧰️/⚡️/{lang}/{m.group(2)}"

@rule(r'^framework$')
def _(m):
    return "🧰️"

# ---- s/module ----
@rule(r'^s/module/2d/js/(.+)$')
def _(m):
    return f"✏️/🔨️/2d/⚡️/🟦️/{m.group(1)}"

@rule(r'^s/module/3d/brep/js/(.+)$')
def _(m):
    return f"✏️/🔨️/3d/⚡️/🟦️/brep/{m.group(1)}"

@rule(r'^s/module/([a-z0-9]+)/(.+)$')
def _(m):
    return f"✏️/🔨️/{m.group(1)}/⚡️/🦀️/{m.group(2)}"

# ---- s/plugin: manifest/artifact, extension, module, app ----
@rule(r'^s/plugin/([^/]+)/manifest/artifact/rs/(.+)$')
def _(m):
    return f"✏️/🔌️/{m.group(1)}/🛂️/🗿️/⚡️/🦀️/{m.group(2)}"

@rule(r'^s/plugin/([^/]+)/extension/([^/]+)/rs/(.+)$')
def _(m):
    return f"✏️/🔌️/{m.group(1)}/🧩️/{m.group(2)}/⚡️/🦀️/{m.group(3)}"

@rule(r'^s/plugin/puzzle/module/asset/(.+)$')
def _(m):
    return f"✏️/🔌️/puzzle/🔨️/asset/⚡️/🟦️/{m.group(1)}"

@rule(r'^s/plugin/([^/]+)/module/([^/]+)/rs/(.+)$')
def _(m):
    return f"✏️/🔌️/{m.group(1)}/🔨️/{m.group(2)}/⚡️/🦀️/{m.group(3)}"

@rule(r'^s/plugin/animate/app/present/js/(.+)$')
def _(m):
    return f"✏️/🔌️/animate/🎛️/present/⚡️/🟦️/{m.group(1)}"

@rule(r'^s/plugin/([^/]+)/app/([^/]+)/(engine|dsl|op|pack|protocol|ui)/rs/(.+)$')
def _(m):
    return f"✏️/🔌️/{m.group(1)}/🎛️/{m.group(2)}/🔨️/{m.group(3)}/⚡️/🦀️/{m.group(4)}"

@rule(r'^s/plugin/([^/]+)/app/([^/]+)/rs/(.+)$')
def _(m):
    return f"✏️/🔌️/{m.group(1)}/🎛️/{m.group(2)}/⚡️/🦀️/{m.group(3)}"

@rule(r'^s/plugin/([^/]+)/app/([^/]+)/(.+)$')
def _(m):
    return f"✏️/🔌️/{m.group(1)}/🎛️/{m.group(2)}/{m.group(3)}"

@rule(r'^s/plugin/cad/(core|runtime|query|renderer|stately|brepjs)/(.+)$')
def _(m):
    return f"✏️/🔌️/cad/🔨️/{m.group(1)}/⚡️/🟦️/{m.group(2)}"

@rule(r'^s/plugin/([^/]+)/(.+)$')
def _(m):
    return f"✏️/🔌️/{m.group(1)}/{m.group(2)}"

@rule(r'^s$')
def _(m):
    return "✏️"

@rule(r'^s/(.+)$')
def _(m):
    return f"✏️/{m.group(1)}"

# ---- hub, mit-bestand ----
@rule(r'^hub/(.+)$')
def _(m):
    return f"🌎️/{m.group(1)}"

@rule(r'^mit-bestand/(.+)$')
def _(m):
    return f"♻️/{m.group(1)}"


def _rename_entry(path):
    if path.endswith('/index.ts') or path == 'index.ts':
        return path[: -len('index.ts')] + '📦️.ts'
    if path.endswith('/index.tsx') or path == 'index.tsx':
        return path[: -len('index.tsx')] + '📦️.tsx'
    return path


def translate(path):
    for pattern, handler in RULES:
        m = pattern.match(path)
        if m:
            return _rename_entry(handler(m))
    return None


if __name__ == '__main__':
    import sys
    for line in sys.stdin:
        line = line.rstrip('\n')
        if not line:
            continue
        print(translate(line))
