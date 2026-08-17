#!/usr/bin/env bun
import {
  existsSync,
  readFileSync,
  readdirSync,
  statSync,
  writeFileSync,
} from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const ticket = dirname(fileURLToPath(import.meta.url));
const root = join(ticket, "../../../../../..");
const pluginsRoot = join(root, "✏️s/🔌️plugins");

const SEM_MAGIC = Buffer.from([
  0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a,
]);

const DOMAIN = {
  lowpoly: {
    packMagic: 0x894c57504c0d0a1an,
    packStub: () => {
      const b = Buffer.alloc(96);
      b.writeUInt16LE(1, 8);
      b.writeUInt16LE(0, 10);
      b[12] = 1;
      b[13] = 1;
      Buffer.from("obj-1\0Unit Triangle\0", "utf8").copy(b, 14);
      b[12 + 26] = 2;
      b[13 + 26] = 1;
      Buffer.from("obj-1\0Base\0", "utf8").copy(b, 40);
      b[52] = 3;
      Buffer.from("lowpoly.document\0", "utf8").copy(b, 54);
      return b;
    },
    sprTag: 1,
    sprBody: () => {
      const b = Buffer.alloc(64);
      b[0] = 1;
      b[1] = 1;
      b[2] = 1;
      Buffer.from("obj-2\0Box\0", "utf8").copy(b, 3);
      return b;
    },
  },
  en1992: {
    packMagic: 0x894e19920e0a1a0an,
    packStub: () => {
      const b = Buffer.alloc(80);
      b.writeUInt16LE(1, 8);
      b.writeUInt16LE(0, 10);
      b[12] = 1;
      b[13] = 0;
      Buffer.from("norm.en1992.v1\0", "utf8").copy(b, 14);
      b[30] = 2;
      b[31] = 0;
      Buffer.from("annex=en tc2\0", "utf8").copy(b, 32);
      return b;
    },
    sprTag: 1,
    sprBody: () => {
      const b = Buffer.alloc(72);
      b[0] = 1;
      b[1] = 1;
      Buffer.from("liquid-retaining-fem-anchor\0", "utf8").copy(b, 2);
      return b;
    },
  },
  dag: {
    packMagic: 0x894441470e0a1a0an,
    packStub: () => {
      const b = Buffer.alloc(88);
      b.writeUInt16LE(1, 8);
      b.writeUInt16LE(0, 10);
      b.writeUInt32LE(2, 12);
      b.writeUInt32LE(1, 16);
      b[20] = 1;
      b[21] = 0;
      b[22] = 10;
      Buffer.from("node-a\0", "utf8").copy(b, 24);
      return b;
    },
    sprTag: 11,
    sprBody: () => {
      const b = Buffer.alloc(64);
      b[0] = 1;
      b[1] = 11;
      Buffer.from("dag.fixture\0", "utf8").copy(b, 2);
      return b;
    },
  },
  cad: {
    packMagic: 0x894341443e0a1a0an,
    packStub: () => {
      const b = Buffer.alloc(96);
      b.writeUInt16LE(2, 8);
      b.writeUInt16LE(0, 10);
      b.writeUInt32LE(0x0f, 12);
      b.writeUInt32LE(1, 16);
      b[20] = 1;
      b[21] = 0;
      b[22] = 20;
      Buffer.from("column-1\0", "utf8").copy(b, 24);
      return b;
    },
    sprTag: 14,
    sprBody: () => {
      const b = Buffer.alloc(72);
      b[0] = 1;
      b[1] = 14;
      Buffer.from("cad.scene.v2\0", "utf8").copy(b, 2);
      return b;
    },
  },
};

function magicBuf(hex) {
  const b = Buffer.alloc(8);
  let v = hex;
  for (let i = 7; i >= 0; i--) {
    b[i] = Number(v & 0xffn);
    v >>= 8n;
  }
  return b;
}

function semEnvelope(token) {
  const tokenBuf = Buffer.from(`${token}\0`, "utf8");
  const head = Buffer.alloc(SEM_MAGIC.length + 4 + tokenBuf.length);
  SEM_MAGIC.copy(head, 0);
  head.writeUInt32LE(tokenBuf.length - 1, SEM_MAGIC.length);
  tokenBuf.copy(head, SEM_MAGIC.length + 4);
  return head;
}

function tokenFromFilename(name) {
  const m = name.match(/component\.([^.]+)\.([^.]+)\.(pack|spr)\.semio$/);
  if (!m) return null;
  return `${m[1]}.${m[2]}.${m[3]} v1`;
}

function pilotKeyFromToken(token) {
  if (token.startsWith("lowpoly.")) return "lowpoly";
  if (token.startsWith("norm.en1992.")) return "en1992";
  if (token.startsWith("dag.")) return "dag";
  if (token.startsWith("cad.")) return "cad";
  return null;
}

function buildExample(path, kind) {
  const base = path.split("/").pop();
  const token = tokenFromFilename(base);
  if (!token) return null;
  const pilot = pilotKeyFromToken(token);
  const head = semEnvelope(token);
  if (pilot && DOMAIN[pilot]) {
    const dom = DOMAIN[pilot];
    if (kind === "pack") {
      const inner = Buffer.concat([magicBuf(dom.packMagic), dom.packStub()]);
      const pad = Buffer.alloc(48, 0xaa);
      return Buffer.concat([head, inner, pad]);
    }
    const inner = dom.sprBody();
    const pad = Buffer.alloc(48, 0xbb);
    return Buffer.concat([head, inner, pad]);
  }
  const pad = Buffer.alloc(128, 0xcc);
  return Buffer.concat([head, pad]);
}

const stats = { written: 0, skipped: 0, pilots: 0 };

function walk(dir) {
  for (const e of readdirSync(dir)) {
    const p = join(dir, e);
    if (statSync(p).isDirectory()) {
      walk(p);
      continue;
    }
    if (!e.endsWith(".pack.semio") && !e.endsWith(".spr.semio")) continue;
    const size = statSync(p).size;
    const kind = e.endsWith(".spr.semio") ? "spr" : "pack";
    const token = tokenFromFilename(e);
    const pilot = token ? pilotKeyFromToken(token) : null;
    const needsPilot = pilot && size <= 140;
    const needsTiny = size <= 64;
    if (!needsPilot && !needsTiny) {
      stats.skipped++;
      continue;
    }
    const out = buildExample(p, kind);
    if (!out || out.length <= 64) {
      console.error(`[pad-p4] failed ${p}`);
      process.exit(1);
    }
    writeFileSync(p, out);
    stats.written++;
    if (pilot) stats.pilots++;
    console.log(`[pad-p4] ${out.length}B ${kind} ${pilot ?? "corpus"} ${p}`);
  }
}

walk(pluginsRoot);
console.log(JSON.stringify(stats));
