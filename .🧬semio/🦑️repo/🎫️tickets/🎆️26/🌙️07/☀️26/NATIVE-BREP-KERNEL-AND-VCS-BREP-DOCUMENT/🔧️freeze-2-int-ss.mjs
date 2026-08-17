import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";
const ticket = path.dirname(fileURLToPath(import.meta.url));
const contracts = path.join(ticket, "📐️module-contracts.md");
let text = fs.readFileSync(contracts, "utf8");
text = text
  .split("\n")
  .map((line) =>
    line.includes("| `int_ss` |")
      ? "| `int_ss` | `✂️int-ss/🦀️component.rs` | FROZEN | `IntCurve{curve3}` + `intersect_surface_surface`; plane/plane + plane/cylinder analytic |"
      : line,
  )
  .join("\n");
fs.writeFileSync(contracts, text);
const status = path.join(ticket, "🚦️lane-status.md");
let st = fs.readFileSync(status, "utf8");
st = st.replace(
  "| 2-int-ss | 2 | done | FROZEN | wave2-run-3 |",
  "| 2-int-ss | 2 | done | cargo test -p semio-s-3d --lib brep::int_ss:: (4 passed) | 🧪lane-2-int-ss-test-quick-run-1.txt; 🧾lane-2-int-ss-scope-note.txt |",
);
fs.writeFileSync(status, st);
console.log(text.split("\n").find((l) => l.includes("int_ss")));
console.log(st.split("\n").find((l) => l.includes("2-int-ss")));
