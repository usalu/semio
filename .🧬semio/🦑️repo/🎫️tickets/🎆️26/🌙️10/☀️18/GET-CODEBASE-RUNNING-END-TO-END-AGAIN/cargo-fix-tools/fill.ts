const [findings, ovRoot, live, out] = process.argv.slice(2);
const rows = (await Bun.file(findings).text()).split("\n").filter(Boolean).map((l) => JSON.parse(l));
const set = new Set<string>();
for (const r of rows) {
  if (r.kind === "mod-missing") {
    const m = / -> (\S.*?) does not exist/.exec(r.msg);
    if (m) set.add(m[1].startsWith("/") ? m[1].slice(ovRoot.length + 1) : m[1]);
  } else if (r.kind === "include-missing") {
    const m = / -> (.*) does not exist$/.exec(r.msg);
    if (m) set.add(m[1].startsWith("/") ? m[1].slice(ovRoot.length + 1) : m[1]);
  }
}
const have = [...set].filter((p) => Bun.spawnSync(["test", "-e", `${live}/${p}`]).exitCode === 0);
await Bun.write(out, have.join("\n") + "\n");
console.log(set.size, "missing;", have.length, "exist live");
