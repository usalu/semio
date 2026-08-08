import {
  policyExtractRustSchemaFields,
  policyExtractJsonSchemaFields,
} from "../../../../../../📜️script.ts";
import { readFileSync, readdirSync } from "fs";

const plugins = readdirSync("./✏️s/🔌️plugins");
const lp = plugins.find((p) => p.includes("lowpoly"))!;
const arts = readdirSync(`./✏️s/🔌️plugins/${lp}/🗿️artifacts`);
const art = arts[0]!;
const base = `./✏️s/🔌️plugins/${lp}/🗿️artifacts/${art}/🔺️diff/🧬️schema`;
const rust = policyExtractRustSchemaFields(readFileSync(`${base}/🦀️component.rs`, "utf8"));
const json = policyExtractJsonSchemaFields(readFileSync(`${base}/🔣️component.json`, "utf8"));
console.log("rust artifact", rust.fields.find((f) => f.name === "artifact"));
console.log("json artifact", json.fields.find((f) => f.name === "artifact"));
console.log("rust objects", rust.fields.find((f) => f.name === "objects"));
console.log("json objects", json.fields.find((f) => f.name === "objects"));
let mismatches = 0;
for (const rf of rust.fields) {
  const jf = json.fields.find((f) => f.name === rf.name);
  if (!jf || jf.optional !== rf.optional || jf.cardinality !== rf.cardinality || jf.scalar !== rf.scalar || jf.state !== rf.state) {
    console.log("MISMATCH", rf.name, { rust: rf, json: jf });
    mismatches++;
  }
}
console.log("done", { rust: rust.fields.length, json: json.fields.length, mismatches });
