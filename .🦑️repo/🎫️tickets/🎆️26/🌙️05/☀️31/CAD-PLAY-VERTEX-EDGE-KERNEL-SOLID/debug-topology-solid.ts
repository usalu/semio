#!/usr/bin/env bun
import { applyModelDiff, Model, solidRef } from "@semio-tech/cad-js-core";
import { BrepjsKernel, boxModelDiff } from "@semio-tech/cad-js-kernel-brepjs";

const kernel = new BrepjsKernel();
const g = new Model();
const solid = solidRef("moved-box");
applyModelDiff(g, boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, solid));
await kernel.syncSolidsFromModel(g);
console.log("[DEBUG] volume before", await kernel.volume(solid));
const topVertex = Object.keys(g.vertices).find((id) => id.includes("v101"))!;
g.vertices[topVertex] = { ...g.vertices[topVertex]!, position: [2, 1, 1] };
g.bump();
await kernel.syncSolidsFromModel(g);
console.log("[DEBUG] volume after", await kernel.volume(solid));
