import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";

const [component, trace] = process.argv.slice(2);
if (!component || !trace) throw Error("Provide the exact component and matching Core trace");
const bytes = readFileSync(component);
const log = readFileSync(trace, "utf8");
const root = Buffer.from("semio_owned_admit_actor_input_v1");
const offsets = new Map<number, Set<number>>();
for (const line of log.split("\n").filter(line => line.includes("leaf functions and byte offsets"))) {
  for (const [, name, offset] of line.matchAll(/Some\(\((\d+), (\d+)\)\)/gu)) {
    const index = Number(name);
    const locations = offsets.get(index) ?? new Set<number>();
    locations.add(Number(offset));
    offsets.set(index, locations);
  }
}

class Cursor {
  constructor(readonly bytes: Buffer, public position = 0) {}

  byte() {
    const value = this.bytes[this.position++];
    if (value === undefined) throw Error("Unexpected end of WebAssembly");
    return value;
  }

  integer() {
    let value = 0;
    for (let shift = 0; shift <= 28; shift += 7) {
      const byte = this.byte();
      value += (byte & 127) * 2 ** shift;
      if (byte < 128 && value <= 0xffffffff) return value;
    }
    throw Error("Invalid WebAssembly unsigned integer");
  }

  take(size: number) {
    const start = this.position;
    this.position += size;
    if (this.position > this.bytes.length) throw Error("WebAssembly section exceeds its extent");
    return this.bytes.subarray(start, this.position);
  }

  *sections() {
    while (this.position < this.bytes.length) {
      const id = this.byte();
      yield [id, this.take(this.integer())] as const;
    }
  }
}

if (bytes.subarray(0, 8).toString("hex") !== "0061736d0d000100") throw Error("A WebAssembly component is required");
const owned = [...new Cursor(bytes, 8).sections()].flatMap(([id, member]) => {
  if (id !== 1 || member.subarray(0, 8).toString("hex") !== "0061736d01000000") return [];
  const sections = new Map(new Cursor(member, 8).sections());
  const exports = sections.get(7);
  return exports?.includes(root) ? [{ member, sections }] : [];
});
if (owned.length !== 1) throw Error("Expected a single owned Core module");
const [{ member, sections }] = owned;
const core = new WebAssembly.Module(member);
const imports = WebAssembly.Module.imports(core).filter(row => row.kind === "function").length;
const bodies = new Cursor(sections.get(10)!);
const length = bodies.integer();
const calls = new Map<number, { caller: number; pc: number }[]>();
console.log("[DEBUG]", JSON.stringify({ sha256: createHash("sha256").update(bytes).digest("hex"), imports, definedFunctions: length }));
for (let index = imports; index < imports + length; index++) {
  const entry = new Cursor(bodies.take(bodies.integer()));
  const requested = offsets.get(index);
  if (!requested) continue;
  const groups = entry.integer();
  let locals = 0;
  for (let group = 0; group < groups; group++) {
    locals += entry.integer();
    entry.byte();
  }
  const body = entry.take(entry.bytes.length - entry.position);
  for (const pc of requested) {
    const opcode = body[pc];
    const immediate = [0x10, 0x11, 0x40, 0xfc].includes(opcode) ? new Cursor(body, pc + 1).integer() : undefined;
    if (opcode === 0x10 && immediate !== undefined) {
      const uses = calls.get(immediate) ?? [];
      uses.push({ caller: index, pc });
      calls.set(immediate, uses);
    }
    console.log("[DEBUG]", JSON.stringify({ function: index, pc, locals, bodyBytes: body.length, opcode: opcode?.toString(16), immediate, nearby: body.subarray(Math.max(0, pc - 8), pc + 12).toString("hex") }));
  }
}
if (calls.size) {
  const revisited = new Cursor(sections.get(10)!);
  revisited.integer();
  for (let index = imports; index < imports + length; index++) {
    const entry = new Cursor(revisited.take(revisited.integer()));
    const uses = calls.get(index);
    if (!uses) continue;
    const groups = entry.integer();
    let locals = 0;
    for (let group = 0; group < groups; group++) {
      locals += entry.integer();
      entry.byte();
    }
    console.log("[DEBUG]", JSON.stringify({ calledFunction: index, locals, bodyBytes: entry.bytes.length - entry.position, uses }));
  }
}
