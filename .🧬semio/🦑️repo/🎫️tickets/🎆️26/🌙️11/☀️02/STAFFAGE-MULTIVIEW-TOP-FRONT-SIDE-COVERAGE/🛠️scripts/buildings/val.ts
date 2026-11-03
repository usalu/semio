import { readFileSync, existsSync } from "node:fs";
const R = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { validateAsset } = await import(`${R}/🔨️modules/🗂️catalog/🟦️.ts`);
for (const id of process.argv.slice(2)) {
  const f = ["🧱️components","🔌️appliances","👤️humans"].map(g=>`${R}/🗂️catalog/${g}/${id}/🔣️.json`).find(existsSync)!;
  const a = JSON.parse(readFileSync(f, "utf8")).asset;
  let issues: any[] = []; try { issues = [...validateAsset(a)]; } catch (e:any) { console.log(id, "THROWS", e.message); continue; }
  const by = new Map<string,number>(); for (const i of issues) by.set(i.code,(by.get(i.code)??0)+1);
  console.log(id.padEnd(26), issues.length, [...by].map(([k,v])=>k+"x"+v).join(" "));
  if (process.env.V) for (const i of issues) console.log("   ", i.code, i.path, i.message.en.slice(0,200));
}
