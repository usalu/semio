import base from "./cfg-test.mjs";
const drop = (process.env.DROP ?? "").split(",").filter(Boolean);
const cfg = JSON.parse(JSON.stringify(base));
for (const m of cfg.modules) if (m.names) m.names = m.names.filter((n) => !drop.includes(n));
export default cfg;
