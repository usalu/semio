import { readFileSync, writeFileSync } from "node:fs";
const file = "r4-x-examples-gen.ts";
let t = readFileSync(file, "utf8");
const swap = (from, to) => { if (!t.includes(from)) throw new Error("missing " + from.slice(0, 80)); t = t.replace(from, to); };
if (!t.includes("sch-finishes")) {
  swap(`      "sch-takeoff": schedule("Material take-off"`, `      "sch-finishes": schedule("Room finish schedule", "Finish", [col(field("number")), col(field("name")), col(field("surface")), col(field("material")), col(field("finish_area"), true)], { sort: [sortBy(field("number")), sortBy(field("surface"))], group: [groupBy(field("storey"))] }),
      "sch-doors-ground": schedule("Ground floor doors", "Door", [col(field("name")), col(field("type")), col(field("width")), col(field("height")), col(field("swing")), col(field("count"), true)], { sort: [sortBy(field("name"))], storeys: ["st-ground"], phases: ["New"] }),
      "sch-takeoff": schedule("Material take-off"`);
  swap(`      "sch-columns": schedule("Column schedule"`, `      "sch-doors": schedule("Door schedule", "Door", [col(field("name")), col(field("type")), col(field("storey")), col(field("width")), col(field("height")), col(field("leaves")), col(field("swing")), col(field("count"), true)], { sort: [sortBy(field("storey")), sortBy(field("name"))] }),
      "sch-windows": schedule("Window schedule", "Window", [col(field("name")), col(field("type")), col(field("storey")), col(field("width")), col(field("height")), col(field("panes")), col(field("count"), true)], { sort: [sortBy(field("storey")), sortBy(field("name"))] }),
      "sch-finishes": schedule("Room finish schedule", "Finish", [col(field("number")), col(field("name")), col(field("surface")), col(field("material")), col(field("finish_area"), true)], { sort: [sortBy(field("number")), sortBy(field("surface"))], group: [groupBy(field("storey"))] }),
      "sch-columns": schedule("Column schedule"`);
}
writeFileSync(file, t);
