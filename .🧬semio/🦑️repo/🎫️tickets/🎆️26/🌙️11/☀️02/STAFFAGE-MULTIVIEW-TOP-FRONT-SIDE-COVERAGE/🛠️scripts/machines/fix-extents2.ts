import { load, save, rep } from "./lib.ts";
{ const d = load("city-car"); for (const v of ["front","plan"]) rep(d,500).geometry[v] = rep(d,500).geometry[v].filter((s:any)=>!/^mirror-[lr]$/.test(s.id)); save("city-car", d); }
{ const d = load("delivery-van"); for (const v of ["front","plan"]) for (const s of rep(d,500).geometry[v]) if (/^mirror-[lr]$/.test(s.id)) s.role="body"; save("delivery-van", d); }
