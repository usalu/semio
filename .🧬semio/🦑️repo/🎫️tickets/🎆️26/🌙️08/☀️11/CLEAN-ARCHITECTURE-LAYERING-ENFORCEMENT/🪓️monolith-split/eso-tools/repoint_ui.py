import os, re, sys
R = "/Users/ueli/Documents/semio"
LEAF = R + "/🧰️framework/🔨️modules/🫧️transient/🟦️.ts"
UI = R + "/🧰️framework/🔨️modules/🖱️ui/"
NAMES = {"ephemeralBox", "ephemeralMap", "ephemeralSet", "EphemeralBox"}
files = [
 "🎯️targets/⚛️react/🟦️.tsx", "🎯️targets/⚛️react/🌐️i18n/🟦️.ts", "🎯️targets/⚛️react/🌓️appearance/🟦️.ts",
 "🔨️modules/🕹️control-keybinding-context/🟦️.tsx", "🔨️modules/🗣️ui-terminology/🟦️.tsx", "🔨️modules/🛟️chrome-safe-area/🟦️.tsx", "🔨️modules/🧲️panel-dock/🟦️.tsx",
 "🧱️elements/🌈️Surface/🟦️.tsx", "🧱️elements/🌳️Tree/🟦️.tsx", "🧱️elements/🎨️Canvas/🟦️.tsx", "🧱️elements/🎬️Scene/🟦️.tsx",
 "🧱️elements/🐚️ShellScope/🟦️.tsx", "🧱️elements/🔣️Icons/🟦️.tsx", "🧱️elements/🚗️UiDriver/🟦️.tsx",
]
pat = re.compile(r'import\s*\{([^}]*)\}\s*from\s*"@semio-tech/ui-styling";')
for rel in files:
    p = UI + rel
    s = open(p, encoding="utf8").read()
    ms = [m for m in pat.finditer(s) if any(re.sub(r'^type\s+', '', x.strip()) in NAMES for x in m.group(1).split(",") if x.strip())]
    assert len(ms) == 1, (rel, len(ms))
    m = ms[0]
    parts = [x.strip() for x in m.group(1).split(",") if x.strip()]
    eph = [x for x in parts if re.sub(r'^type\s+', '', x) in NAMES]
    rest = [x for x in parts if x not in eph]
    spec = os.path.relpath(LEAF, os.path.dirname(p))
    new_leaf = 'import { %s } from "%s";' % (", ".join(eph), spec)
    multiline = "\n" in m.group(0)
    if rest:
        if multiline:
            old = m.group(0)
            head = old[:old.index("{")+1]
            body = "\n" + "".join("  %s,\n" % x for x in rest)
            repl = head + body + '} from "@semio-tech/ui-styling";'
        else:
            repl = 'import { %s } from "@semio-tech/ui-styling";' % ", ".join(rest)
        repl += "\n" + new_leaf
    else:
        repl = new_leaf
    s2 = s[:m.start()] + repl + s[m.end():]
    open(p, "w", encoding="utf8").write(s2)
    print("ok", rel, "->", spec, "| moved:", eph, "| kept:", len(rest))
# styling-internal
for rel, old in [("🎨️styling/🌓️theme/🟦️.ts", '"../🫧️ephemeral-state/🟦️.ts"'), ("🎨️styling/🏗️builder/🌐️vite/🟦️.ts", '"../../🫧️ephemeral-state/🟦️.ts"')]:
    p = UI + rel
    s = open(p, encoding="utf8").read()
    assert s.count(old) == 1, rel
    spec = os.path.relpath(LEAF, os.path.dirname(p))
    open(p, "w", encoding="utf8").write(s.replace(old, '"%s"' % spec))
    print("ok", rel, "->", spec)
