// 🧪️ One-off ticket-scoped codegen (pass 1, part 2): wraps every hand-written
// `impl protocol::MutationKind<P, Op> for X { fn diff(&self, base: &P) -> TYPE { .. } }`
// leaf-level signature across lane 3-B's five norm facets so the return type matches the landed
// `fn diff(&self, base: &P) -> MutationOutcome<<Op as Mutation<P>>::Diff>` trait shape. The delegate
// body (`super::diff::diff(self, base)` or the fully-qualified equivalent) already returns
// `MutationOutcome<..>` after 🧪️w3-b-scalar-convert.ts + the hand-edited en1990 leaves, so only the
// signature line needs the wrap.
import { readFileSync, writeFileSync, readdirSync, statSync } from "fs";
import { join } from "path";

const ROOT = "/Users/ueli/Documents/semio";
const FACET_DIRS = [
  `${ROOT}/✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1992`,
  `${ROOT}/✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1991`,
  `${ROOT}/✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1996`,
  `${ROOT}/✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1994`,
  `${ROOT}/✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1990`,
];

function findFiles(dir: string, predicate: (p: string) => boolean, out: string[] = []): string[] {
  for (const entry of readdirSync(dir)) {
    const full = join(dir, entry);
    const st = statSync(full);
    if (st.isDirectory()) findFiles(full, predicate, out);
    else if (predicate(full)) out.push(full);
  }
  return out;
}

const SIG_RE = /^(\s*)fn diff\(&self, base: &(\w+)\) -> (.+) \{$/m;

let wrapped = 0;
let alreadyWrapped = 0;
let missed: string[] = [];

for (const dir of FACET_DIRS) {
  const files = findFiles(dir, (p) => p.includes("🧬️mutations/") && p.endsWith("🦠️mutation/🦀️component.rs"));
  for (const file of files) {
    const content = readFileSync(file, "utf8");
    const m = SIG_RE.exec(content);
    if (!m) {
      missed.push(file);
      continue;
    }
    const [whole, indent, base, retType] = m;
    if (retType.startsWith("protocol::MutationOutcome<")) {
      alreadyWrapped++;
      continue;
    }
    const replacement = `${indent}fn diff(&self, base: &${base}) -> protocol::MutationOutcome<${retType}> {`;
    const newContent = content.slice(0, m.index) + replacement + content.slice(m.index! + whole.length);
    writeFileSync(file, newContent, "utf8");
    wrapped++;
  }
}

console.log(`Wrapped: ${wrapped}`);
console.log(`Already wrapped: ${alreadyWrapped}`);
console.log(`Missed (${missed.length}):`);
for (const f of missed) console.log(`  ${f}`);
