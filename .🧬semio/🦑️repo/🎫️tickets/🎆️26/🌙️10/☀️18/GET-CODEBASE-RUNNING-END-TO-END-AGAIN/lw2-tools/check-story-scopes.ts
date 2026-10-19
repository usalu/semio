import fg from "fast-glob";
import { readFileSync, existsSync } from "node:fs";
import { join, relative, resolve, sep } from "node:path";
import { buildScopeStoryGlobs, resolveActiveScopes } from "../../../../../../../../.storybook/📖️stories/🧭️coordination/🟦️.ts";
const repoRoot = resolve(import.meta.dir, "../../../../../../../..");
const fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/🧫️storybook-discovery/🔣️.json"), "utf8"));
for (const id of ["puzzle", "fem", "block", "remodel"]) {
  const scopes = resolveActiveScopes(id);
  const globs = buildScopeStoryGlobs(scopes);
  const found = (await fg(globs, { cwd: join(repoRoot, ".storybook"), onlyFiles: true, unique: true })).map(p => relative(repoRoot, resolve(repoRoot, ".storybook", p)).split(sep).join("/")).sort();
  const expected = fixture.stories.map((r: any) => r.path).filter((p: string) => found.some(f => f === p) || p.includes(id === "puzzle" ? "🧩️puzzle" : id === "fem" ? "🏗️fem" : id === "block" ? "🧱️block" : "📸️remodel")).sort();
  console.log(id, "globs", JSON.stringify(globs), "found", found.length, "expected", expected.length, JSON.stringify(found) === JSON.stringify(expected) ? "MATCH" : "DIFF");
  if (JSON.stringify(found) !== JSON.stringify(expected)) { console.log(" only found:", found.filter(f => !expected.includes(f))); console.log(" only expected:", expected.filter((e: string) => !found.includes(e))); }
  for (const s of scopes) for (const r of s.sourceRoots) if (!existsSync(join(repoRoot, r))) console.log(" missing source root", s.id, r);
}
