import fs from "node:fs";
import path from "node:path";
const root = process.argv[2], count = Number(process.argv[3] ?? 300);
fs.rmSync(root, { recursive: true, force: true });
fs.mkdirSync(root, { recursive: true });
const data = Buffer.alloc(2048, 7);
function run(label, perFile) {
  const dir = path.join(root, label);
  fs.mkdirSync(dir);
  const start = performance.now();
  for (let i = 0; i < count; i++) {
    const fd = fs.openSync(path.join(dir, `f${i}`), "wx");
    fs.writeSync(fd, data);
    if (perFile) fs.fsyncSync(fd);
    fs.closeSync(fd);
  }
  if (!perFile) {
    const fd = fs.openSync(path.join(dir, "manifest"), "wx");
    fs.writeSync(fd, Buffer.from(JSON.stringify({ count })));
    fs.fsyncSync(fd);
    fs.closeSync(fd);
  }
  const ms = performance.now() - start;
  console.log(label, count, "files", ms.toFixed(0), "ms", (ms / count).toFixed(2), "ms/file");
}
run("per-file-fsync", true);
run("single-commit-fsync", false);
fs.rmSync(root, { recursive: true, force: true });
