import re, os
V = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/"
p = V + "🟦️.ts"
t = open(p, encoding="utf8", newline="").read()
open("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/taf-snap/verify.orig.ts", "w", encoding="utf8", newline="").write(t)
start_marker = "  /** 📖️ Every `StoryScope.sourceRoots`/`storyGlobs` entry across"
end_marker = "\n}\n//#endregion 🔖️VerifyScript"
assert t.count(start_marker) == 1 and t.count(end_marker) == 1
a = t.index(start_marker)
b = t.index(end_marker)
block = t[a:b]
assert block.rstrip().endswith("}")
head = t[:a].rstrip("\n") + "\n"
tail = t[b:]
newparent = head + tail.lstrip("\n") if False else head + tail[1:] if False else None
newparent = t[:a].rstrip("\n") + t[b:]
call_old = "    await this.checkStorybookFreshness();"
assert newparent.count(call_old) == 1
newparent = newparent.replace(call_old, "    await checkStorybookFreshness(this.root);")
imp_anchor = 'import { dirname, join, relative, resolve } from "node:path";\n'
assert newparent.count(imp_anchor) == 1
newparent = newparent.replace(imp_anchor, imp_anchor + 'import { checkStorybookFreshness } from "./🎨️storybook-freshness/🟦️.ts";\n')
# convert the block to module functions
lines = block.split("\n")
out = []
for l in lines:
    out.append(l[2:] if l.startswith("  ") else l)
c = "\n".join(out)
reps = [
 ("private async checkStorybookFreshness(): Promise<void> {", "export async function checkStorybookFreshness(root: string): Promise<void> {"),
 ("private checkAppTailwindEntries(offenders: string[]): void {", "function checkAppTailwindEntries(root: string, offenders: string[]): void {"),
 ("private listAppTailwindEntries(): string[] {", "function listAppTailwindEntries(root: string): string[] {"),
 ("private findSharedUiGlobalsRel(): string | null {", "function findSharedUiGlobalsRel(root: string): string | null {"),
 ("private cssImportChainReaches(", "function cssImportChainReaches("),
 ("this.checkAppTailwindEntries(offenders)", "checkAppTailwindEntries(root, offenders)"),
 ("this.findSharedUiGlobalsRel()", "findSharedUiGlobalsRel(root)"),
 ("this.listAppTailwindEntries()", "listAppTailwindEntries(root)"),
 ("this.cssImportChainReaches(", "cssImportChainReaches("),
 ("this.root", "root"),
 ('"../../../../../../../.storybook/📖️stories/🧭️coordination/🟦️.ts"', '"../../../../../../../../.storybook/📖️stories/🧭️coordination/🟦️.ts"'),
]
for o, n in reps:
    assert o in c, o
    c = c.replace(o, n)
assert "this." not in c
doc = """/** 🎨️ `verify storybook-freshness` checks: every Storybook scope root and story glob resolves on disk, and every app Tailwind entry stylesheet resolves its relative imports and reaches the shared UI stylesheet.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🟦️.ts
 */
import { existsSync, readFileSync, readdirSync, type Dirent } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";

//#region 🔖️VerifyScript
"""
child = doc + c.rstrip("\n") + "\n//#endregion 🔖️VerifyScript\n"
os.makedirs(V + "🎨️storybook-freshness", exist_ok=True)
assert not os.path.exists(V + "🎨️storybook-freshness/🟦️.ts")
open(V + "🎨️storybook-freshness/🟦️.ts", "w", encoding="utf8", newline="").write(child)
open(p, "w", encoding="utf8", newline="").write(newparent)
print(len(newparent.splitlines()), len(child.splitlines()))
