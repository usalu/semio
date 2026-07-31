#!/usr/bin/env bun
/** Runtime check: curvilinear capture target + blit shader quality knobs. */
import { LinearFilter, LinearSRGBColorSpace, WebGLRenderTarget } from "three";
import { WORLD_CURVILINEAR_CAPTURE_TARGET_OPTIONS, WORLD_CURVILINEAR_FRAGMENT_SHADER } from "../../../../../../infinite/world/r3f/index.tsx";

const dpr = 2;
const css = { width: 800, height: 600 };
const width = Math.max(1, Math.floor(css.width * dpr));
const height = Math.max(1, Math.floor(css.height * dpr));
const target = new WebGLRenderTarget(width, height, { ...WORLD_CURVILINEAR_CAPTURE_TARGET_OPTIONS });

const report = {
  capture: `${target.width}x${target.height}`,
  css: `${css.width}x${css.height}`,
  pixelRatio: dpr,
  magFilter: target.texture.magFilter,
  minFilter: target.texture.minFilter,
  colorSpace: target.texture.colorSpace,
  expectsLinearFilter: LinearFilter,
  expectsLinearSRGB: LinearSRGBColorSpace,
  hasColorspaceInclude: WORLD_CURVILINEAR_FRAGMENT_SHADER.includes("#include <colorspace_fragment>"),
  magOk: target.texture.magFilter === LinearFilter,
  colorOk: target.texture.colorSpace === LinearSRGBColorSpace,
};

console.log("[DEBUG] fisheye capture quality", report);
target.dispose();

if (!report.magOk || !report.colorOk || !report.hasColorspaceInclude) {
  process.exit(1);
}
