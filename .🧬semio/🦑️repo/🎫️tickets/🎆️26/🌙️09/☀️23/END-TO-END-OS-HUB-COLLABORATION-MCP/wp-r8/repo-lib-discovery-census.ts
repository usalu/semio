import { discoverPackageProblems, getWorkspaceRoot } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
const problems = discoverPackageProblems(getWorkspaceRoot());
const byKind = new Map<string, number>();
for (const problem of problems) byKind.set(problem.kind, (byKind.get(problem.kind) ?? 0) + 1);
console.log(JSON.stringify([...byKind], null, 0));
for (const problem of problems) console.log(`${problem.kind}\t${problem.path}\t${problem.message.slice(problem.path.length + 3, problem.path.length + 110)}`);
