import { writeFileSync } from "node:fs";
const m = await import("@semio-tech/print/schema");
console.log("loaded", Object.keys(m).length);
