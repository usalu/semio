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

const cargo = find(
  "/Users/ueli/Documents/semio/✏️s/🔨️modules",
  (n, p) => n === "Cargo.toml" && p.includes("mindmap") && p.includes("packages"),
)[0];
console.log("cargo", cargo);
let t = readFileSync(cargo, "utf8");

// Wrong: alias points at os-kernel. Right: same package as infinite_canvas (os-infinite),
// with a second Cargo alias for the board-normal-directed facade name mindmap expects.
const wrongPkg = 'package = "semio-framework-os-kernel"';
if (!t.includes("infinite_board_normal_directed") || !t.includes(wrongPkg)) {
  console.error("unexpected cargo contents");
  console.log(t);
  process.exit(1);
}

// Replace the bad dependency line with a correct path+package pointing at infinite.
const lines = t.split("\n");
const out = [];
for (const line of lines) {
  if (line.includes("infinite_board_normal_directed") && line.includes("semio-framework-os-kernel")) {
    // Mirror infinite_canvas path, keep the local alias name mindmap uses.
    const canvasLine = lines.find((l) => l.includes("infinite_canvas ="));
    if (!canvasLine) throw new Error("no infinite_canvas line");
    const pathMatch = canvasLine.match(/path = "([^"]+)"/);
    out.push(
      `infinite_board_normal_directed = { path = "${pathMatch[1]}", package = "semio-framework-os-infinite" }`,
    );
  } else {
    out.push(line);
  }
}
t = out.join("\n");
writeFileSync(cargo, t);
console.log("updated deps:\n" + t.split("\n").filter((l) => l.includes("infinite")).join("\n"));

// Also fix extension reexports if GraphExtension still isn't at crate root.
const ext = find(
  "/Users/ueli/Documents/semio/✏️s/🔨️modules",
  (n, p) => n.endsWith(".rs") && p.includes("mindmap") && p.includes("extension") && readFileSync(p, "utf8").includes("MindmapExtension"),
)[0];
console.log("ext", ext);
let e = readFileSync(ext, "utf8");
// After fixing the package, check whether board items are reexported at crate root via infinite glue.
// Infinite `pub use component::*` / board module — GraphExtension may live under board::.
// Prefer explicit board paths for stability:
const desired = `pub use infinite_canvas::board as graph;
pub use infinite_canvas as canvas;
`;
if (e.includes("pub use infinite_board_normal_directed as graph;")) {
  e = e.replace(
    `pub use infinite_board_normal_directed as graph;
pub use infinite_canvas as canvas;
`,
    desired,
  );
  writeFileSync(ext, e);
  console.log("extension reexports updated to infinite_canvas::board");
}
console.log(e.slice(0, 500));
