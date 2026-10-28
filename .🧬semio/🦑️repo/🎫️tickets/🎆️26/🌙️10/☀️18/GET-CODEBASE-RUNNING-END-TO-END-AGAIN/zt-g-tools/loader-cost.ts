// offline micro-benchmark of the sealed stream loader's eager work on the current store snapshot (read-only)
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
const file = process.argv[2]!;
const lap = (name: string, f: () => unknown) => { const t = performance.now(); const r = f(); console.log(name.padEnd(34), (performance.now() - t).toFixed(1), "ms"); return r; };
const buf = lap("read 18.5 MB", () => readFileSync(file)) as Buffer;
lap("sha256 of the whole snapshot", () => createHash("sha256").update(buf).digest("hex"));
const text = lap("utf8 decode", () => buf.toString("utf8")) as string;
const value = lap("JSON.parse", () => JSON.parse(text)) as { files: { base64: string; sha256: string }[] };
lap("eager: decode+reencode+sha per file", () => { let n = 0; for (const row of value.files) { const bytes = Buffer.from(row.base64, "base64"); if (bytes.toString("base64") !== row.base64) throw Error("x"); if (createHash("sha256").update(bytes).digest("hex") !== row.sha256) throw Error("y"); n += bytes.length; } return n; });
lap("lazy: decode 20 files only", () => { let n = 0; for (const row of value.files.slice(0, 20)) n += Buffer.from(row.base64, "base64").length; return n; });
console.log("files", value.files.length, "peak rss MB", Math.round(process.memoryUsage().rss / 1e6));
