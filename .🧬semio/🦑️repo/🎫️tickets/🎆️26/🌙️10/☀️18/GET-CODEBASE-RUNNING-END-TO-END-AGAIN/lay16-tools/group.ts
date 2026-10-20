import { readFileSync } from "node:fs";
const [file, pat, mode] = [process.argv[2]!, new RegExp(process.argv[3] ?? "."), process.argv[4] ?? "to"];
const m = new Map<string, number>();
for (const l of readFileSync(file, "utf8").split("\n")) {
  if (!l) continue;
  const [rule, from, to, spec, types] = l.split("\t");
  if (!pat.test(rule!)) continue;
  const bare = spec!.startsWith("@semio-tech/");
  const key = mode === "to" ? `${bare ? "BARE " + spec!.split("/").slice(0, 2).join("/") : "REL"} -> ${to!.split("/").slice(0, 6).join("/")}` : mode === "from" ? from!.split("/").slice(0, 4).join("/") : `${rule} ${spec}`;
  m.set(key, (m.get(key) ?? 0) + 1);
}
for (const [k, c] of [...m].sort((a, b) => b[1] - a[1])) console.log(String(c).padStart(5), k);
