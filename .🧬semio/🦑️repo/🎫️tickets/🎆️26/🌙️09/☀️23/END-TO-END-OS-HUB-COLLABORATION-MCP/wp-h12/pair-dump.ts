#!/usr/bin/env bun
/** 🔍️ H12 one-off (ticket 26/09/23 session 13): creates `kind` twice under the same name and dumps both active checkpoint
 * pairs frame by frame (kind, part, ordinal, offset, length) with a hex view of every differing region.
 *   bun pair-dump.ts <origin> <kindId> <out-prefix> */
import { writeFileSync } from "node:fs";
import { hubProbeCall, hubProbeCreateArtifact, hubProbeCreateSpace, hubProbeCreationCatalog, hubProbeSignIn } from "/Users/ueli/Documents/semio/🌎️hub/🤝️integration-harness/🟦️.ts";

const [origin, kindId, prefix] = process.argv.slice(2) as [string, string, string];
const token = await hubProbeSignIn(origin, "user1@semio.dev", "gm1-local-dev-pass-1", "pair-dump");
const spaceId = await hubProbeCreateSpace(origin, token, `Pair dump ${new Date().toISOString()}`);
const catalog = await hubProbeCreationCatalog(origin, token, spaceId);
const dumps: { id: string; frames: { kind: number; part: number; body: Buffer; header: Buffer }[] }[] = [];
for (const round of [1, 2]) {
  const created = await hubProbeCreateArtifact(origin, token, spaceId, catalog.generationId, kindId, "Pair dump");
  const answer = await hubProbeCall(origin, "GET", `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(created.artifactId)}/active-checkpoint/pair`, token, undefined, "application/vnd.semio.canonical-checkpoint-pair.v1");
  const stream = Buffer.from(answer.bytes);
  writeFileSync(`${prefix}-${round}.bin`, stream);
  const frames = [];
  for (let at = 0; at + 4 <= stream.length; ) {
    const length = stream.readUInt32BE(at);
    const frame = Buffer.from(stream.subarray(at + 4, at + 4 + length));
    frames.push({ kind: frame[0]!, part: frame[1]!, header: frame.subarray(0, Math.min(frame.length, 18)), body: frame[0] === 2 ? frame.subarray(18) : frame });
    at += 4 + length;
  }
  dumps.push({ id: created.artifactId, frames });
  console.log(`round ${round} id=${created.artifactId} frames=${frames.map((frame) => `${frame.kind}/${frame.part}:${frame.body.length}`).join(" ")}`);
}
const [a, b] = dumps as [(typeof dumps)[0], (typeof dumps)[0]];
for (const [index, frame] of a.frames.entries()) {
  if (frame.kind !== 2) continue;
  const left = Buffer.from(frame.body.toString("latin1").split(a.id).join("<document>"), "latin1");
  const right = Buffer.from(b.frames[index]!.body.toString("latin1").split(b.id).join("<document>"), "latin1");
  const differing = [...left.keys()].filter((at) => left[at] !== right[at]);
  console.log(`part ${frame.part}: ${left.length}/${right.length} bytes, ${differing.length} differ at ${differing.join(",")}`);
  const from = Math.max(0, (differing[0] ?? 0) - 64);
  console.log(`  A ${left.subarray(from, Math.max(from, (differing.at(-1) ?? 0) + 8)).toString("hex")}`);
  console.log(`  B ${right.subarray(from, Math.max(from, (differing.at(-1) ?? 0) + 8)).toString("hex")}`);
}
