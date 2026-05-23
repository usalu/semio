import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const dir = path.dirname(fileURLToPath(import.meta.url));
const oldPath = path.join(dir, "index.old.tsx");
const outPath = path.resolve(dir, "../../../../../../semio/client/lib/react/index.tsx");

const old = fs.readFileSync(oldPath, "utf8");

const pre = fs.readFileSync(path.join(dir, "compose-pre.txt"), "utf8");
const mid = fs.readFileSync(path.join(dir, "compose-mid.txt"), "utf8");
