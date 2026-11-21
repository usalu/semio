import { readFile } from "node:fs/promises";

const [component, trace] = process.argv.slice(2);
if (!component || !trace) throw Error("A linked component and its matching Core trace are required");
const indices = new Set<number>();
for (const line of (await readFile(trace, "utf8")).split("\n")) {
  if (!line.includes("leaf functions and byte offsets")) continue;
  for (const match of line.matchAll(/Some\(\((\d+), \d+\)\)/gu)) indices.add(Number(match[1]));
}
const wasm = await readFile(component);
if (wasm.subarray(0, 8).toString("hex") !== "0061736d0d000100") throw Error("A linked Wasm component is required");
let cursor = 8;
function length(end: number): number {
  let value = 0;
  for (let shift = 0; shift <= 28 && cursor < end; shift += 7) {
    const byte = wasm[cursor++];
    value += (byte & 127) * 2 ** shift;
    if (byte < 128 && value <= 0xffffffff) return value;
  }
  throw Error("Invalid section integer");
}
function textExtent(end: number) {
  const count = length(end);
  const start = cursor;
  cursor += count;
  if (cursor > end) throw Error("Name exceeds its original subsection");
  return { start, end: cursor };
}
function section(parentEnd: number) {
  const id = wasm[cursor++];
  const count = length(parentEnd);
  const end = cursor + count;
  if (end > parentEnd) throw Error("Section exceeds its parent");
  return { id, end };
}
while (cursor < wasm.length && indices.size) {
  const componentSection = section(wasm.length);
  if (componentSection.id !== 1) {
    cursor = componentSection.end;
    continue;
  }
  if (wasm.subarray(cursor, cursor + 8).toString("hex") !== "0061736d01000000") throw Error("A Core module is required");
  cursor += 8;
  while (cursor < componentSection.end && indices.size) {
    const core = section(componentSection.end);
    if (core.id === 0) {
      const name = textExtent(core.end);
      if (wasm.toString("ascii", name.start, name.end) === "name") {
        while (cursor < core.end && indices.size) {
          const names = section(core.end);
          if (names.id === 1) {
            const count = length(names.end);
            for (let index = 0; index < count && indices.size; index++) {
              const functionIndex = length(names.end);
              const extent = textExtent(names.end);
              if (indices.delete(functionIndex)) console.log(`[DEBUG] ${functionIndex} ${wasm.toString("utf8", extent.start, extent.end)}`);
            }
          }
          cursor = names.end;
        }
      }
    }
    cursor = core.end;
  }
  cursor = componentSection.end;
}
if (indices.size) throw Error(`Unresolved Core frame indices ${[...indices].join(",")}`);
