#!/usr/bin/env bun
import fs from "node:fs";
import path from "node:path";

const root = path.resolve(import.meta.dir, "../../../../../../");
const fixturePath = path.join(root, "puzzle/2d/fixture/concrete-forest.2d.json");

const PUZZLE_2D_NODE_RADIUS_PX = 24;

function puzzle2dRectangleHandleAngleToCircleAngle(width: number, height: number, rectAngle: number): number {
  const hw = width / 2;
  const hh = height / 2;
  const dx = Math.cos(rectAngle) * hw;
  const dy = Math.sin(rectAngle) * hh;
  return Math.atan2(dy / hh, dx / hw);
}

const raw = JSON.parse(fs.readFileSync(fixturePath, "utf8")) as Record<string, unknown>;
const nodes = raw.nodes as Record<string, unknown>[];
const node = nodes[0];
const width = Number(node.width);
const height = Number(node.height);
const handles = (node.handles as { id: string; handleKind: string; angle: number; radius: number }[]).map((handle) => ({
  ...handle,
  angle: puzzle2dRectangleHandleAngleToCircleAngle(width, height, handle.angle),
}));
nodes[0] = {
  id: node.id,
  nodeKind: node.nodeKind,
  shape: "circle",
  x: node.x,
  y: node.y,
  radius: PUZZLE_2D_NODE_RADIUS_PX,
  text: node.text,
  handles,
};
raw.meta = {
  manifestId: "concrete-forest",
  kindCompatibility: (raw.meta as { kindCompatibility?: unknown }).kindCompatibility,
};
fs.writeFileSync(fixturePath, `${JSON.stringify(raw, null, 2)}\n`);
console.log("[DEBUG] converted concrete-forest.2d.json");
