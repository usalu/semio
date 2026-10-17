// #region 🧲️Header
/** 🧭️ `@semio-tech/cad-js` — CAD domain module facet. See `cad/AGENTS.md`. */
import type { Vec3 } from "@semio-tech/framework/3d";

// #endregion 🧲️Header
import { InteractionSpec } from "@semio-tech/s-spatial-kernel-ts/geometry/interaction-spec";
import { Model } from "@semio-tech/s-spatial-kernel-ts/geometry/model";
import { ExprEnv, evalExpr } from "@semio-tech/s-spatial-kernel-ts/geometry/expressions";
import { SpatialPreviewKernel } from "@semio-tech/s-spatial-kernel-ts/spatial";
import { withResolvedInteractionPointsContext } from "./🧲️selection-targets/🟦️.ts";
import { CURSOR_RAW_CTX, interactionLengthEntryForState, interactionScalarEntryForState, positiveHeightLock, positiveLengthLock, readInteractionContextVec3, SCALAR_AXIS_T_CTX, scalarEntryAxis, scalarEntryAxisBase } from "./🎰️interaction-state/🟦️.ts";

// #region 📦️🎬️actions
// #region 🖼️Display
/** 🖼️ Resolved display primitive for renderer adapters. */
export interface DisplayItem {
  readonly kind: string;
  readonly id: string;
  readonly role?: string;
  readonly params?: Record<string, unknown>;
}

/** 🖼️ Renderer-neutral snapshot slice consumed by `@semio-tech/cad-js/renderer`. */
export interface DisplayModel {
  readonly prompt?: string;
  readonly items: readonly DisplayItem[];
}

/** 🖼️ Instantiates `display.states[state]` templates using current `context`. */
export function resolveDisplay(spec: InteractionSpec, state: string, context: Record<string, unknown>, preview: SpatialPreviewKernel, model?: Model): DisplayModel {
  const env: ExprEnv = { context: model ? withResolvedInteractionPointsContext(model, context) : context, preview };
  const section = spec.display?.states?.find((s) => s.state === state);
  const raw = section?.items ?? [];
  const items: DisplayItem[] = [];
  for (const it of raw) {
    switch (it.kind) {
      case "point":
        items.push({
          kind: "point",
          id: it.id,
          ...(it.role ? { role: it.role } : {}),
          params: { position: evalExpr(it.position, env) },
        });
        break;
      case "label":
        items.push({
          kind: "label",
          id: it.id,
          ...(it.role ? { role: it.role } : {}),
          params: { text: it.text, position: evalExpr(it.position, env) },
        });
        break;
      case "segment":
        items.push({
          kind: "segment",
          id: it.id,
          ...(it.role ? { role: it.role } : {}),
          params: { from: evalExpr(it.from, env), to: evalExpr(it.to, env) },
        });
        break;
      case "linear-handle":
        items.push({
          kind: "linear-handle",
          id: it.id,
          ...(it.role ? { role: it.role } : {}),
          params: { axis: [...it.axis], origin: evalExpr(it.origin, env) },
        });
        break;
      case "box-preview":
        items.push({
          kind: "box-preview",
          id: it.id,
          ...(it.role ? { role: it.role } : {}),
          params: {
            cornerA: evalExpr(it.cornerA, env),
            cornerB: evalExpr(it.cornerB, env),
            height: evalExpr(it.height, env),
          },
        });
        break;
      case "preview": {
        const params: Record<string, unknown> = {};
        for (const [k, v] of Object.entries(it.params ?? {})) params[k] = evalExpr(v, env);
        items.push({
          kind: "preview",
          id: it.id,
          ...(it.role ? { role: it.role } : {}),
          params: { previewKind: it.previewKind, ...params },
        });
        break;
      }
      case "entity-highlight": {
        const idVal = evalExpr(it.entityId, env);
        items.push({
          kind: "entity-highlight",
          id: it.id,
          ...(it.role ? { role: it.role } : {}),
          params: { entity: { kind: it.geometryEntityKind, id: String(idVal ?? "") } },
        });
        break;
      }
      case "curve":
        items.push({ kind: "curve", id: it.id, ...(it.role ? { role: it.role } : {}) });
        break;
      case "mesh":
        items.push({ kind: "mesh", id: it.id, ...(it.role ? { role: it.role } : {}) });
        break;
      default:
        break;
    }
  }
  const entry = interactionLengthEntryForState(spec, state);
  const lock = positiveLengthLock(context);
  const cursorRaw = readInteractionContextVec3(context, CURSOR_RAW_CTX);
  if (entry && lock != null && cursorRaw) {
    const anchor = readInteractionContextVec3(context, entry.anchor);
    if (anchor) {
      items.push({
        kind: "point",
        id: `${state}-length-cursor`,
        role: "cursor",
        params: { position: cursorRaw },
      });
      items.push({
        kind: "segment",
        id: `${state}-length-guide`,
        role: "guide",
        params: { from: anchor, to: cursorRaw },
      });
      const mid: Vec3 = [(anchor[0] + cursorRaw[0]) / 2, (anchor[1] + cursorRaw[1]) / 2, (anchor[2] + cursorRaw[2]) / 2];
      items.push({
        kind: "label",
        id: `${state}-length-label`,
        role: "prompt",
        params: { text: String(lock), position: mid },
      });
    }
  }
  const scalarEntry = interactionScalarEntryForState(spec, state);
  if (scalarEntry?.axisAnchor) {
    const base = scalarEntryAxisBase(context, scalarEntry);
    const axis = scalarEntryAxis(scalarEntry);
    const heightLock = positiveHeightLock(context);
    const fieldVal = context[scalarEntry.field];
    const height = heightLock ?? (typeof fieldVal === "number" && Number.isFinite(fieldVal) && fieldVal > 0 ? fieldVal : null);
    if (base && height != null) {
      const raw = readInteractionContextVec3(context, CURSOR_RAW_CTX);
      const signedT = typeof context[SCALAR_AXIS_T_CTX] === "number" && Number.isFinite(context[SCALAR_AXIS_T_CTX]) ? (context[SCALAR_AXIS_T_CTX] as number) : height;
      const top = preview.scalarTopOnAxis(base, axis, height, signedT);
      items.push({
        kind: "segment",
        id: `${state}-scalar-height`,
        role: "height",
        params: { from: base, to: top },
      });
      if (raw) {
        const projected = heightLock != null ? top : preview.projectPointOnScalarAxis(base, axis, raw).projected;
        items.push({
          kind: "point",
          id: `${state}-scalar-cursor`,
          role: "cursor",
          params: { position: heightLock != null ? raw : projected },
        });
        if (heightLock != null) {
          items.push({
            kind: "segment",
            id: `${state}-scalar-guide`,
            role: "guide",
            params: { from: top, to: raw },
          });
          const mid: Vec3 = [(top[0] + raw[0]) / 2, (top[1] + raw[1]) / 2, (top[2] + raw[2]) / 2];
          items.push({
            kind: "label",
            id: `${state}-scalar-label`,
            role: "prompt",
            params: { text: String(heightLock), position: mid },
          });
        } else {
          items.push({
            kind: "segment",
            id: `${state}-scalar-guide`,
            role: "guide",
            params: { from: projected, to: raw },
          });
        }
      }
    }
  }
  return { items };
}

// #endregion 🖼️Display
// #endregion 📦️🎬️actions
// #region 🧪️Tests
const __actionsTestRuntime = import.meta.vitest ? await import("../🏃️runtime/🟦️.ts") : null;

/** 🎒️ The values this module hands its extracted suite `./🧪️tests/🧪️semio-tech-cad-js-core-box-display-committed/🟦️.ts`. */
export type ActionsTestDependencies = {
  readonly __actionsTestRuntime: typeof __actionsTestRuntime;
  readonly resolveDisplay: typeof resolveDisplay;
};

if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🧪️semio-tech-cad-js-core-box-display-committed/🟦️.ts");
  await registerTests1(import.meta.vitest, { __actionsTestRuntime, resolveDisplay }, { url: import.meta.url });
}
// #endregion 🧪️Tests
