import { readFileSync, writeFileSync } from "fs";
const path = readFileSync("/tmp/ng-path.txt", "utf8").trim();
let s = readFileSync(path, "utf8");
const old = `pub use infinite_board_port_directed_dag as dag;
pub use infinite_canvas as canvas;`;
const neu = `pub use infinite_canvas::board::ports::directed_dag as dag;
pub use infinite_canvas as canvas;`;
if (!s.includes(old)) {
  console.error("block not found");
  console.log(JSON.stringify(s.slice(0, 500)));
  process.exit(1);
}
s = s.replace(old, neu);
writeFileSync(path, s);
console.log("node-graph dag path fixed");
