#!/usr/bin/env bun
/** 🪢️ Inserts the mount blocks of `🗑️generated/w13-schedules/mounts.txt` before the `//#endregion 🔖️Leaves` anchor of the artifact root (idempotent). */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { artifact, RS } from "./r3-f1-paths.ts";

const root = join(artifact, RS);
const mounts = readFileSync(join(import.meta.dir, "🗑️generated", "w13-schedules", "mounts.txt"), "utf8").replace(/\r?\n$/, "");
const source = readFileSync(root, "utf8");
if (source.includes("pub mod create_schedule")) {
  console.log("already mounted");
} else {
  const anchor = "                        //#endregion 🔖️Leaves";
  if (!source.includes(anchor)) throw new Error("leaves anchor not found");
  writeFileSync(root, source.replace(anchor, `${mounts}\n${anchor}`));
  console.log("mounted");
}
