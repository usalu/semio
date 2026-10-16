V = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/"
G = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/taf-snap/"
t = open(G + "verify.orig.ts", encoding="utf8", newline="").read()
start_marker = "  /** 📖️ Every `StoryScope.sourceRoots`/`storyGlobs` entry across"
end_marker = "\n}\n//#endregion 🔖️VerifyScript"
a = t.index(start_marker); b = t.index(end_marker)
block = t[a:b]
c = "\n".join(l[2:] if l.startswith("  ") else l for l in block.split("\n"))
reps = [
 ("private async checkStorybookFreshness(): Promise<void> {", "export async function checkStorybookFreshness(repoRoot: string): Promise<void> {"),
 ("private checkAppTailwindEntries(offenders: string[]): void {", "function checkAppTailwindEntries(repoRoot: string, offenders: string[]): void {"),
 ("private listAppTailwindEntries(): string[] {", "function listAppTailwindEntries(repoRoot: string): string[] {"),
 ("private findSharedUiGlobalsRel(): string | null {", "function findSharedUiGlobalsRel(repoRoot: string): string | null {"),
 ("private cssImportChainReaches(", "function cssImportChainReaches("),
 ("this.checkAppTailwindEntries(offenders)", "checkAppTailwindEntries(repoRoot, offenders)"),
 ("this.findSharedUiGlobalsRel()", "findSharedUiGlobalsRel(repoRoot)"),
 ("this.listAppTailwindEntries()", "listAppTailwindEntries(repoRoot)"),
 ("this.cssImportChainReaches(", "cssImportChainReaches("),
 ("this.root", "repoRoot"),
 ('"../../../../../../../.storybook/📖️stories/🧭️coordination/🟦️.ts"', '"../../../../../../../../.storybook/📖️stories/🧭️coordination/🟦️.ts"'),
]
for o, n in reps:
    assert o in c, o
    c = c.replace(o, n)
assert "this." not in c
doc = """/** 🖌️ `verify storybook-freshness` checks: every Storybook scope root and story glob resolves on disk, and every app Tailwind entry stylesheet resolves its relative imports and reaches the shared UI stylesheet.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🟦️.ts
 */
import { existsSync, readFileSync, readdirSync, type Dirent } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";

//#region 🔖️VerifyScript
"""
child = doc + c.rstrip("\n") + "\n//#endregion 🔖️VerifyScript\n"
p = V + "🎨️storybook-freshness/🟦️.ts"
open(p, "w", encoding="utf8", newline="").write(child)
print(len(child.splitlines()))
