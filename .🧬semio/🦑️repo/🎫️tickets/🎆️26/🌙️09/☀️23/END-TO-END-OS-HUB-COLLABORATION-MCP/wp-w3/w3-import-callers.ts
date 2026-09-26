/** 🔎️ Triage: names the guest functions that (transitively, by direct calls) reach a component's core imports of one interface prefix. */
import { readFileSync } from "node:fs";

const [componentPath, prefix = "wasi:filesystem", depthArg = "40"] = process.argv.slice(2);
const bytes = readFileSync(componentPath);

class Reader {
  constructor(public b: Uint8Array, public p = 0, public end = b.length) {}
  u8() { return this.b[this.p++]; }
  u32() { let r = 0, s = 0, x; do { x = this.b[this.p++]; r |= (x & 0x7f) << s; s += 7; } while (x & 0x80); return r >>> 0; }
  skipLeb() { while (this.b[this.p++] & 0x80); }
  name() { const n = this.u32(); const s = new TextDecoder().decode(this.b.subarray(this.p, this.p + n)); this.p += n; return s; }
}

function coreModules(component: Uint8Array): Uint8Array[] {
  const r = new Reader(component, 8), out: Uint8Array[] = [];
  while (r.p < r.end) {
    const id = r.u8(), size = r.u32(), start = r.p;
    if (id === 1) out.push(component.subarray(start, start + size));
    if (id === 4) out.push(...coreModules(component.subarray(start, start + size)));
    r.p = start + size;
  }
  return out;
}

function blockType(r: Reader) { const b = r.b[r.p]; if (b === 0x63 || b === 0x64) { r.p++; r.skipLeb(); } else if (b === 0x40 || (b >= 0x6a && b <= 0x7f)) r.p++; else r.skipLeb(); }
function memarg(r: Reader) { const a = r.u32(); if (a & 0x40) r.u32(); r.skipLeb(); }

function calls(body: Reader, out: Set<number>) {
  while (body.p < body.end) {
    const op = body.u8();
    switch (true) {
      case op === 0x02 || op === 0x03 || op === 0x04 || op === 0x06: blockType(body); break;
      case op === 0x0c || op === 0x0d || op === 0x09 || op === 0x18 || op === 0x07 || op === 0x08 || op === 0xd4 || op === 0xd6: body.u32(); break;
      case op === 0x0e: { const n = body.u32(); for (let i = 0; i <= n; i++) body.u32(); break; }
      case op === 0x10 || op === 0x12: out.add(body.u32()); break;
      case op === 0x11 || op === 0x13: body.u32(); body.u32(); break;
      case op === 0x14 || op === 0x15: body.u32(); break;
      case op === 0x1c: { const n = body.u32(); for (let i = 0; i < n; i++) body.u8(); break; }
      case op === 0x1f: { blockType(body); const n = body.u32(); for (let i = 0; i < n; i++) { const k = body.u8(); if (k < 2) body.u32(); body.u32(); } break; }
      case op >= 0x20 && op <= 0x26: body.u32(); break;
      case op >= 0x28 && op <= 0x3e: memarg(body); break;
      case op === 0x3f || op === 0x40: body.u32(); break;
      case op === 0x41 || op === 0x42: body.skipLeb(); break;
      case op === 0x43: body.p += 4; break;
      case op === 0x44: body.p += 8; break;
      case op === 0xd0: body.skipLeb(); break;
      case op === 0xd2: body.u32(); break;
      case op === 0xfc: {
        const s = body.u32();
        if (s === 8 || s === 10 || s === 12 || s === 14) { body.u32(); body.u32(); } else if (s === 9 || s === 11 || s === 13 || (s >= 15 && s <= 17)) body.u32();
        break;
      }
      case op === 0xfd: {
        const s = body.u32();
        if (s <= 11 || s === 92 || s === 93) memarg(body);
        else if (s === 12 || s === 13) body.p += 16;
        else if (s >= 21 && s <= 34) body.p += 1;
        else if (s >= 84 && s <= 91) { memarg(body); body.p += 1; }
        break;
      }
      case op === 0xfe: { const s = body.u32(); if (s === 3) body.p += 1; else memarg(body); break; }
      default: break;
    }
  }
}

for (const [moduleIndex, module] of coreModules(bytes).entries()) {
  const r = new Reader(module, 8);
  const importNames: string[] = [], names = new Map<number, string>(), bodies: Reader[] = [];
  while (r.p < r.end) {
    const id = r.u8(), size = r.u32(), start = r.p;
    if (id === 2) {
      const n = r.u32();
      for (let i = 0; i < n; i++) {
        const mod = r.name(), field = r.name(), kind = r.u8();
        if (kind === 0) { r.u32(); importNames.push(`${mod}#${field}`); }
        else if (kind === 1) { r.u8(); const f = r.u8(); r.u32(); if (f & 1) r.u32(); }
        else if (kind === 2) { const f = r.u8(); r.u32(); if (f & 1) r.u32(); }
        else if (kind === 3) { r.u8(); r.u8(); }
        else if (kind === 4) { r.u8(); r.u32(); }
      }
    } else if (id === 10) {
      const n = r.u32();
      for (let i = 0; i < n; i++) {
        const len = r.u32(), bodyStart = r.p, body = new Reader(module, bodyStart, bodyStart + len);
        const groups = body.u32();
        for (let g = 0; g < groups; g++) { body.u32(); const t = body.u8(); if (t === 0x63 || t === 0x64) body.skipLeb(); }
        bodies.push(body);
        r.p = bodyStart + len;
      }
    } else if (id === 0 && r.name() === "name") {
      while (r.p < start + size) {
        const sub = r.u8(), subSize = r.u32(), subStart = r.p;
        if (sub === 1) { const n = r.u32(); for (let i = 0; i < n; i++) { const idx = r.u32(); names.set(idx, r.name()); } }
        r.p = subStart + subSize;
      }
    }
    r.p = start + size;
  }
  const targets = new Set<number>();
  importNames.forEach((name, index) => { if (name.startsWith(prefix)) targets.add(index); });
  if (!targets.size) continue;
  console.log(`module ${moduleIndex}: ${importNames.length} function imports, ${bodies.length} bodies, ${names.size} names; targets: ${[...targets].map(i => importNames[i]).join(", ")}`);
  const callers = new Map<number, Set<number>>();
  bodies.forEach((body, i) => {
    const out = new Set<number>();
    try { calls(body, out); } catch { console.log(`decode failure in body ${i}`); }
    const self = importNames.length + i;
    for (const callee of out) { if (!callers.has(callee)) callers.set(callee, new Set()); callers.get(callee)!.add(self); }
  });
  const label = (i: number) => i < importNames.length ? importNames[i] : (names.get(i) ?? `func${i}`);
  const seen = new Map<number, number>();
  let frontier = [...targets];
  for (const t of targets) seen.set(t, 0);
  for (let depth = 1; depth <= Number(depthArg) && frontier.length; depth++) {
    const next: number[] = [];
    for (const f of frontier) for (const c of callers.get(f) ?? []) if (!seen.has(c)) { seen.set(c, depth); next.push(c); }
    frontier = next;
  }
  for (const [f, depth] of [...seen].sort((a, b) => a[1] - b[1])) console.log(`${String(depth).padStart(3)} ${label(f)}`);
}
