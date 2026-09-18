// 🧩️ The wfc3d mount contract, read from the SAME committed fixture the Rust and Python halves read.
// Nothing here transcribes an id: the fixture is the one statement of what this artifact mounts, so a
// drift shows up in three languages at once instead of in one.

import { describe, expect, it } from "vitest";

import contract from "../../🧫️fixtures/🧩️mount-contract/🔣️.json";

const SURFACE = /^(?<kind>[a-z0-9.]+)@(?<standard>[^/]+)\/(?<subset>[^#]+)#(?<role>editor|viewer)$/u;

describe("wfc3d mount contract", () => {
  it.each([
    ["editor", contract.editorAppId],
    ["viewer", contract.viewerAppId],
  ])("%s app id is a canonical surface id for this artifact's own dialect", (role, id) => {
    const match = SURFACE.exec(id);
    expect(match?.groups, `${id} is not a canonical surface id`).toBeDefined();
    expect(match?.groups?.kind).toBe("s.wfc.wfc3d");
    expect(match?.groups?.standard).toBe("1");
    expect(match?.groups?.subset).toBe("*");
    expect(match?.groups?.role).toBe(role);
  });

  it("mounts the shared graph window beside its own 3d preview, and one read-only viewer window", () => {
    expect(contract.editorWindowKinds).toEqual(["wfc-graph", "wfc-3d-preview"]);
    expect(contract.viewerWindowKinds).toEqual(["wfc-3d-view"]);
  });

  it("declares fifteen distinct mutation kinds", () => {
    expect(contract.mutations).toHaveLength(15);
    expect(new Set(contract.mutations).size).toBe(15);
  });

  it("answers its solve on the semio.infer cold-job route", () => {
    expect(contract.inferenceToolId).toBe("s.wfc.wfc3d.solve");
    expect(contract.inferenceJobKind).toBe("semio.infer");
    expect(contract.inferencePayloadSchema).toBe("s.wfc.wfc3d.inference.request.v1");
  });

  it("localizes every bundled example in English and German", () => {
    expect(contract.examples).toHaveLength(3);
    for (const example of contract.examples) {
      expect(Object.keys(example.label).sort()).toEqual(["de", "en"]);
      expect(example.slots).toBeGreaterThan(0);
    }
  });
});
