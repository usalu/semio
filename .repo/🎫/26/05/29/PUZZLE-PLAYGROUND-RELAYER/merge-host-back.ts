#!/usr/bin/env bun
import { readFileSync, writeFileSync, unlinkSync } from "node:fs";

for (const dim of ["2d", "3d", "5d"] as const) {
  const reactPath = `c:/git/compose/puzzle/${dim}/react/index.tsx`;
  const hostPath = `c:/git/compose/puzzle/${dim}/play/host.tsx`;
  const react = readFileSync(reactPath, "utf8").trimEnd();
  const host = readFileSync(hostPath, "utf8");
  const hostBody = host.replace(/^\/\*\* @emoji 🛝[\s\S]*?(?=\/\/ #region 🛝PlayHost)/, "");
  writeFileSync(reactPath, `${react}\n\n${hostBody.trim()}\n`);
  try {
    unlinkSync(hostPath);
  } catch {}
  console.log(`[merge] ${dim} host merged into react`);
}
