#!/usr/bin/env bun
/** 🔍 Inspect puzzle wasm manifest examples appId scoping. */
import { join } from "node:path";
const repoRoot = "/Users/ueli/Documents/semio";
const moduleUrl = `file://${join(repoRoot, "framework/product/os/dev/plugin-modules/puzzle/puzzle_plugin.js")}`;
const { createPluginApi } = await import(moduleUrl);
const api = await createPluginApi();
const manifest = JSON.parse(await api.manifest());
const examples = manifest.examples ?? [];
console.log("[DEBUG] puzzle examples:", examples.map((row: { id: string; label: string; appId: string }) => `${row.id}@${row.appId}`).join(", "));
const puzzle3d = examples.filter((row: { appId: string }) => row.appId === "puzzle3d-play");
console.log("[DEBUG] puzzle3d-play examples:", puzzle3d.map((row: { id: string; label: string }) => row.label).join(", "));
