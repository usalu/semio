"""📝 C5C source-text reader re-points: every `readFileSync(<root file of a split module>, "utf8")` of a TS law becomes `readModuleTreeSource(<same path>)` (the module tree is the text the unsplit file held). Each edit asserts a unique anchor; the import of the module-tree reader is added where the file lacks it (relative path computed from the file). usage: tsedit.py [--apply]"""
import os, re, sys
R = "/Users/ueli/Documents/semio/"
M = "🧰️framework/🛍️products/💻️os/🔨️modules/"
PL = M + "🔌️plugin/"
TREE = R + "🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts"
EDITS = {
    M + "🏪️store/🕰️history/📐️planning/🧪️tests/🟦️.ts": [
        ('readFileSync(new URL("../../../../🔌️plugin/⏪️time-travel/🦀️.rs",import.meta.url),"utf8")', 'readModuleTreeSource(new URL("../../../../🔌️plugin/⏪️time-travel/🦀️.rs",import.meta.url))')],
    M + "📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎬️media-slots/🎟️collect/🧪️tests/🟦️.ts": [
        ('const turn=read("⚛️reactor/🔄️turn/🦀️.rs");', 'const turn=readModuleTreeSource(`${base}/⚛️reactor/🔄️turn/🦀️.rs`);')],
    PL + "🏇️mounted-owner/🧪️tests/🟦️.ts": [
        ('turn=readFileSync(resolve(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs"),"utf8")', 'turn=readModuleTreeSource(resolve(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs"))'),
        ('turn=readFileSync(resolve(plugin,"⚛️reactor/🔄️turn/🦀️.rs"),"utf8")', 'turn=readModuleTreeSource(resolve(plugin,"⚛️reactor/🔄️turn/🦀️.rs"))')],
    PL + "🧩️extension/🧪️tests/📨️invoke/🟦️.ts": [
        ('reactor=read("⚛️reactor/🔄️turn/🦀️.rs")', 'reactor=readModuleTreeSource(resolve(root,"⚛️reactor/🔄️turn/🦀️.rs"))')],
    PL + "🧪️tests/🔬️plugin-runtime-runtime-close-budget/🟦️.ts": [
        ('readFileSync(new URL("../../⚛️reactor/🔄️turn/🦀️.rs",import.meta.url),"utf8")', 'readModuleTreeSource(new URL("../../⚛️reactor/🔄️turn/🦀️.rs",import.meta.url))')],
    PL + "🧪️tests/🧪️reactor-contract-oracles/🟦️.ts": [
        ('readFileSync(new URL("../../⚛️reactor/🔄️turn/🦀️.rs", import.meta.url), "utf8")', 'readModuleTreeSource(new URL("../../⚛️reactor/🔄️turn/🦀️.rs", import.meta.url))')],
    PL + "🧪️tests/🧪️time-travel/🟦️.ts": [
        ('readFileSync(join(repoRoot, PLUGIN_ROOT, "⏪️time-travel/🦀️.rs"), "utf8")', 'readModuleTreeSource(join(repoRoot, PLUGIN_ROOT, "⏪️time-travel/🦀️.rs"))')],
    R + "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/🧬️retained-clone-check/🟦️.ts": [
        ('readFileSync(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🦀️.rs"), "utf8")', 'readModuleTreeSource(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🦀️.rs"))')],
    PL + "🛠️tool-machine/♻️retirement/🧪️tests/🟦️.ts": [
        ('readFileSync(new URL("../../../⏯️tool-run/🦀️.rs",import.meta.url),"utf8")', 'readModuleTreeSource(new URL("../../../⏯️tool-run/🦀️.rs",import.meta.url))'),
        ('readFileSync(new URL("../../../⏯️tool-run/🧵️instance/🦀️.rs",import.meta.url),"utf8")', 'readModuleTreeSource(new URL("../../../⏯️tool-run/🧵️instance/🦀️.rs",import.meta.url))')],
    PL + "⏪️time-travel/🧪️tests/🧩️history-partition/🟦️.ts": [
        ('const read = (rel: string): string => readFileSync(join(pluginRoot, rel), "utf8");', 'const read = (rel: string): string => readModuleTreeSource(join(pluginRoot, rel));')],
}
apply = "--apply" in sys.argv
bad = 0
for path, edits in EDITS.items():
    p = R + path if not path.startswith("/") else path
    text = open(p, encoding="utf-8").read()
    for old, new in edits:
        n = text.count(old)
        if n < 1:
            print("MISSING anchor", p.split("/")[-3], old[:70]); bad += 1; continue
        print(f"{p.split('/')[-3]}/{p.split('/')[-2]}: {n} x {old[:60]}")
        text = text.replace(old, new)
    if "readModuleTreeSource" in text and not re.search(r"^import\b[^\n]*readModuleTreeSource", text, flags=re.M):
        rel = os.path.relpath(TREE, os.path.dirname(p))
        imp = f'import {{ readModuleTreeSource }} from "{rel}";\n'
        lines = text.split("\n")
        idx = max(i for i, l in enumerate(lines) if re.match(r"^import\b", l) and l.rstrip().endswith(";")) if any(re.match(r"^import\b", l) for l in lines) else -1
        lines.insert(idx + 1, imp.rstrip("\n"))
        text = "\n".join(lines)
        print("   + import", rel[-40:])
    if apply and not bad:
        open(p, "w", encoding="utf-8").write(text)
print("bad", bad, "applied" if apply and not bad else "dry")
