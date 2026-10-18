import {existsSync, readdirSync} from "node:fs";
import {basename, dirname, join} from "node:path";
const root = "/Users/ueli/Documents/semio";
const L = `${root}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`;
const {implementationLeafBasenameFinding} = await import(`${L}/🔍️discovery/🗂️file-kind-resolution/🟦️.ts`);
const {loadCatalogTaxonomy, semanticDirectoryKindId} = await import(`${L}/🔍️discovery/🔐️taxonomy-authority/🟦️.ts`);
const {pathEmojiStatuteFindings} = await import(`${root}/🧰️framework/🔨️modules/🪪️identity/🛣️path/🟦️.ts`);
const {leadingEmojiIdentity} = await import(`${root}/🧰️framework/🔨️modules/🪪️identity/🧩️grapheme/🟦️.ts`);
const t = loadCatalogTaxonomy();
const bases = process.argv.slice(2);
const SKIP = new Set(["node_modules", ".git", "target", "dist", "🗑️generated", ".🧬semio"]);
const FIXED = /^(go\.mod|go\.sum|README\.md|LICENSE\.md|AGENTS\.md|CHANGELOG\.md)$/u;
const chain = (relDir: string): { kind: string | null; parent?: string; anc: string[] } => {
  let parent: string | undefined;
  let anc: string[] = [];
  let kind: string | null = null;
  for (const seg of relDir.split("/")) {
    kind = semanticDirectoryKindId(seg, t, { parentKindId: parent, ancestorKindIds: [...anc] });
    if (kind) { if (parent) anc = [parent, ...anc]; parent = kind; }
  }
  return { kind, parent, anc };
};
let goFiles = 0, leafFindings = 0, dirsChecked = 0, dirProblems = 0, emojiProblems = 0, caseProblems = 0;
const kinds = new Map<string, number>();
const walk = (rel: string): void => {
  const entries = readdirSync(join(root, rel), { withFileTypes: true });
  const siblings = entries.filter((e) => !SKIP.has(e.name) && !FIXED.test(e.name)).map((e) => ({ path: `${rel}/${e.name}`, nodeKind: (e.isDirectory() ? "directory" : "file") as "directory" | "file" }));
  const generic = t.pathEmojiPolicy.genericEmojiIdentities as string[];
  for (const f of pathEmojiStatuteFindings(siblings.filter((s) => !/\.(json|ts|mjs|md|toml|lock)$/u.test(s.path) || /\/(📋️project\.json|📜️script\.ts)$/u.test(s.path)), generic)) {
    if (f.kind === "duplicate" || f.kind === "multiple" || f.kind === "generic" || f.kind === "presentation") { emojiProblems++; console.log("EMOJI", f.kind, f.path, "vs", (f as { sibling?: string }).sibling ?? ""); }
  }
  for (const e of entries) {
    if (SKIP.has(e.name)) continue;
    const p = `${rel}/${e.name}`;
    if (e.isDirectory()) {
      if (existsSync(join(root, p, "🐹️.go")) && !existsSync(join(root, p, "go.mod"))) {
        dirsChecked++;
        const c = chain(p);
        kinds.set(String(c.kind), (kinds.get(String(c.kind)) ?? 0) + 1);
        const inTests = basename(dirname(p)) === "🧪️tests";
        const ok = inTests ? c.kind === "members-of-tests" || c.kind === "members-of-members-of-members-of-modules" : c.kind === "members-of-members-of-members-of-modules";
        if (!ok) { dirProblems++; console.log("DIRKIND", p, c.kind, inTests ? "expected members-of-tests" : "expected members-of-members-of-members-of-modules"); }
      }
      walk(p);
    } else if (e.name.endsWith(".go")) {
      goFiles++;
      const f = implementationLeafBasenameFinding(p, t);
      if (f) { leafFindings++; console.log("LEAF", f.breachId, p); }
    }
  }
};
for (const base of bases) walk(base);
console.log(JSON.stringify({ goFiles, leafFindings, dirsChecked, dirProblems, emojiProblems, caseProblems, kinds: Object.fromEntries(kinds) }));
