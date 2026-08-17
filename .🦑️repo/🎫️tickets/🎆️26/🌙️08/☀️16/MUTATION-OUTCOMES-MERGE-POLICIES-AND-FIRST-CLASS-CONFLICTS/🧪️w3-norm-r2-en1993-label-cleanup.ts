// 🧹 Follow-up to 🧪️w3-norm-r2-en1993-inputs.ts: cleans the Fatal `mutation.invariant` message
// label from the raw payload field name (`"new_bolt_f_ed_kn"`) to English prose (`"Bolt f ed kn"`)
// so the message matches the contract's "message = English prose" law (C2).
import { readFileSync, writeFileSync, readdirSync, statSync } from "fs";
import { join } from "path";

const ROOT = "/Users/ueli/Documents/semio";
const DIR = `${ROOT}/✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1993`;

function findFiles(dir: string, predicate: (p: string) => boolean, out: string[] = []): string[] {
  for (const entry of readdirSync(dir)) {
    const full = join(dir, entry);
    const st = statSync(full);
    if (st.isDirectory()) findFiles(full, predicate, out);
    else if (predicate(full)) out.push(full);
  }
  return out;
}

const LABEL_RE = /format!\("\{\} must be a finite number, got \{\}\.", "new_(\w+)", payload\.new_\1\)/g;

function toLabel(field: string): string {
  const words = field.split("_");
  return words.map((w, i) => (i === 0 ? w.charAt(0).toUpperCase() + w.slice(1) : w)).join(" ");
}

let changed = 0;
const diffFiles = findFiles(DIR, (p) => /update-.*-inputs\/🔺️diff\/🦀️component\.rs$/.test(p));
for (const f of diffFiles) {
  const content = readFileSync(f, "utf8");
  const newContent = content.replace(LABEL_RE, (whole, field) => {
    changed++;
    return `format!("${toLabel(field)} must be a finite number, got {}.", payload.new_${field})`;
  });
  if (newContent !== content) writeFileSync(f, newContent, "utf8");
}
console.log(`Labels cleaned: ${changed}`);
