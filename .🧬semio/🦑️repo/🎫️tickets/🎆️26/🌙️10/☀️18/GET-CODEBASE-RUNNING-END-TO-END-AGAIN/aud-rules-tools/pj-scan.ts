const S = process.argv[2];
const list = (await Bun.file(`${S}/pj.txt`).text()).split("\n").filter(Boolean);
const proc = Bun.spawn(["git", "cat-file", "--batch"], { cwd: "/Users/ueli/Documents/semio", stdin: "pipe", stdout: "pipe" });
proc.stdin.write(list.map((p) => `HEAD:${p}`).join("\n") + "\n");
proc.stdin.end();
const buf = new Uint8Array(await new Response(proc.stdout).arrayBuffer());
const dec = new TextDecoder();
let pos = 0;
const out: Record<string, { type: string; body: string }> = {};
for (const p of list) {
  let nl = buf.indexOf(10, pos);
  const header = dec.decode(buf.slice(pos, nl));
  pos = nl + 1;
  const parts = header.split(" ");
  if (parts[1] === "missing") { out[p] = { type: "missing", body: "" }; continue; }
  const size = parseInt(parts[2]);
  const body = dec.decode(buf.slice(pos, pos + size));
  pos += size + 1;
  out[p] = { type: parts[1], body };
}
await Bun.write(`${S}/pj-contents.json`, JSON.stringify(out));
console.log("files", Object.keys(out).length);
