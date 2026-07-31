import { spawnSync } from "child_process";

console.log("Starting tectonic compile...");
const res = spawnSync("/Users/ueli/Documents/semio/.repo/cache/tectonic/0.16.9/tectonic", ["--keep-logs", "--reruns", "2", "-Z", "search-path=/Users/ueli/Documents/semio/print/tex", "--outdir", "dist", "verify-cover.tex"], {
  cwd: "/Users/ueli/Documents/semio/mit-bestand/bericht/zwischenbericht",
  env: { ...process.env, TEXINPUTS: "/Users/ueli/Documents/semio/print/tex:" },
});

console.log("STATUS:", res.status);
console.log("SIGNAL:", res.signal);
if (res.error) {
  console.error("ERROR:", res.error);
}
console.log("STDOUT_LEN:", res.stdout ? res.stdout.length : "null");
console.log("STDERR_LEN:", res.stderr ? res.stderr.length : "null");
if (res.stderr && res.stderr.length > 0) {
  console.log("STDERR_TAIL:\n", res.stderr.toString().slice(-2000));
}
if (res.stdout && res.stdout.length > 0) {
  console.log("STDOUT_TAIL:\n", res.stdout.toString().slice(-2000));
}
