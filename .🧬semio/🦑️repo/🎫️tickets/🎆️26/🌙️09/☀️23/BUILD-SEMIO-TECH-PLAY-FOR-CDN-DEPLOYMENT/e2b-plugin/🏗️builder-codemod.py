#!/usr/bin/env python3
"""🏗️ Rewrites every `Plugin::builder(…)` to `PluginBuilder::new(…)` so the instance-facet builder no longer needs an
inherent impl on the reactor-facet `Plugin` (the cut forbids an inherent impl outside its type's crate).

`<prefix>Plugin::<T>::builder(` becomes `<prefix'>PluginBuilder::<_, T>::new(` (the state parameter is inferred: `new`
exists only for `NeedsLabel`); a bare `Plugin::builder(` becomes `PluginBuilder::new(` and the file gains the import of
`PluginBuilder` from wherever it imports `Plugin`. Files modified in the last 20 minutes are skipped (peer guard) and
reported. Usage: builder-codemod.py <file list> [--dry]
"""
import os, re, sys, time
files = [l.strip() for l in open(sys.argv[1], encoding="utf-8") if l.strip()]
dry = "--dry" in sys.argv
REPO = "/Users/ueli/Documents/semio"
TYPED = re.compile(r"((?:[A-Za-z_]\w*::)*)Plugin::<(.+?)>::builder\(")
BARE = re.compile(r"(?<![\w:>])Plugin::builder\(")
done, skipped = 0, []
for rel in files:
    f = os.path.join(REPO, rel)
    if time.time() - os.path.getmtime(f) < 1200: skipped.append(rel); continue
    s = open(f, encoding="utf-8").read()
    def typed(m):
        prefix = m.group(1)
        prefix = {"crate::plugin_program::": "crate::builder::"}.get(prefix, prefix)
        return f"{prefix}PluginBuilder::<_, {m.group(2)}>::new("
    t = TYPED.sub(typed, s)
    bare = BARE.search(t) is not None
    t = BARE.sub("PluginBuilder::new(", t)
    if ("PluginBuilder::<_," in t and re.search(r"(?<![\w:])PluginBuilder::<_,", t)) or bare:
        if not re.search(r"\buse\b[^;]*\bPluginBuilder\b", t) and not re.search(r"(?<![\w:])PluginBuilder::<_,", s):
            m = re.search(r"(?m)^(\s*)use ((?:[\w]+::)+)(?:\{[^}]*\bPlugin\b[^}]*\}|Plugin);", t)
            if m:
                base = m.group(2)
                base = "crate::builder::" if base.startswith("crate::plugin_program") else base
                t = t[:m.end()] + f"\n{m.group(1)}use {base}PluginBuilder;" + t[m.end():]
            elif re.search(r"(?<![\w:])PluginBuilder::", t):
                t = re.sub(r"(?m)\A((?:\s*//!.*\n)*)", lambda mm: mm.group(1) + "use semio_framework_plugin::PluginBuilder;\n", t, count=1) if "semio_framework_plugin" in t else re.sub(r"(?m)\A((?:\s*//!.*\n)*)", lambda mm: mm.group(1) + "use crate::builder::PluginBuilder;\n", t, count=1)
    if t != s:
        done += 1
        if not dry: open(f, "w", encoding="utf-8").write(t)
print(f"[builder] rewrote {done} files; skipped {len(skipped)} recently modified")
for r in skipped: print("   skipped", r)
