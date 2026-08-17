import fs from "fs";
import path from "path";

const root = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite";
const dn = path.join(root, "🎲️board/🔌️ports/➡️directed/➕️normal/🦀️component.rs");
const t = fs.readFileSync(dn, "utf8").split("\n");
console.log("lines", t.length);
// Find mod declarations and region structure at low indent
for (let i = 0; i < t.length; i++) {
  const line = t[i];
  const m = line.match(/^( *)/);
  const ind = m ? m[1].length : 0;
  if (ind <= 0 && (/^(pub )?mod |^pub use |^\/\/ ?#region|^\/\/ ?#endregion|^use /.test(line.trim()) || line.trim().startsWith("}"))) {
    console.log(String(i + 1).padStart(5) + ": " + line.slice(0, 120));
  }
}
// Also find where NodeKindDef / board_json_visible are defined
["NodeKindDef", "board_json_visible_or_true", "board_json_locked_option", "fn distance_between", "struct EdgeData", "pub mod redraw"].forEach((pat) => {
  for (let i = 0; i < t.length; i++) {
    if (t[i].includes(pat)) {
      console.log(`DEF ${pat} @ ${i + 1}: ${t[i].trim().slice(0, 100)}`);
      break;
    }
  }
});

// directed parent
const directed = path.join(root, "🎲️board/🔌️ports/➡️directed/🦀️component.rs");
const d = fs.readFileSync(directed, "utf8").split("\n");
console.log("\ndirected lines", d.length);
for (let i = 0; i < Math.min(d.length, 80); i++) console.log(String(i + 1).padStart(5) + ": " + d[i].slice(0, 120));
console.log("--- directed mods ---");
d.forEach((l, i) => {
  if (/^(pub )?mod |^pub use crate|^\/\/ ?#region|^\/\/ ?#endregion/.test(l.trim()) && (l.match(/^ */)[0].length <= 0))
    console.log(String(i + 1).padStart(5) + ": " + l.slice(0, 120));
});

// Where are shared helpers?
const board = path.join(root, "🎲️board/🦀️component.rs");
const b = fs.readFileSync(board, "utf8");
console.log("\nboard component has NodeKindDef?", b.includes("NodeKindDef"));
console.log("board has board_json?", b.includes("board_json"));
console.log("board lines", b.split("\n").length);

const ports = path.join(root, "🎲️board/🔌️ports/🦀️component.rs");
const p = fs.readFileSync(ports, "utf8");
console.log("ports has NodeKindDef?", p.includes("struct NodeKindDef") || p.includes("pub struct NodeKindDef"));
console.log("ports has board_json?", /fn board_json/.test(p));
["NodeKindDef", "EdgeData", "board_json_locked_option", "distance_between", "ActiveUtility"].forEach((pat) => {
  const idx = p.indexOf(pat);
  console.log(`ports ${pat}:`, idx >= 0 ? "yes" : "no");
});
["NodeKindDef", "EdgeData", "board_json_locked_option", "distance_between", "ActiveUtility"].forEach((pat) => {
  for (let i = 0; i < d.length; i++) {
    if (d[i].includes("struct " + pat) || d[i].includes("fn " + pat) || d[i].includes("enum " + pat)) {
      console.log(`directed DEF ${pat} @ ${i + 1}`);
      break;
    }
  }
});
