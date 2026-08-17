#!/usr/bin/env bun
import { spawnSync } from "node:child_process";
import { writeFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dir, "../../../../../..");
const r = spawnSync("git", ["show", "HEAD:elements/lib/react/core/index.tsx"], { cwd: root, encoding: "utf8" });
if (r.status !== 0) throw new Error(r.stderr);
const lines = r.stdout.split(/\r?\n/);
const start = lines.findIndex((l) => l === "// #region 🎊️UI");
const end = lines.findIndex((l) => l === "// #endregion 🎊️UI");
writeFileSync(join(import.meta.dir, "shell-extract.tsx"), lines.slice(start + 1, end).join("\n"), "utf8");
console.log("lines", end - start - 1);
