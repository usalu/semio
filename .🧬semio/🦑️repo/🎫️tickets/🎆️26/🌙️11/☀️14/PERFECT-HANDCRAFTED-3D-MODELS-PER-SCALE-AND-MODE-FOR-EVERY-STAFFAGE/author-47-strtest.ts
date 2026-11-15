import { openCtx, saveLists, flag } from "./kit-lib.ts";
const a = process.argv.slice(2);
const c = openCtx(a[0]!, Number(flag(a, "--scale")));
const drop = (flag(a, "--drop") ?? "").split(",");
const keep = c.vol.structure.filter((s: any) => !drop.some((d) => d !== "" && (s.id.startsWith(d) || s.part === d)));
console.error("kept", keep.length, "of", c.vol.structure.length);
saveLists(c, { structure: keep });
