"""📖️ Stage R: re-points the ui-runtime source-text readers at the carved crates' roots."""
import re
REPO="/Users/ueli/Documents/semio"
def edit(path,pairs):
    t=open(f"{REPO}/{path}",encoding="utf-8").read()
    for old,new in pairs:
        assert t.count(old)==1,(path,old[:90],t.count(old))
        t=t.replace(old,new)
    open(f"{REPO}/{path}","w",encoding="utf-8").write(t)
UI="🧰️framework/🔨️modules/🖱️ui/🧠️runtime"
edit("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts",[
 ('  "🧰️framework/🔨️modules/🛂️manifest/🦀️.rs",\n',f'  "🧰️framework/🔨️modules/🛂️manifest/🦀️.rs",\n  "{UI}/♻️reconcile/🦀️.rs",\n  "{UI}/⏭️cursor/🦀️.rs",\n  "{UI}/🧱️primitives/🦀️.rs",\n')])
edit("📜️script.ts",[
 (f'const INTERACTIVITY_AUDIT_UI_RECONCILE_FILE = "{UI}/♻️reconcile/🦀️.rs";\n',
  f'const INTERACTIVITY_AUDIT_UI_RECONCILE_FILE = "{UI}/♻️reconcile/🦀️.rs";\nconst INTERACTIVITY_AUDIT_UI_RECONCILE_CURSOR_FILE = "{UI}/⏭️cursor/🦀️.rs";\nconst INTERACTIVITY_AUDIT_UI_RECONCILE_PRIMITIVES_FILE = "{UI}/🧱️primitives/🦀️.rs";\n'),
 ("  const uiReconcile = policyReadRustPolicySource(repoRoot, INTERACTIVITY_AUDIT_UI_RECONCILE_FILE);\n","  const uiReconcile = policyReadUiReconcileSource(repoRoot);\n"),
 ("function policyReadRustPolicySource(repoRoot: string, sourcePath: string): string {",
  "/** ♻️ The live reconcile as the one text the unsplit module held: the authority, cursor and primitives crate roots with their mounted children. */\nfunction policyReadUiReconcileSource(repoRoot: string): string {\n  return [INTERACTIVITY_AUDIT_UI_RECONCILE_FILE, INTERACTIVITY_AUDIT_UI_RECONCILE_CURSOR_FILE, INTERACTIVITY_AUDIT_UI_RECONCILE_PRIMITIVES_FILE].map((file) => policyReadRustPolicySource(repoRoot, file)).join(\"\\n\");\n}\n\nfunction policyReadRustPolicySource(repoRoot: string, sourcePath: string): string {"),
 ("  INTERACTIVITY_AUDIT_UI_RECONCILE_FILE,\n  INTERACTIVITY_AUDIT_UI_VALUE_FILE,","  policyReadUiReconcileSource,\n  INTERACTIVITY_AUDIT_UI_RECONCILE_FILE,\n  INTERACTIVITY_AUDIT_UI_VALUE_FILE,"),
])
T="🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️interactivity-live-reconcile/🟦️.ts"
edit(T,[
 ("import { policyReadRustPolicySource, INTERACTIVITY_AUDIT_UI_RECONCILE_FILE, ","import { policyReadRustPolicySource, policyReadUiReconcileSource, "),
 ("  const reconcile = policyReadRustPolicySource(repoRoot, INTERACTIVITY_AUDIT_UI_RECONCILE_FILE);\n","  const reconcile = policyReadUiReconcileSource(repoRoot);\n"),
])
print("ok")
