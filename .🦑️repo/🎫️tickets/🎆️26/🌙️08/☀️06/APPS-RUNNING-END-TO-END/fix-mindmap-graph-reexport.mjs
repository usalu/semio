import { readFileSync, writeFileSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";

function find(dir, pred, depth = 0, acc = []) {
  if (depth > 10) return acc;
  let ents;
  try {
    ents = readdirSync(dir);
  } catch {
    return acc;
  }
  for (const name of ents) {
    if (["node_modules", "target", "dist", ".git"].includes(name)) continue;
    const p = join(dir, name);
    let st;
    try {
      st = statSync(p);
    } catch {
      continue;
    }
    if (st.isDirectory()) find(p, pred, depth + 1, acc);
    else if (pred(name, p)) acc.push(p);
  }
  return acc;
}

const ext = find(
  "/Users/ueli/Documents/semio/✏️s/🔨️modules",
  (n, p) => n.endsWith(".rs") && p.includes("mindmap") && p.includes("extension") && readFileSync(p, "utf8").includes("MindmapExtension"),
)[0];
let e = readFileSync(ext, "utf8");
console.log("before head:\n", e.slice(0, 400));

// Determine correct path for GraphExtension/NodeId/EdgeId by scanning infinite board sources
const fw = readdirSync("/Users/ueli/Documents/semio").find((n) => n.includes("framework") && readdirSync(join("/Users/ueli/Documents/semio", n)).includes("🛍️products"));
const hits = find(join("/Users/ueli/Documents/semio", fw), (n, p) => n.endsWith(".rs") && p.includes("infinite") && /pub trait GraphExtension|pub type NodeId|type NodeId =/.test(readFileSync(p, "utf8")));
console.log("def hits", hits);
for (const h of hits) {
  const t = readFileSync(h, "utf8");
  t.split("\n").forEach((l, i) => {
    if (/GraphExtension|pub type NodeId|pub type EdgeId|type NodeId|type EdgeId/.test(l)) console.log(h.split("/").slice(-4).join("/"), i + 1, l.trim());
  });
}

e = e.replace("pub use infinite_board_normal_directed as graph;\n", "pub use infinite_canvas::board as graph;\n");
if (!e.includes("pub use infinite_canvas as canvas;")) {
  e = e.replace("pub use infinite_canvas::board as graph;\n", "pub use infinite_canvas::board as graph;\npub use infinite_canvas as canvas;\n");
}
writeFileSync(ext, e);
console.log("after head:\n", e.slice(0, 450));
