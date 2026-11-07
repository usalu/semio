import { createHash } from "node:crypto";
import { createReadStream, lstatSync, readdirSync } from "node:fs";
import { join, relative } from "node:path";

const generated = join(process.env.SEMIO_TICKET_DIR!, "🗑️generated"), first = join(generated, "msvc-stage-397620120"), second = join(generated, "msvc-stage-2765774819");
const files = (root: string): string[] => {
  const pending = [root], found: string[] = [];
  while (pending.length) { const directory = pending.pop()!; for (const entry of readdirSync(directory, { withFileTypes: true })) {
    const path = join(directory, entry.name);
    if (entry.isDirectory()) pending.push(path); else if (entry.isFile() && !entry.isSymbolicLink()) found.push(relative(root, path)); else throw Error("Nonregular staged entry");
  } }
  return found.sort();
};
const hash = async (path: string): Promise<string> => { const h = createHash("sha256"); for await (const bytes of createReadStream(path)) h.update(bytes); return h.digest("hex"); };
const a = files(first), b = files(second);
if (a.length !== 494 || JSON.stringify(a) !== JSON.stringify(b)) throw Error("Staged member sets differ");
for (let i = 0; i < a.length; i++) {
  const x = join(first, a[i]!), y = join(second, a[i]!);
  if (lstatSync(x).size !== lstatSync(y).size || await hash(x) !== await hash(y)) throw Error(`Staged member differs: ${a[i]}`);
  if ((i + 1) % 50 === 0) console.log(`Compared ${i + 1}/${a.length} staged files`);
}
console.log(`Verified ${a.length} staged file hashes agree`);
