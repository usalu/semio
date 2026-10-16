// 🔌️ Lifts the exclusion entries that hide declared members of artifact workspaces from the repository's native discovery, using the generator's own addWorkspaceMembers. Dry run unless --write.
import {readFileSync, writeFileSync} from "node:fs";
import {join} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const R = join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/📝️render/🟦️.ts");
const {addWorkspaceMembers, workspaceAdmissionProblems} = await import(R);
const write = process.argv.includes("--write");
const artifacts = process.argv.slice(2).filter(a => !a.startsWith("--"));
let changed = 0;
for (const artifact of artifacts) {
  const path = join(ROOT, artifact, "Cargo.toml");
  const before = readFileSync(path, "utf8");
  const members: string[] = (Bun.TOML.parse(before) as any).workspace.members ?? [];
  const problemsBefore: string[] = workspaceAdmissionProblems(before);
  const after = addWorkspaceMembers(before, members);
  const problemsAfter: string[] = workspaceAdmissionProblems(after);
  const sameMembers = JSON.stringify((Bun.TOML.parse(after) as any).workspace.members) === JSON.stringify(members.slice().sort((a, b) => Buffer.compare(Buffer.from(a), Buffer.from(b))));
  console.log(JSON.stringify({artifact: artifact.split("/").slice(-3).join("/"), hiddenBefore: problemsBefore.length, hiddenAfter: problemsAfter.length, sameMembers, textChanged: before !== after}));
  if (before !== after && write) { writeFileSync(path, after); changed++; }
}
console.log(write ? `written ${changed}` : "dry run");
