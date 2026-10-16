import re, sys
K = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/"
files = {"🎛️activation-registry": "🧪️RuntimeMetricsTests", "🔌️plugin-source": "🧪️SharedWatchStreamTests", "🗂️plugin-registry": "🧪️ExpandPluginRegistryTests", "🚪️io-router": "🧪️IoRouterTests", "🧭️app-router": "🧪️AppRouterTests", "📡️turn-outcome-broadcast": "🧪️TurnOutcomeBroadcastTests"}
ref = '/// <reference types="vitest/importMeta" />\n'
for d, name in files.items():
    p = K + d + "/🟦️.ts"
    t = open(p, encoding="utf8", newline="").read()
    assert t.startswith(ref), d
    t = t[len(ref):]
    pat = re.compile(r"\n?//#region " + re.escape(name) + r"\n(.*?)//#endregion " + re.escape(name) + r"\n", re.S)
    ms = pat.findall(t)
    assert len(ms) == 1, (d, len(ms))
    assert ms[0].count("if (import.meta.vitest) {") == 1 and len(re.findall(r"registerTests\d", ms[0])) == 2, (d, ms[0].count("import.meta.vitest"))
    t = pat.sub("", t, count=1)
    assert "import.meta.vitest" not in t and "createturnoutcomebroadcast" not in t, d
    open(p, "w", encoding="utf8", newline="").write(t)
    print("ok", d, len(t.splitlines()))
