const fs = require("fs");
const p = process.argv[2];
let s = fs.readFileSync(p, "utf8");
const desc = "Front geometry only: the shape stretches vertically between the origin and the tip of its own bone; a vertex at the height of the bone origin follows the y travel of the origin and one at the height of the bone tip the y travel of the tip, in between linearly.";
const swap = (a, b) => {
  if (!s.includes(a)) throw new Error("missing " + a);
  s = s.replace(a, b);
};
const prop = (indent) => `${indent}"followTip": {\n${indent}  "description": "${desc}",\n${indent}  "type": "boolean"\n${indent}}`;
swap('        "r": {\n          "type": "number",\n          "minimum": 0\n        }\n      }', '        "r": {\n          "type": "number",\n          "minimum": 0\n        },\n' + prop("        ") + "\n      }");
swap('            "r": {\n              "type": "number",\n              "minimum": 0\n            }\n', '            "r": {\n              "type": "number",\n              "minimum": 0\n            },\n' + prop("            ") + "\n");
swap('        "closed": {\n          "type": "boolean"\n        }\n      }', '        "closed": {\n          "type": "boolean"\n        },\n' + prop("        ") + "\n      }");
swap('            "closed": {\n              "type": "boolean"\n            }\n          }', '            "closed": {\n              "type": "boolean"\n            },\n' + prop("            ") + "\n          }");
fs.writeFileSync(p, s);
