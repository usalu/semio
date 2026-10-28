"""🚧 [isolate] batch `fence`: anchored edits of the existing files (process materialized-path fence + its re-exports, the EN/DE refusal text, the test preload entry) from <src> into <dst>; exit 2 names a lost anchor. usage: fence_existing.py <src-root> <dst-root>"""
import os, sys
src, dst = sys.argv[1:3]
def edit(rel, pairs):
    s = open(os.path.join(src, rel), encoding="utf-8").read()
    for old, new in pairs:
        if s.count(old) != 1: sys.stderr.write("ANCHOR %s %d: %s\n" % (rel[-60:], s.count(old), old[:90])); sys.exit(2)
        s = s.replace(old, new)
    os.makedirs(os.path.dirname(os.path.join(dst, rel)), exist_ok=True); open(os.path.join(dst, rel), "w", encoding="utf-8").write(s); print("fence ->", rel[-80:])
edit("🧰️framework/🔨️modules/🏃️process/🔒️leases/🎛️control/🟦️.ts", [
 ('import { dirname, resolve } from "node:path";', 'import { dirname, resolve, sep } from "node:path";'),
 ('''/** 📏️ Checks the complete resolved materialized path in both portable string measures. */
export function requireMaterializedPathV1(path: string): string {
  const absolute = resolve(path);
  if (absolute.includes("\\0") || absolute.length > 256 || [...absolute].length > 256) throw Error("Materialized absolute path exceeds 256 characters");
  return absolute;
}''', '''/** 🚧️ The directories this process must never materialize a path in, each with its refusal; one table per process on a registered symbol, so every loaded copy of this leaf shares it. */
const fencedDirectories: Map<string, string> = ((globalThis as Record<symbol, Map<string, string> | undefined>)[Symbol.for("semio.process.materialized-path-fences.v1")] ??= new Map());
/** 🚧️ Fences directories for the rest of this process: every later materialized path at or below one of them is refused with its reason (a test process fences the checkout's live store this way). */
export function fenceMaterializedPathsV1(directories: readonly string[], reason: string): void {
  for (const directory of directories) fencedDirectories.set(resolve(directory), reason);
}

/** 📏️ Checks the complete resolved materialized path in both portable string measures and refuses one inside a fenced directory. */
export function requireMaterializedPathV1(path: string): string {
  const absolute = resolve(path);
  if (absolute.includes("\\0") || absolute.length > 256 || [...absolute].length > 256) throw Error("Materialized absolute path exceeds 256 characters");
  for (const [directory, reason] of fencedDirectories) if (absolute === directory || absolute.startsWith(directory + sep)) throw Error(reason + ": " + absolute);
  return absolute;
}''')])
edit("🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts", [('export {checkResourceLeaseControl,watchResourceLeaseControlDeadlineV1,requireMaterializedPathV1,', 'export {checkResourceLeaseControl,watchResourceLeaseControlDeadlineV1,fenceMaterializedPathsV1,requireMaterializedPathV1,')])
edit("🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts", [(" formatLeaseViolationV1,", " fenceMaterializedPathsV1, formatLeaseViolationV1,")])
edit("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/📖️texts/🔣️.json", [('    "workspace.nested": {', '    "store.fenced": { "en": "A test reached the live repository store of the checkout {root}; give it a private repository workspace (privateRepositoryWorkspaceV1)", "de": "Ein Test hat den aktiven Repository-Speicher des Checkouts {root} erreicht; gib ihm einen privaten Repository-Arbeitsbereich (privateRepositoryWorkspaceV1)" },\n    "workspace.nested": {')])
edit("bunfig.toml", [('"./🧰️framework/🔨️modules/🏃️process/🧪️testing/🟨️bun/🪦️orphans/📜️script.ts"]', '"./🧰️framework/🔨️modules/🏃️process/🧪️testing/🟨️bun/🪦️orphans/📜️script.ts", "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🚧️fence/📜️script.ts"]')])
