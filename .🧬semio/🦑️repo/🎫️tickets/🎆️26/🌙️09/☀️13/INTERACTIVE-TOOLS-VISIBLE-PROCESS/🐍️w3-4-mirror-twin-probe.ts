/** 🪞️ W3-4 mirrors: answers synthesized `append-content`/`remove-content`/`commit-reconstruction`/`replace-mesh-result`/
 *  `create-asset` cases with the TypeScript twin and prints them as JSON for `🐍️w3-4-mirror-check.py`. */
import { readFileSync } from "node:fs";
import { decodeRemodelingSnapshot, remodelingSnapshotToJsonText, type RemodelingSnapshot } from "../../../../../../../✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts";
import { applyRemodelingMutation, decodeRemodelingMutation, remodelingContentHandle, remodelingMeshContentHandle, remodelingMutationOutcome } from "../../../../../../../✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts";

const fixtures = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations";
const read = (path: string): unknown => JSON.parse(readFileSync(`${fixtures}/${path}`, "utf8"));
const published = decodeRemodelingSnapshot(read("📦append-content/🧱️appends-sparse-leaves/📸️snapshot/➡️after/🔣️.json"));
const contentId = Object.keys(published.durableArtifacts).find((key) => published.durableArtifacts[key]!.kind === "sparse")!;
const leaf = published.durableArtifacts[contentId]!.chunks[0]!;
const bytes = (values: number[]): string => btoa(String.fromCharCode(...values));
const f32 = (values: number[]): number[] => [...new Uint8Array(new Float32Array(values).buffer)];
const u32 = (values: number[]): number[] => [...new Uint8Array(new Uint32Array(values).buffer)];
const encoded = (document: RemodelingSnapshot): unknown => JSON.parse(remodelingSnapshotToJsonText(document));
const cases: unknown[] = [];

function probe(name: string, kind: string, before: RemodelingSnapshot, wire: Record<string, unknown>): RemodelingSnapshot {
  const mutation = decodeRemodelingMutation(wire);
  const after = applyRemodelingMutation(before, mutation);
  cases.push({ name, kind, before: encoded(before), mutation: wire, after: encoded(after), messages: remodelingMutationOutcome(before, mutation).messages });
  return after;
}

const results = published.results;
const commit = (patch: Record<string, unknown>) => ({ mutation: "commitReconstruction", sparse: results.sparse, trajectory: results.trajectory, mesh: null, geo: results.geo, qc: results.qc, assets: [], ...patch });
const append = (patch: Record<string, unknown>) => ({ mutation: "appendContent", contentId, kind: "sparse", mime: null, width: 0, height: 0, first: 0, chunks: [leaf], ...patch });

probe("commit-binds-published-sparse", "commit-reconstruction", published, commit({ sparse: { points: remodelingContentHandle(contentId, 1), colors: null } }));
probe("commit-noop", "commit-reconstruction", published, commit({}));
probe("commit-sparse-wrong-count", "commit-reconstruction", published, commit({ sparse: { points: remodelingContentHandle(contentId, 2), colors: null } }));
probe("commit-sparse-missing", "commit-reconstruction", published, commit({ sparse: { points: remodelingContentHandle("remodeling-asset-nothing", 1), colors: null } }));
probe("commit-clears-results-and-unbinds", "commit-reconstruction", published, commit({ sparse: null, trajectory: null, geo: null, qc: null, assets: [{ id: "asset-spare", contentId: null }] }));
probe("commit-asset-names-sparse", "commit-reconstruction", published, commit({ assets: [{ id: "asset-new", contentId }] }));
const imageId = "remodeling-asset-image-probe";
const withImage = probe("append-image", "append-content", published, append({ contentId: imageId, kind: "image", mime: "image/png", width: 1, height: 1, chunks: [bytes([137, 80, 78, 71]), bytes([1, 2, 3])] }));
probe("commit-binds-image", "commit-reconstruction", withImage, commit({ assets: [{ id: "asset-new", contentId: imageId }, { id: "asset-a", contentId: imageId }] }));
probe("commit-image-unknown", "commit-reconstruction", withImage, commit({ assets: [{ id: "asset-new", contentId: "remodeling-asset-absent" }] }));
const meshId = "remodeling-mesh-probe";
const meshLeaves = [bytes([0, ...f32([0, 0, 0, 1, 0, 0, 0, 1, 0])]), bytes([3, ...u32([0, 1, 2])])];
const withMesh = probe("append-mesh", "append-content", published, append({ contentId: meshId, kind: "mesh", chunks: meshLeaves }));
const mesh = { mesh: remodelingMeshContentHandle(meshId, 2), source: "reconstructed", textureAssetId: null, watertight: null };
probe("commit-binds-mesh", "commit-reconstruction", withMesh, commit({ mesh }));
probe("commit-mesh-wrong-count", "commit-reconstruction", withMesh, commit({ mesh: { ...mesh, mesh: remodelingMeshContentHandle(meshId, 3) } }));
probe("replace-mesh-complete", "replace-mesh-result", withMesh, { mutation: "replaceMeshResult", mesh });
probe("replace-mesh-incomplete", "replace-mesh-result", published, { mutation: "replaceMeshResult", mesh });
const badMesh = probe("append-mesh-out-of-range", "append-content", published, append({ contentId: meshId, kind: "mesh", chunks: [meshLeaves[0], bytes([3, ...u32([0, 1, 7])])] }));
probe("commit-mesh-index-out-of-range", "commit-reconstruction", badMesh, commit({ mesh }));
const unordered = probe("append-mesh-descending-fields", "append-content", published, append({ contentId: meshId, kind: "mesh", chunks: [meshLeaves[1], meshLeaves[0]] }));
probe("commit-mesh-descending-fields", "commit-reconstruction", unordered, commit({ mesh }));
const second = bytes(f32([9, 8, 7]));
const extended = probe("append-extends-overlap", "append-content", published, append({ chunks: [leaf, second] }));
probe("append-conflict", "append-content", published, append({ chunks: [second] }));
probe("append-kind-mismatch", "append-content", published, append({ kind: "mesh", first: 1, chunks: [second] }));
probe("append-capacity", "append-content", extended, append({ first: 2, chunks: [second] }));
probe("append-invalid-leaf", "append-content", published, append({ chunks: ["!!!"] }));
probe("append-no-leaves", "append-content", published, append({ chunks: [] }));
probe("remove-partial", "remove-content", extended, { mutation: "removeContent", contentId, from: 1 });
probe("remove-noop", "remove-content", extended, { mutation: "removeContent", contentId, from: 2 });
probe("remove-gap", "remove-content", extended, { mutation: "removeContent", contentId, from: 3 });
probe("create-asset-content-handle", "create-asset", published, { mutation: "createAsset", key: "asset-new", asset: { mime: "image/png", data: remodelingContentHandle(contentId, 1), width: 1, height: 1 } });

process.stdout.write(JSON.stringify(cases));
