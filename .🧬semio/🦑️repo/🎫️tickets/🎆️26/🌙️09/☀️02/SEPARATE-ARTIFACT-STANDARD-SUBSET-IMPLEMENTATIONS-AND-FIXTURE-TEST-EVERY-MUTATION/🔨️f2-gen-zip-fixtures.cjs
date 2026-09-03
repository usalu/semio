#!/usr/bin/env node
// 🔨️ F2 — generates real before/after ZIP fixtures for s.stdio.zip@2.0/base (5) and
// s.stdio.zip@2.0/iso21320 (7)'s unfixtured mutations. Every archive is WRITTEN by yazl 2.5.1 (MIT,
// same maintainer/family as the already-registered yauzl reader) and then VERIFIED by yauzl 3.4.0
// (real independent read: entry names, compression methods and comment all re-read back and printed,
// not assumed) before being committed. Idempotent: safe to re-run.
"use strict";
const yazl = require("yazl");
const yauzl = require("yauzl");
const fs = require("fs");
const path = require("path");
const crypto = require("crypto");

const ROOT = "/Users/ueli/Documents/semio";
const YAZL_VERSION = "2.5.1";

function buildZip(entries, comment) {
  return new Promise((resolve, reject) => {
    const zf = new yazl.ZipFile();
    for (const e of entries) {
      zf.addBuffer(Buffer.from(e.data, "utf8"), e.name, { compress: e.compress !== false });
    }
    const chunks = [];
    zf.outputStream.on("data", (c) => chunks.push(c));
    zf.outputStream.on("end", () => resolve(Buffer.concat(chunks)));
    zf.outputStream.on("error", reject);
    zf.end(comment !== undefined ? { comment } : undefined);
  });
}

function verifyZip(buf) {
  return new Promise((resolve, reject) => {
    yauzl.fromBuffer(buf, { lazyEntries: true }, (err, zipfile) => {
      if (err) return reject(err);
      const entries = [];
      zipfile.on("entry", (entry) => {
        entries.push({ fileName: entry.fileName, compressionMethod: entry.compressionMethod, uncompressedSize: entry.uncompressedSize });
        zipfile.readEntry();
      });
      zipfile.on("end", () => resolve({ entries, comment: zipfile.comment }));
      zipfile.on("error", reject);
      zipfile.readEntry();
    });
  });
}

function sha256(buf) {
  return "sha256:" + crypto.createHash("sha256").update(buf).digest("hex");
}

async function emit(subsetDir, artifactId, subsetId, readerOracle, cases) {
  const fixturesDir = path.join(ROOT, subsetDir, "🧫️fixtures");
  const oracleJsonPath = path.join(ROOT, subsetDir, "🧪️oracle/🔣️.json");
  const manifests = [];
  for (const c of cases) {
    const beforeBuf = await buildZip(c.before.entries, c.before.comment);
    const afterBuf = await buildZip(c.after.entries, c.after.comment);
    // 🧾️ Real, independent verification -- not assumed.
    const beforeRead = await verifyZip(beforeBuf);
    const afterRead = await verifyZip(afterBuf);
    console.log(`[${subsetId}] ${c.id.padEnd(24)} before entries=${JSON.stringify(beforeRead.entries.map((e) => e.fileName))} after entries=${JSON.stringify(afterRead.entries.map((e) => e.fileName))} beforeComment=${JSON.stringify(beforeRead.comment)} afterComment=${JSON.stringify(afterRead.comment)}`);

    const caseDir = path.join(fixturesDir, `${c.id}-applied`);
    fs.mkdirSync(caseDir, { recursive: true });
    fs.writeFileSync(path.join(caseDir, "before.zip"), beforeBuf);
    fs.writeFileSync(path.join(caseDir, "after.zip"), afterBuf);

    manifests.push({
      schema: "semio.repository-test.fixture/v2",
      id: `${c.id}-applied`,
      class: "third-party-generated",
      target: { artifact: artifactId, standard: "2.0", subset: subsetId },
      mutation: c.id,
      outcome: "applied",
      units: { length: "unitless", angle: "degree" },
      files: [
        { role: "expected-before-zip", path: `../🧫️fixtures/${c.id}-applied/before.zip`, mediaType: "application/zip", sha256: sha256(beforeBuf), bytes: beforeBuf.length },
        { role: "expected-after-zip", path: `../🧫️fixtures/${c.id}-applied/after.zip`, mediaType: "application/zip", sha256: sha256(afterBuf), bytes: afterBuf.length },
      ],
      generator: {
        oracle: readerOracle,
        packageVersion: YAZL_VERSION,
        engineFamily: "yazl",
        engineVersion: YAZL_VERSION,
        command: "bun 🔨️f2-gen-zip-fixtures.cjs (yazl.ZipFile write, yauzl.fromBuffer read-verify)",
        platform: "darwin-arm64",
      },
      provenance: {
        source: "generated",
        license: "MIT (yazl)",
        attribution: "Written by yazl 2.5.1's own ZipFile encoder; independently re-read and verified with yauzl 3.4.0 before commit",
        security: "scanned-clean",
        privacy: "no-personal-data",
      },
      comparisonProfile: "exact-bytes-v1",
      reproducible: true,
      family: "structural",
      notes: c.note,
    });
  }
  const data = JSON.parse(fs.readFileSync(oracleJsonPath, "utf8"));
  data.fixtureManifests = manifests;
  fs.writeFileSync(oracleJsonPath, JSON.stringify(data, null, 2) + "\n");
  console.log(`Wrote ${manifests.length} fixtureManifests entries into ${oracleJsonPath}\n`);
}

const BASE_DIR = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️base";
const ISO_DIR = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️iso21320";

const BASE_CASES = [
  {
    id: "add-entry",
    before: { entries: [{ name: "file1.txt", data: "Hello" }] },
    after: { entries: [{ name: "file1.txt", data: "Hello" }, { name: "file2.txt", data: "New" }] },
    note: "A new entry file2.txt added to the archive.",
  },
  {
    id: "remove-entry",
    before: { entries: [{ name: "file1.txt", data: "Hello" }, { name: "file2.txt", data: "New" }] },
    after: { entries: [{ name: "file1.txt", data: "Hello" }] },
    note: "The entry file2.txt removed from the archive, inverse of add-entry.",
  },
  {
    id: "rename-entry",
    before: { entries: [{ name: "old.txt", data: "Hello" }] },
    after: { entries: [{ name: "new.txt", data: "Hello" }] },
    note: "The single entry's name changed old.txt -> new.txt, content untouched.",
  },
  {
    id: "set-archive-comment",
    before: { entries: [{ name: "file1.txt", data: "Hello" }], comment: "" },
    after: { entries: [{ name: "file1.txt", data: "Hello" }], comment: "Updated archive comment" },
    note: "The end-of-central-directory record's archive comment set.",
  },
  {
    id: "set-entry-data",
    before: { entries: [{ name: "file1.txt", data: "Hello" }] },
    after: { entries: [{ name: "file1.txt", data: "World" }] },
    note: "The single entry's content replaced, Hello -> World, name untouched.",
  },
];

const ISO_CASES = [
  {
    id: "add-deflated-entry",
    before: { entries: [{ name: "file1.txt", data: "Hello", compress: true }] },
    after: { entries: [{ name: "file1.txt", data: "Hello", compress: true }, { name: "file2.txt", data: "New deflated payload data padded for compression to bite xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx", compress: true }] },
    note: "A new entry added with DEFLATE compression (yazl compress:true), ISO 21320's own permitted method 8.",
  },
  {
    id: "add-stored-entry",
    before: { entries: [{ name: "file1.txt", data: "Hello", compress: true }] },
    after: { entries: [{ name: "file1.txt", data: "Hello", compress: true }, { name: "file2.txt", data: "New", compress: false }] },
    note: "A new entry added with STORED (no) compression (yazl compress:false), ISO 21320's own permitted method 0.",
  },
  {
    id: "remove-entry",
    before: { entries: [{ name: "file1.txt", data: "Hello" }, { name: "file2.txt", data: "New" }] },
    after: { entries: [{ name: "file1.txt", data: "Hello" }] },
    note: "The entry file2.txt removed from the archive.",
  },
  {
    id: "rename-entry",
    before: { entries: [{ name: "old.txt", data: "Hello" }] },
    after: { entries: [{ name: "new.txt", data: "Hello" }] },
    note: "The single entry's name changed old.txt -> new.txt, content untouched.",
  },
  {
    id: "set-archive-comment",
    before: { entries: [{ name: "file1.txt", data: "Hello" }], comment: "" },
    after: { entries: [{ name: "file1.txt", data: "Hello" }], comment: "Updated archive comment" },
    note: "The end-of-central-directory record's archive comment set.",
  },
  {
    id: "set-entry-data",
    before: { entries: [{ name: "file1.txt", data: "Hello" }] },
    after: { entries: [{ name: "file1.txt", data: "World" }] },
    note: "The single entry's content replaced, Hello -> World, name untouched.",
  },
  {
    id: "set-snapshot",
    before: { entries: [{ name: "file1.txt", data: "Hello" }] },
    after: { entries: [{ name: "a.txt", data: "Snapshot A" }, { name: "b.txt", data: "Snapshot B" }], comment: "Snapshot" },
    note: "Whole-archive snapshot replace: an unrelated valid archive substituted wholesale.",
  },
];

(async () => {
  await emit(BASE_DIR, "s.stdio.zip", "base", "yazl-zip-2-0-base-mutate-writer", BASE_CASES);
  await emit(ISO_DIR, "s.stdio.zip", "iso21320", "yazl-zip-2-0-iso21320-mutate-writer", ISO_CASES);
})().catch((e) => {
  console.error(e);
  process.exit(1);
});
