/** 🎨️ CAD renderer geometry targets: the pick target styles and hover keys and the typology pattern material. */
import { type MeshStandardMaterial as ThreeMeshStandardMaterial } from "@semio-tech/framework/ui/targets/react";
import { ephemeralMap } from "@semio-tech/framework";
import { WORLD_LOCKED_OPACITY_SCALE } from "@semio-tech/infinite-world-r3f";
import { type ModelEntityKind } from "@semio-tech/s-spatial-kernel-ts/geometry";
import { typologyStyleCacheKey } from "@semio-tech/s-spatial-kernel-ts/geometry/typology";
import { type ResolvedTypologyStyle } from "@semio-tech/s-spatial-kernel-ts/geometry/definition-catalog";
import { THREE } from "../../🔌️ports/🟦️.ts";
import { spatialSceneColors } from "../../🎨️style/🟦️.ts";
import { GEOMETRY_KIND_TO_OBJECT_PICK, type SpatialPickTarget } from "../🎯️pick-model/🟦️.ts";

// #region 🧲️GeometryInteraction
/** 🎨️ Maps a resolved typology style to committed-mesh material props. */
export function typologyStyleToMaterialProps(style: ResolvedTypologyStyle): { readonly color: string; readonly emissive: string; readonly opacity: number } {
  return { color: style.color, emissive: style.color, opacity: style.opacity };
}

export function targetStyle(target: SpatialPickTarget, hovered: boolean, selected: boolean, typologyStyle?: ResolvedTypologyStyle, locked = false): { color: string; emissive: string; opacity: number; lineWidth: number } {
  const palette = spatialSceneColors();
  if (selected) return { color: palette.selected, emissive: palette.selectedEmissive, opacity: target.kind === "vertex" ? 1 : 0.34, lineWidth: 9 };
  if (hovered) return { color: palette.hovered, emissive: palette.hoveredEmissive, opacity: target.kind === "vertex" ? 1 : 0.28, lineWidth: 8 };
  if (locked) {
    const dimOpacity = (target.kind === "vertex" ? 0.55 : 0.12) * WORLD_LOCKED_OPACITY_SCALE;
    if (typologyStyle) {
      return { color: typologyStyle.color, emissive: typologyStyle.color, opacity: dimOpacity, lineWidth: 4 };
    }
    if (target.kind === "vertex") return { color: palette.vertex, emissive: palette.vertexEmissive, opacity: dimOpacity, lineWidth: 4 };
    if (target.kind === "edge") return { color: palette.edge, emissive: palette.edgeEmissive, opacity: dimOpacity, lineWidth: 4 };
    if (target.kind === "object" && !target.geometryKind) return { color: palette.object, emissive: palette.objectEmissive, opacity: dimOpacity, lineWidth: 5 };
    return { color: palette.face, emissive: palette.faceEmissive, opacity: dimOpacity, lineWidth: 4 };
  }
  if (typologyStyle) {
    if (target.kind === "vertex") return { color: typologyStyle.color, emissive: typologyStyle.color, opacity: 1, lineWidth: 5 };
    if (target.kind === "edge") return { color: typologyStyle.edgeColor, emissive: typologyStyle.edgeColor, opacity: 0.85, lineWidth: 5 };
    if (target.kind === "object" && !target.geometryKind) return { color: typologyStyle.color, emissive: typologyStyle.color, opacity: typologyStyle.opacity, lineWidth: 7 };
    return { color: typologyStyle.color, emissive: typologyStyle.color, opacity: Math.min(0.42, typologyStyle.opacity), lineWidth: 5 };
  }
  if (target.kind === "vertex") return { color: palette.vertex, emissive: palette.vertexEmissive, opacity: 1, lineWidth: 5 };
  if (target.kind === "edge") return { color: palette.edge, emissive: palette.edgeEmissive, opacity: 0.8, lineWidth: 5 };
  if (target.kind === "object" && !target.geometryKind) return { color: palette.object, emissive: palette.objectEmissive, opacity: 0.28, lineWidth: 7 };
  return { color: palette.face, emissive: palette.faceEmissive, opacity: 0.16, lineWidth: 5 };
}

export function pinnedPickTargetKeys(keys: ReadonlySet<string>): ReadonlySet<string> {
  const out = new Set<string>();
  for (const key of keys) {
    out.add(key);
    const colon = key.indexOf(":");
    if (colon < 0) continue;
    const kind = key.slice(0, colon);
    const id = key.slice(colon + 1);
    const mapped = GEOMETRY_KIND_TO_OBJECT_PICK[kind as ModelEntityKind];
    if (mapped) out.add(`${mapped}:${id}`);
    if (kind === "object") out.add(`solid:${id}`);
  }
  return out;
}

/** 🪪️ Expands a hover/selection key across geometry pick aliases (`solid:foo` ↔ `object:foo`). */
export function spatialHoverKeyAliases(key: string | null | undefined): ReadonlySet<string> {
  if (!key) return new Set();
  return pinnedPickTargetKeys(new Set([key]));
}

/** 🪪️ True when a canvas pick key matches a shared hover/selection key (including aliases). */
export function spatialHoverKeysMatch(left: string | null | undefined, right: string | null | undefined): boolean {
  if (!left || !right) return false;
  if (left === right) return true;
  return spatialHoverKeyAliases(left).has(right);
}

// #endregion 🧲️GeometryInteraction
// #region 🎨️TypologyPatternMaterial
const typologyPatternMaterialCache = ephemeralMap<string, ThreeMeshStandardMaterial>("s.plugins.cad.modules.renderer.component.tsx.typologyPatternMaterialCache");

function typologyPatternKindUniform(kind: ResolvedTypologyStyle["pattern"]["kind"]): number {
  if (kind === "hatch") return 1;
  if (kind === "crosshatch") return 2;
  if (kind === "dots") return 3;
  return 0;
}

const TYPOLOGY_PATTERN_VERTEX_PREFIX = "varying vec3 vTypologyWorldPos;\n";
const TYPOLOGY_PATTERN_VERTEX_INJECT = `#include <worldpos_vertex>
vec4 typologyWorldPosition = vec4(transformed, 1.0);
#ifdef USE_BATCHING
  typologyWorldPosition = batchingMatrix * typologyWorldPosition;
#endif
#ifdef USE_INSTANCING
  typologyWorldPosition = instanceMatrix * typologyWorldPosition;
#endif
typologyWorldPosition = modelMatrix * typologyWorldPosition;
vTypologyWorldPos = typologyWorldPosition.xyz;
`;
const TYPOLOGY_PATTERN_FRAGMENT_PREFIX = "varying vec3 vTypologyWorldPos;\nuniform float uPatternKind;\nuniform float uPatternDirection;\nuniform float uPatternSpacing;\nuniform float uPatternLineWidth;\nuniform vec3 uPatternColor;\n";
const TYPOLOGY_PATTERN_FRAGMENT_BLEND = `
vec3 applyTypologyPattern(vec3 baseColor, vec3 worldPos, vec3 surfaceNormal) {
  if (uPatternKind < 0.5) return baseColor;
  vec3 n = normalize(surfaceNormal);
  vec3 ref = abs(n.z) < 0.9 ? vec3(0.0, 0.0, 1.0) : vec3(0.0, 1.0, 0.0);
  vec3 tangent = normalize(cross(n, ref));
  vec3 bitangent = cross(n, tangent);
  float angle = uPatternDirection;
  vec2 planeCoord = vec2(dot(worldPos, tangent), dot(worldPos, bitangent));
  mat2 rot = mat2(cos(angle), -sin(angle), sin(angle), cos(angle));
  planeCoord = rot * planeCoord;
  float hatch = 0.0;
  if (uPatternKind < 1.5) {
    float stripe = abs(fract(planeCoord.x / uPatternSpacing) - 0.5) * uPatternSpacing;
    hatch = 1.0 - smoothstep(0.0, uPatternLineWidth, stripe);
  } else if (uPatternKind < 2.5) {
    float stripeA = abs(fract(planeCoord.x / uPatternSpacing) - 0.5) * uPatternSpacing;
    float stripeB = abs(fract(planeCoord.y / uPatternSpacing) - 0.5) * uPatternSpacing;
    float hatchA = 1.0 - smoothstep(0.0, uPatternLineWidth, stripeA);
    float hatchB = 1.0 - smoothstep(0.0, uPatternLineWidth, stripeB);
    hatch = max(hatchA, hatchB);
  } else {
    vec2 cell = planeCoord / uPatternSpacing;
    vec2 grid = fract(cell) - 0.5;
    hatch = 1.0 - smoothstep(uPatternLineWidth * 0.5, uPatternLineWidth, length(grid));
  }
  return mix(baseColor, uPatternColor, hatch * 0.88);
}
`;

/** 🎨️ Builds or reuses a shaded material with optional procedural typology pattern. */
export function createTypologyStyledMaterial(style: ResolvedTypologyStyle): ThreeMeshStandardMaterial {
  const key = typologyStyleCacheKey(style);
  const cached = typologyPatternMaterialCache.get(key);
  if (cached) return cached;
  const props = typologyStyleToMaterialProps(style);
  const material = new THREE.MeshStandardMaterial({
    color: props.color,
    emissive: props.emissive,
    emissiveIntensity: 0.08,
    metalness: 0,
    roughness: 0.45,
    side: THREE.DoubleSide,
    transparent: true,
    opacity: props.opacity,
    depthWrite: false,
    polygonOffset: true,
    polygonOffsetFactor: 1,
    polygonOffsetUnits: 1,
  });
  if (style.pattern.kind !== "none") {
    const pattern = style.pattern;
    material.onBeforeCompile = (shader) => {
      shader.uniforms.uPatternKind = { value: typologyPatternKindUniform(pattern.kind) };
      shader.uniforms.uPatternDirection = { value: (pattern.direction * Math.PI) / 180 };
      shader.uniforms.uPatternSpacing = { value: pattern.spacing };
      shader.uniforms.uPatternLineWidth = { value: pattern.lineWidth };
      shader.uniforms.uPatternColor = { value: new THREE.Color(pattern.color) };
      shader.vertexShader = shader.vertexShader.replace("#include <common>", `#include <common>\n${TYPOLOGY_PATTERN_VERTEX_PREFIX}`);
      shader.vertexShader = shader.vertexShader.replace("#include <worldpos_vertex>", TYPOLOGY_PATTERN_VERTEX_INJECT);
      shader.fragmentShader = shader.fragmentShader.replace("#include <common>", `#include <common>\n${TYPOLOGY_PATTERN_FRAGMENT_PREFIX}`);
      shader.fragmentShader = shader.fragmentShader.replace("#include <output_fragment>", `${TYPOLOGY_PATTERN_FRAGMENT_BLEND}outgoingLight = applyTypologyPattern(outgoingLight, vTypologyWorldPos, normal);\n#include <output_fragment>`);
    };
    material.customProgramCacheKey = () => key;
  }
  typologyPatternMaterialCache.set(key, material);
  return material;
}
// #endregion 🎨️TypologyPatternMaterial
