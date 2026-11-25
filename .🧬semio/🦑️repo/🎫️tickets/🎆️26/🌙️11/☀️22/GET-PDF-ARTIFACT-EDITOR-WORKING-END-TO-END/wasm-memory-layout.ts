/** 📐️ Prints the section sizes, the declared memory limits, the data segment volume and the function body size distribution of a core wasm module, so the initial linear memory of an instance can be attributed to static data, stack and heap.
 * Usage: `bun wasm-memory-layout.ts <core.wasm>`.
 * @see ./🧮️pdf-browser-page-memory.md */

const bytes = new Uint8Array(await Bun.file(process.argv[2]!).arrayBuffer());
let at = 8;
const leb = (): number => { let result = 0, shift = 0, byte; do { byte = bytes[at++]!; result += (byte & 0x7f) * 2 ** shift; shift += 7; } while (byte & 0x80); return result; };
const names = ["custom", "type", "import", "function", "table", "memory", "global", "export", "start", "element", "code", "data", "datacount", "tag"];
const sections: Record<string, number> = {};
const memories: { initialPages: number; maximumPages?: number }[] = [];
const segments: { offset?: number; bytes: number }[] = [];
const functions: number[] = [];
while (at < bytes.length) {
  const id = bytes[at++]!;
  const size = leb();
  const end = at + size;
  const name = names[id] ?? `#${id}`;
  sections[name] = (sections[name] ?? 0) + size;
  if (id === 5) { for (let count = leb(); count > 0; count--) { const flags = leb(); const initialPages = leb(); memories.push(flags & 1 ? { initialPages, maximumPages: leb() } : { initialPages }); } }
  if (id === 10) { for (let count = leb(); count > 0; count--) { const length = leb(); functions.push(length); at += length; } }
  if (id === 11) {
    for (let count = leb(); count > 0; count--) {
      const kind = leb();
      let offset: number | undefined;
      if (kind === 2) leb();
      if (kind !== 1) { const op = bytes[at++]!; offset = op === 0x41 ? leb() : undefined; while (bytes[at] !== 0x0b) at++; at++; }
      const length = leb();
      segments.push({ offset, bytes: length });
      at += length;
    }
  }
  at = end;
}
const mb = (value: number) => Math.round(value / 1048576 * 10) / 10;
const highest = Math.max(...segments.map(segment => (segment.offset ?? 0) + segment.bytes));
console.log(JSON.stringify({ fileMB: mb(bytes.length), sectionsMB: Object.fromEntries(Object.entries(sections).map(([name, size]) => [name, mb(size)])), memories: memories.map(memory => ({ ...memory, initialMB: mb(memory.initialPages * 65536), maximumMB: memory.maximumPages === undefined ? undefined : mb(memory.maximumPages * 65536) })), dataSegments: segments.length, dataMB: mb(segments.reduce((sum, segment) => sum + segment.bytes, 0)), lowestDataOffsetMB: mb(Math.min(...segments.map(segment => segment.offset ?? 0))), highestDataEndMB: mb(highest), largestSegmentsMB: segments.map(segment => segment.bytes).sort((left, right) => right - left).slice(0, 5).map(mb), functions: functions.length, largestFunctionsKB: [...functions].sort((left, right) => right - left).slice(0, 12).map(size => Math.round(size / 1024)), functionsOver100KB: functions.filter(size => size > 102400).length, bytesInFunctionsOver100KBMB: mb(functions.filter(size => size > 102400).reduce((sum, size) => sum + size, 0)), medianFunctionBytes: [...functions].sort((left, right) => left - right)[Math.floor(functions.length / 2)] }, null, 1));
