/** 🪶️ Removes the DWARF custom sections (`.debug*`) from every core module of a WebAssembly component, keeping code, data and the `name` section, so jco's own bounded wasm memory is not exhausted by the debug information of a prototype build. A pure function of the bytes. @see https://github.com/bytecodealliance/jco */
export function stripWasmDebugSectionsV1(bytes: Uint8Array): Uint8Array {
  const source = Buffer.from(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  const readVarint = (buffer: Buffer, at: number): readonly [number, number] => { let value = 0, shift = 0, position = at, byte: number; do { byte = buffer[position++]!; value += (byte & 127) * 2 ** shift; shift += 7; } while (byte & 128); return [value, position]; };
  const writeVarint = (value: number): Buffer => { const out: number[] = []; let rest = value; do { let byte = rest % 128; rest = Math.floor(rest / 128); if (rest) byte |= 128; out.push(byte); } while (rest); return Buffer.from(out); };
  const walk = (buffer: Buffer): Buffer => {
    const parts: Buffer[] = [buffer.subarray(0, 8)];
    const component = buffer[4] === 0x0d;
    for (let at = 8; at < buffer.length;) {
      const id = buffer[at]!, [length, start] = readVarint(buffer, at + 1), end = start + length, body = buffer.subarray(start, end);
      let next = body;
      if (component && (id === 1 || id === 4)) next = walk(body);
      else if (!component && id === 0) { const [size, nameStart] = readVarint(body, 0); if (body.subarray(nameStart, nameStart + size).toString("utf8").startsWith(".debug")) { at = end; continue; } }
      parts.push(next === body ? buffer.subarray(at, end) : Buffer.concat([Buffer.from([id]), writeVarint(next.length), next]));
      at = end;
    }
    return Buffer.concat(parts);
  };
  return walk(source);
}
