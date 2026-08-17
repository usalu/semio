#!/usr/bin/env bun
// 🧪️ Throwaway build runner for verify-break.tex — lives only in this ticket folder.
import { join } from "node:path";
import { buildPrintDocument } from "../../../../../../print/📜️script.ts";

const here = import.meta.dir;
const texAbs = join(here, "verify-break.tex");
const outDir = join(here, "dist");
await buildPrintDocument(texAbs, outDir);
console.log("[DEBUG] verify-break built ->", outDir);
