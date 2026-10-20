/** 🧰️ Pure fixture tasks of the worker pool laws. */
import { readFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { threadId } from "node:worker_threads";

export const sha256Files = (paths, context) => paths.map((path, index) => { const digest = createHash("sha256").update(readFileSync(path)).digest("hex"); context.progress(index + 1, paths.length); return digest; });
export const square = (value) => value * value;
export const count = (total, context) => { for (let index = 1; index <= total; index++) context.progress(index, total); return total; };
export const spin = (milliseconds) => { const until = performance.now() + milliseconds; while (performance.now() < until); return milliseconds; };
export const fail = (message) => { throw new TypeError(message); };
export const crash = () => process.exit(3);
export const thread = () => threadId;
