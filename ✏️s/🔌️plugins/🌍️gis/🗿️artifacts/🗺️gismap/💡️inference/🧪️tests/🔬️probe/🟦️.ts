import { describe, expect, it } from "vitest";
import { UiDocumentStore } from "@semio-tech/framework-renderer-react";
import type { UiNodeRecord } from "@semio-tech/framework";
import Ajv from "ajv";
import deepEqual from "fast-deep-equal";
import { mountedGisMapProbeV1 } from "../../🪟️presentation/🟦️.tsx";
import mountedGisMapProbeFixture from "../../🧫️fixtures/🔬️mounted-gis-map-probe-v1/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
const deepEqualOracle = deepEqual;
const ownerExport = (name: string) => new Ajv({ strict: true }).compile({ $defs: schema.$defs, $ref: `#/$defs/${name}` });
describe("mounted GIS map probe", () => {
  it("projects only exact acknowledged identity and region ids from the retained Shell store", async () => {
    expect(ownerExport("MountedGisMapProbeSourceV1")(mountedGisMapProbeFixture.source)).toBe(true);
    expect(ownerExport("MountedGisMapProbeV1")(mountedGisMapProbeFixture.expected)).toBe(true);
    expect(mountedGisMapProbeFixture.hostile.every((row) => ownerExport("MountedGisMapProbeRefusalV1")(row))).toBe(true);
    const { encodePackValue } = await import("@semio-tech/framework-os");
    const source = mountedGisMapProbeFixture.source;
    const storeFor = (kind: "tiled-map" | "canvas-2d", regions: readonly unknown[], bytes?: readonly number[]): UiDocumentStore => {
      const store = new UiDocumentStore(source.surface);
      const node: UiNodeRecord = {
        id: 0,
        key: "map-root",
        component: { type: "surface", kind, docSchema: `${kind}@1`, doc: { bytes: Array.from(bytes ?? encodePackValue({ regions })) }, bindings: [] },
        layout: { kind: "leaf", width: "fill", height: "fill" },
        style: { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" },
        activity: "idle",
        disabled: false,
        transition: null,
        accessibility: { label: null, description: null, live: "off", shortcut: null, hidden: false },
        bindings: [],
        menu: null,
        children: [],
      };
      expect(
        store.applyPatch({
          surface: source.surface,
          baseRevision: 0,
          revision: 1,
          ops: [
            { type: "upsert", ...node },
            { type: "setRoot", id: 0 },
          ],
        }),
      ).toEqual({ ok: true });
      return store;
    };
    const retained = {
      scope: source.scope,
      clientInstanceId: source.clientInstanceId,
      activationGeneration: source.activationGeneration,
      verifiedSurfaceId: source.verifiedSurfaceId,
      catalogGenerationId: source.catalogGenerationId,
      componentSha256: source.componentSha256,
      descriptorSha256: source.descriptorSha256,
      browserActorSha256: source.browserActorSha256,
      activeCheckpointId: source.activeCheckpointId,
      descriptorDigestV1: source.descriptorDigestV1,
      frontier: source.frontier,
      uiRevision: source.uiRevision,
      sessionInstanceId: 9,
      windowKindId: source.surface,
      store: storeFor("tiled-map", source.regions),
    };
    const projected = mountedGisMapProbeV1(retained);
    expect(projected).toEqual(mountedGisMapProbeFixture.expected);
    expect(deepEqualOracle(projected, mountedGisMapProbeFixture.expected)).toBe(true);
    expect(mountedGisMapProbeV1(null)).toBeNull();
    expect(mountedGisMapProbeV1({ ...retained, uiRevision: 2 })).toBeNull();
    expect(mountedGisMapProbeV1({ ...retained, activeCheckpointId: "foreign" })).toBeNull();
    expect(mountedGisMapProbeV1({ ...retained, frontier: { ...source.frontier, documentId: "foreign" } })).toBeNull();
    expect(mountedGisMapProbeV1({ ...retained, store: storeFor("canvas-2d", source.regions) })).toBeNull();
    expect(mountedGisMapProbeV1({ ...retained, store: storeFor("tiled-map", [], [0xff]) })).toBeNull();
    expect(mountedGisMapProbeV1({ ...retained, store: storeFor("tiled-map", [source.regions[0], source.regions[0]]) })).toBeNull();
  });
});

