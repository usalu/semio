#!/usr/bin/env bun
/** 🔍️ H12 one-off (ticket 26/09/23 session 13): why do residency-watch pair digests differ between rounds? Creates `kind`
 * three times in one fresh space — names A, A, B — fetches each active checkpoint pair, strips the frame headers exactly as
 * the residency watch does, replaces the document id, and prints where each pair first differs from the first one.
 *   bun pair-diff.ts <origin> <kindId> */
import { hubProbeCall, hubProbeCreateArtifact, hubProbeCreateSpace, hubProbeCreationCatalog, hubProbeSignIn } from "/Users/ueli/Documents/semio/🌎️hub/🤝️integration-harness/🟦️.ts";

const [origin, kindId] = process.argv.slice(2) as [string, string];
const token = await hubProbeSignIn(origin, "user1@semio.dev", "gm1-local-dev-pass-1", "pair-diff");
const spaceId = await hubProbeCreateSpace(origin, token, `Pair diff ${new Date().toISOString()}`);
const catalog = await hubProbeCreationCatalog(origin, token, spaceId);
const payload = async (documentId: string) => {
  const answer = await hubProbeCall(origin, "GET", `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/active-checkpoint/pair`, token, undefined, "application/vnd.semio.canonical-checkpoint-pair.v1");
  const stream = Buffer.from(answer.bytes);
  const parts: Buffer[] = [];
  for (let at = 0; at + 4 <= stream.length; ) {
    const length = stream.readUInt32BE(at);
    const frame = stream.subarray(at + 4, at + 4 + length);
    if (frame[0] === 2 && frame.length >= 18) parts.push(Buffer.from(frame.subarray(18)));
    at += 4 + length;
  }
  return Buffer.from(Buffer.concat(parts).toString("latin1").split(documentId).join("<document>"), "latin1");
};
const pairs: { name: string; bytes: Buffer }[] = [];
for (const name of ["Pair A", "Pair A", "Pair B"]) {
  const created = await hubProbeCreateArtifact(origin, token, spaceId, catalog.generationId, kindId, name);
  pairs.push({ name, bytes: await payload(created.artifactId) });
  console.log(`created ${kindId} "${name}" in ${created.ms} ms, pair ${pairs.at(-1)!.bytes.length} bytes`);
}
const printable = (bytes: Buffer) => [...bytes].map((byte) => (byte >= 32 && byte < 127 ? String.fromCharCode(byte) : ".")).join("");
for (const other of pairs.slice(1)) {
  const first = pairs[0]!.bytes;
  let at = 0;
  while (at < first.length && at < other.bytes.length && first[at] === other.bytes[at]) at += 1;
  const same = at === first.length && at === other.bytes.length;
  console.log(`"${pairs[0]!.name}" vs "${other.name}": ${same ? "identical" : `first difference at byte ${at} of ${first.length}/${other.bytes.length}`}`);
  const differing = [...first.keys()].filter((index) => first[index] !== other.bytes[index]);
  if (!same) console.log(`  ${differing.length} differing bytes at ${differing.join(",")}`);
  if (!same) console.log(`  first: ${printable(first.subarray(Math.max(0, at - 48), at + 48))}\n  other: ${printable(other.bytes.subarray(Math.max(0, at - 48), at + 48))}`);
}
