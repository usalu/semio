import { createHotContext as __vite__createHotContext } from "/@vite/client";import.meta.hot = __vite__createHotContext("/@fs/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx");import __vite__cjsImport0_react_jsxDevRuntime from "/@fs/Users/ueli/Documents/semio/node_modules/.vite-os-dev/cad-react/deps/react_jsx-dev-runtime.js?v=75efac35"; const jsxDEV = __vite__cjsImport0_react_jsxDevRuntime["jsxDEV"];
var _s = $RefreshSig$(), _s2 = $RefreshSig$(), _s3 = $RefreshSig$();
import __vite__cjsImport1_react from "/@fs/Users/ueli/Documents/semio/node_modules/.vite-os-dev/cad-react/deps/react.js?v=75efac35"; const useCallback = __vite__cjsImport1_react["useCallback"]; const useEffect = __vite__cjsImport1_react["useEffect"]; const useMemo = __vite__cjsImport1_react["useMemo"]; const useState = __vite__cjsImport1_react["useState"]




;
import {
  isIconName
} from "/@fs/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/📦️index.ts";
import {
  deriveUtilityNodes,
  effectiveActionArgs,
  FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID,
  FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
  FRAMEWORK_PANEL_TAB_DOCUMENT_ICON_ID,
  FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
  FRAMEWORK_PANEL_TAB_HISTORY_ID,
  FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID,
  FRAMEWORK_PANEL_TAB_INSPECTION_ID,
  FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID,
  FRAMEWORK_PANEL_TAB_PARAMETERS_ID,
  missingRequiredArgs,
  panelTabKindId,
  partitionWindowMeasures,
  pendingPanelUiNode,
  RECORD_TUTORIAL_ACTION_ID,
  resolvePluginHostConfig,
  resolveUiDirtyScope,
  resolveWindowActions,
  SET_ACTIVE_TOOL_ACTION_ID,
  SET_ACTIVE_UTILITY_ACTION_ID,
  SHELL_LOCALES,
  START_INTRODUCTION_ACTION_ID,
  START_TUTORIAL_ACTION_ID
} from "/@fs/Users/ueli/Documents/semio/🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts";
import {
  encodeActionWire,
  packValueFromBase64,
  packValueToBase64
} from "/@fs/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🟦️glue.ts";
import {
  decodeWorldProjectionTemplateId
} from "/@fs/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript/🟦️glue.tsx";
import {
  ANCHORS,
  builtinUiDrivers,
  childElementId,
  ChromeAwareWindowScrollSurface,
  createEvenWindowLayout,
  elementIdSegment,
  Icon,
  IconSelector,
  Input,
  resolveTranslationLabel,
  RibbonDivider,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  setUiLocale,
  singleTreeLeaf,
  Slider,
  staticTreePanelDefinition,
  Toggle,
  ToggleGroup,
  Tree,
  TreeCheckbox,
  UI_RIBBON_PARENT_CATEGORIES,
  UI_TERMINOLOGY_NATIVE,
  uiDataLabel,
  uiI18n,
  useLabel,
  useShellScope,
  WindowMeasuresTree,
  WindowMeasureTreeGroup,
  WindowMeasureTreeLeaf
} from "/@fs/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
import {
  declarativeTreeDragController,
  InterpretedUiNode,
  interpretUiNode,
  renderUiControl,
  uiTreeNodeToTreePanelConfig,
  wireLabel
} from "/@fs/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Interpreter/🟦️component.tsx";
import {
  actionStageKey,
  EMPTY_SHELL_LOCKS,
  ShellFaultBoundary
} from "/@fs/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🟦️component.tsx";
import {
  registerPendingWorldProjection
} from "/@fs/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/World3dHost/🟦️component.tsx";
import { groupUtilityNodesByCategory, UTILITY_CATEGORIES, UtilityTree } from "/@fs/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/UtilityTree/🟦️component.tsx";
import { loadPluginModule } from "/@fs/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/PluginRuntime/🟦️component.tsx";
export function syncDocumentId(session, panel, studioMode) {
  if (studioMode && panel?.activeSpawnedId) {
    const spawned = panel.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId);
    if (spawned) return `${spawned.pluginId}-${spawned.instanceId}`;
  }
  return `${session.pluginId}-${session.instanceId}`;
}
export const DEFAULT_PANEL_WIDTH_PX = 300;
export const FRAMEWORK_CATEGORY_DISPLAY_ID = "framework.category.display";
export const FRAMEWORK_CATEGORY_COMMAND_ID = "framework.category.command";
export const FRAMEWORK_CATEGORY_TOOL_ID = "framework.category.tool";
export const PANEL_TAB_BAR_HOSTS = {
  "top-left": "navbar",
  "top-middle": "navbar",
  "top-right": "navbar",
  "bottom-left": "footer",
  "bottom-middle": "footer",
  "bottom-right": "footer"
};
const APP_DOCUMENT_SEPARATOR = " · ";
export const NOTE_WORLD_NAVIGATION_ACTION_ID = "noteWorldNavigation";
const NOTE_SHELL_COMMAND_ACTION_ID = "noteShellCommand";
export const FRAMEWORK_RESERVED_ACTION_IDS = /* @__PURE__ */ new Set(
  [
    "undo",
    "redo",
    "commitCheckpoint",
    "createAlternative",
    "switchAlternative",
    "checkoutCheckpoint",
    "copy",
    "cut",
    "paste",
    "revertToCommand",
    "setHistoryCommandFilter",
    NOTE_SHELL_COMMAND_ACTION_ID,
    "recordTutorial",
    "startIntroduction",
    "startTutorial",
    "setActiveUtility",
    "setActiveTool",
    "suggestionsTick",
    "fillBuildTick"
  ]
);
export function buildNoteShellCommandAction(controllerId, commandId, label, detail) {
  return { controllerId, action: NOTE_SHELL_COMMAND_ACTION_ID, args: { commandId, label, ...detail ? { detail } : {} } };
}
export const TUTORIAL_RECORDING_EXCLUDED_ACTION_IDS = /* @__PURE__ */ new Set([NOTE_WORLD_NAVIGATION_ACTION_ID, NOTE_SHELL_COMMAND_ACTION_ID, START_INTRODUCTION_ACTION_ID, START_TUTORIAL_ACTION_ID, RECORD_TUTORIAL_ACTION_ID]);
export const PRESENCE_CLIENT_STORAGE_KEY = "semio.presence.client";
export const PRESENCE_HEARTBEAT_INTERVAL_MS = 5e3;
function presenceIdentityPackBase64(identity) {
  return packValueToBase64(identity);
}
function presenceIdentityFromPackBase64(encoded) {
  try {
    const decoded = packValueFromBase64(encoded);
    if (decoded.clientId && decoded.name) return { clientId: decoded.clientId, name: decoded.name };
  } catch {
    return null;
  }
  return null;
}
export function presenceClientIdentity(ephemeral = false) {
  if (typeof window === "undefined") return { clientId: "server", name: "Server" };
  if (!ephemeral) {
    const stored = window.sessionStorage.getItem(PRESENCE_CLIENT_STORAGE_KEY);
    if (stored) {
      const parsed = presenceIdentityFromPackBase64(stored);
      if (parsed) return parsed;
    }
  }
  const clientId = `client-${Math.random().toString(36).slice(2, 10)}`;
  const identity = { clientId, name: `Guest ${clientId.slice(-4).toUpperCase()}` };
  if (!ephemeral) window.sessionStorage.setItem(PRESENCE_CLIENT_STORAGE_KEY, presenceIdentityPackBase64(identity));
  return identity;
}
function readBrowserUri() {
  if (typeof window === "undefined") return "/";
  return `${window.location.pathname}${window.location.search}` || "/";
}
export function useUIHistory(initialUri = "/", syncBrowser = false) {
  _s();
  const [history, setHistory] = useState(() => ({
    entries: [{ uri: syncBrowser ? readBrowserUri() : initialUri }],
    index: 0
  }));
  const uri = history.entries[history.index]?.uri ?? initialUri;
  const canGoBack = history.index > 0;
  const canGoForward = history.index < history.entries.length - 1;
  const segments = uri.split("/").filter(Boolean);
  const canGoUp = segments.length > 0;
  const parentUri = canGoUp ? `/${segments.slice(0, -1).join("/")}` : null;
  const goBack = useCallback(() => {
    setHistory((prev) => prev.index > 0 ? { ...prev, index: prev.index - 1 } : prev);
  }, []);
  const goForward = useCallback(() => {
    setHistory((prev) => prev.index < prev.entries.length - 1 ? { ...prev, index: prev.index + 1 } : prev);
  }, []);
  const goUp = useCallback(() => {
    if (!canGoUp || parentUri === null) return;
    setHistory((prev) => {
      const newEntries = prev.entries.slice(0, prev.index + 1);
      return { entries: [...newEntries, { uri: parentUri }], index: newEntries.length };
    });
  }, [canGoUp, parentUri]);
  const navigate = useCallback((targetUri) => {
    setHistory((prev) => {
      const existingIndex = prev.entries.findIndex((entry) => entry.uri === targetUri);
      if (existingIndex >= 0) return { ...prev, index: existingIndex };
      const newEntries = prev.entries.slice(0, prev.index + 1);
      return { entries: [...newEntries, { uri: targetUri }], index: newEntries.length };
    });
  }, []);
  const syncUri = useCallback((targetUri) => {
    setHistory((prev) => {
      const existingIndex = prev.entries.findIndex((entry) => entry.uri === targetUri);
      if (existingIndex >= 0) return { ...prev, index: existingIndex };
      const newEntries = prev.entries.slice(0, prev.index + 1);
      return { entries: [...newEntries, { uri: targetUri }], index: newEntries.length };
    });
  }, []);
  useEffect(() => {
    if (!syncBrowser || typeof window === "undefined") return;
    const current = `${window.location.pathname}${window.location.search}`;
    if (current !== uri) window.history.pushState(null, "", uri);
  }, [syncBrowser, uri]);
  useEffect(() => {
    if (!syncBrowser || typeof window === "undefined") return;
    const onPopState = () => syncUri(readBrowserUri());
    window.addEventListener("popstate", onPopState);
    return () => window.removeEventListener("popstate", onPopState);
  }, [syncBrowser, syncUri]);
  return { uri, canGoBack, canGoForward, canGoUp, parentUri, goBack, goForward, goUp, navigate, syncUri };
}
_s(useUIHistory, "A7p8GNOxgJOQxTp0xynfwIRb3t8=");
export function downloadMediaExport(filename, mimeType, data, encoding) {
  if (typeof document === "undefined") return;
  const payload = encoding === "base64" ? Uint8Array.from(atob(data), (char) => char.charCodeAt(0)) : data;
  const blob = new Blob([payload], { type: mimeType });
  const url = URL.createObjectURL(blob);
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = filename;
  anchor.click();
  URL.revokeObjectURL(url);
}
export function downloadDataUrl(filename, dataUrl) {
  if (typeof document === "undefined") return;
  const anchor = document.createElement("a");
  anchor.href = dataUrl;
  anchor.download = filename;
  anchor.click();
}
export function requestFileOpen(accept, readAs, multiple) {
  if (typeof document === "undefined") return Promise.resolve([]);
  return new Promise((resolve) => {
    const input = document.createElement("input");
    input.type = "file";
    input.accept = accept;
    if (multiple) input.multiple = true;
    input.onchange = async () => {
      const files = input.files ? Array.from(input.files) : [];
      if (files.length === 0) {
        resolve([]);
        return;
      }
      const opened = [];
      for (const file of files) {
        if (readAs === "dataUrl") {
          const contents = await new Promise((resolveFile) => {
            const reader = new FileReader();
            reader.onload = () => resolveFile(typeof reader.result === "string" ? reader.result : null);
            reader.onerror = () => resolveFile(null);
            reader.readAsDataURL(file);
          });
          if (contents !== null) opened.push({ contents, name: file.name });
          continue;
        }
        opened.push({ contents: await file.text(), name: file.name });
      }
      resolve(opened);
    };
    input.click();
  });
}
export function makeEffectDispatchOne(pluginEntry, baseSession, applyEffects) {
  return async (action, args) => {
    const response = await pluginEntry.handle.handleAction(
      baseSession.instanceId,
      encodeActionWire({ controllerId: baseSession.app.controllerId, action, args }),
      baseSession.viewState
    );
    await applyEffects(response.requestedEffects ?? [], baseSession, resolveUiDirtyScope(response.uiScope));
  };
}
export async function dispatchOpenedFiles(opened, importAction, multiple, dispatchOne) {
  const total = opened.length;
  for (let index = 0; index < opened.length; index += 1) {
    const file = opened[index];
    await dispatchOne(importAction, multiple ? { payload: file.contents, name: file.name, index, total } : { payload: file.contents, name: file.name });
  }
}
export function scheduleDispatchAction(action, args, delayMs, dispatchOne, schedule = (fn, ms) => setTimeout(fn, ms)) {
  schedule(() => {
    void dispatchOne(action, args);
  }, delayMs);
}
function walkBmffBoxes(view, start, end) {
  const boxes = [];
  let offset = start;
  while (offset + 8 <= end) {
    const size32 = view.getUint32(offset);
    const type = String.fromCharCode(view.getUint8(offset + 4), view.getUint8(offset + 5), view.getUint8(offset + 6), view.getUint8(offset + 7));
    let headerSize = 8;
    let boxSize = size32;
    if (size32 === 1) {
      if (offset + 16 > end) break;
      boxSize = Number(view.getBigUint64(offset + 8));
      headerSize = 16;
    } else if (size32 === 0) {
      boxSize = end - offset;
    }
    if (boxSize < headerSize || offset + boxSize > end) break;
    boxes.push({ type, start: offset + headerSize, end: offset + boxSize });
    offset += boxSize;
  }
  return boxes;
}
function findBmffBox(boxes, type) {
  return boxes.find((box) => box.type === type);
}
function probeMp4VideoTrack(bytes) {
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  const moov = findBmffBox(walkBmffBoxes(view, 0, bytes.byteLength), "moov");
  if (!moov) return null;
  for (const trak of walkBmffBoxes(view, moov.start, moov.end).filter((box) => box.type === "trak")) {
    const mdia = findBmffBox(walkBmffBoxes(view, trak.start, trak.end), "mdia");
    if (!mdia) continue;
    const mdiaBoxes = walkBmffBoxes(view, mdia.start, mdia.end);
    const hdlr = findBmffBox(mdiaBoxes, "hdlr");
    if (!hdlr || hdlr.end - hdlr.start < 12) continue;
    const handlerType = String.fromCharCode(view.getUint8(hdlr.start + 8), view.getUint8(hdlr.start + 9), view.getUint8(hdlr.start + 10), view.getUint8(hdlr.start + 11));
    if (handlerType !== "vide") continue;
    const mdhd = findBmffBox(mdiaBoxes, "mdhd");
    const minf = findBmffBox(mdiaBoxes, "minf");
    if (!mdhd || !minf) continue;
    const timescale = view.getUint8(mdhd.start) === 1 ? view.getUint32(mdhd.start + 20) : view.getUint32(mdhd.start + 12);
    if (timescale <= 0) continue;
    const stbl = findBmffBox(walkBmffBoxes(view, minf.start, minf.end), "stbl");
    if (!stbl) continue;
    const track = probeSampleTable(view, walkBmffBoxes(view, stbl.start, stbl.end), timescale);
    if (track) return track;
  }
  return null;
}
function parseStsd(view, stsd) {
  if (view.getUint32(stsd.start + 4) < 1) return null;
  const entryOffset = stsd.start + 8;
  const entrySize = view.getUint32(entryOffset);
  const format = String.fromCharCode(
    view.getUint8(entryOffset + 4),
    view.getUint8(entryOffset + 5),
    view.getUint8(entryOffset + 6),
    view.getUint8(entryOffset + 7)
  );
  if (format !== "avc1" && format !== "hvc1" && format !== "hev1") return null;
  const codec = format === "avc1" ? "avc1" : "hvc1";
  const visualEntryStart = entryOffset + 8;
  const width = view.getUint16(visualEntryStart + 24);
  const height = view.getUint16(visualEntryStart + 26);
  const inner = walkBmffBoxes(view, visualEntryStart + 78, entryOffset + entrySize);
  const config = findBmffBox(inner, codec === "avc1" ? "avcC" : "hvcC");
  if (!config) return null;
  return { width, height, codec, description: new Uint8Array(view.buffer.slice(config.start, config.end)) };
}
function parseStsz(view, box) {
  const uniformSize = view.getUint32(box.start + 4);
  const sampleCount = view.getUint32(box.start + 8);
  if (uniformSize !== 0) return new Array(sampleCount).fill(uniformSize);
  const sizes = [];
  for (let i = 0; i < sampleCount; i += 1) sizes.push(view.getUint32(box.start + 12 + i * 4));
  return sizes;
}
function parseChunkOffsets(view, box, is64) {
  const count = view.getUint32(box.start + 4);
  const offsets = [];
  for (let i = 0; i < count; i += 1) {
    offsets.push(is64 ? Number(view.getBigUint64(box.start + 8 + i * 8)) : view.getUint32(box.start + 8 + i * 4));
  }
  return offsets;
}
function parseChunkOfSample(view, box, sampleCount, chunkCount) {
  const entryCount = view.getUint32(box.start + 4);
  const entries = [];
  for (let i = 0; i < entryCount; i += 1) {
    entries.push({ firstChunk: view.getUint32(box.start + 8 + i * 12), samplesPerChunk: view.getUint32(box.start + 12 + i * 12) });
  }
  const chunkOfSample = [];
  for (let entryIndex = 0; entryIndex < entries.length; entryIndex += 1) {
    const entry = entries[entryIndex];
    const nextFirstChunk = entries[entryIndex + 1]?.firstChunk ?? chunkCount + 1;
    for (let chunk = entry.firstChunk; chunk < nextFirstChunk; chunk += 1) {
      for (let inChunk = 0; inChunk < entry.samplesPerChunk; inChunk += 1) chunkOfSample.push(chunk);
    }
  }
  return chunkOfSample.length >= sampleCount ? chunkOfSample : null;
}
function computeSampleOffsets(chunkOfSample, chunkOffsets, sizes) {
  const offsets = [];
  const cursorByChunk = /* @__PURE__ */ new Map();
  for (let i = 0; i < sizes.length; i += 1) {
    const chunk = chunkOfSample[i];
    const base = cursorByChunk.get(chunk) ?? chunkOffsets[chunk - 1] ?? 0;
    offsets.push(base);
    cursorByChunk.set(chunk, base + sizes[i]);
  }
  return offsets;
}
function accumulateTimestampsMs(view, stts, sampleCount, timescale) {
  const entryCount = view.getUint32(stts.start + 4);
  const timestamps = [];
  let ticks = 0;
  for (let entryIndex = 0; entryIndex < entryCount && timestamps.length < sampleCount; entryIndex += 1) {
    const count = view.getUint32(stts.start + 8 + entryIndex * 8);
    const delta = view.getUint32(stts.start + 12 + entryIndex * 8);
    for (let i = 0; i < count && timestamps.length < sampleCount; i += 1) {
      timestamps.push(ticks / timescale * 1e3);
      ticks += delta;
    }
  }
  return timestamps;
}
function parseSyncSamples(view, box) {
  const count = view.getUint32(box.start + 4);
  const sync = /* @__PURE__ */ new Set();
  for (let i = 0; i < count; i += 1) sync.add(view.getUint32(box.start + 8 + i * 4));
  return sync;
}
function probeSampleTable(view, stblBoxes, timescale) {
  const stsd = findBmffBox(stblBoxes, "stsd");
  const stts = findBmffBox(stblBoxes, "stts");
  const stsc = findBmffBox(stblBoxes, "stsc");
  const stsz = findBmffBox(stblBoxes, "stsz");
  const stco = findBmffBox(stblBoxes, "stco") ?? findBmffBox(stblBoxes, "co64");
  if (!stsd || !stts || !stsc || !stsz || !stco) return null;
  const entry = parseStsd(view, stsd);
  if (!entry) return null;
  const sizes = parseStsz(view, stsz);
  const offsets = parseChunkOffsets(view, stco, stco.type === "co64");
  const chunkOfSample = parseChunkOfSample(view, stsc, sizes.length, offsets.length);
  if (!chunkOfSample) return null;
  const sampleOffsets = computeSampleOffsets(chunkOfSample, offsets, sizes);
  const timestampsMs = accumulateTimestampsMs(view, stts, sizes.length, timescale);
  const stss = findBmffBox(stblBoxes, "stss");
  const syncSamples = stss ? parseSyncSamples(view, stss) : null;
  const samples = sizes.map((size, index) => ({
    offset: sampleOffsets[index],
    size,
    timestampMs: timestampsMs[index] ?? 0,
    isSync: syncSamples ? syncSamples.has(index + 1) : true
  }));
  return { width: entry.width, height: entry.height, codec: entry.codec, description: entry.description, samples };
}
function webCodecsAvailable() {
  const scope = window;
  return typeof scope.VideoDecoder === "function" && typeof scope.EncodedVideoChunk === "function";
}
function avcCodecString(description) {
  const hex = (byte) => (byte ?? 0).toString(16).padStart(2, "0");
  return `avc1.${hex(description[1])}${hex(description[2])}${hex(description[3])}`;
}
function jpegDataUrlFromFrame(frame) {
  const canvas = document.createElement("canvas");
  canvas.width = frame.codedWidth;
  canvas.height = frame.codedHeight;
  canvas.getContext("2d")?.drawImage(frame, 0, 0);
  return { dataUrl: canvas.toDataURL("image/jpeg", 0.9), width: frame.codedWidth, height: frame.codedHeight };
}
async function decodeOneMp4Frame(track, bytes, targetIndex) {
  const scope = window;
  let syncIndex = targetIndex;
  while (syncIndex > 0 && !track.samples[syncIndex].isSync) syncIndex -= 1;
  let captured = null;
  await new Promise((resolve, reject) => {
    const decoder = new scope.VideoDecoder({
      output: (frame) => {
        captured = jpegDataUrlFromFrame(frame);
        frame.close();
      },
      error: reject
    });
    decoder.configure({ codec: avcCodecString(track.description), codedWidth: track.width, codedHeight: track.height, description: track.description });
    for (let i = syncIndex; i <= targetIndex; i += 1) {
      const sample = track.samples[i];
      decoder.decode(
        new scope.EncodedVideoChunk({ type: sample.isSync ? "key" : "delta", timestamp: sample.timestampMs * 1e3, data: bytes.subarray(sample.offset, sample.offset + sample.size) })
      );
    }
    decoder.flush().then(() => {
      decoder.close();
      resolve();
    }, reject);
  });
  return captured;
}
async function runTier1VideoFrames(bytes, effect, name, dispatchOne) {
  const track = probeMp4VideoTrack(bytes);
  if (!track || track.samples.length === 0) return false;
  const durationMs = track.samples[track.samples.length - 1].timestampMs;
  const timestamps = sampleMediaFrameTimestampsMs(durationMs, effect.sampleStride, effect.maxFrames, effect.fpsHint);
  let sampledCount = 0;
  for (let index = 0; index < timestamps.length; index += 1) {
    const targetMs = timestamps[index];
    let targetSampleIndex = 0;
    for (let i = 0; i < track.samples.length; i += 1) if (track.samples[i].timestampMs <= targetMs) targetSampleIndex = i;
    const frame = await decodeOneMp4Frame(track, bytes, targetSampleIndex);
    if (!frame) continue;
    sampledCount += 1;
    await dispatchOne(effect.frameAction, {
      payload: frame.dataUrl,
      name,
      frameIndex: index,
      timestampMs: targetMs,
      index,
      total: timestamps.length,
      width: frame.width,
      height: frame.height,
      ...effect.args
    });
  }
  await dispatchOne(effect.doneAction, {
    name,
    durationMs,
    frameCount: track.samples.length,
    sampledCount,
    width: track.width,
    height: track.height,
    codec: track.codec,
    ...effect.args
  });
  return true;
}
export function sampleMediaFrameTimestampsMs(durationMs, sampleStride, maxFrames, fpsHint) {
  const stride = sampleStride > 0 ? sampleStride : 1;
  const fps = fpsHint > 0 ? fpsHint : 30;
  const stepMs = stride / fps * 1e3;
  const timestamps = [];
  if (durationMs <= 0 || stepMs <= 0) return timestamps;
  for (let k = 0; ; k += 1) {
    if (maxFrames > 0 && timestamps.length >= maxFrames) break;
    const ts = k * stepMs;
    if (ts >= durationMs) break;
    timestamps.push(ts);
  }
  return timestamps;
}
function captureCanvasFrame(video, maxLongEdgePx) {
  const sourceWidth = video.videoWidth || 0;
  const sourceHeight = video.videoHeight || 0;
  const scale = maxLongEdgePx > 0 ? Math.min(1, maxLongEdgePx / Math.max(sourceWidth, sourceHeight, 1)) : 1;
  const width = Math.max(1, Math.round(sourceWidth * scale));
  const height = Math.max(1, Math.round(sourceHeight * scale));
  const canvas = document.createElement("canvas");
  canvas.width = width;
  canvas.height = height;
  canvas.getContext("2d")?.drawImage(video, 0, 0, width, height);
  return { dataUrl: canvas.toDataURL("image/jpeg", 0.9), width, height };
}
function waitForVideoEvent(video, type) {
  return new Promise((resolve) => {
    const handler = () => {
      video.removeEventListener(type, handler);
      resolve();
    };
    video.addEventListener(type, handler);
  });
}
export async function runTier2VideoFrames(video, effect, name, dispatchOne) {
  if (video.readyState < 1) await waitForVideoEvent(video, "loadedmetadata");
  const durationMs = Number.isFinite(video.duration) ? video.duration * 1e3 : 0;
  const width = video.videoWidth || 0;
  const height = video.videoHeight || 0;
  const timestamps = sampleMediaFrameTimestampsMs(durationMs, effect.sampleStride, effect.maxFrames, effect.fpsHint);
  const total = timestamps.length;
  for (let index = 0; index < total; index += 1) {
    const timestampMs = timestamps[index];
    video.currentTime = timestampMs / 1e3;
    await waitForVideoEvent(video, "seeked");
    const frame = captureCanvasFrame(video, effect.maxLongEdgePx);
    await dispatchOne(effect.frameAction, {
      payload: frame.dataUrl,
      name,
      frameIndex: index,
      timestampMs,
      index,
      total,
      width: frame.width,
      height: frame.height,
      ...effect.args
    });
  }
  await dispatchOne(effect.doneAction, { name, durationMs, frameCount: total, sampledCount: total, width, height, codec: "unknown", ...effect.args });
}
function bytesFromDataUrl(dataUrl) {
  const binary = atob(dataUrl.slice(dataUrl.indexOf(",") + 1));
  const bytes = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i += 1) bytes[i] = binary.charCodeAt(i);
  return bytes;
}
function bytesToDataUrl(bytes, mime) {
  let binary = "";
  for (let i = 0; i < bytes.length; i += 1) binary += String.fromCharCode(bytes[i]);
  return `data:${mime};base64,${btoa(binary)}`;
}
export async function runRequestMediaFrames(effect, accept, payload, dispatchOne, createVideoElement = () => document.createElement("video")) {
  let bytes;
  let name = "video";
  if (payload) {
    bytes = bytesFromDataUrl(payload);
  } else {
    const opened = await requestFileOpen(accept || "video/*", "dataUrl", false);
    if (opened.length === 0) return;
    bytes = bytesFromDataUrl(opened[0].contents);
    name = opened[0].name;
  }
  try {
    if (webCodecsAvailable() && await runTier1VideoFrames(bytes, effect, name, dispatchOne)) return;
    const url = URL.createObjectURL(new Blob([bytes], { type: "video/mp4" }));
    const video = createVideoElement();
    video.muted = true;
    video.playsInline = true;
    video.src = url;
    try {
      await runTier2VideoFrames(video, effect, name, dispatchOne);
    } finally {
      URL.revokeObjectURL(url);
    }
  } catch (error) {
    console.error("[os-shell] requestMediaFrames: decode failed, falling back to raw bytes", error);
    await dispatchOne(effect.fallbackAction, { payload: bytesToDataUrl(bytes, "video/mp4"), name, ...effect.args });
  }
}
function isStudioMode(pluginFilter) {
  return pluginFilter !== void 0 && resolvePluginHostConfig(pluginFilter) !== void 0;
}
export function parseShellRoute(path) {
  const normalized = (path.split("?")[0] ?? "/").trim() || "/";
  if (normalized === "/") return { kind: "landing" };
  const match = /^\/spaces\/([^/]+)(?:\/instances\/([^/]+))?$/.exec(normalized);
  if (match) return { kind: "space", spaceId: match[1], instanceId: match[2] };
  return { kind: "notFound", path: normalized };
}
export function parseSpaceShellPath(path) {
  const route = parseShellRoute(path);
  if (route.kind !== "space") return null;
  return { spaceId: route.spaceId, instanceId: route.instanceId };
}
export function appDocumentLabel(document2) {
  return document2.join(APP_DOCUMENT_SEPARATOR);
}
export function resolveAppDocument(app, terminology) {
  return app.terminologyDocuments?.[terminology] ?? app.document;
}
export function resolveDocumentByAppId(loadedPlugins, appId, document2, terminology) {
  for (const program of loadedPlugins) {
    const app = program.manifest.apps.find((candidate) => candidate.id === appId);
    if (app) return resolveAppDocument(app, terminology);
  }
  return document2;
}
export function appWindowDocumentLabel(app, terminology, windowLabel, locale = SHELL_LOCALES[0]) {
  const trimmed = windowLabel.trim();
  if (trimmed) return trimmed;
  const override = app.terminologyDocuments?.[terminology];
  return override?.[override.length - 1]?.trim() || resolveManifestLabel(app.label, terminology, locale).trim();
}
export function buildSpacePanelState(programs, spawnedApps, activePanelTab = "s-play-catalogue", activeSpawnedId) {
  return { activePanelTab, programs, spawnedApps, activeSpawnedId };
}
export function panelJsonFromState(state) {
  return packValueToBase64(state);
}
export function parsePanelState(viewState) {
  if (!viewState.panelJson) return null;
  try {
    return packValueFromBase64(viewState.panelJson);
  } catch {
    return null;
  }
}
export function studioPanelFocusingSpawned(panel, spawned) {
  const spawnedApps = panel.spawnedApps.some((entry) => entry.id === spawned.id) ? panel.spawnedApps.map((entry) => entry.id === spawned.id ? spawned : entry) : [...panel.spawnedApps, spawned];
  return buildSpacePanelState(panel.programs, spawnedApps, panel.activePanelTab, spawned.id);
}
export function viewStateWithSpacePanel(viewState, panel) {
  return { ...viewState, panelJson: panelJsonFromState(panel) };
}
export function panelAnchorForGroup(group) {
  if (group === "workbench" || group === "document") return "top-left";
  if (group === "details") return "top-right";
  if (group === "display") return "bottom-left";
  if (group === "settings") return "bottom-right";
  return "top-right";
}
function collectFrameworkLayoutWindowSeeds(node, parentSize = 100) {
  if (node.kind === "window") {
    return [
      {
        windowId: node.instanceId ?? node.windowKindId,
        windowKindId: node.windowKindId,
        title: node.title,
        templateId: node.templateId,
        size: parentSize
      }
    ];
  }
  if (node.kind === "stack") {
    const size = node.size ?? parentSize;
    return node.children.map((child) => ({
      windowId: child.instanceId ?? child.windowKindId,
      windowKindId: child.windowKindId,
      title: child.title,
      templateId: child.templateId,
      size
    }));
  }
  const childSizes = node.children.map((child) => "size" in child ? child.size : void 0);
  const explicitTotal = childSizes.reduce((sum, size) => sum + (size ?? 0), 0);
  const unsetCount = childSizes.filter((size) => size === void 0).length;
  const defaultEach = unsetCount > 0 ? Math.max(0, 100 - explicitTotal) / unsetCount : 0;
  return node.children.flatMap((child, index) => {
    const fraction = childSizes[index] ?? defaultEach;
    return collectFrameworkLayoutWindowSeeds(child, parentSize * (fraction / 100));
  });
}
function resolveFrameworkWindowTitle(windowKindId, instanceId, bakedTitle, windowKinds, terminology, locale) {
  if (instanceId) return bakedTitle ?? windowKindId;
  const kind = windowKinds.find((entry) => entry.id === windowKindId);
  return kind ? resolveManifestLabel(kind.label, terminology, locale) : bakedTitle ?? windowKindId;
}
function convertFrameworkLayoutNodeToModeLayout(node, appLabelsOverlay, windowKinds, terminology, locale) {
  if (node.kind === "window") {
    const id = node.instanceId ?? node.windowKindId;
    const title = resolveFrameworkWindowTitle(node.windowKindId, node.instanceId, node.title, windowKinds, terminology, locale);
    return { kind: "window", id, title: wireLabel(resolveAppLabel(appLabelsOverlay, "windowKind", id, title)) };
  }
  if (node.kind === "stack") {
    return {
      kind: "stack",
      size: node.size,
      children: node.children.map((child) => {
        const id = child.instanceId ?? child.windowKindId;
        const title = resolveFrameworkWindowTitle(child.windowKindId, child.instanceId, child.title, windowKinds, terminology, locale);
        return {
          kind: "window",
          id,
          title: wireLabel(resolveAppLabel(appLabelsOverlay, "windowKind", id, title))
        };
      })
    };
  }
  return {
    kind: node.kind,
    size: node.size,
    children: node.children.map((child) => convertFrameworkLayoutNodeToModeLayout(child, appLabelsOverlay, windowKinds, terminology, locale))
  };
}
export function retitleWindowLayoutNode(node, windowKinds, extraInstances, terminology, locale) {
  if (node.kind === "window") {
    const extra = extraInstances.find((entry) => entry.id === node.id);
    const windowKindId = extra ? extra.windowKindId : node.id;
    const kind = windowKinds.find((entry) => entry.id === windowKindId);
    const title = kind ? wireLabel(resolveManifestLabel(kind.label, terminology, locale)) : node.title ?? uiDataLabel(node.id);
    return { ...node, title };
  }
  return {
    ...node,
    children: node.children.map((child) => retitleWindowLayoutNode(child, windowKinds, extraInstances, terminology, locale))
  };
}
export function resolveFrameworkLayoutSeed(layout, windowKinds, appLabelsOverlay, terminology, locale) {
  const windowIds = windowKinds.map((kind) => kind.id);
  if (!layout?.root) {
    return {
      modeLayout: createEvenWindowLayout(windowIds.length ? windowIds : ["main"]),
      extraInstances: [],
      pendingProjections: []
    };
  }
  const seeds = collectFrameworkLayoutWindowSeeds(layout.root);
  const kindById = new Map(windowKinds.map((kind) => [kind.id, kind]));
  const extraInstances = [];
  const pendingProjections = [];
  for (const seed of seeds) {
    const kind = kindById.get(seed.windowKindId);
    if (!kind) continue;
    if (seed.windowId !== seed.windowKindId) {
      extraInstances.push({
        id: seed.windowId,
        windowKindId: seed.windowKindId,
        title: resolveAppLabel(appLabelsOverlay, "windowKind", seed.windowId, seed.title ?? resolveManifestLabel(kind.label, terminology, locale))
      });
    }
    if (seed.templateId) pendingProjections.push({ windowId: seed.windowId, templateId: seed.templateId });
  }
  return {
    modeLayout: convertFrameworkLayoutNodeToModeLayout(layout.root, appLabelsOverlay, windowKinds, terminology, locale),
    extraInstances,
    pendingProjections
  };
}
export function applyFrameworkLayoutSeed(layout, windowKinds, appLabelsOverlay, terminology, locale) {
  const seed = resolveFrameworkLayoutSeed(layout, windowKinds, appLabelsOverlay, terminology, locale);
  for (const pending of seed.pendingProjections) {
    const projectionSpec = decodeWorldProjectionTemplateId(pending.templateId);
    if (projectionSpec) registerPendingWorldProjection(pending.windowId, projectionSpec);
  }
  return { modeLayout: seed.modeLayout, extraInstances: seed.extraInstances };
}
function modeLayoutNodeToFramework(node, kindByInstanceId) {
  if (node.kind === "window") {
    const windowKindId = kindByInstanceId.get(node.id) ?? node.id;
    const instanceId = kindByInstanceId.has(node.id) ? node.id : void 0;
    return {
      kind: "window",
      windowKindId,
      ...node.title ? { title: node.title } : {},
      ...instanceId ? { instanceId } : {}
    };
  }
  if (node.kind === "stack") {
    return {
      kind: "stack",
      ...node.size !== void 0 ? { size: node.size } : {},
      children: node.children.map((child) => {
        const windowKindId = kindByInstanceId.get(child.id) ?? child.id;
        const instanceId = kindByInstanceId.has(child.id) ? child.id : void 0;
        return {
          kind: "window",
          windowKindId,
          ...child.title ? { title: child.title } : {},
          ...instanceId ? { instanceId } : {}
        };
      })
    };
  }
  return {
    kind: node.kind,
    ...node.size !== void 0 ? { size: node.size } : {},
    children: node.children.map((child) => modeLayoutNodeToFramework(child, kindByInstanceId))
  };
}
export function captureCurrentFrameworkLayout(shellLayout, extraWindowInstances, fallback) {
  if (!shellLayout) return fallback;
  const kindByInstanceId = new Map(extraWindowInstances.map((entry) => [entry.id, entry.windowKindId]));
  const root = modeLayoutNodeToFramework(shellLayout, kindByInstanceId);
  if (root.kind === "window") return { root: { kind: "stack", children: [root] } };
  return { root };
}
export const LAYOUT_CHANGE_SETTLE_MS = 350;
function windowLayoutSkeleton(node) {
  if (node.kind === "window") return { kind: node.kind, id: node.id };
  return { kind: node.kind, children: node.children.map((child) => windowLayoutSkeleton(child)) };
}
function windowLayoutSizedSkeleton(node) {
  if (node.kind === "window") return { kind: node.kind, id: node.id, size: node.size };
  return { kind: node.kind, size: node.size, children: node.children.map((child) => windowLayoutSizedSkeleton(child)) };
}
export function classifyWindowLayoutChange(previous, next) {
  if (previous === next) return null;
  if (!previous || !next) return "rearrange";
  if (JSON.stringify(windowLayoutSkeleton(previous)) !== JSON.stringify(windowLayoutSkeleton(next))) return "rearrange";
  if (JSON.stringify(windowLayoutSizedSkeleton(previous)) !== JSON.stringify(windowLayoutSizedSkeleton(next))) return "resize";
  return null;
}
function windowEngagementControlToSpec(control, onAction) {
  if (!control) return void 0;
  if (control.kind === "ring" || control.kind === "toggleGroup") {
    return {
      kind: control.kind,
      id: control.id,
      label: control.label,
      value: control.value,
      disabled: control.disabled,
      options: control.options.map((row) => ({ id: row.id, label: row.label, disabled: row.disabled })),
      onSelect: control.onSelect ? (id) => onAction({ ...control.onSelect, args: { ...control.onSelect.args, id } }) : void 0
    };
  }
  if (control.kind === "select") {
    return {
      kind: "select",
      id: control.id,
      label: control.label,
      value: control.value,
      placeholder: control.placeholder,
      disabled: control.disabled,
      items: control.items.map((row) => ({ id: row.id, value: row.value, label: row.label })),
      onChange: control.onChange ? (value) => onAction({ ...control.onChange, args: { ...control.onChange.args, value } }) : void 0
    };
  }
  const dispatchNumeric = (action, value) => {
    if (!action) return;
    onAction({ ...action, args: { ...action.args, value } });
  };
  return {
    kind: control.kind,
    id: control.id,
    label: control.label,
    value: control.value,
    min: control.min,
    max: control.max,
    step: control.step,
    unit: control.unit,
    disabled: control.disabled,
    onChange: control.onChange ? (value) => dispatchNumeric(control.onChange, value) : void 0,
    onCommit: control.onCommit ? (value) => dispatchNumeric(control.onCommit, value) : void 0
  };
}
const PLUGIN_LOAD_TIMEOUT_MS = 3e4;
export async function loadPluginModuleResilient(pluginId, moduleUrl) {
  try {
    return await Promise.race(
      [
        loadPluginModule(pluginId, moduleUrl),
        new Promise((_, reject) => {
          window.setTimeout(() => reject(new Error(`timeout loading ${pluginId}`)), PLUGIN_LOAD_TIMEOUT_MS);
        })
      ]
    );
  } catch (error) {
    console.error("[DEBUG] program load failed", pluginId, error);
    return null;
  }
}
function isViewportSurface(surfaceKind) {
  return surfaceKind === "world-3d" || surfaceKind === "node-graph" || surfaceKind === "canvas-2d";
}
function defaultViewportEngagement() {
  return {
    sessionActive: true,
    status: [{ id: "framework.viewport.status", text: shellLabel("ui.engagement.viewport") }]
  };
}
export function resolveWindowEngagement(kind, windowId, byWindowId) {
  const surfaceKind = kind.surfaceKind;
  const declaredEngagement = kind.options.engagement.kind === "some" ? kind.options.engagement.value : void 0;
  return byWindowId[windowId] ?? declaredEngagement ?? (isViewportSurface(surfaceKind) ? defaultViewportEngagement() : void 0);
}
export function windowEngagementToSpec(engagement, onAction) {
  if (!engagement) return void 0;
  const options = engagement.options?.map((option) => ({
    id: option.id,
    label: option.label,
    icon: option.iconId ? /* @__PURE__ */ jsxDEV(Icon, { icon: option.iconId, size: "small" }, void 0, false, {
      fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
      lineNumber: 1360,
      columnNumber: 27
    }, this) : void 0,
    pressed: option.pressed,
    disabled: option.disabled,
    onPress: option.action ? () => onAction(option.action) : void 0
  }));
  const status = engagement.status?.map((row) => ({ id: row.id, content: row.text }));
  const control = windowEngagementControlToSpec(engagement.control, onAction);
  const controls = engagement.controls?.map((row) => windowEngagementControlToSpec(row, onAction)).filter((row) => row !== void 0);
  const hasContent = (options?.length ?? 0) > 0 || Boolean(control) || (controls?.length ?? 0) > 0 || (status?.length ?? 0) > 0;
  if (!hasContent) return void 0;
  return { sessionActive: engagement.sessionActive, options, control, controls, status };
}
export function windowEngagementToSearchSpec(engagement, onAction) {
  if (!engagement) return void 0;
  const input = engagement.input ? {
    id: engagement.input.id,
    value: engagement.input.value,
    placeholder: engagement.input.placeholder,
    disabled: engagement.input.disabled,
    onChange: engagement.input.onChange ? (value) => onAction({ ...engagement.input.onChange, args: { ...engagement.input.onChange.args, value } }) : void 0,
    onSubmit: engagement.input.onSubmit ? (value) => onAction({ ...engagement.input.onSubmit, args: { ...engagement.input.onSubmit.args, value } }) : void 0,
    onRepeatLast: engagement.input.onRepeatLast ? () => onAction(engagement.input.onRepeatLast) : void 0,
    onAbort: engagement.input.onAbort ? () => onAction(engagement.input.onAbort) : void 0
  } : void 0;
  const possibles = engagement.possibleEngagements?.map((row) => ({
    id: row.id,
    label: row.label,
    detail: row.detail,
    onSelect: row.action ? () => onAction(row.action) : void 0
  }));
  const hasContent = Boolean(input) || (possibles?.length ?? 0) > 0;
  if (!hasContent) return void 0;
  return { sessionActive: engagement.sessionActive, input, possibles };
}
function panelTabIcon(tabId, group) {
  if (group === "workbench") return shellTabIcon(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID);
  if (tabId.includes("parameters")) return shellTabIcon(FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID);
  if (tabId.includes("inspector")) return shellTabIcon(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID);
  if (tabId === FRAMEWORK_PANEL_TAB_HISTORY_ID) return shellTabIcon("undo");
  return shellTabIcon(tabId);
}
export function categoryTabIcon(tabs, fallback) {
  const FirstIcon = tabs[0]?.icon;
  return function CategoryTabIcon({ size = 16 }) {
    return FirstIcon ? /* @__PURE__ */ jsxDEV(FirstIcon, { size }, void 0, false, {
      fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
      lineNumber: 1414,
      columnNumber: 24
    }, this) : /* @__PURE__ */ jsxDEV(Icon, { icon: fallback, size: "small" }, void 0, false, {
      fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
      lineNumber: 1414,
      columnNumber: 52
    }, this);
  };
}
export function flattenPanelTabLeaves(tabs) {
  return tabs.flatMap((tab) => tab.children && tab.children.length > 0 ? flattenPanelTabLeaves(tab.children) : [tab]);
}
export function panelTabDefinitionToNode(tab, group, panelUiByKey, onAction, order, appLabelsOverlay, terminology = UI_TERMINOLOGY_NATIVE, locale = SHELL_LOCALES[0]) {
  const tabId = panelTabKindId(tab.kind);
  const label = resolvePanelTabLabel(appLabelsOverlay, tabId, resolveManifestLabel(tab.label, terminology, locale));
  if (tab.children && tab.children.length > 0) {
    return {
      kind: "branch",
      id: tabId,
      icon: panelTabIcon(tabId, group),
      name: label,
      order,
      children: tab.children.map((child, childOrder) => panelTabDefinitionToNode(child, group, panelUiByKey, onAction, childOrder, appLabelsOverlay, terminology, locale))
    };
  }
  return singleTreeLeaf({
    id: tabId,
    icon: panelTabIcon(tabId, group),
    name: label,
    order,
    tree: staticTreePanelDefinition(uiNodeToTreePanelConfig(panelUiByKey[tabId] ?? pendingPanelUiNode(), onAction))
  });
}
export function resolveCanvasBodyKey(app) {
  const windowKind = app.windowKinds[0];
  if (!windowKind) return "main";
  if (windowKind.bodyKey.includes("composite")) {
    const workflow = app.windowKinds.find((kind) => kind.bodyKey.includes("workflow"));
    return workflow?.bodyKey ?? windowKind.bodyKey;
  }
  return windowKind.bodyKey;
}
export function resolveUtilities(app, windowKind) {
  const registry = app.utilities ?? [];
  const refs = windowKind.utilities ?? [];
  if (refs.length === 0) return [...registry];
  const resolved = [];
  for (const ref of refs) {
    const utility = registry.find((entry) => entry.id === ref);
    if (utility) resolved.push(utility);
  }
  return resolved;
}
const CHROME_KNOWN_RIBBON_PARENT_CATEGORIES = new Set(UI_RIBBON_PARENT_CATEGORIES);
function resolveUtilityGroupLabel(group, appLabelsOverlay) {
  const fallback = CHROME_KNOWN_RIBBON_PARENT_CATEGORIES.has(group) ? shellLabel(`ui.ribbon.parent.${group}`) : group;
  return resolveAppLabel(appLabelsOverlay, "group", group, fallback);
}
function utilityDefinitionToSpec(utility, appLabelsOverlay, terminology, locale) {
  return {
    id: utility.id,
    label: resolveAppLabel(appLabelsOverlay, "utility", utility.id, resolveManifestLabel(utility.label, terminology, locale)),
    iconId: utility.iconId,
    group: utility.group ?? void 0,
    groupLabel: utility.group ? resolveUtilityGroupLabel(utility.group, appLabelsOverlay) : void 0,
    category: utility.category ?? "utilities"
  };
}
function tagSetActiveUtilityWindow(nodes, windowId) {
  return nodes.map((node) => {
    if (node.kind === "collection") return { ...node, children: tagSetActiveUtilityWindow(node.children, windowId) };
    if (node.kind === "toggle" && "onChange" in node && node.onChange.action === SET_ACTIVE_UTILITY_ACTION_ID) {
      return { ...node, onChange: { ...node.onChange, args: { ...node.onChange.args, windowId } } };
    }
    return node;
  });
}
export function resolveUtilityNodes(app, windowKind, activeUtilityId, windowId, appLabelsOverlay = EMPTY_APP_LABELS_OVERLAY, terminology = UI_TERMINOLOGY_NATIVE, locale = SHELL_LOCALES[0]) {
  const utilities = resolveUtilities(app, windowKind);
  if (utilities.length === 0) return [];
  return tagSetActiveUtilityWindow(
    deriveUtilityNodes(
      app.controllerId,
      utilities.map((utility) => utilityDefinitionToSpec(utility, appLabelsOverlay, terminology, locale)),
      activeUtilityId ?? void 0
    ),
    windowId
  );
}
export function spawnedWindowChromeForKind(kind, windowId, engagementsByWindowId, measuresByWindowId, activeUtilityId, onAction) {
  const { measures, utilityOptions } = windowMeasuresChrome(measuresByWindowId[windowId] ?? kind.options.measures, activeUtilityId, windowId, onAction);
  const resolvedEngagement = resolveWindowEngagement(kind, windowId, engagementsByWindowId);
  return {
    engagement: windowEngagementToSpec(resolvedEngagement, onAction),
    search: windowEngagementToSearchSpec(resolvedEngagement, onAction),
    measures,
    utilityOptions
  };
}
function isTreeNode(node) {
  return node.type === "tree";
}
export function uiNodeToTreePanelConfig(node, onAction) {
  const treeHasDrag = node.type === "tree" && node.sections.some((s) => s.items.some((i) => i.draggable || i.dragData));
  if (isTreeNode(node)) {
    return {
      ...uiTreeNodeToTreePanelConfig(node, onAction),
      dragAndDropController: node.dropAction || treeHasDrag ? declarativeTreeDragController(node, onAction) : void 0
    };
  }
  return declarativeUiNodeToTreePanelConfig(node, onAction);
}
function declarativeUiNodeToTreePanelConfig(node, onAction) {
  if (node.type === "stack") {
    const emphasized = node.children.find((child) => child.type === "text" && child.emphasize);
    const bodyChildren = node.children.filter((child) => !(child.type === "text" && child.emphasize));
    const sectionNodes = bodyChildren.filter((child) => child.type === "section");
    if (sectionNodes.length > 0 && sectionNodes.length === bodyChildren.length) {
      return {
        sections: sectionNodes.map((section) => ({
          id: section.id,
          label: section.label ?? "",
          defaultOpen: section.defaultOpen,
          items: section.children.flatMap((child, index) => declarativeUiChildToTreeItems(child, `${section.id}.${index}`, onAction))
        })),
        sortableSections: false
      };
    }
    return {
      sections: [
        {
          id: node.id ?? "panel.body",
          label: emphasized && emphasized.type === "text" ? emphasized.value : "",
          defaultOpen: true,
          items: bodyChildren.flatMap((child, index) => declarativeUiChildToTreeItems(child, `${node.id ?? "panel.body"}.${index}`, onAction))
        }
      ],
      sortableSections: false
    };
  }
  if (node.type === "section") {
    return {
      sections: [
        {
          id: node.id,
          label: node.label ?? "",
          defaultOpen: node.defaultOpen,
          items: node.children.flatMap((child, index) => declarativeUiChildToTreeItems(child, `${node.id}.${index}`, onAction))
        }
      ],
      sortableSections: false
    };
  }
  return {
    sections: [
      {
        id: "panel.body",
        label: "",
        defaultOpen: true,
        items: declarativeUiChildToTreeItems(node, "panel.body.0", onAction)
      }
    ],
    sortableSections: false
  };
}
function isUiControlNode(node) {
  switch (node.type) {
    case "button":
    case "input":
    case "select":
    case "toggle":
    case "slider":
    case "numberStepper":
    case "ring":
    case "iconSelect":
    case "keyValue":
      return true;
    default:
      return false;
  }
}
function declarativeUiChildToTreeItems(node, fallbackId, onAction) {
  switch (node.type) {
    case "field": {
      const control = isUiControlNode(node.child) ? renderUiControl(node.child, onAction) : /* @__PURE__ */ jsxDEV(InterpretedUiNode, { node: node.child, onAction }, void 0, false, {
        fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
        lineNumber: 1651,
        columnNumber: 95
      }, this);
      return [{ id: node.id, label: node.label, description: node.description, control }];
    }
    case "text":
      return [{ id: `${fallbackId}.text`, label: node.value }];
    case "button":
      return [{ id: node.id ?? fallbackId, label: node.label, control: renderUiControl(node, onAction) }];
    case "input":
    case "select":
    case "toggle":
    case "slider":
    case "numberStepper":
    case "ring":
    case "iconSelect":
    case "keyValue":
      return [{ id: node.id, label: node.placeholder ?? node.id, control: renderUiControl(node, onAction) }];
    case "stack":
      return node.children.flatMap((child, index) => declarativeUiChildToTreeItems(child, `${fallbackId}.${index}`, onAction));
    case "group":
      return [
        {
          id: node.id,
          label: node.label,
          defaultOpen: node.defaultOpen,
          items: node.children.flatMap((child, index) => declarativeUiChildToTreeItems(child, `${node.id}.${index}`, onAction))
        }
      ];
    case "tree":
      return uiTreeNodeToTreePanelConfig(node, onAction).sections.flatMap((section) => section.items);
    case "separator":
      return [{ id: `${fallbackId}.sep`, label: "—" }];
    default:
      return [
        {
          id: fallbackId,
          label: node.type,
          control: /* @__PURE__ */ jsxDEV(ShellFaultBoundary, { boundaryId: `panel-${fallbackId}`, fallbackLabel: shellLabel("ui.common.renderError"), children: /* @__PURE__ */ jsxDEV(ChromeAwareWindowScrollSurface, { className: "min-h-0 flex-1", children: interpretUiNode(node, { onAction }) }, void 0, false, {
            fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
            lineNumber: 1689,
            columnNumber: 15
          }, this) }, void 0, false, {
            fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
            lineNumber: 1688,
            columnNumber: 9
          }, this)
        }
      ];
  }
}
export function shellTabIcon(iconId) {
  return function ShellTabIcon({ size = 16 }) {
    const iconName = iconId === FRAMEWORK_PANEL_TAB_DOCUMENT_ICON_ID ? "file-text" : iconId === FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID ? "panel-catalogue" : iconId === FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID ? "panel-inspection" : iconId === FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID ? "panel-parameters" : isIconName(iconId) ? iconId : "circle-dot";
    return /* @__PURE__ */ jsxDEV(Icon, { icon: iconName, size }, void 0, false, {
      fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
      lineNumber: 1711,
      columnNumber: 12
    }, this);
  };
}
export function shellLabel(key, options) {
  return wireLabel(resolveTranslationLabel(uiI18n.t(key, options)) ?? key);
}
const FRAMEWORK_PANEL_TAB_LABEL_KEYS = {
  [FRAMEWORK_PANEL_TAB_DOCUMENT_ID]: "ui.panel.document",
  [FRAMEWORK_PANEL_TAB_CATALOGUE_ID]: "ui.panel.catalogue",
  [FRAMEWORK_PANEL_TAB_INSPECTION_ID]: "ui.panel.inspection",
  [FRAMEWORK_PANEL_TAB_PARAMETERS_ID]: "ui.panel.parameters",
  [FRAMEWORK_PANEL_TAB_HISTORY_ID]: "ui.panel.history"
};
export function resolvePanelTabLabel(overlay, tabId, fallback) {
  const chromeKey = FRAMEWORK_PANEL_TAB_LABEL_KEYS[tabId];
  return chromeKey ? shellLabel(chromeKey) : resolveAppLabel(overlay, "panelTab", tabId, fallback);
}
export const EMPTY_APP_LABELS_OVERLAY = {
  windowKindLabels: {},
  panelTabLabels: {},
  modeLabels: {},
  actionLabels: {},
  utilityLabels: {},
  exampleLabels: {},
  actionArgLabels: {},
  dialogLabels: {},
  introductionLabels: {},
  groupLabels: {}
};
export function synthesizeLocalizedLabel(label) {
  if (typeof label !== "string") return label;
  return {
    native: { en: label, de: label },
    reuse: { en: label, de: label }
  };
}
export function resolveManifestLabel(label, terminology, locale) {
  if (label === void 0) return "";
  if (typeof label === "string") return label;
  const byTerminology = label[terminology] ?? label.native ?? label.reuse;
  if (!byTerminology) return "";
  return byTerminology[locale] ?? byTerminology.en ?? Object.values(byTerminology)[0] ?? "";
}
export function resolveAppLabel(overlay, kind, id, fallback) {
  const map = kind === "windowKind" ? overlay.windowKindLabels : kind === "panelTab" ? overlay.panelTabLabels : kind === "mode" ? overlay.modeLabels : kind === "action" ? overlay.actionLabels : kind === "utility" ? overlay.utilityLabels : kind === "example" ? overlay.exampleLabels : kind === "actionArg" ? overlay.actionArgLabels : kind === "dialog" ? overlay.dialogLabels : kind === "introduction" ? overlay.introductionLabels : overlay.groupLabels;
  return map[id] ?? fallback;
}
function resolveActionArgDef(def, scopeId, overlay, terminology, locale) {
  const label = resolveAppLabel(overlay, "actionArg", `${scopeId}.${def.id}`, resolveManifestLabel(def.label, terminology, locale));
  if (def.control.kind !== "select") return label === def.label ? def : { ...def, label };
  const options = def.control.options.map((option) => ({ ...option, label: resolveAppLabel(overlay, "actionArg", `${scopeId}.${def.id}.option.${option.value}`, resolveManifestLabel(option.label, terminology, locale)) }));
  return { ...def, label, control: { ...def.control, options } };
}
export function resolveDialogDefinition(dialog, overlay, terminology, locale) {
  return {
    ...dialog,
    title: resolveAppLabel(overlay, "dialog", `${dialog.id}.title`, resolveManifestLabel(dialog.title, terminology, locale)),
    body: dialog.body ? resolveAppLabel(overlay, "dialog", `${dialog.id}.body`, resolveManifestLabel(dialog.body, terminology, locale)) : dialog.body,
    submitLabel: resolveAppLabel(overlay, "dialog", `${dialog.id}.submit`, resolveManifestLabel(dialog.submitLabel, terminology, locale)),
    cancelLabel: dialog.cancelLabel ? resolveAppLabel(overlay, "dialog", `${dialog.id}.cancel`, resolveManifestLabel(dialog.cancelLabel, terminology, locale)) : dialog.cancelLabel,
    args: dialog.args.map((def) => resolveActionArgDef(def, dialog.id, overlay, terminology, locale))
  };
}
export function resolveIntroductionDefinition(introduction, overlay, terminology, locale) {
  return {
    title: resolveAppLabel(overlay, "introduction", "intro.title", resolveManifestLabel(introduction.title, terminology, locale)),
    steps: introduction.steps.map(
      (step) => ({
        ...step,
        title: resolveAppLabel(overlay, "introduction", `intro.step.${step.id}.title`, resolveManifestLabel(step.title, terminology, locale)),
        body: resolveAppLabel(overlay, "introduction", `intro.step.${step.id}.body`, resolveManifestLabel(step.body, terminology, locale)),
        interactions: (step.interactions ?? []).map((interaction, index) => ({
          ...interaction,
          label: resolveAppLabel(overlay, "introduction", `intro.step.${step.id}.interaction.${index}.label`, interaction.label)
        })),
        ordered: step.ordered ?? false
      })
    )
  };
}
export function captureTutorialUiSnapshot(state, session) {
  const activeUtilityByWindowId = {};
  for (const [windowId, utilityId] of Object.entries(state.actionPane.activeUtilityByWindowId)) {
    if (utilityId) activeUtilityByWindowId[windowId] = utilityId;
  }
  const activePanelTabByGroup = {};
  for (const anchor of ANCHORS) {
    const panelState = state.layout.panels[anchor];
    const tabId = panelState.path[panelState.path.length - 1];
    if (panelState.visible && tabId) activePanelTabByGroup[anchor] = tabId;
  }
  return {
    activeModeId: session?.viewState.activeModeId,
    focusedWindowId: state.layout.activeWindowId ?? void 0,
    activeUtilityByWindowId,
    activeToolId: state.actionPane.activeToolId ?? void 0,
    layout: captureCurrentFrameworkLayout(state.layout.shellLayout, state.layout.extraWindowInstances),
    activePanelTabByGroup,
    panelJson: session?.viewState.panelJson,
    selectionJson: session?.viewState.selectionJson,
    openDialogId: state.overlays.dialog?.dialogId,
    expandedTreeIds: Object.entries(state.layout.treeOpenStates).filter(([, open]) => open).map(([id]) => id),
    commandPanelOpen: state.overlays.searchOpen
  };
}
export function applyTutorialUiSnapshotToShell(dispatch, snapshot, ctx) {
  const windowKinds = ctx.session?.app.windowKinds.map((kind) => ({ id: kind.id, label: kind.label })) ?? [];
  const seed = applyFrameworkLayoutSeed(snapshot.layout, windowKinds, ctx.appLabelsOverlay, ctx.terminology, ctx.locale);
  const panelPatches = {};
  for (const anchor of ANCHORS) {
    const tabId = snapshot.activePanelTabByGroup[anchor];
    panelPatches[anchor] = tabId ? { visible: true, path: [tabId] } : { visible: false, path: [] };
  }
  const treeOpenStates = {};
  for (const id of snapshot.expandedTreeIds) treeOpenStates[id] = true;
  dispatch({
    type: "APPLY_TUTORIAL_UI_SNAPSHOT",
    snapshot: {
      activeWindowId: snapshot.focusedWindowId ?? null,
      shellLayout: seed.modeLayout,
      extraWindowInstances: seed.extraInstances,
      panelPatches,
      treeOpenStates,
      activeUtilityByWindowId: snapshot.activeUtilityByWindowId,
      activeToolId: snapshot.activeToolId ?? null,
      openDialogId: snapshot.openDialogId ?? null,
      commandPanelOpen: snapshot.commandPanelOpen
    }
  });
  if (ctx.session) {
    dispatch({
      type: "SET_SESSION",
      value: (current) => current ? {
        ...current,
        viewState: {
          ...current.viewState,
          activeModeId: snapshot.activeModeId ?? current.viewState.activeModeId,
          panelJson: snapshot.panelJson ?? current.viewState.panelJson,
          selectionJson: snapshot.selectionJson ?? current.viewState.selectionJson
        }
      } : current
    });
  }
}
export function applyTutorialUiChangeToShell(dispatch, change, ctx) {
  switch (change.kind) {
    case "activeMode":
      if (!ctx.session) return;
      dispatch({ type: "SET_SESSION", value: (current) => current ? { ...current, viewState: { ...current.viewState, activeModeId: change.id } } : current });
      return;
    case "focusedWindow":
      dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: change.id ?? null });
      return;
    case "activeUtility":
      dispatch({ type: "SET_ACTIVE_UTILITY", windowId: change.windowId, utilityId: change.utilityId ?? null });
      return;
    case "activeTool":
      dispatch({ type: "SET_ACTIVE_TOOL", toolId: change.id ?? null });
      return;
    case "layout": {
      const windowKinds = ctx.session?.app.windowKinds.map((kind) => ({ id: kind.id, label: kind.label })) ?? [];
      const seed = applyFrameworkLayoutSeed(change.layout, windowKinds, ctx.appLabelsOverlay, ctx.terminology, ctx.locale);
      dispatch({ type: "SET_SHELL_LAYOUT", value: seed.modeLayout });
      dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: seed.extraInstances });
      return;
    }
    case "panelTab": {
      const anchor = change.group;
      if (!ANCHORS.includes(anchor)) return;
      dispatch({ type: "SET_PANEL_VISIBLE", anchor, value: change.tabId != null });
      dispatch({ type: "SET_PANEL_PATH", anchor, value: change.tabId ? [change.tabId] : [] });
      return;
    }
    case "panelState":
      if (!ctx.session) return;
      dispatch({ type: "SET_SESSION", value: (current) => current ? { ...current, viewState: { ...current.viewState, panelJson: change.panelJson } } : current });
      return;
    case "selection":
      if (!ctx.session) return;
      dispatch({ type: "SET_SESSION", value: (current) => current ? { ...current, viewState: { ...current.viewState, selectionJson: change.selectionJson } } : current });
      return;
    case "dialog":
      dispatch({ type: "SET_DIALOG", value: change.id ? { dialogId: change.id, seedArgs: change.args } : null });
      return;
    case "treeExpansion":
      dispatch({ type: "SET_TREE_OPEN_STATE", id: change.id, open: change.expanded });
      return;
    case "commandPanel":
      dispatch({ type: "SET_SEARCH_OPEN", value: change.open });
      return;
    default:
      return;
  }
}
export function shellTerminologyLabel(id) {
  const isChromeKnown = id === "native" || id === "reuse";
  return isChromeKnown ? shellLabel(`ui.settings.terminology.${id}`) : id;
}
export function createLatestAsyncDispatcher(dispatchValue) {
  let running = false;
  let queued;
  let hasQueued = false;
  const dispatchLatest = (value) => {
    if (running) {
      queued = value;
      hasQueued = true;
      return;
    }
    running = true;
    void Promise.resolve(dispatchValue(value)).finally(() => {
      running = false;
      if (!hasQueued) return;
      const next = queued;
      queued = void 0;
      hasQueued = false;
      dispatchLatest(next);
    });
  };
  return dispatchLatest;
}
export function createDirectionalAsyncDispatcher(dispatchValue) {
  let running = false;
  let active = 0;
  const queued = [];
  const dispatchNext = (value) => {
    running = true;
    active = value;
    void Promise.resolve(dispatchValue(value)).finally(() => {
      const next = queued.shift();
      if (next === void 0) {
        running = false;
        return;
      }
      dispatchNext(next);
    });
  };
  return (value) => {
    if (!running) {
      dispatchNext(value);
      return;
    }
    const previous = queued.at(-1);
    if (previous === void 0) {
      if (value !== active) queued.push(value);
      return;
    }
    const anchor = queued.at(-2) ?? active;
    const direction = Math.sign(previous - anchor);
    const nextDirection = Math.sign(value - previous);
    if (nextDirection === 0) return;
    if (direction === 0 || nextDirection === direction) queued[queued.length - 1] = value;
    else
      queued.push(value);
    if (queued.length > 2) queued.splice(0, queued.length - 2);
  };
}
export function createRevealCutoffStore() {
  const values = /* @__PURE__ */ new Map();
  const listeners = /* @__PURE__ */ new Map();
  return {
    get: (groupId) => values.get(groupId),
    set: (groupId, value) => {
      values.set(groupId, value);
      for (const listener of listeners.get(groupId) ?? []) listener(value);
    },
    subscribe: (groupId, listener) => {
      let group = listeners.get(groupId);
      if (!group) {
        group = /* @__PURE__ */ new Set();
        listeners.set(groupId, group);
      }
      group.add(listener);
      return () => {
        group.delete(listener);
      };
    }
  };
}
export const worldRevealCutoffStore = createRevealCutoffStore();
export const PUZZLE3D_FILL_REVEAL_GROUP_ID = "puzzle3d-fill";
export function reconcileCommittedRevealCutoffs(store, committedRef, revealCutoffs) {
  for (const [groupId, value] of Object.entries(revealCutoffs)) {
    if (committedRef.current[groupId] === value) continue;
    committedRef.current = { ...committedRef.current, [groupId]: value };
    store.set(groupId, value);
  }
}
export function isRevealCutoffHidden(instance) {
  if (instance.revealIndex == null) return false;
  const cutoff = worldRevealCutoffStore.get(PUZZLE3D_FILL_REVEAL_GROUP_ID);
  return cutoff !== void 0 && instance.revealIndex >= cutoff;
}
export function createInFlightSkippingInterval(run, delayMs, setIntervalFn = setInterval, clearIntervalFn = clearInterval) {
  let cancelled = false;
  let inFlight = false;
  const tick = () => {
    if (cancelled || inFlight) return;
    inFlight = true;
    void Promise.resolve(run()).finally(() => {
      inFlight = false;
    });
  };
  const timer = setIntervalFn(tick, delayMs);
  return () => {
    cancelled = true;
    clearIntervalFn(timer);
  };
}
export function createCoalescingActionDispatcher(dispatch, isEqual = (a, b) => Object.is(a, b)) {
  let inFlight = false;
  let pending;
  let lastSent;
  const flush = () => {
    if (inFlight || pending === void 0) return;
    const next = pending;
    pending = void 0;
    if (lastSent !== void 0 && isEqual(lastSent, next)) return;
    lastSent = next;
    inFlight = true;
    void Promise.resolve(dispatch(next)).finally(() => {
      inFlight = false;
      flush();
    });
  };
  return (value) => {
    if (pending === void 0 && lastSent !== void 0 && isEqual(lastSent, value)) return;
    pending = value;
    flush();
  };
}
export const registeredPuzzle3dBrushMeshes = /* @__PURE__ */ new Set();
export function windowMeasureTreeContainsId(measures, id) {
  for (const measure of measures) {
    if (measure.id === id) return true;
    if (measure.kind === "group" && windowMeasureTreeContainsId(measure.children, id)) return true;
  }
  return false;
}
function windowMeasureUsesProbabilityReadout(measure) {
  const step = measure.step ?? 1;
  return measure.min === 0 && measure.max <= 1 && step < 1;
}
function windowMeasureProbabilityReadout(value) {
  return `${Math.round(value * 100)}%`;
}
function WindowMeasureSlider({ measure, onAction }) {
  _s2();
  const dispatchValue = useMemo(
    () => createDirectionalAsyncDispatcher((value) => onAction({ ...measure.onChange, args: { ...measure.onChange.args, value } })),
    [measure.onChange, onAction]
  );
  const formatDisplayValue = windowMeasureUsesProbabilityReadout(measure) ? windowMeasureProbabilityReadout : void 0;
  const disabled = measure.disabled === true;
  const revealGroupId = measure.reveal;
  return /* @__PURE__ */ jsxDEV(
    Slider,
    {
      id: measure.id,
      value: [measure.value],
      min: measure.min,
      max: measure.max,
      ready: measure.ready,
      loading: measure.loading === true,
      waiting: measure.waiting === true,
      step: measure.step,
      disabled,
      clampToReady: Boolean(revealGroupId),
      formatDisplayValue,
      onValueChange: (values) => {
        if (disabled) return;
        const value = values[0] ?? measure.value;
        if (revealGroupId) {
          worldRevealCutoffStore.set(revealGroupId, value);
          return;
        }
        dispatchValue(value);
      },
      onValueCommit: revealGroupId ? (values) => {
        if (disabled) return;
        const value = values[0] ?? measure.value;
        worldRevealCutoffStore.set(revealGroupId, value);
        onAction({ ...measure.onChange, args: { ...measure.onChange.args, value } });
      } : void 0,
      onPointerCancel: revealGroupId ? () => worldRevealCutoffStore.set(revealGroupId, measure.value) : void 0
    },
    void 0,
    false,
    {
      fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
      lineNumber: 2203,
      columnNumber: 5
    },
    this
  );
}
_s2(WindowMeasureSlider, "+9Zn3B0394nqV9T2xN+Fz1K+500=");
_c = WindowMeasureSlider;
function windowMeasureGroupHeaderSlider(measure, onAction) {
  if (measure.value === void 0 || measure.onChange === void 0) return void 0;
  const sliderMeasure = {
    kind: "slider",
    id: `${measure.id}.header-slider`,
    label: void 0,
    value: measure.value,
    min: measure.min ?? 0,
    max: measure.max ?? 1,
    step: measure.step,
    ready: measure.ready,
    loading: measure.loading,
    waiting: measure.waiting,
    onChange: measure.onChange
  };
  return /* @__PURE__ */ jsxDEV(WindowMeasureSlider, { measure: sliderMeasure, onAction }, void 0, false, {
    fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
    lineNumber: 2254,
    columnNumber: 10
  }, this);
}
function windowMeasureSelectControl(measure, onAction) {
  return /* @__PURE__ */ jsxDEV(Select, { value: measure.value, onValueChange: (value) => onAction({ ...measure.onChange, args: { ...measure.onChange.args, value } }), children: [
    /* @__PURE__ */ jsxDEV(SelectTrigger, { id: measure.id, className: "h-small w-full min-w-0", size: "sm", children: /* @__PURE__ */ jsxDEV(SelectValue, {}, void 0, false, {
      fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
      lineNumber: 2261,
      columnNumber: 9
    }, this) }, void 0, false, {
      fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
      lineNumber: 2260,
      columnNumber: 7
    }, this),
    /* @__PURE__ */ jsxDEV(SelectContent, { children: measure.items.map(
      (item) => /* @__PURE__ */ jsxDEV(SelectItem, { value: item.value, children: item.label }, item.id, false, {
        fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
        lineNumber: 2265,
        columnNumber: 9
      }, this)
    ) }, void 0, false, {
      fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
      lineNumber: 2263,
      columnNumber: 7
    }, this)
  ] }, void 0, true, {
    fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
    lineNumber: 2259,
    columnNumber: 5
  }, this);
}
function windowMeasureToggleControl(measure, onAction) {
  const label = measure.label ?? measure.text ?? measure.id;
  return /* @__PURE__ */ jsxDEV(
    TreeCheckbox,
    {
      id: measure.id,
      checked: measure.pressed,
      title: label,
      ariaLabel: label,
      onCheckedChange: (pressed) => onAction({ ...measure.onChange, args: { ...measure.onChange.args, pressed } })
    },
    void 0,
    false,
    {
      fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
      lineNumber: 2277,
      columnNumber: 5
    },
    this
  );
}
function windowMeasureToggleIcon(measure) {
  return /* @__PURE__ */ jsxDEV(Icon, { icon: measure.iconId, size: 12 }, void 0, false, {
    fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
    lineNumber: 2288,
    columnNumber: 10
  }, this);
}
function windowMeasuresToTreeItems(measures, onAction, reverseForUpPanel = true) {
  const ordered = reverseForUpPanel ? [...measures].reverse() : [...measures];
  const mapMeasure = (measure) => {
    if (measure.kind === "group") {
      return {
        id: measure.id,
        label: measure.label,
        defaultOpen: measure.defaultOpen,
        control: windowMeasureGroupHeaderSlider(measure, onAction),
        items: measure.children.length > 0 ? windowMeasuresToTreeItems(measure.children, onAction, false) : void 0
      };
    }
    if (measure.kind === "slider") {
      return {
        id: measure.id,
        label: measure.label ?? "",
        control: /* @__PURE__ */ jsxDEV(WindowMeasureSlider, { measure, onAction }, void 0, false, {
          fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
          lineNumber: 2311,
          columnNumber: 18
        }, this),
        loading: measure.loading,
        waiting: measure.waiting
      };
    }
    if (measure.kind === "select") {
      return {
        id: measure.id,
        label: measure.label ?? "",
        control: windowMeasureSelectControl(measure, onAction)
      };
    }
    return {
      id: measure.id,
      label: measure.label ?? measure.text ?? "",
      icon: windowMeasureToggleIcon(measure),
      control: windowMeasureToggleControl(measure, onAction)
    };
  };
  return ordered.map(mapMeasure);
}
function renderWindowMeasure(measure, onAction) {
  if (measure.kind === "group") {
    const headerSlider = windowMeasureGroupHeaderSlider(measure, onAction);
    return /* @__PURE__ */ jsxDEV(WindowMeasureTreeGroup, { id: measure.id, label: measure.label, defaultOpen: measure.defaultOpen, headerControl: headerSlider, children: measure.children.map((child) => renderWindowMeasure(child, onAction)) }, measure.id, false, {
      fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
      lineNumber: 2337,
      columnNumber: 7
    }, this);
  }
  if (measure.kind === "select") {
    return /* @__PURE__ */ jsxDEV(WindowMeasureTreeLeaf, { label: measure.label, children: windowMeasureSelectControl(measure, onAction) }, measure.id, false, {
      fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
      lineNumber: 2344,
      columnNumber: 7
    }, this);
  }
  if (measure.kind === "slider") {
    return /* @__PURE__ */ jsxDEV(WindowMeasureTreeLeaf, { label: measure.label, children: /* @__PURE__ */ jsxDEV(WindowMeasureSlider, { measure, onAction }, void 0, false, {
      fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
      lineNumber: 2352,
      columnNumber: 9
    }, this) }, measure.id, false, {
      fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
      lineNumber: 2351,
      columnNumber: 7
    }, this);
  }
  if (measure.kind === "toggle") {
    return /* @__PURE__ */ jsxDEV(WindowMeasureTreeLeaf, { label: measure.label ?? measure.text, icon: windowMeasureToggleIcon(measure), children: windowMeasureToggleControl(measure, onAction) }, measure.id, false, {
      fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
      lineNumber: 2358,
      columnNumber: 7
    }, this);
  }
  return null;
}
function windowMeasuresOverlay(measures, onAction, direction = "down") {
  if (!measures || measures.length === 0) return void 0;
  return /* @__PURE__ */ jsxDEV(WindowMeasuresTree, { direction, children: measures.map((measure) => renderWindowMeasure(measure, onAction)) }, void 0, false, {
    fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
    lineNumber: 2368,
    columnNumber: 10
  }, this);
}
export function renderWindowMeasuresTree(measures, onAction, direction = "down") {
  return windowMeasuresOverlay(measures, onAction, direction);
}
export function SelectionUtilityOptions({ activeUtilityId, windowId, onAction }) {
  _s3();
  const methodLabel = useLabel("ui.selection.method");
  const modeLabel = useLabel("ui.selection.mode");
  const rectangleLabel = useLabel("ui.selection.rectangle");
  const lassoLabel = useLabel("ui.selection.lasso");
  const selectiveLabel = useLabel("ui.selection.selective");
  const additiveLabel = useLabel("ui.selection.additive");
  const subtractiveLabel = useLabel("ui.selection.subtractive");
  const invertiveLabel = useLabel("ui.selection.invertive");
  const selectionMethod = activeUtilityId === "selectLasso" ? "lasso" : "rectangle";
  const selectionStore = useShellScope().selection;
  const [selectionMode, setSelectionMode] = useState(() => selectionStore.get());
  const handleModeChange = (mode) => {
    selectionStore.set(mode);
    setSelectionMode(mode);
  };
  const handleMethodChange = (method) => {
    onAction({
      controllerId: "window",
      action: SET_ACTIVE_UTILITY_ACTION_ID,
      args: { windowId, utilityId: method === "lasso" ? "selectLasso" : "selectMarquee" }
    });
  };
  return /* @__PURE__ */ jsxDEV("div", { className: "flex items-center gap-double", children: [
    /* @__PURE__ */ jsxDEV("div", { className: "flex items-center gap-single", children: [
      /* @__PURE__ */ jsxDEV("span", { className: "text-tiny text-muted-foreground uppercase tracking-wider font-semibold", children: methodLabel }, void 0, false, {
        fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
        lineNumber: 2409,
        columnNumber: 9
      }, this),
      /* @__PURE__ */ jsxDEV(
        ToggleGroup,
        {
          kind: "single",
          value: selectionMethod,
          onValueChange: (val) => {
            if (val === "rectangle" || val === "lasso") {
              handleMethodChange(val);
            }
          },
          items: [
            { value: "rectangle", icon: /* @__PURE__ */ jsxDEV(Icon, { icon: "square-dashed", size: "small" }, void 0, false, {
              fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
              lineNumber: 2419,
              columnNumber: 39
            }, this), text: rectangleLabel },
            { value: "lasso", icon: /* @__PURE__ */ jsxDEV(Icon, { icon: "lasso", size: "small" }, void 0, false, {
              fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
              lineNumber: 2420,
              columnNumber: 35
            }, this), text: lassoLabel }
          ]
        },
        void 0,
        false,
        {
          fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
          lineNumber: 2410,
          columnNumber: 9
        },
        this
      )
    ] }, void 0, true, {
      fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
      lineNumber: 2408,
      columnNumber: 7
    }, this),
    /* @__PURE__ */ jsxDEV(RibbonDivider, {}, void 0, false, {
      fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
      lineNumber: 2424,
      columnNumber: 7
    }, this),
    /* @__PURE__ */ jsxDEV("div", { className: "flex items-center gap-single", children: [
      /* @__PURE__ */ jsxDEV("span", { className: "text-tiny text-muted-foreground uppercase tracking-wider font-semibold", children: modeLabel }, void 0, false, {
        fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
        lineNumber: 2426,
        columnNumber: 9
      }, this),
      /* @__PURE__ */ jsxDEV(
        ToggleGroup,
        {
          kind: "single",
          value: selectionMode,
          onValueChange: (val) => {
            if (val === "default" || val === "additive" || val === "subtractive" || val === "invertive") {
              handleModeChange(val);
            }
          },
          items: [
            { value: "default", text: selectiveLabel },
            { value: "additive", text: additiveLabel },
            { value: "subtractive", text: subtractiveLabel },
            { value: "invertive", text: invertiveLabel }
          ]
        },
        void 0,
        false,
        {
          fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
          lineNumber: 2427,
          columnNumber: 9
        },
        this
      )
    ] }, void 0, true, {
      fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
      lineNumber: 2425,
      columnNumber: 7
    }, this)
  ] }, void 0, true, {
    fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
    lineNumber: 2407,
    columnNumber: 5
  }, this);
}
_s3(SelectionUtilityOptions, "5IApAt80HiOUdQqYVTirmXFimSo=", false, function() {
  return [useLabel, useLabel, useLabel, useLabel, useLabel, useLabel, useLabel, useLabel, useShellScope];
});
_c2 = SelectionUtilityOptions;
export function windowMeasuresChrome(measures, activeUtilityId, windowId, onAction) {
  const { general, utilityOptions } = partitionWindowMeasures(measures ?? [], activeUtilityId);
  const taggedOnAction = (action) => onAction({ ...action, args: { ...action.args, windowId } });
  return {
    measures: windowMeasuresOverlay(general, taggedOnAction),
    utilityOptions: windowMeasuresOverlay(utilityOptions, taggedOnAction, "up")
  };
}
export function utilityNodeTreeContainsId(nodes, targetId) {
  return nodes.some((node) => node.id === targetId || node.kind === "collection" && utilityNodeTreeContainsId(node.children, targetId));
}
export function utilityBarNode(utilities, windowId, onAction, revealUtilityId, utilityOptions) {
  if (!utilities?.length && !utilityOptions) return void 0;
  const categories = groupUtilityNodesByCategory(utilities ?? [], UTILITY_CATEGORIES);
  if (!categories.length && !utilityOptions) return void 0;
  const grouped = [];
  for (const node of categories) {
    if (node.kind === "collection" && (node.category === "utilities" || node.category === "selection")) {
      if (node.id === "group:Select" || node.id === "group:selection" || node.label === "Select" || node.text === "Select") {
        grouped.push(...node.children);
      } else {
        for (const child of node.children) {
          if (child.kind === "collection" && (child.id === "group:Select" || child.id === "group:selection" || child.label === "Select" || child.text === "Select")) {
            grouped.push(...child.children);
          } else {
            grouped.push(child);
          }
        }
      }
    } else {
      grouped.push(node);
    }
  }
  return /* @__PURE__ */ jsxDEV(UtilityTree, { id: `ui.utilities.${windowId}`, utilities: grouped, onAction, direction: "up", revealUtilityId, utilityOptions }, void 0, false, {
    fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
    lineNumber: 2492,
    columnNumber: 10
  }, this);
}
export function renderStagedArgControl(def, value, onChange, disabled) {
  const control = def.control;
  switch (control.kind) {
    case "text":
      return /* @__PURE__ */ jsxDEV(Input, { id: def.id, type: "text", className: "h-medium w-full min-w-0", value: typeof value === "string" ? value : "", placeholder: control.placeholder, disabled, onChange: (event) => onChange(event.target.value) }, void 0, false, {
        fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
        lineNumber: 2506,
        columnNumber: 14
      }, this);
    case "number":
      return /* @__PURE__ */ jsxDEV(
        Input,
        {
          id: def.id,
          type: "number",
          className: "h-medium w-full min-w-0",
          value: value === void 0 || value === null || value === "" ? "" : String(value),
          min: control.min,
          max: control.max,
          step: control.step,
          disabled,
          onChange: (event) => onChange(event.target.value === "" ? void 0 : Number(event.target.value))
        },
        void 0,
        false,
        {
          fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
          lineNumber: 2509,
          columnNumber: 9
        },
        this
      );
    case "slider": {
      const numeric = typeof value === "number" && Number.isFinite(value) ? value : control.min;
      const slider = /* @__PURE__ */ jsxDEV(Slider, { id: def.id, className: "w-full min-w-0", min: control.min, max: control.max, step: control.step ?? 1, value: [numeric], disabled, onValueChange: (values) => onChange(values[0] ?? numeric) }, void 0, false, {
        fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
        lineNumber: 2523,
        columnNumber: 24
      }, this);
      if (!control.unit) return slider;
      return /* @__PURE__ */ jsxDEV("div", { className: "flex w-full min-w-0 items-center gap-single", children: [
        slider,
        /* @__PURE__ */ jsxDEV("span", { className: "shrink-0 text-xs tabular-nums text-muted-foreground", children: [
          numeric,
          " ",
          control.unit
        ] }, void 0, true, {
          fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
          lineNumber: 2528,
          columnNumber: 11
        }, this)
      ] }, void 0, true, {
        fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
        lineNumber: 2526,
        columnNumber: 11
      }, this);
    }
    case "toggle":
      return /* @__PURE__ */ jsxDEV(Toggle, { id: def.id, pressed: value === true, text: def.label, disabled, onPressedChange: (pressed) => onChange(pressed) }, void 0, false, {
        fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
        lineNumber: 2535,
        columnNumber: 14
      }, this);
    case "select":
      return /* @__PURE__ */ jsxDEV(Select, { value: typeof value === "string" && value ? value : void 0, disabled, onValueChange: (next) => onChange(next), children: [
        /* @__PURE__ */ jsxDEV(SelectTrigger, { id: def.id, className: "h-medium w-full min-w-0", size: "sm", children: /* @__PURE__ */ jsxDEV(SelectValue, { placeholder: def.label }, void 0, false, {
          fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
          lineNumber: 2540,
          columnNumber: 13
        }, this) }, void 0, false, {
          fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
          lineNumber: 2539,
          columnNumber: 11
        }, this),
        /* @__PURE__ */ jsxDEV(SelectContent, { children: control.options.map(
          (option, index) => /* @__PURE__ */ jsxDEV(SelectItem, { value: option.value, children: option.label }, `${def.id}:${index}:${option.value}`, false, {
            fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
            lineNumber: 2544,
            columnNumber: 13
          }, this)
        ) }, void 0, false, {
          fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
          lineNumber: 2542,
          columnNumber: 11
        }, this)
      ] }, void 0, true, {
        fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
        lineNumber: 2538,
        columnNumber: 9
      }, this);
    case "vec3": {
      const tuple = Array.isArray(value) && value.length >= 3 ? value : null;
      const axes = ["x", "y", "z"];
      return /* @__PURE__ */ jsxDEV("div", { className: "grid grid-cols-3 gap-single", children: axes.map(
        (axis, index) => /* @__PURE__ */ jsxDEV(
          Input,
          {
            id: `${def.id}.${axis}`,
            type: "number",
            className: "h-medium w-full min-w-0",
            value: tuple ? String(tuple[index] ?? 0) : "",
            placeholder: axis,
            disabled,
            onChange: (event) => {
              const parsed = Number(event.target.value);
              if (!Number.isFinite(parsed)) return;
              const next = tuple ? [tuple[0] ?? 0, tuple[1] ?? 0, tuple[2] ?? 0] : [0, 0, 0];
              next[index] = parsed;
              onChange(next);
            }
          },
          `${def.id}.${axis}`,
          false,
          {
            fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
            lineNumber: 2557,
            columnNumber: 13
          },
          this
        )
      ) }, void 0, false, {
        fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
        lineNumber: 2555,
        columnNumber: 11
      }, this);
    }
    case "iconSelect":
      return /* @__PURE__ */ jsxDEV(IconSelector, { id: def.id, classifyIconSelectorMode: void 0, value: typeof value === "string" ? value : "", uniform: true, onChange: (next) => onChange(next) }, void 0, false, {
        fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
        lineNumber: 2578,
        columnNumber: 14
      }, this);
  }
}
export function actionRequiresStagedForm(action) {
  return (action.args?.length ?? 0) > 0;
}
export function isEditableEventTarget(target) {
  if (!(target instanceof HTMLElement)) return false;
  const tag = target.tagName;
  if (tag === "INPUT" || tag === "TEXTAREA" || tag === "SELECT") return true;
  if (target.isContentEditable) return true;
  return target.closest("[contenteditable='true'], [role='textbox']") != null;
}
export function keyboardEventMatchesChord(event, chord) {
  const parts = chord.split("+").map((part) => part.trim());
  const key = parts[parts.length - 1] ?? "";
  const needsCtrl = parts.includes("ctrl") || parts.includes("meta") || parts.includes("mod");
  const needsShift = parts.includes("shift");
  const needsAlt = parts.includes("alt");
  const hasCtrl = event.ctrlKey || event.metaKey;
  if (needsCtrl !== hasCtrl) return false;
  if (needsShift !== event.shiftKey) return false;
  if (needsAlt !== event.altKey) return false;
  return event.key.toLowerCase() === key;
}
export function resolveKeybindingIntent(definition, expandedActionId, stagedArgs) {
  if (!definition || !actionRequiresStagedForm(definition)) return { kind: "fire" };
  if (expandedActionId === definition.id) {
    const effective = effectiveActionArgs(definition.args, stagedArgs);
    if (missingRequiredArgs(definition.args, effective).length === 0) return { kind: "execute", actionId: definition.id, args: effective };
  }
  return { kind: "open", actionId: definition.id };
}
export function resolveUtilityActivation(current, requested) {
  return requested === "" || (current ?? null) === requested ? null : requested;
}
export function actionCategoryId(action) {
  return action.category ?? (action.kind === "history" ? "history" : "actions");
}
function actionCategoryLabel(category, appLabelsOverlay) {
  const fallback = CHROME_KNOWN_RIBBON_PARENT_CATEGORIES.has(category) ? shellLabel(`ui.ribbon.parent.${category}`) : category;
  return resolveAppLabel(appLabelsOverlay, "group", category, fallback);
}
export function actionCategories(actions, appLabelsOverlay = EMPTY_APP_LABELS_OVERLAY) {
  const seen = /* @__PURE__ */ new Set();
  const categories = [];
  for (const action of actions) {
    const id = actionCategoryId(action);
    if (seen.has(id)) continue;
    seen.add(id);
    categories.push({ id, label: actionCategoryLabel(id, appLabelsOverlay) });
  }
  return categories;
}
export function buildActionCategoryTree(windowId, controllerId, actions, expandedActionId, stagedArgsByKey, disabled, onExpandedChange, onStageArg, onResetArgs, onExecute, appLabelsOverlay = EMPTY_APP_LABELS_OVERLAY) {
  const categories = actionCategories(actions, appLabelsOverlay);
  const expandedAction = expandedActionId ? actions.find((action) => action.id === expandedActionId) : void 0;
  const sections = [];
  for (const category of categories) {
    const categoryActions = actions.filter((action) => actionCategoryId(action) === category.id);
    sections.push({
      id: `action.category.${category.id}`,
      label: category.label,
      defaultOpen: true,
      items: categoryActions.map((action) => {
        const icon = action.iconId ? /* @__PURE__ */ jsxDEV(Icon, { icon: action.iconId, size: "small" }, void 0, false, {
          fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
          lineNumber: 2686,
          columnNumber: 38
        }, this) : void 0;
        const rowClassName = disabled ? "pointer-events-none opacity-50" : void 0;
        if (!actionRequiresStagedForm(action)) {
          return { id: `action.${action.id}`, label: action.label, icon, className: rowClassName, onClick: () => !disabled && onExecute({ controllerId, action: action.id }) };
        }
        const expanded = expandedActionId === action.id;
        return {
          id: `action.${action.id}`,
          label: `${action.label}…`,
          icon: icon ?? /* @__PURE__ */ jsxDEV(Icon, { icon: expanded ? "chevron-down" : "chevron-right", size: "small" }, void 0, false, {
            fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
            lineNumber: 2695,
            columnNumber: 25
          }, this),
          className: rowClassName,
          onClick: () => !disabled && onExpandedChange(expanded ? null : action.id)
        };
      })
    });
    if (expandedAction && actionCategoryId(expandedAction) === category.id) {
      const staged = stagedArgsByKey[actionStageKey(windowId, expandedAction.id)] ?? {};
      const effective = effectiveActionArgs(expandedAction.args, staged);
      const missing = missingRequiredArgs(expandedAction.args, effective);
      sections.push({
        id: `action.category.${category.id}.form`,
        defaultOpen: true,
        items: expandedAction.args.map(
          (def) => ({
            id: `action.${expandedAction.id}.arg.${def.id}`,
            label: def.label,
            description: def.description,
            control: renderStagedArgControl(def, effective[def.id], (value) => onStageArg(expandedAction.id, def.id, value), disabled)
          })
        ),
        actions: [
          {
            id: childElementId("framework.window", windowId, "action", expandedAction.id, "execute"),
            icon: /* @__PURE__ */ jsxDEV(Icon, { icon: "check", size: "small" }, void 0, false, {
              fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
              lineNumber: 2719,
              columnNumber: 17
            }, this),
            text: shellLabel("ui.common.execute"),
            disabled: disabled || missing.length > 0,
            onClick: () => onExecute({ controllerId, action: expandedAction.id, args: effective })
          },
          {
            id: childElementId("framework.window", windowId, "action", expandedAction.id, "reset"),
            icon: /* @__PURE__ */ jsxDEV(Icon, { icon: "undo", size: "small" }, void 0, false, {
              fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
              lineNumber: 2726,
              columnNumber: 17
            }, this),
            text: shellLabel("ui.common.reset"),
            disabled,
            onClick: () => onResetArgs(expandedAction.id)
          }
        ]
      });
    }
  }
  return sections;
}
export function WindowActionPane(props) {
  const { windowId, controllerId, actions, expandedActionId, stagedArgsByKey, disabled, onExpandedChange, onStageArg, onResetArgs, onExecute, appLabelsOverlay } = props;
  const sections = buildActionCategoryTree(windowId, controllerId, actions, expandedActionId, stagedArgsByKey, disabled, onExpandedChange, onStageArg, onResetArgs, onExecute, appLabelsOverlay);
  return /* @__PURE__ */ jsxDEV("div", { "data-slot": "window-action-pane", className: "flex min-w-0 flex-col", children: /* @__PURE__ */ jsxDEV(Tree, { sections, showLines: false, sortableSections: false }, void 0, false, {
    fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
    lineNumber: 2766,
    columnNumber: 7
  }, this) }, void 0, false, {
    fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
    lineNumber: 2765,
    columnNumber: 5
  }, this);
}
_c3 = WindowActionPane;
export function windowActionPaneNode(app, windowKind, windowId, actionPane, onAction, dispatch, appLabelsOverlay = EMPTY_APP_LABELS_OVERLAY, terminology = UI_TERMINOLOGY_NATIVE, locale = SHELL_LOCALES[0]) {
  const resolvedActions = resolveWindowActions(app, windowKind);
  if (resolvedActions.length === 0) return void 0;
  const actions = resolvedActions.map((action) => ({
    ...action,
    label: resolveAppLabel(appLabelsOverlay, "action", action.id, resolveManifestLabel(action.label, terminology, locale)),
    args: action.args.map((def) => resolveActionArgDef(def, action.id, appLabelsOverlay, terminology, locale))
  }));
  const activeUtilityId = actionPane.activeUtilityByWindowId[windowId] ?? null;
  const activeUtility = activeUtilityId ? (app.utilities ?? []).find((utility) => utility.id === activeUtilityId) : void 0;
  const disabled = Boolean(activeUtility && activeUtility.allowsActionsWhileActive === false);
  return /* @__PURE__ */ jsxDEV(
    WindowActionPane,
    {
      windowId,
      controllerId: app.controllerId,
      actions,
      expandedActionId: actionPane.expandedByWindowId[windowId] ?? null,
      stagedArgsByKey: actionPane.stagedArgsByKey,
      disabled,
      onExpandedChange: (actionId) => dispatch({ type: "SET_ACTION_PANE_EXPANDED", windowId, value: actionId }),
      onStageArg: (actionId, argId, value) => dispatch({ type: "STAGE_ACTION_ARG", windowId, actionId, argId, value }),
      onResetArgs: (actionId) => dispatch({ type: "RESET_ACTION_ARGS", windowId, actionId }),
      onExecute: onAction,
      appLabelsOverlay
    },
    void 0,
    false,
    {
      fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
      lineNumber: 2802,
      columnNumber: 5
    },
    this
  );
}
export function resolveCommands(osCommands, activePluginManifest, app, activeModeId, overlay = EMPTY_APP_LABELS_OVERLAY, terminology = UI_TERMINOLOGY_NATIVE, locale = SHELL_LOCALES[0]) {
  const resolveDefinition = (definition) => ({
    ...definition,
    label: resolveManifestLabel(definition.label, terminology, locale),
    args: definition.args.map((def) => resolveActionArgDef(def, definition.id, overlay, terminology, locale))
  });
  const resolved = osCommands.map((definition) => ({ definition: resolveDefinition(definition), source: { kind: "os" } }));
  for (const definition of activePluginManifest?.commands ?? []) {
    resolved.push({ definition: resolveDefinition(definition), source: { kind: "plugin" } });
  }
  if (!app) return resolved;
  const activeMode = app.modes?.find((mode) => mode.id === activeModeId);
  const modeCommandIds = new Set(activeMode?.commands ?? []);
  for (const definition of app.commands ?? []) {
    if (definition.scope === "app") resolved.push({ definition: resolveDefinition(definition), source: { kind: "app" } });
    else if (definition.scope === "mode" && modeCommandIds.has(definition.id)) resolved.push({ definition: resolveDefinition(definition), source: { kind: "mode", modeId: activeModeId } });
  }
  return resolved;
}
const CHROME_KNOWN_COMMAND_CATEGORIES = /* @__PURE__ */ new Set(["general", "driver", "app", "appearance", "layout", "language", "terminology", "theme"]);
function titleizeCommandCategory(category) {
  return category.replace(/[-_]+/g, " ").replace(/\b\w/g, (char) => char.toUpperCase());
}
export function commandCategoryLabel(category) {
  return CHROME_KNOWN_COMMAND_CATEGORIES.has(category) ? shellLabel(`ui.settings.tab.${category}`) : titleizeCommandCategory(category);
}
export function commandCategories(commands) {
  const seen = /* @__PURE__ */ new Set();
  const categories = [];
  for (const { definition } of commands) {
    if (seen.has(definition.category)) continue;
    seen.add(definition.category);
    categories.push({ id: definition.category, label: commandCategoryLabel(definition.category) });
  }
  return categories;
}
function selectCommandArg(id, label, options) {
  return { id, label, control: { kind: "select", options: options.map((option) => ({ ...option })) }, required: true };
}
export function driverDisplayLabel(driver) {
  if (driver.id === "default") return shellLabel("settings.driver.default");
  if (driver.id === "compact") return shellLabel("settings.driver.compact");
  return driver.label || driver.id;
}
export function buildOsCommands(themeList, terminologies, hasIntroduction, locks = EMPTY_SHELL_LOCKS, driverList = builtinUiDrivers(), tutorials = [], tutorialRecorderAvailable = false, terminology = UI_TERMINOLOGY_NATIVE, locale = SHELL_LOCALES[0]) {
  const lockedCommandIds = /* @__PURE__ */ new Set([...locks.appearance ? ["os.setAppearance"] : [], ...locks.themeId ? ["os.setThemeId"] : [], ...locks.locale ? ["os.setLocale"] : [], ...locks.terminology ? ["os.setTerminology"] : []]);
  const commands = [
    ...hasIntroduction ? [{ id: "os.introduceApp", label: shellLabel("ui.command.introduceApp"), scope: "os", category: "app", inPalette: true, args: [] }] : [],
    // 🎥️ `os.playTutorial` only appears once at least one tutorial is declared (app-own or brand-own);
    // `os.recordTutorial` is dev/studio-only (see `isTutorialRecorderAvailable`) and needs no declared
    // tutorial at all — recording an app IS the authoring path for one.
    ...tutorials.length > 0 ? [{ id: "os.playTutorial", label: shellLabel("ui.command.playTutorial"), scope: "os", category: "app", inPalette: true, args: [selectCommandArg("tutorialId", shellLabel("tutorial.chapter"), tutorials.map((tutorial) => ({ value: tutorial.id, label: resolveManifestLabel(tutorial.title, terminology, locale) })))] }] : [],
    ...tutorialRecorderAvailable ? [{ id: "os.recordTutorial", label: shellLabel("ui.command.recordTutorial"), scope: "os", category: "app", inPalette: true, args: [] }] : [],
    {
      id: "os.setAppearance",
      label: shellLabel("ui.command.setAppearance"),
      scope: "os",
      category: "appearance",
      inPalette: true,
      args: [
        selectCommandArg(
          "appearance",
          shellLabel("ui.settings.tab.appearance"),
          [
            { value: "system", label: shellLabel("ui.settings.appearance.system") },
            { value: "light", label: shellLabel("ui.settings.appearance.light") },
            { value: "dark", label: shellLabel("ui.settings.appearance.dark") }
          ]
        )
      ]
    },
    {
      id: "os.setThemeId",
      label: shellLabel("ui.command.setTheme"),
      scope: "os",
      category: "appearance",
      inPalette: true,
      args: [
        selectCommandArg(
          "themeId",
          shellLabel("ui.settings.tab.theme"),
          themeList.map((theme) => ({ value: theme.id, label: theme.label || theme.id }))
        )
      ]
    },
    {
      id: "os.setLayout",
      label: shellLabel("ui.command.setLayout"),
      scope: "os",
      category: "layout",
      inPalette: true,
      args: [
        selectCommandArg(
          "layout",
          shellLabel("ui.settings.tab.layout"),
          [
            { value: "desktop", label: shellLabel("settings.layout.desktop") },
            { value: "tablet", label: shellLabel("settings.layout.tablet") }
          ]
        )
      ]
    },
    { id: "os.resetDock", label: shellLabel("ui.settings.resetDock"), scope: "os", category: "layout", inPalette: true, args: [] },
    {
      id: "os.setLocale",
      label: shellLabel("ui.command.setLocale"),
      scope: "os",
      category: "language",
      inPalette: true,
      args: [
        selectCommandArg(
          "locale",
          shellLabel("ui.settings.tab.language"),
          [
            { value: "en", label: shellLabel("ui.settings.language.en") },
            { value: "de", label: shellLabel("ui.settings.language.de") }
          ]
        )
      ]
    },
    {
      id: "os.setTerminology",
      label: shellLabel("ui.command.setTerminology"),
      scope: "os",
      category: "language",
      inPalette: true,
      args: [
        selectCommandArg(
          "terminology",
          shellLabel("ui.settings.tab.terminology"),
          terminologies.map((id) => ({ value: id, label: shellTerminologyLabel(id) }))
        )
      ]
    },
    {
      id: "os.setDriver",
      label: shellLabel("ui.command.setDriver"),
      scope: "os",
      category: "layout",
      inPalette: true,
      args: [
        selectCommandArg(
          "driver",
          shellLabel("ui.settings.tab.driver"),
          driverList.map((driver) => ({ value: driver.id, label: driverDisplayLabel(driver) }))
        )
      ]
    }
  ];
  return commands.filter((command) => !lockedCommandIds.has(command.id));
}
export function dispatchOsCommand(commandId, args, dispatch, dockLayoutStore, dockUiStateStore, locks = EMPTY_SHELL_LOCKS) {
  switch (commandId) {
    case "os.introduceApp":
      dispatch({ type: "SET_INTRODUCTION_STEP", value: 0 });
      return;
    case "os.setAppearance":
      if (locks.appearance) return;
      dispatch({ type: "SET_UI_APPEARANCE", value: args?.appearance ?? "system" });
      return;
    case "os.setThemeId":
      if (locks.themeId) return;
      if (typeof args?.themeId === "string") dispatch({ type: "SET_UI_THEME_ID", value: args.themeId });
      return;
    case "os.setLayout":
      dispatch({ type: "SET_UI_LAYOUT", value: args?.layout ?? "desktop" });
      return;
    case "os.resetDock":
      dispatch({ type: "RESET_DOCK" });
      dockLayoutStore.reset();
      dockUiStateStore.reset();
      return;
    case "os.setLocale":
      if (locks.locale) return;
      if (typeof args?.locale === "string") {
        setUiLocale(args.locale);
        dispatch({ type: "SET_UI_LOCALE", value: args.locale });
      }
      return;
    case "os.setTerminology":
      if (locks.terminology) return;
      if (typeof args?.terminology === "string") dispatch({ type: "SET_UI_TERMINOLOGY", value: args.terminology });
      return;
    case "os.setDriver":
      if (typeof args?.driver === "string") dispatch({ type: "SET_UI_DRIVER_ID", value: args.driver });
      return;
    default:
      return;
  }
}
const COMMAND_CATEGORY_ICON = shellTabIcon("wrench");
export function buildCommandCategoryTree(commands, expandedCommandId, stagedArgsByCommandId, onExecute, onToggleExpanded, onStageArg, onResetArgs) {
  const argCarryingCommands = commands.filter((entry) => entry.definition.args.length > 0);
  const autoExpandedSingleton = argCarryingCommands.length === 1 ? argCarryingCommands[0] : void 0;
  const expanded = (expandedCommandId ? commands.find((entry) => entry.definition.id === expandedCommandId) : void 0) ?? autoExpandedSingleton;
  const effectiveExpandedId = expanded?.definition.id ?? null;
  const sections = [];
  if (expanded && expanded.definition.args.length > 0) {
    const staged = stagedArgsByCommandId[expanded.definition.id] ?? {};
    const effective = effectiveActionArgs(expanded.definition.args, staged);
    const missing = missingRequiredArgs(expanded.definition.args, effective);
    sections.push({
      id: `command.category.${expanded.definition.category}.form`,
      items: expanded.definition.args.map(
        (def) => ({
          id: `command.${expanded.definition.id}.arg.${def.id}`,
          label: def.label,
          description: def.description,
          control: renderStagedArgControl(def, effective[def.id], (value) => onStageArg(expanded.definition.id, def.id, value))
        })
      ),
      actions: [
        {
          id: `command-${expanded.definition.id}-execute`,
          icon: /* @__PURE__ */ jsxDEV(Icon, { icon: "check", size: "small" }, void 0, false, {
            fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
            lineNumber: 3111,
            columnNumber: 15
          }, this),
          text: shellLabel("ui.common.execute"),
          disabled: missing.length > 0,
          onClick: () => onExecute(expanded, effective)
        },
        {
          id: `command-${expanded.definition.id}-reset`,
          icon: /* @__PURE__ */ jsxDEV(Icon, { icon: "undo", size: "small" }, void 0, false, {
            fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
            lineNumber: 3118,
            columnNumber: 15
          }, this),
          text: shellLabel("ui.common.reset"),
          onClick: () => onResetArgs(expanded.definition.id)
        }
      ]
    });
  }
  const listCommands = commands.filter((entry) => entry.definition.id !== effectiveExpandedId);
  if (listCommands.length > 0) {
    sections.push({
      id: "command.category.list",
      items: listCommands.map((entry) => {
        const argCarrying = entry.definition.args.length > 0;
        const icon = entry.definition.iconId ? /* @__PURE__ */ jsxDEV(Icon, { icon: entry.definition.iconId, size: "small" }, void 0, false, {
          fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
          lineNumber: 3131,
          columnNumber: 48
        }, this) : void 0;
        if (!argCarrying) return { id: `command.${entry.definition.id}`, label: entry.definition.label, icon, onClick: () => onExecute(entry) };
        return {
          id: `command.${entry.definition.id}`,
          label: `${entry.definition.label}…`,
          icon: /* @__PURE__ */ jsxDEV(Icon, { icon: expandedCommandId === entry.definition.id ? "chevron-down" : "chevron-up", size: "small" }, void 0, false, {
            fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
            lineNumber: 3136,
            columnNumber: 17
          }, this),
          onClick: () => onToggleExpanded(expandedCommandId === entry.definition.id ? null : entry.definition.id)
        };
      })
    });
  }
  return { sections };
}
export function buildCommandCategoryTabs(resolvedCommands, categories, expandedCommandIdRef, stagedArgsByCommandIdRef, onCommand, dispatch) {
  return categories.map((category) => {
    const categoryCommands = resolvedCommands.filter((entry) => entry.definition.category === category.id);
    return singleTreeLeaf({
      id: `command.category.${category.id}`,
      icon: COMMAND_CATEGORY_ICON,
      name: category.label,
      tree: {
        resolveTree: () => buildCommandCategoryTree(
          categoryCommands,
          expandedCommandIdRef.current,
          stagedArgsByCommandIdRef.current,
          (entry, executeArgs) => onCommand(entry.source, entry.definition.id, executeArgs),
          (commandId) => dispatch({ type: "SET_COMMAND_EXPANDED", value: commandId }),
          (commandId, argId, value) => dispatch({ type: "STAGE_COMMAND_ARG", commandId, argId, value }),
          (commandId) => dispatch({ type: "RESET_COMMAND_ARGS", commandId })
        )
      }
    });
  });
}
function buildToolTree(tool, controllerId, isActive, measures, onAction) {
  const iconName = tool.iconId;
  if (isActive && measures && measures.length > 0) {
    return {
      sortableSections: false,
      sections: [
        {
          id: `tool.${tool.id}.options`,
          label: "",
          defaultOpen: true,
          items: windowMeasuresToTreeItems(measures, onAction)
        }
      ]
    };
  }
  return {
    sortableSections: false,
    sections: [
      {
        id: `tool.${tool.id}.activate`,
        label: "",
        defaultOpen: true,
        items: [
          {
            id: `tool.${tool.id}.activate.toggle`,
            label: "",
            control: /* @__PURE__ */ jsxDEV(
              Toggle,
              {
                id: `tool.${tool.id}`,
                pressed: isActive,
                text: tool.label,
                icon: /* @__PURE__ */ jsxDEV(Icon, { icon: iconName, size: "small" }, void 0, false, {
                  fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
                  lineNumber: 3223,
                  columnNumber: 17
                }, this),
                onPressedChange: (pressed) => onAction({ controllerId, action: SET_ACTIVE_TOOL_ACTION_ID, args: { toolId: pressed ? tool.id : "" } })
              },
              void 0,
              false,
              {
                fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
                lineNumber: 3219,
                columnNumber: 9
              },
              this
            )
          }
        ]
      }
    ]
  };
}
export function buildToolTabs(tools, controllerId, activeToolIdRef, toolMeasuresByToolIdRef, onAction) {
  return tools.map(
    (tool) => singleTreeLeaf({
      id: `tool.${tool.id}`,
      icon: shellTabIcon(tool.iconId),
      name: tool.label,
      tree: {
        resolveTree: () => {
          const tree = buildToolTree(tool, controllerId, activeToolIdRef.current === tool.id, toolMeasuresByToolIdRef.current[tool.id], onAction);
          return { sections: tree.sections, sortableSections: tree.sortableSections };
        }
      }
    })
  );
}
export function toolIdFromPanelTabId(tabId) {
  if (!tabId?.startsWith("tool.")) return null;
  const toolId = tabId.slice("tool.".length);
  return toolId.length > 0 ? toolId : null;
}
function uiJsonDeepEqual(a, b) {
  if (a === b) return true;
  if (typeof a !== "object" || typeof b !== "object" || a === null || b === null) return false;
  if (Array.isArray(a) !== Array.isArray(b)) return false;
  if (Array.isArray(a) && Array.isArray(b)) {
    if (a.length !== b.length) return false;
    for (let index = 0; index < a.length; index += 1) {
      if (!uiJsonDeepEqual(a[index], b[index])) return false;
    }
    return true;
  }
  const aRecord = a;
  const bRecord = b;
  const aKeys = Object.keys(aRecord);
  const bKeys = Object.keys(bRecord);
  if (aKeys.length !== bKeys.length) return false;
  for (const key of aKeys) {
    if (!Object.prototype.hasOwnProperty.call(bRecord, key)) return false;
    if (!uiJsonDeepEqual(aRecord[key], bRecord[key])) return false;
  }
  return true;
}
export function preserveJsonIdentity(previous, next) {
  return previous !== void 0 && uiJsonDeepEqual(previous, next) ? previous : next;
}
export function mergeRecordPreservingIdentity(prev, entries) {
  const next = {};
  let changed = Object.keys(prev).length !== entries.length;
  for (const [key, value] of entries) {
    const preserved = preserveJsonIdentity(prev[key], value);
    next[key] = preserved;
    if (preserved !== prev[key]) changed = true;
  }
  return changed ? next : prev;
}
export function patchWorld3dChromeOntoNode(node, patch) {
  if (node.type !== "component" || !node.world3d) return node;
  const next = {
    ...node,
    world3d: {
      ...node.world3d,
      selectionJson: patch.selectionJson,
      ...patch.vorticesJson !== void 0 ? { vorticesJson: patch.vorticesJson } : {}
    }
  };
  return preserveJsonIdentity(node, next);
}
export function patchDocumentTreeSelectedIds(node, selectedIds, highlightedIds) {
  if (node.type !== "tree") return node;
  const next = {
    ...node,
    selectedIds: [...selectedIds],
    ...highlightedIds ? { highlightedIds: [...highlightedIds] } : {}
  };
  return preserveJsonIdentity(node, next);
}
function uiRefreshWantsWindow(scope, bodyKey) {
  return scope.kind === "full" || scope.kind === "partial" && (scope.windowBodies ?? []).includes(bodyKey);
}
function uiRefreshWantsPanel(scope, bodyKey) {
  return scope.kind === "full" || scope.kind === "partial" && (scope.panelBodies ?? []).includes(bodyKey);
}
function uiRefreshWantsFlag(scope, flag) {
  return scope.kind === "full" || scope.kind === "partial" && scope[flag] === true;
}
export function sessionWindowInstances(app, extraWindowInstances) {
  const kindById = new Map(app.windowKinds.map((kind) => [kind.id, kind]));
  const base = app.windowKinds.map((kind) => ({ id: kind.id, bodyKey: kind.bodyKey, windowKindId: kind.id }));
  const extra = extraWindowInstances.flatMap((instance) => {
    const kind = kindById.get(instance.windowKindId);
    return kind ? [{ id: instance.id, bodyKey: kind.bodyKey, windowKindId: instance.windowKindId }] : [];
  });
  return [...base, ...extra];
}
export function introductionTargetsWindow(windowId, windowKindId, targetKindId, targetSegment = null) {
  if (targetKindId && (elementIdSegment(windowId) === elementIdSegment(targetKindId) || elementIdSegment(windowKindId) === elementIdSegment(targetKindId))) return true;
  if (targetSegment && (elementIdSegment(windowId) === targetSegment || elementIdSegment(windowKindId) === targetSegment)) return true;
  return false;
}
export function buildActiveUtilityByWindowId(activeUtilityByWindowId) {
  return Object.fromEntries(Object.entries(activeUtilityByWindowId).flatMap(([windowId, utilityId]) => utilityId ? [[windowId, utilityId]] : []));
}
export function buildUiRefreshRequest(scope, windowInstances, panelTabLeaves, viewState, cache) {
  if (scope.kind === "none") return null;
  const windows = windowInstances.filter((instance) => uiRefreshWantsWindow(scope, instance.bodyKey)).map((instance) => ({ key: instance.id, bodyKey: instance.bodyKey, hash: cache.get(`window:${instance.id}`)?.hash }));
  const panels = panelTabLeaves.filter((tab) => Boolean(tab.bodyKey) && uiRefreshWantsPanel(scope, tab.bodyKey)).map((tab) => ({ key: panelTabKindId(tab.kind), bodyKey: tab.bodyKey, hash: cache.get(`panel:${panelTabKindId(tab.kind)}`)?.hash }));
  const engagements = uiRefreshWantsFlag(scope, "engagements") ? { hash: cache.get("engagements")?.hash } : void 0;
  const measures = uiRefreshWantsFlag(scope, "measures") ? { hash: cache.get("measures")?.hash } : void 0;
  const tools = uiRefreshWantsFlag(scope, "tools") ? { hash: cache.get("tools")?.hash } : void 0;
  const labels = uiRefreshWantsFlag(scope, "labels") ? { hash: cache.get("labels")?.hash } : void 0;
  if (windows.length === 0 && panels.length === 0 && !engagements && !measures && !tools && !labels) return null;
  return { viewState, windows, panels, engagements, measures, tools, labels };
}
function applyUiRefreshSectionsToCache(cache, prefix, entries) {
  for (const entry of entries ?? []) {
    if (entry.value !== void 0) cache.set(`${prefix}:${entry.key}`, { hash: entry.hash, value: entry.value });
  }
}
export function applyUiRefreshResponseToCache(cache, response) {
  applyUiRefreshSectionsToCache(cache, "window", response.windows);
  applyUiRefreshSectionsToCache(cache, "panel", response.panels);
  if (response.engagements?.value !== void 0) cache.set("engagements", { hash: response.engagements.hash, value: response.engagements.value });
  if (response.measures?.value !== void 0) cache.set("measures", { hash: response.measures.hash, value: response.measures.value });
  if (response.tools?.value !== void 0) cache.set("tools", { hash: response.tools.hash, value: response.tools.value });
  if (response.labels?.value !== void 0) cache.set("labels", { hash: response.labels.hash, value: response.labels.value });
}
var _c, _c2, _c3;
$RefreshReg$(_c, "WindowMeasureSlider");
$RefreshReg$(_c2, "SelectionUtilityOptions");
$RefreshReg$(_c3, "WindowActionPane");
import * as RefreshRuntime from "/@react-refresh";
const inWebWorker = typeof WorkerGlobalScope !== "undefined" && self instanceof WorkerGlobalScope;
if (import.meta.hot && !inWebWorker) {
  if (!window.$RefreshReg$) {
    throw new Error(
      "@vitejs/plugin-react can't detect preamble. Something is wrong."
    );
  }
  RefreshRuntime.__hmr_import(import.meta.url).then((currentExports) => {
    RefreshRuntime.registerExportsForReactRefresh("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx", currentExports);
    import.meta.hot.accept((nextExports) => {
      if (!nextExports) return;
      const invalidateMessage = RefreshRuntime.validateRefreshBoundaryAndEnqueueUpdate("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx", currentExports, nextExports);
      if (invalidateMessage) import.meta.hot.invalidate(invalidateMessage);
    });
  });
}
function $RefreshReg$(type, id) {
  return RefreshRuntime.register(type, "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx " + id);
}
function $RefreshSig$() {
  return RefreshRuntime.createSignatureFunctionForTransform();
}

//# sourceMappingURL=data:application/json;base64,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LE1BQU0sQ0FBQztBQUNsSSxNQUFJclMsU0FBUzJyQixPQUFPdFosVUFBVS9GLE9BQVc2aEIsT0FBTXhvQixJQUFJLFNBQVMsRUFBRTBvQixNQUFNcnVCLFNBQVMyckIsTUFBTTBDLE1BQU1oYyxPQUFPclMsU0FBUzJyQixNQUFNdFosTUFBTSxDQUFDO0FBQ3RILE1BQUlyUyxTQUFTdXVCLFFBQVFsYyxVQUFVL0YsT0FBVzZoQixPQUFNeG9CLElBQUksVUFBVSxFQUFFMG9CLE1BQU1ydUIsU0FBU3V1QixPQUFPRixNQUFNaGMsT0FBT3JTLFNBQVN1dUIsT0FBT2xjLE1BQU0sQ0FBQztBQUM1SDtBQUVBLElBQUFzYyxJQUFBQyxLQUFBcEg7QUFBQSxhQUFBbUgsSUFBQTtBQUFBLGFBQUFDLEtBQUE7QUFBQSxhQUFBcEgsS0FBQSIsIm5hbWVzIjpbInVzZUNhbGxiYWNrIiwidXNlRWZmZWN0IiwidXNlTWVtbyIsInVzZVN0YXRlIiwiaXNJY29uTmFtZSIsImRlcml2ZVV0aWxpdHlOb2RlcyIsImVmZmVjdGl2ZUFjdGlvbkFyZ3MiLCJGUkFNRVdPUktfUEFORUxfVEFCX0NBVEFMT0dVRV9JQ09OX0lEIiwiRlJBTUVXT1JLX1BBTkVMX1RBQl9DQVRBTE9HVUVfSUQiLCJGUkFNRVdPUktfUEFORUxfVEFCX0RPQ1VNRU5UX0lDT05fSUQiLCJGUkFNRVdPUktfUEFORUxfVEFCX0RPQ1VNRU5UX0lEIiwiRlJBTUVXT1JLX1BBTkVMX1RBQl9ISVNUT1JZX0lEIiwiRlJBTUVXT1JLX1BBTkVMX1RBQl9JTlNQRUNUSU9OX0lDT05fSUQiLCJGUkFNRVdPUktfUEFORUxfVEFCX0lOU1BFQ1RJT05fSUQiLCJGUkFNRVdPUktfUEFORUxfVEFCX1BBUkFNRVRFUlNfSUNPTl9JRCIsIkZSQU1FV09SS19QQU5FTF9UQUJfUEFSQU1FVEVSU19JRCIsIm1pc3NpbmdSZXF1aXJlZEFyZ3MiLCJwYW5lbFRhYktpbmRJZCIsInBhcnRpdGlvbldpbmRvd01lYXN1cmVzIiwicGVuZGluZ1BhbmVsVWlOb2RlIiwiUkVDT1JEX1RVVE9SSUFMX0FDVElPTl9JRCIsInJlc29sdmVQbHVnaW5Ib3N0Q29uZmlnIiwicmVzb2x2ZVVpRGlydHlTY29wZSIsInJlc29sdmVXaW5kb3dBY3Rpb25zIiwiU0VUX0FDVElWRV9UT09MX0FDVElPTl9JRCIsIlNFVF9BQ1RJVkVfVVRJTElUWV9BQ1RJT05fSUQiLCJTSEVMTF9MT0NBTEVTIiwiU1RBUlRfSU5UUk9EVUNUSU9OX0FDVElPTl9JRCIsIlNUQVJUX1RVVE9SSUFMX0FDVElPTl9JRCIsImVuY29kZUFjdGlvbldpcmUiLCJwYWNrVmFsdWVGcm9tQmFzZTY0IiwicGFja1ZhbHVlVG9CYXNlNjQiLCJkZWNvZGVXb3JsZFByb2plY3Rpb25UZW1wbGF0ZUlkIiwiQU5DSE9SUyIsImJ1aWx0aW5VaURyaXZlcnMiLCJjaGlsZEVsZW1lbnRJZCIsIkNocm9tZUF3YXJlV2luZG93U2Nyb2xsU3VyZmFjZSIsImNyZWF0ZUV2ZW5XaW5kb3dMYXlvdXQiLCJlbGVtZW50SWRTZWdtZW50IiwiSWNvbiIsIkljb25TZWxlY3RvciIsIklucHV0IiwicmVzb2x2ZVRyYW5zbGF0aW9uTGFiZWwiLCJSaWJib25EaXZpZGVyIiwiU2VsZWN0IiwiU2VsZWN0Q29udGVudCIsIlNlbGVjdEl0ZW0iLCJTZWxlY3RUcmlnZ2VyIiwiU2VsZWN0VmFsdWUiLCJzZXRVaUxvY2FsZSIsInNpbmdsZVRyZWVMZWFmIiwiU2xpZGVyIiwic3RhdGljVHJlZVBhbmVsRGVmaW5pdGlvbiIsIlRvZ2dsZSIsIlRvZ2dsZUdyb3VwIiwiVHJlZSIsIlRyZWVDaGVja2JveCIsIlVJX1JJQkJPTl9QQVJFTlRfQ0FURUdPUklFUyIsIlVJX1RFUk1JTk9MT0dZX05BVElWRSIsInVpRGF0YUxhYmVsIiwidWlJMThuIiwidXNlTGFiZWwiLCJ1c2VTaGVsbFNjb3BlIiwiV2luZG93TWVhc3VyZXNUcmVlIiwiV2luZG93TWVhc3VyZVRyZWVHcm91cCIsIldpbmRvd01lYXN1cmVUcmVlTGVhZiIsImRlY2xhcmF0aXZlVHJlZURyYWdDb250cm9sbGVyIiwiSW50ZXJwcmV0ZWRVaU5vZGUiLCJpbnRlcnByZXRVaU5vZGUiLCJyZW5kZXJVaUNvbnRyb2wiLCJ1aVRyZWVOb2RlVG9UcmVlUGFuZWxDb25maWciLCJ3aXJlTGFiZWwiLCJhY3Rpb25TdGFnZUtleSIsIkVNUFRZX1NIRUxMX0xPQ0tTIiwiU2hlbGxGYXVsdEJvdW5kYXJ5IiwicmVnaXN0ZXJQZW5kaW5nV29ybGRQcm9qZWN0aW9uIiwiZ3JvdXBVdGlsaXR5Tm9kZXNCeUNhdGVnb3J5IiwiVVRJTElUWV9DQVRFR09SSUVTIiwiVXRpbGl0eVRyZWUiLCJsb2FkUGx1Z2luTW9kdWxlIiwic3luY0RvY3VtZW50SWQiLCJzZXNzaW9uIiwicGFuZWwiLCJzdHVkaW9Nb2RlIiwiYWN0aXZlU3Bhd25lZElkIiwic3Bhd25lZCIsInNwYXduZWRBcHBzIiwiZmluZCIsImVudHJ5IiwiaWQiLCJwbHVnaW5JZCIsImluc3RhbmNlSWQiLCJERUZBVUxUX1BBTkVMX1dJRFRIX1BYIiwiRlJBTUVXT1JLX0NBVEVHT1JZX0RJU1BMQVlfSUQiLCJGUkFNRVdPUktfQ0FURUdPUllfQ09NTUFORF9JRCIsIkZSQU1FV09SS19DQVRFR09SWV9UT09MX0lEIiwiUEFORUxfVEFCX0JBUl9IT1NUUyIsIkFQUF9ET0NVTUVOVF9TRVBBUkFUT1IiLCJOT1RFX1dPUkxEX05BVklHQVRJT05fQUNUSU9OX0lEIiwiTk9URV9TSEVMTF9DT01NQU5EX0FDVElPTl9JRCIsIkZSQU1FV09SS19SRVNFUlZFRF9BQ1RJT05fSURTIiwiU2V0IiwiYnVpbGROb3RlU2hlbGxDb21tYW5kQWN0aW9uIiwiY29udHJvbGxlcklkIiwiY29tbWFuZElkIiwibGFiZWwiLCJkZXRhaWwiLCJhY3Rpb24iLCJhcmdzIiwiVFVUT1JJQUxfUkVDT1JESU5HX0VYQ0xVREVEX0FDVElPTl9JRFMiLCJQUkVTRU5DRV9DTElFTlRfU1RPUkFHRV9LRVkiLCJQUkVTRU5DRV9IRUFSVEJFQVRfSU5URVJWQUxfTVMiLCJwcmVzZW5jZUlkZW50aXR5UGFja0Jhc2U2NCIsImlkZW50aXR5IiwicHJlc2VuY2VJZGVudGl0eUZyb21QYWNrQmFzZTY0IiwiZW5jb2RlZCIsImRlY29kZWQiLCJjbGllbnRJZCIsIm5hbWUiLCJwcmVzZW5jZUNsaWVudElkZW50aXR5IiwiZXBoZW1lcmFsIiwid2luZG93Iiwic3RvcmVkIiwic2Vzc2lvblN0b3JhZ2UiLCJnZXRJdGVtIiwicGFyc2VkIiwiTWF0aCIsInJhbmRvbSIsInRvU3RyaW5nIiwic2xpY2UiLCJ0b1VwcGVyQ2FzZSIsInNldEl0ZW0iLCJyZWFkQnJvd3NlclVyaSIsImxvY2F0aW9uIiwicGF0aG5hbWUiLCJzZWFyY2giLCJ1c2VVSUhpc3RvcnkiLCJpbml0aWFsVXJpIiwic3luY0Jyb3dzZXIiLCJfcyIsImhpc3RvcnkiLCJzZXRIaXN0b3J5IiwiZW50cmllcyIsInVyaSIsImluZGV4IiwiY2FuR29CYWNrIiwiY2FuR29Gb3J3YXJkIiwibGVuZ3RoIiwic2VnbWVudHMiLCJzcGxpdCIsImZpbHRlciIsIkJvb2xlYW4iLCJjYW5Hb1VwIiwicGFyZW50VXJpIiwiam9pbiIsImdvQmFjayIsInByZXYiLCJnb0ZvcndhcmQiLCJnb1VwIiwibmV3RW50cmllcyIsIm5hdmlnYXRlIiwidGFyZ2V0VXJpIiwiZXhpc3RpbmdJbmRleCIsImZpbmRJbmRleCIsInN5bmNVcmkiLCJjdXJyZW50IiwicHVzaFN0YXRlIiwib25Qb3BTdGF0ZSIsImFkZEV2ZW50TGlzdGVuZXIiLCJyZW1vdmVFdmVudExpc3RlbmVyIiwiZG93bmxvYWRNZWRpYUV4cG9ydCIsImZpbGVuYW1lIiwibWltZVR5cGUiLCJkYXRhIiwiZW5jb2RpbmciLCJkb2N1bWVudCIsInBheWxvYWQiLCJVaW50OEFycmF5IiwiZnJvbSIsImF0b2IiLCJjaGFyIiwiY2hhckNvZGVBdCIsImJsb2IiLCJCbG9iIiwidHlwZSIsInVybCIsIlVSTCIsImNyZWF0ZU9iamVjdFVSTCIsImFuY2hvciIsImNyZWF0ZUVsZW1lbnQiLCJocmVmIiwiZG93bmxvYWQiLCJjbGljayIsInJldm9rZU9iamVjdFVSTCIsImRvd25sb2FkRGF0YVVybCIsImRhdGFVcmwiLCJyZXF1ZXN0RmlsZU9wZW4iLCJhY2NlcHQiLCJyZWFkQXMiLCJtdWx0aXBsZSIsIlByb21pc2UiLCJyZXNvbHZlIiwiaW5wdXQiLCJvbmNoYW5nZSIsImZpbGVzIiwiQXJyYXkiLCJvcGVuZWQiLCJmaWxlIiwiY29udGVudHMiLCJyZXNvbHZlRmlsZSIsInJlYWRlciIsIkZpbGVSZWFkZXIiLCJvbmxvYWQiLCJyZXN1bHQiLCJvbmVycm9yIiwicmVhZEFzRGF0YVVSTCIsInB1c2giLCJ0ZXh0IiwibWFrZUVmZmVjdERpc3BhdGNoT25lIiwicGx1Z2luRW50cnkiLCJiYXNlU2Vzc2lvbiIsImFwcGx5RWZmZWN0cyIsInJlc3BvbnNlIiwiaGFuZGxlIiwiaGFuZGxlQWN0aW9uIiwiYXBwIiwidmlld1N0YXRlIiwicmVxdWVzdGVkRWZmZWN0cyIsInVpU2NvcGUiLCJkaXNwYXRjaE9wZW5lZEZpbGVzIiwiaW1wb3J0QWN0aW9uIiwiZGlzcGF0Y2hPbmUiLCJ0b3RhbCIsInNjaGVkdWxlRGlzcGF0Y2hBY3Rpb24iLCJkZWxheU1zIiwic2NoZWR1bGUiLCJmbiIsIm1zIiwic2V0VGltZW91dCIsIndhbGtCbWZmQm94ZXMiLCJ2aWV3Iiwic3RhcnQiLCJlbmQiLCJib3hlcyIsIm9mZnNldCIsInNpemUzMiIsImdldFVpbnQzMiIsIlN0cmluZyIsImZyb21DaGFyQ29kZSIsImdldFVpbnQ4IiwiaGVhZGVyU2l6ZSIsImJveFNpemUiLCJOdW1iZXIiLCJnZXRCaWdVaW50NjQiLCJmaW5kQm1mZkJveCIsImJveCIsInByb2JlTXA0VmlkZW9UcmFjayIsImJ5dGVzIiwiRGF0YVZpZXciLCJidWZmZXIiLCJieXRlT2Zmc2V0IiwiYnl0ZUxlbmd0aCIsIm1vb3YiLCJ0cmFrIiwibWRpYSIsIm1kaWFCb3hlcyIsImhkbHIiLCJoYW5kbGVyVHlwZSIsIm1kaGQiLCJtaW5mIiwidGltZXNjYWxlIiwic3RibCIsInRyYWNrIiwicHJvYmVTYW1wbGVUYWJsZSIsInBhcnNlU3RzZCIsInN0c2QiLCJlbnRyeU9mZnNldCIsImVudHJ5U2l6ZSIsImZvcm1hdCIsImNvZGVjIiwidmlzdWFsRW50cnlTdGFydCIsIndpZHRoIiwiZ2V0VWludDE2IiwiaGVpZ2h0IiwiaW5uZXIiLCJjb25maWciLCJkZXNjcmlwdGlvbiIsInBhcnNlU3RzeiIsInVuaWZvcm1TaXplIiwic2FtcGxlQ291bnQiLCJmaWxsIiwic2l6ZXMiLCJpIiwicGFyc2VDaHVua09mZnNldHMiLCJpczY0IiwiY291bnQiLCJvZmZzZXRzIiwicGFyc2VDaHVua09mU2FtcGxlIiwiY2h1bmtDb3VudCIsImVudHJ5Q291bnQiLCJmaXJzdENodW5rIiwic2FtcGxlc1BlckNodW5rIiwiY2h1bmtPZlNhbXBsZSIsImVudHJ5SW5kZXgiLCJuZXh0Rmlyc3RDaHVuayIsImNodW5rIiwiaW5DaHVuayIsImNvbXB1dGVTYW1wbGVPZmZzZXRzIiwiY2h1bmtPZmZzZXRzIiwiY3Vyc29yQnlDaHVuayIsIk1hcCIsImJhc2UiLCJnZXQiLCJzZXQiLCJhY2N1bXVsYXRlVGltZXN0YW1wc01zIiwic3R0cyIsInRpbWVzdGFtcHMiLCJ0aWNrcyIsImRlbHRhIiwicGFyc2VTeW5jU2FtcGxlcyIsInN5bmMiLCJhZGQiLCJzdGJsQm94ZXMiLCJzdHNjIiwic3RzeiIsInN0Y28iLCJzYW1wbGVPZmZzZXRzIiwidGltZXN0YW1wc01zIiwic3RzcyIsInN5bmNTYW1wbGVzIiwic2FtcGxlcyIsIm1hcCIsInNpemUiLCJ0aW1lc3RhbXBNcyIsImlzU3luYyIsImhhcyIsIndlYkNvZGVjc0F2YWlsYWJsZSIsInNjb3BlIiwiVmlkZW9EZWNvZGVyIiwiRW5jb2RlZFZpZGVvQ2h1bmsiLCJhdmNDb2RlY1N0cmluZyIsImhleCIsImJ5dGUiLCJwYWRTdGFydCIsImpwZWdEYXRhVXJsRnJvbUZyYW1lIiwiZnJhbWUiLCJjYW52YXMiLCJjb2RlZFdpZHRoIiwiY29kZWRIZWlnaHQiLCJnZXRDb250ZXh0IiwiZHJhd0ltYWdlIiwidG9EYXRhVVJMIiwiZGVjb2RlT25lTXA0RnJhbWUiLCJ0YXJnZXRJbmRleCIsInN5bmNJbmRleCIsImNhcHR1cmVkIiwicmVqZWN0IiwiZGVjb2RlciIsIm91dHB1dCIsImNsb3NlIiwiZXJyb3IiLCJjb25maWd1cmUiLCJzYW1wbGUiLCJkZWNvZGUiLCJ0aW1lc3RhbXAiLCJzdWJhcnJheSIsImZsdXNoIiwidGhlbiIsInJ1blRpZXIxVmlkZW9GcmFtZXMiLCJlZmZlY3QiLCJkdXJhdGlvbk1zIiwic2FtcGxlTWVkaWFGcmFtZVRpbWVzdGFtcHNNcyIsInNhbXBsZVN0cmlkZSIsIm1heEZyYW1lcyIsImZwc0hpbnQiLCJzYW1wbGVkQ291bnQiLCJ0YXJnZXRNcyIsInRhcmdldFNhbXBsZUluZGV4IiwiZnJhbWVBY3Rpb24iLCJmcmFtZUluZGV4IiwiZG9uZUFjdGlvbiIsImZyYW1lQ291bnQiLCJzdHJpZGUiLCJmcHMiLCJzdGVwTXMiLCJrIiwidHMiLCJjYXB0dXJlQ2FudmFzRnJhbWUiLCJ2aWRlbyIsIm1heExvbmdFZGdlUHgiLCJzb3VyY2VXaWR0aCIsInZpZGVvV2lkdGgiLCJzb3VyY2VIZWlnaHQiLCJ2aWRlb0hlaWdodCIsInNjYWxlIiwibWluIiwibWF4Iiwicm91bmQiLCJ3YWl0Rm9yVmlkZW9FdmVudCIsImhhbmRsZXIiLCJydW5UaWVyMlZpZGVvRnJhbWVzIiwicmVhZHlTdGF0ZSIsImlzRmluaXRlIiwiZHVyYXRpb24iLCJjdXJyZW50VGltZSIsImJ5dGVzRnJvbURhdGFVcmwiLCJiaW5hcnkiLCJpbmRleE9mIiwiYnl0ZXNUb0RhdGFVcmwiLCJtaW1lIiwiYnRvYSIsInJ1blJlcXVlc3RNZWRpYUZyYW1lcyIsImNyZWF0ZVZpZGVvRWxlbWVudCIsIm11dGVkIiwicGxheXNJbmxpbmUiLCJzcmMiLCJjb25zb2xlIiwiZmFsbGJhY2tBY3Rpb24iLCJpc1N0dWRpb01vZGUiLCJwbHVnaW5GaWx0ZXIiLCJ1bmRlZmluZWQiLCJwYXJzZVNoZWxsUm91dGUiLCJwYXRoIiwibm9ybWFsaXplZCIsInRyaW0iLCJraW5kIiwibWF0Y2giLCJleGVjIiwic3BhY2VJZCIsInBhcnNlU3BhY2VTaGVsbFBhdGgiLCJyb3V0ZSIsImFwcERvY3VtZW50TGFiZWwiLCJyZXNvbHZlQXBwRG9jdW1lbnQiLCJ0ZXJtaW5vbG9neSIsInRlcm1pbm9sb2d5RG9jdW1lbnRzIiwicmVzb2x2ZURvY3VtZW50QnlBcHBJZCIsImxvYWRlZFBsdWdpbnMiLCJhcHBJZCIsInByb2dyYW0iLCJtYW5pZmVzdCIsImFwcHMiLCJjYW5kaWRhdGUiLCJhcHBXaW5kb3dEb2N1bWVudExhYmVsIiwid2luZG93TGFiZWwiLCJsb2NhbGUiLCJ0cmltbWVkIiwib3ZlcnJpZGUiLCJyZXNvbHZlTWFuaWZlc3RMYWJlbCIsImJ1aWxkU3BhY2VQYW5lbFN0YXRlIiwicHJvZ3JhbXMiLCJhY3RpdmVQYW5lbFRhYiIsInBhbmVsSnNvbkZyb21TdGF0ZSIsInN0YXRlIiwicGFyc2VQYW5lbFN0YXRlIiwicGFuZWxKc29uIiwic3R1ZGlvUGFuZWxGb2N1c2luZ1NwYXduZWQiLCJzb21lIiwidmlld1N0YXRlV2l0aFNwYWNlUGFuZWwiLCJwYW5lbEFuY2hvckZvckdyb3VwIiwiZ3JvdXAiLCJjb2xsZWN0RnJhbWV3b3JrTGF5b3V0V2luZG93U2VlZHMiLCJub2RlIiwicGFyZW50U2l6ZSIsIndpbmRvd0lkIiwid2luZG93S2luZElkIiwidGl0bGUiLCJ0ZW1wbGF0ZUlkIiwiY2hpbGRyZW4iLCJjaGlsZCIsImNoaWxkU2l6ZXMiLCJleHBsaWNpdFRvdGFsIiwicmVkdWNlIiwic3VtIiwidW5zZXRDb3VudCIsImRlZmF1bHRFYWNoIiwiZmxhdE1hcCIsImZyYWN0aW9uIiwicmVzb2x2ZUZyYW1ld29ya1dpbmRvd1RpdGxlIiwiYmFrZWRUaXRsZSIsIndpbmRvd0tpbmRzIiwiY29udmVydEZyYW1ld29ya0xheW91dE5vZGVUb01vZGVMYXlvdXQiLCJhcHBMYWJlbHNPdmVybGF5IiwicmVzb2x2ZUFwcExhYmVsIiwicmV0aXRsZVdpbmRvd0xheW91dE5vZGUiLCJleHRyYUluc3RhbmNlcyIsImV4dHJhIiwicmVzb2x2ZUZyYW1ld29ya0xheW91dFNlZWQiLCJsYXlvdXQiLCJ3aW5kb3dJZHMiLCJyb290IiwibW9kZUxheW91dCIsInBlbmRpbmdQcm9qZWN0aW9ucyIsInNlZWRzIiwia2luZEJ5SWQiLCJzZWVkIiwiYXBwbHlGcmFtZXdvcmtMYXlvdXRTZWVkIiwicGVuZGluZyIsInByb2plY3Rpb25TcGVjIiwibW9kZUxheW91dE5vZGVUb0ZyYW1ld29yayIsImtpbmRCeUluc3RhbmNlSWQiLCJjYXB0dXJlQ3VycmVudEZyYW1ld29ya0xheW91dCIsInNoZWxsTGF5b3V0IiwiZXh0cmFXaW5kb3dJbnN0YW5jZXMiLCJmYWxsYmFjayIsIkxBWU9VVF9DSEFOR0VfU0VUVExFX01TIiwid2luZG93TGF5b3V0U2tlbGV0b24iLCJ3aW5kb3dMYXlvdXRTaXplZFNrZWxldG9uIiwiY2xhc3NpZnlXaW5kb3dMYXlvdXRDaGFuZ2UiLCJwcmV2aW91cyIsIm5leHQiLCJKU09OIiwic3RyaW5naWZ5Iiwid2luZG93RW5nYWdlbWVudENvbnRyb2xUb1NwZWMiLCJjb250cm9sIiwib25BY3Rpb24iLCJ2YWx1ZSIsImRpc2FibGVkIiwib3B0aW9ucyIsInJvdyIsIm9uU2VsZWN0IiwicGxhY2Vob2xkZXIiLCJpdGVtcyIsIm9uQ2hhbmdlIiwiZGlzcGF0Y2hOdW1lcmljIiwic3RlcCIsInVuaXQiLCJvbkNvbW1pdCIsIlBMVUdJTl9MT0FEX1RJTUVPVVRfTVMiLCJsb2FkUGx1Z2luTW9kdWxlUmVzaWxpZW50IiwibW9kdWxlVXJsIiwicmFjZSIsIl8iLCJFcnJvciIsImlzVmlld3BvcnRTdXJmYWNlIiwic3VyZmFjZUtpbmQiLCJkZWZhdWx0Vmlld3BvcnRFbmdhZ2VtZW50Iiwic2Vzc2lvbkFjdGl2ZSIsInN0YXR1cyIsInNoZWxsTGFiZWwiLCJyZXNvbHZlV2luZG93RW5nYWdlbWVudCIsImJ5V2luZG93SWQiLCJkZWNsYXJlZEVuZ2FnZW1lbnQiLCJlbmdhZ2VtZW50Iiwid2luZG93RW5nYWdlbWVudFRvU3BlYyIsIm9wdGlvbiIsImljb24iLCJpY29uSWQiLCJwcmVzc2VkIiwib25QcmVzcyIsImNvbnRlbnQiLCJjb250cm9scyIsImhhc0NvbnRlbnQiLCJ3aW5kb3dFbmdhZ2VtZW50VG9TZWFyY2hTcGVjIiwib25TdWJtaXQiLCJvblJlcGVhdExhc3QiLCJvbkFib3J0IiwicG9zc2libGVzIiwicG9zc2libGVFbmdhZ2VtZW50cyIsInBhbmVsVGFiSWNvbiIsInRhYklkIiwic2hlbGxUYWJJY29uIiwiaW5jbHVkZXMiLCJjYXRlZ29yeVRhYkljb24iLCJ0YWJzIiwiRmlyc3RJY29uIiwiQ2F0ZWdvcnlUYWJJY29uIiwiZmxhdHRlblBhbmVsVGFiTGVhdmVzIiwidGFiIiwicGFuZWxUYWJEZWZpbml0aW9uVG9Ob2RlIiwicGFuZWxVaUJ5S2V5Iiwib3JkZXIiLCJyZXNvbHZlUGFuZWxUYWJMYWJlbCIsImNoaWxkT3JkZXIiLCJ0cmVlIiwidWlOb2RlVG9UcmVlUGFuZWxDb25maWciLCJyZXNvbHZlQ2FudmFzQm9keUtleSIsIndpbmRvd0tpbmQiLCJib2R5S2V5Iiwid29ya2Zsb3ciLCJyZXNvbHZlVXRpbGl0aWVzIiwicmVnaXN0cnkiLCJ1dGlsaXRpZXMiLCJyZWZzIiwicmVzb2x2ZWQiLCJyZWYiLCJ1dGlsaXR5IiwiQ0hST01FX0tOT1dOX1JJQkJPTl9QQVJFTlRfQ0FURUdPUklFUyIsInJlc29sdmVVdGlsaXR5R3JvdXBMYWJlbCIsInV0aWxpdHlEZWZpbml0aW9uVG9TcGVjIiwiZ3JvdXBMYWJlbCIsImNhdGVnb3J5IiwidGFnU2V0QWN0aXZlVXRpbGl0eVdpbmRvdyIsIm5vZGVzIiwicmVzb2x2ZVV0aWxpdHlOb2RlcyIsImFjdGl2ZVV0aWxpdHlJZCIsIkVNUFRZX0FQUF9MQUJFTFNfT1ZFUkxBWSIsInNwYXduZWRXaW5kb3dDaHJvbWVGb3JLaW5kIiwiZW5nYWdlbWVudHNCeVdpbmRvd0lkIiwibWVhc3VyZXNCeVdpbmRvd0lkIiwibWVhc3VyZXMiLCJ1dGlsaXR5T3B0aW9ucyIsIndpbmRvd01lYXN1cmVzQ2hyb21lIiwicmVzb2x2ZWRFbmdhZ2VtZW50IiwiaXNUcmVlTm9kZSIsInRyZWVIYXNEcmFnIiwic2VjdGlvbnMiLCJzIiwiZHJhZ2dhYmxlIiwiZHJhZ0RhdGEiLCJkcmFnQW5kRHJvcENvbnRyb2xsZXIiLCJkcm9wQWN0aW9uIiwiZGVjbGFyYXRpdmVVaU5vZGVUb1RyZWVQYW5lbENvbmZpZyIsImVtcGhhc2l6ZWQiLCJlbXBoYXNpemUiLCJib2R5Q2hpbGRyZW4iLCJzZWN0aW9uTm9kZXMiLCJzZWN0aW9uIiwiZGVmYXVsdE9wZW4iLCJkZWNsYXJhdGl2ZVVpQ2hpbGRUb1RyZWVJdGVtcyIsInNvcnRhYmxlU2VjdGlvbnMiLCJpc1VpQ29udHJvbE5vZGUiLCJmYWxsYmFja0lkIiwiU2hlbGxUYWJJY29uIiwiaWNvbk5hbWUiLCJrZXkiLCJ0IiwiRlJBTUVXT1JLX1BBTkVMX1RBQl9MQUJFTF9LRVlTIiwib3ZlcmxheSIsImNocm9tZUtleSIsIndpbmRvd0tpbmRMYWJlbHMiLCJwYW5lbFRhYkxhYmVscyIsIm1vZGVMYWJlbHMiLCJhY3Rpb25MYWJlbHMiLCJ1dGlsaXR5TGFiZWxzIiwiZXhhbXBsZUxhYmVscyIsImFjdGlvbkFyZ0xhYmVscyIsImRpYWxvZ0xhYmVscyIsImludHJvZHVjdGlvbkxhYmVscyIsImdyb3VwTGFiZWxzIiwic3ludGhlc2l6ZUxvY2FsaXplZExhYmVsIiwibmF0aXZlIiwiZW4iLCJkZSIsInJldXNlIiwiYnlUZXJtaW5vbG9neSIsIk9iamVjdCIsInZhbHVlcyIsInJlc29sdmVBY3Rpb25BcmdEZWYiLCJkZWYiLCJzY29wZUlkIiwicmVzb2x2ZURpYWxvZ0RlZmluaXRpb24iLCJkaWFsb2ciLCJib2R5Iiwic3VibWl0TGFiZWwiLCJjYW5jZWxMYWJlbCIsInJlc29sdmVJbnRyb2R1Y3Rpb25EZWZpbml0aW9uIiwiaW50cm9kdWN0aW9uIiwic3RlcHMiLCJpbnRlcmFjdGlvbnMiLCJpbnRlcmFjdGlvbiIsIm9yZGVyZWQiLCJjYXB0dXJlVHV0b3JpYWxVaVNuYXBzaG90IiwiYWN0aXZlVXRpbGl0eUJ5V2luZG93SWQiLCJ1dGlsaXR5SWQiLCJhY3Rpb25QYW5lIiwiYWN0aXZlUGFuZWxUYWJCeUdyb3VwIiwicGFuZWxTdGF0ZSIsInBhbmVscyIsInZpc2libGUiLCJhY3RpdmVNb2RlSWQiLCJmb2N1c2VkV2luZG93SWQiLCJhY3RpdmVXaW5kb3dJZCIsImFjdGl2ZVRvb2xJZCIsInNlbGVjdGlvbkpzb24iLCJvcGVuRGlhbG9nSWQiLCJvdmVybGF5cyIsImRpYWxvZ0lkIiwiZXhwYW5kZWRUcmVlSWRzIiwidHJlZU9wZW5TdGF0ZXMiLCJvcGVuIiwiY29tbWFuZFBhbmVsT3BlbiIsInNlYXJjaE9wZW4iLCJhcHBseVR1dG9yaWFsVWlTbmFwc2hvdFRvU2hlbGwiLCJkaXNwYXRjaCIsInNuYXBzaG90IiwiY3R4IiwicGFuZWxQYXRjaGVzIiwiYXBwbHlUdXRvcmlhbFVpQ2hhbmdlVG9TaGVsbCIsImNoYW5nZSIsInRvb2xJZCIsInNlZWRBcmdzIiwiZXhwYW5kZWQiLCJzaGVsbFRlcm1pbm9sb2d5TGFiZWwiLCJpc0Nocm9tZUtub3duIiwiY3JlYXRlTGF0ZXN0QXN5bmNEaXNwYXRjaGVyIiwiZGlzcGF0Y2hWYWx1ZSIsInJ1bm5pbmciLCJxdWV1ZWQiLCJoYXNRdWV1ZWQiLCJkaXNwYXRjaExhdGVzdCIsImZpbmFsbHkiLCJjcmVhdGVEaXJlY3Rpb25hbEFzeW5jRGlzcGF0Y2hlciIsImFjdGl2ZSIsImRpc3BhdGNoTmV4dCIsInNoaWZ0IiwiYXQiLCJkaXJlY3Rpb24iLCJzaWduIiwibmV4dERpcmVjdGlvbiIsInNwbGljZSIsImNyZWF0ZVJldmVhbEN1dG9mZlN0b3JlIiwibGlzdGVuZXJzIiwiZ3JvdXBJZCIsImxpc3RlbmVyIiwic3Vic2NyaWJlIiwiZGVsZXRlIiwid29ybGRSZXZlYWxDdXRvZmZTdG9yZSIsIlBVWlpMRTNEX0ZJTExfUkVWRUFMX0dST1VQX0lEIiwicmVjb25jaWxlQ29tbWl0dGVkUmV2ZWFsQ3V0b2ZmcyIsInN0b3JlIiwiY29tbWl0dGVkUmVmIiwicmV2ZWFsQ3V0b2ZmcyIsImlzUmV2ZWFsQ3V0b2ZmSGlkZGVuIiwiaW5zdGFuY2UiLCJyZXZlYWxJbmRleCIsImN1dG9mZiIsImNyZWF0ZUluRmxpZ2h0U2tpcHBpbmdJbnRlcnZhbCIsInJ1biIsInNldEludGVydmFsRm4iLCJzZXRJbnRlcnZhbCIsImNsZWFySW50ZXJ2YWxGbiIsImNsZWFySW50ZXJ2YWwiLCJjYW5jZWxsZWQiLCJpbkZsaWdodCIsInRpY2siLCJ0aW1lciIsImNyZWF0ZUNvYWxlc2NpbmdBY3Rpb25EaXNwYXRjaGVyIiwiaXNFcXVhbCIsImEiLCJiIiwiaXMiLCJsYXN0U2VudCIsInJlZ2lzdGVyZWRQdXp6bGUzZEJydXNoTWVzaGVzIiwid2luZG93TWVhc3VyZVRyZWVDb250YWluc0lkIiwibWVhc3VyZSIsIndpbmRvd01lYXN1cmVVc2VzUHJvYmFiaWxpdHlSZWFkb3V0Iiwid2luZG93TWVhc3VyZVByb2JhYmlsaXR5UmVhZG91dCIsIldpbmRvd01lYXN1cmVTbGlkZXIiLCJfczIiLCJmb3JtYXREaXNwbGF5VmFsdWUiLCJyZXZlYWxHcm91cElkIiwicmV2ZWFsIiwicmVhZHkiLCJsb2FkaW5nIiwid2FpdGluZyIsIndpbmRvd01lYXN1cmVHcm91cEhlYWRlclNsaWRlciIsInNsaWRlck1lYXN1cmUiLCJ3aW5kb3dNZWFzdXJlU2VsZWN0Q29udHJvbCIsIml0ZW0iLCJ3aW5kb3dNZWFzdXJlVG9nZ2xlQ29udHJvbCIsIndpbmRvd01lYXN1cmVUb2dnbGVJY29uIiwid2luZG93TWVhc3VyZXNUb1RyZWVJdGVtcyIsInJldmVyc2VGb3JVcFBhbmVsIiwicmV2ZXJzZSIsIm1hcE1lYXN1cmUiLCJyZW5kZXJXaW5kb3dNZWFzdXJlIiwiaGVhZGVyU2xpZGVyIiwid2luZG93TWVhc3VyZXNPdmVybGF5IiwicmVuZGVyV2luZG93TWVhc3VyZXNUcmVlIiwiU2VsZWN0aW9uVXRpbGl0eU9wdGlvbnMiLCJfczMiLCJtZXRob2RMYWJlbCIsIm1vZGVMYWJlbCIsInJlY3RhbmdsZUxhYmVsIiwibGFzc29MYWJlbCIsInNlbGVjdGl2ZUxhYmVsIiwiYWRkaXRpdmVMYWJlbCIsInN1YnRyYWN0aXZlTGFiZWwiLCJpbnZlcnRpdmVMYWJlbCIsInNlbGVjdGlvbk1ldGhvZCIsInNlbGVjdGlvblN0b3JlIiwic2VsZWN0aW9uIiwic2VsZWN0aW9uTW9kZSIsInNldFNlbGVjdGlvbk1vZGUiLCJoYW5kbGVNb2RlQ2hhbmdlIiwibW9kZSIsImhhbmRsZU1ldGhvZENoYW5nZSIsIm1ldGhvZCIsInZhbCIsImdlbmVyYWwiLCJ0YWdnZWRPbkFjdGlvbiIsInV0aWxpdHlOb2RlVHJlZUNvbnRhaW5zSWQiLCJ0YXJnZXRJZCIsInV0aWxpdHlCYXJOb2RlIiwicmV2ZWFsVXRpbGl0eUlkIiwiY2F0ZWdvcmllcyIsImdyb3VwZWQiLCJyZW5kZXJTdGFnZWRBcmdDb250cm9sIiwiZXZlbnQiLCJ0YXJnZXQiLCJudW1lcmljIiwic2xpZGVyIiwidHVwbGUiLCJpc0FycmF5IiwiYXhlcyIsImF4aXMiLCJhY3Rpb25SZXF1aXJlc1N0YWdlZEZvcm0iLCJpc0VkaXRhYmxlRXZlbnRUYXJnZXQiLCJIVE1MRWxlbWVudCIsInRhZyIsInRhZ05hbWUiLCJpc0NvbnRlbnRFZGl0YWJsZSIsImNsb3Nlc3QiLCJrZXlib2FyZEV2ZW50TWF0Y2hlc0Nob3JkIiwiY2hvcmQiLCJwYXJ0cyIsInBhcnQiLCJuZWVkc0N0cmwiLCJuZWVkc1NoaWZ0IiwibmVlZHNBbHQiLCJoYXNDdHJsIiwiY3RybEtleSIsIm1ldGFLZXkiLCJzaGlmdEtleSIsImFsdEtleSIsInRvTG93ZXJDYXNlIiwicmVzb2x2ZUtleWJpbmRpbmdJbnRlbnQiLCJkZWZpbml0aW9uIiwiZXhwYW5kZWRBY3Rpb25JZCIsInN0YWdlZEFyZ3MiLCJlZmZlY3RpdmUiLCJhY3Rpb25JZCIsInJlc29sdmVVdGlsaXR5QWN0aXZhdGlvbiIsInJlcXVlc3RlZCIsImFjdGlvbkNhdGVnb3J5SWQiLCJhY3Rpb25DYXRlZ29yeUxhYmVsIiwiYWN0aW9uQ2F0ZWdvcmllcyIsImFjdGlvbnMiLCJzZWVuIiwiYnVpbGRBY3Rpb25DYXRlZ29yeVRyZWUiLCJzdGFnZWRBcmdzQnlLZXkiLCJvbkV4cGFuZGVkQ2hhbmdlIiwib25TdGFnZUFyZyIsIm9uUmVzZXRBcmdzIiwib25FeGVjdXRlIiwiZXhwYW5kZWRBY3Rpb24iLCJjYXRlZ29yeUFjdGlvbnMiLCJyb3dDbGFzc05hbWUiLCJjbGFzc05hbWUiLCJvbkNsaWNrIiwic3RhZ2VkIiwibWlzc2luZyIsIldpbmRvd0FjdGlvblBhbmUiLCJwcm9wcyIsIl9jMyIsIndpbmRvd0FjdGlvblBhbmVOb2RlIiwicmVzb2x2ZWRBY3Rpb25zIiwiYWN0aXZlVXRpbGl0eSIsImFsbG93c0FjdGlvbnNXaGlsZUFjdGl2ZSIsImV4cGFuZGVkQnlXaW5kb3dJZCIsImFyZ0lkIiwicmVzb2x2ZUNvbW1hbmRzIiwib3NDb21tYW5kcyIsImFjdGl2ZVBsdWdpbk1hbmlmZXN0IiwicmVzb2x2ZURlZmluaXRpb24iLCJzb3VyY2UiLCJjb21tYW5kcyIsImFjdGl2ZU1vZGUiLCJtb2RlcyIsIm1vZGVDb21tYW5kSWRzIiwibW9kZUlkIiwiQ0hST01FX0tOT1dOX0NPTU1BTkRfQ0FURUdPUklFUyIsInRpdGxlaXplQ29tbWFuZENhdGVnb3J5IiwicmVwbGFjZSIsImNvbW1hbmRDYXRlZ29yeUxhYmVsIiwiY29tbWFuZENhdGVnb3JpZXMiLCJzZWxlY3RDb21tYW5kQXJnIiwicmVxdWlyZWQiLCJkcml2ZXJEaXNwbGF5TGFiZWwiLCJkcml2ZXIiLCJidWlsZE9zQ29tbWFuZHMiLCJ0aGVtZUxpc3QiLCJ0ZXJtaW5vbG9naWVzIiwiaGFzSW50cm9kdWN0aW9uIiwibG9ja3MiLCJkcml2ZXJMaXN0IiwidHV0b3JpYWxzIiwidHV0b3JpYWxSZWNvcmRlckF2YWlsYWJsZSIsImxvY2tlZENvbW1hbmRJZHMiLCJhcHBlYXJhbmNlIiwidGhlbWVJZCIsImluUGFsZXR0ZSIsInR1dG9yaWFsIiwidGhlbWUiLCJjb21tYW5kIiwiZGlzcGF0Y2hPc0NvbW1hbmQiLCJkb2NrTGF5b3V0U3RvcmUiLCJkb2NrVWlTdGF0ZVN0b3JlIiwicmVzZXQiLCJDT01NQU5EX0NBVEVHT1JZX0lDT04iLCJidWlsZENvbW1hbmRDYXRlZ29yeVRyZWUiLCJleHBhbmRlZENvbW1hbmRJZCIsInN0YWdlZEFyZ3NCeUNvbW1hbmRJZCIsIm9uVG9nZ2xlRXhwYW5kZWQiLCJhcmdDYXJyeWluZ0NvbW1hbmRzIiwiYXV0b0V4cGFuZGVkU2luZ2xldG9uIiwiZWZmZWN0aXZlRXhwYW5kZWRJZCIsImxpc3RDb21tYW5kcyIsImFyZ0NhcnJ5aW5nIiwiYnVpbGRDb21tYW5kQ2F0ZWdvcnlUYWJzIiwicmVzb2x2ZWRDb21tYW5kcyIsImV4cGFuZGVkQ29tbWFuZElkUmVmIiwic3RhZ2VkQXJnc0J5Q29tbWFuZElkUmVmIiwib25Db21tYW5kIiwiY2F0ZWdvcnlDb21tYW5kcyIsInJlc29sdmVUcmVlIiwiZXhlY3V0ZUFyZ3MiLCJidWlsZFRvb2xUcmVlIiwidG9vbCIsImlzQWN0aXZlIiwiYnVpbGRUb29sVGFicyIsInRvb2xzIiwiYWN0aXZlVG9vbElkUmVmIiwidG9vbE1lYXN1cmVzQnlUb29sSWRSZWYiLCJ0b29sSWRGcm9tUGFuZWxUYWJJZCIsInN0YXJ0c1dpdGgiLCJ1aUpzb25EZWVwRXF1YWwiLCJhUmVjb3JkIiwiYlJlY29yZCIsImFLZXlzIiwia2V5cyIsImJLZXlzIiwicHJvdG90eXBlIiwiaGFzT3duUHJvcGVydHkiLCJjYWxsIiwicHJlc2VydmVKc29uSWRlbnRpdHkiLCJtZXJnZVJlY29yZFByZXNlcnZpbmdJZGVudGl0eSIsImNoYW5nZWQiLCJwcmVzZXJ2ZWQiLCJwYXRjaFdvcmxkM2RDaHJvbWVPbnRvTm9kZSIsInBhdGNoIiwid29ybGQzZCIsInZvcnRpY2VzSnNvbiIsInBhdGNoRG9jdW1lbnRUcmVlU2VsZWN0ZWRJZHMiLCJzZWxlY3RlZElkcyIsImhpZ2hsaWdodGVkSWRzIiwidWlSZWZyZXNoV2FudHNXaW5kb3ciLCJ3aW5kb3dCb2RpZXMiLCJ1aVJlZnJlc2hXYW50c1BhbmVsIiwicGFuZWxCb2RpZXMiLCJ1aVJlZnJlc2hXYW50c0ZsYWciLCJmbGFnIiwic2Vzc2lvbldpbmRvd0luc3RhbmNlcyIsImludHJvZHVjdGlvblRhcmdldHNXaW5kb3ciLCJ0YXJnZXRLaW5kSWQiLCJ0YXJnZXRTZWdtZW50IiwiYnVpbGRBY3RpdmVVdGlsaXR5QnlXaW5kb3dJZCIsImZyb21FbnRyaWVzIiwiYnVpbGRVaVJlZnJlc2hSZXF1ZXN0Iiwid2luZG93SW5zdGFuY2VzIiwicGFuZWxUYWJMZWF2ZXMiLCJjYWNoZSIsIndpbmRvd3MiLCJoYXNoIiwiZW5nYWdlbWVudHMiLCJsYWJlbHMiLCJhcHBseVVpUmVmcmVzaFNlY3Rpb25zVG9DYWNoZSIsInByZWZpeCIsImFwcGx5VWlSZWZyZXNoUmVzcG9uc2VUb0NhY2hlIiwiX2MiLCJfYzIiXSwiaWdub3JlTGlzdCI6W10sInNvdXJjZXMiOlsi8J+fpu+4j2NvbXBvbmVudC50c3giXSwic291cmNlc0NvbnRlbnQiOlsiLy8gI3JlZ2lvbiDwn6ey77iPSGVhZGVyXG4vLyDwn46o77iPIGZyYW1ld29yay9wcm9kdWN0cy9vcy9tb2R1bGVzL3JlbmRlcmVyL2VuZ2luZS9lbGVtZW50cy9TaGVsbEhlbHBlcnMvY29tcG9uZW50LnRzeFxuLyoqIEBlbW9qaSDwn6ew77iPIGBTaGVsbEhlbHBlcnNgIOKAlCBzaGFyZWQgcGx1bWJpbmcgYmVoaW5kIHRoZSBmcmFtZXdvcmsgT1Mgc2hlbGwgb3JjaGVzdHJhdG9yXG4gKiAoe0BsaW5rIC4uL1NoZWxsSG9zdH0pOiBhY3Rpb24taGlzdG9yeS9yZXNlcnZlZC1pZCBib29ra2VlcGluZywgcHJlc2VuY2UgaWRlbnRpdHksIFVJIGhpc3RvcnksXG4gKiBtZWRpYS1leHBvcnQgZG93bmxvYWQgaGVscGVycywgYHJlcXVlc3RNZWRpYUZyYW1lc2AncyBXZWJDb2RlY3MvYDx2aWRlbz5gIHRpZXJlZCBkZWNvZGUgcGlwZWxpbmUsXG4gKiB3aW5kb3ctbGF5b3V0LWNoYW5nZSBjbGFzc2lmaWNhdGlvbiwgdGhlIHV0aWxpdHktdHJlZS9jb21tYW5kL3Rvb2wgcmVnaXN0cmllcywgdGhlIHR1dG9yaWFsIFVJXG4gKiBicmlkZ2UsIHJldmVhbC1jdXRvZmYgc3RvcmUsIHRoZSB3aW5kb3cgYWN0aW9uIHBhbmUsIGFuZCB0aGUgcGx1Z2luIFVJLXJlZnJlc2ggY2FjaGUuIE5vIHNpbmdsZVxuICogZXhwb3J0ZWQgY29tcG9uZW50IGhlcmUg4oCUIGEgZ3JhYiBiYWcgb2YgdGhlIGZ1bmN0aW9ucy90eXBlcyBgU2hlbGxIb3N0YCBhbmQgc2libGluZyBlbGVtZW50cyBuZWVkLlxuICovXG4vLyAjZW5kcmVnaW9uIPCfp7LvuI9IZWFkZXJcblxuLy8gI3JlZ2lvbiDwn5SM77iPQWRhcHRlcnNcbmltcG9ydCBSZWFjdCwge1xuICB0eXBlIEtleWJvYXJkRXZlbnQsXG4gIHR5cGUgUmVhY3RFbGVtZW50LFxuICB0eXBlIFJlYWN0Tm9kZSxcbiAgdXNlQ2FsbGJhY2ssXG4gIHVzZUVmZmVjdCxcbiAgdXNlTWVtbyxcbiAgdXNlU3RhdGUsXG59IGZyb20gXCJyZWFjdFwiO1xuaW1wb3J0IHtcbiAgaXNJY29uTmFtZSxcbn0gZnJvbSBcIkBzZW1pby10ZWNoL2Fzc2V0c1wiO1xuaW1wb3J0IHtcbiAgdHlwZSBBY3Rpb25BcmdDb250cm9sLFxuICB0eXBlIEFjdGlvbkFyZ0RlZixcbiAgdHlwZSBBY3Rpb25EZWZpbml0aW9uLFxuICB0eXBlIEFjdGlvbkRlc2NyaXB0b3IsXG4gIHR5cGUgQXBwRGVmaW5pdGlvbixcbiAgdHlwZSBBcHBNb2RlRGVmaW5pdGlvbixcbiAgdHlwZSBBcHBQYW5lbFRhYkRlZmluaXRpb24sXG4gIHR5cGUgQXBwV2luZG93S2luZERlZmluaXRpb24sXG4gIHR5cGUgQ29tbWFuZERlZmluaXRpb24sXG4gIHR5cGUgRGVyaXZlZFV0aWxpdHlTcGVjLFxuICBkZXJpdmVVdGlsaXR5Tm9kZXMsXG4gIHR5cGUgRGlhbG9nRGVmaW5pdGlvbixcbiAgRG9ja0xheW91dFN0b3JlLFxuICBEb2NrVWlTdGF0ZVN0b3JlLFxuICBlZmZlY3RpdmVBY3Rpb25BcmdzLFxuICBGUkFNRVdPUktfUEFORUxfVEFCX0NBVEFMT0dVRV9JQ09OX0lELFxuICBGUkFNRVdPUktfUEFORUxfVEFCX0NBVEFMT0dVRV9JRCxcbiAgRlJBTUVXT1JLX1BBTkVMX1RBQl9ET0NVTUVOVF9JQ09OX0lELFxuICBGUkFNRVdPUktfUEFORUxfVEFCX0RPQ1VNRU5UX0lELFxuICBGUkFNRVdPUktfUEFORUxfVEFCX0hJU1RPUllfSUQsXG4gIEZSQU1FV09SS19QQU5FTF9UQUJfSU5TUEVDVElPTl9JQ09OX0lELFxuICBGUkFNRVdPUktfUEFORUxfVEFCX0lOU1BFQ1RJT05fSUQsXG4gIEZSQU1FV09SS19QQU5FTF9UQUJfUEFSQU1FVEVSU19JQ09OX0lELFxuICBGUkFNRVdPUktfUEFORUxfVEFCX1BBUkFNRVRFUlNfSUQsXG4gIHR5cGUgSG9zdEVmZmVjdCxcbiAgdHlwZSBJbnRyb2R1Y3Rpb25EZWZpbml0aW9uLFxuICB0eXBlIEludHJvZHVjdGlvblN0ZXBEZWZpbml0aW9uLFxuICB0eXBlIExvY2FsaXplZExhYmVsLFxuICBtaXNzaW5nUmVxdWlyZWRBcmdzLFxuICB0eXBlIFBhbmVsVGFiS2luZCxcbiAgcGFuZWxUYWJLaW5kSWQsXG4gIHBhcnRpdGlvbldpbmRvd01lYXN1cmVzLFxuICBwZW5kaW5nUGFuZWxVaU5vZGUsXG4gIHR5cGUgUGx1Z2luQXBwTGFiZWxzT3ZlcmxheSxcbiAgdHlwZSBQbHVnaW5VaVJlZnJlc2hSZXF1ZXN0LFxuICB0eXBlIFBsdWdpblVpUmVmcmVzaFJlc3BvbnNlLFxuICB0eXBlIFBsdWdpblVpUmVmcmVzaFNlY3Rpb25SZXNwb25zZSxcbiAgdHlwZSBQbHVnaW5WaWV3U3RhdGUsXG4gIFJFQ09SRF9UVVRPUklBTF9BQ1RJT05fSUQsXG4gIHJlc29sdmVQbHVnaW5Ib3N0Q29uZmlnLFxuICByZXNvbHZlVWlEaXJ0eVNjb3BlLFxuICByZXNvbHZlV2luZG93QWN0aW9ucyxcbiAgU0VUX0FDVElWRV9UT09MX0FDVElPTl9JRCxcbiAgU0VUX0FDVElWRV9VVElMSVRZX0FDVElPTl9JRCxcbiAgU0hFTExfTE9DQUxFUyxcbiAgU1RBUlRfSU5UUk9EVUNUSU9OX0FDVElPTl9JRCxcbiAgU1RBUlRfVFVUT1JJQUxfQUNUSU9OX0lELFxuICB0eXBlIFRvb2xEZWZpbml0aW9uLFxuICB0eXBlIFR1dG9yaWFsVWlDaGFuZ2UsXG4gIHR5cGUgVHV0b3JpYWxVaVNuYXBzaG90LFxuICB0eXBlIFVpQ29udHJvbE5vZGUsXG4gIHR5cGUgVWlEaXJ0eVNjb3BlLFxuICB0eXBlIFVpTm9kZSxcbiAgdHlwZSBVaVRyZWVOb2RlLFxuICB0eXBlIFV0aWxpdHlEZWZpbml0aW9uLFxuICB0eXBlIFV0aWxpdHlOb2RlLFxuICB0eXBlIFdpbmRvd0VuZ2FnZW1lbnQsXG4gIHR5cGUgV2luZG93RW5nYWdlbWVudENvbnRyb2wsXG4gIHR5cGUgV2luZG93TGF5b3V0LFxuICB0eXBlIFdpbmRvd0xheW91dEF4aXNOb2RlLFxuICB0eXBlIFdpbmRvd0xheW91dFN0YWNrTm9kZSxcbiAgdHlwZSBXaW5kb3dMYXlvdXRXaW5kb3dOb2RlLFxuICB0eXBlIFdpbmRvd01lYXN1cmUsXG59IGZyb20gXCJAc2VtaW8tdGVjaC9mcmFtZXdvcmstY29yZVwiO1xuaW1wb3J0IHtcbiAgZW5jb2RlQWN0aW9uV2lyZSxcbiAgcGFja1ZhbHVlRnJvbUJhc2U2NCxcbiAgcGFja1ZhbHVlVG9CYXNlNjQsXG59IGZyb20gXCJAc2VtaW8tdGVjaC9mcmFtZXdvcmstb3MtY29yZVwiO1xuaW1wb3J0IHtcbiAgZGVjb2RlV29ybGRQcm9qZWN0aW9uVGVtcGxhdGVJZCxcbn0gZnJvbSBcIkBzZW1pby10ZWNoL2luZmluaXRlLXdvcmxkLXIzZlwiO1xuaW1wb3J0IHtcbiAgdHlwZSBBbmNob3IsXG4gIEFOQ0hPUlMsXG4gIGJ1aWx0aW5VaURyaXZlcnMsXG4gIGNoaWxkRWxlbWVudElkLFxuICBDaHJvbWVBd2FyZVdpbmRvd1Njcm9sbFN1cmZhY2UsXG4gIGNsYXNzaWZ5SWNvblNlbGVjdG9yTW9kZSxcbiAgY3JlYXRlRXZlbldpbmRvd0xheW91dCxcbiAgZWxlbWVudElkU2VnbWVudCxcbiAgdHlwZSBFbGVtZW50c1N1cmZhY2VBcHBlYXJhbmNlLFxuICB0eXBlIEVuZ2FnZW1lbnRDb250cm9sLFxuICB0eXBlIEVuZ2FnZW1lbnRTcGVjLFxuICBJY29uLFxuICB0eXBlIEljb25OYW1lLFxuICBJY29uU2VsZWN0b3IsXG4gIElucHV0LFxuICB0eXBlIFBhbmVsVGFiTm9kZSxcbiAgcmVzb2x2ZVRyYW5zbGF0aW9uTGFiZWwsXG4gIFJpYmJvbkRpdmlkZXIsXG4gIHR5cGUgU2VhcmNoU3BlYyxcbiAgU2VsZWN0LFxuICBTZWxlY3RDb250ZW50LFxuICB0eXBlIFNlbGVjdGlvbk1lcmdlTW9kZSxcbiAgU2VsZWN0SXRlbSxcbiAgU2VsZWN0VHJpZ2dlcixcbiAgU2VsZWN0VmFsdWUsXG4gIHNldFVpTG9jYWxlLFxuICBzaW5nbGVUcmVlTGVhZixcbiAgU2xpZGVyLFxuICBzdGF0aWNUcmVlUGFuZWxEZWZpbml0aW9uLFxuICBUb2dnbGUsXG4gIFRvZ2dsZUdyb3VwLFxuICBUcmVlLFxuICBUcmVlQ2hlY2tib3gsXG4gIHR5cGUgVHJlZURhdGFJdGVtLFxuICB0eXBlIFRyZWVEYXRhU2VjdGlvbixcbiAgdHlwZSBUcmVlUGFuZWxDb25maWcsXG4gIFVJX1JJQkJPTl9QQVJFTlRfQ0FURUdPUklFUyxcbiAgVUlfVEVSTUlOT0xPR1lfTkFUSVZFLFxuICB0eXBlIFVpQ2hyb21lTGF5b3V0LFxuICB0eXBlIFVpQ2hyb21lVGVybWlub2xvZ3lJZCxcbiAgdWlEYXRhTGFiZWwsXG4gIHR5cGUgVWlEcml2ZXIsXG4gIHVpSTE4bixcbiAgdHlwZSBVaUxhYmVsLFxuICB0eXBlIFVpTG9jYWxlLFxuICB0eXBlIFVpUmliYm9uUGFyZW50Q2F0ZWdvcnksXG4gIHR5cGUgVWlUaGVtZSxcbiAgdHlwZSBVaVRyYW5zbGF0aW9uS2V5LFxuICB1c2VMYWJlbCxcbiAgdXNlU2hlbGxTY29wZSxcbiAgdHlwZSBXaW5kb3dMYXlvdXROb2RlLFxuICBXaW5kb3dNZWFzdXJlc1RyZWUsXG4gIFdpbmRvd01lYXN1cmVUcmVlR3JvdXAsXG4gIFdpbmRvd01lYXN1cmVUcmVlTGVhZixcbn0gZnJvbSBcIkBzZW1pby10ZWNoL3VpLXJlYWN0XCI7XG5pbXBvcnQge1xuICBkZWNsYXJhdGl2ZVRyZWVEcmFnQ29udHJvbGxlcixcbiAgSW50ZXJwcmV0ZWRVaU5vZGUsXG4gIGludGVycHJldFVpTm9kZSxcbiAgcmVuZGVyVWlDb250cm9sLFxuICB1aVRyZWVOb2RlVG9UcmVlUGFuZWxDb25maWcsXG4gIHdpcmVMYWJlbCxcbn0gZnJvbSBcIi4uL0ludGVycHJldGVyL/Cfn6bvuI9jb21wb25lbnQudHN4XCI7XG5pbXBvcnQge1xuICB0eXBlIEFjdGlvblBhbmVTdGF0ZSxcbiAgYWN0aW9uU3RhZ2VLZXksXG4gIHR5cGUgQWN0aXZlU2Vzc2lvbixcbiAgRU1QVFlfU0hFTExfTE9DS1MsXG4gIHR5cGUgRXh0cmFXaW5kb3dJbnN0YW5jZSxcbiAgdHlwZSBMb2FkZWRQcm9ncmFtU3RhdGUsXG4gIHR5cGUgUGx1Z2luTWFuaWZlc3QsXG4gIHR5cGUgUmVzb2x2ZWRTaGVsbExvY2tzLFxuICB0eXBlIFNoZWxsQWN0aW9uLFxuICBTaGVsbEZhdWx0Qm91bmRhcnksXG4gIHR5cGUgU2hlbGxTdGF0ZSxcbiAgdHlwZSBTcGFjZVBhbmVsU3RhdGUsXG4gIHR5cGUgU3BhY2VQcm9ncmFtRW50cnksXG4gIHR5cGUgU3Bhd25lZEFwcEVudHJ5LFxuICB0eXBlIFVJSGlzdG9yeSxcbiAgdHlwZSBWaWV3TW9kZWwsXG59IGZyb20gXCIuLi9TaGVsbC/wn5+m77iPY29tcG9uZW50LnRzeFwiO1xuaW1wb3J0IHtcbiAgcmVnaXN0ZXJQZW5kaW5nV29ybGRQcm9qZWN0aW9uLFxuICB0eXBlIFdvcmxkSW5zdGFuY2VSZWNvcmQsXG59IGZyb20gXCIuLi9Xb3JsZDNkSG9zdC/wn5+m77iPY29tcG9uZW50LnRzeFwiO1xuaW1wb3J0IHsgZ3JvdXBVdGlsaXR5Tm9kZXNCeUNhdGVnb3J5LCBVVElMSVRZX0NBVEVHT1JJRVMsIFV0aWxpdHlUcmVlIH0gZnJvbSBcIi4uL1V0aWxpdHlUcmVlL/Cfn6bvuI9jb21wb25lbnQudHN4XCI7XG5pbXBvcnQgeyBsb2FkUGx1Z2luTW9kdWxlLCB0eXBlIFBsdWdpbldhc21IYW5kbGUgfSBmcm9tIFwiLi4vUGx1Z2luUnVudGltZS/wn5+m77iPY29tcG9uZW50LnRzeFwiO1xuLy8gI2VuZHJlZ2lvbiDwn5SM77iPQWRhcHRlcnNcblxuLy8jcmVnaW9uIFNoZWxsSGVscGVyc1xuZXhwb3J0IGZ1bmN0aW9uIHN5bmNEb2N1bWVudElkKHNlc3Npb246IEFjdGl2ZVNlc3Npb24sIHBhbmVsOiBTcGFjZVBhbmVsU3RhdGUgfCBudWxsLCBzdHVkaW9Nb2RlOiBib29sZWFuKTogc3RyaW5nIHtcbiAgaWYgKHN0dWRpb01vZGUgJiYgcGFuZWw/LmFjdGl2ZVNwYXduZWRJZCkge1xuICAgIGNvbnN0IHNwYXduZWQgPSBwYW5lbC5zcGF3bmVkQXBwcy5maW5kKChlbnRyeSkgPT4gZW50cnkuaWQgPT09IHBhbmVsLmFjdGl2ZVNwYXduZWRJZCk7XG4gICAgaWYgKHNwYXduZWQpIHJldHVybiBgJHtzcGF3bmVkLnBsdWdpbklkfS0ke3NwYXduZWQuaW5zdGFuY2VJZH1gO1xuICB9XG4gIHJldHVybiBgJHtzZXNzaW9uLnBsdWdpbklkfS0ke3Nlc3Npb24uaW5zdGFuY2VJZH1gO1xufVxuXG4vKiogQGVtb2ppIOKGlCBTaGFyZWQgc3RhcnRpbmcgd2lkdGggZm9yIGV2ZXJ5IHBhbmVsIGFuY2hvciwgb25lIGNvbXBhY3Qgc3RlcCB3aWRlciB0aGFuIHRoZSBmb3JtZXIgMjgwcHggRG9jdW1lbnQgcGFuZWwuICovXG5leHBvcnQgY29uc3QgREVGQVVMVF9QQU5FTF9XSURUSF9QWCA9IDMwMDtcblxuLyoqIEBlbW9qaSDwn4yz77iPIFJvb3QgY2F0ZWdvcnkgaWQgZm9yIHRoZSBuZXN0ZWQgZG9jayB0YWIgdHJlZSDigJQgdGhlIHRvcCByb3cgb2Yge0BsaW5rIGRlZmF1bHREb2NrfSdzIGJvdHRvbS1sZWZ0IChEaXNwbGF5KSBhbmNob3IgdGFiczsgdG9wLWxlZnQgKFdvcmtiZW5jaCksIHRvcC1yaWdodCAoRGV0YWlscykgYW5kIGJvdHRvbS1yaWdodCAoU2V0dGluZ3MpIHJlbmRlciB0aGVpciB0YWJzIGZsYXQgaW5zdGVhZCBvZiB1bmRlciBhIGNhdGVnb3J5IGJyYW5jaC4gKi9cbmV4cG9ydCBjb25zdCBGUkFNRVdPUktfQ0FURUdPUllfRElTUExBWV9JRCA9IFwiZnJhbWV3b3JrLmNhdGVnb3J5LmRpc3BsYXlcIjtcbi8qKiBAZW1vamkg8J+Om++4jyBSb290IGNhdGVnb3J5IGlkIGJ1bmRsaW5nIGV2ZXJ5IGNvbW1hbmQtY2F0ZWdvcnkgbGVhZiB1bmRlciBvbmUgZXhwYW5kYWJsZSBDb21tYW5kIHRvZ2dsZSBvbiBib3R0b20tbWlkZGxlIChtaXJyb3JzIERpc3BsYXkgb24gYm90dG9tLWxlZnQpLiAqL1xuZXhwb3J0IGNvbnN0IEZSQU1FV09SS19DQVRFR09SWV9DT01NQU5EX0lEID0gXCJmcmFtZXdvcmsuY2F0ZWdvcnkuY29tbWFuZFwiO1xuLyoqIEBlbW9qaSDwn5ug77iPIFJvb3QgY2F0ZWdvcnkgaWQgYnVuZGxpbmcgZXZlcnkgbW9kZS1sZXZlbCB0b29sIGxlYWYgdW5kZXIgb25lIGV4cGFuZGFibGUgVG9vbCB0b2dnbGUgb25cbiAqIGJvdHRvbS1taWRkbGUsIG9yZGVyZWQgbGVmdCBvZiB0aGUgQ29tbWFuZCBicmFuY2ggKG1pcnJvcnMgQ29tbWFuZCdzIG93biBidW5kbGluZyBvbiB0aGUgc2FtZSBhbmNob3IpLiAqL1xuZXhwb3J0IGNvbnN0IEZSQU1FV09SS19DQVRFR09SWV9UT09MX0lEID0gXCJmcmFtZXdvcmsuY2F0ZWdvcnkudG9vbFwiO1xuXG4vKiogQGVtb2ppIPCfjpvvuI8gQ29ybmVyL3RvcC1taWRkbGUvYm90dG9tLW1pZGRsZSBhbmNob3JzIHBhcmsgdGhlaXIgKmZvbGRlZCogcm9vdCB0YWIgcm93IGluIG5hdmJhci9mb290ZXIgY2hyb21lICh2aWEge0BsaW5rIFBhbmVsQ2hyb21lVGFiQmFyfSk7IHdoaWxlIG9wZW4sIHRoZSBmbG9hdGluZyB7QGxpbmsgUGFuZWx9IGhvc3RzIHRoZSBmdWxsIHN0cmlwIG9uIGl0cyB7QGxpbmsgV2luZG93Q2hyb21lfS4gVGhlIHR3byBzaWRlLW1pZGRsZSBhbmNob3JzIGhhdmUgbm8gbmF2YmFyL2Zvb3RlciBzbG90LCBzbyB0aGV5J3JlIGFic2VudCBoZXJlIGFuZCBmYWxsIGJhY2sgdG8gYFwicGFuZWxcImAgKHNlZSB0aGUgYD8uLjpcInBhbmVsXCJgIHJlYWQgc2l0ZSksIGNhcnJ5aW5nIHRoZWlyIG93biB0YWIgYmFyIHdoZW4gZm9sZGVkIHRvby4gKi9cbmV4cG9ydCBjb25zdCBQQU5FTF9UQUJfQkFSX0hPU1RTOiBQYXJ0aWFsPFJlY29yZDxBbmNob3IsIFwibmF2YmFyXCIgfCBcImZvb3RlclwiPj4gPSB7XG4gIFwidG9wLWxlZnRcIjogXCJuYXZiYXJcIixcbiAgXCJ0b3AtbWlkZGxlXCI6IFwibmF2YmFyXCIsXG4gIFwidG9wLXJpZ2h0XCI6IFwibmF2YmFyXCIsXG4gIFwiYm90dG9tLWxlZnRcIjogXCJmb290ZXJcIixcbiAgXCJib3R0b20tbWlkZGxlXCI6IFwiZm9vdGVyXCIsXG4gIFwiYm90dG9tLXJpZ2h0XCI6IFwiZm9vdGVyXCIsXG59O1xuY29uc3QgQVBQX0RPQ1VNRU5UX1NFUEFSQVRPUiA9IFwiIMK3IFwiO1xuXG4vKiog8J+nre+4jyBTaGVsbC1vbmx5IGFjdGlvbiBpZCBgV29ybGQzZEhvc3RgJ3MgYFdvcmxkT3JiaXRHYXRlZC5vbk5hdmlnYXRpb25HZXN0dXJlc2AgZGlzcGF0Y2hlcyB0aHJvdWdoIHRoZVxuICogc3RhbmRhcmQgYG9uQWN0aW9uYCBmdW5uZWwgdG8gcmVwb3J0IGEgY29tcGxldGVkIHBhbi96b29tL29yYml0IGdlc3R1cmUg4oCUIGludGVyY2VwdGVkIGluIGBvbkFjdGlvbmBcbiAqIChuZXZlciBmb3J3YXJkZWQgdG8gdGhlIHByb2dyYW0pLCBhcmdzIGB7IHdpbmRvd0lkOiBzdHJpbmcsIGdlc3R1cmVzOiByZWFkb25seSBzdHJpbmdbXSB9YC4gKi9cbmV4cG9ydCBjb25zdCBOT1RFX1dPUkxEX05BVklHQVRJT05fQUNUSU9OX0lEID0gXCJub3RlV29ybGROYXZpZ2F0aW9uXCI7XG5cbi8qKiDwn6et77iPIEZyYW1ld29yay1pbmplY3RlZCBhY3Rpb24gaWQsIGRpc3BhdGNoZWQgdmlhIGBub3RlU2hlbGxDb21tYW5kYCAoc2VlIGBvbkFjdGlvbmAncyBjZW50cmFsIGZ1bm5lbCkgdG9cbiAqIGxvZyBhIHNoZWxsLWNocm9tZSBjb21tYW5kICh0aGVtZS9hcHBlYXJhbmNlL2xvY2FsZS9kcml2ZXIvbGF5b3V0IGNoYW5nZSwgZG9jayBkcmFnLCB3aW5kb3dcbiAqIHJlc2l6ZS9yZWFycmFuZ2UvYWN0aXZhdGUvY2xvc2Uvc3BsaXQsIHBhbmVsIHRvZ2dsZS90YWIpIGludG8gdGhlIHBsdWdpbidzIHNlc3Npb24tb25seSBjb21tYW5kLWhpc3RvcnlcbiAqIHBhbmVsIOKAlCBpbnRlcmNlcHRlZCBieSB0aGUgcGx1Z2luIEJFRk9SRSB0aGUgYXBwIGV2ZXIgc2VlcyBpdCwgYXJncyBgeyBjb21tYW5kSWQ6IHN0cmluZywgbGFiZWw6IHN0cmluZyxcbiAqIGRldGFpbD86IHVua25vd24gfWAuIFJvdXRlZCB0aHJvdWdoIHRoZSBleGFjdCBzYW1lIGBoYW5kbGVBY3Rpb25gIGRpc3BhdGNoIHBhdGggYXMgZXZlcnkgb3RoZXIgYWN0aW9uXG4gKiAodW5saWtlIHtAbGluayBOT1RFX1dPUkxEX05BVklHQVRJT05fQUNUSU9OX0lEfSwgd2hpY2ggaXMgZnVsbHkgc2hlbGwtaW50ZXJjZXB0ZWQgYW5kIG5ldmVyIGZvcndhcmRlZCkuICovXG5jb25zdCBOT1RFX1NIRUxMX0NPTU1BTkRfQUNUSU9OX0lEID0gXCJub3RlU2hlbGxDb21tYW5kXCI7XG5cbi8qKiDwn5uh77iPIEFjdGlvbiBpZHMgaW50ZXJjZXB0ZWQgYnkgYFZjc0RvY3VtZW50QXBwOjpkaXNwYXRjaF9hY3Rpb25gIGJlZm9yZSBgY29tbWFuZF9mcm9tX2FjdGlvbmAg4oCUIHVuZGVjbGFyZWRcbiAqIHN1cmZhY2UgdmVyYnMgKGUuZy4gVkZTIGBzZWxlY3RSb3dzYCBvbiBIb21lKSBtdXN0IG5vdCBiZSBmb3J3YXJkZWQgb3IgdGhleSBoYXJkLWVycm9yIHRoZSBicmlkZ2UuICovXG5leHBvcnQgY29uc3QgRlJBTUVXT1JLX1JFU0VSVkVEX0FDVElPTl9JRFM6IFJlYWRvbmx5U2V0PHN0cmluZz4gPSBuZXcgU2V0KFtcbiAgXCJ1bmRvXCIsXG4gIFwicmVkb1wiLFxuICBcImNvbW1pdENoZWNrcG9pbnRcIixcbiAgXCJjcmVhdGVBbHRlcm5hdGl2ZVwiLFxuICBcInN3aXRjaEFsdGVybmF0aXZlXCIsXG4gIFwiY2hlY2tvdXRDaGVja3BvaW50XCIsXG4gIFwiY29weVwiLFxuICBcImN1dFwiLFxuICBcInBhc3RlXCIsXG4gIFwicmV2ZXJ0VG9Db21tYW5kXCIsXG4gIFwic2V0SGlzdG9yeUNvbW1hbmRGaWx0ZXJcIixcbiAgTk9URV9TSEVMTF9DT01NQU5EX0FDVElPTl9JRCxcbiAgXCJyZWNvcmRUdXRvcmlhbFwiLFxuICBcInN0YXJ0SW50cm9kdWN0aW9uXCIsXG4gIFwic3RhcnRUdXRvcmlhbFwiLFxuICBcInNldEFjdGl2ZVV0aWxpdHlcIixcbiAgXCJzZXRBY3RpdmVUb29sXCIsXG4gIFwic3VnZ2VzdGlvbnNUaWNrXCIsXG4gIFwiZmlsbEJ1aWxkVGlja1wiLFxuXSk7XG5cbi8qKiDwn6et77iPIEJ1aWxkcyB0aGUgYG5vdGVTaGVsbENvbW1hbmRgIGFjdGlvbiBkZXNjcmlwdG9yIGBub3RlU2hlbGxDb21tYW5kYCAodGhlIGNvbXBvbmVudCBoZWxwZXIpIGRpc3BhdGNoZXNcbiAqIHRocm91Z2ggdGhlIHN0YW5kYXJkIGBvbkFjdGlvbmAgZnVubmVsIOKAlCBwdXJlIHNvIGl0J3MgdGVzdGFibGUgd2l0aG91dCBhIHNlc3Npb24vY29tcG9uZW50LiAqL1xuZXhwb3J0IGZ1bmN0aW9uIGJ1aWxkTm90ZVNoZWxsQ29tbWFuZEFjdGlvbihjb250cm9sbGVySWQ6IHN0cmluZywgY29tbWFuZElkOiBzdHJpbmcsIGxhYmVsOiBzdHJpbmcsIGRldGFpbD86IFJlY29yZDxzdHJpbmcsIHVua25vd24+KTogQWN0aW9uRGVzY3JpcHRvciB7XG4gIHJldHVybiB7IGNvbnRyb2xsZXJJZCwgYWN0aW9uOiBOT1RFX1NIRUxMX0NPTU1BTkRfQUNUSU9OX0lELCBhcmdzOiB7IGNvbW1hbmRJZCwgbGFiZWwsIC4uLihkZXRhaWwgPyB7IGRldGFpbCB9IDoge30pIH0gfTtcbn1cblxuLyoqIPCfp63vuI8gQWN0aW9uIGlkcyB0aGUgdHV0b3JpYWwgcmVjb3JkZXIgbmV2ZXIgY2FwdHVyZXMgKHNlZSBgb25BY3Rpb25gJ3MgcmVjb3JkZXIgdGFwKSDigJQgdGVsZW1ldHJ5L2Nocm9tZVxuICogbm9pc2UgYSB0dXRvcmlhbCByZXBsYXkgc2hvdWxkIG5ldmVyIGxpdGVyYWxseSByZXByb2R1Y2UsIG9yIGFjdGlvbnMgdGhlIGRpcmVjdG9yL3JlY29yZGVyIGl0c2VsZiBqdXN0XG4gKiBkaXNwYXRjaGVkLiBFeHBvcnRlZCBzbyBpdCdzIGluZGVwZW5kZW50bHkgdGVzdGFibGUuICovXG5leHBvcnQgY29uc3QgVFVUT1JJQUxfUkVDT1JESU5HX0VYQ0xVREVEX0FDVElPTl9JRFM6IFJlYWRvbmx5U2V0PHN0cmluZz4gPSBuZXcgU2V0KFtOT1RFX1dPUkxEX05BVklHQVRJT05fQUNUSU9OX0lELCBOT1RFX1NIRUxMX0NPTU1BTkRfQUNUSU9OX0lELCBTVEFSVF9JTlRST0RVQ1RJT05fQUNUSU9OX0lELCBTVEFSVF9UVVRPUklBTF9BQ1RJT05fSUQsIFJFQ09SRF9UVVRPUklBTF9BQ1RJT05fSURdKTtcblxuZXhwb3J0IGNvbnN0IFBSRVNFTkNFX0NMSUVOVF9TVE9SQUdFX0tFWSA9IFwic2VtaW8ucHJlc2VuY2UuY2xpZW50XCI7XG5leHBvcnQgY29uc3QgUFJFU0VOQ0VfSEVBUlRCRUFUX0lOVEVSVkFMX01TID0gNTAwMDtcblxuZnVuY3Rpb24gcHJlc2VuY2VJZGVudGl0eVBhY2tCYXNlNjQoaWRlbnRpdHk6IHsgcmVhZG9ubHkgY2xpZW50SWQ6IHN0cmluZzsgcmVhZG9ubHkgbmFtZTogc3RyaW5nIH0pOiBzdHJpbmcge1xuICByZXR1cm4gcGFja1ZhbHVlVG9CYXNlNjQoaWRlbnRpdHkpO1xufVxuXG5mdW5jdGlvbiBwcmVzZW5jZUlkZW50aXR5RnJvbVBhY2tCYXNlNjQoZW5jb2RlZDogc3RyaW5nKTogeyByZWFkb25seSBjbGllbnRJZDogc3RyaW5nOyByZWFkb25seSBuYW1lOiBzdHJpbmcgfSB8IG51bGwge1xuICB0cnkge1xuICAgIGNvbnN0IGRlY29kZWQgPSBwYWNrVmFsdWVGcm9tQmFzZTY0KGVuY29kZWQpIGFzIHsgcmVhZG9ubHkgY2xpZW50SWQ/OiBzdHJpbmc7IHJlYWRvbmx5IG5hbWU/OiBzdHJpbmcgfTtcbiAgICBpZiAoZGVjb2RlZC5jbGllbnRJZCAmJiBkZWNvZGVkLm5hbWUpIHJldHVybiB7IGNsaWVudElkOiBkZWNvZGVkLmNsaWVudElkLCBuYW1lOiBkZWNvZGVkLm5hbWUgfTtcbiAgfSBjYXRjaCB7XG4gICAgcmV0dXJuIG51bGw7XG4gIH1cbiAgcmV0dXJuIG51bGw7XG59XG5cbmV4cG9ydCBmdW5jdGlvbiBwcmVzZW5jZUNsaWVudElkZW50aXR5KGVwaGVtZXJhbCA9IGZhbHNlKTogeyByZWFkb25seSBjbGllbnRJZDogc3RyaW5nOyByZWFkb25seSBuYW1lOiBzdHJpbmcgfSB7XG4gIGlmICh0eXBlb2Ygd2luZG93ID09PSBcInVuZGVmaW5lZFwiKSByZXR1cm4geyBjbGllbnRJZDogXCJzZXJ2ZXJcIiwgbmFtZTogXCJTZXJ2ZXJcIiB9O1xuICBpZiAoIWVwaGVtZXJhbCkge1xuICAgIGNvbnN0IHN0b3JlZCA9IHdpbmRvdy5zZXNzaW9uU3RvcmFnZS5nZXRJdGVtKFBSRVNFTkNFX0NMSUVOVF9TVE9SQUdFX0tFWSk7XG4gICAgaWYgKHN0b3JlZCkge1xuICAgICAgY29uc3QgcGFyc2VkID0gcHJlc2VuY2VJZGVudGl0eUZyb21QYWNrQmFzZTY0KHN0b3JlZCk7XG4gICAgICBpZiAocGFyc2VkKSByZXR1cm4gcGFyc2VkO1xuICAgIH1cbiAgfVxuICBjb25zdCBjbGllbnRJZCA9IGBjbGllbnQtJHtNYXRoLnJhbmRvbSgpLnRvU3RyaW5nKDM2KS5zbGljZSgyLCAxMCl9YDtcbiAgY29uc3QgaWRlbnRpdHkgPSB7IGNsaWVudElkLCBuYW1lOiBgR3Vlc3QgJHtjbGllbnRJZC5zbGljZSgtNCkudG9VcHBlckNhc2UoKX1gIH07XG4gIGlmICghZXBoZW1lcmFsKSB3aW5kb3cuc2Vzc2lvblN0b3JhZ2Uuc2V0SXRlbShQUkVTRU5DRV9DTElFTlRfU1RPUkFHRV9LRVksIHByZXNlbmNlSWRlbnRpdHlQYWNrQmFzZTY0KGlkZW50aXR5KSk7XG4gIHJldHVybiBpZGVudGl0eTtcbn1cblxuZnVuY3Rpb24gcmVhZEJyb3dzZXJVcmkoKTogc3RyaW5nIHtcbiAgaWYgKHR5cGVvZiB3aW5kb3cgPT09IFwidW5kZWZpbmVkXCIpIHJldHVybiBcIi9cIjtcbiAgcmV0dXJuIGAke3dpbmRvdy5sb2NhdGlvbi5wYXRobmFtZX0ke3dpbmRvdy5sb2NhdGlvbi5zZWFyY2h9YCB8fCBcIi9cIjtcbn1cblxuZXhwb3J0IGZ1bmN0aW9uIHVzZVVJSGlzdG9yeShpbml0aWFsVXJpID0gXCIvXCIsIHN5bmNCcm93c2VyID0gZmFsc2UpIHtcbiAgY29uc3QgW2hpc3RvcnksIHNldEhpc3RvcnldID0gdXNlU3RhdGU8VUlIaXN0b3J5PigoKSA9PiAoe1xuICAgIGVudHJpZXM6IFt7IHVyaTogc3luY0Jyb3dzZXIgPyByZWFkQnJvd3NlclVyaSgpIDogaW5pdGlhbFVyaSB9XSxcbiAgICBpbmRleDogMCxcbiAgfSkpO1xuICBjb25zdCB1cmkgPSBoaXN0b3J5LmVudHJpZXNbaGlzdG9yeS5pbmRleF0/LnVyaSA/PyBpbml0aWFsVXJpO1xuICBjb25zdCBjYW5Hb0JhY2sgPSBoaXN0b3J5LmluZGV4ID4gMDtcbiAgY29uc3QgY2FuR29Gb3J3YXJkID0gaGlzdG9yeS5pbmRleCA8IGhpc3RvcnkuZW50cmllcy5sZW5ndGggLSAxO1xuICBjb25zdCBzZWdtZW50cyA9IHVyaS5zcGxpdChcIi9cIikuZmlsdGVyKEJvb2xlYW4pO1xuICBjb25zdCBjYW5Hb1VwID0gc2VnbWVudHMubGVuZ3RoID4gMDtcbiAgY29uc3QgcGFyZW50VXJpID0gY2FuR29VcCA/IGAvJHtzZWdtZW50cy5zbGljZSgwLCAtMSkuam9pbihcIi9cIil9YCA6IG51bGw7XG5cbiAgY29uc3QgZ29CYWNrID0gdXNlQ2FsbGJhY2soKCkgPT4ge1xuICAgIHNldEhpc3RvcnkoKHByZXYpID0+IChwcmV2LmluZGV4ID4gMCA/IHsgLi4ucHJldiwgaW5kZXg6IHByZXYuaW5kZXggLSAxIH0gOiBwcmV2KSk7XG4gIH0sIFtdKTtcbiAgY29uc3QgZ29Gb3J3YXJkID0gdXNlQ2FsbGJhY2soKCkgPT4ge1xuICAgIHNldEhpc3RvcnkoKHByZXYpID0+IChwcmV2LmluZGV4IDwgcHJldi5lbnRyaWVzLmxlbmd0aCAtIDEgPyB7IC4uLnByZXYsIGluZGV4OiBwcmV2LmluZGV4ICsgMSB9IDogcHJldikpO1xuICB9LCBbXSk7XG4gIGNvbnN0IGdvVXAgPSB1c2VDYWxsYmFjaygoKSA9PiB7XG4gICAgaWYgKCFjYW5Hb1VwIHx8IHBhcmVudFVyaSA9PT0gbnVsbCkgcmV0dXJuO1xuICAgIHNldEhpc3RvcnkoKHByZXYpID0+IHtcbiAgICAgIGNvbnN0IG5ld0VudHJpZXMgPSBwcmV2LmVudHJpZXMuc2xpY2UoMCwgcHJldi5pbmRleCArIDEpO1xuICAgICAgcmV0dXJuIHsgZW50cmllczogWy4uLm5ld0VudHJpZXMsIHsgdXJpOiBwYXJlbnRVcmkgfV0sIGluZGV4OiBuZXdFbnRyaWVzLmxlbmd0aCB9O1xuICAgIH0pO1xuICB9LCBbY2FuR29VcCwgcGFyZW50VXJpXSk7XG4gIGNvbnN0IG5hdmlnYXRlID0gdXNlQ2FsbGJhY2soKHRhcmdldFVyaTogc3RyaW5nKSA9PiB7XG4gICAgc2V0SGlzdG9yeSgocHJldikgPT4ge1xuICAgICAgY29uc3QgZXhpc3RpbmdJbmRleCA9IHByZXYuZW50cmllcy5maW5kSW5kZXgoKGVudHJ5KSA9PiBlbnRyeS51cmkgPT09IHRhcmdldFVyaSk7XG4gICAgICBpZiAoZXhpc3RpbmdJbmRleCA+PSAwKSByZXR1cm4geyAuLi5wcmV2LCBpbmRleDogZXhpc3RpbmdJbmRleCB9O1xuICAgICAgY29uc3QgbmV3RW50cmllcyA9IHByZXYuZW50cmllcy5zbGljZSgwLCBwcmV2LmluZGV4ICsgMSk7XG4gICAgICByZXR1cm4geyBlbnRyaWVzOiBbLi4ubmV3RW50cmllcywgeyB1cmk6IHRhcmdldFVyaSB9XSwgaW5kZXg6IG5ld0VudHJpZXMubGVuZ3RoIH07XG4gICAgfSk7XG4gIH0sIFtdKTtcbiAgY29uc3Qgc3luY1VyaSA9IHVzZUNhbGxiYWNrKCh0YXJnZXRVcmk6IHN0cmluZykgPT4ge1xuICAgIHNldEhpc3RvcnkoKHByZXYpID0+IHtcbiAgICAgIGNvbnN0IGV4aXN0aW5nSW5kZXggPSBwcmV2LmVudHJpZXMuZmluZEluZGV4KChlbnRyeSkgPT4gZW50cnkudXJpID09PSB0YXJnZXRVcmkpO1xuICAgICAgaWYgKGV4aXN0aW5nSW5kZXggPj0gMCkgcmV0dXJuIHsgLi4ucHJldiwgaW5kZXg6IGV4aXN0aW5nSW5kZXggfTtcbiAgICAgIGNvbnN0IG5ld0VudHJpZXMgPSBwcmV2LmVudHJpZXMuc2xpY2UoMCwgcHJldi5pbmRleCArIDEpO1xuICAgICAgcmV0dXJuIHsgZW50cmllczogWy4uLm5ld0VudHJpZXMsIHsgdXJpOiB0YXJnZXRVcmkgfV0sIGluZGV4OiBuZXdFbnRyaWVzLmxlbmd0aCB9O1xuICAgIH0pO1xuICB9LCBbXSk7XG5cbiAgdXNlRWZmZWN0KCgpID0+IHtcbiAgICBpZiAoIXN5bmNCcm93c2VyIHx8IHR5cGVvZiB3aW5kb3cgPT09IFwidW5kZWZpbmVkXCIpIHJldHVybjtcbiAgICBjb25zdCBjdXJyZW50ID0gYCR7d2luZG93LmxvY2F0aW9uLnBhdGhuYW1lfSR7d2luZG93LmxvY2F0aW9uLnNlYXJjaH1gO1xuICAgIGlmIChjdXJyZW50ICE9PSB1cmkpIHdpbmRvdy5oaXN0b3J5LnB1c2hTdGF0ZShudWxsLCBcIlwiLCB1cmkpO1xuICB9LCBbc3luY0Jyb3dzZXIsIHVyaV0pO1xuXG4gIHVzZUVmZmVjdCgoKSA9PiB7XG4gICAgaWYgKCFzeW5jQnJvd3NlciB8fCB0eXBlb2Ygd2luZG93ID09PSBcInVuZGVmaW5lZFwiKSByZXR1cm47XG4gICAgY29uc3Qgb25Qb3BTdGF0ZSA9ICgpID0+IHN5bmNVcmkocmVhZEJyb3dzZXJVcmkoKSk7XG4gICAgd2luZG93LmFkZEV2ZW50TGlzdGVuZXIoXCJwb3BzdGF0ZVwiLCBvblBvcFN0YXRlKTtcbiAgICByZXR1cm4gKCkgPT4gd2luZG93LnJlbW92ZUV2ZW50TGlzdGVuZXIoXCJwb3BzdGF0ZVwiLCBvblBvcFN0YXRlKTtcbiAgfSwgW3N5bmNCcm93c2VyLCBzeW5jVXJpXSk7XG5cbiAgcmV0dXJuIHsgdXJpLCBjYW5Hb0JhY2ssIGNhbkdvRm9yd2FyZCwgY2FuR29VcCwgcGFyZW50VXJpLCBnb0JhY2ssIGdvRm9yd2FyZCwgZ29VcCwgbmF2aWdhdGUsIHN5bmNVcmkgfTtcbn1cblxuZXhwb3J0IGZ1bmN0aW9uIGRvd25sb2FkTWVkaWFFeHBvcnQoZmlsZW5hbWU6IHN0cmluZywgbWltZVR5cGU6IHN0cmluZywgZGF0YTogc3RyaW5nLCBlbmNvZGluZz86IHN0cmluZyk6IHZvaWQge1xuICBpZiAodHlwZW9mIGRvY3VtZW50ID09PSBcInVuZGVmaW5lZFwiKSByZXR1cm47XG4gIGNvbnN0IHBheWxvYWQgPSBlbmNvZGluZyA9PT0gXCJiYXNlNjRcIiA/IFVpbnQ4QXJyYXkuZnJvbShhdG9iKGRhdGEpLCAoY2hhcikgPT4gY2hhci5jaGFyQ29kZUF0KDApKSA6IGRhdGE7XG4gIGNvbnN0IGJsb2IgPSBuZXcgQmxvYihbcGF5bG9hZF0sIHsgdHlwZTogbWltZVR5cGUgfSk7XG4gIGNvbnN0IHVybCA9IFVSTC5jcmVhdGVPYmplY3RVUkwoYmxvYik7XG4gIGNvbnN0IGFuY2hvciA9IGRvY3VtZW50LmNyZWF0ZUVsZW1lbnQoXCJhXCIpO1xuICBhbmNob3IuaHJlZiA9IHVybDtcbiAgYW5jaG9yLmRvd25sb2FkID0gZmlsZW5hbWU7XG4gIGFuY2hvci5jbGljaygpO1xuICBVUkwucmV2b2tlT2JqZWN0VVJMKHVybCk7XG59XG5cbmV4cG9ydCBmdW5jdGlvbiBkb3dubG9hZERhdGFVcmwoZmlsZW5hbWU6IHN0cmluZywgZGF0YVVybDogc3RyaW5nKTogdm9pZCB7XG4gIGlmICh0eXBlb2YgZG9jdW1lbnQgPT09IFwidW5kZWZpbmVkXCIpIHJldHVybjtcbiAgY29uc3QgYW5jaG9yID0gZG9jdW1lbnQuY3JlYXRlRWxlbWVudChcImFcIik7XG4gIGFuY2hvci5ocmVmID0gZGF0YVVybDtcbiAgYW5jaG9yLmRvd25sb2FkID0gZmlsZW5hbWU7XG4gIGFuY2hvci5jbGljaygpO1xufVxuXG4vKiog8J+TpO+4jyBPcGVucyB0aGUgbmF0aXZlIGZpbGUgcGlja2VyLiBSZXNvbHZlcyB3aXRoIG9uZSBlbnRyeSBwZXIgc2VsZWN0ZWQgZmlsZSwgaW4gc2VsZWN0aW9uIG9yZGVyIOKAlFxuICogYWx3YXlzIGFuIGFycmF5IChlbXB0eSBvbiBjYW5jZWwpIHNvIHNpbmdsZS1maWxlIGNhbGxlcnMganVzdCByZWFkIGBbMF1gIGFuZCBgbXVsdGlwbGVgIGNhbGxlcnMgY2FuXG4gKiBmYW4gb3V0IG92ZXIgdGhlIHdob2xlIGxpc3Q7IHNpbmdsZS1maWxlIGJlaGF2aW9yIChvbmUgYDxpbnB1dD5gLCBvbmUgcmVzb2x2ZWQgZW50cnkpIGlzIHVuY2hhbmdlZFxuICogd2hlbiBgbXVsdGlwbGVgIGlzIGZhbHNlL2Fic2VudC4gKi9cbmV4cG9ydCBmdW5jdGlvbiByZXF1ZXN0RmlsZU9wZW4oYWNjZXB0OiBzdHJpbmcsIHJlYWRBcz86IHN0cmluZywgbXVsdGlwbGU/OiBib29sZWFuKTogUHJvbWlzZTxyZWFkb25seSB7IGNvbnRlbnRzOiBzdHJpbmc7IG5hbWU6IHN0cmluZyB9W10+IHtcbiAgaWYgKHR5cGVvZiBkb2N1bWVudCA9PT0gXCJ1bmRlZmluZWRcIikgcmV0dXJuIFByb21pc2UucmVzb2x2ZShbXSk7XG4gIHJldHVybiBuZXcgUHJvbWlzZSgocmVzb2x2ZSkgPT4ge1xuICAgIGNvbnN0IGlucHV0ID0gZG9jdW1lbnQuY3JlYXRlRWxlbWVudChcImlucHV0XCIpO1xuICAgIGlucHV0LnR5cGUgPSBcImZpbGVcIjtcbiAgICBpbnB1dC5hY2NlcHQgPSBhY2NlcHQ7XG4gICAgaWYgKG11bHRpcGxlKSBpbnB1dC5tdWx0aXBsZSA9IHRydWU7XG4gICAgaW5wdXQub25jaGFuZ2UgPSBhc3luYyAoKSA9PiB7XG4gICAgICBjb25zdCBmaWxlcyA9IGlucHV0LmZpbGVzID8gQXJyYXkuZnJvbShpbnB1dC5maWxlcykgOiBbXTtcbiAgICAgIGlmIChmaWxlcy5sZW5ndGggPT09IDApIHtcbiAgICAgICAgcmVzb2x2ZShbXSk7XG4gICAgICAgIHJldHVybjtcbiAgICAgIH1cbiAgICAgIGNvbnN0IG9wZW5lZDogeyBjb250ZW50czogc3RyaW5nOyBuYW1lOiBzdHJpbmcgfVtdID0gW107XG4gICAgICBmb3IgKGNvbnN0IGZpbGUgb2YgZmlsZXMpIHtcbiAgICAgICAgaWYgKHJlYWRBcyA9PT0gXCJkYXRhVXJsXCIpIHtcbiAgICAgICAgICBjb25zdCBjb250ZW50cyA9IGF3YWl0IG5ldyBQcm9taXNlPHN0cmluZyB8IG51bGw+KChyZXNvbHZlRmlsZSkgPT4ge1xuICAgICAgICAgICAgY29uc3QgcmVhZGVyID0gbmV3IEZpbGVSZWFkZXIoKTtcbiAgICAgICAgICAgIHJlYWRlci5vbmxvYWQgPSAoKSA9PiByZXNvbHZlRmlsZSh0eXBlb2YgcmVhZGVyLnJlc3VsdCA9PT0gXCJzdHJpbmdcIiA/IHJlYWRlci5yZXN1bHQgOiBudWxsKTtcbiAgICAgICAgICAgIHJlYWRlci5vbmVycm9yID0gKCkgPT4gcmVzb2x2ZUZpbGUobnVsbCk7XG4gICAgICAgICAgICByZWFkZXIucmVhZEFzRGF0YVVSTChmaWxlKTtcbiAgICAgICAgICB9KTtcbiAgICAgICAgICBpZiAoY29udGVudHMgIT09IG51bGwpIG9wZW5lZC5wdXNoKHsgY29udGVudHMsIG5hbWU6IGZpbGUubmFtZSB9KTtcbiAgICAgICAgICBjb250aW51ZTtcbiAgICAgICAgfVxuICAgICAgICBvcGVuZWQucHVzaCh7IGNvbnRlbnRzOiBhd2FpdCBmaWxlLnRleHQoKSwgbmFtZTogZmlsZS5uYW1lIH0pO1xuICAgICAgfVxuICAgICAgcmVzb2x2ZShvcGVuZWQpO1xuICAgIH07XG4gICAgaW5wdXQuY2xpY2soKTtcbiAgfSk7XG59XG5cbi8qKiDwn5SB77iPIFRoZSBvbmUtYWN0aW9uLWF0LWEtdGltZSBjYWxsYmFjayBzaGFyZWQgYnkgdGhlIGByZXF1ZXN0RmlsZU9wZW5gL2BkaXNwYXRjaEFjdGlvbmAvXG4gKiBgcmVxdWVzdE1lZGlhRnJhbWVzYCBgYXBwbHlIb3N0RWZmZWN0c2AgYnJhbmNoZXM6IGRpc3BhdGNoZXMgYGFjdGlvbmAgYWdhaW5zdCB0aGUgZW1pdHRpbmcgcHJvZ3JhbVxuICogaW5zdGFuY2UgYW5kIGZlZWRzIGl0cyBvd24gYHJlcXVlc3RlZEVmZmVjdHNgIGJhY2sgdGhyb3VnaCBgYXBwbHlIb3N0RWZmZWN0c2AgcmVjdXJzaXZlbHkuICovXG50eXBlIEVmZmVjdERpc3BhdGNoT25lID0gKGFjdGlvbjogc3RyaW5nLCBhcmdzPzogUmVjb3JkPHN0cmluZywgdW5rbm93bj4pID0+IFByb21pc2U8dm9pZD47XG5cbi8qKiDwn5SB77iPIEJ1aWxkcyBhbiB7QGxpbmsgRWZmZWN0RGlzcGF0Y2hPbmV9IGJvdW5kIHRvIG9uZSBwbHVnaW4gaW5zdGFuY2UgKyBgYXBwbHlIb3N0RWZmZWN0c2AgY2xvc3VyZSDigJRcbiAqIGV4dHJhY3RlZCBzbyB0aGUgRDMvRDIvRDUgZmFuLW91dCBsb29wcyBiZWxvdyBhcmUgcGxhaW4gZnVuY3Rpb25zIHRlc3RhYmxlIHdpdGhvdXQgUmVhY3QvcGx1Z2luXG4gKiB3aXJpbmcsIHdoaWxlIHByb2R1Y3Rpb24gY2FsbGVycyBnZXQgdGhlIGV4YWN0IHNhbWUgYGhhbmRsZUFjdGlvbmAgKyByZWN1cnNpdmUtZWZmZWN0cyBiZWhhdmlvci4gKi9cbmV4cG9ydCBmdW5jdGlvbiBtYWtlRWZmZWN0RGlzcGF0Y2hPbmUoXG4gIHBsdWdpbkVudHJ5OiBMb2FkZWRQcm9ncmFtU3RhdGUsXG4gIGJhc2VTZXNzaW9uOiBBY3RpdmVTZXNzaW9uLFxuICBhcHBseUVmZmVjdHM6IChlZmZlY3RzOiByZWFkb25seSBIb3N0RWZmZWN0W10sIGJhc2VTZXNzaW9uOiBBY3RpdmVTZXNzaW9uLCB1aVNjb3BlPzogVWlEaXJ0eVNjb3BlKSA9PiBQcm9taXNlPHZvaWQ+LFxuKTogRWZmZWN0RGlzcGF0Y2hPbmUge1xuICByZXR1cm4gYXN5bmMgKGFjdGlvbiwgYXJncykgPT4ge1xuICAgIGNvbnN0IHJlc3BvbnNlID0gYXdhaXQgcGx1Z2luRW50cnkuaGFuZGxlLmhhbmRsZUFjdGlvbihcbiAgICAgIGJhc2VTZXNzaW9uLmluc3RhbmNlSWQsXG4gICAgICBlbmNvZGVBY3Rpb25XaXJlKHsgY29udHJvbGxlcklkOiBiYXNlU2Vzc2lvbi5hcHAuY29udHJvbGxlcklkLCBhY3Rpb24sIGFyZ3MgfSksXG4gICAgICBiYXNlU2Vzc2lvbi52aWV3U3RhdGUsXG4gICAgKTtcbiAgICBhd2FpdCBhcHBseUVmZmVjdHMocmVzcG9uc2UucmVxdWVzdGVkRWZmZWN0cyA/PyBbXSwgYmFzZVNlc3Npb24sIHJlc29sdmVVaURpcnR5U2NvcGUocmVzcG9uc2UudWlTY29wZSkpO1xuICB9O1xufVxuXG4vKiog8J+TpO+4jyBEMyBmYW4tb3V0OiBvbmUge0BsaW5rIEVmZmVjdERpc3BhdGNoT25lfSBjYWxsIHBlciBvcGVuZWQgZmlsZSDigJQgc2luZ2xlLWZpbGUgYmVoYXZpb3IgKGBtdWx0aXBsZWBcbiAqIGFic2VudC9mYWxzZSwgZXhhY3RseSBvbmUgY2FsbCwgcGxhaW4gYHtwYXlsb2FkLCBuYW1lfWApIGlzIGJ5dGUtZm9yLWJ5dGUgd2hhdCB0aGlzIGxvb3AgYWx3YXlzIGRpZFxuICogYmVmb3JlIGBtdWx0aXBsZWAgZXhpc3RlZCwgc2luY2UgaXQncyBqdXN0IGEgb25lLWVudHJ5IGBvcGVuZWRgIGFycmF5IHRocm91Z2ggdGhlIHNhbWUgcGF0aC4gKi9cbmV4cG9ydCBhc3luYyBmdW5jdGlvbiBkaXNwYXRjaE9wZW5lZEZpbGVzKFxuICBvcGVuZWQ6IHJlYWRvbmx5IHsgcmVhZG9ubHkgY29udGVudHM6IHN0cmluZzsgcmVhZG9ubHkgbmFtZTogc3RyaW5nIH1bXSxcbiAgaW1wb3J0QWN0aW9uOiBzdHJpbmcsXG4gIG11bHRpcGxlOiBib29sZWFuLFxuICBkaXNwYXRjaE9uZTogRWZmZWN0RGlzcGF0Y2hPbmUsXG4pOiBQcm9taXNlPHZvaWQ+IHtcbiAgY29uc3QgdG90YWwgPSBvcGVuZWQubGVuZ3RoO1xuICBmb3IgKGxldCBpbmRleCA9IDA7IGluZGV4IDwgb3BlbmVkLmxlbmd0aDsgaW5kZXggKz0gMSkge1xuICAgIGNvbnN0IGZpbGUgPSBvcGVuZWRbaW5kZXhdITtcbiAgICBhd2FpdCBkaXNwYXRjaE9uZShpbXBvcnRBY3Rpb24sIG11bHRpcGxlID8geyBwYXlsb2FkOiBmaWxlLmNvbnRlbnRzLCBuYW1lOiBmaWxlLm5hbWUsIGluZGV4LCB0b3RhbCB9IDogeyBwYXlsb2FkOiBmaWxlLmNvbnRlbnRzLCBuYW1lOiBmaWxlLm5hbWUgfSk7XG4gIH1cbn1cblxuLyoqIPCflIHvuI8gRDI6IHNjaGVkdWxlcyBgYWN0aW9uYCBvbnRvIGBkaXNwYXRjaE9uZWAgYWZ0ZXIgYGRlbGF5TXNgICgwID0gbmV4dCB0aWNrKSB2aWEgYHNjaGVkdWxlYCAocmVhbFxuICogY2FsbGVycyBwYXNzIGBzZXRUaW1lb3V0YDsgdGVzdHMgcGFzcyBgdmkudXNlRmFrZVRpbWVycygpYC1kcml2ZW4gYHNldFRpbWVvdXRgIG9yIGEgc3luY2hyb25vdXMgc3R1YikuICovXG5leHBvcnQgZnVuY3Rpb24gc2NoZWR1bGVEaXNwYXRjaEFjdGlvbihcbiAgYWN0aW9uOiBzdHJpbmcsXG4gIGFyZ3M6IFJlY29yZDxzdHJpbmcsIHVua25vd24+IHwgdW5kZWZpbmVkLFxuICBkZWxheU1zOiBudW1iZXIsXG4gIGRpc3BhdGNoT25lOiBFZmZlY3REaXNwYXRjaE9uZSxcbiAgc2NoZWR1bGU6IChmbjogKCkgPT4gdm9pZCwgZGVsYXlNczogbnVtYmVyKSA9PiB2b2lkID0gKGZuLCBtcykgPT4gc2V0VGltZW91dChmbiwgbXMpLFxuKTogdm9pZCB7XG4gIHNjaGVkdWxlKCgpID0+IHtcbiAgICB2b2lkIGRpc3BhdGNoT25lKGFjdGlvbiwgYXJncyk7XG4gIH0sIGRlbGF5TXMpO1xufVxuXG4vLyNyZWdpb24gUmVxdWVzdE1lZGlhRnJhbWVzXG4vLyNyZWdpb24gQm1mZlxuLyoqIPCfp7HvuI8gT25lIHBhcnNlZCBJU08tQk1GRiBib3g6IGBbdHlwZSwgcGF5bG9hZFN0YXJ0LCBwYXlsb2FkRW5kKWAg4oCUIGVub3VnaCB0byByZWN1cnNlIGludG8gY29udGFpbmVyc1xuICogYW5kIHNsaWNlIGxlYWYgcGF5bG9hZHMgd2l0aG91dCBjb3B5aW5nLiAqL1xudHlwZSBCbWZmQm94ID0geyByZWFkb25seSB0eXBlOiBzdHJpbmc7IHJlYWRvbmx5IHN0YXJ0OiBudW1iZXI7IHJlYWRvbmx5IGVuZDogbnVtYmVyIH07XG5cbi8qKiDwn6ex77iPIFdhbGtzIHNpYmxpbmcgYm94ZXMgaW4gYFtzdGFydCwgZW5kKWAg4oCUIGhhbmRsZXMgNjQtYml0IGV4dGVuZGVkIHNpemVzIChgc2l6ZT09PTFgKSBhbmQgdG8tZW5kXG4gKiBib3hlcyAoYHNpemU9PT0wYCk7IG1hbGZvcm1lZC90cnVuY2F0ZWQgaW5wdXQganVzdCBzdG9wcyBlYXJseSByYXRoZXIgdGhhbiB0aHJvd2luZywgc2luY2UgTVA0XG4gKiBwcm9iaW5nIGhlcmUgaXMgYmVzdC1lZmZvcnQg4oCUIHRoZSBUaWVyLTIgYDx2aWRlbz5gIGZhbGxiYWNrIGNvdmVycyBhbnl0aGluZyB0aGlzIGNhbid0IHBhcnNlLiAqL1xuZnVuY3Rpb24gd2Fsa0JtZmZCb3hlcyh2aWV3OiBEYXRhVmlldywgc3RhcnQ6IG51bWJlciwgZW5kOiBudW1iZXIpOiBCbWZmQm94W10ge1xuICBjb25zdCBib3hlczogQm1mZkJveFtdID0gW107XG4gIGxldCBvZmZzZXQgPSBzdGFydDtcbiAgd2hpbGUgKG9mZnNldCArIDggPD0gZW5kKSB7XG4gICAgY29uc3Qgc2l6ZTMyID0gdmlldy5nZXRVaW50MzIob2Zmc2V0KTtcbiAgICBjb25zdCB0eXBlID0gU3RyaW5nLmZyb21DaGFyQ29kZSh2aWV3LmdldFVpbnQ4KG9mZnNldCArIDQpLCB2aWV3LmdldFVpbnQ4KG9mZnNldCArIDUpLCB2aWV3LmdldFVpbnQ4KG9mZnNldCArIDYpLCB2aWV3LmdldFVpbnQ4KG9mZnNldCArIDcpKTtcbiAgICBsZXQgaGVhZGVyU2l6ZSA9IDg7XG4gICAgbGV0IGJveFNpemUgPSBzaXplMzI7XG4gICAgaWYgKHNpemUzMiA9PT0gMSkge1xuICAgICAgaWYgKG9mZnNldCArIDE2ID4gZW5kKSBicmVhaztcbiAgICAgIGJveFNpemUgPSBOdW1iZXIodmlldy5nZXRCaWdVaW50NjQob2Zmc2V0ICsgOCkpO1xuICAgICAgaGVhZGVyU2l6ZSA9IDE2O1xuICAgIH0gZWxzZSBpZiAoc2l6ZTMyID09PSAwKSB7XG4gICAgICBib3hTaXplID0gZW5kIC0gb2Zmc2V0O1xuICAgIH1cbiAgICBpZiAoYm94U2l6ZSA8IGhlYWRlclNpemUgfHwgb2Zmc2V0ICsgYm94U2l6ZSA+IGVuZCkgYnJlYWs7XG4gICAgYm94ZXMucHVzaCh7IHR5cGUsIHN0YXJ0OiBvZmZzZXQgKyBoZWFkZXJTaXplLCBlbmQ6IG9mZnNldCArIGJveFNpemUgfSk7XG4gICAgb2Zmc2V0ICs9IGJveFNpemU7XG4gIH1cbiAgcmV0dXJuIGJveGVzO1xufVxuXG5mdW5jdGlvbiBmaW5kQm1mZkJveChib3hlczogcmVhZG9ubHkgQm1mZkJveFtdLCB0eXBlOiBzdHJpbmcpOiBCbWZmQm94IHwgdW5kZWZpbmVkIHtcbiAgcmV0dXJuIGJveGVzLmZpbmQoKGJveCkgPT4gYm94LnR5cGUgPT09IHR5cGUpO1xufVxuLy8jZW5kcmVnaW9uIEJtZmZcblxuLy8jcmVnaW9uIFRpZXIxXG50eXBlIE1wNFNhbXBsZSA9IHsgcmVhZG9ubHkgb2Zmc2V0OiBudW1iZXI7IHJlYWRvbmx5IHNpemU6IG51bWJlcjsgcmVhZG9ubHkgdGltZXN0YW1wTXM6IG51bWJlcjsgcmVhZG9ubHkgaXNTeW5jOiBib29sZWFuIH07XG50eXBlIE1wNFRyYWNrID0ge1xuICByZWFkb25seSB3aWR0aDogbnVtYmVyO1xuICByZWFkb25seSBoZWlnaHQ6IG51bWJlcjtcbiAgcmVhZG9ubHkgY29kZWM6IFwiYXZjMVwiIHwgXCJodmMxXCI7XG4gIHJlYWRvbmx5IGRlc2NyaXB0aW9uOiBVaW50OEFycmF5O1xuICByZWFkb25seSBzYW1wbGVzOiByZWFkb25seSBNcDRTYW1wbGVbXTtcbn07XG5cbi8qKiDwn46e77iPIE1pbmltYWwgTVA0IHNhbXBsZS10YWJsZSBleHRyYWN0aW9uIOKAlCBgbW9vdiA+IHRyYWtbXSA+IG1kaWEgPiB7bWRoZCwgaGRsciwgbWluZiA+IHN0Ymx9YCBmb3IgdGhlXG4gKiBmaXJzdCB2aWRlbyB0cmFjayAoYGhkbHJgJ3MgaGFuZGxlci10eXBlIGBcInZpZGVcImApLCBlbm91Z2ggdG8gZmVlZCBgVmlkZW9EZWNvZGVyYDogc2FtcGxlIGJ5dGUgcmFuZ2VzXG4gKiBmcm9tIGBzdHNjYCArIGBzdGNvYC9gY282NGAgKyBgc3RzemAsIGRlY29kZSB0aW1lc3RhbXBzIGZyb20gYHN0dHNgLCBzeW5jIGZsYWdzIGZyb20gYHN0c3NgIChhYnNlbnRcbiAqIGBzdHNzYCDih5IgZXZlcnkgc2FtcGxlIGlzIHN5bmMgcGVyIHNwZWMpLCBhbmQgdGhlIEFWQy9IRVZDIGRlY29kZXIgY29uZmlnIGZyb20gYHN0c2RgJ3MgYGF2Y0NgL2BodmNDYC5cbiAqIFJldHVybnMgYG51bGxgIGZvciBhbnl0aGluZyB1bnJlY29nbml6ZWQgKG5vbi1BVkMvSEVWQywgbWlzc2luZyBib3hlcywgbWFsZm9ybWVkIHRhYmxlcykgc28gdGhlXG4gKiBjYWxsZXIgZmFsbHMgYmFjayB0byBUaWVyIDIgcmF0aGVyIHRoYW4gZ3Vlc3NpbmcuICovXG5mdW5jdGlvbiBwcm9iZU1wNFZpZGVvVHJhY2soYnl0ZXM6IFVpbnQ4QXJyYXkpOiBNcDRUcmFjayB8IG51bGwge1xuICBjb25zdCB2aWV3ID0gbmV3IERhdGFWaWV3KGJ5dGVzLmJ1ZmZlciwgYnl0ZXMuYnl0ZU9mZnNldCwgYnl0ZXMuYnl0ZUxlbmd0aCk7XG4gIGNvbnN0IG1vb3YgPSBmaW5kQm1mZkJveCh3YWxrQm1mZkJveGVzKHZpZXcsIDAsIGJ5dGVzLmJ5dGVMZW5ndGgpLCBcIm1vb3ZcIik7XG4gIGlmICghbW9vdikgcmV0dXJuIG51bGw7XG4gIGZvciAoY29uc3QgdHJhayBvZiB3YWxrQm1mZkJveGVzKHZpZXcsIG1vb3Yuc3RhcnQsIG1vb3YuZW5kKS5maWx0ZXIoKGJveCkgPT4gYm94LnR5cGUgPT09IFwidHJha1wiKSkge1xuICAgIGNvbnN0IG1kaWEgPSBmaW5kQm1mZkJveCh3YWxrQm1mZkJveGVzKHZpZXcsIHRyYWsuc3RhcnQsIHRyYWsuZW5kKSwgXCJtZGlhXCIpO1xuICAgIGlmICghbWRpYSkgY29udGludWU7XG4gICAgY29uc3QgbWRpYUJveGVzID0gd2Fsa0JtZmZCb3hlcyh2aWV3LCBtZGlhLnN0YXJ0LCBtZGlhLmVuZCk7XG4gICAgY29uc3QgaGRsciA9IGZpbmRCbWZmQm94KG1kaWFCb3hlcywgXCJoZGxyXCIpO1xuICAgIGlmICghaGRsciB8fCBoZGxyLmVuZCAtIGhkbHIuc3RhcnQgPCAxMikgY29udGludWU7XG4gICAgY29uc3QgaGFuZGxlclR5cGUgPSBTdHJpbmcuZnJvbUNoYXJDb2RlKHZpZXcuZ2V0VWludDgoaGRsci5zdGFydCArIDgpLCB2aWV3LmdldFVpbnQ4KGhkbHIuc3RhcnQgKyA5KSwgdmlldy5nZXRVaW50OChoZGxyLnN0YXJ0ICsgMTApLCB2aWV3LmdldFVpbnQ4KGhkbHIuc3RhcnQgKyAxMSkpO1xuICAgIGlmIChoYW5kbGVyVHlwZSAhPT0gXCJ2aWRlXCIpIGNvbnRpbnVlO1xuICAgIGNvbnN0IG1kaGQgPSBmaW5kQm1mZkJveChtZGlhQm94ZXMsIFwibWRoZFwiKTtcbiAgICBjb25zdCBtaW5mID0gZmluZEJtZmZCb3gobWRpYUJveGVzLCBcIm1pbmZcIik7XG4gICAgaWYgKCFtZGhkIHx8ICFtaW5mKSBjb250aW51ZTtcbiAgICBjb25zdCB0aW1lc2NhbGUgPSB2aWV3LmdldFVpbnQ4KG1kaGQuc3RhcnQpID09PSAxID8gdmlldy5nZXRVaW50MzIobWRoZC5zdGFydCArIDIwKSA6IHZpZXcuZ2V0VWludDMyKG1kaGQuc3RhcnQgKyAxMik7XG4gICAgaWYgKHRpbWVzY2FsZSA8PSAwKSBjb250aW51ZTtcbiAgICBjb25zdCBzdGJsID0gZmluZEJtZmZCb3god2Fsa0JtZmZCb3hlcyh2aWV3LCBtaW5mLnN0YXJ0LCBtaW5mLmVuZCksIFwic3RibFwiKTtcbiAgICBpZiAoIXN0YmwpIGNvbnRpbnVlO1xuICAgIGNvbnN0IHRyYWNrID0gcHJvYmVTYW1wbGVUYWJsZSh2aWV3LCB3YWxrQm1mZkJveGVzKHZpZXcsIHN0Ymwuc3RhcnQsIHN0YmwuZW5kKSwgdGltZXNjYWxlKTtcbiAgICBpZiAodHJhY2spIHJldHVybiB0cmFjaztcbiAgfVxuICByZXR1cm4gbnVsbDtcbn1cblxuZnVuY3Rpb24gcGFyc2VTdHNkKHZpZXc6IERhdGFWaWV3LCBzdHNkOiBCbWZmQm94KTogeyB3aWR0aDogbnVtYmVyOyBoZWlnaHQ6IG51bWJlcjsgY29kZWM6IFwiYXZjMVwiIHwgXCJodmMxXCI7IGRlc2NyaXB0aW9uOiBVaW50OEFycmF5IH0gfCBudWxsIHtcbiAgaWYgKHZpZXcuZ2V0VWludDMyKHN0c2Quc3RhcnQgKyA0KSA8IDEpIHJldHVybiBudWxsO1xuICBjb25zdCBlbnRyeU9mZnNldCA9IHN0c2Quc3RhcnQgKyA4O1xuICBjb25zdCBlbnRyeVNpemUgPSB2aWV3LmdldFVpbnQzMihlbnRyeU9mZnNldCk7XG4gIGNvbnN0IGZvcm1hdCA9IFN0cmluZy5mcm9tQ2hhckNvZGUoXG4gICAgdmlldy5nZXRVaW50OChlbnRyeU9mZnNldCArIDQpLFxuICAgIHZpZXcuZ2V0VWludDgoZW50cnlPZmZzZXQgKyA1KSxcbiAgICB2aWV3LmdldFVpbnQ4KGVudHJ5T2Zmc2V0ICsgNiksXG4gICAgdmlldy5nZXRVaW50OChlbnRyeU9mZnNldCArIDcpLFxuICApO1xuICBpZiAoZm9ybWF0ICE9PSBcImF2YzFcIiAmJiBmb3JtYXQgIT09IFwiaHZjMVwiICYmIGZvcm1hdCAhPT0gXCJoZXYxXCIpIHJldHVybiBudWxsO1xuICBjb25zdCBjb2RlYyA9IGZvcm1hdCA9PT0gXCJhdmMxXCIgPyBcImF2YzFcIiA6IFwiaHZjMVwiO1xuICBjb25zdCB2aXN1YWxFbnRyeVN0YXJ0ID0gZW50cnlPZmZzZXQgKyA4O1xuICBjb25zdCB3aWR0aCA9IHZpZXcuZ2V0VWludDE2KHZpc3VhbEVudHJ5U3RhcnQgKyAyNCk7XG4gIGNvbnN0IGhlaWdodCA9IHZpZXcuZ2V0VWludDE2KHZpc3VhbEVudHJ5U3RhcnQgKyAyNik7XG4gIGNvbnN0IGlubmVyID0gd2Fsa0JtZmZCb3hlcyh2aWV3LCB2aXN1YWxFbnRyeVN0YXJ0ICsgNzgsIGVudHJ5T2Zmc2V0ICsgZW50cnlTaXplKTtcbiAgY29uc3QgY29uZmlnID0gZmluZEJtZmZCb3goaW5uZXIsIGNvZGVjID09PSBcImF2YzFcIiA/IFwiYXZjQ1wiIDogXCJodmNDXCIpO1xuICBpZiAoIWNvbmZpZykgcmV0dXJuIG51bGw7XG4gIHJldHVybiB7IHdpZHRoLCBoZWlnaHQsIGNvZGVjLCBkZXNjcmlwdGlvbjogbmV3IFVpbnQ4QXJyYXkodmlldy5idWZmZXIuc2xpY2UoY29uZmlnLnN0YXJ0LCBjb25maWcuZW5kKSkgfTtcbn1cblxuZnVuY3Rpb24gcGFyc2VTdHN6KHZpZXc6IERhdGFWaWV3LCBib3g6IEJtZmZCb3gpOiBudW1iZXJbXSB7XG4gIGNvbnN0IHVuaWZvcm1TaXplID0gdmlldy5nZXRVaW50MzIoYm94LnN0YXJ0ICsgNCk7XG4gIGNvbnN0IHNhbXBsZUNvdW50ID0gdmlldy5nZXRVaW50MzIoYm94LnN0YXJ0ICsgOCk7XG4gIGlmICh1bmlmb3JtU2l6ZSAhPT0gMCkgcmV0dXJuIG5ldyBBcnJheShzYW1wbGVDb3VudCkuZmlsbCh1bmlmb3JtU2l6ZSkgYXMgbnVtYmVyW107XG4gIGNvbnN0IHNpemVzOiBudW1iZXJbXSA9IFtdO1xuICBmb3IgKGxldCBpID0gMDsgaSA8IHNhbXBsZUNvdW50OyBpICs9IDEpIHNpemVzLnB1c2godmlldy5nZXRVaW50MzIoYm94LnN0YXJ0ICsgMTIgKyBpICogNCkpO1xuICByZXR1cm4gc2l6ZXM7XG59XG5cbmZ1bmN0aW9uIHBhcnNlQ2h1bmtPZmZzZXRzKHZpZXc6IERhdGFWaWV3LCBib3g6IEJtZmZCb3gsIGlzNjQ6IGJvb2xlYW4pOiBudW1iZXJbXSB7XG4gIGNvbnN0IGNvdW50ID0gdmlldy5nZXRVaW50MzIoYm94LnN0YXJ0ICsgNCk7XG4gIGNvbnN0IG9mZnNldHM6IG51bWJlcltdID0gW107XG4gIGZvciAobGV0IGkgPSAwOyBpIDwgY291bnQ7IGkgKz0gMSkge1xuICAgIG9mZnNldHMucHVzaChpczY0ID8gTnVtYmVyKHZpZXcuZ2V0QmlnVWludDY0KGJveC5zdGFydCArIDggKyBpICogOCkpIDogdmlldy5nZXRVaW50MzIoYm94LnN0YXJ0ICsgOCArIGkgKiA0KSk7XG4gIH1cbiAgcmV0dXJuIG9mZnNldHM7XG59XG5cbmZ1bmN0aW9uIHBhcnNlQ2h1bmtPZlNhbXBsZSh2aWV3OiBEYXRhVmlldywgYm94OiBCbWZmQm94LCBzYW1wbGVDb3VudDogbnVtYmVyLCBjaHVua0NvdW50OiBudW1iZXIpOiBudW1iZXJbXSB8IG51bGwge1xuICBjb25zdCBlbnRyeUNvdW50ID0gdmlldy5nZXRVaW50MzIoYm94LnN0YXJ0ICsgNCk7XG4gIGNvbnN0IGVudHJpZXM6IHsgZmlyc3RDaHVuazogbnVtYmVyOyBzYW1wbGVzUGVyQ2h1bms6IG51bWJlciB9W10gPSBbXTtcbiAgZm9yIChsZXQgaSA9IDA7IGkgPCBlbnRyeUNvdW50OyBpICs9IDEpIHtcbiAgICBlbnRyaWVzLnB1c2goeyBmaXJzdENodW5rOiB2aWV3LmdldFVpbnQzMihib3guc3RhcnQgKyA4ICsgaSAqIDEyKSwgc2FtcGxlc1BlckNodW5rOiB2aWV3LmdldFVpbnQzMihib3guc3RhcnQgKyAxMiArIGkgKiAxMikgfSk7XG4gIH1cbiAgY29uc3QgY2h1bmtPZlNhbXBsZTogbnVtYmVyW10gPSBbXTtcbiAgZm9yIChsZXQgZW50cnlJbmRleCA9IDA7IGVudHJ5SW5kZXggPCBlbnRyaWVzLmxlbmd0aDsgZW50cnlJbmRleCArPSAxKSB7XG4gICAgY29uc3QgZW50cnkgPSBlbnRyaWVzW2VudHJ5SW5kZXhdITtcbiAgICBjb25zdCBuZXh0Rmlyc3RDaHVuayA9IGVudHJpZXNbZW50cnlJbmRleCArIDFdPy5maXJzdENodW5rID8/IGNodW5rQ291bnQgKyAxO1xuICAgIGZvciAobGV0IGNodW5rID0gZW50cnkuZmlyc3RDaHVuazsgY2h1bmsgPCBuZXh0Rmlyc3RDaHVuazsgY2h1bmsgKz0gMSkge1xuICAgICAgZm9yIChsZXQgaW5DaHVuayA9IDA7IGluQ2h1bmsgPCBlbnRyeS5zYW1wbGVzUGVyQ2h1bms7IGluQ2h1bmsgKz0gMSkgY2h1bmtPZlNhbXBsZS5wdXNoKGNodW5rKTtcbiAgICB9XG4gIH1cbiAgcmV0dXJuIGNodW5rT2ZTYW1wbGUubGVuZ3RoID49IHNhbXBsZUNvdW50ID8gY2h1bmtPZlNhbXBsZSA6IG51bGw7XG59XG5cbmZ1bmN0aW9uIGNvbXB1dGVTYW1wbGVPZmZzZXRzKGNodW5rT2ZTYW1wbGU6IHJlYWRvbmx5IG51bWJlcltdLCBjaHVua09mZnNldHM6IHJlYWRvbmx5IG51bWJlcltdLCBzaXplczogcmVhZG9ubHkgbnVtYmVyW10pOiBudW1iZXJbXSB7XG4gIGNvbnN0IG9mZnNldHM6IG51bWJlcltdID0gW107XG4gIGNvbnN0IGN1cnNvckJ5Q2h1bmsgPSBuZXcgTWFwPG51bWJlciwgbnVtYmVyPigpO1xuICBmb3IgKGxldCBpID0gMDsgaSA8IHNpemVzLmxlbmd0aDsgaSArPSAxKSB7XG4gICAgY29uc3QgY2h1bmsgPSBjaHVua09mU2FtcGxlW2ldITtcbiAgICBjb25zdCBiYXNlID0gY3Vyc29yQnlDaHVuay5nZXQoY2h1bmspID8/IGNodW5rT2Zmc2V0c1tjaHVuayAtIDFdID8/IDA7XG4gICAgb2Zmc2V0cy5wdXNoKGJhc2UpO1xuICAgIGN1cnNvckJ5Q2h1bmsuc2V0KGNodW5rLCBiYXNlICsgc2l6ZXNbaV0hKTtcbiAgfVxuICByZXR1cm4gb2Zmc2V0cztcbn1cblxuZnVuY3Rpb24gYWNjdW11bGF0ZVRpbWVzdGFtcHNNcyh2aWV3OiBEYXRhVmlldywgc3R0czogQm1mZkJveCwgc2FtcGxlQ291bnQ6IG51bWJlciwgdGltZXNjYWxlOiBudW1iZXIpOiBudW1iZXJbXSB7XG4gIGNvbnN0IGVudHJ5Q291bnQgPSB2aWV3LmdldFVpbnQzMihzdHRzLnN0YXJ0ICsgNCk7XG4gIGNvbnN0IHRpbWVzdGFtcHM6IG51bWJlcltdID0gW107XG4gIGxldCB0aWNrcyA9IDA7XG4gIGZvciAobGV0IGVudHJ5SW5kZXggPSAwOyBlbnRyeUluZGV4IDwgZW50cnlDb3VudCAmJiB0aW1lc3RhbXBzLmxlbmd0aCA8IHNhbXBsZUNvdW50OyBlbnRyeUluZGV4ICs9IDEpIHtcbiAgICBjb25zdCBjb3VudCA9IHZpZXcuZ2V0VWludDMyKHN0dHMuc3RhcnQgKyA4ICsgZW50cnlJbmRleCAqIDgpO1xuICAgIGNvbnN0IGRlbHRhID0gdmlldy5nZXRVaW50MzIoc3R0cy5zdGFydCArIDEyICsgZW50cnlJbmRleCAqIDgpO1xuICAgIGZvciAobGV0IGkgPSAwOyBpIDwgY291bnQgJiYgdGltZXN0YW1wcy5sZW5ndGggPCBzYW1wbGVDb3VudDsgaSArPSAxKSB7XG4gICAgICB0aW1lc3RhbXBzLnB1c2goKHRpY2tzIC8gdGltZXNjYWxlKSAqIDEwMDApO1xuICAgICAgdGlja3MgKz0gZGVsdGE7XG4gICAgfVxuICB9XG4gIHJldHVybiB0aW1lc3RhbXBzO1xufVxuXG5mdW5jdGlvbiBwYXJzZVN5bmNTYW1wbGVzKHZpZXc6IERhdGFWaWV3LCBib3g6IEJtZmZCb3gpOiBTZXQ8bnVtYmVyPiB7XG4gIGNvbnN0IGNvdW50ID0gdmlldy5nZXRVaW50MzIoYm94LnN0YXJ0ICsgNCk7XG4gIGNvbnN0IHN5bmMgPSBuZXcgU2V0PG51bWJlcj4oKTtcbiAgZm9yIChsZXQgaSA9IDA7IGkgPCBjb3VudDsgaSArPSAxKSBzeW5jLmFkZCh2aWV3LmdldFVpbnQzMihib3guc3RhcnQgKyA4ICsgaSAqIDQpKTtcbiAgcmV0dXJuIHN5bmM7XG59XG5cbmZ1bmN0aW9uIHByb2JlU2FtcGxlVGFibGUodmlldzogRGF0YVZpZXcsIHN0YmxCb3hlczogcmVhZG9ubHkgQm1mZkJveFtdLCB0aW1lc2NhbGU6IG51bWJlcik6IE1wNFRyYWNrIHwgbnVsbCB7XG4gIGNvbnN0IHN0c2QgPSBmaW5kQm1mZkJveChzdGJsQm94ZXMsIFwic3RzZFwiKTtcbiAgY29uc3Qgc3R0cyA9IGZpbmRCbWZmQm94KHN0YmxCb3hlcywgXCJzdHRzXCIpO1xuICBjb25zdCBzdHNjID0gZmluZEJtZmZCb3goc3RibEJveGVzLCBcInN0c2NcIik7XG4gIGNvbnN0IHN0c3ogPSBmaW5kQm1mZkJveChzdGJsQm94ZXMsIFwic3RzelwiKTtcbiAgY29uc3Qgc3RjbyA9IGZpbmRCbWZmQm94KHN0YmxCb3hlcywgXCJzdGNvXCIpID8/IGZpbmRCbWZmQm94KHN0YmxCb3hlcywgXCJjbzY0XCIpO1xuICBpZiAoIXN0c2QgfHwgIXN0dHMgfHwgIXN0c2MgfHwgIXN0c3ogfHwgIXN0Y28pIHJldHVybiBudWxsO1xuICBjb25zdCBlbnRyeSA9IHBhcnNlU3RzZCh2aWV3LCBzdHNkKTtcbiAgaWYgKCFlbnRyeSkgcmV0dXJuIG51bGw7XG4gIGNvbnN0IHNpemVzID0gcGFyc2VTdHN6KHZpZXcsIHN0c3opO1xuICBjb25zdCBvZmZzZXRzID0gcGFyc2VDaHVua09mZnNldHModmlldywgc3Rjbywgc3Rjby50eXBlID09PSBcImNvNjRcIik7XG4gIGNvbnN0IGNodW5rT2ZTYW1wbGUgPSBwYXJzZUNodW5rT2ZTYW1wbGUodmlldywgc3RzYywgc2l6ZXMubGVuZ3RoLCBvZmZzZXRzLmxlbmd0aCk7XG4gIGlmICghY2h1bmtPZlNhbXBsZSkgcmV0dXJuIG51bGw7XG4gIGNvbnN0IHNhbXBsZU9mZnNldHMgPSBjb21wdXRlU2FtcGxlT2Zmc2V0cyhjaHVua09mU2FtcGxlLCBvZmZzZXRzLCBzaXplcyk7XG4gIGNvbnN0IHRpbWVzdGFtcHNNcyA9IGFjY3VtdWxhdGVUaW1lc3RhbXBzTXModmlldywgc3R0cywgc2l6ZXMubGVuZ3RoLCB0aW1lc2NhbGUpO1xuICBjb25zdCBzdHNzID0gZmluZEJtZmZCb3goc3RibEJveGVzLCBcInN0c3NcIik7XG4gIGNvbnN0IHN5bmNTYW1wbGVzID0gc3RzcyA/IHBhcnNlU3luY1NhbXBsZXModmlldywgc3RzcykgOiBudWxsO1xuICBjb25zdCBzYW1wbGVzOiBNcDRTYW1wbGVbXSA9IHNpemVzLm1hcCgoc2l6ZSwgaW5kZXgpID0+ICh7XG4gICAgb2Zmc2V0OiBzYW1wbGVPZmZzZXRzW2luZGV4XSEsXG4gICAgc2l6ZSxcbiAgICB0aW1lc3RhbXBNczogdGltZXN0YW1wc01zW2luZGV4XSA/PyAwLFxuICAgIGlzU3luYzogc3luY1NhbXBsZXMgPyBzeW5jU2FtcGxlcy5oYXMoaW5kZXggKyAxKSA6IHRydWUsXG4gIH0pKTtcbiAgcmV0dXJuIHsgd2lkdGg6IGVudHJ5LndpZHRoLCBoZWlnaHQ6IGVudHJ5LmhlaWdodCwgY29kZWM6IGVudHJ5LmNvZGVjLCBkZXNjcmlwdGlvbjogZW50cnkuZGVzY3JpcHRpb24sIHNhbXBsZXMgfTtcbn1cblxuLyoqIPCfjJDvuI8gRmVhdHVyZS1kZXRlY3RzIHRoZSBXZWJDb2RlY3MgYFZpZGVvRGVjb2RlcmAvYEVuY29kZWRWaWRlb0NodW5rYCBnbG9iYWxzIChUaWVyIDEncyBwcmVyZXF1aXNpdGU7XG4gKiBhYnNlbnQgaW4gbW9zdCBKUyB0ZXN0IGVudmlyb25tZW50cyBhbmQgaW4gYnJvd3NlcnMgdGhhdCBvbmx5IHN1cHBvcnQgV2ViTS9WUDkgd2l0aG91dCBhbiBBVkMgcGF0aCkuICovXG5mdW5jdGlvbiB3ZWJDb2RlY3NBdmFpbGFibGUoKTogYm9vbGVhbiB7XG4gIGNvbnN0IHNjb3BlID0gd2luZG93IGFzIHVua25vd24gYXMgeyBWaWRlb0RlY29kZXI/OiB1bmtub3duOyBFbmNvZGVkVmlkZW9DaHVuaz86IHVua25vd24gfTtcbiAgcmV0dXJuIHR5cGVvZiBzY29wZS5WaWRlb0RlY29kZXIgPT09IFwiZnVuY3Rpb25cIiAmJiB0eXBlb2Ygc2NvcGUuRW5jb2RlZFZpZGVvQ2h1bmsgPT09IFwiZnVuY3Rpb25cIjtcbn1cblxuLyoqIPCflKLvuI8gRGVyaXZlcyBhIFdlYkNvZGVjcyBgYXZjMS5QUENDTExgIGNvZGVjIHN0cmluZyBmcm9tIGFuIGBhdmNDYCBib3gncyBwcm9maWxlL2NvbXBhdC9sZXZlbCBieXRlc1xuICogKG9mZnNldHMgMS8yLzMg4oCUIHZlcnNpb24gaXMgYnl0ZSAwKS4gKi9cbmZ1bmN0aW9uIGF2Y0NvZGVjU3RyaW5nKGRlc2NyaXB0aW9uOiBVaW50OEFycmF5KTogc3RyaW5nIHtcbiAgY29uc3QgaGV4ID0gKGJ5dGU6IG51bWJlciB8IHVuZGVmaW5lZCkgPT4gKGJ5dGUgPz8gMCkudG9TdHJpbmcoMTYpLnBhZFN0YXJ0KDIsIFwiMFwiKTtcbiAgcmV0dXJuIGBhdmMxLiR7aGV4KGRlc2NyaXB0aW9uWzFdKX0ke2hleChkZXNjcmlwdGlvblsyXSl9JHtoZXgoZGVzY3JpcHRpb25bM10pfWA7XG59XG5cbnR5cGUgV2ViQ29kZWNzVmlkZW9GcmFtZSA9IHsgcmVhZG9ubHkgY29kZWRXaWR0aDogbnVtYmVyOyByZWFkb25seSBjb2RlZEhlaWdodDogbnVtYmVyOyBjbG9zZTogKCkgPT4gdm9pZCB9O1xudHlwZSBXZWJDb2RlY3NWaWRlb0RlY29kZXJDdG9yID0gbmV3IChpbml0OiB7IG91dHB1dDogKGZyYW1lOiBXZWJDb2RlY3NWaWRlb0ZyYW1lKSA9PiB2b2lkOyBlcnJvcjogKGVycm9yOiB1bmtub3duKSA9PiB2b2lkIH0pID0+IHtcbiAgY29uZmlndXJlOiAoY29uZmlnOiB7IGNvZGVjOiBzdHJpbmc7IGNvZGVkV2lkdGg6IG51bWJlcjsgY29kZWRIZWlnaHQ6IG51bWJlcjsgZGVzY3JpcHRpb246IFVpbnQ4QXJyYXkgfSkgPT4gdm9pZDtcbiAgZGVjb2RlOiAoY2h1bms6IHVua25vd24pID0+IHZvaWQ7XG4gIGZsdXNoOiAoKSA9PiBQcm9taXNlPHZvaWQ+O1xuICBjbG9zZTogKCkgPT4gdm9pZDtcbn07XG50eXBlIFdlYkNvZGVjc0VuY29kZWRWaWRlb0NodW5rQ3RvciA9IG5ldyAoaW5pdDogeyB0eXBlOiBcImtleVwiIHwgXCJkZWx0YVwiOyB0aW1lc3RhbXA6IG51bWJlcjsgZGF0YTogVWludDhBcnJheSB9KSA9PiB1bmtub3duO1xuXG5mdW5jdGlvbiBqcGVnRGF0YVVybEZyb21GcmFtZShmcmFtZTogV2ViQ29kZWNzVmlkZW9GcmFtZSk6IHsgcmVhZG9ubHkgZGF0YVVybDogc3RyaW5nOyByZWFkb25seSB3aWR0aDogbnVtYmVyOyByZWFkb25seSBoZWlnaHQ6IG51bWJlciB9IHtcbiAgY29uc3QgY2FudmFzID0gZG9jdW1lbnQuY3JlYXRlRWxlbWVudChcImNhbnZhc1wiKTtcbiAgY2FudmFzLndpZHRoID0gZnJhbWUuY29kZWRXaWR0aDtcbiAgY2FudmFzLmhlaWdodCA9IGZyYW1lLmNvZGVkSGVpZ2h0O1xuICBjYW52YXMuZ2V0Q29udGV4dChcIjJkXCIpPy5kcmF3SW1hZ2UoZnJhbWUgYXMgdW5rbm93biBhcyBDYW52YXNJbWFnZVNvdXJjZSwgMCwgMCk7XG4gIHJldHVybiB7IGRhdGFVcmw6IGNhbnZhcy50b0RhdGFVUkwoXCJpbWFnZS9qcGVnXCIsIDAuOSksIHdpZHRoOiBmcmFtZS5jb2RlZFdpZHRoLCBoZWlnaHQ6IGZyYW1lLmNvZGVkSGVpZ2h0IH07XG59XG5cbi8qKiDwn46e77iPIERlY29kZXMgZXhhY3RseSB0aGUgc2FtcGxlcyBuZWVkZWQgZm9yIG9uZSB0YXJnZXQgZnJhbWUg4oCUIGZyb20gaXRzIG5lYXJlc3QgcHJlY2VkaW5nIHN5bmMgc2FtcGxlXG4gKiB0aHJvdWdoIHRoZSB0YXJnZXQg4oCUIHZpYSBhIGZyZXNoIGBWaWRlb0RlY29kZXJgLCBjYXB0dXJpbmcgb25seSB0aGUgbGFzdCBvdXRwdXQgZnJhbWUuIFNpbXBsaWZpY2F0aW9uOlxuICogZWFjaCB0YXJnZXQgZnJhbWUgcmUtZGVjb2RlcyBpdHMgR09QIHByZWZpeCBmcm9tIHNjcmF0Y2ggaW5zdGVhZCBvZiBzdHJlYW1pbmcgY29udGludW91c2x5IGFjcm9zc1xuICogdGFyZ2V0cyBhbmQgZGVtdXhpbmcgb3V0cHV0cyBieSB0aW1lc3RhbXA7IGFjY2VwdGFibGUgYmVjYXVzZSBzYW1wbGVkIGluZ2VzdGlvbiAoYHNhbXBsZVN0cmlkZWAvXG4gKiBgbWF4RnJhbWVzYCkga2VlcHMgR09QIHByZWZpeGVzIHNob3J0IGJldHdlZW4gdGFyZ2V0cywgYW5kIFRpZXIgMidzIGA8dmlkZW8+YCBlbGVtZW50IGlzIGFsd2F5cyB0aGVcbiAqIGNvcnJlY3RuZXNzIGZhbGxiYWNrIGlmIFRpZXIgMSBmYWlscyBvciB0aGUgY29kZWMgaXNuJ3QgYmFzZWxpbmUtZnJpZW5kbHkuICovXG5hc3luYyBmdW5jdGlvbiBkZWNvZGVPbmVNcDRGcmFtZSh0cmFjazogTXA0VHJhY2ssIGJ5dGVzOiBVaW50OEFycmF5LCB0YXJnZXRJbmRleDogbnVtYmVyKTogUHJvbWlzZTx7IGRhdGFVcmw6IHN0cmluZzsgd2lkdGg6IG51bWJlcjsgaGVpZ2h0OiBudW1iZXIgfSB8IG51bGw+IHtcbiAgY29uc3Qgc2NvcGUgPSB3aW5kb3cgYXMgdW5rbm93biBhcyB7IFZpZGVvRGVjb2RlcjogV2ViQ29kZWNzVmlkZW9EZWNvZGVyQ3RvcjsgRW5jb2RlZFZpZGVvQ2h1bms6IFdlYkNvZGVjc0VuY29kZWRWaWRlb0NodW5rQ3RvciB9O1xuICBsZXQgc3luY0luZGV4ID0gdGFyZ2V0SW5kZXg7XG4gIHdoaWxlIChzeW5jSW5kZXggPiAwICYmICF0cmFjay5zYW1wbGVzW3N5bmNJbmRleF0hLmlzU3luYykgc3luY0luZGV4IC09IDE7XG4gIGxldCBjYXB0dXJlZDogeyBkYXRhVXJsOiBzdHJpbmc7IHdpZHRoOiBudW1iZXI7IGhlaWdodDogbnVtYmVyIH0gfCBudWxsID0gbnVsbDtcbiAgYXdhaXQgbmV3IFByb21pc2U8dm9pZD4oKHJlc29sdmUsIHJlamVjdCkgPT4ge1xuICAgIGNvbnN0IGRlY29kZXIgPSBuZXcgc2NvcGUuVmlkZW9EZWNvZGVyKHtcbiAgICAgIG91dHB1dDogKGZyYW1lKSA9PiB7XG4gICAgICAgIGNhcHR1cmVkID0ganBlZ0RhdGFVcmxGcm9tRnJhbWUoZnJhbWUpO1xuICAgICAgICBmcmFtZS5jbG9zZSgpO1xuICAgICAgfSxcbiAgICAgIGVycm9yOiByZWplY3QsXG4gICAgfSk7XG4gICAgZGVjb2Rlci5jb25maWd1cmUoeyBjb2RlYzogYXZjQ29kZWNTdHJpbmcodHJhY2suZGVzY3JpcHRpb24pLCBjb2RlZFdpZHRoOiB0cmFjay53aWR0aCwgY29kZWRIZWlnaHQ6IHRyYWNrLmhlaWdodCwgZGVzY3JpcHRpb246IHRyYWNrLmRlc2NyaXB0aW9uIH0pO1xuICAgIGZvciAobGV0IGkgPSBzeW5jSW5kZXg7IGkgPD0gdGFyZ2V0SW5kZXg7IGkgKz0gMSkge1xuICAgICAgY29uc3Qgc2FtcGxlID0gdHJhY2suc2FtcGxlc1tpXSE7XG4gICAgICBkZWNvZGVyLmRlY29kZShcbiAgICAgICAgbmV3IHNjb3BlLkVuY29kZWRWaWRlb0NodW5rKHsgdHlwZTogc2FtcGxlLmlzU3luYyA/IFwia2V5XCIgOiBcImRlbHRhXCIsIHRpbWVzdGFtcDogc2FtcGxlLnRpbWVzdGFtcE1zICogMTAwMCwgZGF0YTogYnl0ZXMuc3ViYXJyYXkoc2FtcGxlLm9mZnNldCwgc2FtcGxlLm9mZnNldCArIHNhbXBsZS5zaXplKSB9KSxcbiAgICAgICk7XG4gICAgfVxuICAgIGRlY29kZXIuZmx1c2goKS50aGVuKCgpID0+IHtcbiAgICAgIGRlY29kZXIuY2xvc2UoKTtcbiAgICAgIHJlc29sdmUoKTtcbiAgICB9LCByZWplY3QpO1xuICB9KTtcbiAgcmV0dXJuIGNhcHR1cmVkO1xufVxuXG4vKiog8J+Onu+4jyBUaWVyIDEgb3JjaGVzdHJhdGlvbjogZGVtdXhlcyBgYnl0ZXNgIGFzIE1QNC9BVkMsIGRlY29kZXMgb25lIGZyYW1lIHBlciBzYW1wbGVkIHRpbWVzdGFtcCwgYW5kXG4gKiBkaXNwYXRjaGVzIGBmcmFtZUFjdGlvbmAgcGVyIGZyYW1lICsgYGRvbmVBY3Rpb25gIG9uY2UuIFJldHVybnMgYGZhbHNlYCAobm8gZGlzcGF0Y2ggcGVyZm9ybWVkIGF0XG4gKiBhbGwpIHdoZW4gdGhlIGRlbXV4IGNhbid0IGZpbmQgYSB1c2FibGUgQVZDIHZpZGVvIHRyYWNrLCBzbyB0aGUgY2FsbGVyIGZhbGxzIHRocm91Z2ggdG8gVGllciAyLiAqL1xuYXN5bmMgZnVuY3Rpb24gcnVuVGllcjFWaWRlb0ZyYW1lcyhieXRlczogVWludDhBcnJheSwgZWZmZWN0OiBSZXF1ZXN0TWVkaWFGcmFtZXNBcmdzLCBuYW1lOiBzdHJpbmcsIGRpc3BhdGNoT25lOiBFZmZlY3REaXNwYXRjaE9uZSk6IFByb21pc2U8Ym9vbGVhbj4ge1xuICBjb25zdCB0cmFjayA9IHByb2JlTXA0VmlkZW9UcmFjayhieXRlcyk7XG4gIGlmICghdHJhY2sgfHwgdHJhY2suc2FtcGxlcy5sZW5ndGggPT09IDApIHJldHVybiBmYWxzZTtcbiAgY29uc3QgZHVyYXRpb25NcyA9IHRyYWNrLnNhbXBsZXNbdHJhY2suc2FtcGxlcy5sZW5ndGggLSAxXSEudGltZXN0YW1wTXM7XG4gIGNvbnN0IHRpbWVzdGFtcHMgPSBzYW1wbGVNZWRpYUZyYW1lVGltZXN0YW1wc01zKGR1cmF0aW9uTXMsIGVmZmVjdC5zYW1wbGVTdHJpZGUsIGVmZmVjdC5tYXhGcmFtZXMsIGVmZmVjdC5mcHNIaW50KTtcbiAgbGV0IHNhbXBsZWRDb3VudCA9IDA7XG4gIGZvciAobGV0IGluZGV4ID0gMDsgaW5kZXggPCB0aW1lc3RhbXBzLmxlbmd0aDsgaW5kZXggKz0gMSkge1xuICAgIGNvbnN0IHRhcmdldE1zID0gdGltZXN0YW1wc1tpbmRleF0hO1xuICAgIGxldCB0YXJnZXRTYW1wbGVJbmRleCA9IDA7XG4gICAgZm9yIChsZXQgaSA9IDA7IGkgPCB0cmFjay5zYW1wbGVzLmxlbmd0aDsgaSArPSAxKSBpZiAodHJhY2suc2FtcGxlc1tpXSEudGltZXN0YW1wTXMgPD0gdGFyZ2V0TXMpIHRhcmdldFNhbXBsZUluZGV4ID0gaTtcbiAgICBjb25zdCBmcmFtZSA9IGF3YWl0IGRlY29kZU9uZU1wNEZyYW1lKHRyYWNrLCBieXRlcywgdGFyZ2V0U2FtcGxlSW5kZXgpO1xuICAgIGlmICghZnJhbWUpIGNvbnRpbnVlO1xuICAgIHNhbXBsZWRDb3VudCArPSAxO1xuICAgIGF3YWl0IGRpc3BhdGNoT25lKGVmZmVjdC5mcmFtZUFjdGlvbiwge1xuICAgICAgcGF5bG9hZDogZnJhbWUuZGF0YVVybCxcbiAgICAgIG5hbWUsXG4gICAgICBmcmFtZUluZGV4OiBpbmRleCxcbiAgICAgIHRpbWVzdGFtcE1zOiB0YXJnZXRNcyxcbiAgICAgIGluZGV4LFxuICAgICAgdG90YWw6IHRpbWVzdGFtcHMubGVuZ3RoLFxuICAgICAgd2lkdGg6IGZyYW1lLndpZHRoLFxuICAgICAgaGVpZ2h0OiBmcmFtZS5oZWlnaHQsXG4gICAgICAuLi5lZmZlY3QuYXJncyxcbiAgICB9KTtcbiAgfVxuICBhd2FpdCBkaXNwYXRjaE9uZShlZmZlY3QuZG9uZUFjdGlvbiwge1xuICAgIG5hbWUsXG4gICAgZHVyYXRpb25NcyxcbiAgICBmcmFtZUNvdW50OiB0cmFjay5zYW1wbGVzLmxlbmd0aCxcbiAgICBzYW1wbGVkQ291bnQsXG4gICAgd2lkdGg6IHRyYWNrLndpZHRoLFxuICAgIGhlaWdodDogdHJhY2suaGVpZ2h0LFxuICAgIGNvZGVjOiB0cmFjay5jb2RlYyxcbiAgICAuLi5lZmZlY3QuYXJncyxcbiAgfSk7XG4gIHJldHVybiB0cnVlO1xufVxuLy8jZW5kcmVnaW9uIFRpZXIxXG5cbi8vI3JlZ2lvbiBUaWVyMlxuLyoqIOKPse+4jyBUaWVyLTIgKGA8dmlkZW8+YCBzZWVrLWFuZC1jYXB0dXJlKSB0YXJnZXQgdGltZXN0YW1wcywgbXMg4oCUIG9uZSBldmVyeSBgc2FtcGxlU3RyaWRlIC9cbiAqIChmcHNIaW50IHx8IDMwKWAgc2Vjb25kcyBzdGFydGluZyBhdCAwLCBjYXBwZWQgYXQgYG1heEZyYW1lc2AgKDAg4oeSIHVubGltaXRlZCwgYm91bmRlZCBvbmx5IGJ5XG4gKiBgZHVyYXRpb25Nc2ApLiBQdXJlL2RldGVybWluaXN0aWMgc28gaXQncyB1bml0LXRlc3RhYmxlIHdpdGhvdXQgYW55IERPTSBvciBtZWRpYSBBUElzLiBDb21wdXRlcyBlYWNoXG4gKiB0aW1lc3RhbXAgYXMgYGsgKiBzdGVwTXNgIHJhdGhlciB0aGFuIGFuIGFjY3VtdWxhdGluZyBgdHMgKz0gc3RlcE1zYCBsb29wIOKAlCByZXBlYXRlZCBmbG9hdCBhZGRpdGlvblxuICogZHJpZnRzIGVub3VnaCBvdmVyIGRvemVucyBvZiBzdGVwcyB0byBvY2Nhc2lvbmFsbHkgbGFuZCBqdXN0IHVuZGVyIGFuIGV4YWN0IG11bHRpcGxlIG9mIGBkdXJhdGlvbk1zYCxcbiAqIHNuZWFraW5nIGluIG9uZSBleHRyYSB0aW1lc3RhbXA7IG11bHRpcGx5aW5nIGZyb20gdGhlIGxvb3AgaW5kZXggaXMgZXhhY3QgcGVyLXN0ZXAgYW5kIGRldGVybWluaXN0aWMuICovXG5leHBvcnQgZnVuY3Rpb24gc2FtcGxlTWVkaWFGcmFtZVRpbWVzdGFtcHNNcyhkdXJhdGlvbk1zOiBudW1iZXIsIHNhbXBsZVN0cmlkZTogbnVtYmVyLCBtYXhGcmFtZXM6IG51bWJlciwgZnBzSGludDogbnVtYmVyKTogbnVtYmVyW10ge1xuICBjb25zdCBzdHJpZGUgPSBzYW1wbGVTdHJpZGUgPiAwID8gc2FtcGxlU3RyaWRlIDogMTtcbiAgY29uc3QgZnBzID0gZnBzSGludCA+IDAgPyBmcHNIaW50IDogMzA7XG4gIGNvbnN0IHN0ZXBNcyA9IChzdHJpZGUgLyBmcHMpICogMTAwMDtcbiAgY29uc3QgdGltZXN0YW1wczogbnVtYmVyW10gPSBbXTtcbiAgaWYgKGR1cmF0aW9uTXMgPD0gMCB8fCBzdGVwTXMgPD0gMCkgcmV0dXJuIHRpbWVzdGFtcHM7XG4gIGZvciAobGV0IGsgPSAwOyA7IGsgKz0gMSkge1xuICAgIGlmIChtYXhGcmFtZXMgPiAwICYmIHRpbWVzdGFtcHMubGVuZ3RoID49IG1heEZyYW1lcykgYnJlYWs7XG4gICAgY29uc3QgdHMgPSBrICogc3RlcE1zO1xuICAgIGlmICh0cyA+PSBkdXJhdGlvbk1zKSBicmVhaztcbiAgICB0aW1lc3RhbXBzLnB1c2godHMpO1xuICB9XG4gIHJldHVybiB0aW1lc3RhbXBzO1xufVxuXG5mdW5jdGlvbiBjYXB0dXJlQ2FudmFzRnJhbWUodmlkZW86IEhUTUxWaWRlb0VsZW1lbnQsIG1heExvbmdFZGdlUHg6IG51bWJlcik6IHsgcmVhZG9ubHkgZGF0YVVybDogc3RyaW5nOyByZWFkb25seSB3aWR0aDogbnVtYmVyOyByZWFkb25seSBoZWlnaHQ6IG51bWJlciB9IHtcbiAgY29uc3Qgc291cmNlV2lkdGggPSB2aWRlby52aWRlb1dpZHRoIHx8IDA7XG4gIGNvbnN0IHNvdXJjZUhlaWdodCA9IHZpZGVvLnZpZGVvSGVpZ2h0IHx8IDA7XG4gIGNvbnN0IHNjYWxlID0gbWF4TG9uZ0VkZ2VQeCA+IDAgPyBNYXRoLm1pbigxLCBtYXhMb25nRWRnZVB4IC8gTWF0aC5tYXgoc291cmNlV2lkdGgsIHNvdXJjZUhlaWdodCwgMSkpIDogMTtcbiAgY29uc3Qgd2lkdGggPSBNYXRoLm1heCgxLCBNYXRoLnJvdW5kKHNvdXJjZVdpZHRoICogc2NhbGUpKTtcbiAgY29uc3QgaGVpZ2h0ID0gTWF0aC5tYXgoMSwgTWF0aC5yb3VuZChzb3VyY2VIZWlnaHQgKiBzY2FsZSkpO1xuICBjb25zdCBjYW52YXMgPSBkb2N1bWVudC5jcmVhdGVFbGVtZW50KFwiY2FudmFzXCIpO1xuICBjYW52YXMud2lkdGggPSB3aWR0aDtcbiAgY2FudmFzLmhlaWdodCA9IGhlaWdodDtcbiAgY2FudmFzLmdldENvbnRleHQoXCIyZFwiKT8uZHJhd0ltYWdlKHZpZGVvLCAwLCAwLCB3aWR0aCwgaGVpZ2h0KTtcbiAgcmV0dXJuIHsgZGF0YVVybDogY2FudmFzLnRvRGF0YVVSTChcImltYWdlL2pwZWdcIiwgMC45KSwgd2lkdGgsIGhlaWdodCB9O1xufVxuXG5mdW5jdGlvbiB3YWl0Rm9yVmlkZW9FdmVudCh2aWRlbzogSFRNTFZpZGVvRWxlbWVudCwgdHlwZTogc3RyaW5nKTogUHJvbWlzZTx2b2lkPiB7XG4gIHJldHVybiBuZXcgUHJvbWlzZSgocmVzb2x2ZSkgPT4ge1xuICAgIGNvbnN0IGhhbmRsZXIgPSAoKSA9PiB7XG4gICAgICB2aWRlby5yZW1vdmVFdmVudExpc3RlbmVyKHR5cGUsIGhhbmRsZXIpO1xuICAgICAgcmVzb2x2ZSgpO1xuICAgIH07XG4gICAgdmlkZW8uYWRkRXZlbnRMaXN0ZW5lcih0eXBlLCBoYW5kbGVyKTtcbiAgfSk7XG59XG5cbi8qKiDwn46e77iPIFRpZXIgMiBvcmNoZXN0cmF0aW9uOiB3YWl0cyBmb3IgYGxvYWRlZG1ldGFkYXRhYCAoaWYgbm90IGFscmVhZHkgYXZhaWxhYmxlKSwgc2Vla3MgYHZpZGVvYFxuICogdGhyb3VnaCB7QGxpbmsgc2FtcGxlTWVkaWFGcmFtZVRpbWVzdGFtcHNNc30ncyBzY2hlZHVsZSwgY2FwdHVyZXMgZWFjaCBsYW5kZWQgZnJhbWUgdG8gYSBzY2FsZWQgSlBFR1xuICogZGF0YSBVUkwsIGRpc3BhdGNoZXMgYGZyYW1lQWN0aW9uYCBwZXIgZnJhbWUsIHRoZW4gYGRvbmVBY3Rpb25gIG9uY2UuIFVzZWQgYm90aCBhcyB0aGUgV2ViTS9uby1cbiAqIFdlYkNvZGVjcyBmYWxsYmFjayBhbmQgZGlyZWN0bHkgYnkgdGVzdHMgKHdoaWNoIGluamVjdCBhIHJlYWwgYDx2aWRlbz5gIGVsZW1lbnQgd2l0aCBvdmVycmlkZGVuXG4gKiBgZHVyYXRpb25gL2B2aWRlb1dpZHRoYC9gdmlkZW9IZWlnaHRgL2ByZWFkeVN0YXRlYCBhbmQgbWFudWFsbHkgZGlzcGF0Y2ggYGxvYWRlZG1ldGFkYXRhYC9gc2Vla2VkYCxcbiAqIHNpbmNlIGhlYWRsZXNzIHRlc3QgZW52aXJvbm1lbnRzIGhhdmUgbm8gcmVhbCBtZWRpYSBkZWNvZGVyKS4gKi9cbmV4cG9ydCBhc3luYyBmdW5jdGlvbiBydW5UaWVyMlZpZGVvRnJhbWVzKHZpZGVvOiBIVE1MVmlkZW9FbGVtZW50LCBlZmZlY3Q6IFJlcXVlc3RNZWRpYUZyYW1lc0FyZ3MsIG5hbWU6IHN0cmluZywgZGlzcGF0Y2hPbmU6IEVmZmVjdERpc3BhdGNoT25lKTogUHJvbWlzZTx2b2lkPiB7XG4gIGlmICh2aWRlby5yZWFkeVN0YXRlIDwgMSkgYXdhaXQgd2FpdEZvclZpZGVvRXZlbnQodmlkZW8sIFwibG9hZGVkbWV0YWRhdGFcIik7XG4gIGNvbnN0IGR1cmF0aW9uTXMgPSBOdW1iZXIuaXNGaW5pdGUodmlkZW8uZHVyYXRpb24pID8gdmlkZW8uZHVyYXRpb24gKiAxMDAwIDogMDtcbiAgY29uc3Qgd2lkdGggPSB2aWRlby52aWRlb1dpZHRoIHx8IDA7XG4gIGNvbnN0IGhlaWdodCA9IHZpZGVvLnZpZGVvSGVpZ2h0IHx8IDA7XG4gIGNvbnN0IHRpbWVzdGFtcHMgPSBzYW1wbGVNZWRpYUZyYW1lVGltZXN0YW1wc01zKGR1cmF0aW9uTXMsIGVmZmVjdC5zYW1wbGVTdHJpZGUsIGVmZmVjdC5tYXhGcmFtZXMsIGVmZmVjdC5mcHNIaW50KTtcbiAgY29uc3QgdG90YWwgPSB0aW1lc3RhbXBzLmxlbmd0aDtcbiAgZm9yIChsZXQgaW5kZXggPSAwOyBpbmRleCA8IHRvdGFsOyBpbmRleCArPSAxKSB7XG4gICAgY29uc3QgdGltZXN0YW1wTXMgPSB0aW1lc3RhbXBzW2luZGV4XSE7XG4gICAgdmlkZW8uY3VycmVudFRpbWUgPSB0aW1lc3RhbXBNcyAvIDEwMDA7XG4gICAgYXdhaXQgd2FpdEZvclZpZGVvRXZlbnQodmlkZW8sIFwic2Vla2VkXCIpO1xuICAgIGNvbnN0IGZyYW1lID0gY2FwdHVyZUNhbnZhc0ZyYW1lKHZpZGVvLCBlZmZlY3QubWF4TG9uZ0VkZ2VQeCk7XG4gICAgYXdhaXQgZGlzcGF0Y2hPbmUoZWZmZWN0LmZyYW1lQWN0aW9uLCB7XG4gICAgICBwYXlsb2FkOiBmcmFtZS5kYXRhVXJsLFxuICAgICAgbmFtZSxcbiAgICAgIGZyYW1lSW5kZXg6IGluZGV4LFxuICAgICAgdGltZXN0YW1wTXMsXG4gICAgICBpbmRleCxcbiAgICAgIHRvdGFsLFxuICAgICAgd2lkdGg6IGZyYW1lLndpZHRoLFxuICAgICAgaGVpZ2h0OiBmcmFtZS5oZWlnaHQsXG4gICAgICAuLi5lZmZlY3QuYXJncyxcbiAgICB9KTtcbiAgfVxuICBhd2FpdCBkaXNwYXRjaE9uZShlZmZlY3QuZG9uZUFjdGlvbiwgeyBuYW1lLCBkdXJhdGlvbk1zLCBmcmFtZUNvdW50OiB0b3RhbCwgc2FtcGxlZENvdW50OiB0b3RhbCwgd2lkdGgsIGhlaWdodCwgY29kZWM6IFwidW5rbm93blwiLCAuLi5lZmZlY3QuYXJncyB9KTtcbn1cbi8vI2VuZHJlZ2lvbiBUaWVyMlxuXG4vKiog8J+Onu+4jyBENSBgUmVxdWVzdE1lZGlhRnJhbWVzYCBmaWVsZHMgdGhlIHR3byBkZWNvZGUgdGllcnMgbmVlZCwgZGVjb3VwbGVkIGZyb20gdGhlIHJhdyBgSG9zdEVmZmVjdGBcbiAqIHVuaW9uIG1lbWJlciBzaGFwZSBzbyBvcmNoZXN0cmF0aW9uIGZ1bmN0aW9ucyBhYm92ZSB0YWtlIGEgcGxhaW4sIGVhc2lseS1jb25zdHJ1Y3RlZC1pbi10ZXN0cyBvYmplY3QuICovXG5leHBvcnQgdHlwZSBSZXF1ZXN0TWVkaWFGcmFtZXNBcmdzID0ge1xuICByZWFkb25seSBmcmFtZUFjdGlvbjogc3RyaW5nO1xuICByZWFkb25seSBkb25lQWN0aW9uOiBzdHJpbmc7XG4gIHJlYWRvbmx5IGZhbGxiYWNrQWN0aW9uOiBzdHJpbmc7XG4gIHJlYWRvbmx5IHNhbXBsZVN0cmlkZTogbnVtYmVyO1xuICByZWFkb25seSBtYXhGcmFtZXM6IG51bWJlcjtcbiAgcmVhZG9ubHkgbWF4TG9uZ0VkZ2VQeDogbnVtYmVyO1xuICByZWFkb25seSBmcHNIaW50OiBudW1iZXI7XG4gIHJlYWRvbmx5IGFyZ3M/OiBSZWNvcmQ8c3RyaW5nLCB1bmtub3duPjtcbn07XG5cbmZ1bmN0aW9uIGJ5dGVzRnJvbURhdGFVcmwoZGF0YVVybDogc3RyaW5nKTogVWludDhBcnJheSB7XG4gIGNvbnN0IGJpbmFyeSA9IGF0b2IoZGF0YVVybC5zbGljZShkYXRhVXJsLmluZGV4T2YoXCIsXCIpICsgMSkpO1xuICBjb25zdCBieXRlcyA9IG5ldyBVaW50OEFycmF5KGJpbmFyeS5sZW5ndGgpO1xuICBmb3IgKGxldCBpID0gMDsgaSA8IGJpbmFyeS5sZW5ndGg7IGkgKz0gMSkgYnl0ZXNbaV0gPSBiaW5hcnkuY2hhckNvZGVBdChpKTtcbiAgcmV0dXJuIGJ5dGVzO1xufVxuXG5mdW5jdGlvbiBieXRlc1RvRGF0YVVybChieXRlczogVWludDhBcnJheSwgbWltZTogc3RyaW5nKTogc3RyaW5nIHtcbiAgbGV0IGJpbmFyeSA9IFwiXCI7XG4gIGZvciAobGV0IGkgPSAwOyBpIDwgYnl0ZXMubGVuZ3RoOyBpICs9IDEpIGJpbmFyeSArPSBTdHJpbmcuZnJvbUNoYXJDb2RlKGJ5dGVzW2ldISk7XG4gIHJldHVybiBgZGF0YToke21pbWV9O2Jhc2U2NCwke2J0b2EoYmluYXJ5KX1gO1xufVxuXG4vKiog8J+Onu+4jyBENSB0b3AtbGV2ZWw6IHNvdXJjZXMgdmlkZW8gYnl0ZXMgKGBwYXlsb2FkYCBkYXRhIFVSTCwgb3IgdGhlIG5hdGl2ZSBmaWxlIHBpY2tlciB3aGVuIHVuc2V0KSxcbiAqIHRyaWVzIFRpZXIgMSB3aGVuIFdlYkNvZGVjcyBpcyBhdmFpbGFibGUgYW5kIHRoZSBkZW11eCBmaW5kcyBhIHVzYWJsZSBBVkMgdHJhY2ssIG90aGVyd2lzZSBUaWVyIDInc1xuICogYDx2aWRlbz5gIHNlZWstYW5kLWNhcHR1cmU7IG9uIHRvdGFsIGZhaWx1cmUgKGNhbid0IGRlbXV4IEFORCBUaWVyIDIgYWxzbyB0aHJvd3MsIGUuZy4gYSBjb3JydXB0XG4gKiBmaWxlKSBkaXNwYXRjaGVzIGBmYWxsYmFja0FjdGlvbmAgb25jZSB3aXRoIHRoZSByYXcgb3JpZ2luYWwgYnl0ZXMgYXMgYSBkYXRhIFVSTC4gKi9cbmV4cG9ydCBhc3luYyBmdW5jdGlvbiBydW5SZXF1ZXN0TWVkaWFGcmFtZXMoXG4gIGVmZmVjdDogUmVxdWVzdE1lZGlhRnJhbWVzQXJncyxcbiAgYWNjZXB0OiBzdHJpbmcsXG4gIHBheWxvYWQ6IHN0cmluZyB8IHVuZGVmaW5lZCxcbiAgZGlzcGF0Y2hPbmU6IEVmZmVjdERpc3BhdGNoT25lLFxuICBjcmVhdGVWaWRlb0VsZW1lbnQ6ICgpID0+IEhUTUxWaWRlb0VsZW1lbnQgPSAoKSA9PiBkb2N1bWVudC5jcmVhdGVFbGVtZW50KFwidmlkZW9cIiksXG4pOiBQcm9taXNlPHZvaWQ+IHtcbiAgbGV0IGJ5dGVzOiBVaW50OEFycmF5O1xuICBsZXQgbmFtZSA9IFwidmlkZW9cIjtcbiAgaWYgKHBheWxvYWQpIHtcbiAgICBieXRlcyA9IGJ5dGVzRnJvbURhdGFVcmwocGF5bG9hZCk7XG4gIH0gZWxzZSB7XG4gICAgY29uc3Qgb3BlbmVkID0gYXdhaXQgcmVxdWVzdEZpbGVPcGVuKGFjY2VwdCB8fCBcInZpZGVvLypcIiwgXCJkYXRhVXJsXCIsIGZhbHNlKTtcbiAgICBpZiAob3BlbmVkLmxlbmd0aCA9PT0gMCkgcmV0dXJuO1xuICAgIGJ5dGVzID0gYnl0ZXNGcm9tRGF0YVVybChvcGVuZWRbMF0hLmNvbnRlbnRzKTtcbiAgICBuYW1lID0gb3BlbmVkWzBdIS5uYW1lO1xuICB9XG4gIHRyeSB7XG4gICAgaWYgKHdlYkNvZGVjc0F2YWlsYWJsZSgpICYmIChhd2FpdCBydW5UaWVyMVZpZGVvRnJhbWVzKGJ5dGVzLCBlZmZlY3QsIG5hbWUsIGRpc3BhdGNoT25lKSkpIHJldHVybjtcbiAgICBjb25zdCB1cmwgPSBVUkwuY3JlYXRlT2JqZWN0VVJMKG5ldyBCbG9iKFtieXRlc10sIHsgdHlwZTogXCJ2aWRlby9tcDRcIiB9KSk7XG4gICAgY29uc3QgdmlkZW8gPSBjcmVhdGVWaWRlb0VsZW1lbnQoKTtcbiAgICB2aWRlby5tdXRlZCA9IHRydWU7XG4gICAgdmlkZW8ucGxheXNJbmxpbmUgPSB0cnVlO1xuICAgIHZpZGVvLnNyYyA9IHVybDtcbiAgICB0cnkge1xuICAgICAgYXdhaXQgcnVuVGllcjJWaWRlb0ZyYW1lcyh2aWRlbywgZWZmZWN0LCBuYW1lLCBkaXNwYXRjaE9uZSk7XG4gICAgfSBmaW5hbGx5IHtcbiAgICAgIFVSTC5yZXZva2VPYmplY3RVUkwodXJsKTtcbiAgICB9XG4gIH0gY2F0Y2ggKGVycm9yKSB7XG4gICAgY29uc29sZS5lcnJvcihcIltvcy1zaGVsbF0gcmVxdWVzdE1lZGlhRnJhbWVzOiBkZWNvZGUgZmFpbGVkLCBmYWxsaW5nIGJhY2sgdG8gcmF3IGJ5dGVzXCIsIGVycm9yKTtcbiAgICBhd2FpdCBkaXNwYXRjaE9uZShlZmZlY3QuZmFsbGJhY2tBY3Rpb24sIHsgcGF5bG9hZDogYnl0ZXNUb0RhdGFVcmwoYnl0ZXMsIFwidmlkZW8vbXA0XCIpLCBuYW1lLCAuLi5lZmZlY3QuYXJncyB9KTtcbiAgfVxufVxuLy8jZW5kcmVnaW9uIFJlcXVlc3RNZWRpYUZyYW1lc1xuXG5mdW5jdGlvbiBpc1N0dWRpb01vZGUocGx1Z2luRmlsdGVyPzogc3RyaW5nKTogYm9vbGVhbiB7XG4gIHJldHVybiBwbHVnaW5GaWx0ZXIgIT09IHVuZGVmaW5lZCAmJiByZXNvbHZlUGx1Z2luSG9zdENvbmZpZyhwbHVnaW5GaWx0ZXIpICE9PSB1bmRlZmluZWQ7XG59XG5cbmV4cG9ydCBpbnRlcmZhY2UgU3BhY2VTaGVsbFBhdGgge1xuICByZWFkb25seSBzcGFjZUlkOiBzdHJpbmc7XG4gIHJlYWRvbmx5IGluc3RhbmNlSWQ/OiBzdHJpbmc7XG59XG5cbmV4cG9ydCB0eXBlIFNoZWxsUm91dGUgPSB7IHJlYWRvbmx5IGtpbmQ6IFwibGFuZGluZ1wiIH0gfCB7IHJlYWRvbmx5IGtpbmQ6IFwic3BhY2VcIjsgcmVhZG9ubHkgc3BhY2VJZDogc3RyaW5nOyByZWFkb25seSBpbnN0YW5jZUlkPzogc3RyaW5nIH0gfCB7IHJlYWRvbmx5IGtpbmQ6IFwibm90Rm91bmRcIjsgcmVhZG9ubHkgcGF0aDogc3RyaW5nIH07XG5cbi8qKiBAZW1vamkg8J+nre+4jyBDbGFzc2lmaWVzIHNoZWxsIGhpc3RvcnkgcGF0aHMgaW50byBsYW5kaW5nLCBzdHVkaW8gc3BhY2UsIG9yIHVua25vd24gcm91dGVzLiAqL1xuZXhwb3J0IGZ1bmN0aW9uIHBhcnNlU2hlbGxSb3V0ZShwYXRoOiBzdHJpbmcpOiBTaGVsbFJvdXRlIHtcbiAgY29uc3Qgbm9ybWFsaXplZCA9IChwYXRoLnNwbGl0KFwiP1wiKVswXSA/PyBcIi9cIikudHJpbSgpIHx8IFwiL1wiO1xuICBpZiAobm9ybWFsaXplZCA9PT0gXCIvXCIpIHJldHVybiB7IGtpbmQ6IFwibGFuZGluZ1wiIH07XG4gIGNvbnN0IG1hdGNoID0gL15cXC9zcGFjZXNcXC8oW14vXSspKD86XFwvaW5zdGFuY2VzXFwvKFteL10rKSk/JC8uZXhlYyhub3JtYWxpemVkKTtcbiAgaWYgKG1hdGNoKSByZXR1cm4geyBraW5kOiBcInNwYWNlXCIsIHNwYWNlSWQ6IG1hdGNoWzFdISwgaW5zdGFuY2VJZDogbWF0Y2hbMl0gfTtcbiAgcmV0dXJuIHsga2luZDogXCJub3RGb3VuZFwiLCBwYXRoOiBub3JtYWxpemVkIH07XG59XG5cbi8qKiBAZGVwcmVjYXRlZCBVc2Uge0BsaW5rIHBhcnNlU2hlbGxSb3V0ZX0gaW5zdGVhZC4gKi9cbmV4cG9ydCBmdW5jdGlvbiBwYXJzZVNwYWNlU2hlbGxQYXRoKHBhdGg6IHN0cmluZyk6IFNwYWNlU2hlbGxQYXRoIHwgbnVsbCB7XG4gIGNvbnN0IHJvdXRlID0gcGFyc2VTaGVsbFJvdXRlKHBhdGgpO1xuICBpZiAocm91dGUua2luZCAhPT0gXCJzcGFjZVwiKSByZXR1cm4gbnVsbDtcbiAgcmV0dXJuIHsgc3BhY2VJZDogcm91dGUuc3BhY2VJZCwgaW5zdGFuY2VJZDogcm91dGUuaW5zdGFuY2VJZCB9O1xufVxuXG5leHBvcnQgZnVuY3Rpb24gYXBwRG9jdW1lbnRMYWJlbChkb2N1bWVudDogcmVhZG9ubHkgc3RyaW5nW10pOiBzdHJpbmcge1xuICByZXR1cm4gZG9jdW1lbnQuam9pbihBUFBfRE9DVU1FTlRfU0VQQVJBVE9SKTtcbn1cblxuLyoqIPCfl7rvuI8gUmVzb2x2ZXMgdGhlIGRvY3VtZW50IHBhdGggZWZmZWN0aXZlIHVuZGVyIHRoZSBhY3RpdmUgdGVybWlub2xvZ3k7IHVua25vd24vbmF0aXZlIGlkcyBmYWxsIGJhY2sgdG8gYGFwcC5kb2N1bWVudGAuICovXG5leHBvcnQgZnVuY3Rpb24gcmVzb2x2ZUFwcERvY3VtZW50KGFwcDogUGljazxBcHBEZWZpbml0aW9uLCBcImRvY3VtZW50XCIgfCBcInRlcm1pbm9sb2d5RG9jdW1lbnRzXCI+LCB0ZXJtaW5vbG9neTogc3RyaW5nKTogcmVhZG9ubHkgc3RyaW5nW10ge1xuICByZXR1cm4gYXBwLnRlcm1pbm9sb2d5RG9jdW1lbnRzPy5bdGVybWlub2xvZ3ldID8/IGFwcC5kb2N1bWVudDtcbn1cblxuLyoqIPCfl7rvuI8gUmVzb2x2ZXMgdGhlIGRvY3VtZW50IHBhdGggZm9yIGEgbm9uLWFjdGl2ZSBhcHAgKHN0dWRpbyBzcGF3biBwYWxldHRlL3NwYXduZWQgZW50cmllcykgYnkgbG9va2luZyB1cCBpdHMgYEFwcERlZmluaXRpb25gIGFjcm9zcyBsb2FkZWQgcGx1Z2luczsgZmFsbHMgYmFjayB0byB0aGUgcmF3IGBkb2N1bWVudGAgd2hlbiB0aGUgYXBwIGNhbid0IGJlIGZvdW5kLiAqL1xuZXhwb3J0IGZ1bmN0aW9uIHJlc29sdmVEb2N1bWVudEJ5QXBwSWQobG9hZGVkUGx1Z2luczogcmVhZG9ubHkgTG9hZGVkUHJvZ3JhbVN0YXRlW10sIGFwcElkOiBzdHJpbmcsIGRvY3VtZW50OiByZWFkb25seSBzdHJpbmdbXSwgdGVybWlub2xvZ3k6IHN0cmluZyk6IHJlYWRvbmx5IHN0cmluZ1tdIHtcbiAgZm9yIChjb25zdCBwcm9ncmFtIG9mIGxvYWRlZFBsdWdpbnMpIHtcbiAgICBjb25zdCBhcHAgPSBwcm9ncmFtLm1hbmlmZXN0LmFwcHMuZmluZCgoY2FuZGlkYXRlKSA9PiBjYW5kaWRhdGUuaWQgPT09IGFwcElkKTtcbiAgICBpZiAoYXBwKSByZXR1cm4gcmVzb2x2ZUFwcERvY3VtZW50KGFwcCwgdGVybWlub2xvZ3kpO1xuICB9XG4gIHJldHVybiBkb2N1bWVudDtcbn1cblxuZXhwb3J0IGZ1bmN0aW9uIGFwcFdpbmRvd0RvY3VtZW50TGFiZWwoYXBwOiBBcHBEZWZpbml0aW9uLCB0ZXJtaW5vbG9neTogc3RyaW5nLCB3aW5kb3dMYWJlbDogc3RyaW5nLCBsb2NhbGU6IHN0cmluZyA9IFNIRUxMX0xPQ0FMRVNbMF0pOiBzdHJpbmcge1xuICBjb25zdCB0cmltbWVkID0gd2luZG93TGFiZWwudHJpbSgpO1xuICBpZiAodHJpbW1lZCkgcmV0dXJuIHRyaW1tZWQ7XG4gIGNvbnN0IG92ZXJyaWRlID0gYXBwLnRlcm1pbm9sb2d5RG9jdW1lbnRzPy5bdGVybWlub2xvZ3ldO1xuICByZXR1cm4gb3ZlcnJpZGU/LltvdmVycmlkZS5sZW5ndGggLSAxXT8udHJpbSgpIHx8IHJlc29sdmVNYW5pZmVzdExhYmVsKGFwcC5sYWJlbCwgdGVybWlub2xvZ3ksIGxvY2FsZSkudHJpbSgpO1xufVxuXG5leHBvcnQgZnVuY3Rpb24gYnVpbGRTcGFjZVBhbmVsU3RhdGUocHJvZ3JhbXM6IHJlYWRvbmx5IFNwYWNlUHJvZ3JhbUVudHJ5W10sIHNwYXduZWRBcHBzOiByZWFkb25seSBTcGF3bmVkQXBwRW50cnlbXSwgYWN0aXZlUGFuZWxUYWIgPSBcInMtcGxheS1jYXRhbG9ndWVcIiwgYWN0aXZlU3Bhd25lZElkPzogc3RyaW5nKTogU3BhY2VQYW5lbFN0YXRlIHtcbiAgcmV0dXJuIHsgYWN0aXZlUGFuZWxUYWIsIHByb2dyYW1zLCBzcGF3bmVkQXBwcywgYWN0aXZlU3Bhd25lZElkIH07XG59XG5cbmV4cG9ydCBmdW5jdGlvbiBwYW5lbEpzb25Gcm9tU3RhdGUoc3RhdGU6IFNwYWNlUGFuZWxTdGF0ZSk6IHN0cmluZyB7XG4gIHJldHVybiBwYWNrVmFsdWVUb0Jhc2U2NChzdGF0ZSk7XG59XG5cbmV4cG9ydCBmdW5jdGlvbiBwYXJzZVBhbmVsU3RhdGUodmlld1N0YXRlOiBWaWV3TW9kZWwpOiBTcGFjZVBhbmVsU3RhdGUgfCBudWxsIHtcbiAgaWYgKCF2aWV3U3RhdGUucGFuZWxKc29uKSByZXR1cm4gbnVsbDtcbiAgdHJ5IHtcbiAgICByZXR1cm4gcGFja1ZhbHVlRnJvbUJhc2U2NCh2aWV3U3RhdGUucGFuZWxKc29uKSBhcyBTcGFjZVBhbmVsU3RhdGU7XG4gIH0gY2F0Y2gge1xuICAgIHJldHVybiBudWxsO1xuICB9XG59XG5cbi8qKlxuICogQGVtb2ppIPCfqp/vuI8gUmV0dXJucyBhIHN0dWRpbyBwYW5lbCB3aXRoIGBzcGF3bmVkYCBwcmVzZW50IGFuZCBmb2N1c2VkIGFzIGBhY3RpdmVTcGF3bmVkSWRgLlxuICogSG9zdC1lZmZlY3QgYXBwbGljYXRpb24gbXVzdCBmb2xkIHRoaXMgaW50byB0aGUgaW4tZmxpZ2h0IGBuZXh0Vmlld1N0YXRlYCBiZWZvcmUgdGhlIGZpbmFsXG4gKiBgU0VUX1NFU1NJT05gIHdyaXRlIOKAlCBhIHNlcGFyYXRlIHBhbmVsIGRpc3BhdGNoIGlzIG92ZXJ3cml0dGVuIGJ5IHRoYXQgd3JpdGUgYW5kIGxlYXZlcyB0aGUgc2hlbGxcbiAqIHN0dWNrIG9uIHRoZSBzdHVkaW8gc3VyZmFjZS5cbiAqIEBzZWUgSG9zdEVmZmVjdC5vcGVuUGx1Z2luSW5zdGFuY2VcbiAqL1xuZXhwb3J0IGZ1bmN0aW9uIHN0dWRpb1BhbmVsRm9jdXNpbmdTcGF3bmVkKHBhbmVsOiBTcGFjZVBhbmVsU3RhdGUsIHNwYXduZWQ6IFNwYXduZWRBcHBFbnRyeSk6IFNwYWNlUGFuZWxTdGF0ZSB7XG4gIGNvbnN0IHNwYXduZWRBcHBzID0gcGFuZWwuc3Bhd25lZEFwcHMuc29tZSgoZW50cnkpID0+IGVudHJ5LmlkID09PSBzcGF3bmVkLmlkKVxuICAgID8gcGFuZWwuc3Bhd25lZEFwcHMubWFwKChlbnRyeSkgPT4gKGVudHJ5LmlkID09PSBzcGF3bmVkLmlkID8gc3Bhd25lZCA6IGVudHJ5KSlcbiAgICA6IFsuLi5wYW5lbC5zcGF3bmVkQXBwcywgc3Bhd25lZF07XG4gIHJldHVybiBidWlsZFNwYWNlUGFuZWxTdGF0ZShwYW5lbC5wcm9ncmFtcywgc3Bhd25lZEFwcHMsIHBhbmVsLmFjdGl2ZVBhbmVsVGFiLCBzcGF3bmVkLmlkKTtcbn1cblxuLyoqIEBlbW9qaSDwn5Ca77iPIENvbW1pdHMgYSBzdHVkaW8gcGFuZWwgaW50byBhIHZpZXcgc3RhdGUncyBgcGFuZWxKc29uYCBmb3IgYSBzaW5nbGUgaG9zdC1lZmZlY3Qgc2Vzc2lvbiB3cml0ZS4gKi9cbmV4cG9ydCBmdW5jdGlvbiB2aWV3U3RhdGVXaXRoU3BhY2VQYW5lbCh2aWV3U3RhdGU6IFZpZXdNb2RlbCwgcGFuZWw6IFNwYWNlUGFuZWxTdGF0ZSk6IFZpZXdNb2RlbCB7XG4gIHJldHVybiB7IC4uLnZpZXdTdGF0ZSwgcGFuZWxKc29uOiBwYW5lbEpzb25Gcm9tU3RhdGUocGFuZWwpIH07XG59XG5cbi8qKiBAZW1vamkg8J+nre+4jyBEZWZhdWx0IGFuY2hvciBhIHBsdWdpbi1kZWNsYXJlZCBwYW5lbC10YWIgYGdyb3VwYCBkb2NrcyBpbnRvIOKAlCBncm91cHMgb25seSBldmVyIG1hcCB0byB0aGUgZm91ciBjb3JuZXJzOyB0aGUgZm91ciBlZGdlLW1pZGRsZSBhbmNob3JzIHN0YXJ0IGVtcHR5IGFuZCBhcmUgdXNlci1wb3B1bGF0ZWQgdmlhIGRyYWctYW5kLWRyb3Agb3IgYSBkb2NrIHNrZWxldG9uIG92ZXJyaWRlLiAqL1xuZXhwb3J0IGZ1bmN0aW9uIHBhbmVsQW5jaG9yRm9yR3JvdXAoZ3JvdXA6IHN0cmluZyk6IEFuY2hvciB7XG4gIGlmIChncm91cCA9PT0gXCJ3b3JrYmVuY2hcIiB8fCBncm91cCA9PT0gXCJkb2N1bWVudFwiKSByZXR1cm4gXCJ0b3AtbGVmdFwiO1xuICBpZiAoZ3JvdXAgPT09IFwiZGV0YWlsc1wiKSByZXR1cm4gXCJ0b3AtcmlnaHRcIjtcbiAgaWYgKGdyb3VwID09PSBcImRpc3BsYXlcIikgcmV0dXJuIFwiYm90dG9tLWxlZnRcIjtcbiAgaWYgKGdyb3VwID09PSBcInNldHRpbmdzXCIpIHJldHVybiBcImJvdHRvbS1yaWdodFwiO1xuICByZXR1cm4gXCJ0b3AtcmlnaHRcIjtcbn1cblxuLyoqIEBlbW9qaSDwn6qf77iPIE9uZSBsZWFmIGluIGEgZnJhbWV3b3JrIGxheW91dCB0cmVlLCB3aXRoIG9wdGlvbmFsIGluc3RhbmNlL3RlbXBsYXRlIGJpbmRpbmcgZm9yIG11bHRpLXBhbmUgd29ybGQgdmlld3MuICovXG50eXBlIEZyYW1ld29ya0xheW91dFdpbmRvd1NlZWQgPSB7XG4gIHJlYWRvbmx5IHdpbmRvd0lkOiBzdHJpbmc7XG4gIHJlYWRvbmx5IHdpbmRvd0tpbmRJZDogc3RyaW5nO1xuICByZWFkb25seSB0aXRsZT86IHN0cmluZztcbiAgcmVhZG9ubHkgdGVtcGxhdGVJZD86IHN0cmluZztcbiAgcmVhZG9ubHkgc2l6ZTogbnVtYmVyO1xufTtcblxuLyoqIEBlbW9qaSDwn6qf77iPIFdhbGtzIGEgZnJhbWV3b3JrIGxheW91dCBhbmQgY29sbGVjdHMgZXZlcnkgd2luZG93IGxlYWYsIHByZWZlcnJpbmcgYGluc3RhbmNlSWRgIGFzIHRoZSBsaXZlIHBhbmUgaWQuICovXG5mdW5jdGlvbiBjb2xsZWN0RnJhbWV3b3JrTGF5b3V0V2luZG93U2VlZHMobm9kZTogV2luZG93TGF5b3V0QXhpc05vZGUgfCBXaW5kb3dMYXlvdXRTdGFja05vZGUgfCBXaW5kb3dMYXlvdXRXaW5kb3dOb2RlLCBwYXJlbnRTaXplID0gMTAwKTogRnJhbWV3b3JrTGF5b3V0V2luZG93U2VlZFtdIHtcbiAgaWYgKG5vZGUua2luZCA9PT0gXCJ3aW5kb3dcIikge1xuICAgIHJldHVybiBbXG4gICAgICB7XG4gICAgICAgIHdpbmRvd0lkOiBub2RlLmluc3RhbmNlSWQgPz8gbm9kZS53aW5kb3dLaW5kSWQsXG4gICAgICAgIHdpbmRvd0tpbmRJZDogbm9kZS53aW5kb3dLaW5kSWQsXG4gICAgICAgIHRpdGxlOiBub2RlLnRpdGxlLFxuICAgICAgICB0ZW1wbGF0ZUlkOiBub2RlLnRlbXBsYXRlSWQsXG4gICAgICAgIHNpemU6IHBhcmVudFNpemUsXG4gICAgICB9LFxuICAgIF07XG4gIH1cbiAgaWYgKG5vZGUua2luZCA9PT0gXCJzdGFja1wiKSB7XG4gICAgY29uc3Qgc2l6ZSA9IG5vZGUuc2l6ZSA/PyBwYXJlbnRTaXplO1xuICAgIHJldHVybiBub2RlLmNoaWxkcmVuLm1hcCgoY2hpbGQpID0+ICh7XG4gICAgICB3aW5kb3dJZDogY2hpbGQuaW5zdGFuY2VJZCA/PyBjaGlsZC53aW5kb3dLaW5kSWQsXG4gICAgICB3aW5kb3dLaW5kSWQ6IGNoaWxkLndpbmRvd0tpbmRJZCxcbiAgICAgIHRpdGxlOiBjaGlsZC50aXRsZSxcbiAgICAgIHRlbXBsYXRlSWQ6IGNoaWxkLnRlbXBsYXRlSWQsXG4gICAgICBzaXplLFxuICAgIH0pKTtcbiAgfVxuICBjb25zdCBjaGlsZFNpemVzID0gbm9kZS5jaGlsZHJlbi5tYXAoKGNoaWxkKSA9PiAoXCJzaXplXCIgaW4gY2hpbGQgPyBjaGlsZC5zaXplIDogdW5kZWZpbmVkKSk7XG4gIGNvbnN0IGV4cGxpY2l0VG90YWwgPSBjaGlsZFNpemVzLnJlZHVjZTxudW1iZXI+KChzdW0sIHNpemUpID0+IHN1bSArIChzaXplID8/IDApLCAwKTtcbiAgY29uc3QgdW5zZXRDb3VudCA9IGNoaWxkU2l6ZXMuZmlsdGVyKChzaXplKSA9PiBzaXplID09PSB1bmRlZmluZWQpLmxlbmd0aDtcbiAgY29uc3QgZGVmYXVsdEVhY2ggPSB1bnNldENvdW50ID4gMCA/IE1hdGgubWF4KDAsIDEwMCAtIGV4cGxpY2l0VG90YWwpIC8gdW5zZXRDb3VudCA6IDA7XG4gIHJldHVybiBub2RlLmNoaWxkcmVuLmZsYXRNYXAoKGNoaWxkLCBpbmRleCkgPT4ge1xuICAgIGNvbnN0IGZyYWN0aW9uID0gY2hpbGRTaXplc1tpbmRleF0gPz8gZGVmYXVsdEVhY2g7XG4gICAgcmV0dXJuIGNvbGxlY3RGcmFtZXdvcmtMYXlvdXRXaW5kb3dTZWVkcyhjaGlsZCwgcGFyZW50U2l6ZSAqIChmcmFjdGlvbiAvIDEwMCkpO1xuICB9KTtcbn1cblxuLyoqIPCfl6PvuI8gRm9yIGEgc2luZ2xlLCBub24taW5zdGFuY2VkIHdpbmRvdyAoYGluc3RhbmNlSWRgIHVuc2V0IOKAlCB0aGUgY29tbW9uIGNhc2UsIG9uZSB3aW5kb3cgcGVyIGtpbmQpLFxuICogdGhlIGB3aW5kb3dLaW5kYCdzIG93biBsYWJlbCBpcyB0aGUgc2luZ2xlIHNvdXJjZSBvZiB0cnV0aDogYSBtYW5pZmVzdC1iYWtlZCBgV2luZG93TGF5b3V0V2luZG93Tm9kZS50aXRsZWBcbiAqIChmcm9tIGEgcGx1Z2luJ3MgYGNyZWF0ZV9kZWZhdWx0X2xheW91dCguLi4sIHRpdGxlcylgIGNhbGwpIGlzIGEgcGxhaW4sIGxvY2FsZS1pbnZhcmlhbnQgc3RyaW5nIHRoYXRcbiAqIHByZWRhdGVzIGxvY2FsZS90ZXJtaW5vbG9neSByZXNvbHV0aW9uIGVudGlyZWx5LCBzbyBpdCBtdXN0IG5ldmVyIHdpbiBvdmVyIGEgcmVhbCBgd2luZG93S2luZHNgIGxvb2t1cCDigJRcbiAqIGl0IG9ubHkgc3Vydml2ZXMgYXMgYSBsYXN0LXJlc29ydCBmYWxsYmFjayBmb3IgYSB3aW5kb3cga2luZCBpZCB0aGF0IGlzbid0IGRlY2xhcmVkIGluIHRoZSBtYW5pZmVzdFxuICogKG1pcnJvcnMge0BsaW5rIHJldGl0bGVXaW5kb3dMYXlvdXROb2RlfSdzIGFscmVhZHktY29ycmVjdCBwcmVjZWRlbmNlKS4gRm9yIGEgbXVsdGktaW5zdGFuY2Ugd2luZG93XG4gKiAoYGluc3RhbmNlSWRgIHNldCDigJQgc2V2ZXJhbCB2aWV3cyBzaGFyaW5nIG9uZSBgd2luZG93S2luZGAsIGUuZy4gXCJUb3BcIi9cIlBlcnNwZWN0aXZlXCIgYm90aCBiYWNrZWQgYnkgYVxuICogc2luZ2xlIDNELXZpZXdwb3J0IGtpbmQpLCB0aGUgc2hhcmVkIGtpbmQgbGFiZWwgY2FuJ3QgZGlzdGluZ3Vpc2ggaW5zdGFuY2VzLCBzbyB0aGUgYmFrZWQgcGVyLWluc3RhbmNlXG4gKiB0aXRsZSBpcyB0aGUgcmVhbCB0aXRsZSBhbmQgbXVzdCB3aW4gaW5zdGVhZC4gKi9cbmZ1bmN0aW9uIHJlc29sdmVGcmFtZXdvcmtXaW5kb3dUaXRsZShcbiAgd2luZG93S2luZElkOiBzdHJpbmcsXG4gIGluc3RhbmNlSWQ6IHN0cmluZyB8IHVuZGVmaW5lZCxcbiAgYmFrZWRUaXRsZTogc3RyaW5nIHwgdW5kZWZpbmVkLFxuICB3aW5kb3dLaW5kczogcmVhZG9ubHkgeyByZWFkb25seSBpZDogc3RyaW5nOyByZWFkb25seSBsYWJlbDogTG9jYWxpemVkTGFiZWwgfCBzdHJpbmcgfVtdLFxuICB0ZXJtaW5vbG9neTogc3RyaW5nLFxuICBsb2NhbGU6IHN0cmluZyxcbik6IHN0cmluZyB7XG4gIGlmIChpbnN0YW5jZUlkKSByZXR1cm4gYmFrZWRUaXRsZSA/PyB3aW5kb3dLaW5kSWQ7XG4gIGNvbnN0IGtpbmQgPSB3aW5kb3dLaW5kcy5maW5kKChlbnRyeSkgPT4gZW50cnkuaWQgPT09IHdpbmRvd0tpbmRJZCk7XG4gIHJldHVybiBraW5kID8gcmVzb2x2ZU1hbmlmZXN0TGFiZWwoa2luZC5sYWJlbCwgdGVybWlub2xvZ3ksIGxvY2FsZSkgOiAoYmFrZWRUaXRsZSA/PyB3aW5kb3dLaW5kSWQpO1xufVxuXG5mdW5jdGlvbiBjb252ZXJ0RnJhbWV3b3JrTGF5b3V0Tm9kZVRvTW9kZUxheW91dChcbiAgbm9kZTogV2luZG93TGF5b3V0QXhpc05vZGUgfCBXaW5kb3dMYXlvdXRTdGFja05vZGUgfCBXaW5kb3dMYXlvdXRXaW5kb3dOb2RlLFxuICBhcHBMYWJlbHNPdmVybGF5OiBQbHVnaW5BcHBMYWJlbHNPdmVybGF5LFxuICB3aW5kb3dLaW5kczogcmVhZG9ubHkgeyByZWFkb25seSBpZDogc3RyaW5nOyByZWFkb25seSBsYWJlbDogTG9jYWxpemVkTGFiZWwgfCBzdHJpbmcgfVtdLFxuICB0ZXJtaW5vbG9neTogc3RyaW5nLFxuICBsb2NhbGU6IHN0cmluZyxcbik6IFdpbmRvd0xheW91dE5vZGUge1xuICBpZiAobm9kZS5raW5kID09PSBcIndpbmRvd1wiKSB7XG4gICAgY29uc3QgaWQgPSBub2RlLmluc3RhbmNlSWQgPz8gbm9kZS53aW5kb3dLaW5kSWQ7XG4gICAgY29uc3QgdGl0bGUgPSByZXNvbHZlRnJhbWV3b3JrV2luZG93VGl0bGUobm9kZS53aW5kb3dLaW5kSWQsIG5vZGUuaW5zdGFuY2VJZCwgbm9kZS50aXRsZSwgd2luZG93S2luZHMsIHRlcm1pbm9sb2d5LCBsb2NhbGUpO1xuICAgIHJldHVybiB7IGtpbmQ6IFwid2luZG93XCIsIGlkLCB0aXRsZTogd2lyZUxhYmVsKHJlc29sdmVBcHBMYWJlbChhcHBMYWJlbHNPdmVybGF5LCBcIndpbmRvd0tpbmRcIiwgaWQsIHRpdGxlKSkgfTtcbiAgfVxuICBpZiAobm9kZS5raW5kID09PSBcInN0YWNrXCIpIHtcbiAgICByZXR1cm4ge1xuICAgICAga2luZDogXCJzdGFja1wiLFxuICAgICAgc2l6ZTogbm9kZS5zaXplLFxuICAgICAgY2hpbGRyZW46IG5vZGUuY2hpbGRyZW4ubWFwKChjaGlsZCkgPT4ge1xuICAgICAgICBjb25zdCBpZCA9IGNoaWxkLmluc3RhbmNlSWQgPz8gY2hpbGQud2luZG93S2luZElkO1xuICAgICAgICBjb25zdCB0aXRsZSA9IHJlc29sdmVGcmFtZXdvcmtXaW5kb3dUaXRsZShjaGlsZC53aW5kb3dLaW5kSWQsIGNoaWxkLmluc3RhbmNlSWQsIGNoaWxkLnRpdGxlLCB3aW5kb3dLaW5kcywgdGVybWlub2xvZ3ksIGxvY2FsZSk7XG4gICAgICAgIHJldHVybiB7XG4gICAgICAgICAga2luZDogXCJ3aW5kb3dcIiBhcyBjb25zdCxcbiAgICAgICAgICBpZCxcbiAgICAgICAgICB0aXRsZTogd2lyZUxhYmVsKHJlc29sdmVBcHBMYWJlbChhcHBMYWJlbHNPdmVybGF5LCBcIndpbmRvd0tpbmRcIiwgaWQsIHRpdGxlKSksXG4gICAgICAgIH07XG4gICAgICB9KSxcbiAgICB9O1xuICB9XG4gIHJldHVybiB7XG4gICAga2luZDogbm9kZS5raW5kLFxuICAgIHNpemU6IG5vZGUuc2l6ZSxcbiAgICBjaGlsZHJlbjogbm9kZS5jaGlsZHJlbi5tYXAoKGNoaWxkKSA9PiBjb252ZXJ0RnJhbWV3b3JrTGF5b3V0Tm9kZVRvTW9kZUxheW91dChjaGlsZCwgYXBwTGFiZWxzT3ZlcmxheSwgd2luZG93S2luZHMsIHRlcm1pbm9sb2d5LCBsb2NhbGUpKSxcbiAgfTtcbn1cblxuLyoqIEBlbW9qaSDwn5ej77iPIFJlLXJlc29sdmVzIGV2ZXJ5IHdpbmRvdydzIHRpdGxlIGZyb20gdGhlIGFwcCBtYW5pZmVzdCdzIHdpbmRvd0tpbmRzIHZpYSByZXNvbHZlTWFuaWZlc3RMYWJlbCBpbiBwbGFjZSwgcHJlc2VydmluZyB0aGUgdHJlZSdzIHN0cnVjdHVyZS9zaXplcy9hcnJhbmdlbWVudCDigJQgdXNlZCB0byByZWFjdCB0byBhIGxvY2FsZS90ZXJtaW5vbG9neSBzd2l0Y2ggd2l0aG91dCBkaXNjYXJkaW5nIHRoZSB1c2VyJ3MgbGl2ZSBsYXlvdXQuICovXG5leHBvcnQgZnVuY3Rpb24gcmV0aXRsZVdpbmRvd0xheW91dE5vZGUoXG4gIG5vZGU6IFdpbmRvd0xheW91dE5vZGUsXG4gIHdpbmRvd0tpbmRzOiByZWFkb25seSB7IHJlYWRvbmx5IGlkOiBzdHJpbmc7IHJlYWRvbmx5IGxhYmVsOiBMb2NhbGl6ZWRMYWJlbCB8IHN0cmluZyB9W10sXG4gIGV4dHJhSW5zdGFuY2VzOiByZWFkb25seSBFeHRyYVdpbmRvd0luc3RhbmNlW10sXG4gIHRlcm1pbm9sb2d5OiBzdHJpbmcsXG4gIGxvY2FsZTogc3RyaW5nLFxuKTogV2luZG93TGF5b3V0Tm9kZSB7XG4gIGlmIChub2RlLmtpbmQgPT09IFwid2luZG93XCIpIHtcbiAgICBjb25zdCBleHRyYSA9IGV4dHJhSW5zdGFuY2VzLmZpbmQoKGVudHJ5KSA9PiBlbnRyeS5pZCA9PT0gbm9kZS5pZCk7XG4gICAgY29uc3Qgd2luZG93S2luZElkID0gZXh0cmEgPyBleHRyYS53aW5kb3dLaW5kSWQgOiBub2RlLmlkO1xuICAgIGNvbnN0IGtpbmQgPSB3aW5kb3dLaW5kcy5maW5kKChlbnRyeSkgPT4gZW50cnkuaWQgPT09IHdpbmRvd0tpbmRJZCk7XG4gICAgY29uc3QgdGl0bGUgPSBraW5kID8gd2lyZUxhYmVsKHJlc29sdmVNYW5pZmVzdExhYmVsKGtpbmQubGFiZWwsIHRlcm1pbm9sb2d5LCBsb2NhbGUpKSA6IChub2RlLnRpdGxlID8/IHVpRGF0YUxhYmVsKG5vZGUuaWQpKTtcbiAgICByZXR1cm4geyAuLi5ub2RlLCB0aXRsZSB9O1xuICB9XG4gIHJldHVybiB7XG4gICAgLi4ubm9kZSxcbiAgICBjaGlsZHJlbjogbm9kZS5jaGlsZHJlbi5tYXAoKGNoaWxkKSA9PiByZXRpdGxlV2luZG93TGF5b3V0Tm9kZShjaGlsZCwgd2luZG93S2luZHMsIGV4dHJhSW5zdGFuY2VzLCB0ZXJtaW5vbG9neSwgbG9jYWxlKSksXG4gIH0gYXMgV2luZG93TGF5b3V0Tm9kZTtcbn1cblxuLyoqIEBlbW9qaSDwn6qf77iPIFJlc29sdmVzIGEgZnJhbWV3b3JrIGxheW91dCBpbnRvIHRoZSBsaXZlIG1vZGUgdHJlZSwgZXh0cmEgaW5zdGFuY2VzLCBhbmQgcGVuZGluZyBwcm9qZWN0aW9uIHRlbXBsYXRlcyB3aXRob3V0IGluZmVycmluZyB3aW5kb3cgZm9jdXMgKG5vIHNpZGUgZWZmZWN0cykuICovXG5leHBvcnQgZnVuY3Rpb24gcmVzb2x2ZUZyYW1ld29ya0xheW91dFNlZWQoXG4gIGxheW91dDogV2luZG93TGF5b3V0IHwgdW5kZWZpbmVkLFxuICB3aW5kb3dLaW5kczogcmVhZG9ubHkgeyByZWFkb25seSBpZDogc3RyaW5nOyByZWFkb25seSBsYWJlbDogTG9jYWxpemVkTGFiZWwgfCBzdHJpbmcgfVtdLFxuICBhcHBMYWJlbHNPdmVybGF5OiBQbHVnaW5BcHBMYWJlbHNPdmVybGF5LFxuICB0ZXJtaW5vbG9neTogc3RyaW5nLFxuICBsb2NhbGU6IHN0cmluZyxcbik6IHtcbiAgcmVhZG9ubHkgbW9kZUxheW91dDogV2luZG93TGF5b3V0Tm9kZTtcbiAgcmVhZG9ubHkgZXh0cmFJbnN0YW5jZXM6IHJlYWRvbmx5IEV4dHJhV2luZG93SW5zdGFuY2VbXTtcbiAgcmVhZG9ubHkgcGVuZGluZ1Byb2plY3Rpb25zOiByZWFkb25seSB7IHJlYWRvbmx5IHdpbmRvd0lkOiBzdHJpbmc7IHJlYWRvbmx5IHRlbXBsYXRlSWQ6IHN0cmluZyB9W107XG59IHtcbiAgY29uc3Qgd2luZG93SWRzID0gd2luZG93S2luZHMubWFwKChraW5kKSA9PiBraW5kLmlkKTtcbiAgaWYgKCFsYXlvdXQ/LnJvb3QpIHtcbiAgICByZXR1cm4ge1xuICAgICAgbW9kZUxheW91dDogY3JlYXRlRXZlbldpbmRvd0xheW91dCh3aW5kb3dJZHMubGVuZ3RoID8gd2luZG93SWRzIDogW1wibWFpblwiXSksXG4gICAgICBleHRyYUluc3RhbmNlczogW10sXG4gICAgICBwZW5kaW5nUHJvamVjdGlvbnM6IFtdLFxuICAgIH07XG4gIH1cbiAgY29uc3Qgc2VlZHMgPSBjb2xsZWN0RnJhbWV3b3JrTGF5b3V0V2luZG93U2VlZHMobGF5b3V0LnJvb3QpO1xuICBjb25zdCBraW5kQnlJZCA9IG5ldyBNYXAod2luZG93S2luZHMubWFwKChraW5kKSA9PiBba2luZC5pZCwga2luZF0gYXMgY29uc3QpKTtcbiAgY29uc3QgZXh0cmFJbnN0YW5jZXM6IEV4dHJhV2luZG93SW5zdGFuY2VbXSA9IFtdO1xuICBjb25zdCBwZW5kaW5nUHJvamVjdGlvbnM6IHsgcmVhZG9ubHkgd2luZG93SWQ6IHN0cmluZzsgcmVhZG9ubHkgdGVtcGxhdGVJZDogc3RyaW5nIH1bXSA9IFtdO1xuICBmb3IgKGNvbnN0IHNlZWQgb2Ygc2VlZHMpIHtcbiAgICBjb25zdCBraW5kID0ga2luZEJ5SWQuZ2V0KHNlZWQud2luZG93S2luZElkKTtcbiAgICBpZiAoIWtpbmQpIGNvbnRpbnVlO1xuICAgIGlmIChzZWVkLndpbmRvd0lkICE9PSBzZWVkLndpbmRvd0tpbmRJZCkge1xuICAgICAgZXh0cmFJbnN0YW5jZXMucHVzaCh7XG4gICAgICAgIGlkOiBzZWVkLndpbmRvd0lkLFxuICAgICAgICB3aW5kb3dLaW5kSWQ6IHNlZWQud2luZG93S2luZElkLFxuICAgICAgICB0aXRsZTogcmVzb2x2ZUFwcExhYmVsKGFwcExhYmVsc092ZXJsYXksIFwid2luZG93S2luZFwiLCBzZWVkLndpbmRvd0lkLCBzZWVkLnRpdGxlID8/IHJlc29sdmVNYW5pZmVzdExhYmVsKGtpbmQubGFiZWwsIHRlcm1pbm9sb2d5LCBsb2NhbGUpKSxcbiAgICAgIH0pO1xuICAgIH1cbiAgICBpZiAoc2VlZC50ZW1wbGF0ZUlkKSBwZW5kaW5nUHJvamVjdGlvbnMucHVzaCh7IHdpbmRvd0lkOiBzZWVkLndpbmRvd0lkLCB0ZW1wbGF0ZUlkOiBzZWVkLnRlbXBsYXRlSWQgfSk7XG4gIH1cbiAgcmV0dXJuIHtcbiAgICBtb2RlTGF5b3V0OiBjb252ZXJ0RnJhbWV3b3JrTGF5b3V0Tm9kZVRvTW9kZUxheW91dChsYXlvdXQucm9vdCwgYXBwTGFiZWxzT3ZlcmxheSwgd2luZG93S2luZHMsIHRlcm1pbm9sb2d5LCBsb2NhbGUpLFxuICAgIGV4dHJhSW5zdGFuY2VzLFxuICAgIHBlbmRpbmdQcm9qZWN0aW9ucyxcbiAgfTtcbn1cblxuLyoqIEBlbW9qaSDwn6qf77iPIEFwcGxpZXMgYSByZXNvbHZlZCBmcmFtZXdvcmsgbGF5b3V0IHNlZWQ6IHJlZ2lzdGVycyBvbmUtc2hvdCB3b3JsZCBwcm9qZWN0aW9ucywgdGhlbiByZXR1cm5zIHRoZSBsaXZlIGxheW91dCBwYXlsb2FkLiAqL1xuZXhwb3J0IGZ1bmN0aW9uIGFwcGx5RnJhbWV3b3JrTGF5b3V0U2VlZChcbiAgbGF5b3V0OiBXaW5kb3dMYXlvdXQgfCB1bmRlZmluZWQsXG4gIHdpbmRvd0tpbmRzOiByZWFkb25seSB7IHJlYWRvbmx5IGlkOiBzdHJpbmc7IHJlYWRvbmx5IGxhYmVsOiBMb2NhbGl6ZWRMYWJlbCB8IHN0cmluZyB9W10sXG4gIGFwcExhYmVsc092ZXJsYXk6IFBsdWdpbkFwcExhYmVsc092ZXJsYXksXG4gIHRlcm1pbm9sb2d5OiBzdHJpbmcsXG4gIGxvY2FsZTogc3RyaW5nLFxuKToge1xuICByZWFkb25seSBtb2RlTGF5b3V0OiBXaW5kb3dMYXlvdXROb2RlO1xuICByZWFkb25seSBleHRyYUluc3RhbmNlczogcmVhZG9ubHkgRXh0cmFXaW5kb3dJbnN0YW5jZVtdO1xufSB7XG4gIGNvbnN0IHNlZWQgPSByZXNvbHZlRnJhbWV3b3JrTGF5b3V0U2VlZChsYXlvdXQsIHdpbmRvd0tpbmRzLCBhcHBMYWJlbHNPdmVybGF5LCB0ZXJtaW5vbG9neSwgbG9jYWxlKTtcbiAgZm9yIChjb25zdCBwZW5kaW5nIG9mIHNlZWQucGVuZGluZ1Byb2plY3Rpb25zKSB7XG4gICAgY29uc3QgcHJvamVjdGlvblNwZWMgPSBkZWNvZGVXb3JsZFByb2plY3Rpb25UZW1wbGF0ZUlkKHBlbmRpbmcudGVtcGxhdGVJZCk7XG4gICAgaWYgKHByb2plY3Rpb25TcGVjKSByZWdpc3RlclBlbmRpbmdXb3JsZFByb2plY3Rpb24ocGVuZGluZy53aW5kb3dJZCwgcHJvamVjdGlvblNwZWMpO1xuICB9XG4gIHJldHVybiB7IG1vZGVMYXlvdXQ6IHNlZWQubW9kZUxheW91dCwgZXh0cmFJbnN0YW5jZXM6IHNlZWQuZXh0cmFJbnN0YW5jZXMgfTtcbn1cblxuZnVuY3Rpb24gbW9kZUxheW91dE5vZGVUb0ZyYW1ld29yayhub2RlOiBXaW5kb3dMYXlvdXROb2RlLCBraW5kQnlJbnN0YW5jZUlkOiBSZWFkb25seU1hcDxzdHJpbmcsIHN0cmluZz4pOiBXaW5kb3dMYXlvdXRBeGlzTm9kZSB8IFdpbmRvd0xheW91dFN0YWNrTm9kZSB8IFdpbmRvd0xheW91dFdpbmRvd05vZGUge1xuICBpZiAobm9kZS5raW5kID09PSBcIndpbmRvd1wiKSB7XG4gICAgY29uc3Qgd2luZG93S2luZElkID0ga2luZEJ5SW5zdGFuY2VJZC5nZXQobm9kZS5pZCkgPz8gbm9kZS5pZDtcbiAgICBjb25zdCBpbnN0YW5jZUlkID0ga2luZEJ5SW5zdGFuY2VJZC5oYXMobm9kZS5pZCkgPyBub2RlLmlkIDogdW5kZWZpbmVkO1xuICAgIHJldHVybiB7XG4gICAgICBraW5kOiBcIndpbmRvd1wiLFxuICAgICAgd2luZG93S2luZElkLFxuICAgICAgLi4uKG5vZGUudGl0bGUgPyB7IHRpdGxlOiBub2RlLnRpdGxlIH0gOiB7fSksXG4gICAgICAuLi4oaW5zdGFuY2VJZCA/IHsgaW5zdGFuY2VJZCB9IDoge30pLFxuICAgIH07XG4gIH1cbiAgaWYgKG5vZGUua2luZCA9PT0gXCJzdGFja1wiKSB7XG4gICAgcmV0dXJuIHtcbiAgICAgIGtpbmQ6IFwic3RhY2tcIixcbiAgICAgIC4uLihub2RlLnNpemUgIT09IHVuZGVmaW5lZCA/IHsgc2l6ZTogbm9kZS5zaXplIH0gOiB7fSksXG4gICAgICBjaGlsZHJlbjogbm9kZS5jaGlsZHJlbi5tYXAoKGNoaWxkKSA9PiB7XG4gICAgICAgIGNvbnN0IHdpbmRvd0tpbmRJZCA9IGtpbmRCeUluc3RhbmNlSWQuZ2V0KGNoaWxkLmlkKSA/PyBjaGlsZC5pZDtcbiAgICAgICAgY29uc3QgaW5zdGFuY2VJZCA9IGtpbmRCeUluc3RhbmNlSWQuaGFzKGNoaWxkLmlkKSA/IGNoaWxkLmlkIDogdW5kZWZpbmVkO1xuICAgICAgICByZXR1cm4ge1xuICAgICAgICAgIGtpbmQ6IFwid2luZG93XCIgYXMgY29uc3QsXG4gICAgICAgICAgd2luZG93S2luZElkLFxuICAgICAgICAgIC4uLihjaGlsZC50aXRsZSA/IHsgdGl0bGU6IGNoaWxkLnRpdGxlIH0gOiB7fSksXG4gICAgICAgICAgLi4uKGluc3RhbmNlSWQgPyB7IGluc3RhbmNlSWQgfSA6IHt9KSxcbiAgICAgICAgfTtcbiAgICAgIH0pLFxuICAgIH07XG4gIH1cbiAgcmV0dXJuIHtcbiAgICBraW5kOiBub2RlLmtpbmQsXG4gICAgLi4uKG5vZGUuc2l6ZSAhPT0gdW5kZWZpbmVkID8geyBzaXplOiBub2RlLnNpemUgfSA6IHt9KSxcbiAgICBjaGlsZHJlbjogbm9kZS5jaGlsZHJlbi5tYXAoKGNoaWxkKSA9PiBtb2RlTGF5b3V0Tm9kZVRvRnJhbWV3b3JrKGNoaWxkLCBraW5kQnlJbnN0YW5jZUlkKSBhcyBXaW5kb3dMYXlvdXRTdGFja05vZGUgfCBXaW5kb3dMYXlvdXRBeGlzTm9kZSksXG4gIH07XG59XG5cbmV4cG9ydCBmdW5jdGlvbiBjYXB0dXJlQ3VycmVudEZyYW1ld29ya0xheW91dChzaGVsbExheW91dDogV2luZG93TGF5b3V0Tm9kZSB8IG51bGwsIGV4dHJhV2luZG93SW5zdGFuY2VzOiByZWFkb25seSBFeHRyYVdpbmRvd0luc3RhbmNlW10sIGZhbGxiYWNrPzogV2luZG93TGF5b3V0KTogV2luZG93TGF5b3V0IHwgdW5kZWZpbmVkIHtcbiAgaWYgKCFzaGVsbExheW91dCkgcmV0dXJuIGZhbGxiYWNrO1xuICBjb25zdCBraW5kQnlJbnN0YW5jZUlkID0gbmV3IE1hcChleHRyYVdpbmRvd0luc3RhbmNlcy5tYXAoKGVudHJ5KSA9PiBbZW50cnkuaWQsIGVudHJ5LndpbmRvd0tpbmRJZF0gYXMgY29uc3QpKTtcbiAgY29uc3Qgcm9vdCA9IG1vZGVMYXlvdXROb2RlVG9GcmFtZXdvcmsoc2hlbGxMYXlvdXQsIGtpbmRCeUluc3RhbmNlSWQpO1xuICBpZiAocm9vdC5raW5kID09PSBcIndpbmRvd1wiKSByZXR1cm4geyByb290OiB7IGtpbmQ6IFwic3RhY2tcIiwgY2hpbGRyZW46IFtyb290XSB9IH07XG4gIHJldHVybiB7IHJvb3QgfTtcbn1cblxuLy8jcmVnaW9uIFdpbmRvd0xheW91dENoYW5nZUNsYXNzaWZ5XG4vKiog8J+qn++4jyBUcmFpbGluZyBzZXR0bGUgZGVsYXkgZm9yIGBNb2RlLm9uTGF5b3V0Q2hhbmdlYCAoZmlyZXMgY29udGludW91c2x5IGR1cmluZyBhIGxpdmUgZHJhZy9yZXNpemUpIGJlZm9yZVxuICogbm90aW5nIG9uZSBgc2hlbGwud2luZG93UmVzaXplYC9gc2hlbGwud2luZG93TW92ZWAgY29tbWFuZCBmb3IgdGhlIHdob2xlIGdlc3R1cmUg4oCUIG1hdGNoZXMgQm9hcmQyZEhvc3Qnc1xuICogb3duIGNhbWVyYS1zeW5jIHNldHRsZSBkZWJvdW5jZSAoYGJlZ2luQ2FtZXJhSW50ZXJhY3Rpb25gKSwgdGhlIG9ubHkgcHJlY2VkZW50IGZvciB0aGlzIGtpbmQgb2ZcbiAqIGRyYWctc2V0dGxlIHBhdHRlcm4gYWxyZWFkeSBpbiB0aGlzIGZpbGUuICovXG5leHBvcnQgY29uc3QgTEFZT1VUX0NIQU5HRV9TRVRUTEVfTVMgPSAzNTA7XG5cbi8qKiDwn6qf77iPIFJlY3Vyc2l2ZSBza2VsZXRvbiBvZiBhIHtAbGluayBXaW5kb3dMYXlvdXROb2RlfSDigJQga2luZC9pZC9uZXN0aW5nIG9ubHksIHN0cmlwcGluZyBgc2l6ZWAgKHJlc2l6ZSkgYW5kXG4gKiBhIHN0YWNrJ3MgYGFjdGl2ZUlkYCAobWVyZSBmb2N1cyBlY2hvKSDigJQgc28gdHdvIHRyZWVzIGNvbXBhcmUgZXF1YWwgaGVyZSBpZmYgbmVpdGhlciBkaWZmZXJzLiAqL1xudHlwZSBXaW5kb3dMYXlvdXRTa2VsZXRvbk5vZGUgPSB7IHJlYWRvbmx5IGtpbmQ6IHN0cmluZzsgcmVhZG9ubHkgaWQ/OiBzdHJpbmc7IHJlYWRvbmx5IGNoaWxkcmVuPzogcmVhZG9ubHkgV2luZG93TGF5b3V0U2tlbGV0b25Ob2RlW10gfTtcbmZ1bmN0aW9uIHdpbmRvd0xheW91dFNrZWxldG9uKG5vZGU6IFdpbmRvd0xheW91dE5vZGUpOiBXaW5kb3dMYXlvdXRTa2VsZXRvbk5vZGUge1xuICBpZiAobm9kZS5raW5kID09PSBcIndpbmRvd1wiKSByZXR1cm4geyBraW5kOiBub2RlLmtpbmQsIGlkOiBub2RlLmlkIH07XG4gIHJldHVybiB7IGtpbmQ6IG5vZGUua2luZCwgY2hpbGRyZW46IG5vZGUuY2hpbGRyZW4ubWFwKChjaGlsZCkgPT4gd2luZG93TGF5b3V0U2tlbGV0b24oY2hpbGQgYXMgV2luZG93TGF5b3V0Tm9kZSkpIH07XG59XG5cbi8qKiDwn6qf77iPIExpa2Uge0BsaW5rIHdpbmRvd0xheW91dFNrZWxldG9ufSBidXQga2VlcHMgZWFjaCBub2RlJ3MgYHNpemVgIChzdGlsbCBpZ25vcmVzIGEgc3RhY2sncyBgYWN0aXZlSWRgKSDigJRcbiAqIGNvbXBhcmluZyB0d28gb2YgdGhlc2UgKGFmdGVyIHRoZWlyIHBsYWluIHNrZWxldG9ucyBhbHJlYWR5IG1hdGNoZWQpIGlzIGhvdyB7QGxpbmsgY2xhc3NpZnlXaW5kb3dMYXlvdXRDaGFuZ2V9XG4gKiB0ZWxscyBhIHB1cmUgcmVzaXplIGFwYXJ0IGZyb20gbm8gY2hhbmdlIGF0IGFsbC4gKi9cbnR5cGUgV2luZG93TGF5b3V0U2l6ZWRTa2VsZXRvbk5vZGUgPSB7IHJlYWRvbmx5IGtpbmQ6IHN0cmluZzsgcmVhZG9ubHkgaWQ/OiBzdHJpbmc7IHJlYWRvbmx5IHNpemU/OiBudW1iZXI7IHJlYWRvbmx5IGNoaWxkcmVuPzogcmVhZG9ubHkgV2luZG93TGF5b3V0U2l6ZWRTa2VsZXRvbk5vZGVbXSB9O1xuZnVuY3Rpb24gd2luZG93TGF5b3V0U2l6ZWRTa2VsZXRvbihub2RlOiBXaW5kb3dMYXlvdXROb2RlKTogV2luZG93TGF5b3V0U2l6ZWRTa2VsZXRvbk5vZGUge1xuICBpZiAobm9kZS5raW5kID09PSBcIndpbmRvd1wiKSByZXR1cm4geyBraW5kOiBub2RlLmtpbmQsIGlkOiBub2RlLmlkLCBzaXplOiBub2RlLnNpemUgfTtcbiAgcmV0dXJuIHsga2luZDogbm9kZS5raW5kLCBzaXplOiBub2RlLnNpemUsIGNoaWxkcmVuOiBub2RlLmNoaWxkcmVuLm1hcCgoY2hpbGQpID0+IHdpbmRvd0xheW91dFNpemVkU2tlbGV0b24oY2hpbGQgYXMgV2luZG93TGF5b3V0Tm9kZSkpIH07XG59XG5cbi8qKiDwn6qf77iPIENsYXNzaWZpZXMgYSBgTW9kZS5vbkxheW91dENoYW5nZWAgZGVsdGEgYnkgY29tcGFyaW5nIHRoZSBwcmV2aW91cyBhbmQgbmV4dCBsYXlvdXQgdHJlZSDigJQgYFwicmVhcnJhbmdlXCJgXG4gKiB3aGVuIHdpbmRvdyBpZHMvbmVzdGluZyBzdHJ1Y3R1cmUgZGlmZmVyIChkcmFnLXRvLW5ldy1wb3NpdGlvbiwgc3BsaXQsIGNsb3NlKSwgYFwicmVzaXplXCJgIHdoZW4gb25seSBwYW5lXG4gKiBzaXplcyBkaWZmZXIsIGBudWxsYCB3aGVuIG5laXRoZXIgZGlmZmVycyAoYSBwdXJlIGFjdGl2ZS13aW5kb3ctZmxhZyBlY2hvLCBoYW5kbGVkIGJ5IHRoZSBkZWRpY2F0ZWRcbiAqIGFjdGl2ZS13aW5kb3cgc2VhbSBpbnN0ZWFkIOKAlCBuZXZlciB3b3J0aCBpdHMgb3duIHNoZWxsIGNvbW1hbmQpLiAqL1xuZXhwb3J0IGZ1bmN0aW9uIGNsYXNzaWZ5V2luZG93TGF5b3V0Q2hhbmdlKHByZXZpb3VzOiBXaW5kb3dMYXlvdXROb2RlIHwgbnVsbCwgbmV4dDogV2luZG93TGF5b3V0Tm9kZSB8IG51bGwpOiBcInJlc2l6ZVwiIHwgXCJyZWFycmFuZ2VcIiB8IG51bGwge1xuICBpZiAocHJldmlvdXMgPT09IG5leHQpIHJldHVybiBudWxsO1xuICBpZiAoIXByZXZpb3VzIHx8ICFuZXh0KSByZXR1cm4gXCJyZWFycmFuZ2VcIjtcbiAgaWYgKEpTT04uc3RyaW5naWZ5KHdpbmRvd0xheW91dFNrZWxldG9uKHByZXZpb3VzKSkgIT09IEpTT04uc3RyaW5naWZ5KHdpbmRvd0xheW91dFNrZWxldG9uKG5leHQpKSkgcmV0dXJuIFwicmVhcnJhbmdlXCI7XG4gIGlmIChKU09OLnN0cmluZ2lmeSh3aW5kb3dMYXlvdXRTaXplZFNrZWxldG9uKHByZXZpb3VzKSkgIT09IEpTT04uc3RyaW5naWZ5KHdpbmRvd0xheW91dFNpemVkU2tlbGV0b24obmV4dCkpKSByZXR1cm4gXCJyZXNpemVcIjtcbiAgcmV0dXJuIG51bGw7XG59XG4vLyNlbmRyZWdpb24gV2luZG93TGF5b3V0Q2hhbmdlQ2xhc3NpZnlcblxuZnVuY3Rpb24gd2luZG93RW5nYWdlbWVudENvbnRyb2xUb1NwZWMoY29udHJvbDogV2luZG93RW5nYWdlbWVudENvbnRyb2wgfCB1bmRlZmluZWQsIG9uQWN0aW9uOiAoYWN0aW9uOiBBY3Rpb25EZXNjcmlwdG9yKSA9PiB2b2lkKTogRW5nYWdlbWVudENvbnRyb2wgfCB1bmRlZmluZWQge1xuICBpZiAoIWNvbnRyb2wpIHJldHVybiB1bmRlZmluZWQ7XG4gIGlmIChjb250cm9sLmtpbmQgPT09IFwicmluZ1wiIHx8IGNvbnRyb2wua2luZCA9PT0gXCJ0b2dnbGVHcm91cFwiKSB7XG4gICAgcmV0dXJuIHtcbiAgICAgIGtpbmQ6IGNvbnRyb2wua2luZCxcbiAgICAgIGlkOiBjb250cm9sLmlkLFxuICAgICAgbGFiZWw6IGNvbnRyb2wubGFiZWwsXG4gICAgICB2YWx1ZTogY29udHJvbC52YWx1ZSxcbiAgICAgIGRpc2FibGVkOiBjb250cm9sLmRpc2FibGVkLFxuICAgICAgb3B0aW9uczogY29udHJvbC5vcHRpb25zLm1hcCgocm93KSA9PiAoeyBpZDogcm93LmlkLCBsYWJlbDogcm93LmxhYmVsLCBkaXNhYmxlZDogcm93LmRpc2FibGVkIH0pKSxcbiAgICAgIG9uU2VsZWN0OiBjb250cm9sLm9uU2VsZWN0ID8gKGlkOiBzdHJpbmcpID0+IG9uQWN0aW9uKHsgLi4uY29udHJvbC5vblNlbGVjdCEsIGFyZ3M6IHsgLi4uKGNvbnRyb2wub25TZWxlY3QhLmFyZ3MgYXMgb2JqZWN0IHwgdW5kZWZpbmVkKSwgaWQgfSB9KSA6IHVuZGVmaW5lZCxcbiAgICB9O1xuICB9XG4gIGlmIChjb250cm9sLmtpbmQgPT09IFwic2VsZWN0XCIpIHtcbiAgICByZXR1cm4ge1xuICAgICAga2luZDogXCJzZWxlY3RcIixcbiAgICAgIGlkOiBjb250cm9sLmlkLFxuICAgICAgbGFiZWw6IGNvbnRyb2wubGFiZWwsXG4gICAgICB2YWx1ZTogY29udHJvbC52YWx1ZSxcbiAgICAgIHBsYWNlaG9sZGVyOiBjb250cm9sLnBsYWNlaG9sZGVyLFxuICAgICAgZGlzYWJsZWQ6IGNvbnRyb2wuZGlzYWJsZWQsXG4gICAgICBpdGVtczogY29udHJvbC5pdGVtcy5tYXAoKHJvdykgPT4gKHsgaWQ6IHJvdy5pZCwgdmFsdWU6IHJvdy52YWx1ZSwgbGFiZWw6IHJvdy5sYWJlbCB9KSksXG4gICAgICBvbkNoYW5nZTogY29udHJvbC5vbkNoYW5nZSA/ICh2YWx1ZTogc3RyaW5nKSA9PiBvbkFjdGlvbih7IC4uLmNvbnRyb2wub25DaGFuZ2UhLCBhcmdzOiB7IC4uLihjb250cm9sLm9uQ2hhbmdlIS5hcmdzIGFzIG9iamVjdCB8IHVuZGVmaW5lZCksIHZhbHVlIH0gfSkgOiB1bmRlZmluZWQsXG4gICAgfTtcbiAgfVxuICBjb25zdCBkaXNwYXRjaE51bWVyaWMgPSAoYWN0aW9uOiBBY3Rpb25EZXNjcmlwdG9yIHwgdW5kZWZpbmVkLCB2YWx1ZTogbnVtYmVyKSA9PiB7XG4gICAgaWYgKCFhY3Rpb24pIHJldHVybjtcbiAgICBvbkFjdGlvbih7IC4uLmFjdGlvbiwgYXJnczogeyAuLi4oYWN0aW9uLmFyZ3MgYXMgb2JqZWN0IHwgdW5kZWZpbmVkKSwgdmFsdWUgfSB9KTtcbiAgfTtcbiAgcmV0dXJuIHtcbiAgICBraW5kOiBjb250cm9sLmtpbmQsXG4gICAgaWQ6IGNvbnRyb2wuaWQsXG4gICAgbGFiZWw6IGNvbnRyb2wubGFiZWwsXG4gICAgdmFsdWU6IGNvbnRyb2wudmFsdWUsXG4gICAgbWluOiBjb250cm9sLm1pbixcbiAgICBtYXg6IGNvbnRyb2wubWF4LFxuICAgIHN0ZXA6IGNvbnRyb2wuc3RlcCxcbiAgICB1bml0OiBjb250cm9sLnVuaXQsXG4gICAgZGlzYWJsZWQ6IGNvbnRyb2wuZGlzYWJsZWQsXG4gICAgb25DaGFuZ2U6IGNvbnRyb2wub25DaGFuZ2UgPyAodmFsdWU6IG51bWJlcikgPT4gZGlzcGF0Y2hOdW1lcmljKGNvbnRyb2wub25DaGFuZ2UsIHZhbHVlKSA6IHVuZGVmaW5lZCxcbiAgICBvbkNvbW1pdDogY29udHJvbC5vbkNvbW1pdCA/ICh2YWx1ZTogbnVtYmVyKSA9PiBkaXNwYXRjaE51bWVyaWMoY29udHJvbC5vbkNvbW1pdCwgdmFsdWUpIDogdW5kZWZpbmVkLFxuICB9O1xufVxuXG5jb25zdCBQTFVHSU5fTE9BRF9USU1FT1VUX01TID0gMzBfMDAwO1xuXG4vKiogQGVtb2ppIPCflIzvuI8gUmVzdWx0IG9mIHtAbGluayBpbnN0YWxsUGx1Z2lufSDigJQgdGhlIGJvb3QgZWZmZWN0IG11c3Qgbm90IGluZmVyIHN1Y2Nlc3MgZnJvbVxuICogYGxvYWRlZFBsdWdpbnNSZWZgLCB3aGljaCBvbmx5IHVwZGF0ZXMgYWZ0ZXIgdGhlIG5leHQgUmVhY3QgY29tbWl0LiAqL1xudHlwZSBQbHVnaW5JbnN0YWxsT3V0Y29tZSA9IFwibG9hZGVkXCIgfCBcImFscmVhZHktbG9hZGVkXCIgfCBcImluLWZsaWdodFwiIHwgXCJtaXNzaW5nLXJlZ2lzdHJ5XCIgfCBcImZhaWxlZFwiO1xuXG5leHBvcnQgYXN5bmMgZnVuY3Rpb24gbG9hZFBsdWdpbk1vZHVsZVJlc2lsaWVudChwbHVnaW5JZDogc3RyaW5nLCBtb2R1bGVVcmw6IHN0cmluZyk6IFByb21pc2U8UGx1Z2luV2FzbUhhbmRsZSB8IG51bGw+IHtcbiAgdHJ5IHtcbiAgICByZXR1cm4gYXdhaXQgUHJvbWlzZS5yYWNlKFtcbiAgICAgIGxvYWRQbHVnaW5Nb2R1bGUocGx1Z2luSWQsIG1vZHVsZVVybCksXG4gICAgICBuZXcgUHJvbWlzZTxuZXZlcj4oKF8sIHJlamVjdCkgPT4ge1xuICAgICAgICB3aW5kb3cuc2V0VGltZW91dCgoKSA9PiByZWplY3QobmV3IEVycm9yKGB0aW1lb3V0IGxvYWRpbmcgJHtwbHVnaW5JZH1gKSksIFBMVUdJTl9MT0FEX1RJTUVPVVRfTVMpO1xuICAgICAgfSksXG4gICAgXSk7XG4gIH0gY2F0Y2ggKGVycm9yKSB7XG4gICAgY29uc29sZS5lcnJvcihcIltERUJVR10gcHJvZ3JhbSBsb2FkIGZhaWxlZFwiLCBwbHVnaW5JZCwgZXJyb3IpO1xuICAgIHJldHVybiBudWxsO1xuICB9XG59XG5cbmZ1bmN0aW9uIGlzVmlld3BvcnRTdXJmYWNlKHN1cmZhY2VLaW5kOiBzdHJpbmcgfCB1bmRlZmluZWQpOiBib29sZWFuIHtcbiAgcmV0dXJuIHN1cmZhY2VLaW5kID09PSBcIndvcmxkLTNkXCIgfHwgc3VyZmFjZUtpbmQgPT09IFwibm9kZS1ncmFwaFwiIHx8IHN1cmZhY2VLaW5kID09PSBcImNhbnZhcy0yZFwiO1xufVxuXG5mdW5jdGlvbiBkZWZhdWx0Vmlld3BvcnRFbmdhZ2VtZW50KCk6IFdpbmRvd0VuZ2FnZW1lbnQge1xuICByZXR1cm4ge1xuICAgIHNlc3Npb25BY3RpdmU6IHRydWUsXG4gICAgc3RhdHVzOiBbeyBpZDogXCJmcmFtZXdvcmsudmlld3BvcnQuc3RhdHVzXCIsIHRleHQ6IHNoZWxsTGFiZWwoXCJ1aS5lbmdhZ2VtZW50LnZpZXdwb3J0XCIpIH1dLFxuICB9O1xufVxuXG5leHBvcnQgZnVuY3Rpb24gcmVzb2x2ZVdpbmRvd0VuZ2FnZW1lbnQoa2luZDogQXBwRGVmaW5pdGlvbltcIndpbmRvd0tpbmRzXCJdW251bWJlcl0sIHdpbmRvd0lkOiBzdHJpbmcsIGJ5V2luZG93SWQ6IFJlYWRvbmx5PFJlY29yZDxzdHJpbmcsIFdpbmRvd0VuZ2FnZW1lbnQ+Pik6IFdpbmRvd0VuZ2FnZW1lbnQgfCB1bmRlZmluZWQge1xuICBjb25zdCBzdXJmYWNlS2luZCA9IChraW5kIGFzIHsgc3VyZmFjZUtpbmQ/OiBzdHJpbmcgfSkuc3VyZmFjZUtpbmQ7XG4gIGNvbnN0IGRlY2xhcmVkRW5nYWdlbWVudCA9IGtpbmQub3B0aW9ucy5lbmdhZ2VtZW50LmtpbmQgPT09IFwic29tZVwiID8ga2luZC5vcHRpb25zLmVuZ2FnZW1lbnQudmFsdWUgOiB1bmRlZmluZWQ7XG4gIHJldHVybiBieVdpbmRvd0lkW3dpbmRvd0lkXSA/PyBkZWNsYXJlZEVuZ2FnZW1lbnQgPz8gKGlzVmlld3BvcnRTdXJmYWNlKHN1cmZhY2VLaW5kKSA/IGRlZmF1bHRWaWV3cG9ydEVuZ2FnZW1lbnQoKSA6IHVuZGVmaW5lZCk7XG59XG5cbmV4cG9ydCBmdW5jdGlvbiB3aW5kb3dFbmdhZ2VtZW50VG9TcGVjKGVuZ2FnZW1lbnQ6IFdpbmRvd0VuZ2FnZW1lbnQgfCB1bmRlZmluZWQsIG9uQWN0aW9uOiAoYWN0aW9uOiBBY3Rpb25EZXNjcmlwdG9yKSA9PiB2b2lkKTogRW5nYWdlbWVudFNwZWMgfCB1bmRlZmluZWQge1xuICBpZiAoIWVuZ2FnZW1lbnQpIHJldHVybiB1bmRlZmluZWQ7XG4gIGNvbnN0IG9wdGlvbnMgPSBlbmdhZ2VtZW50Lm9wdGlvbnM/Lm1hcCgob3B0aW9uKSA9PiAoe1xuICAgIGlkOiBvcHRpb24uaWQsXG4gICAgbGFiZWw6IG9wdGlvbi5sYWJlbCxcbiAgICBpY29uOiBvcHRpb24uaWNvbklkID8gPEljb24gaWNvbj17b3B0aW9uLmljb25JZCBhcyBJY29uTmFtZX0gc2l6ZT1cInNtYWxsXCIgLz4gOiB1bmRlZmluZWQsXG4gICAgcHJlc3NlZDogb3B0aW9uLnByZXNzZWQsXG4gICAgZGlzYWJsZWQ6IG9wdGlvbi5kaXNhYmxlZCxcbiAgICBvblByZXNzOiBvcHRpb24uYWN0aW9uID8gKCkgPT4gb25BY3Rpb24ob3B0aW9uLmFjdGlvbiEpIDogdW5kZWZpbmVkLFxuICB9KSk7XG4gIGNvbnN0IHN0YXR1cyA9IGVuZ2FnZW1lbnQuc3RhdHVzPy5tYXAoKHJvdykgPT4gKHsgaWQ6IHJvdy5pZCwgY29udGVudDogcm93LnRleHQgfSkpO1xuICBjb25zdCBjb250cm9sID0gd2luZG93RW5nYWdlbWVudENvbnRyb2xUb1NwZWMoZW5nYWdlbWVudC5jb250cm9sLCBvbkFjdGlvbik7XG4gIGNvbnN0IGNvbnRyb2xzID0gZW5nYWdlbWVudC5jb250cm9scz8ubWFwKChyb3cpID0+IHdpbmRvd0VuZ2FnZW1lbnRDb250cm9sVG9TcGVjKHJvdywgb25BY3Rpb24pKS5maWx0ZXIoKHJvdyk6IHJvdyBpcyBFbmdhZ2VtZW50Q29udHJvbCA9PiByb3cgIT09IHVuZGVmaW5lZCk7XG4gIGNvbnN0IGhhc0NvbnRlbnQgPSAob3B0aW9ucz8ubGVuZ3RoID8/IDApID4gMCB8fCBCb29sZWFuKGNvbnRyb2wpIHx8IChjb250cm9scz8ubGVuZ3RoID8/IDApID4gMCB8fCAoc3RhdHVzPy5sZW5ndGggPz8gMCkgPiAwO1xuICBpZiAoIWhhc0NvbnRlbnQpIHJldHVybiB1bmRlZmluZWQ7XG4gIHJldHVybiB7IHNlc3Npb25BY3RpdmU6IGVuZ2FnZW1lbnQuc2Vzc2lvbkFjdGl2ZSwgb3B0aW9ucywgY29udHJvbCwgY29udHJvbHMsIHN0YXR1cyB9O1xufVxuXG4vKiogQGVtb2ppIPCflI7vuI8gQnVpbGRzIHRoZSB0b3AtbWlkZGxlIHdpbmRvdyB7QGxpbmsgU2VhcmNoU3BlY30gZnJvbSB0aGUgc2FtZSBSdXN0IGVuZ2FnZW1lbnQgcGF5bG9hZDogdHlwZWQgYWN0aW9uIGlucHV0IGFuZCBhdXRvY29tcGxldGUgcG9zc2libGVzLiAqL1xuZXhwb3J0IGZ1bmN0aW9uIHdpbmRvd0VuZ2FnZW1lbnRUb1NlYXJjaFNwZWMoZW5nYWdlbWVudDogV2luZG93RW5nYWdlbWVudCB8IHVuZGVmaW5lZCwgb25BY3Rpb246IChhY3Rpb246IEFjdGlvbkRlc2NyaXB0b3IpID0+IHZvaWQpOiBTZWFyY2hTcGVjIHwgdW5kZWZpbmVkIHtcbiAgaWYgKCFlbmdhZ2VtZW50KSByZXR1cm4gdW5kZWZpbmVkO1xuICBjb25zdCBpbnB1dCA9IGVuZ2FnZW1lbnQuaW5wdXRcbiAgICA/IHtcbiAgICAgICAgaWQ6IGVuZ2FnZW1lbnQuaW5wdXQuaWQsXG4gICAgICAgIHZhbHVlOiBlbmdhZ2VtZW50LmlucHV0LnZhbHVlLFxuICAgICAgICBwbGFjZWhvbGRlcjogZW5nYWdlbWVudC5pbnB1dC5wbGFjZWhvbGRlcixcbiAgICAgICAgZGlzYWJsZWQ6IGVuZ2FnZW1lbnQuaW5wdXQuZGlzYWJsZWQsXG4gICAgICAgIG9uQ2hhbmdlOiBlbmdhZ2VtZW50LmlucHV0Lm9uQ2hhbmdlID8gKHZhbHVlOiBzdHJpbmcpID0+IG9uQWN0aW9uKHsgLi4uZW5nYWdlbWVudC5pbnB1dCEub25DaGFuZ2UhLCBhcmdzOiB7IC4uLihlbmdhZ2VtZW50LmlucHV0IS5vbkNoYW5nZSEuYXJncyBhcyBvYmplY3QgfCB1bmRlZmluZWQpLCB2YWx1ZSB9IH0pIDogdW5kZWZpbmVkLFxuICAgICAgICBvblN1Ym1pdDogZW5nYWdlbWVudC5pbnB1dC5vblN1Ym1pdCA/ICh2YWx1ZTogc3RyaW5nKSA9PiBvbkFjdGlvbih7IC4uLmVuZ2FnZW1lbnQuaW5wdXQhLm9uU3VibWl0ISwgYXJnczogeyAuLi4oZW5nYWdlbWVudC5pbnB1dCEub25TdWJtaXQhLmFyZ3MgYXMgb2JqZWN0IHwgdW5kZWZpbmVkKSwgdmFsdWUgfSB9KSA6IHVuZGVmaW5lZCxcbiAgICAgICAgb25SZXBlYXRMYXN0OiBlbmdhZ2VtZW50LmlucHV0Lm9uUmVwZWF0TGFzdCA/ICgpID0+IG9uQWN0aW9uKGVuZ2FnZW1lbnQuaW5wdXQhLm9uUmVwZWF0TGFzdCEpIDogdW5kZWZpbmVkLFxuICAgICAgICBvbkFib3J0OiBlbmdhZ2VtZW50LmlucHV0Lm9uQWJvcnQgPyAoKSA9PiBvbkFjdGlvbihlbmdhZ2VtZW50LmlucHV0IS5vbkFib3J0ISkgOiB1bmRlZmluZWQsXG4gICAgICB9XG4gICAgOiB1bmRlZmluZWQ7XG4gIGNvbnN0IHBvc3NpYmxlcyA9IGVuZ2FnZW1lbnQucG9zc2libGVFbmdhZ2VtZW50cz8ubWFwKChyb3cpID0+ICh7XG4gICAgaWQ6IHJvdy5pZCxcbiAgICBsYWJlbDogcm93LmxhYmVsLFxuICAgIGRldGFpbDogcm93LmRldGFpbCxcbiAgICBvblNlbGVjdDogcm93LmFjdGlvbiA/ICgpID0+IG9uQWN0aW9uKHJvdy5hY3Rpb24hKSA6IHVuZGVmaW5lZCxcbiAgfSkpO1xuICBjb25zdCBoYXNDb250ZW50ID0gQm9vbGVhbihpbnB1dCkgfHwgKHBvc3NpYmxlcz8ubGVuZ3RoID8/IDApID4gMDtcbiAgaWYgKCFoYXNDb250ZW50KSByZXR1cm4gdW5kZWZpbmVkO1xuICByZXR1cm4geyBzZXNzaW9uQWN0aXZlOiBlbmdhZ2VtZW50LnNlc3Npb25BY3RpdmUsIGlucHV0LCBwb3NzaWJsZXMgfTtcbn1cblxuZnVuY3Rpb24gcGFuZWxUYWJJY29uKHRhYklkOiBzdHJpbmcsIGdyb3VwOiBzdHJpbmcpOiBSZWFjdC5GQzx7IHNpemU/OiBudW1iZXIgfT4ge1xuICAvLyDwn4yx77iPIGBncm91cCA9PT0gXCJ3b3JrYmVuY2hcImAgYWxyZWFkeSBjb3ZlcnMgZXZlcnkgaG9zdC1hcHAgY2F0YWxvZ3VlIHRhYiAoZWFjaCBzdWNoIGFwcCBkZWNsYXJlcyBpdHNcbiAgLy8gY2F0YWxvZ3VlIHRhYiB1bmRlciBgUGFuZWxHcm91cDo6V29ya2JlbmNoYCDigJQgc2VlIGBzL3BsdWdpbi9yc2AncyBgQXBwOjpidWlsZGVyKC4uLikucGFuZWxfdGFiKC4uLilgKVxuICAvLyBzbyBubyBzZXBhcmF0ZSBhcHAtc3BlY2lmaWMgdGFiLWlkIGxpdGVyYWwgaXMgbmVlZGVkIGhlcmUuXG4gIGlmIChncm91cCA9PT0gXCJ3b3JrYmVuY2hcIikgcmV0dXJuIHNoZWxsVGFiSWNvbihGUkFNRVdPUktfUEFORUxfVEFCX0NBVEFMT0dVRV9JQ09OX0lEKTtcbiAgaWYgKHRhYklkLmluY2x1ZGVzKFwicGFyYW1ldGVyc1wiKSkgcmV0dXJuIHNoZWxsVGFiSWNvbihGUkFNRVdPUktfUEFORUxfVEFCX1BBUkFNRVRFUlNfSUNPTl9JRCk7XG4gIGlmICh0YWJJZC5pbmNsdWRlcyhcImluc3BlY3RvclwiKSkgcmV0dXJuIHNoZWxsVGFiSWNvbihGUkFNRVdPUktfUEFORUxfVEFCX0lOU1BFQ1RJT05fSUNPTl9JRCk7XG4gIGlmICh0YWJJZCA9PT0gRlJBTUVXT1JLX1BBTkVMX1RBQl9ISVNUT1JZX0lEKSByZXR1cm4gc2hlbGxUYWJJY29uKFwidW5kb1wiKTtcbiAgcmV0dXJuIHNoZWxsVGFiSWNvbih0YWJJZCk7XG59XG5cbi8qKiBAZW1vamkg8J+Ms++4jyBDYXRlZ29yeS1yb3cgaWNvbjogdGhlIGZpcnN0IGNoaWxkJ3MgaWNvbiwgb3IgYGZhbGxiYWNrYCB3aGVuIHRoZSBjYXRlZ29yeSBoYXMgbm8gdGFicyB5ZXQuICovXG5leHBvcnQgZnVuY3Rpb24gY2F0ZWdvcnlUYWJJY29uKHRhYnM6IHJlYWRvbmx5IFBhbmVsVGFiTm9kZVtdLCBmYWxsYmFjazogSWNvbk5hbWUpOiBSZWFjdC5GQzx7IHNpemU/OiBudW1iZXIgfT4ge1xuICBjb25zdCBGaXJzdEljb24gPSB0YWJzWzBdPy5pY29uO1xuICByZXR1cm4gZnVuY3Rpb24gQ2F0ZWdvcnlUYWJJY29uKHsgc2l6ZSA9IDE2IH06IHsgc2l6ZT86IG51bWJlciB9KSB7XG4gICAgcmV0dXJuIEZpcnN0SWNvbiA/IDxGaXJzdEljb24gc2l6ZT17c2l6ZX0gLz4gOiA8SWNvbiBpY29uPXtmYWxsYmFja30gc2l6ZT1cInNtYWxsXCIgLz47XG4gIH07XG59XG5cbi8qKiBAZW1vamkg8J+Ms++4jyBEZXB0aC1maXJzdCBsZWF2ZXMgb2YgYSByZWN1cnNpdmUgcGFuZWwtdGFiIHRyZWUg4oCUIHRoZSBub2RlcyB0aGF0IGFjdHVhbGx5IGNhcnJ5IGEgYGJvZHlLZXlgIHRvIHJlbmRlci4gKi9cbmV4cG9ydCBmdW5jdGlvbiBmbGF0dGVuUGFuZWxUYWJMZWF2ZXM8VCBleHRlbmRzIHsgcmVhZG9ubHkgY2hpbGRyZW4/OiByZWFkb25seSBUW10gfT4odGFiczogcmVhZG9ubHkgVFtdKTogVFtdIHtcbiAgcmV0dXJuIHRhYnMuZmxhdE1hcCgodGFiKSA9PiAodGFiLmNoaWxkcmVuICYmIHRhYi5jaGlsZHJlbi5sZW5ndGggPiAwID8gZmxhdHRlblBhbmVsVGFiTGVhdmVzKHRhYi5jaGlsZHJlbikgOiBbdGFiXSkpO1xufVxuXG4vKiogQGVtb2ppIPCfjLPvuI8gQ29udmVydHMgb25lIHBsdWdpbi1kZWNsYXJlZCB7QGxpbmsgQXBwUGFuZWxUYWJEZWZpbml0aW9ufSAocmVjdXJzaXZlbHkpIGludG8gYSB7QGxpbmsgUGFuZWxUYWJOb2RlfS4gKi9cbmV4cG9ydCBmdW5jdGlvbiBwYW5lbFRhYkRlZmluaXRpb25Ub05vZGUoXG4gIHRhYjogQXBwUGFuZWxUYWJEZWZpbml0aW9uLFxuICBncm91cDogc3RyaW5nLFxuICBwYW5lbFVpQnlLZXk6IFJlYWRvbmx5PFJlY29yZDxzdHJpbmcsIFVpTm9kZT4+LFxuICBvbkFjdGlvbjogKGFjdGlvbjogQWN0aW9uRGVzY3JpcHRvcikgPT4gdm9pZCxcbiAgb3JkZXI6IG51bWJlcixcbiAgYXBwTGFiZWxzT3ZlcmxheTogUGx1Z2luQXBwTGFiZWxzT3ZlcmxheSxcbiAgdGVybWlub2xvZ3k6IHN0cmluZyA9IFVJX1RFUk1JTk9MT0dZX05BVElWRSxcbiAgbG9jYWxlOiBzdHJpbmcgPSBTSEVMTF9MT0NBTEVTWzBdLFxuKTogUGFuZWxUYWJOb2RlIHtcbiAgY29uc3QgdGFiSWQgPSBwYW5lbFRhYktpbmRJZCh0YWIua2luZCk7XG4gIGNvbnN0IGxhYmVsID0gcmVzb2x2ZVBhbmVsVGFiTGFiZWwoYXBwTGFiZWxzT3ZlcmxheSwgdGFiSWQsIHJlc29sdmVNYW5pZmVzdExhYmVsKHRhYi5sYWJlbCwgdGVybWlub2xvZ3ksIGxvY2FsZSkpO1xuICBpZiAodGFiLmNoaWxkcmVuICYmIHRhYi5jaGlsZHJlbi5sZW5ndGggPiAwKSB7XG4gICAgcmV0dXJuIHtcbiAgICAgIGtpbmQ6IFwiYnJhbmNoXCIsXG4gICAgICBpZDogdGFiSWQsXG4gICAgICBpY29uOiBwYW5lbFRhYkljb24odGFiSWQsIGdyb3VwKSxcbiAgICAgIG5hbWU6IGxhYmVsLFxuICAgICAgb3JkZXIsXG4gICAgICBjaGlsZHJlbjogdGFiLmNoaWxkcmVuLm1hcCgoY2hpbGQsIGNoaWxkT3JkZXIpID0+IHBhbmVsVGFiRGVmaW5pdGlvblRvTm9kZShjaGlsZCwgZ3JvdXAsIHBhbmVsVWlCeUtleSwgb25BY3Rpb24sIGNoaWxkT3JkZXIsIGFwcExhYmVsc092ZXJsYXksIHRlcm1pbm9sb2d5LCBsb2NhbGUpKSxcbiAgICB9O1xuICB9XG4gIHJldHVybiBzaW5nbGVUcmVlTGVhZih7XG4gICAgaWQ6IHRhYklkLFxuICAgIGljb246IHBhbmVsVGFiSWNvbih0YWJJZCwgZ3JvdXApLFxuICAgIG5hbWU6IGxhYmVsLFxuICAgIG9yZGVyLFxuICAgIHRyZWU6IHN0YXRpY1RyZWVQYW5lbERlZmluaXRpb24odWlOb2RlVG9UcmVlUGFuZWxDb25maWcocGFuZWxVaUJ5S2V5W3RhYklkXSA/PyBwZW5kaW5nUGFuZWxVaU5vZGUoKSwgb25BY3Rpb24pKSxcbiAgfSk7XG59XG5cbmV4cG9ydCBmdW5jdGlvbiByZXNvbHZlQ2FudmFzQm9keUtleShhcHA6IEFwcERlZmluaXRpb24pOiBzdHJpbmcge1xuICBjb25zdCB3aW5kb3dLaW5kID0gYXBwLndpbmRvd0tpbmRzWzBdO1xuICBpZiAoIXdpbmRvd0tpbmQpIHJldHVybiBcIm1haW5cIjtcbiAgaWYgKHdpbmRvd0tpbmQuYm9keUtleS5pbmNsdWRlcyhcImNvbXBvc2l0ZVwiKSkge1xuICAgIGNvbnN0IHdvcmtmbG93ID0gYXBwLndpbmRvd0tpbmRzLmZpbmQoKGtpbmQpID0+IGtpbmQuYm9keUtleS5pbmNsdWRlcyhcIndvcmtmbG93XCIpKTtcbiAgICByZXR1cm4gd29ya2Zsb3c/LmJvZHlLZXkgPz8gd2luZG93S2luZC5ib2R5S2V5O1xuICB9XG4gIHJldHVybiB3aW5kb3dLaW5kLmJvZHlLZXk7XG59XG5cbi8vI3JlZ2lvbiDwn6ew77iPVXRpbGl0eVJlZ2lzdHJ5XG4vKipcbiAqIPCfp7DvuI8gUmVzb2x2ZXMgdGhlIGBVdGlsaXR5RGVmaW5pdGlvbmBzIGluIHNjb3BlIGZvciBvbmUgd2luZG93IGtpbmQgYWdhaW5zdCB0aGUgYXBwJ3MgdXRpbGl0eSByZWdpc3RyeTpcbiAqIHRoZSB3aW5kb3cga2luZCdzIG93biBgdXRpbGl0aWVzYCByZWZzIHdoZW4gbm9uLWVtcHR5LCBvdGhlcndpc2UgZXZlcnkgdXRpbGl0eSB0aGUgYXBwIGRlY2xhcmVzICh0aGVcbiAqIHNjb3BpbmcgZmFsbGJhY2ssIG1pcnJvcmluZyBgcmVzb2x2ZVdpbmRvd0FjdGlvbnNgJyBpbnRlbnQgZm9yIHV0aWxpdGllcykuIFVucmVzb2x2YWJsZSByZWZzIGFyZSBkcm9wcGVkLlxuICovXG5leHBvcnQgZnVuY3Rpb24gcmVzb2x2ZVV0aWxpdGllcyhhcHA6IFBpY2s8QXBwRGVmaW5pdGlvbiwgXCJ1dGlsaXRpZXNcIj4sIHdpbmRvd0tpbmQ6IFBpY2s8QXBwV2luZG93S2luZERlZmluaXRpb24sIFwidXRpbGl0aWVzXCI+KTogVXRpbGl0eURlZmluaXRpb25bXSB7XG4gIGNvbnN0IHJlZ2lzdHJ5ID0gYXBwLnV0aWxpdGllcyA/PyBbXTtcbiAgY29uc3QgcmVmcyA9IHdpbmRvd0tpbmQudXRpbGl0aWVzID8/IFtdO1xuICBpZiAocmVmcy5sZW5ndGggPT09IDApIHJldHVybiBbLi4ucmVnaXN0cnldO1xuICBjb25zdCByZXNvbHZlZDogVXRpbGl0eURlZmluaXRpb25bXSA9IFtdO1xuICBmb3IgKGNvbnN0IHJlZiBvZiByZWZzKSB7XG4gICAgY29uc3QgdXRpbGl0eSA9IHJlZ2lzdHJ5LmZpbmQoKGVudHJ5KSA9PiBlbnRyeS5pZCA9PT0gcmVmKTtcbiAgICBpZiAodXRpbGl0eSkgcmVzb2x2ZWQucHVzaCh1dGlsaXR5KTtcbiAgfVxuICByZXR1cm4gcmVzb2x2ZWQ7XG59XG5cbi8qKiDwn6ew77iPIENocm9tZS1rbm93biByaWJib24tZ3JvdXAgaWRzIHRoYXQgYWxyZWFkeSBoYXZlIGEgYHVpLnJpYmJvbi5wYXJlbnQuKmAgdHJhbnNsYXRpb24ga2V5IOKAlCB0aGUgZmFsbGJhY2sgdGllciBmb3IgcGx1Z2luLWRlY2xhcmVkIHV0aWxpdHkgZ3JvdXBzIG5vdCBjb3ZlcmVkIGJ5IHRoYXQgcGx1Z2luJ3Mgb3duIGBncm91cExhYmVsc2Agb3ZlcmxheS4gKi9cbmNvbnN0IENIUk9NRV9LTk9XTl9SSUJCT05fUEFSRU5UX0NBVEVHT1JJRVMgPSBuZXcgU2V0KFVJX1JJQkJPTl9QQVJFTlRfQ0FURUdPUklFUyk7XG5cbi8qKiDwn6ew77iPIFJlc29sdmVzIGEgYFV0aWxpdHlEZWZpbml0aW9uLmdyb3VwYCBpZCdzIGRpc3BsYXkgbGFiZWw6IHRoZSBhcHAncyBvd24gYGdyb3VwTGFiZWxzYCBvdmVybGF5IGZpcnN0LCB0aGVuIHRoZSBzaGFyZWQgYHVpLnJpYmJvbi5wYXJlbnQuKmAgY2hyb21lIHZvY2FidWxhcnkgZm9yIGtub3duIGNhdGVnb3J5IGlkcywgZWxzZSB0aGUgcmF3IGlkLiAqL1xuZnVuY3Rpb24gcmVzb2x2ZVV0aWxpdHlHcm91cExhYmVsKGdyb3VwOiBzdHJpbmcsIGFwcExhYmVsc092ZXJsYXk6IFBsdWdpbkFwcExhYmVsc092ZXJsYXkpOiBzdHJpbmcge1xuICBjb25zdCBmYWxsYmFjayA9IENIUk9NRV9LTk9XTl9SSUJCT05fUEFSRU5UX0NBVEVHT1JJRVMuaGFzKGdyb3VwKSA/IHNoZWxsTGFiZWwoYHVpLnJpYmJvbi5wYXJlbnQuJHtncm91cCBhcyBVaVJpYmJvblBhcmVudENhdGVnb3J5fWApIDogZ3JvdXA7XG4gIHJldHVybiByZXNvbHZlQXBwTGFiZWwoYXBwTGFiZWxzT3ZlcmxheSwgXCJncm91cFwiLCBncm91cCwgZmFsbGJhY2spO1xufVxuXG4vKiog8J+nsO+4jyBPbmUgYFV0aWxpdHlEZWZpbml0aW9uYCDihpIgdGhlIGxlYW4gYERlcml2ZWRVdGlsaXR5U3BlY2AgY29uc3VtZWQgYnkge0BsaW5rIGRlcml2ZVV0aWxpdHlOb2Rlc30sIHJlc29sdmluZyB0aGUgbGFiZWwgKGFuZCwgZm9yIGdyb3VwZWQgdXRpbGl0aWVzLCB0aGUgZ3JvdXAgbGFiZWwpIHRocm91Z2ggdGhlIGFwcCdzIGxvY2FsZS90ZXJtaW5vbG9neSBvdmVybGF5LiBgVXRpbGl0eURlZmluaXRpb24ubGFiZWxgIGlzIGEgbWFuaWZlc3QgYExvY2FsaXplZExhYmVsYCBmaWVsZC4gKi9cbmZ1bmN0aW9uIHV0aWxpdHlEZWZpbml0aW9uVG9TcGVjKHV0aWxpdHk6IFV0aWxpdHlEZWZpbml0aW9uLCBhcHBMYWJlbHNPdmVybGF5OiBQbHVnaW5BcHBMYWJlbHNPdmVybGF5LCB0ZXJtaW5vbG9neTogc3RyaW5nLCBsb2NhbGU6IHN0cmluZyk6IERlcml2ZWRVdGlsaXR5U3BlYyB7XG4gIHJldHVybiB7XG4gICAgaWQ6IHV0aWxpdHkuaWQsXG4gICAgbGFiZWw6IHJlc29sdmVBcHBMYWJlbChhcHBMYWJlbHNPdmVybGF5LCBcInV0aWxpdHlcIiwgdXRpbGl0eS5pZCwgcmVzb2x2ZU1hbmlmZXN0TGFiZWwodXRpbGl0eS5sYWJlbCwgdGVybWlub2xvZ3ksIGxvY2FsZSkpLFxuICAgIGljb25JZDogdXRpbGl0eS5pY29uSWQsXG4gICAgZ3JvdXA6IHV0aWxpdHkuZ3JvdXAgPz8gdW5kZWZpbmVkLFxuICAgIGdyb3VwTGFiZWw6IHV0aWxpdHkuZ3JvdXAgPyByZXNvbHZlVXRpbGl0eUdyb3VwTGFiZWwodXRpbGl0eS5ncm91cCwgYXBwTGFiZWxzT3ZlcmxheSkgOiB1bmRlZmluZWQsXG4gICAgY2F0ZWdvcnk6IHV0aWxpdHkuY2F0ZWdvcnkgPz8gXCJ1dGlsaXRpZXNcIixcbiAgfTtcbn1cblxuLyoqIPCfp7DvuI8gU3RhbXBzIHRoZSBvd25pbmcgYHdpbmRvd0lkYCBvbnRvIGV2ZXJ5IGBzZXRBY3RpdmVVdGlsaXR5YCBkZXNjcmlwdG9yIGluIGEgZGVyaXZlZCB1dGlsaXR5IHRyZWUgc28gdGhlIHNoZWxsJ3MgYG9uQWN0aW9uYCBpbnRlcmNlcHRvciB0YXJnZXRzIHRoZSByaWdodCB3aW5kb3cgcmVnYXJkbGVzcyBvZiB3aGljaCB3aW5kb3cgaXMgZ2xvYmFsbHkgYWN0aXZlLiAqL1xuZnVuY3Rpb24gdGFnU2V0QWN0aXZlVXRpbGl0eVdpbmRvdyhub2RlczogcmVhZG9ubHkgVXRpbGl0eU5vZGVbXSwgd2luZG93SWQ6IHN0cmluZyk6IFV0aWxpdHlOb2RlW10ge1xuICByZXR1cm4gbm9kZXMubWFwKChub2RlKSA9PiB7XG4gICAgaWYgKG5vZGUua2luZCA9PT0gXCJjb2xsZWN0aW9uXCIpIHJldHVybiB7IC4uLm5vZGUsIGNoaWxkcmVuOiB0YWdTZXRBY3RpdmVVdGlsaXR5V2luZG93KG5vZGUuY2hpbGRyZW4sIHdpbmRvd0lkKSB9O1xuICAgIGlmIChub2RlLmtpbmQgPT09IFwidG9nZ2xlXCIgJiYgXCJvbkNoYW5nZVwiIGluIG5vZGUgJiYgbm9kZS5vbkNoYW5nZS5hY3Rpb24gPT09IFNFVF9BQ1RJVkVfVVRJTElUWV9BQ1RJT05fSUQpIHtcbiAgICAgIHJldHVybiB7IC4uLm5vZGUsIG9uQ2hhbmdlOiB7IC4uLm5vZGUub25DaGFuZ2UsIGFyZ3M6IHsgLi4uKG5vZGUub25DaGFuZ2UuYXJncyBhcyBvYmplY3QgfCB1bmRlZmluZWQpLCB3aW5kb3dJZCB9IH0gfTtcbiAgICB9XG4gICAgcmV0dXJuIG5vZGU7XG4gIH0pO1xufVxuXG4vKipcbiAqIPCfp7DvuI8gQnVpbGRzIHRoZSB3aW5kb3cgdXRpbGl0eSBiYXIgYFV0aWxpdHlOb2RlW11gIGZvciBvbmUgd2luZG93IHB1cmVseSBmcm9tIHRoZSBzdGF0aWMgdXRpbGl0eSByZWdpc3RyeSBwbHVzXG4gKiB0aGUgaG9zdC1vd25lZCBhY3RpdmUgdXRpbGl0eSBpZCDigJQgdGhlIHJlcGxhY2VtZW50IGZvciB0aGUgZGVsZXRlZCBwcm9ncmFtIGBsaXN0LXRvb2xzYCBzb3VyY2luZy4gRWFjaFxuICogYHNldEFjdGl2ZVV0aWxpdHlgIGRlc2NyaXB0b3IgaXMgdGFnZ2VkIHdpdGggYHdpbmRvd0lkYCBzbyBhY3RpdmF0aW9uIGlzIHNjb3BlZCB0byB0aGlzIGV4YWN0IHdpbmRvdy5cbiAqL1xuZXhwb3J0IGZ1bmN0aW9uIHJlc29sdmVVdGlsaXR5Tm9kZXMoXG4gIGFwcDogUGljazxBcHBEZWZpbml0aW9uLCBcInV0aWxpdGllc1wiIHwgXCJjb250cm9sbGVySWRcIj4sXG4gIHdpbmRvd0tpbmQ6IFBpY2s8QXBwV2luZG93S2luZERlZmluaXRpb24sIFwidXRpbGl0aWVzXCI+LFxuICBhY3RpdmVVdGlsaXR5SWQ6IHN0cmluZyB8IG51bGwgfCB1bmRlZmluZWQsXG4gIHdpbmRvd0lkOiBzdHJpbmcsXG4gIGFwcExhYmVsc092ZXJsYXk6IFBsdWdpbkFwcExhYmVsc092ZXJsYXkgPSBFTVBUWV9BUFBfTEFCRUxTX09WRVJMQVksXG4gIHRlcm1pbm9sb2d5OiBzdHJpbmcgPSBVSV9URVJNSU5PTE9HWV9OQVRJVkUsXG4gIGxvY2FsZTogc3RyaW5nID0gU0hFTExfTE9DQUxFU1swXSxcbik6IFV0aWxpdHlOb2RlW10ge1xuICBjb25zdCB1dGlsaXRpZXMgPSByZXNvbHZlVXRpbGl0aWVzKGFwcCwgd2luZG93S2luZCk7XG4gIGlmICh1dGlsaXRpZXMubGVuZ3RoID09PSAwKSByZXR1cm4gW107XG4gIHJldHVybiB0YWdTZXRBY3RpdmVVdGlsaXR5V2luZG93KFxuICAgIGRlcml2ZVV0aWxpdHlOb2RlcyhcbiAgICAgIGFwcC5jb250cm9sbGVySWQsXG4gICAgICB1dGlsaXRpZXMubWFwKCh1dGlsaXR5KSA9PiB1dGlsaXR5RGVmaW5pdGlvblRvU3BlYyh1dGlsaXR5LCBhcHBMYWJlbHNPdmVybGF5LCB0ZXJtaW5vbG9neSwgbG9jYWxlKSksXG4gICAgICBhY3RpdmVVdGlsaXR5SWQgPz8gdW5kZWZpbmVkLFxuICAgICksXG4gICAgd2luZG93SWQsXG4gICk7XG59XG4vLyNlbmRyZWdpb24g8J+nsO+4j1V0aWxpdHlSZWdpc3RyeVxuXG4vKiogQGVtb2ppIPCfkqzvuI8gQnVpbGRzIHNwYXduZWQtd2luZG93IGVuZ2FnZW1lbnQsIHNlYXJjaCwgbWVhc3VyZXMsIGFuZCB1dGlsaXR5LW9wdGlvbnMgY2hyb21lIGZvciBvbmUgd2luZG93IGluc3RhbmNlLiAqL1xuZXhwb3J0IGZ1bmN0aW9uIHNwYXduZWRXaW5kb3dDaHJvbWVGb3JLaW5kKFxuICBraW5kOiBBcHBEZWZpbml0aW9uW1wid2luZG93S2luZHNcIl1bbnVtYmVyXSxcbiAgd2luZG93SWQ6IHN0cmluZyxcbiAgZW5nYWdlbWVudHNCeVdpbmRvd0lkOiBSZWFkb25seTxSZWNvcmQ8c3RyaW5nLCBXaW5kb3dFbmdhZ2VtZW50Pj4sXG4gIG1lYXN1cmVzQnlXaW5kb3dJZDogUmVhZG9ubHk8UmVjb3JkPHN0cmluZywgcmVhZG9ubHkgV2luZG93TWVhc3VyZVtdPj4sXG4gIGFjdGl2ZVV0aWxpdHlJZDogc3RyaW5nIHwgdW5kZWZpbmVkLFxuICBvbkFjdGlvbjogKGFjdGlvbjogQWN0aW9uRGVzY3JpcHRvcikgPT4gdm9pZCxcbik6IHsgcmVhZG9ubHkgZW5nYWdlbWVudD86IEVuZ2FnZW1lbnRTcGVjOyByZWFkb25seSBzZWFyY2g/OiBTZWFyY2hTcGVjOyByZWFkb25seSBtZWFzdXJlczogUmVhY3ROb2RlOyByZWFkb25seSB1dGlsaXR5T3B0aW9uczogUmVhY3ROb2RlIH0ge1xuICBjb25zdCB7IG1lYXN1cmVzLCB1dGlsaXR5T3B0aW9ucyB9ID0gd2luZG93TWVhc3VyZXNDaHJvbWUobWVhc3VyZXNCeVdpbmRvd0lkW3dpbmRvd0lkXSA/PyBraW5kLm9wdGlvbnMubWVhc3VyZXMsIGFjdGl2ZVV0aWxpdHlJZCwgd2luZG93SWQsIG9uQWN0aW9uKTtcbiAgY29uc3QgcmVzb2x2ZWRFbmdhZ2VtZW50ID0gcmVzb2x2ZVdpbmRvd0VuZ2FnZW1lbnQoa2luZCwgd2luZG93SWQsIGVuZ2FnZW1lbnRzQnlXaW5kb3dJZCk7XG4gIHJldHVybiB7XG4gICAgZW5nYWdlbWVudDogd2luZG93RW5nYWdlbWVudFRvU3BlYyhyZXNvbHZlZEVuZ2FnZW1lbnQsIG9uQWN0aW9uKSxcbiAgICBzZWFyY2g6IHdpbmRvd0VuZ2FnZW1lbnRUb1NlYXJjaFNwZWMocmVzb2x2ZWRFbmdhZ2VtZW50LCBvbkFjdGlvbiksXG4gICAgbWVhc3VyZXMsXG4gICAgdXRpbGl0eU9wdGlvbnMsXG4gIH07XG59XG5cbmZ1bmN0aW9uIGlzVHJlZU5vZGUobm9kZTogVWlOb2RlKTogbm9kZSBpcyBVaVRyZWVOb2RlIHtcbiAgcmV0dXJuIG5vZGUudHlwZSA9PT0gXCJ0cmVlXCI7XG59XG5cbmV4cG9ydCBmdW5jdGlvbiB1aU5vZGVUb1RyZWVQYW5lbENvbmZpZyhub2RlOiBVaU5vZGUsIG9uQWN0aW9uOiAoYWN0aW9uOiBBY3Rpb25EZXNjcmlwdG9yKSA9PiB2b2lkKTogVHJlZVBhbmVsQ29uZmlnIHtcbiAgY29uc3QgdHJlZUhhc0RyYWcgPSBub2RlLnR5cGUgPT09IFwidHJlZVwiICYmIG5vZGUuc2VjdGlvbnMuc29tZSgocykgPT4gcy5pdGVtcy5zb21lKChpKSA9PiBpLmRyYWdnYWJsZSB8fCBpLmRyYWdEYXRhKSk7XG4gIGlmIChpc1RyZWVOb2RlKG5vZGUpKSB7XG4gICAgcmV0dXJuIHtcbiAgICAgIC4uLnVpVHJlZU5vZGVUb1RyZWVQYW5lbENvbmZpZyhub2RlLCBvbkFjdGlvbiksXG4gICAgICBkcmFnQW5kRHJvcENvbnRyb2xsZXI6IG5vZGUuZHJvcEFjdGlvbiB8fCB0cmVlSGFzRHJhZyA/IGRlY2xhcmF0aXZlVHJlZURyYWdDb250cm9sbGVyKG5vZGUsIG9uQWN0aW9uKSA6IHVuZGVmaW5lZCxcbiAgICB9O1xuICB9XG4gIHJldHVybiBkZWNsYXJhdGl2ZVVpTm9kZVRvVHJlZVBhbmVsQ29uZmlnKG5vZGUsIG9uQWN0aW9uKTtcbn1cblxuLyoqIEBlbW9qaSDwn4yy77iPIE1hcHMgbm9uLXRyZWUgZGVjbGFyYXRpdmUgVUkgKHN0YWNrL3NlY3Rpb24vZmllbGQvY29udHJvbHMpIHRvIHRoZSBzYW1lIFRyZWVQYW5lbCBzaGFwZSBTZXR0aW5ncy9UaGVtZSB1c2Ug4oCUIG5ldmVyIGFuIGVtcHR5LWxhYmVsIHdyYXBwZXIgaG9zdCAodGhhdCByZW5kZXJlZCBhcyBhIGxvbmUgZG9jdW1lbnQgaWNvbiBhYm92ZSBuZXN0ZWQgY29udGVudCkuICovXG5mdW5jdGlvbiBkZWNsYXJhdGl2ZVVpTm9kZVRvVHJlZVBhbmVsQ29uZmlnKG5vZGU6IFVpTm9kZSwgb25BY3Rpb246IChhY3Rpb246IEFjdGlvbkRlc2NyaXB0b3IpID0+IHZvaWQpOiBUcmVlUGFuZWxDb25maWcge1xuICBpZiAobm9kZS50eXBlID09PSBcInN0YWNrXCIpIHtcbiAgICBjb25zdCBlbXBoYXNpemVkID0gbm9kZS5jaGlsZHJlbi5maW5kKChjaGlsZCkgPT4gY2hpbGQudHlwZSA9PT0gXCJ0ZXh0XCIgJiYgY2hpbGQuZW1waGFzaXplKTtcbiAgICBjb25zdCBib2R5Q2hpbGRyZW4gPSBub2RlLmNoaWxkcmVuLmZpbHRlcigoY2hpbGQpID0+ICEoY2hpbGQudHlwZSA9PT0gXCJ0ZXh0XCIgJiYgY2hpbGQuZW1waGFzaXplKSk7XG4gICAgY29uc3Qgc2VjdGlvbk5vZGVzID0gYm9keUNoaWxkcmVuLmZpbHRlcigoY2hpbGQpID0+IGNoaWxkLnR5cGUgPT09IFwic2VjdGlvblwiKTtcbiAgICBpZiAoc2VjdGlvbk5vZGVzLmxlbmd0aCA+IDAgJiYgc2VjdGlvbk5vZGVzLmxlbmd0aCA9PT0gYm9keUNoaWxkcmVuLmxlbmd0aCkge1xuICAgICAgcmV0dXJuIHtcbiAgICAgICAgc2VjdGlvbnM6IHNlY3Rpb25Ob2Rlcy5tYXAoKHNlY3Rpb24pID0+ICh7XG4gICAgICAgICAgaWQ6IHNlY3Rpb24uaWQsXG4gICAgICAgICAgbGFiZWw6IHNlY3Rpb24ubGFiZWwgPz8gXCJcIixcbiAgICAgICAgICBkZWZhdWx0T3Blbjogc2VjdGlvbi5kZWZhdWx0T3BlbixcbiAgICAgICAgICBpdGVtczogc2VjdGlvbi5jaGlsZHJlbi5mbGF0TWFwKChjaGlsZCwgaW5kZXgpID0+IGRlY2xhcmF0aXZlVWlDaGlsZFRvVHJlZUl0ZW1zKGNoaWxkLCBgJHtzZWN0aW9uLmlkfS4ke2luZGV4fWAsIG9uQWN0aW9uKSksXG4gICAgICAgIH0pKSxcbiAgICAgICAgc29ydGFibGVTZWN0aW9uczogZmFsc2UsXG4gICAgICB9O1xuICAgIH1cbiAgICByZXR1cm4ge1xuICAgICAgc2VjdGlvbnM6IFtcbiAgICAgICAge1xuICAgICAgICAgIGlkOiBub2RlLmlkID8/IFwicGFuZWwuYm9keVwiLFxuICAgICAgICAgIGxhYmVsOiBlbXBoYXNpemVkICYmIGVtcGhhc2l6ZWQudHlwZSA9PT0gXCJ0ZXh0XCIgPyBlbXBoYXNpemVkLnZhbHVlIDogXCJcIixcbiAgICAgICAgICBkZWZhdWx0T3BlbjogdHJ1ZSxcbiAgICAgICAgICBpdGVtczogYm9keUNoaWxkcmVuLmZsYXRNYXAoKGNoaWxkLCBpbmRleCkgPT4gZGVjbGFyYXRpdmVVaUNoaWxkVG9UcmVlSXRlbXMoY2hpbGQsIGAke25vZGUuaWQgPz8gXCJwYW5lbC5ib2R5XCJ9LiR7aW5kZXh9YCwgb25BY3Rpb24pKSxcbiAgICAgICAgfSxcbiAgICAgIF0sXG4gICAgICBzb3J0YWJsZVNlY3Rpb25zOiBmYWxzZSxcbiAgICB9O1xuICB9XG4gIGlmIChub2RlLnR5cGUgPT09IFwic2VjdGlvblwiKSB7XG4gICAgcmV0dXJuIHtcbiAgICAgIHNlY3Rpb25zOiBbXG4gICAgICAgIHtcbiAgICAgICAgICBpZDogbm9kZS5pZCxcbiAgICAgICAgICBsYWJlbDogbm9kZS5sYWJlbCA/PyBcIlwiLFxuICAgICAgICAgIGRlZmF1bHRPcGVuOiBub2RlLmRlZmF1bHRPcGVuLFxuICAgICAgICAgIGl0ZW1zOiBub2RlLmNoaWxkcmVuLmZsYXRNYXAoKGNoaWxkLCBpbmRleCkgPT4gZGVjbGFyYXRpdmVVaUNoaWxkVG9UcmVlSXRlbXMoY2hpbGQsIGAke25vZGUuaWR9LiR7aW5kZXh9YCwgb25BY3Rpb24pKSxcbiAgICAgICAgfSxcbiAgICAgIF0sXG4gICAgICBzb3J0YWJsZVNlY3Rpb25zOiBmYWxzZSxcbiAgICB9O1xuICB9XG4gIHJldHVybiB7XG4gICAgc2VjdGlvbnM6IFtcbiAgICAgIHtcbiAgICAgICAgaWQ6IFwicGFuZWwuYm9keVwiLFxuICAgICAgICBsYWJlbDogXCJcIixcbiAgICAgICAgZGVmYXVsdE9wZW46IHRydWUsXG4gICAgICAgIGl0ZW1zOiBkZWNsYXJhdGl2ZVVpQ2hpbGRUb1RyZWVJdGVtcyhub2RlLCBcInBhbmVsLmJvZHkuMFwiLCBvbkFjdGlvbiksXG4gICAgICB9LFxuICAgIF0sXG4gICAgc29ydGFibGVTZWN0aW9uczogZmFsc2UsXG4gIH07XG59XG5cbmZ1bmN0aW9uIGlzVWlDb250cm9sTm9kZShub2RlOiBVaU5vZGUpOiBub2RlIGlzIFVpQ29udHJvbE5vZGUge1xuICBzd2l0Y2ggKG5vZGUudHlwZSkge1xuICAgIGNhc2UgXCJidXR0b25cIjpcbiAgICBjYXNlIFwiaW5wdXRcIjpcbiAgICBjYXNlIFwic2VsZWN0XCI6XG4gICAgY2FzZSBcInRvZ2dsZVwiOlxuICAgIGNhc2UgXCJzbGlkZXJcIjpcbiAgICBjYXNlIFwibnVtYmVyU3RlcHBlclwiOlxuICAgIGNhc2UgXCJyaW5nXCI6XG4gICAgY2FzZSBcImljb25TZWxlY3RcIjpcbiAgICBjYXNlIFwia2V5VmFsdWVcIjpcbiAgICAgIHJldHVybiB0cnVlO1xuICAgIGRlZmF1bHQ6XG4gICAgICByZXR1cm4gZmFsc2U7XG4gIH1cbn1cblxuZnVuY3Rpb24gZGVjbGFyYXRpdmVVaUNoaWxkVG9UcmVlSXRlbXMobm9kZTogVWlOb2RlLCBmYWxsYmFja0lkOiBzdHJpbmcsIG9uQWN0aW9uOiAoYWN0aW9uOiBBY3Rpb25EZXNjcmlwdG9yKSA9PiB2b2lkKTogVHJlZURhdGFJdGVtW10ge1xuICBzd2l0Y2ggKG5vZGUudHlwZSkge1xuICAgIGNhc2UgXCJmaWVsZFwiOiB7XG4gICAgICBjb25zdCBjb250cm9sID0gaXNVaUNvbnRyb2xOb2RlKG5vZGUuY2hpbGQpID8gcmVuZGVyVWlDb250cm9sKG5vZGUuY2hpbGQsIG9uQWN0aW9uKSA6IDxJbnRlcnByZXRlZFVpTm9kZSBub2RlPXtub2RlLmNoaWxkfSBvbkFjdGlvbj17b25BY3Rpb259IC8+O1xuICAgICAgcmV0dXJuIFt7IGlkOiBub2RlLmlkLCBsYWJlbDogbm9kZS5sYWJlbCwgZGVzY3JpcHRpb246IG5vZGUuZGVzY3JpcHRpb24sIGNvbnRyb2wgfV07XG4gICAgfVxuICAgIGNhc2UgXCJ0ZXh0XCI6XG4gICAgICByZXR1cm4gW3sgaWQ6IGAke2ZhbGxiYWNrSWR9LnRleHRgLCBsYWJlbDogbm9kZS52YWx1ZSB9XTtcbiAgICBjYXNlIFwiYnV0dG9uXCI6XG4gICAgICByZXR1cm4gW3sgaWQ6IG5vZGUuaWQgPz8gZmFsbGJhY2tJZCwgbGFiZWw6IG5vZGUubGFiZWwsIGNvbnRyb2w6IHJlbmRlclVpQ29udHJvbChub2RlLCBvbkFjdGlvbikgfV07XG4gICAgY2FzZSBcImlucHV0XCI6XG4gICAgY2FzZSBcInNlbGVjdFwiOlxuICAgIGNhc2UgXCJ0b2dnbGVcIjpcbiAgICBjYXNlIFwic2xpZGVyXCI6XG4gICAgY2FzZSBcIm51bWJlclN0ZXBwZXJcIjpcbiAgICBjYXNlIFwicmluZ1wiOlxuICAgIGNhc2UgXCJpY29uU2VsZWN0XCI6XG4gICAgY2FzZSBcImtleVZhbHVlXCI6XG4gICAgICByZXR1cm4gW3sgaWQ6IG5vZGUuaWQsIGxhYmVsOiBub2RlLnBsYWNlaG9sZGVyID8/IG5vZGUuaWQsIGNvbnRyb2w6IHJlbmRlclVpQ29udHJvbChub2RlLCBvbkFjdGlvbikgfV07XG4gICAgY2FzZSBcInN0YWNrXCI6XG4gICAgICByZXR1cm4gbm9kZS5jaGlsZHJlbi5mbGF0TWFwKChjaGlsZCwgaW5kZXgpID0+IGRlY2xhcmF0aXZlVWlDaGlsZFRvVHJlZUl0ZW1zKGNoaWxkLCBgJHtmYWxsYmFja0lkfS4ke2luZGV4fWAsIG9uQWN0aW9uKSk7XG4gICAgY2FzZSBcImdyb3VwXCI6XG4gICAgICByZXR1cm4gW1xuICAgICAgICB7XG4gICAgICAgICAgaWQ6IG5vZGUuaWQsXG4gICAgICAgICAgbGFiZWw6IG5vZGUubGFiZWwsXG4gICAgICAgICAgZGVmYXVsdE9wZW46IG5vZGUuZGVmYXVsdE9wZW4sXG4gICAgICAgICAgaXRlbXM6IG5vZGUuY2hpbGRyZW4uZmxhdE1hcCgoY2hpbGQsIGluZGV4KSA9PiBkZWNsYXJhdGl2ZVVpQ2hpbGRUb1RyZWVJdGVtcyhjaGlsZCwgYCR7bm9kZS5pZH0uJHtpbmRleH1gLCBvbkFjdGlvbikpLFxuICAgICAgICB9LFxuICAgICAgXTtcbiAgICBjYXNlIFwidHJlZVwiOlxuICAgICAgcmV0dXJuIHVpVHJlZU5vZGVUb1RyZWVQYW5lbENvbmZpZyhub2RlLCBvbkFjdGlvbikuc2VjdGlvbnMuZmxhdE1hcCgoc2VjdGlvbikgPT4gc2VjdGlvbi5pdGVtcyk7XG4gICAgY2FzZSBcInNlcGFyYXRvclwiOlxuICAgICAgcmV0dXJuIFt7IGlkOiBgJHtmYWxsYmFja0lkfS5zZXBgLCBsYWJlbDogXCLigJRcIiB9XTtcbiAgICBkZWZhdWx0OlxuICAgICAgcmV0dXJuIFtcbiAgICAgICAge1xuICAgICAgICAgIGlkOiBmYWxsYmFja0lkLFxuICAgICAgICAgIGxhYmVsOiBub2RlLnR5cGUsXG4gICAgICAgICAgY29udHJvbDogKFxuICAgICAgICAgICAgPFNoZWxsRmF1bHRCb3VuZGFyeSBib3VuZGFyeUlkPXtgcGFuZWwtJHtmYWxsYmFja0lkfWB9IGZhbGxiYWNrTGFiZWw9e3NoZWxsTGFiZWwoXCJ1aS5jb21tb24ucmVuZGVyRXJyb3JcIil9PlxuICAgICAgICAgICAgICA8Q2hyb21lQXdhcmVXaW5kb3dTY3JvbGxTdXJmYWNlIGNsYXNzTmFtZT1cIm1pbi1oLTAgZmxleC0xXCI+e2ludGVycHJldFVpTm9kZShub2RlLCB7IG9uQWN0aW9uIH0pfTwvQ2hyb21lQXdhcmVXaW5kb3dTY3JvbGxTdXJmYWNlPlxuICAgICAgICAgICAgPC9TaGVsbEZhdWx0Qm91bmRhcnk+XG4gICAgICAgICAgKSxcbiAgICAgICAgfSxcbiAgICAgIF07XG4gIH1cbn1cblxuZXhwb3J0IGZ1bmN0aW9uIHNoZWxsVGFiSWNvbihpY29uSWQ6IEljb25OYW1lIHwgc3RyaW5nKTogUmVhY3QuRkM8eyBzaXplPzogbnVtYmVyIH0+IHtcbiAgcmV0dXJuIGZ1bmN0aW9uIFNoZWxsVGFiSWNvbih7IHNpemUgPSAxNiB9OiB7IHNpemU/OiBudW1iZXIgfSkge1xuICAgIGNvbnN0IGljb25OYW1lOiBJY29uTmFtZSA9XG4gICAgICBpY29uSWQgPT09IEZSQU1FV09SS19QQU5FTF9UQUJfRE9DVU1FTlRfSUNPTl9JRFxuICAgICAgICA/IFwiZmlsZS10ZXh0XCJcbiAgICAgICAgOiBpY29uSWQgPT09IEZSQU1FV09SS19QQU5FTF9UQUJfQ0FUQUxPR1VFX0lDT05fSURcbiAgICAgICAgICA/IFwicGFuZWwtY2F0YWxvZ3VlXCJcbiAgICAgICAgICA6IGljb25JZCA9PT0gRlJBTUVXT1JLX1BBTkVMX1RBQl9JTlNQRUNUSU9OX0lDT05fSURcbiAgICAgICAgICAgID8gXCJwYW5lbC1pbnNwZWN0aW9uXCJcbiAgICAgICAgICAgIDogaWNvbklkID09PSBGUkFNRVdPUktfUEFORUxfVEFCX1BBUkFNRVRFUlNfSUNPTl9JRFxuICAgICAgICAgICAgICA/IFwicGFuZWwtcGFyYW1ldGVyc1wiXG4gICAgICAgICAgICAgIDogaXNJY29uTmFtZShpY29uSWQpXG4gICAgICAgICAgICAgICAgPyBpY29uSWRcbiAgICAgICAgICAgICAgICA6IFwiY2lyY2xlLWRvdFwiO1xuICAgIHJldHVybiA8SWNvbiBpY29uPXtpY29uTmFtZX0gc2l6ZT17c2l6ZX0gLz47XG4gIH07XG59XG5cbi8qKiBAZW1vamkg8J+MkO+4jyBSZXNvbHZlcyBhIGNocm9tZSB0cmFuc2xhdGlvbiBrZXkgb3V0c2lkZSBob29rIGNvbnRleHQgKHRyZWUgYnVpbGRlcnMgcnVuIHRoZXJlKS4gQm90aCBidW5kbGVzXG4gKiBhcmUgZ3VhcmFudGVlZCBjb21wbGV0ZSBmb3IgZXZlcnkga2V5IHZpYSBgc2F0aXNmaWVzIFVpVHJhbnNsYXRpb25TY2hlbWFgLCBzbyBgPz8ga2V5YCBpcyB1bnJlYWNoYWJsZSBpblxuICogcHJhY3RpY2Ug4oCUIGtlcHQgb25seSBhcyBhIGxhc3QtcmVzb3J0IGxpdGVyYWwgcmF0aGVyIHRoYW4gYSB0aHJvd24gZXJyb3IuIGBvcHRpb25zYCBzdXBwb3J0cyBpMThuZXh0XG4gKiBpbnRlcnBvbGF0aW9uIGZvciBrZXlzIHdpdGggYHt7cGxhY2Vob2xkZXJzfX1gLiAqL1xuZXhwb3J0IGZ1bmN0aW9uIHNoZWxsTGFiZWwoa2V5OiBVaVRyYW5zbGF0aW9uS2V5LCBvcHRpb25zPzogUmVjb3JkPHN0cmluZywgdW5rbm93bj4pOiBVaUxhYmVsIHtcbiAgcmV0dXJuIHdpcmVMYWJlbChyZXNvbHZlVHJhbnNsYXRpb25MYWJlbCh1aUkxOG4udChrZXksIG9wdGlvbnMpKSA/PyBrZXkpO1xufVxuXG4vKiogQGVtb2ppIPCfp63vuI8gVGhlIGZpdmUgcGFuZWwgdGFicyB0aGUgZnJhbWV3b3JrIGl0c2VsZiBvd25zIChuZXZlciBhcHAtc3VwcGxpZWQpIOKAlCByb3V0ZWQgdGhyb3VnaCB0aGUgdHlwZWQgY2hyb21lIHNjaGVtYSBpbnN0ZWFkIG9mIHRoZSBwbHVnaW4gb3ZlcmxheSBzbyBhIGxvY2FsZS1sb2NrZWQgc2hlbGwgY2FuIG5ldmVyIHNob3cgdGhlaXIgRW5nbGlzaCBtYW5pZmVzdCBsYWJlbC4gKi9cbmNvbnN0IEZSQU1FV09SS19QQU5FTF9UQUJfTEFCRUxfS0VZUzogUmVhZG9ubHk8UmVjb3JkPHN0cmluZywgVWlUcmFuc2xhdGlvbktleT4+ID0ge1xuICBbRlJBTUVXT1JLX1BBTkVMX1RBQl9ET0NVTUVOVF9JRF06IFwidWkucGFuZWwuZG9jdW1lbnRcIixcbiAgW0ZSQU1FV09SS19QQU5FTF9UQUJfQ0FUQUxPR1VFX0lEXTogXCJ1aS5wYW5lbC5jYXRhbG9ndWVcIixcbiAgW0ZSQU1FV09SS19QQU5FTF9UQUJfSU5TUEVDVElPTl9JRF06IFwidWkucGFuZWwuaW5zcGVjdGlvblwiLFxuICBbRlJBTUVXT1JLX1BBTkVMX1RBQl9QQVJBTUVURVJTX0lEXTogXCJ1aS5wYW5lbC5wYXJhbWV0ZXJzXCIsXG4gIFtGUkFNRVdPUktfUEFORUxfVEFCX0hJU1RPUllfSURdOiBcInVpLnBhbmVsLmhpc3RvcnlcIixcbn07XG5cbi8qKiBAZW1vamkg8J+nre+4jyBGcmFtZXdvcmstb3duZWQgcGFuZWwgdGFicyByZXNvbHZlIHRocm91Z2ggdGhlIGNocm9tZSBzY2hlbWEgKGBzaGVsbExhYmVsYCk7IGV2ZXJ5IG90aGVyIGFwcC1kZWNsYXJlZCB0YWIgc3RpbGwgcmVzb2x2ZXMgdGhyb3VnaCB0aGUgcGx1Z2luIG92ZXJsYXkgKGByZXNvbHZlQXBwTGFiZWxgKS4gKi9cbmV4cG9ydCBmdW5jdGlvbiByZXNvbHZlUGFuZWxUYWJMYWJlbChvdmVybGF5OiBQbHVnaW5BcHBMYWJlbHNPdmVybGF5LCB0YWJJZDogc3RyaW5nLCBmYWxsYmFjazogc3RyaW5nKTogc3RyaW5nIHtcbiAgY29uc3QgY2hyb21lS2V5ID0gRlJBTUVXT1JLX1BBTkVMX1RBQl9MQUJFTF9LRVlTW3RhYklkXTtcbiAgcmV0dXJuIGNocm9tZUtleSA/IHNoZWxsTGFiZWwoY2hyb21lS2V5KSA6IHJlc29sdmVBcHBMYWJlbChvdmVybGF5LCBcInBhbmVsVGFiXCIsIHRhYklkLCBmYWxsYmFjayk7XG59XG5cbi8qKiBAZW1vamkg8J+Xo++4jyBTdGFibGUgZW1wdHkgb3ZlcmxheSByZWZlcmVuY2Ugc28gY29tcG9uZW50cyBkZXBlbmRpbmcgb24gaXQgZG9uJ3QgcmUtcmVuZGVyIGJlZm9yZSB0aGUgZmlyc3QgYGFwcExhYmVsc2AgZmV0Y2ggcmVzb2x2ZXMuICovXG5leHBvcnQgY29uc3QgRU1QVFlfQVBQX0xBQkVMU19PVkVSTEFZOiBQbHVnaW5BcHBMYWJlbHNPdmVybGF5ID0ge1xuICB3aW5kb3dLaW5kTGFiZWxzOiB7fSxcbiAgcGFuZWxUYWJMYWJlbHM6IHt9LFxuICBtb2RlTGFiZWxzOiB7fSxcbiAgYWN0aW9uTGFiZWxzOiB7fSxcbiAgdXRpbGl0eUxhYmVsczoge30sXG4gIGV4YW1wbGVMYWJlbHM6IHt9LFxuICBhY3Rpb25BcmdMYWJlbHM6IHt9LFxuICBkaWFsb2dMYWJlbHM6IHt9LFxuICBpbnRyb2R1Y3Rpb25MYWJlbHM6IHt9LFxuICBncm91cExhYmVsczoge30sXG59O1xuXG4vKiog8J+Xuu+4jyBTeW50aGVzaXplcyBhIGZ1bGwgYExvY2FsaXplZExhYmVsYCBtYXRyaXggZnJvbSBhIHVzZXItYXV0aG9yZWQgc3RyaW5nIGJ5IGJyb2FkY2FzdGluZyBpdCBhY3Jvc3MgYWxsIGNlbGxzIChuYXRpdmUvcmV1c2Ugw5cgZW4vZGUpLCBtYXRjaGluZyBSdXN0J3MgYExvY2FsaXplZExhYmVsOjpkYXRhKC4uLilgLiBBbHNvIGFjY2VwdHMgYW4gZXhpc3RpbmcgYExvY2FsaXplZExhYmVsYCBpZGVtcG90ZW50bHkuICovXG5leHBvcnQgZnVuY3Rpb24gc3ludGhlc2l6ZUxvY2FsaXplZExhYmVsKGxhYmVsOiBzdHJpbmcgfCBMb2NhbGl6ZWRMYWJlbCk6IExvY2FsaXplZExhYmVsIHtcbiAgaWYgKHR5cGVvZiBsYWJlbCAhPT0gXCJzdHJpbmdcIikgcmV0dXJuIGxhYmVsO1xuICByZXR1cm4ge1xuICAgIG5hdGl2ZTogeyBlbjogbGFiZWwsIGRlOiBsYWJlbCB9LFxuICAgIHJldXNlOiB7IGVuOiBsYWJlbCwgZGU6IGxhYmVsIH0sXG4gIH07XG59XG5cbi8qKiDwn5e677iPIFJlc29sdmVzIGEgbWFuaWZlc3QgbGFiZWwgZmllbGQgZm9yIHRoZSBhY3RpdmUgdGVybWlub2xvZ3kvbG9jYWxlLiBFdmVyeSBhcHAtbWFuaWZlc3Qgc3RydWN0J3NcbiAqIGBsYWJlbGAvYHRpdGxlYC9gYm9keWAvYHN1Ym1pdExhYmVsYC9gY2FuY2VsTGFiZWxgL2BkZXNjcmlwdGlvbmAvYHRleHRgIGZpZWxkIGlzIG5vdyBSdXN0J3NcbiAqIGBMb2NhbGl6ZWRMYWJlbGAgb24gdGhlIHdpcmUg4oCUIGEgYHsgbmF0aXZlOiB7IGVuLCBkZSB9LCByZXVzZTogeyBlbiwgZGUgfSB9YCBtYXRyaXgg4oCUIGluc3RlYWQgb2YgdGhlXG4gKiBwbGFpbiBzdHJpbmcgdGhlc2UgZmllbGRzIHVzZWQgdG8gYmUuIEZhbGxzIGJhY2sgZ3JhY2VmdWxseSAocmV1c2XihpJuYXRpdmUsIG1pc3NpbmcgbG9jYWxl4oaSZW4sIG1pc3NpbmdcbiAqIGVudGlyZWx54oaSXCJcIikgc28gYSBzdGFsZS9wYXJ0aWFsIHBheWxvYWQgbmV2ZXIgdGhyb3dzOyBhbHNvIHRvbGVyYXRlcyBhIGJhcmUgYHN0cmluZ2AgZGVmZW5zaXZlbHkgc2luY2VcbiAqIHRoZSB0cy1ycyBtaXJyb3IgZm9yIHRoZXNlIGZpZWxkcyBpcyBzdGlsbCBgdW5rbm93bmAvc3RhbGUgKHNlZSBgZnJhbWV3b3JrL2NvcmUvcnMvbGliLnJzYCdzXG4gKiBgTG9jYWxpemVkTGFiZWxgIGZvbGxvdy11cCBub3Rlcykg4oCUIHNvbWUgY2FsbCBzaXRlcyBtYXkgc3RpbGwgc2VlIHRoZSBwcmUtbWlncmF0aW9uIHNoYXBlIHVudGlsIHRoYXRcbiAqIHR5cGVnZW4gbGFuZHMuICovXG5leHBvcnQgZnVuY3Rpb24gcmVzb2x2ZU1hbmlmZXN0TGFiZWwobGFiZWw6IExvY2FsaXplZExhYmVsIHwgc3RyaW5nIHwgdW5kZWZpbmVkLCB0ZXJtaW5vbG9neTogc3RyaW5nLCBsb2NhbGU6IHN0cmluZyk6IHN0cmluZyB7XG4gIGlmIChsYWJlbCA9PT0gdW5kZWZpbmVkKSByZXR1cm4gXCJcIjtcbiAgaWYgKHR5cGVvZiBsYWJlbCA9PT0gXCJzdHJpbmdcIikgcmV0dXJuIGxhYmVsO1xuICBjb25zdCBieVRlcm1pbm9sb2d5ID0gbGFiZWxbdGVybWlub2xvZ3kgYXMga2V5b2YgTG9jYWxpemVkTGFiZWxdID8/IGxhYmVsLm5hdGl2ZSA/PyBsYWJlbC5yZXVzZTtcbiAgaWYgKCFieVRlcm1pbm9sb2d5KSByZXR1cm4gXCJcIjtcbiAgcmV0dXJuIGJ5VGVybWlub2xvZ3lbbG9jYWxlIGFzIGtleW9mIHR5cGVvZiBieVRlcm1pbm9sb2d5XSA/PyBieVRlcm1pbm9sb2d5LmVuID8/IE9iamVjdC52YWx1ZXMoYnlUZXJtaW5vbG9neSlbMF0gPz8gXCJcIjtcbn1cblxuLyoqIEBlbW9qaSDwn5ej77iPIFJlc29sdmVzIGEgd2luZG93LWtpbmQvcGFuZWwtdGFiL21vZGUvYWN0aW9uL3V0aWxpdHkvZXhhbXBsZS9hY3Rpb25BcmcvZGlhbG9nL2ludHJvZHVjdGlvbi9ncm91cCBpZCdzIGxvY2FsZS1hd2FyZSBsYWJlbCBmcm9tIHRoZSBhY3RpdmUgYXBwJ3Mgb3ZlcmxheSwgZmFsbGluZyBiYWNrIHRvIHRoZSBzdGF0aWMgbWFuaWZlc3QgbGFiZWwuICovXG5leHBvcnQgZnVuY3Rpb24gcmVzb2x2ZUFwcExhYmVsKG92ZXJsYXk6IFBsdWdpbkFwcExhYmVsc092ZXJsYXksIGtpbmQ6IFwid2luZG93S2luZFwiIHwgXCJwYW5lbFRhYlwiIHwgXCJtb2RlXCIgfCBcImFjdGlvblwiIHwgXCJ1dGlsaXR5XCIgfCBcImV4YW1wbGVcIiB8IFwiYWN0aW9uQXJnXCIgfCBcImRpYWxvZ1wiIHwgXCJpbnRyb2R1Y3Rpb25cIiB8IFwiZ3JvdXBcIiwgaWQ6IHN0cmluZywgZmFsbGJhY2s6IHN0cmluZyk6IHN0cmluZyB7XG4gIGNvbnN0IG1hcCA9XG4gICAga2luZCA9PT0gXCJ3aW5kb3dLaW5kXCJcbiAgICAgID8gb3ZlcmxheS53aW5kb3dLaW5kTGFiZWxzXG4gICAgICA6IGtpbmQgPT09IFwicGFuZWxUYWJcIlxuICAgICAgICA/IG92ZXJsYXkucGFuZWxUYWJMYWJlbHNcbiAgICAgICAgOiBraW5kID09PSBcIm1vZGVcIlxuICAgICAgICAgID8gb3ZlcmxheS5tb2RlTGFiZWxzXG4gICAgICAgICAgOiBraW5kID09PSBcImFjdGlvblwiXG4gICAgICAgICAgICA/IG92ZXJsYXkuYWN0aW9uTGFiZWxzXG4gICAgICAgICAgICA6IGtpbmQgPT09IFwidXRpbGl0eVwiXG4gICAgICAgICAgICAgID8gb3ZlcmxheS51dGlsaXR5TGFiZWxzXG4gICAgICAgICAgICAgIDoga2luZCA9PT0gXCJleGFtcGxlXCJcbiAgICAgICAgICAgICAgICA/IG92ZXJsYXkuZXhhbXBsZUxhYmVsc1xuICAgICAgICAgICAgICAgIDoga2luZCA9PT0gXCJhY3Rpb25BcmdcIlxuICAgICAgICAgICAgICAgICAgPyBvdmVybGF5LmFjdGlvbkFyZ0xhYmVsc1xuICAgICAgICAgICAgICAgICAgOiBraW5kID09PSBcImRpYWxvZ1wiXG4gICAgICAgICAgICAgICAgICAgID8gb3ZlcmxheS5kaWFsb2dMYWJlbHNcbiAgICAgICAgICAgICAgICAgICAgOiBraW5kID09PSBcImludHJvZHVjdGlvblwiXG4gICAgICAgICAgICAgICAgICAgICAgPyBvdmVybGF5LmludHJvZHVjdGlvbkxhYmVsc1xuICAgICAgICAgICAgICAgICAgICAgIDogb3ZlcmxheS5ncm91cExhYmVscztcbiAgcmV0dXJuIG1hcFtpZF0gPz8gZmFsbGJhY2s7XG59XG5cbi8qKiBAZW1vamkg8J+Xo++4jyBSZXNvbHZlcyBvbmUgYWN0aW9uLWFyZydzIGxhYmVsICsgKGZvciBgc2VsZWN0YCBjb250cm9scykgaXRzIG9wdGlvbnMnIGxhYmVscyBmcm9tIHRoZSBvdmVybGF5J3MgYGFjdGlvbkFyZ0xhYmVsc2AgbWFwLCBrZXllZCBgXCJ7c2NvcGVJZH0ue2FyZ0lkfVwiYCAvIGBcIntzY29wZUlkfS57YXJnSWR9Lm9wdGlvbi57dmFsdWV9XCJgLiBgc2NvcGVJZGAgaXMgYW4gYWN0aW9uIGlkIGZvciBzdGFnZWQvcGFsZXR0ZSBmb3JtcywgYSBkaWFsb2cgaWQgZm9yIGRpYWxvZyBhcmdzLCBvciBhIGNvbW1hbmQgaWQgZm9yIGNvbW1hbmQgYXJncy4gYEFjdGlvbkFyZ0RlZi5sYWJlbGAvYEFjdGlvbkFyZ09wdGlvbi5sYWJlbGAgYXJlIG1hbmlmZXN0IGBMb2NhbGl6ZWRMYWJlbGAgZmllbGRzLCByZXNvbHZlZCBmb3IgYHRlcm1pbm9sb2d5YC9gbG9jYWxlYCBiZWZvcmUgdGhlIG92ZXJsYXkncyAoYWx3YXlzLWVtcHR5LCBzZWUgdGhlIGBBcHBMYWJlbHNPdmVybGF5YCBkZWxldGlvbiBub3RlKSBmYWxsYmFjayBsb29rdXAgZXZlbiBhcHBsaWVzLiAqL1xuZnVuY3Rpb24gcmVzb2x2ZUFjdGlvbkFyZ0RlZihkZWY6IEFjdGlvbkFyZ0RlZiwgc2NvcGVJZDogc3RyaW5nLCBvdmVybGF5OiBQbHVnaW5BcHBMYWJlbHNPdmVybGF5LCB0ZXJtaW5vbG9neTogc3RyaW5nLCBsb2NhbGU6IHN0cmluZyk6IEFjdGlvbkFyZ0RlZiB7XG4gIGNvbnN0IGxhYmVsID0gcmVzb2x2ZUFwcExhYmVsKG92ZXJsYXksIFwiYWN0aW9uQXJnXCIsIGAke3Njb3BlSWR9LiR7ZGVmLmlkfWAsIHJlc29sdmVNYW5pZmVzdExhYmVsKGRlZi5sYWJlbCwgdGVybWlub2xvZ3ksIGxvY2FsZSkpO1xuICBpZiAoZGVmLmNvbnRyb2wua2luZCAhPT0gXCJzZWxlY3RcIikgcmV0dXJuIGxhYmVsID09PSBkZWYubGFiZWwgPyBkZWYgOiB7IC4uLmRlZiwgbGFiZWwgfTtcbiAgY29uc3Qgb3B0aW9ucyA9IGRlZi5jb250cm9sLm9wdGlvbnMubWFwKChvcHRpb24pID0+ICh7IC4uLm9wdGlvbiwgbGFiZWw6IHJlc29sdmVBcHBMYWJlbChvdmVybGF5LCBcImFjdGlvbkFyZ1wiLCBgJHtzY29wZUlkfS4ke2RlZi5pZH0ub3B0aW9uLiR7b3B0aW9uLnZhbHVlfWAsIHJlc29sdmVNYW5pZmVzdExhYmVsKG9wdGlvbi5sYWJlbCwgdGVybWlub2xvZ3ksIGxvY2FsZSkpIH0pKTtcbiAgcmV0dXJuIHsgLi4uZGVmLCBsYWJlbCwgY29udHJvbDogeyAuLi5kZWYuY29udHJvbCwgb3B0aW9ucyB9IH07XG59XG5cbi8qKiBAZW1vamkg8J+Xo++4jyBSZXNvbHZlcyBhIGBEaWFsb2dEZWZpbml0aW9uYCdzIHRpdGxlL2JvZHkvc3VibWl0TGFiZWwvY2FuY2VsTGFiZWwvYXJncyBmcm9tIHRoZSBvdmVybGF5J3MgYGRpYWxvZ0xhYmVsc2AvYGFjdGlvbkFyZ0xhYmVsc2AgbWFwcywga2V5ZWQgYnkgdGhlIGRpYWxvZydzIG93biBpZC4gYHRpdGxlYC9gYm9keWAvYHN1Ym1pdExhYmVsYC9gY2FuY2VsTGFiZWxgIGFyZSBhbGwgbWFuaWZlc3QgYExvY2FsaXplZExhYmVsYCBmaWVsZHMuICovXG5leHBvcnQgZnVuY3Rpb24gcmVzb2x2ZURpYWxvZ0RlZmluaXRpb24oZGlhbG9nOiBEaWFsb2dEZWZpbml0aW9uLCBvdmVybGF5OiBQbHVnaW5BcHBMYWJlbHNPdmVybGF5LCB0ZXJtaW5vbG9neTogc3RyaW5nLCBsb2NhbGU6IHN0cmluZyk6IERpYWxvZ0RlZmluaXRpb24ge1xuICByZXR1cm4ge1xuICAgIC4uLmRpYWxvZyxcbiAgICB0aXRsZTogcmVzb2x2ZUFwcExhYmVsKG92ZXJsYXksIFwiZGlhbG9nXCIsIGAke2RpYWxvZy5pZH0udGl0bGVgLCByZXNvbHZlTWFuaWZlc3RMYWJlbChkaWFsb2cudGl0bGUsIHRlcm1pbm9sb2d5LCBsb2NhbGUpKSxcbiAgICBib2R5OiBkaWFsb2cuYm9keSA/IHJlc29sdmVBcHBMYWJlbChvdmVybGF5LCBcImRpYWxvZ1wiLCBgJHtkaWFsb2cuaWR9LmJvZHlgLCByZXNvbHZlTWFuaWZlc3RMYWJlbChkaWFsb2cuYm9keSwgdGVybWlub2xvZ3ksIGxvY2FsZSkpIDogZGlhbG9nLmJvZHksXG4gICAgc3VibWl0TGFiZWw6IHJlc29sdmVBcHBMYWJlbChvdmVybGF5LCBcImRpYWxvZ1wiLCBgJHtkaWFsb2cuaWR9LnN1Ym1pdGAsIHJlc29sdmVNYW5pZmVzdExhYmVsKGRpYWxvZy5zdWJtaXRMYWJlbCwgdGVybWlub2xvZ3ksIGxvY2FsZSkpLFxuICAgIGNhbmNlbExhYmVsOiBkaWFsb2cuY2FuY2VsTGFiZWwgPyByZXNvbHZlQXBwTGFiZWwob3ZlcmxheSwgXCJkaWFsb2dcIiwgYCR7ZGlhbG9nLmlkfS5jYW5jZWxgLCByZXNvbHZlTWFuaWZlc3RMYWJlbChkaWFsb2cuY2FuY2VsTGFiZWwsIHRlcm1pbm9sb2d5LCBsb2NhbGUpKSA6IGRpYWxvZy5jYW5jZWxMYWJlbCxcbiAgICBhcmdzOiBkaWFsb2cuYXJncy5tYXAoKGRlZikgPT4gcmVzb2x2ZUFjdGlvbkFyZ0RlZihkZWYsIGRpYWxvZy5pZCwgb3ZlcmxheSwgdGVybWlub2xvZ3ksIGxvY2FsZSkpLFxuICB9O1xufVxuXG4vKiogQGVtb2ppIPCfl6PvuI8gUmVzb2x2ZXMgYW4gYEludHJvZHVjdGlvbkRlZmluaXRpb25gJ3MgdGl0bGUgYW5kIGV2ZXJ5IHN0ZXAncyB0aXRsZS9ib2R5IGxhYmVscyBmcm9tIHRoZVxuICogb3ZlcmxheSdzIGBpbnRyb2R1Y3Rpb25MYWJlbHNgIG1hcC4gYHRpdGxlYC9gYm9keWAgYXJlIG1hbmlmZXN0IGBMb2NhbGl6ZWRMYWJlbGAgZmllbGRzO1xuICogYEludHJvZHVjdGlvbkludGVyYWN0aW9uLmxhYmVsYCBpcyBhIHNob3J0IGNoZWNrbGlzdCBjYXB0aW9uIHRoYXQgaXMgc3RpbGwgYSBwbGFpbiBgU3RyaW5nYCBvbiB0aGUgUnVzdFxuICogc2lkZSAobm90IHBhcnQgb2YgdGhlIGBMb2NhbGl6ZWRMYWJlbGAgbWlncmF0aW9uKSwgc28gaXQgaXMgbGVmdCBhcy1pcy4gKi9cbmV4cG9ydCBmdW5jdGlvbiByZXNvbHZlSW50cm9kdWN0aW9uRGVmaW5pdGlvbihpbnRyb2R1Y3Rpb246IEludHJvZHVjdGlvbkRlZmluaXRpb24sIG92ZXJsYXk6IFBsdWdpbkFwcExhYmVsc092ZXJsYXksIHRlcm1pbm9sb2d5OiBzdHJpbmcsIGxvY2FsZTogc3RyaW5nKTogSW50cm9kdWN0aW9uRGVmaW5pdGlvbiB7XG4gIHJldHVybiB7XG4gICAgdGl0bGU6IHJlc29sdmVBcHBMYWJlbChvdmVybGF5LCBcImludHJvZHVjdGlvblwiLCBcImludHJvLnRpdGxlXCIsIHJlc29sdmVNYW5pZmVzdExhYmVsKGludHJvZHVjdGlvbi50aXRsZSwgdGVybWlub2xvZ3ksIGxvY2FsZSkpLFxuICAgIHN0ZXBzOiBpbnRyb2R1Y3Rpb24uc3RlcHMubWFwKFxuICAgICAgKHN0ZXApOiBJbnRyb2R1Y3Rpb25TdGVwRGVmaW5pdGlvbiA9PiAoe1xuICAgICAgICAuLi5zdGVwLFxuICAgICAgICB0aXRsZTogcmVzb2x2ZUFwcExhYmVsKG92ZXJsYXksIFwiaW50cm9kdWN0aW9uXCIsIGBpbnRyby5zdGVwLiR7c3RlcC5pZH0udGl0bGVgLCByZXNvbHZlTWFuaWZlc3RMYWJlbChzdGVwLnRpdGxlLCB0ZXJtaW5vbG9neSwgbG9jYWxlKSksXG4gICAgICAgIGJvZHk6IHJlc29sdmVBcHBMYWJlbChvdmVybGF5LCBcImludHJvZHVjdGlvblwiLCBgaW50cm8uc3RlcC4ke3N0ZXAuaWR9LmJvZHlgLCByZXNvbHZlTWFuaWZlc3RMYWJlbChzdGVwLmJvZHksIHRlcm1pbm9sb2d5LCBsb2NhbGUpKSxcbiAgICAgICAgaW50ZXJhY3Rpb25zOiAoc3RlcC5pbnRlcmFjdGlvbnMgPz8gW10pLm1hcCgoaW50ZXJhY3Rpb24sIGluZGV4KSA9PiAoe1xuICAgICAgICAgIC4uLmludGVyYWN0aW9uLFxuICAgICAgICAgIGxhYmVsOiByZXNvbHZlQXBwTGFiZWwob3ZlcmxheSwgXCJpbnRyb2R1Y3Rpb25cIiwgYGludHJvLnN0ZXAuJHtzdGVwLmlkfS5pbnRlcmFjdGlvbi4ke2luZGV4fS5sYWJlbGAsIGludGVyYWN0aW9uLmxhYmVsKSxcbiAgICAgICAgfSkpLFxuICAgICAgICBvcmRlcmVkOiBzdGVwLm9yZGVyZWQgPz8gZmFsc2UsXG4gICAgICB9KSxcbiAgICApLFxuICB9O1xufVxuXG4vLyNyZWdpb24g8J+Ope+4j1R1dG9yaWFsVWlCcmlkZ2Vcbi8qKiBAZW1vamkg8J+Ope+4jyBDYXB0dXJlcyB0aGUgc2hlbGwncyBjdXJyZW50IGBTaGVsbFN0YXRlYCAoKyBhY3RpdmUgc2Vzc2lvbikgYXMgYSByZW5kZXJlci1uZXV0cmFsIGBUdXRvcmlhbFVpU25hcHNob3RgIOKAlCB0aGUgcmVjb3JkZXIncyBwZXJpb2RpYyBmdWxsLXNuYXBzaG90IGtleWZyYW1lcyBhbmQgdGhlIGBUdXRvcmlhbEJhcmAncyBcInJlY29yZFwiIHBhdGggYm90aCBjYWxsIHRoaXMuIFNlZSB0aGUgUnVzdCBkb2MgY29tbWVudCBvbiBgVHV0b3JpYWxVaVNuYXBzaG90YCBmb3Igd2h5IHRoaXMgaXMgZGVsaWJlcmF0ZWx5IE5PVCBhIHNlcmlhbGl6YXRpb24gb2YgYFNoZWxsU3RhdGVgIGl0c2VsZi4gKi9cbmV4cG9ydCBmdW5jdGlvbiBjYXB0dXJlVHV0b3JpYWxVaVNuYXBzaG90KHN0YXRlOiBTaGVsbFN0YXRlLCBzZXNzaW9uOiBBY3RpdmVTZXNzaW9uIHwgbnVsbCk6IFR1dG9yaWFsVWlTbmFwc2hvdCB7XG4gIGNvbnN0IGFjdGl2ZVV0aWxpdHlCeVdpbmRvd0lkOiBSZWNvcmQ8c3RyaW5nLCBzdHJpbmc+ID0ge307XG4gIGZvciAoY29uc3QgW3dpbmRvd0lkLCB1dGlsaXR5SWRdIG9mIE9iamVjdC5lbnRyaWVzKHN0YXRlLmFjdGlvblBhbmUuYWN0aXZlVXRpbGl0eUJ5V2luZG93SWQpKSB7XG4gICAgaWYgKHV0aWxpdHlJZCkgYWN0aXZlVXRpbGl0eUJ5V2luZG93SWRbd2luZG93SWRdID0gdXRpbGl0eUlkO1xuICB9XG4gIGNvbnN0IGFjdGl2ZVBhbmVsVGFiQnlHcm91cDogUmVjb3JkPHN0cmluZywgc3RyaW5nPiA9IHt9O1xuICBmb3IgKGNvbnN0IGFuY2hvciBvZiBBTkNIT1JTKSB7XG4gICAgY29uc3QgcGFuZWxTdGF0ZSA9IHN0YXRlLmxheW91dC5wYW5lbHNbYW5jaG9yXTtcbiAgICBjb25zdCB0YWJJZCA9IHBhbmVsU3RhdGUucGF0aFtwYW5lbFN0YXRlLnBhdGgubGVuZ3RoIC0gMV07XG4gICAgaWYgKHBhbmVsU3RhdGUudmlzaWJsZSAmJiB0YWJJZCkgYWN0aXZlUGFuZWxUYWJCeUdyb3VwW2FuY2hvcl0gPSB0YWJJZDtcbiAgfVxuICByZXR1cm4ge1xuICAgIGFjdGl2ZU1vZGVJZDogc2Vzc2lvbj8udmlld1N0YXRlLmFjdGl2ZU1vZGVJZCxcbiAgICBmb2N1c2VkV2luZG93SWQ6IHN0YXRlLmxheW91dC5hY3RpdmVXaW5kb3dJZCA/PyB1bmRlZmluZWQsXG4gICAgYWN0aXZlVXRpbGl0eUJ5V2luZG93SWQsXG4gICAgYWN0aXZlVG9vbElkOiBzdGF0ZS5hY3Rpb25QYW5lLmFjdGl2ZVRvb2xJZCA/PyB1bmRlZmluZWQsXG4gICAgbGF5b3V0OiBjYXB0dXJlQ3VycmVudEZyYW1ld29ya0xheW91dChzdGF0ZS5sYXlvdXQuc2hlbGxMYXlvdXQsIHN0YXRlLmxheW91dC5leHRyYVdpbmRvd0luc3RhbmNlcyksXG4gICAgYWN0aXZlUGFuZWxUYWJCeUdyb3VwLFxuICAgIHBhbmVsSnNvbjogc2Vzc2lvbj8udmlld1N0YXRlLnBhbmVsSnNvbixcbiAgICBzZWxlY3Rpb25Kc29uOiBzZXNzaW9uPy52aWV3U3RhdGUuc2VsZWN0aW9uSnNvbixcbiAgICBvcGVuRGlhbG9nSWQ6IHN0YXRlLm92ZXJsYXlzLmRpYWxvZz8uZGlhbG9nSWQsXG4gICAgZXhwYW5kZWRUcmVlSWRzOiBPYmplY3QuZW50cmllcyhzdGF0ZS5sYXlvdXQudHJlZU9wZW5TdGF0ZXMpLmZpbHRlcigoWywgb3Blbl0pID0+IG9wZW4pLm1hcCgoW2lkXSkgPT4gaWQpLFxuICAgIGNvbW1hbmRQYW5lbE9wZW46IHN0YXRlLm92ZXJsYXlzLnNlYXJjaE9wZW4sXG4gIH07XG59XG5cbi8qKiBAZW1vamkg8J+Ope+4jyBDb250ZXh0IGV2ZXJ5IGBhcHBseVR1dG9yaWFsVWlTbmFwc2hvdFRvU2hlbGxgL2BhcHBseVR1dG9yaWFsVWlDaGFuZ2VUb1NoZWxsYCBjYWxsIG5lZWRzIGJleW9uZCBgZGlzcGF0Y2hgIGl0c2VsZiDigJQgcmVzb2x2ZWQgb25jZSBwZXIgcmVuZGVyIGJ5IHRoZSBjYWxsZXIgKHRoZSBkaXJlY3Rvci9zZWVrL2RldmlhdGlvbi1jb252ZXJnZSBwYXRocyBhbGwgc2hhcmUgaXQpLiAqL1xudHlwZSBUdXRvcmlhbFVpQnJpZGdlQ29udGV4dCA9IHtcbiAgcmVhZG9ubHkgc2Vzc2lvbjogQWN0aXZlU2Vzc2lvbiB8IG51bGw7XG4gIHJlYWRvbmx5IGFwcExhYmVsc092ZXJsYXk6IFBsdWdpbkFwcExhYmVsc092ZXJsYXk7XG4gIHJlYWRvbmx5IHRlcm1pbm9sb2d5OiBzdHJpbmc7XG4gIHJlYWRvbmx5IGxvY2FsZTogc3RyaW5nO1xufTtcblxuLyoqIEBlbW9qaSDwn46l77iPIEFwcGxpZXMgYSBmdWxsIGBUdXRvcmlhbFVpU25hcHNob3RgIChhIGBUdXRvcmlhbFVpU2FtcGxlOjpTbmFwc2hvdGAsIG9yIHRoZSBjb21wb3NlZCB0YXJnZXQgb2YgYSBzZWVrL2RldmlhdGlvbi1jb252ZXJnZSkgb250byB0aGUgbGl2ZSBgU2hlbGxTdGF0ZWAg4oCUIHNuYXBzIGV2ZXJ5IGZpZWxkIGluc3RhbnRseSAoY2FtZXJhIGlzIHRoZSBvbmx5IGludGVycG9sYXRlZCB0cmFjaywgYXBwbGllZCBzZXBhcmF0ZWx5IGJ5IHRoZSBkaXJlY3RvcikuIERpc3BhdGNoZXMgdGhlIGF0b21pYyBgQVBQTFlfVFVUT1JJQUxfVUlfU05BUFNIT1RgIGZvciBldmVyeXRoaW5nIHJlc29sdmFibGUgcHVyZWx5IGZyb20gYFNoZWxsU3RhdGVgLCBwbHVzIG9uZSBgU0VUX1NFU1NJT05gIGZvciB0aGUgZmllbGRzIHRoYXQgbGl2ZSBvbiBgQWN0aXZlU2Vzc2lvbi52aWV3U3RhdGVgIChgYWN0aXZlTW9kZUlkYC9gcGFuZWxKc29uYC9gc2VsZWN0aW9uSnNvbmApLiAqL1xuZXhwb3J0IGZ1bmN0aW9uIGFwcGx5VHV0b3JpYWxVaVNuYXBzaG90VG9TaGVsbChkaXNwYXRjaDogKGFjdGlvbjogU2hlbGxBY3Rpb24pID0+IHZvaWQsIHNuYXBzaG90OiBUdXRvcmlhbFVpU25hcHNob3QsIGN0eDogVHV0b3JpYWxVaUJyaWRnZUNvbnRleHQpOiB2b2lkIHtcbiAgY29uc3Qgd2luZG93S2luZHMgPSBjdHguc2Vzc2lvbj8uYXBwLndpbmRvd0tpbmRzLm1hcCgoa2luZCkgPT4gKHsgaWQ6IGtpbmQuaWQsIGxhYmVsOiBraW5kLmxhYmVsIH0pKSA/PyBbXTtcbiAgY29uc3Qgc2VlZCA9IGFwcGx5RnJhbWV3b3JrTGF5b3V0U2VlZChzbmFwc2hvdC5sYXlvdXQsIHdpbmRvd0tpbmRzLCBjdHguYXBwTGFiZWxzT3ZlcmxheSwgY3R4LnRlcm1pbm9sb2d5LCBjdHgubG9jYWxlKTtcbiAgY29uc3QgcGFuZWxQYXRjaGVzOiBQYXJ0aWFsPFJlY29yZDxBbmNob3IsIHsgcmVhZG9ubHkgdmlzaWJsZTogYm9vbGVhbjsgcmVhZG9ubHkgcGF0aDogcmVhZG9ubHkgc3RyaW5nW10gfT4+ID0ge307XG4gIGZvciAoY29uc3QgYW5jaG9yIG9mIEFOQ0hPUlMpIHtcbiAgICBjb25zdCB0YWJJZCA9IHNuYXBzaG90LmFjdGl2ZVBhbmVsVGFiQnlHcm91cFthbmNob3JdO1xuICAgIHBhbmVsUGF0Y2hlc1thbmNob3JdID0gdGFiSWQgPyB7IHZpc2libGU6IHRydWUsIHBhdGg6IFt0YWJJZF0gfSA6IHsgdmlzaWJsZTogZmFsc2UsIHBhdGg6IFtdIH07XG4gIH1cbiAgY29uc3QgdHJlZU9wZW5TdGF0ZXM6IFJlY29yZDxzdHJpbmcsIGJvb2xlYW4+ID0ge307XG4gIGZvciAoY29uc3QgaWQgb2Ygc25hcHNob3QuZXhwYW5kZWRUcmVlSWRzKSB0cmVlT3BlblN0YXRlc1tpZF0gPSB0cnVlO1xuICBkaXNwYXRjaCh7XG4gICAgdHlwZTogXCJBUFBMWV9UVVRPUklBTF9VSV9TTkFQU0hPVFwiLFxuICAgIHNuYXBzaG90OiB7XG4gICAgICBhY3RpdmVXaW5kb3dJZDogc25hcHNob3QuZm9jdXNlZFdpbmRvd0lkID8/IG51bGwsXG4gICAgICBzaGVsbExheW91dDogc2VlZC5tb2RlTGF5b3V0LFxuICAgICAgZXh0cmFXaW5kb3dJbnN0YW5jZXM6IHNlZWQuZXh0cmFJbnN0YW5jZXMsXG4gICAgICBwYW5lbFBhdGNoZXMsXG4gICAgICB0cmVlT3BlblN0YXRlcyxcbiAgICAgIGFjdGl2ZVV0aWxpdHlCeVdpbmRvd0lkOiBzbmFwc2hvdC5hY3RpdmVVdGlsaXR5QnlXaW5kb3dJZCxcbiAgICAgIGFjdGl2ZVRvb2xJZDogc25hcHNob3QuYWN0aXZlVG9vbElkID8/IG51bGwsXG4gICAgICBvcGVuRGlhbG9nSWQ6IHNuYXBzaG90Lm9wZW5EaWFsb2dJZCA/PyBudWxsLFxuICAgICAgY29tbWFuZFBhbmVsT3Blbjogc25hcHNob3QuY29tbWFuZFBhbmVsT3BlbixcbiAgICB9LFxuICB9KTtcbiAgaWYgKGN0eC5zZXNzaW9uKSB7XG4gICAgZGlzcGF0Y2goe1xuICAgICAgdHlwZTogXCJTRVRfU0VTU0lPTlwiLFxuICAgICAgdmFsdWU6IChjdXJyZW50KSA9PlxuICAgICAgICBjdXJyZW50XG4gICAgICAgICAgPyB7XG4gICAgICAgICAgICAgIC4uLmN1cnJlbnQsXG4gICAgICAgICAgICAgIHZpZXdTdGF0ZToge1xuICAgICAgICAgICAgICAgIC4uLmN1cnJlbnQudmlld1N0YXRlLFxuICAgICAgICAgICAgICAgIGFjdGl2ZU1vZGVJZDogc25hcHNob3QuYWN0aXZlTW9kZUlkID8/IGN1cnJlbnQudmlld1N0YXRlLmFjdGl2ZU1vZGVJZCxcbiAgICAgICAgICAgICAgICBwYW5lbEpzb246IHNuYXBzaG90LnBhbmVsSnNvbiA/PyBjdXJyZW50LnZpZXdTdGF0ZS5wYW5lbEpzb24sXG4gICAgICAgICAgICAgICAgc2VsZWN0aW9uSnNvbjogc25hcHNob3Quc2VsZWN0aW9uSnNvbiA/PyBjdXJyZW50LnZpZXdTdGF0ZS5zZWxlY3Rpb25Kc29uLFxuICAgICAgICAgICAgICB9LFxuICAgICAgICAgICAgfVxuICAgICAgICAgIDogY3VycmVudCxcbiAgICB9KTtcbiAgfVxufVxuXG4vKiogQGVtb2ppIPCfjqXvuI8gQXBwbGllcyBvbmUgc3BhcnNlIGBUdXRvcmlhbFVpQ2hhbmdlYCAoYSBgVHV0b3JpYWxVaVNhbXBsZTo6RGVsdGFgIGVudHJ5LCByZXBsYXllZCBieSB0aGUgZGlyZWN0b3IncyBwZXItdGljayBgdHV0b3JpYWxTbGljZWApIG9udG8gdGhlIGxpdmUgYFNoZWxsU3RhdGVgIGJ5IGRpc3BhdGNoaW5nIHRoZSBTQU1FIGV4aXN0aW5nLCB0YXJnZXRlZCBgU2hlbGxBY3Rpb25gcyB0aGUgcmVhbCBVSSdzIG93biBpbnRlcmFjdGlvbnMgdXNlIOKAlCBuZXZlciBhIGJlc3Bva2UgdHV0b3JpYWwtb25seSBtdXRhdGlvbiBjaGFubmVsLiAqL1xuZXhwb3J0IGZ1bmN0aW9uIGFwcGx5VHV0b3JpYWxVaUNoYW5nZVRvU2hlbGwoZGlzcGF0Y2g6IChhY3Rpb246IFNoZWxsQWN0aW9uKSA9PiB2b2lkLCBjaGFuZ2U6IFR1dG9yaWFsVWlDaGFuZ2UsIGN0eDogVHV0b3JpYWxVaUJyaWRnZUNvbnRleHQpOiB2b2lkIHtcbiAgc3dpdGNoIChjaGFuZ2Uua2luZCkge1xuICAgIGNhc2UgXCJhY3RpdmVNb2RlXCI6XG4gICAgICBpZiAoIWN0eC5zZXNzaW9uKSByZXR1cm47XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1NFU1NJT05cIiwgdmFsdWU6IChjdXJyZW50KSA9PiAoY3VycmVudCA/IHsgLi4uY3VycmVudCwgdmlld1N0YXRlOiB7IC4uLmN1cnJlbnQudmlld1N0YXRlLCBhY3RpdmVNb2RlSWQ6IGNoYW5nZS5pZCB9IH0gOiBjdXJyZW50KSB9KTtcbiAgICAgIHJldHVybjtcbiAgICBjYXNlIFwiZm9jdXNlZFdpbmRvd1wiOlxuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9BQ1RJVkVfV0lORE9XX0lEXCIsIHZhbHVlOiBjaGFuZ2UuaWQgPz8gbnVsbCB9KTtcbiAgICAgIHJldHVybjtcbiAgICBjYXNlIFwiYWN0aXZlVXRpbGl0eVwiOlxuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9BQ1RJVkVfVVRJTElUWVwiLCB3aW5kb3dJZDogY2hhbmdlLndpbmRvd0lkLCB1dGlsaXR5SWQ6IGNoYW5nZS51dGlsaXR5SWQgPz8gbnVsbCB9KTtcbiAgICAgIHJldHVybjtcbiAgICBjYXNlIFwiYWN0aXZlVG9vbFwiOlxuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9BQ1RJVkVfVE9PTFwiLCB0b29sSWQ6IGNoYW5nZS5pZCA/PyBudWxsIH0pO1xuICAgICAgcmV0dXJuO1xuICAgIGNhc2UgXCJsYXlvdXRcIjoge1xuICAgICAgY29uc3Qgd2luZG93S2luZHMgPSBjdHguc2Vzc2lvbj8uYXBwLndpbmRvd0tpbmRzLm1hcCgoa2luZCkgPT4gKHsgaWQ6IGtpbmQuaWQsIGxhYmVsOiBraW5kLmxhYmVsIH0pKSA/PyBbXTtcbiAgICAgIGNvbnN0IHNlZWQgPSBhcHBseUZyYW1ld29ya0xheW91dFNlZWQoY2hhbmdlLmxheW91dCwgd2luZG93S2luZHMsIGN0eC5hcHBMYWJlbHNPdmVybGF5LCBjdHgudGVybWlub2xvZ3ksIGN0eC5sb2NhbGUpO1xuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9TSEVMTF9MQVlPVVRcIiwgdmFsdWU6IHNlZWQubW9kZUxheW91dCB9KTtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfRVhUUkFfV0lORE9XX0lOU1RBTkNFU1wiLCB2YWx1ZTogc2VlZC5leHRyYUluc3RhbmNlcyB9KTtcbiAgICAgIHJldHVybjtcbiAgICB9XG4gICAgY2FzZSBcInBhbmVsVGFiXCI6IHtcbiAgICAgIGNvbnN0IGFuY2hvciA9IGNoYW5nZS5ncm91cCBhcyBBbmNob3I7XG4gICAgICBpZiAoIShBTkNIT1JTIGFzIHJlYWRvbmx5IHN0cmluZ1tdKS5pbmNsdWRlcyhhbmNob3IpKSByZXR1cm47XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1BBTkVMX1ZJU0lCTEVcIiwgYW5jaG9yLCB2YWx1ZTogY2hhbmdlLnRhYklkICE9IG51bGwgfSk7XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1BBTkVMX1BBVEhcIiwgYW5jaG9yLCB2YWx1ZTogY2hhbmdlLnRhYklkID8gW2NoYW5nZS50YWJJZF0gOiBbXSB9KTtcbiAgICAgIHJldHVybjtcbiAgICB9XG4gICAgY2FzZSBcInBhbmVsU3RhdGVcIjpcbiAgICAgIGlmICghY3R4LnNlc3Npb24pIHJldHVybjtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfU0VTU0lPTlwiLCB2YWx1ZTogKGN1cnJlbnQpID0+IChjdXJyZW50ID8geyAuLi5jdXJyZW50LCB2aWV3U3RhdGU6IHsgLi4uY3VycmVudC52aWV3U3RhdGUsIHBhbmVsSnNvbjogY2hhbmdlLnBhbmVsSnNvbiB9IH0gOiBjdXJyZW50KSB9KTtcbiAgICAgIHJldHVybjtcbiAgICBjYXNlIFwic2VsZWN0aW9uXCI6XG4gICAgICBpZiAoIWN0eC5zZXNzaW9uKSByZXR1cm47XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1NFU1NJT05cIiwgdmFsdWU6IChjdXJyZW50KSA9PiAoY3VycmVudCA/IHsgLi4uY3VycmVudCwgdmlld1N0YXRlOiB7IC4uLmN1cnJlbnQudmlld1N0YXRlLCBzZWxlY3Rpb25Kc29uOiBjaGFuZ2Uuc2VsZWN0aW9uSnNvbiB9IH0gOiBjdXJyZW50KSB9KTtcbiAgICAgIHJldHVybjtcbiAgICBjYXNlIFwiZGlhbG9nXCI6XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0RJQUxPR1wiLCB2YWx1ZTogY2hhbmdlLmlkID8geyBkaWFsb2dJZDogY2hhbmdlLmlkLCBzZWVkQXJnczogY2hhbmdlLmFyZ3MgYXMgUmVjb3JkPHN0cmluZywgdW5rbm93bj4gfCB1bmRlZmluZWQgfSA6IG51bGwgfSk7XG4gICAgICByZXR1cm47XG4gICAgY2FzZSBcInRyZWVFeHBhbnNpb25cIjpcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfVFJFRV9PUEVOX1NUQVRFXCIsIGlkOiBjaGFuZ2UuaWQsIG9wZW46IGNoYW5nZS5leHBhbmRlZCB9KTtcbiAgICAgIHJldHVybjtcbiAgICBjYXNlIFwiY29tbWFuZFBhbmVsXCI6XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1NFQVJDSF9PUEVOXCIsIHZhbHVlOiBjaGFuZ2Uub3BlbiB9KTtcbiAgICAgIHJldHVybjtcbiAgICBkZWZhdWx0OlxuICAgICAgcmV0dXJuO1xuICB9XG59XG4vLyNlbmRyZWdpb24g8J+Ope+4j1R1dG9yaWFsVWlCcmlkZ2VcblxuLyoqIEBlbW9qaSDwn5ej77iPIFJlc29sdmVzIGEgdGVybWlub2xvZ3kgaWQncyBkaXNwbGF5IG5hbWU7IGNocm9tZS1rbm93biBpZHMgZ2V0IGEgdHJhbnNsYXRlZCBsYWJlbCwgYXBwLWRlY2xhcmVkIGlkcyBmYWxsIGJhY2sgdG8gdGhlaXIgcmF3IGlkLiAqL1xuZXhwb3J0IGZ1bmN0aW9uIHNoZWxsVGVybWlub2xvZ3lMYWJlbChpZDogc3RyaW5nKTogc3RyaW5nIHtcbiAgY29uc3QgaXNDaHJvbWVLbm93biA9IGlkID09PSBcIm5hdGl2ZVwiIHx8IGlkID09PSBcInJldXNlXCI7XG4gIHJldHVybiBpc0Nocm9tZUtub3duID8gc2hlbGxMYWJlbChgdWkuc2V0dGluZ3MudGVybWlub2xvZ3kuJHtpZCBhcyBVaUNocm9tZVRlcm1pbm9sb2d5SWR9YCkgOiBpZDtcbn1cblxuLyoqIEBlbW9qaSDwn46a77iPIFNlcmlhbGl6ZXMgYXN5bmMgdXBkYXRlcyB3aGlsZSByZXRhaW5pbmcgb25seSB0aGUgbmV3ZXN0IHZhbHVlIHJlcXVlc3RlZCBkdXJpbmcgYW4gaW4tZmxpZ2h0IHVwZGF0ZS4gKi9cbmV4cG9ydCBmdW5jdGlvbiBjcmVhdGVMYXRlc3RBc3luY0Rpc3BhdGNoZXI8VD4oZGlzcGF0Y2hWYWx1ZTogKHZhbHVlOiBUKSA9PiB1bmtub3duKTogKHZhbHVlOiBUKSA9PiB2b2lkIHtcbiAgbGV0IHJ1bm5pbmcgPSBmYWxzZTtcbiAgbGV0IHF1ZXVlZDogVCB8IHVuZGVmaW5lZDtcbiAgbGV0IGhhc1F1ZXVlZCA9IGZhbHNlO1xuICBjb25zdCBkaXNwYXRjaExhdGVzdCA9ICh2YWx1ZTogVCkgPT4ge1xuICAgIGlmIChydW5uaW5nKSB7XG4gICAgICBxdWV1ZWQgPSB2YWx1ZTtcbiAgICAgIGhhc1F1ZXVlZCA9IHRydWU7XG4gICAgICByZXR1cm47XG4gICAgfVxuICAgIHJ1bm5pbmcgPSB0cnVlO1xuICAgIHZvaWQgUHJvbWlzZS5yZXNvbHZlKGRpc3BhdGNoVmFsdWUodmFsdWUpKS5maW5hbGx5KCgpID0+IHtcbiAgICAgIHJ1bm5pbmcgPSBmYWxzZTtcbiAgICAgIGlmICghaGFzUXVldWVkKSByZXR1cm47XG4gICAgICBjb25zdCBuZXh0ID0gcXVldWVkIGFzIFQ7XG4gICAgICBxdWV1ZWQgPSB1bmRlZmluZWQ7XG4gICAgICBoYXNRdWV1ZWQgPSBmYWxzZTtcbiAgICAgIGRpc3BhdGNoTGF0ZXN0KG5leHQpO1xuICAgIH0pO1xuICB9O1xuICByZXR1cm4gZGlzcGF0Y2hMYXRlc3Q7XG59XG5cbi8qKiBAZW1vamkg4oaV77iPIFNlcmlhbGl6ZXMgbnVtZXJpYyBzbGlkZXIgdXBkYXRlcyB3aGlsZSByZXRhaW5pbmcgZXZlcnkgZGlyZWN0aW9uIGNoYW5nZSBhbmQgY29hbGVzY2luZyBtb3ZlbWVudCB3aXRoaW4gb25lIGRpcmVjdGlvbi4gKi9cbmV4cG9ydCBmdW5jdGlvbiBjcmVhdGVEaXJlY3Rpb25hbEFzeW5jRGlzcGF0Y2hlcihkaXNwYXRjaFZhbHVlOiAodmFsdWU6IG51bWJlcikgPT4gdW5rbm93bik6ICh2YWx1ZTogbnVtYmVyKSA9PiB2b2lkIHtcbiAgbGV0IHJ1bm5pbmcgPSBmYWxzZTtcbiAgbGV0IGFjdGl2ZSA9IDA7XG4gIGNvbnN0IHF1ZXVlZDogbnVtYmVyW10gPSBbXTtcbiAgY29uc3QgZGlzcGF0Y2hOZXh0ID0gKHZhbHVlOiBudW1iZXIpID0+IHtcbiAgICBydW5uaW5nID0gdHJ1ZTtcbiAgICBhY3RpdmUgPSB2YWx1ZTtcbiAgICB2b2lkIFByb21pc2UucmVzb2x2ZShkaXNwYXRjaFZhbHVlKHZhbHVlKSkuZmluYWxseSgoKSA9PiB7XG4gICAgICBjb25zdCBuZXh0ID0gcXVldWVkLnNoaWZ0KCk7XG4gICAgICBpZiAobmV4dCA9PT0gdW5kZWZpbmVkKSB7XG4gICAgICAgIHJ1bm5pbmcgPSBmYWxzZTtcbiAgICAgICAgcmV0dXJuO1xuICAgICAgfVxuICAgICAgZGlzcGF0Y2hOZXh0KG5leHQpO1xuICAgIH0pO1xuICB9O1xuICByZXR1cm4gKHZhbHVlKSA9PiB7XG4gICAgaWYgKCFydW5uaW5nKSB7XG4gICAgICBkaXNwYXRjaE5leHQodmFsdWUpO1xuICAgICAgcmV0dXJuO1xuICAgIH1cbiAgICBjb25zdCBwcmV2aW91cyA9IHF1ZXVlZC5hdCgtMSk7XG4gICAgaWYgKHByZXZpb3VzID09PSB1bmRlZmluZWQpIHtcbiAgICAgIGlmICh2YWx1ZSAhPT0gYWN0aXZlKSBxdWV1ZWQucHVzaCh2YWx1ZSk7XG4gICAgICByZXR1cm47XG4gICAgfVxuICAgIGNvbnN0IGFuY2hvciA9IHF1ZXVlZC5hdCgtMikgPz8gYWN0aXZlO1xuICAgIGNvbnN0IGRpcmVjdGlvbiA9IE1hdGguc2lnbihwcmV2aW91cyAtIGFuY2hvcik7XG4gICAgY29uc3QgbmV4dERpcmVjdGlvbiA9IE1hdGguc2lnbih2YWx1ZSAtIHByZXZpb3VzKTtcbiAgICBpZiAobmV4dERpcmVjdGlvbiA9PT0gMCkgcmV0dXJuO1xuICAgIGlmIChkaXJlY3Rpb24gPT09IDAgfHwgbmV4dERpcmVjdGlvbiA9PT0gZGlyZWN0aW9uKSBxdWV1ZWRbcXVldWVkLmxlbmd0aCAtIDFdID0gdmFsdWU7XG4gICAgZWxzZSBxdWV1ZWQucHVzaCh2YWx1ZSk7XG4gICAgLy8g8J+Uge+4jyBBIGppdHRlcnkgZHJhZyAocmFwaWQgZGlyZWN0aW9uIHJldmVyc2FscyB3aGlsZSBhIHJvdW5kIHRyaXAgaXMgaW4gZmxpZ2h0KSB3b3VsZCBvdGhlcndpc2UgZ3Jvd1xuICAgIC8vIGBxdWV1ZWRgIGJ5IG9uZSBlbnRyeSBwZXIgcmV2ZXJzYWw7IG9ubHkgdGhlIGxhc3QgdHdvIGFyZSBldmVyIG5lZWRlZCAodGhlIHBlbmRpbmcgdmFsdWUgYW5kIHRoZVxuICAgIC8vIGFuY2hvciB1c2VkIHRvIGRldGVjdCB0aGUgbmV4dCByZXZlcnNhbCksIHNvIGNhcCBpdCB0aGVyZS5cbiAgICBpZiAocXVldWVkLmxlbmd0aCA+IDIpIHF1ZXVlZC5zcGxpY2UoMCwgcXVldWVkLmxlbmd0aCAtIDIpO1xuICB9O1xufVxuXG4vLyNyZWdpb24gUmV2ZWFsQ3V0b2ZmU3RvcmVcbi8qKlxuICogQGVtb2ppIPCfqqPvuI8gTGl2ZSBwZXItZ2VzdHVyZSB2aXNpYmlsaXR5IGN1dG9mZiBmb3IgcmV2ZWFsLXRhZ2dlZCBpbnN0YW5jZXMgKGBXb3JsZEluc3RhbmNlUmVjb3JkLnJldmVhbEluZGV4YCxcbiAqIHNldCBieSBhIGBXaW5kb3dNZWFzdXJlLlNsaWRlci5yZXZlYWxgIGdyb3VwKS4gTWFpbi10aHJlYWQtb25seSBhbmQgbmV2ZXIgZGlzcGF0Y2hlZDogYSBzbGlkZXIgZHJhZyB3cml0ZXNcbiAqIGhlcmUgZGlyZWN0bHksIGBXb3JsZEluc3RhbmNlc0xheWVyYCBzdWJzY3JpYmVzIGFuZCBpbXBlcmF0aXZlbHkgdG9nZ2xlcyBgT2JqZWN0M0QudmlzaWJsZWAg4oCUIHplcm8gUmVhY3RcbiAqIHJlLXJlbmRlciwgemVybyBXQVNNIHJvdW5kIHRyaXAuIFJlY29uY2lsZWQgZnJvbSB0aGUgcGx1Z2luJ3MgY29tbWl0dGVkIGBXb3JsZEludGVyYWN0aW9uUmVjb3JkLnJldmVhbEN1dG9mZnNgXG4gKiB3aGVuZXZlciB0aGF0IHZhbHVlIGNoYW5nZXMgKGEgbm8tb3BlcmF0aW9uIGR1cmluZyBhIGxpdmUgZHJhZywgc2luY2UgdGhlIGNvbW1pdHRlZCB2YWx1ZSBvbmx5IGNoYW5nZXMgb24gY29tbWl0KS5cbiAqL1xuZXhwb3J0IHR5cGUgUmV2ZWFsQ3V0b2ZmU3RvcmUgPSB7XG4gIGdldChncm91cElkOiBzdHJpbmcpOiBudW1iZXIgfCB1bmRlZmluZWQ7XG4gIHNldChncm91cElkOiBzdHJpbmcsIHZhbHVlOiBudW1iZXIpOiB2b2lkO1xuICBzdWJzY3JpYmUoZ3JvdXBJZDogc3RyaW5nLCBsaXN0ZW5lcjogKHZhbHVlOiBudW1iZXIgfCB1bmRlZmluZWQpID0+IHZvaWQpOiAoKSA9PiB2b2lkO1xufTtcblxuZXhwb3J0IGZ1bmN0aW9uIGNyZWF0ZVJldmVhbEN1dG9mZlN0b3JlKCk6IFJldmVhbEN1dG9mZlN0b3JlIHtcbiAgY29uc3QgdmFsdWVzID0gbmV3IE1hcDxzdHJpbmcsIG51bWJlcj4oKTtcbiAgY29uc3QgbGlzdGVuZXJzID0gbmV3IE1hcDxzdHJpbmcsIFNldDwodmFsdWU6IG51bWJlciB8IHVuZGVmaW5lZCkgPT4gdm9pZD4+KCk7XG4gIHJldHVybiB7XG4gICAgZ2V0OiAoZ3JvdXBJZCkgPT4gdmFsdWVzLmdldChncm91cElkKSxcbiAgICBzZXQ6IChncm91cElkLCB2YWx1ZSkgPT4ge1xuICAgICAgdmFsdWVzLnNldChncm91cElkLCB2YWx1ZSk7XG4gICAgICBmb3IgKGNvbnN0IGxpc3RlbmVyIG9mIGxpc3RlbmVycy5nZXQoZ3JvdXBJZCkgPz8gW10pIGxpc3RlbmVyKHZhbHVlKTtcbiAgICB9LFxuICAgIHN1YnNjcmliZTogKGdyb3VwSWQsIGxpc3RlbmVyKSA9PiB7XG4gICAgICBsZXQgZ3JvdXAgPSBsaXN0ZW5lcnMuZ2V0KGdyb3VwSWQpO1xuICAgICAgaWYgKCFncm91cCkge1xuICAgICAgICBncm91cCA9IG5ldyBTZXQoKTtcbiAgICAgICAgbGlzdGVuZXJzLnNldChncm91cElkLCBncm91cCk7XG4gICAgICB9XG4gICAgICBncm91cC5hZGQobGlzdGVuZXIpO1xuICAgICAgcmV0dXJuICgpID0+IHtcbiAgICAgICAgZ3JvdXAhLmRlbGV0ZShsaXN0ZW5lcik7XG4gICAgICB9O1xuICAgIH0sXG4gIH07XG59XG5cbi8qKiBTaGFyZWQgaW5zdGFuY2Ug4oCUIGEgcmV2ZWFsIGdyb3VwIGlkIGlzIGFwcC1pbnN0YW5jZS1nbG9iYWwgaW4gdjE7IG5hbWVzcGFjZSBieSBhcHAgaW5zdGFuY2UgaWQgaWYgYSBzZWNvbmQgY29uY3VycmVudCBkb2N1bWVudCBpbnN0YW5jZSBldmVyIG5lZWRzIGluZGVwZW5kZW50IGN1dG9mZnMuICovXG5leHBvcnQgY29uc3Qgd29ybGRSZXZlYWxDdXRvZmZTdG9yZSA9IGNyZWF0ZVJldmVhbEN1dG9mZlN0b3JlKCk7XG5cbi8qKiBUaGUgb25seSByZXZlYWwgZ3JvdXAgdGhhdCBleGlzdHMgdG9kYXkg4oCUIHB1enpsZTNkJ3MgZmlsbC1wbGFuIHNsaWRlci4gKi9cbmV4cG9ydCBjb25zdCBQVVpaTEUzRF9GSUxMX1JFVkVBTF9HUk9VUF9JRCA9IFwicHV6emxlM2QtZmlsbFwiO1xuXG4vKipcbiAqIEBlbW9qaSDwn6qj77iPIFdyaXRlcyBjb21taXR0ZWQgcmV2ZWFsIGN1dG9mZnMgaW50byBgc3RvcmVgIG9ubHkgd2hlbiB0aGUgbnVtZXJpYyB2YWx1ZSBmb3IgYSBncm91cCBjaGFuZ2VzLlxuICogSWdub3JlcyBvYmplY3QtaWRlbnRpdHkgY2h1cm4gZnJvbSBgZmlsbEJ1aWxkVGlja2AgcmVmcmVzaGVzIHNvIGEgbGl2ZSBzbGlkZXIgZHJhZyBpcyBub3QgcmVzZXQgbWlkLWdlc3R1cmUuXG4gKi9cbmV4cG9ydCBmdW5jdGlvbiByZWNvbmNpbGVDb21taXR0ZWRSZXZlYWxDdXRvZmZzKFxuICBzdG9yZTogUmV2ZWFsQ3V0b2ZmU3RvcmUsXG4gIGNvbW1pdHRlZFJlZjogeyBjdXJyZW50OiBSZWFkb25seTxSZWNvcmQ8c3RyaW5nLCBudW1iZXI+PiB9LFxuICByZXZlYWxDdXRvZmZzOiBSZWFkb25seTxSZWNvcmQ8c3RyaW5nLCBudW1iZXI+Pixcbik6IHZvaWQge1xuICBmb3IgKGNvbnN0IFtncm91cElkLCB2YWx1ZV0gb2YgT2JqZWN0LmVudHJpZXMocmV2ZWFsQ3V0b2ZmcykpIHtcbiAgICBpZiAoY29tbWl0dGVkUmVmLmN1cnJlbnRbZ3JvdXBJZF0gPT09IHZhbHVlKSBjb250aW51ZTtcbiAgICBjb21taXR0ZWRSZWYuY3VycmVudCA9IHsgLi4uY29tbWl0dGVkUmVmLmN1cnJlbnQsIFtncm91cElkXTogdmFsdWUgfTtcbiAgICBzdG9yZS5zZXQoZ3JvdXBJZCwgdmFsdWUpO1xuICB9XG59XG5cbi8qKiBAZW1vamkg8J+ZiO+4jyBUcnVlIGZvciBhIHJldmVhbC10YWdnZWQgaW5zdGFuY2UgYmV5b25kIHRoZSBsaXZlIGN1dG9mZiDigJQgYFdvcmxkSW5zdGFuY2VzTGF5ZXJgIGFscmVhZHlcbiAqIGhpZGVzIGl0cyByb290IGltcGVyYXRpdmVseSwgYnV0IHB1cmUgZnVuY3Rpb25zIHRoYXQgcmVhZCBgaW5zdGFuY2VzYCBkYXRhIGRpcmVjdGx5IChtYXJxdWVlIGhpdFxuICogdGVzdGluZykgZG9uJ3Qgc2VlIHRocmVlLmpzIGBPYmplY3QzRC52aXNpYmxlYCBhbmQgbmVlZCB0aGlzIGNoZWNrIGluc3RlYWQuIFVudGFnZ2VkIGluc3RhbmNlcyBhcmVcbiAqIG5ldmVyIGN1dG9mZi1oaWRkZW46IHRoZSBudWxsaXNoIGd1YXJkIGFsc28gcmVqZWN0cyBhIEpTT04gYG51bGxgLCB3aGljaCB3b3VsZCBvdGhlcndpc2UgY29tcGFyZSBhcyBgMGBcbiAqIGFuZCBoaWRlIGV2ZXJ5IG9yZGluYXJ5IG9iamVjdCB3aGlsZSB0aGUgY3V0b2ZmIHNpdHMgYXQgaXRzIGJvb3QgdmFsdWUuICovXG5leHBvcnQgZnVuY3Rpb24gaXNSZXZlYWxDdXRvZmZIaWRkZW4oaW5zdGFuY2U6IFBpY2s8V29ybGRJbnN0YW5jZVJlY29yZCwgXCJyZXZlYWxJbmRleFwiPik6IGJvb2xlYW4ge1xuICBpZiAoaW5zdGFuY2UucmV2ZWFsSW5kZXggPT0gbnVsbCkgcmV0dXJuIGZhbHNlO1xuICBjb25zdCBjdXRvZmYgPSB3b3JsZFJldmVhbEN1dG9mZlN0b3JlLmdldChQVVpaTEUzRF9GSUxMX1JFVkVBTF9HUk9VUF9JRCk7XG4gIHJldHVybiBjdXRvZmYgIT09IHVuZGVmaW5lZCAmJiBpbnN0YW5jZS5yZXZlYWxJbmRleCA+PSBjdXRvZmY7XG59XG4vLyNlbmRyZWdpb24gUmV2ZWFsQ3V0b2ZmU3RvcmVcblxuLyoqXG4gKiBAZW1vamkg8J+apu+4jyBGaXJlcyBgcnVuYCBhdCBtb3N0IG9uY2UgYXQgYSB0aW1lIOKAlCBpbnRlcnZhbCB0aWNrcyB0aGF0IGFycml2ZSB3aGlsZSBhIHByZXZpb3VzIHJ1biBpcyBzdGlsbFxuICogaW4gZmxpZ2h0IGFyZSBkcm9wcGVkIChub3QgcXVldWVkKS4gVXNlZCBieSBXb3JsZDNkSG9zdCdzIGBzdWdnZXN0aW9uc1RpY2tgL2BmaWxsQnVpbGRUaWNrYCBsb29wcyBzbyBhXG4gKiBzbG93IHByb2dyYW0gdGljayBjYW5ub3QgdW5ib3VuZGVkLXF1ZXVlIGludG8gdGhlIHNlcmlhbGl6ZWQgV0FTTSBoYW5kbGUgYW5kIHN0YXJ2ZSB0aGUgZmlsbCB1dGlsaXR5LlxuICovXG5leHBvcnQgZnVuY3Rpb24gY3JlYXRlSW5GbGlnaHRTa2lwcGluZ0ludGVydmFsKHJ1bjogKCkgPT4gdW5rbm93biwgZGVsYXlNczogbnVtYmVyLCBzZXRJbnRlcnZhbEZuOiB0eXBlb2Ygc2V0SW50ZXJ2YWwgPSBzZXRJbnRlcnZhbCwgY2xlYXJJbnRlcnZhbEZuOiB0eXBlb2YgY2xlYXJJbnRlcnZhbCA9IGNsZWFySW50ZXJ2YWwpOiAoKSA9PiB2b2lkIHtcbiAgbGV0IGNhbmNlbGxlZCA9IGZhbHNlO1xuICBsZXQgaW5GbGlnaHQgPSBmYWxzZTtcbiAgY29uc3QgdGljayA9ICgpID0+IHtcbiAgICBpZiAoY2FuY2VsbGVkIHx8IGluRmxpZ2h0KSByZXR1cm47XG4gICAgaW5GbGlnaHQgPSB0cnVlO1xuICAgIHZvaWQgUHJvbWlzZS5yZXNvbHZlKHJ1bigpKS5maW5hbGx5KCgpID0+IHtcbiAgICAgIGluRmxpZ2h0ID0gZmFsc2U7XG4gICAgfSk7XG4gIH07XG4gIGNvbnN0IHRpbWVyID0gc2V0SW50ZXJ2YWxGbih0aWNrLCBkZWxheU1zKTtcbiAgcmV0dXJuICgpID0+IHtcbiAgICBjYW5jZWxsZWQgPSB0cnVlO1xuICAgIGNsZWFySW50ZXJ2YWxGbih0aW1lcik7XG4gIH07XG59XG5cbi8qKlxuICogQGVtb2ppIPCfjq/vuI8gQ29hbGVzY2VzIHJhcGlkIGRpc3BhdGNoZXMgdG8gdGhlIGxhdGVzdCB2YWx1ZSDigJQgc2tpcHMgd2hlbiB1bmNoYW5nZWQgYW5kIGtlZXBzIGF0IG1vc3Qgb25lXG4gKiBpbi1mbGlnaHQgcm91bmQgdHJpcCAodXNlZCBieSBXb3JsZDNkSG9zdCBob3ZlciBzbyBwb2ludGVybW92ZSBjYW5ub3QgZmxvb2QgdGhlIFdBU00gaGFuZGxlKS5cbiAqL1xuZXhwb3J0IGZ1bmN0aW9uIGNyZWF0ZUNvYWxlc2NpbmdBY3Rpb25EaXNwYXRjaGVyPFQ+KGRpc3BhdGNoOiAodmFsdWU6IFQpID0+IHVua25vd24sIGlzRXF1YWw6IChhOiBULCBiOiBUKSA9PiBib29sZWFuID0gKGEsIGIpID0+IE9iamVjdC5pcyhhLCBiKSk6ICh2YWx1ZTogVCkgPT4gdm9pZCB7XG4gIGxldCBpbkZsaWdodCA9IGZhbHNlO1xuICBsZXQgcGVuZGluZzogVCB8IHVuZGVmaW5lZDtcbiAgbGV0IGxhc3RTZW50OiBUIHwgdW5kZWZpbmVkO1xuICBjb25zdCBmbHVzaCA9ICgpID0+IHtcbiAgICBpZiAoaW5GbGlnaHQgfHwgcGVuZGluZyA9PT0gdW5kZWZpbmVkKSByZXR1cm47XG4gICAgY29uc3QgbmV4dCA9IHBlbmRpbmc7XG4gICAgcGVuZGluZyA9IHVuZGVmaW5lZDtcbiAgICBpZiAobGFzdFNlbnQgIT09IHVuZGVmaW5lZCAmJiBpc0VxdWFsKGxhc3RTZW50LCBuZXh0KSkgcmV0dXJuO1xuICAgIGxhc3RTZW50ID0gbmV4dDtcbiAgICBpbkZsaWdodCA9IHRydWU7XG4gICAgdm9pZCBQcm9taXNlLnJlc29sdmUoZGlzcGF0Y2gobmV4dCkpLmZpbmFsbHkoKCkgPT4ge1xuICAgICAgaW5GbGlnaHQgPSBmYWxzZTtcbiAgICAgIGZsdXNoKCk7XG4gICAgfSk7XG4gIH07XG4gIHJldHVybiAodmFsdWU6IFQpID0+IHtcbiAgICBpZiAocGVuZGluZyA9PT0gdW5kZWZpbmVkICYmIGxhc3RTZW50ICE9PSB1bmRlZmluZWQgJiYgaXNFcXVhbChsYXN0U2VudCwgdmFsdWUpKSByZXR1cm47XG4gICAgcGVuZGluZyA9IHZhbHVlO1xuICAgIGZsdXNoKCk7XG4gIH07XG59XG5cbmV4cG9ydCBjb25zdCByZWdpc3RlcmVkUHV6emxlM2RCcnVzaE1lc2hlcyA9IG5ldyBTZXQ8c3RyaW5nPigpO1xuXG4vKiogQGVtb2ppIPCfjprvuI8gV2hldGhlciBhbnkgbWVhc3VyZSAoaW5jbHVkaW5nIG5lc3RlZCBncm91cCBjaGlsZHJlbikgZGVjbGFyZXMgYGlkYC4gKi9cbmV4cG9ydCBmdW5jdGlvbiB3aW5kb3dNZWFzdXJlVHJlZUNvbnRhaW5zSWQobWVhc3VyZXM6IHJlYWRvbmx5IFdpbmRvd01lYXN1cmVbXSwgaWQ6IHN0cmluZyk6IGJvb2xlYW4ge1xuICBmb3IgKGNvbnN0IG1lYXN1cmUgb2YgbWVhc3VyZXMpIHtcbiAgICBpZiAobWVhc3VyZS5pZCA9PT0gaWQpIHJldHVybiB0cnVlO1xuICAgIGlmIChtZWFzdXJlLmtpbmQgPT09IFwiZ3JvdXBcIiAmJiB3aW5kb3dNZWFzdXJlVHJlZUNvbnRhaW5zSWQobWVhc3VyZS5jaGlsZHJlbiwgaWQpKSByZXR1cm4gdHJ1ZTtcbiAgfVxuICByZXR1cm4gZmFsc2U7XG59XG5cbi8qKiBAZW1vamkg8J+Tiu+4jyBQcm9iYWJpbGl0eSB3ZWlnaHRzICgw4oCTMSBzaW1wbGV4IHNsaWRlcnMpIHJlYWQgb3V0IGFzIHdob2xlLXBlcmNlbnQgbGFiZWxzLCBub3QgcmF3IGZyYWN0aW9ucy4gKi9cbmZ1bmN0aW9uIHdpbmRvd01lYXN1cmVVc2VzUHJvYmFiaWxpdHlSZWFkb3V0KG1lYXN1cmU6IEV4dHJhY3Q8V2luZG93TWVhc3VyZSwgeyBraW5kOiBcInNsaWRlclwiIH0+KTogYm9vbGVhbiB7XG4gIGNvbnN0IHN0ZXAgPSBtZWFzdXJlLnN0ZXAgPz8gMTtcbiAgcmV0dXJuIG1lYXN1cmUubWluID09PSAwICYmIG1lYXN1cmUubWF4IDw9IDEgJiYgc3RlcCA8IDE7XG59XG5cbmZ1bmN0aW9uIHdpbmRvd01lYXN1cmVQcm9iYWJpbGl0eVJlYWRvdXQodmFsdWU6IG51bWJlcik6IHN0cmluZyB7XG4gIHJldHVybiBgJHtNYXRoLnJvdW5kKHZhbHVlICogMTAwKX0lYDtcbn1cblxuLyoqIEBlbW9qaSDwn46a77iPIEtlZXBzIGEgbWVhc3VyZSBzbGlkZXIgbGl2ZSB3aXRob3V0IGFjY3VtdWxhdGluZyBzdGFsZSBkb2N1bWVudCBhY3Rpb25zIGJlaGluZCB0aGUgcG9pbnRlci4gKi9cbmZ1bmN0aW9uIFdpbmRvd01lYXN1cmVTbGlkZXIoeyBtZWFzdXJlLCBvbkFjdGlvbiB9OiB7IHJlYWRvbmx5IG1lYXN1cmU6IEV4dHJhY3Q8V2luZG93TWVhc3VyZSwgeyBraW5kOiBcInNsaWRlclwiIH0+OyByZWFkb25seSBvbkFjdGlvbjogKGFjdGlvbjogQWN0aW9uRGVzY3JpcHRvcikgPT4gdW5rbm93biB9KSB7XG4gIGNvbnN0IGRpc3BhdGNoVmFsdWUgPSB1c2VNZW1vKFxuICAgICgpID0+IGNyZWF0ZURpcmVjdGlvbmFsQXN5bmNEaXNwYXRjaGVyKCh2YWx1ZSkgPT4gb25BY3Rpb24oeyAuLi5tZWFzdXJlLm9uQ2hhbmdlLCBhcmdzOiB7IC4uLihtZWFzdXJlLm9uQ2hhbmdlLmFyZ3MgYXMgb2JqZWN0IHwgdW5kZWZpbmVkKSwgdmFsdWUgfSB9KSksXG4gICAgW21lYXN1cmUub25DaGFuZ2UsIG9uQWN0aW9uXSxcbiAgKTtcbiAgY29uc3QgZm9ybWF0RGlzcGxheVZhbHVlID0gd2luZG93TWVhc3VyZVVzZXNQcm9iYWJpbGl0eVJlYWRvdXQobWVhc3VyZSkgPyB3aW5kb3dNZWFzdXJlUHJvYmFiaWxpdHlSZWFkb3V0IDogdW5kZWZpbmVkO1xuICBjb25zdCBkaXNhYmxlZCA9IG1lYXN1cmUuZGlzYWJsZWQgPT09IHRydWU7XG4gIC8vIPCfqqPvuI8gQSByZXZlYWwtZ3JvdXAgbWVhc3VyZSAoZS5nLiBwdXp6bGUzZCdzIGZpbGwtY291bnQgc2xpZGVyKSBtdXN0IG5vdCByb3VuZC10cmlwIHRocm91Z2ggV0FTTSBvblxuICAvLyBldmVyeSBkcmFnIHZhbHVlIOKAlCB0aGUgcGx1Z2luIGFscmVhZHkgcmVuZGVyZWQgZXZlcnkgcGxhbm5lZCBwaWVjZSB0YWdnZWQgd2l0aCBpdHMgcmV2ZWFsIGluZGV4LCBzb1xuICAvLyBkcmFnZ2luZyBvbmx5IG5lZWRzIHRvIG1vdmUgYSBtYWluLXRocmVhZCB2aXNpYmlsaXR5IGN1dG9mZi4gT25seSB0aGUgZmluYWwgdmFsdWUgcm91bmQtdHJpcHMsIG9uY2UsXG4gIC8vIG9uIGdlc3R1cmUgcmVsZWFzZS5cbiAgY29uc3QgcmV2ZWFsR3JvdXBJZCA9IG1lYXN1cmUucmV2ZWFsO1xuXG4gIHJldHVybiAoXG4gICAgPFNsaWRlclxuICAgICAgaWQ9e21lYXN1cmUuaWR9XG4gICAgICB2YWx1ZT17W21lYXN1cmUudmFsdWVdfVxuICAgICAgbWluPXttZWFzdXJlLm1pbn1cbiAgICAgIG1heD17bWVhc3VyZS5tYXh9XG4gICAgICByZWFkeT17bWVhc3VyZS5yZWFkeX1cbiAgICAgIGxvYWRpbmc9e21lYXN1cmUubG9hZGluZyA9PT0gdHJ1ZX1cbiAgICAgIHdhaXRpbmc9e21lYXN1cmUud2FpdGluZyA9PT0gdHJ1ZX1cbiAgICAgIHN0ZXA9e21lYXN1cmUuc3RlcH1cbiAgICAgIGRpc2FibGVkPXtkaXNhYmxlZH1cbiAgICAgIGNsYW1wVG9SZWFkeT17Qm9vbGVhbihyZXZlYWxHcm91cElkKX1cbiAgICAgIGZvcm1hdERpc3BsYXlWYWx1ZT17Zm9ybWF0RGlzcGxheVZhbHVlfVxuICAgICAgb25WYWx1ZUNoYW5nZT17KHZhbHVlcykgPT4ge1xuICAgICAgICBpZiAoZGlzYWJsZWQpIHJldHVybjtcbiAgICAgICAgY29uc3QgdmFsdWUgPSB2YWx1ZXNbMF0gPz8gbWVhc3VyZS52YWx1ZTtcbiAgICAgICAgaWYgKHJldmVhbEdyb3VwSWQpIHtcbiAgICAgICAgICB3b3JsZFJldmVhbEN1dG9mZlN0b3JlLnNldChyZXZlYWxHcm91cElkLCB2YWx1ZSk7XG4gICAgICAgICAgcmV0dXJuO1xuICAgICAgICB9XG4gICAgICAgIGRpc3BhdGNoVmFsdWUodmFsdWUpO1xuICAgICAgfX1cbiAgICAgIG9uVmFsdWVDb21taXQ9e1xuICAgICAgICByZXZlYWxHcm91cElkXG4gICAgICAgICAgPyAodmFsdWVzKSA9PiB7XG4gICAgICAgICAgICAgIGlmIChkaXNhYmxlZCkgcmV0dXJuO1xuICAgICAgICAgICAgICBjb25zdCB2YWx1ZSA9IHZhbHVlc1swXSA/PyBtZWFzdXJlLnZhbHVlO1xuICAgICAgICAgICAgICB3b3JsZFJldmVhbEN1dG9mZlN0b3JlLnNldChyZXZlYWxHcm91cElkLCB2YWx1ZSk7XG4gICAgICAgICAgICAgIG9uQWN0aW9uKHsgLi4ubWVhc3VyZS5vbkNoYW5nZSwgYXJnczogeyAuLi4obWVhc3VyZS5vbkNoYW5nZS5hcmdzIGFzIG9iamVjdCB8IHVuZGVmaW5lZCksIHZhbHVlIH0gfSk7XG4gICAgICAgICAgICB9XG4gICAgICAgICAgOiB1bmRlZmluZWRcbiAgICAgIH1cbiAgICAgIG9uUG9pbnRlckNhbmNlbD17cmV2ZWFsR3JvdXBJZCA/ICgpID0+IHdvcmxkUmV2ZWFsQ3V0b2ZmU3RvcmUuc2V0KHJldmVhbEdyb3VwSWQsIG1lYXN1cmUudmFsdWUpIDogdW5kZWZpbmVkfVxuICAgIC8+XG4gICk7XG59XG5cbmZ1bmN0aW9uIHdpbmRvd01lYXN1cmVHcm91cEhlYWRlclNsaWRlcihtZWFzdXJlOiBFeHRyYWN0PFdpbmRvd01lYXN1cmUsIHsga2luZDogXCJncm91cFwiIH0+LCBvbkFjdGlvbjogKGFjdGlvbjogQWN0aW9uRGVzY3JpcHRvcikgPT4gdW5rbm93bik6IFJlYWN0Tm9kZSB8IHVuZGVmaW5lZCB7XG4gIGlmIChtZWFzdXJlLnZhbHVlID09PSB1bmRlZmluZWQgfHwgbWVhc3VyZS5vbkNoYW5nZSA9PT0gdW5kZWZpbmVkKSByZXR1cm4gdW5kZWZpbmVkO1xuICBjb25zdCBzbGlkZXJNZWFzdXJlOiBFeHRyYWN0PFdpbmRvd01lYXN1cmUsIHsga2luZDogXCJzbGlkZXJcIiB9PiA9IHtcbiAgICBraW5kOiBcInNsaWRlclwiLFxuICAgIGlkOiBgJHttZWFzdXJlLmlkfS5oZWFkZXItc2xpZGVyYCxcbiAgICBsYWJlbDogdW5kZWZpbmVkLFxuICAgIHZhbHVlOiBtZWFzdXJlLnZhbHVlLFxuICAgIG1pbjogbWVhc3VyZS5taW4gPz8gMCxcbiAgICBtYXg6IG1lYXN1cmUubWF4ID8/IDEsXG4gICAgc3RlcDogbWVhc3VyZS5zdGVwLFxuICAgIHJlYWR5OiBtZWFzdXJlLnJlYWR5LFxuICAgIGxvYWRpbmc6IG1lYXN1cmUubG9hZGluZyxcbiAgICB3YWl0aW5nOiBtZWFzdXJlLndhaXRpbmcsXG4gICAgb25DaGFuZ2U6IG1lYXN1cmUub25DaGFuZ2UsXG4gIH07XG4gIHJldHVybiA8V2luZG93TWVhc3VyZVNsaWRlciBtZWFzdXJlPXtzbGlkZXJNZWFzdXJlfSBvbkFjdGlvbj17b25BY3Rpb259IC8+O1xufVxuXG5mdW5jdGlvbiB3aW5kb3dNZWFzdXJlU2VsZWN0Q29udHJvbChtZWFzdXJlOiBFeHRyYWN0PFdpbmRvd01lYXN1cmUsIHsga2luZDogXCJzZWxlY3RcIiB9Piwgb25BY3Rpb246IChhY3Rpb246IEFjdGlvbkRlc2NyaXB0b3IpID0+IHVua25vd24pOiBSZWFjdE5vZGUge1xuICByZXR1cm4gKFxuICAgIDxTZWxlY3QgdmFsdWU9e21lYXN1cmUudmFsdWV9IG9uVmFsdWVDaGFuZ2U9eyh2YWx1ZSkgPT4gb25BY3Rpb24oeyAuLi5tZWFzdXJlLm9uQ2hhbmdlLCBhcmdzOiB7IC4uLihtZWFzdXJlLm9uQ2hhbmdlLmFyZ3MgYXMgb2JqZWN0IHwgdW5kZWZpbmVkKSwgdmFsdWUgfSB9KX0+XG4gICAgICA8U2VsZWN0VHJpZ2dlciBpZD17bWVhc3VyZS5pZH0gY2xhc3NOYW1lPVwiaC1zbWFsbCB3LWZ1bGwgbWluLXctMFwiIHNpemU9XCJzbVwiPlxuICAgICAgICA8U2VsZWN0VmFsdWUgLz5cbiAgICAgIDwvU2VsZWN0VHJpZ2dlcj5cbiAgICAgIDxTZWxlY3RDb250ZW50PlxuICAgICAgICB7bWVhc3VyZS5pdGVtcy5tYXAoKGl0ZW0pID0+IChcbiAgICAgICAgICA8U2VsZWN0SXRlbSBrZXk9e2l0ZW0uaWR9IHZhbHVlPXtpdGVtLnZhbHVlfT5cbiAgICAgICAgICAgIHtpdGVtLmxhYmVsfVxuICAgICAgICAgIDwvU2VsZWN0SXRlbT5cbiAgICAgICAgKSl9XG4gICAgICA8L1NlbGVjdENvbnRlbnQ+XG4gICAgPC9TZWxlY3Q+XG4gICk7XG59XG5cbmZ1bmN0aW9uIHdpbmRvd01lYXN1cmVUb2dnbGVDb250cm9sKG1lYXN1cmU6IEV4dHJhY3Q8V2luZG93TWVhc3VyZSwgeyBraW5kOiBcInRvZ2dsZVwiIH0+LCBvbkFjdGlvbjogKGFjdGlvbjogQWN0aW9uRGVzY3JpcHRvcikgPT4gdW5rbm93bik6IFJlYWN0Tm9kZSB7XG4gIGNvbnN0IGxhYmVsID0gbWVhc3VyZS5sYWJlbCA/PyBtZWFzdXJlLnRleHQgPz8gbWVhc3VyZS5pZDtcbiAgcmV0dXJuIChcbiAgICA8VHJlZUNoZWNrYm94XG4gICAgICBpZD17bWVhc3VyZS5pZH1cbiAgICAgIGNoZWNrZWQ9e21lYXN1cmUucHJlc3NlZH1cbiAgICAgIHRpdGxlPXtsYWJlbH1cbiAgICAgIGFyaWFMYWJlbD17bGFiZWx9XG4gICAgICBvbkNoZWNrZWRDaGFuZ2U9eyhwcmVzc2VkKSA9PiBvbkFjdGlvbih7IC4uLm1lYXN1cmUub25DaGFuZ2UsIGFyZ3M6IHsgLi4uKG1lYXN1cmUub25DaGFuZ2UuYXJncyBhcyBvYmplY3QgfCB1bmRlZmluZWQpLCBwcmVzc2VkIH0gfSl9XG4gICAgLz5cbiAgKTtcbn1cblxuZnVuY3Rpb24gd2luZG93TWVhc3VyZVRvZ2dsZUljb24obWVhc3VyZTogRXh0cmFjdDxXaW5kb3dNZWFzdXJlLCB7IGtpbmQ6IFwidG9nZ2xlXCIgfT4pOiBSZWFjdE5vZGUge1xuICByZXR1cm4gPEljb24gaWNvbj17bWVhc3VyZS5pY29uSWQgYXMgSWNvbk5hbWV9IHNpemU9ezEyfSAvPjtcbn1cblxuLyoqXG4gKiDwn4yy77iPIE1hcHMgd2luZG93IG1lYXN1cmVzIHRvIG5hdGl2ZSBwYW5lbC10cmVlIHJvd3Mg4oCUIHNhbWUgY2hyb21lIGFzIGxlZnQtY29ybmVyIHRyZWVzIChsYWJlbCBsZWZ0LCBjb250cm9sIHJpZ2h0LCBndWlkZSBsaW5lcykuXG4gKiBQcmUtcmV2ZXJzZXMgdG9wLWxldmVsIG1lYXN1cmVzIHNvIGJvdHRvbS1hbmNob3JlZCBgZGlyZWN0aW9uPVwidXBcImAgcGFuZWxzIHJlYWQgQ291bnQgYXQgdGhlIGJvdHRvbSwgRGlzdHJpYnV0aW9uIGFib3ZlLlxuICovXG5mdW5jdGlvbiB3aW5kb3dNZWFzdXJlc1RvVHJlZUl0ZW1zKG1lYXN1cmVzOiByZWFkb25seSBXaW5kb3dNZWFzdXJlW10sIG9uQWN0aW9uOiAoYWN0aW9uOiBBY3Rpb25EZXNjcmlwdG9yKSA9PiB1bmtub3duLCByZXZlcnNlRm9yVXBQYW5lbCA9IHRydWUpOiBUcmVlRGF0YUl0ZW1bXSB7XG4gIGNvbnN0IG9yZGVyZWQgPSByZXZlcnNlRm9yVXBQYW5lbCA/IFsuLi5tZWFzdXJlc10ucmV2ZXJzZSgpIDogWy4uLm1lYXN1cmVzXTtcbiAgY29uc3QgbWFwTWVhc3VyZSA9IChtZWFzdXJlOiBXaW5kb3dNZWFzdXJlKTogVHJlZURhdGFJdGVtID0+IHtcbiAgICBpZiAobWVhc3VyZS5raW5kID09PSBcImdyb3VwXCIpIHtcbiAgICAgIHJldHVybiB7XG4gICAgICAgIGlkOiBtZWFzdXJlLmlkLFxuICAgICAgICBsYWJlbDogbWVhc3VyZS5sYWJlbCxcbiAgICAgICAgZGVmYXVsdE9wZW46IG1lYXN1cmUuZGVmYXVsdE9wZW4sXG4gICAgICAgIGNvbnRyb2w6IHdpbmRvd01lYXN1cmVHcm91cEhlYWRlclNsaWRlcihtZWFzdXJlLCBvbkFjdGlvbiksXG4gICAgICAgIGl0ZW1zOiBtZWFzdXJlLmNoaWxkcmVuLmxlbmd0aCA+IDAgPyB3aW5kb3dNZWFzdXJlc1RvVHJlZUl0ZW1zKG1lYXN1cmUuY2hpbGRyZW4sIG9uQWN0aW9uLCBmYWxzZSkgOiB1bmRlZmluZWQsXG4gICAgICB9O1xuICAgIH1cbiAgICBpZiAobWVhc3VyZS5raW5kID09PSBcInNsaWRlclwiKSB7XG4gICAgICByZXR1cm4ge1xuICAgICAgICBpZDogbWVhc3VyZS5pZCxcbiAgICAgICAgbGFiZWw6IG1lYXN1cmUubGFiZWwgPz8gXCJcIixcbiAgICAgICAgY29udHJvbDogPFdpbmRvd01lYXN1cmVTbGlkZXIgbWVhc3VyZT17bWVhc3VyZX0gb25BY3Rpb249e29uQWN0aW9ufSAvPixcbiAgICAgICAgbG9hZGluZzogbWVhc3VyZS5sb2FkaW5nLFxuICAgICAgICB3YWl0aW5nOiBtZWFzdXJlLndhaXRpbmcsXG4gICAgICB9O1xuICAgIH1cbiAgICBpZiAobWVhc3VyZS5raW5kID09PSBcInNlbGVjdFwiKSB7XG4gICAgICByZXR1cm4ge1xuICAgICAgICBpZDogbWVhc3VyZS5pZCxcbiAgICAgICAgbGFiZWw6IG1lYXN1cmUubGFiZWwgPz8gXCJcIixcbiAgICAgICAgY29udHJvbDogd2luZG93TWVhc3VyZVNlbGVjdENvbnRyb2wobWVhc3VyZSwgb25BY3Rpb24pLFxuICAgICAgfTtcbiAgICB9XG4gICAgcmV0dXJuIHtcbiAgICAgIGlkOiBtZWFzdXJlLmlkLFxuICAgICAgbGFiZWw6IG1lYXN1cmUubGFiZWwgPz8gbWVhc3VyZS50ZXh0ID8/IFwiXCIsXG4gICAgICBpY29uOiB3aW5kb3dNZWFzdXJlVG9nZ2xlSWNvbihtZWFzdXJlKSxcbiAgICAgIGNvbnRyb2w6IHdpbmRvd01lYXN1cmVUb2dnbGVDb250cm9sKG1lYXN1cmUsIG9uQWN0aW9uKSxcbiAgICB9O1xuICB9O1xuICByZXR1cm4gb3JkZXJlZC5tYXAobWFwTWVhc3VyZSk7XG59XG5cbmZ1bmN0aW9uIHJlbmRlcldpbmRvd01lYXN1cmUobWVhc3VyZTogV2luZG93TWVhc3VyZSwgb25BY3Rpb246IChhY3Rpb246IEFjdGlvbkRlc2NyaXB0b3IpID0+IHVua25vd24pOiBSZWFjdE5vZGUge1xuICBpZiAobWVhc3VyZS5raW5kID09PSBcImdyb3VwXCIpIHtcbiAgICBjb25zdCBoZWFkZXJTbGlkZXIgPSB3aW5kb3dNZWFzdXJlR3JvdXBIZWFkZXJTbGlkZXIobWVhc3VyZSwgb25BY3Rpb24pO1xuICAgIHJldHVybiAoXG4gICAgICA8V2luZG93TWVhc3VyZVRyZWVHcm91cCBrZXk9e21lYXN1cmUuaWR9IGlkPXttZWFzdXJlLmlkfSBsYWJlbD17bWVhc3VyZS5sYWJlbH0gZGVmYXVsdE9wZW49e21lYXN1cmUuZGVmYXVsdE9wZW59IGhlYWRlckNvbnRyb2w9e2hlYWRlclNsaWRlcn0+XG4gICAgICAgIHttZWFzdXJlLmNoaWxkcmVuLm1hcCgoY2hpbGQpID0+IHJlbmRlcldpbmRvd01lYXN1cmUoY2hpbGQsIG9uQWN0aW9uKSl9XG4gICAgICA8L1dpbmRvd01lYXN1cmVUcmVlR3JvdXA+XG4gICAgKTtcbiAgfVxuICBpZiAobWVhc3VyZS5raW5kID09PSBcInNlbGVjdFwiKSB7XG4gICAgcmV0dXJuIChcbiAgICAgIDxXaW5kb3dNZWFzdXJlVHJlZUxlYWYga2V5PXttZWFzdXJlLmlkfSBsYWJlbD17bWVhc3VyZS5sYWJlbH0+XG4gICAgICAgIHt3aW5kb3dNZWFzdXJlU2VsZWN0Q29udHJvbChtZWFzdXJlLCBvbkFjdGlvbil9XG4gICAgICA8L1dpbmRvd01lYXN1cmVUcmVlTGVhZj5cbiAgICApO1xuICB9XG4gIGlmIChtZWFzdXJlLmtpbmQgPT09IFwic2xpZGVyXCIpIHtcbiAgICByZXR1cm4gKFxuICAgICAgPFdpbmRvd01lYXN1cmVUcmVlTGVhZiBrZXk9e21lYXN1cmUuaWR9IGxhYmVsPXttZWFzdXJlLmxhYmVsfT5cbiAgICAgICAgPFdpbmRvd01lYXN1cmVTbGlkZXIgbWVhc3VyZT17bWVhc3VyZX0gb25BY3Rpb249e29uQWN0aW9ufSAvPlxuICAgICAgPC9XaW5kb3dNZWFzdXJlVHJlZUxlYWY+XG4gICAgKTtcbiAgfVxuICBpZiAobWVhc3VyZS5raW5kID09PSBcInRvZ2dsZVwiKSB7XG4gICAgcmV0dXJuIChcbiAgICAgIDxXaW5kb3dNZWFzdXJlVHJlZUxlYWYga2V5PXttZWFzdXJlLmlkfSBsYWJlbD17bWVhc3VyZS5sYWJlbCA/PyBtZWFzdXJlLnRleHR9IGljb249e3dpbmRvd01lYXN1cmVUb2dnbGVJY29uKG1lYXN1cmUpfT5cbiAgICAgICAge3dpbmRvd01lYXN1cmVUb2dnbGVDb250cm9sKG1lYXN1cmUsIG9uQWN0aW9uKX1cbiAgICAgIDwvV2luZG93TWVhc3VyZVRyZWVMZWFmPlxuICAgICk7XG4gIH1cbiAgcmV0dXJuIG51bGw7XG59XG5cbmZ1bmN0aW9uIHdpbmRvd01lYXN1cmVzT3ZlcmxheShtZWFzdXJlczogcmVhZG9ubHkgV2luZG93TWVhc3VyZVtdIHwgdW5kZWZpbmVkLCBvbkFjdGlvbjogKGFjdGlvbjogQWN0aW9uRGVzY3JpcHRvcikgPT4gdW5rbm93biwgZGlyZWN0aW9uOiBcInVwXCIgfCBcImRvd25cIiA9IFwiZG93blwiKTogUmVhY3ROb2RlIHwgdW5kZWZpbmVkIHtcbiAgaWYgKCFtZWFzdXJlcyB8fCBtZWFzdXJlcy5sZW5ndGggPT09IDApIHJldHVybiB1bmRlZmluZWQ7XG4gIHJldHVybiA8V2luZG93TWVhc3VyZXNUcmVlIGRpcmVjdGlvbj17ZGlyZWN0aW9ufT57bWVhc3VyZXMubWFwKChtZWFzdXJlKSA9PiByZW5kZXJXaW5kb3dNZWFzdXJlKG1lYXN1cmUsIG9uQWN0aW9uKSl9PC9XaW5kb3dNZWFzdXJlc1RyZWU+O1xufVxuXG4vKiogQGVtb2ppIPCfqp/vuI8gUHVibGljIHdpbmRvdy1vcHRpb25zIHRyZWUgZm9yIG1lYXN1cmVzIHJhaWxzIGFuZCB0ZXN0cyDigJQgaWNvbiBiZWZvcmUgbGFiZWwsIGNoZWNrYm94IGZvciB0b2dnbGVzLiAqL1xuZXhwb3J0IGZ1bmN0aW9uIHJlbmRlcldpbmRvd01lYXN1cmVzVHJlZShtZWFzdXJlczogcmVhZG9ubHkgV2luZG93TWVhc3VyZVtdLCBvbkFjdGlvbjogKGFjdGlvbjogQWN0aW9uRGVzY3JpcHRvcikgPT4gdW5rbm93biwgZGlyZWN0aW9uOiBcInVwXCIgfCBcImRvd25cIiA9IFwiZG93blwiKTogUmVhY3ROb2RlIHwgdW5kZWZpbmVkIHtcbiAgcmV0dXJuIHdpbmRvd01lYXN1cmVzT3ZlcmxheShtZWFzdXJlcywgb25BY3Rpb24sIGRpcmVjdGlvbik7XG59XG5cbmV4cG9ydCBmdW5jdGlvbiBTZWxlY3Rpb25VdGlsaXR5T3B0aW9ucyh7IGFjdGl2ZVV0aWxpdHlJZCwgd2luZG93SWQsIG9uQWN0aW9uIH06IHsgcmVhZG9ubHkgYWN0aXZlVXRpbGl0eUlkOiBzdHJpbmcgfCB1bmRlZmluZWQ7IHJlYWRvbmx5IHdpbmRvd0lkOiBzdHJpbmc7IHJlYWRvbmx5IG9uQWN0aW9uOiAoYWN0aW9uOiBBY3Rpb25EZXNjcmlwdG9yKSA9PiB2b2lkIH0pIHtcbiAgY29uc3QgbWV0aG9kTGFiZWwgPSB1c2VMYWJlbChcInVpLnNlbGVjdGlvbi5tZXRob2RcIik7XG4gIGNvbnN0IG1vZGVMYWJlbCA9IHVzZUxhYmVsKFwidWkuc2VsZWN0aW9uLm1vZGVcIik7XG4gIGNvbnN0IHJlY3RhbmdsZUxhYmVsID0gdXNlTGFiZWwoXCJ1aS5zZWxlY3Rpb24ucmVjdGFuZ2xlXCIpO1xuICBjb25zdCBsYXNzb0xhYmVsID0gdXNlTGFiZWwoXCJ1aS5zZWxlY3Rpb24ubGFzc29cIik7XG4gIGNvbnN0IHNlbGVjdGl2ZUxhYmVsID0gdXNlTGFiZWwoXCJ1aS5zZWxlY3Rpb24uc2VsZWN0aXZlXCIpO1xuICBjb25zdCBhZGRpdGl2ZUxhYmVsID0gdXNlTGFiZWwoXCJ1aS5zZWxlY3Rpb24uYWRkaXRpdmVcIik7XG4gIGNvbnN0IHN1YnRyYWN0aXZlTGFiZWwgPSB1c2VMYWJlbChcInVpLnNlbGVjdGlvbi5zdWJ0cmFjdGl2ZVwiKTtcbiAgY29uc3QgaW52ZXJ0aXZlTGFiZWwgPSB1c2VMYWJlbChcInVpLnNlbGVjdGlvbi5pbnZlcnRpdmVcIik7XG4gIGNvbnN0IHNlbGVjdGlvbk1ldGhvZCA9IGFjdGl2ZVV0aWxpdHlJZCA9PT0gXCJzZWxlY3RMYXNzb1wiID8gXCJsYXNzb1wiIDogXCJyZWN0YW5nbGVcIjtcbiAgLy8g8J+Qmu+4jyBSZXBsYWNlcyB0aGUgb2xkIGAoZ2xvYmFsVGhpcykuX19zZWxlY3Rpb25Nb2RlYCArIGB3aW5kb3dgIGBcInNlbWlvOnNlbGVjdGlvbk9wdGlvbnNDaGFuZ2VkXCJgXG4gIC8vIGJyb2FkY2FzdCDigJQgdGhpcyBzaGVsbCdzIG93biBzdG9yZSwgc28gaXRzIHNlbGVjdGlvbi1tb2RlIHRvZ2dsZSBuZXZlciByZWNvbmZpZ3VyZXMgYW5vdGhlciBtb3VudGVkXG4gIC8vIHNoZWxsJ3MgbWFycXVlZSBnZXN0dXJlcy5cbiAgY29uc3Qgc2VsZWN0aW9uU3RvcmUgPSB1c2VTaGVsbFNjb3BlKCkuc2VsZWN0aW9uO1xuXG4gIGNvbnN0IFtzZWxlY3Rpb25Nb2RlLCBzZXRTZWxlY3Rpb25Nb2RlXSA9IHVzZVN0YXRlPFNlbGVjdGlvbk1lcmdlTW9kZT4oKCkgPT4gc2VsZWN0aW9uU3RvcmUuZ2V0KCkpO1xuXG4gIGNvbnN0IGhhbmRsZU1vZGVDaGFuZ2UgPSAobW9kZTogU2VsZWN0aW9uTWVyZ2VNb2RlKSA9PiB7XG4gICAgc2VsZWN0aW9uU3RvcmUuc2V0KG1vZGUpO1xuICAgIHNldFNlbGVjdGlvbk1vZGUobW9kZSk7XG4gIH07XG5cbiAgY29uc3QgaGFuZGxlTWV0aG9kQ2hhbmdlID0gKG1ldGhvZDogXCJyZWN0YW5nbGVcIiB8IFwibGFzc29cIikgPT4ge1xuICAgIG9uQWN0aW9uKHtcbiAgICAgIGNvbnRyb2xsZXJJZDogXCJ3aW5kb3dcIixcbiAgICAgIGFjdGlvbjogU0VUX0FDVElWRV9VVElMSVRZX0FDVElPTl9JRCxcbiAgICAgIGFyZ3M6IHsgd2luZG93SWQsIHV0aWxpdHlJZDogbWV0aG9kID09PSBcImxhc3NvXCIgPyBcInNlbGVjdExhc3NvXCIgOiBcInNlbGVjdE1hcnF1ZWVcIiB9LFxuICAgIH0pO1xuICB9O1xuXG4gIHJldHVybiAoXG4gICAgPGRpdiBjbGFzc05hbWU9XCJmbGV4IGl0ZW1zLWNlbnRlciBnYXAtZG91YmxlXCI+XG4gICAgICA8ZGl2IGNsYXNzTmFtZT1cImZsZXggaXRlbXMtY2VudGVyIGdhcC1zaW5nbGVcIj5cbiAgICAgICAgPHNwYW4gY2xhc3NOYW1lPVwidGV4dC10aW55IHRleHQtbXV0ZWQtZm9yZWdyb3VuZCB1cHBlcmNhc2UgdHJhY2tpbmctd2lkZXIgZm9udC1zZW1pYm9sZFwiPnttZXRob2RMYWJlbH08L3NwYW4+XG4gICAgICAgIDxUb2dnbGVHcm91cFxuICAgICAgICAgIGtpbmQ9XCJzaW5nbGVcIlxuICAgICAgICAgIHZhbHVlPXtzZWxlY3Rpb25NZXRob2R9XG4gICAgICAgICAgb25WYWx1ZUNoYW5nZT17KHZhbCkgPT4ge1xuICAgICAgICAgICAgaWYgKHZhbCA9PT0gXCJyZWN0YW5nbGVcIiB8fCB2YWwgPT09IFwibGFzc29cIikge1xuICAgICAgICAgICAgICBoYW5kbGVNZXRob2RDaGFuZ2UodmFsKTtcbiAgICAgICAgICAgIH1cbiAgICAgICAgICB9fVxuICAgICAgICAgIGl0ZW1zPXtbXG4gICAgICAgICAgICB7IHZhbHVlOiBcInJlY3RhbmdsZVwiLCBpY29uOiA8SWNvbiBpY29uPVwic3F1YXJlLWRhc2hlZFwiIHNpemU9XCJzbWFsbFwiIC8+LCB0ZXh0OiByZWN0YW5nbGVMYWJlbCB9LFxuICAgICAgICAgICAgeyB2YWx1ZTogXCJsYXNzb1wiLCBpY29uOiA8SWNvbiBpY29uPVwibGFzc29cIiBzaXplPVwic21hbGxcIiAvPiwgdGV4dDogbGFzc29MYWJlbCB9LFxuICAgICAgICAgIF19XG4gICAgICAgIC8+XG4gICAgICA8L2Rpdj5cbiAgICAgIDxSaWJib25EaXZpZGVyIC8+XG4gICAgICA8ZGl2IGNsYXNzTmFtZT1cImZsZXggaXRlbXMtY2VudGVyIGdhcC1zaW5nbGVcIj5cbiAgICAgICAgPHNwYW4gY2xhc3NOYW1lPVwidGV4dC10aW55IHRleHQtbXV0ZWQtZm9yZWdyb3VuZCB1cHBlcmNhc2UgdHJhY2tpbmctd2lkZXIgZm9udC1zZW1pYm9sZFwiPnttb2RlTGFiZWx9PC9zcGFuPlxuICAgICAgICA8VG9nZ2xlR3JvdXBcbiAgICAgICAgICBraW5kPVwic2luZ2xlXCJcbiAgICAgICAgICB2YWx1ZT17c2VsZWN0aW9uTW9kZX1cbiAgICAgICAgICBvblZhbHVlQ2hhbmdlPXsodmFsKSA9PiB7XG4gICAgICAgICAgICBpZiAodmFsID09PSBcImRlZmF1bHRcIiB8fCB2YWwgPT09IFwiYWRkaXRpdmVcIiB8fCB2YWwgPT09IFwic3VidHJhY3RpdmVcIiB8fCB2YWwgPT09IFwiaW52ZXJ0aXZlXCIpIHtcbiAgICAgICAgICAgICAgaGFuZGxlTW9kZUNoYW5nZSh2YWwpO1xuICAgICAgICAgICAgfVxuICAgICAgICAgIH19XG4gICAgICAgICAgaXRlbXM9e1tcbiAgICAgICAgICAgIHsgdmFsdWU6IFwiZGVmYXVsdFwiLCB0ZXh0OiBzZWxlY3RpdmVMYWJlbCB9LFxuICAgICAgICAgICAgeyB2YWx1ZTogXCJhZGRpdGl2ZVwiLCB0ZXh0OiBhZGRpdGl2ZUxhYmVsIH0sXG4gICAgICAgICAgICB7IHZhbHVlOiBcInN1YnRyYWN0aXZlXCIsIHRleHQ6IHN1YnRyYWN0aXZlTGFiZWwgfSxcbiAgICAgICAgICAgIHsgdmFsdWU6IFwiaW52ZXJ0aXZlXCIsIHRleHQ6IGludmVydGl2ZUxhYmVsIH0sXG4gICAgICAgICAgXX1cbiAgICAgICAgLz5cbiAgICAgIDwvZGl2PlxuICAgIDwvZGl2PlxuICApO1xufVxuXG5leHBvcnQgZnVuY3Rpb24gd2luZG93TWVhc3VyZXNDaHJvbWUoXG4gIG1lYXN1cmVzOiByZWFkb25seSBXaW5kb3dNZWFzdXJlW10gfCB1bmRlZmluZWQsXG4gIGFjdGl2ZVV0aWxpdHlJZDogc3RyaW5nIHwgdW5kZWZpbmVkLFxuICB3aW5kb3dJZDogc3RyaW5nLFxuICBvbkFjdGlvbjogKGFjdGlvbjogQWN0aW9uRGVzY3JpcHRvcikgPT4gdW5rbm93bixcbik6IHsgcmVhZG9ubHkgbWVhc3VyZXM6IFJlYWN0Tm9kZSB8IHVuZGVmaW5lZDsgcmVhZG9ubHkgdXRpbGl0eU9wdGlvbnM6IFJlYWN0Tm9kZSB8IHVuZGVmaW5lZCB9IHtcbiAgY29uc3QgeyBnZW5lcmFsLCB1dGlsaXR5T3B0aW9ucyB9ID0gcGFydGl0aW9uV2luZG93TWVhc3VyZXMobWVhc3VyZXMgPz8gW10sIGFjdGl2ZVV0aWxpdHlJZCk7XG4gIC8vIPCfqp/vuI8gU3RhbXBzIHRoaXMgY2hyb21lJ3Mgb3duaW5nIGB3aW5kb3dJZGAgb250byBldmVyeSBtZWFzdXJlIGFjdGlvbiwgbWlycm9yaW5nIGB0YWdTZXRBY3RpdmVVdGlsaXR5V2luZG93YFxuICAvLyBmb3IgdGhlIHV0aWxpdHkgYmFyIOKAlCB0aGUgZ2VuZXJpYyBgb25BY3Rpb25gIGRpc3BhdGNoIHBhdGggcmVhZHMgaXQgYmFjayBvdXQgdG8gdGFyZ2V0IHRoZSBwbHVnaW4gY2FsbCdzXG4gIC8vIGB2aWV3X3N0YXRlLndpbmRvd0lkYCwgc28gYSBncmlkL0xPRC9zZWxlY3Rpb24gdG9nZ2xlIG9ubHkgZXZlciBtdXRhdGVzIElUUyBPV04gd2luZG93J3Mgb3B0aW9ucy5cbiAgY29uc3QgdGFnZ2VkT25BY3Rpb24gPSAoYWN0aW9uOiBBY3Rpb25EZXNjcmlwdG9yKSA9PiBvbkFjdGlvbih7IC4uLmFjdGlvbiwgYXJnczogeyAuLi4oYWN0aW9uLmFyZ3MgYXMgb2JqZWN0IHwgdW5kZWZpbmVkKSwgd2luZG93SWQgfSB9KTtcbiAgcmV0dXJuIHtcbiAgICBtZWFzdXJlczogd2luZG93TWVhc3VyZXNPdmVybGF5KGdlbmVyYWwsIHRhZ2dlZE9uQWN0aW9uKSxcbiAgICB1dGlsaXR5T3B0aW9uczogd2luZG93TWVhc3VyZXNPdmVybGF5KHV0aWxpdHlPcHRpb25zLCB0YWdnZWRPbkFjdGlvbiwgXCJ1cFwiKSxcbiAgfTtcbn1cblxuLyoqIEBlbW9qaSDwn46T77iPIFdoZXRoZXIgYSB1dGlsaXR5IG5vZGUgdHJlZSBoYXMgYSBub2RlIChsZWFmIG9yIGdyb3VwKSB3aXRoIHRoZSBnaXZlbiBpZCBhbnl3aGVyZSBpbiBpdCDigJQgdXNlZFxuICogdG8gZGVjaWRlIGlmIHRoaXMgd2luZG93J3MgdXRpbGl0eSBiYXIgaXMgdGhlIG9uZSBhbiBpbnRyb2R1Y3Rpb24gc3RlcCdzIGBVdGlsaXR5YCBhbmNob3IgdGFyZ2V0cy4gKi9cbmV4cG9ydCBmdW5jdGlvbiB1dGlsaXR5Tm9kZVRyZWVDb250YWluc0lkKG5vZGVzOiByZWFkb25seSBVdGlsaXR5Tm9kZVtdLCB0YXJnZXRJZDogc3RyaW5nKTogYm9vbGVhbiB7XG4gIHJldHVybiBub2Rlcy5zb21lKChub2RlKSA9PiBub2RlLmlkID09PSB0YXJnZXRJZCB8fCAobm9kZS5raW5kID09PSBcImNvbGxlY3Rpb25cIiAmJiB1dGlsaXR5Tm9kZVRyZWVDb250YWluc0lkKG5vZGUuY2hpbGRyZW4sIHRhcmdldElkKSkpO1xufVxuXG5leHBvcnQgZnVuY3Rpb24gdXRpbGl0eUJhck5vZGUodXRpbGl0aWVzOiByZWFkb25seSBVdGlsaXR5Tm9kZVtdIHwgdW5kZWZpbmVkLCB3aW5kb3dJZDogc3RyaW5nLCBvbkFjdGlvbjogKGFjdGlvbjogQWN0aW9uRGVzY3JpcHRvcikgPT4gdm9pZCwgcmV2ZWFsVXRpbGl0eUlkPzogc3RyaW5nIHwgbnVsbCwgdXRpbGl0eU9wdGlvbnM/OiBSZWFjdE5vZGUpOiBSZWFjdE5vZGUge1xuICBpZiAoIXV0aWxpdGllcz8ubGVuZ3RoICYmICF1dGlsaXR5T3B0aW9ucykgcmV0dXJuIHVuZGVmaW5lZDtcbiAgY29uc3QgY2F0ZWdvcmllcyA9IGdyb3VwVXRpbGl0eU5vZGVzQnlDYXRlZ29yeSh1dGlsaXRpZXMgPz8gW10sIFVUSUxJVFlfQ0FURUdPUklFUyk7XG4gIGlmICghY2F0ZWdvcmllcy5sZW5ndGggJiYgIXV0aWxpdHlPcHRpb25zKSByZXR1cm4gdW5kZWZpbmVkO1xuICBjb25zdCBncm91cGVkOiBVdGlsaXR5Tm9kZVtdID0gW107XG4gIGZvciAoY29uc3Qgbm9kZSBvZiBjYXRlZ29yaWVzKSB7XG4gICAgaWYgKG5vZGUua2luZCA9PT0gXCJjb2xsZWN0aW9uXCIgJiYgKG5vZGUuY2F0ZWdvcnkgPT09IFwidXRpbGl0aWVzXCIgfHwgbm9kZS5jYXRlZ29yeSA9PT0gXCJzZWxlY3Rpb25cIikpIHtcbiAgICAgIGlmIChub2RlLmlkID09PSBcImdyb3VwOlNlbGVjdFwiIHx8IG5vZGUuaWQgPT09IFwiZ3JvdXA6c2VsZWN0aW9uXCIgfHwgbm9kZS5sYWJlbCA9PT0gXCJTZWxlY3RcIiB8fCBub2RlLnRleHQgPT09IFwiU2VsZWN0XCIpIHtcbiAgICAgICAgZ3JvdXBlZC5wdXNoKC4uLm5vZGUuY2hpbGRyZW4pO1xuICAgICAgfSBlbHNlIHtcbiAgICAgICAgZm9yIChjb25zdCBjaGlsZCBvZiBub2RlLmNoaWxkcmVuKSB7XG4gICAgICAgICAgaWYgKGNoaWxkLmtpbmQgPT09IFwiY29sbGVjdGlvblwiICYmIChjaGlsZC5pZCA9PT0gXCJncm91cDpTZWxlY3RcIiB8fCBjaGlsZC5pZCA9PT0gXCJncm91cDpzZWxlY3Rpb25cIiB8fCBjaGlsZC5sYWJlbCA9PT0gXCJTZWxlY3RcIiB8fCBjaGlsZC50ZXh0ID09PSBcIlNlbGVjdFwiKSkge1xuICAgICAgICAgICAgZ3JvdXBlZC5wdXNoKC4uLmNoaWxkLmNoaWxkcmVuKTtcbiAgICAgICAgICB9IGVsc2Uge1xuICAgICAgICAgICAgZ3JvdXBlZC5wdXNoKGNoaWxkKTtcbiAgICAgICAgICB9XG4gICAgICAgIH1cbiAgICAgIH1cbiAgICB9IGVsc2Uge1xuICAgICAgZ3JvdXBlZC5wdXNoKG5vZGUpO1xuICAgIH1cbiAgfVxuICByZXR1cm4gPFV0aWxpdHlUcmVlIGlkPXtgdWkudXRpbGl0aWVzLiR7d2luZG93SWR9YH0gdXRpbGl0aWVzPXtncm91cGVkfSBvbkFjdGlvbj17b25BY3Rpb259IGRpcmVjdGlvbj1cInVwXCIgcmV2ZWFsVXRpbGl0eUlkPXtyZXZlYWxVdGlsaXR5SWR9IHV0aWxpdHlPcHRpb25zPXt1dGlsaXR5T3B0aW9uc30gLz47XG59XG5cbi8vI3JlZ2lvbiDwn6ew77iPV2luZG93QWN0aW9uUGFuZVxuLyoqXG4gKiDwn46b77iPIFJlbmRlcnMgb25lIHtAbGluayBBY3Rpb25BcmdDb250cm9sfSBpbnRvIGEgU1RBR0VEIGZvcm0gZmllbGQg4oCUIHRoZSBjcnVjaWFsIGRpZmZlcmVuY2UgZnJvbVxuICogYHJlbmRlclVpQ29udHJvbGAgaW4gYHVpLWludGVycHJldGVyLnRzeGAgaXMgdGhhdCB0aGlzIGRpc3BhdGNoZXMgTk9USElORyBnbG9iYWxseTsgYG9uQ2hhbmdlYCBvbmx5XG4gKiB3cml0ZXMgdG8gdGhlIGNhbGxlcidzIGxvY2FsIHN0YWdlZCBidWZmZXIuIGB2YWx1ZWAgaXMgdGhlIGFscmVhZHktcmVzb2x2ZWQgZWZmZWN0aXZlIHZhbHVlXG4gKiAoc3RhZ2VkID8/IGRlZmF1bHQgPz8gdW5zZXQpLlxuICovXG5leHBvcnQgZnVuY3Rpb24gcmVuZGVyU3RhZ2VkQXJnQ29udHJvbChkZWY6IEFjdGlvbkFyZ0RlZiwgdmFsdWU6IHVua25vd24sIG9uQ2hhbmdlOiAodmFsdWU6IHVua25vd24pID0+IHZvaWQsIGRpc2FibGVkPzogYm9vbGVhbik6IFJlYWN0RWxlbWVudCB7XG4gIGNvbnN0IGNvbnRyb2w6IEFjdGlvbkFyZ0NvbnRyb2wgPSBkZWYuY29udHJvbDtcbiAgc3dpdGNoIChjb250cm9sLmtpbmQpIHtcbiAgICBjYXNlIFwidGV4dFwiOlxuICAgICAgcmV0dXJuIDxJbnB1dCBpZD17ZGVmLmlkfSB0eXBlPVwidGV4dFwiIGNsYXNzTmFtZT1cImgtbWVkaXVtIHctZnVsbCBtaW4tdy0wXCIgdmFsdWU9e3R5cGVvZiB2YWx1ZSA9PT0gXCJzdHJpbmdcIiA/IHZhbHVlIDogXCJcIn0gcGxhY2Vob2xkZXI9e2NvbnRyb2wucGxhY2Vob2xkZXJ9IGRpc2FibGVkPXtkaXNhYmxlZH0gb25DaGFuZ2U9eyhldmVudCkgPT4gb25DaGFuZ2UoZXZlbnQudGFyZ2V0LnZhbHVlKX0gLz47XG4gICAgY2FzZSBcIm51bWJlclwiOlxuICAgICAgcmV0dXJuIChcbiAgICAgICAgPElucHV0XG4gICAgICAgICAgaWQ9e2RlZi5pZH1cbiAgICAgICAgICB0eXBlPVwibnVtYmVyXCJcbiAgICAgICAgICBjbGFzc05hbWU9XCJoLW1lZGl1bSB3LWZ1bGwgbWluLXctMFwiXG4gICAgICAgICAgdmFsdWU9e3ZhbHVlID09PSB1bmRlZmluZWQgfHwgdmFsdWUgPT09IG51bGwgfHwgdmFsdWUgPT09IFwiXCIgPyBcIlwiIDogU3RyaW5nKHZhbHVlKX1cbiAgICAgICAgICBtaW49e2NvbnRyb2wubWlufVxuICAgICAgICAgIG1heD17Y29udHJvbC5tYXh9XG4gICAgICAgICAgc3RlcD17Y29udHJvbC5zdGVwfVxuICAgICAgICAgIGRpc2FibGVkPXtkaXNhYmxlZH1cbiAgICAgICAgICBvbkNoYW5nZT17KGV2ZW50KSA9PiBvbkNoYW5nZShldmVudC50YXJnZXQudmFsdWUgPT09IFwiXCIgPyB1bmRlZmluZWQgOiBOdW1iZXIoZXZlbnQudGFyZ2V0LnZhbHVlKSl9XG4gICAgICAgIC8+XG4gICAgICApO1xuICAgIGNhc2UgXCJzbGlkZXJcIjoge1xuICAgICAgY29uc3QgbnVtZXJpYyA9IHR5cGVvZiB2YWx1ZSA9PT0gXCJudW1iZXJcIiAmJiBOdW1iZXIuaXNGaW5pdGUodmFsdWUpID8gdmFsdWUgOiBjb250cm9sLm1pbjtcbiAgICAgIGNvbnN0IHNsaWRlciA9IDxTbGlkZXIgaWQ9e2RlZi5pZH0gY2xhc3NOYW1lPVwidy1mdWxsIG1pbi13LTBcIiBtaW49e2NvbnRyb2wubWlufSBtYXg9e2NvbnRyb2wubWF4fSBzdGVwPXtjb250cm9sLnN0ZXAgPz8gMX0gdmFsdWU9e1tudW1lcmljXX0gZGlzYWJsZWQ9e2Rpc2FibGVkfSBvblZhbHVlQ2hhbmdlPXsodmFsdWVzKSA9PiBvbkNoYW5nZSh2YWx1ZXNbMF0gPz8gbnVtZXJpYyl9IC8+O1xuICAgICAgaWYgKCFjb250cm9sLnVuaXQpIHJldHVybiBzbGlkZXI7XG4gICAgICByZXR1cm4gKFxuICAgICAgICA8ZGl2IGNsYXNzTmFtZT1cImZsZXggdy1mdWxsIG1pbi13LTAgaXRlbXMtY2VudGVyIGdhcC1zaW5nbGVcIj5cbiAgICAgICAgICB7c2xpZGVyfVxuICAgICAgICAgIDxzcGFuIGNsYXNzTmFtZT1cInNocmluay0wIHRleHQteHMgdGFidWxhci1udW1zIHRleHQtbXV0ZWQtZm9yZWdyb3VuZFwiPlxuICAgICAgICAgICAge251bWVyaWN9IHtjb250cm9sLnVuaXR9XG4gICAgICAgICAgPC9zcGFuPlxuICAgICAgICA8L2Rpdj5cbiAgICAgICk7XG4gICAgfVxuICAgIGNhc2UgXCJ0b2dnbGVcIjpcbiAgICAgIHJldHVybiA8VG9nZ2xlIGlkPXtkZWYuaWR9IHByZXNzZWQ9e3ZhbHVlID09PSB0cnVlfSB0ZXh0PXtkZWYubGFiZWx9IGRpc2FibGVkPXtkaXNhYmxlZH0gb25QcmVzc2VkQ2hhbmdlPXsocHJlc3NlZCkgPT4gb25DaGFuZ2UocHJlc3NlZCl9IC8+O1xuICAgIGNhc2UgXCJzZWxlY3RcIjpcbiAgICAgIHJldHVybiAoXG4gICAgICAgIDxTZWxlY3QgdmFsdWU9e3R5cGVvZiB2YWx1ZSA9PT0gXCJzdHJpbmdcIiAmJiB2YWx1ZSA/IHZhbHVlIDogdW5kZWZpbmVkfSBkaXNhYmxlZD17ZGlzYWJsZWR9IG9uVmFsdWVDaGFuZ2U9eyhuZXh0KSA9PiBvbkNoYW5nZShuZXh0KX0+XG4gICAgICAgICAgPFNlbGVjdFRyaWdnZXIgaWQ9e2RlZi5pZH0gY2xhc3NOYW1lPVwiaC1tZWRpdW0gdy1mdWxsIG1pbi13LTBcIiBzaXplPVwic21cIj5cbiAgICAgICAgICAgIDxTZWxlY3RWYWx1ZSBwbGFjZWhvbGRlcj17ZGVmLmxhYmVsfSAvPlxuICAgICAgICAgIDwvU2VsZWN0VHJpZ2dlcj5cbiAgICAgICAgICA8U2VsZWN0Q29udGVudD5cbiAgICAgICAgICAgIHtjb250cm9sLm9wdGlvbnMubWFwKChvcHRpb24sIGluZGV4KSA9PiAoXG4gICAgICAgICAgICAgIDxTZWxlY3RJdGVtIGtleT17YCR7ZGVmLmlkfToke2luZGV4fToke29wdGlvbi52YWx1ZX1gfSB2YWx1ZT17b3B0aW9uLnZhbHVlfT5cbiAgICAgICAgICAgICAgICB7b3B0aW9uLmxhYmVsfVxuICAgICAgICAgICAgICA8L1NlbGVjdEl0ZW0+XG4gICAgICAgICAgICApKX1cbiAgICAgICAgICA8L1NlbGVjdENvbnRlbnQ+XG4gICAgICAgIDwvU2VsZWN0PlxuICAgICAgKTtcbiAgICBjYXNlIFwidmVjM1wiOiB7XG4gICAgICBjb25zdCB0dXBsZSA9IEFycmF5LmlzQXJyYXkodmFsdWUpICYmIHZhbHVlLmxlbmd0aCA+PSAzID8gKHZhbHVlIGFzIHJlYWRvbmx5IG51bWJlcltdKSA6IG51bGw7XG4gICAgICBjb25zdCBheGVzID0gW1wieFwiLCBcInlcIiwgXCJ6XCJdIGFzIGNvbnN0O1xuICAgICAgcmV0dXJuIChcbiAgICAgICAgPGRpdiBjbGFzc05hbWU9XCJncmlkIGdyaWQtY29scy0zIGdhcC1zaW5nbGVcIj5cbiAgICAgICAgICB7YXhlcy5tYXAoKGF4aXMsIGluZGV4KSA9PiAoXG4gICAgICAgICAgICA8SW5wdXRcbiAgICAgICAgICAgICAga2V5PXtgJHtkZWYuaWR9LiR7YXhpc31gfVxuICAgICAgICAgICAgICBpZD17YCR7ZGVmLmlkfS4ke2F4aXN9YH1cbiAgICAgICAgICAgICAgdHlwZT1cIm51bWJlclwiXG4gICAgICAgICAgICAgIGNsYXNzTmFtZT1cImgtbWVkaXVtIHctZnVsbCBtaW4tdy0wXCJcbiAgICAgICAgICAgICAgdmFsdWU9e3R1cGxlID8gU3RyaW5nKHR1cGxlW2luZGV4XSA/PyAwKSA6IFwiXCJ9XG4gICAgICAgICAgICAgIHBsYWNlaG9sZGVyPXtheGlzfVxuICAgICAgICAgICAgICBkaXNhYmxlZD17ZGlzYWJsZWR9XG4gICAgICAgICAgICAgIG9uQ2hhbmdlPXsoZXZlbnQpID0+IHtcbiAgICAgICAgICAgICAgICBjb25zdCBwYXJzZWQgPSBOdW1iZXIoZXZlbnQudGFyZ2V0LnZhbHVlKTtcbiAgICAgICAgICAgICAgICBpZiAoIU51bWJlci5pc0Zpbml0ZShwYXJzZWQpKSByZXR1cm47XG4gICAgICAgICAgICAgICAgY29uc3QgbmV4dDogW251bWJlciwgbnVtYmVyLCBudW1iZXJdID0gdHVwbGUgPyBbdHVwbGVbMF0gPz8gMCwgdHVwbGVbMV0gPz8gMCwgdHVwbGVbMl0gPz8gMF0gOiBbMCwgMCwgMF07XG4gICAgICAgICAgICAgICAgbmV4dFtpbmRleF0gPSBwYXJzZWQ7XG4gICAgICAgICAgICAgICAgb25DaGFuZ2UobmV4dCk7XG4gICAgICAgICAgICAgIH19XG4gICAgICAgICAgICAvPlxuICAgICAgICAgICkpfVxuICAgICAgICA8L2Rpdj5cbiAgICAgICk7XG4gICAgfVxuICAgIGNhc2UgXCJpY29uU2VsZWN0XCI6XG4gICAgICByZXR1cm4gPEljb25TZWxlY3RvciBpZD17ZGVmLmlkfSBjbGFzc2lmeUljb25TZWxlY3Rvck1vZGU9e3VuZGVmaW5lZH0gdmFsdWU9e3R5cGVvZiB2YWx1ZSA9PT0gXCJzdHJpbmdcIiA/IHZhbHVlIDogXCJcIn0gdW5pZm9ybSBvbkNoYW5nZT17KG5leHQpID0+IG9uQ2hhbmdlKG5leHQpfSAvPjtcbiAgfVxufVxuXG4vKiog8J+nsO+4jyBUcnVlIHdoZW4gYW4gYWN0aW9uIGNhcnJpZXMgYXJndW1lbnRzIGFuZCB0aGVyZWZvcmUgc3RhZ2VzIGEgZm9ybSBpbnN0ZWFkIG9mIGZpcmluZyBpbW1lZGlhdGVseSAoUDHigJNQNCkuICovXG5leHBvcnQgZnVuY3Rpb24gYWN0aW9uUmVxdWlyZXNTdGFnZWRGb3JtKGFjdGlvbjogUGljazxBY3Rpb25EZWZpbml0aW9uLCBcImFyZ3NcIj4pOiBib29sZWFuIHtcbiAgcmV0dXJuIChhY3Rpb24uYXJncz8ubGVuZ3RoID8/IDApID4gMDtcbn1cblxuLyoqIPCfp7DvuI8gVGhlIGRlY2lzaW9uIGEgYm91bmQgaG90a2V5IG1ha2VzIGZvciBvbmUgYWN0aW9uIChQNCkuICovXG4vKiog4oyo77iPIFRydWUgd2hlbiBhIGtleWRvd24ncyB0YXJnZXQgaXMgYSB0ZXh0LWVkaXRpbmcgc3VyZmFjZSAoaW5wdXQvdGV4dGFyZWEvc2VsZWN0L2NvbnRlbnRlZGl0YWJsZSkg4oCUIGhvdGtleXMgbmV2ZXIgZmlyZSB3aGlsZSB0aGUgdXNlciBpcyB0eXBpbmcuICovXG5leHBvcnQgZnVuY3Rpb24gaXNFZGl0YWJsZUV2ZW50VGFyZ2V0KHRhcmdldDogRXZlbnRUYXJnZXQgfCBudWxsKTogYm9vbGVhbiB7XG4gIGlmICghKHRhcmdldCBpbnN0YW5jZW9mIEhUTUxFbGVtZW50KSkgcmV0dXJuIGZhbHNlO1xuICBjb25zdCB0YWcgPSB0YXJnZXQudGFnTmFtZTtcbiAgaWYgKHRhZyA9PT0gXCJJTlBVVFwiIHx8IHRhZyA9PT0gXCJURVhUQVJFQVwiIHx8IHRhZyA9PT0gXCJTRUxFQ1RcIikgcmV0dXJuIHRydWU7XG4gIGlmICh0YXJnZXQuaXNDb250ZW50RWRpdGFibGUpIHJldHVybiB0cnVlO1xuICByZXR1cm4gdGFyZ2V0LmNsb3Nlc3QoXCJbY29udGVudGVkaXRhYmxlPSd0cnVlJ10sIFtyb2xlPSd0ZXh0Ym94J11cIikgIT0gbnVsbDtcbn1cblxuLyoqIOKMqO+4jyBUcnVlIHdoZW4gYSBrZXlkb3duIGV2ZW50IG1hdGNoZXMgb25lIGArYC1qb2luZWQgY2hvcmQgKGUuZy4gYFwibW9kK3NoaWZ0K3pcImApLCB3aGVyZSBgbW9kYCBhY2NlcHRzIGVpdGhlciBjdHJsIG9yIG1ldGEuICovXG5leHBvcnQgZnVuY3Rpb24ga2V5Ym9hcmRFdmVudE1hdGNoZXNDaG9yZChldmVudDogS2V5Ym9hcmRFdmVudCwgY2hvcmQ6IHN0cmluZyk6IGJvb2xlYW4ge1xuICBjb25zdCBwYXJ0cyA9IGNob3JkLnNwbGl0KFwiK1wiKS5tYXAoKHBhcnQpID0+IHBhcnQudHJpbSgpKTtcbiAgY29uc3Qga2V5ID0gcGFydHNbcGFydHMubGVuZ3RoIC0gMV0gPz8gXCJcIjtcbiAgY29uc3QgbmVlZHNDdHJsID0gcGFydHMuaW5jbHVkZXMoXCJjdHJsXCIpIHx8IHBhcnRzLmluY2x1ZGVzKFwibWV0YVwiKSB8fCBwYXJ0cy5pbmNsdWRlcyhcIm1vZFwiKTtcbiAgY29uc3QgbmVlZHNTaGlmdCA9IHBhcnRzLmluY2x1ZGVzKFwic2hpZnRcIik7XG4gIGNvbnN0IG5lZWRzQWx0ID0gcGFydHMuaW5jbHVkZXMoXCJhbHRcIik7XG4gIGNvbnN0IGhhc0N0cmwgPSBldmVudC5jdHJsS2V5IHx8IGV2ZW50Lm1ldGFLZXk7XG4gIGlmIChuZWVkc0N0cmwgIT09IGhhc0N0cmwpIHJldHVybiBmYWxzZTtcbiAgaWYgKG5lZWRzU2hpZnQgIT09IGV2ZW50LnNoaWZ0S2V5KSByZXR1cm4gZmFsc2U7XG4gIGlmIChuZWVkc0FsdCAhPT0gZXZlbnQuYWx0S2V5KSByZXR1cm4gZmFsc2U7XG4gIHJldHVybiBldmVudC5rZXkudG9Mb3dlckNhc2UoKSA9PT0ga2V5O1xufVxuXG5leHBvcnQgdHlwZSBLZXliaW5kaW5nSW50ZW50ID0geyByZWFkb25seSBraW5kOiBcImZpcmVcIiB9IHwgeyByZWFkb25seSBraW5kOiBcIm9wZW5cIjsgcmVhZG9ubHkgYWN0aW9uSWQ6IHN0cmluZyB9IHwgeyByZWFkb25seSBraW5kOiBcImV4ZWN1dGVcIjsgcmVhZG9ubHkgYWN0aW9uSWQ6IHN0cmluZzsgcmVhZG9ubHkgYXJnczogUmVjb3JkPHN0cmluZywgdW5rbm93bj4gfTtcblxuLyoqXG4gKiDinI3vuI8gUHVyZSBQNCBkZWNpc2lvbjogYW4gYXJnLWxlc3MgYWN0aW9uIGZpcmVzIGRpcmVjdGx5OyBhbiBhcmctY2FycnlpbmcgYWN0aW9uIG9wZW5zIGl0cyBzdGFnZWQgZm9ybSxcbiAqIHVubGVzcyB0aGF0IGZvcm0gaXMgYWxyZWFkeSB0aGUgZXhwYW5kZWQgb25lIGluIHRoZSBhY3RpdmUgd2luZG93IEFORCB2YWxpZGF0aW9uIHBhc3NlcywgaW4gd2hpY2ggY2FzZVxuICogdGhlIGhvdGtleSBleGVjdXRlcyB3aXRoIHRoZSBtZXJnZWQgZWZmZWN0aXZlIGFyZ3MuIEFuIGFscmVhZHktb3Blbi1idXQtaW52YWxpZCBmb3JtIHN0YXlzIG9wZW4uXG4gKi9cbmV4cG9ydCBmdW5jdGlvbiByZXNvbHZlS2V5YmluZGluZ0ludGVudChkZWZpbml0aW9uOiBBY3Rpb25EZWZpbml0aW9uIHwgdW5kZWZpbmVkLCBleHBhbmRlZEFjdGlvbklkOiBzdHJpbmcgfCBudWxsLCBzdGFnZWRBcmdzOiBSZWFkb25seTxSZWNvcmQ8c3RyaW5nLCB1bmtub3duPj4pOiBLZXliaW5kaW5nSW50ZW50IHtcbiAgaWYgKCFkZWZpbml0aW9uIHx8ICFhY3Rpb25SZXF1aXJlc1N0YWdlZEZvcm0oZGVmaW5pdGlvbikpIHJldHVybiB7IGtpbmQ6IFwiZmlyZVwiIH07XG4gIGlmIChleHBhbmRlZEFjdGlvbklkID09PSBkZWZpbml0aW9uLmlkKSB7XG4gICAgY29uc3QgZWZmZWN0aXZlID0gZWZmZWN0aXZlQWN0aW9uQXJncyhkZWZpbml0aW9uLmFyZ3MsIHN0YWdlZEFyZ3MpO1xuICAgIGlmIChtaXNzaW5nUmVxdWlyZWRBcmdzKGRlZmluaXRpb24uYXJncywgZWZmZWN0aXZlKS5sZW5ndGggPT09IDApIHJldHVybiB7IGtpbmQ6IFwiZXhlY3V0ZVwiLCBhY3Rpb25JZDogZGVmaW5pdGlvbi5pZCwgYXJnczogZWZmZWN0aXZlIH07XG4gIH1cbiAgcmV0dXJuIHsga2luZDogXCJvcGVuXCIsIGFjdGlvbklkOiBkZWZpbml0aW9uLmlkIH07XG59XG5cbi8qKiDwn6ew77iPIFB1cmUgUDUgYWN0aXZhdGlvbiBkZWNpc2lvbjogYW4gZW1wdHkgcmVxdWVzdCwgb3IgcmUtcmVxdWVzdGluZyB0aGUgYWxyZWFkeS1hY3RpdmUgdXRpbGl0eSwgZGVhY3RpdmF0ZXMgKG51bGwpOyBvdGhlcndpc2UgdGhlIHJlcXVlc3RlZCB1dGlsaXR5IGJlY29tZXMgYWN0aXZlLiAqL1xuZXhwb3J0IGZ1bmN0aW9uIHJlc29sdmVVdGlsaXR5QWN0aXZhdGlvbihjdXJyZW50OiBzdHJpbmcgfCBudWxsIHwgdW5kZWZpbmVkLCByZXF1ZXN0ZWQ6IHN0cmluZyk6IHN0cmluZyB8IG51bGwge1xuICByZXR1cm4gcmVxdWVzdGVkID09PSBcIlwiIHx8IChjdXJyZW50ID8/IG51bGwpID09PSByZXF1ZXN0ZWQgPyBudWxsIDogcmVxdWVzdGVkO1xufVxuXG4vKiog8J+Xgu+4jyBDYXRlZ29yeSBpZCBmb3Igb25lIGFjdGlvbjogZGVjbGFyZWQgY2F0ZWdvcnksIGVsc2UgYFwiaGlzdG9yeVwiYCBmb3IgaGlzdG9yeSBhY3Rpb25zLCBlbHNlIGBcImFjdGlvbnNcImAgKG1pcnJvcnMgdGhlIGNvbW1hbmQtcGFsZXR0ZSBmYWxsYmFjayBhdCB7QGxpbmsgcmVzb2x2ZUNvbW1hbmRzfSdzIHNpYmxpbmcgYHNlYXJjaEl0ZW1zYCBidWlsZGVyKS4gKi9cbmV4cG9ydCBmdW5jdGlvbiBhY3Rpb25DYXRlZ29yeUlkKGFjdGlvbjogUGljazxBY3Rpb25EZWZpbml0aW9uLCBcImNhdGVnb3J5XCIgfCBcImtpbmRcIj4pOiBzdHJpbmcge1xuICByZXR1cm4gYWN0aW9uLmNhdGVnb3J5ID8/IChhY3Rpb24ua2luZCA9PT0gXCJoaXN0b3J5XCIgPyBcImhpc3RvcnlcIiA6IFwiYWN0aW9uc1wiKTtcbn1cblxuLyoqIPCfl4LvuI8gUmVzb2x2ZXMgYW4gYWN0aW9uIGNhdGVnb3J5J3MgZGlzcGxheSBsYWJlbDogdGhlIGFwcCdzIG93biBncm91cC1sYWJlbCBvdmVybGF5IGZpcnN0LCB0aGVuIHRoZSBzaGFyZWQgYHVpLnJpYmJvbi5wYXJlbnQuKmAgY2hyb21lIHZvY2FidWxhcnkgZm9yIGtub3duIGNhdGVnb3J5IGlkcywgZWxzZSB0aGUgcmF3IGlkIChtaXJyb3JzIHtAbGluayByZXNvbHZlVXRpbGl0eUdyb3VwTGFiZWx9KS4gKi9cbmZ1bmN0aW9uIGFjdGlvbkNhdGVnb3J5TGFiZWwoY2F0ZWdvcnk6IHN0cmluZywgYXBwTGFiZWxzT3ZlcmxheTogUGx1Z2luQXBwTGFiZWxzT3ZlcmxheSk6IHN0cmluZyB7XG4gIGNvbnN0IGZhbGxiYWNrID0gQ0hST01FX0tOT1dOX1JJQkJPTl9QQVJFTlRfQ0FURUdPUklFUy5oYXMoY2F0ZWdvcnkpID8gc2hlbGxMYWJlbChgdWkucmliYm9uLnBhcmVudC4ke2NhdGVnb3J5IGFzIFVpUmliYm9uUGFyZW50Q2F0ZWdvcnl9YCkgOiBjYXRlZ29yeTtcbiAgcmV0dXJuIHJlc29sdmVBcHBMYWJlbChhcHBMYWJlbHNPdmVybGF5LCBcImdyb3VwXCIsIGNhdGVnb3J5LCBmYWxsYmFjayk7XG59XG5cbi8qKiDwn5eC77iPIE9yZGVyZWQsIGRlZHVwZWQgY2F0ZWdvcmllcyBmcm9tIHJlc29sdmVkIGFjdGlvbnMgKHNpYmxpbmcgb2Yge0BsaW5rIGNvbW1hbmRDYXRlZ29yaWVzfSkuICovXG5leHBvcnQgZnVuY3Rpb24gYWN0aW9uQ2F0ZWdvcmllcyhhY3Rpb25zOiByZWFkb25seSBBY3Rpb25EZWZpbml0aW9uW10sIGFwcExhYmVsc092ZXJsYXk6IFBsdWdpbkFwcExhYmVsc092ZXJsYXkgPSBFTVBUWV9BUFBfTEFCRUxTX09WRVJMQVkpOiB7IHJlYWRvbmx5IGlkOiBzdHJpbmc7IHJlYWRvbmx5IGxhYmVsOiBzdHJpbmcgfVtdIHtcbiAgY29uc3Qgc2VlbiA9IG5ldyBTZXQ8c3RyaW5nPigpO1xuICBjb25zdCBjYXRlZ29yaWVzOiB7IHJlYWRvbmx5IGlkOiBzdHJpbmc7IHJlYWRvbmx5IGxhYmVsOiBzdHJpbmcgfVtdID0gW107XG4gIGZvciAoY29uc3QgYWN0aW9uIG9mIGFjdGlvbnMpIHtcbiAgICBjb25zdCBpZCA9IGFjdGlvbkNhdGVnb3J5SWQoYWN0aW9uKTtcbiAgICBpZiAoc2Vlbi5oYXMoaWQpKSBjb250aW51ZTtcbiAgICBzZWVuLmFkZChpZCk7XG4gICAgY2F0ZWdvcmllcy5wdXNoKHsgaWQsIGxhYmVsOiBhY3Rpb25DYXRlZ29yeUxhYmVsKGlkLCBhcHBMYWJlbHNPdmVybGF5KSB9KTtcbiAgfVxuICByZXR1cm4gY2F0ZWdvcmllcztcbn1cblxuLyoqXG4gKiDwn46b77iPIENhdGVnb3J5IHNlY3Rpb25zIG9mIG9uZSB3aW5kb3cncyBBY3Rpb25zIHJhaWwgKFRyZWUgdHdpbiBvZiB7QGxpbmsgYnVpbGRDb21tYW5kQ2F0ZWdvcnlUcmVlfSk6XG4gKiBvbmUgc2VjdGlvbiBwZXIgY2F0ZWdvcnksIHplcm8tYXJnIGFjdGlvbnMgZmlyZSBkaXJlY3RseSwgYXJnLWNhcnJ5aW5nIGFjdGlvbnMgdG9nZ2xlIGEgc2libGluZyBmb3JtXG4gKiBzZWN0aW9uIOKAlCBleGFjdGx5IHtAbGluayBidWlsZENvbW1hbmRDYXRlZ29yeVRyZWV9J3MgbGlzdC9mb3JtIHNwbGl0LCBsb2NhbGl6ZWQgcGVyIGNhdGVnb3J5IHNvXG4gKiBtdWx0aXBsZSBjYXRlZ29yaWVzIGNhbiByZW5kZXIgc2lkZSBieSBzaWRlLiBPbmx5IG9uZSBhY3Rpb24gKGFjcm9zcyBhbGwgY2F0ZWdvcmllcykgaXMgZXhwYW5kZWQgYXQgYVxuICogdGltZSwgcGVyIGBleHBhbmRlZEFjdGlvbklkYC5cbiAqL1xuZXhwb3J0IGZ1bmN0aW9uIGJ1aWxkQWN0aW9uQ2F0ZWdvcnlUcmVlKFxuICB3aW5kb3dJZDogc3RyaW5nLFxuICBjb250cm9sbGVySWQ6IHN0cmluZyxcbiAgYWN0aW9uczogcmVhZG9ubHkgQWN0aW9uRGVmaW5pdGlvbltdLFxuICBleHBhbmRlZEFjdGlvbklkOiBzdHJpbmcgfCBudWxsLFxuICBzdGFnZWRBcmdzQnlLZXk6IFJlYWRvbmx5PFJlY29yZDxzdHJpbmcsIFJlYWRvbmx5PFJlY29yZDxzdHJpbmcsIHVua25vd24+Pj4+LFxuICBkaXNhYmxlZDogYm9vbGVhbixcbiAgb25FeHBhbmRlZENoYW5nZTogKGFjdGlvbklkOiBzdHJpbmcgfCBudWxsKSA9PiB2b2lkLFxuICBvblN0YWdlQXJnOiAoYWN0aW9uSWQ6IHN0cmluZywgYXJnSWQ6IHN0cmluZywgdmFsdWU6IHVua25vd24pID0+IHZvaWQsXG4gIG9uUmVzZXRBcmdzOiAoYWN0aW9uSWQ6IHN0cmluZykgPT4gdm9pZCxcbiAgb25FeGVjdXRlOiAoZGVzY3JpcHRvcjogQWN0aW9uRGVzY3JpcHRvcikgPT4gdm9pZCxcbiAgYXBwTGFiZWxzT3ZlcmxheTogUGx1Z2luQXBwTGFiZWxzT3ZlcmxheSA9IEVNUFRZX0FQUF9MQUJFTFNfT1ZFUkxBWSxcbik6IFRyZWVEYXRhU2VjdGlvbltdIHtcbiAgY29uc3QgY2F0ZWdvcmllcyA9IGFjdGlvbkNhdGVnb3JpZXMoYWN0aW9ucywgYXBwTGFiZWxzT3ZlcmxheSk7XG4gIGNvbnN0IGV4cGFuZGVkQWN0aW9uID0gZXhwYW5kZWRBY3Rpb25JZCA/IGFjdGlvbnMuZmluZCgoYWN0aW9uKSA9PiBhY3Rpb24uaWQgPT09IGV4cGFuZGVkQWN0aW9uSWQpIDogdW5kZWZpbmVkO1xuICBjb25zdCBzZWN0aW9uczogVHJlZURhdGFTZWN0aW9uW10gPSBbXTtcbiAgZm9yIChjb25zdCBjYXRlZ29yeSBvZiBjYXRlZ29yaWVzKSB7XG4gICAgY29uc3QgY2F0ZWdvcnlBY3Rpb25zID0gYWN0aW9ucy5maWx0ZXIoKGFjdGlvbikgPT4gYWN0aW9uQ2F0ZWdvcnlJZChhY3Rpb24pID09PSBjYXRlZ29yeS5pZCk7XG4gICAgc2VjdGlvbnMucHVzaCh7XG4gICAgICBpZDogYGFjdGlvbi5jYXRlZ29yeS4ke2NhdGVnb3J5LmlkfWAsXG4gICAgICBsYWJlbDogY2F0ZWdvcnkubGFiZWwsXG4gICAgICBkZWZhdWx0T3BlbjogdHJ1ZSxcbiAgICAgIGl0ZW1zOiBjYXRlZ29yeUFjdGlvbnMubWFwKChhY3Rpb24pOiBUcmVlRGF0YUl0ZW0gPT4ge1xuICAgICAgICBjb25zdCBpY29uID0gYWN0aW9uLmljb25JZCA/IDxJY29uIGljb249e2FjdGlvbi5pY29uSWQgYXMgSWNvbk5hbWV9IHNpemU9XCJzbWFsbFwiIC8+IDogdW5kZWZpbmVkO1xuICAgICAgICBjb25zdCByb3dDbGFzc05hbWUgPSBkaXNhYmxlZCA/IFwicG9pbnRlci1ldmVudHMtbm9uZSBvcGFjaXR5LTUwXCIgOiB1bmRlZmluZWQ7XG4gICAgICAgIGlmICghYWN0aW9uUmVxdWlyZXNTdGFnZWRGb3JtKGFjdGlvbikpIHtcbiAgICAgICAgICByZXR1cm4geyBpZDogYGFjdGlvbi4ke2FjdGlvbi5pZH1gLCBsYWJlbDogYWN0aW9uLmxhYmVsLCBpY29uLCBjbGFzc05hbWU6IHJvd0NsYXNzTmFtZSwgb25DbGljazogKCkgPT4gIWRpc2FibGVkICYmIG9uRXhlY3V0ZSh7IGNvbnRyb2xsZXJJZCwgYWN0aW9uOiBhY3Rpb24uaWQgfSkgfTtcbiAgICAgICAgfVxuICAgICAgICBjb25zdCBleHBhbmRlZCA9IGV4cGFuZGVkQWN0aW9uSWQgPT09IGFjdGlvbi5pZDtcbiAgICAgICAgcmV0dXJuIHtcbiAgICAgICAgICBpZDogYGFjdGlvbi4ke2FjdGlvbi5pZH1gLFxuICAgICAgICAgIGxhYmVsOiBgJHthY3Rpb24ubGFiZWx94oCmYCxcbiAgICAgICAgICBpY29uOiBpY29uID8/IDxJY29uIGljb249e2V4cGFuZGVkID8gXCJjaGV2cm9uLWRvd25cIiA6IFwiY2hldnJvbi1yaWdodFwifSBzaXplPVwic21hbGxcIiAvPixcbiAgICAgICAgICBjbGFzc05hbWU6IHJvd0NsYXNzTmFtZSxcbiAgICAgICAgICBvbkNsaWNrOiAoKSA9PiAhZGlzYWJsZWQgJiYgb25FeHBhbmRlZENoYW5nZShleHBhbmRlZCA/IG51bGwgOiBhY3Rpb24uaWQpLFxuICAgICAgICB9O1xuICAgICAgfSksXG4gICAgfSk7XG4gICAgaWYgKGV4cGFuZGVkQWN0aW9uICYmIGFjdGlvbkNhdGVnb3J5SWQoZXhwYW5kZWRBY3Rpb24pID09PSBjYXRlZ29yeS5pZCkge1xuICAgICAgY29uc3Qgc3RhZ2VkID0gc3RhZ2VkQXJnc0J5S2V5W2FjdGlvblN0YWdlS2V5KHdpbmRvd0lkLCBleHBhbmRlZEFjdGlvbi5pZCldID8/IHt9O1xuICAgICAgY29uc3QgZWZmZWN0aXZlID0gZWZmZWN0aXZlQWN0aW9uQXJncyhleHBhbmRlZEFjdGlvbi5hcmdzLCBzdGFnZWQpO1xuICAgICAgY29uc3QgbWlzc2luZyA9IG1pc3NpbmdSZXF1aXJlZEFyZ3MoZXhwYW5kZWRBY3Rpb24uYXJncywgZWZmZWN0aXZlKTtcbiAgICAgIHNlY3Rpb25zLnB1c2goe1xuICAgICAgICBpZDogYGFjdGlvbi5jYXRlZ29yeS4ke2NhdGVnb3J5LmlkfS5mb3JtYCxcbiAgICAgICAgZGVmYXVsdE9wZW46IHRydWUsXG4gICAgICAgIGl0ZW1zOiBleHBhbmRlZEFjdGlvbi5hcmdzLm1hcChcbiAgICAgICAgICAoZGVmKTogVHJlZURhdGFJdGVtID0+ICh7XG4gICAgICAgICAgICBpZDogYGFjdGlvbi4ke2V4cGFuZGVkQWN0aW9uLmlkfS5hcmcuJHtkZWYuaWR9YCxcbiAgICAgICAgICAgIGxhYmVsOiBkZWYubGFiZWwsXG4gICAgICAgICAgICBkZXNjcmlwdGlvbjogZGVmLmRlc2NyaXB0aW9uLFxuICAgICAgICAgICAgY29udHJvbDogcmVuZGVyU3RhZ2VkQXJnQ29udHJvbChkZWYsIGVmZmVjdGl2ZVtkZWYuaWRdLCAodmFsdWUpID0+IG9uU3RhZ2VBcmcoZXhwYW5kZWRBY3Rpb24uaWQsIGRlZi5pZCwgdmFsdWUpLCBkaXNhYmxlZCksXG4gICAgICAgICAgfSksXG4gICAgICAgICksXG4gICAgICAgIGFjdGlvbnM6IFtcbiAgICAgICAgICB7XG4gICAgICAgICAgICBpZDogY2hpbGRFbGVtZW50SWQoXCJmcmFtZXdvcmsud2luZG93XCIsIHdpbmRvd0lkLCBcImFjdGlvblwiLCBleHBhbmRlZEFjdGlvbi5pZCwgXCJleGVjdXRlXCIpLFxuICAgICAgICAgICAgaWNvbjogPEljb24gaWNvbj1cImNoZWNrXCIgc2l6ZT1cInNtYWxsXCIgLz4sXG4gICAgICAgICAgICB0ZXh0OiBzaGVsbExhYmVsKFwidWkuY29tbW9uLmV4ZWN1dGVcIiksXG4gICAgICAgICAgICBkaXNhYmxlZDogZGlzYWJsZWQgfHwgbWlzc2luZy5sZW5ndGggPiAwLFxuICAgICAgICAgICAgb25DbGljazogKCkgPT4gb25FeGVjdXRlKHsgY29udHJvbGxlcklkLCBhY3Rpb246IGV4cGFuZGVkQWN0aW9uLmlkLCBhcmdzOiBlZmZlY3RpdmUgfSksXG4gICAgICAgICAgfSxcbiAgICAgICAgICB7XG4gICAgICAgICAgICBpZDogY2hpbGRFbGVtZW50SWQoXCJmcmFtZXdvcmsud2luZG93XCIsIHdpbmRvd0lkLCBcImFjdGlvblwiLCBleHBhbmRlZEFjdGlvbi5pZCwgXCJyZXNldFwiKSxcbiAgICAgICAgICAgIGljb246IDxJY29uIGljb249XCJ1bmRvXCIgc2l6ZT1cInNtYWxsXCIgLz4sXG4gICAgICAgICAgICB0ZXh0OiBzaGVsbExhYmVsKFwidWkuY29tbW9uLnJlc2V0XCIpLFxuICAgICAgICAgICAgZGlzYWJsZWQsXG4gICAgICAgICAgICBvbkNsaWNrOiAoKSA9PiBvblJlc2V0QXJncyhleHBhbmRlZEFjdGlvbi5pZCksXG4gICAgICAgICAgfSxcbiAgICAgICAgXSxcbiAgICAgIH0pO1xuICAgIH1cbiAgfVxuICByZXR1cm4gc2VjdGlvbnM7XG59XG5cbi8qKiDwn46b77iPIFByb3BzIGZvciB0aGUgcGVyLXdpbmRvdyBBY3Rpb24gcmFpbCBib2R5IChQMS9QMikuICovXG5leHBvcnQgdHlwZSBXaW5kb3dBY3Rpb25QYW5lUHJvcHMgPSB7XG4gIHJlYWRvbmx5IHdpbmRvd0lkOiBzdHJpbmc7XG4gIHJlYWRvbmx5IGNvbnRyb2xsZXJJZDogc3RyaW5nO1xuICByZWFkb25seSBhY3Rpb25zOiByZWFkb25seSBBY3Rpb25EZWZpbml0aW9uW107XG4gIHJlYWRvbmx5IGV4cGFuZGVkQWN0aW9uSWQ6IHN0cmluZyB8IG51bGw7XG4gIHJlYWRvbmx5IHN0YWdlZEFyZ3NCeUtleTogUmVhZG9ubHk8UmVjb3JkPHN0cmluZywgUmVhZG9ubHk8UmVjb3JkPHN0cmluZywgdW5rbm93bj4+Pj47XG4gIHJlYWRvbmx5IGRpc2FibGVkOiBib29sZWFuO1xuICByZWFkb25seSBvbkV4cGFuZGVkQ2hhbmdlOiAoYWN0aW9uSWQ6IHN0cmluZyB8IG51bGwpID0+IHZvaWQ7XG4gIHJlYWRvbmx5IG9uU3RhZ2VBcmc6IChhY3Rpb25JZDogc3RyaW5nLCBhcmdJZDogc3RyaW5nLCB2YWx1ZTogdW5rbm93bikgPT4gdm9pZDtcbiAgcmVhZG9ubHkgb25SZXNldEFyZ3M6IChhY3Rpb25JZDogc3RyaW5nKSA9PiB2b2lkO1xuICByZWFkb25seSBvbkV4ZWN1dGU6IChkZXNjcmlwdG9yOiBBY3Rpb25EZXNjcmlwdG9yKSA9PiB2b2lkO1xuICByZWFkb25seSBhcHBMYWJlbHNPdmVybGF5PzogUGx1Z2luQXBwTGFiZWxzT3ZlcmxheTtcbn07XG5cbi8qKlxuICog8J+Om++4jyBUaGUgcGVyLXdpbmRvdyBBY3Rpb25zIHJhaWwgYm9keSAoUDEvUDIpLCBncm91cGVkIGludG8gY2F0ZWdvcmllcyBsaWtlIHRoZSBjb21tYW5kIHBhbmVsLiBaZXJvLWFyZ1xuICogYWN0aW9ucyBmaXJlIGRpcmVjdGx5OyBhcmctY2FycnlpbmcgYWN0aW9ucyBleHBhbmQgYSBsb2NhbGx5LWJ1ZmZlcmVkIHN0YWdlZCBmb3JtIChzYW1lIGlubGluZVxuICogcHJvcGVydHktcm93IGNvbnRyb2xzIGFzIHV0aWxpdHkgbWVhc3VyZXMpIOKAlCBub3RoaW5nIGRpc3BhdGNoZXMgb24gZWRpdCwgZWZmZWN0aXZlIHZhbHVlIGlzXG4gKiBgc3RhZ2VkID8/IGRlZmF1bHQgPz8gdW5zZXRgLCBFeGVjdXRlIGlzIGVuYWJsZWQgb25seSB3aGVuIGV2ZXJ5IHJlcXVpcmVkIGFyZyBoYXMgYW4gZWZmZWN0aXZlIHZhbHVlLFxuICogZmlyZXMgZXhhY3RseSBPTkUgYEFjdGlvbkRlc2NyaXB0b3JgIHdpdGggdGhlIG1lcmdlZCBhcmdzLCBhbmQga2VlcHMgdGhlIHN0YWdlZCB2YWx1ZXMgYWZ0ZXJ3YXJkLlxuICogV2hlbiBgZGlzYWJsZWRgIChhbiBhY3RpdmUgdXRpbGl0eSB3aXRoIGBhbGxvd3NBY3Rpb25zV2hpbGVBY3RpdmUgPT09IGZhbHNlYCksIGV2ZXJ5IHJvdyByZW5kZXJzIGRpc2FibGVkLlxuICovXG5leHBvcnQgZnVuY3Rpb24gV2luZG93QWN0aW9uUGFuZShwcm9wczogV2luZG93QWN0aW9uUGFuZVByb3BzKTogUmVhY3RFbGVtZW50IHtcbiAgY29uc3QgeyB3aW5kb3dJZCwgY29udHJvbGxlcklkLCBhY3Rpb25zLCBleHBhbmRlZEFjdGlvbklkLCBzdGFnZWRBcmdzQnlLZXksIGRpc2FibGVkLCBvbkV4cGFuZGVkQ2hhbmdlLCBvblN0YWdlQXJnLCBvblJlc2V0QXJncywgb25FeGVjdXRlLCBhcHBMYWJlbHNPdmVybGF5IH0gPSBwcm9wcztcbiAgY29uc3Qgc2VjdGlvbnMgPSBidWlsZEFjdGlvbkNhdGVnb3J5VHJlZSh3aW5kb3dJZCwgY29udHJvbGxlcklkLCBhY3Rpb25zLCBleHBhbmRlZEFjdGlvbklkLCBzdGFnZWRBcmdzQnlLZXksIGRpc2FibGVkLCBvbkV4cGFuZGVkQ2hhbmdlLCBvblN0YWdlQXJnLCBvblJlc2V0QXJncywgb25FeGVjdXRlLCBhcHBMYWJlbHNPdmVybGF5KTtcbiAgcmV0dXJuIChcbiAgICA8ZGl2IGRhdGEtc2xvdD1cIndpbmRvdy1hY3Rpb24tcGFuZVwiIGNsYXNzTmFtZT1cImZsZXggbWluLXctMCBmbGV4LWNvbFwiPlxuICAgICAgPFRyZWUgc2VjdGlvbnM9e3NlY3Rpb25zfSBzaG93TGluZXM9e2ZhbHNlfSBzb3J0YWJsZVNlY3Rpb25zPXtmYWxzZX0gLz5cbiAgICA8L2Rpdj5cbiAgKTtcbn1cblxuLyoqIPCfp7DvuI8gU2xpY2Ugb2YgdGhlIHtAbGluayBBY3Rpb25QYW5lU3RhdGV9IHRoZSB7QGxpbmsgd2luZG93QWN0aW9uUGFuZU5vZGV9IGJ1aWxkZXIgcmVhZHMuICovXG50eXBlIEFjdGlvblBhbmVTbGljZSA9IFBpY2s8QWN0aW9uUGFuZVN0YXRlLCBcImV4cGFuZGVkQnlXaW5kb3dJZFwiIHwgXCJzdGFnZWRBcmdzQnlLZXlcIiB8IFwiYWN0aXZlVXRpbGl0eUJ5V2luZG93SWRcIj47XG5cbi8qKlxuICog8J+nsO+4jyBTaWJsaW5nIG9mIHtAbGluayB1dGlsaXR5QmFyTm9kZX06IHJlc29sdmVzIGEgd2luZG93IGtpbmQncyBwYW5lbC1lbGlnaWJsZSBhY3Rpb25zIGFuZCByZXR1cm5zIGFcbiAqIGJvdW5kIHtAbGluayBXaW5kb3dBY3Rpb25QYW5lfSwgb3IgYHVuZGVmaW5lZGAgd2hlbiB0aGUgd2luZG93IGhhcyBubyByZXNvbHZlZCBhY3Rpb25zIChzbyB0aGUgcmFpbFxuICogY2hpcCBuZXZlciByZW5kZXJzKS4gUm93cyByZW5kZXIgZGlzYWJsZWQgd2hpbGUgYW4gYWN0aXZlIHV0aWxpdHkgZ2F0ZXMgYWN0aW9uc1xuICogKGBhbGxvd3NBY3Rpb25zV2hpbGVBY3RpdmUgPT09IGZhbHNlYCkuXG4gKi9cbmV4cG9ydCBmdW5jdGlvbiB3aW5kb3dBY3Rpb25QYW5lTm9kZShcbiAgYXBwOiBBcHBEZWZpbml0aW9uLFxuICB3aW5kb3dLaW5kOiBBcHBXaW5kb3dLaW5kRGVmaW5pdGlvbixcbiAgd2luZG93SWQ6IHN0cmluZyxcbiAgYWN0aW9uUGFuZTogQWN0aW9uUGFuZVNsaWNlLFxuICBvbkFjdGlvbjogKGFjdGlvbjogQWN0aW9uRGVzY3JpcHRvcikgPT4gdm9pZCxcbiAgZGlzcGF0Y2g6IChhY3Rpb246IFNoZWxsQWN0aW9uKSA9PiB2b2lkLFxuICBhcHBMYWJlbHNPdmVybGF5OiBQbHVnaW5BcHBMYWJlbHNPdmVybGF5ID0gRU1QVFlfQVBQX0xBQkVMU19PVkVSTEFZLFxuICB0ZXJtaW5vbG9neTogc3RyaW5nID0gVUlfVEVSTUlOT0xPR1lfTkFUSVZFLFxuICBsb2NhbGU6IHN0cmluZyA9IFNIRUxMX0xPQ0FMRVNbMF0sXG4pOiBSZWFjdE5vZGUge1xuICBjb25zdCByZXNvbHZlZEFjdGlvbnMgPSByZXNvbHZlV2luZG93QWN0aW9ucyhhcHAsIHdpbmRvd0tpbmQpO1xuICBpZiAocmVzb2x2ZWRBY3Rpb25zLmxlbmd0aCA9PT0gMCkgcmV0dXJuIHVuZGVmaW5lZDtcbiAgY29uc3QgYWN0aW9ucyA9IHJlc29sdmVkQWN0aW9ucy5tYXAoKGFjdGlvbikgPT4gKHtcbiAgICAuLi5hY3Rpb24sXG4gICAgbGFiZWw6IHJlc29sdmVBcHBMYWJlbChhcHBMYWJlbHNPdmVybGF5LCBcImFjdGlvblwiLCBhY3Rpb24uaWQsIHJlc29sdmVNYW5pZmVzdExhYmVsKGFjdGlvbi5sYWJlbCwgdGVybWlub2xvZ3ksIGxvY2FsZSkpLFxuICAgIGFyZ3M6IGFjdGlvbi5hcmdzLm1hcCgoZGVmKSA9PiByZXNvbHZlQWN0aW9uQXJnRGVmKGRlZiwgYWN0aW9uLmlkLCBhcHBMYWJlbHNPdmVybGF5LCB0ZXJtaW5vbG9neSwgbG9jYWxlKSksXG4gIH0pKTtcbiAgY29uc3QgYWN0aXZlVXRpbGl0eUlkID0gYWN0aW9uUGFuZS5hY3RpdmVVdGlsaXR5QnlXaW5kb3dJZFt3aW5kb3dJZF0gPz8gbnVsbDtcbiAgY29uc3QgYWN0aXZlVXRpbGl0eSA9IGFjdGl2ZVV0aWxpdHlJZCA/IChhcHAudXRpbGl0aWVzID8/IFtdKS5maW5kKCh1dGlsaXR5KSA9PiB1dGlsaXR5LmlkID09PSBhY3RpdmVVdGlsaXR5SWQpIDogdW5kZWZpbmVkO1xuICBjb25zdCBkaXNhYmxlZCA9IEJvb2xlYW4oYWN0aXZlVXRpbGl0eSAmJiBhY3RpdmVVdGlsaXR5LmFsbG93c0FjdGlvbnNXaGlsZUFjdGl2ZSA9PT0gZmFsc2UpO1xuICByZXR1cm4gKFxuICAgIDxXaW5kb3dBY3Rpb25QYW5lXG4gICAgICB3aW5kb3dJZD17d2luZG93SWR9XG4gICAgICBjb250cm9sbGVySWQ9e2FwcC5jb250cm9sbGVySWR9XG4gICAgICBhY3Rpb25zPXthY3Rpb25zfVxuICAgICAgZXhwYW5kZWRBY3Rpb25JZD17YWN0aW9uUGFuZS5leHBhbmRlZEJ5V2luZG93SWRbd2luZG93SWRdID8/IG51bGx9XG4gICAgICBzdGFnZWRBcmdzQnlLZXk9e2FjdGlvblBhbmUuc3RhZ2VkQXJnc0J5S2V5fVxuICAgICAgZGlzYWJsZWQ9e2Rpc2FibGVkfVxuICAgICAgb25FeHBhbmRlZENoYW5nZT17KGFjdGlvbklkKSA9PiBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0FDVElPTl9QQU5FX0VYUEFOREVEXCIsIHdpbmRvd0lkLCB2YWx1ZTogYWN0aW9uSWQgfSl9XG4gICAgICBvblN0YWdlQXJnPXsoYWN0aW9uSWQsIGFyZ0lkLCB2YWx1ZSkgPT4gZGlzcGF0Y2goeyB0eXBlOiBcIlNUQUdFX0FDVElPTl9BUkdcIiwgd2luZG93SWQsIGFjdGlvbklkLCBhcmdJZCwgdmFsdWUgfSl9XG4gICAgICBvblJlc2V0QXJncz17KGFjdGlvbklkKSA9PiBkaXNwYXRjaCh7IHR5cGU6IFwiUkVTRVRfQUNUSU9OX0FSR1NcIiwgd2luZG93SWQsIGFjdGlvbklkIH0pfVxuICAgICAgb25FeGVjdXRlPXtvbkFjdGlvbn1cbiAgICAgIGFwcExhYmVsc092ZXJsYXk9e2FwcExhYmVsc092ZXJsYXl9XG4gICAgLz5cbiAgKTtcbn1cbi8vI2VuZHJlZ2lvbiDwn6ew77iPV2luZG93QWN0aW9uUGFuZVxuXG4vLyNyZWdpb24g8J+Om++4j0NvbW1hbmRSZWdpc3RyeVxuLyoqIPCfjpvvuI8gV2hlcmUgYSByZXNvbHZlZCBjb21tYW5kIGNhbWUgZnJvbSDigJQgZHJpdmVzIHBhbGV0dGUvZm9vdGVyIGNhdGVnb3J5IGdyb3VwaW5nIGFuZCBkaXNwYXRjaCByb3V0aW5nLiAqL1xuZXhwb3J0IHR5cGUgUmVzb2x2ZWRDb21tYW5kID0ge1xuICByZWFkb25seSBkZWZpbml0aW9uOiBDb21tYW5kRGVmaW5pdGlvbjtcbiAgcmVhZG9ubHkgc291cmNlOiB7IHJlYWRvbmx5IGtpbmQ6IFwib3NcIiB9IHwgeyByZWFkb25seSBraW5kOiBcInBsdWdpblwiIH0gfCB7IHJlYWRvbmx5IGtpbmQ6IFwiYXBwXCIgfSB8IHsgcmVhZG9ubHkga2luZDogXCJtb2RlXCI7IHJlYWRvbmx5IG1vZGVJZDogc3RyaW5nIH07XG59O1xuXG4vKipcbiAqIPCfjpvvuI8gQWdncmVnYXRlcyBldmVyeSBjb21tYW5kIHZpc2libGUgZm9yIHRoZSBjdXJyZW50IHNlc3Npb246IG9zIGJ1aWx0LWlucywgdGhlIGFjdGl2ZSBzZXNzaW9uJ3NcbiAqIHBsdWdpbi1zY29wZSBjb21tYW5kcywgdGhlIGFwcCdzIEFwcC1zY29wZSBjb21tYW5kcywgYW5kIE1vZGUtc2NvcGUgY29tbWFuZHMgcmVmZXJlbmNlZCBieSB0aGVcbiAqIGFjdGl2ZSBtb2RlJ3MgYGNvbW1hbmRzYCByZWZzLiBUaGVyZSBhcmUgbm8gd2luZG93LWxldmVsIGNvbW1hbmRzIChzZWUgYENvbW1hbmRTY29wZWApIOKAlCB1bmxpa2VcbiAqIGByZXNvbHZlV2luZG93QWN0aW9uc2AvYHJlc29sdmVVdGlsaXRpZXNgLCB0aGlzIG5ldmVyIHRha2VzIGEgd2luZG93IGtpbmQuXG4gKi9cbmV4cG9ydCBmdW5jdGlvbiByZXNvbHZlQ29tbWFuZHMoXG4gIG9zQ29tbWFuZHM6IHJlYWRvbmx5IENvbW1hbmREZWZpbml0aW9uW10sXG4gIGFjdGl2ZVBsdWdpbk1hbmlmZXN0OiBQaWNrPFBsdWdpbk1hbmlmZXN0LCBcImNvbW1hbmRzXCI+IHwgbnVsbCB8IHVuZGVmaW5lZCxcbiAgYXBwOiBQaWNrPEFwcERlZmluaXRpb24sIFwiY29tbWFuZHNcIiB8IFwibW9kZXNcIj4gfCBudWxsIHwgdW5kZWZpbmVkLFxuICBhY3RpdmVNb2RlSWQ6IHN0cmluZyxcbiAgb3ZlcmxheTogUGx1Z2luQXBwTGFiZWxzT3ZlcmxheSA9IEVNUFRZX0FQUF9MQUJFTFNfT1ZFUkxBWSxcbiAgdGVybWlub2xvZ3k6IHN0cmluZyA9IFVJX1RFUk1JTk9MT0dZX05BVElWRSxcbiAgbG9jYWxlOiBzdHJpbmcgPSBTSEVMTF9MT0NBTEVTWzBdLFxuKTogUmVzb2x2ZWRDb21tYW5kW10ge1xuICAvLyDwn5e677iPIGBDb21tYW5kRGVmaW5pdGlvbi5sYWJlbGAvYC5hcmdzW10ubGFiZWxgIGFyZSBtYW5pZmVzdCBgTG9jYWxpemVkTGFiZWxgIGZpZWxkcyDigJQgdGhlcmUgaXMgbm9cbiAgLy8gXCJjb21tYW5kXCIvXCJjb21tYW5kQXJnXCIgb3ZlcmxheSBjYXRlZ29yeSAoY29tbWFuZHMgbmV2ZXIgd2VudCB0aHJvdWdoIGBBcHBMYWJlbHNPdmVybGF5YCksIHNvIHRoaXMgaXNcbiAgLy8gdGhlIHNpbmdsZSBjaG9rZSBwb2ludCB0aGF0IHJlc29sdmVzIHRoZW0gdG8gcGxhaW4gc3RyaW5ncyBmb3IgZXZlcnkgZG93bnN0cmVhbSBjb25zdW1lciAodGhlXG4gIC8vIGZvb3RlciBjb21tYW5kIHBhbmVsLCB0aGUgY29tbWFuZCBwYWxldHRlLCBgbm90ZVNoZWxsQ29tbWFuZGAncyBoaXN0b3J5IGxhYmVsKTsgYG9zQ29tbWFuZHNgIGFyZVxuICAvLyBhbHJlYWR5IHBsYWluIHN0cmluZ3MgKGJ1aWx0IGJ5IGBidWlsZE9zQ29tbWFuZHNgIHZpYSBgc2hlbGxMYWJlbGApIGFuZCBwYXNzIHRocm91Z2ggdW5jaGFuZ2VkLlxuICBjb25zdCByZXNvbHZlRGVmaW5pdGlvbiA9IChkZWZpbml0aW9uOiBDb21tYW5kRGVmaW5pdGlvbik6IENvbW1hbmREZWZpbml0aW9uID0+ICh7XG4gICAgLi4uZGVmaW5pdGlvbixcbiAgICBsYWJlbDogcmVzb2x2ZU1hbmlmZXN0TGFiZWwoZGVmaW5pdGlvbi5sYWJlbCwgdGVybWlub2xvZ3ksIGxvY2FsZSksXG4gICAgYXJnczogZGVmaW5pdGlvbi5hcmdzLm1hcCgoZGVmKSA9PiByZXNvbHZlQWN0aW9uQXJnRGVmKGRlZiwgZGVmaW5pdGlvbi5pZCwgb3ZlcmxheSwgdGVybWlub2xvZ3ksIGxvY2FsZSkpLFxuICB9KTtcbiAgY29uc3QgcmVzb2x2ZWQ6IFJlc29sdmVkQ29tbWFuZFtdID0gb3NDb21tYW5kcy5tYXAoKGRlZmluaXRpb24pID0+ICh7IGRlZmluaXRpb246IHJlc29sdmVEZWZpbml0aW9uKGRlZmluaXRpb24pLCBzb3VyY2U6IHsga2luZDogXCJvc1wiIGFzIGNvbnN0IH0gfSkpO1xuICBmb3IgKGNvbnN0IGRlZmluaXRpb24gb2YgYWN0aXZlUGx1Z2luTWFuaWZlc3Q/LmNvbW1hbmRzID8/IFtdKSB7XG4gICAgcmVzb2x2ZWQucHVzaCh7IGRlZmluaXRpb246IHJlc29sdmVEZWZpbml0aW9uKGRlZmluaXRpb24pLCBzb3VyY2U6IHsga2luZDogXCJwbHVnaW5cIiBhcyBjb25zdCB9IH0pO1xuICB9XG4gIGlmICghYXBwKSByZXR1cm4gcmVzb2x2ZWQ7XG4gIGNvbnN0IGFjdGl2ZU1vZGUgPSAoYXBwLm1vZGVzIGFzIHJlYWRvbmx5IEFwcE1vZGVEZWZpbml0aW9uW10gfCB1bmRlZmluZWQpPy5maW5kKChtb2RlKSA9PiBtb2RlLmlkID09PSBhY3RpdmVNb2RlSWQpO1xuICBjb25zdCBtb2RlQ29tbWFuZElkcyA9IG5ldyBTZXQoYWN0aXZlTW9kZT8uY29tbWFuZHMgPz8gW10pO1xuICBmb3IgKGNvbnN0IGRlZmluaXRpb24gb2YgYXBwLmNvbW1hbmRzID8/IFtdKSB7XG4gICAgaWYgKGRlZmluaXRpb24uc2NvcGUgPT09IFwiYXBwXCIpIHJlc29sdmVkLnB1c2goeyBkZWZpbml0aW9uOiByZXNvbHZlRGVmaW5pdGlvbihkZWZpbml0aW9uKSwgc291cmNlOiB7IGtpbmQ6IFwiYXBwXCIgYXMgY29uc3QgfSB9KTtcbiAgICBlbHNlIGlmIChkZWZpbml0aW9uLnNjb3BlID09PSBcIm1vZGVcIiAmJiBtb2RlQ29tbWFuZElkcy5oYXMoZGVmaW5pdGlvbi5pZCkpIHJlc29sdmVkLnB1c2goeyBkZWZpbml0aW9uOiByZXNvbHZlRGVmaW5pdGlvbihkZWZpbml0aW9uKSwgc291cmNlOiB7IGtpbmQ6IFwibW9kZVwiIGFzIGNvbnN0LCBtb2RlSWQ6IGFjdGl2ZU1vZGVJZCB9IH0pO1xuICB9XG4gIHJldHVybiByZXNvbHZlZDtcbn1cblxuLyoqIPCfjpvvuI8gQ2hyb21lLWtub3duIGNvbW1hbmQgY2F0ZWdvcnkgaWRzIHRoYXQgYWxyZWFkeSBoYXZlIGEgYHVpLnNldHRpbmdzLnRhYi4qYCB0cmFuc2xhdGlvbiBrZXkuICovXG5jb25zdCBDSFJPTUVfS05PV05fQ09NTUFORF9DQVRFR09SSUVTID0gbmV3IFNldChbXCJnZW5lcmFsXCIsIFwiZHJpdmVyXCIsIFwiYXBwXCIsIFwiYXBwZWFyYW5jZVwiLCBcImxheW91dFwiLCBcImxhbmd1YWdlXCIsIFwidGVybWlub2xvZ3lcIiwgXCJ0aGVtZVwiXSk7XG5cbi8qKiDwn46b77iPIExvb3NlIHRpdGxlLWNhc2UgZm9yIGFuIG9wZW4tc2V0IGNvbW1hbmQgY2F0ZWdvcnkgaWQgKGUuZy4gXCJhcHBlYXJhbmNlXCIgLT4gXCJBcHBlYXJhbmNlXCIpLiBGYWxscyBiYWNrIHRvIHRoaXMgZm9yIGFwcC9wbHVnaW4taW52ZW50ZWQgY2F0ZWdvcmllcyB0aGF0IGhhdmUgbm8gZml4ZWQgZnJhbWV3b3JrIHZvY2FidWxhcnkgZW50cnkuICovXG5mdW5jdGlvbiB0aXRsZWl6ZUNvbW1hbmRDYXRlZ29yeShjYXRlZ29yeTogc3RyaW5nKTogc3RyaW5nIHtcbiAgcmV0dXJuIGNhdGVnb3J5LnJlcGxhY2UoL1stX10rL2csIFwiIFwiKS5yZXBsYWNlKC9cXGJcXHcvZywgKGNoYXIpID0+IGNoYXIudG9VcHBlckNhc2UoKSk7XG59XG5cbi8qKiDwn46b77iPIFJlc29sdmVzIGEgY29tbWFuZCBjYXRlZ29yeSdzIGRpc3BsYXkgbGFiZWwsIHJldXNpbmcgdGhlIGV4aXN0aW5nIGB1aS5zZXR0aW5ncy50YWIuKmAga2V5cyBmb3IgY2hyb21lLWtub3duIGlkcyBhbmQgZmFsbGluZyBiYWNrIHRvIGEgbG9vc2UgdGl0bGUtY2FzZSBmb3Igb3Blbi1zZXQgYXBwL3BsdWdpbiBjYXRlZ29yaWVzLiAqL1xuZXhwb3J0IGZ1bmN0aW9uIGNvbW1hbmRDYXRlZ29yeUxhYmVsKGNhdGVnb3J5OiBzdHJpbmcpOiBzdHJpbmcge1xuICByZXR1cm4gQ0hST01FX0tOT1dOX0NPTU1BTkRfQ0FURUdPUklFUy5oYXMoY2F0ZWdvcnkpID8gc2hlbGxMYWJlbChgdWkuc2V0dGluZ3MudGFiLiR7Y2F0ZWdvcnkgYXMgXCJnZW5lcmFsXCIgfCBcImRyaXZlclwiIHwgXCJhcHBcIiB8IFwiYXBwZWFyYW5jZVwiIHwgXCJsYXlvdXRcIiB8IFwibGFuZ3VhZ2VcIiB8IFwidGVybWlub2xvZ3lcIiB8IFwidGhlbWVcIn1gKSA6IHRpdGxlaXplQ29tbWFuZENhdGVnb3J5KGNhdGVnb3J5KTtcbn1cblxuLyoqIPCfjpvvuI8gT3JkZXJlZCwgZGVkdXBlZCBjYXRlZ29yeSB0YWJzIGZvciB0aGUgZm9vdGVyIGNvbW1hbmQgcGFuZWwsIGRlcml2ZWQgZnJvbSB3aGF0ZXZlciBjb21tYW5kcyBhY3R1YWxseSByZXNvbHZlZC4gKi9cbmV4cG9ydCBmdW5jdGlvbiBjb21tYW5kQ2F0ZWdvcmllcyhjb21tYW5kczogcmVhZG9ubHkgUmVzb2x2ZWRDb21tYW5kW10pOiB7IHJlYWRvbmx5IGlkOiBzdHJpbmc7IHJlYWRvbmx5IGxhYmVsOiBzdHJpbmcgfVtdIHtcbiAgY29uc3Qgc2VlbiA9IG5ldyBTZXQ8c3RyaW5nPigpO1xuICBjb25zdCBjYXRlZ29yaWVzOiB7IHJlYWRvbmx5IGlkOiBzdHJpbmc7IHJlYWRvbmx5IGxhYmVsOiBzdHJpbmcgfVtdID0gW107XG4gIGZvciAoY29uc3QgeyBkZWZpbml0aW9uIH0gb2YgY29tbWFuZHMpIHtcbiAgICBpZiAoc2Vlbi5oYXMoZGVmaW5pdGlvbi5jYXRlZ29yeSkpIGNvbnRpbnVlO1xuICAgIHNlZW4uYWRkKGRlZmluaXRpb24uY2F0ZWdvcnkpO1xuICAgIGNhdGVnb3JpZXMucHVzaCh7IGlkOiBkZWZpbml0aW9uLmNhdGVnb3J5LCBsYWJlbDogY29tbWFuZENhdGVnb3J5TGFiZWwoZGVmaW5pdGlvbi5jYXRlZ29yeSkgfSk7XG4gIH1cbiAgcmV0dXJuIGNhdGVnb3JpZXM7XG59XG5cbmZ1bmN0aW9uIHNlbGVjdENvbW1hbmRBcmcoaWQ6IHN0cmluZywgbGFiZWw6IHN0cmluZywgb3B0aW9uczogcmVhZG9ubHkgeyByZWFkb25seSB2YWx1ZTogc3RyaW5nOyByZWFkb25seSBsYWJlbDogc3RyaW5nIH1bXSk6IEFjdGlvbkFyZ0RlZiB7XG4gIHJldHVybiB7IGlkLCBsYWJlbCwgY29udHJvbDogeyBraW5kOiBcInNlbGVjdFwiLCBvcHRpb25zOiBvcHRpb25zLm1hcCgob3B0aW9uKSA9PiAoeyAuLi5vcHRpb24gfSkpIH0sIHJlcXVpcmVkOiB0cnVlIH07XG59XG5cbi8qKiBAZW1vamkg8J+al++4jyBUcmFuc2xhdGVkIGRpc3BsYXkgbmFtZSBmb3IgYSBidWlsdC1pbiBkcml2ZXIgaWQ7IGEgY3VzdG9tICh1c2VyLWF1dGhvcmVkKSBkcml2ZXIgaGFzIG5vXG4gKiB0cmFuc2xhdGlvbiBrZXksIHNvIGl0cyBvd24ge0BsaW5rIFVpRHJpdmVyLmxhYmVsfSAoZ2VudWluZSBydW50aW1lIGRhdGEpIGlzIHRoZSBjb3JyZWN0IGZhbGxiYWNrLiAqL1xuZXhwb3J0IGZ1bmN0aW9uIGRyaXZlckRpc3BsYXlMYWJlbChkcml2ZXI6IFVpRHJpdmVyKTogc3RyaW5nIHtcbiAgaWYgKGRyaXZlci5pZCA9PT0gXCJkZWZhdWx0XCIpIHJldHVybiBzaGVsbExhYmVsKFwic2V0dGluZ3MuZHJpdmVyLmRlZmF1bHRcIik7XG4gIGlmIChkcml2ZXIuaWQgPT09IFwiY29tcGFjdFwiKSByZXR1cm4gc2hlbGxMYWJlbChcInNldHRpbmdzLmRyaXZlci5jb21wYWN0XCIpO1xuICByZXR1cm4gZHJpdmVyLmxhYmVsIHx8IGRyaXZlci5pZDtcbn1cblxuLyoqXG4gKiDwn46b77iPIE9zLWxldmVsIGJ1aWx0LWluIGNvbW1hbmRzIOKAlCBhcHAgaW50cm9kdWN0aW9uL3RoZW1lL2xheW91dC9sb2NhbGUvYXBwZWFyYW5jZS9kcml2ZXIsXG4gKiBgc2NvcGU6IFwib3NcImAsIGhhbmRsZWRcbiAqIGxvY2FsbHkgYnkgdGhlIHNoZWxsIChuZXZlciByb3V0ZWQgdG8gYSBwcm9ncmFtKS4gUmVidWlsdCB2aWEgYHVzZU1lbW9gIHNpbmNlIHRoZSB0aGVtZSBhbmRcbiAqIHRlcm1pbm9sb2d5IG9wdGlvbiBsaXN0cyBhcmUgbGl2ZSBzdGF0ZS5cbiAqL1xuZXhwb3J0IGZ1bmN0aW9uIGJ1aWxkT3NDb21tYW5kcyhcbiAgdGhlbWVMaXN0OiByZWFkb25seSBVaVRoZW1lW10sXG4gIHRlcm1pbm9sb2dpZXM6IHJlYWRvbmx5IHN0cmluZ1tdLFxuICBoYXNJbnRyb2R1Y3Rpb246IGJvb2xlYW4sXG4gIGxvY2tzOiBSZXNvbHZlZFNoZWxsTG9ja3MgPSBFTVBUWV9TSEVMTF9MT0NLUyxcbiAgZHJpdmVyTGlzdDogcmVhZG9ubHkgVWlEcml2ZXJbXSA9IGJ1aWx0aW5VaURyaXZlcnMoKSxcbiAgdHV0b3JpYWxzOiByZWFkb25seSB7IHJlYWRvbmx5IGlkOiBzdHJpbmc7IHJlYWRvbmx5IHRpdGxlOiBMb2NhbGl6ZWRMYWJlbCB8IHN0cmluZyB9W10gPSBbXSxcbiAgdHV0b3JpYWxSZWNvcmRlckF2YWlsYWJsZSA9IGZhbHNlLFxuICB0ZXJtaW5vbG9neTogc3RyaW5nID0gVUlfVEVSTUlOT0xPR1lfTkFUSVZFLFxuICBsb2NhbGU6IHN0cmluZyA9IFNIRUxMX0xPQ0FMRVNbMF0sXG4pOiBDb21tYW5kRGVmaW5pdGlvbltdIHtcbiAgY29uc3QgbG9ja2VkQ29tbWFuZElkcyA9IG5ldyBTZXQ8c3RyaW5nPihbLi4uKGxvY2tzLmFwcGVhcmFuY2UgPyBbXCJvcy5zZXRBcHBlYXJhbmNlXCJdIDogW10pLCAuLi4obG9ja3MudGhlbWVJZCA/IFtcIm9zLnNldFRoZW1lSWRcIl0gOiBbXSksIC4uLihsb2Nrcy5sb2NhbGUgPyBbXCJvcy5zZXRMb2NhbGVcIl0gOiBbXSksIC4uLihsb2Nrcy50ZXJtaW5vbG9neSA/IFtcIm9zLnNldFRlcm1pbm9sb2d5XCJdIDogW10pXSk7XG4gIGNvbnN0IGNvbW1hbmRzOiBDb21tYW5kRGVmaW5pdGlvbltdID0gW1xuICAgIC4uLihoYXNJbnRyb2R1Y3Rpb24gPyBbeyBpZDogXCJvcy5pbnRyb2R1Y2VBcHBcIiwgbGFiZWw6IHNoZWxsTGFiZWwoXCJ1aS5jb21tYW5kLmludHJvZHVjZUFwcFwiKSwgc2NvcGU6IFwib3NcIiBhcyBjb25zdCwgY2F0ZWdvcnk6IFwiYXBwXCIsIGluUGFsZXR0ZTogdHJ1ZSwgYXJnczogW10gfV0gOiBbXSksXG4gICAgLy8g8J+Ope+4jyBgb3MucGxheVR1dG9yaWFsYCBvbmx5IGFwcGVhcnMgb25jZSBhdCBsZWFzdCBvbmUgdHV0b3JpYWwgaXMgZGVjbGFyZWQgKGFwcC1vd24gb3IgYnJhbmQtb3duKTtcbiAgICAvLyBgb3MucmVjb3JkVHV0b3JpYWxgIGlzIGRldi9zdHVkaW8tb25seSAoc2VlIGBpc1R1dG9yaWFsUmVjb3JkZXJBdmFpbGFibGVgKSBhbmQgbmVlZHMgbm8gZGVjbGFyZWRcbiAgICAvLyB0dXRvcmlhbCBhdCBhbGwg4oCUIHJlY29yZGluZyBhbiBhcHAgSVMgdGhlIGF1dGhvcmluZyBwYXRoIGZvciBvbmUuXG4gICAgLi4uKHR1dG9yaWFscy5sZW5ndGggPiAwXG4gICAgICA/IFt7IGlkOiBcIm9zLnBsYXlUdXRvcmlhbFwiLCBsYWJlbDogc2hlbGxMYWJlbChcInVpLmNvbW1hbmQucGxheVR1dG9yaWFsXCIpLCBzY29wZTogXCJvc1wiIGFzIGNvbnN0LCBjYXRlZ29yeTogXCJhcHBcIiwgaW5QYWxldHRlOiB0cnVlLCBhcmdzOiBbc2VsZWN0Q29tbWFuZEFyZyhcInR1dG9yaWFsSWRcIiwgc2hlbGxMYWJlbChcInR1dG9yaWFsLmNoYXB0ZXJcIiksIHR1dG9yaWFscy5tYXAoKHR1dG9yaWFsKSA9PiAoeyB2YWx1ZTogdHV0b3JpYWwuaWQsIGxhYmVsOiByZXNvbHZlTWFuaWZlc3RMYWJlbCh0dXRvcmlhbC50aXRsZSwgdGVybWlub2xvZ3ksIGxvY2FsZSkgfSkpKV0gfV1cbiAgICAgIDogW10pLFxuICAgIC4uLih0dXRvcmlhbFJlY29yZGVyQXZhaWxhYmxlID8gW3sgaWQ6IFwib3MucmVjb3JkVHV0b3JpYWxcIiwgbGFiZWw6IHNoZWxsTGFiZWwoXCJ1aS5jb21tYW5kLnJlY29yZFR1dG9yaWFsXCIpLCBzY29wZTogXCJvc1wiIGFzIGNvbnN0LCBjYXRlZ29yeTogXCJhcHBcIiwgaW5QYWxldHRlOiB0cnVlLCBhcmdzOiBbXSB9XSA6IFtdKSxcbiAgICB7XG4gICAgICBpZDogXCJvcy5zZXRBcHBlYXJhbmNlXCIsXG4gICAgICBsYWJlbDogc2hlbGxMYWJlbChcInVpLmNvbW1hbmQuc2V0QXBwZWFyYW5jZVwiKSxcbiAgICAgIHNjb3BlOiBcIm9zXCIsXG4gICAgICBjYXRlZ29yeTogXCJhcHBlYXJhbmNlXCIsXG4gICAgICBpblBhbGV0dGU6IHRydWUsXG4gICAgICBhcmdzOiBbXG4gICAgICAgIHNlbGVjdENvbW1hbmRBcmcoXCJhcHBlYXJhbmNlXCIsIHNoZWxsTGFiZWwoXCJ1aS5zZXR0aW5ncy50YWIuYXBwZWFyYW5jZVwiKSwgW1xuICAgICAgICAgIHsgdmFsdWU6IFwic3lzdGVtXCIsIGxhYmVsOiBzaGVsbExhYmVsKFwidWkuc2V0dGluZ3MuYXBwZWFyYW5jZS5zeXN0ZW1cIikgfSxcbiAgICAgICAgICB7IHZhbHVlOiBcImxpZ2h0XCIsIGxhYmVsOiBzaGVsbExhYmVsKFwidWkuc2V0dGluZ3MuYXBwZWFyYW5jZS5saWdodFwiKSB9LFxuICAgICAgICAgIHsgdmFsdWU6IFwiZGFya1wiLCBsYWJlbDogc2hlbGxMYWJlbChcInVpLnNldHRpbmdzLmFwcGVhcmFuY2UuZGFya1wiKSB9LFxuICAgICAgICBdKSxcbiAgICAgIF0sXG4gICAgfSxcbiAgICB7XG4gICAgICBpZDogXCJvcy5zZXRUaGVtZUlkXCIsXG4gICAgICBsYWJlbDogc2hlbGxMYWJlbChcInVpLmNvbW1hbmQuc2V0VGhlbWVcIiksXG4gICAgICBzY29wZTogXCJvc1wiLFxuICAgICAgY2F0ZWdvcnk6IFwiYXBwZWFyYW5jZVwiLFxuICAgICAgaW5QYWxldHRlOiB0cnVlLFxuICAgICAgYXJnczogW1xuICAgICAgICBzZWxlY3RDb21tYW5kQXJnKFxuICAgICAgICAgIFwidGhlbWVJZFwiLFxuICAgICAgICAgIHNoZWxsTGFiZWwoXCJ1aS5zZXR0aW5ncy50YWIudGhlbWVcIiksXG4gICAgICAgICAgdGhlbWVMaXN0Lm1hcCgodGhlbWUpID0+ICh7IHZhbHVlOiB0aGVtZS5pZCwgbGFiZWw6IHRoZW1lLmxhYmVsIHx8IHRoZW1lLmlkIH0pKSxcbiAgICAgICAgKSxcbiAgICAgIF0sXG4gICAgfSxcbiAgICB7XG4gICAgICBpZDogXCJvcy5zZXRMYXlvdXRcIixcbiAgICAgIGxhYmVsOiBzaGVsbExhYmVsKFwidWkuY29tbWFuZC5zZXRMYXlvdXRcIiksXG4gICAgICBzY29wZTogXCJvc1wiLFxuICAgICAgY2F0ZWdvcnk6IFwibGF5b3V0XCIsXG4gICAgICBpblBhbGV0dGU6IHRydWUsXG4gICAgICBhcmdzOiBbXG4gICAgICAgIHNlbGVjdENvbW1hbmRBcmcoXCJsYXlvdXRcIiwgc2hlbGxMYWJlbChcInVpLnNldHRpbmdzLnRhYi5sYXlvdXRcIiksIFtcbiAgICAgICAgICB7IHZhbHVlOiBcImRlc2t0b3BcIiwgbGFiZWw6IHNoZWxsTGFiZWwoXCJzZXR0aW5ncy5sYXlvdXQuZGVza3RvcFwiKSB9LFxuICAgICAgICAgIHsgdmFsdWU6IFwidGFibGV0XCIsIGxhYmVsOiBzaGVsbExhYmVsKFwic2V0dGluZ3MubGF5b3V0LnRhYmxldFwiKSB9LFxuICAgICAgICBdKSxcbiAgICAgIF0sXG4gICAgfSxcbiAgICB7IGlkOiBcIm9zLnJlc2V0RG9ja1wiLCBsYWJlbDogc2hlbGxMYWJlbChcInVpLnNldHRpbmdzLnJlc2V0RG9ja1wiKSwgc2NvcGU6IFwib3NcIiwgY2F0ZWdvcnk6IFwibGF5b3V0XCIsIGluUGFsZXR0ZTogdHJ1ZSwgYXJnczogW10gfSxcbiAgICB7XG4gICAgICBpZDogXCJvcy5zZXRMb2NhbGVcIixcbiAgICAgIGxhYmVsOiBzaGVsbExhYmVsKFwidWkuY29tbWFuZC5zZXRMb2NhbGVcIiksXG4gICAgICBzY29wZTogXCJvc1wiLFxuICAgICAgY2F0ZWdvcnk6IFwibGFuZ3VhZ2VcIixcbiAgICAgIGluUGFsZXR0ZTogdHJ1ZSxcbiAgICAgIGFyZ3M6IFtcbiAgICAgICAgc2VsZWN0Q29tbWFuZEFyZyhcImxvY2FsZVwiLCBzaGVsbExhYmVsKFwidWkuc2V0dGluZ3MudGFiLmxhbmd1YWdlXCIpLCBbXG4gICAgICAgICAgeyB2YWx1ZTogXCJlblwiLCBsYWJlbDogc2hlbGxMYWJlbChcInVpLnNldHRpbmdzLmxhbmd1YWdlLmVuXCIpIH0sXG4gICAgICAgICAgeyB2YWx1ZTogXCJkZVwiLCBsYWJlbDogc2hlbGxMYWJlbChcInVpLnNldHRpbmdzLmxhbmd1YWdlLmRlXCIpIH0sXG4gICAgICAgIF0pLFxuICAgICAgXSxcbiAgICB9LFxuICAgIHtcbiAgICAgIGlkOiBcIm9zLnNldFRlcm1pbm9sb2d5XCIsXG4gICAgICBsYWJlbDogc2hlbGxMYWJlbChcInVpLmNvbW1hbmQuc2V0VGVybWlub2xvZ3lcIiksXG4gICAgICBzY29wZTogXCJvc1wiLFxuICAgICAgY2F0ZWdvcnk6IFwibGFuZ3VhZ2VcIixcbiAgICAgIGluUGFsZXR0ZTogdHJ1ZSxcbiAgICAgIGFyZ3M6IFtcbiAgICAgICAgc2VsZWN0Q29tbWFuZEFyZyhcbiAgICAgICAgICBcInRlcm1pbm9sb2d5XCIsXG4gICAgICAgICAgc2hlbGxMYWJlbChcInVpLnNldHRpbmdzLnRhYi50ZXJtaW5vbG9neVwiKSxcbiAgICAgICAgICB0ZXJtaW5vbG9naWVzLm1hcCgoaWQpID0+ICh7IHZhbHVlOiBpZCwgbGFiZWw6IHNoZWxsVGVybWlub2xvZ3lMYWJlbChpZCkgfSkpLFxuICAgICAgICApLFxuICAgICAgXSxcbiAgICB9LFxuICAgIHtcbiAgICAgIGlkOiBcIm9zLnNldERyaXZlclwiLFxuICAgICAgbGFiZWw6IHNoZWxsTGFiZWwoXCJ1aS5jb21tYW5kLnNldERyaXZlclwiKSxcbiAgICAgIHNjb3BlOiBcIm9zXCIsXG4gICAgICBjYXRlZ29yeTogXCJsYXlvdXRcIixcbiAgICAgIGluUGFsZXR0ZTogdHJ1ZSxcbiAgICAgIGFyZ3M6IFtcbiAgICAgICAgc2VsZWN0Q29tbWFuZEFyZyhcbiAgICAgICAgICBcImRyaXZlclwiLFxuICAgICAgICAgIHNoZWxsTGFiZWwoXCJ1aS5zZXR0aW5ncy50YWIuZHJpdmVyXCIpLFxuICAgICAgICAgIGRyaXZlckxpc3QubWFwKChkcml2ZXIpID0+ICh7IHZhbHVlOiBkcml2ZXIuaWQsIGxhYmVsOiBkcml2ZXJEaXNwbGF5TGFiZWwoZHJpdmVyKSB9KSksXG4gICAgICAgICksXG4gICAgICBdLFxuICAgIH0sXG4gIF07XG4gIHJldHVybiBjb21tYW5kcy5maWx0ZXIoKGNvbW1hbmQpID0+ICFsb2NrZWRDb21tYW5kSWRzLmhhcyhjb21tYW5kLmlkKSk7XG59XG5cbi8qKiDwn46b77iPIE9zLXNjb3BlIGNvbW1hbmQgaWRzIHRoYXQgYXJlIGhhbmRsZWQgbG9jYWxseSBieSB0aGUgc2hlbGwg4oCUIG1pcnJvcnMge0BsaW5rIGJ1aWxkT3NDb21tYW5kc30uICovXG5leHBvcnQgZnVuY3Rpb24gZGlzcGF0Y2hPc0NvbW1hbmQoXG4gIGNvbW1hbmRJZDogc3RyaW5nLFxuICBhcmdzOiBSZWNvcmQ8c3RyaW5nLCB1bmtub3duPiB8IHVuZGVmaW5lZCxcbiAgZGlzcGF0Y2g6IChhY3Rpb246IFNoZWxsQWN0aW9uKSA9PiB2b2lkLFxuICBkb2NrTGF5b3V0U3RvcmU6IERvY2tMYXlvdXRTdG9yZSxcbiAgZG9ja1VpU3RhdGVTdG9yZTogRG9ja1VpU3RhdGVTdG9yZSxcbiAgbG9ja3M6IFJlc29sdmVkU2hlbGxMb2NrcyA9IEVNUFRZX1NIRUxMX0xPQ0tTLFxuKTogdm9pZCB7XG4gIHN3aXRjaCAoY29tbWFuZElkKSB7XG4gICAgY2FzZSBcIm9zLmludHJvZHVjZUFwcFwiOlxuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9JTlRST0RVQ1RJT05fU1RFUFwiLCB2YWx1ZTogMCB9KTtcbiAgICAgIHJldHVybjtcbiAgICBjYXNlIFwib3Muc2V0QXBwZWFyYW5jZVwiOlxuICAgICAgaWYgKGxvY2tzLmFwcGVhcmFuY2UpIHJldHVybjtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfVUlfQVBQRUFSQU5DRVwiLCB2YWx1ZTogKGFyZ3M/LmFwcGVhcmFuY2UgYXMgRWxlbWVudHNTdXJmYWNlQXBwZWFyYW5jZSkgPz8gXCJzeXN0ZW1cIiB9KTtcbiAgICAgIHJldHVybjtcbiAgICBjYXNlIFwib3Muc2V0VGhlbWVJZFwiOlxuICAgICAgaWYgKGxvY2tzLnRoZW1lSWQpIHJldHVybjtcbiAgICAgIGlmICh0eXBlb2YgYXJncz8udGhlbWVJZCA9PT0gXCJzdHJpbmdcIikgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9VSV9USEVNRV9JRFwiLCB2YWx1ZTogYXJncy50aGVtZUlkIH0pO1xuICAgICAgcmV0dXJuO1xuICAgIGNhc2UgXCJvcy5zZXRMYXlvdXRcIjpcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfVUlfTEFZT1VUXCIsIHZhbHVlOiAoYXJncz8ubGF5b3V0IGFzIFVpQ2hyb21lTGF5b3V0KSA/PyBcImRlc2t0b3BcIiB9KTtcbiAgICAgIHJldHVybjtcbiAgICBjYXNlIFwib3MucmVzZXREb2NrXCI6XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiUkVTRVRfRE9DS1wiIH0pO1xuICAgICAgZG9ja0xheW91dFN0b3JlLnJlc2V0KCk7XG4gICAgICBkb2NrVWlTdGF0ZVN0b3JlLnJlc2V0KCk7XG4gICAgICByZXR1cm47XG4gICAgY2FzZSBcIm9zLnNldExvY2FsZVwiOlxuICAgICAgaWYgKGxvY2tzLmxvY2FsZSkgcmV0dXJuO1xuICAgICAgaWYgKHR5cGVvZiBhcmdzPy5sb2NhbGUgPT09IFwic3RyaW5nXCIpIHtcbiAgICAgICAgc2V0VWlMb2NhbGUoYXJncy5sb2NhbGUgYXMgVWlMb2NhbGUpO1xuICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1VJX0xPQ0FMRVwiLCB2YWx1ZTogYXJncy5sb2NhbGUgYXMgVWlMb2NhbGUgfSk7XG4gICAgICB9XG4gICAgICByZXR1cm47XG4gICAgY2FzZSBcIm9zLnNldFRlcm1pbm9sb2d5XCI6XG4gICAgICBpZiAobG9ja3MudGVybWlub2xvZ3kpIHJldHVybjtcbiAgICAgIGlmICh0eXBlb2YgYXJncz8udGVybWlub2xvZ3kgPT09IFwic3RyaW5nXCIpIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfVUlfVEVSTUlOT0xPR1lcIiwgdmFsdWU6IGFyZ3MudGVybWlub2xvZ3kgfSk7XG4gICAgICByZXR1cm47XG4gICAgY2FzZSBcIm9zLnNldERyaXZlclwiOlxuICAgICAgaWYgKHR5cGVvZiBhcmdzPy5kcml2ZXIgPT09IFwic3RyaW5nXCIpIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfVUlfRFJJVkVSX0lEXCIsIHZhbHVlOiBhcmdzLmRyaXZlciB9KTtcbiAgICAgIHJldHVybjtcbiAgICBkZWZhdWx0OlxuICAgICAgcmV0dXJuO1xuICB9XG59XG5cbi8qKiBAZW1vamkg8J+Om++4jyBGYWxsYmFjayBpY29uIGZvciBldmVyeSBjb21tYW5kLWNhdGVnb3J5IGxlYWYg4oCUIGNhdGVnb3JpZXMgYXJlIG9wZW4tc2V0IHN0cmluZ3MgYW55IHBsdWdpbi9hcHAvbW9kZSBhdXRob3IgY2FuIGludmVudCwgc28gdGhlcmUncyBubyBwZXItY2F0ZWdvcnkgaWNvbiBtZXRhZGF0YSB0byBrZXkgb2ZmICh1bmxpa2UgdGhlIGZyYW1ld29yaydzIG93biBXb3JrYmVuY2gvRGV0YWlscy9EaXNwbGF5L1NldHRpbmdzIGNhdGVnb3JpZXMpLiAqL1xuY29uc3QgQ09NTUFORF9DQVRFR09SWV9JQ09OID0gc2hlbGxUYWJJY29uKFwid3JlbmNoXCIpO1xuXG4vKipcbiAqIPCfjpvvuI8gT25lIGNhdGVnb3J5J3MgY29tbWFuZCBsaXN0IChhbmQsIGlmIGEgY29tbWFuZCBpcyBleHBhbmRlZCwgaXRzIHN0YWdlZCBhcmcgZm9ybSkgYXMgYSBgVHJlZVBhbmVsQ29uZmlnYFxuICog4oCUIHRoZSBjb250ZW50IGEgY2F0ZWdvcnkgYFBhbmVsVGFiTGVhZmAgcmVzb2x2ZXMgdG8uIEEgemVyby1hcmcgY29tbWFuZCdzIHJvdyBmaXJlcyBpbW1lZGlhdGVseSBvbiBjbGlja1xuICogKGEgcGxhaW4gZmlyZS1hbmQtZm9yZ2V0IHRyZWUgcm93LCBzYW1lIHBhdHRlcm4gYXMge0BsaW5rIGdyb3VwTmFtZWRMYXlvdXRzVG9UcmVlSXRlbXN9J3MgbGF5b3V0IHJvd3Mg4oCUXG4gKiBubyBgc2VsZWN0ZWRJZHNgL2BvblNlbGVjdGlvbkNoYW5nZWAgb3ZlcnJpZGUsIHNvIGl0IHRha2VzIGBUcmVlYCdzIGRlZmF1bHQgc2luZ2xlLXNlbGVjdCBoaWdobGlnaHQgYWZ0ZXJcbiAqIGZpcmluZywgc2FtZSBhcyBjbGlja2luZyBhIERpc3BsYXnihpJMYXlvdXQgcm93IGRvZXMpLiBBbiBhcmctY2FycnlpbmcgY29tbWFuZCdzIHJvdyB0b2dnbGVzIGBleHBhbmRlZENvbW1hbmRJZGBcbiAqIGl0c2VsZiAoa2VwdCBhcyBpdHMgb3duIGV4Y2x1c2l2ZSwgYmVzcG9rZSBzdGF0ZSDigJQgbm90IGBUcmVlYCdzIHBlci1yb3cgYG9wZW5TdGF0ZXNgLCB3aGljaCBpc24ndCBuYXR1cmFsbHlcbiAqIGV4Y2x1c2l2ZSBhY3Jvc3Mgc2libGluZyByb3dzKSBhbmQsIHdoZW4gZXhwYW5kZWQsIGEgc3ludGhldGljIGZvcm0gc2VjdGlvbiAob25lIHJvdyBwZXIgYXJnLCBgY29udHJvbGBcbiAqIGhvbGRpbmcgdGhlIHN0YWdlZCBpbnB1dCwgcmVwbGFjaW5nIHRoZSBvbGQgYEZpZWxkYCB3cmFwcGVyIHNpbmNlIGBUcmVlRGF0YUl0ZW1gIGFscmVhZHkgcmVuZGVycyBsYWJlbCArXG4gKiBkZXNjcmlwdGlvbiArIGNvbnRyb2wgaW4gdGhlIHNhbWUgdHdvLWNvbHVtbiBsYXlvdXQpIGlzIHByZXBlbmRlZCBzbyBpdCByZW5kZXJzIGFib3ZlIHRoZSBjb21tYW5kIGxpc3Qg4oCUXG4gKiBgVHJlZWAgcmV2ZXJzZXMgdG9wLWxldmVsIGBzZWN0aW9uc2AgZm9yIGBkaXJlY3Rpb249XCJ1cFwiYCAoYm90dG9tIGFuY2hvcnMpLCB0aHJlYWRlZCBoZXJlIHZpYSBgZmxvd0Zyb21BbmNob3JgL1xuICogYEZsb3dQcm92aWRlcmAvYHVzZUZsb3dgIGRvd24gZnJvbSB0aGUgaG9zdGluZyBgUGFuZWxgLCBub3QgYW55IG1hbnVhbCByZXZlcnNhbCBpbiB0aGlzIGZ1bmN0aW9uLlxuICovXG5leHBvcnQgZnVuY3Rpb24gYnVpbGRDb21tYW5kQ2F0ZWdvcnlUcmVlKFxuICBjb21tYW5kczogcmVhZG9ubHkgUmVzb2x2ZWRDb21tYW5kW10sXG4gIGV4cGFuZGVkQ29tbWFuZElkOiBzdHJpbmcgfCBudWxsLFxuICBzdGFnZWRBcmdzQnlDb21tYW5kSWQ6IFJlYWRvbmx5PFJlY29yZDxzdHJpbmcsIFJlYWRvbmx5PFJlY29yZDxzdHJpbmcsIHVua25vd24+Pj4+LFxuICBvbkV4ZWN1dGU6IChlbnRyeTogUmVzb2x2ZWRDb21tYW5kLCBleGVjdXRlQXJncz86IFJlY29yZDxzdHJpbmcsIHVua25vd24+KSA9PiB2b2lkLFxuICBvblRvZ2dsZUV4cGFuZGVkOiAoY29tbWFuZElkOiBzdHJpbmcgfCBudWxsKSA9PiB2b2lkLFxuICBvblN0YWdlQXJnOiAoY29tbWFuZElkOiBzdHJpbmcsIGFyZ0lkOiBzdHJpbmcsIHZhbHVlOiB1bmtub3duKSA9PiB2b2lkLFxuICBvblJlc2V0QXJnczogKGNvbW1hbmRJZDogc3RyaW5nKSA9PiB2b2lkLFxuKTogVHJlZVBhbmVsQ29uZmlnIHtcbiAgY29uc3QgYXJnQ2FycnlpbmdDb21tYW5kcyA9IGNvbW1hbmRzLmZpbHRlcigoZW50cnkpID0+IGVudHJ5LmRlZmluaXRpb24uYXJncy5sZW5ndGggPiAwKTtcbiAgY29uc3QgYXV0b0V4cGFuZGVkU2luZ2xldG9uID0gYXJnQ2FycnlpbmdDb21tYW5kcy5sZW5ndGggPT09IDEgPyBhcmdDYXJyeWluZ0NvbW1hbmRzWzBdIDogdW5kZWZpbmVkO1xuICBjb25zdCBleHBhbmRlZCA9IChleHBhbmRlZENvbW1hbmRJZCA/IGNvbW1hbmRzLmZpbmQoKGVudHJ5KSA9PiBlbnRyeS5kZWZpbml0aW9uLmlkID09PSBleHBhbmRlZENvbW1hbmRJZCkgOiB1bmRlZmluZWQpID8/IGF1dG9FeHBhbmRlZFNpbmdsZXRvbjtcbiAgY29uc3QgZWZmZWN0aXZlRXhwYW5kZWRJZCA9IGV4cGFuZGVkPy5kZWZpbml0aW9uLmlkID8/IG51bGw7XG4gIGNvbnN0IHNlY3Rpb25zOiBUcmVlRGF0YVNlY3Rpb25bXSA9IFtdO1xuICBpZiAoZXhwYW5kZWQgJiYgZXhwYW5kZWQuZGVmaW5pdGlvbi5hcmdzLmxlbmd0aCA+IDApIHtcbiAgICBjb25zdCBzdGFnZWQgPSBzdGFnZWRBcmdzQnlDb21tYW5kSWRbZXhwYW5kZWQuZGVmaW5pdGlvbi5pZF0gPz8ge307XG4gICAgY29uc3QgZWZmZWN0aXZlID0gZWZmZWN0aXZlQWN0aW9uQXJncyhleHBhbmRlZC5kZWZpbml0aW9uLmFyZ3MsIHN0YWdlZCk7XG4gICAgY29uc3QgbWlzc2luZyA9IG1pc3NpbmdSZXF1aXJlZEFyZ3MoZXhwYW5kZWQuZGVmaW5pdGlvbi5hcmdzLCBlZmZlY3RpdmUpO1xuICAgIHNlY3Rpb25zLnB1c2goe1xuICAgICAgaWQ6IGBjb21tYW5kLmNhdGVnb3J5LiR7ZXhwYW5kZWQuZGVmaW5pdGlvbi5jYXRlZ29yeX0uZm9ybWAsXG4gICAgICBpdGVtczogZXhwYW5kZWQuZGVmaW5pdGlvbi5hcmdzLm1hcChcbiAgICAgICAgKGRlZik6IFRyZWVEYXRhSXRlbSA9PiAoe1xuICAgICAgICAgIGlkOiBgY29tbWFuZC4ke2V4cGFuZGVkLmRlZmluaXRpb24uaWR9LmFyZy4ke2RlZi5pZH1gLFxuICAgICAgICAgIGxhYmVsOiBkZWYubGFiZWwsXG4gICAgICAgICAgZGVzY3JpcHRpb246IGRlZi5kZXNjcmlwdGlvbixcbiAgICAgICAgICBjb250cm9sOiByZW5kZXJTdGFnZWRBcmdDb250cm9sKGRlZiwgZWZmZWN0aXZlW2RlZi5pZF0sICh2YWx1ZSkgPT4gb25TdGFnZUFyZyhleHBhbmRlZC5kZWZpbml0aW9uLmlkLCBkZWYuaWQsIHZhbHVlKSksXG4gICAgICAgIH0pLFxuICAgICAgKSxcbiAgICAgIGFjdGlvbnM6IFtcbiAgICAgICAge1xuICAgICAgICAgIGlkOiBgY29tbWFuZC0ke2V4cGFuZGVkLmRlZmluaXRpb24uaWR9LWV4ZWN1dGVgLFxuICAgICAgICAgIGljb246IDxJY29uIGljb249XCJjaGVja1wiIHNpemU9XCJzbWFsbFwiIC8+LFxuICAgICAgICAgIHRleHQ6IHNoZWxsTGFiZWwoXCJ1aS5jb21tb24uZXhlY3V0ZVwiKSxcbiAgICAgICAgICBkaXNhYmxlZDogbWlzc2luZy5sZW5ndGggPiAwLFxuICAgICAgICAgIG9uQ2xpY2s6ICgpID0+IG9uRXhlY3V0ZShleHBhbmRlZCwgZWZmZWN0aXZlKSxcbiAgICAgICAgfSxcbiAgICAgICAge1xuICAgICAgICAgIGlkOiBgY29tbWFuZC0ke2V4cGFuZGVkLmRlZmluaXRpb24uaWR9LXJlc2V0YCxcbiAgICAgICAgICBpY29uOiA8SWNvbiBpY29uPVwidW5kb1wiIHNpemU9XCJzbWFsbFwiIC8+LFxuICAgICAgICAgIHRleHQ6IHNoZWxsTGFiZWwoXCJ1aS5jb21tb24ucmVzZXRcIiksXG4gICAgICAgICAgb25DbGljazogKCkgPT4gb25SZXNldEFyZ3MoZXhwYW5kZWQuZGVmaW5pdGlvbi5pZCksXG4gICAgICAgIH0sXG4gICAgICBdLFxuICAgIH0pO1xuICB9XG4gIGNvbnN0IGxpc3RDb21tYW5kcyA9IGNvbW1hbmRzLmZpbHRlcigoZW50cnkpID0+IGVudHJ5LmRlZmluaXRpb24uaWQgIT09IGVmZmVjdGl2ZUV4cGFuZGVkSWQpO1xuICBpZiAobGlzdENvbW1hbmRzLmxlbmd0aCA+IDApIHtcbiAgICBzZWN0aW9ucy5wdXNoKHtcbiAgICAgIGlkOiBcImNvbW1hbmQuY2F0ZWdvcnkubGlzdFwiLFxuICAgICAgaXRlbXM6IGxpc3RDb21tYW5kcy5tYXAoKGVudHJ5KTogVHJlZURhdGFJdGVtID0+IHtcbiAgICAgICAgY29uc3QgYXJnQ2FycnlpbmcgPSBlbnRyeS5kZWZpbml0aW9uLmFyZ3MubGVuZ3RoID4gMDtcbiAgICAgICAgY29uc3QgaWNvbiA9IGVudHJ5LmRlZmluaXRpb24uaWNvbklkID8gPEljb24gaWNvbj17ZW50cnkuZGVmaW5pdGlvbi5pY29uSWQgYXMgSWNvbk5hbWV9IHNpemU9XCJzbWFsbFwiIC8+IDogdW5kZWZpbmVkO1xuICAgICAgICBpZiAoIWFyZ0NhcnJ5aW5nKSByZXR1cm4geyBpZDogYGNvbW1hbmQuJHtlbnRyeS5kZWZpbml0aW9uLmlkfWAsIGxhYmVsOiBlbnRyeS5kZWZpbml0aW9uLmxhYmVsLCBpY29uLCBvbkNsaWNrOiAoKSA9PiBvbkV4ZWN1dGUoZW50cnkpIH07XG4gICAgICAgIHJldHVybiB7XG4gICAgICAgICAgaWQ6IGBjb21tYW5kLiR7ZW50cnkuZGVmaW5pdGlvbi5pZH1gLFxuICAgICAgICAgIGxhYmVsOiBgJHtlbnRyeS5kZWZpbml0aW9uLmxhYmVsfeKApmAsXG4gICAgICAgICAgaWNvbjogPEljb24gaWNvbj17ZXhwYW5kZWRDb21tYW5kSWQgPT09IGVudHJ5LmRlZmluaXRpb24uaWQgPyBcImNoZXZyb24tZG93blwiIDogXCJjaGV2cm9uLXVwXCJ9IHNpemU9XCJzbWFsbFwiIC8+LFxuICAgICAgICAgIG9uQ2xpY2s6ICgpID0+IG9uVG9nZ2xlRXhwYW5kZWQoZXhwYW5kZWRDb21tYW5kSWQgPT09IGVudHJ5LmRlZmluaXRpb24uaWQgPyBudWxsIDogZW50cnkuZGVmaW5pdGlvbi5pZCksXG4gICAgICAgIH07XG4gICAgICB9KSxcbiAgICB9KTtcbiAgfVxuICByZXR1cm4geyBzZWN0aW9ucyB9O1xufVxuXG4vKipcbiAqIPCfjpvvuI8gT25lIGBQYW5lbFRhYkxlYWZgIHBlciByZXNvbHZlZCBjb21tYW5kIGNhdGVnb3J5IOKAlCBjb25zdW1lcnMgd3JhcCB0aGVzZSB1bmRlciB0aGUgQ29tbWFuZCBicmFuY2hcbiAqIChgRlJBTUVXT1JLX0NBVEVHT1JZX0NPTU1BTkRfSURgKSBvbiBgZGVmYXVsdERvY2suYW5jaG9yc1tcImJvdHRvbS1taWRkbGVcIl1gIHNvIHRoZSBmb2xkZWQgY2hyb21lIHNob3dzXG4gKiBhIHNpbmdsZSBleHBhbmRhYmxlIENvbW1hbmQgdG9nZ2xlLiBUaGUgY29tbWFuZCBwYWxldHRlJ3MgZm9sZC9hY3RpdmUtY2F0ZWdvcnkvc2l6ZS9wZXJzaXN0ZW5jZSBpcyB0aGVcbiAqIGdlbmVyaWMgcGVyLWFuY2hvciBgUGFuZWxgIHN0YXRlIChzZWUgYGJ1aWxkUGFuZWxQcm9wc2ApOyB0aGlzIG9ubHkgYnVpbGRzIHRoZSBjYXRlZ29yeSB0YWIgbGVhdmVzLlxuICogQ29udGVudCBpcyBhICpsYXp5KiBgcmVzb2x2ZVRyZWVgIChtaXJyb3JzIHtAbGluayBjcmVhdGVGcmFtZXdvcmtEaXNwbGF5UGFuZWxUYWJzfSdzIHdpbmRvd3MgdGFiKSBzb1xuICogdGhpcyBhcnJheSDigJQgYW5kIHRoZXJlZm9yZSBgZGVmYXVsdERvY2tgJ3Mgb3duIG1lbW8g4oCUIG5ldmVyIGRlcGVuZHMgb24gYGV4cGFuZGVkQ29tbWFuZElkYC9cbiAqIGBzdGFnZWRBcmdzQnlDb21tYW5kSWRgLCB3aGljaCBjaGFuZ2Ugb24gZXZlcnkga2V5c3Ryb2tlIHdoaWxlIHN0YWdpbmcgYSBjb21tYW5kIGFyZ3VtZW50OyBgcmVzb2x2ZVRyZWVgXG4gKiByZWFkcyB0aG9zZSBmcmVzaCBvZmYgcmVmcyBhdCByZW5kZXIgdGltZSBpbnN0ZWFkLlxuICovXG5leHBvcnQgZnVuY3Rpb24gYnVpbGRDb21tYW5kQ2F0ZWdvcnlUYWJzKFxuICByZXNvbHZlZENvbW1hbmRzOiByZWFkb25seSBSZXNvbHZlZENvbW1hbmRbXSxcbiAgY2F0ZWdvcmllczogcmVhZG9ubHkgeyByZWFkb25seSBpZDogc3RyaW5nOyByZWFkb25seSBsYWJlbDogc3RyaW5nIH1bXSxcbiAgZXhwYW5kZWRDb21tYW5kSWRSZWY6IFJlYWN0LlJlZk9iamVjdDxzdHJpbmcgfCBudWxsPixcbiAgc3RhZ2VkQXJnc0J5Q29tbWFuZElkUmVmOiBSZWFjdC5SZWZPYmplY3Q8UmVhZG9ubHk8UmVjb3JkPHN0cmluZywgUmVhZG9ubHk8UmVjb3JkPHN0cmluZywgdW5rbm93bj4+Pj4+LFxuICBvbkNvbW1hbmQ6IChzb3VyY2U6IFJlc29sdmVkQ29tbWFuZFtcInNvdXJjZVwiXSwgY29tbWFuZElkOiBzdHJpbmcsIGFyZ3M/OiBSZWNvcmQ8c3RyaW5nLCB1bmtub3duPikgPT4gdm9pZCxcbiAgZGlzcGF0Y2g6IChhY3Rpb246IFNoZWxsQWN0aW9uKSA9PiB2b2lkLFxuKTogUGFuZWxUYWJOb2RlW10ge1xuICByZXR1cm4gY2F0ZWdvcmllcy5tYXAoKGNhdGVnb3J5KSA9PiB7XG4gICAgY29uc3QgY2F0ZWdvcnlDb21tYW5kcyA9IHJlc29sdmVkQ29tbWFuZHMuZmlsdGVyKChlbnRyeSkgPT4gZW50cnkuZGVmaW5pdGlvbi5jYXRlZ29yeSA9PT0gY2F0ZWdvcnkuaWQpO1xuICAgIHJldHVybiBzaW5nbGVUcmVlTGVhZih7XG4gICAgICBpZDogYGNvbW1hbmQuY2F0ZWdvcnkuJHtjYXRlZ29yeS5pZH1gLFxuICAgICAgaWNvbjogQ09NTUFORF9DQVRFR09SWV9JQ09OLFxuICAgICAgbmFtZTogY2F0ZWdvcnkubGFiZWwsXG4gICAgICB0cmVlOiB7XG4gICAgICAgIHJlc29sdmVUcmVlOiAoKSA9PlxuICAgICAgICAgIGJ1aWxkQ29tbWFuZENhdGVnb3J5VHJlZShcbiAgICAgICAgICAgIGNhdGVnb3J5Q29tbWFuZHMsXG4gICAgICAgICAgICBleHBhbmRlZENvbW1hbmRJZFJlZi5jdXJyZW50LFxuICAgICAgICAgICAgc3RhZ2VkQXJnc0J5Q29tbWFuZElkUmVmLmN1cnJlbnQsXG4gICAgICAgICAgICAoZW50cnksIGV4ZWN1dGVBcmdzKSA9PiBvbkNvbW1hbmQoZW50cnkuc291cmNlLCBlbnRyeS5kZWZpbml0aW9uLmlkLCBleGVjdXRlQXJncyksXG4gICAgICAgICAgICAoY29tbWFuZElkKSA9PiBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0NPTU1BTkRfRVhQQU5ERURcIiwgdmFsdWU6IGNvbW1hbmRJZCB9KSxcbiAgICAgICAgICAgIChjb21tYW5kSWQsIGFyZ0lkLCB2YWx1ZSkgPT4gZGlzcGF0Y2goeyB0eXBlOiBcIlNUQUdFX0NPTU1BTkRfQVJHXCIsIGNvbW1hbmRJZCwgYXJnSWQsIHZhbHVlIH0pLFxuICAgICAgICAgICAgKGNvbW1hbmRJZCkgPT4gZGlzcGF0Y2goeyB0eXBlOiBcIlJFU0VUX0NPTU1BTkRfQVJHU1wiLCBjb21tYW5kSWQgfSksXG4gICAgICAgICAgKSxcbiAgICAgIH0sXG4gICAgfSk7XG4gIH0pO1xufVxuLy8jZW5kcmVnaW9uIPCfjpvvuI9Db21tYW5kUmVnaXN0cnlcblxuLy8jcmVnaW9uIPCfm6DvuI9Ub29sUmVnaXN0cnlcbi8qKlxuICog8J+boO+4jyBPbmUgdG9vbCdzIG1lYXN1cmUtdHJlZSBjb250ZW50LiBTZWxlY3RpbmcgdGhlIHRvb2wgdGFiIGFjdGl2YXRlcyBpdCAoc2VlIGBidWlsZFRvb2xUYWJzYCAvXG4gKiBwYW5lbCBwYXRoIGNoYW5nZSk7IHRoZSB0cmVlIGl0c2VsZiBpcyBhIHNpbmdsZSBoZWFkZXJsZXNzIHNlY3Rpb24gbWFwcGVkIHRvIG5hdGl2ZSBgVHJlZURhdGFJdGVtYHNcbiAqIHNvIEZpbGwgb3BlbnMgZGlyZWN0bHkgb250byBjb3VudCArIGRpc3RyaWJ1dGlvbiB3aXRoIHRoZSBzYW1lIGNocm9tZSBhcyBsZWZ0LWNvcm5lciBwYW5lbCB0cmVlcy5cbiAqL1xuZnVuY3Rpb24gYnVpbGRUb29sVHJlZSh0b29sOiBUb29sRGVmaW5pdGlvbiwgY29udHJvbGxlcklkOiBzdHJpbmcsIGlzQWN0aXZlOiBib29sZWFuLCBtZWFzdXJlczogcmVhZG9ubHkgV2luZG93TWVhc3VyZVtdIHwgdW5kZWZpbmVkLCBvbkFjdGlvbjogKGFjdGlvbjogQWN0aW9uRGVzY3JpcHRvcikgPT4gdW5rbm93bik6IHsgcmVhZG9ubHkgc2VjdGlvbnM6IFRyZWVEYXRhU2VjdGlvbltdOyByZWFkb25seSBzb3J0YWJsZVNlY3Rpb25zOiBmYWxzZSB9IHtcbiAgY29uc3QgaWNvbk5hbWU6IEljb25OYW1lID0gdG9vbC5pY29uSWQgYXMgSWNvbk5hbWU7XG4gIGlmIChpc0FjdGl2ZSAmJiBtZWFzdXJlcyAmJiBtZWFzdXJlcy5sZW5ndGggPiAwKSB7XG4gICAgcmV0dXJuIHtcbiAgICAgIHNvcnRhYmxlU2VjdGlvbnM6IGZhbHNlLFxuICAgICAgc2VjdGlvbnM6IFtcbiAgICAgICAge1xuICAgICAgICAgIGlkOiBgdG9vbC4ke3Rvb2wuaWR9Lm9wdGlvbnNgLFxuICAgICAgICAgIGxhYmVsOiBcIlwiLFxuICAgICAgICAgIGRlZmF1bHRPcGVuOiB0cnVlLFxuICAgICAgICAgIGl0ZW1zOiB3aW5kb3dNZWFzdXJlc1RvVHJlZUl0ZW1zKG1lYXN1cmVzLCBvbkFjdGlvbiksXG4gICAgICAgIH0sXG4gICAgICBdLFxuICAgIH07XG4gIH1cbiAgcmV0dXJuIHtcbiAgICBzb3J0YWJsZVNlY3Rpb25zOiBmYWxzZSxcbiAgICBzZWN0aW9uczogW1xuICAgICAge1xuICAgICAgICBpZDogYHRvb2wuJHt0b29sLmlkfS5hY3RpdmF0ZWAsXG4gICAgICAgIGxhYmVsOiBcIlwiLFxuICAgICAgICBkZWZhdWx0T3BlbjogdHJ1ZSxcbiAgICAgICAgaXRlbXM6IFtcbiAgICAgICAgICB7XG4gICAgICAgICAgICBpZDogYHRvb2wuJHt0b29sLmlkfS5hY3RpdmF0ZS50b2dnbGVgLFxuICAgICAgICAgICAgbGFiZWw6IFwiXCIsXG4gICAgICAgICAgICBjb250cm9sOiAoXG4gICAgICAgICAgICAgIDxUb2dnbGVcbiAgICAgICAgICAgICAgICBpZD17YHRvb2wuJHt0b29sLmlkfWB9XG4gICAgICAgICAgICAgICAgcHJlc3NlZD17aXNBY3RpdmV9XG4gICAgICAgICAgICAgICAgdGV4dD17dG9vbC5sYWJlbH1cbiAgICAgICAgICAgICAgICBpY29uPXs8SWNvbiBpY29uPXtpY29uTmFtZX0gc2l6ZT1cInNtYWxsXCIgLz59XG4gICAgICAgICAgICAgICAgb25QcmVzc2VkQ2hhbmdlPXsocHJlc3NlZCkgPT4gb25BY3Rpb24oeyBjb250cm9sbGVySWQsIGFjdGlvbjogU0VUX0FDVElWRV9UT09MX0FDVElPTl9JRCwgYXJnczogeyB0b29sSWQ6IHByZXNzZWQgPyB0b29sLmlkIDogXCJcIiB9IH0pfVxuICAgICAgICAgICAgICAvPlxuICAgICAgICAgICAgKSxcbiAgICAgICAgICB9LFxuICAgICAgICBdLFxuICAgICAgfSxcbiAgICBdLFxuICB9O1xufVxuXG4vKipcbiAqIPCfm6DvuI8gT25lIGBQYW5lbFRhYkxlYWZgIHBlciByZXNvbHZlZCBtb2RlIHRvb2wg4oCUIGNvbnN1bWVycyB3cmFwIHRoZXNlIHVuZGVyIHRoZSBUb29sIGJyYW5jaFxuICogKGBGUkFNRVdPUktfQ0FURUdPUllfVE9PTF9JRGApIG9uIGBkZWZhdWx0RG9jay5hbmNob3JzW1wiYm90dG9tLW1pZGRsZVwiXWAsIG9yZGVyZWQgbGVmdCBvZiB0aGUgQ29tbWFuZFxuICogYnJhbmNoLCBzbyB0aGUgZm9sZGVkIGNocm9tZSBzaG93cyBhIHNpbmdsZSBUb29sIHRvZ2dsZS4gQ29udGVudCBpcyBhICpsYXp5KiBgcmVzb2x2ZVRyZWVgIChtaXJyb3JzXG4gKiBgYnVpbGRDb21tYW5kQ2F0ZWdvcnlUYWJzYCdzIHdpbmRvd3MgdGFiKSBzbyB0aGlzIGFycmF5IOKAlCBhbmQgdGhlcmVmb3JlIGBkZWZhdWx0RG9ja2AncyBvd24gbWVtbyDigJRcbiAqIG5ldmVyIGRlcGVuZHMgb24gYGFjdGl2ZVRvb2xJZGAvYHRvb2xNZWFzdXJlc0J5VG9vbElkYCwgd2hpY2ggY2hhbmdlIG9uIGV2ZXJ5IGFjdGl2YXRpb24vc2xpZGVyIHRpY2s7XG4gKiBgcmVzb2x2ZVRyZWVgIHJlYWRzIHRob3NlIGZyZXNoIG9mZiByZWZzIGF0IHJlbmRlciB0aW1lIGluc3RlYWQuXG4gKi9cbmV4cG9ydCBmdW5jdGlvbiBidWlsZFRvb2xUYWJzKFxuICB0b29sczogcmVhZG9ubHkgVG9vbERlZmluaXRpb25bXSxcbiAgY29udHJvbGxlcklkOiBzdHJpbmcsXG4gIGFjdGl2ZVRvb2xJZFJlZjogUmVhY3QuUmVmT2JqZWN0PHN0cmluZyB8IG51bGw+LFxuICB0b29sTWVhc3VyZXNCeVRvb2xJZFJlZjogUmVhY3QuUmVmT2JqZWN0PFJlYWRvbmx5PFJlY29yZDxzdHJpbmcsIHJlYWRvbmx5IFdpbmRvd01lYXN1cmVbXT4+PixcbiAgb25BY3Rpb246IChhY3Rpb246IEFjdGlvbkRlc2NyaXB0b3IpID0+IHVua25vd24sXG4pOiBQYW5lbFRhYk5vZGVbXSB7XG4gIHJldHVybiB0b29scy5tYXAoKHRvb2wpID0+XG4gICAgc2luZ2xlVHJlZUxlYWYoe1xuICAgICAgaWQ6IGB0b29sLiR7dG9vbC5pZH1gLFxuICAgICAgaWNvbjogc2hlbGxUYWJJY29uKHRvb2wuaWNvbklkKSxcbiAgICAgIG5hbWU6IHRvb2wubGFiZWwsXG4gICAgICB0cmVlOiB7XG4gICAgICAgIHJlc29sdmVUcmVlOiAoKSA9PiB7XG4gICAgICAgICAgY29uc3QgdHJlZSA9IGJ1aWxkVG9vbFRyZWUodG9vbCwgY29udHJvbGxlcklkLCBhY3RpdmVUb29sSWRSZWYuY3VycmVudCA9PT0gdG9vbC5pZCwgdG9vbE1lYXN1cmVzQnlUb29sSWRSZWYuY3VycmVudFt0b29sLmlkXSwgb25BY3Rpb24pO1xuICAgICAgICAgIHJldHVybiB7IHNlY3Rpb25zOiB0cmVlLnNlY3Rpb25zLCBzb3J0YWJsZVNlY3Rpb25zOiB0cmVlLnNvcnRhYmxlU2VjdGlvbnMgfTtcbiAgICAgICAgfSxcbiAgICAgIH0sXG4gICAgfSksXG4gICk7XG59XG5cbi8qKiDwn5ug77iPIEFjdGl2YXRlcyB0aGUgbW9kZSB0b29sIHdob3NlIGZvb3RlciB0YWIgd2FzIGp1c3Qgc2VsZWN0ZWQgKGB0b29sLjxpZD5gKSwgbWlycm9yaW5nIHV0aWxpdHktYmFyIHByZXNzIOKGkiBvcHRpb25zLiAqL1xuZXhwb3J0IGZ1bmN0aW9uIHRvb2xJZEZyb21QYW5lbFRhYklkKHRhYklkOiBzdHJpbmcgfCB1bmRlZmluZWQpOiBzdHJpbmcgfCBudWxsIHtcbiAgaWYgKCF0YWJJZD8uc3RhcnRzV2l0aChcInRvb2wuXCIpKSByZXR1cm4gbnVsbDtcbiAgY29uc3QgdG9vbElkID0gdGFiSWQuc2xpY2UoXCJ0b29sLlwiLmxlbmd0aCk7XG4gIHJldHVybiB0b29sSWQubGVuZ3RoID4gMCA/IHRvb2xJZCA6IG51bGw7XG59XG4vLyNlbmRyZWdpb24g8J+boO+4j1Rvb2xSZWdpc3RyeVxuXG4vKiogQGVtb2ppIPCfkKLvuI8gU3RydWN0dXJhbCBlcXVhbGl0eSBvdmVyIHBsYWluIEpTT04tc2hhcGVkIHZhbHVlcyAodGhlIHNoYXBlIGV2ZXJ5IGBVaU5vZGVgL2BXaW5kb3dFbmdhZ2VtZW50YC9gV2luZG93TWVhc3VyZWAgcHJvZ3JhbSBwYXlsb2FkIHRha2VzKSDigJQgbm8gY3ljbGVzLCBubyBub24tSlNPTiB0eXBlcy4gKi9cbmZ1bmN0aW9uIHVpSnNvbkRlZXBFcXVhbChhOiB1bmtub3duLCBiOiB1bmtub3duKTogYm9vbGVhbiB7XG4gIGlmIChhID09PSBiKSByZXR1cm4gdHJ1ZTtcbiAgaWYgKHR5cGVvZiBhICE9PSBcIm9iamVjdFwiIHx8IHR5cGVvZiBiICE9PSBcIm9iamVjdFwiIHx8IGEgPT09IG51bGwgfHwgYiA9PT0gbnVsbCkgcmV0dXJuIGZhbHNlO1xuICBpZiAoQXJyYXkuaXNBcnJheShhKSAhPT0gQXJyYXkuaXNBcnJheShiKSkgcmV0dXJuIGZhbHNlO1xuICBpZiAoQXJyYXkuaXNBcnJheShhKSAmJiBBcnJheS5pc0FycmF5KGIpKSB7XG4gICAgaWYgKGEubGVuZ3RoICE9PSBiLmxlbmd0aCkgcmV0dXJuIGZhbHNlO1xuICAgIGZvciAobGV0IGluZGV4ID0gMDsgaW5kZXggPCBhLmxlbmd0aDsgaW5kZXggKz0gMSkge1xuICAgICAgaWYgKCF1aUpzb25EZWVwRXF1YWwoYVtpbmRleF0sIGJbaW5kZXhdKSkgcmV0dXJuIGZhbHNlO1xuICAgIH1cbiAgICByZXR1cm4gdHJ1ZTtcbiAgfVxuICBjb25zdCBhUmVjb3JkID0gYSBhcyBSZWNvcmQ8c3RyaW5nLCB1bmtub3duPjtcbiAgY29uc3QgYlJlY29yZCA9IGIgYXMgUmVjb3JkPHN0cmluZywgdW5rbm93bj47XG4gIGNvbnN0IGFLZXlzID0gT2JqZWN0LmtleXMoYVJlY29yZCk7XG4gIGNvbnN0IGJLZXlzID0gT2JqZWN0LmtleXMoYlJlY29yZCk7XG4gIGlmIChhS2V5cy5sZW5ndGggIT09IGJLZXlzLmxlbmd0aCkgcmV0dXJuIGZhbHNlO1xuICBmb3IgKGNvbnN0IGtleSBvZiBhS2V5cykge1xuICAgIGlmICghT2JqZWN0LnByb3RvdHlwZS5oYXNPd25Qcm9wZXJ0eS5jYWxsKGJSZWNvcmQsIGtleSkpIHJldHVybiBmYWxzZTtcbiAgICBpZiAoIXVpSnNvbkRlZXBFcXVhbChhUmVjb3JkW2tleV0sIGJSZWNvcmRba2V5XSkpIHJldHVybiBmYWxzZTtcbiAgfVxuICByZXR1cm4gdHJ1ZTtcbn1cblxuLyoqXG4gKiBAZW1vamkg8J+Qou+4jyBSZXVzZXMgYHByZXZpb3VzYCdzIG9iamVjdCBpZGVudGl0eSB3aGVuIGl0J3Mgc3RydWN0dXJhbGx5IGVxdWFsIHRvIGBuZXh0YCDigJQgZXZlcnkgcHJvZ3JhbVxuICogYHJlbmRlcigpYC9gdXRpbGl0aWVzKClgL2B3aW5kb3dFbmdhZ2VtZW50cygpYC9gd2luZG93TWVhc3VyZXMoKWAgY2FsbCByZS1wYXJzZXMgYSBmcmVzaCBKU09OIHBheWxvYWRcbiAqIGV2ZXJ5IHRpbWUsIGV2ZW4gd2hlbiBub3RoaW5nIGFib3V0IHRoYXQgYm9keSBhY3R1YWxseSBjaGFuZ2VkIChlLmcuIGEgY2FtZXJhLW9ubHkgb3Igc2VsZWN0aW9uLW9ubHlcbiAqIGFjdGlvbiBzdGlsbCByZXR1cm5zIGJ5dGUtaWRlbnRpY2FsIHBhbmVsL3V0aWxpdHkgSlNPTikuIFdpdGhvdXQgdGhpcywgZXZlcnkgZG93bnN0cmVhbSBgUmVhY3QubWVtb2BcbiAqIChzZWUgYEludGVycHJldGVkVWlOb2RlYCkgc2VlcyBhIG5ldyBwcm9wIHJlZmVyZW5jZSBldmVyeSByZW5kZXIgYW5kIGNhbiBuZXZlciBiYWlsLlxuICovXG5leHBvcnQgZnVuY3Rpb24gcHJlc2VydmVKc29uSWRlbnRpdHk8VD4ocHJldmlvdXM6IFQgfCB1bmRlZmluZWQsIG5leHQ6IFQpOiBUIHtcbiAgcmV0dXJuIHByZXZpb3VzICE9PSB1bmRlZmluZWQgJiYgdWlKc29uRGVlcEVxdWFsKHByZXZpb3VzLCBuZXh0KSA/IHByZXZpb3VzIDogbmV4dDtcbn1cblxuLyoqXG4gKiBAZW1vamkg8J+Qou+4jyBCdWlsZHMgYSBgUmVjb3JkPHN0cmluZywgVj5gIGZyb20gYGVudHJpZXNgLCByZXVzaW5nIGBwcmV2YCdzIHBlci1rZXkgdmFsdWUgcmVmZXJlbmNlIHdoZXJlXG4gKiBgcHJlc2VydmVKc29uSWRlbnRpdHlgIGZpbmRzIG5vIHN0cnVjdHVyYWwgY2hhbmdlLCBhbmQgcmV1c2luZyBgcHJldmAgaXRzZWxmICh0aGUgd2hvbGUgcmVjb3JkKSB3aGVuXG4gKiBubyBrZXkgYWN0dWFsbHkgY2hhbmdlZCDigJQgc28gYSBuby1vcGVyYXRpb24gYWN0aW9uJ3MgYGRpc3BhdGNoYCBkb2Vzbid0IGhhbmQgYHdpbmRvd1VpQnlXaW5kb3dJZGAvZXRjLiBhIG5ld1xuICogb2JqZWN0IHJlZmVyZW5jZSBhbmQgY2FzY2FkZSBhbiB1bm1lbW9pemFibGUgcmUtcmVuZGVyIHRocm91Z2ggZXZlcnkgZG93bnN0cmVhbSBjb25zdW1lci5cbiAqL1xuZXhwb3J0IGZ1bmN0aW9uIG1lcmdlUmVjb3JkUHJlc2VydmluZ0lkZW50aXR5PFY+KHByZXY6IFJlYWRvbmx5PFJlY29yZDxzdHJpbmcsIFY+PiwgZW50cmllczogcmVhZG9ubHkgKHJlYWRvbmx5IFtzdHJpbmcsIFZdKVtdKTogUmVhZG9ubHk8UmVjb3JkPHN0cmluZywgVj4+IHtcbiAgY29uc3QgbmV4dDogUmVjb3JkPHN0cmluZywgVj4gPSB7fTtcbiAgbGV0IGNoYW5nZWQgPSBPYmplY3Qua2V5cyhwcmV2KS5sZW5ndGggIT09IGVudHJpZXMubGVuZ3RoO1xuICBmb3IgKGNvbnN0IFtrZXksIHZhbHVlXSBvZiBlbnRyaWVzKSB7XG4gICAgY29uc3QgcHJlc2VydmVkID0gcHJlc2VydmVKc29uSWRlbnRpdHkocHJldltrZXldLCB2YWx1ZSk7XG4gICAgbmV4dFtrZXldID0gcHJlc2VydmVkO1xuICAgIGlmIChwcmVzZXJ2ZWQgIT09IHByZXZba2V5XSkgY2hhbmdlZCA9IHRydWU7XG4gIH1cbiAgcmV0dXJuIGNoYW5nZWQgPyBuZXh0IDogcHJldjtcbn1cblxuLyoqIEBlbW9qaSDwn46v77iPIE1lcmdlcyBzZWxlY3Rpb24gY2hyb21lIGludG8gYW4gZXhpc3Rpbmcgd29ybGQtM2QgY29tcG9uZW50IHNjZW5lIHdpdGhvdXQgdG91Y2hpbmcgaW5zdGFuY2UgZ2VvbWV0cnkuICovXG5leHBvcnQgZnVuY3Rpb24gcGF0Y2hXb3JsZDNkQ2hyb21lT250b05vZGUobm9kZTogVWlOb2RlLCBwYXRjaDogeyByZWFkb25seSBzZWxlY3Rpb25Kc29uOiBzdHJpbmc7IHJlYWRvbmx5IHZvcnRpY2VzSnNvbj86IHN0cmluZyB9KTogVWlOb2RlIHtcbiAgaWYgKG5vZGUudHlwZSAhPT0gXCJjb21wb25lbnRcIiB8fCAhbm9kZS53b3JsZDNkKSByZXR1cm4gbm9kZTtcbiAgY29uc3QgbmV4dDogVWlOb2RlID0ge1xuICAgIC4uLm5vZGUsXG4gICAgd29ybGQzZDoge1xuICAgICAgLi4ubm9kZS53b3JsZDNkLFxuICAgICAgc2VsZWN0aW9uSnNvbjogcGF0Y2guc2VsZWN0aW9uSnNvbixcbiAgICAgIC4uLihwYXRjaC52b3J0aWNlc0pzb24gIT09IHVuZGVmaW5lZCA/IHsgdm9ydGljZXNKc29uOiBwYXRjaC52b3J0aWNlc0pzb24gfSA6IHt9KSxcbiAgICB9LFxuICB9O1xuICByZXR1cm4gcHJlc2VydmVKc29uSWRlbnRpdHkobm9kZSwgbmV4dCk7XG59XG5cbi8qKiBAZW1vamkg8J+Msu+4jyBVcGRhdGVzIHRyZWUtbGV2ZWwgc2VsZWN0aW9uIGhpZ2hsaWdodHMgd2l0aG91dCByZWJ1aWxkaW5nIHN0cnVjdHVyYWwgc2VjdGlvbnMuICovXG5leHBvcnQgZnVuY3Rpb24gcGF0Y2hEb2N1bWVudFRyZWVTZWxlY3RlZElkcyhub2RlOiBVaU5vZGUsIHNlbGVjdGVkSWRzOiByZWFkb25seSBzdHJpbmdbXSwgaGlnaGxpZ2h0ZWRJZHM/OiByZWFkb25seSBzdHJpbmdbXSk6IFVpTm9kZSB7XG4gIGlmIChub2RlLnR5cGUgIT09IFwidHJlZVwiKSByZXR1cm4gbm9kZTtcbiAgY29uc3QgbmV4dDogVWlOb2RlID0ge1xuICAgIC4uLm5vZGUsXG4gICAgc2VsZWN0ZWRJZHM6IFsuLi5zZWxlY3RlZElkc10sXG4gICAgLi4uKGhpZ2hsaWdodGVkSWRzID8geyBoaWdobGlnaHRlZElkczogWy4uLmhpZ2hsaWdodGVkSWRzXSB9IDoge30pLFxuICB9O1xuICByZXR1cm4gcHJlc2VydmVKc29uSWRlbnRpdHkobm9kZSwgbmV4dCk7XG59XG5cbi8vI3JlZ2lvbiBVaVJlZnJlc2hcbi8qKiBAZW1vamkg8J+Qou+4jyBPbmUgY2FjaGVkIHNlY3Rpb24gdmFsdWUga2V5ZWQgYnkgYCR7c2VjdGlvbn06JHtrZXl9YCAoZS5nLiBgd2luZG93OjJkLW92ZXJ2aWV3YCwgYGVuZ2FnZW1lbnRzYCkg4oCUIHRoZSBoYXNoIGlzIHdoYXQgZ2V0cyBzZW50IGJhY2sgdG8gdGhlIHBsdWdpbiBuZXh0IHRpbWUgc28gaXQgY2FuIHNraXAgcmUtc2VyaWFsaXppbmcgdW5jaGFuZ2VkIGNvbnRlbnQuICovXG5leHBvcnQgdHlwZSBVaVJlZnJlc2hDYWNoZSA9IE1hcDxzdHJpbmcsIHsgcmVhZG9ubHkgaGFzaDogc3RyaW5nOyByZWFkb25seSB2YWx1ZTogdW5rbm93biB9PjtcblxuZnVuY3Rpb24gdWlSZWZyZXNoV2FudHNXaW5kb3coc2NvcGU6IFVpRGlydHlTY29wZSwgYm9keUtleTogc3RyaW5nKTogYm9vbGVhbiB7XG4gIHJldHVybiBzY29wZS5raW5kID09PSBcImZ1bGxcIiB8fCAoc2NvcGUua2luZCA9PT0gXCJwYXJ0aWFsXCIgJiYgKHNjb3BlLndpbmRvd0JvZGllcyA/PyBbXSkuaW5jbHVkZXMoYm9keUtleSkpO1xufVxuZnVuY3Rpb24gdWlSZWZyZXNoV2FudHNQYW5lbChzY29wZTogVWlEaXJ0eVNjb3BlLCBib2R5S2V5OiBzdHJpbmcpOiBib29sZWFuIHtcbiAgcmV0dXJuIHNjb3BlLmtpbmQgPT09IFwiZnVsbFwiIHx8IChzY29wZS5raW5kID09PSBcInBhcnRpYWxcIiAmJiAoc2NvcGUucGFuZWxCb2RpZXMgPz8gW10pLmluY2x1ZGVzKGJvZHlLZXkpKTtcbn1cbmZ1bmN0aW9uIHVpUmVmcmVzaFdhbnRzRmxhZyhzY29wZTogVWlEaXJ0eVNjb3BlLCBmbGFnOiBcImVuZ2FnZW1lbnRzXCIgfCBcIm1lYXN1cmVzXCIgfCBcInRvb2xzXCIgfCBcImxhYmVsc1wiKTogYm9vbGVhbiB7XG4gIHJldHVybiBzY29wZS5raW5kID09PSBcImZ1bGxcIiB8fCAoc2NvcGUua2luZCA9PT0gXCJwYXJ0aWFsXCIgJiYgc2NvcGVbZmxhZ10gPT09IHRydWUpO1xufVxuXG4vKipcbiAqIPCfqp/vuI8gRXZlcnkgbGl2ZSB3aW5kb3cgaW5zdGFuY2UgZm9yIGEgc2Vzc2lvbiDigJQgb25lIHBlciBiYXNlIGBBcHBEZWZpbml0aW9uLndpbmRvd0tpbmRzYCBlbnRyeSAoaWQgPT1cbiAqIGtpbmQgaWQpIHBsdXMgb25lIHBlciBzcGxpdC9zcGF3bmVkIGV4dHJhIOKAlCBzbyBgcmVmcmVzaFVpYCBmZXRjaGVzIGFuZCB0aGUgcGx1Z2luIHJldHVybnMgc3RhdGUgZm9yXG4gKiBldmVyeSBhY3R1YWwgd2luZG93LCBuZXZlciBjb2xsYXBzaW5nIHR3byBzYW1lLWtpbmQgaW5zdGFuY2VzIChlLmcuIHNwbGl0IHRvcC9wZXJzcGVjdGl2ZSBwYW5lcykgb250b1xuICogb25lIHNoYXJlZCBlbnRyeS5cbiAqL1xuZXhwb3J0IGZ1bmN0aW9uIHNlc3Npb25XaW5kb3dJbnN0YW5jZXMoXG4gIGFwcDogeyByZWFkb25seSB3aW5kb3dLaW5kczogcmVhZG9ubHkgeyByZWFkb25seSBpZDogc3RyaW5nOyByZWFkb25seSBib2R5S2V5OiBzdHJpbmcgfVtdIH0sXG4gIGV4dHJhV2luZG93SW5zdGFuY2VzOiByZWFkb25seSBFeHRyYVdpbmRvd0luc3RhbmNlW10sXG4pOiByZWFkb25seSB7IHJlYWRvbmx5IGlkOiBzdHJpbmc7IHJlYWRvbmx5IGJvZHlLZXk6IHN0cmluZzsgcmVhZG9ubHkgd2luZG93S2luZElkOiBzdHJpbmcgfVtdIHtcbiAgY29uc3Qga2luZEJ5SWQgPSBuZXcgTWFwKGFwcC53aW5kb3dLaW5kcy5tYXAoKGtpbmQpID0+IFtraW5kLmlkLCBraW5kXSBhcyBjb25zdCkpO1xuICBjb25zdCBiYXNlID0gYXBwLndpbmRvd0tpbmRzLm1hcCgoa2luZCkgPT4gKHsgaWQ6IGtpbmQuaWQsIGJvZHlLZXk6IGtpbmQuYm9keUtleSwgd2luZG93S2luZElkOiBraW5kLmlkIH0pKTtcbiAgY29uc3QgZXh0cmEgPSBleHRyYVdpbmRvd0luc3RhbmNlcy5mbGF0TWFwKChpbnN0YW5jZSkgPT4ge1xuICAgIGNvbnN0IGtpbmQgPSBraW5kQnlJZC5nZXQoaW5zdGFuY2Uud2luZG93S2luZElkKTtcbiAgICByZXR1cm4ga2luZCA/IFt7IGlkOiBpbnN0YW5jZS5pZCwgYm9keUtleToga2luZC5ib2R5S2V5LCB3aW5kb3dLaW5kSWQ6IGluc3RhbmNlLndpbmRvd0tpbmRJZCB9XSA6IFtdO1xuICB9KTtcbiAgcmV0dXJuIFsuLi5iYXNlLCAuLi5leHRyYV07XG59XG5cbi8qKiDwn46T77iPIEtpbmQtbGV2ZWwgaW50cm9kdWN0aW9uIHRhcmdldHMgbXVzdCBhbHNvIG1hdGNoIGxpdmUgd2luZG93ICppbnN0YW5jZXMqIG9mIHRoYXQga2luZFxuICogKGBwdXp6bGUzZC1tYWluLXRvcGAgLyBgcHV6emxlM2QtbWFpbi1wZXJzcGVjdGl2ZWAgZm9yIGtpbmQgYHB1enpsZTNkLW1haW5gKSDigJQgb3RoZXJ3aXNlIGZvcmNlLXVuZm9sZFxuICogb2YgdGhlIHV0aWxpdHkgYmFyIC8gQWN0aW9ucyByYWlsIG5ldmVyIHJlYWNoZXMgdGhlIHBhbmVzIHRoZSB1c2VyIGFjdHVhbGx5IHNlZXMuIGB0YXJnZXRLaW5kSWRgIGlzIGFcbiAqIHJhdyB3aW5kb3cta2luZCBpZDsgYHRhcmdldFNlZ21lbnRgIGlzIGFuIGFscmVhZHktbm9ybWFsaXplZCBgZWxlbWVudElkU2VnbWVudGAgKGUuZy4gZnJvbSBhXG4gKiBgZnJhbWV3b3JrLndpbmRvdy57c2VnbWVudH0uYWN0aW9uLipgIGludHJvZHVjZSBpZCkuICovXG5leHBvcnQgZnVuY3Rpb24gaW50cm9kdWN0aW9uVGFyZ2V0c1dpbmRvdyhcbiAgd2luZG93SWQ6IHN0cmluZyxcbiAgd2luZG93S2luZElkOiBzdHJpbmcsXG4gIHRhcmdldEtpbmRJZDogc3RyaW5nIHwgbnVsbCxcbiAgdGFyZ2V0U2VnbWVudDogc3RyaW5nIHwgbnVsbCA9IG51bGwsXG4pOiBib29sZWFuIHtcbiAgaWYgKHRhcmdldEtpbmRJZCAmJiAoZWxlbWVudElkU2VnbWVudCh3aW5kb3dJZCkgPT09IGVsZW1lbnRJZFNlZ21lbnQodGFyZ2V0S2luZElkKSB8fCBlbGVtZW50SWRTZWdtZW50KHdpbmRvd0tpbmRJZCkgPT09IGVsZW1lbnRJZFNlZ21lbnQodGFyZ2V0S2luZElkKSkpIHJldHVybiB0cnVlO1xuICBpZiAodGFyZ2V0U2VnbWVudCAmJiAoZWxlbWVudElkU2VnbWVudCh3aW5kb3dJZCkgPT09IHRhcmdldFNlZ21lbnQgfHwgZWxlbWVudElkU2VnbWVudCh3aW5kb3dLaW5kSWQpID09PSB0YXJnZXRTZWdtZW50KSkgcmV0dXJuIHRydWU7XG4gIHJldHVybiBmYWxzZTtcbn1cblxuLyoqIEBlbW9qaSDwn6ew77iPIE1hdGVyaWFsaXplcyB0aGUgc2hlbGwncyBwZXItd2luZG93IHV0aWxpdHkgbWFwIGZvciBiYXRjaGVkIGByZWZyZXNoLXVpYCDigJQgb21pdHMgbnVsbCBlbnRyaWVzLiAqL1xuZXhwb3J0IGZ1bmN0aW9uIGJ1aWxkQWN0aXZlVXRpbGl0eUJ5V2luZG93SWQoYWN0aXZlVXRpbGl0eUJ5V2luZG93SWQ6IFJlYWRvbmx5PFJlY29yZDxzdHJpbmcsIHN0cmluZyB8IG51bGw+Pik6IFJlY29yZDxzdHJpbmcsIHN0cmluZz4ge1xuICByZXR1cm4gT2JqZWN0LmZyb21FbnRyaWVzKE9iamVjdC5lbnRyaWVzKGFjdGl2ZVV0aWxpdHlCeVdpbmRvd0lkKS5mbGF0TWFwKChbd2luZG93SWQsIHV0aWxpdHlJZF0pID0+ICh1dGlsaXR5SWQgPyBbW3dpbmRvd0lkLCB1dGlsaXR5SWRdXSA6IFtdKSkpO1xufVxuXG4vKipcbiAqIEBlbW9qaSDwn5Ci77iPIEJ1aWxkcyBvbmUgYmF0Y2hlZCBgcmVmcmVzaC11aWAgcmVxdWVzdCByZXN0cmljdGVkIHRvIGBzY29wZWAg4oCUIGBudWxsYCB3aGVuIHRoZSBzY29wZVxuICogcmVzb2x2ZXMgdG8gbm90aGluZyB3b3J0aCBmZXRjaGluZyAoYG5vbmVgLCBvciBhIGBwYXJ0aWFsYCB3aG9zZSBmaWVsZHMgYWxsIG1pc3MgdGhpcyBhcHAncyBhY3R1YWxcbiAqIGJvZGllcy9pbnN0YW5jZXMpLiBFdmVyeSByZXF1ZXN0ZWQgZW50cnkgY2FycmllcyB0aGUgaG9zdCdzIGNhY2hlZCBoYXNoIHNvIHRoZSBwbHVnaW4gY2FuIG9taXQgcGF5bG9hZHNcbiAqIGZvciBzZWN0aW9ucyB0aGF0IGRpZG4ndCBjaGFuZ2UuIGB3aW5kb3dJbnN0YW5jZXNgIGlzIGtleWVkIGJ5IHdpbmRvdyBJTlNUQU5DRSBpZCAoYmFzZSB3aW5kb3dzIHBsdXMgYW55XG4gKiBzcGxpdC9zcGF3bmVkIGV4dHJhcykg4oCUIG5ldmVyIGJ5IHdpbmRvdyBraW5kIOKAlCBzbyB0d28gaW5zdGFuY2VzIG9mIHRoZSBzYW1lIGtpbmQgZ2V0IGluZGVwZW5kZW50XG4gKiBjYWNoZSBlbnRyaWVzIGFuZCBpbmRlcGVuZGVudCByZW5kZXJlZCBib2RpZXMuXG4gKi9cbmV4cG9ydCBmdW5jdGlvbiBidWlsZFVpUmVmcmVzaFJlcXVlc3QoXG4gIHNjb3BlOiBVaURpcnR5U2NvcGUsXG4gIHdpbmRvd0luc3RhbmNlczogcmVhZG9ubHkgeyByZWFkb25seSBpZDogc3RyaW5nOyByZWFkb25seSBib2R5S2V5OiBzdHJpbmcgfVtdLFxuICBwYW5lbFRhYkxlYXZlczogcmVhZG9ubHkgeyByZWFkb25seSBraW5kOiBQYW5lbFRhYktpbmQ7IHJlYWRvbmx5IGJvZHlLZXk/OiBzdHJpbmcgfVtdLFxuICB2aWV3U3RhdGU6IFBsdWdpblZpZXdTdGF0ZSxcbiAgY2FjaGU6IFVpUmVmcmVzaENhY2hlLFxuKTogUGx1Z2luVWlSZWZyZXNoUmVxdWVzdCB8IG51bGwge1xuICBpZiAoc2NvcGUua2luZCA9PT0gXCJub25lXCIpIHJldHVybiBudWxsO1xuICBjb25zdCB3aW5kb3dzID0gd2luZG93SW5zdGFuY2VzLmZpbHRlcigoaW5zdGFuY2UpID0+IHVpUmVmcmVzaFdhbnRzV2luZG93KHNjb3BlLCBpbnN0YW5jZS5ib2R5S2V5KSkubWFwKChpbnN0YW5jZSkgPT4gKHsga2V5OiBpbnN0YW5jZS5pZCwgYm9keUtleTogaW5zdGFuY2UuYm9keUtleSwgaGFzaDogY2FjaGUuZ2V0KGB3aW5kb3c6JHtpbnN0YW5jZS5pZH1gKT8uaGFzaCB9KSk7XG4gIGNvbnN0IHBhbmVscyA9IHBhbmVsVGFiTGVhdmVzXG4gICAgLmZpbHRlcigodGFiKTogdGFiIGlzIHsgcmVhZG9ubHkga2luZDogc3RyaW5nOyByZWFkb25seSBib2R5S2V5OiBzdHJpbmcgfSA9PiBCb29sZWFuKHRhYi5ib2R5S2V5KSAmJiB1aVJlZnJlc2hXYW50c1BhbmVsKHNjb3BlLCB0YWIuYm9keUtleSEpKVxuICAgIC5tYXAoKHRhYikgPT4gKHsga2V5OiBwYW5lbFRhYktpbmRJZCh0YWIua2luZCksIGJvZHlLZXk6IHRhYi5ib2R5S2V5LCBoYXNoOiBjYWNoZS5nZXQoYHBhbmVsOiR7cGFuZWxUYWJLaW5kSWQodGFiLmtpbmQpfWApPy5oYXNoIH0pKTtcbiAgY29uc3QgZW5nYWdlbWVudHMgPSB1aVJlZnJlc2hXYW50c0ZsYWcoc2NvcGUsIFwiZW5nYWdlbWVudHNcIikgPyB7IGhhc2g6IGNhY2hlLmdldChcImVuZ2FnZW1lbnRzXCIpPy5oYXNoIH0gOiB1bmRlZmluZWQ7XG4gIGNvbnN0IG1lYXN1cmVzID0gdWlSZWZyZXNoV2FudHNGbGFnKHNjb3BlLCBcIm1lYXN1cmVzXCIpID8geyBoYXNoOiBjYWNoZS5nZXQoXCJtZWFzdXJlc1wiKT8uaGFzaCB9IDogdW5kZWZpbmVkO1xuICBjb25zdCB0b29scyA9IHVpUmVmcmVzaFdhbnRzRmxhZyhzY29wZSwgXCJ0b29sc1wiKSA/IHsgaGFzaDogY2FjaGUuZ2V0KFwidG9vbHNcIik/Lmhhc2ggfSA6IHVuZGVmaW5lZDtcbiAgY29uc3QgbGFiZWxzID0gdWlSZWZyZXNoV2FudHNGbGFnKHNjb3BlLCBcImxhYmVsc1wiKSA/IHsgaGFzaDogY2FjaGUuZ2V0KFwibGFiZWxzXCIpPy5oYXNoIH0gOiB1bmRlZmluZWQ7XG4gIGlmICh3aW5kb3dzLmxlbmd0aCA9PT0gMCAmJiBwYW5lbHMubGVuZ3RoID09PSAwICYmICFlbmdhZ2VtZW50cyAmJiAhbWVhc3VyZXMgJiYgIXRvb2xzICYmICFsYWJlbHMpIHJldHVybiBudWxsO1xuICByZXR1cm4geyB2aWV3U3RhdGUsIHdpbmRvd3MsIHBhbmVscywgZW5nYWdlbWVudHMsIG1lYXN1cmVzLCB0b29scywgbGFiZWxzIH07XG59XG5cbi8qKiBAZW1vamkg8J+Qou+4jyBXcml0ZXMgZXZlcnkgY2hhbmdlZCBzZWN0aW9uIChgdmFsdWUgIT09IHVuZGVmaW5lZGApIGZyb20gYSBgcmVmcmVzaC11aWAgcmVzcG9uc2UgaW50byBgY2FjaGVgOyB1bmNoYW5nZWQgc2VjdGlvbnMgYXJlIGxlZnQgYXMtaXMgc2luY2UgdGhlIGNhY2hlZCB2YWx1ZSBpcyBzdGlsbCBjdXJyZW50LiAqL1xuZnVuY3Rpb24gYXBwbHlVaVJlZnJlc2hTZWN0aW9uc1RvQ2FjaGUoY2FjaGU6IFVpUmVmcmVzaENhY2hlLCBwcmVmaXg6IHN0cmluZywgZW50cmllczogcmVhZG9ubHkgUGx1Z2luVWlSZWZyZXNoU2VjdGlvblJlc3BvbnNlW10gfCB1bmRlZmluZWQpOiB2b2lkIHtcbiAgZm9yIChjb25zdCBlbnRyeSBvZiBlbnRyaWVzID8/IFtdKSB7XG4gICAgaWYgKGVudHJ5LnZhbHVlICE9PSB1bmRlZmluZWQpIGNhY2hlLnNldChgJHtwcmVmaXh9OiR7ZW50cnkua2V5fWAsIHsgaGFzaDogZW50cnkuaGFzaCwgdmFsdWU6IGVudHJ5LnZhbHVlIH0pO1xuICB9XG59XG5cbmV4cG9ydCBmdW5jdGlvbiBhcHBseVVpUmVmcmVzaFJlc3BvbnNlVG9DYWNoZShjYWNoZTogVWlSZWZyZXNoQ2FjaGUsIHJlc3BvbnNlOiBQbHVnaW5VaVJlZnJlc2hSZXNwb25zZSk6IHZvaWQge1xuICBhcHBseVVpUmVmcmVzaFNlY3Rpb25zVG9DYWNoZShjYWNoZSwgXCJ3aW5kb3dcIiwgcmVzcG9uc2Uud2luZG93cyk7XG4gIGFwcGx5VWlSZWZyZXNoU2VjdGlvbnNUb0NhY2hlKGNhY2hlLCBcInBhbmVsXCIsIHJlc3BvbnNlLnBhbmVscyk7XG4gIGlmIChyZXNwb25zZS5lbmdhZ2VtZW50cz8udmFsdWUgIT09IHVuZGVmaW5lZCkgY2FjaGUuc2V0KFwiZW5nYWdlbWVudHNcIiwgeyBoYXNoOiByZXNwb25zZS5lbmdhZ2VtZW50cy5oYXNoLCB2YWx1ZTogcmVzcG9uc2UuZW5nYWdlbWVudHMudmFsdWUgfSk7XG4gIGlmIChyZXNwb25zZS5tZWFzdXJlcz8udmFsdWUgIT09IHVuZGVmaW5lZCkgY2FjaGUuc2V0KFwibWVhc3VyZXNcIiwgeyBoYXNoOiByZXNwb25zZS5tZWFzdXJlcy5oYXNoLCB2YWx1ZTogcmVzcG9uc2UubWVhc3VyZXMudmFsdWUgfSk7XG4gIGlmIChyZXNwb25zZS50b29scz8udmFsdWUgIT09IHVuZGVmaW5lZCkgY2FjaGUuc2V0KFwidG9vbHNcIiwgeyBoYXNoOiByZXNwb25zZS50b29scy5oYXNoLCB2YWx1ZTogcmVzcG9uc2UudG9vbHMudmFsdWUgfSk7XG4gIGlmIChyZXNwb25zZS5sYWJlbHM/LnZhbHVlICE9PSB1bmRlZmluZWQpIGNhY2hlLnNldChcImxhYmVsc1wiLCB7IGhhc2g6IHJlc3BvbnNlLmxhYmVscy5oYXNoLCB2YWx1ZTogcmVzcG9uc2UubGFiZWxzLnZhbHVlIH0pO1xufVxuLy8jZW5kcmVnaW9uIFVpUmVmcmVzaFxuLy8jZW5kcmVnaW9uIFNoZWxsSGVscGVyc1xuIl0sImZpbGUiOiIvVXNlcnMvdWVsaS9Eb2N1bWVudHMvc2VtaW8v8J+nsO+4j2ZyYW1ld29yay/wn5uN77iPcHJvZHVjdHMv8J+Su++4j29zL/CflKjvuI9tb2R1bGVzL/Cfk7rvuI9yZW5kZXJlci/wn6eR77iP4oCN8J+OqO+4j2VuZ2luZS/wn6ex77iPZWxlbWVudHMvU2hlbGxIZWxwZXJzL/Cfn6bvuI9jb21wb25lbnQudHN4In0=