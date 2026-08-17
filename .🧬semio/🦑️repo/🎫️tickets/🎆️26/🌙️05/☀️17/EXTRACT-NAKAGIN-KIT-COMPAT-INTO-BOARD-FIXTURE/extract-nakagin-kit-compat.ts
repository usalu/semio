// #region 🧲️Header
// 💻️ Temporary ticket script: read `assets/fixtures/metabolism.kit.compose.json` Nakagin family `ports` + `compatiblePorts`, emit `BoardKindCompatEntry[]` as `compose.metabolism.light.handle.<portId>` pairs (`specificity: handle`, `bidirectional: true`), deduped. Optional `--write-board` merges into `.storybook/fixtures/nakagin-capsule-tower.board.json` under `meta.kindCompatibility` for manual diff review.
// #endregion 🧲️Header

import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const __dir = dirname(fileURLToPath(import.meta.url));

function findRepoRoot(startDir: string): string {
  let d = startDir;
  for (let i = 0; i < 20; i++) {
    const candidate = join(d, "compose", "assets", "fixtures", "metabolism.kit.compose.json");
    if (existsSync(candidate)) {
      return d;
    }
    const parent = dirname(d);
    if (parent === d) {
      break;
    }
    d = parent;
  }
  throw new Error(`[extract-nakagin-kit-compat] could not find repo root from ${startDir}`);
}

const repoRoot = findRepoRoot(__dir);
const kitPath = join(repoRoot, "assets/fixtures/metabolism.kit.compose.json");
const outExtracted = join(__dir, "kind-compatibility.extracted.json");
const boardPath = join(repoRoot, ".storybook/fixtures/nakagin-capsule-tower.board.json");

/** @emoji 🔗️ Matches handle kind ids used on the Nakagin board fixture (same port UUIDs as light kit). */
function handleKindForPortId(portId: string): string {
  return `compose.metabolism.light.handle.${portId}`;
}

type KitPort = { compatiblePorts?: { items?: { id: string }[] }; id: string };

function extractKindCompatibility(): { bidirectional: true; source: string; specificity: "handle"; target: string }[] {
  const raw = JSON.parse(readFileSync(kitPath, "utf8")) as {
    wip: { initialKit: { families: { items: { name?: string; ports?: { items?: KitPort[] } }[] } } };
  };
  const fam = raw.wip.initialKit.families.items.find((f) => f.name === "Nakagin Capsule Tower");
  const ports = fam?.ports?.items;
  if (!ports?.length) {
    throw new Error("[extract-nakagin-kit-compat] Nakagin family ports missing");
  }
  const seen = new Set<string>();
  const rules: { bidirectional: true; source: string; specificity: "handle"; target: string }[] = [];
  for (const p of ports) {
    const src = handleKindForPortId(p.id);
    for (const o of p.compatiblePorts?.items ?? []) {
      const tgt = handleKindForPortId(o.id);
      const lo = src < tgt ? src : tgt;
      const hi = src < tgt ? tgt : src;
      const key = `${lo}\n${hi}`;
      if (seen.has(key)) {
        continue;
      }
      seen.add(key);
      rules.push({ source: src, target: tgt, bidirectional: true, specificity: "handle" });
    }
  }
  rules.sort((a, b) => a.source.localeCompare(b.source) || a.target.localeCompare(b.target));
  return rules;
}

function main(): void {
  const writeBoard = process.argv.includes("--write-board");
  const rules = extractKindCompatibility();
  writeFileSync(outExtracted, `${JSON.stringify(rules, null, 2)}\n`);
  console.log(`[extract-nakagin-kit-compat] wrote ${outExtracted} (${rules.length} rules)`);
  if (writeBoard) {
    const board = JSON.parse(readFileSync(boardPath, "utf8")) as { meta?: Record<string, unknown> };
    if (!board.meta || typeof board.meta !== "object") {
      board.meta = {};
    }
    board.meta.kindCompatibility = rules;
    writeFileSync(boardPath, `${JSON.stringify(board, null, 2)}\n`);
    console.log(`[extract-nakagin-kit-compat] merged meta.kindCompatibility into ${boardPath}`);
  }
}

main();
