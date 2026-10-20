// 🔭 Preload: records every path a process reads through node:fs (sync, callback and promises) into $TRACE_OUT as sorted unique absolute paths; Bun.file reads are not seen.
import { createRequire } from "node:module";
import { resolve } from "node:path";
const require = createRequire(import.meta.url), fs = require("node:fs"), fsp = require("node:fs/promises"), seen = new Set<string>();
const note = (value: unknown): void => { if (typeof value === "string") seen.add(resolve(value)); else if (value instanceof URL) seen.add(value.pathname); else if (value instanceof Uint8Array) seen.add(resolve(Buffer.from(value).toString())); };
const wrap = (target: any, names: readonly string[]): void => { for (const name of names) { const original = target[name]; if (typeof original === "function") target[name] = function (this: unknown, path: unknown, ...rest: unknown[]) { note(path); return original.call(this, path, ...rest); }; } };
wrap(fs, ["readFileSync", "readdirSync", "statSync", "lstatSync", "existsSync", "openSync", "readlinkSync", "realpathSync", "opendirSync", "createReadStream", "readFile", "readdir", "stat", "lstat", "open", "readlink", "realpath", "opendir"]);
wrap(fsp, ["readFile", "readdir", "stat", "lstat", "open", "readlink", "realpath", "opendir"]);
const write = fs.writeFileSync.bind(fs);
process.on("exit", () => { if (process.env.TRACE_OUT) write(process.env.TRACE_OUT, [...seen].sort().join("\n") + "\n"); });
