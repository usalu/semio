/** 🧪️ Buffer independently reproduces every FEM3d namespaced identity vector. */
import assert from "node:assert/strict";
import Ajv from "ajv";
import { testSharedMapDeltaOracle } from "@semio-tech/framework-replication/mutation/map/testing/shared-map-delta-source";
import { readFileSync } from "node:fs";
import { applyPatch, type Operation } from "fast-json-patch";
import { testDeclaredVariantFieldOracle } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🧪️tests/🌿️variant-field/🟦️.ts";

/** 🪪️ Validates namespace collisions, load ownership and UTF8 segment lengths without the runtime codec. */
export function testFem3dIdentityOracle(): void {
  testDeclaredVariantFieldOracle();
  testSharedMapDeltaOracle();
  const layerCounts = JSON.parse(readFileSync(new URL("../../../🎮️commands/🩹️patch-solid/🧪️tests/🧫️fixtures/🔣️.json", import.meta.url), "utf8")).layerCounts;
  const countOracle = new Ajv({ strict: true }).compile({ type: "integer", minimum: 0 });
  for (const row of layerCounts) {
    const parsed = Number(row.text);
    const admitted = countOracle(parsed);
    assert.equal(admitted, row.layers !== null);
    if (admitted) assert.equal(parsed, row.layers);
  }
  console.log("[DEBUG] independent Ajv count oracle: sparse unsigned solid layers and negative/fractional refusal");
  const vector = JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  const identities = vector.targets.map(({ segments, identity }: { segments: string[]; identity: string }) => {
    const reference = "scope:1:" + segments.map(segment => `${Buffer.byteLength(segment, "utf8")}:${segment}`).join("");
    assert.equal(reference, identity);
    return identity;
  });
  assert.equal(new Set(identities).size, identities.length);
  for (const row of vector.selectionLabels) {
    const segments = vector.targets.find((target: { identity: string }) => target.identity === row.identity)!.segments;
    const nouns = row.locale === "en" ? { node: "Node", load: "Load" } : { node: "Knoten", load: "Last" };
    const expected = segments[0] === "load" ? `${nouns.load} ${segments[1]} / ${segments[2]}` : `${nouns.node} ${segments[1]}`;
    assert.equal(expected, row.expected);
    assert(!row.expected.includes("scope:1:"));
  }
  console.log("[DEBUG] FEM3d human selection label vectors preserve localized nouns and load ownership");
  for (const row of vector.invalidOwnerCases) {
    const candidates: string[][] = structuredClone(vector.targets.map((target: { segments: string[] }) => target.segments));
    if (row.kind === "emptyNode") candidates[0]![1] = "";
    else if (row.kind === "duplicateNode") candidates.push([...candidates[0]!]);
    else if (row.kind === "duplicateLoad") candidates.push([...candidates[8]!]);
    else if (row.kind === "oversizedNode") candidates[0]![1] = "x".repeat(row.bytes);
    else throw Error("unknown owner admission vector");
    const encoded = candidates.map(segments => "scope:1:" + segments.map(segment => `${Buffer.byteLength(segment, "utf8")}:${segment}`).join(""));
    const admitted = candidates.every(segments => segments.every(Boolean)) && encoded.every(identity => Buffer.byteLength(identity, "utf8") <= 65536) && new Set(encoded).size === encoded.length;
    assert.equal(admitted, false, row.kind);
  }
  console.log("[DEBUG] FEM3d independent Buffer/Set oracle refused four empty, duplicate and oversized owner vectors");
  const peer = vector.peerCatchUp;
  const x: Operation = { op: "replace", path: "/x", value: peer.after.x };
  const y: Operation = { op: "replace", path: "/y", value: peer.after.y };
  for (const delivery of [[x, y], [y, x], [x, y, y]]) assert.deepEqual(applyPatch(structuredClone(peer.before), delivery, true).newDocument, peer.after);
  console.log("[DEBUG] FEM3d independent peer field intents converge across reversed and duplicate delivery");
  console.log("[DEBUG] FEM3d canonical identity vectors separate eight namespaces and two load owners with UTF8-safe lengths");
}
   