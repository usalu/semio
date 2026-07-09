#!/usr/bin/env bun
/** Creates declarative action JSON for interaction commit ops and aligns typology.actions. */
import { readdirSync, readFileSync, writeFileSync, mkdirSync, existsSync } from "node:fs";
import { join, dirname } from "node:path";

const root = "c:/git/compose/spatial/assets/modelDefinition";
const norm = (p: string) => p.replace(/\\/g, "/");

const actionIds = new Set<string>();

function walk(dir: string, onFile: (path: string) => void): void {
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const p = join(dir, entry.name);
    if (entry.isDirectory()) walk(p, onFile);
    else onFile(p);
  }
}

walk(root, (p) => {
  if (!norm(p).includes("/action/") || !p.endsWith(".json")) return;
  const j = JSON.parse(readFileSync(p, "utf8")) as { id?: string };
  if (j.id) actionIds.add(j.id);
});

const capabilityDoc = (id: string, label: string) =>
  `${JSON.stringify(
    {
      schema: "spatial.action/v1",
      id,
      version: "1.0.0",
      label,
      steps: [
        { op: "kernel.call", function: "spatial.action.capability", assignTo: "result" },
        { op: "return", result: { kind: "var", name: "result" } },
      ],
    },
    null,
    2,
  )}\n`;

let created = 0;
let typologyFixed = 0;

walk(root, (p) => {
  if (!norm(p).includes("/interaction/") || !p.endsWith(".json")) return;
  const interaction = JSON.parse(readFileSync(p, "utf8")) as {
    id?: string;
    label?: string;
    commit?: { operation?: { action?: string } };
  };
  const commitId = interaction.commit?.operation?.action;
  if (!commitId || actionIds.has(commitId)) return;
  const actionDir = join(dirname(p), "..", "action");
  mkdirSync(actionDir, { recursive: true });
  const base = commitId.split(".").pop() ?? commitId;
  const file = join(actionDir, `${base}.json`);
  if (!existsSync(file)) {
    writeFileSync(file, capabilityDoc(commitId, interaction.label ?? base));
    actionIds.add(commitId);
    created++;
  }
  const typologyPath = join(dirname(p), "..", "typology.json");
  if (!existsSync(typologyPath)) return;
  const typology = JSON.parse(readFileSync(typologyPath, "utf8")) as { schema?: string; actions?: string[] };
  if (typology.schema !== "spatial.typology/v1") return;
  const kept = (typology.actions ?? []).filter((id) => actionIds.has(id));
  const next = [...new Set([commitId, ...kept])];
  if (JSON.stringify(typology.actions) === JSON.stringify(next)) return;
  typology.actions = next;
  writeFileSync(typologyPath, `${JSON.stringify(typology, null, 2)}\n`);
  typologyFixed++;
});

console.log(`[DEBUG] created ${created} commit action JSON; updated ${typologyFixed} typologies`);
