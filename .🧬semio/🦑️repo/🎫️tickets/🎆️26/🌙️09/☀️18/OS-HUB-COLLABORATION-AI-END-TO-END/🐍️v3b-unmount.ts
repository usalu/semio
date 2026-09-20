#!/usr/bin/env bun
/** ✂️ V3b: removes mounts this slice added for leaves that do not compile, by `#[path]` target.
 *
 * A mount that breaks the build is worse than the gate row it silences, so a cluster whose leaves
 * cannot type-check is reverted and reported rather than left in the tree. Only the exact block
 * `🐍️v3b-mount-repair.ts` emitted is removed — the optional `#[cfg(test)]`, the `#[path]` line and the
 * single `mod`/`pub mod` line under it — and the `//#region 🪢️TaxonomyMounts` wrapper is dropped when
 * it ends up empty. Every host is re-read immediately before it is written.
 *
 * Usage: `bun 🐍️v3b-unmount.ts <targets.txt> [--apply]`   (one repo-relative leaf path per line)
 */
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";

const repoRoot = "/Users/ueli/Documents/semio";
const apply = process.argv.includes("--apply");
const wanted = new Set(readFileSync(process.argv[2]!, "utf8").split("\n").map((line) => line.trim()).filter(Boolean));
if (wanted.size === 0) throw new Error("empty target list — the unmount would silently do nothing");

const applyRows = readFileSync(join(repoRoot, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🗑️generated/v3b-s4-mount-apply.txt"), "utf8").split("\n");
const byHost = new Map<string, Set<string>>();
let matched = 0;
for (const row of applyRows) {
  const columns = row.split("\t");
  if (columns.length < 4 || !columns[1]!.startsWith("✏️s/")) continue;
  const hostAbs = join(repoRoot, columns[1]!);
  const target = columns[3]!;
  const leafRel = relative(repoRoot, resolve(dirname(hostAbs), target)).replaceAll("\\", "/");
  if (!wanted.has(leafRel)) continue;
  matched += 1;
  byHost.set(hostAbs, (byHost.get(hostAbs) ?? new Set<string>()).add(target));
}

let removed = 0;
for (const [hostAbs, targets] of byHost) {
  const lines = readFileSync(hostAbs, "utf8").split("\n");
  const keep: string[] = [];
  for (let index = 0; index < lines.length; index++) {
    const target = /^#\[path = "([^"]+)"\]$/u.exec(lines[index]!)?.[1];
    if (target !== undefined && targets.has(target) && /^(?:pub )?mod [A-Za-z_][A-Za-z0-9_]*;$/u.test(lines[index + 1] ?? "")) {
      if (keep[keep.length - 1] === "#[cfg(test)]") keep.pop();
      index += 1;
      removed += 1;
      continue;
    }
    keep.push(lines[index]!);
  }
  for (let index = keep.length - 1; index > 0; index--) {
    if (keep[index] === "//#endregion 🪢️TaxonomyMounts" && keep[index - 1] === "//#region 🪢️TaxonomyMounts") {
      keep.splice(index - 1, 2);
      if (keep[index - 2] === "") keep.splice(index - 2, 1);
    }
  }
  if (apply) writeFileSync(hostAbs, keep.join("\n"));
}
console.log(`${apply ? "unmounted" : "would unmount"} ${removed} leaf/leaves across ${byHost.size} host file(s); requested ${wanted.size}, matched ${matched}`);
