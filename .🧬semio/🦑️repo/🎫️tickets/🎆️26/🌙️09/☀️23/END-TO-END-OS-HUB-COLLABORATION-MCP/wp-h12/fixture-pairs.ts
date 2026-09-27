#!/usr/bin/env bun
/** 🧫️ H12 one-off (ticket 26/09/23 session 13): prints the captured canonical pair streams, their document ids, SPR frames and
 * content digests as the `🪞️pair-content-v1` fixture body. bun fixture-pairs.ts */
import { readFileSync } from "node:fs";
import { pairContentDigest, sprFrames } from "/Users/ueli/Documents/semio/🌎️hub/🧪️tests/🧠️residency/🟦️.ts";

const captured = [
  { name: "note-first", kindId: "s.note.note", file: "generated/pair-note-1.bin", documentId: "artifact-e5af511564e3d96974b442b1badd7e6e" },
  { name: "note-second", kindId: "s.note.note", file: "generated/pair-note-2.bin", documentId: "artifact-9f704e64a0f471ab8fbfc5dd89f6f5be" },
  { name: "drawing-first", kindId: "2d.drawing", file: "generated/pair-drawing-1.bin", documentId: "artifact-fde6fcb599f56f44b46ec04249bca238" },
  { name: "drawing-second", kindId: "2d.drawing", file: "generated/pair-drawing-2.bin", documentId: "artifact-b32425ccd56dd414b00dde14e404c468" },
];
const pairs = captured.map((pair) => {
  const stream = readFileSync(pair.file);
  const records: Buffer[] = [];
  for (let at = 0; at + 4 <= stream.length; at += 4 + stream.readUInt32BE(at)) {
    const record = stream.subarray(at + 4, at + 4 + stream.readUInt32BE(at));
    if (record[0] === 2 && record[1] === 2) records.push(record.subarray(18));
  }
  const frames = sprFrames(Buffer.concat(records)).map((frame) => ({ kind: frame.kind, flags: frame.flags, rawLength: frame.rawLength, payloadLength: frame.payload.length }));
  return { name: pair.name, kindId: pair.kindId, documentId: pair.documentId, streamHex: stream.toString("hex"), sprFrames: frames, contentDigest: pairContentDigest(stream, pair.documentId) };
});
console.log(JSON.stringify(pairs, null, 2));
