// [farmid] A/B of the snapshot hand-off to a worker: descriptor 3 as a private regular file (what the farm does) vs a pipe the parent writes, while the parent's event loop is busy the way a graph child is (synchronous slices of SLICE_MS between yields). Reports the child's time from spawn to having read and hashed every byte. usage: bun handoff.ts <snapshot file> [rounds] [sliceMs]. Not a library file.
import { spawn } from "node:child_process";
import { openSync, closeSync, readFileSync, constants } from "node:fs";
const [file, roundsText = "5", sliceText = "50"] = process.argv.slice(2), rounds = Number(roundsText), slice = Number(sliceText), bytes = readFileSync(file);
const child = `const fs=require("node:fs"),c=require("node:crypto"),t0=Number(process.argv[1]);const f=fs.fstatSync(3);let b;if(f.isFile()){b=Buffer.allocUnsafe(f.size);let u=0;while(u<b.length)u+=fs.readSync(3,b,u,b.length-u,null);}else{const parts=[];const k=Buffer.alloc(65536);for(;;){const n=fs.readSync(3,k,0,k.length,null);if(!n)break;parts.push(Buffer.from(k.subarray(0,n)));}b=Buffer.concat(parts);}c.createHash("sha256").update(b).digest("hex");process.stdout.write(String(Date.now()-t0));`;
let busy = true; const spin = () => { if (!busy) return; const end = performance.now() + slice; while (performance.now() < end); setTimeout(spin, 0); };
const once = (mode: "file" | "pipe") => new Promise<number>(done => {
  const t0 = Date.now(), fd = mode === "file" ? openSync(file, constants.O_RDONLY | (constants.O_NOFOLLOW ?? 0)) : undefined;
  const proc = spawn(process.execPath, ["-e", child, String(t0)], { stdio: ["ignore", "pipe", "inherit", fd ?? "pipe"] });
  if (fd !== undefined) closeSync(fd); else { const w = proc.stdio[3] as import("node:stream").Writable; let at = 0; const pump = () => { while (at < bytes.length) { const chunk = bytes.subarray(at, at + 65536); at += chunk.length; if (!w.write(chunk)) { w.once("drain", pump); return; } } w.end(); }; pump(); }
  let out = ""; proc.stdout!.on("data", d => out += d); proc.on("exit", () => done(Number(out)));
});
spin();
const rows: Record<string, number[]> = { file: [], pipe: [] };
for (let r = 0; r < rounds; r++) for (const mode of (r % 2 ? ["pipe", "file"] : ["file", "pipe"]) as ("file" | "pipe")[]) rows[mode].push(await once(mode));
busy = false;
const median = (a: number[]) => [...a].sort((x, y) => x - y)[Math.floor(a.length / 2)];
console.log(JSON.stringify({ bytes: bytes.length, sliceMs: slice, file: rows.file, pipe: rows.pipe, medianFileMs: median(rows.file), medianPipeMs: median(rows.pipe) }));
