import {playgroundCatalog, resolvePlaygroundFilter} from "@semio-tech/framework-plugin/build/plan";
const rows = playgroundCatalog();
console.log("[DEBUG] variants", rows.map(r => r.variant).join(","));
const s = rows.find(r => r.variant === "s");
console.log("[DEBUG] s row", JSON.stringify(s, null, 1));
console.log("[DEBUG] resolved", JSON.stringify(resolvePlaygroundFilter("s"), null, 1).slice(0, 4000));
