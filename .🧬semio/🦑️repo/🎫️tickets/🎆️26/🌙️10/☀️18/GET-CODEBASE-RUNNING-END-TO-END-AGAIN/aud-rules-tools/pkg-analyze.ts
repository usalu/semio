const S = "/private/tmp/claude-501/-Users-ueli-Documents-semio/f251ca6a-3613-4a20-b3b7-6e7765237e44/scratchpad";
const list = (await Bun.file(`${S}/pkgjson.txt`).text()).split("\n").filter(Boolean);
const rows: string[] = [];
for (const p of list) {
  const r = Bun.spawnSync(["git", "show", `HEAD:${p}`], { cwd: "/Users/ueli/Documents/semio" });
  let j: any; try { j = JSON.parse(new TextDecoder().decode(r.stdout)); } catch { continue; }
  for (const [n, v] of Object.entries<string>(j.dependencies ?? {})) rows.push(`${p}\t${j.name ?? "?"}\t${n}\t${v}`);
}
await Bun.write(`${S}/pkg-deps.tsv`, rows.join("\n") + "\n");
console.log("dep rows:", rows.length);
