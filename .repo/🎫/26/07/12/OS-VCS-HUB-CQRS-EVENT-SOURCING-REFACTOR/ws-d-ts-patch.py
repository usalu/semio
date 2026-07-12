#!/usr/bin/env python3
"""WS-D: patch framework/core/js/index.ts to the typed ActionResponse contract.
Idempotent, presence-checked replacements to survive concurrent edits."""
import sys

PATH = "framework/core/js/index.ts"
src = open(PATH, encoding="utf-8").read()
orig = src
report = []

def rep(name, old, new, required=True):
    global src
    if old in src:
        if new in src and new != old and old not in src:
            report.append(f"SKIP {name} (already applied)")
            return
        src = src.replace(old, new, 1)
        report.append(f"OK   {name}")
    else:
        report.append(("MISS " if required else "skip ") + name)

ACTION_RESPONSE = '''//#region ActionResponse
/** @emoji 🕰️ Hybrid logical clock stamp carried by every kernel operation. */
export type HybridLogicalTimestamp = { readonly wall: number; readonly counter: number };

/** @emoji 🩹 A schema-tagged document mutation payload (forward diff or inverse diff). */
export type DocumentDiff = { readonly schemaId: string; readonly payload: unknown };

/** @emoji ↩️ Undo semantics for a single kernel operation. */
export type UndoPolicy = "exactBaseOnly" | "transformAgainstConcurrent" | "semanticUndo" | "compensatingAction";

/** @emoji ↩️ The true inverse of a kernel operation, recorded from the store's `Edit.backwards`. */
export type InverseOperation = {
  readonly targetOperation: string;
  readonly inverseDiff: DocumentDiff;
  readonly baseVersion: number;
  readonly dependencies?: readonly string[];
  readonly undoPolicy: UndoPolicy;
};

/** @emoji 🔁 One typed document operation with its true inverse — the CQRS wire unit. */
export type KernelOperation = {
  readonly id: string;
  readonly document: number;
  readonly baseVersion: number;
  readonly actionId: string;
  readonly diff: DocumentDiff;
  readonly inverse: InverseOperation;
  readonly dependencies?: readonly string[];
  readonly author: string;
  readonly timestamp: HybridLogicalTimestamp;
};

/** @emoji 🎁 The undo group binding an action invocation to its operations + inverses. */
export type UndoGroup = {
  readonly actionId: string;
  readonly operations: readonly string[];
  readonly inverseOperations: readonly InverseOperation[];
};

/** @emoji 📣 An out-of-band app event surfaced to the shell (e.g. history changed). */
export type AppEvent = { readonly kind: string; readonly payload: unknown };

/** @emoji 🩺 A diagnostic emitted alongside an action result. */
export type Diagnostic = { readonly level: string; readonly message: string };

/**
 * @emoji 🐚 A typed side effect the shell performs on the app's behalf. Mirrors the Rust
 * `HostEffect` enum (externally tagged: unit variants are the plain tag string, struct variants are
 * a single-key object keyed by the camelCase variant name).
 */
export type HostEffect =
  | "requestSync"
  | { readonly openWindow: { readonly kind: string; readonly params: unknown } }
  | { readonly closeWindow: { readonly window: number } }
  | { readonly notify: { readonly message: string } }
  | { readonly navigate: { readonly uri: string } }
  | { readonly setPanel: { readonly panelJson: string } }
  | { readonly downloadMediaExport: { readonly filename: string; readonly mimeType: string; readonly data: string; readonly encoding?: string } }
  | { readonly iconRenderExport: { readonly items: readonly { readonly filename: string; readonly request: unknown }[] } }
  | { readonly requestFileOpen: { readonly accept: string; readonly readAs?: string; readonly importAction: string } }
  | { readonly spawnPluginInstance: { readonly programId: string; readonly appId: string; readonly osInstanceId?: string; readonly label?: string; readonly documentJson?: string } }
  | { readonly openPluginInstance: { readonly programId: string; readonly appId: string; readonly osInstanceId?: string } };

/**
 * @emoji 📤 Typed result of a plugin `handle-action` call — mirrors the Rust `ActionResult`. Replaces
 * the legacy `string[]` JSON-patch shape: operations are now typed `KernelOperation`s with true
 * inverses, and the shell applies `requestedEffects` through `applyHostEffects` (WS-E).
 */
export type ActionResponse = {
  readonly output: unknown;
  readonly operations: readonly KernelOperation[];
  readonly inverseGroup: UndoGroup;
  readonly diagnostics?: readonly Diagnostic[];
  readonly requestedEffects?: readonly HostEffect[];
  readonly events?: readonly AppEvent[];
};

const EMPTY_ACTION_RESPONSE: ActionResponse = {
  output: null,
  operations: [],
  inverseGroup: { actionId: "", operations: [], inverseOperations: [] },
};

/** @emoji 📥 Parses a raw plugin `handle-action` response string into a typed {@link ActionResponse}. */
export function parseActionResponse(raw: string): ActionResponse {
  try {
    const parsed = JSON.parse(raw) as Partial<ActionResponse> | null;
    if (parsed && typeof parsed === "object" && Array.isArray(parsed.operations)) {
      return parsed as ActionResponse;
    }
  } catch {
    // fall through to the empty response
  }
  return EMPTY_ACTION_RESPONSE;
}
//#endregion ActionResponse'''

OLD_PATCHOPS = '''type KernelOperationPayload = {
  readonly diff?: {
    readonly payload?: unknown;
  };
};

type ActionResultPayload = {
  readonly operations?: readonly KernelOperationPayload[];
};

/** @emoji 🔧 Normalizes plugin action responses into legacy JSON patch op strings. */
export function patchOpsFromActionResponse(raw: string): string[] {
  let parsed: unknown;
  try {
    parsed = JSON.parse(raw);
  } catch {
    return [];
  }
  if (Array.isArray(parsed)) {
    return parsed.map((entry) => (typeof entry === "string" ? entry : JSON.stringify(entry)));
  }
  if (parsed && typeof parsed === "object") {
    const result = parsed as ActionResultPayload;
    if (Array.isArray(result.operations)) {
      return result.operations
        .map((operation) => operation?.diff?.payload)
        .filter((payload): payload is Record<string, unknown> => payload != null && typeof payload === "object")
        .map((payload) => JSON.stringify(payload));
    }
  }
  return [];
}'''

rep("R1 types+patchOps->ActionResponse", OLD_PATCHOPS, ACTION_RESPONSE)

rep("R2 handle type",
    "  readonly handleAction: (instanceId: number, actionJson: string, viewState: PluginViewState) => Promise<string[]>;\n",
    "  readonly handleAction: (instanceId: number, actionJson: string, viewState: PluginViewState) => Promise<ActionResponse>;\n"
    "  readonly applyOperations?: (instanceId: number, operationsJson: string) => Promise<void>;\n"
    "  readonly readAppDocument?: (instanceId: number) => Promise<string>;\n"
    "  readonly loadAppDocument?: (instanceId: number, documentJson: string) => Promise<void>;\n"
    "  readonly attachBackbone?: (instanceId: number, uri: string) => Promise<void>;\n"
    "  readonly detachBackbone?: (instanceId: number) => Promise<void>;\n")

# R3 serialized wrapper: add pass-throughs before dispose
rep("R3 wrapper pass-throughs",
    "    appLabels: (instanceId, viewState) => runSerialized(() => handle.appLabels(instanceId, viewState)),\n    dispose: handle.dispose,",
    "    appLabels: (instanceId, viewState) => runSerialized(() => handle.appLabels(instanceId, viewState)),\n"
    "    applyOperations: handle.applyOperations ? (instanceId, operationsJson) => runSerialized(() => handle.applyOperations!(instanceId, operationsJson)) : undefined,\n"
    "    readAppDocument: handle.readAppDocument ? (instanceId) => runSerialized(() => handle.readAppDocument!(instanceId)) : undefined,\n"
    "    loadAppDocument: handle.loadAppDocument ? (instanceId, documentJson) => runSerialized(() => handle.loadAppDocument!(instanceId, documentJson)) : undefined,\n"
    "    attachBackbone: handle.attachBackbone ? (instanceId, uri) => runSerialized(() => handle.attachBackbone!(instanceId, uri)) : undefined,\n"
    "    detachBackbone: handle.detachBackbone ? (instanceId) => runSerialized(() => handle.detachBackbone!(instanceId)) : undefined,\n"
    "    dispose: handle.dispose,")

# R4 createPluginApi type additions
rep("R4 api type",
    "      appLabels?: (instanceId: number, viewStateJson: string) => Promise<string>;\n    }>;",
    "      appLabels?: (instanceId: number, viewStateJson: string) => Promise<string>;\n"
    "      applyOperations?: (instanceId: number, operationsJson: string) => Promise<void>;\n"
    "      readAppDocument?: (instanceId: number) => Promise<string>;\n"
    "      loadAppDocument?: (instanceId: number, documentJson: string) => Promise<void>;\n"
    "      attachBackbone?: (instanceId: number, uri: string) => Promise<void>;\n"
    "      detachBackbone?: (instanceId: number) => Promise<void>;\n    }>;")

# R5 module surface type additions
rep("R5 module type",
    "    semio_plugin_app_labels?: (instanceId: number, viewStateJson: string) => string;\n  };",
    "    semio_plugin_app_labels?: (instanceId: number, viewStateJson: string) => string;\n"
    "    semio_plugin_apply_operations?: (instanceId: number, operationsJson: string) => void;\n"
    "    semio_plugin_read_app_document?: (instanceId: number) => string;\n"
    "    semio_plugin_load_app_document?: (instanceId: number, documentJson: string) => void;\n"
    "    semio_plugin_attach_backbone?: (instanceId: number, uri: string) => void;\n"
    "    semio_plugin_detach_backbone?: (instanceId: number) => void;\n  };")

# R6 createPluginApi impl handleAction return
rep("R6 api handleAction return",
    "        const raw = await api.handleAction(instanceId, actionJson, JSON.stringify(viewState));\n        return patchOpsFromActionResponse(raw);",
    "        const raw = await api.handleAction(instanceId, actionJson, JSON.stringify(viewState));\n        return parseActionResponse(raw);")

# R7 createPluginApi impl new methods before dispose
rep("R7 api impl methods",
    "        return normalizeAppLabelsOverlay(JSON.parse(await api.appLabels(instanceId, JSON.stringify(viewState))));\n      },\n      dispose() {},",
    "        return normalizeAppLabelsOverlay(JSON.parse(await api.appLabels(instanceId, JSON.stringify(viewState))));\n      },\n"
    "      applyOperations: api.applyOperations ? (instanceId, operationsJson) => api.applyOperations!(instanceId, operationsJson) : undefined,\n"
    "      readAppDocument: api.readAppDocument ? (instanceId) => api.readAppDocument!(instanceId) : undefined,\n"
    "      loadAppDocument: api.loadAppDocument ? (instanceId, documentJson) => api.loadAppDocument!(instanceId, documentJson) : undefined,\n"
    "      attachBackbone: api.attachBackbone ? (instanceId, uri) => api.attachBackbone!(instanceId, uri) : undefined,\n"
    "      detachBackbone: api.detachBackbone ? (instanceId) => api.detachBackbone!(instanceId) : undefined,\n"
    "      dispose() {},")

# R8 module impl handleAction guard + return
rep("R8 module handleAction",
    "      const handle = module.semio_plugin_handle_action;\n      if (!handle) return [];\n      const raw = handle(instanceId, actionJson, JSON.stringify(viewState));\n      return patchOpsFromActionResponse(raw);",
    "      const handle = module.semio_plugin_handle_action;\n      if (!handle) return EMPTY_ACTION_RESPONSE;\n      const raw = handle(instanceId, actionJson, JSON.stringify(viewState));\n      return parseActionResponse(raw);")

# R9 module impl new methods before dispose
rep("R9 module impl methods",
    "      return normalizeAppLabelsOverlay(JSON.parse(labels(instanceId, JSON.stringify(viewState))));\n    },\n    dispose() {},",
    "      return normalizeAppLabelsOverlay(JSON.parse(labels(instanceId, JSON.stringify(viewState))));\n    },\n"
    "    applyOperations: module.semio_plugin_apply_operations ? async (instanceId, operationsJson) => { module.semio_plugin_apply_operations!(instanceId, operationsJson); } : undefined,\n"
    "    readAppDocument: module.semio_plugin_read_app_document ? async (instanceId) => module.semio_plugin_read_app_document!(instanceId) : undefined,\n"
    "    loadAppDocument: module.semio_plugin_load_app_document ? async (instanceId, documentJson) => { module.semio_plugin_load_app_document!(instanceId, documentJson); } : undefined,\n"
    "    attachBackbone: module.semio_plugin_attach_backbone ? async (instanceId, uri) => { module.semio_plugin_attach_backbone!(instanceId, uri); } : undefined,\n"
    "    detachBackbone: module.semio_plugin_detach_backbone ? async (instanceId) => { module.semio_plugin_detach_backbone!(instanceId); } : undefined,\n"
    "    dispose() {},")

if src != orig:
    open(PATH, "w", encoding="utf-8").write(src)

print("\n".join(report))
missing = [r for r in report if r.startswith("MISS")]
sys.exit(1 if missing else 0)
