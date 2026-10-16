import sys, os
R = "/Users/ueli/Documents/semio/"
def edit(path, pairs):
    t = open(R + path, encoding='utf-8').read()
    for old, new in pairs:
        if t.count(old) != 1: raise SystemExit(f'anchor count {t.count(old)}: {old[:90]}')
        t = t.replace(old, new)
    open(R + path, 'w', encoding='utf-8').write(t)
edit('📜️script.ts', [
 ('const manifest = policyReadModuleTreeSource(root, "🧰️framework/🔨️modules/🛂️manifest/🦀️.rs");', 'const manifest = policyReadFileSafe(root, "🧰️framework/🔨️modules/🛂️manifest/🕹️actions/🦀️.rs");'),
 ('const INTERACTIVITY_AUDIT_KERNEL_FILE = "🧰️framework/🔨️modules/🎠️kernel/🦀️.rs";', 'const INTERACTIVITY_AUDIT_KERNEL_FILE = "🧰️framework/🔨️modules/🎠️kernel/🦀️.rs";\nconst INTERACTIVITY_AUDIT_KERNEL_UI_FILE = "🧰️framework/🔨️modules/🎠️kernel/🖼️ui/🦀️.rs";'),
 ('  const kernel = policyReadRustPolicySource(repoRoot, INTERACTIVITY_AUDIT_KERNEL_FILE);\n  const kernelTurn = kernel.slice(kernel.indexOf("//#region 🔖️TurnResult"), kernel.indexOf("//#endregion 🔖️TurnResult"));', '  const kernelTurn = [INTERACTIVITY_AUDIT_KERNEL_UI_FILE, INTERACTIVITY_AUDIT_KERNEL_FILE].map((file) => { const text = policyReadRustPolicySource(repoRoot, file); return text.slice(text.indexOf("//#region 🔖️TurnResult"), text.indexOf("//#endregion 🔖️TurnResult")); }).join("\\n");'),
 ('  INTERACTIVITY_AUDIT_KERNEL_FILE,\n', '  INTERACTIVITY_AUDIT_KERNEL_FILE,\n  INTERACTIVITY_AUDIT_KERNEL_UI_FILE,\n'),
])
edit('🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts', [
 ('  "🧰️framework/🔨️modules/🛂️manifest/🦀️.rs",\n', ''),
])
print('ok')
