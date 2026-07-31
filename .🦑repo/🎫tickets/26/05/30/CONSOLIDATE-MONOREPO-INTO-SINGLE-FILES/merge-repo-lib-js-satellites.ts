import { readFileSync, writeFileSync, unlinkSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dirname, "../../../../../../");
const dir = join(root, "repo/lib/js/src");

function stripHeaderAndLocalImports(src: string): string {
  const lines = src.split(/\r?\n/);
  let i = 0;
  if (lines[i]?.includes("#region") && lines[i]?.includes("Header")) {
    i++;
    while (i < lines.length && !lines[i]?.includes("#endregion")) i++;
    i++;
  }
  const out: string[] = [];
  for (; i < lines.length; i++) {
    const line = lines[i]!;
    if (/^import\s+[\s\S]*?\s+from\s+["']\.\/(uloc-metrics|micro-commit|commit)\.ts["'];?\s*$/.test(line.trim())) continue;
    if (/^import\s*\{[\s\S]*?\}\s*from\s+["']\.\/(uloc-metrics|micro-commit|commit)\.ts["'];?\s*$/.test(line.trim())) continue;
    out.push(line);
  }
  while (out.length > 0 && out[0]?.trim() === "") out.shift();
  return out.join("\n");
}

const indexPath = join(dir, "index.ts");
let index = readFileSync(indexPath, "utf8");

index = index.replace(
  /\nexport \{[\s\S]*?\} from "\.\/micro-commit\.ts";\nexport type \{ MicroCommitLevel \} from "\.\/micro-commit\.ts";\n\nexport \{[\s\S]*?\} from "\.\/commit\.ts";\nexport type \{ CommitBundleDateSection, CommitBundleSection, CommitLevel, CommitSteps \} from "\.\/commit\.ts";\n?$/,
  "",
);

const uloc = stripHeaderAndLocalImports(readFileSync(join(dir, "uloc-metrics.ts"), "utf8"));
const micro = stripHeaderAndLocalImports(readFileSync(join(dir, "micro-commit.ts"), "utf8"));
const commit = stripHeaderAndLocalImports(readFileSync(join(dir, "commit.ts"), "utf8"));

const block = `
//#region 🔖uloc-metrics
${uloc}
//#endregion 🔖uloc-metrics

//#region 🔖micro-commit
${micro}
//#endregion 🔖micro-commit

//#region 🔖commit
${commit}
//#endregion 🔖commit
`;

index = index.trimEnd() + "\n" + block;

writeFileSync(indexPath, index);
for (const f of ["uloc-metrics.ts", "micro-commit.ts", "commit.ts", "bundle-script.ts", "dependency-boundary.ts"]) {
  const p = join(dir, f);
  try {
    unlinkSync(p);
  } catch {
    /* already removed */
  }
}

console.log("merged repo/lib/js satellites into index.ts");
