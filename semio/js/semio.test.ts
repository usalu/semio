// #region 🔖Header
// [👤semio📚js🥼semiotest](semiorepo://p/u/semio/b/l/js/f/semio.test.ts)

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🔖Header

import { InvalidKit, InvalidKitValidation, MetabolismKit, MetabolismKitDiff, MetabolismKitDiffed, MetabolismKitDiffInverted, DragDesign, DragPieces, DragOffset, DragDiffDesign, ModelSelectionCases } from "@semio/assets";
import { NodeIO } from "@gltf-transform/core";
import { execFileSync } from "node:child_process";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import * as ElementsBundle from "../../semio-elements/ui";
import { Action as UiAction, buildControlTree, ControlDef } from "../../semio-elements/ui";
import { getOntologyNodeDescriptor, getValidationNodeDescriptor, type OntologyTreeNode, type ValidationTreeNode } from "../../semio-coda/desktop/renderer";
import { createElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it } from "vitest";
import {
  applyDesignDiff,
  applyKitDiff,
  areKitDiffsEqual,
  areKitsEqual,
  areValidationResultsEqual,
  createClusteredDesign,
  Design,
  deserializeKit,
  dragPiecesInDesign,
  exportDesignModel,
  exportKit,
  EXPORT_MODEL_FORMATS,
  flattenDesign,
  getGeometricInsightsForModel,
  getIncludedDesigns,
  getKitChange,
  getKitDiff,
  hasErrors,
  importKit,
  inverseKitDiff,
  Kit,
  Model,
  Plane,
  replaceClusterWithDesign,
  selectBestModel,
  serializeKit,
  sumQualityInDesign,
  validateKit,
  ValidationResult,
} from "./semio";

const TOLERANCE = 0.001;

const planesEqual = (p1?: Plane, p2?: Plane): boolean => {
  if (!p1 || !p2) return false;
  if (!p1.origin || !p2.origin || !p1.xAxis || !p2.xAxis || !p1.yAxis || !p2.yAxis) return false;
  return (
    Math.abs(p1.origin.x - p2.origin.x) < TOLERANCE &&
    Math.abs(p1.origin.y - p2.origin.y) < TOLERANCE &&
    Math.abs(p1.origin.z - p2.origin.z) < TOLERANCE &&
    Math.abs(p1.xAxis.x - p2.xAxis.x) < TOLERANCE &&
    Math.abs(p1.xAxis.y - p2.xAxis.y) < TOLERANCE &&
    Math.abs(p1.xAxis.z - p2.xAxis.z) < TOLERANCE &&
    Math.abs(p1.yAxis.x - p2.yAxis.x) < TOLERANCE &&
    Math.abs(p1.yAxis.y - p2.yAxis.y) < TOLERANCE &&
    Math.abs(p1.yAxis.z - p2.yAxis.z) < TOLERANCE
  );
};

const centersEqual = (c1: { u: number; v: number } | undefined, c2: { u: number; v: number } | undefined): boolean => {
  if (!c1 || !c2) return c1 === c2;
  return Math.abs(c1.u - c2.u) < TOLERANCE && Math.abs(c1.v - c2.v) < TOLERANCE;
};

const findDesign = (kit: Kit, name: string, parentName?: string) => {
  let parentGuid: string | undefined;
  if (parentName) {
    const p = kit.designs?.find((d) => d.name === parentName);
    if (!p) throw new Error(`Parent ${parentName} not found`);
    parentGuid = p.guid;
  }
  const d = kit.designs?.find((d) => d.name === name && (parentGuid ? d.parent?.guid === parentGuid : !d.parent));
  if (!d) throw new Error(`Design ${name} not found`);
  return d;
};

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const EXPORT_REPORTS_DIR = resolve(__dirname, "../../reports/export-design-model");

const writeExportReport = (implementation: string, bytes: Uint8Array<ArrayBufferLike>) => {
  mkdirSync(EXPORT_REPORTS_DIR, { recursive: true });
  const reportPath = resolve(EXPORT_REPORTS_DIR, `${implementation}.gltf`);
  writeFileSync(reportPath, bytes);
  return reportPath;
};

const roundSceneNumber = (value: number) => {
  const rounded = Math.round(value * 10_000) / 10_000;
  return Object.is(rounded, -0) ? 0 : rounded;
};

const composeNodeMatrix = (node: {
  matrix?: number[];
  translation?: number[];
  rotation?: number[];
  scale?: number[];
}) => {
  if (node.matrix) {
    return node.matrix.map((value) => roundSceneNumber(value));
  }
  const translation = node.translation ?? [0, 0, 0];
  const rotation = node.rotation ?? [0, 0, 0, 1];
  const scale = node.scale ?? [1, 1, 1];
  const [x, y, z, w] = rotation;
  const x2 = x + x;
  const y2 = y + y;
  const z2 = z + z;
  const xx = x * x2;
  const xy = x * y2;
  const xz = x * z2;
  const yy = y * y2;
  const yz = y * z2;
  const zz = z * z2;
  const wx = w * x2;
  const wy = w * y2;
  const wz = w * z2;
  const sx = scale[0];
  const sy = scale[1];
  const sz = scale[2];
  return [
    roundSceneNumber((1 - (yy + zz)) * sx),
    roundSceneNumber((xy + wz) * sx),
    roundSceneNumber((xz - wy) * sx),
    0,
    roundSceneNumber((xy - wz) * sy),
    roundSceneNumber((1 - (xx + zz)) * sy),
    roundSceneNumber((yz + wx) * sy),
    0,
    roundSceneNumber((xz + wy) * sz),
    roundSceneNumber((yz - wx) * sz),
    roundSceneNumber((1 - (xx + yy)) * sz),
    0,
    roundSceneNumber(translation[0]),
    roundSceneNumber(translation[1]),
    roundSceneNumber(translation[2]),
    1,
  ];
};

const normalizeSceneGraph = (gltfText: string) => {
  const gltf = JSON.parse(gltfText) as {
    scene?: number;
    scenes?: Array<{ nodes?: number[] }>;
    nodes?: Array<{ name?: string; children?: number[]; matrix?: number[]; mesh?: number; translation?: number[]; rotation?: number[]; scale?: number[] }>;
  };
  const nodes = gltf.nodes ?? [];
  const defaultScene = gltf.scenes?.[gltf.scene ?? 0] ?? { nodes: [] };
  const names = nodes.map((node, index) => node.name ?? `__node_${index}`);
  const parents = new Map<string, string | null>();
  for (const name of names) parents.set(name, null);
  for (let index = 0; index < nodes.length; index += 1) {
    for (const childIndex of nodes[index].children ?? []) {
      parents.set(names[childIndex], names[index]);
    }
  }
  let normalizedRoots = [...(defaultScene.nodes ?? [])].map((index) => names[index]).sort();
  let normalizedNodes = nodes
    .map((node, index) => ({
      name: names[index],
      parent: parents.get(names[index]) ?? null,
      children: [...(node.children ?? [])].map((childIndex) => names[childIndex]).sort(),
      hasMesh: node.mesh !== undefined,
      matrix: composeNodeMatrix(node),
    }))
    .sort((left, right) => left.name.localeCompare(right.name));
  const syntheticWorld = normalizedNodes.find((node) => node.name === "world");
  if (
    syntheticWorld
    && !syntheticWorld.hasMesh
    && syntheticWorld.parent === null
    && syntheticWorld.children.length === 1
    && normalizedRoots.length === 1
    && normalizedRoots[0] === "world"
    && syntheticWorld.matrix.every((value, index) => value === [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1][index])
  ) {
    const childName = syntheticWorld.children[0];
    normalizedRoots = [childName];
    normalizedNodes = normalizedNodes
      .filter((node) => node.name !== "world")
      .map((node) => node.name === childName ? { ...node, parent: null } : node);
  }
  return {
    roots: normalizedRoots,
    nodes: normalizedNodes,
  };
};

const runExportReportCommand = (command: string, args: string[], cwd: string) => {
  execFileSync(command, args, {
    cwd,
    stdio: "pipe",
  });
};

const parseSelfContainedGltf = async (reportText: string) => {
  const parsed = JSON.parse(reportText) as {
    buffers?: Array<{ uri?: string }>;
    images?: Array<{ uri?: string }>;
  };
  const resources: Record<string, Uint8Array> = {};
  const collectResource = (uri?: string) => {
    if (!uri?.startsWith("data:")) return;
    const base64 = uri.slice(uri.indexOf(",") + 1);
    resources[uri] = new Uint8Array(Buffer.from(base64, "base64"));
  };
  for (const buffer of parsed.buffers ?? []) collectResource(buffer.uri);
  for (const image of parsed.images ?? []) collectResource(image.uri);
  const io = new NodeIO();
  return io.readJSON({ json: parsed as any, resources });
};

const getMeshNames = (reportText: string) => {
  const parsed = JSON.parse(reportText) as { meshes?: Array<{ name?: string }> };
  return (parsed.meshes ?? []).map((mesh) => mesh.name).filter((name): name is string => Boolean(name));
};

describe("Change", () => {
  describe("Metabolism", () => {
    const kitOriginal = { ...(MetabolismKit as any), designs: (MetabolismKit as any).designs?.filter((d: any) => !d.parent) };
    const kitDiff = MetabolismKitDiff as any;
    const kitDiffInverted = MetabolismKitDiffInverted as any;
    const kitDiffed = MetabolismKitDiffed as any;

    it("Kit + Change.Forward = DiffedKit & DiffedKit + Change.Backward = Kit", () => {
      const change = getKitChange(kitOriginal, kitDiffed);
      const computedDiff = getKitDiff(kitOriginal, kitDiffed);
      expect(areKitDiffsEqual(computedDiff, kitDiff)).toBe(true);
      const computedInverseDiff = inverseKitDiff(kitOriginal, change.forward);
      expect(areKitDiffsEqual(computedInverseDiff, kitDiffInverted)).toBe(true);
      expect(areKitDiffsEqual(change.forward, kitDiff)).toBe(true);
      expect(areKitDiffsEqual(change.backward, kitDiffInverted)).toBe(true);
      const appliedForward = applyKitDiff(kitOriginal, change.forward);
      expect(areKitsEqual(appliedForward, kitDiffed)).toBe(true);
      const appliedInverse = applyKitDiff(kitDiffed, change.backward);
      expect(areKitsEqual(appliedInverse, kitOriginal)).toBe(true);
    });

    describe("Design/Model", () => {
      it("selectBestModel uses tag filtering + modified jaccard and matches shared semio asset cases", () => {
        const payload = ModelSelectionCases as {
          cases: Array<{
            name: string;
            selectedTagGuids: string[];
            expectedGuid: string | null;
            models: Array<{ guid: string; fileGuid: string; tagGuids: string[] }>;
          }>;
        };
        payload.cases.forEach((testCase) => {
          const models: Model[] = testCase.models.map((model) => ({
            guid: model.guid,
            file: { guid: model.fileGuid },
            tags: model.tagGuids.map((guid) => ({ guid })),
          }));
          const selected = selectBestModel(models, testCase.selectedTagGuids);
          expect(selected?.guid ?? null).toBe(testCase.expectedGuid);
        });
      });
    });
  });
});

describe("Flatten", () => {
  const kit = MetabolismKit as Kit;

  const testFlatten = (designName: string, parentName?: string) => {
    const design = findDesign(kit, designName, parentName);
    const expectedDesign = kit.designs?.find((d) => d.name === "Flat" && d.parent?.guid === design.guid);
    expect(expectedDesign).toBeDefined();
    const flatDesignChange = flattenDesign(kit, design.guid);
    const flatDesign = applyDesignDiff(design, flatDesignChange.forward);

    flatDesign!.pieces?.forEach((p) => {
      const expectedPiece = expectedDesign!.pieces?.find((ep) => ep.name === p.name);
      expect(expectedPiece).toBeDefined();
      expect(p.plane).toBeDefined();
      expect(p.center).toBeDefined();
      expect(planesEqual(p.plane, expectedPiece!.plane)).toBe(true);
      expect(centersEqual(p.center, expectedPiece!.center)).toBe(true);
    });
  };

  describe("Nakagin Capsule Tower", () => {
    it("Kit -> Flatten -> Diff -> Apply = Flat", () => {
      testFlatten("Nakagin Capsule Tower");
    });
    describe("Slanted", () => {
      it("Kit -> Flatten -> Diff -> Apply = Flat", () => {
        testFlatten("Slanted", "Nakagin Capsule Tower");
      });
    });
    describe("Twisted", () => {
      it("Kit -> Flatten -> Diff -> Apply = Flat", () => {
        testFlatten("Twisted", "Nakagin Capsule Tower");
      });
    });
    describe("Dancing", () => {
      it("Kit -> Flatten -> Diff -> Apply = Flat", () => {
        testFlatten("Dancing", "Nakagin Capsule Tower");
      });
    });
  });

  describe("Capsule Dream", () => {
    it("Kit -> Flatten -> Diff -> Apply = Flat", () => {
      testFlatten("Capsule Dream");
    });
  });
});

describe("Roundtrip", () => {
  describe("Metabolism", () => {
    it("Json -> Memory -> Json, Json -> Zip, Zip -> Json", async () => {
      const fs = await import("node:fs");
      const path = await import("node:path");

      const kit = MetabolismKit as unknown as Kit;
      const serializedKit = serializeKit(kit);
      const deserializedKit = deserializeKit(serializedKit);
      expect(areKitsEqual(kit, deserializedKit)).toBe(true);

      const zipPath = path.join(__dirname, "../assets/semio/metabolism.zip");
      const zipBuffer = fs.readFileSync(zipPath);
      const { kit: zipKit } = await importKit(zipBuffer.buffer);
      expect(areKitsEqual(kit, zipKit)).toBe(true);

      const exportedZip = await exportKit(kit);
      const { kit: reKit } = await importKit(exportedZip);
      expect(areKitsEqual(kit, reKit)).toBe(true);
    });
  });
});

describe("Validation", () => {
  describe("Metabolism", () => {
    it("Metabolism Kit -> Validate = Empty report", () => {
      const validKit = MetabolismKit as unknown as Kit;
      expect(hasErrors(validateKit(validKit))).toBe(false);
    });
  });

  describe("Invalid", () => {
    it("Invalid Kit -> Validate = Invalid Report", () => {
      const invalidKit = InvalidKit as unknown as Kit;
      const result = validateKit(invalidKit);
      const expected = InvalidKitValidation as unknown as ValidationResult;
      expect(areValidationResultsEqual(result, expected)).toBe(true);
    });
  });
});

describe("Cluster", () => {
  it("Cluster replacement uses design-guid designPiece and yields included design entry", () => {
    const design = {
      guid: "design-root",
      name: "Root",
      pieces: [
        { guid: "piece-a", type: { guid: "type-a" } },
        { guid: "piece-b", type: { guid: "type-b" } },
        { guid: "piece-c", type: { guid: "type-c" } },
      ],
      connections: [
        {
          guid: "conn-ab",
          connecting: { piece: { guid: "piece-a" } },
          connected: { piece: { guid: "piece-b" } },
        },
        {
          guid: "conn-bc",
          connecting: { piece: { guid: "piece-b" } },
          connected: { piece: { guid: "piece-c" } },
        },
      ],
      createdAt: "2025-01-01T00:00:00.000Z",
      updatedAt: "2025-01-01T00:00:00.000Z",
    } as Design;

    const { clusteredDesign, externalConnections } = createClusteredDesign(design, ["piece-a", "piece-b"], "Cluster");
    const change = replaceClusterWithDesign(design, ["piece-a", "piece-b"], clusteredDesign, externalConnections);
    const updatedDesign = applyDesignDiff(design, change.forward);

    const clusterConnection = updatedDesign.connections?.find((c) => c.guid === "conn-bc");
    expect(clusterConnection?.connecting.designPiece?.guid).toBe(clusteredDesign.guid);
    expect(clusterConnection?.connected.designPiece?.guid).toBeUndefined();

    const included = getIncludedDesigns(updatedDesign);
    expect(included.length).toBe(1);
    expect(included[0].guid).toBe(clusteredDesign.guid);
    expect(included[0].designGuid).toBe(clusteredDesign.guid);
  });
});

describe("Drag", () => {
  it("Design + Pieces + Offset = DiffDesign", () => {
    const design = DragDesign as unknown as Design;
    const pieces = DragPieces as unknown as Design;
    const offset = DragOffset as { u: number; v: number };
    const expectedDiff = DragDiffDesign as any;
    const computedDiff = dragPiecesInDesign(design, pieces, offset);
    const computedPieceUpdates = (computedDiff.pieces?.updated ?? []).sort((a, b) => a.piece.guid.localeCompare(b.piece.guid));
    const expectedPieceUpdates = (expectedDiff.pieces?.updated ?? []).sort((a: any, b: any) => a.piece.guid.localeCompare(b.piece.guid));
    expect(computedPieceUpdates.length).toBe(expectedPieceUpdates.length);
    for (let i = 0; i < computedPieceUpdates.length; i++) {
      expect(computedPieceUpdates[i].piece.guid).toBe(expectedPieceUpdates[i].piece.guid);
      expect(computedPieceUpdates[i].diff.center?.u).toBe(expectedPieceUpdates[i].diff.center.u);
      expect(computedPieceUpdates[i].diff.center?.v).toBe(expectedPieceUpdates[i].diff.center.v);
    }
    const computedConnUpdates = (computedDiff.connections?.updated ?? []).sort((a, b) => a.connection.guid.localeCompare(b.connection.guid));
    const expectedConnUpdates = (expectedDiff.connections?.updated ?? []).sort((a: any, b: any) => a.connection.guid.localeCompare(b.connection.guid));
    expect(computedConnUpdates.length).toBe(expectedConnUpdates.length);
    for (let i = 0; i < computedConnUpdates.length; i++) {
      expect(computedConnUpdates[i].connection.guid).toBe(expectedConnUpdates[i].connection.guid);
      expect(computedConnUpdates[i].diff.u).toBe(expectedConnUpdates[i].diff.u);
      expect(computedConnUpdates[i].diff.v).toBe(expectedConnUpdates[i].diff.v);
    }
  });
});

describe("Sketchpad ControlTree", () => {
  it("builds nested folders from paths and applies case-insensitive filter on leaf keys", () => {
    const controls: ControlDef[] = [
      { path: "Transform/Position/X", controlKind: "number", value: 1, onChange: () => { } },
      { path: "Transform/Position/Y", controlKind: "number", value: 2, onChange: () => { } },
      { path: "Appearance/Material/roughness", controlKind: "slider", value: 0.5, onChange: () => { } },
    ];
    const folderSettings = {
      Transform: { path: "Transform", order: 2 },
      "Appearance/Material": { path: "Appearance/Material", order: 1, collapsed: true },
    };
    const fullTree = buildControlTree(controls, "", folderSettings);
    expect(Object.keys(fullTree)).toEqual(["Transform", "Appearance"]);
    expect(fullTree.Transform.kind).toBe("folder");
    expect(fullTree.Transform.order).toBe(2);
    expect(fullTree.Transform.children?.Position.kind).toBe("folder");
    expect(fullTree.Transform.children?.Position.children?.X.kind).toBe("control");
    expect(fullTree.Appearance.children?.Material.order).toBe(1);
    const filteredTree = buildControlTree(controls, "rouGH", folderSettings);
    expect(Object.keys(filteredTree)).toEqual(["Appearance"]);
    expect(filteredTree.Appearance.children?.Material.children?.roughness.kind).toBe("control");
    expect(filteredTree.Appearance.children?.Material.children?.roughness.path).toBe("Appearance/Material/roughness");
  });
});

describe("Elements Bundle", () => {
  it("sources shared element primitives directly from semio-elements ui", () => {
    expect(UiAction).toBe(ElementsBundle.Action);
    expect(buildControlTree).toBe(ElementsBundle.buildControlTree);
    expect(ElementsBundle.LevelProvider).toBeDefined();
    expect(ElementsBundle.SectionSpecificity).toBeDefined();
  });

  it("renders an explicit TreeItem label even when an id is present", () => {
    const html = renderToStaticMarkup(
      createElement(
        ElementsBundle.Tree,
        null,
        createElement(ElementsBundle.TreeItem, {
          id: "storybook.missing.translation.key",
          label: createElement("span", { className: "tree-explicit-label" }, "Explicit Tree Label"),
          icon: createElement("span", null, "∧"),
        }),
      ),
    );

    expect(html).toContain("Explicit Tree Label");
    expect(html).toContain("tree-explicit-label");
  });
});

describe("Coda Tree Descriptors", () => {
  it("keeps ontology fragments and validation witness/count semantics stable", () => {
    const ontologyNode: OntologyTreeNode = {
      id: "ontology-1",
      kind: "ExactCardinality",
      label: "EXACTLY 2 verbindet",
      fragment: "verbindet exactly 2 (...)",
      children: [],
    };
    const ontologyDescriptor = getOntologyNodeDescriptor(ontologyNode);
    expect(ontologyDescriptor.icon).toBe("=n");
    expect(ontologyDescriptor.primaryText).toBe("EXACTLY 2 verbindet");
    expect(ontologyDescriptor.secondaryText).toBe("verbindet exactly 2 (...)");

    const countedWitness: ValidationTreeNode = {
      id: "validation-1",
      kind: "Witness",
      label: "Geschoss_EG",
      individual: "Geschoss_EG",
      truth: "true",
      counted: true,
      summary: "counted filler 1 of 2",
      children: [],
    };
    const countedWitnessDescriptor = getValidationNodeDescriptor(countedWitness);
    expect(countedWitnessDescriptor.primaryText).toBe("Geschoss_EG");
    expect(countedWitnessDescriptor.chips).toContain("counted");
    expect(countedWitnessDescriptor.dimmed).toBe(false);

    const notMatchingWitness: ValidationTreeNode = {
      id: "validation-2",
      kind: "Witness",
      label: "Technikraum_Dach",
      individual: "Technikraum_Dach",
      truth: "unknown",
      counted: false,
      summary: "additional filler that does not satisfy the restriction",
      children: [],
    };
    const notMatchingWitnessDescriptor = getValidationNodeDescriptor(notMatchingWitness);
    expect(notMatchingWitnessDescriptor.chips).toContain("not matching");
    expect(notMatchingWitnessDescriptor.dimmed).toBe(true);

    const cardinalityNode: ValidationTreeNode = {
      id: "validation-3",
      kind: "ExactCardinality",
      label: "EXACTLY 1 in",
      fragment: "in exactly 1 (...)",
      truth: "true",
      expectedCardinality: 1,
      matchingCount: 1,
      children: [],
    };
    const cardinalityDescriptor = getValidationNodeDescriptor(cardinalityNode);
    expect(cardinalityDescriptor.icon).toBe("=n");
    expect(cardinalityDescriptor.chips).toContain("1/1");
    expect(cardinalityDescriptor.secondaryText).toBe("in exactly 1 (...)");

    const dataValueNode: ValidationTreeNode = {
      id: "validation-4",
      kind: "DataValue",
      label: "180.0",
      value: "180.0",
      datatype: "xsd:float",
      truth: "true",
      children: [],
    };
    const dataValueDescriptor = getValidationNodeDescriptor(dataValueNode);
    expect(dataValueDescriptor.primaryText).toBe("180.0");
    expect(dataValueDescriptor.chips).toContain("xsd:float");
  });
});

describe("Design/Quality/Sum", () => {
  const kit = MetabolismKit as Kit;
  describe("Nakagin Capsule Tower", () => {
    it("sums effective floor area to ~2349.53", () => {
      const design = kit.designs?.find((d) => d.name === "Nakagin Capsule Tower" && !d.parent);
      expect(design).toBeDefined();
      const quality = kit.qualities?.find((q) => q.name === "effective floor area");
      expect(quality).toBeDefined();
      const result = sumQualityInDesign(kit, design!.guid, quality!.guid);
      expect(Math.abs(result - 2349.53)).toBeLessThan(0.01);
    });
  });
});

describe("ExportDesignModel", () => {
  const kit = MetabolismKit as Kit;
  const design = kit.designs?.find((d) => d.name === "Nakagin Capsule Tower" && !d.parent)!;

  it("exports .glb format with valid GLB header", async () => {
    const result = await exportDesignModel(kit, design.guid, ".glb");
    expect(result.byteLength).toBeGreaterThan(0);

    const view = new DataView(result);
    const magic = view.getUint32(0, true);
    expect(magic).toBe(0x46546c67);

    const version = view.getUint32(4, true);
    expect(version).toBe(2);

    const totalLength = view.getUint32(8, true);
    expect(totalLength).toBe(result.byteLength);
  });

  it("exports .gltf format as valid JSON string", async () => {
    const result = await exportDesignModel(kit, design.guid, ".gltf");
    const decoder = new TextDecoder();
    const str = decoder.decode(result);
    expect(() => JSON.parse(str)).not.toThrow();
    const parsed = JSON.parse(str);
    expect(parsed).toBeDefined();
    expect(typeof parsed).toBe("object");
  });

  it("EXPORT_MODEL_FORMATS includes .glb and .gltf", () => {
    expect(EXPORT_MODEL_FORMATS[".glb"]).toBeDefined();
    expect(EXPORT_MODEL_FORMATS[".gltf"]).toBeDefined();
  });

  it("exports identical Nakagin scene graph across implementations and writes reports", async () => {
    mkdirSync(EXPORT_REPORTS_DIR, { recursive: true });

    const jsResult = new Uint8Array(await exportDesignModel(kit, design.guid, ".gltf"));
    writeExportReport("js", jsResult);

    runExportReportCommand("uv", ["run", "pytest", "semio.test.py", "-k", "export_scene_graph_report", "-q"], resolve(__dirname, "../py"));
    runExportReportCommand("go", ["test", "./...", "-run", "TestExportDesignModelSceneGraphReport$", "-count=1"], resolve(__dirname, "../go"));
    runExportReportCommand("cargo", ["test", "export_scene_graph_report", "--", "--nocapture"], resolve(__dirname, "../rs"));
    runExportReportCommand("dotnet", ["test", "Semio.Tests.csproj", "-f", "net8.0", "--filter", "FullyQualifiedName=Semio.Tests.Tests+ExportDesignModel.Nakagin_Capsule_Tower_Export_Scene_Graph_Report"], resolve(__dirname, "../net/Semio.Tests"));

    const implementations = ["js", "py", "go", "rs", "net"] as const;
    const normalizedByImplementation = Object.fromEntries(
      implementations.map((implementation) => {
        const reportPath = resolve(EXPORT_REPORTS_DIR, `${implementation}.gltf`);
        const reportText = readFileSync(reportPath, "utf8");
        return [implementation, normalizeSceneGraph(reportText)];
      }),
    );

    writeFileSync(resolve(EXPORT_REPORTS_DIR, "scene-graphs.json"), JSON.stringify(normalizedByImplementation, null, 2));

    const baseline = normalizedByImplementation.js;
    for (const implementation of implementations) {
      expect(normalizedByImplementation[implementation]).toEqual(baseline);
    }

    for (const implementation of implementations) {
      const reportPath = resolve(EXPORT_REPORTS_DIR, `${implementation}.gltf`);
      const reportText = readFileSync(reportPath, "utf8");
      const parsed = JSON.parse(reportText) as { buffers?: Array<{ uri?: string }>; images?: Array<{ uri?: string; bufferView?: number }> };
      for (const buffer of parsed.buffers ?? []) {
        expect(buffer.uri?.startsWith("data:")).toBe(true);
      }
      for (const image of parsed.images ?? []) {
        expect(image.uri?.startsWith("data:") ?? image.bufferView !== undefined).toBe(true);
      }
      const doc = await parseSelfContainedGltf(reportText);
      expect(doc.getRoot().listMeshes().length).toBeGreaterThan(0);
      const meshNames = getMeshNames(reportText);
      expect(meshNames.some((name) => name === "base.glb")).toBe(true);
      expect(meshNames.some((name) => /^capsule_.*\.glb$/i.test(name))).toBe(true);
    }
  }, 300000);
});

describe("Model/KPI", () => {
  it("getGeometricInsightsForModel(nakagin-capsule-tower.gltf) returns insights", async () => {
    const modelPath = resolve(__dirname, "../assets/semio/nakagin-capsule-tower.gltf");
    const insights = await getGeometricInsightsForModel(modelPath);
    expect(insights.boundingBoxMin).toBeDefined();
    expect(insights.boundingBoxMax).toBeDefined();
    expect(insights.dimensionX).toBeDefined();
    expect(insights.dimensionY).toBeDefined();
    expect(insights.dimensionZ).toBeDefined();
    expect(insights.characteristicLength).toBeDefined();
    expect(insights.totalSurfaceArea).toBeDefined();
    expect(insights.vertexCount).toBeGreaterThan(0);
    expect(insights.faceCount).toBeGreaterThan(0);
    expect(insights.centroid).toBeDefined();
    expect(insights.eulerCharacteristic).toBeDefined();
  });
});
