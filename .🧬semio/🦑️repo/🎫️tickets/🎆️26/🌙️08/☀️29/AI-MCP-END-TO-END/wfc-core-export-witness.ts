import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";

const component = await readFile(process.argv[2]!);
const selected = new Set(process.argv.slice(3).map(Number));
const core = Buffer.from([0, 97, 115, 109, 1, 0, 0, 0]);
function namesFrom(module: Buffer) {
  let position = core.length;
  const word = () => {
    let value = 0;
    for (let shift = 0; shift < 35; shift += 7) {
      const byte = module.readUInt8(position++);
      value += (byte & 127) * 2 ** shift;
      if (byte < 128) return value;
    }
    throw Error("Bad Core section integer");
  };
  const string = () => {
    const length = word();
    const value = module.subarray(position, position + length).toString("utf8");
    position += length;
    return value;
  };
  const matched: { name: string; kind: string; index: number }[] = [];
  const imports: string[] = [];
  const resourceCounts: Record<number, number> = {};
  const limits = () => {
    const flags = word();
    word();
    if (flags & 1) word();
  };
  while (position < module.length) {
    const type = module[position++]!;
    if (type > 13) break;
    const length = word();
    const end = position + length;
    if (end > module.length) break;
    if (type === 2) {
      const total = word();
      for (let index = 0; index < total; index++) {
        const namespace = string();
        const name = string();
        switch (module.readUInt8(position++)) {
          case 0: imports.push(`${namespace}/${name}`); word(); break;
          case 1: word(); limits(); break;
          case 2: limits(); break;
          case 3: word(); position++; break;
          case 4: word(); word(); break;
          default: throw Error("Unknown Core import");
        }
      }
      assert.equal(position, end);
    } else if (type === 7) {
      const total = word();
      for (let index = 0; index < total; index++) {
        const name = string();
        const kind = module.readUInt8(position++);
        const indexInKind = word();
        if (kind === 0 && selected.has(indexInKind)) matched.push({ name, kind: "export", index: indexInKind });
      }
      assert.equal(position, end);
    } else if (type === 10 && process.env.SEMIO_DUMP_WFC_CORE_WITNESS === "1") {
      const total = word();
      if (matched.length) console.log("[DEBUG] Core function imports", JSON.stringify(imports));
      for (let index = 0; index < total; index++) {
        const size = word();
        const functionIndex = imports.length + index;
        if (selected.has(functionIndex)) {
          console.log("[DEBUG] Core function body", functionIndex, position, module.subarray(position, position + Math.min(size, 176)).toString("hex"));
        }
        position += size;
      }
    } else if (type === 0 && string() === "name") {
      while (position < end) {
        const subsection = module.readUInt8(position++);
        const size = word();
        const limit = position + size;
        assert(limit <= end);
        if (subsection === 1) {
          const total = word();
          for (let index = 0; index < total; index++) {
            const functionIndex = word();
            const name = string();
            if (selected.has(functionIndex)) matched.push({ name, kind: "debug-name", index: functionIndex });
          }
        }
        position = limit;
      }
    } else if (process.env.SEMIO_DUMP_WFC_CORE_WITNESS === "1" && [4, 5, 6, 9, 11].includes(type)) {
      resourceCounts[type] = word();
    }
    position = end;
  }
  if (matched.length && Object.keys(resourceCounts).length) console.log("[DEBUG] Core resource counts by section", JSON.stringify(resourceCounts));
  return matched;
}
function scanComponent(start: number, end: number): number {
  let position = start + 8;
  let matches = 0;
  while (position < end) {
    const section = component.readUInt8(position++);
    let length = 0;
    for (let shift = 0; ; shift += 7) {
      const byte = component.readUInt8(position++);
      length += (byte & 127) * 2 ** shift;
      if (byte < 128) break;
      assert(shift < 28);
    }
    const limit = position + length;
    assert(limit <= end);
    if (section === 1 && component.subarray(position, position + core.length).equals(core)) {
      const matched = namesFrom(component.subarray(position, limit));
      if (matched.length) {
        console.log("[DEBUG] nested Core symbols", position, JSON.stringify(matched));
        matches += matched.length;
      }
    }
    if (section === 4) matches += scanComponent(position, limit);
    position = limit;
  }
  return matches;
}
assert(scanComponent(0, component.length) > 0);
