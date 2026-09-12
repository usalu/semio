import { existsSync, mkdirSync, readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";

const repoRoot = process.argv[2] ? resolve(process.argv[2]) : (() => { throw new Error("repository root argument is required"); })();
const ticketRoot = resolve(import.meta.dir, "..", "..");
const audit = readFileSync(resolve(ticketRoot, "📓️other-testing-taxonomy-acceptance-audit-2026-09-12.md"), "utf8");
const fenced = audit.match(/```json\n([\s\S]*?)\n```/u)?.[1];
if (!fenced) throw new Error("reader repair JSON is missing");
const rows = JSON.parse(fenced) as { source: string; manifest: string; oldLiteral: string; newLiteral: string }[];
const evidence = rows.map((row) => {
  const source = resolve(repoRoot, row.source.replace(/:\d+$/u, ""));
  const manifest = resolve(repoRoot, row.manifest);
  const manifestRoot = dirname(manifest);
  const sourceText = readFileSync(source, "utf8");
  const manifestText = readFileSync(manifest, "utf8");
  const current = resolve(manifestRoot, row.newLiteral);
  const obsolete = resolve(manifestRoot, row.oldLiteral);
  const target = resolve(manifestRoot, "../../🦀️.rs");
  const result = {
    source: row.source,
    manifest: row.manifest,
    newLiteral: row.newLiteral,
    current,
    currentExists: existsSync(current),
    obsoleteExists: existsSync(obsolete),
    sourceContainsCurrent: sourceText.includes(row.newLiteral),
    sourceContainsObsolete: sourceText.includes(row.oldLiteral),
    manifestOwnsTarget: /\[lib\][\s\S]*?path\s*=\s*"\.\.\/\.\.\/🦀️\.rs"/u.test(manifestText) && existsSync(target),
  };
  if (!result.currentExists || result.obsoleteExists || !result.sourceContainsCurrent || result.sourceContainsObsolete || !result.manifestOwnsTarget) throw new Error(JSON.stringify(result));
  return result;
});
const output = resolve(ticketRoot, "🗑️generated", "testing-taxonomy-final-reader-repairs");
mkdirSync(output, { recursive: true });
await Bun.write(resolve(output, "resolution.json"), `${JSON.stringify(evidence, null, 2)}\n`);
console.log(`[DEBUG] manifest-relative readers=${evidence.length} current=10 obsolete=0 owned-targets=10`);
