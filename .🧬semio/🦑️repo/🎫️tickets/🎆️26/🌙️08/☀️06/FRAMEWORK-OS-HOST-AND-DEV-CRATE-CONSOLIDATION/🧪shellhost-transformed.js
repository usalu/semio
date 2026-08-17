import { createHotContext as __vite__createHotContext } from "/@vite/client";import.meta.hot = __vite__createHotContext("/@fs/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx");import.meta.env = {"BASE_URL": "/", "DEV": true, "MODE": "development", "PROD": false, "SSR": false, "VITE_SEMIO_BRAND": "", "VITE_SEMIO_PLUGIN": "s", "VITE_SEMIO_RENDERER": "react"};import __vite__cjsImport0_react_jsxDevRuntime from "/@fs/Users/ueli/Documents/semio/node_modules/.vite-os-dev/s-react/deps/react_jsx-dev-runtime.js?v=f32c2070"; const Fragment = __vite__cjsImport0_react_jsxDevRuntime["Fragment"]; const jsxDEV = __vite__cjsImport0_react_jsxDevRuntime["jsxDEV"];
var _s = $RefreshSig$(), _s2 = $RefreshSig$(), _s3 = $RefreshSig$(), _s4 = $RefreshSig$(), _s5 = $RefreshSig$(), _s6 = $RefreshSig$(), _s7 = $RefreshSig$();
import __vite__cjsImport1_react from "/@fs/Users/ueli/Documents/semio/node_modules/.vite-os-dev/s-react/deps/react.js?v=f32c2070"; const createContext = __vite__cjsImport1_react["createContext"]; const useCallback = __vite__cjsImport1_react["useCallback"]; const useContext = __vite__cjsImport1_react["useContext"]; const useEffect = __vite__cjsImport1_react["useEffect"]; const useMemo = __vite__cjsImport1_react["useMemo"]; const useReducer = __vite__cjsImport1_react["useReducer"]; const useRef = __vite__cjsImport1_react["useRef"]; const useState = __vite__cjsImport1_react["useState"]








;
import {
  buildContributionsJson,
  createBrowserStoragePort,
  createDevPluginSource,
  createMemoryStoragePort,
  createScopedStoragePort,
  DockLayoutStore,
  DockUiStateStore,
  evictPluginModule,
  expandPluginRegistry,
  FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
  FRAMEWORK_PANEL_TAB_DOCUMENT_ICON_ID,
  FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
  FRAMEWORK_PANEL_TAB_HISTORY_ID,
  NamedLayoutStore,
  normalizeAppLabelsOverlay,
  organizeContextMenu,
  panelTabKindId,
  pendingPanelUiNode,
  pendingWindowUiNode,
  postPluginBackboneInbound,
  RECORD_TUTORIAL_ACTION_ID,
  registerPluginBackboneRoute,
  resolveExternalSlots,
  resolveLayoutForMode,
  resolveModeTools,
  resolvePlaygroundDefaultAppId,
  resolvePluginHostConfig,
  resolvePluginRegistryId,
  resolveUiDirtyScope,
  resolveWindowActions,
  SET_ACTIVE_TOOL_ACTION_ID,
  SET_ACTIVE_UTILITY_ACTION_ID,
  START_INTRODUCTION_ACTION_ID,
  START_TUTORIAL_ACTION_ID,
  TUTORIAL_CONVERGE_MS,
  windowElementId
} from "/@fs/Users/ueli/Documents/semio/🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts";
import {
  buildFileBackboneUri,
  buildFolderBackboneUri,
  buildFrameworkSyncUtilities,
  buildRemoteBackboneUri,
  decodeBackboneMessage,
  decodeBackboneWorkerResponse,
  decodePackValue,
  encodeActionWire,
  encodeBackboneMessage,
  encodeBackboneWorkerRequest,
  encodeOperationEnvelopesPack,
  FRAMEWORK_SYNC_CONTROLLER_ID,
  operationEnvelopeFromWire,
  operationEnvelopeToWire
} from "/@fs/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🟦️glue.ts";
import {
  decodeWorldProjectionTemplateId,
  worldProjectionSpecIconId,
  worldProjectionSpecLabel
} from "/@fs/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript/🟦️glue.tsx";
import {
  ANCHORS,
  App,
  applyDockSkeleton,
  applyUiThemeToRoot,
  borderNormalBottomClass,
  buildKeysByActionId,
  builtinUiDrivers,
  builtinUiThemes,
  ButtonGroup,
  ButtonGroupItem,
  CanvasSkeleton,
  CELEBRATE_STAMP_DURATION_MS,
  celebrateAllElements,
  celebrateElements,
  childElementId,
  ChromeAwareWindowScrollSurface,
  clearUiThemeFromRoot,
  cn,
  composeControlKeybindings,
  composeTutorialUi,
  ContextMenuController,
  createShellScope,
  createTutorialClock,
  DEFAULT_UI_DRIVER,
  detectShellLocale,
  disposeShellI18nInstance,
  dockSkeletonOf,
  dockSkeletonsEqual,
  elementIdSelector,
  findPanelTabInDock,
  findPanelTabNode,
  findPanelTabPath,
  Footer,
  getTutorialCameraDriver,
  Icon,
  iconRenderPort,
  insertWindowAtDropZone,
  interactiveActiveFillClass,
  interpolateTutorialCamera,
  isContextMenuPointerTarget,
  Layout,
  LevelProvider,
  loadingBorderClass,
  Mode,
  moveTabInDock,
  moveTreeUnitInDock,
  Navbar,
  NavbarExampleSelect,
  navbarFillItem,
  PanelChromeTabBar,
  PanelDockProvider,
  panelTabChildren,
  parseUiTheme,
  readStoredIntroductionSeen,
  readStoredUiChromeLocale,
  readStoredUiChromeThemeSnapshot,
  reconcileActivePath,
  resolveUiDriver,
  SemioLogo,
  semioTheme,
  serializeUiTheme,
  setActiveUiTheme,
  ShellBrandLogo,
  shellChromeTitleClassName,
  ShellScopeProvider,
  singleTreeLeaf,
  staticTreePanelDefinition,
  TextSelectionContextMenuHost,
  Toggle,
  TutorialBar,
  tutorialCameraAt,
  TutorialCaptions,
  tutorialCuesBetween,
  TutorialGhostPointer,
  tutorialSlice,
  TutorialVideoOverlay,
  UI_MOBILE_MEDIA_QUERY,
  UI_TERMINOLOGY_NATIVE,
  UIDialog,
  UIIntroduction,
  UiKeybindingsProvider,
  useActionHotkey,
  useElementsSurfaceChrome,
  useLabel,
  useMediaQuery,
  usePanelChromeHotkeys,
  useShellKeydown,
  useShellScope,
  useTutorialClock,
  validateTutorial,
  WindowBodySkeleton,
  writeStoredIntroductionSeen,
  writeStoredUiChromeAppearance,
  writeStoredUiChromeLayout,
  writeStoredUiChromeLocale,
  writeStoredUiChromeTerminology,
  writeStoredUiChromeThemeId,
  writeStoredUiChromeThemeSnapshot,
  writeStoredUiCustomDrivers,
  writeStoredUiCustomThemes,
  writeStoredUiDriverId,
  writeStoredUiKeybindingOverrides
} from "/@fs/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx";
import {
  declarativeSurfaceStatus,
  InterpretedUiNode,
  PluginSurfaceActionsContext,
  ShellContextMenuFallbackContext,
  wireLabel
} from "/@fs/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Interpreter/🟦️component.tsx";
import {
  actionStageKey,
  EMPTY_SHELL_DEFAULTS,
  EMPTY_SHELL_LOCKS,
  initialShellState,
  isEphemeralShellBrand,
  resolveBootExampleId,
  ShellFaultBoundary,
  shellReducer,
  shouldPersistIntroductionSeen,
  shouldReplayIntroductionOnLoad
} from "/@fs/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🟦️component.tsx";
import {
  beginInteractivePluginAction,
  clearPendingWorldProjection,
  endInteractivePluginAction,
  mapContextMenuSpecs,
  registerPendingWorldProjection,
  WindowInstanceIdContext
} from "/@fs/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/World3dHost/🟦️component.tsx";
import {
  DEFAULT_PANEL_WIDTH_PX,
  EMPTY_APP_LABELS_OVERLAY,
  FRAMEWORK_CATEGORY_COMMAND_ID,
  FRAMEWORK_CATEGORY_DISPLAY_ID,
  FRAMEWORK_CATEGORY_TOOL_ID,
  FRAMEWORK_RESERVED_ACTION_IDS,
  LAYOUT_CHANGE_SETTLE_MS,
  NOTE_WORLD_NAVIGATION_ACTION_ID,
  PANEL_TAB_BAR_HOSTS,
  PRESENCE_HEARTBEAT_INTERVAL_MS,
  TUTORIAL_RECORDING_EXCLUDED_ACTION_IDS,
  actionCategoryId,
  actionRequiresStagedForm,
  appDocumentLabel,
  appWindowDocumentLabel,
  applyFrameworkLayoutSeed,
  applyTutorialUiChangeToShell,
  applyTutorialUiSnapshotToShell,
  applyUiRefreshResponseToCache,
  buildActiveUtilityByWindowId,
  buildCommandCategoryTabs,
  buildNoteShellCommandAction,
  buildOsCommands,
  buildSpacePanelState,
  buildToolTabs,
  buildUiRefreshRequest,
  captureCurrentFrameworkLayout,
  captureTutorialUiSnapshot,
  categoryTabIcon,
  classifyWindowLayoutChange,
  commandCategories,
  commandCategoryLabel,
  dispatchOpenedFiles,
  dispatchOsCommand,
  downloadDataUrl,
  downloadMediaExport,
  flattenPanelTabLeaves,
  introductionTargetsWindow,
  loadPluginModuleResilient,
  makeEffectDispatchOne,
  mergeRecordPreservingIdentity,
  panelAnchorForGroup,
  panelJsonFromState,
  panelTabDefinitionToNode,
  parsePanelState,
  parseShellRoute,
  patchDocumentTreeSelectedIds,
  patchWorld3dChromeOntoNode,
  presenceClientIdentity,
  preserveJsonIdentity,
  renderStagedArgControl,
  requestFileOpen,
  resolveAppDocument,
  resolveAppLabel,
  resolveCanvasBodyKey,
  resolveCommands,
  resolveDialogDefinition,
  resolveDocumentByAppId,
  resolveFrameworkLayoutSeed,
  resolveIntroductionDefinition,
  resolveKeybindingIntent,
  resolveManifestLabel,
  resolvePanelTabLabel,
  resolveUtilityActivation,
  resolveUtilityNodes,
  resolveWindowEngagement,
  retitleWindowLayoutNode,
  runRequestMediaFrames,
  scheduleDispatchAction,
  sessionWindowInstances,
  shellLabel,
  shellTabIcon,
  spawnedWindowChromeForKind,
  studioPanelFocusingSpawned,
  syncDocumentId,
  synthesizeLocalizedLabel,
  toolIdFromPanelTabId,
  useUIHistory,
  utilityBarNode,
  utilityNodeTreeContainsId,
  viewStateWithSpacePanel,
  windowActionPaneNode,
  windowEngagementToSearchSpec,
  windowEngagementToSpec,
  windowMeasureTreeContainsId,
  windowMeasuresChrome
} from "/@fs/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx";
import { aProjectOfLuhUdkFooterItem, fundedByZukunftBauFooterItem } from "/@fs/Users/ueli/Documents/semio/♻️mit-bestand/🧺️demonstrator/⚛️footer.tsx";
import { ENTWERFEN_MIT_BESTAND_BRAND_IDS } from "/@fs/Users/ueli/Documents/semio/♻️mit-bestand/🧺️demonstrator/🟦️brand.ts";
import { createFrameworkDisplayPanelTabs, createFrameworkPluginsPanelTabs, createFrameworkSettingsPanelTabs, PluginRecoveryPanel, ShellRouteNotFoundPage, useNamedLayoutHost } from "/@fs/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ChromePanels/🟦️component.tsx";
import { SyncAttachCard } from "/@fs/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellSync/🟦️component.tsx";
import { UIFind, UIFindProvider, UISearch } from "/@fs/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellSearch/🟦️component.tsx";
import { UTILITY_CATEGORY_ICON_ID } from "/@fs/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/UtilityTree/🟦️component.tsx";
import { coerceWireBytes } from "/@fs/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/PluginRuntime/🟦️component.tsx";
export const SetWindowTitleContext = createContext(null);
export const SetWindowIconContext = createContext(null);
const EMPTY_KEYS_BY_ACTION_ID = /* @__PURE__ */ new Map();
const AppKeybindingsContext = createContext(EMPTY_KEYS_BY_ACTION_ID);
_c = AppKeybindingsContext;
export function useAppKeybindingsByActionId() {
  _s();
  return useContext(AppKeybindingsContext);
}
_s(useAppKeybindingsByActionId, "gDsCjeeItUuvgOWf1v4qoK9RF6k=");
export function useMapContextMenuSpecs(dispatch) {
  _s2();
  const keysByActionId = useAppKeybindingsByActionId();
  return useCallback((specs) => mapContextMenuSpecs(specs, dispatch, keysByActionId), [dispatch, keysByActionId]);
}
_s2(useMapContextMenuSpecs, "cxUhn+dKiu67TuQC3JX1UO+fBxU=", false, function() {
  return [useAppKeybindingsByActionId];
});
function tutorialAssetSrcToUrl(src) {
  if (src.kind === "url") return src.url;
  if (src.kind === "dataUrl") return src.data;
  console.warn("[DEBUG] tutorial blob asset src not resolvable in this scope", src.hash);
  return null;
}
const TutorialCaptionsHost = ({ tutorial, clock, captionsOn, terminology, locale }) => {
  _s3();
  const timeMs = useTutorialClock(clock);
  const cue = tutorialCuesBetween(tutorial.tracks.narration, timeMs)[0] ?? null;
  return /* @__PURE__ */ jsxDEV(TutorialCaptions, { text: cue ? resolveManifestLabel(cue.text, terminology, locale) : null, visible: captionsOn }, void 0, false, {
    fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
    lineNumber: 426,
    columnNumber: 10
  }, this);
};
_s3(TutorialCaptionsHost, "OohsVXzayHMVWLzpDlrc+X6Ca6I=", false, function() {
  return [useTutorialClock];
});
_c2 = TutorialCaptionsHost;
const TUTORIAL_DEFAULT_VIDEO_RECT = { x: 0.72, y: 0.7, width: 0.24, height: 0.24 };
const TutorialVideoOverlayHost = ({
  tutorial,
  clock,
  muted,
  playing,
  rate
}) => {
  _s4();
  const timeMs = useTutorialClock(clock);
  const cue = tutorialCuesBetween(tutorial.tracks.video, timeMs)[0] ?? null;
  const src = cue ? tutorialAssetSrcToUrl(cue.src) : null;
  const localTimeMs = cue ? timeMs - cue.at + cue.sourceOffsetMs : 0;
  return /* @__PURE__ */ jsxDEV(TutorialVideoOverlay, { src, rect: cue?.rect ?? TUTORIAL_DEFAULT_VIDEO_RECT, muted: muted || (cue?.muted ?? false), playing, rate, localTimeMs }, void 0, false, {
    fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
    lineNumber: 443,
    columnNumber: 10
  }, this);
};
_s4(TutorialVideoOverlayHost, "OohsVXzayHMVWLzpDlrc+X6Ca6I=", false, function() {
  return [useTutorialClock];
});
_c3 = TutorialVideoOverlayHost;
const TutorialGhostPointerHost = ({ tutorial, clock }) => {
  _s5();
  const timeMs = useTutorialClock(clock);
  const cue = tutorialCuesBetween(tutorial.tracks.gestures, timeMs)[0] ?? null;
  const progress = cue ? Math.min(1, Math.max(0, (timeMs - cue.at) / Math.max(cue.durationMs, 1))) : 0;
  return /* @__PURE__ */ jsxDEV(TutorialGhostPointer, { cue, progress }, void 0, false, {
    fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
    lineNumber: 451,
    columnNumber: 10
  }, this);
};
_s5(TutorialGhostPointerHost, "OohsVXzayHMVWLzpDlrc+X6Ca6I=", false, function() {
  return [useTutorialClock];
});
_c4 = TutorialGhostPointerHost;
function diffTutorialUiSnapshot(prev, next) {
  const changes = [];
  if (prev.activeModeId !== next.activeModeId && next.activeModeId != null) changes.push({ kind: "activeMode", id: next.activeModeId });
  if (prev.focusedWindowId !== next.focusedWindowId) changes.push({ kind: "focusedWindow", id: next.focusedWindowId });
  const utilityWindowIds = /* @__PURE__ */ new Set([...Object.keys(prev.activeUtilityByWindowId), ...Object.keys(next.activeUtilityByWindowId)]);
  for (const windowId of utilityWindowIds) {
    if (prev.activeUtilityByWindowId[windowId] !== next.activeUtilityByWindowId[windowId]) changes.push({ kind: "activeUtility", windowId, utilityId: next.activeUtilityByWindowId[windowId] });
  }
  if (prev.activeToolId !== next.activeToolId) changes.push({ kind: "activeTool", id: next.activeToolId });
  if (next.layout && JSON.stringify(prev.layout) !== JSON.stringify(next.layout)) changes.push({ kind: "layout", layout: next.layout });
  const groups = /* @__PURE__ */ new Set([...Object.keys(prev.activePanelTabByGroup), ...Object.keys(next.activePanelTabByGroup)]);
  for (const group of groups) {
    if (prev.activePanelTabByGroup[group] !== next.activePanelTabByGroup[group]) changes.push({ kind: "panelTab", group, tabId: next.activePanelTabByGroup[group] });
  }
  if (next.panelJson != null && prev.panelJson !== next.panelJson) changes.push({ kind: "panelState", panelJson: next.panelJson });
  if (next.selectionJson != null && prev.selectionJson !== next.selectionJson) changes.push({ kind: "selection", selectionJson: next.selectionJson });
  if (prev.openDialogId !== next.openDialogId) changes.push({ kind: "dialog", id: next.openDialogId });
  const prevTree = new Set(prev.expandedTreeIds);
  const nextTree = new Set(next.expandedTreeIds);
  for (const id of nextTree) if (!prevTree.has(id)) changes.push({ kind: "treeExpansion", id, expanded: true });
  for (const id of prevTree) if (!nextTree.has(id)) changes.push({ kind: "treeExpansion", id, expanded: false });
  if (prev.commandPanelOpen !== next.commandPanelOpen) changes.push({ kind: "commandPanel", open: next.commandPanelOpen });
  return changes;
}
function tutorialCameraPoseEquals(a, b) {
  if (a.kind !== b.kind) return false;
  if (a.kind === "orbit" && b.kind === "orbit") return a.position.every((value, index) => Math.abs(value - b.position[index]) < 1e-4) && a.target.every((value, index) => Math.abs(value - b.target[index]) < 1e-4);
  if (a.kind === "canvas" && b.kind === "canvas") return Math.abs(a.x - b.x) < 1e-4 && Math.abs(a.y - b.y) < 1e-4 && Math.abs(a.zoom - b.zoom) < 1e-4;
  return false;
}
export class TutorialRecorder {
  startedAtMs;
  baseUiSnapshot;
  baseDocumentJson;
  events = [];
  uiKeyframes = [];
  cameraKeyframes = [];
  chapters = [];
  lastUiSnapshot;
  lastCameraByWindow = /* @__PURE__ */ new Map();
  constructor(baseUiSnapshot, baseDocumentJson) {
    this.startedAtMs = performance.now();
    this.baseUiSnapshot = baseUiSnapshot;
    this.lastUiSnapshot = baseUiSnapshot;
    this.baseDocumentJson = baseDocumentJson;
  }
  nowMs() {
    return Math.max(0, Math.round(performance.now() - this.startedAtMs));
  }
  recordEvent(kind) {
    this.events.push({ at: this.nowMs(), kind });
  }
  recordUiDiff(next) {
    const changes = diffTutorialUiSnapshot(this.lastUiSnapshot, next);
    if (changes.length > 0) this.uiKeyframes.push({ at: this.nowMs(), sample: { kind: "delta", changes } });
    this.lastUiSnapshot = next;
  }
  recordSnapshot(state) {
    this.uiKeyframes.push({ at: this.nowMs(), sample: { kind: "snapshot", state } });
    this.lastUiSnapshot = state;
  }
  sampleCamera(windowId, camera) {
    const prev = this.lastCameraByWindow.get(windowId);
    if (prev && tutorialCameraPoseEquals(prev, camera)) return;
    this.lastCameraByWindow.set(windowId, camera);
    this.cameraKeyframes.push({ at: this.nowMs(), windowId, camera, easing: "easeInOut" });
  }
  /** 📖️ `ui.tutorial.addChapter` — marks the current elapsed time as a scrub-bar chapter with an
   * auto-numbered title (no naming-prompt UI in this scope; a recorded tutorial's authored titles can
   * always be hand-edited in the downloaded JSON afterward). Synthesizes a `LocalizedLabel` matrix. */
  addChapter(title) {
    const index = this.chapters.length + 1;
    const rawTitle = title ?? `Chapter ${index}`;
    this.chapters.push({ id: `chapter-${index}`, at: this.nowMs(), title: synthesizeLocalizedLabel(rawTitle) });
  }
  build(id, title, exampleId) {
    const durationMs = Math.max(1e3, this.nowMs());
    return {
      id,
      title: synthesizeLocalizedLabel(title),
      durationMs,
      chapters: this.chapters,
      base: { documentJson: this.baseDocumentJson ?? void 0, exampleId, ui: this.baseUiSnapshot, cameras: [] },
      tracks: { narration: [], video: [], events: this.events, ui: this.uiKeyframes, document: [], camera: this.cameraKeyframes, gestures: [] },
      recordedAt: (/* @__PURE__ */ new Date()).toISOString()
    };
  }
}
function resolveShellScopeStorage(ephemeral, storageNamespace) {
  if (ephemeral) return createMemoryStoragePort();
  const browser = createBrowserStoragePort();
  return storageNamespace ? createScopedStoragePort(browser, storageNamespace) : browser;
}
export function FrameworkOsShell(props) {
  _s6();
  const { shellId, storageNamespace, ownsPage = false, brand, locks, ...innerProps } = props;
  const ephemeral = isEphemeralShellBrand(brand);
  const [scope] = useState(() => {
    const storage = resolveShellScopeStorage(ephemeral, storageNamespace);
    const initialLocale = locks?.locale ?? readStoredUiChromeLocale(storage) ?? detectShellLocale(typeof navigator !== "undefined" ? navigator.language : void 0);
    return createShellScope({ shellId, ownsPage, storage, initialLocale });
  });
  const [, bumpAfterRootAttach] = useState(0);
  const setRoot = useCallback((node) => {
    scope.rootRef.current = node;
    bumpAfterRootAttach((n) => n + 1);
  }, [scope]);
  const setPortalLayer = useCallback((node) => {
    scope.portalLayerRef.current = node;
  }, [scope]);
  useEffect(() => () => disposeShellI18nInstance(scope.i18n), [scope]);
  return /* @__PURE__ */ jsxDEV("div", { ref: setRoot, className: "semio-scope", "data-shell-id": scope.shellId, style: { height: "100%", width: "100%", isolation: "isolate" }, children: /* @__PURE__ */ jsxDEV(ShellScopeProvider, { scope, children: [
    /* @__PURE__ */ jsxDEV(FrameworkOsShellInner, { ...innerProps, locks, brand }, void 0, false, {
      fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
      lineNumber: 636,
      columnNumber: 9
    }, this),
    /* @__PURE__ */ jsxDEV("div", { "data-semio-portal-layer": true, ref: setPortalLayer }, void 0, false, {
      fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
      lineNumber: 637,
      columnNumber: 9
    }, this)
  ] }, void 0, true, {
    fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
    lineNumber: 635,
    columnNumber: 7
  }, this) }, void 0, false, {
    fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
    lineNumber: 634,
    columnNumber: 5
  }, this);
}
_s6(FrameworkOsShell, "1TJcMafEbHyW9iPO1rXOoldxP+c=");
_c5 = FrameworkOsShell;
function FrameworkOsShellInner({
  pluginFilter,
  plugins,
  appId,
  locks: locksProp,
  defaults: defaultsProp,
  brand,
  suppressAutoIntroduction = false
}) {
  _s7();
  const scope = useShellScope();
  const shellContextMenuTitleLabel = useLabel("ui.surfaceContextMenu.workspace");
  const hostConfig = pluginFilter ? resolvePluginHostConfig(pluginFilter) : void 0;
  const studioMode = hostConfig !== void 0;
  const mobile = useMediaQuery(UI_MOBILE_MEDIA_QUERY);
  const locks = locksProp ?? EMPTY_SHELL_LOCKS;
  const defaults = defaultsProp ?? EMPTY_SHELL_DEFAULTS;
  const ephemeral = isEphemeralShellBrand(brand);
  const [shellState, dispatch] = useReducer(shellReducer, void 0, () => initialShellState({ pluginFilter, plugins, locks, defaults, storage: scope.storage }));
  const { loadedPlugins, pluginStatusById, pluginSupervisorById, session, error } = shellState.pluginRuntime;
  const hostPlugin = useMemo(() => hostConfig ? loadedPlugins.find((entry) => entry.handle.pluginId === hostConfig.pluginId) : void 0, [loadedPlugins, hostConfig]);
  const hostApp = useMemo(() => hostPlugin?.manifest.apps.find((app) => app.id === hostConfig?.hostAppId), [hostPlugin, hostConfig]);
  const landingApp = useMemo(() => hostPlugin?.manifest.apps.find((app) => app.id === hostConfig?.landingAppId) ?? hostPlugin?.manifest.apps[0], [hostPlugin, hostConfig]);
  const landingAppId = hostConfig?.landingAppId;
  const hostAppId = hostConfig?.hostAppId;
  const hostControllerId = hostApp?.controllerId;
  const landingControllerId = landingApp?.controllerId;
  const hostCatalogueTabId = hostApp?.panelTabs[0] ? panelTabKindId(hostApp.panelTabs[0].kind) : void 0;
  const { windowUiByWindowId, windowEngagementsByWindowId, windowMeasuresByWindowId, toolMeasuresByToolId, panelUiByKey, appLabelsOverlay } = shellState.windowUi;
  const { spawnedWindowUi, spawnedWindowEngagements, spawnedWindowMeasures } = shellState.spawnedWindow;
  const { foldedByWindowId: actionPaneFoldedByWindowId, expandedByWindowId: actionPaneExpandedByWindowId, stagedArgsByKey: actionPaneStagedArgsByKey, activeUtilityByWindowId, activeToolId } = shellState.actionPane;
  const { expandedCommandId, stagedArgsByCommandId: commandStagedArgsByCommandId } = shellState.commandPanel;
  const { panels, dockOverride, panelPathMemory, treeOpenStates, activeWindowId, shellLayout, activeExampleId, mobilePanelPath, mobilePanelVisible, extraWindowInstances, windowTitlesById, windowIconsById } = shellState.layout;
  const { searchOpen, findOpen, introductionStepIndex, introductionCompletedInteractions, dialog: overlayDialog } = shellState.overlays;
  const { activeTutorialId, playing: tutorialPlaying, rate: tutorialRate, muted: tutorialMuted, captionsOn: tutorialCaptionsOn, recording: tutorialRecording, deviated: tutorialDeviated } = shellState.tutorial;
  const { uiAppearance, uiLayout, uiDriverId, uiCustomDrivers, uiDriverDraft, uiLocale, uiTerminology, uiThemeId, uiCustomThemes, uiThemeDraft, uiKeybindingOverrides } = shellState.uiPrefs;
  const { syncBackboneUri, syncCardKind, syncDraftPath, syncStatusByDocumentId } = shellState.sync;
  const importSpaceInputRef = useRef(null);
  const refreshGenerationRef = useRef(0);
  const contributionsJsonRef = useRef(null);
  const appRegistrationsJsonRef = useRef(null);
  const spawnedRefreshGenerationRef = useRef(0);
  const contributorInstancesRef = useRef(/* @__PURE__ */ new Map());
  const layoutSeedKeyRef = useRef(null);
  const noExampleResetInstanceIdRef = useRef(null);
  const extraWindowCounterRef = useRef(0);
  const [shellContextMenu, setShellContextMenu] = useState(null);
  const extraWindowInstancesRef = useRef([]);
  extraWindowInstancesRef.current = extraWindowInstances;
  const setWindowTitle = useCallback((windowId, title) => {
    dispatch({ type: "SET_WINDOW_TITLE", windowId, title });
  }, []);
  const setWindowIcon = useCallback((windowId, iconId) => {
    dispatch({ type: "SET_WINDOW_ICON", windowId, iconId });
  }, []);
  const uiRefreshCacheRef = useRef(/* @__PURE__ */ new Map());
  const spawnedUiRefreshCacheRef = useRef(/* @__PURE__ */ new Map());
  const spawnedLayoutSeedRef = useRef(null);
  const openSpaceIdRef = useRef(null);
  const openInstanceIdRef = useRef(null);
  const sessionRef = useRef(null);
  const uiDevice = mobile ? "mobile" : uiLayout;
  const uiTheme = useMemo(() => {
    if (uiThemeDraft) return uiThemeDraft;
    const found = builtinUiThemes().find((t) => t.id === uiThemeId) ?? uiCustomThemes[uiThemeId];
    return found ?? readStoredUiChromeThemeSnapshot(scope.storage) ?? semioTheme();
  }, [uiThemeId, uiCustomThemes, uiThemeDraft, scope.storage]);
  const uiDriver = useMemo(() => uiDriverDraft ?? resolveUiDriver(uiDriverId, uiCustomDrivers), [uiDriverId, uiCustomDrivers, uiDriverDraft]);
  const backboneWorkerRef = useRef(null);
  const shellActorIdRef = useRef(`client-${Math.random().toString(36).slice(2)}`);
  const openDocumentSessionsRef = useRef(/* @__PURE__ */ new Map());
  const pluginBackboneRouteUnregistersRef = useRef(/* @__PURE__ */ new Map());
  const loadedPluginsRef = useRef([]);
  loadedPluginsRef.current = loadedPlugins;
  const pluginModuleUrlByIdRef = useRef(/* @__PURE__ */ new Map());
  const pluginOpInFlightRef = useRef(/* @__PURE__ */ new Set());
  const ensureBackboneWorker = useCallback(() => {
    if (backboneWorkerRef.current) return backboneWorkerRef.current;
    const worker = new Worker(new URL(/* @vite-ignore */ "/@fs/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/⚡️implementations/🟦️typescript/🟦️backbone-worker.ts?worker_file&type=module", import.meta.url), { type: "module" });
    worker.onmessage = (messageEvent) => {
      const message = "wire" in messageEvent.data ? decodeBackboneWorkerResponse(messageEvent.data.wire) : messageEvent.data;
      if (message.kind !== "event") return;
      const entry = openDocumentSessionsRef.current.get(message.documentId);
      if (!entry) return;
      const { event } = message;
      if (event.kind === "status") {
        dispatch({ type: "SET_SYNC_STATUS_FOR_DOCUMENT", documentId: message.documentId, status: { persisted: event.persisted, pendingOperations: event.pendingOperations, remote: event.remote } });
      } else if (event.kind === "presence") {
        const peersJson = JSON.stringify(event.peers.map((peer) => ({ clientId: peer.actor, name: peer.label ?? peer.actor, selectionCount: 0 })));
        dispatch({
          type: "SET_SESSION",
          value: (current) => current && current.instanceId === entry.session.instanceId ? { ...current, viewState: { ...current.viewState, presencePeersJson: peersJson } } : current
        });
      } else if (event.kind === "remoteOperations" && entry.plugin.applyOperations) {
        void entry.plugin.applyOperations(entry.session.instanceId, encodeOperationEnvelopesPack(event.envelopes));
        const actorUri = `actor://${message.documentId}`;
        postPluginBackboneInbound(
          entry.session.pluginId,
          actorUri,
          [
            encodeBackboneMessage({
              kind: "operations",
              envelopes: event.envelopes.map(
                (envelope, index) => operationEnvelopeToWire(envelope, { actor: 0, physical_ms: Date.now(), logical: index + 1 })
              )
            })
          ]
        );
      } else if (event.kind === "snapshotReplaced" && entry.plugin.loadAppDocument) {
        const packBytes = new Uint8Array(event.pack);
        let documentJson;
        try {
          documentJson = JSON.stringify(decodePackValue(packBytes));
        } catch {
          documentJson = JSON.stringify({ pack: Array.from(event.pack), spr: Array.from(event.spr) });
        }
        void entry.plugin.loadAppDocument(entry.session.instanceId, documentJson);
        const actorUri = `actor://${message.documentId}`;
        postPluginBackboneInbound(
          entry.session.pluginId,
          actorUri,
          [
            encodeBackboneMessage({ kind: "snapshot", pack: packBytes, spr: new Uint8Array(event.spr) })
          ]
        );
      } else if (event.kind === "conflict") {
        console.warn("[os-shell] sync conflict", message.documentId, event.message);
      }
    };
    backboneWorkerRef.current = worker;
    return worker;
  }, []);
  const { uri: shellUri, canGoBack, canGoForward, canGoUp, goBack, goForward, goUp, navigate: navigateHistory } = useUIHistory("/", studioMode && scope.ownsPage);
  const shellRoute = useMemo(() => parseShellRoute(shellUri.split("?")[0] ?? "/"), [shellUri]);
  const shellStorage = scope.storage;
  const namedLayoutStore = useMemo(() => new NamedLayoutStore(session?.app.id ?? "framework-os", shellStorage), [session?.app.id, shellStorage]);
  const dockLayoutStore = useMemo(() => new DockLayoutStore(shellStorage, session?.app.id), [session?.app.id, shellStorage]);
  const dockUiStateStore = useMemo(() => new DockUiStateStore(shellStorage, session?.app.id), [session?.app.id, shellStorage]);
  const registry = useMemo(() => {
    const expanded = expandPluginRegistry(plugins, pluginFilter ? resolvePluginRegistryId(pluginFilter) : void 0, studioMode);
    if (studioMode) return expanded;
    return pluginFilter ? expanded : plugins;
  }, [pluginFilter, plugins, studioMode]);
  const primaryPluginId = useMemo(() => hostConfig?.pluginId ?? (pluginFilter ? resolvePluginRegistryId(pluginFilter) : void 0) ?? registry[0]?.pluginId, [hostConfig, pluginFilter, registry]);
  const shellPluginCanvasStatus = useMemo(() => {
    if (!session) return "loading";
    if (!primaryPluginId) return void 0;
    const pluginStatus = pluginStatusById[primaryPluginId];
    if (pluginStatus === "installing" || pluginStatus === "reloading") return "loading";
    return void 0;
  }, [session, primaryPluginId, pluginStatusById]);
  const pluginSource = useMemo(() => createDevPluginSource(registry), [registry]);
  const establishPrimarySession = useCallback(
    async (handle) => {
      const manifest = handle.manifest;
      if (hostConfig) {
        const sApp = manifest.apps.find((app) => app.id === hostConfig.landingAppId) ?? manifest.apps[0];
        if (!sApp) throw new Error("host program missing landing app");
        const panelState = buildSpacePanelState([], []);
        const instanceId2 = await handle.createApp(sApp.id);
        const viewState = { activeModeId: sApp.defaultModeId ?? sApp.modes[0]?.id, panelJson: panelJsonFromState(panelState) };
        const seeded2 = applyFrameworkLayoutSeed(sApp.defaultLayout, sApp.windowKinds, EMPTY_APP_LABELS_OVERLAY, uiTerminology, uiLocale);
        extraWindowInstancesRef.current = seeded2.extraInstances;
        extraWindowCounterRef.current = seeded2.extraInstances.length;
        dispatch({ type: "SET_SESSION", value: { pluginId: handle.pluginId, instanceId: instanceId2, app: sApp, viewState } });
        dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: seeded2.extraInstances });
        dispatch({ type: "SET_SHELL_LAYOUT", value: seeded2.modeLayout });
        dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: null });
        dispatch({ type: "SET_ERROR", value: null });
        return;
      }
      const primaryApp = appId ? (() => {
        const found = manifest.apps.find((app) => app.id === appId);
        if (!found) throw new Error(`appId "${appId}" does not resolve to any app in the loaded program manifest`);
        return found;
      })() : (() => {
        const defaultAppId = pluginFilter ? resolvePlaygroundDefaultAppId(pluginFilter) : void 0;
        return (defaultAppId ? manifest.apps.find((app) => app.id === defaultAppId) : void 0) ?? manifest.apps[0];
      })();
      if (!primaryApp) return;
      const instanceId = await handle.createApp(primaryApp.id);
      const seeded = applyFrameworkLayoutSeed(primaryApp.defaultLayout, primaryApp.windowKinds, EMPTY_APP_LABELS_OVERLAY, uiTerminology, uiLocale);
      extraWindowInstancesRef.current = seeded.extraInstances;
      extraWindowCounterRef.current = seeded.extraInstances.length;
      dispatch({
        type: "SET_SESSION",
        value: { pluginId: handle.pluginId, instanceId, app: primaryApp, viewState: { activeModeId: primaryApp.defaultModeId ?? primaryApp.modes[0]?.id } }
      });
      dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: seeded.extraInstances });
      dispatch({ type: "SET_SHELL_LAYOUT", value: seeded.modeLayout });
      dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: null });
      dispatch({ type: "SET_ERROR", value: null });
    },
    [hostConfig, appId, pluginFilter, uiTerminology, uiLocale]
  );
  const installPlugin = useCallback(
    async (pluginId, rebuiltAt) => {
      if (pluginOpInFlightRef.current.has(pluginId)) return "in-flight";
      if (loadedPluginsRef.current.some((entry2) => entry2.handle.pluginId === pluginId)) return "already-loaded";
      const entry = registry.find((candidate) => candidate.pluginId === pluginId);
      if (!entry) return "missing-registry";
      pluginOpInFlightRef.current.add(pluginId);
      dispatch({ type: "SET_PLUGIN_STATUS", pluginId, value: "installing" });
      try {
        const moduleUrl = pluginSource.moduleUrl(pluginId, rebuiltAt);
        const handle = await loadPluginModuleResilient(pluginId, moduleUrl);
        if (!handle) {
          dispatch({ type: "SET_PLUGIN_STATUS", pluginId, value: "failed" });
          dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: "crashed" });
          return "failed";
        }
        pluginModuleUrlByIdRef.current.set(pluginId, moduleUrl);
        dispatch({ type: "UPSERT_LOADED_PLUGIN", value: { handle, manifest: handle.manifest } });
        dispatch({ type: "SET_PLUGIN_STATUS", pluginId, value: "loaded" });
        dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: "loaded" });
        if (pluginId === primaryPluginId && !sessionRef.current) {
          try {
            await establishPrimarySession(handle);
            dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: "running" });
          } catch (bootError) {
            console.error("[DEBUG] framework os boot failed", bootError);
            dispatch({ type: "SET_ERROR", value: bootError instanceof Error ? bootError.message : String(bootError) });
            return "failed";
          }
        }
        return "loaded";
      } finally {
        pluginOpInFlightRef.current.delete(pluginId);
      }
    },
    [registry, pluginSource, primaryPluginId, establishPrimarySession]
  );
  const reloadPlugin = useCallback(
    async (pluginId, rebuiltAt) => {
      if (pluginOpInFlightRef.current.has(pluginId)) return;
      const current = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === pluginId);
      if (!current) return installPlugin(pluginId, rebuiltAt);
      const oldModuleUrl = pluginModuleUrlByIdRef.current.get(pluginId);
      pluginOpInFlightRef.current.add(pluginId);
      dispatch({ type: "SET_PLUGIN_STATUS", pluginId, value: "reloading" });
      dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: "restarting" });
      let newHandle = null;
      try {
        const moduleUrl = pluginSource.moduleUrl(pluginId, rebuiltAt);
        newHandle = await loadPluginModuleResilient(pluginId, moduleUrl);
        if (!newHandle) throw new Error(`program ${pluginId} failed to reload`);
        if (newHandle.manifest.apps.length === 0) throw new Error(`program ${pluginId} reload declares no apps`);
        const activeSession = sessionRef.current;
        const ownsSession = activeSession?.pluginId === pluginId;
        if (ownsSession && activeSession && !newHandle.manifest.apps.some((app) => app.id === activeSession.app.id)) {
          throw new Error(`program ${pluginId} reload dropped the active session's app "${activeSession.app.id}"`);
        }
        const oldAppIds = new Set(current.manifest.apps.map((app) => app.id));
        const newAppIds = new Set(newHandle.manifest.apps.map((app) => app.id));
        const hotSwapEvent = {
          pluginId,
          version: newHandle.manifest.version,
          addedApps: [...newAppIds].filter((id) => !oldAppIds.has(id)),
          removedApps: [...oldAppIds].filter((id) => !newAppIds.has(id))
        };
        console.log(`[DEBUG] hot-swap ${pluginId}`, hotSwapEvent);
        if (ownsSession && activeSession) {
          await current.handle.destroyApp(activeSession.instanceId).catch(() => {
          });
        }
        for (const spawned of spawnedAppsRef.current.filter((entry) => entry.pluginId === pluginId)) {
          await current.handle.destroyApp(spawned.instanceId).catch(() => {
          });
        }
        const contributorInstanceId = contributorInstancesRef.current.get(pluginId);
        if (contributorInstanceId != null) {
          await current.handle.destroyApp(contributorInstanceId).catch(() => {
          });
          contributorInstancesRef.current.delete(pluginId);
        }
        if (studioMode && activeSession) {
          const currentPanel = parsePanelState(activeSession.viewState);
          const dropped = currentPanel?.spawnedApps.filter((entry) => entry.pluginId === pluginId) ?? [];
          if (currentPanel && dropped.length > 0) {
            console.log(
              `[DEBUG] hot-swap ${pluginId} dropped ${dropped.length} spawned instance(s)`,
              dropped.map((entry) => entry.id)
            );
            const survivingSpawned = currentPanel.spawnedApps.filter((entry) => entry.pluginId !== pluginId);
            const activeSpawnedId = currentPanel.activeSpawnedId && dropped.some((entry) => entry.id === currentPanel.activeSpawnedId) ? void 0 : currentPanel.activeSpawnedId;
            const nextPanel = { ...currentPanel, spawnedApps: survivingSpawned, activeSpawnedId };
            dispatch({
              type: "SET_SESSION",
              value: (nextSession) => nextSession ? { ...nextSession, viewState: { ...nextSession.viewState, panelJson: panelJsonFromState(nextPanel) } } : nextSession
            });
          }
        }
        pluginModuleUrlByIdRef.current.set(pluginId, moduleUrl);
        dispatch({ type: "UPSERT_LOADED_PLUGIN", value: { handle: newHandle, manifest: newHandle.manifest } });
        dispatch({ type: "SET_PLUGIN_STATUS", pluginId, value: "loaded" });
        dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: ownsSession ? "running" : "loaded" });
        if (ownsSession) await establishPrimarySession(newHandle);
        current.handle.dispose();
        if (oldModuleUrl) evictPluginModule(oldModuleUrl);
      } catch (error2) {
        console.warn(`[DEBUG] hot-swap rolled back for ${pluginId}`, error2);
        newHandle?.dispose();
        dispatch({ type: "SET_PLUGIN_STATUS", pluginId, value: "loaded" });
        dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: "crashed" });
      } finally {
        pluginOpInFlightRef.current.delete(pluginId);
      }
    },
    [installPlugin, establishPrimarySession, studioMode, pluginSource]
  );
  const uninstallPlugin = useCallback(
    async (pluginId) => {
      if (pluginOpInFlightRef.current.has(pluginId)) return;
      const current = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === pluginId);
      if (!current) return;
      if (pluginId === primaryPluginId) {
        console.warn(`[DEBUG] refusing to uninstall the host/primary plugin: ${pluginId}`);
        return;
      }
      if (sessionRef.current?.pluginId === pluginId) {
        console.warn(`[DEBUG] refusing to uninstall the active session's plugin: ${pluginId}`);
        return;
      }
      pluginOpInFlightRef.current.add(pluginId);
      try {
        for (const spawned of spawnedAppsRef.current.filter((entry) => entry.pluginId === pluginId)) {
          await current.handle.destroyApp(spawned.instanceId).catch(() => {
          });
        }
        const contributorInstanceId = contributorInstancesRef.current.get(pluginId);
        if (contributorInstanceId != null) {
          await current.handle.destroyApp(contributorInstanceId).catch(() => {
          });
          contributorInstancesRef.current.delete(pluginId);
        }
        if (studioMode && sessionRef.current) {
          const activeSession = sessionRef.current;
          const currentPanel = parsePanelState(activeSession.viewState);
          const dropped = currentPanel?.spawnedApps.filter((entry) => entry.pluginId === pluginId) ?? [];
          if (currentPanel && dropped.length > 0) {
            const survivingSpawned = currentPanel.spawnedApps.filter((entry) => entry.pluginId !== pluginId);
            const activeSpawnedId = currentPanel.activeSpawnedId && dropped.some((entry) => entry.id === currentPanel.activeSpawnedId) ? void 0 : currentPanel.activeSpawnedId;
            const nextPanel = { ...currentPanel, spawnedApps: survivingSpawned, activeSpawnedId };
            dispatch({
              type: "SET_SESSION",
              value: (nextSession) => nextSession ? { ...nextSession, viewState: { ...nextSession.viewState, panelJson: panelJsonFromState(nextPanel) } } : nextSession
            });
          }
        }
        dispatch({ type: "REMOVE_LOADED_PLUGIN", pluginId });
        dispatch({ type: "SET_PLUGIN_STATUS", pluginId, value: "available" });
        current.handle.dispose();
        const moduleUrl = pluginModuleUrlByIdRef.current.get(pluginId);
        pluginModuleUrlByIdRef.current.delete(pluginId);
        if (moduleUrl) evictPluginModule(moduleUrl);
      } finally {
        pluginOpInFlightRef.current.delete(pluginId);
      }
    },
    [primaryPluginId, studioMode]
  );
  const panel = useMemo(() => session ? parsePanelState(session.viewState) : null, [session?.viewState.panelJson]);
  const spawnedAppsRef = useRef([]);
  spawnedAppsRef.current = panel?.spawnedApps ?? [];
  const activeSpawnedEntry = panel?.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId);
  const activeAppTitle = appDocumentLabel(activeSpawnedEntry ? resolveDocumentByAppId(loadedPlugins, activeSpawnedEntry.appId, activeSpawnedEntry.document, uiTerminology) : session ? resolveAppDocument(session.app, uiTerminology) : []);
  useEffect(() => {
    sessionRef.current = session;
  }, [session]);
  const activeIntroduction = brand?.introduction ?? session?.app.introduction;
  const introductionSeenKey = session ? brand ? `${brand.id}:${session.app.id}` : session.app.id : "";
  const replayIntroductionOnLoad = shouldReplayIntroductionOnLoad(brand);
  const persistIntroductionSeen = shouldPersistIntroductionSeen(brand);
  const activeIntroductionRef = useRef(activeIntroduction);
  activeIntroductionRef.current = activeIntroduction;
  useEffect(() => {
    if (!session || !activeIntroduction || shellState.tutorial.activeTutorialId != null) return;
    if (typeof window !== "undefined" && window.self !== window.top) return;
    if (suppressAutoIntroduction) return;
    if (!replayIntroductionOnLoad && readStoredIntroductionSeen(scope.storage, introductionSeenKey)) return;
    dispatch({ type: "SET_INTRODUCTION_STEP", value: 0 });
  }, [session?.app.id, activeIntroduction, introductionSeenKey, replayIntroductionOnLoad, shellState.tutorial.activeTutorialId, suppressAutoIntroduction]);
  const activeTutorials = useMemo(() => [...brand?.tutorials ?? [], ...session?.app.tutorials ?? []], [brand?.tutorials, session?.app.tutorials]);
  const tutorialRecorderAvailable = useMemo(() => {
    try {
      return Boolean(import.meta.env?.DEV);
    } catch {
      return false;
    }
  }, []);
  const activeUtilityByWindowIdRef = useRef(activeUtilityByWindowId);
  activeUtilityByWindowIdRef.current = activeUtilityByWindowId;
  const activeToolIdRef = useRef(activeToolId);
  activeToolIdRef.current = activeToolId;
  const setActiveUtilityForWindow = useCallback((windowId, utilityId) => {
    activeUtilityByWindowIdRef.current = { ...activeUtilityByWindowIdRef.current, [windowId]: utilityId };
    dispatch({ type: "SET_ACTIVE_UTILITY", windowId, utilityId });
  }, []);
  const clearAllWindowUtilities = useCallback(() => {
    const next = { ...activeUtilityByWindowIdRef.current };
    for (const windowId of Object.keys(next)) {
      if (next[windowId]) {
        next[windowId] = null;
        dispatch({ type: "SET_ACTIVE_UTILITY", windowId, utilityId: null });
      }
    }
    activeUtilityByWindowIdRef.current = next;
  }, []);
  const toolMeasuresByToolIdRef = useRef(toolMeasuresByToolId);
  toolMeasuresByToolIdRef.current = toolMeasuresByToolId;
  const activeWindowIdRef = useRef(activeWindowId);
  activeWindowIdRef.current = activeWindowId;
  const actionPaneExpandedByWindowIdRef = useRef(actionPaneExpandedByWindowId);
  actionPaneExpandedByWindowIdRef.current = actionPaneExpandedByWindowId;
  const actionPaneStagedArgsByKeyRef = useRef(actionPaneStagedArgsByKey);
  actionPaneStagedArgsByKeyRef.current = actionPaneStagedArgsByKey;
  const introductionStepIndexRef = useRef(introductionStepIndex);
  introductionStepIndexRef.current = introductionStepIndex;
  const introductionCompletedInteractionsRef = useRef(introductionCompletedInteractions);
  introductionCompletedInteractionsRef.current = introductionCompletedInteractions;
  const startTutorialRef = useRef(() => {
  });
  const stopTutorialRef = useRef(() => {
  });
  const toggleTutorialRecordingRef = useRef(() => {
  });
  const tutorialDrivenRef = useRef(false);
  const tutorialPlayingRef = useRef(tutorialPlaying);
  tutorialPlayingRef.current = tutorialPlaying;
  const tutorialRecordingRef = useRef(tutorialRecording);
  tutorialRecordingRef.current = tutorialRecording;
  const tutorialRecorderRef = useRef(null);
  const shellStateRef = useRef(shellState);
  shellStateRef.current = shellState;
  const dismissIntroduction = useCallback(
    (completed) => {
      if (completed && scope.rootRef.current) celebrateAllElements(CELEBRATE_STAMP_DURATION_MS, scope.rootRef.current);
      dispatch({ type: "SET_INTRODUCTION_STEP", value: null });
      if (persistIntroductionSeen) writeStoredIntroductionSeen(scope.storage, introductionSeenKey);
    },
    [introductionSeenKey, persistIntroductionSeen]
  );
  const advanceIntroductionByDoing = useCallback(
    (celebrateOverride) => {
      const stepIndex = introductionStepIndexRef.current;
      const introduction = activeIntroductionRef.current;
      if (stepIndex == null || !introduction) return;
      const step = introduction.steps[stepIndex];
      if (stepIndex >= introduction.steps.length - 1) {
        dismissIntroduction(true);
        return;
      }
      const celebrateId = celebrateOverride ?? step?.introduce;
      if (step && (step.interactions ?? []).length > 0 && celebrateId && scope.rootRef.current) celebrateElements(elementIdSelector(celebrateId), CELEBRATE_STAMP_DURATION_MS, scope.rootRef.current);
      dispatch({ type: "SET_INTRODUCTION_STEP", value: stepIndex + 1 });
    },
    [dismissIntroduction]
  );
  const completeIntroductionInteraction = useCallback(
    (matches, celebrateOverride) => {
      const stepIndex = introductionStepIndexRef.current;
      const introduction = activeIntroductionRef.current;
      if (stepIndex == null || !introduction) return;
      const step = introduction.steps[stepIndex];
      if (!step || (step.interactions ?? []).length === 0) return;
      const completed = introductionCompletedInteractionsRef.current;
      const interactions = step.interactions ?? [];
      const index = interactions.findIndex((interaction, i) => !completed.includes(i) && matches(interaction));
      if (index < 0) return;
      if (step.ordered && index !== completed.length) return;
      const celebrateId = celebrateOverride ?? interactions[index].celebrate ?? step.introduce;
      if (celebrateId && scope.rootRef.current) celebrateElements(elementIdSelector(celebrateId), CELEBRATE_STAMP_DURATION_MS, scope.rootRef.current);
      introductionCompletedInteractionsRef.current = [...completed, index];
      dispatch({ type: "COMPLETE_INTRODUCTION_INTERACTION", index });
      if (introductionCompletedInteractionsRef.current.length >= interactions.length) advanceIntroductionByDoing(celebrateOverride);
    },
    [advanceIntroductionByDoing]
  );
  const expandedCommandIdRef = useRef(expandedCommandId);
  expandedCommandIdRef.current = expandedCommandId;
  const commandStagedArgsByCommandIdRef = useRef(commandStagedArgsByCommandId);
  commandStagedArgsByCommandIdRef.current = commandStagedArgsByCommandId;
  const injectActiveTool = useCallback((viewState) => {
    const toolId = activeToolIdRef.current ?? void 0;
    return viewState.activeToolId === toolId ? viewState : { ...viewState, activeToolId: toolId };
  }, []);
  const injectActiveUtility = useCallback((viewState, windowId) => {
    const key = windowId ?? activeWindowIdRef.current;
    const utilityId = key ? activeUtilityByWindowIdRef.current[key] ?? void 0 : void 0;
    const withUtility = viewState.activeUtilityId === utilityId ? viewState : { ...viewState, activeUtilityId: utilityId };
    return injectActiveTool(withUtility);
  }, [injectActiveTool]);
  useEffect(() => {
    dispatch({ type: "SET_SYNC_BACKBONE_URI", value: null });
    dispatch({ type: "SET_SYNC_CARD_KIND", value: null });
  }, [panel?.activeSpawnedId, session, studioMode]);
  const relayPluginBackboneMessage = useCallback((uri, messageBytes) => {
    const documentId = uri.startsWith("actor://") ? uri.slice("actor://".length) : null;
    if (!documentId) return;
    const worker = backboneWorkerRef.current;
    if (!worker) return;
    let actorMessage;
    try {
      const parsed = decodeBackboneMessage(messageBytes);
      if (parsed.kind === "operations") {
        actorMessage = {
          kind: "localOperations",
          envelopes: parsed.envelopes.map((envelope) => operationEnvelopeFromWire(envelope))
        };
      } else if (parsed.kind === "snapshot") {
        actorMessage = { kind: "localSnapshot", pack: Array.from(parsed.pack), spr: Array.from(parsed.spr) };
      } else {
        return;
      }
    } catch {
      return;
    }
    const request = { kind: "send", documentId, message: actorMessage };
    worker.postMessage({ wire: encodeBackboneWorkerRequest(request) });
  }, []);
  useEffect(() => {
    const worker = backboneWorkerRef.current;
    return () => worker?.terminate();
  }, []);
  useEffect(() => {
    return () => {
      for (const unregister of pluginBackboneRouteUnregistersRef.current.values()) unregister();
      pluginBackboneRouteUnregistersRef.current.clear();
      const primary = sessionRef.current;
      if (primary) {
        const plugin = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === primary.pluginId)?.handle;
        void plugin?.destroyApp(primary.instanceId).catch(() => {
        });
      }
      for (const spawned of spawnedAppsRef.current) {
        const plugin = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === spawned.pluginId)?.handle;
        void plugin?.destroyApp(spawned.instanceId).catch(() => {
        });
      }
      for (const [pluginId, instanceId] of contributorInstancesRef.current) {
        const plugin = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === pluginId)?.handle;
        void plugin?.destroyApp(instanceId).catch(() => {
        });
      }
      contributorInstancesRef.current.clear();
      for (const entry of loadedPluginsRef.current) entry.handle.dispose();
    };
  }, []);
  useEffect(() => {
    if (!scope.ownsPage) return;
    if (brand) {
      document.title = brand.windowTitle;
    } else if (activeAppTitle) {
      document.title = activeAppTitle;
    }
  }, [activeAppTitle, brand, scope.ownsPage]);
  useEffect(() => {
    if (!primaryPluginId) return;
    if (loadedPluginsRef.current.some((entry) => entry.handle.pluginId === primaryPluginId)) return;
    void (async () => {
      const outcome = await installPlugin(primaryPluginId);
      if (outcome === "failed") {
        dispatch({ type: "SET_ERROR", value: shellLabel("ui.common.noPluginsLoaded") });
      }
    })();
  }, [primaryPluginId, installPlugin]);
  useEffect(() => {
    const registryIds = new Set(registry.map((entry) => entry.pluginId));
    const handlePluginAvailable = (pluginId, rebuiltAt) => {
      if (!registryIds.has(pluginId)) return;
      const alreadyLoaded = loadedPluginsRef.current.some((entry) => entry.handle.pluginId === pluginId);
      void (alreadyLoaded ? reloadPlugin(pluginId, rebuiltAt) : installPlugin(pluginId, rebuiltAt));
    };
    return pluginSource.subscribe((event) => {
      if (event.kind === "snapshot") {
        for (const plugin of event.plugins) handlePluginAvailable(plugin.pluginId, plugin.rebuiltAt);
        return;
      }
      handlePluginAvailable(event.pluginId, event.rebuiltAt);
    });
  }, [registry, pluginSource, installPlugin, reloadPlugin]);
  const findPluginForAction = useCallback(
    (action) => {
      const byController = loadedPlugins.find((entry) => entry.manifest.apps.some((app) => app.controllerId === action.controllerId));
      if (byController) return byController;
      return loadedPlugins.find((entry) => entry.handle.pluginId === session?.pluginId);
    },
    [loadedPlugins, session?.pluginId]
  );
  const requestContextMenu = useCallback(
    async (request) => {
      if (!session) return [];
      const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === session.pluginId)?.handle;
      if (!plugin?.contextMenu) return [];
      return plugin.contextMenu(session.instanceId, request);
    },
    [loadedPlugins, session]
  );
  const refreshUi = useCallback(
    // 🪟️ `extraInstancesOverride` lets a caller that just synchronously computed a NEW extra-window list
    // (split/drop, layout/mode switch) hand it straight to this fetch instead of reading `extraWindowInstances`
    // from React state, which wouldn't reflect the just-dispatched change until the next render.
    async (nextSession, scopeArg = { kind: "full" }, extraInstancesOverride) => {
      if (scopeArg.kind === "none") return;
      const generation = ++refreshGenerationRef.current;
      const program = loadedPlugins.find((entry) => entry.handle.pluginId === nextSession.pluginId)?.handle;
      if (!program) return;
      const layoutSeedKey = `${nextSession.pluginId}:${nextSession.app.id}:${nextSession.instanceId}`;
      const isSessionSwitch = layoutSeedKeyRef.current !== layoutSeedKey;
      let scope2 = scopeArg;
      if (isSessionSwitch) {
        uiRefreshCacheRef.current = /* @__PURE__ */ new Map();
        scope2 = { kind: "full" };
      }
      const cache = uiRefreshCacheRef.current;
      const layoutSeed = isSessionSwitch ? applyFrameworkLayoutSeed(nextSession.app.defaultLayout, nextSession.app.windowKinds, appLabelsOverlay, uiTerminology, uiLocale) : void 0;
      const extraInstancesForFetch = extraInstancesOverride ?? layoutSeed?.extraInstances ?? extraWindowInstancesRef.current;
      const windowInstances = sessionWindowInstances(nextSession.app, extraInstancesForFetch);
      const contributionsJson = buildContributionsJson(loadedPlugins.map((entry) => ({ pluginId: entry.handle.pluginId, manifest: entry.manifest })));
      const appRegistrationsJson = JSON.stringify(loadedPlugins.flatMap((entry) => (entry.manifest.apps ?? []).map((app) => ({ pluginId: entry.handle.pluginId, app }))));
      const viewState = injectActiveTool({
        ...nextSession.viewState,
        contributionsJson,
        locale: uiLocale,
        terminology: uiTerminology,
        windowInstances: windowInstances.map((instance) => ({ id: instance.id, windowKindId: instance.windowKindId })),
        activeUtilityByWindowId: buildActiveUtilityByWindowId(activeUtilityByWindowIdRef.current),
        activeUtilityId: void 0
      });
      const panelTabLeaves = flattenPanelTabLeaves(nextSession.app.panelTabs);
      const request = buildUiRefreshRequest(scope2, windowInstances, panelTabLeaves, viewState, cache);
      if (request) {
        const response = await program.refreshUi(nextSession.instanceId, request);
        if (generation !== refreshGenerationRef.current) return;
        const slotContext = {
          plugins: new Map(loadedPlugins.map((entry) => [entry.handle.pluginId, entry.handle])),
          contributorInstances: contributorInstancesRef.current,
          viewState
        };
        const resolveIfChanged = async (entry) => entry.value !== void 0 ? { ...entry, value: await resolveExternalSlots(entry.value, slotContext) } : entry;
        const [resolvedWindows, resolvedPanels] = await Promise.all([Promise.all((response.windows ?? []).map(resolveIfChanged)), Promise.all((response.panels ?? []).map(resolveIfChanged))]);
        if (generation !== refreshGenerationRef.current) return;
        applyUiRefreshResponseToCache(cache, { ...response, windows: resolvedWindows, panels: resolvedPanels });
        if (response.requestedEffects?.length) await applyHostEffects(response.requestedEffects, nextSession);
      }
      if (contributionsJson) {
        const contributionsPushKey = `${nextSession.instanceId}::${contributionsJson}`;
        if (contributionsPushKey !== contributionsJsonRef.current) {
          contributionsJsonRef.current = contributionsPushKey;
          const pluginEntry2 = loadedPlugins.find((entry) => entry.handle.pluginId === nextSession.pluginId);
          if (pluginEntry2) {
            try {
              const wire = encodeActionWire({ controllerId: nextSession.app.controllerId, action: "setContributions", args: { json: contributionsJson } });
              await pluginEntry2.handle.handleAction(nextSession.instanceId, wire, nextSession.viewState);
            } catch (error2) {
              console.warn("[DEBUG] setContributions push skipped", error2 instanceof Error ? error2.message : String(error2));
            }
          }
        }
      }
      if (appRegistrationsJson) {
        const appRegistrationsPushKey = `${nextSession.instanceId}::${appRegistrationsJson}`;
        if (appRegistrationsPushKey !== appRegistrationsJsonRef.current) {
          appRegistrationsJsonRef.current = appRegistrationsPushKey;
          const pluginEntry2 = loadedPlugins.find((entry) => entry.handle.pluginId === nextSession.pluginId);
          if (pluginEntry2) {
            try {
              const wire = encodeActionWire({ controllerId: nextSession.app.controllerId, action: "setAppRegistrations", args: { json: appRegistrationsJson } });
              await pluginEntry2.handle.handleAction(nextSession.instanceId, wire, nextSession.viewState);
            } catch (error2) {
              console.warn("[DEBUG] setAppRegistrations push skipped", error2 instanceof Error ? error2.message : String(error2));
            }
          }
        }
      }
      dispatch({
        type: "SET_WINDOW_UI_BY_WINDOW_ID",
        value: (current) => mergeRecordPreservingIdentity(
          current,
          windowInstances.map((instance) => [instance.id, cache.get(`window:${instance.id}`)?.value ?? current[instance.id] ?? pendingWindowUiNode()])
        )
      });
      const dynamicEngagements = cache.get("engagements")?.value ?? {};
      dispatch({
        type: "SET_WINDOW_ENGAGEMENTS_BY_WINDOW_ID",
        value: (current) => mergeRecordPreservingIdentity(current, Object.entries(dynamicEngagements))
      });
      const dynamicMeasures = cache.get("measures")?.value ?? {};
      dispatch({
        type: "SET_WINDOW_MEASURES_BY_WINDOW_ID",
        value: (current) => mergeRecordPreservingIdentity(current, Object.entries(dynamicMeasures))
      });
      const dynamicToolMeasures = cache.get("tools")?.value ?? {};
      dispatch({
        type: "SET_TOOL_MEASURES_BY_TOOL_ID",
        value: (current) => mergeRecordPreservingIdentity(current, Object.entries(dynamicToolMeasures))
      });
      const freshAppLabelsOverlay = normalizeAppLabelsOverlay(cache.get("labels")?.value);
      dispatch({ type: "SET_APP_LABELS_OVERLAY", value: (current) => preserveJsonIdentity(current, freshAppLabelsOverlay) });
      dispatch({
        type: "SET_PANEL_UI_BY_KEY",
        value: (current) => mergeRecordPreservingIdentity(
          current,
          panelTabLeaves.filter((tab) => tab.bodyKey).map((tab) => [panelTabKindId(tab.kind), cache.get(`panel:${panelTabKindId(tab.kind)}`)?.value ?? current[panelTabKindId(tab.kind)] ?? pendingPanelUiNode()])
        )
      });
      if (isSessionSwitch && layoutSeed) {
        layoutSeedKeyRef.current = layoutSeedKey;
        extraWindowInstancesRef.current = layoutSeed.extraInstances;
        extraWindowCounterRef.current = layoutSeed.extraInstances.length;
        dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: layoutSeed.extraInstances });
        dispatch({ type: "SET_SHELL_LAYOUT", value: layoutSeed.modeLayout });
        dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: null });
      }
    },
    // 🐢️ `applyHostEffects` is declared later in this component (its own deps need `updateSpacePanel`/
    // `syncSpawnedPluginDocument`, declared later still) — referencing it here in the body only (never
    // added to this array) avoids a temporal-dead-zone reference-before-init; safe because this callback
    // is only ever invoked after render completes, by which point `applyHostEffects` is initialized.
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [appLabelsOverlay, injectActiveTool, loadedPlugins, uiLocale, uiTerminology]
  );
  useEffect(() => {
    const windowKinds = session?.app.windowKinds;
    if (!windowKinds) return;
    dispatch({
      type: "SET_SHELL_LAYOUT",
      value: (current) => current ? retitleWindowLayoutNode(current, windowKinds, extraWindowInstancesRef.current, uiTerminology, uiLocale) : current
    });
    dispatch({
      type: "SET_EXTRA_WINDOW_INSTANCES",
      value: (current) => {
        const next = current.map((entry) => {
          const kind = windowKinds.find((k) => k.id === entry.windowKindId || k.id === entry.id);
          const title = kind ? resolveManifestLabel(kind.label, uiTerminology, uiLocale) : entry.title;
          return { ...entry, title };
        });
        extraWindowInstancesRef.current = next;
        return next;
      }
    });
  }, [uiTerminology, uiLocale]);
  const refreshSpawnedUi = useCallback(
    async (spawned, viewState, scopeArg = { kind: "full" }) => {
      if (scopeArg.kind === "none") return;
      const generation = ++spawnedRefreshGenerationRef.current;
      const pluginEntry2 = loadedPlugins.find((entry) => entry.handle.pluginId === spawned.pluginId);
      const plugin = pluginEntry2?.handle;
      const app = pluginEntry2?.manifest.apps.find((candidate) => candidate.id === spawned.appId);
      if (!plugin || !app) {
        console.warn("[os-shell] refreshSpawnedUi: plugin/app unavailable", { pluginId: spawned.pluginId, appId: spawned.appId });
        dispatch({ type: "SET_SPAWNED_WINDOW_UI", value: { type: "text", value: `Plugin unavailable: ${spawned.pluginId}/${spawned.appId}` } });
        dispatch({ type: "SET_SPAWNED_WINDOW_ENGAGEMENTS", value: {} });
        dispatch({ type: "SET_SPAWNED_WINDOW_MEASURES", value: {} });
        return;
      }
      const spawnedSeed = `${spawned.pluginId}:${spawned.appId}:${spawned.instanceId}`;
      if (spawnedLayoutSeedRef.current !== spawnedSeed) {
        spawnedLayoutSeedRef.current = spawnedSeed;
        spawnedUiRefreshCacheRef.current = /* @__PURE__ */ new Map();
      }
      const cache = spawnedUiRefreshCacheRef.current;
      const contributionsJson = buildContributionsJson(loadedPlugins.map((entry) => ({ pluginId: entry.handle.pluginId, manifest: entry.manifest })));
      const bodyKey = resolveCanvasBodyKey(app);
      const fullViewState = injectActiveUtility(
        { ...viewState, contributionsJson, locale: uiLocale, terminology: uiTerminology, windowId: bodyKey, windowInstances: [{ id: bodyKey, windowKindId: bodyKey }] },
        spawned.id
      );
      const singleWindowKind = [{ id: bodyKey, bodyKey }];
      const request = buildUiRefreshRequest({ kind: "full" }, singleWindowKind, [], fullViewState, cache);
      if (request) {
        const response = await plugin.refreshUi(spawned.instanceId, request);
        if (generation !== spawnedRefreshGenerationRef.current) return;
        applyUiRefreshResponseToCache(cache, response);
      }
      const ui = cache.get(`window:${bodyKey}`)?.value ?? pendingWindowUiNode();
      const dynamicEngagements = cache.get("engagements")?.value ?? {};
      const dynamicMeasures = cache.get("measures")?.value ?? {};
      dispatch({ type: "SET_SPAWNED_WINDOW_UI", value: (current) => preserveJsonIdentity(current ?? void 0, ui) });
      dispatch({ type: "SET_SPAWNED_WINDOW_ENGAGEMENTS", value: dynamicEngagements });
      dispatch({ type: "SET_SPAWNED_WINDOW_MEASURES", value: dynamicMeasures });
    },
    [injectActiveUtility, loadedPlugins, uiLocale, uiTerminology]
  );
  const sessionIdentityKey = session ? `${session.pluginId}:${session.app.id}:${session.instanceId}` : null;
  useEffect(() => {
    const current = sessionRef.current;
    if (!current) return;
    void refreshUi(current).catch((renderError) => {
      console.error("[DEBUG] render failed", renderError);
      dispatch({ type: "SET_ERROR", value: renderError instanceof Error ? renderError.message : String(renderError) });
    });
  }, [loadedPlugins, refreshUi, sessionIdentityKey]);
  useEffect(() => {
    if (!studioMode || !session) {
      dispatch({ type: "SET_SPAWNED_WINDOW_UI", value: null });
      dispatch({ type: "SET_SPAWNED_WINDOW_ENGAGEMENTS", value: {} });
      dispatch({ type: "SET_SPAWNED_WINDOW_MEASURES", value: {} });
      return;
    }
    const activeSpawned = panel?.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId);
    if (!activeSpawned) {
      dispatch({ type: "SET_SPAWNED_WINDOW_UI", value: null });
      dispatch({ type: "SET_SPAWNED_WINDOW_ENGAGEMENTS", value: {} });
      dispatch({ type: "SET_SPAWNED_WINDOW_MEASURES", value: {} });
      return;
    }
    void refreshSpawnedUi(activeSpawned, session.viewState).catch((renderError) => {
      console.error("[DEBUG] spawned render failed", renderError);
      dispatch({ type: "SET_SPAWNED_WINDOW_UI", value: null });
    });
  }, [loadedPlugins, panel, refreshSpawnedUi, session, studioMode]);
  const updateSpacePanel = useCallback((panelState) => {
    dispatch({
      type: "SET_SESSION",
      value: (current) => {
        if (!current) return current;
        return { ...current, viewState: { ...current.viewState, panelJson: panelJsonFromState(panelState) } };
      }
    });
  }, []);
  const switchToManagedApp = useCallback(
    async (appId2, viewState) => {
      const sPlugin = hostConfig ? loadedPlugins.find((entry) => entry.handle.pluginId === hostConfig.pluginId) : void 0;
      const app = sPlugin?.manifest.apps.find((candidate) => candidate.id === appId2);
      if (!sPlugin || !app) return null;
      if (session?.pluginId === sPlugin.handle.pluginId && session.app.id === appId2) {
        if (!viewState) return session;
        const nextSession2 = { ...session, viewState };
        dispatch({ type: "SET_SESSION", value: nextSession2 });
        await refreshUi(nextSession2);
        return nextSession2;
      }
      const instanceId = await sPlugin.handle.createApp(app.id);
      const nextViewState = viewState ?? {
        activeModeId: app.defaultModeId ?? app.modes[0]?.id,
        panelJson: panelJsonFromState(buildSpacePanelState([], []))
      };
      const nextSession = { pluginId: sPlugin.handle.pluginId, instanceId, app, viewState: nextViewState };
      dispatch({ type: "SET_SESSION", value: nextSession });
      const seeded = applyFrameworkLayoutSeed(app.defaultLayout, app.windowKinds, appLabelsOverlay, uiTerminology, uiLocale);
      extraWindowInstancesRef.current = seeded.extraInstances;
      extraWindowCounterRef.current = seeded.extraInstances.length;
      dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: seeded.extraInstances });
      dispatch({ type: "SET_SHELL_LAYOUT", value: seeded.modeLayout });
      dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: null });
      if (appId2 === landingAppId) {
        openSpaceIdRef.current = null;
        openInstanceIdRef.current = null;
      }
      await refreshUi(nextSession);
      return nextSession;
    },
    [loadedPlugins, refreshUi, session, appLabelsOverlay, hostConfig, landingAppId, uiTerminology, uiLocale]
  );
  const syncSpawnedPluginDocument = useCallback(async (plugin, app, pluginInstanceId, documentJson, viewState) => {
    try {
      const document2 = JSON.parse(documentJson);
      await plugin.handleAction(pluginInstanceId, encodeActionWire({ controllerId: app.controllerId, action: "setDocument", args: { document: document2 } }), viewState);
    } catch (syncError) {
      console.error("[DEBUG] spawned program document sync failed", syncError);
    }
  }, []);
  const ensureSpawnedPlugin = useCallback(
    async (program, label, osInstanceId, documentJson, sourceViewState) => {
      const pluginEntry2 = loadedPlugins.find((entry) => entry.handle.pluginId === program.pluginId);
      if (!pluginEntry2 || !session) return null;
      const app = pluginEntry2.manifest.apps.find((candidate) => candidate.id === program.appId);
      const currentPanel = parsePanelState(sourceViewState ?? session.viewState) ?? buildSpacePanelState([], []);
      const existing = osInstanceId ? currentPanel.spawnedApps.find((entry) => entry.id === osInstanceId) : currentPanel.spawnedApps.find((entry) => entry.appId === program.appId && entry.pluginId === program.pluginId);
      if (existing) {
        if (documentJson && app) {
          await syncSpawnedPluginDocument(pluginEntry2.handle, app, existing.instanceId, documentJson, sourceViewState ?? session.viewState);
        }
        return studioPanelFocusingSpawned(currentPanel, existing);
      }
      const instanceId = await pluginEntry2.handle.createApp(program.appId);
      if (documentJson && app) {
        await syncSpawnedPluginDocument(pluginEntry2.handle, app, instanceId, documentJson, sourceViewState ?? session.viewState);
      }
      const spawnedId = osInstanceId ?? `${program.pluginId}-${instanceId}`;
      return studioPanelFocusingSpawned(currentPanel, {
        id: spawnedId,
        pluginId: program.pluginId,
        instanceId,
        appId: program.appId,
        label: label ?? program.label,
        document: program.document
      });
    },
    [loadedPlugins, session, syncSpawnedPluginDocument]
  );
  const applyHostEffects = useCallback(
    async (effects, baseSession, uiScope = { kind: "full" }) => {
      let nextViewState = baseSession.viewState;
      for (const effect of effects) {
        if (effect === "requestSync") continue;
        if ("setPanel" in effect) {
          nextViewState = { ...nextViewState, panelJson: effect.setPanel.panelJson };
          continue;
        }
        if ("setActiveUtility" in effect) {
          const { windowId, utilityId } = effect.setActiveUtility;
          setActiveUtilityForWindow(windowId, utilityId || null);
          if (utilityId && activeToolIdRef.current) {
            activeToolIdRef.current = null;
            dispatch({ type: "SET_ACTIVE_TOOL", toolId: null });
          }
          if (windowId === activeWindowIdRef.current) nextViewState = { ...nextViewState, activeUtilityId: utilityId || void 0, activeToolId: utilityId ? void 0 : nextViewState.activeToolId };
          continue;
        }
        if ("setActiveTool" in effect) {
          const { toolId } = effect.setActiveTool;
          activeToolIdRef.current = toolId || null;
          dispatch({ type: "SET_ACTIVE_TOOL", toolId: toolId || null });
          if (toolId) clearAllWindowUtilities();
          nextViewState = { ...nextViewState, activeToolId: toolId || void 0, activeUtilityId: toolId ? void 0 : nextViewState.activeUtilityId };
          continue;
        }
        if ("patchWorld3dChrome" in effect) {
          const { selectionJson, vorticesJson, documentSelectedIds, documentHighlightedIds } = effect.patchWorld3dChrome;
          const patch = { selectionJson, vorticesJson };
          const windowInstances = sessionWindowInstances(baseSession.app, extraWindowInstancesRef.current);
          const documentPanelKey = panelTabKindId(FRAMEWORK_PANEL_TAB_DOCUMENT_ID);
          dispatch({
            type: "SET_WINDOW_UI_BY_WINDOW_ID",
            value: (current) => mergeRecordPreservingIdentity(
              current,
              windowInstances.map((instance) => {
                const node = current[instance.id];
                return [instance.id, node ? patchWorld3dChromeOntoNode(node, patch) : node];
              })
            )
          });
          dispatch({
            type: "SET_PANEL_UI_BY_KEY",
            value: (current) => {
              const documentNode = current[documentPanelKey];
              if (!documentNode) return current;
              return mergeRecordPreservingIdentity(current, [[documentPanelKey, patchDocumentTreeSelectedIds(documentNode, documentSelectedIds, documentHighlightedIds)]]);
            }
          });
          const cache = uiRefreshCacheRef.current;
          for (const instance of windowInstances) {
            const cached = cache.get(`window:${instance.id}`);
            if (cached?.value) {
              cache.set(`window:${instance.id}`, { hash: cached.hash, value: patchWorld3dChromeOntoNode(cached.value, patch) });
            }
          }
          const documentCached = cache.get(`panel:${documentPanelKey}`);
          if (documentCached?.value) {
            cache.set(`panel:${documentPanelKey}`, {
              hash: documentCached.hash,
              value: patchDocumentTreeSelectedIds(documentCached.value, documentSelectedIds, documentHighlightedIds)
            });
          }
          continue;
        }
        if ("openDialog" in effect) {
          const { dialogId, args } = effect.openDialog;
          if (baseSession.app.dialogs?.some((entry) => entry.id === dialogId)) {
            dispatch({ type: "SET_DIALOG", value: { dialogId, seedArgs: args } });
          } else {
            console.error(`[os-shell] openDialog: app ${baseSession.app.id} declares no dialog "${dialogId}"`);
          }
          continue;
        }
        if ("navigate" in effect) {
          navigateHistory(effect.navigate.uri);
          continue;
        }
        if ("loadDocument" in effect) {
          const pluginEntry2 = loadedPlugins.find((entry) => entry.handle.pluginId === baseSession.pluginId);
          const payload = effect.loadDocument;
          if (payload.pack && payload.spr && pluginEntry2?.handle.loadAppDocumentPack) {
            const packBytes = coerceWireBytes(payload.pack);
            const sprBytes = coerceWireBytes(payload.spr);
            console.log("[DEBUG] loadDocument pack/spr for instance", baseSession.instanceId, "pack", packBytes.length, "spr", sprBytes.length);
            await pluginEntry2.handle.loadAppDocumentPack(baseSession.instanceId, packBytes, sprBytes);
          } else if (payload.documentJson && pluginEntry2?.handle.loadAppDocument) {
            console.log("[DEBUG] loadDocument for instance", baseSession.instanceId, "bytes", payload.documentJson.length);
            await pluginEntry2.handle.loadAppDocument(baseSession.instanceId, payload.documentJson);
          } else {
            console.error("[os-shell] loadDocument: program has no pack/json loader", baseSession.pluginId, Object.keys(payload));
          }
          continue;
        }
        if ("openExternalUrl" in effect) {
          window.open(effect.openExternalUrl.url, "_blank", "noopener,noreferrer");
          continue;
        }
        if ("downloadMediaExport" in effect) {
          const { filename, mimeType, data, encoding } = effect.downloadMediaExport;
          downloadMediaExport(filename, mimeType, data, encoding);
          continue;
        }
        if ("iconRenderExport" in effect) {
          for (const item of effect.iconRenderExport.items) {
            try {
              const result = await iconRenderPort.render(item.request);
              downloadDataUrl(item.filename, result.dataUrl);
            } catch (error2) {
              console.error(`icon render export failed for ${item.filename}`, error2);
            }
          }
          continue;
        }
        if ("requestFileOpen" in effect) {
          const { accept, readAs, importAction, multiple } = effect.requestFileOpen;
          const opened = await requestFileOpen(accept || ".spk,.dsl,.ops,application/octet-stream", readAs, multiple);
          if (opened.length > 0) {
            const pluginEntry2 = loadedPlugins.find((entry) => entry.handle.pluginId === baseSession.pluginId);
            if (pluginEntry2) {
              await dispatchOpenedFiles(opened, importAction, Boolean(multiple), makeEffectDispatchOne(pluginEntry2, baseSession, applyHostEffects));
            }
          }
          continue;
        }
        if ("dispatchAction" in effect) {
          const { action: dispatchActionId, args: dispatchArgs, delayMs } = effect.dispatchAction;
          const pluginEntry2 = loadedPlugins.find((entry) => entry.handle.pluginId === baseSession.pluginId);
          if (pluginEntry2) {
            scheduleDispatchAction(dispatchActionId, dispatchArgs, delayMs, makeEffectDispatchOne(pluginEntry2, baseSession, applyHostEffects));
          }
          continue;
        }
        if ("requestMediaFrames" in effect) {
          const { accept, payload, frameAction, doneAction, fallbackAction, sampleStride, maxFrames, maxLongEdgePx, fpsHint, args } = effect.requestMediaFrames;
          const pluginEntry2 = loadedPlugins.find((entry) => entry.handle.pluginId === baseSession.pluginId);
          if (pluginEntry2) {
            await runRequestMediaFrames(
              {
                frameAction,
                doneAction,
                fallbackAction,
                sampleStride: sampleStride ?? 0,
                maxFrames: maxFrames ?? 0,
                maxLongEdgePx: maxLongEdgePx ?? 0,
                fpsHint: fpsHint ?? 0,
                args
              },
              accept,
              payload,
              makeEffectDispatchOne(pluginEntry2, baseSession, applyHostEffects)
            );
          }
          continue;
        }
        if ("requestPluginExchange" in effect) {
          const { pluginId, appId: appId2, requestJson, responseAction } = effect.requestPluginExchange;
          const request = JSON.parse(requestJson);
          const contributor = loadedPlugins.find((entry) => entry.handle.pluginId === pluginId);
          if (contributor && request.operatorId && request.inputJson != null && request.nodeHash != null) {
            try {
              const bim = await import("/@fs/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/📦️packages/🦀️rust/pkg/semio_s_plugin_flow_extension_bim.js");
              const outputJson = typeof bim.evaluate === "function" ? bim.evaluate(request.operatorId, request.inputJson) : "";
              console.log("[DEBUG] requestPluginExchange resolved extension eval", { pluginId, appId: appId2, operatorId: request.operatorId, nodeHash: request.nodeHash });
              await makeEffectDispatchOne(pluginEntry, baseSession, applyHostEffects)(responseAction, {
                nodeHash: request.nodeHash,
                outputJson
              });
            } catch (error2) {
              console.warn("[os-shell] requestPluginExchange failed", { pluginId, appId: appId2, error: error2 });
            }
          }
          continue;
        }
        if ("spawnPluginInstance" in effect) {
          const { pluginId, appId: appId2, osInstanceId, label, documentJson } = effect.spawnPluginInstance;
          const currentPanel = parsePanelState(nextViewState) ?? buildSpacePanelState([], []);
          const catalog = currentPanel.programs.length > 0 ? currentPanel.programs : [];
          const program = catalog.find((entry) => entry.pluginId === pluginId && entry.appId === appId2) ?? catalog.find((entry) => entry.pluginId === pluginId);
          if (program) {
            const nextPanel = await ensureSpawnedPlugin(program, label, osInstanceId, documentJson, nextViewState);
            if (nextPanel) nextViewState = viewStateWithSpacePanel(nextViewState, nextPanel);
          }
          continue;
        }
        if ("openPluginInstance" in effect) {
          const { pluginId, appId: appId2, osInstanceId } = effect.openPluginInstance;
          const currentPanel = parsePanelState(nextViewState) ?? buildSpacePanelState([], []);
          const catalog = currentPanel.programs.length > 0 ? currentPanel.programs : [];
          const program = catalog.find((entry) => entry.pluginId === pluginId && entry.appId === appId2) ?? catalog.find((entry) => entry.pluginId === pluginId);
          if (program) {
            const nextPanel = await ensureSpawnedPlugin(program, void 0, osInstanceId, void 0, nextViewState);
            if (nextPanel) {
              nextViewState = viewStateWithSpacePanel(nextViewState, nextPanel);
              console.log("[DEBUG] openPluginInstance focused spawned app", {
                pluginId,
                appId: appId2,
                osInstanceId,
                activeSpawnedId: nextPanel.activeSpawnedId,
                spawnedCount: nextPanel.spawnedApps.length
              });
            }
            if (osInstanceId && openSpaceIdRef.current) {
              openInstanceIdRef.current = osInstanceId;
              navigateHistory(`/spaces/${openSpaceIdRef.current}/instances/${osInstanceId}`);
            }
          } else {
            console.warn(
              "[os-shell] openPluginInstance: no program matches",
              { pluginId, appId: appId2 },
              "available:",
              catalog.map((entry) => `${entry.pluginId}/${entry.appId}`)
            );
          }
          continue;
        }
      }
      const nextSession = { ...baseSession, viewState: nextViewState };
      const isSpawnedPluginSession = studioMode && session && baseSession.pluginId !== session.pluginId;
      dispatch({
        type: "SET_SESSION",
        value: (current) => {
          if (!current) return nextSession;
          if (isSpawnedPluginSession) return current.viewState === nextViewState ? current : { ...current, viewState: nextViewState };
          if (current.instanceId !== nextSession.instanceId) return current;
          return current.viewState === nextViewState ? current : { ...current, viewState: nextViewState };
        }
      });
      if (isSpawnedPluginSession) {
        const spawned = parsePanelState(nextViewState)?.spawnedApps.find((entry) => entry.pluginId === baseSession.pluginId && entry.instanceId === baseSession.instanceId);
        if (spawned) await refreshSpawnedUi(spawned, nextViewState, uiScope);
      } else if (session?.instanceId === nextSession.instanceId || baseSession.instanceId === nextSession.instanceId) {
        await refreshUi(nextSession, uiScope);
      }
    },
    [clearAllWindowUtilities, ensureSpawnedPlugin, loadedPlugins, navigateHistory, refreshSpawnedUi, refreshUi, session, setActiveUtilityForWindow, studioMode]
  );
  const applyShellUri = useCallback(
    async (uri, preservedViewState) => {
      const currentSession = sessionRef.current;
      if (!hostConfig || !currentSession || loadedPlugins.length === 0) return;
      const path = uri.split("?")[0] ?? "/";
      const route = parseShellRoute(path);
      const sPlugin = loadedPlugins.find((entry) => entry.handle.pluginId === hostConfig.pluginId)?.handle;
      if (!sPlugin) return;
      if (route.kind === "landing") {
        openSpaceIdRef.current = null;
        openInstanceIdRef.current = null;
        if (currentSession.app.id !== hostConfig.landingAppId) await switchToManagedApp(hostConfig.landingAppId, preservedViewState);
        return;
      }
      if (route.kind === "notFound") {
        openSpaceIdRef.current = null;
        openInstanceIdRef.current = null;
        return;
      }
      const { spaceId, instanceId } = route;
      const studioChanged = openSpaceIdRef.current !== spaceId;
      openSpaceIdRef.current = spaceId;
      const studioSession = currentSession.app.id === hostConfig.hostAppId ? currentSession : await switchToManagedApp(hostConfig.hostAppId, preservedViewState);
      if (!studioSession) return;
      const studioControllerId = studioSession.app.controllerId;
      if (studioChanged) {
        openInstanceIdRef.current = null;
        console.log("[DEBUG] applyShellUri openSpace", spaceId);
        const openResponse = await sPlugin.handleAction(studioSession.instanceId, encodeActionWire({ controllerId: studioControllerId, action: "openSpace", args: { spaceId } }), studioSession.viewState);
        await applyHostEffects(openResponse.requestedEffects ?? [], studioSession, resolveUiDirtyScope(openResponse.uiScope));
      }
      if (openInstanceIdRef.current === (instanceId ?? null)) return;
      openInstanceIdRef.current = instanceId ?? null;
      if (instanceId) {
        const response = await sPlugin.handleAction(studioSession.instanceId, encodeActionWire({ controllerId: studioControllerId, action: "openInstance", args: { instanceId } }), studioSession.viewState);
        await applyHostEffects(response.requestedEffects ?? [], studioSession, resolveUiDirtyScope(response.uiScope));
      } else {
        const response = await sPlugin.handleAction(studioSession.instanceId, encodeActionWire({ controllerId: studioControllerId, action: "closeFocusedInstance" }), studioSession.viewState);
        const currentPanel = parsePanelState(studioSession.viewState) ?? buildSpacePanelState([], []);
        updateSpacePanel(buildSpacePanelState(currentPanel.programs, currentPanel.spawnedApps, currentPanel.activePanelTab, void 0));
        await applyHostEffects(response.requestedEffects ?? [], studioSession, resolveUiDirtyScope(response.uiScope));
      }
    },
    [applyHostEffects, loadedPlugins, refreshUi, hostConfig, switchToManagedApp, updateSpacePanel]
  );
  useEffect(() => {
    if (!studioMode || loadedPlugins.length === 0) return;
    void applyShellUri(shellUri).catch((uriError) => {
      console.error("[DEBUG] shell uri apply failed", uriError);
    });
  }, [applyShellUri, loadedPlugins.length, shellUri, studioMode]);
  const resolveSyncTargetSession = useCallback(() => {
    if (!session) return null;
    if (studioMode && panel?.activeSpawnedId) {
      const spawned = panel.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId);
      if (spawned) {
        const app = loadedPlugins.find((entry) => entry.handle.pluginId === spawned.pluginId)?.manifest.apps.find((candidate) => candidate.id === spawned.appId);
        if (app) return { pluginId: spawned.pluginId, instanceId: spawned.instanceId, app, viewState: session.viewState };
      }
    }
    return session;
  }, [loadedPlugins, panel, session, studioMode]);
  const openDocument = useCallback(
    async (ref, bindings) => {
      const targetSession = resolveSyncTargetSession();
      if (!targetSession) return;
      const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === targetSession.pluginId)?.handle;
      if (!plugin) return;
      const worker = ensureBackboneWorker();
      openDocumentSessionsRef.current.set(ref.documentId, { session: targetSession, plugin });
      pluginBackboneRouteUnregistersRef.current.get(ref.documentId)?.();
      pluginBackboneRouteUnregistersRef.current.set(ref.documentId, registerPluginBackboneRoute(ref.documentId, relayPluginBackboneMessage));
      const request = {
        kind: "open",
        documentId: ref.documentId,
        schema: ref.schema,
        bindings,
        watchExternal: true,
        actor: shellActorIdRef.current
      };
      worker.postMessage(request);
      const uri = `actor://${ref.documentId}`;
      if (plugin.attachBackbone) await plugin.attachBackbone(targetSession.instanceId, uri);
      dispatch({ type: "SET_SYNC_BACKBONE_URI", value: uri });
      dispatch({ type: "SET_SYNC_CARD_KIND", value: null });
    },
    [loadedPlugins, relayPluginBackboneMessage, resolveSyncTargetSession]
  );
  const closeDocument = useCallback((documentId) => {
    const entry = openDocumentSessionsRef.current.get(documentId);
    if (entry?.plugin.detachBackbone) void entry.plugin.detachBackbone(entry.session.instanceId);
    openDocumentSessionsRef.current.delete(documentId);
    pluginBackboneRouteUnregistersRef.current.get(documentId)?.();
    pluginBackboneRouteUnregistersRef.current.delete(documentId);
    const request = { kind: "close", documentId };
    backboneWorkerRef.current?.postMessage(request);
  }, []);
  const attachSyncBackbone = useCallback(
    async (uri) => {
      const targetSession = resolveSyncTargetSession();
      if (!targetSession) return;
      const documentId = syncDocumentId(targetSession, panel, studioMode);
      const bindings = uri.startsWith("remote://") ? (() => {
        const rest = uri.slice("remote://".length);
        const slash = rest.indexOf("/");
        const baseUrl = slash > 0 ? `http://${rest.slice(0, slash)}` : `http://${rest}`;
        const spaceId = slash > 0 ? rest.slice(slash + 1) || "default" : "default";
        return [{ kind: "hub", baseUrl, spaceId }];
      })() : uri.startsWith("folder://") ? [{ kind: "folder", path: uri.slice("folder://".length) }] : uri.startsWith("file://") ? [{ kind: "folder", path: uri.slice("file://".length).replace(/\/[^/]*$/, "") }] : [];
      await openDocument({ documentId, schema: targetSession.app.document.join(".") }, bindings);
    },
    [openDocument, panel, resolveSyncTargetSession, studioMode]
  );
  const detachSyncBackbone = useCallback(() => {
    if (syncBackboneUri) closeDocument(syncBackboneUri.replace(/^actor:\/\//, ""));
    dispatch({ type: "SET_SYNC_BACKBONE_URI", value: null });
    dispatch({ type: "SET_SYNC_CARD_KIND", value: null });
  }, [closeDocument, syncBackboneUri]);
  const spawnProgram = useCallback(
    async (program) => {
      const pluginEntry2 = loadedPlugins.find((entry) => entry.handle.pluginId === program.pluginId);
      if (!pluginEntry2 || !session) return;
      const instanceId = await pluginEntry2.handle.createApp(program.appId);
      const currentPanel = parsePanelState(session.viewState) ?? buildSpacePanelState([], []);
      const spawnedId = `${program.pluginId}-${instanceId}`;
      updateSpacePanel(
        studioPanelFocusingSpawned(currentPanel, {
          id: spawnedId,
          pluginId: program.pluginId,
          instanceId,
          appId: program.appId,
          label: program.label,
          document: program.document
        })
      );
    },
    [loadedPlugins, session, updateSpacePanel]
  );
  const onAction = useCallback(
    (action) => {
      if (action.controllerId === "recovery") {
        const args = typeof action.args === "object" && action.args != null ? action.args : {};
        const pluginId = args.pluginId ?? primaryPluginId;
        if (!pluginId) return;
        if (action.action === "recovery.restartApp") {
          dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: "restarting" });
          void reloadPlugin(pluginId);
          return;
        }
        if (action.action === "recovery.disablePlugin") {
          dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: "quarantined" });
          if (pluginId !== primaryPluginId) void uninstallPlugin(pluginId);
          return;
        }
        if (action.action === "recovery.showDiagnostics") {
          console.log("[DEBUG] recovery diagnostics", { pluginId, supervisor: pluginSupervisorById[pluginId] });
          return;
        }
      }
      if (!session) return;
      if (action.action === START_INTRODUCTION_ACTION_ID) {
        dispatch({ type: "SET_INTRODUCTION_STEP", value: 0 });
        return;
      }
      if (action.action === START_TUTORIAL_ACTION_ID) {
        const args = typeof action.args === "object" && action.args != null ? action.args : {};
        if (typeof args.tutorialId === "string") startTutorialRef.current(args.tutorialId);
        return;
      }
      if (action.action === RECORD_TUTORIAL_ACTION_ID) {
        toggleTutorialRecordingRef.current();
        return;
      }
      if (tutorialPlayingRef.current && !tutorialDrivenRef.current) {
        dispatch({ type: "SET_TUTORIAL_PLAYING", value: false });
        dispatch({ type: "SET_TUTORIAL_DEVIATED", value: true });
      }
      if (tutorialRecordingRef.current && !tutorialDrivenRef.current) {
        if (!TUTORIAL_RECORDING_EXCLUDED_ACTION_IDS.has(action.action)) {
          tutorialRecorderRef.current?.recordEvent({ kind: "action", action: action.action, args: action.args });
        }
      }
      if (action.action === NOTE_WORLD_NAVIGATION_ACTION_ID) {
        const args = typeof action.args === "object" && action.args != null ? action.args : {};
        const windowId = typeof args.windowId === "string" ? args.windowId : "";
        const gestures = Array.isArray(args.gestures) ? args.gestures : [];
        if (windowId) {
          const windowKindId = sessionWindowInstances(session.app, extraWindowInstancesRef.current).find((instance) => instance.id === windowId)?.windowKindId ?? windowId;
          for (const gesture of gestures) {
            completeIntroductionInteraction(
              (interaction) => interaction.on.kind === gesture && introductionTargetsWindow(windowId, windowKindId, interaction.on.id),
              windowElementId(windowId)
            );
          }
        }
        return;
      }
      if (action.action === SET_ACTIVE_UTILITY_ACTION_ID) {
        const args = typeof action.args === "object" && action.args != null ? action.args : {};
        const windowId = typeof args.windowId === "string" && args.windowId ? args.windowId : activeWindowIdRef.current ?? "";
        if (!windowId) return;
        const requested = typeof args.utilityId === "string" ? args.utilityId : "";
        const next = resolveUtilityActivation(activeUtilityByWindowIdRef.current[windowId], requested);
        setActiveUtilityForWindow(windowId, next);
        if (next && activeToolIdRef.current) {
          activeToolIdRef.current = null;
          dispatch({ type: "SET_ACTIVE_TOOL", toolId: null });
        }
        if (next) completeIntroductionInteraction((interaction) => interaction.on.kind === "utility" && interaction.on.id === next);
        const pluginEntry3 = findPluginForAction(action);
        const program = pluginEntry3?.handle;
        if (plugin) {
          const viewState = { ...session.viewState, activeUtilityId: next ?? void 0, activeToolId: next ? void 0 : activeToolIdRef.current ?? void 0, windowId };
          const forwarded = { controllerId: action.controllerId, action: action.action, args: { utilityId: next } };
          void program.handleAction(session.instanceId, encodeActionWire(forwarded), viewState).then((response) => applyHostEffects(response.requestedEffects ?? [], { ...session, viewState }, resolveUiDirtyScope(response.uiScope))).catch((utilityError) => console.error("[DEBUG] setActiveUtility failed", utilityError));
        }
        return;
      }
      if (action.action === SET_ACTIVE_TOOL_ACTION_ID) {
        const args = typeof action.args === "object" && action.args != null ? action.args : {};
        const requested = typeof args.toolId === "string" ? args.toolId : "";
        const next = resolveUtilityActivation(activeToolIdRef.current, requested);
        activeToolIdRef.current = next;
        dispatch({ type: "SET_ACTIVE_TOOL", toolId: next });
        if (next) clearAllWindowUtilities();
        if (next) completeIntroductionInteraction((interaction) => interaction.on.kind === "tool" && interaction.on.id === next);
        const pluginEntry3 = findPluginForAction(action);
        const program = pluginEntry3?.handle;
        if (plugin) {
          const viewState = { ...session.viewState, activeToolId: next ?? void 0, activeUtilityId: next ? void 0 : session.viewState.activeUtilityId };
          const forwarded = { controllerId: action.controllerId, action: action.action, args: { toolId: next } };
          void program.handleAction(session.instanceId, encodeActionWire(forwarded), viewState).then((response) => applyHostEffects(response.requestedEffects ?? [], { ...session, viewState }, resolveUiDirtyScope(response.uiScope))).catch((toolError) => console.error("[DEBUG] setActiveTool failed", toolError));
        }
        return;
      }
      completeIntroductionInteraction((interaction) => interaction.on.kind === "action" && interaction.on.id === action.action);
      if (action.controllerId === FRAMEWORK_SYNC_CONTROLLER_ID) {
        if (action.action === "selectFile") {
          dispatch({ type: "SET_SYNC_CARD_KIND", value: "file" });
          dispatch({ type: "SET_SYNC_DRAFT_PATH", value: syncBackboneUri?.startsWith("file://") ? syncBackboneUri.slice("file://".length) : "" });
          return;
        }
        if (action.action === "selectFolder") {
          dispatch({ type: "SET_SYNC_CARD_KIND", value: "folder" });
          dispatch({ type: "SET_SYNC_DRAFT_PATH", value: syncBackboneUri?.startsWith("folder://") ? syncBackboneUri.slice("folder://".length) : "" });
          return;
        }
        if (action.action === "selectRemote") {
          dispatch({ type: "SET_SYNC_CARD_KIND", value: "remote" });
          const remote = syncBackboneUri?.startsWith("remote://") ? syncBackboneUri.slice("remote://".length) : "";
          dispatch({ type: "SET_SYNC_DRAFT_PATH", value: remote });
          return;
        }
        if (action.action === "attach") {
          const path = typeof action.args === "object" && action.args != null && "path" in action.args ? String(action.args.path ?? "") : syncDraftPath;
          if (!path.trim()) return;
          const uri = action.args && typeof action.args === "object" && "kind" in action.args ? String(action.args.kind) === "remote" ? (() => {
            const [hostPort, ...rest] = path.split("/");
            const [spaceId, documentId] = rest.length >= 2 ? [rest[0], rest.slice(1).join("/")] : ["default", rest[0] || syncDocumentId(session, panel, studioMode)];
            return buildRemoteBackboneUri(hostPort ?? "127.0.0.1:8787", spaceId, documentId);
          })() : String(action.args.kind) === "folder" ? buildFolderBackboneUri(path) : buildFileBackboneUri(path) : buildFileBackboneUri(path);
          void attachSyncBackbone(uri);
          return;
        }
        if (action.action === "detach") {
          void detachSyncBackbone();
          return;
        }
        return;
      }
      if (studioMode && action.controllerId === landingControllerId && action.action === "importSpace") {
        importSpaceInputRef.current?.click();
        return;
      }
      if (studioMode && action.action === "spawnApp" && action.controllerId !== hostControllerId) {
        const pluginId = typeof action.args === "object" && action.args != null && "pluginId" in action.args ? String(action.args.pluginId ?? "") : "";
        const currentPanel = parsePanelState(session.viewState);
        const program = currentPanel?.programs.find((entry) => entry.pluginId === pluginId);
        if (program) void spawnProgram(program);
        return;
      }
      if (studioMode && action.controllerId === hostControllerId && action.action === "setActivePanelTab") {
        const tabId = typeof action.args === "object" && action.args != null && "tabId" in action.args ? String(action.args.tabId ?? hostCatalogueTabId ?? "") : hostCatalogueTabId ?? "";
        const currentPanel = parsePanelState(session.viewState) ?? buildSpacePanelState([], []);
        updateSpacePanel(buildSpacePanelState(currentPanel.programs, currentPanel.spawnedApps, tabId, currentPanel.activeSpawnedId));
        return;
      }
      const pluginEntry2 = findPluginForAction(action);
      const plugin = pluginEntry2?.handle;
      if (!plugin) return;
      const targetSession = studioMode && action.controllerId !== session.app.controllerId ? (() => {
        const spawned = panel?.spawnedApps.find((entry) => {
          const app2 = loadedPlugins.find((p) => p.handle.pluginId === entry.pluginId)?.manifest.apps.find((a) => a.id === entry.appId);
          return app2?.controllerId === action.controllerId;
        });
        if (!spawned) return session;
        const app = loadedPlugins.find((p) => p.handle.pluginId === spawned.pluginId)?.manifest.apps.find((a) => a.id === spawned.appId);
        if (!app) return session;
        return { pluginId: spawned.pluginId, instanceId: spawned.instanceId, app, viewState: session.viewState };
      })() : session;
      const actionWindowId = typeof action.args === "object" && action.args != null && typeof action.args.windowId === "string" ? action.args.windowId : void 0;
      const dispatchWindowId = actionWindowId ?? activeWindowIdRef.current ?? void 0;
      const dispatchViewState = injectActiveUtility(
        {
          ...targetSession.viewState,
          windowId: dispatchWindowId,
          windowInstances: sessionWindowInstances(targetSession.app, extraWindowInstancesRef.current).map((instance) => ({ id: instance.id, windowKindId: instance.windowKindId }))
        },
        dispatchWindowId
      );
      const declaredAction = targetSession.app.actions?.some((entry) => entry.id === action.action) ?? false;
      if (!declaredAction && !FRAMEWORK_RESERVED_ACTION_IDS.has(action.action)) {
        console.warn("[DEBUG] skipping undeclared action", action.action, targetSession.app.id);
        return;
      }
      const interactiveAction = action.action !== "suggestionsTick" && action.action !== "fillBuildTick";
      if (interactiveAction) beginInteractivePluginAction();
      return plugin.handleAction(targetSession.instanceId, encodeActionWire(action), dispatchViewState).then((response) => applyHostEffects(response.requestedEffects ?? [], { ...targetSession, viewState: dispatchViewState }, resolveUiDirtyScope(response.uiScope))).catch((actionError) => {
        console.error("[DEBUG] action failed", action.action, action.args, actionError);
      }).finally(() => {
        if (interactiveAction) endInteractivePluginAction();
      });
    },
    [
      applyHostEffects,
      attachSyncBackbone,
      clearAllWindowUtilities,
      detachSyncBackbone,
      findPluginForAction,
      injectActiveUtility,
      loadedPlugins,
      panel,
      session,
      setActiveUtilityForWindow,
      spawnProgram,
      studioMode,
      syncBackboneUri,
      syncDraftPath,
      updateSpacePanel,
      hostControllerId,
      landingControllerId,
      hostCatalogueTabId,
      completeIntroductionInteraction,
      primaryPluginId,
      reloadPlugin,
      uninstallPlugin,
      pluginSupervisorById
    ]
  );
  const noteShellCommand = useCallback(
    (commandId, label, detail) => {
      if (!session) return;
      onAction(buildNoteShellCommandAction(session.app.controllerId, commandId, label, detail));
    },
    [session, onAction]
  );
  const onActionRef = useRef(onAction);
  useEffect(() => {
    onActionRef.current = onAction;
  }, [onAction]);
  const onActionStable = useCallback((action) => onActionRef.current(action), []);
  const TUTORIAL_DIRECTOR_TICK_MS = 90;
  const activeTutorial = useMemo(() => activeTutorials.find((tutorial) => tutorial.id === activeTutorialId) ?? null, [activeTutorials, activeTutorialId]);
  const tutorialClockRef = useRef(null);
  if (!tutorialClockRef.current) tutorialClockRef.current = createTutorialClock(activeTutorial?.durationMs ?? 0);
  const tutorialClock = tutorialClockRef.current;
  useEffect(() => () => tutorialClockRef.current?.dispose(), []);
  useEffect(() => {
    tutorialClock.setDurationMs(activeTutorial?.durationMs ?? 0);
  }, [activeTutorial?.durationMs, tutorialClock]);
  useEffect(() => {
    tutorialClock.setRate(tutorialRate);
  }, [tutorialRate, tutorialClock]);
  useEffect(() => {
    if (tutorialPlaying) tutorialClock.play();
    else
      tutorialClock.pause();
  }, [tutorialPlaying, tutorialClock]);
  const uiBridgeCtxRef = useRef({ session, appLabelsOverlay, terminology: uiTerminology, locale: uiLocale });
  uiBridgeCtxRef.current = { session, appLabelsOverlay, terminology: uiTerminology, locale: uiLocale };
  const tutorialLastAppliedMsRef = useRef(0);
  const tutorialDocumentSnapshotRef = useRef(null);
  const prevActiveTutorialIdRef = useRef(null);
  useEffect(() => {
    const previousId = prevActiveTutorialIdRef.current;
    prevActiveTutorialIdRef.current = activeTutorialId;
    if (previousId === activeTutorialId || !session) return;
    const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === session.pluginId)?.handle;
    if (!plugin) return;
    if (activeTutorialId) {
      const def = activeTutorials.find((tutorial) => tutorial.id === activeTutorialId);
      if (!def) return;
      tutorialDrivenRef.current = true;
      void (async () => {
        try {
          if (plugin.readAppDocument) tutorialDocumentSnapshotRef.current = await plugin.readAppDocument(session.instanceId);
        } catch (snapshotError) {
          console.error("[DEBUG] tutorial sandbox snapshot failed", snapshotError);
        }
        try {
          if (def.base.documentJson && plugin.loadAppDocument) await plugin.loadAppDocument(session.instanceId, def.base.documentJson);
          else if (def.base.exampleId) dispatch({ type: "SET_ACTIVE_EXAMPLE_ID", value: def.base.exampleId });
        } catch (loadError) {
          console.error("[DEBUG] tutorial base document load failed", loadError);
        }
        applyTutorialUiSnapshotToShell(dispatch, def.base.ui, uiBridgeCtxRef.current);
        for (const cameraKeyframe of def.base.cameras) getTutorialCameraDriver(cameraKeyframe.windowId)?.set(cameraKeyframe.camera);
        tutorialLastAppliedMsRef.current = 0;
        tutorialClock.seek(0);
        await refreshUi(session, { kind: "full" });
        tutorialDrivenRef.current = false;
      })();
    } else if (previousId) {
      tutorialDrivenRef.current = true;
      void (async () => {
        try {
          const snapshotJson = tutorialDocumentSnapshotRef.current;
          if (snapshotJson && plugin.loadAppDocument) await plugin.loadAppDocument(session.instanceId, snapshotJson);
        } catch (restoreError) {
          console.error("[DEBUG] tutorial sandbox restore failed", restoreError);
        }
        tutorialDocumentSnapshotRef.current = null;
        await refreshUi(session, { kind: "full" });
        tutorialDrivenRef.current = false;
      })();
    }
  }, [activeTutorialId, activeTutorials, session, loadedPlugins, tutorialClock, refreshUi]);
  const applyTutorialSliceToShell = useCallback(
    async (slice, activeSession) => {
      for (const change of slice.uiChanges) applyTutorialUiChangeToShell(dispatch, change, uiBridgeCtxRef.current);
      const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === activeSession.pluginId)?.handle;
      let documentTouched = false;
      for (const documentEvent of slice.document) {
        const kind = documentEvent.kind;
        if (kind.kind === "edit") {
          documentTouched = true;
          const operations = slice.forward ? kind.forwards : kind.backwards;
          if (plugin?.applyOperations) await plugin.applyOperations(activeSession.instanceId, encodeOperationEnvelopesPack(operations));
        } else if (kind.kind === "load") {
          documentTouched = true;
          const documentJson = slice.forward ? kind.documentJson : kind.previousJson;
          if (plugin?.loadAppDocument) await plugin.loadAppDocument(activeSession.instanceId, documentJson);
        } else if (kind.kind === "undo") {
          onActionRef.current({ controllerId: activeSession.app.controllerId, action: slice.forward ? "undo" : "redo" });
        } else if (kind.kind === "redo") {
          onActionRef.current({ controllerId: activeSession.app.controllerId, action: slice.forward ? "redo" : "undo" });
        } else if (kind.kind === "checkpoint") {
          if (slice.forward) onActionRef.current({ controllerId: activeSession.app.controllerId, action: "commitCheckpoint" });
        } else if (kind.kind === "checkoutCheckpoint") {
          onActionRef.current({ controllerId: activeSession.app.controllerId, action: "checkoutCheckpoint", args: { checkpointId: kind.checkpointId } });
        } else if (kind.kind === "switchAlternative") {
          onActionRef.current({ controllerId: activeSession.app.controllerId, action: "switchAlternative", args: { alternativeId: kind.alternativeId } });
        }
      }
      for (const event of slice.events) {
        const kind = event.kind;
        const targetId = kind.kind === "action" ? kind.action : kind.kind === "command" ? kind.command : void 0;
        if (targetId && scope.rootRef.current) celebrateElements(elementIdSelector(targetId), CELEBRATE_STAMP_DURATION_MS, scope.rootRef.current);
      }
      if (documentTouched) await refreshUi(activeSession, { kind: "full" });
    },
    [loadedPlugins, refreshUi]
  );
  useEffect(() => {
    const def = activeTutorial;
    if (!def || !session) return;
    let lastHeavyTickAt = 0;
    const cameraWindowIds = new Set([...def.base.cameras, ...def.tracks.camera].map((keyframe) => keyframe.windowId));
    const unsubscribe = tutorialClock.subscribe(() => {
      const t = tutorialClock.getTimeMs();
      for (const windowId of cameraWindowIds) {
        const pose = tutorialCameraAt(def, windowId, t);
        if (pose) getTutorialCameraDriver(windowId)?.set(pose);
      }
      if (!tutorialClock.isPlaying()) return;
      const now = performance.now();
      if (now - lastHeavyTickAt < TUTORIAL_DIRECTOR_TICK_MS) return;
      lastHeavyTickAt = now;
      const from = tutorialLastAppliedMsRef.current;
      if (from === t) return;
      const slice = tutorialSlice(def, from, t);
      tutorialLastAppliedMsRef.current = t;
      tutorialDrivenRef.current = true;
      void applyTutorialSliceToShell(slice, session).finally(() => {
        tutorialDrivenRef.current = false;
      });
    });
    return unsubscribe;
  }, [activeTutorial, session, tutorialClock, applyTutorialSliceToShell]);
  const seekTutorial = useCallback(
    (ms) => {
      const def = activeTutorial;
      if (!def || !session) return;
      const clamped = Math.min(def.durationMs, Math.max(0, ms));
      const from = tutorialLastAppliedMsRef.current;
      tutorialDrivenRef.current = true;
      void (async () => {
        applyTutorialUiSnapshotToShell(dispatch, composeTutorialUi(def, clamped), uiBridgeCtxRef.current);
        const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === session.pluginId)?.handle;
        const slice = tutorialSlice(def, from, clamped);
        let documentTouched = false;
        for (const documentEvent of slice.document) {
          const kind = documentEvent.kind;
          if (kind.kind === "edit") {
            documentTouched = true;
            const operations = slice.forward ? kind.forwards : kind.backwards;
            if (plugin?.applyOperations) await plugin.applyOperations(session.instanceId, encodeOperationEnvelopesPack(operations));
          } else if (kind.kind === "load") {
            documentTouched = true;
            const documentJson = slice.forward ? kind.documentJson : kind.previousJson;
            if (plugin?.loadAppDocument) await plugin.loadAppDocument(session.instanceId, documentJson);
          }
        }
        const cameraWindowIds = new Set([...def.base.cameras, ...def.tracks.camera].map((keyframe) => keyframe.windowId));
        for (const windowId of cameraWindowIds) {
          const pose = tutorialCameraAt(def, windowId, clamped);
          if (pose) getTutorialCameraDriver(windowId)?.set(pose);
        }
        tutorialLastAppliedMsRef.current = clamped;
        tutorialClock.seek(clamped);
        if (documentTouched) await refreshUi(session, { kind: "full" });
        console.log("[DEBUG] tutorial rebuild", { atMs: clamped });
        tutorialDrivenRef.current = false;
      })();
    },
    [activeTutorial, session, loadedPlugins, tutorialClock, refreshUi]
  );
  const playPauseTutorial = useCallback(() => {
    if (!activeTutorial) return;
    if (tutorialPlaying) {
      dispatch({ type: "SET_TUTORIAL_PLAYING", value: false });
      return;
    }
    if (tutorialDeviated && session) {
      const def = activeTutorial;
      const atMs = tutorialClock.getTimeMs();
      tutorialDrivenRef.current = true;
      applyTutorialUiSnapshotToShell(dispatch, composeTutorialUi(def, atMs), uiBridgeCtxRef.current);
      const cameraWindowIds = new Set([...def.base.cameras, ...def.tracks.camera].map((keyframe) => keyframe.windowId));
      const startPoseByWindow = /* @__PURE__ */ new Map();
      for (const windowId of cameraWindowIds) {
        const live = getTutorialCameraDriver(windowId)?.get();
        if (live) startPoseByWindow.set(windowId, live);
      }
      const startedAt = performance.now();
      const tween = (now) => {
        const progress = Math.min(1, (now - startedAt) / TUTORIAL_CONVERGE_MS);
        for (const windowId of cameraWindowIds) {
          const targetPose = tutorialCameraAt(def, windowId, atMs);
          if (!targetPose) continue;
          const driver = getTutorialCameraDriver(windowId);
          if (!driver) continue;
          const startPose = startPoseByWindow.get(windowId);
          if (startPose && startPose.kind === targetPose.kind) {
            driver.set(interpolateTutorialCamera({ at: 0, windowId, camera: startPose, easing: "linear" }, { at: TUTORIAL_CONVERGE_MS, windowId, camera: targetPose, easing: "linear" }, progress * TUTORIAL_CONVERGE_MS));
          } else {
            driver.set(targetPose);
          }
        }
        if (progress < 1) requestAnimationFrame(tween);
        else {
          tutorialDrivenRef.current = false;
          dispatch({ type: "SET_TUTORIAL_DEVIATED", value: false });
          dispatch({ type: "SET_TUTORIAL_PLAYING", value: true });
        }
      };
      requestAnimationFrame(tween);
      return;
    }
    dispatch({ type: "SET_TUTORIAL_PLAYING", value: true });
  }, [activeTutorial, tutorialPlaying, tutorialDeviated, session, tutorialClock]);
  const startTutorial = useCallback(
    (tutorialId) => {
      if (!activeTutorials.some((tutorial) => tutorial.id === tutorialId)) return;
      dispatch({ type: "SET_TUTORIAL", value: tutorialId });
    },
    [activeTutorials]
  );
  const stopTutorial = useCallback(() => {
    dispatch({ type: "SET_TUTORIAL", value: null });
  }, []);
  const toggleTutorialRecording = useCallback(() => {
    if (!session) return;
    const recorder = tutorialRecorderRef.current;
    if (recorder) {
      tutorialRecorderRef.current = null;
      const id = `recorded-${session.app.id}-${Date.now()}`;
      const def = recorder.build(id, `${session.app.id} recording`);
      const validationError = validateTutorial(def);
      if (validationError) console.error("[DEBUG] tutorial recording validation failed", validationError);
      const json = JSON.stringify(def, null, 2);
      console.log("[DEBUG] tutorial recording", json);
      downloadMediaExport(`tutorial-${session.app.id}-${Date.now()}.ops`, "text/plain", json);
      dispatch({ type: "SET_TUTORIAL_RECORDING", value: false });
      return;
    }
    void (async () => {
      const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === session.pluginId)?.handle;
      let documentJson = null;
      try {
        if (plugin?.readAppDocument) documentJson = await plugin.readAppDocument(session.instanceId);
      } catch (captureError) {
        console.error("[DEBUG] tutorial recorder base capture failed", captureError);
      }
      tutorialRecorderRef.current = new TutorialRecorder(captureTutorialUiSnapshot(shellStateRef.current, session), documentJson);
      dispatch({ type: "SET_TUTORIAL_RECORDING", value: true });
    })();
  }, [session, loadedPlugins]);
  useEffect(() => {
    startTutorialRef.current = startTutorial;
    stopTutorialRef.current = stopTutorial;
    toggleTutorialRecordingRef.current = toggleTutorialRecording;
  }, [startTutorial, stopTutorial, toggleTutorialRecording]);
  useEffect(() => {
    if (!tutorialRecording) return;
    tutorialRecorderRef.current?.recordUiDiff(captureTutorialUiSnapshot(shellState, session));
  }, [tutorialRecording, shellState, session]);
  useEffect(() => {
    if (!tutorialRecording || !session || typeof window === "undefined") return;
    const interval = window.setInterval(() => {
      tutorialRecorderRef.current?.recordSnapshot(captureTutorialUiSnapshot(shellStateRef.current, session));
    }, 5e3);
    return () => window.clearInterval(interval);
  }, [tutorialRecording, session]);
  useEffect(() => {
    if (!tutorialRecording || !session || typeof window === "undefined") return;
    const interval = window.setInterval(() => {
      const recorder = tutorialRecorderRef.current;
      if (!recorder) return;
      for (const instance of sessionWindowInstances(session.app, extraWindowInstancesRef.current)) {
        const pose = getTutorialCameraDriver(instance.id)?.get();
        if (pose) recorder.sampleCamera(instance.id, pose);
      }
    }, 100);
    return () => window.clearInterval(interval);
  }, [tutorialRecording, session]);
  const addTutorialChapter = useCallback(() => {
    tutorialRecorderRef.current?.addChapter();
  }, []);
  const tutorialChapterMarkers = useMemo(
    () => activeTutorial ? activeTutorial.chapters.map((chapter) => ({ id: chapter.id, title: resolveManifestLabel(chapter.title, uiTerminology, uiLocale), atMs: chapter.at })) : [],
    [activeTutorial, uiTerminology, uiLocale]
  );
  const studioSessionActive = studioMode && session?.app.id === hostAppId;
  const studioSessionControllerId = studioSessionActive ? session?.app.controllerId : void 0;
  useEffect(() => {
    if (!studioSessionActive || !studioSessionControllerId || typeof window === "undefined") return;
    const identity = presenceClientIdentity(ephemeral);
    const beat = () => onActionRef.current({ controllerId: studioSessionControllerId, action: "presenceHeartbeat", args: identity });
    const initial = window.setTimeout(beat, 1e3);
    const timer = window.setInterval(beat, PRESENCE_HEARTBEAT_INTERVAL_MS);
    return () => {
      window.clearTimeout(initial);
      window.clearInterval(timer);
    };
  }, [studioSessionActive, studioSessionControllerId, ephemeral]);
  usePanelChromeHotkeys({
    // 📱️ All eight anchor hotkeys collapse onto the single mobile panel toggle on mobile. Same `shell.panelToggle`
    // commandId as the mouse-driven toggle in `buildPanelSelectionProps` (so keyboard/mouse fold together),
    // flagged `hotkey: true` in detail.
    onToggle: (anchor) => {
      if (mobile) dispatch({ type: "SET_MOBILE_PANEL_VISIBLE", value: (visible) => !visible });
      else
        dispatch({ type: "SET_PANEL_VISIBLE", anchor, value: (visible) => !visible });
      noteShellCommand("shell.panelToggle", shellLabel("ui.shellCommand.panelToggle"), { anchor: mobile ? void 0 : anchor, hotkey: true });
    }
  });
  useElementsSurfaceChrome({ appearance: uiAppearance, device: uiDevice, driver: uiDriver }, scope.rootRef.current ?? void 0);
  useEffect(() => {
    if (!locks.appearance) writeStoredUiChromeAppearance(scope.storage, uiAppearance);
    writeStoredUiChromeLayout(scope.storage, uiLayout);
    writeStoredUiDriverId(scope.storage, uiDriverId);
    writeStoredUiCustomDrivers(scope.storage, uiCustomDrivers);
    writeStoredUiKeybindingOverrides(scope.storage, uiKeybindingOverrides);
    if (!locks.locale) writeStoredUiChromeLocale(scope.storage, uiLocale);
    void scope.i18n.changeLanguage(uiLocale);
    if (scope.ownsPage) {
      if (typeof document !== "undefined") document.documentElement.lang = uiLocale;
    } else if (scope.rootRef.current) {
      scope.rootRef.current.lang = uiLocale;
    }
    if (!locks.terminology) writeStoredUiChromeTerminology(scope.storage, uiTerminology);
    if (scope.ownsPage) {
      setActiveUiTheme(uiTheme);
    } else if (scope.rootRef.current) {
      applyUiThemeToRoot(scope.rootRef.current, uiTheme);
    }
    if (!locks.themeId) {
      writeStoredUiChromeThemeSnapshot(scope.storage, uiTheme);
      writeStoredUiChromeThemeId(scope.storage, uiThemeId);
    }
    writeStoredUiCustomThemes(scope.storage, uiCustomThemes);
  }, [uiAppearance, uiLayout, uiDriverId, uiCustomDrivers, uiKeybindingOverrides, uiLocale, uiTerminology, uiTheme, uiThemeId, uiCustomThemes, locks, scope]);
  useEffect(() => {
    if (scope.ownsPage) return;
    return () => {
      if (scope.rootRef.current) clearUiThemeFromRoot(scope.rootRef.current);
    };
  }, [scope]);
  useActionHotkey(
    "ui.nav.back",
    useCallback(() => {
      if (canGoBack) goBack();
    }, [canGoBack, goBack]),
    void 0,
    [canGoBack, goBack],
    { overrides: uiKeybindingOverrides }
  );
  useActionHotkey(
    "ui.nav.forward",
    useCallback(() => {
      if (canGoForward) goForward();
    }, [canGoForward, goForward]),
    void 0,
    [canGoForward, goForward],
    { overrides: uiKeybindingOverrides }
  );
  useActionHotkey(
    "ui.nav.up",
    useCallback(() => {
      if (canGoUp) goUp();
    }, [canGoUp, goUp]),
    void 0,
    [canGoUp, goUp],
    { overrides: uiKeybindingOverrides }
  );
  useActionHotkey(
    "ui.search.toggle",
    useCallback(() => dispatch({ type: "SET_SEARCH_OPEN", value: (open) => !open }), []),
    void 0,
    [],
    { overrides: uiKeybindingOverrides }
  );
  useActionHotkey(
    "ui.find.toggle",
    useCallback(() => dispatch({ type: "SET_FIND_OPEN", value: (open) => !open }), []),
    void 0,
    [],
    { overrides: uiKeybindingOverrides }
  );
  const applyNamedLayout = useCallback(
    (layout) => {
      if (!session) return;
      const seeded = applyFrameworkLayoutSeed(layout, session.app.windowKinds, appLabelsOverlay, uiTerminology, uiLocale);
      extraWindowInstancesRef.current = seeded.extraInstances;
      extraWindowCounterRef.current = seeded.extraInstances.length;
      dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: seeded.extraInstances });
      dispatch({ type: "SET_SHELL_LAYOUT", value: seeded.modeLayout });
      dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: null });
      void refreshUi(session, { kind: "full" }, seeded.extraInstances);
    },
    [session, appLabelsOverlay, refreshUi, uiTerminology, uiLocale]
  );
  const applyModeChange = useCallback(
    (modeId) => {
      dispatch({ type: "SET_ACTIVE_TOOL", toolId: null });
      dispatch({
        type: "SET_SESSION",
        value: (current) => {
          if (!current) return current;
          const layout = resolveLayoutForMode(current.app, modeId);
          const nextSession = { ...current, viewState: { ...current.viewState, activeModeId: modeId, activeToolId: void 0 } };
          if (layout) {
            const seeded = applyFrameworkLayoutSeed(layout, current.app.windowKinds, appLabelsOverlay, uiTerminology, uiLocale);
            extraWindowInstancesRef.current = seeded.extraInstances;
            extraWindowCounterRef.current = seeded.extraInstances.length;
            dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: seeded.extraInstances });
            dispatch({ type: "SET_SHELL_LAYOUT", value: seeded.modeLayout });
            dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: null });
            void refreshUi(nextSession, { kind: "full" }, seeded.extraInstances);
          }
          return nextSession;
        }
      });
    },
    [appLabelsOverlay, refreshUi, uiTerminology, uiLocale]
  );
  const handleTemplateDrop = useCallback(
    (payload, target) => {
      if (!session) return;
      const kind = session.app.windowKinds.find((entry) => entry.id === payload.windowKindId);
      if (!kind) return;
      extraWindowCounterRef.current += 1;
      const instanceId = `${payload.windowKindId}-${extraWindowCounterRef.current}`;
      const projectionSpec = decodeWorldProjectionTemplateId(payload.templateId);
      if (projectionSpec) registerPendingWorldProjection(instanceId, projectionSpec);
      const title = projectionSpec ? worldProjectionSpecLabel(projectionSpec) : resolveAppLabel(appLabelsOverlay, "windowKind", kind.id, resolveManifestLabel(kind.label, uiTerminology, uiLocale));
      const nextExtraInstances = [...extraWindowInstancesRef.current, { id: instanceId, windowKindId: payload.windowKindId, title }];
      extraWindowInstancesRef.current = nextExtraInstances;
      dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: nextExtraInstances });
      if (projectionSpec) {
        dispatch({ type: "SET_WINDOW_TITLE", windowId: instanceId, title });
        dispatch({ type: "SET_WINDOW_ICON", windowId: instanceId, iconId: worldProjectionSpecIconId(projectionSpec) });
      }
      void refreshUi(session, { kind: "full" }, nextExtraInstances);
      dispatch({
        type: "SET_SHELL_LAYOUT",
        value: (current) => {
          const base = current ?? resolveFrameworkLayoutSeed(session.app.defaultLayout, session.app.windowKinds, appLabelsOverlay, uiTerminology, uiLocale).modeLayout;
          return insertWindowAtDropZone(base, instanceId, target);
        }
      });
      dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: instanceId });
      noteShellCommand("shell.windowSplit", shellLabel("ui.shellCommand.windowSplit"), { windowKindId: payload.windowKindId, instanceId });
    },
    [appLabelsOverlay, refreshUi, session, noteShellCommand, uiTerminology, uiLocale]
  );
  const displayHostRef = useRef(null);
  const displayHost = useNamedLayoutHost({
    appId: session?.app.id ?? "framework-os",
    windowKinds: session?.app.windowKinds.map((kind) => ({ ...kind, label: resolveAppLabel(appLabelsOverlay, "windowKind", kind.id, resolveManifestLabel(kind.label, uiTerminology, uiLocale)) })) ?? [],
    builtinLayouts: session?.app.namedLayouts ?? [],
    currentLayout: captureCurrentFrameworkLayout(shellLayout, extraWindowInstances, session?.app.defaultLayout),
    onApplyLayout: applyNamedLayout,
    namedLayoutStore
  });
  displayHostRef.current = displayHost;
  const uiThemeBase = uiThemeDraft ?? uiTheme;
  const uiThemeDirty = uiThemeDraft !== null;
  const uiThemeList = useMemo(() => [...builtinUiThemes(), ...Object.values(uiCustomThemes)], [uiCustomThemes]);
  const uiDriverList = useMemo(() => [...builtinUiDrivers(), ...Object.values(uiCustomDrivers)], [uiCustomDrivers]);
  const keysByActionId = useMemo(() => buildKeysByActionId(session?.app.keybindings ?? []), [session?.app.keybindings]);
  const controlKeybindings = useMemo(() => composeControlKeybindings(keysByActionId, uiKeybindingOverrides), [keysByActionId, uiKeybindingOverrides]);
  const osCommands = useMemo(
    () => buildOsCommands(uiThemeList, [UI_TERMINOLOGY_NATIVE, ...session?.app.terminologies ?? []], activeIntroduction != null, locks, uiDriverList, activeTutorials, tutorialRecorderAvailable, uiTerminology, uiLocale),
    [uiThemeList, session?.app.terminologies, activeIntroduction, uiLocale, uiTerminology, locks, uiDriverList, activeTutorials, tutorialRecorderAvailable]
  );
  const noteOsCommand = useCallback(
    (commandId, detail) => {
      const label = osCommands.find((entry) => entry.id === commandId)?.label ?? commandId;
      noteShellCommand(commandId, label, detail);
    },
    [osCommands, noteShellCommand]
  );
  const draftThemePatch = useCallback(
    (patch) => {
      const next = structuredClone(uiThemeBase);
      patch(next);
      dispatch({ type: "SET_UI_THEME_DRAFT", value: next });
    },
    [uiThemeBase]
  );
  const setThemeId = useCallback(
    (id) => {
      dispatch({ type: "SET_UI_THEME_DRAFT", value: null });
      dispatch({ type: "SET_UI_THEME_ID", value: id });
      noteOsCommand("os.setThemeId", { themeId: id });
    },
    [noteOsCommand]
  );
  const setThemeColor = useCallback(
    (key, hex) => draftThemePatch((next) => {
      next.colors[key] = hex;
    }),
    [draftThemePatch]
  );
  const setThemeSpacing = useCallback(
    (key, value) => draftThemePatch((next) => {
      next.spacing[key] = value;
    }),
    [draftThemePatch]
  );
  const setThemeFontStack = useCallback(
    (key, value) => draftThemePatch((next) => {
      next.fontStacks[key] = value;
    }),
    [draftThemePatch]
  );
  const setThemeStroke = useCallback(
    (key, value) => draftThemePatch((next) => {
      next.strokes[key] = value;
    }),
    [draftThemePatch]
  );
  const setThemeRadius = useCallback(
    (key, value) => draftThemePatch((next) => {
      next.radii[key] = value;
    }),
    [draftThemePatch]
  );
  const setThemeOpacity = useCallback(
    (key, value) => draftThemePatch((next) => {
      next.opacities[key] = value;
    }),
    [draftThemePatch]
  );
  const setThemeMetric = useCallback(
    (section, key, value) => draftThemePatch((next) => {
      next.metrics[section] = { ...next.metrics[section] ?? {}, [key]: value };
    }),
    [draftThemePatch]
  );
  const setThemeAppearancePaint = useCallback(
    (appearance, group, key, hex, alpha) => draftThemePatch((next) => {
      next.appearances[appearance][group][key] = alpha === void 0 ? { hex } : { hex, alpha };
    }),
    [draftThemePatch]
  );
  const resetTheme = useCallback(() => {
    dispatch({ type: "SET_UI_THEME_DRAFT", value: null });
    dispatch({ type: "SET_UI_THEME_ID", value: "semio" });
  }, []);
  const saveTheme = useCallback(
    (label) => {
      const trimmed = label.trim();
      if (!trimmed) return;
      const slug = trimmed.toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/(^-+|-+$)/g, "");
      if (!slug) return;
      const id = `custom.${slug}`;
      const saved = { ...uiThemeBase, id, label: trimmed };
      dispatch({ type: "SET_UI_CUSTOM_THEMES", value: (current) => ({ ...current, [id]: saved }) });
      dispatch({ type: "SET_UI_THEME_DRAFT", value: null });
      dispatch({ type: "SET_UI_THEME_ID", value: id });
    },
    [uiThemeBase]
  );
  const deleteTheme = useCallback((id) => {
    if (!id.startsWith("custom.")) return;
    dispatch({
      type: "SET_UI_CUSTOM_THEMES",
      value: (current) => {
        const { [id]: _removed, ...rest } = current;
        return rest;
      }
    });
    dispatch({ type: "SET_UI_THEME_ID", value: (current) => current === id ? "semio" : current });
    dispatch({ type: "SET_UI_THEME_DRAFT", value: null });
  }, []);
  const exportTheme = useCallback(() => {
    downloadMediaExport(`${uiThemeBase.id}.theme.dsl`, "text/plain", serializeUiTheme(uiThemeBase));
  }, [uiThemeBase]);
  const importTheme = useCallback(async () => {
    const opened = (await requestFileOpen(".theme.dsl,.dsl,text/plain"))[0];
    if (!opened) return;
    try {
      const parsed = parseUiTheme(JSON.parse(opened.contents));
      saveTheme(parsed.label || parsed.id);
    } catch {
    }
  }, [saveTheme]);
  const uiDriverBase = uiDriverDraft ?? uiDriver;
  const uiDriverDirty = uiDriverDraft !== null;
  const setDriverId = useCallback(
    (id) => {
      dispatch({ type: "SET_UI_DRIVER_DRAFT", value: null });
      dispatch({ type: "SET_UI_DRIVER_ID", value: id });
      noteOsCommand("os.setDriver", { driver: id });
    },
    [noteOsCommand]
  );
  const setDriverField = useCallback(
    (key, value) => {
      dispatch({ type: "SET_UI_DRIVER_DRAFT", value: { ...uiDriverBase, [key]: value } });
    },
    [uiDriverBase]
  );
  const saveDriver = useCallback(
    (label) => {
      const trimmed = label.trim();
      if (!trimmed) return;
      const slug = trimmed.toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/(^-+|-+$)/g, "");
      if (!slug) return;
      const id = `custom.${slug}`;
      const saved = { ...uiDriverBase, id, label: trimmed };
      dispatch({ type: "SET_UI_CUSTOM_DRIVERS", value: (current) => ({ ...current, [id]: saved }) });
      dispatch({ type: "SET_UI_DRIVER_DRAFT", value: null });
      dispatch({ type: "SET_UI_DRIVER_ID", value: id });
    },
    [uiDriverBase]
  );
  const deleteDriver = useCallback((id) => {
    if (!id.startsWith("custom.")) return;
    dispatch({
      type: "SET_UI_CUSTOM_DRIVERS",
      value: (current) => {
        const { [id]: _removed, ...rest } = current;
        return rest;
      }
    });
    dispatch({ type: "SET_UI_DRIVER_ID", value: (current) => current === id ? DEFAULT_UI_DRIVER.id : current });
    dispatch({ type: "SET_UI_DRIVER_DRAFT", value: null });
  }, []);
  const [themeSaveLabel, setThemeSaveLabel] = useState("");
  const [driverSaveLabel, setDriverSaveLabel] = useState("");
  const [keybindingCaptureControlId, setKeybindingCaptureControlId] = useState(null);
  const setKeybindingOverride = useCallback((controlId, keys) => {
    dispatch({ type: "SET_UI_KEYBINDING_OVERRIDES", value: (current) => ({ ...current, [controlId]: keys }) });
  }, []);
  const resetKeybindingOverride = useCallback((controlId) => {
    dispatch({
      type: "SET_UI_KEYBINDING_OVERRIDES",
      value: (current) => {
        const { [controlId]: _removed, ...rest } = current;
        return rest;
      }
    });
  }, []);
  useEffect(() => {
    const onNavigateToHotkey = (event) => {
      const path = event.detail?.path;
      if (path) setKeybindingCaptureControlId(path);
      dispatch({ type: "SET_PANEL_VISIBLE", anchor: "bottom-right", value: true });
      dispatch({ type: "SET_PANEL_PATH", anchor: "bottom-right", value: ["framework.settings.keybindings"] });
    };
    window.addEventListener("navigate-to-hotkey", onNavigateToHotkey);
    return () => window.removeEventListener("navigate-to-hotkey", onNavigateToHotkey);
  }, [dispatch]);
  const settingsHostRef = useRef(null);
  const settingsHost = useMemo(
    () => ({
      appId: session?.app.id,
      appLabel: session ? appDocumentLabel(resolveAppDocument(session.app, uiTerminology)) : void 0,
      controllerId: session?.app.controllerId,
      pluginId: session?.pluginId,
      driverId: uiDriverId,
      driver: uiDriverBase,
      driverDirty: uiDriverDirty,
      drivers: uiDriverList,
      setDriverId,
      setDriverField,
      saveDriver,
      deleteDriver,
      driverSaveLabel,
      setDriverSaveLabel,
      appearance: uiAppearance,
      setAppearance: (value) => {
        dispatch({ type: "SET_UI_APPEARANCE", value });
        noteOsCommand("os.setAppearance", { appearance: value });
      },
      layout: uiLayout,
      setLayout: (value) => {
        dispatch({ type: "SET_UI_LAYOUT", value });
        noteOsCommand("os.setLayout", { layout: value });
      },
      mobileActive: mobile,
      onResetDock: () => {
        dispatch({ type: "RESET_DOCK" });
        dockLayoutStore.reset();
        dockUiStateStore.reset();
        noteOsCommand("os.resetDock");
      },
      locale: uiLocale,
      setLocale: (value) => {
        dispatch({ type: "SET_UI_LOCALE", value });
        noteOsCommand("os.setLocale", { locale: value });
      },
      terminology: uiTerminology,
      setTerminology: (value) => {
        dispatch({ type: "SET_UI_TERMINOLOGY", value });
        noteOsCommand("os.setTerminology", { terminology: value });
      },
      terminologies: [UI_TERMINOLOGY_NATIVE, ...session?.app.terminologies ?? []],
      theme: uiThemeBase,
      themeId: uiThemeId,
      themeDirty: uiThemeDirty,
      themes: uiThemeList,
      setThemeId,
      setThemeColor,
      setThemeSpacing,
      setThemeFontStack,
      setThemeStroke,
      setThemeRadius,
      setThemeOpacity,
      setThemeMetric,
      setThemeAppearancePaint,
      saveTheme,
      deleteTheme,
      resetTheme,
      exportTheme,
      importTheme,
      themeSaveLabel,
      setThemeSaveLabel,
      controlKeybindings,
      keybindingCaptureControlId,
      setKeybindingCaptureControlId,
      setKeybindingOverride,
      resetKeybindingOverride,
      locks
    }),
    [
      session,
      dockLayoutStore,
      uiDriverId,
      uiDriverBase,
      uiDriverDirty,
      uiDriverList,
      setDriverId,
      setDriverField,
      saveDriver,
      deleteDriver,
      driverSaveLabel,
      setDriverSaveLabel,
      controlKeybindings,
      keybindingCaptureControlId,
      setKeybindingOverride,
      resetKeybindingOverride,
      uiAppearance,
      uiLayout,
      mobile,
      uiLocale,
      uiTerminology,
      uiThemeBase,
      uiThemeId,
      uiThemeDirty,
      uiThemeList,
      locks,
      setThemeId,
      setThemeColor,
      setThemeSpacing,
      setThemeFontStack,
      setThemeStroke,
      setThemeRadius,
      setThemeOpacity,
      setThemeMetric,
      setThemeAppearancePaint,
      saveTheme,
      deleteTheme,
      resetTheme,
      exportTheme,
      importTheme,
      themeSaveLabel,
      setThemeSaveLabel,
      noteOsCommand
    ]
  );
  settingsHostRef.current = settingsHost;
  const frameworkDisplayTabs = useMemo(() => createFrameworkDisplayPanelTabs(() => displayHostRef.current), [displayHost, uiLocale]);
  const frameworkSettingsTabs = useMemo(() => createFrameworkSettingsPanelTabs(() => settingsHostRef.current), [settingsHost]);
  const pluginsHostRef = useRef(null);
  const pluginsHost = useMemo(
    () => ({
      plugins: registry.map((entry) => {
        const loadedEntry = loadedPlugins.find((candidate) => candidate.handle.pluginId === entry.pluginId);
        return {
          pluginId: entry.pluginId,
          label: loadedEntry?.manifest.label ?? entry.pluginId,
          version: loadedEntry?.manifest.version,
          status: pluginStatusById[entry.pluginId] ?? "available",
          sourceId: pluginSource.id,
          canUninstall: entry.pluginId !== primaryPluginId && session?.pluginId !== entry.pluginId
        };
      }),
      install: (pluginId) => void installPlugin(pluginId),
      uninstall: (pluginId) => void uninstallPlugin(pluginId),
      reload: (pluginId) => void reloadPlugin(pluginId)
    }),
    [registry, loadedPlugins, pluginStatusById, pluginSource, primaryPluginId, session?.pluginId, installPlugin, uninstallPlugin, reloadPlugin]
  );
  pluginsHostRef.current = pluginsHost;
  const frameworkPluginsTabs = useMemo(() => createFrameworkPluginsPanelTabs(() => pluginsHostRef.current), [pluginsHost]);
  const handleAppKeydown = useCallback(
    (event) => {
      if (!session) return;
      const parseKeys = (keys) => keys.split(",").map((key) => key.trim().toLowerCase()).filter(Boolean);
      const isEditableTarget = (target) => {
        if (!(target instanceof HTMLElement)) return false;
        const tag = target.tagName;
        if (tag === "INPUT" || tag === "TEXTAREA" || tag === "SELECT") return true;
        if (target.isContentEditable) return true;
        return target.closest("[contenteditable='true'], [role='textbox']") != null;
      };
      const matches = (event2, binding) => {
        const parts = binding.split("+").map((part) => part.trim());
        const key = parts[parts.length - 1] ?? "";
        const needsCtrl = parts.includes("ctrl") || parts.includes("meta") || parts.includes("mod");
        const needsShift = parts.includes("shift");
        const needsAlt = parts.includes("alt");
        const hasCtrl = event2.ctrlKey || event2.metaKey;
        if (needsCtrl !== hasCtrl) return false;
        if (needsShift !== event2.shiftKey) return false;
        if (needsAlt !== event2.altKey) return false;
        return event2.key.toLowerCase() === key;
      };
      const actionById = new Map(session.app.actions.map((action) => [action.id, action]));
      if (isEditableTarget(event.target)) return;
      if (event.key === "Escape") {
        const windowId = activeWindowIdRef.current;
        if (windowId && activeUtilityByWindowIdRef.current[windowId]) {
          event.preventDefault();
          onAction({ controllerId: session.app.controllerId, action: SET_ACTIVE_UTILITY_ACTION_ID, args: { windowId, utilityId: "" } });
          return;
        }
        if (activeToolIdRef.current) {
          event.preventDefault();
          onAction({ controllerId: session.app.controllerId, action: SET_ACTIVE_TOOL_ACTION_ID, args: { toolId: "" } });
          return;
        }
      }
      for (const binding of session.app.keybindings) {
        for (const chord of parseKeys(binding.keys)) {
          if (!matches(event, chord)) continue;
          event.preventDefault();
          const definition = actionById.get(binding.action.action);
          if (definition && actionRequiresStagedForm(definition)) {
            const windowId = activeWindowIdRef.current;
            if (!windowId) return;
            const expanded = actionPaneExpandedByWindowIdRef.current[windowId] ?? null;
            const staged = actionPaneStagedArgsByKeyRef.current[actionStageKey(windowId, definition.id)] ?? {};
            const intent = resolveKeybindingIntent(definition, expanded, staged);
            if (intent.kind === "execute") {
              onAction({ controllerId: session.app.controllerId, action: intent.actionId, args: intent.args });
            } else if (intent.kind === "open") {
              dispatch({ type: "SET_ACTION_PANE_FOLDED", windowId, value: false });
              dispatch({ type: "SET_ACTION_PANE_EXPANDED", windowId, value: intent.actionId });
            }
            return;
          }
          onAction(binding.action);
          return;
        }
      }
    },
    [onAction, session]
  );
  useShellKeydown(scope.rootRef, handleAppKeydown, [handleAppKeydown]);
  const activeRightPanelTab = session?.app.panelTabs.find((tab) => panelAnchorForGroup(tab.group) === "top-right");
  const activePanelTabId = panel?.activePanelTab ?? (activeRightPanelTab ? panelTabKindId(activeRightPanelTab.kind) : void 0) ?? (session?.app.panelTabs[0] ? panelTabKindId(session.app.panelTabs[0].kind) : void 0);
  const workbenchLeftTabs = useMemo(() => {
    if (!session) return [];
    const pluginLeftTabs = session.app.panelTabs.filter((tab) => panelAnchorForGroup(tab.group) === "top-left").map((tab, order) => panelTabDefinitionToNode(tab, tab.group, panelUiByKey, onAction, order, appLabelsOverlay, uiTerminology, uiLocale));
    if (studioMode && session.app.id === hostAppId && pluginLeftTabs.length > 0) return pluginLeftTabs;
    const hasPluginDocumentTab = pluginLeftTabs.some((tab) => tab.id === FRAMEWORK_PANEL_TAB_DOCUMENT_ID);
    if (hasPluginDocumentTab) return pluginLeftTabs;
    const documentTab = singleTreeLeaf({
      id: FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
      icon: shellTabIcon(FRAMEWORK_PANEL_TAB_DOCUMENT_ICON_ID),
      name: shellLabel("ui.panel.document"),
      order: 0,
      tree: staticTreePanelDefinition({
        sections: [
          {
            id: "document.root",
            label: shellLabel("ui.panel.document"),
            items: [{ id: "document.empty", label: studioMode ? `${panel?.spawnedApps.length ?? 0} ${shellLabel("ui.panel.spawnedAppsSuffix")}` : shellLabel("ui.panel.documentEmpty") }]
          }
        ]
      })
    });
    return [documentTab, ...pluginLeftTabs];
  }, [appLabelsOverlay, onAction, panel?.spawnedApps.length, panelUiByKey, session, studioMode, uiLocale, uiTerminology, hostAppId]);
  const detailsRightTabs = useMemo(() => {
    if (!session) return [];
    return session.app.panelTabs.filter((tab) => panelAnchorForGroup(tab.group) === "top-right").map((tab, order) => panelTabDefinitionToNode(tab, tab.group, panelUiByKey, onAction, order, appLabelsOverlay, uiTerminology, uiLocale));
  }, [appLabelsOverlay, onAction, panelUiByKey, session, uiTerminology, uiLocale]);
  const settingsRightTabs = useMemo(() => frameworkSettingsTabs, [frameworkSettingsTabs]);
  const frameworkUtilitiesHistoryTab = useMemo(() => {
    if (!session) return null;
    const tab = session.app.panelTabs.find((candidate) => panelTabKindId(candidate.kind) === FRAMEWORK_PANEL_TAB_HISTORY_ID);
    if (!tab) return null;
    return panelTabDefinitionToNode(tab, tab.group, panelUiByKey, onAction, 1, appLabelsOverlay, uiTerminology, uiLocale);
  }, [appLabelsOverlay, onAction, panelUiByKey, session, uiTerminology, uiLocale]);
  const frameworkSyncTab = useMemo(() => {
    const syncUtilities = buildFrameworkSyncUtilities(syncBackboneUri);
    if (!syncUtilities.length) return null;
    const syncStatus = syncBackboneUri ? syncStatusByDocumentId[syncBackboneUri.replace(/^actor:\/\//, "")] ?? null : null;
    return singleTreeLeaf({
      id: "framework.sync",
      icon: shellTabIcon(UTILITY_CATEGORY_ICON_ID.sync),
      name: shellLabel("ui.panel.sync"),
      order: 0,
      tree: {
        sections: [
          {
            id: "framework.sync.root",
            label: "",
            items: [
              {
                id: "framework.sync.card",
                label: "",
                control: /* @__PURE__ */ jsxDEV(
                  SyncAttachCard,
                  {
                    activeUri: syncBackboneUri,
                    cardKind: syncCardKind,
                    draftPath: syncDraftPath,
                    syncUtilities,
                    status: syncStatus,
                    onAction,
                    onDraftPathChange: (value) => dispatch({ type: "SET_SYNC_DRAFT_PATH", value }),
                    onClose: () => dispatch({ type: "SET_SYNC_CARD_KIND", value: null }),
                    onAttach: attachSyncBackbone,
                    onDetach: detachSyncBackbone
                  },
                  void 0,
                  false,
                  {
                    fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
                    lineNumber: 3600,
                    columnNumber: 13
                  },
                  this
                )
              }
            ]
          }
        ]
      }
    });
  }, [attachSyncBackbone, detachSyncBackbone, onAction, syncBackboneUri, syncCardKind, syncDraftPath, syncStatusByDocumentId, uiLocale]);
  const activePluginManifest = useMemo(() => loadedPlugins.find((entry) => entry.handle.pluginId === session?.pluginId)?.manifest, [loadedPlugins, session?.pluginId]);
  const activeModeId = session?.viewState.activeModeId ?? session?.app.modes[0]?.id ?? session?.app.id ?? "";
  const exampleOptions = useMemo(() => {
    const appId2 = session?.app.id ?? "";
    if (!appId2) return [];
    const seen = /* @__PURE__ */ new Set();
    return (activePluginManifest?.examples ?? []).filter((example) => example.appId === appId2).filter((example) => {
      if (seen.has(example.id)) return false;
      seen.add(example.id);
      return true;
    }).map((example) => ({
      id: example.id,
      label: resolveAppLabel(appLabelsOverlay, "example", example.id, resolveManifestLabel(example.label, uiTerminology, uiLocale)),
      icon: example.iconId
    }));
  }, [activePluginManifest, session?.app.id, appLabelsOverlay, uiTerminology, uiLocale]);
  const dispatchActiveExample = useCallback(
    (exampleId) => {
      if (!session) return;
      const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === session.pluginId)?.handle;
      if (!plugin) return;
      onAction({ controllerId: session.app.controllerId, action: "setActiveExample", args: { exampleId: exampleId || "" } });
    },
    [applyHostEffects, injectActiveUtility, loadedPlugins, onAction, session]
  );
  const exampleSelectElement = useMemo(() => {
    if (!session || exampleOptions.length === 0 || locks.exampleId || studioMode && session.app.id === landingAppId) return null;
    return /* @__PURE__ */ jsxDEV(
      NavbarExampleSelect,
      {
        id: "playground.navbar.fixture",
        value: activeExampleId,
        options: exampleOptions,
        onValueChange: (exampleId) => {
          dispatch({ type: "SET_ACTIVE_EXAMPLE_ID", value: exampleId });
          dispatchActiveExample(exampleId || "");
        }
      },
      "fixture",
      false,
      {
        fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
        lineNumber: 3659,
        columnNumber: 7
      },
      this
    );
  }, [session, exampleOptions, locks.exampleId, studioMode, landingAppId, activeExampleId, dispatchActiveExample]);
  const modeSwitcherElement = useMemo(() => {
    if (!session || session.app.modes.length <= 1) return null;
    return /* @__PURE__ */ jsxDEV(ButtonGroup, { id: "playground.navbar.modes", children: session.app.modes.map((mode) => {
      const isActive = activeModeId === mode.id;
      return /* @__PURE__ */ jsxDEV(
        ButtonGroupItem,
        {
          id: `playground.navbar.modes.${mode.id}`,
          className: cn(isActive && interactiveActiveFillClass),
          "data-state": isActive ? "on" : void 0,
          onClick: () => applyModeChange(mode.id),
          icon: mode.iconId,
          text: resolveAppLabel(appLabelsOverlay, "mode", mode.id, resolveManifestLabel(mode.label, uiTerminology, uiLocale))
        },
        mode.id,
        false,
        {
          fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
          lineNumber: 3680,
          columnNumber: 13
        },
        this
      );
    }) }, "modes", false, {
      fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
      lineNumber: 3676,
      columnNumber: 7
    }, this);
  }, [session, activeModeId, applyModeChange, appLabelsOverlay, uiTerminology, uiLocale]);
  const resolvedCommands = useMemo(
    () => resolveCommands(osCommands, activePluginManifest, session?.app, activeModeId, appLabelsOverlay, uiTerminology, uiLocale),
    [osCommands, activePluginManifest, session?.app, activeModeId, appLabelsOverlay, uiTerminology, uiLocale]
  );
  const commandCategoryList = useMemo(() => commandCategories(resolvedCommands), [resolvedCommands, uiLocale]);
  const onCommand = useCallback(
    (source, commandId, args) => {
      if (source.kind === "os" && commandId === "os.playTutorial") {
        const tutorialId = typeof args?.tutorialId === "string" ? args.tutorialId : "";
        if (tutorialId) startTutorialRef.current(tutorialId);
        return;
      }
      if (source.kind === "os" && commandId === "os.recordTutorial") {
        toggleTutorialRecordingRef.current();
        return;
      }
      if (source.kind === "os") {
        dispatchOsCommand(commandId, args, dispatch, dockLayoutStore, dockUiStateStore, locks);
        const label = resolvedCommands.find((entry) => entry.definition.id === commandId)?.definition.label ?? commandId;
        noteShellCommand(commandId, label, args);
        return;
      }
      if (!session) return;
      if (tutorialRecordingRef.current && !tutorialDrivenRef.current) {
        tutorialRecorderRef.current?.recordEvent({ kind: "command", command: commandId, args });
      }
      const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === session.pluginId)?.handle;
      if (!plugin?.handleAction) return;
      const dispatchViewState = injectActiveUtility(session.viewState);
      void plugin.handleAction(session.instanceId, encodeActionWire({ controllerId: session.app.controllerId, action: commandId, args }), dispatchViewState).then((response) => applyHostEffects(response.requestedEffects ?? [], { ...session, viewState: dispatchViewState }, resolveUiDirtyScope(response.uiScope))).catch((commandError) => {
        console.error("[DEBUG] command failed", commandError);
      });
    },
    [applyHostEffects, dockLayoutStore, dockUiStateStore, injectActiveUtility, loadedPlugins, session, locks, resolvedCommands, noteShellCommand]
  );
  const commandCategoryTabs = useMemo(() => buildCommandCategoryTabs(resolvedCommands, commandCategoryList, expandedCommandIdRef, commandStagedArgsByCommandIdRef, onCommand, dispatch), [resolvedCommands, commandCategoryList, onCommand]);
  const resolvedModeTools = useMemo(
    () => resolveModeTools(session?.app, activeModeId).map((tool) => ({ ...tool, label: resolveManifestLabel(tool.label, uiTerminology, uiLocale) })),
    [session?.app, activeModeId, uiTerminology, uiLocale]
  );
  const toolTabs = useMemo(
    () => session ? buildToolTabs(resolvedModeTools, session.app.controllerId, activeToolIdRef, toolMeasuresByToolIdRef, onActionStable) : [],
    [resolvedModeTools, session?.app.controllerId, onActionStable]
  );
  const defaultDock = useMemo(() => {
    const topLeft = [...workbenchLeftTabs];
    const bottomLeft = [];
    if (frameworkDisplayTabs.length > 0) {
      bottomLeft.push({ kind: "branch", id: FRAMEWORK_CATEGORY_DISPLAY_ID, icon: categoryTabIcon(frameworkDisplayTabs, "layout-grid"), name: shellLabel("ui.panelToggle.display"), order: 0, children: frameworkDisplayTabs });
    }
    if (frameworkSyncTab) bottomLeft.push(frameworkSyncTab);
    const topRight = [...detailsRightTabs];
    const bottomRight = [...settingsRightTabs, ...frameworkPluginsTabs];
    if (frameworkUtilitiesHistoryTab) bottomRight.push(frameworkUtilitiesHistoryTab);
    const bottomMiddle = [
      ...toolTabs.length > 0 ? [{ kind: "branch", id: FRAMEWORK_CATEGORY_TOOL_ID, icon: categoryTabIcon(toolTabs, "hammer"), name: shellLabel("ui.panelToggle.tool"), order: 0, children: toolTabs }] : [],
      ...commandCategoryTabs.length > 0 ? [{ kind: "branch", id: FRAMEWORK_CATEGORY_COMMAND_ID, icon: categoryTabIcon(commandCategoryTabs, "wrench"), name: shellLabel("ui.panelToggle.command"), order: 1, children: commandCategoryTabs }] : []
    ];
    return { anchors: { "top-left": topLeft, "top-middle": [], "top-right": topRight, "right-middle": [], "bottom-right": bottomRight, "bottom-middle": bottomMiddle, "bottom-left": bottomLeft, "left-middle": [] } };
  }, [commandCategoryTabs, detailsRightTabs, frameworkDisplayTabs, frameworkPluginsTabs, frameworkSyncTab, frameworkUtilitiesHistoryTab, settingsRightTabs, toolTabs, uiLocale, workbenchLeftTabs]);
  useEffect(() => {
    dispatch({ type: "SET_DOCK_OVERRIDE", value: dockLayoutStore.getSnapshot() });
  }, [dockLayoutStore]);
  const dock = useMemo(() => applyDockSkeleton(defaultDock, dockOverride), [defaultDock, dockOverride]);
  const mobilePanelTabs = useMemo(() => {
    const anchorTabs = ANCHORS.flatMap((anchor) => defaultDock.anchors[anchor]);
    if (!exampleSelectElement && !modeSwitcherElement) return anchorTabs;
    const appTab = singleTreeLeaf({
      id: "framework.mobile.app",
      icon: shellTabIcon("smartphone"),
      name: shellLabel("ui.mobilePanel.app"),
      order: 99,
      tree: {
        sections: [
          {
            id: "framework.mobile.app.root",
            label: "",
            items: [
              ...exampleSelectElement ? [{ id: "framework.mobile.app.example", label: "", control: exampleSelectElement }] : [],
              ...modeSwitcherElement ? [{ id: "framework.mobile.app.modes", label: "", control: modeSwitcherElement }] : []
            ]
          }
        ]
      }
    });
    return [...anchorTabs, appTab];
  }, [defaultDock, exampleSelectElement, modeSwitcherElement]);
  const dockPersistedOnceRef = useRef(false);
  useEffect(() => {
    if (!dockPersistedOnceRef.current) {
      dockPersistedOnceRef.current = true;
      return;
    }
    const nextSkeleton = dockSkeletonOf(dock);
    const defaultSkeleton = dockSkeletonOf(defaultDock);
    dockLayoutStore.save(dockSkeletonsEqual(nextSkeleton, defaultSkeleton) ? null : nextSkeleton);
  }, [dock, defaultDock, dockLayoutStore]);
  useEffect(() => {
    dispatch({ type: "HYDRATE_DOCK_UI", value: dockUiStateStore.getSnapshot() });
  }, [dockUiStateStore]);
  const dockUiPersistedOnceRef = useRef(false);
  const dockUiPersistedStoreRef = useRef(dockUiStateStore);
  useEffect(() => {
    if (dockUiPersistedStoreRef.current !== dockUiStateStore) {
      dockUiPersistedStoreRef.current = dockUiStateStore;
      dockUiPersistedOnceRef.current = false;
    }
    if (!dockUiPersistedOnceRef.current) {
      dockUiPersistedOnceRef.current = true;
      return;
    }
    const anchors = {};
    for (const anchor of ANCHORS) {
      const panelState = panels[anchor];
      const entry = {};
      if (panelState.visible) entry.visible = true;
      if (panelState.size !== DEFAULT_PANEL_WIDTH_PX) entry.size = panelState.size;
      if (panelState.path.length > 0) entry.path = panelState.path;
      if (Object.keys(entry).length > 0) anchors[anchor] = entry;
    }
    const hasPathMemory = Object.keys(panelPathMemory).length > 0;
    const hasTreeOpen = Object.keys(treeOpenStates).length > 0;
    const isDefault = Object.keys(anchors).length === 0 && !hasPathMemory && !hasTreeOpen;
    dockUiStateStore.save(isDefault ? null : { version: 3, anchors, pathMemory: hasPathMemory ? panelPathMemory : void 0, treeOpen: hasTreeOpen ? treeOpenStates : void 0 });
  }, [panels, panelPathMemory, treeOpenStates, dockUiStateStore]);
  const handleTabDockDrop = useCallback(
    (move) => {
      const nextDock = moveTabInDock(dock, move);
      if (nextDock === dock) return;
      const nextSkeleton = dockSkeletonOf(nextDock);
      const defaultSkeleton = dockSkeletonOf(defaultDock);
      dispatch({ type: "SET_DOCK_OVERRIDE", value: dockSkeletonsEqual(nextSkeleton, defaultSkeleton) ? null : nextSkeleton });
      const targetPath = findPanelTabPath(nextDock.anchors[move.target.anchor], move.tabId);
      if (targetPath) dispatch({ type: "SET_PANEL_PATH", anchor: move.target.anchor, value: targetPath });
      if (move.fromAnchor !== move.target.anchor) {
        const sourceTabs = nextDock.anchors[move.fromAnchor];
        dispatch({ type: "SET_PANEL_PATH", anchor: move.fromAnchor, value: (prev) => reconcileActivePath(sourceTabs, prev, panelTabChildren) });
      }
      dispatch({ type: "SET_PANEL_VISIBLE", anchor: move.target.anchor, value: true });
      noteShellCommand("shell.dockMove", shellLabel("ui.shellCommand.dockMove"), { tabId: move.tabId, fromAnchor: move.fromAnchor, toAnchor: move.target.anchor });
    },
    [dock, defaultDock, noteShellCommand]
  );
  const handleTreeUnitDockDrop = useCallback(
    (move) => {
      const nextDock = moveTreeUnitInDock(dock, move);
      if (nextDock === dock) return;
      const nextSkeleton = dockSkeletonOf(nextDock);
      const defaultSkeleton = dockSkeletonOf(defaultDock);
      dispatch({ type: "SET_DOCK_OVERRIDE", value: dockSkeletonsEqual(nextSkeleton, defaultSkeleton) ? null : nextSkeleton });
      dispatch({ type: "SET_PANEL_VISIBLE", anchor: move.target.anchor, value: true });
      noteShellCommand("shell.dockMove", shellLabel("ui.shellCommand.dockMove"), { toAnchor: move.target.anchor });
    },
    [dock, defaultDock, noteShellCommand]
  );
  const studioOverrideTabId = studioMode && session?.app.id === hostAppId ? panel?.activePanelTab ?? hostCatalogueTabId : void 0;
  const studioOverrideAnchor = studioOverrideTabId ? findPanelTabInDock(dock, studioOverrideTabId)?.anchor : void 0;
  const detailsOverrideTabId = panel?.activePanelTab;
  const detailsOverrideAnchor = detailsOverrideTabId ? findPanelTabInDock(dock, detailsOverrideTabId)?.anchor : void 0;
  const activeIntroductionStep = activeIntroduction && introductionStepIndex != null ? activeIntroduction.steps[introductionStepIndex] ?? null : null;
  const introductionElementIds = useMemo(
    () => activeIntroductionStep ? [activeIntroductionStep.introduce, ...activeIntroductionStep.show].filter((id) => Boolean(id)) : [],
    [activeIntroductionStep]
  );
  const introductionUtilityId = useMemo(() => {
    if (!session) return null;
    const utilities = session.app.utilities ?? [];
    return introductionElementIds.find((id) => utilities.some((utility) => utility.id === id)) ?? null;
  }, [introductionElementIds, session]);
  const introductionActionWindowSegment = useMemo(() => {
    for (const id of introductionElementIds) {
      const rest = id.startsWith("framework.window.") ? id.slice("framework.window.".length) : null;
      const actionIndex = rest?.indexOf(".action.") ?? -1;
      if (rest && actionIndex >= 0) return rest.slice(0, actionIndex);
    }
    return null;
  }, [introductionElementIds]);
  const introductionPanelTabId = useMemo(() => {
    for (const id of introductionElementIds) {
      if (id.startsWith("framework.panelTab.")) {
        const rest = id.slice("framework.panelTab.".length);
        return rest.endsWith(".firstDraggable") ? rest.slice(0, -".firstDraggable".length) : rest;
      }
    }
    return null;
  }, [introductionElementIds]);
  const introductionToolPickIds = useMemo(() => {
    const fromInteractions = (activeIntroductionStep?.interactions ?? []).filter((interaction) => interaction.on.kind === "tool").map((interaction) => interaction.on.id);
    if (fromInteractions.length > 0) return fromInteractions;
    return introductionElementIds.flatMap((id) => {
      const match = /^tool\.([a-z][a-zA-Z0-9]*)$/.exec(id);
      return match?.[1] ? [match[1]] : [];
    });
  }, [activeIntroductionStep, introductionElementIds]);
  const introductionPanelTabAnchor = introductionPanelTabId ? findPanelTabInDock(dock, introductionPanelTabId)?.anchor : void 0;
  const introductionUtilityWindowId = useMemo(() => {
    if (!introductionUtilityId || !session) return null;
    for (const kind of session.app.windowKinds) {
      const utilities = resolveUtilityNodes(session.app, kind, null, kind.id, appLabelsOverlay, uiTerminology, uiLocale);
      if (utilityNodeTreeContainsId(utilities, introductionUtilityId)) return kind.id;
    }
    return null;
  }, [appLabelsOverlay, introductionUtilityId, session, uiTerminology, uiLocale]);
  const introductionMeasureWindowId = useMemo(() => {
    if (!session || introductionElementIds.length === 0) return null;
    for (const kind of session.app.windowKinds) {
      const kindMeasures = kind.options.measures ?? [];
      if (introductionElementIds.some((id) => windowMeasureTreeContainsId(kindMeasures, id))) return kind.id;
      for (const [windowId, measures] of Object.entries(windowMeasuresByWindowId)) {
        if (!introductionElementIds.some((id) => windowMeasureTreeContainsId(measures, id))) continue;
        if (windowId === kind.id || extraWindowInstances.some((instance) => instance.id === windowId && instance.windowKindId === kind.id)) return kind.id;
      }
    }
    return null;
  }, [extraWindowInstances, introductionElementIds, session, windowMeasuresByWindowId]);
  const introductionToolId = useMemo(() => {
    if (introductionElementIds.length === 0) return null;
    for (const [toolId, measures] of Object.entries(toolMeasuresByToolId)) {
      if (introductionElementIds.some((id) => windowMeasureTreeContainsId(measures, id))) return toolId;
    }
    return null;
  }, [introductionElementIds, toolMeasuresByToolId]);
  const lastIntroductionToolIdRef = useRef(null);
  useEffect(() => {
    if (!introductionToolId || !session) {
      lastIntroductionToolIdRef.current = null;
      return;
    }
    if (lastIntroductionToolIdRef.current === introductionToolId && activeToolIdRef.current === introductionToolId) return;
    lastIntroductionToolIdRef.current = introductionToolId;
    if (activeToolIdRef.current === introductionToolId) return;
    onActionStable({ controllerId: session.app.controllerId, action: SET_ACTIVE_TOOL_ACTION_ID, args: { toolId: introductionToolId } });
  }, [introductionToolId, onActionStable, session]);
  const lastIntroductionToolPickStepIdRef = useRef(null);
  useEffect(() => {
    if (!session || introductionToolPickIds.length === 0 || !activeIntroductionStep) {
      lastIntroductionToolPickStepIdRef.current = null;
      return;
    }
    if (introductionToolId) return;
    if (lastIntroductionToolPickStepIdRef.current === activeIntroductionStep.id) return;
    lastIntroductionToolPickStepIdRef.current = activeIntroductionStep.id;
    for (const toolId of introductionToolPickIds) {
      if (activeToolIdRef.current === toolId) {
        onActionStable({ controllerId: session.app.controllerId, action: SET_ACTIVE_TOOL_ACTION_ID, args: { toolId: "" } });
      }
    }
    if (mobile) {
      const resolved2 = findPanelTabPath(mobilePanelTabs, FRAMEWORK_CATEGORY_TOOL_ID);
      if (resolved2) dispatch({ type: "SET_MOBILE_PANEL_PATH", value: resolved2 });
      dispatch({ type: "SET_MOBILE_PANEL_VISIBLE", value: true });
      return;
    }
    const toolAnchor = findPanelTabInDock(dock, FRAMEWORK_CATEGORY_TOOL_ID)?.anchor ?? "bottom-middle";
    const resolved = findPanelTabPath(dock.anchors[toolAnchor], FRAMEWORK_CATEGORY_TOOL_ID);
    if (resolved) dispatch({ type: "SET_PANEL_PATH", anchor: toolAnchor, value: resolved });
    dispatch({ type: "SET_PANEL_VISIBLE", anchor: toolAnchor, value: true });
  }, [activeIntroductionStep, dock, introductionToolId, introductionToolPickIds, mobile, mobilePanelTabs, onActionStable, session]);
  const lastIntroductionPanelTabIdRef = useRef(void 0);
  useEffect(() => {
    if (!introductionPanelTabId || !introductionPanelTabAnchor) {
      lastIntroductionPanelTabIdRef.current = void 0;
      return;
    }
    if (lastIntroductionPanelTabIdRef.current === introductionPanelTabId) return;
    lastIntroductionPanelTabIdRef.current = introductionPanelTabId;
    if (mobile) {
      const resolved2 = findPanelTabPath(mobilePanelTabs, introductionPanelTabId);
      if (resolved2) dispatch({ type: "SET_MOBILE_PANEL_PATH", value: resolved2 });
      dispatch({ type: "SET_MOBILE_PANEL_VISIBLE", value: true });
      return;
    }
    const resolved = findPanelTabPath(dock.anchors[introductionPanelTabAnchor], introductionPanelTabId);
    if (resolved) dispatch({ type: "SET_PANEL_PATH", anchor: introductionPanelTabAnchor, value: resolved });
    dispatch({ type: "SET_PANEL_VISIBLE", anchor: introductionPanelTabAnchor, value: true });
  }, [introductionPanelTabId, introductionPanelTabAnchor, dock, mobile, mobilePanelTabs]);
  useEffect(() => {
    if (!activeIntroductionStep) return;
    for (const interaction of activeIntroductionStep.interactions ?? []) {
      if (interaction.on.kind !== "panel") continue;
      const tabId = interaction.on.id;
      const located = findPanelTabInDock(dock, tabId);
      if (!located) continue;
      const panel2 = panels[located.anchor];
      if (!panel2.visible || !panel2.path.includes(tabId)) continue;
      completeIntroductionInteraction((candidate) => candidate.on.kind === "panel" && candidate.on.id === tabId);
    }
  }, [activeIntroductionStep, completeIntroductionInteraction, dock, panels]);
  const lastIntroductionExpandStepIdRef = useRef(null);
  useEffect(() => {
    const expandInteractions = (activeIntroductionStep?.interactions ?? []).filter((interaction) => interaction.on.kind === "expand");
    if (!activeIntroductionStep || expandInteractions.length === 0) {
      lastIntroductionExpandStepIdRef.current = null;
      return;
    }
    if (lastIntroductionExpandStepIdRef.current !== activeIntroductionStep.id) {
      lastIntroductionExpandStepIdRef.current = activeIntroductionStep.id;
      for (const interaction of expandInteractions) {
        const stateSuffix = `tree-section-${interaction.on.id}`;
        const catalogueKey = `${FRAMEWORK_PANEL_TAB_CATALOGUE_ID}.tree:${stateSuffix}`;
        dispatch({ type: "SET_TREE_OPEN_STATE", id: catalogueKey, open: false });
      }
      return;
    }
    for (const interaction of expandInteractions) {
      const sectionId = interaction.on.id;
      const stateSuffix = `tree-section-${sectionId}`;
      const expanded = Object.entries(treeOpenStates).some(([key, open]) => open && key.endsWith(stateSuffix));
      if (expanded) completeIntroductionInteraction((candidate) => candidate.on.kind === "expand" && candidate.on.id === sectionId);
    }
  }, [activeIntroductionStep, completeIntroductionInteraction, treeOpenStates]);
  const panelActivePaths = useMemo(() => {
    const result = {};
    for (const anchor of ANCHORS) result[anchor] = reconcileActivePath(dock.anchors[anchor], panels[anchor].path, panelTabChildren);
    return result;
  }, [panels, dock]);
  const lastStudioOverrideTabIdRef = useRef(void 0);
  useEffect(() => {
    if (!studioOverrideTabId || !studioOverrideAnchor) {
      lastStudioOverrideTabIdRef.current = void 0;
      return;
    }
    if (lastStudioOverrideTabIdRef.current === studioOverrideTabId) return;
    lastStudioOverrideTabIdRef.current = studioOverrideTabId;
    if (mobile) {
      if (mobilePanelPath[0] === FRAMEWORK_CATEGORY_DISPLAY_ID) return;
      const resolved2 = findPanelTabPath(mobilePanelTabs, studioOverrideTabId);
      if (resolved2) dispatch({ type: "SET_MOBILE_PANEL_PATH", value: resolved2 });
      return;
    }
    if (panels[studioOverrideAnchor].path[0] === FRAMEWORK_CATEGORY_DISPLAY_ID) return;
    const resolved = findPanelTabPath(dock.anchors[studioOverrideAnchor], studioOverrideTabId);
    if (resolved) dispatch({ type: "SET_PANEL_PATH", anchor: studioOverrideAnchor, value: resolved });
  }, [studioOverrideTabId, studioOverrideAnchor, dock, panels, mobile, mobilePanelTabs, mobilePanelPath]);
  const lastDetailsOverrideTabIdRef = useRef(void 0);
  useEffect(() => {
    if (!detailsOverrideTabId || !detailsOverrideAnchor) {
      lastDetailsOverrideTabIdRef.current = void 0;
      return;
    }
    if (lastDetailsOverrideTabIdRef.current === detailsOverrideTabId) return;
    lastDetailsOverrideTabIdRef.current = detailsOverrideTabId;
    if (detailsOverrideAnchor === studioOverrideAnchor) return;
    if (mobile) {
      if (settingsRightTabs.some((tab) => tab.id === mobilePanelPath[0])) return;
      const resolved2 = findPanelTabPath(mobilePanelTabs, detailsOverrideTabId);
      if (resolved2) dispatch({ type: "SET_MOBILE_PANEL_PATH", value: resolved2 });
      return;
    }
    if (settingsRightTabs.some((tab) => tab.id === panels[detailsOverrideAnchor].path[0])) return;
    const resolved = findPanelTabPath(dock.anchors[detailsOverrideAnchor], detailsOverrideTabId);
    if (resolved) dispatch({ type: "SET_PANEL_PATH", anchor: detailsOverrideAnchor, value: resolved });
  }, [detailsOverrideTabId, detailsOverrideAnchor, studioOverrideAnchor, dock, panels, settingsRightTabs, mobile, mobilePanelTabs, mobilePanelPath]);
  const mobilePanel = useMemo(() => {
    if (mobilePanelTabs.length === 0) return void 0;
    return {
      visible: mobilePanelVisible,
      tabs: mobilePanelTabs,
      activeTabPath: mobilePanelPath,
      onActiveTabPathChange: (path) => {
        dispatch({ type: "SET_MOBILE_PANEL_PATH", value: path });
        const tabId = path[path.length - 1];
        if (tabId && studioMode && session?.app.id === hostAppId && findPanelTabNode(mobilePanelTabs, path)?.kind === "leaf") {
          onAction({ controllerId: session.app.controllerId, action: "setActivePanelTab", args: { tabId } });
        }
      },
      pathMemory: panelPathMemory,
      onPathMemoryChange: (value) => dispatch({ type: "SET_PANEL_PATH_MEMORY", value }),
      treeOpenStates,
      onTreeOpenStateChange: (id, open) => dispatch({ type: "SET_TREE_OPEN_STATE", id, open }),
      // ♻️ Lazy tool/command trees read measures + active tool from refs — revision forces re-resolve.
      treeContentRevision: { activeToolId, toolMeasuresByToolId, actionPaneStagedArgsByKey }
    };
  }, [mobilePanelVisible, mobilePanelPath, mobilePanelTabs, onAction, panelPathMemory, session, studioMode, treeOpenStates, hostAppId, activeToolId, toolMeasuresByToolId, actionPaneStagedArgsByKey]);
  useEffect(() => {
    if (exampleOptions.length === 0) return;
    dispatch({ type: "SET_ACTIVE_EXAMPLE_ID", value: (current) => !current || exampleOptions.some((option) => option.id === current) ? current : "" });
  }, [exampleOptions, session?.app.id, session?.pluginId]);
  useEffect(() => {
    if (exampleOptions.length === 0 || !session) return;
    if (studioMode) {
      noExampleResetInstanceIdRef.current = session.instanceId;
      return;
    }
    if (noExampleResetInstanceIdRef.current === session.instanceId) return;
    noExampleResetInstanceIdRef.current = session.instanceId;
    const exampleId = resolveBootExampleId(activeExampleId, exampleOptions, defaults.exampleId);
    if (exampleId !== activeExampleId) {
      dispatch({ type: "SET_ACTIVE_EXAMPLE_ID", value: exampleId });
    }
    dispatchActiveExample(exampleId);
  }, [activeExampleId, defaults.exampleId, dispatchActiveExample, exampleOptions, session, studioMode]);
  const buildPanelSelectionProps = useCallback(
    (anchor) => ({
      tabs: dock.anchors[anchor],
      visible: panels[anchor].visible,
      onVisibleChange: (value) => {
        dispatch({ type: "SET_PANEL_VISIBLE", anchor, value });
        noteShellCommand("shell.panelToggle", shellLabel("ui.shellCommand.panelToggle"), { anchor, visible: value });
      },
      activeTabPath: panelActivePaths[anchor],
      onActiveTabPathChange: (path) => {
        const pathChanged = (panelActivePaths[anchor] ?? []).join("/") !== path.join("/");
        dispatch({ type: "SET_PANEL_PATH", anchor, value: path });
        if (anchor === "bottom-middle" && panels[anchor].path[1] !== path[1]) {
          dispatch({ type: "SET_COMMAND_EXPANDED", value: null });
        }
        const tabId = path[path.length - 1];
        if (anchor === "bottom-middle" && session && findPanelTabNode(dock.anchors[anchor], path)?.kind === "leaf") {
          const selectedToolId = toolIdFromPanelTabId(tabId);
          if (selectedToolId && selectedToolId !== activeToolIdRef.current) {
            onAction({ controllerId: session.app.controllerId, action: SET_ACTIVE_TOOL_ACTION_ID, args: { toolId: selectedToolId } });
          }
        }
        if (tabId && studioMode && session?.app.id === hostAppId && findPanelTabNode(dock.anchors[anchor], path)?.kind === "leaf") {
          onAction({ controllerId: session.app.controllerId, action: "setActivePanelTab", args: { tabId } });
        }
        if (pathChanged && tabId) noteShellCommand("shell.panelTab", shellLabel("ui.shellCommand.panelTab"), { anchor, tabId });
      },
      pathMemory: panelPathMemory,
      onPathMemoryChange: (value) => dispatch({ type: "SET_PANEL_PATH_MEMORY", value })
    }),
    [dock, onAction, panelActivePaths, panelPathMemory, panels, session, studioMode, hostAppId, noteShellCommand]
  );
  const navbarItems = useMemo(() => {
    if (!session) return [];
    const logoAndTitle = /* @__PURE__ */ jsxDEV("div", { className: "flex min-w-0 shrink-0 items-center gap-single", children: [
      brand?.logoSvg ? /* @__PURE__ */ jsxDEV(ShellBrandLogo, { svg: brand.logoSvg, className: "size-workbench shrink-0" }, void 0, false, {
        fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
        lineNumber: 4246,
        columnNumber: 27
      }, this) : /* @__PURE__ */ jsxDEV(SemioLogo, { className: "size-workbench shrink-0" }, void 0, false, {
        fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
        lineNumber: 4246,
        columnNumber: 104
      }, this),
      /* @__PURE__ */ jsxDEV("span", { "data-slot": "app-name", className: cn("px-single", shellChromeTitleClassName), children: appDocumentLabel(resolveAppDocument(session.app, uiTerminology)) }, void 0, false, {
        fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
        lineNumber: 4247,
        columnNumber: 9
      }, this)
    ] }, "logoAndTitle", true, {
      fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
      lineNumber: 4245,
      columnNumber: 5
    }, this);
    const showExampleSelect = exampleOptions.length > 0 && !locks.exampleId && (!studioMode || session.app.id !== landingAppId);
    if (mobile) {
      return [
        { key: "logoAndTitle", content: logoAndTitle },
        navbarFillItem("navbarTrailingFill"),
        {
          key: "mobilePanelToggle",
          content: /* @__PURE__ */ jsxDEV(Toggle, { id: "ui.mobilePanel.toggle", pressed: mobilePanelVisible, onPressedChange: (value) => dispatch({ type: "SET_MOBILE_PANEL_VISIBLE", value }), icon: "panel-left" }, void 0, false, {
            fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
            lineNumber: 4262,
            columnNumber: 18
          }, this)
        }
      ];
    }
    const centerContent = [logoAndTitle];
    if (showExampleSelect && exampleSelectElement) centerContent.push(exampleSelectElement);
    if (modeSwitcherElement) centerContent.push(modeSwitcherElement);
    return [
      { key: "topLeftPanelTabs", content: /* @__PURE__ */ jsxDEV(PanelChromeTabBar, { anchor: "top-left", ...buildPanelSelectionProps("top-left") }, void 0, false, {
        fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
        lineNumber: 4272,
        columnNumber: 41
      }, this) },
      navbarFillItem("navbarTrailingFill"),
      { key: "topRightPanelTabs", content: /* @__PURE__ */ jsxDEV(PanelChromeTabBar, { anchor: "top-right", ...buildPanelSelectionProps("top-right") }, void 0, false, {
        fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
        lineNumber: 4274,
        columnNumber: 42
      }, this) },
      {
        key: "center",
        centered: true,
        content: /* @__PURE__ */ jsxDEV("div", { className: "flex min-w-0 items-center gap-double", children: [
          centerContent,
          /* @__PURE__ */ jsxDEV(PanelChromeTabBar, { anchor: "top-middle", ...buildPanelSelectionProps("top-middle") }, void 0, false, {
            fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
            lineNumber: 4281,
            columnNumber: 13
          }, this)
        ] }, void 0, true, {
          fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
          lineNumber: 4279,
          columnNumber: 7
        }, this)
      }
    ];
  }, [brand, buildPanelSelectionProps, exampleOptions, exampleSelectElement, locks.exampleId, mobile, mobilePanelVisible, modeSwitcherElement, session, uiTerminology, studioMode, landingAppId]);
  const searchItems = useMemo(() => {
    if (!session) return [];
    const items = [];
    for (const tab of flattenPanelTabLeaves(session.app.panelTabs)) {
      const tabId = panelTabKindId(tab.kind);
      items.push({
        id: `panel.${tabId}`,
        label: resolvePanelTabLabel(appLabelsOverlay, tabId, resolveManifestLabel(tab.label, uiTerminology, uiLocale)),
        category: shellLabel("ui.search.category.panels"),
        icon: /* @__PURE__ */ jsxDEV(Icon, { icon: "panel-left", size: "small" }, void 0, false, {
          fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
          lineNumber: 4297,
          columnNumber: 15
        }, this),
        onSelect: () => onAction({ controllerId: session.app.controllerId, action: "setActivePanelTab", args: { tabId } })
      });
    }
    for (const kind of session.app.windowKinds) {
      items.push({
        id: `window.${kind.id}`,
        label: resolveAppLabel(appLabelsOverlay, "windowKind", kind.id, resolveManifestLabel(kind.label, uiTerminology, uiLocale)),
        category: shellLabel("ui.search.category.windows"),
        icon: /* @__PURE__ */ jsxDEV(Icon, { icon: "app-window", size: "small" }, void 0, false, {
          fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
          lineNumber: 4306,
          columnNumber: 15
        }, this),
        onSelect: () => dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: kind.id })
      });
    }
    const keysByActionId2 = new Map(session.app.keybindings.map((binding) => [binding.action.action, binding.keys]));
    const declaredActionIds = /* @__PURE__ */ new Set();
    const hostWindowForAction = (actionId) => {
      for (const kind of session.app.windowKinds) {
        if (resolveWindowActions(session.app, kind).some((entry) => entry.id === actionId)) return kind.id;
      }
      return activeWindowId ?? session.app.windowKinds[0]?.id;
    };
    for (const action of session.app.actions ?? []) {
      if (!action.inPalette) continue;
      declaredActionIds.add(action.id);
      const argCarrying = actionRequiresStagedForm(action);
      const resolvedActionLabel = resolveAppLabel(appLabelsOverlay, "action", action.id, resolveManifestLabel(action.label, uiTerminology, uiLocale));
      items.push({
        id: `action.${action.id}`,
        // ✍️ Arg-carrying actions never fire from the palette (P3): the "…" entry activates the hosting
        // window, unfolds its top-left Actions pane, and expands this action's staged form instead of dispatching.
        label: argCarrying ? `${resolvedActionLabel}…` : resolvedActionLabel,
        description: action.keys ?? keysByActionId2.get(action.id),
        category: action.category ?? (action.kind === "history" ? shellLabel("ui.ribbon.parent.history") : shellLabel("ui.ribbon.parent.actions")),
        onSelect: () => {
          if (argCarrying) {
            const windowId = hostWindowForAction(action.id);
            if (windowId) {
              dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: windowId });
              dispatch({ type: "SET_ACTION_PANE_FOLDED", windowId, value: false });
              dispatch({ type: "SET_ACTION_PANE_EXPANDED", windowId, value: action.id });
            }
            dispatch({ type: "SET_SEARCH_OPEN", value: false });
            return;
          }
          onAction({ controllerId: session.app.controllerId, action: action.id });
        }
      });
    }
    for (const binding of session.app.keybindings) {
      if (declaredActionIds.has(binding.action.action)) continue;
      items.push({
        id: `keybinding.${binding.keys}`,
        label: binding.action.action,
        description: binding.keys,
        category: shellLabel("ui.ribbon.parent.actions"),
        onSelect: () => onAction(binding.action)
      });
    }
    for (const { definition, source } of resolvedCommands) {
      if (!definition.inPalette) continue;
      const argCarrying = (definition.args?.length ?? 0) > 0;
      items.push({
        id: `command.${definition.id}`,
        label: argCarrying ? `${definition.label}…` : definition.label,
        description: definition.keys,
        category: commandCategoryLabel(definition.category),
        onSelect: () => {
          if (argCarrying) {
            const commandPath = [FRAMEWORK_CATEGORY_COMMAND_ID, `command.category.${definition.category}`];
            if (mobile) {
              dispatch({ type: "SET_MOBILE_PANEL_VISIBLE", value: true });
              dispatch({ type: "SET_MOBILE_PANEL_PATH", value: commandPath });
            } else {
              dispatch({ type: "SET_PANEL_VISIBLE", anchor: "bottom-middle", value: true });
              dispatch({ type: "SET_PANEL_PATH", anchor: "bottom-middle", value: commandPath });
            }
            dispatch({ type: "SET_COMMAND_EXPANDED", value: definition.id });
            dispatch({ type: "SET_SEARCH_OPEN", value: false });
            return;
          }
          onCommand(source, definition.id);
        }
      });
    }
    if (studioMode && panel) {
      for (const program of panel.programs) {
        items.push({
          id: `spawn.${program.pluginId}`,
          label: `${shellLabel("ui.palette.spawnPrefix")} ${appDocumentLabel(resolveDocumentByAppId(loadedPlugins, program.appId, program.document, uiTerminology))}`,
          category: shellLabel("ui.search.category.catalogue"),
          onSelect: () => onAction({ controllerId: hostControllerId ?? "", action: "spawnApp", args: { pluginId: program.pluginId } })
        });
      }
      items.push(
        {
          id: "studio.undo",
          label: shellLabel("ui.palette.undo"),
          category: shellLabel("ui.search.category.studio"),
          icon: /* @__PURE__ */ jsxDEV(Icon, { icon: "undo-2", size: "small" }, void 0, false, {
            fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
            lineNumber: 4402,
            columnNumber: 17
          }, this),
          onSelect: () => onAction({ controllerId: hostControllerId ?? "", action: "undo" })
        },
        {
          id: "studio.redo",
          label: shellLabel("ui.palette.redo"),
          category: shellLabel("ui.search.category.studio"),
          icon: /* @__PURE__ */ jsxDEV(Icon, { icon: "redo-2", size: "small" }, void 0, false, {
            fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
            lineNumber: 4409,
            columnNumber: 17
          }, this),
          onSelect: () => onAction({ controllerId: hostControllerId ?? "", action: "redo" })
        },
        {
          id: "studio.home",
          label: shellLabel("ui.palette.goHome"),
          category: shellLabel("ui.search.category.navigation"),
          onSelect: () => onAction({ controllerId: hostControllerId ?? "", action: "goHome" })
        }
      );
    }
    return items;
  }, [activeWindowId, appLabelsOverlay, loadedPlugins, mobile, onAction, onCommand, panel, resolvedCommands, session, studioMode, uiLocale, uiTerminology, hostControllerId]);
  const modeWindows = useMemo(
    () => {
      if (!session) return [];
      const actionPaneSlice = { expandedByWindowId: actionPaneExpandedByWindowId, stagedArgsByKey: actionPaneStagedArgsByKey, activeUtilityByWindowId };
      const actionsFoldedFor = (windowId, windowKindId = windowId) => introductionTargetsWindow(windowId, windowKindId, null, introductionActionWindowSegment) ? false : actionPaneFoldedByWindowId[windowId] ?? true;
      const utilityBarFoldedFor = (windowId, windowKindId = windowId) => introductionTargetsWindow(windowId, windowKindId, introductionUtilityWindowId) ? false : void 0;
      const measuresFoldedFor = (windowId, windowKindId = windowId) => introductionTargetsWindow(windowId, windowKindId, introductionMeasureWindowId) ? false : void 0;
      const onActionsFoldedFor = (windowId) => (folded) => dispatch({ type: "SET_ACTION_PANE_FOLDED", windowId, value: folded });
      const cursorFor = (app, windowId) => {
        const utilityId = activeUtilityByWindowId[windowId];
        const cursor = utilityId ? (app.utilities ?? []).find((utility) => utility.id === utilityId)?.cursor : void 0;
        return cursor ? { cursor } : void 0;
      };
      if (studioMode && spawnedWindowUi && panel?.activeSpawnedId) {
        const spawned = panel.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId);
        if (spawned) {
          const spawnedApp = loadedPlugins.find((entry) => entry.handle.pluginId === spawned.pluginId)?.manifest.apps.find((candidate) => candidate.id === spawned.appId);
          const windowKind = spawnedApp?.windowKinds[0];
          const chrome = windowKind ? spawnedWindowChromeForKind(windowKind, spawned.id, spawnedWindowEngagements, spawnedWindowMeasures, activeUtilityByWindowId[spawned.id], onActionStable) : void 0;
          const spawnedUtilities = spawnedApp && windowKind ? resolveUtilityNodes(spawnedApp, windowKind, activeUtilityByWindowId[spawned.id], spawned.id, appLabelsOverlay, uiTerminology, uiLocale) : [];
          return [
            {
              id: spawned.id,
              title: wireLabel(appDocumentLabel(spawnedApp ? resolveAppDocument(spawnedApp, uiTerminology) : spawned.document)),
              fill: true,
              showControls: true,
              measures: chrome?.measures,
              measuresFolded: measuresFoldedFor(spawned.id, windowKind?.id ?? spawned.id),
              engagement: chrome?.engagement,
              search: chrome?.search,
              utilityBar: spawnedApp && windowKind ? utilityBarNode(spawnedUtilities, spawned.id, onActionStable, introductionUtilityId, chrome?.utilityOptions) : void 0,
              utilityBarFolded: utilityBarFoldedFor(spawned.id, windowKind?.id ?? spawned.id),
              actionPane: spawnedApp && windowKind ? windowActionPaneNode(spawnedApp, windowKind, spawned.id, actionPaneSlice, onActionStable, dispatch, appLabelsOverlay, uiTerminology, uiLocale) : void 0,
              actionsFolded: actionsFoldedFor(spawned.id, windowKind?.id ?? spawned.id),
              onActionsFoldedChange: onActionsFoldedFor(spawned.id),
              children: /* @__PURE__ */ jsxDEV(ChromeAwareWindowScrollSurface, { className: "relative flex h-full min-h-0 min-w-0 flex-1 flex-col overflow-hidden", style: spawnedApp ? cursorFor(spawnedApp, spawned.id) : void 0, children: /* @__PURE__ */ jsxDEV(ShellFaultBoundary, { boundaryId: `window-${spawned.id}`, fallbackLabel: shellLabel("ui.common.renderError"), children: /* @__PURE__ */ jsxDEV(InterpretedUiNode, { node: spawnedWindowUi, onAction: onActionStable }, void 0, false, {
                fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
                lineNumber: 4467,
                columnNumber: 19
              }, this) }, void 0, false, {
                fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
                lineNumber: 4466,
                columnNumber: 17
              }, this) }, void 0, false, {
                fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
                lineNumber: 4465,
                columnNumber: 11
              }, this)
            }
          ];
        }
      }
      if (Object.keys(windowUiByWindowId).length === 0) return [];
      const baseWindows = session.app.windowKinds.map((kind) => {
        const utilities = resolveUtilityNodes(session.app, kind, activeUtilityByWindowId[kind.id], kind.id, appLabelsOverlay, uiTerminology, uiLocale);
        const chrome = windowMeasuresChrome(windowMeasuresByWindowId[kind.id] ?? kind.options.measures, activeUtilityByWindowId[kind.id], kind.id, onActionStable);
        const resolvedEngagement = resolveWindowEngagement(kind, kind.id, windowEngagementsByWindowId);
        return {
          id: kind.id,
          iconId: windowIconsById[kind.id] ?? kind.iconId,
          title: windowTitlesById[kind.id] ?? appWindowDocumentLabel(session.app, uiTerminology, resolveAppLabel(appLabelsOverlay, "windowKind", kind.id, resolveManifestLabel(kind.label, uiTerminology, uiLocale)), uiLocale),
          fill: true,
          showControls: true,
          measures: chrome.measures,
          measuresFolded: measuresFoldedFor(kind.id, kind.id),
          engagement: windowEngagementToSpec(resolvedEngagement, onActionStable),
          search: windowEngagementToSearchSpec(resolvedEngagement, onActionStable),
          utilityBar: utilityBarNode(utilities, kind.id, onActionStable, introductionUtilityId, chrome.utilityOptions),
          utilityBarFolded: utilityBarFoldedFor(kind.id, kind.id),
          actionPane: windowActionPaneNode(session.app, kind, kind.id, actionPaneSlice, onActionStable, dispatch, appLabelsOverlay, uiTerminology, uiLocale),
          actionsFolded: actionsFoldedFor(kind.id, kind.id),
          onActionsFoldedChange: onActionsFoldedFor(kind.id),
          status: declarativeSurfaceStatus(windowUiByWindowId[kind.id]),
          skeleton: /* @__PURE__ */ jsxDEV(WindowBodySkeleton, {}, void 0, false, {
            fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
            lineNumber: 4496,
            columnNumber: 19
          }, this),
          children: /* @__PURE__ */ jsxDEV(ChromeAwareWindowScrollSurface, { id: childElementId("framework.window", kind.id), className: "relative flex h-full min-h-0 min-w-0 flex-1 flex-col overflow-hidden", style: cursorFor(session.app, kind.id), children: /* @__PURE__ */ jsxDEV(WindowInstanceIdContext.Provider, { value: kind.id, children: /* @__PURE__ */ jsxDEV(ShellFaultBoundary, { boundaryId: `window-${kind.id}`, fallbackLabel: shellLabel("ui.common.renderError"), children: /* @__PURE__ */ jsxDEV(InterpretedUiNode, { node: windowUiByWindowId[kind.id] ?? pendingWindowUiNode(), onAction: onActionStable }, void 0, false, {
            fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
            lineNumber: 4501,
            columnNumber: 17
          }, this) }, void 0, false, {
            fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
            lineNumber: 4500,
            columnNumber: 15
          }, this) }, void 0, false, {
            fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
            lineNumber: 4499,
            columnNumber: 13
          }, this) }, void 0, false, {
            fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
            lineNumber: 4498,
            columnNumber: 9
          }, this)
        };
      });
      const extraWindows = extraWindowInstances.flatMap((instance) => {
        const kind = session.app.windowKinds.find((entry) => entry.id === instance.windowKindId);
        if (!kind) return [];
        const utilities = resolveUtilityNodes(session.app, kind, activeUtilityByWindowId[instance.id], instance.id, appLabelsOverlay, uiTerminology, uiLocale);
        const chrome = windowMeasuresChrome(windowMeasuresByWindowId[instance.id] ?? kind.options.measures, activeUtilityByWindowId[instance.id], instance.id, onActionStable);
        const resolvedEngagement = resolveWindowEngagement(kind, instance.id, windowEngagementsByWindowId);
        return [
          {
            id: instance.id,
            iconId: windowIconsById[instance.id] ?? kind.iconId,
            title: windowTitlesById[instance.id] ?? instance.title,
            fill: true,
            showControls: true,
            measures: chrome.measures,
            measuresFolded: measuresFoldedFor(instance.id, instance.windowKindId),
            engagement: windowEngagementToSpec(resolvedEngagement, onActionStable),
            search: windowEngagementToSearchSpec(resolvedEngagement, onActionStable),
            utilityBar: utilityBarNode(utilities, instance.id, onActionStable, introductionUtilityId, chrome.utilityOptions),
            utilityBarFolded: utilityBarFoldedFor(instance.id, instance.windowKindId),
            actionPane: windowActionPaneNode(session.app, kind, instance.id, actionPaneSlice, onActionStable, dispatch, appLabelsOverlay, uiTerminology, uiLocale),
            actionsFolded: actionsFoldedFor(instance.id, instance.windowKindId),
            onActionsFoldedChange: onActionsFoldedFor(instance.id),
            status: declarativeSurfaceStatus(windowUiByWindowId[instance.id]),
            skeleton: /* @__PURE__ */ jsxDEV(WindowBodySkeleton, {}, void 0, false, {
              fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
              lineNumber: 4536,
              columnNumber: 19
            }, this),
            children: /* @__PURE__ */ jsxDEV(
              ChromeAwareWindowScrollSurface,
              {
                id: childElementId("framework.window", instance.id),
                "data-element-alias": childElementId("framework.window", kind.id),
                className: "relative flex h-full min-h-0 min-w-0 flex-1 flex-col overflow-hidden",
                style: cursorFor(session.app, instance.id),
                children: /* @__PURE__ */ jsxDEV(WindowInstanceIdContext.Provider, { value: instance.id, children: /* @__PURE__ */ jsxDEV(ShellFaultBoundary, { boundaryId: `window-${instance.id}`, fallbackLabel: shellLabel("ui.common.renderError"), children: /* @__PURE__ */ jsxDEV(InterpretedUiNode, { node: windowUiByWindowId[instance.id] ?? pendingWindowUiNode(), onAction: onActionStable }, void 0, false, {
                  fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
                  lineNumber: 4546,
                  columnNumber: 19
                }, this) }, void 0, false, {
                  fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
                  lineNumber: 4545,
                  columnNumber: 17
                }, this) }, void 0, false, {
                  fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
                  lineNumber: 4544,
                  columnNumber: 15
                }, this)
              },
              void 0,
              false,
              {
                fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
                lineNumber: 4538,
                columnNumber: 9
              },
              this
            )
          }
        ];
      });
      return [...baseWindows, ...extraWindows];
    },
    [
      actionPaneExpandedByWindowId,
      actionPaneFoldedByWindowId,
      actionPaneStagedArgsByKey,
      activeUtilityByWindowId,
      appLabelsOverlay,
      extraWindowInstances,
      introductionActionWindowSegment,
      introductionUtilityId,
      introductionUtilityWindowId,
      loadedPlugins,
      onActionStable,
      panel,
      session,
      spawnedWindowEngagements,
      spawnedWindowMeasures,
      spawnedWindowUi,
      studioMode,
      uiLocale,
      uiTerminology,
      windowEngagementsByWindowId,
      windowMeasuresByWindowId,
      windowTitlesById,
      windowIconsById,
      windowUiByWindowId
    ]
  );
  const effectiveModeLayout = useMemo(
    () => shellLayout ?? (session ? resolveFrameworkLayoutSeed(session.app.defaultLayout, session.app.windowKinds, appLabelsOverlay, uiTerminology, uiLocale).modeLayout : { kind: "stack", children: [] }),
    [appLabelsOverlay, session, shellLayout, uiTerminology, uiLocale]
  );
  const handleActiveWindowChange = useCallback(
    (value) => {
      dispatch({ type: "SET_ACTIVE_WINDOW_ID", value });
      if (value) noteShellCommand("shell.windowActivate", shellLabel("ui.shellCommand.windowActivate"), { windowId: value });
    },
    [noteShellCommand]
  );
  const layoutChangeSettleTimeoutRef = useRef(null);
  const layoutChangeClassificationRef = useRef(null);
  const layoutChangePreviousRef = useRef(effectiveModeLayout);
  useEffect(() => {
    layoutChangePreviousRef.current = effectiveModeLayout;
  }, [effectiveModeLayout]);
  useEffect(
    () => () => {
      if (layoutChangeSettleTimeoutRef.current) clearTimeout(layoutChangeSettleTimeoutRef.current);
    },
    []
  );
  const handleModeLayoutChange = useCallback(
    (value) => {
      dispatch({ type: "SET_SHELL_LAYOUT", value });
      const classification = classifyWindowLayoutChange(layoutChangePreviousRef.current, value);
      layoutChangePreviousRef.current = value;
      if (classification) layoutChangeClassificationRef.current = classification;
      if (layoutChangeSettleTimeoutRef.current) clearTimeout(layoutChangeSettleTimeoutRef.current);
      layoutChangeSettleTimeoutRef.current = setTimeout(() => {
        layoutChangeSettleTimeoutRef.current = null;
        const finalClassification = layoutChangeClassificationRef.current;
        layoutChangeClassificationRef.current = null;
        if (finalClassification === "resize") noteShellCommand("shell.windowResize", shellLabel("ui.shellCommand.windowResize"));
        else if (finalClassification === "rearrange") noteShellCommand("shell.windowMove", shellLabel("ui.shellCommand.windowMove"));
      }, LAYOUT_CHANGE_SETTLE_MS);
    },
    [noteShellCommand]
  );
  const canvas = useMemo(() => {
    if (studioMode && shellRoute.kind === "notFound") {
      return /* @__PURE__ */ jsxDEV(ShellRouteNotFoundPage, { path: shellRoute.path, onHome: () => navigateHistory("/") }, void 0, false, {
        fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
        lineNumber: 4633,
        columnNumber: 14
      }, this);
    }
    const supervisorPluginId = primaryPluginId;
    const supervisorState = supervisorPluginId ? pluginSupervisorById[supervisorPluginId] : void 0;
    if (supervisorState === "crashed" || supervisorState === "quarantined") {
      return /* @__PURE__ */ jsxDEV(
        PluginRecoveryPanel,
        {
          pluginId: supervisorPluginId,
          quarantined: supervisorState === "quarantined",
          onRestart: () => {
            dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId: supervisorPluginId, value: "restarting" });
            void reloadPlugin(supervisorPluginId);
          },
          onDisable: () => {
            dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId: supervisorPluginId, value: "quarantined" });
            if (supervisorPluginId !== primaryPluginId) void uninstallPlugin(supervisorPluginId);
          }
        },
        void 0,
        false,
        {
          fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
          lineNumber: 4639,
          columnNumber: 9
        },
        this
      );
    }
    if (error)
      return /* @__PURE__ */ jsxDEV("p", { className: "p-double text-sm text-destructive", role: "alert", "data-semio-os-shell-error": "", children: error }, void 0, false, {
        fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
        lineNumber: 4655,
        columnNumber: 7
      }, this);
    if (!session) return /* @__PURE__ */ jsxDEV(CanvasSkeleton, { label: shellLabel("ui.common.loadingPlugins"), className: cn(loadingBorderClass, "h-full w-full") }, void 0, false, {
      fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
      lineNumber: 4659,
      columnNumber: 26
    }, this);
    const modes = session.app.modes.length > 0 ? session.app.modes : [{ id: session.app.id, label: appDocumentLabel(resolveAppDocument(session.app, uiTerminology)) }];
    const studioHomeBar = studioMode && session.app.id === hostAppId && !panel?.activeSpawnedId ? /* @__PURE__ */ jsxDEV(
      "button",
      {
        type: "button",
        className: cn(borderNormalBottomClass, "px-single py-single text-left text-sm text-muted-foreground hover:bg-muted/40 hover:text-foreground"),
        onClick: () => onAction({ controllerId: session.app.controllerId, action: "goHome" }),
        children: [
          "← ",
          shellLabel("ui.common.home")
        ]
      },
      void 0,
      true,
      {
        fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
        lineNumber: 4663,
        columnNumber: 5
      },
      this
    ) : null;
    const focusedSpawned = panel?.activeSpawnedId ? panel.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId) : void 0;
    const focusedBar = focusedSpawned ? /* @__PURE__ */ jsxDEV("div", { className: cn(borderNormalBottomClass, "flex items-center gap-single px-single py-single text-sm text-muted-foreground"), children: [
      /* @__PURE__ */ jsxDEV("button", { type: "button", className: "hover:text-foreground", onClick: () => openSpaceIdRef.current ? navigateHistory(`/spaces/${openSpaceIdRef.current}`) : onAction({ controllerId: session.app.controllerId, action: "closeFocusedInstance" }), children: [
        "← ",
        shellLabel("ui.common.backToWorkflow")
      ] }, void 0, true, {
        fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
        lineNumber: 4674,
        columnNumber: 9
      }, this),
      /* @__PURE__ */ jsxDEV("span", { children: "·" }, void 0, false, {
        fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
        lineNumber: 4677,
        columnNumber: 9
      }, this),
      /* @__PURE__ */ jsxDEV("span", { children: appDocumentLabel(resolveDocumentByAppId(loadedPlugins, focusedSpawned.appId, focusedSpawned.document, uiTerminology)) }, void 0, false, {
        fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
        lineNumber: 4678,
        columnNumber: 9
      }, this)
    ] }, void 0, true, {
      fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
      lineNumber: 4673,
      columnNumber: 5
    }, this) : null;
    return /* @__PURE__ */ jsxDEV("div", { className: "flex h-full min-h-0 flex-col overflow-hidden", children: [
      studioHomeBar,
      focusedBar,
      /* @__PURE__ */ jsxDEV(
        "input",
        {
          ref: importSpaceInputRef,
          type: "file",
          accept: ".spk,.dsl,.ops,application/octet-stream",
          className: "hidden",
          onChange: (event) => {
            const file = event.target.files?.[0];
            if (!file) return;
            if (file.name.toLowerCase().endsWith(".pack")) {
              const reader = new FileReader();
              reader.onload = () => {
                const payload = typeof reader.result === "string" ? reader.result : "";
                onAction({ controllerId: landingControllerId ?? "", action: "importSpacePackPayload", args: { payload } });
                event.target.value = "";
              };
              reader.readAsDataURL(file);
              return;
            }
            void file.text().then((json) => {
              onAction({ controllerId: landingControllerId ?? "", action: "importSpace", args: { json } });
              event.target.value = "";
            });
          }
        },
        void 0,
        false,
        {
          fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
          lineNumber: 4685,
          columnNumber: 9
        },
        this
      ),
      /* @__PURE__ */ jsxDEV("div", { className: "min-h-0 flex-1", children: /* @__PURE__ */ jsxDEV(ShellFaultBoundary, { boundaryId: "session-canvas", fallbackLabel: shellLabel("ui.common.renderError"), children: /* @__PURE__ */ jsxDEV(
        App,
        {
          modes: modes.map((mode) => ({ id: mode.id, label: resolveAppLabel(appLabelsOverlay, "mode", mode.id, resolveManifestLabel(mode.label, uiTerminology, uiLocale)), children: null })),
          activeModeId: session.viewState.activeModeId ?? modes[0]?.id ?? session.app.id,
          onActiveModeChange: applyModeChange,
          chrome: false,
          children: /* @__PURE__ */ jsxDEV(
            Mode,
            {
              className: "h-full w-full",
              mobile,
              windows: modeWindows,
              layout: effectiveModeLayout,
              activeWindowId,
              onActiveWindowChange: handleActiveWindowChange,
              onLayoutChange: handleModeLayoutChange,
              onTemplateDrop: mobile ? void 0 : handleTemplateDrop,
              onWindowClose: (windowId) => {
                noteShellCommand("shell.windowClose", shellLabel("ui.shellCommand.windowClose"), { windowId });
                if (studioMode && panel?.spawnedApps.some((entry) => entry.id === windowId)) {
                  const closedSpawned = panel.spawnedApps.find((entry) => entry.id === windowId);
                  const nextSpawned = panel.spawnedApps.filter((entry) => entry.id !== windowId);
                  updateSpacePanel(buildSpacePanelState(panel.programs, nextSpawned, panel.activePanelTab, nextSpawned[0]?.id));
                  if (closedSpawned) {
                    const closedPlugin = loadedPlugins.find((entry) => entry.handle.pluginId === closedSpawned.pluginId)?.handle;
                    void closedPlugin?.destroyApp(closedSpawned.instanceId).catch(() => {
                    });
                  }
                }
                clearPendingWorldProjection(windowId);
                dispatch({
                  type: "SET_EXTRA_WINDOW_INSTANCES",
                  value: (current) => {
                    const next = current.filter((entry) => entry.id !== windowId);
                    extraWindowInstancesRef.current = next;
                    return next;
                  }
                });
                dispatch({
                  type: "SET_SHELL_LAYOUT",
                  value: (current) => current ?? resolveFrameworkLayoutSeed(session.app.defaultLayout, session.app.windowKinds, appLabelsOverlay, uiTerminology, uiLocale).modeLayout
                });
              }
            },
            void 0,
            false,
            {
              fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
              lineNumber: 4721,
              columnNumber: 13
            },
            this
          )
        },
        void 0,
        false,
        {
          fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
          lineNumber: 4715,
          columnNumber: 13
        },
        this
      ) }, void 0, false, {
        fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
        lineNumber: 4714,
        columnNumber: 11
      }, this) }, void 0, false, {
        fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
        lineNumber: 4713,
        columnNumber: 9
      }, this)
    ] }, void 0, true, {
      fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
      lineNumber: 4682,
      columnNumber: 7
    }, this);
  }, [activeWindowId, effectiveModeLayout, error, handleActiveWindowChange, handleModeLayoutChange, handleTemplateDrop, loadedPlugins, mobile, modeWindows, navigateHistory, noteShellCommand, onAction, panel, pluginSupervisorById, primaryPluginId, reloadPlugin, session, shellRoute, studioMode, uiLocale, uiTerminology, updateSpacePanel, dispatch, uninstallPlugin]);
  const footerItems = useMemo(() => {
    const items = mobile ? [] : [
      { key: "bottomLeftPanelTabs", content: /* @__PURE__ */ jsxDEV(PanelChromeTabBar, { anchor: "bottom-left", ...buildPanelSelectionProps("bottom-left") }, void 0, false, {
        fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
        lineNumber: 4775,
        columnNumber: 44
      }, this) },
      { key: "bottomMiddlePanelTabs", centered: true, content: /* @__PURE__ */ jsxDEV(PanelChromeTabBar, { anchor: "bottom-middle", ...buildPanelSelectionProps("bottom-middle") }, void 0, false, {
        fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
        lineNumber: 4776,
        columnNumber: 62
      }, this) }
    ];
    if (brand?.id && ENTWERFEN_MIT_BESTAND_BRAND_IDS.includes(brand.id)) {
      items.push(
        { key: "footerProjectOfGap", className: "w-huge", content: null },
        aProjectOfLuhUdkFooterItem("aProjectOfLuhUdk", uiLocale, mobile),
        navbarFillItem("footerLeadingFill"),
        fundedByZukunftBauFooterItem("fundedByZukunftBau", uiLocale, mobile),
        { key: "footerFundedByGap", className: "w-huge", content: null }
      );
    } else {
      items.push(navbarFillItem("footerLeadingFill"));
    }
    if (!mobile) items.push({ key: "bottomRightPanelTabs", content: /* @__PURE__ */ jsxDEV(PanelChromeTabBar, { anchor: "bottom-right", ...buildPanelSelectionProps("bottom-right") }, void 0, false, {
      fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
      lineNumber: 4789,
      columnNumber: 69
    }, this) });
    return items;
  }, [brand?.id, buildPanelSelectionProps, mobile, uiLocale]);
  const buildPanelProps = useCallback(
    (anchor) => ({
      ...buildPanelSelectionProps(anchor),
      size: panels[anchor].size,
      onSizeChange: (value) => dispatch({ type: "SET_PANEL_SIZE", anchor, value }),
      tabBarHost: PANEL_TAB_BAR_HOSTS[anchor] ? "chrome" : "panel",
      treeOpenStates,
      onTreeOpenStateChange: (id, open) => dispatch({ type: "SET_TREE_OPEN_STATE", id, open })
    }),
    [buildPanelSelectionProps, panels, treeOpenStates]
  );
  useEffect(() => {
    const root = document.documentElement;
    const beaconId = pluginFilter ?? "unknown";
    const notFound = studioMode && shellRoute.kind === "notFound";
    if (notFound) {
      root.dataset.semioOsNotFound = beaconId;
      delete root.dataset.semioOsReady;
      delete root.dataset.semioOsError;
    } else if (error) {
      root.dataset.semioOsError = beaconId;
      delete root.dataset.semioOsReady;
      delete root.dataset.semioOsNotFound;
    } else if (session) {
      root.dataset.semioOsReady = beaconId;
      delete root.dataset.semioOsError;
      delete root.dataset.semioOsNotFound;
    }
    return () => {
      delete root.dataset.semioOsReady;
      delete root.dataset.semioOsError;
      delete root.dataset.semioOsNotFound;
    };
  }, [session, error, pluginFilter, shellRoute.kind, studioMode]);
  const dispatchShellMenuAction = useCallback(
    (action, args) => {
      if (!session) return;
      if (action === "shell.openActionPane") {
        const windowKind = session.app.windowKinds.find((kind) => kind.id === activeWindowId) ?? session.app.windowKinds[0];
        const actionId = typeof args?.actionId === "string" ? args.actionId : void 0;
        if (!windowKind || !actionId) return;
        dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: windowKind.id });
        dispatch({ type: "SET_ACTION_PANE_FOLDED", windowId: windowKind.id, value: false });
        dispatch({ type: "SET_ACTION_PANE_EXPANDED", windowId: windowKind.id, value: actionId });
        return;
      }
      if (action === "shell.openPalette") {
        dispatch({ type: "SET_SEARCH_OPEN", value: true });
        return;
      }
      onAction({ controllerId: session.app.controllerId, action });
    },
    [session, activeWindowId, onAction, dispatch]
  );
  const buildShellContextMenuItems = useCallback(() => {
    if (!session) return [];
    const windowKind = session.app.windowKinds.find((kind) => kind.id === activeWindowId) ?? session.app.windowKinds[0];
    const specs = [];
    const categoryByActionId = /* @__PURE__ */ new Map();
    if (windowKind) {
      for (const action of resolveWindowActions(session.app, windowKind)) {
        if (!action.inPalette) continue;
        const argCarrying = actionRequiresStagedForm(action);
        categoryByActionId.set(action.id, actionCategoryId(action));
        specs.push({
          id: `shell-menu.action.${action.id}`,
          label: resolveAppLabel(appLabelsOverlay, "action", action.id, resolveManifestLabel(action.label, uiTerminology, uiLocale)) + (argCarrying ? "…" : ""),
          icon: action.iconId,
          shortcut: action.keys ?? keysByActionId.get(action.id),
          destructive: action.kind === "operation" && action.id.toLowerCase().includes("delete"),
          action: argCarrying ? "shell.openActionPane" : action.id,
          args: argCarrying ? { actionId: action.id } : void 0
        });
      }
    }
    if (specs.length > 0) specs.push({ id: "shell-menu.separator", separator: true });
    specs.push({
      id: "shell.openPalette",
      label: shellLabel("ui.search.toggle"),
      icon: "search",
      action: "shell.openPalette"
    });
    const organized = organizeContextMenu(specs, (id) => categoryByActionId.get(id));
    return mapContextMenuSpecs(organized, dispatchShellMenuAction, keysByActionId);
  }, [session, activeWindowId, appLabelsOverlay, keysByActionId, dispatchShellMenuAction, uiTerminology, uiLocale]);
  useEffect(() => {
    const handleContextMenu = (event) => {
      if (isContextMenuPointerTarget(event.target)) return;
      const items = buildShellContextMenuItems();
      if (items.length === 0) return;
      event.preventDefault();
      setShellContextMenu({ x: event.clientX, y: event.clientY, items });
    };
    window.addEventListener("contextmenu", handleContextMenu);
    return () => window.removeEventListener("contextmenu", handleContextMenu);
  }, [buildShellContextMenuItems]);
  return /* @__PURE__ */ jsxDEV(SetWindowTitleContext.Provider, { value: setWindowTitle, children: /* @__PURE__ */ jsxDEV(SetWindowIconContext.Provider, { value: setWindowIcon, children: /* @__PURE__ */ jsxDEV(AppKeybindingsContext.Provider, { value: keysByActionId, children: /* @__PURE__ */ jsxDEV(UiKeybindingsProvider, { bindings: controlKeybindings, children: /* @__PURE__ */ jsxDEV(PluginSurfaceActionsContext.Provider, { value: requestContextMenu, children: /* @__PURE__ */ jsxDEV(ShellContextMenuFallbackContext.Provider, { value: buildShellContextMenuItems, children: /* @__PURE__ */ jsxDEV(ShellFaultBoundary, { boundaryId: "shell-root", fallbackLabel: shellLabel("ui.common.renderError"), children: /* @__PURE__ */ jsxDEV(UIFindProvider, { children: /* @__PURE__ */ jsxDEV(LevelProvider, { level: "base", children: [
    /* @__PURE__ */ jsxDEV("div", { className: "flex h-screen min-h-0 w-screen flex-col bg-transparent", "data-level": "base", children: /* @__PURE__ */ jsxDEV(PanelDockProvider, { dock, onTabDockDrop: handleTabDockDrop, onTreeUnitDockDrop: handleTreeUnitDockDrop, children: /* @__PURE__ */ jsxDEV(
      Layout,
      {
        mobile,
        mobilePanel,
        navbar: /* @__PURE__ */ jsxDEV(Navbar, { items: navbarItems, showFullscreenToggle: !mobile }, void 0, false, {
          fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
          lineNumber: 4928,
          columnNumber: 37
        }, this),
        subnavbar: activeTutorial ? /* @__PURE__ */ jsxDEV(
          TutorialBar,
          {
            title: resolveManifestLabel(activeTutorial.title, uiTerminology, uiLocale),
            durationMs: activeTutorial.durationMs,
            playing: tutorialPlaying,
            rate: tutorialRate,
            muted: tutorialMuted,
            captionsOn: tutorialCaptionsOn,
            recording: tutorialRecording,
            recordAvailable: tutorialRecorderAvailable,
            chapters: tutorialChapterMarkers,
            clock: tutorialClock,
            onPlayPause: playPauseTutorial,
            onStop: stopTutorial,
            onSeek: seekTutorial,
            onRateChange: (value) => dispatch({ type: "SET_TUTORIAL_RATE", value }),
            onMutedChange: (value) => dispatch({ type: "SET_TUTORIAL_MUTED", value }),
            onCaptionsChange: (value) => dispatch({ type: "SET_TUTORIAL_CAPTIONS", value }),
            onRecordToggle: toggleTutorialRecording,
            onAddChapter: addTutorialChapter
          },
          void 0,
          false,
          {
            fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
            lineNumber: 4931,
            columnNumber: 29
          },
          this
        ) : void 0,
        footer: /* @__PURE__ */ jsxDEV(Footer, { items: footerItems }, void 0, false, {
          fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
          lineNumber: 4953,
          columnNumber: 37
        }, this),
        panels: Object.fromEntries(ANCHORS.map((anchor) => [anchor, buildPanelProps(anchor)])),
        canvasStatus: shellPluginCanvasStatus,
        canvasSkeleton: /* @__PURE__ */ jsxDEV(CanvasSkeleton, { label: shellLabel("ui.common.loadingPlugins") }, void 0, false, {
          fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
          lineNumber: 4956,
          columnNumber: 45
        }, this),
        canvas: /* @__PURE__ */ jsxDEV(ShellFaultBoundary, { boundaryId: "route-canvas", fallbackLabel: shellLabel("ui.common.renderError"), children: canvas }, void 0, false, {
          fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
          lineNumber: 4958,
          columnNumber: 29
        }, this)
      },
      void 0,
      false,
      {
        fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
        lineNumber: 4925,
        columnNumber: 13
      },
      this
    ) }, void 0, false, {
      fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
      lineNumber: 4924,
      columnNumber: 11
    }, this) }, void 0, false, {
      fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
      lineNumber: 4923,
      columnNumber: 9
    }, this),
    /* @__PURE__ */ jsxDEV(UISearch, { items: searchItems, open: searchOpen, onOpenChange: (value) => dispatch({ type: "SET_SEARCH_OPEN", value }) }, void 0, false, {
      fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
      lineNumber: 4965,
      columnNumber: 9
    }, this),
    /* @__PURE__ */ jsxDEV(UIFind, { open: findOpen, onOpenChange: (value) => dispatch({ type: "SET_FIND_OPEN", value }) }, void 0, false, {
      fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
      lineNumber: 4966,
      columnNumber: 9
    }, this),
    /* @__PURE__ */ jsxDEV(TextSelectionContextMenuHost, {}, void 0, false, {
      fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
      lineNumber: 4967,
      columnNumber: 9
    }, this),
    /* @__PURE__ */ jsxDEV(
      ContextMenuController,
      {
        title: shellContextMenuTitleLabel,
        open: shellContextMenu != null,
        position: shellContextMenu,
        items: shellContextMenu?.items ?? [],
        onOpenChange: (open) => {
          if (!open) setShellContextMenu(null);
        }
      },
      void 0,
      false,
      {
        fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
        lineNumber: 4968,
        columnNumber: 9
      },
      this
    ),
    session && activeIntroduction && introductionStepIndex != null && /* @__PURE__ */ jsxDEV(
      UIIntroduction,
      {
        introduction: brand?.introduction ?? resolveIntroductionDefinition(activeIntroduction, appLabelsOverlay, uiTerminology, uiLocale),
        stepIndex: introductionStepIndex,
        completedInteractionIndices: introductionCompletedInteractions,
        onStepIndexChange: (value) => dispatch({ type: "SET_INTRODUCTION_STEP", value }),
        onDismiss: dismissIntroduction
      },
      void 0,
      false,
      {
        fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
        lineNumber: 4978,
        columnNumber: 23
      },
      this
    ),
    activeTutorial && /* @__PURE__ */ jsxDEV(Fragment, { children: [
      /* @__PURE__ */ jsxDEV(TutorialCaptionsHost, { tutorial: activeTutorial, clock: tutorialClock, captionsOn: tutorialCaptionsOn, terminology: uiTerminology, locale: uiLocale }, void 0, false, {
        fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
        lineNumber: 4988,
        columnNumber: 13
      }, this),
      /* @__PURE__ */ jsxDEV(TutorialVideoOverlayHost, { tutorial: activeTutorial, clock: tutorialClock, muted: tutorialMuted, playing: tutorialPlaying, rate: tutorialRate }, void 0, false, {
        fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
        lineNumber: 4989,
        columnNumber: 13
      }, this),
      /* @__PURE__ */ jsxDEV(TutorialGhostPointerHost, { tutorial: activeTutorial, clock: tutorialClock }, void 0, false, {
        fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
        lineNumber: 4990,
        columnNumber: 13
      }, this)
    ] }, void 0, true, {
      fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
      lineNumber: 4987,
      columnNumber: 23
    }, this),
    session && overlayDialog && (() => {
      const dialog = session.app.dialogs?.find((entry) => entry.id === overlayDialog.dialogId);
      if (!dialog) return null;
      return /* @__PURE__ */ jsxDEV(
        UIDialog,
        {
          dialog: resolveDialogDefinition(dialog, appLabelsOverlay, uiTerminology, uiLocale),
          seedArgs: overlayDialog.seedArgs,
          renderField: (def, value, onChange) => renderStagedArgControl(def, value, onChange),
          onSubmit: (args) => {
            dispatch({ type: "SET_DIALOG", value: null });
            onAction({ controllerId: session.app.controllerId, action: dialog.submitAction, args });
          },
          onCancel: () => {
            dispatch({ type: "SET_DIALOG", value: null });
            if (dialog.cancelAction) onAction({ controllerId: session.app.controllerId, action: dialog.cancelAction });
          }
        },
        void 0,
        false,
        {
          fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
          lineNumber: 4999,
          columnNumber: 27
        },
        this
      );
    })()
  ] }, void 0, true, {
    fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
    lineNumber: 4922,
    columnNumber: 7
  }, this) }, void 0, false, {
    fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
    lineNumber: 4921,
    columnNumber: 5
  }, this) }, void 0, false, {
    fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
    lineNumber: 4920,
    columnNumber: 5
  }, this) }, void 0, false, {
    fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
    lineNumber: 4919,
    columnNumber: 5
  }, this) }, void 0, false, {
    fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
    lineNumber: 4918,
    columnNumber: 5
  }, this) }, void 0, false, {
    fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
    lineNumber: 4917,
    columnNumber: 5
  }, this) }, void 0, false, {
    fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
    lineNumber: 4916,
    columnNumber: 5
  }, this) }, void 0, false, {
    fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
    lineNumber: 4915,
    columnNumber: 5
  }, this) }, void 0, false, {
    fileName: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
    lineNumber: 4914,
    columnNumber: 5
  }, this);
}
_s7(FrameworkOsShellInner, "S8TiyDI67ZijEB3pm0Rf9JpzOYE=", false, function() {
  return [useShellScope, useLabel, useMediaQuery, useUIHistory, usePanelChromeHotkeys, useElementsSurfaceChrome, useActionHotkey, useActionHotkey, useActionHotkey, useActionHotkey, useActionHotkey, useNamedLayoutHost, useShellKeydown];
});
_c6 = FrameworkOsShellInner;
var _c, _c2, _c3, _c4, _c5, _c6;
$RefreshReg$(_c, "AppKeybindingsContext");
$RefreshReg$(_c2, "TutorialCaptionsHost");
$RefreshReg$(_c3, "TutorialVideoOverlayHost");
$RefreshReg$(_c4, "TutorialGhostPointerHost");
$RefreshReg$(_c5, "FrameworkOsShell");
$RefreshReg$(_c6, "FrameworkOsShellInner");
import * as RefreshRuntime from "/@react-refresh";
const inWebWorker = typeof WorkerGlobalScope !== "undefined" && self instanceof WorkerGlobalScope;
if (import.meta.hot && !inWebWorker) {
  if (!window.$RefreshReg$) {
    throw new Error(
      "@vitejs/plugin-react can't detect preamble. Something is wrong."
    );
  }
  RefreshRuntime.__hmr_import(import.meta.url).then((currentExports) => {
    RefreshRuntime.registerExportsForReactRefresh("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx", currentExports);
    import.meta.hot.accept((nextExports) => {
      if (!nextExports) return;
      const invalidateMessage = RefreshRuntime.validateRefreshBoundaryAndEnqueueUpdate("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx", currentExports, nextExports);
      if (invalidateMessage) import.meta.hot.invalidate(invalidateMessage);
    });
  });
}
function $RefreshReg$(type, id) {
  return RefreshRuntime.register(type, "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx " + id);
}
function $RefreshSig$() {
  return RefreshRuntime.createSignatureFunctionForTransform();
}

//# sourceMappingURL=data:application/json;base64,eyJ2ZXJzaW9uIjozLCJtYXBwaW5ncyI6IkFBeWFTLFNBaTlJQyxVQWo5SUQ7O0FBaGFUO0FBQUEsRUFDRUE7QUFBQUEsRUFNQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsT0FDSztBQUNQO0FBQUEsRUFHRUM7QUFBQUEsRUFFQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFFQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFJQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFNQUM7QUFBQUEsRUFFQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFFQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFFQUM7QUFBQUEsRUFjQUM7QUFBQUEsT0FJSztBQUNQO0FBQUEsRUFHRUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFFQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsT0FFSztBQUNQO0FBQUEsRUFDRUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsT0FDSztBQUNQO0FBQUEsRUFFRUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFFQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFHQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFFQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFHQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFFQUM7QUFBQUEsRUFFQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFLQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFFQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFHQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFJQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFFQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFFQUM7QUFBQUEsRUFFQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFJQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFHQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsT0FDSztBQUNQO0FBQUEsRUFDRUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsT0FDSztBQUNQO0FBQUEsRUFDRUM7QUFBQUEsRUFFQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFHQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFFQUM7QUFBQUEsRUFFQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsT0FLSztBQUNQO0FBQUEsRUFDRUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsT0FDSztBQUNQO0FBQUEsRUFDRUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFD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QTtBQUFBLGFBQUFDLEtBQUE7QUFBQSxhQUFBQyxLQUFBIiwibmFtZXMiOlsiY3JlYXRlQ29udGV4dCIsInVzZUNhbGxiYWNrIiwidXNlQ29udGV4dCIsInVzZUVmZmVjdCIsInVzZU1lbW8iLCJ1c2VSZWR1Y2VyIiwidXNlUmVmIiwidXNlU3RhdGUiLCJidWlsZENvbnRyaWJ1dGlvbnNKc29uIiwiY3JlYXRlQnJvd3NlclN0b3JhZ2VQb3J0IiwiY3JlYXRlRGV2UGx1Z2luU291cmNlIiwiY3JlYXRlTWVtb3J5U3RvcmFnZVBvcnQiLCJjcmVhdGVTY29wZWRTdG9yYWdlUG9ydCIsIkRvY2tMYXlvdXRTdG9yZSIsIkRvY2tVaVN0YXRlU3RvcmUiLCJldmljdFBsdWdpbk1vZHVsZSIsImV4cGFuZFBsdWdpblJlZ2lzdHJ5IiwiRlJBTUVXT1JLX1BBTkVMX1RBQl9DQVRBTE9HVUVfSUQiLCJGUkFNRVdPUktfUEFORUxfVEFCX0RPQ1VNRU5UX0lDT05fSUQiLCJGUkFNRVdPUktfUEFORUxfVEFCX0RPQ1VNRU5UX0lEIiwiRlJBTUVXT1JLX1BBTkVMX1RBQl9ISVNUT1JZX0lEIiwiTmFtZWRMYXlvdXRTdG9yZSIsIm5vcm1hbGl6ZUFwcExhYmVsc092ZXJsYXkiLCJvcmdhbml6ZUNvbnRleHRNZW51IiwicGFuZWxUYWJLaW5kSWQiLCJwZW5kaW5nUGFuZWxVaU5vZGUiLCJwZW5kaW5nV2luZG93VWlOb2RlIiwicG9zdFBsdWdpbkJhY2tib25lSW5ib3VuZCIsIlJFQ09SRF9UVVRPUklBTF9BQ1RJT05fSUQiLCJyZWdpc3RlclBsdWdpbkJhY2tib25lUm91dGUiLCJyZXNvbHZlRXh0ZXJuYWxTbG90cyIsInJlc29sdmVMYXlvdXRGb3JNb2RlIiwicmVzb2x2ZU1vZGVUb29scyIsInJlc29sdmVQbGF5Z3JvdW5kRGVmYXVsdEFwcElkIiwicmVzb2x2ZVBsdWdpbkhvc3RDb25maWciLCJyZXNvbHZlUGx1Z2luUmVnaXN0cnlJZCIsInJlc29sdmVVaURpcnR5U2NvcGUiLCJyZXNvbHZlV2luZG93QWN0aW9ucyIsIlNFVF9BQ1RJVkVfVE9PTF9BQ1RJT05fSUQiLCJTRVRfQUNUSVZFX1VUSUxJVFlfQUNUSU9OX0lEIiwiU1RBUlRfSU5UUk9EVUNUSU9OX0FDVElPTl9JRCIsIlNUQVJUX1RVVE9SSUFMX0FDVElPTl9JRCIsIlRVVE9SSUFMX0NPTlZFUkdFX01TIiwid2luZG93RWxlbWVudElkIiwiYnVpbGRGaWxlQmFja2JvbmVVcmkiLCJidWlsZEZvbGRlckJhY2tib25lVXJpIiwiYnVpbGRGcmFtZXdvcmtTeW5jVXRpbGl0aWVzIiwiYnVpbGRSZW1vdGVCYWNrYm9uZVVyaSIsImRlY29kZUJhY2tib25lTWVzc2FnZSIsImRlY29kZUJhY2tib25lV29ya2VyUmVzcG9uc2UiLCJkZWNvZGVQYWNrVmFsdWUiLCJlbmNvZGVBY3Rpb25XaXJlIiwiZW5jb2RlQmFja2JvbmVNZXNzYWdlIiwiZW5jb2RlQmFja2JvbmVXb3JrZXJSZXF1ZXN0IiwiZW5jb2RlT3BlcmF0aW9uRW52ZWxvcGVzUGFjayIsIkZSQU1FV09SS19TWU5DX0NPTlRST0xMRVJfSUQiLCJvcGVyYXRpb25FbnZlbG9wZUZyb21XaXJlIiwib3BlcmF0aW9uRW52ZWxvcGVUb1dpcmUiLCJkZWNvZGVXb3JsZFByb2plY3Rpb25UZW1wbGF0ZUlkIiwid29ybGRQcm9qZWN0aW9uU3BlY0ljb25JZCIsIndvcmxkUHJvamVjdGlvblNwZWNMYWJlbCIsIkFOQ0hPUlMiLCJBcHAiLCJhcHBseURvY2tTa2VsZXRvbiIsImFwcGx5VWlUaGVtZVRvUm9vdCIsImJvcmRlck5vcm1hbEJvdHRvbUNsYXNzIiwiYnVpbGRLZXlzQnlBY3Rpb25JZCIsImJ1aWx0aW5VaURyaXZlcnMiLCJidWlsdGluVWlUaGVtZXMiLCJCdXR0b25Hcm91cCIsIkJ1dHRvbkdyb3VwSXRlbSIsIkNhbnZhc1NrZWxldG9uIiwiQ0VMRUJSQVRFX1NUQU1QX0RVUkFUSU9OX01TIiwiY2VsZWJyYXRlQWxsRWxlbWVudHMiLCJjZWxlYnJhdGVFbGVtZW50cyIsImNoaWxkRWxlbWVudElkIiwiQ2hyb21lQXdhcmVXaW5kb3dTY3JvbGxTdXJmYWNlIiwiY2xlYXJVaVRoZW1lRnJvbVJvb3QiLCJjbiIsImNvbXBvc2VDb250cm9sS2V5YmluZGluZ3MiLCJjb21wb3NlVHV0b3JpYWxVaSIsIkNvbnRleHRNZW51Q29udHJvbGxlciIsImNyZWF0ZVNoZWxsU2NvcGUiLCJjcmVhdGVUdXRvcmlhbENsb2NrIiwiREVGQVVMVF9VSV9EUklWRVIiLCJkZXRlY3RTaGVsbExvY2FsZSIsImRpc3Bvc2VTaGVsbEkxOG5JbnN0YW5jZSIsImRvY2tTa2VsZXRvbk9mIiwiZG9ja1NrZWxldG9uc0VxdWFsIiwiZWxlbWVudElkU2VsZWN0b3IiLCJmaW5kUGFuZWxUYWJJbkRvY2siLCJmaW5kUGFuZWxUYWJOb2RlIiwiZmluZFBhbmVsVGFiUGF0aCIsIkZvb3RlciIsImdldFR1dG9yaWFsQ2FtZXJhRHJpdmVyIiwiSWNvbiIsImljb25SZW5kZXJQb3J0IiwiaW5zZXJ0V2luZG93QXREcm9wWm9uZSIsImludGVyYWN0aXZlQWN0aXZlRmlsbENsYXNzIiwiaW50ZXJwb2xhdGVUdXRvcmlhbENhbWVyYSIsImlzQ29udGV4dE1lbnVQb2ludGVyVGFyZ2V0IiwiTGF5b3V0IiwiTGV2ZWxQcm92aWRlciIsImxvYWRpbmdCb3JkZXJDbGFzcyIsIk1vZGUiLCJtb3ZlVGFiSW5Eb2NrIiwibW92ZVRyZWVVbml0SW5Eb2NrIiwiTmF2YmFyIiwiTmF2YmFyRXhhbXBsZVNlbGVjdCIsIm5hdmJhckZpbGxJdGVtIiwiUGFuZWxDaHJvbWVUYWJCYXIiLCJQYW5lbERvY2tQcm92aWRlciIsInBhbmVsVGFiQ2hpbGRyZW4iLCJwYXJzZVVpVGhlbWUiLCJyZWFkU3RvcmVkSW50cm9kdWN0aW9uU2VlbiIsInJlYWRTdG9yZWRVaUNocm9tZUxvY2FsZSIsInJlYWRTdG9yZWRVaUNocm9tZVRoZW1lU25hcHNob3QiLCJyZWNvbmNpbGVBY3RpdmVQYXRoIiwicmVzb2x2ZVVpRHJpdmVyIiwiU2VtaW9Mb2dvIiwic2VtaW9UaGVtZSIsInNlcmlhbGl6ZVVpVGhlbWUiLCJzZXRBY3RpdmVVaVRoZW1lIiwiU2hlbGxCcmFuZExvZ28iLCJzaGVsbENocm9tZVRpdGxlQ2xhc3NOYW1lIiwiU2hlbGxTY29wZVByb3ZpZGVyIiwic2luZ2xlVHJlZUxlYWYiLCJzdGF0aWNUcmVlUGFuZWxEZWZpbml0aW9uIiwiVGV4dFNlbGVjdGlvbkNvbnRleHRNZW51SG9zdCIsIlRvZ2dsZSIsIlR1dG9yaWFsQmFyIiwidHV0b3JpYWxDYW1lcmFBdCIsIlR1dG9yaWFsQ2FwdGlvbnMiLCJ0dXRvcmlhbEN1ZXNCZXR3ZWVuIiwiVHV0b3JpYWxHaG9zdFBvaW50ZXIiLCJ0dXRvcmlhbFNsaWNlIiwiVHV0b3JpYWxWaWRlb092ZXJsYXkiLCJVSV9NT0JJTEVfTUVESUFfUVVFUlkiLCJVSV9URVJNSU5PTE9HWV9OQVRJVkUiLCJVSURpYWxvZyIsIlVJSW50cm9kdWN0aW9uIiwiVWlLZXliaW5kaW5nc1Byb3ZpZGVyIiwidXNlQWN0aW9uSG90a2V5IiwidXNlRWxlbWVudHNTdXJmYWNlQ2hyb21lIiwidXNlTGFiZWwiLCJ1c2VNZWRpYVF1ZXJ5IiwidXNlUGFuZWxDaHJvbWVIb3RrZXlzIiwidXNlU2hlbGxLZXlkb3duIiwidXNlU2hlbGxTY29wZSIsInVzZVR1dG9yaWFsQ2xvY2siLCJ2YWxpZGF0ZVR1dG9yaWFsIiwiV2luZG93Qm9keVNrZWxldG9uIiwid3JpdGVTdG9yZWRJbnRyb2R1Y3Rpb25TZWVuIiwid3JpdGVTdG9yZWRVaUNocm9tZUFwcGVhcmFuY2UiLCJ3cml0ZVN0b3JlZFVpQ2hyb21lTGF5b3V0Iiwid3JpdGVTdG9yZWRVaUNocm9tZUxvY2FsZSIsIndyaXRlU3RvcmVkVWlDaHJvbWVUZXJtaW5vbG9neSIsIndyaXRlU3RvcmVkVWlDaHJvbWVUaGVtZUlkIiwid3JpdGVTdG9yZWRVaUNocm9tZVRoZW1lU25hcHNob3QiLCJ3cml0ZVN0b3JlZFVpQ3VzdG9tRHJpdmVycyIsIndyaXRlU3RvcmVkVWlDdXN0b21UaGVtZXMiLCJ3cml0ZVN0b3JlZFVpRHJpdmVySWQiLCJ3cml0ZVN0b3JlZFVpS2V5YmluZGluZ092ZXJyaWRlcyIsImRlY2xhcmF0aXZlU3VyZmFjZVN0YXR1cyIsIkludGVycHJldGVkVWlOb2RlIiwiUGx1Z2luU3VyZmFjZUFjdGlvbnNDb250ZXh0IiwiU2hlbGxDb250ZXh0TWVudUZhbGxiYWNrQ29udGV4dCIsIndpcmVMYWJlbCIsImFjdGlvblN0YWdlS2V5IiwiRU1QVFlfU0hFTExfREVGQVVMVFMiLCJFTVBUWV9TSEVMTF9MT0NLUyIsImluaXRpYWxTaGVsbFN0YXRlIiwiaXNFcGhlbWVyYWxTaGVsbEJyYW5kIiwicmVzb2x2ZUJvb3RFeGFtcGxlSWQiLCJTaGVsbEZhdWx0Qm91bmRhcnkiLCJzaGVsbFJlZHVjZXIiLCJzaG91bGRQZXJzaXN0SW50cm9kdWN0aW9uU2VlbiIsInNob3VsZFJlcGxheUludHJvZHVjdGlvbk9uTG9hZCIsImJlZ2luSW50ZXJhY3RpdmVQbHVnaW5BY3Rpb24iLCJjbGVhclBlbmRpbmdXb3JsZFByb2plY3Rpb24iLCJlbmRJbnRlcmFjdGl2ZVBsdWdpbkFjdGlvbiIsIm1hcENvbnRleHRNZW51U3BlY3MiLCJyZWdpc3RlclBlbmRpbmdXb3JsZFByb2plY3Rpb24iLCJXaW5kb3dJbnN0YW5jZUlkQ29udGV4dCIsIkRFRkFVTFRfUEFORUxfV0lEVEhfUFgiLCJFTVBUWV9BUFBfTEFCRUxTX09WRVJMQVkiLCJGUkFNRVdPUktfQ0FURUdPUllfQ09NTUFORF9JRCIsIkZSQU1FV09SS19DQVRFR09SWV9ESVNQTEFZX0lEIiwiRlJBTUVXT1JLX0NBVEVHT1JZX1RPT0xfSUQiLCJGUkFNRVdPUktfUkVTRVJWRURfQUNUSU9OX0lEUyIsIkxBWU9VVF9DSEFOR0VfU0VUVExFX01TIiwiTk9URV9XT1JMRF9OQVZJR0FUSU9OX0FDVElPTl9JRCIsIlBBTkVMX1RBQl9CQVJfSE9TVFMiLCJQUkVTRU5DRV9IRUFSVEJFQVRfSU5URVJWQUxfTVMiLCJUVVRPUklBTF9SRUNPUkRJTkdfRVhDTFVERURfQUNUSU9OX0lEUyIsImFjdGlvbkNhdGVnb3J5SWQiLCJhY3Rpb25SZXF1aXJlc1N0YWdlZEZvcm0iLCJhcHBEb2N1bWVudExhYmVsIiwiYXBwV2luZG93RG9jdW1lbnRMYWJlbCIsImFwcGx5RnJhbWV3b3JrTGF5b3V0U2VlZCIsImFwcGx5VHV0b3JpYWxVaUNoYW5nZVRvU2hlbGwiLCJhcHBseVR1dG9yaWFsVWlTbmFwc2hvdFRvU2hlbGwiLCJhcHBseVVpUmVmcmVzaFJlc3BvbnNlVG9DYWNoZSIsImJ1aWxkQWN0aXZlVXRpbGl0eUJ5V2luZG93SWQiLCJidWlsZENvbW1hbmRDYXRlZ29yeVRhYnMiLCJidWlsZE5vdGVTaGVsbENvbW1hbmRBY3Rpb24iLCJidWlsZE9zQ29tbWFuZHMiLCJidWlsZFNwYWNlUGFuZWxTdGF0ZSIsImJ1aWxkVG9vbFRhYnMiLCJidWlsZFVpUmVmcmVzaFJlcXVlc3QiLCJjYXB0dXJlQ3VycmVudEZyYW1ld29ya0xheW91dCIsImNhcHR1cmVUdXRvcmlhbFVpU25hcHNob3QiLCJjYXRlZ29yeVRhYkljb24iLCJjbGFzc2lmeVdpbmRvd0xheW91dENoYW5nZSIsImNvbW1hbmRDYXRlZ29yaWVzIiwiY29tbWFuZENhdGVnb3J5TGFiZWwiLCJkaXNwYXRjaE9wZW5lZEZpbGVzIiwiZGlzcGF0Y2hPc0NvbW1hbmQiLCJkb3dubG9hZERhdGFVcmwiLCJkb3dubG9hZE1lZGlhRXhwb3J0IiwiZmxhdHRlblBhbmVsVGFiTGVhdmVzIiwiaW50cm9kdWN0aW9uVGFyZ2V0c1dpbmRvdyIsImxvYWRQbHVnaW5Nb2R1bGVSZXNpbGllbnQiLCJtYWtlRWZmZWN0RGlzcGF0Y2hPbmUiLCJtZXJnZVJlY29yZFByZXNlcnZpbmdJZGVudGl0eSIsInBhbmVsQW5jaG9yRm9yR3JvdXAiLCJwYW5lbEpzb25Gcm9tU3RhdGUiLCJwYW5lbFRhYkRlZmluaXRpb25Ub05vZGUiLCJwYXJzZVBhbmVsU3RhdGUiLCJwYXJzZVNoZWxsUm91dGUiLCJwYXRjaERvY3VtZW50VHJlZVNlbGVjdGVkSWRzIiwicGF0Y2hXb3JsZDNkQ2hyb21lT250b05vZGUiLCJwcmVzZW5jZUNsaWVudElkZW50aXR5IiwicHJlc2VydmVKc29uSWRlbnRpdHkiLCJyZW5kZXJTdGFnZWRBcmdDb250cm9sIiwicmVxdWVzdEZpbGVPcGVuIiwicmVzb2x2ZUFwcERvY3VtZW50IiwicmVzb2x2ZUFwcExhYmVsIiwicmVzb2x2ZUNhbnZhc0JvZHlLZXkiLCJyZXNvbHZlQ29tbWFuZHMiLCJyZXNvbHZlRGlhbG9nRGVmaW5pdGlvbiIsInJlc29sdmVEb2N1bWVudEJ5QXBwSWQiLCJyZXNvbHZlRnJhbWV3b3JrTGF5b3V0U2VlZCIsInJlc29sdmVJbnRyb2R1Y3Rpb25EZWZpbml0aW9uIiwicmVzb2x2ZUtleWJpbmRpbmdJbnRlbnQiLCJyZXNvbHZlTWFuaWZlc3RMYWJlbCIsInJlc29sdmVQYW5lbFRhYkxhYmVsIiwicmVzb2x2ZVV0aWxpdHlBY3RpdmF0aW9uIiwicmVzb2x2ZVV0aWxpdHlOb2RlcyIsInJlc29sdmVXaW5kb3dFbmdhZ2VtZW50IiwicmV0aXRsZVdpbmRvd0xheW91dE5vZGUiLCJydW5SZXF1ZXN0TWVkaWFGcmFtZXMiLCJzY2hlZHVsZURpc3BhdGNoQWN0aW9uIiwic2Vzc2lvbldpbmRvd0luc3RhbmNlcyIsInNoZWxsTGFiZWwiLCJzaGVsbFRhYkljb24iLCJzcGF3bmVkV2luZG93Q2hyb21lRm9yS2luZCIsInN0dWRpb1BhbmVsRm9jdXNpbmdTcGF3bmVkIiwic3luY0RvY3VtZW50SWQiLCJzeW50aGVzaXplTG9jYWxpemVkTGFiZWwiLCJ0b29sSWRGcm9tUGFuZWxUYWJJZCIsInVzZVVJSGlzdG9yeSIsInV0aWxpdHlCYXJOb2RlIiwidXRpbGl0eU5vZGVUcmVlQ29udGFpbnNJZCIsInZpZXdTdGF0ZVdpdGhTcGFjZVBhbmVsIiwid2luZG93QWN0aW9uUGFuZU5vZGUiLCJ3aW5kb3dFbmdhZ2VtZW50VG9TZWFyY2hTcGVjIiwid2luZG93RW5nYWdlbWVudFRvU3BlYyIsIndpbmRvd01lYXN1cmVUcmVlQ29udGFpbnNJZCIsIndpbmRvd01lYXN1cmVzQ2hyb21lIiwiYVByb2plY3RPZkx1aFVka0Zvb3Rlckl0ZW0iLCJmdW5kZWRCeVp1a3VuZnRCYXVGb290ZXJJdGVtIiwiRU5UV0VSRkVOX01JVF9CRVNUQU5EX0JSQU5EX0lEUyIsImNyZWF0ZUZyYW1ld29ya0Rpc3BsYXlQYW5lbFRhYnMiLCJjcmVhdGVGcmFtZXdvcmtQbHVnaW5zUGFuZWxUYWJzIiwiY3JlYXRlRnJhbWV3b3JrU2V0dGluZ3NQYW5lbFRhYnMiLCJQbHVnaW5SZWNvdmVyeVBhbmVsIiwiU2hlbGxSb3V0ZU5vdEZvdW5kUGFnZSIsInVzZU5hbWVkTGF5b3V0SG9zdCIsIlN5bmNBdHRhY2hDYXJkIiwiVUlGaW5kIiwiVUlGaW5kUHJvdmlkZXIiLCJVSVNlYXJjaCIsIlVUSUxJVFlfQ0FURUdPUllfSUNPTl9JRCIsImNvZXJjZVdpcmVCeXRlcyIsIlNldFdpbmRvd1RpdGxlQ29udGV4dCIsIlNldFdpbmRvd0ljb25Db250ZXh0IiwiRU1QVFlfS0VZU19CWV9BQ1RJT05fSUQiLCJNYXAiLCJBcHBLZXliaW5kaW5nc0NvbnRleHQiLCJfYyIsInVzZUFwcEtleWJpbmRpbmdzQnlBY3Rpb25JZCIsIl9zIiwidXNlTWFwQ29udGV4dE1lbnVTcGVjcyIsImRpc3BhdGNoIiwiX3MyIiwia2V5c0J5QWN0aW9uSWQiLCJzcGVjcyIsInR1dG9yaWFsQXNzZXRTcmNUb1VybCIsInNyYyIsImtpbmQiLCJ1cmwiLCJkYXRhIiwiY29uc29sZSIsIndhcm4iLCJoYXNoIiwiVHV0b3JpYWxDYXB0aW9uc0hvc3QiLCJ0dXRvcmlhbCIsImNsb2NrIiwiY2FwdGlvbnNPbiIsInRlcm1pbm9sb2d5IiwibG9jYWxlIiwiX3MzIiwidGltZU1zIiwiY3VlIiwidHJhY2tzIiwibmFycmF0aW9uIiwidGV4dCIsIlRVVE9SSUFMX0RFRkFVTFRfVklERU9fUkVDVCIsIngiLCJ5Iiwid2lkdGgiLCJoZWlnaHQiLCJUdXRvcmlhbFZpZGVvT3ZlcmxheUhvc3QiLCJtdXRlZCIsInBsYXlpbmciLCJyYXRlIiwiX3M0IiwidmlkZW8iLCJsb2NhbFRpbWVNcyIsImF0Iiwic291cmNlT2Zmc2V0TXMiLCJyZWN0IiwiVHV0b3JpYWxHaG9zdFBvaW50ZXJIb3N0IiwiX3M1IiwiZ2VzdHVyZXMiLCJwcm9ncmVzcyIsIk1hdGgiLCJtaW4iLCJtYXgiLCJkdXJhdGlvbk1zIiwiZGlmZlR1dG9yaWFsVWlTbmFwc2hvdCIsInByZXYiLCJuZXh0IiwiY2hhbmdlcyIsImFjdGl2ZU1vZGVJZCIsInB1c2giLCJpZCIsImZvY3VzZWRXaW5kb3dJZCIsInV0aWxpdHlXaW5kb3dJZHMiLCJTZXQiLCJPYmplY3QiLCJrZXlzIiwiYWN0aXZlVXRpbGl0eUJ5V2luZG93SWQiLCJ3aW5kb3dJZCIsInV0aWxpdHlJZCIsImFjdGl2ZVRvb2xJZCIsImxheW91dCIsIkpTT04iLCJzdHJpbmdpZnkiLCJncm91cHMiLCJhY3RpdmVQYW5lbFRhYkJ5R3JvdXAiLCJncm91cCIsInRhYklkIiwicGFuZWxKc29uIiwic2VsZWN0aW9uSnNvbiIsIm9wZW5EaWFsb2dJZCIsInByZXZUcmVlIiwiZXhwYW5kZWRUcmVlSWRzIiwibmV4dFRyZWUiLCJoYXMiLCJleHBhbmRlZCIsImNvbW1hbmRQYW5lbE9wZW4iLCJvcGVuIiwidHV0b3JpYWxDYW1lcmFQb3NlRXF1YWxzIiwiYSIsImIiLCJwb3NpdGlvbiIsImV2ZXJ5IiwidmFsdWUiLCJpbmRleCIsImFicyIsInRhcmdldCIsInpvb20iLCJUdXRvcmlhbFJlY29yZGVyIiwic3RhcnRlZEF0TXMiLCJiYXNlVWlTbmFwc2hvdCIsImJhc2VEb2N1bWVudEpzb24iLCJldmVudHMiLCJ1aUtleWZyYW1lcyIsImNhbWVyYUtleWZyYW1lcyIsImNoYXB0ZXJzIiwibGFzdFVpU25hcHNob3QiLCJsYXN0Q2FtZXJhQnlXaW5kb3ciLCJjb25zdHJ1Y3RvciIsInBlcmZvcm1hbmNlIiwibm93Iiwibm93TXMiLCJyb3VuZCIsInJlY29yZEV2ZW50IiwicmVjb3JkVWlEaWZmIiwibGVuZ3RoIiwic2FtcGxlIiwicmVjb3JkU25hcHNob3QiLCJzdGF0ZSIsInNhbXBsZUNhbWVyYSIsImNhbWVyYSIsImdldCIsInNldCIsImVhc2luZyIsImFkZENoYXB0ZXIiLCJ0aXRsZSIsInJhd1RpdGxlIiwiYnVpbGQiLCJleGFtcGxlSWQiLCJiYXNlIiwiZG9jdW1lbnRKc29uIiwidW5kZWZpbmVkIiwidWkiLCJjYW1lcmFzIiwiZG9jdW1lbnQiLCJyZWNvcmRlZEF0IiwiRGF0ZSIsInRvSVNPU3RyaW5nIiwicmVzb2x2ZVNoZWxsU2NvcGVTdG9yYWdlIiwiZXBoZW1lcmFsIiwic3RvcmFnZU5hbWVzcGFjZSIsImJyb3dzZXIiLCJGcmFtZXdvcmtPc1NoZWxsIiwicHJvcHMiLCJfczYiLCJzaGVsbElkIiwib3duc1BhZ2UiLCJicmFuZCIsImxvY2tzIiwiaW5uZXJQcm9wcyIsInNjb3BlIiwic3RvcmFnZSIsImluaXRpYWxMb2NhbGUiLCJuYXZpZ2F0b3IiLCJsYW5ndWFnZSIsImJ1bXBBZnRlclJvb3RBdHRhY2giLCJzZXRSb290Iiwibm9kZSIsInJvb3RSZWYiLCJjdXJyZW50IiwibiIsInNldFBvcnRhbExheWVyIiwicG9ydGFsTGF5ZXJSZWYiLCJpMThuIiwiaXNvbGF0aW9uIiwiRnJhbWV3b3JrT3NTaGVsbElubmVyIiwicGx1Z2luRmlsdGVyIiwicGx1Z2lucyIsImFwcElkIiwibG9ja3NQcm9wIiwiZGVmYXVsdHMiLCJkZWZhdWx0c1Byb3AiLCJzdXBwcmVzc0F1dG9JbnRyb2R1Y3Rpb24iLCJfczciLCJzaGVsbENvbnRleHRNZW51VGl0bGVMYWJlbCIsImhvc3RDb25maWciLCJzdHVkaW9Nb2RlIiwibW9iaWxlIiwic2hlbGxTdGF0ZSIsImxvYWRlZFBsdWdpbnMiLCJwbHVnaW5TdGF0dXNCeUlkIiwicGx1Z2luU3VwZXJ2aXNvckJ5SWQiLCJzZXNzaW9uIiwiZXJyb3IiLCJwbHVnaW5SdW50aW1lIiwiaG9zdFBsdWdpbiIsImZpbmQiLCJlbnRyeSIsImhhbmRsZSIsInBsdWdpbklkIiwiaG9zdEFwcCIsIm1hbmlmZXN0IiwiYXBwcyIsImFwcCIsImhvc3RBcHBJZCIsImxhbmRpbmdBcHAiLCJsYW5kaW5nQXBwSWQiLCJob3N0Q29udHJvbGxlcklkIiwiY29udHJvbGxlcklkIiwibGFuZGluZ0NvbnRyb2xsZXJJZCIsImhvc3RDYXRhbG9ndWVUYWJJZCIsInBhbmVsVGFicyIsIndpbmRvd1VpQnlXaW5kb3dJZCIsIndpbmRvd0VuZ2FnZW1lbnRzQnlXaW5kb3dJZCIsIndpbmRvd01lYXN1cmVzQnlXaW5kb3dJZCIsInRvb2xNZWFzdXJlc0J5VG9vbElkIiwicGFuZWxVaUJ5S2V5IiwiYXBwTGFiZWxzT3ZlcmxheSIsIndpbmRvd1VpIiwic3Bhd25lZFdpbmRvd1VpIiwic3Bhd25lZFdpbmRvd0VuZ2FnZW1lbnRzIiwic3Bhd25lZFdpbmRvd01lYXN1cmVzIiwic3Bhd25lZFdpbmRvdyIsImZvbGRlZEJ5V2luZG93SWQiLCJhY3Rpb25QYW5lRm9sZGVkQnlXaW5kb3dJZCIsImV4cGFuZGVkQnlXaW5kb3dJZCIsImFjdGlvblBhbmVFeHBhbmRlZEJ5V2luZG93SWQiLCJzdGFnZWRBcmdzQnlLZXkiLCJhY3Rpb25QYW5lU3RhZ2VkQXJnc0J5S2V5IiwiYWN0aW9uUGFuZSIsImV4cGFuZGVkQ29tbWFuZElkIiwic3RhZ2VkQXJnc0J5Q29tbWFuZElkIiwiY29tbWFuZFN0YWdlZEFyZ3NCeUNvbW1hbmRJZCIsImNvbW1hbmRQYW5lbCIsInBhbmVscyIsImRvY2tPdmVycmlkZSIsInBhbmVsUGF0aE1lbW9yeSIsInRyZWVPcGVuU3RhdGVzIiwiYWN0aXZlV2luZG93SWQiLCJzaGVsbExheW91dCIsImFjdGl2ZUV4YW1wbGVJZCIsIm1vYmlsZVBhbmVsUGF0aCIsIm1vYmlsZVBhbmVsVmlzaWJsZSIsImV4dHJhV2luZG93SW5zdGFuY2VzIiwid2luZG93VGl0bGVzQnlJZCIsIndpbmRvd0ljb25zQnlJZCIsInNlYXJjaE9wZW4iLCJmaW5kT3BlbiIsImludHJvZHVjdGlvblN0ZXBJbmRleCIsImludHJvZHVjdGlvbkNvbXBsZXRlZEludGVyYWN0aW9ucyIsImRpYWxvZyIsIm92ZXJsYXlEaWFsb2ciLCJvdmVybGF5cyIsImFjdGl2ZVR1dG9yaWFsSWQiLCJ0dXRvcmlhbFBsYXlpbmciLCJ0dXRvcmlhbFJhdGUiLCJ0dXRvcmlhbE11dGVkIiwidHV0b3JpYWxDYXB0aW9uc09uIiwicmVjb3JkaW5nIiwidHV0b3JpYWxSZWNvcmRpbmciLCJkZXZpYXRlZCIsInR1dG9yaWFsRGV2aWF0ZWQiLCJ1aUFwcGVhcmFuY2UiLCJ1aUxheW91dCIsInVpRHJpdmVySWQiLCJ1aUN1c3RvbURyaXZlcnMiLCJ1aURyaXZlckRyYWZ0IiwidWlMb2NhbGUiLCJ1aVRlcm1pbm9sb2d5IiwidWlUaGVtZUlkIiwidWlDdXN0b21UaGVtZXMiLCJ1aVRoZW1lRHJhZnQiLCJ1aUtleWJpbmRpbmdPdmVycmlkZXMiLCJ1aVByZWZzIiwic3luY0JhY2tib25lVXJpIiwic3luY0NhcmRLaW5kIiwic3luY0RyYWZ0UGF0aCIsInN5bmNTdGF0dXNCeURvY3VtZW50SWQiLCJzeW5jIiwiaW1wb3J0U3BhY2VJbnB1dFJlZiIsInJlZnJlc2hHZW5lcmF0aW9uUmVmIiwiY29udHJpYnV0aW9uc0pzb25SZWYiLCJhcHBSZWdpc3RyYXRpb25zSnNvblJlZiIsInNwYXduZWRSZWZyZXNoR2VuZXJhdGlvblJlZiIsImNvbnRyaWJ1dG9ySW5zdGFuY2VzUmVmIiwibGF5b3V0U2VlZEtleVJlZiIsIm5vRXhhbXBsZVJlc2V0SW5zdGFuY2VJZFJlZiIsImV4dHJhV2luZG93Q291bnRlclJlZiIsInNoZWxsQ29udGV4dE1lbnUiLCJzZXRTaGVsbENvbnRleHRNZW51IiwiZXh0cmFXaW5kb3dJbnN0YW5jZXNSZWYiLCJzZXRXaW5kb3dUaXRsZSIsInR5cGUiLCJzZXRXaW5kb3dJY29uIiwiaWNvbklkIiwidWlSZWZyZXNoQ2FjaGVSZWYiLCJzcGF3bmVkVWlSZWZyZXNoQ2FjaGVSZWYiLCJzcGF3bmVkTGF5b3V0U2VlZFJlZiIsIm9wZW5TcGFjZUlkUmVmIiwib3Blbkluc3RhbmNlSWRSZWYiLCJzZXNzaW9uUmVmIiwidWlEZXZpY2UiLCJ1aVRoZW1lIiwiZm91bmQiLCJ0IiwidWlEcml2ZXIiLCJiYWNrYm9uZVdvcmtlclJlZiIsInNoZWxsQWN0b3JJZFJlZiIsInJhbmRvbSIsInRvU3RyaW5nIiwic2xpY2UiLCJvcGVuRG9jdW1lbnRTZXNzaW9uc1JlZiIsInBsdWdpbkJhY2tib25lUm91dGVVbnJlZ2lzdGVyc1JlZiIsImxvYWRlZFBsdWdpbnNSZWYiLCJwbHVnaW5Nb2R1bGVVcmxCeUlkUmVmIiwicGx1Z2luT3BJbkZsaWdodFJlZiIsImVuc3VyZUJhY2tib25lV29ya2VyIiwid29ya2VyIiwiV29ya2VyIiwiVVJMIiwiaW1wb3J0Iiwib25tZXNzYWdlIiwibWVzc2FnZUV2ZW50IiwibWVzc2FnZSIsIndpcmUiLCJkb2N1bWVudElkIiwiZXZlbnQiLCJzdGF0dXMiLCJwZXJzaXN0ZWQiLCJwZW5kaW5nT3BlcmF0aW9ucyIsInJlbW90ZSIsInBlZXJzSnNvbiIsInBlZXJzIiwibWFwIiwicGVlciIsImNsaWVudElkIiwiYWN0b3IiLCJuYW1lIiwibGFiZWwiLCJzZWxlY3Rpb25Db3VudCIsImluc3RhbmNlSWQiLCJ2aWV3U3RhdGUiLCJwcmVzZW5jZVBlZXJzSnNvbiIsInBsdWdpbiIsImFwcGx5T3BlcmF0aW9ucyIsImVudmVsb3BlcyIsImFjdG9yVXJpIiwiZW52ZWxvcGUiLCJwaHlzaWNhbF9tcyIsImxvZ2ljYWwiLCJsb2FkQXBwRG9jdW1lbnQiLCJwYWNrQnl0ZXMiLCJVaW50OEFycmF5IiwicGFjayIsIkFycmF5IiwiZnJvbSIsInNwciIsInVyaSIsInNoZWxsVXJpIiwiY2FuR29CYWNrIiwiY2FuR29Gb3J3YXJkIiwiY2FuR29VcCIsImdvQmFjayIsImdvRm9yd2FyZCIsImdvVXAiLCJuYXZpZ2F0ZSIsIm5hdmlnYXRlSGlzdG9yeSIsInNoZWxsUm91dGUiLCJzcGxpdCIsInNoZWxsU3RvcmFnZSIsIm5hbWVkTGF5b3V0U3RvcmUiLCJkb2NrTGF5b3V0U3RvcmUiLCJkb2NrVWlTdGF0ZVN0b3JlIiwicmVnaXN0cnkiLCJwcmltYXJ5UGx1Z2luSWQiLCJzaGVsbFBsdWdpbkNhbnZhc1N0YXR1cyIsInBsdWdpblN0YXR1cyIsInBsdWdpblNvdXJjZSIsImVzdGFibGlzaFByaW1hcnlTZXNzaW9uIiwic0FwcCIsIkVycm9yIiwicGFuZWxTdGF0ZSIsImNyZWF0ZUFwcCIsImRlZmF1bHRNb2RlSWQiLCJtb2RlcyIsInNlZWRlZCIsImRlZmF1bHRMYXlvdXQiLCJ3aW5kb3dLaW5kcyIsImV4dHJhSW5zdGFuY2VzIiwibW9kZUxheW91dCIsInByaW1hcnlBcHAiLCJkZWZhdWx0QXBwSWQiLCJpbnN0YWxsUGx1Z2luIiwicmVidWlsdEF0Iiwic29tZSIsImNhbmRpZGF0ZSIsImFkZCIsIm1vZHVsZVVybCIsImJvb3RFcnJvciIsIlN0cmluZyIsImRlbGV0ZSIsInJlbG9hZFBsdWdpbiIsIm9sZE1vZHVsZVVybCIsIm5ld0hhbmRsZSIsImFjdGl2ZVNlc3Npb24iLCJvd25zU2Vzc2lvbiIsIm9sZEFwcElkcyIsIm5ld0FwcElkcyIsImhvdFN3YXBFdmVudCIsInZlcnNpb24iLCJhZGRlZEFwcHMiLCJmaWx0ZXIiLCJyZW1vdmVkQXBwcyIsImxvZyIsImRlc3Ryb3lBcHAiLCJjYXRjaCIsInNwYXduZWQiLCJzcGF3bmVkQXBwc1JlZiIsImNvbnRyaWJ1dG9ySW5zdGFuY2VJZCIsImN1cnJlbnRQYW5lbCIsImRyb3BwZWQiLCJzcGF3bmVkQXBwcyIsInN1cnZpdmluZ1NwYXduZWQiLCJhY3RpdmVTcGF3bmVkSWQiLCJuZXh0UGFuZWwiLCJuZXh0U2Vzc2lvbiIsImRpc3Bvc2UiLCJ1bmluc3RhbGxQbHVnaW4iLCJwYW5lbCIsImFjdGl2ZVNwYXduZWRFbnRyeSIsImFjdGl2ZUFwcFRpdGxlIiwiYWN0aXZlSW50cm9kdWN0aW9uIiwiaW50cm9kdWN0aW9uIiwiaW50cm9kdWN0aW9uU2VlbktleSIsInJlcGxheUludHJvZHVjdGlvbk9uTG9hZCIsInBlcnNpc3RJbnRyb2R1Y3Rpb25TZWVuIiwiYWN0aXZlSW50cm9kdWN0aW9uUmVmIiwid2luZG93Iiwic2VsZiIsInRvcCIsImFjdGl2ZVR1dG9yaWFscyIsInR1dG9yaWFscyIsInR1dG9yaWFsUmVjb3JkZXJBdmFpbGFibGUiLCJCb29sZWFuIiwiZW52IiwiREVWIiwiYWN0aXZlVXRpbGl0eUJ5V2luZG93SWRSZWYiLCJhY3RpdmVUb29sSWRSZWYiLCJzZXRBY3RpdmVVdGlsaXR5Rm9yV2luZG93IiwiY2xlYXJBbGxXaW5kb3dVdGlsaXRpZXMiLCJ0b29sTWVhc3VyZXNCeVRvb2xJZFJlZiIsImFjdGl2ZVdpbmRvd0lkUmVmIiwiYWN0aW9uUGFuZUV4cGFuZGVkQnlXaW5kb3dJZFJlZiIsImFjdGlvblBhbmVTdGFnZWRBcmdzQnlLZXlSZWYiLCJpbnRyb2R1Y3Rpb25TdGVwSW5kZXhSZWYiLCJpbnRyb2R1Y3Rpb25Db21wbGV0ZWRJbnRlcmFjdGlvbnNSZWYiLCJzdGFydFR1dG9yaWFsUmVmIiwic3RvcFR1dG9yaWFsUmVmIiwidG9nZ2xlVHV0b3JpYWxSZWNvcmRpbmdSZWYiLCJ0dXRvcmlhbERyaXZlblJlZiIsInR1dG9yaWFsUGxheWluZ1JlZiIsInR1dG9yaWFsUmVjb3JkaW5nUmVmIiwidHV0b3JpYWxSZWNvcmRlclJlZiIsInNoZWxsU3RhdGVSZWYiLCJkaXNtaXNzSW50cm9kdWN0aW9uIiwiY29tcGxldGVkIiwiYWR2YW5jZUludHJvZHVjdGlvbkJ5RG9pbmciLCJjZWxlYnJhdGVPdmVycmlkZSIsInN0ZXBJbmRleCIsInN0ZXAiLCJzdGVwcyIsImNlbGVicmF0ZUlkIiwiaW50cm9kdWNlIiwiaW50ZXJhY3Rpb25zIiwiY29tcGxldGVJbnRyb2R1Y3Rpb25JbnRlcmFjdGlvbiIsIm1hdGNoZXMiLCJmaW5kSW5kZXgiLCJpbnRlcmFjdGlvbiIsImkiLCJpbmNsdWRlcyIsIm9yZGVyZWQiLCJjZWxlYnJhdGUiLCJleHBhbmRlZENvbW1hbmRJZFJlZiIsImNvbW1hbmRTdGFnZWRBcmdzQnlDb21tYW5kSWRSZWYiLCJpbmplY3RBY3RpdmVUb29sIiwidG9vbElkIiwiaW5qZWN0QWN0aXZlVXRpbGl0eSIsImtleSIsIndpdGhVdGlsaXR5IiwiYWN0aXZlVXRpbGl0eUlkIiwicmVsYXlQbHVnaW5CYWNrYm9uZU1lc3NhZ2UiLCJtZXNzYWdlQnl0ZXMiLCJzdGFydHNXaXRoIiwiYWN0b3JNZXNzYWdlIiwicGFyc2VkIiwicmVxdWVzdCIsInBvc3RNZXNzYWdlIiwidGVybWluYXRlIiwidW5yZWdpc3RlciIsInZhbHVlcyIsImNsZWFyIiwicHJpbWFyeSIsIndpbmRvd1RpdGxlIiwib3V0Y29tZSIsInJlZ2lzdHJ5SWRzIiwiaGFuZGxlUGx1Z2luQXZhaWxhYmxlIiwiYWxyZWFkeUxvYWRlZCIsInN1YnNjcmliZSIsImZpbmRQbHVnaW5Gb3JBY3Rpb24iLCJhY3Rpb24iLCJieUNvbnRyb2xsZXIiLCJyZXF1ZXN0Q29udGV4dE1lbnUiLCJjb250ZXh0TWVudSIsInJlZnJlc2hVaSIsInNjb3BlQXJnIiwiZXh0cmFJbnN0YW5jZXNPdmVycmlkZSIsImdlbmVyYXRpb24iLCJwcm9ncmFtIiwibGF5b3V0U2VlZEtleSIsImlzU2Vzc2lvblN3aXRjaCIsImNhY2hlIiwibGF5b3V0U2VlZCIsImV4dHJhSW5zdGFuY2VzRm9yRmV0Y2giLCJ3aW5kb3dJbnN0YW5jZXMiLCJjb250cmlidXRpb25zSnNvbiIsImFwcFJlZ2lzdHJhdGlvbnNKc29uIiwiZmxhdE1hcCIsImluc3RhbmNlIiwid2luZG93S2luZElkIiwicGFuZWxUYWJMZWF2ZXMiLCJyZXNwb25zZSIsInNsb3RDb250ZXh0IiwiY29udHJpYnV0b3JJbnN0YW5jZXMiLCJyZXNvbHZlSWZDaGFuZ2VkIiwicmVzb2x2ZWRXaW5kb3dzIiwicmVzb2x2ZWRQYW5lbHMiLCJQcm9taXNlIiwiYWxsIiwid2luZG93cyIsInJlcXVlc3RlZEVmZmVjdHMiLCJhcHBseUhvc3RFZmZlY3RzIiwiY29udHJpYnV0aW9uc1B1c2hLZXkiLCJwbHVnaW5FbnRyeSIsImFyZ3MiLCJqc29uIiwiaGFuZGxlQWN0aW9uIiwiYXBwUmVnaXN0cmF0aW9uc1B1c2hLZXkiLCJkeW5hbWljRW5nYWdlbWVudHMiLCJlbnRyaWVzIiwiZHluYW1pY01lYXN1cmVzIiwiZHluYW1pY1Rvb2xNZWFzdXJlcyIsImZyZXNoQXBwTGFiZWxzT3ZlcmxheSIsInRhYiIsImJvZHlLZXkiLCJrIiwicmVmcmVzaFNwYXduZWRVaSIsInNwYXduZWRTZWVkIiwiZnVsbFZpZXdTdGF0ZSIsInNpbmdsZVdpbmRvd0tpbmQiLCJzZXNzaW9uSWRlbnRpdHlLZXkiLCJyZW5kZXJFcnJvciIsImFjdGl2ZVNwYXduZWQiLCJ1cGRhdGVTcGFjZVBhbmVsIiwic3dpdGNoVG9NYW5hZ2VkQXBwIiwic1BsdWdpbiIsIm5leHRWaWV3U3RhdGUiLCJzeW5jU3Bhd25lZFBsdWdpbkRvY3VtZW50IiwicGx1Z2luSW5zdGFuY2VJZCIsInBhcnNlIiwic3luY0Vycm9yIiwiZW5zdXJlU3Bhd25lZFBsdWdpbiIsIm9zSW5zdGFuY2VJZCIsInNvdXJjZVZpZXdTdGF0ZSIsImV4aXN0aW5nIiwic3Bhd25lZElkIiwiZWZmZWN0cyIsImJhc2VTZXNzaW9uIiwidWlTY29wZSIsImVmZmVjdCIsInNldFBhbmVsIiwic2V0QWN0aXZlVXRpbGl0eSIsInNldEFjdGl2ZVRvb2wiLCJ2b3J0aWNlc0pzb24iLCJkb2N1bWVudFNlbGVjdGVkSWRzIiwiZG9jdW1lbnRIaWdobGlnaHRlZElkcyIsInBhdGNoV29ybGQzZENocm9tZSIsInBhdGNoIiwiZG9jdW1lbnRQYW5lbEtleSIsImRvY3VtZW50Tm9kZSIsImNhY2hlZCIsImRvY3VtZW50Q2FjaGVkIiwiZGlhbG9nSWQiLCJvcGVuRGlhbG9nIiwiZGlhbG9ncyIsInNlZWRBcmdzIiwicGF5bG9hZCIsImxvYWREb2N1bWVudCIsImxvYWRBcHBEb2N1bWVudFBhY2siLCJzcHJCeXRlcyIsIm9wZW5FeHRlcm5hbFVybCIsImZpbGVuYW1lIiwibWltZVR5cGUiLCJlbmNvZGluZyIsIml0ZW0iLCJpY29uUmVuZGVyRXhwb3J0IiwiaXRlbXMiLCJyZXN1bHQiLCJyZW5kZXIiLCJkYXRhVXJsIiwiYWNjZXB0IiwicmVhZEFzIiwiaW1wb3J0QWN0aW9uIiwibXVsdGlwbGUiLCJvcGVuZWQiLCJkaXNwYXRjaEFjdGlvbklkIiwiZGlzcGF0Y2hBcmdzIiwiZGVsYXlNcyIsImRpc3BhdGNoQWN0aW9uIiwiZnJhbWVBY3Rpb24iLCJkb25lQWN0aW9uIiwiZmFsbGJhY2tBY3Rpb24iLCJzYW1wbGVTdHJpZGUiLCJtYXhGcmFtZXMiLCJtYXhMb25nRWRnZVB4IiwiZnBzSGludCIsInJlcXVlc3RNZWRpYUZyYW1lcyIsInJlcXVlc3RKc29uIiwicmVzcG9uc2VBY3Rpb24iLCJyZXF1ZXN0UGx1Z2luRXhjaGFuZ2UiLCJjb250cmlidXRvciIsIm9wZXJhdG9ySWQiLCJpbnB1dEpzb24iLCJub2RlSGFzaCIsImJpbSIsIm91dHB1dEpzb24iLCJldmFsdWF0ZSIsInNwYXduUGx1Z2luSW5zdGFuY2UiLCJjYXRhbG9nIiwicHJvZ3JhbXMiLCJvcGVuUGx1Z2luSW5zdGFuY2UiLCJzcGF3bmVkQ291bnQiLCJpc1NwYXduZWRQbHVnaW5TZXNzaW9uIiwiYXBwbHlTaGVsbFVyaSIsInByZXNlcnZlZFZpZXdTdGF0ZSIsImN1cnJlbnRTZXNzaW9uIiwicGF0aCIsInJvdXRlIiwic3BhY2VJZCIsInN0dWRpb0NoYW5nZWQiLCJzdHVkaW9TZXNzaW9uIiwic3R1ZGlvQ29udHJvbGxlcklkIiwib3BlblJlc3BvbnNlIiwiYWN0aXZlUGFuZWxUYWIiLCJ1cmlFcnJvciIsInJlc29sdmVTeW5jVGFyZ2V0U2Vzc2lvbiIsIm9wZW5Eb2N1bWVudCIsInJlZiIsImJpbmRpbmdzIiwidGFyZ2V0U2Vzc2lvbiIsInNjaGVtYSIsIndhdGNoRXh0ZXJuYWwiLCJhdHRhY2hCYWNrYm9uZSIsImNsb3NlRG9jdW1lbnQiLCJkZXRhY2hCYWNrYm9uZSIsImF0dGFjaFN5bmNCYWNrYm9uZSIsInJlc3QiLCJzbGFzaCIsImluZGV4T2YiLCJiYXNlVXJsIiwicmVwbGFjZSIsImpvaW4iLCJkZXRhY2hTeW5jQmFja2JvbmUiLCJzcGF3blByb2dyYW0iLCJvbkFjdGlvbiIsInN1cGVydmlzb3IiLCJ0dXRvcmlhbElkIiwiaXNBcnJheSIsImdlc3R1cmUiLCJvbiIsInJlcXVlc3RlZCIsImZvcndhcmRlZCIsInRoZW4iLCJ1dGlsaXR5RXJyb3IiLCJ0b29sRXJyb3IiLCJ0cmltIiwiaG9zdFBvcnQiLCJjbGljayIsInAiLCJhY3Rpb25XaW5kb3dJZCIsImRpc3BhdGNoV2luZG93SWQiLCJkaXNwYXRjaFZpZXdTdGF0ZSIsImRlY2xhcmVkQWN0aW9uIiwiYWN0aW9ucyIsImludGVyYWN0aXZlQWN0aW9uIiwiYWN0aW9uRXJyb3IiLCJmaW5hbGx5Iiwibm90ZVNoZWxsQ29tbWFuZCIsImNvbW1hbmRJZCIsImRldGFpbCIsIm9uQWN0aW9uUmVmIiwib25BY3Rpb25TdGFibGUiLCJUVVRPUklBTF9ESVJFQ1RPUl9USUNLX01TIiwiYWN0aXZlVHV0b3JpYWwiLCJ0dXRvcmlhbENsb2NrUmVmIiwidHV0b3JpYWxDbG9jayIsInNldER1cmF0aW9uTXMiLCJzZXRSYXRlIiwicGxheSIsInBhdXNlIiwidWlCcmlkZ2VDdHhSZWYiLCJ0dXRvcmlhbExhc3RBcHBsaWVkTXNSZWYiLCJ0dXRvcmlhbERvY3VtZW50U25hcHNob3RSZWYiLCJwcmV2QWN0aXZlVHV0b3JpYWxJZFJlZiIsInByZXZpb3VzSWQiLCJkZWYiLCJyZWFkQXBwRG9jdW1lbnQiLCJzbmFwc2hvdEVycm9yIiwibG9hZEVycm9yIiwiY2FtZXJhS2V5ZnJhbWUiLCJzZWVrIiwic25hcHNob3RKc29uIiwicmVzdG9yZUVycm9yIiwiYXBwbHlUdXRvcmlhbFNsaWNlVG9TaGVsbCIsImNoYW5nZSIsInVpQ2hhbmdlcyIsImRvY3VtZW50VG91Y2hlZCIsImRvY3VtZW50RXZlbnQiLCJvcGVyYXRpb25zIiwiZm9yd2FyZCIsImZvcndhcmRzIiwiYmFja3dhcmRzIiwicHJldmlvdXNKc29uIiwiY2hlY2twb2ludElkIiwiYWx0ZXJuYXRpdmVJZCIsInRhcmdldElkIiwiY29tbWFuZCIsImxhc3RIZWF2eVRpY2tBdCIsImNhbWVyYVdpbmRvd0lkcyIsImtleWZyYW1lIiwidW5zdWJzY3JpYmUiLCJnZXRUaW1lTXMiLCJwb3NlIiwiaXNQbGF5aW5nIiwic2Vla1R1dG9yaWFsIiwibXMiLCJjbGFtcGVkIiwiYXRNcyIsInBsYXlQYXVzZVR1dG9yaWFsIiwic3RhcnRQb3NlQnlXaW5kb3ciLCJsaXZlIiwic3RhcnRlZEF0IiwidHdlZW4iLCJ0YXJnZXRQb3NlIiwiZHJpdmVyIiwic3RhcnRQb3NlIiwicmVxdWVzdEFuaW1hdGlvbkZyYW1lIiwic3RhcnRUdXRvcmlhbCIsInN0b3BUdXRvcmlhbCIsInRvZ2dsZVR1dG9yaWFsUmVjb3JkaW5nIiwicmVjb3JkZXIiLCJ2YWxpZGF0aW9uRXJyb3IiLCJjYXB0dXJlRXJyb3IiLCJpbnRlcnZhbCIsInNldEludGVydmFsIiwiY2xlYXJJbnRlcnZhbCIsImFkZFR1dG9yaWFsQ2hhcHRlciIsInR1dG9yaWFsQ2hhcHRlck1hcmtlcnMiLCJjaGFwdGVyIiwic3R1ZGlvU2Vzc2lvbkFjdGl2ZSIsInN0dWRpb1Nlc3Npb25Db250cm9sbGVySWQiLCJpZGVudGl0eSIsImJlYXQiLCJpbml0aWFsIiwic2V0VGltZW91dCIsInRpbWVyIiwiY2xlYXJUaW1lb3V0Iiwib25Ub2dnbGUiLCJhbmNob3IiLCJ2aXNpYmxlIiwiaG90a2V5IiwiYXBwZWFyYW5jZSIsImRldmljZSIsImNoYW5nZUxhbmd1YWdlIiwiZG9jdW1lbnRFbGVtZW50IiwibGFuZyIsInRoZW1lSWQiLCJvdmVycmlkZXMiLCJhcHBseU5hbWVkTGF5b3V0IiwiYXBwbHlNb2RlQ2hhbmdlIiwibW9kZUlkIiwiaGFuZGxlVGVtcGxhdGVEcm9wIiwicHJvamVjdGlvblNwZWMiLCJ0ZW1wbGF0ZUlkIiwibmV4dEV4dHJhSW5zdGFuY2VzIiwiZGlzcGxheUhvc3RSZWYiLCJkaXNwbGF5SG9zdCIsImJ1aWx0aW5MYXlvdXRzIiwibmFtZWRMYXlvdXRzIiwiY3VycmVudExheW91dCIsIm9uQXBwbHlMYXlvdXQiLCJ1aVRoZW1lQmFzZSIsInVpVGhlbWVEaXJ0eSIsInVpVGhlbWVMaXN0IiwidWlEcml2ZXJMaXN0Iiwia2V5YmluZGluZ3MiLCJjb250cm9sS2V5YmluZGluZ3MiLCJvc0NvbW1hbmRzIiwidGVybWlub2xvZ2llcyIsIm5vdGVPc0NvbW1hbmQiLCJkcmFmdFRoZW1lUGF0Y2giLCJzdHJ1Y3R1cmVkQ2xvbmUiLCJzZXRUaGVtZUlkIiwic2V0VGhlbWVDb2xvciIsImhleCIsImNvbG9ycyIsInNldFRoZW1lU3BhY2luZyIsInNwYWNpbmciLCJzZXRUaGVtZUZvbnRTdGFjayIsImZvbnRTdGFja3MiLCJzZXRUaGVtZVN0cm9rZSIsInN0cm9rZXMiLCJzZXRUaGVtZVJhZGl1cyIsInJhZGlpIiwic2V0VGhlbWVPcGFjaXR5Iiwib3BhY2l0aWVzIiwic2V0VGhlbWVNZXRyaWMiLCJzZWN0aW9uIiwibWV0cmljcyIsInNldFRoZW1lQXBwZWFyYW5jZVBhaW50IiwiYWxwaGEiLCJhcHBlYXJhbmNlcyIsInJlc2V0VGhlbWUiLCJzYXZlVGhlbWUiLCJ0cmltbWVkIiwic2x1ZyIsInRvTG93ZXJDYXNlIiwic2F2ZWQiLCJkZWxldGVUaGVtZSIsIl9yZW1vdmVkIiwiZXhwb3J0VGhlbWUiLCJpbXBvcnRUaGVtZSIsImNvbnRlbnRzIiwidWlEcml2ZXJCYXNlIiwidWlEcml2ZXJEaXJ0eSIsInNldERyaXZlcklkIiwic2V0RHJpdmVyRmllbGQiLCJzYXZlRHJpdmVyIiwiZGVsZXRlRHJpdmVyIiwidGhlbWVTYXZlTGFiZWwiLCJzZXRUaGVtZVNhdmVMYWJlbCIsImRyaXZlclNhdmVMYWJlbCIsInNldERyaXZlclNhdmVMYWJlbCIsImtleWJpbmRpbmdDYXB0dXJlQ29udHJvbElkIiwic2V0S2V5YmluZGluZ0NhcHR1cmVDb250cm9sSWQiLCJzZXRLZXliaW5kaW5nT3ZlcnJpZGUiLCJjb250cm9sSWQiLCJyZXNldEtleWJpbmRpbmdPdmVycmlkZSIsIm9uTmF2aWdhdGVUb0hvdGtleSIsImFkZEV2ZW50TGlzdGVuZXIiLCJyZW1vdmVFdmVudExpc3RlbmVyIiwic2V0dGluZ3NIb3N0UmVmIiwic2V0dGluZ3NIb3N0IiwiYXBwTGFiZWwiLCJkcml2ZXJJZCIsImRyaXZlckRpcnR5IiwiZHJpdmVycyIsInNldEFwcGVhcmFuY2UiLCJzZXRMYXlvdXQiLCJtb2JpbGVBY3RpdmUiLCJvblJlc2V0RG9jayIsInJlc2V0Iiwic2V0TG9jYWxlIiwic2V0VGVybWlub2xvZ3kiLCJ0aGVtZSIsInRoZW1lRGlydHkiLCJ0aGVtZXMiLCJmcmFtZXdvcmtEaXNwbGF5VGFicyIsImZyYW1ld29ya1NldHRpbmdzVGFicyIsInBsdWdpbnNIb3N0UmVmIiwicGx1Z2luc0hvc3QiLCJsb2FkZWRFbnRyeSIsInNvdXJjZUlkIiwiY2FuVW5pbnN0YWxsIiwiaW5zdGFsbCIsInVuaW5zdGFsbCIsInJlbG9hZCIsImZyYW1ld29ya1BsdWdpbnNUYWJzIiwiaGFuZGxlQXBwS2V5ZG93biIsInBhcnNlS2V5cyIsImlzRWRpdGFibGVUYXJnZXQiLCJIVE1MRWxlbWVudCIsInRhZyIsInRhZ05hbWUiLCJpc0NvbnRlbnRFZGl0YWJsZSIsImNsb3Nlc3QiLCJiaW5kaW5nIiwicGFydHMiLCJwYXJ0IiwibmVlZHNDdHJsIiwibmVlZHNTaGlmdCIsIm5lZWRzQWx0IiwiaGFzQ3RybCIsImN0cmxLZXkiLCJtZXRhS2V5Iiwic2hpZnRLZXkiLCJhbHRLZXkiLCJhY3Rpb25CeUlkIiwicHJldmVudERlZmF1bHQiLCJjaG9yZCIsImRlZmluaXRpb24iLCJzdGFnZWQiLCJpbnRlbnQiLCJhY3Rpb25JZCIsImFjdGl2ZVJpZ2h0UGFuZWxUYWIiLCJhY3RpdmVQYW5lbFRhYklkIiwid29ya2JlbmNoTGVmdFRhYnMiLCJwbHVnaW5MZWZ0VGFicyIsIm9yZGVyIiwiaGFzUGx1Z2luRG9jdW1lbnRUYWIiLCJkb2N1bWVudFRhYiIsImljb24iLCJ0cmVlIiwic2VjdGlvbnMiLCJkZXRhaWxzUmlnaHRUYWJzIiwic2V0dGluZ3NSaWdodFRhYnMiLCJmcmFtZXdvcmtVdGlsaXRpZXNIaXN0b3J5VGFiIiwiZnJhbWV3b3JrU3luY1RhYiIsInN5bmNVdGlsaXRpZXMiLCJzeW5jU3RhdHVzIiwiY29udHJvbCIsImFjdGl2ZVBsdWdpbk1hbmlmZXN0IiwiZXhhbXBsZU9wdGlvbnMiLCJzZWVuIiwiZXhhbXBsZXMiLCJleGFtcGxlIiwiZGlzcGF0Y2hBY3RpdmVFeGFtcGxlIiwiZXhhbXBsZVNlbGVjdEVsZW1lbnQiLCJtb2RlU3dpdGNoZXJFbGVtZW50IiwibW9kZSIsImlzQWN0aXZlIiwicmVzb2x2ZWRDb21tYW5kcyIsImNvbW1hbmRDYXRlZ29yeUxpc3QiLCJvbkNvbW1hbmQiLCJzb3VyY2UiLCJjb21tYW5kRXJyb3IiLCJjb21tYW5kQ2F0ZWdvcnlUYWJzIiwicmVzb2x2ZWRNb2RlVG9vbHMiLCJ0b29sIiwidG9vbFRhYnMiLCJkZWZhdWx0RG9jayIsInRvcExlZnQiLCJib3R0b21MZWZ0IiwiY2hpbGRyZW4iLCJ0b3BSaWdodCIsImJvdHRvbVJpZ2h0IiwiYm90dG9tTWlkZGxlIiwiYW5jaG9ycyIsImdldFNuYXBzaG90IiwiZG9jayIsIm1vYmlsZVBhbmVsVGFicyIsImFuY2hvclRhYnMiLCJhcHBUYWIiLCJkb2NrUGVyc2lzdGVkT25jZVJlZiIsIm5leHRTa2VsZXRvbiIsImRlZmF1bHRTa2VsZXRvbiIsInNhdmUiLCJkb2NrVWlQZXJzaXN0ZWRPbmNlUmVmIiwiZG9ja1VpUGVyc2lzdGVkU3RvcmVSZWYiLCJzaXplIiwiaGFzUGF0aE1lbW9yeSIsImhhc1RyZWVPcGVuIiwiaXNEZWZhdWx0IiwicGF0aE1lbW9yeSIsInRyZWVPcGVuIiwiaGFuZGxlVGFiRG9ja0Ryb3AiLCJtb3ZlIiwibmV4dERvY2siLCJ0YXJnZXRQYXRoIiwiZnJvbUFuY2hvciIsInNvdXJjZVRhYnMiLCJ0b0FuY2hvciIsImhhbmRsZVRyZWVVbml0RG9ja0Ryb3AiLCJzdHVkaW9PdmVycmlkZVRhYklkIiwic3R1ZGlvT3ZlcnJpZGVBbmNob3IiLCJkZXRhaWxzT3ZlcnJpZGVUYWJJZCIsImRldGFpbHNPdmVycmlkZUFuY2hvciIsImFjdGl2ZUludHJvZHVjdGlvblN0ZXAiLCJpbnRyb2R1Y3Rpb25FbGVtZW50SWRzIiwic2hvdyIsImludHJvZHVjdGlvblV0aWxpdHlJZCIsInV0aWxpdGllcyIsInV0aWxpdHkiLCJpbnRyb2R1Y3Rpb25BY3Rpb25XaW5kb3dTZWdtZW50IiwiYWN0aW9uSW5kZXgiLCJpbnRyb2R1Y3Rpb25QYW5lbFRhYklkIiwiZW5kc1dpdGgiLCJpbnRyb2R1Y3Rpb25Ub29sUGlja0lkcyIsImZyb21JbnRlcmFjdGlvbnMiLCJtYXRjaCIsImV4ZWMiLCJpbnRyb2R1Y3Rpb25QYW5lbFRhYkFuY2hvciIsImludHJvZHVjdGlvblV0aWxpdHlXaW5kb3dJZCIsImludHJvZHVjdGlvbk1lYXN1cmVXaW5kb3dJZCIsImtpbmRNZWFzdXJlcyIsIm9wdGlvbnMiLCJtZWFzdXJlcyIsImludHJvZHVjdGlvblRvb2xJZCIsImxhc3RJbnRyb2R1Y3Rpb25Ub29sSWRSZWYiLCJsYXN0SW50cm9kdWN0aW9uVG9vbFBpY2tTdGVwSWRSZWYiLCJyZXNvbHZlZCIsInRvb2xBbmNob3IiLCJsYXN0SW50cm9kdWN0aW9uUGFuZWxUYWJJZFJlZiIsImxvY2F0ZWQiLCJsYXN0SW50cm9kdWN0aW9uRXhwYW5kU3RlcElkUmVmIiwiZXhwYW5kSW50ZXJhY3Rpb25zIiwic3RhdGVTdWZmaXgiLCJjYXRhbG9ndWVLZXkiLCJzZWN0aW9uSWQiLCJwYW5lbEFjdGl2ZVBhdGhzIiwibGFzdFN0dWRpb092ZXJyaWRlVGFiSWRSZWYiLCJsYXN0RGV0YWlsc092ZXJyaWRlVGFiSWRSZWYiLCJtb2JpbGVQYW5lbCIsInRhYnMiLCJhY3RpdmVUYWJQYXRoIiwib25BY3RpdmVUYWJQYXRoQ2hhbmdlIiwib25QYXRoTWVtb3J5Q2hhbmdlIiwib25UcmVlT3BlblN0YXRlQ2hhbmdlIiwidHJlZUNvbnRlbnRSZXZpc2lvbiIsIm9wdGlvbiIsImJ1aWxkUGFuZWxTZWxlY3Rpb25Qcm9wcyIsIm9uVmlzaWJsZUNoYW5nZSIsInBhdGhDaGFuZ2VkIiwic2VsZWN0ZWRUb29sSWQiLCJuYXZiYXJJdGVtcyIsImxvZ29BbmRUaXRsZSIsImxvZ29TdmciLCJzaG93RXhhbXBsZVNlbGVjdCIsImNvbnRlbnQiLCJjZW50ZXJDb250ZW50IiwiY2VudGVyZWQiLCJzZWFyY2hJdGVtcyIsImNhdGVnb3J5Iiwib25TZWxlY3QiLCJkZWNsYXJlZEFjdGlvbklkcyIsImhvc3RXaW5kb3dGb3JBY3Rpb24iLCJpblBhbGV0dGUiLCJhcmdDYXJyeWluZyIsInJlc29sdmVkQWN0aW9uTGFiZWwiLCJkZXNjcmlwdGlvbiIsImNvbW1hbmRQYXRoIiwibW9kZVdpbmRvd3MiLCJhY3Rpb25QYW5lU2xpY2UiLCJhY3Rpb25zRm9sZGVkRm9yIiwidXRpbGl0eUJhckZvbGRlZEZvciIsIm1lYXN1cmVzRm9sZGVkRm9yIiwib25BY3Rpb25zRm9sZGVkRm9yIiwiZm9sZGVkIiwiY3Vyc29yRm9yIiwiY3Vyc29yIiwic3Bhd25lZEFwcCIsIndpbmRvd0tpbmQiLCJjaHJvbWUiLCJzcGF3bmVkVXRpbGl0aWVzIiwiZmlsbCIsInNob3dDb250cm9scyIsIm1lYXN1cmVzRm9sZGVkIiwiZW5nYWdlbWVudCIsInNlYXJjaCIsInV0aWxpdHlCYXIiLCJ1dGlsaXR5T3B0aW9ucyIsInV0aWxpdHlCYXJGb2xkZWQiLCJhY3Rpb25zRm9sZGVkIiwib25BY3Rpb25zRm9sZGVkQ2hhbmdlIiwiYmFzZVdpbmRvd3MiLCJyZXNvbHZlZEVuZ2FnZW1lbnQiLCJza2VsZXRvbiIsImV4dHJhV2luZG93cyIsImVmZmVjdGl2ZU1vZGVMYXlvdXQiLCJoYW5kbGVBY3RpdmVXaW5kb3dDaGFuZ2UiLCJsYXlvdXRDaGFuZ2VTZXR0bGVUaW1lb3V0UmVmIiwibGF5b3V0Q2hhbmdlQ2xhc3NpZmljYXRpb25SZWYiLCJsYXlvdXRDaGFuZ2VQcmV2aW91c1JlZiIsImhhbmRsZU1vZGVMYXlvdXRDaGFuZ2UiLCJjbGFzc2lmaWNhdGlvbiIsImZpbmFsQ2xhc3NpZmljYXRpb24iLCJjYW52YXMiLCJzdXBlcnZpc29yUGx1Z2luSWQiLCJzdXBlcnZpc29yU3RhdGUiLCJzdHVkaW9Ib21lQmFyIiwiZm9jdXNlZFNwYXduZWQiLCJmb2N1c2VkQmFyIiwiZmlsZSIsImZpbGVzIiwicmVhZGVyIiwiRmlsZVJlYWRlciIsIm9ubG9hZCIsInJlYWRBc0RhdGFVUkwiLCJjbG9zZWRTcGF3bmVkIiwibmV4dFNwYXduZWQiLCJjbG9zZWRQbHVnaW4iLCJmb290ZXJJdGVtcyIsImNsYXNzTmFtZSIsImJ1aWxkUGFuZWxQcm9wcyIsIm9uU2l6ZUNoYW5nZSIsInRhYkJhckhvc3QiLCJyb290IiwiYmVhY29uSWQiLCJub3RGb3VuZCIsImRhdGFzZXQiLCJzZW1pb09zTm90Rm91bmQiLCJzZW1pb09zUmVhZHkiLCJzZW1pb09zRXJyb3IiLCJkaXNwYXRjaFNoZWxsTWVudUFjdGlvbiIsImJ1aWxkU2hlbGxDb250ZXh0TWVudUl0ZW1zIiwiY2F0ZWdvcnlCeUFjdGlvbklkIiwic2hvcnRjdXQiLCJkZXN0cnVjdGl2ZSIsInNlcGFyYXRvciIsIm9yZ2FuaXplZCIsImhhbmRsZUNvbnRleHRNZW51IiwiY2xpZW50WCIsImNsaWVudFkiLCJmcm9tRW50cmllcyIsIm9uQ2hhbmdlIiwic3VibWl0QWN0aW9uIiwiY2FuY2VsQWN0aW9uIiwiX2MyIiwiX2MzIiwiX2M0IiwiX2M1IiwiX2M2Il0sImlnbm9yZUxpc3QiOltdLCJzb3VyY2VzIjpbIvCfn6bvuI9jb21wb25lbnQudHN4Il0sInNvdXJjZXNDb250ZW50IjpbIi8vICNyZWdpb24g8J+nsu+4j0hlYWRlclxuLy8g8J+OqO+4jyBmcmFtZXdvcmsvcHJvZHVjdHMvb3MvbW9kdWxlcy9yZW5kZXJlci9lbmdpbmUvZWxlbWVudHMvU2hlbGxIb3N0L2NvbXBvbmVudC50c3hcbi8qKiBAZW1vamkg8J+Pl++4jyBgU2hlbGxIb3N0YCDigJQgdGhlIGBGcmFtZXdvcmtPc1NoZWxsYCBvcmNoZXN0cmF0b3I6IGJvb3RzL2hvdC1zd2FwcyBwbHVnaW4gd2FzbSBtb2R1bGVzLFxuICogb3ducyB0aGUgd2luZG93L2RvY2svcGFuZWwgbGF5b3V0LCB3aXJlcyB0aGUgdHV0b3JpYWwgcmVjb3JkZXIvcGxheWVyLCBwcmVzZW5jZSwgYmFja2JvbmUgc3luYyxcbiAqIGNvbW1hbmQvdG9vbC91dGlsaXR5IHJpYmJvbnMsIGNvbnRleHQgbWVudXMsIGFuZCBtb3VudHMgZXZlcnkgcGVyLWFwcCB3aW5kb3cgdmlhIGBJbnRlcnByZXRlcmAuXG4gKiBUaGUgc2luZ2xlIGxhcmdlc3QgY29tcG9uZW50IGluIHRoZSByZW5kZXJlci1yZWFjdCBwYWNrYWdlLiAqL1xuLy8gI2VuZHJlZ2lvbiDwn6ey77iPSGVhZGVyXG5cbi8vICNyZWdpb24g8J+UjO+4j0FkYXB0ZXJzXG5pbXBvcnQgUmVhY3QsIHtcbiAgY3JlYXRlQ29udGV4dCxcbiAgdHlwZSBDU1NQcm9wZXJ0aWVzLFxuICB0eXBlIEtleWJvYXJkRXZlbnQsXG4gIHR5cGUgTW91c2VFdmVudCxcbiAgdHlwZSBSZWFjdEVsZW1lbnQsXG4gIHR5cGUgUmVhY3ROb2RlLFxuICB1c2VDYWxsYmFjayxcbiAgdXNlQ29udGV4dCxcbiAgdXNlRWZmZWN0LFxuICB1c2VNZW1vLFxuICB1c2VSZWR1Y2VyLFxuICB1c2VSZWYsXG4gIHVzZVN0YXRlLFxufSBmcm9tIFwicmVhY3RcIjtcbmltcG9ydCB7XG4gIHR5cGUgQWN0aW9uRGVzY3JpcHRvcixcbiAgdHlwZSBBcHBEZWZpbml0aW9uLFxuICBidWlsZENvbnRyaWJ1dGlvbnNKc29uLFxuICB0eXBlIENvbnRleHRNZW51SXRlbVNwZWMsXG4gIGNyZWF0ZUJyb3dzZXJTdG9yYWdlUG9ydCxcbiAgY3JlYXRlRGV2UGx1Z2luU291cmNlLFxuICBjcmVhdGVNZW1vcnlTdG9yYWdlUG9ydCxcbiAgY3JlYXRlU2NvcGVkU3RvcmFnZVBvcnQsXG4gIERvY2tMYXlvdXRTdG9yZSxcbiAgdHlwZSBEb2NrVWlQYW5lbFN0YXRlLFxuICBEb2NrVWlTdGF0ZVN0b3JlLFxuICBldmljdFBsdWdpbk1vZHVsZSxcbiAgZXhwYW5kUGx1Z2luUmVnaXN0cnksXG4gIEZSQU1FV09SS19QQU5FTF9UQUJfQ0FUQUxPR1VFX0lELFxuICBGUkFNRVdPUktfUEFORUxfVEFCX0RPQ1VNRU5UX0lDT05fSUQsXG4gIEZSQU1FV09SS19QQU5FTF9UQUJfRE9DVU1FTlRfSUQsXG4gIEZSQU1FV09SS19QQU5FTF9UQUJfSElTVE9SWV9JRCxcbiAgdHlwZSBIb3N0RWZmZWN0LFxuICB0eXBlIEludHJvZHVjdGlvbkludGVyYWN0aW9uLFxuICB0eXBlIExvY2FsaXplZExhYmVsLFxuICBOYW1lZExheW91dFN0b3JlLFxuICBub3JtYWxpemVBcHBMYWJlbHNPdmVybGF5LFxuICBvcmdhbml6ZUNvbnRleHRNZW51LFxuICBwYW5lbFRhYktpbmRJZCxcbiAgcGVuZGluZ1BhbmVsVWlOb2RlLFxuICBwZW5kaW5nV2luZG93VWlOb2RlLFxuICB0eXBlIFBsdWdpbkFwcExhYmVsc092ZXJsYXksXG4gIHR5cGUgUGx1Z2luQ29udGV4dE1lbnVSZXF1ZXN0LFxuICB0eXBlIFBsdWdpblNvdXJjZSxcbiAgdHlwZSBQbHVnaW5Tb3VyY2VFdmVudCxcbiAgdHlwZSBQbHVnaW5VaVJlZnJlc2hTZWN0aW9uUmVzcG9uc2UsXG4gIHBvc3RQbHVnaW5CYWNrYm9uZUluYm91bmQsXG4gIHR5cGUgUHJvZ3JhbUhvdFN3YXBFdmVudCxcbiAgUkVDT1JEX1RVVE9SSUFMX0FDVElPTl9JRCxcbiAgcmVnaXN0ZXJQbHVnaW5CYWNrYm9uZVJvdXRlLFxuICByZXNvbHZlRXh0ZXJuYWxTbG90cyxcbiAgcmVzb2x2ZUxheW91dEZvck1vZGUsXG4gIHJlc29sdmVNb2RlVG9vbHMsXG4gIHJlc29sdmVQbGF5Z3JvdW5kRGVmYXVsdEFwcElkLFxuICByZXNvbHZlUGx1Z2luSG9zdENvbmZpZyxcbiAgcmVzb2x2ZVBsdWdpblJlZ2lzdHJ5SWQsXG4gIHJlc29sdmVVaURpcnR5U2NvcGUsXG4gIHJlc29sdmVXaW5kb3dBY3Rpb25zLFxuICBTRVRfQUNUSVZFX1RPT0xfQUNUSU9OX0lELFxuICBTRVRfQUNUSVZFX1VUSUxJVFlfQUNUSU9OX0lELFxuICB0eXBlIFNoZWxsQnJhbmQsXG4gIFNUQVJUX0lOVFJPRFVDVElPTl9BQ1RJT05fSUQsXG4gIFNUQVJUX1RVVE9SSUFMX0FDVElPTl9JRCxcbiAgdHlwZSBTdG9yYWdlUG9ydCxcbiAgVFVUT1JJQUxfQ09OVkVSR0VfTVMsXG4gIHR5cGUgVHV0b3JpYWxBc3NldFNyYyxcbiAgdHlwZSBUdXRvcmlhbENhbWVyYVN0YXRlLFxuICB0eXBlIFR1dG9yaWFsQ2hhcHRlcixcbiAgdHlwZSBUdXRvcmlhbERlZmluaXRpb24sXG4gIHR5cGUgVHV0b3JpYWxEb2N1bWVudEV2ZW50S2luZCxcbiAgdHlwZSBUdXRvcmlhbEV2ZW50LFxuICB0eXBlIFR1dG9yaWFsR2VzdHVyZUN1ZSxcbiAgdHlwZSBUdXRvcmlhbFVpQ2hhbmdlLFxuICB0eXBlIFR1dG9yaWFsVWlTbmFwc2hvdCxcbiAgdHlwZSBUdXRvcmlhbFZpZGVvQ3VlLFxuICB0eXBlIFVpRGlydHlTY29wZSxcbiAgdHlwZSBVaU5vZGUsXG4gIHR5cGUgVXRpbGl0eU5vZGUsXG4gIHdpbmRvd0VsZW1lbnRJZCxcbiAgdHlwZSBXaW5kb3dFbmdhZ2VtZW50LFxuICB0eXBlIFdpbmRvd0xheW91dCxcbiAgdHlwZSBXaW5kb3dNZWFzdXJlLFxufSBmcm9tIFwiQHNlbWlvLXRlY2gvZnJhbWV3b3JrLWNvcmVcIjtcbmltcG9ydCB7XG4gIHR5cGUgQmFja2JvbmVXb3JrZXJSZXF1ZXN0LFxuICB0eXBlIEJhY2tib25lV29ya2VyUmVzcG9uc2UsXG4gIGJ1aWxkRmlsZUJhY2tib25lVXJpLFxuICBidWlsZEZvbGRlckJhY2tib25lVXJpLFxuICBidWlsZEZyYW1ld29ya1N5bmNVdGlsaXRpZXMsXG4gIGJ1aWxkUmVtb3RlQmFja2JvbmVVcmksXG4gIGRlY29kZUJhY2tib25lTWVzc2FnZSxcbiAgZGVjb2RlQmFja2JvbmVXb3JrZXJSZXNwb25zZSxcbiAgZGVjb2RlUGFja1ZhbHVlLFxuICB0eXBlIERvY3VtZW50QWN0b3JNc2csXG4gIGVuY29kZUFjdGlvbldpcmUsXG4gIGVuY29kZUJhY2tib25lTWVzc2FnZSxcbiAgZW5jb2RlQmFja2JvbmVXb3JrZXJSZXF1ZXN0LFxuICBlbmNvZGVPcGVyYXRpb25FbnZlbG9wZXNQYWNrLFxuICBGUkFNRVdPUktfU1lOQ19DT05UUk9MTEVSX0lELFxuICBvcGVyYXRpb25FbnZlbG9wZUZyb21XaXJlLFxuICBvcGVyYXRpb25FbnZlbG9wZVRvV2lyZSxcbiAgdHlwZSBQZXJzaXN0ZW5jZUJpbmRpbmcsXG59IGZyb20gXCJAc2VtaW8tdGVjaC9mcmFtZXdvcmstb3MtY29yZVwiO1xuaW1wb3J0IHtcbiAgZGVjb2RlV29ybGRQcm9qZWN0aW9uVGVtcGxhdGVJZCxcbiAgd29ybGRQcm9qZWN0aW9uU3BlY0ljb25JZCxcbiAgd29ybGRQcm9qZWN0aW9uU3BlY0xhYmVsLFxufSBmcm9tIFwiQHNlbWlvLXRlY2gvaW5maW5pdGUtd29ybGQtcjNmXCI7XG5pbXBvcnQge1xuICB0eXBlIEFuY2hvcixcbiAgQU5DSE9SUyxcbiAgQXBwLFxuICBhcHBseURvY2tTa2VsZXRvbixcbiAgYXBwbHlVaVRoZW1lVG9Sb290LFxuICBib3JkZXJOb3JtYWxCb3R0b21DbGFzcyxcbiAgYnVpbGRLZXlzQnlBY3Rpb25JZCxcbiAgYnVpbHRpblVpRHJpdmVycyxcbiAgYnVpbHRpblVpVGhlbWVzLFxuICBCdXR0b25Hcm91cCxcbiAgQnV0dG9uR3JvdXBJdGVtLFxuICBDYW52YXNTa2VsZXRvbixcbiAgQ0VMRUJSQVRFX1NUQU1QX0RVUkFUSU9OX01TLFxuICBjZWxlYnJhdGVBbGxFbGVtZW50cyxcbiAgY2VsZWJyYXRlRWxlbWVudHMsXG4gIGNoaWxkRWxlbWVudElkLFxuICBDaHJvbWVBd2FyZVdpbmRvd1Njcm9sbFN1cmZhY2UsXG4gIGNsZWFyVWlUaGVtZUZyb21Sb290LFxuICBjbixcbiAgY29tcG9zZUNvbnRyb2xLZXliaW5kaW5ncyxcbiAgY29tcG9zZVR1dG9yaWFsVWksXG4gIENvbnRleHRNZW51Q29udHJvbGxlcixcbiAgdHlwZSBDb250ZXh0TWVudUl0ZW0sXG4gIGNyZWF0ZVNoZWxsU2NvcGUsXG4gIGNyZWF0ZVR1dG9yaWFsQ2xvY2ssXG4gIERFRkFVTFRfVUlfRFJJVkVSLFxuICBkZXRlY3RTaGVsbExvY2FsZSxcbiAgZGlzcG9zZVNoZWxsSTE4bkluc3RhbmNlLFxuICBkb2NrU2tlbGV0b25PZixcbiAgZG9ja1NrZWxldG9uc0VxdWFsLFxuICBlbGVtZW50SWRTZWxlY3RvcixcbiAgdHlwZSBFbGVtZW50c1N1cmZhY2VBcHBlYXJhbmNlLFxuICB0eXBlIEVsZW1lbnRzU3VyZmFjZURldmljZSxcbiAgZmluZFBhbmVsVGFiSW5Eb2NrLFxuICBmaW5kUGFuZWxUYWJOb2RlLFxuICBmaW5kUGFuZWxUYWJQYXRoLFxuICBGb290ZXIsXG4gIGdldFR1dG9yaWFsQ2FtZXJhRHJpdmVyLFxuICBJY29uLFxuICB0eXBlIEljb25OYW1lLFxuICBpY29uUmVuZGVyUG9ydCxcbiAgaW5zZXJ0V2luZG93QXREcm9wWm9uZSxcbiAgaW50ZXJhY3RpdmVBY3RpdmVGaWxsQ2xhc3MsXG4gIGludGVycG9sYXRlVHV0b3JpYWxDYW1lcmEsXG4gIGlzQ29udGV4dE1lbnVQb2ludGVyVGFyZ2V0LFxuICBMYXlvdXQsXG4gIExldmVsUHJvdmlkZXIsXG4gIGxvYWRpbmdCb3JkZXJDbGFzcyxcbiAgTW9kZSxcbiAgdHlwZSBNb2RlQ2FudmFzRHJvcFRhcmdldCxcbiAgdHlwZSBNb2RlV2luZG93RGVzY3JpcHRvcixcbiAgbW92ZVRhYkluRG9jayxcbiAgbW92ZVRyZWVVbml0SW5Eb2NrLFxuICBOYXZiYXIsXG4gIE5hdmJhckV4YW1wbGVTZWxlY3QsXG4gIG5hdmJhckZpbGxJdGVtLFxuICB0eXBlIE5hdmJhckl0ZW0sXG4gIFBhbmVsQ2hyb21lVGFiQmFyLFxuICB0eXBlIFBhbmVsRG9jayxcbiAgUGFuZWxEb2NrUHJvdmlkZXIsXG4gIHBhbmVsVGFiQ2hpbGRyZW4sXG4gIHR5cGUgUGFuZWxUYWJEb2NrTW92ZSxcbiAgdHlwZSBQYW5lbFRhYk5vZGUsXG4gIHR5cGUgUGFuZWxUYWJTZWxlY3Rpb25PcHRpb25zLFxuICB0eXBlIFBhbmVsVHJlZVVuaXREb2NrTW92ZSxcbiAgcGFyc2VVaVRoZW1lLFxuICByZWFkU3RvcmVkSW50cm9kdWN0aW9uU2VlbixcbiAgcmVhZFN0b3JlZFVpQ2hyb21lTG9jYWxlLFxuICByZWFkU3RvcmVkVWlDaHJvbWVUaGVtZVNuYXBzaG90LFxuICByZWNvbmNpbGVBY3RpdmVQYXRoLFxuICByZXNvbHZlVWlEcml2ZXIsXG4gIFNlbWlvTG9nbyxcbiAgc2VtaW9UaGVtZSxcbiAgc2VyaWFsaXplVWlUaGVtZSxcbiAgc2V0QWN0aXZlVWlUaGVtZSxcbiAgU2hlbGxCcmFuZExvZ28sXG4gIHNoZWxsQ2hyb21lVGl0bGVDbGFzc05hbWUsXG4gIHR5cGUgU2hlbGxTY29wZSxcbiAgU2hlbGxTY29wZVByb3ZpZGVyLFxuICBzaW5nbGVUcmVlTGVhZixcbiAgc3RhdGljVHJlZVBhbmVsRGVmaW5pdGlvbixcbiAgVGV4dFNlbGVjdGlvbkNvbnRleHRNZW51SG9zdCxcbiAgdHlwZSBUaGVtZUFwcGVhcmFuY2VOYW1lLFxuICB0eXBlIFRoZW1lUGFsZXR0ZUdyb3VwLFxuICBUb2dnbGUsXG4gIFR1dG9yaWFsQmFyLFxuICB0dXRvcmlhbENhbWVyYUF0LFxuICBUdXRvcmlhbENhcHRpb25zLFxuICB0eXBlIFR1dG9yaWFsQ2hhcHRlck1hcmtlcixcbiAgdHlwZSBUdXRvcmlhbENsb2NrLFxuICB0eXBlIFR1dG9yaWFsQ2xvY2tQb3J0LFxuICB0dXRvcmlhbEN1ZXNCZXR3ZWVuLFxuICBUdXRvcmlhbEdob3N0UG9pbnRlcixcbiAgdHV0b3JpYWxTbGljZSxcbiAgdHlwZSBUdXRvcmlhbFNsaWNlLFxuICBUdXRvcmlhbFZpZGVvT3ZlcmxheSxcbiAgVUlfTU9CSUxFX01FRElBX1FVRVJZLFxuICBVSV9URVJNSU5PTE9HWV9OQVRJVkUsXG4gIHR5cGUgVWlDaHJvbWVMYXlvdXQsXG4gIFVJRGlhbG9nLFxuICB0eXBlIFVpRHJpdmVyLFxuICBVSUludHJvZHVjdGlvbixcbiAgVWlLZXliaW5kaW5nc1Byb3ZpZGVyLFxuICB0eXBlIFVpTG9jYWxlLFxuICB0eXBlIFVpU3RhdHVzLFxuICB0eXBlIFVpVGhlbWUsXG4gIHVzZUFjdGlvbkhvdGtleSxcbiAgdXNlRWxlbWVudHNTdXJmYWNlQ2hyb21lLFxuICB1c2VMYWJlbCxcbiAgdXNlTWVkaWFRdWVyeSxcbiAgdXNlUGFuZWxDaHJvbWVIb3RrZXlzLFxuICB1c2VTaGVsbEtleWRvd24sXG4gIHVzZVNoZWxsU2NvcGUsXG4gIHVzZVR1dG9yaWFsQ2xvY2ssXG4gIHZhbGlkYXRlVHV0b3JpYWwsXG4gIFdpbmRvd0JvZHlTa2VsZXRvbixcbiAgdHlwZSBXaW5kb3dMYXlvdXROb2RlLFxuICB0eXBlIFdpbmRvd1RlbXBsYXRlRHJvcFBheWxvYWQsXG4gIHdyaXRlU3RvcmVkSW50cm9kdWN0aW9uU2VlbixcbiAgd3JpdGVTdG9yZWRVaUNocm9tZUFwcGVhcmFuY2UsXG4gIHdyaXRlU3RvcmVkVWlDaHJvbWVMYXlvdXQsXG4gIHdyaXRlU3RvcmVkVWlDaHJvbWVMb2NhbGUsXG4gIHdyaXRlU3RvcmVkVWlDaHJvbWVUZXJtaW5vbG9neSxcbiAgd3JpdGVTdG9yZWRVaUNocm9tZVRoZW1lSWQsXG4gIHdyaXRlU3RvcmVkVWlDaHJvbWVUaGVtZVNuYXBzaG90LFxuICB3cml0ZVN0b3JlZFVpQ3VzdG9tRHJpdmVycyxcbiAgd3JpdGVTdG9yZWRVaUN1c3RvbVRoZW1lcyxcbiAgd3JpdGVTdG9yZWRVaURyaXZlcklkLFxuICB3cml0ZVN0b3JlZFVpS2V5YmluZGluZ092ZXJyaWRlcyxcbn0gZnJvbSBcIkBzZW1pby10ZWNoL3VpLXJlYWN0XCI7XG5pbXBvcnQge1xuICBkZWNsYXJhdGl2ZVN1cmZhY2VTdGF0dXMsXG4gIEludGVycHJldGVkVWlOb2RlLFxuICBQbHVnaW5TdXJmYWNlQWN0aW9uc0NvbnRleHQsXG4gIFNoZWxsQ29udGV4dE1lbnVGYWxsYmFja0NvbnRleHQsXG4gIHdpcmVMYWJlbCxcbn0gZnJvbSBcIi4uL0ludGVycHJldGVyL/Cfn6bvuI9jb21wb25lbnQudHN4XCI7XG5pbXBvcnQge1xuICBhY3Rpb25TdGFnZUtleSxcbiAgdHlwZSBBY3RpdmVTZXNzaW9uLFxuICBFTVBUWV9TSEVMTF9ERUZBVUxUUyxcbiAgRU1QVFlfU0hFTExfTE9DS1MsXG4gIHR5cGUgRXh0cmFXaW5kb3dJbnN0YW5jZSxcbiAgdHlwZSBGcmFtZXdvcmtPc0RlZmF1bHRzLFxuICBpbml0aWFsU2hlbGxTdGF0ZSxcbiAgaXNFcGhlbWVyYWxTaGVsbEJyYW5kLFxuICB0eXBlIExvYWRlZFByb2dyYW1TdGF0ZSxcbiAgcmVzb2x2ZUJvb3RFeGFtcGxlSWQsXG4gIHR5cGUgUmVzb2x2ZWRTaGVsbExvY2tzLFxuICBTaGVsbEZhdWx0Qm91bmRhcnksXG4gIHNoZWxsUmVkdWNlcixcbiAgc2hvdWxkUGVyc2lzdEludHJvZHVjdGlvblNlZW4sXG4gIHNob3VsZFJlcGxheUludHJvZHVjdGlvbk9uTG9hZCxcbiAgdHlwZSBTcGFjZVBhbmVsU3RhdGUsXG4gIHR5cGUgU3BhY2VQcm9ncmFtRW50cnksXG4gIHR5cGUgU3Bhd25lZEFwcEVudHJ5LFxuICB0eXBlIFZpZXdNb2RlbCxcbn0gZnJvbSBcIi4uL1NoZWxsL/Cfn6bvuI9jb21wb25lbnQudHN4XCI7XG5pbXBvcnQge1xuICBiZWdpbkludGVyYWN0aXZlUGx1Z2luQWN0aW9uLFxuICBjbGVhclBlbmRpbmdXb3JsZFByb2plY3Rpb24sXG4gIGVuZEludGVyYWN0aXZlUGx1Z2luQWN0aW9uLFxuICBtYXBDb250ZXh0TWVudVNwZWNzLFxuICByZWdpc3RlclBlbmRpbmdXb3JsZFByb2plY3Rpb24sXG4gIFdpbmRvd0luc3RhbmNlSWRDb250ZXh0LFxufSBmcm9tIFwiLi4vV29ybGQzZEhvc3Qv8J+fpu+4j2NvbXBvbmVudC50c3hcIjtcbmltcG9ydCB7XG4gIERFRkFVTFRfUEFORUxfV0lEVEhfUFgsXG4gIEVNUFRZX0FQUF9MQUJFTFNfT1ZFUkxBWSxcbiAgRlJBTUVXT1JLX0NBVEVHT1JZX0NPTU1BTkRfSUQsXG4gIEZSQU1FV09SS19DQVRFR09SWV9ESVNQTEFZX0lELFxuICBGUkFNRVdPUktfQ0FURUdPUllfVE9PTF9JRCxcbiAgRlJBTUVXT1JLX1JFU0VSVkVEX0FDVElPTl9JRFMsXG4gIExBWU9VVF9DSEFOR0VfU0VUVExFX01TLFxuICBOT1RFX1dPUkxEX05BVklHQVRJT05fQUNUSU9OX0lELFxuICBQQU5FTF9UQUJfQkFSX0hPU1RTLFxuICBQUkVTRU5DRV9IRUFSVEJFQVRfSU5URVJWQUxfTVMsXG4gIFRVVE9SSUFMX1JFQ09SRElOR19FWENMVURFRF9BQ1RJT05fSURTLFxuICBhY3Rpb25DYXRlZ29yeUlkLFxuICBhY3Rpb25SZXF1aXJlc1N0YWdlZEZvcm0sXG4gIGFwcERvY3VtZW50TGFiZWwsXG4gIGFwcFdpbmRvd0RvY3VtZW50TGFiZWwsXG4gIGFwcGx5RnJhbWV3b3JrTGF5b3V0U2VlZCxcbiAgYXBwbHlUdXRvcmlhbFVpQ2hhbmdlVG9TaGVsbCxcbiAgYXBwbHlUdXRvcmlhbFVpU25hcHNob3RUb1NoZWxsLFxuICBhcHBseVVpUmVmcmVzaFJlc3BvbnNlVG9DYWNoZSxcbiAgYnVpbGRBY3RpdmVVdGlsaXR5QnlXaW5kb3dJZCxcbiAgYnVpbGRDb21tYW5kQ2F0ZWdvcnlUYWJzLFxuICBidWlsZE5vdGVTaGVsbENvbW1hbmRBY3Rpb24sXG4gIGJ1aWxkT3NDb21tYW5kcyxcbiAgYnVpbGRTcGFjZVBhbmVsU3RhdGUsXG4gIGJ1aWxkVG9vbFRhYnMsXG4gIGJ1aWxkVWlSZWZyZXNoUmVxdWVzdCxcbiAgY2FwdHVyZUN1cnJlbnRGcmFtZXdvcmtMYXlvdXQsXG4gIGNhcHR1cmVUdXRvcmlhbFVpU25hcHNob3QsXG4gIGNhdGVnb3J5VGFiSWNvbixcbiAgY2xhc3NpZnlXaW5kb3dMYXlvdXRDaGFuZ2UsXG4gIGNvbW1hbmRDYXRlZ29yaWVzLFxuICBjb21tYW5kQ2F0ZWdvcnlMYWJlbCxcbiAgZGlzcGF0Y2hPcGVuZWRGaWxlcyxcbiAgZGlzcGF0Y2hPc0NvbW1hbmQsXG4gIGRvd25sb2FkRGF0YVVybCxcbiAgZG93bmxvYWRNZWRpYUV4cG9ydCxcbiAgZmxhdHRlblBhbmVsVGFiTGVhdmVzLFxuICBpbnRyb2R1Y3Rpb25UYXJnZXRzV2luZG93LFxuICBsb2FkUGx1Z2luTW9kdWxlUmVzaWxpZW50LFxuICBtYWtlRWZmZWN0RGlzcGF0Y2hPbmUsXG4gIG1lcmdlUmVjb3JkUHJlc2VydmluZ0lkZW50aXR5LFxuICBwYW5lbEFuY2hvckZvckdyb3VwLFxuICBwYW5lbEpzb25Gcm9tU3RhdGUsXG4gIHBhbmVsVGFiRGVmaW5pdGlvblRvTm9kZSxcbiAgcGFyc2VQYW5lbFN0YXRlLFxuICBwYXJzZVNoZWxsUm91dGUsXG4gIHBhdGNoRG9jdW1lbnRUcmVlU2VsZWN0ZWRJZHMsXG4gIHBhdGNoV29ybGQzZENocm9tZU9udG9Ob2RlLFxuICBwcmVzZW5jZUNsaWVudElkZW50aXR5LFxuICBwcmVzZXJ2ZUpzb25JZGVudGl0eSxcbiAgcmVuZGVyU3RhZ2VkQXJnQ29udHJvbCxcbiAgcmVxdWVzdEZpbGVPcGVuLFxuICByZXNvbHZlQXBwRG9jdW1lbnQsXG4gIHJlc29sdmVBcHBMYWJlbCxcbiAgcmVzb2x2ZUNhbnZhc0JvZHlLZXksXG4gIHJlc29sdmVDb21tYW5kcyxcbiAgcmVzb2x2ZURpYWxvZ0RlZmluaXRpb24sXG4gIHJlc29sdmVEb2N1bWVudEJ5QXBwSWQsXG4gIHJlc29sdmVGcmFtZXdvcmtMYXlvdXRTZWVkLFxuICByZXNvbHZlSW50cm9kdWN0aW9uRGVmaW5pdGlvbixcbiAgcmVzb2x2ZUtleWJpbmRpbmdJbnRlbnQsXG4gIHJlc29sdmVNYW5pZmVzdExhYmVsLFxuICByZXNvbHZlUGFuZWxUYWJMYWJlbCxcbiAgcmVzb2x2ZVV0aWxpdHlBY3RpdmF0aW9uLFxuICByZXNvbHZlVXRpbGl0eU5vZGVzLFxuICByZXNvbHZlV2luZG93RW5nYWdlbWVudCxcbiAgcmV0aXRsZVdpbmRvd0xheW91dE5vZGUsXG4gIHJ1blJlcXVlc3RNZWRpYUZyYW1lcyxcbiAgc2NoZWR1bGVEaXNwYXRjaEFjdGlvbixcbiAgc2Vzc2lvbldpbmRvd0luc3RhbmNlcyxcbiAgc2hlbGxMYWJlbCxcbiAgc2hlbGxUYWJJY29uLFxuICBzcGF3bmVkV2luZG93Q2hyb21lRm9yS2luZCxcbiAgc3R1ZGlvUGFuZWxGb2N1c2luZ1NwYXduZWQsXG4gIHN5bmNEb2N1bWVudElkLFxuICBzeW50aGVzaXplTG9jYWxpemVkTGFiZWwsXG4gIHRvb2xJZEZyb21QYW5lbFRhYklkLFxuICB1c2VVSUhpc3RvcnksXG4gIHV0aWxpdHlCYXJOb2RlLFxuICB1dGlsaXR5Tm9kZVRyZWVDb250YWluc0lkLFxuICB2aWV3U3RhdGVXaXRoU3BhY2VQYW5lbCxcbiAgd2luZG93QWN0aW9uUGFuZU5vZGUsXG4gIHdpbmRvd0VuZ2FnZW1lbnRUb1NlYXJjaFNwZWMsXG4gIHdpbmRvd0VuZ2FnZW1lbnRUb1NwZWMsXG4gIHdpbmRvd01lYXN1cmVUcmVlQ29udGFpbnNJZCxcbiAgd2luZG93TWVhc3VyZXNDaHJvbWUsXG4gIHR5cGUgUmVzb2x2ZWRDb21tYW5kLFxuICB0eXBlIFVpUmVmcmVzaENhY2hlLFxufSBmcm9tIFwiLi4vU2hlbGxIZWxwZXJzL/Cfn6bvuI9jb21wb25lbnQudHN4XCI7XG5cbmltcG9ydCB7IGFQcm9qZWN0T2ZMdWhVZGtGb290ZXJJdGVtLCBmdW5kZWRCeVp1a3VuZnRCYXVGb290ZXJJdGVtIH0gZnJvbSBcIi4uLy4uLy4uLy4uLy4uLy4uLy4uLy4uL+KZu++4j21pdC1iZXN0YW5kL/Cfp7rvuI9kZW1vbnN0cmF0b3Iv4pqb77iPZm9vdGVyLnRzeFwiO1xuaW1wb3J0IHsgRU5UV0VSRkVOX01JVF9CRVNUQU5EX0JSQU5EX0lEUyB9IGZyb20gXCIuLi8uLi8uLi8uLi8uLi8uLi8uLi8uLi/imbvvuI9taXQtYmVzdGFuZC/wn6e677iPZGVtb25zdHJhdG9yL/Cfn6bvuI9icmFuZC50c1wiO1xuaW1wb3J0IHsgY3JlYXRlRnJhbWV3b3JrRGlzcGxheVBhbmVsVGFicywgY3JlYXRlRnJhbWV3b3JrUGx1Z2luc1BhbmVsVGFicywgY3JlYXRlRnJhbWV3b3JrU2V0dGluZ3NQYW5lbFRhYnMsIHR5cGUgRGlzcGxheUhvc3RBcGksIFBsdWdpblJlY292ZXJ5UGFuZWwsIHR5cGUgUGx1Z2luc0hvc3RBcGksIHR5cGUgUGx1Z2luc1BhbmVsRW50cnksIHR5cGUgU2V0dGluZ3NIb3N0QXBpLCBTaGVsbFJvdXRlTm90Rm91bmRQYWdlLCB1c2VOYW1lZExheW91dEhvc3QgfSBmcm9tIFwiLi4vQ2hyb21lUGFuZWxzL/Cfn6bvuI9jb21wb25lbnQudHN4XCI7XG5pbXBvcnQgeyB0eXBlIFBsdWdpbldhc21IYW5kbGUgfSBmcm9tIFwiLi4vUGx1Z2luUnVudGltZS/wn5+m77iPY29tcG9uZW50LnRzeFwiO1xuXG5pbXBvcnQgeyBTeW5jQXR0YWNoQ2FyZCB9IGZyb20gXCIuLi9TaGVsbFN5bmMv8J+fpu+4j2NvbXBvbmVudC50c3hcIjtcbmltcG9ydCB7IFVJRmluZCwgVUlGaW5kUHJvdmlkZXIsIFVJU2VhcmNoLCB0eXBlIFVJU2VhcmNoSXRlbSB9IGZyb20gXCIuLi9TaGVsbFNlYXJjaC/wn5+m77iPY29tcG9uZW50LnRzeFwiO1xuaW1wb3J0IHsgVVRJTElUWV9DQVRFR09SWV9JQ09OX0lEIH0gZnJvbSBcIi4uL1V0aWxpdHlUcmVlL/Cfn6bvuI9jb21wb25lbnQudHN4XCI7XG5pbXBvcnQgeyBjb2VyY2VXaXJlQnl0ZXMgfSBmcm9tIFwiLi4vUGx1Z2luUnVudGltZS/wn5+m77iPY29tcG9uZW50LnRzeFwiO1xuLy8gI2VuZHJlZ2lvbiDwn5SM77iPQWRhcHRlcnNcblxuLy8jcmVnaW9uIEZyYW1ld29ya09zU2hlbGxcbi8qKiBAZW1vamkg8J+Pt++4jyBMZXRzIGEgcGVyLXdpbmRvdyBob3N0IHJld3JpdGUgaXRzIE1vZGUgd2luZG93IHRpdGxlIChlLmcuIGxpdmUgcHJvamVjdGlvbiBsYWJlbCkuICovXG5leHBvcnQgY29uc3QgU2V0V2luZG93VGl0bGVDb250ZXh0ID0gY3JlYXRlQ29udGV4dDwoKHdpbmRvd0lkOiBzdHJpbmcsIHRpdGxlOiBzdHJpbmcpID0+IHZvaWQpIHwgbnVsbD4obnVsbCk7XG5cbi8qKiBAZW1vamkg8J+WvO+4jyBMZXRzIGEgcGVyLXdpbmRvdyBob3N0IHJld3JpdGUgaXRzIE1vZGUgd2luZG93IGljb24gKGUuZy4gbGl2ZSBwcm9qZWN0aW9uIGdseXBoKS4gKi9cbmV4cG9ydCBjb25zdCBTZXRXaW5kb3dJY29uQ29udGV4dCA9IGNyZWF0ZUNvbnRleHQ8KCh3aW5kb3dJZDogc3RyaW5nLCBpY29uSWQ6IEljb25OYW1lKSA9PiB2b2lkKSB8IG51bGw+KG51bGwpO1xuXG5jb25zdCBFTVBUWV9LRVlTX0JZX0FDVElPTl9JRCA9IG5ldyBNYXA8c3RyaW5nLCBzdHJpbmc+KCk7XG5cbi8qKiBAZW1vamkg4oyo77iPIExhc3Qtd2lucyBhcHAga2V5YmluZGluZ3MgZm9yIGVucmljaGluZyBjb250ZXh0LW1lbnUgc2hvcnRjdXQgbGFiZWxzIGluIHNjZW5lIGhvc3RzLiAqL1xuY29uc3QgQXBwS2V5YmluZGluZ3NDb250ZXh0ID0gY3JlYXRlQ29udGV4dDxSZWFkb25seU1hcDxzdHJpbmcsIHN0cmluZz4+KEVNUFRZX0tFWVNfQllfQUNUSU9OX0lEKTtcblxuLyoqIEBlbW9qaSDijKjvuI8gUmVzb2x2ZXMgYWN0aW9u4oaSa2V5cyBiaW5kaW5ncyBmcm9tIHRoZSBuZWFyZXN0IHtAbGluayBBcHBLZXliaW5kaW5nc0NvbnRleHR9IHByb3ZpZGVyLiAqL1xuZXhwb3J0IGZ1bmN0aW9uIHVzZUFwcEtleWJpbmRpbmdzQnlBY3Rpb25JZCgpOiBSZWFkb25seU1hcDxzdHJpbmcsIHN0cmluZz4ge1xuICByZXR1cm4gdXNlQ29udGV4dChBcHBLZXliaW5kaW5nc0NvbnRleHQpO1xufVxuXG4vKiogQGVtb2ppIPCflrHvuI8gTWFwcyBwcm9ncmFtIGNvbnRleHQtbWVudSBzcGVjcyB3aXRoIGFwcCBrZXliaW5kaW5nIHNob3J0Y3V0IGVucmljaG1lbnQuICovXG5leHBvcnQgZnVuY3Rpb24gdXNlTWFwQ29udGV4dE1lbnVTcGVjcyhkaXNwYXRjaDogKGFjdGlvbjogc3RyaW5nLCBhcmdzPzogUmVjb3JkPHN0cmluZywgdW5rbm93bj4pID0+IHZvaWQpIHtcbiAgY29uc3Qga2V5c0J5QWN0aW9uSWQgPSB1c2VBcHBLZXliaW5kaW5nc0J5QWN0aW9uSWQoKTtcbiAgcmV0dXJuIHVzZUNhbGxiYWNrKChzcGVjczogcmVhZG9ubHkgQ29udGV4dE1lbnVJdGVtU3BlY1tdKSA9PiBtYXBDb250ZXh0TWVudVNwZWNzKHNwZWNzLCBkaXNwYXRjaCwga2V5c0J5QWN0aW9uSWQpLCBbZGlzcGF0Y2gsIGtleXNCeUFjdGlvbklkXSk7XG59XG5cbi8vI3JlZ2lvbiDwn46l77iPVHV0b3JpYWxPdmVybGF5SG9zdHNcbi8qKiBAZW1vamkg8J+Tpu+4jyBSZXNvbHZlcyBhIGBUdXRvcmlhbEFzc2V0U3JjYCB0byBhIHZhbHVlIHVzYWJsZSBhcyBhbiBgPHZpZGVvPmAvYDxhdWRpbz5gIGBzcmNgIOKAlCBgQmxvYmAgKGFcbiAqIHN0dWRpbyBgQmxvYlN0b3JlYCByZWZlcmVuY2UpIGlzbid0IHJlc29sdmFibGUgZnJvbSB0aGlzIHNjb3BlIChubyBibG9iLXN0b3JlIGJyaWRnZSBoZXJlKSBhbmQgcmV0dXJuc1xuICogYG51bGxgIHdpdGggYSBjb25zb2xlIHdhcm5pbmc7IGBVcmxgL2BEYXRhVXJsYCByZXNvbHZlIGRpcmVjdGx5LiAqL1xuZnVuY3Rpb24gdHV0b3JpYWxBc3NldFNyY1RvVXJsKHNyYzogVHV0b3JpYWxBc3NldFNyYyk6IHN0cmluZyB8IG51bGwge1xuICBpZiAoc3JjLmtpbmQgPT09IFwidXJsXCIpIHJldHVybiBzcmMudXJsO1xuICBpZiAoc3JjLmtpbmQgPT09IFwiZGF0YVVybFwiKSByZXR1cm4gc3JjLmRhdGE7XG4gIGNvbnNvbGUud2FybihcIltERUJVR10gdHV0b3JpYWwgYmxvYiBhc3NldCBzcmMgbm90IHJlc29sdmFibGUgaW4gdGhpcyBzY29wZVwiLCBzcmMuaGFzaCk7XG4gIHJldHVybiBudWxsO1xufVxuXG4vKiogQGVtb2ppIPCfkqzvuI8gU2VsZi1zdWJzY3JpYmVzIHRvIHRoZSB0dXRvcmlhbCBjbG9jayAoc2VlIGB1c2VUdXRvcmlhbENsb2NrYCkgc28gb25seSBUSElTIGxlYWYgcmUtcmVuZGVycyBldmVyeSBmcmFtZSDigJQgbmV2ZXIgdGhlIHdob2xlIHNoZWxsIOKAlCBtaXJyb3JpbmcgYFR1dG9yaWFsQmFyYCdzIG93biBzdWJzY3JpcHRpb24uICovXG5jb25zdCBUdXRvcmlhbENhcHRpb25zSG9zdDogUmVhY3QuRkM8eyByZWFkb25seSB0dXRvcmlhbDogVHV0b3JpYWxEZWZpbml0aW9uOyByZWFkb25seSBjbG9jazogVHV0b3JpYWxDbG9ja1BvcnQ7IHJlYWRvbmx5IGNhcHRpb25zT246IGJvb2xlYW47IHJlYWRvbmx5IHRlcm1pbm9sb2d5OiBzdHJpbmc7IHJlYWRvbmx5IGxvY2FsZTogc3RyaW5nIH0+ID0gKHsgdHV0b3JpYWwsIGNsb2NrLCBjYXB0aW9uc09uLCB0ZXJtaW5vbG9neSwgbG9jYWxlIH0pID0+IHtcbiAgY29uc3QgdGltZU1zID0gdXNlVHV0b3JpYWxDbG9jayhjbG9jayk7XG4gIGNvbnN0IGN1ZSA9IHR1dG9yaWFsQ3Vlc0JldHdlZW4odHV0b3JpYWwudHJhY2tzLm5hcnJhdGlvbiwgdGltZU1zKVswXSA/PyBudWxsO1xuICByZXR1cm4gPFR1dG9yaWFsQ2FwdGlvbnMgdGV4dD17Y3VlID8gcmVzb2x2ZU1hbmlmZXN0TGFiZWwoY3VlLnRleHQsIHRlcm1pbm9sb2d5LCBsb2NhbGUpIDogbnVsbH0gdmlzaWJsZT17Y2FwdGlvbnNPbn0gLz47XG59O1xuXG5jb25zdCBUVVRPUklBTF9ERUZBVUxUX1ZJREVPX1JFQ1QgPSB7IHg6IDAuNzIsIHk6IDAuNywgd2lkdGg6IDAuMjQsIGhlaWdodDogMC4yNCB9IGFzIGNvbnN0O1xuXG4vKiogQGVtb2ppIPCfk7nvuI8gU2VsZi1zdWJzY3JpYmVzIHRvIHRoZSB0dXRvcmlhbCBjbG9jazsgcmVzb2x2ZXMgdGhlIGNvdmVyaW5nIGBUdXRvcmlhbFZpZGVvQ3VlYCAoaWYgYW55KSBhbmQgaXRzIHNvdXJjZS1yZWxhdGl2ZSBsb2NhbCB0aW1lLiAqL1xuY29uc3QgVHV0b3JpYWxWaWRlb092ZXJsYXlIb3N0OiBSZWFjdC5GQzx7IHJlYWRvbmx5IHR1dG9yaWFsOiBUdXRvcmlhbERlZmluaXRpb247IHJlYWRvbmx5IGNsb2NrOiBUdXRvcmlhbENsb2NrUG9ydDsgcmVhZG9ubHkgbXV0ZWQ6IGJvb2xlYW47IHJlYWRvbmx5IHBsYXlpbmc6IGJvb2xlYW47IHJlYWRvbmx5IHJhdGU6IG51bWJlciB9PiA9ICh7XG4gIHR1dG9yaWFsLFxuICBjbG9jayxcbiAgbXV0ZWQsXG4gIHBsYXlpbmcsXG4gIHJhdGUsXG59KSA9PiB7XG4gIGNvbnN0IHRpbWVNcyA9IHVzZVR1dG9yaWFsQ2xvY2soY2xvY2spO1xuICBjb25zdCBjdWU6IFR1dG9yaWFsVmlkZW9DdWUgfCBudWxsID0gdHV0b3JpYWxDdWVzQmV0d2Vlbih0dXRvcmlhbC50cmFja3MudmlkZW8sIHRpbWVNcylbMF0gPz8gbnVsbDtcbiAgY29uc3Qgc3JjID0gY3VlID8gdHV0b3JpYWxBc3NldFNyY1RvVXJsKGN1ZS5zcmMpIDogbnVsbDtcbiAgY29uc3QgbG9jYWxUaW1lTXMgPSBjdWUgPyB0aW1lTXMgLSBjdWUuYXQgKyBjdWUuc291cmNlT2Zmc2V0TXMgOiAwO1xuICByZXR1cm4gPFR1dG9yaWFsVmlkZW9PdmVybGF5IHNyYz17c3JjfSByZWN0PXtjdWU/LnJlY3QgPz8gVFVUT1JJQUxfREVGQVVMVF9WSURFT19SRUNUfSBtdXRlZD17bXV0ZWQgfHwgKGN1ZT8ubXV0ZWQgPz8gZmFsc2UpfSBwbGF5aW5nPXtwbGF5aW5nfSByYXRlPXtyYXRlfSBsb2NhbFRpbWVNcz17bG9jYWxUaW1lTXN9IC8+O1xufTtcblxuLyoqIEBlbW9qaSDwn5G777iPIFNlbGYtc3Vic2NyaWJlcyB0byB0aGUgdHV0b3JpYWwgY2xvY2s7IHJlc29sdmVzIHRoZSBjb3ZlcmluZyBgVHV0b3JpYWxHZXN0dXJlQ3VlYCAoaWYgYW55KSBhbmQgcHJvZ3Jlc3MgKDDigJMxKSB0aHJvdWdoIGl0LCBkcml2aW5nIGBUdXRvcmlhbEdob3N0UG9pbnRlcmAgb2ZmIHRoZSBQTEFZSEVBRCByYXRoZXIgdGhhbiBpdHMgb3duIGludGVybmFsIGNsb2NrICh1bmxpa2UgdGhlIGludHJvZHVjdGlvbiBkZW1vbnN0cmF0aW9uIG92ZXJsYXkpLiAqL1xuY29uc3QgVHV0b3JpYWxHaG9zdFBvaW50ZXJIb3N0OiBSZWFjdC5GQzx7IHJlYWRvbmx5IHR1dG9yaWFsOiBUdXRvcmlhbERlZmluaXRpb247IHJlYWRvbmx5IGNsb2NrOiBUdXRvcmlhbENsb2NrUG9ydCB9PiA9ICh7IHR1dG9yaWFsLCBjbG9jayB9KSA9PiB7XG4gIGNvbnN0IHRpbWVNcyA9IHVzZVR1dG9yaWFsQ2xvY2soY2xvY2spO1xuICBjb25zdCBjdWU6IFR1dG9yaWFsR2VzdHVyZUN1ZSB8IG51bGwgPSB0dXRvcmlhbEN1ZXNCZXR3ZWVuKHR1dG9yaWFsLnRyYWNrcy5nZXN0dXJlcywgdGltZU1zKVswXSA/PyBudWxsO1xuICBjb25zdCBwcm9ncmVzcyA9IGN1ZSA/IE1hdGgubWluKDEsIE1hdGgubWF4KDAsICh0aW1lTXMgLSBjdWUuYXQpIC8gTWF0aC5tYXgoY3VlLmR1cmF0aW9uTXMsIDEpKSkgOiAwO1xuICByZXR1cm4gPFR1dG9yaWFsR2hvc3RQb2ludGVyIGN1ZT17Y3VlfSBwcm9ncmVzcz17cHJvZ3Jlc3N9IC8+O1xufTtcbi8vI2VuZHJlZ2lvbiDwn46l77iPVHV0b3JpYWxPdmVybGF5SG9zdHNcblxuLy8jcmVnaW9uIPCfjqXvuI9UdXRvcmlhbFJlY29yZGVyXG4vKiogQGVtb2ppIOKGlCBGaWVsZC1ieS1maWVsZCBzdHJ1Y3R1cmFsIGRpZmYgb2YgdHdvIGBUdXRvcmlhbFVpU25hcHNob3RgcyBpbnRvIHRoZSBzcGFyc2UgYFR1dG9yaWFsVWlDaGFuZ2VgXG4gKiBhbHBoYWJldCDigJQgdGhlIHJlY29yZGVyJ3MgVUktZGlmZiBlZmZlY3QgY2FsbHMgdGhpcyBldmVyeSBgU2hlbGxTdGF0ZWAgY2hhbmdlIHdoaWxlIGFybWVkLiAqL1xuZnVuY3Rpb24gZGlmZlR1dG9yaWFsVWlTbmFwc2hvdChwcmV2OiBUdXRvcmlhbFVpU25hcHNob3QsIG5leHQ6IFR1dG9yaWFsVWlTbmFwc2hvdCk6IFR1dG9yaWFsVWlDaGFuZ2VbXSB7XG4gIGNvbnN0IGNoYW5nZXM6IFR1dG9yaWFsVWlDaGFuZ2VbXSA9IFtdO1xuICBpZiAocHJldi5hY3RpdmVNb2RlSWQgIT09IG5leHQuYWN0aXZlTW9kZUlkICYmIG5leHQuYWN0aXZlTW9kZUlkICE9IG51bGwpIGNoYW5nZXMucHVzaCh7IGtpbmQ6IFwiYWN0aXZlTW9kZVwiLCBpZDogbmV4dC5hY3RpdmVNb2RlSWQgfSk7XG4gIGlmIChwcmV2LmZvY3VzZWRXaW5kb3dJZCAhPT0gbmV4dC5mb2N1c2VkV2luZG93SWQpIGNoYW5nZXMucHVzaCh7IGtpbmQ6IFwiZm9jdXNlZFdpbmRvd1wiLCBpZDogbmV4dC5mb2N1c2VkV2luZG93SWQgfSk7XG4gIGNvbnN0IHV0aWxpdHlXaW5kb3dJZHMgPSBuZXcgU2V0KFsuLi5PYmplY3Qua2V5cyhwcmV2LmFjdGl2ZVV0aWxpdHlCeVdpbmRvd0lkKSwgLi4uT2JqZWN0LmtleXMobmV4dC5hY3RpdmVVdGlsaXR5QnlXaW5kb3dJZCldKTtcbiAgZm9yIChjb25zdCB3aW5kb3dJZCBvZiB1dGlsaXR5V2luZG93SWRzKSB7XG4gICAgaWYgKHByZXYuYWN0aXZlVXRpbGl0eUJ5V2luZG93SWRbd2luZG93SWRdICE9PSBuZXh0LmFjdGl2ZVV0aWxpdHlCeVdpbmRvd0lkW3dpbmRvd0lkXSkgY2hhbmdlcy5wdXNoKHsga2luZDogXCJhY3RpdmVVdGlsaXR5XCIsIHdpbmRvd0lkLCB1dGlsaXR5SWQ6IG5leHQuYWN0aXZlVXRpbGl0eUJ5V2luZG93SWRbd2luZG93SWRdIH0pO1xuICB9XG4gIGlmIChwcmV2LmFjdGl2ZVRvb2xJZCAhPT0gbmV4dC5hY3RpdmVUb29sSWQpIGNoYW5nZXMucHVzaCh7IGtpbmQ6IFwiYWN0aXZlVG9vbFwiLCBpZDogbmV4dC5hY3RpdmVUb29sSWQgfSk7XG4gIGlmIChuZXh0LmxheW91dCAmJiBKU09OLnN0cmluZ2lmeShwcmV2LmxheW91dCkgIT09IEpTT04uc3RyaW5naWZ5KG5leHQubGF5b3V0KSkgY2hhbmdlcy5wdXNoKHsga2luZDogXCJsYXlvdXRcIiwgbGF5b3V0OiBuZXh0LmxheW91dCB9KTtcbiAgY29uc3QgZ3JvdXBzID0gbmV3IFNldChbLi4uT2JqZWN0LmtleXMocHJldi5hY3RpdmVQYW5lbFRhYkJ5R3JvdXApLCAuLi5PYmplY3Qua2V5cyhuZXh0LmFjdGl2ZVBhbmVsVGFiQnlHcm91cCldKTtcbiAgZm9yIChjb25zdCBncm91cCBvZiBncm91cHMpIHtcbiAgICBpZiAocHJldi5hY3RpdmVQYW5lbFRhYkJ5R3JvdXBbZ3JvdXBdICE9PSBuZXh0LmFjdGl2ZVBhbmVsVGFiQnlHcm91cFtncm91cF0pIGNoYW5nZXMucHVzaCh7IGtpbmQ6IFwicGFuZWxUYWJcIiwgZ3JvdXAsIHRhYklkOiBuZXh0LmFjdGl2ZVBhbmVsVGFiQnlHcm91cFtncm91cF0gfSk7XG4gIH1cbiAgaWYgKG5leHQucGFuZWxKc29uICE9IG51bGwgJiYgcHJldi5wYW5lbEpzb24gIT09IG5leHQucGFuZWxKc29uKSBjaGFuZ2VzLnB1c2goeyBraW5kOiBcInBhbmVsU3RhdGVcIiwgcGFuZWxKc29uOiBuZXh0LnBhbmVsSnNvbiB9KTtcbiAgaWYgKG5leHQuc2VsZWN0aW9uSnNvbiAhPSBudWxsICYmIHByZXYuc2VsZWN0aW9uSnNvbiAhPT0gbmV4dC5zZWxlY3Rpb25Kc29uKSBjaGFuZ2VzLnB1c2goeyBraW5kOiBcInNlbGVjdGlvblwiLCBzZWxlY3Rpb25Kc29uOiBuZXh0LnNlbGVjdGlvbkpzb24gfSk7XG4gIGlmIChwcmV2Lm9wZW5EaWFsb2dJZCAhPT0gbmV4dC5vcGVuRGlhbG9nSWQpIGNoYW5nZXMucHVzaCh7IGtpbmQ6IFwiZGlhbG9nXCIsIGlkOiBuZXh0Lm9wZW5EaWFsb2dJZCB9KTtcbiAgY29uc3QgcHJldlRyZWUgPSBuZXcgU2V0KHByZXYuZXhwYW5kZWRUcmVlSWRzKTtcbiAgY29uc3QgbmV4dFRyZWUgPSBuZXcgU2V0KG5leHQuZXhwYW5kZWRUcmVlSWRzKTtcbiAgZm9yIChjb25zdCBpZCBvZiBuZXh0VHJlZSkgaWYgKCFwcmV2VHJlZS5oYXMoaWQpKSBjaGFuZ2VzLnB1c2goeyBraW5kOiBcInRyZWVFeHBhbnNpb25cIiwgaWQsIGV4cGFuZGVkOiB0cnVlIH0pO1xuICBmb3IgKGNvbnN0IGlkIG9mIHByZXZUcmVlKSBpZiAoIW5leHRUcmVlLmhhcyhpZCkpIGNoYW5nZXMucHVzaCh7IGtpbmQ6IFwidHJlZUV4cGFuc2lvblwiLCBpZCwgZXhwYW5kZWQ6IGZhbHNlIH0pO1xuICBpZiAocHJldi5jb21tYW5kUGFuZWxPcGVuICE9PSBuZXh0LmNvbW1hbmRQYW5lbE9wZW4pIGNoYW5nZXMucHVzaCh7IGtpbmQ6IFwiY29tbWFuZFBhbmVsXCIsIG9wZW46IG5leHQuY29tbWFuZFBhbmVsT3BlbiB9KTtcbiAgcmV0dXJuIGNoYW5nZXM7XG59XG5cbi8qKiBAZW1vamkg8J+Ope+4jyBFcHNpbG9uLWVxdWFsaXR5IGZvciB0d28gY2FtZXJhIHBvc2VzIOKAlCB0aGUgcmVjb3JkZXIncyAxMEh6IGNhbWVyYSBzYW1wbGVyIHNraXBzIHdyaXRpbmcgYVxuICogbmV3IGtleWZyYW1lIHdoZW4gdGhlIGxpdmUgcG9zZSBoYXNuJ3QgbWVhbmluZ2Z1bGx5IG1vdmVkIHNpbmNlIHRoZSBsYXN0IHNhbXBsZS4gKi9cbmZ1bmN0aW9uIHR1dG9yaWFsQ2FtZXJhUG9zZUVxdWFscyhhOiBUdXRvcmlhbENhbWVyYVN0YXRlLCBiOiBUdXRvcmlhbENhbWVyYVN0YXRlKTogYm9vbGVhbiB7XG4gIGlmIChhLmtpbmQgIT09IGIua2luZCkgcmV0dXJuIGZhbHNlO1xuICBpZiAoYS5raW5kID09PSBcIm9yYml0XCIgJiYgYi5raW5kID09PSBcIm9yYml0XCIpIHJldHVybiBhLnBvc2l0aW9uLmV2ZXJ5KCh2YWx1ZSwgaW5kZXgpID0+IE1hdGguYWJzKHZhbHVlIC0gYi5wb3NpdGlvbltpbmRleF0pIDwgMWUtNCkgJiYgYS50YXJnZXQuZXZlcnkoKHZhbHVlLCBpbmRleCkgPT4gTWF0aC5hYnModmFsdWUgLSBiLnRhcmdldFtpbmRleF0pIDwgMWUtNCk7XG4gIGlmIChhLmtpbmQgPT09IFwiY2FudmFzXCIgJiYgYi5raW5kID09PSBcImNhbnZhc1wiKSByZXR1cm4gTWF0aC5hYnMoYS54IC0gYi54KSA8IDFlLTQgJiYgTWF0aC5hYnMoYS55IC0gYi55KSA8IDFlLTQgJiYgTWF0aC5hYnMoYS56b29tIC0gYi56b29tKSA8IDFlLTQ7XG4gIHJldHVybiBmYWxzZTtcbn1cblxuLyoqIEBlbW9qaSDwn46l77iPIENhcHR1cmVzIGEgbGl2ZSBzZXNzaW9uIGludG8gYSBgVHV0b3JpYWxEZWZpbml0aW9uYCDigJQgYSByZWNvcmRpbmcgSVMgYSBgVHV0b3JpYWxEZWZpbml0aW9uYCxcbiAqIHNvIHRoaXMgY2xhc3Mgc2ltcGx5IGFjY3VtdWxhdGVzIGEgZGVuc2VseS1zYW1wbGVkIG9uZSAoc2VlIHRoZSBSdXN0IGNvcmUgZG9jIGNvbW1lbnQgb25cbiAqIGBUdXRvcmlhbERlZmluaXRpb25gKS4gRGVsaWJlcmF0ZWx5IHByb2R1Y2VzIGV2ZW50cy9VSS9jYW1lcmEvZG9jdW1lbnQgdHJhY2tzIG9ubHk6IHdlYmNhbS9taWMgY2FwdHVyZVxuICogKGBNZWRpYVJlY29yZGVyYCkgaXMgYW4gZXhwbGljaXQsIHJlcG9ydGVkIHNjb3BlIGN1dCDigJQgc2VlIHRoZSB0aWNrZXQgY2xvc2Utb3V0IHN1bW1hcnkg4oCUIGEgdGV4dC1vbmx5XG4gKiByZWNvcmRpbmcgaXMgc3RpbGwgYSBmdWxseSB2YWxpZCwgdXNlZnVsIGBUdXRvcmlhbERlZmluaXRpb25gIHBlciB0aGUgUnVzdCBtb2RlbCdzIG93biBvcHRpb25hbGl0eVxuICogKG5hcnJhdGlvbi92aWRlbyB0cmFja3MgZGVmYXVsdCB0byBlbXB0eSkuIERvY3VtZW50IGBFZGl0YCBvcGVyYXRpb25zIGFyZSBOT1QgY2FwdHVyZWQgKHRoYXQgd291bGRcbiAqIHJlcXVpcmUgaW50ZXJjZXB0aW5nIHRoZSBwbHVnaW4ncyBpbnRlcm5hbCB2Y3Mgb3BlcmF0aW9uIHN0cmVhbSBpbiBwZXItb3AgZm9ybSwgd2hpY2ggaXNuJ3QgZXhwb3NlZCB0b1xuICogdGhpcyBzaGVsbCkg4oCUIGFsc28gYSByZXBvcnRlZCBzY29wZSBjdXQ7IFVJL2NhbWVyYS9ldmVudHMgc3RpbGwgcmVwbGF5IGZhaXRoZnVsbHkuICovXG5leHBvcnQgY2xhc3MgVHV0b3JpYWxSZWNvcmRlciB7XG4gIHByaXZhdGUgcmVhZG9ubHkgc3RhcnRlZEF0TXM6IG51bWJlcjtcbiAgcHJpdmF0ZSByZWFkb25seSBiYXNlVWlTbmFwc2hvdDogVHV0b3JpYWxVaVNuYXBzaG90O1xuICBwcml2YXRlIHJlYWRvbmx5IGJhc2VEb2N1bWVudEpzb246IHN0cmluZyB8IG51bGw7XG4gIHByaXZhdGUgcmVhZG9ubHkgZXZlbnRzOiBUdXRvcmlhbEV2ZW50W10gPSBbXTtcbiAgcHJpdmF0ZSByZWFkb25seSB1aUtleWZyYW1lczogeyByZWFkb25seSBhdDogbnVtYmVyOyByZWFkb25seSBzYW1wbGU6IHsgcmVhZG9ubHkga2luZDogXCJzbmFwc2hvdFwiOyByZWFkb25seSBzdGF0ZTogVHV0b3JpYWxVaVNuYXBzaG90IH0gfCB7IHJlYWRvbmx5IGtpbmQ6IFwiZGVsdGFcIjsgcmVhZG9ubHkgY2hhbmdlczogVHV0b3JpYWxVaUNoYW5nZVtdIH0gfVtdID0gW107XG4gIHByaXZhdGUgcmVhZG9ubHkgY2FtZXJhS2V5ZnJhbWVzOiB7IHJlYWRvbmx5IGF0OiBudW1iZXI7IHJlYWRvbmx5IHdpbmRvd0lkOiBzdHJpbmc7IHJlYWRvbmx5IGNhbWVyYTogVHV0b3JpYWxDYW1lcmFTdGF0ZTsgcmVhZG9ubHkgZWFzaW5nOiBcImVhc2VJbk91dFwiIH1bXSA9IFtdO1xuICBwcml2YXRlIHJlYWRvbmx5IGNoYXB0ZXJzOiBUdXRvcmlhbENoYXB0ZXJbXSA9IFtdO1xuICBwcml2YXRlIGxhc3RVaVNuYXBzaG90OiBUdXRvcmlhbFVpU25hcHNob3Q7XG4gIHByaXZhdGUgcmVhZG9ubHkgbGFzdENhbWVyYUJ5V2luZG93ID0gbmV3IE1hcDxzdHJpbmcsIFR1dG9yaWFsQ2FtZXJhU3RhdGU+KCk7XG5cbiAgY29uc3RydWN0b3IoYmFzZVVpU25hcHNob3Q6IFR1dG9yaWFsVWlTbmFwc2hvdCwgYmFzZURvY3VtZW50SnNvbjogc3RyaW5nIHwgbnVsbCkge1xuICAgIHRoaXMuc3RhcnRlZEF0TXMgPSBwZXJmb3JtYW5jZS5ub3coKTtcbiAgICB0aGlzLmJhc2VVaVNuYXBzaG90ID0gYmFzZVVpU25hcHNob3Q7XG4gICAgdGhpcy5sYXN0VWlTbmFwc2hvdCA9IGJhc2VVaVNuYXBzaG90O1xuICAgIHRoaXMuYmFzZURvY3VtZW50SnNvbiA9IGJhc2VEb2N1bWVudEpzb247XG4gIH1cblxuICBwcml2YXRlIG5vd01zKCk6IG51bWJlciB7XG4gICAgcmV0dXJuIE1hdGgubWF4KDAsIE1hdGgucm91bmQocGVyZm9ybWFuY2Uubm93KCkgLSB0aGlzLnN0YXJ0ZWRBdE1zKSk7XG4gIH1cblxuICByZWNvcmRFdmVudChraW5kOiBUdXRvcmlhbEV2ZW50W1wia2luZFwiXSk6IHZvaWQge1xuICAgIHRoaXMuZXZlbnRzLnB1c2goeyBhdDogdGhpcy5ub3dNcygpLCBraW5kIH0pO1xuICB9XG5cbiAgcmVjb3JkVWlEaWZmKG5leHQ6IFR1dG9yaWFsVWlTbmFwc2hvdCk6IHZvaWQge1xuICAgIGNvbnN0IGNoYW5nZXMgPSBkaWZmVHV0b3JpYWxVaVNuYXBzaG90KHRoaXMubGFzdFVpU25hcHNob3QsIG5leHQpO1xuICAgIGlmIChjaGFuZ2VzLmxlbmd0aCA+IDApIHRoaXMudWlLZXlmcmFtZXMucHVzaCh7IGF0OiB0aGlzLm5vd01zKCksIHNhbXBsZTogeyBraW5kOiBcImRlbHRhXCIsIGNoYW5nZXMgfSB9KTtcbiAgICB0aGlzLmxhc3RVaVNuYXBzaG90ID0gbmV4dDtcbiAgfVxuXG4gIHJlY29yZFNuYXBzaG90KHN0YXRlOiBUdXRvcmlhbFVpU25hcHNob3QpOiB2b2lkIHtcbiAgICB0aGlzLnVpS2V5ZnJhbWVzLnB1c2goeyBhdDogdGhpcy5ub3dNcygpLCBzYW1wbGU6IHsga2luZDogXCJzbmFwc2hvdFwiLCBzdGF0ZSB9IH0pO1xuICAgIHRoaXMubGFzdFVpU25hcHNob3QgPSBzdGF0ZTtcbiAgfVxuXG4gIHNhbXBsZUNhbWVyYSh3aW5kb3dJZDogc3RyaW5nLCBjYW1lcmE6IFR1dG9yaWFsQ2FtZXJhU3RhdGUpOiB2b2lkIHtcbiAgICBjb25zdCBwcmV2ID0gdGhpcy5sYXN0Q2FtZXJhQnlXaW5kb3cuZ2V0KHdpbmRvd0lkKTtcbiAgICBpZiAocHJldiAmJiB0dXRvcmlhbENhbWVyYVBvc2VFcXVhbHMocHJldiwgY2FtZXJhKSkgcmV0dXJuO1xuICAgIHRoaXMubGFzdENhbWVyYUJ5V2luZG93LnNldCh3aW5kb3dJZCwgY2FtZXJhKTtcbiAgICB0aGlzLmNhbWVyYUtleWZyYW1lcy5wdXNoKHsgYXQ6IHRoaXMubm93TXMoKSwgd2luZG93SWQsIGNhbWVyYSwgZWFzaW5nOiBcImVhc2VJbk91dFwiIH0pO1xuICB9XG5cbiAgLyoqIPCfk5bvuI8gYHVpLnR1dG9yaWFsLmFkZENoYXB0ZXJgIOKAlCBtYXJrcyB0aGUgY3VycmVudCBlbGFwc2VkIHRpbWUgYXMgYSBzY3J1Yi1iYXIgY2hhcHRlciB3aXRoIGFuXG4gICAqIGF1dG8tbnVtYmVyZWQgdGl0bGUgKG5vIG5hbWluZy1wcm9tcHQgVUkgaW4gdGhpcyBzY29wZTsgYSByZWNvcmRlZCB0dXRvcmlhbCdzIGF1dGhvcmVkIHRpdGxlcyBjYW5cbiAgICogYWx3YXlzIGJlIGhhbmQtZWRpdGVkIGluIHRoZSBkb3dubG9hZGVkIEpTT04gYWZ0ZXJ3YXJkKS4gU3ludGhlc2l6ZXMgYSBgTG9jYWxpemVkTGFiZWxgIG1hdHJpeC4gKi9cbiAgYWRkQ2hhcHRlcih0aXRsZT86IHN0cmluZyB8IExvY2FsaXplZExhYmVsKTogdm9pZCB7XG4gICAgY29uc3QgaW5kZXggPSB0aGlzLmNoYXB0ZXJzLmxlbmd0aCArIDE7XG4gICAgY29uc3QgcmF3VGl0bGUgPSB0aXRsZSA/PyBgQ2hhcHRlciAke2luZGV4fWA7XG4gICAgdGhpcy5jaGFwdGVycy5wdXNoKHsgaWQ6IGBjaGFwdGVyLSR7aW5kZXh9YCwgYXQ6IHRoaXMubm93TXMoKSwgdGl0bGU6IHN5bnRoZXNpemVMb2NhbGl6ZWRMYWJlbChyYXdUaXRsZSkgfSk7XG4gIH1cblxuICBidWlsZChpZDogc3RyaW5nLCB0aXRsZTogc3RyaW5nIHwgTG9jYWxpemVkTGFiZWwsIGV4YW1wbGVJZD86IHN0cmluZyk6IFR1dG9yaWFsRGVmaW5pdGlvbiB7XG4gICAgY29uc3QgZHVyYXRpb25NcyA9IE1hdGgubWF4KDEwMDAsIHRoaXMubm93TXMoKSk7XG4gICAgcmV0dXJuIHtcbiAgICAgIGlkLFxuICAgICAgdGl0bGU6IHN5bnRoZXNpemVMb2NhbGl6ZWRMYWJlbCh0aXRsZSksXG4gICAgICBkdXJhdGlvbk1zLFxuICAgICAgY2hhcHRlcnM6IHRoaXMuY2hhcHRlcnMsXG4gICAgICBiYXNlOiB7IGRvY3VtZW50SnNvbjogdGhpcy5iYXNlRG9jdW1lbnRKc29uID8/IHVuZGVmaW5lZCwgZXhhbXBsZUlkLCB1aTogdGhpcy5iYXNlVWlTbmFwc2hvdCwgY2FtZXJhczogW10gfSxcbiAgICAgIHRyYWNrczogeyBuYXJyYXRpb246IFtdLCB2aWRlbzogW10sIGV2ZW50czogdGhpcy5ldmVudHMsIHVpOiB0aGlzLnVpS2V5ZnJhbWVzLCBkb2N1bWVudDogW10sIGNhbWVyYTogdGhpcy5jYW1lcmFLZXlmcmFtZXMsIGdlc3R1cmVzOiBbXSB9LFxuICAgICAgcmVjb3JkZWRBdDogbmV3IERhdGUoKS50b0lTT1N0cmluZygpLFxuICAgIH07XG4gIH1cbn1cbi8vI2VuZHJlZ2lvbiDwn46l77iPVHV0b3JpYWxSZWNvcmRlclxuXG4vLyNyZWdpb24g8J+Qmu+4j1NoZWxsTW91bnRcbi8qKiBAZW1vamkg8J+Qmu+4jyBQdWJsaWMgcHJvcHMgZm9yIHtAbGluayBGcmFtZXdvcmtPc1NoZWxsfSDigJQgdGhlIG11bHRpLWluc3RhbmNlLXNhZmUgZW50cnkgcG9pbnQuIGBzaGVsbElkYCxcbiAqIGBzdG9yYWdlTmFtZXNwYWNlYCwgYW5kIGBvd25zUGFnZWAgZXhpc3Qgc28gc2V2ZXJhbCBzaGVsbHMgY2FuIGJlIG1vdW50ZWQgb24gb25lIHBhZ2U6IGBvd25zUGFnZWBcbiAqIGdhdGVzIHRoZSBoYW5kZnVsIG9mIGJlaGF2aW9ycyB0aGF0IGFyZSBsZWdpdGltYXRlbHkgcGFnZS1nbG9iYWwgKGRvY3VtZW50IHRpdGxlLCBicm93c2VyIGhpc3RvcnlcbiAqIHN5bmMgdmlhIGBib290RnJhbWV3b3JrT3NgKSwgYHN0b3JhZ2VOYW1lc3BhY2VgIHByZWZpeGVzIHRoaXMgc2hlbGwncyBkdXJhYmxlIHN0b3JhZ2Uga2V5cyBzb1xuICogY28tbW91bnRlZCBzaGVsbHMgZG9uJ3Qgc2hhcmUgYHNlbWlvLm9zLmRvY2tgL2B1aS5jaHJvbWUuKmAgc3RhdGUuICovXG5leHBvcnQgaW50ZXJmYWNlIEZyYW1ld29ya09zU2hlbGxQcm9wcyB7XG4gIHJlYWRvbmx5IHBsdWdpbkZpbHRlcj86IHN0cmluZztcbiAgcmVhZG9ubHkgcGx1Z2luczogcmVhZG9ubHkgeyByZWFkb25seSBwbHVnaW5JZDogc3RyaW5nOyByZWFkb25seSBtb2R1bGVVcmw6IHN0cmluZyB9W107XG4gIHJlYWRvbmx5IGFwcElkPzogc3RyaW5nO1xuICByZWFkb25seSBsb2Nrcz86IFJlc29sdmVkU2hlbGxMb2NrcztcbiAgcmVhZG9ubHkgZGVmYXVsdHM/OiBGcmFtZXdvcmtPc0RlZmF1bHRzO1xuICByZWFkb25seSBicmFuZD86IFNoZWxsQnJhbmQ7XG4gIHJlYWRvbmx5IHNoZWxsSWQ/OiBzdHJpbmc7XG4gIHJlYWRvbmx5IHN0b3JhZ2VOYW1lc3BhY2U/OiBzdHJpbmc7XG4gIHJlYWRvbmx5IG93bnNQYWdlPzogYm9vbGVhbjtcbiAgLyoqIPCfkJrvuI8gU2tpcHMgdGhlIGJyYW5kL2FwcCBpbnRyb2R1Y3Rpb24gYXV0by1zdGFydCAoYW5kIGFueSBicmFuZC1vd25lZCB0dXRvcmlhbCdzIG93biBhdXRvLWNvbnNpZGVyZWRcbiAgICogcmV2ZWFsKSBmb3IgYSBzaGVsbCB0aGF0J3MgbW91bnRlZCBidXQgbm90IHRoZSBvbmUgdGhlIHVzZXIgaXMgYWN0dWFsbHkgbG9va2luZyBhdCDigJQgYSBsaXZlXG4gICAqIG11bHRpLXNoZWxsIHBhZ2UgKGUuZy4gdGhlIG1pdC1iZXN0YW5kIGRlbW9uc3RyYXRvcidzIGJhY2tncm91bmQgcGFuZXMpIGhhcyBubyBpZnJhbWUgYm91bmRhcnkgZm9yXG4gICAqIHRoZSBleGlzdGluZyBgd2luZG93LnNlbGYgIT09IHdpbmRvdy50b3BgIGhldXJpc3RpYyBiZWxvdyB0byBrZXkgb2ZmLCBzbyBzZXZlcmFsIHNoZWxscyB3b3VsZFxuICAgKiBvdGhlcndpc2UgYWxsIGF1dG8tcGxheSB0aGVpciBvbmJvYXJkaW5nIGF0IG9uY2UgdGhlIG1vbWVudCB0aGV5IGJvb3QuIERlZmF1bHRzIHRvIGBmYWxzZWAgKGV4aXN0aW5nXG4gICAqIHNpbmdsZS1zaGVsbC1wZXItcGFnZSBiZWhhdmlvciB1bmNoYW5nZWQpLiAqL1xuICByZWFkb25seSBzdXBwcmVzc0F1dG9JbnRyb2R1Y3Rpb24/OiBib29sZWFuO1xufVxuXG4vKiogQGVtb2ppIPCfkJrvuI8gUmVzb2x2ZXMgdGhlIHtAbGluayBTaGVsbFNjb3BlLnN0b3JhZ2V9IHBvcnQgZm9yIGEgc2hlbGwgbW91bnQ6IGVwaGVtZXJhbCBicmFuZHMgYWx3YXlzIGdldFxuICogYW4gaW4tbWVtb3J5IHBvcnQgKG5ldmVyIGR1cmFibGUsIHJlZ2FyZGxlc3Mgb2YgbmFtZXNwYWNlKTsgYSBuYW1lc3BhY2VkIG5vbi1lcGhlbWVyYWwgc2hlbGwgZ2V0cyBhXG4gKiBzY29wZWQgdmlldyBvdmVyIGJyb3dzZXIgc3RvcmFnZTsgYSBiYXJlIG5vbi1lcGhlbWVyYWwgc2hlbGwgKHRoZSBoaXN0b3JpY2FsIHNpbmdsZS1hcHAtcGVyLXBhZ2VcbiAqIGNhc2UpIGdldHMgdGhlIHBsYWluIHNoYXJlZCBicm93c2VyIHBvcnQuICovXG5mdW5jdGlvbiByZXNvbHZlU2hlbGxTY29wZVN0b3JhZ2UoZXBoZW1lcmFsOiBib29sZWFuLCBzdG9yYWdlTmFtZXNwYWNlOiBzdHJpbmcgfCB1bmRlZmluZWQpOiBTdG9yYWdlUG9ydCB7XG4gIGlmIChlcGhlbWVyYWwpIHJldHVybiBjcmVhdGVNZW1vcnlTdG9yYWdlUG9ydCgpO1xuICBjb25zdCBicm93c2VyID0gY3JlYXRlQnJvd3NlclN0b3JhZ2VQb3J0KCk7XG4gIHJldHVybiBzdG9yYWdlTmFtZXNwYWNlID8gY3JlYXRlU2NvcGVkU3RvcmFnZVBvcnQoYnJvd3Nlciwgc3RvcmFnZU5hbWVzcGFjZSkgOiBicm93c2VyO1xufVxuXG4vKiogQGVtb2ppIPCfkJrvuI8gTW91bnRzIGEgYC5zZW1pby1zY29wZWAgcm9vdCAodGhlbWUvYXBwZWFyYW5jZS9pZCBzY29waW5nIGxhbmRzIHdpdGggbGF0ZXIgd2F2ZXMpIGNhcnJ5aW5nIGFcbiAqIHtAbGluayBTaGVsbFNjb3BlfSDigJQgdGhlIHNlYW0gdGhhdCBsZXRzIHNldmVyYWwgb2YgdGhlc2UgY29leGlzdCBvbiBvbmUgcGFnZSDigJQgYXJvdW5kIHRoZSBhY3R1YWwgc2hlbGxcbiAqIGltcGxlbWVudGF0aW9uIGluIHtAbGluayBGcmFtZXdvcmtPc1NoZWxsSW5uZXJ9LiAqL1xuZXhwb3J0IGZ1bmN0aW9uIEZyYW1ld29ya09zU2hlbGwocHJvcHM6IEZyYW1ld29ya09zU2hlbGxQcm9wcyk6IFJlYWN0LlJlYWN0RWxlbWVudCB7XG4gIGNvbnN0IHsgc2hlbGxJZCwgc3RvcmFnZU5hbWVzcGFjZSwgb3duc1BhZ2UgPSBmYWxzZSwgYnJhbmQsIGxvY2tzLCAuLi5pbm5lclByb3BzIH0gPSBwcm9wcztcbiAgY29uc3QgZXBoZW1lcmFsID0gaXNFcGhlbWVyYWxTaGVsbEJyYW5kKGJyYW5kKTtcbiAgY29uc3QgW3Njb3BlXSA9IHVzZVN0YXRlPFNoZWxsU2NvcGU+KCgpID0+IHtcbiAgICBjb25zdCBzdG9yYWdlID0gcmVzb2x2ZVNoZWxsU2NvcGVTdG9yYWdlKGVwaGVtZXJhbCwgc3RvcmFnZU5hbWVzcGFjZSk7XG4gICAgLy8g8J+Qmu+4jyBSZXNvbHZlZCBzeW5jaHJvbm91c2x5IChub3QgaW4gYSBgdXNlRWZmZWN0YCkgc28gYW4gZW1iZWRkZWQgc2hlbGwgbmV2ZXIgZmxhc2hlcyB0aGUgd3JvbmdcbiAgICAvLyBsb2NhbGUncyBjaHJvbWUgb24gaXRzIGZpcnN0IHBhaW50LCBtaXJyb3JpbmcgYGluaXRVaUxvY2FsZVN5bmNgJ3MgcmVhc29uaW5nIGZvciB0aGUgcGFnZS1vd25pbmdcbiAgICAvLyBjYXNlLiBgbG9ja3MubG9jYWxlYCBhbmQgYW55IHByZXZpb3VzbHktc3RvcmVkIHByZWZlcmVuY2UgY292ZXIgdGhlIGNvbW1vbiBjYXNlczsgYSBicmFuZCdzIG93blxuICAgIC8vIGBkZWZhdWx0cy5sb2NhbGVgIChub3QgYXZhaWxhYmxlIHlldCBoZXJlKSBzdGlsbCBsYW5kcyBtb21lbnRzIGxhdGVyIHZpYSB0aGUgdWlQcmVmcyBlZmZlY3QgYmVsb3cuXG4gICAgY29uc3QgaW5pdGlhbExvY2FsZSA9IGxvY2tzPy5sb2NhbGUgPz8gcmVhZFN0b3JlZFVpQ2hyb21lTG9jYWxlKHN0b3JhZ2UpID8/IGRldGVjdFNoZWxsTG9jYWxlKHR5cGVvZiBuYXZpZ2F0b3IgIT09IFwidW5kZWZpbmVkXCIgPyBuYXZpZ2F0b3IubGFuZ3VhZ2UgOiB1bmRlZmluZWQpO1xuICAgIHJldHVybiBjcmVhdGVTaGVsbFNjb3BlKHsgc2hlbGxJZCwgb3duc1BhZ2UsIHN0b3JhZ2UsIGluaXRpYWxMb2NhbGUgfSk7XG4gIH0pO1xuICAvLyDwn5Ca77iPIGBzY29wZS5yb290UmVmYCBpcyBhIHN0YWJsZSBvYmplY3QgKGl0cyBpZGVudGl0eSBuZXZlciBjaGFuZ2VzKSwgc28gYSBkZXNjZW5kYW50IGhvb2sgdGhhdCBwdXRzXG4gIC8vIHRoZSBSRUYgSVRTRUxGIGluIGEgYHVzZUVmZmVjdGAvYHVzZUxheW91dEVmZmVjdGAgZGVwZW5kZW5jeSBhcnJheSB3b3VsZCBuZXZlciByZS1maXJlIG9uY2UgdGhlIHJlZlxuICAvLyBhdHRhY2hlcy4gVGhpcyBzdGF0ZSBidW1wIGZvcmNlcyBvbmUgZ3VhcmFudGVlZCByZS1yZW5kZXIgcmlnaHQgYWZ0ZXIgYXR0YWNobWVudCBzbyBkZXNjZW5kYW50cyB0aGF0XG4gIC8vIHJlYWQgYHNjb3BlLnJvb3RSZWYuY3VycmVudGAgZnJlc2ggYXQgcmVuZGVyIHRpbWUgKHNlZSBgRnJhbWV3b3JrT3NTaGVsbElubmVyYCdzXG4gIC8vIGB1c2VFbGVtZW50c1N1cmZhY2VDaHJvbWVgL2B1c2VDYW52YXNBcHBlYXJhbmNlU3luY2AgY2FsbHMpIHBpY2sgdXAgdGhlIHJlYWwgZWxlbWVudCBpbnN0ZWFkIG9mXG4gIC8vIHN0aWNraW5nIHdpdGggd2hhdGV2ZXIgdGhleSBzYXcgKHVzdWFsbHkgYG51bGxgKSBvbiB0aGUgdmVyeSBmaXJzdCByZW5kZXIuXG4gIGNvbnN0IFssIGJ1bXBBZnRlclJvb3RBdHRhY2hdID0gdXNlU3RhdGUoMCk7XG4gIGNvbnN0IHNldFJvb3QgPSB1c2VDYWxsYmFjaygobm9kZTogSFRNTERpdkVsZW1lbnQgfCBudWxsKSA9PiB7XG4gICAgc2NvcGUucm9vdFJlZi5jdXJyZW50ID0gbm9kZTtcbiAgICBidW1wQWZ0ZXJSb290QXR0YWNoKChuKSA9PiBuICsgMSk7XG4gIH0sIFtzY29wZV0pO1xuICBjb25zdCBzZXRQb3J0YWxMYXllciA9IHVzZUNhbGxiYWNrKChub2RlOiBIVE1MRGl2RWxlbWVudCB8IG51bGwpID0+IHtcbiAgICBzY29wZS5wb3J0YWxMYXllclJlZi5jdXJyZW50ID0gbm9kZTtcbiAgfSwgW3Njb3BlXSk7XG4gIHVzZUVmZmVjdCgoKSA9PiAoKSA9PiBkaXNwb3NlU2hlbGxJMThuSW5zdGFuY2Uoc2NvcGUuaTE4biksIFtzY29wZV0pO1xuICByZXR1cm4gKFxuICAgIDxkaXYgcmVmPXtzZXRSb290fSBjbGFzc05hbWU9XCJzZW1pby1zY29wZVwiIGRhdGEtc2hlbGwtaWQ9e3Njb3BlLnNoZWxsSWR9IHN0eWxlPXt7IGhlaWdodDogXCIxMDAlXCIsIHdpZHRoOiBcIjEwMCVcIiwgaXNvbGF0aW9uOiBcImlzb2xhdGVcIiB9fT5cbiAgICAgIDxTaGVsbFNjb3BlUHJvdmlkZXIgc2NvcGU9e3Njb3BlfT5cbiAgICAgICAgPEZyYW1ld29ya09zU2hlbGxJbm5lciB7Li4uaW5uZXJQcm9wc30gbG9ja3M9e2xvY2tzfSBicmFuZD17YnJhbmR9IC8+XG4gICAgICAgIDxkaXYgZGF0YS1zZW1pby1wb3J0YWwtbGF5ZXIgcmVmPXtzZXRQb3J0YWxMYXllcn0gLz5cbiAgICAgIDwvU2hlbGxTY29wZVByb3ZpZGVyPlxuICAgIDwvZGl2PlxuICApO1xufVxuLy8jZW5kcmVnaW9uIPCfkJrvuI9TaGVsbE1vdW50XG5cbmZ1bmN0aW9uIEZyYW1ld29ya09zU2hlbGxJbm5lcih7XG4gIHBsdWdpbkZpbHRlcixcbiAgcGx1Z2lucyxcbiAgYXBwSWQsXG4gIGxvY2tzOiBsb2Nrc1Byb3AsXG4gIGRlZmF1bHRzOiBkZWZhdWx0c1Byb3AsXG4gIGJyYW5kLFxuICBzdXBwcmVzc0F1dG9JbnRyb2R1Y3Rpb24gPSBmYWxzZSxcbn06IHtcbiAgcmVhZG9ubHkgcGx1Z2luRmlsdGVyPzogc3RyaW5nO1xuICByZWFkb25seSBwbHVnaW5zOiByZWFkb25seSB7IHJlYWRvbmx5IHBsdWdpbklkOiBzdHJpbmc7IHJlYWRvbmx5IG1vZHVsZVVybDogc3RyaW5nIH1bXTtcbiAgcmVhZG9ubHkgYXBwSWQ/OiBzdHJpbmc7XG4gIHJlYWRvbmx5IGxvY2tzPzogUmVzb2x2ZWRTaGVsbExvY2tzO1xuICByZWFkb25seSBkZWZhdWx0cz86IEZyYW1ld29ya09zRGVmYXVsdHM7XG4gIHJlYWRvbmx5IGJyYW5kPzogU2hlbGxCcmFuZDtcbiAgcmVhZG9ubHkgc3VwcHJlc3NBdXRvSW50cm9kdWN0aW9uPzogYm9vbGVhbjtcbn0pIHtcbiAgY29uc3Qgc2NvcGUgPSB1c2VTaGVsbFNjb3BlKCk7XG4gIGNvbnN0IHNoZWxsQ29udGV4dE1lbnVUaXRsZUxhYmVsID0gdXNlTGFiZWwoXCJ1aS5zdXJmYWNlQ29udGV4dE1lbnUud29ya3NwYWNlXCIpO1xuICAvLyDwn4+g77iP8J+ns++4jyBgaG9zdENvbmZpZ2AgaXMgdGhlIHNvbGUgcGllY2Ugb2YgcGVyLXBsdWdpbiBpZGVudGl0eSBrbm93bGVkZ2UgdGhlIHNoZWxsIG5lZWRzICh3aGljaCBhcHAgaWQgaXNcbiAgLy8gXCJsYW5kaW5nXCIsIHdoaWNoIGlzIFwiaG9zdFwiKSDigJQgZXZlcnkgY29udHJvbGxlciBpZCAvIGRlZmF1bHQgcGFuZWwgdGFiIGRlcml2ZXMgZnJvbSB0aGUgKmxvYWRlZCpcbiAgLy8gbWFuaWZlc3QncyBvd24gYGNvbnRyb2xsZXJJZGAvYHBhbmVsVGFic2Agb24gdGhvc2UgYXBwcyBiZWxvdywgbmV2ZXIgZnJvbSBhIHNlcGFyYXRlIGxpdGVyYWwuXG4gIGNvbnN0IGhvc3RDb25maWcgPSBwbHVnaW5GaWx0ZXIgPyByZXNvbHZlUGx1Z2luSG9zdENvbmZpZyhwbHVnaW5GaWx0ZXIpIDogdW5kZWZpbmVkO1xuICBjb25zdCBzdHVkaW9Nb2RlID0gaG9zdENvbmZpZyAhPT0gdW5kZWZpbmVkO1xuICBjb25zdCBtb2JpbGUgPSB1c2VNZWRpYVF1ZXJ5KFVJX01PQklMRV9NRURJQV9RVUVSWSk7XG4gIGNvbnN0IGxvY2tzID0gbG9ja3NQcm9wID8/IEVNUFRZX1NIRUxMX0xPQ0tTO1xuICBjb25zdCBkZWZhdWx0cyA9IGRlZmF1bHRzUHJvcCA/PyBFTVBUWV9TSEVMTF9ERUZBVUxUUztcbiAgY29uc3QgZXBoZW1lcmFsID0gaXNFcGhlbWVyYWxTaGVsbEJyYW5kKGJyYW5kKTtcbiAgY29uc3QgW3NoZWxsU3RhdGUsIGRpc3BhdGNoXSA9IHVzZVJlZHVjZXIoc2hlbGxSZWR1Y2VyLCB1bmRlZmluZWQsICgpID0+IGluaXRpYWxTaGVsbFN0YXRlKHsgcGx1Z2luRmlsdGVyLCBwbHVnaW5zLCBsb2NrcywgZGVmYXVsdHMsIHN0b3JhZ2U6IHNjb3BlLnN0b3JhZ2UgfSkpO1xuICBjb25zdCB7IGxvYWRlZFBsdWdpbnMsIHBsdWdpblN0YXR1c0J5SWQsIHBsdWdpblN1cGVydmlzb3JCeUlkLCBzZXNzaW9uLCBlcnJvciB9ID0gc2hlbGxTdGF0ZS5wbHVnaW5SdW50aW1lO1xuICBjb25zdCBob3N0UGx1Z2luID0gdXNlTWVtbygoKSA9PiAoaG9zdENvbmZpZyA/IGxvYWRlZFBsdWdpbnMuZmluZCgoZW50cnkpID0+IGVudHJ5LmhhbmRsZS5wbHVnaW5JZCA9PT0gaG9zdENvbmZpZy5wbHVnaW5JZCkgOiB1bmRlZmluZWQpLCBbbG9hZGVkUGx1Z2lucywgaG9zdENvbmZpZ10pO1xuICBjb25zdCBob3N0QXBwID0gdXNlTWVtbygoKSA9PiBob3N0UGx1Z2luPy5tYW5pZmVzdC5hcHBzLmZpbmQoKGFwcCkgPT4gYXBwLmlkID09PSBob3N0Q29uZmlnPy5ob3N0QXBwSWQpLCBbaG9zdFBsdWdpbiwgaG9zdENvbmZpZ10pO1xuICBjb25zdCBsYW5kaW5nQXBwID0gdXNlTWVtbygoKSA9PiBob3N0UGx1Z2luPy5tYW5pZmVzdC5hcHBzLmZpbmQoKGFwcCkgPT4gYXBwLmlkID09PSBob3N0Q29uZmlnPy5sYW5kaW5nQXBwSWQpID8/IGhvc3RQbHVnaW4/Lm1hbmlmZXN0LmFwcHNbMF0sIFtob3N0UGx1Z2luLCBob3N0Q29uZmlnXSk7XG4gIGNvbnN0IGxhbmRpbmdBcHBJZCA9IGhvc3RDb25maWc/LmxhbmRpbmdBcHBJZDtcbiAgY29uc3QgaG9zdEFwcElkID0gaG9zdENvbmZpZz8uaG9zdEFwcElkO1xuICBjb25zdCBob3N0Q29udHJvbGxlcklkID0gaG9zdEFwcD8uY29udHJvbGxlcklkO1xuICBjb25zdCBsYW5kaW5nQ29udHJvbGxlcklkID0gbGFuZGluZ0FwcD8uY29udHJvbGxlcklkO1xuICBjb25zdCBob3N0Q2F0YWxvZ3VlVGFiSWQgPSBob3N0QXBwPy5wYW5lbFRhYnNbMF0gPyBwYW5lbFRhYktpbmRJZChob3N0QXBwLnBhbmVsVGFic1swXS5raW5kKSA6IHVuZGVmaW5lZDtcbiAgY29uc3QgeyB3aW5kb3dVaUJ5V2luZG93SWQsIHdpbmRvd0VuZ2FnZW1lbnRzQnlXaW5kb3dJZCwgd2luZG93TWVhc3VyZXNCeVdpbmRvd0lkLCB0b29sTWVhc3VyZXNCeVRvb2xJZCwgcGFuZWxVaUJ5S2V5LCBhcHBMYWJlbHNPdmVybGF5IH0gPSBzaGVsbFN0YXRlLndpbmRvd1VpO1xuICBjb25zdCB7IHNwYXduZWRXaW5kb3dVaSwgc3Bhd25lZFdpbmRvd0VuZ2FnZW1lbnRzLCBzcGF3bmVkV2luZG93TWVhc3VyZXMgfSA9IHNoZWxsU3RhdGUuc3Bhd25lZFdpbmRvdztcbiAgY29uc3QgeyBmb2xkZWRCeVdpbmRvd0lkOiBhY3Rpb25QYW5lRm9sZGVkQnlXaW5kb3dJZCwgZXhwYW5kZWRCeVdpbmRvd0lkOiBhY3Rpb25QYW5lRXhwYW5kZWRCeVdpbmRvd0lkLCBzdGFnZWRBcmdzQnlLZXk6IGFjdGlvblBhbmVTdGFnZWRBcmdzQnlLZXksIGFjdGl2ZVV0aWxpdHlCeVdpbmRvd0lkLCBhY3RpdmVUb29sSWQgfSA9IHNoZWxsU3RhdGUuYWN0aW9uUGFuZTtcbiAgY29uc3QgeyBleHBhbmRlZENvbW1hbmRJZCwgc3RhZ2VkQXJnc0J5Q29tbWFuZElkOiBjb21tYW5kU3RhZ2VkQXJnc0J5Q29tbWFuZElkIH0gPSBzaGVsbFN0YXRlLmNvbW1hbmRQYW5lbDtcbiAgY29uc3QgeyBwYW5lbHMsIGRvY2tPdmVycmlkZSwgcGFuZWxQYXRoTWVtb3J5LCB0cmVlT3BlblN0YXRlcywgYWN0aXZlV2luZG93SWQsIHNoZWxsTGF5b3V0LCBhY3RpdmVFeGFtcGxlSWQsIG1vYmlsZVBhbmVsUGF0aCwgbW9iaWxlUGFuZWxWaXNpYmxlLCBleHRyYVdpbmRvd0luc3RhbmNlcywgd2luZG93VGl0bGVzQnlJZCwgd2luZG93SWNvbnNCeUlkIH0gPSBzaGVsbFN0YXRlLmxheW91dDtcbiAgY29uc3QgeyBzZWFyY2hPcGVuLCBmaW5kT3BlbiwgaW50cm9kdWN0aW9uU3RlcEluZGV4LCBpbnRyb2R1Y3Rpb25Db21wbGV0ZWRJbnRlcmFjdGlvbnMsIGRpYWxvZzogb3ZlcmxheURpYWxvZyB9ID0gc2hlbGxTdGF0ZS5vdmVybGF5cztcbiAgY29uc3QgeyBhY3RpdmVUdXRvcmlhbElkLCBwbGF5aW5nOiB0dXRvcmlhbFBsYXlpbmcsIHJhdGU6IHR1dG9yaWFsUmF0ZSwgbXV0ZWQ6IHR1dG9yaWFsTXV0ZWQsIGNhcHRpb25zT246IHR1dG9yaWFsQ2FwdGlvbnNPbiwgcmVjb3JkaW5nOiB0dXRvcmlhbFJlY29yZGluZywgZGV2aWF0ZWQ6IHR1dG9yaWFsRGV2aWF0ZWQgfSA9IHNoZWxsU3RhdGUudHV0b3JpYWw7XG4gIGNvbnN0IHsgdWlBcHBlYXJhbmNlLCB1aUxheW91dCwgdWlEcml2ZXJJZCwgdWlDdXN0b21Ecml2ZXJzLCB1aURyaXZlckRyYWZ0LCB1aUxvY2FsZSwgdWlUZXJtaW5vbG9neSwgdWlUaGVtZUlkLCB1aUN1c3RvbVRoZW1lcywgdWlUaGVtZURyYWZ0LCB1aUtleWJpbmRpbmdPdmVycmlkZXMgfSA9IHNoZWxsU3RhdGUudWlQcmVmcztcbiAgY29uc3QgeyBzeW5jQmFja2JvbmVVcmksIHN5bmNDYXJkS2luZCwgc3luY0RyYWZ0UGF0aCwgc3luY1N0YXR1c0J5RG9jdW1lbnRJZCB9ID0gc2hlbGxTdGF0ZS5zeW5jO1xuICBjb25zdCBpbXBvcnRTcGFjZUlucHV0UmVmID0gdXNlUmVmPEhUTUxJbnB1dEVsZW1lbnQ+KG51bGwpO1xuICBjb25zdCByZWZyZXNoR2VuZXJhdGlvblJlZiA9IHVzZVJlZigwKTtcbiAgY29uc3QgY29udHJpYnV0aW9uc0pzb25SZWYgPSB1c2VSZWY8c3RyaW5nIHwgbnVsbD4obnVsbCk7XG4gIGNvbnN0IGFwcFJlZ2lzdHJhdGlvbnNKc29uUmVmID0gdXNlUmVmPHN0cmluZyB8IG51bGw+KG51bGwpO1xuICBjb25zdCBzcGF3bmVkUmVmcmVzaEdlbmVyYXRpb25SZWYgPSB1c2VSZWYoMCk7XG4gIGNvbnN0IGNvbnRyaWJ1dG9ySW5zdGFuY2VzUmVmID0gdXNlUmVmPE1hcDxzdHJpbmcsIG51bWJlcj4+KG5ldyBNYXAoKSk7XG4gIGNvbnN0IGxheW91dFNlZWRLZXlSZWYgPSB1c2VSZWY8c3RyaW5nIHwgbnVsbD4obnVsbCk7XG4gIGNvbnN0IG5vRXhhbXBsZVJlc2V0SW5zdGFuY2VJZFJlZiA9IHVzZVJlZjxudW1iZXIgfCBudWxsPihudWxsKTtcbiAgY29uc3QgZXh0cmFXaW5kb3dDb3VudGVyUmVmID0gdXNlUmVmKDApO1xuICAvLyDwn5ax77iPIFNoZWxsLWxldmVsIGNvbnRleHQtbWVudSBmYWxsYmFjazogb3BlbnMgZm9yIGFueSByaWdodC1jbGljayB0aGUgc2hlbGwgaGFzbid0IGFscmVhZHkgY2xhaW1lZFxuICAvLyAoZXZlcnkgZXhpc3RpbmcgcGVyLXN1cmZhY2UgYG9uQ29udGV4dE1lbnVgIG5vdyBjYWxscyBgc3RvcFByb3BhZ2F0aW9uKClgIG9uY2UgaXQgZGVjaWRlcyB0byBzaG93XG4gIC8vIGl0cyBvd24gbWVudSDigJQgc2VlIHRoZSBg8J+Wse+4j1NoZWxsQ29udGV4dE1lbnVgIHJlZ2lvbiBiZWxvdykuIENvdmVycyB3aW5kb3ctbGV2ZWwgZGVjbGFyZWQgYWN0aW9uc1xuICAvLyBwbHVzIHRoZSBPUyBjb21tYW5kIHBhbGV0dGUsIHNvIGV2ZXJ5IHdpbmRvdy9iYWNrZ3JvdW5kIGFsd2F5cyBzaG93cyAqc29tZXRoaW5nKi5cbiAgY29uc3QgW3NoZWxsQ29udGV4dE1lbnUsIHNldFNoZWxsQ29udGV4dE1lbnVdID0gdXNlU3RhdGU8eyByZWFkb25seSB4OiBudW1iZXI7IHJlYWRvbmx5IHk6IG51bWJlcjsgcmVhZG9ubHkgaXRlbXM6IHJlYWRvbmx5IENvbnRleHRNZW51SXRlbVtdIH0gfCBudWxsPihudWxsKTtcbiAgLy8g8J+qn++4jyBMaXZlIGV4dHJhLXdpbmRvdyBsaXN0LCB1cGRhdGVkIHN5bmNocm9ub3VzbHkgb24gZXZlcnkgc2VlZC9zcGxpdC9kcm9wIOKAlCBgcmVmcmVzaFVpYCByZWFkcyB0aGlzXG4gIC8vIGluc3RlYWQgb2YgdGhlIHJlbmRlci1jbG9zdXJlIGBleHRyYVdpbmRvd0luc3RhbmNlc2Agc28gYSBjb25jdXJyZW50IGFjdGlvbiByZWZyZXNoIChlLmcuIGJvb3RcbiAgLy8gYHNldEFjdGl2ZUV4YW1wbGVgKSB0aGF0IHN0YXJ0cyBhZnRlciB0aGUgc2Vzc2lvbi1zd2l0Y2ggcmVmcmVzaCB3cm90ZSBleHRyYXMgYnV0IGJlZm9yZSBSZWFjdFxuICAvLyByZS1yZW5kZXJlZCBjYW5ub3QgZmV0Y2ggd2l0aCBgW11gIGFuZCB3aXBlIFRvcC9QZXJzcGVjdGl2ZSBib2RpZXMgdG8gXCJtaXNzaW5nIHdpbmRvd1wiLlxuICBjb25zdCBleHRyYVdpbmRvd0luc3RhbmNlc1JlZiA9IHVzZVJlZjxyZWFkb25seSBFeHRyYVdpbmRvd0luc3RhbmNlW10+KFtdKTtcbiAgZXh0cmFXaW5kb3dJbnN0YW5jZXNSZWYuY3VycmVudCA9IGV4dHJhV2luZG93SW5zdGFuY2VzO1xuICBjb25zdCBzZXRXaW5kb3dUaXRsZSA9IHVzZUNhbGxiYWNrKCh3aW5kb3dJZDogc3RyaW5nLCB0aXRsZTogc3RyaW5nKSA9PiB7XG4gICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9XSU5ET1dfVElUTEVcIiwgd2luZG93SWQsIHRpdGxlIH0pO1xuICB9LCBbXSk7XG4gIGNvbnN0IHNldFdpbmRvd0ljb24gPSB1c2VDYWxsYmFjaygod2luZG93SWQ6IHN0cmluZywgaWNvbklkOiBJY29uTmFtZSkgPT4ge1xuICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfV0lORE9XX0lDT05cIiwgd2luZG93SWQsIGljb25JZCB9KTtcbiAgfSwgW10pO1xuICAvLyDwn5Ci77iPIFBlci1pbnN0YW5jZSBjb250ZW50LWhhc2ggY2FjaGUgZm9yIHRoZSBiYXRjaGVkIGByZWZyZXNoLXVpYCBjYWxsLCBrZXllZCBieSB0aGUgc2FtZVxuICAvLyBgcGx1Z2luSWQ6YXBwSWQ6aW5zdGFuY2VJZGAgdHJpcGxlIGFzIGBsYXlvdXRTZWVkS2V5UmVmYCDigJQgY2xlYXJlZCBvbiBzZXNzaW9uIHN3aXRjaCBiZWxvdy5cbiAgY29uc3QgdWlSZWZyZXNoQ2FjaGVSZWYgPSB1c2VSZWY8VWlSZWZyZXNoQ2FjaGU+KG5ldyBNYXAoKSk7XG4gIC8vIPCfkKLvuI8gU2FtZSBpZGVhIGZvciB0aGUgc3R1ZGlvLW1vZGUgc3Bhd25lZC1pbnN0YW5jZSB2aWV3LCBrZXllZCBieSBzcGF3bmVkIGluc3RhbmNlSWQg4oCUIGNsZWFyZWQgd2hlblxuICAvLyB0aGUgc3Bhd25lZCBpbnN0YW5jZSBpdHNlbGYgY2hhbmdlcyAodHJhY2tlZCB2aWEgYHNwYXduZWRMYXlvdXRTZWVkUmVmYCkuXG4gIGNvbnN0IHNwYXduZWRVaVJlZnJlc2hDYWNoZVJlZiA9IHVzZVJlZjxVaVJlZnJlc2hDYWNoZT4obmV3IE1hcCgpKTtcbiAgY29uc3Qgc3Bhd25lZExheW91dFNlZWRSZWYgPSB1c2VSZWY8c3RyaW5nIHwgbnVsbD4obnVsbCk7XG4gIGNvbnN0IG9wZW5TcGFjZUlkUmVmID0gdXNlUmVmPHN0cmluZyB8IG51bGw+KG51bGwpO1xuICBjb25zdCBvcGVuSW5zdGFuY2VJZFJlZiA9IHVzZVJlZjxzdHJpbmcgfCBudWxsPihudWxsKTtcbiAgY29uc3Qgc2Vzc2lvblJlZiA9IHVzZVJlZjxBY3RpdmVTZXNzaW9uIHwgbnVsbD4obnVsbCk7XG4gIGNvbnN0IHVpRGV2aWNlOiBFbGVtZW50c1N1cmZhY2VEZXZpY2UgPSBtb2JpbGUgPyBcIm1vYmlsZVwiIDogdWlMYXlvdXQ7XG4gIGNvbnN0IHVpVGhlbWU6IFVpVGhlbWUgPSB1c2VNZW1vKCgpID0+IHtcbiAgICBpZiAodWlUaGVtZURyYWZ0KSByZXR1cm4gdWlUaGVtZURyYWZ0O1xuICAgIGNvbnN0IGZvdW5kID0gYnVpbHRpblVpVGhlbWVzKCkuZmluZCgodCkgPT4gdC5pZCA9PT0gdWlUaGVtZUlkKSA/PyB1aUN1c3RvbVRoZW1lc1t1aVRoZW1lSWRdO1xuICAgIHJldHVybiBmb3VuZCA/PyByZWFkU3RvcmVkVWlDaHJvbWVUaGVtZVNuYXBzaG90KHNjb3BlLnN0b3JhZ2UpID8/IHNlbWlvVGhlbWUoKTtcbiAgfSwgW3VpVGhlbWVJZCwgdWlDdXN0b21UaGVtZXMsIHVpVGhlbWVEcmFmdCwgc2NvcGUuc3RvcmFnZV0pO1xuICBjb25zdCB1aURyaXZlcjogVWlEcml2ZXIgPSB1c2VNZW1vKCgpID0+IHVpRHJpdmVyRHJhZnQgPz8gcmVzb2x2ZVVpRHJpdmVyKHVpRHJpdmVySWQsIHVpQ3VzdG9tRHJpdmVycyksIFt1aURyaXZlcklkLCB1aUN1c3RvbURyaXZlcnMsIHVpRHJpdmVyRHJhZnRdKTtcbiAgLyoqIPCfp7XvuI8gTGF6aWx5LWNyZWF0ZWQgd29ya2VyIHJ1bm5pbmcgYPCfn6bvuI9iYWNrYm9uZS3wn5+m77iPd29ya2VyLnRzYCDigJQgb25lIHBlciBzaGVsbCBpbnN0YW5jZSwgcmV1c2VkIGFjcm9zcyBgb3BlbkRvY3VtZW50YCBjYWxscy4gKi9cbiAgY29uc3QgYmFja2JvbmVXb3JrZXJSZWYgPSB1c2VSZWY8V29ya2VyIHwgbnVsbD4obnVsbCk7XG4gIC8qKiDwn5aL77iPIFN0YWJsZSBwZXItdGFiIGFjdG9yIGlkIGZvciBodWIgYEhlbGxvYC9wcmVzZW5jZSBmcmFtZXMgYW5kIG9wZXJhdGlvbi1vcmlnaW4gZmlsdGVyaW5nLiAqL1xuICBjb25zdCBzaGVsbEFjdG9ySWRSZWYgPSB1c2VSZWY8c3RyaW5nPihgY2xpZW50LSR7TWF0aC5yYW5kb20oKS50b1N0cmluZygzNikuc2xpY2UoMil9YCk7XG4gIC8qKiDwn5eC77iPIFdoaWNoIHNlc3Npb24vcGx1Z2luIG93bnMgZWFjaCBvcGVuIGRvY3VtZW50IGlkLCBzbyBpbmNvbWluZyB3b3JrZXIgZXZlbnRzIHJvdXRlIGNvcnJlY3RseS4gKi9cbiAgY29uc3Qgb3BlbkRvY3VtZW50U2Vzc2lvbnNSZWYgPSB1c2VSZWY8TWFwPHN0cmluZywgeyBzZXNzaW9uOiBBY3RpdmVTZXNzaW9uOyBwbHVnaW46IFBsdWdpbldhc21IYW5kbGUgfT4+KG5ldyBNYXAoKSk7XG4gIC8qKiDwn5Ca77iPIFVucmVnaXN0ZXJzIHRoaXMgc2hlbGwncyBgcmVnaXN0ZXJQbHVnaW5CYWNrYm9uZVJvdXRlYCBlbnRyeSBmb3IgZWFjaCBvcGVuIGRvY3VtZW50IGlkIOKAlCBjYWxsZWRcbiAgICogZnJvbSBgY2xvc2VEb2N1bWVudGAgYW5kIChmb3Igd2hhdGV2ZXIgaXMgc3RpbGwgb3Blbikgb24gc2hlbGwgdW5tb3VudC4gKi9cbiAgY29uc3QgcGx1Z2luQmFja2JvbmVSb3V0ZVVucmVnaXN0ZXJzUmVmID0gdXNlUmVmPE1hcDxzdHJpbmcsICgpID0+IHZvaWQ+PihuZXcgTWFwKCkpO1xuICAvKiog8J+Qmu+4jyBNaXJyb3JzIGBsb2FkZWRQbHVnaW5zYCBmb3IgdGhlIHVubW91bnQtY2xlYW51cCBlZmZlY3QgYmVsb3csIHdoaWNoIG5lZWRzIHRoZSBsYXRlc3QgdmFsdWUgYXRcbiAgICogdGVhcmRvd24gdGltZSB3aXRob3V0IGRlcGVuZGluZyBvbiBpdCAoYSBkZXBlbmRlbmN5IHdvdWxkIHRlYXIgZG93biBhbmQgcmUtcnVuIG9uIGV2ZXJ5IHJlbG9hZCkuICovXG4gIGNvbnN0IGxvYWRlZFBsdWdpbnNSZWYgPSB1c2VSZWY8cmVhZG9ubHkgTG9hZGVkUHJvZ3JhbVN0YXRlW10+KFtdKTtcbiAgbG9hZGVkUGx1Z2luc1JlZi5jdXJyZW50ID0gbG9hZGVkUGx1Z2lucztcbiAgLyoqIPCflIzvuI8gVGhlIGV4YWN0IChwb3NzaWJseSBjYWNoZS1idXN0ZWQgYD92PWApIG1vZHVsZSBVUkwgZWFjaCBjdXJyZW50bHktbG9hZGVkIHBsdWdpbiB3YXMgYWNxdWlyZWRcbiAgICogYXQg4oCUIGBMb2FkZWRQcm9ncmFtU3RhdGVgL2BQbHVnaW5XYXNtSGFuZGxlYCBjYXJyeSBubyBVUkwgb2YgdGhlaXIgb3duLCBidXQgYHJlbG9hZFBsdWdpbmAvXG4gICAqIGB1bmluc3RhbGxQbHVnaW5gIG5lZWQgdGhlIE9MRCB1cmwgdG8gYGV2aWN0UGx1Z2luTW9kdWxlYCBhZnRlciBzd2FwcGluZyBpbiBhIG5ldyBsZWFzZSBhdCBhXG4gICAqIGRpZmZlcmVudCB1cmwgKHNlZSB0aGUgbGVhc2UgcG9vbCdzIGtleSBjb252ZW50aW9uIGluIGBAc2VtaW8tdGVjaC9mcmFtZXdvcmstY29yZWApLiAqL1xuICBjb25zdCBwbHVnaW5Nb2R1bGVVcmxCeUlkUmVmID0gdXNlUmVmPE1hcDxzdHJpbmcsIHN0cmluZz4+KG5ldyBNYXAoKSk7XG4gIC8qKiDwn5SM77iPIFBlci1wbHVnaW5JZCBtdXR1YWwgZXhjbHVzaW9uIGFjcm9zcyBgaW5zdGFsbFBsdWdpbmAvYHJlbG9hZFBsdWdpbmAvYHVuaW5zdGFsbFBsdWdpbmAg4oCUIHRoZVxuICAgKiBib290IGVmZmVjdCBhbmQgdGhlIGBQbHVnaW5Tb3VyY2VgIHN1YnNjcmlwdGlvbiBlZmZlY3QgY2FuIGJvdGggcmVxdWVzdCB0aGUgc2FtZSBwbHVnaW5JZCBhcm91bmRcbiAgICogbW91bnQgKGUuZy4gdGhlIGhvc3QgcGx1Z2luIGFscmVhZHkgYXBwZWFycyBpbiB0aGUgY29ubmVjdC10aW1lIGBzbmFwc2hvdGApLCBhbmQgd2l0aG91dCB0aGlzIGd1YXJkXG4gICAqIGJvdGggY2FsbHMgd291bGQgaW5kZXBlbmRlbnRseSBhY3F1aXJlIGEgbW9kdWxlIGxlYXNlLCByYWNlIHRoZWlyIGBVUFNFUlRfTE9BREVEX1BMVUdJTmAgZGlzcGF0Y2hlcyxcbiAgICogYW5kIGxlYWsgd2hpY2hldmVyIGxlYXNlIGxvc3QgdGhlIHJhY2UgKG5vdGhpbmcgbGVmdCBob2xkaW5nIGEgcmVmZXJlbmNlIHRvIHJlbGVhc2UgaXQpLiAqL1xuICBjb25zdCBwbHVnaW5PcEluRmxpZ2h0UmVmID0gdXNlUmVmPFNldDxzdHJpbmc+PihuZXcgU2V0KCkpO1xuXG4gIGNvbnN0IGVuc3VyZUJhY2tib25lV29ya2VyID0gdXNlQ2FsbGJhY2soKCk6IFdvcmtlciA9PiB7XG4gICAgaWYgKGJhY2tib25lV29ya2VyUmVmLmN1cnJlbnQpIHJldHVybiBiYWNrYm9uZVdvcmtlclJlZi5jdXJyZW50O1xuICAgIGNvbnN0IHdvcmtlciA9IG5ldyBXb3JrZXIobmV3IFVSTChcIi4uLy4uLy4uLy4uLy4uL+Kaoe+4j2ltcGxlbWVudGF0aW9ucy/wn5+m77iPdHlwZXNjcmlwdC/wn5+m77iPYmFja2JvbmUtd29ya2VyLnRzXCIsIGltcG9ydC5tZXRhLnVybCksIHsgdHlwZTogXCJtb2R1bGVcIiB9KTtcbiAgICB3b3JrZXIub25tZXNzYWdlID0gKG1lc3NhZ2VFdmVudDogTWVzc2FnZUV2ZW50PEJhY2tib25lV29ya2VyUmVzcG9uc2UgfCB7IHJlYWRvbmx5IHdpcmU6IFVpbnQ4QXJyYXkgfT4pID0+IHtcbiAgICAgIGNvbnN0IG1lc3NhZ2UgPSBcIndpcmVcIiBpbiBtZXNzYWdlRXZlbnQuZGF0YSA/IGRlY29kZUJhY2tib25lV29ya2VyUmVzcG9uc2UobWVzc2FnZUV2ZW50LmRhdGEud2lyZSkgOiBtZXNzYWdlRXZlbnQuZGF0YTtcbiAgICAgIGlmIChtZXNzYWdlLmtpbmQgIT09IFwiZXZlbnRcIikgcmV0dXJuO1xuICAgICAgY29uc3QgZW50cnkgPSBvcGVuRG9jdW1lbnRTZXNzaW9uc1JlZi5jdXJyZW50LmdldChtZXNzYWdlLmRvY3VtZW50SWQpO1xuICAgICAgaWYgKCFlbnRyeSkgcmV0dXJuO1xuICAgICAgY29uc3QgeyBldmVudCB9ID0gbWVzc2FnZTtcbiAgICAgIGlmIChldmVudC5raW5kID09PSBcInN0YXR1c1wiKSB7XG4gICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfU1lOQ19TVEFUVVNfRk9SX0RPQ1VNRU5UXCIsIGRvY3VtZW50SWQ6IG1lc3NhZ2UuZG9jdW1lbnRJZCwgc3RhdHVzOiB7IHBlcnNpc3RlZDogZXZlbnQucGVyc2lzdGVkLCBwZW5kaW5nT3BlcmF0aW9uczogZXZlbnQucGVuZGluZ09wZXJhdGlvbnMsIHJlbW90ZTogZXZlbnQucmVtb3RlIH0gfSk7XG4gICAgICB9IGVsc2UgaWYgKGV2ZW50LmtpbmQgPT09IFwicHJlc2VuY2VcIikge1xuICAgICAgICBjb25zdCBwZWVyc0pzb24gPSBKU09OLnN0cmluZ2lmeShldmVudC5wZWVycy5tYXAoKHBlZXIpID0+ICh7IGNsaWVudElkOiBwZWVyLmFjdG9yLCBuYW1lOiBwZWVyLmxhYmVsID8/IHBlZXIuYWN0b3IsIHNlbGVjdGlvbkNvdW50OiAwIH0pKSk7XG4gICAgICAgIGRpc3BhdGNoKHtcbiAgICAgICAgICB0eXBlOiBcIlNFVF9TRVNTSU9OXCIsXG4gICAgICAgICAgdmFsdWU6IChjdXJyZW50KSA9PiAoY3VycmVudCAmJiBjdXJyZW50Lmluc3RhbmNlSWQgPT09IGVudHJ5LnNlc3Npb24uaW5zdGFuY2VJZCA/IHsgLi4uY3VycmVudCwgdmlld1N0YXRlOiB7IC4uLmN1cnJlbnQudmlld1N0YXRlLCBwcmVzZW5jZVBlZXJzSnNvbjogcGVlcnNKc29uIH0gfSA6IGN1cnJlbnQpLFxuICAgICAgICB9KTtcbiAgICAgIH0gZWxzZSBpZiAoZXZlbnQua2luZCA9PT0gXCJyZW1vdGVPcGVyYXRpb25zXCIgJiYgZW50cnkucGx1Z2luLmFwcGx5T3BlcmF0aW9ucykge1xuICAgICAgICB2b2lkIGVudHJ5LnBsdWdpbi5hcHBseU9wZXJhdGlvbnMoZW50cnkuc2Vzc2lvbi5pbnN0YW5jZUlkLCBlbmNvZGVPcGVyYXRpb25FbnZlbG9wZXNQYWNrKGV2ZW50LmVudmVsb3BlcykpO1xuICAgICAgICBjb25zdCBhY3RvclVyaSA9IGBhY3RvcjovLyR7bWVzc2FnZS5kb2N1bWVudElkfWA7XG4gICAgICAgIHBvc3RQbHVnaW5CYWNrYm9uZUluYm91bmQoZW50cnkuc2Vzc2lvbi5wbHVnaW5JZCwgYWN0b3JVcmksIFtcbiAgICAgICAgICBlbmNvZGVCYWNrYm9uZU1lc3NhZ2Uoe1xuICAgICAgICAgICAga2luZDogXCJvcGVyYXRpb25zXCIsXG4gICAgICAgICAgICBlbnZlbG9wZXM6IGV2ZW50LmVudmVsb3Blcy5tYXAoKGVudmVsb3BlLCBpbmRleCkgPT5cbiAgICAgICAgICAgICAgb3BlcmF0aW9uRW52ZWxvcGVUb1dpcmUoZW52ZWxvcGUsIHsgYWN0b3I6IDAsIHBoeXNpY2FsX21zOiBEYXRlLm5vdygpLCBsb2dpY2FsOiBpbmRleCArIDEgfSksXG4gICAgICAgICAgICApLFxuICAgICAgICAgIH0pLFxuICAgICAgICBdKTtcbiAgICAgIH0gZWxzZSBpZiAoZXZlbnQua2luZCA9PT0gXCJzbmFwc2hvdFJlcGxhY2VkXCIgJiYgZW50cnkucGx1Z2luLmxvYWRBcHBEb2N1bWVudCkge1xuICAgICAgICBjb25zdCBwYWNrQnl0ZXMgPSBuZXcgVWludDhBcnJheShldmVudC5wYWNrKTtcbiAgICAgICAgbGV0IGRvY3VtZW50SnNvbjogc3RyaW5nO1xuICAgICAgICB0cnkge1xuICAgICAgICAgIGRvY3VtZW50SnNvbiA9IEpTT04uc3RyaW5naWZ5KGRlY29kZVBhY2tWYWx1ZShwYWNrQnl0ZXMpKTtcbiAgICAgICAgfSBjYXRjaCB7XG4gICAgICAgICAgZG9jdW1lbnRKc29uID0gSlNPTi5zdHJpbmdpZnkoeyBwYWNrOiBBcnJheS5mcm9tKGV2ZW50LnBhY2spLCBzcHI6IEFycmF5LmZyb20oZXZlbnQuc3ByKSB9KTtcbiAgICAgICAgfVxuICAgICAgICB2b2lkIGVudHJ5LnBsdWdpbi5sb2FkQXBwRG9jdW1lbnQoZW50cnkuc2Vzc2lvbi5pbnN0YW5jZUlkLCBkb2N1bWVudEpzb24pO1xuICAgICAgICBjb25zdCBhY3RvclVyaSA9IGBhY3RvcjovLyR7bWVzc2FnZS5kb2N1bWVudElkfWA7XG4gICAgICAgIHBvc3RQbHVnaW5CYWNrYm9uZUluYm91bmQoZW50cnkuc2Vzc2lvbi5wbHVnaW5JZCwgYWN0b3JVcmksIFtcbiAgICAgICAgICBlbmNvZGVCYWNrYm9uZU1lc3NhZ2UoeyBraW5kOiBcInNuYXBzaG90XCIsIHBhY2s6IHBhY2tCeXRlcywgc3ByOiBuZXcgVWludDhBcnJheShldmVudC5zcHIpIH0pLFxuICAgICAgICBdKTtcbiAgICAgIH0gZWxzZSBpZiAoZXZlbnQua2luZCA9PT0gXCJjb25mbGljdFwiKSB7XG4gICAgICAgIGNvbnNvbGUud2FybihcIltvcy1zaGVsbF0gc3luYyBjb25mbGljdFwiLCBtZXNzYWdlLmRvY3VtZW50SWQsIGV2ZW50Lm1lc3NhZ2UpO1xuICAgICAgfVxuICAgIH07XG4gICAgYmFja2JvbmVXb3JrZXJSZWYuY3VycmVudCA9IHdvcmtlcjtcbiAgICByZXR1cm4gd29ya2VyO1xuICB9LCBbXSk7XG5cbiAgLy8g8J+Qmu+4jyBPbmx5IGEgcGFnZS1vd25pbmcgc3R1ZGlvIHNoZWxsIHN5bmNzIHRvIHRoZSByZWFsIGJyb3dzZXIgVVJMIGJhci9oaXN0b3J5IOKAlCBhbiBlbWJlZGRlZCBzaGVsbFxuICAvLyBzaGFyaW5nIHRoZSBwYWdlIHdpdGggb3RoZXJzIG11c3Qgbm90IGZpZ2h0IHRoZW0gb3ZlciBgd2luZG93Lmhpc3RvcnlgLlxuICBjb25zdCB7IHVyaTogc2hlbGxVcmksIGNhbkdvQmFjaywgY2FuR29Gb3J3YXJkLCBjYW5Hb1VwLCBnb0JhY2ssIGdvRm9yd2FyZCwgZ29VcCwgbmF2aWdhdGU6IG5hdmlnYXRlSGlzdG9yeSB9ID0gdXNlVUlIaXN0b3J5KFwiL1wiLCBzdHVkaW9Nb2RlICYmIHNjb3BlLm93bnNQYWdlKTtcbiAgY29uc3Qgc2hlbGxSb3V0ZSA9IHVzZU1lbW8oKCkgPT4gcGFyc2VTaGVsbFJvdXRlKHNoZWxsVXJpLnNwbGl0KFwiP1wiKVswXSA/PyBcIi9cIiksIFtzaGVsbFVyaV0pO1xuXG4gIC8vIPCfkJrvuI8gYHNjb3BlLnN0b3JhZ2VgIChub3QgYSBzZXBhcmF0ZWx5LXJlc29sdmVkIGVwaGVtZXJhbC9icm93c2VyIHBvcnQgaGVyZSkg4oCUIHR3byBzaGVsbHMgc2hhcmluZyBhXG4gIC8vIHBhZ2UgbXVzdCBub3QgY2xvYmJlciBlYWNoIG90aGVyJ3MgcGFuZWwgbGF5b3V0L2RvY2sgc3RhdGUgdGhyb3VnaCBhbiB1bm5hbWVzcGFjZWQgbG9jYWxTdG9yYWdlIGtleS5cbiAgY29uc3Qgc2hlbGxTdG9yYWdlID0gc2NvcGUuc3RvcmFnZTtcbiAgY29uc3QgbmFtZWRMYXlvdXRTdG9yZSA9IHVzZU1lbW8oKCkgPT4gbmV3IE5hbWVkTGF5b3V0U3RvcmUoc2Vzc2lvbj8uYXBwLmlkID8/IFwiZnJhbWV3b3JrLW9zXCIsIHNoZWxsU3RvcmFnZSksIFtzZXNzaW9uPy5hcHAuaWQsIHNoZWxsU3RvcmFnZV0pO1xuICBjb25zdCBkb2NrTGF5b3V0U3RvcmUgPSB1c2VNZW1vKCgpID0+IG5ldyBEb2NrTGF5b3V0U3RvcmUoc2hlbGxTdG9yYWdlLCBzZXNzaW9uPy5hcHAuaWQpLCBbc2Vzc2lvbj8uYXBwLmlkLCBzaGVsbFN0b3JhZ2VdKTtcbiAgY29uc3QgZG9ja1VpU3RhdGVTdG9yZSA9IHVzZU1lbW8oKCkgPT4gbmV3IERvY2tVaVN0YXRlU3RvcmUoc2hlbGxTdG9yYWdlLCBzZXNzaW9uPy5hcHAuaWQpLCBbc2Vzc2lvbj8uYXBwLmlkLCBzaGVsbFN0b3JhZ2VdKTtcblxuICBjb25zdCByZWdpc3RyeSA9IHVzZU1lbW8oKCkgPT4ge1xuICAgIGNvbnN0IGV4cGFuZGVkID0gZXhwYW5kUGx1Z2luUmVnaXN0cnkocGx1Z2lucywgcGx1Z2luRmlsdGVyID8gcmVzb2x2ZVBsdWdpblJlZ2lzdHJ5SWQocGx1Z2luRmlsdGVyKSA6IHVuZGVmaW5lZCwgc3R1ZGlvTW9kZSk7XG4gICAgaWYgKHN0dWRpb01vZGUpIHJldHVybiBleHBhbmRlZDtcbiAgICByZXR1cm4gcGx1Z2luRmlsdGVyID8gZXhwYW5kZWQgOiBwbHVnaW5zO1xuICB9LCBbcGx1Z2luRmlsdGVyLCBwbHVnaW5zLCBzdHVkaW9Nb2RlXSk7XG5cbiAgLy8jcmVnaW9uIPCflIzvuI9QbHVnaW5SdW50aW1lXG4gIC8qKiDwn5SM77iPIFRoZSBvbmUgcmVnaXN0cnkgZW50cnkgdGhlIHNoZWxsIG11c3QgaGF2ZSBsb2FkZWQgYmVmb3JlIGl0IGNhbiBjcmVhdGUgYSBzZXNzaW9uIOKAlCB0aGUgc3R1ZGlvXG4gICAqIGhvc3QgcGx1Z2luIChgaG9zdENvbmZpZy5wbHVnaW5JZGApIGluIHN0dWRpbyBtb2RlLCBvdGhlcndpc2UgdGhlIHJlc29sdmVkIHNpbmdsZS1hcHAgdmFyaWFudC5cbiAgICogRXZlcnkgb3RoZXIgcmVnaXN0cnkgZW50cnkgc3RyZWFtcyBpbiBpbmRlcGVuZGVudGx5IGFuZCBpcyBuZXZlciBmYXRhbCB0byBib290LiAqL1xuICBjb25zdCBwcmltYXJ5UGx1Z2luSWQgPSB1c2VNZW1vKCgpID0+IGhvc3RDb25maWc/LnBsdWdpbklkID8/IChwbHVnaW5GaWx0ZXIgPyByZXNvbHZlUGx1Z2luUmVnaXN0cnlJZChwbHVnaW5GaWx0ZXIpIDogdW5kZWZpbmVkKSA/PyByZWdpc3RyeVswXT8ucGx1Z2luSWQsIFtob3N0Q29uZmlnLCBwbHVnaW5GaWx0ZXIsIHJlZ2lzdHJ5XSk7XG4gIGNvbnN0IHNoZWxsUGx1Z2luQ2FudmFzU3RhdHVzID0gdXNlTWVtbygoKTogVWlTdGF0dXMgfCB1bmRlZmluZWQgPT4ge1xuICAgIGlmICghc2Vzc2lvbikgcmV0dXJuIFwibG9hZGluZ1wiO1xuICAgIGlmICghcHJpbWFyeVBsdWdpbklkKSByZXR1cm4gdW5kZWZpbmVkO1xuICAgIGNvbnN0IHBsdWdpblN0YXR1cyA9IHBsdWdpblN0YXR1c0J5SWRbcHJpbWFyeVBsdWdpbklkXTtcbiAgICBpZiAocGx1Z2luU3RhdHVzID09PSBcImluc3RhbGxpbmdcIiB8fCBwbHVnaW5TdGF0dXMgPT09IFwicmVsb2FkaW5nXCIpIHJldHVybiBcImxvYWRpbmdcIjtcbiAgICByZXR1cm4gdW5kZWZpbmVkO1xuICB9LCBbc2Vzc2lvbiwgcHJpbWFyeVBsdWdpbklkLCBwbHVnaW5TdGF0dXNCeUlkXSk7XG4gIC8qKiDwn5SM77iPIERldi1vbmx5IHRvZGF5IChgY3JlYXRlRGV2UGx1Z2luU291cmNlYCkg4oCUIGEgZnV0dXJlIGh1Yi1iYWNrZWQgc291cmNlIGltcGxlbWVudHMgdGhlIHNhbWVcbiAgICogYFBsdWdpblNvdXJjZWAgY29udHJhY3QgYW5kIHN3YXBzIGluIGhlcmUgd2l0aCBubyBvdGhlciBjaGFuZ2UgdG8gdGhlIHJ1bnRpbWUgYmVsb3cuICovXG4gIGNvbnN0IHBsdWdpblNvdXJjZTogUGx1Z2luU291cmNlID0gdXNlTWVtbygoKSA9PiBjcmVhdGVEZXZQbHVnaW5Tb3VyY2UocmVnaXN0cnkpLCBbcmVnaXN0cnldKTtcblxuICAvKiog8J+UjO+4jyBSZWNyZWF0ZXMgdGhlIHByaW1hcnkgc2Vzc2lvbiBpbnN0YW5jZSBmb3IgYGhhbmRsZWAg4oCUIHRoZSBleGFjdCBgaG9zdENvbmZpZ2Avbm9uLXN0dWRpb1xuICAgKiBhcHAtcmVzb2x1dGlvbiBsb2dpYyB0aGUgYm9vdCBlZmZlY3QgdXNlZCB0byBydW4gb25jZSBpbmxpbmUsIG5vdyBzaGFyZWQgd2l0aCBgcmVsb2FkUGx1Z2luYCBzbyBhXG4gICAqIGhvdC1zd2FwIG9mIHRoZSBzZXNzaW9uLW93bmluZyBwbHVnaW4gcmUtZXN0YWJsaXNoZXMgdGhlIHNlc3Npb24gdGhlIHNhbWUgd2F5IGJvb3QgZG9lcy4gKi9cbiAgY29uc3QgZXN0YWJsaXNoUHJpbWFyeVNlc3Npb24gPSB1c2VDYWxsYmFjayhcbiAgICBhc3luYyAoaGFuZGxlOiBQbHVnaW5XYXNtSGFuZGxlKSA9PiB7XG4gICAgICBjb25zdCBtYW5pZmVzdCA9IGhhbmRsZS5tYW5pZmVzdDtcbiAgICAgIGlmIChob3N0Q29uZmlnKSB7XG4gICAgICAgIGNvbnN0IHNBcHAgPSBtYW5pZmVzdC5hcHBzLmZpbmQoKGFwcCkgPT4gYXBwLmlkID09PSBob3N0Q29uZmlnLmxhbmRpbmdBcHBJZCkgPz8gbWFuaWZlc3QuYXBwc1swXTtcbiAgICAgICAgaWYgKCFzQXBwKSB0aHJvdyBuZXcgRXJyb3IoXCJob3N0IHByb2dyYW0gbWlzc2luZyBsYW5kaW5nIGFwcFwiKTtcbiAgICAgICAgLy8g8J+qpu+4jyBgbWFuaWZlc3Qud29ya2Zsb3dzYCAodGhlIHNvdXJjZSBgYnVpbGRTcGFjZVByb2dyYW1zYCB1c2VkIHRvIHJlYWQpIHdhcyBkZWxldGVkIGZyb20gdGhlXG4gICAgICAgIC8vIFJ1c3QgYFBsdWdpbk1hbmlmZXN0YCDigJQgdGhlIHN0dWRpbyBjYXRhbG9ndWUgaXMgbm93IHJlZ2lzdHJ5LWRyaXZlbiAoc2VlIGBTcGFjZUNvbW1hbmQ6OlNldEFwcFJlZ2lzdHJhdGlvbnNgKSxcbiAgICAgICAgLy8gc28gYFNwYWNlUGFuZWxTdGF0ZS5wcm9ncmFtc2AgaXMgcGVybWFuZW50bHkgZW1wdHk7IGBzcGF3bmVkQXBwc2AvYGFjdGl2ZVBhbmVsVGFiYC9gYWN0aXZlU3Bhd25lZElkYCBhcmVcbiAgICAgICAgLy8gc3RpbGwgcmVhbCwgbGl2ZSBzdGF0ZSwgc28gYFNwYWNlUGFuZWxTdGF0ZWAgaXRzZWxmIHN0YXlzLlxuICAgICAgICBjb25zdCBwYW5lbFN0YXRlID0gYnVpbGRTcGFjZVBhbmVsU3RhdGUoW10sIFtdKTtcbiAgICAgICAgY29uc3QgaW5zdGFuY2VJZCA9IGF3YWl0IGhhbmRsZS5jcmVhdGVBcHAoc0FwcC5pZCk7XG4gICAgICAgIGNvbnN0IHZpZXdTdGF0ZTogVmlld01vZGVsID0geyBhY3RpdmVNb2RlSWQ6IHNBcHAuZGVmYXVsdE1vZGVJZCA/PyBzQXBwLm1vZGVzWzBdPy5pZCwgcGFuZWxKc29uOiBwYW5lbEpzb25Gcm9tU3RhdGUocGFuZWxTdGF0ZSkgfTtcbiAgICAgICAgLy8g8J+qn++4jyBTZWVkIGRlZmF1bHQtbGF5b3V0IHBhbmVzIChUb3AvUGVyc3BlY3RpdmUpIGJlZm9yZSBhbnkgZWZmZWN0IGNhbiBmaXJlIGFjdGlvbnMg4oCUIG90aGVyd2lzZVxuICAgICAgICAvLyBib290IGBzZXRBY3RpdmVFeGFtcGxlYCByYWNlcyB0aGUgc2Vzc2lvbi1zd2l0Y2ggcmVmcmVzaCBhbmQgd2lwZXMgcGFuZSBib2RpZXMuXG4gICAgICAgIGNvbnN0IHNlZWRlZCA9IGFwcGx5RnJhbWV3b3JrTGF5b3V0U2VlZChzQXBwLmRlZmF1bHRMYXlvdXQsIHNBcHAud2luZG93S2luZHMsIEVNUFRZX0FQUF9MQUJFTFNfT1ZFUkxBWSwgdWlUZXJtaW5vbG9neSwgdWlMb2NhbGUpO1xuICAgICAgICBleHRyYVdpbmRvd0luc3RhbmNlc1JlZi5jdXJyZW50ID0gc2VlZGVkLmV4dHJhSW5zdGFuY2VzO1xuICAgICAgICBleHRyYVdpbmRvd0NvdW50ZXJSZWYuY3VycmVudCA9IHNlZWRlZC5leHRyYUluc3RhbmNlcy5sZW5ndGg7XG4gICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfU0VTU0lPTlwiLCB2YWx1ZTogeyBwbHVnaW5JZDogaGFuZGxlLnBsdWdpbklkLCBpbnN0YW5jZUlkLCBhcHA6IHNBcHAsIHZpZXdTdGF0ZSB9IH0pO1xuICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0VYVFJBX1dJTkRPV19JTlNUQU5DRVNcIiwgdmFsdWU6IHNlZWRlZC5leHRyYUluc3RhbmNlcyB9KTtcbiAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9TSEVMTF9MQVlPVVRcIiwgdmFsdWU6IHNlZWRlZC5tb2RlTGF5b3V0IH0pO1xuICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0FDVElWRV9XSU5ET1dfSURcIiwgdmFsdWU6IG51bGwgfSk7XG4gICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfRVJST1JcIiwgdmFsdWU6IG51bGwgfSk7XG4gICAgICAgIHJldHVybjtcbiAgICAgIH1cbiAgICAgIGNvbnN0IHByaW1hcnlBcHAgPSBhcHBJZFxuICAgICAgICA/ICgoKSA9PiB7XG4gICAgICAgICAgICBjb25zdCBmb3VuZCA9IG1hbmlmZXN0LmFwcHMuZmluZCgoYXBwKSA9PiBhcHAuaWQgPT09IGFwcElkKTtcbiAgICAgICAgICAgIGlmICghZm91bmQpIHRocm93IG5ldyBFcnJvcihgYXBwSWQgXCIke2FwcElkfVwiIGRvZXMgbm90IHJlc29sdmUgdG8gYW55IGFwcCBpbiB0aGUgbG9hZGVkIHByb2dyYW0gbWFuaWZlc3RgKTtcbiAgICAgICAgICAgIHJldHVybiBmb3VuZDtcbiAgICAgICAgICB9KSgpXG4gICAgICAgIDogKCgpID0+IHtcbiAgICAgICAgICAgIGNvbnN0IGRlZmF1bHRBcHBJZCA9IHBsdWdpbkZpbHRlciA/IHJlc29sdmVQbGF5Z3JvdW5kRGVmYXVsdEFwcElkKHBsdWdpbkZpbHRlcikgOiB1bmRlZmluZWQ7XG4gICAgICAgICAgICByZXR1cm4gKGRlZmF1bHRBcHBJZCA/IG1hbmlmZXN0LmFwcHMuZmluZCgoYXBwKSA9PiBhcHAuaWQgPT09IGRlZmF1bHRBcHBJZCkgOiB1bmRlZmluZWQpID8/IG1hbmlmZXN0LmFwcHNbMF07XG4gICAgICAgICAgfSkoKTtcbiAgICAgIGlmICghcHJpbWFyeUFwcCkgcmV0dXJuO1xuICAgICAgY29uc3QgaW5zdGFuY2VJZCA9IGF3YWl0IGhhbmRsZS5jcmVhdGVBcHAocHJpbWFyeUFwcC5pZCk7XG4gICAgICBjb25zdCBzZWVkZWQgPSBhcHBseUZyYW1ld29ya0xheW91dFNlZWQocHJpbWFyeUFwcC5kZWZhdWx0TGF5b3V0LCBwcmltYXJ5QXBwLndpbmRvd0tpbmRzLCBFTVBUWV9BUFBfTEFCRUxTX09WRVJMQVksIHVpVGVybWlub2xvZ3ksIHVpTG9jYWxlKTtcbiAgICAgIGV4dHJhV2luZG93SW5zdGFuY2VzUmVmLmN1cnJlbnQgPSBzZWVkZWQuZXh0cmFJbnN0YW5jZXM7XG4gICAgICBleHRyYVdpbmRvd0NvdW50ZXJSZWYuY3VycmVudCA9IHNlZWRlZC5leHRyYUluc3RhbmNlcy5sZW5ndGg7XG4gICAgICBkaXNwYXRjaCh7XG4gICAgICAgIHR5cGU6IFwiU0VUX1NFU1NJT05cIixcbiAgICAgICAgdmFsdWU6IHsgcGx1Z2luSWQ6IGhhbmRsZS5wbHVnaW5JZCwgaW5zdGFuY2VJZCwgYXBwOiBwcmltYXJ5QXBwLCB2aWV3U3RhdGU6IHsgYWN0aXZlTW9kZUlkOiBwcmltYXJ5QXBwLmRlZmF1bHRNb2RlSWQgPz8gcHJpbWFyeUFwcC5tb2Rlc1swXT8uaWQgfSB9LFxuICAgICAgfSk7XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0VYVFJBX1dJTkRPV19JTlNUQU5DRVNcIiwgdmFsdWU6IHNlZWRlZC5leHRyYUluc3RhbmNlcyB9KTtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfU0hFTExfTEFZT1VUXCIsIHZhbHVlOiBzZWVkZWQubW9kZUxheW91dCB9KTtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfQUNUSVZFX1dJTkRPV19JRFwiLCB2YWx1ZTogbnVsbCB9KTtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfRVJST1JcIiwgdmFsdWU6IG51bGwgfSk7XG4gICAgfSxcbiAgICBbaG9zdENvbmZpZywgYXBwSWQsIHBsdWdpbkZpbHRlciwgdWlUZXJtaW5vbG9neSwgdWlMb2NhbGVdLFxuICApO1xuXG4gIC8qKiDwn5SM77iPIEluc3RhbGxzIGEgcmVnaXN0cnkgZW50cnkgdGhhdCBpc24ndCBsb2FkZWQgeWV0OiBhY3F1aXJlcyBpdHMgbW9kdWxlICh3b3JrZXItYmFja2VkLCByZWZjb3VudGVkXG4gICAqIOKAlCBzZWUgYGFjcXVpcmVQbHVnaW5Nb2R1bGVgKSwgdXBzZXJ0cyBpdCBpbnRvIGBsb2FkZWRQbHVnaW5zYCwgYW5kIOKAlCBpZiB0aGlzIGlzIHRoZSBwcmltYXJ5IHBsdWdpblxuICAgKiBhbmQgbm8gc2Vzc2lvbiBleGlzdHMgeWV0IOKAlCBlc3RhYmxpc2hlcyB0aGUgc2Vzc2lvbi4gU2hhcmVkIGJ5IHRoZSBib290IGVmZmVjdCAocHJpbWFyeSBwbHVnaW5cbiAgICogb25seSkgYW5kIHRoZSBgUGx1Z2luU291cmNlYCBzdWJzY3JpcHRpb24gZWZmZWN0IChldmVyeSBvdGhlciBwbHVnaW4sIGFzIGl0cyBidWlsZCBsYW5kcykuICovXG4gIGNvbnN0IGluc3RhbGxQbHVnaW4gPSB1c2VDYWxsYmFjayhcbiAgICBhc3luYyAocGx1Z2luSWQ6IHN0cmluZywgcmVidWlsdEF0PzogbnVtYmVyKTogUHJvbWlzZTxQbHVnaW5JbnN0YWxsT3V0Y29tZT4gPT4ge1xuICAgICAgaWYgKHBsdWdpbk9wSW5GbGlnaHRSZWYuY3VycmVudC5oYXMocGx1Z2luSWQpKSByZXR1cm4gXCJpbi1mbGlnaHRcIjtcbiAgICAgIGlmIChsb2FkZWRQbHVnaW5zUmVmLmN1cnJlbnQuc29tZSgoZW50cnkpID0+IGVudHJ5LmhhbmRsZS5wbHVnaW5JZCA9PT0gcGx1Z2luSWQpKSByZXR1cm4gXCJhbHJlYWR5LWxvYWRlZFwiO1xuICAgICAgY29uc3QgZW50cnkgPSByZWdpc3RyeS5maW5kKChjYW5kaWRhdGUpID0+IGNhbmRpZGF0ZS5wbHVnaW5JZCA9PT0gcGx1Z2luSWQpO1xuICAgICAgaWYgKCFlbnRyeSkgcmV0dXJuIFwibWlzc2luZy1yZWdpc3RyeVwiO1xuICAgICAgcGx1Z2luT3BJbkZsaWdodFJlZi5jdXJyZW50LmFkZChwbHVnaW5JZCk7XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1BMVUdJTl9TVEFUVVNcIiwgcGx1Z2luSWQsIHZhbHVlOiBcImluc3RhbGxpbmdcIiB9KTtcbiAgICAgIHRyeSB7XG4gICAgICAgIGNvbnN0IG1vZHVsZVVybCA9IHBsdWdpblNvdXJjZS5tb2R1bGVVcmwocGx1Z2luSWQsIHJlYnVpbHRBdCk7XG4gICAgICAgIGNvbnN0IGhhbmRsZSA9IGF3YWl0IGxvYWRQbHVnaW5Nb2R1bGVSZXNpbGllbnQocGx1Z2luSWQsIG1vZHVsZVVybCk7XG4gICAgICAgIGlmICghaGFuZGxlKSB7XG4gICAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9QTFVHSU5fU1RBVFVTXCIsIHBsdWdpbklkLCB2YWx1ZTogXCJmYWlsZWRcIiB9KTtcbiAgICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1BMVUdJTl9TVVBFUlZJU09SXCIsIHBsdWdpbklkLCB2YWx1ZTogXCJjcmFzaGVkXCIgfSk7XG4gICAgICAgICAgcmV0dXJuIFwiZmFpbGVkXCI7XG4gICAgICAgIH1cbiAgICAgICAgcGx1Z2luTW9kdWxlVXJsQnlJZFJlZi5jdXJyZW50LnNldChwbHVnaW5JZCwgbW9kdWxlVXJsKTtcbiAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlVQU0VSVF9MT0FERURfUExVR0lOXCIsIHZhbHVlOiB7IGhhbmRsZSwgbWFuaWZlc3Q6IGhhbmRsZS5tYW5pZmVzdCB9IH0pO1xuICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1BMVUdJTl9TVEFUVVNcIiwgcGx1Z2luSWQsIHZhbHVlOiBcImxvYWRlZFwiIH0pO1xuICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1BMVUdJTl9TVVBFUlZJU09SXCIsIHBsdWdpbklkLCB2YWx1ZTogXCJsb2FkZWRcIiB9KTtcbiAgICAgICAgaWYgKHBsdWdpbklkID09PSBwcmltYXJ5UGx1Z2luSWQgJiYgIXNlc3Npb25SZWYuY3VycmVudCkge1xuICAgICAgICAgIHRyeSB7XG4gICAgICAgICAgICBhd2FpdCBlc3RhYmxpc2hQcmltYXJ5U2Vzc2lvbihoYW5kbGUpO1xuICAgICAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9QTFVHSU5fU1VQRVJWSVNPUlwiLCBwbHVnaW5JZCwgdmFsdWU6IFwicnVubmluZ1wiIH0pO1xuICAgICAgICAgIH0gY2F0Y2ggKGJvb3RFcnJvcikge1xuICAgICAgICAgICAgY29uc29sZS5lcnJvcihcIltERUJVR10gZnJhbWV3b3JrIG9zIGJvb3QgZmFpbGVkXCIsIGJvb3RFcnJvcik7XG4gICAgICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0VSUk9SXCIsIHZhbHVlOiBib290RXJyb3IgaW5zdGFuY2VvZiBFcnJvciA/IGJvb3RFcnJvci5tZXNzYWdlIDogU3RyaW5nKGJvb3RFcnJvcikgfSk7XG4gICAgICAgICAgICByZXR1cm4gXCJmYWlsZWRcIjtcbiAgICAgICAgICB9XG4gICAgICAgIH1cbiAgICAgICAgcmV0dXJuIFwibG9hZGVkXCI7XG4gICAgICB9IGZpbmFsbHkge1xuICAgICAgICBwbHVnaW5PcEluRmxpZ2h0UmVmLmN1cnJlbnQuZGVsZXRlKHBsdWdpbklkKTtcbiAgICAgIH1cbiAgICB9LFxuICAgIFtyZWdpc3RyeSwgcGx1Z2luU291cmNlLCBwcmltYXJ5UGx1Z2luSWQsIGVzdGFibGlzaFByaW1hcnlTZXNzaW9uXSxcbiAgKTtcblxuICAvKiog8J+UjO+4jyBIb3Qtc3dhcHMgYW4gYWxyZWFkeS1sb2FkZWQgcGx1Z2luIHRvIGEgbmV3bHkgYnVpbHQgbW9kdWxlIOKAlCBtaXJyb3JzIHRoZSBvcy1jb3JlIGtlcm5lbCdzXG4gICAqIGBQbHVnaW5Ib3N0Ojpob3Rfc3dhcF9wbHVnaW5gIGNvbnRyYWN0ICh2YWxpZGF0ZSDihpIgZGVzdHJveSBhZmZlY3RlZCBpbnN0YW5jZXMg4oaSIHN3YXAg4oaSIHJlY3JlYXRlIHRoZVxuICAgKiBzZXNzaW9uIGlmIGl0IHdhcyB0aGlzIHBsdWdpbidzIOKGkiByZWxlYXNlIHRoZSBvbGQgbW9kdWxlKSB3aXRob3V0IGludmVudGluZyBhIHNlcGFyYXRlIG9uZTpcbiAgICogYWNxdWlyZXMgdGhlIG5ldyBtb2R1bGUgQkVGT1JFIHRlYXJpbmcgYW55dGhpbmcgZG93biAodGhlIG9sZCBoYW5kbGUga2VlcHMgc2VydmluZyBjb25jdXJyZW50XG4gICAqIHRyYWZmaWMgZHVyaW5nIHRoZSBzd2FwKSwgdmFsaWRhdGVzIHRoZSBuZXcgbWFuaWZlc3Qgc3RpbGwgZGVjbGFyZXMgYXBwcyAoYW5kLCBpZiB0aGlzIHBsdWdpbiBvd25zXG4gICAqIHRoZSBhY3RpdmUgc2Vzc2lvbiwgc3RpbGwgZGVjbGFyZXMgdGhlIHNlc3Npb24ncyBhcHAgaWQpLCB0aGVuIG9ubHkgY29tbWl0cy4gQSB2YWxpZGF0aW9uIGZhaWx1cmVcbiAgICogZGlzcG9zZXMgdGhlIG5ldyBsZWFzZSBhbmQgbGVhdmVzIHRoZSBvbGQgcGx1Z2luIGV4YWN0bHkgYXMgaXQgd2FzIOKAlCBub3RoaW5nIGRlc3Ryb3llZCwgc3RhdHVzIGJhY2tcbiAgICogdG8gYFwibG9hZGVkXCJgLiAqL1xuICBjb25zdCByZWxvYWRQbHVnaW4gPSB1c2VDYWxsYmFjayhcbiAgICBhc3luYyAocGx1Z2luSWQ6IHN0cmluZywgcmVidWlsdEF0PzogbnVtYmVyKSA9PiB7XG4gICAgICBpZiAocGx1Z2luT3BJbkZsaWdodFJlZi5jdXJyZW50LmhhcyhwbHVnaW5JZCkpIHJldHVybjtcbiAgICAgIGNvbnN0IGN1cnJlbnQgPSBsb2FkZWRQbHVnaW5zUmVmLmN1cnJlbnQuZmluZCgoZW50cnkpID0+IGVudHJ5LmhhbmRsZS5wbHVnaW5JZCA9PT0gcGx1Z2luSWQpO1xuICAgICAgaWYgKCFjdXJyZW50KSByZXR1cm4gaW5zdGFsbFBsdWdpbihwbHVnaW5JZCwgcmVidWlsdEF0KTtcbiAgICAgIGNvbnN0IG9sZE1vZHVsZVVybCA9IHBsdWdpbk1vZHVsZVVybEJ5SWRSZWYuY3VycmVudC5nZXQocGx1Z2luSWQpO1xuICAgICAgcGx1Z2luT3BJbkZsaWdodFJlZi5jdXJyZW50LmFkZChwbHVnaW5JZCk7XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1BMVUdJTl9TVEFUVVNcIiwgcGx1Z2luSWQsIHZhbHVlOiBcInJlbG9hZGluZ1wiIH0pO1xuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9QTFVHSU5fU1VQRVJWSVNPUlwiLCBwbHVnaW5JZCwgdmFsdWU6IFwicmVzdGFydGluZ1wiIH0pO1xuICAgICAgbGV0IG5ld0hhbmRsZTogUGx1Z2luV2FzbUhhbmRsZSB8IG51bGwgPSBudWxsO1xuICAgICAgdHJ5IHtcbiAgICAgICAgY29uc3QgbW9kdWxlVXJsID0gcGx1Z2luU291cmNlLm1vZHVsZVVybChwbHVnaW5JZCwgcmVidWlsdEF0KTtcbiAgICAgICAgbmV3SGFuZGxlID0gYXdhaXQgbG9hZFBsdWdpbk1vZHVsZVJlc2lsaWVudChwbHVnaW5JZCwgbW9kdWxlVXJsKTtcbiAgICAgICAgaWYgKCFuZXdIYW5kbGUpIHRocm93IG5ldyBFcnJvcihgcHJvZ3JhbSAke3BsdWdpbklkfSBmYWlsZWQgdG8gcmVsb2FkYCk7XG4gICAgICAgIGlmIChuZXdIYW5kbGUubWFuaWZlc3QuYXBwcy5sZW5ndGggPT09IDApIHRocm93IG5ldyBFcnJvcihgcHJvZ3JhbSAke3BsdWdpbklkfSByZWxvYWQgZGVjbGFyZXMgbm8gYXBwc2ApO1xuICAgICAgICBjb25zdCBhY3RpdmVTZXNzaW9uID0gc2Vzc2lvblJlZi5jdXJyZW50O1xuICAgICAgICBjb25zdCBvd25zU2Vzc2lvbiA9IGFjdGl2ZVNlc3Npb24/LnBsdWdpbklkID09PSBwbHVnaW5JZDtcbiAgICAgICAgaWYgKG93bnNTZXNzaW9uICYmIGFjdGl2ZVNlc3Npb24gJiYgIW5ld0hhbmRsZS5tYW5pZmVzdC5hcHBzLnNvbWUoKGFwcCkgPT4gYXBwLmlkID09PSBhY3RpdmVTZXNzaW9uLmFwcC5pZCkpIHtcbiAgICAgICAgICB0aHJvdyBuZXcgRXJyb3IoYHByb2dyYW0gJHtwbHVnaW5JZH0gcmVsb2FkIGRyb3BwZWQgdGhlIGFjdGl2ZSBzZXNzaW9uJ3MgYXBwIFwiJHthY3RpdmVTZXNzaW9uLmFwcC5pZH1cImApO1xuICAgICAgICB9XG5cbiAgICAgICAgY29uc3Qgb2xkQXBwSWRzID0gbmV3IFNldChjdXJyZW50Lm1hbmlmZXN0LmFwcHMubWFwKChhcHApID0+IGFwcC5pZCkpO1xuICAgICAgICBjb25zdCBuZXdBcHBJZHMgPSBuZXcgU2V0KG5ld0hhbmRsZS5tYW5pZmVzdC5hcHBzLm1hcCgoYXBwKSA9PiBhcHAuaWQpKTtcbiAgICAgICAgY29uc3QgaG90U3dhcEV2ZW50OiBQcm9ncmFtSG90U3dhcEV2ZW50ID0ge1xuICAgICAgICAgIHBsdWdpbklkLFxuICAgICAgICAgIHZlcnNpb246IG5ld0hhbmRsZS5tYW5pZmVzdC52ZXJzaW9uLFxuICAgICAgICAgIGFkZGVkQXBwczogWy4uLm5ld0FwcElkc10uZmlsdGVyKChpZCkgPT4gIW9sZEFwcElkcy5oYXMoaWQpKSxcbiAgICAgICAgICByZW1vdmVkQXBwczogWy4uLm9sZEFwcElkc10uZmlsdGVyKChpZCkgPT4gIW5ld0FwcElkcy5oYXMoaWQpKSxcbiAgICAgICAgfTtcbiAgICAgICAgY29uc29sZS5sb2coYFtERUJVR10gaG90LXN3YXAgJHtwbHVnaW5JZH1gLCBob3RTd2FwRXZlbnQpO1xuXG4gICAgICAgIC8vIPCfqqbvuI8gRGVzdHJveSB0aGlzIHBsdWdpbidzIGxpdmUgaW5zdGFuY2VzIHVuZGVyIHRoZSBPTEQgaGFuZGxlIGJlZm9yZSBzd2FwcGluZyDigJQgdGhlIHByaW1hcnlcbiAgICAgICAgLy8gc2Vzc2lvbiBpbnN0YW5jZSAoaWYgb3duZWQpLCBldmVyeSBzdHVkaW8tc3Bhd25lZCBpbnN0YW5jZSwgYW5kIGFueSBleHRlcm5hbC1zbG90IGNvbnRyaWJ1dG9yXG4gICAgICAgIC8vIGluc3RhbmNlLiBNaXJyb3JzIHRoZSBzaGVsbC11bm1vdW50IHRlYXJkb3duIGVmZmVjdCwgc2NvcGVkIHRvIG9uZSBwbHVnaW5JZCBpbnN0ZWFkIG9mIGV2ZXJ5XG4gICAgICAgIC8vIGxvYWRlZCBwbHVnaW4uXG4gICAgICAgIGlmIChvd25zU2Vzc2lvbiAmJiBhY3RpdmVTZXNzaW9uKSB7XG4gICAgICAgICAgYXdhaXQgY3VycmVudC5oYW5kbGUuZGVzdHJveUFwcChhY3RpdmVTZXNzaW9uLmluc3RhbmNlSWQpLmNhdGNoKCgpID0+IHt9KTtcbiAgICAgICAgfVxuICAgICAgICBmb3IgKGNvbnN0IHNwYXduZWQgb2Ygc3Bhd25lZEFwcHNSZWYuY3VycmVudC5maWx0ZXIoKGVudHJ5KSA9PiBlbnRyeS5wbHVnaW5JZCA9PT0gcGx1Z2luSWQpKSB7XG4gICAgICAgICAgYXdhaXQgY3VycmVudC5oYW5kbGUuZGVzdHJveUFwcChzcGF3bmVkLmluc3RhbmNlSWQpLmNhdGNoKCgpID0+IHt9KTtcbiAgICAgICAgfVxuICAgICAgICBjb25zdCBjb250cmlidXRvckluc3RhbmNlSWQgPSBjb250cmlidXRvckluc3RhbmNlc1JlZi5jdXJyZW50LmdldChwbHVnaW5JZCk7XG4gICAgICAgIGlmIChjb250cmlidXRvckluc3RhbmNlSWQgIT0gbnVsbCkge1xuICAgICAgICAgIGF3YWl0IGN1cnJlbnQuaGFuZGxlLmRlc3Ryb3lBcHAoY29udHJpYnV0b3JJbnN0YW5jZUlkKS5jYXRjaCgoKSA9PiB7fSk7XG4gICAgICAgICAgY29udHJpYnV0b3JJbnN0YW5jZXNSZWYuY3VycmVudC5kZWxldGUocGx1Z2luSWQpO1xuICAgICAgICB9XG4gICAgICAgIGlmIChzdHVkaW9Nb2RlICYmIGFjdGl2ZVNlc3Npb24pIHtcbiAgICAgICAgICBjb25zdCBjdXJyZW50UGFuZWwgPSBwYXJzZVBhbmVsU3RhdGUoYWN0aXZlU2Vzc2lvbi52aWV3U3RhdGUpO1xuICAgICAgICAgIGNvbnN0IGRyb3BwZWQgPSBjdXJyZW50UGFuZWw/LnNwYXduZWRBcHBzLmZpbHRlcigoZW50cnkpID0+IGVudHJ5LnBsdWdpbklkID09PSBwbHVnaW5JZCkgPz8gW107XG4gICAgICAgICAgaWYgKGN1cnJlbnRQYW5lbCAmJiBkcm9wcGVkLmxlbmd0aCA+IDApIHtcbiAgICAgICAgICAgIGNvbnNvbGUubG9nKFxuICAgICAgICAgICAgICBgW0RFQlVHXSBob3Qtc3dhcCAke3BsdWdpbklkfSBkcm9wcGVkICR7ZHJvcHBlZC5sZW5ndGh9IHNwYXduZWQgaW5zdGFuY2UocylgLFxuICAgICAgICAgICAgICBkcm9wcGVkLm1hcCgoZW50cnkpID0+IGVudHJ5LmlkKSxcbiAgICAgICAgICAgICk7XG4gICAgICAgICAgICBjb25zdCBzdXJ2aXZpbmdTcGF3bmVkID0gY3VycmVudFBhbmVsLnNwYXduZWRBcHBzLmZpbHRlcigoZW50cnkpID0+IGVudHJ5LnBsdWdpbklkICE9PSBwbHVnaW5JZCk7XG4gICAgICAgICAgICBjb25zdCBhY3RpdmVTcGF3bmVkSWQgPSBjdXJyZW50UGFuZWwuYWN0aXZlU3Bhd25lZElkICYmIGRyb3BwZWQuc29tZSgoZW50cnkpID0+IGVudHJ5LmlkID09PSBjdXJyZW50UGFuZWwuYWN0aXZlU3Bhd25lZElkKSA/IHVuZGVmaW5lZCA6IGN1cnJlbnRQYW5lbC5hY3RpdmVTcGF3bmVkSWQ7XG4gICAgICAgICAgICBjb25zdCBuZXh0UGFuZWwgPSB7IC4uLmN1cnJlbnRQYW5lbCwgc3Bhd25lZEFwcHM6IHN1cnZpdmluZ1NwYXduZWQsIGFjdGl2ZVNwYXduZWRJZCB9O1xuICAgICAgICAgICAgZGlzcGF0Y2goe1xuICAgICAgICAgICAgICB0eXBlOiBcIlNFVF9TRVNTSU9OXCIsXG4gICAgICAgICAgICAgIHZhbHVlOiAobmV4dFNlc3Npb24pID0+IChuZXh0U2Vzc2lvbiA/IHsgLi4ubmV4dFNlc3Npb24sIHZpZXdTdGF0ZTogeyAuLi5uZXh0U2Vzc2lvbi52aWV3U3RhdGUsIHBhbmVsSnNvbjogcGFuZWxKc29uRnJvbVN0YXRlKG5leHRQYW5lbCkgfSB9IDogbmV4dFNlc3Npb24pLFxuICAgICAgICAgICAgfSk7XG4gICAgICAgICAgfVxuICAgICAgICB9XG5cbiAgICAgICAgcGx1Z2luTW9kdWxlVXJsQnlJZFJlZi5jdXJyZW50LnNldChwbHVnaW5JZCwgbW9kdWxlVXJsKTtcbiAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlVQU0VSVF9MT0FERURfUExVR0lOXCIsIHZhbHVlOiB7IGhhbmRsZTogbmV3SGFuZGxlLCBtYW5pZmVzdDogbmV3SGFuZGxlLm1hbmlmZXN0IH0gfSk7XG4gICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfUExVR0lOX1NUQVRVU1wiLCBwbHVnaW5JZCwgdmFsdWU6IFwibG9hZGVkXCIgfSk7XG4gICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfUExVR0lOX1NVUEVSVklTT1JcIiwgcGx1Z2luSWQsIHZhbHVlOiBvd25zU2Vzc2lvbiA/IFwicnVubmluZ1wiIDogXCJsb2FkZWRcIiB9KTtcblxuICAgICAgICBpZiAob3duc1Nlc3Npb24pIGF3YWl0IGVzdGFibGlzaFByaW1hcnlTZXNzaW9uKG5ld0hhbmRsZSk7XG5cbiAgICAgICAgY3VycmVudC5oYW5kbGUuZGlzcG9zZSgpO1xuICAgICAgICBpZiAob2xkTW9kdWxlVXJsKSBldmljdFBsdWdpbk1vZHVsZShvbGRNb2R1bGVVcmwpO1xuICAgICAgfSBjYXRjaCAoZXJyb3IpIHtcbiAgICAgICAgY29uc29sZS53YXJuKGBbREVCVUddIGhvdC1zd2FwIHJvbGxlZCBiYWNrIGZvciAke3BsdWdpbklkfWAsIGVycm9yKTtcbiAgICAgICAgbmV3SGFuZGxlPy5kaXNwb3NlKCk7XG4gICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfUExVR0lOX1NUQVRVU1wiLCBwbHVnaW5JZCwgdmFsdWU6IFwibG9hZGVkXCIgfSk7XG4gICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfUExVR0lOX1NVUEVSVklTT1JcIiwgcGx1Z2luSWQsIHZhbHVlOiBcImNyYXNoZWRcIiB9KTtcbiAgICAgIH0gZmluYWxseSB7XG4gICAgICAgIHBsdWdpbk9wSW5GbGlnaHRSZWYuY3VycmVudC5kZWxldGUocGx1Z2luSWQpO1xuICAgICAgfVxuICAgIH0sXG4gICAgW2luc3RhbGxQbHVnaW4sIGVzdGFibGlzaFByaW1hcnlTZXNzaW9uLCBzdHVkaW9Nb2RlLCBwbHVnaW5Tb3VyY2VdLFxuICApO1xuXG4gIC8qKiDwn5SM77iPIFJlbW92ZXMgYW4gYWxyZWFkeS1sb2FkZWQgcGx1Z2luOiByZWZ1c2VzIHRoZSBob3N0L3ByaW1hcnkgcGx1Z2luIGFuZCB3aGljaGV2ZXIgcGx1Z2luIG93bnMgdGhlXG4gICAqIGFjdGl2ZSBzZXNzaW9uICh0aGVyZSBpcyBub3RoaW5nIHRvIGZhbGwgYmFjayB0byksIG90aGVyd2lzZSBkZXN0cm95cyBpdHMgbGl2ZSBpbnN0YW5jZXMgdGhlIHNhbWVcbiAgICogd2F5IGByZWxvYWRQbHVnaW5gIGRvZXMsIGRyb3BzIGl0IGZyb20gYGxvYWRlZFBsdWdpbnNgLCBhbmQgZXZpY3RzIGl0cyBtb2R1bGUgbGVhc2UgaW1tZWRpYXRlbHlcbiAgICogKHJhdGhlciB0aGFuIHRoZSBwb29sJ3Mgbm9ybWFsIDMwcyBsaW5nZXIg4oCUIGZyZWVpbmcgaXQgcmlnaHQgYXdheSBpcyB0aGUgcG9pbnQgb2YgYW4gZXhwbGljaXRcbiAgICogdW5pbnN0YWxsKS4gKi9cbiAgY29uc3QgdW5pbnN0YWxsUGx1Z2luID0gdXNlQ2FsbGJhY2soXG4gICAgYXN5bmMgKHBsdWdpbklkOiBzdHJpbmcpID0+IHtcbiAgICAgIGlmIChwbHVnaW5PcEluRmxpZ2h0UmVmLmN1cnJlbnQuaGFzKHBsdWdpbklkKSkgcmV0dXJuO1xuICAgICAgY29uc3QgY3VycmVudCA9IGxvYWRlZFBsdWdpbnNSZWYuY3VycmVudC5maW5kKChlbnRyeSkgPT4gZW50cnkuaGFuZGxlLnBsdWdpbklkID09PSBwbHVnaW5JZCk7XG4gICAgICBpZiAoIWN1cnJlbnQpIHJldHVybjtcbiAgICAgIGlmIChwbHVnaW5JZCA9PT0gcHJpbWFyeVBsdWdpbklkKSB7XG4gICAgICAgIGNvbnNvbGUud2FybihgW0RFQlVHXSByZWZ1c2luZyB0byB1bmluc3RhbGwgdGhlIGhvc3QvcHJpbWFyeSBwbHVnaW46ICR7cGx1Z2luSWR9YCk7XG4gICAgICAgIHJldHVybjtcbiAgICAgIH1cbiAgICAgIGlmIChzZXNzaW9uUmVmLmN1cnJlbnQ/LnBsdWdpbklkID09PSBwbHVnaW5JZCkge1xuICAgICAgICBjb25zb2xlLndhcm4oYFtERUJVR10gcmVmdXNpbmcgdG8gdW5pbnN0YWxsIHRoZSBhY3RpdmUgc2Vzc2lvbidzIHBsdWdpbjogJHtwbHVnaW5JZH1gKTtcbiAgICAgICAgcmV0dXJuO1xuICAgICAgfVxuICAgICAgcGx1Z2luT3BJbkZsaWdodFJlZi5jdXJyZW50LmFkZChwbHVnaW5JZCk7XG4gICAgICB0cnkge1xuICAgICAgICBmb3IgKGNvbnN0IHNwYXduZWQgb2Ygc3Bhd25lZEFwcHNSZWYuY3VycmVudC5maWx0ZXIoKGVudHJ5KSA9PiBlbnRyeS5wbHVnaW5JZCA9PT0gcGx1Z2luSWQpKSB7XG4gICAgICAgICAgYXdhaXQgY3VycmVudC5oYW5kbGUuZGVzdHJveUFwcChzcGF3bmVkLmluc3RhbmNlSWQpLmNhdGNoKCgpID0+IHt9KTtcbiAgICAgICAgfVxuICAgICAgICBjb25zdCBjb250cmlidXRvckluc3RhbmNlSWQgPSBjb250cmlidXRvckluc3RhbmNlc1JlZi5jdXJyZW50LmdldChwbHVnaW5JZCk7XG4gICAgICAgIGlmIChjb250cmlidXRvckluc3RhbmNlSWQgIT0gbnVsbCkge1xuICAgICAgICAgIGF3YWl0IGN1cnJlbnQuaGFuZGxlLmRlc3Ryb3lBcHAoY29udHJpYnV0b3JJbnN0YW5jZUlkKS5jYXRjaCgoKSA9PiB7fSk7XG4gICAgICAgICAgY29udHJpYnV0b3JJbnN0YW5jZXNSZWYuY3VycmVudC5kZWxldGUocGx1Z2luSWQpO1xuICAgICAgICB9XG4gICAgICAgIGlmIChzdHVkaW9Nb2RlICYmIHNlc3Npb25SZWYuY3VycmVudCkge1xuICAgICAgICAgIGNvbnN0IGFjdGl2ZVNlc3Npb24gPSBzZXNzaW9uUmVmLmN1cnJlbnQ7XG4gICAgICAgICAgY29uc3QgY3VycmVudFBhbmVsID0gcGFyc2VQYW5lbFN0YXRlKGFjdGl2ZVNlc3Npb24udmlld1N0YXRlKTtcbiAgICAgICAgICBjb25zdCBkcm9wcGVkID0gY3VycmVudFBhbmVsPy5zcGF3bmVkQXBwcy5maWx0ZXIoKGVudHJ5KSA9PiBlbnRyeS5wbHVnaW5JZCA9PT0gcGx1Z2luSWQpID8/IFtdO1xuICAgICAgICAgIGlmIChjdXJyZW50UGFuZWwgJiYgZHJvcHBlZC5sZW5ndGggPiAwKSB7XG4gICAgICAgICAgICBjb25zdCBzdXJ2aXZpbmdTcGF3bmVkID0gY3VycmVudFBhbmVsLnNwYXduZWRBcHBzLmZpbHRlcigoZW50cnkpID0+IGVudHJ5LnBsdWdpbklkICE9PSBwbHVnaW5JZCk7XG4gICAgICAgICAgICBjb25zdCBhY3RpdmVTcGF3bmVkSWQgPSBjdXJyZW50UGFuZWwuYWN0aXZlU3Bhd25lZElkICYmIGRyb3BwZWQuc29tZSgoZW50cnkpID0+IGVudHJ5LmlkID09PSBjdXJyZW50UGFuZWwuYWN0aXZlU3Bhd25lZElkKSA/IHVuZGVmaW5lZCA6IGN1cnJlbnRQYW5lbC5hY3RpdmVTcGF3bmVkSWQ7XG4gICAgICAgICAgICBjb25zdCBuZXh0UGFuZWwgPSB7IC4uLmN1cnJlbnRQYW5lbCwgc3Bhd25lZEFwcHM6IHN1cnZpdmluZ1NwYXduZWQsIGFjdGl2ZVNwYXduZWRJZCB9O1xuICAgICAgICAgICAgZGlzcGF0Y2goe1xuICAgICAgICAgICAgICB0eXBlOiBcIlNFVF9TRVNTSU9OXCIsXG4gICAgICAgICAgICAgIHZhbHVlOiAobmV4dFNlc3Npb24pID0+IChuZXh0U2Vzc2lvbiA/IHsgLi4ubmV4dFNlc3Npb24sIHZpZXdTdGF0ZTogeyAuLi5uZXh0U2Vzc2lvbi52aWV3U3RhdGUsIHBhbmVsSnNvbjogcGFuZWxKc29uRnJvbVN0YXRlKG5leHRQYW5lbCkgfSB9IDogbmV4dFNlc3Npb24pLFxuICAgICAgICAgICAgfSk7XG4gICAgICAgICAgfVxuICAgICAgICB9XG4gICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJSRU1PVkVfTE9BREVEX1BMVUdJTlwiLCBwbHVnaW5JZCB9KTtcbiAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9QTFVHSU5fU1RBVFVTXCIsIHBsdWdpbklkLCB2YWx1ZTogXCJhdmFpbGFibGVcIiB9KTtcbiAgICAgICAgY3VycmVudC5oYW5kbGUuZGlzcG9zZSgpO1xuICAgICAgICBjb25zdCBtb2R1bGVVcmwgPSBwbHVnaW5Nb2R1bGVVcmxCeUlkUmVmLmN1cnJlbnQuZ2V0KHBsdWdpbklkKTtcbiAgICAgICAgcGx1Z2luTW9kdWxlVXJsQnlJZFJlZi5jdXJyZW50LmRlbGV0ZShwbHVnaW5JZCk7XG4gICAgICAgIGlmIChtb2R1bGVVcmwpIGV2aWN0UGx1Z2luTW9kdWxlKG1vZHVsZVVybCk7XG4gICAgICB9IGZpbmFsbHkge1xuICAgICAgICBwbHVnaW5PcEluRmxpZ2h0UmVmLmN1cnJlbnQuZGVsZXRlKHBsdWdpbklkKTtcbiAgICAgIH1cbiAgICB9LFxuICAgIFtwcmltYXJ5UGx1Z2luSWQsIHN0dWRpb01vZGVdLFxuICApO1xuICAvLyNlbmRyZWdpb24g8J+UjO+4j1BsdWdpblJ1bnRpbWVcblxuICAvLyDwn5Ci77iPIE1lbW9pemVkIG9uIHRoZSByYXcgYHBhbmVsSnNvbmAgc3RyaW5nIChub3QgYHNlc3Npb25gIG9iamVjdCBpZGVudGl0eSwgd2hpY2ggY2h1cm5zIGV2ZXJ5XG4gIC8vIGFjdGlvbikgc28gYSBgc2Vzc2lvbmAgcmVmcmVzaCB0aGF0IGxlYXZlcyBgcGFuZWxKc29uYCB1bnRvdWNoZWQgcmV1c2VzIHRoZSBzYW1lIHBhcnNlZCBgcGFuZWxgXG4gIC8vIG9iamVjdCDigJQgYSBwcmVyZXF1aXNpdGUgZm9yIGFueSBkb3duc3RyZWFtIGB1c2VNZW1vYC9gUmVhY3QubWVtb2Aga2V5ZWQgb24gYHBhbmVsYCB0byBiYWlsLlxuICBjb25zdCBwYW5lbCA9IHVzZU1lbW8oKCkgPT4gKHNlc3Npb24gPyBwYXJzZVBhbmVsU3RhdGUoc2Vzc2lvbi52aWV3U3RhdGUpIDogbnVsbCksIFtzZXNzaW9uPy52aWV3U3RhdGUucGFuZWxKc29uXSk7XG4gIC8qKiDwn5Ca77iPIE1pcnJvcnMgYHBhbmVsPy5zcGF3bmVkQXBwc2AgZm9yIHRoZSB1bm1vdW50LWNsZWFudXAgZWZmZWN0IGJlbG93IOKAlCBzYW1lIHJhdGlvbmFsZSBhc1xuICAgKiBgbG9hZGVkUGx1Z2luc1JlZmA6IG5lZWRzIHRoZSBsYXRlc3QgdmFsdWUgYXQgdGVhcmRvd24gdGltZSB3aXRob3V0IGRlcGVuZGluZyBvbiBpdC4gKi9cbiAgY29uc3Qgc3Bhd25lZEFwcHNSZWYgPSB1c2VSZWY8cmVhZG9ubHkgU3Bhd25lZEFwcEVudHJ5W10+KFtdKTtcbiAgc3Bhd25lZEFwcHNSZWYuY3VycmVudCA9IHBhbmVsPy5zcGF3bmVkQXBwcyA/PyBbXTtcbiAgY29uc3QgYWN0aXZlU3Bhd25lZEVudHJ5ID0gcGFuZWw/LnNwYXduZWRBcHBzLmZpbmQoKGVudHJ5KSA9PiBlbnRyeS5pZCA9PT0gcGFuZWwuYWN0aXZlU3Bhd25lZElkKTtcbiAgY29uc3QgYWN0aXZlQXBwVGl0bGUgPSBhcHBEb2N1bWVudExhYmVsKGFjdGl2ZVNwYXduZWRFbnRyeSA/IHJlc29sdmVEb2N1bWVudEJ5QXBwSWQobG9hZGVkUGx1Z2lucywgYWN0aXZlU3Bhd25lZEVudHJ5LmFwcElkLCBhY3RpdmVTcGF3bmVkRW50cnkuZG9jdW1lbnQsIHVpVGVybWlub2xvZ3kpIDogc2Vzc2lvbiA/IHJlc29sdmVBcHBEb2N1bWVudChzZXNzaW9uLmFwcCwgdWlUZXJtaW5vbG9neSkgOiBbXSk7XG5cbiAgdXNlRWZmZWN0KCgpID0+IHtcbiAgICBzZXNzaW9uUmVmLmN1cnJlbnQgPSBzZXNzaW9uO1xuICB9LCBbc2Vzc2lvbl0pO1xuXG4gIC8vIPCfjpPvuI8gQSBicmFuZC1vd25lZCBpbnRyb2R1Y3Rpb24gZnVsbHkgcmVwbGFjZXMgdGhlIGFwcCdzIG93biAoYWxyZWFkeSBsb2NhbGl6ZWQsIHJlbmRlcmVkIHZlcmJhdGltKTtcbiAgLy8gaXRzIGZpcnN0LXJ1bi1zZWVuIGZsYWcgaXMgYnJhbmQtc2NvcGVkIHNvIHRoZSBicmFuZGVkIHRvdXIgcGxheXMgZXZlbiBvbiBhIGRldmljZSB0aGF0IHNhdyB0aGVcbiAgLy8gdW5icmFuZGVkIG9uZS4gQnJhbmRzIHdpdGggYHJlcGxheUludHJvZHVjdGlvbk9uTG9hZGAgc2tpcCBwZXJzaXN0ZW5jZSBhbmQgYXV0by1zdGFydCBldmVyeSBsb2FkLlxuICBjb25zdCBhY3RpdmVJbnRyb2R1Y3Rpb24gPSBicmFuZD8uaW50cm9kdWN0aW9uID8/IHNlc3Npb24/LmFwcC5pbnRyb2R1Y3Rpb247XG4gIGNvbnN0IGludHJvZHVjdGlvblNlZW5LZXkgPSBzZXNzaW9uID8gKGJyYW5kID8gYCR7YnJhbmQuaWR9OiR7c2Vzc2lvbi5hcHAuaWR9YCA6IHNlc3Npb24uYXBwLmlkKSA6IFwiXCI7XG4gIGNvbnN0IHJlcGxheUludHJvZHVjdGlvbk9uTG9hZCA9IHNob3VsZFJlcGxheUludHJvZHVjdGlvbk9uTG9hZChicmFuZCk7XG4gIGNvbnN0IHBlcnNpc3RJbnRyb2R1Y3Rpb25TZWVuID0gc2hvdWxkUGVyc2lzdEludHJvZHVjdGlvblNlZW4oYnJhbmQpO1xuICBjb25zdCBhY3RpdmVJbnRyb2R1Y3Rpb25SZWYgPSB1c2VSZWYoYWN0aXZlSW50cm9kdWN0aW9uKTtcbiAgYWN0aXZlSW50cm9kdWN0aW9uUmVmLmN1cnJlbnQgPSBhY3RpdmVJbnRyb2R1Y3Rpb247XG5cbiAgLy8g8J+Ok++4jyBBdXRvLXN0YXJ0cyBhbiBhcHAncyBpbnRyb2R1Y3Rpb24gdGhlIGZpcnN0IHRpbWUgaXQgbGF1bmNoZXMgb24gdGhpcyBkZXZpY2UgKG9yIGV2ZXJ5IGxvYWQgd2hlblxuICAvLyB0aGUgYnJhbmQgb3B0cyBpbik7IHJlcGxheWluZyBzdGF5cyBhdmFpbGFibGUgYWZ0ZXJ3YXJkIHZpYSB0aGUgc2hlbGwtb3duZWQgSW50cm9kdWNlIEFwcCBjb21tYW5kLlxuICAvLyDwn46l77iPIE5ldmVyIGF1dG8tc3RhcnRzIHdoaWxlIGEgdHV0b3JpYWwgaXMgYWN0aXZlIChtdXR1YWwgZXhjbHVzaXZpdHkpIOKAlCBgYWN0aXZlVHV0b3JpYWxJZGAgaXMgZGVjbGFyZWRcbiAgLy8ganVzdCBiZWxvdyAodGhlIFR1dG9yaWFsT3JjaGVzdHJhdGlvbiBibG9jaydzIHN0YXRlIHJlc29sdXRpb24pOyByZWFkIHZpYSBgc2hlbGxTdGF0ZS50dXRvcmlhbGBcbiAgLy8gZGlyZWN0bHkgaGVyZSByYXRoZXIgdGhhbiB0aGUgbm90LXlldC1kZWNsYXJlZCBsb2NhbCB0byBhdm9pZCBhIGRlZmluaXRpb24tb3JkZXIgZGVwZW5kZW5jeS5cbiAgdXNlRWZmZWN0KCgpID0+IHtcbiAgICBpZiAoIXNlc3Npb24gfHwgIWFjdGl2ZUludHJvZHVjdGlvbiB8fCBzaGVsbFN0YXRlLnR1dG9yaWFsLmFjdGl2ZVR1dG9yaWFsSWQgIT0gbnVsbCkgcmV0dXJuO1xuICAgIGlmICh0eXBlb2Ygd2luZG93ICE9PSBcInVuZGVmaW5lZFwiICYmIHdpbmRvdy5zZWxmICE9PSB3aW5kb3cudG9wKSByZXR1cm47XG4gICAgaWYgKHN1cHByZXNzQXV0b0ludHJvZHVjdGlvbikgcmV0dXJuO1xuICAgIGlmICghcmVwbGF5SW50cm9kdWN0aW9uT25Mb2FkICYmIHJlYWRTdG9yZWRJbnRyb2R1Y3Rpb25TZWVuKHNjb3BlLnN0b3JhZ2UsIGludHJvZHVjdGlvblNlZW5LZXkpKSByZXR1cm47XG4gICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9JTlRST0RVQ1RJT05fU1RFUFwiLCB2YWx1ZTogMCB9KTtcbiAgfSwgW3Nlc3Npb24/LmFwcC5pZCwgYWN0aXZlSW50cm9kdWN0aW9uLCBpbnRyb2R1Y3Rpb25TZWVuS2V5LCByZXBsYXlJbnRyb2R1Y3Rpb25PbkxvYWQsIHNoZWxsU3RhdGUudHV0b3JpYWwuYWN0aXZlVHV0b3JpYWxJZCwgc3VwcHJlc3NBdXRvSW50cm9kdWN0aW9uXSk7XG5cbiAgLy8g8J+Ope+4jyBaZXJvIHBlci1hcHAgd29yazogYW55IGFwcC9icmFuZCB0aGF0IGRlY2xhcmVzIGB0dXRvcmlhbHNgIGdldHMgc2hlbGwgc3VwcG9ydCBhdXRvbWF0aWNhbGx5LlxuICAvLyBCcmFuZC1vd25lZCB0dXRvcmlhbHMgYXJlIHNob3duIEFMT05HU0lERSB0aGUgYXBwJ3Mgb3duIChuZXZlciByZXBsYWNpbmcgdGhlbSwgdW5saWtlIGBpbnRyb2R1Y3Rpb25gKS5cbiAgY29uc3QgYWN0aXZlVHV0b3JpYWxzID0gdXNlTWVtbygoKTogcmVhZG9ubHkgVHV0b3JpYWxEZWZpbml0aW9uW10gPT4gWy4uLihicmFuZD8udHV0b3JpYWxzID8/IFtdKSwgLi4uKHNlc3Npb24/LmFwcC50dXRvcmlhbHMgPz8gW10pXSwgW2JyYW5kPy50dXRvcmlhbHMsIHNlc3Npb24/LmFwcC50dXRvcmlhbHNdKTtcbiAgLyoqIOKPuu+4jyBUaGUgcmVjb3JkZXIgaXMgZGV2L3N0dWRpby1vbmx5IOKAlCBWaXRlIGFsd2F5cyBkZWZpbmVzIGBpbXBvcnQubWV0YS5lbnYuREVWYDsgZ3VhcmRlZCBmb3Igbm9uLVZpdGUgKGUuZy4gYGJ1biB0ZXN0YCkgZXZhbHVhdGlvbi4gKi9cbiAgY29uc3QgdHV0b3JpYWxSZWNvcmRlckF2YWlsYWJsZSA9IHVzZU1lbW8oKCkgPT4ge1xuICAgIHRyeSB7XG4gICAgICByZXR1cm4gQm9vbGVhbigoaW1wb3J0Lm1ldGEgYXMgdW5rbm93biBhcyB7IHJlYWRvbmx5IGVudj86IHsgcmVhZG9ubHkgREVWPzogYm9vbGVhbiB9IH0pLmVudj8uREVWKTtcbiAgICB9IGNhdGNoIHtcbiAgICAgIHJldHVybiBmYWxzZTtcbiAgICB9XG4gIH0sIFtdKTtcblxuICAvLyDwn6ew77iPIFJlZnMgc28gYHJlZnJlc2hVaWAvYG9uQWN0aW9uYC9gYXBwbHlIb3N0RWZmZWN0c2AgY2FuIHJlYWQgdGhlIGN1cnJlbnQgaG9zdC1vd25lZCBhY3RpdmUgdXRpbGl0eSBhbmRcbiAgLy8gYWN0aXZlIHdpbmRvdyB3aXRob3V0IHJlLWNyZWF0aW5nIHRob3NlIGNhbGxiYWNrcyBvbiBldmVyeSB1dGlsaXR5IHN3aXRjaC5cbiAgY29uc3QgYWN0aXZlVXRpbGl0eUJ5V2luZG93SWRSZWYgPSB1c2VSZWYoYWN0aXZlVXRpbGl0eUJ5V2luZG93SWQpO1xuICBhY3RpdmVVdGlsaXR5QnlXaW5kb3dJZFJlZi5jdXJyZW50ID0gYWN0aXZlVXRpbGl0eUJ5V2luZG93SWQ7XG4gIGNvbnN0IGFjdGl2ZVRvb2xJZFJlZiA9IHVzZVJlZihhY3RpdmVUb29sSWQpO1xuICBhY3RpdmVUb29sSWRSZWYuY3VycmVudCA9IGFjdGl2ZVRvb2xJZDtcbiAgLyoqIPCfp7DvuI8gRGlzcGF0Y2ggKyBzeW5jIHRoZSByZWYgaW1tZWRpYXRlbHkg4oCUIGByZWZyZXNoVWlgIHJlYWRzIHRoZSByZWYgYmVmb3JlIHRoZSBuZXh0IHJlbmRlciwgc28gYVxuICAgKiBiYXJlIGBkaXNwYXRjaChTRVRfQUNUSVZFX1VUSUxJVFkpYCBhbG9uZSBsZWF2ZXMgdGhlIG1hcCBzdGFsZSBhbmQgdGhlIGd1bWJhbGwgbmV2ZXIgYXBwZWFycy4gKi9cbiAgY29uc3Qgc2V0QWN0aXZlVXRpbGl0eUZvcldpbmRvdyA9IHVzZUNhbGxiYWNrKCh3aW5kb3dJZDogc3RyaW5nLCB1dGlsaXR5SWQ6IHN0cmluZyB8IG51bGwpID0+IHtcbiAgICBhY3RpdmVVdGlsaXR5QnlXaW5kb3dJZFJlZi5jdXJyZW50ID0geyAuLi5hY3RpdmVVdGlsaXR5QnlXaW5kb3dJZFJlZi5jdXJyZW50LCBbd2luZG93SWRdOiB1dGlsaXR5SWQgfTtcbiAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0FDVElWRV9VVElMSVRZXCIsIHdpbmRvd0lkLCB1dGlsaXR5SWQgfSk7XG4gIH0sIFtdKTtcbiAgLyoqIPCfp7DvuI8gQ2xlYXIgZXZlcnkgd2luZG93J3MgdXRpbGl0eSBpbiB0aGUgcmVmICsgc3RvcmUgYXQgb25jZSAodG9vbC91dGlsaXR5IG11dHVhbCBleGNsdXNpb24pLiAqL1xuICBjb25zdCBjbGVhckFsbFdpbmRvd1V0aWxpdGllcyA9IHVzZUNhbGxiYWNrKCgpID0+IHtcbiAgICBjb25zdCBuZXh0OiBSZWNvcmQ8c3RyaW5nLCBzdHJpbmcgfCBudWxsPiA9IHsgLi4uYWN0aXZlVXRpbGl0eUJ5V2luZG93SWRSZWYuY3VycmVudCB9O1xuICAgIGZvciAoY29uc3Qgd2luZG93SWQgb2YgT2JqZWN0LmtleXMobmV4dCkpIHtcbiAgICAgIGlmIChuZXh0W3dpbmRvd0lkXSkge1xuICAgICAgICBuZXh0W3dpbmRvd0lkXSA9IG51bGw7XG4gICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfQUNUSVZFX1VUSUxJVFlcIiwgd2luZG93SWQsIHV0aWxpdHlJZDogbnVsbCB9KTtcbiAgICAgIH1cbiAgICB9XG4gICAgYWN0aXZlVXRpbGl0eUJ5V2luZG93SWRSZWYuY3VycmVudCA9IG5leHQ7XG4gIH0sIFtdKTtcbiAgY29uc3QgdG9vbE1lYXN1cmVzQnlUb29sSWRSZWYgPSB1c2VSZWYodG9vbE1lYXN1cmVzQnlUb29sSWQpO1xuICB0b29sTWVhc3VyZXNCeVRvb2xJZFJlZi5jdXJyZW50ID0gdG9vbE1lYXN1cmVzQnlUb29sSWQ7XG4gIGNvbnN0IGFjdGl2ZVdpbmRvd0lkUmVmID0gdXNlUmVmKGFjdGl2ZVdpbmRvd0lkKTtcbiAgYWN0aXZlV2luZG93SWRSZWYuY3VycmVudCA9IGFjdGl2ZVdpbmRvd0lkO1xuICBjb25zdCBhY3Rpb25QYW5lRXhwYW5kZWRCeVdpbmRvd0lkUmVmID0gdXNlUmVmKGFjdGlvblBhbmVFeHBhbmRlZEJ5V2luZG93SWQpO1xuICBhY3Rpb25QYW5lRXhwYW5kZWRCeVdpbmRvd0lkUmVmLmN1cnJlbnQgPSBhY3Rpb25QYW5lRXhwYW5kZWRCeVdpbmRvd0lkO1xuICBjb25zdCBhY3Rpb25QYW5lU3RhZ2VkQXJnc0J5S2V5UmVmID0gdXNlUmVmKGFjdGlvblBhbmVTdGFnZWRBcmdzQnlLZXkpO1xuICBhY3Rpb25QYW5lU3RhZ2VkQXJnc0J5S2V5UmVmLmN1cnJlbnQgPSBhY3Rpb25QYW5lU3RhZ2VkQXJnc0J5S2V5O1xuICBjb25zdCBpbnRyb2R1Y3Rpb25TdGVwSW5kZXhSZWYgPSB1c2VSZWYoaW50cm9kdWN0aW9uU3RlcEluZGV4KTtcbiAgaW50cm9kdWN0aW9uU3RlcEluZGV4UmVmLmN1cnJlbnQgPSBpbnRyb2R1Y3Rpb25TdGVwSW5kZXg7XG4gIGNvbnN0IGludHJvZHVjdGlvbkNvbXBsZXRlZEludGVyYWN0aW9uc1JlZiA9IHVzZVJlZihpbnRyb2R1Y3Rpb25Db21wbGV0ZWRJbnRlcmFjdGlvbnMpO1xuICBpbnRyb2R1Y3Rpb25Db21wbGV0ZWRJbnRlcmFjdGlvbnNSZWYuY3VycmVudCA9IGludHJvZHVjdGlvbkNvbXBsZXRlZEludGVyYWN0aW9ucztcblxuICAvLyDwn46l77iPIEZvcndhcmQtZGVjbGFyZWQgcmVmcyBzbyBgb25BY3Rpb25gIChkZWZpbmVkIGJlbG93LCBiZWZvcmUgdGhlIGZ1bGwgdHV0b3JpYWwgb3JjaGVzdHJhdGlvbiBmdXJ0aGVyXG4gIC8vIGRvd24gdGhpcyBjb21wb25lbnQpIGNhbiBzaGVsbC1pbnRlcmNlcHQgYFNUQVJUX1RVVE9SSUFMX0FDVElPTl9JRGAvYFJFQ09SRF9UVVRPUklBTF9BQ1RJT05fSURgXG4gIC8vIHdpdGhvdXQgYSBkZWZpbml0aW9uLW9yZGVyIGN5Y2xlIOKAlCBtaXJyb3JzIHRoZSBgb25BY3Rpb25SZWZgIHBhdHRlcm4gdXNlZCB0aGUgb3RoZXIgd2F5IGFyb3VuZC5cbiAgLy8gUG9wdWxhdGVkIGJ5IHRoZSBUdXRvcmlhbE9yY2hlc3RyYXRpb24gYmxvY2sncyBlZmZlY3Qgb25jZSB0aGUgcmVhbCBjYWxsYmFja3MgZXhpc3QuXG4gIGNvbnN0IHN0YXJ0VHV0b3JpYWxSZWYgPSB1c2VSZWY8KHR1dG9yaWFsSWQ6IHN0cmluZykgPT4gdm9pZD4oKCkgPT4ge30pO1xuICBjb25zdCBzdG9wVHV0b3JpYWxSZWYgPSB1c2VSZWY8KCkgPT4gdm9pZD4oKCkgPT4ge30pO1xuICBjb25zdCB0b2dnbGVUdXRvcmlhbFJlY29yZGluZ1JlZiA9IHVzZVJlZjwoKSA9PiB2b2lkPigoKSA9PiB7fSk7XG4gIC8qKiDwn6ey77iPIFRydWUgZm9yIHRoZSBkdXJhdGlvbiBvZiBhbnkgZGlyZWN0b3Ivc2Vlay9jb252ZXJnZS1kcml2ZW4gZGlzcGF0Y2gg4oCUIGBvbkFjdGlvbmAncyBkZXZpYXRpb25cbiAgICogY2hlY2sgYmVsb3cgc2tpcHMgc2V0dGluZyBgZGV2aWF0ZWRgL2F1dG8tcGF1c2luZyBmb3IgYW55dGhpbmcgc3RhbXBlZCB3aGlsZSB0aGlzIGlzIHRydWUsIG1pcnJvcmluZ1xuICAgKiBob3cgdGhlIGludHJvZHVjdGlvbiBtZWNoYW5pc20ncyBvd24gaW50ZXJjZXB0aW9uIGRpc3Rpbmd1aXNoZXMgc2hlbGwtb3JpZ2luYXRlZCBmcm9tIHVzZXItb3JpZ2luYXRlZFxuICAgKiBhY3Rpdml0eS4gTmV2ZXIgcmVhZCBkdXJpbmcgcmVuZGVyLCBvbmx5IGluc2lkZSBldmVudCBjYWxsYmFja3Mg4oCUIGEgcGxhaW4gbXV0YWJsZSByZWYgaXMgY29ycmVjdC4gKi9cbiAgY29uc3QgdHV0b3JpYWxEcml2ZW5SZWYgPSB1c2VSZWYoZmFsc2UpO1xuICBjb25zdCB0dXRvcmlhbFBsYXlpbmdSZWYgPSB1c2VSZWYodHV0b3JpYWxQbGF5aW5nKTtcbiAgdHV0b3JpYWxQbGF5aW5nUmVmLmN1cnJlbnQgPSB0dXRvcmlhbFBsYXlpbmc7XG4gIGNvbnN0IHR1dG9yaWFsUmVjb3JkaW5nUmVmID0gdXNlUmVmKHR1dG9yaWFsUmVjb3JkaW5nKTtcbiAgdHV0b3JpYWxSZWNvcmRpbmdSZWYuY3VycmVudCA9IHR1dG9yaWFsUmVjb3JkaW5nO1xuICAvKiog4o+677iPIE5vbi1udWxsIHdoaWxlIGFybWVkIOKAlCBtdXRhdGVkIGJ5IGB0b2dnbGVUdXRvcmlhbFJlY29yZGluZ2AgKGRlZmluZWQgaW4gdGhlIFR1dG9yaWFsT3JjaGVzdHJhdGlvbiBibG9jayBiZWxvdyksIHJlYWQvYXBwZW5kZWQtdG8gYnkgYG9uQWN0aW9uYCdzIHJlY29yZGVyIHRhcCByaWdodCBiZWxvdy4gKi9cbiAgY29uc3QgdHV0b3JpYWxSZWNvcmRlclJlZiA9IHVzZVJlZjxUdXRvcmlhbFJlY29yZGVyIHwgbnVsbD4obnVsbCk7XG4gIGNvbnN0IHNoZWxsU3RhdGVSZWYgPSB1c2VSZWYoc2hlbGxTdGF0ZSk7XG4gIHNoZWxsU3RhdGVSZWYuY3VycmVudCA9IHNoZWxsU3RhdGU7XG5cbiAgLyoqIPCfjpPvuI8gRW5kcyB0aGUgYWN0aXZlIGludHJvZHVjdGlvbiDigJQgcGVyc2lzdHMgdGhlIHNlZW4gZmxhZyB3aGVuIGNvbmZpZ3VyZWQsIGFuZCBvbiBzdWNjZXNzZnVsXG4gICAqIGNvbXBsZXRpb24gKERvbmUgLyBsYXN0IGludGVyYWN0aW9uKSBmaXJlcyB0aGUgdG91ci1maW5hbGUge0BsaW5rIGNlbGVicmF0ZUFsbEVsZW1lbnRzfSBzdGFtcFxuICAgKiBhY3Jvc3MgZXZlcnkgbW91bnRlZCBVSSBlbGVtZW50LiBTa2lwL2VzY2FwZSBwYXNzZXMgYGNvbXBsZXRlZDogZmFsc2VgIGFuZCBkb2VzIG5vdCBjZWxlYnJhdGUuICovXG4gIGNvbnN0IGRpc21pc3NJbnRyb2R1Y3Rpb24gPSB1c2VDYWxsYmFjayhcbiAgICAoY29tcGxldGVkOiBib29sZWFuKSA9PiB7XG4gICAgICBpZiAoY29tcGxldGVkICYmIHNjb3BlLnJvb3RSZWYuY3VycmVudCkgY2VsZWJyYXRlQWxsRWxlbWVudHMoQ0VMRUJSQVRFX1NUQU1QX0RVUkFUSU9OX01TLCBzY29wZS5yb290UmVmLmN1cnJlbnQpO1xuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9JTlRST0RVQ1RJT05fU1RFUFwiLCB2YWx1ZTogbnVsbCB9KTtcbiAgICAgIGlmIChwZXJzaXN0SW50cm9kdWN0aW9uU2Vlbikgd3JpdGVTdG9yZWRJbnRyb2R1Y3Rpb25TZWVuKHNjb3BlLnN0b3JhZ2UsIGludHJvZHVjdGlvblNlZW5LZXkpO1xuICAgIH0sXG4gICAgW2ludHJvZHVjdGlvblNlZW5LZXksIHBlcnNpc3RJbnRyb2R1Y3Rpb25TZWVuXSxcbiAgKTtcblxuICAvKiog8J+Ok++4jyBTaGFyZWQgc3RlcC1jb21wbGV0ZSBwYXRoOiBmaXJlcyBvbmNlIGV2ZXJ5IGludGVyYWN0aW9uLWdhdGVkIHN0ZXAncyBgaW50ZXJhY3Rpb25zYCBhcmUgYWxsIGRvbmVcbiAgICogKHZpYSBgY29tcGxldGVJbnRyb2R1Y3Rpb25JbnRlcmFjdGlvbmAgYmVsb3cpLCBjZWxlYnJhdGluZyBgaW50cm9kdWNlYCBvbiB0b3Agb2YgZWFjaCBpbnRlcmFjdGlvbidzXG4gICAqIG93biBjZWxlYnJhdGlvbiwgdGhlbiBhZHZhbmNlcyBvciBmaW5pc2hlcyB0aGUgdG91ci4gRmluaXNoaW5nIHRoZSBsYXN0IHN0ZXAgY2VsZWJyYXRlcyBldmVyeSBVSVxuICAgKiBlbGVtZW50IHZpYSB7QGxpbmsgZGlzbWlzc0ludHJvZHVjdGlvbn0odHJ1ZSkgaW5zdGVhZCBvZiBvbmx5IHRoZSBpbnRyb2R1Y2UgdGFyZ2V0LiBgY2VsZWJyYXRlT3ZlcnJpZGVgXG4gICAqICh0aHJlYWRlZCB0aHJvdWdoIGZyb20gYGNvbXBsZXRlSW50cm9kdWN0aW9uSW50ZXJhY3Rpb25gKSBuYXJyb3dzIHRoaXMgdG8gdGhlIG9uZSBlbGVtZW50IHJlc3BvbnNpYmxlXG4gICAqIGZvciB0aGUganVzdC1jb21wbGV0ZWQgaW50ZXJhY3Rpb24g4oCUIGUuZy4gdGhlIHNwZWNpZmljIDNEIHdpbmRvdyBwYW5lIHRoYXQgd2FzIG9yYml0ZWQg4oCUIGluc3RlYWQgb2ZcbiAgICogZXZlcnkgZWxlbWVudCBhbGlhc2VkIHRvIHRoZSBzdGVwJ3MgYGludHJvZHVjZWAga2luZCAoZXZlcnkgb3BlbiBwYW5lIG9mIHRoYXQgd2luZG93IGtpbmQpLiAqL1xuICBjb25zdCBhZHZhbmNlSW50cm9kdWN0aW9uQnlEb2luZyA9IHVzZUNhbGxiYWNrKFxuICAgIChjZWxlYnJhdGVPdmVycmlkZT86IHN0cmluZykgPT4ge1xuICAgICAgY29uc3Qgc3RlcEluZGV4ID0gaW50cm9kdWN0aW9uU3RlcEluZGV4UmVmLmN1cnJlbnQ7XG4gICAgICBjb25zdCBpbnRyb2R1Y3Rpb24gPSBhY3RpdmVJbnRyb2R1Y3Rpb25SZWYuY3VycmVudDtcbiAgICAgIGlmIChzdGVwSW5kZXggPT0gbnVsbCB8fCAhaW50cm9kdWN0aW9uKSByZXR1cm47XG4gICAgICBjb25zdCBzdGVwID0gaW50cm9kdWN0aW9uLnN0ZXBzW3N0ZXBJbmRleF07XG4gICAgICBpZiAoc3RlcEluZGV4ID49IGludHJvZHVjdGlvbi5zdGVwcy5sZW5ndGggLSAxKSB7XG4gICAgICAgIGRpc21pc3NJbnRyb2R1Y3Rpb24odHJ1ZSk7XG4gICAgICAgIHJldHVybjtcbiAgICAgIH1cbiAgICAgIGNvbnN0IGNlbGVicmF0ZUlkID0gY2VsZWJyYXRlT3ZlcnJpZGUgPz8gc3RlcD8uaW50cm9kdWNlO1xuICAgICAgaWYgKHN0ZXAgJiYgKHN0ZXAuaW50ZXJhY3Rpb25zID8/IFtdKS5sZW5ndGggPiAwICYmIGNlbGVicmF0ZUlkICYmIHNjb3BlLnJvb3RSZWYuY3VycmVudCkgY2VsZWJyYXRlRWxlbWVudHMoZWxlbWVudElkU2VsZWN0b3IoY2VsZWJyYXRlSWQpLCBDRUxFQlJBVEVfU1RBTVBfRFVSQVRJT05fTVMsIHNjb3BlLnJvb3RSZWYuY3VycmVudCk7XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0lOVFJPRFVDVElPTl9TVEVQXCIsIHZhbHVlOiBzdGVwSW5kZXggKyAxIH0pO1xuICAgIH0sXG4gICAgW2Rpc21pc3NJbnRyb2R1Y3Rpb25dLFxuICApO1xuXG4gIC8qKiDinIXvuI8gQ29tcGxldGVzIHRoZSBmaXJzdCBub3QteWV0LWRvbmUgaW50ZXJhY3Rpb24gb2YgdGhlIGFjdGl2ZSBzdGVwIG1hdGNoaW5nIGBtYXRjaGVzYCAocmVzcGVjdGluZ1xuICAgKiBgc3RlcC5vcmRlcmVkYCDigJQgb25seSB0aGUgbmV4dCBpbi1vcmRlciBpbnRlcmFjdGlvbiBtYXkgY29tcGxldGUpLCBjZWxlYnJhdGVzIGl0cyB0YXJnZXQgZWxlbWVudCwgYW5kXG4gICAqIGFkdmFuY2VzIHRoZSBzdGVwIG9uY2UgZXZlcnkgaW50ZXJhY3Rpb24gaXMgZG9uZS4gTWlycm9ycyB0aGUgd2dwdSBzaGVsbCdzXG4gICAqIGBjaHJvbWVfdG91cl9jb21wbGV0ZV9pbnRlcmFjdGlvbmAuIGBjZWxlYnJhdGVPdmVycmlkZWAg4oCUIHBhc3NlZCBieSBjYWxsZXJzIHRoYXQga25vdyBleGFjdGx5IHdoaWNoXG4gICAqIERPTSBlbGVtZW50IGNhdXNlZCB0aGUgY29tcGxldGlvbiAoZS5nLiB0aGUgZ2VzdHVyZSBpbnRlcmNlcHQga25vd3MgdGhlIG9uZSB3aW5kb3cgcGFuZSB0aGF0IHdhc1xuICAgKiBhY3R1YWxseSBvcmJpdGVkKSDigJQgdGFrZXMgcHJlY2VkZW5jZSBvdmVyIGBpbnRlcmFjdGlvbi5jZWxlYnJhdGUgPz8gc3RlcC5pbnRyb2R1Y2VgLiBXaXRob3V0IGl0LCBhXG4gICAqIHdpbmRvdy1raW5kIGBpbnRyb2R1Y2VgL2BjZWxlYnJhdGVgIGlkIHdvdWxkIGNlbGVicmF0ZSBldmVyeSBwYW5lIGFsaWFzZWQgdG8gdGhhdCBraW5kLCBub3QganVzdCB0aGVcbiAgICogb25lIHRoYXQgY29tcGxldGVkIHRoZSBpbnRlcmFjdGlvbi4gKi9cbiAgY29uc3QgY29tcGxldGVJbnRyb2R1Y3Rpb25JbnRlcmFjdGlvbiA9IHVzZUNhbGxiYWNrKFxuICAgIChtYXRjaGVzOiAoaW50ZXJhY3Rpb246IEludHJvZHVjdGlvbkludGVyYWN0aW9uKSA9PiBib29sZWFuLCBjZWxlYnJhdGVPdmVycmlkZT86IHN0cmluZykgPT4ge1xuICAgICAgY29uc3Qgc3RlcEluZGV4ID0gaW50cm9kdWN0aW9uU3RlcEluZGV4UmVmLmN1cnJlbnQ7XG4gICAgICBjb25zdCBpbnRyb2R1Y3Rpb24gPSBhY3RpdmVJbnRyb2R1Y3Rpb25SZWYuY3VycmVudDtcbiAgICAgIGlmIChzdGVwSW5kZXggPT0gbnVsbCB8fCAhaW50cm9kdWN0aW9uKSByZXR1cm47XG4gICAgICBjb25zdCBzdGVwID0gaW50cm9kdWN0aW9uLnN0ZXBzW3N0ZXBJbmRleF07XG4gICAgICBpZiAoIXN0ZXAgfHwgKHN0ZXAuaW50ZXJhY3Rpb25zID8/IFtdKS5sZW5ndGggPT09IDApIHJldHVybjtcbiAgICAgIGNvbnN0IGNvbXBsZXRlZCA9IGludHJvZHVjdGlvbkNvbXBsZXRlZEludGVyYWN0aW9uc1JlZi5jdXJyZW50O1xuICAgICAgY29uc3QgaW50ZXJhY3Rpb25zID0gc3RlcC5pbnRlcmFjdGlvbnMgPz8gW107XG4gICAgICBjb25zdCBpbmRleCA9IGludGVyYWN0aW9ucy5maW5kSW5kZXgoKGludGVyYWN0aW9uLCBpKSA9PiAhY29tcGxldGVkLmluY2x1ZGVzKGkpICYmIG1hdGNoZXMoaW50ZXJhY3Rpb24pKTtcbiAgICAgIGlmIChpbmRleCA8IDApIHJldHVybjtcbiAgICAgIGlmIChzdGVwLm9yZGVyZWQgJiYgaW5kZXggIT09IGNvbXBsZXRlZC5sZW5ndGgpIHJldHVybjtcbiAgICAgIGNvbnN0IGNlbGVicmF0ZUlkID0gY2VsZWJyYXRlT3ZlcnJpZGUgPz8gaW50ZXJhY3Rpb25zW2luZGV4XS5jZWxlYnJhdGUgPz8gc3RlcC5pbnRyb2R1Y2U7XG4gICAgICBpZiAoY2VsZWJyYXRlSWQgJiYgc2NvcGUucm9vdFJlZi5jdXJyZW50KSBjZWxlYnJhdGVFbGVtZW50cyhlbGVtZW50SWRTZWxlY3RvcihjZWxlYnJhdGVJZCksIENFTEVCUkFURV9TVEFNUF9EVVJBVElPTl9NUywgc2NvcGUucm9vdFJlZi5jdXJyZW50KTtcbiAgICAgIGludHJvZHVjdGlvbkNvbXBsZXRlZEludGVyYWN0aW9uc1JlZi5jdXJyZW50ID0gWy4uLmNvbXBsZXRlZCwgaW5kZXhdO1xuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIkNPTVBMRVRFX0lOVFJPRFVDVElPTl9JTlRFUkFDVElPTlwiLCBpbmRleCB9KTtcbiAgICAgIGlmIChpbnRyb2R1Y3Rpb25Db21wbGV0ZWRJbnRlcmFjdGlvbnNSZWYuY3VycmVudC5sZW5ndGggPj0gaW50ZXJhY3Rpb25zLmxlbmd0aCkgYWR2YW5jZUludHJvZHVjdGlvbkJ5RG9pbmcoY2VsZWJyYXRlT3ZlcnJpZGUpO1xuICAgIH0sXG4gICAgW2FkdmFuY2VJbnRyb2R1Y3Rpb25CeURvaW5nXSxcbiAgKTtcbiAgLy8g8J+Om++4jyBTbyB0aGUgY29tbWFuZC1jYXRlZ29yeSBsZWF2ZXMnIGxhemlseS1yZXNvbHZlZCB0cmVlIGNvbnRlbnQgKGJ1aWx0IG9uY2UgcGVyIHJlc29sdmVkLWNvbW1hbmRzXG4gIC8vIGNoYW5nZSwgbm90IHBlciBrZXlzdHJva2Ug4oCUIHNlZSBgYnVpbGRDb21tYW5kQ2F0ZWdvcnlUYWJzYCkgY2FuIHJlYWQgdGhlIGxhdGVzdCBleHBhbmQvc3RhZ2VkLWFyZ1xuICAvLyBzdGF0ZSB3aXRob3V0IGJlY29taW5nIGEgYGRlZmF1bHREb2NrYCBtZW1vIGRlcGVuZGVuY3ksIHdoaWNoIHdvdWxkIG90aGVyd2lzZSBwZXJzaXN0LXdyaXRlIHRoZSBkb2NrXG4gIC8vIHNrZWxldG9uIG9uIGV2ZXJ5IGtleXN0cm9rZSB3aGlsZSBzdGFnaW5nIGEgY29tbWFuZCBhcmd1bWVudC5cbiAgY29uc3QgZXhwYW5kZWRDb21tYW5kSWRSZWYgPSB1c2VSZWYoZXhwYW5kZWRDb21tYW5kSWQpO1xuICBleHBhbmRlZENvbW1hbmRJZFJlZi5jdXJyZW50ID0gZXhwYW5kZWRDb21tYW5kSWQ7XG4gIGNvbnN0IGNvbW1hbmRTdGFnZWRBcmdzQnlDb21tYW5kSWRSZWYgPSB1c2VSZWYoY29tbWFuZFN0YWdlZEFyZ3NCeUNvbW1hbmRJZCk7XG4gIGNvbW1hbmRTdGFnZWRBcmdzQnlDb21tYW5kSWRSZWYuY3VycmVudCA9IGNvbW1hbmRTdGFnZWRBcmdzQnlDb21tYW5kSWQ7XG5cbiAgLyoqIPCfm6DvuI8gT3ZlcmxheXMgdGhlIG1vZGUtbGV2ZWwgaG9zdC1vd25lZCBgYWN0aXZlVG9vbElkYCBvbnRvIGEgdmlldyBzdGF0ZSBhdCBwbHVnaW4tY2FsbCB0aW1lIOKAlFxuICAgKiBtaXJyb3JzIGBpbmplY3RBY3RpdmVVdGlsaXR5YCBidXQgaXMgd2luZG93bGVzcyAoYSB0b29sIGlzIHNjb3BlZCB0byB0aGUgYWN0aXZlIG1vZGUsIG5vdCBhIHdpbmRvdykuICovXG4gIGNvbnN0IGluamVjdEFjdGl2ZVRvb2wgPSB1c2VDYWxsYmFjaygodmlld1N0YXRlOiBWaWV3TW9kZWwpOiBWaWV3TW9kZWwgPT4ge1xuICAgIGNvbnN0IHRvb2xJZCA9IGFjdGl2ZVRvb2xJZFJlZi5jdXJyZW50ID8/IHVuZGVmaW5lZDtcbiAgICByZXR1cm4gdmlld1N0YXRlLmFjdGl2ZVRvb2xJZCA9PT0gdG9vbElkID8gdmlld1N0YXRlIDogeyAuLi52aWV3U3RhdGUsIGFjdGl2ZVRvb2xJZDogdG9vbElkIH07XG4gIH0sIFtdKTtcblxuICAvKiog8J+nsO+4jyBPdmVybGF5cyB0aGUgYWN0aXZlIHdpbmRvdydzIGhvc3Qtb3duZWQgYGFjdGl2ZVV0aWxpdHlJZGAgKGFuZCB0aGUgbW9kZSdzIGBhY3RpdmVUb29sSWRgKSBvbnRvIGEgdmlldyBzdGF0ZSBhdCBwbHVnaW4tY2FsbCB0aW1lLiAqL1xuICBjb25zdCBpbmplY3RBY3RpdmVVdGlsaXR5ID0gdXNlQ2FsbGJhY2soKHZpZXdTdGF0ZTogVmlld01vZGVsLCB3aW5kb3dJZD86IHN0cmluZyB8IG51bGwpOiBWaWV3TW9kZWwgPT4ge1xuICAgIGNvbnN0IGtleSA9IHdpbmRvd0lkID8/IGFjdGl2ZVdpbmRvd0lkUmVmLmN1cnJlbnQ7XG4gICAgY29uc3QgdXRpbGl0eUlkID0ga2V5ID8gKGFjdGl2ZVV0aWxpdHlCeVdpbmRvd0lkUmVmLmN1cnJlbnRba2V5XSA/PyB1bmRlZmluZWQpIDogdW5kZWZpbmVkO1xuICAgIGNvbnN0IHdpdGhVdGlsaXR5ID0gdmlld1N0YXRlLmFjdGl2ZVV0aWxpdHlJZCA9PT0gdXRpbGl0eUlkID8gdmlld1N0YXRlIDogeyAuLi52aWV3U3RhdGUsIGFjdGl2ZVV0aWxpdHlJZDogdXRpbGl0eUlkIH07XG4gICAgcmV0dXJuIGluamVjdEFjdGl2ZVRvb2wod2l0aFV0aWxpdHkpO1xuICB9LCBbaW5qZWN0QWN0aXZlVG9vbF0pO1xuXG4gIHVzZUVmZmVjdCgoKSA9PiB7XG4gICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9TWU5DX0JBQ0tCT05FX1VSSVwiLCB2YWx1ZTogbnVsbCB9KTtcbiAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1NZTkNfQ0FSRF9LSU5EXCIsIHZhbHVlOiBudWxsIH0pO1xuICB9LCBbcGFuZWw/LmFjdGl2ZVNwYXduZWRJZCwgc2Vzc2lvbiwgc3R1ZGlvTW9kZV0pO1xuXG4gIC8qKiDwn5Ca77iPIFRoZSByZWxheSBhIGRvY3VtZW50J3MgYHJlZ2lzdGVyUGx1Z2luQmFja2JvbmVSb3V0ZWAgZW50cnkgdXNlcyDigJQgZm9yd2FyZHMgYSBwbHVnaW4ncyBvdXRib3VuZFxuICAgKiBiYWNrYm9uZSBieXRlcyBpbnRvIFRISVMgc2hlbGwncyBvd24gYmFja2JvbmUgd29ya2VyLiBSZWdpc3RlcmVkIHBlciBvcGVuIGRvY3VtZW50IChpblxuICAgKiBgb3BlbkRvY3VtZW50YC9gY2xvc2VEb2N1bWVudGAgYmVsb3cpIHJhdGhlciB0aGFuIG9uY2UgZm9yIHRoZSB3aG9sZSBzaGVsbDogdGhlIG9sZCBzaW5nbGVcbiAgICogcGFnZS1nbG9iYWwgcmVsYXkgc2xvdCAoYHNldFBsdWdpbkJhY2tib25lT3V0Ym91bmRSZWxheWApIG1lYW50IGEgc2Vjb25kIG1vdW50ZWQgc2hlbGwgc2lsZW50bHlcbiAgICogc3RvbGUgZXZlcnkgZG9jdW1lbnQncyBvdXRib3VuZCByb3V0aW5nLCB0aGVuIHNldmVyZWQgaXQgZW50aXJlbHkgb24gdGhhdCBzaGVsbCdzIHVubW91bnQuICovXG4gIGNvbnN0IHJlbGF5UGx1Z2luQmFja2JvbmVNZXNzYWdlID0gdXNlQ2FsbGJhY2soKHVyaTogc3RyaW5nLCBtZXNzYWdlQnl0ZXM6IFVpbnQ4QXJyYXkpID0+IHtcbiAgICBjb25zdCBkb2N1bWVudElkID0gdXJpLnN0YXJ0c1dpdGgoXCJhY3RvcjovL1wiKSA/IHVyaS5zbGljZShcImFjdG9yOi8vXCIubGVuZ3RoKSA6IG51bGw7XG4gICAgaWYgKCFkb2N1bWVudElkKSByZXR1cm47XG4gICAgY29uc3Qgd29ya2VyID0gYmFja2JvbmVXb3JrZXJSZWYuY3VycmVudDtcbiAgICBpZiAoIXdvcmtlcikgcmV0dXJuO1xuICAgIGxldCBhY3Rvck1lc3NhZ2U6IERvY3VtZW50QWN0b3JNc2c7XG4gICAgdHJ5IHtcbiAgICAgIGNvbnN0IHBhcnNlZCA9IGRlY29kZUJhY2tib25lTWVzc2FnZShtZXNzYWdlQnl0ZXMpO1xuICAgICAgaWYgKHBhcnNlZC5raW5kID09PSBcIm9wZXJhdGlvbnNcIikge1xuICAgICAgICBhY3Rvck1lc3NhZ2UgPSB7XG4gICAgICAgICAga2luZDogXCJsb2NhbE9wZXJhdGlvbnNcIixcbiAgICAgICAgICBlbnZlbG9wZXM6IHBhcnNlZC5lbnZlbG9wZXMubWFwKChlbnZlbG9wZSkgPT4gb3BlcmF0aW9uRW52ZWxvcGVGcm9tV2lyZShlbnZlbG9wZSkpLFxuICAgICAgICB9O1xuICAgICAgfSBlbHNlIGlmIChwYXJzZWQua2luZCA9PT0gXCJzbmFwc2hvdFwiKSB7XG4gICAgICAgIGFjdG9yTWVzc2FnZSA9IHsga2luZDogXCJsb2NhbFNuYXBzaG90XCIsIHBhY2s6IEFycmF5LmZyb20ocGFyc2VkLnBhY2spLCBzcHI6IEFycmF5LmZyb20ocGFyc2VkLnNwcikgfTtcbiAgICAgIH0gZWxzZSB7XG4gICAgICAgIHJldHVybjtcbiAgICAgIH1cbiAgICB9IGNhdGNoIHtcbiAgICAgIHJldHVybjtcbiAgICB9XG4gICAgY29uc3QgcmVxdWVzdDogQmFja2JvbmVXb3JrZXJSZXF1ZXN0ID0geyBraW5kOiBcInNlbmRcIiwgZG9jdW1lbnRJZCwgbWVzc2FnZTogYWN0b3JNZXNzYWdlIH07XG4gICAgd29ya2VyLnBvc3RNZXNzYWdlKHsgd2lyZTogZW5jb2RlQmFja2JvbmVXb3JrZXJSZXF1ZXN0KHJlcXVlc3QpIH0pO1xuICB9LCBbXSk7XG5cbiAgdXNlRWZmZWN0KCgpID0+IHtcbiAgICBjb25zdCB3b3JrZXIgPSBiYWNrYm9uZVdvcmtlclJlZi5jdXJyZW50O1xuICAgIHJldHVybiAoKSA9PiB3b3JrZXI/LnRlcm1pbmF0ZSgpO1xuICB9LCBbXSk7XG5cbiAgdXNlRWZmZWN0KCgpID0+IHtcbiAgICByZXR1cm4gKCkgPT4ge1xuICAgICAgZm9yIChjb25zdCB1bnJlZ2lzdGVyIG9mIHBsdWdpbkJhY2tib25lUm91dGVVbnJlZ2lzdGVyc1JlZi5jdXJyZW50LnZhbHVlcygpKSB1bnJlZ2lzdGVyKCk7XG4gICAgICBwbHVnaW5CYWNrYm9uZVJvdXRlVW5yZWdpc3RlcnNSZWYuY3VycmVudC5jbGVhcigpO1xuICAgICAgY29uc3QgcHJpbWFyeSA9IHNlc3Npb25SZWYuY3VycmVudDtcbiAgICAgIGlmIChwcmltYXJ5KSB7XG4gICAgICAgIGNvbnN0IHBsdWdpbiA9IGxvYWRlZFBsdWdpbnNSZWYuY3VycmVudC5maW5kKChlbnRyeSkgPT4gZW50cnkuaGFuZGxlLnBsdWdpbklkID09PSBwcmltYXJ5LnBsdWdpbklkKT8uaGFuZGxlO1xuICAgICAgICB2b2lkIHBsdWdpbj8uZGVzdHJveUFwcChwcmltYXJ5Lmluc3RhbmNlSWQpLmNhdGNoKCgpID0+IHt9KTtcbiAgICAgIH1cbiAgICAgIC8vIPCfqrbvuI8gQ2xvc2VzIHRoZSBwcmV2aW91c2x5LWRvY3VtZW50ZWQgV2F2ZS0xIGdhcDogc3R1ZGlvLW1vZGUgc3Bhd25lZCBhcHBzIChgcGFuZWwuc3Bhd25lZEFwcHNgKVxuICAgICAgLy8gYW5kIGV4dGVybmFsLXNsb3QgY29udHJpYnV0b3IgaW5zdGFuY2VzIChgY29udHJpYnV0b3JJbnN0YW5jZXNSZWZgKSBlYWNoIGhvbGQgYSBsaXZlIHBsdWdpblxuICAgICAgLy8gaW5zdGFuY2UgdG9vIOKAlCBsZWF2aW5nIHRoZW0gcnVubmluZyBwYXN0IHNoZWxsIHVubW91bnQgd2FzIHB1cmUgbGVha2VkIG1lbW9yeSAoc2VlXG4gICAgICAvLyBSRURVQ0UtREVNT05TVFJBVE9SLUlETEUtTUVNT1JZLUZPT1RQUklOVCkuIEJlc3QtZWZmb3J0OiBhbiBpbnN0YW5jZSB0aGUgZ3Vlc3QgYWxyZWFkeSBkcm9wcGVkLFxuICAgICAgLy8gb3Igd2hvc2UgcGx1Z2luIGFscmVhZHkgZGlzcG9zZWQsIGp1c3QgcmVqZWN0cyBoYXJtbGVzc2x5IHZpYSB0aGUgc2FtZSBgLmNhdGNoKCgpID0+IHt9KWBcbiAgICAgIC8vIHBhdHRlcm4gdGhlIHByaW1hcnkgc2Vzc2lvbidzIG93biBkZXN0cm95IGFscmVhZHkgdXNlZCBhYm92ZS5cbiAgICAgIGZvciAoY29uc3Qgc3Bhd25lZCBvZiBzcGF3bmVkQXBwc1JlZi5jdXJyZW50KSB7XG4gICAgICAgIGNvbnN0IHBsdWdpbiA9IGxvYWRlZFBsdWdpbnNSZWYuY3VycmVudC5maW5kKChlbnRyeSkgPT4gZW50cnkuaGFuZGxlLnBsdWdpbklkID09PSBzcGF3bmVkLnBsdWdpbklkKT8uaGFuZGxlO1xuICAgICAgICB2b2lkIHBsdWdpbj8uZGVzdHJveUFwcChzcGF3bmVkLmluc3RhbmNlSWQpLmNhdGNoKCgpID0+IHt9KTtcbiAgICAgIH1cbiAgICAgIGZvciAoY29uc3QgW3BsdWdpbklkLCBpbnN0YW5jZUlkXSBvZiBjb250cmlidXRvckluc3RhbmNlc1JlZi5jdXJyZW50KSB7XG4gICAgICAgIGNvbnN0IHBsdWdpbiA9IGxvYWRlZFBsdWdpbnNSZWYuY3VycmVudC5maW5kKChlbnRyeSkgPT4gZW50cnkuaGFuZGxlLnBsdWdpbklkID09PSBwbHVnaW5JZCk/LmhhbmRsZTtcbiAgICAgICAgdm9pZCBwbHVnaW4/LmRlc3Ryb3lBcHAoaW5zdGFuY2VJZCkuY2F0Y2goKCkgPT4ge30pO1xuICAgICAgfVxuICAgICAgY29udHJpYnV0b3JJbnN0YW5jZXNSZWYuY3VycmVudC5jbGVhcigpO1xuICAgICAgZm9yIChjb25zdCBlbnRyeSBvZiBsb2FkZWRQbHVnaW5zUmVmLmN1cnJlbnQpIGVudHJ5LmhhbmRsZS5kaXNwb3NlKCk7XG4gICAgfTtcbiAgfSwgW10pO1xuXG4gIHVzZUVmZmVjdCgoKSA9PiB7XG4gICAgLy8g8J+Qmu+4jyBPbmx5IHRoZSBwYWdlLW93bmluZyBzaGVsbCBtYXkgd3JpdGUgdGhlIGJyb3dzZXIgdGFiIHRpdGxlIOKAlCBhbiBlbWJlZGRlZCBzaGVsbCAoZS5nLiBvbmVcbiAgICAvLyBkZW1vbnN0cmF0b3IgcGFuZSkgc2hhcmluZyB0aGUgcGFnZSB3aXRoIG90aGVycyBtdXN0IG5vdCBmaWdodCB0aGVtIG92ZXIgaXQuXG4gICAgaWYgKCFzY29wZS5vd25zUGFnZSkgcmV0dXJuO1xuICAgIGlmIChicmFuZCkge1xuICAgICAgZG9jdW1lbnQudGl0bGUgPSBicmFuZC53aW5kb3dUaXRsZTtcbiAgICB9IGVsc2UgaWYgKGFjdGl2ZUFwcFRpdGxlKSB7XG4gICAgICBkb2N1bWVudC50aXRsZSA9IGFjdGl2ZUFwcFRpdGxlO1xuICAgIH1cbiAgfSwgW2FjdGl2ZUFwcFRpdGxlLCBicmFuZCwgc2NvcGUub3duc1BhZ2VdKTtcblxuICAvLyDwn5SM77iPIEJvb3QgZ2F0ZXMgb24gdGhlIHByaW1hcnkvaG9zdCBwbHVnaW4gT05MWSDigJQgZXZlcnkgb3RoZXIgcmVnaXN0cnkgZW50cnkgc3RyZWFtcyBpbiB2aWEgdGhlXG4gIC8vIHN1YnNjcmlwdGlvbiBlZmZlY3QgYmVsb3cgYXMgaXRzIGJ1aWxkIGxhbmRzLCBpbnN0ZWFkIG9mIHRoZSB3aG9sZSBzaGVsbCB3YWl0aW5nIG9uIGFsbCB+MzcgY3JhdGVzXG4gIC8vIChzZWUgYGJ1aWxkUGx1Z2luc1N0cmVhbWluZ2AgaW4gdGhlIGRldiBydW5uZXIpLiBBIHByaW1hcnkgdGhhdCBmYWlscyB0byBsb2FkICh0aW1lb3V0L2Vycm9yKSBpc1xuICAvLyBzdGlsbCBmYXRhbCwgbWlycm9yaW5nIHRoZSBvbGQgYG5vUGx1Z2luc0xvYWRlZGAvXCJob3N0IHByb2dyYW0gbWlzc2luZyBsYW5kaW5nIGFwcFwiIGJvb3QgZmFpbHVyZXMuXG4gIHVzZUVmZmVjdCgoKSA9PiB7XG4gICAgaWYgKCFwcmltYXJ5UGx1Z2luSWQpIHJldHVybjtcbiAgICBpZiAobG9hZGVkUGx1Z2luc1JlZi5jdXJyZW50LnNvbWUoKGVudHJ5KSA9PiBlbnRyeS5oYW5kbGUucGx1Z2luSWQgPT09IHByaW1hcnlQbHVnaW5JZCkpIHJldHVybjtcbiAgICB2b2lkIChhc3luYyAoKSA9PiB7XG4gICAgICBjb25zdCBvdXRjb21lID0gYXdhaXQgaW5zdGFsbFBsdWdpbihwcmltYXJ5UGx1Z2luSWQpO1xuICAgICAgaWYgKG91dGNvbWUgPT09IFwiZmFpbGVkXCIpIHtcbiAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9FUlJPUlwiLCB2YWx1ZTogc2hlbGxMYWJlbChcInVpLmNvbW1vbi5ub1BsdWdpbnNMb2FkZWRcIikgfSk7XG4gICAgICB9XG4gICAgfSkoKTtcbiAgfSwgW3ByaW1hcnlQbHVnaW5JZCwgaW5zdGFsbFBsdWdpbl0pO1xuXG4gIC8vIPCflIzvuI8gU3RyZWFtcyBldmVyeSByZWdpc3RyeSBlbnRyeSBpbiBpbmRlcGVuZGVudGx5IG9mIGJvb3Q6IG9uZSBjb25uZWN0LXRpbWUgYHNuYXBzaG90YCAod2hhdGV2ZXInc1xuICAvLyBhbHJlYWR5IGJ1aWx0LCBpbmNsdWRpbmcgYSBkZXYgc2VydmVyIHRoYXQgd2FzIGFscmVhZHkgZnVsbHkgYnVpbHQgYmVmb3JlIHRoaXMgc2hlbGwgbW91bnRlZCkgcGx1c1xuICAvLyBhIGBidWlsdGAgZXZlbnQgcGVyIGNyYXRlIGFzIGBidWlsZFBsdWdpbnNTdHJlYW1pbmdgL3RoZSBmb2xkZWQtaW4gd2F0Y2ggbG9vcCBmaW5pc2hlcyBpdC4gQW4gZXZlbnRcbiAgLy8gZm9yIGFuIGFscmVhZHktbG9hZGVkIHBsdWdpbiByb3V0ZXMgdG8gYHJlbG9hZFBsdWdpbmAgKGhvdC1zd2FwKSBpbnN0ZWFkIG9mIGBpbnN0YWxsUGx1Z2luYC5cbiAgdXNlRWZmZWN0KCgpID0+IHtcbiAgICBjb25zdCByZWdpc3RyeUlkcyA9IG5ldyBTZXQocmVnaXN0cnkubWFwKChlbnRyeSkgPT4gZW50cnkucGx1Z2luSWQpKTtcbiAgICBjb25zdCBoYW5kbGVQbHVnaW5BdmFpbGFibGUgPSAocGx1Z2luSWQ6IHN0cmluZywgcmVidWlsdEF0OiBudW1iZXIpID0+IHtcbiAgICAgIGlmICghcmVnaXN0cnlJZHMuaGFzKHBsdWdpbklkKSkgcmV0dXJuO1xuICAgICAgY29uc3QgYWxyZWFkeUxvYWRlZCA9IGxvYWRlZFBsdWdpbnNSZWYuY3VycmVudC5zb21lKChlbnRyeSkgPT4gZW50cnkuaGFuZGxlLnBsdWdpbklkID09PSBwbHVnaW5JZCk7XG4gICAgICB2b2lkIChhbHJlYWR5TG9hZGVkID8gcmVsb2FkUGx1Z2luKHBsdWdpbklkLCByZWJ1aWx0QXQpIDogaW5zdGFsbFBsdWdpbihwbHVnaW5JZCwgcmVidWlsdEF0KSk7XG4gICAgfTtcbiAgICByZXR1cm4gcGx1Z2luU291cmNlLnN1YnNjcmliZSgoZXZlbnQ6IFBsdWdpblNvdXJjZUV2ZW50KSA9PiB7XG4gICAgICBpZiAoZXZlbnQua2luZCA9PT0gXCJzbmFwc2hvdFwiKSB7XG4gICAgICAgIGZvciAoY29uc3QgcGx1Z2luIG9mIGV2ZW50LnBsdWdpbnMpIGhhbmRsZVBsdWdpbkF2YWlsYWJsZShwbHVnaW4ucGx1Z2luSWQsIHBsdWdpbi5yZWJ1aWx0QXQpO1xuICAgICAgICByZXR1cm47XG4gICAgICB9XG4gICAgICBoYW5kbGVQbHVnaW5BdmFpbGFibGUoZXZlbnQucGx1Z2luSWQsIGV2ZW50LnJlYnVpbHRBdCk7XG4gICAgfSk7XG4gIH0sIFtyZWdpc3RyeSwgcGx1Z2luU291cmNlLCBpbnN0YWxsUGx1Z2luLCByZWxvYWRQbHVnaW5dKTtcblxuICBjb25zdCBmaW5kUGx1Z2luRm9yQWN0aW9uID0gdXNlQ2FsbGJhY2soXG4gICAgKGFjdGlvbjogQWN0aW9uRGVzY3JpcHRvcikgPT4ge1xuICAgICAgY29uc3QgYnlDb250cm9sbGVyID0gbG9hZGVkUGx1Z2lucy5maW5kKChlbnRyeSkgPT4gZW50cnkubWFuaWZlc3QuYXBwcy5zb21lKChhcHApID0+IGFwcC5jb250cm9sbGVySWQgPT09IGFjdGlvbi5jb250cm9sbGVySWQpKTtcbiAgICAgIGlmIChieUNvbnRyb2xsZXIpIHJldHVybiBieUNvbnRyb2xsZXI7XG4gICAgICByZXR1cm4gbG9hZGVkUGx1Z2lucy5maW5kKChlbnRyeSkgPT4gZW50cnkuaGFuZGxlLnBsdWdpbklkID09PSBzZXNzaW9uPy5wbHVnaW5JZCk7XG4gICAgfSxcbiAgICBbbG9hZGVkUGx1Z2lucywgc2Vzc2lvbj8ucGx1Z2luSWRdLFxuICApO1xuXG4gIGNvbnN0IHJlcXVlc3RDb250ZXh0TWVudSA9IHVzZUNhbGxiYWNrKFxuICAgIGFzeW5jIChyZXF1ZXN0OiBQbHVnaW5Db250ZXh0TWVudVJlcXVlc3QpOiBQcm9taXNlPHJlYWRvbmx5IENvbnRleHRNZW51SXRlbVNwZWNbXT4gPT4ge1xuICAgICAgaWYgKCFzZXNzaW9uKSByZXR1cm4gW107XG4gICAgICBjb25zdCBwbHVnaW4gPSBsb2FkZWRQbHVnaW5zLmZpbmQoKGVudHJ5KSA9PiBlbnRyeS5oYW5kbGUucGx1Z2luSWQgPT09IHNlc3Npb24ucGx1Z2luSWQpPy5oYW5kbGU7XG4gICAgICBpZiAoIXBsdWdpbj8uY29udGV4dE1lbnUpIHJldHVybiBbXTtcbiAgICAgIC8vIPCflrHvuI8gTm8gdmlldyBzdGF0ZSBvbiB0aGUgd2lyZSDigJQgdGhlIFNESydzIENvbnRleHRNZW51V2lyZVJlcXVlc3QgZHJvcHBlZCBpdCAodGhlIHBsdWdpbidzXG4gICAgICAvLyBvd24gcGVyc2lzdGVkIHNlbGVjdGlvbi9ob3ZlciBzdGF0ZSBhbHJlYWR5IGFuc3dlcnMgXCJ3aGF0J3Mgc2VsZWN0ZWRcIiwgc2VlIEFwcEFjdGlvblJlZ2lzdHJ5XG4gICAgICAvLyBmdW5uZWwpOyBzZW5kaW5nIG9uZSBoZXJlIHdvdWxkIGp1c3QgYmUgc2lsZW50bHkgZGlzY2FyZGVkIG9uIHRoZSBSdXN0IHNpZGUuXG4gICAgICByZXR1cm4gcGx1Z2luLmNvbnRleHRNZW51KHNlc3Npb24uaW5zdGFuY2VJZCwgcmVxdWVzdCk7XG4gICAgfSxcbiAgICBbbG9hZGVkUGx1Z2lucywgc2Vzc2lvbl0sXG4gICk7XG5cbiAgY29uc3QgcmVmcmVzaFVpID0gdXNlQ2FsbGJhY2soXG4gICAgLy8g8J+qn++4jyBgZXh0cmFJbnN0YW5jZXNPdmVycmlkZWAgbGV0cyBhIGNhbGxlciB0aGF0IGp1c3Qgc3luY2hyb25vdXNseSBjb21wdXRlZCBhIE5FVyBleHRyYS13aW5kb3cgbGlzdFxuICAgIC8vIChzcGxpdC9kcm9wLCBsYXlvdXQvbW9kZSBzd2l0Y2gpIGhhbmQgaXQgc3RyYWlnaHQgdG8gdGhpcyBmZXRjaCBpbnN0ZWFkIG9mIHJlYWRpbmcgYGV4dHJhV2luZG93SW5zdGFuY2VzYFxuICAgIC8vIGZyb20gUmVhY3Qgc3RhdGUsIHdoaWNoIHdvdWxkbid0IHJlZmxlY3QgdGhlIGp1c3QtZGlzcGF0Y2hlZCBjaGFuZ2UgdW50aWwgdGhlIG5leHQgcmVuZGVyLlxuICAgIGFzeW5jIChuZXh0U2Vzc2lvbjogQWN0aXZlU2Vzc2lvbiwgc2NvcGVBcmc6IFVpRGlydHlTY29wZSA9IHsga2luZDogXCJmdWxsXCIgfSwgZXh0cmFJbnN0YW5jZXNPdmVycmlkZT86IHJlYWRvbmx5IEV4dHJhV2luZG93SW5zdGFuY2VbXSkgPT4ge1xuICAgICAgaWYgKHNjb3BlQXJnLmtpbmQgPT09IFwibm9uZVwiKSByZXR1cm47XG4gICAgICBjb25zdCBnZW5lcmF0aW9uID0gKytyZWZyZXNoR2VuZXJhdGlvblJlZi5jdXJyZW50O1xuICAgICAgY29uc3QgcHJvZ3JhbSA9IGxvYWRlZFBsdWdpbnMuZmluZCgoZW50cnkpID0+IGVudHJ5LmhhbmRsZS5wbHVnaW5JZCA9PT0gbmV4dFNlc3Npb24ucGx1Z2luSWQpPy5oYW5kbGU7XG4gICAgICBpZiAoIXByb2dyYW0pIHJldHVybjtcbiAgICAgIGNvbnN0IGxheW91dFNlZWRLZXkgPSBgJHtuZXh0U2Vzc2lvbi5wbHVnaW5JZH06JHtuZXh0U2Vzc2lvbi5hcHAuaWR9OiR7bmV4dFNlc3Npb24uaW5zdGFuY2VJZH1gO1xuICAgICAgY29uc3QgaXNTZXNzaW9uU3dpdGNoID0gbGF5b3V0U2VlZEtleVJlZi5jdXJyZW50ICE9PSBsYXlvdXRTZWVkS2V5O1xuICAgICAgLy8g8J+Qou+4jyBBIHNlc3Npb24gc3dpdGNoIGludmFsaWRhdGVzIGV2ZXJ5IGNhY2hlZCBoYXNoIGZyb20gdGhlIHByZXZpb3VzIGluc3RhbmNlIOKAlCBmb3JjZSBhIGZ1bGxcbiAgICAgIC8vIGZldGNoIHJlZ2FyZGxlc3Mgb2Ygd2hhdCBzY29wZSB0aGlzIHBhcnRpY3VsYXIgY2FsbCB3YXMgZ2l2ZW4uXG4gICAgICBsZXQgc2NvcGUgPSBzY29wZUFyZztcbiAgICAgIGlmIChpc1Nlc3Npb25Td2l0Y2gpIHtcbiAgICAgICAgdWlSZWZyZXNoQ2FjaGVSZWYuY3VycmVudCA9IG5ldyBNYXAoKTtcbiAgICAgICAgc2NvcGUgPSB7IGtpbmQ6IFwiZnVsbFwiIH07XG4gICAgICB9XG4gICAgICBjb25zdCBjYWNoZSA9IHVpUmVmcmVzaENhY2hlUmVmLmN1cnJlbnQ7XG4gICAgICAvLyDwn6qf77iPIE9uIGEgc2Vzc2lvbiBzd2l0Y2gsIHNlZWQgdGhlIGRlZmF1bHQgbGF5b3V0J3MgZXh0cmEgaW5zdGFuY2VzIEJFRk9SRSBmZXRjaGluZyAobm90IGFmdGVyKSwgc29cbiAgICAgIC8vIHRoaXMgdmVyeSBmaXJzdCBmZXRjaCBhbHJlYWR5IHJlcXVlc3RzIGV2ZXJ5IGRlZmF1bHQtbGF5b3V0IHBhbmUncyBib2R5L21lYXN1cmVzL2VuZ2FnZW1lbnRzXG4gICAgICAvLyBpbnN0ZWFkIG9mIGxlYXZpbmcgbmV3bHktc2VlZGVkIHBhbmVzIHRvIHNob3cgXCJtaXNzaW5nIHdpbmRvd1wiIHVudGlsIHNvbWUgbGF0ZXIsIHVucmVsYXRlZCByZWZyZXNoLlxuICAgICAgY29uc3QgbGF5b3V0U2VlZCA9IGlzU2Vzc2lvblN3aXRjaCA/IGFwcGx5RnJhbWV3b3JrTGF5b3V0U2VlZChuZXh0U2Vzc2lvbi5hcHAuZGVmYXVsdExheW91dCwgbmV4dFNlc3Npb24uYXBwLndpbmRvd0tpbmRzLCBhcHBMYWJlbHNPdmVybGF5LCB1aVRlcm1pbm9sb2d5LCB1aUxvY2FsZSkgOiB1bmRlZmluZWQ7XG4gICAgICAvLyDwn6qf77iPIFByZWZlciB0aGUgb3ZlcnJpZGUsIHRoZW4gdGhlIGp1c3QtY29tcHV0ZWQgc2Vzc2lvbi1zd2l0Y2ggc2VlZCwgdGhlbiB0aGUgbGl2ZSByZWYgKG5ldmVyIHRoZVxuICAgICAgLy8gcmVuZGVyLWNsb3N1cmUgc25hcHNob3QpIHNvIGEgY29uY3VycmVudCByZWZyZXNoIGNhbm5vdCBkcm9wIGRlZmF1bHQtbGF5b3V0IHBhbmVzLlxuICAgICAgY29uc3QgZXh0cmFJbnN0YW5jZXNGb3JGZXRjaCA9IGV4dHJhSW5zdGFuY2VzT3ZlcnJpZGUgPz8gbGF5b3V0U2VlZD8uZXh0cmFJbnN0YW5jZXMgPz8gZXh0cmFXaW5kb3dJbnN0YW5jZXNSZWYuY3VycmVudDtcbiAgICAgIGNvbnN0IHdpbmRvd0luc3RhbmNlcyA9IHNlc3Npb25XaW5kb3dJbnN0YW5jZXMobmV4dFNlc3Npb24uYXBwLCBleHRyYUluc3RhbmNlc0ZvckZldGNoKTtcbiAgICAgIGNvbnN0IGNvbnRyaWJ1dGlvbnNKc29uID0gYnVpbGRDb250cmlidXRpb25zSnNvbihsb2FkZWRQbHVnaW5zLm1hcCgoZW50cnkpID0+ICh7IHBsdWdpbklkOiBlbnRyeS5oYW5kbGUucGx1Z2luSWQsIG1hbmlmZXN0OiBlbnRyeS5tYW5pZmVzdCB9KSkpO1xuICAgICAgLy8g8J+qkO+4jyBFdmVyeSBsb2FkZWQgcGx1Z2luJ3MgZGVjbGFyZWQgYXBwcywgZmxhdHRlbmVkIGZvciB0aGUgc3BhY2UgYXBwJ3MgY2F0YWxvZ3VlIOKAlCBtaXJyb3JzXG4gICAgICAvLyBgY29udHJpYnV0aW9uc0pzb25gIGFib3ZlIGV4YWN0bHkgKHNhbWUgb3B0LWluIGhpbnQtcHVzaCBzaGFwZSBiZWxvdyksIGJlY2F1c2UgdGhlIHNwYWNlIGFwcCBpc1xuICAgICAgLy8gaXRzIG93biB3YXNtIGNvbXBvbmVudDogYHNlbWlvX2ZyYW1ld29ya19vczo6QVBQX1JFR0lTVFJBVElPTlNgIChwb3B1bGF0ZWQgYXQgbmF0aXZlL3Rlc3RcbiAgICAgIC8vIGBQbHVnaW5Ib3N0Ojpsb2FkX3BsdWdpbmAvYGhvdF9zd2FwX3BsdWdpbmAgdGltZSkgbGl2ZXMgaW4gYSBzZXBhcmF0ZSBsaW5lYXIgbWVtb3J5IGZyb20gdGhlXG4gICAgICAvLyBzcGFjZSBhcHAncyBvd24gc3RhdGljYWxseS1saW5rZWQgY29weSBvZiB0aGUgc2FtZSBvcy1jb3JlIGNyYXRlLCBzbyBub3RoaW5nIGNyb3NzZXMgdGhlIHdhc21cbiAgICAgIC8vIGJvdW5kYXJ5IHVubGVzcyB0aGlzIHNoZWxsIHB1c2hlcyBpdCBleHBsaWNpdGx5LlxuICAgICAgY29uc3QgYXBwUmVnaXN0cmF0aW9uc0pzb24gPSBKU09OLnN0cmluZ2lmeShsb2FkZWRQbHVnaW5zLmZsYXRNYXAoKGVudHJ5KSA9PiAoZW50cnkubWFuaWZlc3QuYXBwcyA/PyBbXSkubWFwKChhcHApID0+ICh7IHBsdWdpbklkOiBlbnRyeS5oYW5kbGUucGx1Z2luSWQsIGFwcCB9KSkpKTtcbiAgICAgIGNvbnN0IHZpZXdTdGF0ZTogVmlld01vZGVsID0gaW5qZWN0QWN0aXZlVG9vbCh7XG4gICAgICAgIC4uLm5leHRTZXNzaW9uLnZpZXdTdGF0ZSxcbiAgICAgICAgY29udHJpYnV0aW9uc0pzb24sXG4gICAgICAgIGxvY2FsZTogdWlMb2NhbGUsXG4gICAgICAgIHRlcm1pbm9sb2d5OiB1aVRlcm1pbm9sb2d5LFxuICAgICAgICB3aW5kb3dJbnN0YW5jZXM6IHdpbmRvd0luc3RhbmNlcy5tYXAoKGluc3RhbmNlKSA9PiAoeyBpZDogaW5zdGFuY2UuaWQsIHdpbmRvd0tpbmRJZDogaW5zdGFuY2Uud2luZG93S2luZElkIH0pKSxcbiAgICAgICAgYWN0aXZlVXRpbGl0eUJ5V2luZG93SWQ6IGJ1aWxkQWN0aXZlVXRpbGl0eUJ5V2luZG93SWQoYWN0aXZlVXRpbGl0eUJ5V2luZG93SWRSZWYuY3VycmVudCksXG4gICAgICAgIGFjdGl2ZVV0aWxpdHlJZDogdW5kZWZpbmVkLFxuICAgICAgfSk7XG4gICAgICBjb25zdCBwYW5lbFRhYkxlYXZlcyA9IGZsYXR0ZW5QYW5lbFRhYkxlYXZlcyhuZXh0U2Vzc2lvbi5hcHAucGFuZWxUYWJzKTtcbiAgICAgIC8vIPCfkKLvuI8gT25lIGJhdGNoZWQsIGhhc2gtY29uZGl0aW9uYWwgcm91bmQgdHJpcCByZXBsYWNlcyB0aGUgb2xkIH4xMiBzZXF1ZW50aWFsXG4gICAgICAvLyByZW5kZXIvdXRpbGl0aWVzL3dpbmRvd0VuZ2FnZW1lbnRzL3dpbmRvd01lYXN1cmVzL2FwcExhYmVscyBjYWxscyDigJQgdGhlIHBsdWdpbiBvbWl0cyBwYXlsb2FkcyBmb3JcbiAgICAgIC8vIGFueSBzZWN0aW9uIHdob3NlIGhhc2ggc3RpbGwgbWF0Y2hlcyB3aGF0IGBjYWNoZWAgYWxyZWFkeSBob2xkcy5cbiAgICAgIGNvbnN0IHJlcXVlc3QgPSBidWlsZFVpUmVmcmVzaFJlcXVlc3Qoc2NvcGUsIHdpbmRvd0luc3RhbmNlcywgcGFuZWxUYWJMZWF2ZXMsIHZpZXdTdGF0ZSwgY2FjaGUpO1xuICAgICAgaWYgKHJlcXVlc3QpIHtcbiAgICAgICAgY29uc3QgcmVzcG9uc2UgPSBhd2FpdCBwcm9ncmFtLnJlZnJlc2hVaShuZXh0U2Vzc2lvbi5pbnN0YW5jZUlkLCByZXF1ZXN0KTtcbiAgICAgICAgaWYgKGdlbmVyYXRpb24gIT09IHJlZnJlc2hHZW5lcmF0aW9uUmVmLmN1cnJlbnQpIHJldHVybjtcbiAgICAgICAgY29uc3Qgc2xvdENvbnRleHQgPSB7XG4gICAgICAgICAgcGx1Z2luczogbmV3IE1hcChsb2FkZWRQbHVnaW5zLm1hcCgoZW50cnkpID0+IFtlbnRyeS5oYW5kbGUucGx1Z2luSWQsIGVudHJ5LmhhbmRsZV0pKSxcbiAgICAgICAgICBjb250cmlidXRvckluc3RhbmNlczogY29udHJpYnV0b3JJbnN0YW5jZXNSZWYuY3VycmVudCxcbiAgICAgICAgICB2aWV3U3RhdGUsXG4gICAgICAgIH07XG4gICAgICAgIC8vIFJlc29sdmUgZXh0ZXJuYWwgc2xvdHMgb24gZnJlc2hseS1jaGFuZ2VkIHdpbmRvdy9wYW5lbCBib2RpZXMgb25seSwgYmVmb3JlIGNhY2hpbmcgdGhlbSwgc28gYVxuICAgICAgICAvLyBsYXRlciBuby1vcGVyYXRpb24gcmVmcmVzaCByZXVzZXMgdGhlIGFscmVhZHktcmVzb2x2ZWQgY2FjaGVkIHZhbHVlIGluc3RlYWQgb2YgcmUtcmVzb2x2aW5nLlxuICAgICAgICBjb25zdCByZXNvbHZlSWZDaGFuZ2VkID0gYXN5bmMgKGVudHJ5OiBQbHVnaW5VaVJlZnJlc2hTZWN0aW9uUmVzcG9uc2UpOiBQcm9taXNlPFBsdWdpblVpUmVmcmVzaFNlY3Rpb25SZXNwb25zZT4gPT4gKGVudHJ5LnZhbHVlICE9PSB1bmRlZmluZWQgPyB7IC4uLmVudHJ5LCB2YWx1ZTogYXdhaXQgcmVzb2x2ZUV4dGVybmFsU2xvdHMoZW50cnkudmFsdWUgYXMgVWlOb2RlLCBzbG90Q29udGV4dCkgfSA6IGVudHJ5KTtcbiAgICAgICAgY29uc3QgW3Jlc29sdmVkV2luZG93cywgcmVzb2x2ZWRQYW5lbHNdID0gYXdhaXQgUHJvbWlzZS5hbGwoW1Byb21pc2UuYWxsKChyZXNwb25zZS53aW5kb3dzID8/IFtdKS5tYXAocmVzb2x2ZUlmQ2hhbmdlZCkpLCBQcm9taXNlLmFsbCgocmVzcG9uc2UucGFuZWxzID8/IFtdKS5tYXAocmVzb2x2ZUlmQ2hhbmdlZCkpXSk7XG4gICAgICAgIGlmIChnZW5lcmF0aW9uICE9PSByZWZyZXNoR2VuZXJhdGlvblJlZi5jdXJyZW50KSByZXR1cm47XG4gICAgICAgIGFwcGx5VWlSZWZyZXNoUmVzcG9uc2VUb0NhY2hlKGNhY2hlLCB7IC4uLnJlc3BvbnNlLCB3aW5kb3dzOiByZXNvbHZlZFdpbmRvd3MsIHBhbmVsczogcmVzb2x2ZWRQYW5lbHMgfSk7XG4gICAgICAgIC8vIOKPse+4jyBTZWUgYERvY3VtZW50QXBwOjpwZW5kaW5nX2VmZmVjdHNgIOKAlCBlLmcuIHJlc3VtaW5nIGEgYGZsb3dFdmFsVGlja2AgY2hhaW4gYWZ0ZXIgdGhpcyByZWZyZXNoLlxuICAgICAgICBpZiAocmVzcG9uc2UucmVxdWVzdGVkRWZmZWN0cz8ubGVuZ3RoKSBhd2FpdCBhcHBseUhvc3RFZmZlY3RzKHJlc3BvbnNlLnJlcXVlc3RlZEVmZmVjdHMsIG5leHRTZXNzaW9uKTtcbiAgICAgIH1cbiAgICAgIC8vIPCfjq8gQm90aCBwdXNoIGd1YXJkcyBiZWxvdyBhcmUga2V5ZWQgb24gYCR7bmV4dFNlc3Npb24uaW5zdGFuY2VJZH06OiR7anNvbn1gLCBOT1Qgb24gdGhlIGpzb25cbiAgICAgIC8vIGNvbnRlbnQgYWxvbmUg4oCUIHRoZSBjb250ZW50IGlzIGRlcml2ZWQgcHVyZWx5IGZyb20gYGxvYWRlZFBsdWdpbnNgLCB3aGljaCBzdGFiaWxpemVzIHJpZ2h0IGFmdGVyXG4gICAgICAvLyBib290LCBzbyBhIGNvbnRlbnQtb25seSBrZXkgd291bGQgb25seSBldmVyIHVubG9jayBPTkUgcHVzaCBmb3IgdGhlIHByb2Nlc3MgbGlmZXRpbWUgKHRoZSB2ZXJ5XG4gICAgICAvLyBmaXJzdCBgcmVmcmVzaFVpYCBjYWxsLCB3aGljaCBhbHdheXMgdGFyZ2V0cyB3aGF0ZXZlciBzZXNzaW9uIGV4aXN0cyBhdCBib290IOKAlCB1c3VhbGx5IGBob21lYCxcbiAgICAgIC8vIHdoaWNoIGRvZXNuJ3QgaW1wbGVtZW50IGVpdGhlciBhY3Rpb24gYW5kIHJlamVjdHMgaXQpLiBGb2xkaW5nIGBpbnN0YW5jZUlkYCBpbnRvIHRoZSBrZXkgbWFrZXMgYVxuICAgICAgLy8gc2Vzc2lvbiBzd2l0Y2ggKG5ldyBzdHVkaW8vc3BhY2UgaW5zdGFuY2Ugb3BlbmVkLCBzYW1lIHVuY2hhbmdlZCBqc29uKSByZXRyaWdnZXIgdGhlIHB1c2ggaW5zdGVhZFxuICAgICAgLy8gb2YgYmVpbmcgc2lsZW50bHkgc3dhbGxvd2VkIGJ5IGEgZ3VhcmQgdGhhdCBhbHJlYWR5IGNvbnNpZGVyZWQgdGhpcyBjb250ZW50IFwiZGVsaXZlcmVkXCIuXG4gICAgICBpZiAoY29udHJpYnV0aW9uc0pzb24pIHtcbiAgICAgICAgY29uc3QgY29udHJpYnV0aW9uc1B1c2hLZXkgPSBgJHtuZXh0U2Vzc2lvbi5pbnN0YW5jZUlkfTo6JHtjb250cmlidXRpb25zSnNvbn1gO1xuICAgICAgICBpZiAoY29udHJpYnV0aW9uc1B1c2hLZXkgIT09IGNvbnRyaWJ1dGlvbnNKc29uUmVmLmN1cnJlbnQpIHtcbiAgICAgICAgICBjb250cmlidXRpb25zSnNvblJlZi5jdXJyZW50ID0gY29udHJpYnV0aW9uc1B1c2hLZXk7XG4gICAgICAgICAgY29uc3QgcGx1Z2luRW50cnkgPSBsb2FkZWRQbHVnaW5zLmZpbmQoKGVudHJ5KSA9PiBlbnRyeS5oYW5kbGUucGx1Z2luSWQgPT09IG5leHRTZXNzaW9uLnBsdWdpbklkKTtcbiAgICAgICAgICAvLyDwn5uh77iPIGBzZXRDb250cmlidXRpb25zYCBpcyBhbiBvcHQtaW4gaGludCBwdXNoIOKAlCBvbmx5IGBwcm9jZWR1cmFsM2RgJ3MgYFByb2NlZHVyYWwzZENvbW1hbmQ6OlNldENvbnRyaWJ1dGlvbnNgXG4gICAgICAgICAgLy8gKGZsb3cuZXh0ZW5zaW9uIGhvdC1zd2FwKSBhbmQgYGZvcm1zYCdzIGBGb3Jtc0NvbW1hbmQ6OlNldENvbnRyaWJ1dGlvbnNgIChwbGF5Ym9vayBibG9jay1raW5kIGNhdGFsb2d1ZSlcbiAgICAgICAgICAvLyBhY3R1YWxseSBpbXBsZW1lbnQgaXQ7IGl0IGlzIGRlbGliZXJhdGVseSBOT1QgZGVjbGFyZWQgaW4gZWl0aGVyIGFwcCdzIGFjdGlvbiBjYXRhbG9nIChzYW1lXG4gICAgICAgICAgLy8gdW5jYXRhbG9ndWVkLWJyaWRnZSBzaGFwZSBhcyBgc2V0TG9jYWxlYCksIHNvIGNhdGFsb2cgbWVtYmVyc2hpcCBjYW4ndCBnYXRlIHRoaXMgY2FsbC4gRXZlcnkgb3RoZXJcbiAgICAgICAgICAvLyBhcHAncyBgRG9jdW1lbnRBcHA6OmNvbW1hbmRfZnJvbV9hY3Rpb25gIGRlZmF1bHQgcmVqZWN0cyB1bmtub3duIGlkcyDigJQgc3dhbGxvdyB0aGF0IHJlamVjdGlvbiBoZXJlXG4gICAgICAgICAgLy8gcmF0aGVyIHRoYW4gZ2F0aW5nIGJ5IGFwcCBpZCwgc28gdGhpcyBzdGF5cyBjb3JyZWN0IGlmIGEgZnV0dXJlIGFwcCBhZGRzIGl0cyBvd24gYFNldENvbnRyaWJ1dGlvbnNgXG4gICAgICAgICAgLy8gdmFyaWFudCB3aXRob3V0IHRoaXMgY2FsbCBzaXRlIG5lZWRpbmcgdG8ga25vdyBhYm91dCBpdC5cbiAgICAgICAgICAvLyDwn6e177iPIEIxOiBNVVNUIGdvIHRocm91Z2ggYGhhbmRsZUFjdGlvbmAgKGtpbmQ6XCJhY3Rpb25cIiDihpIgYGRpc3BhdGNoX2FjdGlvbmAg4oaSIGBjb21tYW5kX2Zyb21fYWN0aW9uYFxuICAgICAgICAgIC8vIOKGkiBgZGlzcGF0Y2hfdHlwZWRfY29tbWFuZF9pbm5lcmApIOKAlCBgaGFuZGxlQ29tbWFuZGAgKGtpbmQ6XCJjb21tYW5kXCIpIGFsd2F5cyBoYXJkLWVycm9ycyBub3csIHNlZVxuICAgICAgICAgIC8vIGBWY3NEb2N1bWVudEFwcDo6ZGlzcGF0Y2hfY29tbWFuZGAncyBkb2M7IHRoZXJlIGFyZSBubyBmcmFtZXdvcmstcmVzZXJ2ZWQgQ09NTUFORFMsIG9ubHkgYWN0aW9ucy5cbiAgICAgICAgICBpZiAocGx1Z2luRW50cnkpIHtcbiAgICAgICAgICAgIHRyeSB7XG4gICAgICAgICAgICAgIGNvbnN0IHdpcmUgPSBlbmNvZGVBY3Rpb25XaXJlKHsgY29udHJvbGxlcklkOiBuZXh0U2Vzc2lvbi5hcHAuY29udHJvbGxlcklkLCBhY3Rpb246IFwic2V0Q29udHJpYnV0aW9uc1wiLCBhcmdzOiB7IGpzb246IGNvbnRyaWJ1dGlvbnNKc29uIH0gfSk7XG4gICAgICAgICAgICAgIGF3YWl0IHBsdWdpbkVudHJ5LmhhbmRsZS5oYW5kbGVBY3Rpb24obmV4dFNlc3Npb24uaW5zdGFuY2VJZCwgd2lyZSwgbmV4dFNlc3Npb24udmlld1N0YXRlKTtcbiAgICAgICAgICAgIH0gY2F0Y2ggKGVycm9yKSB7XG4gICAgICAgICAgICAgIGNvbnNvbGUud2FybihcIltERUJVR10gc2V0Q29udHJpYnV0aW9ucyBwdXNoIHNraXBwZWRcIiwgZXJyb3IgaW5zdGFuY2VvZiBFcnJvciA/IGVycm9yLm1lc3NhZ2UgOiBTdHJpbmcoZXJyb3IpKTtcbiAgICAgICAgICAgIH1cbiAgICAgICAgICB9XG4gICAgICAgIH1cbiAgICAgIH1cbiAgICAgIGlmIChhcHBSZWdpc3RyYXRpb25zSnNvbikge1xuICAgICAgICBjb25zdCBhcHBSZWdpc3RyYXRpb25zUHVzaEtleSA9IGAke25leHRTZXNzaW9uLmluc3RhbmNlSWR9Ojoke2FwcFJlZ2lzdHJhdGlvbnNKc29ufWA7XG4gICAgICAgIGlmIChhcHBSZWdpc3RyYXRpb25zUHVzaEtleSAhPT0gYXBwUmVnaXN0cmF0aW9uc0pzb25SZWYuY3VycmVudCkge1xuICAgICAgICAgIGFwcFJlZ2lzdHJhdGlvbnNKc29uUmVmLmN1cnJlbnQgPSBhcHBSZWdpc3RyYXRpb25zUHVzaEtleTtcbiAgICAgICAgICBjb25zdCBwbHVnaW5FbnRyeSA9IGxvYWRlZFBsdWdpbnMuZmluZCgoZW50cnkpID0+IGVudHJ5LmhhbmRsZS5wbHVnaW5JZCA9PT0gbmV4dFNlc3Npb24ucGx1Z2luSWQpO1xuICAgICAgICAgIC8vIPCfqpDvuI8gYHNldEFwcFJlZ2lzdHJhdGlvbnNgIG1pcnJvcnMgYHNldENvbnRyaWJ1dGlvbnNgIGltbWVkaWF0ZWx5IGFib3ZlIGV4YWN0bHk6IGFuIG9wdC1pbiBoaW50XG4gICAgICAgICAgLy8gcHVzaCwgY3VycmVudGx5IG9ubHkgaW1wbGVtZW50ZWQgYnkgdGhlIHNwYWNlIGFwcCdzIGBTcGFjZUNvbW1hbmQ6OlNldEFwcFJlZ2lzdHJhdGlvbnNgXG4gICAgICAgICAgLy8gKHBvcHVsYXRlcyBpdHMgb3duIGxpbmtlZC1pbiBjb3B5IG9mIGBzZW1pb19mcmFtZXdvcmtfb3M6OkFQUF9SRUdJU1RSQVRJT05TYCBzb1xuICAgICAgICAgIC8vIGB3b3JrZmxvd19wYWxldHRlKClgL2BidWlsZF9jYXRhbG9ndWVfdHJlZWAgY2FuIGxpc3QgZXZlcnkgbG9hZGVkIGFwcCkuIE5vdCBkZWNsYXJlZCBpbiBhbnlcbiAgICAgICAgICAvLyBhcHAncyBhY3Rpb24gY2F0YWxvZywgc28g4oCUIHNhbWUgYXMgYHNldENvbnRyaWJ1dGlvbnNgIOKAlCBnYXRlIGJ5IHN3YWxsb3dpbmcgdGhlIHJlamVjdGlvbiBldmVyeVxuICAgICAgICAgIC8vIG90aGVyIGFwcCdzIGBEb2N1bWVudEFwcDo6Y29tbWFuZF9mcm9tX2FjdGlvbmAgZGVmYXVsdCB0aHJvd3MgZm9yIGFuIHVua25vd24gaWQsIHJhdGhlciB0aGFuIGJ5XG4gICAgICAgICAgLy8gYXBwIGlkLCBzbyB0aGlzIHN0YXlzIGNvcnJlY3QgaWYgYSBmdXR1cmUgYXBwIGFkZHMgaXRzIG93biBgU2V0QXBwUmVnaXN0cmF0aW9uc2AgdmFyaWFudFxuICAgICAgICAgIC8vIHdpdGhvdXQgdGhpcyBjYWxsIHNpdGUgbmVlZGluZyB0byBrbm93IGFib3V0IGl0LlxuICAgICAgICAgIC8vIPCfp7XvuI8gQjE6IE1VU1QgZ28gdGhyb3VnaCBgaGFuZGxlQWN0aW9uYCAoa2luZDpcImFjdGlvblwiIOKGkiBgZGlzcGF0Y2hfYWN0aW9uYCDihpIgYGNvbW1hbmRfZnJvbV9hY3Rpb25gXG4gICAgICAgICAgLy8g4oaSIGBkaXNwYXRjaF90eXBlZF9jb21tYW5kX2lubmVyYCkg4oCUIGBoYW5kbGVDb21tYW5kYCAoa2luZDpcImNvbW1hbmRcIikgYWx3YXlzIGhhcmQtZXJyb3JzIG5vdywgc2VlXG4gICAgICAgICAgLy8gYFZjc0RvY3VtZW50QXBwOjpkaXNwYXRjaF9jb21tYW5kYCdzIGRvYzsgdGhlcmUgYXJlIG5vIGZyYW1ld29yay1yZXNlcnZlZCBDT01NQU5EUywgb25seSBhY3Rpb25zLlxuICAgICAgICAgIGlmIChwbHVnaW5FbnRyeSkge1xuICAgICAgICAgICAgdHJ5IHtcbiAgICAgICAgICAgICAgY29uc3Qgd2lyZSA9IGVuY29kZUFjdGlvbldpcmUoeyBjb250cm9sbGVySWQ6IG5leHRTZXNzaW9uLmFwcC5jb250cm9sbGVySWQsIGFjdGlvbjogXCJzZXRBcHBSZWdpc3RyYXRpb25zXCIsIGFyZ3M6IHsganNvbjogYXBwUmVnaXN0cmF0aW9uc0pzb24gfSB9KTtcbiAgICAgICAgICAgICAgYXdhaXQgcGx1Z2luRW50cnkuaGFuZGxlLmhhbmRsZUFjdGlvbihuZXh0U2Vzc2lvbi5pbnN0YW5jZUlkLCB3aXJlLCBuZXh0U2Vzc2lvbi52aWV3U3RhdGUpO1xuICAgICAgICAgICAgfSBjYXRjaCAoZXJyb3IpIHtcbiAgICAgICAgICAgICAgY29uc29sZS53YXJuKFwiW0RFQlVHXSBzZXRBcHBSZWdpc3RyYXRpb25zIHB1c2ggc2tpcHBlZFwiLCBlcnJvciBpbnN0YW5jZW9mIEVycm9yID8gZXJyb3IubWVzc2FnZSA6IFN0cmluZyhlcnJvcikpO1xuICAgICAgICAgICAgfVxuICAgICAgICAgIH1cbiAgICAgICAgfVxuICAgICAgfVxuICAgICAgLy8g8J+Qou+4jyBNZXJnZS13aXRoLWlkZW50aXR5LXByZXNlcnZhdGlvbjogdW5yZXF1ZXN0ZWQvdW5jaGFuZ2VkIHNlY3Rpb25zIGtlZXAgZXhhY3RseSB0aGUgb2JqZWN0XG4gICAgICAvLyByZWZlcmVuY2UgYWxyZWFkeSBpbiBgY2FjaGVgIChkaXNwYXRjaGVkIGZyb20gYSBwcmlvciByZWZyZXNoKSwgc28gYG1lcmdlUmVjb3JkUHJlc2VydmluZ0lkZW50aXR5YFxuICAgICAgLy8gYmFpbHMgb24gdGhlbSB2aWEgcmVmZXJlbmNlIGVxdWFsaXR5IOKAlCB0aGlzIGlzIHdoYXQgbGV0cyBgSW50ZXJwcmV0ZWRVaU5vZGVgJ3MgYFJlYWN0Lm1lbW9gIChhbmRcbiAgICAgIC8vIGBtb2RlV2luZG93c2AncyBgdXNlTWVtb2ApIHNraXAgcmVjb25jaWxpbmcgdGhlIHdob2xlIHNoZWxsIG9uIGV2ZXJ5IGludGVyYWN0aW9uLlxuICAgICAgZGlzcGF0Y2goe1xuICAgICAgICB0eXBlOiBcIlNFVF9XSU5ET1dfVUlfQllfV0lORE9XX0lEXCIsXG4gICAgICAgIHZhbHVlOiAoY3VycmVudCkgPT5cbiAgICAgICAgICBtZXJnZVJlY29yZFByZXNlcnZpbmdJZGVudGl0eShcbiAgICAgICAgICAgIGN1cnJlbnQsXG4gICAgICAgICAgICB3aW5kb3dJbnN0YW5jZXMubWFwKChpbnN0YW5jZSkgPT4gW2luc3RhbmNlLmlkLCAoY2FjaGUuZ2V0KGB3aW5kb3c6JHtpbnN0YW5jZS5pZH1gKT8udmFsdWUgYXMgVWlOb2RlIHwgdW5kZWZpbmVkKSA/PyBjdXJyZW50W2luc3RhbmNlLmlkXSA/PyBwZW5kaW5nV2luZG93VWlOb2RlKCldIGFzIGNvbnN0KSxcbiAgICAgICAgICApLFxuICAgICAgfSk7XG4gICAgICBjb25zdCBkeW5hbWljRW5nYWdlbWVudHMgPSAoY2FjaGUuZ2V0KFwiZW5nYWdlbWVudHNcIik/LnZhbHVlIGFzIFJlYWRvbmx5PFJlY29yZDxzdHJpbmcsIFdpbmRvd0VuZ2FnZW1lbnQ+PiB8IHVuZGVmaW5lZCkgPz8ge307XG4gICAgICBkaXNwYXRjaCh7XG4gICAgICAgIHR5cGU6IFwiU0VUX1dJTkRPV19FTkdBR0VNRU5UU19CWV9XSU5ET1dfSURcIixcbiAgICAgICAgdmFsdWU6IChjdXJyZW50KSA9PiBtZXJnZVJlY29yZFByZXNlcnZpbmdJZGVudGl0eShjdXJyZW50LCBPYmplY3QuZW50cmllcyhkeW5hbWljRW5nYWdlbWVudHMpKSxcbiAgICAgIH0pO1xuICAgICAgY29uc3QgZHluYW1pY01lYXN1cmVzID0gKGNhY2hlLmdldChcIm1lYXN1cmVzXCIpPy52YWx1ZSBhcyBSZWFkb25seTxSZWNvcmQ8c3RyaW5nLCByZWFkb25seSBXaW5kb3dNZWFzdXJlW10+PiB8IHVuZGVmaW5lZCkgPz8ge307XG4gICAgICBkaXNwYXRjaCh7XG4gICAgICAgIHR5cGU6IFwiU0VUX1dJTkRPV19NRUFTVVJFU19CWV9XSU5ET1dfSURcIixcbiAgICAgICAgdmFsdWU6IChjdXJyZW50KSA9PiBtZXJnZVJlY29yZFByZXNlcnZpbmdJZGVudGl0eShjdXJyZW50LCBPYmplY3QuZW50cmllcyhkeW5hbWljTWVhc3VyZXMpKSxcbiAgICAgIH0pO1xuICAgICAgY29uc3QgZHluYW1pY1Rvb2xNZWFzdXJlcyA9IChjYWNoZS5nZXQoXCJ0b29sc1wiKT8udmFsdWUgYXMgUmVhZG9ubHk8UmVjb3JkPHN0cmluZywgcmVhZG9ubHkgV2luZG93TWVhc3VyZVtdPj4gfCB1bmRlZmluZWQpID8/IHt9O1xuICAgICAgZGlzcGF0Y2goe1xuICAgICAgICB0eXBlOiBcIlNFVF9UT09MX01FQVNVUkVTX0JZX1RPT0xfSURcIixcbiAgICAgICAgdmFsdWU6IChjdXJyZW50KSA9PiBtZXJnZVJlY29yZFByZXNlcnZpbmdJZGVudGl0eShjdXJyZW50LCBPYmplY3QuZW50cmllcyhkeW5hbWljVG9vbE1lYXN1cmVzKSksXG4gICAgICB9KTtcbiAgICAgIGNvbnN0IGZyZXNoQXBwTGFiZWxzT3ZlcmxheSA9IG5vcm1hbGl6ZUFwcExhYmVsc092ZXJsYXkoY2FjaGUuZ2V0KFwibGFiZWxzXCIpPy52YWx1ZSBhcyBQYXJ0aWFsPFBsdWdpbkFwcExhYmVsc092ZXJsYXk+IHwgdW5kZWZpbmVkKTtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfQVBQX0xBQkVMU19PVkVSTEFZXCIsIHZhbHVlOiAoY3VycmVudCkgPT4gcHJlc2VydmVKc29uSWRlbnRpdHkoY3VycmVudCwgZnJlc2hBcHBMYWJlbHNPdmVybGF5KSB9KTtcbiAgICAgIGRpc3BhdGNoKHtcbiAgICAgICAgdHlwZTogXCJTRVRfUEFORUxfVUlfQllfS0VZXCIsXG4gICAgICAgIHZhbHVlOiAoY3VycmVudCkgPT5cbiAgICAgICAgICBtZXJnZVJlY29yZFByZXNlcnZpbmdJZGVudGl0eShcbiAgICAgICAgICAgIGN1cnJlbnQsXG4gICAgICAgICAgICBwYW5lbFRhYkxlYXZlc1xuICAgICAgICAgICAgICAuZmlsdGVyKCh0YWIpID0+IHRhYi5ib2R5S2V5KVxuICAgICAgICAgICAgICAubWFwKCh0YWIpID0+IFtwYW5lbFRhYktpbmRJZCh0YWIua2luZCksIChjYWNoZS5nZXQoYHBhbmVsOiR7cGFuZWxUYWJLaW5kSWQodGFiLmtpbmQpfWApPy52YWx1ZSBhcyBVaU5vZGUgfCB1bmRlZmluZWQpID8/IGN1cnJlbnRbcGFuZWxUYWJLaW5kSWQodGFiLmtpbmQpXSA/PyBwZW5kaW5nUGFuZWxVaU5vZGUoKV0gYXMgY29uc3QpLFxuICAgICAgICAgICksXG4gICAgICB9KTtcbiAgICAgIGlmIChpc1Nlc3Npb25Td2l0Y2ggJiYgbGF5b3V0U2VlZCkge1xuICAgICAgICBsYXlvdXRTZWVkS2V5UmVmLmN1cnJlbnQgPSBsYXlvdXRTZWVkS2V5O1xuICAgICAgICBleHRyYVdpbmRvd0luc3RhbmNlc1JlZi5jdXJyZW50ID0gbGF5b3V0U2VlZC5leHRyYUluc3RhbmNlcztcbiAgICAgICAgZXh0cmFXaW5kb3dDb3VudGVyUmVmLmN1cnJlbnQgPSBsYXlvdXRTZWVkLmV4dHJhSW5zdGFuY2VzLmxlbmd0aDtcbiAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9FWFRSQV9XSU5ET1dfSU5TVEFOQ0VTXCIsIHZhbHVlOiBsYXlvdXRTZWVkLmV4dHJhSW5zdGFuY2VzIH0pO1xuICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1NIRUxMX0xBWU9VVFwiLCB2YWx1ZTogbGF5b3V0U2VlZC5tb2RlTGF5b3V0IH0pO1xuICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0FDVElWRV9XSU5ET1dfSURcIiwgdmFsdWU6IG51bGwgfSk7XG4gICAgICB9XG4gICAgfSxcbiAgICAvLyDwn5Ci77iPIGBhcHBseUhvc3RFZmZlY3RzYCBpcyBkZWNsYXJlZCBsYXRlciBpbiB0aGlzIGNvbXBvbmVudCAoaXRzIG93biBkZXBzIG5lZWQgYHVwZGF0ZVNwYWNlUGFuZWxgL1xuICAgIC8vIGBzeW5jU3Bhd25lZFBsdWdpbkRvY3VtZW50YCwgZGVjbGFyZWQgbGF0ZXIgc3RpbGwpIOKAlCByZWZlcmVuY2luZyBpdCBoZXJlIGluIHRoZSBib2R5IG9ubHkgKG5ldmVyXG4gICAgLy8gYWRkZWQgdG8gdGhpcyBhcnJheSkgYXZvaWRzIGEgdGVtcG9yYWwtZGVhZC16b25lIHJlZmVyZW5jZS1iZWZvcmUtaW5pdDsgc2FmZSBiZWNhdXNlIHRoaXMgY2FsbGJhY2tcbiAgICAvLyBpcyBvbmx5IGV2ZXIgaW52b2tlZCBhZnRlciByZW5kZXIgY29tcGxldGVzLCBieSB3aGljaCBwb2ludCBgYXBwbHlIb3N0RWZmZWN0c2AgaXMgaW5pdGlhbGl6ZWQuXG4gICAgLy8gZXNsaW50LWRpc2FibGUtbmV4dC1saW5lIHJlYWN0LWhvb2tzL2V4aGF1c3RpdmUtZGVwc1xuICAgIFthcHBMYWJlbHNPdmVybGF5LCBpbmplY3RBY3RpdmVUb29sLCBsb2FkZWRQbHVnaW5zLCB1aUxvY2FsZSwgdWlUZXJtaW5vbG9neV0sXG4gICk7XG5cbiAgLyoqIEBlbW9qaSDwn5ej77iPIEtlZXBzIGFscmVhZHktYnVpbHQgd2luZG93IHRpdGxlcyAod29ya2JlbmNoIGxheW91dCwgZXh0cmEgc3Bhd25lZCB3aW5kb3dzKSBpbiBzeW5jIG9uIGV2ZXJ5IGxvY2FsZS90ZXJtaW5vbG9neSBzd2l0Y2gg4oCUIGByZWZyZXNoVWlgIG9ubHkgcmVidWlsZHMgYHNoZWxsTGF5b3V0YCBmcm9tIHNjcmF0Y2ggb24gYSBzZXNzaW9uIGNoYW5nZSwgc28gYW4gZXhpc3Rpbmcgc2Vzc2lvbidzIGJha2VkLWluIHRpdGxlcyB3b3VsZCBvdGhlcndpc2UgZ28gc3RhbGUuICovXG4gIHVzZUVmZmVjdCgoKSA9PiB7XG4gICAgY29uc3Qgd2luZG93S2luZHMgPSBzZXNzaW9uPy5hcHAud2luZG93S2luZHM7XG4gICAgaWYgKCF3aW5kb3dLaW5kcykgcmV0dXJuO1xuICAgIGRpc3BhdGNoKHtcbiAgICAgIHR5cGU6IFwiU0VUX1NIRUxMX0xBWU9VVFwiLFxuICAgICAgdmFsdWU6IChjdXJyZW50KSA9PiAoY3VycmVudCA/IHJldGl0bGVXaW5kb3dMYXlvdXROb2RlKGN1cnJlbnQsIHdpbmRvd0tpbmRzLCBleHRyYVdpbmRvd0luc3RhbmNlc1JlZi5jdXJyZW50LCB1aVRlcm1pbm9sb2d5LCB1aUxvY2FsZSkgOiBjdXJyZW50KSxcbiAgICB9KTtcbiAgICBkaXNwYXRjaCh7XG4gICAgICB0eXBlOiBcIlNFVF9FWFRSQV9XSU5ET1dfSU5TVEFOQ0VTXCIsXG4gICAgICB2YWx1ZTogKGN1cnJlbnQpID0+IHtcbiAgICAgICAgY29uc3QgbmV4dCA9IGN1cnJlbnQubWFwKChlbnRyeSkgPT4ge1xuICAgICAgICAgIGNvbnN0IGtpbmQgPSB3aW5kb3dLaW5kcy5maW5kKChrKSA9PiBrLmlkID09PSBlbnRyeS53aW5kb3dLaW5kSWQgfHwgay5pZCA9PT0gZW50cnkuaWQpO1xuICAgICAgICAgIGNvbnN0IHRpdGxlID0ga2luZCA/IHJlc29sdmVNYW5pZmVzdExhYmVsKGtpbmQubGFiZWwsIHVpVGVybWlub2xvZ3ksIHVpTG9jYWxlKSA6IGVudHJ5LnRpdGxlO1xuICAgICAgICAgIHJldHVybiB7IC4uLmVudHJ5LCB0aXRsZSB9O1xuICAgICAgICB9KTtcbiAgICAgICAgZXh0cmFXaW5kb3dJbnN0YW5jZXNSZWYuY3VycmVudCA9IG5leHQ7XG4gICAgICAgIHJldHVybiBuZXh0O1xuICAgICAgfSxcbiAgICB9KTtcbiAgfSwgW3VpVGVybWlub2xvZ3ksIHVpTG9jYWxlXSk7XG5cbiAgY29uc3QgcmVmcmVzaFNwYXduZWRVaSA9IHVzZUNhbGxiYWNrKFxuICAgIGFzeW5jIChzcGF3bmVkOiBTcGF3bmVkQXBwRW50cnksIHZpZXdTdGF0ZTogVmlld01vZGVsLCBzY29wZUFyZzogVWlEaXJ0eVNjb3BlID0geyBraW5kOiBcImZ1bGxcIiB9KSA9PiB7XG4gICAgICBpZiAoc2NvcGVBcmcua2luZCA9PT0gXCJub25lXCIpIHJldHVybjtcbiAgICAgIGNvbnN0IGdlbmVyYXRpb24gPSArK3NwYXduZWRSZWZyZXNoR2VuZXJhdGlvblJlZi5jdXJyZW50O1xuICAgICAgY29uc3QgcGx1Z2luRW50cnkgPSBsb2FkZWRQbHVnaW5zLmZpbmQoKGVudHJ5KSA9PiBlbnRyeS5oYW5kbGUucGx1Z2luSWQgPT09IHNwYXduZWQucGx1Z2luSWQpO1xuICAgICAgY29uc3QgcGx1Z2luID0gcGx1Z2luRW50cnk/LmhhbmRsZTtcbiAgICAgIGNvbnN0IGFwcCA9IHBsdWdpbkVudHJ5Py5tYW5pZmVzdC5hcHBzLmZpbmQoKGNhbmRpZGF0ZSkgPT4gY2FuZGlkYXRlLmlkID09PSBzcGF3bmVkLmFwcElkKTtcbiAgICAgIGlmICghcGx1Z2luIHx8ICFhcHApIHtcbiAgICAgICAgY29uc29sZS53YXJuKFwiW29zLXNoZWxsXSByZWZyZXNoU3Bhd25lZFVpOiBwbHVnaW4vYXBwIHVuYXZhaWxhYmxlXCIsIHsgcGx1Z2luSWQ6IHNwYXduZWQucGx1Z2luSWQsIGFwcElkOiBzcGF3bmVkLmFwcElkIH0pO1xuICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1NQQVdORURfV0lORE9XX1VJXCIsIHZhbHVlOiB7IHR5cGU6IFwidGV4dFwiLCB2YWx1ZTogYFBsdWdpbiB1bmF2YWlsYWJsZTogJHtzcGF3bmVkLnBsdWdpbklkfS8ke3NwYXduZWQuYXBwSWR9YCB9IGFzIFVpTm9kZSB9KTtcbiAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9TUEFXTkVEX1dJTkRPV19FTkdBR0VNRU5UU1wiLCB2YWx1ZToge30gfSk7XG4gICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfU1BBV05FRF9XSU5ET1dfTUVBU1VSRVNcIiwgdmFsdWU6IHt9IH0pO1xuICAgICAgICByZXR1cm47XG4gICAgICB9XG4gICAgICBjb25zdCBzcGF3bmVkU2VlZCA9IGAke3NwYXduZWQucGx1Z2luSWR9OiR7c3Bhd25lZC5hcHBJZH06JHtzcGF3bmVkLmluc3RhbmNlSWR9YDtcbiAgICAgIGlmIChzcGF3bmVkTGF5b3V0U2VlZFJlZi5jdXJyZW50ICE9PSBzcGF3bmVkU2VlZCkge1xuICAgICAgICBzcGF3bmVkTGF5b3V0U2VlZFJlZi5jdXJyZW50ID0gc3Bhd25lZFNlZWQ7XG4gICAgICAgIHNwYXduZWRVaVJlZnJlc2hDYWNoZVJlZi5jdXJyZW50ID0gbmV3IE1hcCgpO1xuICAgICAgfVxuICAgICAgY29uc3QgY2FjaGUgPSBzcGF3bmVkVWlSZWZyZXNoQ2FjaGVSZWYuY3VycmVudDtcbiAgICAgIGNvbnN0IGNvbnRyaWJ1dGlvbnNKc29uID0gYnVpbGRDb250cmlidXRpb25zSnNvbihsb2FkZWRQbHVnaW5zLm1hcCgoZW50cnkpID0+ICh7IHBsdWdpbklkOiBlbnRyeS5oYW5kbGUucGx1Z2luSWQsIG1hbmlmZXN0OiBlbnRyeS5tYW5pZmVzdCB9KSkpO1xuICAgICAgY29uc3QgYm9keUtleSA9IHJlc29sdmVDYW52YXNCb2R5S2V5KGFwcCk7XG4gICAgICBjb25zdCBmdWxsVmlld1N0YXRlOiBWaWV3TW9kZWwgPSBpbmplY3RBY3RpdmVVdGlsaXR5KFxuICAgICAgICB7IC4uLnZpZXdTdGF0ZSwgY29udHJpYnV0aW9uc0pzb24sIGxvY2FsZTogdWlMb2NhbGUsIHRlcm1pbm9sb2d5OiB1aVRlcm1pbm9sb2d5LCB3aW5kb3dJZDogYm9keUtleSwgd2luZG93SW5zdGFuY2VzOiBbeyBpZDogYm9keUtleSwgd2luZG93S2luZElkOiBib2R5S2V5IH1dIH0sXG4gICAgICAgIHNwYXduZWQuaWQsXG4gICAgICApO1xuICAgICAgLy8g8J+Qou+4jyBBIHNwYXduZWQgaW5zdGFuY2UncyB2aWV3IGlzIGEgc2luZ2xlIGJvZHkgKyB1dGlsaXRpZXMgKyBlbmdhZ2VtZW50cyArIG1lYXN1cmVzIChubyBwYW5lbHMsIG5vXG4gICAgICAvLyBsYWJlbHMpIOKAlCB0aGF0J3MgYWxyZWFkeSB0aGUgbWluaW1hbCBncm91cGluZywgc28gdGhlcmUgaXMgbm8gbmFycm93ZXItdGhhbi1mdWxsIFwicGFydGlhbFwiIHNjb3BlXG4gICAgICAvLyB3b3J0aCBleHByZXNzaW5nIGhlcmU7IG9ubHkgYG5vbmVgIChoYW5kbGVkIGFib3ZlKSBzaG9ydC1jaXJjdWl0cyB0aGUgcmVxdWVzdC5cbiAgICAgIGNvbnN0IHNpbmdsZVdpbmRvd0tpbmQgPSBbeyBpZDogYm9keUtleSwgYm9keUtleSB9XTtcbiAgICAgIGNvbnN0IHJlcXVlc3QgPSBidWlsZFVpUmVmcmVzaFJlcXVlc3QoeyBraW5kOiBcImZ1bGxcIiB9LCBzaW5nbGVXaW5kb3dLaW5kLCBbXSwgZnVsbFZpZXdTdGF0ZSwgY2FjaGUpO1xuICAgICAgaWYgKHJlcXVlc3QpIHtcbiAgICAgICAgY29uc3QgcmVzcG9uc2UgPSBhd2FpdCBwbHVnaW4ucmVmcmVzaFVpKHNwYXduZWQuaW5zdGFuY2VJZCwgcmVxdWVzdCk7XG4gICAgICAgIGlmIChnZW5lcmF0aW9uICE9PSBzcGF3bmVkUmVmcmVzaEdlbmVyYXRpb25SZWYuY3VycmVudCkgcmV0dXJuO1xuICAgICAgICBhcHBseVVpUmVmcmVzaFJlc3BvbnNlVG9DYWNoZShjYWNoZSwgcmVzcG9uc2UpO1xuICAgICAgfVxuICAgICAgY29uc3QgdWkgPSAoY2FjaGUuZ2V0KGB3aW5kb3c6JHtib2R5S2V5fWApPy52YWx1ZSBhcyBVaU5vZGUgfCB1bmRlZmluZWQpID8/IHBlbmRpbmdXaW5kb3dVaU5vZGUoKTtcbiAgICAgIGNvbnN0IGR5bmFtaWNFbmdhZ2VtZW50cyA9IChjYWNoZS5nZXQoXCJlbmdhZ2VtZW50c1wiKT8udmFsdWUgYXMgUmVhZG9ubHk8UmVjb3JkPHN0cmluZywgV2luZG93RW5nYWdlbWVudD4+IHwgdW5kZWZpbmVkKSA/PyB7fTtcbiAgICAgIGNvbnN0IGR5bmFtaWNNZWFzdXJlcyA9IChjYWNoZS5nZXQoXCJtZWFzdXJlc1wiKT8udmFsdWUgYXMgUmVhZG9ubHk8UmVjb3JkPHN0cmluZywgcmVhZG9ubHkgV2luZG93TWVhc3VyZVtdPj4gfCB1bmRlZmluZWQpID8/IHt9O1xuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9TUEFXTkVEX1dJTkRPV19VSVwiLCB2YWx1ZTogKGN1cnJlbnQ6IFVpTm9kZSB8IG51bGwpID0+IHByZXNlcnZlSnNvbklkZW50aXR5KGN1cnJlbnQgPz8gdW5kZWZpbmVkLCB1aSkgfSk7XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1NQQVdORURfV0lORE9XX0VOR0FHRU1FTlRTXCIsIHZhbHVlOiBkeW5hbWljRW5nYWdlbWVudHMgfSk7XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1NQQVdORURfV0lORE9XX01FQVNVUkVTXCIsIHZhbHVlOiBkeW5hbWljTWVhc3VyZXMgfSk7XG4gICAgfSxcbiAgICBbaW5qZWN0QWN0aXZlVXRpbGl0eSwgbG9hZGVkUGx1Z2lucywgdWlMb2NhbGUsIHVpVGVybWlub2xvZ3ldLFxuICApO1xuXG4gIC8vIPCfkKLvuI8gS2V5ZWQgb24gdGhlIHBsdWdpbklkL2FwcC9pbnN0YW5jZSB0cmlwbGUgKG5vdCBgc2Vzc2lvbmAgb2JqZWN0IGlkZW50aXR5KSBzbyB0aGlzIG9ubHkgZmlyZXMgb25cbiAgLy8gYSBnZW51aW5lIHNlc3Npb24gc3dpdGNoIChhcHAgb3Blbi9zcGF3bi9pbnN0YW5jZSBjaGFuZ2UpIOKAlCBldmVyeSBvdGhlciBhY3Rpb24gYWxyZWFkeSBjYWxsc1xuICAvLyBgcmVmcmVzaFVpYCBleHBsaWNpdGx5IHZpYSBgYXBwbHlIb3N0RWZmZWN0c2AsIGFuZCByZS1ydW5uaW5nIGl0IGhlcmUgdG9vIG9uIGV2ZXJ5IGBzZXNzaW9uYCBvYmplY3RcbiAgLy8gY2h1cm4gd2FzIGEgc2Vjb25kLCByZWR1bmRhbnQgZnVsbC1zaGVsbCByZWZyZXNoIGNhc2NhZGUgcGVyIGludGVyYWN0aW9uLlxuICBjb25zdCBzZXNzaW9uSWRlbnRpdHlLZXkgPSBzZXNzaW9uID8gYCR7c2Vzc2lvbi5wbHVnaW5JZH06JHtzZXNzaW9uLmFwcC5pZH06JHtzZXNzaW9uLmluc3RhbmNlSWR9YCA6IG51bGw7XG4gIHVzZUVmZmVjdCgoKSA9PiB7XG4gICAgY29uc3QgY3VycmVudCA9IHNlc3Npb25SZWYuY3VycmVudDtcbiAgICBpZiAoIWN1cnJlbnQpIHJldHVybjtcbiAgICB2b2lkIHJlZnJlc2hVaShjdXJyZW50KS5jYXRjaCgocmVuZGVyRXJyb3IpID0+IHtcbiAgICAgIGNvbnNvbGUuZXJyb3IoXCJbREVCVUddIHJlbmRlciBmYWlsZWRcIiwgcmVuZGVyRXJyb3IpO1xuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9FUlJPUlwiLCB2YWx1ZTogcmVuZGVyRXJyb3IgaW5zdGFuY2VvZiBFcnJvciA/IHJlbmRlckVycm9yLm1lc3NhZ2UgOiBTdHJpbmcocmVuZGVyRXJyb3IpIH0pO1xuICAgIH0pO1xuICB9LCBbbG9hZGVkUGx1Z2lucywgcmVmcmVzaFVpLCBzZXNzaW9uSWRlbnRpdHlLZXldKTtcblxuICB1c2VFZmZlY3QoKCkgPT4ge1xuICAgIGlmICghc3R1ZGlvTW9kZSB8fCAhc2Vzc2lvbikge1xuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9TUEFXTkVEX1dJTkRPV19VSVwiLCB2YWx1ZTogbnVsbCB9KTtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfU1BBV05FRF9XSU5ET1dfRU5HQUdFTUVOVFNcIiwgdmFsdWU6IHt9IH0pO1xuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9TUEFXTkVEX1dJTkRPV19NRUFTVVJFU1wiLCB2YWx1ZToge30gfSk7XG4gICAgICByZXR1cm47XG4gICAgfVxuICAgIGNvbnN0IGFjdGl2ZVNwYXduZWQgPSBwYW5lbD8uc3Bhd25lZEFwcHMuZmluZCgoZW50cnkpID0+IGVudHJ5LmlkID09PSBwYW5lbC5hY3RpdmVTcGF3bmVkSWQpO1xuICAgIGlmICghYWN0aXZlU3Bhd25lZCkge1xuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9TUEFXTkVEX1dJTkRPV19VSVwiLCB2YWx1ZTogbnVsbCB9KTtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfU1BBV05FRF9XSU5ET1dfRU5HQUdFTUVOVFNcIiwgdmFsdWU6IHt9IH0pO1xuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9TUEFXTkVEX1dJTkRPV19NRUFTVVJFU1wiLCB2YWx1ZToge30gfSk7XG4gICAgICByZXR1cm47XG4gICAgfVxuICAgIHZvaWQgcmVmcmVzaFNwYXduZWRVaShhY3RpdmVTcGF3bmVkLCBzZXNzaW9uLnZpZXdTdGF0ZSkuY2F0Y2goKHJlbmRlckVycm9yKSA9PiB7XG4gICAgICBjb25zb2xlLmVycm9yKFwiW0RFQlVHXSBzcGF3bmVkIHJlbmRlciBmYWlsZWRcIiwgcmVuZGVyRXJyb3IpO1xuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9TUEFXTkVEX1dJTkRPV19VSVwiLCB2YWx1ZTogbnVsbCB9KTtcbiAgICB9KTtcbiAgfSwgW2xvYWRlZFBsdWdpbnMsIHBhbmVsLCByZWZyZXNoU3Bhd25lZFVpLCBzZXNzaW9uLCBzdHVkaW9Nb2RlXSk7XG5cbiAgY29uc3QgdXBkYXRlU3BhY2VQYW5lbCA9IHVzZUNhbGxiYWNrKChwYW5lbFN0YXRlOiBTcGFjZVBhbmVsU3RhdGUpID0+IHtcbiAgICBkaXNwYXRjaCh7XG4gICAgICB0eXBlOiBcIlNFVF9TRVNTSU9OXCIsXG4gICAgICB2YWx1ZTogKGN1cnJlbnQpID0+IHtcbiAgICAgICAgaWYgKCFjdXJyZW50KSByZXR1cm4gY3VycmVudDtcbiAgICAgICAgcmV0dXJuIHsgLi4uY3VycmVudCwgdmlld1N0YXRlOiB7IC4uLmN1cnJlbnQudmlld1N0YXRlLCBwYW5lbEpzb246IHBhbmVsSnNvbkZyb21TdGF0ZShwYW5lbFN0YXRlKSB9IH07XG4gICAgICB9LFxuICAgIH0pO1xuICB9LCBbXSk7XG5cbiAgLy8g8J+PoO+4j/Cfp7PvuI8gR2VuZXJpYyByZXBsYWNlbWVudCBmb3IgdGhlIG9sZCBgc3dpdGNoVG9TQXBwYCDigJQgc3dpdGNoZXMgdG8gZWl0aGVyIHRoZSBob3N0IHBsdWdpbidzIGxhbmRpbmdcbiAgLy8gb3IgaG9zdCBhcHAgYnkgaWQgKGJvdGggcmVzb2x2ZWQgdmlhIGBob3N0Q29uZmlnYCwgbmV2ZXIgYSBzcGVjaWZpYyBhcHAncyBpZGVudGl0eSkuXG4gIGNvbnN0IHN3aXRjaFRvTWFuYWdlZEFwcCA9IHVzZUNhbGxiYWNrKFxuICAgIGFzeW5jIChhcHBJZDogc3RyaW5nLCB2aWV3U3RhdGU/OiBWaWV3TW9kZWwpOiBQcm9taXNlPEFjdGl2ZVNlc3Npb24gfCBudWxsPiA9PiB7XG4gICAgICBjb25zdCBzUGx1Z2luID0gaG9zdENvbmZpZyA/IGxvYWRlZFBsdWdpbnMuZmluZCgoZW50cnkpID0+IGVudHJ5LmhhbmRsZS5wbHVnaW5JZCA9PT0gaG9zdENvbmZpZy5wbHVnaW5JZCkgOiB1bmRlZmluZWQ7XG4gICAgICBjb25zdCBhcHAgPSBzUGx1Z2luPy5tYW5pZmVzdC5hcHBzLmZpbmQoKGNhbmRpZGF0ZSkgPT4gY2FuZGlkYXRlLmlkID09PSBhcHBJZCk7XG4gICAgICBpZiAoIXNQbHVnaW4gfHwgIWFwcCkgcmV0dXJuIG51bGw7XG4gICAgICBpZiAoc2Vzc2lvbj8ucGx1Z2luSWQgPT09IHNQbHVnaW4uaGFuZGxlLnBsdWdpbklkICYmIHNlc3Npb24uYXBwLmlkID09PSBhcHBJZCkge1xuICAgICAgICBpZiAoIXZpZXdTdGF0ZSkgcmV0dXJuIHNlc3Npb247XG4gICAgICAgIGNvbnN0IG5leHRTZXNzaW9uOiBBY3RpdmVTZXNzaW9uID0geyAuLi5zZXNzaW9uLCB2aWV3U3RhdGUgfTtcbiAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9TRVNTSU9OXCIsIHZhbHVlOiBuZXh0U2Vzc2lvbiB9KTtcbiAgICAgICAgYXdhaXQgcmVmcmVzaFVpKG5leHRTZXNzaW9uKTtcbiAgICAgICAgcmV0dXJuIG5leHRTZXNzaW9uO1xuICAgICAgfVxuICAgICAgY29uc3QgaW5zdGFuY2VJZCA9IGF3YWl0IHNQbHVnaW4uaGFuZGxlLmNyZWF0ZUFwcChhcHAuaWQpO1xuICAgICAgLy8g8J+qpu+4jyBTZWUgYGVzdGFibGlzaFByaW1hcnlTZXNzaW9uYCdzIGNvbW1lbnQgYWJvdmUg4oCUIGBwcm9ncmFtc2AgaXMgcGVybWFuZW50bHkgZW1wdHkgbm93LlxuICAgICAgY29uc3QgbmV4dFZpZXdTdGF0ZTogVmlld01vZGVsID0gdmlld1N0YXRlID8/IHtcbiAgICAgICAgYWN0aXZlTW9kZUlkOiBhcHAuZGVmYXVsdE1vZGVJZCA/PyBhcHAubW9kZXNbMF0/LmlkLFxuICAgICAgICBwYW5lbEpzb246IHBhbmVsSnNvbkZyb21TdGF0ZShidWlsZFNwYWNlUGFuZWxTdGF0ZShbXSwgW10pKSxcbiAgICAgIH07XG4gICAgICBjb25zdCBuZXh0U2Vzc2lvbjogQWN0aXZlU2Vzc2lvbiA9IHsgcGx1Z2luSWQ6IHNQbHVnaW4uaGFuZGxlLnBsdWdpbklkLCBpbnN0YW5jZUlkLCBhcHAsIHZpZXdTdGF0ZTogbmV4dFZpZXdTdGF0ZSB9O1xuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9TRVNTSU9OXCIsIHZhbHVlOiBuZXh0U2Vzc2lvbiB9KTtcbiAgICAgIGNvbnN0IHNlZWRlZCA9IGFwcGx5RnJhbWV3b3JrTGF5b3V0U2VlZChhcHAuZGVmYXVsdExheW91dCwgYXBwLndpbmRvd0tpbmRzLCBhcHBMYWJlbHNPdmVybGF5LCB1aVRlcm1pbm9sb2d5LCB1aUxvY2FsZSk7XG4gICAgICBleHRyYVdpbmRvd0luc3RhbmNlc1JlZi5jdXJyZW50ID0gc2VlZGVkLmV4dHJhSW5zdGFuY2VzO1xuICAgICAgZXh0cmFXaW5kb3dDb3VudGVyUmVmLmN1cnJlbnQgPSBzZWVkZWQuZXh0cmFJbnN0YW5jZXMubGVuZ3RoO1xuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9FWFRSQV9XSU5ET1dfSU5TVEFOQ0VTXCIsIHZhbHVlOiBzZWVkZWQuZXh0cmFJbnN0YW5jZXMgfSk7XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1NIRUxMX0xBWU9VVFwiLCB2YWx1ZTogc2VlZGVkLm1vZGVMYXlvdXQgfSk7XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0FDVElWRV9XSU5ET1dfSURcIiwgdmFsdWU6IG51bGwgfSk7XG4gICAgICBpZiAoYXBwSWQgPT09IGxhbmRpbmdBcHBJZCkge1xuICAgICAgICBvcGVuU3BhY2VJZFJlZi5jdXJyZW50ID0gbnVsbDtcbiAgICAgICAgb3Blbkluc3RhbmNlSWRSZWYuY3VycmVudCA9IG51bGw7XG4gICAgICB9XG4gICAgICBhd2FpdCByZWZyZXNoVWkobmV4dFNlc3Npb24pO1xuICAgICAgcmV0dXJuIG5leHRTZXNzaW9uO1xuICAgIH0sXG4gICAgW2xvYWRlZFBsdWdpbnMsIHJlZnJlc2hVaSwgc2Vzc2lvbiwgYXBwTGFiZWxzT3ZlcmxheSwgaG9zdENvbmZpZywgbGFuZGluZ0FwcElkLCB1aVRlcm1pbm9sb2d5LCB1aUxvY2FsZV0sXG4gICk7XG5cbiAgY29uc3Qgc3luY1NwYXduZWRQbHVnaW5Eb2N1bWVudCA9IHVzZUNhbGxiYWNrKGFzeW5jIChwbHVnaW46IFBsdWdpbldhc21IYW5kbGUsIGFwcDogQXBwRGVmaW5pdGlvbiwgcGx1Z2luSW5zdGFuY2VJZDogbnVtYmVyLCBkb2N1bWVudEpzb246IHN0cmluZywgdmlld1N0YXRlOiBWaWV3TW9kZWwpID0+IHtcbiAgICB0cnkge1xuICAgICAgY29uc3QgZG9jdW1lbnQgPSBKU09OLnBhcnNlKGRvY3VtZW50SnNvbikgYXMgUmVjb3JkPHN0cmluZywgdW5rbm93bj47XG4gICAgICBhd2FpdCBwbHVnaW4uaGFuZGxlQWN0aW9uKHBsdWdpbkluc3RhbmNlSWQsIGVuY29kZUFjdGlvbldpcmUoeyBjb250cm9sbGVySWQ6IGFwcC5jb250cm9sbGVySWQsIGFjdGlvbjogXCJzZXREb2N1bWVudFwiLCBhcmdzOiB7IGRvY3VtZW50IH0gfSksIHZpZXdTdGF0ZSk7XG4gICAgfSBjYXRjaCAoc3luY0Vycm9yKSB7XG4gICAgICBjb25zb2xlLmVycm9yKFwiW0RFQlVHXSBzcGF3bmVkIHByb2dyYW0gZG9jdW1lbnQgc3luYyBmYWlsZWRcIiwgc3luY0Vycm9yKTtcbiAgICB9XG4gIH0sIFtdKTtcblxuICBjb25zdCBlbnN1cmVTcGF3bmVkUGx1Z2luID0gdXNlQ2FsbGJhY2soXG4gICAgYXN5bmMgKHByb2dyYW06IFNwYWNlUHJvZ3JhbUVudHJ5LCBsYWJlbD86IHN0cmluZywgb3NJbnN0YW5jZUlkPzogc3RyaW5nLCBkb2N1bWVudEpzb24/OiBzdHJpbmcsIHNvdXJjZVZpZXdTdGF0ZT86IFZpZXdNb2RlbCk6IFByb21pc2U8U3BhY2VQYW5lbFN0YXRlIHwgbnVsbD4gPT4ge1xuICAgICAgY29uc3QgcGx1Z2luRW50cnkgPSBsb2FkZWRQbHVnaW5zLmZpbmQoKGVudHJ5KSA9PiBlbnRyeS5oYW5kbGUucGx1Z2luSWQgPT09IHByb2dyYW0ucGx1Z2luSWQpO1xuICAgICAgaWYgKCFwbHVnaW5FbnRyeSB8fCAhc2Vzc2lvbikgcmV0dXJuIG51bGw7XG4gICAgICBjb25zdCBhcHAgPSBwbHVnaW5FbnRyeS5tYW5pZmVzdC5hcHBzLmZpbmQoKGNhbmRpZGF0ZSkgPT4gY2FuZGlkYXRlLmlkID09PSBwcm9ncmFtLmFwcElkKTtcbiAgICAgIGNvbnN0IGN1cnJlbnRQYW5lbCA9IHBhcnNlUGFuZWxTdGF0ZShzb3VyY2VWaWV3U3RhdGUgPz8gc2Vzc2lvbi52aWV3U3RhdGUpID8/IGJ1aWxkU3BhY2VQYW5lbFN0YXRlKFtdLCBbXSk7XG4gICAgICBjb25zdCBleGlzdGluZyA9IG9zSW5zdGFuY2VJZCA/IGN1cnJlbnRQYW5lbC5zcGF3bmVkQXBwcy5maW5kKChlbnRyeSkgPT4gZW50cnkuaWQgPT09IG9zSW5zdGFuY2VJZCkgOiBjdXJyZW50UGFuZWwuc3Bhd25lZEFwcHMuZmluZCgoZW50cnkpID0+IGVudHJ5LmFwcElkID09PSBwcm9ncmFtLmFwcElkICYmIGVudHJ5LnBsdWdpbklkID09PSBwcm9ncmFtLnBsdWdpbklkKTtcbiAgICAgIGlmIChleGlzdGluZykge1xuICAgICAgICBpZiAoZG9jdW1lbnRKc29uICYmIGFwcCkge1xuICAgICAgICAgIGF3YWl0IHN5bmNTcGF3bmVkUGx1Z2luRG9jdW1lbnQocGx1Z2luRW50cnkuaGFuZGxlLCBhcHAsIGV4aXN0aW5nLmluc3RhbmNlSWQsIGRvY3VtZW50SnNvbiwgc291cmNlVmlld1N0YXRlID8/IHNlc3Npb24udmlld1N0YXRlKTtcbiAgICAgICAgfVxuICAgICAgICByZXR1cm4gc3R1ZGlvUGFuZWxGb2N1c2luZ1NwYXduZWQoY3VycmVudFBhbmVsLCBleGlzdGluZyk7XG4gICAgICB9XG4gICAgICBjb25zdCBpbnN0YW5jZUlkID0gYXdhaXQgcGx1Z2luRW50cnkuaGFuZGxlLmNyZWF0ZUFwcChwcm9ncmFtLmFwcElkKTtcbiAgICAgIGlmIChkb2N1bWVudEpzb24gJiYgYXBwKSB7XG4gICAgICAgIGF3YWl0IHN5bmNTcGF3bmVkUGx1Z2luRG9jdW1lbnQocGx1Z2luRW50cnkuaGFuZGxlLCBhcHAsIGluc3RhbmNlSWQsIGRvY3VtZW50SnNvbiwgc291cmNlVmlld1N0YXRlID8/IHNlc3Npb24udmlld1N0YXRlKTtcbiAgICAgIH1cbiAgICAgIGNvbnN0IHNwYXduZWRJZCA9IG9zSW5zdGFuY2VJZCA/PyBgJHtwcm9ncmFtLnBsdWdpbklkfS0ke2luc3RhbmNlSWR9YDtcbiAgICAgIHJldHVybiBzdHVkaW9QYW5lbEZvY3VzaW5nU3Bhd25lZChjdXJyZW50UGFuZWwsIHtcbiAgICAgICAgaWQ6IHNwYXduZWRJZCxcbiAgICAgICAgcGx1Z2luSWQ6IHByb2dyYW0ucGx1Z2luSWQsXG4gICAgICAgIGluc3RhbmNlSWQsXG4gICAgICAgIGFwcElkOiBwcm9ncmFtLmFwcElkLFxuICAgICAgICBsYWJlbDogbGFiZWwgPz8gcHJvZ3JhbS5sYWJlbCxcbiAgICAgICAgZG9jdW1lbnQ6IHByb2dyYW0uZG9jdW1lbnQsXG4gICAgICB9KTtcbiAgICB9LFxuICAgIFtsb2FkZWRQbHVnaW5zLCBzZXNzaW9uLCBzeW5jU3Bhd25lZFBsdWdpbkRvY3VtZW50XSxcbiAgKTtcblxuICAvKipcbiAgICog8J+Qmu+4jyBDb25zdW1lcyBhIHBsdWdpbiBhY3Rpb24ncyB0eXBlZCBgcmVxdWVzdGVkRWZmZWN0czogSG9zdEVmZmVjdFtdYCAoV1MtRCdzIGBJbnZvY2F0aW9uUmVzcG9uc2VgKSDigJRcbiAgICogcmVwbGFjZXMgdGhlIGRlbGV0ZWQgYHByb2Nlc3NQbHVnaW5PcGVyYXRpb25zYCBzdHJpbmctbWF0Y2hpbmcuIFRoZSBsZWdhY3kgYHNldERvY3VtZW50YC1taXJyb3JcbiAgICogYmFja2JvbmUtd3JpdGUgYmxvY2sgaXMgZ29uZSBlbnRpcmVseTogZG9jdW1lbnQgY29udGVudCBzeW5jIG5vdyBmbG93cyB0aHJvdWdoXG4gICAqIGBvcGVuRG9jdW1lbnRgL2BjbG9zZURvY3VtZW50YCdzIHdvcmtlci1iYWNrZWQgYERvY3VtZW50SG9zdGAgbGlmZWN5Y2xlLCBub3QgYSBwZXItb3BlcmF0aW9uIEpTIG1pcnJvci5cbiAgICovXG4gIGNvbnN0IGFwcGx5SG9zdEVmZmVjdHMgPSB1c2VDYWxsYmFjayhcbiAgICBhc3luYyAoZWZmZWN0czogcmVhZG9ubHkgSG9zdEVmZmVjdFtdLCBiYXNlU2Vzc2lvbjogQWN0aXZlU2Vzc2lvbiwgdWlTY29wZTogVWlEaXJ0eVNjb3BlID0geyBraW5kOiBcImZ1bGxcIiB9KSA9PiB7XG4gICAgICBsZXQgbmV4dFZpZXdTdGF0ZSA9IGJhc2VTZXNzaW9uLnZpZXdTdGF0ZTtcbiAgICAgIGZvciAoY29uc3QgZWZmZWN0IG9mIGVmZmVjdHMpIHtcbiAgICAgICAgaWYgKGVmZmVjdCA9PT0gXCJyZXF1ZXN0U3luY1wiKSBjb250aW51ZTtcbiAgICAgICAgaWYgKFwic2V0UGFuZWxcIiBpbiBlZmZlY3QpIHtcbiAgICAgICAgICBuZXh0Vmlld1N0YXRlID0geyAuLi5uZXh0Vmlld1N0YXRlLCBwYW5lbEpzb246IGVmZmVjdC5zZXRQYW5lbC5wYW5lbEpzb24gfTtcbiAgICAgICAgICBjb250aW51ZTtcbiAgICAgICAgfVxuICAgICAgICBpZiAoXCJzZXRBY3RpdmVVdGlsaXR5XCIgaW4gZWZmZWN0KSB7XG4gICAgICAgICAgLy8g8J+nsO+4jyBBIHByb2dyYW0gcHJvZ3JhbW1hdGljYWxseSBzd2l0Y2hlZCB1dGlsaXR5OiBtaXJyb3IgaXQgaW50byB0aGUgaG9zdC1vd25lZCBzdG9yZSBzbGljZSBBTkRcbiAgICAgICAgICAvLyB0aGUgcmVmIGByZWZyZXNoVWlgIHJlYWRzIChiYXJlIGBkaXNwYXRjaGAgYWxvbmUgbGVhdmVzIHRoZSBtYXAgc3RhbGUgdW50aWwgdGhlIG5leHQgcmVuZGVyIOKAlFxuICAgICAgICAgIC8vIHdoaWNoIGlzIGFmdGVyIHRoaXMgc2FtZSBwYXNzJ3MgcmVmcmVzaCwgc28gYnJ1c2gvc3VnZ2VzdGlvbiBnaG9zdHMgYW5kIGd1bWJhbGxzIG5ldmVyIGFwcGVhcikuXG4gICAgICAgICAgY29uc3QgeyB3aW5kb3dJZCwgdXRpbGl0eUlkIH0gPSBlZmZlY3Quc2V0QWN0aXZlVXRpbGl0eTtcbiAgICAgICAgICBzZXRBY3RpdmVVdGlsaXR5Rm9yV2luZG93KHdpbmRvd0lkLCB1dGlsaXR5SWQgfHwgbnVsbCk7XG4gICAgICAgICAgaWYgKHV0aWxpdHlJZCAmJiBhY3RpdmVUb29sSWRSZWYuY3VycmVudCkge1xuICAgICAgICAgICAgYWN0aXZlVG9vbElkUmVmLmN1cnJlbnQgPSBudWxsO1xuICAgICAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9BQ1RJVkVfVE9PTFwiLCB0b29sSWQ6IG51bGwgfSk7XG4gICAgICAgICAgfVxuICAgICAgICAgIGlmICh3aW5kb3dJZCA9PT0gYWN0aXZlV2luZG93SWRSZWYuY3VycmVudCkgbmV4dFZpZXdTdGF0ZSA9IHsgLi4ubmV4dFZpZXdTdGF0ZSwgYWN0aXZlVXRpbGl0eUlkOiB1dGlsaXR5SWQgfHwgdW5kZWZpbmVkLCBhY3RpdmVUb29sSWQ6IHV0aWxpdHlJZCA/IHVuZGVmaW5lZCA6IG5leHRWaWV3U3RhdGUuYWN0aXZlVG9vbElkIH07XG4gICAgICAgICAgY29udGludWU7XG4gICAgICAgIH1cbiAgICAgICAgaWYgKFwic2V0QWN0aXZlVG9vbFwiIGluIGVmZmVjdCkge1xuICAgICAgICAgIC8vIPCfm6DvuI8gQSBwcm9ncmFtIHByb2dyYW1tYXRpY2FsbHkgc3dpdGNoZWQgdG9vbHMgKGUuZy4gcHV6emxlM2QgZmlsbCB2aWEgZW5nYWdlbWVudCB0ZXh0IGNvbW1hbmQpOlxuICAgICAgICAgIC8vIG1pcnJvciBpdCBpbnRvIHRoZSBob3N0LW93bmVkIHN0b3JlIHNsaWNlLCBjbGVhciBldmVyeSB3aW5kb3cncyBhY3RpdmUgdXRpbGl0eSAobXV0dWFsXG4gICAgICAgICAgLy8gZXhjbHVzaW9uIOKAlCBhIHRvb2wgYW5kIGEgd2luZG93IHV0aWxpdHkgbmV2ZXIgYm90aCBjbGFpbSB0aGUgcG9pbnRlciksIGFuZCBmb2xkIGl0IGludG8gdGhlXG4gICAgICAgICAgLy8gdmlldyBzdGF0ZSBmZWQgdG8gdGhlIGZvbGxvdy11cCByZWZyZXNoLlxuICAgICAgICAgIGNvbnN0IHsgdG9vbElkIH0gPSBlZmZlY3Quc2V0QWN0aXZlVG9vbDtcbiAgICAgICAgICBhY3RpdmVUb29sSWRSZWYuY3VycmVudCA9IHRvb2xJZCB8fCBudWxsO1xuICAgICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfQUNUSVZFX1RPT0xcIiwgdG9vbElkOiB0b29sSWQgfHwgbnVsbCB9KTtcbiAgICAgICAgICBpZiAodG9vbElkKSBjbGVhckFsbFdpbmRvd1V0aWxpdGllcygpO1xuICAgICAgICAgIG5leHRWaWV3U3RhdGUgPSB7IC4uLm5leHRWaWV3U3RhdGUsIGFjdGl2ZVRvb2xJZDogdG9vbElkIHx8IHVuZGVmaW5lZCwgYWN0aXZlVXRpbGl0eUlkOiB0b29sSWQgPyB1bmRlZmluZWQgOiBuZXh0Vmlld1N0YXRlLmFjdGl2ZVV0aWxpdHlJZCB9O1xuICAgICAgICAgIGNvbnRpbnVlO1xuICAgICAgICB9XG4gICAgICAgIGlmIChcInBhdGNoV29ybGQzZENocm9tZVwiIGluIGVmZmVjdCkge1xuICAgICAgICAgIGNvbnN0IHsgc2VsZWN0aW9uSnNvbiwgdm9ydGljZXNKc29uLCBkb2N1bWVudFNlbGVjdGVkSWRzLCBkb2N1bWVudEhpZ2hsaWdodGVkSWRzIH0gPSBlZmZlY3QucGF0Y2hXb3JsZDNkQ2hyb21lO1xuICAgICAgICAgIGNvbnN0IHBhdGNoID0geyBzZWxlY3Rpb25Kc29uLCB2b3J0aWNlc0pzb24gfTtcbiAgICAgICAgICBjb25zdCB3aW5kb3dJbnN0YW5jZXMgPSBzZXNzaW9uV2luZG93SW5zdGFuY2VzKGJhc2VTZXNzaW9uLmFwcCwgZXh0cmFXaW5kb3dJbnN0YW5jZXNSZWYuY3VycmVudCk7XG4gICAgICAgICAgY29uc3QgZG9jdW1lbnRQYW5lbEtleSA9IHBhbmVsVGFiS2luZElkKEZSQU1FV09SS19QQU5FTF9UQUJfRE9DVU1FTlRfSUQpO1xuICAgICAgICAgIGRpc3BhdGNoKHtcbiAgICAgICAgICAgIHR5cGU6IFwiU0VUX1dJTkRPV19VSV9CWV9XSU5ET1dfSURcIixcbiAgICAgICAgICAgIHZhbHVlOiAoY3VycmVudCkgPT5cbiAgICAgICAgICAgICAgbWVyZ2VSZWNvcmRQcmVzZXJ2aW5nSWRlbnRpdHkoXG4gICAgICAgICAgICAgICAgY3VycmVudCxcbiAgICAgICAgICAgICAgICB3aW5kb3dJbnN0YW5jZXMubWFwKChpbnN0YW5jZSkgPT4ge1xuICAgICAgICAgICAgICAgICAgY29uc3Qgbm9kZSA9IGN1cnJlbnRbaW5zdGFuY2UuaWRdO1xuICAgICAgICAgICAgICAgICAgcmV0dXJuIFtpbnN0YW5jZS5pZCwgbm9kZSA/IHBhdGNoV29ybGQzZENocm9tZU9udG9Ob2RlKG5vZGUsIHBhdGNoKSA6IG5vZGVdIGFzIGNvbnN0O1xuICAgICAgICAgICAgICAgIH0pLFxuICAgICAgICAgICAgICApLFxuICAgICAgICAgIH0pO1xuICAgICAgICAgIGRpc3BhdGNoKHtcbiAgICAgICAgICAgIHR5cGU6IFwiU0VUX1BBTkVMX1VJX0JZX0tFWVwiLFxuICAgICAgICAgICAgdmFsdWU6IChjdXJyZW50KSA9PiB7XG4gICAgICAgICAgICAgIGNvbnN0IGRvY3VtZW50Tm9kZSA9IGN1cnJlbnRbZG9jdW1lbnRQYW5lbEtleV07XG4gICAgICAgICAgICAgIGlmICghZG9jdW1lbnROb2RlKSByZXR1cm4gY3VycmVudDtcbiAgICAgICAgICAgICAgcmV0dXJuIG1lcmdlUmVjb3JkUHJlc2VydmluZ0lkZW50aXR5KGN1cnJlbnQsIFtbZG9jdW1lbnRQYW5lbEtleSwgcGF0Y2hEb2N1bWVudFRyZWVTZWxlY3RlZElkcyhkb2N1bWVudE5vZGUsIGRvY3VtZW50U2VsZWN0ZWRJZHMsIGRvY3VtZW50SGlnaGxpZ2h0ZWRJZHMpXV0pO1xuICAgICAgICAgICAgfSxcbiAgICAgICAgICB9KTtcbiAgICAgICAgICBjb25zdCBjYWNoZSA9IHVpUmVmcmVzaENhY2hlUmVmLmN1cnJlbnQ7XG4gICAgICAgICAgZm9yIChjb25zdCBpbnN0YW5jZSBvZiB3aW5kb3dJbnN0YW5jZXMpIHtcbiAgICAgICAgICAgIGNvbnN0IGNhY2hlZCA9IGNhY2hlLmdldChgd2luZG93OiR7aW5zdGFuY2UuaWR9YCk7XG4gICAgICAgICAgICBpZiAoY2FjaGVkPy52YWx1ZSkge1xuICAgICAgICAgICAgICBjYWNoZS5zZXQoYHdpbmRvdzoke2luc3RhbmNlLmlkfWAsIHsgaGFzaDogY2FjaGVkLmhhc2gsIHZhbHVlOiBwYXRjaFdvcmxkM2RDaHJvbWVPbnRvTm9kZShjYWNoZWQudmFsdWUgYXMgVWlOb2RlLCBwYXRjaCkgfSk7XG4gICAgICAgICAgICB9XG4gICAgICAgICAgfVxuICAgICAgICAgIGNvbnN0IGRvY3VtZW50Q2FjaGVkID0gY2FjaGUuZ2V0KGBwYW5lbDoke2RvY3VtZW50UGFuZWxLZXl9YCk7XG4gICAgICAgICAgaWYgKGRvY3VtZW50Q2FjaGVkPy52YWx1ZSkge1xuICAgICAgICAgICAgY2FjaGUuc2V0KGBwYW5lbDoke2RvY3VtZW50UGFuZWxLZXl9YCwge1xuICAgICAgICAgICAgICBoYXNoOiBkb2N1bWVudENhY2hlZC5oYXNoLFxuICAgICAgICAgICAgICB2YWx1ZTogcGF0Y2hEb2N1bWVudFRyZWVTZWxlY3RlZElkcyhkb2N1bWVudENhY2hlZC52YWx1ZSBhcyBVaU5vZGUsIGRvY3VtZW50U2VsZWN0ZWRJZHMsIGRvY3VtZW50SGlnaGxpZ2h0ZWRJZHMpLFxuICAgICAgICAgICAgfSk7XG4gICAgICAgICAgfVxuICAgICAgICAgIGNvbnRpbnVlO1xuICAgICAgICB9XG4gICAgICAgIGlmIChcIm9wZW5EaWFsb2dcIiBpbiBlZmZlY3QpIHtcbiAgICAgICAgICAvLyDwn5eo77iPIFJlbmRlcnMgZnJvbSB0aGUgYWN0aXZlIGBiYXNlU2Vzc2lvbi5hcHBgIOKAlCBkaWFsb2dzIG9wZW5lZCBieSBzcGF3bmVkIHByb2dyYW1cbiAgICAgICAgICAvLyBpbnN0YW5jZXMgYXJlIHYxLW91dC1vZi1zY29wZSwgbWlycm9yaW5nIHRoZSBpbnRyb2R1Y3Rpb24ncyBhY3RpdmUtc2Vzc2lvbi1vbmx5IHNjb3BlLlxuICAgICAgICAgIGNvbnN0IHsgZGlhbG9nSWQsIGFyZ3MgfSA9IGVmZmVjdC5vcGVuRGlhbG9nO1xuICAgICAgICAgIGlmIChiYXNlU2Vzc2lvbi5hcHAuZGlhbG9ncz8uc29tZSgoZW50cnkpID0+IGVudHJ5LmlkID09PSBkaWFsb2dJZCkpIHtcbiAgICAgICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfRElBTE9HXCIsIHZhbHVlOiB7IGRpYWxvZ0lkLCBzZWVkQXJnczogYXJncyBhcyBSZWNvcmQ8c3RyaW5nLCB1bmtub3duPiB8IHVuZGVmaW5lZCB9IH0pO1xuICAgICAgICAgIH0gZWxzZSB7XG4gICAgICAgICAgICBjb25zb2xlLmVycm9yKGBbb3Mtc2hlbGxdIG9wZW5EaWFsb2c6IGFwcCAke2Jhc2VTZXNzaW9uLmFwcC5pZH0gZGVjbGFyZXMgbm8gZGlhbG9nIFwiJHtkaWFsb2dJZH1cImApO1xuICAgICAgICAgIH1cbiAgICAgICAgICBjb250aW51ZTtcbiAgICAgICAgfVxuICAgICAgICBpZiAoXCJuYXZpZ2F0ZVwiIGluIGVmZmVjdCkge1xuICAgICAgICAgIG5hdmlnYXRlSGlzdG9yeShlZmZlY3QubmF2aWdhdGUudXJpKTtcbiAgICAgICAgICBjb250aW51ZTtcbiAgICAgICAgfVxuICAgICAgICBpZiAoXCJsb2FkRG9jdW1lbnRcIiBpbiBlZmZlY3QpIHtcbiAgICAgICAgICBjb25zdCBwbHVnaW5FbnRyeSA9IGxvYWRlZFBsdWdpbnMuZmluZCgoZW50cnkpID0+IGVudHJ5LmhhbmRsZS5wbHVnaW5JZCA9PT0gYmFzZVNlc3Npb24ucGx1Z2luSWQpO1xuICAgICAgICAgIGNvbnN0IHBheWxvYWQgPSBlZmZlY3QubG9hZERvY3VtZW50O1xuICAgICAgICAgIGlmIChwYXlsb2FkLnBhY2sgJiYgcGF5bG9hZC5zcHIgJiYgcGx1Z2luRW50cnk/LmhhbmRsZS5sb2FkQXBwRG9jdW1lbnRQYWNrKSB7XG4gICAgICAgICAgICBjb25zdCBwYWNrQnl0ZXMgPSBjb2VyY2VXaXJlQnl0ZXMocGF5bG9hZC5wYWNrKTtcbiAgICAgICAgICAgIGNvbnN0IHNwckJ5dGVzID0gY29lcmNlV2lyZUJ5dGVzKHBheWxvYWQuc3ByKTtcbiAgICAgICAgICAgIGNvbnNvbGUubG9nKFwiW0RFQlVHXSBsb2FkRG9jdW1lbnQgcGFjay9zcHIgZm9yIGluc3RhbmNlXCIsIGJhc2VTZXNzaW9uLmluc3RhbmNlSWQsIFwicGFja1wiLCBwYWNrQnl0ZXMubGVuZ3RoLCBcInNwclwiLCBzcHJCeXRlcy5sZW5ndGgpO1xuICAgICAgICAgICAgYXdhaXQgcGx1Z2luRW50cnkuaGFuZGxlLmxvYWRBcHBEb2N1bWVudFBhY2soYmFzZVNlc3Npb24uaW5zdGFuY2VJZCwgcGFja0J5dGVzLCBzcHJCeXRlcyk7XG4gICAgICAgICAgfSBlbHNlIGlmIChwYXlsb2FkLmRvY3VtZW50SnNvbiAmJiBwbHVnaW5FbnRyeT8uaGFuZGxlLmxvYWRBcHBEb2N1bWVudCkge1xuICAgICAgICAgICAgY29uc29sZS5sb2coXCJbREVCVUddIGxvYWREb2N1bWVudCBmb3IgaW5zdGFuY2VcIiwgYmFzZVNlc3Npb24uaW5zdGFuY2VJZCwgXCJieXRlc1wiLCBwYXlsb2FkLmRvY3VtZW50SnNvbi5sZW5ndGgpO1xuICAgICAgICAgICAgYXdhaXQgcGx1Z2luRW50cnkuaGFuZGxlLmxvYWRBcHBEb2N1bWVudChiYXNlU2Vzc2lvbi5pbnN0YW5jZUlkLCBwYXlsb2FkLmRvY3VtZW50SnNvbik7XG4gICAgICAgICAgfSBlbHNlIHtcbiAgICAgICAgICAgIGNvbnNvbGUuZXJyb3IoXCJbb3Mtc2hlbGxdIGxvYWREb2N1bWVudDogcHJvZ3JhbSBoYXMgbm8gcGFjay9qc29uIGxvYWRlclwiLCBiYXNlU2Vzc2lvbi5wbHVnaW5JZCwgT2JqZWN0LmtleXMocGF5bG9hZCkpO1xuICAgICAgICAgIH1cbiAgICAgICAgICBjb250aW51ZTtcbiAgICAgICAgfVxuICAgICAgICBpZiAoXCJvcGVuRXh0ZXJuYWxVcmxcIiBpbiBlZmZlY3QpIHtcbiAgICAgICAgICB3aW5kb3cub3BlbihlZmZlY3Qub3BlbkV4dGVybmFsVXJsLnVybCwgXCJfYmxhbmtcIiwgXCJub29wZW5lcixub3JlZmVycmVyXCIpO1xuICAgICAgICAgIGNvbnRpbnVlO1xuICAgICAgICB9XG4gICAgICAgIGlmIChcImRvd25sb2FkTWVkaWFFeHBvcnRcIiBpbiBlZmZlY3QpIHtcbiAgICAgICAgICBjb25zdCB7IGZpbGVuYW1lLCBtaW1lVHlwZSwgZGF0YSwgZW5jb2RpbmcgfSA9IGVmZmVjdC5kb3dubG9hZE1lZGlhRXhwb3J0O1xuICAgICAgICAgIGRvd25sb2FkTWVkaWFFeHBvcnQoZmlsZW5hbWUsIG1pbWVUeXBlLCBkYXRhLCBlbmNvZGluZyk7XG4gICAgICAgICAgY29udGludWU7XG4gICAgICAgIH1cbiAgICAgICAgaWYgKFwiaWNvblJlbmRlckV4cG9ydFwiIGluIGVmZmVjdCkge1xuICAgICAgICAgIGZvciAoY29uc3QgaXRlbSBvZiBlZmZlY3QuaWNvblJlbmRlckV4cG9ydC5pdGVtcykge1xuICAgICAgICAgICAgdHJ5IHtcbiAgICAgICAgICAgICAgY29uc3QgcmVzdWx0ID0gYXdhaXQgaWNvblJlbmRlclBvcnQucmVuZGVyKGl0ZW0ucmVxdWVzdCBhcyBQYXJhbWV0ZXJzPHR5cGVvZiBpY29uUmVuZGVyUG9ydC5yZW5kZXI+WzBdKTtcbiAgICAgICAgICAgICAgZG93bmxvYWREYXRhVXJsKGl0ZW0uZmlsZW5hbWUsIHJlc3VsdC5kYXRhVXJsKTtcbiAgICAgICAgICAgIH0gY2F0Y2ggKGVycm9yKSB7XG4gICAgICAgICAgICAgIGNvbnNvbGUuZXJyb3IoYGljb24gcmVuZGVyIGV4cG9ydCBmYWlsZWQgZm9yICR7aXRlbS5maWxlbmFtZX1gLCBlcnJvcik7XG4gICAgICAgICAgICB9XG4gICAgICAgICAgfVxuICAgICAgICAgIGNvbnRpbnVlO1xuICAgICAgICB9XG4gICAgICAgIGlmIChcInJlcXVlc3RGaWxlT3BlblwiIGluIGVmZmVjdCkge1xuICAgICAgICAgIGNvbnN0IHsgYWNjZXB0LCByZWFkQXMsIGltcG9ydEFjdGlvbiwgbXVsdGlwbGUgfSA9IGVmZmVjdC5yZXF1ZXN0RmlsZU9wZW47XG4gICAgICAgICAgY29uc3Qgb3BlbmVkID0gYXdhaXQgcmVxdWVzdEZpbGVPcGVuKGFjY2VwdCB8fCBcIi5zcGssLmRzbCwub3BzLGFwcGxpY2F0aW9uL29jdGV0LXN0cmVhbVwiLCByZWFkQXMsIG11bHRpcGxlKTtcbiAgICAgICAgICBpZiAob3BlbmVkLmxlbmd0aCA+IDApIHtcbiAgICAgICAgICAgIGNvbnN0IHBsdWdpbkVudHJ5ID0gbG9hZGVkUGx1Z2lucy5maW5kKChlbnRyeSkgPT4gZW50cnkuaGFuZGxlLnBsdWdpbklkID09PSBiYXNlU2Vzc2lvbi5wbHVnaW5JZCk7XG4gICAgICAgICAgICBpZiAocGx1Z2luRW50cnkpIHtcbiAgICAgICAgICAgICAgLy8g8J+TpO+4jyBTaW5nbGUtZmlsZSAobXVsdGlwbGUgYWJzZW50L2ZhbHNlKTogaWRlbnRpY2FsIHRvIHRoZSBwcmUtbXVsdGktc2VsZWN0IHNoYXBlLCBvbmVcbiAgICAgICAgICAgICAgLy8gYGhhbmRsZUFjdGlvbmAgY2FsbCB3aXRoIGB7cGF5bG9hZCwgbmFtZX1gLiBNdWx0aS1maWxlOiBvbmUgc2VxdWVudGlhbCBjYWxsIHBlciBzZWxlY3RlZFxuICAgICAgICAgICAgICAvLyBmaWxlLCBlYWNoIGV4dGVuZGluZyBhcmdzIHdpdGggYHtpbmRleCwgdG90YWx9YCBzbyB0aGUgcGx1Z2luIGNhbiBzdGFnZS9tZXJnZSBpbXBvcnRzLlxuICAgICAgICAgICAgICBhd2FpdCBkaXNwYXRjaE9wZW5lZEZpbGVzKG9wZW5lZCwgaW1wb3J0QWN0aW9uLCBCb29sZWFuKG11bHRpcGxlKSwgbWFrZUVmZmVjdERpc3BhdGNoT25lKHBsdWdpbkVudHJ5LCBiYXNlU2Vzc2lvbiwgYXBwbHlIb3N0RWZmZWN0cykpO1xuICAgICAgICAgICAgfVxuICAgICAgICAgIH1cbiAgICAgICAgICBjb250aW51ZTtcbiAgICAgICAgfVxuICAgICAgICBpZiAoXCJkaXNwYXRjaEFjdGlvblwiIGluIGVmZmVjdCkge1xuICAgICAgICAgIC8vIPCflIHvuI8gU2VsZiByZS1kaXNwYXRjaCAoRDIpOiByZS1pbnZva2VzIHRoZSBzYW1lIHBsdWdpbiBpbnN0YW5jZSB3aXRoIGBhY3Rpb25gIGFmdGVyIGBkZWxheU1zYCxcbiAgICAgICAgICAvLyB3aXRob3V0IGJsb2NraW5nIHRoZSBjdXJyZW50IGBhcHBseUhvc3RFZmZlY3RzYCBwYXNzIOKAlCBgc2V0VGltZW91dGAgKDAgaXMgXCJuZXh0IHRpY2tcIikgZmlyZXNcbiAgICAgICAgICAvLyB0aGUgZm9sbG93LXVwIGNhbGwgYW5kIGZlZWRzIGl0cyBvd24gYHJlcXVlc3RlZEVmZmVjdHNgIGJhY2sgdGhyb3VnaCBgYXBwbHlIb3N0RWZmZWN0c2BcbiAgICAgICAgICAvLyByZWN1cnNpdmVseSwgc28gYSBwbHVnaW4gY2FuIGNoYWluIHNldmVyYWwgdGlja3Mgb2Ygc3RhZ2VkL3Byb2dyZXNzaXZlIHdvcmsgKGUuZy4gYVxuICAgICAgICAgIC8vIG11bHRpLXBhc3MgcmVjb25zdHJ1Y3Rpb24pIHB1cmVseSBieSByZS1lbWl0dGluZyBgZGlzcGF0Y2hBY3Rpb25gIGZyb20gaXRzIG93biBoYW5kbGVyLlxuICAgICAgICAgIGNvbnN0IHsgYWN0aW9uOiBkaXNwYXRjaEFjdGlvbklkLCBhcmdzOiBkaXNwYXRjaEFyZ3MsIGRlbGF5TXMgfSA9IGVmZmVjdC5kaXNwYXRjaEFjdGlvbjtcbiAgICAgICAgICBjb25zdCBwbHVnaW5FbnRyeSA9IGxvYWRlZFBsdWdpbnMuZmluZCgoZW50cnkpID0+IGVudHJ5LmhhbmRsZS5wbHVnaW5JZCA9PT0gYmFzZVNlc3Npb24ucGx1Z2luSWQpO1xuICAgICAgICAgIGlmIChwbHVnaW5FbnRyeSkge1xuICAgICAgICAgICAgc2NoZWR1bGVEaXNwYXRjaEFjdGlvbihkaXNwYXRjaEFjdGlvbklkLCBkaXNwYXRjaEFyZ3MgYXMgUmVjb3JkPHN0cmluZywgdW5rbm93bj4gfCB1bmRlZmluZWQsIGRlbGF5TXMsIG1ha2VFZmZlY3REaXNwYXRjaE9uZShwbHVnaW5FbnRyeSwgYmFzZVNlc3Npb24sIGFwcGx5SG9zdEVmZmVjdHMpKTtcbiAgICAgICAgICB9XG4gICAgICAgICAgY29udGludWU7XG4gICAgICAgIH1cbiAgICAgICAgaWYgKFwicmVxdWVzdE1lZGlhRnJhbWVzXCIgaW4gZWZmZWN0KSB7XG4gICAgICAgICAgLy8g8J+Onu+4jyBENTogZGVjb2RlcyBhIHZpZGVvIChmaWxlIHBpY2tlciwgb3IgYHBheWxvYWRgIGJ5dGVzIGFscmVhZHkgaW4gaGFuZCBmcm9tIGEgZHJvcCB6b25lKVxuICAgICAgICAgIC8vIGFuZCBmYW5zIHNhbXBsZWQgZnJhbWVzICsgYSBjb21wbGV0aW9uIG1hcmtlciBvdXQgdGhyb3VnaCB0aGUgc2FtZSBgZGlzcGF0Y2hPbmVgIHBhdGggYXNcbiAgICAgICAgICAvLyBldmVyeSBvdGhlciBlZmZlY3QgYnJhbmNoIOKAlCBzZWUgYHJ1blJlcXVlc3RNZWRpYUZyYW1lc2AgZm9yIHRoZSBUaWVyIDEgKFdlYkNvZGVjcykvVGllciAyXG4gICAgICAgICAgLy8gKGA8dmlkZW8+YCBzZWVrLWFuZC1jYXB0dXJlKS9mYWxsYmFjayBkZWNpc2lvbiB0cmVlLlxuICAgICAgICAgIGNvbnN0IHsgYWNjZXB0LCBwYXlsb2FkLCBmcmFtZUFjdGlvbiwgZG9uZUFjdGlvbiwgZmFsbGJhY2tBY3Rpb24sIHNhbXBsZVN0cmlkZSwgbWF4RnJhbWVzLCBtYXhMb25nRWRnZVB4LCBmcHNIaW50LCBhcmdzIH0gPSBlZmZlY3QucmVxdWVzdE1lZGlhRnJhbWVzO1xuICAgICAgICAgIGNvbnN0IHBsdWdpbkVudHJ5ID0gbG9hZGVkUGx1Z2lucy5maW5kKChlbnRyeSkgPT4gZW50cnkuaGFuZGxlLnBsdWdpbklkID09PSBiYXNlU2Vzc2lvbi5wbHVnaW5JZCk7XG4gICAgICAgICAgaWYgKHBsdWdpbkVudHJ5KSB7XG4gICAgICAgICAgICBhd2FpdCBydW5SZXF1ZXN0TWVkaWFGcmFtZXMoXG4gICAgICAgICAgICAgIHtcbiAgICAgICAgICAgICAgICBmcmFtZUFjdGlvbixcbiAgICAgICAgICAgICAgICBkb25lQWN0aW9uLFxuICAgICAgICAgICAgICAgIGZhbGxiYWNrQWN0aW9uLFxuICAgICAgICAgICAgICAgIHNhbXBsZVN0cmlkZTogc2FtcGxlU3RyaWRlID8/IDAsXG4gICAgICAgICAgICAgICAgbWF4RnJhbWVzOiBtYXhGcmFtZXMgPz8gMCxcbiAgICAgICAgICAgICAgICBtYXhMb25nRWRnZVB4OiBtYXhMb25nRWRnZVB4ID8/IDAsXG4gICAgICAgICAgICAgICAgZnBzSGludDogZnBzSGludCA/PyAwLFxuICAgICAgICAgICAgICAgIGFyZ3M6IGFyZ3MgYXMgUmVjb3JkPHN0cmluZywgdW5rbm93bj4gfCB1bmRlZmluZWQsXG4gICAgICAgICAgICAgIH0sXG4gICAgICAgICAgICAgIGFjY2VwdCxcbiAgICAgICAgICAgICAgcGF5bG9hZCxcbiAgICAgICAgICAgICAgbWFrZUVmZmVjdERpc3BhdGNoT25lKHBsdWdpbkVudHJ5LCBiYXNlU2Vzc2lvbiwgYXBwbHlIb3N0RWZmZWN0cyksXG4gICAgICAgICAgICApO1xuICAgICAgICAgIH1cbiAgICAgICAgICBjb250aW51ZTtcbiAgICAgICAgfVxuICAgICAgICBpZiAoXCJyZXF1ZXN0UGx1Z2luRXhjaGFuZ2VcIiBpbiBlZmZlY3QpIHtcbiAgICAgICAgICBjb25zdCB7IHBsdWdpbklkLCBhcHBJZCwgcmVxdWVzdEpzb24sIHJlc3BvbnNlQWN0aW9uIH0gPSBlZmZlY3QucmVxdWVzdFBsdWdpbkV4Y2hhbmdlO1xuICAgICAgICAgIGNvbnN0IHJlcXVlc3QgPSBKU09OLnBhcnNlKHJlcXVlc3RKc29uKSBhcyB7IG9wZXJhdG9ySWQ/OiBzdHJpbmc7IGlucHV0SnNvbj86IHN0cmluZzsgbm9kZUhhc2g/OiBudW1iZXIgfTtcbiAgICAgICAgICBjb25zdCBjb250cmlidXRvciA9IGxvYWRlZFBsdWdpbnMuZmluZCgoZW50cnkpID0+IGVudHJ5LmhhbmRsZS5wbHVnaW5JZCA9PT0gcGx1Z2luSWQpO1xuICAgICAgICAgIGlmIChjb250cmlidXRvciAmJiByZXF1ZXN0Lm9wZXJhdG9ySWQgJiYgcmVxdWVzdC5pbnB1dEpzb24gIT0gbnVsbCAmJiByZXF1ZXN0Lm5vZGVIYXNoICE9IG51bGwpIHtcbiAgICAgICAgICAgIHRyeSB7XG4gICAgICAgICAgICAgIGNvbnN0IGJpbSA9IChhd2FpdCBpbXBvcnQoXCJAc2VtaW8tdGVjaC9mbG93LW1vZHVsZS1iaW1cIikpIGFzIHsgZXZhbHVhdGU/OiAoa2luZElkOiBzdHJpbmcsIGlucHV0SnNvbjogc3RyaW5nKSA9PiBzdHJpbmcgfTtcbiAgICAgICAgICAgICAgY29uc3Qgb3V0cHV0SnNvbiA9IHR5cGVvZiBiaW0uZXZhbHVhdGUgPT09IFwiZnVuY3Rpb25cIiA/IGJpbS5ldmFsdWF0ZShyZXF1ZXN0Lm9wZXJhdG9ySWQsIHJlcXVlc3QuaW5wdXRKc29uKSA6IFwiXCI7XG4gICAgICAgICAgICAgIGNvbnNvbGUubG9nKFwiW0RFQlVHXSByZXF1ZXN0UGx1Z2luRXhjaGFuZ2UgcmVzb2x2ZWQgZXh0ZW5zaW9uIGV2YWxcIiwgeyBwbHVnaW5JZCwgYXBwSWQsIG9wZXJhdG9ySWQ6IHJlcXVlc3Qub3BlcmF0b3JJZCwgbm9kZUhhc2g6IHJlcXVlc3Qubm9kZUhhc2ggfSk7XG4gICAgICAgICAgICAgIGF3YWl0IG1ha2VFZmZlY3REaXNwYXRjaE9uZShwbHVnaW5FbnRyeSwgYmFzZVNlc3Npb24sIGFwcGx5SG9zdEVmZmVjdHMpKHJlc3BvbnNlQWN0aW9uLCB7XG4gICAgICAgICAgICAgICAgbm9kZUhhc2g6IHJlcXVlc3Qubm9kZUhhc2gsXG4gICAgICAgICAgICAgICAgb3V0cHV0SnNvbixcbiAgICAgICAgICAgICAgfSk7XG4gICAgICAgICAgICB9IGNhdGNoIChlcnJvcikge1xuICAgICAgICAgICAgICBjb25zb2xlLndhcm4oXCJbb3Mtc2hlbGxdIHJlcXVlc3RQbHVnaW5FeGNoYW5nZSBmYWlsZWRcIiwgeyBwbHVnaW5JZCwgYXBwSWQsIGVycm9yIH0pO1xuICAgICAgICAgICAgfVxuICAgICAgICAgIH1cbiAgICAgICAgICBjb250aW51ZTtcbiAgICAgICAgfVxuICAgICAgICBpZiAoXCJzcGF3blBsdWdpbkluc3RhbmNlXCIgaW4gZWZmZWN0KSB7XG4gICAgICAgICAgY29uc3QgeyBwbHVnaW5JZCwgYXBwSWQsIG9zSW5zdGFuY2VJZCwgbGFiZWwsIGRvY3VtZW50SnNvbiB9ID0gZWZmZWN0LnNwYXduUGx1Z2luSW5zdGFuY2U7XG4gICAgICAgICAgY29uc3QgY3VycmVudFBhbmVsID0gcGFyc2VQYW5lbFN0YXRlKG5leHRWaWV3U3RhdGUpID8/IGJ1aWxkU3BhY2VQYW5lbFN0YXRlKFtdLCBbXSk7XG4gICAgICAgICAgLy8g8J+qpu+4jyBTZWUgYGVzdGFibGlzaFByaW1hcnlTZXNzaW9uYCdzIGNvbW1lbnQgYWJvdmUg4oCUIHRoZSBgbWFuaWZlc3Qud29ya2Zsb3dzYCBmYWxsYmFjayBzb3VyY2UgaXMgZGVhZDsgYGNhdGFsb2dgIGlzIGBjdXJyZW50UGFuZWwucHJvZ3JhbXNgIG9yIGVtcHR5LlxuICAgICAgICAgIGNvbnN0IGNhdGFsb2cgPSBjdXJyZW50UGFuZWwucHJvZ3JhbXMubGVuZ3RoID4gMCA/IGN1cnJlbnRQYW5lbC5wcm9ncmFtcyA6IFtdO1xuICAgICAgICAgIGNvbnN0IHByb2dyYW0gPSBjYXRhbG9nLmZpbmQoKGVudHJ5KSA9PiBlbnRyeS5wbHVnaW5JZCA9PT0gcGx1Z2luSWQgJiYgZW50cnkuYXBwSWQgPT09IGFwcElkKSA/PyBjYXRhbG9nLmZpbmQoKGVudHJ5KSA9PiBlbnRyeS5wbHVnaW5JZCA9PT0gcGx1Z2luSWQpO1xuICAgICAgICAgIGlmIChwcm9ncmFtKSB7XG4gICAgICAgICAgICAvLyDwn6qf77iPIEZvbGQgc3Bhd24gaW50byBgbmV4dFZpZXdTdGF0ZWAg4oCUIGEgc2VwYXJhdGUgU0VUX1NFU1NJT04gd291bGQgYmUgY2xvYmJlcmVkIGJ5IHRoZVxuICAgICAgICAgICAgLy8gZmluYWwgd3JpdGUgYmVsb3cgYW5kIGxlYXZlIHRoZSBzaGVsbCBzdHVjayBvbiB0aGUgc3R1ZGlvIHN1cmZhY2UuXG4gICAgICAgICAgICBjb25zdCBuZXh0UGFuZWwgPSBhd2FpdCBlbnN1cmVTcGF3bmVkUGx1Z2luKHByb2dyYW0sIGxhYmVsLCBvc0luc3RhbmNlSWQsIGRvY3VtZW50SnNvbiwgbmV4dFZpZXdTdGF0ZSk7XG4gICAgICAgICAgICBpZiAobmV4dFBhbmVsKSBuZXh0Vmlld1N0YXRlID0gdmlld1N0YXRlV2l0aFNwYWNlUGFuZWwobmV4dFZpZXdTdGF0ZSwgbmV4dFBhbmVsKTtcbiAgICAgICAgICB9XG4gICAgICAgICAgY29udGludWU7XG4gICAgICAgIH1cbiAgICAgICAgaWYgKFwib3BlblBsdWdpbkluc3RhbmNlXCIgaW4gZWZmZWN0KSB7XG4gICAgICAgICAgY29uc3QgeyBwbHVnaW5JZCwgYXBwSWQsIG9zSW5zdGFuY2VJZCB9ID0gZWZmZWN0Lm9wZW5QbHVnaW5JbnN0YW5jZTtcbiAgICAgICAgICBjb25zdCBjdXJyZW50UGFuZWwgPSBwYXJzZVBhbmVsU3RhdGUobmV4dFZpZXdTdGF0ZSkgPz8gYnVpbGRTcGFjZVBhbmVsU3RhdGUoW10sIFtdKTtcbiAgICAgICAgICAvLyDwn6qm77iPIFNlZSBgZXN0YWJsaXNoUHJpbWFyeVNlc3Npb25gJ3MgY29tbWVudCBhYm92ZSDigJQgdGhlIGBtYW5pZmVzdC53b3JrZmxvd3NgIGZhbGxiYWNrIHNvdXJjZSBpcyBkZWFkOyBgY2F0YWxvZ2AgaXMgYGN1cnJlbnRQYW5lbC5wcm9ncmFtc2Agb3IgZW1wdHkuXG4gICAgICAgICAgY29uc3QgY2F0YWxvZyA9IGN1cnJlbnRQYW5lbC5wcm9ncmFtcy5sZW5ndGggPiAwID8gY3VycmVudFBhbmVsLnByb2dyYW1zIDogW107XG4gICAgICAgICAgY29uc3QgcHJvZ3JhbSA9IGNhdGFsb2cuZmluZCgoZW50cnkpID0+IGVudHJ5LnBsdWdpbklkID09PSBwbHVnaW5JZCAmJiBlbnRyeS5hcHBJZCA9PT0gYXBwSWQpID8/IGNhdGFsb2cuZmluZCgoZW50cnkpID0+IGVudHJ5LnBsdWdpbklkID09PSBwbHVnaW5JZCk7XG4gICAgICAgICAgaWYgKHByb2dyYW0pIHtcbiAgICAgICAgICAgIC8vIPCfqp/vuI8gRm9sZCBmb2N1cyBpbnRvIGBuZXh0Vmlld1N0YXRlYCBzbyB0aGUgZmluYWwgU0VUX1NFU1NJT04ga2VlcHMgYGFjdGl2ZVNwYXduZWRJZGBcbiAgICAgICAgICAgIC8vIChvcGVuaW5nIGEgd29ya2Zsb3cgbm9kZSBkZXBlbmRzIG9uIHRoaXMg4oCUIG90aGVyd2lzZSBub3RoaW5nIGFwcGVhcnMgdG8gaGFwcGVuKS5cbiAgICAgICAgICAgIGNvbnN0IG5leHRQYW5lbCA9IGF3YWl0IGVuc3VyZVNwYXduZWRQbHVnaW4ocHJvZ3JhbSwgdW5kZWZpbmVkLCBvc0luc3RhbmNlSWQsIHVuZGVmaW5lZCwgbmV4dFZpZXdTdGF0ZSk7XG4gICAgICAgICAgICBpZiAobmV4dFBhbmVsKSB7XG4gICAgICAgICAgICAgIG5leHRWaWV3U3RhdGUgPSB2aWV3U3RhdGVXaXRoU3BhY2VQYW5lbChuZXh0Vmlld1N0YXRlLCBuZXh0UGFuZWwpO1xuICAgICAgICAgICAgICBjb25zb2xlLmxvZyhcIltERUJVR10gb3BlblBsdWdpbkluc3RhbmNlIGZvY3VzZWQgc3Bhd25lZCBhcHBcIiwge1xuICAgICAgICAgICAgICAgIHBsdWdpbklkLFxuICAgICAgICAgICAgICAgIGFwcElkLFxuICAgICAgICAgICAgICAgIG9zSW5zdGFuY2VJZCxcbiAgICAgICAgICAgICAgICBhY3RpdmVTcGF3bmVkSWQ6IG5leHRQYW5lbC5hY3RpdmVTcGF3bmVkSWQsXG4gICAgICAgICAgICAgICAgc3Bhd25lZENvdW50OiBuZXh0UGFuZWwuc3Bhd25lZEFwcHMubGVuZ3RoLFxuICAgICAgICAgICAgICB9KTtcbiAgICAgICAgICAgIH1cbiAgICAgICAgICAgIGlmIChvc0luc3RhbmNlSWQgJiYgb3BlblNwYWNlSWRSZWYuY3VycmVudCkge1xuICAgICAgICAgICAgICBvcGVuSW5zdGFuY2VJZFJlZi5jdXJyZW50ID0gb3NJbnN0YW5jZUlkO1xuICAgICAgICAgICAgICBuYXZpZ2F0ZUhpc3RvcnkoYC9zcGFjZXMvJHtvcGVuU3BhY2VJZFJlZi5jdXJyZW50fS9pbnN0YW5jZXMvJHtvc0luc3RhbmNlSWR9YCk7XG4gICAgICAgICAgICB9XG4gICAgICAgICAgfSBlbHNlIHtcbiAgICAgICAgICAgIGNvbnNvbGUud2FybihcbiAgICAgICAgICAgICAgXCJbb3Mtc2hlbGxdIG9wZW5QbHVnaW5JbnN0YW5jZTogbm8gcHJvZ3JhbSBtYXRjaGVzXCIsXG4gICAgICAgICAgICAgIHsgcGx1Z2luSWQsIGFwcElkIH0sXG4gICAgICAgICAgICAgIFwiYXZhaWxhYmxlOlwiLFxuICAgICAgICAgICAgICBjYXRhbG9nLm1hcCgoZW50cnkpID0+IGAke2VudHJ5LnBsdWdpbklkfS8ke2VudHJ5LmFwcElkfWApLFxuICAgICAgICAgICAgKTtcbiAgICAgICAgICB9XG4gICAgICAgICAgY29udGludWU7XG4gICAgICAgIH1cbiAgICAgIH1cbiAgICAgIGNvbnN0IG5leHRTZXNzaW9uID0geyAuLi5iYXNlU2Vzc2lvbiwgdmlld1N0YXRlOiBuZXh0Vmlld1N0YXRlIH07XG4gICAgICBjb25zdCBpc1NwYXduZWRQbHVnaW5TZXNzaW9uID0gc3R1ZGlvTW9kZSAmJiBzZXNzaW9uICYmIGJhc2VTZXNzaW9uLnBsdWdpbklkICE9PSBzZXNzaW9uLnBsdWdpbklkO1xuICAgICAgZGlzcGF0Y2goe1xuICAgICAgICB0eXBlOiBcIlNFVF9TRVNTSU9OXCIsXG4gICAgICAgIHZhbHVlOiAoY3VycmVudCkgPT4ge1xuICAgICAgICAgIGlmICghY3VycmVudCkgcmV0dXJuIG5leHRTZXNzaW9uO1xuICAgICAgICAgIGlmIChpc1NwYXduZWRQbHVnaW5TZXNzaW9uKSByZXR1cm4gY3VycmVudC52aWV3U3RhdGUgPT09IG5leHRWaWV3U3RhdGUgPyBjdXJyZW50IDogeyAuLi5jdXJyZW50LCB2aWV3U3RhdGU6IG5leHRWaWV3U3RhdGUgfTtcbiAgICAgICAgICBpZiAoY3VycmVudC5pbnN0YW5jZUlkICE9PSBuZXh0U2Vzc2lvbi5pbnN0YW5jZUlkKSByZXR1cm4gY3VycmVudDtcbiAgICAgICAgICAvLyDwn5Ci77iPIFByZXNlcnZlIGBjdXJyZW50YCdzIGlkZW50aXR5IHdoZW4gdGhlIHZpZXdTdGF0ZSBkaWRuJ3QgYWN0dWFsbHkgY2hhbmdlIOKAlCBvdGhlcndpc2UgZXZlcnlcbiAgICAgICAgICAvLyBhY3Rpb24gbWludHMgYSBuZXcgYHNlc3Npb25gIG9iamVjdCwgd2hpY2ggY2FzY2FkZXMgaW50byBhIG5ldyBgb25BY3Rpb25gIGlkZW50aXR5LCB3aGljaFxuICAgICAgICAgIC8vIGJ1c3RzIGV2ZXJ5IG1lbW8ga2V5ZWQgb24gaXQgKHdpbmRvd3MsIHBhbmVscywgdGhlIGJvb3QtcmVmcmVzaCBlZmZlY3QgYmVsb3cpIGV2ZW4gd2hlblxuICAgICAgICAgIC8vIG5vdGhpbmcgYWJvdXQgdGhlIHNlc3Npb24gY2hhbmdlZC5cbiAgICAgICAgICByZXR1cm4gY3VycmVudC52aWV3U3RhdGUgPT09IG5leHRWaWV3U3RhdGUgPyBjdXJyZW50IDogeyAuLi5jdXJyZW50LCB2aWV3U3RhdGU6IG5leHRWaWV3U3RhdGUgfTtcbiAgICAgICAgfSxcbiAgICAgIH0pO1xuICAgICAgaWYgKGlzU3Bhd25lZFBsdWdpblNlc3Npb24pIHtcbiAgICAgICAgY29uc3Qgc3Bhd25lZCA9IHBhcnNlUGFuZWxTdGF0ZShuZXh0Vmlld1N0YXRlKT8uc3Bhd25lZEFwcHMuZmluZCgoZW50cnkpID0+IGVudHJ5LnBsdWdpbklkID09PSBiYXNlU2Vzc2lvbi5wbHVnaW5JZCAmJiBlbnRyeS5pbnN0YW5jZUlkID09PSBiYXNlU2Vzc2lvbi5pbnN0YW5jZUlkKTtcbiAgICAgICAgaWYgKHNwYXduZWQpIGF3YWl0IHJlZnJlc2hTcGF3bmVkVWkoc3Bhd25lZCwgbmV4dFZpZXdTdGF0ZSwgdWlTY29wZSk7XG4gICAgICB9IGVsc2UgaWYgKHNlc3Npb24/Lmluc3RhbmNlSWQgPT09IG5leHRTZXNzaW9uLmluc3RhbmNlSWQgfHwgYmFzZVNlc3Npb24uaW5zdGFuY2VJZCA9PT0gbmV4dFNlc3Npb24uaW5zdGFuY2VJZCkge1xuICAgICAgICBhd2FpdCByZWZyZXNoVWkobmV4dFNlc3Npb24sIHVpU2NvcGUpO1xuICAgICAgfVxuICAgIH0sXG4gICAgW2NsZWFyQWxsV2luZG93VXRpbGl0aWVzLCBlbnN1cmVTcGF3bmVkUGx1Z2luLCBsb2FkZWRQbHVnaW5zLCBuYXZpZ2F0ZUhpc3RvcnksIHJlZnJlc2hTcGF3bmVkVWksIHJlZnJlc2hVaSwgc2Vzc2lvbiwgc2V0QWN0aXZlVXRpbGl0eUZvcldpbmRvdywgc3R1ZGlvTW9kZV0sXG4gICk7XG5cbiAgY29uc3QgYXBwbHlTaGVsbFVyaSA9IHVzZUNhbGxiYWNrKFxuICAgIGFzeW5jICh1cmk6IHN0cmluZywgcHJlc2VydmVkVmlld1N0YXRlPzogVmlld01vZGVsKSA9PiB7XG4gICAgICBjb25zdCBjdXJyZW50U2Vzc2lvbiA9IHNlc3Npb25SZWYuY3VycmVudDtcbiAgICAgIGlmICghaG9zdENvbmZpZyB8fCAhY3VycmVudFNlc3Npb24gfHwgbG9hZGVkUGx1Z2lucy5sZW5ndGggPT09IDApIHJldHVybjtcbiAgICAgIGNvbnN0IHBhdGggPSB1cmkuc3BsaXQoXCI/XCIpWzBdID8/IFwiL1wiO1xuICAgICAgY29uc3Qgcm91dGUgPSBwYXJzZVNoZWxsUm91dGUocGF0aCk7XG4gICAgICBjb25zdCBzUGx1Z2luID0gbG9hZGVkUGx1Z2lucy5maW5kKChlbnRyeSkgPT4gZW50cnkuaGFuZGxlLnBsdWdpbklkID09PSBob3N0Q29uZmlnLnBsdWdpbklkKT8uaGFuZGxlO1xuICAgICAgaWYgKCFzUGx1Z2luKSByZXR1cm47XG4gICAgICBpZiAocm91dGUua2luZCA9PT0gXCJsYW5kaW5nXCIpIHtcbiAgICAgICAgb3BlblNwYWNlSWRSZWYuY3VycmVudCA9IG51bGw7XG4gICAgICAgIG9wZW5JbnN0YW5jZUlkUmVmLmN1cnJlbnQgPSBudWxsO1xuICAgICAgICBpZiAoY3VycmVudFNlc3Npb24uYXBwLmlkICE9PSBob3N0Q29uZmlnLmxhbmRpbmdBcHBJZCkgYXdhaXQgc3dpdGNoVG9NYW5hZ2VkQXBwKGhvc3RDb25maWcubGFuZGluZ0FwcElkLCBwcmVzZXJ2ZWRWaWV3U3RhdGUpO1xuICAgICAgICByZXR1cm47XG4gICAgICB9XG4gICAgICBpZiAocm91dGUua2luZCA9PT0gXCJub3RGb3VuZFwiKSB7XG4gICAgICAgIG9wZW5TcGFjZUlkUmVmLmN1cnJlbnQgPSBudWxsO1xuICAgICAgICBvcGVuSW5zdGFuY2VJZFJlZi5jdXJyZW50ID0gbnVsbDtcbiAgICAgICAgcmV0dXJuO1xuICAgICAgfVxuICAgICAgY29uc3QgeyBzcGFjZUlkLCBpbnN0YW5jZUlkIH0gPSByb3V0ZTtcbiAgICAgIC8vIPCfp63vuI8gUGluIHRoZSByb3V0ZSBzdHVkaW8gaWQgYmVmb3JlIHRoZSBhc3luYyBhcHAgc3dpdGNoIHNvIHRoZSBib290IGV4YW1wbGUgZWZmZWN0IGNhbm5vdFxuICAgICAgLy8gcmFjZS1uYXZpZ2F0ZSB0byBgL3NwYWNlcy9kZW1vYCB3aGlsZSBgc3dpdGNoVG9NYW5hZ2VkQXBwYCBpcyBzdGlsbCBhd2FpdGluZy5cbiAgICAgIGNvbnN0IHN0dWRpb0NoYW5nZWQgPSBvcGVuU3BhY2VJZFJlZi5jdXJyZW50ICE9PSBzcGFjZUlkO1xuICAgICAgb3BlblNwYWNlSWRSZWYuY3VycmVudCA9IHNwYWNlSWQ7XG4gICAgICBjb25zdCBzdHVkaW9TZXNzaW9uID0gY3VycmVudFNlc3Npb24uYXBwLmlkID09PSBob3N0Q29uZmlnLmhvc3RBcHBJZCA/IGN1cnJlbnRTZXNzaW9uIDogYXdhaXQgc3dpdGNoVG9NYW5hZ2VkQXBwKGhvc3RDb25maWcuaG9zdEFwcElkLCBwcmVzZXJ2ZWRWaWV3U3RhdGUpO1xuICAgICAgaWYgKCFzdHVkaW9TZXNzaW9uKSByZXR1cm47XG4gICAgICBjb25zdCBzdHVkaW9Db250cm9sbGVySWQgPSBzdHVkaW9TZXNzaW9uLmFwcC5jb250cm9sbGVySWQ7XG4gICAgICBpZiAoc3R1ZGlvQ2hhbmdlZCkge1xuICAgICAgICBvcGVuSW5zdGFuY2VJZFJlZi5jdXJyZW50ID0gbnVsbDtcbiAgICAgICAgY29uc29sZS5sb2coXCJbREVCVUddIGFwcGx5U2hlbGxVcmkgb3BlblNwYWNlXCIsIHNwYWNlSWQpO1xuICAgICAgICBjb25zdCBvcGVuUmVzcG9uc2UgPSBhd2FpdCBzUGx1Z2luLmhhbmRsZUFjdGlvbihzdHVkaW9TZXNzaW9uLmluc3RhbmNlSWQsIGVuY29kZUFjdGlvbldpcmUoeyBjb250cm9sbGVySWQ6IHN0dWRpb0NvbnRyb2xsZXJJZCwgYWN0aW9uOiBcIm9wZW5TcGFjZVwiLCBhcmdzOiB7IHNwYWNlSWQgfSB9KSwgc3R1ZGlvU2Vzc2lvbi52aWV3U3RhdGUpO1xuICAgICAgICBhd2FpdCBhcHBseUhvc3RFZmZlY3RzKG9wZW5SZXNwb25zZS5yZXF1ZXN0ZWRFZmZlY3RzID8/IFtdLCBzdHVkaW9TZXNzaW9uLCByZXNvbHZlVWlEaXJ0eVNjb3BlKG9wZW5SZXNwb25zZS51aVNjb3BlKSk7XG4gICAgICB9XG4gICAgICBpZiAob3Blbkluc3RhbmNlSWRSZWYuY3VycmVudCA9PT0gKGluc3RhbmNlSWQgPz8gbnVsbCkpIHJldHVybjtcbiAgICAgIG9wZW5JbnN0YW5jZUlkUmVmLmN1cnJlbnQgPSBpbnN0YW5jZUlkID8/IG51bGw7XG4gICAgICBpZiAoaW5zdGFuY2VJZCkge1xuICAgICAgICBjb25zdCByZXNwb25zZSA9IGF3YWl0IHNQbHVnaW4uaGFuZGxlQWN0aW9uKHN0dWRpb1Nlc3Npb24uaW5zdGFuY2VJZCwgZW5jb2RlQWN0aW9uV2lyZSh7IGNvbnRyb2xsZXJJZDogc3R1ZGlvQ29udHJvbGxlcklkLCBhY3Rpb246IFwib3Blbkluc3RhbmNlXCIsIGFyZ3M6IHsgaW5zdGFuY2VJZCB9IH0pLCBzdHVkaW9TZXNzaW9uLnZpZXdTdGF0ZSk7XG4gICAgICAgIGF3YWl0IGFwcGx5SG9zdEVmZmVjdHMocmVzcG9uc2UucmVxdWVzdGVkRWZmZWN0cyA/PyBbXSwgc3R1ZGlvU2Vzc2lvbiwgcmVzb2x2ZVVpRGlydHlTY29wZShyZXNwb25zZS51aVNjb3BlKSk7XG4gICAgICB9IGVsc2Uge1xuICAgICAgICBjb25zdCByZXNwb25zZSA9IGF3YWl0IHNQbHVnaW4uaGFuZGxlQWN0aW9uKHN0dWRpb1Nlc3Npb24uaW5zdGFuY2VJZCwgZW5jb2RlQWN0aW9uV2lyZSh7IGNvbnRyb2xsZXJJZDogc3R1ZGlvQ29udHJvbGxlcklkLCBhY3Rpb246IFwiY2xvc2VGb2N1c2VkSW5zdGFuY2VcIiB9KSwgc3R1ZGlvU2Vzc2lvbi52aWV3U3RhdGUpO1xuICAgICAgICBjb25zdCBjdXJyZW50UGFuZWwgPSBwYXJzZVBhbmVsU3RhdGUoc3R1ZGlvU2Vzc2lvbi52aWV3U3RhdGUpID8/IGJ1aWxkU3BhY2VQYW5lbFN0YXRlKFtdLCBbXSk7XG4gICAgICAgIHVwZGF0ZVNwYWNlUGFuZWwoYnVpbGRTcGFjZVBhbmVsU3RhdGUoY3VycmVudFBhbmVsLnByb2dyYW1zLCBjdXJyZW50UGFuZWwuc3Bhd25lZEFwcHMsIGN1cnJlbnRQYW5lbC5hY3RpdmVQYW5lbFRhYiwgdW5kZWZpbmVkKSk7XG4gICAgICAgIGF3YWl0IGFwcGx5SG9zdEVmZmVjdHMocmVzcG9uc2UucmVxdWVzdGVkRWZmZWN0cyA/PyBbXSwgc3R1ZGlvU2Vzc2lvbiwgcmVzb2x2ZVVpRGlydHlTY29wZShyZXNwb25zZS51aVNjb3BlKSk7XG4gICAgICB9XG4gICAgfSxcbiAgICBbYXBwbHlIb3N0RWZmZWN0cywgbG9hZGVkUGx1Z2lucywgcmVmcmVzaFVpLCBob3N0Q29uZmlnLCBzd2l0Y2hUb01hbmFnZWRBcHAsIHVwZGF0ZVNwYWNlUGFuZWxdLFxuICApO1xuXG4gIHVzZUVmZmVjdCgoKSA9PiB7XG4gICAgaWYgKCFzdHVkaW9Nb2RlIHx8IGxvYWRlZFBsdWdpbnMubGVuZ3RoID09PSAwKSByZXR1cm47XG4gICAgdm9pZCBhcHBseVNoZWxsVXJpKHNoZWxsVXJpKS5jYXRjaCgodXJpRXJyb3IpID0+IHtcbiAgICAgIGNvbnNvbGUuZXJyb3IoXCJbREVCVUddIHNoZWxsIHVyaSBhcHBseSBmYWlsZWRcIiwgdXJpRXJyb3IpO1xuICAgIH0pO1xuICB9LCBbYXBwbHlTaGVsbFVyaSwgbG9hZGVkUGx1Z2lucy5sZW5ndGgsIHNoZWxsVXJpLCBzdHVkaW9Nb2RlXSk7XG5cbiAgY29uc3QgcmVzb2x2ZVN5bmNUYXJnZXRTZXNzaW9uID0gdXNlQ2FsbGJhY2soKCk6IEFjdGl2ZVNlc3Npb24gfCBudWxsID0+IHtcbiAgICBpZiAoIXNlc3Npb24pIHJldHVybiBudWxsO1xuICAgIGlmIChzdHVkaW9Nb2RlICYmIHBhbmVsPy5hY3RpdmVTcGF3bmVkSWQpIHtcbiAgICAgIGNvbnN0IHNwYXduZWQgPSBwYW5lbC5zcGF3bmVkQXBwcy5maW5kKChlbnRyeSkgPT4gZW50cnkuaWQgPT09IHBhbmVsLmFjdGl2ZVNwYXduZWRJZCk7XG4gICAgICBpZiAoc3Bhd25lZCkge1xuICAgICAgICBjb25zdCBhcHAgPSBsb2FkZWRQbHVnaW5zLmZpbmQoKGVudHJ5KSA9PiBlbnRyeS5oYW5kbGUucGx1Z2luSWQgPT09IHNwYXduZWQucGx1Z2luSWQpPy5tYW5pZmVzdC5hcHBzLmZpbmQoKGNhbmRpZGF0ZSkgPT4gY2FuZGlkYXRlLmlkID09PSBzcGF3bmVkLmFwcElkKTtcbiAgICAgICAgaWYgKGFwcCkgcmV0dXJuIHsgcGx1Z2luSWQ6IHNwYXduZWQucGx1Z2luSWQsIGluc3RhbmNlSWQ6IHNwYXduZWQuaW5zdGFuY2VJZCwgYXBwLCB2aWV3U3RhdGU6IHNlc3Npb24udmlld1N0YXRlIH07XG4gICAgICB9XG4gICAgfVxuICAgIHJldHVybiBzZXNzaW9uO1xuICB9LCBbbG9hZGVkUGx1Z2lucywgcGFuZWwsIHNlc3Npb24sIHN0dWRpb01vZGVdKTtcblxuICAvKipcbiAgICog8J+nte+4jyBgb3BlbkRvY3VtZW50KHJlZiwgYmluZGluZ3MpYCDigJQgcmVwbGFjZXMgYGF0dGFjaFN5bmNCYWNrYm9uZWAncyBVUkktc3RyaW5nIG1pcnJvci4gU3BpbnMgdXAgKG9yXG4gICAqIHJldXNlcykgYPCfn6bvuI9iYWNrYm9uZS3wn5+m77iPd29ya2VyLnRzYCwgdGVsbHMgaXQgdG8gb3BlbiB0aGUgZG9jdW1lbnQsIHN1YnNjcmliZXMgdG8gaXRzIHBvc3RNZXNzYWdlIGV2ZW50cyxcbiAgICogYW5kIGNhbGxzIHRoZSBwbHVnaW4gaW5zdGFuY2UncyBgYXR0YWNoQmFja2JvbmVgL2Bsb2FkQXBwRG9jdW1lbnRgIFdJVC1leHBvcnRlZCBtZXRob2RzIChXUy1EKSBzb1xuICAgKiB0aGUgcGx1Z2luLXNpZGUgc3RvcmUgc3RhcnRzIHB1bXBpbmcgdGhyb3VnaCB0aGUgc2FtZSBsb2dpY2FsIGNoYW5uZWwuIFRoZSBgYWN0b3I6Ly88ZG9jdW1lbnRJZD5gXG4gICAqIHVyaSBtaXJyb3JzIGBmcmFtZXdvcmsvc3luY2AncyBgQ2hhbm5lbEJhY2tib25lOjpwYWlyYCBjb252ZW50aW9uIG9uIHRoZSBSdXN0IHNpZGUuXG4gICAqXG4gICAqIEZ1bGwgbG9vcCBub3RlOiB0aGlzIHdpcmVzIHRoZSBtYWluLXRocmVhZCBoYWxmIG9mIHRoZSBjb250cmFjdC4gVGhlIHJlbWFpbmluZyBob3Ag4oCUIHRoZVxuICAgKiBzYW5kYm94ZWQgcGx1Z2luJ3Mgb3duIGBiYWNrYm9uZS1zZW5kYC9gYmFja2JvbmUtcG9sbGAgV0lUIGhvc3QtaW1wb3J0IGNhbGxzIHJlbGF5aW5nIHRocm91Z2ggaXRzXG4gICAqIGRlZGljYXRlZCBwcm9ncmFtIHdvcmtlciwgdGhyb3VnaCB0aGlzIG1haW4gdGhyZWFkLCBpbnRvIGDwn5+m77iPYmFja2JvbmUt8J+fpu+4j3dvcmtlci50c2Ag4oCUIGlzXG4gICAqIGBmcmFtZXdvcmsvb3MvZGV2L3NjcmlwdC50c2AncyBgcGx1Z2luV29ya2VyU291cmNlYCByZXNwb25zaWJpbGl0eSAoZGV2IHdvcmtmbG93LCBkZWZlcnJlZFxuICAgKiBwZXIgdGhpcyBzZXNzaW9uJ3MgcHJpb3JpdHkgb3JkZXIgaWYgbm90IG90aGVyd2lzZSBjb21wbGV0ZWQpOyBzZWUgdGhhdCBmaWxlJ3Mgb3duIG5vdGVzLlxuICAgKi9cbiAgY29uc3Qgb3BlbkRvY3VtZW50ID0gdXNlQ2FsbGJhY2soXG4gICAgYXN5bmMgKHJlZjogeyByZWFkb25seSBkb2N1bWVudElkOiBzdHJpbmc7IHJlYWRvbmx5IHNjaGVtYTogc3RyaW5nIH0sIGJpbmRpbmdzOiByZWFkb25seSBQZXJzaXN0ZW5jZUJpbmRpbmdbXSkgPT4ge1xuICAgICAgY29uc3QgdGFyZ2V0U2Vzc2lvbiA9IHJlc29sdmVTeW5jVGFyZ2V0U2Vzc2lvbigpO1xuICAgICAgaWYgKCF0YXJnZXRTZXNzaW9uKSByZXR1cm47XG4gICAgICBjb25zdCBwbHVnaW4gPSBsb2FkZWRQbHVnaW5zLmZpbmQoKGVudHJ5KSA9PiBlbnRyeS5oYW5kbGUucGx1Z2luSWQgPT09IHRhcmdldFNlc3Npb24ucGx1Z2luSWQpPy5oYW5kbGU7XG4gICAgICBpZiAoIXBsdWdpbikgcmV0dXJuO1xuICAgICAgY29uc3Qgd29ya2VyID0gZW5zdXJlQmFja2JvbmVXb3JrZXIoKTtcbiAgICAgIG9wZW5Eb2N1bWVudFNlc3Npb25zUmVmLmN1cnJlbnQuc2V0KHJlZi5kb2N1bWVudElkLCB7IHNlc3Npb246IHRhcmdldFNlc3Npb24sIHBsdWdpbiB9KTtcbiAgICAgIC8vIPCfkJrvuI8gUmVnaXN0ZXJzIFRISVMgc2hlbGwgYXMgdGhlIHJvdXRlIGZvciB0aGlzIGRvY3VtZW50J3Mgb3V0Ym91bmQgYmFja2JvbmUgYnl0ZXMgYmVmb3JlIHRoZVxuICAgICAgLy8gcGx1Z2luIGNhbiBwb3NzaWJseSBlbWl0IGFueSAoYXR0YWNoQmFja2JvbmUgYmVsb3cpIOKAlCBzZWUgYHJlbGF5UGx1Z2luQmFja2JvbmVNZXNzYWdlYCdzIGRvYy5cbiAgICAgIHBsdWdpbkJhY2tib25lUm91dGVVbnJlZ2lzdGVyc1JlZi5jdXJyZW50LmdldChyZWYuZG9jdW1lbnRJZCk/LigpO1xuICAgICAgcGx1Z2luQmFja2JvbmVSb3V0ZVVucmVnaXN0ZXJzUmVmLmN1cnJlbnQuc2V0KHJlZi5kb2N1bWVudElkLCByZWdpc3RlclBsdWdpbkJhY2tib25lUm91dGUocmVmLmRvY3VtZW50SWQsIHJlbGF5UGx1Z2luQmFja2JvbmVNZXNzYWdlKSk7XG4gICAgICBjb25zdCByZXF1ZXN0OiBCYWNrYm9uZVdvcmtlclJlcXVlc3QgPSB7XG4gICAgICAgIGtpbmQ6IFwib3BlblwiLFxuICAgICAgICBkb2N1bWVudElkOiByZWYuZG9jdW1lbnRJZCxcbiAgICAgICAgc2NoZW1hOiByZWYuc2NoZW1hLFxuICAgICAgICBiaW5kaW5ncyxcbiAgICAgICAgd2F0Y2hFeHRlcm5hbDogdHJ1ZSxcbiAgICAgICAgYWN0b3I6IHNoZWxsQWN0b3JJZFJlZi5jdXJyZW50LFxuICAgICAgfTtcbiAgICAgIHdvcmtlci5wb3N0TWVzc2FnZShyZXF1ZXN0KTtcbiAgICAgIGNvbnN0IHVyaSA9IGBhY3RvcjovLyR7cmVmLmRvY3VtZW50SWR9YDtcbiAgICAgIGlmIChwbHVnaW4uYXR0YWNoQmFja2JvbmUpIGF3YWl0IHBsdWdpbi5hdHRhY2hCYWNrYm9uZSh0YXJnZXRTZXNzaW9uLmluc3RhbmNlSWQsIHVyaSk7XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1NZTkNfQkFDS0JPTkVfVVJJXCIsIHZhbHVlOiB1cmkgfSk7XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1NZTkNfQ0FSRF9LSU5EXCIsIHZhbHVlOiBudWxsIH0pO1xuICAgIH0sXG4gICAgW2xvYWRlZFBsdWdpbnMsIHJlbGF5UGx1Z2luQmFja2JvbmVNZXNzYWdlLCByZXNvbHZlU3luY1RhcmdldFNlc3Npb25dLFxuICApO1xuXG4gIGNvbnN0IGNsb3NlRG9jdW1lbnQgPSB1c2VDYWxsYmFjaygoZG9jdW1lbnRJZDogc3RyaW5nKSA9PiB7XG4gICAgY29uc3QgZW50cnkgPSBvcGVuRG9jdW1lbnRTZXNzaW9uc1JlZi5jdXJyZW50LmdldChkb2N1bWVudElkKTtcbiAgICBpZiAoZW50cnk/LnBsdWdpbi5kZXRhY2hCYWNrYm9uZSkgdm9pZCBlbnRyeS5wbHVnaW4uZGV0YWNoQmFja2JvbmUoZW50cnkuc2Vzc2lvbi5pbnN0YW5jZUlkKTtcbiAgICBvcGVuRG9jdW1lbnRTZXNzaW9uc1JlZi5jdXJyZW50LmRlbGV0ZShkb2N1bWVudElkKTtcbiAgICBwbHVnaW5CYWNrYm9uZVJvdXRlVW5yZWdpc3RlcnNSZWYuY3VycmVudC5nZXQoZG9jdW1lbnRJZCk/LigpO1xuICAgIHBsdWdpbkJhY2tib25lUm91dGVVbnJlZ2lzdGVyc1JlZi5jdXJyZW50LmRlbGV0ZShkb2N1bWVudElkKTtcbiAgICBjb25zdCByZXF1ZXN0OiBCYWNrYm9uZVdvcmtlclJlcXVlc3QgPSB7IGtpbmQ6IFwiY2xvc2VcIiwgZG9jdW1lbnRJZCB9O1xuICAgIGJhY2tib25lV29ya2VyUmVmLmN1cnJlbnQ/LnBvc3RNZXNzYWdlKHJlcXVlc3QpO1xuICB9LCBbXSk7XG5cbiAgLyoqIEBkZXByZWNhdGVkIHN1cGVyc2VkZWQgYnkge0BsaW5rIG9wZW5Eb2N1bWVudH07IGtlcHQgYXMgYSB0aGluIFVSSS1wYXJzaW5nIGFkYXB0ZXIgb25seSBmb3IgdGhlXG4gICAqIGV4aXN0aW5nIHN5bmMtY2FyZCBVSSAoYG9uQWN0aW9uYCdzIGBhdHRhY2hgIGhhbmRsZXIgYmVsb3cpLCB3aGljaCBzdGlsbCBjb2xsZWN0cyBhIHNpbmdsZSB1cmlcbiAgICogZnJvbSBmaWxlL2ZvbGRlci9yZW1vdGUgcGlja2VycyDigJQgdHJhbnNsYXRlcyB0aGF0IHVyaSBpbnRvIGFuIGBPc0RvY3VtZW50UmVmYCArIGBQZXJzaXN0ZW5jZUJpbmRpbmdgLiAqL1xuICBjb25zdCBhdHRhY2hTeW5jQmFja2JvbmUgPSB1c2VDYWxsYmFjayhcbiAgICBhc3luYyAodXJpOiBzdHJpbmcpID0+IHtcbiAgICAgIGNvbnN0IHRhcmdldFNlc3Npb24gPSByZXNvbHZlU3luY1RhcmdldFNlc3Npb24oKTtcbiAgICAgIGlmICghdGFyZ2V0U2Vzc2lvbikgcmV0dXJuO1xuICAgICAgY29uc3QgZG9jdW1lbnRJZCA9IHN5bmNEb2N1bWVudElkKHRhcmdldFNlc3Npb24sIHBhbmVsLCBzdHVkaW9Nb2RlKTtcbiAgICAgIGNvbnN0IGJpbmRpbmdzOiBQZXJzaXN0ZW5jZUJpbmRpbmdbXSA9IHVyaS5zdGFydHNXaXRoKFwicmVtb3RlOi8vXCIpXG4gICAgICAgID8gKCgpID0+IHtcbiAgICAgICAgICAgIGNvbnN0IHJlc3QgPSB1cmkuc2xpY2UoXCJyZW1vdGU6Ly9cIi5sZW5ndGgpO1xuICAgICAgICAgICAgY29uc3Qgc2xhc2ggPSByZXN0LmluZGV4T2YoXCIvXCIpO1xuICAgICAgICAgICAgY29uc3QgYmFzZVVybCA9IHNsYXNoID4gMCA/IGBodHRwOi8vJHtyZXN0LnNsaWNlKDAsIHNsYXNoKX1gIDogYGh0dHA6Ly8ke3Jlc3R9YDtcbiAgICAgICAgICAgIGNvbnN0IHNwYWNlSWQgPSBzbGFzaCA+IDAgPyByZXN0LnNsaWNlKHNsYXNoICsgMSkgfHwgXCJkZWZhdWx0XCIgOiBcImRlZmF1bHRcIjtcbiAgICAgICAgICAgIHJldHVybiBbeyBraW5kOiBcImh1YlwiLCBiYXNlVXJsLCBzcGFjZUlkIH1dO1xuICAgICAgICAgIH0pKClcbiAgICAgICAgOiB1cmkuc3RhcnRzV2l0aChcImZvbGRlcjovL1wiKVxuICAgICAgICAgID8gW3sga2luZDogXCJmb2xkZXJcIiwgcGF0aDogdXJpLnNsaWNlKFwiZm9sZGVyOi8vXCIubGVuZ3RoKSB9XVxuICAgICAgICAgIDogdXJpLnN0YXJ0c1dpdGgoXCJmaWxlOi8vXCIpXG4gICAgICAgICAgICA/IFt7IGtpbmQ6IFwiZm9sZGVyXCIsIHBhdGg6IHVyaS5zbGljZShcImZpbGU6Ly9cIi5sZW5ndGgpLnJlcGxhY2UoL1xcL1teL10qJC8sIFwiXCIpIH1dXG4gICAgICAgICAgICA6IFtdO1xuICAgICAgYXdhaXQgb3BlbkRvY3VtZW50KHsgZG9jdW1lbnRJZCwgc2NoZW1hOiB0YXJnZXRTZXNzaW9uLmFwcC5kb2N1bWVudC5qb2luKFwiLlwiKSB9LCBiaW5kaW5ncyk7XG4gICAgfSxcbiAgICBbb3BlbkRvY3VtZW50LCBwYW5lbCwgcmVzb2x2ZVN5bmNUYXJnZXRTZXNzaW9uLCBzdHVkaW9Nb2RlXSxcbiAgKTtcblxuICBjb25zdCBkZXRhY2hTeW5jQmFja2JvbmUgPSB1c2VDYWxsYmFjaygoKSA9PiB7XG4gICAgaWYgKHN5bmNCYWNrYm9uZVVyaSkgY2xvc2VEb2N1bWVudChzeW5jQmFja2JvbmVVcmkucmVwbGFjZSgvXmFjdG9yOlxcL1xcLy8sIFwiXCIpKTtcbiAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1NZTkNfQkFDS0JPTkVfVVJJXCIsIHZhbHVlOiBudWxsIH0pO1xuICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfU1lOQ19DQVJEX0tJTkRcIiwgdmFsdWU6IG51bGwgfSk7XG4gIH0sIFtjbG9zZURvY3VtZW50LCBzeW5jQmFja2JvbmVVcmldKTtcblxuICBjb25zdCBzcGF3blByb2dyYW0gPSB1c2VDYWxsYmFjayhcbiAgICBhc3luYyAocHJvZ3JhbTogU3BhY2VQcm9ncmFtRW50cnkpID0+IHtcbiAgICAgIGNvbnN0IHBsdWdpbkVudHJ5ID0gbG9hZGVkUGx1Z2lucy5maW5kKChlbnRyeSkgPT4gZW50cnkuaGFuZGxlLnBsdWdpbklkID09PSBwcm9ncmFtLnBsdWdpbklkKTtcbiAgICAgIGlmICghcGx1Z2luRW50cnkgfHwgIXNlc3Npb24pIHJldHVybjtcbiAgICAgIGNvbnN0IGluc3RhbmNlSWQgPSBhd2FpdCBwbHVnaW5FbnRyeS5oYW5kbGUuY3JlYXRlQXBwKHByb2dyYW0uYXBwSWQpO1xuICAgICAgY29uc3QgY3VycmVudFBhbmVsID0gcGFyc2VQYW5lbFN0YXRlKHNlc3Npb24udmlld1N0YXRlKSA/PyBidWlsZFNwYWNlUGFuZWxTdGF0ZShbXSwgW10pO1xuICAgICAgY29uc3Qgc3Bhd25lZElkID0gYCR7cHJvZ3JhbS5wbHVnaW5JZH0tJHtpbnN0YW5jZUlkfWA7XG4gICAgICB1cGRhdGVTcGFjZVBhbmVsKFxuICAgICAgICBzdHVkaW9QYW5lbEZvY3VzaW5nU3Bhd25lZChjdXJyZW50UGFuZWwsIHtcbiAgICAgICAgICBpZDogc3Bhd25lZElkLFxuICAgICAgICAgIHBsdWdpbklkOiBwcm9ncmFtLnBsdWdpbklkLFxuICAgICAgICAgIGluc3RhbmNlSWQsXG4gICAgICAgICAgYXBwSWQ6IHByb2dyYW0uYXBwSWQsXG4gICAgICAgICAgbGFiZWw6IHByb2dyYW0ubGFiZWwsXG4gICAgICAgICAgZG9jdW1lbnQ6IHByb2dyYW0uZG9jdW1lbnQsXG4gICAgICAgIH0pLFxuICAgICAgKTtcbiAgICB9LFxuICAgIFtsb2FkZWRQbHVnaW5zLCBzZXNzaW9uLCB1cGRhdGVTcGFjZVBhbmVsXSxcbiAgKTtcblxuICBjb25zdCBvbkFjdGlvbiA9IHVzZUNhbGxiYWNrKFxuICAgIChhY3Rpb246IEFjdGlvbkRlc2NyaXB0b3IpID0+IHtcbiAgICAgIGlmIChhY3Rpb24uY29udHJvbGxlcklkID09PSBcInJlY292ZXJ5XCIpIHtcbiAgICAgICAgY29uc3QgYXJncyA9IHR5cGVvZiBhY3Rpb24uYXJncyA9PT0gXCJvYmplY3RcIiAmJiBhY3Rpb24uYXJncyAhPSBudWxsID8gKGFjdGlvbi5hcmdzIGFzIHsgcGx1Z2luSWQ/OiBzdHJpbmcgfSkgOiB7fTtcbiAgICAgICAgY29uc3QgcGx1Z2luSWQgPSBhcmdzLnBsdWdpbklkID8/IHByaW1hcnlQbHVnaW5JZDtcbiAgICAgICAgaWYgKCFwbHVnaW5JZCkgcmV0dXJuO1xuICAgICAgICBpZiAoYWN0aW9uLmFjdGlvbiA9PT0gXCJyZWNvdmVyeS5yZXN0YXJ0QXBwXCIpIHtcbiAgICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1BMVUdJTl9TVVBFUlZJU09SXCIsIHBsdWdpbklkLCB2YWx1ZTogXCJyZXN0YXJ0aW5nXCIgfSk7XG4gICAgICAgICAgdm9pZCByZWxvYWRQbHVnaW4ocGx1Z2luSWQpO1xuICAgICAgICAgIHJldHVybjtcbiAgICAgICAgfVxuICAgICAgICBpZiAoYWN0aW9uLmFjdGlvbiA9PT0gXCJyZWNvdmVyeS5kaXNhYmxlUGx1Z2luXCIpIHtcbiAgICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1BMVUdJTl9TVVBFUlZJU09SXCIsIHBsdWdpbklkLCB2YWx1ZTogXCJxdWFyYW50aW5lZFwiIH0pO1xuICAgICAgICAgIGlmIChwbHVnaW5JZCAhPT0gcHJpbWFyeVBsdWdpbklkKSB2b2lkIHVuaW5zdGFsbFBsdWdpbihwbHVnaW5JZCk7XG4gICAgICAgICAgcmV0dXJuO1xuICAgICAgICB9XG4gICAgICAgIGlmIChhY3Rpb24uYWN0aW9uID09PSBcInJlY292ZXJ5LnNob3dEaWFnbm9zdGljc1wiKSB7XG4gICAgICAgICAgY29uc29sZS5sb2coXCJbREVCVUddIHJlY292ZXJ5IGRpYWdub3N0aWNzXCIsIHsgcGx1Z2luSWQsIHN1cGVydmlzb3I6IHBsdWdpblN1cGVydmlzb3JCeUlkW3BsdWdpbklkXSB9KTtcbiAgICAgICAgICByZXR1cm47XG4gICAgICAgIH1cbiAgICAgIH1cblxuICAgICAgaWYgKCFzZXNzaW9uKSByZXR1cm47XG5cbiAgICAgIC8vIPCfjpPvuI8gRmlyc3QtcnVuIHdhbGt0aHJvdWdoIChtaXJyb3JzIHNldEFjdGl2ZVV0aWxpdHkgYmVsb3cpOiBmdWxseSBzaGVsbC1pbnRlcmNlcHRlZCwgcmVzZXRzXG4gICAgICAvLyBwbGF5YmFjayB0byB0aGUgZmlyc3Qgc3RlcCwgbmV2ZXIgZm9yd2FyZGVkIHRvIHRoZSBwcm9ncmFtLlxuICAgICAgaWYgKGFjdGlvbi5hY3Rpb24gPT09IFNUQVJUX0lOVFJPRFVDVElPTl9BQ1RJT05fSUQpIHtcbiAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9JTlRST0RVQ1RJT05fU1RFUFwiLCB2YWx1ZTogMCB9KTtcbiAgICAgICAgcmV0dXJuO1xuICAgICAgfVxuXG4gICAgICAvLyDwn46l77iPIEZ1bGx5IHNoZWxsLWludGVyY2VwdGVkLCBtaXJyb3JpbmcgYFNUQVJUX0lOVFJPRFVDVElPTl9BQ1RJT05fSURgIGFib3ZlOiBzYW5kYm94ZXMgdGhlXG4gICAgICAvLyBkb2N1bWVudCBhbmQgc3RhcnRzIHR1dG9yaWFsIHBsYXliYWNrIGZyb20gdD0wIChyZWFsIHdvcmsgaGFwcGVucyBpbiBgc3RhcnRUdXRvcmlhbFJlZmAsIHdpcmVkIHVwXG4gICAgICAvLyBieSB0aGUgVHV0b3JpYWxPcmNoZXN0cmF0aW9uIGJsb2NrIGZ1cnRoZXIgZG93biB0aGlzIGNvbXBvbmVudCkuXG4gICAgICBpZiAoYWN0aW9uLmFjdGlvbiA9PT0gU1RBUlRfVFVUT1JJQUxfQUNUSU9OX0lEKSB7XG4gICAgICAgIGNvbnN0IGFyZ3MgPSB0eXBlb2YgYWN0aW9uLmFyZ3MgPT09IFwib2JqZWN0XCIgJiYgYWN0aW9uLmFyZ3MgIT0gbnVsbCA/IChhY3Rpb24uYXJncyBhcyB7IHR1dG9yaWFsSWQ/OiB1bmtub3duIH0pIDoge307XG4gICAgICAgIGlmICh0eXBlb2YgYXJncy50dXRvcmlhbElkID09PSBcInN0cmluZ1wiKSBzdGFydFR1dG9yaWFsUmVmLmN1cnJlbnQoYXJncy50dXRvcmlhbElkKTtcbiAgICAgICAgcmV0dXJuO1xuICAgICAgfVxuICAgICAgaWYgKGFjdGlvbi5hY3Rpb24gPT09IFJFQ09SRF9UVVRPUklBTF9BQ1RJT05fSUQpIHtcbiAgICAgICAgdG9nZ2xlVHV0b3JpYWxSZWNvcmRpbmdSZWYuY3VycmVudCgpO1xuICAgICAgICByZXR1cm47XG4gICAgICB9XG5cbiAgICAgIC8vIPCfjqXvuI8gRGV2aWF0aW9uIGRldGVjdGlvbjogYW55IGFjdGlvbiBOT1Qgc3RhbXBlZCBieSB0aGUgdHV0b3JpYWwgZGlyZWN0b3Ivc2Vlay9jb252ZXJnZSBwYXRoIHdoaWxlXG4gICAgICAvLyBhIHR1dG9yaWFsIGlzIGFjdGl2ZWx5IHBsYXlpbmcgbWVhbnMgdGhlIHVzZXIgZGl2ZXJnZWQgZnJvbSB0aGUgcmVjb3JkaW5nIOKAlCBhdXRvLXBhdXNlIGFuZCBmbGFnXG4gICAgICAvLyBgZGV2aWF0ZWRgIHNvIHByZXNzaW5nIFBsYXkgYWdhaW4gY29udmVyZ2VzIGluc3RlYWQgb2YgcmVzdW1pbmcgYmxpbmRseSBtaWQtZHJpZnQuXG4gICAgICBpZiAodHV0b3JpYWxQbGF5aW5nUmVmLmN1cnJlbnQgJiYgIXR1dG9yaWFsRHJpdmVuUmVmLmN1cnJlbnQpIHtcbiAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9UVVRPUklBTF9QTEFZSU5HXCIsIHZhbHVlOiBmYWxzZSB9KTtcbiAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9UVVRPUklBTF9ERVZJQVRFRFwiLCB2YWx1ZTogdHJ1ZSB9KTtcbiAgICAgIH1cblxuICAgICAgLy8g4o+677iPIFJlY29yZGVyIHRhcDogYW5ub3RhdGlvbmFsLW9ubHkgY2FwdHVyZSAoc2VlIGBUdXRvcmlhbFRyYWNrcy5ldmVudHNgIGRvYyBjb21tZW50KSDigJQgbmV2ZXJcbiAgICAgIC8vIHJlLWRpc3BhdGNoZWQgb24gcGxheWJhY2suIFNraXBzIG5hdmlnYXRpb24vaW50cm9kdWN0aW9uL3R1dG9yaWFsLWNvbnRyb2wgYWN0aW9ucyAobm9pc2UsIG9yXG4gICAgICAvLyBtZWFuaW5nbGVzcyB0byByZXBsYXkpIGFuZCBhbnl0aGluZyB0aGUgZGlyZWN0b3IgaXRzZWxmIGp1c3QgZGlzcGF0Y2hlZC5cbiAgICAgIGlmICh0dXRvcmlhbFJlY29yZGluZ1JlZi5jdXJyZW50ICYmICF0dXRvcmlhbERyaXZlblJlZi5jdXJyZW50KSB7XG4gICAgICAgIGlmICghVFVUT1JJQUxfUkVDT1JESU5HX0VYQ0xVREVEX0FDVElPTl9JRFMuaGFzKGFjdGlvbi5hY3Rpb24pKSB7XG4gICAgICAgICAgdHV0b3JpYWxSZWNvcmRlclJlZi5jdXJyZW50Py5yZWNvcmRFdmVudCh7IGtpbmQ6IFwiYWN0aW9uXCIsIGFjdGlvbjogYWN0aW9uLmFjdGlvbiwgYXJnczogYWN0aW9uLmFyZ3MgYXMgUmVjb3JkPHN0cmluZywgdW5rbm93bj4gfCB1bmRlZmluZWQgfSk7XG4gICAgICAgIH1cbiAgICAgIH1cblxuICAgICAgLy8g8J+nre+4jyBDYW1lcmEtbmF2aWdhdGlvbiBnZXN0dXJlIHJlcG9ydCBmcm9tIGEgM0Qgd2luZG93J3MgYFdvcmxkT3JiaXRHYXRlZGAgKHNoZWxsLW9ubHksIG5ldmVyXG4gICAgICAvLyBmb3J3YXJkZWQgdG8gdGhlIHByb2dyYW0pIOKAlCBjb21wbGV0ZXMgYW55IHBhbi96b29tL29yYml0IGludGVyYWN0aW9uIG9mIHRoZSBhY3RpdmUgc3RlcCB0aGF0XG4gICAgICAvLyB0YXJnZXRzIHRoZSB3aW5kb3cgdGhlIGdlc3R1cmUgaGFwcGVuZWQgb24uIENlbGVicmF0ZXMgb25seSBgd2luZG93SWRgJ3Mgb3duIHBhbmUgKHZpYVxuICAgICAgLy8gYHdpbmRvd0VsZW1lbnRJZGAsIGl0cyB1bmlxdWUgcGVyLWluc3RhbmNlIGVsZW1lbnQgaWQpIOKAlCBuZXZlciB0aGUgd2hvbGUgd2luZG93LWtpbmQgYWxpYXNcbiAgICAgIC8vIHNlbGVjdG9yLCB3aGljaCB3b3VsZCBjZWxlYnJhdGUgZXZlcnkgb3RoZXIgb3BlbiBwYW5lIG9mIHRoYXQgc2FtZSBraW5kIHRvbyAoZS5nLiBhIHNwbGl0IHZpZXcpLlxuICAgICAgaWYgKGFjdGlvbi5hY3Rpb24gPT09IE5PVEVfV09STERfTkFWSUdBVElPTl9BQ1RJT05fSUQpIHtcbiAgICAgICAgY29uc3QgYXJncyA9IHR5cGVvZiBhY3Rpb24uYXJncyA9PT0gXCJvYmplY3RcIiAmJiBhY3Rpb24uYXJncyAhPSBudWxsID8gKGFjdGlvbi5hcmdzIGFzIHsgd2luZG93SWQ/OiB1bmtub3duOyBnZXN0dXJlcz86IHVua25vd24gfSkgOiB7fTtcbiAgICAgICAgY29uc3Qgd2luZG93SWQgPSB0eXBlb2YgYXJncy53aW5kb3dJZCA9PT0gXCJzdHJpbmdcIiA/IGFyZ3Mud2luZG93SWQgOiBcIlwiO1xuICAgICAgICBjb25zdCBnZXN0dXJlcyA9IEFycmF5LmlzQXJyYXkoYXJncy5nZXN0dXJlcykgPyAoYXJncy5nZXN0dXJlcyBhcyByZWFkb25seSBzdHJpbmdbXSkgOiBbXTtcbiAgICAgICAgaWYgKHdpbmRvd0lkKSB7XG4gICAgICAgICAgY29uc3Qgd2luZG93S2luZElkID0gc2Vzc2lvbldpbmRvd0luc3RhbmNlcyhzZXNzaW9uLmFwcCwgZXh0cmFXaW5kb3dJbnN0YW5jZXNSZWYuY3VycmVudCkuZmluZCgoaW5zdGFuY2UpID0+IGluc3RhbmNlLmlkID09PSB3aW5kb3dJZCk/LndpbmRvd0tpbmRJZCA/PyB3aW5kb3dJZDtcbiAgICAgICAgICBmb3IgKGNvbnN0IGdlc3R1cmUgb2YgZ2VzdHVyZXMpIHtcbiAgICAgICAgICAgIGNvbXBsZXRlSW50cm9kdWN0aW9uSW50ZXJhY3Rpb24oXG4gICAgICAgICAgICAgIChpbnRlcmFjdGlvbikgPT4gaW50ZXJhY3Rpb24ub24ua2luZCA9PT0gZ2VzdHVyZSAmJiBpbnRyb2R1Y3Rpb25UYXJnZXRzV2luZG93KHdpbmRvd0lkLCB3aW5kb3dLaW5kSWQsIGludGVyYWN0aW9uLm9uLmlkKSxcbiAgICAgICAgICAgICAgd2luZG93RWxlbWVudElkKHdpbmRvd0lkKSxcbiAgICAgICAgICAgICk7XG4gICAgICAgICAgfVxuICAgICAgICB9XG4gICAgICAgIHJldHVybjtcbiAgICAgIH1cblxuICAgICAgLy8g8J+nsO+4jyBVdGlsaXR5IGFjdGl2YXRpb24gKFA1KTogaG9zdC1vd25lZCBzZXNzaW9uIHN0YXRlLCBuZXZlciBhIGRvY3VtZW50IG9wZXJhdGlvbi4gUmUtY2xpY2tpbmcgdGhlIGFjdGl2ZVxuICAgICAgLy8gdXRpbGl0eSAob3IgYW4gZW1wdHkgdXRpbGl0eUlkKSBkZWFjdGl2YXRlcy4gV2UgcmVzb2x2ZSB0aGUgdGFyZ2V0IHdpbmRvdyBmcm9tIHRoZSBkZXNjcmlwdG9yJ3MgdGFnZ2VkXG4gICAgICAvLyBgd2luZG93SWRgIChzZWUgYHRhZ1NldEFjdGl2ZVV0aWxpdHlXaW5kb3dgKSwgZmFsbGluZyBiYWNrIHRvIHRoZSBhY3RpdmUgd2luZG93LCB1cGRhdGUgdGhlIHN0b3JlLFxuICAgICAgLy8gdGhlbiBmb3J3YXJkIHRoZSByZXNvbHZlZCB1dGlsaXR5IHRvIHRoZSBwbHVnaW4gc28gaXQgY2FuIGNsZWFyL3ByZXBhcmUgc2NyYXRjaC5cbiAgICAgIGlmIChhY3Rpb24uYWN0aW9uID09PSBTRVRfQUNUSVZFX1VUSUxJVFlfQUNUSU9OX0lEKSB7XG4gICAgICAgIGNvbnN0IGFyZ3MgPSB0eXBlb2YgYWN0aW9uLmFyZ3MgPT09IFwib2JqZWN0XCIgJiYgYWN0aW9uLmFyZ3MgIT0gbnVsbCA/IChhY3Rpb24uYXJncyBhcyB7IHV0aWxpdHlJZD86IHVua25vd247IHdpbmRvd0lkPzogdW5rbm93biB9KSA6IHt9O1xuICAgICAgICBjb25zdCB3aW5kb3dJZCA9IHR5cGVvZiBhcmdzLndpbmRvd0lkID09PSBcInN0cmluZ1wiICYmIGFyZ3Mud2luZG93SWQgPyBhcmdzLndpbmRvd0lkIDogKGFjdGl2ZVdpbmRvd0lkUmVmLmN1cnJlbnQgPz8gXCJcIik7XG4gICAgICAgIGlmICghd2luZG93SWQpIHJldHVybjtcbiAgICAgICAgY29uc3QgcmVxdWVzdGVkID0gdHlwZW9mIGFyZ3MudXRpbGl0eUlkID09PSBcInN0cmluZ1wiID8gYXJncy51dGlsaXR5SWQgOiBcIlwiO1xuICAgICAgICBjb25zdCBuZXh0ID0gcmVzb2x2ZVV0aWxpdHlBY3RpdmF0aW9uKGFjdGl2ZVV0aWxpdHlCeVdpbmRvd0lkUmVmLmN1cnJlbnRbd2luZG93SWRdLCByZXF1ZXN0ZWQpO1xuICAgICAgICBzZXRBY3RpdmVVdGlsaXR5Rm9yV2luZG93KHdpbmRvd0lkLCBuZXh0KTtcbiAgICAgICAgLy8g8J+boO+4jyBBIHRvb2wgYW5kIGEgd2luZG93IHV0aWxpdHkgYXJlIG11dHVhbGx5IGV4Y2x1c2l2ZSBpbnRlcmFjdGlvbiBvd25lcnMg4oCUIGFjdGl2YXRpbmcgYSByZWFsXG4gICAgICAgIC8vIHV0aWxpdHkgY2xlYXJzIGFueSBhY3RpdmUgbW9kZS1sZXZlbCB0b29sLlxuICAgICAgICBpZiAobmV4dCAmJiBhY3RpdmVUb29sSWRSZWYuY3VycmVudCkge1xuICAgICAgICAgIGFjdGl2ZVRvb2xJZFJlZi5jdXJyZW50ID0gbnVsbDtcbiAgICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0FDVElWRV9UT09MXCIsIHRvb2xJZDogbnVsbCB9KTtcbiAgICAgICAgfVxuICAgICAgICBpZiAobmV4dCkgY29tcGxldGVJbnRyb2R1Y3Rpb25JbnRlcmFjdGlvbigoaW50ZXJhY3Rpb24pID0+IGludGVyYWN0aW9uLm9uLmtpbmQgPT09IFwidXRpbGl0eVwiICYmIGludGVyYWN0aW9uLm9uLmlkID09PSBuZXh0KTtcbiAgICAgICAgY29uc3QgcGx1Z2luRW50cnkgPSBmaW5kUGx1Z2luRm9yQWN0aW9uKGFjdGlvbik7XG4gICAgICAgIGNvbnN0IHByb2dyYW0gPSBwbHVnaW5FbnRyeT8uaGFuZGxlO1xuICAgICAgICBpZiAocGx1Z2luKSB7XG4gICAgICAgICAgY29uc3Qgdmlld1N0YXRlOiBWaWV3TW9kZWwgPSB7IC4uLnNlc3Npb24udmlld1N0YXRlLCBhY3RpdmVVdGlsaXR5SWQ6IG5leHQgPz8gdW5kZWZpbmVkLCBhY3RpdmVUb29sSWQ6IG5leHQgPyB1bmRlZmluZWQgOiBhY3RpdmVUb29sSWRSZWYuY3VycmVudCA/PyB1bmRlZmluZWQsIHdpbmRvd0lkIH07XG4gICAgICAgICAgY29uc3QgZm9yd2FyZGVkOiBBY3Rpb25EZXNjcmlwdG9yID0geyBjb250cm9sbGVySWQ6IGFjdGlvbi5jb250cm9sbGVySWQsIGFjdGlvbjogYWN0aW9uLmFjdGlvbiwgYXJnczogeyB1dGlsaXR5SWQ6IG5leHQgfSB9O1xuICAgICAgICAgIHZvaWQgcHJvZ3JhbVxuICAgICAgICAgICAgLmhhbmRsZUFjdGlvbihzZXNzaW9uLmluc3RhbmNlSWQsIGVuY29kZUFjdGlvbldpcmUoZm9yd2FyZGVkKSwgdmlld1N0YXRlKVxuICAgICAgICAgICAgLnRoZW4oKHJlc3BvbnNlKSA9PiBhcHBseUhvc3RFZmZlY3RzKHJlc3BvbnNlLnJlcXVlc3RlZEVmZmVjdHMgPz8gW10sIHsgLi4uc2Vzc2lvbiwgdmlld1N0YXRlIH0sIHJlc29sdmVVaURpcnR5U2NvcGUocmVzcG9uc2UudWlTY29wZSkpKVxuICAgICAgICAgICAgLmNhdGNoKCh1dGlsaXR5RXJyb3IpID0+IGNvbnNvbGUuZXJyb3IoXCJbREVCVUddIHNldEFjdGl2ZVV0aWxpdHkgZmFpbGVkXCIsIHV0aWxpdHlFcnJvcikpO1xuICAgICAgICB9XG4gICAgICAgIHJldHVybjtcbiAgICAgIH1cblxuICAgICAgLy8g8J+boO+4jyBUb29sIGFjdGl2YXRpb246IGhvc3Qtb3duZWQgc2Vzc2lvbiBzdGF0ZSAobW9kZS1zY29wZWQsIHdpbmRvd2xlc3MpLCBuZXZlciBhIGRvY3VtZW50IG9wZXJhdGlvbi5cbiAgICAgIC8vIFJlLWNsaWNraW5nIHRoZSBhY3RpdmUgdG9vbCAob3IgYW4gZW1wdHkgdG9vbElkKSBkZWFjdGl2YXRlcy4gTXV0dWFsbHkgZXhjbHVzaXZlIHdpdGggZXZlcnlcbiAgICAgIC8vIHdpbmRvdydzIGFjdGl2ZSB1dGlsaXR5IOKAlCBhY3RpdmF0aW5nIGEgdG9vbCBjbGVhcnMgdGhlbSBhbGwsIG1pcnJvcmluZyBgU0VUX0FDVElWRV9VVElMSVRZX0FDVElPTl9JRGAuXG4gICAgICBpZiAoYWN0aW9uLmFjdGlvbiA9PT0gU0VUX0FDVElWRV9UT09MX0FDVElPTl9JRCkge1xuICAgICAgICBjb25zdCBhcmdzID0gdHlwZW9mIGFjdGlvbi5hcmdzID09PSBcIm9iamVjdFwiICYmIGFjdGlvbi5hcmdzICE9IG51bGwgPyAoYWN0aW9uLmFyZ3MgYXMgeyB0b29sSWQ/OiB1bmtub3duIH0pIDoge307XG4gICAgICAgIGNvbnN0IHJlcXVlc3RlZCA9IHR5cGVvZiBhcmdzLnRvb2xJZCA9PT0gXCJzdHJpbmdcIiA/IGFyZ3MudG9vbElkIDogXCJcIjtcbiAgICAgICAgY29uc3QgbmV4dCA9IHJlc29sdmVVdGlsaXR5QWN0aXZhdGlvbihhY3RpdmVUb29sSWRSZWYuY3VycmVudCwgcmVxdWVzdGVkKTtcbiAgICAgICAgYWN0aXZlVG9vbElkUmVmLmN1cnJlbnQgPSBuZXh0O1xuICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0FDVElWRV9UT09MXCIsIHRvb2xJZDogbmV4dCB9KTtcbiAgICAgICAgaWYgKG5leHQpIGNsZWFyQWxsV2luZG93VXRpbGl0aWVzKCk7XG4gICAgICAgIGlmIChuZXh0KSBjb21wbGV0ZUludHJvZHVjdGlvbkludGVyYWN0aW9uKChpbnRlcmFjdGlvbikgPT4gaW50ZXJhY3Rpb24ub24ua2luZCA9PT0gXCJ0b29sXCIgJiYgaW50ZXJhY3Rpb24ub24uaWQgPT09IG5leHQpO1xuICAgICAgICBjb25zdCBwbHVnaW5FbnRyeSA9IGZpbmRQbHVnaW5Gb3JBY3Rpb24oYWN0aW9uKTtcbiAgICAgICAgY29uc3QgcHJvZ3JhbSA9IHBsdWdpbkVudHJ5Py5oYW5kbGU7XG4gICAgICAgIGlmIChwbHVnaW4pIHtcbiAgICAgICAgICBjb25zdCB2aWV3U3RhdGU6IFZpZXdNb2RlbCA9IHsgLi4uc2Vzc2lvbi52aWV3U3RhdGUsIGFjdGl2ZVRvb2xJZDogbmV4dCA/PyB1bmRlZmluZWQsIGFjdGl2ZVV0aWxpdHlJZDogbmV4dCA/IHVuZGVmaW5lZCA6IHNlc3Npb24udmlld1N0YXRlLmFjdGl2ZVV0aWxpdHlJZCB9O1xuICAgICAgICAgIGNvbnN0IGZvcndhcmRlZDogQWN0aW9uRGVzY3JpcHRvciA9IHsgY29udHJvbGxlcklkOiBhY3Rpb24uY29udHJvbGxlcklkLCBhY3Rpb246IGFjdGlvbi5hY3Rpb24sIGFyZ3M6IHsgdG9vbElkOiBuZXh0IH0gfTtcbiAgICAgICAgICB2b2lkIHByb2dyYW1cbiAgICAgICAgICAgIC5oYW5kbGVBY3Rpb24oc2Vzc2lvbi5pbnN0YW5jZUlkLCBlbmNvZGVBY3Rpb25XaXJlKGZvcndhcmRlZCksIHZpZXdTdGF0ZSlcbiAgICAgICAgICAgIC50aGVuKChyZXNwb25zZSkgPT4gYXBwbHlIb3N0RWZmZWN0cyhyZXNwb25zZS5yZXF1ZXN0ZWRFZmZlY3RzID8/IFtdLCB7IC4uLnNlc3Npb24sIHZpZXdTdGF0ZSB9LCByZXNvbHZlVWlEaXJ0eVNjb3BlKHJlc3BvbnNlLnVpU2NvcGUpKSlcbiAgICAgICAgICAgIC5jYXRjaCgodG9vbEVycm9yKSA9PiBjb25zb2xlLmVycm9yKFwiW0RFQlVHXSBzZXRBY3RpdmVUb29sIGZhaWxlZFwiLCB0b29sRXJyb3IpKTtcbiAgICAgICAgfVxuICAgICAgICByZXR1cm47XG4gICAgICB9XG5cbiAgICAgIGNvbXBsZXRlSW50cm9kdWN0aW9uSW50ZXJhY3Rpb24oKGludGVyYWN0aW9uKSA9PiBpbnRlcmFjdGlvbi5vbi5raW5kID09PSBcImFjdGlvblwiICYmIGludGVyYWN0aW9uLm9uLmlkID09PSBhY3Rpb24uYWN0aW9uKTtcblxuICAgICAgaWYgKGFjdGlvbi5jb250cm9sbGVySWQgPT09IEZSQU1FV09SS19TWU5DX0NPTlRST0xMRVJfSUQpIHtcbiAgICAgICAgaWYgKGFjdGlvbi5hY3Rpb24gPT09IFwic2VsZWN0RmlsZVwiKSB7XG4gICAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9TWU5DX0NBUkRfS0lORFwiLCB2YWx1ZTogXCJmaWxlXCIgfSk7XG4gICAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9TWU5DX0RSQUZUX1BBVEhcIiwgdmFsdWU6IHN5bmNCYWNrYm9uZVVyaT8uc3RhcnRzV2l0aChcImZpbGU6Ly9cIikgPyBzeW5jQmFja2JvbmVVcmkuc2xpY2UoXCJmaWxlOi8vXCIubGVuZ3RoKSA6IFwiXCIgfSk7XG4gICAgICAgICAgcmV0dXJuO1xuICAgICAgICB9XG4gICAgICAgIGlmIChhY3Rpb24uYWN0aW9uID09PSBcInNlbGVjdEZvbGRlclwiKSB7XG4gICAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9TWU5DX0NBUkRfS0lORFwiLCB2YWx1ZTogXCJmb2xkZXJcIiB9KTtcbiAgICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1NZTkNfRFJBRlRfUEFUSFwiLCB2YWx1ZTogc3luY0JhY2tib25lVXJpPy5zdGFydHNXaXRoKFwiZm9sZGVyOi8vXCIpID8gc3luY0JhY2tib25lVXJpLnNsaWNlKFwiZm9sZGVyOi8vXCIubGVuZ3RoKSA6IFwiXCIgfSk7XG4gICAgICAgICAgcmV0dXJuO1xuICAgICAgICB9XG4gICAgICAgIGlmIChhY3Rpb24uYWN0aW9uID09PSBcInNlbGVjdFJlbW90ZVwiKSB7XG4gICAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9TWU5DX0NBUkRfS0lORFwiLCB2YWx1ZTogXCJyZW1vdGVcIiB9KTtcbiAgICAgICAgICBjb25zdCByZW1vdGUgPSBzeW5jQmFja2JvbmVVcmk/LnN0YXJ0c1dpdGgoXCJyZW1vdGU6Ly9cIikgPyBzeW5jQmFja2JvbmVVcmkuc2xpY2UoXCJyZW1vdGU6Ly9cIi5sZW5ndGgpIDogXCJcIjtcbiAgICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1NZTkNfRFJBRlRfUEFUSFwiLCB2YWx1ZTogcmVtb3RlIH0pO1xuICAgICAgICAgIHJldHVybjtcbiAgICAgICAgfVxuICAgICAgICBpZiAoYWN0aW9uLmFjdGlvbiA9PT0gXCJhdHRhY2hcIikge1xuICAgICAgICAgIGNvbnN0IHBhdGggPSB0eXBlb2YgYWN0aW9uLmFyZ3MgPT09IFwib2JqZWN0XCIgJiYgYWN0aW9uLmFyZ3MgIT0gbnVsbCAmJiBcInBhdGhcIiBpbiBhY3Rpb24uYXJncyA/IFN0cmluZygoYWN0aW9uLmFyZ3MgYXMgeyBwYXRoPzogc3RyaW5nIH0pLnBhdGggPz8gXCJcIikgOiBzeW5jRHJhZnRQYXRoO1xuICAgICAgICAgIGlmICghcGF0aC50cmltKCkpIHJldHVybjtcbiAgICAgICAgICBjb25zdCB1cmkgPVxuICAgICAgICAgICAgYWN0aW9uLmFyZ3MgJiYgdHlwZW9mIGFjdGlvbi5hcmdzID09PSBcIm9iamVjdFwiICYmIFwia2luZFwiIGluIGFjdGlvbi5hcmdzXG4gICAgICAgICAgICAgID8gU3RyaW5nKChhY3Rpb24uYXJncyBhcyB7IGtpbmQ/OiBzdHJpbmcgfSkua2luZCkgPT09IFwicmVtb3RlXCJcbiAgICAgICAgICAgICAgICA/ICgoKSA9PiB7XG4gICAgICAgICAgICAgICAgICAgIGNvbnN0IFtob3N0UG9ydCwgLi4ucmVzdF0gPSBwYXRoLnNwbGl0KFwiL1wiKTtcbiAgICAgICAgICAgICAgICAgICAgY29uc3QgW3NwYWNlSWQsIGRvY3VtZW50SWRdID0gcmVzdC5sZW5ndGggPj0gMiA/IFtyZXN0WzBdLCByZXN0LnNsaWNlKDEpLmpvaW4oXCIvXCIpXSA6IFtcImRlZmF1bHRcIiwgcmVzdFswXSB8fCBzeW5jRG9jdW1lbnRJZChzZXNzaW9uLCBwYW5lbCwgc3R1ZGlvTW9kZSldO1xuICAgICAgICAgICAgICAgICAgICByZXR1cm4gYnVpbGRSZW1vdGVCYWNrYm9uZVVyaShob3N0UG9ydCA/PyBcIjEyNy4wLjAuMTo4Nzg3XCIsIHNwYWNlSWQsIGRvY3VtZW50SWQpO1xuICAgICAgICAgICAgICAgICAgfSkoKVxuICAgICAgICAgICAgICAgIDogU3RyaW5nKChhY3Rpb24uYXJncyBhcyB7IGtpbmQ/OiBzdHJpbmcgfSkua2luZCkgPT09IFwiZm9sZGVyXCJcbiAgICAgICAgICAgICAgICAgID8gYnVpbGRGb2xkZXJCYWNrYm9uZVVyaShwYXRoKVxuICAgICAgICAgICAgICAgICAgOiBidWlsZEZpbGVCYWNrYm9uZVVyaShwYXRoKVxuICAgICAgICAgICAgICA6IGJ1aWxkRmlsZUJhY2tib25lVXJpKHBhdGgpO1xuICAgICAgICAgIHZvaWQgYXR0YWNoU3luY0JhY2tib25lKHVyaSk7XG4gICAgICAgICAgcmV0dXJuO1xuICAgICAgICB9XG4gICAgICAgIGlmIChhY3Rpb24uYWN0aW9uID09PSBcImRldGFjaFwiKSB7XG4gICAgICAgICAgdm9pZCBkZXRhY2hTeW5jQmFja2JvbmUoKTtcbiAgICAgICAgICByZXR1cm47XG4gICAgICAgIH1cbiAgICAgICAgcmV0dXJuO1xuICAgICAgfVxuXG4gICAgICBpZiAoc3R1ZGlvTW9kZSAmJiBhY3Rpb24uY29udHJvbGxlcklkID09PSBsYW5kaW5nQ29udHJvbGxlcklkICYmIGFjdGlvbi5hY3Rpb24gPT09IFwiaW1wb3J0U3BhY2VcIikge1xuICAgICAgICBpbXBvcnRTcGFjZUlucHV0UmVmLmN1cnJlbnQ/LmNsaWNrKCk7XG4gICAgICAgIHJldHVybjtcbiAgICAgIH1cblxuICAgICAgaWYgKHN0dWRpb01vZGUgJiYgYWN0aW9uLmFjdGlvbiA9PT0gXCJzcGF3bkFwcFwiICYmIGFjdGlvbi5jb250cm9sbGVySWQgIT09IGhvc3RDb250cm9sbGVySWQpIHtcbiAgICAgICAgY29uc3QgcGx1Z2luSWQgPSB0eXBlb2YgYWN0aW9uLmFyZ3MgPT09IFwib2JqZWN0XCIgJiYgYWN0aW9uLmFyZ3MgIT0gbnVsbCAmJiBcInBsdWdpbklkXCIgaW4gYWN0aW9uLmFyZ3MgPyBTdHJpbmcoKGFjdGlvbi5hcmdzIGFzIHsgcGx1Z2luSWQ/OiBzdHJpbmcgfSkucGx1Z2luSWQgPz8gXCJcIikgOiBcIlwiO1xuICAgICAgICBjb25zdCBjdXJyZW50UGFuZWwgPSBwYXJzZVBhbmVsU3RhdGUoc2Vzc2lvbi52aWV3U3RhdGUpO1xuICAgICAgICBjb25zdCBwcm9ncmFtID0gY3VycmVudFBhbmVsPy5wcm9ncmFtcy5maW5kKChlbnRyeSkgPT4gZW50cnkucGx1Z2luSWQgPT09IHBsdWdpbklkKTtcbiAgICAgICAgaWYgKHByb2dyYW0pIHZvaWQgc3Bhd25Qcm9ncmFtKHByb2dyYW0pO1xuICAgICAgICByZXR1cm47XG4gICAgICB9XG5cbiAgICAgIGlmIChzdHVkaW9Nb2RlICYmIGFjdGlvbi5jb250cm9sbGVySWQgPT09IGhvc3RDb250cm9sbGVySWQgJiYgYWN0aW9uLmFjdGlvbiA9PT0gXCJzZXRBY3RpdmVQYW5lbFRhYlwiKSB7XG4gICAgICAgIGNvbnN0IHRhYklkID0gdHlwZW9mIGFjdGlvbi5hcmdzID09PSBcIm9iamVjdFwiICYmIGFjdGlvbi5hcmdzICE9IG51bGwgJiYgXCJ0YWJJZFwiIGluIGFjdGlvbi5hcmdzID8gU3RyaW5nKChhY3Rpb24uYXJncyBhcyB7IHRhYklkPzogc3RyaW5nIH0pLnRhYklkID8/IGhvc3RDYXRhbG9ndWVUYWJJZCA/PyBcIlwiKSA6IChob3N0Q2F0YWxvZ3VlVGFiSWQgPz8gXCJcIik7XG4gICAgICAgIGNvbnN0IGN1cnJlbnRQYW5lbCA9IHBhcnNlUGFuZWxTdGF0ZShzZXNzaW9uLnZpZXdTdGF0ZSkgPz8gYnVpbGRTcGFjZVBhbmVsU3RhdGUoW10sIFtdKTtcbiAgICAgICAgdXBkYXRlU3BhY2VQYW5lbChidWlsZFNwYWNlUGFuZWxTdGF0ZShjdXJyZW50UGFuZWwucHJvZ3JhbXMsIGN1cnJlbnRQYW5lbC5zcGF3bmVkQXBwcywgdGFiSWQsIGN1cnJlbnRQYW5lbC5hY3RpdmVTcGF3bmVkSWQpKTtcbiAgICAgICAgcmV0dXJuO1xuICAgICAgfVxuXG4gICAgICBjb25zdCBwbHVnaW5FbnRyeSA9IGZpbmRQbHVnaW5Gb3JBY3Rpb24oYWN0aW9uKTtcbiAgICAgIGNvbnN0IHBsdWdpbiA9IHBsdWdpbkVudHJ5Py5oYW5kbGU7XG4gICAgICBpZiAoIXBsdWdpbikgcmV0dXJuO1xuXG4gICAgICBjb25zdCB0YXJnZXRTZXNzaW9uID1cbiAgICAgICAgc3R1ZGlvTW9kZSAmJiBhY3Rpb24uY29udHJvbGxlcklkICE9PSBzZXNzaW9uLmFwcC5jb250cm9sbGVySWRcbiAgICAgICAgICA/ICgoKSA9PiB7XG4gICAgICAgICAgICAgIGNvbnN0IHNwYXduZWQgPSBwYW5lbD8uc3Bhd25lZEFwcHMuZmluZCgoZW50cnkpID0+IHtcbiAgICAgICAgICAgICAgICBjb25zdCBhcHAgPSBsb2FkZWRQbHVnaW5zLmZpbmQoKHApID0+IHAuaGFuZGxlLnBsdWdpbklkID09PSBlbnRyeS5wbHVnaW5JZCk/Lm1hbmlmZXN0LmFwcHMuZmluZCgoYSkgPT4gYS5pZCA9PT0gZW50cnkuYXBwSWQpO1xuICAgICAgICAgICAgICAgIHJldHVybiBhcHA/LmNvbnRyb2xsZXJJZCA9PT0gYWN0aW9uLmNvbnRyb2xsZXJJZDtcbiAgICAgICAgICAgICAgfSk7XG4gICAgICAgICAgICAgIGlmICghc3Bhd25lZCkgcmV0dXJuIHNlc3Npb247XG4gICAgICAgICAgICAgIGNvbnN0IGFwcCA9IGxvYWRlZFBsdWdpbnMuZmluZCgocCkgPT4gcC5oYW5kbGUucGx1Z2luSWQgPT09IHNwYXduZWQucGx1Z2luSWQpPy5tYW5pZmVzdC5hcHBzLmZpbmQoKGEpID0+IGEuaWQgPT09IHNwYXduZWQuYXBwSWQpO1xuICAgICAgICAgICAgICBpZiAoIWFwcCkgcmV0dXJuIHNlc3Npb247XG4gICAgICAgICAgICAgIHJldHVybiB7IHBsdWdpbklkOiBzcGF3bmVkLnBsdWdpbklkLCBpbnN0YW5jZUlkOiBzcGF3bmVkLmluc3RhbmNlSWQsIGFwcCwgdmlld1N0YXRlOiBzZXNzaW9uLnZpZXdTdGF0ZSB9O1xuICAgICAgICAgICAgfSkoKVxuICAgICAgICAgIDogc2Vzc2lvbjtcblxuICAgICAgLy8g8J+aq++4jyBUaGUgb2xkIGBzZXREb2N1bWVudGAg4oaSIGBwYXRjaEFwcFNvdXJjZWAgbWlycm9yIChzcGF3bmVkLWluc3RhbmNlIGNvbnRlbnQgd3JpdGUtYmFjayBvbiB0aGVcbiAgICAgIC8vIG9zIGRvY3VtZW50KSBpcyBkZWxldGVkIOKAlCBhcHAgY29udGVudCBubyBsb25nZXIgZW1iZWRzIG9uIHRoZSBvcyBkb2N1bWVudCBhdCBhbGxcbiAgICAgIC8vIChgT3NBcHBJbnN0YW5jZS5kb2N1bWVudGAgaXMgbm93IGp1c3QgYW4gYE9zRG9jdW1lbnRSZWZgIGhhbmRsZSkuIEEgc3Bhd25lZCBpbnN0YW5jZSdzIGNvbnRlbnRcbiAgICAgIC8vIHN5bmMgbm93IGdvZXMgdGhyb3VnaCBpdHMgb3duIGBvcGVuRG9jdW1lbnRgLW9wZW5lZCBgRG9jdW1lbnRIb3N0YCBjaGFubmVsLCBzYW1lIGFzIGFueSBvdGhlclxuICAgICAgLy8gZG9jdW1lbnQ7IHRoZXJlIGlzIG5vIGhvc3Qtc2lkZSBKUyBtaXJyb3Jpbmcgc3RlcCBhbnltb3JlLlxuICAgICAgLy8g8J+qn++4jyBgd2luZG93SWRgIGlzIHJlYWQgYmFjayBvZmYgdGhlIHRhZ2dlZCBgYWN0aW9uLmFyZ3NgIChzZWUgYHdpbmRvd01lYXN1cmVzQ2hyb21lYC9gdGFnU2V0QWN0aXZlVXRpbGl0eVdpbmRvd2ApLFxuICAgICAgLy8gZmFsbGluZyBiYWNrIHRvIHRoZSBhY3RpdmUgd2luZG93IOKAlCBzdGFtcGVkIGludG8gdGhlIGRpc3BhdGNoZWQgdmlldyBzdGF0ZSBzbyB0aGUgcGx1Z2luIGNhbiBrZXkgYW55XG4gICAgICAvLyBwZXItd2luZG93IG9wdGlvbiBtdXRhdGlvbiBvZmYgYHZpZXdfc3RhdGUud2luZG93SWRgIGluc3RlYWQgb2YgZXZlciBndWVzc2luZyBhdCB0aGUgYWN0aXZlIHdpbmRvdy5cbiAgICAgIGNvbnN0IGFjdGlvbldpbmRvd0lkID0gdHlwZW9mIGFjdGlvbi5hcmdzID09PSBcIm9iamVjdFwiICYmIGFjdGlvbi5hcmdzICE9IG51bGwgJiYgdHlwZW9mIChhY3Rpb24uYXJncyBhcyB7IHdpbmRvd0lkPzogdW5rbm93biB9KS53aW5kb3dJZCA9PT0gXCJzdHJpbmdcIiA/IChhY3Rpb24uYXJncyBhcyB7IHdpbmRvd0lkOiBzdHJpbmcgfSkud2luZG93SWQgOiB1bmRlZmluZWQ7XG4gICAgICBjb25zdCBkaXNwYXRjaFdpbmRvd0lkID0gYWN0aW9uV2luZG93SWQgPz8gYWN0aXZlV2luZG93SWRSZWYuY3VycmVudCA/PyB1bmRlZmluZWQ7XG4gICAgICBjb25zdCBkaXNwYXRjaFZpZXdTdGF0ZSA9IGluamVjdEFjdGl2ZVV0aWxpdHkoXG4gICAgICAgIHtcbiAgICAgICAgICAuLi50YXJnZXRTZXNzaW9uLnZpZXdTdGF0ZSxcbiAgICAgICAgICB3aW5kb3dJZDogZGlzcGF0Y2hXaW5kb3dJZCxcbiAgICAgICAgICB3aW5kb3dJbnN0YW5jZXM6IHNlc3Npb25XaW5kb3dJbnN0YW5jZXModGFyZ2V0U2Vzc2lvbi5hcHAsIGV4dHJhV2luZG93SW5zdGFuY2VzUmVmLmN1cnJlbnQpLm1hcCgoaW5zdGFuY2UpID0+ICh7IGlkOiBpbnN0YW5jZS5pZCwgd2luZG93S2luZElkOiBpbnN0YW5jZS53aW5kb3dLaW5kSWQgfSkpLFxuICAgICAgICB9LFxuICAgICAgICBkaXNwYXRjaFdpbmRvd0lkLFxuICAgICAgKTtcbiAgICAgIGNvbnN0IGRlY2xhcmVkQWN0aW9uID0gdGFyZ2V0U2Vzc2lvbi5hcHAuYWN0aW9ucz8uc29tZSgoZW50cnkpID0+IGVudHJ5LmlkID09PSBhY3Rpb24uYWN0aW9uKSA/PyBmYWxzZTtcbiAgICAgIGlmICghZGVjbGFyZWRBY3Rpb24gJiYgIUZSQU1FV09SS19SRVNFUlZFRF9BQ1RJT05fSURTLmhhcyhhY3Rpb24uYWN0aW9uKSkge1xuICAgICAgICBjb25zb2xlLndhcm4oXCJbREVCVUddIHNraXBwaW5nIHVuZGVjbGFyZWQgYWN0aW9uXCIsIGFjdGlvbi5hY3Rpb24sIHRhcmdldFNlc3Npb24uYXBwLmlkKTtcbiAgICAgICAgcmV0dXJuO1xuICAgICAgfVxuXG4gICAgICBjb25zdCBpbnRlcmFjdGl2ZUFjdGlvbiA9IGFjdGlvbi5hY3Rpb24gIT09IFwic3VnZ2VzdGlvbnNUaWNrXCIgJiYgYWN0aW9uLmFjdGlvbiAhPT0gXCJmaWxsQnVpbGRUaWNrXCI7XG4gICAgICBpZiAoaW50ZXJhY3RpdmVBY3Rpb24pIGJlZ2luSW50ZXJhY3RpdmVQbHVnaW5BY3Rpb24oKTtcbiAgICAgIHJldHVybiBwbHVnaW5cbiAgICAgICAgLmhhbmRsZUFjdGlvbih0YXJnZXRTZXNzaW9uLmluc3RhbmNlSWQsIGVuY29kZUFjdGlvbldpcmUoYWN0aW9uKSwgZGlzcGF0Y2hWaWV3U3RhdGUpXG4gICAgICAgIC50aGVuKChyZXNwb25zZSkgPT4gYXBwbHlIb3N0RWZmZWN0cyhyZXNwb25zZS5yZXF1ZXN0ZWRFZmZlY3RzID8/IFtdLCB7IC4uLnRhcmdldFNlc3Npb24sIHZpZXdTdGF0ZTogZGlzcGF0Y2hWaWV3U3RhdGUgfSwgcmVzb2x2ZVVpRGlydHlTY29wZShyZXNwb25zZS51aVNjb3BlKSkpXG4gICAgICAgIC5jYXRjaCgoYWN0aW9uRXJyb3IpID0+IHtcbiAgICAgICAgICBjb25zb2xlLmVycm9yKFwiW0RFQlVHXSBhY3Rpb24gZmFpbGVkXCIsIGFjdGlvbi5hY3Rpb24sIGFjdGlvbi5hcmdzLCBhY3Rpb25FcnJvcik7XG4gICAgICAgIH0pXG4gICAgICAgIC5maW5hbGx5KCgpID0+IHtcbiAgICAgICAgICBpZiAoaW50ZXJhY3RpdmVBY3Rpb24pIGVuZEludGVyYWN0aXZlUGx1Z2luQWN0aW9uKCk7XG4gICAgICAgIH0pO1xuICAgIH0sXG4gICAgW1xuICAgICAgYXBwbHlIb3N0RWZmZWN0cyxcbiAgICAgIGF0dGFjaFN5bmNCYWNrYm9uZSxcbiAgICAgIGNsZWFyQWxsV2luZG93VXRpbGl0aWVzLFxuICAgICAgZGV0YWNoU3luY0JhY2tib25lLFxuICAgICAgZmluZFBsdWdpbkZvckFjdGlvbixcbiAgICAgIGluamVjdEFjdGl2ZVV0aWxpdHksXG4gICAgICBsb2FkZWRQbHVnaW5zLFxuICAgICAgcGFuZWwsXG4gICAgICBzZXNzaW9uLFxuICAgICAgc2V0QWN0aXZlVXRpbGl0eUZvcldpbmRvdyxcbiAgICAgIHNwYXduUHJvZ3JhbSxcbiAgICAgIHN0dWRpb01vZGUsXG4gICAgICBzeW5jQmFja2JvbmVVcmksXG4gICAgICBzeW5jRHJhZnRQYXRoLFxuICAgICAgdXBkYXRlU3BhY2VQYW5lbCxcbiAgICAgIGhvc3RDb250cm9sbGVySWQsXG4gICAgICBsYW5kaW5nQ29udHJvbGxlcklkLFxuICAgICAgaG9zdENhdGFsb2d1ZVRhYklkLFxuICAgICAgY29tcGxldGVJbnRyb2R1Y3Rpb25JbnRlcmFjdGlvbixcbiAgICAgIHByaW1hcnlQbHVnaW5JZCxcbiAgICAgIHJlbG9hZFBsdWdpbixcbiAgICAgIHVuaW5zdGFsbFBsdWdpbixcbiAgICAgIHBsdWdpblN1cGVydmlzb3JCeUlkLFxuICAgIF0sXG4gICk7XG5cbiAgLyoqIPCfp63vuI8gTG9ncyBhIHNoZWxsLWNocm9tZSBjb21tYW5kICh0aGVtZSBjaGFuZ2UsIGRvY2sgZHJhZywgd2luZG93IHJlc2l6ZSwgcGFuZWwgdG9nZ2xlLCDigKYpIGludG8gdGhlXG4gICAqIHBsdWdpbidzIHNlc3Npb24tb25seSBjb21tYW5kLWhpc3RvcnkgcGFuZWwg4oCUIHJvdXRlZCB0aHJvdWdoIHRoZSBleGFjdCBzYW1lIGBvbkFjdGlvbmAgZnVubmVsIGFzIGV2ZXJ5XG4gICAqIG90aGVyIGFjdGlvbiAoc2VlIGBOT1RFX1NIRUxMX0NPTU1BTkRfQUNUSU9OX0lEYCkgc28gaXQgbGFuZHMgb24gYHRhcmdldFNlc3Npb24uaW5zdGFuY2VJZGAgdmlhIHRoZVxuICAgKiBzdGFuZGFyZCBgaGFuZGxlQWN0aW9uYCBjYWxsLCBqdXN0IHRhZ2dlZCB3aXRoIGFuIGlkIHRoZSBwbHVnaW4gaW50ZXJjZXB0cyBiZWZvcmUgdGhlIGFwcCBzZWVzIGl0LlxuICAgKiBOby1vcHMgd2hlbiB0aGVyZSdzIG5vIGFjdGl2ZSBhcHAgc2Vzc2lvbi4gKi9cbiAgY29uc3Qgbm90ZVNoZWxsQ29tbWFuZCA9IHVzZUNhbGxiYWNrKFxuICAgIChjb21tYW5kSWQ6IHN0cmluZywgbGFiZWw6IHN0cmluZywgZGV0YWlsPzogUmVjb3JkPHN0cmluZywgdW5rbm93bj4pID0+IHtcbiAgICAgIGlmICghc2Vzc2lvbikgcmV0dXJuO1xuICAgICAgb25BY3Rpb24oYnVpbGROb3RlU2hlbGxDb21tYW5kQWN0aW9uKHNlc3Npb24uYXBwLmNvbnRyb2xsZXJJZCwgY29tbWFuZElkLCBsYWJlbCwgZGV0YWlsKSk7XG4gICAgfSxcbiAgICBbc2Vzc2lvbiwgb25BY3Rpb25dLFxuICApO1xuXG4gIGNvbnN0IG9uQWN0aW9uUmVmID0gdXNlUmVmKG9uQWN0aW9uKTtcbiAgdXNlRWZmZWN0KCgpID0+IHtcbiAgICBvbkFjdGlvblJlZi5jdXJyZW50ID0gb25BY3Rpb247XG4gIH0sIFtvbkFjdGlvbl0pO1xuXG4gIC8vIPCfkKLvuI8gYG9uQWN0aW9uYCdzIG93biBpZGVudGl0eSBjaHVybnMgZXZlcnkgYWN0aW9uIChpdHMgZGVwcyBpbmNsdWRlIGBzZXNzaW9uYCwgYHBhbmVsYCwg4oCmKS4gUmVuZGVyXG4gIC8vIHRyZWVzIGJ1aWx0IGZyb20gYFVpTm9kZWBzIG9ubHkgbmVlZCBhICpjYWxsYWJsZSogYWN0aW9uIGRpc3BhdGNoZXIsIG5vdCBhIGZyZXNoIG9uZSBlYWNoIHRpbWUg4oCUXG4gIC8vIHJvdXRlIHRoZW0gdGhyb3VnaCB0aGlzIHBlcm1hbmVudGx5LXN0YWJsZSByZWYgaW5kaXJlY3Rpb24gc28gYGludGVycHJldFVpTm9kZWAncyBgUmVhY3QubWVtb2BcbiAgLy8gKGFuZCBhbnkgYHVzZU1lbW9gIGtleWVkIG9uIHRoZSBkaXNwYXRjaGVyIHBhc3NlZCB0byBpdCkgY2FuIGFjdHVhbGx5IGJhaWwuXG4gIGNvbnN0IG9uQWN0aW9uU3RhYmxlID0gdXNlQ2FsbGJhY2soKGFjdGlvbjogUGFyYW1ldGVyczx0eXBlb2Ygb25BY3Rpb24+WzBdKSA9PiBvbkFjdGlvblJlZi5jdXJyZW50KGFjdGlvbiksIFtdKTtcblxuICAvLyNyZWdpb24g8J+Ope+4j1R1dG9yaWFsT3JjaGVzdHJhdGlvblxuICAvKiog4o+x77iPIFJlYWwtdGltZSB0aHJvdHRsZSBmb3IgdGhlIGRpcmVjdG9yJ3MgVUkvZG9jdW1lbnQvZXZlbnQgYXBwbGljYXRpb24gKH4xMEh6KSDigJQgY2FtZXJhIHN0YXlzXG4gICAqIHNtb290aCBldmVyeSBjbG9jayB0aWNrIHJlZ2FyZGxlc3MgKHNlZSB0aGUgYHN1YnNjcmliZWAgY2FsbGJhY2sgYmVsb3cpLiAqL1xuICBjb25zdCBUVVRPUklBTF9ESVJFQ1RPUl9USUNLX01TID0gOTA7XG5cbiAgY29uc3QgYWN0aXZlVHV0b3JpYWwgPSB1c2VNZW1vKCgpID0+IGFjdGl2ZVR1dG9yaWFscy5maW5kKCh0dXRvcmlhbCkgPT4gdHV0b3JpYWwuaWQgPT09IGFjdGl2ZVR1dG9yaWFsSWQpID8/IG51bGwsIFthY3RpdmVUdXRvcmlhbHMsIGFjdGl2ZVR1dG9yaWFsSWRdKTtcblxuICBjb25zdCB0dXRvcmlhbENsb2NrUmVmID0gdXNlUmVmPFR1dG9yaWFsQ2xvY2sgfCBudWxsPihudWxsKTtcbiAgaWYgKCF0dXRvcmlhbENsb2NrUmVmLmN1cnJlbnQpIHR1dG9yaWFsQ2xvY2tSZWYuY3VycmVudCA9IGNyZWF0ZVR1dG9yaWFsQ2xvY2soYWN0aXZlVHV0b3JpYWw/LmR1cmF0aW9uTXMgPz8gMCk7XG4gIGNvbnN0IHR1dG9yaWFsQ2xvY2sgPSB0dXRvcmlhbENsb2NrUmVmLmN1cnJlbnQ7XG4gIHVzZUVmZmVjdCgoKSA9PiAoKSA9PiB0dXRvcmlhbENsb2NrUmVmLmN1cnJlbnQ/LmRpc3Bvc2UoKSwgW10pO1xuICB1c2VFZmZlY3QoKCkgPT4ge1xuICAgIHR1dG9yaWFsQ2xvY2suc2V0RHVyYXRpb25NcyhhY3RpdmVUdXRvcmlhbD8uZHVyYXRpb25NcyA/PyAwKTtcbiAgfSwgW2FjdGl2ZVR1dG9yaWFsPy5kdXJhdGlvbk1zLCB0dXRvcmlhbENsb2NrXSk7XG4gIHVzZUVmZmVjdCgoKSA9PiB7XG4gICAgdHV0b3JpYWxDbG9jay5zZXRSYXRlKHR1dG9yaWFsUmF0ZSk7XG4gIH0sIFt0dXRvcmlhbFJhdGUsIHR1dG9yaWFsQ2xvY2tdKTtcbiAgdXNlRWZmZWN0KCgpID0+IHtcbiAgICBpZiAodHV0b3JpYWxQbGF5aW5nKSB0dXRvcmlhbENsb2NrLnBsYXkoKTtcbiAgICBlbHNlIHR1dG9yaWFsQ2xvY2sucGF1c2UoKTtcbiAgfSwgW3R1dG9yaWFsUGxheWluZywgdHV0b3JpYWxDbG9ja10pO1xuXG4gIGNvbnN0IHVpQnJpZGdlQ3R4UmVmID0gdXNlUmVmPFR1dG9yaWFsVWlCcmlkZ2VDb250ZXh0Pih7IHNlc3Npb24sIGFwcExhYmVsc092ZXJsYXksIHRlcm1pbm9sb2d5OiB1aVRlcm1pbm9sb2d5LCBsb2NhbGU6IHVpTG9jYWxlIH0pO1xuICB1aUJyaWRnZUN0eFJlZi5jdXJyZW50ID0geyBzZXNzaW9uLCBhcHBMYWJlbHNPdmVybGF5LCB0ZXJtaW5vbG9neTogdWlUZXJtaW5vbG9neSwgbG9jYWxlOiB1aUxvY2FsZSB9O1xuXG4gIC8qKiDij7HvuI8gUGxheWhlYWQgKG1zKSB0aGUgZGlyZWN0b3Ivc2VlayBsYXN0IGFwcGxpZWQgZG9jdW1lbnQvVUkgdHJhY2tzIHVwIHRvIOKAlCB0aGUgXCJmcm9tXCIgc2lkZSBvZiB0aGVcbiAgICogbmV4dCBgdHV0b3JpYWxTbGljZShkZWYsIGZyb20sIHRvKWAgY2FsbC4gUmVzZXQgdG8gMCBvbiBzYW5kYm94IChyZSlzdGFydC4gKi9cbiAgY29uc3QgdHV0b3JpYWxMYXN0QXBwbGllZE1zUmVmID0gdXNlUmVmKDApO1xuICAvKiog8J+OrO+4jyBTYW5kYm94ZWQtb3V0IGxpdmUgZG9jdW1lbnQgKGZ1bGwgYERvY3VtZW50RW52ZWxvcGVgIEpTT04pLCByZXN0b3JlZCBvbiBzdG9wL2V4aXQuICovXG4gIGNvbnN0IHR1dG9yaWFsRG9jdW1lbnRTbmFwc2hvdFJlZiA9IHVzZVJlZjxzdHJpbmcgfCBudWxsPihudWxsKTtcblxuICAvLyDwn46s77iPIFNhbmRib3ggc3RhcnQvc3RvcCAoZGVzaWduIHBvaW50IDMpOiBvbiBhY3RpdmF0aW9uLCBzbmFwc2hvdCB0aGUgbGl2ZSBkb2N1bWVudCwgbG9hZCBgYmFzZWAsIGFwcGx5XG4gIC8vIGBiYXNlLnVpYC9gYmFzZS5jYW1lcmFzYCwgYW5kIHNlZWsgdGhlIGNsb2NrIHRvIDA7IG9uIGRlYWN0aXZhdGlvbiwgcmVzdG9yZSB0aGUgc25hcHNob3QuXG4gIGNvbnN0IHByZXZBY3RpdmVUdXRvcmlhbElkUmVmID0gdXNlUmVmPHN0cmluZyB8IG51bGw+KG51bGwpO1xuICB1c2VFZmZlY3QoKCkgPT4ge1xuICAgIGNvbnN0IHByZXZpb3VzSWQgPSBwcmV2QWN0aXZlVHV0b3JpYWxJZFJlZi5jdXJyZW50O1xuICAgIHByZXZBY3RpdmVUdXRvcmlhbElkUmVmLmN1cnJlbnQgPSBhY3RpdmVUdXRvcmlhbElkO1xuICAgIGlmIChwcmV2aW91c0lkID09PSBhY3RpdmVUdXRvcmlhbElkIHx8ICFzZXNzaW9uKSByZXR1cm47XG4gICAgY29uc3QgcGx1Z2luID0gbG9hZGVkUGx1Z2lucy5maW5kKChlbnRyeSkgPT4gZW50cnkuaGFuZGxlLnBsdWdpbklkID09PSBzZXNzaW9uLnBsdWdpbklkKT8uaGFuZGxlO1xuICAgIGlmICghcGx1Z2luKSByZXR1cm47XG4gICAgaWYgKGFjdGl2ZVR1dG9yaWFsSWQpIHtcbiAgICAgIGNvbnN0IGRlZiA9IGFjdGl2ZVR1dG9yaWFscy5maW5kKCh0dXRvcmlhbCkgPT4gdHV0b3JpYWwuaWQgPT09IGFjdGl2ZVR1dG9yaWFsSWQpO1xuICAgICAgaWYgKCFkZWYpIHJldHVybjtcbiAgICAgIHR1dG9yaWFsRHJpdmVuUmVmLmN1cnJlbnQgPSB0cnVlO1xuICAgICAgdm9pZCAoYXN5bmMgKCkgPT4ge1xuICAgICAgICB0cnkge1xuICAgICAgICAgIGlmIChwbHVnaW4ucmVhZEFwcERvY3VtZW50KSB0dXRvcmlhbERvY3VtZW50U25hcHNob3RSZWYuY3VycmVudCA9IGF3YWl0IHBsdWdpbi5yZWFkQXBwRG9jdW1lbnQoc2Vzc2lvbi5pbnN0YW5jZUlkKTtcbiAgICAgICAgfSBjYXRjaCAoc25hcHNob3RFcnJvcikge1xuICAgICAgICAgIGNvbnNvbGUuZXJyb3IoXCJbREVCVUddIHR1dG9yaWFsIHNhbmRib3ggc25hcHNob3QgZmFpbGVkXCIsIHNuYXBzaG90RXJyb3IpO1xuICAgICAgICB9XG4gICAgICAgIHRyeSB7XG4gICAgICAgICAgaWYgKGRlZi5iYXNlLmRvY3VtZW50SnNvbiAmJiBwbHVnaW4ubG9hZEFwcERvY3VtZW50KSBhd2FpdCBwbHVnaW4ubG9hZEFwcERvY3VtZW50KHNlc3Npb24uaW5zdGFuY2VJZCwgZGVmLmJhc2UuZG9jdW1lbnRKc29uKTtcbiAgICAgICAgICBlbHNlIGlmIChkZWYuYmFzZS5leGFtcGxlSWQpIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfQUNUSVZFX0VYQU1QTEVfSURcIiwgdmFsdWU6IGRlZi5iYXNlLmV4YW1wbGVJZCB9KTtcbiAgICAgICAgfSBjYXRjaCAobG9hZEVycm9yKSB7XG4gICAgICAgICAgY29uc29sZS5lcnJvcihcIltERUJVR10gdHV0b3JpYWwgYmFzZSBkb2N1bWVudCBsb2FkIGZhaWxlZFwiLCBsb2FkRXJyb3IpO1xuICAgICAgICB9XG4gICAgICAgIGFwcGx5VHV0b3JpYWxVaVNuYXBzaG90VG9TaGVsbChkaXNwYXRjaCwgZGVmLmJhc2UudWksIHVpQnJpZGdlQ3R4UmVmLmN1cnJlbnQpO1xuICAgICAgICBmb3IgKGNvbnN0IGNhbWVyYUtleWZyYW1lIG9mIGRlZi5iYXNlLmNhbWVyYXMpIGdldFR1dG9yaWFsQ2FtZXJhRHJpdmVyKGNhbWVyYUtleWZyYW1lLndpbmRvd0lkKT8uc2V0KGNhbWVyYUtleWZyYW1lLmNhbWVyYSk7XG4gICAgICAgIHR1dG9yaWFsTGFzdEFwcGxpZWRNc1JlZi5jdXJyZW50ID0gMDtcbiAgICAgICAgdHV0b3JpYWxDbG9jay5zZWVrKDApO1xuICAgICAgICBhd2FpdCByZWZyZXNoVWkoc2Vzc2lvbiwgeyBraW5kOiBcImZ1bGxcIiB9KTtcbiAgICAgICAgdHV0b3JpYWxEcml2ZW5SZWYuY3VycmVudCA9IGZhbHNlO1xuICAgICAgfSkoKTtcbiAgICB9IGVsc2UgaWYgKHByZXZpb3VzSWQpIHtcbiAgICAgIHR1dG9yaWFsRHJpdmVuUmVmLmN1cnJlbnQgPSB0cnVlO1xuICAgICAgdm9pZCAoYXN5bmMgKCkgPT4ge1xuICAgICAgICB0cnkge1xuICAgICAgICAgIGNvbnN0IHNuYXBzaG90SnNvbiA9IHR1dG9yaWFsRG9jdW1lbnRTbmFwc2hvdFJlZi5jdXJyZW50O1xuICAgICAgICAgIGlmIChzbmFwc2hvdEpzb24gJiYgcGx1Z2luLmxvYWRBcHBEb2N1bWVudCkgYXdhaXQgcGx1Z2luLmxvYWRBcHBEb2N1bWVudChzZXNzaW9uLmluc3RhbmNlSWQsIHNuYXBzaG90SnNvbik7XG4gICAgICAgIH0gY2F0Y2ggKHJlc3RvcmVFcnJvcikge1xuICAgICAgICAgIGNvbnNvbGUuZXJyb3IoXCJbREVCVUddIHR1dG9yaWFsIHNhbmRib3ggcmVzdG9yZSBmYWlsZWRcIiwgcmVzdG9yZUVycm9yKTtcbiAgICAgICAgfVxuICAgICAgICB0dXRvcmlhbERvY3VtZW50U25hcHNob3RSZWYuY3VycmVudCA9IG51bGw7XG4gICAgICAgIGF3YWl0IHJlZnJlc2hVaShzZXNzaW9uLCB7IGtpbmQ6IFwiZnVsbFwiIH0pO1xuICAgICAgICB0dXRvcmlhbERyaXZlblJlZi5jdXJyZW50ID0gZmFsc2U7XG4gICAgICB9KSgpO1xuICAgIH1cbiAgfSwgW2FjdGl2ZVR1dG9yaWFsSWQsIGFjdGl2ZVR1dG9yaWFscywgc2Vzc2lvbiwgbG9hZGVkUGx1Z2lucywgdHV0b3JpYWxDbG9jaywgcmVmcmVzaFVpXSk7XG5cbiAgLyoqIPCfjqzvuI8gQXBwbGllcyBldmVyeSBlbnRyeSBvZiBvbmUgYFR1dG9yaWFsU2xpY2VgIChhIGRpcmVjdG9yIHRpY2sgb3IgYSBzZWVrIHNwYW4pIG9udG8gdGhlIGxpdmVcbiAgICogc2Vzc2lvbiDigJQgVUkgY2hhbmdlcyBmaXJzdCwgdGhlbiBkb2N1bWVudC10cmFjayBlbnRyaWVzIHRocm91Z2ggdGhlIHBsdWdpbiBicmlkZ2U6IGBFZGl0YCB2aWFcbiAgICogYGFwcGx5T3BlcmF0aW9uc2AgKGZvcndhcmQvYmFja3dhcmQgcGVyIGBzbGljZS5mb3J3YXJkYCksIGBMb2FkYCB2aWEgYGxvYWRBcHBEb2N1bWVudGAsXG4gICAqIGBVbmRvYC9gUmVkb2AvYENoZWNrcG9pbnRgL2BDaGVja291dENoZWNrcG9pbnRgL2BTd2l0Y2hBbHRlcm5hdGl2ZWAgdmlhIHRoZSBTQU1FIEhpc3RvcnktYWN0aW9uXG4gICAqIGBvbkFjdGlvbmAgZnVubmVsIHRoZSBhcHAncyBvd24gdW5kby9yZWRvIGJ1dHRvbnMgZGlzcGF0Y2ggdGhyb3VnaCAobmV2ZXIgYSBiZXNwb2tlIGNoYW5uZWwpIOKAlCB0aGVuXG4gICAqIHB1bHNlcyBhbnkgYW5ub3RhdGlvbmFsIGV2ZW50J3MgdGFyZ2V0IGVsZW1lbnQgdmlhIHRoZSBleGlzdGluZyBgY2VsZWJyYXRlRWxlbWVudHNgIHZvY2FidWxhcnkuICovXG4gIGNvbnN0IGFwcGx5VHV0b3JpYWxTbGljZVRvU2hlbGwgPSB1c2VDYWxsYmFjayhcbiAgICBhc3luYyAoc2xpY2U6IFR1dG9yaWFsU2xpY2UsIGFjdGl2ZVNlc3Npb246IEFjdGl2ZVNlc3Npb24pID0+IHtcbiAgICAgIGZvciAoY29uc3QgY2hhbmdlIG9mIHNsaWNlLnVpQ2hhbmdlcykgYXBwbHlUdXRvcmlhbFVpQ2hhbmdlVG9TaGVsbChkaXNwYXRjaCwgY2hhbmdlLCB1aUJyaWRnZUN0eFJlZi5jdXJyZW50KTtcbiAgICAgIGNvbnN0IHBsdWdpbiA9IGxvYWRlZFBsdWdpbnMuZmluZCgoZW50cnkpID0+IGVudHJ5LmhhbmRsZS5wbHVnaW5JZCA9PT0gYWN0aXZlU2Vzc2lvbi5wbHVnaW5JZCk/LmhhbmRsZTtcbiAgICAgIGxldCBkb2N1bWVudFRvdWNoZWQgPSBmYWxzZTtcbiAgICAgIGZvciAoY29uc3QgZG9jdW1lbnRFdmVudCBvZiBzbGljZS5kb2N1bWVudCkge1xuICAgICAgICBjb25zdCBraW5kOiBUdXRvcmlhbERvY3VtZW50RXZlbnRLaW5kID0gZG9jdW1lbnRFdmVudC5raW5kO1xuICAgICAgICBpZiAoa2luZC5raW5kID09PSBcImVkaXRcIikge1xuICAgICAgICAgIGRvY3VtZW50VG91Y2hlZCA9IHRydWU7XG4gICAgICAgICAgY29uc3Qgb3BlcmF0aW9ucyA9IHNsaWNlLmZvcndhcmQgPyBraW5kLmZvcndhcmRzIDoga2luZC5iYWNrd2FyZHM7XG4gICAgICAgICAgaWYgKHBsdWdpbj8uYXBwbHlPcGVyYXRpb25zKSBhd2FpdCBwbHVnaW4uYXBwbHlPcGVyYXRpb25zKGFjdGl2ZVNlc3Npb24uaW5zdGFuY2VJZCwgZW5jb2RlT3BlcmF0aW9uRW52ZWxvcGVzUGFjayhvcGVyYXRpb25zKSk7XG4gICAgICAgIH0gZWxzZSBpZiAoa2luZC5raW5kID09PSBcImxvYWRcIikge1xuICAgICAgICAgIGRvY3VtZW50VG91Y2hlZCA9IHRydWU7XG4gICAgICAgICAgY29uc3QgZG9jdW1lbnRKc29uID0gc2xpY2UuZm9yd2FyZCA/IGtpbmQuZG9jdW1lbnRKc29uIDoga2luZC5wcmV2aW91c0pzb247XG4gICAgICAgICAgaWYgKHBsdWdpbj8ubG9hZEFwcERvY3VtZW50KSBhd2FpdCBwbHVnaW4ubG9hZEFwcERvY3VtZW50KGFjdGl2ZVNlc3Npb24uaW5zdGFuY2VJZCwgZG9jdW1lbnRKc29uKTtcbiAgICAgICAgfSBlbHNlIGlmIChraW5kLmtpbmQgPT09IFwidW5kb1wiKSB7XG4gICAgICAgICAgb25BY3Rpb25SZWYuY3VycmVudCh7IGNvbnRyb2xsZXJJZDogYWN0aXZlU2Vzc2lvbi5hcHAuY29udHJvbGxlcklkLCBhY3Rpb246IHNsaWNlLmZvcndhcmQgPyBcInVuZG9cIiA6IFwicmVkb1wiIH0pO1xuICAgICAgICB9IGVsc2UgaWYgKGtpbmQua2luZCA9PT0gXCJyZWRvXCIpIHtcbiAgICAgICAgICBvbkFjdGlvblJlZi5jdXJyZW50KHsgY29udHJvbGxlcklkOiBhY3RpdmVTZXNzaW9uLmFwcC5jb250cm9sbGVySWQsIGFjdGlvbjogc2xpY2UuZm9yd2FyZCA/IFwicmVkb1wiIDogXCJ1bmRvXCIgfSk7XG4gICAgICAgIH0gZWxzZSBpZiAoa2luZC5raW5kID09PSBcImNoZWNrcG9pbnRcIikge1xuICAgICAgICAgIGlmIChzbGljZS5mb3J3YXJkKSBvbkFjdGlvblJlZi5jdXJyZW50KHsgY29udHJvbGxlcklkOiBhY3RpdmVTZXNzaW9uLmFwcC5jb250cm9sbGVySWQsIGFjdGlvbjogXCJjb21taXRDaGVja3BvaW50XCIgfSk7XG4gICAgICAgIH0gZWxzZSBpZiAoa2luZC5raW5kID09PSBcImNoZWNrb3V0Q2hlY2twb2ludFwiKSB7XG4gICAgICAgICAgb25BY3Rpb25SZWYuY3VycmVudCh7IGNvbnRyb2xsZXJJZDogYWN0aXZlU2Vzc2lvbi5hcHAuY29udHJvbGxlcklkLCBhY3Rpb246IFwiY2hlY2tvdXRDaGVja3BvaW50XCIsIGFyZ3M6IHsgY2hlY2twb2ludElkOiBraW5kLmNoZWNrcG9pbnRJZCB9IH0pO1xuICAgICAgICB9IGVsc2UgaWYgKGtpbmQua2luZCA9PT0gXCJzd2l0Y2hBbHRlcm5hdGl2ZVwiKSB7XG4gICAgICAgICAgb25BY3Rpb25SZWYuY3VycmVudCh7IGNvbnRyb2xsZXJJZDogYWN0aXZlU2Vzc2lvbi5hcHAuY29udHJvbGxlcklkLCBhY3Rpb246IFwic3dpdGNoQWx0ZXJuYXRpdmVcIiwgYXJnczogeyBhbHRlcm5hdGl2ZUlkOiBraW5kLmFsdGVybmF0aXZlSWQgfSB9KTtcbiAgICAgICAgfVxuICAgICAgfVxuICAgICAgZm9yIChjb25zdCBldmVudCBvZiBzbGljZS5ldmVudHMpIHtcbiAgICAgICAgY29uc3Qga2luZCA9IGV2ZW50LmtpbmQ7XG4gICAgICAgIGNvbnN0IHRhcmdldElkID0ga2luZC5raW5kID09PSBcImFjdGlvblwiID8ga2luZC5hY3Rpb24gOiBraW5kLmtpbmQgPT09IFwiY29tbWFuZFwiID8ga2luZC5jb21tYW5kIDogdW5kZWZpbmVkO1xuICAgICAgICBpZiAodGFyZ2V0SWQgJiYgc2NvcGUucm9vdFJlZi5jdXJyZW50KSBjZWxlYnJhdGVFbGVtZW50cyhlbGVtZW50SWRTZWxlY3Rvcih0YXJnZXRJZCksIENFTEVCUkFURV9TVEFNUF9EVVJBVElPTl9NUywgc2NvcGUucm9vdFJlZi5jdXJyZW50KTtcbiAgICAgIH1cbiAgICAgIGlmIChkb2N1bWVudFRvdWNoZWQpIGF3YWl0IHJlZnJlc2hVaShhY3RpdmVTZXNzaW9uLCB7IGtpbmQ6IFwiZnVsbFwiIH0pO1xuICAgIH0sXG4gICAgW2xvYWRlZFBsdWdpbnMsIHJlZnJlc2hVaV0sXG4gICk7XG5cbiAgLy8g8J+OrO+4jyBEaXJlY3Rvcjogb25lIHN1YnNjcmlwdGlvbiB0byB0aGUgY2xvY2sncyByQUYtZHJpdmVuIHRpY2tzLiBDYW1lcmEgaW50ZXJwb2xhdGlvbiBhcHBsaWVzIGV2ZXJ5XG4gIC8vIHRpY2sgKHNtb290aCk7IFVJL2RvY3VtZW50L2V2ZW50IGFwcGxpY2F0aW9uIHRocm90dGxlcyB0byBgVFVUT1JJQUxfRElSRUNUT1JfVElDS19NU2AuXG4gIHVzZUVmZmVjdCgoKSA9PiB7XG4gICAgY29uc3QgZGVmID0gYWN0aXZlVHV0b3JpYWw7XG4gICAgaWYgKCFkZWYgfHwgIXNlc3Npb24pIHJldHVybjtcbiAgICBsZXQgbGFzdEhlYXZ5VGlja0F0ID0gMDtcbiAgICBjb25zdCBjYW1lcmFXaW5kb3dJZHMgPSBuZXcgU2V0KFsuLi5kZWYuYmFzZS5jYW1lcmFzLCAuLi5kZWYudHJhY2tzLmNhbWVyYV0ubWFwKChrZXlmcmFtZSkgPT4ga2V5ZnJhbWUud2luZG93SWQpKTtcbiAgICBjb25zdCB1bnN1YnNjcmliZSA9IHR1dG9yaWFsQ2xvY2suc3Vic2NyaWJlKCgpID0+IHtcbiAgICAgIGNvbnN0IHQgPSB0dXRvcmlhbENsb2NrLmdldFRpbWVNcygpO1xuICAgICAgZm9yIChjb25zdCB3aW5kb3dJZCBvZiBjYW1lcmFXaW5kb3dJZHMpIHtcbiAgICAgICAgY29uc3QgcG9zZSA9IHR1dG9yaWFsQ2FtZXJhQXQoZGVmLCB3aW5kb3dJZCwgdCk7XG4gICAgICAgIGlmIChwb3NlKSBnZXRUdXRvcmlhbENhbWVyYURyaXZlcih3aW5kb3dJZCk/LnNldChwb3NlKTtcbiAgICAgIH1cbiAgICAgIGlmICghdHV0b3JpYWxDbG9jay5pc1BsYXlpbmcoKSkgcmV0dXJuO1xuICAgICAgY29uc3Qgbm93ID0gcGVyZm9ybWFuY2Uubm93KCk7XG4gICAgICBpZiAobm93IC0gbGFzdEhlYXZ5VGlja0F0IDwgVFVUT1JJQUxfRElSRUNUT1JfVElDS19NUykgcmV0dXJuO1xuICAgICAgbGFzdEhlYXZ5VGlja0F0ID0gbm93O1xuICAgICAgY29uc3QgZnJvbSA9IHR1dG9yaWFsTGFzdEFwcGxpZWRNc1JlZi5jdXJyZW50O1xuICAgICAgaWYgKGZyb20gPT09IHQpIHJldHVybjtcbiAgICAgIGNvbnN0IHNsaWNlID0gdHV0b3JpYWxTbGljZShkZWYsIGZyb20sIHQpO1xuICAgICAgdHV0b3JpYWxMYXN0QXBwbGllZE1zUmVmLmN1cnJlbnQgPSB0O1xuICAgICAgdHV0b3JpYWxEcml2ZW5SZWYuY3VycmVudCA9IHRydWU7XG4gICAgICB2b2lkIGFwcGx5VHV0b3JpYWxTbGljZVRvU2hlbGwoc2xpY2UsIHNlc3Npb24pLmZpbmFsbHkoKCkgPT4ge1xuICAgICAgICB0dXRvcmlhbERyaXZlblJlZi5jdXJyZW50ID0gZmFsc2U7XG4gICAgICB9KTtcbiAgICB9KTtcbiAgICByZXR1cm4gdW5zdWJzY3JpYmU7XG4gIH0sIFthY3RpdmVUdXRvcmlhbCwgc2Vzc2lvbiwgdHV0b3JpYWxDbG9jaywgYXBwbHlUdXRvcmlhbFNsaWNlVG9TaGVsbF0pO1xuXG4gIC8qKiDinILvuI8gU2Vlay9yZWJ1aWxkIChkZXNpZ24gcG9pbnQgNSk6IGNvbXBvc2VzIFVJIHdob2xlc2FsZSAobmV2ZXIgYWNjdW11bGF0ZXMgZGVsdGFzIGFjcm9zcyBhIHNlZWsg4oCUXG4gICAqIG1pcnJvcnMgdGhlIFJ1c3QgYHR1dG9yaWFsX3NsaWNlYCBkb2MgY29tbWVudCdzIG93biB3YXJuaW5nKSwgYXBwbGllcyB0aGUgZm9yd2FyZC9iYWNrd2FyZCBkb2N1bWVudFxuICAgKiBzcGFuIGNyb3NzZWQgc2luY2UgdGhlIGxhc3QgYXBwbGllZCBwbGF5aGVhZCwgc2V0cyBldmVyeSBjYW1lcmEgZXhhY3RseSAobm8gaW50ZXJwb2xhdGlvbiBvbiBhIHNlZWspLFxuICAgKiBhbmQgbW92ZXMgdGhlIGNsb2NrLiAqL1xuICBjb25zdCBzZWVrVHV0b3JpYWwgPSB1c2VDYWxsYmFjayhcbiAgICAobXM6IG51bWJlcikgPT4ge1xuICAgICAgY29uc3QgZGVmID0gYWN0aXZlVHV0b3JpYWw7XG4gICAgICBpZiAoIWRlZiB8fCAhc2Vzc2lvbikgcmV0dXJuO1xuICAgICAgY29uc3QgY2xhbXBlZCA9IE1hdGgubWluKGRlZi5kdXJhdGlvbk1zLCBNYXRoLm1heCgwLCBtcykpO1xuICAgICAgY29uc3QgZnJvbSA9IHR1dG9yaWFsTGFzdEFwcGxpZWRNc1JlZi5jdXJyZW50O1xuICAgICAgdHV0b3JpYWxEcml2ZW5SZWYuY3VycmVudCA9IHRydWU7XG4gICAgICB2b2lkIChhc3luYyAoKSA9PiB7XG4gICAgICAgIGFwcGx5VHV0b3JpYWxVaVNuYXBzaG90VG9TaGVsbChkaXNwYXRjaCwgY29tcG9zZVR1dG9yaWFsVWkoZGVmLCBjbGFtcGVkKSwgdWlCcmlkZ2VDdHhSZWYuY3VycmVudCk7XG4gICAgICAgIGNvbnN0IHBsdWdpbiA9IGxvYWRlZFBsdWdpbnMuZmluZCgoZW50cnkpID0+IGVudHJ5LmhhbmRsZS5wbHVnaW5JZCA9PT0gc2Vzc2lvbi5wbHVnaW5JZCk/LmhhbmRsZTtcbiAgICAgICAgY29uc3Qgc2xpY2UgPSB0dXRvcmlhbFNsaWNlKGRlZiwgZnJvbSwgY2xhbXBlZCk7XG4gICAgICAgIGxldCBkb2N1bWVudFRvdWNoZWQgPSBmYWxzZTtcbiAgICAgICAgZm9yIChjb25zdCBkb2N1bWVudEV2ZW50IG9mIHNsaWNlLmRvY3VtZW50KSB7XG4gICAgICAgICAgY29uc3Qga2luZDogVHV0b3JpYWxEb2N1bWVudEV2ZW50S2luZCA9IGRvY3VtZW50RXZlbnQua2luZDtcbiAgICAgICAgICBpZiAoa2luZC5raW5kID09PSBcImVkaXRcIikge1xuICAgICAgICAgICAgZG9jdW1lbnRUb3VjaGVkID0gdHJ1ZTtcbiAgICAgICAgICAgIGNvbnN0IG9wZXJhdGlvbnMgPSBzbGljZS5mb3J3YXJkID8ga2luZC5mb3J3YXJkcyA6IGtpbmQuYmFja3dhcmRzO1xuICAgICAgICAgICAgaWYgKHBsdWdpbj8uYXBwbHlPcGVyYXRpb25zKSBhd2FpdCBwbHVnaW4uYXBwbHlPcGVyYXRpb25zKHNlc3Npb24uaW5zdGFuY2VJZCwgZW5jb2RlT3BlcmF0aW9uRW52ZWxvcGVzUGFjayhvcGVyYXRpb25zKSk7XG4gICAgICAgICAgfSBlbHNlIGlmIChraW5kLmtpbmQgPT09IFwibG9hZFwiKSB7XG4gICAgICAgICAgICBkb2N1bWVudFRvdWNoZWQgPSB0cnVlO1xuICAgICAgICAgICAgY29uc3QgZG9jdW1lbnRKc29uID0gc2xpY2UuZm9yd2FyZCA/IGtpbmQuZG9jdW1lbnRKc29uIDoga2luZC5wcmV2aW91c0pzb247XG4gICAgICAgICAgICBpZiAocGx1Z2luPy5sb2FkQXBwRG9jdW1lbnQpIGF3YWl0IHBsdWdpbi5sb2FkQXBwRG9jdW1lbnQoc2Vzc2lvbi5pbnN0YW5jZUlkLCBkb2N1bWVudEpzb24pO1xuICAgICAgICAgIH1cbiAgICAgICAgICAvLyDwn5qn77iPIFVuZG8vUmVkby9DaGVja3BvaW50L0NoZWNrb3V0Q2hlY2twb2ludC9Td2l0Y2hBbHRlcm5hdGl2ZSBjcm9zc2luZ3MgbWlkLXNlZWsgYXJlIGFuIGhvbmVzdFxuICAgICAgICAgIC8vIHNjb3BlIGN1dCBoZXJlIChyZXBsYXlpbmcgYSBjcm9zc2VkIGhpc3Rvcnkgb3Agb3V0IG9mIGl0cyBuYXR1cmFsIGxpdmUtZGlzcGF0Y2ggb3JkZXIgaXNcbiAgICAgICAgICAvLyBhbWJpZ3VvdXMgd2l0aG91dCBtb3JlIFZDUy1zaWRlIGluZnJhc3RydWN0dXJlKSDigJQgdGhlIGRpcmVjdG9yJ3MgcGVyLXRpY2sgZm9yd2FyZCBwbGF5YmFja1xuICAgICAgICAgIC8vIGFib3ZlIHN0aWxsIGFwcGxpZXMgdGhlbSBjb3JyZWN0bHk7IG9ubHkgYSBsYXJnZSBzY3J1YiBqdW1waW5nIE9WRVIgb25lIG9mIHRoZXNlIGVudHJpZXMgbWlzc2VzIGl0LlxuICAgICAgICB9XG4gICAgICAgIGNvbnN0IGNhbWVyYVdpbmRvd0lkcyA9IG5ldyBTZXQoWy4uLmRlZi5iYXNlLmNhbWVyYXMsIC4uLmRlZi50cmFja3MuY2FtZXJhXS5tYXAoKGtleWZyYW1lKSA9PiBrZXlmcmFtZS53aW5kb3dJZCkpO1xuICAgICAgICBmb3IgKGNvbnN0IHdpbmRvd0lkIG9mIGNhbWVyYVdpbmRvd0lkcykge1xuICAgICAgICAgIGNvbnN0IHBvc2UgPSB0dXRvcmlhbENhbWVyYUF0KGRlZiwgd2luZG93SWQsIGNsYW1wZWQpO1xuICAgICAgICAgIGlmIChwb3NlKSBnZXRUdXRvcmlhbENhbWVyYURyaXZlcih3aW5kb3dJZCk/LnNldChwb3NlKTtcbiAgICAgICAgfVxuICAgICAgICB0dXRvcmlhbExhc3RBcHBsaWVkTXNSZWYuY3VycmVudCA9IGNsYW1wZWQ7XG4gICAgICAgIHR1dG9yaWFsQ2xvY2suc2VlayhjbGFtcGVkKTtcbiAgICAgICAgaWYgKGRvY3VtZW50VG91Y2hlZCkgYXdhaXQgcmVmcmVzaFVpKHNlc3Npb24sIHsga2luZDogXCJmdWxsXCIgfSk7XG4gICAgICAgIGNvbnNvbGUubG9nKFwiW0RFQlVHXSB0dXRvcmlhbCByZWJ1aWxkXCIsIHsgYXRNczogY2xhbXBlZCB9KTtcbiAgICAgICAgdHV0b3JpYWxEcml2ZW5SZWYuY3VycmVudCA9IGZhbHNlO1xuICAgICAgfSkoKTtcbiAgICB9LFxuICAgIFthY3RpdmVUdXRvcmlhbCwgc2Vzc2lvbiwgbG9hZGVkUGx1Z2lucywgdHV0b3JpYWxDbG9jaywgcmVmcmVzaFVpXSxcbiAgKTtcblxuICAvKiog4pa277iPIFBsYXkvcGF1c2UgdG9nZ2xlIOKAlCB0aGUgZGV2aWF0aW9uLWNvbnZlcmdlIHBhdGggKGRlc2lnbiBwb2ludCA2KTogc25hcHMgZG9jdW1lbnQrVUkgdG8gdGhlXG4gICAqIGNvbXBvc2VkIHRhcmdldCBhdCB0aGUgY3VycmVudCBwbGF5aGVhZCwgdHdlZW5zIHRoZSBjYW1lcmEgb3ZlciBgVFVUT1JJQUxfQ09OVkVSR0VfTVNgIChyZWFsLXRpbWUsXG4gICAqIHJhdGUtaW5kZXBlbmRlbnQpIGZyb20gZWFjaCB3aW5kb3cncyBMSVZFIHBvc2UgdG8gaXRzIHRhcmdldCBwb3NlLCB0aGVuIHJlc3VtZXMgdGhlIGNsb2NrLiAqL1xuICBjb25zdCBwbGF5UGF1c2VUdXRvcmlhbCA9IHVzZUNhbGxiYWNrKCgpID0+IHtcbiAgICBpZiAoIWFjdGl2ZVR1dG9yaWFsKSByZXR1cm47XG4gICAgaWYgKHR1dG9yaWFsUGxheWluZykge1xuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9UVVRPUklBTF9QTEFZSU5HXCIsIHZhbHVlOiBmYWxzZSB9KTtcbiAgICAgIHJldHVybjtcbiAgICB9XG4gICAgaWYgKHR1dG9yaWFsRGV2aWF0ZWQgJiYgc2Vzc2lvbikge1xuICAgICAgY29uc3QgZGVmID0gYWN0aXZlVHV0b3JpYWw7XG4gICAgICBjb25zdCBhdE1zID0gdHV0b3JpYWxDbG9jay5nZXRUaW1lTXMoKTtcbiAgICAgIHR1dG9yaWFsRHJpdmVuUmVmLmN1cnJlbnQgPSB0cnVlO1xuICAgICAgYXBwbHlUdXRvcmlhbFVpU25hcHNob3RUb1NoZWxsKGRpc3BhdGNoLCBjb21wb3NlVHV0b3JpYWxVaShkZWYsIGF0TXMpLCB1aUJyaWRnZUN0eFJlZi5jdXJyZW50KTtcbiAgICAgIGNvbnN0IGNhbWVyYVdpbmRvd0lkcyA9IG5ldyBTZXQoWy4uLmRlZi5iYXNlLmNhbWVyYXMsIC4uLmRlZi50cmFja3MuY2FtZXJhXS5tYXAoKGtleWZyYW1lKSA9PiBrZXlmcmFtZS53aW5kb3dJZCkpO1xuICAgICAgY29uc3Qgc3RhcnRQb3NlQnlXaW5kb3cgPSBuZXcgTWFwPHN0cmluZywgVHV0b3JpYWxDYW1lcmFTdGF0ZT4oKTtcbiAgICAgIGZvciAoY29uc3Qgd2luZG93SWQgb2YgY2FtZXJhV2luZG93SWRzKSB7XG4gICAgICAgIGNvbnN0IGxpdmUgPSBnZXRUdXRvcmlhbENhbWVyYURyaXZlcih3aW5kb3dJZCk/LmdldCgpO1xuICAgICAgICBpZiAobGl2ZSkgc3RhcnRQb3NlQnlXaW5kb3cuc2V0KHdpbmRvd0lkLCBsaXZlKTtcbiAgICAgIH1cbiAgICAgIGNvbnN0IHN0YXJ0ZWRBdCA9IHBlcmZvcm1hbmNlLm5vdygpO1xuICAgICAgY29uc3QgdHdlZW4gPSAobm93OiBudW1iZXIpID0+IHtcbiAgICAgICAgY29uc3QgcHJvZ3Jlc3MgPSBNYXRoLm1pbigxLCAobm93IC0gc3RhcnRlZEF0KSAvIFRVVE9SSUFMX0NPTlZFUkdFX01TKTtcbiAgICAgICAgZm9yIChjb25zdCB3aW5kb3dJZCBvZiBjYW1lcmFXaW5kb3dJZHMpIHtcbiAgICAgICAgICBjb25zdCB0YXJnZXRQb3NlID0gdHV0b3JpYWxDYW1lcmFBdChkZWYsIHdpbmRvd0lkLCBhdE1zKTtcbiAgICAgICAgICBpZiAoIXRhcmdldFBvc2UpIGNvbnRpbnVlO1xuICAgICAgICAgIGNvbnN0IGRyaXZlciA9IGdldFR1dG9yaWFsQ2FtZXJhRHJpdmVyKHdpbmRvd0lkKTtcbiAgICAgICAgICBpZiAoIWRyaXZlcikgY29udGludWU7XG4gICAgICAgICAgY29uc3Qgc3RhcnRQb3NlID0gc3RhcnRQb3NlQnlXaW5kb3cuZ2V0KHdpbmRvd0lkKTtcbiAgICAgICAgICBpZiAoc3RhcnRQb3NlICYmIHN0YXJ0UG9zZS5raW5kID09PSB0YXJnZXRQb3NlLmtpbmQpIHtcbiAgICAgICAgICAgIGRyaXZlci5zZXQoaW50ZXJwb2xhdGVUdXRvcmlhbENhbWVyYSh7IGF0OiAwLCB3aW5kb3dJZCwgY2FtZXJhOiBzdGFydFBvc2UsIGVhc2luZzogXCJsaW5lYXJcIiB9LCB7IGF0OiBUVVRPUklBTF9DT05WRVJHRV9NUywgd2luZG93SWQsIGNhbWVyYTogdGFyZ2V0UG9zZSwgZWFzaW5nOiBcImxpbmVhclwiIH0sIHByb2dyZXNzICogVFVUT1JJQUxfQ09OVkVSR0VfTVMpKTtcbiAgICAgICAgICB9IGVsc2Uge1xuICAgICAgICAgICAgZHJpdmVyLnNldCh0YXJnZXRQb3NlKTtcbiAgICAgICAgICB9XG4gICAgICAgIH1cbiAgICAgICAgaWYgKHByb2dyZXNzIDwgMSkgcmVxdWVzdEFuaW1hdGlvbkZyYW1lKHR3ZWVuKTtcbiAgICAgICAgZWxzZSB7XG4gICAgICAgICAgdHV0b3JpYWxEcml2ZW5SZWYuY3VycmVudCA9IGZhbHNlO1xuICAgICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfVFVUT1JJQUxfREVWSUFURURcIiwgdmFsdWU6IGZhbHNlIH0pO1xuICAgICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfVFVUT1JJQUxfUExBWUlOR1wiLCB2YWx1ZTogdHJ1ZSB9KTtcbiAgICAgICAgfVxuICAgICAgfTtcbiAgICAgIHJlcXVlc3RBbmltYXRpb25GcmFtZSh0d2Vlbik7XG4gICAgICByZXR1cm47XG4gICAgfVxuICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfVFVUT1JJQUxfUExBWUlOR1wiLCB2YWx1ZTogdHJ1ZSB9KTtcbiAgfSwgW2FjdGl2ZVR1dG9yaWFsLCB0dXRvcmlhbFBsYXlpbmcsIHR1dG9yaWFsRGV2aWF0ZWQsIHNlc3Npb24sIHR1dG9yaWFsQ2xvY2tdKTtcblxuICBjb25zdCBzdGFydFR1dG9yaWFsID0gdXNlQ2FsbGJhY2soXG4gICAgKHR1dG9yaWFsSWQ6IHN0cmluZykgPT4ge1xuICAgICAgaWYgKCFhY3RpdmVUdXRvcmlhbHMuc29tZSgodHV0b3JpYWwpID0+IHR1dG9yaWFsLmlkID09PSB0dXRvcmlhbElkKSkgcmV0dXJuO1xuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9UVVRPUklBTFwiLCB2YWx1ZTogdHV0b3JpYWxJZCB9KTtcbiAgICB9LFxuICAgIFthY3RpdmVUdXRvcmlhbHNdLFxuICApO1xuICBjb25zdCBzdG9wVHV0b3JpYWwgPSB1c2VDYWxsYmFjaygoKSA9PiB7XG4gICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9UVVRPUklBTFwiLCB2YWx1ZTogbnVsbCB9KTtcbiAgfSwgW10pO1xuXG4gIC8qKiDij7rvuI8gQXJtcy9kaXNhcm1zIGBUdXRvcmlhbFJlY29yZGVyYCBhZ2FpbnN0IHRoZSBMSVZFIChuZXZlciBzYW5kYm94ZWQpIGRvY3VtZW50IOKAlCBhIHJlY29yZGluZyBJUyB0aGVcbiAgICogdXNlcidzIHdvcmsuIE9uIHN0b3A6IGxpZ2h0IGB2YWxpZGF0ZVR1dG9yaWFsYCBzYW5pdHkgY2hlY2ssIHRoZW4gc2VyaWFsaXplICsgdHJpZ2dlciBhIGJyb3dzZXJcbiAgICogZG93bmxvYWQsIG1hdGNoaW5nIHRoZSByZXBvJ3MgZXhpc3RpbmcgbWVkaWEtZXhwb3J0IGRvd25sb2FkIHBhdHRlcm4uICovXG4gIGNvbnN0IHRvZ2dsZVR1dG9yaWFsUmVjb3JkaW5nID0gdXNlQ2FsbGJhY2soKCkgPT4ge1xuICAgIGlmICghc2Vzc2lvbikgcmV0dXJuO1xuICAgIGNvbnN0IHJlY29yZGVyID0gdHV0b3JpYWxSZWNvcmRlclJlZi5jdXJyZW50O1xuICAgIGlmIChyZWNvcmRlcikge1xuICAgICAgdHV0b3JpYWxSZWNvcmRlclJlZi5jdXJyZW50ID0gbnVsbDtcbiAgICAgIGNvbnN0IGlkID0gYHJlY29yZGVkLSR7c2Vzc2lvbi5hcHAuaWR9LSR7RGF0ZS5ub3coKX1gO1xuICAgICAgY29uc3QgZGVmID0gcmVjb3JkZXIuYnVpbGQoaWQsIGAke3Nlc3Npb24uYXBwLmlkfSByZWNvcmRpbmdgKTtcbiAgICAgIGNvbnN0IHZhbGlkYXRpb25FcnJvciA9IHZhbGlkYXRlVHV0b3JpYWwoZGVmKTtcbiAgICAgIGlmICh2YWxpZGF0aW9uRXJyb3IpIGNvbnNvbGUuZXJyb3IoXCJbREVCVUddIHR1dG9yaWFsIHJlY29yZGluZyB2YWxpZGF0aW9uIGZhaWxlZFwiLCB2YWxpZGF0aW9uRXJyb3IpO1xuICAgICAgY29uc3QganNvbiA9IEpTT04uc3RyaW5naWZ5KGRlZiwgbnVsbCwgMik7XG4gICAgICBjb25zb2xlLmxvZyhcIltERUJVR10gdHV0b3JpYWwgcmVjb3JkaW5nXCIsIGpzb24pO1xuICAgICAgZG93bmxvYWRNZWRpYUV4cG9ydChgdHV0b3JpYWwtJHtzZXNzaW9uLmFwcC5pZH0tJHtEYXRlLm5vdygpfS5vcHNgLCBcInRleHQvcGxhaW5cIiwganNvbik7XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1RVVE9SSUFMX1JFQ09SRElOR1wiLCB2YWx1ZTogZmFsc2UgfSk7XG4gICAgICByZXR1cm47XG4gICAgfVxuICAgIHZvaWQgKGFzeW5jICgpID0+IHtcbiAgICAgIGNvbnN0IHBsdWdpbiA9IGxvYWRlZFBsdWdpbnMuZmluZCgoZW50cnkpID0+IGVudHJ5LmhhbmRsZS5wbHVnaW5JZCA9PT0gc2Vzc2lvbi5wbHVnaW5JZCk/LmhhbmRsZTtcbiAgICAgIGxldCBkb2N1bWVudEpzb246IHN0cmluZyB8IG51bGwgPSBudWxsO1xuICAgICAgdHJ5IHtcbiAgICAgICAgaWYgKHBsdWdpbj8ucmVhZEFwcERvY3VtZW50KSBkb2N1bWVudEpzb24gPSBhd2FpdCBwbHVnaW4ucmVhZEFwcERvY3VtZW50KHNlc3Npb24uaW5zdGFuY2VJZCk7XG4gICAgICB9IGNhdGNoIChjYXB0dXJlRXJyb3IpIHtcbiAgICAgICAgY29uc29sZS5lcnJvcihcIltERUJVR10gdHV0b3JpYWwgcmVjb3JkZXIgYmFzZSBjYXB0dXJlIGZhaWxlZFwiLCBjYXB0dXJlRXJyb3IpO1xuICAgICAgfVxuICAgICAgdHV0b3JpYWxSZWNvcmRlclJlZi5jdXJyZW50ID0gbmV3IFR1dG9yaWFsUmVjb3JkZXIoY2FwdHVyZVR1dG9yaWFsVWlTbmFwc2hvdChzaGVsbFN0YXRlUmVmLmN1cnJlbnQsIHNlc3Npb24pLCBkb2N1bWVudEpzb24pO1xuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9UVVRPUklBTF9SRUNPUkRJTkdcIiwgdmFsdWU6IHRydWUgfSk7XG4gICAgfSkoKTtcbiAgfSwgW3Nlc3Npb24sIGxvYWRlZFBsdWdpbnNdKTtcblxuICB1c2VFZmZlY3QoKCkgPT4ge1xuICAgIHN0YXJ0VHV0b3JpYWxSZWYuY3VycmVudCA9IHN0YXJ0VHV0b3JpYWw7XG4gICAgc3RvcFR1dG9yaWFsUmVmLmN1cnJlbnQgPSBzdG9wVHV0b3JpYWw7XG4gICAgdG9nZ2xlVHV0b3JpYWxSZWNvcmRpbmdSZWYuY3VycmVudCA9IHRvZ2dsZVR1dG9yaWFsUmVjb3JkaW5nO1xuICB9LCBbc3RhcnRUdXRvcmlhbCwgc3RvcFR1dG9yaWFsLCB0b2dnbGVUdXRvcmlhbFJlY29yZGluZ10pO1xuXG4gIC8vIOKPuu+4jyBSZWNvcmRlcjogVUktc3RhdGUgZGlmZiBvbiBldmVyeSBgU2hlbGxTdGF0ZWAgY2hhbmdlIChjYXRjaGVzIHBhbmVsLXRhYiBjbGlja3MvdHJlZSBleHBhbmRzL2V0Yy5cbiAgLy8gdGhhdCBieXBhc3MgYG9uQWN0aW9uYCksIGEgcGVyaW9kaWMgZnVsbC1zbmFwc2hvdCBrZXlmcmFtZSBldmVyeSA1cywgYW5kIGEgMTBIeiBlcHNpbG9uLWZpbHRlcmVkXG4gIC8vIGNhbWVyYSBzYW1wbGVyIHBlciByZWdpc3RlcmVkIGRyaXZlciAod29ybGQgZHJhZ3MgYnlwYXNzIGBvbkFjdGlvbmAgZW50aXJlbHkpLlxuICB1c2VFZmZlY3QoKCkgPT4ge1xuICAgIGlmICghdHV0b3JpYWxSZWNvcmRpbmcpIHJldHVybjtcbiAgICB0dXRvcmlhbFJlY29yZGVyUmVmLmN1cnJlbnQ/LnJlY29yZFVpRGlmZihjYXB0dXJlVHV0b3JpYWxVaVNuYXBzaG90KHNoZWxsU3RhdGUsIHNlc3Npb24pKTtcbiAgfSwgW3R1dG9yaWFsUmVjb3JkaW5nLCBzaGVsbFN0YXRlLCBzZXNzaW9uXSk7XG5cbiAgdXNlRWZmZWN0KCgpID0+IHtcbiAgICBpZiAoIXR1dG9yaWFsUmVjb3JkaW5nIHx8ICFzZXNzaW9uIHx8IHR5cGVvZiB3aW5kb3cgPT09IFwidW5kZWZpbmVkXCIpIHJldHVybjtcbiAgICBjb25zdCBpbnRlcnZhbCA9IHdpbmRvdy5zZXRJbnRlcnZhbCgoKSA9PiB7XG4gICAgICB0dXRvcmlhbFJlY29yZGVyUmVmLmN1cnJlbnQ/LnJlY29yZFNuYXBzaG90KGNhcHR1cmVUdXRvcmlhbFVpU25hcHNob3Qoc2hlbGxTdGF0ZVJlZi5jdXJyZW50LCBzZXNzaW9uKSk7XG4gICAgfSwgNTAwMCk7XG4gICAgcmV0dXJuICgpID0+IHdpbmRvdy5jbGVhckludGVydmFsKGludGVydmFsKTtcbiAgfSwgW3R1dG9yaWFsUmVjb3JkaW5nLCBzZXNzaW9uXSk7XG5cbiAgdXNlRWZmZWN0KCgpID0+IHtcbiAgICBpZiAoIXR1dG9yaWFsUmVjb3JkaW5nIHx8ICFzZXNzaW9uIHx8IHR5cGVvZiB3aW5kb3cgPT09IFwidW5kZWZpbmVkXCIpIHJldHVybjtcbiAgICBjb25zdCBpbnRlcnZhbCA9IHdpbmRvdy5zZXRJbnRlcnZhbCgoKSA9PiB7XG4gICAgICBjb25zdCByZWNvcmRlciA9IHR1dG9yaWFsUmVjb3JkZXJSZWYuY3VycmVudDtcbiAgICAgIGlmICghcmVjb3JkZXIpIHJldHVybjtcbiAgICAgIGZvciAoY29uc3QgaW5zdGFuY2Ugb2Ygc2Vzc2lvbldpbmRvd0luc3RhbmNlcyhzZXNzaW9uLmFwcCwgZXh0cmFXaW5kb3dJbnN0YW5jZXNSZWYuY3VycmVudCkpIHtcbiAgICAgICAgY29uc3QgcG9zZSA9IGdldFR1dG9yaWFsQ2FtZXJhRHJpdmVyKGluc3RhbmNlLmlkKT8uZ2V0KCk7XG4gICAgICAgIGlmIChwb3NlKSByZWNvcmRlci5zYW1wbGVDYW1lcmEoaW5zdGFuY2UuaWQsIHBvc2UpO1xuICAgICAgfVxuICAgIH0sIDEwMCk7XG4gICAgcmV0dXJuICgpID0+IHdpbmRvdy5jbGVhckludGVydmFsKGludGVydmFsKTtcbiAgfSwgW3R1dG9yaWFsUmVjb3JkaW5nLCBzZXNzaW9uXSk7XG5cbiAgY29uc3QgYWRkVHV0b3JpYWxDaGFwdGVyID0gdXNlQ2FsbGJhY2soKCkgPT4ge1xuICAgIHR1dG9yaWFsUmVjb3JkZXJSZWYuY3VycmVudD8uYWRkQ2hhcHRlcigpO1xuICB9LCBbXSk7XG5cbiAgY29uc3QgdHV0b3JpYWxDaGFwdGVyTWFya2VycyA9IHVzZU1lbW8oXG4gICAgKCk6IHJlYWRvbmx5IFR1dG9yaWFsQ2hhcHRlck1hcmtlcltdID0+IChhY3RpdmVUdXRvcmlhbCA/IGFjdGl2ZVR1dG9yaWFsLmNoYXB0ZXJzLm1hcCgoY2hhcHRlcikgPT4gKHsgaWQ6IGNoYXB0ZXIuaWQsIHRpdGxlOiByZXNvbHZlTWFuaWZlc3RMYWJlbChjaGFwdGVyLnRpdGxlLCB1aVRlcm1pbm9sb2d5LCB1aUxvY2FsZSksIGF0TXM6IGNoYXB0ZXIuYXQgfSkpIDogW10pLFxuICAgIFthY3RpdmVUdXRvcmlhbCwgdWlUZXJtaW5vbG9neSwgdWlMb2NhbGVdLFxuICApO1xuICAvLyNlbmRyZWdpb24g8J+Ope+4j1R1dG9yaWFsT3JjaGVzdHJhdGlvblxuXG4gIGNvbnN0IHN0dWRpb1Nlc3Npb25BY3RpdmUgPSBzdHVkaW9Nb2RlICYmIHNlc3Npb24/LmFwcC5pZCA9PT0gaG9zdEFwcElkO1xuICAvLyDwn4+g77iP8J+ns++4jyBPbmNlIGBzdHVkaW9TZXNzaW9uQWN0aXZlYCBpcyB0cnVlLCBgc2Vzc2lvbi5hcHBgICppcyogdGhlIGhvc3QgYXBwLCBzbyBpdHMgb3duIHNlbGYtZGVjbGFyZWRcbiAgLy8gYGNvbnRyb2xsZXJJZGAgaXMgdGhlIHJpZ2h0IHZhbHVlIOKAlCBubyBzZXBhcmF0ZSBhcHAtaWRlbnRpdHkgbG9va3VwIG5lZWRlZC5cbiAgY29uc3Qgc3R1ZGlvU2Vzc2lvbkNvbnRyb2xsZXJJZCA9IHN0dWRpb1Nlc3Npb25BY3RpdmUgPyBzZXNzaW9uPy5hcHAuY29udHJvbGxlcklkIDogdW5kZWZpbmVkO1xuICB1c2VFZmZlY3QoKCkgPT4ge1xuICAgIGlmICghc3R1ZGlvU2Vzc2lvbkFjdGl2ZSB8fCAhc3R1ZGlvU2Vzc2lvbkNvbnRyb2xsZXJJZCB8fCB0eXBlb2Ygd2luZG93ID09PSBcInVuZGVmaW5lZFwiKSByZXR1cm47XG4gICAgY29uc3QgaWRlbnRpdHkgPSBwcmVzZW5jZUNsaWVudElkZW50aXR5KGVwaGVtZXJhbCk7XG4gICAgY29uc3QgYmVhdCA9ICgpID0+IG9uQWN0aW9uUmVmLmN1cnJlbnQoeyBjb250cm9sbGVySWQ6IHN0dWRpb1Nlc3Npb25Db250cm9sbGVySWQsIGFjdGlvbjogXCJwcmVzZW5jZUhlYXJ0YmVhdFwiLCBhcmdzOiBpZGVudGl0eSB9KTtcbiAgICBjb25zdCBpbml0aWFsID0gd2luZG93LnNldFRpbWVvdXQoYmVhdCwgMTAwMCk7XG4gICAgY29uc3QgdGltZXIgPSB3aW5kb3cuc2V0SW50ZXJ2YWwoYmVhdCwgUFJFU0VOQ0VfSEVBUlRCRUFUX0lOVEVSVkFMX01TKTtcbiAgICByZXR1cm4gKCkgPT4ge1xuICAgICAgd2luZG93LmNsZWFyVGltZW91dChpbml0aWFsKTtcbiAgICAgIHdpbmRvdy5jbGVhckludGVydmFsKHRpbWVyKTtcbiAgICB9O1xuICB9LCBbc3R1ZGlvU2Vzc2lvbkFjdGl2ZSwgc3R1ZGlvU2Vzc2lvbkNvbnRyb2xsZXJJZCwgZXBoZW1lcmFsXSk7XG5cbiAgdXNlUGFuZWxDaHJvbWVIb3RrZXlzKHtcbiAgICAvLyDwn5Ox77iPIEFsbCBlaWdodCBhbmNob3IgaG90a2V5cyBjb2xsYXBzZSBvbnRvIHRoZSBzaW5nbGUgbW9iaWxlIHBhbmVsIHRvZ2dsZSBvbiBtb2JpbGUuIFNhbWUgYHNoZWxsLnBhbmVsVG9nZ2xlYFxuICAgIC8vIGNvbW1hbmRJZCBhcyB0aGUgbW91c2UtZHJpdmVuIHRvZ2dsZSBpbiBgYnVpbGRQYW5lbFNlbGVjdGlvblByb3BzYCAoc28ga2V5Ym9hcmQvbW91c2UgZm9sZCB0b2dldGhlciksXG4gICAgLy8gZmxhZ2dlZCBgaG90a2V5OiB0cnVlYCBpbiBkZXRhaWwuXG4gICAgb25Ub2dnbGU6IChhbmNob3IpID0+IHtcbiAgICAgIGlmIChtb2JpbGUpIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfTU9CSUxFX1BBTkVMX1ZJU0lCTEVcIiwgdmFsdWU6ICh2aXNpYmxlKSA9PiAhdmlzaWJsZSB9KTtcbiAgICAgIGVsc2UgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9QQU5FTF9WSVNJQkxFXCIsIGFuY2hvciwgdmFsdWU6ICh2aXNpYmxlKSA9PiAhdmlzaWJsZSB9KTtcbiAgICAgIG5vdGVTaGVsbENvbW1hbmQoXCJzaGVsbC5wYW5lbFRvZ2dsZVwiLCBzaGVsbExhYmVsKFwidWkuc2hlbGxDb21tYW5kLnBhbmVsVG9nZ2xlXCIpLCB7IGFuY2hvcjogbW9iaWxlID8gdW5kZWZpbmVkIDogYW5jaG9yLCBob3RrZXk6IHRydWUgfSk7XG4gICAgfSxcbiAgfSk7XG5cbiAgdXNlRWxlbWVudHNTdXJmYWNlQ2hyb21lKHsgYXBwZWFyYW5jZTogdWlBcHBlYXJhbmNlLCBkZXZpY2U6IHVpRGV2aWNlLCBkcml2ZXI6IHVpRHJpdmVyIH0sIHNjb3BlLnJvb3RSZWYuY3VycmVudCA/PyB1bmRlZmluZWQpO1xuXG4gIC8vI3JlZ2lvbiDwn5K+77iPIHVpUHJlZnMgcGVyc2lzdGVuY2UgKHNraXBzIHdyaXRlcyBmb3IgYW55IGxvY2tlZCBwcmVmZXJlbmNlOyBhbiBlcGhlbWVyYWwgYnJhbmQnc1xuICAvLyBgc2NvcGUuc3RvcmFnZWAgaXMgYWxyZWFkeSBhbiBpbi1tZW1vcnkgcG9ydCwgc28gdGhlIHdyaXRlcyBiZWxvdyBhcmUgaGFybWxlc3MgdGhlcmUgdG9vIOKAlCBubyBtb3JlXG4gIC8vIGBlcGhlbWVyYWxgIGJyYW5jaCBuZWVkZWQgdG8gc2tpcCB0aGVtIG91dHJpZ2h0KVxuICB1c2VFZmZlY3QoKCkgPT4ge1xuICAgIGlmICghbG9ja3MuYXBwZWFyYW5jZSkgd3JpdGVTdG9yZWRVaUNocm9tZUFwcGVhcmFuY2Uoc2NvcGUuc3RvcmFnZSwgdWlBcHBlYXJhbmNlKTtcbiAgICB3cml0ZVN0b3JlZFVpQ2hyb21lTGF5b3V0KHNjb3BlLnN0b3JhZ2UsIHVpTGF5b3V0KTtcbiAgICB3cml0ZVN0b3JlZFVpRHJpdmVySWQoc2NvcGUuc3RvcmFnZSwgdWlEcml2ZXJJZCk7XG4gICAgd3JpdGVTdG9yZWRVaUN1c3RvbURyaXZlcnMoc2NvcGUuc3RvcmFnZSwgdWlDdXN0b21Ecml2ZXJzKTtcbiAgICB3cml0ZVN0b3JlZFVpS2V5YmluZGluZ092ZXJyaWRlcyhzY29wZS5zdG9yYWdlLCB1aUtleWJpbmRpbmdPdmVycmlkZXMpO1xuICAgIGlmICghbG9ja3MubG9jYWxlKSB3cml0ZVN0b3JlZFVpQ2hyb21lTG9jYWxlKHNjb3BlLnN0b3JhZ2UsIHVpTG9jYWxlKTtcbiAgICAvLyDwn5Ca77iPIFRoaXMgc2hlbGwncyBvd24gaTE4bmV4dCBpbnN0YW5jZSAobm90IHRoZSBzaGFyZWQgYHVpSTE4bmAgc2luZ2xldG9uKSDigJQgYW5kIGl0cyBvd24gcm9vdCdzXG4gICAgLy8gYGxhbmdgIGF0dHJpYnV0ZTsgYGRvY3VtZW50LmRvY3VtZW50RWxlbWVudC5sYW5nYCBzdGF5cyByZXNlcnZlZCBmb3IgdGhlIHBhZ2Utb3duaW5nIGNhc2UuXG4gICAgdm9pZCBzY29wZS5pMThuLmNoYW5nZUxhbmd1YWdlKHVpTG9jYWxlKTtcbiAgICBpZiAoc2NvcGUub3duc1BhZ2UpIHtcbiAgICAgIGlmICh0eXBlb2YgZG9jdW1lbnQgIT09IFwidW5kZWZpbmVkXCIpIGRvY3VtZW50LmRvY3VtZW50RWxlbWVudC5sYW5nID0gdWlMb2NhbGU7XG4gICAgfSBlbHNlIGlmIChzY29wZS5yb290UmVmLmN1cnJlbnQpIHtcbiAgICAgIHNjb3BlLnJvb3RSZWYuY3VycmVudC5sYW5nID0gdWlMb2NhbGU7XG4gICAgfVxuICAgIGlmICghbG9ja3MudGVybWlub2xvZ3kpIHdyaXRlU3RvcmVkVWlDaHJvbWVUZXJtaW5vbG9neShzY29wZS5zdG9yYWdlLCB1aVRlcm1pbm9sb2d5KTtcbiAgICAvLyDwn5Ca77iPIGBzZXRBY3RpdmVVaVRoZW1lYCBpcyBwYWdlLWdsb2JhbCAod3JpdGVzIGBkb2N1bWVudC5kb2N1bWVudEVsZW1lbnRgJ3MgQ1NTIHZhcnMpIOKAlCBjb3JyZWN0IG9ubHlcbiAgICAvLyBmb3IgdGhlIHBhZ2Utb3duaW5nIHNoZWxsLiBBIGNvLW1vdW50ZWQgZW1iZWRkZWQgc2hlbGwgcGFpbnRzIGl0cyBvd24gdGhlbWUgdG9rZW5zIG9udG8gaXRzIG93blxuICAgIC8vIGAuc2VtaW8tc2NvcGVgIHJvb3QgaW5zdGVhZCwgdmlhIGBhcHBseVVpVGhlbWVUb1Jvb3RgLCBzbyB0d28gc2hlbGxzIHdpdGggZGlmZmVyZW50IGB0aGVtZUlkYCBsb2Nrc1xuICAgIC8vIG5ldmVyIGZpZ2h0IG92ZXIgdGhlIHNhbWUgZG9jdW1lbnQtd2lkZSB0b2tlbnMuXG4gICAgaWYgKHNjb3BlLm93bnNQYWdlKSB7XG4gICAgICBzZXRBY3RpdmVVaVRoZW1lKHVpVGhlbWUpO1xuICAgIH0gZWxzZSBpZiAoc2NvcGUucm9vdFJlZi5jdXJyZW50KSB7XG4gICAgICBhcHBseVVpVGhlbWVUb1Jvb3Qoc2NvcGUucm9vdFJlZi5jdXJyZW50LCB1aVRoZW1lKTtcbiAgICB9XG4gICAgaWYgKCFsb2Nrcy50aGVtZUlkKSB7XG4gICAgICB3cml0ZVN0b3JlZFVpQ2hyb21lVGhlbWVTbmFwc2hvdChzY29wZS5zdG9yYWdlLCB1aVRoZW1lKTtcbiAgICAgIHdyaXRlU3RvcmVkVWlDaHJvbWVUaGVtZUlkKHNjb3BlLnN0b3JhZ2UsIHVpVGhlbWVJZCk7XG4gICAgfVxuICAgIHdyaXRlU3RvcmVkVWlDdXN0b21UaGVtZXMoc2NvcGUuc3RvcmFnZSwgdWlDdXN0b21UaGVtZXMpO1xuICB9LCBbdWlBcHBlYXJhbmNlLCB1aUxheW91dCwgdWlEcml2ZXJJZCwgdWlDdXN0b21Ecml2ZXJzLCB1aUtleWJpbmRpbmdPdmVycmlkZXMsIHVpTG9jYWxlLCB1aVRlcm1pbm9sb2d5LCB1aVRoZW1lLCB1aVRoZW1lSWQsIHVpQ3VzdG9tVGhlbWVzLCBsb2Nrcywgc2NvcGVdKTtcblxuICAvLyDwn5Ca77iPIFVubW91bnQgY2xlYW51cCBmb3IgdGhlIGVtYmVkZGVkIChub24tcGFnZS1vd25pbmcpIGNhc2Ug4oCUIGEgc2hlbGwgdGhhdCBwYWludGVkIGl0cyBvd24gcm9vdCdzXG4gIC8vIHRoZW1lIHRva2VucyBtdXN0IHJlbW92ZSB0aGVtIG9uIHVubW91bnQsIG9yIGEgbGF0ZXIsIHVucmVsYXRlZCBlbGVtZW50IHJldXNlZCBhdCB0aGUgc2FtZSBET01cbiAgLy8gcG9zaXRpb24gKFJlYWN0L3ZpdGUgSE1SIHJldXNlLCBvciBhbm90aGVyIHNoZWxsJ3MgY2FudmFzLWNsb25lIGFzc2V0cyBpbiBhIGRldiBoYXJuZXNzKSB3b3VsZFxuICAvLyBzaWxlbnRseSBpbmhlcml0IGEgc3RhbGUgdGhlbWUncyBpbmxpbmUgb3ZlcnJpZGVzLiBUaGUgcGFnZS1vd25pbmcgY2FzZSBpcyBpbnRlbnRpb25hbGx5IGxlZnQgYWxvbmU6XG4gIC8vIGBkb2N1bWVudC5kb2N1bWVudEVsZW1lbnRgIG91dGxpdmVzIGFueSBzaW5nbGUgc2hlbGwncyBsaWZldGltZS5cbiAgdXNlRWZmZWN0KCgpID0+IHtcbiAgICBpZiAoc2NvcGUub3duc1BhZ2UpIHJldHVybjtcbiAgICByZXR1cm4gKCkgPT4ge1xuICAgICAgaWYgKHNjb3BlLnJvb3RSZWYuY3VycmVudCkgY2xlYXJVaVRoZW1lRnJvbVJvb3Qoc2NvcGUucm9vdFJlZi5jdXJyZW50KTtcbiAgICB9O1xuICB9LCBbc2NvcGVdKTtcbiAgLy8jZW5kcmVnaW9uXG5cbiAgdXNlQWN0aW9uSG90a2V5KFxuICAgIFwidWkubmF2LmJhY2tcIixcbiAgICB1c2VDYWxsYmFjaygoKSA9PiB7XG4gICAgICBpZiAoY2FuR29CYWNrKSBnb0JhY2soKTtcbiAgICB9LCBbY2FuR29CYWNrLCBnb0JhY2tdKSxcbiAgICB1bmRlZmluZWQsXG4gICAgW2NhbkdvQmFjaywgZ29CYWNrXSxcbiAgICB7IG92ZXJyaWRlczogdWlLZXliaW5kaW5nT3ZlcnJpZGVzIH0sXG4gICk7XG4gIHVzZUFjdGlvbkhvdGtleShcbiAgICBcInVpLm5hdi5mb3J3YXJkXCIsXG4gICAgdXNlQ2FsbGJhY2soKCkgPT4ge1xuICAgICAgaWYgKGNhbkdvRm9yd2FyZCkgZ29Gb3J3YXJkKCk7XG4gICAgfSwgW2NhbkdvRm9yd2FyZCwgZ29Gb3J3YXJkXSksXG4gICAgdW5kZWZpbmVkLFxuICAgIFtjYW5Hb0ZvcndhcmQsIGdvRm9yd2FyZF0sXG4gICAgeyBvdmVycmlkZXM6IHVpS2V5YmluZGluZ092ZXJyaWRlcyB9LFxuICApO1xuICB1c2VBY3Rpb25Ib3RrZXkoXG4gICAgXCJ1aS5uYXYudXBcIixcbiAgICB1c2VDYWxsYmFjaygoKSA9PiB7XG4gICAgICBpZiAoY2FuR29VcCkgZ29VcCgpO1xuICAgIH0sIFtjYW5Hb1VwLCBnb1VwXSksXG4gICAgdW5kZWZpbmVkLFxuICAgIFtjYW5Hb1VwLCBnb1VwXSxcbiAgICB7IG92ZXJyaWRlczogdWlLZXliaW5kaW5nT3ZlcnJpZGVzIH0sXG4gICk7XG4gIHVzZUFjdGlvbkhvdGtleShcbiAgICBcInVpLnNlYXJjaC50b2dnbGVcIixcbiAgICB1c2VDYWxsYmFjaygoKSA9PiBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1NFQVJDSF9PUEVOXCIsIHZhbHVlOiAob3BlbikgPT4gIW9wZW4gfSksIFtdKSxcbiAgICB1bmRlZmluZWQsXG4gICAgW10sXG4gICAgeyBvdmVycmlkZXM6IHVpS2V5YmluZGluZ092ZXJyaWRlcyB9LFxuICApO1xuICB1c2VBY3Rpb25Ib3RrZXkoXG4gICAgXCJ1aS5maW5kLnRvZ2dsZVwiLFxuICAgIHVzZUNhbGxiYWNrKCgpID0+IGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfRklORF9PUEVOXCIsIHZhbHVlOiAob3BlbikgPT4gIW9wZW4gfSksIFtdKSxcbiAgICB1bmRlZmluZWQsXG4gICAgW10sXG4gICAgeyBvdmVycmlkZXM6IHVpS2V5YmluZGluZ092ZXJyaWRlcyB9LFxuICApO1xuXG4gIGNvbnN0IGFwcGx5TmFtZWRMYXlvdXQgPSB1c2VDYWxsYmFjayhcbiAgICAobGF5b3V0OiBXaW5kb3dMYXlvdXQpID0+IHtcbiAgICAgIGlmICghc2Vzc2lvbikgcmV0dXJuO1xuICAgICAgY29uc3Qgc2VlZGVkID0gYXBwbHlGcmFtZXdvcmtMYXlvdXRTZWVkKGxheW91dCwgc2Vzc2lvbi5hcHAud2luZG93S2luZHMsIGFwcExhYmVsc092ZXJsYXksIHVpVGVybWlub2xvZ3ksIHVpTG9jYWxlKTtcbiAgICAgIGV4dHJhV2luZG93SW5zdGFuY2VzUmVmLmN1cnJlbnQgPSBzZWVkZWQuZXh0cmFJbnN0YW5jZXM7XG4gICAgICBleHRyYVdpbmRvd0NvdW50ZXJSZWYuY3VycmVudCA9IHNlZWRlZC5leHRyYUluc3RhbmNlcy5sZW5ndGg7XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0VYVFJBX1dJTkRPV19JTlNUQU5DRVNcIiwgdmFsdWU6IHNlZWRlZC5leHRyYUluc3RhbmNlcyB9KTtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfU0hFTExfTEFZT1VUXCIsIHZhbHVlOiBzZWVkZWQubW9kZUxheW91dCB9KTtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfQUNUSVZFX1dJTkRPV19JRFwiLCB2YWx1ZTogbnVsbCB9KTtcbiAgICAgIC8vIPCfqp/vuI8gSGFuZCB0aGUganVzdC1jb21wdXRlZCBpbnN0YW5jZSBsaXN0IHN0cmFpZ2h0IHRvIHRoZSBmZXRjaCByYXRoZXIgdGhhbiByZWFkaW5nIGBleHRyYVdpbmRvd0luc3RhbmNlc2BcbiAgICAgIC8vIHN0YXRlICh3aGljaCB3b3VsZG4ndCByZWZsZWN0IHRoaXMgZGlzcGF0Y2ggdW50aWwgdGhlIG5leHQgcmVuZGVyKSDigJQgZXZlcnkgbmV3bHktc2VlZGVkIHBhbmUncyBvd25cbiAgICAgIC8vIGJvZHkvbWVhc3VyZXMvZW5nYWdlbWVudCBnZXRzIGZldGNoZWQgaW1tZWRpYXRlbHkgaW5zdGVhZCBvZiBzaG93aW5nIFwibWlzc2luZyB3aW5kb3dcIiB1bnRpbCBsYXRlci5cbiAgICAgIHZvaWQgcmVmcmVzaFVpKHNlc3Npb24sIHsga2luZDogXCJmdWxsXCIgfSwgc2VlZGVkLmV4dHJhSW5zdGFuY2VzKTtcbiAgICB9LFxuICAgIFtzZXNzaW9uLCBhcHBMYWJlbHNPdmVybGF5LCByZWZyZXNoVWksIHVpVGVybWlub2xvZ3ksIHVpTG9jYWxlXSxcbiAgKTtcblxuICBjb25zdCBhcHBseU1vZGVDaGFuZ2UgPSB1c2VDYWxsYmFjayhcbiAgICAobW9kZUlkOiBzdHJpbmcpID0+IHtcbiAgICAgIC8vIPCfm6DvuI8gVG9vbHMgYXJlIHNjb3BlZCB0byBhIG1vZGUg4oCUIHN3aXRjaGluZyBtb2RlcyBhbHdheXMgY2xlYXJzIHRoZSBhY3RpdmUgdG9vbCAoYW5kIGV2ZXJ5XG4gICAgICAvLyB3aW5kb3cncyBhY3RpdmUgdXRpbGl0eSksIG1pcnJvcmluZyBob3cgYSBmcmVzaCBtb2RlIHN0YXJ0cyB3aXRoIG5vIHV0aWxpdHkgcHJlc3NlZCBlaXRoZXIuXG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0FDVElWRV9UT09MXCIsIHRvb2xJZDogbnVsbCB9KTtcbiAgICAgIGRpc3BhdGNoKHtcbiAgICAgICAgdHlwZTogXCJTRVRfU0VTU0lPTlwiLFxuICAgICAgICB2YWx1ZTogKGN1cnJlbnQpID0+IHtcbiAgICAgICAgICBpZiAoIWN1cnJlbnQpIHJldHVybiBjdXJyZW50O1xuICAgICAgICAgIGNvbnN0IGxheW91dCA9IHJlc29sdmVMYXlvdXRGb3JNb2RlKGN1cnJlbnQuYXBwLCBtb2RlSWQpO1xuICAgICAgICAgIGNvbnN0IG5leHRTZXNzaW9uOiBBY3RpdmVTZXNzaW9uID0geyAuLi5jdXJyZW50LCB2aWV3U3RhdGU6IHsgLi4uY3VycmVudC52aWV3U3RhdGUsIGFjdGl2ZU1vZGVJZDogbW9kZUlkLCBhY3RpdmVUb29sSWQ6IHVuZGVmaW5lZCB9IH07XG4gICAgICAgICAgaWYgKGxheW91dCkge1xuICAgICAgICAgICAgY29uc3Qgc2VlZGVkID0gYXBwbHlGcmFtZXdvcmtMYXlvdXRTZWVkKGxheW91dCwgY3VycmVudC5hcHAud2luZG93S2luZHMsIGFwcExhYmVsc092ZXJsYXksIHVpVGVybWlub2xvZ3ksIHVpTG9jYWxlKTtcbiAgICAgICAgICAgIGV4dHJhV2luZG93SW5zdGFuY2VzUmVmLmN1cnJlbnQgPSBzZWVkZWQuZXh0cmFJbnN0YW5jZXM7XG4gICAgICAgICAgICBleHRyYVdpbmRvd0NvdW50ZXJSZWYuY3VycmVudCA9IHNlZWRlZC5leHRyYUluc3RhbmNlcy5sZW5ndGg7XG4gICAgICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0VYVFJBX1dJTkRPV19JTlNUQU5DRVNcIiwgdmFsdWU6IHNlZWRlZC5leHRyYUluc3RhbmNlcyB9KTtcbiAgICAgICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfU0hFTExfTEFZT1VUXCIsIHZhbHVlOiBzZWVkZWQubW9kZUxheW91dCB9KTtcbiAgICAgICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfQUNUSVZFX1dJTkRPV19JRFwiLCB2YWx1ZTogbnVsbCB9KTtcbiAgICAgICAgICAgIHZvaWQgcmVmcmVzaFVpKG5leHRTZXNzaW9uLCB7IGtpbmQ6IFwiZnVsbFwiIH0sIHNlZWRlZC5leHRyYUluc3RhbmNlcyk7XG4gICAgICAgICAgfVxuICAgICAgICAgIHJldHVybiBuZXh0U2Vzc2lvbjtcbiAgICAgICAgfSxcbiAgICAgIH0pO1xuICAgIH0sXG4gICAgW2FwcExhYmVsc092ZXJsYXksIHJlZnJlc2hVaSwgdWlUZXJtaW5vbG9neSwgdWlMb2NhbGVdLFxuICApO1xuXG4gIGNvbnN0IGhhbmRsZVRlbXBsYXRlRHJvcCA9IHVzZUNhbGxiYWNrKFxuICAgIChwYXlsb2FkOiBXaW5kb3dUZW1wbGF0ZURyb3BQYXlsb2FkLCB0YXJnZXQ6IE1vZGVDYW52YXNEcm9wVGFyZ2V0KSA9PiB7XG4gICAgICBpZiAoIXNlc3Npb24pIHJldHVybjtcbiAgICAgIGNvbnN0IGtpbmQgPSBzZXNzaW9uLmFwcC53aW5kb3dLaW5kcy5maW5kKChlbnRyeSkgPT4gZW50cnkuaWQgPT09IHBheWxvYWQud2luZG93S2luZElkKTtcbiAgICAgIGlmICgha2luZCkgcmV0dXJuO1xuICAgICAgZXh0cmFXaW5kb3dDb3VudGVyUmVmLmN1cnJlbnQgKz0gMTtcbiAgICAgIGNvbnN0IGluc3RhbmNlSWQgPSBgJHtwYXlsb2FkLndpbmRvd0tpbmRJZH0tJHtleHRyYVdpbmRvd0NvdW50ZXJSZWYuY3VycmVudH1gO1xuICAgICAgY29uc3QgcHJvamVjdGlvblNwZWMgPSBkZWNvZGVXb3JsZFByb2plY3Rpb25UZW1wbGF0ZUlkKHBheWxvYWQudGVtcGxhdGVJZCk7XG4gICAgICBpZiAocHJvamVjdGlvblNwZWMpIHJlZ2lzdGVyUGVuZGluZ1dvcmxkUHJvamVjdGlvbihpbnN0YW5jZUlkLCBwcm9qZWN0aW9uU3BlYyk7XG4gICAgICBjb25zdCB0aXRsZSA9IHByb2plY3Rpb25TcGVjID8gd29ybGRQcm9qZWN0aW9uU3BlY0xhYmVsKHByb2plY3Rpb25TcGVjKSA6IHJlc29sdmVBcHBMYWJlbChhcHBMYWJlbHNPdmVybGF5LCBcIndpbmRvd0tpbmRcIiwga2luZC5pZCwgcmVzb2x2ZU1hbmlmZXN0TGFiZWwoa2luZC5sYWJlbCwgdWlUZXJtaW5vbG9neSwgdWlMb2NhbGUpKTtcbiAgICAgIGNvbnN0IG5leHRFeHRyYUluc3RhbmNlcyA9IFsuLi5leHRyYVdpbmRvd0luc3RhbmNlc1JlZi5jdXJyZW50LCB7IGlkOiBpbnN0YW5jZUlkLCB3aW5kb3dLaW5kSWQ6IHBheWxvYWQud2luZG93S2luZElkLCB0aXRsZSB9XTtcbiAgICAgIGV4dHJhV2luZG93SW5zdGFuY2VzUmVmLmN1cnJlbnQgPSBuZXh0RXh0cmFJbnN0YW5jZXM7XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0VYVFJBX1dJTkRPV19JTlNUQU5DRVNcIiwgdmFsdWU6IG5leHRFeHRyYUluc3RhbmNlcyB9KTtcbiAgICAgIGlmIChwcm9qZWN0aW9uU3BlYykge1xuICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1dJTkRPV19USVRMRVwiLCB3aW5kb3dJZDogaW5zdGFuY2VJZCwgdGl0bGUgfSk7XG4gICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfV0lORE9XX0lDT05cIiwgd2luZG93SWQ6IGluc3RhbmNlSWQsIGljb25JZDogd29ybGRQcm9qZWN0aW9uU3BlY0ljb25JZChwcm9qZWN0aW9uU3BlYykgYXMgSWNvbk5hbWUgfSk7XG4gICAgICB9XG4gICAgICAvLyDwn6qf77iPIFRoZSBuZXcgc3BsaXQgcGFuZSBpcyBpdHMgb3duIHdpbmRvdyBpbnN0YW5jZSDigJQgZmV0Y2ggaXRzIGJvZHkvbWVhc3VyZXMvZW5nYWdlbWVudCByaWdodCBhd2F5XG4gICAgICAvLyAoc2VlIGBhcHBseU5hbWVkTGF5b3V0YCdzIGNvbW1lbnQpIHJhdGhlciB0aGFuIHdhaXRpbmcgZm9yIGFuIHVucmVsYXRlZCBhY3Rpb24gdG8gdHJpZ2dlciBhIHJlZnJlc2guXG4gICAgICB2b2lkIHJlZnJlc2hVaShzZXNzaW9uLCB7IGtpbmQ6IFwiZnVsbFwiIH0sIG5leHRFeHRyYUluc3RhbmNlcyk7XG4gICAgICBkaXNwYXRjaCh7XG4gICAgICAgIHR5cGU6IFwiU0VUX1NIRUxMX0xBWU9VVFwiLFxuICAgICAgICB2YWx1ZTogKGN1cnJlbnQpID0+IHtcbiAgICAgICAgICBjb25zdCBiYXNlID1cbiAgICAgICAgICAgIGN1cnJlbnQgPz9cbiAgICAgICAgICAgIHJlc29sdmVGcmFtZXdvcmtMYXlvdXRTZWVkKHNlc3Npb24uYXBwLmRlZmF1bHRMYXlvdXQsIHNlc3Npb24uYXBwLndpbmRvd0tpbmRzLCBhcHBMYWJlbHNPdmVybGF5LCB1aVRlcm1pbm9sb2d5LCB1aUxvY2FsZSkubW9kZUxheW91dDtcbiAgICAgICAgICByZXR1cm4gaW5zZXJ0V2luZG93QXREcm9wWm9uZShiYXNlLCBpbnN0YW5jZUlkLCB0YXJnZXQpO1xuICAgICAgICB9LFxuICAgICAgfSk7XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0FDVElWRV9XSU5ET1dfSURcIiwgdmFsdWU6IGluc3RhbmNlSWQgfSk7XG4gICAgICBub3RlU2hlbGxDb21tYW5kKFwic2hlbGwud2luZG93U3BsaXRcIiwgc2hlbGxMYWJlbChcInVpLnNoZWxsQ29tbWFuZC53aW5kb3dTcGxpdFwiKSwgeyB3aW5kb3dLaW5kSWQ6IHBheWxvYWQud2luZG93S2luZElkLCBpbnN0YW5jZUlkIH0pO1xuICAgIH0sXG4gICAgW2FwcExhYmVsc092ZXJsYXksIHJlZnJlc2hVaSwgc2Vzc2lvbiwgbm90ZVNoZWxsQ29tbWFuZCwgdWlUZXJtaW5vbG9neSwgdWlMb2NhbGVdLFxuICApO1xuXG4gIGNvbnN0IGRpc3BsYXlIb3N0UmVmID0gdXNlUmVmPERpc3BsYXlIb3N0QXBpIHwgbnVsbD4obnVsbCk7XG4gIGNvbnN0IGRpc3BsYXlIb3N0ID0gdXNlTmFtZWRMYXlvdXRIb3N0KHtcbiAgICBhcHBJZDogc2Vzc2lvbj8uYXBwLmlkID8/IFwiZnJhbWV3b3JrLW9zXCIsXG4gICAgd2luZG93S2luZHM6IHNlc3Npb24/LmFwcC53aW5kb3dLaW5kcy5tYXAoKGtpbmQpID0+ICh7IC4uLmtpbmQsIGxhYmVsOiByZXNvbHZlQXBwTGFiZWwoYXBwTGFiZWxzT3ZlcmxheSwgXCJ3aW5kb3dLaW5kXCIsIGtpbmQuaWQsIHJlc29sdmVNYW5pZmVzdExhYmVsKGtpbmQubGFiZWwsIHVpVGVybWlub2xvZ3ksIHVpTG9jYWxlKSkgfSkpID8/IFtdLFxuICAgIGJ1aWx0aW5MYXlvdXRzOiBzZXNzaW9uPy5hcHAubmFtZWRMYXlvdXRzID8/IFtdLFxuICAgIGN1cnJlbnRMYXlvdXQ6IGNhcHR1cmVDdXJyZW50RnJhbWV3b3JrTGF5b3V0KHNoZWxsTGF5b3V0LCBleHRyYVdpbmRvd0luc3RhbmNlcywgc2Vzc2lvbj8uYXBwLmRlZmF1bHRMYXlvdXQpLFxuICAgIG9uQXBwbHlMYXlvdXQ6IGFwcGx5TmFtZWRMYXlvdXQsXG4gICAgbmFtZWRMYXlvdXRTdG9yZSxcbiAgfSk7XG4gIGRpc3BsYXlIb3N0UmVmLmN1cnJlbnQgPSBkaXNwbGF5SG9zdDtcblxuICAvLyNyZWdpb24g8J+Ulu+4j1RoZW1lTXV0YXRvcnNcbiAgY29uc3QgdWlUaGVtZUJhc2UgPSB1aVRoZW1lRHJhZnQgPz8gdWlUaGVtZTtcbiAgY29uc3QgdWlUaGVtZURpcnR5ID0gdWlUaGVtZURyYWZ0ICE9PSBudWxsO1xuICBjb25zdCB1aVRoZW1lTGlzdCA9IHVzZU1lbW8oKCk6IHJlYWRvbmx5IFVpVGhlbWVbXSA9PiBbLi4uYnVpbHRpblVpVGhlbWVzKCksIC4uLk9iamVjdC52YWx1ZXModWlDdXN0b21UaGVtZXMpXSwgW3VpQ3VzdG9tVGhlbWVzXSk7XG4gIGNvbnN0IHVpRHJpdmVyTGlzdCA9IHVzZU1lbW8oKCk6IHJlYWRvbmx5IFVpRHJpdmVyW10gPT4gWy4uLmJ1aWx0aW5VaURyaXZlcnMoKSwgLi4uT2JqZWN0LnZhbHVlcyh1aUN1c3RvbURyaXZlcnMpXSwgW3VpQ3VzdG9tRHJpdmVyc10pO1xuICBjb25zdCBrZXlzQnlBY3Rpb25JZCA9IHVzZU1lbW8oKCkgPT4gYnVpbGRLZXlzQnlBY3Rpb25JZChzZXNzaW9uPy5hcHAua2V5YmluZGluZ3MgPz8gW10pLCBbc2Vzc2lvbj8uYXBwLmtleWJpbmRpbmdzXSk7XG4gIGNvbnN0IGNvbnRyb2xLZXliaW5kaW5ncyA9IHVzZU1lbW8oKCkgPT4gY29tcG9zZUNvbnRyb2xLZXliaW5kaW5ncyhrZXlzQnlBY3Rpb25JZCwgdWlLZXliaW5kaW5nT3ZlcnJpZGVzKSwgW2tleXNCeUFjdGlvbklkLCB1aUtleWJpbmRpbmdPdmVycmlkZXNdKTtcbiAgY29uc3Qgb3NDb21tYW5kcyA9IHVzZU1lbW8oXG4gICAgKCkgPT4gYnVpbGRPc0NvbW1hbmRzKHVpVGhlbWVMaXN0LCBbVUlfVEVSTUlOT0xPR1lfTkFUSVZFLCAuLi4oc2Vzc2lvbj8uYXBwLnRlcm1pbm9sb2dpZXMgPz8gW10pXSwgYWN0aXZlSW50cm9kdWN0aW9uICE9IG51bGwsIGxvY2tzLCB1aURyaXZlckxpc3QsIGFjdGl2ZVR1dG9yaWFscywgdHV0b3JpYWxSZWNvcmRlckF2YWlsYWJsZSwgdWlUZXJtaW5vbG9neSwgdWlMb2NhbGUpLFxuICAgIFt1aVRoZW1lTGlzdCwgc2Vzc2lvbj8uYXBwLnRlcm1pbm9sb2dpZXMsIGFjdGl2ZUludHJvZHVjdGlvbiwgdWlMb2NhbGUsIHVpVGVybWlub2xvZ3ksIGxvY2tzLCB1aURyaXZlckxpc3QsIGFjdGl2ZVR1dG9yaWFscywgdHV0b3JpYWxSZWNvcmRlckF2YWlsYWJsZV0sXG4gICk7XG5cbiAgLyoqIPCfp63vuI8gRGlyZWN0IHRoZW1lL2FwcGVhcmFuY2UvbG9jYWxlL3Rlcm1pbm9sb2d5L2RyaXZlci9sYXlvdXQgc2V0dGVycyBiZWxvdyAoc2V0dGluZ3MgcGFuZWwsIHRoZW1lL2RyaXZlclxuICAgKiBlZGl0b3JzKSBieXBhc3MgYGRpc3BhdGNoT3NDb21tYW5kYCdzIG5hbWVkLWNvbW1hbmQgcGF0aCBlbnRpcmVseSDigJQgdGhpcyByZXVzZXMgdGhlIGV4YWN0IHNhbWUgYG9zLipgXG4gICAqIGNvbW1hbmQgaWQgKGFuZCBpdHMgYG9zQ29tbWFuZHNgLXJlc29sdmVkLCBsb2NhbGUtYWRhcHRlZCBsYWJlbCkgc28gYSBkaXJlY3QtcGF0aCBjaGFuZ2UgZm9sZHMgdG9nZXRoZXJcbiAgICogd2l0aCBhIGNvbW1hbmQtcGFsZXR0ZS10cmlnZ2VyZWQgb25lIGluIHRoZSBoaXN0b3J5IHBhbmVsIHJlZ2FyZGxlc3Mgb2Ygd2hpY2ggcGF0aCB0cmlnZ2VyZWQgaXQuICovXG4gIGNvbnN0IG5vdGVPc0NvbW1hbmQgPSB1c2VDYWxsYmFjayhcbiAgICAoY29tbWFuZElkOiBzdHJpbmcsIGRldGFpbD86IFJlY29yZDxzdHJpbmcsIHVua25vd24+KSA9PiB7XG4gICAgICBjb25zdCBsYWJlbCA9IG9zQ29tbWFuZHMuZmluZCgoZW50cnkpID0+IGVudHJ5LmlkID09PSBjb21tYW5kSWQpPy5sYWJlbCA/PyBjb21tYW5kSWQ7XG4gICAgICBub3RlU2hlbGxDb21tYW5kKGNvbW1hbmRJZCwgbGFiZWwsIGRldGFpbCk7XG4gICAgfSxcbiAgICBbb3NDb21tYW5kcywgbm90ZVNoZWxsQ29tbWFuZF0sXG4gICk7XG5cbiAgY29uc3QgZHJhZnRUaGVtZVBhdGNoID0gdXNlQ2FsbGJhY2soXG4gICAgKHBhdGNoOiAobmV4dDogVWlUaGVtZSkgPT4gdm9pZCkgPT4ge1xuICAgICAgY29uc3QgbmV4dCA9IHN0cnVjdHVyZWRDbG9uZSh1aVRoZW1lQmFzZSk7XG4gICAgICBwYXRjaChuZXh0KTtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfVUlfVEhFTUVfRFJBRlRcIiwgdmFsdWU6IG5leHQgfSk7XG4gICAgfSxcbiAgICBbdWlUaGVtZUJhc2VdLFxuICApO1xuXG4gIGNvbnN0IHNldFRoZW1lSWQgPSB1c2VDYWxsYmFjayhcbiAgICAoaWQ6IHN0cmluZykgPT4ge1xuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9VSV9USEVNRV9EUkFGVFwiLCB2YWx1ZTogbnVsbCB9KTtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfVUlfVEhFTUVfSURcIiwgdmFsdWU6IGlkIH0pO1xuICAgICAgbm90ZU9zQ29tbWFuZChcIm9zLnNldFRoZW1lSWRcIiwgeyB0aGVtZUlkOiBpZCB9KTtcbiAgICB9LFxuICAgIFtub3RlT3NDb21tYW5kXSxcbiAgKTtcblxuICBjb25zdCBzZXRUaGVtZUNvbG9yID0gdXNlQ2FsbGJhY2soXG4gICAgKGtleTogc3RyaW5nLCBoZXg6IHN0cmluZykgPT5cbiAgICAgIGRyYWZ0VGhlbWVQYXRjaCgobmV4dCkgPT4ge1xuICAgICAgICBuZXh0LmNvbG9yc1trZXldID0gaGV4O1xuICAgICAgfSksXG4gICAgW2RyYWZ0VGhlbWVQYXRjaF0sXG4gICk7XG4gIGNvbnN0IHNldFRoZW1lU3BhY2luZyA9IHVzZUNhbGxiYWNrKFxuICAgIChrZXk6IHN0cmluZywgdmFsdWU6IHN0cmluZykgPT5cbiAgICAgIGRyYWZ0VGhlbWVQYXRjaCgobmV4dCkgPT4ge1xuICAgICAgICBuZXh0LnNwYWNpbmdba2V5XSA9IHZhbHVlO1xuICAgICAgfSksXG4gICAgW2RyYWZ0VGhlbWVQYXRjaF0sXG4gICk7XG4gIGNvbnN0IHNldFRoZW1lRm9udFN0YWNrID0gdXNlQ2FsbGJhY2soXG4gICAgKGtleTogc3RyaW5nLCB2YWx1ZTogc3RyaW5nKSA9PlxuICAgICAgZHJhZnRUaGVtZVBhdGNoKChuZXh0KSA9PiB7XG4gICAgICAgIG5leHQuZm9udFN0YWNrc1trZXldID0gdmFsdWU7XG4gICAgICB9KSxcbiAgICBbZHJhZnRUaGVtZVBhdGNoXSxcbiAgKTtcbiAgY29uc3Qgc2V0VGhlbWVTdHJva2UgPSB1c2VDYWxsYmFjayhcbiAgICAoa2V5OiBzdHJpbmcsIHZhbHVlOiBudW1iZXIgfCBudW1iZXJbXSkgPT5cbiAgICAgIGRyYWZ0VGhlbWVQYXRjaCgobmV4dCkgPT4ge1xuICAgICAgICBuZXh0LnN0cm9rZXNba2V5XSA9IHZhbHVlO1xuICAgICAgfSksXG4gICAgW2RyYWZ0VGhlbWVQYXRjaF0sXG4gICk7XG4gIGNvbnN0IHNldFRoZW1lUmFkaXVzID0gdXNlQ2FsbGJhY2soXG4gICAgKGtleTogc3RyaW5nLCB2YWx1ZTogbnVtYmVyKSA9PlxuICAgICAgZHJhZnRUaGVtZVBhdGNoKChuZXh0KSA9PiB7XG4gICAgICAgIG5leHQucmFkaWlba2V5XSA9IHZhbHVlO1xuICAgICAgfSksXG4gICAgW2RyYWZ0VGhlbWVQYXRjaF0sXG4gICk7XG4gIGNvbnN0IHNldFRoZW1lT3BhY2l0eSA9IHVzZUNhbGxiYWNrKFxuICAgIChrZXk6IHN0cmluZywgdmFsdWU6IG51bWJlcikgPT5cbiAgICAgIGRyYWZ0VGhlbWVQYXRjaCgobmV4dCkgPT4ge1xuICAgICAgICBuZXh0Lm9wYWNpdGllc1trZXldID0gdmFsdWU7XG4gICAgICB9KSxcbiAgICBbZHJhZnRUaGVtZVBhdGNoXSxcbiAgKTtcbiAgY29uc3Qgc2V0VGhlbWVNZXRyaWMgPSB1c2VDYWxsYmFjayhcbiAgICAoc2VjdGlvbjogc3RyaW5nLCBrZXk6IHN0cmluZywgdmFsdWU6IG51bWJlciB8IG51bWJlcltdKSA9PlxuICAgICAgZHJhZnRUaGVtZVBhdGNoKChuZXh0KSA9PiB7XG4gICAgICAgIG5leHQubWV0cmljc1tzZWN0aW9uXSA9IHsgLi4uKG5leHQubWV0cmljc1tzZWN0aW9uXSA/PyB7fSksIFtrZXldOiB2YWx1ZSB9O1xuICAgICAgfSksXG4gICAgW2RyYWZ0VGhlbWVQYXRjaF0sXG4gICk7XG4gIGNvbnN0IHNldFRoZW1lQXBwZWFyYW5jZVBhaW50ID0gdXNlQ2FsbGJhY2soXG4gICAgKGFwcGVhcmFuY2U6IFRoZW1lQXBwZWFyYW5jZU5hbWUsIGdyb3VwOiBUaGVtZVBhbGV0dGVHcm91cCwga2V5OiBzdHJpbmcsIGhleDogc3RyaW5nLCBhbHBoYT86IG51bWJlcikgPT5cbiAgICAgIGRyYWZ0VGhlbWVQYXRjaCgobmV4dCkgPT4ge1xuICAgICAgICBuZXh0LmFwcGVhcmFuY2VzW2FwcGVhcmFuY2VdW2dyb3VwXVtrZXldID0gYWxwaGEgPT09IHVuZGVmaW5lZCA/IHsgaGV4IH0gOiB7IGhleCwgYWxwaGEgfTtcbiAgICAgIH0pLFxuICAgIFtkcmFmdFRoZW1lUGF0Y2hdLFxuICApO1xuXG4gIGNvbnN0IHJlc2V0VGhlbWUgPSB1c2VDYWxsYmFjaygoKSA9PiB7XG4gICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9VSV9USEVNRV9EUkFGVFwiLCB2YWx1ZTogbnVsbCB9KTtcbiAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1VJX1RIRU1FX0lEXCIsIHZhbHVlOiBcInNlbWlvXCIgfSk7XG4gIH0sIFtdKTtcblxuICBjb25zdCBzYXZlVGhlbWUgPSB1c2VDYWxsYmFjayhcbiAgICAobGFiZWw6IHN0cmluZykgPT4ge1xuICAgICAgY29uc3QgdHJpbW1lZCA9IGxhYmVsLnRyaW0oKTtcbiAgICAgIGlmICghdHJpbW1lZCkgcmV0dXJuO1xuICAgICAgY29uc3Qgc2x1ZyA9IHRyaW1tZWRcbiAgICAgICAgLnRvTG93ZXJDYXNlKClcbiAgICAgICAgLnJlcGxhY2UoL1teYS16MC05XSsvZywgXCItXCIpXG4gICAgICAgIC5yZXBsYWNlKC8oXi0rfC0rJCkvZywgXCJcIik7XG4gICAgICBpZiAoIXNsdWcpIHJldHVybjtcbiAgICAgIGNvbnN0IGlkID0gYGN1c3RvbS4ke3NsdWd9YDtcbiAgICAgIGNvbnN0IHNhdmVkOiBVaVRoZW1lID0geyAuLi51aVRoZW1lQmFzZSwgaWQsIGxhYmVsOiB0cmltbWVkIH07XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1VJX0NVU1RPTV9USEVNRVNcIiwgdmFsdWU6IChjdXJyZW50KSA9PiAoeyAuLi5jdXJyZW50LCBbaWRdOiBzYXZlZCB9KSB9KTtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfVUlfVEhFTUVfRFJBRlRcIiwgdmFsdWU6IG51bGwgfSk7XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1VJX1RIRU1FX0lEXCIsIHZhbHVlOiBpZCB9KTtcbiAgICB9LFxuICAgIFt1aVRoZW1lQmFzZV0sXG4gICk7XG5cbiAgY29uc3QgZGVsZXRlVGhlbWUgPSB1c2VDYWxsYmFjaygoaWQ6IHN0cmluZykgPT4ge1xuICAgIGlmICghaWQuc3RhcnRzV2l0aChcImN1c3RvbS5cIikpIHJldHVybjtcbiAgICBkaXNwYXRjaCh7XG4gICAgICB0eXBlOiBcIlNFVF9VSV9DVVNUT01fVEhFTUVTXCIsXG4gICAgICB2YWx1ZTogKGN1cnJlbnQpID0+IHtcbiAgICAgICAgY29uc3QgeyBbaWRdOiBfcmVtb3ZlZCwgLi4ucmVzdCB9ID0gY3VycmVudDtcbiAgICAgICAgcmV0dXJuIHJlc3Q7XG4gICAgICB9LFxuICAgIH0pO1xuICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfVUlfVEhFTUVfSURcIiwgdmFsdWU6IChjdXJyZW50KSA9PiAoY3VycmVudCA9PT0gaWQgPyBcInNlbWlvXCIgOiBjdXJyZW50KSB9KTtcbiAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1VJX1RIRU1FX0RSQUZUXCIsIHZhbHVlOiBudWxsIH0pO1xuICB9LCBbXSk7XG5cbiAgY29uc3QgZXhwb3J0VGhlbWUgPSB1c2VDYWxsYmFjaygoKSA9PiB7XG4gICAgZG93bmxvYWRNZWRpYUV4cG9ydChgJHt1aVRoZW1lQmFzZS5pZH0udGhlbWUuZHNsYCwgXCJ0ZXh0L3BsYWluXCIsIHNlcmlhbGl6ZVVpVGhlbWUodWlUaGVtZUJhc2UpKTtcbiAgfSwgW3VpVGhlbWVCYXNlXSk7XG5cbiAgY29uc3QgaW1wb3J0VGhlbWUgPSB1c2VDYWxsYmFjayhhc3luYyAoKSA9PiB7XG4gICAgY29uc3Qgb3BlbmVkID0gKGF3YWl0IHJlcXVlc3RGaWxlT3BlbihcIi50aGVtZS5kc2wsLmRzbCx0ZXh0L3BsYWluXCIpKVswXTtcbiAgICBpZiAoIW9wZW5lZCkgcmV0dXJuO1xuICAgIHRyeSB7XG4gICAgICBjb25zdCBwYXJzZWQgPSBwYXJzZVVpVGhlbWUoSlNPTi5wYXJzZShvcGVuZWQuY29udGVudHMpKTtcbiAgICAgIHNhdmVUaGVtZShwYXJzZWQubGFiZWwgfHwgcGFyc2VkLmlkKTtcbiAgICB9IGNhdGNoIHtcbiAgICAgIC8qIGludmFsaWQgdGhlbWUgZmlsZSwgaWdub3JlICovXG4gICAgfVxuICB9LCBbc2F2ZVRoZW1lXSk7XG4gIC8vI2VuZHJlZ2lvbiDwn5SW77iPVGhlbWVNdXRhdG9yc1xuXG4gIC8vI3JlZ2lvbiDwn5qX77iPRHJpdmVyTXV0YXRvcnNcbiAgY29uc3QgdWlEcml2ZXJCYXNlID0gdWlEcml2ZXJEcmFmdCA/PyB1aURyaXZlcjtcbiAgY29uc3QgdWlEcml2ZXJEaXJ0eSA9IHVpRHJpdmVyRHJhZnQgIT09IG51bGw7XG5cbiAgY29uc3Qgc2V0RHJpdmVySWQgPSB1c2VDYWxsYmFjayhcbiAgICAoaWQ6IHN0cmluZykgPT4ge1xuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9VSV9EUklWRVJfRFJBRlRcIiwgdmFsdWU6IG51bGwgfSk7XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1VJX0RSSVZFUl9JRFwiLCB2YWx1ZTogaWQgfSk7XG4gICAgICBub3RlT3NDb21tYW5kKFwib3Muc2V0RHJpdmVyXCIsIHsgZHJpdmVyOiBpZCB9KTtcbiAgICB9LFxuICAgIFtub3RlT3NDb21tYW5kXSxcbiAgKTtcblxuICBjb25zdCBzZXREcml2ZXJGaWVsZCA9IHVzZUNhbGxiYWNrKFxuICAgIDxLIGV4dGVuZHMga2V5b2YgT21pdDxVaURyaXZlciwgXCJpZFwiIHwgXCJsYWJlbFwiPj4oa2V5OiBLLCB2YWx1ZTogVWlEcml2ZXJbS10pID0+IHtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfVUlfRFJJVkVSX0RSQUZUXCIsIHZhbHVlOiB7IC4uLnVpRHJpdmVyQmFzZSwgW2tleV06IHZhbHVlIH0gfSk7XG4gICAgfSxcbiAgICBbdWlEcml2ZXJCYXNlXSxcbiAgKTtcblxuICBjb25zdCBzYXZlRHJpdmVyID0gdXNlQ2FsbGJhY2soXG4gICAgKGxhYmVsOiBzdHJpbmcpID0+IHtcbiAgICAgIGNvbnN0IHRyaW1tZWQgPSBsYWJlbC50cmltKCk7XG4gICAgICBpZiAoIXRyaW1tZWQpIHJldHVybjtcbiAgICAgIGNvbnN0IHNsdWcgPSB0cmltbWVkXG4gICAgICAgIC50b0xvd2VyQ2FzZSgpXG4gICAgICAgIC5yZXBsYWNlKC9bXmEtejAtOV0rL2csIFwiLVwiKVxuICAgICAgICAucmVwbGFjZSgvKF4tK3wtKyQpL2csIFwiXCIpO1xuICAgICAgaWYgKCFzbHVnKSByZXR1cm47XG4gICAgICBjb25zdCBpZCA9IGBjdXN0b20uJHtzbHVnfWA7XG4gICAgICBjb25zdCBzYXZlZDogVWlEcml2ZXIgPSB7IC4uLnVpRHJpdmVyQmFzZSwgaWQsIGxhYmVsOiB0cmltbWVkIH07XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1VJX0NVU1RPTV9EUklWRVJTXCIsIHZhbHVlOiAoY3VycmVudCkgPT4gKHsgLi4uY3VycmVudCwgW2lkXTogc2F2ZWQgfSkgfSk7XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1VJX0RSSVZFUl9EUkFGVFwiLCB2YWx1ZTogbnVsbCB9KTtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfVUlfRFJJVkVSX0lEXCIsIHZhbHVlOiBpZCB9KTtcbiAgICB9LFxuICAgIFt1aURyaXZlckJhc2VdLFxuICApO1xuXG4gIGNvbnN0IGRlbGV0ZURyaXZlciA9IHVzZUNhbGxiYWNrKChpZDogc3RyaW5nKSA9PiB7XG4gICAgaWYgKCFpZC5zdGFydHNXaXRoKFwiY3VzdG9tLlwiKSkgcmV0dXJuO1xuICAgIGRpc3BhdGNoKHtcbiAgICAgIHR5cGU6IFwiU0VUX1VJX0NVU1RPTV9EUklWRVJTXCIsXG4gICAgICB2YWx1ZTogKGN1cnJlbnQpID0+IHtcbiAgICAgICAgY29uc3QgeyBbaWRdOiBfcmVtb3ZlZCwgLi4ucmVzdCB9ID0gY3VycmVudDtcbiAgICAgICAgcmV0dXJuIHJlc3Q7XG4gICAgICB9LFxuICAgIH0pO1xuICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfVUlfRFJJVkVSX0lEXCIsIHZhbHVlOiAoY3VycmVudCkgPT4gKGN1cnJlbnQgPT09IGlkID8gREVGQVVMVF9VSV9EUklWRVIuaWQgOiBjdXJyZW50KSB9KTtcbiAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1VJX0RSSVZFUl9EUkFGVFwiLCB2YWx1ZTogbnVsbCB9KTtcbiAgfSwgW10pO1xuICAvLyNlbmRyZWdpb24g8J+al++4j0RyaXZlck11dGF0b3JzXG5cbiAgY29uc3QgW3RoZW1lU2F2ZUxhYmVsLCBzZXRUaGVtZVNhdmVMYWJlbF0gPSB1c2VTdGF0ZShcIlwiKTtcbiAgY29uc3QgW2RyaXZlclNhdmVMYWJlbCwgc2V0RHJpdmVyU2F2ZUxhYmVsXSA9IHVzZVN0YXRlKFwiXCIpO1xuICBjb25zdCBba2V5YmluZGluZ0NhcHR1cmVDb250cm9sSWQsIHNldEtleWJpbmRpbmdDYXB0dXJlQ29udHJvbElkXSA9IHVzZVN0YXRlPHN0cmluZyB8IG51bGw+KG51bGwpO1xuICBjb25zdCBzZXRLZXliaW5kaW5nT3ZlcnJpZGUgPSB1c2VDYWxsYmFjaygoY29udHJvbElkOiBzdHJpbmcsIGtleXM6IHN0cmluZykgPT4ge1xuICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfVUlfS0VZQklORElOR19PVkVSUklERVNcIiwgdmFsdWU6IChjdXJyZW50KSA9PiAoeyAuLi5jdXJyZW50LCBbY29udHJvbElkXToga2V5cyB9KSB9KTtcbiAgfSwgW10pO1xuICBjb25zdCByZXNldEtleWJpbmRpbmdPdmVycmlkZSA9IHVzZUNhbGxiYWNrKChjb250cm9sSWQ6IHN0cmluZykgPT4ge1xuICAgIGRpc3BhdGNoKHtcbiAgICAgIHR5cGU6IFwiU0VUX1VJX0tFWUJJTkRJTkdfT1ZFUlJJREVTXCIsXG4gICAgICB2YWx1ZTogKGN1cnJlbnQpID0+IHtcbiAgICAgICAgY29uc3QgeyBbY29udHJvbElkXTogX3JlbW92ZWQsIC4uLnJlc3QgfSA9IGN1cnJlbnQ7XG4gICAgICAgIHJldHVybiByZXN0O1xuICAgICAgfSxcbiAgICB9KTtcbiAgfSwgW10pO1xuICB1c2VFZmZlY3QoKCkgPT4ge1xuICAgIGNvbnN0IG9uTmF2aWdhdGVUb0hvdGtleSA9IChldmVudDogRXZlbnQpID0+IHtcbiAgICAgIGNvbnN0IHBhdGggPSAoZXZlbnQgYXMgQ3VzdG9tRXZlbnQ8eyByZWFkb25seSBwYXRoPzogc3RyaW5nIH0+KS5kZXRhaWw/LnBhdGg7XG4gICAgICBpZiAocGF0aCkgc2V0S2V5YmluZGluZ0NhcHR1cmVDb250cm9sSWQocGF0aCk7XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1BBTkVMX1ZJU0lCTEVcIiwgYW5jaG9yOiBcImJvdHRvbS1yaWdodFwiLCB2YWx1ZTogdHJ1ZSB9KTtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfUEFORUxfUEFUSFwiLCBhbmNob3I6IFwiYm90dG9tLXJpZ2h0XCIsIHZhbHVlOiBbXCJmcmFtZXdvcmsuc2V0dGluZ3Mua2V5YmluZGluZ3NcIl0gfSk7XG4gICAgfTtcbiAgICB3aW5kb3cuYWRkRXZlbnRMaXN0ZW5lcihcIm5hdmlnYXRlLXRvLWhvdGtleVwiLCBvbk5hdmlnYXRlVG9Ib3RrZXkpO1xuICAgIHJldHVybiAoKSA9PiB3aW5kb3cucmVtb3ZlRXZlbnRMaXN0ZW5lcihcIm5hdmlnYXRlLXRvLWhvdGtleVwiLCBvbk5hdmlnYXRlVG9Ib3RrZXkpO1xuICB9LCBbZGlzcGF0Y2hdKTtcbiAgY29uc3Qgc2V0dGluZ3NIb3N0UmVmID0gdXNlUmVmPFNldHRpbmdzSG9zdEFwaSB8IG51bGw+KG51bGwpO1xuICBjb25zdCBzZXR0aW5nc0hvc3Q6IFNldHRpbmdzSG9zdEFwaSA9IHVzZU1lbW8oXG4gICAgKCkgPT4gKHtcbiAgICAgIGFwcElkOiBzZXNzaW9uPy5hcHAuaWQsXG4gICAgICBhcHBMYWJlbDogc2Vzc2lvbiA/IGFwcERvY3VtZW50TGFiZWwocmVzb2x2ZUFwcERvY3VtZW50KHNlc3Npb24uYXBwLCB1aVRlcm1pbm9sb2d5KSkgOiB1bmRlZmluZWQsXG4gICAgICBjb250cm9sbGVySWQ6IHNlc3Npb24/LmFwcC5jb250cm9sbGVySWQsXG4gICAgICBwbHVnaW5JZDogc2Vzc2lvbj8ucGx1Z2luSWQsXG4gICAgICBkcml2ZXJJZDogdWlEcml2ZXJJZCxcbiAgICAgIGRyaXZlcjogdWlEcml2ZXJCYXNlLFxuICAgICAgZHJpdmVyRGlydHk6IHVpRHJpdmVyRGlydHksXG4gICAgICBkcml2ZXJzOiB1aURyaXZlckxpc3QsXG4gICAgICBzZXREcml2ZXJJZCxcbiAgICAgIHNldERyaXZlckZpZWxkLFxuICAgICAgc2F2ZURyaXZlcixcbiAgICAgIGRlbGV0ZURyaXZlcixcbiAgICAgIGRyaXZlclNhdmVMYWJlbCxcbiAgICAgIHNldERyaXZlclNhdmVMYWJlbCxcbiAgICAgIGFwcGVhcmFuY2U6IHVpQXBwZWFyYW5jZSxcbiAgICAgIHNldEFwcGVhcmFuY2U6ICh2YWx1ZTogc3RyaW5nKSA9PiB7XG4gICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfVUlfQVBQRUFSQU5DRVwiLCB2YWx1ZTogdmFsdWUgYXMgRWxlbWVudHNTdXJmYWNlQXBwZWFyYW5jZSB9KTtcbiAgICAgICAgbm90ZU9zQ29tbWFuZChcIm9zLnNldEFwcGVhcmFuY2VcIiwgeyBhcHBlYXJhbmNlOiB2YWx1ZSB9KTtcbiAgICAgIH0sXG4gICAgICBsYXlvdXQ6IHVpTGF5b3V0LFxuICAgICAgc2V0TGF5b3V0OiAodmFsdWU6IFVpQ2hyb21lTGF5b3V0KSA9PiB7XG4gICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfVUlfTEFZT1VUXCIsIHZhbHVlIH0pO1xuICAgICAgICBub3RlT3NDb21tYW5kKFwib3Muc2V0TGF5b3V0XCIsIHsgbGF5b3V0OiB2YWx1ZSB9KTtcbiAgICAgIH0sXG4gICAgICBtb2JpbGVBY3RpdmU6IG1vYmlsZSxcbiAgICAgIG9uUmVzZXREb2NrOiAoKSA9PiB7XG4gICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJSRVNFVF9ET0NLXCIgfSk7XG4gICAgICAgIGRvY2tMYXlvdXRTdG9yZS5yZXNldCgpO1xuICAgICAgICBkb2NrVWlTdGF0ZVN0b3JlLnJlc2V0KCk7XG4gICAgICAgIG5vdGVPc0NvbW1hbmQoXCJvcy5yZXNldERvY2tcIik7XG4gICAgICB9LFxuICAgICAgbG9jYWxlOiB1aUxvY2FsZSxcbiAgICAgIHNldExvY2FsZTogKHZhbHVlOiBVaUxvY2FsZSkgPT4ge1xuICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1VJX0xPQ0FMRVwiLCB2YWx1ZSB9KTtcbiAgICAgICAgbm90ZU9zQ29tbWFuZChcIm9zLnNldExvY2FsZVwiLCB7IGxvY2FsZTogdmFsdWUgfSk7XG4gICAgICB9LFxuICAgICAgdGVybWlub2xvZ3k6IHVpVGVybWlub2xvZ3ksXG4gICAgICBzZXRUZXJtaW5vbG9neTogKHZhbHVlOiBzdHJpbmcpID0+IHtcbiAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9VSV9URVJNSU5PTE9HWVwiLCB2YWx1ZSB9KTtcbiAgICAgICAgbm90ZU9zQ29tbWFuZChcIm9zLnNldFRlcm1pbm9sb2d5XCIsIHsgdGVybWlub2xvZ3k6IHZhbHVlIH0pO1xuICAgICAgfSxcbiAgICAgIHRlcm1pbm9sb2dpZXM6IFtVSV9URVJNSU5PTE9HWV9OQVRJVkUsIC4uLihzZXNzaW9uPy5hcHAudGVybWlub2xvZ2llcyA/PyBbXSldLFxuICAgICAgdGhlbWU6IHVpVGhlbWVCYXNlLFxuICAgICAgdGhlbWVJZDogdWlUaGVtZUlkLFxuICAgICAgdGhlbWVEaXJ0eTogdWlUaGVtZURpcnR5LFxuICAgICAgdGhlbWVzOiB1aVRoZW1lTGlzdCxcbiAgICAgIHNldFRoZW1lSWQsXG4gICAgICBzZXRUaGVtZUNvbG9yLFxuICAgICAgc2V0VGhlbWVTcGFjaW5nLFxuICAgICAgc2V0VGhlbWVGb250U3RhY2ssXG4gICAgICBzZXRUaGVtZVN0cm9rZSxcbiAgICAgIHNldFRoZW1lUmFkaXVzLFxuICAgICAgc2V0VGhlbWVPcGFjaXR5LFxuICAgICAgc2V0VGhlbWVNZXRyaWMsXG4gICAgICBzZXRUaGVtZUFwcGVhcmFuY2VQYWludCxcbiAgICAgIHNhdmVUaGVtZSxcbiAgICAgIGRlbGV0ZVRoZW1lLFxuICAgICAgcmVzZXRUaGVtZSxcbiAgICAgIGV4cG9ydFRoZW1lLFxuICAgICAgaW1wb3J0VGhlbWUsXG4gICAgICB0aGVtZVNhdmVMYWJlbCxcbiAgICAgIHNldFRoZW1lU2F2ZUxhYmVsLFxuICAgICAgY29udHJvbEtleWJpbmRpbmdzLFxuICAgICAga2V5YmluZGluZ0NhcHR1cmVDb250cm9sSWQsXG4gICAgICBzZXRLZXliaW5kaW5nQ2FwdHVyZUNvbnRyb2xJZCxcbiAgICAgIHNldEtleWJpbmRpbmdPdmVycmlkZSxcbiAgICAgIHJlc2V0S2V5YmluZGluZ092ZXJyaWRlLFxuICAgICAgbG9ja3MsXG4gICAgfSksXG4gICAgW1xuICAgICAgc2Vzc2lvbixcbiAgICAgIGRvY2tMYXlvdXRTdG9yZSxcbiAgICAgIHVpRHJpdmVySWQsXG4gICAgICB1aURyaXZlckJhc2UsXG4gICAgICB1aURyaXZlckRpcnR5LFxuICAgICAgdWlEcml2ZXJMaXN0LFxuICAgICAgc2V0RHJpdmVySWQsXG4gICAgICBzZXREcml2ZXJGaWVsZCxcbiAgICAgIHNhdmVEcml2ZXIsXG4gICAgICBkZWxldGVEcml2ZXIsXG4gICAgICBkcml2ZXJTYXZlTGFiZWwsXG4gICAgICBzZXREcml2ZXJTYXZlTGFiZWwsXG4gICAgICBjb250cm9sS2V5YmluZGluZ3MsXG4gICAgICBrZXliaW5kaW5nQ2FwdHVyZUNvbnRyb2xJZCxcbiAgICAgIHNldEtleWJpbmRpbmdPdmVycmlkZSxcbiAgICAgIHJlc2V0S2V5YmluZGluZ092ZXJyaWRlLFxuICAgICAgdWlBcHBlYXJhbmNlLFxuICAgICAgdWlMYXlvdXQsXG4gICAgICBtb2JpbGUsXG4gICAgICB1aUxvY2FsZSxcbiAgICAgIHVpVGVybWlub2xvZ3ksXG4gICAgICB1aVRoZW1lQmFzZSxcbiAgICAgIHVpVGhlbWVJZCxcbiAgICAgIHVpVGhlbWVEaXJ0eSxcbiAgICAgIHVpVGhlbWVMaXN0LFxuICAgICAgbG9ja3MsXG4gICAgICBzZXRUaGVtZUlkLFxuICAgICAgc2V0VGhlbWVDb2xvcixcbiAgICAgIHNldFRoZW1lU3BhY2luZyxcbiAgICAgIHNldFRoZW1lRm9udFN0YWNrLFxuICAgICAgc2V0VGhlbWVTdHJva2UsXG4gICAgICBzZXRUaGVtZVJhZGl1cyxcbiAgICAgIHNldFRoZW1lT3BhY2l0eSxcbiAgICAgIHNldFRoZW1lTWV0cmljLFxuICAgICAgc2V0VGhlbWVBcHBlYXJhbmNlUGFpbnQsXG4gICAgICBzYXZlVGhlbWUsXG4gICAgICBkZWxldGVUaGVtZSxcbiAgICAgIHJlc2V0VGhlbWUsXG4gICAgICBleHBvcnRUaGVtZSxcbiAgICAgIGltcG9ydFRoZW1lLFxuICAgICAgdGhlbWVTYXZlTGFiZWwsXG4gICAgICBzZXRUaGVtZVNhdmVMYWJlbCxcbiAgICAgIG5vdGVPc0NvbW1hbmQsXG4gICAgXSxcbiAgKTtcbiAgc2V0dGluZ3NIb3N0UmVmLmN1cnJlbnQgPSBzZXR0aW5nc0hvc3Q7XG5cbiAgY29uc3QgZnJhbWV3b3JrRGlzcGxheVRhYnMgPSB1c2VNZW1vKCgpID0+IGNyZWF0ZUZyYW1ld29ya0Rpc3BsYXlQYW5lbFRhYnMoKCkgPT4gZGlzcGxheUhvc3RSZWYuY3VycmVudCksIFtkaXNwbGF5SG9zdCwgdWlMb2NhbGVdKTtcbiAgY29uc3QgZnJhbWV3b3JrU2V0dGluZ3NUYWJzID0gdXNlTWVtbygoKSA9PiBjcmVhdGVGcmFtZXdvcmtTZXR0aW5nc1BhbmVsVGFicygoKSA9PiBzZXR0aW5nc0hvc3RSZWYuY3VycmVudCksIFtzZXR0aW5nc0hvc3RdKTtcblxuICBjb25zdCBwbHVnaW5zSG9zdFJlZiA9IHVzZVJlZjxQbHVnaW5zSG9zdEFwaSB8IG51bGw+KG51bGwpO1xuICBjb25zdCBwbHVnaW5zSG9zdDogUGx1Z2luc0hvc3RBcGkgPSB1c2VNZW1vKFxuICAgICgpID0+ICh7XG4gICAgICBwbHVnaW5zOiByZWdpc3RyeS5tYXAoKGVudHJ5KTogUGx1Z2luc1BhbmVsRW50cnkgPT4ge1xuICAgICAgICBjb25zdCBsb2FkZWRFbnRyeSA9IGxvYWRlZFBsdWdpbnMuZmluZCgoY2FuZGlkYXRlKSA9PiBjYW5kaWRhdGUuaGFuZGxlLnBsdWdpbklkID09PSBlbnRyeS5wbHVnaW5JZCk7XG4gICAgICAgIHJldHVybiB7XG4gICAgICAgICAgcGx1Z2luSWQ6IGVudHJ5LnBsdWdpbklkLFxuICAgICAgICAgIGxhYmVsOiBsb2FkZWRFbnRyeT8ubWFuaWZlc3QubGFiZWwgPz8gZW50cnkucGx1Z2luSWQsXG4gICAgICAgICAgdmVyc2lvbjogbG9hZGVkRW50cnk/Lm1hbmlmZXN0LnZlcnNpb24sXG4gICAgICAgICAgc3RhdHVzOiBwbHVnaW5TdGF0dXNCeUlkW2VudHJ5LnBsdWdpbklkXSA/PyBcImF2YWlsYWJsZVwiLFxuICAgICAgICAgIHNvdXJjZUlkOiBwbHVnaW5Tb3VyY2UuaWQsXG4gICAgICAgICAgY2FuVW5pbnN0YWxsOiBlbnRyeS5wbHVnaW5JZCAhPT0gcHJpbWFyeVBsdWdpbklkICYmIHNlc3Npb24/LnBsdWdpbklkICE9PSBlbnRyeS5wbHVnaW5JZCxcbiAgICAgICAgfTtcbiAgICAgIH0pLFxuICAgICAgaW5zdGFsbDogKHBsdWdpbklkKSA9PiB2b2lkIGluc3RhbGxQbHVnaW4ocGx1Z2luSWQpLFxuICAgICAgdW5pbnN0YWxsOiAocGx1Z2luSWQpID0+IHZvaWQgdW5pbnN0YWxsUGx1Z2luKHBsdWdpbklkKSxcbiAgICAgIHJlbG9hZDogKHBsdWdpbklkKSA9PiB2b2lkIHJlbG9hZFBsdWdpbihwbHVnaW5JZCksXG4gICAgfSksXG4gICAgW3JlZ2lzdHJ5LCBsb2FkZWRQbHVnaW5zLCBwbHVnaW5TdGF0dXNCeUlkLCBwbHVnaW5Tb3VyY2UsIHByaW1hcnlQbHVnaW5JZCwgc2Vzc2lvbj8ucGx1Z2luSWQsIGluc3RhbGxQbHVnaW4sIHVuaW5zdGFsbFBsdWdpbiwgcmVsb2FkUGx1Z2luXSxcbiAgKTtcbiAgcGx1Z2luc0hvc3RSZWYuY3VycmVudCA9IHBsdWdpbnNIb3N0O1xuICBjb25zdCBmcmFtZXdvcmtQbHVnaW5zVGFicyA9IHVzZU1lbW8oKCkgPT4gY3JlYXRlRnJhbWV3b3JrUGx1Z2luc1BhbmVsVGFicygoKSA9PiBwbHVnaW5zSG9zdFJlZi5jdXJyZW50KSwgW3BsdWdpbnNIb3N0XSk7XG5cbiAgLy8g8J+Qmu+4jyBHYXRlZCB0byB0aGlzIHNoZWxsIHZpYSBgdXNlU2hlbGxLZXlkb3duYCBiZWxvdyDigJQgd2FzIGFuIHVuY29uZGl0aW9uYWwgYHdpbmRvd2Aga2V5ZG93biBsaXN0ZW5lcixcbiAgLy8gc28gZXZlcnkgbW91bnRlZCBzaGVsbCBmaXJlZCBpdHMgYm91bmQgYWN0aW9uIChhbmQgY291bGQgYHByZXZlbnREZWZhdWx0KClgIG91dCBmcm9tIHVuZGVyIGFub3RoZXJcbiAgLy8gc2hlbGwpIGZvciBldmVyeSBrZXlzdHJva2Ugb24gdGhlIHBhZ2UgcmVnYXJkbGVzcyBvZiB3aGljaCBzaGVsbCB0aGUgdXNlciB3YXMgYWN0dWFsbHkgdXNpbmcuXG4gIGNvbnN0IGhhbmRsZUFwcEtleWRvd24gPSB1c2VDYWxsYmFjayhcbiAgICAoZXZlbnQ6IEtleWJvYXJkRXZlbnQpID0+IHtcbiAgICAgIGlmICghc2Vzc2lvbikgcmV0dXJuO1xuICAgICAgY29uc3QgcGFyc2VLZXlzID0gKGtleXM6IHN0cmluZykgPT5cbiAgICAgICAga2V5c1xuICAgICAgICAgIC5zcGxpdChcIixcIilcbiAgICAgICAgICAubWFwKChrZXkpID0+IGtleS50cmltKCkudG9Mb3dlckNhc2UoKSlcbiAgICAgICAgICAuZmlsdGVyKEJvb2xlYW4pO1xuICAgICAgY29uc3QgaXNFZGl0YWJsZVRhcmdldCA9ICh0YXJnZXQ6IEV2ZW50VGFyZ2V0IHwgbnVsbCkgPT4ge1xuICAgICAgICBpZiAoISh0YXJnZXQgaW5zdGFuY2VvZiBIVE1MRWxlbWVudCkpIHJldHVybiBmYWxzZTtcbiAgICAgICAgY29uc3QgdGFnID0gdGFyZ2V0LnRhZ05hbWU7XG4gICAgICAgIGlmICh0YWcgPT09IFwiSU5QVVRcIiB8fCB0YWcgPT09IFwiVEVYVEFSRUFcIiB8fCB0YWcgPT09IFwiU0VMRUNUXCIpIHJldHVybiB0cnVlO1xuICAgICAgICBpZiAodGFyZ2V0LmlzQ29udGVudEVkaXRhYmxlKSByZXR1cm4gdHJ1ZTtcbiAgICAgICAgcmV0dXJuIHRhcmdldC5jbG9zZXN0KFwiW2NvbnRlbnRlZGl0YWJsZT0ndHJ1ZSddLCBbcm9sZT0ndGV4dGJveCddXCIpICE9IG51bGw7XG4gICAgICB9O1xuICAgICAgY29uc3QgbWF0Y2hlcyA9IChldmVudDogS2V5Ym9hcmRFdmVudCwgYmluZGluZzogc3RyaW5nKSA9PiB7XG4gICAgICAgIGNvbnN0IHBhcnRzID0gYmluZGluZy5zcGxpdChcIitcIikubWFwKChwYXJ0KSA9PiBwYXJ0LnRyaW0oKSk7XG4gICAgICAgIGNvbnN0IGtleSA9IHBhcnRzW3BhcnRzLmxlbmd0aCAtIDFdID8/IFwiXCI7XG4gICAgICAgIGNvbnN0IG5lZWRzQ3RybCA9IHBhcnRzLmluY2x1ZGVzKFwiY3RybFwiKSB8fCBwYXJ0cy5pbmNsdWRlcyhcIm1ldGFcIikgfHwgcGFydHMuaW5jbHVkZXMoXCJtb2RcIik7XG4gICAgICAgIGNvbnN0IG5lZWRzU2hpZnQgPSBwYXJ0cy5pbmNsdWRlcyhcInNoaWZ0XCIpO1xuICAgICAgICBjb25zdCBuZWVkc0FsdCA9IHBhcnRzLmluY2x1ZGVzKFwiYWx0XCIpO1xuICAgICAgICBjb25zdCBoYXNDdHJsID0gZXZlbnQuY3RybEtleSB8fCBldmVudC5tZXRhS2V5O1xuICAgICAgICBpZiAobmVlZHNDdHJsICE9PSBoYXNDdHJsKSByZXR1cm4gZmFsc2U7XG4gICAgICAgIGlmIChuZWVkc1NoaWZ0ICE9PSBldmVudC5zaGlmdEtleSkgcmV0dXJuIGZhbHNlO1xuICAgICAgICBpZiAobmVlZHNBbHQgIT09IGV2ZW50LmFsdEtleSkgcmV0dXJuIGZhbHNlO1xuICAgICAgICByZXR1cm4gZXZlbnQua2V5LnRvTG93ZXJDYXNlKCkgPT09IGtleTtcbiAgICAgIH07XG4gICAgICBjb25zdCBhY3Rpb25CeUlkID0gbmV3IE1hcChzZXNzaW9uLmFwcC5hY3Rpb25zLm1hcCgoYWN0aW9uKSA9PiBbYWN0aW9uLmlkLCBhY3Rpb25dKSk7XG4gICAgICBpZiAoaXNFZGl0YWJsZVRhcmdldChldmVudC50YXJnZXQpKSByZXR1cm47XG4gICAgICAvLyDwn6ew77iP8J+boO+4jyBFc2NhcGUgZGVhY3RpdmF0ZXMgdGhlIGFjdGl2ZSB3aW5kb3cncyBhY3RpdmUgdXRpbGl0eSAoUDUpLCBvciDigJQgd2hlbiBubyB1dGlsaXR5IGlzIGFjdGl2ZSDigJRcbiAgICAgIC8vIHRoZSBhY3RpdmUgbW9kZS1sZXZlbCB0b29sLCB3aGVuIG5vdGhpbmcgaXMgYmVpbmcgdHlwZWQuXG4gICAgICBpZiAoZXZlbnQua2V5ID09PSBcIkVzY2FwZVwiKSB7XG4gICAgICAgIGNvbnN0IHdpbmRvd0lkID0gYWN0aXZlV2luZG93SWRSZWYuY3VycmVudDtcbiAgICAgICAgaWYgKHdpbmRvd0lkICYmIGFjdGl2ZVV0aWxpdHlCeVdpbmRvd0lkUmVmLmN1cnJlbnRbd2luZG93SWRdKSB7XG4gICAgICAgICAgZXZlbnQucHJldmVudERlZmF1bHQoKTtcbiAgICAgICAgICBvbkFjdGlvbih7IGNvbnRyb2xsZXJJZDogc2Vzc2lvbi5hcHAuY29udHJvbGxlcklkLCBhY3Rpb246IFNFVF9BQ1RJVkVfVVRJTElUWV9BQ1RJT05fSUQsIGFyZ3M6IHsgd2luZG93SWQsIHV0aWxpdHlJZDogXCJcIiB9IH0pO1xuICAgICAgICAgIHJldHVybjtcbiAgICAgICAgfVxuICAgICAgICBpZiAoYWN0aXZlVG9vbElkUmVmLmN1cnJlbnQpIHtcbiAgICAgICAgICBldmVudC5wcmV2ZW50RGVmYXVsdCgpO1xuICAgICAgICAgIG9uQWN0aW9uKHsgY29udHJvbGxlcklkOiBzZXNzaW9uLmFwcC5jb250cm9sbGVySWQsIGFjdGlvbjogU0VUX0FDVElWRV9UT09MX0FDVElPTl9JRCwgYXJnczogeyB0b29sSWQ6IFwiXCIgfSB9KTtcbiAgICAgICAgICByZXR1cm47XG4gICAgICAgIH1cbiAgICAgIH1cbiAgICAgIGZvciAoY29uc3QgYmluZGluZyBvZiBzZXNzaW9uLmFwcC5rZXliaW5kaW5ncykge1xuICAgICAgICBmb3IgKGNvbnN0IGNob3JkIG9mIHBhcnNlS2V5cyhiaW5kaW5nLmtleXMpKSB7XG4gICAgICAgICAgaWYgKCFtYXRjaGVzKGV2ZW50LCBjaG9yZCkpIGNvbnRpbnVlO1xuICAgICAgICAgIGV2ZW50LnByZXZlbnREZWZhdWx0KCk7XG4gICAgICAgICAgLy8g4pyN77iPIEFyZy1jYXJyeWluZyBob3RrZXlzIG5ldmVyIHNpbGVudC1maXJlIGRlZmF1bHRzIChQNCk6IG9wZW4gdGhlIHN0YWdlZCBmb3JtLCBvciDigJQgaWYgdGhhdFxuICAgICAgICAgIC8vIGZvcm0gaXMgYWxyZWFkeSBleHBhbmRlZCBpbiB0aGUgYWN0aXZlIHdpbmRvdyDigJQgdHJlYXQgdGhlIGhvdGtleSBhcyBFeGVjdXRlICh3aXRoIHZhbGlkYXRpb24pLlxuICAgICAgICAgIGNvbnN0IGRlZmluaXRpb24gPSBhY3Rpb25CeUlkLmdldChiaW5kaW5nLmFjdGlvbi5hY3Rpb24pO1xuICAgICAgICAgIGlmIChkZWZpbml0aW9uICYmIGFjdGlvblJlcXVpcmVzU3RhZ2VkRm9ybShkZWZpbml0aW9uKSkge1xuICAgICAgICAgICAgY29uc3Qgd2luZG93SWQgPSBhY3RpdmVXaW5kb3dJZFJlZi5jdXJyZW50O1xuICAgICAgICAgICAgaWYgKCF3aW5kb3dJZCkgcmV0dXJuO1xuICAgICAgICAgICAgY29uc3QgZXhwYW5kZWQgPSBhY3Rpb25QYW5lRXhwYW5kZWRCeVdpbmRvd0lkUmVmLmN1cnJlbnRbd2luZG93SWRdID8/IG51bGw7XG4gICAgICAgICAgICBjb25zdCBzdGFnZWQgPSBhY3Rpb25QYW5lU3RhZ2VkQXJnc0J5S2V5UmVmLmN1cnJlbnRbYWN0aW9uU3RhZ2VLZXkod2luZG93SWQsIGRlZmluaXRpb24uaWQpXSA/PyB7fTtcbiAgICAgICAgICAgIGNvbnN0IGludGVudCA9IHJlc29sdmVLZXliaW5kaW5nSW50ZW50KGRlZmluaXRpb24sIGV4cGFuZGVkLCBzdGFnZWQpO1xuICAgICAgICAgICAgaWYgKGludGVudC5raW5kID09PSBcImV4ZWN1dGVcIikge1xuICAgICAgICAgICAgICBvbkFjdGlvbih7IGNvbnRyb2xsZXJJZDogc2Vzc2lvbi5hcHAuY29udHJvbGxlcklkLCBhY3Rpb246IGludGVudC5hY3Rpb25JZCwgYXJnczogaW50ZW50LmFyZ3MgfSk7XG4gICAgICAgICAgICB9IGVsc2UgaWYgKGludGVudC5raW5kID09PSBcIm9wZW5cIikge1xuICAgICAgICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0FDVElPTl9QQU5FX0ZPTERFRFwiLCB3aW5kb3dJZCwgdmFsdWU6IGZhbHNlIH0pO1xuICAgICAgICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0FDVElPTl9QQU5FX0VYUEFOREVEXCIsIHdpbmRvd0lkLCB2YWx1ZTogaW50ZW50LmFjdGlvbklkIH0pO1xuICAgICAgICAgICAgfVxuICAgICAgICAgICAgcmV0dXJuO1xuICAgICAgICAgIH1cbiAgICAgICAgICBvbkFjdGlvbihiaW5kaW5nLmFjdGlvbik7XG4gICAgICAgICAgcmV0dXJuO1xuICAgICAgICB9XG4gICAgICB9XG4gICAgfSxcbiAgICBbb25BY3Rpb24sIHNlc3Npb25dLFxuICApO1xuICB1c2VTaGVsbEtleWRvd24oc2NvcGUucm9vdFJlZiwgaGFuZGxlQXBwS2V5ZG93biwgW2hhbmRsZUFwcEtleWRvd25dKTtcblxuICBjb25zdCBhY3RpdmVSaWdodFBhbmVsVGFiID0gc2Vzc2lvbj8uYXBwLnBhbmVsVGFicy5maW5kKCh0YWIpID0+IHBhbmVsQW5jaG9yRm9yR3JvdXAodGFiLmdyb3VwKSA9PT0gXCJ0b3AtcmlnaHRcIik7XG4gIGNvbnN0IGFjdGl2ZVBhbmVsVGFiSWQgPSBwYW5lbD8uYWN0aXZlUGFuZWxUYWIgPz8gKGFjdGl2ZVJpZ2h0UGFuZWxUYWIgPyBwYW5lbFRhYktpbmRJZChhY3RpdmVSaWdodFBhbmVsVGFiLmtpbmQpIDogdW5kZWZpbmVkKSA/PyAoc2Vzc2lvbj8uYXBwLnBhbmVsVGFic1swXSA/IHBhbmVsVGFiS2luZElkKHNlc3Npb24uYXBwLnBhbmVsVGFic1swXS5raW5kKSA6IHVuZGVmaW5lZCk7XG5cbiAgY29uc3Qgd29ya2JlbmNoTGVmdFRhYnMgPSB1c2VNZW1vKCgpOiBQYW5lbFRhYk5vZGVbXSA9PiB7XG4gICAgaWYgKCFzZXNzaW9uKSByZXR1cm4gW107XG4gICAgY29uc3QgcGx1Z2luTGVmdFRhYnMgPSBzZXNzaW9uLmFwcC5wYW5lbFRhYnMuZmlsdGVyKCh0YWIpID0+IHBhbmVsQW5jaG9yRm9yR3JvdXAodGFiLmdyb3VwKSA9PT0gXCJ0b3AtbGVmdFwiKS5tYXAoKHRhYiwgb3JkZXIpID0+IHBhbmVsVGFiRGVmaW5pdGlvblRvTm9kZSh0YWIsIHRhYi5ncm91cCwgcGFuZWxVaUJ5S2V5LCBvbkFjdGlvbiwgb3JkZXIsIGFwcExhYmVsc092ZXJsYXksIHVpVGVybWlub2xvZ3ksIHVpTG9jYWxlKSk7XG4gICAgaWYgKHN0dWRpb01vZGUgJiYgc2Vzc2lvbi5hcHAuaWQgPT09IGhvc3RBcHBJZCAmJiBwbHVnaW5MZWZ0VGFicy5sZW5ndGggPiAwKSByZXR1cm4gcGx1Z2luTGVmdFRhYnM7XG4gICAgY29uc3QgaGFzUGx1Z2luRG9jdW1lbnRUYWIgPSBwbHVnaW5MZWZ0VGFicy5zb21lKCh0YWIpID0+IHRhYi5pZCA9PT0gRlJBTUVXT1JLX1BBTkVMX1RBQl9ET0NVTUVOVF9JRCk7XG4gICAgaWYgKGhhc1BsdWdpbkRvY3VtZW50VGFiKSByZXR1cm4gcGx1Z2luTGVmdFRhYnM7XG4gICAgY29uc3QgZG9jdW1lbnRUYWIgPSBzaW5nbGVUcmVlTGVhZih7XG4gICAgICBpZDogRlJBTUVXT1JLX1BBTkVMX1RBQl9ET0NVTUVOVF9JRCxcbiAgICAgIGljb246IHNoZWxsVGFiSWNvbihGUkFNRVdPUktfUEFORUxfVEFCX0RPQ1VNRU5UX0lDT05fSUQpLFxuICAgICAgbmFtZTogc2hlbGxMYWJlbChcInVpLnBhbmVsLmRvY3VtZW50XCIpLFxuICAgICAgb3JkZXI6IDAsXG4gICAgICB0cmVlOiBzdGF0aWNUcmVlUGFuZWxEZWZpbml0aW9uKHtcbiAgICAgICAgc2VjdGlvbnM6IFtcbiAgICAgICAgICB7XG4gICAgICAgICAgICBpZDogXCJkb2N1bWVudC5yb290XCIsXG4gICAgICAgICAgICBsYWJlbDogc2hlbGxMYWJlbChcInVpLnBhbmVsLmRvY3VtZW50XCIpLFxuICAgICAgICAgICAgaXRlbXM6IFt7IGlkOiBcImRvY3VtZW50LmVtcHR5XCIsIGxhYmVsOiBzdHVkaW9Nb2RlID8gYCR7cGFuZWw/LnNwYXduZWRBcHBzLmxlbmd0aCA/PyAwfSAke3NoZWxsTGFiZWwoXCJ1aS5wYW5lbC5zcGF3bmVkQXBwc1N1ZmZpeFwiKX1gIDogc2hlbGxMYWJlbChcInVpLnBhbmVsLmRvY3VtZW50RW1wdHlcIikgfV0sXG4gICAgICAgICAgfSxcbiAgICAgICAgXSxcbiAgICAgIH0pLFxuICAgIH0pO1xuICAgIHJldHVybiBbZG9jdW1lbnRUYWIsIC4uLnBsdWdpbkxlZnRUYWJzXTtcbiAgfSwgW2FwcExhYmVsc092ZXJsYXksIG9uQWN0aW9uLCBwYW5lbD8uc3Bhd25lZEFwcHMubGVuZ3RoLCBwYW5lbFVpQnlLZXksIHNlc3Npb24sIHN0dWRpb01vZGUsIHVpTG9jYWxlLCB1aVRlcm1pbm9sb2d5LCBob3N0QXBwSWRdKTtcblxuICBjb25zdCBkZXRhaWxzUmlnaHRUYWJzID0gdXNlTWVtbygoKTogUGFuZWxUYWJOb2RlW10gPT4ge1xuICAgIGlmICghc2Vzc2lvbikgcmV0dXJuIFtdO1xuICAgIHJldHVybiBzZXNzaW9uLmFwcC5wYW5lbFRhYnMuZmlsdGVyKCh0YWIpID0+IHBhbmVsQW5jaG9yRm9yR3JvdXAodGFiLmdyb3VwKSA9PT0gXCJ0b3AtcmlnaHRcIikubWFwKCh0YWIsIG9yZGVyKSA9PiBwYW5lbFRhYkRlZmluaXRpb25Ub05vZGUodGFiLCB0YWIuZ3JvdXAsIHBhbmVsVWlCeUtleSwgb25BY3Rpb24sIG9yZGVyLCBhcHBMYWJlbHNPdmVybGF5LCB1aVRlcm1pbm9sb2d5LCB1aUxvY2FsZSkpO1xuICB9LCBbYXBwTGFiZWxzT3ZlcmxheSwgb25BY3Rpb24sIHBhbmVsVWlCeUtleSwgc2Vzc2lvbiwgdWlUZXJtaW5vbG9neSwgdWlMb2NhbGVdKTtcblxuICBjb25zdCBzZXR0aW5nc1JpZ2h0VGFicyA9IHVzZU1lbW8oKCk6IFBhbmVsVGFiTm9kZVtdID0+IGZyYW1ld29ya1NldHRpbmdzVGFicywgW2ZyYW1ld29ya1NldHRpbmdzVGFic10pO1xuXG4gIC8vI3JlZ2lvbiDwn6ew77iPRm9vdGVyVXRpbGl0eUxlYXZlcyDigJQgYm90dG9tLXJpZ2h0J3MgSGlzdG9yeSB0YWIsIHNvdXJjZWQgZnJvbSB0aGUgZnJhbWV3b3JrLWluamVjdGVkXG4gIC8vIGBmcmFtZXdvcmsucGFuZWwuaGlzdG9yeWAgcGFuZWwgdGFiIChldmVyeSBhcHAgZ2V0cyBvbmUg4oCUIHNlZSBgQXBwQnVpbGRlcjo6YnVpbGRfZGVmaW5pdGlvbmApLlxuICBjb25zdCBmcmFtZXdvcmtVdGlsaXRpZXNIaXN0b3J5VGFiID0gdXNlTWVtbygoKTogUGFuZWxUYWJOb2RlIHwgbnVsbCA9PiB7XG4gICAgaWYgKCFzZXNzaW9uKSByZXR1cm4gbnVsbDtcbiAgICBjb25zdCB0YWIgPSBzZXNzaW9uLmFwcC5wYW5lbFRhYnMuZmluZCgoY2FuZGlkYXRlKSA9PiBwYW5lbFRhYktpbmRJZChjYW5kaWRhdGUua2luZCkgPT09IEZSQU1FV09SS19QQU5FTF9UQUJfSElTVE9SWV9JRCk7XG4gICAgaWYgKCF0YWIpIHJldHVybiBudWxsO1xuICAgIHJldHVybiBwYW5lbFRhYkRlZmluaXRpb25Ub05vZGUodGFiLCB0YWIuZ3JvdXAsIHBhbmVsVWlCeUtleSwgb25BY3Rpb24sIDEsIGFwcExhYmVsc092ZXJsYXksIHVpVGVybWlub2xvZ3ksIHVpTG9jYWxlKTtcbiAgfSwgW2FwcExhYmVsc092ZXJsYXksIG9uQWN0aW9uLCBwYW5lbFVpQnlLZXksIHNlc3Npb24sIHVpVGVybWlub2xvZ3ksIHVpTG9jYWxlXSk7XG4gIC8vI2VuZHJlZ2lvbiDwn6ew77iPRm9vdGVyVXRpbGl0eUxlYXZlc1xuXG4gIC8vI3JlZ2lvbiDwn5SE77iPU3luY0xlYWYg4oCUIGJvdHRvbS1sZWZ0J3Mgc3luYyB0YWIsIHJlcGxhY2luZyB0aGUgb2xkIGZsb2F0aW5nIGZvb3RlciBTeW5jQXR0YWNoQ2FyZC5cbiAgY29uc3QgZnJhbWV3b3JrU3luY1RhYiA9IHVzZU1lbW8oKCk6IFBhbmVsVGFiTm9kZSB8IG51bGwgPT4ge1xuICAgIGNvbnN0IHN5bmNVdGlsaXRpZXMgPSBidWlsZEZyYW1ld29ya1N5bmNVdGlsaXRpZXMoc3luY0JhY2tib25lVXJpKSBhcyByZWFkb25seSBVdGlsaXR5Tm9kZVtdO1xuICAgIGlmICghc3luY1V0aWxpdGllcy5sZW5ndGgpIHJldHVybiBudWxsO1xuICAgIGNvbnN0IHN5bmNTdGF0dXMgPSBzeW5jQmFja2JvbmVVcmkgPyAoc3luY1N0YXR1c0J5RG9jdW1lbnRJZFtzeW5jQmFja2JvbmVVcmkucmVwbGFjZSgvXmFjdG9yOlxcL1xcLy8sIFwiXCIpXSA/PyBudWxsKSA6IG51bGw7XG4gICAgcmV0dXJuIHNpbmdsZVRyZWVMZWFmKHtcbiAgICAgIGlkOiBcImZyYW1ld29yay5zeW5jXCIsXG4gICAgICBpY29uOiBzaGVsbFRhYkljb24oVVRJTElUWV9DQVRFR09SWV9JQ09OX0lELnN5bmMpLFxuICAgICAgbmFtZTogc2hlbGxMYWJlbChcInVpLnBhbmVsLnN5bmNcIiksXG4gICAgICBvcmRlcjogMCxcbiAgICAgIHRyZWU6IHtcbiAgICAgICAgc2VjdGlvbnM6IFtcbiAgICAgICAgICB7XG4gICAgICAgICAgICBpZDogXCJmcmFtZXdvcmsuc3luYy5yb290XCIsXG4gICAgICAgICAgICBsYWJlbDogXCJcIixcbiAgICAgICAgICAgIGl0ZW1zOiBbXG4gICAgICAgICAgICAgIHtcbiAgICAgICAgICAgICAgICBpZDogXCJmcmFtZXdvcmsuc3luYy5jYXJkXCIsXG4gICAgICAgICAgICAgICAgbGFiZWw6IFwiXCIsXG4gICAgICAgICAgICAgICAgY29udHJvbDogKFxuICAgICAgICAgICAgICAgICAgPFN5bmNBdHRhY2hDYXJkXG4gICAgICAgICAgICAgICAgICAgIGFjdGl2ZVVyaT17c3luY0JhY2tib25lVXJpfVxuICAgICAgICAgICAgICAgICAgICBjYXJkS2luZD17c3luY0NhcmRLaW5kfVxuICAgICAgICAgICAgICAgICAgICBkcmFmdFBhdGg9e3N5bmNEcmFmdFBhdGh9XG4gICAgICAgICAgICAgICAgICAgIHN5bmNVdGlsaXRpZXM9e3N5bmNVdGlsaXRpZXN9XG4gICAgICAgICAgICAgICAgICAgIHN0YXR1cz17c3luY1N0YXR1c31cbiAgICAgICAgICAgICAgICAgICAgb25BY3Rpb249e29uQWN0aW9ufVxuICAgICAgICAgICAgICAgICAgICBvbkRyYWZ0UGF0aENoYW5nZT17KHZhbHVlKSA9PiBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1NZTkNfRFJBRlRfUEFUSFwiLCB2YWx1ZSB9KX1cbiAgICAgICAgICAgICAgICAgICAgb25DbG9zZT17KCkgPT4gZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9TWU5DX0NBUkRfS0lORFwiLCB2YWx1ZTogbnVsbCB9KX1cbiAgICAgICAgICAgICAgICAgICAgb25BdHRhY2g9e2F0dGFjaFN5bmNCYWNrYm9uZX1cbiAgICAgICAgICAgICAgICAgICAgb25EZXRhY2g9e2RldGFjaFN5bmNCYWNrYm9uZX1cbiAgICAgICAgICAgICAgICAgIC8+XG4gICAgICAgICAgICAgICAgKSxcbiAgICAgICAgICAgICAgfSxcbiAgICAgICAgICAgIF0sXG4gICAgICAgICAgfSxcbiAgICAgICAgXSxcbiAgICAgIH0sXG4gICAgfSk7XG4gIH0sIFthdHRhY2hTeW5jQmFja2JvbmUsIGRldGFjaFN5bmNCYWNrYm9uZSwgb25BY3Rpb24sIHN5bmNCYWNrYm9uZVVyaSwgc3luY0NhcmRLaW5kLCBzeW5jRHJhZnRQYXRoLCBzeW5jU3RhdHVzQnlEb2N1bWVudElkLCB1aUxvY2FsZV0pO1xuICAvLyNlbmRyZWdpb24g8J+UhO+4j1N5bmNMZWFmXG5cbiAgY29uc3QgYWN0aXZlUGx1Z2luTWFuaWZlc3QgPSB1c2VNZW1vKCgpID0+IGxvYWRlZFBsdWdpbnMuZmluZCgoZW50cnkpID0+IGVudHJ5LmhhbmRsZS5wbHVnaW5JZCA9PT0gc2Vzc2lvbj8ucGx1Z2luSWQpPy5tYW5pZmVzdCwgW2xvYWRlZFBsdWdpbnMsIHNlc3Npb24/LnBsdWdpbklkXSk7XG4gIGNvbnN0IGFjdGl2ZU1vZGVJZCA9IHNlc3Npb24/LnZpZXdTdGF0ZS5hY3RpdmVNb2RlSWQgPz8gc2Vzc2lvbj8uYXBwLm1vZGVzWzBdPy5pZCA/PyBzZXNzaW9uPy5hcHAuaWQgPz8gXCJcIjtcblxuICAvLyDwn5Ox77iPIE1vdmVkIGFoZWFkIG9mIGBtb2JpbGVQYW5lbFRhYnNgIChiZWxvdykgc28gaXRzIHN5bnRoZXRpYyBtb2JpbGUgXCJBcHBcIiB0YWIgY2FuIHNoYXJlIHRoZSBleGFjdFxuICAvLyBleGFtcGxlLXNlbGVjdC9tb2RlLXN3aXRjaGVyIGVsZW1lbnRzIHRoZSBkZXNrdG9wIG5hdmJhciBjZW50ZXIgY2x1c3RlciByZW5kZXJzIOKAlCBzaW5nbGUgc291cmNlIG9mIHRydXRoLlxuICBjb25zdCBleGFtcGxlT3B0aW9ucyA9IHVzZU1lbW8oKCkgPT4ge1xuICAgIGNvbnN0IGFwcElkID0gc2Vzc2lvbj8uYXBwLmlkID8/IFwiXCI7XG4gICAgaWYgKCFhcHBJZCkgcmV0dXJuIFtdO1xuICAgIGNvbnN0IHNlZW4gPSBuZXcgU2V0PHN0cmluZz4oKTtcbiAgICByZXR1cm4gKGFjdGl2ZVBsdWdpbk1hbmlmZXN0Py5leGFtcGxlcyA/PyBbXSlcbiAgICAgIC5maWx0ZXIoKGV4YW1wbGUpID0+IGV4YW1wbGUuYXBwSWQgPT09IGFwcElkKVxuICAgICAgLmZpbHRlcigoZXhhbXBsZSkgPT4ge1xuICAgICAgICBpZiAoc2Vlbi5oYXMoZXhhbXBsZS5pZCkpIHJldHVybiBmYWxzZTtcbiAgICAgICAgc2Vlbi5hZGQoZXhhbXBsZS5pZCk7XG4gICAgICAgIHJldHVybiB0cnVlO1xuICAgICAgfSlcbiAgICAgIC5tYXAoKGV4YW1wbGUpID0+ICh7XG4gICAgICAgIGlkOiBleGFtcGxlLmlkLFxuICAgICAgICBsYWJlbDogcmVzb2x2ZUFwcExhYmVsKGFwcExhYmVsc092ZXJsYXksIFwiZXhhbXBsZVwiLCBleGFtcGxlLmlkLCByZXNvbHZlTWFuaWZlc3RMYWJlbChleGFtcGxlLmxhYmVsLCB1aVRlcm1pbm9sb2d5LCB1aUxvY2FsZSkpLFxuICAgICAgICBpY29uOiBleGFtcGxlLmljb25JZCxcbiAgICAgIH0pKTtcbiAgfSwgW2FjdGl2ZVBsdWdpbk1hbmlmZXN0LCBzZXNzaW9uPy5hcHAuaWQsIGFwcExhYmVsc092ZXJsYXksIHVpVGVybWlub2xvZ3ksIHVpTG9jYWxlXSk7XG5cbiAgY29uc3QgZGlzcGF0Y2hBY3RpdmVFeGFtcGxlID0gdXNlQ2FsbGJhY2soXG4gICAgKGV4YW1wbGVJZDogc3RyaW5nKSA9PiB7XG4gICAgICBpZiAoIXNlc3Npb24pIHJldHVybjtcbiAgICAgIGNvbnN0IHBsdWdpbiA9IGxvYWRlZFBsdWdpbnMuZmluZCgoZW50cnkpID0+IGVudHJ5LmhhbmRsZS5wbHVnaW5JZCA9PT0gc2Vzc2lvbi5wbHVnaW5JZCk/LmhhbmRsZTtcbiAgICAgIGlmICghcGx1Z2luKSByZXR1cm47XG4gICAgICBvbkFjdGlvbih7IGNvbnRyb2xsZXJJZDogc2Vzc2lvbi5hcHAuY29udHJvbGxlcklkLCBhY3Rpb246IFwic2V0QWN0aXZlRXhhbXBsZVwiLCBhcmdzOiB7IGV4YW1wbGVJZDogZXhhbXBsZUlkIHx8IFwiXCIgfSB9KTtcbiAgICB9LFxuICAgIFthcHBseUhvc3RFZmZlY3RzLCBpbmplY3RBY3RpdmVVdGlsaXR5LCBsb2FkZWRQbHVnaW5zLCBvbkFjdGlvbiwgc2Vzc2lvbl0sXG4gICk7XG5cbiAgLyoqIEBlbW9qaSDwn46b77iPIFNoYXJlZCBieSB0aGUgZGVza3RvcCBuYXZiYXIgY2VudGVyIGNsdXN0ZXIgYW5kIHRoZSBtb2JpbGUgcGFuZWwncyBzeW50aGV0aWMgXCJBcHBcIiB0YWIgKHNlZSBgbW9iaWxlUGFuZWxUYWJzYCkuICovXG4gIGNvbnN0IGV4YW1wbGVTZWxlY3RFbGVtZW50ID0gdXNlTWVtbygoKSA9PiB7XG4gICAgaWYgKCFzZXNzaW9uIHx8IGV4YW1wbGVPcHRpb25zLmxlbmd0aCA9PT0gMCB8fCBsb2Nrcy5leGFtcGxlSWQgfHwgKHN0dWRpb01vZGUgJiYgc2Vzc2lvbi5hcHAuaWQgPT09IGxhbmRpbmdBcHBJZCkpIHJldHVybiBudWxsO1xuICAgIHJldHVybiAoXG4gICAgICA8TmF2YmFyRXhhbXBsZVNlbGVjdFxuICAgICAgICBrZXk9XCJmaXh0dXJlXCJcbiAgICAgICAgaWQ9XCJwbGF5Z3JvdW5kLm5hdmJhci5maXh0dXJlXCJcbiAgICAgICAgdmFsdWU9e2FjdGl2ZUV4YW1wbGVJZH1cbiAgICAgICAgb3B0aW9ucz17ZXhhbXBsZU9wdGlvbnN9XG4gICAgICAgIG9uVmFsdWVDaGFuZ2U9eyhleGFtcGxlSWQpID0+IHtcbiAgICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0FDVElWRV9FWEFNUExFX0lEXCIsIHZhbHVlOiBleGFtcGxlSWQgfSk7XG4gICAgICAgICAgZGlzcGF0Y2hBY3RpdmVFeGFtcGxlKGV4YW1wbGVJZCB8fCBcIlwiKTtcbiAgICAgICAgfX1cbiAgICAgIC8+XG4gICAgKTtcbiAgfSwgW3Nlc3Npb24sIGV4YW1wbGVPcHRpb25zLCBsb2Nrcy5leGFtcGxlSWQsIHN0dWRpb01vZGUsIGxhbmRpbmdBcHBJZCwgYWN0aXZlRXhhbXBsZUlkLCBkaXNwYXRjaEFjdGl2ZUV4YW1wbGVdKTtcblxuICAvKiogQGVtb2ppIPCfjpvvuI8gU2hhcmVkIGJ5IHRoZSBkZXNrdG9wIG5hdmJhciBjZW50ZXIgY2x1c3RlciBhbmQgdGhlIG1vYmlsZSBwYW5lbCdzIHN5bnRoZXRpYyBcIkFwcFwiIHRhYiAoc2VlIGBtb2JpbGVQYW5lbFRhYnNgKS4gKi9cbiAgY29uc3QgbW9kZVN3aXRjaGVyRWxlbWVudCA9IHVzZU1lbW8oKCkgPT4ge1xuICAgIGlmICghc2Vzc2lvbiB8fCBzZXNzaW9uLmFwcC5tb2Rlcy5sZW5ndGggPD0gMSkgcmV0dXJuIG51bGw7XG4gICAgcmV0dXJuIChcbiAgICAgIDxCdXR0b25Hcm91cCBrZXk9XCJtb2Rlc1wiIGlkPVwicGxheWdyb3VuZC5uYXZiYXIubW9kZXNcIj5cbiAgICAgICAge3Nlc3Npb24uYXBwLm1vZGVzLm1hcCgobW9kZSkgPT4ge1xuICAgICAgICAgIGNvbnN0IGlzQWN0aXZlID0gYWN0aXZlTW9kZUlkID09PSBtb2RlLmlkO1xuICAgICAgICAgIHJldHVybiAoXG4gICAgICAgICAgICA8QnV0dG9uR3JvdXBJdGVtXG4gICAgICAgICAgICAgIGtleT17bW9kZS5pZH1cbiAgICAgICAgICAgICAgaWQ9e2BwbGF5Z3JvdW5kLm5hdmJhci5tb2Rlcy4ke21vZGUuaWR9YH1cbiAgICAgICAgICAgICAgY2xhc3NOYW1lPXtjbihpc0FjdGl2ZSAmJiBpbnRlcmFjdGl2ZUFjdGl2ZUZpbGxDbGFzcyl9XG4gICAgICAgICAgICAgIGRhdGEtc3RhdGU9e2lzQWN0aXZlID8gXCJvblwiIDogdW5kZWZpbmVkfVxuICAgICAgICAgICAgICBvbkNsaWNrPXsoKSA9PiBhcHBseU1vZGVDaGFuZ2UobW9kZS5pZCl9XG4gICAgICAgICAgICAgIGljb249e21vZGUuaWNvbklkfVxuICAgICAgICAgICAgICB0ZXh0PXtyZXNvbHZlQXBwTGFiZWwoYXBwTGFiZWxzT3ZlcmxheSwgXCJtb2RlXCIsIG1vZGUuaWQsIHJlc29sdmVNYW5pZmVzdExhYmVsKG1vZGUubGFiZWwsIHVpVGVybWlub2xvZ3ksIHVpTG9jYWxlKSl9XG4gICAgICAgICAgICAvPlxuICAgICAgICAgICk7XG4gICAgICAgIH0pfVxuICAgICAgPC9CdXR0b25Hcm91cD5cbiAgICApO1xuICB9LCBbc2Vzc2lvbiwgYWN0aXZlTW9kZUlkLCBhcHBseU1vZGVDaGFuZ2UsIGFwcExhYmVsc092ZXJsYXksIHVpVGVybWlub2xvZ3ksIHVpTG9jYWxlXSk7XG5cbiAgY29uc3QgcmVzb2x2ZWRDb21tYW5kcyA9IHVzZU1lbW8oXG4gICAgKCkgPT4gcmVzb2x2ZUNvbW1hbmRzKG9zQ29tbWFuZHMsIGFjdGl2ZVBsdWdpbk1hbmlmZXN0LCBzZXNzaW9uPy5hcHAsIGFjdGl2ZU1vZGVJZCwgYXBwTGFiZWxzT3ZlcmxheSwgdWlUZXJtaW5vbG9neSwgdWlMb2NhbGUpLFxuICAgIFtvc0NvbW1hbmRzLCBhY3RpdmVQbHVnaW5NYW5pZmVzdCwgc2Vzc2lvbj8uYXBwLCBhY3RpdmVNb2RlSWQsIGFwcExhYmVsc092ZXJsYXksIHVpVGVybWlub2xvZ3ksIHVpTG9jYWxlXSxcbiAgKTtcblxuICBjb25zdCBjb21tYW5kQ2F0ZWdvcnlMaXN0ID0gdXNlTWVtbygoKSA9PiBjb21tYW5kQ2F0ZWdvcmllcyhyZXNvbHZlZENvbW1hbmRzKSwgW3Jlc29sdmVkQ29tbWFuZHMsIHVpTG9jYWxlXSk7XG5cbiAgLyoqXG4gICAqIPCfjpvvuI8gRGlzcGF0Y2hlcyBhIHJlc29sdmVkIGNvbW1hbmQ6IG9zLXNjb3BlIGNvbW1hbmRzIGFyZSBoYW5kbGVkIGxvY2FsbHkgKG5vIHByb2dyYW0gcm91bmQgdHJpcCk7XG4gICAqIHBsdWdpbi9hcHAvbW9kZS1zY29wZSBjb21tYW5kcyByb3V0ZSB0aHJvdWdoIHRoZSBhY3RpdmUgc2Vzc2lvbidzIHByb2dyYW0gYGhhbmRsZUNvbW1hbmRgLCBtaXJyb3JpbmdcbiAgICogYG9uQWN0aW9uYCdzIHRhaWwuIFBsdWdpbiBjb21tYW5kcyBhcmUgb25seSByZXNvbHZhYmxlL2Rpc3BhdGNoYWJsZSBmb3IgdGhlIGFjdGl2ZSBzZXNzaW9uJ3MgcHJvZ3JhbVxuICAgKiBpbnN0YW5jZSAobm8gaGVhZGxlc3MtaW5zdGFuY2Ugcm91dGluZyBmb3Igbm9uLWZvY3VzZWQgcGx1Z2lucyB5ZXQpLlxuICAgKi9cbiAgY29uc3Qgb25Db21tYW5kID0gdXNlQ2FsbGJhY2soXG4gICAgKHNvdXJjZTogUmVzb2x2ZWRDb21tYW5kW1wic291cmNlXCJdLCBjb21tYW5kSWQ6IHN0cmluZywgYXJncz86IFJlY29yZDxzdHJpbmcsIHVua25vd24+KSA9PiB7XG4gICAgICAvLyDwn46l77iPIFNhbWUgc2FuZGJveC1zdGFydC9yZWNvcmRlci1hcm0gc2lkZSBlZmZlY3RzIGBTVEFSVF9UVVRPUklBTF9BQ1RJT05fSURgL2BSRUNPUkRfVFVUT1JJQUxfQUNUSU9OX0lEYFxuICAgICAgLy8gbmVlZCDigJQgcm91dGVkIHRocm91Z2ggdGhlIGBzdGFydFR1dG9yaWFsUmVmYC9gdG9nZ2xlVHV0b3JpYWxSZWNvcmRpbmdSZWZgIGJyaWRnZSBzaW5jZSB0aGV5IG5lZWRcbiAgICAgIC8vIG1vcmUgY29udGV4dCAocGx1Z2luIGJyaWRnZSwgc2FuZGJveCBzbmFwc2hvdCkgdGhhbiBhIGJhcmUgYGRpc3BhdGNoYCBnaXZlcyBgZGlzcGF0Y2hPc0NvbW1hbmRgLlxuICAgICAgaWYgKHNvdXJjZS5raW5kID09PSBcIm9zXCIgJiYgY29tbWFuZElkID09PSBcIm9zLnBsYXlUdXRvcmlhbFwiKSB7XG4gICAgICAgIGNvbnN0IHR1dG9yaWFsSWQgPSB0eXBlb2YgYXJncz8udHV0b3JpYWxJZCA9PT0gXCJzdHJpbmdcIiA/IGFyZ3MudHV0b3JpYWxJZCA6IFwiXCI7XG4gICAgICAgIGlmICh0dXRvcmlhbElkKSBzdGFydFR1dG9yaWFsUmVmLmN1cnJlbnQodHV0b3JpYWxJZCk7XG4gICAgICAgIHJldHVybjtcbiAgICAgIH1cbiAgICAgIGlmIChzb3VyY2Uua2luZCA9PT0gXCJvc1wiICYmIGNvbW1hbmRJZCA9PT0gXCJvcy5yZWNvcmRUdXRvcmlhbFwiKSB7XG4gICAgICAgIHRvZ2dsZVR1dG9yaWFsUmVjb3JkaW5nUmVmLmN1cnJlbnQoKTtcbiAgICAgICAgcmV0dXJuO1xuICAgICAgfVxuICAgICAgaWYgKHNvdXJjZS5raW5kID09PSBcIm9zXCIpIHtcbiAgICAgICAgZGlzcGF0Y2hPc0NvbW1hbmQoY29tbWFuZElkLCBhcmdzLCBkaXNwYXRjaCwgZG9ja0xheW91dFN0b3JlLCBkb2NrVWlTdGF0ZVN0b3JlLCBsb2Nrcyk7XG4gICAgICAgIGNvbnN0IGxhYmVsID0gcmVzb2x2ZWRDb21tYW5kcy5maW5kKChlbnRyeSkgPT4gZW50cnkuZGVmaW5pdGlvbi5pZCA9PT0gY29tbWFuZElkKT8uZGVmaW5pdGlvbi5sYWJlbCA/PyBjb21tYW5kSWQ7XG4gICAgICAgIG5vdGVTaGVsbENvbW1hbmQoY29tbWFuZElkLCBsYWJlbCwgYXJncyk7XG4gICAgICAgIHJldHVybjtcbiAgICAgIH1cbiAgICAgIGlmICghc2Vzc2lvbikgcmV0dXJuO1xuICAgICAgLy8g4o+677iPIFJlY29yZGVyIHRhcCBmb3IgcGx1Z2luL2FwcC9tb2RlLXNjb3BlIGNvbW1hbmRzIOKAlCBtaXJyb3JzIGBvbkFjdGlvbmAncyB0YXAgYWJvdmUuXG4gICAgICBpZiAodHV0b3JpYWxSZWNvcmRpbmdSZWYuY3VycmVudCAmJiAhdHV0b3JpYWxEcml2ZW5SZWYuY3VycmVudCkge1xuICAgICAgICB0dXRvcmlhbFJlY29yZGVyUmVmLmN1cnJlbnQ/LnJlY29yZEV2ZW50KHsga2luZDogXCJjb21tYW5kXCIsIGNvbW1hbmQ6IGNvbW1hbmRJZCwgYXJncyB9KTtcbiAgICAgIH1cbiAgICAgIGNvbnN0IHBsdWdpbiA9IGxvYWRlZFBsdWdpbnMuZmluZCgoZW50cnkpID0+IGVudHJ5LmhhbmRsZS5wbHVnaW5JZCA9PT0gc2Vzc2lvbi5wbHVnaW5JZCk/LmhhbmRsZTtcbiAgICAgIGlmICghcGx1Z2luPy5oYW5kbGVBY3Rpb24pIHJldHVybjtcbiAgICAgIGNvbnN0IGRpc3BhdGNoVmlld1N0YXRlID0gaW5qZWN0QWN0aXZlVXRpbGl0eShzZXNzaW9uLnZpZXdTdGF0ZSk7XG4gICAgICAvLyDwn46v77iPIEFwcCBwYWxldHRlIGNvbW1hbmRzIHNoYXJlIHRoZSBhY3Rpb24gd2lyZSArIGBjb21tYW5kX2Zyb21fYWN0aW9uYCBicmlkZ2Ug4oCUIHRoZXJlIGFyZSBub1xuICAgICAgLy8gZnJhbWV3b3JrLXJlc2VydmVkIENPTU1BTkRTLCBzbyBgaGFuZGxlQ29tbWFuZGAvYGtpbmQ6XCJjb21tYW5kXCJgIGFsd2F5cyBoYXJkLWVycm9ycyBwb2ludGluZyBhdFxuICAgICAgLy8gdGhlIHR5cGVkIGNoYW5uZWwgKHNlZSBgVmNzRG9jdW1lbnRBcHA6OmRpc3BhdGNoX2NvbW1hbmRgKS5cbiAgICAgIHZvaWQgcGx1Z2luXG4gICAgICAgIC5oYW5kbGVBY3Rpb24oc2Vzc2lvbi5pbnN0YW5jZUlkLCBlbmNvZGVBY3Rpb25XaXJlKHsgY29udHJvbGxlcklkOiBzZXNzaW9uLmFwcC5jb250cm9sbGVySWQsIGFjdGlvbjogY29tbWFuZElkLCBhcmdzIH0pLCBkaXNwYXRjaFZpZXdTdGF0ZSlcbiAgICAgICAgLnRoZW4oKHJlc3BvbnNlKSA9PiBhcHBseUhvc3RFZmZlY3RzKHJlc3BvbnNlLnJlcXVlc3RlZEVmZmVjdHMgPz8gW10sIHsgLi4uc2Vzc2lvbiwgdmlld1N0YXRlOiBkaXNwYXRjaFZpZXdTdGF0ZSB9LCByZXNvbHZlVWlEaXJ0eVNjb3BlKHJlc3BvbnNlLnVpU2NvcGUpKSlcbiAgICAgICAgLmNhdGNoKChjb21tYW5kRXJyb3IpID0+IHtcbiAgICAgICAgICBjb25zb2xlLmVycm9yKFwiW0RFQlVHXSBjb21tYW5kIGZhaWxlZFwiLCBjb21tYW5kRXJyb3IpO1xuICAgICAgICB9KTtcbiAgICB9LFxuICAgIFthcHBseUhvc3RFZmZlY3RzLCBkb2NrTGF5b3V0U3RvcmUsIGRvY2tVaVN0YXRlU3RvcmUsIGluamVjdEFjdGl2ZVV0aWxpdHksIGxvYWRlZFBsdWdpbnMsIHNlc3Npb24sIGxvY2tzLCByZXNvbHZlZENvbW1hbmRzLCBub3RlU2hlbGxDb21tYW5kXSxcbiAgKTtcblxuICBjb25zdCBjb21tYW5kQ2F0ZWdvcnlUYWJzID0gdXNlTWVtbygoKSA9PiBidWlsZENvbW1hbmRDYXRlZ29yeVRhYnMocmVzb2x2ZWRDb21tYW5kcywgY29tbWFuZENhdGVnb3J5TGlzdCwgZXhwYW5kZWRDb21tYW5kSWRSZWYsIGNvbW1hbmRTdGFnZWRBcmdzQnlDb21tYW5kSWRSZWYsIG9uQ29tbWFuZCwgZGlzcGF0Y2gpLCBbcmVzb2x2ZWRDb21tYW5kcywgY29tbWFuZENhdGVnb3J5TGlzdCwgb25Db21tYW5kXSk7XG5cbiAgLy8g8J+Xuu+4jyBgVG9vbERlZmluaXRpb24ubGFiZWxgIGlzIGEgbWFuaWZlc3QgYExvY2FsaXplZExhYmVsYCBmaWVsZCDigJQgcmVzb2x2ZWQgaGVyZSwgcmlnaHQgYWZ0ZXJcbiAgLy8gYHJlc29sdmVNb2RlVG9vbHNgIChhbiBleHRlcm5hbCBgZnJhbWV3b3JrLW9zLWNvcmVgIGhlbHBlciB0aGlzIGZpbGUgY2Fubm90IGVkaXQpLCBzbyBldmVyeVxuICAvLyBkb3duc3RyZWFtIGNvbnN1bWVyIChgYnVpbGRUb29sVHJlZWAvYGJ1aWxkVG9vbFRhYnNgKSBrZWVwcyByZWFkaW5nIGFuIGFscmVhZHktcGxhaW4tc3RyaW5nIGBsYWJlbGAuXG4gIGNvbnN0IHJlc29sdmVkTW9kZVRvb2xzID0gdXNlTWVtbyhcbiAgICAoKSA9PiByZXNvbHZlTW9kZVRvb2xzKHNlc3Npb24/LmFwcCwgYWN0aXZlTW9kZUlkKS5tYXAoKHRvb2wpID0+ICh7IC4uLnRvb2wsIGxhYmVsOiByZXNvbHZlTWFuaWZlc3RMYWJlbCh0b29sLmxhYmVsLCB1aVRlcm1pbm9sb2d5LCB1aUxvY2FsZSkgfSkpLFxuICAgIFtzZXNzaW9uPy5hcHAsIGFjdGl2ZU1vZGVJZCwgdWlUZXJtaW5vbG9neSwgdWlMb2NhbGVdLFxuICApO1xuXG4gIGNvbnN0IHRvb2xUYWJzID0gdXNlTWVtbyhcbiAgICAoKSA9PiAoc2Vzc2lvbiA/IGJ1aWxkVG9vbFRhYnMocmVzb2x2ZWRNb2RlVG9vbHMsIHNlc3Npb24uYXBwLmNvbnRyb2xsZXJJZCwgYWN0aXZlVG9vbElkUmVmLCB0b29sTWVhc3VyZXNCeVRvb2xJZFJlZiwgb25BY3Rpb25TdGFibGUpIDogW10pLFxuICAgIFtyZXNvbHZlZE1vZGVUb29scywgc2Vzc2lvbj8uYXBwLmNvbnRyb2xsZXJJZCwgb25BY3Rpb25TdGFibGVdLFxuICApO1xuXG4gIC8vI3JlZ2lvbiDwn6et77iPRG9ja0Fzc2VtYmx5IOKAlCBkZWZhdWx0IGZvdXItY29ybmVyIGFycmFuZ2VtZW50ICh0aGUgdHdvIG1pZGRsZSBhbmNob3JzIHN0YXJ0IGVtcHR5IHNhdmUgdGhlIGNvbW1hbmQgcGFsZXR0ZSBpbiBib3R0b20tbWlkZGxlKSArIHBlcnNpc3RlZC1vdmVycmlkZSByZWNvbmNpbGlhdGlvbiArIGRyYWctYW5kLWRyb3Agd2lyaW5nLlxuICBjb25zdCBkZWZhdWx0RG9jayA9IHVzZU1lbW8oKCk6IFBhbmVsRG9jayA9PiB7XG4gICAgLy8g8J+nre+4jyBUb3AtbGVmdCAoV29ya2JlbmNoOiBEb2N1bWVudC9DYXRhbG9ndWUpLCB0b3AtcmlnaHQgKERldGFpbHM6IEluc3BlY3Rpb24vUGFyYW1ldGVycykgYW5kIGJvdHRvbS1yaWdodFxuICAgIC8vIChTZXR0aW5nczogVGhlbWUvU2V0dGluZ3MpIHJlbmRlciB0aGVpciB0YWJzIGZsYXQsIG9uZSBsZXZlbCB1cCBmcm9tIHdoZXJlIHRoZXkgdXNlZCB0byBzaXQg4oCUIHRoZVxuICAgIC8vIGNhdGVnb3J5LWJyYW5jaCB3cmFwcGVyIHRhYiBpcyBnb25lLCBzbyBlYWNoIGxlYWYgaXMgYSB0b3AtbGV2ZWwgdG9nZ2xlIGluc3RlYWQgb2YgdHdvIGNsaWNrcyBkZWVwLlxuICAgIGNvbnN0IHRvcExlZnQ6IFBhbmVsVGFiTm9kZVtdID0gWy4uLndvcmtiZW5jaExlZnRUYWJzXTtcbiAgICBjb25zdCBib3R0b21MZWZ0OiBQYW5lbFRhYk5vZGVbXSA9IFtdO1xuICAgIGlmIChmcmFtZXdvcmtEaXNwbGF5VGFicy5sZW5ndGggPiAwKSB7XG4gICAgICBib3R0b21MZWZ0LnB1c2goeyBraW5kOiBcImJyYW5jaFwiLCBpZDogRlJBTUVXT1JLX0NBVEVHT1JZX0RJU1BMQVlfSUQsIGljb246IGNhdGVnb3J5VGFiSWNvbihmcmFtZXdvcmtEaXNwbGF5VGFicywgXCJsYXlvdXQtZ3JpZFwiKSwgbmFtZTogc2hlbGxMYWJlbChcInVpLnBhbmVsVG9nZ2xlLmRpc3BsYXlcIiksIG9yZGVyOiAwLCBjaGlsZHJlbjogZnJhbWV3b3JrRGlzcGxheVRhYnMgfSk7XG4gICAgfVxuICAgIGlmIChmcmFtZXdvcmtTeW5jVGFiKSBib3R0b21MZWZ0LnB1c2goZnJhbWV3b3JrU3luY1RhYik7XG4gICAgY29uc3QgdG9wUmlnaHQ6IFBhbmVsVGFiTm9kZVtdID0gWy4uLmRldGFpbHNSaWdodFRhYnNdO1xuICAgIGNvbnN0IGJvdHRvbVJpZ2h0OiBQYW5lbFRhYk5vZGVbXSA9IFsuLi5zZXR0aW5nc1JpZ2h0VGFicywgLi4uZnJhbWV3b3JrUGx1Z2luc1RhYnNdO1xuICAgIGlmIChmcmFtZXdvcmtVdGlsaXRpZXNIaXN0b3J5VGFiKSBib3R0b21SaWdodC5wdXNoKGZyYW1ld29ya1V0aWxpdGllc0hpc3RvcnlUYWIpO1xuICAgIC8vIPCfm6DvuI8gVG9vbCBjYXRlZ29yaWVzIHN0YXkgbmVzdGVkIHVuZGVyIG9uZSBleHBhbmRhYmxlIFRvb2wgYnJhbmNoLCBleGFjdGx5IGxpa2UgQ29tbWFuZCBjYXRlZ29yaWVzLFxuICAgIC8vIHBsYWNlZCBsZWZ0IG9mIENvbW1hbmQgKG9yZGVyIDAgdnMgMSkg4oCUIGxpa2UgY29tbWFuZHMgbm90IGJlaW5nIHdpbmRvdy1sZXZlbCwgdG9vbHMgYXJlIG5vdFxuICAgIC8vIHdpbmRvdy1sZXZlbCBlaXRoZXI7IGJvdGggbGl2ZSBvbmx5IG9uIHRoaXMgc2hhcmVkIG1vZGUtc2NvcGVkIGFuY2hvci5cbiAgICAvLyDwn46b77iPIENvbW1hbmQgY2F0ZWdvcmllcyBzdGF5IG5lc3RlZCB1bmRlciBvbmUgZXhwYW5kYWJsZSBDb21tYW5kIGJyYW5jaCAodW5saWtlIGZsYXQgVGhlbWUvU2V0dGluZ3NcbiAgICAvLyBmb290ZXIgdG9nZ2xlcykgc28gdGhlIGZvbGRlZCBib3R0b20tbWlkZGxlIGNocm9tZSBzaG93cyBhIHNpbmdsZSBDb21tYW5kIHRvZ2dsZSwgbm90IGV2ZXJ5XG4gICAgLy8gY2F0ZWdvcnkgbGVhZiBpbmxpbmVkIGFsb25nIHRoZSBmb290ZXIuXG4gICAgY29uc3QgYm90dG9tTWlkZGxlOiBQYW5lbFRhYk5vZGVbXSA9IFtcbiAgICAgIC4uLih0b29sVGFicy5sZW5ndGggPiAwID8gW3sga2luZDogXCJicmFuY2hcIiBhcyBjb25zdCwgaWQ6IEZSQU1FV09SS19DQVRFR09SWV9UT09MX0lELCBpY29uOiBjYXRlZ29yeVRhYkljb24odG9vbFRhYnMsIFwiaGFtbWVyXCIpLCBuYW1lOiBzaGVsbExhYmVsKFwidWkucGFuZWxUb2dnbGUudG9vbFwiKSwgb3JkZXI6IDAsIGNoaWxkcmVuOiB0b29sVGFicyB9XSA6IFtdKSxcbiAgICAgIC4uLihjb21tYW5kQ2F0ZWdvcnlUYWJzLmxlbmd0aCA+IDAgPyBbeyBraW5kOiBcImJyYW5jaFwiIGFzIGNvbnN0LCBpZDogRlJBTUVXT1JLX0NBVEVHT1JZX0NPTU1BTkRfSUQsIGljb246IGNhdGVnb3J5VGFiSWNvbihjb21tYW5kQ2F0ZWdvcnlUYWJzLCBcIndyZW5jaFwiKSwgbmFtZTogc2hlbGxMYWJlbChcInVpLnBhbmVsVG9nZ2xlLmNvbW1hbmRcIiksIG9yZGVyOiAxLCBjaGlsZHJlbjogY29tbWFuZENhdGVnb3J5VGFicyB9XSA6IFtdKSxcbiAgICBdO1xuICAgIHJldHVybiB7IGFuY2hvcnM6IHsgXCJ0b3AtbGVmdFwiOiB0b3BMZWZ0LCBcInRvcC1taWRkbGVcIjogW10sIFwidG9wLXJpZ2h0XCI6IHRvcFJpZ2h0LCBcInJpZ2h0LW1pZGRsZVwiOiBbXSwgXCJib3R0b20tcmlnaHRcIjogYm90dG9tUmlnaHQsIFwiYm90dG9tLW1pZGRsZVwiOiBib3R0b21NaWRkbGUsIFwiYm90dG9tLWxlZnRcIjogYm90dG9tTGVmdCwgXCJsZWZ0LW1pZGRsZVwiOiBbXSB9IH07XG4gIH0sIFtjb21tYW5kQ2F0ZWdvcnlUYWJzLCBkZXRhaWxzUmlnaHRUYWJzLCBmcmFtZXdvcmtEaXNwbGF5VGFicywgZnJhbWV3b3JrUGx1Z2luc1RhYnMsIGZyYW1ld29ya1N5bmNUYWIsIGZyYW1ld29ya1V0aWxpdGllc0hpc3RvcnlUYWIsIHNldHRpbmdzUmlnaHRUYWJzLCB0b29sVGFicywgdWlMb2NhbGUsIHdvcmtiZW5jaExlZnRUYWJzXSk7XG5cbiAgdXNlRWZmZWN0KCgpID0+IHtcbiAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0RPQ0tfT1ZFUlJJREVcIiwgdmFsdWU6IGRvY2tMYXlvdXRTdG9yZS5nZXRTbmFwc2hvdCgpIH0pO1xuICB9LCBbZG9ja0xheW91dFN0b3JlXSk7XG5cbiAgY29uc3QgZG9jayA9IHVzZU1lbW8oKCk6IFBhbmVsRG9jayA9PiBhcHBseURvY2tTa2VsZXRvbihkZWZhdWx0RG9jaywgZG9ja092ZXJyaWRlKSwgW2RlZmF1bHREb2NrLCBkb2NrT3ZlcnJpZGVdKTtcblxuICAvLyDwn5Ox77iPIEFsbCBlaWdodCBhbmNob3JzJyB0YWJzIGZsYXR0ZW5lZCBpbnRvIHRoZSBzaW5nbGUgbW9iaWxlIHBhbmVsJ3MgdGFiIGxpc3Qg4oCUIGRlZmluZWQgaGVyZSAoYWhlYWQgb2YgdGhlXG4gIC8vIGRvY2stYXNzZW1ibHkgb3ZlcnJpZGUgZWZmZWN0cyBiZWxvdykgc28gdGhvc2UgZWZmZWN0cyBjYW4gcmVzb2x2ZSBhIG1vYmlsZS1wYW5lbCBwYXRoIGFsb25nc2lkZSB0aGVcbiAgLy8gZGVza3RvcCBwZXItYW5jaG9yIG9uZS5cbiAgY29uc3QgbW9iaWxlUGFuZWxUYWJzID0gdXNlTWVtbygoKSA9PiB7XG4gICAgY29uc3QgYW5jaG9yVGFicyA9IEFOQ0hPUlMuZmxhdE1hcCgoYW5jaG9yKSA9PiBkZWZhdWx0RG9jay5hbmNob3JzW2FuY2hvcl0pO1xuICAgIC8vIPCfk7HvuI8gVGhlIGV4YW1wbGUgc2VsZWN0b3IgYW5kIG1vZGUgc3dpdGNoZXIgaGF2ZSBubyBuYXZiYXIgcm9vbSBvbiBtb2JpbGUgKHNlZSBgbmF2YmFySXRlbXNgKSDigJQgdGhleVxuICAgIC8vIHN1cmZhY2UgYXMgb25lIG1vcmUgdGFiIGluIHRoZSBtZXJnZWQgbW9iaWxlIHBhbmVsIGluc3RlYWQsIHNoYXJpbmcgdGhlIGV4YWN0IHNhbWUgZWxlbWVudHMgdGhlXG4gICAgLy8gZGVza3RvcCBuYXZiYXIgY2VudGVyIGNsdXN0ZXIgcmVuZGVycy5cbiAgICBpZiAoIWV4YW1wbGVTZWxlY3RFbGVtZW50ICYmICFtb2RlU3dpdGNoZXJFbGVtZW50KSByZXR1cm4gYW5jaG9yVGFicztcbiAgICBjb25zdCBhcHBUYWIgPSBzaW5nbGVUcmVlTGVhZih7XG4gICAgICBpZDogXCJmcmFtZXdvcmsubW9iaWxlLmFwcFwiLFxuICAgICAgaWNvbjogc2hlbGxUYWJJY29uKFwic21hcnRwaG9uZVwiKSxcbiAgICAgIG5hbWU6IHNoZWxsTGFiZWwoXCJ1aS5tb2JpbGVQYW5lbC5hcHBcIiksXG4gICAgICBvcmRlcjogOTksXG4gICAgICB0cmVlOiB7XG4gICAgICAgIHNlY3Rpb25zOiBbXG4gICAgICAgICAge1xuICAgICAgICAgICAgaWQ6IFwiZnJhbWV3b3JrLm1vYmlsZS5hcHAucm9vdFwiLFxuICAgICAgICAgICAgbGFiZWw6IFwiXCIsXG4gICAgICAgICAgICBpdGVtczogW1xuICAgICAgICAgICAgICAuLi4oZXhhbXBsZVNlbGVjdEVsZW1lbnQgPyBbeyBpZDogXCJmcmFtZXdvcmsubW9iaWxlLmFwcC5leGFtcGxlXCIsIGxhYmVsOiBcIlwiLCBjb250cm9sOiBleGFtcGxlU2VsZWN0RWxlbWVudCB9XSA6IFtdKSxcbiAgICAgICAgICAgICAgLi4uKG1vZGVTd2l0Y2hlckVsZW1lbnQgPyBbeyBpZDogXCJmcmFtZXdvcmsubW9iaWxlLmFwcC5tb2Rlc1wiLCBsYWJlbDogXCJcIiwgY29udHJvbDogbW9kZVN3aXRjaGVyRWxlbWVudCB9XSA6IFtdKSxcbiAgICAgICAgICAgIF0sXG4gICAgICAgICAgfSxcbiAgICAgICAgXSxcbiAgICAgIH0sXG4gICAgfSk7XG4gICAgcmV0dXJuIFsuLi5hbmNob3JUYWJzLCBhcHBUYWJdO1xuICB9LCBbZGVmYXVsdERvY2ssIGV4YW1wbGVTZWxlY3RFbGVtZW50LCBtb2RlU3dpdGNoZXJFbGVtZW50XSk7XG5cbiAgLyoqIPCfl4TvuI8gU2tpcHMgdGhlIHZlcnkgZmlyc3QgKHByZS1oeWRyYXRpb24pIGNvbW1pdCBzbyBhIHBlcnNpc3RlZCBza2VsZXRvbiBpc24ndCBjbG9iYmVyZWQgd2l0aCBgbnVsbGAgYmVmb3JlIHRoZSBzZWVkaW5nIGVmZmVjdCBhYm92ZSBoYXMgYSBjaGFuY2UgdG8gcmVhZCBhbmQgYXBwbHkgaXQuICovXG4gIGNvbnN0IGRvY2tQZXJzaXN0ZWRPbmNlUmVmID0gdXNlUmVmKGZhbHNlKTtcbiAgdXNlRWZmZWN0KCgpID0+IHtcbiAgICBpZiAoIWRvY2tQZXJzaXN0ZWRPbmNlUmVmLmN1cnJlbnQpIHtcbiAgICAgIGRvY2tQZXJzaXN0ZWRPbmNlUmVmLmN1cnJlbnQgPSB0cnVlO1xuICAgICAgcmV0dXJuO1xuICAgIH1cbiAgICBjb25zdCBuZXh0U2tlbGV0b24gPSBkb2NrU2tlbGV0b25PZihkb2NrKTtcbiAgICBjb25zdCBkZWZhdWx0U2tlbGV0b24gPSBkb2NrU2tlbGV0b25PZihkZWZhdWx0RG9jayk7XG4gICAgZG9ja0xheW91dFN0b3JlLnNhdmUoZG9ja1NrZWxldG9uc0VxdWFsKG5leHRTa2VsZXRvbiwgZGVmYXVsdFNrZWxldG9uKSA/IG51bGwgOiBuZXh0U2tlbGV0b24pO1xuICB9LCBbZG9jaywgZGVmYXVsdERvY2ssIGRvY2tMYXlvdXRTdG9yZV0pO1xuXG4gIHVzZUVmZmVjdCgoKSA9PiB7XG4gICAgZGlzcGF0Y2goeyB0eXBlOiBcIkhZRFJBVEVfRE9DS19VSVwiLCB2YWx1ZTogZG9ja1VpU3RhdGVTdG9yZS5nZXRTbmFwc2hvdCgpIH0pO1xuICB9LCBbZG9ja1VpU3RhdGVTdG9yZV0pO1xuXG4gIC8qKiDwn5eE77iPIFNhbWUgZmlyc3QtY29tbWl0LXNraXAgYXMgdGhlIGRvY2sgc2tlbGV0b24gZWZmZWN0IGFib3ZlLCBidXQgYWxzbyByZS1hcm1zIHdoZW4gdGhlIHN0b3JlIGlkZW50aXR5IGl0c2VsZiBjaGFuZ2VzIChhcHAgc3dpdGNoKSDigJQgb3RoZXJ3aXNlIHRoZSBuZXcgYXBwJ3MgcHJlLWh5ZHJhdGlvbiBzdGF0ZSB3b3VsZCBiZSB3cml0dGVuIGludG8gaXRzIG93biBrZXkgb24gdGhlIGZpcnN0IHBvc3Qtc3dpdGNoIGNvbW1pdC4gKi9cbiAgY29uc3QgZG9ja1VpUGVyc2lzdGVkT25jZVJlZiA9IHVzZVJlZihmYWxzZSk7XG4gIGNvbnN0IGRvY2tVaVBlcnNpc3RlZFN0b3JlUmVmID0gdXNlUmVmKGRvY2tVaVN0YXRlU3RvcmUpO1xuICB1c2VFZmZlY3QoKCkgPT4ge1xuICAgIGlmIChkb2NrVWlQZXJzaXN0ZWRTdG9yZVJlZi5jdXJyZW50ICE9PSBkb2NrVWlTdGF0ZVN0b3JlKSB7XG4gICAgICBkb2NrVWlQZXJzaXN0ZWRTdG9yZVJlZi5jdXJyZW50ID0gZG9ja1VpU3RhdGVTdG9yZTtcbiAgICAgIGRvY2tVaVBlcnNpc3RlZE9uY2VSZWYuY3VycmVudCA9IGZhbHNlO1xuICAgIH1cbiAgICBpZiAoIWRvY2tVaVBlcnNpc3RlZE9uY2VSZWYuY3VycmVudCkge1xuICAgICAgZG9ja1VpUGVyc2lzdGVkT25jZVJlZi5jdXJyZW50ID0gdHJ1ZTtcbiAgICAgIHJldHVybjtcbiAgICB9XG4gICAgY29uc3QgYW5jaG9yczogUGFydGlhbDxSZWNvcmQ8QW5jaG9yLCBEb2NrVWlQYW5lbFN0YXRlPj4gPSB7fTtcbiAgICBmb3IgKGNvbnN0IGFuY2hvciBvZiBBTkNIT1JTKSB7XG4gICAgICBjb25zdCBwYW5lbFN0YXRlID0gcGFuZWxzW2FuY2hvcl07XG4gICAgICBjb25zdCBlbnRyeTogRG9ja1VpUGFuZWxTdGF0ZSA9IHt9O1xuICAgICAgaWYgKHBhbmVsU3RhdGUudmlzaWJsZSkgZW50cnkudmlzaWJsZSA9IHRydWU7XG4gICAgICBpZiAocGFuZWxTdGF0ZS5zaXplICE9PSBERUZBVUxUX1BBTkVMX1dJRFRIX1BYKSBlbnRyeS5zaXplID0gcGFuZWxTdGF0ZS5zaXplO1xuICAgICAgaWYgKHBhbmVsU3RhdGUucGF0aC5sZW5ndGggPiAwKSBlbnRyeS5wYXRoID0gcGFuZWxTdGF0ZS5wYXRoO1xuICAgICAgaWYgKE9iamVjdC5rZXlzKGVudHJ5KS5sZW5ndGggPiAwKSBhbmNob3JzW2FuY2hvcl0gPSBlbnRyeTtcbiAgICB9XG4gICAgY29uc3QgaGFzUGF0aE1lbW9yeSA9IE9iamVjdC5rZXlzKHBhbmVsUGF0aE1lbW9yeSkubGVuZ3RoID4gMDtcbiAgICBjb25zdCBoYXNUcmVlT3BlbiA9IE9iamVjdC5rZXlzKHRyZWVPcGVuU3RhdGVzKS5sZW5ndGggPiAwO1xuICAgIGNvbnN0IGlzRGVmYXVsdCA9IE9iamVjdC5rZXlzKGFuY2hvcnMpLmxlbmd0aCA9PT0gMCAmJiAhaGFzUGF0aE1lbW9yeSAmJiAhaGFzVHJlZU9wZW47XG4gICAgZG9ja1VpU3RhdGVTdG9yZS5zYXZlKGlzRGVmYXVsdCA/IG51bGwgOiB7IHZlcnNpb246IDMsIGFuY2hvcnMsIHBhdGhNZW1vcnk6IGhhc1BhdGhNZW1vcnkgPyBwYW5lbFBhdGhNZW1vcnkgOiB1bmRlZmluZWQsIHRyZWVPcGVuOiBoYXNUcmVlT3BlbiA/IHRyZWVPcGVuU3RhdGVzIDogdW5kZWZpbmVkIH0pO1xuICB9LCBbcGFuZWxzLCBwYW5lbFBhdGhNZW1vcnksIHRyZWVPcGVuU3RhdGVzLCBkb2NrVWlTdGF0ZVN0b3JlXSk7XG5cbiAgY29uc3QgaGFuZGxlVGFiRG9ja0Ryb3AgPSB1c2VDYWxsYmFjayhcbiAgICAobW92ZTogUGFuZWxUYWJEb2NrTW92ZSkgPT4ge1xuICAgICAgY29uc3QgbmV4dERvY2sgPSBtb3ZlVGFiSW5Eb2NrKGRvY2ssIG1vdmUpO1xuICAgICAgaWYgKG5leHREb2NrID09PSBkb2NrKSByZXR1cm47XG4gICAgICBjb25zdCBuZXh0U2tlbGV0b24gPSBkb2NrU2tlbGV0b25PZihuZXh0RG9jayk7XG4gICAgICBjb25zdCBkZWZhdWx0U2tlbGV0b24gPSBkb2NrU2tlbGV0b25PZihkZWZhdWx0RG9jayk7XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0RPQ0tfT1ZFUlJJREVcIiwgdmFsdWU6IGRvY2tTa2VsZXRvbnNFcXVhbChuZXh0U2tlbGV0b24sIGRlZmF1bHRTa2VsZXRvbikgPyBudWxsIDogbmV4dFNrZWxldG9uIH0pO1xuICAgICAgY29uc3QgdGFyZ2V0UGF0aCA9IGZpbmRQYW5lbFRhYlBhdGgobmV4dERvY2suYW5jaG9yc1ttb3ZlLnRhcmdldC5hbmNob3JdLCBtb3ZlLnRhYklkKTtcbiAgICAgIGlmICh0YXJnZXRQYXRoKSBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1BBTkVMX1BBVEhcIiwgYW5jaG9yOiBtb3ZlLnRhcmdldC5hbmNob3IsIHZhbHVlOiB0YXJnZXRQYXRoIH0pO1xuICAgICAgaWYgKG1vdmUuZnJvbUFuY2hvciAhPT0gbW92ZS50YXJnZXQuYW5jaG9yKSB7XG4gICAgICAgIGNvbnN0IHNvdXJjZVRhYnMgPSBuZXh0RG9jay5hbmNob3JzW21vdmUuZnJvbUFuY2hvcl07XG4gICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfUEFORUxfUEFUSFwiLCBhbmNob3I6IG1vdmUuZnJvbUFuY2hvciwgdmFsdWU6IChwcmV2KSA9PiByZWNvbmNpbGVBY3RpdmVQYXRoKHNvdXJjZVRhYnMsIHByZXYsIHBhbmVsVGFiQ2hpbGRyZW4pIH0pO1xuICAgICAgfVxuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9QQU5FTF9WSVNJQkxFXCIsIGFuY2hvcjogbW92ZS50YXJnZXQuYW5jaG9yLCB2YWx1ZTogdHJ1ZSB9KTtcbiAgICAgIG5vdGVTaGVsbENvbW1hbmQoXCJzaGVsbC5kb2NrTW92ZVwiLCBzaGVsbExhYmVsKFwidWkuc2hlbGxDb21tYW5kLmRvY2tNb3ZlXCIpLCB7IHRhYklkOiBtb3ZlLnRhYklkLCBmcm9tQW5jaG9yOiBtb3ZlLmZyb21BbmNob3IsIHRvQW5jaG9yOiBtb3ZlLnRhcmdldC5hbmNob3IgfSk7XG4gICAgfSxcbiAgICBbZG9jaywgZGVmYXVsdERvY2ssIG5vdGVTaGVsbENvbW1hbmRdLFxuICApO1xuXG4gIGNvbnN0IGhhbmRsZVRyZWVVbml0RG9ja0Ryb3AgPSB1c2VDYWxsYmFjayhcbiAgICAobW92ZTogUGFuZWxUcmVlVW5pdERvY2tNb3ZlKSA9PiB7XG4gICAgICBjb25zdCBuZXh0RG9jayA9IG1vdmVUcmVlVW5pdEluRG9jayhkb2NrLCBtb3ZlKTtcbiAgICAgIGlmIChuZXh0RG9jayA9PT0gZG9jaykgcmV0dXJuO1xuICAgICAgY29uc3QgbmV4dFNrZWxldG9uID0gZG9ja1NrZWxldG9uT2YobmV4dERvY2spO1xuICAgICAgY29uc3QgZGVmYXVsdFNrZWxldG9uID0gZG9ja1NrZWxldG9uT2YoZGVmYXVsdERvY2spO1xuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9ET0NLX09WRVJSSURFXCIsIHZhbHVlOiBkb2NrU2tlbGV0b25zRXF1YWwobmV4dFNrZWxldG9uLCBkZWZhdWx0U2tlbGV0b24pID8gbnVsbCA6IG5leHRTa2VsZXRvbiB9KTtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfUEFORUxfVklTSUJMRVwiLCBhbmNob3I6IG1vdmUudGFyZ2V0LmFuY2hvciwgdmFsdWU6IHRydWUgfSk7XG4gICAgICBub3RlU2hlbGxDb21tYW5kKFwic2hlbGwuZG9ja01vdmVcIiwgc2hlbGxMYWJlbChcInVpLnNoZWxsQ29tbWFuZC5kb2NrTW92ZVwiKSwgeyB0b0FuY2hvcjogbW92ZS50YXJnZXQuYW5jaG9yIH0pO1xuICAgIH0sXG4gICAgW2RvY2ssIGRlZmF1bHREb2NrLCBub3RlU2hlbGxDb21tYW5kXSxcbiAgKTtcblxuICBjb25zdCBzdHVkaW9PdmVycmlkZVRhYklkID0gc3R1ZGlvTW9kZSAmJiBzZXNzaW9uPy5hcHAuaWQgPT09IGhvc3RBcHBJZCA/IChwYW5lbD8uYWN0aXZlUGFuZWxUYWIgPz8gaG9zdENhdGFsb2d1ZVRhYklkKSA6IHVuZGVmaW5lZDtcbiAgY29uc3Qgc3R1ZGlvT3ZlcnJpZGVBbmNob3IgPSBzdHVkaW9PdmVycmlkZVRhYklkID8gZmluZFBhbmVsVGFiSW5Eb2NrKGRvY2ssIHN0dWRpb092ZXJyaWRlVGFiSWQpPy5hbmNob3IgOiB1bmRlZmluZWQ7XG4gIGNvbnN0IGRldGFpbHNPdmVycmlkZVRhYklkID0gcGFuZWw/LmFjdGl2ZVBhbmVsVGFiO1xuICBjb25zdCBkZXRhaWxzT3ZlcnJpZGVBbmNob3IgPSBkZXRhaWxzT3ZlcnJpZGVUYWJJZCA/IGZpbmRQYW5lbFRhYkluRG9jayhkb2NrLCBkZXRhaWxzT3ZlcnJpZGVUYWJJZCk/LmFuY2hvciA6IHVuZGVmaW5lZDtcblxuICAvKiogQGVtb2ppIPCfjpPvuI8gVGhlIGN1cnJlbnQgaW50cm9kdWN0aW9uIHN0ZXAncyB0YXJnZXQgZWxlbWVudCBpZHMgKGBpbnRyb2R1Y2VgICsgYHNob3dgKSwgY2xhc3NpZmllZCBieVxuICAgKiBzaGFwZSDigJQgYG51bGxgIHVubGVzcyB0aGF0IHNoYXBlIGlzIHByZXNlbnQsIHNvIGV2ZXJ5IHJldmVhbCBvdmVycmlkZSBiZWxvdyAoaGVyZSBhbmQgaW5cbiAgICogYG1vZGVXaW5kb3dzYCkgaXMgYSBwbGFpbiB0cnV0aGluZXNzIGNoZWNrLiBBIGZvbGRlZCB1dGlsaXR5IGJhci9BY3Rpb25zIHJhaWwvZG9jayBwYW5lbCB3b3VsZFxuICAgKiBvdGhlcndpc2UgaGlkZSB0aGUgdGFyZ2V0IGZyb20gZXZlciBtb3VudGluZyAoc2VlIGB1c2VJbnRyb2R1Y3Rpb25BbmNob3JSZWN0YCksIGxlYXZpbmcgdGhlIHN0ZXBcbiAgICogY2VudGVyZWQgd2l0aCBubyBjdXRvdXQgYW5kIG5vIHdheSBmb3IgdGhlIHVzZXIgdG8gZmluZCB3aGF0IHRvIGRvLiBJZHMgYXJlIG1hdGNoZWQsIG5ldmVyXG4gICAqIHJlY29uc3RydWN0ZWQ6IGEgYGZyYW1ld29yay53aW5kb3cue3NlZ21lbnR9YCBpZCdzIHNlZ21lbnQgaXMgYGVsZW1lbnRJZFNlZ21lbnQod2luZG93SWQpYCwgYSBsb3NzeVxuICAgKiBjYW1lbENhc2Ugbm9ybWFsaXphdGlvbiDigJQgY29tcGFyaW5nIGBlbGVtZW50SWRTZWdtZW50KHdpbmRvd0lkKSA9PT0gc2VnbWVudGAgT1IgdGhlIHNhbWUgZm9yIHRoZVxuICAgKiBpbnN0YW5jZSdzIHdpbmRvdy1raW5kIGlkIGlzIHRoZSBvbmx5IHNhZmUgY2hlY2sgKFRvcC9QZXJzcGVjdGl2ZSBpbnN0YW5jZXMgc2hhcmUgYSBraW5kKS4gKi9cbiAgY29uc3QgYWN0aXZlSW50cm9kdWN0aW9uU3RlcCA9IGFjdGl2ZUludHJvZHVjdGlvbiAmJiBpbnRyb2R1Y3Rpb25TdGVwSW5kZXggIT0gbnVsbCA/IChhY3RpdmVJbnRyb2R1Y3Rpb24uc3RlcHNbaW50cm9kdWN0aW9uU3RlcEluZGV4XSA/PyBudWxsKSA6IG51bGw7XG4gIGNvbnN0IGludHJvZHVjdGlvbkVsZW1lbnRJZHMgPSB1c2VNZW1vKFxuICAgICgpOiByZWFkb25seSBzdHJpbmdbXSA9PiAoYWN0aXZlSW50cm9kdWN0aW9uU3RlcCA/IFthY3RpdmVJbnRyb2R1Y3Rpb25TdGVwLmludHJvZHVjZSwgLi4uYWN0aXZlSW50cm9kdWN0aW9uU3RlcC5zaG93XS5maWx0ZXIoKGlkKTogaWQgaXMgc3RyaW5nID0+IEJvb2xlYW4oaWQpKSA6IFtdKSxcbiAgICBbYWN0aXZlSW50cm9kdWN0aW9uU3RlcF0sXG4gICk7XG4gIGNvbnN0IGludHJvZHVjdGlvblV0aWxpdHlJZCA9IHVzZU1lbW8oKCkgPT4ge1xuICAgIGlmICghc2Vzc2lvbikgcmV0dXJuIG51bGw7XG4gICAgY29uc3QgdXRpbGl0aWVzID0gc2Vzc2lvbi5hcHAudXRpbGl0aWVzID8/IFtdO1xuICAgIHJldHVybiBpbnRyb2R1Y3Rpb25FbGVtZW50SWRzLmZpbmQoKGlkKSA9PiB1dGlsaXRpZXMuc29tZSgodXRpbGl0eSkgPT4gdXRpbGl0eS5pZCA9PT0gaWQpKSA/PyBudWxsO1xuICB9LCBbaW50cm9kdWN0aW9uRWxlbWVudElkcywgc2Vzc2lvbl0pO1xuICBjb25zdCBpbnRyb2R1Y3Rpb25BY3Rpb25XaW5kb3dTZWdtZW50ID0gdXNlTWVtbygoKSA9PiB7XG4gICAgZm9yIChjb25zdCBpZCBvZiBpbnRyb2R1Y3Rpb25FbGVtZW50SWRzKSB7XG4gICAgICBjb25zdCByZXN0ID0gaWQuc3RhcnRzV2l0aChcImZyYW1ld29yay53aW5kb3cuXCIpID8gaWQuc2xpY2UoXCJmcmFtZXdvcmsud2luZG93LlwiLmxlbmd0aCkgOiBudWxsO1xuICAgICAgY29uc3QgYWN0aW9uSW5kZXggPSByZXN0Py5pbmRleE9mKFwiLmFjdGlvbi5cIikgPz8gLTE7XG4gICAgICBpZiAocmVzdCAmJiBhY3Rpb25JbmRleCA+PSAwKSByZXR1cm4gcmVzdC5zbGljZSgwLCBhY3Rpb25JbmRleCk7XG4gICAgfVxuICAgIHJldHVybiBudWxsO1xuICB9LCBbaW50cm9kdWN0aW9uRWxlbWVudElkc10pO1xuICBjb25zdCBpbnRyb2R1Y3Rpb25QYW5lbFRhYklkID0gdXNlTWVtbygoKSA9PiB7XG4gICAgZm9yIChjb25zdCBpZCBvZiBpbnRyb2R1Y3Rpb25FbGVtZW50SWRzKSB7XG4gICAgICBpZiAoaWQuc3RhcnRzV2l0aChcImZyYW1ld29yay5wYW5lbFRhYi5cIikpIHtcbiAgICAgICAgY29uc3QgcmVzdCA9IGlkLnNsaWNlKFwiZnJhbWV3b3JrLnBhbmVsVGFiLlwiLmxlbmd0aCk7XG4gICAgICAgIHJldHVybiByZXN0LmVuZHNXaXRoKFwiLmZpcnN0RHJhZ2dhYmxlXCIpID8gcmVzdC5zbGljZSgwLCAtXCIuZmlyc3REcmFnZ2FibGVcIi5sZW5ndGgpIDogcmVzdDtcbiAgICAgIH1cbiAgICB9XG4gICAgcmV0dXJuIG51bGw7XG4gIH0sIFtpbnRyb2R1Y3Rpb25FbGVtZW50SWRzXSk7XG4gIC8qKiDwn5ug77iPIFRvb2wgaWRzIHRoZSBhY3RpdmUgc3RlcCBhc2tzIHRoZSB1c2VyIHRvIGFjdGl2YXRlIChgaW50ZXJhY3Rpb25zYCBvZiBraW5kIGB0b29sYCwgb3IgYSBiYXJlXG4gICAqIGB0b29sLjxpZD5gIGludHJvZHVjZS9zaG93KS4gUmV2ZWFscyB0aGUgVG9vbCBjYXRlZ29yeSBjaHJvbWUgc28gdGhlIGxlYWYgdGFiIGNhbiBiZSBwcmVzc2VkIOKAlFxuICAgKiBuZXZlciBkcmlsbHMgaW50byB0aGUgbGVhZiBpdHNlbGYgKHRoYXQgd291bGQgb3BlbiB0aGUgaW5hY3RpdmUgYWN0aXZhdGUtdG9nZ2xlIHRyZWUgYW5kLCB2aWEgdGFiXG4gICAqIHNlbGVjdGlvbiwgYXV0by1hY3RpdmF0ZSArIGNlbGVicmF0ZSBiZWZvcmUgdGhlIHVzZXIgYWN0cykuICovXG4gIGNvbnN0IGludHJvZHVjdGlvblRvb2xQaWNrSWRzID0gdXNlTWVtbygoKTogcmVhZG9ubHkgc3RyaW5nW10gPT4ge1xuICAgIGNvbnN0IGZyb21JbnRlcmFjdGlvbnMgPSAoYWN0aXZlSW50cm9kdWN0aW9uU3RlcD8uaW50ZXJhY3Rpb25zID8/IFtdKVxuICAgICAgLmZpbHRlcigoaW50ZXJhY3Rpb24pOiBpbnRlcmFjdGlvbiBpcyBJbnRyb2R1Y3Rpb25JbnRlcmFjdGlvbiAmIHsgcmVhZG9ubHkgb246IHsgcmVhZG9ubHkga2luZDogXCJ0b29sXCI7IHJlYWRvbmx5IGlkOiBzdHJpbmcgfSB9ID0+IGludGVyYWN0aW9uLm9uLmtpbmQgPT09IFwidG9vbFwiKVxuICAgICAgLm1hcCgoaW50ZXJhY3Rpb24pID0+IGludGVyYWN0aW9uLm9uLmlkKTtcbiAgICBpZiAoZnJvbUludGVyYWN0aW9ucy5sZW5ndGggPiAwKSByZXR1cm4gZnJvbUludGVyYWN0aW9ucztcbiAgICByZXR1cm4gaW50cm9kdWN0aW9uRWxlbWVudElkcy5mbGF0TWFwKChpZCkgPT4ge1xuICAgICAgY29uc3QgbWF0Y2ggPSAvXnRvb2xcXC4oW2Etel1bYS16QS1aMC05XSopJC8uZXhlYyhpZCk7XG4gICAgICByZXR1cm4gbWF0Y2g/LlsxXSA/IFttYXRjaFsxXV0gOiBbXTtcbiAgICB9KTtcbiAgfSwgW2FjdGl2ZUludHJvZHVjdGlvblN0ZXAsIGludHJvZHVjdGlvbkVsZW1lbnRJZHNdKTtcbiAgY29uc3QgaW50cm9kdWN0aW9uUGFuZWxUYWJBbmNob3IgPSBpbnRyb2R1Y3Rpb25QYW5lbFRhYklkID8gZmluZFBhbmVsVGFiSW5Eb2NrKGRvY2ssIGludHJvZHVjdGlvblBhbmVsVGFiSWQpPy5hbmNob3IgOiB1bmRlZmluZWQ7XG4gIGNvbnN0IGludHJvZHVjdGlvblV0aWxpdHlXaW5kb3dJZCA9IHVzZU1lbW8oKCkgPT4ge1xuICAgIGlmICghaW50cm9kdWN0aW9uVXRpbGl0eUlkIHx8ICFzZXNzaW9uKSByZXR1cm4gbnVsbDtcbiAgICBmb3IgKGNvbnN0IGtpbmQgb2Ygc2Vzc2lvbi5hcHAud2luZG93S2luZHMpIHtcbiAgICAgIGNvbnN0IHV0aWxpdGllcyA9IHJlc29sdmVVdGlsaXR5Tm9kZXMoc2Vzc2lvbi5hcHAsIGtpbmQsIG51bGwsIGtpbmQuaWQsIGFwcExhYmVsc092ZXJsYXksIHVpVGVybWlub2xvZ3ksIHVpTG9jYWxlKTtcbiAgICAgIGlmICh1dGlsaXR5Tm9kZVRyZWVDb250YWluc0lkKHV0aWxpdGllcywgaW50cm9kdWN0aW9uVXRpbGl0eUlkKSkgcmV0dXJuIGtpbmQuaWQ7XG4gICAgfVxuICAgIHJldHVybiBudWxsO1xuICB9LCBbYXBwTGFiZWxzT3ZlcmxheSwgaW50cm9kdWN0aW9uVXRpbGl0eUlkLCBzZXNzaW9uLCB1aVRlcm1pbm9sb2d5LCB1aUxvY2FsZV0pO1xuICAvKiog8J+Ok++4jyBXaW5kb3cta2luZCBpZCB3aG9zZSBtZWFzdXJlcyB0cmVlIG93bnMgYW4gaW50cm9kdWNlL3Nob3cgbWVhc3VyZSBpZCDigJQgZm9yY2UtdW5mb2xkcyB0aGUgV2luZG93XG4gICAqIE9wdGlvbnMgcmFpbCBzbyB0YXJnZXRzIGxpa2UgYHB1enpsZTNkLXBsYXktdm9ydGV4LXNob3dgIGNhbiBtb3VudCBmb3IgdGhlIHRvdXIuICovXG4gIGNvbnN0IGludHJvZHVjdGlvbk1lYXN1cmVXaW5kb3dJZCA9IHVzZU1lbW8oKCkgPT4ge1xuICAgIGlmICghc2Vzc2lvbiB8fCBpbnRyb2R1Y3Rpb25FbGVtZW50SWRzLmxlbmd0aCA9PT0gMCkgcmV0dXJuIG51bGw7XG4gICAgZm9yIChjb25zdCBraW5kIG9mIHNlc3Npb24uYXBwLndpbmRvd0tpbmRzKSB7XG4gICAgICBjb25zdCBraW5kTWVhc3VyZXMgPSBraW5kLm9wdGlvbnMubWVhc3VyZXMgPz8gW107XG4gICAgICBpZiAoaW50cm9kdWN0aW9uRWxlbWVudElkcy5zb21lKChpZCkgPT4gd2luZG93TWVhc3VyZVRyZWVDb250YWluc0lkKGtpbmRNZWFzdXJlcywgaWQpKSkgcmV0dXJuIGtpbmQuaWQ7XG4gICAgICBmb3IgKGNvbnN0IFt3aW5kb3dJZCwgbWVhc3VyZXNdIG9mIE9iamVjdC5lbnRyaWVzKHdpbmRvd01lYXN1cmVzQnlXaW5kb3dJZCkpIHtcbiAgICAgICAgaWYgKCFpbnRyb2R1Y3Rpb25FbGVtZW50SWRzLnNvbWUoKGlkKSA9PiB3aW5kb3dNZWFzdXJlVHJlZUNvbnRhaW5zSWQobWVhc3VyZXMsIGlkKSkpIGNvbnRpbnVlO1xuICAgICAgICBpZiAod2luZG93SWQgPT09IGtpbmQuaWQgfHwgZXh0cmFXaW5kb3dJbnN0YW5jZXMuc29tZSgoaW5zdGFuY2UpID0+IGluc3RhbmNlLmlkID09PSB3aW5kb3dJZCAmJiBpbnN0YW5jZS53aW5kb3dLaW5kSWQgPT09IGtpbmQuaWQpKSByZXR1cm4ga2luZC5pZDtcbiAgICAgIH1cbiAgICB9XG4gICAgcmV0dXJuIG51bGw7XG4gIH0sIFtleHRyYVdpbmRvd0luc3RhbmNlcywgaW50cm9kdWN0aW9uRWxlbWVudElkcywgc2Vzc2lvbiwgd2luZG93TWVhc3VyZXNCeVdpbmRvd0lkXSk7XG5cbiAgLyoqIPCfm6DvuI8gVG9vbCBpZCB3aG9zZSBtZWFzdXJlIHRyZWUgb3ducyBhbiBpbnRyb2R1Y2Uvc2hvdyBpZCDigJQga2VlcHMgbW9kZS1sZXZlbCB0b29scyBsaWtlIGZpbGxcbiAgICogYWN0aXZlIHNvIHRhcmdldHMgc3VjaCBhcyBgcHV6emxlM2QtcGxheS1kaXN0cmlidXRpb25gIHN0YXkgbW91bnRlZCBmb3IgdGhlIHRvdXIuICovXG4gIGNvbnN0IGludHJvZHVjdGlvblRvb2xJZCA9IHVzZU1lbW8oKCkgPT4ge1xuICAgIGlmIChpbnRyb2R1Y3Rpb25FbGVtZW50SWRzLmxlbmd0aCA9PT0gMCkgcmV0dXJuIG51bGw7XG4gICAgZm9yIChjb25zdCBbdG9vbElkLCBtZWFzdXJlc10gb2YgT2JqZWN0LmVudHJpZXModG9vbE1lYXN1cmVzQnlUb29sSWQpKSB7XG4gICAgICBpZiAoaW50cm9kdWN0aW9uRWxlbWVudElkcy5zb21lKChpZCkgPT4gd2luZG93TWVhc3VyZVRyZWVDb250YWluc0lkKG1lYXN1cmVzLCBpZCkpKSByZXR1cm4gdG9vbElkO1xuICAgIH1cbiAgICByZXR1cm4gbnVsbDtcbiAgfSwgW2ludHJvZHVjdGlvbkVsZW1lbnRJZHMsIHRvb2xNZWFzdXJlc0J5VG9vbElkXSk7XG5cbiAgY29uc3QgbGFzdEludHJvZHVjdGlvblRvb2xJZFJlZiA9IHVzZVJlZjxzdHJpbmcgfCBudWxsPihudWxsKTtcbiAgdXNlRWZmZWN0KCgpID0+IHtcbiAgICBpZiAoIWludHJvZHVjdGlvblRvb2xJZCB8fCAhc2Vzc2lvbikge1xuICAgICAgbGFzdEludHJvZHVjdGlvblRvb2xJZFJlZi5jdXJyZW50ID0gbnVsbDtcbiAgICAgIHJldHVybjtcbiAgICB9XG4gICAgaWYgKGxhc3RJbnRyb2R1Y3Rpb25Ub29sSWRSZWYuY3VycmVudCA9PT0gaW50cm9kdWN0aW9uVG9vbElkICYmIGFjdGl2ZVRvb2xJZFJlZi5jdXJyZW50ID09PSBpbnRyb2R1Y3Rpb25Ub29sSWQpIHJldHVybjtcbiAgICBsYXN0SW50cm9kdWN0aW9uVG9vbElkUmVmLmN1cnJlbnQgPSBpbnRyb2R1Y3Rpb25Ub29sSWQ7XG4gICAgaWYgKGFjdGl2ZVRvb2xJZFJlZi5jdXJyZW50ID09PSBpbnRyb2R1Y3Rpb25Ub29sSWQpIHJldHVybjtcbiAgICBvbkFjdGlvblN0YWJsZSh7IGNvbnRyb2xsZXJJZDogc2Vzc2lvbi5hcHAuY29udHJvbGxlcklkLCBhY3Rpb246IFNFVF9BQ1RJVkVfVE9PTF9BQ1RJT05fSUQsIGFyZ3M6IHsgdG9vbElkOiBpbnRyb2R1Y3Rpb25Ub29sSWQgfSB9KTtcbiAgfSwgW2ludHJvZHVjdGlvblRvb2xJZCwgb25BY3Rpb25TdGFibGUsIHNlc3Npb25dKTtcblxuICAvKiog8J+boO+4jyBUb29sLXBpY2sgc3RlcHMgKGUuZy4gRsO8bGxlbik6IG9wZW4gdGhlIFRvb2wgY2F0ZWdvcnkgc28gYHRvb2wuPGlkPmAgbGVhZiB0YWJzIG1vdW50IGluIHRoZVxuICAgKiBwYW5lbCBjaHJvbWUsIGNsZWFyIGFueSBhbHJlYWR5LWFjdGl2ZSB0b29sIHNvIHRoZSB1c2VyIG11c3QgYWN0aXZhdGUgaXQsIGFuZCBuZXZlciBzZWxlY3QgdGhlXG4gICAqIGxlYWYgcGF0aCAoc2VsZWN0aW5nIGF1dG8tYWN0aXZhdGVzIGFuZCB3b3VsZCBjZWxlYnJhdGUgYmVmb3JlIHRoZXkgYWN0KS4gKi9cbiAgY29uc3QgbGFzdEludHJvZHVjdGlvblRvb2xQaWNrU3RlcElkUmVmID0gdXNlUmVmPHN0cmluZyB8IG51bGw+KG51bGwpO1xuICB1c2VFZmZlY3QoKCkgPT4ge1xuICAgIGlmICghc2Vzc2lvbiB8fCBpbnRyb2R1Y3Rpb25Ub29sUGlja0lkcy5sZW5ndGggPT09IDAgfHwgIWFjdGl2ZUludHJvZHVjdGlvblN0ZXApIHtcbiAgICAgIGxhc3RJbnRyb2R1Y3Rpb25Ub29sUGlja1N0ZXBJZFJlZi5jdXJyZW50ID0gbnVsbDtcbiAgICAgIHJldHVybjtcbiAgICB9XG4gICAgLy8g8J+boO+4jyBNZWFzdXJlLWRyaXZlbiBrZWVwLWFsaXZlIChgaW50cm9kdWN0aW9uVG9vbElkYCkgb3ducyBhY3RpdmF0aW9uIGZvciBzdGVwcyB0aGF0IGludHJvZHVjZVxuICAgIC8vIHRvb2wgbWVhc3VyZXMgKGZpbGwtZGlzdHJpYnV0aW9uKSDigJQgZG9uJ3QgZmlnaHQgaXQgYnkgY2xlYXJpbmcgdGhlIHRvb2wuXG4gICAgaWYgKGludHJvZHVjdGlvblRvb2xJZCkgcmV0dXJuO1xuICAgIGlmIChsYXN0SW50cm9kdWN0aW9uVG9vbFBpY2tTdGVwSWRSZWYuY3VycmVudCA9PT0gYWN0aXZlSW50cm9kdWN0aW9uU3RlcC5pZCkgcmV0dXJuO1xuICAgIGxhc3RJbnRyb2R1Y3Rpb25Ub29sUGlja1N0ZXBJZFJlZi5jdXJyZW50ID0gYWN0aXZlSW50cm9kdWN0aW9uU3RlcC5pZDtcbiAgICBmb3IgKGNvbnN0IHRvb2xJZCBvZiBpbnRyb2R1Y3Rpb25Ub29sUGlja0lkcykge1xuICAgICAgaWYgKGFjdGl2ZVRvb2xJZFJlZi5jdXJyZW50ID09PSB0b29sSWQpIHtcbiAgICAgICAgb25BY3Rpb25TdGFibGUoeyBjb250cm9sbGVySWQ6IHNlc3Npb24uYXBwLmNvbnRyb2xsZXJJZCwgYWN0aW9uOiBTRVRfQUNUSVZFX1RPT0xfQUNUSU9OX0lELCBhcmdzOiB7IHRvb2xJZDogXCJcIiB9IH0pO1xuICAgICAgfVxuICAgIH1cbiAgICBpZiAobW9iaWxlKSB7XG4gICAgICBjb25zdCByZXNvbHZlZCA9IGZpbmRQYW5lbFRhYlBhdGgobW9iaWxlUGFuZWxUYWJzLCBGUkFNRVdPUktfQ0FURUdPUllfVE9PTF9JRCk7XG4gICAgICBpZiAocmVzb2x2ZWQpIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfTU9CSUxFX1BBTkVMX1BBVEhcIiwgdmFsdWU6IHJlc29sdmVkIH0pO1xuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9NT0JJTEVfUEFORUxfVklTSUJMRVwiLCB2YWx1ZTogdHJ1ZSB9KTtcbiAgICAgIHJldHVybjtcbiAgICB9XG4gICAgY29uc3QgdG9vbEFuY2hvciA9IGZpbmRQYW5lbFRhYkluRG9jayhkb2NrLCBGUkFNRVdPUktfQ0FURUdPUllfVE9PTF9JRCk/LmFuY2hvciA/PyBcImJvdHRvbS1taWRkbGVcIjtcbiAgICBjb25zdCByZXNvbHZlZCA9IGZpbmRQYW5lbFRhYlBhdGgoZG9jay5hbmNob3JzW3Rvb2xBbmNob3JdLCBGUkFNRVdPUktfQ0FURUdPUllfVE9PTF9JRCk7XG4gICAgaWYgKHJlc29sdmVkKSBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1BBTkVMX1BBVEhcIiwgYW5jaG9yOiB0b29sQW5jaG9yLCB2YWx1ZTogcmVzb2x2ZWQgfSk7XG4gICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9QQU5FTF9WSVNJQkxFXCIsIGFuY2hvcjogdG9vbEFuY2hvciwgdmFsdWU6IHRydWUgfSk7XG4gIH0sIFthY3RpdmVJbnRyb2R1Y3Rpb25TdGVwLCBkb2NrLCBpbnRyb2R1Y3Rpb25Ub29sSWQsIGludHJvZHVjdGlvblRvb2xQaWNrSWRzLCBtb2JpbGUsIG1vYmlsZVBhbmVsVGFicywgb25BY3Rpb25TdGFibGUsIHNlc3Npb25dKTtcblxuICBjb25zdCBsYXN0SW50cm9kdWN0aW9uUGFuZWxUYWJJZFJlZiA9IHVzZVJlZjxzdHJpbmcgfCB1bmRlZmluZWQ+KHVuZGVmaW5lZCk7XG4gIHVzZUVmZmVjdCgoKSA9PiB7XG4gICAgaWYgKCFpbnRyb2R1Y3Rpb25QYW5lbFRhYklkIHx8ICFpbnRyb2R1Y3Rpb25QYW5lbFRhYkFuY2hvcikge1xuICAgICAgbGFzdEludHJvZHVjdGlvblBhbmVsVGFiSWRSZWYuY3VycmVudCA9IHVuZGVmaW5lZDtcbiAgICAgIHJldHVybjtcbiAgICB9XG4gICAgaWYgKGxhc3RJbnRyb2R1Y3Rpb25QYW5lbFRhYklkUmVmLmN1cnJlbnQgPT09IGludHJvZHVjdGlvblBhbmVsVGFiSWQpIHJldHVybjtcbiAgICBsYXN0SW50cm9kdWN0aW9uUGFuZWxUYWJJZFJlZi5jdXJyZW50ID0gaW50cm9kdWN0aW9uUGFuZWxUYWJJZDtcbiAgICBpZiAobW9iaWxlKSB7XG4gICAgICBjb25zdCByZXNvbHZlZCA9IGZpbmRQYW5lbFRhYlBhdGgobW9iaWxlUGFuZWxUYWJzLCBpbnRyb2R1Y3Rpb25QYW5lbFRhYklkKTtcbiAgICAgIGlmIChyZXNvbHZlZCkgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9NT0JJTEVfUEFORUxfUEFUSFwiLCB2YWx1ZTogcmVzb2x2ZWQgfSk7XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX01PQklMRV9QQU5FTF9WSVNJQkxFXCIsIHZhbHVlOiB0cnVlIH0pO1xuICAgICAgcmV0dXJuO1xuICAgIH1cbiAgICBjb25zdCByZXNvbHZlZCA9IGZpbmRQYW5lbFRhYlBhdGgoZG9jay5hbmNob3JzW2ludHJvZHVjdGlvblBhbmVsVGFiQW5jaG9yXSwgaW50cm9kdWN0aW9uUGFuZWxUYWJJZCk7XG4gICAgaWYgKHJlc29sdmVkKSBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1BBTkVMX1BBVEhcIiwgYW5jaG9yOiBpbnRyb2R1Y3Rpb25QYW5lbFRhYkFuY2hvciwgdmFsdWU6IHJlc29sdmVkIH0pO1xuICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfUEFORUxfVklTSUJMRVwiLCBhbmNob3I6IGludHJvZHVjdGlvblBhbmVsVGFiQW5jaG9yLCB2YWx1ZTogdHJ1ZSB9KTtcbiAgfSwgW2ludHJvZHVjdGlvblBhbmVsVGFiSWQsIGludHJvZHVjdGlvblBhbmVsVGFiQW5jaG9yLCBkb2NrLCBtb2JpbGUsIG1vYmlsZVBhbmVsVGFic10pO1xuXG4gIC8qKiDwn46T77iPIFBhbmVsIGludGVyYWN0aW9ucyBjb21wbGV0ZSB3aGVuIHRoZWlyIG5hbWVkIHBhbmVsIHRhYiBpcyBvcGVuIGFuZCB2aXNpYmxlIOKAlCBjaGVja2VkIGZvciBldmVyeVxuICAgKiBgcGFuZWxgIGludGVyYWN0aW9uIG9mIHRoZSBhY3RpdmUgc3RlcCwgbm90IGp1c3QgdGhlIGZpcnN0LCBzbyBhIHN0ZXAgY2FuIHJlcXVpcmUgb3BlbmluZyBzZXZlcmFsLiAqL1xuICB1c2VFZmZlY3QoKCkgPT4ge1xuICAgIGlmICghYWN0aXZlSW50cm9kdWN0aW9uU3RlcCkgcmV0dXJuO1xuICAgIGZvciAoY29uc3QgaW50ZXJhY3Rpb24gb2YgYWN0aXZlSW50cm9kdWN0aW9uU3RlcC5pbnRlcmFjdGlvbnMgPz8gW10pIHtcbiAgICAgIGlmIChpbnRlcmFjdGlvbi5vbi5raW5kICE9PSBcInBhbmVsXCIpIGNvbnRpbnVlO1xuICAgICAgY29uc3QgdGFiSWQgPSBpbnRlcmFjdGlvbi5vbi5pZDtcbiAgICAgIGNvbnN0IGxvY2F0ZWQgPSBmaW5kUGFuZWxUYWJJbkRvY2soZG9jaywgdGFiSWQpO1xuICAgICAgaWYgKCFsb2NhdGVkKSBjb250aW51ZTtcbiAgICAgIGNvbnN0IHBhbmVsID0gcGFuZWxzW2xvY2F0ZWQuYW5jaG9yXTtcbiAgICAgIGlmICghcGFuZWwudmlzaWJsZSB8fCAhcGFuZWwucGF0aC5pbmNsdWRlcyh0YWJJZCkpIGNvbnRpbnVlO1xuICAgICAgY29tcGxldGVJbnRyb2R1Y3Rpb25JbnRlcmFjdGlvbigoY2FuZGlkYXRlKSA9PiBjYW5kaWRhdGUub24ua2luZCA9PT0gXCJwYW5lbFwiICYmIGNhbmRpZGF0ZS5vbi5pZCA9PT0gdGFiSWQpO1xuICAgIH1cbiAgfSwgW2FjdGl2ZUludHJvZHVjdGlvblN0ZXAsIGNvbXBsZXRlSW50cm9kdWN0aW9uSW50ZXJhY3Rpb24sIGRvY2ssIHBhbmVsc10pO1xuXG4gIC8qKiDwn46T77iPIEV4cGFuZCBpbnRlcmFjdGlvbnMgc3RhcnQgd2l0aCBldmVyeSBuYW1lZCB0cmVlIHNlY3Rpb24gZm9yY2VkIGNsb3NlZCBvbiBzdGVwIGVudHJ5LCB0aGVuXG4gICAqIGNvbXBsZXRlIGluZGl2aWR1YWxseSBhcyB0aGUgdXNlciBvcGVucyBlYWNoIG9uZS4gKi9cbiAgY29uc3QgbGFzdEludHJvZHVjdGlvbkV4cGFuZFN0ZXBJZFJlZiA9IHVzZVJlZjxzdHJpbmcgfCBudWxsPihudWxsKTtcbiAgdXNlRWZmZWN0KCgpID0+IHtcbiAgICBjb25zdCBleHBhbmRJbnRlcmFjdGlvbnMgPSAoYWN0aXZlSW50cm9kdWN0aW9uU3RlcD8uaW50ZXJhY3Rpb25zID8/IFtdKS5maWx0ZXIoKGludGVyYWN0aW9uKSA9PiBpbnRlcmFjdGlvbi5vbi5raW5kID09PSBcImV4cGFuZFwiKTtcbiAgICBpZiAoIWFjdGl2ZUludHJvZHVjdGlvblN0ZXAgfHwgZXhwYW5kSW50ZXJhY3Rpb25zLmxlbmd0aCA9PT0gMCkge1xuICAgICAgbGFzdEludHJvZHVjdGlvbkV4cGFuZFN0ZXBJZFJlZi5jdXJyZW50ID0gbnVsbDtcbiAgICAgIHJldHVybjtcbiAgICB9XG4gICAgaWYgKGxhc3RJbnRyb2R1Y3Rpb25FeHBhbmRTdGVwSWRSZWYuY3VycmVudCAhPT0gYWN0aXZlSW50cm9kdWN0aW9uU3RlcC5pZCkge1xuICAgICAgbGFzdEludHJvZHVjdGlvbkV4cGFuZFN0ZXBJZFJlZi5jdXJyZW50ID0gYWN0aXZlSW50cm9kdWN0aW9uU3RlcC5pZDtcbiAgICAgIGZvciAoY29uc3QgaW50ZXJhY3Rpb24gb2YgZXhwYW5kSW50ZXJhY3Rpb25zKSB7XG4gICAgICAgIGNvbnN0IHN0YXRlU3VmZml4ID0gYHRyZWUtc2VjdGlvbi0ke2ludGVyYWN0aW9uLm9uLmlkfWA7XG4gICAgICAgIGNvbnN0IGNhdGFsb2d1ZUtleSA9IGAke0ZSQU1FV09SS19QQU5FTF9UQUJfQ0FUQUxPR1VFX0lEfS50cmVlOiR7c3RhdGVTdWZmaXh9YDtcbiAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9UUkVFX09QRU5fU1RBVEVcIiwgaWQ6IGNhdGFsb2d1ZUtleSwgb3BlbjogZmFsc2UgfSk7XG4gICAgICB9XG4gICAgICByZXR1cm47XG4gICAgfVxuICAgIGZvciAoY29uc3QgaW50ZXJhY3Rpb24gb2YgZXhwYW5kSW50ZXJhY3Rpb25zKSB7XG4gICAgICBjb25zdCBzZWN0aW9uSWQgPSBpbnRlcmFjdGlvbi5vbi5pZDtcbiAgICAgIGNvbnN0IHN0YXRlU3VmZml4ID0gYHRyZWUtc2VjdGlvbi0ke3NlY3Rpb25JZH1gO1xuICAgICAgY29uc3QgZXhwYW5kZWQgPSBPYmplY3QuZW50cmllcyh0cmVlT3BlblN0YXRlcykuc29tZSgoW2tleSwgb3Blbl0pID0+IG9wZW4gJiYga2V5LmVuZHNXaXRoKHN0YXRlU3VmZml4KSk7XG4gICAgICBpZiAoZXhwYW5kZWQpIGNvbXBsZXRlSW50cm9kdWN0aW9uSW50ZXJhY3Rpb24oKGNhbmRpZGF0ZSkgPT4gY2FuZGlkYXRlLm9uLmtpbmQgPT09IFwiZXhwYW5kXCIgJiYgY2FuZGlkYXRlLm9uLmlkID09PSBzZWN0aW9uSWQpO1xuICAgIH1cbiAgfSwgW2FjdGl2ZUludHJvZHVjdGlvblN0ZXAsIGNvbXBsZXRlSW50cm9kdWN0aW9uSW50ZXJhY3Rpb24sIHRyZWVPcGVuU3RhdGVzXSk7XG5cbiAgLyoqIPCfp63vuI8gUHJvZ3Jlc3NpdmUgcmV2ZWFsIG1lYW5zIGEgc3RvcmVkIHBhdGggY2FuIGxlZ2l0aW1hdGVseSBlbmQgYXQgYSBicmFuY2ggKG9yIGJlIGVtcHR5KSDigJQgdGhpcyBpcyBub3cgYSBwbGFpbiBwZXItYW5jaG9yIHRydW5jYXRpb24tdmFsaWRhdGUsIG5vIG92ZXJyaWRlIHJlYXNzZXJ0aW9uIChzZWUgdGhlIHdyaXRlLXRocm91Z2ggZWZmZWN0cyBiZWxvdykuICovXG4gIGNvbnN0IHBhbmVsQWN0aXZlUGF0aHMgPSB1c2VNZW1vKCgpOiBSZWNvcmQ8QW5jaG9yLCByZWFkb25seSBzdHJpbmdbXT4gPT4ge1xuICAgIGNvbnN0IHJlc3VsdCA9IHt9IGFzIFJlY29yZDxBbmNob3IsIHJlYWRvbmx5IHN0cmluZ1tdPjtcbiAgICBmb3IgKGNvbnN0IGFuY2hvciBvZiBBTkNIT1JTKSByZXN1bHRbYW5jaG9yXSA9IHJlY29uY2lsZUFjdGl2ZVBhdGgoZG9jay5hbmNob3JzW2FuY2hvcl0sIHBhbmVsc1thbmNob3JdLnBhdGgsIHBhbmVsVGFiQ2hpbGRyZW4pO1xuICAgIHJldHVybiByZXN1bHQ7XG4gIH0sIFtwYW5lbHMsIGRvY2tdKTtcblxuICAvKipcbiAgICog8J+nre+4jyBHZW5lcmFsaXplcyB0aGUgb2xkIGBsZWZ0UGFuZWxBY3RpdmVQYXRoYC9gcmlnaHRQYW5lbEFjdGl2ZVBhdGhgIHN0dWRpby9wbHVnaW4gXCJzbmFwIHRvIHRoZSBhY3RpdmUgcGFuZWxcbiAgICogdGFiXCIgb3ZlcnJpZGVzIGFjcm9zcyBhbGwgZWlnaHQgYW5jaG9ycy4gV3JpdGUtdGhyb3VnaCByYXRoZXIgdGhhbiByZWFkLXRpbWU6IGVhY2ggb3ZlcnJpZGUgZGlzcGF0Y2hlc1xuICAgKiBgU0VUX1BBTkVMX1BBVEhgIG9ubHkgd2hlbiBpdHMgdGFyZ2V0IHRhYiBpZCBhY3R1YWxseSBjaGFuZ2VzLCBzbyBhIHVzZXIncyBvd24gY29sbGFwc2UvbmF2aWdhdGlvblxuICAgKiBhZnRlcndhcmQgc3RpY2tzIGluc3RlYWQgb2YgYmVpbmcgcmVhc3NlcnRlZCBvbiBldmVyeSByZW5kZXIgKHByb2dyZXNzaXZlIHJldmVhbCBtYWRlIHJlYWQtdGltZSByZWFzc2VydGlvblxuICAgKiBmaWdodCB0aGUgdXNlcidzIG93biBjb2xsYXBzZXMpLiBTdHVkaW8gd2lucyBvdmVyIGRldGFpbHMgd2hlbiBib3RoIHdvdWxkIHRvdWNoIHRoZSBzYW1lIGFuY2hvci5cbiAgICoqL1xuICBjb25zdCBsYXN0U3R1ZGlvT3ZlcnJpZGVUYWJJZFJlZiA9IHVzZVJlZjxzdHJpbmcgfCB1bmRlZmluZWQ+KHVuZGVmaW5lZCk7XG4gIHVzZUVmZmVjdCgoKSA9PiB7XG4gICAgaWYgKCFzdHVkaW9PdmVycmlkZVRhYklkIHx8ICFzdHVkaW9PdmVycmlkZUFuY2hvcikge1xuICAgICAgbGFzdFN0dWRpb092ZXJyaWRlVGFiSWRSZWYuY3VycmVudCA9IHVuZGVmaW5lZDtcbiAgICAgIHJldHVybjtcbiAgICB9XG4gICAgaWYgKGxhc3RTdHVkaW9PdmVycmlkZVRhYklkUmVmLmN1cnJlbnQgPT09IHN0dWRpb092ZXJyaWRlVGFiSWQpIHJldHVybjtcbiAgICBsYXN0U3R1ZGlvT3ZlcnJpZGVUYWJJZFJlZi5jdXJyZW50ID0gc3R1ZGlvT3ZlcnJpZGVUYWJJZDtcbiAgICBpZiAobW9iaWxlKSB7XG4gICAgICBpZiAobW9iaWxlUGFuZWxQYXRoWzBdID09PSBGUkFNRVdPUktfQ0FURUdPUllfRElTUExBWV9JRCkgcmV0dXJuO1xuICAgICAgY29uc3QgcmVzb2x2ZWQgPSBmaW5kUGFuZWxUYWJQYXRoKG1vYmlsZVBhbmVsVGFicywgc3R1ZGlvT3ZlcnJpZGVUYWJJZCk7XG4gICAgICBpZiAocmVzb2x2ZWQpIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfTU9CSUxFX1BBTkVMX1BBVEhcIiwgdmFsdWU6IHJlc29sdmVkIH0pO1xuICAgICAgcmV0dXJuO1xuICAgIH1cbiAgICBpZiAocGFuZWxzW3N0dWRpb092ZXJyaWRlQW5jaG9yXS5wYXRoWzBdID09PSBGUkFNRVdPUktfQ0FURUdPUllfRElTUExBWV9JRCkgcmV0dXJuO1xuICAgIGNvbnN0IHJlc29sdmVkID0gZmluZFBhbmVsVGFiUGF0aChkb2NrLmFuY2hvcnNbc3R1ZGlvT3ZlcnJpZGVBbmNob3JdLCBzdHVkaW9PdmVycmlkZVRhYklkKTtcbiAgICBpZiAocmVzb2x2ZWQpIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfUEFORUxfUEFUSFwiLCBhbmNob3I6IHN0dWRpb092ZXJyaWRlQW5jaG9yLCB2YWx1ZTogcmVzb2x2ZWQgfSk7XG4gIH0sIFtzdHVkaW9PdmVycmlkZVRhYklkLCBzdHVkaW9PdmVycmlkZUFuY2hvciwgZG9jaywgcGFuZWxzLCBtb2JpbGUsIG1vYmlsZVBhbmVsVGFicywgbW9iaWxlUGFuZWxQYXRoXSk7XG5cbiAgY29uc3QgbGFzdERldGFpbHNPdmVycmlkZVRhYklkUmVmID0gdXNlUmVmPHN0cmluZyB8IHVuZGVmaW5lZD4odW5kZWZpbmVkKTtcbiAgdXNlRWZmZWN0KCgpID0+IHtcbiAgICBpZiAoIWRldGFpbHNPdmVycmlkZVRhYklkIHx8ICFkZXRhaWxzT3ZlcnJpZGVBbmNob3IpIHtcbiAgICAgIGxhc3REZXRhaWxzT3ZlcnJpZGVUYWJJZFJlZi5jdXJyZW50ID0gdW5kZWZpbmVkO1xuICAgICAgcmV0dXJuO1xuICAgIH1cbiAgICBpZiAobGFzdERldGFpbHNPdmVycmlkZVRhYklkUmVmLmN1cnJlbnQgPT09IGRldGFpbHNPdmVycmlkZVRhYklkKSByZXR1cm47XG4gICAgbGFzdERldGFpbHNPdmVycmlkZVRhYklkUmVmLmN1cnJlbnQgPSBkZXRhaWxzT3ZlcnJpZGVUYWJJZDtcbiAgICBpZiAoZGV0YWlsc092ZXJyaWRlQW5jaG9yID09PSBzdHVkaW9PdmVycmlkZUFuY2hvcikgcmV0dXJuO1xuICAgIC8vIPCfp63vuI8gU2V0dGluZ3MgdGFicyByZW5kZXIgZmxhdCBub3cgKG5vIGNhdGVnb3J5IGJyYW5jaCB0byBjaGVjayBhZ2FpbnN0KSDigJQgc2tpcCB0aGUgb3ZlcnJpZGUgaWYgdGhlXG4gICAgLy8gYW5jaG9yJ3MgYWN0aXZlIGxlYWYgYWxyZWFkeSBiZWxvbmdzIHRvIFNldHRpbmdzLCBzbyBicm93c2luZyBUaGVtZS9TZXR0aW5ncyB0aGVyZSBkb2Vzbid0IGdldCBzdG9tcGVkLlxuICAgIGlmIChtb2JpbGUpIHtcbiAgICAgIGlmIChzZXR0aW5nc1JpZ2h0VGFicy5zb21lKCh0YWIpID0+IHRhYi5pZCA9PT0gbW9iaWxlUGFuZWxQYXRoWzBdKSkgcmV0dXJuO1xuICAgICAgY29uc3QgcmVzb2x2ZWQgPSBmaW5kUGFuZWxUYWJQYXRoKG1vYmlsZVBhbmVsVGFicywgZGV0YWlsc092ZXJyaWRlVGFiSWQpO1xuICAgICAgaWYgKHJlc29sdmVkKSBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX01PQklMRV9QQU5FTF9QQVRIXCIsIHZhbHVlOiByZXNvbHZlZCB9KTtcbiAgICAgIHJldHVybjtcbiAgICB9XG4gICAgaWYgKHNldHRpbmdzUmlnaHRUYWJzLnNvbWUoKHRhYikgPT4gdGFiLmlkID09PSBwYW5lbHNbZGV0YWlsc092ZXJyaWRlQW5jaG9yXS5wYXRoWzBdKSkgcmV0dXJuO1xuICAgIGNvbnN0IHJlc29sdmVkID0gZmluZFBhbmVsVGFiUGF0aChkb2NrLmFuY2hvcnNbZGV0YWlsc092ZXJyaWRlQW5jaG9yXSwgZGV0YWlsc092ZXJyaWRlVGFiSWQpO1xuICAgIGlmIChyZXNvbHZlZCkgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9QQU5FTF9QQVRIXCIsIGFuY2hvcjogZGV0YWlsc092ZXJyaWRlQW5jaG9yLCB2YWx1ZTogcmVzb2x2ZWQgfSk7XG4gIH0sIFtkZXRhaWxzT3ZlcnJpZGVUYWJJZCwgZGV0YWlsc092ZXJyaWRlQW5jaG9yLCBzdHVkaW9PdmVycmlkZUFuY2hvciwgZG9jaywgcGFuZWxzLCBzZXR0aW5nc1JpZ2h0VGFicywgbW9iaWxlLCBtb2JpbGVQYW5lbFRhYnMsIG1vYmlsZVBhbmVsUGF0aF0pO1xuICAvLyNlbmRyZWdpb24g8J+nre+4j0RvY2tBc3NlbWJseVxuXG4gIGNvbnN0IG1vYmlsZVBhbmVsID0gdXNlTWVtbygoKSA9PiB7XG4gICAgaWYgKG1vYmlsZVBhbmVsVGFicy5sZW5ndGggPT09IDApIHJldHVybiB1bmRlZmluZWQ7XG4gICAgcmV0dXJuIHtcbiAgICAgIHZpc2libGU6IG1vYmlsZVBhbmVsVmlzaWJsZSxcbiAgICAgIHRhYnM6IG1vYmlsZVBhbmVsVGFicyxcbiAgICAgIGFjdGl2ZVRhYlBhdGg6IG1vYmlsZVBhbmVsUGF0aCxcbiAgICAgIG9uQWN0aXZlVGFiUGF0aENoYW5nZTogKHBhdGg6IHJlYWRvbmx5IHN0cmluZ1tdKSA9PiB7XG4gICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfTU9CSUxFX1BBTkVMX1BBVEhcIiwgdmFsdWU6IHBhdGggfSk7XG4gICAgICAgIGNvbnN0IHRhYklkID0gcGF0aFtwYXRoLmxlbmd0aCAtIDFdO1xuICAgICAgICAvLyDwn4yx77iPIFByb2dyZXNzaXZlIHBhdGhzIG9mdGVuIGVuZCBhdCBhIGJyYW5jaCAob3IgYXJlIGVtcHR5KSDigJQgb25seSBsZWF2ZXMgYXJlIG1lYW5pbmdmdWwgXCJhY3RpdmUgcGFuZWwgdGFiXCIgc2VsZWN0aW9ucy5cbiAgICAgICAgaWYgKHRhYklkICYmIHN0dWRpb01vZGUgJiYgc2Vzc2lvbj8uYXBwLmlkID09PSBob3N0QXBwSWQgJiYgZmluZFBhbmVsVGFiTm9kZShtb2JpbGVQYW5lbFRhYnMsIHBhdGgpPy5raW5kID09PSBcImxlYWZcIikge1xuICAgICAgICAgIG9uQWN0aW9uKHsgY29udHJvbGxlcklkOiBzZXNzaW9uLmFwcC5jb250cm9sbGVySWQsIGFjdGlvbjogXCJzZXRBY3RpdmVQYW5lbFRhYlwiLCBhcmdzOiB7IHRhYklkIH0gfSk7XG4gICAgICAgIH1cbiAgICAgIH0sXG4gICAgICBwYXRoTWVtb3J5OiBwYW5lbFBhdGhNZW1vcnksXG4gICAgICBvblBhdGhNZW1vcnlDaGFuZ2U6ICh2YWx1ZTogUmVhZG9ubHk8UmVjb3JkPHN0cmluZywgc3RyaW5nPj4pID0+IGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfUEFORUxfUEFUSF9NRU1PUllcIiwgdmFsdWUgfSksXG4gICAgICB0cmVlT3BlblN0YXRlcyxcbiAgICAgIG9uVHJlZU9wZW5TdGF0ZUNoYW5nZTogKGlkOiBzdHJpbmcsIG9wZW46IGJvb2xlYW4pID0+IGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfVFJFRV9PUEVOX1NUQVRFXCIsIGlkLCBvcGVuIH0pLFxuICAgICAgLy8g4pm777iPIExhenkgdG9vbC9jb21tYW5kIHRyZWVzIHJlYWQgbWVhc3VyZXMgKyBhY3RpdmUgdG9vbCBmcm9tIHJlZnMg4oCUIHJldmlzaW9uIGZvcmNlcyByZS1yZXNvbHZlLlxuICAgICAgdHJlZUNvbnRlbnRSZXZpc2lvbjogeyBhY3RpdmVUb29sSWQsIHRvb2xNZWFzdXJlc0J5VG9vbElkLCBhY3Rpb25QYW5lU3RhZ2VkQXJnc0J5S2V5IH0sXG4gICAgfTtcbiAgfSwgW21vYmlsZVBhbmVsVmlzaWJsZSwgbW9iaWxlUGFuZWxQYXRoLCBtb2JpbGVQYW5lbFRhYnMsIG9uQWN0aW9uLCBwYW5lbFBhdGhNZW1vcnksIHNlc3Npb24sIHN0dWRpb01vZGUsIHRyZWVPcGVuU3RhdGVzLCBob3N0QXBwSWQsIGFjdGl2ZVRvb2xJZCwgdG9vbE1lYXN1cmVzQnlUb29sSWQsIGFjdGlvblBhbmVTdGFnZWRBcmdzQnlLZXldKTtcblxuICB1c2VFZmZlY3QoKCkgPT4ge1xuICAgIGlmIChleGFtcGxlT3B0aW9ucy5sZW5ndGggPT09IDApIHJldHVybjtcbiAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0FDVElWRV9FWEFNUExFX0lEXCIsIHZhbHVlOiAoY3VycmVudCkgPT4gKCFjdXJyZW50IHx8IGV4YW1wbGVPcHRpb25zLnNvbWUoKG9wdGlvbikgPT4gb3B0aW9uLmlkID09PSBjdXJyZW50KSA/IGN1cnJlbnQgOiBcIlwiKSB9KTtcbiAgfSwgW2V4YW1wbGVPcHRpb25zLCBzZXNzaW9uPy5hcHAuaWQsIHNlc3Npb24/LnBsdWdpbklkXSk7XG5cbiAgLy8g8J+Om++4jyBBbm5vdW5jZXMgdGhlIGJvb3QgZXhhbXBsZSB0byB0aGUgZnJlc2ggc2Vzc2lvbiBleGFjdGx5IG9uY2UgcGVyIGluc3RhbmNlLiBXaGVuIG5vdGhpbmcgaXNcbiAgLy8gbG9ja2VkL2RlZmF1bHRlZCwgc2VlZCB0aGUgZmlyc3QgcmVnaXN0ZXJlZCBleGFtcGxlIHNvIHRoZSBkcm9wZG93biBtYXRjaGVzIHRoZSBwbHVnaW4gZGVmYXVsdFxuICAvLyBkb2N1bWVudCAoZS5nLiBwcm9jZWR1cmFsM2QgaGV4YWdvbmFsIGNvbHVtbikg4oCUIHNhbWUgcnVsZSBhcyB3Z3B1IGBzeW5jX3Nlc3Npb25fY2hyb21lYC5cbiAgLy8gU3R1ZGlvLW1vZGUgcm91dGVzIGxvYWQgZG9jdW1lbnRzIHZpYSBgYXBwbHlTaGVsbFVyaWAvYG9wZW5TcGFjZWA7IG5ldmVyIGJvb3Qtb3ZlcnJpZGUgdGhvc2UuXG4gIHVzZUVmZmVjdCgoKSA9PiB7XG4gICAgaWYgKGV4YW1wbGVPcHRpb25zLmxlbmd0aCA9PT0gMCB8fCAhc2Vzc2lvbikgcmV0dXJuO1xuICAgIGlmIChzdHVkaW9Nb2RlKSB7XG4gICAgICBub0V4YW1wbGVSZXNldEluc3RhbmNlSWRSZWYuY3VycmVudCA9IHNlc3Npb24uaW5zdGFuY2VJZDtcbiAgICAgIHJldHVybjtcbiAgICB9XG4gICAgaWYgKG5vRXhhbXBsZVJlc2V0SW5zdGFuY2VJZFJlZi5jdXJyZW50ID09PSBzZXNzaW9uLmluc3RhbmNlSWQpIHJldHVybjtcbiAgICBub0V4YW1wbGVSZXNldEluc3RhbmNlSWRSZWYuY3VycmVudCA9IHNlc3Npb24uaW5zdGFuY2VJZDtcbiAgICBjb25zdCBleGFtcGxlSWQgPSByZXNvbHZlQm9vdEV4YW1wbGVJZChhY3RpdmVFeGFtcGxlSWQsIGV4YW1wbGVPcHRpb25zLCBkZWZhdWx0cy5leGFtcGxlSWQpO1xuICAgIGlmIChleGFtcGxlSWQgIT09IGFjdGl2ZUV4YW1wbGVJZCkge1xuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9BQ1RJVkVfRVhBTVBMRV9JRFwiLCB2YWx1ZTogZXhhbXBsZUlkIH0pO1xuICAgIH1cbiAgICBkaXNwYXRjaEFjdGl2ZUV4YW1wbGUoZXhhbXBsZUlkKTtcbiAgfSwgW2FjdGl2ZUV4YW1wbGVJZCwgZGVmYXVsdHMuZXhhbXBsZUlkLCBkaXNwYXRjaEFjdGl2ZUV4YW1wbGUsIGV4YW1wbGVPcHRpb25zLCBzZXNzaW9uLCBzdHVkaW9Nb2RlXSk7XG5cbiAgLy8jcmVnaW9uIPCfjpvvuI9QYW5lbFRhYkJhckhvc3Rpbmcg4oCUIGBidWlsZFBhbmVsU2VsZWN0aW9uUHJvcHNgIGlzIHRoZSBzaW5nbGUgc291cmNlIG9mIGFuIGFuY2hvcidzIHRhYlxuICAvLyBzZWxlY3Rpb24gc3RhdGUsIHNoYXJlZCBieSB0aGUgY2hyb21lLWhvc3RlZCBgUGFuZWxDaHJvbWVUYWJCYXJgIChiZWxvdywgZm9yIGFuY2hvcnMgaW5cbiAgLy8ge0BsaW5rIFBBTkVMX1RBQl9CQVJfSE9TVFN9KSBhbmQgdGhlIGZsb2F0aW5nIGBQYW5lbGAgaXRzZWxmIChgYnVpbGRQYW5lbFByb3BzYCkg4oCUIHRoZSB0d28gaG9zdHMgb2YgdGhlXG4gIC8vIFNBTUUgYW5jaG9yIGFsd2F5cyByZWFkL3dyaXRlIHRoZSBleGFjdCBzYW1lIGNvbnRyb2xsZWQgc3RhdGUuXG4gIGNvbnN0IGJ1aWxkUGFuZWxTZWxlY3Rpb25Qcm9wcyA9IHVzZUNhbGxiYWNrKFxuICAgIChhbmNob3I6IEFuY2hvcik6IFBhbmVsVGFiU2VsZWN0aW9uT3B0aW9ucyA9PiAoe1xuICAgICAgdGFiczogZG9jay5hbmNob3JzW2FuY2hvcl0sXG4gICAgICB2aXNpYmxlOiBwYW5lbHNbYW5jaG9yXS52aXNpYmxlLFxuICAgICAgb25WaXNpYmxlQ2hhbmdlOiAodmFsdWU6IGJvb2xlYW4pID0+IHtcbiAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9QQU5FTF9WSVNJQkxFXCIsIGFuY2hvciwgdmFsdWUgfSk7XG4gICAgICAgIG5vdGVTaGVsbENvbW1hbmQoXCJzaGVsbC5wYW5lbFRvZ2dsZVwiLCBzaGVsbExhYmVsKFwidWkuc2hlbGxDb21tYW5kLnBhbmVsVG9nZ2xlXCIpLCB7IGFuY2hvciwgdmlzaWJsZTogdmFsdWUgfSk7XG4gICAgICB9LFxuICAgICAgYWN0aXZlVGFiUGF0aDogcGFuZWxBY3RpdmVQYXRoc1thbmNob3JdLFxuICAgICAgb25BY3RpdmVUYWJQYXRoQ2hhbmdlOiAocGF0aDogcmVhZG9ubHkgc3RyaW5nW10pID0+IHtcbiAgICAgICAgY29uc3QgcGF0aENoYW5nZWQgPSAocGFuZWxBY3RpdmVQYXRoc1thbmNob3JdID8/IFtdKS5qb2luKFwiL1wiKSAhPT0gcGF0aC5qb2luKFwiL1wiKTtcbiAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9QQU5FTF9QQVRIXCIsIGFuY2hvciwgdmFsdWU6IHBhdGggfSk7XG4gICAgICAgIC8vIPCfjpvvuI8gQ29tbWFuZCBwYWxldHRlIG9ubHk6IHN3aXRjaGluZyBjYXRlZ29yeSBsZWF2ZXMgYWx3YXlzIGNvbGxhcHNlcyBhbnkgZXhwYW5kZWQgYXJnIGZvcm0g4oCUIHRoZVxuICAgICAgICAvLyBuZXh0IGhpZXJhcmNoeSBsZXZlbCB1cCBvbmx5IG1ha2VzIHNlbnNlIHVuZGVyIGl0cyBvd24gY2F0ZWdvcnkncyBjb21tYW5kIGxpc3QgKG1pcnJvcnMgdGhlIG9sZFxuICAgICAgICAvLyBkZWRpY2F0ZWQgYFNFVF9DT01NQU5EX0NBVEVHT1JZYCByZWR1Y2VyIGNhc2UsIG5vdyBleHByZXNzZWQgYXQgdGhlIGdlbmVyaWMgcGF0aC1jaGFuZ2UgY2FsbCBzaXRlXG4gICAgICAgIC8vIHNpbmNlIGNhdGVnb3J5LWFjdGl2ZSBzdGF0ZSBpdHNlbGYgaXMganVzdCB0aGlzIGFuY2hvcidzIGBhY3RpdmVUYWJQYXRoYCkuIENhdGVnb3JpZXMgc2l0IHVuZGVyXG4gICAgICAgIC8vIHRoZSBDb21tYW5kIGJyYW5jaCwgc28gY29tcGFyZSB0aGUgY2F0ZWdvcnkgc2VnbWVudCAocGF0aFsxXSksIG5vdCB0aGUgc2hhcmVkIGJyYW5jaCByb290LlxuICAgICAgICBpZiAoYW5jaG9yID09PSBcImJvdHRvbS1taWRkbGVcIiAmJiBwYW5lbHNbYW5jaG9yXS5wYXRoWzFdICE9PSBwYXRoWzFdKSB7XG4gICAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9DT01NQU5EX0VYUEFOREVEXCIsIHZhbHVlOiBudWxsIH0pO1xuICAgICAgICB9XG4gICAgICAgIGNvbnN0IHRhYklkID0gcGF0aFtwYXRoLmxlbmd0aCAtIDFdO1xuICAgICAgICAvLyDwn5ug77iPIFNlbGVjdGluZyBhIG1vZGUtdG9vbCBsZWFmIChgdG9vbC48aWQ+YCkgYWN0aXZhdGVzIHRoYXQgdG9vbCBzbyBpdHMgbWVhc3VyZXMgcmVuZGVyIGltbWVkaWF0ZWx5XG4gICAgICAgIC8vIHVuZGVyIHRoZSB0YWIg4oCUIG5vIG5lc3RlZCBGaWxsIHRvZ2dsZSBpbnNpZGUgdGhlIHRyZWUuXG4gICAgICAgIGlmIChhbmNob3IgPT09IFwiYm90dG9tLW1pZGRsZVwiICYmIHNlc3Npb24gJiYgZmluZFBhbmVsVGFiTm9kZShkb2NrLmFuY2hvcnNbYW5jaG9yXSwgcGF0aCk/LmtpbmQgPT09IFwibGVhZlwiKSB7XG4gICAgICAgICAgY29uc3Qgc2VsZWN0ZWRUb29sSWQgPSB0b29sSWRGcm9tUGFuZWxUYWJJZCh0YWJJZCk7XG4gICAgICAgICAgaWYgKHNlbGVjdGVkVG9vbElkICYmIHNlbGVjdGVkVG9vbElkICE9PSBhY3RpdmVUb29sSWRSZWYuY3VycmVudCkge1xuICAgICAgICAgICAgb25BY3Rpb24oeyBjb250cm9sbGVySWQ6IHNlc3Npb24uYXBwLmNvbnRyb2xsZXJJZCwgYWN0aW9uOiBTRVRfQUNUSVZFX1RPT0xfQUNUSU9OX0lELCBhcmdzOiB7IHRvb2xJZDogc2VsZWN0ZWRUb29sSWQgfSB9KTtcbiAgICAgICAgICB9XG4gICAgICAgIH1cbiAgICAgICAgLy8g8J+Mse+4jyBQcm9ncmVzc2l2ZSBwYXRocyBvZnRlbiBlbmQgYXQgYSBicmFuY2ggKG9yIGFyZSBlbXB0eSkg4oCUIG9ubHkgbGVhdmVzIGFyZSBtZWFuaW5nZnVsIFwiYWN0aXZlIHBhbmVsIHRhYlwiIHNlbGVjdGlvbnMuXG4gICAgICAgIGlmICh0YWJJZCAmJiBzdHVkaW9Nb2RlICYmIHNlc3Npb24/LmFwcC5pZCA9PT0gaG9zdEFwcElkICYmIGZpbmRQYW5lbFRhYk5vZGUoZG9jay5hbmNob3JzW2FuY2hvcl0sIHBhdGgpPy5raW5kID09PSBcImxlYWZcIikge1xuICAgICAgICAgIG9uQWN0aW9uKHsgY29udHJvbGxlcklkOiBzZXNzaW9uLmFwcC5jb250cm9sbGVySWQsIGFjdGlvbjogXCJzZXRBY3RpdmVQYW5lbFRhYlwiLCBhcmdzOiB7IHRhYklkIH0gfSk7XG4gICAgICAgIH1cbiAgICAgICAgaWYgKHBhdGhDaGFuZ2VkICYmIHRhYklkKSBub3RlU2hlbGxDb21tYW5kKFwic2hlbGwucGFuZWxUYWJcIiwgc2hlbGxMYWJlbChcInVpLnNoZWxsQ29tbWFuZC5wYW5lbFRhYlwiKSwgeyBhbmNob3IsIHRhYklkIH0pO1xuICAgICAgfSxcbiAgICAgIHBhdGhNZW1vcnk6IHBhbmVsUGF0aE1lbW9yeSxcbiAgICAgIG9uUGF0aE1lbW9yeUNoYW5nZTogKHZhbHVlOiBSZWFkb25seTxSZWNvcmQ8c3RyaW5nLCBzdHJpbmc+PikgPT4gZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9QQU5FTF9QQVRIX01FTU9SWVwiLCB2YWx1ZSB9KSxcbiAgICB9KSxcbiAgICBbZG9jaywgb25BY3Rpb24sIHBhbmVsQWN0aXZlUGF0aHMsIHBhbmVsUGF0aE1lbW9yeSwgcGFuZWxzLCBzZXNzaW9uLCBzdHVkaW9Nb2RlLCBob3N0QXBwSWQsIG5vdGVTaGVsbENvbW1hbmRdLFxuICApO1xuICAvLyNlbmRyZWdpb24g8J+Om++4j1BhbmVsVGFiQmFySG9zdGluZ1xuXG4gIGNvbnN0IG5hdmJhckl0ZW1zID0gdXNlTWVtbygoKTogTmF2YmFySXRlbVtdID0+IHtcbiAgICBpZiAoIXNlc3Npb24pIHJldHVybiBbXTtcbiAgICBjb25zdCBsb2dvQW5kVGl0bGUgPSAoXG4gICAgICA8ZGl2IGtleT1cImxvZ29BbmRUaXRsZVwiIGNsYXNzTmFtZT1cImZsZXggbWluLXctMCBzaHJpbmstMCBpdGVtcy1jZW50ZXIgZ2FwLXNpbmdsZVwiPlxuICAgICAgICB7YnJhbmQ/LmxvZ29TdmcgPyA8U2hlbGxCcmFuZExvZ28gc3ZnPXticmFuZC5sb2dvU3ZnfSBjbGFzc05hbWU9XCJzaXplLXdvcmtiZW5jaCBzaHJpbmstMFwiIC8+IDogPFNlbWlvTG9nbyBjbGFzc05hbWU9XCJzaXplLXdvcmtiZW5jaCBzaHJpbmstMFwiIC8+fVxuICAgICAgICA8c3BhbiBkYXRhLXNsb3Q9XCJhcHAtbmFtZVwiIGNsYXNzTmFtZT17Y24oXCJweC1zaW5nbGVcIiwgc2hlbGxDaHJvbWVUaXRsZUNsYXNzTmFtZSl9PlxuICAgICAgICAgIHthcHBEb2N1bWVudExhYmVsKHJlc29sdmVBcHBEb2N1bWVudChzZXNzaW9uLmFwcCwgdWlUZXJtaW5vbG9neSkpfVxuICAgICAgICA8L3NwYW4+XG4gICAgICA8L2Rpdj5cbiAgICApO1xuICAgIGNvbnN0IHNob3dFeGFtcGxlU2VsZWN0ID0gZXhhbXBsZU9wdGlvbnMubGVuZ3RoID4gMCAmJiAhbG9ja3MuZXhhbXBsZUlkICYmICghc3R1ZGlvTW9kZSB8fCBzZXNzaW9uLmFwcC5pZCAhPT0gbGFuZGluZ0FwcElkKTtcbiAgICAvLyDwn5Ox77iPIE1vYmlsZSBoYXMgbm8gcm9vbSBmb3IgdGFiIGJhcnMsIGV4YW1wbGUgc2VsZWN0b3IsIG9yIG1vZGUgc3dpdGNoZXIgaW4gdGhlIG5hdmJhciDigJQganVzdCB0aGVcbiAgICAvLyBsb2dvL3RpdGxlIGFuZCB0aGUgc2luZ2xlIHRvZ2dsZSBmb3IgdGhlIG1lcmdlZCBtb2JpbGUgcGFuZWwgKHRoZSB0d28gZHJvcHBlZCBjb250cm9scyByZXN1cmZhY2UgYXNcbiAgICAvLyB0aGUgcGFuZWwncyBzeW50aGV0aWMgXCJBcHBcIiB0YWIsIHNlZSBgbW9iaWxlUGFuZWxUYWJzYCkuXG4gICAgaWYgKG1vYmlsZSkge1xuICAgICAgcmV0dXJuIFtcbiAgICAgICAgeyBrZXk6IFwibG9nb0FuZFRpdGxlXCIsIGNvbnRlbnQ6IGxvZ29BbmRUaXRsZSB9LFxuICAgICAgICBuYXZiYXJGaWxsSXRlbShcIm5hdmJhclRyYWlsaW5nRmlsbFwiKSxcbiAgICAgICAge1xuICAgICAgICAgIGtleTogXCJtb2JpbGVQYW5lbFRvZ2dsZVwiLFxuICAgICAgICAgIGNvbnRlbnQ6IDxUb2dnbGUgaWQ9XCJ1aS5tb2JpbGVQYW5lbC50b2dnbGVcIiBwcmVzc2VkPXttb2JpbGVQYW5lbFZpc2libGV9IG9uUHJlc3NlZENoYW5nZT17KHZhbHVlKSA9PiBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX01PQklMRV9QQU5FTF9WSVNJQkxFXCIsIHZhbHVlIH0pfSBpY29uPVwicGFuZWwtbGVmdFwiIC8+LFxuICAgICAgICB9LFxuICAgICAgXTtcbiAgICB9XG4gICAgLy8gTG9nby90aXRsZSwgZXhhbXBsZSBzZWxlY3RvciwgYW5kIG1vZGUgc3dpdGNoZXIgcmVuZGVyIGFzIG9uZSBjbHVzdGVyLCBjZW50ZXJlZCBhcyBhIGdyb3VwIGluIHRoZSBuYXZiYXJcbiAgICAvLyAodmlhIGBjZW50ZXJlZGApIHJhdGhlciB0aGFuIGxlZnQtYW5jaG9yZWQgd2l0aCBmaWxsIHNwYWNlcnMgcHVzaGluZyB0aGUgcmVzdCB0b3dhcmQgdGhlIHRyYWlsaW5nIGVkZ2UuXG4gICAgY29uc3QgY2VudGVyQ29udGVudDogUmVhY3ROb2RlW10gPSBbbG9nb0FuZFRpdGxlXTtcbiAgICBpZiAoc2hvd0V4YW1wbGVTZWxlY3QgJiYgZXhhbXBsZVNlbGVjdEVsZW1lbnQpIGNlbnRlckNvbnRlbnQucHVzaChleGFtcGxlU2VsZWN0RWxlbWVudCk7XG4gICAgaWYgKG1vZGVTd2l0Y2hlckVsZW1lbnQpIGNlbnRlckNvbnRlbnQucHVzaChtb2RlU3dpdGNoZXJFbGVtZW50KTtcbiAgICByZXR1cm4gW1xuICAgICAgeyBrZXk6IFwidG9wTGVmdFBhbmVsVGFic1wiLCBjb250ZW50OiA8UGFuZWxDaHJvbWVUYWJCYXIgYW5jaG9yPVwidG9wLWxlZnRcIiB7Li4uYnVpbGRQYW5lbFNlbGVjdGlvblByb3BzKFwidG9wLWxlZnRcIil9IC8+IH0sXG4gICAgICBuYXZiYXJGaWxsSXRlbShcIm5hdmJhclRyYWlsaW5nRmlsbFwiKSxcbiAgICAgIHsga2V5OiBcInRvcFJpZ2h0UGFuZWxUYWJzXCIsIGNvbnRlbnQ6IDxQYW5lbENocm9tZVRhYkJhciBhbmNob3I9XCJ0b3AtcmlnaHRcIiB7Li4uYnVpbGRQYW5lbFNlbGVjdGlvblByb3BzKFwidG9wLXJpZ2h0XCIpfSAvPiB9LFxuICAgICAge1xuICAgICAgICBrZXk6IFwiY2VudGVyXCIsXG4gICAgICAgIGNlbnRlcmVkOiB0cnVlLFxuICAgICAgICBjb250ZW50OiAoXG4gICAgICAgICAgPGRpdiBjbGFzc05hbWU9XCJmbGV4IG1pbi13LTAgaXRlbXMtY2VudGVyIGdhcC1kb3VibGVcIj5cbiAgICAgICAgICAgIHtjZW50ZXJDb250ZW50fVxuICAgICAgICAgICAgPFBhbmVsQ2hyb21lVGFiQmFyIGFuY2hvcj1cInRvcC1taWRkbGVcIiB7Li4uYnVpbGRQYW5lbFNlbGVjdGlvblByb3BzKFwidG9wLW1pZGRsZVwiKX0gLz5cbiAgICAgICAgICA8L2Rpdj5cbiAgICAgICAgKSxcbiAgICAgIH0sXG4gICAgXTtcbiAgfSwgW2JyYW5kLCBidWlsZFBhbmVsU2VsZWN0aW9uUHJvcHMsIGV4YW1wbGVPcHRpb25zLCBleGFtcGxlU2VsZWN0RWxlbWVudCwgbG9ja3MuZXhhbXBsZUlkLCBtb2JpbGUsIG1vYmlsZVBhbmVsVmlzaWJsZSwgbW9kZVN3aXRjaGVyRWxlbWVudCwgc2Vzc2lvbiwgdWlUZXJtaW5vbG9neSwgc3R1ZGlvTW9kZSwgbGFuZGluZ0FwcElkXSk7XG5cbiAgY29uc3Qgc2VhcmNoSXRlbXMgPSB1c2VNZW1vKCgpID0+IHtcbiAgICBpZiAoIXNlc3Npb24pIHJldHVybiBbXTtcbiAgICBjb25zdCBpdGVtczogVUlTZWFyY2hJdGVtW10gPSBbXTtcbiAgICBmb3IgKGNvbnN0IHRhYiBvZiBmbGF0dGVuUGFuZWxUYWJMZWF2ZXMoc2Vzc2lvbi5hcHAucGFuZWxUYWJzKSkge1xuICAgICAgY29uc3QgdGFiSWQgPSBwYW5lbFRhYktpbmRJZCh0YWIua2luZCk7XG4gICAgICBpdGVtcy5wdXNoKHtcbiAgICAgICAgaWQ6IGBwYW5lbC4ke3RhYklkfWAsXG4gICAgICAgIGxhYmVsOiByZXNvbHZlUGFuZWxUYWJMYWJlbChhcHBMYWJlbHNPdmVybGF5LCB0YWJJZCwgcmVzb2x2ZU1hbmlmZXN0TGFiZWwodGFiLmxhYmVsLCB1aVRlcm1pbm9sb2d5LCB1aUxvY2FsZSkpLFxuICAgICAgICBjYXRlZ29yeTogc2hlbGxMYWJlbChcInVpLnNlYXJjaC5jYXRlZ29yeS5wYW5lbHNcIiksXG4gICAgICAgIGljb246IDxJY29uIGljb249XCJwYW5lbC1sZWZ0XCIgc2l6ZT1cInNtYWxsXCIgLz4sXG4gICAgICAgIG9uU2VsZWN0OiAoKSA9PiBvbkFjdGlvbih7IGNvbnRyb2xsZXJJZDogc2Vzc2lvbi5hcHAuY29udHJvbGxlcklkLCBhY3Rpb246IFwic2V0QWN0aXZlUGFuZWxUYWJcIiwgYXJnczogeyB0YWJJZCB9IH0pLFxuICAgICAgfSk7XG4gICAgfVxuICAgIGZvciAoY29uc3Qga2luZCBvZiBzZXNzaW9uLmFwcC53aW5kb3dLaW5kcykge1xuICAgICAgaXRlbXMucHVzaCh7XG4gICAgICAgIGlkOiBgd2luZG93LiR7a2luZC5pZH1gLFxuICAgICAgICBsYWJlbDogcmVzb2x2ZUFwcExhYmVsKGFwcExhYmVsc092ZXJsYXksIFwid2luZG93S2luZFwiLCBraW5kLmlkLCByZXNvbHZlTWFuaWZlc3RMYWJlbChraW5kLmxhYmVsLCB1aVRlcm1pbm9sb2d5LCB1aUxvY2FsZSkpLFxuICAgICAgICBjYXRlZ29yeTogc2hlbGxMYWJlbChcInVpLnNlYXJjaC5jYXRlZ29yeS53aW5kb3dzXCIpLFxuICAgICAgICBpY29uOiA8SWNvbiBpY29uPVwiYXBwLXdpbmRvd1wiIHNpemU9XCJzbWFsbFwiIC8+LFxuICAgICAgICBvblNlbGVjdDogKCkgPT4gZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9BQ1RJVkVfV0lORE9XX0lEXCIsIHZhbHVlOiBraW5kLmlkIH0pLFxuICAgICAgfSk7XG4gICAgfVxuICAgIGNvbnN0IGtleXNCeUFjdGlvbklkID0gbmV3IE1hcChzZXNzaW9uLmFwcC5rZXliaW5kaW5ncy5tYXAoKGJpbmRpbmcpID0+IFtiaW5kaW5nLmFjdGlvbi5hY3Rpb24sIGJpbmRpbmcua2V5c10pKTtcbiAgICBjb25zdCBkZWNsYXJlZEFjdGlvbklkcyA9IG5ldyBTZXQ8c3RyaW5nPigpO1xuICAgIC8vIPCfk4fvuI8gRmlyc3Qgd2luZG93IGtpbmQgd2hvc2UgcmVzb2x2ZWQgYWN0aW9ucyBpbmNsdWRlIHRoaXMgaWQgKG9ycGhhbi9nbG9iYWwgYWN0aW9ucyBmYWxsIHRocm91Z2ggdG9cbiAgICAvLyB0aGUgYWN0aXZlIHdpbmRvdywgdGhlbiB0aGUgZmlyc3Qgd2luZG93KSDigJQgdGhlIHJlZGlyZWN0IHRhcmdldCBmb3IgYXJnLWNhcnJ5aW5nIHBhbGV0dGUgZW50cmllcy5cbiAgICBjb25zdCBob3N0V2luZG93Rm9yQWN0aW9uID0gKGFjdGlvbklkOiBzdHJpbmcpOiBzdHJpbmcgfCB1bmRlZmluZWQgPT4ge1xuICAgICAgZm9yIChjb25zdCBraW5kIG9mIHNlc3Npb24uYXBwLndpbmRvd0tpbmRzKSB7XG4gICAgICAgIGlmIChyZXNvbHZlV2luZG93QWN0aW9ucyhzZXNzaW9uLmFwcCwga2luZCkuc29tZSgoZW50cnkpID0+IGVudHJ5LmlkID09PSBhY3Rpb25JZCkpIHJldHVybiBraW5kLmlkO1xuICAgICAgfVxuICAgICAgcmV0dXJuIGFjdGl2ZVdpbmRvd0lkID8/IHNlc3Npb24uYXBwLndpbmRvd0tpbmRzWzBdPy5pZDtcbiAgICB9O1xuICAgIGZvciAoY29uc3QgYWN0aW9uIG9mIHNlc3Npb24uYXBwLmFjdGlvbnMgPz8gW10pIHtcbiAgICAgIGlmICghYWN0aW9uLmluUGFsZXR0ZSkgY29udGludWU7XG4gICAgICBkZWNsYXJlZEFjdGlvbklkcy5hZGQoYWN0aW9uLmlkKTtcbiAgICAgIGNvbnN0IGFyZ0NhcnJ5aW5nID0gYWN0aW9uUmVxdWlyZXNTdGFnZWRGb3JtKGFjdGlvbik7XG4gICAgICBjb25zdCByZXNvbHZlZEFjdGlvbkxhYmVsID0gcmVzb2x2ZUFwcExhYmVsKGFwcExhYmVsc092ZXJsYXksIFwiYWN0aW9uXCIsIGFjdGlvbi5pZCwgcmVzb2x2ZU1hbmlmZXN0TGFiZWwoYWN0aW9uLmxhYmVsLCB1aVRlcm1pbm9sb2d5LCB1aUxvY2FsZSkpO1xuICAgICAgaXRlbXMucHVzaCh7XG4gICAgICAgIGlkOiBgYWN0aW9uLiR7YWN0aW9uLmlkfWAsXG4gICAgICAgIC8vIOKcje+4jyBBcmctY2FycnlpbmcgYWN0aW9ucyBuZXZlciBmaXJlIGZyb20gdGhlIHBhbGV0dGUgKFAzKTogdGhlIFwi4oCmXCIgZW50cnkgYWN0aXZhdGVzIHRoZSBob3N0aW5nXG4gICAgICAgIC8vIHdpbmRvdywgdW5mb2xkcyBpdHMgdG9wLWxlZnQgQWN0aW9ucyBwYW5lLCBhbmQgZXhwYW5kcyB0aGlzIGFjdGlvbidzIHN0YWdlZCBmb3JtIGluc3RlYWQgb2YgZGlzcGF0Y2hpbmcuXG4gICAgICAgIGxhYmVsOiBhcmdDYXJyeWluZyA/IGAke3Jlc29sdmVkQWN0aW9uTGFiZWx94oCmYCA6IHJlc29sdmVkQWN0aW9uTGFiZWwsXG4gICAgICAgIGRlc2NyaXB0aW9uOiBhY3Rpb24ua2V5cyA/PyBrZXlzQnlBY3Rpb25JZC5nZXQoYWN0aW9uLmlkKSxcbiAgICAgICAgY2F0ZWdvcnk6IGFjdGlvbi5jYXRlZ29yeSA/PyAoYWN0aW9uLmtpbmQgPT09IFwiaGlzdG9yeVwiID8gc2hlbGxMYWJlbChcInVpLnJpYmJvbi5wYXJlbnQuaGlzdG9yeVwiKSA6IHNoZWxsTGFiZWwoXCJ1aS5yaWJib24ucGFyZW50LmFjdGlvbnNcIikpLFxuICAgICAgICBvblNlbGVjdDogKCkgPT4ge1xuICAgICAgICAgIGlmIChhcmdDYXJyeWluZykge1xuICAgICAgICAgICAgY29uc3Qgd2luZG93SWQgPSBob3N0V2luZG93Rm9yQWN0aW9uKGFjdGlvbi5pZCk7XG4gICAgICAgICAgICBpZiAod2luZG93SWQpIHtcbiAgICAgICAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9BQ1RJVkVfV0lORE9XX0lEXCIsIHZhbHVlOiB3aW5kb3dJZCB9KTtcbiAgICAgICAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9BQ1RJT05fUEFORV9GT0xERURcIiwgd2luZG93SWQsIHZhbHVlOiBmYWxzZSB9KTtcbiAgICAgICAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9BQ1RJT05fUEFORV9FWFBBTkRFRFwiLCB3aW5kb3dJZCwgdmFsdWU6IGFjdGlvbi5pZCB9KTtcbiAgICAgICAgICAgIH1cbiAgICAgICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfU0VBUkNIX09QRU5cIiwgdmFsdWU6IGZhbHNlIH0pO1xuICAgICAgICAgICAgcmV0dXJuO1xuICAgICAgICAgIH1cbiAgICAgICAgICBvbkFjdGlvbih7IGNvbnRyb2xsZXJJZDogc2Vzc2lvbi5hcHAuY29udHJvbGxlcklkLCBhY3Rpb246IGFjdGlvbi5pZCB9KTtcbiAgICAgICAgfSxcbiAgICAgIH0pO1xuICAgIH1cbiAgICBmb3IgKGNvbnN0IGJpbmRpbmcgb2Ygc2Vzc2lvbi5hcHAua2V5YmluZGluZ3MpIHtcbiAgICAgIGlmIChkZWNsYXJlZEFjdGlvbklkcy5oYXMoYmluZGluZy5hY3Rpb24uYWN0aW9uKSkgY29udGludWU7XG4gICAgICBpdGVtcy5wdXNoKHtcbiAgICAgICAgaWQ6IGBrZXliaW5kaW5nLiR7YmluZGluZy5rZXlzfWAsXG4gICAgICAgIGxhYmVsOiBiaW5kaW5nLmFjdGlvbi5hY3Rpb24sXG4gICAgICAgIGRlc2NyaXB0aW9uOiBiaW5kaW5nLmtleXMsXG4gICAgICAgIGNhdGVnb3J5OiBzaGVsbExhYmVsKFwidWkucmliYm9uLnBhcmVudC5hY3Rpb25zXCIpLFxuICAgICAgICBvblNlbGVjdDogKCkgPT4gb25BY3Rpb24oYmluZGluZy5hY3Rpb24pLFxuICAgICAgfSk7XG4gICAgfVxuICAgIC8vIPCfjpvvuI8gQ29tbWFuZHMgKG9zL3BsdWdpbi9hcHAvbW9kZSkg4oCUIHRoZSBmb290ZXIgdHdpbiBvZiB0aGUgd2luZG93LXJhaWwgUDMgcmVkaXJlY3QgYWJvdmU6IGFuXG4gICAgLy8gYXJnLWNhcnJ5aW5nIGNvbW1hbmQgbmV2ZXIgZmlyZXMgZnJvbSB0aGUgcGFsZXR0ZSwgaXQgb3BlbnMgdGhlIGJvdHRvbS1taWRkbGUgY29tbWFuZCBwYW5lbCBhdCBpdHNcbiAgICAvLyBjYXRlZ29yeSBhbmQgZXhwYW5kcyBpdHMgZm9ybSBpbnN0ZWFkLlxuICAgIGZvciAoY29uc3QgeyBkZWZpbml0aW9uLCBzb3VyY2UgfSBvZiByZXNvbHZlZENvbW1hbmRzKSB7XG4gICAgICBpZiAoIWRlZmluaXRpb24uaW5QYWxldHRlKSBjb250aW51ZTtcbiAgICAgIGNvbnN0IGFyZ0NhcnJ5aW5nID0gKGRlZmluaXRpb24uYXJncz8ubGVuZ3RoID8/IDApID4gMDtcbiAgICAgIGl0ZW1zLnB1c2goe1xuICAgICAgICBpZDogYGNvbW1hbmQuJHtkZWZpbml0aW9uLmlkfWAsXG4gICAgICAgIGxhYmVsOiBhcmdDYXJyeWluZyA/IGAke2RlZmluaXRpb24ubGFiZWx94oCmYCA6IGRlZmluaXRpb24ubGFiZWwsXG4gICAgICAgIGRlc2NyaXB0aW9uOiBkZWZpbml0aW9uLmtleXMsXG4gICAgICAgIGNhdGVnb3J5OiBjb21tYW5kQ2F0ZWdvcnlMYWJlbChkZWZpbml0aW9uLmNhdGVnb3J5KSxcbiAgICAgICAgb25TZWxlY3Q6ICgpID0+IHtcbiAgICAgICAgICBpZiAoYXJnQ2FycnlpbmcpIHtcbiAgICAgICAgICAgIGNvbnN0IGNvbW1hbmRQYXRoID0gW0ZSQU1FV09SS19DQVRFR09SWV9DT01NQU5EX0lELCBgY29tbWFuZC5jYXRlZ29yeS4ke2RlZmluaXRpb24uY2F0ZWdvcnl9YF07XG4gICAgICAgICAgICAvLyDwn5Ox77iPIE9uIG1vYmlsZSBldmVyeSBhbmNob3IncyB0YWJzIGFyZSBtZXJnZWQgaW50byB0aGUgc2luZ2xlIG1vYmlsZSBwYW5lbCDigJQgcm91dGUgdGhlIHNhbWVcbiAgICAgICAgICAgIC8vIHBhdGggdGhlcmUgaW5zdGVhZCBvZiB0aGUgKHVucmVuZGVyZWQpIGJvdHRvbS1taWRkbGUgYW5jaG9yLCBhbmQgb3BlbiB0aGUgbW9iaWxlIHBhbmVsIGl0c2VsZi5cbiAgICAgICAgICAgIGlmIChtb2JpbGUpIHtcbiAgICAgICAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9NT0JJTEVfUEFORUxfVklTSUJMRVwiLCB2YWx1ZTogdHJ1ZSB9KTtcbiAgICAgICAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9NT0JJTEVfUEFORUxfUEFUSFwiLCB2YWx1ZTogY29tbWFuZFBhdGggfSk7XG4gICAgICAgICAgICB9IGVsc2Uge1xuICAgICAgICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1BBTkVMX1ZJU0lCTEVcIiwgYW5jaG9yOiBcImJvdHRvbS1taWRkbGVcIiwgdmFsdWU6IHRydWUgfSk7XG4gICAgICAgICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfUEFORUxfUEFUSFwiLCBhbmNob3I6IFwiYm90dG9tLW1pZGRsZVwiLCB2YWx1ZTogY29tbWFuZFBhdGggfSk7XG4gICAgICAgICAgICB9XG4gICAgICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0NPTU1BTkRfRVhQQU5ERURcIiwgdmFsdWU6IGRlZmluaXRpb24uaWQgfSk7XG4gICAgICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1NFQVJDSF9PUEVOXCIsIHZhbHVlOiBmYWxzZSB9KTtcbiAgICAgICAgICAgIHJldHVybjtcbiAgICAgICAgICB9XG4gICAgICAgICAgb25Db21tYW5kKHNvdXJjZSwgZGVmaW5pdGlvbi5pZCk7XG4gICAgICAgIH0sXG4gICAgICB9KTtcbiAgICB9XG4gICAgaWYgKHN0dWRpb01vZGUgJiYgcGFuZWwpIHtcbiAgICAgIGZvciAoY29uc3QgcHJvZ3JhbSBvZiBwYW5lbC5wcm9ncmFtcykge1xuICAgICAgICBpdGVtcy5wdXNoKHtcbiAgICAgICAgICBpZDogYHNwYXduLiR7cHJvZ3JhbS5wbHVnaW5JZH1gLFxuICAgICAgICAgIGxhYmVsOiBgJHtzaGVsbExhYmVsKFwidWkucGFsZXR0ZS5zcGF3blByZWZpeFwiKX0gJHthcHBEb2N1bWVudExhYmVsKHJlc29sdmVEb2N1bWVudEJ5QXBwSWQobG9hZGVkUGx1Z2lucywgcHJvZ3JhbS5hcHBJZCwgcHJvZ3JhbS5kb2N1bWVudCwgdWlUZXJtaW5vbG9neSkpfWAsXG4gICAgICAgICAgY2F0ZWdvcnk6IHNoZWxsTGFiZWwoXCJ1aS5zZWFyY2guY2F0ZWdvcnkuY2F0YWxvZ3VlXCIpLFxuICAgICAgICAgIG9uU2VsZWN0OiAoKSA9PiBvbkFjdGlvbih7IGNvbnRyb2xsZXJJZDogaG9zdENvbnRyb2xsZXJJZCA/PyBcIlwiLCBhY3Rpb246IFwic3Bhd25BcHBcIiwgYXJnczogeyBwbHVnaW5JZDogcHJvZ3JhbS5wbHVnaW5JZCB9IH0pLFxuICAgICAgICB9KTtcbiAgICAgIH1cbiAgICAgIGl0ZW1zLnB1c2goXG4gICAgICAgIHtcbiAgICAgICAgICBpZDogXCJzdHVkaW8udW5kb1wiLFxuICAgICAgICAgIGxhYmVsOiBzaGVsbExhYmVsKFwidWkucGFsZXR0ZS51bmRvXCIpLFxuICAgICAgICAgIGNhdGVnb3J5OiBzaGVsbExhYmVsKFwidWkuc2VhcmNoLmNhdGVnb3J5LnN0dWRpb1wiKSxcbiAgICAgICAgICBpY29uOiA8SWNvbiBpY29uPVwidW5kby0yXCIgc2l6ZT1cInNtYWxsXCIgLz4sXG4gICAgICAgICAgb25TZWxlY3Q6ICgpID0+IG9uQWN0aW9uKHsgY29udHJvbGxlcklkOiBob3N0Q29udHJvbGxlcklkID8/IFwiXCIsIGFjdGlvbjogXCJ1bmRvXCIgfSksXG4gICAgICAgIH0sXG4gICAgICAgIHtcbiAgICAgICAgICBpZDogXCJzdHVkaW8ucmVkb1wiLFxuICAgICAgICAgIGxhYmVsOiBzaGVsbExhYmVsKFwidWkucGFsZXR0ZS5yZWRvXCIpLFxuICAgICAgICAgIGNhdGVnb3J5OiBzaGVsbExhYmVsKFwidWkuc2VhcmNoLmNhdGVnb3J5LnN0dWRpb1wiKSxcbiAgICAgICAgICBpY29uOiA8SWNvbiBpY29uPVwicmVkby0yXCIgc2l6ZT1cInNtYWxsXCIgLz4sXG4gICAgICAgICAgb25TZWxlY3Q6ICgpID0+IG9uQWN0aW9uKHsgY29udHJvbGxlcklkOiBob3N0Q29udHJvbGxlcklkID8/IFwiXCIsIGFjdGlvbjogXCJyZWRvXCIgfSksXG4gICAgICAgIH0sXG4gICAgICAgIHtcbiAgICAgICAgICBpZDogXCJzdHVkaW8uaG9tZVwiLFxuICAgICAgICAgIGxhYmVsOiBzaGVsbExhYmVsKFwidWkucGFsZXR0ZS5nb0hvbWVcIiksXG4gICAgICAgICAgY2F0ZWdvcnk6IHNoZWxsTGFiZWwoXCJ1aS5zZWFyY2guY2F0ZWdvcnkubmF2aWdhdGlvblwiKSxcbiAgICAgICAgICBvblNlbGVjdDogKCkgPT4gb25BY3Rpb24oeyBjb250cm9sbGVySWQ6IGhvc3RDb250cm9sbGVySWQgPz8gXCJcIiwgYWN0aW9uOiBcImdvSG9tZVwiIH0pLFxuICAgICAgICB9LFxuICAgICAgKTtcbiAgICB9XG4gICAgcmV0dXJuIGl0ZW1zO1xuICB9LCBbYWN0aXZlV2luZG93SWQsIGFwcExhYmVsc092ZXJsYXksIGxvYWRlZFBsdWdpbnMsIG1vYmlsZSwgb25BY3Rpb24sIG9uQ29tbWFuZCwgcGFuZWwsIHJlc29sdmVkQ29tbWFuZHMsIHNlc3Npb24sIHN0dWRpb01vZGUsIHVpTG9jYWxlLCB1aVRlcm1pbm9sb2d5LCBob3N0Q29udHJvbGxlcklkXSk7XG5cbiAgY29uc3QgbW9kZVdpbmRvd3MgPSB1c2VNZW1vKCgpOiBNb2RlV2luZG93RGVzY3JpcHRvcltdID0+IHtcbiAgICBpZiAoIXNlc3Npb24pIHJldHVybiBbXTtcbiAgICBjb25zdCBhY3Rpb25QYW5lU2xpY2U6IEFjdGlvblBhbmVTbGljZSA9IHsgZXhwYW5kZWRCeVdpbmRvd0lkOiBhY3Rpb25QYW5lRXhwYW5kZWRCeVdpbmRvd0lkLCBzdGFnZWRBcmdzQnlLZXk6IGFjdGlvblBhbmVTdGFnZWRBcmdzQnlLZXksIGFjdGl2ZVV0aWxpdHlCeVdpbmRvd0lkIH07XG4gICAgY29uc3QgYWN0aW9uc0ZvbGRlZEZvciA9ICh3aW5kb3dJZDogc3RyaW5nLCB3aW5kb3dLaW5kSWQ6IHN0cmluZyA9IHdpbmRvd0lkKSA9PlxuICAgICAgaW50cm9kdWN0aW9uVGFyZ2V0c1dpbmRvdyh3aW5kb3dJZCwgd2luZG93S2luZElkLCBudWxsLCBpbnRyb2R1Y3Rpb25BY3Rpb25XaW5kb3dTZWdtZW50KSA/IGZhbHNlIDogKGFjdGlvblBhbmVGb2xkZWRCeVdpbmRvd0lkW3dpbmRvd0lkXSA/PyB0cnVlKTtcbiAgICAvLyDwn46T77iPIGB1bmRlZmluZWRgIGtlZXBzIHRoZSBXaW5kb3cncyBvd24gaW50ZXJuYWwgZm9sZCBzdGF0ZSDigJQgb25seSB3aW5kb3dzIG9mIHRoZSBpbnRyb2R1Y3Rpb24nc1xuICAgIC8vIHRhcmdldCBraW5kIChpbmNsdWRpbmcgZXZlcnkgb3BlbiBpbnN0YW5jZSkgYXJlIGZvcmNlLWNvbnRyb2xsZWQgdG8gYGZhbHNlYCB3aGlsZSBpdHMgdXRpbGl0eSBzdGVwXG4gICAgLy8gaXMgYWN0aXZlLlxuICAgIGNvbnN0IHV0aWxpdHlCYXJGb2xkZWRGb3IgPSAod2luZG93SWQ6IHN0cmluZywgd2luZG93S2luZElkOiBzdHJpbmcgPSB3aW5kb3dJZCk6IGJvb2xlYW4gfCB1bmRlZmluZWQgPT5cbiAgICAgIGludHJvZHVjdGlvblRhcmdldHNXaW5kb3cod2luZG93SWQsIHdpbmRvd0tpbmRJZCwgaW50cm9kdWN0aW9uVXRpbGl0eVdpbmRvd0lkKSA/IGZhbHNlIDogdW5kZWZpbmVkO1xuICAgIGNvbnN0IG1lYXN1cmVzRm9sZGVkRm9yID0gKHdpbmRvd0lkOiBzdHJpbmcsIHdpbmRvd0tpbmRJZDogc3RyaW5nID0gd2luZG93SWQpOiBib29sZWFuIHwgdW5kZWZpbmVkID0+XG4gICAgICBpbnRyb2R1Y3Rpb25UYXJnZXRzV2luZG93KHdpbmRvd0lkLCB3aW5kb3dLaW5kSWQsIGludHJvZHVjdGlvbk1lYXN1cmVXaW5kb3dJZCkgPyBmYWxzZSA6IHVuZGVmaW5lZDtcbiAgICBjb25zdCBvbkFjdGlvbnNGb2xkZWRGb3IgPSAod2luZG93SWQ6IHN0cmluZykgPT4gKGZvbGRlZDogYm9vbGVhbikgPT4gZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9BQ1RJT05fUEFORV9GT0xERURcIiwgd2luZG93SWQsIHZhbHVlOiBmb2xkZWQgfSk7XG4gICAgLy8g8J+Wse+4jyBXaW5kb3ctYm9keSBjdXJzb3IgZm9sbG93cyB0aGUgYWN0aXZlIHV0aWxpdHkncyBkZWNsYXJlZCBgY3Vyc29yYCAoUDUpLlxuICAgIGNvbnN0IGN1cnNvckZvciA9IChhcHA6IEFwcERlZmluaXRpb24sIHdpbmRvd0lkOiBzdHJpbmcpOiBDU1NQcm9wZXJ0aWVzIHwgdW5kZWZpbmVkID0+IHtcbiAgICAgIGNvbnN0IHV0aWxpdHlJZCA9IGFjdGl2ZVV0aWxpdHlCeVdpbmRvd0lkW3dpbmRvd0lkXTtcbiAgICAgIGNvbnN0IGN1cnNvciA9IHV0aWxpdHlJZCA/IChhcHAudXRpbGl0aWVzID8/IFtdKS5maW5kKCh1dGlsaXR5KSA9PiB1dGlsaXR5LmlkID09PSB1dGlsaXR5SWQpPy5jdXJzb3IgOiB1bmRlZmluZWQ7XG4gICAgICByZXR1cm4gY3Vyc29yID8geyBjdXJzb3IgfSA6IHVuZGVmaW5lZDtcbiAgICB9O1xuICAgIGlmIChzdHVkaW9Nb2RlICYmIHNwYXduZWRXaW5kb3dVaSAmJiBwYW5lbD8uYWN0aXZlU3Bhd25lZElkKSB7XG4gICAgICBjb25zdCBzcGF3bmVkID0gcGFuZWwuc3Bhd25lZEFwcHMuZmluZCgoZW50cnkpID0+IGVudHJ5LmlkID09PSBwYW5lbC5hY3RpdmVTcGF3bmVkSWQpO1xuICAgICAgaWYgKHNwYXduZWQpIHtcbiAgICAgICAgY29uc3Qgc3Bhd25lZEFwcCA9IGxvYWRlZFBsdWdpbnMuZmluZCgoZW50cnkpID0+IGVudHJ5LmhhbmRsZS5wbHVnaW5JZCA9PT0gc3Bhd25lZC5wbHVnaW5JZCk/Lm1hbmlmZXN0LmFwcHMuZmluZCgoY2FuZGlkYXRlKSA9PiBjYW5kaWRhdGUuaWQgPT09IHNwYXduZWQuYXBwSWQpO1xuICAgICAgICBjb25zdCB3aW5kb3dLaW5kID0gc3Bhd25lZEFwcD8ud2luZG93S2luZHNbMF07XG4gICAgICAgIGNvbnN0IGNocm9tZSA9IHdpbmRvd0tpbmQgPyBzcGF3bmVkV2luZG93Q2hyb21lRm9yS2luZCh3aW5kb3dLaW5kLCBzcGF3bmVkLmlkLCBzcGF3bmVkV2luZG93RW5nYWdlbWVudHMsIHNwYXduZWRXaW5kb3dNZWFzdXJlcywgYWN0aXZlVXRpbGl0eUJ5V2luZG93SWRbc3Bhd25lZC5pZF0sIG9uQWN0aW9uU3RhYmxlKSA6IHVuZGVmaW5lZDtcbiAgICAgICAgY29uc3Qgc3Bhd25lZFV0aWxpdGllcyA9IHNwYXduZWRBcHAgJiYgd2luZG93S2luZCA/IHJlc29sdmVVdGlsaXR5Tm9kZXMoc3Bhd25lZEFwcCwgd2luZG93S2luZCwgYWN0aXZlVXRpbGl0eUJ5V2luZG93SWRbc3Bhd25lZC5pZF0sIHNwYXduZWQuaWQsIGFwcExhYmVsc092ZXJsYXksIHVpVGVybWlub2xvZ3ksIHVpTG9jYWxlKSA6IFtdO1xuICAgICAgICByZXR1cm4gW1xuICAgICAgICAgIHtcbiAgICAgICAgICAgIGlkOiBzcGF3bmVkLmlkLFxuICAgICAgICAgICAgdGl0bGU6IHdpcmVMYWJlbChhcHBEb2N1bWVudExhYmVsKHNwYXduZWRBcHAgPyByZXNvbHZlQXBwRG9jdW1lbnQoc3Bhd25lZEFwcCwgdWlUZXJtaW5vbG9neSkgOiBzcGF3bmVkLmRvY3VtZW50KSksXG4gICAgICAgICAgICBmaWxsOiB0cnVlLFxuICAgICAgICAgICAgc2hvd0NvbnRyb2xzOiB0cnVlLFxuICAgICAgICAgICAgbWVhc3VyZXM6IGNocm9tZT8ubWVhc3VyZXMsXG4gICAgICAgICAgICBtZWFzdXJlc0ZvbGRlZDogbWVhc3VyZXNGb2xkZWRGb3Ioc3Bhd25lZC5pZCwgd2luZG93S2luZD8uaWQgPz8gc3Bhd25lZC5pZCksXG4gICAgICAgICAgICBlbmdhZ2VtZW50OiBjaHJvbWU/LmVuZ2FnZW1lbnQsXG4gICAgICAgICAgICBzZWFyY2g6IGNocm9tZT8uc2VhcmNoLFxuICAgICAgICAgICAgdXRpbGl0eUJhcjogc3Bhd25lZEFwcCAmJiB3aW5kb3dLaW5kID8gdXRpbGl0eUJhck5vZGUoc3Bhd25lZFV0aWxpdGllcywgc3Bhd25lZC5pZCwgb25BY3Rpb25TdGFibGUsIGludHJvZHVjdGlvblV0aWxpdHlJZCwgY2hyb21lPy51dGlsaXR5T3B0aW9ucykgOiB1bmRlZmluZWQsXG4gICAgICAgICAgICB1dGlsaXR5QmFyRm9sZGVkOiB1dGlsaXR5QmFyRm9sZGVkRm9yKHNwYXduZWQuaWQsIHdpbmRvd0tpbmQ/LmlkID8/IHNwYXduZWQuaWQpLFxuICAgICAgICAgICAgYWN0aW9uUGFuZTogc3Bhd25lZEFwcCAmJiB3aW5kb3dLaW5kID8gd2luZG93QWN0aW9uUGFuZU5vZGUoc3Bhd25lZEFwcCwgd2luZG93S2luZCwgc3Bhd25lZC5pZCwgYWN0aW9uUGFuZVNsaWNlLCBvbkFjdGlvblN0YWJsZSwgZGlzcGF0Y2gsIGFwcExhYmVsc092ZXJsYXksIHVpVGVybWlub2xvZ3ksIHVpTG9jYWxlKSA6IHVuZGVmaW5lZCxcbiAgICAgICAgICAgIGFjdGlvbnNGb2xkZWQ6IGFjdGlvbnNGb2xkZWRGb3Ioc3Bhd25lZC5pZCwgd2luZG93S2luZD8uaWQgPz8gc3Bhd25lZC5pZCksXG4gICAgICAgICAgICBvbkFjdGlvbnNGb2xkZWRDaGFuZ2U6IG9uQWN0aW9uc0ZvbGRlZEZvcihzcGF3bmVkLmlkKSxcbiAgICAgICAgICAgIGNoaWxkcmVuOiAoXG4gICAgICAgICAgICAgIDxDaHJvbWVBd2FyZVdpbmRvd1Njcm9sbFN1cmZhY2UgY2xhc3NOYW1lPVwicmVsYXRpdmUgZmxleCBoLWZ1bGwgbWluLWgtMCBtaW4tdy0wIGZsZXgtMSBmbGV4LWNvbCBvdmVyZmxvdy1oaWRkZW5cIiBzdHlsZT17c3Bhd25lZEFwcCA/IGN1cnNvckZvcihzcGF3bmVkQXBwLCBzcGF3bmVkLmlkKSA6IHVuZGVmaW5lZH0+XG4gICAgICAgICAgICAgICAgPFNoZWxsRmF1bHRCb3VuZGFyeSBib3VuZGFyeUlkPXtgd2luZG93LSR7c3Bhd25lZC5pZH1gfSBmYWxsYmFja0xhYmVsPXtzaGVsbExhYmVsKFwidWkuY29tbW9uLnJlbmRlckVycm9yXCIpfT5cbiAgICAgICAgICAgICAgICAgIDxJbnRlcnByZXRlZFVpTm9kZSBub2RlPXtzcGF3bmVkV2luZG93VWl9IG9uQWN0aW9uPXtvbkFjdGlvblN0YWJsZX0gLz5cbiAgICAgICAgICAgICAgICA8L1NoZWxsRmF1bHRCb3VuZGFyeT5cbiAgICAgICAgICAgICAgPC9DaHJvbWVBd2FyZVdpbmRvd1Njcm9sbFN1cmZhY2U+XG4gICAgICAgICAgICApLFxuICAgICAgICAgIH0sXG4gICAgICAgIF07XG4gICAgICB9XG4gICAgfVxuICAgIGlmIChPYmplY3Qua2V5cyh3aW5kb3dVaUJ5V2luZG93SWQpLmxlbmd0aCA9PT0gMCkgcmV0dXJuIFtdO1xuICAgIGNvbnN0IGJhc2VXaW5kb3dzID0gc2Vzc2lvbi5hcHAud2luZG93S2luZHMubWFwKChraW5kKSA9PiB7XG4gICAgICBjb25zdCB1dGlsaXRpZXMgPSByZXNvbHZlVXRpbGl0eU5vZGVzKHNlc3Npb24uYXBwLCBraW5kLCBhY3RpdmVVdGlsaXR5QnlXaW5kb3dJZFtraW5kLmlkXSwga2luZC5pZCwgYXBwTGFiZWxzT3ZlcmxheSwgdWlUZXJtaW5vbG9neSwgdWlMb2NhbGUpO1xuICAgICAgY29uc3QgY2hyb21lID0gd2luZG93TWVhc3VyZXNDaHJvbWUod2luZG93TWVhc3VyZXNCeVdpbmRvd0lkW2tpbmQuaWRdID8/IGtpbmQub3B0aW9ucy5tZWFzdXJlcywgYWN0aXZlVXRpbGl0eUJ5V2luZG93SWRba2luZC5pZF0sIGtpbmQuaWQsIG9uQWN0aW9uU3RhYmxlKTtcbiAgICAgIGNvbnN0IHJlc29sdmVkRW5nYWdlbWVudCA9IHJlc29sdmVXaW5kb3dFbmdhZ2VtZW50KGtpbmQsIGtpbmQuaWQsIHdpbmRvd0VuZ2FnZW1lbnRzQnlXaW5kb3dJZCk7XG4gICAgICByZXR1cm4ge1xuICAgICAgICBpZDoga2luZC5pZCxcbiAgICAgICAgaWNvbklkOiB3aW5kb3dJY29uc0J5SWRba2luZC5pZF0gPz8ga2luZC5pY29uSWQsXG4gICAgICAgIHRpdGxlOiB3aW5kb3dUaXRsZXNCeUlkW2tpbmQuaWRdID8/IGFwcFdpbmRvd0RvY3VtZW50TGFiZWwoc2Vzc2lvbi5hcHAsIHVpVGVybWlub2xvZ3ksIHJlc29sdmVBcHBMYWJlbChhcHBMYWJlbHNPdmVybGF5LCBcIndpbmRvd0tpbmRcIiwga2luZC5pZCwgcmVzb2x2ZU1hbmlmZXN0TGFiZWwoa2luZC5sYWJlbCwgdWlUZXJtaW5vbG9neSwgdWlMb2NhbGUpKSwgdWlMb2NhbGUpLFxuICAgICAgICBmaWxsOiB0cnVlLFxuICAgICAgICBzaG93Q29udHJvbHM6IHRydWUsXG4gICAgICAgIG1lYXN1cmVzOiBjaHJvbWUubWVhc3VyZXMsXG4gICAgICAgIG1lYXN1cmVzRm9sZGVkOiBtZWFzdXJlc0ZvbGRlZEZvcihraW5kLmlkLCBraW5kLmlkKSxcbiAgICAgICAgZW5nYWdlbWVudDogd2luZG93RW5nYWdlbWVudFRvU3BlYyhyZXNvbHZlZEVuZ2FnZW1lbnQsIG9uQWN0aW9uU3RhYmxlKSxcbiAgICAgICAgc2VhcmNoOiB3aW5kb3dFbmdhZ2VtZW50VG9TZWFyY2hTcGVjKHJlc29sdmVkRW5nYWdlbWVudCwgb25BY3Rpb25TdGFibGUpLFxuICAgICAgICB1dGlsaXR5QmFyOiB1dGlsaXR5QmFyTm9kZSh1dGlsaXRpZXMsIGtpbmQuaWQsIG9uQWN0aW9uU3RhYmxlLCBpbnRyb2R1Y3Rpb25VdGlsaXR5SWQsIGNocm9tZS51dGlsaXR5T3B0aW9ucyksXG4gICAgICAgIHV0aWxpdHlCYXJGb2xkZWQ6IHV0aWxpdHlCYXJGb2xkZWRGb3Ioa2luZC5pZCwga2luZC5pZCksXG4gICAgICAgIGFjdGlvblBhbmU6IHdpbmRvd0FjdGlvblBhbmVOb2RlKHNlc3Npb24uYXBwLCBraW5kLCBraW5kLmlkLCBhY3Rpb25QYW5lU2xpY2UsIG9uQWN0aW9uU3RhYmxlLCBkaXNwYXRjaCwgYXBwTGFiZWxzT3ZlcmxheSwgdWlUZXJtaW5vbG9neSwgdWlMb2NhbGUpLFxuICAgICAgICBhY3Rpb25zRm9sZGVkOiBhY3Rpb25zRm9sZGVkRm9yKGtpbmQuaWQsIGtpbmQuaWQpLFxuICAgICAgICBvbkFjdGlvbnNGb2xkZWRDaGFuZ2U6IG9uQWN0aW9uc0ZvbGRlZEZvcihraW5kLmlkKSxcbiAgICAgICAgc3RhdHVzOiBkZWNsYXJhdGl2ZVN1cmZhY2VTdGF0dXMod2luZG93VWlCeVdpbmRvd0lkW2tpbmQuaWRdKSxcbiAgICAgICAgc2tlbGV0b246IDxXaW5kb3dCb2R5U2tlbGV0b24gLz4sXG4gICAgICAgIGNoaWxkcmVuOiAoXG4gICAgICAgICAgPENocm9tZUF3YXJlV2luZG93U2Nyb2xsU3VyZmFjZSBpZD17Y2hpbGRFbGVtZW50SWQoXCJmcmFtZXdvcmsud2luZG93XCIsIGtpbmQuaWQpfSBjbGFzc05hbWU9XCJyZWxhdGl2ZSBmbGV4IGgtZnVsbCBtaW4taC0wIG1pbi13LTAgZmxleC0xIGZsZXgtY29sIG92ZXJmbG93LWhpZGRlblwiIHN0eWxlPXtjdXJzb3JGb3Ioc2Vzc2lvbi5hcHAsIGtpbmQuaWQpfT5cbiAgICAgICAgICAgIDxXaW5kb3dJbnN0YW5jZUlkQ29udGV4dC5Qcm92aWRlciB2YWx1ZT17a2luZC5pZH0+XG4gICAgICAgICAgICAgIDxTaGVsbEZhdWx0Qm91bmRhcnkgYm91bmRhcnlJZD17YHdpbmRvdy0ke2tpbmQuaWR9YH0gZmFsbGJhY2tMYWJlbD17c2hlbGxMYWJlbChcInVpLmNvbW1vbi5yZW5kZXJFcnJvclwiKX0+XG4gICAgICAgICAgICAgICAgPEludGVycHJldGVkVWlOb2RlIG5vZGU9e3dpbmRvd1VpQnlXaW5kb3dJZFtraW5kLmlkXSA/PyBwZW5kaW5nV2luZG93VWlOb2RlKCl9IG9uQWN0aW9uPXtvbkFjdGlvblN0YWJsZX0gLz5cbiAgICAgICAgICAgICAgPC9TaGVsbEZhdWx0Qm91bmRhcnk+XG4gICAgICAgICAgICA8L1dpbmRvd0luc3RhbmNlSWRDb250ZXh0LlByb3ZpZGVyPlxuICAgICAgICAgIDwvQ2hyb21lQXdhcmVXaW5kb3dTY3JvbGxTdXJmYWNlPlxuICAgICAgICApLFxuICAgICAgfTtcbiAgICB9KTtcbiAgICAvLyDwn6qf77iPIEVhY2ggZXh0cmEgKHNwbGl0L3NwYXduZWQpIGluc3RhbmNlIHJlbmRlcnMgaXRzIE9XTiBgd2luZG93VWlCeVdpbmRvd0lkW2luc3RhbmNlLmlkXWAgYm9keSxcbiAgICAvLyBtZWFzdXJlcywgYW5kIGVuZ2FnZW1lbnQg4oCUIG5ldmVyIHRoZSBiYXNlIGtpbmQncyBzaGFyZWQgZW50cnkg4oCUIHNvIHR3byBpbnN0YW5jZXMgb2YgdGhlIHNhbWUga2luZFxuICAgIC8vIChlLmcuIHNwbGl0IHRvcC9wZXJzcGVjdGl2ZSBwYW5lcykgbmV2ZXIgc2hvdyBvciBhZmZlY3QgZWFjaCBvdGhlcidzIG9wdGlvbnMuIGBkYXRhLWVsZW1lbnQtYWxpYXNgXG4gICAgLy8gYWxpYXNlcyB0aGUgaW5zdGFuY2UgdG8gaXRzIHdpbmRvdyBraW5kJ3MgZWxlbWVudCBpZCBzbyBhbiBpbnRyb2R1Y3Rpb24gYHNob3dgIHRhcmdldCBvZiB0aGUga2luZFxuICAgIC8vIChub3QgYSBzcGVjaWZpYyBpbnN0YW5jZSkgcmFpc2VzIGV2ZXJ5IG9wZW4gaW5zdGFuY2UgYWJvdmUgdGhlIGdsYXNzLCBub3Qgb25seSB0aGUgYmFzZSBvbmUuXG4gICAgY29uc3QgZXh0cmFXaW5kb3dzID0gZXh0cmFXaW5kb3dJbnN0YW5jZXMuZmxhdE1hcCgoaW5zdGFuY2UpID0+IHtcbiAgICAgIGNvbnN0IGtpbmQgPSBzZXNzaW9uLmFwcC53aW5kb3dLaW5kcy5maW5kKChlbnRyeSkgPT4gZW50cnkuaWQgPT09IGluc3RhbmNlLndpbmRvd0tpbmRJZCk7XG4gICAgICBpZiAoIWtpbmQpIHJldHVybiBbXTtcbiAgICAgIGNvbnN0IHV0aWxpdGllcyA9IHJlc29sdmVVdGlsaXR5Tm9kZXMoc2Vzc2lvbi5hcHAsIGtpbmQsIGFjdGl2ZVV0aWxpdHlCeVdpbmRvd0lkW2luc3RhbmNlLmlkXSwgaW5zdGFuY2UuaWQsIGFwcExhYmVsc092ZXJsYXksIHVpVGVybWlub2xvZ3ksIHVpTG9jYWxlKTtcbiAgICAgIGNvbnN0IGNocm9tZSA9IHdpbmRvd01lYXN1cmVzQ2hyb21lKHdpbmRvd01lYXN1cmVzQnlXaW5kb3dJZFtpbnN0YW5jZS5pZF0gPz8ga2luZC5vcHRpb25zLm1lYXN1cmVzLCBhY3RpdmVVdGlsaXR5QnlXaW5kb3dJZFtpbnN0YW5jZS5pZF0sIGluc3RhbmNlLmlkLCBvbkFjdGlvblN0YWJsZSk7XG4gICAgICBjb25zdCByZXNvbHZlZEVuZ2FnZW1lbnQgPSByZXNvbHZlV2luZG93RW5nYWdlbWVudChraW5kLCBpbnN0YW5jZS5pZCwgd2luZG93RW5nYWdlbWVudHNCeVdpbmRvd0lkKTtcbiAgICAgIHJldHVybiBbXG4gICAgICAgIHtcbiAgICAgICAgICBpZDogaW5zdGFuY2UuaWQsXG4gICAgICAgICAgaWNvbklkOiB3aW5kb3dJY29uc0J5SWRbaW5zdGFuY2UuaWRdID8/IGtpbmQuaWNvbklkLFxuICAgICAgICAgIHRpdGxlOiB3aW5kb3dUaXRsZXNCeUlkW2luc3RhbmNlLmlkXSA/PyBpbnN0YW5jZS50aXRsZSxcbiAgICAgICAgICBmaWxsOiB0cnVlLFxuICAgICAgICAgIHNob3dDb250cm9sczogdHJ1ZSxcbiAgICAgICAgICBtZWFzdXJlczogY2hyb21lLm1lYXN1cmVzLFxuICAgICAgICAgIG1lYXN1cmVzRm9sZGVkOiBtZWFzdXJlc0ZvbGRlZEZvcihpbnN0YW5jZS5pZCwgaW5zdGFuY2Uud2luZG93S2luZElkKSxcbiAgICAgICAgICBlbmdhZ2VtZW50OiB3aW5kb3dFbmdhZ2VtZW50VG9TcGVjKHJlc29sdmVkRW5nYWdlbWVudCwgb25BY3Rpb25TdGFibGUpLFxuICAgICAgICAgIHNlYXJjaDogd2luZG93RW5nYWdlbWVudFRvU2VhcmNoU3BlYyhyZXNvbHZlZEVuZ2FnZW1lbnQsIG9uQWN0aW9uU3RhYmxlKSxcbiAgICAgICAgICB1dGlsaXR5QmFyOiB1dGlsaXR5QmFyTm9kZSh1dGlsaXRpZXMsIGluc3RhbmNlLmlkLCBvbkFjdGlvblN0YWJsZSwgaW50cm9kdWN0aW9uVXRpbGl0eUlkLCBjaHJvbWUudXRpbGl0eU9wdGlvbnMpLFxuICAgICAgICAgIHV0aWxpdHlCYXJGb2xkZWQ6IHV0aWxpdHlCYXJGb2xkZWRGb3IoaW5zdGFuY2UuaWQsIGluc3RhbmNlLndpbmRvd0tpbmRJZCksXG4gICAgICAgICAgYWN0aW9uUGFuZTogd2luZG93QWN0aW9uUGFuZU5vZGUoc2Vzc2lvbi5hcHAsIGtpbmQsIGluc3RhbmNlLmlkLCBhY3Rpb25QYW5lU2xpY2UsIG9uQWN0aW9uU3RhYmxlLCBkaXNwYXRjaCwgYXBwTGFiZWxzT3ZlcmxheSwgdWlUZXJtaW5vbG9neSwgdWlMb2NhbGUpLFxuICAgICAgICAgIGFjdGlvbnNGb2xkZWQ6IGFjdGlvbnNGb2xkZWRGb3IoaW5zdGFuY2UuaWQsIGluc3RhbmNlLndpbmRvd0tpbmRJZCksXG4gICAgICAgICAgb25BY3Rpb25zRm9sZGVkQ2hhbmdlOiBvbkFjdGlvbnNGb2xkZWRGb3IoaW5zdGFuY2UuaWQpLFxuICAgICAgICAgIHN0YXR1czogZGVjbGFyYXRpdmVTdXJmYWNlU3RhdHVzKHdpbmRvd1VpQnlXaW5kb3dJZFtpbnN0YW5jZS5pZF0pLFxuICAgICAgICAgIHNrZWxldG9uOiA8V2luZG93Qm9keVNrZWxldG9uIC8+LFxuICAgICAgICAgIGNoaWxkcmVuOiAoXG4gICAgICAgICAgICA8Q2hyb21lQXdhcmVXaW5kb3dTY3JvbGxTdXJmYWNlXG4gICAgICAgICAgICAgIGlkPXtjaGlsZEVsZW1lbnRJZChcImZyYW1ld29yay53aW5kb3dcIiwgaW5zdGFuY2UuaWQpfVxuICAgICAgICAgICAgICBkYXRhLWVsZW1lbnQtYWxpYXM9e2NoaWxkRWxlbWVudElkKFwiZnJhbWV3b3JrLndpbmRvd1wiLCBraW5kLmlkKX1cbiAgICAgICAgICAgICAgY2xhc3NOYW1lPVwicmVsYXRpdmUgZmxleCBoLWZ1bGwgbWluLWgtMCBtaW4tdy0wIGZsZXgtMSBmbGV4LWNvbCBvdmVyZmxvdy1oaWRkZW5cIlxuICAgICAgICAgICAgICBzdHlsZT17Y3Vyc29yRm9yKHNlc3Npb24uYXBwLCBpbnN0YW5jZS5pZCl9XG4gICAgICAgICAgICA+XG4gICAgICAgICAgICAgIDxXaW5kb3dJbnN0YW5jZUlkQ29udGV4dC5Qcm92aWRlciB2YWx1ZT17aW5zdGFuY2UuaWR9PlxuICAgICAgICAgICAgICAgIDxTaGVsbEZhdWx0Qm91bmRhcnkgYm91bmRhcnlJZD17YHdpbmRvdy0ke2luc3RhbmNlLmlkfWB9IGZhbGxiYWNrTGFiZWw9e3NoZWxsTGFiZWwoXCJ1aS5jb21tb24ucmVuZGVyRXJyb3JcIil9PlxuICAgICAgICAgICAgICAgICAgPEludGVycHJldGVkVWlOb2RlIG5vZGU9e3dpbmRvd1VpQnlXaW5kb3dJZFtpbnN0YW5jZS5pZF0gPz8gcGVuZGluZ1dpbmRvd1VpTm9kZSgpfSBvbkFjdGlvbj17b25BY3Rpb25TdGFibGV9IC8+XG4gICAgICAgICAgICAgICAgPC9TaGVsbEZhdWx0Qm91bmRhcnk+XG4gICAgICAgICAgICAgIDwvV2luZG93SW5zdGFuY2VJZENvbnRleHQuUHJvdmlkZXI+XG4gICAgICAgICAgICA8L0Nocm9tZUF3YXJlV2luZG93U2Nyb2xsU3VyZmFjZT5cbiAgICAgICAgICApLFxuICAgICAgICB9LFxuICAgICAgXTtcbiAgICB9KTtcbiAgICByZXR1cm4gWy4uLmJhc2VXaW5kb3dzLCAuLi5leHRyYVdpbmRvd3NdO1xuICB9LCBbXG4gICAgYWN0aW9uUGFuZUV4cGFuZGVkQnlXaW5kb3dJZCxcbiAgICBhY3Rpb25QYW5lRm9sZGVkQnlXaW5kb3dJZCxcbiAgICBhY3Rpb25QYW5lU3RhZ2VkQXJnc0J5S2V5LFxuICAgIGFjdGl2ZVV0aWxpdHlCeVdpbmRvd0lkLFxuICAgIGFwcExhYmVsc092ZXJsYXksXG4gICAgZXh0cmFXaW5kb3dJbnN0YW5jZXMsXG4gICAgaW50cm9kdWN0aW9uQWN0aW9uV2luZG93U2VnbWVudCxcbiAgICBpbnRyb2R1Y3Rpb25VdGlsaXR5SWQsXG4gICAgaW50cm9kdWN0aW9uVXRpbGl0eVdpbmRvd0lkLFxuICAgIGxvYWRlZFBsdWdpbnMsXG4gICAgb25BY3Rpb25TdGFibGUsXG4gICAgcGFuZWwsXG4gICAgc2Vzc2lvbixcbiAgICBzcGF3bmVkV2luZG93RW5nYWdlbWVudHMsXG4gICAgc3Bhd25lZFdpbmRvd01lYXN1cmVzLFxuICAgIHNwYXduZWRXaW5kb3dVaSxcbiAgICBzdHVkaW9Nb2RlLFxuICAgIHVpTG9jYWxlLFxuICAgIHVpVGVybWlub2xvZ3ksXG4gICAgd2luZG93RW5nYWdlbWVudHNCeVdpbmRvd0lkLFxuICAgIHdpbmRvd01lYXN1cmVzQnlXaW5kb3dJZCxcbiAgICB3aW5kb3dUaXRsZXNCeUlkLFxuICAgIHdpbmRvd0ljb25zQnlJZCxcbiAgICB3aW5kb3dVaUJ5V2luZG93SWQsXG4gIF0pO1xuXG4gIGNvbnN0IGVmZmVjdGl2ZU1vZGVMYXlvdXQgPSB1c2VNZW1vKFxuICAgICgpID0+XG4gICAgICBzaGVsbExheW91dCA/P1xuICAgICAgKHNlc3Npb24gPyByZXNvbHZlRnJhbWV3b3JrTGF5b3V0U2VlZChzZXNzaW9uLmFwcC5kZWZhdWx0TGF5b3V0LCBzZXNzaW9uLmFwcC53aW5kb3dLaW5kcywgYXBwTGFiZWxzT3ZlcmxheSwgdWlUZXJtaW5vbG9neSwgdWlMb2NhbGUpLm1vZGVMYXlvdXQgOiB7IGtpbmQ6IFwic3RhY2tcIiBhcyBjb25zdCwgY2hpbGRyZW46IFtdIH0pLFxuICAgIFthcHBMYWJlbHNPdmVybGF5LCBzZXNzaW9uLCBzaGVsbExheW91dCwgdWlUZXJtaW5vbG9neSwgdWlMb2NhbGVdLFxuICApO1xuXG4gIGNvbnN0IGhhbmRsZUFjdGl2ZVdpbmRvd0NoYW5nZSA9IHVzZUNhbGxiYWNrKFxuICAgICh2YWx1ZTogc3RyaW5nIHwgbnVsbCkgPT4ge1xuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9BQ1RJVkVfV0lORE9XX0lEXCIsIHZhbHVlIH0pO1xuICAgICAgaWYgKHZhbHVlKSBub3RlU2hlbGxDb21tYW5kKFwic2hlbGwud2luZG93QWN0aXZhdGVcIiwgc2hlbGxMYWJlbChcInVpLnNoZWxsQ29tbWFuZC53aW5kb3dBY3RpdmF0ZVwiKSwgeyB3aW5kb3dJZDogdmFsdWUgfSk7XG4gICAgfSxcbiAgICBbbm90ZVNoZWxsQ29tbWFuZF0sXG4gICk7XG5cbiAgLy8g8J+qn++4jyBgTW9kZS5vbkxheW91dENoYW5nZWAgZmlyZXMgY29udGludW91c2x5IGR1cmluZyBhIGxpdmUgZHJhZy9yZXNpemUgKG9uZSBjYWxsIHBlciBmcmFtZSkg4oCUIGNsYXNzaWZ5XG4gIC8vIGVhY2ggZGVsdGEgYWdhaW5zdCB0aGUgbGFzdC1zZWVuIGxheW91dCwgcmVtZW1iZXIgb25seSB0aGUgbGF0ZXN0IG5vbi1udWxsIGNsYXNzaWZpY2F0aW9uLCBhbmQgbm90ZSBhXG4gIC8vIHNpbmdsZSBzaGVsbCBjb21tYW5kIG9uY2UgdGhlIGRyYWcgc2V0dGxlcyAoc2VlIGBMQVlPVVRfQ0hBTkdFX1NFVFRMRV9NU2ApLiBBIHB1cmUgYWN0aXZlLXdpbmRvdy1mbGFnXG4gIC8vIGVjaG8gY2xhc3NpZmllcyBgbnVsbGAgYW5kIGlzIHNpbGVudGx5IHNraXBwZWQgaGVyZSAoaGFuZGxlZCBieSBgaGFuZGxlQWN0aXZlV2luZG93Q2hhbmdlYCBpbnN0ZWFkKS5cbiAgY29uc3QgbGF5b3V0Q2hhbmdlU2V0dGxlVGltZW91dFJlZiA9IHVzZVJlZjxSZXR1cm5UeXBlPHR5cGVvZiBzZXRUaW1lb3V0PiB8IG51bGw+KG51bGwpO1xuICBjb25zdCBsYXlvdXRDaGFuZ2VDbGFzc2lmaWNhdGlvblJlZiA9IHVzZVJlZjxcInJlc2l6ZVwiIHwgXCJyZWFycmFuZ2VcIiB8IG51bGw+KG51bGwpO1xuICBjb25zdCBsYXlvdXRDaGFuZ2VQcmV2aW91c1JlZiA9IHVzZVJlZjxXaW5kb3dMYXlvdXROb2RlIHwgbnVsbD4oZWZmZWN0aXZlTW9kZUxheW91dCk7XG4gIHVzZUVmZmVjdCgoKSA9PiB7XG4gICAgbGF5b3V0Q2hhbmdlUHJldmlvdXNSZWYuY3VycmVudCA9IGVmZmVjdGl2ZU1vZGVMYXlvdXQ7XG4gIH0sIFtlZmZlY3RpdmVNb2RlTGF5b3V0XSk7XG4gIHVzZUVmZmVjdChcbiAgICAoKSA9PiAoKSA9PiB7XG4gICAgICBpZiAobGF5b3V0Q2hhbmdlU2V0dGxlVGltZW91dFJlZi5jdXJyZW50KSBjbGVhclRpbWVvdXQobGF5b3V0Q2hhbmdlU2V0dGxlVGltZW91dFJlZi5jdXJyZW50KTtcbiAgICB9LFxuICAgIFtdLFxuICApO1xuICBjb25zdCBoYW5kbGVNb2RlTGF5b3V0Q2hhbmdlID0gdXNlQ2FsbGJhY2soXG4gICAgKHZhbHVlOiBXaW5kb3dMYXlvdXROb2RlKSA9PiB7XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1NIRUxMX0xBWU9VVFwiLCB2YWx1ZSB9KTtcbiAgICAgIGNvbnN0IGNsYXNzaWZpY2F0aW9uID0gY2xhc3NpZnlXaW5kb3dMYXlvdXRDaGFuZ2UobGF5b3V0Q2hhbmdlUHJldmlvdXNSZWYuY3VycmVudCwgdmFsdWUpO1xuICAgICAgbGF5b3V0Q2hhbmdlUHJldmlvdXNSZWYuY3VycmVudCA9IHZhbHVlO1xuICAgICAgaWYgKGNsYXNzaWZpY2F0aW9uKSBsYXlvdXRDaGFuZ2VDbGFzc2lmaWNhdGlvblJlZi5jdXJyZW50ID0gY2xhc3NpZmljYXRpb247XG4gICAgICBpZiAobGF5b3V0Q2hhbmdlU2V0dGxlVGltZW91dFJlZi5jdXJyZW50KSBjbGVhclRpbWVvdXQobGF5b3V0Q2hhbmdlU2V0dGxlVGltZW91dFJlZi5jdXJyZW50KTtcbiAgICAgIGxheW91dENoYW5nZVNldHRsZVRpbWVvdXRSZWYuY3VycmVudCA9IHNldFRpbWVvdXQoKCkgPT4ge1xuICAgICAgICBsYXlvdXRDaGFuZ2VTZXR0bGVUaW1lb3V0UmVmLmN1cnJlbnQgPSBudWxsO1xuICAgICAgICBjb25zdCBmaW5hbENsYXNzaWZpY2F0aW9uID0gbGF5b3V0Q2hhbmdlQ2xhc3NpZmljYXRpb25SZWYuY3VycmVudDtcbiAgICAgICAgbGF5b3V0Q2hhbmdlQ2xhc3NpZmljYXRpb25SZWYuY3VycmVudCA9IG51bGw7XG4gICAgICAgIGlmIChmaW5hbENsYXNzaWZpY2F0aW9uID09PSBcInJlc2l6ZVwiKSBub3RlU2hlbGxDb21tYW5kKFwic2hlbGwud2luZG93UmVzaXplXCIsIHNoZWxsTGFiZWwoXCJ1aS5zaGVsbENvbW1hbmQud2luZG93UmVzaXplXCIpKTtcbiAgICAgICAgZWxzZSBpZiAoZmluYWxDbGFzc2lmaWNhdGlvbiA9PT0gXCJyZWFycmFuZ2VcIikgbm90ZVNoZWxsQ29tbWFuZChcInNoZWxsLndpbmRvd01vdmVcIiwgc2hlbGxMYWJlbChcInVpLnNoZWxsQ29tbWFuZC53aW5kb3dNb3ZlXCIpKTtcbiAgICAgIH0sIExBWU9VVF9DSEFOR0VfU0VUVExFX01TKTtcbiAgICB9LFxuICAgIFtub3RlU2hlbGxDb21tYW5kXSxcbiAgKTtcblxuICBjb25zdCBjYW52YXMgPSB1c2VNZW1vKCgpID0+IHtcbiAgICBpZiAoc3R1ZGlvTW9kZSAmJiBzaGVsbFJvdXRlLmtpbmQgPT09IFwibm90Rm91bmRcIikge1xuICAgICAgcmV0dXJuIDxTaGVsbFJvdXRlTm90Rm91bmRQYWdlIHBhdGg9e3NoZWxsUm91dGUucGF0aH0gb25Ib21lPXsoKSA9PiBuYXZpZ2F0ZUhpc3RvcnkoXCIvXCIpfSAvPjtcbiAgICB9XG4gICAgY29uc3Qgc3VwZXJ2aXNvclBsdWdpbklkID0gcHJpbWFyeVBsdWdpbklkO1xuICAgIGNvbnN0IHN1cGVydmlzb3JTdGF0ZSA9IHN1cGVydmlzb3JQbHVnaW5JZCA/IHBsdWdpblN1cGVydmlzb3JCeUlkW3N1cGVydmlzb3JQbHVnaW5JZF0gOiB1bmRlZmluZWQ7XG4gICAgaWYgKHN1cGVydmlzb3JTdGF0ZSA9PT0gXCJjcmFzaGVkXCIgfHwgc3VwZXJ2aXNvclN0YXRlID09PSBcInF1YXJhbnRpbmVkXCIpIHtcbiAgICAgIHJldHVybiAoXG4gICAgICAgIDxQbHVnaW5SZWNvdmVyeVBhbmVsXG4gICAgICAgICAgcGx1Z2luSWQ9e3N1cGVydmlzb3JQbHVnaW5JZCF9XG4gICAgICAgICAgcXVhcmFudGluZWQ9e3N1cGVydmlzb3JTdGF0ZSA9PT0gXCJxdWFyYW50aW5lZFwifVxuICAgICAgICAgIG9uUmVzdGFydD17KCkgPT4ge1xuICAgICAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9QTFVHSU5fU1VQRVJWSVNPUlwiLCBwbHVnaW5JZDogc3VwZXJ2aXNvclBsdWdpbklkISwgdmFsdWU6IFwicmVzdGFydGluZ1wiIH0pO1xuICAgICAgICAgICAgdm9pZCByZWxvYWRQbHVnaW4oc3VwZXJ2aXNvclBsdWdpbklkISk7XG4gICAgICAgICAgfX1cbiAgICAgICAgICBvbkRpc2FibGU9eygpID0+IHtcbiAgICAgICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfUExVR0lOX1NVUEVSVklTT1JcIiwgcGx1Z2luSWQ6IHN1cGVydmlzb3JQbHVnaW5JZCEsIHZhbHVlOiBcInF1YXJhbnRpbmVkXCIgfSk7XG4gICAgICAgICAgICBpZiAoc3VwZXJ2aXNvclBsdWdpbklkICE9PSBwcmltYXJ5UGx1Z2luSWQpIHZvaWQgdW5pbnN0YWxsUGx1Z2luKHN1cGVydmlzb3JQbHVnaW5JZCEpO1xuICAgICAgICAgIH19XG4gICAgICAgIC8+XG4gICAgICApO1xuICAgIH1cbiAgICBpZiAoZXJyb3IpXG4gICAgICByZXR1cm4gKFxuICAgICAgICA8cCBjbGFzc05hbWU9XCJwLWRvdWJsZSB0ZXh0LXNtIHRleHQtZGVzdHJ1Y3RpdmVcIiByb2xlPVwiYWxlcnRcIiBkYXRhLXNlbWlvLW9zLXNoZWxsLWVycm9yPVwiXCI+XG4gICAgICAgICAge2Vycm9yfVxuICAgICAgICA8L3A+XG4gICAgICApO1xuICAgIGlmICghc2Vzc2lvbikgcmV0dXJuIDxDYW52YXNTa2VsZXRvbiBsYWJlbD17c2hlbGxMYWJlbChcInVpLmNvbW1vbi5sb2FkaW5nUGx1Z2luc1wiKX0gY2xhc3NOYW1lPXtjbihsb2FkaW5nQm9yZGVyQ2xhc3MsIFwiaC1mdWxsIHctZnVsbFwiKX0gLz47XG4gICAgY29uc3QgbW9kZXMgPSBzZXNzaW9uLmFwcC5tb2Rlcy5sZW5ndGggPiAwID8gc2Vzc2lvbi5hcHAubW9kZXMgOiBbeyBpZDogc2Vzc2lvbi5hcHAuaWQsIGxhYmVsOiBhcHBEb2N1bWVudExhYmVsKHJlc29sdmVBcHBEb2N1bWVudChzZXNzaW9uLmFwcCwgdWlUZXJtaW5vbG9neSkpIH1dO1xuICAgIGNvbnN0IHN0dWRpb0hvbWVCYXIgPVxuICAgICAgc3R1ZGlvTW9kZSAmJiBzZXNzaW9uLmFwcC5pZCA9PT0gaG9zdEFwcElkICYmICFwYW5lbD8uYWN0aXZlU3Bhd25lZElkID8gKFxuICAgICAgICA8YnV0dG9uXG4gICAgICAgICAgdHlwZT1cImJ1dHRvblwiXG4gICAgICAgICAgY2xhc3NOYW1lPXtjbihib3JkZXJOb3JtYWxCb3R0b21DbGFzcywgXCJweC1zaW5nbGUgcHktc2luZ2xlIHRleHQtbGVmdCB0ZXh0LXNtIHRleHQtbXV0ZWQtZm9yZWdyb3VuZCBob3ZlcjpiZy1tdXRlZC80MCBob3Zlcjp0ZXh0LWZvcmVncm91bmRcIil9XG4gICAgICAgICAgb25DbGljaz17KCkgPT4gb25BY3Rpb24oeyBjb250cm9sbGVySWQ6IHNlc3Npb24uYXBwLmNvbnRyb2xsZXJJZCwgYWN0aW9uOiBcImdvSG9tZVwiIH0pfVxuICAgICAgICA+XG4gICAgICAgICAg4oaQIHtzaGVsbExhYmVsKFwidWkuY29tbW9uLmhvbWVcIil9XG4gICAgICAgIDwvYnV0dG9uPlxuICAgICAgKSA6IG51bGw7XG4gICAgY29uc3QgZm9jdXNlZFNwYXduZWQgPSBwYW5lbD8uYWN0aXZlU3Bhd25lZElkID8gcGFuZWwuc3Bhd25lZEFwcHMuZmluZCgoZW50cnkpID0+IGVudHJ5LmlkID09PSBwYW5lbC5hY3RpdmVTcGF3bmVkSWQpIDogdW5kZWZpbmVkO1xuICAgIGNvbnN0IGZvY3VzZWRCYXIgPSBmb2N1c2VkU3Bhd25lZCA/IChcbiAgICAgIDxkaXYgY2xhc3NOYW1lPXtjbihib3JkZXJOb3JtYWxCb3R0b21DbGFzcywgXCJmbGV4IGl0ZW1zLWNlbnRlciBnYXAtc2luZ2xlIHB4LXNpbmdsZSBweS1zaW5nbGUgdGV4dC1zbSB0ZXh0LW11dGVkLWZvcmVncm91bmRcIil9PlxuICAgICAgICA8YnV0dG9uIHR5cGU9XCJidXR0b25cIiBjbGFzc05hbWU9XCJob3Zlcjp0ZXh0LWZvcmVncm91bmRcIiBvbkNsaWNrPXsoKSA9PiAob3BlblNwYWNlSWRSZWYuY3VycmVudCA/IG5hdmlnYXRlSGlzdG9yeShgL3NwYWNlcy8ke29wZW5TcGFjZUlkUmVmLmN1cnJlbnR9YCkgOiBvbkFjdGlvbih7IGNvbnRyb2xsZXJJZDogc2Vzc2lvbi5hcHAuY29udHJvbGxlcklkLCBhY3Rpb246IFwiY2xvc2VGb2N1c2VkSW5zdGFuY2VcIiB9KSl9PlxuICAgICAgICAgIOKGkCB7c2hlbGxMYWJlbChcInVpLmNvbW1vbi5iYWNrVG9Xb3JrZmxvd1wiKX1cbiAgICAgICAgPC9idXR0b24+XG4gICAgICAgIDxzcGFuPsK3PC9zcGFuPlxuICAgICAgICA8c3Bhbj57YXBwRG9jdW1lbnRMYWJlbChyZXNvbHZlRG9jdW1lbnRCeUFwcElkKGxvYWRlZFBsdWdpbnMsIGZvY3VzZWRTcGF3bmVkLmFwcElkLCBmb2N1c2VkU3Bhd25lZC5kb2N1bWVudCwgdWlUZXJtaW5vbG9neSkpfTwvc3Bhbj5cbiAgICAgIDwvZGl2PlxuICAgICkgOiBudWxsO1xuICAgIHJldHVybiAoXG4gICAgICA8ZGl2IGNsYXNzTmFtZT1cImZsZXggaC1mdWxsIG1pbi1oLTAgZmxleC1jb2wgb3ZlcmZsb3ctaGlkZGVuXCI+XG4gICAgICAgIHtzdHVkaW9Ib21lQmFyfVxuICAgICAgICB7Zm9jdXNlZEJhcn1cbiAgICAgICAgPGlucHV0XG4gICAgICAgICAgcmVmPXtpbXBvcnRTcGFjZUlucHV0UmVmfVxuICAgICAgICAgIHR5cGU9XCJmaWxlXCJcbiAgICAgICAgICAvLyDwn5Om77iPIGAucGFja2AgZmlsZXMgYnJhbmNoIHRvIGBzL3BsdWdpbmAncyBwYWNrLWF3YXJlIGBpbXBvcnRTcGFjZVBhY2tQYXlsb2FkYCBhY3Rpb25cbiAgICAgICAgICAvLyAoYHNlbWlvX2ZyYW1ld29ya19vczo6aW1wb3J0X29zX3NwYWNlX2Zyb21fcGFja2AsIHdhdmUgMiBzK3Nob21lK3NzdHVkaW8gZmFtaWx5KSDigJRcbiAgICAgICAgICAvLyByZWFkIGFzIGEgZGF0YVVybCwgc2FtZSBzaGFwZSBhcyB0aGUgZ2VuZXJpYyBgUmVxdWVzdEZpbGVPcGVuYC9gcmVhZEFzOiBcImRhdGFVcmxcImAgcGF0aFxuICAgICAgICAgIC8vIGJlbG93LiBBbnl0aGluZyBlbHNlIGtlZXBzIHJlYWRpbmcgYXMgdGV4dCBhbmQgZGlzcGF0Y2hpbmcgdGhlIEpTT04tZW52ZWxvcGUgXCJpbXBvcnRTcGFjZVwiLlxuICAgICAgICAgIGFjY2VwdD1cIi5zcGssLmRzbCwub3BzLGFwcGxpY2F0aW9uL29jdGV0LXN0cmVhbVwiXG4gICAgICAgICAgY2xhc3NOYW1lPVwiaGlkZGVuXCJcbiAgICAgICAgICBvbkNoYW5nZT17KGV2ZW50KSA9PiB7XG4gICAgICAgICAgICBjb25zdCBmaWxlID0gZXZlbnQudGFyZ2V0LmZpbGVzPy5bMF07XG4gICAgICAgICAgICBpZiAoIWZpbGUpIHJldHVybjtcbiAgICAgICAgICAgIGlmIChmaWxlLm5hbWUudG9Mb3dlckNhc2UoKS5lbmRzV2l0aChcIi5wYWNrXCIpKSB7XG4gICAgICAgICAgICAgIGNvbnN0IHJlYWRlciA9IG5ldyBGaWxlUmVhZGVyKCk7XG4gICAgICAgICAgICAgIHJlYWRlci5vbmxvYWQgPSAoKSA9PiB7XG4gICAgICAgICAgICAgICAgY29uc3QgcGF5bG9hZCA9IHR5cGVvZiByZWFkZXIucmVzdWx0ID09PSBcInN0cmluZ1wiID8gcmVhZGVyLnJlc3VsdCA6IFwiXCI7XG4gICAgICAgICAgICAgICAgb25BY3Rpb24oeyBjb250cm9sbGVySWQ6IGxhbmRpbmdDb250cm9sbGVySWQgPz8gXCJcIiwgYWN0aW9uOiBcImltcG9ydFNwYWNlUGFja1BheWxvYWRcIiwgYXJnczogeyBwYXlsb2FkIH0gfSk7XG4gICAgICAgICAgICAgICAgZXZlbnQudGFyZ2V0LnZhbHVlID0gXCJcIjtcbiAgICAgICAgICAgICAgfTtcbiAgICAgICAgICAgICAgcmVhZGVyLnJlYWRBc0RhdGFVUkwoZmlsZSk7XG4gICAgICAgICAgICAgIHJldHVybjtcbiAgICAgICAgICAgIH1cbiAgICAgICAgICAgIHZvaWQgZmlsZS50ZXh0KCkudGhlbigoanNvbikgPT4ge1xuICAgICAgICAgICAgICBvbkFjdGlvbih7IGNvbnRyb2xsZXJJZDogbGFuZGluZ0NvbnRyb2xsZXJJZCA/PyBcIlwiLCBhY3Rpb246IFwiaW1wb3J0U3BhY2VcIiwgYXJnczogeyBqc29uIH0gfSk7XG4gICAgICAgICAgICAgIGV2ZW50LnRhcmdldC52YWx1ZSA9IFwiXCI7XG4gICAgICAgICAgICB9KTtcbiAgICAgICAgICB9fVxuICAgICAgICAvPlxuICAgICAgICA8ZGl2IGNsYXNzTmFtZT1cIm1pbi1oLTAgZmxleC0xXCI+XG4gICAgICAgICAgPFNoZWxsRmF1bHRCb3VuZGFyeSBib3VuZGFyeUlkPVwic2Vzc2lvbi1jYW52YXNcIiBmYWxsYmFja0xhYmVsPXtzaGVsbExhYmVsKFwidWkuY29tbW9uLnJlbmRlckVycm9yXCIpfT5cbiAgICAgICAgICAgIDxBcHBcbiAgICAgICAgICAgIG1vZGVzPXttb2Rlcy5tYXAoKG1vZGUpID0+ICh7IGlkOiBtb2RlLmlkLCBsYWJlbDogcmVzb2x2ZUFwcExhYmVsKGFwcExhYmVsc092ZXJsYXksIFwibW9kZVwiLCBtb2RlLmlkLCByZXNvbHZlTWFuaWZlc3RMYWJlbChtb2RlLmxhYmVsLCB1aVRlcm1pbm9sb2d5LCB1aUxvY2FsZSkpLCBjaGlsZHJlbjogbnVsbCB9KSl9XG4gICAgICAgICAgICBhY3RpdmVNb2RlSWQ9e3Nlc3Npb24udmlld1N0YXRlLmFjdGl2ZU1vZGVJZCA/PyBtb2Rlc1swXT8uaWQgPz8gc2Vzc2lvbi5hcHAuaWR9XG4gICAgICAgICAgICBvbkFjdGl2ZU1vZGVDaGFuZ2U9e2FwcGx5TW9kZUNoYW5nZX1cbiAgICAgICAgICAgIGNocm9tZT17ZmFsc2V9XG4gICAgICAgICAgPlxuICAgICAgICAgICAgPE1vZGVcbiAgICAgICAgICAgICAgY2xhc3NOYW1lPVwiaC1mdWxsIHctZnVsbFwiXG4gICAgICAgICAgICAgIG1vYmlsZT17bW9iaWxlfVxuICAgICAgICAgICAgICB3aW5kb3dzPXttb2RlV2luZG93c31cbiAgICAgICAgICAgICAgbGF5b3V0PXtlZmZlY3RpdmVNb2RlTGF5b3V0fVxuICAgICAgICAgICAgICBhY3RpdmVXaW5kb3dJZD17YWN0aXZlV2luZG93SWR9XG4gICAgICAgICAgICAgIG9uQWN0aXZlV2luZG93Q2hhbmdlPXtoYW5kbGVBY3RpdmVXaW5kb3dDaGFuZ2V9XG4gICAgICAgICAgICAgIG9uTGF5b3V0Q2hhbmdlPXtoYW5kbGVNb2RlTGF5b3V0Q2hhbmdlfVxuICAgICAgICAgICAgICBvblRlbXBsYXRlRHJvcD17bW9iaWxlID8gdW5kZWZpbmVkIDogaGFuZGxlVGVtcGxhdGVEcm9wfVxuICAgICAgICAgICAgICBvbldpbmRvd0Nsb3NlPXsod2luZG93SWQpID0+IHtcbiAgICAgICAgICAgICAgICBub3RlU2hlbGxDb21tYW5kKFwic2hlbGwud2luZG93Q2xvc2VcIiwgc2hlbGxMYWJlbChcInVpLnNoZWxsQ29tbWFuZC53aW5kb3dDbG9zZVwiKSwgeyB3aW5kb3dJZCB9KTtcbiAgICAgICAgICAgICAgICBpZiAoc3R1ZGlvTW9kZSAmJiBwYW5lbD8uc3Bhd25lZEFwcHMuc29tZSgoZW50cnkpID0+IGVudHJ5LmlkID09PSB3aW5kb3dJZCkpIHtcbiAgICAgICAgICAgICAgICAgIGNvbnN0IGNsb3NlZFNwYXduZWQgPSBwYW5lbC5zcGF3bmVkQXBwcy5maW5kKChlbnRyeSkgPT4gZW50cnkuaWQgPT09IHdpbmRvd0lkKTtcbiAgICAgICAgICAgICAgICAgIGNvbnN0IG5leHRTcGF3bmVkID0gcGFuZWwuc3Bhd25lZEFwcHMuZmlsdGVyKChlbnRyeSkgPT4gZW50cnkuaWQgIT09IHdpbmRvd0lkKTtcbiAgICAgICAgICAgICAgICAgIHVwZGF0ZVNwYWNlUGFuZWwoYnVpbGRTcGFjZVBhbmVsU3RhdGUocGFuZWwucHJvZ3JhbXMsIG5leHRTcGF3bmVkLCBwYW5lbC5hY3RpdmVQYW5lbFRhYiwgbmV4dFNwYXduZWRbMF0/LmlkKSk7XG4gICAgICAgICAgICAgICAgICAvLyDwn6q277iPIENsb3NpbmcgYSBzcGF3bmVkIGFwcCdzIHdpbmRvdyB1c2VkIHRvIGxlYXZlIGl0cyBwbHVnaW4gaW5zdGFuY2UgcnVubmluZyBmb3JldmVyXG4gICAgICAgICAgICAgICAgICAvLyAoc2VlIFJFRFVDRS1ERU1PTlNUUkFUT1ItSURMRS1NRU1PUlktRk9PVFBSSU5UJ3MgZG9jdW1lbnRlZCB0ZWFyZG93biBnYXApIOKAlCB0aGUgcGFuZWxcbiAgICAgICAgICAgICAgICAgIC8vIGVudHJ5IHdhcyBkcm9wcGVkIGZyb20gdGhlIFVJLCBidXQgbm90aGluZyBldmVyIHRvbGQgdGhlIGd1ZXN0IHRvIGZyZWUgaXQuXG4gICAgICAgICAgICAgICAgICBpZiAoY2xvc2VkU3Bhd25lZCkge1xuICAgICAgICAgICAgICAgICAgICBjb25zdCBjbG9zZWRQbHVnaW4gPSBsb2FkZWRQbHVnaW5zLmZpbmQoKGVudHJ5KSA9PiBlbnRyeS5oYW5kbGUucGx1Z2luSWQgPT09IGNsb3NlZFNwYXduZWQucGx1Z2luSWQpPy5oYW5kbGU7XG4gICAgICAgICAgICAgICAgICAgIHZvaWQgY2xvc2VkUGx1Z2luPy5kZXN0cm95QXBwKGNsb3NlZFNwYXduZWQuaW5zdGFuY2VJZCkuY2F0Y2goKCkgPT4ge30pO1xuICAgICAgICAgICAgICAgICAgfVxuICAgICAgICAgICAgICAgIH1cbiAgICAgICAgICAgICAgICBjbGVhclBlbmRpbmdXb3JsZFByb2plY3Rpb24od2luZG93SWQpO1xuICAgICAgICAgICAgICAgIGRpc3BhdGNoKHtcbiAgICAgICAgICAgICAgICAgIHR5cGU6IFwiU0VUX0VYVFJBX1dJTkRPV19JTlNUQU5DRVNcIixcbiAgICAgICAgICAgICAgICAgIHZhbHVlOiAoY3VycmVudCkgPT4ge1xuICAgICAgICAgICAgICAgICAgICBjb25zdCBuZXh0ID0gY3VycmVudC5maWx0ZXIoKGVudHJ5KSA9PiBlbnRyeS5pZCAhPT0gd2luZG93SWQpO1xuICAgICAgICAgICAgICAgICAgICBleHRyYVdpbmRvd0luc3RhbmNlc1JlZi5jdXJyZW50ID0gbmV4dDtcbiAgICAgICAgICAgICAgICAgICAgcmV0dXJuIG5leHQ7XG4gICAgICAgICAgICAgICAgICB9LFxuICAgICAgICAgICAgICAgIH0pO1xuICAgICAgICAgICAgICAgIGRpc3BhdGNoKHtcbiAgICAgICAgICAgICAgICAgIHR5cGU6IFwiU0VUX1NIRUxMX0xBWU9VVFwiLFxuICAgICAgICAgICAgICAgICAgdmFsdWU6IChjdXJyZW50KSA9PiBjdXJyZW50ID8/IHJlc29sdmVGcmFtZXdvcmtMYXlvdXRTZWVkKHNlc3Npb24uYXBwLmRlZmF1bHRMYXlvdXQsIHNlc3Npb24uYXBwLndpbmRvd0tpbmRzLCBhcHBMYWJlbHNPdmVybGF5LCB1aVRlcm1pbm9sb2d5LCB1aUxvY2FsZSkubW9kZUxheW91dCxcbiAgICAgICAgICAgICAgICB9KTtcbiAgICAgICAgICAgICAgfX1cbiAgICAgICAgICAgIC8+XG4gICAgICAgICAgPC9BcHA+XG4gICAgICAgICAgPC9TaGVsbEZhdWx0Qm91bmRhcnk+XG4gICAgICAgIDwvZGl2PlxuICAgICAgPC9kaXY+XG4gICAgKTtcbiAgfSwgW2FjdGl2ZVdpbmRvd0lkLCBlZmZlY3RpdmVNb2RlTGF5b3V0LCBlcnJvciwgaGFuZGxlQWN0aXZlV2luZG93Q2hhbmdlLCBoYW5kbGVNb2RlTGF5b3V0Q2hhbmdlLCBoYW5kbGVUZW1wbGF0ZURyb3AsIGxvYWRlZFBsdWdpbnMsIG1vYmlsZSwgbW9kZVdpbmRvd3MsIG5hdmlnYXRlSGlzdG9yeSwgbm90ZVNoZWxsQ29tbWFuZCwgb25BY3Rpb24sIHBhbmVsLCBwbHVnaW5TdXBlcnZpc29yQnlJZCwgcHJpbWFyeVBsdWdpbklkLCByZWxvYWRQbHVnaW4sIHNlc3Npb24sIHNoZWxsUm91dGUsIHN0dWRpb01vZGUsIHVpTG9jYWxlLCB1aVRlcm1pbm9sb2d5LCB1cGRhdGVTcGFjZVBhbmVsLCBkaXNwYXRjaCwgdW5pbnN0YWxsUGx1Z2luXSk7XG5cbiAgY29uc3QgZm9vdGVySXRlbXMgPSB1c2VNZW1vKCgpOiBOYXZiYXJJdGVtW10gPT4ge1xuICAgIC8vIPCfj5vvuI8gTWl0IEJlc3RhbmQgQWdncmVnYXRvciBwYXJ0bmVyIGNyZWRpdHM6IGxlZnQgXCJFaW4gUHJvamVrdCB2b24gTFVIIHVuZCBVZEtcIiwgcmlnaHQgXCJHZWbDtnJkZXJ0IGR1cmNoIFp1a3VuZnQgQmF1XCIuXG4gICAgLy8gQSBzaW5nbGUgbWlkZGxlIGZsZXgtMSBmaWxsIHB1c2hlcyB0aGUgZnVuZGluZyBjcmVkaXQgdG8gdGhlIHRyYWlsaW5nIGVkZ2U7IGZpeGVkIGB3LWh1Z2VgIGdhcHMga2VlcCBlYWNoIGNyZWRpdFxuICAgIC8vIG9mZiB0aGUgZXhhY3QgY29ybmVyIHBpeGVsIHRoYXQgZmxvYXRpbmcgY29ybmVyIHBhbmVscyBhbHNvIGFuY2hvciB0byAoYSBzZWNvbmQgZmxleC0xIHdvdWxkIGNlbnRlciB0aGUgZnVuZGluZ1xuICAgIC8vIGNyZWRpdCB1bmRlciB0aGUgQ29tbWFuZCBvdmVybGF5OyBgdy1kb3VibGVgIHJlYWRzIGFzIGZsdXNoIGFnYWluc3QgdGhlIHRvZ2dsZSBncm91cCkuXG4gICAgLy8g8J+Tse+4jyBUaGUgdGhyZWUgdGFiIGJhcnMgaGF2ZSBubyBhbmNob3Igb24gbW9iaWxlIChhbGwgYW5jaG9ycyBtZXJnZSBpbnRvIHRoZSBtb2JpbGUgcGFuZWwpIOKAlCBvbmx5IHRoZSBjcmVkaXRzIHN0YXkuXG4gICAgY29uc3QgaXRlbXM6IE5hdmJhckl0ZW1bXSA9IG1vYmlsZVxuICAgICAgPyBbXVxuICAgICAgOiBbXG4gICAgICAgICAgeyBrZXk6IFwiYm90dG9tTGVmdFBhbmVsVGFic1wiLCBjb250ZW50OiA8UGFuZWxDaHJvbWVUYWJCYXIgYW5jaG9yPVwiYm90dG9tLWxlZnRcIiB7Li4uYnVpbGRQYW5lbFNlbGVjdGlvblByb3BzKFwiYm90dG9tLWxlZnRcIil9IC8+IH0sXG4gICAgICAgICAgeyBrZXk6IFwiYm90dG9tTWlkZGxlUGFuZWxUYWJzXCIsIGNlbnRlcmVkOiB0cnVlLCBjb250ZW50OiA8UGFuZWxDaHJvbWVUYWJCYXIgYW5jaG9yPVwiYm90dG9tLW1pZGRsZVwiIHsuLi5idWlsZFBhbmVsU2VsZWN0aW9uUHJvcHMoXCJib3R0b20tbWlkZGxlXCIpfSAvPiB9LFxuICAgICAgICBdO1xuICAgIGlmIChicmFuZD8uaWQgJiYgKEVOVFdFUkZFTl9NSVRfQkVTVEFORF9CUkFORF9JRFMgYXMgcmVhZG9ubHkgc3RyaW5nW10pLmluY2x1ZGVzKGJyYW5kLmlkKSkge1xuICAgICAgaXRlbXMucHVzaChcbiAgICAgICAgeyBrZXk6IFwiZm9vdGVyUHJvamVjdE9mR2FwXCIsIGNsYXNzTmFtZTogXCJ3LWh1Z2VcIiwgY29udGVudDogbnVsbCB9LFxuICAgICAgICBhUHJvamVjdE9mTHVoVWRrRm9vdGVySXRlbShcImFQcm9qZWN0T2ZMdWhVZGtcIiwgdWlMb2NhbGUsIG1vYmlsZSksXG4gICAgICAgIG5hdmJhckZpbGxJdGVtKFwiZm9vdGVyTGVhZGluZ0ZpbGxcIiksXG4gICAgICAgIGZ1bmRlZEJ5WnVrdW5mdEJhdUZvb3Rlckl0ZW0oXCJmdW5kZWRCeVp1a3VuZnRCYXVcIiwgdWlMb2NhbGUsIG1vYmlsZSksXG4gICAgICAgIHsga2V5OiBcImZvb3RlckZ1bmRlZEJ5R2FwXCIsIGNsYXNzTmFtZTogXCJ3LWh1Z2VcIiwgY29udGVudDogbnVsbCB9LFxuICAgICAgKTtcbiAgICB9IGVsc2Uge1xuICAgICAgaXRlbXMucHVzaChuYXZiYXJGaWxsSXRlbShcImZvb3RlckxlYWRpbmdGaWxsXCIpKTtcbiAgICB9XG4gICAgaWYgKCFtb2JpbGUpIGl0ZW1zLnB1c2goeyBrZXk6IFwiYm90dG9tUmlnaHRQYW5lbFRhYnNcIiwgY29udGVudDogPFBhbmVsQ2hyb21lVGFiQmFyIGFuY2hvcj1cImJvdHRvbS1yaWdodFwiIHsuLi5idWlsZFBhbmVsU2VsZWN0aW9uUHJvcHMoXCJib3R0b20tcmlnaHRcIil9IC8+IH0pO1xuICAgIHJldHVybiBpdGVtcztcbiAgfSwgW2JyYW5kPy5pZCwgYnVpbGRQYW5lbFNlbGVjdGlvblByb3BzLCBtb2JpbGUsIHVpTG9jYWxlXSk7XG5cbiAgY29uc3QgYnVpbGRQYW5lbFByb3BzID0gdXNlQ2FsbGJhY2soXG4gICAgKGFuY2hvcjogQW5jaG9yKSA9PiAoe1xuICAgICAgLi4uYnVpbGRQYW5lbFNlbGVjdGlvblByb3BzKGFuY2hvciksXG4gICAgICBzaXplOiBwYW5lbHNbYW5jaG9yXS5zaXplLFxuICAgICAgb25TaXplQ2hhbmdlOiAodmFsdWU6IG51bWJlcikgPT4gZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9QQU5FTF9TSVpFXCIsIGFuY2hvciwgdmFsdWUgfSksXG4gICAgICB0YWJCYXJIb3N0OiAoUEFORUxfVEFCX0JBUl9IT1NUU1thbmNob3JdID8gXCJjaHJvbWVcIiA6IFwicGFuZWxcIikgYXMgXCJwYW5lbFwiIHwgXCJjaHJvbWVcIixcbiAgICAgIHRyZWVPcGVuU3RhdGVzLFxuICAgICAgb25UcmVlT3BlblN0YXRlQ2hhbmdlOiAoaWQ6IHN0cmluZywgb3BlbjogYm9vbGVhbikgPT4gZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9UUkVFX09QRU5fU1RBVEVcIiwgaWQsIG9wZW4gfSksXG4gICAgfSksXG4gICAgW2J1aWxkUGFuZWxTZWxlY3Rpb25Qcm9wcywgcGFuZWxzLCB0cmVlT3BlblN0YXRlc10sXG4gICk7XG5cbiAgLy8gI3JlZ2lvbiDwn5SW77iPUmVhZGluZXNzQmVhY29uXG4gIC8qKiDwn5qm77iPIERldGVybWluaXN0aWMgRE9NIGJlYWNvbiBmb3IgaGVhZGxlc3Mgc21va2UgdGVzdHMgKGUuZy4gU3Rvcnlib29rJ3MgT1Mtc2hlbGwgcGx1Z2luLWJvb3QgbWF0cml4KVxuICAgKiB0byB3YWl0IG9uIGluc3RlYWQgb2Ygc2NyZWVuc2hvdHMvdGltZW91dHMg4oCUIHNldCBvbmNlIGEgc2Vzc2lvbiByZXNvbHZlcyBvciBlcnJvcnMsIGNsZWFyZWQgb24gdW5tb3VudC4gKi9cbiAgdXNlRWZmZWN0KCgpID0+IHtcbiAgICBjb25zdCByb290ID0gZG9jdW1lbnQuZG9jdW1lbnRFbGVtZW50O1xuICAgIGNvbnN0IGJlYWNvbklkID0gcGx1Z2luRmlsdGVyID8/IFwidW5rbm93blwiO1xuICAgIGNvbnN0IG5vdEZvdW5kID0gc3R1ZGlvTW9kZSAmJiBzaGVsbFJvdXRlLmtpbmQgPT09IFwibm90Rm91bmRcIjtcbiAgICBpZiAobm90Rm91bmQpIHtcbiAgICAgIHJvb3QuZGF0YXNldC5zZW1pb09zTm90Rm91bmQgPSBiZWFjb25JZDtcbiAgICAgIGRlbGV0ZSByb290LmRhdGFzZXQuc2VtaW9Pc1JlYWR5O1xuICAgICAgZGVsZXRlIHJvb3QuZGF0YXNldC5zZW1pb09zRXJyb3I7XG4gICAgfSBlbHNlIGlmIChlcnJvcikge1xuICAgICAgcm9vdC5kYXRhc2V0LnNlbWlvT3NFcnJvciA9IGJlYWNvbklkO1xuICAgICAgZGVsZXRlIHJvb3QuZGF0YXNldC5zZW1pb09zUmVhZHk7XG4gICAgICBkZWxldGUgcm9vdC5kYXRhc2V0LnNlbWlvT3NOb3RGb3VuZDtcbiAgICB9IGVsc2UgaWYgKHNlc3Npb24pIHtcbiAgICAgIHJvb3QuZGF0YXNldC5zZW1pb09zUmVhZHkgPSBiZWFjb25JZDtcbiAgICAgIGRlbGV0ZSByb290LmRhdGFzZXQuc2VtaW9Pc0Vycm9yO1xuICAgICAgZGVsZXRlIHJvb3QuZGF0YXNldC5zZW1pb09zTm90Rm91bmQ7XG4gICAgfVxuICAgIHJldHVybiAoKSA9PiB7XG4gICAgICBkZWxldGUgcm9vdC5kYXRhc2V0LnNlbWlvT3NSZWFkeTtcbiAgICAgIGRlbGV0ZSByb290LmRhdGFzZXQuc2VtaW9Pc0Vycm9yO1xuICAgICAgZGVsZXRlIHJvb3QuZGF0YXNldC5zZW1pb09zTm90Rm91bmQ7XG4gICAgfTtcbiAgfSwgW3Nlc3Npb24sIGVycm9yLCBwbHVnaW5GaWx0ZXIsIHNoZWxsUm91dGUua2luZCwgc3R1ZGlvTW9kZV0pO1xuICAvLyAjZW5kcmVnaW9uIPCflJbvuI9SZWFkaW5lc3NCZWFjb25cblxuICAvLyNyZWdpb24g8J+Wse+4j1NoZWxsQ29udGV4dE1lbnVcbiAgLyoqIPCflrHvuI8gRGlzcGF0Y2ggc2luayBmb3IgdGhlIHNoZWxsIGZhbGxiYWNrIG1lbnUncyBgQ29udGV4dE1lbnVJdGVtU3BlY2BzIChzZWVcbiAgICogYGJ1aWxkU2hlbGxDb250ZXh0TWVudUl0ZW1zYCkg4oCUIGludGVyY2VwdHMgdGhlIHR3byByZXNlcnZlZCBpZHMgdGhlIGJ1aWxkZXIgZW1pdHMgaW4gcGxhY2Ugb2YgYVxuICAgKiByZWFsIGRpc3BhdGNoIChgXCJzaGVsbC5vcGVuQWN0aW9uUGFuZVwiYC9gXCJzaGVsbC5vcGVuUGFsZXR0ZVwiYCkgYW5kIGZvcndhcmRzIGV2ZXJ5dGhpbmcgZWxzZSB0b1xuICAgKiBgb25BY3Rpb25gLCBtaXJyb3JpbmcgdGhlIGNvbW1hbmQgcGFsZXR0ZSdzIG93biBhcmctY2FycnlpbmcgcmVkaXJlY3QuICovXG4gIGNvbnN0IGRpc3BhdGNoU2hlbGxNZW51QWN0aW9uID0gdXNlQ2FsbGJhY2soXG4gICAgKGFjdGlvbjogc3RyaW5nLCBhcmdzPzogUmVjb3JkPHN0cmluZywgdW5rbm93bj4pID0+IHtcbiAgICAgIGlmICghc2Vzc2lvbikgcmV0dXJuO1xuICAgICAgaWYgKGFjdGlvbiA9PT0gXCJzaGVsbC5vcGVuQWN0aW9uUGFuZVwiKSB7XG4gICAgICAgIGNvbnN0IHdpbmRvd0tpbmQgPSBzZXNzaW9uLmFwcC53aW5kb3dLaW5kcy5maW5kKChraW5kKSA9PiBraW5kLmlkID09PSBhY3RpdmVXaW5kb3dJZCkgPz8gc2Vzc2lvbi5hcHAud2luZG93S2luZHNbMF07XG4gICAgICAgIGNvbnN0IGFjdGlvbklkID0gdHlwZW9mIGFyZ3M/LmFjdGlvbklkID09PSBcInN0cmluZ1wiID8gYXJncy5hY3Rpb25JZCA6IHVuZGVmaW5lZDtcbiAgICAgICAgaWYgKCF3aW5kb3dLaW5kIHx8ICFhY3Rpb25JZCkgcmV0dXJuO1xuICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0FDVElWRV9XSU5ET1dfSURcIiwgdmFsdWU6IHdpbmRvd0tpbmQuaWQgfSk7XG4gICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfQUNUSU9OX1BBTkVfRk9MREVEXCIsIHdpbmRvd0lkOiB3aW5kb3dLaW5kLmlkLCB2YWx1ZTogZmFsc2UgfSk7XG4gICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfQUNUSU9OX1BBTkVfRVhQQU5ERURcIiwgd2luZG93SWQ6IHdpbmRvd0tpbmQuaWQsIHZhbHVlOiBhY3Rpb25JZCB9KTtcbiAgICAgICAgcmV0dXJuO1xuICAgICAgfVxuICAgICAgaWYgKGFjdGlvbiA9PT0gXCJzaGVsbC5vcGVuUGFsZXR0ZVwiKSB7XG4gICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfU0VBUkNIX09QRU5cIiwgdmFsdWU6IHRydWUgfSk7XG4gICAgICAgIHJldHVybjtcbiAgICAgIH1cbiAgICAgIG9uQWN0aW9uKHsgY29udHJvbGxlcklkOiBzZXNzaW9uLmFwcC5jb250cm9sbGVySWQsIGFjdGlvbiB9KTtcbiAgICB9LFxuICAgIFtzZXNzaW9uLCBhY3RpdmVXaW5kb3dJZCwgb25BY3Rpb24sIGRpc3BhdGNoXSxcbiAgKTtcblxuICAvKiog8J+Wse+4jyBCdWlsZHMgdGhlIHNoZWxsLWxldmVsIGZhbGxiYWNrIG1lbnU6IHRoZSBhY3RpdmUgd2luZG93J3MgZGVjbGFyZWQgYWN0aW9ucyAodW5kby9yZWRvLCB2aWV3XG4gICAqIGFjdGlvbnMsIC4uLikgcGx1cyBhIGNvbW1hbmQtcGFsZXR0ZSBvcGVuZXIg4oCUIHNob3duIGZvciBhbnkgcmlnaHQtY2xpY2sgbm8gaW5uZXIgc3VyZmFjZSBjbGFpbWVkXG4gICAqICh3aW5kb3cgYmFja2dyb3VuZCwgZW1wdHkgcGFuZWwvbmF2YmFyL2Zvb3RlciBzcGFjZSwgYW4gYXBwIHdpdGggbm8gc2NlbmUgYXQgYWxsKS4gQXJnLWNhcnJ5aW5nXG4gICAqIGFjdGlvbnMgcm91dGUgdGhyb3VnaCB0aGUgcmVzZXJ2ZWQgYFwic2hlbGwub3BlbkFjdGlvblBhbmVcImAgaWQgKHBhcml0eSB3aXRoIHRoZSB3Z3B1IHNoZWxsJ3NcbiAgICogYGJ1aWxkX3NoZWxsX2NvbnRleHRfbWVudV9zcGVjc2ApLCB0aGUgd2hvbGUgc3BlYyBsaXN0IHJ1bnMgdGhyb3VnaCBgb3JnYW5pemVDb250ZXh0TWVudWAsIHRoZW5cbiAgICogYG1hcENvbnRleHRNZW51U3BlY3NgIGJpbmRzIGl0IHRvIGBkaXNwYXRjaFNoZWxsTWVudUFjdGlvbmAuICovXG4gIGNvbnN0IGJ1aWxkU2hlbGxDb250ZXh0TWVudUl0ZW1zID0gdXNlQ2FsbGJhY2soKCk6IENvbnRleHRNZW51SXRlbVtdID0+IHtcbiAgICBpZiAoIXNlc3Npb24pIHJldHVybiBbXTtcbiAgICBjb25zdCB3aW5kb3dLaW5kID0gc2Vzc2lvbi5hcHAud2luZG93S2luZHMuZmluZCgoa2luZCkgPT4ga2luZC5pZCA9PT0gYWN0aXZlV2luZG93SWQpID8/IHNlc3Npb24uYXBwLndpbmRvd0tpbmRzWzBdO1xuICAgIGNvbnN0IHNwZWNzOiBDb250ZXh0TWVudUl0ZW1TcGVjW10gPSBbXTtcbiAgICBjb25zdCBjYXRlZ29yeUJ5QWN0aW9uSWQgPSBuZXcgTWFwPHN0cmluZywgc3RyaW5nPigpO1xuICAgIGlmICh3aW5kb3dLaW5kKSB7XG4gICAgICBmb3IgKGNvbnN0IGFjdGlvbiBvZiByZXNvbHZlV2luZG93QWN0aW9ucyhzZXNzaW9uLmFwcCwgd2luZG93S2luZCkpIHtcbiAgICAgICAgLy8g8J+nue+4jyBTYW1lIGN1cmF0aW9uIGFzIHRoZSBjb21tYW5kIHBhbGV0dGUgKGBpZiAoIWFjdGlvbi5pblBhbGV0dGUpIGNvbnRpbnVlYCkg4oCUIG1vc3QgYXBwc1xuICAgICAgICAvLyBkZWNsYXJlIGludGVybmFsL3BvaW50ZXItdHJhY2tpbmcgdmlldyBhY3Rpb25zICh3b3JsZEhvdmVyLCBlbmdhZ2VtZW50SW5wdXQsIC4uLikgYXMgd2luZG93XG4gICAgICAgIC8vIGFjdGlvbnMgcHVyZWx5IGZvciBkaXNwYXRjaCBwbHVtYmluZzsgb25seSBwYWxldHRlLXdvcnRoeSBvbmVzIGJlbG9uZyBpbiBhIHVzZXItZmFjaW5nIG1lbnUuXG4gICAgICAgIGlmICghYWN0aW9uLmluUGFsZXR0ZSkgY29udGludWU7XG4gICAgICAgIGNvbnN0IGFyZ0NhcnJ5aW5nID0gYWN0aW9uUmVxdWlyZXNTdGFnZWRGb3JtKGFjdGlvbik7XG4gICAgICAgIGNhdGVnb3J5QnlBY3Rpb25JZC5zZXQoYWN0aW9uLmlkLCBhY3Rpb25DYXRlZ29yeUlkKGFjdGlvbikpO1xuICAgICAgICBzcGVjcy5wdXNoKHtcbiAgICAgICAgICBpZDogYHNoZWxsLW1lbnUuYWN0aW9uLiR7YWN0aW9uLmlkfWAsXG4gICAgICAgICAgbGFiZWw6IHJlc29sdmVBcHBMYWJlbChhcHBMYWJlbHNPdmVybGF5LCBcImFjdGlvblwiLCBhY3Rpb24uaWQsIHJlc29sdmVNYW5pZmVzdExhYmVsKGFjdGlvbi5sYWJlbCwgdWlUZXJtaW5vbG9neSwgdWlMb2NhbGUpKSArIChhcmdDYXJyeWluZyA/IFwi4oCmXCIgOiBcIlwiKSxcbiAgICAgICAgICBpY29uOiBhY3Rpb24uaWNvbklkLFxuICAgICAgICAgIHNob3J0Y3V0OiBhY3Rpb24ua2V5cyA/PyBrZXlzQnlBY3Rpb25JZC5nZXQoYWN0aW9uLmlkKSxcbiAgICAgICAgICBkZXN0cnVjdGl2ZTogYWN0aW9uLmtpbmQgPT09IFwib3BlcmF0aW9uXCIgJiYgYWN0aW9uLmlkLnRvTG93ZXJDYXNlKCkuaW5jbHVkZXMoXCJkZWxldGVcIiksXG4gICAgICAgICAgYWN0aW9uOiBhcmdDYXJyeWluZyA/IFwic2hlbGwub3BlbkFjdGlvblBhbmVcIiA6IGFjdGlvbi5pZCxcbiAgICAgICAgICBhcmdzOiBhcmdDYXJyeWluZyA/IHsgYWN0aW9uSWQ6IGFjdGlvbi5pZCB9IDogdW5kZWZpbmVkLFxuICAgICAgICB9KTtcbiAgICAgIH1cbiAgICB9XG4gICAgaWYgKHNwZWNzLmxlbmd0aCA+IDApIHNwZWNzLnB1c2goeyBpZDogXCJzaGVsbC1tZW51LnNlcGFyYXRvclwiLCBzZXBhcmF0b3I6IHRydWUgfSk7XG4gICAgc3BlY3MucHVzaCh7XG4gICAgICBpZDogXCJzaGVsbC5vcGVuUGFsZXR0ZVwiLFxuICAgICAgbGFiZWw6IHNoZWxsTGFiZWwoXCJ1aS5zZWFyY2gudG9nZ2xlXCIpLFxuICAgICAgaWNvbjogXCJzZWFyY2hcIixcbiAgICAgIGFjdGlvbjogXCJzaGVsbC5vcGVuUGFsZXR0ZVwiLFxuICAgIH0pO1xuICAgIGNvbnN0IG9yZ2FuaXplZCA9IG9yZ2FuaXplQ29udGV4dE1lbnUoc3BlY3MsIChpZCkgPT4gY2F0ZWdvcnlCeUFjdGlvbklkLmdldChpZCkpO1xuICAgIHJldHVybiBtYXBDb250ZXh0TWVudVNwZWNzKG9yZ2FuaXplZCwgZGlzcGF0Y2hTaGVsbE1lbnVBY3Rpb24sIGtleXNCeUFjdGlvbklkKTtcbiAgfSwgW3Nlc3Npb24sIGFjdGl2ZVdpbmRvd0lkLCBhcHBMYWJlbHNPdmVybGF5LCBrZXlzQnlBY3Rpb25JZCwgZGlzcGF0Y2hTaGVsbE1lbnVBY3Rpb24sIHVpVGVybWlub2xvZ3ksIHVpTG9jYWxlXSk7XG5cbiAgdXNlRWZmZWN0KCgpID0+IHtcbiAgICBjb25zdCBoYW5kbGVDb250ZXh0TWVudSA9IChldmVudDogTW91c2VFdmVudCkgPT4ge1xuICAgICAgaWYgKGlzQ29udGV4dE1lbnVQb2ludGVyVGFyZ2V0KGV2ZW50LnRhcmdldCkpIHJldHVybjtcbiAgICAgIGNvbnN0IGl0ZW1zID0gYnVpbGRTaGVsbENvbnRleHRNZW51SXRlbXMoKTtcbiAgICAgIGlmIChpdGVtcy5sZW5ndGggPT09IDApIHJldHVybjtcbiAgICAgIGV2ZW50LnByZXZlbnREZWZhdWx0KCk7XG4gICAgICBzZXRTaGVsbENvbnRleHRNZW51KHsgeDogZXZlbnQuY2xpZW50WCwgeTogZXZlbnQuY2xpZW50WSwgaXRlbXMgfSk7XG4gICAgfTtcbiAgICB3aW5kb3cuYWRkRXZlbnRMaXN0ZW5lcihcImNvbnRleHRtZW51XCIsIGhhbmRsZUNvbnRleHRNZW51KTtcbiAgICByZXR1cm4gKCkgPT4gd2luZG93LnJlbW92ZUV2ZW50TGlzdGVuZXIoXCJjb250ZXh0bWVudVwiLCBoYW5kbGVDb250ZXh0TWVudSk7XG4gIH0sIFtidWlsZFNoZWxsQ29udGV4dE1lbnVJdGVtc10pO1xuICAvLyNlbmRyZWdpb24g8J+Wse+4j1NoZWxsQ29udGV4dE1lbnVcblxuICByZXR1cm4gKFxuICAgIDxTZXRXaW5kb3dUaXRsZUNvbnRleHQuUHJvdmlkZXIgdmFsdWU9e3NldFdpbmRvd1RpdGxlfT5cbiAgICA8U2V0V2luZG93SWNvbkNvbnRleHQuUHJvdmlkZXIgdmFsdWU9e3NldFdpbmRvd0ljb259PlxuICAgIDxBcHBLZXliaW5kaW5nc0NvbnRleHQuUHJvdmlkZXIgdmFsdWU9e2tleXNCeUFjdGlvbklkfT5cbiAgICA8VWlLZXliaW5kaW5nc1Byb3ZpZGVyIGJpbmRpbmdzPXtjb250cm9sS2V5YmluZGluZ3N9PlxuICAgIDxQbHVnaW5TdXJmYWNlQWN0aW9uc0NvbnRleHQuUHJvdmlkZXIgdmFsdWU9e3JlcXVlc3RDb250ZXh0TWVudX0+XG4gICAgPFNoZWxsQ29udGV4dE1lbnVGYWxsYmFja0NvbnRleHQuUHJvdmlkZXIgdmFsdWU9e2J1aWxkU2hlbGxDb250ZXh0TWVudUl0ZW1zfT5cbiAgICA8U2hlbGxGYXVsdEJvdW5kYXJ5IGJvdW5kYXJ5SWQ9XCJzaGVsbC1yb290XCIgZmFsbGJhY2tMYWJlbD17c2hlbGxMYWJlbChcInVpLmNvbW1vbi5yZW5kZXJFcnJvclwiKX0+XG4gICAgPFVJRmluZFByb3ZpZGVyPlxuICAgICAgPExldmVsUHJvdmlkZXIgbGV2ZWw9XCJiYXNlXCI+XG4gICAgICAgIDxkaXYgY2xhc3NOYW1lPVwiZmxleCBoLXNjcmVlbiBtaW4taC0wIHctc2NyZWVuIGZsZXgtY29sIGJnLXRyYW5zcGFyZW50XCIgZGF0YS1sZXZlbD1cImJhc2VcIj5cbiAgICAgICAgICA8UGFuZWxEb2NrUHJvdmlkZXIgZG9jaz17ZG9ja30gb25UYWJEb2NrRHJvcD17aGFuZGxlVGFiRG9ja0Ryb3B9IG9uVHJlZVVuaXREb2NrRHJvcD17aGFuZGxlVHJlZVVuaXREb2NrRHJvcH0+XG4gICAgICAgICAgICA8TGF5b3V0XG4gICAgICAgICAgICAgIG1vYmlsZT17bW9iaWxlfVxuICAgICAgICAgICAgICBtb2JpbGVQYW5lbD17bW9iaWxlUGFuZWx9XG4gICAgICAgICAgICAgIG5hdmJhcj17PE5hdmJhciBpdGVtcz17bmF2YmFySXRlbXN9IHNob3dGdWxsc2NyZWVuVG9nZ2xlPXshbW9iaWxlfSAvPn1cbiAgICAgICAgICAgICAgc3VibmF2YmFyPXtcbiAgICAgICAgICAgICAgICBhY3RpdmVUdXRvcmlhbCA/IChcbiAgICAgICAgICAgICAgICAgIDxUdXRvcmlhbEJhclxuICAgICAgICAgICAgICAgICAgICB0aXRsZT17cmVzb2x2ZU1hbmlmZXN0TGFiZWwoYWN0aXZlVHV0b3JpYWwudGl0bGUsIHVpVGVybWlub2xvZ3ksIHVpTG9jYWxlKX1cbiAgICAgICAgICAgICAgICAgICAgZHVyYXRpb25Ncz17YWN0aXZlVHV0b3JpYWwuZHVyYXRpb25Nc31cbiAgICAgICAgICAgICAgICAgICAgcGxheWluZz17dHV0b3JpYWxQbGF5aW5nfVxuICAgICAgICAgICAgICAgICAgICByYXRlPXt0dXRvcmlhbFJhdGV9XG4gICAgICAgICAgICAgICAgICAgIG11dGVkPXt0dXRvcmlhbE11dGVkfVxuICAgICAgICAgICAgICAgICAgICBjYXB0aW9uc09uPXt0dXRvcmlhbENhcHRpb25zT259XG4gICAgICAgICAgICAgICAgICAgIHJlY29yZGluZz17dHV0b3JpYWxSZWNvcmRpbmd9XG4gICAgICAgICAgICAgICAgICAgIHJlY29yZEF2YWlsYWJsZT17dHV0b3JpYWxSZWNvcmRlckF2YWlsYWJsZX1cbiAgICAgICAgICAgICAgICAgICAgY2hhcHRlcnM9e3R1dG9yaWFsQ2hhcHRlck1hcmtlcnN9XG4gICAgICAgICAgICAgICAgICAgIGNsb2NrPXt0dXRvcmlhbENsb2NrfVxuICAgICAgICAgICAgICAgICAgICBvblBsYXlQYXVzZT17cGxheVBhdXNlVHV0b3JpYWx9XG4gICAgICAgICAgICAgICAgICAgIG9uU3RvcD17c3RvcFR1dG9yaWFsfVxuICAgICAgICAgICAgICAgICAgICBvblNlZWs9e3NlZWtUdXRvcmlhbH1cbiAgICAgICAgICAgICAgICAgICAgb25SYXRlQ2hhbmdlPXsodmFsdWUpID0+IGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfVFVUT1JJQUxfUkFURVwiLCB2YWx1ZSB9KX1cbiAgICAgICAgICAgICAgICAgICAgb25NdXRlZENoYW5nZT17KHZhbHVlKSA9PiBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1RVVE9SSUFMX01VVEVEXCIsIHZhbHVlIH0pfVxuICAgICAgICAgICAgICAgICAgICBvbkNhcHRpb25zQ2hhbmdlPXsodmFsdWUpID0+IGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfVFVUT1JJQUxfQ0FQVElPTlNcIiwgdmFsdWUgfSl9XG4gICAgICAgICAgICAgICAgICAgIG9uUmVjb3JkVG9nZ2xlPXt0b2dnbGVUdXRvcmlhbFJlY29yZGluZ31cbiAgICAgICAgICAgICAgICAgICAgb25BZGRDaGFwdGVyPXthZGRUdXRvcmlhbENoYXB0ZXJ9XG4gICAgICAgICAgICAgICAgICAvPlxuICAgICAgICAgICAgICAgICkgOiB1bmRlZmluZWRcbiAgICAgICAgICAgICAgfVxuICAgICAgICAgICAgICBmb290ZXI9ezxGb290ZXIgaXRlbXM9e2Zvb3Rlckl0ZW1zfSAvPn1cbiAgICAgICAgICAgICAgcGFuZWxzPXtPYmplY3QuZnJvbUVudHJpZXMoQU5DSE9SUy5tYXAoKGFuY2hvcikgPT4gW2FuY2hvciwgYnVpbGRQYW5lbFByb3BzKGFuY2hvcildKSkgYXMgUmVjb3JkPEFuY2hvciwgUmV0dXJuVHlwZTx0eXBlb2YgYnVpbGRQYW5lbFByb3BzPj59XG4gICAgICAgICAgICAgIGNhbnZhc1N0YXR1cz17c2hlbGxQbHVnaW5DYW52YXNTdGF0dXN9XG4gICAgICAgICAgICAgIGNhbnZhc1NrZWxldG9uPXs8Q2FudmFzU2tlbGV0b24gbGFiZWw9e3NoZWxsTGFiZWwoXCJ1aS5jb21tb24ubG9hZGluZ1BsdWdpbnNcIil9IC8+fVxuICAgICAgICAgICAgICBjYW52YXM9e1xuICAgICAgICAgICAgICAgIDxTaGVsbEZhdWx0Qm91bmRhcnkgYm91bmRhcnlJZD1cInJvdXRlLWNhbnZhc1wiIGZhbGxiYWNrTGFiZWw9e3NoZWxsTGFiZWwoXCJ1aS5jb21tb24ucmVuZGVyRXJyb3JcIil9PlxuICAgICAgICAgICAgICAgICAge2NhbnZhc31cbiAgICAgICAgICAgICAgICA8L1NoZWxsRmF1bHRCb3VuZGFyeT5cbiAgICAgICAgICAgICAgfVxuICAgICAgICAgICAgLz5cbiAgICAgICAgICA8L1BhbmVsRG9ja1Byb3ZpZGVyPlxuICAgICAgICA8L2Rpdj5cbiAgICAgICAgPFVJU2VhcmNoIGl0ZW1zPXtzZWFyY2hJdGVtc30gb3Blbj17c2VhcmNoT3Blbn0gb25PcGVuQ2hhbmdlPXsodmFsdWUpID0+IGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfU0VBUkNIX09QRU5cIiwgdmFsdWUgfSl9IC8+XG4gICAgICAgIDxVSUZpbmQgb3Blbj17ZmluZE9wZW59IG9uT3BlbkNoYW5nZT17KHZhbHVlKSA9PiBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0ZJTkRfT1BFTlwiLCB2YWx1ZSB9KX0gLz5cbiAgICAgICAgPFRleHRTZWxlY3Rpb25Db250ZXh0TWVudUhvc3QgLz5cbiAgICAgICAgPENvbnRleHRNZW51Q29udHJvbGxlclxuICAgICAgICAgIHRpdGxlPXtzaGVsbENvbnRleHRNZW51VGl0bGVMYWJlbH1cbiAgICAgICAgICBvcGVuPXtzaGVsbENvbnRleHRNZW51ICE9IG51bGx9XG4gICAgICAgICAgcG9zaXRpb249e3NoZWxsQ29udGV4dE1lbnV9XG4gICAgICAgICAgaXRlbXM9e3NoZWxsQ29udGV4dE1lbnU/Lml0ZW1zID8/IFtdfVxuICAgICAgICAgIG9uT3BlbkNoYW5nZT17KG9wZW4pID0+IHtcbiAgICAgICAgICAgIGlmICghb3Blbikgc2V0U2hlbGxDb250ZXh0TWVudShudWxsKTtcbiAgICAgICAgICB9fVxuICAgICAgICAvPlxuICAgICAgICB7c2Vzc2lvbiAmJiBhY3RpdmVJbnRyb2R1Y3Rpb24gJiYgaW50cm9kdWN0aW9uU3RlcEluZGV4ICE9IG51bGwgJiYgKFxuICAgICAgICAgIDxVSUludHJvZHVjdGlvblxuICAgICAgICAgICAgaW50cm9kdWN0aW9uPXticmFuZD8uaW50cm9kdWN0aW9uID8/IHJlc29sdmVJbnRyb2R1Y3Rpb25EZWZpbml0aW9uKGFjdGl2ZUludHJvZHVjdGlvbiwgYXBwTGFiZWxzT3ZlcmxheSwgdWlUZXJtaW5vbG9neSwgdWlMb2NhbGUpfVxuICAgICAgICAgICAgc3RlcEluZGV4PXtpbnRyb2R1Y3Rpb25TdGVwSW5kZXh9XG4gICAgICAgICAgICBjb21wbGV0ZWRJbnRlcmFjdGlvbkluZGljZXM9e2ludHJvZHVjdGlvbkNvbXBsZXRlZEludGVyYWN0aW9uc31cbiAgICAgICAgICAgIG9uU3RlcEluZGV4Q2hhbmdlPXsodmFsdWUpID0+IGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfSU5UUk9EVUNUSU9OX1NURVBcIiwgdmFsdWUgfSl9XG4gICAgICAgICAgICBvbkRpc21pc3M9e2Rpc21pc3NJbnRyb2R1Y3Rpb259XG4gICAgICAgICAgLz5cbiAgICAgICAgKX1cbiAgICAgICAge2FjdGl2ZVR1dG9yaWFsICYmIChcbiAgICAgICAgICA8PlxuICAgICAgICAgICAgPFR1dG9yaWFsQ2FwdGlvbnNIb3N0IHR1dG9yaWFsPXthY3RpdmVUdXRvcmlhbH0gY2xvY2s9e3R1dG9yaWFsQ2xvY2t9IGNhcHRpb25zT249e3R1dG9yaWFsQ2FwdGlvbnNPbn0gdGVybWlub2xvZ3k9e3VpVGVybWlub2xvZ3l9IGxvY2FsZT17dWlMb2NhbGV9IC8+XG4gICAgICAgICAgICA8VHV0b3JpYWxWaWRlb092ZXJsYXlIb3N0IHR1dG9yaWFsPXthY3RpdmVUdXRvcmlhbH0gY2xvY2s9e3R1dG9yaWFsQ2xvY2t9IG11dGVkPXt0dXRvcmlhbE11dGVkfSBwbGF5aW5nPXt0dXRvcmlhbFBsYXlpbmd9IHJhdGU9e3R1dG9yaWFsUmF0ZX0gLz5cbiAgICAgICAgICAgIDxUdXRvcmlhbEdob3N0UG9pbnRlckhvc3QgdHV0b3JpYWw9e2FjdGl2ZVR1dG9yaWFsfSBjbG9jaz17dHV0b3JpYWxDbG9ja30gLz5cbiAgICAgICAgICA8Lz5cbiAgICAgICAgKX1cbiAgICAgICAge3Nlc3Npb24gJiZcbiAgICAgICAgICBvdmVybGF5RGlhbG9nICYmXG4gICAgICAgICAgKCgpID0+IHtcbiAgICAgICAgICAgIGNvbnN0IGRpYWxvZyA9IHNlc3Npb24uYXBwLmRpYWxvZ3M/LmZpbmQoKGVudHJ5KSA9PiBlbnRyeS5pZCA9PT0gb3ZlcmxheURpYWxvZy5kaWFsb2dJZCk7XG4gICAgICAgICAgICBpZiAoIWRpYWxvZykgcmV0dXJuIG51bGw7XG4gICAgICAgICAgICByZXR1cm4gKFxuICAgICAgICAgICAgICA8VUlEaWFsb2dcbiAgICAgICAgICAgICAgICBkaWFsb2c9e3Jlc29sdmVEaWFsb2dEZWZpbml0aW9uKGRpYWxvZywgYXBwTGFiZWxzT3ZlcmxheSwgdWlUZXJtaW5vbG9neSwgdWlMb2NhbGUpfVxuICAgICAgICAgICAgICAgIHNlZWRBcmdzPXtvdmVybGF5RGlhbG9nLnNlZWRBcmdzfVxuICAgICAgICAgICAgICAgIHJlbmRlckZpZWxkPXsoZGVmLCB2YWx1ZSwgb25DaGFuZ2UpID0+IHJlbmRlclN0YWdlZEFyZ0NvbnRyb2woZGVmLCB2YWx1ZSwgb25DaGFuZ2UpfVxuICAgICAgICAgICAgICAgIG9uU3VibWl0PXsoYXJncykgPT4ge1xuICAgICAgICAgICAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9ESUFMT0dcIiwgdmFsdWU6IG51bGwgfSk7XG4gICAgICAgICAgICAgICAgICBvbkFjdGlvbih7IGNvbnRyb2xsZXJJZDogc2Vzc2lvbi5hcHAuY29udHJvbGxlcklkLCBhY3Rpb246IGRpYWxvZy5zdWJtaXRBY3Rpb24sIGFyZ3MgfSk7XG4gICAgICAgICAgICAgICAgfX1cbiAgICAgICAgICAgICAgICBvbkNhbmNlbD17KCkgPT4ge1xuICAgICAgICAgICAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9ESUFMT0dcIiwgdmFsdWU6IG51bGwgfSk7XG4gICAgICAgICAgICAgICAgICBpZiAoZGlhbG9nLmNhbmNlbEFjdGlvbikgb25BY3Rpb24oeyBjb250cm9sbGVySWQ6IHNlc3Npb24uYXBwLmNvbnRyb2xsZXJJZCwgYWN0aW9uOiBkaWFsb2cuY2FuY2VsQWN0aW9uIH0pO1xuICAgICAgICAgICAgICAgIH19XG4gICAgICAgICAgICAgIC8+XG4gICAgICAgICAgICApO1xuICAgICAgICAgIH0pKCl9XG4gICAgICA8L0xldmVsUHJvdmlkZXI+XG4gICAgPC9VSUZpbmRQcm92aWRlcj5cbiAgICA8L1NoZWxsRmF1bHRCb3VuZGFyeT5cbiAgICA8L1NoZWxsQ29udGV4dE1lbnVGYWxsYmFja0NvbnRleHQuUHJvdmlkZXI+XG4gICAgPC9QbHVnaW5TdXJmYWNlQWN0aW9uc0NvbnRleHQuUHJvdmlkZXI+XG4gICAgPC9VaUtleWJpbmRpbmdzUHJvdmlkZXI+XG4gICAgPC9BcHBLZXliaW5kaW5nc0NvbnRleHQuUHJvdmlkZXI+XG4gICAgPC9TZXRXaW5kb3dJY29uQ29udGV4dC5Qcm92aWRlcj5cbiAgICA8L1NldFdpbmRvd1RpdGxlQ29udGV4dC5Qcm92aWRlcj5cbiAgKTtcbn1cbi8vI2VuZHJlZ2lvbiBGcmFtZXdvcmtPc1NoZWxsXG4iXSwiZmlsZSI6Ii9Vc2Vycy91ZWxpL0RvY3VtZW50cy9zZW1pby/wn6ew77iPZnJhbWV3b3JrL/Cfm43vuI9wcm9kdWN0cy/wn5K777iPb3Mv8J+UqO+4j21vZHVsZXMv8J+Tuu+4j3JlbmRlcmVyL/Cfp5HvuI/igI3wn46o77iPZW5naW5lL/Cfp7HvuI9lbGVtZW50cy9TaGVsbEhvc3Qv8J+fpu+4j2NvbXBvbmVudC50c3gifQ==