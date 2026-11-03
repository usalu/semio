import { load, save, rep } from "./lib.ts";
const d = load("city-car"); const o = rep(d, 500).decisions.omitted;
o.en = o.en.replace("front headrests (0.18 m wide but 0.09 m deep = 0.18 mm, below the 0.2 mm print minimum)", "front headrests (0.05 m deep in plan = 0.1 mm, below the 0.2 mm print minimum)");
save("city-car", d); console.log(o.en.slice(-260));
