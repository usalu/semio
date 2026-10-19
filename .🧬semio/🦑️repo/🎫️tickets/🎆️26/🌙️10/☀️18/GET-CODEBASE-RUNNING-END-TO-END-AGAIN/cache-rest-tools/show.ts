import {readFileSync} from "node:fs";
const F=process.argv[2];
const d=JSON.parse(readFileSync(F,"utf8"));
console.log("PROJECT",d.name,"namedInputs:",Object.keys(d.namedInputs??{}).join(","));
for (const [n,t] of Object.entries<any>(d.targets??{})) {
  const o=t.options??{};
  let cmd=o.command??JSON.stringify(o.commands??"");
  cmd=cmd.replaceAll("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/","<L>/");
  const extra:any={};
  for (const k of ["cache","inputs","outputs","dependsOn","continuous","executor"]) if (t[k]!==undefined) extra[k]=t[k];
  if (o.cwd) extra.cwd=o.cwd;
  console.log(`${n} :: ${cmd.slice(0,130)} :: ${JSON.stringify(extra).slice(0,300)}`);
}
