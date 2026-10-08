#!/usr/bin/env bun
/** 🪢️ Inserts the mount blocks of `🗑️generated/f1-foundation/mounts.txt` before the `//#endregion 🔖️Leaves` anchor of the artifact root. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { artifact, RS } from "./r3-f1-paths.ts";

const root = join(artifact, RS);
const mounts = readFileSync(join(import.meta.dir, "🗑️generated", "f1-foundation", "mounts.txt"), "utf8").replace(/\n$/, "");
let source = readFileSync(root, "utf8");
if (source.includes("//@@leaves@@")) source = source.replace("//@@leaves@@", mounts);
writeFileSync(root, source);
console.log(source.includes("pub mod set_storey_height") ? "leaves mounted" : "nothing mounted");
