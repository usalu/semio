/** @emoji 🔧️ Harness maintenance: `sync` re-points every `path =` dependency in `Cargo.toml` to the directory that currently exists for its ASCII skeleton, so the harness survives emoji re-labels of framework crate folders. */
import { existsSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, sep } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const manifest = join(here, "Cargo.toml");

/** @emoji 🔤️ Strips every non-ASCII code point so `🔺️⚙️mesh-engine` and `🔺️mesh-engine` compare equal. */
function skeleton(segment: string): string {
  return segment.replace(/[^\x20-\x7e]/g, "");
}

/** @emoji 🧭️ Resolves one absolute path segment by segment, substituting a sibling with the same ASCII skeleton when the literal segment is missing. */
function resolveBySkeleton(path: string): string {
  const parts = path.split(sep).filter((p) => p.length > 0);
  let current = sep;
  for (const part of parts) {
    const literal = join(current, part);
    if (existsSync(literal)) {
      current = literal;
      continue;
    }
    const want = skeleton(part);
    const candidates = existsSync(current) ? readdirSync(current).filter((name) => skeleton(name) === want) : [];
    if (candidates.length !== 1) throw new Error(`cannot resolve segment ${part} under ${current}: ${candidates.length} skeleton matches`);
    current = join(current, candidates[0]!);
  }
  return current;
}

/** @emoji 🔁️ Rewrites the manifest in place and reports every substitution. */
function sync(): void {
  const before = readFileSync(manifest, "utf8");
  let changed = 0;
  const after = before.replace(/path = "([^"]+)"/g, (match, path: string) => {
    if (!path.startsWith(sep)) return match;
    const resolved = resolveBySkeleton(path);
    if (resolved !== path) {
      changed += 1;
      console.log(`${path}\n  -> ${resolved}`);
    }
    return `path = "${resolved}"`;
  });
  if (changed > 0) writeFileSync(manifest, after);
  console.log(`sync: ${changed} path dependenc${changed === 1 ? "y" : "ies"} re-pointed`);
}

const [command] = process.argv.slice(2);
if (command === "sync") sync();
else {
  console.error("usage: bun ./📜️script.ts sync");
  process.exit(2);
}
