import { readFileSync, writeFileSync } from "node:fs";
const [file, graph] = process.argv.slice(2);
let text = readFileSync(file, "utf8");
if (!text.includes("mod tests_graph;")) {
  const tail = '#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;';
  if (!text.includes(tail)) throw new Error("tests mount not found");
  text = text.replace(tail, tail + '\n#[cfg(test)]\n#[path = "🧪️tests/🧰️kit/🦀️.rs"]\nmod kit;\n#[cfg(test)]\n#[path = "🧪️tests/🗂️rows/🦀️.rs"]\nmod tests_rows;\n#[cfg(test)]\n#[path = "🧪️tests/🧮️table/🦀️.rs"]\nmod tests_table;\n#[cfg(test)]\n#[path = "🧪️tests/🕸️graph/🦀️.rs"]\nmod tests_graph;');
  writeFileSync(file, text);
}
let g = readFileSync(graph, "utf8");
g = g.replace('    snapshot.walls.retain(|_, wall| wall.storey != "st-first");\n    snapshot.openings.retain(|_, opening| opening.host != "w-first-south");\n', '');
writeFileSync(graph, g);
