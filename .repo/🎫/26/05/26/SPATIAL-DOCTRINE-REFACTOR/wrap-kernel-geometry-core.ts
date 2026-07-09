/** @emoji 🔄 Wrap brep refs/records in export namespace kernelGeometry in core/index.ts. */
import { readFileSync, writeFileSync } from "node:fs";

const path = "c:/git/compose/spatial/js/core/index.ts";
let s = readFileSync(path, "utf8");

if (!s.includes("export namespace kernelGeometry")) {
  s = s.replace(
    "// #region 🪪Refs\n/** @emoji 🪪 Opaque branded string ids for editable topology entities. */\nexport type AnchorRef",
    "// #region 🧱kernelGeometry\n/** @emoji 🧱 Kernel-private brep document (use `Object` / `Model` in framework code). */\nexport namespace kernelGeometry {\n/** @emoji 🪪 Opaque branded string ids for editable topology entities. */\nexport type AnchorRef",
  );
  s = s.replace(
    "/** @emoji 🪪 Builds a branded `CellRef` from an opaque id string. */\nexport function cellRef(id: string): CellRef {\n\treturn id as CellRef;\n}\n// #endregion 🪪Refs",
    "/** @emoji 🪪 Builds a branded `CellRef` from an opaque id string. */\nexport function cellRef(id: string): CellRef {\n\treturn id as CellRef;\n}\n}\n\nexport type AnchorRef = kernelGeometry.AnchorRef;\nexport type VertexRef = kernelGeometry.VertexRef;\nexport type EdgeRef = kernelGeometry.EdgeRef;\nexport type WireRef = kernelGeometry.WireRef;\nexport type FaceRef = kernelGeometry.FaceRef;\nexport type ShellRef = kernelGeometry.ShellRef;\nexport type CellRef = kernelGeometry.CellRef;\nexport type CellComplexRef = kernelGeometry.CellComplexRef;\nexport type ClusterRef = kernelGeometry.ClusterRef;\nexport type EditableEntityKind = kernelGeometry.EditableEntityKind;\nexport const cellRef = kernelGeometry.cellRef;\n// #endregion 🧱kernelGeometry",
  );
  s = s.replace("/** @emoji 🧱 Editable topology kinds from `spatial/AGENTS.md`. */\nexport type EditableEntityKind =", "/** @emoji 🧱 Kernel-private geometry entity kinds for selection and query adapters. */\nexport type EditableEntityKind =");
}

if (!s.includes("export namespace kernelGeometry") || s.indexOf("export namespace kernelGeometry") === s.lastIndexOf("export namespace kernelGeometry")) {
  s = s.replace("// #region 🧱ModelGeometry\n/** @emoji 🧱 Kernel-private vertex payload", "export namespace kernelGeometry {\n// #region 🧱ModelGeometry\n/** @emoji 🧱 Kernel-private vertex payload");
  s = s.replace(
    "readonly memberIds: readonly string[];\n}\n\n/** @emoji 🪪 Opaque object id in a model.",
    "readonly memberIds: readonly string[];\n}\n}\n\nexport type VertexRecord = kernelGeometry.VertexRecord;\nexport type AnchorAttachment = kernelGeometry.AnchorAttachment;\nexport type AnchorRecord = kernelGeometry.AnchorRecord;\nexport type EdgeRecord = kernelGeometry.EdgeRecord;\nexport type WireRecord = kernelGeometry.WireRecord;\nexport type FaceSurface = kernelGeometry.FaceSurface;\nexport type FaceRecord = kernelGeometry.FaceRecord;\nexport type ShellRecord = kernelGeometry.ShellRecord;\nexport type CellSolid = kernelGeometry.CellSolid;\nexport type CellRecord = kernelGeometry.CellRecord;\nexport type CellComplexRecord = kernelGeometry.CellComplexRecord;\nexport type ClusterRecord = kernelGeometry.ClusterRecord;\nexport type KernelGeometryJson = kernelGeometry.KernelGeometryJson;\n\n/** @emoji 🪪 Opaque object id in a model.",
  );
}

writeFileSync(path, s);
