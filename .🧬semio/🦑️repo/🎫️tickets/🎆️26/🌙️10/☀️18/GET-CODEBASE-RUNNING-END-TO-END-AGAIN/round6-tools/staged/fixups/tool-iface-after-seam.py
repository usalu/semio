#!/usr/bin/env python3
"""Re-bases the tool-iface Bun.spawn site edits on the text runtime-seam leaves (`runtimeSpawnV1([...argv], options)` / `runtimeSpawnSyncV1`) and on the typegen exporter that now launches Cargo through the repo launcher (its edits are dropped); the runtime-spawn import is removed where no use remains.
usage: tool-iface-after-seam.py <tool-iface copy>"""
import re, sys

path = sys.argv[1] + "/build/edits_bun.py"
text = open(path, encoding="utf8").read()
pairs = [
    ('const result = Bun.spawn(["bun", join(this.root, "node_modules/vitest/vitest.mjs"), "run",', 'const result = runtimeSpawnV1([runtimeExecutableV1(), join(this.root, "node_modules/vitest/vitest.mjs"), "run",'),
    ('const result = Bun.spawn([process.execPath, "test", join(browserBundleDirectory', 'const result = runtimeSpawnV1([process.execPath, "test", join(browserBundleDirectory'),
    ("child=Bun.spawn([process.execPath,compiler", "child=runtimeSpawnV1([process.execPath,compiler"),
    ("const result=Bun.spawnSync([process.execPath,step.path", "const result=runtimeSpawnSyncV1([process.execPath,step.path"),
    ('const child = Bun.spawn(["rustc"', 'const child = runtimeSpawnV1(["rustc"'),
]
for old, new in pairs:
    assert text.count(old) == 1, old
    text = text.replace(old, new)
vitest_new = "'const result = startToolV1(\"vitest\", [\"run\",'"
assert text.count(vitest_new) == 1
start = text.index("    TYPEGEN: with_import(TYPEGEN,")
end = text.index("    ]),\n", start) + len("    ]),\n")
text = text[:start] + text[end:]
old_def = "def with_import(path, names, edits):\n    return [*edits, lambda text: ensure_import(path, text, names)]\n"
assert text.count(old_def) == 1
new_def = '''def drop_unused_runtime_spawn(text):
    import re
    for name in ("runtimeSpawnV1", "runtimeSpawnSyncV1"):
        if text.count(name) == 1:
            text = re.sub(r"^import \\{ " + name + r" \\} from \\"[^\\"]+\\";\\n", "", text, flags=re.M)
    return text

def with_import(path, names, edits):
    return [*edits, lambda text: ensure_import(path, text, names), drop_unused_runtime_spawn]
'''
text = text.replace(old_def, new_def)
open(path, "w", encoding="utf8").write(text)
print("tool-iface-after-seam: spawn sites re-based")
