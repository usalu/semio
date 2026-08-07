#!/usr/bin/env node
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const ticketDir = dirname(fileURLToPath(import.meta.url));
const uiIndex = readFileSync(join(ticketDir, "ui-index.path"), "utf8").trim();
let text = readFileSync(uiIndex, "utf8");
let n = 0;

function mustReplace(label, old, neu) {
  if (!text.includes(old)) {
    console.error("MISSING:", label);
    console.error("Looking for first 120 chars:", JSON.stringify(old.slice(0, 120)));
    process.exit(1);
  }
  text = text.replace(old, neu);
  n++;
  console.log("ok:", label);
}

// --- 1. export useIsActiveShellRoot ---
mustReplace(
  "export useIsActiveShellRoot",
  `import { registerShellActivityRoot, activeShellRoot, useShellKeydown, NULL_SHELL_ROOT_REF } from "../../../../🧱️elements/