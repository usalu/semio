/** @emoji ⭕ One-off migration: puzzle 2d fixture nodes → uniform circles (ticket workspace). */
import { readFileSync, writeFileSync } from "node:fs";

const RADIUS = 20;

function rayToRectEdge(hw: number, hh: number, ux: number, uy: number): { x: number; y: number } {
  const ax = Math.abs(ux);
  const ay = Math.abs(uy);
  if (ax < 1e-12 && ay < 1e-12) {
    return { x: 0, y: 0 };
  }
  const sx = ax < 1e-12 ? Number.POSITIVE_INFINITY : hw / ax;
  const sy = ay < 1e-12 ? Number.POSITIVE_INFINITY : hh / ay;
  const t = Math.min(sx, sy);
  return { x: ux * t, y: uy * t };
}

function handlePositionRectangle(cx: number, cy: number, width: number, height: number, angle: number): { x: number; y: number } {
  const hw = width / 2;
  const hh = height / 2;
  const ux = -Math.sin(angle);
  const uy = -Math.cos(angle);
  const local = rayToRectEdge(hw, hh, ux, uy);
  return { x: cx + local.x, y: cy + local.y };
}

function rectangleHandleAngleToCircleAngle(width: number, height: number, rectAngle: number): number {
  const p = handlePositionRectangle(0, 0, width, height, rectAngle);
  return Math.atan2(p.y, p.x);
}

type RawNode = Record<string, unknown> & {
  handles?: { angle: number; [key: string]: unknown }[];
  shape?: string;
  width?: number;
  height?: number;
  radius?: number;
};

function migrateNode(node: RawNode): RawNode {
  const isRect = node.shape === "rectangle";
  const width = isRect ? Number(node.width) : (Number(node.radius) || RADIUS) * 2;
  const height = isRect ? Number(node.height) : (Number(node.radius) || RADIUS) * 2;
  const handles = (node.handles ?? []).map((h) => ({
    ...h,
    angle: isRect ? rectangleHandleAngleToCircleAngle(width, height, h.angle) : h.angle,
  }));
  const next: RawNode = { ...node, handles, radius: RADIUS };
  delete next.shape;
  delete next.width;
  delete next.height;
  return next;
}

function migrateFixture(path: string): void {
  const data = JSON.parse(readFileSync(path, "utf8")) as {
    nodes?: RawNode[];
    meta?: { kindCatalogs?: { nodes?: Record<string, unknown>[] } };
  };
  if (Array.isArray(data.nodes)) {
    data.nodes = data.nodes.map(migrateNode);
  }
  const catalogNodes = data.meta?.kindCatalogs?.nodes;
  if (Array.isArray(catalogNodes)) {
    data.meta!.kindCatalogs!.nodes = catalogNodes.map((row) => {
      const next = { ...row };
      delete next.shape;
      delete next.scale;
      delete next.defaultShapeProps;
      return next;
    });
  }
  writeFileSync(path, `${JSON.stringify(data, null, 2)}\n`);
  console.log(`[DEBUG] migrated ${path}`);
}

for (const path of process.argv.slice(2)) {
  migrateFixture(path);
}
