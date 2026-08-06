import { createHotContext as __vite__createHotContext } from "/@vite/client";import.meta.hot = __vite__createHotContext("/@fs/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx");import.meta.env = {"BASE_URL": "/", "DEV": true, "MODE": "development", "PROD": false, "SSR": false, "VITE_SEMIO_APP_ID": "cad-play", "VITE_SEMIO_BRAND": "", "VITE_SEMIO_PLUGIN": "cad", "VITE_SEMIO_RENDERER": "react"};import __vite__cjsImport0_react_jsxDevRuntime from "/@fs/Users/ueli/Documents/semio/node_modules/.vite-os-dev/cad-react/deps/react_jsx-dev-runtime.js?v=75efac35"; const Fragment = __vite__cjsImport0_react_jsxDevRuntime["Fragment"]; const jsxDEV = __vite__cjsImport0_react_jsxDevRuntime["jsxDEV"];
var _s = $RefreshSig$(), _s2 = $RefreshSig$(), _s3 = $RefreshSig$(), _s4 = $RefreshSig$(), _s5 = $RefreshSig$(), _s6 = $RefreshSig$(), _s7 = $RefreshSig$();
import __vite__cjsImport1_react from "/@fs/Users/ueli/Documents/semio/node_modules/.vite-os-dev/cad-react/deps/react.js?v=75efac35"; const createContext = __vite__cjsImport1_react["createContext"]; const useCallback = __vite__cjsImport1_react["useCallback"]; const useContext = __vite__cjsImport1_react["useContext"]; const useEffect = __vite__cjsImport1_react["useEffect"]; const useMemo = __vite__cjsImport1_react["useMemo"]; const useReducer = __vite__cjsImport1_react["useReducer"]; const useRef = __vite__cjsImport1_react["useRef"]; const useState = __vite__cjsImport1_react["useState"]








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
    const worker = new Worker(new URL(/* @vite-ignore */ "/@fs/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts?worker_file&type=module", import.meta.url), { type: "module" });
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

//# sourceMappingURL=data:application/json;base64,eyJ2ZXJzaW9uIjozLCJtYXBwaW5ncyI6IkFBeWFTLFNBaTlJQyxVQWo5SUQ7O0FBaGFUO0FBQUEsRUFDRUE7QUFBQUEsRUFNQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsT0FDSztBQUNQO0FBQUEsRUFHRUM7QUFBQUEsRUFFQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFFQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFJQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFNQUM7QUFBQUEsRUFFQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFFQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFFQUM7QUFBQUEsRUFjQUM7QUFBQUEsT0FJSztBQUNQO0FBQUEsRUFHRUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFFQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsT0FFSztBQUNQO0FBQUEsRUFDRUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsT0FDSztBQUNQO0FBQUEsRUFFRUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFFQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFHQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFFQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFHQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFFQUM7QUFBQUEsRUFFQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFLQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFFQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFHQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFJQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFFQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFFQUM7QUFBQUEsRUFFQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFJQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFHQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsT0FDSztBQUNQO0FBQUEsRUFDRUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsT0FDSztBQUNQO0FBQUEsRUFDRUM7QUFBQUEsRUFFQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFHQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFFQUM7QUFBQUEsRUFFQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsT0FLSztBQUNQO0FBQUEsRUFDRUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsT0FDSztBQUNQO0FBQUEsRUFDRUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFDQUM7QUFBQUEsRUFD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QTtBQUFBLGFBQUFDLEtBQUE7QUFBQSxhQUFBQyxLQUFBIiwibmFtZXMiOlsiY3JlYXRlQ29udGV4dCIsInVzZUNhbGxiYWNrIiwidXNlQ29udGV4dCIsInVzZUVmZmVjdCIsInVzZU1lbW8iLCJ1c2VSZWR1Y2VyIiwidXNlUmVmIiwidXNlU3RhdGUiLCJidWlsZENvbnRyaWJ1dGlvbnNKc29uIiwiY3JlYXRlQnJvd3NlclN0b3JhZ2VQb3J0IiwiY3JlYXRlRGV2UGx1Z2luU291cmNlIiwiY3JlYXRlTWVtb3J5U3RvcmFnZVBvcnQiLCJjcmVhdGVTY29wZWRTdG9yYWdlUG9ydCIsIkRvY2tMYXlvdXRTdG9yZSIsIkRvY2tVaVN0YXRlU3RvcmUiLCJldmljdFBsdWdpbk1vZHVsZSIsImV4cGFuZFBsdWdpblJlZ2lzdHJ5IiwiRlJBTUVXT1JLX1BBTkVMX1RBQl9DQVRBTE9HVUVfSUQiLCJGUkFNRVdPUktfUEFORUxfVEFCX0RPQ1VNRU5UX0lDT05fSUQiLCJGUkFNRVdPUktfUEFORUxfVEFCX0RPQ1VNRU5UX0lEIiwiRlJBTUVXT1JLX1BBTkVMX1RBQl9ISVNUT1JZX0lEIiwiTmFtZWRMYXlvdXRTdG9yZSIsIm5vcm1hbGl6ZUFwcExhYmVsc092ZXJsYXkiLCJvcmdhbml6ZUNvbnRleHRNZW51IiwicGFuZWxUYWJLaW5kSWQiLCJwZW5kaW5nUGFuZWxVaU5vZGUiLCJwZW5kaW5nV2luZG93VWlOb2RlIiwicG9zdFBsdWdpbkJhY2tib25lSW5ib3VuZCIsIlJFQ09SRF9UVVRPUklBTF9BQ1RJT05fSUQiLCJyZWdpc3RlclBsdWdpbkJhY2tib25lUm91dGUiLCJyZXNvbHZlRXh0ZXJuYWxTbG90cyIsInJlc29sdmVMYXlvdXRGb3JNb2RlIiwicmVzb2x2ZU1vZGVUb29scyIsInJlc29sdmVQbGF5Z3JvdW5kRGVmYXVsdEFwcElkIiwicmVzb2x2ZVBsdWdpbkhvc3RDb25maWciLCJyZXNvbHZlUGx1Z2luUmVnaXN0cnlJZCIsInJlc29sdmVVaURpcnR5U2NvcGUiLCJyZXNvbHZlV2luZG93QWN0aW9ucyIsIlNFVF9BQ1RJVkVfVE9PTF9BQ1RJT05fSUQiLCJTRVRfQUNUSVZFX1VUSUxJVFlfQUNUSU9OX0lEIiwiU1RBUlRfSU5UUk9EVUNUSU9OX0FDVElPTl9JRCIsIlNUQVJUX1RVVE9SSUFMX0FDVElPTl9JRCIsIlRVVE9SSUFMX0NPTlZFUkdFX01TIiwid2luZG93RWxlbWVudElkIiwiYnVpbGRGaWxlQmFja2JvbmVVcmkiLCJidWlsZEZvbGRlckJhY2tib25lVXJpIiwiYnVpbGRGcmFtZXdvcmtTeW5jVXRpbGl0aWVzIiwiYnVpbGRSZW1vdGVCYWNrYm9uZVVyaSIsImRlY29kZUJhY2tib25lTWVzc2FnZSIsImRlY29kZUJhY2tib25lV29ya2VyUmVzcG9uc2UiLCJkZWNvZGVQYWNrVmFsdWUiLCJlbmNvZGVBY3Rpb25XaXJlIiwiZW5jb2RlQmFja2JvbmVNZXNzYWdlIiwiZW5jb2RlQmFja2JvbmVXb3JrZXJSZXF1ZXN0IiwiZW5jb2RlT3BlcmF0aW9uRW52ZWxvcGVzUGFjayIsIkZSQU1FV09SS19TWU5DX0NPTlRST0xMRVJfSUQiLCJvcGVyYXRpb25FbnZlbG9wZUZyb21XaXJlIiwib3BlcmF0aW9uRW52ZWxvcGVUb1dpcmUiLCJkZWNvZGVXb3JsZFByb2plY3Rpb25UZW1wbGF0ZUlkIiwid29ybGRQcm9qZWN0aW9uU3BlY0ljb25JZCIsIndvcmxkUHJvamVjdGlvblNwZWNMYWJlbCIsIkFOQ0hPUlMiLCJBcHAiLCJhcHBseURvY2tTa2VsZXRvbiIsImFwcGx5VWlUaGVtZVRvUm9vdCIsImJvcmRlck5vcm1hbEJvdHRvbUNsYXNzIiwiYnVpbGRLZXlzQnlBY3Rpb25JZCIsImJ1aWx0aW5VaURyaXZlcnMiLCJidWlsdGluVWlUaGVtZXMiLCJCdXR0b25Hcm91cCIsIkJ1dHRvbkdyb3VwSXRlbSIsIkNhbnZhc1NrZWxldG9uIiwiQ0VMRUJSQVRFX1NUQU1QX0RVUkFUSU9OX01TIiwiY2VsZWJyYXRlQWxsRWxlbWVudHMiLCJjZWxlYnJhdGVFbGVtZW50cyIsImNoaWxkRWxlbWVudElkIiwiQ2hyb21lQXdhcmVXaW5kb3dTY3JvbGxTdXJmYWNlIiwiY2xlYXJVaVRoZW1lRnJvbVJvb3QiLCJjbiIsImNvbXBvc2VDb250cm9sS2V5YmluZGluZ3MiLCJjb21wb3NlVHV0b3JpYWxVaSIsIkNvbnRleHRNZW51Q29udHJvbGxlciIsImNyZWF0ZVNoZWxsU2NvcGUiLCJjcmVhdGVUdXRvcmlhbENsb2NrIiwiREVGQVVMVF9VSV9EUklWRVIiLCJkZXRlY3RTaGVsbExvY2FsZSIsImRpc3Bvc2VTaGVsbEkxOG5JbnN0YW5jZSIsImRvY2tTa2VsZXRvbk9mIiwiZG9ja1NrZWxldG9uc0VxdWFsIiwiZWxlbWVudElkU2VsZWN0b3IiLCJmaW5kUGFuZWxUYWJJbkRvY2siLCJmaW5kUGFuZWxUYWJOb2RlIiwiZmluZFBhbmVsVGFiUGF0aCIsIkZvb3RlciIsImdldFR1dG9yaWFsQ2FtZXJhRHJpdmVyIiwiSWNvbiIsImljb25SZW5kZXJQb3J0IiwiaW5zZXJ0V2luZG93QXREcm9wWm9uZSIsImludGVyYWN0aXZlQWN0aXZlRmlsbENsYXNzIiwiaW50ZXJwb2xhdGVUdXRvcmlhbENhbWVyYSIsImlzQ29udGV4dE1lbnVQb2ludGVyVGFyZ2V0IiwiTGF5b3V0IiwiTGV2ZWxQcm92aWRlciIsImxvYWRpbmdCb3JkZXJDbGFzcyIsIk1vZGUiLCJtb3ZlVGFiSW5Eb2NrIiwibW92ZVRyZWVVbml0SW5Eb2NrIiwiTmF2YmFyIiwiTmF2YmFyRXhhbXBsZVNlbGVjdCIsIm5hdmJhckZpbGxJdGVtIiwiUGFuZWxDaHJvbWVUYWJCYXIiLCJQYW5lbERvY2tQcm92aWRlciIsInBhbmVsVGFiQ2hpbGRyZW4iLCJwYXJzZVVpVGhlbWUiLCJyZWFkU3RvcmVkSW50cm9kdWN0aW9uU2VlbiIsInJlYWRTdG9yZWRVaUNocm9tZUxvY2FsZSIsInJlYWRTdG9yZWRVaUNocm9tZVRoZW1lU25hcHNob3QiLCJyZWNvbmNpbGVBY3RpdmVQYXRoIiwicmVzb2x2ZVVpRHJpdmVyIiwiU2VtaW9Mb2dvIiwic2VtaW9UaGVtZSIsInNlcmlhbGl6ZVVpVGhlbWUiLCJzZXRBY3RpdmVVaVRoZW1lIiwiU2hlbGxCcmFuZExvZ28iLCJzaGVsbENocm9tZVRpdGxlQ2xhc3NOYW1lIiwiU2hlbGxTY29wZVByb3ZpZGVyIiwic2luZ2xlVHJlZUxlYWYiLCJzdGF0aWNUcmVlUGFuZWxEZWZpbml0aW9uIiwiVGV4dFNlbGVjdGlvbkNvbnRleHRNZW51SG9zdCIsIlRvZ2dsZSIsIlR1dG9yaWFsQmFyIiwidHV0b3JpYWxDYW1lcmFBdCIsIlR1dG9yaWFsQ2FwdGlvbnMiLCJ0dXRvcmlhbEN1ZXNCZXR3ZWVuIiwiVHV0b3JpYWxHaG9zdFBvaW50ZXIiLCJ0dXRvcmlhbFNsaWNlIiwiVHV0b3JpYWxWaWRlb092ZXJsYXkiLCJVSV9NT0JJTEVfTUVESUFfUVVFUlkiLCJVSV9URVJNSU5PTE9HWV9OQVRJVkUiLCJVSURpYWxvZyIsIlVJSW50cm9kdWN0aW9uIiwiVWlLZXliaW5kaW5nc1Byb3ZpZGVyIiwidXNlQWN0aW9uSG90a2V5IiwidXNlRWxlbWVudHNTdXJmYWNlQ2hyb21lIiwidXNlTGFiZWwiLCJ1c2VNZWRpYVF1ZXJ5IiwidXNlUGFuZWxDaHJvbWVIb3RrZXlzIiwidXNlU2hlbGxLZXlkb3duIiwidXNlU2hlbGxTY29wZSIsInVzZVR1dG9yaWFsQ2xvY2siLCJ2YWxpZGF0ZVR1dG9yaWFsIiwiV2luZG93Qm9keVNrZWxldG9uIiwid3JpdGVTdG9yZWRJbnRyb2R1Y3Rpb25TZWVuIiwid3JpdGVTdG9yZWRVaUNocm9tZUFwcGVhcmFuY2UiLCJ3cml0ZVN0b3JlZFVpQ2hyb21lTGF5b3V0Iiwid3JpdGVTdG9yZWRVaUNocm9tZUxvY2FsZSIsIndyaXRlU3RvcmVkVWlDaHJvbWVUZXJtaW5vbG9neSIsIndyaXRlU3RvcmVkVWlDaHJvbWVUaGVtZUlkIiwid3JpdGVTdG9yZWRVaUNocm9tZVRoZW1lU25hcHNob3QiLCJ3cml0ZVN0b3JlZFVpQ3VzdG9tRHJpdmVycyIsIndyaXRlU3RvcmVkVWlDdXN0b21UaGVtZXMiLCJ3cml0ZVN0b3JlZFVpRHJpdmVySWQiLCJ3cml0ZVN0b3JlZFVpS2V5YmluZGluZ092ZXJyaWRlcyIsImRlY2xhcmF0aXZlU3VyZmFjZVN0YXR1cyIsIkludGVycHJldGVkVWlOb2RlIiwiUGx1Z2luU3VyZmFjZUFjdGlvbnNDb250ZXh0IiwiU2hlbGxDb250ZXh0TWVudUZhbGxiYWNrQ29udGV4dCIsIndpcmVMYWJlbCIsImFjdGlvblN0YWdlS2V5IiwiRU1QVFlfU0hFTExfREVGQVVMVFMiLCJFTVBUWV9TSEVMTF9MT0NLUyIsImluaXRpYWxTaGVsbFN0YXRlIiwiaXNFcGhlbWVyYWxTaGVsbEJyYW5kIiwicmVzb2x2ZUJvb3RFeGFtcGxlSWQiLCJTaGVsbEZhdWx0Qm91bmRhcnkiLCJzaGVsbFJlZHVjZXIiLCJzaG91bGRQZXJzaXN0SW50cm9kdWN0aW9uU2VlbiIsInNob3VsZFJlcGxheUludHJvZHVjdGlvbk9uTG9hZCIsImJlZ2luSW50ZXJhY3RpdmVQbHVnaW5BY3Rpb24iLCJjbGVhclBlbmRpbmdXb3JsZFByb2plY3Rpb24iLCJlbmRJbnRlcmFjdGl2ZVBsdWdpbkFjdGlvbiIsIm1hcENvbnRleHRNZW51U3BlY3MiLCJyZWdpc3RlclBlbmRpbmdXb3JsZFByb2plY3Rpb24iLCJXaW5kb3dJbnN0YW5jZUlkQ29udGV4dCIsIkRFRkFVTFRfUEFORUxfV0lEVEhfUFgiLCJFTVBUWV9BUFBfTEFCRUxTX09WRVJMQVkiLCJGUkFNRVdPUktfQ0FURUdPUllfQ09NTUFORF9JRCIsIkZSQU1FV09SS19DQVRFR09SWV9ESVNQTEFZX0lEIiwiRlJBTUVXT1JLX0NBVEVHT1JZX1RPT0xfSUQiLCJGUkFNRVdPUktfUkVTRVJWRURfQUNUSU9OX0lEUyIsIkxBWU9VVF9DSEFOR0VfU0VUVExFX01TIiwiTk9URV9XT1JMRF9OQVZJR0FUSU9OX0FDVElPTl9JRCIsIlBBTkVMX1RBQl9CQVJfSE9TVFMiLCJQUkVTRU5DRV9IRUFSVEJFQVRfSU5URVJWQUxfTVMiLCJUVVRPUklBTF9SRUNPUkRJTkdfRVhDTFVERURfQUNUSU9OX0lEUyIsImFjdGlvbkNhdGVnb3J5SWQiLCJhY3Rpb25SZXF1aXJlc1N0YWdlZEZvcm0iLCJhcHBEb2N1bWVudExhYmVsIiwiYXBwV2luZG93RG9jdW1lbnRMYWJlbCIsImFwcGx5RnJhbWV3b3JrTGF5b3V0U2VlZCIsImFwcGx5VHV0b3JpYWxVaUNoYW5nZVRvU2hlbGwiLCJhcHBseVR1dG9yaWFsVWlTbmFwc2hvdFRvU2hlbGwiLCJhcHBseVVpUmVmcmVzaFJlc3BvbnNlVG9DYWNoZSIsImJ1aWxkQWN0aXZlVXRpbGl0eUJ5V2luZG93SWQiLCJidWlsZENvbW1hbmRDYXRlZ29yeVRhYnMiLCJidWlsZE5vdGVTaGVsbENvbW1hbmRBY3Rpb24iLCJidWlsZE9zQ29tbWFuZHMiLCJidWlsZFNwYWNlUGFuZWxTdGF0ZSIsImJ1aWxkVG9vbFRhYnMiLCJidWlsZFVpUmVmcmVzaFJlcXVlc3QiLCJjYXB0dXJlQ3VycmVudEZyYW1ld29ya0xheW91dCIsImNhcHR1cmVUdXRvcmlhbFVpU25hcHNob3QiLCJjYXRlZ29yeVRhYkljb24iLCJjbGFzc2lmeVdpbmRvd0xheW91dENoYW5nZSIsImNvbW1hbmRDYXRlZ29yaWVzIiwiY29tbWFuZENhdGVnb3J5TGFiZWwiLCJkaXNwYXRjaE9wZW5lZEZpbGVzIiwiZGlzcGF0Y2hPc0NvbW1hbmQiLCJkb3dubG9hZERhdGFVcmwiLCJkb3dubG9hZE1lZGlhRXhwb3J0IiwiZmxhdHRlblBhbmVsVGFiTGVhdmVzIiwiaW50cm9kdWN0aW9uVGFyZ2V0c1dpbmRvdyIsImxvYWRQbHVnaW5Nb2R1bGVSZXNpbGllbnQiLCJtYWtlRWZmZWN0RGlzcGF0Y2hPbmUiLCJtZXJnZVJlY29yZFByZXNlcnZpbmdJZGVudGl0eSIsInBhbmVsQW5jaG9yRm9yR3JvdXAiLCJwYW5lbEpzb25Gcm9tU3RhdGUiLCJwYW5lbFRhYkRlZmluaXRpb25Ub05vZGUiLCJwYXJzZVBhbmVsU3RhdGUiLCJwYXJzZVNoZWxsUm91dGUiLCJwYXRjaERvY3VtZW50VHJlZVNlbGVjdGVkSWRzIiwicGF0Y2hXb3JsZDNkQ2hyb21lT250b05vZGUiLCJwcmVzZW5jZUNsaWVudElkZW50aXR5IiwicHJlc2VydmVKc29uSWRlbnRpdHkiLCJyZW5kZXJTdGFnZWRBcmdDb250cm9sIiwicmVxdWVzdEZpbGVPcGVuIiwicmVzb2x2ZUFwcERvY3VtZW50IiwicmVzb2x2ZUFwcExhYmVsIiwicmVzb2x2ZUNhbnZhc0JvZHlLZXkiLCJyZXNvbHZlQ29tbWFuZHMiLCJyZXNvbHZlRGlhbG9nRGVmaW5pdGlvbiIsInJlc29sdmVEb2N1bWVudEJ5QXBwSWQiLCJyZXNvbHZlRnJhbWV3b3JrTGF5b3V0U2VlZCIsInJlc29sdmVJbnRyb2R1Y3Rpb25EZWZpbml0aW9uIiwicmVzb2x2ZUtleWJpbmRpbmdJbnRlbnQiLCJyZXNvbHZlTWFuaWZlc3RMYWJlbCIsInJlc29sdmVQYW5lbFRhYkxhYmVsIiwicmVzb2x2ZVV0aWxpdHlBY3RpdmF0aW9uIiwicmVzb2x2ZVV0aWxpdHlOb2RlcyIsInJlc29sdmVXaW5kb3dFbmdhZ2VtZW50IiwicmV0aXRsZVdpbmRvd0xheW91dE5vZGUiLCJydW5SZXF1ZXN0TWVkaWFGcmFtZXMiLCJzY2hlZHVsZURpc3BhdGNoQWN0aW9uIiwic2Vzc2lvbldpbmRvd0luc3RhbmNlcyIsInNoZWxsTGFiZWwiLCJzaGVsbFRhYkljb24iLCJzcGF3bmVkV2luZG93Q2hyb21lRm9yS2luZCIsInN0dWRpb1BhbmVsRm9jdXNpbmdTcGF3bmVkIiwic3luY0RvY3VtZW50SWQiLCJzeW50aGVzaXplTG9jYWxpemVkTGFiZWwiLCJ0b29sSWRGcm9tUGFuZWxUYWJJZCIsInVzZVVJSGlzdG9yeSIsInV0aWxpdHlCYXJOb2RlIiwidXRpbGl0eU5vZGVUcmVlQ29udGFpbnNJZCIsInZpZXdTdGF0ZVdpdGhTcGFjZVBhbmVsIiwid2luZG93QWN0aW9uUGFuZU5vZGUiLCJ3aW5kb3dFbmdhZ2VtZW50VG9TZWFyY2hTcGVjIiwid2luZG93RW5nYWdlbWVudFRvU3BlYyIsIndpbmRvd01lYXN1cmVUcmVlQ29udGFpbnNJZCIsIndpbmRvd01lYXN1cmVzQ2hyb21lIiwiYVByb2plY3RPZkx1aFVka0Zvb3Rlckl0ZW0iLCJmdW5kZWRCeVp1a3VuZnRCYXVGb290ZXJJdGVtIiwiRU5UV0VSRkVOX01JVF9CRVNUQU5EX0JSQU5EX0lEUyIsImNyZWF0ZUZyYW1ld29ya0Rpc3BsYXlQYW5lbFRhYnMiLCJjcmVhdGVGcmFtZXdvcmtQbHVnaW5zUGFuZWxUYWJzIiwiY3JlYXRlRnJhbWV3b3JrU2V0dGluZ3NQYW5lbFRhYnMiLCJQbHVnaW5SZWNvdmVyeVBhbmVsIiwiU2hlbGxSb3V0ZU5vdEZvdW5kUGFnZSIsInVzZU5hbWVkTGF5b3V0SG9zdCIsIlN5bmNBdHRhY2hDYXJkIiwiVUlGaW5kIiwiVUlGaW5kUHJvdmlkZXIiLCJVSVNlYXJjaCIsIlVUSUxJVFlfQ0FURUdPUllfSUNPTl9JRCIsImNvZXJjZVdpcmVCeXRlcyIsIlNldFdpbmRvd1RpdGxlQ29udGV4dCIsIlNldFdpbmRvd0ljb25Db250ZXh0IiwiRU1QVFlfS0VZU19CWV9BQ1RJT05fSUQiLCJNYXAiLCJBcHBLZXliaW5kaW5nc0NvbnRleHQiLCJfYyIsInVzZUFwcEtleWJpbmRpbmdzQnlBY3Rpb25JZCIsIl9zIiwidXNlTWFwQ29udGV4dE1lbnVTcGVjcyIsImRpc3BhdGNoIiwiX3MyIiwia2V5c0J5QWN0aW9uSWQiLCJzcGVjcyIsInR1dG9yaWFsQXNzZXRTcmNUb1VybCIsInNyYyIsImtpbmQiLCJ1cmwiLCJkYXRhIiwiY29uc29sZSIsIndhcm4iLCJoYXNoIiwiVHV0b3JpYWxDYXB0aW9uc0hvc3QiLCJ0dXRvcmlhbCIsImNsb2NrIiwiY2FwdGlvbnNPbiIsInRlcm1pbm9sb2d5IiwibG9jYWxlIiwiX3MzIiwidGltZU1zIiwiY3VlIiwidHJhY2tzIiwibmFycmF0aW9uIiwidGV4dCIsIlRVVE9SSUFMX0RFRkFVTFRfVklERU9fUkVDVCIsIngiLCJ5Iiwid2lkdGgiLCJoZWlnaHQiLCJUdXRvcmlhbFZpZGVvT3ZlcmxheUhvc3QiLCJtdXRlZCIsInBsYXlpbmciLCJyYXRlIiwiX3M0IiwidmlkZW8iLCJsb2NhbFRpbWVNcyIsImF0Iiwic291cmNlT2Zmc2V0TXMiLCJyZWN0IiwiVHV0b3JpYWxHaG9zdFBvaW50ZXJIb3N0IiwiX3M1IiwiZ2VzdHVyZXMiLCJwcm9ncmVzcyIsIk1hdGgiLCJtaW4iLCJtYXgiLCJkdXJhdGlvbk1zIiwiZGlmZlR1dG9yaWFsVWlTbmFwc2hvdCIsInByZXYiLCJuZXh0IiwiY2hhbmdlcyIsImFjdGl2ZU1vZGVJZCIsInB1c2giLCJpZCIsImZvY3VzZWRXaW5kb3dJZCIsInV0aWxpdHlXaW5kb3dJZHMiLCJTZXQiLCJPYmplY3QiLCJrZXlzIiwiYWN0aXZlVXRpbGl0eUJ5V2luZG93SWQiLCJ3aW5kb3dJZCIsInV0aWxpdHlJZCIsImFjdGl2ZVRvb2xJZCIsImxheW91dCIsIkpTT04iLCJzdHJpbmdpZnkiLCJncm91cHMiLCJhY3RpdmVQYW5lbFRhYkJ5R3JvdXAiLCJncm91cCIsInRhYklkIiwicGFuZWxKc29uIiwic2VsZWN0aW9uSnNvbiIsIm9wZW5EaWFsb2dJZCIsInByZXZUcmVlIiwiZXhwYW5kZWRUcmVlSWRzIiwibmV4dFRyZWUiLCJoYXMiLCJleHBhbmRlZCIsImNvbW1hbmRQYW5lbE9wZW4iLCJvcGVuIiwidHV0b3JpYWxDYW1lcmFQb3NlRXF1YWxzIiwiYSIsImIiLCJwb3NpdGlvbiIsImV2ZXJ5IiwidmFsdWUiLCJpbmRleCIsImFicyIsInRhcmdldCIsInpvb20iLCJUdXRvcmlhbFJlY29yZGVyIiwic3RhcnRlZEF0TXMiLCJiYXNlVWlTbmFwc2hvdCIsImJhc2VEb2N1bWVudEpzb24iLCJldmVudHMiLCJ1aUtleWZyYW1lcyIsImNhbWVyYUtleWZyYW1lcyIsImNoYXB0ZXJzIiwibGFzdFVpU25hcHNob3QiLCJsYXN0Q2FtZXJhQnlXaW5kb3ciLCJjb25zdHJ1Y3RvciIsInBlcmZvcm1hbmNlIiwibm93Iiwibm93TXMiLCJyb3VuZCIsInJlY29yZEV2ZW50IiwicmVjb3JkVWlEaWZmIiwibGVuZ3RoIiwic2FtcGxlIiwicmVjb3JkU25hcHNob3QiLCJzdGF0ZSIsInNhbXBsZUNhbWVyYSIsImNhbWVyYSIsImdldCIsInNldCIsImVhc2luZyIsImFkZENoYXB0ZXIiLCJ0aXRsZSIsInJhd1RpdGxlIiwiYnVpbGQiLCJleGFtcGxlSWQiLCJiYXNlIiwiZG9jdW1lbnRKc29uIiwidW5kZWZpbmVkIiwidWkiLCJjYW1lcmFzIiwiZG9jdW1lbnQiLCJyZWNvcmRlZEF0IiwiRGF0ZSIsInRvSVNPU3RyaW5nIiwicmVzb2x2ZVNoZWxsU2NvcGVTdG9yYWdlIiwiZXBoZW1lcmFsIiwic3RvcmFnZU5hbWVzcGFjZSIsImJyb3dzZXIiLCJGcmFtZXdvcmtPc1NoZWxsIiwicHJvcHMiLCJfczYiLCJzaGVsbElkIiwib3duc1BhZ2UiLCJicmFuZCIsImxvY2tzIiwiaW5uZXJQcm9wcyIsInNjb3BlIiwic3RvcmFnZSIsImluaXRpYWxMb2NhbGUiLCJuYXZpZ2F0b3IiLCJsYW5ndWFnZSIsImJ1bXBBZnRlclJvb3RBdHRhY2giLCJzZXRSb290Iiwibm9kZSIsInJvb3RSZWYiLCJjdXJyZW50IiwibiIsInNldFBvcnRhbExheWVyIiwicG9ydGFsTGF5ZXJSZWYiLCJpMThuIiwiaXNvbGF0aW9uIiwiRnJhbWV3b3JrT3NTaGVsbElubmVyIiwicGx1Z2luRmlsdGVyIiwicGx1Z2lucyIsImFwcElkIiwibG9ja3NQcm9wIiwiZGVmYXVsdHMiLCJkZWZhdWx0c1Byb3AiLCJzdXBwcmVzc0F1dG9JbnRyb2R1Y3Rpb24iLCJfczciLCJzaGVsbENvbnRleHRNZW51VGl0bGVMYWJlbCIsImhvc3RDb25maWciLCJzdHVkaW9Nb2RlIiwibW9iaWxlIiwic2hlbGxTdGF0ZSIsImxvYWRlZFBsdWdpbnMiLCJwbHVnaW5TdGF0dXNCeUlkIiwicGx1Z2luU3VwZXJ2aXNvckJ5SWQiLCJzZXNzaW9uIiwiZXJyb3IiLCJwbHVnaW5SdW50aW1lIiwiaG9zdFBsdWdpbiIsImZpbmQiLCJlbnRyeSIsImhhbmRsZSIsInBsdWdpbklkIiwiaG9zdEFwcCIsIm1hbmlmZXN0IiwiYXBwcyIsImFwcCIsImhvc3RBcHBJZCIsImxhbmRpbmdBcHAiLCJsYW5kaW5nQXBwSWQiLCJob3N0Q29udHJvbGxlcklkIiwiY29udHJvbGxlcklkIiwibGFuZGluZ0NvbnRyb2xsZXJJZCIsImhvc3RDYXRhbG9ndWVUYWJJZCIsInBhbmVsVGFicyIsIndpbmRvd1VpQnlXaW5kb3dJZCIsIndpbmRvd0VuZ2FnZW1lbnRzQnlXaW5kb3dJZCIsIndpbmRvd01lYXN1cmVzQnlXaW5kb3dJZCIsInRvb2xNZWFzdXJlc0J5VG9vbElkIiwicGFuZWxVaUJ5S2V5IiwiYXBwTGFiZWxzT3ZlcmxheSIsIndpbmRvd1VpIiwic3Bhd25lZFdpbmRvd1VpIiwic3Bhd25lZFdpbmRvd0VuZ2FnZW1lbnRzIiwic3Bhd25lZFdpbmRvd01lYXN1cmVzIiwic3Bhd25lZFdpbmRvdyIsImZvbGRlZEJ5V2luZG93SWQiLCJhY3Rpb25QYW5lRm9sZGVkQnlXaW5kb3dJZCIsImV4cGFuZGVkQnlXaW5kb3dJZCIsImFjdGlvblBhbmVFeHBhbmRlZEJ5V2luZG93SWQiLCJzdGFnZWRBcmdzQnlLZXkiLCJhY3Rpb25QYW5lU3RhZ2VkQXJnc0J5S2V5IiwiYWN0aW9uUGFuZSIsImV4cGFuZGVkQ29tbWFuZElkIiwic3RhZ2VkQXJnc0J5Q29tbWFuZElkIiwiY29tbWFuZFN0YWdlZEFyZ3NCeUNvbW1hbmRJZCIsImNvbW1hbmRQYW5lbCIsInBhbmVscyIsImRvY2tPdmVycmlkZSIsInBhbmVsUGF0aE1lbW9yeSIsInRyZWVPcGVuU3RhdGVzIiwiYWN0aXZlV2luZG93SWQiLCJzaGVsbExheW91dCIsImFjdGl2ZUV4YW1wbGVJZCIsIm1vYmlsZVBhbmVsUGF0aCIsIm1vYmlsZVBhbmVsVmlzaWJsZSIsImV4dHJhV2luZG93SW5zdGFuY2VzIiwid2luZG93VGl0bGVzQnlJZCIsIndpbmRvd0ljb25zQnlJZCIsInNlYXJjaE9wZW4iLCJmaW5kT3BlbiIsImludHJvZHVjdGlvblN0ZXBJbmRleCIsImludHJvZHVjdGlvbkNvbXBsZXRlZEludGVyYWN0aW9ucyIsImRpYWxvZyIsIm92ZXJsYXlEaWFsb2ciLCJvdmVybGF5cyIsImFjdGl2ZVR1dG9yaWFsSWQiLCJ0dXRvcmlhbFBsYXlpbmciLCJ0dXRvcmlhbFJhdGUiLCJ0dXRvcmlhbE11dGVkIiwidHV0b3JpYWxDYXB0aW9uc09uIiwicmVjb3JkaW5nIiwidHV0b3JpYWxSZWNvcmRpbmciLCJkZXZpYXRlZCIsInR1dG9yaWFsRGV2aWF0ZWQiLCJ1aUFwcGVhcmFuY2UiLCJ1aUxheW91dCIsInVpRHJpdmVySWQiLCJ1aUN1c3RvbURyaXZlcnMiLCJ1aURyaXZlckRyYWZ0IiwidWlMb2NhbGUiLCJ1aVRlcm1pbm9sb2d5IiwidWlUaGVtZUlkIiwidWlDdXN0b21UaGVtZXMiLCJ1aVRoZW1lRHJhZnQiLCJ1aUtleWJpbmRpbmdPdmVycmlkZXMiLCJ1aVByZWZzIiwic3luY0JhY2tib25lVXJpIiwic3luY0NhcmRLaW5kIiwic3luY0RyYWZ0UGF0aCIsInN5bmNTdGF0dXNCeURvY3VtZW50SWQiLCJzeW5jIiwiaW1wb3J0U3BhY2VJbnB1dFJlZiIsInJlZnJlc2hHZW5lcmF0aW9uUmVmIiwiY29udHJpYnV0aW9uc0pzb25SZWYiLCJhcHBSZWdpc3RyYXRpb25zSnNvblJlZiIsInNwYXduZWRSZWZyZXNoR2VuZXJhdGlvblJlZiIsImNvbnRyaWJ1dG9ySW5zdGFuY2VzUmVmIiwibGF5b3V0U2VlZEtleVJlZiIsIm5vRXhhbXBsZVJlc2V0SW5zdGFuY2VJZFJlZiIsImV4dHJhV2luZG93Q291bnRlclJlZiIsInNoZWxsQ29udGV4dE1lbnUiLCJzZXRTaGVsbENvbnRleHRNZW51IiwiZXh0cmFXaW5kb3dJbnN0YW5jZXNSZWYiLCJzZXRXaW5kb3dUaXRsZSIsInR5cGUiLCJzZXRXaW5kb3dJY29uIiwiaWNvbklkIiwidWlSZWZyZXNoQ2FjaGVSZWYiLCJzcGF3bmVkVWlSZWZyZXNoQ2FjaGVSZWYiLCJzcGF3bmVkTGF5b3V0U2VlZFJlZiIsIm9wZW5TcGFjZUlkUmVmIiwib3Blbkluc3RhbmNlSWRSZWYiLCJzZXNzaW9uUmVmIiwidWlEZXZpY2UiLCJ1aVRoZW1lIiwiZm91bmQiLCJ0IiwidWlEcml2ZXIiLCJiYWNrYm9uZVdvcmtlclJlZiIsInNoZWxsQWN0b3JJZFJlZiIsInJhbmRvbSIsInRvU3RyaW5nIiwic2xpY2UiLCJvcGVuRG9jdW1lbnRTZXNzaW9uc1JlZiIsInBsdWdpbkJhY2tib25lUm91dGVVbnJlZ2lzdGVyc1JlZiIsImxvYWRlZFBsdWdpbnNSZWYiLCJwbHVnaW5Nb2R1bGVVcmxCeUlkUmVmIiwicGx1Z2luT3BJbkZsaWdodFJlZiIsImVuc3VyZUJhY2tib25lV29ya2VyIiwid29ya2VyIiwiV29ya2VyIiwiVVJMIiwiaW1wb3J0Iiwib25tZXNzYWdlIiwibWVzc2FnZUV2ZW50IiwibWVzc2FnZSIsIndpcmUiLCJkb2N1bWVudElkIiwiZXZlbnQiLCJzdGF0dXMiLCJwZXJzaXN0ZWQiLCJwZW5kaW5nT3BlcmF0aW9ucyIsInJlbW90ZSIsInBlZXJzSnNvbiIsInBlZXJzIiwibWFwIiwicGVlciIsImNsaWVudElkIiwiYWN0b3IiLCJuYW1lIiwibGFiZWwiLCJzZWxlY3Rpb25Db3VudCIsImluc3RhbmNlSWQiLCJ2aWV3U3RhdGUiLCJwcmVzZW5jZVBlZXJzSnNvbiIsInBsdWdpbiIsImFwcGx5T3BlcmF0aW9ucyIsImVudmVsb3BlcyIsImFjdG9yVXJpIiwiZW52ZWxvcGUiLCJwaHlzaWNhbF9tcyIsImxvZ2ljYWwiLCJsb2FkQXBwRG9jdW1lbnQiLCJwYWNrQnl0ZXMiLCJVaW50OEFycmF5IiwicGFjayIsIkFycmF5IiwiZnJvbSIsInNwciIsInVyaSIsInNoZWxsVXJpIiwiY2FuR29CYWNrIiwiY2FuR29Gb3J3YXJkIiwiY2FuR29VcCIsImdvQmFjayIsImdvRm9yd2FyZCIsImdvVXAiLCJuYXZpZ2F0ZSIsIm5hdmlnYXRlSGlzdG9yeSIsInNoZWxsUm91dGUiLCJzcGxpdCIsInNoZWxsU3RvcmFnZSIsIm5hbWVkTGF5b3V0U3RvcmUiLCJkb2NrTGF5b3V0U3RvcmUiLCJkb2NrVWlTdGF0ZVN0b3JlIiwicmVnaXN0cnkiLCJwcmltYXJ5UGx1Z2luSWQiLCJzaGVsbFBsdWdpbkNhbnZhc1N0YXR1cyIsInBsdWdpblN0YXR1cyIsInBsdWdpblNvdXJjZSIsImVzdGFibGlzaFByaW1hcnlTZXNzaW9uIiwic0FwcCIsIkVycm9yIiwicGFuZWxTdGF0ZSIsImNyZWF0ZUFwcCIsImRlZmF1bHRNb2RlSWQiLCJtb2RlcyIsInNlZWRlZCIsImRlZmF1bHRMYXlvdXQiLCJ3aW5kb3dLaW5kcyIsImV4dHJhSW5zdGFuY2VzIiwibW9kZUxheW91dCIsInByaW1hcnlBcHAiLCJkZWZhdWx0QXBwSWQiLCJpbnN0YWxsUGx1Z2luIiwicmVidWlsdEF0Iiwic29tZSIsImNhbmRpZGF0ZSIsImFkZCIsIm1vZHVsZVVybCIsImJvb3RFcnJvciIsIlN0cmluZyIsImRlbGV0ZSIsInJlbG9hZFBsdWdpbiIsIm9sZE1vZHVsZVVybCIsIm5ld0hhbmRsZSIsImFjdGl2ZVNlc3Npb24iLCJvd25zU2Vzc2lvbiIsIm9sZEFwcElkcyIsIm5ld0FwcElkcyIsImhvdFN3YXBFdmVudCIsInZlcnNpb24iLCJhZGRlZEFwcHMiLCJmaWx0ZXIiLCJyZW1vdmVkQXBwcyIsImxvZyIsImRlc3Ryb3lBcHAiLCJjYXRjaCIsInNwYXduZWQiLCJzcGF3bmVkQXBwc1JlZiIsImNvbnRyaWJ1dG9ySW5zdGFuY2VJZCIsImN1cnJlbnRQYW5lbCIsImRyb3BwZWQiLCJzcGF3bmVkQXBwcyIsInN1cnZpdmluZ1NwYXduZWQiLCJhY3RpdmVTcGF3bmVkSWQiLCJuZXh0UGFuZWwiLCJuZXh0U2Vzc2lvbiIsImRpc3Bvc2UiLCJ1bmluc3RhbGxQbHVnaW4iLCJwYW5lbCIsImFjdGl2ZVNwYXduZWRFbnRyeSIsImFjdGl2ZUFwcFRpdGxlIiwiYWN0aXZlSW50cm9kdWN0aW9uIiwiaW50cm9kdWN0aW9uIiwiaW50cm9kdWN0aW9uU2VlbktleSIsInJlcGxheUludHJvZHVjdGlvbk9uTG9hZCIsInBlcnNpc3RJbnRyb2R1Y3Rpb25TZWVuIiwiYWN0aXZlSW50cm9kdWN0aW9uUmVmIiwid2luZG93Iiwic2VsZiIsInRvcCIsImFjdGl2ZVR1dG9yaWFscyIsInR1dG9yaWFscyIsInR1dG9yaWFsUmVjb3JkZXJBdmFpbGFibGUiLCJCb29sZWFuIiwiZW52IiwiREVWIiwiYWN0aXZlVXRpbGl0eUJ5V2luZG93SWRSZWYiLCJhY3RpdmVUb29sSWRSZWYiLCJzZXRBY3RpdmVVdGlsaXR5Rm9yV2luZG93IiwiY2xlYXJBbGxXaW5kb3dVdGlsaXRpZXMiLCJ0b29sTWVhc3VyZXNCeVRvb2xJZFJlZiIsImFjdGl2ZVdpbmRvd0lkUmVmIiwiYWN0aW9uUGFuZUV4cGFuZGVkQnlXaW5kb3dJZFJlZiIsImFjdGlvblBhbmVTdGFnZWRBcmdzQnlLZXlSZWYiLCJpbnRyb2R1Y3Rpb25TdGVwSW5kZXhSZWYiLCJpbnRyb2R1Y3Rpb25Db21wbGV0ZWRJbnRlcmFjdGlvbnNSZWYiLCJzdGFydFR1dG9yaWFsUmVmIiwic3RvcFR1dG9yaWFsUmVmIiwidG9nZ2xlVHV0b3JpYWxSZWNvcmRpbmdSZWYiLCJ0dXRvcmlhbERyaXZlblJlZiIsInR1dG9yaWFsUGxheWluZ1JlZiIsInR1dG9yaWFsUmVjb3JkaW5nUmVmIiwidHV0b3JpYWxSZWNvcmRlclJlZiIsInNoZWxsU3RhdGVSZWYiLCJkaXNtaXNzSW50cm9kdWN0aW9uIiwiY29tcGxldGVkIiwiYWR2YW5jZUludHJvZHVjdGlvbkJ5RG9pbmciLCJjZWxlYnJhdGVPdmVycmlkZSIsInN0ZXBJbmRleCIsInN0ZXAiLCJzdGVwcyIsImNlbGVicmF0ZUlkIiwiaW50cm9kdWNlIiwiaW50ZXJhY3Rpb25zIiwiY29tcGxldGVJbnRyb2R1Y3Rpb25JbnRlcmFjdGlvbiIsIm1hdGNoZXMiLCJmaW5kSW5kZXgiLCJpbnRlcmFjdGlvbiIsImkiLCJpbmNsdWRlcyIsIm9yZGVyZWQiLCJjZWxlYnJhdGUiLCJleHBhbmRlZENvbW1hbmRJZFJlZiIsImNvbW1hbmRTdGFnZWRBcmdzQnlDb21tYW5kSWRSZWYiLCJpbmplY3RBY3RpdmVUb29sIiwidG9vbElkIiwiaW5qZWN0QWN0aXZlVXRpbGl0eSIsImtleSIsIndpdGhVdGlsaXR5IiwiYWN0aXZlVXRpbGl0eUlkIiwicmVsYXlQbHVnaW5CYWNrYm9uZU1lc3NhZ2UiLCJtZXNzYWdlQnl0ZXMiLCJzdGFydHNXaXRoIiwiYWN0b3JNZXNzYWdlIiwicGFyc2VkIiwicmVxdWVzdCIsInBvc3RNZXNzYWdlIiwidGVybWluYXRlIiwidW5yZWdpc3RlciIsInZhbHVlcyIsImNsZWFyIiwicHJpbWFyeSIsIndpbmRvd1RpdGxlIiwib3V0Y29tZSIsInJlZ2lzdHJ5SWRzIiwiaGFuZGxlUGx1Z2luQXZhaWxhYmxlIiwiYWxyZWFkeUxvYWRlZCIsInN1YnNjcmliZSIsImZpbmRQbHVnaW5Gb3JBY3Rpb24iLCJhY3Rpb24iLCJieUNvbnRyb2xsZXIiLCJyZXF1ZXN0Q29udGV4dE1lbnUiLCJjb250ZXh0TWVudSIsInJlZnJlc2hVaSIsInNjb3BlQXJnIiwiZXh0cmFJbnN0YW5jZXNPdmVycmlkZSIsImdlbmVyYXRpb24iLCJwcm9ncmFtIiwibGF5b3V0U2VlZEtleSIsImlzU2Vzc2lvblN3aXRjaCIsImNhY2hlIiwibGF5b3V0U2VlZCIsImV4dHJhSW5zdGFuY2VzRm9yRmV0Y2giLCJ3aW5kb3dJbnN0YW5jZXMiLCJjb250cmlidXRpb25zSnNvbiIsImFwcFJlZ2lzdHJhdGlvbnNKc29uIiwiZmxhdE1hcCIsImluc3RhbmNlIiwid2luZG93S2luZElkIiwicGFuZWxUYWJMZWF2ZXMiLCJyZXNwb25zZSIsInNsb3RDb250ZXh0IiwiY29udHJpYnV0b3JJbnN0YW5jZXMiLCJyZXNvbHZlSWZDaGFuZ2VkIiwicmVzb2x2ZWRXaW5kb3dzIiwicmVzb2x2ZWRQYW5lbHMiLCJQcm9taXNlIiwiYWxsIiwid2luZG93cyIsInJlcXVlc3RlZEVmZmVjdHMiLCJhcHBseUhvc3RFZmZlY3RzIiwiY29udHJpYnV0aW9uc1B1c2hLZXkiLCJwbHVnaW5FbnRyeSIsImFyZ3MiLCJqc29uIiwiaGFuZGxlQWN0aW9uIiwiYXBwUmVnaXN0cmF0aW9uc1B1c2hLZXkiLCJkeW5hbWljRW5nYWdlbWVudHMiLCJlbnRyaWVzIiwiZHluYW1pY01lYXN1cmVzIiwiZHluYW1pY1Rvb2xNZWFzdXJlcyIsImZyZXNoQXBwTGFiZWxzT3ZlcmxheSIsInRhYiIsImJvZHlLZXkiLCJrIiwicmVmcmVzaFNwYXduZWRVaSIsInNwYXduZWRTZWVkIiwiZnVsbFZpZXdTdGF0ZSIsInNpbmdsZVdpbmRvd0tpbmQiLCJzZXNzaW9uSWRlbnRpdHlLZXkiLCJyZW5kZXJFcnJvciIsImFjdGl2ZVNwYXduZWQiLCJ1cGRhdGVTcGFjZVBhbmVsIiwic3dpdGNoVG9NYW5hZ2VkQXBwIiwic1BsdWdpbiIsIm5leHRWaWV3U3RhdGUiLCJzeW5jU3Bhd25lZFBsdWdpbkRvY3VtZW50IiwicGx1Z2luSW5zdGFuY2VJZCIsInBhcnNlIiwic3luY0Vycm9yIiwiZW5zdXJlU3Bhd25lZFBsdWdpbiIsIm9zSW5zdGFuY2VJZCIsInNvdXJjZVZpZXdTdGF0ZSIsImV4aXN0aW5nIiwic3Bhd25lZElkIiwiZWZmZWN0cyIsImJhc2VTZXNzaW9uIiwidWlTY29wZSIsImVmZmVjdCIsInNldFBhbmVsIiwic2V0QWN0aXZlVXRpbGl0eSIsInNldEFjdGl2ZVRvb2wiLCJ2b3J0aWNlc0pzb24iLCJkb2N1bWVudFNlbGVjdGVkSWRzIiwiZG9jdW1lbnRIaWdobGlnaHRlZElkcyIsInBhdGNoV29ybGQzZENocm9tZSIsInBhdGNoIiwiZG9jdW1lbnRQYW5lbEtleSIsImRvY3VtZW50Tm9kZSIsImNhY2hlZCIsImRvY3VtZW50Q2FjaGVkIiwiZGlhbG9nSWQiLCJvcGVuRGlhbG9nIiwiZGlhbG9ncyIsInNlZWRBcmdzIiwicGF5bG9hZCIsImxvYWREb2N1bWVudCIsImxvYWRBcHBEb2N1bWVudFBhY2siLCJzcHJCeXRlcyIsIm9wZW5FeHRlcm5hbFVybCIsImZpbGVuYW1lIiwibWltZVR5cGUiLCJlbmNvZGluZyIsIml0ZW0iLCJpY29uUmVuZGVyRXhwb3J0IiwiaXRlbXMiLCJyZXN1bHQiLCJyZW5kZXIiLCJkYXRhVXJsIiwiYWNjZXB0IiwicmVhZEFzIiwiaW1wb3J0QWN0aW9uIiwibXVsdGlwbGUiLCJvcGVuZWQiLCJkaXNwYXRjaEFjdGlvbklkIiwiZGlzcGF0Y2hBcmdzIiwiZGVsYXlNcyIsImRpc3BhdGNoQWN0aW9uIiwiZnJhbWVBY3Rpb24iLCJkb25lQWN0aW9uIiwiZmFsbGJhY2tBY3Rpb24iLCJzYW1wbGVTdHJpZGUiLCJtYXhGcmFtZXMiLCJtYXhMb25nRWRnZVB4IiwiZnBzSGludCIsInJlcXVlc3RNZWRpYUZyYW1lcyIsInJlcXVlc3RKc29uIiwicmVzcG9uc2VBY3Rpb24iLCJyZXF1ZXN0UGx1Z2luRXhjaGFuZ2UiLCJjb250cmlidXRvciIsIm9wZXJhdG9ySWQiLCJpbnB1dEpzb24iLCJub2RlSGFzaCIsImJpbSIsIm91dHB1dEpzb24iLCJldmFsdWF0ZSIsInNwYXduUGx1Z2luSW5zdGFuY2UiLCJjYXRhbG9nIiwicHJvZ3JhbXMiLCJvcGVuUGx1Z2luSW5zdGFuY2UiLCJzcGF3bmVkQ291bnQiLCJpc1NwYXduZWRQbHVnaW5TZXNzaW9uIiwiYXBwbHlTaGVsbFVyaSIsInByZXNlcnZlZFZpZXdTdGF0ZSIsImN1cnJlbnRTZXNzaW9uIiwicGF0aCIsInJvdXRlIiwic3BhY2VJZCIsInN0dWRpb0NoYW5nZWQiLCJzdHVkaW9TZXNzaW9uIiwic3R1ZGlvQ29udHJvbGxlcklkIiwib3BlblJlc3BvbnNlIiwiYWN0aXZlUGFuZWxUYWIiLCJ1cmlFcnJvciIsInJlc29sdmVTeW5jVGFyZ2V0U2Vzc2lvbiIsIm9wZW5Eb2N1bWVudCIsInJlZiIsImJpbmRpbmdzIiwidGFyZ2V0U2Vzc2lvbiIsInNjaGVtYSIsIndhdGNoRXh0ZXJuYWwiLCJhdHRhY2hCYWNrYm9uZSIsImNsb3NlRG9jdW1lbnQiLCJkZXRhY2hCYWNrYm9uZSIsImF0dGFjaFN5bmNCYWNrYm9uZSIsInJlc3QiLCJzbGFzaCIsImluZGV4T2YiLCJiYXNlVXJsIiwicmVwbGFjZSIsImpvaW4iLCJkZXRhY2hTeW5jQmFja2JvbmUiLCJzcGF3blByb2dyYW0iLCJvbkFjdGlvbiIsInN1cGVydmlzb3IiLCJ0dXRvcmlhbElkIiwiaXNBcnJheSIsImdlc3R1cmUiLCJvbiIsInJlcXVlc3RlZCIsImZvcndhcmRlZCIsInRoZW4iLCJ1dGlsaXR5RXJyb3IiLCJ0b29sRXJyb3IiLCJ0cmltIiwiaG9zdFBvcnQiLCJjbGljayIsInAiLCJhY3Rpb25XaW5kb3dJZCIsImRpc3BhdGNoV2luZG93SWQiLCJkaXNwYXRjaFZpZXdTdGF0ZSIsImRlY2xhcmVkQWN0aW9uIiwiYWN0aW9ucyIsImludGVyYWN0aXZlQWN0aW9uIiwiYWN0aW9uRXJyb3IiLCJmaW5hbGx5Iiwibm90ZVNoZWxsQ29tbWFuZCIsImNvbW1hbmRJZCIsImRldGFpbCIsIm9uQWN0aW9uUmVmIiwib25BY3Rpb25TdGFibGUiLCJUVVRPUklBTF9ESVJFQ1RPUl9USUNLX01TIiwiYWN0aXZlVHV0b3JpYWwiLCJ0dXRvcmlhbENsb2NrUmVmIiwidHV0b3JpYWxDbG9jayIsInNldER1cmF0aW9uTXMiLCJzZXRSYXRlIiwicGxheSIsInBhdXNlIiwidWlCcmlkZ2VDdHhSZWYiLCJ0dXRvcmlhbExhc3RBcHBsaWVkTXNSZWYiLCJ0dXRvcmlhbERvY3VtZW50U25hcHNob3RSZWYiLCJwcmV2QWN0aXZlVHV0b3JpYWxJZFJlZiIsInByZXZpb3VzSWQiLCJkZWYiLCJyZWFkQXBwRG9jdW1lbnQiLCJzbmFwc2hvdEVycm9yIiwibG9hZEVycm9yIiwiY2FtZXJhS2V5ZnJhbWUiLCJzZWVrIiwic25hcHNob3RKc29uIiwicmVzdG9yZUVycm9yIiwiYXBwbHlUdXRvcmlhbFNsaWNlVG9TaGVsbCIsImNoYW5nZSIsInVpQ2hhbmdlcyIsImRvY3VtZW50VG91Y2hlZCIsImRvY3VtZW50RXZlbnQiLCJvcGVyYXRpb25zIiwiZm9yd2FyZCIsImZvcndhcmRzIiwiYmFja3dhcmRzIiwicHJldmlvdXNKc29uIiwiY2hlY2twb2ludElkIiwiYWx0ZXJuYXRpdmVJZCIsInRhcmdldElkIiwiY29tbWFuZCIsImxhc3RIZWF2eVRpY2tBdCIsImNhbWVyYVdpbmRvd0lkcyIsImtleWZyYW1lIiwidW5zdWJzY3JpYmUiLCJnZXRUaW1lTXMiLCJwb3NlIiwiaXNQbGF5aW5nIiwic2Vla1R1dG9yaWFsIiwibXMiLCJjbGFtcGVkIiwiYXRNcyIsInBsYXlQYXVzZVR1dG9yaWFsIiwic3RhcnRQb3NlQnlXaW5kb3ciLCJsaXZlIiwic3RhcnRlZEF0IiwidHdlZW4iLCJ0YXJnZXRQb3NlIiwiZHJpdmVyIiwic3RhcnRQb3NlIiwicmVxdWVzdEFuaW1hdGlvbkZyYW1lIiwic3RhcnRUdXRvcmlhbCIsInN0b3BUdXRvcmlhbCIsInRvZ2dsZVR1dG9yaWFsUmVjb3JkaW5nIiwicmVjb3JkZXIiLCJ2YWxpZGF0aW9uRXJyb3IiLCJjYXB0dXJlRXJyb3IiLCJpbnRlcnZhbCIsInNldEludGVydmFsIiwiY2xlYXJJbnRlcnZhbCIsImFkZFR1dG9yaWFsQ2hhcHRlciIsInR1dG9yaWFsQ2hhcHRlck1hcmtlcnMiLCJjaGFwdGVyIiwic3R1ZGlvU2Vzc2lvbkFjdGl2ZSIsInN0dWRpb1Nlc3Npb25Db250cm9sbGVySWQiLCJpZGVudGl0eSIsImJlYXQiLCJpbml0aWFsIiwic2V0VGltZW91dCIsInRpbWVyIiwiY2xlYXJUaW1lb3V0Iiwib25Ub2dnbGUiLCJhbmNob3IiLCJ2aXNpYmxlIiwiaG90a2V5IiwiYXBwZWFyYW5jZSIsImRldmljZSIsImNoYW5nZUxhbmd1YWdlIiwiZG9jdW1lbnRFbGVtZW50IiwibGFuZyIsInRoZW1lSWQiLCJvdmVycmlkZXMiLCJhcHBseU5hbWVkTGF5b3V0IiwiYXBwbHlNb2RlQ2hhbmdlIiwibW9kZUlkIiwiaGFuZGxlVGVtcGxhdGVEcm9wIiwicHJvamVjdGlvblNwZWMiLCJ0ZW1wbGF0ZUlkIiwibmV4dEV4dHJhSW5zdGFuY2VzIiwiZGlzcGxheUhvc3RSZWYiLCJkaXNwbGF5SG9zdCIsImJ1aWx0aW5MYXlvdXRzIiwibmFtZWRMYXlvdXRzIiwiY3VycmVudExheW91dCIsIm9uQXBwbHlMYXlvdXQiLCJ1aVRoZW1lQmFzZSIsInVpVGhlbWVEaXJ0eSIsInVpVGhlbWVMaXN0IiwidWlEcml2ZXJMaXN0Iiwia2V5YmluZGluZ3MiLCJjb250cm9sS2V5YmluZGluZ3MiLCJvc0NvbW1hbmRzIiwidGVybWlub2xvZ2llcyIsIm5vdGVPc0NvbW1hbmQiLCJkcmFmdFRoZW1lUGF0Y2giLCJzdHJ1Y3R1cmVkQ2xvbmUiLCJzZXRUaGVtZUlkIiwic2V0VGhlbWVDb2xvciIsImhleCIsImNvbG9ycyIsInNldFRoZW1lU3BhY2luZyIsInNwYWNpbmciLCJzZXRUaGVtZUZvbnRTdGFjayIsImZvbnRTdGFja3MiLCJzZXRUaGVtZVN0cm9rZSIsInN0cm9rZXMiLCJzZXRUaGVtZVJhZGl1cyIsInJhZGlpIiwic2V0VGhlbWVPcGFjaXR5Iiwib3BhY2l0aWVzIiwic2V0VGhlbWVNZXRyaWMiLCJzZWN0aW9uIiwibWV0cmljcyIsInNldFRoZW1lQXBwZWFyYW5jZVBhaW50IiwiYWxwaGEiLCJhcHBlYXJhbmNlcyIsInJlc2V0VGhlbWUiLCJzYXZlVGhlbWUiLCJ0cmltbWVkIiwic2x1ZyIsInRvTG93ZXJDYXNlIiwic2F2ZWQiLCJkZWxldGVUaGVtZSIsIl9yZW1vdmVkIiwiZXhwb3J0VGhlbWUiLCJpbXBvcnRUaGVtZSIsImNvbnRlbnRzIiwidWlEcml2ZXJCYXNlIiwidWlEcml2ZXJEaXJ0eSIsInNldERyaXZlcklkIiwic2V0RHJpdmVyRmllbGQiLCJzYXZlRHJpdmVyIiwiZGVsZXRlRHJpdmVyIiwidGhlbWVTYXZlTGFiZWwiLCJzZXRUaGVtZVNhdmVMYWJlbCIsImRyaXZlclNhdmVMYWJlbCIsInNldERyaXZlclNhdmVMYWJlbCIsImtleWJpbmRpbmdDYXB0dXJlQ29udHJvbElkIiwic2V0S2V5YmluZGluZ0NhcHR1cmVDb250cm9sSWQiLCJzZXRLZXliaW5kaW5nT3ZlcnJpZGUiLCJjb250cm9sSWQiLCJyZXNldEtleWJpbmRpbmdPdmVycmlkZSIsIm9uTmF2aWdhdGVUb0hvdGtleSIsImFkZEV2ZW50TGlzdGVuZXIiLCJyZW1vdmVFdmVudExpc3RlbmVyIiwic2V0dGluZ3NIb3N0UmVmIiwic2V0dGluZ3NIb3N0IiwiYXBwTGFiZWwiLCJkcml2ZXJJZCIsImRyaXZlckRpcnR5IiwiZHJpdmVycyIsInNldEFwcGVhcmFuY2UiLCJzZXRMYXlvdXQiLCJtb2JpbGVBY3RpdmUiLCJvblJlc2V0RG9jayIsInJlc2V0Iiwic2V0TG9jYWxlIiwic2V0VGVybWlub2xvZ3kiLCJ0aGVtZSIsInRoZW1lRGlydHkiLCJ0aGVtZXMiLCJmcmFtZXdvcmtEaXNwbGF5VGFicyIsImZyYW1ld29ya1NldHRpbmdzVGFicyIsInBsdWdpbnNIb3N0UmVmIiwicGx1Z2luc0hvc3QiLCJsb2FkZWRFbnRyeSIsInNvdXJjZUlkIiwiY2FuVW5pbnN0YWxsIiwiaW5zdGFsbCIsInVuaW5zdGFsbCIsInJlbG9hZCIsImZyYW1ld29ya1BsdWdpbnNUYWJzIiwiaGFuZGxlQXBwS2V5ZG93biIsInBhcnNlS2V5cyIsImlzRWRpdGFibGVUYXJnZXQiLCJIVE1MRWxlbWVudCIsInRhZyIsInRhZ05hbWUiLCJpc0NvbnRlbnRFZGl0YWJsZSIsImNsb3Nlc3QiLCJiaW5kaW5nIiwicGFydHMiLCJwYXJ0IiwibmVlZHNDdHJsIiwibmVlZHNTaGlmdCIsIm5lZWRzQWx0IiwiaGFzQ3RybCIsImN0cmxLZXkiLCJtZXRhS2V5Iiwic2hpZnRLZXkiLCJhbHRLZXkiLCJhY3Rpb25CeUlkIiwicHJldmVudERlZmF1bHQiLCJjaG9yZCIsImRlZmluaXRpb24iLCJzdGFnZWQiLCJpbnRlbnQiLCJhY3Rpb25JZCIsImFjdGl2ZVJpZ2h0UGFuZWxUYWIiLCJhY3RpdmVQYW5lbFRhYklkIiwid29ya2JlbmNoTGVmdFRhYnMiLCJwbHVnaW5MZWZ0VGFicyIsIm9yZGVyIiwiaGFzUGx1Z2luRG9jdW1lbnRUYWIiLCJkb2N1bWVudFRhYiIsImljb24iLCJ0cmVlIiwic2VjdGlvbnMiLCJkZXRhaWxzUmlnaHRUYWJzIiwic2V0dGluZ3NSaWdodFRhYnMiLCJmcmFtZXdvcmtVdGlsaXRpZXNIaXN0b3J5VGFiIiwiZnJhbWV3b3JrU3luY1RhYiIsInN5bmNVdGlsaXRpZXMiLCJzeW5jU3RhdHVzIiwiY29udHJvbCIsImFjdGl2ZVBsdWdpbk1hbmlmZXN0IiwiZXhhbXBsZU9wdGlvbnMiLCJzZWVuIiwiZXhhbXBsZXMiLCJleGFtcGxlIiwiZGlzcGF0Y2hBY3RpdmVFeGFtcGxlIiwiZXhhbXBsZVNlbGVjdEVsZW1lbnQiLCJtb2RlU3dpdGNoZXJFbGVtZW50IiwibW9kZSIsImlzQWN0aXZlIiwicmVzb2x2ZWRDb21tYW5kcyIsImNvbW1hbmRDYXRlZ29yeUxpc3QiLCJvbkNvbW1hbmQiLCJzb3VyY2UiLCJjb21tYW5kRXJyb3IiLCJjb21tYW5kQ2F0ZWdvcnlUYWJzIiwicmVzb2x2ZWRNb2RlVG9vbHMiLCJ0b29sIiwidG9vbFRhYnMiLCJkZWZhdWx0RG9jayIsInRvcExlZnQiLCJib3R0b21MZWZ0IiwiY2hpbGRyZW4iLCJ0b3BSaWdodCIsImJvdHRvbVJpZ2h0IiwiYm90dG9tTWlkZGxlIiwiYW5jaG9ycyIsImdldFNuYXBzaG90IiwiZG9jayIsIm1vYmlsZVBhbmVsVGFicyIsImFuY2hvclRhYnMiLCJhcHBUYWIiLCJkb2NrUGVyc2lzdGVkT25jZVJlZiIsIm5leHRTa2VsZXRvbiIsImRlZmF1bHRTa2VsZXRvbiIsInNhdmUiLCJkb2NrVWlQZXJzaXN0ZWRPbmNlUmVmIiwiZG9ja1VpUGVyc2lzdGVkU3RvcmVSZWYiLCJzaXplIiwiaGFzUGF0aE1lbW9yeSIsImhhc1RyZWVPcGVuIiwiaXNEZWZhdWx0IiwicGF0aE1lbW9yeSIsInRyZWVPcGVuIiwiaGFuZGxlVGFiRG9ja0Ryb3AiLCJtb3ZlIiwibmV4dERvY2siLCJ0YXJnZXRQYXRoIiwiZnJvbUFuY2hvciIsInNvdXJjZVRhYnMiLCJ0b0FuY2hvciIsImhhbmRsZVRyZWVVbml0RG9ja0Ryb3AiLCJzdHVkaW9PdmVycmlkZVRhYklkIiwic3R1ZGlvT3ZlcnJpZGVBbmNob3IiLCJkZXRhaWxzT3ZlcnJpZGVUYWJJZCIsImRldGFpbHNPdmVycmlkZUFuY2hvciIsImFjdGl2ZUludHJvZHVjdGlvblN0ZXAiLCJpbnRyb2R1Y3Rpb25FbGVtZW50SWRzIiwic2hvdyIsImludHJvZHVjdGlvblV0aWxpdHlJZCIsInV0aWxpdGllcyIsInV0aWxpdHkiLCJpbnRyb2R1Y3Rpb25BY3Rpb25XaW5kb3dTZWdtZW50IiwiYWN0aW9uSW5kZXgiLCJpbnRyb2R1Y3Rpb25QYW5lbFRhYklkIiwiZW5kc1dpdGgiLCJpbnRyb2R1Y3Rpb25Ub29sUGlja0lkcyIsImZyb21JbnRlcmFjdGlvbnMiLCJtYXRjaCIsImV4ZWMiLCJpbnRyb2R1Y3Rpb25QYW5lbFRhYkFuY2hvciIsImludHJvZHVjdGlvblV0aWxpdHlXaW5kb3dJZCIsImludHJvZHVjdGlvbk1lYXN1cmVXaW5kb3dJZCIsImtpbmRNZWFzdXJlcyIsIm9wdGlvbnMiLCJtZWFzdXJlcyIsImludHJvZHVjdGlvblRvb2xJZCIsImxhc3RJbnRyb2R1Y3Rpb25Ub29sSWRSZWYiLCJsYXN0SW50cm9kdWN0aW9uVG9vbFBpY2tTdGVwSWRSZWYiLCJyZXNvbHZlZCIsInRvb2xBbmNob3IiLCJsYXN0SW50cm9kdWN0aW9uUGFuZWxUYWJJZFJlZiIsImxvY2F0ZWQiLCJsYXN0SW50cm9kdWN0aW9uRXhwYW5kU3RlcElkUmVmIiwiZXhwYW5kSW50ZXJhY3Rpb25zIiwic3RhdGVTdWZmaXgiLCJjYXRhbG9ndWVLZXkiLCJzZWN0aW9uSWQiLCJwYW5lbEFjdGl2ZVBhdGhzIiwibGFzdFN0dWRpb092ZXJyaWRlVGFiSWRSZWYiLCJsYXN0RGV0YWlsc092ZXJyaWRlVGFiSWRSZWYiLCJtb2JpbGVQYW5lbCIsInRhYnMiLCJhY3RpdmVUYWJQYXRoIiwib25BY3RpdmVUYWJQYXRoQ2hhbmdlIiwib25QYXRoTWVtb3J5Q2hhbmdlIiwib25UcmVlT3BlblN0YXRlQ2hhbmdlIiwidHJlZUNvbnRlbnRSZXZpc2lvbiIsIm9wdGlvbiIsImJ1aWxkUGFuZWxTZWxlY3Rpb25Qcm9wcyIsIm9uVmlzaWJsZUNoYW5nZSIsInBhdGhDaGFuZ2VkIiwic2VsZWN0ZWRUb29sSWQiLCJuYXZiYXJJdGVtcyIsImxvZ29BbmRUaXRsZSIsImxvZ29TdmciLCJzaG93RXhhbXBsZVNlbGVjdCIsImNvbnRlbnQiLCJjZW50ZXJDb250ZW50IiwiY2VudGVyZWQiLCJzZWFyY2hJdGVtcyIsImNhdGVnb3J5Iiwib25TZWxlY3QiLCJkZWNsYXJlZEFjdGlvbklkcyIsImhvc3RXaW5kb3dGb3JBY3Rpb24iLCJpblBhbGV0dGUiLCJhcmdDYXJyeWluZyIsInJlc29sdmVkQWN0aW9uTGFiZWwiLCJkZXNjcmlwdGlvbiIsImNvbW1hbmRQYXRoIiwibW9kZVdpbmRvd3MiLCJhY3Rpb25QYW5lU2xpY2UiLCJhY3Rpb25zRm9sZGVkRm9yIiwidXRpbGl0eUJhckZvbGRlZEZvciIsIm1lYXN1cmVzRm9sZGVkRm9yIiwib25BY3Rpb25zRm9sZGVkRm9yIiwiZm9sZGVkIiwiY3Vyc29yRm9yIiwiY3Vyc29yIiwic3Bhd25lZEFwcCIsIndpbmRvd0tpbmQiLCJjaHJvbWUiLCJzcGF3bmVkVXRpbGl0aWVzIiwiZmlsbCIsInNob3dDb250cm9scyIsIm1lYXN1cmVzRm9sZGVkIiwiZW5nYWdlbWVudCIsInNlYXJjaCIsInV0aWxpdHlCYXIiLCJ1dGlsaXR5T3B0aW9ucyIsInV0aWxpdHlCYXJGb2xkZWQiLCJhY3Rpb25zRm9sZGVkIiwib25BY3Rpb25zRm9sZGVkQ2hhbmdlIiwiYmFzZVdpbmRvd3MiLCJyZXNvbHZlZEVuZ2FnZW1lbnQiLCJza2VsZXRvbiIsImV4dHJhV2luZG93cyIsImVmZmVjdGl2ZU1vZGVMYXlvdXQiLCJoYW5kbGVBY3RpdmVXaW5kb3dDaGFuZ2UiLCJsYXlvdXRDaGFuZ2VTZXR0bGVUaW1lb3V0UmVmIiwibGF5b3V0Q2hhbmdlQ2xhc3NpZmljYXRpb25SZWYiLCJsYXlvdXRDaGFuZ2VQcmV2aW91c1JlZiIsImhhbmRsZU1vZGVMYXlvdXRDaGFuZ2UiLCJjbGFzc2lmaWNhdGlvbiIsImZpbmFsQ2xhc3NpZmljYXRpb24iLCJjYW52YXMiLCJzdXBlcnZpc29yUGx1Z2luSWQiLCJzdXBlcnZpc29yU3RhdGUiLCJzdHVkaW9Ib21lQmFyIiwiZm9jdXNlZFNwYXduZWQiLCJmb2N1c2VkQmFyIiwiZmlsZSIsImZpbGVzIiwicmVhZGVyIiwiRmlsZVJlYWRlciIsIm9ubG9hZCIsInJlYWRBc0RhdGFVUkwiLCJjbG9zZWRTcGF3bmVkIiwibmV4dFNwYXduZWQiLCJjbG9zZWRQbHVnaW4iLCJmb290ZXJJdGVtcyIsImNsYXNzTmFtZSIsImJ1aWxkUGFuZWxQcm9wcyIsIm9uU2l6ZUNoYW5nZSIsInRhYkJhckhvc3QiLCJyb290IiwiYmVhY29uSWQiLCJub3RGb3VuZCIsImRhdGFzZXQiLCJzZW1pb09zTm90Rm91bmQiLCJzZW1pb09zUmVhZHkiLCJzZW1pb09zRXJyb3IiLCJkaXNwYXRjaFNoZWxsTWVudUFjdGlvbiIsImJ1aWxkU2hlbGxDb250ZXh0TWVudUl0ZW1zIiwiY2F0ZWdvcnlCeUFjdGlvbklkIiwic2hvcnRjdXQiLCJkZXN0cnVjdGl2ZSIsInNlcGFyYXRvciIsIm9yZ2FuaXplZCIsImhhbmRsZUNvbnRleHRNZW51IiwiY2xpZW50WCIsImNsaWVudFkiLCJmcm9tRW50cmllcyIsIm9uQ2hhbmdlIiwic3VibWl0QWN0aW9uIiwiY2FuY2VsQWN0aW9uIiwiX2MyIiwiX2MzIiwiX2M0IiwiX2M1IiwiX2M2Il0sImlnbm9yZUxpc3QiOltdLCJzb3VyY2VzIjpbIvCfn6bvuI9jb21wb25lbnQudHN4Il0sInNvdXJjZXNDb250ZW50IjpbIi8vICNyZWdpb24g8J+nsu+4j0hlYWRlclxuLy8g8J+OqO+4jyBmcmFtZXdvcmsvcHJvZHVjdHMvb3MvbW9kdWxlcy9yZW5kZXJlci9lbmdpbmUvZWxlbWVudHMvU2hlbGxIb3N0L2NvbXBvbmVudC50c3hcbi8qKiBAZW1vamkg8J+Pl++4jyBgU2hlbGxIb3N0YCDigJQgdGhlIGBGcmFtZXdvcmtPc1NoZWxsYCBvcmNoZXN0cmF0b3I6IGJvb3RzL2hvdC1zd2FwcyBwbHVnaW4gd2FzbSBtb2R1bGVzLFxuICogb3ducyB0aGUgd2luZG93L2RvY2svcGFuZWwgbGF5b3V0LCB3aXJlcyB0aGUgdHV0b3JpYWwgcmVjb3JkZXIvcGxheWVyLCBwcmVzZW5jZSwgYmFja2JvbmUgc3luYyxcbiAqIGNvbW1hbmQvdG9vbC91dGlsaXR5IHJpYmJvbnMsIGNvbnRleHQgbWVudXMsIGFuZCBtb3VudHMgZXZlcnkgcGVyLWFwcCB3aW5kb3cgdmlhIGBJbnRlcnByZXRlcmAuXG4gKiBUaGUgc2luZ2xlIGxhcmdlc3QgY29tcG9uZW50IGluIHRoZSByZW5kZXJlci1yZWFjdCBwYWNrYWdlLiAqL1xuLy8gI2VuZHJlZ2lvbiDwn6ey77iPSGVhZGVyXG5cbi8vICNyZWdpb24g8J+UjO+4j0FkYXB0ZXJzXG5pbXBvcnQgUmVhY3QsIHtcbiAgY3JlYXRlQ29udGV4dCxcbiAgdHlwZSBDU1NQcm9wZXJ0aWVzLFxuICB0eXBlIEtleWJvYXJkRXZlbnQsXG4gIHR5cGUgTW91c2VFdmVudCxcbiAgdHlwZSBSZWFjdEVsZW1lbnQsXG4gIHR5cGUgUmVhY3ROb2RlLFxuICB1c2VDYWxsYmFjayxcbiAgdXNlQ29udGV4dCxcbiAgdXNlRWZmZWN0LFxuICB1c2VNZW1vLFxuICB1c2VSZWR1Y2VyLFxuICB1c2VSZWYsXG4gIHVzZVN0YXRlLFxufSBmcm9tIFwicmVhY3RcIjtcbmltcG9ydCB7XG4gIHR5cGUgQWN0aW9uRGVzY3JpcHRvcixcbiAgdHlwZSBBcHBEZWZpbml0aW9uLFxuICBidWlsZENvbnRyaWJ1dGlvbnNKc29uLFxuICB0eXBlIENvbnRleHRNZW51SXRlbVNwZWMsXG4gIGNyZWF0ZUJyb3dzZXJTdG9yYWdlUG9ydCxcbiAgY3JlYXRlRGV2UGx1Z2luU291cmNlLFxuICBjcmVhdGVNZW1vcnlTdG9yYWdlUG9ydCxcbiAgY3JlYXRlU2NvcGVkU3RvcmFnZVBvcnQsXG4gIERvY2tMYXlvdXRTdG9yZSxcbiAgdHlwZSBEb2NrVWlQYW5lbFN0YXRlLFxuICBEb2NrVWlTdGF0ZVN0b3JlLFxuICBldmljdFBsdWdpbk1vZHVsZSxcbiAgZXhwYW5kUGx1Z2luUmVnaXN0cnksXG4gIEZSQU1FV09SS19QQU5FTF9UQUJfQ0FUQUxPR1VFX0lELFxuICBGUkFNRVdPUktfUEFORUxfVEFCX0RPQ1VNRU5UX0lDT05fSUQsXG4gIEZSQU1FV09SS19QQU5FTF9UQUJfRE9DVU1FTlRfSUQsXG4gIEZSQU1FV09SS19QQU5FTF9UQUJfSElTVE9SWV9JRCxcbiAgdHlwZSBIb3N0RWZmZWN0LFxuICB0eXBlIEludHJvZHVjdGlvbkludGVyYWN0aW9uLFxuICB0eXBlIExvY2FsaXplZExhYmVsLFxuICBOYW1lZExheW91dFN0b3JlLFxuICBub3JtYWxpemVBcHBMYWJlbHNPdmVybGF5LFxuICBvcmdhbml6ZUNvbnRleHRNZW51LFxuICBwYW5lbFRhYktpbmRJZCxcbiAgcGVuZGluZ1BhbmVsVWlOb2RlLFxuICBwZW5kaW5nV2luZG93VWlOb2RlLFxuICB0eXBlIFBsdWdpbkFwcExhYmVsc092ZXJsYXksXG4gIHR5cGUgUGx1Z2luQ29udGV4dE1lbnVSZXF1ZXN0LFxuICB0eXBlIFBsdWdpblNvdXJjZSxcbiAgdHlwZSBQbHVnaW5Tb3VyY2VFdmVudCxcbiAgdHlwZSBQbHVnaW5VaVJlZnJlc2hTZWN0aW9uUmVzcG9uc2UsXG4gIHBvc3RQbHVnaW5CYWNrYm9uZUluYm91bmQsXG4gIHR5cGUgUHJvZ3JhbUhvdFN3YXBFdmVudCxcbiAgUkVDT1JEX1RVVE9SSUFMX0FDVElPTl9JRCxcbiAgcmVnaXN0ZXJQbHVnaW5CYWNrYm9uZVJvdXRlLFxuICByZXNvbHZlRXh0ZXJuYWxTbG90cyxcbiAgcmVzb2x2ZUxheW91dEZvck1vZGUsXG4gIHJlc29sdmVNb2RlVG9vbHMsXG4gIHJlc29sdmVQbGF5Z3JvdW5kRGVmYXVsdEFwcElkLFxuICByZXNvbHZlUGx1Z2luSG9zdENvbmZpZyxcbiAgcmVzb2x2ZVBsdWdpblJlZ2lzdHJ5SWQsXG4gIHJlc29sdmVVaURpcnR5U2NvcGUsXG4gIHJlc29sdmVXaW5kb3dBY3Rpb25zLFxuICBTRVRfQUNUSVZFX1RPT0xfQUNUSU9OX0lELFxuICBTRVRfQUNUSVZFX1VUSUxJVFlfQUNUSU9OX0lELFxuICB0eXBlIFNoZWxsQnJhbmQsXG4gIFNUQVJUX0lOVFJPRFVDVElPTl9BQ1RJT05fSUQsXG4gIFNUQVJUX1RVVE9SSUFMX0FDVElPTl9JRCxcbiAgdHlwZSBTdG9yYWdlUG9ydCxcbiAgVFVUT1JJQUxfQ09OVkVSR0VfTVMsXG4gIHR5cGUgVHV0b3JpYWxBc3NldFNyYyxcbiAgdHlwZSBUdXRvcmlhbENhbWVyYVN0YXRlLFxuICB0eXBlIFR1dG9yaWFsQ2hhcHRlcixcbiAgdHlwZSBUdXRvcmlhbERlZmluaXRpb24sXG4gIHR5cGUgVHV0b3JpYWxEb2N1bWVudEV2ZW50S2luZCxcbiAgdHlwZSBUdXRvcmlhbEV2ZW50LFxuICB0eXBlIFR1dG9yaWFsR2VzdHVyZUN1ZSxcbiAgdHlwZSBUdXRvcmlhbFVpQ2hhbmdlLFxuICB0eXBlIFR1dG9yaWFsVWlTbmFwc2hvdCxcbiAgdHlwZSBUdXRvcmlhbFZpZGVvQ3VlLFxuICB0eXBlIFVpRGlydHlTY29wZSxcbiAgdHlwZSBVaU5vZGUsXG4gIHR5cGUgVXRpbGl0eU5vZGUsXG4gIHdpbmRvd0VsZW1lbnRJZCxcbiAgdHlwZSBXaW5kb3dFbmdhZ2VtZW50LFxuICB0eXBlIFdpbmRvd0xheW91dCxcbiAgdHlwZSBXaW5kb3dNZWFzdXJlLFxufSBmcm9tIFwiQHNlbWlvLXRlY2gvZnJhbWV3b3JrLWNvcmVcIjtcbmltcG9ydCB7XG4gIHR5cGUgQmFja2JvbmVXb3JrZXJSZXF1ZXN0LFxuICB0eXBlIEJhY2tib25lV29ya2VyUmVzcG9uc2UsXG4gIGJ1aWxkRmlsZUJhY2tib25lVXJpLFxuICBidWlsZEZvbGRlckJhY2tib25lVXJpLFxuICBidWlsZEZyYW1ld29ya1N5bmNVdGlsaXRpZXMsXG4gIGJ1aWxkUmVtb3RlQmFja2JvbmVVcmksXG4gIGRlY29kZUJhY2tib25lTWVzc2FnZSxcbiAgZGVjb2RlQmFja2JvbmVXb3JrZXJSZXNwb25zZSxcbiAgZGVjb2RlUGFja1ZhbHVlLFxuICB0eXBlIERvY3VtZW50QWN0b3JNc2csXG4gIGVuY29kZUFjdGlvbldpcmUsXG4gIGVuY29kZUJhY2tib25lTWVzc2FnZSxcbiAgZW5jb2RlQmFja2JvbmVXb3JrZXJSZXF1ZXN0LFxuICBlbmNvZGVPcGVyYXRpb25FbnZlbG9wZXNQYWNrLFxuICBGUkFNRVdPUktfU1lOQ19DT05UUk9MTEVSX0lELFxuICBvcGVyYXRpb25FbnZlbG9wZUZyb21XaXJlLFxuICBvcGVyYXRpb25FbnZlbG9wZVRvV2lyZSxcbiAgdHlwZSBQZXJzaXN0ZW5jZUJpbmRpbmcsXG59IGZyb20gXCJAc2VtaW8tdGVjaC9mcmFtZXdvcmstb3MtY29yZVwiO1xuaW1wb3J0IHtcbiAgZGVjb2RlV29ybGRQcm9qZWN0aW9uVGVtcGxhdGVJZCxcbiAgd29ybGRQcm9qZWN0aW9uU3BlY0ljb25JZCxcbiAgd29ybGRQcm9qZWN0aW9uU3BlY0xhYmVsLFxufSBmcm9tIFwiQHNlbWlvLXRlY2gvaW5maW5pdGUtd29ybGQtcjNmXCI7XG5pbXBvcnQge1xuICB0eXBlIEFuY2hvcixcbiAgQU5DSE9SUyxcbiAgQXBwLFxuICBhcHBseURvY2tTa2VsZXRvbixcbiAgYXBwbHlVaVRoZW1lVG9Sb290LFxuICBib3JkZXJOb3JtYWxCb3R0b21DbGFzcyxcbiAgYnVpbGRLZXlzQnlBY3Rpb25JZCxcbiAgYnVpbHRpblVpRHJpdmVycyxcbiAgYnVpbHRpblVpVGhlbWVzLFxuICBCdXR0b25Hcm91cCxcbiAgQnV0dG9uR3JvdXBJdGVtLFxuICBDYW52YXNTa2VsZXRvbixcbiAgQ0VMRUJSQVRFX1NUQU1QX0RVUkFUSU9OX01TLFxuICBjZWxlYnJhdGVBbGxFbGVtZW50cyxcbiAgY2VsZWJyYXRlRWxlbWVudHMsXG4gIGNoaWxkRWxlbWVudElkLFxuICBDaHJvbWVBd2FyZVdpbmRvd1Njcm9sbFN1cmZhY2UsXG4gIGNsZWFyVWlUaGVtZUZyb21Sb290LFxuICBjbixcbiAgY29tcG9zZUNvbnRyb2xLZXliaW5kaW5ncyxcbiAgY29tcG9zZVR1dG9yaWFsVWksXG4gIENvbnRleHRNZW51Q29udHJvbGxlcixcbiAgdHlwZSBDb250ZXh0TWVudUl0ZW0sXG4gIGNyZWF0ZVNoZWxsU2NvcGUsXG4gIGNyZWF0ZVR1dG9yaWFsQ2xvY2ssXG4gIERFRkFVTFRfVUlfRFJJVkVSLFxuICBkZXRlY3RTaGVsbExvY2FsZSxcbiAgZGlzcG9zZVNoZWxsSTE4bkluc3RhbmNlLFxuICBkb2NrU2tlbGV0b25PZixcbiAgZG9ja1NrZWxldG9uc0VxdWFsLFxuICBlbGVtZW50SWRTZWxlY3RvcixcbiAgdHlwZSBFbGVtZW50c1N1cmZhY2VBcHBlYXJhbmNlLFxuICB0eXBlIEVsZW1lbnRzU3VyZmFjZURldmljZSxcbiAgZmluZFBhbmVsVGFiSW5Eb2NrLFxuICBmaW5kUGFuZWxUYWJOb2RlLFxuICBmaW5kUGFuZWxUYWJQYXRoLFxuICBGb290ZXIsXG4gIGdldFR1dG9yaWFsQ2FtZXJhRHJpdmVyLFxuICBJY29uLFxuICB0eXBlIEljb25OYW1lLFxuICBpY29uUmVuZGVyUG9ydCxcbiAgaW5zZXJ0V2luZG93QXREcm9wWm9uZSxcbiAgaW50ZXJhY3RpdmVBY3RpdmVGaWxsQ2xhc3MsXG4gIGludGVycG9sYXRlVHV0b3JpYWxDYW1lcmEsXG4gIGlzQ29udGV4dE1lbnVQb2ludGVyVGFyZ2V0LFxuICBMYXlvdXQsXG4gIExldmVsUHJvdmlkZXIsXG4gIGxvYWRpbmdCb3JkZXJDbGFzcyxcbiAgTW9kZSxcbiAgdHlwZSBNb2RlQ2FudmFzRHJvcFRhcmdldCxcbiAgdHlwZSBNb2RlV2luZG93RGVzY3JpcHRvcixcbiAgbW92ZVRhYkluRG9jayxcbiAgbW92ZVRyZWVVbml0SW5Eb2NrLFxuICBOYXZiYXIsXG4gIE5hdmJhckV4YW1wbGVTZWxlY3QsXG4gIG5hdmJhckZpbGxJdGVtLFxuICB0eXBlIE5hdmJhckl0ZW0sXG4gIFBhbmVsQ2hyb21lVGFiQmFyLFxuICB0eXBlIFBhbmVsRG9jayxcbiAgUGFuZWxEb2NrUHJvdmlkZXIsXG4gIHBhbmVsVGFiQ2hpbGRyZW4sXG4gIHR5cGUgUGFuZWxUYWJEb2NrTW92ZSxcbiAgdHlwZSBQYW5lbFRhYk5vZGUsXG4gIHR5cGUgUGFuZWxUYWJTZWxlY3Rpb25PcHRpb25zLFxuICB0eXBlIFBhbmVsVHJlZVVuaXREb2NrTW92ZSxcbiAgcGFyc2VVaVRoZW1lLFxuICByZWFkU3RvcmVkSW50cm9kdWN0aW9uU2VlbixcbiAgcmVhZFN0b3JlZFVpQ2hyb21lTG9jYWxlLFxuICByZWFkU3RvcmVkVWlDaHJvbWVUaGVtZVNuYXBzaG90LFxuICByZWNvbmNpbGVBY3RpdmVQYXRoLFxuICByZXNvbHZlVWlEcml2ZXIsXG4gIFNlbWlvTG9nbyxcbiAgc2VtaW9UaGVtZSxcbiAgc2VyaWFsaXplVWlUaGVtZSxcbiAgc2V0QWN0aXZlVWlUaGVtZSxcbiAgU2hlbGxCcmFuZExvZ28sXG4gIHNoZWxsQ2hyb21lVGl0bGVDbGFzc05hbWUsXG4gIHR5cGUgU2hlbGxTY29wZSxcbiAgU2hlbGxTY29wZVByb3ZpZGVyLFxuICBzaW5nbGVUcmVlTGVhZixcbiAgc3RhdGljVHJlZVBhbmVsRGVmaW5pdGlvbixcbiAgVGV4dFNlbGVjdGlvbkNvbnRleHRNZW51SG9zdCxcbiAgdHlwZSBUaGVtZUFwcGVhcmFuY2VOYW1lLFxuICB0eXBlIFRoZW1lUGFsZXR0ZUdyb3VwLFxuICBUb2dnbGUsXG4gIFR1dG9yaWFsQmFyLFxuICB0dXRvcmlhbENhbWVyYUF0LFxuICBUdXRvcmlhbENhcHRpb25zLFxuICB0eXBlIFR1dG9yaWFsQ2hhcHRlck1hcmtlcixcbiAgdHlwZSBUdXRvcmlhbENsb2NrLFxuICB0eXBlIFR1dG9yaWFsQ2xvY2tQb3J0LFxuICB0dXRvcmlhbEN1ZXNCZXR3ZWVuLFxuICBUdXRvcmlhbEdob3N0UG9pbnRlcixcbiAgdHV0b3JpYWxTbGljZSxcbiAgdHlwZSBUdXRvcmlhbFNsaWNlLFxuICBUdXRvcmlhbFZpZGVvT3ZlcmxheSxcbiAgVUlfTU9CSUxFX01FRElBX1FVRVJZLFxuICBVSV9URVJNSU5PTE9HWV9OQVRJVkUsXG4gIHR5cGUgVWlDaHJvbWVMYXlvdXQsXG4gIFVJRGlhbG9nLFxuICB0eXBlIFVpRHJpdmVyLFxuICBVSUludHJvZHVjdGlvbixcbiAgVWlLZXliaW5kaW5nc1Byb3ZpZGVyLFxuICB0eXBlIFVpTG9jYWxlLFxuICB0eXBlIFVpU3RhdHVzLFxuICB0eXBlIFVpVGhlbWUsXG4gIHVzZUFjdGlvbkhvdGtleSxcbiAgdXNlRWxlbWVudHNTdXJmYWNlQ2hyb21lLFxuICB1c2VMYWJlbCxcbiAgdXNlTWVkaWFRdWVyeSxcbiAgdXNlUGFuZWxDaHJvbWVIb3RrZXlzLFxuICB1c2VTaGVsbEtleWRvd24sXG4gIHVzZVNoZWxsU2NvcGUsXG4gIHVzZVR1dG9yaWFsQ2xvY2ssXG4gIHZhbGlkYXRlVHV0b3JpYWwsXG4gIFdpbmRvd0JvZHlTa2VsZXRvbixcbiAgdHlwZSBXaW5kb3dMYXlvdXROb2RlLFxuICB0eXBlIFdpbmRvd1RlbXBsYXRlRHJvcFBheWxvYWQsXG4gIHdyaXRlU3RvcmVkSW50cm9kdWN0aW9uU2VlbixcbiAgd3JpdGVTdG9yZWRVaUNocm9tZUFwcGVhcmFuY2UsXG4gIHdyaXRlU3RvcmVkVWlDaHJvbWVMYXlvdXQsXG4gIHdyaXRlU3RvcmVkVWlDaHJvbWVMb2NhbGUsXG4gIHdyaXRlU3RvcmVkVWlDaHJvbWVUZXJtaW5vbG9neSxcbiAgd3JpdGVTdG9yZWRVaUNocm9tZVRoZW1lSWQsXG4gIHdyaXRlU3RvcmVkVWlDaHJvbWVUaGVtZVNuYXBzaG90LFxuICB3cml0ZVN0b3JlZFVpQ3VzdG9tRHJpdmVycyxcbiAgd3JpdGVTdG9yZWRVaUN1c3RvbVRoZW1lcyxcbiAgd3JpdGVTdG9yZWRVaURyaXZlcklkLFxuICB3cml0ZVN0b3JlZFVpS2V5YmluZGluZ092ZXJyaWRlcyxcbn0gZnJvbSBcIkBzZW1pby10ZWNoL3VpLXJlYWN0XCI7XG5pbXBvcnQge1xuICBkZWNsYXJhdGl2ZVN1cmZhY2VTdGF0dXMsXG4gIEludGVycHJldGVkVWlOb2RlLFxuICBQbHVnaW5TdXJmYWNlQWN0aW9uc0NvbnRleHQsXG4gIFNoZWxsQ29udGV4dE1lbnVGYWxsYmFja0NvbnRleHQsXG4gIHdpcmVMYWJlbCxcbn0gZnJvbSBcIi4uL0ludGVycHJldGVyL/Cfn6bvuI9jb21wb25lbnQudHN4XCI7XG5pbXBvcnQge1xuICBhY3Rpb25TdGFnZUtleSxcbiAgdHlwZSBBY3RpdmVTZXNzaW9uLFxuICBFTVBUWV9TSEVMTF9ERUZBVUxUUyxcbiAgRU1QVFlfU0hFTExfTE9DS1MsXG4gIHR5cGUgRXh0cmFXaW5kb3dJbnN0YW5jZSxcbiAgdHlwZSBGcmFtZXdvcmtPc0RlZmF1bHRzLFxuICBpbml0aWFsU2hlbGxTdGF0ZSxcbiAgaXNFcGhlbWVyYWxTaGVsbEJyYW5kLFxuICB0eXBlIExvYWRlZFByb2dyYW1TdGF0ZSxcbiAgcmVzb2x2ZUJvb3RFeGFtcGxlSWQsXG4gIHR5cGUgUmVzb2x2ZWRTaGVsbExvY2tzLFxuICBTaGVsbEZhdWx0Qm91bmRhcnksXG4gIHNoZWxsUmVkdWNlcixcbiAgc2hvdWxkUGVyc2lzdEludHJvZHVjdGlvblNlZW4sXG4gIHNob3VsZFJlcGxheUludHJvZHVjdGlvbk9uTG9hZCxcbiAgdHlwZSBTcGFjZVBhbmVsU3RhdGUsXG4gIHR5cGUgU3BhY2VQcm9ncmFtRW50cnksXG4gIHR5cGUgU3Bhd25lZEFwcEVudHJ5LFxuICB0eXBlIFZpZXdNb2RlbCxcbn0gZnJvbSBcIi4uL1NoZWxsL/Cfn6bvuI9jb21wb25lbnQudHN4XCI7XG5pbXBvcnQge1xuICBiZWdpbkludGVyYWN0aXZlUGx1Z2luQWN0aW9uLFxuICBjbGVhclBlbmRpbmdXb3JsZFByb2plY3Rpb24sXG4gIGVuZEludGVyYWN0aXZlUGx1Z2luQWN0aW9uLFxuICBtYXBDb250ZXh0TWVudVNwZWNzLFxuICByZWdpc3RlclBlbmRpbmdXb3JsZFByb2plY3Rpb24sXG4gIFdpbmRvd0luc3RhbmNlSWRDb250ZXh0LFxufSBmcm9tIFwiLi4vV29ybGQzZEhvc3Qv8J+fpu+4j2NvbXBvbmVudC50c3hcIjtcbmltcG9ydCB7XG4gIERFRkFVTFRfUEFORUxfV0lEVEhfUFgsXG4gIEVNUFRZX0FQUF9MQUJFTFNfT1ZFUkxBWSxcbiAgRlJBTUVXT1JLX0NBVEVHT1JZX0NPTU1BTkRfSUQsXG4gIEZSQU1FV09SS19DQVRFR09SWV9ESVNQTEFZX0lELFxuICBGUkFNRVdPUktfQ0FURUdPUllfVE9PTF9JRCxcbiAgRlJBTUVXT1JLX1JFU0VSVkVEX0FDVElPTl9JRFMsXG4gIExBWU9VVF9DSEFOR0VfU0VUVExFX01TLFxuICBOT1RFX1dPUkxEX05BVklHQVRJT05fQUNUSU9OX0lELFxuICBQQU5FTF9UQUJfQkFSX0hPU1RTLFxuICBQUkVTRU5DRV9IRUFSVEJFQVRfSU5URVJWQUxfTVMsXG4gIFRVVE9SSUFMX1JFQ09SRElOR19FWENMVURFRF9BQ1RJT05fSURTLFxuICBhY3Rpb25DYXRlZ29yeUlkLFxuICBhY3Rpb25SZXF1aXJlc1N0YWdlZEZvcm0sXG4gIGFwcERvY3VtZW50TGFiZWwsXG4gIGFwcFdpbmRvd0RvY3VtZW50TGFiZWwsXG4gIGFwcGx5RnJhbWV3b3JrTGF5b3V0U2VlZCxcbiAgYXBwbHlUdXRvcmlhbFVpQ2hhbmdlVG9TaGVsbCxcbiAgYXBwbHlUdXRvcmlhbFVpU25hcHNob3RUb1NoZWxsLFxuICBhcHBseVVpUmVmcmVzaFJlc3BvbnNlVG9DYWNoZSxcbiAgYnVpbGRBY3RpdmVVdGlsaXR5QnlXaW5kb3dJZCxcbiAgYnVpbGRDb21tYW5kQ2F0ZWdvcnlUYWJzLFxuICBidWlsZE5vdGVTaGVsbENvbW1hbmRBY3Rpb24sXG4gIGJ1aWxkT3NDb21tYW5kcyxcbiAgYnVpbGRTcGFjZVBhbmVsU3RhdGUsXG4gIGJ1aWxkVG9vbFRhYnMsXG4gIGJ1aWxkVWlSZWZyZXNoUmVxdWVzdCxcbiAgY2FwdHVyZUN1cnJlbnRGcmFtZXdvcmtMYXlvdXQsXG4gIGNhcHR1cmVUdXRvcmlhbFVpU25hcHNob3QsXG4gIGNhdGVnb3J5VGFiSWNvbixcbiAgY2xhc3NpZnlXaW5kb3dMYXlvdXRDaGFuZ2UsXG4gIGNvbW1hbmRDYXRlZ29yaWVzLFxuICBjb21tYW5kQ2F0ZWdvcnlMYWJlbCxcbiAgZGlzcGF0Y2hPcGVuZWRGaWxlcyxcbiAgZGlzcGF0Y2hPc0NvbW1hbmQsXG4gIGRvd25sb2FkRGF0YVVybCxcbiAgZG93bmxvYWRNZWRpYUV4cG9ydCxcbiAgZmxhdHRlblBhbmVsVGFiTGVhdmVzLFxuICBpbnRyb2R1Y3Rpb25UYXJnZXRzV2luZG93LFxuICBsb2FkUGx1Z2luTW9kdWxlUmVzaWxpZW50LFxuICBtYWtlRWZmZWN0RGlzcGF0Y2hPbmUsXG4gIG1lcmdlUmVjb3JkUHJlc2VydmluZ0lkZW50aXR5LFxuICBwYW5lbEFuY2hvckZvckdyb3VwLFxuICBwYW5lbEpzb25Gcm9tU3RhdGUsXG4gIHBhbmVsVGFiRGVmaW5pdGlvblRvTm9kZSxcbiAgcGFyc2VQYW5lbFN0YXRlLFxuICBwYXJzZVNoZWxsUm91dGUsXG4gIHBhdGNoRG9jdW1lbnRUcmVlU2VsZWN0ZWRJZHMsXG4gIHBhdGNoV29ybGQzZENocm9tZU9udG9Ob2RlLFxuICBwcmVzZW5jZUNsaWVudElkZW50aXR5LFxuICBwcmVzZXJ2ZUpzb25JZGVudGl0eSxcbiAgcmVuZGVyU3RhZ2VkQXJnQ29udHJvbCxcbiAgcmVxdWVzdEZpbGVPcGVuLFxuICByZXNvbHZlQXBwRG9jdW1lbnQsXG4gIHJlc29sdmVBcHBMYWJlbCxcbiAgcmVzb2x2ZUNhbnZhc0JvZHlLZXksXG4gIHJlc29sdmVDb21tYW5kcyxcbiAgcmVzb2x2ZURpYWxvZ0RlZmluaXRpb24sXG4gIHJlc29sdmVEb2N1bWVudEJ5QXBwSWQsXG4gIHJlc29sdmVGcmFtZXdvcmtMYXlvdXRTZWVkLFxuICByZXNvbHZlSW50cm9kdWN0aW9uRGVmaW5pdGlvbixcbiAgcmVzb2x2ZUtleWJpbmRpbmdJbnRlbnQsXG4gIHJlc29sdmVNYW5pZmVzdExhYmVsLFxuICByZXNvbHZlUGFuZWxUYWJMYWJlbCxcbiAgcmVzb2x2ZVV0aWxpdHlBY3RpdmF0aW9uLFxuICByZXNvbHZlVXRpbGl0eU5vZGVzLFxuICByZXNvbHZlV2luZG93RW5nYWdlbWVudCxcbiAgcmV0aXRsZVdpbmRvd0xheW91dE5vZGUsXG4gIHJ1blJlcXVlc3RNZWRpYUZyYW1lcyxcbiAgc2NoZWR1bGVEaXNwYXRjaEFjdGlvbixcbiAgc2Vzc2lvbldpbmRvd0luc3RhbmNlcyxcbiAgc2hlbGxMYWJlbCxcbiAgc2hlbGxUYWJJY29uLFxuICBzcGF3bmVkV2luZG93Q2hyb21lRm9yS2luZCxcbiAgc3R1ZGlvUGFuZWxGb2N1c2luZ1NwYXduZWQsXG4gIHN5bmNEb2N1bWVudElkLFxuICBzeW50aGVzaXplTG9jYWxpemVkTGFiZWwsXG4gIHRvb2xJZEZyb21QYW5lbFRhYklkLFxuICB1c2VVSUhpc3RvcnksXG4gIHV0aWxpdHlCYXJOb2RlLFxuICB1dGlsaXR5Tm9kZVRyZWVDb250YWluc0lkLFxuICB2aWV3U3RhdGVXaXRoU3BhY2VQYW5lbCxcbiAgd2luZG93QWN0aW9uUGFuZU5vZGUsXG4gIHdpbmRvd0VuZ2FnZW1lbnRUb1NlYXJjaFNwZWMsXG4gIHdpbmRvd0VuZ2FnZW1lbnRUb1NwZWMsXG4gIHdpbmRvd01lYXN1cmVUcmVlQ29udGFpbnNJZCxcbiAgd2luZG93TWVhc3VyZXNDaHJvbWUsXG4gIHR5cGUgUmVzb2x2ZWRDb21tYW5kLFxuICB0eXBlIFVpUmVmcmVzaENhY2hlLFxufSBmcm9tIFwiLi4vU2hlbGxIZWxwZXJzL/Cfn6bvuI9jb21wb25lbnQudHN4XCI7XG5cbmltcG9ydCB7IGFQcm9qZWN0T2ZMdWhVZGtGb290ZXJJdGVtLCBmdW5kZWRCeVp1a3VuZnRCYXVGb290ZXJJdGVtIH0gZnJvbSBcIi4uLy4uLy4uLy4uLy4uLy4uLy4uLy4uL+KZu++4j21pdC1iZXN0YW5kL/Cfp7rvuI9kZW1vbnN0cmF0b3Iv4pqb77iPZm9vdGVyLnRzeFwiO1xuaW1wb3J0IHsgRU5UV0VSRkVOX01JVF9CRVNUQU5EX0JSQU5EX0lEUyB9IGZyb20gXCIuLi8uLi8uLi8uLi8uLi8uLi8uLi8uLi/imbvvuI9taXQtYmVzdGFuZC/wn6e677iPZGVtb25zdHJhdG9yL/Cfn6bvuI9icmFuZC50c1wiO1xuaW1wb3J0IHsgY3JlYXRlRnJhbWV3b3JrRGlzcGxheVBhbmVsVGFicywgY3JlYXRlRnJhbWV3b3JrUGx1Z2luc1BhbmVsVGFicywgY3JlYXRlRnJhbWV3b3JrU2V0dGluZ3NQYW5lbFRhYnMsIHR5cGUgRGlzcGxheUhvc3RBcGksIFBsdWdpblJlY292ZXJ5UGFuZWwsIHR5cGUgUGx1Z2luc0hvc3RBcGksIHR5cGUgUGx1Z2luc1BhbmVsRW50cnksIHR5cGUgU2V0dGluZ3NIb3N0QXBpLCBTaGVsbFJvdXRlTm90Rm91bmRQYWdlLCB1c2VOYW1lZExheW91dEhvc3QgfSBmcm9tIFwiLi4vQ2hyb21lUGFuZWxzL/Cfn6bvuI9jb21wb25lbnQudHN4XCI7XG5pbXBvcnQgeyB0eXBlIFBsdWdpbldhc21IYW5kbGUgfSBmcm9tIFwiLi4vUGx1Z2luUnVudGltZS/wn5+m77iPY29tcG9uZW50LnRzeFwiO1xuXG5pbXBvcnQgeyBTeW5jQXR0YWNoQ2FyZCB9IGZyb20gXCIuLi9TaGVsbFN5bmMv8J+fpu+4j2NvbXBvbmVudC50c3hcIjtcbmltcG9ydCB7IFVJRmluZCwgVUlGaW5kUHJvdmlkZXIsIFVJU2VhcmNoLCB0eXBlIFVJU2VhcmNoSXRlbSB9IGZyb20gXCIuLi9TaGVsbFNlYXJjaC/wn5+m77iPY29tcG9uZW50LnRzeFwiO1xuaW1wb3J0IHsgVVRJTElUWV9DQVRFR09SWV9JQ09OX0lEIH0gZnJvbSBcIi4uL1V0aWxpdHlUcmVlL/Cfn6bvuI9jb21wb25lbnQudHN4XCI7XG5pbXBvcnQgeyBjb2VyY2VXaXJlQnl0ZXMgfSBmcm9tIFwiLi4vUGx1Z2luUnVudGltZS/wn5+m77iPY29tcG9uZW50LnRzeFwiO1xuLy8gI2VuZHJlZ2lvbiDwn5SM77iPQWRhcHRlcnNcblxuLy8jcmVnaW9uIEZyYW1ld29ya09zU2hlbGxcbi8qKiBAZW1vamkg8J+Pt++4jyBMZXRzIGEgcGVyLXdpbmRvdyBob3N0IHJld3JpdGUgaXRzIE1vZGUgd2luZG93IHRpdGxlIChlLmcuIGxpdmUgcHJvamVjdGlvbiBsYWJlbCkuICovXG5leHBvcnQgY29uc3QgU2V0V2luZG93VGl0bGVDb250ZXh0ID0gY3JlYXRlQ29udGV4dDwoKHdpbmRvd0lkOiBzdHJpbmcsIHRpdGxlOiBzdHJpbmcpID0+IHZvaWQpIHwgbnVsbD4obnVsbCk7XG5cbi8qKiBAZW1vamkg8J+WvO+4jyBMZXRzIGEgcGVyLXdpbmRvdyBob3N0IHJld3JpdGUgaXRzIE1vZGUgd2luZG93IGljb24gKGUuZy4gbGl2ZSBwcm9qZWN0aW9uIGdseXBoKS4gKi9cbmV4cG9ydCBjb25zdCBTZXRXaW5kb3dJY29uQ29udGV4dCA9IGNyZWF0ZUNvbnRleHQ8KCh3aW5kb3dJZDogc3RyaW5nLCBpY29uSWQ6IEljb25OYW1lKSA9PiB2b2lkKSB8IG51bGw+KG51bGwpO1xuXG5jb25zdCBFTVBUWV9LRVlTX0JZX0FDVElPTl9JRCA9IG5ldyBNYXA8c3RyaW5nLCBzdHJpbmc+KCk7XG5cbi8qKiBAZW1vamkg4oyo77iPIExhc3Qtd2lucyBhcHAga2V5YmluZGluZ3MgZm9yIGVucmljaGluZyBjb250ZXh0LW1lbnUgc2hvcnRjdXQgbGFiZWxzIGluIHNjZW5lIGhvc3RzLiAqL1xuY29uc3QgQXBwS2V5YmluZGluZ3NDb250ZXh0ID0gY3JlYXRlQ29udGV4dDxSZWFkb25seU1hcDxzdHJpbmcsIHN0cmluZz4+KEVNUFRZX0tFWVNfQllfQUNUSU9OX0lEKTtcblxuLyoqIEBlbW9qaSDijKjvuI8gUmVzb2x2ZXMgYWN0aW9u4oaSa2V5cyBiaW5kaW5ncyBmcm9tIHRoZSBuZWFyZXN0IHtAbGluayBBcHBLZXliaW5kaW5nc0NvbnRleHR9IHByb3ZpZGVyLiAqL1xuZXhwb3J0IGZ1bmN0aW9uIHVzZUFwcEtleWJpbmRpbmdzQnlBY3Rpb25JZCgpOiBSZWFkb25seU1hcDxzdHJpbmcsIHN0cmluZz4ge1xuICByZXR1cm4gdXNlQ29udGV4dChBcHBLZXliaW5kaW5nc0NvbnRleHQpO1xufVxuXG4vKiogQGVtb2ppIPCflrHvuI8gTWFwcyBwcm9ncmFtIGNvbnRleHQtbWVudSBzcGVjcyB3aXRoIGFwcCBrZXliaW5kaW5nIHNob3J0Y3V0IGVucmljaG1lbnQuICovXG5leHBvcnQgZnVuY3Rpb24gdXNlTWFwQ29udGV4dE1lbnVTcGVjcyhkaXNwYXRjaDogKGFjdGlvbjogc3RyaW5nLCBhcmdzPzogUmVjb3JkPHN0cmluZywgdW5rbm93bj4pID0+IHZvaWQpIHtcbiAgY29uc3Qga2V5c0J5QWN0aW9uSWQgPSB1c2VBcHBLZXliaW5kaW5nc0J5QWN0aW9uSWQoKTtcbiAgcmV0dXJuIHVzZUNhbGxiYWNrKChzcGVjczogcmVhZG9ubHkgQ29udGV4dE1lbnVJdGVtU3BlY1tdKSA9PiBtYXBDb250ZXh0TWVudVNwZWNzKHNwZWNzLCBkaXNwYXRjaCwga2V5c0J5QWN0aW9uSWQpLCBbZGlzcGF0Y2gsIGtleXNCeUFjdGlvbklkXSk7XG59XG5cbi8vI3JlZ2lvbiDwn46l77iPVHV0b3JpYWxPdmVybGF5SG9zdHNcbi8qKiBAZW1vamkg8J+Tpu+4jyBSZXNvbHZlcyBhIGBUdXRvcmlhbEFzc2V0U3JjYCB0byBhIHZhbHVlIHVzYWJsZSBhcyBhbiBgPHZpZGVvPmAvYDxhdWRpbz5gIGBzcmNgIOKAlCBgQmxvYmAgKGFcbiAqIHN0dWRpbyBgQmxvYlN0b3JlYCByZWZlcmVuY2UpIGlzbid0IHJlc29sdmFibGUgZnJvbSB0aGlzIHNjb3BlIChubyBibG9iLXN0b3JlIGJyaWRnZSBoZXJlKSBhbmQgcmV0dXJuc1xuICogYG51bGxgIHdpdGggYSBjb25zb2xlIHdhcm5pbmc7IGBVcmxgL2BEYXRhVXJsYCByZXNvbHZlIGRpcmVjdGx5LiAqL1xuZnVuY3Rpb24gdHV0b3JpYWxBc3NldFNyY1RvVXJsKHNyYzogVHV0b3JpYWxBc3NldFNyYyk6IHN0cmluZyB8IG51bGwge1xuICBpZiAoc3JjLmtpbmQgPT09IFwidXJsXCIpIHJldHVybiBzcmMudXJsO1xuICBpZiAoc3JjLmtpbmQgPT09IFwiZGF0YVVybFwiKSByZXR1cm4gc3JjLmRhdGE7XG4gIGNvbnNvbGUud2FybihcIltERUJVR10gdHV0b3JpYWwgYmxvYiBhc3NldCBzcmMgbm90IHJlc29sdmFibGUgaW4gdGhpcyBzY29wZVwiLCBzcmMuaGFzaCk7XG4gIHJldHVybiBudWxsO1xufVxuXG4vKiogQGVtb2ppIPCfkqzvuI8gU2VsZi1zdWJzY3JpYmVzIHRvIHRoZSB0dXRvcmlhbCBjbG9jayAoc2VlIGB1c2VUdXRvcmlhbENsb2NrYCkgc28gb25seSBUSElTIGxlYWYgcmUtcmVuZGVycyBldmVyeSBmcmFtZSDigJQgbmV2ZXIgdGhlIHdob2xlIHNoZWxsIOKAlCBtaXJyb3JpbmcgYFR1dG9yaWFsQmFyYCdzIG93biBzdWJzY3JpcHRpb24uICovXG5jb25zdCBUdXRvcmlhbENhcHRpb25zSG9zdDogUmVhY3QuRkM8eyByZWFkb25seSB0dXRvcmlhbDogVHV0b3JpYWxEZWZpbml0aW9uOyByZWFkb25seSBjbG9jazogVHV0b3JpYWxDbG9ja1BvcnQ7IHJlYWRvbmx5IGNhcHRpb25zT246IGJvb2xlYW47IHJlYWRvbmx5IHRlcm1pbm9sb2d5OiBzdHJpbmc7IHJlYWRvbmx5IGxvY2FsZTogc3RyaW5nIH0+ID0gKHsgdHV0b3JpYWwsIGNsb2NrLCBjYXB0aW9uc09uLCB0ZXJtaW5vbG9neSwgbG9jYWxlIH0pID0+IHtcbiAgY29uc3QgdGltZU1zID0gdXNlVHV0b3JpYWxDbG9jayhjbG9jayk7XG4gIGNvbnN0IGN1ZSA9IHR1dG9yaWFsQ3Vlc0JldHdlZW4odHV0b3JpYWwudHJhY2tzLm5hcnJhdGlvbiwgdGltZU1zKVswXSA/PyBudWxsO1xuICByZXR1cm4gPFR1dG9yaWFsQ2FwdGlvbnMgdGV4dD17Y3VlID8gcmVzb2x2ZU1hbmlmZXN0TGFiZWwoY3VlLnRleHQsIHRlcm1pbm9sb2d5LCBsb2NhbGUpIDogbnVsbH0gdmlzaWJsZT17Y2FwdGlvbnNPbn0gLz47XG59O1xuXG5jb25zdCBUVVRPUklBTF9ERUZBVUxUX1ZJREVPX1JFQ1QgPSB7IHg6IDAuNzIsIHk6IDAuNywgd2lkdGg6IDAuMjQsIGhlaWdodDogMC4yNCB9IGFzIGNvbnN0O1xuXG4vKiogQGVtb2ppIPCfk7nvuI8gU2VsZi1zdWJzY3JpYmVzIHRvIHRoZSB0dXRvcmlhbCBjbG9jazsgcmVzb2x2ZXMgdGhlIGNvdmVyaW5nIGBUdXRvcmlhbFZpZGVvQ3VlYCAoaWYgYW55KSBhbmQgaXRzIHNvdXJjZS1yZWxhdGl2ZSBsb2NhbCB0aW1lLiAqL1xuY29uc3QgVHV0b3JpYWxWaWRlb092ZXJsYXlIb3N0OiBSZWFjdC5GQzx7IHJlYWRvbmx5IHR1dG9yaWFsOiBUdXRvcmlhbERlZmluaXRpb247IHJlYWRvbmx5IGNsb2NrOiBUdXRvcmlhbENsb2NrUG9ydDsgcmVhZG9ubHkgbXV0ZWQ6IGJvb2xlYW47IHJlYWRvbmx5IHBsYXlpbmc6IGJvb2xlYW47IHJlYWRvbmx5IHJhdGU6IG51bWJlciB9PiA9ICh7XG4gIHR1dG9yaWFsLFxuICBjbG9jayxcbiAgbXV0ZWQsXG4gIHBsYXlpbmcsXG4gIHJhdGUsXG59KSA9PiB7XG4gIGNvbnN0IHRpbWVNcyA9IHVzZVR1dG9yaWFsQ2xvY2soY2xvY2spO1xuICBjb25zdCBjdWU6IFR1dG9yaWFsVmlkZW9DdWUgfCBudWxsID0gdHV0b3JpYWxDdWVzQmV0d2Vlbih0dXRvcmlhbC50cmFja3MudmlkZW8sIHRpbWVNcylbMF0gPz8gbnVsbDtcbiAgY29uc3Qgc3JjID0gY3VlID8gdHV0b3JpYWxBc3NldFNyY1RvVXJsKGN1ZS5zcmMpIDogbnVsbDtcbiAgY29uc3QgbG9jYWxUaW1lTXMgPSBjdWUgPyB0aW1lTXMgLSBjdWUuYXQgKyBjdWUuc291cmNlT2Zmc2V0TXMgOiAwO1xuICByZXR1cm4gPFR1dG9yaWFsVmlkZW9PdmVybGF5IHNyYz17c3JjfSByZWN0PXtjdWU/LnJlY3QgPz8gVFVUT1JJQUxfREVGQVVMVF9WSURFT19SRUNUfSBtdXRlZD17bXV0ZWQgfHwgKGN1ZT8ubXV0ZWQgPz8gZmFsc2UpfSBwbGF5aW5nPXtwbGF5aW5nfSByYXRlPXtyYXRlfSBsb2NhbFRpbWVNcz17bG9jYWxUaW1lTXN9IC8+O1xufTtcblxuLyoqIEBlbW9qaSDwn5G777iPIFNlbGYtc3Vic2NyaWJlcyB0byB0aGUgdHV0b3JpYWwgY2xvY2s7IHJlc29sdmVzIHRoZSBjb3ZlcmluZyBgVHV0b3JpYWxHZXN0dXJlQ3VlYCAoaWYgYW55KSBhbmQgcHJvZ3Jlc3MgKDDigJMxKSB0aHJvdWdoIGl0LCBkcml2aW5nIGBUdXRvcmlhbEdob3N0UG9pbnRlcmAgb2ZmIHRoZSBQTEFZSEVBRCByYXRoZXIgdGhhbiBpdHMgb3duIGludGVybmFsIGNsb2NrICh1bmxpa2UgdGhlIGludHJvZHVjdGlvbiBkZW1vbnN0cmF0aW9uIG92ZXJsYXkpLiAqL1xuY29uc3QgVHV0b3JpYWxHaG9zdFBvaW50ZXJIb3N0OiBSZWFjdC5GQzx7IHJlYWRvbmx5IHR1dG9yaWFsOiBUdXRvcmlhbERlZmluaXRpb247IHJlYWRvbmx5IGNsb2NrOiBUdXRvcmlhbENsb2NrUG9ydCB9PiA9ICh7IHR1dG9yaWFsLCBjbG9jayB9KSA9PiB7XG4gIGNvbnN0IHRpbWVNcyA9IHVzZVR1dG9yaWFsQ2xvY2soY2xvY2spO1xuICBjb25zdCBjdWU6IFR1dG9yaWFsR2VzdHVyZUN1ZSB8IG51bGwgPSB0dXRvcmlhbEN1ZXNCZXR3ZWVuKHR1dG9yaWFsLnRyYWNrcy5nZXN0dXJlcywgdGltZU1zKVswXSA/PyBudWxsO1xuICBjb25zdCBwcm9ncmVzcyA9IGN1ZSA/IE1hdGgubWluKDEsIE1hdGgubWF4KDAsICh0aW1lTXMgLSBjdWUuYXQpIC8gTWF0aC5tYXgoY3VlLmR1cmF0aW9uTXMsIDEpKSkgOiAwO1xuICByZXR1cm4gPFR1dG9yaWFsR2hvc3RQb2ludGVyIGN1ZT17Y3VlfSBwcm9ncmVzcz17cHJvZ3Jlc3N9IC8+O1xufTtcbi8vI2VuZHJlZ2lvbiDwn46l77iPVHV0b3JpYWxPdmVybGF5SG9zdHNcblxuLy8jcmVnaW9uIPCfjqXvuI9UdXRvcmlhbFJlY29yZGVyXG4vKiogQGVtb2ppIOKGlCBGaWVsZC1ieS1maWVsZCBzdHJ1Y3R1cmFsIGRpZmYgb2YgdHdvIGBUdXRvcmlhbFVpU25hcHNob3RgcyBpbnRvIHRoZSBzcGFyc2UgYFR1dG9yaWFsVWlDaGFuZ2VgXG4gKiBhbHBoYWJldCDigJQgdGhlIHJlY29yZGVyJ3MgVUktZGlmZiBlZmZlY3QgY2FsbHMgdGhpcyBldmVyeSBgU2hlbGxTdGF0ZWAgY2hhbmdlIHdoaWxlIGFybWVkLiAqL1xuZnVuY3Rpb24gZGlmZlR1dG9yaWFsVWlTbmFwc2hvdChwcmV2OiBUdXRvcmlhbFVpU25hcHNob3QsIG5leHQ6IFR1dG9yaWFsVWlTbmFwc2hvdCk6IFR1dG9yaWFsVWlDaGFuZ2VbXSB7XG4gIGNvbnN0IGNoYW5nZXM6IFR1dG9yaWFsVWlDaGFuZ2VbXSA9IFtdO1xuICBpZiAocHJldi5hY3RpdmVNb2RlSWQgIT09IG5leHQuYWN0aXZlTW9kZUlkICYmIG5leHQuYWN0aXZlTW9kZUlkICE9IG51bGwpIGNoYW5nZXMucHVzaCh7IGtpbmQ6IFwiYWN0aXZlTW9kZVwiLCBpZDogbmV4dC5hY3RpdmVNb2RlSWQgfSk7XG4gIGlmIChwcmV2LmZvY3VzZWRXaW5kb3dJZCAhPT0gbmV4dC5mb2N1c2VkV2luZG93SWQpIGNoYW5nZXMucHVzaCh7IGtpbmQ6IFwiZm9jdXNlZFdpbmRvd1wiLCBpZDogbmV4dC5mb2N1c2VkV2luZG93SWQgfSk7XG4gIGNvbnN0IHV0aWxpdHlXaW5kb3dJZHMgPSBuZXcgU2V0KFsuLi5PYmplY3Qua2V5cyhwcmV2LmFjdGl2ZVV0aWxpdHlCeVdpbmRvd0lkKSwgLi4uT2JqZWN0LmtleXMobmV4dC5hY3RpdmVVdGlsaXR5QnlXaW5kb3dJZCldKTtcbiAgZm9yIChjb25zdCB3aW5kb3dJZCBvZiB1dGlsaXR5V2luZG93SWRzKSB7XG4gICAgaWYgKHByZXYuYWN0aXZlVXRpbGl0eUJ5V2luZG93SWRbd2luZG93SWRdICE9PSBuZXh0LmFjdGl2ZVV0aWxpdHlCeVdpbmRvd0lkW3dpbmRvd0lkXSkgY2hhbmdlcy5wdXNoKHsga2luZDogXCJhY3RpdmVVdGlsaXR5XCIsIHdpbmRvd0lkLCB1dGlsaXR5SWQ6IG5leHQuYWN0aXZlVXRpbGl0eUJ5V2luZG93SWRbd2luZG93SWRdIH0pO1xuICB9XG4gIGlmIChwcmV2LmFjdGl2ZVRvb2xJZCAhPT0gbmV4dC5hY3RpdmVUb29sSWQpIGNoYW5nZXMucHVzaCh7IGtpbmQ6IFwiYWN0aXZlVG9vbFwiLCBpZDogbmV4dC5hY3RpdmVUb29sSWQgfSk7XG4gIGlmIChuZXh0LmxheW91dCAmJiBKU09OLnN0cmluZ2lmeShwcmV2LmxheW91dCkgIT09IEpTT04uc3RyaW5naWZ5KG5leHQubGF5b3V0KSkgY2hhbmdlcy5wdXNoKHsga2luZDogXCJsYXlvdXRcIiwgbGF5b3V0OiBuZXh0LmxheW91dCB9KTtcbiAgY29uc3QgZ3JvdXBzID0gbmV3IFNldChbLi4uT2JqZWN0LmtleXMocHJldi5hY3RpdmVQYW5lbFRhYkJ5R3JvdXApLCAuLi5PYmplY3Qua2V5cyhuZXh0LmFjdGl2ZVBhbmVsVGFiQnlHcm91cCldKTtcbiAgZm9yIChjb25zdCBncm91cCBvZiBncm91cHMpIHtcbiAgICBpZiAocHJldi5hY3RpdmVQYW5lbFRhYkJ5R3JvdXBbZ3JvdXBdICE9PSBuZXh0LmFjdGl2ZVBhbmVsVGFiQnlHcm91cFtncm91cF0pIGNoYW5nZXMucHVzaCh7IGtpbmQ6IFwicGFuZWxUYWJcIiwgZ3JvdXAsIHRhYklkOiBuZXh0LmFjdGl2ZVBhbmVsVGFiQnlHcm91cFtncm91cF0gfSk7XG4gIH1cbiAgaWYgKG5leHQucGFuZWxKc29uICE9IG51bGwgJiYgcHJldi5wYW5lbEpzb24gIT09IG5leHQucGFuZWxKc29uKSBjaGFuZ2VzLnB1c2goeyBraW5kOiBcInBhbmVsU3RhdGVcIiwgcGFuZWxKc29uOiBuZXh0LnBhbmVsSnNvbiB9KTtcbiAgaWYgKG5leHQuc2VsZWN0aW9uSnNvbiAhPSBudWxsICYmIHByZXYuc2VsZWN0aW9uSnNvbiAhPT0gbmV4dC5zZWxlY3Rpb25Kc29uKSBjaGFuZ2VzLnB1c2goeyBraW5kOiBcInNlbGVjdGlvblwiLCBzZWxlY3Rpb25Kc29uOiBuZXh0LnNlbGVjdGlvbkpzb24gfSk7XG4gIGlmIChwcmV2Lm9wZW5EaWFsb2dJZCAhPT0gbmV4dC5vcGVuRGlhbG9nSWQpIGNoYW5nZXMucHVzaCh7IGtpbmQ6IFwiZGlhbG9nXCIsIGlkOiBuZXh0Lm9wZW5EaWFsb2dJZCB9KTtcbiAgY29uc3QgcHJldlRyZWUgPSBuZXcgU2V0KHByZXYuZXhwYW5kZWRUcmVlSWRzKTtcbiAgY29uc3QgbmV4dFRyZWUgPSBuZXcgU2V0KG5leHQuZXhwYW5kZWRUcmVlSWRzKTtcbiAgZm9yIChjb25zdCBpZCBvZiBuZXh0VHJlZSkgaWYgKCFwcmV2VHJlZS5oYXMoaWQpKSBjaGFuZ2VzLnB1c2goeyBraW5kOiBcInRyZWVFeHBhbnNpb25cIiwgaWQsIGV4cGFuZGVkOiB0cnVlIH0pO1xuICBmb3IgKGNvbnN0IGlkIG9mIHByZXZUcmVlKSBpZiAoIW5leHRUcmVlLmhhcyhpZCkpIGNoYW5nZXMucHVzaCh7IGtpbmQ6IFwidHJlZUV4cGFuc2lvblwiLCBpZCwgZXhwYW5kZWQ6IGZhbHNlIH0pO1xuICBpZiAocHJldi5jb21tYW5kUGFuZWxPcGVuICE9PSBuZXh0LmNvbW1hbmRQYW5lbE9wZW4pIGNoYW5nZXMucHVzaCh7IGtpbmQ6IFwiY29tbWFuZFBhbmVsXCIsIG9wZW46IG5leHQuY29tbWFuZFBhbmVsT3BlbiB9KTtcbiAgcmV0dXJuIGNoYW5nZXM7XG59XG5cbi8qKiBAZW1vamkg8J+Ope+4jyBFcHNpbG9uLWVxdWFsaXR5IGZvciB0d28gY2FtZXJhIHBvc2VzIOKAlCB0aGUgcmVjb3JkZXIncyAxMEh6IGNhbWVyYSBzYW1wbGVyIHNraXBzIHdyaXRpbmcgYVxuICogbmV3IGtleWZyYW1lIHdoZW4gdGhlIGxpdmUgcG9zZSBoYXNuJ3QgbWVhbmluZ2Z1bGx5IG1vdmVkIHNpbmNlIHRoZSBsYXN0IHNhbXBsZS4gKi9cbmZ1bmN0aW9uIHR1dG9yaWFsQ2FtZXJhUG9zZUVxdWFscyhhOiBUdXRvcmlhbENhbWVyYVN0YXRlLCBiOiBUdXRvcmlhbENhbWVyYVN0YXRlKTogYm9vbGVhbiB7XG4gIGlmIChhLmtpbmQgIT09IGIua2luZCkgcmV0dXJuIGZhbHNlO1xuICBpZiAoYS5raW5kID09PSBcIm9yYml0XCIgJiYgYi5raW5kID09PSBcIm9yYml0XCIpIHJldHVybiBhLnBvc2l0aW9uLmV2ZXJ5KCh2YWx1ZSwgaW5kZXgpID0+IE1hdGguYWJzKHZhbHVlIC0gYi5wb3NpdGlvbltpbmRleF0pIDwgMWUtNCkgJiYgYS50YXJnZXQuZXZlcnkoKHZhbHVlLCBpbmRleCkgPT4gTWF0aC5hYnModmFsdWUgLSBiLnRhcmdldFtpbmRleF0pIDwgMWUtNCk7XG4gIGlmIChhLmtpbmQgPT09IFwiY2FudmFzXCIgJiYgYi5raW5kID09PSBcImNhbnZhc1wiKSByZXR1cm4gTWF0aC5hYnMoYS54IC0gYi54KSA8IDFlLTQgJiYgTWF0aC5hYnMoYS55IC0gYi55KSA8IDFlLTQgJiYgTWF0aC5hYnMoYS56b29tIC0gYi56b29tKSA8IDFlLTQ7XG4gIHJldHVybiBmYWxzZTtcbn1cblxuLyoqIEBlbW9qaSDwn46l77iPIENhcHR1cmVzIGEgbGl2ZSBzZXNzaW9uIGludG8gYSBgVHV0b3JpYWxEZWZpbml0aW9uYCDigJQgYSByZWNvcmRpbmcgSVMgYSBgVHV0b3JpYWxEZWZpbml0aW9uYCxcbiAqIHNvIHRoaXMgY2xhc3Mgc2ltcGx5IGFjY3VtdWxhdGVzIGEgZGVuc2VseS1zYW1wbGVkIG9uZSAoc2VlIHRoZSBSdXN0IGNvcmUgZG9jIGNvbW1lbnQgb25cbiAqIGBUdXRvcmlhbERlZmluaXRpb25gKS4gRGVsaWJlcmF0ZWx5IHByb2R1Y2VzIGV2ZW50cy9VSS9jYW1lcmEvZG9jdW1lbnQgdHJhY2tzIG9ubHk6IHdlYmNhbS9taWMgY2FwdHVyZVxuICogKGBNZWRpYVJlY29yZGVyYCkgaXMgYW4gZXhwbGljaXQsIHJlcG9ydGVkIHNjb3BlIGN1dCDigJQgc2VlIHRoZSB0aWNrZXQgY2xvc2Utb3V0IHN1bW1hcnkg4oCUIGEgdGV4dC1vbmx5XG4gKiByZWNvcmRpbmcgaXMgc3RpbGwgYSBmdWxseSB2YWxpZCwgdXNlZnVsIGBUdXRvcmlhbERlZmluaXRpb25gIHBlciB0aGUgUnVzdCBtb2RlbCdzIG93biBvcHRpb25hbGl0eVxuICogKG5hcnJhdGlvbi92aWRlbyB0cmFja3MgZGVmYXVsdCB0byBlbXB0eSkuIERvY3VtZW50IGBFZGl0YCBvcGVyYXRpb25zIGFyZSBOT1QgY2FwdHVyZWQgKHRoYXQgd291bGRcbiAqIHJlcXVpcmUgaW50ZXJjZXB0aW5nIHRoZSBwbHVnaW4ncyBpbnRlcm5hbCB2Y3Mgb3BlcmF0aW9uIHN0cmVhbSBpbiBwZXItb3AgZm9ybSwgd2hpY2ggaXNuJ3QgZXhwb3NlZCB0b1xuICogdGhpcyBzaGVsbCkg4oCUIGFsc28gYSByZXBvcnRlZCBzY29wZSBjdXQ7IFVJL2NhbWVyYS9ldmVudHMgc3RpbGwgcmVwbGF5IGZhaXRoZnVsbHkuICovXG5leHBvcnQgY2xhc3MgVHV0b3JpYWxSZWNvcmRlciB7XG4gIHByaXZhdGUgcmVhZG9ubHkgc3RhcnRlZEF0TXM6IG51bWJlcjtcbiAgcHJpdmF0ZSByZWFkb25seSBiYXNlVWlTbmFwc2hvdDogVHV0b3JpYWxVaVNuYXBzaG90O1xuICBwcml2YXRlIHJlYWRvbmx5IGJhc2VEb2N1bWVudEpzb246IHN0cmluZyB8IG51bGw7XG4gIHByaXZhdGUgcmVhZG9ubHkgZXZlbnRzOiBUdXRvcmlhbEV2ZW50W10gPSBbXTtcbiAgcHJpdmF0ZSByZWFkb25seSB1aUtleWZyYW1lczogeyByZWFkb25seSBhdDogbnVtYmVyOyByZWFkb25seSBzYW1wbGU6IHsgcmVhZG9ubHkga2luZDogXCJzbmFwc2hvdFwiOyByZWFkb25seSBzdGF0ZTogVHV0b3JpYWxVaVNuYXBzaG90IH0gfCB7IHJlYWRvbmx5IGtpbmQ6IFwiZGVsdGFcIjsgcmVhZG9ubHkgY2hhbmdlczogVHV0b3JpYWxVaUNoYW5nZVtdIH0gfVtdID0gW107XG4gIHByaXZhdGUgcmVhZG9ubHkgY2FtZXJhS2V5ZnJhbWVzOiB7IHJlYWRvbmx5IGF0OiBudW1iZXI7IHJlYWRvbmx5IHdpbmRvd0lkOiBzdHJpbmc7IHJlYWRvbmx5IGNhbWVyYTogVHV0b3JpYWxDYW1lcmFTdGF0ZTsgcmVhZG9ubHkgZWFzaW5nOiBcImVhc2VJbk91dFwiIH1bXSA9IFtdO1xuICBwcml2YXRlIHJlYWRvbmx5IGNoYXB0ZXJzOiBUdXRvcmlhbENoYXB0ZXJbXSA9IFtdO1xuICBwcml2YXRlIGxhc3RVaVNuYXBzaG90OiBUdXRvcmlhbFVpU25hcHNob3Q7XG4gIHByaXZhdGUgcmVhZG9ubHkgbGFzdENhbWVyYUJ5V2luZG93ID0gbmV3IE1hcDxzdHJpbmcsIFR1dG9yaWFsQ2FtZXJhU3RhdGU+KCk7XG5cbiAgY29uc3RydWN0b3IoYmFzZVVpU25hcHNob3Q6IFR1dG9yaWFsVWlTbmFwc2hvdCwgYmFzZURvY3VtZW50SnNvbjogc3RyaW5nIHwgbnVsbCkge1xuICAgIHRoaXMuc3RhcnRlZEF0TXMgPSBwZXJmb3JtYW5jZS5ub3coKTtcbiAgICB0aGlzLmJhc2VVaVNuYXBzaG90ID0gYmFzZVVpU25hcHNob3Q7XG4gICAgdGhpcy5sYXN0VWlTbmFwc2hvdCA9IGJhc2VVaVNuYXBzaG90O1xuICAgIHRoaXMuYmFzZURvY3VtZW50SnNvbiA9IGJhc2VEb2N1bWVudEpzb247XG4gIH1cblxuICBwcml2YXRlIG5vd01zKCk6IG51bWJlciB7XG4gICAgcmV0dXJuIE1hdGgubWF4KDAsIE1hdGgucm91bmQocGVyZm9ybWFuY2Uubm93KCkgLSB0aGlzLnN0YXJ0ZWRBdE1zKSk7XG4gIH1cblxuICByZWNvcmRFdmVudChraW5kOiBUdXRvcmlhbEV2ZW50W1wia2luZFwiXSk6IHZvaWQge1xuICAgIHRoaXMuZXZlbnRzLnB1c2goeyBhdDogdGhpcy5ub3dNcygpLCBraW5kIH0pO1xuICB9XG5cbiAgcmVjb3JkVWlEaWZmKG5leHQ6IFR1dG9yaWFsVWlTbmFwc2hvdCk6IHZvaWQge1xuICAgIGNvbnN0IGNoYW5nZXMgPSBkaWZmVHV0b3JpYWxVaVNuYXBzaG90KHRoaXMubGFzdFVpU25hcHNob3QsIG5leHQpO1xuICAgIGlmIChjaGFuZ2VzLmxlbmd0aCA+IDApIHRoaXMudWlLZXlmcmFtZXMucHVzaCh7IGF0OiB0aGlzLm5vd01zKCksIHNhbXBsZTogeyBraW5kOiBcImRlbHRhXCIsIGNoYW5nZXMgfSB9KTtcbiAgICB0aGlzLmxhc3RVaVNuYXBzaG90ID0gbmV4dDtcbiAgfVxuXG4gIHJlY29yZFNuYXBzaG90KHN0YXRlOiBUdXRvcmlhbFVpU25hcHNob3QpOiB2b2lkIHtcbiAgICB0aGlzLnVpS2V5ZnJhbWVzLnB1c2goeyBhdDogdGhpcy5ub3dNcygpLCBzYW1wbGU6IHsga2luZDogXCJzbmFwc2hvdFwiLCBzdGF0ZSB9IH0pO1xuICAgIHRoaXMubGFzdFVpU25hcHNob3QgPSBzdGF0ZTtcbiAgfVxuXG4gIHNhbXBsZUNhbWVyYSh3aW5kb3dJZDogc3RyaW5nLCBjYW1lcmE6IFR1dG9yaWFsQ2FtZXJhU3RhdGUpOiB2b2lkIHtcbiAgICBjb25zdCBwcmV2ID0gdGhpcy5sYXN0Q2FtZXJhQnlXaW5kb3cuZ2V0KHdpbmRvd0lkKTtcbiAgICBpZiAocHJldiAmJiB0dXRvcmlhbENhbWVyYVBvc2VFcXVhbHMocHJldiwgY2FtZXJhKSkgcmV0dXJuO1xuICAgIHRoaXMubGFzdENhbWVyYUJ5V2luZG93LnNldCh3aW5kb3dJZCwgY2FtZXJhKTtcbiAgICB0aGlzLmNhbWVyYUtleWZyYW1lcy5wdXNoKHsgYXQ6IHRoaXMubm93TXMoKSwgd2luZG93SWQsIGNhbWVyYSwgZWFzaW5nOiBcImVhc2VJbk91dFwiIH0pO1xuICB9XG5cbiAgLyoqIPCfk5bvuI8gYHVpLnR1dG9yaWFsLmFkZENoYXB0ZXJgIOKAlCBtYXJrcyB0aGUgY3VycmVudCBlbGFwc2VkIHRpbWUgYXMgYSBzY3J1Yi1iYXIgY2hhcHRlciB3aXRoIGFuXG4gICAqIGF1dG8tbnVtYmVyZWQgdGl0bGUgKG5vIG5hbWluZy1wcm9tcHQgVUkgaW4gdGhpcyBzY29wZTsgYSByZWNvcmRlZCB0dXRvcmlhbCdzIGF1dGhvcmVkIHRpdGxlcyBjYW5cbiAgICogYWx3YXlzIGJlIGhhbmQtZWRpdGVkIGluIHRoZSBkb3dubG9hZGVkIEpTT04gYWZ0ZXJ3YXJkKS4gU3ludGhlc2l6ZXMgYSBgTG9jYWxpemVkTGFiZWxgIG1hdHJpeC4gKi9cbiAgYWRkQ2hhcHRlcih0aXRsZT86IHN0cmluZyB8IExvY2FsaXplZExhYmVsKTogdm9pZCB7XG4gICAgY29uc3QgaW5kZXggPSB0aGlzLmNoYXB0ZXJzLmxlbmd0aCArIDE7XG4gICAgY29uc3QgcmF3VGl0bGUgPSB0aXRsZSA/PyBgQ2hhcHRlciAke2luZGV4fWA7XG4gICAgdGhpcy5jaGFwdGVycy5wdXNoKHsgaWQ6IGBjaGFwdGVyLSR7aW5kZXh9YCwgYXQ6IHRoaXMubm93TXMoKSwgdGl0bGU6IHN5bnRoZXNpemVMb2NhbGl6ZWRMYWJlbChyYXdUaXRsZSkgfSk7XG4gIH1cblxuICBidWlsZChpZDogc3RyaW5nLCB0aXRsZTogc3RyaW5nIHwgTG9jYWxpemVkTGFiZWwsIGV4YW1wbGVJZD86IHN0cmluZyk6IFR1dG9yaWFsRGVmaW5pdGlvbiB7XG4gICAgY29uc3QgZHVyYXRpb25NcyA9IE1hdGgubWF4KDEwMDAsIHRoaXMubm93TXMoKSk7XG4gICAgcmV0dXJuIHtcbiAgICAgIGlkLFxuICAgICAgdGl0bGU6IHN5bnRoZXNpemVMb2NhbGl6ZWRMYWJlbCh0aXRsZSksXG4gICAgICBkdXJhdGlvbk1zLFxuICAgICAgY2hhcHRlcnM6IHRoaXMuY2hhcHRlcnMsXG4gICAgICBiYXNlOiB7IGRvY3VtZW50SnNvbjogdGhpcy5iYXNlRG9jdW1lbnRKc29uID8/IHVuZGVmaW5lZCwgZXhhbXBsZUlkLCB1aTogdGhpcy5iYXNlVWlTbmFwc2hvdCwgY2FtZXJhczogW10gfSxcbiAgICAgIHRyYWNrczogeyBuYXJyYXRpb246IFtdLCB2aWRlbzogW10sIGV2ZW50czogdGhpcy5ldmVudHMsIHVpOiB0aGlzLnVpS2V5ZnJhbWVzLCBkb2N1bWVudDogW10sIGNhbWVyYTogdGhpcy5jYW1lcmFLZXlmcmFtZXMsIGdlc3R1cmVzOiBbXSB9LFxuICAgICAgcmVjb3JkZWRBdDogbmV3IERhdGUoKS50b0lTT1N0cmluZygpLFxuICAgIH07XG4gIH1cbn1cbi8vI2VuZHJlZ2lvbiDwn46l77iPVHV0b3JpYWxSZWNvcmRlclxuXG4vLyNyZWdpb24g8J+Qmu+4j1NoZWxsTW91bnRcbi8qKiBAZW1vamkg8J+Qmu+4jyBQdWJsaWMgcHJvcHMgZm9yIHtAbGluayBGcmFtZXdvcmtPc1NoZWxsfSDigJQgdGhlIG11bHRpLWluc3RhbmNlLXNhZmUgZW50cnkgcG9pbnQuIGBzaGVsbElkYCxcbiAqIGBzdG9yYWdlTmFtZXNwYWNlYCwgYW5kIGBvd25zUGFnZWAgZXhpc3Qgc28gc2V2ZXJhbCBzaGVsbHMgY2FuIGJlIG1vdW50ZWQgb24gb25lIHBhZ2U6IGBvd25zUGFnZWBcbiAqIGdhdGVzIHRoZSBoYW5kZnVsIG9mIGJlaGF2aW9ycyB0aGF0IGFyZSBsZWdpdGltYXRlbHkgcGFnZS1nbG9iYWwgKGRvY3VtZW50IHRpdGxlLCBicm93c2VyIGhpc3RvcnlcbiAqIHN5bmMgdmlhIGBib290RnJhbWV3b3JrT3NgKSwgYHN0b3JhZ2VOYW1lc3BhY2VgIHByZWZpeGVzIHRoaXMgc2hlbGwncyBkdXJhYmxlIHN0b3JhZ2Uga2V5cyBzb1xuICogY28tbW91bnRlZCBzaGVsbHMgZG9uJ3Qgc2hhcmUgYHNlbWlvLm9zLmRvY2tgL2B1aS5jaHJvbWUuKmAgc3RhdGUuICovXG5leHBvcnQgaW50ZXJmYWNlIEZyYW1ld29ya09zU2hlbGxQcm9wcyB7XG4gIHJlYWRvbmx5IHBsdWdpbkZpbHRlcj86IHN0cmluZztcbiAgcmVhZG9ubHkgcGx1Z2luczogcmVhZG9ubHkgeyByZWFkb25seSBwbHVnaW5JZDogc3RyaW5nOyByZWFkb25seSBtb2R1bGVVcmw6IHN0cmluZyB9W107XG4gIHJlYWRvbmx5IGFwcElkPzogc3RyaW5nO1xuICByZWFkb25seSBsb2Nrcz86IFJlc29sdmVkU2hlbGxMb2NrcztcbiAgcmVhZG9ubHkgZGVmYXVsdHM/OiBGcmFtZXdvcmtPc0RlZmF1bHRzO1xuICByZWFkb25seSBicmFuZD86IFNoZWxsQnJhbmQ7XG4gIHJlYWRvbmx5IHNoZWxsSWQ/OiBzdHJpbmc7XG4gIHJlYWRvbmx5IHN0b3JhZ2VOYW1lc3BhY2U/OiBzdHJpbmc7XG4gIHJlYWRvbmx5IG93bnNQYWdlPzogYm9vbGVhbjtcbiAgLyoqIPCfkJrvuI8gU2tpcHMgdGhlIGJyYW5kL2FwcCBpbnRyb2R1Y3Rpb24gYXV0by1zdGFydCAoYW5kIGFueSBicmFuZC1vd25lZCB0dXRvcmlhbCdzIG93biBhdXRvLWNvbnNpZGVyZWRcbiAgICogcmV2ZWFsKSBmb3IgYSBzaGVsbCB0aGF0J3MgbW91bnRlZCBidXQgbm90IHRoZSBvbmUgdGhlIHVzZXIgaXMgYWN0dWFsbHkgbG9va2luZyBhdCDigJQgYSBsaXZlXG4gICAqIG11bHRpLXNoZWxsIHBhZ2UgKGUuZy4gdGhlIG1pdC1iZXN0YW5kIGRlbW9uc3RyYXRvcidzIGJhY2tncm91bmQgcGFuZXMpIGhhcyBubyBpZnJhbWUgYm91bmRhcnkgZm9yXG4gICAqIHRoZSBleGlzdGluZyBgd2luZG93LnNlbGYgIT09IHdpbmRvdy50b3BgIGhldXJpc3RpYyBiZWxvdyB0byBrZXkgb2ZmLCBzbyBzZXZlcmFsIHNoZWxscyB3b3VsZFxuICAgKiBvdGhlcndpc2UgYWxsIGF1dG8tcGxheSB0aGVpciBvbmJvYXJkaW5nIGF0IG9uY2UgdGhlIG1vbWVudCB0aGV5IGJvb3QuIERlZmF1bHRzIHRvIGBmYWxzZWAgKGV4aXN0aW5nXG4gICAqIHNpbmdsZS1zaGVsbC1wZXItcGFnZSBiZWhhdmlvciB1bmNoYW5nZWQpLiAqL1xuICByZWFkb25seSBzdXBwcmVzc0F1dG9JbnRyb2R1Y3Rpb24/OiBib29sZWFuO1xufVxuXG4vKiogQGVtb2ppIPCfkJrvuI8gUmVzb2x2ZXMgdGhlIHtAbGluayBTaGVsbFNjb3BlLnN0b3JhZ2V9IHBvcnQgZm9yIGEgc2hlbGwgbW91bnQ6IGVwaGVtZXJhbCBicmFuZHMgYWx3YXlzIGdldFxuICogYW4gaW4tbWVtb3J5IHBvcnQgKG5ldmVyIGR1cmFibGUsIHJlZ2FyZGxlc3Mgb2YgbmFtZXNwYWNlKTsgYSBuYW1lc3BhY2VkIG5vbi1lcGhlbWVyYWwgc2hlbGwgZ2V0cyBhXG4gKiBzY29wZWQgdmlldyBvdmVyIGJyb3dzZXIgc3RvcmFnZTsgYSBiYXJlIG5vbi1lcGhlbWVyYWwgc2hlbGwgKHRoZSBoaXN0b3JpY2FsIHNpbmdsZS1hcHAtcGVyLXBhZ2VcbiAqIGNhc2UpIGdldHMgdGhlIHBsYWluIHNoYXJlZCBicm93c2VyIHBvcnQuICovXG5mdW5jdGlvbiByZXNvbHZlU2hlbGxTY29wZVN0b3JhZ2UoZXBoZW1lcmFsOiBib29sZWFuLCBzdG9yYWdlTmFtZXNwYWNlOiBzdHJpbmcgfCB1bmRlZmluZWQpOiBTdG9yYWdlUG9ydCB7XG4gIGlmIChlcGhlbWVyYWwpIHJldHVybiBjcmVhdGVNZW1vcnlTdG9yYWdlUG9ydCgpO1xuICBjb25zdCBicm93c2VyID0gY3JlYXRlQnJvd3NlclN0b3JhZ2VQb3J0KCk7XG4gIHJldHVybiBzdG9yYWdlTmFtZXNwYWNlID8gY3JlYXRlU2NvcGVkU3RvcmFnZVBvcnQoYnJvd3Nlciwgc3RvcmFnZU5hbWVzcGFjZSkgOiBicm93c2VyO1xufVxuXG4vKiogQGVtb2ppIPCfkJrvuI8gTW91bnRzIGEgYC5zZW1pby1zY29wZWAgcm9vdCAodGhlbWUvYXBwZWFyYW5jZS9pZCBzY29waW5nIGxhbmRzIHdpdGggbGF0ZXIgd2F2ZXMpIGNhcnJ5aW5nIGFcbiAqIHtAbGluayBTaGVsbFNjb3BlfSDigJQgdGhlIHNlYW0gdGhhdCBsZXRzIHNldmVyYWwgb2YgdGhlc2UgY29leGlzdCBvbiBvbmUgcGFnZSDigJQgYXJvdW5kIHRoZSBhY3R1YWwgc2hlbGxcbiAqIGltcGxlbWVudGF0aW9uIGluIHtAbGluayBGcmFtZXdvcmtPc1NoZWxsSW5uZXJ9LiAqL1xuZXhwb3J0IGZ1bmN0aW9uIEZyYW1ld29ya09zU2hlbGwocHJvcHM6IEZyYW1ld29ya09zU2hlbGxQcm9wcyk6IFJlYWN0LlJlYWN0RWxlbWVudCB7XG4gIGNvbnN0IHsgc2hlbGxJZCwgc3RvcmFnZU5hbWVzcGFjZSwgb3duc1BhZ2UgPSBmYWxzZSwgYnJhbmQsIGxvY2tzLCAuLi5pbm5lclByb3BzIH0gPSBwcm9wcztcbiAgY29uc3QgZXBoZW1lcmFsID0gaXNFcGhlbWVyYWxTaGVsbEJyYW5kKGJyYW5kKTtcbiAgY29uc3QgW3Njb3BlXSA9IHVzZVN0YXRlPFNoZWxsU2NvcGU+KCgpID0+IHtcbiAgICBjb25zdCBzdG9yYWdlID0gcmVzb2x2ZVNoZWxsU2NvcGVTdG9yYWdlKGVwaGVtZXJhbCwgc3RvcmFnZU5hbWVzcGFjZSk7XG4gICAgLy8g8J+Qmu+4jyBSZXNvbHZlZCBzeW5jaHJvbm91c2x5IChub3QgaW4gYSBgdXNlRWZmZWN0YCkgc28gYW4gZW1iZWRkZWQgc2hlbGwgbmV2ZXIgZmxhc2hlcyB0aGUgd3JvbmdcbiAgICAvLyBsb2NhbGUncyBjaHJvbWUgb24gaXRzIGZpcnN0IHBhaW50LCBtaXJyb3JpbmcgYGluaXRVaUxvY2FsZVN5bmNgJ3MgcmVhc29uaW5nIGZvciB0aGUgcGFnZS1vd25pbmdcbiAgICAvLyBjYXNlLiBgbG9ja3MubG9jYWxlYCBhbmQgYW55IHByZXZpb3VzbHktc3RvcmVkIHByZWZlcmVuY2UgY292ZXIgdGhlIGNvbW1vbiBjYXNlczsgYSBicmFuZCdzIG93blxuICAgIC8vIGBkZWZhdWx0cy5sb2NhbGVgIChub3QgYXZhaWxhYmxlIHlldCBoZXJlKSBzdGlsbCBsYW5kcyBtb21lbnRzIGxhdGVyIHZpYSB0aGUgdWlQcmVmcyBlZmZlY3QgYmVsb3cuXG4gICAgY29uc3QgaW5pdGlhbExvY2FsZSA9IGxvY2tzPy5sb2NhbGUgPz8gcmVhZFN0b3JlZFVpQ2hyb21lTG9jYWxlKHN0b3JhZ2UpID8/IGRldGVjdFNoZWxsTG9jYWxlKHR5cGVvZiBuYXZpZ2F0b3IgIT09IFwidW5kZWZpbmVkXCIgPyBuYXZpZ2F0b3IubGFuZ3VhZ2UgOiB1bmRlZmluZWQpO1xuICAgIHJldHVybiBjcmVhdGVTaGVsbFNjb3BlKHsgc2hlbGxJZCwgb3duc1BhZ2UsIHN0b3JhZ2UsIGluaXRpYWxMb2NhbGUgfSk7XG4gIH0pO1xuICAvLyDwn5Ca77iPIGBzY29wZS5yb290UmVmYCBpcyBhIHN0YWJsZSBvYmplY3QgKGl0cyBpZGVudGl0eSBuZXZlciBjaGFuZ2VzKSwgc28gYSBkZXNjZW5kYW50IGhvb2sgdGhhdCBwdXRzXG4gIC8vIHRoZSBSRUYgSVRTRUxGIGluIGEgYHVzZUVmZmVjdGAvYHVzZUxheW91dEVmZmVjdGAgZGVwZW5kZW5jeSBhcnJheSB3b3VsZCBuZXZlciByZS1maXJlIG9uY2UgdGhlIHJlZlxuICAvLyBhdHRhY2hlcy4gVGhpcyBzdGF0ZSBidW1wIGZvcmNlcyBvbmUgZ3VhcmFudGVlZCByZS1yZW5kZXIgcmlnaHQgYWZ0ZXIgYXR0YWNobWVudCBzbyBkZXNjZW5kYW50cyB0aGF0XG4gIC8vIHJlYWQgYHNjb3BlLnJvb3RSZWYuY3VycmVudGAgZnJlc2ggYXQgcmVuZGVyIHRpbWUgKHNlZSBgRnJhbWV3b3JrT3NTaGVsbElubmVyYCdzXG4gIC8vIGB1c2VFbGVtZW50c1N1cmZhY2VDaHJvbWVgL2B1c2VDYW52YXNBcHBlYXJhbmNlU3luY2AgY2FsbHMpIHBpY2sgdXAgdGhlIHJlYWwgZWxlbWVudCBpbnN0ZWFkIG9mXG4gIC8vIHN0aWNraW5nIHdpdGggd2hhdGV2ZXIgdGhleSBzYXcgKHVzdWFsbHkgYG51bGxgKSBvbiB0aGUgdmVyeSBmaXJzdCByZW5kZXIuXG4gIGNvbnN0IFssIGJ1bXBBZnRlclJvb3RBdHRhY2hdID0gdXNlU3RhdGUoMCk7XG4gIGNvbnN0IHNldFJvb3QgPSB1c2VDYWxsYmFjaygobm9kZTogSFRNTERpdkVsZW1lbnQgfCBudWxsKSA9PiB7XG4gICAgc2NvcGUucm9vdFJlZi5jdXJyZW50ID0gbm9kZTtcbiAgICBidW1wQWZ0ZXJSb290QXR0YWNoKChuKSA9PiBuICsgMSk7XG4gIH0sIFtzY29wZV0pO1xuICBjb25zdCBzZXRQb3J0YWxMYXllciA9IHVzZUNhbGxiYWNrKChub2RlOiBIVE1MRGl2RWxlbWVudCB8IG51bGwpID0+IHtcbiAgICBzY29wZS5wb3J0YWxMYXllclJlZi5jdXJyZW50ID0gbm9kZTtcbiAgfSwgW3Njb3BlXSk7XG4gIHVzZUVmZmVjdCgoKSA9PiAoKSA9PiBkaXNwb3NlU2hlbGxJMThuSW5zdGFuY2Uoc2NvcGUuaTE4biksIFtzY29wZV0pO1xuICByZXR1cm4gKFxuICAgIDxkaXYgcmVmPXtzZXRSb290fSBjbGFzc05hbWU9XCJzZW1pby1zY29wZVwiIGRhdGEtc2hlbGwtaWQ9e3Njb3BlLnNoZWxsSWR9IHN0eWxlPXt7IGhlaWdodDogXCIxMDAlXCIsIHdpZHRoOiBcIjEwMCVcIiwgaXNvbGF0aW9uOiBcImlzb2xhdGVcIiB9fT5cbiAgICAgIDxTaGVsbFNjb3BlUHJvdmlkZXIgc2NvcGU9e3Njb3BlfT5cbiAgICAgICAgPEZyYW1ld29ya09zU2hlbGxJbm5lciB7Li4uaW5uZXJQcm9wc30gbG9ja3M9e2xvY2tzfSBicmFuZD17YnJhbmR9IC8+XG4gICAgICAgIDxkaXYgZGF0YS1zZW1pby1wb3J0YWwtbGF5ZXIgcmVmPXtzZXRQb3J0YWxMYXllcn0gLz5cbiAgICAgIDwvU2hlbGxTY29wZVByb3ZpZGVyPlxuICAgIDwvZGl2PlxuICApO1xufVxuLy8jZW5kcmVnaW9uIPCfkJrvuI9TaGVsbE1vdW50XG5cbmZ1bmN0aW9uIEZyYW1ld29ya09zU2hlbGxJbm5lcih7XG4gIHBsdWdpbkZpbHRlcixcbiAgcGx1Z2lucyxcbiAgYXBwSWQsXG4gIGxvY2tzOiBsb2Nrc1Byb3AsXG4gIGRlZmF1bHRzOiBkZWZhdWx0c1Byb3AsXG4gIGJyYW5kLFxuICBzdXBwcmVzc0F1dG9JbnRyb2R1Y3Rpb24gPSBmYWxzZSxcbn06IHtcbiAgcmVhZG9ubHkgcGx1Z2luRmlsdGVyPzogc3RyaW5nO1xuICByZWFkb25seSBwbHVnaW5zOiByZWFkb25seSB7IHJlYWRvbmx5IHBsdWdpbklkOiBzdHJpbmc7IHJlYWRvbmx5IG1vZHVsZVVybDogc3RyaW5nIH1bXTtcbiAgcmVhZG9ubHkgYXBwSWQ/OiBzdHJpbmc7XG4gIHJlYWRvbmx5IGxvY2tzPzogUmVzb2x2ZWRTaGVsbExvY2tzO1xuICByZWFkb25seSBkZWZhdWx0cz86IEZyYW1ld29ya09zRGVmYXVsdHM7XG4gIHJlYWRvbmx5IGJyYW5kPzogU2hlbGxCcmFuZDtcbiAgcmVhZG9ubHkgc3VwcHJlc3NBdXRvSW50cm9kdWN0aW9uPzogYm9vbGVhbjtcbn0pIHtcbiAgY29uc3Qgc2NvcGUgPSB1c2VTaGVsbFNjb3BlKCk7XG4gIGNvbnN0IHNoZWxsQ29udGV4dE1lbnVUaXRsZUxhYmVsID0gdXNlTGFiZWwoXCJ1aS5zdXJmYWNlQ29udGV4dE1lbnUud29ya3NwYWNlXCIpO1xuICAvLyDwn4+g77iP8J+ns++4jyBgaG9zdENvbmZpZ2AgaXMgdGhlIHNvbGUgcGllY2Ugb2YgcGVyLXBsdWdpbiBpZGVudGl0eSBrbm93bGVkZ2UgdGhlIHNoZWxsIG5lZWRzICh3aGljaCBhcHAgaWQgaXNcbiAgLy8gXCJsYW5kaW5nXCIsIHdoaWNoIGlzIFwiaG9zdFwiKSDigJQgZXZlcnkgY29udHJvbGxlciBpZCAvIGRlZmF1bHQgcGFuZWwgdGFiIGRlcml2ZXMgZnJvbSB0aGUgKmxvYWRlZCpcbiAgLy8gbWFuaWZlc3QncyBvd24gYGNvbnRyb2xsZXJJZGAvYHBhbmVsVGFic2Agb24gdGhvc2UgYXBwcyBiZWxvdywgbmV2ZXIgZnJvbSBhIHNlcGFyYXRlIGxpdGVyYWwuXG4gIGNvbnN0IGhvc3RDb25maWcgPSBwbHVnaW5GaWx0ZXIgPyByZXNvbHZlUGx1Z2luSG9zdENvbmZpZyhwbHVnaW5GaWx0ZXIpIDogdW5kZWZpbmVkO1xuICBjb25zdCBzdHVkaW9Nb2RlID0gaG9zdENvbmZpZyAhPT0gdW5kZWZpbmVkO1xuICBjb25zdCBtb2JpbGUgPSB1c2VNZWRpYVF1ZXJ5KFVJX01PQklMRV9NRURJQV9RVUVSWSk7XG4gIGNvbnN0IGxvY2tzID0gbG9ja3NQcm9wID8/IEVNUFRZX1NIRUxMX0xPQ0tTO1xuICBjb25zdCBkZWZhdWx0cyA9IGRlZmF1bHRzUHJvcCA/PyBFTVBUWV9TSEVMTF9ERUZBVUxUUztcbiAgY29uc3QgZXBoZW1lcmFsID0gaXNFcGhlbWVyYWxTaGVsbEJyYW5kKGJyYW5kKTtcbiAgY29uc3QgW3NoZWxsU3RhdGUsIGRpc3BhdGNoXSA9IHVzZVJlZHVjZXIoc2hlbGxSZWR1Y2VyLCB1bmRlZmluZWQsICgpID0+IGluaXRpYWxTaGVsbFN0YXRlKHsgcGx1Z2luRmlsdGVyLCBwbHVnaW5zLCBsb2NrcywgZGVmYXVsdHMsIHN0b3JhZ2U6IHNjb3BlLnN0b3JhZ2UgfSkpO1xuICBjb25zdCB7IGxvYWRlZFBsdWdpbnMsIHBsdWdpblN0YXR1c0J5SWQsIHBsdWdpblN1cGVydmlzb3JCeUlkLCBzZXNzaW9uLCBlcnJvciB9ID0gc2hlbGxTdGF0ZS5wbHVnaW5SdW50aW1lO1xuICBjb25zdCBob3N0UGx1Z2luID0gdXNlTWVtbygoKSA9PiAoaG9zdENvbmZpZyA/IGxvYWRlZFBsdWdpbnMuZmluZCgoZW50cnkpID0+IGVudHJ5LmhhbmRsZS5wbHVnaW5JZCA9PT0gaG9zdENvbmZpZy5wbHVnaW5JZCkgOiB1bmRlZmluZWQpLCBbbG9hZGVkUGx1Z2lucywgaG9zdENvbmZpZ10pO1xuICBjb25zdCBob3N0QXBwID0gdXNlTWVtbygoKSA9PiBob3N0UGx1Z2luPy5tYW5pZmVzdC5hcHBzLmZpbmQoKGFwcCkgPT4gYXBwLmlkID09PSBob3N0Q29uZmlnPy5ob3N0QXBwSWQpLCBbaG9zdFBsdWdpbiwgaG9zdENvbmZpZ10pO1xuICBjb25zdCBsYW5kaW5nQXBwID0gdXNlTWVtbygoKSA9PiBob3N0UGx1Z2luPy5tYW5pZmVzdC5hcHBzLmZpbmQoKGFwcCkgPT4gYXBwLmlkID09PSBob3N0Q29uZmlnPy5sYW5kaW5nQXBwSWQpID8/IGhvc3RQbHVnaW4/Lm1hbmlmZXN0LmFwcHNbMF0sIFtob3N0UGx1Z2luLCBob3N0Q29uZmlnXSk7XG4gIGNvbnN0IGxhbmRpbmdBcHBJZCA9IGhvc3RDb25maWc/LmxhbmRpbmdBcHBJZDtcbiAgY29uc3QgaG9zdEFwcElkID0gaG9zdENvbmZpZz8uaG9zdEFwcElkO1xuICBjb25zdCBob3N0Q29udHJvbGxlcklkID0gaG9zdEFwcD8uY29udHJvbGxlcklkO1xuICBjb25zdCBsYW5kaW5nQ29udHJvbGxlcklkID0gbGFuZGluZ0FwcD8uY29udHJvbGxlcklkO1xuICBjb25zdCBob3N0Q2F0YWxvZ3VlVGFiSWQgPSBob3N0QXBwPy5wYW5lbFRhYnNbMF0gPyBwYW5lbFRhYktpbmRJZChob3N0QXBwLnBhbmVsVGFic1swXS5raW5kKSA6IHVuZGVmaW5lZDtcbiAgY29uc3QgeyB3aW5kb3dVaUJ5V2luZG93SWQsIHdpbmRvd0VuZ2FnZW1lbnRzQnlXaW5kb3dJZCwgd2luZG93TWVhc3VyZXNCeVdpbmRvd0lkLCB0b29sTWVhc3VyZXNCeVRvb2xJZCwgcGFuZWxVaUJ5S2V5LCBhcHBMYWJlbHNPdmVybGF5IH0gPSBzaGVsbFN0YXRlLndpbmRvd1VpO1xuICBjb25zdCB7IHNwYXduZWRXaW5kb3dVaSwgc3Bhd25lZFdpbmRvd0VuZ2FnZW1lbnRzLCBzcGF3bmVkV2luZG93TWVhc3VyZXMgfSA9IHNoZWxsU3RhdGUuc3Bhd25lZFdpbmRvdztcbiAgY29uc3QgeyBmb2xkZWRCeVdpbmRvd0lkOiBhY3Rpb25QYW5lRm9sZGVkQnlXaW5kb3dJZCwgZXhwYW5kZWRCeVdpbmRvd0lkOiBhY3Rpb25QYW5lRXhwYW5kZWRCeVdpbmRvd0lkLCBzdGFnZWRBcmdzQnlLZXk6IGFjdGlvblBhbmVTdGFnZWRBcmdzQnlLZXksIGFjdGl2ZVV0aWxpdHlCeVdpbmRvd0lkLCBhY3RpdmVUb29sSWQgfSA9IHNoZWxsU3RhdGUuYWN0aW9uUGFuZTtcbiAgY29uc3QgeyBleHBhbmRlZENvbW1hbmRJZCwgc3RhZ2VkQXJnc0J5Q29tbWFuZElkOiBjb21tYW5kU3RhZ2VkQXJnc0J5Q29tbWFuZElkIH0gPSBzaGVsbFN0YXRlLmNvbW1hbmRQYW5lbDtcbiAgY29uc3QgeyBwYW5lbHMsIGRvY2tPdmVycmlkZSwgcGFuZWxQYXRoTWVtb3J5LCB0cmVlT3BlblN0YXRlcywgYWN0aXZlV2luZG93SWQsIHNoZWxsTGF5b3V0LCBhY3RpdmVFeGFtcGxlSWQsIG1vYmlsZVBhbmVsUGF0aCwgbW9iaWxlUGFuZWxWaXNpYmxlLCBleHRyYVdpbmRvd0luc3RhbmNlcywgd2luZG93VGl0bGVzQnlJZCwgd2luZG93SWNvbnNCeUlkIH0gPSBzaGVsbFN0YXRlLmxheW91dDtcbiAgY29uc3QgeyBzZWFyY2hPcGVuLCBmaW5kT3BlbiwgaW50cm9kdWN0aW9uU3RlcEluZGV4LCBpbnRyb2R1Y3Rpb25Db21wbGV0ZWRJbnRlcmFjdGlvbnMsIGRpYWxvZzogb3ZlcmxheURpYWxvZyB9ID0gc2hlbGxTdGF0ZS5vdmVybGF5cztcbiAgY29uc3QgeyBhY3RpdmVUdXRvcmlhbElkLCBwbGF5aW5nOiB0dXRvcmlhbFBsYXlpbmcsIHJhdGU6IHR1dG9yaWFsUmF0ZSwgbXV0ZWQ6IHR1dG9yaWFsTXV0ZWQsIGNhcHRpb25zT246IHR1dG9yaWFsQ2FwdGlvbnNPbiwgcmVjb3JkaW5nOiB0dXRvcmlhbFJlY29yZGluZywgZGV2aWF0ZWQ6IHR1dG9yaWFsRGV2aWF0ZWQgfSA9IHNoZWxsU3RhdGUudHV0b3JpYWw7XG4gIGNvbnN0IHsgdWlBcHBlYXJhbmNlLCB1aUxheW91dCwgdWlEcml2ZXJJZCwgdWlDdXN0b21Ecml2ZXJzLCB1aURyaXZlckRyYWZ0LCB1aUxvY2FsZSwgdWlUZXJtaW5vbG9neSwgdWlUaGVtZUlkLCB1aUN1c3RvbVRoZW1lcywgdWlUaGVtZURyYWZ0LCB1aUtleWJpbmRpbmdPdmVycmlkZXMgfSA9IHNoZWxsU3RhdGUudWlQcmVmcztcbiAgY29uc3QgeyBzeW5jQmFja2JvbmVVcmksIHN5bmNDYXJkS2luZCwgc3luY0RyYWZ0UGF0aCwgc3luY1N0YXR1c0J5RG9jdW1lbnRJZCB9ID0gc2hlbGxTdGF0ZS5zeW5jO1xuICBjb25zdCBpbXBvcnRTcGFjZUlucHV0UmVmID0gdXNlUmVmPEhUTUxJbnB1dEVsZW1lbnQ+KG51bGwpO1xuICBjb25zdCByZWZyZXNoR2VuZXJhdGlvblJlZiA9IHVzZVJlZigwKTtcbiAgY29uc3QgY29udHJpYnV0aW9uc0pzb25SZWYgPSB1c2VSZWY8c3RyaW5nIHwgbnVsbD4obnVsbCk7XG4gIGNvbnN0IGFwcFJlZ2lzdHJhdGlvbnNKc29uUmVmID0gdXNlUmVmPHN0cmluZyB8IG51bGw+KG51bGwpO1xuICBjb25zdCBzcGF3bmVkUmVmcmVzaEdlbmVyYXRpb25SZWYgPSB1c2VSZWYoMCk7XG4gIGNvbnN0IGNvbnRyaWJ1dG9ySW5zdGFuY2VzUmVmID0gdXNlUmVmPE1hcDxzdHJpbmcsIG51bWJlcj4+KG5ldyBNYXAoKSk7XG4gIGNvbnN0IGxheW91dFNlZWRLZXlSZWYgPSB1c2VSZWY8c3RyaW5nIHwgbnVsbD4obnVsbCk7XG4gIGNvbnN0IG5vRXhhbXBsZVJlc2V0SW5zdGFuY2VJZFJlZiA9IHVzZVJlZjxudW1iZXIgfCBudWxsPihudWxsKTtcbiAgY29uc3QgZXh0cmFXaW5kb3dDb3VudGVyUmVmID0gdXNlUmVmKDApO1xuICAvLyDwn5ax77iPIFNoZWxsLWxldmVsIGNvbnRleHQtbWVudSBmYWxsYmFjazogb3BlbnMgZm9yIGFueSByaWdodC1jbGljayB0aGUgc2hlbGwgaGFzbid0IGFscmVhZHkgY2xhaW1lZFxuICAvLyAoZXZlcnkgZXhpc3RpbmcgcGVyLXN1cmZhY2UgYG9uQ29udGV4dE1lbnVgIG5vdyBjYWxscyBgc3RvcFByb3BhZ2F0aW9uKClgIG9uY2UgaXQgZGVjaWRlcyB0byBzaG93XG4gIC8vIGl0cyBvd24gbWVudSDigJQgc2VlIHRoZSBg8J+Wse+4j1NoZWxsQ29udGV4dE1lbnVgIHJlZ2lvbiBiZWxvdykuIENvdmVycyB3aW5kb3ctbGV2ZWwgZGVjbGFyZWQgYWN0aW9uc1xuICAvLyBwbHVzIHRoZSBPUyBjb21tYW5kIHBhbGV0dGUsIHNvIGV2ZXJ5IHdpbmRvdy9iYWNrZ3JvdW5kIGFsd2F5cyBzaG93cyAqc29tZXRoaW5nKi5cbiAgY29uc3QgW3NoZWxsQ29udGV4dE1lbnUsIHNldFNoZWxsQ29udGV4dE1lbnVdID0gdXNlU3RhdGU8eyByZWFkb25seSB4OiBudW1iZXI7IHJlYWRvbmx5IHk6IG51bWJlcjsgcmVhZG9ubHkgaXRlbXM6IHJlYWRvbmx5IENvbnRleHRNZW51SXRlbVtdIH0gfCBudWxsPihudWxsKTtcbiAgLy8g8J+qn++4jyBMaXZlIGV4dHJhLXdpbmRvdyBsaXN0LCB1cGRhdGVkIHN5bmNocm9ub3VzbHkgb24gZXZlcnkgc2VlZC9zcGxpdC9kcm9wIOKAlCBgcmVmcmVzaFVpYCByZWFkcyB0aGlzXG4gIC8vIGluc3RlYWQgb2YgdGhlIHJlbmRlci1jbG9zdXJlIGBleHRyYVdpbmRvd0luc3RhbmNlc2Agc28gYSBjb25jdXJyZW50IGFjdGlvbiByZWZyZXNoIChlLmcuIGJvb3RcbiAgLy8gYHNldEFjdGl2ZUV4YW1wbGVgKSB0aGF0IHN0YXJ0cyBhZnRlciB0aGUgc2Vzc2lvbi1zd2l0Y2ggcmVmcmVzaCB3cm90ZSBleHRyYXMgYnV0IGJlZm9yZSBSZWFjdFxuICAvLyByZS1yZW5kZXJlZCBjYW5ub3QgZmV0Y2ggd2l0aCBgW11gIGFuZCB3aXBlIFRvcC9QZXJzcGVjdGl2ZSBib2RpZXMgdG8gXCJtaXNzaW5nIHdpbmRvd1wiLlxuICBjb25zdCBleHRyYVdpbmRvd0luc3RhbmNlc1JlZiA9IHVzZVJlZjxyZWFkb25seSBFeHRyYVdpbmRvd0luc3RhbmNlW10+KFtdKTtcbiAgZXh0cmFXaW5kb3dJbnN0YW5jZXNSZWYuY3VycmVudCA9IGV4dHJhV2luZG93SW5zdGFuY2VzO1xuICBjb25zdCBzZXRXaW5kb3dUaXRsZSA9IHVzZUNhbGxiYWNrKCh3aW5kb3dJZDogc3RyaW5nLCB0aXRsZTogc3RyaW5nKSA9PiB7XG4gICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9XSU5ET1dfVElUTEVcIiwgd2luZG93SWQsIHRpdGxlIH0pO1xuICB9LCBbXSk7XG4gIGNvbnN0IHNldFdpbmRvd0ljb24gPSB1c2VDYWxsYmFjaygod2luZG93SWQ6IHN0cmluZywgaWNvbklkOiBJY29uTmFtZSkgPT4ge1xuICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfV0lORE9XX0lDT05cIiwgd2luZG93SWQsIGljb25JZCB9KTtcbiAgfSwgW10pO1xuICAvLyDwn5Ci77iPIFBlci1pbnN0YW5jZSBjb250ZW50LWhhc2ggY2FjaGUgZm9yIHRoZSBiYXRjaGVkIGByZWZyZXNoLXVpYCBjYWxsLCBrZXllZCBieSB0aGUgc2FtZVxuICAvLyBgcGx1Z2luSWQ6YXBwSWQ6aW5zdGFuY2VJZGAgdHJpcGxlIGFzIGBsYXlvdXRTZWVkS2V5UmVmYCDigJQgY2xlYXJlZCBvbiBzZXNzaW9uIHN3aXRjaCBiZWxvdy5cbiAgY29uc3QgdWlSZWZyZXNoQ2FjaGVSZWYgPSB1c2VSZWY8VWlSZWZyZXNoQ2FjaGU+KG5ldyBNYXAoKSk7XG4gIC8vIPCfkKLvuI8gU2FtZSBpZGVhIGZvciB0aGUgc3R1ZGlvLW1vZGUgc3Bhd25lZC1pbnN0YW5jZSB2aWV3LCBrZXllZCBieSBzcGF3bmVkIGluc3RhbmNlSWQg4oCUIGNsZWFyZWQgd2hlblxuICAvLyB0aGUgc3Bhd25lZCBpbnN0YW5jZSBpdHNlbGYgY2hhbmdlcyAodHJhY2tlZCB2aWEgYHNwYXduZWRMYXlvdXRTZWVkUmVmYCkuXG4gIGNvbnN0IHNwYXduZWRVaVJlZnJlc2hDYWNoZVJlZiA9IHVzZVJlZjxVaVJlZnJlc2hDYWNoZT4obmV3IE1hcCgpKTtcbiAgY29uc3Qgc3Bhd25lZExheW91dFNlZWRSZWYgPSB1c2VSZWY8c3RyaW5nIHwgbnVsbD4obnVsbCk7XG4gIGNvbnN0IG9wZW5TcGFjZUlkUmVmID0gdXNlUmVmPHN0cmluZyB8IG51bGw+KG51bGwpO1xuICBjb25zdCBvcGVuSW5zdGFuY2VJZFJlZiA9IHVzZVJlZjxzdHJpbmcgfCBudWxsPihudWxsKTtcbiAgY29uc3Qgc2Vzc2lvblJlZiA9IHVzZVJlZjxBY3RpdmVTZXNzaW9uIHwgbnVsbD4obnVsbCk7XG4gIGNvbnN0IHVpRGV2aWNlOiBFbGVtZW50c1N1cmZhY2VEZXZpY2UgPSBtb2JpbGUgPyBcIm1vYmlsZVwiIDogdWlMYXlvdXQ7XG4gIGNvbnN0IHVpVGhlbWU6IFVpVGhlbWUgPSB1c2VNZW1vKCgpID0+IHtcbiAgICBpZiAodWlUaGVtZURyYWZ0KSByZXR1cm4gdWlUaGVtZURyYWZ0O1xuICAgIGNvbnN0IGZvdW5kID0gYnVpbHRpblVpVGhlbWVzKCkuZmluZCgodCkgPT4gdC5pZCA9PT0gdWlUaGVtZUlkKSA/PyB1aUN1c3RvbVRoZW1lc1t1aVRoZW1lSWRdO1xuICAgIHJldHVybiBmb3VuZCA/PyByZWFkU3RvcmVkVWlDaHJvbWVUaGVtZVNuYXBzaG90KHNjb3BlLnN0b3JhZ2UpID8/IHNlbWlvVGhlbWUoKTtcbiAgfSwgW3VpVGhlbWVJZCwgdWlDdXN0b21UaGVtZXMsIHVpVGhlbWVEcmFmdCwgc2NvcGUuc3RvcmFnZV0pO1xuICBjb25zdCB1aURyaXZlcjogVWlEcml2ZXIgPSB1c2VNZW1vKCgpID0+IHVpRHJpdmVyRHJhZnQgPz8gcmVzb2x2ZVVpRHJpdmVyKHVpRHJpdmVySWQsIHVpQ3VzdG9tRHJpdmVycyksIFt1aURyaXZlcklkLCB1aUN1c3RvbURyaXZlcnMsIHVpRHJpdmVyRHJhZnRdKTtcbiAgLyoqIPCfp7XvuI8gTGF6aWx5LWNyZWF0ZWQgd29ya2VyIHJ1bm5pbmcgYPCfn6bvuI9iYWNrYm9uZS3wn5+m77iPd29ya2VyLnRzYCDigJQgb25lIHBlciBzaGVsbCBpbnN0YW5jZSwgcmV1c2VkIGFjcm9zcyBgb3BlbkRvY3VtZW50YCBjYWxscy4gKi9cbiAgY29uc3QgYmFja2JvbmVXb3JrZXJSZWYgPSB1c2VSZWY8V29ya2VyIHwgbnVsbD4obnVsbCk7XG4gIC8qKiDwn5aL77iPIFN0YWJsZSBwZXItdGFiIGFjdG9yIGlkIGZvciBodWIgYEhlbGxvYC9wcmVzZW5jZSBmcmFtZXMgYW5kIG9wZXJhdGlvbi1vcmlnaW4gZmlsdGVyaW5nLiAqL1xuICBjb25zdCBzaGVsbEFjdG9ySWRSZWYgPSB1c2VSZWY8c3RyaW5nPihgY2xpZW50LSR7TWF0aC5yYW5kb20oKS50b1N0cmluZygzNikuc2xpY2UoMil9YCk7XG4gIC8qKiDwn5eC77iPIFdoaWNoIHNlc3Npb24vcGx1Z2luIG93bnMgZWFjaCBvcGVuIGRvY3VtZW50IGlkLCBzbyBpbmNvbWluZyB3b3JrZXIgZXZlbnRzIHJvdXRlIGNvcnJlY3RseS4gKi9cbiAgY29uc3Qgb3BlbkRvY3VtZW50U2Vzc2lvbnNSZWYgPSB1c2VSZWY8TWFwPHN0cmluZywgeyBzZXNzaW9uOiBBY3RpdmVTZXNzaW9uOyBwbHVnaW46IFBsdWdpbldhc21IYW5kbGUgfT4+KG5ldyBNYXAoKSk7XG4gIC8qKiDwn5Ca77iPIFVucmVnaXN0ZXJzIHRoaXMgc2hlbGwncyBgcmVnaXN0ZXJQbHVnaW5CYWNrYm9uZVJvdXRlYCBlbnRyeSBmb3IgZWFjaCBvcGVuIGRvY3VtZW50IGlkIOKAlCBjYWxsZWRcbiAgICogZnJvbSBgY2xvc2VEb2N1bWVudGAgYW5kIChmb3Igd2hhdGV2ZXIgaXMgc3RpbGwgb3Blbikgb24gc2hlbGwgdW5tb3VudC4gKi9cbiAgY29uc3QgcGx1Z2luQmFja2JvbmVSb3V0ZVVucmVnaXN0ZXJzUmVmID0gdXNlUmVmPE1hcDxzdHJpbmcsICgpID0+IHZvaWQ+PihuZXcgTWFwKCkpO1xuICAvKiog8J+Qmu+4jyBNaXJyb3JzIGBsb2FkZWRQbHVnaW5zYCBmb3IgdGhlIHVubW91bnQtY2xlYW51cCBlZmZlY3QgYmVsb3csIHdoaWNoIG5lZWRzIHRoZSBsYXRlc3QgdmFsdWUgYXRcbiAgICogdGVhcmRvd24gdGltZSB3aXRob3V0IGRlcGVuZGluZyBvbiBpdCAoYSBkZXBlbmRlbmN5IHdvdWxkIHRlYXIgZG93biBhbmQgcmUtcnVuIG9uIGV2ZXJ5IHJlbG9hZCkuICovXG4gIGNvbnN0IGxvYWRlZFBsdWdpbnNSZWYgPSB1c2VSZWY8cmVhZG9ubHkgTG9hZGVkUHJvZ3JhbVN0YXRlW10+KFtdKTtcbiAgbG9hZGVkUGx1Z2luc1JlZi5jdXJyZW50ID0gbG9hZGVkUGx1Z2lucztcbiAgLyoqIPCflIzvuI8gVGhlIGV4YWN0IChwb3NzaWJseSBjYWNoZS1idXN0ZWQgYD92PWApIG1vZHVsZSBVUkwgZWFjaCBjdXJyZW50bHktbG9hZGVkIHBsdWdpbiB3YXMgYWNxdWlyZWRcbiAgICogYXQg4oCUIGBMb2FkZWRQcm9ncmFtU3RhdGVgL2BQbHVnaW5XYXNtSGFuZGxlYCBjYXJyeSBubyBVUkwgb2YgdGhlaXIgb3duLCBidXQgYHJlbG9hZFBsdWdpbmAvXG4gICAqIGB1bmluc3RhbGxQbHVnaW5gIG5lZWQgdGhlIE9MRCB1cmwgdG8gYGV2aWN0UGx1Z2luTW9kdWxlYCBhZnRlciBzd2FwcGluZyBpbiBhIG5ldyBsZWFzZSBhdCBhXG4gICAqIGRpZmZlcmVudCB1cmwgKHNlZSB0aGUgbGVhc2UgcG9vbCdzIGtleSBjb252ZW50aW9uIGluIGBAc2VtaW8tdGVjaC9mcmFtZXdvcmstY29yZWApLiAqL1xuICBjb25zdCBwbHVnaW5Nb2R1bGVVcmxCeUlkUmVmID0gdXNlUmVmPE1hcDxzdHJpbmcsIHN0cmluZz4+KG5ldyBNYXAoKSk7XG4gIC8qKiDwn5SM77iPIFBlci1wbHVnaW5JZCBtdXR1YWwgZXhjbHVzaW9uIGFjcm9zcyBgaW5zdGFsbFBsdWdpbmAvYHJlbG9hZFBsdWdpbmAvYHVuaW5zdGFsbFBsdWdpbmAg4oCUIHRoZVxuICAgKiBib290IGVmZmVjdCBhbmQgdGhlIGBQbHVnaW5Tb3VyY2VgIHN1YnNjcmlwdGlvbiBlZmZlY3QgY2FuIGJvdGggcmVxdWVzdCB0aGUgc2FtZSBwbHVnaW5JZCBhcm91bmRcbiAgICogbW91bnQgKGUuZy4gdGhlIGhvc3QgcGx1Z2luIGFscmVhZHkgYXBwZWFycyBpbiB0aGUgY29ubmVjdC10aW1lIGBzbmFwc2hvdGApLCBhbmQgd2l0aG91dCB0aGlzIGd1YXJkXG4gICAqIGJvdGggY2FsbHMgd291bGQgaW5kZXBlbmRlbnRseSBhY3F1aXJlIGEgbW9kdWxlIGxlYXNlLCByYWNlIHRoZWlyIGBVUFNFUlRfTE9BREVEX1BMVUdJTmAgZGlzcGF0Y2hlcyxcbiAgICogYW5kIGxlYWsgd2hpY2hldmVyIGxlYXNlIGxvc3QgdGhlIHJhY2UgKG5vdGhpbmcgbGVmdCBob2xkaW5nIGEgcmVmZXJlbmNlIHRvIHJlbGVhc2UgaXQpLiAqL1xuICBjb25zdCBwbHVnaW5PcEluRmxpZ2h0UmVmID0gdXNlUmVmPFNldDxzdHJpbmc+PihuZXcgU2V0KCkpO1xuXG4gIGNvbnN0IGVuc3VyZUJhY2tib25lV29ya2VyID0gdXNlQ2FsbGJhY2soKCk6IFdvcmtlciA9PiB7XG4gICAgaWYgKGJhY2tib25lV29ya2VyUmVmLmN1cnJlbnQpIHJldHVybiBiYWNrYm9uZVdvcmtlclJlZi5jdXJyZW50O1xuICAgIGNvbnN0IHdvcmtlciA9IG5ldyBXb3JrZXIobmV3IFVSTChcIi4uLy4uLy4uLy4uLy4uL/Cfn6bvuI9iYWNrYm9uZS13b3JrZXIudHNcIiwgaW1wb3J0Lm1ldGEudXJsKSwgeyB0eXBlOiBcIm1vZHVsZVwiIH0pO1xuICAgIHdvcmtlci5vbm1lc3NhZ2UgPSAobWVzc2FnZUV2ZW50OiBNZXNzYWdlRXZlbnQ8QmFja2JvbmVXb3JrZXJSZXNwb25zZSB8IHsgcmVhZG9ubHkgd2lyZTogVWludDhBcnJheSB9PikgPT4ge1xuICAgICAgY29uc3QgbWVzc2FnZSA9IFwid2lyZVwiIGluIG1lc3NhZ2VFdmVudC5kYXRhID8gZGVjb2RlQmFja2JvbmVXb3JrZXJSZXNwb25zZShtZXNzYWdlRXZlbnQuZGF0YS53aXJlKSA6IG1lc3NhZ2VFdmVudC5kYXRhO1xuICAgICAgaWYgKG1lc3NhZ2Uua2luZCAhPT0gXCJldmVudFwiKSByZXR1cm47XG4gICAgICBjb25zdCBlbnRyeSA9IG9wZW5Eb2N1bWVudFNlc3Npb25zUmVmLmN1cnJlbnQuZ2V0KG1lc3NhZ2UuZG9jdW1lbnRJZCk7XG4gICAgICBpZiAoIWVudHJ5KSByZXR1cm47XG4gICAgICBjb25zdCB7IGV2ZW50IH0gPSBtZXNzYWdlO1xuICAgICAgaWYgKGV2ZW50LmtpbmQgPT09IFwic3RhdHVzXCIpIHtcbiAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9TWU5DX1NUQVRVU19GT1JfRE9DVU1FTlRcIiwgZG9jdW1lbnRJZDogbWVzc2FnZS5kb2N1bWVudElkLCBzdGF0dXM6IHsgcGVyc2lzdGVkOiBldmVudC5wZXJzaXN0ZWQsIHBlbmRpbmdPcGVyYXRpb25zOiBldmVudC5wZW5kaW5nT3BlcmF0aW9ucywgcmVtb3RlOiBldmVudC5yZW1vdGUgfSB9KTtcbiAgICAgIH0gZWxzZSBpZiAoZXZlbnQua2luZCA9PT0gXCJwcmVzZW5jZVwiKSB7XG4gICAgICAgIGNvbnN0IHBlZXJzSnNvbiA9IEpTT04uc3RyaW5naWZ5KGV2ZW50LnBlZXJzLm1hcCgocGVlcikgPT4gKHsgY2xpZW50SWQ6IHBlZXIuYWN0b3IsIG5hbWU6IHBlZXIubGFiZWwgPz8gcGVlci5hY3Rvciwgc2VsZWN0aW9uQ291bnQ6IDAgfSkpKTtcbiAgICAgICAgZGlzcGF0Y2goe1xuICAgICAgICAgIHR5cGU6IFwiU0VUX1NFU1NJT05cIixcbiAgICAgICAgICB2YWx1ZTogKGN1cnJlbnQpID0+IChjdXJyZW50ICYmIGN1cnJlbnQuaW5zdGFuY2VJZCA9PT0gZW50cnkuc2Vzc2lvbi5pbnN0YW5jZUlkID8geyAuLi5jdXJyZW50LCB2aWV3U3RhdGU6IHsgLi4uY3VycmVudC52aWV3U3RhdGUsIHByZXNlbmNlUGVlcnNKc29uOiBwZWVyc0pzb24gfSB9IDogY3VycmVudCksXG4gICAgICAgIH0pO1xuICAgICAgfSBlbHNlIGlmIChldmVudC5raW5kID09PSBcInJlbW90ZU9wZXJhdGlvbnNcIiAmJiBlbnRyeS5wbHVnaW4uYXBwbHlPcGVyYXRpb25zKSB7XG4gICAgICAgIHZvaWQgZW50cnkucGx1Z2luLmFwcGx5T3BlcmF0aW9ucyhlbnRyeS5zZXNzaW9uLmluc3RhbmNlSWQsIGVuY29kZU9wZXJhdGlvbkVudmVsb3Blc1BhY2soZXZlbnQuZW52ZWxvcGVzKSk7XG4gICAgICAgIGNvbnN0IGFjdG9yVXJpID0gYGFjdG9yOi8vJHttZXNzYWdlLmRvY3VtZW50SWR9YDtcbiAgICAgICAgcG9zdFBsdWdpbkJhY2tib25lSW5ib3VuZChlbnRyeS5zZXNzaW9uLnBsdWdpbklkLCBhY3RvclVyaSwgW1xuICAgICAgICAgIGVuY29kZUJhY2tib25lTWVzc2FnZSh7XG4gICAgICAgICAgICBraW5kOiBcIm9wZXJhdGlvbnNcIixcbiAgICAgICAgICAgIGVudmVsb3BlczogZXZlbnQuZW52ZWxvcGVzLm1hcCgoZW52ZWxvcGUsIGluZGV4KSA9PlxuICAgICAgICAgICAgICBvcGVyYXRpb25FbnZlbG9wZVRvV2lyZShlbnZlbG9wZSwgeyBhY3RvcjogMCwgcGh5c2ljYWxfbXM6IERhdGUubm93KCksIGxvZ2ljYWw6IGluZGV4ICsgMSB9KSxcbiAgICAgICAgICAgICksXG4gICAgICAgICAgfSksXG4gICAgICAgIF0pO1xuICAgICAgfSBlbHNlIGlmIChldmVudC5raW5kID09PSBcInNuYXBzaG90UmVwbGFjZWRcIiAmJiBlbnRyeS5wbHVnaW4ubG9hZEFwcERvY3VtZW50KSB7XG4gICAgICAgIGNvbnN0IHBhY2tCeXRlcyA9IG5ldyBVaW50OEFycmF5KGV2ZW50LnBhY2spO1xuICAgICAgICBsZXQgZG9jdW1lbnRKc29uOiBzdHJpbmc7XG4gICAgICAgIHRyeSB7XG4gICAgICAgICAgZG9jdW1lbnRKc29uID0gSlNPTi5zdHJpbmdpZnkoZGVjb2RlUGFja1ZhbHVlKHBhY2tCeXRlcykpO1xuICAgICAgICB9IGNhdGNoIHtcbiAgICAgICAgICBkb2N1bWVudEpzb24gPSBKU09OLnN0cmluZ2lmeSh7IHBhY2s6IEFycmF5LmZyb20oZXZlbnQucGFjayksIHNwcjogQXJyYXkuZnJvbShldmVudC5zcHIpIH0pO1xuICAgICAgICB9XG4gICAgICAgIHZvaWQgZW50cnkucGx1Z2luLmxvYWRBcHBEb2N1bWVudChlbnRyeS5zZXNzaW9uLmluc3RhbmNlSWQsIGRvY3VtZW50SnNvbik7XG4gICAgICAgIGNvbnN0IGFjdG9yVXJpID0gYGFjdG9yOi8vJHttZXNzYWdlLmRvY3VtZW50SWR9YDtcbiAgICAgICAgcG9zdFBsdWdpbkJhY2tib25lSW5ib3VuZChlbnRyeS5zZXNzaW9uLnBsdWdpbklkLCBhY3RvclVyaSwgW1xuICAgICAgICAgIGVuY29kZUJhY2tib25lTWVzc2FnZSh7IGtpbmQ6IFwic25hcHNob3RcIiwgcGFjazogcGFja0J5dGVzLCBzcHI6IG5ldyBVaW50OEFycmF5KGV2ZW50LnNwcikgfSksXG4gICAgICAgIF0pO1xuICAgICAgfSBlbHNlIGlmIChldmVudC5raW5kID09PSBcImNvbmZsaWN0XCIpIHtcbiAgICAgICAgY29uc29sZS53YXJuKFwiW29zLXNoZWxsXSBzeW5jIGNvbmZsaWN0XCIsIG1lc3NhZ2UuZG9jdW1lbnRJZCwgZXZlbnQubWVzc2FnZSk7XG4gICAgICB9XG4gICAgfTtcbiAgICBiYWNrYm9uZVdvcmtlclJlZi5jdXJyZW50ID0gd29ya2VyO1xuICAgIHJldHVybiB3b3JrZXI7XG4gIH0sIFtdKTtcblxuICAvLyDwn5Ca77iPIE9ubHkgYSBwYWdlLW93bmluZyBzdHVkaW8gc2hlbGwgc3luY3MgdG8gdGhlIHJlYWwgYnJvd3NlciBVUkwgYmFyL2hpc3Rvcnkg4oCUIGFuIGVtYmVkZGVkIHNoZWxsXG4gIC8vIHNoYXJpbmcgdGhlIHBhZ2Ugd2l0aCBvdGhlcnMgbXVzdCBub3QgZmlnaHQgdGhlbSBvdmVyIGB3aW5kb3cuaGlzdG9yeWAuXG4gIGNvbnN0IHsgdXJpOiBzaGVsbFVyaSwgY2FuR29CYWNrLCBjYW5Hb0ZvcndhcmQsIGNhbkdvVXAsIGdvQmFjaywgZ29Gb3J3YXJkLCBnb1VwLCBuYXZpZ2F0ZTogbmF2aWdhdGVIaXN0b3J5IH0gPSB1c2VVSUhpc3RvcnkoXCIvXCIsIHN0dWRpb01vZGUgJiYgc2NvcGUub3duc1BhZ2UpO1xuICBjb25zdCBzaGVsbFJvdXRlID0gdXNlTWVtbygoKSA9PiBwYXJzZVNoZWxsUm91dGUoc2hlbGxVcmkuc3BsaXQoXCI/XCIpWzBdID8/IFwiL1wiKSwgW3NoZWxsVXJpXSk7XG5cbiAgLy8g8J+Qmu+4jyBgc2NvcGUuc3RvcmFnZWAgKG5vdCBhIHNlcGFyYXRlbHktcmVzb2x2ZWQgZXBoZW1lcmFsL2Jyb3dzZXIgcG9ydCBoZXJlKSDigJQgdHdvIHNoZWxscyBzaGFyaW5nIGFcbiAgLy8gcGFnZSBtdXN0IG5vdCBjbG9iYmVyIGVhY2ggb3RoZXIncyBwYW5lbCBsYXlvdXQvZG9jayBzdGF0ZSB0aHJvdWdoIGFuIHVubmFtZXNwYWNlZCBsb2NhbFN0b3JhZ2Uga2V5LlxuICBjb25zdCBzaGVsbFN0b3JhZ2UgPSBzY29wZS5zdG9yYWdlO1xuICBjb25zdCBuYW1lZExheW91dFN0b3JlID0gdXNlTWVtbygoKSA9PiBuZXcgTmFtZWRMYXlvdXRTdG9yZShzZXNzaW9uPy5hcHAuaWQgPz8gXCJmcmFtZXdvcmstb3NcIiwgc2hlbGxTdG9yYWdlKSwgW3Nlc3Npb24/LmFwcC5pZCwgc2hlbGxTdG9yYWdlXSk7XG4gIGNvbnN0IGRvY2tMYXlvdXRTdG9yZSA9IHVzZU1lbW8oKCkgPT4gbmV3IERvY2tMYXlvdXRTdG9yZShzaGVsbFN0b3JhZ2UsIHNlc3Npb24/LmFwcC5pZCksIFtzZXNzaW9uPy5hcHAuaWQsIHNoZWxsU3RvcmFnZV0pO1xuICBjb25zdCBkb2NrVWlTdGF0ZVN0b3JlID0gdXNlTWVtbygoKSA9PiBuZXcgRG9ja1VpU3RhdGVTdG9yZShzaGVsbFN0b3JhZ2UsIHNlc3Npb24/LmFwcC5pZCksIFtzZXNzaW9uPy5hcHAuaWQsIHNoZWxsU3RvcmFnZV0pO1xuXG4gIGNvbnN0IHJlZ2lzdHJ5ID0gdXNlTWVtbygoKSA9PiB7XG4gICAgY29uc3QgZXhwYW5kZWQgPSBleHBhbmRQbHVnaW5SZWdpc3RyeShwbHVnaW5zLCBwbHVnaW5GaWx0ZXIgPyByZXNvbHZlUGx1Z2luUmVnaXN0cnlJZChwbHVnaW5GaWx0ZXIpIDogdW5kZWZpbmVkLCBzdHVkaW9Nb2RlKTtcbiAgICBpZiAoc3R1ZGlvTW9kZSkgcmV0dXJuIGV4cGFuZGVkO1xuICAgIHJldHVybiBwbHVnaW5GaWx0ZXIgPyBleHBhbmRlZCA6IHBsdWdpbnM7XG4gIH0sIFtwbHVnaW5GaWx0ZXIsIHBsdWdpbnMsIHN0dWRpb01vZGVdKTtcblxuICAvLyNyZWdpb24g8J+UjO+4j1BsdWdpblJ1bnRpbWVcbiAgLyoqIPCflIzvuI8gVGhlIG9uZSByZWdpc3RyeSBlbnRyeSB0aGUgc2hlbGwgbXVzdCBoYXZlIGxvYWRlZCBiZWZvcmUgaXQgY2FuIGNyZWF0ZSBhIHNlc3Npb24g4oCUIHRoZSBzdHVkaW9cbiAgICogaG9zdCBwbHVnaW4gKGBob3N0Q29uZmlnLnBsdWdpbklkYCkgaW4gc3R1ZGlvIG1vZGUsIG90aGVyd2lzZSB0aGUgcmVzb2x2ZWQgc2luZ2xlLWFwcCB2YXJpYW50LlxuICAgKiBFdmVyeSBvdGhlciByZWdpc3RyeSBlbnRyeSBzdHJlYW1zIGluIGluZGVwZW5kZW50bHkgYW5kIGlzIG5ldmVyIGZhdGFsIHRvIGJvb3QuICovXG4gIGNvbnN0IHByaW1hcnlQbHVnaW5JZCA9IHVzZU1lbW8oKCkgPT4gaG9zdENvbmZpZz8ucGx1Z2luSWQgPz8gKHBsdWdpbkZpbHRlciA/IHJlc29sdmVQbHVnaW5SZWdpc3RyeUlkKHBsdWdpbkZpbHRlcikgOiB1bmRlZmluZWQpID8/IHJlZ2lzdHJ5WzBdPy5wbHVnaW5JZCwgW2hvc3RDb25maWcsIHBsdWdpbkZpbHRlciwgcmVnaXN0cnldKTtcbiAgY29uc3Qgc2hlbGxQbHVnaW5DYW52YXNTdGF0dXMgPSB1c2VNZW1vKCgpOiBVaVN0YXR1cyB8IHVuZGVmaW5lZCA9PiB7XG4gICAgaWYgKCFzZXNzaW9uKSByZXR1cm4gXCJsb2FkaW5nXCI7XG4gICAgaWYgKCFwcmltYXJ5UGx1Z2luSWQpIHJldHVybiB1bmRlZmluZWQ7XG4gICAgY29uc3QgcGx1Z2luU3RhdHVzID0gcGx1Z2luU3RhdHVzQnlJZFtwcmltYXJ5UGx1Z2luSWRdO1xuICAgIGlmIChwbHVnaW5TdGF0dXMgPT09IFwiaW5zdGFsbGluZ1wiIHx8IHBsdWdpblN0YXR1cyA9PT0gXCJyZWxvYWRpbmdcIikgcmV0dXJuIFwibG9hZGluZ1wiO1xuICAgIHJldHVybiB1bmRlZmluZWQ7XG4gIH0sIFtzZXNzaW9uLCBwcmltYXJ5UGx1Z2luSWQsIHBsdWdpblN0YXR1c0J5SWRdKTtcbiAgLyoqIPCflIzvuI8gRGV2LW9ubHkgdG9kYXkgKGBjcmVhdGVEZXZQbHVnaW5Tb3VyY2VgKSDigJQgYSBmdXR1cmUgaHViLWJhY2tlZCBzb3VyY2UgaW1wbGVtZW50cyB0aGUgc2FtZVxuICAgKiBgUGx1Z2luU291cmNlYCBjb250cmFjdCBhbmQgc3dhcHMgaW4gaGVyZSB3aXRoIG5vIG90aGVyIGNoYW5nZSB0byB0aGUgcnVudGltZSBiZWxvdy4gKi9cbiAgY29uc3QgcGx1Z2luU291cmNlOiBQbHVnaW5Tb3VyY2UgPSB1c2VNZW1vKCgpID0+IGNyZWF0ZURldlBsdWdpblNvdXJjZShyZWdpc3RyeSksIFtyZWdpc3RyeV0pO1xuXG4gIC8qKiDwn5SM77iPIFJlY3JlYXRlcyB0aGUgcHJpbWFyeSBzZXNzaW9uIGluc3RhbmNlIGZvciBgaGFuZGxlYCDigJQgdGhlIGV4YWN0IGBob3N0Q29uZmlnYC9ub24tc3R1ZGlvXG4gICAqIGFwcC1yZXNvbHV0aW9uIGxvZ2ljIHRoZSBib290IGVmZmVjdCB1c2VkIHRvIHJ1biBvbmNlIGlubGluZSwgbm93IHNoYXJlZCB3aXRoIGByZWxvYWRQbHVnaW5gIHNvIGFcbiAgICogaG90LXN3YXAgb2YgdGhlIHNlc3Npb24tb3duaW5nIHBsdWdpbiByZS1lc3RhYmxpc2hlcyB0aGUgc2Vzc2lvbiB0aGUgc2FtZSB3YXkgYm9vdCBkb2VzLiAqL1xuICBjb25zdCBlc3RhYmxpc2hQcmltYXJ5U2Vzc2lvbiA9IHVzZUNhbGxiYWNrKFxuICAgIGFzeW5jIChoYW5kbGU6IFBsdWdpbldhc21IYW5kbGUpID0+IHtcbiAgICAgIGNvbnN0IG1hbmlmZXN0ID0gaGFuZGxlLm1hbmlmZXN0O1xuICAgICAgaWYgKGhvc3RDb25maWcpIHtcbiAgICAgICAgY29uc3Qgc0FwcCA9IG1hbmlmZXN0LmFwcHMuZmluZCgoYXBwKSA9PiBhcHAuaWQgPT09IGhvc3RDb25maWcubGFuZGluZ0FwcElkKSA/PyBtYW5pZmVzdC5hcHBzWzBdO1xuICAgICAgICBpZiAoIXNBcHApIHRocm93IG5ldyBFcnJvcihcImhvc3QgcHJvZ3JhbSBtaXNzaW5nIGxhbmRpbmcgYXBwXCIpO1xuICAgICAgICAvLyDwn6qm77iPIGBtYW5pZmVzdC53b3JrZmxvd3NgICh0aGUgc291cmNlIGBidWlsZFNwYWNlUHJvZ3JhbXNgIHVzZWQgdG8gcmVhZCkgd2FzIGRlbGV0ZWQgZnJvbSB0aGVcbiAgICAgICAgLy8gUnVzdCBgUGx1Z2luTWFuaWZlc3RgIOKAlCB0aGUgc3R1ZGlvIGNhdGFsb2d1ZSBpcyBub3cgcmVnaXN0cnktZHJpdmVuIChzZWUgYFNwYWNlQ29tbWFuZDo6U2V0QXBwUmVnaXN0cmF0aW9uc2ApLFxuICAgICAgICAvLyBzbyBgU3BhY2VQYW5lbFN0YXRlLnByb2dyYW1zYCBpcyBwZXJtYW5lbnRseSBlbXB0eTsgYHNwYXduZWRBcHBzYC9gYWN0aXZlUGFuZWxUYWJgL2BhY3RpdmVTcGF3bmVkSWRgIGFyZVxuICAgICAgICAvLyBzdGlsbCByZWFsLCBsaXZlIHN0YXRlLCBzbyBgU3BhY2VQYW5lbFN0YXRlYCBpdHNlbGYgc3RheXMuXG4gICAgICAgIGNvbnN0IHBhbmVsU3RhdGUgPSBidWlsZFNwYWNlUGFuZWxTdGF0ZShbXSwgW10pO1xuICAgICAgICBjb25zdCBpbnN0YW5jZUlkID0gYXdhaXQgaGFuZGxlLmNyZWF0ZUFwcChzQXBwLmlkKTtcbiAgICAgICAgY29uc3Qgdmlld1N0YXRlOiBWaWV3TW9kZWwgPSB7IGFjdGl2ZU1vZGVJZDogc0FwcC5kZWZhdWx0TW9kZUlkID8/IHNBcHAubW9kZXNbMF0/LmlkLCBwYW5lbEpzb246IHBhbmVsSnNvbkZyb21TdGF0ZShwYW5lbFN0YXRlKSB9O1xuICAgICAgICAvLyDwn6qf77iPIFNlZWQgZGVmYXVsdC1sYXlvdXQgcGFuZXMgKFRvcC9QZXJzcGVjdGl2ZSkgYmVmb3JlIGFueSBlZmZlY3QgY2FuIGZpcmUgYWN0aW9ucyDigJQgb3RoZXJ3aXNlXG4gICAgICAgIC8vIGJvb3QgYHNldEFjdGl2ZUV4YW1wbGVgIHJhY2VzIHRoZSBzZXNzaW9uLXN3aXRjaCByZWZyZXNoIGFuZCB3aXBlcyBwYW5lIGJvZGllcy5cbiAgICAgICAgY29uc3Qgc2VlZGVkID0gYXBwbHlGcmFtZXdvcmtMYXlvdXRTZWVkKHNBcHAuZGVmYXVsdExheW91dCwgc0FwcC53aW5kb3dLaW5kcywgRU1QVFlfQVBQX0xBQkVMU19PVkVSTEFZLCB1aVRlcm1pbm9sb2d5LCB1aUxvY2FsZSk7XG4gICAgICAgIGV4dHJhV2luZG93SW5zdGFuY2VzUmVmLmN1cnJlbnQgPSBzZWVkZWQuZXh0cmFJbnN0YW5jZXM7XG4gICAgICAgIGV4dHJhV2luZG93Q291bnRlclJlZi5jdXJyZW50ID0gc2VlZGVkLmV4dHJhSW5zdGFuY2VzLmxlbmd0aDtcbiAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9TRVNTSU9OXCIsIHZhbHVlOiB7IHBsdWdpbklkOiBoYW5kbGUucGx1Z2luSWQsIGluc3RhbmNlSWQsIGFwcDogc0FwcCwgdmlld1N0YXRlIH0gfSk7XG4gICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfRVhUUkFfV0lORE9XX0lOU1RBTkNFU1wiLCB2YWx1ZTogc2VlZGVkLmV4dHJhSW5zdGFuY2VzIH0pO1xuICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1NIRUxMX0xBWU9VVFwiLCB2YWx1ZTogc2VlZGVkLm1vZGVMYXlvdXQgfSk7XG4gICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfQUNUSVZFX1dJTkRPV19JRFwiLCB2YWx1ZTogbnVsbCB9KTtcbiAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9FUlJPUlwiLCB2YWx1ZTogbnVsbCB9KTtcbiAgICAgICAgcmV0dXJuO1xuICAgICAgfVxuICAgICAgY29uc3QgcHJpbWFyeUFwcCA9IGFwcElkXG4gICAgICAgID8gKCgpID0+IHtcbiAgICAgICAgICAgIGNvbnN0IGZvdW5kID0gbWFuaWZlc3QuYXBwcy5maW5kKChhcHApID0+IGFwcC5pZCA9PT0gYXBwSWQpO1xuICAgICAgICAgICAgaWYgKCFmb3VuZCkgdGhyb3cgbmV3IEVycm9yKGBhcHBJZCBcIiR7YXBwSWR9XCIgZG9lcyBub3QgcmVzb2x2ZSB0byBhbnkgYXBwIGluIHRoZSBsb2FkZWQgcHJvZ3JhbSBtYW5pZmVzdGApO1xuICAgICAgICAgICAgcmV0dXJuIGZvdW5kO1xuICAgICAgICAgIH0pKClcbiAgICAgICAgOiAoKCkgPT4ge1xuICAgICAgICAgICAgY29uc3QgZGVmYXVsdEFwcElkID0gcGx1Z2luRmlsdGVyID8gcmVzb2x2ZVBsYXlncm91bmREZWZhdWx0QXBwSWQocGx1Z2luRmlsdGVyKSA6IHVuZGVmaW5lZDtcbiAgICAgICAgICAgIHJldHVybiAoZGVmYXVsdEFwcElkID8gbWFuaWZlc3QuYXBwcy5maW5kKChhcHApID0+IGFwcC5pZCA9PT0gZGVmYXVsdEFwcElkKSA6IHVuZGVmaW5lZCkgPz8gbWFuaWZlc3QuYXBwc1swXTtcbiAgICAgICAgICB9KSgpO1xuICAgICAgaWYgKCFwcmltYXJ5QXBwKSByZXR1cm47XG4gICAgICBjb25zdCBpbnN0YW5jZUlkID0gYXdhaXQgaGFuZGxlLmNyZWF0ZUFwcChwcmltYXJ5QXBwLmlkKTtcbiAgICAgIGNvbnN0IHNlZWRlZCA9IGFwcGx5RnJhbWV3b3JrTGF5b3V0U2VlZChwcmltYXJ5QXBwLmRlZmF1bHRMYXlvdXQsIHByaW1hcnlBcHAud2luZG93S2luZHMsIEVNUFRZX0FQUF9MQUJFTFNfT1ZFUkxBWSwgdWlUZXJtaW5vbG9neSwgdWlMb2NhbGUpO1xuICAgICAgZXh0cmFXaW5kb3dJbnN0YW5jZXNSZWYuY3VycmVudCA9IHNlZWRlZC5leHRyYUluc3RhbmNlcztcbiAgICAgIGV4dHJhV2luZG93Q291bnRlclJlZi5jdXJyZW50ID0gc2VlZGVkLmV4dHJhSW5zdGFuY2VzLmxlbmd0aDtcbiAgICAgIGRpc3BhdGNoKHtcbiAgICAgICAgdHlwZTogXCJTRVRfU0VTU0lPTlwiLFxuICAgICAgICB2YWx1ZTogeyBwbHVnaW5JZDogaGFuZGxlLnBsdWdpbklkLCBpbnN0YW5jZUlkLCBhcHA6IHByaW1hcnlBcHAsIHZpZXdTdGF0ZTogeyBhY3RpdmVNb2RlSWQ6IHByaW1hcnlBcHAuZGVmYXVsdE1vZGVJZCA/PyBwcmltYXJ5QXBwLm1vZGVzWzBdPy5pZCB9IH0sXG4gICAgICB9KTtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfRVhUUkFfV0lORE9XX0lOU1RBTkNFU1wiLCB2YWx1ZTogc2VlZGVkLmV4dHJhSW5zdGFuY2VzIH0pO1xuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9TSEVMTF9MQVlPVVRcIiwgdmFsdWU6IHNlZWRlZC5tb2RlTGF5b3V0IH0pO1xuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9BQ1RJVkVfV0lORE9XX0lEXCIsIHZhbHVlOiBudWxsIH0pO1xuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9FUlJPUlwiLCB2YWx1ZTogbnVsbCB9KTtcbiAgICB9LFxuICAgIFtob3N0Q29uZmlnLCBhcHBJZCwgcGx1Z2luRmlsdGVyLCB1aVRlcm1pbm9sb2d5LCB1aUxvY2FsZV0sXG4gICk7XG5cbiAgLyoqIPCflIzvuI8gSW5zdGFsbHMgYSByZWdpc3RyeSBlbnRyeSB0aGF0IGlzbid0IGxvYWRlZCB5ZXQ6IGFjcXVpcmVzIGl0cyBtb2R1bGUgKHdvcmtlci1iYWNrZWQsIHJlZmNvdW50ZWRcbiAgICog4oCUIHNlZSBgYWNxdWlyZVBsdWdpbk1vZHVsZWApLCB1cHNlcnRzIGl0IGludG8gYGxvYWRlZFBsdWdpbnNgLCBhbmQg4oCUIGlmIHRoaXMgaXMgdGhlIHByaW1hcnkgcGx1Z2luXG4gICAqIGFuZCBubyBzZXNzaW9uIGV4aXN0cyB5ZXQg4oCUIGVzdGFibGlzaGVzIHRoZSBzZXNzaW9uLiBTaGFyZWQgYnkgdGhlIGJvb3QgZWZmZWN0IChwcmltYXJ5IHBsdWdpblxuICAgKiBvbmx5KSBhbmQgdGhlIGBQbHVnaW5Tb3VyY2VgIHN1YnNjcmlwdGlvbiBlZmZlY3QgKGV2ZXJ5IG90aGVyIHBsdWdpbiwgYXMgaXRzIGJ1aWxkIGxhbmRzKS4gKi9cbiAgY29uc3QgaW5zdGFsbFBsdWdpbiA9IHVzZUNhbGxiYWNrKFxuICAgIGFzeW5jIChwbHVnaW5JZDogc3RyaW5nLCByZWJ1aWx0QXQ/OiBudW1iZXIpOiBQcm9taXNlPFBsdWdpbkluc3RhbGxPdXRjb21lPiA9PiB7XG4gICAgICBpZiAocGx1Z2luT3BJbkZsaWdodFJlZi5jdXJyZW50LmhhcyhwbHVnaW5JZCkpIHJldHVybiBcImluLWZsaWdodFwiO1xuICAgICAgaWYgKGxvYWRlZFBsdWdpbnNSZWYuY3VycmVudC5zb21lKChlbnRyeSkgPT4gZW50cnkuaGFuZGxlLnBsdWdpbklkID09PSBwbHVnaW5JZCkpIHJldHVybiBcImFscmVhZHktbG9hZGVkXCI7XG4gICAgICBjb25zdCBlbnRyeSA9IHJlZ2lzdHJ5LmZpbmQoKGNhbmRpZGF0ZSkgPT4gY2FuZGlkYXRlLnBsdWdpbklkID09PSBwbHVnaW5JZCk7XG4gICAgICBpZiAoIWVudHJ5KSByZXR1cm4gXCJtaXNzaW5nLXJlZ2lzdHJ5XCI7XG4gICAgICBwbHVnaW5PcEluRmxpZ2h0UmVmLmN1cnJlbnQuYWRkKHBsdWdpbklkKTtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfUExVR0lOX1NUQVRVU1wiLCBwbHVnaW5JZCwgdmFsdWU6IFwiaW5zdGFsbGluZ1wiIH0pO1xuICAgICAgdHJ5IHtcbiAgICAgICAgY29uc3QgbW9kdWxlVXJsID0gcGx1Z2luU291cmNlLm1vZHVsZVVybChwbHVnaW5JZCwgcmVidWlsdEF0KTtcbiAgICAgICAgY29uc3QgaGFuZGxlID0gYXdhaXQgbG9hZFBsdWdpbk1vZHVsZVJlc2lsaWVudChwbHVnaW5JZCwgbW9kdWxlVXJsKTtcbiAgICAgICAgaWYgKCFoYW5kbGUpIHtcbiAgICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1BMVUdJTl9TVEFUVVNcIiwgcGx1Z2luSWQsIHZhbHVlOiBcImZhaWxlZFwiIH0pO1xuICAgICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfUExVR0lOX1NVUEVSVklTT1JcIiwgcGx1Z2luSWQsIHZhbHVlOiBcImNyYXNoZWRcIiB9KTtcbiAgICAgICAgICByZXR1cm4gXCJmYWlsZWRcIjtcbiAgICAgICAgfVxuICAgICAgICBwbHVnaW5Nb2R1bGVVcmxCeUlkUmVmLmN1cnJlbnQuc2V0KHBsdWdpbklkLCBtb2R1bGVVcmwpO1xuICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiVVBTRVJUX0xPQURFRF9QTFVHSU5cIiwgdmFsdWU6IHsgaGFuZGxlLCBtYW5pZmVzdDogaGFuZGxlLm1hbmlmZXN0IH0gfSk7XG4gICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfUExVR0lOX1NUQVRVU1wiLCBwbHVnaW5JZCwgdmFsdWU6IFwibG9hZGVkXCIgfSk7XG4gICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfUExVR0lOX1NVUEVSVklTT1JcIiwgcGx1Z2luSWQsIHZhbHVlOiBcImxvYWRlZFwiIH0pO1xuICAgICAgICBpZiAocGx1Z2luSWQgPT09IHByaW1hcnlQbHVnaW5JZCAmJiAhc2Vzc2lvblJlZi5jdXJyZW50KSB7XG4gICAgICAgICAgdHJ5IHtcbiAgICAgICAgICAgIGF3YWl0IGVzdGFibGlzaFByaW1hcnlTZXNzaW9uKGhhbmRsZSk7XG4gICAgICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1BMVUdJTl9TVVBFUlZJU09SXCIsIHBsdWdpbklkLCB2YWx1ZTogXCJydW5uaW5nXCIgfSk7XG4gICAgICAgICAgfSBjYXRjaCAoYm9vdEVycm9yKSB7XG4gICAgICAgICAgICBjb25zb2xlLmVycm9yKFwiW0RFQlVHXSBmcmFtZXdvcmsgb3MgYm9vdCBmYWlsZWRcIiwgYm9vdEVycm9yKTtcbiAgICAgICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfRVJST1JcIiwgdmFsdWU6IGJvb3RFcnJvciBpbnN0YW5jZW9mIEVycm9yID8gYm9vdEVycm9yLm1lc3NhZ2UgOiBTdHJpbmcoYm9vdEVycm9yKSB9KTtcbiAgICAgICAgICAgIHJldHVybiBcImZhaWxlZFwiO1xuICAgICAgICAgIH1cbiAgICAgICAgfVxuICAgICAgICByZXR1cm4gXCJsb2FkZWRcIjtcbiAgICAgIH0gZmluYWxseSB7XG4gICAgICAgIHBsdWdpbk9wSW5GbGlnaHRSZWYuY3VycmVudC5kZWxldGUocGx1Z2luSWQpO1xuICAgICAgfVxuICAgIH0sXG4gICAgW3JlZ2lzdHJ5LCBwbHVnaW5Tb3VyY2UsIHByaW1hcnlQbHVnaW5JZCwgZXN0YWJsaXNoUHJpbWFyeVNlc3Npb25dLFxuICApO1xuXG4gIC8qKiDwn5SM77iPIEhvdC1zd2FwcyBhbiBhbHJlYWR5LWxvYWRlZCBwbHVnaW4gdG8gYSBuZXdseSBidWlsdCBtb2R1bGUg4oCUIG1pcnJvcnMgdGhlIG9zLWNvcmUga2VybmVsJ3NcbiAgICogYFBsdWdpbkhvc3Q6OmhvdF9zd2FwX3BsdWdpbmAgY29udHJhY3QgKHZhbGlkYXRlIOKGkiBkZXN0cm95IGFmZmVjdGVkIGluc3RhbmNlcyDihpIgc3dhcCDihpIgcmVjcmVhdGUgdGhlXG4gICAqIHNlc3Npb24gaWYgaXQgd2FzIHRoaXMgcGx1Z2luJ3Mg4oaSIHJlbGVhc2UgdGhlIG9sZCBtb2R1bGUpIHdpdGhvdXQgaW52ZW50aW5nIGEgc2VwYXJhdGUgb25lOlxuICAgKiBhY3F1aXJlcyB0aGUgbmV3IG1vZHVsZSBCRUZPUkUgdGVhcmluZyBhbnl0aGluZyBkb3duICh0aGUgb2xkIGhhbmRsZSBrZWVwcyBzZXJ2aW5nIGNvbmN1cnJlbnRcbiAgICogdHJhZmZpYyBkdXJpbmcgdGhlIHN3YXApLCB2YWxpZGF0ZXMgdGhlIG5ldyBtYW5pZmVzdCBzdGlsbCBkZWNsYXJlcyBhcHBzIChhbmQsIGlmIHRoaXMgcGx1Z2luIG93bnNcbiAgICogdGhlIGFjdGl2ZSBzZXNzaW9uLCBzdGlsbCBkZWNsYXJlcyB0aGUgc2Vzc2lvbidzIGFwcCBpZCksIHRoZW4gb25seSBjb21taXRzLiBBIHZhbGlkYXRpb24gZmFpbHVyZVxuICAgKiBkaXNwb3NlcyB0aGUgbmV3IGxlYXNlIGFuZCBsZWF2ZXMgdGhlIG9sZCBwbHVnaW4gZXhhY3RseSBhcyBpdCB3YXMg4oCUIG5vdGhpbmcgZGVzdHJveWVkLCBzdGF0dXMgYmFja1xuICAgKiB0byBgXCJsb2FkZWRcImAuICovXG4gIGNvbnN0IHJlbG9hZFBsdWdpbiA9IHVzZUNhbGxiYWNrKFxuICAgIGFzeW5jIChwbHVnaW5JZDogc3RyaW5nLCByZWJ1aWx0QXQ/OiBudW1iZXIpID0+IHtcbiAgICAgIGlmIChwbHVnaW5PcEluRmxpZ2h0UmVmLmN1cnJlbnQuaGFzKHBsdWdpbklkKSkgcmV0dXJuO1xuICAgICAgY29uc3QgY3VycmVudCA9IGxvYWRlZFBsdWdpbnNSZWYuY3VycmVudC5maW5kKChlbnRyeSkgPT4gZW50cnkuaGFuZGxlLnBsdWdpbklkID09PSBwbHVnaW5JZCk7XG4gICAgICBpZiAoIWN1cnJlbnQpIHJldHVybiBpbnN0YWxsUGx1Z2luKHBsdWdpbklkLCByZWJ1aWx0QXQpO1xuICAgICAgY29uc3Qgb2xkTW9kdWxlVXJsID0gcGx1Z2luTW9kdWxlVXJsQnlJZFJlZi5jdXJyZW50LmdldChwbHVnaW5JZCk7XG4gICAgICBwbHVnaW5PcEluRmxpZ2h0UmVmLmN1cnJlbnQuYWRkKHBsdWdpbklkKTtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfUExVR0lOX1NUQVRVU1wiLCBwbHVnaW5JZCwgdmFsdWU6IFwicmVsb2FkaW5nXCIgfSk7XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1BMVUdJTl9TVVBFUlZJU09SXCIsIHBsdWdpbklkLCB2YWx1ZTogXCJyZXN0YXJ0aW5nXCIgfSk7XG4gICAgICBsZXQgbmV3SGFuZGxlOiBQbHVnaW5XYXNtSGFuZGxlIHwgbnVsbCA9IG51bGw7XG4gICAgICB0cnkge1xuICAgICAgICBjb25zdCBtb2R1bGVVcmwgPSBwbHVnaW5Tb3VyY2UubW9kdWxlVXJsKHBsdWdpbklkLCByZWJ1aWx0QXQpO1xuICAgICAgICBuZXdIYW5kbGUgPSBhd2FpdCBsb2FkUGx1Z2luTW9kdWxlUmVzaWxpZW50KHBsdWdpbklkLCBtb2R1bGVVcmwpO1xuICAgICAgICBpZiAoIW5ld0hhbmRsZSkgdGhyb3cgbmV3IEVycm9yKGBwcm9ncmFtICR7cGx1Z2luSWR9IGZhaWxlZCB0byByZWxvYWRgKTtcbiAgICAgICAgaWYgKG5ld0hhbmRsZS5tYW5pZmVzdC5hcHBzLmxlbmd0aCA9PT0gMCkgdGhyb3cgbmV3IEVycm9yKGBwcm9ncmFtICR7cGx1Z2luSWR9IHJlbG9hZCBkZWNsYXJlcyBubyBhcHBzYCk7XG4gICAgICAgIGNvbnN0IGFjdGl2ZVNlc3Npb24gPSBzZXNzaW9uUmVmLmN1cnJlbnQ7XG4gICAgICAgIGNvbnN0IG93bnNTZXNzaW9uID0gYWN0aXZlU2Vzc2lvbj8ucGx1Z2luSWQgPT09IHBsdWdpbklkO1xuICAgICAgICBpZiAob3duc1Nlc3Npb24gJiYgYWN0aXZlU2Vzc2lvbiAmJiAhbmV3SGFuZGxlLm1hbmlmZXN0LmFwcHMuc29tZSgoYXBwKSA9PiBhcHAuaWQgPT09IGFjdGl2ZVNlc3Npb24uYXBwLmlkKSkge1xuICAgICAgICAgIHRocm93IG5ldyBFcnJvcihgcHJvZ3JhbSAke3BsdWdpbklkfSByZWxvYWQgZHJvcHBlZCB0aGUgYWN0aXZlIHNlc3Npb24ncyBhcHAgXCIke2FjdGl2ZVNlc3Npb24uYXBwLmlkfVwiYCk7XG4gICAgICAgIH1cblxuICAgICAgICBjb25zdCBvbGRBcHBJZHMgPSBuZXcgU2V0KGN1cnJlbnQubWFuaWZlc3QuYXBwcy5tYXAoKGFwcCkgPT4gYXBwLmlkKSk7XG4gICAgICAgIGNvbnN0IG5ld0FwcElkcyA9IG5ldyBTZXQobmV3SGFuZGxlLm1hbmlmZXN0LmFwcHMubWFwKChhcHApID0+IGFwcC5pZCkpO1xuICAgICAgICBjb25zdCBob3RTd2FwRXZlbnQ6IFByb2dyYW1Ib3RTd2FwRXZlbnQgPSB7XG4gICAgICAgICAgcGx1Z2luSWQsXG4gICAgICAgICAgdmVyc2lvbjogbmV3SGFuZGxlLm1hbmlmZXN0LnZlcnNpb24sXG4gICAgICAgICAgYWRkZWRBcHBzOiBbLi4ubmV3QXBwSWRzXS5maWx0ZXIoKGlkKSA9PiAhb2xkQXBwSWRzLmhhcyhpZCkpLFxuICAgICAgICAgIHJlbW92ZWRBcHBzOiBbLi4ub2xkQXBwSWRzXS5maWx0ZXIoKGlkKSA9PiAhbmV3QXBwSWRzLmhhcyhpZCkpLFxuICAgICAgICB9O1xuICAgICAgICBjb25zb2xlLmxvZyhgW0RFQlVHXSBob3Qtc3dhcCAke3BsdWdpbklkfWAsIGhvdFN3YXBFdmVudCk7XG5cbiAgICAgICAgLy8g8J+qpu+4jyBEZXN0cm95IHRoaXMgcGx1Z2luJ3MgbGl2ZSBpbnN0YW5jZXMgdW5kZXIgdGhlIE9MRCBoYW5kbGUgYmVmb3JlIHN3YXBwaW5nIOKAlCB0aGUgcHJpbWFyeVxuICAgICAgICAvLyBzZXNzaW9uIGluc3RhbmNlIChpZiBvd25lZCksIGV2ZXJ5IHN0dWRpby1zcGF3bmVkIGluc3RhbmNlLCBhbmQgYW55IGV4dGVybmFsLXNsb3QgY29udHJpYnV0b3JcbiAgICAgICAgLy8gaW5zdGFuY2UuIE1pcnJvcnMgdGhlIHNoZWxsLXVubW91bnQgdGVhcmRvd24gZWZmZWN0LCBzY29wZWQgdG8gb25lIHBsdWdpbklkIGluc3RlYWQgb2YgZXZlcnlcbiAgICAgICAgLy8gbG9hZGVkIHBsdWdpbi5cbiAgICAgICAgaWYgKG93bnNTZXNzaW9uICYmIGFjdGl2ZVNlc3Npb24pIHtcbiAgICAgICAgICBhd2FpdCBjdXJyZW50LmhhbmRsZS5kZXN0cm95QXBwKGFjdGl2ZVNlc3Npb24uaW5zdGFuY2VJZCkuY2F0Y2goKCkgPT4ge30pO1xuICAgICAgICB9XG4gICAgICAgIGZvciAoY29uc3Qgc3Bhd25lZCBvZiBzcGF3bmVkQXBwc1JlZi5jdXJyZW50LmZpbHRlcigoZW50cnkpID0+IGVudHJ5LnBsdWdpbklkID09PSBwbHVnaW5JZCkpIHtcbiAgICAgICAgICBhd2FpdCBjdXJyZW50LmhhbmRsZS5kZXN0cm95QXBwKHNwYXduZWQuaW5zdGFuY2VJZCkuY2F0Y2goKCkgPT4ge30pO1xuICAgICAgICB9XG4gICAgICAgIGNvbnN0IGNvbnRyaWJ1dG9ySW5zdGFuY2VJZCA9IGNvbnRyaWJ1dG9ySW5zdGFuY2VzUmVmLmN1cnJlbnQuZ2V0KHBsdWdpbklkKTtcbiAgICAgICAgaWYgKGNvbnRyaWJ1dG9ySW5zdGFuY2VJZCAhPSBudWxsKSB7XG4gICAgICAgICAgYXdhaXQgY3VycmVudC5oYW5kbGUuZGVzdHJveUFwcChjb250cmlidXRvckluc3RhbmNlSWQpLmNhdGNoKCgpID0+IHt9KTtcbiAgICAgICAgICBjb250cmlidXRvckluc3RhbmNlc1JlZi5jdXJyZW50LmRlbGV0ZShwbHVnaW5JZCk7XG4gICAgICAgIH1cbiAgICAgICAgaWYgKHN0dWRpb01vZGUgJiYgYWN0aXZlU2Vzc2lvbikge1xuICAgICAgICAgIGNvbnN0IGN1cnJlbnRQYW5lbCA9IHBhcnNlUGFuZWxTdGF0ZShhY3RpdmVTZXNzaW9uLnZpZXdTdGF0ZSk7XG4gICAgICAgICAgY29uc3QgZHJvcHBlZCA9IGN1cnJlbnRQYW5lbD8uc3Bhd25lZEFwcHMuZmlsdGVyKChlbnRyeSkgPT4gZW50cnkucGx1Z2luSWQgPT09IHBsdWdpbklkKSA/PyBbXTtcbiAgICAgICAgICBpZiAoY3VycmVudFBhbmVsICYmIGRyb3BwZWQubGVuZ3RoID4gMCkge1xuICAgICAgICAgICAgY29uc29sZS5sb2coXG4gICAgICAgICAgICAgIGBbREVCVUddIGhvdC1zd2FwICR7cGx1Z2luSWR9IGRyb3BwZWQgJHtkcm9wcGVkLmxlbmd0aH0gc3Bhd25lZCBpbnN0YW5jZShzKWAsXG4gICAgICAgICAgICAgIGRyb3BwZWQubWFwKChlbnRyeSkgPT4gZW50cnkuaWQpLFxuICAgICAgICAgICAgKTtcbiAgICAgICAgICAgIGNvbnN0IHN1cnZpdmluZ1NwYXduZWQgPSBjdXJyZW50UGFuZWwuc3Bhd25lZEFwcHMuZmlsdGVyKChlbnRyeSkgPT4gZW50cnkucGx1Z2luSWQgIT09IHBsdWdpbklkKTtcbiAgICAgICAgICAgIGNvbnN0IGFjdGl2ZVNwYXduZWRJZCA9IGN1cnJlbnRQYW5lbC5hY3RpdmVTcGF3bmVkSWQgJiYgZHJvcHBlZC5zb21lKChlbnRyeSkgPT4gZW50cnkuaWQgPT09IGN1cnJlbnRQYW5lbC5hY3RpdmVTcGF3bmVkSWQpID8gdW5kZWZpbmVkIDogY3VycmVudFBhbmVsLmFjdGl2ZVNwYXduZWRJZDtcbiAgICAgICAgICAgIGNvbnN0IG5leHRQYW5lbCA9IHsgLi4uY3VycmVudFBhbmVsLCBzcGF3bmVkQXBwczogc3Vydml2aW5nU3Bhd25lZCwgYWN0aXZlU3Bhd25lZElkIH07XG4gICAgICAgICAgICBkaXNwYXRjaCh7XG4gICAgICAgICAgICAgIHR5cGU6IFwiU0VUX1NFU1NJT05cIixcbiAgICAgICAgICAgICAgdmFsdWU6IChuZXh0U2Vzc2lvbikgPT4gKG5leHRTZXNzaW9uID8geyAuLi5uZXh0U2Vzc2lvbiwgdmlld1N0YXRlOiB7IC4uLm5leHRTZXNzaW9uLnZpZXdTdGF0ZSwgcGFuZWxKc29uOiBwYW5lbEpzb25Gcm9tU3RhdGUobmV4dFBhbmVsKSB9IH0gOiBuZXh0U2Vzc2lvbiksXG4gICAgICAgICAgICB9KTtcbiAgICAgICAgICB9XG4gICAgICAgIH1cblxuICAgICAgICBwbHVnaW5Nb2R1bGVVcmxCeUlkUmVmLmN1cnJlbnQuc2V0KHBsdWdpbklkLCBtb2R1bGVVcmwpO1xuICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiVVBTRVJUX0xPQURFRF9QTFVHSU5cIiwgdmFsdWU6IHsgaGFuZGxlOiBuZXdIYW5kbGUsIG1hbmlmZXN0OiBuZXdIYW5kbGUubWFuaWZlc3QgfSB9KTtcbiAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9QTFVHSU5fU1RBVFVTXCIsIHBsdWdpbklkLCB2YWx1ZTogXCJsb2FkZWRcIiB9KTtcbiAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9QTFVHSU5fU1VQRVJWSVNPUlwiLCBwbHVnaW5JZCwgdmFsdWU6IG93bnNTZXNzaW9uID8gXCJydW5uaW5nXCIgOiBcImxvYWRlZFwiIH0pO1xuXG4gICAgICAgIGlmIChvd25zU2Vzc2lvbikgYXdhaXQgZXN0YWJsaXNoUHJpbWFyeVNlc3Npb24obmV3SGFuZGxlKTtcblxuICAgICAgICBjdXJyZW50LmhhbmRsZS5kaXNwb3NlKCk7XG4gICAgICAgIGlmIChvbGRNb2R1bGVVcmwpIGV2aWN0UGx1Z2luTW9kdWxlKG9sZE1vZHVsZVVybCk7XG4gICAgICB9IGNhdGNoIChlcnJvcikge1xuICAgICAgICBjb25zb2xlLndhcm4oYFtERUJVR10gaG90LXN3YXAgcm9sbGVkIGJhY2sgZm9yICR7cGx1Z2luSWR9YCwgZXJyb3IpO1xuICAgICAgICBuZXdIYW5kbGU/LmRpc3Bvc2UoKTtcbiAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9QTFVHSU5fU1RBVFVTXCIsIHBsdWdpbklkLCB2YWx1ZTogXCJsb2FkZWRcIiB9KTtcbiAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9QTFVHSU5fU1VQRVJWSVNPUlwiLCBwbHVnaW5JZCwgdmFsdWU6IFwiY3Jhc2hlZFwiIH0pO1xuICAgICAgfSBmaW5hbGx5IHtcbiAgICAgICAgcGx1Z2luT3BJbkZsaWdodFJlZi5jdXJyZW50LmRlbGV0ZShwbHVnaW5JZCk7XG4gICAgICB9XG4gICAgfSxcbiAgICBbaW5zdGFsbFBsdWdpbiwgZXN0YWJsaXNoUHJpbWFyeVNlc3Npb24sIHN0dWRpb01vZGUsIHBsdWdpblNvdXJjZV0sXG4gICk7XG5cbiAgLyoqIPCflIzvuI8gUmVtb3ZlcyBhbiBhbHJlYWR5LWxvYWRlZCBwbHVnaW46IHJlZnVzZXMgdGhlIGhvc3QvcHJpbWFyeSBwbHVnaW4gYW5kIHdoaWNoZXZlciBwbHVnaW4gb3ducyB0aGVcbiAgICogYWN0aXZlIHNlc3Npb24gKHRoZXJlIGlzIG5vdGhpbmcgdG8gZmFsbCBiYWNrIHRvKSwgb3RoZXJ3aXNlIGRlc3Ryb3lzIGl0cyBsaXZlIGluc3RhbmNlcyB0aGUgc2FtZVxuICAgKiB3YXkgYHJlbG9hZFBsdWdpbmAgZG9lcywgZHJvcHMgaXQgZnJvbSBgbG9hZGVkUGx1Z2luc2AsIGFuZCBldmljdHMgaXRzIG1vZHVsZSBsZWFzZSBpbW1lZGlhdGVseVxuICAgKiAocmF0aGVyIHRoYW4gdGhlIHBvb2wncyBub3JtYWwgMzBzIGxpbmdlciDigJQgZnJlZWluZyBpdCByaWdodCBhd2F5IGlzIHRoZSBwb2ludCBvZiBhbiBleHBsaWNpdFxuICAgKiB1bmluc3RhbGwpLiAqL1xuICBjb25zdCB1bmluc3RhbGxQbHVnaW4gPSB1c2VDYWxsYmFjayhcbiAgICBhc3luYyAocGx1Z2luSWQ6IHN0cmluZykgPT4ge1xuICAgICAgaWYgKHBsdWdpbk9wSW5GbGlnaHRSZWYuY3VycmVudC5oYXMocGx1Z2luSWQpKSByZXR1cm47XG4gICAgICBjb25zdCBjdXJyZW50ID0gbG9hZGVkUGx1Z2luc1JlZi5jdXJyZW50LmZpbmQoKGVudHJ5KSA9PiBlbnRyeS5oYW5kbGUucGx1Z2luSWQgPT09IHBsdWdpbklkKTtcbiAgICAgIGlmICghY3VycmVudCkgcmV0dXJuO1xuICAgICAgaWYgKHBsdWdpbklkID09PSBwcmltYXJ5UGx1Z2luSWQpIHtcbiAgICAgICAgY29uc29sZS53YXJuKGBbREVCVUddIHJlZnVzaW5nIHRvIHVuaW5zdGFsbCB0aGUgaG9zdC9wcmltYXJ5IHBsdWdpbjogJHtwbHVnaW5JZH1gKTtcbiAgICAgICAgcmV0dXJuO1xuICAgICAgfVxuICAgICAgaWYgKHNlc3Npb25SZWYuY3VycmVudD8ucGx1Z2luSWQgPT09IHBsdWdpbklkKSB7XG4gICAgICAgIGNvbnNvbGUud2FybihgW0RFQlVHXSByZWZ1c2luZyB0byB1bmluc3RhbGwgdGhlIGFjdGl2ZSBzZXNzaW9uJ3MgcGx1Z2luOiAke3BsdWdpbklkfWApO1xuICAgICAgICByZXR1cm47XG4gICAgICB9XG4gICAgICBwbHVnaW5PcEluRmxpZ2h0UmVmLmN1cnJlbnQuYWRkKHBsdWdpbklkKTtcbiAgICAgIHRyeSB7XG4gICAgICAgIGZvciAoY29uc3Qgc3Bhd25lZCBvZiBzcGF3bmVkQXBwc1JlZi5jdXJyZW50LmZpbHRlcigoZW50cnkpID0+IGVudHJ5LnBsdWdpbklkID09PSBwbHVnaW5JZCkpIHtcbiAgICAgICAgICBhd2FpdCBjdXJyZW50LmhhbmRsZS5kZXN0cm95QXBwKHNwYXduZWQuaW5zdGFuY2VJZCkuY2F0Y2goKCkgPT4ge30pO1xuICAgICAgICB9XG4gICAgICAgIGNvbnN0IGNvbnRyaWJ1dG9ySW5zdGFuY2VJZCA9IGNvbnRyaWJ1dG9ySW5zdGFuY2VzUmVmLmN1cnJlbnQuZ2V0KHBsdWdpbklkKTtcbiAgICAgICAgaWYgKGNvbnRyaWJ1dG9ySW5zdGFuY2VJZCAhPSBudWxsKSB7XG4gICAgICAgICAgYXdhaXQgY3VycmVudC5oYW5kbGUuZGVzdHJveUFwcChjb250cmlidXRvckluc3RhbmNlSWQpLmNhdGNoKCgpID0+IHt9KTtcbiAgICAgICAgICBjb250cmlidXRvckluc3RhbmNlc1JlZi5jdXJyZW50LmRlbGV0ZShwbHVnaW5JZCk7XG4gICAgICAgIH1cbiAgICAgICAgaWYgKHN0dWRpb01vZGUgJiYgc2Vzc2lvblJlZi5jdXJyZW50KSB7XG4gICAgICAgICAgY29uc3QgYWN0aXZlU2Vzc2lvbiA9IHNlc3Npb25SZWYuY3VycmVudDtcbiAgICAgICAgICBjb25zdCBjdXJyZW50UGFuZWwgPSBwYXJzZVBhbmVsU3RhdGUoYWN0aXZlU2Vzc2lvbi52aWV3U3RhdGUpO1xuICAgICAgICAgIGNvbnN0IGRyb3BwZWQgPSBjdXJyZW50UGFuZWw/LnNwYXduZWRBcHBzLmZpbHRlcigoZW50cnkpID0+IGVudHJ5LnBsdWdpbklkID09PSBwbHVnaW5JZCkgPz8gW107XG4gICAgICAgICAgaWYgKGN1cnJlbnRQYW5lbCAmJiBkcm9wcGVkLmxlbmd0aCA+IDApIHtcbiAgICAgICAgICAgIGNvbnN0IHN1cnZpdmluZ1NwYXduZWQgPSBjdXJyZW50UGFuZWwuc3Bhd25lZEFwcHMuZmlsdGVyKChlbnRyeSkgPT4gZW50cnkucGx1Z2luSWQgIT09IHBsdWdpbklkKTtcbiAgICAgICAgICAgIGNvbnN0IGFjdGl2ZVNwYXduZWRJZCA9IGN1cnJlbnRQYW5lbC5hY3RpdmVTcGF3bmVkSWQgJiYgZHJvcHBlZC5zb21lKChlbnRyeSkgPT4gZW50cnkuaWQgPT09IGN1cnJlbnRQYW5lbC5hY3RpdmVTcGF3bmVkSWQpID8gdW5kZWZpbmVkIDogY3VycmVudFBhbmVsLmFjdGl2ZVNwYXduZWRJZDtcbiAgICAgICAgICAgIGNvbnN0IG5leHRQYW5lbCA9IHsgLi4uY3VycmVudFBhbmVsLCBzcGF3bmVkQXBwczogc3Vydml2aW5nU3Bhd25lZCwgYWN0aXZlU3Bhd25lZElkIH07XG4gICAgICAgICAgICBkaXNwYXRjaCh7XG4gICAgICAgICAgICAgIHR5cGU6IFwiU0VUX1NFU1NJT05cIixcbiAgICAgICAgICAgICAgdmFsdWU6IChuZXh0U2Vzc2lvbikgPT4gKG5leHRTZXNzaW9uID8geyAuLi5uZXh0U2Vzc2lvbiwgdmlld1N0YXRlOiB7IC4uLm5leHRTZXNzaW9uLnZpZXdTdGF0ZSwgcGFuZWxKc29uOiBwYW5lbEpzb25Gcm9tU3RhdGUobmV4dFBhbmVsKSB9IH0gOiBuZXh0U2Vzc2lvbiksXG4gICAgICAgICAgICB9KTtcbiAgICAgICAgICB9XG4gICAgICAgIH1cbiAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlJFTU9WRV9MT0FERURfUExVR0lOXCIsIHBsdWdpbklkIH0pO1xuICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1BMVUdJTl9TVEFUVVNcIiwgcGx1Z2luSWQsIHZhbHVlOiBcImF2YWlsYWJsZVwiIH0pO1xuICAgICAgICBjdXJyZW50LmhhbmRsZS5kaXNwb3NlKCk7XG4gICAgICAgIGNvbnN0IG1vZHVsZVVybCA9IHBsdWdpbk1vZHVsZVVybEJ5SWRSZWYuY3VycmVudC5nZXQocGx1Z2luSWQpO1xuICAgICAgICBwbHVnaW5Nb2R1bGVVcmxCeUlkUmVmLmN1cnJlbnQuZGVsZXRlKHBsdWdpbklkKTtcbiAgICAgICAgaWYgKG1vZHVsZVVybCkgZXZpY3RQbHVnaW5Nb2R1bGUobW9kdWxlVXJsKTtcbiAgICAgIH0gZmluYWxseSB7XG4gICAgICAgIHBsdWdpbk9wSW5GbGlnaHRSZWYuY3VycmVudC5kZWxldGUocGx1Z2luSWQpO1xuICAgICAgfVxuICAgIH0sXG4gICAgW3ByaW1hcnlQbHVnaW5JZCwgc3R1ZGlvTW9kZV0sXG4gICk7XG4gIC8vI2VuZHJlZ2lvbiDwn5SM77iPUGx1Z2luUnVudGltZVxuXG4gIC8vIPCfkKLvuI8gTWVtb2l6ZWQgb24gdGhlIHJhdyBgcGFuZWxKc29uYCBzdHJpbmcgKG5vdCBgc2Vzc2lvbmAgb2JqZWN0IGlkZW50aXR5LCB3aGljaCBjaHVybnMgZXZlcnlcbiAgLy8gYWN0aW9uKSBzbyBhIGBzZXNzaW9uYCByZWZyZXNoIHRoYXQgbGVhdmVzIGBwYW5lbEpzb25gIHVudG91Y2hlZCByZXVzZXMgdGhlIHNhbWUgcGFyc2VkIGBwYW5lbGBcbiAgLy8gb2JqZWN0IOKAlCBhIHByZXJlcXVpc2l0ZSBmb3IgYW55IGRvd25zdHJlYW0gYHVzZU1lbW9gL2BSZWFjdC5tZW1vYCBrZXllZCBvbiBgcGFuZWxgIHRvIGJhaWwuXG4gIGNvbnN0IHBhbmVsID0gdXNlTWVtbygoKSA9PiAoc2Vzc2lvbiA/IHBhcnNlUGFuZWxTdGF0ZShzZXNzaW9uLnZpZXdTdGF0ZSkgOiBudWxsKSwgW3Nlc3Npb24/LnZpZXdTdGF0ZS5wYW5lbEpzb25dKTtcbiAgLyoqIPCfkJrvuI8gTWlycm9ycyBgcGFuZWw/LnNwYXduZWRBcHBzYCBmb3IgdGhlIHVubW91bnQtY2xlYW51cCBlZmZlY3QgYmVsb3cg4oCUIHNhbWUgcmF0aW9uYWxlIGFzXG4gICAqIGBsb2FkZWRQbHVnaW5zUmVmYDogbmVlZHMgdGhlIGxhdGVzdCB2YWx1ZSBhdCB0ZWFyZG93biB0aW1lIHdpdGhvdXQgZGVwZW5kaW5nIG9uIGl0LiAqL1xuICBjb25zdCBzcGF3bmVkQXBwc1JlZiA9IHVzZVJlZjxyZWFkb25seSBTcGF3bmVkQXBwRW50cnlbXT4oW10pO1xuICBzcGF3bmVkQXBwc1JlZi5jdXJyZW50ID0gcGFuZWw/LnNwYXduZWRBcHBzID8/IFtdO1xuICBjb25zdCBhY3RpdmVTcGF3bmVkRW50cnkgPSBwYW5lbD8uc3Bhd25lZEFwcHMuZmluZCgoZW50cnkpID0+IGVudHJ5LmlkID09PSBwYW5lbC5hY3RpdmVTcGF3bmVkSWQpO1xuICBjb25zdCBhY3RpdmVBcHBUaXRsZSA9IGFwcERvY3VtZW50TGFiZWwoYWN0aXZlU3Bhd25lZEVudHJ5ID8gcmVzb2x2ZURvY3VtZW50QnlBcHBJZChsb2FkZWRQbHVnaW5zLCBhY3RpdmVTcGF3bmVkRW50cnkuYXBwSWQsIGFjdGl2ZVNwYXduZWRFbnRyeS5kb2N1bWVudCwgdWlUZXJtaW5vbG9neSkgOiBzZXNzaW9uID8gcmVzb2x2ZUFwcERvY3VtZW50KHNlc3Npb24uYXBwLCB1aVRlcm1pbm9sb2d5KSA6IFtdKTtcblxuICB1c2VFZmZlY3QoKCkgPT4ge1xuICAgIHNlc3Npb25SZWYuY3VycmVudCA9IHNlc3Npb247XG4gIH0sIFtzZXNzaW9uXSk7XG5cbiAgLy8g8J+Ok++4jyBBIGJyYW5kLW93bmVkIGludHJvZHVjdGlvbiBmdWxseSByZXBsYWNlcyB0aGUgYXBwJ3Mgb3duIChhbHJlYWR5IGxvY2FsaXplZCwgcmVuZGVyZWQgdmVyYmF0aW0pO1xuICAvLyBpdHMgZmlyc3QtcnVuLXNlZW4gZmxhZyBpcyBicmFuZC1zY29wZWQgc28gdGhlIGJyYW5kZWQgdG91ciBwbGF5cyBldmVuIG9uIGEgZGV2aWNlIHRoYXQgc2F3IHRoZVxuICAvLyB1bmJyYW5kZWQgb25lLiBCcmFuZHMgd2l0aCBgcmVwbGF5SW50cm9kdWN0aW9uT25Mb2FkYCBza2lwIHBlcnNpc3RlbmNlIGFuZCBhdXRvLXN0YXJ0IGV2ZXJ5IGxvYWQuXG4gIGNvbnN0IGFjdGl2ZUludHJvZHVjdGlvbiA9IGJyYW5kPy5pbnRyb2R1Y3Rpb24gPz8gc2Vzc2lvbj8uYXBwLmludHJvZHVjdGlvbjtcbiAgY29uc3QgaW50cm9kdWN0aW9uU2VlbktleSA9IHNlc3Npb24gPyAoYnJhbmQgPyBgJHticmFuZC5pZH06JHtzZXNzaW9uLmFwcC5pZH1gIDogc2Vzc2lvbi5hcHAuaWQpIDogXCJcIjtcbiAgY29uc3QgcmVwbGF5SW50cm9kdWN0aW9uT25Mb2FkID0gc2hvdWxkUmVwbGF5SW50cm9kdWN0aW9uT25Mb2FkKGJyYW5kKTtcbiAgY29uc3QgcGVyc2lzdEludHJvZHVjdGlvblNlZW4gPSBzaG91bGRQZXJzaXN0SW50cm9kdWN0aW9uU2VlbihicmFuZCk7XG4gIGNvbnN0IGFjdGl2ZUludHJvZHVjdGlvblJlZiA9IHVzZVJlZihhY3RpdmVJbnRyb2R1Y3Rpb24pO1xuICBhY3RpdmVJbnRyb2R1Y3Rpb25SZWYuY3VycmVudCA9IGFjdGl2ZUludHJvZHVjdGlvbjtcblxuICAvLyDwn46T77iPIEF1dG8tc3RhcnRzIGFuIGFwcCdzIGludHJvZHVjdGlvbiB0aGUgZmlyc3QgdGltZSBpdCBsYXVuY2hlcyBvbiB0aGlzIGRldmljZSAob3IgZXZlcnkgbG9hZCB3aGVuXG4gIC8vIHRoZSBicmFuZCBvcHRzIGluKTsgcmVwbGF5aW5nIHN0YXlzIGF2YWlsYWJsZSBhZnRlcndhcmQgdmlhIHRoZSBzaGVsbC1vd25lZCBJbnRyb2R1Y2UgQXBwIGNvbW1hbmQuXG4gIC8vIPCfjqXvuI8gTmV2ZXIgYXV0by1zdGFydHMgd2hpbGUgYSB0dXRvcmlhbCBpcyBhY3RpdmUgKG11dHVhbCBleGNsdXNpdml0eSkg4oCUIGBhY3RpdmVUdXRvcmlhbElkYCBpcyBkZWNsYXJlZFxuICAvLyBqdXN0IGJlbG93ICh0aGUgVHV0b3JpYWxPcmNoZXN0cmF0aW9uIGJsb2NrJ3Mgc3RhdGUgcmVzb2x1dGlvbik7IHJlYWQgdmlhIGBzaGVsbFN0YXRlLnR1dG9yaWFsYFxuICAvLyBkaXJlY3RseSBoZXJlIHJhdGhlciB0aGFuIHRoZSBub3QteWV0LWRlY2xhcmVkIGxvY2FsIHRvIGF2b2lkIGEgZGVmaW5pdGlvbi1vcmRlciBkZXBlbmRlbmN5LlxuICB1c2VFZmZlY3QoKCkgPT4ge1xuICAgIGlmICghc2Vzc2lvbiB8fCAhYWN0aXZlSW50cm9kdWN0aW9uIHx8IHNoZWxsU3RhdGUudHV0b3JpYWwuYWN0aXZlVHV0b3JpYWxJZCAhPSBudWxsKSByZXR1cm47XG4gICAgaWYgKHR5cGVvZiB3aW5kb3cgIT09IFwidW5kZWZpbmVkXCIgJiYgd2luZG93LnNlbGYgIT09IHdpbmRvdy50b3ApIHJldHVybjtcbiAgICBpZiAoc3VwcHJlc3NBdXRvSW50cm9kdWN0aW9uKSByZXR1cm47XG4gICAgaWYgKCFyZXBsYXlJbnRyb2R1Y3Rpb25PbkxvYWQgJiYgcmVhZFN0b3JlZEludHJvZHVjdGlvblNlZW4oc2NvcGUuc3RvcmFnZSwgaW50cm9kdWN0aW9uU2VlbktleSkpIHJldHVybjtcbiAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0lOVFJPRFVDVElPTl9TVEVQXCIsIHZhbHVlOiAwIH0pO1xuICB9LCBbc2Vzc2lvbj8uYXBwLmlkLCBhY3RpdmVJbnRyb2R1Y3Rpb24sIGludHJvZHVjdGlvblNlZW5LZXksIHJlcGxheUludHJvZHVjdGlvbk9uTG9hZCwgc2hlbGxTdGF0ZS50dXRvcmlhbC5hY3RpdmVUdXRvcmlhbElkLCBzdXBwcmVzc0F1dG9JbnRyb2R1Y3Rpb25dKTtcblxuICAvLyDwn46l77iPIFplcm8gcGVyLWFwcCB3b3JrOiBhbnkgYXBwL2JyYW5kIHRoYXQgZGVjbGFyZXMgYHR1dG9yaWFsc2AgZ2V0cyBzaGVsbCBzdXBwb3J0IGF1dG9tYXRpY2FsbHkuXG4gIC8vIEJyYW5kLW93bmVkIHR1dG9yaWFscyBhcmUgc2hvd24gQUxPTkdTSURFIHRoZSBhcHAncyBvd24gKG5ldmVyIHJlcGxhY2luZyB0aGVtLCB1bmxpa2UgYGludHJvZHVjdGlvbmApLlxuICBjb25zdCBhY3RpdmVUdXRvcmlhbHMgPSB1c2VNZW1vKCgpOiByZWFkb25seSBUdXRvcmlhbERlZmluaXRpb25bXSA9PiBbLi4uKGJyYW5kPy50dXRvcmlhbHMgPz8gW10pLCAuLi4oc2Vzc2lvbj8uYXBwLnR1dG9yaWFscyA/PyBbXSldLCBbYnJhbmQ/LnR1dG9yaWFscywgc2Vzc2lvbj8uYXBwLnR1dG9yaWFsc10pO1xuICAvKiog4o+677iPIFRoZSByZWNvcmRlciBpcyBkZXYvc3R1ZGlvLW9ubHkg4oCUIFZpdGUgYWx3YXlzIGRlZmluZXMgYGltcG9ydC5tZXRhLmVudi5ERVZgOyBndWFyZGVkIGZvciBub24tVml0ZSAoZS5nLiBgYnVuIHRlc3RgKSBldmFsdWF0aW9uLiAqL1xuICBjb25zdCB0dXRvcmlhbFJlY29yZGVyQXZhaWxhYmxlID0gdXNlTWVtbygoKSA9PiB7XG4gICAgdHJ5IHtcbiAgICAgIHJldHVybiBCb29sZWFuKChpbXBvcnQubWV0YSBhcyB1bmtub3duIGFzIHsgcmVhZG9ubHkgZW52PzogeyByZWFkb25seSBERVY/OiBib29sZWFuIH0gfSkuZW52Py5ERVYpO1xuICAgIH0gY2F0Y2gge1xuICAgICAgcmV0dXJuIGZhbHNlO1xuICAgIH1cbiAgfSwgW10pO1xuXG4gIC8vIPCfp7DvuI8gUmVmcyBzbyBgcmVmcmVzaFVpYC9gb25BY3Rpb25gL2BhcHBseUhvc3RFZmZlY3RzYCBjYW4gcmVhZCB0aGUgY3VycmVudCBob3N0LW93bmVkIGFjdGl2ZSB1dGlsaXR5IGFuZFxuICAvLyBhY3RpdmUgd2luZG93IHdpdGhvdXQgcmUtY3JlYXRpbmcgdGhvc2UgY2FsbGJhY2tzIG9uIGV2ZXJ5IHV0aWxpdHkgc3dpdGNoLlxuICBjb25zdCBhY3RpdmVVdGlsaXR5QnlXaW5kb3dJZFJlZiA9IHVzZVJlZihhY3RpdmVVdGlsaXR5QnlXaW5kb3dJZCk7XG4gIGFjdGl2ZVV0aWxpdHlCeVdpbmRvd0lkUmVmLmN1cnJlbnQgPSBhY3RpdmVVdGlsaXR5QnlXaW5kb3dJZDtcbiAgY29uc3QgYWN0aXZlVG9vbElkUmVmID0gdXNlUmVmKGFjdGl2ZVRvb2xJZCk7XG4gIGFjdGl2ZVRvb2xJZFJlZi5jdXJyZW50ID0gYWN0aXZlVG9vbElkO1xuICAvKiog8J+nsO+4jyBEaXNwYXRjaCArIHN5bmMgdGhlIHJlZiBpbW1lZGlhdGVseSDigJQgYHJlZnJlc2hVaWAgcmVhZHMgdGhlIHJlZiBiZWZvcmUgdGhlIG5leHQgcmVuZGVyLCBzbyBhXG4gICAqIGJhcmUgYGRpc3BhdGNoKFNFVF9BQ1RJVkVfVVRJTElUWSlgIGFsb25lIGxlYXZlcyB0aGUgbWFwIHN0YWxlIGFuZCB0aGUgZ3VtYmFsbCBuZXZlciBhcHBlYXJzLiAqL1xuICBjb25zdCBzZXRBY3RpdmVVdGlsaXR5Rm9yV2luZG93ID0gdXNlQ2FsbGJhY2soKHdpbmRvd0lkOiBzdHJpbmcsIHV0aWxpdHlJZDogc3RyaW5nIHwgbnVsbCkgPT4ge1xuICAgIGFjdGl2ZVV0aWxpdHlCeVdpbmRvd0lkUmVmLmN1cnJlbnQgPSB7IC4uLmFjdGl2ZVV0aWxpdHlCeVdpbmRvd0lkUmVmLmN1cnJlbnQsIFt3aW5kb3dJZF06IHV0aWxpdHlJZCB9O1xuICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfQUNUSVZFX1VUSUxJVFlcIiwgd2luZG93SWQsIHV0aWxpdHlJZCB9KTtcbiAgfSwgW10pO1xuICAvKiog8J+nsO+4jyBDbGVhciBldmVyeSB3aW5kb3cncyB1dGlsaXR5IGluIHRoZSByZWYgKyBzdG9yZSBhdCBvbmNlICh0b29sL3V0aWxpdHkgbXV0dWFsIGV4Y2x1c2lvbikuICovXG4gIGNvbnN0IGNsZWFyQWxsV2luZG93VXRpbGl0aWVzID0gdXNlQ2FsbGJhY2soKCkgPT4ge1xuICAgIGNvbnN0IG5leHQ6IFJlY29yZDxzdHJpbmcsIHN0cmluZyB8IG51bGw+ID0geyAuLi5hY3RpdmVVdGlsaXR5QnlXaW5kb3dJZFJlZi5jdXJyZW50IH07XG4gICAgZm9yIChjb25zdCB3aW5kb3dJZCBvZiBPYmplY3Qua2V5cyhuZXh0KSkge1xuICAgICAgaWYgKG5leHRbd2luZG93SWRdKSB7XG4gICAgICAgIG5leHRbd2luZG93SWRdID0gbnVsbDtcbiAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9BQ1RJVkVfVVRJTElUWVwiLCB3aW5kb3dJZCwgdXRpbGl0eUlkOiBudWxsIH0pO1xuICAgICAgfVxuICAgIH1cbiAgICBhY3RpdmVVdGlsaXR5QnlXaW5kb3dJZFJlZi5jdXJyZW50ID0gbmV4dDtcbiAgfSwgW10pO1xuICBjb25zdCB0b29sTWVhc3VyZXNCeVRvb2xJZFJlZiA9IHVzZVJlZih0b29sTWVhc3VyZXNCeVRvb2xJZCk7XG4gIHRvb2xNZWFzdXJlc0J5VG9vbElkUmVmLmN1cnJlbnQgPSB0b29sTWVhc3VyZXNCeVRvb2xJZDtcbiAgY29uc3QgYWN0aXZlV2luZG93SWRSZWYgPSB1c2VSZWYoYWN0aXZlV2luZG93SWQpO1xuICBhY3RpdmVXaW5kb3dJZFJlZi5jdXJyZW50ID0gYWN0aXZlV2luZG93SWQ7XG4gIGNvbnN0IGFjdGlvblBhbmVFeHBhbmRlZEJ5V2luZG93SWRSZWYgPSB1c2VSZWYoYWN0aW9uUGFuZUV4cGFuZGVkQnlXaW5kb3dJZCk7XG4gIGFjdGlvblBhbmVFeHBhbmRlZEJ5V2luZG93SWRSZWYuY3VycmVudCA9IGFjdGlvblBhbmVFeHBhbmRlZEJ5V2luZG93SWQ7XG4gIGNvbnN0IGFjdGlvblBhbmVTdGFnZWRBcmdzQnlLZXlSZWYgPSB1c2VSZWYoYWN0aW9uUGFuZVN0YWdlZEFyZ3NCeUtleSk7XG4gIGFjdGlvblBhbmVTdGFnZWRBcmdzQnlLZXlSZWYuY3VycmVudCA9IGFjdGlvblBhbmVTdGFnZWRBcmdzQnlLZXk7XG4gIGNvbnN0IGludHJvZHVjdGlvblN0ZXBJbmRleFJlZiA9IHVzZVJlZihpbnRyb2R1Y3Rpb25TdGVwSW5kZXgpO1xuICBpbnRyb2R1Y3Rpb25TdGVwSW5kZXhSZWYuY3VycmVudCA9IGludHJvZHVjdGlvblN0ZXBJbmRleDtcbiAgY29uc3QgaW50cm9kdWN0aW9uQ29tcGxldGVkSW50ZXJhY3Rpb25zUmVmID0gdXNlUmVmKGludHJvZHVjdGlvbkNvbXBsZXRlZEludGVyYWN0aW9ucyk7XG4gIGludHJvZHVjdGlvbkNvbXBsZXRlZEludGVyYWN0aW9uc1JlZi5jdXJyZW50ID0gaW50cm9kdWN0aW9uQ29tcGxldGVkSW50ZXJhY3Rpb25zO1xuXG4gIC8vIPCfjqXvuI8gRm9yd2FyZC1kZWNsYXJlZCByZWZzIHNvIGBvbkFjdGlvbmAgKGRlZmluZWQgYmVsb3csIGJlZm9yZSB0aGUgZnVsbCB0dXRvcmlhbCBvcmNoZXN0cmF0aW9uIGZ1cnRoZXJcbiAgLy8gZG93biB0aGlzIGNvbXBvbmVudCkgY2FuIHNoZWxsLWludGVyY2VwdCBgU1RBUlRfVFVUT1JJQUxfQUNUSU9OX0lEYC9gUkVDT1JEX1RVVE9SSUFMX0FDVElPTl9JRGBcbiAgLy8gd2l0aG91dCBhIGRlZmluaXRpb24tb3JkZXIgY3ljbGUg4oCUIG1pcnJvcnMgdGhlIGBvbkFjdGlvblJlZmAgcGF0dGVybiB1c2VkIHRoZSBvdGhlciB3YXkgYXJvdW5kLlxuICAvLyBQb3B1bGF0ZWQgYnkgdGhlIFR1dG9yaWFsT3JjaGVzdHJhdGlvbiBibG9jaydzIGVmZmVjdCBvbmNlIHRoZSByZWFsIGNhbGxiYWNrcyBleGlzdC5cbiAgY29uc3Qgc3RhcnRUdXRvcmlhbFJlZiA9IHVzZVJlZjwodHV0b3JpYWxJZDogc3RyaW5nKSA9PiB2b2lkPigoKSA9PiB7fSk7XG4gIGNvbnN0IHN0b3BUdXRvcmlhbFJlZiA9IHVzZVJlZjwoKSA9PiB2b2lkPigoKSA9PiB7fSk7XG4gIGNvbnN0IHRvZ2dsZVR1dG9yaWFsUmVjb3JkaW5nUmVmID0gdXNlUmVmPCgpID0+IHZvaWQ+KCgpID0+IHt9KTtcbiAgLyoqIPCfp7LvuI8gVHJ1ZSBmb3IgdGhlIGR1cmF0aW9uIG9mIGFueSBkaXJlY3Rvci9zZWVrL2NvbnZlcmdlLWRyaXZlbiBkaXNwYXRjaCDigJQgYG9uQWN0aW9uYCdzIGRldmlhdGlvblxuICAgKiBjaGVjayBiZWxvdyBza2lwcyBzZXR0aW5nIGBkZXZpYXRlZGAvYXV0by1wYXVzaW5nIGZvciBhbnl0aGluZyBzdGFtcGVkIHdoaWxlIHRoaXMgaXMgdHJ1ZSwgbWlycm9yaW5nXG4gICAqIGhvdyB0aGUgaW50cm9kdWN0aW9uIG1lY2hhbmlzbSdzIG93biBpbnRlcmNlcHRpb24gZGlzdGluZ3Vpc2hlcyBzaGVsbC1vcmlnaW5hdGVkIGZyb20gdXNlci1vcmlnaW5hdGVkXG4gICAqIGFjdGl2aXR5LiBOZXZlciByZWFkIGR1cmluZyByZW5kZXIsIG9ubHkgaW5zaWRlIGV2ZW50IGNhbGxiYWNrcyDigJQgYSBwbGFpbiBtdXRhYmxlIHJlZiBpcyBjb3JyZWN0LiAqL1xuICBjb25zdCB0dXRvcmlhbERyaXZlblJlZiA9IHVzZVJlZihmYWxzZSk7XG4gIGNvbnN0IHR1dG9yaWFsUGxheWluZ1JlZiA9IHVzZVJlZih0dXRvcmlhbFBsYXlpbmcpO1xuICB0dXRvcmlhbFBsYXlpbmdSZWYuY3VycmVudCA9IHR1dG9yaWFsUGxheWluZztcbiAgY29uc3QgdHV0b3JpYWxSZWNvcmRpbmdSZWYgPSB1c2VSZWYodHV0b3JpYWxSZWNvcmRpbmcpO1xuICB0dXRvcmlhbFJlY29yZGluZ1JlZi5jdXJyZW50ID0gdHV0b3JpYWxSZWNvcmRpbmc7XG4gIC8qKiDij7rvuI8gTm9uLW51bGwgd2hpbGUgYXJtZWQg4oCUIG11dGF0ZWQgYnkgYHRvZ2dsZVR1dG9yaWFsUmVjb3JkaW5nYCAoZGVmaW5lZCBpbiB0aGUgVHV0b3JpYWxPcmNoZXN0cmF0aW9uIGJsb2NrIGJlbG93KSwgcmVhZC9hcHBlbmRlZC10byBieSBgb25BY3Rpb25gJ3MgcmVjb3JkZXIgdGFwIHJpZ2h0IGJlbG93LiAqL1xuICBjb25zdCB0dXRvcmlhbFJlY29yZGVyUmVmID0gdXNlUmVmPFR1dG9yaWFsUmVjb3JkZXIgfCBudWxsPihudWxsKTtcbiAgY29uc3Qgc2hlbGxTdGF0ZVJlZiA9IHVzZVJlZihzaGVsbFN0YXRlKTtcbiAgc2hlbGxTdGF0ZVJlZi5jdXJyZW50ID0gc2hlbGxTdGF0ZTtcblxuICAvKiog8J+Ok++4jyBFbmRzIHRoZSBhY3RpdmUgaW50cm9kdWN0aW9uIOKAlCBwZXJzaXN0cyB0aGUgc2VlbiBmbGFnIHdoZW4gY29uZmlndXJlZCwgYW5kIG9uIHN1Y2Nlc3NmdWxcbiAgICogY29tcGxldGlvbiAoRG9uZSAvIGxhc3QgaW50ZXJhY3Rpb24pIGZpcmVzIHRoZSB0b3VyLWZpbmFsZSB7QGxpbmsgY2VsZWJyYXRlQWxsRWxlbWVudHN9IHN0YW1wXG4gICAqIGFjcm9zcyBldmVyeSBtb3VudGVkIFVJIGVsZW1lbnQuIFNraXAvZXNjYXBlIHBhc3NlcyBgY29tcGxldGVkOiBmYWxzZWAgYW5kIGRvZXMgbm90IGNlbGVicmF0ZS4gKi9cbiAgY29uc3QgZGlzbWlzc0ludHJvZHVjdGlvbiA9IHVzZUNhbGxiYWNrKFxuICAgIChjb21wbGV0ZWQ6IGJvb2xlYW4pID0+IHtcbiAgICAgIGlmIChjb21wbGV0ZWQgJiYgc2NvcGUucm9vdFJlZi5jdXJyZW50KSBjZWxlYnJhdGVBbGxFbGVtZW50cyhDRUxFQlJBVEVfU1RBTVBfRFVSQVRJT05fTVMsIHNjb3BlLnJvb3RSZWYuY3VycmVudCk7XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0lOVFJPRFVDVElPTl9TVEVQXCIsIHZhbHVlOiBudWxsIH0pO1xuICAgICAgaWYgKHBlcnNpc3RJbnRyb2R1Y3Rpb25TZWVuKSB3cml0ZVN0b3JlZEludHJvZHVjdGlvblNlZW4oc2NvcGUuc3RvcmFnZSwgaW50cm9kdWN0aW9uU2VlbktleSk7XG4gICAgfSxcbiAgICBbaW50cm9kdWN0aW9uU2VlbktleSwgcGVyc2lzdEludHJvZHVjdGlvblNlZW5dLFxuICApO1xuXG4gIC8qKiDwn46T77iPIFNoYXJlZCBzdGVwLWNvbXBsZXRlIHBhdGg6IGZpcmVzIG9uY2UgZXZlcnkgaW50ZXJhY3Rpb24tZ2F0ZWQgc3RlcCdzIGBpbnRlcmFjdGlvbnNgIGFyZSBhbGwgZG9uZVxuICAgKiAodmlhIGBjb21wbGV0ZUludHJvZHVjdGlvbkludGVyYWN0aW9uYCBiZWxvdyksIGNlbGVicmF0aW5nIGBpbnRyb2R1Y2VgIG9uIHRvcCBvZiBlYWNoIGludGVyYWN0aW9uJ3NcbiAgICogb3duIGNlbGVicmF0aW9uLCB0aGVuIGFkdmFuY2VzIG9yIGZpbmlzaGVzIHRoZSB0b3VyLiBGaW5pc2hpbmcgdGhlIGxhc3Qgc3RlcCBjZWxlYnJhdGVzIGV2ZXJ5IFVJXG4gICAqIGVsZW1lbnQgdmlhIHtAbGluayBkaXNtaXNzSW50cm9kdWN0aW9ufSh0cnVlKSBpbnN0ZWFkIG9mIG9ubHkgdGhlIGludHJvZHVjZSB0YXJnZXQuIGBjZWxlYnJhdGVPdmVycmlkZWBcbiAgICogKHRocmVhZGVkIHRocm91Z2ggZnJvbSBgY29tcGxldGVJbnRyb2R1Y3Rpb25JbnRlcmFjdGlvbmApIG5hcnJvd3MgdGhpcyB0byB0aGUgb25lIGVsZW1lbnQgcmVzcG9uc2libGVcbiAgICogZm9yIHRoZSBqdXN0LWNvbXBsZXRlZCBpbnRlcmFjdGlvbiDigJQgZS5nLiB0aGUgc3BlY2lmaWMgM0Qgd2luZG93IHBhbmUgdGhhdCB3YXMgb3JiaXRlZCDigJQgaW5zdGVhZCBvZlxuICAgKiBldmVyeSBlbGVtZW50IGFsaWFzZWQgdG8gdGhlIHN0ZXAncyBgaW50cm9kdWNlYCBraW5kIChldmVyeSBvcGVuIHBhbmUgb2YgdGhhdCB3aW5kb3cga2luZCkuICovXG4gIGNvbnN0IGFkdmFuY2VJbnRyb2R1Y3Rpb25CeURvaW5nID0gdXNlQ2FsbGJhY2soXG4gICAgKGNlbGVicmF0ZU92ZXJyaWRlPzogc3RyaW5nKSA9PiB7XG4gICAgICBjb25zdCBzdGVwSW5kZXggPSBpbnRyb2R1Y3Rpb25TdGVwSW5kZXhSZWYuY3VycmVudDtcbiAgICAgIGNvbnN0IGludHJvZHVjdGlvbiA9IGFjdGl2ZUludHJvZHVjdGlvblJlZi5jdXJyZW50O1xuICAgICAgaWYgKHN0ZXBJbmRleCA9PSBudWxsIHx8ICFpbnRyb2R1Y3Rpb24pIHJldHVybjtcbiAgICAgIGNvbnN0IHN0ZXAgPSBpbnRyb2R1Y3Rpb24uc3RlcHNbc3RlcEluZGV4XTtcbiAgICAgIGlmIChzdGVwSW5kZXggPj0gaW50cm9kdWN0aW9uLnN0ZXBzLmxlbmd0aCAtIDEpIHtcbiAgICAgICAgZGlzbWlzc0ludHJvZHVjdGlvbih0cnVlKTtcbiAgICAgICAgcmV0dXJuO1xuICAgICAgfVxuICAgICAgY29uc3QgY2VsZWJyYXRlSWQgPSBjZWxlYnJhdGVPdmVycmlkZSA/PyBzdGVwPy5pbnRyb2R1Y2U7XG4gICAgICBpZiAoc3RlcCAmJiAoc3RlcC5pbnRlcmFjdGlvbnMgPz8gW10pLmxlbmd0aCA+IDAgJiYgY2VsZWJyYXRlSWQgJiYgc2NvcGUucm9vdFJlZi5jdXJyZW50KSBjZWxlYnJhdGVFbGVtZW50cyhlbGVtZW50SWRTZWxlY3RvcihjZWxlYnJhdGVJZCksIENFTEVCUkFURV9TVEFNUF9EVVJBVElPTl9NUywgc2NvcGUucm9vdFJlZi5jdXJyZW50KTtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfSU5UUk9EVUNUSU9OX1NURVBcIiwgdmFsdWU6IHN0ZXBJbmRleCArIDEgfSk7XG4gICAgfSxcbiAgICBbZGlzbWlzc0ludHJvZHVjdGlvbl0sXG4gICk7XG5cbiAgLyoqIOKche+4jyBDb21wbGV0ZXMgdGhlIGZpcnN0IG5vdC15ZXQtZG9uZSBpbnRlcmFjdGlvbiBvZiB0aGUgYWN0aXZlIHN0ZXAgbWF0Y2hpbmcgYG1hdGNoZXNgIChyZXNwZWN0aW5nXG4gICAqIGBzdGVwLm9yZGVyZWRgIOKAlCBvbmx5IHRoZSBuZXh0IGluLW9yZGVyIGludGVyYWN0aW9uIG1heSBjb21wbGV0ZSksIGNlbGVicmF0ZXMgaXRzIHRhcmdldCBlbGVtZW50LCBhbmRcbiAgICogYWR2YW5jZXMgdGhlIHN0ZXAgb25jZSBldmVyeSBpbnRlcmFjdGlvbiBpcyBkb25lLiBNaXJyb3JzIHRoZSB3Z3B1IHNoZWxsJ3NcbiAgICogYGNocm9tZV90b3VyX2NvbXBsZXRlX2ludGVyYWN0aW9uYC4gYGNlbGVicmF0ZU92ZXJyaWRlYCDigJQgcGFzc2VkIGJ5IGNhbGxlcnMgdGhhdCBrbm93IGV4YWN0bHkgd2hpY2hcbiAgICogRE9NIGVsZW1lbnQgY2F1c2VkIHRoZSBjb21wbGV0aW9uIChlLmcuIHRoZSBnZXN0dXJlIGludGVyY2VwdCBrbm93cyB0aGUgb25lIHdpbmRvdyBwYW5lIHRoYXQgd2FzXG4gICAqIGFjdHVhbGx5IG9yYml0ZWQpIOKAlCB0YWtlcyBwcmVjZWRlbmNlIG92ZXIgYGludGVyYWN0aW9uLmNlbGVicmF0ZSA/PyBzdGVwLmludHJvZHVjZWAuIFdpdGhvdXQgaXQsIGFcbiAgICogd2luZG93LWtpbmQgYGludHJvZHVjZWAvYGNlbGVicmF0ZWAgaWQgd291bGQgY2VsZWJyYXRlIGV2ZXJ5IHBhbmUgYWxpYXNlZCB0byB0aGF0IGtpbmQsIG5vdCBqdXN0IHRoZVxuICAgKiBvbmUgdGhhdCBjb21wbGV0ZWQgdGhlIGludGVyYWN0aW9uLiAqL1xuICBjb25zdCBjb21wbGV0ZUludHJvZHVjdGlvbkludGVyYWN0aW9uID0gdXNlQ2FsbGJhY2soXG4gICAgKG1hdGNoZXM6IChpbnRlcmFjdGlvbjogSW50cm9kdWN0aW9uSW50ZXJhY3Rpb24pID0+IGJvb2xlYW4sIGNlbGVicmF0ZU92ZXJyaWRlPzogc3RyaW5nKSA9PiB7XG4gICAgICBjb25zdCBzdGVwSW5kZXggPSBpbnRyb2R1Y3Rpb25TdGVwSW5kZXhSZWYuY3VycmVudDtcbiAgICAgIGNvbnN0IGludHJvZHVjdGlvbiA9IGFjdGl2ZUludHJvZHVjdGlvblJlZi5jdXJyZW50O1xuICAgICAgaWYgKHN0ZXBJbmRleCA9PSBudWxsIHx8ICFpbnRyb2R1Y3Rpb24pIHJldHVybjtcbiAgICAgIGNvbnN0IHN0ZXAgPSBpbnRyb2R1Y3Rpb24uc3RlcHNbc3RlcEluZGV4XTtcbiAgICAgIGlmICghc3RlcCB8fCAoc3RlcC5pbnRlcmFjdGlvbnMgPz8gW10pLmxlbmd0aCA9PT0gMCkgcmV0dXJuO1xuICAgICAgY29uc3QgY29tcGxldGVkID0gaW50cm9kdWN0aW9uQ29tcGxldGVkSW50ZXJhY3Rpb25zUmVmLmN1cnJlbnQ7XG4gICAgICBjb25zdCBpbnRlcmFjdGlvbnMgPSBzdGVwLmludGVyYWN0aW9ucyA/PyBbXTtcbiAgICAgIGNvbnN0IGluZGV4ID0gaW50ZXJhY3Rpb25zLmZpbmRJbmRleCgoaW50ZXJhY3Rpb24sIGkpID0+ICFjb21wbGV0ZWQuaW5jbHVkZXMoaSkgJiYgbWF0Y2hlcyhpbnRlcmFjdGlvbikpO1xuICAgICAgaWYgKGluZGV4IDwgMCkgcmV0dXJuO1xuICAgICAgaWYgKHN0ZXAub3JkZXJlZCAmJiBpbmRleCAhPT0gY29tcGxldGVkLmxlbmd0aCkgcmV0dXJuO1xuICAgICAgY29uc3QgY2VsZWJyYXRlSWQgPSBjZWxlYnJhdGVPdmVycmlkZSA/PyBpbnRlcmFjdGlvbnNbaW5kZXhdLmNlbGVicmF0ZSA/PyBzdGVwLmludHJvZHVjZTtcbiAgICAgIGlmIChjZWxlYnJhdGVJZCAmJiBzY29wZS5yb290UmVmLmN1cnJlbnQpIGNlbGVicmF0ZUVsZW1lbnRzKGVsZW1lbnRJZFNlbGVjdG9yKGNlbGVicmF0ZUlkKSwgQ0VMRUJSQVRFX1NUQU1QX0RVUkFUSU9OX01TLCBzY29wZS5yb290UmVmLmN1cnJlbnQpO1xuICAgICAgaW50cm9kdWN0aW9uQ29tcGxldGVkSW50ZXJhY3Rpb25zUmVmLmN1cnJlbnQgPSBbLi4uY29tcGxldGVkLCBpbmRleF07XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiQ09NUExFVEVfSU5UUk9EVUNUSU9OX0lOVEVSQUNUSU9OXCIsIGluZGV4IH0pO1xuICAgICAgaWYgKGludHJvZHVjdGlvbkNvbXBsZXRlZEludGVyYWN0aW9uc1JlZi5jdXJyZW50Lmxlbmd0aCA+PSBpbnRlcmFjdGlvbnMubGVuZ3RoKSBhZHZhbmNlSW50cm9kdWN0aW9uQnlEb2luZyhjZWxlYnJhdGVPdmVycmlkZSk7XG4gICAgfSxcbiAgICBbYWR2YW5jZUludHJvZHVjdGlvbkJ5RG9pbmddLFxuICApO1xuICAvLyDwn46b77iPIFNvIHRoZSBjb21tYW5kLWNhdGVnb3J5IGxlYXZlcycgbGF6aWx5LXJlc29sdmVkIHRyZWUgY29udGVudCAoYnVpbHQgb25jZSBwZXIgcmVzb2x2ZWQtY29tbWFuZHNcbiAgLy8gY2hhbmdlLCBub3QgcGVyIGtleXN0cm9rZSDigJQgc2VlIGBidWlsZENvbW1hbmRDYXRlZ29yeVRhYnNgKSBjYW4gcmVhZCB0aGUgbGF0ZXN0IGV4cGFuZC9zdGFnZWQtYXJnXG4gIC8vIHN0YXRlIHdpdGhvdXQgYmVjb21pbmcgYSBgZGVmYXVsdERvY2tgIG1lbW8gZGVwZW5kZW5jeSwgd2hpY2ggd291bGQgb3RoZXJ3aXNlIHBlcnNpc3Qtd3JpdGUgdGhlIGRvY2tcbiAgLy8gc2tlbGV0b24gb24gZXZlcnkga2V5c3Ryb2tlIHdoaWxlIHN0YWdpbmcgYSBjb21tYW5kIGFyZ3VtZW50LlxuICBjb25zdCBleHBhbmRlZENvbW1hbmRJZFJlZiA9IHVzZVJlZihleHBhbmRlZENvbW1hbmRJZCk7XG4gIGV4cGFuZGVkQ29tbWFuZElkUmVmLmN1cnJlbnQgPSBleHBhbmRlZENvbW1hbmRJZDtcbiAgY29uc3QgY29tbWFuZFN0YWdlZEFyZ3NCeUNvbW1hbmRJZFJlZiA9IHVzZVJlZihjb21tYW5kU3RhZ2VkQXJnc0J5Q29tbWFuZElkKTtcbiAgY29tbWFuZFN0YWdlZEFyZ3NCeUNvbW1hbmRJZFJlZi5jdXJyZW50ID0gY29tbWFuZFN0YWdlZEFyZ3NCeUNvbW1hbmRJZDtcblxuICAvKiog8J+boO+4jyBPdmVybGF5cyB0aGUgbW9kZS1sZXZlbCBob3N0LW93bmVkIGBhY3RpdmVUb29sSWRgIG9udG8gYSB2aWV3IHN0YXRlIGF0IHBsdWdpbi1jYWxsIHRpbWUg4oCUXG4gICAqIG1pcnJvcnMgYGluamVjdEFjdGl2ZVV0aWxpdHlgIGJ1dCBpcyB3aW5kb3dsZXNzIChhIHRvb2wgaXMgc2NvcGVkIHRvIHRoZSBhY3RpdmUgbW9kZSwgbm90IGEgd2luZG93KS4gKi9cbiAgY29uc3QgaW5qZWN0QWN0aXZlVG9vbCA9IHVzZUNhbGxiYWNrKCh2aWV3U3RhdGU6IFZpZXdNb2RlbCk6IFZpZXdNb2RlbCA9PiB7XG4gICAgY29uc3QgdG9vbElkID0gYWN0aXZlVG9vbElkUmVmLmN1cnJlbnQgPz8gdW5kZWZpbmVkO1xuICAgIHJldHVybiB2aWV3U3RhdGUuYWN0aXZlVG9vbElkID09PSB0b29sSWQgPyB2aWV3U3RhdGUgOiB7IC4uLnZpZXdTdGF0ZSwgYWN0aXZlVG9vbElkOiB0b29sSWQgfTtcbiAgfSwgW10pO1xuXG4gIC8qKiDwn6ew77iPIE92ZXJsYXlzIHRoZSBhY3RpdmUgd2luZG93J3MgaG9zdC1vd25lZCBgYWN0aXZlVXRpbGl0eUlkYCAoYW5kIHRoZSBtb2RlJ3MgYGFjdGl2ZVRvb2xJZGApIG9udG8gYSB2aWV3IHN0YXRlIGF0IHBsdWdpbi1jYWxsIHRpbWUuICovXG4gIGNvbnN0IGluamVjdEFjdGl2ZVV0aWxpdHkgPSB1c2VDYWxsYmFjaygodmlld1N0YXRlOiBWaWV3TW9kZWwsIHdpbmRvd0lkPzogc3RyaW5nIHwgbnVsbCk6IFZpZXdNb2RlbCA9PiB7XG4gICAgY29uc3Qga2V5ID0gd2luZG93SWQgPz8gYWN0aXZlV2luZG93SWRSZWYuY3VycmVudDtcbiAgICBjb25zdCB1dGlsaXR5SWQgPSBrZXkgPyAoYWN0aXZlVXRpbGl0eUJ5V2luZG93SWRSZWYuY3VycmVudFtrZXldID8/IHVuZGVmaW5lZCkgOiB1bmRlZmluZWQ7XG4gICAgY29uc3Qgd2l0aFV0aWxpdHkgPSB2aWV3U3RhdGUuYWN0aXZlVXRpbGl0eUlkID09PSB1dGlsaXR5SWQgPyB2aWV3U3RhdGUgOiB7IC4uLnZpZXdTdGF0ZSwgYWN0aXZlVXRpbGl0eUlkOiB1dGlsaXR5SWQgfTtcbiAgICByZXR1cm4gaW5qZWN0QWN0aXZlVG9vbCh3aXRoVXRpbGl0eSk7XG4gIH0sIFtpbmplY3RBY3RpdmVUb29sXSk7XG5cbiAgdXNlRWZmZWN0KCgpID0+IHtcbiAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1NZTkNfQkFDS0JPTkVfVVJJXCIsIHZhbHVlOiBudWxsIH0pO1xuICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfU1lOQ19DQVJEX0tJTkRcIiwgdmFsdWU6IG51bGwgfSk7XG4gIH0sIFtwYW5lbD8uYWN0aXZlU3Bhd25lZElkLCBzZXNzaW9uLCBzdHVkaW9Nb2RlXSk7XG5cbiAgLyoqIPCfkJrvuI8gVGhlIHJlbGF5IGEgZG9jdW1lbnQncyBgcmVnaXN0ZXJQbHVnaW5CYWNrYm9uZVJvdXRlYCBlbnRyeSB1c2VzIOKAlCBmb3J3YXJkcyBhIHBsdWdpbidzIG91dGJvdW5kXG4gICAqIGJhY2tib25lIGJ5dGVzIGludG8gVEhJUyBzaGVsbCdzIG93biBiYWNrYm9uZSB3b3JrZXIuIFJlZ2lzdGVyZWQgcGVyIG9wZW4gZG9jdW1lbnQgKGluXG4gICAqIGBvcGVuRG9jdW1lbnRgL2BjbG9zZURvY3VtZW50YCBiZWxvdykgcmF0aGVyIHRoYW4gb25jZSBmb3IgdGhlIHdob2xlIHNoZWxsOiB0aGUgb2xkIHNpbmdsZVxuICAgKiBwYWdlLWdsb2JhbCByZWxheSBzbG90IChgc2V0UGx1Z2luQmFja2JvbmVPdXRib3VuZFJlbGF5YCkgbWVhbnQgYSBzZWNvbmQgbW91bnRlZCBzaGVsbCBzaWxlbnRseVxuICAgKiBzdG9sZSBldmVyeSBkb2N1bWVudCdzIG91dGJvdW5kIHJvdXRpbmcsIHRoZW4gc2V2ZXJlZCBpdCBlbnRpcmVseSBvbiB0aGF0IHNoZWxsJ3MgdW5tb3VudC4gKi9cbiAgY29uc3QgcmVsYXlQbHVnaW5CYWNrYm9uZU1lc3NhZ2UgPSB1c2VDYWxsYmFjaygodXJpOiBzdHJpbmcsIG1lc3NhZ2VCeXRlczogVWludDhBcnJheSkgPT4ge1xuICAgIGNvbnN0IGRvY3VtZW50SWQgPSB1cmkuc3RhcnRzV2l0aChcImFjdG9yOi8vXCIpID8gdXJpLnNsaWNlKFwiYWN0b3I6Ly9cIi5sZW5ndGgpIDogbnVsbDtcbiAgICBpZiAoIWRvY3VtZW50SWQpIHJldHVybjtcbiAgICBjb25zdCB3b3JrZXIgPSBiYWNrYm9uZVdvcmtlclJlZi5jdXJyZW50O1xuICAgIGlmICghd29ya2VyKSByZXR1cm47XG4gICAgbGV0IGFjdG9yTWVzc2FnZTogRG9jdW1lbnRBY3Rvck1zZztcbiAgICB0cnkge1xuICAgICAgY29uc3QgcGFyc2VkID0gZGVjb2RlQmFja2JvbmVNZXNzYWdlKG1lc3NhZ2VCeXRlcyk7XG4gICAgICBpZiAocGFyc2VkLmtpbmQgPT09IFwib3BlcmF0aW9uc1wiKSB7XG4gICAgICAgIGFjdG9yTWVzc2FnZSA9IHtcbiAgICAgICAgICBraW5kOiBcImxvY2FsT3BlcmF0aW9uc1wiLFxuICAgICAgICAgIGVudmVsb3BlczogcGFyc2VkLmVudmVsb3Blcy5tYXAoKGVudmVsb3BlKSA9PiBvcGVyYXRpb25FbnZlbG9wZUZyb21XaXJlKGVudmVsb3BlKSksXG4gICAgICAgIH07XG4gICAgICB9IGVsc2UgaWYgKHBhcnNlZC5raW5kID09PSBcInNuYXBzaG90XCIpIHtcbiAgICAgICAgYWN0b3JNZXNzYWdlID0geyBraW5kOiBcImxvY2FsU25hcHNob3RcIiwgcGFjazogQXJyYXkuZnJvbShwYXJzZWQucGFjayksIHNwcjogQXJyYXkuZnJvbShwYXJzZWQuc3ByKSB9O1xuICAgICAgfSBlbHNlIHtcbiAgICAgICAgcmV0dXJuO1xuICAgICAgfVxuICAgIH0gY2F0Y2gge1xuICAgICAgcmV0dXJuO1xuICAgIH1cbiAgICBjb25zdCByZXF1ZXN0OiBCYWNrYm9uZVdvcmtlclJlcXVlc3QgPSB7IGtpbmQ6IFwic2VuZFwiLCBkb2N1bWVudElkLCBtZXNzYWdlOiBhY3Rvck1lc3NhZ2UgfTtcbiAgICB3b3JrZXIucG9zdE1lc3NhZ2UoeyB3aXJlOiBlbmNvZGVCYWNrYm9uZVdvcmtlclJlcXVlc3QocmVxdWVzdCkgfSk7XG4gIH0sIFtdKTtcblxuICB1c2VFZmZlY3QoKCkgPT4ge1xuICAgIGNvbnN0IHdvcmtlciA9IGJhY2tib25lV29ya2VyUmVmLmN1cnJlbnQ7XG4gICAgcmV0dXJuICgpID0+IHdvcmtlcj8udGVybWluYXRlKCk7XG4gIH0sIFtdKTtcblxuICB1c2VFZmZlY3QoKCkgPT4ge1xuICAgIHJldHVybiAoKSA9PiB7XG4gICAgICBmb3IgKGNvbnN0IHVucmVnaXN0ZXIgb2YgcGx1Z2luQmFja2JvbmVSb3V0ZVVucmVnaXN0ZXJzUmVmLmN1cnJlbnQudmFsdWVzKCkpIHVucmVnaXN0ZXIoKTtcbiAgICAgIHBsdWdpbkJhY2tib25lUm91dGVVbnJlZ2lzdGVyc1JlZi5jdXJyZW50LmNsZWFyKCk7XG4gICAgICBjb25zdCBwcmltYXJ5ID0gc2Vzc2lvblJlZi5jdXJyZW50O1xuICAgICAgaWYgKHByaW1hcnkpIHtcbiAgICAgICAgY29uc3QgcGx1Z2luID0gbG9hZGVkUGx1Z2luc1JlZi5jdXJyZW50LmZpbmQoKGVudHJ5KSA9PiBlbnRyeS5oYW5kbGUucGx1Z2luSWQgPT09IHByaW1hcnkucGx1Z2luSWQpPy5oYW5kbGU7XG4gICAgICAgIHZvaWQgcGx1Z2luPy5kZXN0cm95QXBwKHByaW1hcnkuaW5zdGFuY2VJZCkuY2F0Y2goKCkgPT4ge30pO1xuICAgICAgfVxuICAgICAgLy8g8J+qtu+4jyBDbG9zZXMgdGhlIHByZXZpb3VzbHktZG9jdW1lbnRlZCBXYXZlLTEgZ2FwOiBzdHVkaW8tbW9kZSBzcGF3bmVkIGFwcHMgKGBwYW5lbC5zcGF3bmVkQXBwc2ApXG4gICAgICAvLyBhbmQgZXh0ZXJuYWwtc2xvdCBjb250cmlidXRvciBpbnN0YW5jZXMgKGBjb250cmlidXRvckluc3RhbmNlc1JlZmApIGVhY2ggaG9sZCBhIGxpdmUgcGx1Z2luXG4gICAgICAvLyBpbnN0YW5jZSB0b28g4oCUIGxlYXZpbmcgdGhlbSBydW5uaW5nIHBhc3Qgc2hlbGwgdW5tb3VudCB3YXMgcHVyZSBsZWFrZWQgbWVtb3J5IChzZWVcbiAgICAgIC8vIFJFRFVDRS1ERU1PTlNUUkFUT1ItSURMRS1NRU1PUlktRk9PVFBSSU5UKS4gQmVzdC1lZmZvcnQ6IGFuIGluc3RhbmNlIHRoZSBndWVzdCBhbHJlYWR5IGRyb3BwZWQsXG4gICAgICAvLyBvciB3aG9zZSBwbHVnaW4gYWxyZWFkeSBkaXNwb3NlZCwganVzdCByZWplY3RzIGhhcm1sZXNzbHkgdmlhIHRoZSBzYW1lIGAuY2F0Y2goKCkgPT4ge30pYFxuICAgICAgLy8gcGF0dGVybiB0aGUgcHJpbWFyeSBzZXNzaW9uJ3Mgb3duIGRlc3Ryb3kgYWxyZWFkeSB1c2VkIGFib3ZlLlxuICAgICAgZm9yIChjb25zdCBzcGF3bmVkIG9mIHNwYXduZWRBcHBzUmVmLmN1cnJlbnQpIHtcbiAgICAgICAgY29uc3QgcGx1Z2luID0gbG9hZGVkUGx1Z2luc1JlZi5jdXJyZW50LmZpbmQoKGVudHJ5KSA9PiBlbnRyeS5oYW5kbGUucGx1Z2luSWQgPT09IHNwYXduZWQucGx1Z2luSWQpPy5oYW5kbGU7XG4gICAgICAgIHZvaWQgcGx1Z2luPy5kZXN0cm95QXBwKHNwYXduZWQuaW5zdGFuY2VJZCkuY2F0Y2goKCkgPT4ge30pO1xuICAgICAgfVxuICAgICAgZm9yIChjb25zdCBbcGx1Z2luSWQsIGluc3RhbmNlSWRdIG9mIGNvbnRyaWJ1dG9ySW5zdGFuY2VzUmVmLmN1cnJlbnQpIHtcbiAgICAgICAgY29uc3QgcGx1Z2luID0gbG9hZGVkUGx1Z2luc1JlZi5jdXJyZW50LmZpbmQoKGVudHJ5KSA9PiBlbnRyeS5oYW5kbGUucGx1Z2luSWQgPT09IHBsdWdpbklkKT8uaGFuZGxlO1xuICAgICAgICB2b2lkIHBsdWdpbj8uZGVzdHJveUFwcChpbnN0YW5jZUlkKS5jYXRjaCgoKSA9PiB7fSk7XG4gICAgICB9XG4gICAgICBjb250cmlidXRvckluc3RhbmNlc1JlZi5jdXJyZW50LmNsZWFyKCk7XG4gICAgICBmb3IgKGNvbnN0IGVudHJ5IG9mIGxvYWRlZFBsdWdpbnNSZWYuY3VycmVudCkgZW50cnkuaGFuZGxlLmRpc3Bvc2UoKTtcbiAgICB9O1xuICB9LCBbXSk7XG5cbiAgdXNlRWZmZWN0KCgpID0+IHtcbiAgICAvLyDwn5Ca77iPIE9ubHkgdGhlIHBhZ2Utb3duaW5nIHNoZWxsIG1heSB3cml0ZSB0aGUgYnJvd3NlciB0YWIgdGl0bGUg4oCUIGFuIGVtYmVkZGVkIHNoZWxsIChlLmcuIG9uZVxuICAgIC8vIGRlbW9uc3RyYXRvciBwYW5lKSBzaGFyaW5nIHRoZSBwYWdlIHdpdGggb3RoZXJzIG11c3Qgbm90IGZpZ2h0IHRoZW0gb3ZlciBpdC5cbiAgICBpZiAoIXNjb3BlLm93bnNQYWdlKSByZXR1cm47XG4gICAgaWYgKGJyYW5kKSB7XG4gICAgICBkb2N1bWVudC50aXRsZSA9IGJyYW5kLndpbmRvd1RpdGxlO1xuICAgIH0gZWxzZSBpZiAoYWN0aXZlQXBwVGl0bGUpIHtcbiAgICAgIGRvY3VtZW50LnRpdGxlID0gYWN0aXZlQXBwVGl0bGU7XG4gICAgfVxuICB9LCBbYWN0aXZlQXBwVGl0bGUsIGJyYW5kLCBzY29wZS5vd25zUGFnZV0pO1xuXG4gIC8vIPCflIzvuI8gQm9vdCBnYXRlcyBvbiB0aGUgcHJpbWFyeS9ob3N0IHBsdWdpbiBPTkxZIOKAlCBldmVyeSBvdGhlciByZWdpc3RyeSBlbnRyeSBzdHJlYW1zIGluIHZpYSB0aGVcbiAgLy8gc3Vic2NyaXB0aW9uIGVmZmVjdCBiZWxvdyBhcyBpdHMgYnVpbGQgbGFuZHMsIGluc3RlYWQgb2YgdGhlIHdob2xlIHNoZWxsIHdhaXRpbmcgb24gYWxsIH4zNyBjcmF0ZXNcbiAgLy8gKHNlZSBgYnVpbGRQbHVnaW5zU3RyZWFtaW5nYCBpbiB0aGUgZGV2IHJ1bm5lcikuIEEgcHJpbWFyeSB0aGF0IGZhaWxzIHRvIGxvYWQgKHRpbWVvdXQvZXJyb3IpIGlzXG4gIC8vIHN0aWxsIGZhdGFsLCBtaXJyb3JpbmcgdGhlIG9sZCBgbm9QbHVnaW5zTG9hZGVkYC9cImhvc3QgcHJvZ3JhbSBtaXNzaW5nIGxhbmRpbmcgYXBwXCIgYm9vdCBmYWlsdXJlcy5cbiAgdXNlRWZmZWN0KCgpID0+IHtcbiAgICBpZiAoIXByaW1hcnlQbHVnaW5JZCkgcmV0dXJuO1xuICAgIGlmIChsb2FkZWRQbHVnaW5zUmVmLmN1cnJlbnQuc29tZSgoZW50cnkpID0+IGVudHJ5LmhhbmRsZS5wbHVnaW5JZCA9PT0gcHJpbWFyeVBsdWdpbklkKSkgcmV0dXJuO1xuICAgIHZvaWQgKGFzeW5jICgpID0+IHtcbiAgICAgIGNvbnN0IG91dGNvbWUgPSBhd2FpdCBpbnN0YWxsUGx1Z2luKHByaW1hcnlQbHVnaW5JZCk7XG4gICAgICBpZiAob3V0Y29tZSA9PT0gXCJmYWlsZWRcIikge1xuICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0VSUk9SXCIsIHZhbHVlOiBzaGVsbExhYmVsKFwidWkuY29tbW9uLm5vUGx1Z2luc0xvYWRlZFwiKSB9KTtcbiAgICAgIH1cbiAgICB9KSgpO1xuICB9LCBbcHJpbWFyeVBsdWdpbklkLCBpbnN0YWxsUGx1Z2luXSk7XG5cbiAgLy8g8J+UjO+4jyBTdHJlYW1zIGV2ZXJ5IHJlZ2lzdHJ5IGVudHJ5IGluIGluZGVwZW5kZW50bHkgb2YgYm9vdDogb25lIGNvbm5lY3QtdGltZSBgc25hcHNob3RgICh3aGF0ZXZlcidzXG4gIC8vIGFscmVhZHkgYnVpbHQsIGluY2x1ZGluZyBhIGRldiBzZXJ2ZXIgdGhhdCB3YXMgYWxyZWFkeSBmdWxseSBidWlsdCBiZWZvcmUgdGhpcyBzaGVsbCBtb3VudGVkKSBwbHVzXG4gIC8vIGEgYGJ1aWx0YCBldmVudCBwZXIgY3JhdGUgYXMgYGJ1aWxkUGx1Z2luc1N0cmVhbWluZ2AvdGhlIGZvbGRlZC1pbiB3YXRjaCBsb29wIGZpbmlzaGVzIGl0LiBBbiBldmVudFxuICAvLyBmb3IgYW4gYWxyZWFkeS1sb2FkZWQgcGx1Z2luIHJvdXRlcyB0byBgcmVsb2FkUGx1Z2luYCAoaG90LXN3YXApIGluc3RlYWQgb2YgYGluc3RhbGxQbHVnaW5gLlxuICB1c2VFZmZlY3QoKCkgPT4ge1xuICAgIGNvbnN0IHJlZ2lzdHJ5SWRzID0gbmV3IFNldChyZWdpc3RyeS5tYXAoKGVudHJ5KSA9PiBlbnRyeS5wbHVnaW5JZCkpO1xuICAgIGNvbnN0IGhhbmRsZVBsdWdpbkF2YWlsYWJsZSA9IChwbHVnaW5JZDogc3RyaW5nLCByZWJ1aWx0QXQ6IG51bWJlcikgPT4ge1xuICAgICAgaWYgKCFyZWdpc3RyeUlkcy5oYXMocGx1Z2luSWQpKSByZXR1cm47XG4gICAgICBjb25zdCBhbHJlYWR5TG9hZGVkID0gbG9hZGVkUGx1Z2luc1JlZi5jdXJyZW50LnNvbWUoKGVudHJ5KSA9PiBlbnRyeS5oYW5kbGUucGx1Z2luSWQgPT09IHBsdWdpbklkKTtcbiAgICAgIHZvaWQgKGFscmVhZHlMb2FkZWQgPyByZWxvYWRQbHVnaW4ocGx1Z2luSWQsIHJlYnVpbHRBdCkgOiBpbnN0YWxsUGx1Z2luKHBsdWdpbklkLCByZWJ1aWx0QXQpKTtcbiAgICB9O1xuICAgIHJldHVybiBwbHVnaW5Tb3VyY2Uuc3Vic2NyaWJlKChldmVudDogUGx1Z2luU291cmNlRXZlbnQpID0+IHtcbiAgICAgIGlmIChldmVudC5raW5kID09PSBcInNuYXBzaG90XCIpIHtcbiAgICAgICAgZm9yIChjb25zdCBwbHVnaW4gb2YgZXZlbnQucGx1Z2lucykgaGFuZGxlUGx1Z2luQXZhaWxhYmxlKHBsdWdpbi5wbHVnaW5JZCwgcGx1Z2luLnJlYnVpbHRBdCk7XG4gICAgICAgIHJldHVybjtcbiAgICAgIH1cbiAgICAgIGhhbmRsZVBsdWdpbkF2YWlsYWJsZShldmVudC5wbHVnaW5JZCwgZXZlbnQucmVidWlsdEF0KTtcbiAgICB9KTtcbiAgfSwgW3JlZ2lzdHJ5LCBwbHVnaW5Tb3VyY2UsIGluc3RhbGxQbHVnaW4sIHJlbG9hZFBsdWdpbl0pO1xuXG4gIGNvbnN0IGZpbmRQbHVnaW5Gb3JBY3Rpb24gPSB1c2VDYWxsYmFjayhcbiAgICAoYWN0aW9uOiBBY3Rpb25EZXNjcmlwdG9yKSA9PiB7XG4gICAgICBjb25zdCBieUNvbnRyb2xsZXIgPSBsb2FkZWRQbHVnaW5zLmZpbmQoKGVudHJ5KSA9PiBlbnRyeS5tYW5pZmVzdC5hcHBzLnNvbWUoKGFwcCkgPT4gYXBwLmNvbnRyb2xsZXJJZCA9PT0gYWN0aW9uLmNvbnRyb2xsZXJJZCkpO1xuICAgICAgaWYgKGJ5Q29udHJvbGxlcikgcmV0dXJuIGJ5Q29udHJvbGxlcjtcbiAgICAgIHJldHVybiBsb2FkZWRQbHVnaW5zLmZpbmQoKGVudHJ5KSA9PiBlbnRyeS5oYW5kbGUucGx1Z2luSWQgPT09IHNlc3Npb24/LnBsdWdpbklkKTtcbiAgICB9LFxuICAgIFtsb2FkZWRQbHVnaW5zLCBzZXNzaW9uPy5wbHVnaW5JZF0sXG4gICk7XG5cbiAgY29uc3QgcmVxdWVzdENvbnRleHRNZW51ID0gdXNlQ2FsbGJhY2soXG4gICAgYXN5bmMgKHJlcXVlc3Q6IFBsdWdpbkNvbnRleHRNZW51UmVxdWVzdCk6IFByb21pc2U8cmVhZG9ubHkgQ29udGV4dE1lbnVJdGVtU3BlY1tdPiA9PiB7XG4gICAgICBpZiAoIXNlc3Npb24pIHJldHVybiBbXTtcbiAgICAgIGNvbnN0IHBsdWdpbiA9IGxvYWRlZFBsdWdpbnMuZmluZCgoZW50cnkpID0+IGVudHJ5LmhhbmRsZS5wbHVnaW5JZCA9PT0gc2Vzc2lvbi5wbHVnaW5JZCk/LmhhbmRsZTtcbiAgICAgIGlmICghcGx1Z2luPy5jb250ZXh0TWVudSkgcmV0dXJuIFtdO1xuICAgICAgLy8g8J+Wse+4jyBObyB2aWV3IHN0YXRlIG9uIHRoZSB3aXJlIOKAlCB0aGUgU0RLJ3MgQ29udGV4dE1lbnVXaXJlUmVxdWVzdCBkcm9wcGVkIGl0ICh0aGUgcGx1Z2luJ3NcbiAgICAgIC8vIG93biBwZXJzaXN0ZWQgc2VsZWN0aW9uL2hvdmVyIHN0YXRlIGFscmVhZHkgYW5zd2VycyBcIndoYXQncyBzZWxlY3RlZFwiLCBzZWUgQXBwQWN0aW9uUmVnaXN0cnlcbiAgICAgIC8vIGZ1bm5lbCk7IHNlbmRpbmcgb25lIGhlcmUgd291bGQganVzdCBiZSBzaWxlbnRseSBkaXNjYXJkZWQgb24gdGhlIFJ1c3Qgc2lkZS5cbiAgICAgIHJldHVybiBwbHVnaW4uY29udGV4dE1lbnUoc2Vzc2lvbi5pbnN0YW5jZUlkLCByZXF1ZXN0KTtcbiAgICB9LFxuICAgIFtsb2FkZWRQbHVnaW5zLCBzZXNzaW9uXSxcbiAgKTtcblxuICBjb25zdCByZWZyZXNoVWkgPSB1c2VDYWxsYmFjayhcbiAgICAvLyDwn6qf77iPIGBleHRyYUluc3RhbmNlc092ZXJyaWRlYCBsZXRzIGEgY2FsbGVyIHRoYXQganVzdCBzeW5jaHJvbm91c2x5IGNvbXB1dGVkIGEgTkVXIGV4dHJhLXdpbmRvdyBsaXN0XG4gICAgLy8gKHNwbGl0L2Ryb3AsIGxheW91dC9tb2RlIHN3aXRjaCkgaGFuZCBpdCBzdHJhaWdodCB0byB0aGlzIGZldGNoIGluc3RlYWQgb2YgcmVhZGluZyBgZXh0cmFXaW5kb3dJbnN0YW5jZXNgXG4gICAgLy8gZnJvbSBSZWFjdCBzdGF0ZSwgd2hpY2ggd291bGRuJ3QgcmVmbGVjdCB0aGUganVzdC1kaXNwYXRjaGVkIGNoYW5nZSB1bnRpbCB0aGUgbmV4dCByZW5kZXIuXG4gICAgYXN5bmMgKG5leHRTZXNzaW9uOiBBY3RpdmVTZXNzaW9uLCBzY29wZUFyZzogVWlEaXJ0eVNjb3BlID0geyBraW5kOiBcImZ1bGxcIiB9LCBleHRyYUluc3RhbmNlc092ZXJyaWRlPzogcmVhZG9ubHkgRXh0cmFXaW5kb3dJbnN0YW5jZVtdKSA9PiB7XG4gICAgICBpZiAoc2NvcGVBcmcua2luZCA9PT0gXCJub25lXCIpIHJldHVybjtcbiAgICAgIGNvbnN0IGdlbmVyYXRpb24gPSArK3JlZnJlc2hHZW5lcmF0aW9uUmVmLmN1cnJlbnQ7XG4gICAgICBjb25zdCBwcm9ncmFtID0gbG9hZGVkUGx1Z2lucy5maW5kKChlbnRyeSkgPT4gZW50cnkuaGFuZGxlLnBsdWdpbklkID09PSBuZXh0U2Vzc2lvbi5wbHVnaW5JZCk/LmhhbmRsZTtcbiAgICAgIGlmICghcHJvZ3JhbSkgcmV0dXJuO1xuICAgICAgY29uc3QgbGF5b3V0U2VlZEtleSA9IGAke25leHRTZXNzaW9uLnBsdWdpbklkfToke25leHRTZXNzaW9uLmFwcC5pZH06JHtuZXh0U2Vzc2lvbi5pbnN0YW5jZUlkfWA7XG4gICAgICBjb25zdCBpc1Nlc3Npb25Td2l0Y2ggPSBsYXlvdXRTZWVkS2V5UmVmLmN1cnJlbnQgIT09IGxheW91dFNlZWRLZXk7XG4gICAgICAvLyDwn5Ci77iPIEEgc2Vzc2lvbiBzd2l0Y2ggaW52YWxpZGF0ZXMgZXZlcnkgY2FjaGVkIGhhc2ggZnJvbSB0aGUgcHJldmlvdXMgaW5zdGFuY2Ug4oCUIGZvcmNlIGEgZnVsbFxuICAgICAgLy8gZmV0Y2ggcmVnYXJkbGVzcyBvZiB3aGF0IHNjb3BlIHRoaXMgcGFydGljdWxhciBjYWxsIHdhcyBnaXZlbi5cbiAgICAgIGxldCBzY29wZSA9IHNjb3BlQXJnO1xuICAgICAgaWYgKGlzU2Vzc2lvblN3aXRjaCkge1xuICAgICAgICB1aVJlZnJlc2hDYWNoZVJlZi5jdXJyZW50ID0gbmV3IE1hcCgpO1xuICAgICAgICBzY29wZSA9IHsga2luZDogXCJmdWxsXCIgfTtcbiAgICAgIH1cbiAgICAgIGNvbnN0IGNhY2hlID0gdWlSZWZyZXNoQ2FjaGVSZWYuY3VycmVudDtcbiAgICAgIC8vIPCfqp/vuI8gT24gYSBzZXNzaW9uIHN3aXRjaCwgc2VlZCB0aGUgZGVmYXVsdCBsYXlvdXQncyBleHRyYSBpbnN0YW5jZXMgQkVGT1JFIGZldGNoaW5nIChub3QgYWZ0ZXIpLCBzb1xuICAgICAgLy8gdGhpcyB2ZXJ5IGZpcnN0IGZldGNoIGFscmVhZHkgcmVxdWVzdHMgZXZlcnkgZGVmYXVsdC1sYXlvdXQgcGFuZSdzIGJvZHkvbWVhc3VyZXMvZW5nYWdlbWVudHNcbiAgICAgIC8vIGluc3RlYWQgb2YgbGVhdmluZyBuZXdseS1zZWVkZWQgcGFuZXMgdG8gc2hvdyBcIm1pc3Npbmcgd2luZG93XCIgdW50aWwgc29tZSBsYXRlciwgdW5yZWxhdGVkIHJlZnJlc2guXG4gICAgICBjb25zdCBsYXlvdXRTZWVkID0gaXNTZXNzaW9uU3dpdGNoID8gYXBwbHlGcmFtZXdvcmtMYXlvdXRTZWVkKG5leHRTZXNzaW9uLmFwcC5kZWZhdWx0TGF5b3V0LCBuZXh0U2Vzc2lvbi5hcHAud2luZG93S2luZHMsIGFwcExhYmVsc092ZXJsYXksIHVpVGVybWlub2xvZ3ksIHVpTG9jYWxlKSA6IHVuZGVmaW5lZDtcbiAgICAgIC8vIPCfqp/vuI8gUHJlZmVyIHRoZSBvdmVycmlkZSwgdGhlbiB0aGUganVzdC1jb21wdXRlZCBzZXNzaW9uLXN3aXRjaCBzZWVkLCB0aGVuIHRoZSBsaXZlIHJlZiAobmV2ZXIgdGhlXG4gICAgICAvLyByZW5kZXItY2xvc3VyZSBzbmFwc2hvdCkgc28gYSBjb25jdXJyZW50IHJlZnJlc2ggY2Fubm90IGRyb3AgZGVmYXVsdC1sYXlvdXQgcGFuZXMuXG4gICAgICBjb25zdCBleHRyYUluc3RhbmNlc0ZvckZldGNoID0gZXh0cmFJbnN0YW5jZXNPdmVycmlkZSA/PyBsYXlvdXRTZWVkPy5leHRyYUluc3RhbmNlcyA/PyBleHRyYVdpbmRvd0luc3RhbmNlc1JlZi5jdXJyZW50O1xuICAgICAgY29uc3Qgd2luZG93SW5zdGFuY2VzID0gc2Vzc2lvbldpbmRvd0luc3RhbmNlcyhuZXh0U2Vzc2lvbi5hcHAsIGV4dHJhSW5zdGFuY2VzRm9yRmV0Y2gpO1xuICAgICAgY29uc3QgY29udHJpYnV0aW9uc0pzb24gPSBidWlsZENvbnRyaWJ1dGlvbnNKc29uKGxvYWRlZFBsdWdpbnMubWFwKChlbnRyeSkgPT4gKHsgcGx1Z2luSWQ6IGVudHJ5LmhhbmRsZS5wbHVnaW5JZCwgbWFuaWZlc3Q6IGVudHJ5Lm1hbmlmZXN0IH0pKSk7XG4gICAgICAvLyDwn6qQ77iPIEV2ZXJ5IGxvYWRlZCBwbHVnaW4ncyBkZWNsYXJlZCBhcHBzLCBmbGF0dGVuZWQgZm9yIHRoZSBzcGFjZSBhcHAncyBjYXRhbG9ndWUg4oCUIG1pcnJvcnNcbiAgICAgIC8vIGBjb250cmlidXRpb25zSnNvbmAgYWJvdmUgZXhhY3RseSAoc2FtZSBvcHQtaW4gaGludC1wdXNoIHNoYXBlIGJlbG93KSwgYmVjYXVzZSB0aGUgc3BhY2UgYXBwIGlzXG4gICAgICAvLyBpdHMgb3duIHdhc20gY29tcG9uZW50OiBgc2VtaW9fZnJhbWV3b3JrX29zOjpBUFBfUkVHSVNUUkFUSU9OU2AgKHBvcHVsYXRlZCBhdCBuYXRpdmUvdGVzdFxuICAgICAgLy8gYFBsdWdpbkhvc3Q6OmxvYWRfcGx1Z2luYC9gaG90X3N3YXBfcGx1Z2luYCB0aW1lKSBsaXZlcyBpbiBhIHNlcGFyYXRlIGxpbmVhciBtZW1vcnkgZnJvbSB0aGVcbiAgICAgIC8vIHNwYWNlIGFwcCdzIG93biBzdGF0aWNhbGx5LWxpbmtlZCBjb3B5IG9mIHRoZSBzYW1lIG9zLWNvcmUgY3JhdGUsIHNvIG5vdGhpbmcgY3Jvc3NlcyB0aGUgd2FzbVxuICAgICAgLy8gYm91bmRhcnkgdW5sZXNzIHRoaXMgc2hlbGwgcHVzaGVzIGl0IGV4cGxpY2l0bHkuXG4gICAgICBjb25zdCBhcHBSZWdpc3RyYXRpb25zSnNvbiA9IEpTT04uc3RyaW5naWZ5KGxvYWRlZFBsdWdpbnMuZmxhdE1hcCgoZW50cnkpID0+IChlbnRyeS5tYW5pZmVzdC5hcHBzID8/IFtdKS5tYXAoKGFwcCkgPT4gKHsgcGx1Z2luSWQ6IGVudHJ5LmhhbmRsZS5wbHVnaW5JZCwgYXBwIH0pKSkpO1xuICAgICAgY29uc3Qgdmlld1N0YXRlOiBWaWV3TW9kZWwgPSBpbmplY3RBY3RpdmVUb29sKHtcbiAgICAgICAgLi4ubmV4dFNlc3Npb24udmlld1N0YXRlLFxuICAgICAgICBjb250cmlidXRpb25zSnNvbixcbiAgICAgICAgbG9jYWxlOiB1aUxvY2FsZSxcbiAgICAgICAgdGVybWlub2xvZ3k6IHVpVGVybWlub2xvZ3ksXG4gICAgICAgIHdpbmRvd0luc3RhbmNlczogd2luZG93SW5zdGFuY2VzLm1hcCgoaW5zdGFuY2UpID0+ICh7IGlkOiBpbnN0YW5jZS5pZCwgd2luZG93S2luZElkOiBpbnN0YW5jZS53aW5kb3dLaW5kSWQgfSkpLFxuICAgICAgICBhY3RpdmVVdGlsaXR5QnlXaW5kb3dJZDogYnVpbGRBY3RpdmVVdGlsaXR5QnlXaW5kb3dJZChhY3RpdmVVdGlsaXR5QnlXaW5kb3dJZFJlZi5jdXJyZW50KSxcbiAgICAgICAgYWN0aXZlVXRpbGl0eUlkOiB1bmRlZmluZWQsXG4gICAgICB9KTtcbiAgICAgIGNvbnN0IHBhbmVsVGFiTGVhdmVzID0gZmxhdHRlblBhbmVsVGFiTGVhdmVzKG5leHRTZXNzaW9uLmFwcC5wYW5lbFRhYnMpO1xuICAgICAgLy8g8J+Qou+4jyBPbmUgYmF0Y2hlZCwgaGFzaC1jb25kaXRpb25hbCByb3VuZCB0cmlwIHJlcGxhY2VzIHRoZSBvbGQgfjEyIHNlcXVlbnRpYWxcbiAgICAgIC8vIHJlbmRlci91dGlsaXRpZXMvd2luZG93RW5nYWdlbWVudHMvd2luZG93TWVhc3VyZXMvYXBwTGFiZWxzIGNhbGxzIOKAlCB0aGUgcGx1Z2luIG9taXRzIHBheWxvYWRzIGZvclxuICAgICAgLy8gYW55IHNlY3Rpb24gd2hvc2UgaGFzaCBzdGlsbCBtYXRjaGVzIHdoYXQgYGNhY2hlYCBhbHJlYWR5IGhvbGRzLlxuICAgICAgY29uc3QgcmVxdWVzdCA9IGJ1aWxkVWlSZWZyZXNoUmVxdWVzdChzY29wZSwgd2luZG93SW5zdGFuY2VzLCBwYW5lbFRhYkxlYXZlcywgdmlld1N0YXRlLCBjYWNoZSk7XG4gICAgICBpZiAocmVxdWVzdCkge1xuICAgICAgICBjb25zdCByZXNwb25zZSA9IGF3YWl0IHByb2dyYW0ucmVmcmVzaFVpKG5leHRTZXNzaW9uLmluc3RhbmNlSWQsIHJlcXVlc3QpO1xuICAgICAgICBpZiAoZ2VuZXJhdGlvbiAhPT0gcmVmcmVzaEdlbmVyYXRpb25SZWYuY3VycmVudCkgcmV0dXJuO1xuICAgICAgICBjb25zdCBzbG90Q29udGV4dCA9IHtcbiAgICAgICAgICBwbHVnaW5zOiBuZXcgTWFwKGxvYWRlZFBsdWdpbnMubWFwKChlbnRyeSkgPT4gW2VudHJ5LmhhbmRsZS5wbHVnaW5JZCwgZW50cnkuaGFuZGxlXSkpLFxuICAgICAgICAgIGNvbnRyaWJ1dG9ySW5zdGFuY2VzOiBjb250cmlidXRvckluc3RhbmNlc1JlZi5jdXJyZW50LFxuICAgICAgICAgIHZpZXdTdGF0ZSxcbiAgICAgICAgfTtcbiAgICAgICAgLy8gUmVzb2x2ZSBleHRlcm5hbCBzbG90cyBvbiBmcmVzaGx5LWNoYW5nZWQgd2luZG93L3BhbmVsIGJvZGllcyBvbmx5LCBiZWZvcmUgY2FjaGluZyB0aGVtLCBzbyBhXG4gICAgICAgIC8vIGxhdGVyIG5vLW9wZXJhdGlvbiByZWZyZXNoIHJldXNlcyB0aGUgYWxyZWFkeS1yZXNvbHZlZCBjYWNoZWQgdmFsdWUgaW5zdGVhZCBvZiByZS1yZXNvbHZpbmcuXG4gICAgICAgIGNvbnN0IHJlc29sdmVJZkNoYW5nZWQgPSBhc3luYyAoZW50cnk6IFBsdWdpblVpUmVmcmVzaFNlY3Rpb25SZXNwb25zZSk6IFByb21pc2U8UGx1Z2luVWlSZWZyZXNoU2VjdGlvblJlc3BvbnNlPiA9PiAoZW50cnkudmFsdWUgIT09IHVuZGVmaW5lZCA/IHsgLi4uZW50cnksIHZhbHVlOiBhd2FpdCByZXNvbHZlRXh0ZXJuYWxTbG90cyhlbnRyeS52YWx1ZSBhcyBVaU5vZGUsIHNsb3RDb250ZXh0KSB9IDogZW50cnkpO1xuICAgICAgICBjb25zdCBbcmVzb2x2ZWRXaW5kb3dzLCByZXNvbHZlZFBhbmVsc10gPSBhd2FpdCBQcm9taXNlLmFsbChbUHJvbWlzZS5hbGwoKHJlc3BvbnNlLndpbmRvd3MgPz8gW10pLm1hcChyZXNvbHZlSWZDaGFuZ2VkKSksIFByb21pc2UuYWxsKChyZXNwb25zZS5wYW5lbHMgPz8gW10pLm1hcChyZXNvbHZlSWZDaGFuZ2VkKSldKTtcbiAgICAgICAgaWYgKGdlbmVyYXRpb24gIT09IHJlZnJlc2hHZW5lcmF0aW9uUmVmLmN1cnJlbnQpIHJldHVybjtcbiAgICAgICAgYXBwbHlVaVJlZnJlc2hSZXNwb25zZVRvQ2FjaGUoY2FjaGUsIHsgLi4ucmVzcG9uc2UsIHdpbmRvd3M6IHJlc29sdmVkV2luZG93cywgcGFuZWxzOiByZXNvbHZlZFBhbmVscyB9KTtcbiAgICAgICAgLy8g4o+x77iPIFNlZSBgRG9jdW1lbnRBcHA6OnBlbmRpbmdfZWZmZWN0c2Ag4oCUIGUuZy4gcmVzdW1pbmcgYSBgZmxvd0V2YWxUaWNrYCBjaGFpbiBhZnRlciB0aGlzIHJlZnJlc2guXG4gICAgICAgIGlmIChyZXNwb25zZS5yZXF1ZXN0ZWRFZmZlY3RzPy5sZW5ndGgpIGF3YWl0IGFwcGx5SG9zdEVmZmVjdHMocmVzcG9uc2UucmVxdWVzdGVkRWZmZWN0cywgbmV4dFNlc3Npb24pO1xuICAgICAgfVxuICAgICAgLy8g8J+OryBCb3RoIHB1c2ggZ3VhcmRzIGJlbG93IGFyZSBrZXllZCBvbiBgJHtuZXh0U2Vzc2lvbi5pbnN0YW5jZUlkfTo6JHtqc29ufWAsIE5PVCBvbiB0aGUganNvblxuICAgICAgLy8gY29udGVudCBhbG9uZSDigJQgdGhlIGNvbnRlbnQgaXMgZGVyaXZlZCBwdXJlbHkgZnJvbSBgbG9hZGVkUGx1Z2luc2AsIHdoaWNoIHN0YWJpbGl6ZXMgcmlnaHQgYWZ0ZXJcbiAgICAgIC8vIGJvb3QsIHNvIGEgY29udGVudC1vbmx5IGtleSB3b3VsZCBvbmx5IGV2ZXIgdW5sb2NrIE9ORSBwdXNoIGZvciB0aGUgcHJvY2VzcyBsaWZldGltZSAodGhlIHZlcnlcbiAgICAgIC8vIGZpcnN0IGByZWZyZXNoVWlgIGNhbGwsIHdoaWNoIGFsd2F5cyB0YXJnZXRzIHdoYXRldmVyIHNlc3Npb24gZXhpc3RzIGF0IGJvb3Qg4oCUIHVzdWFsbHkgYGhvbWVgLFxuICAgICAgLy8gd2hpY2ggZG9lc24ndCBpbXBsZW1lbnQgZWl0aGVyIGFjdGlvbiBhbmQgcmVqZWN0cyBpdCkuIEZvbGRpbmcgYGluc3RhbmNlSWRgIGludG8gdGhlIGtleSBtYWtlcyBhXG4gICAgICAvLyBzZXNzaW9uIHN3aXRjaCAobmV3IHN0dWRpby9zcGFjZSBpbnN0YW5jZSBvcGVuZWQsIHNhbWUgdW5jaGFuZ2VkIGpzb24pIHJldHJpZ2dlciB0aGUgcHVzaCBpbnN0ZWFkXG4gICAgICAvLyBvZiBiZWluZyBzaWxlbnRseSBzd2FsbG93ZWQgYnkgYSBndWFyZCB0aGF0IGFscmVhZHkgY29uc2lkZXJlZCB0aGlzIGNvbnRlbnQgXCJkZWxpdmVyZWRcIi5cbiAgICAgIGlmIChjb250cmlidXRpb25zSnNvbikge1xuICAgICAgICBjb25zdCBjb250cmlidXRpb25zUHVzaEtleSA9IGAke25leHRTZXNzaW9uLmluc3RhbmNlSWR9Ojoke2NvbnRyaWJ1dGlvbnNKc29ufWA7XG4gICAgICAgIGlmIChjb250cmlidXRpb25zUHVzaEtleSAhPT0gY29udHJpYnV0aW9uc0pzb25SZWYuY3VycmVudCkge1xuICAgICAgICAgIGNvbnRyaWJ1dGlvbnNKc29uUmVmLmN1cnJlbnQgPSBjb250cmlidXRpb25zUHVzaEtleTtcbiAgICAgICAgICBjb25zdCBwbHVnaW5FbnRyeSA9IGxvYWRlZFBsdWdpbnMuZmluZCgoZW50cnkpID0+IGVudHJ5LmhhbmRsZS5wbHVnaW5JZCA9PT0gbmV4dFNlc3Npb24ucGx1Z2luSWQpO1xuICAgICAgICAgIC8vIPCfm6HvuI8gYHNldENvbnRyaWJ1dGlvbnNgIGlzIGFuIG9wdC1pbiBoaW50IHB1c2gg4oCUIG9ubHkgYHByb2NlZHVyYWwzZGAncyBgUHJvY2VkdXJhbDNkQ29tbWFuZDo6U2V0Q29udHJpYnV0aW9uc2BcbiAgICAgICAgICAvLyAoZmxvdy5leHRlbnNpb24gaG90LXN3YXApIGFuZCBgZm9ybXNgJ3MgYEZvcm1zQ29tbWFuZDo6U2V0Q29udHJpYnV0aW9uc2AgKHBsYXlib29rIGJsb2NrLWtpbmQgY2F0YWxvZ3VlKVxuICAgICAgICAgIC8vIGFjdHVhbGx5IGltcGxlbWVudCBpdDsgaXQgaXMgZGVsaWJlcmF0ZWx5IE5PVCBkZWNsYXJlZCBpbiBlaXRoZXIgYXBwJ3MgYWN0aW9uIGNhdGFsb2cgKHNhbWVcbiAgICAgICAgICAvLyB1bmNhdGFsb2d1ZWQtYnJpZGdlIHNoYXBlIGFzIGBzZXRMb2NhbGVgKSwgc28gY2F0YWxvZyBtZW1iZXJzaGlwIGNhbid0IGdhdGUgdGhpcyBjYWxsLiBFdmVyeSBvdGhlclxuICAgICAgICAgIC8vIGFwcCdzIGBEb2N1bWVudEFwcDo6Y29tbWFuZF9mcm9tX2FjdGlvbmAgZGVmYXVsdCByZWplY3RzIHVua25vd24gaWRzIOKAlCBzd2FsbG93IHRoYXQgcmVqZWN0aW9uIGhlcmVcbiAgICAgICAgICAvLyByYXRoZXIgdGhhbiBnYXRpbmcgYnkgYXBwIGlkLCBzbyB0aGlzIHN0YXlzIGNvcnJlY3QgaWYgYSBmdXR1cmUgYXBwIGFkZHMgaXRzIG93biBgU2V0Q29udHJpYnV0aW9uc2BcbiAgICAgICAgICAvLyB2YXJpYW50IHdpdGhvdXQgdGhpcyBjYWxsIHNpdGUgbmVlZGluZyB0byBrbm93IGFib3V0IGl0LlxuICAgICAgICAgIC8vIPCfp7XvuI8gQjE6IE1VU1QgZ28gdGhyb3VnaCBgaGFuZGxlQWN0aW9uYCAoa2luZDpcImFjdGlvblwiIOKGkiBgZGlzcGF0Y2hfYWN0aW9uYCDihpIgYGNvbW1hbmRfZnJvbV9hY3Rpb25gXG4gICAgICAgICAgLy8g4oaSIGBkaXNwYXRjaF90eXBlZF9jb21tYW5kX2lubmVyYCkg4oCUIGBoYW5kbGVDb21tYW5kYCAoa2luZDpcImNvbW1hbmRcIikgYWx3YXlzIGhhcmQtZXJyb3JzIG5vdywgc2VlXG4gICAgICAgICAgLy8gYFZjc0RvY3VtZW50QXBwOjpkaXNwYXRjaF9jb21tYW5kYCdzIGRvYzsgdGhlcmUgYXJlIG5vIGZyYW1ld29yay1yZXNlcnZlZCBDT01NQU5EUywgb25seSBhY3Rpb25zLlxuICAgICAgICAgIGlmIChwbHVnaW5FbnRyeSkge1xuICAgICAgICAgICAgdHJ5IHtcbiAgICAgICAgICAgICAgY29uc3Qgd2lyZSA9IGVuY29kZUFjdGlvbldpcmUoeyBjb250cm9sbGVySWQ6IG5leHRTZXNzaW9uLmFwcC5jb250cm9sbGVySWQsIGFjdGlvbjogXCJzZXRDb250cmlidXRpb25zXCIsIGFyZ3M6IHsganNvbjogY29udHJpYnV0aW9uc0pzb24gfSB9KTtcbiAgICAgICAgICAgICAgYXdhaXQgcGx1Z2luRW50cnkuaGFuZGxlLmhhbmRsZUFjdGlvbihuZXh0U2Vzc2lvbi5pbnN0YW5jZUlkLCB3aXJlLCBuZXh0U2Vzc2lvbi52aWV3U3RhdGUpO1xuICAgICAgICAgICAgfSBjYXRjaCAoZXJyb3IpIHtcbiAgICAgICAgICAgICAgY29uc29sZS53YXJuKFwiW0RFQlVHXSBzZXRDb250cmlidXRpb25zIHB1c2ggc2tpcHBlZFwiLCBlcnJvciBpbnN0YW5jZW9mIEVycm9yID8gZXJyb3IubWVzc2FnZSA6IFN0cmluZyhlcnJvcikpO1xuICAgICAgICAgICAgfVxuICAgICAgICAgIH1cbiAgICAgICAgfVxuICAgICAgfVxuICAgICAgaWYgKGFwcFJlZ2lzdHJhdGlvbnNKc29uKSB7XG4gICAgICAgIGNvbnN0IGFwcFJlZ2lzdHJhdGlvbnNQdXNoS2V5ID0gYCR7bmV4dFNlc3Npb24uaW5zdGFuY2VJZH06OiR7YXBwUmVnaXN0cmF0aW9uc0pzb259YDtcbiAgICAgICAgaWYgKGFwcFJlZ2lzdHJhdGlvbnNQdXNoS2V5ICE9PSBhcHBSZWdpc3RyYXRpb25zSnNvblJlZi5jdXJyZW50KSB7XG4gICAgICAgICAgYXBwUmVnaXN0cmF0aW9uc0pzb25SZWYuY3VycmVudCA9IGFwcFJlZ2lzdHJhdGlvbnNQdXNoS2V5O1xuICAgICAgICAgIGNvbnN0IHBsdWdpbkVudHJ5ID0gbG9hZGVkUGx1Z2lucy5maW5kKChlbnRyeSkgPT4gZW50cnkuaGFuZGxlLnBsdWdpbklkID09PSBuZXh0U2Vzc2lvbi5wbHVnaW5JZCk7XG4gICAgICAgICAgLy8g8J+qkO+4jyBgc2V0QXBwUmVnaXN0cmF0aW9uc2AgbWlycm9ycyBgc2V0Q29udHJpYnV0aW9uc2AgaW1tZWRpYXRlbHkgYWJvdmUgZXhhY3RseTogYW4gb3B0LWluIGhpbnRcbiAgICAgICAgICAvLyBwdXNoLCBjdXJyZW50bHkgb25seSBpbXBsZW1lbnRlZCBieSB0aGUgc3BhY2UgYXBwJ3MgYFNwYWNlQ29tbWFuZDo6U2V0QXBwUmVnaXN0cmF0aW9uc2BcbiAgICAgICAgICAvLyAocG9wdWxhdGVzIGl0cyBvd24gbGlua2VkLWluIGNvcHkgb2YgYHNlbWlvX2ZyYW1ld29ya19vczo6QVBQX1JFR0lTVFJBVElPTlNgIHNvXG4gICAgICAgICAgLy8gYHdvcmtmbG93X3BhbGV0dGUoKWAvYGJ1aWxkX2NhdGFsb2d1ZV90cmVlYCBjYW4gbGlzdCBldmVyeSBsb2FkZWQgYXBwKS4gTm90IGRlY2xhcmVkIGluIGFueVxuICAgICAgICAgIC8vIGFwcCdzIGFjdGlvbiBjYXRhbG9nLCBzbyDigJQgc2FtZSBhcyBgc2V0Q29udHJpYnV0aW9uc2Ag4oCUIGdhdGUgYnkgc3dhbGxvd2luZyB0aGUgcmVqZWN0aW9uIGV2ZXJ5XG4gICAgICAgICAgLy8gb3RoZXIgYXBwJ3MgYERvY3VtZW50QXBwOjpjb21tYW5kX2Zyb21fYWN0aW9uYCBkZWZhdWx0IHRocm93cyBmb3IgYW4gdW5rbm93biBpZCwgcmF0aGVyIHRoYW4gYnlcbiAgICAgICAgICAvLyBhcHAgaWQsIHNvIHRoaXMgc3RheXMgY29ycmVjdCBpZiBhIGZ1dHVyZSBhcHAgYWRkcyBpdHMgb3duIGBTZXRBcHBSZWdpc3RyYXRpb25zYCB2YXJpYW50XG4gICAgICAgICAgLy8gd2l0aG91dCB0aGlzIGNhbGwgc2l0ZSBuZWVkaW5nIHRvIGtub3cgYWJvdXQgaXQuXG4gICAgICAgICAgLy8g8J+nte+4jyBCMTogTVVTVCBnbyB0aHJvdWdoIGBoYW5kbGVBY3Rpb25gIChraW5kOlwiYWN0aW9uXCIg4oaSIGBkaXNwYXRjaF9hY3Rpb25gIOKGkiBgY29tbWFuZF9mcm9tX2FjdGlvbmBcbiAgICAgICAgICAvLyDihpIgYGRpc3BhdGNoX3R5cGVkX2NvbW1hbmRfaW5uZXJgKSDigJQgYGhhbmRsZUNvbW1hbmRgIChraW5kOlwiY29tbWFuZFwiKSBhbHdheXMgaGFyZC1lcnJvcnMgbm93LCBzZWVcbiAgICAgICAgICAvLyBgVmNzRG9jdW1lbnRBcHA6OmRpc3BhdGNoX2NvbW1hbmRgJ3MgZG9jOyB0aGVyZSBhcmUgbm8gZnJhbWV3b3JrLXJlc2VydmVkIENPTU1BTkRTLCBvbmx5IGFjdGlvbnMuXG4gICAgICAgICAgaWYgKHBsdWdpbkVudHJ5KSB7XG4gICAgICAgICAgICB0cnkge1xuICAgICAgICAgICAgICBjb25zdCB3aXJlID0gZW5jb2RlQWN0aW9uV2lyZSh7IGNvbnRyb2xsZXJJZDogbmV4dFNlc3Npb24uYXBwLmNvbnRyb2xsZXJJZCwgYWN0aW9uOiBcInNldEFwcFJlZ2lzdHJhdGlvbnNcIiwgYXJnczogeyBqc29uOiBhcHBSZWdpc3RyYXRpb25zSnNvbiB9IH0pO1xuICAgICAgICAgICAgICBhd2FpdCBwbHVnaW5FbnRyeS5oYW5kbGUuaGFuZGxlQWN0aW9uKG5leHRTZXNzaW9uLmluc3RhbmNlSWQsIHdpcmUsIG5leHRTZXNzaW9uLnZpZXdTdGF0ZSk7XG4gICAgICAgICAgICB9IGNhdGNoIChlcnJvcikge1xuICAgICAgICAgICAgICBjb25zb2xlLndhcm4oXCJbREVCVUddIHNldEFwcFJlZ2lzdHJhdGlvbnMgcHVzaCBza2lwcGVkXCIsIGVycm9yIGluc3RhbmNlb2YgRXJyb3IgPyBlcnJvci5tZXNzYWdlIDogU3RyaW5nKGVycm9yKSk7XG4gICAgICAgICAgICB9XG4gICAgICAgICAgfVxuICAgICAgICB9XG4gICAgICB9XG4gICAgICAvLyDwn5Ci77iPIE1lcmdlLXdpdGgtaWRlbnRpdHktcHJlc2VydmF0aW9uOiB1bnJlcXVlc3RlZC91bmNoYW5nZWQgc2VjdGlvbnMga2VlcCBleGFjdGx5IHRoZSBvYmplY3RcbiAgICAgIC8vIHJlZmVyZW5jZSBhbHJlYWR5IGluIGBjYWNoZWAgKGRpc3BhdGNoZWQgZnJvbSBhIHByaW9yIHJlZnJlc2gpLCBzbyBgbWVyZ2VSZWNvcmRQcmVzZXJ2aW5nSWRlbnRpdHlgXG4gICAgICAvLyBiYWlscyBvbiB0aGVtIHZpYSByZWZlcmVuY2UgZXF1YWxpdHkg4oCUIHRoaXMgaXMgd2hhdCBsZXRzIGBJbnRlcnByZXRlZFVpTm9kZWAncyBgUmVhY3QubWVtb2AgKGFuZFxuICAgICAgLy8gYG1vZGVXaW5kb3dzYCdzIGB1c2VNZW1vYCkgc2tpcCByZWNvbmNpbGluZyB0aGUgd2hvbGUgc2hlbGwgb24gZXZlcnkgaW50ZXJhY3Rpb24uXG4gICAgICBkaXNwYXRjaCh7XG4gICAgICAgIHR5cGU6IFwiU0VUX1dJTkRPV19VSV9CWV9XSU5ET1dfSURcIixcbiAgICAgICAgdmFsdWU6IChjdXJyZW50KSA9PlxuICAgICAgICAgIG1lcmdlUmVjb3JkUHJlc2VydmluZ0lkZW50aXR5KFxuICAgICAgICAgICAgY3VycmVudCxcbiAgICAgICAgICAgIHdpbmRvd0luc3RhbmNlcy5tYXAoKGluc3RhbmNlKSA9PiBbaW5zdGFuY2UuaWQsIChjYWNoZS5nZXQoYHdpbmRvdzoke2luc3RhbmNlLmlkfWApPy52YWx1ZSBhcyBVaU5vZGUgfCB1bmRlZmluZWQpID8/IGN1cnJlbnRbaW5zdGFuY2UuaWRdID8/IHBlbmRpbmdXaW5kb3dVaU5vZGUoKV0gYXMgY29uc3QpLFxuICAgICAgICAgICksXG4gICAgICB9KTtcbiAgICAgIGNvbnN0IGR5bmFtaWNFbmdhZ2VtZW50cyA9IChjYWNoZS5nZXQoXCJlbmdhZ2VtZW50c1wiKT8udmFsdWUgYXMgUmVhZG9ubHk8UmVjb3JkPHN0cmluZywgV2luZG93RW5nYWdlbWVudD4+IHwgdW5kZWZpbmVkKSA/PyB7fTtcbiAgICAgIGRpc3BhdGNoKHtcbiAgICAgICAgdHlwZTogXCJTRVRfV0lORE9XX0VOR0FHRU1FTlRTX0JZX1dJTkRPV19JRFwiLFxuICAgICAgICB2YWx1ZTogKGN1cnJlbnQpID0+IG1lcmdlUmVjb3JkUHJlc2VydmluZ0lkZW50aXR5KGN1cnJlbnQsIE9iamVjdC5lbnRyaWVzKGR5bmFtaWNFbmdhZ2VtZW50cykpLFxuICAgICAgfSk7XG4gICAgICBjb25zdCBkeW5hbWljTWVhc3VyZXMgPSAoY2FjaGUuZ2V0KFwibWVhc3VyZXNcIik/LnZhbHVlIGFzIFJlYWRvbmx5PFJlY29yZDxzdHJpbmcsIHJlYWRvbmx5IFdpbmRvd01lYXN1cmVbXT4+IHwgdW5kZWZpbmVkKSA/PyB7fTtcbiAgICAgIGRpc3BhdGNoKHtcbiAgICAgICAgdHlwZTogXCJTRVRfV0lORE9XX01FQVNVUkVTX0JZX1dJTkRPV19JRFwiLFxuICAgICAgICB2YWx1ZTogKGN1cnJlbnQpID0+IG1lcmdlUmVjb3JkUHJlc2VydmluZ0lkZW50aXR5KGN1cnJlbnQsIE9iamVjdC5lbnRyaWVzKGR5bmFtaWNNZWFzdXJlcykpLFxuICAgICAgfSk7XG4gICAgICBjb25zdCBkeW5hbWljVG9vbE1lYXN1cmVzID0gKGNhY2hlLmdldChcInRvb2xzXCIpPy52YWx1ZSBhcyBSZWFkb25seTxSZWNvcmQ8c3RyaW5nLCByZWFkb25seSBXaW5kb3dNZWFzdXJlW10+PiB8IHVuZGVmaW5lZCkgPz8ge307XG4gICAgICBkaXNwYXRjaCh7XG4gICAgICAgIHR5cGU6IFwiU0VUX1RPT0xfTUVBU1VSRVNfQllfVE9PTF9JRFwiLFxuICAgICAgICB2YWx1ZTogKGN1cnJlbnQpID0+IG1lcmdlUmVjb3JkUHJlc2VydmluZ0lkZW50aXR5KGN1cnJlbnQsIE9iamVjdC5lbnRyaWVzKGR5bmFtaWNUb29sTWVhc3VyZXMpKSxcbiAgICAgIH0pO1xuICAgICAgY29uc3QgZnJlc2hBcHBMYWJlbHNPdmVybGF5ID0gbm9ybWFsaXplQXBwTGFiZWxzT3ZlcmxheShjYWNoZS5nZXQoXCJsYWJlbHNcIik/LnZhbHVlIGFzIFBhcnRpYWw8UGx1Z2luQXBwTGFiZWxzT3ZlcmxheT4gfCB1bmRlZmluZWQpO1xuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9BUFBfTEFCRUxTX09WRVJMQVlcIiwgdmFsdWU6IChjdXJyZW50KSA9PiBwcmVzZXJ2ZUpzb25JZGVudGl0eShjdXJyZW50LCBmcmVzaEFwcExhYmVsc092ZXJsYXkpIH0pO1xuICAgICAgZGlzcGF0Y2goe1xuICAgICAgICB0eXBlOiBcIlNFVF9QQU5FTF9VSV9CWV9LRVlcIixcbiAgICAgICAgdmFsdWU6IChjdXJyZW50KSA9PlxuICAgICAgICAgIG1lcmdlUmVjb3JkUHJlc2VydmluZ0lkZW50aXR5KFxuICAgICAgICAgICAgY3VycmVudCxcbiAgICAgICAgICAgIHBhbmVsVGFiTGVhdmVzXG4gICAgICAgICAgICAgIC5maWx0ZXIoKHRhYikgPT4gdGFiLmJvZHlLZXkpXG4gICAgICAgICAgICAgIC5tYXAoKHRhYikgPT4gW3BhbmVsVGFiS2luZElkKHRhYi5raW5kKSwgKGNhY2hlLmdldChgcGFuZWw6JHtwYW5lbFRhYktpbmRJZCh0YWIua2luZCl9YCk/LnZhbHVlIGFzIFVpTm9kZSB8IHVuZGVmaW5lZCkgPz8gY3VycmVudFtwYW5lbFRhYktpbmRJZCh0YWIua2luZCldID8/IHBlbmRpbmdQYW5lbFVpTm9kZSgpXSBhcyBjb25zdCksXG4gICAgICAgICAgKSxcbiAgICAgIH0pO1xuICAgICAgaWYgKGlzU2Vzc2lvblN3aXRjaCAmJiBsYXlvdXRTZWVkKSB7XG4gICAgICAgIGxheW91dFNlZWRLZXlSZWYuY3VycmVudCA9IGxheW91dFNlZWRLZXk7XG4gICAgICAgIGV4dHJhV2luZG93SW5zdGFuY2VzUmVmLmN1cnJlbnQgPSBsYXlvdXRTZWVkLmV4dHJhSW5zdGFuY2VzO1xuICAgICAgICBleHRyYVdpbmRvd0NvdW50ZXJSZWYuY3VycmVudCA9IGxheW91dFNlZWQuZXh0cmFJbnN0YW5jZXMubGVuZ3RoO1xuICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0VYVFJBX1dJTkRPV19JTlNUQU5DRVNcIiwgdmFsdWU6IGxheW91dFNlZWQuZXh0cmFJbnN0YW5jZXMgfSk7XG4gICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfU0hFTExfTEFZT1VUXCIsIHZhbHVlOiBsYXlvdXRTZWVkLm1vZGVMYXlvdXQgfSk7XG4gICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfQUNUSVZFX1dJTkRPV19JRFwiLCB2YWx1ZTogbnVsbCB9KTtcbiAgICAgIH1cbiAgICB9LFxuICAgIC8vIPCfkKLvuI8gYGFwcGx5SG9zdEVmZmVjdHNgIGlzIGRlY2xhcmVkIGxhdGVyIGluIHRoaXMgY29tcG9uZW50IChpdHMgb3duIGRlcHMgbmVlZCBgdXBkYXRlU3BhY2VQYW5lbGAvXG4gICAgLy8gYHN5bmNTcGF3bmVkUGx1Z2luRG9jdW1lbnRgLCBkZWNsYXJlZCBsYXRlciBzdGlsbCkg4oCUIHJlZmVyZW5jaW5nIGl0IGhlcmUgaW4gdGhlIGJvZHkgb25seSAobmV2ZXJcbiAgICAvLyBhZGRlZCB0byB0aGlzIGFycmF5KSBhdm9pZHMgYSB0ZW1wb3JhbC1kZWFkLXpvbmUgcmVmZXJlbmNlLWJlZm9yZS1pbml0OyBzYWZlIGJlY2F1c2UgdGhpcyBjYWxsYmFja1xuICAgIC8vIGlzIG9ubHkgZXZlciBpbnZva2VkIGFmdGVyIHJlbmRlciBjb21wbGV0ZXMsIGJ5IHdoaWNoIHBvaW50IGBhcHBseUhvc3RFZmZlY3RzYCBpcyBpbml0aWFsaXplZC5cbiAgICAvLyBlc2xpbnQtZGlzYWJsZS1uZXh0LWxpbmUgcmVhY3QtaG9va3MvZXhoYXVzdGl2ZS1kZXBzXG4gICAgW2FwcExhYmVsc092ZXJsYXksIGluamVjdEFjdGl2ZVRvb2wsIGxvYWRlZFBsdWdpbnMsIHVpTG9jYWxlLCB1aVRlcm1pbm9sb2d5XSxcbiAgKTtcblxuICAvKiogQGVtb2ppIPCfl6PvuI8gS2VlcHMgYWxyZWFkeS1idWlsdCB3aW5kb3cgdGl0bGVzICh3b3JrYmVuY2ggbGF5b3V0LCBleHRyYSBzcGF3bmVkIHdpbmRvd3MpIGluIHN5bmMgb24gZXZlcnkgbG9jYWxlL3Rlcm1pbm9sb2d5IHN3aXRjaCDigJQgYHJlZnJlc2hVaWAgb25seSByZWJ1aWxkcyBgc2hlbGxMYXlvdXRgIGZyb20gc2NyYXRjaCBvbiBhIHNlc3Npb24gY2hhbmdlLCBzbyBhbiBleGlzdGluZyBzZXNzaW9uJ3MgYmFrZWQtaW4gdGl0bGVzIHdvdWxkIG90aGVyd2lzZSBnbyBzdGFsZS4gKi9cbiAgdXNlRWZmZWN0KCgpID0+IHtcbiAgICBjb25zdCB3aW5kb3dLaW5kcyA9IHNlc3Npb24/LmFwcC53aW5kb3dLaW5kcztcbiAgICBpZiAoIXdpbmRvd0tpbmRzKSByZXR1cm47XG4gICAgZGlzcGF0Y2goe1xuICAgICAgdHlwZTogXCJTRVRfU0hFTExfTEFZT1VUXCIsXG4gICAgICB2YWx1ZTogKGN1cnJlbnQpID0+IChjdXJyZW50ID8gcmV0aXRsZVdpbmRvd0xheW91dE5vZGUoY3VycmVudCwgd2luZG93S2luZHMsIGV4dHJhV2luZG93SW5zdGFuY2VzUmVmLmN1cnJlbnQsIHVpVGVybWlub2xvZ3ksIHVpTG9jYWxlKSA6IGN1cnJlbnQpLFxuICAgIH0pO1xuICAgIGRpc3BhdGNoKHtcbiAgICAgIHR5cGU6IFwiU0VUX0VYVFJBX1dJTkRPV19JTlNUQU5DRVNcIixcbiAgICAgIHZhbHVlOiAoY3VycmVudCkgPT4ge1xuICAgICAgICBjb25zdCBuZXh0ID0gY3VycmVudC5tYXAoKGVudHJ5KSA9PiB7XG4gICAgICAgICAgY29uc3Qga2luZCA9IHdpbmRvd0tpbmRzLmZpbmQoKGspID0+IGsuaWQgPT09IGVudHJ5LndpbmRvd0tpbmRJZCB8fCBrLmlkID09PSBlbnRyeS5pZCk7XG4gICAgICAgICAgY29uc3QgdGl0bGUgPSBraW5kID8gcmVzb2x2ZU1hbmlmZXN0TGFiZWwoa2luZC5sYWJlbCwgdWlUZXJtaW5vbG9neSwgdWlMb2NhbGUpIDogZW50cnkudGl0bGU7XG4gICAgICAgICAgcmV0dXJuIHsgLi4uZW50cnksIHRpdGxlIH07XG4gICAgICAgIH0pO1xuICAgICAgICBleHRyYVdpbmRvd0luc3RhbmNlc1JlZi5jdXJyZW50ID0gbmV4dDtcbiAgICAgICAgcmV0dXJuIG5leHQ7XG4gICAgICB9LFxuICAgIH0pO1xuICB9LCBbdWlUZXJtaW5vbG9neSwgdWlMb2NhbGVdKTtcblxuICBjb25zdCByZWZyZXNoU3Bhd25lZFVpID0gdXNlQ2FsbGJhY2soXG4gICAgYXN5bmMgKHNwYXduZWQ6IFNwYXduZWRBcHBFbnRyeSwgdmlld1N0YXRlOiBWaWV3TW9kZWwsIHNjb3BlQXJnOiBVaURpcnR5U2NvcGUgPSB7IGtpbmQ6IFwiZnVsbFwiIH0pID0+IHtcbiAgICAgIGlmIChzY29wZUFyZy5raW5kID09PSBcIm5vbmVcIikgcmV0dXJuO1xuICAgICAgY29uc3QgZ2VuZXJhdGlvbiA9ICsrc3Bhd25lZFJlZnJlc2hHZW5lcmF0aW9uUmVmLmN1cnJlbnQ7XG4gICAgICBjb25zdCBwbHVnaW5FbnRyeSA9IGxvYWRlZFBsdWdpbnMuZmluZCgoZW50cnkpID0+IGVudHJ5LmhhbmRsZS5wbHVnaW5JZCA9PT0gc3Bhd25lZC5wbHVnaW5JZCk7XG4gICAgICBjb25zdCBwbHVnaW4gPSBwbHVnaW5FbnRyeT8uaGFuZGxlO1xuICAgICAgY29uc3QgYXBwID0gcGx1Z2luRW50cnk/Lm1hbmlmZXN0LmFwcHMuZmluZCgoY2FuZGlkYXRlKSA9PiBjYW5kaWRhdGUuaWQgPT09IHNwYXduZWQuYXBwSWQpO1xuICAgICAgaWYgKCFwbHVnaW4gfHwgIWFwcCkge1xuICAgICAgICBjb25zb2xlLndhcm4oXCJbb3Mtc2hlbGxdIHJlZnJlc2hTcGF3bmVkVWk6IHBsdWdpbi9hcHAgdW5hdmFpbGFibGVcIiwgeyBwbHVnaW5JZDogc3Bhd25lZC5wbHVnaW5JZCwgYXBwSWQ6IHNwYXduZWQuYXBwSWQgfSk7XG4gICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfU1BBV05FRF9XSU5ET1dfVUlcIiwgdmFsdWU6IHsgdHlwZTogXCJ0ZXh0XCIsIHZhbHVlOiBgUGx1Z2luIHVuYXZhaWxhYmxlOiAke3NwYXduZWQucGx1Z2luSWR9LyR7c3Bhd25lZC5hcHBJZH1gIH0gYXMgVWlOb2RlIH0pO1xuICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1NQQVdORURfV0lORE9XX0VOR0FHRU1FTlRTXCIsIHZhbHVlOiB7fSB9KTtcbiAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9TUEFXTkVEX1dJTkRPV19NRUFTVVJFU1wiLCB2YWx1ZToge30gfSk7XG4gICAgICAgIHJldHVybjtcbiAgICAgIH1cbiAgICAgIGNvbnN0IHNwYXduZWRTZWVkID0gYCR7c3Bhd25lZC5wbHVnaW5JZH06JHtzcGF3bmVkLmFwcElkfToke3NwYXduZWQuaW5zdGFuY2VJZH1gO1xuICAgICAgaWYgKHNwYXduZWRMYXlvdXRTZWVkUmVmLmN1cnJlbnQgIT09IHNwYXduZWRTZWVkKSB7XG4gICAgICAgIHNwYXduZWRMYXlvdXRTZWVkUmVmLmN1cnJlbnQgPSBzcGF3bmVkU2VlZDtcbiAgICAgICAgc3Bhd25lZFVpUmVmcmVzaENhY2hlUmVmLmN1cnJlbnQgPSBuZXcgTWFwKCk7XG4gICAgICB9XG4gICAgICBjb25zdCBjYWNoZSA9IHNwYXduZWRVaVJlZnJlc2hDYWNoZVJlZi5jdXJyZW50O1xuICAgICAgY29uc3QgY29udHJpYnV0aW9uc0pzb24gPSBidWlsZENvbnRyaWJ1dGlvbnNKc29uKGxvYWRlZFBsdWdpbnMubWFwKChlbnRyeSkgPT4gKHsgcGx1Z2luSWQ6IGVudHJ5LmhhbmRsZS5wbHVnaW5JZCwgbWFuaWZlc3Q6IGVudHJ5Lm1hbmlmZXN0IH0pKSk7XG4gICAgICBjb25zdCBib2R5S2V5ID0gcmVzb2x2ZUNhbnZhc0JvZHlLZXkoYXBwKTtcbiAgICAgIGNvbnN0IGZ1bGxWaWV3U3RhdGU6IFZpZXdNb2RlbCA9IGluamVjdEFjdGl2ZVV0aWxpdHkoXG4gICAgICAgIHsgLi4udmlld1N0YXRlLCBjb250cmlidXRpb25zSnNvbiwgbG9jYWxlOiB1aUxvY2FsZSwgdGVybWlub2xvZ3k6IHVpVGVybWlub2xvZ3ksIHdpbmRvd0lkOiBib2R5S2V5LCB3aW5kb3dJbnN0YW5jZXM6IFt7IGlkOiBib2R5S2V5LCB3aW5kb3dLaW5kSWQ6IGJvZHlLZXkgfV0gfSxcbiAgICAgICAgc3Bhd25lZC5pZCxcbiAgICAgICk7XG4gICAgICAvLyDwn5Ci77iPIEEgc3Bhd25lZCBpbnN0YW5jZSdzIHZpZXcgaXMgYSBzaW5nbGUgYm9keSArIHV0aWxpdGllcyArIGVuZ2FnZW1lbnRzICsgbWVhc3VyZXMgKG5vIHBhbmVscywgbm9cbiAgICAgIC8vIGxhYmVscykg4oCUIHRoYXQncyBhbHJlYWR5IHRoZSBtaW5pbWFsIGdyb3VwaW5nLCBzbyB0aGVyZSBpcyBubyBuYXJyb3dlci10aGFuLWZ1bGwgXCJwYXJ0aWFsXCIgc2NvcGVcbiAgICAgIC8vIHdvcnRoIGV4cHJlc3NpbmcgaGVyZTsgb25seSBgbm9uZWAgKGhhbmRsZWQgYWJvdmUpIHNob3J0LWNpcmN1aXRzIHRoZSByZXF1ZXN0LlxuICAgICAgY29uc3Qgc2luZ2xlV2luZG93S2luZCA9IFt7IGlkOiBib2R5S2V5LCBib2R5S2V5IH1dO1xuICAgICAgY29uc3QgcmVxdWVzdCA9IGJ1aWxkVWlSZWZyZXNoUmVxdWVzdCh7IGtpbmQ6IFwiZnVsbFwiIH0sIHNpbmdsZVdpbmRvd0tpbmQsIFtdLCBmdWxsVmlld1N0YXRlLCBjYWNoZSk7XG4gICAgICBpZiAocmVxdWVzdCkge1xuICAgICAgICBjb25zdCByZXNwb25zZSA9IGF3YWl0IHBsdWdpbi5yZWZyZXNoVWkoc3Bhd25lZC5pbnN0YW5jZUlkLCByZXF1ZXN0KTtcbiAgICAgICAgaWYgKGdlbmVyYXRpb24gIT09IHNwYXduZWRSZWZyZXNoR2VuZXJhdGlvblJlZi5jdXJyZW50KSByZXR1cm47XG4gICAgICAgIGFwcGx5VWlSZWZyZXNoUmVzcG9uc2VUb0NhY2hlKGNhY2hlLCByZXNwb25zZSk7XG4gICAgICB9XG4gICAgICBjb25zdCB1aSA9IChjYWNoZS5nZXQoYHdpbmRvdzoke2JvZHlLZXl9YCk/LnZhbHVlIGFzIFVpTm9kZSB8IHVuZGVmaW5lZCkgPz8gcGVuZGluZ1dpbmRvd1VpTm9kZSgpO1xuICAgICAgY29uc3QgZHluYW1pY0VuZ2FnZW1lbnRzID0gKGNhY2hlLmdldChcImVuZ2FnZW1lbnRzXCIpPy52YWx1ZSBhcyBSZWFkb25seTxSZWNvcmQ8c3RyaW5nLCBXaW5kb3dFbmdhZ2VtZW50Pj4gfCB1bmRlZmluZWQpID8/IHt9O1xuICAgICAgY29uc3QgZHluYW1pY01lYXN1cmVzID0gKGNhY2hlLmdldChcIm1lYXN1cmVzXCIpPy52YWx1ZSBhcyBSZWFkb25seTxSZWNvcmQ8c3RyaW5nLCByZWFkb25seSBXaW5kb3dNZWFzdXJlW10+PiB8IHVuZGVmaW5lZCkgPz8ge307XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1NQQVdORURfV0lORE9XX1VJXCIsIHZhbHVlOiAoY3VycmVudDogVWlOb2RlIHwgbnVsbCkgPT4gcHJlc2VydmVKc29uSWRlbnRpdHkoY3VycmVudCA/PyB1bmRlZmluZWQsIHVpKSB9KTtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfU1BBV05FRF9XSU5ET1dfRU5HQUdFTUVOVFNcIiwgdmFsdWU6IGR5bmFtaWNFbmdhZ2VtZW50cyB9KTtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfU1BBV05FRF9XSU5ET1dfTUVBU1VSRVNcIiwgdmFsdWU6IGR5bmFtaWNNZWFzdXJlcyB9KTtcbiAgICB9LFxuICAgIFtpbmplY3RBY3RpdmVVdGlsaXR5LCBsb2FkZWRQbHVnaW5zLCB1aUxvY2FsZSwgdWlUZXJtaW5vbG9neV0sXG4gICk7XG5cbiAgLy8g8J+Qou+4jyBLZXllZCBvbiB0aGUgcGx1Z2luSWQvYXBwL2luc3RhbmNlIHRyaXBsZSAobm90IGBzZXNzaW9uYCBvYmplY3QgaWRlbnRpdHkpIHNvIHRoaXMgb25seSBmaXJlcyBvblxuICAvLyBhIGdlbnVpbmUgc2Vzc2lvbiBzd2l0Y2ggKGFwcCBvcGVuL3NwYXduL2luc3RhbmNlIGNoYW5nZSkg4oCUIGV2ZXJ5IG90aGVyIGFjdGlvbiBhbHJlYWR5IGNhbGxzXG4gIC8vIGByZWZyZXNoVWlgIGV4cGxpY2l0bHkgdmlhIGBhcHBseUhvc3RFZmZlY3RzYCwgYW5kIHJlLXJ1bm5pbmcgaXQgaGVyZSB0b28gb24gZXZlcnkgYHNlc3Npb25gIG9iamVjdFxuICAvLyBjaHVybiB3YXMgYSBzZWNvbmQsIHJlZHVuZGFudCBmdWxsLXNoZWxsIHJlZnJlc2ggY2FzY2FkZSBwZXIgaW50ZXJhY3Rpb24uXG4gIGNvbnN0IHNlc3Npb25JZGVudGl0eUtleSA9IHNlc3Npb24gPyBgJHtzZXNzaW9uLnBsdWdpbklkfToke3Nlc3Npb24uYXBwLmlkfToke3Nlc3Npb24uaW5zdGFuY2VJZH1gIDogbnVsbDtcbiAgdXNlRWZmZWN0KCgpID0+IHtcbiAgICBjb25zdCBjdXJyZW50ID0gc2Vzc2lvblJlZi5jdXJyZW50O1xuICAgIGlmICghY3VycmVudCkgcmV0dXJuO1xuICAgIHZvaWQgcmVmcmVzaFVpKGN1cnJlbnQpLmNhdGNoKChyZW5kZXJFcnJvcikgPT4ge1xuICAgICAgY29uc29sZS5lcnJvcihcIltERUJVR10gcmVuZGVyIGZhaWxlZFwiLCByZW5kZXJFcnJvcik7XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0VSUk9SXCIsIHZhbHVlOiByZW5kZXJFcnJvciBpbnN0YW5jZW9mIEVycm9yID8gcmVuZGVyRXJyb3IubWVzc2FnZSA6IFN0cmluZyhyZW5kZXJFcnJvcikgfSk7XG4gICAgfSk7XG4gIH0sIFtsb2FkZWRQbHVnaW5zLCByZWZyZXNoVWksIHNlc3Npb25JZGVudGl0eUtleV0pO1xuXG4gIHVzZUVmZmVjdCgoKSA9PiB7XG4gICAgaWYgKCFzdHVkaW9Nb2RlIHx8ICFzZXNzaW9uKSB7XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1NQQVdORURfV0lORE9XX1VJXCIsIHZhbHVlOiBudWxsIH0pO1xuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9TUEFXTkVEX1dJTkRPV19FTkdBR0VNRU5UU1wiLCB2YWx1ZToge30gfSk7XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1NQQVdORURfV0lORE9XX01FQVNVUkVTXCIsIHZhbHVlOiB7fSB9KTtcbiAgICAgIHJldHVybjtcbiAgICB9XG4gICAgY29uc3QgYWN0aXZlU3Bhd25lZCA9IHBhbmVsPy5zcGF3bmVkQXBwcy5maW5kKChlbnRyeSkgPT4gZW50cnkuaWQgPT09IHBhbmVsLmFjdGl2ZVNwYXduZWRJZCk7XG4gICAgaWYgKCFhY3RpdmVTcGF3bmVkKSB7XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1NQQVdORURfV0lORE9XX1VJXCIsIHZhbHVlOiBudWxsIH0pO1xuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9TUEFXTkVEX1dJTkRPV19FTkdBR0VNRU5UU1wiLCB2YWx1ZToge30gfSk7XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1NQQVdORURfV0lORE9XX01FQVNVUkVTXCIsIHZhbHVlOiB7fSB9KTtcbiAgICAgIHJldHVybjtcbiAgICB9XG4gICAgdm9pZCByZWZyZXNoU3Bhd25lZFVpKGFjdGl2ZVNwYXduZWQsIHNlc3Npb24udmlld1N0YXRlKS5jYXRjaCgocmVuZGVyRXJyb3IpID0+IHtcbiAgICAgIGNvbnNvbGUuZXJyb3IoXCJbREVCVUddIHNwYXduZWQgcmVuZGVyIGZhaWxlZFwiLCByZW5kZXJFcnJvcik7XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1NQQVdORURfV0lORE9XX1VJXCIsIHZhbHVlOiBudWxsIH0pO1xuICAgIH0pO1xuICB9LCBbbG9hZGVkUGx1Z2lucywgcGFuZWwsIHJlZnJlc2hTcGF3bmVkVWksIHNlc3Npb24sIHN0dWRpb01vZGVdKTtcblxuICBjb25zdCB1cGRhdGVTcGFjZVBhbmVsID0gdXNlQ2FsbGJhY2soKHBhbmVsU3RhdGU6IFNwYWNlUGFuZWxTdGF0ZSkgPT4ge1xuICAgIGRpc3BhdGNoKHtcbiAgICAgIHR5cGU6IFwiU0VUX1NFU1NJT05cIixcbiAgICAgIHZhbHVlOiAoY3VycmVudCkgPT4ge1xuICAgICAgICBpZiAoIWN1cnJlbnQpIHJldHVybiBjdXJyZW50O1xuICAgICAgICByZXR1cm4geyAuLi5jdXJyZW50LCB2aWV3U3RhdGU6IHsgLi4uY3VycmVudC52aWV3U3RhdGUsIHBhbmVsSnNvbjogcGFuZWxKc29uRnJvbVN0YXRlKHBhbmVsU3RhdGUpIH0gfTtcbiAgICAgIH0sXG4gICAgfSk7XG4gIH0sIFtdKTtcblxuICAvLyDwn4+g77iP8J+ns++4jyBHZW5lcmljIHJlcGxhY2VtZW50IGZvciB0aGUgb2xkIGBzd2l0Y2hUb1NBcHBgIOKAlCBzd2l0Y2hlcyB0byBlaXRoZXIgdGhlIGhvc3QgcGx1Z2luJ3MgbGFuZGluZ1xuICAvLyBvciBob3N0IGFwcCBieSBpZCAoYm90aCByZXNvbHZlZCB2aWEgYGhvc3RDb25maWdgLCBuZXZlciBhIHNwZWNpZmljIGFwcCdzIGlkZW50aXR5KS5cbiAgY29uc3Qgc3dpdGNoVG9NYW5hZ2VkQXBwID0gdXNlQ2FsbGJhY2soXG4gICAgYXN5bmMgKGFwcElkOiBzdHJpbmcsIHZpZXdTdGF0ZT86IFZpZXdNb2RlbCk6IFByb21pc2U8QWN0aXZlU2Vzc2lvbiB8IG51bGw+ID0+IHtcbiAgICAgIGNvbnN0IHNQbHVnaW4gPSBob3N0Q29uZmlnID8gbG9hZGVkUGx1Z2lucy5maW5kKChlbnRyeSkgPT4gZW50cnkuaGFuZGxlLnBsdWdpbklkID09PSBob3N0Q29uZmlnLnBsdWdpbklkKSA6IHVuZGVmaW5lZDtcbiAgICAgIGNvbnN0IGFwcCA9IHNQbHVnaW4/Lm1hbmlmZXN0LmFwcHMuZmluZCgoY2FuZGlkYXRlKSA9PiBjYW5kaWRhdGUuaWQgPT09IGFwcElkKTtcbiAgICAgIGlmICghc1BsdWdpbiB8fCAhYXBwKSByZXR1cm4gbnVsbDtcbiAgICAgIGlmIChzZXNzaW9uPy5wbHVnaW5JZCA9PT0gc1BsdWdpbi5oYW5kbGUucGx1Z2luSWQgJiYgc2Vzc2lvbi5hcHAuaWQgPT09IGFwcElkKSB7XG4gICAgICAgIGlmICghdmlld1N0YXRlKSByZXR1cm4gc2Vzc2lvbjtcbiAgICAgICAgY29uc3QgbmV4dFNlc3Npb246IEFjdGl2ZVNlc3Npb24gPSB7IC4uLnNlc3Npb24sIHZpZXdTdGF0ZSB9O1xuICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1NFU1NJT05cIiwgdmFsdWU6IG5leHRTZXNzaW9uIH0pO1xuICAgICAgICBhd2FpdCByZWZyZXNoVWkobmV4dFNlc3Npb24pO1xuICAgICAgICByZXR1cm4gbmV4dFNlc3Npb247XG4gICAgICB9XG4gICAgICBjb25zdCBpbnN0YW5jZUlkID0gYXdhaXQgc1BsdWdpbi5oYW5kbGUuY3JlYXRlQXBwKGFwcC5pZCk7XG4gICAgICAvLyDwn6qm77iPIFNlZSBgZXN0YWJsaXNoUHJpbWFyeVNlc3Npb25gJ3MgY29tbWVudCBhYm92ZSDigJQgYHByb2dyYW1zYCBpcyBwZXJtYW5lbnRseSBlbXB0eSBub3cuXG4gICAgICBjb25zdCBuZXh0Vmlld1N0YXRlOiBWaWV3TW9kZWwgPSB2aWV3U3RhdGUgPz8ge1xuICAgICAgICBhY3RpdmVNb2RlSWQ6IGFwcC5kZWZhdWx0TW9kZUlkID8/IGFwcC5tb2Rlc1swXT8uaWQsXG4gICAgICAgIHBhbmVsSnNvbjogcGFuZWxKc29uRnJvbVN0YXRlKGJ1aWxkU3BhY2VQYW5lbFN0YXRlKFtdLCBbXSkpLFxuICAgICAgfTtcbiAgICAgIGNvbnN0IG5leHRTZXNzaW9uOiBBY3RpdmVTZXNzaW9uID0geyBwbHVnaW5JZDogc1BsdWdpbi5oYW5kbGUucGx1Z2luSWQsIGluc3RhbmNlSWQsIGFwcCwgdmlld1N0YXRlOiBuZXh0Vmlld1N0YXRlIH07XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1NFU1NJT05cIiwgdmFsdWU6IG5leHRTZXNzaW9uIH0pO1xuICAgICAgY29uc3Qgc2VlZGVkID0gYXBwbHlGcmFtZXdvcmtMYXlvdXRTZWVkKGFwcC5kZWZhdWx0TGF5b3V0LCBhcHAud2luZG93S2luZHMsIGFwcExhYmVsc092ZXJsYXksIHVpVGVybWlub2xvZ3ksIHVpTG9jYWxlKTtcbiAgICAgIGV4dHJhV2luZG93SW5zdGFuY2VzUmVmLmN1cnJlbnQgPSBzZWVkZWQuZXh0cmFJbnN0YW5jZXM7XG4gICAgICBleHRyYVdpbmRvd0NvdW50ZXJSZWYuY3VycmVudCA9IHNlZWRlZC5leHRyYUluc3RhbmNlcy5sZW5ndGg7XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0VYVFJBX1dJTkRPV19JTlNUQU5DRVNcIiwgdmFsdWU6IHNlZWRlZC5leHRyYUluc3RhbmNlcyB9KTtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfU0hFTExfTEFZT1VUXCIsIHZhbHVlOiBzZWVkZWQubW9kZUxheW91dCB9KTtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfQUNUSVZFX1dJTkRPV19JRFwiLCB2YWx1ZTogbnVsbCB9KTtcbiAgICAgIGlmIChhcHBJZCA9PT0gbGFuZGluZ0FwcElkKSB7XG4gICAgICAgIG9wZW5TcGFjZUlkUmVmLmN1cnJlbnQgPSBudWxsO1xuICAgICAgICBvcGVuSW5zdGFuY2VJZFJlZi5jdXJyZW50ID0gbnVsbDtcbiAgICAgIH1cbiAgICAgIGF3YWl0IHJlZnJlc2hVaShuZXh0U2Vzc2lvbik7XG4gICAgICByZXR1cm4gbmV4dFNlc3Npb247XG4gICAgfSxcbiAgICBbbG9hZGVkUGx1Z2lucywgcmVmcmVzaFVpLCBzZXNzaW9uLCBhcHBMYWJlbHNPdmVybGF5LCBob3N0Q29uZmlnLCBsYW5kaW5nQXBwSWQsIHVpVGVybWlub2xvZ3ksIHVpTG9jYWxlXSxcbiAgKTtcblxuICBjb25zdCBzeW5jU3Bhd25lZFBsdWdpbkRvY3VtZW50ID0gdXNlQ2FsbGJhY2soYXN5bmMgKHBsdWdpbjogUGx1Z2luV2FzbUhhbmRsZSwgYXBwOiBBcHBEZWZpbml0aW9uLCBwbHVnaW5JbnN0YW5jZUlkOiBudW1iZXIsIGRvY3VtZW50SnNvbjogc3RyaW5nLCB2aWV3U3RhdGU6IFZpZXdNb2RlbCkgPT4ge1xuICAgIHRyeSB7XG4gICAgICBjb25zdCBkb2N1bWVudCA9IEpTT04ucGFyc2UoZG9jdW1lbnRKc29uKSBhcyBSZWNvcmQ8c3RyaW5nLCB1bmtub3duPjtcbiAgICAgIGF3YWl0IHBsdWdpbi5oYW5kbGVBY3Rpb24ocGx1Z2luSW5zdGFuY2VJZCwgZW5jb2RlQWN0aW9uV2lyZSh7IGNvbnRyb2xsZXJJZDogYXBwLmNvbnRyb2xsZXJJZCwgYWN0aW9uOiBcInNldERvY3VtZW50XCIsIGFyZ3M6IHsgZG9jdW1lbnQgfSB9KSwgdmlld1N0YXRlKTtcbiAgICB9IGNhdGNoIChzeW5jRXJyb3IpIHtcbiAgICAgIGNvbnNvbGUuZXJyb3IoXCJbREVCVUddIHNwYXduZWQgcHJvZ3JhbSBkb2N1bWVudCBzeW5jIGZhaWxlZFwiLCBzeW5jRXJyb3IpO1xuICAgIH1cbiAgfSwgW10pO1xuXG4gIGNvbnN0IGVuc3VyZVNwYXduZWRQbHVnaW4gPSB1c2VDYWxsYmFjayhcbiAgICBhc3luYyAocHJvZ3JhbTogU3BhY2VQcm9ncmFtRW50cnksIGxhYmVsPzogc3RyaW5nLCBvc0luc3RhbmNlSWQ/OiBzdHJpbmcsIGRvY3VtZW50SnNvbj86IHN0cmluZywgc291cmNlVmlld1N0YXRlPzogVmlld01vZGVsKTogUHJvbWlzZTxTcGFjZVBhbmVsU3RhdGUgfCBudWxsPiA9PiB7XG4gICAgICBjb25zdCBwbHVnaW5FbnRyeSA9IGxvYWRlZFBsdWdpbnMuZmluZCgoZW50cnkpID0+IGVudHJ5LmhhbmRsZS5wbHVnaW5JZCA9PT0gcHJvZ3JhbS5wbHVnaW5JZCk7XG4gICAgICBpZiAoIXBsdWdpbkVudHJ5IHx8ICFzZXNzaW9uKSByZXR1cm4gbnVsbDtcbiAgICAgIGNvbnN0IGFwcCA9IHBsdWdpbkVudHJ5Lm1hbmlmZXN0LmFwcHMuZmluZCgoY2FuZGlkYXRlKSA9PiBjYW5kaWRhdGUuaWQgPT09IHByb2dyYW0uYXBwSWQpO1xuICAgICAgY29uc3QgY3VycmVudFBhbmVsID0gcGFyc2VQYW5lbFN0YXRlKHNvdXJjZVZpZXdTdGF0ZSA/PyBzZXNzaW9uLnZpZXdTdGF0ZSkgPz8gYnVpbGRTcGFjZVBhbmVsU3RhdGUoW10sIFtdKTtcbiAgICAgIGNvbnN0IGV4aXN0aW5nID0gb3NJbnN0YW5jZUlkID8gY3VycmVudFBhbmVsLnNwYXduZWRBcHBzLmZpbmQoKGVudHJ5KSA9PiBlbnRyeS5pZCA9PT0gb3NJbnN0YW5jZUlkKSA6IGN1cnJlbnRQYW5lbC5zcGF3bmVkQXBwcy5maW5kKChlbnRyeSkgPT4gZW50cnkuYXBwSWQgPT09IHByb2dyYW0uYXBwSWQgJiYgZW50cnkucGx1Z2luSWQgPT09IHByb2dyYW0ucGx1Z2luSWQpO1xuICAgICAgaWYgKGV4aXN0aW5nKSB7XG4gICAgICAgIGlmIChkb2N1bWVudEpzb24gJiYgYXBwKSB7XG4gICAgICAgICAgYXdhaXQgc3luY1NwYXduZWRQbHVnaW5Eb2N1bWVudChwbHVnaW5FbnRyeS5oYW5kbGUsIGFwcCwgZXhpc3RpbmcuaW5zdGFuY2VJZCwgZG9jdW1lbnRKc29uLCBzb3VyY2VWaWV3U3RhdGUgPz8gc2Vzc2lvbi52aWV3U3RhdGUpO1xuICAgICAgICB9XG4gICAgICAgIHJldHVybiBzdHVkaW9QYW5lbEZvY3VzaW5nU3Bhd25lZChjdXJyZW50UGFuZWwsIGV4aXN0aW5nKTtcbiAgICAgIH1cbiAgICAgIGNvbnN0IGluc3RhbmNlSWQgPSBhd2FpdCBwbHVnaW5FbnRyeS5oYW5kbGUuY3JlYXRlQXBwKHByb2dyYW0uYXBwSWQpO1xuICAgICAgaWYgKGRvY3VtZW50SnNvbiAmJiBhcHApIHtcbiAgICAgICAgYXdhaXQgc3luY1NwYXduZWRQbHVnaW5Eb2N1bWVudChwbHVnaW5FbnRyeS5oYW5kbGUsIGFwcCwgaW5zdGFuY2VJZCwgZG9jdW1lbnRKc29uLCBzb3VyY2VWaWV3U3RhdGUgPz8gc2Vzc2lvbi52aWV3U3RhdGUpO1xuICAgICAgfVxuICAgICAgY29uc3Qgc3Bhd25lZElkID0gb3NJbnN0YW5jZUlkID8/IGAke3Byb2dyYW0ucGx1Z2luSWR9LSR7aW5zdGFuY2VJZH1gO1xuICAgICAgcmV0dXJuIHN0dWRpb1BhbmVsRm9jdXNpbmdTcGF3bmVkKGN1cnJlbnRQYW5lbCwge1xuICAgICAgICBpZDogc3Bhd25lZElkLFxuICAgICAgICBwbHVnaW5JZDogcHJvZ3JhbS5wbHVnaW5JZCxcbiAgICAgICAgaW5zdGFuY2VJZCxcbiAgICAgICAgYXBwSWQ6IHByb2dyYW0uYXBwSWQsXG4gICAgICAgIGxhYmVsOiBsYWJlbCA/PyBwcm9ncmFtLmxhYmVsLFxuICAgICAgICBkb2N1bWVudDogcHJvZ3JhbS5kb2N1bWVudCxcbiAgICAgIH0pO1xuICAgIH0sXG4gICAgW2xvYWRlZFBsdWdpbnMsIHNlc3Npb24sIHN5bmNTcGF3bmVkUGx1Z2luRG9jdW1lbnRdLFxuICApO1xuXG4gIC8qKlxuICAgKiDwn5Ca77iPIENvbnN1bWVzIGEgcGx1Z2luIGFjdGlvbidzIHR5cGVkIGByZXF1ZXN0ZWRFZmZlY3RzOiBIb3N0RWZmZWN0W11gIChXUy1EJ3MgYEludm9jYXRpb25SZXNwb25zZWApIOKAlFxuICAgKiByZXBsYWNlcyB0aGUgZGVsZXRlZCBgcHJvY2Vzc1BsdWdpbk9wZXJhdGlvbnNgIHN0cmluZy1tYXRjaGluZy4gVGhlIGxlZ2FjeSBgc2V0RG9jdW1lbnRgLW1pcnJvclxuICAgKiBiYWNrYm9uZS13cml0ZSBibG9jayBpcyBnb25lIGVudGlyZWx5OiBkb2N1bWVudCBjb250ZW50IHN5bmMgbm93IGZsb3dzIHRocm91Z2hcbiAgICogYG9wZW5Eb2N1bWVudGAvYGNsb3NlRG9jdW1lbnRgJ3Mgd29ya2VyLWJhY2tlZCBgRG9jdW1lbnRIb3N0YCBsaWZlY3ljbGUsIG5vdCBhIHBlci1vcGVyYXRpb24gSlMgbWlycm9yLlxuICAgKi9cbiAgY29uc3QgYXBwbHlIb3N0RWZmZWN0cyA9IHVzZUNhbGxiYWNrKFxuICAgIGFzeW5jIChlZmZlY3RzOiByZWFkb25seSBIb3N0RWZmZWN0W10sIGJhc2VTZXNzaW9uOiBBY3RpdmVTZXNzaW9uLCB1aVNjb3BlOiBVaURpcnR5U2NvcGUgPSB7IGtpbmQ6IFwiZnVsbFwiIH0pID0+IHtcbiAgICAgIGxldCBuZXh0Vmlld1N0YXRlID0gYmFzZVNlc3Npb24udmlld1N0YXRlO1xuICAgICAgZm9yIChjb25zdCBlZmZlY3Qgb2YgZWZmZWN0cykge1xuICAgICAgICBpZiAoZWZmZWN0ID09PSBcInJlcXVlc3RTeW5jXCIpIGNvbnRpbnVlO1xuICAgICAgICBpZiAoXCJzZXRQYW5lbFwiIGluIGVmZmVjdCkge1xuICAgICAgICAgIG5leHRWaWV3U3RhdGUgPSB7IC4uLm5leHRWaWV3U3RhdGUsIHBhbmVsSnNvbjogZWZmZWN0LnNldFBhbmVsLnBhbmVsSnNvbiB9O1xuICAgICAgICAgIGNvbnRpbnVlO1xuICAgICAgICB9XG4gICAgICAgIGlmIChcInNldEFjdGl2ZVV0aWxpdHlcIiBpbiBlZmZlY3QpIHtcbiAgICAgICAgICAvLyDwn6ew77iPIEEgcHJvZ3JhbSBwcm9ncmFtbWF0aWNhbGx5IHN3aXRjaGVkIHV0aWxpdHk6IG1pcnJvciBpdCBpbnRvIHRoZSBob3N0LW93bmVkIHN0b3JlIHNsaWNlIEFORFxuICAgICAgICAgIC8vIHRoZSByZWYgYHJlZnJlc2hVaWAgcmVhZHMgKGJhcmUgYGRpc3BhdGNoYCBhbG9uZSBsZWF2ZXMgdGhlIG1hcCBzdGFsZSB1bnRpbCB0aGUgbmV4dCByZW5kZXIg4oCUXG4gICAgICAgICAgLy8gd2hpY2ggaXMgYWZ0ZXIgdGhpcyBzYW1lIHBhc3MncyByZWZyZXNoLCBzbyBicnVzaC9zdWdnZXN0aW9uIGdob3N0cyBhbmQgZ3VtYmFsbHMgbmV2ZXIgYXBwZWFyKS5cbiAgICAgICAgICBjb25zdCB7IHdpbmRvd0lkLCB1dGlsaXR5SWQgfSA9IGVmZmVjdC5zZXRBY3RpdmVVdGlsaXR5O1xuICAgICAgICAgIHNldEFjdGl2ZVV0aWxpdHlGb3JXaW5kb3cod2luZG93SWQsIHV0aWxpdHlJZCB8fCBudWxsKTtcbiAgICAgICAgICBpZiAodXRpbGl0eUlkICYmIGFjdGl2ZVRvb2xJZFJlZi5jdXJyZW50KSB7XG4gICAgICAgICAgICBhY3RpdmVUb29sSWRSZWYuY3VycmVudCA9IG51bGw7XG4gICAgICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0FDVElWRV9UT09MXCIsIHRvb2xJZDogbnVsbCB9KTtcbiAgICAgICAgICB9XG4gICAgICAgICAgaWYgKHdpbmRvd0lkID09PSBhY3RpdmVXaW5kb3dJZFJlZi5jdXJyZW50KSBuZXh0Vmlld1N0YXRlID0geyAuLi5uZXh0Vmlld1N0YXRlLCBhY3RpdmVVdGlsaXR5SWQ6IHV0aWxpdHlJZCB8fCB1bmRlZmluZWQsIGFjdGl2ZVRvb2xJZDogdXRpbGl0eUlkID8gdW5kZWZpbmVkIDogbmV4dFZpZXdTdGF0ZS5hY3RpdmVUb29sSWQgfTtcbiAgICAgICAgICBjb250aW51ZTtcbiAgICAgICAgfVxuICAgICAgICBpZiAoXCJzZXRBY3RpdmVUb29sXCIgaW4gZWZmZWN0KSB7XG4gICAgICAgICAgLy8g8J+boO+4jyBBIHByb2dyYW0gcHJvZ3JhbW1hdGljYWxseSBzd2l0Y2hlZCB0b29scyAoZS5nLiBwdXp6bGUzZCBmaWxsIHZpYSBlbmdhZ2VtZW50IHRleHQgY29tbWFuZCk6XG4gICAgICAgICAgLy8gbWlycm9yIGl0IGludG8gdGhlIGhvc3Qtb3duZWQgc3RvcmUgc2xpY2UsIGNsZWFyIGV2ZXJ5IHdpbmRvdydzIGFjdGl2ZSB1dGlsaXR5IChtdXR1YWxcbiAgICAgICAgICAvLyBleGNsdXNpb24g4oCUIGEgdG9vbCBhbmQgYSB3aW5kb3cgdXRpbGl0eSBuZXZlciBib3RoIGNsYWltIHRoZSBwb2ludGVyKSwgYW5kIGZvbGQgaXQgaW50byB0aGVcbiAgICAgICAgICAvLyB2aWV3IHN0YXRlIGZlZCB0byB0aGUgZm9sbG93LXVwIHJlZnJlc2guXG4gICAgICAgICAgY29uc3QgeyB0b29sSWQgfSA9IGVmZmVjdC5zZXRBY3RpdmVUb29sO1xuICAgICAgICAgIGFjdGl2ZVRvb2xJZFJlZi5jdXJyZW50ID0gdG9vbElkIHx8IG51bGw7XG4gICAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9BQ1RJVkVfVE9PTFwiLCB0b29sSWQ6IHRvb2xJZCB8fCBudWxsIH0pO1xuICAgICAgICAgIGlmICh0b29sSWQpIGNsZWFyQWxsV2luZG93VXRpbGl0aWVzKCk7XG4gICAgICAgICAgbmV4dFZpZXdTdGF0ZSA9IHsgLi4ubmV4dFZpZXdTdGF0ZSwgYWN0aXZlVG9vbElkOiB0b29sSWQgfHwgdW5kZWZpbmVkLCBhY3RpdmVVdGlsaXR5SWQ6IHRvb2xJZCA/IHVuZGVmaW5lZCA6IG5leHRWaWV3U3RhdGUuYWN0aXZlVXRpbGl0eUlkIH07XG4gICAgICAgICAgY29udGludWU7XG4gICAgICAgIH1cbiAgICAgICAgaWYgKFwicGF0Y2hXb3JsZDNkQ2hyb21lXCIgaW4gZWZmZWN0KSB7XG4gICAgICAgICAgY29uc3QgeyBzZWxlY3Rpb25Kc29uLCB2b3J0aWNlc0pzb24sIGRvY3VtZW50U2VsZWN0ZWRJZHMsIGRvY3VtZW50SGlnaGxpZ2h0ZWRJZHMgfSA9IGVmZmVjdC5wYXRjaFdvcmxkM2RDaHJvbWU7XG4gICAgICAgICAgY29uc3QgcGF0Y2ggPSB7IHNlbGVjdGlvbkpzb24sIHZvcnRpY2VzSnNvbiB9O1xuICAgICAgICAgIGNvbnN0IHdpbmRvd0luc3RhbmNlcyA9IHNlc3Npb25XaW5kb3dJbnN0YW5jZXMoYmFzZVNlc3Npb24uYXBwLCBleHRyYVdpbmRvd0luc3RhbmNlc1JlZi5jdXJyZW50KTtcbiAgICAgICAgICBjb25zdCBkb2N1bWVudFBhbmVsS2V5ID0gcGFuZWxUYWJLaW5kSWQoRlJBTUVXT1JLX1BBTkVMX1RBQl9ET0NVTUVOVF9JRCk7XG4gICAgICAgICAgZGlzcGF0Y2goe1xuICAgICAgICAgICAgdHlwZTogXCJTRVRfV0lORE9XX1VJX0JZX1dJTkRPV19JRFwiLFxuICAgICAgICAgICAgdmFsdWU6IChjdXJyZW50KSA9PlxuICAgICAgICAgICAgICBtZXJnZVJlY29yZFByZXNlcnZpbmdJZGVudGl0eShcbiAgICAgICAgICAgICAgICBjdXJyZW50LFxuICAgICAgICAgICAgICAgIHdpbmRvd0luc3RhbmNlcy5tYXAoKGluc3RhbmNlKSA9PiB7XG4gICAgICAgICAgICAgICAgICBjb25zdCBub2RlID0gY3VycmVudFtpbnN0YW5jZS5pZF07XG4gICAgICAgICAgICAgICAgICByZXR1cm4gW2luc3RhbmNlLmlkLCBub2RlID8gcGF0Y2hXb3JsZDNkQ2hyb21lT250b05vZGUobm9kZSwgcGF0Y2gpIDogbm9kZV0gYXMgY29uc3Q7XG4gICAgICAgICAgICAgICAgfSksXG4gICAgICAgICAgICAgICksXG4gICAgICAgICAgfSk7XG4gICAgICAgICAgZGlzcGF0Y2goe1xuICAgICAgICAgICAgdHlwZTogXCJTRVRfUEFORUxfVUlfQllfS0VZXCIsXG4gICAgICAgICAgICB2YWx1ZTogKGN1cnJlbnQpID0+IHtcbiAgICAgICAgICAgICAgY29uc3QgZG9jdW1lbnROb2RlID0gY3VycmVudFtkb2N1bWVudFBhbmVsS2V5XTtcbiAgICAgICAgICAgICAgaWYgKCFkb2N1bWVudE5vZGUpIHJldHVybiBjdXJyZW50O1xuICAgICAgICAgICAgICByZXR1cm4gbWVyZ2VSZWNvcmRQcmVzZXJ2aW5nSWRlbnRpdHkoY3VycmVudCwgW1tkb2N1bWVudFBhbmVsS2V5LCBwYXRjaERvY3VtZW50VHJlZVNlbGVjdGVkSWRzKGRvY3VtZW50Tm9kZSwgZG9jdW1lbnRTZWxlY3RlZElkcywgZG9jdW1lbnRIaWdobGlnaHRlZElkcyldXSk7XG4gICAgICAgICAgICB9LFxuICAgICAgICAgIH0pO1xuICAgICAgICAgIGNvbnN0IGNhY2hlID0gdWlSZWZyZXNoQ2FjaGVSZWYuY3VycmVudDtcbiAgICAgICAgICBmb3IgKGNvbnN0IGluc3RhbmNlIG9mIHdpbmRvd0luc3RhbmNlcykge1xuICAgICAgICAgICAgY29uc3QgY2FjaGVkID0gY2FjaGUuZ2V0KGB3aW5kb3c6JHtpbnN0YW5jZS5pZH1gKTtcbiAgICAgICAgICAgIGlmIChjYWNoZWQ/LnZhbHVlKSB7XG4gICAgICAgICAgICAgIGNhY2hlLnNldChgd2luZG93OiR7aW5zdGFuY2UuaWR9YCwgeyBoYXNoOiBjYWNoZWQuaGFzaCwgdmFsdWU6IHBhdGNoV29ybGQzZENocm9tZU9udG9Ob2RlKGNhY2hlZC52YWx1ZSBhcyBVaU5vZGUsIHBhdGNoKSB9KTtcbiAgICAgICAgICAgIH1cbiAgICAgICAgICB9XG4gICAgICAgICAgY29uc3QgZG9jdW1lbnRDYWNoZWQgPSBjYWNoZS5nZXQoYHBhbmVsOiR7ZG9jdW1lbnRQYW5lbEtleX1gKTtcbiAgICAgICAgICBpZiAoZG9jdW1lbnRDYWNoZWQ/LnZhbHVlKSB7XG4gICAgICAgICAgICBjYWNoZS5zZXQoYHBhbmVsOiR7ZG9jdW1lbnRQYW5lbEtleX1gLCB7XG4gICAgICAgICAgICAgIGhhc2g6IGRvY3VtZW50Q2FjaGVkLmhhc2gsXG4gICAgICAgICAgICAgIHZhbHVlOiBwYXRjaERvY3VtZW50VHJlZVNlbGVjdGVkSWRzKGRvY3VtZW50Q2FjaGVkLnZhbHVlIGFzIFVpTm9kZSwgZG9jdW1lbnRTZWxlY3RlZElkcywgZG9jdW1lbnRIaWdobGlnaHRlZElkcyksXG4gICAgICAgICAgICB9KTtcbiAgICAgICAgICB9XG4gICAgICAgICAgY29udGludWU7XG4gICAgICAgIH1cbiAgICAgICAgaWYgKFwib3BlbkRpYWxvZ1wiIGluIGVmZmVjdCkge1xuICAgICAgICAgIC8vIPCfl6jvuI8gUmVuZGVycyBmcm9tIHRoZSBhY3RpdmUgYGJhc2VTZXNzaW9uLmFwcGAg4oCUIGRpYWxvZ3Mgb3BlbmVkIGJ5IHNwYXduZWQgcHJvZ3JhbVxuICAgICAgICAgIC8vIGluc3RhbmNlcyBhcmUgdjEtb3V0LW9mLXNjb3BlLCBtaXJyb3JpbmcgdGhlIGludHJvZHVjdGlvbidzIGFjdGl2ZS1zZXNzaW9uLW9ubHkgc2NvcGUuXG4gICAgICAgICAgY29uc3QgeyBkaWFsb2dJZCwgYXJncyB9ID0gZWZmZWN0Lm9wZW5EaWFsb2c7XG4gICAgICAgICAgaWYgKGJhc2VTZXNzaW9uLmFwcC5kaWFsb2dzPy5zb21lKChlbnRyeSkgPT4gZW50cnkuaWQgPT09IGRpYWxvZ0lkKSkge1xuICAgICAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9ESUFMT0dcIiwgdmFsdWU6IHsgZGlhbG9nSWQsIHNlZWRBcmdzOiBhcmdzIGFzIFJlY29yZDxzdHJpbmcsIHVua25vd24+IHwgdW5kZWZpbmVkIH0gfSk7XG4gICAgICAgICAgfSBlbHNlIHtcbiAgICAgICAgICAgIGNvbnNvbGUuZXJyb3IoYFtvcy1zaGVsbF0gb3BlbkRpYWxvZzogYXBwICR7YmFzZVNlc3Npb24uYXBwLmlkfSBkZWNsYXJlcyBubyBkaWFsb2cgXCIke2RpYWxvZ0lkfVwiYCk7XG4gICAgICAgICAgfVxuICAgICAgICAgIGNvbnRpbnVlO1xuICAgICAgICB9XG4gICAgICAgIGlmIChcIm5hdmlnYXRlXCIgaW4gZWZmZWN0KSB7XG4gICAgICAgICAgbmF2aWdhdGVIaXN0b3J5KGVmZmVjdC5uYXZpZ2F0ZS51cmkpO1xuICAgICAgICAgIGNvbnRpbnVlO1xuICAgICAgICB9XG4gICAgICAgIGlmIChcImxvYWREb2N1bWVudFwiIGluIGVmZmVjdCkge1xuICAgICAgICAgIGNvbnN0IHBsdWdpbkVudHJ5ID0gbG9hZGVkUGx1Z2lucy5maW5kKChlbnRyeSkgPT4gZW50cnkuaGFuZGxlLnBsdWdpbklkID09PSBiYXNlU2Vzc2lvbi5wbHVnaW5JZCk7XG4gICAgICAgICAgY29uc3QgcGF5bG9hZCA9IGVmZmVjdC5sb2FkRG9jdW1lbnQ7XG4gICAgICAgICAgaWYgKHBheWxvYWQucGFjayAmJiBwYXlsb2FkLnNwciAmJiBwbHVnaW5FbnRyeT8uaGFuZGxlLmxvYWRBcHBEb2N1bWVudFBhY2spIHtcbiAgICAgICAgICAgIGNvbnN0IHBhY2tCeXRlcyA9IGNvZXJjZVdpcmVCeXRlcyhwYXlsb2FkLnBhY2spO1xuICAgICAgICAgICAgY29uc3Qgc3ByQnl0ZXMgPSBjb2VyY2VXaXJlQnl0ZXMocGF5bG9hZC5zcHIpO1xuICAgICAgICAgICAgY29uc29sZS5sb2coXCJbREVCVUddIGxvYWREb2N1bWVudCBwYWNrL3NwciBmb3IgaW5zdGFuY2VcIiwgYmFzZVNlc3Npb24uaW5zdGFuY2VJZCwgXCJwYWNrXCIsIHBhY2tCeXRlcy5sZW5ndGgsIFwic3ByXCIsIHNwckJ5dGVzLmxlbmd0aCk7XG4gICAgICAgICAgICBhd2FpdCBwbHVnaW5FbnRyeS5oYW5kbGUubG9hZEFwcERvY3VtZW50UGFjayhiYXNlU2Vzc2lvbi5pbnN0YW5jZUlkLCBwYWNrQnl0ZXMsIHNwckJ5dGVzKTtcbiAgICAgICAgICB9IGVsc2UgaWYgKHBheWxvYWQuZG9jdW1lbnRKc29uICYmIHBsdWdpbkVudHJ5Py5oYW5kbGUubG9hZEFwcERvY3VtZW50KSB7XG4gICAgICAgICAgICBjb25zb2xlLmxvZyhcIltERUJVR10gbG9hZERvY3VtZW50IGZvciBpbnN0YW5jZVwiLCBiYXNlU2Vzc2lvbi5pbnN0YW5jZUlkLCBcImJ5dGVzXCIsIHBheWxvYWQuZG9jdW1lbnRKc29uLmxlbmd0aCk7XG4gICAgICAgICAgICBhd2FpdCBwbHVnaW5FbnRyeS5oYW5kbGUubG9hZEFwcERvY3VtZW50KGJhc2VTZXNzaW9uLmluc3RhbmNlSWQsIHBheWxvYWQuZG9jdW1lbnRKc29uKTtcbiAgICAgICAgICB9IGVsc2Uge1xuICAgICAgICAgICAgY29uc29sZS5lcnJvcihcIltvcy1zaGVsbF0gbG9hZERvY3VtZW50OiBwcm9ncmFtIGhhcyBubyBwYWNrL2pzb24gbG9hZGVyXCIsIGJhc2VTZXNzaW9uLnBsdWdpbklkLCBPYmplY3Qua2V5cyhwYXlsb2FkKSk7XG4gICAgICAgICAgfVxuICAgICAgICAgIGNvbnRpbnVlO1xuICAgICAgICB9XG4gICAgICAgIGlmIChcIm9wZW5FeHRlcm5hbFVybFwiIGluIGVmZmVjdCkge1xuICAgICAgICAgIHdpbmRvdy5vcGVuKGVmZmVjdC5vcGVuRXh0ZXJuYWxVcmwudXJsLCBcIl9ibGFua1wiLCBcIm5vb3BlbmVyLG5vcmVmZXJyZXJcIik7XG4gICAgICAgICAgY29udGludWU7XG4gICAgICAgIH1cbiAgICAgICAgaWYgKFwiZG93bmxvYWRNZWRpYUV4cG9ydFwiIGluIGVmZmVjdCkge1xuICAgICAgICAgIGNvbnN0IHsgZmlsZW5hbWUsIG1pbWVUeXBlLCBkYXRhLCBlbmNvZGluZyB9ID0gZWZmZWN0LmRvd25sb2FkTWVkaWFFeHBvcnQ7XG4gICAgICAgICAgZG93bmxvYWRNZWRpYUV4cG9ydChmaWxlbmFtZSwgbWltZVR5cGUsIGRhdGEsIGVuY29kaW5nKTtcbiAgICAgICAgICBjb250aW51ZTtcbiAgICAgICAgfVxuICAgICAgICBpZiAoXCJpY29uUmVuZGVyRXhwb3J0XCIgaW4gZWZmZWN0KSB7XG4gICAgICAgICAgZm9yIChjb25zdCBpdGVtIG9mIGVmZmVjdC5pY29uUmVuZGVyRXhwb3J0Lml0ZW1zKSB7XG4gICAgICAgICAgICB0cnkge1xuICAgICAgICAgICAgICBjb25zdCByZXN1bHQgPSBhd2FpdCBpY29uUmVuZGVyUG9ydC5yZW5kZXIoaXRlbS5yZXF1ZXN0IGFzIFBhcmFtZXRlcnM8dHlwZW9mIGljb25SZW5kZXJQb3J0LnJlbmRlcj5bMF0pO1xuICAgICAgICAgICAgICBkb3dubG9hZERhdGFVcmwoaXRlbS5maWxlbmFtZSwgcmVzdWx0LmRhdGFVcmwpO1xuICAgICAgICAgICAgfSBjYXRjaCAoZXJyb3IpIHtcbiAgICAgICAgICAgICAgY29uc29sZS5lcnJvcihgaWNvbiByZW5kZXIgZXhwb3J0IGZhaWxlZCBmb3IgJHtpdGVtLmZpbGVuYW1lfWAsIGVycm9yKTtcbiAgICAgICAgICAgIH1cbiAgICAgICAgICB9XG4gICAgICAgICAgY29udGludWU7XG4gICAgICAgIH1cbiAgICAgICAgaWYgKFwicmVxdWVzdEZpbGVPcGVuXCIgaW4gZWZmZWN0KSB7XG4gICAgICAgICAgY29uc3QgeyBhY2NlcHQsIHJlYWRBcywgaW1wb3J0QWN0aW9uLCBtdWx0aXBsZSB9ID0gZWZmZWN0LnJlcXVlc3RGaWxlT3BlbjtcbiAgICAgICAgICBjb25zdCBvcGVuZWQgPSBhd2FpdCByZXF1ZXN0RmlsZU9wZW4oYWNjZXB0IHx8IFwiLnNwaywuZHNsLC5vcHMsYXBwbGljYXRpb24vb2N0ZXQtc3RyZWFtXCIsIHJlYWRBcywgbXVsdGlwbGUpO1xuICAgICAgICAgIGlmIChvcGVuZWQubGVuZ3RoID4gMCkge1xuICAgICAgICAgICAgY29uc3QgcGx1Z2luRW50cnkgPSBsb2FkZWRQbHVnaW5zLmZpbmQoKGVudHJ5KSA9PiBlbnRyeS5oYW5kbGUucGx1Z2luSWQgPT09IGJhc2VTZXNzaW9uLnBsdWdpbklkKTtcbiAgICAgICAgICAgIGlmIChwbHVnaW5FbnRyeSkge1xuICAgICAgICAgICAgICAvLyDwn5Ok77iPIFNpbmdsZS1maWxlIChtdWx0aXBsZSBhYnNlbnQvZmFsc2UpOiBpZGVudGljYWwgdG8gdGhlIHByZS1tdWx0aS1zZWxlY3Qgc2hhcGUsIG9uZVxuICAgICAgICAgICAgICAvLyBgaGFuZGxlQWN0aW9uYCBjYWxsIHdpdGggYHtwYXlsb2FkLCBuYW1lfWAuIE11bHRpLWZpbGU6IG9uZSBzZXF1ZW50aWFsIGNhbGwgcGVyIHNlbGVjdGVkXG4gICAgICAgICAgICAgIC8vIGZpbGUsIGVhY2ggZXh0ZW5kaW5nIGFyZ3Mgd2l0aCBge2luZGV4LCB0b3RhbH1gIHNvIHRoZSBwbHVnaW4gY2FuIHN0YWdlL21lcmdlIGltcG9ydHMuXG4gICAgICAgICAgICAgIGF3YWl0IGRpc3BhdGNoT3BlbmVkRmlsZXMob3BlbmVkLCBpbXBvcnRBY3Rpb24sIEJvb2xlYW4obXVsdGlwbGUpLCBtYWtlRWZmZWN0RGlzcGF0Y2hPbmUocGx1Z2luRW50cnksIGJhc2VTZXNzaW9uLCBhcHBseUhvc3RFZmZlY3RzKSk7XG4gICAgICAgICAgICB9XG4gICAgICAgICAgfVxuICAgICAgICAgIGNvbnRpbnVlO1xuICAgICAgICB9XG4gICAgICAgIGlmIChcImRpc3BhdGNoQWN0aW9uXCIgaW4gZWZmZWN0KSB7XG4gICAgICAgICAgLy8g8J+Uge+4jyBTZWxmIHJlLWRpc3BhdGNoIChEMik6IHJlLWludm9rZXMgdGhlIHNhbWUgcGx1Z2luIGluc3RhbmNlIHdpdGggYGFjdGlvbmAgYWZ0ZXIgYGRlbGF5TXNgLFxuICAgICAgICAgIC8vIHdpdGhvdXQgYmxvY2tpbmcgdGhlIGN1cnJlbnQgYGFwcGx5SG9zdEVmZmVjdHNgIHBhc3Mg4oCUIGBzZXRUaW1lb3V0YCAoMCBpcyBcIm5leHQgdGlja1wiKSBmaXJlc1xuICAgICAgICAgIC8vIHRoZSBmb2xsb3ctdXAgY2FsbCBhbmQgZmVlZHMgaXRzIG93biBgcmVxdWVzdGVkRWZmZWN0c2AgYmFjayB0aHJvdWdoIGBhcHBseUhvc3RFZmZlY3RzYFxuICAgICAgICAgIC8vIHJlY3Vyc2l2ZWx5LCBzbyBhIHBsdWdpbiBjYW4gY2hhaW4gc2V2ZXJhbCB0aWNrcyBvZiBzdGFnZWQvcHJvZ3Jlc3NpdmUgd29yayAoZS5nLiBhXG4gICAgICAgICAgLy8gbXVsdGktcGFzcyByZWNvbnN0cnVjdGlvbikgcHVyZWx5IGJ5IHJlLWVtaXR0aW5nIGBkaXNwYXRjaEFjdGlvbmAgZnJvbSBpdHMgb3duIGhhbmRsZXIuXG4gICAgICAgICAgY29uc3QgeyBhY3Rpb246IGRpc3BhdGNoQWN0aW9uSWQsIGFyZ3M6IGRpc3BhdGNoQXJncywgZGVsYXlNcyB9ID0gZWZmZWN0LmRpc3BhdGNoQWN0aW9uO1xuICAgICAgICAgIGNvbnN0IHBsdWdpbkVudHJ5ID0gbG9hZGVkUGx1Z2lucy5maW5kKChlbnRyeSkgPT4gZW50cnkuaGFuZGxlLnBsdWdpbklkID09PSBiYXNlU2Vzc2lvbi5wbHVnaW5JZCk7XG4gICAgICAgICAgaWYgKHBsdWdpbkVudHJ5KSB7XG4gICAgICAgICAgICBzY2hlZHVsZURpc3BhdGNoQWN0aW9uKGRpc3BhdGNoQWN0aW9uSWQsIGRpc3BhdGNoQXJncyBhcyBSZWNvcmQ8c3RyaW5nLCB1bmtub3duPiB8IHVuZGVmaW5lZCwgZGVsYXlNcywgbWFrZUVmZmVjdERpc3BhdGNoT25lKHBsdWdpbkVudHJ5LCBiYXNlU2Vzc2lvbiwgYXBwbHlIb3N0RWZmZWN0cykpO1xuICAgICAgICAgIH1cbiAgICAgICAgICBjb250aW51ZTtcbiAgICAgICAgfVxuICAgICAgICBpZiAoXCJyZXF1ZXN0TWVkaWFGcmFtZXNcIiBpbiBlZmZlY3QpIHtcbiAgICAgICAgICAvLyDwn46e77iPIEQ1OiBkZWNvZGVzIGEgdmlkZW8gKGZpbGUgcGlja2VyLCBvciBgcGF5bG9hZGAgYnl0ZXMgYWxyZWFkeSBpbiBoYW5kIGZyb20gYSBkcm9wIHpvbmUpXG4gICAgICAgICAgLy8gYW5kIGZhbnMgc2FtcGxlZCBmcmFtZXMgKyBhIGNvbXBsZXRpb24gbWFya2VyIG91dCB0aHJvdWdoIHRoZSBzYW1lIGBkaXNwYXRjaE9uZWAgcGF0aCBhc1xuICAgICAgICAgIC8vIGV2ZXJ5IG90aGVyIGVmZmVjdCBicmFuY2gg4oCUIHNlZSBgcnVuUmVxdWVzdE1lZGlhRnJhbWVzYCBmb3IgdGhlIFRpZXIgMSAoV2ViQ29kZWNzKS9UaWVyIDJcbiAgICAgICAgICAvLyAoYDx2aWRlbz5gIHNlZWstYW5kLWNhcHR1cmUpL2ZhbGxiYWNrIGRlY2lzaW9uIHRyZWUuXG4gICAgICAgICAgY29uc3QgeyBhY2NlcHQsIHBheWxvYWQsIGZyYW1lQWN0aW9uLCBkb25lQWN0aW9uLCBmYWxsYmFja0FjdGlvbiwgc2FtcGxlU3RyaWRlLCBtYXhGcmFtZXMsIG1heExvbmdFZGdlUHgsIGZwc0hpbnQsIGFyZ3MgfSA9IGVmZmVjdC5yZXF1ZXN0TWVkaWFGcmFtZXM7XG4gICAgICAgICAgY29uc3QgcGx1Z2luRW50cnkgPSBsb2FkZWRQbHVnaW5zLmZpbmQoKGVudHJ5KSA9PiBlbnRyeS5oYW5kbGUucGx1Z2luSWQgPT09IGJhc2VTZXNzaW9uLnBsdWdpbklkKTtcbiAgICAgICAgICBpZiAocGx1Z2luRW50cnkpIHtcbiAgICAgICAgICAgIGF3YWl0IHJ1blJlcXVlc3RNZWRpYUZyYW1lcyhcbiAgICAgICAgICAgICAge1xuICAgICAgICAgICAgICAgIGZyYW1lQWN0aW9uLFxuICAgICAgICAgICAgICAgIGRvbmVBY3Rpb24sXG4gICAgICAgICAgICAgICAgZmFsbGJhY2tBY3Rpb24sXG4gICAgICAgICAgICAgICAgc2FtcGxlU3RyaWRlOiBzYW1wbGVTdHJpZGUgPz8gMCxcbiAgICAgICAgICAgICAgICBtYXhGcmFtZXM6IG1heEZyYW1lcyA/PyAwLFxuICAgICAgICAgICAgICAgIG1heExvbmdFZGdlUHg6IG1heExvbmdFZGdlUHggPz8gMCxcbiAgICAgICAgICAgICAgICBmcHNIaW50OiBmcHNIaW50ID8/IDAsXG4gICAgICAgICAgICAgICAgYXJnczogYXJncyBhcyBSZWNvcmQ8c3RyaW5nLCB1bmtub3duPiB8IHVuZGVmaW5lZCxcbiAgICAgICAgICAgICAgfSxcbiAgICAgICAgICAgICAgYWNjZXB0LFxuICAgICAgICAgICAgICBwYXlsb2FkLFxuICAgICAgICAgICAgICBtYWtlRWZmZWN0RGlzcGF0Y2hPbmUocGx1Z2luRW50cnksIGJhc2VTZXNzaW9uLCBhcHBseUhvc3RFZmZlY3RzKSxcbiAgICAgICAgICAgICk7XG4gICAgICAgICAgfVxuICAgICAgICAgIGNvbnRpbnVlO1xuICAgICAgICB9XG4gICAgICAgIGlmIChcInJlcXVlc3RQbHVnaW5FeGNoYW5nZVwiIGluIGVmZmVjdCkge1xuICAgICAgICAgIGNvbnN0IHsgcGx1Z2luSWQsIGFwcElkLCByZXF1ZXN0SnNvbiwgcmVzcG9uc2VBY3Rpb24gfSA9IGVmZmVjdC5yZXF1ZXN0UGx1Z2luRXhjaGFuZ2U7XG4gICAgICAgICAgY29uc3QgcmVxdWVzdCA9IEpTT04ucGFyc2UocmVxdWVzdEpzb24pIGFzIHsgb3BlcmF0b3JJZD86IHN0cmluZzsgaW5wdXRKc29uPzogc3RyaW5nOyBub2RlSGFzaD86IG51bWJlciB9O1xuICAgICAgICAgIGNvbnN0IGNvbnRyaWJ1dG9yID0gbG9hZGVkUGx1Z2lucy5maW5kKChlbnRyeSkgPT4gZW50cnkuaGFuZGxlLnBsdWdpbklkID09PSBwbHVnaW5JZCk7XG4gICAgICAgICAgaWYgKGNvbnRyaWJ1dG9yICYmIHJlcXVlc3Qub3BlcmF0b3JJZCAmJiByZXF1ZXN0LmlucHV0SnNvbiAhPSBudWxsICYmIHJlcXVlc3Qubm9kZUhhc2ggIT0gbnVsbCkge1xuICAgICAgICAgICAgdHJ5IHtcbiAgICAgICAgICAgICAgY29uc3QgYmltID0gKGF3YWl0IGltcG9ydChcIkBzZW1pby10ZWNoL2Zsb3ctbW9kdWxlLWJpbVwiKSkgYXMgeyBldmFsdWF0ZT86IChraW5kSWQ6IHN0cmluZywgaW5wdXRKc29uOiBzdHJpbmcpID0+IHN0cmluZyB9O1xuICAgICAgICAgICAgICBjb25zdCBvdXRwdXRKc29uID0gdHlwZW9mIGJpbS5ldmFsdWF0ZSA9PT0gXCJmdW5jdGlvblwiID8gYmltLmV2YWx1YXRlKHJlcXVlc3Qub3BlcmF0b3JJZCwgcmVxdWVzdC5pbnB1dEpzb24pIDogXCJcIjtcbiAgICAgICAgICAgICAgY29uc29sZS5sb2coXCJbREVCVUddIHJlcXVlc3RQbHVnaW5FeGNoYW5nZSByZXNvbHZlZCBleHRlbnNpb24gZXZhbFwiLCB7IHBsdWdpbklkLCBhcHBJZCwgb3BlcmF0b3JJZDogcmVxdWVzdC5vcGVyYXRvcklkLCBub2RlSGFzaDogcmVxdWVzdC5ub2RlSGFzaCB9KTtcbiAgICAgICAgICAgICAgYXdhaXQgbWFrZUVmZmVjdERpc3BhdGNoT25lKHBsdWdpbkVudHJ5LCBiYXNlU2Vzc2lvbiwgYXBwbHlIb3N0RWZmZWN0cykocmVzcG9uc2VBY3Rpb24sIHtcbiAgICAgICAgICAgICAgICBub2RlSGFzaDogcmVxdWVzdC5ub2RlSGFzaCxcbiAgICAgICAgICAgICAgICBvdXRwdXRKc29uLFxuICAgICAgICAgICAgICB9KTtcbiAgICAgICAgICAgIH0gY2F0Y2ggKGVycm9yKSB7XG4gICAgICAgICAgICAgIGNvbnNvbGUud2FybihcIltvcy1zaGVsbF0gcmVxdWVzdFBsdWdpbkV4Y2hhbmdlIGZhaWxlZFwiLCB7IHBsdWdpbklkLCBhcHBJZCwgZXJyb3IgfSk7XG4gICAgICAgICAgICB9XG4gICAgICAgICAgfVxuICAgICAgICAgIGNvbnRpbnVlO1xuICAgICAgICB9XG4gICAgICAgIGlmIChcInNwYXduUGx1Z2luSW5zdGFuY2VcIiBpbiBlZmZlY3QpIHtcbiAgICAgICAgICBjb25zdCB7IHBsdWdpbklkLCBhcHBJZCwgb3NJbnN0YW5jZUlkLCBsYWJlbCwgZG9jdW1lbnRKc29uIH0gPSBlZmZlY3Quc3Bhd25QbHVnaW5JbnN0YW5jZTtcbiAgICAgICAgICBjb25zdCBjdXJyZW50UGFuZWwgPSBwYXJzZVBhbmVsU3RhdGUobmV4dFZpZXdTdGF0ZSkgPz8gYnVpbGRTcGFjZVBhbmVsU3RhdGUoW10sIFtdKTtcbiAgICAgICAgICAvLyDwn6qm77iPIFNlZSBgZXN0YWJsaXNoUHJpbWFyeVNlc3Npb25gJ3MgY29tbWVudCBhYm92ZSDigJQgdGhlIGBtYW5pZmVzdC53b3JrZmxvd3NgIGZhbGxiYWNrIHNvdXJjZSBpcyBkZWFkOyBgY2F0YWxvZ2AgaXMgYGN1cnJlbnRQYW5lbC5wcm9ncmFtc2Agb3IgZW1wdHkuXG4gICAgICAgICAgY29uc3QgY2F0YWxvZyA9IGN1cnJlbnRQYW5lbC5wcm9ncmFtcy5sZW5ndGggPiAwID8gY3VycmVudFBhbmVsLnByb2dyYW1zIDogW107XG4gICAgICAgICAgY29uc3QgcHJvZ3JhbSA9IGNhdGFsb2cuZmluZCgoZW50cnkpID0+IGVudHJ5LnBsdWdpbklkID09PSBwbHVnaW5JZCAmJiBlbnRyeS5hcHBJZCA9PT0gYXBwSWQpID8/IGNhdGFsb2cuZmluZCgoZW50cnkpID0+IGVudHJ5LnBsdWdpbklkID09PSBwbHVnaW5JZCk7XG4gICAgICAgICAgaWYgKHByb2dyYW0pIHtcbiAgICAgICAgICAgIC8vIPCfqp/vuI8gRm9sZCBzcGF3biBpbnRvIGBuZXh0Vmlld1N0YXRlYCDigJQgYSBzZXBhcmF0ZSBTRVRfU0VTU0lPTiB3b3VsZCBiZSBjbG9iYmVyZWQgYnkgdGhlXG4gICAgICAgICAgICAvLyBmaW5hbCB3cml0ZSBiZWxvdyBhbmQgbGVhdmUgdGhlIHNoZWxsIHN0dWNrIG9uIHRoZSBzdHVkaW8gc3VyZmFjZS5cbiAgICAgICAgICAgIGNvbnN0IG5leHRQYW5lbCA9IGF3YWl0IGVuc3VyZVNwYXduZWRQbHVnaW4ocHJvZ3JhbSwgbGFiZWwsIG9zSW5zdGFuY2VJZCwgZG9jdW1lbnRKc29uLCBuZXh0Vmlld1N0YXRlKTtcbiAgICAgICAgICAgIGlmIChuZXh0UGFuZWwpIG5leHRWaWV3U3RhdGUgPSB2aWV3U3RhdGVXaXRoU3BhY2VQYW5lbChuZXh0Vmlld1N0YXRlLCBuZXh0UGFuZWwpO1xuICAgICAgICAgIH1cbiAgICAgICAgICBjb250aW51ZTtcbiAgICAgICAgfVxuICAgICAgICBpZiAoXCJvcGVuUGx1Z2luSW5zdGFuY2VcIiBpbiBlZmZlY3QpIHtcbiAgICAgICAgICBjb25zdCB7IHBsdWdpbklkLCBhcHBJZCwgb3NJbnN0YW5jZUlkIH0gPSBlZmZlY3Qub3BlblBsdWdpbkluc3RhbmNlO1xuICAgICAgICAgIGNvbnN0IGN1cnJlbnRQYW5lbCA9IHBhcnNlUGFuZWxTdGF0ZShuZXh0Vmlld1N0YXRlKSA/PyBidWlsZFNwYWNlUGFuZWxTdGF0ZShbXSwgW10pO1xuICAgICAgICAgIC8vIPCfqqbvuI8gU2VlIGBlc3RhYmxpc2hQcmltYXJ5U2Vzc2lvbmAncyBjb21tZW50IGFib3ZlIOKAlCB0aGUgYG1hbmlmZXN0LndvcmtmbG93c2AgZmFsbGJhY2sgc291cmNlIGlzIGRlYWQ7IGBjYXRhbG9nYCBpcyBgY3VycmVudFBhbmVsLnByb2dyYW1zYCBvciBlbXB0eS5cbiAgICAgICAgICBjb25zdCBjYXRhbG9nID0gY3VycmVudFBhbmVsLnByb2dyYW1zLmxlbmd0aCA+IDAgPyBjdXJyZW50UGFuZWwucHJvZ3JhbXMgOiBbXTtcbiAgICAgICAgICBjb25zdCBwcm9ncmFtID0gY2F0YWxvZy5maW5kKChlbnRyeSkgPT4gZW50cnkucGx1Z2luSWQgPT09IHBsdWdpbklkICYmIGVudHJ5LmFwcElkID09PSBhcHBJZCkgPz8gY2F0YWxvZy5maW5kKChlbnRyeSkgPT4gZW50cnkucGx1Z2luSWQgPT09IHBsdWdpbklkKTtcbiAgICAgICAgICBpZiAocHJvZ3JhbSkge1xuICAgICAgICAgICAgLy8g8J+qn++4jyBGb2xkIGZvY3VzIGludG8gYG5leHRWaWV3U3RhdGVgIHNvIHRoZSBmaW5hbCBTRVRfU0VTU0lPTiBrZWVwcyBgYWN0aXZlU3Bhd25lZElkYFxuICAgICAgICAgICAgLy8gKG9wZW5pbmcgYSB3b3JrZmxvdyBub2RlIGRlcGVuZHMgb24gdGhpcyDigJQgb3RoZXJ3aXNlIG5vdGhpbmcgYXBwZWFycyB0byBoYXBwZW4pLlxuICAgICAgICAgICAgY29uc3QgbmV4dFBhbmVsID0gYXdhaXQgZW5zdXJlU3Bhd25lZFBsdWdpbihwcm9ncmFtLCB1bmRlZmluZWQsIG9zSW5zdGFuY2VJZCwgdW5kZWZpbmVkLCBuZXh0Vmlld1N0YXRlKTtcbiAgICAgICAgICAgIGlmIChuZXh0UGFuZWwpIHtcbiAgICAgICAgICAgICAgbmV4dFZpZXdTdGF0ZSA9IHZpZXdTdGF0ZVdpdGhTcGFjZVBhbmVsKG5leHRWaWV3U3RhdGUsIG5leHRQYW5lbCk7XG4gICAgICAgICAgICAgIGNvbnNvbGUubG9nKFwiW0RFQlVHXSBvcGVuUGx1Z2luSW5zdGFuY2UgZm9jdXNlZCBzcGF3bmVkIGFwcFwiLCB7XG4gICAgICAgICAgICAgICAgcGx1Z2luSWQsXG4gICAgICAgICAgICAgICAgYXBwSWQsXG4gICAgICAgICAgICAgICAgb3NJbnN0YW5jZUlkLFxuICAgICAgICAgICAgICAgIGFjdGl2ZVNwYXduZWRJZDogbmV4dFBhbmVsLmFjdGl2ZVNwYXduZWRJZCxcbiAgICAgICAgICAgICAgICBzcGF3bmVkQ291bnQ6IG5leHRQYW5lbC5zcGF3bmVkQXBwcy5sZW5ndGgsXG4gICAgICAgICAgICAgIH0pO1xuICAgICAgICAgICAgfVxuICAgICAgICAgICAgaWYgKG9zSW5zdGFuY2VJZCAmJiBvcGVuU3BhY2VJZFJlZi5jdXJyZW50KSB7XG4gICAgICAgICAgICAgIG9wZW5JbnN0YW5jZUlkUmVmLmN1cnJlbnQgPSBvc0luc3RhbmNlSWQ7XG4gICAgICAgICAgICAgIG5hdmlnYXRlSGlzdG9yeShgL3NwYWNlcy8ke29wZW5TcGFjZUlkUmVmLmN1cnJlbnR9L2luc3RhbmNlcy8ke29zSW5zdGFuY2VJZH1gKTtcbiAgICAgICAgICAgIH1cbiAgICAgICAgICB9IGVsc2Uge1xuICAgICAgICAgICAgY29uc29sZS53YXJuKFxuICAgICAgICAgICAgICBcIltvcy1zaGVsbF0gb3BlblBsdWdpbkluc3RhbmNlOiBubyBwcm9ncmFtIG1hdGNoZXNcIixcbiAgICAgICAgICAgICAgeyBwbHVnaW5JZCwgYXBwSWQgfSxcbiAgICAgICAgICAgICAgXCJhdmFpbGFibGU6XCIsXG4gICAgICAgICAgICAgIGNhdGFsb2cubWFwKChlbnRyeSkgPT4gYCR7ZW50cnkucGx1Z2luSWR9LyR7ZW50cnkuYXBwSWR9YCksXG4gICAgICAgICAgICApO1xuICAgICAgICAgIH1cbiAgICAgICAgICBjb250aW51ZTtcbiAgICAgICAgfVxuICAgICAgfVxuICAgICAgY29uc3QgbmV4dFNlc3Npb24gPSB7IC4uLmJhc2VTZXNzaW9uLCB2aWV3U3RhdGU6IG5leHRWaWV3U3RhdGUgfTtcbiAgICAgIGNvbnN0IGlzU3Bhd25lZFBsdWdpblNlc3Npb24gPSBzdHVkaW9Nb2RlICYmIHNlc3Npb24gJiYgYmFzZVNlc3Npb24ucGx1Z2luSWQgIT09IHNlc3Npb24ucGx1Z2luSWQ7XG4gICAgICBkaXNwYXRjaCh7XG4gICAgICAgIHR5cGU6IFwiU0VUX1NFU1NJT05cIixcbiAgICAgICAgdmFsdWU6IChjdXJyZW50KSA9PiB7XG4gICAgICAgICAgaWYgKCFjdXJyZW50KSByZXR1cm4gbmV4dFNlc3Npb247XG4gICAgICAgICAgaWYgKGlzU3Bhd25lZFBsdWdpblNlc3Npb24pIHJldHVybiBjdXJyZW50LnZpZXdTdGF0ZSA9PT0gbmV4dFZpZXdTdGF0ZSA/IGN1cnJlbnQgOiB7IC4uLmN1cnJlbnQsIHZpZXdTdGF0ZTogbmV4dFZpZXdTdGF0ZSB9O1xuICAgICAgICAgIGlmIChjdXJyZW50Lmluc3RhbmNlSWQgIT09IG5leHRTZXNzaW9uLmluc3RhbmNlSWQpIHJldHVybiBjdXJyZW50O1xuICAgICAgICAgIC8vIPCfkKLvuI8gUHJlc2VydmUgYGN1cnJlbnRgJ3MgaWRlbnRpdHkgd2hlbiB0aGUgdmlld1N0YXRlIGRpZG4ndCBhY3R1YWxseSBjaGFuZ2Ug4oCUIG90aGVyd2lzZSBldmVyeVxuICAgICAgICAgIC8vIGFjdGlvbiBtaW50cyBhIG5ldyBgc2Vzc2lvbmAgb2JqZWN0LCB3aGljaCBjYXNjYWRlcyBpbnRvIGEgbmV3IGBvbkFjdGlvbmAgaWRlbnRpdHksIHdoaWNoXG4gICAgICAgICAgLy8gYnVzdHMgZXZlcnkgbWVtbyBrZXllZCBvbiBpdCAod2luZG93cywgcGFuZWxzLCB0aGUgYm9vdC1yZWZyZXNoIGVmZmVjdCBiZWxvdykgZXZlbiB3aGVuXG4gICAgICAgICAgLy8gbm90aGluZyBhYm91dCB0aGUgc2Vzc2lvbiBjaGFuZ2VkLlxuICAgICAgICAgIHJldHVybiBjdXJyZW50LnZpZXdTdGF0ZSA9PT0gbmV4dFZpZXdTdGF0ZSA/IGN1cnJlbnQgOiB7IC4uLmN1cnJlbnQsIHZpZXdTdGF0ZTogbmV4dFZpZXdTdGF0ZSB9O1xuICAgICAgICB9LFxuICAgICAgfSk7XG4gICAgICBpZiAoaXNTcGF3bmVkUGx1Z2luU2Vzc2lvbikge1xuICAgICAgICBjb25zdCBzcGF3bmVkID0gcGFyc2VQYW5lbFN0YXRlKG5leHRWaWV3U3RhdGUpPy5zcGF3bmVkQXBwcy5maW5kKChlbnRyeSkgPT4gZW50cnkucGx1Z2luSWQgPT09IGJhc2VTZXNzaW9uLnBsdWdpbklkICYmIGVudHJ5Lmluc3RhbmNlSWQgPT09IGJhc2VTZXNzaW9uLmluc3RhbmNlSWQpO1xuICAgICAgICBpZiAoc3Bhd25lZCkgYXdhaXQgcmVmcmVzaFNwYXduZWRVaShzcGF3bmVkLCBuZXh0Vmlld1N0YXRlLCB1aVNjb3BlKTtcbiAgICAgIH0gZWxzZSBpZiAoc2Vzc2lvbj8uaW5zdGFuY2VJZCA9PT0gbmV4dFNlc3Npb24uaW5zdGFuY2VJZCB8fCBiYXNlU2Vzc2lvbi5pbnN0YW5jZUlkID09PSBuZXh0U2Vzc2lvbi5pbnN0YW5jZUlkKSB7XG4gICAgICAgIGF3YWl0IHJlZnJlc2hVaShuZXh0U2Vzc2lvbiwgdWlTY29wZSk7XG4gICAgICB9XG4gICAgfSxcbiAgICBbY2xlYXJBbGxXaW5kb3dVdGlsaXRpZXMsIGVuc3VyZVNwYXduZWRQbHVnaW4sIGxvYWRlZFBsdWdpbnMsIG5hdmlnYXRlSGlzdG9yeSwgcmVmcmVzaFNwYXduZWRVaSwgcmVmcmVzaFVpLCBzZXNzaW9uLCBzZXRBY3RpdmVVdGlsaXR5Rm9yV2luZG93LCBzdHVkaW9Nb2RlXSxcbiAgKTtcblxuICBjb25zdCBhcHBseVNoZWxsVXJpID0gdXNlQ2FsbGJhY2soXG4gICAgYXN5bmMgKHVyaTogc3RyaW5nLCBwcmVzZXJ2ZWRWaWV3U3RhdGU/OiBWaWV3TW9kZWwpID0+IHtcbiAgICAgIGNvbnN0IGN1cnJlbnRTZXNzaW9uID0gc2Vzc2lvblJlZi5jdXJyZW50O1xuICAgICAgaWYgKCFob3N0Q29uZmlnIHx8ICFjdXJyZW50U2Vzc2lvbiB8fCBsb2FkZWRQbHVnaW5zLmxlbmd0aCA9PT0gMCkgcmV0dXJuO1xuICAgICAgY29uc3QgcGF0aCA9IHVyaS5zcGxpdChcIj9cIilbMF0gPz8gXCIvXCI7XG4gICAgICBjb25zdCByb3V0ZSA9IHBhcnNlU2hlbGxSb3V0ZShwYXRoKTtcbiAgICAgIGNvbnN0IHNQbHVnaW4gPSBsb2FkZWRQbHVnaW5zLmZpbmQoKGVudHJ5KSA9PiBlbnRyeS5oYW5kbGUucGx1Z2luSWQgPT09IGhvc3RDb25maWcucGx1Z2luSWQpPy5oYW5kbGU7XG4gICAgICBpZiAoIXNQbHVnaW4pIHJldHVybjtcbiAgICAgIGlmIChyb3V0ZS5raW5kID09PSBcImxhbmRpbmdcIikge1xuICAgICAgICBvcGVuU3BhY2VJZFJlZi5jdXJyZW50ID0gbnVsbDtcbiAgICAgICAgb3Blbkluc3RhbmNlSWRSZWYuY3VycmVudCA9IG51bGw7XG4gICAgICAgIGlmIChjdXJyZW50U2Vzc2lvbi5hcHAuaWQgIT09IGhvc3RDb25maWcubGFuZGluZ0FwcElkKSBhd2FpdCBzd2l0Y2hUb01hbmFnZWRBcHAoaG9zdENvbmZpZy5sYW5kaW5nQXBwSWQsIHByZXNlcnZlZFZpZXdTdGF0ZSk7XG4gICAgICAgIHJldHVybjtcbiAgICAgIH1cbiAgICAgIGlmIChyb3V0ZS5raW5kID09PSBcIm5vdEZvdW5kXCIpIHtcbiAgICAgICAgb3BlblNwYWNlSWRSZWYuY3VycmVudCA9IG51bGw7XG4gICAgICAgIG9wZW5JbnN0YW5jZUlkUmVmLmN1cnJlbnQgPSBudWxsO1xuICAgICAgICByZXR1cm47XG4gICAgICB9XG4gICAgICBjb25zdCB7IHNwYWNlSWQsIGluc3RhbmNlSWQgfSA9IHJvdXRlO1xuICAgICAgLy8g8J+nre+4jyBQaW4gdGhlIHJvdXRlIHN0dWRpbyBpZCBiZWZvcmUgdGhlIGFzeW5jIGFwcCBzd2l0Y2ggc28gdGhlIGJvb3QgZXhhbXBsZSBlZmZlY3QgY2Fubm90XG4gICAgICAvLyByYWNlLW5hdmlnYXRlIHRvIGAvc3BhY2VzL2RlbW9gIHdoaWxlIGBzd2l0Y2hUb01hbmFnZWRBcHBgIGlzIHN0aWxsIGF3YWl0aW5nLlxuICAgICAgY29uc3Qgc3R1ZGlvQ2hhbmdlZCA9IG9wZW5TcGFjZUlkUmVmLmN1cnJlbnQgIT09IHNwYWNlSWQ7XG4gICAgICBvcGVuU3BhY2VJZFJlZi5jdXJyZW50ID0gc3BhY2VJZDtcbiAgICAgIGNvbnN0IHN0dWRpb1Nlc3Npb24gPSBjdXJyZW50U2Vzc2lvbi5hcHAuaWQgPT09IGhvc3RDb25maWcuaG9zdEFwcElkID8gY3VycmVudFNlc3Npb24gOiBhd2FpdCBzd2l0Y2hUb01hbmFnZWRBcHAoaG9zdENvbmZpZy5ob3N0QXBwSWQsIHByZXNlcnZlZFZpZXdTdGF0ZSk7XG4gICAgICBpZiAoIXN0dWRpb1Nlc3Npb24pIHJldHVybjtcbiAgICAgIGNvbnN0IHN0dWRpb0NvbnRyb2xsZXJJZCA9IHN0dWRpb1Nlc3Npb24uYXBwLmNvbnRyb2xsZXJJZDtcbiAgICAgIGlmIChzdHVkaW9DaGFuZ2VkKSB7XG4gICAgICAgIG9wZW5JbnN0YW5jZUlkUmVmLmN1cnJlbnQgPSBudWxsO1xuICAgICAgICBjb25zb2xlLmxvZyhcIltERUJVR10gYXBwbHlTaGVsbFVyaSBvcGVuU3BhY2VcIiwgc3BhY2VJZCk7XG4gICAgICAgIGNvbnN0IG9wZW5SZXNwb25zZSA9IGF3YWl0IHNQbHVnaW4uaGFuZGxlQWN0aW9uKHN0dWRpb1Nlc3Npb24uaW5zdGFuY2VJZCwgZW5jb2RlQWN0aW9uV2lyZSh7IGNvbnRyb2xsZXJJZDogc3R1ZGlvQ29udHJvbGxlcklkLCBhY3Rpb246IFwib3BlblNwYWNlXCIsIGFyZ3M6IHsgc3BhY2VJZCB9IH0pLCBzdHVkaW9TZXNzaW9uLnZpZXdTdGF0ZSk7XG4gICAgICAgIGF3YWl0IGFwcGx5SG9zdEVmZmVjdHMob3BlblJlc3BvbnNlLnJlcXVlc3RlZEVmZmVjdHMgPz8gW10sIHN0dWRpb1Nlc3Npb24sIHJlc29sdmVVaURpcnR5U2NvcGUob3BlblJlc3BvbnNlLnVpU2NvcGUpKTtcbiAgICAgIH1cbiAgICAgIGlmIChvcGVuSW5zdGFuY2VJZFJlZi5jdXJyZW50ID09PSAoaW5zdGFuY2VJZCA/PyBudWxsKSkgcmV0dXJuO1xuICAgICAgb3Blbkluc3RhbmNlSWRSZWYuY3VycmVudCA9IGluc3RhbmNlSWQgPz8gbnVsbDtcbiAgICAgIGlmIChpbnN0YW5jZUlkKSB7XG4gICAgICAgIGNvbnN0IHJlc3BvbnNlID0gYXdhaXQgc1BsdWdpbi5oYW5kbGVBY3Rpb24oc3R1ZGlvU2Vzc2lvbi5pbnN0YW5jZUlkLCBlbmNvZGVBY3Rpb25XaXJlKHsgY29udHJvbGxlcklkOiBzdHVkaW9Db250cm9sbGVySWQsIGFjdGlvbjogXCJvcGVuSW5zdGFuY2VcIiwgYXJnczogeyBpbnN0YW5jZUlkIH0gfSksIHN0dWRpb1Nlc3Npb24udmlld1N0YXRlKTtcbiAgICAgICAgYXdhaXQgYXBwbHlIb3N0RWZmZWN0cyhyZXNwb25zZS5yZXF1ZXN0ZWRFZmZlY3RzID8/IFtdLCBzdHVkaW9TZXNzaW9uLCByZXNvbHZlVWlEaXJ0eVNjb3BlKHJlc3BvbnNlLnVpU2NvcGUpKTtcbiAgICAgIH0gZWxzZSB7XG4gICAgICAgIGNvbnN0IHJlc3BvbnNlID0gYXdhaXQgc1BsdWdpbi5oYW5kbGVBY3Rpb24oc3R1ZGlvU2Vzc2lvbi5pbnN0YW5jZUlkLCBlbmNvZGVBY3Rpb25XaXJlKHsgY29udHJvbGxlcklkOiBzdHVkaW9Db250cm9sbGVySWQsIGFjdGlvbjogXCJjbG9zZUZvY3VzZWRJbnN0YW5jZVwiIH0pLCBzdHVkaW9TZXNzaW9uLnZpZXdTdGF0ZSk7XG4gICAgICAgIGNvbnN0IGN1cnJlbnRQYW5lbCA9IHBhcnNlUGFuZWxTdGF0ZShzdHVkaW9TZXNzaW9uLnZpZXdTdGF0ZSkgPz8gYnVpbGRTcGFjZVBhbmVsU3RhdGUoW10sIFtdKTtcbiAgICAgICAgdXBkYXRlU3BhY2VQYW5lbChidWlsZFNwYWNlUGFuZWxTdGF0ZShjdXJyZW50UGFuZWwucHJvZ3JhbXMsIGN1cnJlbnRQYW5lbC5zcGF3bmVkQXBwcywgY3VycmVudFBhbmVsLmFjdGl2ZVBhbmVsVGFiLCB1bmRlZmluZWQpKTtcbiAgICAgICAgYXdhaXQgYXBwbHlIb3N0RWZmZWN0cyhyZXNwb25zZS5yZXF1ZXN0ZWRFZmZlY3RzID8/IFtdLCBzdHVkaW9TZXNzaW9uLCByZXNvbHZlVWlEaXJ0eVNjb3BlKHJlc3BvbnNlLnVpU2NvcGUpKTtcbiAgICAgIH1cbiAgICB9LFxuICAgIFthcHBseUhvc3RFZmZlY3RzLCBsb2FkZWRQbHVnaW5zLCByZWZyZXNoVWksIGhvc3RDb25maWcsIHN3aXRjaFRvTWFuYWdlZEFwcCwgdXBkYXRlU3BhY2VQYW5lbF0sXG4gICk7XG5cbiAgdXNlRWZmZWN0KCgpID0+IHtcbiAgICBpZiAoIXN0dWRpb01vZGUgfHwgbG9hZGVkUGx1Z2lucy5sZW5ndGggPT09IDApIHJldHVybjtcbiAgICB2b2lkIGFwcGx5U2hlbGxVcmkoc2hlbGxVcmkpLmNhdGNoKCh1cmlFcnJvcikgPT4ge1xuICAgICAgY29uc29sZS5lcnJvcihcIltERUJVR10gc2hlbGwgdXJpIGFwcGx5IGZhaWxlZFwiLCB1cmlFcnJvcik7XG4gICAgfSk7XG4gIH0sIFthcHBseVNoZWxsVXJpLCBsb2FkZWRQbHVnaW5zLmxlbmd0aCwgc2hlbGxVcmksIHN0dWRpb01vZGVdKTtcblxuICBjb25zdCByZXNvbHZlU3luY1RhcmdldFNlc3Npb24gPSB1c2VDYWxsYmFjaygoKTogQWN0aXZlU2Vzc2lvbiB8IG51bGwgPT4ge1xuICAgIGlmICghc2Vzc2lvbikgcmV0dXJuIG51bGw7XG4gICAgaWYgKHN0dWRpb01vZGUgJiYgcGFuZWw/LmFjdGl2ZVNwYXduZWRJZCkge1xuICAgICAgY29uc3Qgc3Bhd25lZCA9IHBhbmVsLnNwYXduZWRBcHBzLmZpbmQoKGVudHJ5KSA9PiBlbnRyeS5pZCA9PT0gcGFuZWwuYWN0aXZlU3Bhd25lZElkKTtcbiAgICAgIGlmIChzcGF3bmVkKSB7XG4gICAgICAgIGNvbnN0IGFwcCA9IGxvYWRlZFBsdWdpbnMuZmluZCgoZW50cnkpID0+IGVudHJ5LmhhbmRsZS5wbHVnaW5JZCA9PT0gc3Bhd25lZC5wbHVnaW5JZCk/Lm1hbmlmZXN0LmFwcHMuZmluZCgoY2FuZGlkYXRlKSA9PiBjYW5kaWRhdGUuaWQgPT09IHNwYXduZWQuYXBwSWQpO1xuICAgICAgICBpZiAoYXBwKSByZXR1cm4geyBwbHVnaW5JZDogc3Bhd25lZC5wbHVnaW5JZCwgaW5zdGFuY2VJZDogc3Bhd25lZC5pbnN0YW5jZUlkLCBhcHAsIHZpZXdTdGF0ZTogc2Vzc2lvbi52aWV3U3RhdGUgfTtcbiAgICAgIH1cbiAgICB9XG4gICAgcmV0dXJuIHNlc3Npb247XG4gIH0sIFtsb2FkZWRQbHVnaW5zLCBwYW5lbCwgc2Vzc2lvbiwgc3R1ZGlvTW9kZV0pO1xuXG4gIC8qKlxuICAgKiDwn6e177iPIGBvcGVuRG9jdW1lbnQocmVmLCBiaW5kaW5ncylgIOKAlCByZXBsYWNlcyBgYXR0YWNoU3luY0JhY2tib25lYCdzIFVSSS1zdHJpbmcgbWlycm9yLiBTcGlucyB1cCAob3JcbiAgICogcmV1c2VzKSBg8J+fpu+4j2JhY2tib25lLfCfn6bvuI93b3JrZXIudHNgLCB0ZWxscyBpdCB0byBvcGVuIHRoZSBkb2N1bWVudCwgc3Vic2NyaWJlcyB0byBpdHMgcG9zdE1lc3NhZ2UgZXZlbnRzLFxuICAgKiBhbmQgY2FsbHMgdGhlIHBsdWdpbiBpbnN0YW5jZSdzIGBhdHRhY2hCYWNrYm9uZWAvYGxvYWRBcHBEb2N1bWVudGAgV0lULWV4cG9ydGVkIG1ldGhvZHMgKFdTLUQpIHNvXG4gICAqIHRoZSBwbHVnaW4tc2lkZSBzdG9yZSBzdGFydHMgcHVtcGluZyB0aHJvdWdoIHRoZSBzYW1lIGxvZ2ljYWwgY2hhbm5lbC4gVGhlIGBhY3RvcjovLzxkb2N1bWVudElkPmBcbiAgICogdXJpIG1pcnJvcnMgYGZyYW1ld29yay9zeW5jYCdzIGBDaGFubmVsQmFja2JvbmU6OnBhaXJgIGNvbnZlbnRpb24gb24gdGhlIFJ1c3Qgc2lkZS5cbiAgICpcbiAgICogRnVsbCBsb29wIG5vdGU6IHRoaXMgd2lyZXMgdGhlIG1haW4tdGhyZWFkIGhhbGYgb2YgdGhlIGNvbnRyYWN0LiBUaGUgcmVtYWluaW5nIGhvcCDigJQgdGhlXG4gICAqIHNhbmRib3hlZCBwbHVnaW4ncyBvd24gYGJhY2tib25lLXNlbmRgL2BiYWNrYm9uZS1wb2xsYCBXSVQgaG9zdC1pbXBvcnQgY2FsbHMgcmVsYXlpbmcgdGhyb3VnaCBpdHNcbiAgICogZGVkaWNhdGVkIHByb2dyYW0gd29ya2VyLCB0aHJvdWdoIHRoaXMgbWFpbiB0aHJlYWQsIGludG8gYPCfn6bvuI9iYWNrYm9uZS3wn5+m77iPd29ya2VyLnRzYCDigJQgaXNcbiAgICogYGZyYW1ld29yay9vcy9kZXYvc2NyaXB0LnRzYCdzIGBwbHVnaW5Xb3JrZXJTb3VyY2VgIHJlc3BvbnNpYmlsaXR5IChkZXYgd29ya2Zsb3csIGRlZmVycmVkXG4gICAqIHBlciB0aGlzIHNlc3Npb24ncyBwcmlvcml0eSBvcmRlciBpZiBub3Qgb3RoZXJ3aXNlIGNvbXBsZXRlZCk7IHNlZSB0aGF0IGZpbGUncyBvd24gbm90ZXMuXG4gICAqL1xuICBjb25zdCBvcGVuRG9jdW1lbnQgPSB1c2VDYWxsYmFjayhcbiAgICBhc3luYyAocmVmOiB7IHJlYWRvbmx5IGRvY3VtZW50SWQ6IHN0cmluZzsgcmVhZG9ubHkgc2NoZW1hOiBzdHJpbmcgfSwgYmluZGluZ3M6IHJlYWRvbmx5IFBlcnNpc3RlbmNlQmluZGluZ1tdKSA9PiB7XG4gICAgICBjb25zdCB0YXJnZXRTZXNzaW9uID0gcmVzb2x2ZVN5bmNUYXJnZXRTZXNzaW9uKCk7XG4gICAgICBpZiAoIXRhcmdldFNlc3Npb24pIHJldHVybjtcbiAgICAgIGNvbnN0IHBsdWdpbiA9IGxvYWRlZFBsdWdpbnMuZmluZCgoZW50cnkpID0+IGVudHJ5LmhhbmRsZS5wbHVnaW5JZCA9PT0gdGFyZ2V0U2Vzc2lvbi5wbHVnaW5JZCk/LmhhbmRsZTtcbiAgICAgIGlmICghcGx1Z2luKSByZXR1cm47XG4gICAgICBjb25zdCB3b3JrZXIgPSBlbnN1cmVCYWNrYm9uZVdvcmtlcigpO1xuICAgICAgb3BlbkRvY3VtZW50U2Vzc2lvbnNSZWYuY3VycmVudC5zZXQocmVmLmRvY3VtZW50SWQsIHsgc2Vzc2lvbjogdGFyZ2V0U2Vzc2lvbiwgcGx1Z2luIH0pO1xuICAgICAgLy8g8J+Qmu+4jyBSZWdpc3RlcnMgVEhJUyBzaGVsbCBhcyB0aGUgcm91dGUgZm9yIHRoaXMgZG9jdW1lbnQncyBvdXRib3VuZCBiYWNrYm9uZSBieXRlcyBiZWZvcmUgdGhlXG4gICAgICAvLyBwbHVnaW4gY2FuIHBvc3NpYmx5IGVtaXQgYW55IChhdHRhY2hCYWNrYm9uZSBiZWxvdykg4oCUIHNlZSBgcmVsYXlQbHVnaW5CYWNrYm9uZU1lc3NhZ2VgJ3MgZG9jLlxuICAgICAgcGx1Z2luQmFja2JvbmVSb3V0ZVVucmVnaXN0ZXJzUmVmLmN1cnJlbnQuZ2V0KHJlZi5kb2N1bWVudElkKT8uKCk7XG4gICAgICBwbHVnaW5CYWNrYm9uZVJvdXRlVW5yZWdpc3RlcnNSZWYuY3VycmVudC5zZXQocmVmLmRvY3VtZW50SWQsIHJlZ2lzdGVyUGx1Z2luQmFja2JvbmVSb3V0ZShyZWYuZG9jdW1lbnRJZCwgcmVsYXlQbHVnaW5CYWNrYm9uZU1lc3NhZ2UpKTtcbiAgICAgIGNvbnN0IHJlcXVlc3Q6IEJhY2tib25lV29ya2VyUmVxdWVzdCA9IHtcbiAgICAgICAga2luZDogXCJvcGVuXCIsXG4gICAgICAgIGRvY3VtZW50SWQ6IHJlZi5kb2N1bWVudElkLFxuICAgICAgICBzY2hlbWE6IHJlZi5zY2hlbWEsXG4gICAgICAgIGJpbmRpbmdzLFxuICAgICAgICB3YXRjaEV4dGVybmFsOiB0cnVlLFxuICAgICAgICBhY3Rvcjogc2hlbGxBY3RvcklkUmVmLmN1cnJlbnQsXG4gICAgICB9O1xuICAgICAgd29ya2VyLnBvc3RNZXNzYWdlKHJlcXVlc3QpO1xuICAgICAgY29uc3QgdXJpID0gYGFjdG9yOi8vJHtyZWYuZG9jdW1lbnRJZH1gO1xuICAgICAgaWYgKHBsdWdpbi5hdHRhY2hCYWNrYm9uZSkgYXdhaXQgcGx1Z2luLmF0dGFjaEJhY2tib25lKHRhcmdldFNlc3Npb24uaW5zdGFuY2VJZCwgdXJpKTtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfU1lOQ19CQUNLQk9ORV9VUklcIiwgdmFsdWU6IHVyaSB9KTtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfU1lOQ19DQVJEX0tJTkRcIiwgdmFsdWU6IG51bGwgfSk7XG4gICAgfSxcbiAgICBbbG9hZGVkUGx1Z2lucywgcmVsYXlQbHVnaW5CYWNrYm9uZU1lc3NhZ2UsIHJlc29sdmVTeW5jVGFyZ2V0U2Vzc2lvbl0sXG4gICk7XG5cbiAgY29uc3QgY2xvc2VEb2N1bWVudCA9IHVzZUNhbGxiYWNrKChkb2N1bWVudElkOiBzdHJpbmcpID0+IHtcbiAgICBjb25zdCBlbnRyeSA9IG9wZW5Eb2N1bWVudFNlc3Npb25zUmVmLmN1cnJlbnQuZ2V0KGRvY3VtZW50SWQpO1xuICAgIGlmIChlbnRyeT8ucGx1Z2luLmRldGFjaEJhY2tib25lKSB2b2lkIGVudHJ5LnBsdWdpbi5kZXRhY2hCYWNrYm9uZShlbnRyeS5zZXNzaW9uLmluc3RhbmNlSWQpO1xuICAgIG9wZW5Eb2N1bWVudFNlc3Npb25zUmVmLmN1cnJlbnQuZGVsZXRlKGRvY3VtZW50SWQpO1xuICAgIHBsdWdpbkJhY2tib25lUm91dGVVbnJlZ2lzdGVyc1JlZi5jdXJyZW50LmdldChkb2N1bWVudElkKT8uKCk7XG4gICAgcGx1Z2luQmFja2JvbmVSb3V0ZVVucmVnaXN0ZXJzUmVmLmN1cnJlbnQuZGVsZXRlKGRvY3VtZW50SWQpO1xuICAgIGNvbnN0IHJlcXVlc3Q6IEJhY2tib25lV29ya2VyUmVxdWVzdCA9IHsga2luZDogXCJjbG9zZVwiLCBkb2N1bWVudElkIH07XG4gICAgYmFja2JvbmVXb3JrZXJSZWYuY3VycmVudD8ucG9zdE1lc3NhZ2UocmVxdWVzdCk7XG4gIH0sIFtdKTtcblxuICAvKiogQGRlcHJlY2F0ZWQgc3VwZXJzZWRlZCBieSB7QGxpbmsgb3BlbkRvY3VtZW50fTsga2VwdCBhcyBhIHRoaW4gVVJJLXBhcnNpbmcgYWRhcHRlciBvbmx5IGZvciB0aGVcbiAgICogZXhpc3Rpbmcgc3luYy1jYXJkIFVJIChgb25BY3Rpb25gJ3MgYGF0dGFjaGAgaGFuZGxlciBiZWxvdyksIHdoaWNoIHN0aWxsIGNvbGxlY3RzIGEgc2luZ2xlIHVyaVxuICAgKiBmcm9tIGZpbGUvZm9sZGVyL3JlbW90ZSBwaWNrZXJzIOKAlCB0cmFuc2xhdGVzIHRoYXQgdXJpIGludG8gYW4gYE9zRG9jdW1lbnRSZWZgICsgYFBlcnNpc3RlbmNlQmluZGluZ2AuICovXG4gIGNvbnN0IGF0dGFjaFN5bmNCYWNrYm9uZSA9IHVzZUNhbGxiYWNrKFxuICAgIGFzeW5jICh1cmk6IHN0cmluZykgPT4ge1xuICAgICAgY29uc3QgdGFyZ2V0U2Vzc2lvbiA9IHJlc29sdmVTeW5jVGFyZ2V0U2Vzc2lvbigpO1xuICAgICAgaWYgKCF0YXJnZXRTZXNzaW9uKSByZXR1cm47XG4gICAgICBjb25zdCBkb2N1bWVudElkID0gc3luY0RvY3VtZW50SWQodGFyZ2V0U2Vzc2lvbiwgcGFuZWwsIHN0dWRpb01vZGUpO1xuICAgICAgY29uc3QgYmluZGluZ3M6IFBlcnNpc3RlbmNlQmluZGluZ1tdID0gdXJpLnN0YXJ0c1dpdGgoXCJyZW1vdGU6Ly9cIilcbiAgICAgICAgPyAoKCkgPT4ge1xuICAgICAgICAgICAgY29uc3QgcmVzdCA9IHVyaS5zbGljZShcInJlbW90ZTovL1wiLmxlbmd0aCk7XG4gICAgICAgICAgICBjb25zdCBzbGFzaCA9IHJlc3QuaW5kZXhPZihcIi9cIik7XG4gICAgICAgICAgICBjb25zdCBiYXNlVXJsID0gc2xhc2ggPiAwID8gYGh0dHA6Ly8ke3Jlc3Quc2xpY2UoMCwgc2xhc2gpfWAgOiBgaHR0cDovLyR7cmVzdH1gO1xuICAgICAgICAgICAgY29uc3Qgc3BhY2VJZCA9IHNsYXNoID4gMCA/IHJlc3Quc2xpY2Uoc2xhc2ggKyAxKSB8fCBcImRlZmF1bHRcIiA6IFwiZGVmYXVsdFwiO1xuICAgICAgICAgICAgcmV0dXJuIFt7IGtpbmQ6IFwiaHViXCIsIGJhc2VVcmwsIHNwYWNlSWQgfV07XG4gICAgICAgICAgfSkoKVxuICAgICAgICA6IHVyaS5zdGFydHNXaXRoKFwiZm9sZGVyOi8vXCIpXG4gICAgICAgICAgPyBbeyBraW5kOiBcImZvbGRlclwiLCBwYXRoOiB1cmkuc2xpY2UoXCJmb2xkZXI6Ly9cIi5sZW5ndGgpIH1dXG4gICAgICAgICAgOiB1cmkuc3RhcnRzV2l0aChcImZpbGU6Ly9cIilcbiAgICAgICAgICAgID8gW3sga2luZDogXCJmb2xkZXJcIiwgcGF0aDogdXJpLnNsaWNlKFwiZmlsZTovL1wiLmxlbmd0aCkucmVwbGFjZSgvXFwvW14vXSokLywgXCJcIikgfV1cbiAgICAgICAgICAgIDogW107XG4gICAgICBhd2FpdCBvcGVuRG9jdW1lbnQoeyBkb2N1bWVudElkLCBzY2hlbWE6IHRhcmdldFNlc3Npb24uYXBwLmRvY3VtZW50LmpvaW4oXCIuXCIpIH0sIGJpbmRpbmdzKTtcbiAgICB9LFxuICAgIFtvcGVuRG9jdW1lbnQsIHBhbmVsLCByZXNvbHZlU3luY1RhcmdldFNlc3Npb24sIHN0dWRpb01vZGVdLFxuICApO1xuXG4gIGNvbnN0IGRldGFjaFN5bmNCYWNrYm9uZSA9IHVzZUNhbGxiYWNrKCgpID0+IHtcbiAgICBpZiAoc3luY0JhY2tib25lVXJpKSBjbG9zZURvY3VtZW50KHN5bmNCYWNrYm9uZVVyaS5yZXBsYWNlKC9eYWN0b3I6XFwvXFwvLywgXCJcIikpO1xuICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfU1lOQ19CQUNLQk9ORV9VUklcIiwgdmFsdWU6IG51bGwgfSk7XG4gICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9TWU5DX0NBUkRfS0lORFwiLCB2YWx1ZTogbnVsbCB9KTtcbiAgfSwgW2Nsb3NlRG9jdW1lbnQsIHN5bmNCYWNrYm9uZVVyaV0pO1xuXG4gIGNvbnN0IHNwYXduUHJvZ3JhbSA9IHVzZUNhbGxiYWNrKFxuICAgIGFzeW5jIChwcm9ncmFtOiBTcGFjZVByb2dyYW1FbnRyeSkgPT4ge1xuICAgICAgY29uc3QgcGx1Z2luRW50cnkgPSBsb2FkZWRQbHVnaW5zLmZpbmQoKGVudHJ5KSA9PiBlbnRyeS5oYW5kbGUucGx1Z2luSWQgPT09IHByb2dyYW0ucGx1Z2luSWQpO1xuICAgICAgaWYgKCFwbHVnaW5FbnRyeSB8fCAhc2Vzc2lvbikgcmV0dXJuO1xuICAgICAgY29uc3QgaW5zdGFuY2VJZCA9IGF3YWl0IHBsdWdpbkVudHJ5LmhhbmRsZS5jcmVhdGVBcHAocHJvZ3JhbS5hcHBJZCk7XG4gICAgICBjb25zdCBjdXJyZW50UGFuZWwgPSBwYXJzZVBhbmVsU3RhdGUoc2Vzc2lvbi52aWV3U3RhdGUpID8/IGJ1aWxkU3BhY2VQYW5lbFN0YXRlKFtdLCBbXSk7XG4gICAgICBjb25zdCBzcGF3bmVkSWQgPSBgJHtwcm9ncmFtLnBsdWdpbklkfS0ke2luc3RhbmNlSWR9YDtcbiAgICAgIHVwZGF0ZVNwYWNlUGFuZWwoXG4gICAgICAgIHN0dWRpb1BhbmVsRm9jdXNpbmdTcGF3bmVkKGN1cnJlbnRQYW5lbCwge1xuICAgICAgICAgIGlkOiBzcGF3bmVkSWQsXG4gICAgICAgICAgcGx1Z2luSWQ6IHByb2dyYW0ucGx1Z2luSWQsXG4gICAgICAgICAgaW5zdGFuY2VJZCxcbiAgICAgICAgICBhcHBJZDogcHJvZ3JhbS5hcHBJZCxcbiAgICAgICAgICBsYWJlbDogcHJvZ3JhbS5sYWJlbCxcbiAgICAgICAgICBkb2N1bWVudDogcHJvZ3JhbS5kb2N1bWVudCxcbiAgICAgICAgfSksXG4gICAgICApO1xuICAgIH0sXG4gICAgW2xvYWRlZFBsdWdpbnMsIHNlc3Npb24sIHVwZGF0ZVNwYWNlUGFuZWxdLFxuICApO1xuXG4gIGNvbnN0IG9uQWN0aW9uID0gdXNlQ2FsbGJhY2soXG4gICAgKGFjdGlvbjogQWN0aW9uRGVzY3JpcHRvcikgPT4ge1xuICAgICAgaWYgKGFjdGlvbi5jb250cm9sbGVySWQgPT09IFwicmVjb3ZlcnlcIikge1xuICAgICAgICBjb25zdCBhcmdzID0gdHlwZW9mIGFjdGlvbi5hcmdzID09PSBcIm9iamVjdFwiICYmIGFjdGlvbi5hcmdzICE9IG51bGwgPyAoYWN0aW9uLmFyZ3MgYXMgeyBwbHVnaW5JZD86IHN0cmluZyB9KSA6IHt9O1xuICAgICAgICBjb25zdCBwbHVnaW5JZCA9IGFyZ3MucGx1Z2luSWQgPz8gcHJpbWFyeVBsdWdpbklkO1xuICAgICAgICBpZiAoIXBsdWdpbklkKSByZXR1cm47XG4gICAgICAgIGlmIChhY3Rpb24uYWN0aW9uID09PSBcInJlY292ZXJ5LnJlc3RhcnRBcHBcIikge1xuICAgICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfUExVR0lOX1NVUEVSVklTT1JcIiwgcGx1Z2luSWQsIHZhbHVlOiBcInJlc3RhcnRpbmdcIiB9KTtcbiAgICAgICAgICB2b2lkIHJlbG9hZFBsdWdpbihwbHVnaW5JZCk7XG4gICAgICAgICAgcmV0dXJuO1xuICAgICAgICB9XG4gICAgICAgIGlmIChhY3Rpb24uYWN0aW9uID09PSBcInJlY292ZXJ5LmRpc2FibGVQbHVnaW5cIikge1xuICAgICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfUExVR0lOX1NVUEVSVklTT1JcIiwgcGx1Z2luSWQsIHZhbHVlOiBcInF1YXJhbnRpbmVkXCIgfSk7XG4gICAgICAgICAgaWYgKHBsdWdpbklkICE9PSBwcmltYXJ5UGx1Z2luSWQpIHZvaWQgdW5pbnN0YWxsUGx1Z2luKHBsdWdpbklkKTtcbiAgICAgICAgICByZXR1cm47XG4gICAgICAgIH1cbiAgICAgICAgaWYgKGFjdGlvbi5hY3Rpb24gPT09IFwicmVjb3Zlcnkuc2hvd0RpYWdub3N0aWNzXCIpIHtcbiAgICAgICAgICBjb25zb2xlLmxvZyhcIltERUJVR10gcmVjb3ZlcnkgZGlhZ25vc3RpY3NcIiwgeyBwbHVnaW5JZCwgc3VwZXJ2aXNvcjogcGx1Z2luU3VwZXJ2aXNvckJ5SWRbcGx1Z2luSWRdIH0pO1xuICAgICAgICAgIHJldHVybjtcbiAgICAgICAgfVxuICAgICAgfVxuXG4gICAgICBpZiAoIXNlc3Npb24pIHJldHVybjtcblxuICAgICAgLy8g8J+Ok++4jyBGaXJzdC1ydW4gd2Fsa3Rocm91Z2ggKG1pcnJvcnMgc2V0QWN0aXZlVXRpbGl0eSBiZWxvdyk6IGZ1bGx5IHNoZWxsLWludGVyY2VwdGVkLCByZXNldHNcbiAgICAgIC8vIHBsYXliYWNrIHRvIHRoZSBmaXJzdCBzdGVwLCBuZXZlciBmb3J3YXJkZWQgdG8gdGhlIHByb2dyYW0uXG4gICAgICBpZiAoYWN0aW9uLmFjdGlvbiA9PT0gU1RBUlRfSU5UUk9EVUNUSU9OX0FDVElPTl9JRCkge1xuICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0lOVFJPRFVDVElPTl9TVEVQXCIsIHZhbHVlOiAwIH0pO1xuICAgICAgICByZXR1cm47XG4gICAgICB9XG5cbiAgICAgIC8vIPCfjqXvuI8gRnVsbHkgc2hlbGwtaW50ZXJjZXB0ZWQsIG1pcnJvcmluZyBgU1RBUlRfSU5UUk9EVUNUSU9OX0FDVElPTl9JRGAgYWJvdmU6IHNhbmRib3hlcyB0aGVcbiAgICAgIC8vIGRvY3VtZW50IGFuZCBzdGFydHMgdHV0b3JpYWwgcGxheWJhY2sgZnJvbSB0PTAgKHJlYWwgd29yayBoYXBwZW5zIGluIGBzdGFydFR1dG9yaWFsUmVmYCwgd2lyZWQgdXBcbiAgICAgIC8vIGJ5IHRoZSBUdXRvcmlhbE9yY2hlc3RyYXRpb24gYmxvY2sgZnVydGhlciBkb3duIHRoaXMgY29tcG9uZW50KS5cbiAgICAgIGlmIChhY3Rpb24uYWN0aW9uID09PSBTVEFSVF9UVVRPUklBTF9BQ1RJT05fSUQpIHtcbiAgICAgICAgY29uc3QgYXJncyA9IHR5cGVvZiBhY3Rpb24uYXJncyA9PT0gXCJvYmplY3RcIiAmJiBhY3Rpb24uYXJncyAhPSBudWxsID8gKGFjdGlvbi5hcmdzIGFzIHsgdHV0b3JpYWxJZD86IHVua25vd24gfSkgOiB7fTtcbiAgICAgICAgaWYgKHR5cGVvZiBhcmdzLnR1dG9yaWFsSWQgPT09IFwic3RyaW5nXCIpIHN0YXJ0VHV0b3JpYWxSZWYuY3VycmVudChhcmdzLnR1dG9yaWFsSWQpO1xuICAgICAgICByZXR1cm47XG4gICAgICB9XG4gICAgICBpZiAoYWN0aW9uLmFjdGlvbiA9PT0gUkVDT1JEX1RVVE9SSUFMX0FDVElPTl9JRCkge1xuICAgICAgICB0b2dnbGVUdXRvcmlhbFJlY29yZGluZ1JlZi5jdXJyZW50KCk7XG4gICAgICAgIHJldHVybjtcbiAgICAgIH1cblxuICAgICAgLy8g8J+Ope+4jyBEZXZpYXRpb24gZGV0ZWN0aW9uOiBhbnkgYWN0aW9uIE5PVCBzdGFtcGVkIGJ5IHRoZSB0dXRvcmlhbCBkaXJlY3Rvci9zZWVrL2NvbnZlcmdlIHBhdGggd2hpbGVcbiAgICAgIC8vIGEgdHV0b3JpYWwgaXMgYWN0aXZlbHkgcGxheWluZyBtZWFucyB0aGUgdXNlciBkaXZlcmdlZCBmcm9tIHRoZSByZWNvcmRpbmcg4oCUIGF1dG8tcGF1c2UgYW5kIGZsYWdcbiAgICAgIC8vIGBkZXZpYXRlZGAgc28gcHJlc3NpbmcgUGxheSBhZ2FpbiBjb252ZXJnZXMgaW5zdGVhZCBvZiByZXN1bWluZyBibGluZGx5IG1pZC1kcmlmdC5cbiAgICAgIGlmICh0dXRvcmlhbFBsYXlpbmdSZWYuY3VycmVudCAmJiAhdHV0b3JpYWxEcml2ZW5SZWYuY3VycmVudCkge1xuICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1RVVE9SSUFMX1BMQVlJTkdcIiwgdmFsdWU6IGZhbHNlIH0pO1xuICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1RVVE9SSUFMX0RFVklBVEVEXCIsIHZhbHVlOiB0cnVlIH0pO1xuICAgICAgfVxuXG4gICAgICAvLyDij7rvuI8gUmVjb3JkZXIgdGFwOiBhbm5vdGF0aW9uYWwtb25seSBjYXB0dXJlIChzZWUgYFR1dG9yaWFsVHJhY2tzLmV2ZW50c2AgZG9jIGNvbW1lbnQpIOKAlCBuZXZlclxuICAgICAgLy8gcmUtZGlzcGF0Y2hlZCBvbiBwbGF5YmFjay4gU2tpcHMgbmF2aWdhdGlvbi9pbnRyb2R1Y3Rpb24vdHV0b3JpYWwtY29udHJvbCBhY3Rpb25zIChub2lzZSwgb3JcbiAgICAgIC8vIG1lYW5pbmdsZXNzIHRvIHJlcGxheSkgYW5kIGFueXRoaW5nIHRoZSBkaXJlY3RvciBpdHNlbGYganVzdCBkaXNwYXRjaGVkLlxuICAgICAgaWYgKHR1dG9yaWFsUmVjb3JkaW5nUmVmLmN1cnJlbnQgJiYgIXR1dG9yaWFsRHJpdmVuUmVmLmN1cnJlbnQpIHtcbiAgICAgICAgaWYgKCFUVVRPUklBTF9SRUNPUkRJTkdfRVhDTFVERURfQUNUSU9OX0lEUy5oYXMoYWN0aW9uLmFjdGlvbikpIHtcbiAgICAgICAgICB0dXRvcmlhbFJlY29yZGVyUmVmLmN1cnJlbnQ/LnJlY29yZEV2ZW50KHsga2luZDogXCJhY3Rpb25cIiwgYWN0aW9uOiBhY3Rpb24uYWN0aW9uLCBhcmdzOiBhY3Rpb24uYXJncyBhcyBSZWNvcmQ8c3RyaW5nLCB1bmtub3duPiB8IHVuZGVmaW5lZCB9KTtcbiAgICAgICAgfVxuICAgICAgfVxuXG4gICAgICAvLyDwn6et77iPIENhbWVyYS1uYXZpZ2F0aW9uIGdlc3R1cmUgcmVwb3J0IGZyb20gYSAzRCB3aW5kb3cncyBgV29ybGRPcmJpdEdhdGVkYCAoc2hlbGwtb25seSwgbmV2ZXJcbiAgICAgIC8vIGZvcndhcmRlZCB0byB0aGUgcHJvZ3JhbSkg4oCUIGNvbXBsZXRlcyBhbnkgcGFuL3pvb20vb3JiaXQgaW50ZXJhY3Rpb24gb2YgdGhlIGFjdGl2ZSBzdGVwIHRoYXRcbiAgICAgIC8vIHRhcmdldHMgdGhlIHdpbmRvdyB0aGUgZ2VzdHVyZSBoYXBwZW5lZCBvbi4gQ2VsZWJyYXRlcyBvbmx5IGB3aW5kb3dJZGAncyBvd24gcGFuZSAodmlhXG4gICAgICAvLyBgd2luZG93RWxlbWVudElkYCwgaXRzIHVuaXF1ZSBwZXItaW5zdGFuY2UgZWxlbWVudCBpZCkg4oCUIG5ldmVyIHRoZSB3aG9sZSB3aW5kb3cta2luZCBhbGlhc1xuICAgICAgLy8gc2VsZWN0b3IsIHdoaWNoIHdvdWxkIGNlbGVicmF0ZSBldmVyeSBvdGhlciBvcGVuIHBhbmUgb2YgdGhhdCBzYW1lIGtpbmQgdG9vIChlLmcuIGEgc3BsaXQgdmlldykuXG4gICAgICBpZiAoYWN0aW9uLmFjdGlvbiA9PT0gTk9URV9XT1JMRF9OQVZJR0FUSU9OX0FDVElPTl9JRCkge1xuICAgICAgICBjb25zdCBhcmdzID0gdHlwZW9mIGFjdGlvbi5hcmdzID09PSBcIm9iamVjdFwiICYmIGFjdGlvbi5hcmdzICE9IG51bGwgPyAoYWN0aW9uLmFyZ3MgYXMgeyB3aW5kb3dJZD86IHVua25vd247IGdlc3R1cmVzPzogdW5rbm93biB9KSA6IHt9O1xuICAgICAgICBjb25zdCB3aW5kb3dJZCA9IHR5cGVvZiBhcmdzLndpbmRvd0lkID09PSBcInN0cmluZ1wiID8gYXJncy53aW5kb3dJZCA6IFwiXCI7XG4gICAgICAgIGNvbnN0IGdlc3R1cmVzID0gQXJyYXkuaXNBcnJheShhcmdzLmdlc3R1cmVzKSA/IChhcmdzLmdlc3R1cmVzIGFzIHJlYWRvbmx5IHN0cmluZ1tdKSA6IFtdO1xuICAgICAgICBpZiAod2luZG93SWQpIHtcbiAgICAgICAgICBjb25zdCB3aW5kb3dLaW5kSWQgPSBzZXNzaW9uV2luZG93SW5zdGFuY2VzKHNlc3Npb24uYXBwLCBleHRyYVdpbmRvd0luc3RhbmNlc1JlZi5jdXJyZW50KS5maW5kKChpbnN0YW5jZSkgPT4gaW5zdGFuY2UuaWQgPT09IHdpbmRvd0lkKT8ud2luZG93S2luZElkID8/IHdpbmRvd0lkO1xuICAgICAgICAgIGZvciAoY29uc3QgZ2VzdHVyZSBvZiBnZXN0dXJlcykge1xuICAgICAgICAgICAgY29tcGxldGVJbnRyb2R1Y3Rpb25JbnRlcmFjdGlvbihcbiAgICAgICAgICAgICAgKGludGVyYWN0aW9uKSA9PiBpbnRlcmFjdGlvbi5vbi5raW5kID09PSBnZXN0dXJlICYmIGludHJvZHVjdGlvblRhcmdldHNXaW5kb3cod2luZG93SWQsIHdpbmRvd0tpbmRJZCwgaW50ZXJhY3Rpb24ub24uaWQpLFxuICAgICAgICAgICAgICB3aW5kb3dFbGVtZW50SWQod2luZG93SWQpLFxuICAgICAgICAgICAgKTtcbiAgICAgICAgICB9XG4gICAgICAgIH1cbiAgICAgICAgcmV0dXJuO1xuICAgICAgfVxuXG4gICAgICAvLyDwn6ew77iPIFV0aWxpdHkgYWN0aXZhdGlvbiAoUDUpOiBob3N0LW93bmVkIHNlc3Npb24gc3RhdGUsIG5ldmVyIGEgZG9jdW1lbnQgb3BlcmF0aW9uLiBSZS1jbGlja2luZyB0aGUgYWN0aXZlXG4gICAgICAvLyB1dGlsaXR5IChvciBhbiBlbXB0eSB1dGlsaXR5SWQpIGRlYWN0aXZhdGVzLiBXZSByZXNvbHZlIHRoZSB0YXJnZXQgd2luZG93IGZyb20gdGhlIGRlc2NyaXB0b3IncyB0YWdnZWRcbiAgICAgIC8vIGB3aW5kb3dJZGAgKHNlZSBgdGFnU2V0QWN0aXZlVXRpbGl0eVdpbmRvd2ApLCBmYWxsaW5nIGJhY2sgdG8gdGhlIGFjdGl2ZSB3aW5kb3csIHVwZGF0ZSB0aGUgc3RvcmUsXG4gICAgICAvLyB0aGVuIGZvcndhcmQgdGhlIHJlc29sdmVkIHV0aWxpdHkgdG8gdGhlIHBsdWdpbiBzbyBpdCBjYW4gY2xlYXIvcHJlcGFyZSBzY3JhdGNoLlxuICAgICAgaWYgKGFjdGlvbi5hY3Rpb24gPT09IFNFVF9BQ1RJVkVfVVRJTElUWV9BQ1RJT05fSUQpIHtcbiAgICAgICAgY29uc3QgYXJncyA9IHR5cGVvZiBhY3Rpb24uYXJncyA9PT0gXCJvYmplY3RcIiAmJiBhY3Rpb24uYXJncyAhPSBudWxsID8gKGFjdGlvbi5hcmdzIGFzIHsgdXRpbGl0eUlkPzogdW5rbm93bjsgd2luZG93SWQ/OiB1bmtub3duIH0pIDoge307XG4gICAgICAgIGNvbnN0IHdpbmRvd0lkID0gdHlwZW9mIGFyZ3Mud2luZG93SWQgPT09IFwic3RyaW5nXCIgJiYgYXJncy53aW5kb3dJZCA/IGFyZ3Mud2luZG93SWQgOiAoYWN0aXZlV2luZG93SWRSZWYuY3VycmVudCA/PyBcIlwiKTtcbiAgICAgICAgaWYgKCF3aW5kb3dJZCkgcmV0dXJuO1xuICAgICAgICBjb25zdCByZXF1ZXN0ZWQgPSB0eXBlb2YgYXJncy51dGlsaXR5SWQgPT09IFwic3RyaW5nXCIgPyBhcmdzLnV0aWxpdHlJZCA6IFwiXCI7XG4gICAgICAgIGNvbnN0IG5leHQgPSByZXNvbHZlVXRpbGl0eUFjdGl2YXRpb24oYWN0aXZlVXRpbGl0eUJ5V2luZG93SWRSZWYuY3VycmVudFt3aW5kb3dJZF0sIHJlcXVlc3RlZCk7XG4gICAgICAgIHNldEFjdGl2ZVV0aWxpdHlGb3JXaW5kb3cod2luZG93SWQsIG5leHQpO1xuICAgICAgICAvLyDwn5ug77iPIEEgdG9vbCBhbmQgYSB3aW5kb3cgdXRpbGl0eSBhcmUgbXV0dWFsbHkgZXhjbHVzaXZlIGludGVyYWN0aW9uIG93bmVycyDigJQgYWN0aXZhdGluZyBhIHJlYWxcbiAgICAgICAgLy8gdXRpbGl0eSBjbGVhcnMgYW55IGFjdGl2ZSBtb2RlLWxldmVsIHRvb2wuXG4gICAgICAgIGlmIChuZXh0ICYmIGFjdGl2ZVRvb2xJZFJlZi5jdXJyZW50KSB7XG4gICAgICAgICAgYWN0aXZlVG9vbElkUmVmLmN1cnJlbnQgPSBudWxsO1xuICAgICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfQUNUSVZFX1RPT0xcIiwgdG9vbElkOiBudWxsIH0pO1xuICAgICAgICB9XG4gICAgICAgIGlmIChuZXh0KSBjb21wbGV0ZUludHJvZHVjdGlvbkludGVyYWN0aW9uKChpbnRlcmFjdGlvbikgPT4gaW50ZXJhY3Rpb24ub24ua2luZCA9PT0gXCJ1dGlsaXR5XCIgJiYgaW50ZXJhY3Rpb24ub24uaWQgPT09IG5leHQpO1xuICAgICAgICBjb25zdCBwbHVnaW5FbnRyeSA9IGZpbmRQbHVnaW5Gb3JBY3Rpb24oYWN0aW9uKTtcbiAgICAgICAgY29uc3QgcHJvZ3JhbSA9IHBsdWdpbkVudHJ5Py5oYW5kbGU7XG4gICAgICAgIGlmIChwbHVnaW4pIHtcbiAgICAgICAgICBjb25zdCB2aWV3U3RhdGU6IFZpZXdNb2RlbCA9IHsgLi4uc2Vzc2lvbi52aWV3U3RhdGUsIGFjdGl2ZVV0aWxpdHlJZDogbmV4dCA/PyB1bmRlZmluZWQsIGFjdGl2ZVRvb2xJZDogbmV4dCA/IHVuZGVmaW5lZCA6IGFjdGl2ZVRvb2xJZFJlZi5jdXJyZW50ID8/IHVuZGVmaW5lZCwgd2luZG93SWQgfTtcbiAgICAgICAgICBjb25zdCBmb3J3YXJkZWQ6IEFjdGlvbkRlc2NyaXB0b3IgPSB7IGNvbnRyb2xsZXJJZDogYWN0aW9uLmNvbnRyb2xsZXJJZCwgYWN0aW9uOiBhY3Rpb24uYWN0aW9uLCBhcmdzOiB7IHV0aWxpdHlJZDogbmV4dCB9IH07XG4gICAgICAgICAgdm9pZCBwcm9ncmFtXG4gICAgICAgICAgICAuaGFuZGxlQWN0aW9uKHNlc3Npb24uaW5zdGFuY2VJZCwgZW5jb2RlQWN0aW9uV2lyZShmb3J3YXJkZWQpLCB2aWV3U3RhdGUpXG4gICAgICAgICAgICAudGhlbigocmVzcG9uc2UpID0+IGFwcGx5SG9zdEVmZmVjdHMocmVzcG9uc2UucmVxdWVzdGVkRWZmZWN0cyA/PyBbXSwgeyAuLi5zZXNzaW9uLCB2aWV3U3RhdGUgfSwgcmVzb2x2ZVVpRGlydHlTY29wZShyZXNwb25zZS51aVNjb3BlKSkpXG4gICAgICAgICAgICAuY2F0Y2goKHV0aWxpdHlFcnJvcikgPT4gY29uc29sZS5lcnJvcihcIltERUJVR10gc2V0QWN0aXZlVXRpbGl0eSBmYWlsZWRcIiwgdXRpbGl0eUVycm9yKSk7XG4gICAgICAgIH1cbiAgICAgICAgcmV0dXJuO1xuICAgICAgfVxuXG4gICAgICAvLyDwn5ug77iPIFRvb2wgYWN0aXZhdGlvbjogaG9zdC1vd25lZCBzZXNzaW9uIHN0YXRlIChtb2RlLXNjb3BlZCwgd2luZG93bGVzcyksIG5ldmVyIGEgZG9jdW1lbnQgb3BlcmF0aW9uLlxuICAgICAgLy8gUmUtY2xpY2tpbmcgdGhlIGFjdGl2ZSB0b29sIChvciBhbiBlbXB0eSB0b29sSWQpIGRlYWN0aXZhdGVzLiBNdXR1YWxseSBleGNsdXNpdmUgd2l0aCBldmVyeVxuICAgICAgLy8gd2luZG93J3MgYWN0aXZlIHV0aWxpdHkg4oCUIGFjdGl2YXRpbmcgYSB0b29sIGNsZWFycyB0aGVtIGFsbCwgbWlycm9yaW5nIGBTRVRfQUNUSVZFX1VUSUxJVFlfQUNUSU9OX0lEYC5cbiAgICAgIGlmIChhY3Rpb24uYWN0aW9uID09PSBTRVRfQUNUSVZFX1RPT0xfQUNUSU9OX0lEKSB7XG4gICAgICAgIGNvbnN0IGFyZ3MgPSB0eXBlb2YgYWN0aW9uLmFyZ3MgPT09IFwib2JqZWN0XCIgJiYgYWN0aW9uLmFyZ3MgIT0gbnVsbCA/IChhY3Rpb24uYXJncyBhcyB7IHRvb2xJZD86IHVua25vd24gfSkgOiB7fTtcbiAgICAgICAgY29uc3QgcmVxdWVzdGVkID0gdHlwZW9mIGFyZ3MudG9vbElkID09PSBcInN0cmluZ1wiID8gYXJncy50b29sSWQgOiBcIlwiO1xuICAgICAgICBjb25zdCBuZXh0ID0gcmVzb2x2ZVV0aWxpdHlBY3RpdmF0aW9uKGFjdGl2ZVRvb2xJZFJlZi5jdXJyZW50LCByZXF1ZXN0ZWQpO1xuICAgICAgICBhY3RpdmVUb29sSWRSZWYuY3VycmVudCA9IG5leHQ7XG4gICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfQUNUSVZFX1RPT0xcIiwgdG9vbElkOiBuZXh0IH0pO1xuICAgICAgICBpZiAobmV4dCkgY2xlYXJBbGxXaW5kb3dVdGlsaXRpZXMoKTtcbiAgICAgICAgaWYgKG5leHQpIGNvbXBsZXRlSW50cm9kdWN0aW9uSW50ZXJhY3Rpb24oKGludGVyYWN0aW9uKSA9PiBpbnRlcmFjdGlvbi5vbi5raW5kID09PSBcInRvb2xcIiAmJiBpbnRlcmFjdGlvbi5vbi5pZCA9PT0gbmV4dCk7XG4gICAgICAgIGNvbnN0IHBsdWdpbkVudHJ5ID0gZmluZFBsdWdpbkZvckFjdGlvbihhY3Rpb24pO1xuICAgICAgICBjb25zdCBwcm9ncmFtID0gcGx1Z2luRW50cnk/LmhhbmRsZTtcbiAgICAgICAgaWYgKHBsdWdpbikge1xuICAgICAgICAgIGNvbnN0IHZpZXdTdGF0ZTogVmlld01vZGVsID0geyAuLi5zZXNzaW9uLnZpZXdTdGF0ZSwgYWN0aXZlVG9vbElkOiBuZXh0ID8/IHVuZGVmaW5lZCwgYWN0aXZlVXRpbGl0eUlkOiBuZXh0ID8gdW5kZWZpbmVkIDogc2Vzc2lvbi52aWV3U3RhdGUuYWN0aXZlVXRpbGl0eUlkIH07XG4gICAgICAgICAgY29uc3QgZm9yd2FyZGVkOiBBY3Rpb25EZXNjcmlwdG9yID0geyBjb250cm9sbGVySWQ6IGFjdGlvbi5jb250cm9sbGVySWQsIGFjdGlvbjogYWN0aW9uLmFjdGlvbiwgYXJnczogeyB0b29sSWQ6IG5leHQgfSB9O1xuICAgICAgICAgIHZvaWQgcHJvZ3JhbVxuICAgICAgICAgICAgLmhhbmRsZUFjdGlvbihzZXNzaW9uLmluc3RhbmNlSWQsIGVuY29kZUFjdGlvbldpcmUoZm9yd2FyZGVkKSwgdmlld1N0YXRlKVxuICAgICAgICAgICAgLnRoZW4oKHJlc3BvbnNlKSA9PiBhcHBseUhvc3RFZmZlY3RzKHJlc3BvbnNlLnJlcXVlc3RlZEVmZmVjdHMgPz8gW10sIHsgLi4uc2Vzc2lvbiwgdmlld1N0YXRlIH0sIHJlc29sdmVVaURpcnR5U2NvcGUocmVzcG9uc2UudWlTY29wZSkpKVxuICAgICAgICAgICAgLmNhdGNoKCh0b29sRXJyb3IpID0+IGNvbnNvbGUuZXJyb3IoXCJbREVCVUddIHNldEFjdGl2ZVRvb2wgZmFpbGVkXCIsIHRvb2xFcnJvcikpO1xuICAgICAgICB9XG4gICAgICAgIHJldHVybjtcbiAgICAgIH1cblxuICAgICAgY29tcGxldGVJbnRyb2R1Y3Rpb25JbnRlcmFjdGlvbigoaW50ZXJhY3Rpb24pID0+IGludGVyYWN0aW9uLm9uLmtpbmQgPT09IFwiYWN0aW9uXCIgJiYgaW50ZXJhY3Rpb24ub24uaWQgPT09IGFjdGlvbi5hY3Rpb24pO1xuXG4gICAgICBpZiAoYWN0aW9uLmNvbnRyb2xsZXJJZCA9PT0gRlJBTUVXT1JLX1NZTkNfQ09OVFJPTExFUl9JRCkge1xuICAgICAgICBpZiAoYWN0aW9uLmFjdGlvbiA9PT0gXCJzZWxlY3RGaWxlXCIpIHtcbiAgICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1NZTkNfQ0FSRF9LSU5EXCIsIHZhbHVlOiBcImZpbGVcIiB9KTtcbiAgICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1NZTkNfRFJBRlRfUEFUSFwiLCB2YWx1ZTogc3luY0JhY2tib25lVXJpPy5zdGFydHNXaXRoKFwiZmlsZTovL1wiKSA/IHN5bmNCYWNrYm9uZVVyaS5zbGljZShcImZpbGU6Ly9cIi5sZW5ndGgpIDogXCJcIiB9KTtcbiAgICAgICAgICByZXR1cm47XG4gICAgICAgIH1cbiAgICAgICAgaWYgKGFjdGlvbi5hY3Rpb24gPT09IFwic2VsZWN0Rm9sZGVyXCIpIHtcbiAgICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1NZTkNfQ0FSRF9LSU5EXCIsIHZhbHVlOiBcImZvbGRlclwiIH0pO1xuICAgICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfU1lOQ19EUkFGVF9QQVRIXCIsIHZhbHVlOiBzeW5jQmFja2JvbmVVcmk/LnN0YXJ0c1dpdGgoXCJmb2xkZXI6Ly9cIikgPyBzeW5jQmFja2JvbmVVcmkuc2xpY2UoXCJmb2xkZXI6Ly9cIi5sZW5ndGgpIDogXCJcIiB9KTtcbiAgICAgICAgICByZXR1cm47XG4gICAgICAgIH1cbiAgICAgICAgaWYgKGFjdGlvbi5hY3Rpb24gPT09IFwic2VsZWN0UmVtb3RlXCIpIHtcbiAgICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1NZTkNfQ0FSRF9LSU5EXCIsIHZhbHVlOiBcInJlbW90ZVwiIH0pO1xuICAgICAgICAgIGNvbnN0IHJlbW90ZSA9IHN5bmNCYWNrYm9uZVVyaT8uc3RhcnRzV2l0aChcInJlbW90ZTovL1wiKSA/IHN5bmNCYWNrYm9uZVVyaS5zbGljZShcInJlbW90ZTovL1wiLmxlbmd0aCkgOiBcIlwiO1xuICAgICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfU1lOQ19EUkFGVF9QQVRIXCIsIHZhbHVlOiByZW1vdGUgfSk7XG4gICAgICAgICAgcmV0dXJuO1xuICAgICAgICB9XG4gICAgICAgIGlmIChhY3Rpb24uYWN0aW9uID09PSBcImF0dGFjaFwiKSB7XG4gICAgICAgICAgY29uc3QgcGF0aCA9IHR5cGVvZiBhY3Rpb24uYXJncyA9PT0gXCJvYmplY3RcIiAmJiBhY3Rpb24uYXJncyAhPSBudWxsICYmIFwicGF0aFwiIGluIGFjdGlvbi5hcmdzID8gU3RyaW5nKChhY3Rpb24uYXJncyBhcyB7IHBhdGg/OiBzdHJpbmcgfSkucGF0aCA/PyBcIlwiKSA6IHN5bmNEcmFmdFBhdGg7XG4gICAgICAgICAgaWYgKCFwYXRoLnRyaW0oKSkgcmV0dXJuO1xuICAgICAgICAgIGNvbnN0IHVyaSA9XG4gICAgICAgICAgICBhY3Rpb24uYXJncyAmJiB0eXBlb2YgYWN0aW9uLmFyZ3MgPT09IFwib2JqZWN0XCIgJiYgXCJraW5kXCIgaW4gYWN0aW9uLmFyZ3NcbiAgICAgICAgICAgICAgPyBTdHJpbmcoKGFjdGlvbi5hcmdzIGFzIHsga2luZD86IHN0cmluZyB9KS5raW5kKSA9PT0gXCJyZW1vdGVcIlxuICAgICAgICAgICAgICAgID8gKCgpID0+IHtcbiAgICAgICAgICAgICAgICAgICAgY29uc3QgW2hvc3RQb3J0LCAuLi5yZXN0XSA9IHBhdGguc3BsaXQoXCIvXCIpO1xuICAgICAgICAgICAgICAgICAgICBjb25zdCBbc3BhY2VJZCwgZG9jdW1lbnRJZF0gPSByZXN0Lmxlbmd0aCA+PSAyID8gW3Jlc3RbMF0sIHJlc3Quc2xpY2UoMSkuam9pbihcIi9cIildIDogW1wiZGVmYXVsdFwiLCByZXN0WzBdIHx8IHN5bmNEb2N1bWVudElkKHNlc3Npb24sIHBhbmVsLCBzdHVkaW9Nb2RlKV07XG4gICAgICAgICAgICAgICAgICAgIHJldHVybiBidWlsZFJlbW90ZUJhY2tib25lVXJpKGhvc3RQb3J0ID8/IFwiMTI3LjAuMC4xOjg3ODdcIiwgc3BhY2VJZCwgZG9jdW1lbnRJZCk7XG4gICAgICAgICAgICAgICAgICB9KSgpXG4gICAgICAgICAgICAgICAgOiBTdHJpbmcoKGFjdGlvbi5hcmdzIGFzIHsga2luZD86IHN0cmluZyB9KS5raW5kKSA9PT0gXCJmb2xkZXJcIlxuICAgICAgICAgICAgICAgICAgPyBidWlsZEZvbGRlckJhY2tib25lVXJpKHBhdGgpXG4gICAgICAgICAgICAgICAgICA6IGJ1aWxkRmlsZUJhY2tib25lVXJpKHBhdGgpXG4gICAgICAgICAgICAgIDogYnVpbGRGaWxlQmFja2JvbmVVcmkocGF0aCk7XG4gICAgICAgICAgdm9pZCBhdHRhY2hTeW5jQmFja2JvbmUodXJpKTtcbiAgICAgICAgICByZXR1cm47XG4gICAgICAgIH1cbiAgICAgICAgaWYgKGFjdGlvbi5hY3Rpb24gPT09IFwiZGV0YWNoXCIpIHtcbiAgICAgICAgICB2b2lkIGRldGFjaFN5bmNCYWNrYm9uZSgpO1xuICAgICAgICAgIHJldHVybjtcbiAgICAgICAgfVxuICAgICAgICByZXR1cm47XG4gICAgICB9XG5cbiAgICAgIGlmIChzdHVkaW9Nb2RlICYmIGFjdGlvbi5jb250cm9sbGVySWQgPT09IGxhbmRpbmdDb250cm9sbGVySWQgJiYgYWN0aW9uLmFjdGlvbiA9PT0gXCJpbXBvcnRTcGFjZVwiKSB7XG4gICAgICAgIGltcG9ydFNwYWNlSW5wdXRSZWYuY3VycmVudD8uY2xpY2soKTtcbiAgICAgICAgcmV0dXJuO1xuICAgICAgfVxuXG4gICAgICBpZiAoc3R1ZGlvTW9kZSAmJiBhY3Rpb24uYWN0aW9uID09PSBcInNwYXduQXBwXCIgJiYgYWN0aW9uLmNvbnRyb2xsZXJJZCAhPT0gaG9zdENvbnRyb2xsZXJJZCkge1xuICAgICAgICBjb25zdCBwbHVnaW5JZCA9IHR5cGVvZiBhY3Rpb24uYXJncyA9PT0gXCJvYmplY3RcIiAmJiBhY3Rpb24uYXJncyAhPSBudWxsICYmIFwicGx1Z2luSWRcIiBpbiBhY3Rpb24uYXJncyA/IFN0cmluZygoYWN0aW9uLmFyZ3MgYXMgeyBwbHVnaW5JZD86IHN0cmluZyB9KS5wbHVnaW5JZCA/PyBcIlwiKSA6IFwiXCI7XG4gICAgICAgIGNvbnN0IGN1cnJlbnRQYW5lbCA9IHBhcnNlUGFuZWxTdGF0ZShzZXNzaW9uLnZpZXdTdGF0ZSk7XG4gICAgICAgIGNvbnN0IHByb2dyYW0gPSBjdXJyZW50UGFuZWw/LnByb2dyYW1zLmZpbmQoKGVudHJ5KSA9PiBlbnRyeS5wbHVnaW5JZCA9PT0gcGx1Z2luSWQpO1xuICAgICAgICBpZiAocHJvZ3JhbSkgdm9pZCBzcGF3blByb2dyYW0ocHJvZ3JhbSk7XG4gICAgICAgIHJldHVybjtcbiAgICAgIH1cblxuICAgICAgaWYgKHN0dWRpb01vZGUgJiYgYWN0aW9uLmNvbnRyb2xsZXJJZCA9PT0gaG9zdENvbnRyb2xsZXJJZCAmJiBhY3Rpb24uYWN0aW9uID09PSBcInNldEFjdGl2ZVBhbmVsVGFiXCIpIHtcbiAgICAgICAgY29uc3QgdGFiSWQgPSB0eXBlb2YgYWN0aW9uLmFyZ3MgPT09IFwib2JqZWN0XCIgJiYgYWN0aW9uLmFyZ3MgIT0gbnVsbCAmJiBcInRhYklkXCIgaW4gYWN0aW9uLmFyZ3MgPyBTdHJpbmcoKGFjdGlvbi5hcmdzIGFzIHsgdGFiSWQ/OiBzdHJpbmcgfSkudGFiSWQgPz8gaG9zdENhdGFsb2d1ZVRhYklkID8/IFwiXCIpIDogKGhvc3RDYXRhbG9ndWVUYWJJZCA/PyBcIlwiKTtcbiAgICAgICAgY29uc3QgY3VycmVudFBhbmVsID0gcGFyc2VQYW5lbFN0YXRlKHNlc3Npb24udmlld1N0YXRlKSA/PyBidWlsZFNwYWNlUGFuZWxTdGF0ZShbXSwgW10pO1xuICAgICAgICB1cGRhdGVTcGFjZVBhbmVsKGJ1aWxkU3BhY2VQYW5lbFN0YXRlKGN1cnJlbnRQYW5lbC5wcm9ncmFtcywgY3VycmVudFBhbmVsLnNwYXduZWRBcHBzLCB0YWJJZCwgY3VycmVudFBhbmVsLmFjdGl2ZVNwYXduZWRJZCkpO1xuICAgICAgICByZXR1cm47XG4gICAgICB9XG5cbiAgICAgIGNvbnN0IHBsdWdpbkVudHJ5ID0gZmluZFBsdWdpbkZvckFjdGlvbihhY3Rpb24pO1xuICAgICAgY29uc3QgcGx1Z2luID0gcGx1Z2luRW50cnk/LmhhbmRsZTtcbiAgICAgIGlmICghcGx1Z2luKSByZXR1cm47XG5cbiAgICAgIGNvbnN0IHRhcmdldFNlc3Npb24gPVxuICAgICAgICBzdHVkaW9Nb2RlICYmIGFjdGlvbi5jb250cm9sbGVySWQgIT09IHNlc3Npb24uYXBwLmNvbnRyb2xsZXJJZFxuICAgICAgICAgID8gKCgpID0+IHtcbiAgICAgICAgICAgICAgY29uc3Qgc3Bhd25lZCA9IHBhbmVsPy5zcGF3bmVkQXBwcy5maW5kKChlbnRyeSkgPT4ge1xuICAgICAgICAgICAgICAgIGNvbnN0IGFwcCA9IGxvYWRlZFBsdWdpbnMuZmluZCgocCkgPT4gcC5oYW5kbGUucGx1Z2luSWQgPT09IGVudHJ5LnBsdWdpbklkKT8ubWFuaWZlc3QuYXBwcy5maW5kKChhKSA9PiBhLmlkID09PSBlbnRyeS5hcHBJZCk7XG4gICAgICAgICAgICAgICAgcmV0dXJuIGFwcD8uY29udHJvbGxlcklkID09PSBhY3Rpb24uY29udHJvbGxlcklkO1xuICAgICAgICAgICAgICB9KTtcbiAgICAgICAgICAgICAgaWYgKCFzcGF3bmVkKSByZXR1cm4gc2Vzc2lvbjtcbiAgICAgICAgICAgICAgY29uc3QgYXBwID0gbG9hZGVkUGx1Z2lucy5maW5kKChwKSA9PiBwLmhhbmRsZS5wbHVnaW5JZCA9PT0gc3Bhd25lZC5wbHVnaW5JZCk/Lm1hbmlmZXN0LmFwcHMuZmluZCgoYSkgPT4gYS5pZCA9PT0gc3Bhd25lZC5hcHBJZCk7XG4gICAgICAgICAgICAgIGlmICghYXBwKSByZXR1cm4gc2Vzc2lvbjtcbiAgICAgICAgICAgICAgcmV0dXJuIHsgcGx1Z2luSWQ6IHNwYXduZWQucGx1Z2luSWQsIGluc3RhbmNlSWQ6IHNwYXduZWQuaW5zdGFuY2VJZCwgYXBwLCB2aWV3U3RhdGU6IHNlc3Npb24udmlld1N0YXRlIH07XG4gICAgICAgICAgICB9KSgpXG4gICAgICAgICAgOiBzZXNzaW9uO1xuXG4gICAgICAvLyDwn5qr77iPIFRoZSBvbGQgYHNldERvY3VtZW50YCDihpIgYHBhdGNoQXBwU291cmNlYCBtaXJyb3IgKHNwYXduZWQtaW5zdGFuY2UgY29udGVudCB3cml0ZS1iYWNrIG9uIHRoZVxuICAgICAgLy8gb3MgZG9jdW1lbnQpIGlzIGRlbGV0ZWQg4oCUIGFwcCBjb250ZW50IG5vIGxvbmdlciBlbWJlZHMgb24gdGhlIG9zIGRvY3VtZW50IGF0IGFsbFxuICAgICAgLy8gKGBPc0FwcEluc3RhbmNlLmRvY3VtZW50YCBpcyBub3cganVzdCBhbiBgT3NEb2N1bWVudFJlZmAgaGFuZGxlKS4gQSBzcGF3bmVkIGluc3RhbmNlJ3MgY29udGVudFxuICAgICAgLy8gc3luYyBub3cgZ29lcyB0aHJvdWdoIGl0cyBvd24gYG9wZW5Eb2N1bWVudGAtb3BlbmVkIGBEb2N1bWVudEhvc3RgIGNoYW5uZWwsIHNhbWUgYXMgYW55IG90aGVyXG4gICAgICAvLyBkb2N1bWVudDsgdGhlcmUgaXMgbm8gaG9zdC1zaWRlIEpTIG1pcnJvcmluZyBzdGVwIGFueW1vcmUuXG4gICAgICAvLyDwn6qf77iPIGB3aW5kb3dJZGAgaXMgcmVhZCBiYWNrIG9mZiB0aGUgdGFnZ2VkIGBhY3Rpb24uYXJnc2AgKHNlZSBgd2luZG93TWVhc3VyZXNDaHJvbWVgL2B0YWdTZXRBY3RpdmVVdGlsaXR5V2luZG93YCksXG4gICAgICAvLyBmYWxsaW5nIGJhY2sgdG8gdGhlIGFjdGl2ZSB3aW5kb3cg4oCUIHN0YW1wZWQgaW50byB0aGUgZGlzcGF0Y2hlZCB2aWV3IHN0YXRlIHNvIHRoZSBwbHVnaW4gY2FuIGtleSBhbnlcbiAgICAgIC8vIHBlci13aW5kb3cgb3B0aW9uIG11dGF0aW9uIG9mZiBgdmlld19zdGF0ZS53aW5kb3dJZGAgaW5zdGVhZCBvZiBldmVyIGd1ZXNzaW5nIGF0IHRoZSBhY3RpdmUgd2luZG93LlxuICAgICAgY29uc3QgYWN0aW9uV2luZG93SWQgPSB0eXBlb2YgYWN0aW9uLmFyZ3MgPT09IFwib2JqZWN0XCIgJiYgYWN0aW9uLmFyZ3MgIT0gbnVsbCAmJiB0eXBlb2YgKGFjdGlvbi5hcmdzIGFzIHsgd2luZG93SWQ/OiB1bmtub3duIH0pLndpbmRvd0lkID09PSBcInN0cmluZ1wiID8gKGFjdGlvbi5hcmdzIGFzIHsgd2luZG93SWQ6IHN0cmluZyB9KS53aW5kb3dJZCA6IHVuZGVmaW5lZDtcbiAgICAgIGNvbnN0IGRpc3BhdGNoV2luZG93SWQgPSBhY3Rpb25XaW5kb3dJZCA/PyBhY3RpdmVXaW5kb3dJZFJlZi5jdXJyZW50ID8/IHVuZGVmaW5lZDtcbiAgICAgIGNvbnN0IGRpc3BhdGNoVmlld1N0YXRlID0gaW5qZWN0QWN0aXZlVXRpbGl0eShcbiAgICAgICAge1xuICAgICAgICAgIC4uLnRhcmdldFNlc3Npb24udmlld1N0YXRlLFxuICAgICAgICAgIHdpbmRvd0lkOiBkaXNwYXRjaFdpbmRvd0lkLFxuICAgICAgICAgIHdpbmRvd0luc3RhbmNlczogc2Vzc2lvbldpbmRvd0luc3RhbmNlcyh0YXJnZXRTZXNzaW9uLmFwcCwgZXh0cmFXaW5kb3dJbnN0YW5jZXNSZWYuY3VycmVudCkubWFwKChpbnN0YW5jZSkgPT4gKHsgaWQ6IGluc3RhbmNlLmlkLCB3aW5kb3dLaW5kSWQ6IGluc3RhbmNlLndpbmRvd0tpbmRJZCB9KSksXG4gICAgICAgIH0sXG4gICAgICAgIGRpc3BhdGNoV2luZG93SWQsXG4gICAgICApO1xuICAgICAgY29uc3QgZGVjbGFyZWRBY3Rpb24gPSB0YXJnZXRTZXNzaW9uLmFwcC5hY3Rpb25zPy5zb21lKChlbnRyeSkgPT4gZW50cnkuaWQgPT09IGFjdGlvbi5hY3Rpb24pID8/IGZhbHNlO1xuICAgICAgaWYgKCFkZWNsYXJlZEFjdGlvbiAmJiAhRlJBTUVXT1JLX1JFU0VSVkVEX0FDVElPTl9JRFMuaGFzKGFjdGlvbi5hY3Rpb24pKSB7XG4gICAgICAgIGNvbnNvbGUud2FybihcIltERUJVR10gc2tpcHBpbmcgdW5kZWNsYXJlZCBhY3Rpb25cIiwgYWN0aW9uLmFjdGlvbiwgdGFyZ2V0U2Vzc2lvbi5hcHAuaWQpO1xuICAgICAgICByZXR1cm47XG4gICAgICB9XG5cbiAgICAgIGNvbnN0IGludGVyYWN0aXZlQWN0aW9uID0gYWN0aW9uLmFjdGlvbiAhPT0gXCJzdWdnZXN0aW9uc1RpY2tcIiAmJiBhY3Rpb24uYWN0aW9uICE9PSBcImZpbGxCdWlsZFRpY2tcIjtcbiAgICAgIGlmIChpbnRlcmFjdGl2ZUFjdGlvbikgYmVnaW5JbnRlcmFjdGl2ZVBsdWdpbkFjdGlvbigpO1xuICAgICAgcmV0dXJuIHBsdWdpblxuICAgICAgICAuaGFuZGxlQWN0aW9uKHRhcmdldFNlc3Npb24uaW5zdGFuY2VJZCwgZW5jb2RlQWN0aW9uV2lyZShhY3Rpb24pLCBkaXNwYXRjaFZpZXdTdGF0ZSlcbiAgICAgICAgLnRoZW4oKHJlc3BvbnNlKSA9PiBhcHBseUhvc3RFZmZlY3RzKHJlc3BvbnNlLnJlcXVlc3RlZEVmZmVjdHMgPz8gW10sIHsgLi4udGFyZ2V0U2Vzc2lvbiwgdmlld1N0YXRlOiBkaXNwYXRjaFZpZXdTdGF0ZSB9LCByZXNvbHZlVWlEaXJ0eVNjb3BlKHJlc3BvbnNlLnVpU2NvcGUpKSlcbiAgICAgICAgLmNhdGNoKChhY3Rpb25FcnJvcikgPT4ge1xuICAgICAgICAgIGNvbnNvbGUuZXJyb3IoXCJbREVCVUddIGFjdGlvbiBmYWlsZWRcIiwgYWN0aW9uLmFjdGlvbiwgYWN0aW9uLmFyZ3MsIGFjdGlvbkVycm9yKTtcbiAgICAgICAgfSlcbiAgICAgICAgLmZpbmFsbHkoKCkgPT4ge1xuICAgICAgICAgIGlmIChpbnRlcmFjdGl2ZUFjdGlvbikgZW5kSW50ZXJhY3RpdmVQbHVnaW5BY3Rpb24oKTtcbiAgICAgICAgfSk7XG4gICAgfSxcbiAgICBbXG4gICAgICBhcHBseUhvc3RFZmZlY3RzLFxuICAgICAgYXR0YWNoU3luY0JhY2tib25lLFxuICAgICAgY2xlYXJBbGxXaW5kb3dVdGlsaXRpZXMsXG4gICAgICBkZXRhY2hTeW5jQmFja2JvbmUsXG4gICAgICBmaW5kUGx1Z2luRm9yQWN0aW9uLFxuICAgICAgaW5qZWN0QWN0aXZlVXRpbGl0eSxcbiAgICAgIGxvYWRlZFBsdWdpbnMsXG4gICAgICBwYW5lbCxcbiAgICAgIHNlc3Npb24sXG4gICAgICBzZXRBY3RpdmVVdGlsaXR5Rm9yV2luZG93LFxuICAgICAgc3Bhd25Qcm9ncmFtLFxuICAgICAgc3R1ZGlvTW9kZSxcbiAgICAgIHN5bmNCYWNrYm9uZVVyaSxcbiAgICAgIHN5bmNEcmFmdFBhdGgsXG4gICAgICB1cGRhdGVTcGFjZVBhbmVsLFxuICAgICAgaG9zdENvbnRyb2xsZXJJZCxcbiAgICAgIGxhbmRpbmdDb250cm9sbGVySWQsXG4gICAgICBob3N0Q2F0YWxvZ3VlVGFiSWQsXG4gICAgICBjb21wbGV0ZUludHJvZHVjdGlvbkludGVyYWN0aW9uLFxuICAgICAgcHJpbWFyeVBsdWdpbklkLFxuICAgICAgcmVsb2FkUGx1Z2luLFxuICAgICAgdW5pbnN0YWxsUGx1Z2luLFxuICAgICAgcGx1Z2luU3VwZXJ2aXNvckJ5SWQsXG4gICAgXSxcbiAgKTtcblxuICAvKiog8J+nre+4jyBMb2dzIGEgc2hlbGwtY2hyb21lIGNvbW1hbmQgKHRoZW1lIGNoYW5nZSwgZG9jayBkcmFnLCB3aW5kb3cgcmVzaXplLCBwYW5lbCB0b2dnbGUsIOKApikgaW50byB0aGVcbiAgICogcGx1Z2luJ3Mgc2Vzc2lvbi1vbmx5IGNvbW1hbmQtaGlzdG9yeSBwYW5lbCDigJQgcm91dGVkIHRocm91Z2ggdGhlIGV4YWN0IHNhbWUgYG9uQWN0aW9uYCBmdW5uZWwgYXMgZXZlcnlcbiAgICogb3RoZXIgYWN0aW9uIChzZWUgYE5PVEVfU0hFTExfQ09NTUFORF9BQ1RJT05fSURgKSBzbyBpdCBsYW5kcyBvbiBgdGFyZ2V0U2Vzc2lvbi5pbnN0YW5jZUlkYCB2aWEgdGhlXG4gICAqIHN0YW5kYXJkIGBoYW5kbGVBY3Rpb25gIGNhbGwsIGp1c3QgdGFnZ2VkIHdpdGggYW4gaWQgdGhlIHBsdWdpbiBpbnRlcmNlcHRzIGJlZm9yZSB0aGUgYXBwIHNlZXMgaXQuXG4gICAqIE5vLW9wcyB3aGVuIHRoZXJlJ3Mgbm8gYWN0aXZlIGFwcCBzZXNzaW9uLiAqL1xuICBjb25zdCBub3RlU2hlbGxDb21tYW5kID0gdXNlQ2FsbGJhY2soXG4gICAgKGNvbW1hbmRJZDogc3RyaW5nLCBsYWJlbDogc3RyaW5nLCBkZXRhaWw/OiBSZWNvcmQ8c3RyaW5nLCB1bmtub3duPikgPT4ge1xuICAgICAgaWYgKCFzZXNzaW9uKSByZXR1cm47XG4gICAgICBvbkFjdGlvbihidWlsZE5vdGVTaGVsbENvbW1hbmRBY3Rpb24oc2Vzc2lvbi5hcHAuY29udHJvbGxlcklkLCBjb21tYW5kSWQsIGxhYmVsLCBkZXRhaWwpKTtcbiAgICB9LFxuICAgIFtzZXNzaW9uLCBvbkFjdGlvbl0sXG4gICk7XG5cbiAgY29uc3Qgb25BY3Rpb25SZWYgPSB1c2VSZWYob25BY3Rpb24pO1xuICB1c2VFZmZlY3QoKCkgPT4ge1xuICAgIG9uQWN0aW9uUmVmLmN1cnJlbnQgPSBvbkFjdGlvbjtcbiAgfSwgW29uQWN0aW9uXSk7XG5cbiAgLy8g8J+Qou+4jyBgb25BY3Rpb25gJ3Mgb3duIGlkZW50aXR5IGNodXJucyBldmVyeSBhY3Rpb24gKGl0cyBkZXBzIGluY2x1ZGUgYHNlc3Npb25gLCBgcGFuZWxgLCDigKYpLiBSZW5kZXJcbiAgLy8gdHJlZXMgYnVpbHQgZnJvbSBgVWlOb2RlYHMgb25seSBuZWVkIGEgKmNhbGxhYmxlKiBhY3Rpb24gZGlzcGF0Y2hlciwgbm90IGEgZnJlc2ggb25lIGVhY2ggdGltZSDigJRcbiAgLy8gcm91dGUgdGhlbSB0aHJvdWdoIHRoaXMgcGVybWFuZW50bHktc3RhYmxlIHJlZiBpbmRpcmVjdGlvbiBzbyBgaW50ZXJwcmV0VWlOb2RlYCdzIGBSZWFjdC5tZW1vYFxuICAvLyAoYW5kIGFueSBgdXNlTWVtb2Aga2V5ZWQgb24gdGhlIGRpc3BhdGNoZXIgcGFzc2VkIHRvIGl0KSBjYW4gYWN0dWFsbHkgYmFpbC5cbiAgY29uc3Qgb25BY3Rpb25TdGFibGUgPSB1c2VDYWxsYmFjaygoYWN0aW9uOiBQYXJhbWV0ZXJzPHR5cGVvZiBvbkFjdGlvbj5bMF0pID0+IG9uQWN0aW9uUmVmLmN1cnJlbnQoYWN0aW9uKSwgW10pO1xuXG4gIC8vI3JlZ2lvbiDwn46l77iPVHV0b3JpYWxPcmNoZXN0cmF0aW9uXG4gIC8qKiDij7HvuI8gUmVhbC10aW1lIHRocm90dGxlIGZvciB0aGUgZGlyZWN0b3IncyBVSS9kb2N1bWVudC9ldmVudCBhcHBsaWNhdGlvbiAofjEwSHopIOKAlCBjYW1lcmEgc3RheXNcbiAgICogc21vb3RoIGV2ZXJ5IGNsb2NrIHRpY2sgcmVnYXJkbGVzcyAoc2VlIHRoZSBgc3Vic2NyaWJlYCBjYWxsYmFjayBiZWxvdykuICovXG4gIGNvbnN0IFRVVE9SSUFMX0RJUkVDVE9SX1RJQ0tfTVMgPSA5MDtcblxuICBjb25zdCBhY3RpdmVUdXRvcmlhbCA9IHVzZU1lbW8oKCkgPT4gYWN0aXZlVHV0b3JpYWxzLmZpbmQoKHR1dG9yaWFsKSA9PiB0dXRvcmlhbC5pZCA9PT0gYWN0aXZlVHV0b3JpYWxJZCkgPz8gbnVsbCwgW2FjdGl2ZVR1dG9yaWFscywgYWN0aXZlVHV0b3JpYWxJZF0pO1xuXG4gIGNvbnN0IHR1dG9yaWFsQ2xvY2tSZWYgPSB1c2VSZWY8VHV0b3JpYWxDbG9jayB8IG51bGw+KG51bGwpO1xuICBpZiAoIXR1dG9yaWFsQ2xvY2tSZWYuY3VycmVudCkgdHV0b3JpYWxDbG9ja1JlZi5jdXJyZW50ID0gY3JlYXRlVHV0b3JpYWxDbG9jayhhY3RpdmVUdXRvcmlhbD8uZHVyYXRpb25NcyA/PyAwKTtcbiAgY29uc3QgdHV0b3JpYWxDbG9jayA9IHR1dG9yaWFsQ2xvY2tSZWYuY3VycmVudDtcbiAgdXNlRWZmZWN0KCgpID0+ICgpID0+IHR1dG9yaWFsQ2xvY2tSZWYuY3VycmVudD8uZGlzcG9zZSgpLCBbXSk7XG4gIHVzZUVmZmVjdCgoKSA9PiB7XG4gICAgdHV0b3JpYWxDbG9jay5zZXREdXJhdGlvbk1zKGFjdGl2ZVR1dG9yaWFsPy5kdXJhdGlvbk1zID8/IDApO1xuICB9LCBbYWN0aXZlVHV0b3JpYWw/LmR1cmF0aW9uTXMsIHR1dG9yaWFsQ2xvY2tdKTtcbiAgdXNlRWZmZWN0KCgpID0+IHtcbiAgICB0dXRvcmlhbENsb2NrLnNldFJhdGUodHV0b3JpYWxSYXRlKTtcbiAgfSwgW3R1dG9yaWFsUmF0ZSwgdHV0b3JpYWxDbG9ja10pO1xuICB1c2VFZmZlY3QoKCkgPT4ge1xuICAgIGlmICh0dXRvcmlhbFBsYXlpbmcpIHR1dG9yaWFsQ2xvY2sucGxheSgpO1xuICAgIGVsc2UgdHV0b3JpYWxDbG9jay5wYXVzZSgpO1xuICB9LCBbdHV0b3JpYWxQbGF5aW5nLCB0dXRvcmlhbENsb2NrXSk7XG5cbiAgY29uc3QgdWlCcmlkZ2VDdHhSZWYgPSB1c2VSZWY8VHV0b3JpYWxVaUJyaWRnZUNvbnRleHQ+KHsgc2Vzc2lvbiwgYXBwTGFiZWxzT3ZlcmxheSwgdGVybWlub2xvZ3k6IHVpVGVybWlub2xvZ3ksIGxvY2FsZTogdWlMb2NhbGUgfSk7XG4gIHVpQnJpZGdlQ3R4UmVmLmN1cnJlbnQgPSB7IHNlc3Npb24sIGFwcExhYmVsc092ZXJsYXksIHRlcm1pbm9sb2d5OiB1aVRlcm1pbm9sb2d5LCBsb2NhbGU6IHVpTG9jYWxlIH07XG5cbiAgLyoqIOKPse+4jyBQbGF5aGVhZCAobXMpIHRoZSBkaXJlY3Rvci9zZWVrIGxhc3QgYXBwbGllZCBkb2N1bWVudC9VSSB0cmFja3MgdXAgdG8g4oCUIHRoZSBcImZyb21cIiBzaWRlIG9mIHRoZVxuICAgKiBuZXh0IGB0dXRvcmlhbFNsaWNlKGRlZiwgZnJvbSwgdG8pYCBjYWxsLiBSZXNldCB0byAwIG9uIHNhbmRib3ggKHJlKXN0YXJ0LiAqL1xuICBjb25zdCB0dXRvcmlhbExhc3RBcHBsaWVkTXNSZWYgPSB1c2VSZWYoMCk7XG4gIC8qKiDwn46s77iPIFNhbmRib3hlZC1vdXQgbGl2ZSBkb2N1bWVudCAoZnVsbCBgRG9jdW1lbnRFbnZlbG9wZWAgSlNPTiksIHJlc3RvcmVkIG9uIHN0b3AvZXhpdC4gKi9cbiAgY29uc3QgdHV0b3JpYWxEb2N1bWVudFNuYXBzaG90UmVmID0gdXNlUmVmPHN0cmluZyB8IG51bGw+KG51bGwpO1xuXG4gIC8vIPCfjqzvuI8gU2FuZGJveCBzdGFydC9zdG9wIChkZXNpZ24gcG9pbnQgMyk6IG9uIGFjdGl2YXRpb24sIHNuYXBzaG90IHRoZSBsaXZlIGRvY3VtZW50LCBsb2FkIGBiYXNlYCwgYXBwbHlcbiAgLy8gYGJhc2UudWlgL2BiYXNlLmNhbWVyYXNgLCBhbmQgc2VlayB0aGUgY2xvY2sgdG8gMDsgb24gZGVhY3RpdmF0aW9uLCByZXN0b3JlIHRoZSBzbmFwc2hvdC5cbiAgY29uc3QgcHJldkFjdGl2ZVR1dG9yaWFsSWRSZWYgPSB1c2VSZWY8c3RyaW5nIHwgbnVsbD4obnVsbCk7XG4gIHVzZUVmZmVjdCgoKSA9PiB7XG4gICAgY29uc3QgcHJldmlvdXNJZCA9IHByZXZBY3RpdmVUdXRvcmlhbElkUmVmLmN1cnJlbnQ7XG4gICAgcHJldkFjdGl2ZVR1dG9yaWFsSWRSZWYuY3VycmVudCA9IGFjdGl2ZVR1dG9yaWFsSWQ7XG4gICAgaWYgKHByZXZpb3VzSWQgPT09IGFjdGl2ZVR1dG9yaWFsSWQgfHwgIXNlc3Npb24pIHJldHVybjtcbiAgICBjb25zdCBwbHVnaW4gPSBsb2FkZWRQbHVnaW5zLmZpbmQoKGVudHJ5KSA9PiBlbnRyeS5oYW5kbGUucGx1Z2luSWQgPT09IHNlc3Npb24ucGx1Z2luSWQpPy5oYW5kbGU7XG4gICAgaWYgKCFwbHVnaW4pIHJldHVybjtcbiAgICBpZiAoYWN0aXZlVHV0b3JpYWxJZCkge1xuICAgICAgY29uc3QgZGVmID0gYWN0aXZlVHV0b3JpYWxzLmZpbmQoKHR1dG9yaWFsKSA9PiB0dXRvcmlhbC5pZCA9PT0gYWN0aXZlVHV0b3JpYWxJZCk7XG4gICAgICBpZiAoIWRlZikgcmV0dXJuO1xuICAgICAgdHV0b3JpYWxEcml2ZW5SZWYuY3VycmVudCA9IHRydWU7XG4gICAgICB2b2lkIChhc3luYyAoKSA9PiB7XG4gICAgICAgIHRyeSB7XG4gICAgICAgICAgaWYgKHBsdWdpbi5yZWFkQXBwRG9jdW1lbnQpIHR1dG9yaWFsRG9jdW1lbnRTbmFwc2hvdFJlZi5jdXJyZW50ID0gYXdhaXQgcGx1Z2luLnJlYWRBcHBEb2N1bWVudChzZXNzaW9uLmluc3RhbmNlSWQpO1xuICAgICAgICB9IGNhdGNoIChzbmFwc2hvdEVycm9yKSB7XG4gICAgICAgICAgY29uc29sZS5lcnJvcihcIltERUJVR10gdHV0b3JpYWwgc2FuZGJveCBzbmFwc2hvdCBmYWlsZWRcIiwgc25hcHNob3RFcnJvcik7XG4gICAgICAgIH1cbiAgICAgICAgdHJ5IHtcbiAgICAgICAgICBpZiAoZGVmLmJhc2UuZG9jdW1lbnRKc29uICYmIHBsdWdpbi5sb2FkQXBwRG9jdW1lbnQpIGF3YWl0IHBsdWdpbi5sb2FkQXBwRG9jdW1lbnQoc2Vzc2lvbi5pbnN0YW5jZUlkLCBkZWYuYmFzZS5kb2N1bWVudEpzb24pO1xuICAgICAgICAgIGVsc2UgaWYgKGRlZi5iYXNlLmV4YW1wbGVJZCkgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9BQ1RJVkVfRVhBTVBMRV9JRFwiLCB2YWx1ZTogZGVmLmJhc2UuZXhhbXBsZUlkIH0pO1xuICAgICAgICB9IGNhdGNoIChsb2FkRXJyb3IpIHtcbiAgICAgICAgICBjb25zb2xlLmVycm9yKFwiW0RFQlVHXSB0dXRvcmlhbCBiYXNlIGRvY3VtZW50IGxvYWQgZmFpbGVkXCIsIGxvYWRFcnJvcik7XG4gICAgICAgIH1cbiAgICAgICAgYXBwbHlUdXRvcmlhbFVpU25hcHNob3RUb1NoZWxsKGRpc3BhdGNoLCBkZWYuYmFzZS51aSwgdWlCcmlkZ2VDdHhSZWYuY3VycmVudCk7XG4gICAgICAgIGZvciAoY29uc3QgY2FtZXJhS2V5ZnJhbWUgb2YgZGVmLmJhc2UuY2FtZXJhcykgZ2V0VHV0b3JpYWxDYW1lcmFEcml2ZXIoY2FtZXJhS2V5ZnJhbWUud2luZG93SWQpPy5zZXQoY2FtZXJhS2V5ZnJhbWUuY2FtZXJhKTtcbiAgICAgICAgdHV0b3JpYWxMYXN0QXBwbGllZE1zUmVmLmN1cnJlbnQgPSAwO1xuICAgICAgICB0dXRvcmlhbENsb2NrLnNlZWsoMCk7XG4gICAgICAgIGF3YWl0IHJlZnJlc2hVaShzZXNzaW9uLCB7IGtpbmQ6IFwiZnVsbFwiIH0pO1xuICAgICAgICB0dXRvcmlhbERyaXZlblJlZi5jdXJyZW50ID0gZmFsc2U7XG4gICAgICB9KSgpO1xuICAgIH0gZWxzZSBpZiAocHJldmlvdXNJZCkge1xuICAgICAgdHV0b3JpYWxEcml2ZW5SZWYuY3VycmVudCA9IHRydWU7XG4gICAgICB2b2lkIChhc3luYyAoKSA9PiB7XG4gICAgICAgIHRyeSB7XG4gICAgICAgICAgY29uc3Qgc25hcHNob3RKc29uID0gdHV0b3JpYWxEb2N1bWVudFNuYXBzaG90UmVmLmN1cnJlbnQ7XG4gICAgICAgICAgaWYgKHNuYXBzaG90SnNvbiAmJiBwbHVnaW4ubG9hZEFwcERvY3VtZW50KSBhd2FpdCBwbHVnaW4ubG9hZEFwcERvY3VtZW50KHNlc3Npb24uaW5zdGFuY2VJZCwgc25hcHNob3RKc29uKTtcbiAgICAgICAgfSBjYXRjaCAocmVzdG9yZUVycm9yKSB7XG4gICAgICAgICAgY29uc29sZS5lcnJvcihcIltERUJVR10gdHV0b3JpYWwgc2FuZGJveCByZXN0b3JlIGZhaWxlZFwiLCByZXN0b3JlRXJyb3IpO1xuICAgICAgICB9XG4gICAgICAgIHR1dG9yaWFsRG9jdW1lbnRTbmFwc2hvdFJlZi5jdXJyZW50ID0gbnVsbDtcbiAgICAgICAgYXdhaXQgcmVmcmVzaFVpKHNlc3Npb24sIHsga2luZDogXCJmdWxsXCIgfSk7XG4gICAgICAgIHR1dG9yaWFsRHJpdmVuUmVmLmN1cnJlbnQgPSBmYWxzZTtcbiAgICAgIH0pKCk7XG4gICAgfVxuICB9LCBbYWN0aXZlVHV0b3JpYWxJZCwgYWN0aXZlVHV0b3JpYWxzLCBzZXNzaW9uLCBsb2FkZWRQbHVnaW5zLCB0dXRvcmlhbENsb2NrLCByZWZyZXNoVWldKTtcblxuICAvKiog8J+OrO+4jyBBcHBsaWVzIGV2ZXJ5IGVudHJ5IG9mIG9uZSBgVHV0b3JpYWxTbGljZWAgKGEgZGlyZWN0b3IgdGljayBvciBhIHNlZWsgc3Bhbikgb250byB0aGUgbGl2ZVxuICAgKiBzZXNzaW9uIOKAlCBVSSBjaGFuZ2VzIGZpcnN0LCB0aGVuIGRvY3VtZW50LXRyYWNrIGVudHJpZXMgdGhyb3VnaCB0aGUgcGx1Z2luIGJyaWRnZTogYEVkaXRgIHZpYVxuICAgKiBgYXBwbHlPcGVyYXRpb25zYCAoZm9yd2FyZC9iYWNrd2FyZCBwZXIgYHNsaWNlLmZvcndhcmRgKSwgYExvYWRgIHZpYSBgbG9hZEFwcERvY3VtZW50YCxcbiAgICogYFVuZG9gL2BSZWRvYC9gQ2hlY2twb2ludGAvYENoZWNrb3V0Q2hlY2twb2ludGAvYFN3aXRjaEFsdGVybmF0aXZlYCB2aWEgdGhlIFNBTUUgSGlzdG9yeS1hY3Rpb25cbiAgICogYG9uQWN0aW9uYCBmdW5uZWwgdGhlIGFwcCdzIG93biB1bmRvL3JlZG8gYnV0dG9ucyBkaXNwYXRjaCB0aHJvdWdoIChuZXZlciBhIGJlc3Bva2UgY2hhbm5lbCkg4oCUIHRoZW5cbiAgICogcHVsc2VzIGFueSBhbm5vdGF0aW9uYWwgZXZlbnQncyB0YXJnZXQgZWxlbWVudCB2aWEgdGhlIGV4aXN0aW5nIGBjZWxlYnJhdGVFbGVtZW50c2Agdm9jYWJ1bGFyeS4gKi9cbiAgY29uc3QgYXBwbHlUdXRvcmlhbFNsaWNlVG9TaGVsbCA9IHVzZUNhbGxiYWNrKFxuICAgIGFzeW5jIChzbGljZTogVHV0b3JpYWxTbGljZSwgYWN0aXZlU2Vzc2lvbjogQWN0aXZlU2Vzc2lvbikgPT4ge1xuICAgICAgZm9yIChjb25zdCBjaGFuZ2Ugb2Ygc2xpY2UudWlDaGFuZ2VzKSBhcHBseVR1dG9yaWFsVWlDaGFuZ2VUb1NoZWxsKGRpc3BhdGNoLCBjaGFuZ2UsIHVpQnJpZGdlQ3R4UmVmLmN1cnJlbnQpO1xuICAgICAgY29uc3QgcGx1Z2luID0gbG9hZGVkUGx1Z2lucy5maW5kKChlbnRyeSkgPT4gZW50cnkuaGFuZGxlLnBsdWdpbklkID09PSBhY3RpdmVTZXNzaW9uLnBsdWdpbklkKT8uaGFuZGxlO1xuICAgICAgbGV0IGRvY3VtZW50VG91Y2hlZCA9IGZhbHNlO1xuICAgICAgZm9yIChjb25zdCBkb2N1bWVudEV2ZW50IG9mIHNsaWNlLmRvY3VtZW50KSB7XG4gICAgICAgIGNvbnN0IGtpbmQ6IFR1dG9yaWFsRG9jdW1lbnRFdmVudEtpbmQgPSBkb2N1bWVudEV2ZW50LmtpbmQ7XG4gICAgICAgIGlmIChraW5kLmtpbmQgPT09IFwiZWRpdFwiKSB7XG4gICAgICAgICAgZG9jdW1lbnRUb3VjaGVkID0gdHJ1ZTtcbiAgICAgICAgICBjb25zdCBvcGVyYXRpb25zID0gc2xpY2UuZm9yd2FyZCA/IGtpbmQuZm9yd2FyZHMgOiBraW5kLmJhY2t3YXJkcztcbiAgICAgICAgICBpZiAocGx1Z2luPy5hcHBseU9wZXJhdGlvbnMpIGF3YWl0IHBsdWdpbi5hcHBseU9wZXJhdGlvbnMoYWN0aXZlU2Vzc2lvbi5pbnN0YW5jZUlkLCBlbmNvZGVPcGVyYXRpb25FbnZlbG9wZXNQYWNrKG9wZXJhdGlvbnMpKTtcbiAgICAgICAgfSBlbHNlIGlmIChraW5kLmtpbmQgPT09IFwibG9hZFwiKSB7XG4gICAgICAgICAgZG9jdW1lbnRUb3VjaGVkID0gdHJ1ZTtcbiAgICAgICAgICBjb25zdCBkb2N1bWVudEpzb24gPSBzbGljZS5mb3J3YXJkID8ga2luZC5kb2N1bWVudEpzb24gOiBraW5kLnByZXZpb3VzSnNvbjtcbiAgICAgICAgICBpZiAocGx1Z2luPy5sb2FkQXBwRG9jdW1lbnQpIGF3YWl0IHBsdWdpbi5sb2FkQXBwRG9jdW1lbnQoYWN0aXZlU2Vzc2lvbi5pbnN0YW5jZUlkLCBkb2N1bWVudEpzb24pO1xuICAgICAgICB9IGVsc2UgaWYgKGtpbmQua2luZCA9PT0gXCJ1bmRvXCIpIHtcbiAgICAgICAgICBvbkFjdGlvblJlZi5jdXJyZW50KHsgY29udHJvbGxlcklkOiBhY3RpdmVTZXNzaW9uLmFwcC5jb250cm9sbGVySWQsIGFjdGlvbjogc2xpY2UuZm9yd2FyZCA/IFwidW5kb1wiIDogXCJyZWRvXCIgfSk7XG4gICAgICAgIH0gZWxzZSBpZiAoa2luZC5raW5kID09PSBcInJlZG9cIikge1xuICAgICAgICAgIG9uQWN0aW9uUmVmLmN1cnJlbnQoeyBjb250cm9sbGVySWQ6IGFjdGl2ZVNlc3Npb24uYXBwLmNvbnRyb2xsZXJJZCwgYWN0aW9uOiBzbGljZS5mb3J3YXJkID8gXCJyZWRvXCIgOiBcInVuZG9cIiB9KTtcbiAgICAgICAgfSBlbHNlIGlmIChraW5kLmtpbmQgPT09IFwiY2hlY2twb2ludFwiKSB7XG4gICAgICAgICAgaWYgKHNsaWNlLmZvcndhcmQpIG9uQWN0aW9uUmVmLmN1cnJlbnQoeyBjb250cm9sbGVySWQ6IGFjdGl2ZVNlc3Npb24uYXBwLmNvbnRyb2xsZXJJZCwgYWN0aW9uOiBcImNvbW1pdENoZWNrcG9pbnRcIiB9KTtcbiAgICAgICAgfSBlbHNlIGlmIChraW5kLmtpbmQgPT09IFwiY2hlY2tvdXRDaGVja3BvaW50XCIpIHtcbiAgICAgICAgICBvbkFjdGlvblJlZi5jdXJyZW50KHsgY29udHJvbGxlcklkOiBhY3RpdmVTZXNzaW9uLmFwcC5jb250cm9sbGVySWQsIGFjdGlvbjogXCJjaGVja291dENoZWNrcG9pbnRcIiwgYXJnczogeyBjaGVja3BvaW50SWQ6IGtpbmQuY2hlY2twb2ludElkIH0gfSk7XG4gICAgICAgIH0gZWxzZSBpZiAoa2luZC5raW5kID09PSBcInN3aXRjaEFsdGVybmF0aXZlXCIpIHtcbiAgICAgICAgICBvbkFjdGlvblJlZi5jdXJyZW50KHsgY29udHJvbGxlcklkOiBhY3RpdmVTZXNzaW9uLmFwcC5jb250cm9sbGVySWQsIGFjdGlvbjogXCJzd2l0Y2hBbHRlcm5hdGl2ZVwiLCBhcmdzOiB7IGFsdGVybmF0aXZlSWQ6IGtpbmQuYWx0ZXJuYXRpdmVJZCB9IH0pO1xuICAgICAgICB9XG4gICAgICB9XG4gICAgICBmb3IgKGNvbnN0IGV2ZW50IG9mIHNsaWNlLmV2ZW50cykge1xuICAgICAgICBjb25zdCBraW5kID0gZXZlbnQua2luZDtcbiAgICAgICAgY29uc3QgdGFyZ2V0SWQgPSBraW5kLmtpbmQgPT09IFwiYWN0aW9uXCIgPyBraW5kLmFjdGlvbiA6IGtpbmQua2luZCA9PT0gXCJjb21tYW5kXCIgPyBraW5kLmNvbW1hbmQgOiB1bmRlZmluZWQ7XG4gICAgICAgIGlmICh0YXJnZXRJZCAmJiBzY29wZS5yb290UmVmLmN1cnJlbnQpIGNlbGVicmF0ZUVsZW1lbnRzKGVsZW1lbnRJZFNlbGVjdG9yKHRhcmdldElkKSwgQ0VMRUJSQVRFX1NUQU1QX0RVUkFUSU9OX01TLCBzY29wZS5yb290UmVmLmN1cnJlbnQpO1xuICAgICAgfVxuICAgICAgaWYgKGRvY3VtZW50VG91Y2hlZCkgYXdhaXQgcmVmcmVzaFVpKGFjdGl2ZVNlc3Npb24sIHsga2luZDogXCJmdWxsXCIgfSk7XG4gICAgfSxcbiAgICBbbG9hZGVkUGx1Z2lucywgcmVmcmVzaFVpXSxcbiAgKTtcblxuICAvLyDwn46s77iPIERpcmVjdG9yOiBvbmUgc3Vic2NyaXB0aW9uIHRvIHRoZSBjbG9jaydzIHJBRi1kcml2ZW4gdGlja3MuIENhbWVyYSBpbnRlcnBvbGF0aW9uIGFwcGxpZXMgZXZlcnlcbiAgLy8gdGljayAoc21vb3RoKTsgVUkvZG9jdW1lbnQvZXZlbnQgYXBwbGljYXRpb24gdGhyb3R0bGVzIHRvIGBUVVRPUklBTF9ESVJFQ1RPUl9USUNLX01TYC5cbiAgdXNlRWZmZWN0KCgpID0+IHtcbiAgICBjb25zdCBkZWYgPSBhY3RpdmVUdXRvcmlhbDtcbiAgICBpZiAoIWRlZiB8fCAhc2Vzc2lvbikgcmV0dXJuO1xuICAgIGxldCBsYXN0SGVhdnlUaWNrQXQgPSAwO1xuICAgIGNvbnN0IGNhbWVyYVdpbmRvd0lkcyA9IG5ldyBTZXQoWy4uLmRlZi5iYXNlLmNhbWVyYXMsIC4uLmRlZi50cmFja3MuY2FtZXJhXS5tYXAoKGtleWZyYW1lKSA9PiBrZXlmcmFtZS53aW5kb3dJZCkpO1xuICAgIGNvbnN0IHVuc3Vic2NyaWJlID0gdHV0b3JpYWxDbG9jay5zdWJzY3JpYmUoKCkgPT4ge1xuICAgICAgY29uc3QgdCA9IHR1dG9yaWFsQ2xvY2suZ2V0VGltZU1zKCk7XG4gICAgICBmb3IgKGNvbnN0IHdpbmRvd0lkIG9mIGNhbWVyYVdpbmRvd0lkcykge1xuICAgICAgICBjb25zdCBwb3NlID0gdHV0b3JpYWxDYW1lcmFBdChkZWYsIHdpbmRvd0lkLCB0KTtcbiAgICAgICAgaWYgKHBvc2UpIGdldFR1dG9yaWFsQ2FtZXJhRHJpdmVyKHdpbmRvd0lkKT8uc2V0KHBvc2UpO1xuICAgICAgfVxuICAgICAgaWYgKCF0dXRvcmlhbENsb2NrLmlzUGxheWluZygpKSByZXR1cm47XG4gICAgICBjb25zdCBub3cgPSBwZXJmb3JtYW5jZS5ub3coKTtcbiAgICAgIGlmIChub3cgLSBsYXN0SGVhdnlUaWNrQXQgPCBUVVRPUklBTF9ESVJFQ1RPUl9USUNLX01TKSByZXR1cm47XG4gICAgICBsYXN0SGVhdnlUaWNrQXQgPSBub3c7XG4gICAgICBjb25zdCBmcm9tID0gdHV0b3JpYWxMYXN0QXBwbGllZE1zUmVmLmN1cnJlbnQ7XG4gICAgICBpZiAoZnJvbSA9PT0gdCkgcmV0dXJuO1xuICAgICAgY29uc3Qgc2xpY2UgPSB0dXRvcmlhbFNsaWNlKGRlZiwgZnJvbSwgdCk7XG4gICAgICB0dXRvcmlhbExhc3RBcHBsaWVkTXNSZWYuY3VycmVudCA9IHQ7XG4gICAgICB0dXRvcmlhbERyaXZlblJlZi5jdXJyZW50ID0gdHJ1ZTtcbiAgICAgIHZvaWQgYXBwbHlUdXRvcmlhbFNsaWNlVG9TaGVsbChzbGljZSwgc2Vzc2lvbikuZmluYWxseSgoKSA9PiB7XG4gICAgICAgIHR1dG9yaWFsRHJpdmVuUmVmLmN1cnJlbnQgPSBmYWxzZTtcbiAgICAgIH0pO1xuICAgIH0pO1xuICAgIHJldHVybiB1bnN1YnNjcmliZTtcbiAgfSwgW2FjdGl2ZVR1dG9yaWFsLCBzZXNzaW9uLCB0dXRvcmlhbENsb2NrLCBhcHBseVR1dG9yaWFsU2xpY2VUb1NoZWxsXSk7XG5cbiAgLyoqIOKcgu+4jyBTZWVrL3JlYnVpbGQgKGRlc2lnbiBwb2ludCA1KTogY29tcG9zZXMgVUkgd2hvbGVzYWxlIChuZXZlciBhY2N1bXVsYXRlcyBkZWx0YXMgYWNyb3NzIGEgc2VlayDigJRcbiAgICogbWlycm9ycyB0aGUgUnVzdCBgdHV0b3JpYWxfc2xpY2VgIGRvYyBjb21tZW50J3Mgb3duIHdhcm5pbmcpLCBhcHBsaWVzIHRoZSBmb3J3YXJkL2JhY2t3YXJkIGRvY3VtZW50XG4gICAqIHNwYW4gY3Jvc3NlZCBzaW5jZSB0aGUgbGFzdCBhcHBsaWVkIHBsYXloZWFkLCBzZXRzIGV2ZXJ5IGNhbWVyYSBleGFjdGx5IChubyBpbnRlcnBvbGF0aW9uIG9uIGEgc2VlayksXG4gICAqIGFuZCBtb3ZlcyB0aGUgY2xvY2suICovXG4gIGNvbnN0IHNlZWtUdXRvcmlhbCA9IHVzZUNhbGxiYWNrKFxuICAgIChtczogbnVtYmVyKSA9PiB7XG4gICAgICBjb25zdCBkZWYgPSBhY3RpdmVUdXRvcmlhbDtcbiAgICAgIGlmICghZGVmIHx8ICFzZXNzaW9uKSByZXR1cm47XG4gICAgICBjb25zdCBjbGFtcGVkID0gTWF0aC5taW4oZGVmLmR1cmF0aW9uTXMsIE1hdGgubWF4KDAsIG1zKSk7XG4gICAgICBjb25zdCBmcm9tID0gdHV0b3JpYWxMYXN0QXBwbGllZE1zUmVmLmN1cnJlbnQ7XG4gICAgICB0dXRvcmlhbERyaXZlblJlZi5jdXJyZW50ID0gdHJ1ZTtcbiAgICAgIHZvaWQgKGFzeW5jICgpID0+IHtcbiAgICAgICAgYXBwbHlUdXRvcmlhbFVpU25hcHNob3RUb1NoZWxsKGRpc3BhdGNoLCBjb21wb3NlVHV0b3JpYWxVaShkZWYsIGNsYW1wZWQpLCB1aUJyaWRnZUN0eFJlZi5jdXJyZW50KTtcbiAgICAgICAgY29uc3QgcGx1Z2luID0gbG9hZGVkUGx1Z2lucy5maW5kKChlbnRyeSkgPT4gZW50cnkuaGFuZGxlLnBsdWdpbklkID09PSBzZXNzaW9uLnBsdWdpbklkKT8uaGFuZGxlO1xuICAgICAgICBjb25zdCBzbGljZSA9IHR1dG9yaWFsU2xpY2UoZGVmLCBmcm9tLCBjbGFtcGVkKTtcbiAgICAgICAgbGV0IGRvY3VtZW50VG91Y2hlZCA9IGZhbHNlO1xuICAgICAgICBmb3IgKGNvbnN0IGRvY3VtZW50RXZlbnQgb2Ygc2xpY2UuZG9jdW1lbnQpIHtcbiAgICAgICAgICBjb25zdCBraW5kOiBUdXRvcmlhbERvY3VtZW50RXZlbnRLaW5kID0gZG9jdW1lbnRFdmVudC5raW5kO1xuICAgICAgICAgIGlmIChraW5kLmtpbmQgPT09IFwiZWRpdFwiKSB7XG4gICAgICAgICAgICBkb2N1bWVudFRvdWNoZWQgPSB0cnVlO1xuICAgICAgICAgICAgY29uc3Qgb3BlcmF0aW9ucyA9IHNsaWNlLmZvcndhcmQgPyBraW5kLmZvcndhcmRzIDoga2luZC5iYWNrd2FyZHM7XG4gICAgICAgICAgICBpZiAocGx1Z2luPy5hcHBseU9wZXJhdGlvbnMpIGF3YWl0IHBsdWdpbi5hcHBseU9wZXJhdGlvbnMoc2Vzc2lvbi5pbnN0YW5jZUlkLCBlbmNvZGVPcGVyYXRpb25FbnZlbG9wZXNQYWNrKG9wZXJhdGlvbnMpKTtcbiAgICAgICAgICB9IGVsc2UgaWYgKGtpbmQua2luZCA9PT0gXCJsb2FkXCIpIHtcbiAgICAgICAgICAgIGRvY3VtZW50VG91Y2hlZCA9IHRydWU7XG4gICAgICAgICAgICBjb25zdCBkb2N1bWVudEpzb24gPSBzbGljZS5mb3J3YXJkID8ga2luZC5kb2N1bWVudEpzb24gOiBraW5kLnByZXZpb3VzSnNvbjtcbiAgICAgICAgICAgIGlmIChwbHVnaW4/LmxvYWRBcHBEb2N1bWVudCkgYXdhaXQgcGx1Z2luLmxvYWRBcHBEb2N1bWVudChzZXNzaW9uLmluc3RhbmNlSWQsIGRvY3VtZW50SnNvbik7XG4gICAgICAgICAgfVxuICAgICAgICAgIC8vIPCfmqfvuI8gVW5kby9SZWRvL0NoZWNrcG9pbnQvQ2hlY2tvdXRDaGVja3BvaW50L1N3aXRjaEFsdGVybmF0aXZlIGNyb3NzaW5ncyBtaWQtc2VlayBhcmUgYW4gaG9uZXN0XG4gICAgICAgICAgLy8gc2NvcGUgY3V0IGhlcmUgKHJlcGxheWluZyBhIGNyb3NzZWQgaGlzdG9yeSBvcCBvdXQgb2YgaXRzIG5hdHVyYWwgbGl2ZS1kaXNwYXRjaCBvcmRlciBpc1xuICAgICAgICAgIC8vIGFtYmlndW91cyB3aXRob3V0IG1vcmUgVkNTLXNpZGUgaW5mcmFzdHJ1Y3R1cmUpIOKAlCB0aGUgZGlyZWN0b3IncyBwZXItdGljayBmb3J3YXJkIHBsYXliYWNrXG4gICAgICAgICAgLy8gYWJvdmUgc3RpbGwgYXBwbGllcyB0aGVtIGNvcnJlY3RseTsgb25seSBhIGxhcmdlIHNjcnViIGp1bXBpbmcgT1ZFUiBvbmUgb2YgdGhlc2UgZW50cmllcyBtaXNzZXMgaXQuXG4gICAgICAgIH1cbiAgICAgICAgY29uc3QgY2FtZXJhV2luZG93SWRzID0gbmV3IFNldChbLi4uZGVmLmJhc2UuY2FtZXJhcywgLi4uZGVmLnRyYWNrcy5jYW1lcmFdLm1hcCgoa2V5ZnJhbWUpID0+IGtleWZyYW1lLndpbmRvd0lkKSk7XG4gICAgICAgIGZvciAoY29uc3Qgd2luZG93SWQgb2YgY2FtZXJhV2luZG93SWRzKSB7XG4gICAgICAgICAgY29uc3QgcG9zZSA9IHR1dG9yaWFsQ2FtZXJhQXQoZGVmLCB3aW5kb3dJZCwgY2xhbXBlZCk7XG4gICAgICAgICAgaWYgKHBvc2UpIGdldFR1dG9yaWFsQ2FtZXJhRHJpdmVyKHdpbmRvd0lkKT8uc2V0KHBvc2UpO1xuICAgICAgICB9XG4gICAgICAgIHR1dG9yaWFsTGFzdEFwcGxpZWRNc1JlZi5jdXJyZW50ID0gY2xhbXBlZDtcbiAgICAgICAgdHV0b3JpYWxDbG9jay5zZWVrKGNsYW1wZWQpO1xuICAgICAgICBpZiAoZG9jdW1lbnRUb3VjaGVkKSBhd2FpdCByZWZyZXNoVWkoc2Vzc2lvbiwgeyBraW5kOiBcImZ1bGxcIiB9KTtcbiAgICAgICAgY29uc29sZS5sb2coXCJbREVCVUddIHR1dG9yaWFsIHJlYnVpbGRcIiwgeyBhdE1zOiBjbGFtcGVkIH0pO1xuICAgICAgICB0dXRvcmlhbERyaXZlblJlZi5jdXJyZW50ID0gZmFsc2U7XG4gICAgICB9KSgpO1xuICAgIH0sXG4gICAgW2FjdGl2ZVR1dG9yaWFsLCBzZXNzaW9uLCBsb2FkZWRQbHVnaW5zLCB0dXRvcmlhbENsb2NrLCByZWZyZXNoVWldLFxuICApO1xuXG4gIC8qKiDilrbvuI8gUGxheS9wYXVzZSB0b2dnbGUg4oCUIHRoZSBkZXZpYXRpb24tY29udmVyZ2UgcGF0aCAoZGVzaWduIHBvaW50IDYpOiBzbmFwcyBkb2N1bWVudCtVSSB0byB0aGVcbiAgICogY29tcG9zZWQgdGFyZ2V0IGF0IHRoZSBjdXJyZW50IHBsYXloZWFkLCB0d2VlbnMgdGhlIGNhbWVyYSBvdmVyIGBUVVRPUklBTF9DT05WRVJHRV9NU2AgKHJlYWwtdGltZSxcbiAgICogcmF0ZS1pbmRlcGVuZGVudCkgZnJvbSBlYWNoIHdpbmRvdydzIExJVkUgcG9zZSB0byBpdHMgdGFyZ2V0IHBvc2UsIHRoZW4gcmVzdW1lcyB0aGUgY2xvY2suICovXG4gIGNvbnN0IHBsYXlQYXVzZVR1dG9yaWFsID0gdXNlQ2FsbGJhY2soKCkgPT4ge1xuICAgIGlmICghYWN0aXZlVHV0b3JpYWwpIHJldHVybjtcbiAgICBpZiAodHV0b3JpYWxQbGF5aW5nKSB7XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1RVVE9SSUFMX1BMQVlJTkdcIiwgdmFsdWU6IGZhbHNlIH0pO1xuICAgICAgcmV0dXJuO1xuICAgIH1cbiAgICBpZiAodHV0b3JpYWxEZXZpYXRlZCAmJiBzZXNzaW9uKSB7XG4gICAgICBjb25zdCBkZWYgPSBhY3RpdmVUdXRvcmlhbDtcbiAgICAgIGNvbnN0IGF0TXMgPSB0dXRvcmlhbENsb2NrLmdldFRpbWVNcygpO1xuICAgICAgdHV0b3JpYWxEcml2ZW5SZWYuY3VycmVudCA9IHRydWU7XG4gICAgICBhcHBseVR1dG9yaWFsVWlTbmFwc2hvdFRvU2hlbGwoZGlzcGF0Y2gsIGNvbXBvc2VUdXRvcmlhbFVpKGRlZiwgYXRNcyksIHVpQnJpZGdlQ3R4UmVmLmN1cnJlbnQpO1xuICAgICAgY29uc3QgY2FtZXJhV2luZG93SWRzID0gbmV3IFNldChbLi4uZGVmLmJhc2UuY2FtZXJhcywgLi4uZGVmLnRyYWNrcy5jYW1lcmFdLm1hcCgoa2V5ZnJhbWUpID0+IGtleWZyYW1lLndpbmRvd0lkKSk7XG4gICAgICBjb25zdCBzdGFydFBvc2VCeVdpbmRvdyA9IG5ldyBNYXA8c3RyaW5nLCBUdXRvcmlhbENhbWVyYVN0YXRlPigpO1xuICAgICAgZm9yIChjb25zdCB3aW5kb3dJZCBvZiBjYW1lcmFXaW5kb3dJZHMpIHtcbiAgICAgICAgY29uc3QgbGl2ZSA9IGdldFR1dG9yaWFsQ2FtZXJhRHJpdmVyKHdpbmRvd0lkKT8uZ2V0KCk7XG4gICAgICAgIGlmIChsaXZlKSBzdGFydFBvc2VCeVdpbmRvdy5zZXQod2luZG93SWQsIGxpdmUpO1xuICAgICAgfVxuICAgICAgY29uc3Qgc3RhcnRlZEF0ID0gcGVyZm9ybWFuY2Uubm93KCk7XG4gICAgICBjb25zdCB0d2VlbiA9IChub3c6IG51bWJlcikgPT4ge1xuICAgICAgICBjb25zdCBwcm9ncmVzcyA9IE1hdGgubWluKDEsIChub3cgLSBzdGFydGVkQXQpIC8gVFVUT1JJQUxfQ09OVkVSR0VfTVMpO1xuICAgICAgICBmb3IgKGNvbnN0IHdpbmRvd0lkIG9mIGNhbWVyYVdpbmRvd0lkcykge1xuICAgICAgICAgIGNvbnN0IHRhcmdldFBvc2UgPSB0dXRvcmlhbENhbWVyYUF0KGRlZiwgd2luZG93SWQsIGF0TXMpO1xuICAgICAgICAgIGlmICghdGFyZ2V0UG9zZSkgY29udGludWU7XG4gICAgICAgICAgY29uc3QgZHJpdmVyID0gZ2V0VHV0b3JpYWxDYW1lcmFEcml2ZXIod2luZG93SWQpO1xuICAgICAgICAgIGlmICghZHJpdmVyKSBjb250aW51ZTtcbiAgICAgICAgICBjb25zdCBzdGFydFBvc2UgPSBzdGFydFBvc2VCeVdpbmRvdy5nZXQod2luZG93SWQpO1xuICAgICAgICAgIGlmIChzdGFydFBvc2UgJiYgc3RhcnRQb3NlLmtpbmQgPT09IHRhcmdldFBvc2Uua2luZCkge1xuICAgICAgICAgICAgZHJpdmVyLnNldChpbnRlcnBvbGF0ZVR1dG9yaWFsQ2FtZXJhKHsgYXQ6IDAsIHdpbmRvd0lkLCBjYW1lcmE6IHN0YXJ0UG9zZSwgZWFzaW5nOiBcImxpbmVhclwiIH0sIHsgYXQ6IFRVVE9SSUFMX0NPTlZFUkdFX01TLCB3aW5kb3dJZCwgY2FtZXJhOiB0YXJnZXRQb3NlLCBlYXNpbmc6IFwibGluZWFyXCIgfSwgcHJvZ3Jlc3MgKiBUVVRPUklBTF9DT05WRVJHRV9NUykpO1xuICAgICAgICAgIH0gZWxzZSB7XG4gICAgICAgICAgICBkcml2ZXIuc2V0KHRhcmdldFBvc2UpO1xuICAgICAgICAgIH1cbiAgICAgICAgfVxuICAgICAgICBpZiAocHJvZ3Jlc3MgPCAxKSByZXF1ZXN0QW5pbWF0aW9uRnJhbWUodHdlZW4pO1xuICAgICAgICBlbHNlIHtcbiAgICAgICAgICB0dXRvcmlhbERyaXZlblJlZi5jdXJyZW50ID0gZmFsc2U7XG4gICAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9UVVRPUklBTF9ERVZJQVRFRFwiLCB2YWx1ZTogZmFsc2UgfSk7XG4gICAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9UVVRPUklBTF9QTEFZSU5HXCIsIHZhbHVlOiB0cnVlIH0pO1xuICAgICAgICB9XG4gICAgICB9O1xuICAgICAgcmVxdWVzdEFuaW1hdGlvbkZyYW1lKHR3ZWVuKTtcbiAgICAgIHJldHVybjtcbiAgICB9XG4gICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9UVVRPUklBTF9QTEFZSU5HXCIsIHZhbHVlOiB0cnVlIH0pO1xuICB9LCBbYWN0aXZlVHV0b3JpYWwsIHR1dG9yaWFsUGxheWluZywgdHV0b3JpYWxEZXZpYXRlZCwgc2Vzc2lvbiwgdHV0b3JpYWxDbG9ja10pO1xuXG4gIGNvbnN0IHN0YXJ0VHV0b3JpYWwgPSB1c2VDYWxsYmFjayhcbiAgICAodHV0b3JpYWxJZDogc3RyaW5nKSA9PiB7XG4gICAgICBpZiAoIWFjdGl2ZVR1dG9yaWFscy5zb21lKCh0dXRvcmlhbCkgPT4gdHV0b3JpYWwuaWQgPT09IHR1dG9yaWFsSWQpKSByZXR1cm47XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1RVVE9SSUFMXCIsIHZhbHVlOiB0dXRvcmlhbElkIH0pO1xuICAgIH0sXG4gICAgW2FjdGl2ZVR1dG9yaWFsc10sXG4gICk7XG4gIGNvbnN0IHN0b3BUdXRvcmlhbCA9IHVzZUNhbGxiYWNrKCgpID0+IHtcbiAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1RVVE9SSUFMXCIsIHZhbHVlOiBudWxsIH0pO1xuICB9LCBbXSk7XG5cbiAgLyoqIOKPuu+4jyBBcm1zL2Rpc2FybXMgYFR1dG9yaWFsUmVjb3JkZXJgIGFnYWluc3QgdGhlIExJVkUgKG5ldmVyIHNhbmRib3hlZCkgZG9jdW1lbnQg4oCUIGEgcmVjb3JkaW5nIElTIHRoZVxuICAgKiB1c2VyJ3Mgd29yay4gT24gc3RvcDogbGlnaHQgYHZhbGlkYXRlVHV0b3JpYWxgIHNhbml0eSBjaGVjaywgdGhlbiBzZXJpYWxpemUgKyB0cmlnZ2VyIGEgYnJvd3NlclxuICAgKiBkb3dubG9hZCwgbWF0Y2hpbmcgdGhlIHJlcG8ncyBleGlzdGluZyBtZWRpYS1leHBvcnQgZG93bmxvYWQgcGF0dGVybi4gKi9cbiAgY29uc3QgdG9nZ2xlVHV0b3JpYWxSZWNvcmRpbmcgPSB1c2VDYWxsYmFjaygoKSA9PiB7XG4gICAgaWYgKCFzZXNzaW9uKSByZXR1cm47XG4gICAgY29uc3QgcmVjb3JkZXIgPSB0dXRvcmlhbFJlY29yZGVyUmVmLmN1cnJlbnQ7XG4gICAgaWYgKHJlY29yZGVyKSB7XG4gICAgICB0dXRvcmlhbFJlY29yZGVyUmVmLmN1cnJlbnQgPSBudWxsO1xuICAgICAgY29uc3QgaWQgPSBgcmVjb3JkZWQtJHtzZXNzaW9uLmFwcC5pZH0tJHtEYXRlLm5vdygpfWA7XG4gICAgICBjb25zdCBkZWYgPSByZWNvcmRlci5idWlsZChpZCwgYCR7c2Vzc2lvbi5hcHAuaWR9IHJlY29yZGluZ2ApO1xuICAgICAgY29uc3QgdmFsaWRhdGlvbkVycm9yID0gdmFsaWRhdGVUdXRvcmlhbChkZWYpO1xuICAgICAgaWYgKHZhbGlkYXRpb25FcnJvcikgY29uc29sZS5lcnJvcihcIltERUJVR10gdHV0b3JpYWwgcmVjb3JkaW5nIHZhbGlkYXRpb24gZmFpbGVkXCIsIHZhbGlkYXRpb25FcnJvcik7XG4gICAgICBjb25zdCBqc29uID0gSlNPTi5zdHJpbmdpZnkoZGVmLCBudWxsLCAyKTtcbiAgICAgIGNvbnNvbGUubG9nKFwiW0RFQlVHXSB0dXRvcmlhbCByZWNvcmRpbmdcIiwganNvbik7XG4gICAgICBkb3dubG9hZE1lZGlhRXhwb3J0KGB0dXRvcmlhbC0ke3Nlc3Npb24uYXBwLmlkfS0ke0RhdGUubm93KCl9Lm9wc2AsIFwidGV4dC9wbGFpblwiLCBqc29uKTtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfVFVUT1JJQUxfUkVDT1JESU5HXCIsIHZhbHVlOiBmYWxzZSB9KTtcbiAgICAgIHJldHVybjtcbiAgICB9XG4gICAgdm9pZCAoYXN5bmMgKCkgPT4ge1xuICAgICAgY29uc3QgcGx1Z2luID0gbG9hZGVkUGx1Z2lucy5maW5kKChlbnRyeSkgPT4gZW50cnkuaGFuZGxlLnBsdWdpbklkID09PSBzZXNzaW9uLnBsdWdpbklkKT8uaGFuZGxlO1xuICAgICAgbGV0IGRvY3VtZW50SnNvbjogc3RyaW5nIHwgbnVsbCA9IG51bGw7XG4gICAgICB0cnkge1xuICAgICAgICBpZiAocGx1Z2luPy5yZWFkQXBwRG9jdW1lbnQpIGRvY3VtZW50SnNvbiA9IGF3YWl0IHBsdWdpbi5yZWFkQXBwRG9jdW1lbnQoc2Vzc2lvbi5pbnN0YW5jZUlkKTtcbiAgICAgIH0gY2F0Y2ggKGNhcHR1cmVFcnJvcikge1xuICAgICAgICBjb25zb2xlLmVycm9yKFwiW0RFQlVHXSB0dXRvcmlhbCByZWNvcmRlciBiYXNlIGNhcHR1cmUgZmFpbGVkXCIsIGNhcHR1cmVFcnJvcik7XG4gICAgICB9XG4gICAgICB0dXRvcmlhbFJlY29yZGVyUmVmLmN1cnJlbnQgPSBuZXcgVHV0b3JpYWxSZWNvcmRlcihjYXB0dXJlVHV0b3JpYWxVaVNuYXBzaG90KHNoZWxsU3RhdGVSZWYuY3VycmVudCwgc2Vzc2lvbiksIGRvY3VtZW50SnNvbik7XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1RVVE9SSUFMX1JFQ09SRElOR1wiLCB2YWx1ZTogdHJ1ZSB9KTtcbiAgICB9KSgpO1xuICB9LCBbc2Vzc2lvbiwgbG9hZGVkUGx1Z2luc10pO1xuXG4gIHVzZUVmZmVjdCgoKSA9PiB7XG4gICAgc3RhcnRUdXRvcmlhbFJlZi5jdXJyZW50ID0gc3RhcnRUdXRvcmlhbDtcbiAgICBzdG9wVHV0b3JpYWxSZWYuY3VycmVudCA9IHN0b3BUdXRvcmlhbDtcbiAgICB0b2dnbGVUdXRvcmlhbFJlY29yZGluZ1JlZi5jdXJyZW50ID0gdG9nZ2xlVHV0b3JpYWxSZWNvcmRpbmc7XG4gIH0sIFtzdGFydFR1dG9yaWFsLCBzdG9wVHV0b3JpYWwsIHRvZ2dsZVR1dG9yaWFsUmVjb3JkaW5nXSk7XG5cbiAgLy8g4o+677iPIFJlY29yZGVyOiBVSS1zdGF0ZSBkaWZmIG9uIGV2ZXJ5IGBTaGVsbFN0YXRlYCBjaGFuZ2UgKGNhdGNoZXMgcGFuZWwtdGFiIGNsaWNrcy90cmVlIGV4cGFuZHMvZXRjLlxuICAvLyB0aGF0IGJ5cGFzcyBgb25BY3Rpb25gKSwgYSBwZXJpb2RpYyBmdWxsLXNuYXBzaG90IGtleWZyYW1lIGV2ZXJ5IDVzLCBhbmQgYSAxMEh6IGVwc2lsb24tZmlsdGVyZWRcbiAgLy8gY2FtZXJhIHNhbXBsZXIgcGVyIHJlZ2lzdGVyZWQgZHJpdmVyICh3b3JsZCBkcmFncyBieXBhc3MgYG9uQWN0aW9uYCBlbnRpcmVseSkuXG4gIHVzZUVmZmVjdCgoKSA9PiB7XG4gICAgaWYgKCF0dXRvcmlhbFJlY29yZGluZykgcmV0dXJuO1xuICAgIHR1dG9yaWFsUmVjb3JkZXJSZWYuY3VycmVudD8ucmVjb3JkVWlEaWZmKGNhcHR1cmVUdXRvcmlhbFVpU25hcHNob3Qoc2hlbGxTdGF0ZSwgc2Vzc2lvbikpO1xuICB9LCBbdHV0b3JpYWxSZWNvcmRpbmcsIHNoZWxsU3RhdGUsIHNlc3Npb25dKTtcblxuICB1c2VFZmZlY3QoKCkgPT4ge1xuICAgIGlmICghdHV0b3JpYWxSZWNvcmRpbmcgfHwgIXNlc3Npb24gfHwgdHlwZW9mIHdpbmRvdyA9PT0gXCJ1bmRlZmluZWRcIikgcmV0dXJuO1xuICAgIGNvbnN0IGludGVydmFsID0gd2luZG93LnNldEludGVydmFsKCgpID0+IHtcbiAgICAgIHR1dG9yaWFsUmVjb3JkZXJSZWYuY3VycmVudD8ucmVjb3JkU25hcHNob3QoY2FwdHVyZVR1dG9yaWFsVWlTbmFwc2hvdChzaGVsbFN0YXRlUmVmLmN1cnJlbnQsIHNlc3Npb24pKTtcbiAgICB9LCA1MDAwKTtcbiAgICByZXR1cm4gKCkgPT4gd2luZG93LmNsZWFySW50ZXJ2YWwoaW50ZXJ2YWwpO1xuICB9LCBbdHV0b3JpYWxSZWNvcmRpbmcsIHNlc3Npb25dKTtcblxuICB1c2VFZmZlY3QoKCkgPT4ge1xuICAgIGlmICghdHV0b3JpYWxSZWNvcmRpbmcgfHwgIXNlc3Npb24gfHwgdHlwZW9mIHdpbmRvdyA9PT0gXCJ1bmRlZmluZWRcIikgcmV0dXJuO1xuICAgIGNvbnN0IGludGVydmFsID0gd2luZG93LnNldEludGVydmFsKCgpID0+IHtcbiAgICAgIGNvbnN0IHJlY29yZGVyID0gdHV0b3JpYWxSZWNvcmRlclJlZi5jdXJyZW50O1xuICAgICAgaWYgKCFyZWNvcmRlcikgcmV0dXJuO1xuICAgICAgZm9yIChjb25zdCBpbnN0YW5jZSBvZiBzZXNzaW9uV2luZG93SW5zdGFuY2VzKHNlc3Npb24uYXBwLCBleHRyYVdpbmRvd0luc3RhbmNlc1JlZi5jdXJyZW50KSkge1xuICAgICAgICBjb25zdCBwb3NlID0gZ2V0VHV0b3JpYWxDYW1lcmFEcml2ZXIoaW5zdGFuY2UuaWQpPy5nZXQoKTtcbiAgICAgICAgaWYgKHBvc2UpIHJlY29yZGVyLnNhbXBsZUNhbWVyYShpbnN0YW5jZS5pZCwgcG9zZSk7XG4gICAgICB9XG4gICAgfSwgMTAwKTtcbiAgICByZXR1cm4gKCkgPT4gd2luZG93LmNsZWFySW50ZXJ2YWwoaW50ZXJ2YWwpO1xuICB9LCBbdHV0b3JpYWxSZWNvcmRpbmcsIHNlc3Npb25dKTtcblxuICBjb25zdCBhZGRUdXRvcmlhbENoYXB0ZXIgPSB1c2VDYWxsYmFjaygoKSA9PiB7XG4gICAgdHV0b3JpYWxSZWNvcmRlclJlZi5jdXJyZW50Py5hZGRDaGFwdGVyKCk7XG4gIH0sIFtdKTtcblxuICBjb25zdCB0dXRvcmlhbENoYXB0ZXJNYXJrZXJzID0gdXNlTWVtbyhcbiAgICAoKTogcmVhZG9ubHkgVHV0b3JpYWxDaGFwdGVyTWFya2VyW10gPT4gKGFjdGl2ZVR1dG9yaWFsID8gYWN0aXZlVHV0b3JpYWwuY2hhcHRlcnMubWFwKChjaGFwdGVyKSA9PiAoeyBpZDogY2hhcHRlci5pZCwgdGl0bGU6IHJlc29sdmVNYW5pZmVzdExhYmVsKGNoYXB0ZXIudGl0bGUsIHVpVGVybWlub2xvZ3ksIHVpTG9jYWxlKSwgYXRNczogY2hhcHRlci5hdCB9KSkgOiBbXSksXG4gICAgW2FjdGl2ZVR1dG9yaWFsLCB1aVRlcm1pbm9sb2d5LCB1aUxvY2FsZV0sXG4gICk7XG4gIC8vI2VuZHJlZ2lvbiDwn46l77iPVHV0b3JpYWxPcmNoZXN0cmF0aW9uXG5cbiAgY29uc3Qgc3R1ZGlvU2Vzc2lvbkFjdGl2ZSA9IHN0dWRpb01vZGUgJiYgc2Vzc2lvbj8uYXBwLmlkID09PSBob3N0QXBwSWQ7XG4gIC8vIPCfj6DvuI/wn6ez77iPIE9uY2UgYHN0dWRpb1Nlc3Npb25BY3RpdmVgIGlzIHRydWUsIGBzZXNzaW9uLmFwcGAgKmlzKiB0aGUgaG9zdCBhcHAsIHNvIGl0cyBvd24gc2VsZi1kZWNsYXJlZFxuICAvLyBgY29udHJvbGxlcklkYCBpcyB0aGUgcmlnaHQgdmFsdWUg4oCUIG5vIHNlcGFyYXRlIGFwcC1pZGVudGl0eSBsb29rdXAgbmVlZGVkLlxuICBjb25zdCBzdHVkaW9TZXNzaW9uQ29udHJvbGxlcklkID0gc3R1ZGlvU2Vzc2lvbkFjdGl2ZSA/IHNlc3Npb24/LmFwcC5jb250cm9sbGVySWQgOiB1bmRlZmluZWQ7XG4gIHVzZUVmZmVjdCgoKSA9PiB7XG4gICAgaWYgKCFzdHVkaW9TZXNzaW9uQWN0aXZlIHx8ICFzdHVkaW9TZXNzaW9uQ29udHJvbGxlcklkIHx8IHR5cGVvZiB3aW5kb3cgPT09IFwidW5kZWZpbmVkXCIpIHJldHVybjtcbiAgICBjb25zdCBpZGVudGl0eSA9IHByZXNlbmNlQ2xpZW50SWRlbnRpdHkoZXBoZW1lcmFsKTtcbiAgICBjb25zdCBiZWF0ID0gKCkgPT4gb25BY3Rpb25SZWYuY3VycmVudCh7IGNvbnRyb2xsZXJJZDogc3R1ZGlvU2Vzc2lvbkNvbnRyb2xsZXJJZCwgYWN0aW9uOiBcInByZXNlbmNlSGVhcnRiZWF0XCIsIGFyZ3M6IGlkZW50aXR5IH0pO1xuICAgIGNvbnN0IGluaXRpYWwgPSB3aW5kb3cuc2V0VGltZW91dChiZWF0LCAxMDAwKTtcbiAgICBjb25zdCB0aW1lciA9IHdpbmRvdy5zZXRJbnRlcnZhbChiZWF0LCBQUkVTRU5DRV9IRUFSVEJFQVRfSU5URVJWQUxfTVMpO1xuICAgIHJldHVybiAoKSA9PiB7XG4gICAgICB3aW5kb3cuY2xlYXJUaW1lb3V0KGluaXRpYWwpO1xuICAgICAgd2luZG93LmNsZWFySW50ZXJ2YWwodGltZXIpO1xuICAgIH07XG4gIH0sIFtzdHVkaW9TZXNzaW9uQWN0aXZlLCBzdHVkaW9TZXNzaW9uQ29udHJvbGxlcklkLCBlcGhlbWVyYWxdKTtcblxuICB1c2VQYW5lbENocm9tZUhvdGtleXMoe1xuICAgIC8vIPCfk7HvuI8gQWxsIGVpZ2h0IGFuY2hvciBob3RrZXlzIGNvbGxhcHNlIG9udG8gdGhlIHNpbmdsZSBtb2JpbGUgcGFuZWwgdG9nZ2xlIG9uIG1vYmlsZS4gU2FtZSBgc2hlbGwucGFuZWxUb2dnbGVgXG4gICAgLy8gY29tbWFuZElkIGFzIHRoZSBtb3VzZS1kcml2ZW4gdG9nZ2xlIGluIGBidWlsZFBhbmVsU2VsZWN0aW9uUHJvcHNgIChzbyBrZXlib2FyZC9tb3VzZSBmb2xkIHRvZ2V0aGVyKSxcbiAgICAvLyBmbGFnZ2VkIGBob3RrZXk6IHRydWVgIGluIGRldGFpbC5cbiAgICBvblRvZ2dsZTogKGFuY2hvcikgPT4ge1xuICAgICAgaWYgKG1vYmlsZSkgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9NT0JJTEVfUEFORUxfVklTSUJMRVwiLCB2YWx1ZTogKHZpc2libGUpID0+ICF2aXNpYmxlIH0pO1xuICAgICAgZWxzZSBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1BBTkVMX1ZJU0lCTEVcIiwgYW5jaG9yLCB2YWx1ZTogKHZpc2libGUpID0+ICF2aXNpYmxlIH0pO1xuICAgICAgbm90ZVNoZWxsQ29tbWFuZChcInNoZWxsLnBhbmVsVG9nZ2xlXCIsIHNoZWxsTGFiZWwoXCJ1aS5zaGVsbENvbW1hbmQucGFuZWxUb2dnbGVcIiksIHsgYW5jaG9yOiBtb2JpbGUgPyB1bmRlZmluZWQgOiBhbmNob3IsIGhvdGtleTogdHJ1ZSB9KTtcbiAgICB9LFxuICB9KTtcblxuICB1c2VFbGVtZW50c1N1cmZhY2VDaHJvbWUoeyBhcHBlYXJhbmNlOiB1aUFwcGVhcmFuY2UsIGRldmljZTogdWlEZXZpY2UsIGRyaXZlcjogdWlEcml2ZXIgfSwgc2NvcGUucm9vdFJlZi5jdXJyZW50ID8/IHVuZGVmaW5lZCk7XG5cbiAgLy8jcmVnaW9uIPCfkr7vuI8gdWlQcmVmcyBwZXJzaXN0ZW5jZSAoc2tpcHMgd3JpdGVzIGZvciBhbnkgbG9ja2VkIHByZWZlcmVuY2U7IGFuIGVwaGVtZXJhbCBicmFuZCdzXG4gIC8vIGBzY29wZS5zdG9yYWdlYCBpcyBhbHJlYWR5IGFuIGluLW1lbW9yeSBwb3J0LCBzbyB0aGUgd3JpdGVzIGJlbG93IGFyZSBoYXJtbGVzcyB0aGVyZSB0b28g4oCUIG5vIG1vcmVcbiAgLy8gYGVwaGVtZXJhbGAgYnJhbmNoIG5lZWRlZCB0byBza2lwIHRoZW0gb3V0cmlnaHQpXG4gIHVzZUVmZmVjdCgoKSA9PiB7XG4gICAgaWYgKCFsb2Nrcy5hcHBlYXJhbmNlKSB3cml0ZVN0b3JlZFVpQ2hyb21lQXBwZWFyYW5jZShzY29wZS5zdG9yYWdlLCB1aUFwcGVhcmFuY2UpO1xuICAgIHdyaXRlU3RvcmVkVWlDaHJvbWVMYXlvdXQoc2NvcGUuc3RvcmFnZSwgdWlMYXlvdXQpO1xuICAgIHdyaXRlU3RvcmVkVWlEcml2ZXJJZChzY29wZS5zdG9yYWdlLCB1aURyaXZlcklkKTtcbiAgICB3cml0ZVN0b3JlZFVpQ3VzdG9tRHJpdmVycyhzY29wZS5zdG9yYWdlLCB1aUN1c3RvbURyaXZlcnMpO1xuICAgIHdyaXRlU3RvcmVkVWlLZXliaW5kaW5nT3ZlcnJpZGVzKHNjb3BlLnN0b3JhZ2UsIHVpS2V5YmluZGluZ092ZXJyaWRlcyk7XG4gICAgaWYgKCFsb2Nrcy5sb2NhbGUpIHdyaXRlU3RvcmVkVWlDaHJvbWVMb2NhbGUoc2NvcGUuc3RvcmFnZSwgdWlMb2NhbGUpO1xuICAgIC8vIPCfkJrvuI8gVGhpcyBzaGVsbCdzIG93biBpMThuZXh0IGluc3RhbmNlIChub3QgdGhlIHNoYXJlZCBgdWlJMThuYCBzaW5nbGV0b24pIOKAlCBhbmQgaXRzIG93biByb290J3NcbiAgICAvLyBgbGFuZ2AgYXR0cmlidXRlOyBgZG9jdW1lbnQuZG9jdW1lbnRFbGVtZW50LmxhbmdgIHN0YXlzIHJlc2VydmVkIGZvciB0aGUgcGFnZS1vd25pbmcgY2FzZS5cbiAgICB2b2lkIHNjb3BlLmkxOG4uY2hhbmdlTGFuZ3VhZ2UodWlMb2NhbGUpO1xuICAgIGlmIChzY29wZS5vd25zUGFnZSkge1xuICAgICAgaWYgKHR5cGVvZiBkb2N1bWVudCAhPT0gXCJ1bmRlZmluZWRcIikgZG9jdW1lbnQuZG9jdW1lbnRFbGVtZW50LmxhbmcgPSB1aUxvY2FsZTtcbiAgICB9IGVsc2UgaWYgKHNjb3BlLnJvb3RSZWYuY3VycmVudCkge1xuICAgICAgc2NvcGUucm9vdFJlZi5jdXJyZW50LmxhbmcgPSB1aUxvY2FsZTtcbiAgICB9XG4gICAgaWYgKCFsb2Nrcy50ZXJtaW5vbG9neSkgd3JpdGVTdG9yZWRVaUNocm9tZVRlcm1pbm9sb2d5KHNjb3BlLnN0b3JhZ2UsIHVpVGVybWlub2xvZ3kpO1xuICAgIC8vIPCfkJrvuI8gYHNldEFjdGl2ZVVpVGhlbWVgIGlzIHBhZ2UtZ2xvYmFsICh3cml0ZXMgYGRvY3VtZW50LmRvY3VtZW50RWxlbWVudGAncyBDU1MgdmFycykg4oCUIGNvcnJlY3Qgb25seVxuICAgIC8vIGZvciB0aGUgcGFnZS1vd25pbmcgc2hlbGwuIEEgY28tbW91bnRlZCBlbWJlZGRlZCBzaGVsbCBwYWludHMgaXRzIG93biB0aGVtZSB0b2tlbnMgb250byBpdHMgb3duXG4gICAgLy8gYC5zZW1pby1zY29wZWAgcm9vdCBpbnN0ZWFkLCB2aWEgYGFwcGx5VWlUaGVtZVRvUm9vdGAsIHNvIHR3byBzaGVsbHMgd2l0aCBkaWZmZXJlbnQgYHRoZW1lSWRgIGxvY2tzXG4gICAgLy8gbmV2ZXIgZmlnaHQgb3ZlciB0aGUgc2FtZSBkb2N1bWVudC13aWRlIHRva2Vucy5cbiAgICBpZiAoc2NvcGUub3duc1BhZ2UpIHtcbiAgICAgIHNldEFjdGl2ZVVpVGhlbWUodWlUaGVtZSk7XG4gICAgfSBlbHNlIGlmIChzY29wZS5yb290UmVmLmN1cnJlbnQpIHtcbiAgICAgIGFwcGx5VWlUaGVtZVRvUm9vdChzY29wZS5yb290UmVmLmN1cnJlbnQsIHVpVGhlbWUpO1xuICAgIH1cbiAgICBpZiAoIWxvY2tzLnRoZW1lSWQpIHtcbiAgICAgIHdyaXRlU3RvcmVkVWlDaHJvbWVUaGVtZVNuYXBzaG90KHNjb3BlLnN0b3JhZ2UsIHVpVGhlbWUpO1xuICAgICAgd3JpdGVTdG9yZWRVaUNocm9tZVRoZW1lSWQoc2NvcGUuc3RvcmFnZSwgdWlUaGVtZUlkKTtcbiAgICB9XG4gICAgd3JpdGVTdG9yZWRVaUN1c3RvbVRoZW1lcyhzY29wZS5zdG9yYWdlLCB1aUN1c3RvbVRoZW1lcyk7XG4gIH0sIFt1aUFwcGVhcmFuY2UsIHVpTGF5b3V0LCB1aURyaXZlcklkLCB1aUN1c3RvbURyaXZlcnMsIHVpS2V5YmluZGluZ092ZXJyaWRlcywgdWlMb2NhbGUsIHVpVGVybWlub2xvZ3ksIHVpVGhlbWUsIHVpVGhlbWVJZCwgdWlDdXN0b21UaGVtZXMsIGxvY2tzLCBzY29wZV0pO1xuXG4gIC8vIPCfkJrvuI8gVW5tb3VudCBjbGVhbnVwIGZvciB0aGUgZW1iZWRkZWQgKG5vbi1wYWdlLW93bmluZykgY2FzZSDigJQgYSBzaGVsbCB0aGF0IHBhaW50ZWQgaXRzIG93biByb290J3NcbiAgLy8gdGhlbWUgdG9rZW5zIG11c3QgcmVtb3ZlIHRoZW0gb24gdW5tb3VudCwgb3IgYSBsYXRlciwgdW5yZWxhdGVkIGVsZW1lbnQgcmV1c2VkIGF0IHRoZSBzYW1lIERPTVxuICAvLyBwb3NpdGlvbiAoUmVhY3Qvdml0ZSBITVIgcmV1c2UsIG9yIGFub3RoZXIgc2hlbGwncyBjYW52YXMtY2xvbmUgYXNzZXRzIGluIGEgZGV2IGhhcm5lc3MpIHdvdWxkXG4gIC8vIHNpbGVudGx5IGluaGVyaXQgYSBzdGFsZSB0aGVtZSdzIGlubGluZSBvdmVycmlkZXMuIFRoZSBwYWdlLW93bmluZyBjYXNlIGlzIGludGVudGlvbmFsbHkgbGVmdCBhbG9uZTpcbiAgLy8gYGRvY3VtZW50LmRvY3VtZW50RWxlbWVudGAgb3V0bGl2ZXMgYW55IHNpbmdsZSBzaGVsbCdzIGxpZmV0aW1lLlxuICB1c2VFZmZlY3QoKCkgPT4ge1xuICAgIGlmIChzY29wZS5vd25zUGFnZSkgcmV0dXJuO1xuICAgIHJldHVybiAoKSA9PiB7XG4gICAgICBpZiAoc2NvcGUucm9vdFJlZi5jdXJyZW50KSBjbGVhclVpVGhlbWVGcm9tUm9vdChzY29wZS5yb290UmVmLmN1cnJlbnQpO1xuICAgIH07XG4gIH0sIFtzY29wZV0pO1xuICAvLyNlbmRyZWdpb25cblxuICB1c2VBY3Rpb25Ib3RrZXkoXG4gICAgXCJ1aS5uYXYuYmFja1wiLFxuICAgIHVzZUNhbGxiYWNrKCgpID0+IHtcbiAgICAgIGlmIChjYW5Hb0JhY2spIGdvQmFjaygpO1xuICAgIH0sIFtjYW5Hb0JhY2ssIGdvQmFja10pLFxuICAgIHVuZGVmaW5lZCxcbiAgICBbY2FuR29CYWNrLCBnb0JhY2tdLFxuICAgIHsgb3ZlcnJpZGVzOiB1aUtleWJpbmRpbmdPdmVycmlkZXMgfSxcbiAgKTtcbiAgdXNlQWN0aW9uSG90a2V5KFxuICAgIFwidWkubmF2LmZvcndhcmRcIixcbiAgICB1c2VDYWxsYmFjaygoKSA9PiB7XG4gICAgICBpZiAoY2FuR29Gb3J3YXJkKSBnb0ZvcndhcmQoKTtcbiAgICB9LCBbY2FuR29Gb3J3YXJkLCBnb0ZvcndhcmRdKSxcbiAgICB1bmRlZmluZWQsXG4gICAgW2NhbkdvRm9yd2FyZCwgZ29Gb3J3YXJkXSxcbiAgICB7IG92ZXJyaWRlczogdWlLZXliaW5kaW5nT3ZlcnJpZGVzIH0sXG4gICk7XG4gIHVzZUFjdGlvbkhvdGtleShcbiAgICBcInVpLm5hdi51cFwiLFxuICAgIHVzZUNhbGxiYWNrKCgpID0+IHtcbiAgICAgIGlmIChjYW5Hb1VwKSBnb1VwKCk7XG4gICAgfSwgW2NhbkdvVXAsIGdvVXBdKSxcbiAgICB1bmRlZmluZWQsXG4gICAgW2NhbkdvVXAsIGdvVXBdLFxuICAgIHsgb3ZlcnJpZGVzOiB1aUtleWJpbmRpbmdPdmVycmlkZXMgfSxcbiAgKTtcbiAgdXNlQWN0aW9uSG90a2V5KFxuICAgIFwidWkuc2VhcmNoLnRvZ2dsZVwiLFxuICAgIHVzZUNhbGxiYWNrKCgpID0+IGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfU0VBUkNIX09QRU5cIiwgdmFsdWU6IChvcGVuKSA9PiAhb3BlbiB9KSwgW10pLFxuICAgIHVuZGVmaW5lZCxcbiAgICBbXSxcbiAgICB7IG92ZXJyaWRlczogdWlLZXliaW5kaW5nT3ZlcnJpZGVzIH0sXG4gICk7XG4gIHVzZUFjdGlvbkhvdGtleShcbiAgICBcInVpLmZpbmQudG9nZ2xlXCIsXG4gICAgdXNlQ2FsbGJhY2soKCkgPT4gZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9GSU5EX09QRU5cIiwgdmFsdWU6IChvcGVuKSA9PiAhb3BlbiB9KSwgW10pLFxuICAgIHVuZGVmaW5lZCxcbiAgICBbXSxcbiAgICB7IG92ZXJyaWRlczogdWlLZXliaW5kaW5nT3ZlcnJpZGVzIH0sXG4gICk7XG5cbiAgY29uc3QgYXBwbHlOYW1lZExheW91dCA9IHVzZUNhbGxiYWNrKFxuICAgIChsYXlvdXQ6IFdpbmRvd0xheW91dCkgPT4ge1xuICAgICAgaWYgKCFzZXNzaW9uKSByZXR1cm47XG4gICAgICBjb25zdCBzZWVkZWQgPSBhcHBseUZyYW1ld29ya0xheW91dFNlZWQobGF5b3V0LCBzZXNzaW9uLmFwcC53aW5kb3dLaW5kcywgYXBwTGFiZWxzT3ZlcmxheSwgdWlUZXJtaW5vbG9neSwgdWlMb2NhbGUpO1xuICAgICAgZXh0cmFXaW5kb3dJbnN0YW5jZXNSZWYuY3VycmVudCA9IHNlZWRlZC5leHRyYUluc3RhbmNlcztcbiAgICAgIGV4dHJhV2luZG93Q291bnRlclJlZi5jdXJyZW50ID0gc2VlZGVkLmV4dHJhSW5zdGFuY2VzLmxlbmd0aDtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfRVhUUkFfV0lORE9XX0lOU1RBTkNFU1wiLCB2YWx1ZTogc2VlZGVkLmV4dHJhSW5zdGFuY2VzIH0pO1xuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9TSEVMTF9MQVlPVVRcIiwgdmFsdWU6IHNlZWRlZC5tb2RlTGF5b3V0IH0pO1xuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9BQ1RJVkVfV0lORE9XX0lEXCIsIHZhbHVlOiBudWxsIH0pO1xuICAgICAgLy8g8J+qn++4jyBIYW5kIHRoZSBqdXN0LWNvbXB1dGVkIGluc3RhbmNlIGxpc3Qgc3RyYWlnaHQgdG8gdGhlIGZldGNoIHJhdGhlciB0aGFuIHJlYWRpbmcgYGV4dHJhV2luZG93SW5zdGFuY2VzYFxuICAgICAgLy8gc3RhdGUgKHdoaWNoIHdvdWxkbid0IHJlZmxlY3QgdGhpcyBkaXNwYXRjaCB1bnRpbCB0aGUgbmV4dCByZW5kZXIpIOKAlCBldmVyeSBuZXdseS1zZWVkZWQgcGFuZSdzIG93blxuICAgICAgLy8gYm9keS9tZWFzdXJlcy9lbmdhZ2VtZW50IGdldHMgZmV0Y2hlZCBpbW1lZGlhdGVseSBpbnN0ZWFkIG9mIHNob3dpbmcgXCJtaXNzaW5nIHdpbmRvd1wiIHVudGlsIGxhdGVyLlxuICAgICAgdm9pZCByZWZyZXNoVWkoc2Vzc2lvbiwgeyBraW5kOiBcImZ1bGxcIiB9LCBzZWVkZWQuZXh0cmFJbnN0YW5jZXMpO1xuICAgIH0sXG4gICAgW3Nlc3Npb24sIGFwcExhYmVsc092ZXJsYXksIHJlZnJlc2hVaSwgdWlUZXJtaW5vbG9neSwgdWlMb2NhbGVdLFxuICApO1xuXG4gIGNvbnN0IGFwcGx5TW9kZUNoYW5nZSA9IHVzZUNhbGxiYWNrKFxuICAgIChtb2RlSWQ6IHN0cmluZykgPT4ge1xuICAgICAgLy8g8J+boO+4jyBUb29scyBhcmUgc2NvcGVkIHRvIGEgbW9kZSDigJQgc3dpdGNoaW5nIG1vZGVzIGFsd2F5cyBjbGVhcnMgdGhlIGFjdGl2ZSB0b29sIChhbmQgZXZlcnlcbiAgICAgIC8vIHdpbmRvdydzIGFjdGl2ZSB1dGlsaXR5KSwgbWlycm9yaW5nIGhvdyBhIGZyZXNoIG1vZGUgc3RhcnRzIHdpdGggbm8gdXRpbGl0eSBwcmVzc2VkIGVpdGhlci5cbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfQUNUSVZFX1RPT0xcIiwgdG9vbElkOiBudWxsIH0pO1xuICAgICAgZGlzcGF0Y2goe1xuICAgICAgICB0eXBlOiBcIlNFVF9TRVNTSU9OXCIsXG4gICAgICAgIHZhbHVlOiAoY3VycmVudCkgPT4ge1xuICAgICAgICAgIGlmICghY3VycmVudCkgcmV0dXJuIGN1cnJlbnQ7XG4gICAgICAgICAgY29uc3QgbGF5b3V0ID0gcmVzb2x2ZUxheW91dEZvck1vZGUoY3VycmVudC5hcHAsIG1vZGVJZCk7XG4gICAgICAgICAgY29uc3QgbmV4dFNlc3Npb246IEFjdGl2ZVNlc3Npb24gPSB7IC4uLmN1cnJlbnQsIHZpZXdTdGF0ZTogeyAuLi5jdXJyZW50LnZpZXdTdGF0ZSwgYWN0aXZlTW9kZUlkOiBtb2RlSWQsIGFjdGl2ZVRvb2xJZDogdW5kZWZpbmVkIH0gfTtcbiAgICAgICAgICBpZiAobGF5b3V0KSB7XG4gICAgICAgICAgICBjb25zdCBzZWVkZWQgPSBhcHBseUZyYW1ld29ya0xheW91dFNlZWQobGF5b3V0LCBjdXJyZW50LmFwcC53aW5kb3dLaW5kcywgYXBwTGFiZWxzT3ZlcmxheSwgdWlUZXJtaW5vbG9neSwgdWlMb2NhbGUpO1xuICAgICAgICAgICAgZXh0cmFXaW5kb3dJbnN0YW5jZXNSZWYuY3VycmVudCA9IHNlZWRlZC5leHRyYUluc3RhbmNlcztcbiAgICAgICAgICAgIGV4dHJhV2luZG93Q291bnRlclJlZi5jdXJyZW50ID0gc2VlZGVkLmV4dHJhSW5zdGFuY2VzLmxlbmd0aDtcbiAgICAgICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfRVhUUkFfV0lORE9XX0lOU1RBTkNFU1wiLCB2YWx1ZTogc2VlZGVkLmV4dHJhSW5zdGFuY2VzIH0pO1xuICAgICAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9TSEVMTF9MQVlPVVRcIiwgdmFsdWU6IHNlZWRlZC5tb2RlTGF5b3V0IH0pO1xuICAgICAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9BQ1RJVkVfV0lORE9XX0lEXCIsIHZhbHVlOiBudWxsIH0pO1xuICAgICAgICAgICAgdm9pZCByZWZyZXNoVWkobmV4dFNlc3Npb24sIHsga2luZDogXCJmdWxsXCIgfSwgc2VlZGVkLmV4dHJhSW5zdGFuY2VzKTtcbiAgICAgICAgICB9XG4gICAgICAgICAgcmV0dXJuIG5leHRTZXNzaW9uO1xuICAgICAgICB9LFxuICAgICAgfSk7XG4gICAgfSxcbiAgICBbYXBwTGFiZWxzT3ZlcmxheSwgcmVmcmVzaFVpLCB1aVRlcm1pbm9sb2d5LCB1aUxvY2FsZV0sXG4gICk7XG5cbiAgY29uc3QgaGFuZGxlVGVtcGxhdGVEcm9wID0gdXNlQ2FsbGJhY2soXG4gICAgKHBheWxvYWQ6IFdpbmRvd1RlbXBsYXRlRHJvcFBheWxvYWQsIHRhcmdldDogTW9kZUNhbnZhc0Ryb3BUYXJnZXQpID0+IHtcbiAgICAgIGlmICghc2Vzc2lvbikgcmV0dXJuO1xuICAgICAgY29uc3Qga2luZCA9IHNlc3Npb24uYXBwLndpbmRvd0tpbmRzLmZpbmQoKGVudHJ5KSA9PiBlbnRyeS5pZCA9PT0gcGF5bG9hZC53aW5kb3dLaW5kSWQpO1xuICAgICAgaWYgKCFraW5kKSByZXR1cm47XG4gICAgICBleHRyYVdpbmRvd0NvdW50ZXJSZWYuY3VycmVudCArPSAxO1xuICAgICAgY29uc3QgaW5zdGFuY2VJZCA9IGAke3BheWxvYWQud2luZG93S2luZElkfS0ke2V4dHJhV2luZG93Q291bnRlclJlZi5jdXJyZW50fWA7XG4gICAgICBjb25zdCBwcm9qZWN0aW9uU3BlYyA9IGRlY29kZVdvcmxkUHJvamVjdGlvblRlbXBsYXRlSWQocGF5bG9hZC50ZW1wbGF0ZUlkKTtcbiAgICAgIGlmIChwcm9qZWN0aW9uU3BlYykgcmVnaXN0ZXJQZW5kaW5nV29ybGRQcm9qZWN0aW9uKGluc3RhbmNlSWQsIHByb2plY3Rpb25TcGVjKTtcbiAgICAgIGNvbnN0IHRpdGxlID0gcHJvamVjdGlvblNwZWMgPyB3b3JsZFByb2plY3Rpb25TcGVjTGFiZWwocHJvamVjdGlvblNwZWMpIDogcmVzb2x2ZUFwcExhYmVsKGFwcExhYmVsc092ZXJsYXksIFwid2luZG93S2luZFwiLCBraW5kLmlkLCByZXNvbHZlTWFuaWZlc3RMYWJlbChraW5kLmxhYmVsLCB1aVRlcm1pbm9sb2d5LCB1aUxvY2FsZSkpO1xuICAgICAgY29uc3QgbmV4dEV4dHJhSW5zdGFuY2VzID0gWy4uLmV4dHJhV2luZG93SW5zdGFuY2VzUmVmLmN1cnJlbnQsIHsgaWQ6IGluc3RhbmNlSWQsIHdpbmRvd0tpbmRJZDogcGF5bG9hZC53aW5kb3dLaW5kSWQsIHRpdGxlIH1dO1xuICAgICAgZXh0cmFXaW5kb3dJbnN0YW5jZXNSZWYuY3VycmVudCA9IG5leHRFeHRyYUluc3RhbmNlcztcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfRVhUUkFfV0lORE9XX0lOU1RBTkNFU1wiLCB2YWx1ZTogbmV4dEV4dHJhSW5zdGFuY2VzIH0pO1xuICAgICAgaWYgKHByb2plY3Rpb25TcGVjKSB7XG4gICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfV0lORE9XX1RJVExFXCIsIHdpbmRvd0lkOiBpbnN0YW5jZUlkLCB0aXRsZSB9KTtcbiAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9XSU5ET1dfSUNPTlwiLCB3aW5kb3dJZDogaW5zdGFuY2VJZCwgaWNvbklkOiB3b3JsZFByb2plY3Rpb25TcGVjSWNvbklkKHByb2plY3Rpb25TcGVjKSBhcyBJY29uTmFtZSB9KTtcbiAgICAgIH1cbiAgICAgIC8vIPCfqp/vuI8gVGhlIG5ldyBzcGxpdCBwYW5lIGlzIGl0cyBvd24gd2luZG93IGluc3RhbmNlIOKAlCBmZXRjaCBpdHMgYm9keS9tZWFzdXJlcy9lbmdhZ2VtZW50IHJpZ2h0IGF3YXlcbiAgICAgIC8vIChzZWUgYGFwcGx5TmFtZWRMYXlvdXRgJ3MgY29tbWVudCkgcmF0aGVyIHRoYW4gd2FpdGluZyBmb3IgYW4gdW5yZWxhdGVkIGFjdGlvbiB0byB0cmlnZ2VyIGEgcmVmcmVzaC5cbiAgICAgIHZvaWQgcmVmcmVzaFVpKHNlc3Npb24sIHsga2luZDogXCJmdWxsXCIgfSwgbmV4dEV4dHJhSW5zdGFuY2VzKTtcbiAgICAgIGRpc3BhdGNoKHtcbiAgICAgICAgdHlwZTogXCJTRVRfU0hFTExfTEFZT1VUXCIsXG4gICAgICAgIHZhbHVlOiAoY3VycmVudCkgPT4ge1xuICAgICAgICAgIGNvbnN0IGJhc2UgPVxuICAgICAgICAgICAgY3VycmVudCA/P1xuICAgICAgICAgICAgcmVzb2x2ZUZyYW1ld29ya0xheW91dFNlZWQoc2Vzc2lvbi5hcHAuZGVmYXVsdExheW91dCwgc2Vzc2lvbi5hcHAud2luZG93S2luZHMsIGFwcExhYmVsc092ZXJsYXksIHVpVGVybWlub2xvZ3ksIHVpTG9jYWxlKS5tb2RlTGF5b3V0O1xuICAgICAgICAgIHJldHVybiBpbnNlcnRXaW5kb3dBdERyb3Bab25lKGJhc2UsIGluc3RhbmNlSWQsIHRhcmdldCk7XG4gICAgICAgIH0sXG4gICAgICB9KTtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfQUNUSVZFX1dJTkRPV19JRFwiLCB2YWx1ZTogaW5zdGFuY2VJZCB9KTtcbiAgICAgIG5vdGVTaGVsbENvbW1hbmQoXCJzaGVsbC53aW5kb3dTcGxpdFwiLCBzaGVsbExhYmVsKFwidWkuc2hlbGxDb21tYW5kLndpbmRvd1NwbGl0XCIpLCB7IHdpbmRvd0tpbmRJZDogcGF5bG9hZC53aW5kb3dLaW5kSWQsIGluc3RhbmNlSWQgfSk7XG4gICAgfSxcbiAgICBbYXBwTGFiZWxzT3ZlcmxheSwgcmVmcmVzaFVpLCBzZXNzaW9uLCBub3RlU2hlbGxDb21tYW5kLCB1aVRlcm1pbm9sb2d5LCB1aUxvY2FsZV0sXG4gICk7XG5cbiAgY29uc3QgZGlzcGxheUhvc3RSZWYgPSB1c2VSZWY8RGlzcGxheUhvc3RBcGkgfCBudWxsPihudWxsKTtcbiAgY29uc3QgZGlzcGxheUhvc3QgPSB1c2VOYW1lZExheW91dEhvc3Qoe1xuICAgIGFwcElkOiBzZXNzaW9uPy5hcHAuaWQgPz8gXCJmcmFtZXdvcmstb3NcIixcbiAgICB3aW5kb3dLaW5kczogc2Vzc2lvbj8uYXBwLndpbmRvd0tpbmRzLm1hcCgoa2luZCkgPT4gKHsgLi4ua2luZCwgbGFiZWw6IHJlc29sdmVBcHBMYWJlbChhcHBMYWJlbHNPdmVybGF5LCBcIndpbmRvd0tpbmRcIiwga2luZC5pZCwgcmVzb2x2ZU1hbmlmZXN0TGFiZWwoa2luZC5sYWJlbCwgdWlUZXJtaW5vbG9neSwgdWlMb2NhbGUpKSB9KSkgPz8gW10sXG4gICAgYnVpbHRpbkxheW91dHM6IHNlc3Npb24/LmFwcC5uYW1lZExheW91dHMgPz8gW10sXG4gICAgY3VycmVudExheW91dDogY2FwdHVyZUN1cnJlbnRGcmFtZXdvcmtMYXlvdXQoc2hlbGxMYXlvdXQsIGV4dHJhV2luZG93SW5zdGFuY2VzLCBzZXNzaW9uPy5hcHAuZGVmYXVsdExheW91dCksXG4gICAgb25BcHBseUxheW91dDogYXBwbHlOYW1lZExheW91dCxcbiAgICBuYW1lZExheW91dFN0b3JlLFxuICB9KTtcbiAgZGlzcGxheUhvc3RSZWYuY3VycmVudCA9IGRpc3BsYXlIb3N0O1xuXG4gIC8vI3JlZ2lvbiDwn5SW77iPVGhlbWVNdXRhdG9yc1xuICBjb25zdCB1aVRoZW1lQmFzZSA9IHVpVGhlbWVEcmFmdCA/PyB1aVRoZW1lO1xuICBjb25zdCB1aVRoZW1lRGlydHkgPSB1aVRoZW1lRHJhZnQgIT09IG51bGw7XG4gIGNvbnN0IHVpVGhlbWVMaXN0ID0gdXNlTWVtbygoKTogcmVhZG9ubHkgVWlUaGVtZVtdID0+IFsuLi5idWlsdGluVWlUaGVtZXMoKSwgLi4uT2JqZWN0LnZhbHVlcyh1aUN1c3RvbVRoZW1lcyldLCBbdWlDdXN0b21UaGVtZXNdKTtcbiAgY29uc3QgdWlEcml2ZXJMaXN0ID0gdXNlTWVtbygoKTogcmVhZG9ubHkgVWlEcml2ZXJbXSA9PiBbLi4uYnVpbHRpblVpRHJpdmVycygpLCAuLi5PYmplY3QudmFsdWVzKHVpQ3VzdG9tRHJpdmVycyldLCBbdWlDdXN0b21Ecml2ZXJzXSk7XG4gIGNvbnN0IGtleXNCeUFjdGlvbklkID0gdXNlTWVtbygoKSA9PiBidWlsZEtleXNCeUFjdGlvbklkKHNlc3Npb24/LmFwcC5rZXliaW5kaW5ncyA/PyBbXSksIFtzZXNzaW9uPy5hcHAua2V5YmluZGluZ3NdKTtcbiAgY29uc3QgY29udHJvbEtleWJpbmRpbmdzID0gdXNlTWVtbygoKSA9PiBjb21wb3NlQ29udHJvbEtleWJpbmRpbmdzKGtleXNCeUFjdGlvbklkLCB1aUtleWJpbmRpbmdPdmVycmlkZXMpLCBba2V5c0J5QWN0aW9uSWQsIHVpS2V5YmluZGluZ092ZXJyaWRlc10pO1xuICBjb25zdCBvc0NvbW1hbmRzID0gdXNlTWVtbyhcbiAgICAoKSA9PiBidWlsZE9zQ29tbWFuZHModWlUaGVtZUxpc3QsIFtVSV9URVJNSU5PTE9HWV9OQVRJVkUsIC4uLihzZXNzaW9uPy5hcHAudGVybWlub2xvZ2llcyA/PyBbXSldLCBhY3RpdmVJbnRyb2R1Y3Rpb24gIT0gbnVsbCwgbG9ja3MsIHVpRHJpdmVyTGlzdCwgYWN0aXZlVHV0b3JpYWxzLCB0dXRvcmlhbFJlY29yZGVyQXZhaWxhYmxlLCB1aVRlcm1pbm9sb2d5LCB1aUxvY2FsZSksXG4gICAgW3VpVGhlbWVMaXN0LCBzZXNzaW9uPy5hcHAudGVybWlub2xvZ2llcywgYWN0aXZlSW50cm9kdWN0aW9uLCB1aUxvY2FsZSwgdWlUZXJtaW5vbG9neSwgbG9ja3MsIHVpRHJpdmVyTGlzdCwgYWN0aXZlVHV0b3JpYWxzLCB0dXRvcmlhbFJlY29yZGVyQXZhaWxhYmxlXSxcbiAgKTtcblxuICAvKiog8J+nre+4jyBEaXJlY3QgdGhlbWUvYXBwZWFyYW5jZS9sb2NhbGUvdGVybWlub2xvZ3kvZHJpdmVyL2xheW91dCBzZXR0ZXJzIGJlbG93IChzZXR0aW5ncyBwYW5lbCwgdGhlbWUvZHJpdmVyXG4gICAqIGVkaXRvcnMpIGJ5cGFzcyBgZGlzcGF0Y2hPc0NvbW1hbmRgJ3MgbmFtZWQtY29tbWFuZCBwYXRoIGVudGlyZWx5IOKAlCB0aGlzIHJldXNlcyB0aGUgZXhhY3Qgc2FtZSBgb3MuKmBcbiAgICogY29tbWFuZCBpZCAoYW5kIGl0cyBgb3NDb21tYW5kc2AtcmVzb2x2ZWQsIGxvY2FsZS1hZGFwdGVkIGxhYmVsKSBzbyBhIGRpcmVjdC1wYXRoIGNoYW5nZSBmb2xkcyB0b2dldGhlclxuICAgKiB3aXRoIGEgY29tbWFuZC1wYWxldHRlLXRyaWdnZXJlZCBvbmUgaW4gdGhlIGhpc3RvcnkgcGFuZWwgcmVnYXJkbGVzcyBvZiB3aGljaCBwYXRoIHRyaWdnZXJlZCBpdC4gKi9cbiAgY29uc3Qgbm90ZU9zQ29tbWFuZCA9IHVzZUNhbGxiYWNrKFxuICAgIChjb21tYW5kSWQ6IHN0cmluZywgZGV0YWlsPzogUmVjb3JkPHN0cmluZywgdW5rbm93bj4pID0+IHtcbiAgICAgIGNvbnN0IGxhYmVsID0gb3NDb21tYW5kcy5maW5kKChlbnRyeSkgPT4gZW50cnkuaWQgPT09IGNvbW1hbmRJZCk/LmxhYmVsID8/IGNvbW1hbmRJZDtcbiAgICAgIG5vdGVTaGVsbENvbW1hbmQoY29tbWFuZElkLCBsYWJlbCwgZGV0YWlsKTtcbiAgICB9LFxuICAgIFtvc0NvbW1hbmRzLCBub3RlU2hlbGxDb21tYW5kXSxcbiAgKTtcblxuICBjb25zdCBkcmFmdFRoZW1lUGF0Y2ggPSB1c2VDYWxsYmFjayhcbiAgICAocGF0Y2g6IChuZXh0OiBVaVRoZW1lKSA9PiB2b2lkKSA9PiB7XG4gICAgICBjb25zdCBuZXh0ID0gc3RydWN0dXJlZENsb25lKHVpVGhlbWVCYXNlKTtcbiAgICAgIHBhdGNoKG5leHQpO1xuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9VSV9USEVNRV9EUkFGVFwiLCB2YWx1ZTogbmV4dCB9KTtcbiAgICB9LFxuICAgIFt1aVRoZW1lQmFzZV0sXG4gICk7XG5cbiAgY29uc3Qgc2V0VGhlbWVJZCA9IHVzZUNhbGxiYWNrKFxuICAgIChpZDogc3RyaW5nKSA9PiB7XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1VJX1RIRU1FX0RSQUZUXCIsIHZhbHVlOiBudWxsIH0pO1xuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9VSV9USEVNRV9JRFwiLCB2YWx1ZTogaWQgfSk7XG4gICAgICBub3RlT3NDb21tYW5kKFwib3Muc2V0VGhlbWVJZFwiLCB7IHRoZW1lSWQ6IGlkIH0pO1xuICAgIH0sXG4gICAgW25vdGVPc0NvbW1hbmRdLFxuICApO1xuXG4gIGNvbnN0IHNldFRoZW1lQ29sb3IgPSB1c2VDYWxsYmFjayhcbiAgICAoa2V5OiBzdHJpbmcsIGhleDogc3RyaW5nKSA9PlxuICAgICAgZHJhZnRUaGVtZVBhdGNoKChuZXh0KSA9PiB7XG4gICAgICAgIG5leHQuY29sb3JzW2tleV0gPSBoZXg7XG4gICAgICB9KSxcbiAgICBbZHJhZnRUaGVtZVBhdGNoXSxcbiAgKTtcbiAgY29uc3Qgc2V0VGhlbWVTcGFjaW5nID0gdXNlQ2FsbGJhY2soXG4gICAgKGtleTogc3RyaW5nLCB2YWx1ZTogc3RyaW5nKSA9PlxuICAgICAgZHJhZnRUaGVtZVBhdGNoKChuZXh0KSA9PiB7XG4gICAgICAgIG5leHQuc3BhY2luZ1trZXldID0gdmFsdWU7XG4gICAgICB9KSxcbiAgICBbZHJhZnRUaGVtZVBhdGNoXSxcbiAgKTtcbiAgY29uc3Qgc2V0VGhlbWVGb250U3RhY2sgPSB1c2VDYWxsYmFjayhcbiAgICAoa2V5OiBzdHJpbmcsIHZhbHVlOiBzdHJpbmcpID0+XG4gICAgICBkcmFmdFRoZW1lUGF0Y2goKG5leHQpID0+IHtcbiAgICAgICAgbmV4dC5mb250U3RhY2tzW2tleV0gPSB2YWx1ZTtcbiAgICAgIH0pLFxuICAgIFtkcmFmdFRoZW1lUGF0Y2hdLFxuICApO1xuICBjb25zdCBzZXRUaGVtZVN0cm9rZSA9IHVzZUNhbGxiYWNrKFxuICAgIChrZXk6IHN0cmluZywgdmFsdWU6IG51bWJlciB8IG51bWJlcltdKSA9PlxuICAgICAgZHJhZnRUaGVtZVBhdGNoKChuZXh0KSA9PiB7XG4gICAgICAgIG5leHQuc3Ryb2tlc1trZXldID0gdmFsdWU7XG4gICAgICB9KSxcbiAgICBbZHJhZnRUaGVtZVBhdGNoXSxcbiAgKTtcbiAgY29uc3Qgc2V0VGhlbWVSYWRpdXMgPSB1c2VDYWxsYmFjayhcbiAgICAoa2V5OiBzdHJpbmcsIHZhbHVlOiBudW1iZXIpID0+XG4gICAgICBkcmFmdFRoZW1lUGF0Y2goKG5leHQpID0+IHtcbiAgICAgICAgbmV4dC5yYWRpaVtrZXldID0gdmFsdWU7XG4gICAgICB9KSxcbiAgICBbZHJhZnRUaGVtZVBhdGNoXSxcbiAgKTtcbiAgY29uc3Qgc2V0VGhlbWVPcGFjaXR5ID0gdXNlQ2FsbGJhY2soXG4gICAgKGtleTogc3RyaW5nLCB2YWx1ZTogbnVtYmVyKSA9PlxuICAgICAgZHJhZnRUaGVtZVBhdGNoKChuZXh0KSA9PiB7XG4gICAgICAgIG5leHQub3BhY2l0aWVzW2tleV0gPSB2YWx1ZTtcbiAgICAgIH0pLFxuICAgIFtkcmFmdFRoZW1lUGF0Y2hdLFxuICApO1xuICBjb25zdCBzZXRUaGVtZU1ldHJpYyA9IHVzZUNhbGxiYWNrKFxuICAgIChzZWN0aW9uOiBzdHJpbmcsIGtleTogc3RyaW5nLCB2YWx1ZTogbnVtYmVyIHwgbnVtYmVyW10pID0+XG4gICAgICBkcmFmdFRoZW1lUGF0Y2goKG5leHQpID0+IHtcbiAgICAgICAgbmV4dC5tZXRyaWNzW3NlY3Rpb25dID0geyAuLi4obmV4dC5tZXRyaWNzW3NlY3Rpb25dID8/IHt9KSwgW2tleV06IHZhbHVlIH07XG4gICAgICB9KSxcbiAgICBbZHJhZnRUaGVtZVBhdGNoXSxcbiAgKTtcbiAgY29uc3Qgc2V0VGhlbWVBcHBlYXJhbmNlUGFpbnQgPSB1c2VDYWxsYmFjayhcbiAgICAoYXBwZWFyYW5jZTogVGhlbWVBcHBlYXJhbmNlTmFtZSwgZ3JvdXA6IFRoZW1lUGFsZXR0ZUdyb3VwLCBrZXk6IHN0cmluZywgaGV4OiBzdHJpbmcsIGFscGhhPzogbnVtYmVyKSA9PlxuICAgICAgZHJhZnRUaGVtZVBhdGNoKChuZXh0KSA9PiB7XG4gICAgICAgIG5leHQuYXBwZWFyYW5jZXNbYXBwZWFyYW5jZV1bZ3JvdXBdW2tleV0gPSBhbHBoYSA9PT0gdW5kZWZpbmVkID8geyBoZXggfSA6IHsgaGV4LCBhbHBoYSB9O1xuICAgICAgfSksXG4gICAgW2RyYWZ0VGhlbWVQYXRjaF0sXG4gICk7XG5cbiAgY29uc3QgcmVzZXRUaGVtZSA9IHVzZUNhbGxiYWNrKCgpID0+IHtcbiAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1VJX1RIRU1FX0RSQUZUXCIsIHZhbHVlOiBudWxsIH0pO1xuICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfVUlfVEhFTUVfSURcIiwgdmFsdWU6IFwic2VtaW9cIiB9KTtcbiAgfSwgW10pO1xuXG4gIGNvbnN0IHNhdmVUaGVtZSA9IHVzZUNhbGxiYWNrKFxuICAgIChsYWJlbDogc3RyaW5nKSA9PiB7XG4gICAgICBjb25zdCB0cmltbWVkID0gbGFiZWwudHJpbSgpO1xuICAgICAgaWYgKCF0cmltbWVkKSByZXR1cm47XG4gICAgICBjb25zdCBzbHVnID0gdHJpbW1lZFxuICAgICAgICAudG9Mb3dlckNhc2UoKVxuICAgICAgICAucmVwbGFjZSgvW15hLXowLTldKy9nLCBcIi1cIilcbiAgICAgICAgLnJlcGxhY2UoLyheLSt8LSskKS9nLCBcIlwiKTtcbiAgICAgIGlmICghc2x1ZykgcmV0dXJuO1xuICAgICAgY29uc3QgaWQgPSBgY3VzdG9tLiR7c2x1Z31gO1xuICAgICAgY29uc3Qgc2F2ZWQ6IFVpVGhlbWUgPSB7IC4uLnVpVGhlbWVCYXNlLCBpZCwgbGFiZWw6IHRyaW1tZWQgfTtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfVUlfQ1VTVE9NX1RIRU1FU1wiLCB2YWx1ZTogKGN1cnJlbnQpID0+ICh7IC4uLmN1cnJlbnQsIFtpZF06IHNhdmVkIH0pIH0pO1xuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9VSV9USEVNRV9EUkFGVFwiLCB2YWx1ZTogbnVsbCB9KTtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfVUlfVEhFTUVfSURcIiwgdmFsdWU6IGlkIH0pO1xuICAgIH0sXG4gICAgW3VpVGhlbWVCYXNlXSxcbiAgKTtcblxuICBjb25zdCBkZWxldGVUaGVtZSA9IHVzZUNhbGxiYWNrKChpZDogc3RyaW5nKSA9PiB7XG4gICAgaWYgKCFpZC5zdGFydHNXaXRoKFwiY3VzdG9tLlwiKSkgcmV0dXJuO1xuICAgIGRpc3BhdGNoKHtcbiAgICAgIHR5cGU6IFwiU0VUX1VJX0NVU1RPTV9USEVNRVNcIixcbiAgICAgIHZhbHVlOiAoY3VycmVudCkgPT4ge1xuICAgICAgICBjb25zdCB7IFtpZF06IF9yZW1vdmVkLCAuLi5yZXN0IH0gPSBjdXJyZW50O1xuICAgICAgICByZXR1cm4gcmVzdDtcbiAgICAgIH0sXG4gICAgfSk7XG4gICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9VSV9USEVNRV9JRFwiLCB2YWx1ZTogKGN1cnJlbnQpID0+IChjdXJyZW50ID09PSBpZCA/IFwic2VtaW9cIiA6IGN1cnJlbnQpIH0pO1xuICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfVUlfVEhFTUVfRFJBRlRcIiwgdmFsdWU6IG51bGwgfSk7XG4gIH0sIFtdKTtcblxuICBjb25zdCBleHBvcnRUaGVtZSA9IHVzZUNhbGxiYWNrKCgpID0+IHtcbiAgICBkb3dubG9hZE1lZGlhRXhwb3J0KGAke3VpVGhlbWVCYXNlLmlkfS50aGVtZS5kc2xgLCBcInRleHQvcGxhaW5cIiwgc2VyaWFsaXplVWlUaGVtZSh1aVRoZW1lQmFzZSkpO1xuICB9LCBbdWlUaGVtZUJhc2VdKTtcblxuICBjb25zdCBpbXBvcnRUaGVtZSA9IHVzZUNhbGxiYWNrKGFzeW5jICgpID0+IHtcbiAgICBjb25zdCBvcGVuZWQgPSAoYXdhaXQgcmVxdWVzdEZpbGVPcGVuKFwiLnRoZW1lLmRzbCwuZHNsLHRleHQvcGxhaW5cIikpWzBdO1xuICAgIGlmICghb3BlbmVkKSByZXR1cm47XG4gICAgdHJ5IHtcbiAgICAgIGNvbnN0IHBhcnNlZCA9IHBhcnNlVWlUaGVtZShKU09OLnBhcnNlKG9wZW5lZC5jb250ZW50cykpO1xuICAgICAgc2F2ZVRoZW1lKHBhcnNlZC5sYWJlbCB8fCBwYXJzZWQuaWQpO1xuICAgIH0gY2F0Y2gge1xuICAgICAgLyogaW52YWxpZCB0aGVtZSBmaWxlLCBpZ25vcmUgKi9cbiAgICB9XG4gIH0sIFtzYXZlVGhlbWVdKTtcbiAgLy8jZW5kcmVnaW9uIPCflJbvuI9UaGVtZU11dGF0b3JzXG5cbiAgLy8jcmVnaW9uIPCfmpfvuI9Ecml2ZXJNdXRhdG9yc1xuICBjb25zdCB1aURyaXZlckJhc2UgPSB1aURyaXZlckRyYWZ0ID8/IHVpRHJpdmVyO1xuICBjb25zdCB1aURyaXZlckRpcnR5ID0gdWlEcml2ZXJEcmFmdCAhPT0gbnVsbDtcblxuICBjb25zdCBzZXREcml2ZXJJZCA9IHVzZUNhbGxiYWNrKFxuICAgIChpZDogc3RyaW5nKSA9PiB7XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1VJX0RSSVZFUl9EUkFGVFwiLCB2YWx1ZTogbnVsbCB9KTtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfVUlfRFJJVkVSX0lEXCIsIHZhbHVlOiBpZCB9KTtcbiAgICAgIG5vdGVPc0NvbW1hbmQoXCJvcy5zZXREcml2ZXJcIiwgeyBkcml2ZXI6IGlkIH0pO1xuICAgIH0sXG4gICAgW25vdGVPc0NvbW1hbmRdLFxuICApO1xuXG4gIGNvbnN0IHNldERyaXZlckZpZWxkID0gdXNlQ2FsbGJhY2soXG4gICAgPEsgZXh0ZW5kcyBrZXlvZiBPbWl0PFVpRHJpdmVyLCBcImlkXCIgfCBcImxhYmVsXCI+PihrZXk6IEssIHZhbHVlOiBVaURyaXZlcltLXSkgPT4ge1xuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9VSV9EUklWRVJfRFJBRlRcIiwgdmFsdWU6IHsgLi4udWlEcml2ZXJCYXNlLCBba2V5XTogdmFsdWUgfSB9KTtcbiAgICB9LFxuICAgIFt1aURyaXZlckJhc2VdLFxuICApO1xuXG4gIGNvbnN0IHNhdmVEcml2ZXIgPSB1c2VDYWxsYmFjayhcbiAgICAobGFiZWw6IHN0cmluZykgPT4ge1xuICAgICAgY29uc3QgdHJpbW1lZCA9IGxhYmVsLnRyaW0oKTtcbiAgICAgIGlmICghdHJpbW1lZCkgcmV0dXJuO1xuICAgICAgY29uc3Qgc2x1ZyA9IHRyaW1tZWRcbiAgICAgICAgLnRvTG93ZXJDYXNlKClcbiAgICAgICAgLnJlcGxhY2UoL1teYS16MC05XSsvZywgXCItXCIpXG4gICAgICAgIC5yZXBsYWNlKC8oXi0rfC0rJCkvZywgXCJcIik7XG4gICAgICBpZiAoIXNsdWcpIHJldHVybjtcbiAgICAgIGNvbnN0IGlkID0gYGN1c3RvbS4ke3NsdWd9YDtcbiAgICAgIGNvbnN0IHNhdmVkOiBVaURyaXZlciA9IHsgLi4udWlEcml2ZXJCYXNlLCBpZCwgbGFiZWw6IHRyaW1tZWQgfTtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfVUlfQ1VTVE9NX0RSSVZFUlNcIiwgdmFsdWU6IChjdXJyZW50KSA9PiAoeyAuLi5jdXJyZW50LCBbaWRdOiBzYXZlZCB9KSB9KTtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfVUlfRFJJVkVSX0RSQUZUXCIsIHZhbHVlOiBudWxsIH0pO1xuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9VSV9EUklWRVJfSURcIiwgdmFsdWU6IGlkIH0pO1xuICAgIH0sXG4gICAgW3VpRHJpdmVyQmFzZV0sXG4gICk7XG5cbiAgY29uc3QgZGVsZXRlRHJpdmVyID0gdXNlQ2FsbGJhY2soKGlkOiBzdHJpbmcpID0+IHtcbiAgICBpZiAoIWlkLnN0YXJ0c1dpdGgoXCJjdXN0b20uXCIpKSByZXR1cm47XG4gICAgZGlzcGF0Y2goe1xuICAgICAgdHlwZTogXCJTRVRfVUlfQ1VTVE9NX0RSSVZFUlNcIixcbiAgICAgIHZhbHVlOiAoY3VycmVudCkgPT4ge1xuICAgICAgICBjb25zdCB7IFtpZF06IF9yZW1vdmVkLCAuLi5yZXN0IH0gPSBjdXJyZW50O1xuICAgICAgICByZXR1cm4gcmVzdDtcbiAgICAgIH0sXG4gICAgfSk7XG4gICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9VSV9EUklWRVJfSURcIiwgdmFsdWU6IChjdXJyZW50KSA9PiAoY3VycmVudCA9PT0gaWQgPyBERUZBVUxUX1VJX0RSSVZFUi5pZCA6IGN1cnJlbnQpIH0pO1xuICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfVUlfRFJJVkVSX0RSQUZUXCIsIHZhbHVlOiBudWxsIH0pO1xuICB9LCBbXSk7XG4gIC8vI2VuZHJlZ2lvbiDwn5qX77iPRHJpdmVyTXV0YXRvcnNcblxuICBjb25zdCBbdGhlbWVTYXZlTGFiZWwsIHNldFRoZW1lU2F2ZUxhYmVsXSA9IHVzZVN0YXRlKFwiXCIpO1xuICBjb25zdCBbZHJpdmVyU2F2ZUxhYmVsLCBzZXREcml2ZXJTYXZlTGFiZWxdID0gdXNlU3RhdGUoXCJcIik7XG4gIGNvbnN0IFtrZXliaW5kaW5nQ2FwdHVyZUNvbnRyb2xJZCwgc2V0S2V5YmluZGluZ0NhcHR1cmVDb250cm9sSWRdID0gdXNlU3RhdGU8c3RyaW5nIHwgbnVsbD4obnVsbCk7XG4gIGNvbnN0IHNldEtleWJpbmRpbmdPdmVycmlkZSA9IHVzZUNhbGxiYWNrKChjb250cm9sSWQ6IHN0cmluZywga2V5czogc3RyaW5nKSA9PiB7XG4gICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9VSV9LRVlCSU5ESU5HX09WRVJSSURFU1wiLCB2YWx1ZTogKGN1cnJlbnQpID0+ICh7IC4uLmN1cnJlbnQsIFtjb250cm9sSWRdOiBrZXlzIH0pIH0pO1xuICB9LCBbXSk7XG4gIGNvbnN0IHJlc2V0S2V5YmluZGluZ092ZXJyaWRlID0gdXNlQ2FsbGJhY2soKGNvbnRyb2xJZDogc3RyaW5nKSA9PiB7XG4gICAgZGlzcGF0Y2goe1xuICAgICAgdHlwZTogXCJTRVRfVUlfS0VZQklORElOR19PVkVSUklERVNcIixcbiAgICAgIHZhbHVlOiAoY3VycmVudCkgPT4ge1xuICAgICAgICBjb25zdCB7IFtjb250cm9sSWRdOiBfcmVtb3ZlZCwgLi4ucmVzdCB9ID0gY3VycmVudDtcbiAgICAgICAgcmV0dXJuIHJlc3Q7XG4gICAgICB9LFxuICAgIH0pO1xuICB9LCBbXSk7XG4gIHVzZUVmZmVjdCgoKSA9PiB7XG4gICAgY29uc3Qgb25OYXZpZ2F0ZVRvSG90a2V5ID0gKGV2ZW50OiBFdmVudCkgPT4ge1xuICAgICAgY29uc3QgcGF0aCA9IChldmVudCBhcyBDdXN0b21FdmVudDx7IHJlYWRvbmx5IHBhdGg/OiBzdHJpbmcgfT4pLmRldGFpbD8ucGF0aDtcbiAgICAgIGlmIChwYXRoKSBzZXRLZXliaW5kaW5nQ2FwdHVyZUNvbnRyb2xJZChwYXRoKTtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfUEFORUxfVklTSUJMRVwiLCBhbmNob3I6IFwiYm90dG9tLXJpZ2h0XCIsIHZhbHVlOiB0cnVlIH0pO1xuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9QQU5FTF9QQVRIXCIsIGFuY2hvcjogXCJib3R0b20tcmlnaHRcIiwgdmFsdWU6IFtcImZyYW1ld29yay5zZXR0aW5ncy5rZXliaW5kaW5nc1wiXSB9KTtcbiAgICB9O1xuICAgIHdpbmRvdy5hZGRFdmVudExpc3RlbmVyKFwibmF2aWdhdGUtdG8taG90a2V5XCIsIG9uTmF2aWdhdGVUb0hvdGtleSk7XG4gICAgcmV0dXJuICgpID0+IHdpbmRvdy5yZW1vdmVFdmVudExpc3RlbmVyKFwibmF2aWdhdGUtdG8taG90a2V5XCIsIG9uTmF2aWdhdGVUb0hvdGtleSk7XG4gIH0sIFtkaXNwYXRjaF0pO1xuICBjb25zdCBzZXR0aW5nc0hvc3RSZWYgPSB1c2VSZWY8U2V0dGluZ3NIb3N0QXBpIHwgbnVsbD4obnVsbCk7XG4gIGNvbnN0IHNldHRpbmdzSG9zdDogU2V0dGluZ3NIb3N0QXBpID0gdXNlTWVtbyhcbiAgICAoKSA9PiAoe1xuICAgICAgYXBwSWQ6IHNlc3Npb24/LmFwcC5pZCxcbiAgICAgIGFwcExhYmVsOiBzZXNzaW9uID8gYXBwRG9jdW1lbnRMYWJlbChyZXNvbHZlQXBwRG9jdW1lbnQoc2Vzc2lvbi5hcHAsIHVpVGVybWlub2xvZ3kpKSA6IHVuZGVmaW5lZCxcbiAgICAgIGNvbnRyb2xsZXJJZDogc2Vzc2lvbj8uYXBwLmNvbnRyb2xsZXJJZCxcbiAgICAgIHBsdWdpbklkOiBzZXNzaW9uPy5wbHVnaW5JZCxcbiAgICAgIGRyaXZlcklkOiB1aURyaXZlcklkLFxuICAgICAgZHJpdmVyOiB1aURyaXZlckJhc2UsXG4gICAgICBkcml2ZXJEaXJ0eTogdWlEcml2ZXJEaXJ0eSxcbiAgICAgIGRyaXZlcnM6IHVpRHJpdmVyTGlzdCxcbiAgICAgIHNldERyaXZlcklkLFxuICAgICAgc2V0RHJpdmVyRmllbGQsXG4gICAgICBzYXZlRHJpdmVyLFxuICAgICAgZGVsZXRlRHJpdmVyLFxuICAgICAgZHJpdmVyU2F2ZUxhYmVsLFxuICAgICAgc2V0RHJpdmVyU2F2ZUxhYmVsLFxuICAgICAgYXBwZWFyYW5jZTogdWlBcHBlYXJhbmNlLFxuICAgICAgc2V0QXBwZWFyYW5jZTogKHZhbHVlOiBzdHJpbmcpID0+IHtcbiAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9VSV9BUFBFQVJBTkNFXCIsIHZhbHVlOiB2YWx1ZSBhcyBFbGVtZW50c1N1cmZhY2VBcHBlYXJhbmNlIH0pO1xuICAgICAgICBub3RlT3NDb21tYW5kKFwib3Muc2V0QXBwZWFyYW5jZVwiLCB7IGFwcGVhcmFuY2U6IHZhbHVlIH0pO1xuICAgICAgfSxcbiAgICAgIGxheW91dDogdWlMYXlvdXQsXG4gICAgICBzZXRMYXlvdXQ6ICh2YWx1ZTogVWlDaHJvbWVMYXlvdXQpID0+IHtcbiAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9VSV9MQVlPVVRcIiwgdmFsdWUgfSk7XG4gICAgICAgIG5vdGVPc0NvbW1hbmQoXCJvcy5zZXRMYXlvdXRcIiwgeyBsYXlvdXQ6IHZhbHVlIH0pO1xuICAgICAgfSxcbiAgICAgIG1vYmlsZUFjdGl2ZTogbW9iaWxlLFxuICAgICAgb25SZXNldERvY2s6ICgpID0+IHtcbiAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlJFU0VUX0RPQ0tcIiB9KTtcbiAgICAgICAgZG9ja0xheW91dFN0b3JlLnJlc2V0KCk7XG4gICAgICAgIGRvY2tVaVN0YXRlU3RvcmUucmVzZXQoKTtcbiAgICAgICAgbm90ZU9zQ29tbWFuZChcIm9zLnJlc2V0RG9ja1wiKTtcbiAgICAgIH0sXG4gICAgICBsb2NhbGU6IHVpTG9jYWxlLFxuICAgICAgc2V0TG9jYWxlOiAodmFsdWU6IFVpTG9jYWxlKSA9PiB7XG4gICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfVUlfTE9DQUxFXCIsIHZhbHVlIH0pO1xuICAgICAgICBub3RlT3NDb21tYW5kKFwib3Muc2V0TG9jYWxlXCIsIHsgbG9jYWxlOiB2YWx1ZSB9KTtcbiAgICAgIH0sXG4gICAgICB0ZXJtaW5vbG9neTogdWlUZXJtaW5vbG9neSxcbiAgICAgIHNldFRlcm1pbm9sb2d5OiAodmFsdWU6IHN0cmluZykgPT4ge1xuICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1VJX1RFUk1JTk9MT0dZXCIsIHZhbHVlIH0pO1xuICAgICAgICBub3RlT3NDb21tYW5kKFwib3Muc2V0VGVybWlub2xvZ3lcIiwgeyB0ZXJtaW5vbG9neTogdmFsdWUgfSk7XG4gICAgICB9LFxuICAgICAgdGVybWlub2xvZ2llczogW1VJX1RFUk1JTk9MT0dZX05BVElWRSwgLi4uKHNlc3Npb24/LmFwcC50ZXJtaW5vbG9naWVzID8/IFtdKV0sXG4gICAgICB0aGVtZTogdWlUaGVtZUJhc2UsXG4gICAgICB0aGVtZUlkOiB1aVRoZW1lSWQsXG4gICAgICB0aGVtZURpcnR5OiB1aVRoZW1lRGlydHksXG4gICAgICB0aGVtZXM6IHVpVGhlbWVMaXN0LFxuICAgICAgc2V0VGhlbWVJZCxcbiAgICAgIHNldFRoZW1lQ29sb3IsXG4gICAgICBzZXRUaGVtZVNwYWNpbmcsXG4gICAgICBzZXRUaGVtZUZvbnRTdGFjayxcbiAgICAgIHNldFRoZW1lU3Ryb2tlLFxuICAgICAgc2V0VGhlbWVSYWRpdXMsXG4gICAgICBzZXRUaGVtZU9wYWNpdHksXG4gICAgICBzZXRUaGVtZU1ldHJpYyxcbiAgICAgIHNldFRoZW1lQXBwZWFyYW5jZVBhaW50LFxuICAgICAgc2F2ZVRoZW1lLFxuICAgICAgZGVsZXRlVGhlbWUsXG4gICAgICByZXNldFRoZW1lLFxuICAgICAgZXhwb3J0VGhlbWUsXG4gICAgICBpbXBvcnRUaGVtZSxcbiAgICAgIHRoZW1lU2F2ZUxhYmVsLFxuICAgICAgc2V0VGhlbWVTYXZlTGFiZWwsXG4gICAgICBjb250cm9sS2V5YmluZGluZ3MsXG4gICAgICBrZXliaW5kaW5nQ2FwdHVyZUNvbnRyb2xJZCxcbiAgICAgIHNldEtleWJpbmRpbmdDYXB0dXJlQ29udHJvbElkLFxuICAgICAgc2V0S2V5YmluZGluZ092ZXJyaWRlLFxuICAgICAgcmVzZXRLZXliaW5kaW5nT3ZlcnJpZGUsXG4gICAgICBsb2NrcyxcbiAgICB9KSxcbiAgICBbXG4gICAgICBzZXNzaW9uLFxuICAgICAgZG9ja0xheW91dFN0b3JlLFxuICAgICAgdWlEcml2ZXJJZCxcbiAgICAgIHVpRHJpdmVyQmFzZSxcbiAgICAgIHVpRHJpdmVyRGlydHksXG4gICAgICB1aURyaXZlckxpc3QsXG4gICAgICBzZXREcml2ZXJJZCxcbiAgICAgIHNldERyaXZlckZpZWxkLFxuICAgICAgc2F2ZURyaXZlcixcbiAgICAgIGRlbGV0ZURyaXZlcixcbiAgICAgIGRyaXZlclNhdmVMYWJlbCxcbiAgICAgIHNldERyaXZlclNhdmVMYWJlbCxcbiAgICAgIGNvbnRyb2xLZXliaW5kaW5ncyxcbiAgICAgIGtleWJpbmRpbmdDYXB0dXJlQ29udHJvbElkLFxuICAgICAgc2V0S2V5YmluZGluZ092ZXJyaWRlLFxuICAgICAgcmVzZXRLZXliaW5kaW5nT3ZlcnJpZGUsXG4gICAgICB1aUFwcGVhcmFuY2UsXG4gICAgICB1aUxheW91dCxcbiAgICAgIG1vYmlsZSxcbiAgICAgIHVpTG9jYWxlLFxuICAgICAgdWlUZXJtaW5vbG9neSxcbiAgICAgIHVpVGhlbWVCYXNlLFxuICAgICAgdWlUaGVtZUlkLFxuICAgICAgdWlUaGVtZURpcnR5LFxuICAgICAgdWlUaGVtZUxpc3QsXG4gICAgICBsb2NrcyxcbiAgICAgIHNldFRoZW1lSWQsXG4gICAgICBzZXRUaGVtZUNvbG9yLFxuICAgICAgc2V0VGhlbWVTcGFjaW5nLFxuICAgICAgc2V0VGhlbWVGb250U3RhY2ssXG4gICAgICBzZXRUaGVtZVN0cm9rZSxcbiAgICAgIHNldFRoZW1lUmFkaXVzLFxuICAgICAgc2V0VGhlbWVPcGFjaXR5LFxuICAgICAgc2V0VGhlbWVNZXRyaWMsXG4gICAgICBzZXRUaGVtZUFwcGVhcmFuY2VQYWludCxcbiAgICAgIHNhdmVUaGVtZSxcbiAgICAgIGRlbGV0ZVRoZW1lLFxuICAgICAgcmVzZXRUaGVtZSxcbiAgICAgIGV4cG9ydFRoZW1lLFxuICAgICAgaW1wb3J0VGhlbWUsXG4gICAgICB0aGVtZVNhdmVMYWJlbCxcbiAgICAgIHNldFRoZW1lU2F2ZUxhYmVsLFxuICAgICAgbm90ZU9zQ29tbWFuZCxcbiAgICBdLFxuICApO1xuICBzZXR0aW5nc0hvc3RSZWYuY3VycmVudCA9IHNldHRpbmdzSG9zdDtcblxuICBjb25zdCBmcmFtZXdvcmtEaXNwbGF5VGFicyA9IHVzZU1lbW8oKCkgPT4gY3JlYXRlRnJhbWV3b3JrRGlzcGxheVBhbmVsVGFicygoKSA9PiBkaXNwbGF5SG9zdFJlZi5jdXJyZW50KSwgW2Rpc3BsYXlIb3N0LCB1aUxvY2FsZV0pO1xuICBjb25zdCBmcmFtZXdvcmtTZXR0aW5nc1RhYnMgPSB1c2VNZW1vKCgpID0+IGNyZWF0ZUZyYW1ld29ya1NldHRpbmdzUGFuZWxUYWJzKCgpID0+IHNldHRpbmdzSG9zdFJlZi5jdXJyZW50KSwgW3NldHRpbmdzSG9zdF0pO1xuXG4gIGNvbnN0IHBsdWdpbnNIb3N0UmVmID0gdXNlUmVmPFBsdWdpbnNIb3N0QXBpIHwgbnVsbD4obnVsbCk7XG4gIGNvbnN0IHBsdWdpbnNIb3N0OiBQbHVnaW5zSG9zdEFwaSA9IHVzZU1lbW8oXG4gICAgKCkgPT4gKHtcbiAgICAgIHBsdWdpbnM6IHJlZ2lzdHJ5Lm1hcCgoZW50cnkpOiBQbHVnaW5zUGFuZWxFbnRyeSA9PiB7XG4gICAgICAgIGNvbnN0IGxvYWRlZEVudHJ5ID0gbG9hZGVkUGx1Z2lucy5maW5kKChjYW5kaWRhdGUpID0+IGNhbmRpZGF0ZS5oYW5kbGUucGx1Z2luSWQgPT09IGVudHJ5LnBsdWdpbklkKTtcbiAgICAgICAgcmV0dXJuIHtcbiAgICAgICAgICBwbHVnaW5JZDogZW50cnkucGx1Z2luSWQsXG4gICAgICAgICAgbGFiZWw6IGxvYWRlZEVudHJ5Py5tYW5pZmVzdC5sYWJlbCA/PyBlbnRyeS5wbHVnaW5JZCxcbiAgICAgICAgICB2ZXJzaW9uOiBsb2FkZWRFbnRyeT8ubWFuaWZlc3QudmVyc2lvbixcbiAgICAgICAgICBzdGF0dXM6IHBsdWdpblN0YXR1c0J5SWRbZW50cnkucGx1Z2luSWRdID8/IFwiYXZhaWxhYmxlXCIsXG4gICAgICAgICAgc291cmNlSWQ6IHBsdWdpblNvdXJjZS5pZCxcbiAgICAgICAgICBjYW5Vbmluc3RhbGw6IGVudHJ5LnBsdWdpbklkICE9PSBwcmltYXJ5UGx1Z2luSWQgJiYgc2Vzc2lvbj8ucGx1Z2luSWQgIT09IGVudHJ5LnBsdWdpbklkLFxuICAgICAgICB9O1xuICAgICAgfSksXG4gICAgICBpbnN0YWxsOiAocGx1Z2luSWQpID0+IHZvaWQgaW5zdGFsbFBsdWdpbihwbHVnaW5JZCksXG4gICAgICB1bmluc3RhbGw6IChwbHVnaW5JZCkgPT4gdm9pZCB1bmluc3RhbGxQbHVnaW4ocGx1Z2luSWQpLFxuICAgICAgcmVsb2FkOiAocGx1Z2luSWQpID0+IHZvaWQgcmVsb2FkUGx1Z2luKHBsdWdpbklkKSxcbiAgICB9KSxcbiAgICBbcmVnaXN0cnksIGxvYWRlZFBsdWdpbnMsIHBsdWdpblN0YXR1c0J5SWQsIHBsdWdpblNvdXJjZSwgcHJpbWFyeVBsdWdpbklkLCBzZXNzaW9uPy5wbHVnaW5JZCwgaW5zdGFsbFBsdWdpbiwgdW5pbnN0YWxsUGx1Z2luLCByZWxvYWRQbHVnaW5dLFxuICApO1xuICBwbHVnaW5zSG9zdFJlZi5jdXJyZW50ID0gcGx1Z2luc0hvc3Q7XG4gIGNvbnN0IGZyYW1ld29ya1BsdWdpbnNUYWJzID0gdXNlTWVtbygoKSA9PiBjcmVhdGVGcmFtZXdvcmtQbHVnaW5zUGFuZWxUYWJzKCgpID0+IHBsdWdpbnNIb3N0UmVmLmN1cnJlbnQpLCBbcGx1Z2luc0hvc3RdKTtcblxuICAvLyDwn5Ca77iPIEdhdGVkIHRvIHRoaXMgc2hlbGwgdmlhIGB1c2VTaGVsbEtleWRvd25gIGJlbG93IOKAlCB3YXMgYW4gdW5jb25kaXRpb25hbCBgd2luZG93YCBrZXlkb3duIGxpc3RlbmVyLFxuICAvLyBzbyBldmVyeSBtb3VudGVkIHNoZWxsIGZpcmVkIGl0cyBib3VuZCBhY3Rpb24gKGFuZCBjb3VsZCBgcHJldmVudERlZmF1bHQoKWAgb3V0IGZyb20gdW5kZXIgYW5vdGhlclxuICAvLyBzaGVsbCkgZm9yIGV2ZXJ5IGtleXN0cm9rZSBvbiB0aGUgcGFnZSByZWdhcmRsZXNzIG9mIHdoaWNoIHNoZWxsIHRoZSB1c2VyIHdhcyBhY3R1YWxseSB1c2luZy5cbiAgY29uc3QgaGFuZGxlQXBwS2V5ZG93biA9IHVzZUNhbGxiYWNrKFxuICAgIChldmVudDogS2V5Ym9hcmRFdmVudCkgPT4ge1xuICAgICAgaWYgKCFzZXNzaW9uKSByZXR1cm47XG4gICAgICBjb25zdCBwYXJzZUtleXMgPSAoa2V5czogc3RyaW5nKSA9PlxuICAgICAgICBrZXlzXG4gICAgICAgICAgLnNwbGl0KFwiLFwiKVxuICAgICAgICAgIC5tYXAoKGtleSkgPT4ga2V5LnRyaW0oKS50b0xvd2VyQ2FzZSgpKVxuICAgICAgICAgIC5maWx0ZXIoQm9vbGVhbik7XG4gICAgICBjb25zdCBpc0VkaXRhYmxlVGFyZ2V0ID0gKHRhcmdldDogRXZlbnRUYXJnZXQgfCBudWxsKSA9PiB7XG4gICAgICAgIGlmICghKHRhcmdldCBpbnN0YW5jZW9mIEhUTUxFbGVtZW50KSkgcmV0dXJuIGZhbHNlO1xuICAgICAgICBjb25zdCB0YWcgPSB0YXJnZXQudGFnTmFtZTtcbiAgICAgICAgaWYgKHRhZyA9PT0gXCJJTlBVVFwiIHx8IHRhZyA9PT0gXCJURVhUQVJFQVwiIHx8IHRhZyA9PT0gXCJTRUxFQ1RcIikgcmV0dXJuIHRydWU7XG4gICAgICAgIGlmICh0YXJnZXQuaXNDb250ZW50RWRpdGFibGUpIHJldHVybiB0cnVlO1xuICAgICAgICByZXR1cm4gdGFyZ2V0LmNsb3Nlc3QoXCJbY29udGVudGVkaXRhYmxlPSd0cnVlJ10sIFtyb2xlPSd0ZXh0Ym94J11cIikgIT0gbnVsbDtcbiAgICAgIH07XG4gICAgICBjb25zdCBtYXRjaGVzID0gKGV2ZW50OiBLZXlib2FyZEV2ZW50LCBiaW5kaW5nOiBzdHJpbmcpID0+IHtcbiAgICAgICAgY29uc3QgcGFydHMgPSBiaW5kaW5nLnNwbGl0KFwiK1wiKS5tYXAoKHBhcnQpID0+IHBhcnQudHJpbSgpKTtcbiAgICAgICAgY29uc3Qga2V5ID0gcGFydHNbcGFydHMubGVuZ3RoIC0gMV0gPz8gXCJcIjtcbiAgICAgICAgY29uc3QgbmVlZHNDdHJsID0gcGFydHMuaW5jbHVkZXMoXCJjdHJsXCIpIHx8IHBhcnRzLmluY2x1ZGVzKFwibWV0YVwiKSB8fCBwYXJ0cy5pbmNsdWRlcyhcIm1vZFwiKTtcbiAgICAgICAgY29uc3QgbmVlZHNTaGlmdCA9IHBhcnRzLmluY2x1ZGVzKFwic2hpZnRcIik7XG4gICAgICAgIGNvbnN0IG5lZWRzQWx0ID0gcGFydHMuaW5jbHVkZXMoXCJhbHRcIik7XG4gICAgICAgIGNvbnN0IGhhc0N0cmwgPSBldmVudC5jdHJsS2V5IHx8IGV2ZW50Lm1ldGFLZXk7XG4gICAgICAgIGlmIChuZWVkc0N0cmwgIT09IGhhc0N0cmwpIHJldHVybiBmYWxzZTtcbiAgICAgICAgaWYgKG5lZWRzU2hpZnQgIT09IGV2ZW50LnNoaWZ0S2V5KSByZXR1cm4gZmFsc2U7XG4gICAgICAgIGlmIChuZWVkc0FsdCAhPT0gZXZlbnQuYWx0S2V5KSByZXR1cm4gZmFsc2U7XG4gICAgICAgIHJldHVybiBldmVudC5rZXkudG9Mb3dlckNhc2UoKSA9PT0ga2V5O1xuICAgICAgfTtcbiAgICAgIGNvbnN0IGFjdGlvbkJ5SWQgPSBuZXcgTWFwKHNlc3Npb24uYXBwLmFjdGlvbnMubWFwKChhY3Rpb24pID0+IFthY3Rpb24uaWQsIGFjdGlvbl0pKTtcbiAgICAgIGlmIChpc0VkaXRhYmxlVGFyZ2V0KGV2ZW50LnRhcmdldCkpIHJldHVybjtcbiAgICAgIC8vIPCfp7DvuI/wn5ug77iPIEVzY2FwZSBkZWFjdGl2YXRlcyB0aGUgYWN0aXZlIHdpbmRvdydzIGFjdGl2ZSB1dGlsaXR5IChQNSksIG9yIOKAlCB3aGVuIG5vIHV0aWxpdHkgaXMgYWN0aXZlIOKAlFxuICAgICAgLy8gdGhlIGFjdGl2ZSBtb2RlLWxldmVsIHRvb2wsIHdoZW4gbm90aGluZyBpcyBiZWluZyB0eXBlZC5cbiAgICAgIGlmIChldmVudC5rZXkgPT09IFwiRXNjYXBlXCIpIHtcbiAgICAgICAgY29uc3Qgd2luZG93SWQgPSBhY3RpdmVXaW5kb3dJZFJlZi5jdXJyZW50O1xuICAgICAgICBpZiAod2luZG93SWQgJiYgYWN0aXZlVXRpbGl0eUJ5V2luZG93SWRSZWYuY3VycmVudFt3aW5kb3dJZF0pIHtcbiAgICAgICAgICBldmVudC5wcmV2ZW50RGVmYXVsdCgpO1xuICAgICAgICAgIG9uQWN0aW9uKHsgY29udHJvbGxlcklkOiBzZXNzaW9uLmFwcC5jb250cm9sbGVySWQsIGFjdGlvbjogU0VUX0FDVElWRV9VVElMSVRZX0FDVElPTl9JRCwgYXJnczogeyB3aW5kb3dJZCwgdXRpbGl0eUlkOiBcIlwiIH0gfSk7XG4gICAgICAgICAgcmV0dXJuO1xuICAgICAgICB9XG4gICAgICAgIGlmIChhY3RpdmVUb29sSWRSZWYuY3VycmVudCkge1xuICAgICAgICAgIGV2ZW50LnByZXZlbnREZWZhdWx0KCk7XG4gICAgICAgICAgb25BY3Rpb24oeyBjb250cm9sbGVySWQ6IHNlc3Npb24uYXBwLmNvbnRyb2xsZXJJZCwgYWN0aW9uOiBTRVRfQUNUSVZFX1RPT0xfQUNUSU9OX0lELCBhcmdzOiB7IHRvb2xJZDogXCJcIiB9IH0pO1xuICAgICAgICAgIHJldHVybjtcbiAgICAgICAgfVxuICAgICAgfVxuICAgICAgZm9yIChjb25zdCBiaW5kaW5nIG9mIHNlc3Npb24uYXBwLmtleWJpbmRpbmdzKSB7XG4gICAgICAgIGZvciAoY29uc3QgY2hvcmQgb2YgcGFyc2VLZXlzKGJpbmRpbmcua2V5cykpIHtcbiAgICAgICAgICBpZiAoIW1hdGNoZXMoZXZlbnQsIGNob3JkKSkgY29udGludWU7XG4gICAgICAgICAgZXZlbnQucHJldmVudERlZmF1bHQoKTtcbiAgICAgICAgICAvLyDinI3vuI8gQXJnLWNhcnJ5aW5nIGhvdGtleXMgbmV2ZXIgc2lsZW50LWZpcmUgZGVmYXVsdHMgKFA0KTogb3BlbiB0aGUgc3RhZ2VkIGZvcm0sIG9yIOKAlCBpZiB0aGF0XG4gICAgICAgICAgLy8gZm9ybSBpcyBhbHJlYWR5IGV4cGFuZGVkIGluIHRoZSBhY3RpdmUgd2luZG93IOKAlCB0cmVhdCB0aGUgaG90a2V5IGFzIEV4ZWN1dGUgKHdpdGggdmFsaWRhdGlvbikuXG4gICAgICAgICAgY29uc3QgZGVmaW5pdGlvbiA9IGFjdGlvbkJ5SWQuZ2V0KGJpbmRpbmcuYWN0aW9uLmFjdGlvbik7XG4gICAgICAgICAgaWYgKGRlZmluaXRpb24gJiYgYWN0aW9uUmVxdWlyZXNTdGFnZWRGb3JtKGRlZmluaXRpb24pKSB7XG4gICAgICAgICAgICBjb25zdCB3aW5kb3dJZCA9IGFjdGl2ZVdpbmRvd0lkUmVmLmN1cnJlbnQ7XG4gICAgICAgICAgICBpZiAoIXdpbmRvd0lkKSByZXR1cm47XG4gICAgICAgICAgICBjb25zdCBleHBhbmRlZCA9IGFjdGlvblBhbmVFeHBhbmRlZEJ5V2luZG93SWRSZWYuY3VycmVudFt3aW5kb3dJZF0gPz8gbnVsbDtcbiAgICAgICAgICAgIGNvbnN0IHN0YWdlZCA9IGFjdGlvblBhbmVTdGFnZWRBcmdzQnlLZXlSZWYuY3VycmVudFthY3Rpb25TdGFnZUtleSh3aW5kb3dJZCwgZGVmaW5pdGlvbi5pZCldID8/IHt9O1xuICAgICAgICAgICAgY29uc3QgaW50ZW50ID0gcmVzb2x2ZUtleWJpbmRpbmdJbnRlbnQoZGVmaW5pdGlvbiwgZXhwYW5kZWQsIHN0YWdlZCk7XG4gICAgICAgICAgICBpZiAoaW50ZW50LmtpbmQgPT09IFwiZXhlY3V0ZVwiKSB7XG4gICAgICAgICAgICAgIG9uQWN0aW9uKHsgY29udHJvbGxlcklkOiBzZXNzaW9uLmFwcC5jb250cm9sbGVySWQsIGFjdGlvbjogaW50ZW50LmFjdGlvbklkLCBhcmdzOiBpbnRlbnQuYXJncyB9KTtcbiAgICAgICAgICAgIH0gZWxzZSBpZiAoaW50ZW50LmtpbmQgPT09IFwib3BlblwiKSB7XG4gICAgICAgICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfQUNUSU9OX1BBTkVfRk9MREVEXCIsIHdpbmRvd0lkLCB2YWx1ZTogZmFsc2UgfSk7XG4gICAgICAgICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfQUNUSU9OX1BBTkVfRVhQQU5ERURcIiwgd2luZG93SWQsIHZhbHVlOiBpbnRlbnQuYWN0aW9uSWQgfSk7XG4gICAgICAgICAgICB9XG4gICAgICAgICAgICByZXR1cm47XG4gICAgICAgICAgfVxuICAgICAgICAgIG9uQWN0aW9uKGJpbmRpbmcuYWN0aW9uKTtcbiAgICAgICAgICByZXR1cm47XG4gICAgICAgIH1cbiAgICAgIH1cbiAgICB9LFxuICAgIFtvbkFjdGlvbiwgc2Vzc2lvbl0sXG4gICk7XG4gIHVzZVNoZWxsS2V5ZG93bihzY29wZS5yb290UmVmLCBoYW5kbGVBcHBLZXlkb3duLCBbaGFuZGxlQXBwS2V5ZG93bl0pO1xuXG4gIGNvbnN0IGFjdGl2ZVJpZ2h0UGFuZWxUYWIgPSBzZXNzaW9uPy5hcHAucGFuZWxUYWJzLmZpbmQoKHRhYikgPT4gcGFuZWxBbmNob3JGb3JHcm91cCh0YWIuZ3JvdXApID09PSBcInRvcC1yaWdodFwiKTtcbiAgY29uc3QgYWN0aXZlUGFuZWxUYWJJZCA9IHBhbmVsPy5hY3RpdmVQYW5lbFRhYiA/PyAoYWN0aXZlUmlnaHRQYW5lbFRhYiA/IHBhbmVsVGFiS2luZElkKGFjdGl2ZVJpZ2h0UGFuZWxUYWIua2luZCkgOiB1bmRlZmluZWQpID8/IChzZXNzaW9uPy5hcHAucGFuZWxUYWJzWzBdID8gcGFuZWxUYWJLaW5kSWQoc2Vzc2lvbi5hcHAucGFuZWxUYWJzWzBdLmtpbmQpIDogdW5kZWZpbmVkKTtcblxuICBjb25zdCB3b3JrYmVuY2hMZWZ0VGFicyA9IHVzZU1lbW8oKCk6IFBhbmVsVGFiTm9kZVtdID0+IHtcbiAgICBpZiAoIXNlc3Npb24pIHJldHVybiBbXTtcbiAgICBjb25zdCBwbHVnaW5MZWZ0VGFicyA9IHNlc3Npb24uYXBwLnBhbmVsVGFicy5maWx0ZXIoKHRhYikgPT4gcGFuZWxBbmNob3JGb3JHcm91cCh0YWIuZ3JvdXApID09PSBcInRvcC1sZWZ0XCIpLm1hcCgodGFiLCBvcmRlcikgPT4gcGFuZWxUYWJEZWZpbml0aW9uVG9Ob2RlKHRhYiwgdGFiLmdyb3VwLCBwYW5lbFVpQnlLZXksIG9uQWN0aW9uLCBvcmRlciwgYXBwTGFiZWxzT3ZlcmxheSwgdWlUZXJtaW5vbG9neSwgdWlMb2NhbGUpKTtcbiAgICBpZiAoc3R1ZGlvTW9kZSAmJiBzZXNzaW9uLmFwcC5pZCA9PT0gaG9zdEFwcElkICYmIHBsdWdpbkxlZnRUYWJzLmxlbmd0aCA+IDApIHJldHVybiBwbHVnaW5MZWZ0VGFicztcbiAgICBjb25zdCBoYXNQbHVnaW5Eb2N1bWVudFRhYiA9IHBsdWdpbkxlZnRUYWJzLnNvbWUoKHRhYikgPT4gdGFiLmlkID09PSBGUkFNRVdPUktfUEFORUxfVEFCX0RPQ1VNRU5UX0lEKTtcbiAgICBpZiAoaGFzUGx1Z2luRG9jdW1lbnRUYWIpIHJldHVybiBwbHVnaW5MZWZ0VGFicztcbiAgICBjb25zdCBkb2N1bWVudFRhYiA9IHNpbmdsZVRyZWVMZWFmKHtcbiAgICAgIGlkOiBGUkFNRVdPUktfUEFORUxfVEFCX0RPQ1VNRU5UX0lELFxuICAgICAgaWNvbjogc2hlbGxUYWJJY29uKEZSQU1FV09SS19QQU5FTF9UQUJfRE9DVU1FTlRfSUNPTl9JRCksXG4gICAgICBuYW1lOiBzaGVsbExhYmVsKFwidWkucGFuZWwuZG9jdW1lbnRcIiksXG4gICAgICBvcmRlcjogMCxcbiAgICAgIHRyZWU6IHN0YXRpY1RyZWVQYW5lbERlZmluaXRpb24oe1xuICAgICAgICBzZWN0aW9uczogW1xuICAgICAgICAgIHtcbiAgICAgICAgICAgIGlkOiBcImRvY3VtZW50LnJvb3RcIixcbiAgICAgICAgICAgIGxhYmVsOiBzaGVsbExhYmVsKFwidWkucGFuZWwuZG9jdW1lbnRcIiksXG4gICAgICAgICAgICBpdGVtczogW3sgaWQ6IFwiZG9jdW1lbnQuZW1wdHlcIiwgbGFiZWw6IHN0dWRpb01vZGUgPyBgJHtwYW5lbD8uc3Bhd25lZEFwcHMubGVuZ3RoID8/IDB9ICR7c2hlbGxMYWJlbChcInVpLnBhbmVsLnNwYXduZWRBcHBzU3VmZml4XCIpfWAgOiBzaGVsbExhYmVsKFwidWkucGFuZWwuZG9jdW1lbnRFbXB0eVwiKSB9XSxcbiAgICAgICAgICB9LFxuICAgICAgICBdLFxuICAgICAgfSksXG4gICAgfSk7XG4gICAgcmV0dXJuIFtkb2N1bWVudFRhYiwgLi4ucGx1Z2luTGVmdFRhYnNdO1xuICB9LCBbYXBwTGFiZWxzT3ZlcmxheSwgb25BY3Rpb24sIHBhbmVsPy5zcGF3bmVkQXBwcy5sZW5ndGgsIHBhbmVsVWlCeUtleSwgc2Vzc2lvbiwgc3R1ZGlvTW9kZSwgdWlMb2NhbGUsIHVpVGVybWlub2xvZ3ksIGhvc3RBcHBJZF0pO1xuXG4gIGNvbnN0IGRldGFpbHNSaWdodFRhYnMgPSB1c2VNZW1vKCgpOiBQYW5lbFRhYk5vZGVbXSA9PiB7XG4gICAgaWYgKCFzZXNzaW9uKSByZXR1cm4gW107XG4gICAgcmV0dXJuIHNlc3Npb24uYXBwLnBhbmVsVGFicy5maWx0ZXIoKHRhYikgPT4gcGFuZWxBbmNob3JGb3JHcm91cCh0YWIuZ3JvdXApID09PSBcInRvcC1yaWdodFwiKS5tYXAoKHRhYiwgb3JkZXIpID0+IHBhbmVsVGFiRGVmaW5pdGlvblRvTm9kZSh0YWIsIHRhYi5ncm91cCwgcGFuZWxVaUJ5S2V5LCBvbkFjdGlvbiwgb3JkZXIsIGFwcExhYmVsc092ZXJsYXksIHVpVGVybWlub2xvZ3ksIHVpTG9jYWxlKSk7XG4gIH0sIFthcHBMYWJlbHNPdmVybGF5LCBvbkFjdGlvbiwgcGFuZWxVaUJ5S2V5LCBzZXNzaW9uLCB1aVRlcm1pbm9sb2d5LCB1aUxvY2FsZV0pO1xuXG4gIGNvbnN0IHNldHRpbmdzUmlnaHRUYWJzID0gdXNlTWVtbygoKTogUGFuZWxUYWJOb2RlW10gPT4gZnJhbWV3b3JrU2V0dGluZ3NUYWJzLCBbZnJhbWV3b3JrU2V0dGluZ3NUYWJzXSk7XG5cbiAgLy8jcmVnaW9uIPCfp7DvuI9Gb290ZXJVdGlsaXR5TGVhdmVzIOKAlCBib3R0b20tcmlnaHQncyBIaXN0b3J5IHRhYiwgc291cmNlZCBmcm9tIHRoZSBmcmFtZXdvcmstaW5qZWN0ZWRcbiAgLy8gYGZyYW1ld29yay5wYW5lbC5oaXN0b3J5YCBwYW5lbCB0YWIgKGV2ZXJ5IGFwcCBnZXRzIG9uZSDigJQgc2VlIGBBcHBCdWlsZGVyOjpidWlsZF9kZWZpbml0aW9uYCkuXG4gIGNvbnN0IGZyYW1ld29ya1V0aWxpdGllc0hpc3RvcnlUYWIgPSB1c2VNZW1vKCgpOiBQYW5lbFRhYk5vZGUgfCBudWxsID0+IHtcbiAgICBpZiAoIXNlc3Npb24pIHJldHVybiBudWxsO1xuICAgIGNvbnN0IHRhYiA9IHNlc3Npb24uYXBwLnBhbmVsVGFicy5maW5kKChjYW5kaWRhdGUpID0+IHBhbmVsVGFiS2luZElkKGNhbmRpZGF0ZS5raW5kKSA9PT0gRlJBTUVXT1JLX1BBTkVMX1RBQl9ISVNUT1JZX0lEKTtcbiAgICBpZiAoIXRhYikgcmV0dXJuIG51bGw7XG4gICAgcmV0dXJuIHBhbmVsVGFiRGVmaW5pdGlvblRvTm9kZSh0YWIsIHRhYi5ncm91cCwgcGFuZWxVaUJ5S2V5LCBvbkFjdGlvbiwgMSwgYXBwTGFiZWxzT3ZlcmxheSwgdWlUZXJtaW5vbG9neSwgdWlMb2NhbGUpO1xuICB9LCBbYXBwTGFiZWxzT3ZlcmxheSwgb25BY3Rpb24sIHBhbmVsVWlCeUtleSwgc2Vzc2lvbiwgdWlUZXJtaW5vbG9neSwgdWlMb2NhbGVdKTtcbiAgLy8jZW5kcmVnaW9uIPCfp7DvuI9Gb290ZXJVdGlsaXR5TGVhdmVzXG5cbiAgLy8jcmVnaW9uIPCflITvuI9TeW5jTGVhZiDigJQgYm90dG9tLWxlZnQncyBzeW5jIHRhYiwgcmVwbGFjaW5nIHRoZSBvbGQgZmxvYXRpbmcgZm9vdGVyIFN5bmNBdHRhY2hDYXJkLlxuICBjb25zdCBmcmFtZXdvcmtTeW5jVGFiID0gdXNlTWVtbygoKTogUGFuZWxUYWJOb2RlIHwgbnVsbCA9PiB7XG4gICAgY29uc3Qgc3luY1V0aWxpdGllcyA9IGJ1aWxkRnJhbWV3b3JrU3luY1V0aWxpdGllcyhzeW5jQmFja2JvbmVVcmkpIGFzIHJlYWRvbmx5IFV0aWxpdHlOb2RlW107XG4gICAgaWYgKCFzeW5jVXRpbGl0aWVzLmxlbmd0aCkgcmV0dXJuIG51bGw7XG4gICAgY29uc3Qgc3luY1N0YXR1cyA9IHN5bmNCYWNrYm9uZVVyaSA/IChzeW5jU3RhdHVzQnlEb2N1bWVudElkW3N5bmNCYWNrYm9uZVVyaS5yZXBsYWNlKC9eYWN0b3I6XFwvXFwvLywgXCJcIildID8/IG51bGwpIDogbnVsbDtcbiAgICByZXR1cm4gc2luZ2xlVHJlZUxlYWYoe1xuICAgICAgaWQ6IFwiZnJhbWV3b3JrLnN5bmNcIixcbiAgICAgIGljb246IHNoZWxsVGFiSWNvbihVVElMSVRZX0NBVEVHT1JZX0lDT05fSUQuc3luYyksXG4gICAgICBuYW1lOiBzaGVsbExhYmVsKFwidWkucGFuZWwuc3luY1wiKSxcbiAgICAgIG9yZGVyOiAwLFxuICAgICAgdHJlZToge1xuICAgICAgICBzZWN0aW9uczogW1xuICAgICAgICAgIHtcbiAgICAgICAgICAgIGlkOiBcImZyYW1ld29yay5zeW5jLnJvb3RcIixcbiAgICAgICAgICAgIGxhYmVsOiBcIlwiLFxuICAgICAgICAgICAgaXRlbXM6IFtcbiAgICAgICAgICAgICAge1xuICAgICAgICAgICAgICAgIGlkOiBcImZyYW1ld29yay5zeW5jLmNhcmRcIixcbiAgICAgICAgICAgICAgICBsYWJlbDogXCJcIixcbiAgICAgICAgICAgICAgICBjb250cm9sOiAoXG4gICAgICAgICAgICAgICAgICA8U3luY0F0dGFjaENhcmRcbiAgICAgICAgICAgICAgICAgICAgYWN0aXZlVXJpPXtzeW5jQmFja2JvbmVVcml9XG4gICAgICAgICAgICAgICAgICAgIGNhcmRLaW5kPXtzeW5jQ2FyZEtpbmR9XG4gICAgICAgICAgICAgICAgICAgIGRyYWZ0UGF0aD17c3luY0RyYWZ0UGF0aH1cbiAgICAgICAgICAgICAgICAgICAgc3luY1V0aWxpdGllcz17c3luY1V0aWxpdGllc31cbiAgICAgICAgICAgICAgICAgICAgc3RhdHVzPXtzeW5jU3RhdHVzfVxuICAgICAgICAgICAgICAgICAgICBvbkFjdGlvbj17b25BY3Rpb259XG4gICAgICAgICAgICAgICAgICAgIG9uRHJhZnRQYXRoQ2hhbmdlPXsodmFsdWUpID0+IGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfU1lOQ19EUkFGVF9QQVRIXCIsIHZhbHVlIH0pfVxuICAgICAgICAgICAgICAgICAgICBvbkNsb3NlPXsoKSA9PiBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1NZTkNfQ0FSRF9LSU5EXCIsIHZhbHVlOiBudWxsIH0pfVxuICAgICAgICAgICAgICAgICAgICBvbkF0dGFjaD17YXR0YWNoU3luY0JhY2tib25lfVxuICAgICAgICAgICAgICAgICAgICBvbkRldGFjaD17ZGV0YWNoU3luY0JhY2tib25lfVxuICAgICAgICAgICAgICAgICAgLz5cbiAgICAgICAgICAgICAgICApLFxuICAgICAgICAgICAgICB9LFxuICAgICAgICAgICAgXSxcbiAgICAgICAgICB9LFxuICAgICAgICBdLFxuICAgICAgfSxcbiAgICB9KTtcbiAgfSwgW2F0dGFjaFN5bmNCYWNrYm9uZSwgZGV0YWNoU3luY0JhY2tib25lLCBvbkFjdGlvbiwgc3luY0JhY2tib25lVXJpLCBzeW5jQ2FyZEtpbmQsIHN5bmNEcmFmdFBhdGgsIHN5bmNTdGF0dXNCeURvY3VtZW50SWQsIHVpTG9jYWxlXSk7XG4gIC8vI2VuZHJlZ2lvbiDwn5SE77iPU3luY0xlYWZcblxuICBjb25zdCBhY3RpdmVQbHVnaW5NYW5pZmVzdCA9IHVzZU1lbW8oKCkgPT4gbG9hZGVkUGx1Z2lucy5maW5kKChlbnRyeSkgPT4gZW50cnkuaGFuZGxlLnBsdWdpbklkID09PSBzZXNzaW9uPy5wbHVnaW5JZCk/Lm1hbmlmZXN0LCBbbG9hZGVkUGx1Z2lucywgc2Vzc2lvbj8ucGx1Z2luSWRdKTtcbiAgY29uc3QgYWN0aXZlTW9kZUlkID0gc2Vzc2lvbj8udmlld1N0YXRlLmFjdGl2ZU1vZGVJZCA/PyBzZXNzaW9uPy5hcHAubW9kZXNbMF0/LmlkID8/IHNlc3Npb24/LmFwcC5pZCA/PyBcIlwiO1xuXG4gIC8vIPCfk7HvuI8gTW92ZWQgYWhlYWQgb2YgYG1vYmlsZVBhbmVsVGFic2AgKGJlbG93KSBzbyBpdHMgc3ludGhldGljIG1vYmlsZSBcIkFwcFwiIHRhYiBjYW4gc2hhcmUgdGhlIGV4YWN0XG4gIC8vIGV4YW1wbGUtc2VsZWN0L21vZGUtc3dpdGNoZXIgZWxlbWVudHMgdGhlIGRlc2t0b3AgbmF2YmFyIGNlbnRlciBjbHVzdGVyIHJlbmRlcnMg4oCUIHNpbmdsZSBzb3VyY2Ugb2YgdHJ1dGguXG4gIGNvbnN0IGV4YW1wbGVPcHRpb25zID0gdXNlTWVtbygoKSA9PiB7XG4gICAgY29uc3QgYXBwSWQgPSBzZXNzaW9uPy5hcHAuaWQgPz8gXCJcIjtcbiAgICBpZiAoIWFwcElkKSByZXR1cm4gW107XG4gICAgY29uc3Qgc2VlbiA9IG5ldyBTZXQ8c3RyaW5nPigpO1xuICAgIHJldHVybiAoYWN0aXZlUGx1Z2luTWFuaWZlc3Q/LmV4YW1wbGVzID8/IFtdKVxuICAgICAgLmZpbHRlcigoZXhhbXBsZSkgPT4gZXhhbXBsZS5hcHBJZCA9PT0gYXBwSWQpXG4gICAgICAuZmlsdGVyKChleGFtcGxlKSA9PiB7XG4gICAgICAgIGlmIChzZWVuLmhhcyhleGFtcGxlLmlkKSkgcmV0dXJuIGZhbHNlO1xuICAgICAgICBzZWVuLmFkZChleGFtcGxlLmlkKTtcbiAgICAgICAgcmV0dXJuIHRydWU7XG4gICAgICB9KVxuICAgICAgLm1hcCgoZXhhbXBsZSkgPT4gKHtcbiAgICAgICAgaWQ6IGV4YW1wbGUuaWQsXG4gICAgICAgIGxhYmVsOiByZXNvbHZlQXBwTGFiZWwoYXBwTGFiZWxzT3ZlcmxheSwgXCJleGFtcGxlXCIsIGV4YW1wbGUuaWQsIHJlc29sdmVNYW5pZmVzdExhYmVsKGV4YW1wbGUubGFiZWwsIHVpVGVybWlub2xvZ3ksIHVpTG9jYWxlKSksXG4gICAgICAgIGljb246IGV4YW1wbGUuaWNvbklkLFxuICAgICAgfSkpO1xuICB9LCBbYWN0aXZlUGx1Z2luTWFuaWZlc3QsIHNlc3Npb24/LmFwcC5pZCwgYXBwTGFiZWxzT3ZlcmxheSwgdWlUZXJtaW5vbG9neSwgdWlMb2NhbGVdKTtcblxuICBjb25zdCBkaXNwYXRjaEFjdGl2ZUV4YW1wbGUgPSB1c2VDYWxsYmFjayhcbiAgICAoZXhhbXBsZUlkOiBzdHJpbmcpID0+IHtcbiAgICAgIGlmICghc2Vzc2lvbikgcmV0dXJuO1xuICAgICAgY29uc3QgcGx1Z2luID0gbG9hZGVkUGx1Z2lucy5maW5kKChlbnRyeSkgPT4gZW50cnkuaGFuZGxlLnBsdWdpbklkID09PSBzZXNzaW9uLnBsdWdpbklkKT8uaGFuZGxlO1xuICAgICAgaWYgKCFwbHVnaW4pIHJldHVybjtcbiAgICAgIG9uQWN0aW9uKHsgY29udHJvbGxlcklkOiBzZXNzaW9uLmFwcC5jb250cm9sbGVySWQsIGFjdGlvbjogXCJzZXRBY3RpdmVFeGFtcGxlXCIsIGFyZ3M6IHsgZXhhbXBsZUlkOiBleGFtcGxlSWQgfHwgXCJcIiB9IH0pO1xuICAgIH0sXG4gICAgW2FwcGx5SG9zdEVmZmVjdHMsIGluamVjdEFjdGl2ZVV0aWxpdHksIGxvYWRlZFBsdWdpbnMsIG9uQWN0aW9uLCBzZXNzaW9uXSxcbiAgKTtcblxuICAvKiogQGVtb2ppIPCfjpvvuI8gU2hhcmVkIGJ5IHRoZSBkZXNrdG9wIG5hdmJhciBjZW50ZXIgY2x1c3RlciBhbmQgdGhlIG1vYmlsZSBwYW5lbCdzIHN5bnRoZXRpYyBcIkFwcFwiIHRhYiAoc2VlIGBtb2JpbGVQYW5lbFRhYnNgKS4gKi9cbiAgY29uc3QgZXhhbXBsZVNlbGVjdEVsZW1lbnQgPSB1c2VNZW1vKCgpID0+IHtcbiAgICBpZiAoIXNlc3Npb24gfHwgZXhhbXBsZU9wdGlvbnMubGVuZ3RoID09PSAwIHx8IGxvY2tzLmV4YW1wbGVJZCB8fCAoc3R1ZGlvTW9kZSAmJiBzZXNzaW9uLmFwcC5pZCA9PT0gbGFuZGluZ0FwcElkKSkgcmV0dXJuIG51bGw7XG4gICAgcmV0dXJuIChcbiAgICAgIDxOYXZiYXJFeGFtcGxlU2VsZWN0XG4gICAgICAgIGtleT1cImZpeHR1cmVcIlxuICAgICAgICBpZD1cInBsYXlncm91bmQubmF2YmFyLmZpeHR1cmVcIlxuICAgICAgICB2YWx1ZT17YWN0aXZlRXhhbXBsZUlkfVxuICAgICAgICBvcHRpb25zPXtleGFtcGxlT3B0aW9uc31cbiAgICAgICAgb25WYWx1ZUNoYW5nZT17KGV4YW1wbGVJZCkgPT4ge1xuICAgICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfQUNUSVZFX0VYQU1QTEVfSURcIiwgdmFsdWU6IGV4YW1wbGVJZCB9KTtcbiAgICAgICAgICBkaXNwYXRjaEFjdGl2ZUV4YW1wbGUoZXhhbXBsZUlkIHx8IFwiXCIpO1xuICAgICAgICB9fVxuICAgICAgLz5cbiAgICApO1xuICB9LCBbc2Vzc2lvbiwgZXhhbXBsZU9wdGlvbnMsIGxvY2tzLmV4YW1wbGVJZCwgc3R1ZGlvTW9kZSwgbGFuZGluZ0FwcElkLCBhY3RpdmVFeGFtcGxlSWQsIGRpc3BhdGNoQWN0aXZlRXhhbXBsZV0pO1xuXG4gIC8qKiBAZW1vamkg8J+Om++4jyBTaGFyZWQgYnkgdGhlIGRlc2t0b3AgbmF2YmFyIGNlbnRlciBjbHVzdGVyIGFuZCB0aGUgbW9iaWxlIHBhbmVsJ3Mgc3ludGhldGljIFwiQXBwXCIgdGFiIChzZWUgYG1vYmlsZVBhbmVsVGFic2ApLiAqL1xuICBjb25zdCBtb2RlU3dpdGNoZXJFbGVtZW50ID0gdXNlTWVtbygoKSA9PiB7XG4gICAgaWYgKCFzZXNzaW9uIHx8IHNlc3Npb24uYXBwLm1vZGVzLmxlbmd0aCA8PSAxKSByZXR1cm4gbnVsbDtcbiAgICByZXR1cm4gKFxuICAgICAgPEJ1dHRvbkdyb3VwIGtleT1cIm1vZGVzXCIgaWQ9XCJwbGF5Z3JvdW5kLm5hdmJhci5tb2Rlc1wiPlxuICAgICAgICB7c2Vzc2lvbi5hcHAubW9kZXMubWFwKChtb2RlKSA9PiB7XG4gICAgICAgICAgY29uc3QgaXNBY3RpdmUgPSBhY3RpdmVNb2RlSWQgPT09IG1vZGUuaWQ7XG4gICAgICAgICAgcmV0dXJuIChcbiAgICAgICAgICAgIDxCdXR0b25Hcm91cEl0ZW1cbiAgICAgICAgICAgICAga2V5PXttb2RlLmlkfVxuICAgICAgICAgICAgICBpZD17YHBsYXlncm91bmQubmF2YmFyLm1vZGVzLiR7bW9kZS5pZH1gfVxuICAgICAgICAgICAgICBjbGFzc05hbWU9e2NuKGlzQWN0aXZlICYmIGludGVyYWN0aXZlQWN0aXZlRmlsbENsYXNzKX1cbiAgICAgICAgICAgICAgZGF0YS1zdGF0ZT17aXNBY3RpdmUgPyBcIm9uXCIgOiB1bmRlZmluZWR9XG4gICAgICAgICAgICAgIG9uQ2xpY2s9eygpID0+IGFwcGx5TW9kZUNoYW5nZShtb2RlLmlkKX1cbiAgICAgICAgICAgICAgaWNvbj17bW9kZS5pY29uSWR9XG4gICAgICAgICAgICAgIHRleHQ9e3Jlc29sdmVBcHBMYWJlbChhcHBMYWJlbHNPdmVybGF5LCBcIm1vZGVcIiwgbW9kZS5pZCwgcmVzb2x2ZU1hbmlmZXN0TGFiZWwobW9kZS5sYWJlbCwgdWlUZXJtaW5vbG9neSwgdWlMb2NhbGUpKX1cbiAgICAgICAgICAgIC8+XG4gICAgICAgICAgKTtcbiAgICAgICAgfSl9XG4gICAgICA8L0J1dHRvbkdyb3VwPlxuICAgICk7XG4gIH0sIFtzZXNzaW9uLCBhY3RpdmVNb2RlSWQsIGFwcGx5TW9kZUNoYW5nZSwgYXBwTGFiZWxzT3ZlcmxheSwgdWlUZXJtaW5vbG9neSwgdWlMb2NhbGVdKTtcblxuICBjb25zdCByZXNvbHZlZENvbW1hbmRzID0gdXNlTWVtbyhcbiAgICAoKSA9PiByZXNvbHZlQ29tbWFuZHMob3NDb21tYW5kcywgYWN0aXZlUGx1Z2luTWFuaWZlc3QsIHNlc3Npb24/LmFwcCwgYWN0aXZlTW9kZUlkLCBhcHBMYWJlbHNPdmVybGF5LCB1aVRlcm1pbm9sb2d5LCB1aUxvY2FsZSksXG4gICAgW29zQ29tbWFuZHMsIGFjdGl2ZVBsdWdpbk1hbmlmZXN0LCBzZXNzaW9uPy5hcHAsIGFjdGl2ZU1vZGVJZCwgYXBwTGFiZWxzT3ZlcmxheSwgdWlUZXJtaW5vbG9neSwgdWlMb2NhbGVdLFxuICApO1xuXG4gIGNvbnN0IGNvbW1hbmRDYXRlZ29yeUxpc3QgPSB1c2VNZW1vKCgpID0+IGNvbW1hbmRDYXRlZ29yaWVzKHJlc29sdmVkQ29tbWFuZHMpLCBbcmVzb2x2ZWRDb21tYW5kcywgdWlMb2NhbGVdKTtcblxuICAvKipcbiAgICog8J+Om++4jyBEaXNwYXRjaGVzIGEgcmVzb2x2ZWQgY29tbWFuZDogb3Mtc2NvcGUgY29tbWFuZHMgYXJlIGhhbmRsZWQgbG9jYWxseSAobm8gcHJvZ3JhbSByb3VuZCB0cmlwKTtcbiAgICogcGx1Z2luL2FwcC9tb2RlLXNjb3BlIGNvbW1hbmRzIHJvdXRlIHRocm91Z2ggdGhlIGFjdGl2ZSBzZXNzaW9uJ3MgcHJvZ3JhbSBgaGFuZGxlQ29tbWFuZGAsIG1pcnJvcmluZ1xuICAgKiBgb25BY3Rpb25gJ3MgdGFpbC4gUGx1Z2luIGNvbW1hbmRzIGFyZSBvbmx5IHJlc29sdmFibGUvZGlzcGF0Y2hhYmxlIGZvciB0aGUgYWN0aXZlIHNlc3Npb24ncyBwcm9ncmFtXG4gICAqIGluc3RhbmNlIChubyBoZWFkbGVzcy1pbnN0YW5jZSByb3V0aW5nIGZvciBub24tZm9jdXNlZCBwbHVnaW5zIHlldCkuXG4gICAqL1xuICBjb25zdCBvbkNvbW1hbmQgPSB1c2VDYWxsYmFjayhcbiAgICAoc291cmNlOiBSZXNvbHZlZENvbW1hbmRbXCJzb3VyY2VcIl0sIGNvbW1hbmRJZDogc3RyaW5nLCBhcmdzPzogUmVjb3JkPHN0cmluZywgdW5rbm93bj4pID0+IHtcbiAgICAgIC8vIPCfjqXvuI8gU2FtZSBzYW5kYm94LXN0YXJ0L3JlY29yZGVyLWFybSBzaWRlIGVmZmVjdHMgYFNUQVJUX1RVVE9SSUFMX0FDVElPTl9JRGAvYFJFQ09SRF9UVVRPUklBTF9BQ1RJT05fSURgXG4gICAgICAvLyBuZWVkIOKAlCByb3V0ZWQgdGhyb3VnaCB0aGUgYHN0YXJ0VHV0b3JpYWxSZWZgL2B0b2dnbGVUdXRvcmlhbFJlY29yZGluZ1JlZmAgYnJpZGdlIHNpbmNlIHRoZXkgbmVlZFxuICAgICAgLy8gbW9yZSBjb250ZXh0IChwbHVnaW4gYnJpZGdlLCBzYW5kYm94IHNuYXBzaG90KSB0aGFuIGEgYmFyZSBgZGlzcGF0Y2hgIGdpdmVzIGBkaXNwYXRjaE9zQ29tbWFuZGAuXG4gICAgICBpZiAoc291cmNlLmtpbmQgPT09IFwib3NcIiAmJiBjb21tYW5kSWQgPT09IFwib3MucGxheVR1dG9yaWFsXCIpIHtcbiAgICAgICAgY29uc3QgdHV0b3JpYWxJZCA9IHR5cGVvZiBhcmdzPy50dXRvcmlhbElkID09PSBcInN0cmluZ1wiID8gYXJncy50dXRvcmlhbElkIDogXCJcIjtcbiAgICAgICAgaWYgKHR1dG9yaWFsSWQpIHN0YXJ0VHV0b3JpYWxSZWYuY3VycmVudCh0dXRvcmlhbElkKTtcbiAgICAgICAgcmV0dXJuO1xuICAgICAgfVxuICAgICAgaWYgKHNvdXJjZS5raW5kID09PSBcIm9zXCIgJiYgY29tbWFuZElkID09PSBcIm9zLnJlY29yZFR1dG9yaWFsXCIpIHtcbiAgICAgICAgdG9nZ2xlVHV0b3JpYWxSZWNvcmRpbmdSZWYuY3VycmVudCgpO1xuICAgICAgICByZXR1cm47XG4gICAgICB9XG4gICAgICBpZiAoc291cmNlLmtpbmQgPT09IFwib3NcIikge1xuICAgICAgICBkaXNwYXRjaE9zQ29tbWFuZChjb21tYW5kSWQsIGFyZ3MsIGRpc3BhdGNoLCBkb2NrTGF5b3V0U3RvcmUsIGRvY2tVaVN0YXRlU3RvcmUsIGxvY2tzKTtcbiAgICAgICAgY29uc3QgbGFiZWwgPSByZXNvbHZlZENvbW1hbmRzLmZpbmQoKGVudHJ5KSA9PiBlbnRyeS5kZWZpbml0aW9uLmlkID09PSBjb21tYW5kSWQpPy5kZWZpbml0aW9uLmxhYmVsID8/IGNvbW1hbmRJZDtcbiAgICAgICAgbm90ZVNoZWxsQ29tbWFuZChjb21tYW5kSWQsIGxhYmVsLCBhcmdzKTtcbiAgICAgICAgcmV0dXJuO1xuICAgICAgfVxuICAgICAgaWYgKCFzZXNzaW9uKSByZXR1cm47XG4gICAgICAvLyDij7rvuI8gUmVjb3JkZXIgdGFwIGZvciBwbHVnaW4vYXBwL21vZGUtc2NvcGUgY29tbWFuZHMg4oCUIG1pcnJvcnMgYG9uQWN0aW9uYCdzIHRhcCBhYm92ZS5cbiAgICAgIGlmICh0dXRvcmlhbFJlY29yZGluZ1JlZi5jdXJyZW50ICYmICF0dXRvcmlhbERyaXZlblJlZi5jdXJyZW50KSB7XG4gICAgICAgIHR1dG9yaWFsUmVjb3JkZXJSZWYuY3VycmVudD8ucmVjb3JkRXZlbnQoeyBraW5kOiBcImNvbW1hbmRcIiwgY29tbWFuZDogY29tbWFuZElkLCBhcmdzIH0pO1xuICAgICAgfVxuICAgICAgY29uc3QgcGx1Z2luID0gbG9hZGVkUGx1Z2lucy5maW5kKChlbnRyeSkgPT4gZW50cnkuaGFuZGxlLnBsdWdpbklkID09PSBzZXNzaW9uLnBsdWdpbklkKT8uaGFuZGxlO1xuICAgICAgaWYgKCFwbHVnaW4/LmhhbmRsZUFjdGlvbikgcmV0dXJuO1xuICAgICAgY29uc3QgZGlzcGF0Y2hWaWV3U3RhdGUgPSBpbmplY3RBY3RpdmVVdGlsaXR5KHNlc3Npb24udmlld1N0YXRlKTtcbiAgICAgIC8vIPCfjq/vuI8gQXBwIHBhbGV0dGUgY29tbWFuZHMgc2hhcmUgdGhlIGFjdGlvbiB3aXJlICsgYGNvbW1hbmRfZnJvbV9hY3Rpb25gIGJyaWRnZSDigJQgdGhlcmUgYXJlIG5vXG4gICAgICAvLyBmcmFtZXdvcmstcmVzZXJ2ZWQgQ09NTUFORFMsIHNvIGBoYW5kbGVDb21tYW5kYC9ga2luZDpcImNvbW1hbmRcImAgYWx3YXlzIGhhcmQtZXJyb3JzIHBvaW50aW5nIGF0XG4gICAgICAvLyB0aGUgdHlwZWQgY2hhbm5lbCAoc2VlIGBWY3NEb2N1bWVudEFwcDo6ZGlzcGF0Y2hfY29tbWFuZGApLlxuICAgICAgdm9pZCBwbHVnaW5cbiAgICAgICAgLmhhbmRsZUFjdGlvbihzZXNzaW9uLmluc3RhbmNlSWQsIGVuY29kZUFjdGlvbldpcmUoeyBjb250cm9sbGVySWQ6IHNlc3Npb24uYXBwLmNvbnRyb2xsZXJJZCwgYWN0aW9uOiBjb21tYW5kSWQsIGFyZ3MgfSksIGRpc3BhdGNoVmlld1N0YXRlKVxuICAgICAgICAudGhlbigocmVzcG9uc2UpID0+IGFwcGx5SG9zdEVmZmVjdHMocmVzcG9uc2UucmVxdWVzdGVkRWZmZWN0cyA/PyBbXSwgeyAuLi5zZXNzaW9uLCB2aWV3U3RhdGU6IGRpc3BhdGNoVmlld1N0YXRlIH0sIHJlc29sdmVVaURpcnR5U2NvcGUocmVzcG9uc2UudWlTY29wZSkpKVxuICAgICAgICAuY2F0Y2goKGNvbW1hbmRFcnJvcikgPT4ge1xuICAgICAgICAgIGNvbnNvbGUuZXJyb3IoXCJbREVCVUddIGNvbW1hbmQgZmFpbGVkXCIsIGNvbW1hbmRFcnJvcik7XG4gICAgICAgIH0pO1xuICAgIH0sXG4gICAgW2FwcGx5SG9zdEVmZmVjdHMsIGRvY2tMYXlvdXRTdG9yZSwgZG9ja1VpU3RhdGVTdG9yZSwgaW5qZWN0QWN0aXZlVXRpbGl0eSwgbG9hZGVkUGx1Z2lucywgc2Vzc2lvbiwgbG9ja3MsIHJlc29sdmVkQ29tbWFuZHMsIG5vdGVTaGVsbENvbW1hbmRdLFxuICApO1xuXG4gIGNvbnN0IGNvbW1hbmRDYXRlZ29yeVRhYnMgPSB1c2VNZW1vKCgpID0+IGJ1aWxkQ29tbWFuZENhdGVnb3J5VGFicyhyZXNvbHZlZENvbW1hbmRzLCBjb21tYW5kQ2F0ZWdvcnlMaXN0LCBleHBhbmRlZENvbW1hbmRJZFJlZiwgY29tbWFuZFN0YWdlZEFyZ3NCeUNvbW1hbmRJZFJlZiwgb25Db21tYW5kLCBkaXNwYXRjaCksIFtyZXNvbHZlZENvbW1hbmRzLCBjb21tYW5kQ2F0ZWdvcnlMaXN0LCBvbkNvbW1hbmRdKTtcblxuICAvLyDwn5e677iPIGBUb29sRGVmaW5pdGlvbi5sYWJlbGAgaXMgYSBtYW5pZmVzdCBgTG9jYWxpemVkTGFiZWxgIGZpZWxkIOKAlCByZXNvbHZlZCBoZXJlLCByaWdodCBhZnRlclxuICAvLyBgcmVzb2x2ZU1vZGVUb29sc2AgKGFuIGV4dGVybmFsIGBmcmFtZXdvcmstb3MtY29yZWAgaGVscGVyIHRoaXMgZmlsZSBjYW5ub3QgZWRpdCksIHNvIGV2ZXJ5XG4gIC8vIGRvd25zdHJlYW0gY29uc3VtZXIgKGBidWlsZFRvb2xUcmVlYC9gYnVpbGRUb29sVGFic2ApIGtlZXBzIHJlYWRpbmcgYW4gYWxyZWFkeS1wbGFpbi1zdHJpbmcgYGxhYmVsYC5cbiAgY29uc3QgcmVzb2x2ZWRNb2RlVG9vbHMgPSB1c2VNZW1vKFxuICAgICgpID0+IHJlc29sdmVNb2RlVG9vbHMoc2Vzc2lvbj8uYXBwLCBhY3RpdmVNb2RlSWQpLm1hcCgodG9vbCkgPT4gKHsgLi4udG9vbCwgbGFiZWw6IHJlc29sdmVNYW5pZmVzdExhYmVsKHRvb2wubGFiZWwsIHVpVGVybWlub2xvZ3ksIHVpTG9jYWxlKSB9KSksXG4gICAgW3Nlc3Npb24/LmFwcCwgYWN0aXZlTW9kZUlkLCB1aVRlcm1pbm9sb2d5LCB1aUxvY2FsZV0sXG4gICk7XG5cbiAgY29uc3QgdG9vbFRhYnMgPSB1c2VNZW1vKFxuICAgICgpID0+IChzZXNzaW9uID8gYnVpbGRUb29sVGFicyhyZXNvbHZlZE1vZGVUb29scywgc2Vzc2lvbi5hcHAuY29udHJvbGxlcklkLCBhY3RpdmVUb29sSWRSZWYsIHRvb2xNZWFzdXJlc0J5VG9vbElkUmVmLCBvbkFjdGlvblN0YWJsZSkgOiBbXSksXG4gICAgW3Jlc29sdmVkTW9kZVRvb2xzLCBzZXNzaW9uPy5hcHAuY29udHJvbGxlcklkLCBvbkFjdGlvblN0YWJsZV0sXG4gICk7XG5cbiAgLy8jcmVnaW9uIPCfp63vuI9Eb2NrQXNzZW1ibHkg4oCUIGRlZmF1bHQgZm91ci1jb3JuZXIgYXJyYW5nZW1lbnQgKHRoZSB0d28gbWlkZGxlIGFuY2hvcnMgc3RhcnQgZW1wdHkgc2F2ZSB0aGUgY29tbWFuZCBwYWxldHRlIGluIGJvdHRvbS1taWRkbGUpICsgcGVyc2lzdGVkLW92ZXJyaWRlIHJlY29uY2lsaWF0aW9uICsgZHJhZy1hbmQtZHJvcCB3aXJpbmcuXG4gIGNvbnN0IGRlZmF1bHREb2NrID0gdXNlTWVtbygoKTogUGFuZWxEb2NrID0+IHtcbiAgICAvLyDwn6et77iPIFRvcC1sZWZ0IChXb3JrYmVuY2g6IERvY3VtZW50L0NhdGFsb2d1ZSksIHRvcC1yaWdodCAoRGV0YWlsczogSW5zcGVjdGlvbi9QYXJhbWV0ZXJzKSBhbmQgYm90dG9tLXJpZ2h0XG4gICAgLy8gKFNldHRpbmdzOiBUaGVtZS9TZXR0aW5ncykgcmVuZGVyIHRoZWlyIHRhYnMgZmxhdCwgb25lIGxldmVsIHVwIGZyb20gd2hlcmUgdGhleSB1c2VkIHRvIHNpdCDigJQgdGhlXG4gICAgLy8gY2F0ZWdvcnktYnJhbmNoIHdyYXBwZXIgdGFiIGlzIGdvbmUsIHNvIGVhY2ggbGVhZiBpcyBhIHRvcC1sZXZlbCB0b2dnbGUgaW5zdGVhZCBvZiB0d28gY2xpY2tzIGRlZXAuXG4gICAgY29uc3QgdG9wTGVmdDogUGFuZWxUYWJOb2RlW10gPSBbLi4ud29ya2JlbmNoTGVmdFRhYnNdO1xuICAgIGNvbnN0IGJvdHRvbUxlZnQ6IFBhbmVsVGFiTm9kZVtdID0gW107XG4gICAgaWYgKGZyYW1ld29ya0Rpc3BsYXlUYWJzLmxlbmd0aCA+IDApIHtcbiAgICAgIGJvdHRvbUxlZnQucHVzaCh7IGtpbmQ6IFwiYnJhbmNoXCIsIGlkOiBGUkFNRVdPUktfQ0FURUdPUllfRElTUExBWV9JRCwgaWNvbjogY2F0ZWdvcnlUYWJJY29uKGZyYW1ld29ya0Rpc3BsYXlUYWJzLCBcImxheW91dC1ncmlkXCIpLCBuYW1lOiBzaGVsbExhYmVsKFwidWkucGFuZWxUb2dnbGUuZGlzcGxheVwiKSwgb3JkZXI6IDAsIGNoaWxkcmVuOiBmcmFtZXdvcmtEaXNwbGF5VGFicyB9KTtcbiAgICB9XG4gICAgaWYgKGZyYW1ld29ya1N5bmNUYWIpIGJvdHRvbUxlZnQucHVzaChmcmFtZXdvcmtTeW5jVGFiKTtcbiAgICBjb25zdCB0b3BSaWdodDogUGFuZWxUYWJOb2RlW10gPSBbLi4uZGV0YWlsc1JpZ2h0VGFic107XG4gICAgY29uc3QgYm90dG9tUmlnaHQ6IFBhbmVsVGFiTm9kZVtdID0gWy4uLnNldHRpbmdzUmlnaHRUYWJzLCAuLi5mcmFtZXdvcmtQbHVnaW5zVGFic107XG4gICAgaWYgKGZyYW1ld29ya1V0aWxpdGllc0hpc3RvcnlUYWIpIGJvdHRvbVJpZ2h0LnB1c2goZnJhbWV3b3JrVXRpbGl0aWVzSGlzdG9yeVRhYik7XG4gICAgLy8g8J+boO+4jyBUb29sIGNhdGVnb3JpZXMgc3RheSBuZXN0ZWQgdW5kZXIgb25lIGV4cGFuZGFibGUgVG9vbCBicmFuY2gsIGV4YWN0bHkgbGlrZSBDb21tYW5kIGNhdGVnb3JpZXMsXG4gICAgLy8gcGxhY2VkIGxlZnQgb2YgQ29tbWFuZCAob3JkZXIgMCB2cyAxKSDigJQgbGlrZSBjb21tYW5kcyBub3QgYmVpbmcgd2luZG93LWxldmVsLCB0b29scyBhcmUgbm90XG4gICAgLy8gd2luZG93LWxldmVsIGVpdGhlcjsgYm90aCBsaXZlIG9ubHkgb24gdGhpcyBzaGFyZWQgbW9kZS1zY29wZWQgYW5jaG9yLlxuICAgIC8vIPCfjpvvuI8gQ29tbWFuZCBjYXRlZ29yaWVzIHN0YXkgbmVzdGVkIHVuZGVyIG9uZSBleHBhbmRhYmxlIENvbW1hbmQgYnJhbmNoICh1bmxpa2UgZmxhdCBUaGVtZS9TZXR0aW5nc1xuICAgIC8vIGZvb3RlciB0b2dnbGVzKSBzbyB0aGUgZm9sZGVkIGJvdHRvbS1taWRkbGUgY2hyb21lIHNob3dzIGEgc2luZ2xlIENvbW1hbmQgdG9nZ2xlLCBub3QgZXZlcnlcbiAgICAvLyBjYXRlZ29yeSBsZWFmIGlubGluZWQgYWxvbmcgdGhlIGZvb3Rlci5cbiAgICBjb25zdCBib3R0b21NaWRkbGU6IFBhbmVsVGFiTm9kZVtdID0gW1xuICAgICAgLi4uKHRvb2xUYWJzLmxlbmd0aCA+IDAgPyBbeyBraW5kOiBcImJyYW5jaFwiIGFzIGNvbnN0LCBpZDogRlJBTUVXT1JLX0NBVEVHT1JZX1RPT0xfSUQsIGljb246IGNhdGVnb3J5VGFiSWNvbih0b29sVGFicywgXCJoYW1tZXJcIiksIG5hbWU6IHNoZWxsTGFiZWwoXCJ1aS5wYW5lbFRvZ2dsZS50b29sXCIpLCBvcmRlcjogMCwgY2hpbGRyZW46IHRvb2xUYWJzIH1dIDogW10pLFxuICAgICAgLi4uKGNvbW1hbmRDYXRlZ29yeVRhYnMubGVuZ3RoID4gMCA/IFt7IGtpbmQ6IFwiYnJhbmNoXCIgYXMgY29uc3QsIGlkOiBGUkFNRVdPUktfQ0FURUdPUllfQ09NTUFORF9JRCwgaWNvbjogY2F0ZWdvcnlUYWJJY29uKGNvbW1hbmRDYXRlZ29yeVRhYnMsIFwid3JlbmNoXCIpLCBuYW1lOiBzaGVsbExhYmVsKFwidWkucGFuZWxUb2dnbGUuY29tbWFuZFwiKSwgb3JkZXI6IDEsIGNoaWxkcmVuOiBjb21tYW5kQ2F0ZWdvcnlUYWJzIH1dIDogW10pLFxuICAgIF07XG4gICAgcmV0dXJuIHsgYW5jaG9yczogeyBcInRvcC1sZWZ0XCI6IHRvcExlZnQsIFwidG9wLW1pZGRsZVwiOiBbXSwgXCJ0b3AtcmlnaHRcIjogdG9wUmlnaHQsIFwicmlnaHQtbWlkZGxlXCI6IFtdLCBcImJvdHRvbS1yaWdodFwiOiBib3R0b21SaWdodCwgXCJib3R0b20tbWlkZGxlXCI6IGJvdHRvbU1pZGRsZSwgXCJib3R0b20tbGVmdFwiOiBib3R0b21MZWZ0LCBcImxlZnQtbWlkZGxlXCI6IFtdIH0gfTtcbiAgfSwgW2NvbW1hbmRDYXRlZ29yeVRhYnMsIGRldGFpbHNSaWdodFRhYnMsIGZyYW1ld29ya0Rpc3BsYXlUYWJzLCBmcmFtZXdvcmtQbHVnaW5zVGFicywgZnJhbWV3b3JrU3luY1RhYiwgZnJhbWV3b3JrVXRpbGl0aWVzSGlzdG9yeVRhYiwgc2V0dGluZ3NSaWdodFRhYnMsIHRvb2xUYWJzLCB1aUxvY2FsZSwgd29ya2JlbmNoTGVmdFRhYnNdKTtcblxuICB1c2VFZmZlY3QoKCkgPT4ge1xuICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfRE9DS19PVkVSUklERVwiLCB2YWx1ZTogZG9ja0xheW91dFN0b3JlLmdldFNuYXBzaG90KCkgfSk7XG4gIH0sIFtkb2NrTGF5b3V0U3RvcmVdKTtcblxuICBjb25zdCBkb2NrID0gdXNlTWVtbygoKTogUGFuZWxEb2NrID0+IGFwcGx5RG9ja1NrZWxldG9uKGRlZmF1bHREb2NrLCBkb2NrT3ZlcnJpZGUpLCBbZGVmYXVsdERvY2ssIGRvY2tPdmVycmlkZV0pO1xuXG4gIC8vIPCfk7HvuI8gQWxsIGVpZ2h0IGFuY2hvcnMnIHRhYnMgZmxhdHRlbmVkIGludG8gdGhlIHNpbmdsZSBtb2JpbGUgcGFuZWwncyB0YWIgbGlzdCDigJQgZGVmaW5lZCBoZXJlIChhaGVhZCBvZiB0aGVcbiAgLy8gZG9jay1hc3NlbWJseSBvdmVycmlkZSBlZmZlY3RzIGJlbG93KSBzbyB0aG9zZSBlZmZlY3RzIGNhbiByZXNvbHZlIGEgbW9iaWxlLXBhbmVsIHBhdGggYWxvbmdzaWRlIHRoZVxuICAvLyBkZXNrdG9wIHBlci1hbmNob3Igb25lLlxuICBjb25zdCBtb2JpbGVQYW5lbFRhYnMgPSB1c2VNZW1vKCgpID0+IHtcbiAgICBjb25zdCBhbmNob3JUYWJzID0gQU5DSE9SUy5mbGF0TWFwKChhbmNob3IpID0+IGRlZmF1bHREb2NrLmFuY2hvcnNbYW5jaG9yXSk7XG4gICAgLy8g8J+Tse+4jyBUaGUgZXhhbXBsZSBzZWxlY3RvciBhbmQgbW9kZSBzd2l0Y2hlciBoYXZlIG5vIG5hdmJhciByb29tIG9uIG1vYmlsZSAoc2VlIGBuYXZiYXJJdGVtc2ApIOKAlCB0aGV5XG4gICAgLy8gc3VyZmFjZSBhcyBvbmUgbW9yZSB0YWIgaW4gdGhlIG1lcmdlZCBtb2JpbGUgcGFuZWwgaW5zdGVhZCwgc2hhcmluZyB0aGUgZXhhY3Qgc2FtZSBlbGVtZW50cyB0aGVcbiAgICAvLyBkZXNrdG9wIG5hdmJhciBjZW50ZXIgY2x1c3RlciByZW5kZXJzLlxuICAgIGlmICghZXhhbXBsZVNlbGVjdEVsZW1lbnQgJiYgIW1vZGVTd2l0Y2hlckVsZW1lbnQpIHJldHVybiBhbmNob3JUYWJzO1xuICAgIGNvbnN0IGFwcFRhYiA9IHNpbmdsZVRyZWVMZWFmKHtcbiAgICAgIGlkOiBcImZyYW1ld29yay5tb2JpbGUuYXBwXCIsXG4gICAgICBpY29uOiBzaGVsbFRhYkljb24oXCJzbWFydHBob25lXCIpLFxuICAgICAgbmFtZTogc2hlbGxMYWJlbChcInVpLm1vYmlsZVBhbmVsLmFwcFwiKSxcbiAgICAgIG9yZGVyOiA5OSxcbiAgICAgIHRyZWU6IHtcbiAgICAgICAgc2VjdGlvbnM6IFtcbiAgICAgICAgICB7XG4gICAgICAgICAgICBpZDogXCJmcmFtZXdvcmsubW9iaWxlLmFwcC5yb290XCIsXG4gICAgICAgICAgICBsYWJlbDogXCJcIixcbiAgICAgICAgICAgIGl0ZW1zOiBbXG4gICAgICAgICAgICAgIC4uLihleGFtcGxlU2VsZWN0RWxlbWVudCA/IFt7IGlkOiBcImZyYW1ld29yay5tb2JpbGUuYXBwLmV4YW1wbGVcIiwgbGFiZWw6IFwiXCIsIGNvbnRyb2w6IGV4YW1wbGVTZWxlY3RFbGVtZW50IH1dIDogW10pLFxuICAgICAgICAgICAgICAuLi4obW9kZVN3aXRjaGVyRWxlbWVudCA/IFt7IGlkOiBcImZyYW1ld29yay5tb2JpbGUuYXBwLm1vZGVzXCIsIGxhYmVsOiBcIlwiLCBjb250cm9sOiBtb2RlU3dpdGNoZXJFbGVtZW50IH1dIDogW10pLFxuICAgICAgICAgICAgXSxcbiAgICAgICAgICB9LFxuICAgICAgICBdLFxuICAgICAgfSxcbiAgICB9KTtcbiAgICByZXR1cm4gWy4uLmFuY2hvclRhYnMsIGFwcFRhYl07XG4gIH0sIFtkZWZhdWx0RG9jaywgZXhhbXBsZVNlbGVjdEVsZW1lbnQsIG1vZGVTd2l0Y2hlckVsZW1lbnRdKTtcblxuICAvKiog8J+XhO+4jyBTa2lwcyB0aGUgdmVyeSBmaXJzdCAocHJlLWh5ZHJhdGlvbikgY29tbWl0IHNvIGEgcGVyc2lzdGVkIHNrZWxldG9uIGlzbid0IGNsb2JiZXJlZCB3aXRoIGBudWxsYCBiZWZvcmUgdGhlIHNlZWRpbmcgZWZmZWN0IGFib3ZlIGhhcyBhIGNoYW5jZSB0byByZWFkIGFuZCBhcHBseSBpdC4gKi9cbiAgY29uc3QgZG9ja1BlcnNpc3RlZE9uY2VSZWYgPSB1c2VSZWYoZmFsc2UpO1xuICB1c2VFZmZlY3QoKCkgPT4ge1xuICAgIGlmICghZG9ja1BlcnNpc3RlZE9uY2VSZWYuY3VycmVudCkge1xuICAgICAgZG9ja1BlcnNpc3RlZE9uY2VSZWYuY3VycmVudCA9IHRydWU7XG4gICAgICByZXR1cm47XG4gICAgfVxuICAgIGNvbnN0IG5leHRTa2VsZXRvbiA9IGRvY2tTa2VsZXRvbk9mKGRvY2spO1xuICAgIGNvbnN0IGRlZmF1bHRTa2VsZXRvbiA9IGRvY2tTa2VsZXRvbk9mKGRlZmF1bHREb2NrKTtcbiAgICBkb2NrTGF5b3V0U3RvcmUuc2F2ZShkb2NrU2tlbGV0b25zRXF1YWwobmV4dFNrZWxldG9uLCBkZWZhdWx0U2tlbGV0b24pID8gbnVsbCA6IG5leHRTa2VsZXRvbik7XG4gIH0sIFtkb2NrLCBkZWZhdWx0RG9jaywgZG9ja0xheW91dFN0b3JlXSk7XG5cbiAgdXNlRWZmZWN0KCgpID0+IHtcbiAgICBkaXNwYXRjaCh7IHR5cGU6IFwiSFlEUkFURV9ET0NLX1VJXCIsIHZhbHVlOiBkb2NrVWlTdGF0ZVN0b3JlLmdldFNuYXBzaG90KCkgfSk7XG4gIH0sIFtkb2NrVWlTdGF0ZVN0b3JlXSk7XG5cbiAgLyoqIPCfl4TvuI8gU2FtZSBmaXJzdC1jb21taXQtc2tpcCBhcyB0aGUgZG9jayBza2VsZXRvbiBlZmZlY3QgYWJvdmUsIGJ1dCBhbHNvIHJlLWFybXMgd2hlbiB0aGUgc3RvcmUgaWRlbnRpdHkgaXRzZWxmIGNoYW5nZXMgKGFwcCBzd2l0Y2gpIOKAlCBvdGhlcndpc2UgdGhlIG5ldyBhcHAncyBwcmUtaHlkcmF0aW9uIHN0YXRlIHdvdWxkIGJlIHdyaXR0ZW4gaW50byBpdHMgb3duIGtleSBvbiB0aGUgZmlyc3QgcG9zdC1zd2l0Y2ggY29tbWl0LiAqL1xuICBjb25zdCBkb2NrVWlQZXJzaXN0ZWRPbmNlUmVmID0gdXNlUmVmKGZhbHNlKTtcbiAgY29uc3QgZG9ja1VpUGVyc2lzdGVkU3RvcmVSZWYgPSB1c2VSZWYoZG9ja1VpU3RhdGVTdG9yZSk7XG4gIHVzZUVmZmVjdCgoKSA9PiB7XG4gICAgaWYgKGRvY2tVaVBlcnNpc3RlZFN0b3JlUmVmLmN1cnJlbnQgIT09IGRvY2tVaVN0YXRlU3RvcmUpIHtcbiAgICAgIGRvY2tVaVBlcnNpc3RlZFN0b3JlUmVmLmN1cnJlbnQgPSBkb2NrVWlTdGF0ZVN0b3JlO1xuICAgICAgZG9ja1VpUGVyc2lzdGVkT25jZVJlZi5jdXJyZW50ID0gZmFsc2U7XG4gICAgfVxuICAgIGlmICghZG9ja1VpUGVyc2lzdGVkT25jZVJlZi5jdXJyZW50KSB7XG4gICAgICBkb2NrVWlQZXJzaXN0ZWRPbmNlUmVmLmN1cnJlbnQgPSB0cnVlO1xuICAgICAgcmV0dXJuO1xuICAgIH1cbiAgICBjb25zdCBhbmNob3JzOiBQYXJ0aWFsPFJlY29yZDxBbmNob3IsIERvY2tVaVBhbmVsU3RhdGU+PiA9IHt9O1xuICAgIGZvciAoY29uc3QgYW5jaG9yIG9mIEFOQ0hPUlMpIHtcbiAgICAgIGNvbnN0IHBhbmVsU3RhdGUgPSBwYW5lbHNbYW5jaG9yXTtcbiAgICAgIGNvbnN0IGVudHJ5OiBEb2NrVWlQYW5lbFN0YXRlID0ge307XG4gICAgICBpZiAocGFuZWxTdGF0ZS52aXNpYmxlKSBlbnRyeS52aXNpYmxlID0gdHJ1ZTtcbiAgICAgIGlmIChwYW5lbFN0YXRlLnNpemUgIT09IERFRkFVTFRfUEFORUxfV0lEVEhfUFgpIGVudHJ5LnNpemUgPSBwYW5lbFN0YXRlLnNpemU7XG4gICAgICBpZiAocGFuZWxTdGF0ZS5wYXRoLmxlbmd0aCA+IDApIGVudHJ5LnBhdGggPSBwYW5lbFN0YXRlLnBhdGg7XG4gICAgICBpZiAoT2JqZWN0LmtleXMoZW50cnkpLmxlbmd0aCA+IDApIGFuY2hvcnNbYW5jaG9yXSA9IGVudHJ5O1xuICAgIH1cbiAgICBjb25zdCBoYXNQYXRoTWVtb3J5ID0gT2JqZWN0LmtleXMocGFuZWxQYXRoTWVtb3J5KS5sZW5ndGggPiAwO1xuICAgIGNvbnN0IGhhc1RyZWVPcGVuID0gT2JqZWN0LmtleXModHJlZU9wZW5TdGF0ZXMpLmxlbmd0aCA+IDA7XG4gICAgY29uc3QgaXNEZWZhdWx0ID0gT2JqZWN0LmtleXMoYW5jaG9ycykubGVuZ3RoID09PSAwICYmICFoYXNQYXRoTWVtb3J5ICYmICFoYXNUcmVlT3BlbjtcbiAgICBkb2NrVWlTdGF0ZVN0b3JlLnNhdmUoaXNEZWZhdWx0ID8gbnVsbCA6IHsgdmVyc2lvbjogMywgYW5jaG9ycywgcGF0aE1lbW9yeTogaGFzUGF0aE1lbW9yeSA/IHBhbmVsUGF0aE1lbW9yeSA6IHVuZGVmaW5lZCwgdHJlZU9wZW46IGhhc1RyZWVPcGVuID8gdHJlZU9wZW5TdGF0ZXMgOiB1bmRlZmluZWQgfSk7XG4gIH0sIFtwYW5lbHMsIHBhbmVsUGF0aE1lbW9yeSwgdHJlZU9wZW5TdGF0ZXMsIGRvY2tVaVN0YXRlU3RvcmVdKTtcblxuICBjb25zdCBoYW5kbGVUYWJEb2NrRHJvcCA9IHVzZUNhbGxiYWNrKFxuICAgIChtb3ZlOiBQYW5lbFRhYkRvY2tNb3ZlKSA9PiB7XG4gICAgICBjb25zdCBuZXh0RG9jayA9IG1vdmVUYWJJbkRvY2soZG9jaywgbW92ZSk7XG4gICAgICBpZiAobmV4dERvY2sgPT09IGRvY2spIHJldHVybjtcbiAgICAgIGNvbnN0IG5leHRTa2VsZXRvbiA9IGRvY2tTa2VsZXRvbk9mKG5leHREb2NrKTtcbiAgICAgIGNvbnN0IGRlZmF1bHRTa2VsZXRvbiA9IGRvY2tTa2VsZXRvbk9mKGRlZmF1bHREb2NrKTtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfRE9DS19PVkVSUklERVwiLCB2YWx1ZTogZG9ja1NrZWxldG9uc0VxdWFsKG5leHRTa2VsZXRvbiwgZGVmYXVsdFNrZWxldG9uKSA/IG51bGwgOiBuZXh0U2tlbGV0b24gfSk7XG4gICAgICBjb25zdCB0YXJnZXRQYXRoID0gZmluZFBhbmVsVGFiUGF0aChuZXh0RG9jay5hbmNob3JzW21vdmUudGFyZ2V0LmFuY2hvcl0sIG1vdmUudGFiSWQpO1xuICAgICAgaWYgKHRhcmdldFBhdGgpIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfUEFORUxfUEFUSFwiLCBhbmNob3I6IG1vdmUudGFyZ2V0LmFuY2hvciwgdmFsdWU6IHRhcmdldFBhdGggfSk7XG4gICAgICBpZiAobW92ZS5mcm9tQW5jaG9yICE9PSBtb3ZlLnRhcmdldC5hbmNob3IpIHtcbiAgICAgICAgY29uc3Qgc291cmNlVGFicyA9IG5leHREb2NrLmFuY2hvcnNbbW92ZS5mcm9tQW5jaG9yXTtcbiAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9QQU5FTF9QQVRIXCIsIGFuY2hvcjogbW92ZS5mcm9tQW5jaG9yLCB2YWx1ZTogKHByZXYpID0+IHJlY29uY2lsZUFjdGl2ZVBhdGgoc291cmNlVGFicywgcHJldiwgcGFuZWxUYWJDaGlsZHJlbikgfSk7XG4gICAgICB9XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1BBTkVMX1ZJU0lCTEVcIiwgYW5jaG9yOiBtb3ZlLnRhcmdldC5hbmNob3IsIHZhbHVlOiB0cnVlIH0pO1xuICAgICAgbm90ZVNoZWxsQ29tbWFuZChcInNoZWxsLmRvY2tNb3ZlXCIsIHNoZWxsTGFiZWwoXCJ1aS5zaGVsbENvbW1hbmQuZG9ja01vdmVcIiksIHsgdGFiSWQ6IG1vdmUudGFiSWQsIGZyb21BbmNob3I6IG1vdmUuZnJvbUFuY2hvciwgdG9BbmNob3I6IG1vdmUudGFyZ2V0LmFuY2hvciB9KTtcbiAgICB9LFxuICAgIFtkb2NrLCBkZWZhdWx0RG9jaywgbm90ZVNoZWxsQ29tbWFuZF0sXG4gICk7XG5cbiAgY29uc3QgaGFuZGxlVHJlZVVuaXREb2NrRHJvcCA9IHVzZUNhbGxiYWNrKFxuICAgIChtb3ZlOiBQYW5lbFRyZWVVbml0RG9ja01vdmUpID0+IHtcbiAgICAgIGNvbnN0IG5leHREb2NrID0gbW92ZVRyZWVVbml0SW5Eb2NrKGRvY2ssIG1vdmUpO1xuICAgICAgaWYgKG5leHREb2NrID09PSBkb2NrKSByZXR1cm47XG4gICAgICBjb25zdCBuZXh0U2tlbGV0b24gPSBkb2NrU2tlbGV0b25PZihuZXh0RG9jayk7XG4gICAgICBjb25zdCBkZWZhdWx0U2tlbGV0b24gPSBkb2NrU2tlbGV0b25PZihkZWZhdWx0RG9jayk7XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0RPQ0tfT1ZFUlJJREVcIiwgdmFsdWU6IGRvY2tTa2VsZXRvbnNFcXVhbChuZXh0U2tlbGV0b24sIGRlZmF1bHRTa2VsZXRvbikgPyBudWxsIDogbmV4dFNrZWxldG9uIH0pO1xuICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9QQU5FTF9WSVNJQkxFXCIsIGFuY2hvcjogbW92ZS50YXJnZXQuYW5jaG9yLCB2YWx1ZTogdHJ1ZSB9KTtcbiAgICAgIG5vdGVTaGVsbENvbW1hbmQoXCJzaGVsbC5kb2NrTW92ZVwiLCBzaGVsbExhYmVsKFwidWkuc2hlbGxDb21tYW5kLmRvY2tNb3ZlXCIpLCB7IHRvQW5jaG9yOiBtb3ZlLnRhcmdldC5hbmNob3IgfSk7XG4gICAgfSxcbiAgICBbZG9jaywgZGVmYXVsdERvY2ssIG5vdGVTaGVsbENvbW1hbmRdLFxuICApO1xuXG4gIGNvbnN0IHN0dWRpb092ZXJyaWRlVGFiSWQgPSBzdHVkaW9Nb2RlICYmIHNlc3Npb24/LmFwcC5pZCA9PT0gaG9zdEFwcElkID8gKHBhbmVsPy5hY3RpdmVQYW5lbFRhYiA/PyBob3N0Q2F0YWxvZ3VlVGFiSWQpIDogdW5kZWZpbmVkO1xuICBjb25zdCBzdHVkaW9PdmVycmlkZUFuY2hvciA9IHN0dWRpb092ZXJyaWRlVGFiSWQgPyBmaW5kUGFuZWxUYWJJbkRvY2soZG9jaywgc3R1ZGlvT3ZlcnJpZGVUYWJJZCk/LmFuY2hvciA6IHVuZGVmaW5lZDtcbiAgY29uc3QgZGV0YWlsc092ZXJyaWRlVGFiSWQgPSBwYW5lbD8uYWN0aXZlUGFuZWxUYWI7XG4gIGNvbnN0IGRldGFpbHNPdmVycmlkZUFuY2hvciA9IGRldGFpbHNPdmVycmlkZVRhYklkID8gZmluZFBhbmVsVGFiSW5Eb2NrKGRvY2ssIGRldGFpbHNPdmVycmlkZVRhYklkKT8uYW5jaG9yIDogdW5kZWZpbmVkO1xuXG4gIC8qKiBAZW1vamkg8J+Ok++4jyBUaGUgY3VycmVudCBpbnRyb2R1Y3Rpb24gc3RlcCdzIHRhcmdldCBlbGVtZW50IGlkcyAoYGludHJvZHVjZWAgKyBgc2hvd2ApLCBjbGFzc2lmaWVkIGJ5XG4gICAqIHNoYXBlIOKAlCBgbnVsbGAgdW5sZXNzIHRoYXQgc2hhcGUgaXMgcHJlc2VudCwgc28gZXZlcnkgcmV2ZWFsIG92ZXJyaWRlIGJlbG93IChoZXJlIGFuZCBpblxuICAgKiBgbW9kZVdpbmRvd3NgKSBpcyBhIHBsYWluIHRydXRoaW5lc3MgY2hlY2suIEEgZm9sZGVkIHV0aWxpdHkgYmFyL0FjdGlvbnMgcmFpbC9kb2NrIHBhbmVsIHdvdWxkXG4gICAqIG90aGVyd2lzZSBoaWRlIHRoZSB0YXJnZXQgZnJvbSBldmVyIG1vdW50aW5nIChzZWUgYHVzZUludHJvZHVjdGlvbkFuY2hvclJlY3RgKSwgbGVhdmluZyB0aGUgc3RlcFxuICAgKiBjZW50ZXJlZCB3aXRoIG5vIGN1dG91dCBhbmQgbm8gd2F5IGZvciB0aGUgdXNlciB0byBmaW5kIHdoYXQgdG8gZG8uIElkcyBhcmUgbWF0Y2hlZCwgbmV2ZXJcbiAgICogcmVjb25zdHJ1Y3RlZDogYSBgZnJhbWV3b3JrLndpbmRvdy57c2VnbWVudH1gIGlkJ3Mgc2VnbWVudCBpcyBgZWxlbWVudElkU2VnbWVudCh3aW5kb3dJZClgLCBhIGxvc3N5XG4gICAqIGNhbWVsQ2FzZSBub3JtYWxpemF0aW9uIOKAlCBjb21wYXJpbmcgYGVsZW1lbnRJZFNlZ21lbnQod2luZG93SWQpID09PSBzZWdtZW50YCBPUiB0aGUgc2FtZSBmb3IgdGhlXG4gICAqIGluc3RhbmNlJ3Mgd2luZG93LWtpbmQgaWQgaXMgdGhlIG9ubHkgc2FmZSBjaGVjayAoVG9wL1BlcnNwZWN0aXZlIGluc3RhbmNlcyBzaGFyZSBhIGtpbmQpLiAqL1xuICBjb25zdCBhY3RpdmVJbnRyb2R1Y3Rpb25TdGVwID0gYWN0aXZlSW50cm9kdWN0aW9uICYmIGludHJvZHVjdGlvblN0ZXBJbmRleCAhPSBudWxsID8gKGFjdGl2ZUludHJvZHVjdGlvbi5zdGVwc1tpbnRyb2R1Y3Rpb25TdGVwSW5kZXhdID8/IG51bGwpIDogbnVsbDtcbiAgY29uc3QgaW50cm9kdWN0aW9uRWxlbWVudElkcyA9IHVzZU1lbW8oXG4gICAgKCk6IHJlYWRvbmx5IHN0cmluZ1tdID0+IChhY3RpdmVJbnRyb2R1Y3Rpb25TdGVwID8gW2FjdGl2ZUludHJvZHVjdGlvblN0ZXAuaW50cm9kdWNlLCAuLi5hY3RpdmVJbnRyb2R1Y3Rpb25TdGVwLnNob3ddLmZpbHRlcigoaWQpOiBpZCBpcyBzdHJpbmcgPT4gQm9vbGVhbihpZCkpIDogW10pLFxuICAgIFthY3RpdmVJbnRyb2R1Y3Rpb25TdGVwXSxcbiAgKTtcbiAgY29uc3QgaW50cm9kdWN0aW9uVXRpbGl0eUlkID0gdXNlTWVtbygoKSA9PiB7XG4gICAgaWYgKCFzZXNzaW9uKSByZXR1cm4gbnVsbDtcbiAgICBjb25zdCB1dGlsaXRpZXMgPSBzZXNzaW9uLmFwcC51dGlsaXRpZXMgPz8gW107XG4gICAgcmV0dXJuIGludHJvZHVjdGlvbkVsZW1lbnRJZHMuZmluZCgoaWQpID0+IHV0aWxpdGllcy5zb21lKCh1dGlsaXR5KSA9PiB1dGlsaXR5LmlkID09PSBpZCkpID8/IG51bGw7XG4gIH0sIFtpbnRyb2R1Y3Rpb25FbGVtZW50SWRzLCBzZXNzaW9uXSk7XG4gIGNvbnN0IGludHJvZHVjdGlvbkFjdGlvbldpbmRvd1NlZ21lbnQgPSB1c2VNZW1vKCgpID0+IHtcbiAgICBmb3IgKGNvbnN0IGlkIG9mIGludHJvZHVjdGlvbkVsZW1lbnRJZHMpIHtcbiAgICAgIGNvbnN0IHJlc3QgPSBpZC5zdGFydHNXaXRoKFwiZnJhbWV3b3JrLndpbmRvdy5cIikgPyBpZC5zbGljZShcImZyYW1ld29yay53aW5kb3cuXCIubGVuZ3RoKSA6IG51bGw7XG4gICAgICBjb25zdCBhY3Rpb25JbmRleCA9IHJlc3Q/LmluZGV4T2YoXCIuYWN0aW9uLlwiKSA/PyAtMTtcbiAgICAgIGlmIChyZXN0ICYmIGFjdGlvbkluZGV4ID49IDApIHJldHVybiByZXN0LnNsaWNlKDAsIGFjdGlvbkluZGV4KTtcbiAgICB9XG4gICAgcmV0dXJuIG51bGw7XG4gIH0sIFtpbnRyb2R1Y3Rpb25FbGVtZW50SWRzXSk7XG4gIGNvbnN0IGludHJvZHVjdGlvblBhbmVsVGFiSWQgPSB1c2VNZW1vKCgpID0+IHtcbiAgICBmb3IgKGNvbnN0IGlkIG9mIGludHJvZHVjdGlvbkVsZW1lbnRJZHMpIHtcbiAgICAgIGlmIChpZC5zdGFydHNXaXRoKFwiZnJhbWV3b3JrLnBhbmVsVGFiLlwiKSkge1xuICAgICAgICBjb25zdCByZXN0ID0gaWQuc2xpY2UoXCJmcmFtZXdvcmsucGFuZWxUYWIuXCIubGVuZ3RoKTtcbiAgICAgICAgcmV0dXJuIHJlc3QuZW5kc1dpdGgoXCIuZmlyc3REcmFnZ2FibGVcIikgPyByZXN0LnNsaWNlKDAsIC1cIi5maXJzdERyYWdnYWJsZVwiLmxlbmd0aCkgOiByZXN0O1xuICAgICAgfVxuICAgIH1cbiAgICByZXR1cm4gbnVsbDtcbiAgfSwgW2ludHJvZHVjdGlvbkVsZW1lbnRJZHNdKTtcbiAgLyoqIPCfm6DvuI8gVG9vbCBpZHMgdGhlIGFjdGl2ZSBzdGVwIGFza3MgdGhlIHVzZXIgdG8gYWN0aXZhdGUgKGBpbnRlcmFjdGlvbnNgIG9mIGtpbmQgYHRvb2xgLCBvciBhIGJhcmVcbiAgICogYHRvb2wuPGlkPmAgaW50cm9kdWNlL3Nob3cpLiBSZXZlYWxzIHRoZSBUb29sIGNhdGVnb3J5IGNocm9tZSBzbyB0aGUgbGVhZiB0YWIgY2FuIGJlIHByZXNzZWQg4oCUXG4gICAqIG5ldmVyIGRyaWxscyBpbnRvIHRoZSBsZWFmIGl0c2VsZiAodGhhdCB3b3VsZCBvcGVuIHRoZSBpbmFjdGl2ZSBhY3RpdmF0ZS10b2dnbGUgdHJlZSBhbmQsIHZpYSB0YWJcbiAgICogc2VsZWN0aW9uLCBhdXRvLWFjdGl2YXRlICsgY2VsZWJyYXRlIGJlZm9yZSB0aGUgdXNlciBhY3RzKS4gKi9cbiAgY29uc3QgaW50cm9kdWN0aW9uVG9vbFBpY2tJZHMgPSB1c2VNZW1vKCgpOiByZWFkb25seSBzdHJpbmdbXSA9PiB7XG4gICAgY29uc3QgZnJvbUludGVyYWN0aW9ucyA9IChhY3RpdmVJbnRyb2R1Y3Rpb25TdGVwPy5pbnRlcmFjdGlvbnMgPz8gW10pXG4gICAgICAuZmlsdGVyKChpbnRlcmFjdGlvbik6IGludGVyYWN0aW9uIGlzIEludHJvZHVjdGlvbkludGVyYWN0aW9uICYgeyByZWFkb25seSBvbjogeyByZWFkb25seSBraW5kOiBcInRvb2xcIjsgcmVhZG9ubHkgaWQ6IHN0cmluZyB9IH0gPT4gaW50ZXJhY3Rpb24ub24ua2luZCA9PT0gXCJ0b29sXCIpXG4gICAgICAubWFwKChpbnRlcmFjdGlvbikgPT4gaW50ZXJhY3Rpb24ub24uaWQpO1xuICAgIGlmIChmcm9tSW50ZXJhY3Rpb25zLmxlbmd0aCA+IDApIHJldHVybiBmcm9tSW50ZXJhY3Rpb25zO1xuICAgIHJldHVybiBpbnRyb2R1Y3Rpb25FbGVtZW50SWRzLmZsYXRNYXAoKGlkKSA9PiB7XG4gICAgICBjb25zdCBtYXRjaCA9IC9edG9vbFxcLihbYS16XVthLXpBLVowLTldKikkLy5leGVjKGlkKTtcbiAgICAgIHJldHVybiBtYXRjaD8uWzFdID8gW21hdGNoWzFdXSA6IFtdO1xuICAgIH0pO1xuICB9LCBbYWN0aXZlSW50cm9kdWN0aW9uU3RlcCwgaW50cm9kdWN0aW9uRWxlbWVudElkc10pO1xuICBjb25zdCBpbnRyb2R1Y3Rpb25QYW5lbFRhYkFuY2hvciA9IGludHJvZHVjdGlvblBhbmVsVGFiSWQgPyBmaW5kUGFuZWxUYWJJbkRvY2soZG9jaywgaW50cm9kdWN0aW9uUGFuZWxUYWJJZCk/LmFuY2hvciA6IHVuZGVmaW5lZDtcbiAgY29uc3QgaW50cm9kdWN0aW9uVXRpbGl0eVdpbmRvd0lkID0gdXNlTWVtbygoKSA9PiB7XG4gICAgaWYgKCFpbnRyb2R1Y3Rpb25VdGlsaXR5SWQgfHwgIXNlc3Npb24pIHJldHVybiBudWxsO1xuICAgIGZvciAoY29uc3Qga2luZCBvZiBzZXNzaW9uLmFwcC53aW5kb3dLaW5kcykge1xuICAgICAgY29uc3QgdXRpbGl0aWVzID0gcmVzb2x2ZVV0aWxpdHlOb2RlcyhzZXNzaW9uLmFwcCwga2luZCwgbnVsbCwga2luZC5pZCwgYXBwTGFiZWxzT3ZlcmxheSwgdWlUZXJtaW5vbG9neSwgdWlMb2NhbGUpO1xuICAgICAgaWYgKHV0aWxpdHlOb2RlVHJlZUNvbnRhaW5zSWQodXRpbGl0aWVzLCBpbnRyb2R1Y3Rpb25VdGlsaXR5SWQpKSByZXR1cm4ga2luZC5pZDtcbiAgICB9XG4gICAgcmV0dXJuIG51bGw7XG4gIH0sIFthcHBMYWJlbHNPdmVybGF5LCBpbnRyb2R1Y3Rpb25VdGlsaXR5SWQsIHNlc3Npb24sIHVpVGVybWlub2xvZ3ksIHVpTG9jYWxlXSk7XG4gIC8qKiDwn46T77iPIFdpbmRvdy1raW5kIGlkIHdob3NlIG1lYXN1cmVzIHRyZWUgb3ducyBhbiBpbnRyb2R1Y2Uvc2hvdyBtZWFzdXJlIGlkIOKAlCBmb3JjZS11bmZvbGRzIHRoZSBXaW5kb3dcbiAgICogT3B0aW9ucyByYWlsIHNvIHRhcmdldHMgbGlrZSBgcHV6emxlM2QtcGxheS12b3J0ZXgtc2hvd2AgY2FuIG1vdW50IGZvciB0aGUgdG91ci4gKi9cbiAgY29uc3QgaW50cm9kdWN0aW9uTWVhc3VyZVdpbmRvd0lkID0gdXNlTWVtbygoKSA9PiB7XG4gICAgaWYgKCFzZXNzaW9uIHx8IGludHJvZHVjdGlvbkVsZW1lbnRJZHMubGVuZ3RoID09PSAwKSByZXR1cm4gbnVsbDtcbiAgICBmb3IgKGNvbnN0IGtpbmQgb2Ygc2Vzc2lvbi5hcHAud2luZG93S2luZHMpIHtcbiAgICAgIGNvbnN0IGtpbmRNZWFzdXJlcyA9IGtpbmQub3B0aW9ucy5tZWFzdXJlcyA/PyBbXTtcbiAgICAgIGlmIChpbnRyb2R1Y3Rpb25FbGVtZW50SWRzLnNvbWUoKGlkKSA9PiB3aW5kb3dNZWFzdXJlVHJlZUNvbnRhaW5zSWQoa2luZE1lYXN1cmVzLCBpZCkpKSByZXR1cm4ga2luZC5pZDtcbiAgICAgIGZvciAoY29uc3QgW3dpbmRvd0lkLCBtZWFzdXJlc10gb2YgT2JqZWN0LmVudHJpZXMod2luZG93TWVhc3VyZXNCeVdpbmRvd0lkKSkge1xuICAgICAgICBpZiAoIWludHJvZHVjdGlvbkVsZW1lbnRJZHMuc29tZSgoaWQpID0+IHdpbmRvd01lYXN1cmVUcmVlQ29udGFpbnNJZChtZWFzdXJlcywgaWQpKSkgY29udGludWU7XG4gICAgICAgIGlmICh3aW5kb3dJZCA9PT0ga2luZC5pZCB8fCBleHRyYVdpbmRvd0luc3RhbmNlcy5zb21lKChpbnN0YW5jZSkgPT4gaW5zdGFuY2UuaWQgPT09IHdpbmRvd0lkICYmIGluc3RhbmNlLndpbmRvd0tpbmRJZCA9PT0ga2luZC5pZCkpIHJldHVybiBraW5kLmlkO1xuICAgICAgfVxuICAgIH1cbiAgICByZXR1cm4gbnVsbDtcbiAgfSwgW2V4dHJhV2luZG93SW5zdGFuY2VzLCBpbnRyb2R1Y3Rpb25FbGVtZW50SWRzLCBzZXNzaW9uLCB3aW5kb3dNZWFzdXJlc0J5V2luZG93SWRdKTtcblxuICAvKiog8J+boO+4jyBUb29sIGlkIHdob3NlIG1lYXN1cmUgdHJlZSBvd25zIGFuIGludHJvZHVjZS9zaG93IGlkIOKAlCBrZWVwcyBtb2RlLWxldmVsIHRvb2xzIGxpa2UgZmlsbFxuICAgKiBhY3RpdmUgc28gdGFyZ2V0cyBzdWNoIGFzIGBwdXp6bGUzZC1wbGF5LWRpc3RyaWJ1dGlvbmAgc3RheSBtb3VudGVkIGZvciB0aGUgdG91ci4gKi9cbiAgY29uc3QgaW50cm9kdWN0aW9uVG9vbElkID0gdXNlTWVtbygoKSA9PiB7XG4gICAgaWYgKGludHJvZHVjdGlvbkVsZW1lbnRJZHMubGVuZ3RoID09PSAwKSByZXR1cm4gbnVsbDtcbiAgICBmb3IgKGNvbnN0IFt0b29sSWQsIG1lYXN1cmVzXSBvZiBPYmplY3QuZW50cmllcyh0b29sTWVhc3VyZXNCeVRvb2xJZCkpIHtcbiAgICAgIGlmIChpbnRyb2R1Y3Rpb25FbGVtZW50SWRzLnNvbWUoKGlkKSA9PiB3aW5kb3dNZWFzdXJlVHJlZUNvbnRhaW5zSWQobWVhc3VyZXMsIGlkKSkpIHJldHVybiB0b29sSWQ7XG4gICAgfVxuICAgIHJldHVybiBudWxsO1xuICB9LCBbaW50cm9kdWN0aW9uRWxlbWVudElkcywgdG9vbE1lYXN1cmVzQnlUb29sSWRdKTtcblxuICBjb25zdCBsYXN0SW50cm9kdWN0aW9uVG9vbElkUmVmID0gdXNlUmVmPHN0cmluZyB8IG51bGw+KG51bGwpO1xuICB1c2VFZmZlY3QoKCkgPT4ge1xuICAgIGlmICghaW50cm9kdWN0aW9uVG9vbElkIHx8ICFzZXNzaW9uKSB7XG4gICAgICBsYXN0SW50cm9kdWN0aW9uVG9vbElkUmVmLmN1cnJlbnQgPSBudWxsO1xuICAgICAgcmV0dXJuO1xuICAgIH1cbiAgICBpZiAobGFzdEludHJvZHVjdGlvblRvb2xJZFJlZi5jdXJyZW50ID09PSBpbnRyb2R1Y3Rpb25Ub29sSWQgJiYgYWN0aXZlVG9vbElkUmVmLmN1cnJlbnQgPT09IGludHJvZHVjdGlvblRvb2xJZCkgcmV0dXJuO1xuICAgIGxhc3RJbnRyb2R1Y3Rpb25Ub29sSWRSZWYuY3VycmVudCA9IGludHJvZHVjdGlvblRvb2xJZDtcbiAgICBpZiAoYWN0aXZlVG9vbElkUmVmLmN1cnJlbnQgPT09IGludHJvZHVjdGlvblRvb2xJZCkgcmV0dXJuO1xuICAgIG9uQWN0aW9uU3RhYmxlKHsgY29udHJvbGxlcklkOiBzZXNzaW9uLmFwcC5jb250cm9sbGVySWQsIGFjdGlvbjogU0VUX0FDVElWRV9UT09MX0FDVElPTl9JRCwgYXJnczogeyB0b29sSWQ6IGludHJvZHVjdGlvblRvb2xJZCB9IH0pO1xuICB9LCBbaW50cm9kdWN0aW9uVG9vbElkLCBvbkFjdGlvblN0YWJsZSwgc2Vzc2lvbl0pO1xuXG4gIC8qKiDwn5ug77iPIFRvb2wtcGljayBzdGVwcyAoZS5nLiBGw7xsbGVuKTogb3BlbiB0aGUgVG9vbCBjYXRlZ29yeSBzbyBgdG9vbC48aWQ+YCBsZWFmIHRhYnMgbW91bnQgaW4gdGhlXG4gICAqIHBhbmVsIGNocm9tZSwgY2xlYXIgYW55IGFscmVhZHktYWN0aXZlIHRvb2wgc28gdGhlIHVzZXIgbXVzdCBhY3RpdmF0ZSBpdCwgYW5kIG5ldmVyIHNlbGVjdCB0aGVcbiAgICogbGVhZiBwYXRoIChzZWxlY3RpbmcgYXV0by1hY3RpdmF0ZXMgYW5kIHdvdWxkIGNlbGVicmF0ZSBiZWZvcmUgdGhleSBhY3QpLiAqL1xuICBjb25zdCBsYXN0SW50cm9kdWN0aW9uVG9vbFBpY2tTdGVwSWRSZWYgPSB1c2VSZWY8c3RyaW5nIHwgbnVsbD4obnVsbCk7XG4gIHVzZUVmZmVjdCgoKSA9PiB7XG4gICAgaWYgKCFzZXNzaW9uIHx8IGludHJvZHVjdGlvblRvb2xQaWNrSWRzLmxlbmd0aCA9PT0gMCB8fCAhYWN0aXZlSW50cm9kdWN0aW9uU3RlcCkge1xuICAgICAgbGFzdEludHJvZHVjdGlvblRvb2xQaWNrU3RlcElkUmVmLmN1cnJlbnQgPSBudWxsO1xuICAgICAgcmV0dXJuO1xuICAgIH1cbiAgICAvLyDwn5ug77iPIE1lYXN1cmUtZHJpdmVuIGtlZXAtYWxpdmUgKGBpbnRyb2R1Y3Rpb25Ub29sSWRgKSBvd25zIGFjdGl2YXRpb24gZm9yIHN0ZXBzIHRoYXQgaW50cm9kdWNlXG4gICAgLy8gdG9vbCBtZWFzdXJlcyAoZmlsbC1kaXN0cmlidXRpb24pIOKAlCBkb24ndCBmaWdodCBpdCBieSBjbGVhcmluZyB0aGUgdG9vbC5cbiAgICBpZiAoaW50cm9kdWN0aW9uVG9vbElkKSByZXR1cm47XG4gICAgaWYgKGxhc3RJbnRyb2R1Y3Rpb25Ub29sUGlja1N0ZXBJZFJlZi5jdXJyZW50ID09PSBhY3RpdmVJbnRyb2R1Y3Rpb25TdGVwLmlkKSByZXR1cm47XG4gICAgbGFzdEludHJvZHVjdGlvblRvb2xQaWNrU3RlcElkUmVmLmN1cnJlbnQgPSBhY3RpdmVJbnRyb2R1Y3Rpb25TdGVwLmlkO1xuICAgIGZvciAoY29uc3QgdG9vbElkIG9mIGludHJvZHVjdGlvblRvb2xQaWNrSWRzKSB7XG4gICAgICBpZiAoYWN0aXZlVG9vbElkUmVmLmN1cnJlbnQgPT09IHRvb2xJZCkge1xuICAgICAgICBvbkFjdGlvblN0YWJsZSh7IGNvbnRyb2xsZXJJZDogc2Vzc2lvbi5hcHAuY29udHJvbGxlcklkLCBhY3Rpb246IFNFVF9BQ1RJVkVfVE9PTF9BQ1RJT05fSUQsIGFyZ3M6IHsgdG9vbElkOiBcIlwiIH0gfSk7XG4gICAgICB9XG4gICAgfVxuICAgIGlmIChtb2JpbGUpIHtcbiAgICAgIGNvbnN0IHJlc29sdmVkID0gZmluZFBhbmVsVGFiUGF0aChtb2JpbGVQYW5lbFRhYnMsIEZSQU1FV09SS19DQVRFR09SWV9UT09MX0lEKTtcbiAgICAgIGlmIChyZXNvbHZlZCkgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9NT0JJTEVfUEFORUxfUEFUSFwiLCB2YWx1ZTogcmVzb2x2ZWQgfSk7XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX01PQklMRV9QQU5FTF9WSVNJQkxFXCIsIHZhbHVlOiB0cnVlIH0pO1xuICAgICAgcmV0dXJuO1xuICAgIH1cbiAgICBjb25zdCB0b29sQW5jaG9yID0gZmluZFBhbmVsVGFiSW5Eb2NrKGRvY2ssIEZSQU1FV09SS19DQVRFR09SWV9UT09MX0lEKT8uYW5jaG9yID8/IFwiYm90dG9tLW1pZGRsZVwiO1xuICAgIGNvbnN0IHJlc29sdmVkID0gZmluZFBhbmVsVGFiUGF0aChkb2NrLmFuY2hvcnNbdG9vbEFuY2hvcl0sIEZSQU1FV09SS19DQVRFR09SWV9UT09MX0lEKTtcbiAgICBpZiAocmVzb2x2ZWQpIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfUEFORUxfUEFUSFwiLCBhbmNob3I6IHRvb2xBbmNob3IsIHZhbHVlOiByZXNvbHZlZCB9KTtcbiAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1BBTkVMX1ZJU0lCTEVcIiwgYW5jaG9yOiB0b29sQW5jaG9yLCB2YWx1ZTogdHJ1ZSB9KTtcbiAgfSwgW2FjdGl2ZUludHJvZHVjdGlvblN0ZXAsIGRvY2ssIGludHJvZHVjdGlvblRvb2xJZCwgaW50cm9kdWN0aW9uVG9vbFBpY2tJZHMsIG1vYmlsZSwgbW9iaWxlUGFuZWxUYWJzLCBvbkFjdGlvblN0YWJsZSwgc2Vzc2lvbl0pO1xuXG4gIGNvbnN0IGxhc3RJbnRyb2R1Y3Rpb25QYW5lbFRhYklkUmVmID0gdXNlUmVmPHN0cmluZyB8IHVuZGVmaW5lZD4odW5kZWZpbmVkKTtcbiAgdXNlRWZmZWN0KCgpID0+IHtcbiAgICBpZiAoIWludHJvZHVjdGlvblBhbmVsVGFiSWQgfHwgIWludHJvZHVjdGlvblBhbmVsVGFiQW5jaG9yKSB7XG4gICAgICBsYXN0SW50cm9kdWN0aW9uUGFuZWxUYWJJZFJlZi5jdXJyZW50ID0gdW5kZWZpbmVkO1xuICAgICAgcmV0dXJuO1xuICAgIH1cbiAgICBpZiAobGFzdEludHJvZHVjdGlvblBhbmVsVGFiSWRSZWYuY3VycmVudCA9PT0gaW50cm9kdWN0aW9uUGFuZWxUYWJJZCkgcmV0dXJuO1xuICAgIGxhc3RJbnRyb2R1Y3Rpb25QYW5lbFRhYklkUmVmLmN1cnJlbnQgPSBpbnRyb2R1Y3Rpb25QYW5lbFRhYklkO1xuICAgIGlmIChtb2JpbGUpIHtcbiAgICAgIGNvbnN0IHJlc29sdmVkID0gZmluZFBhbmVsVGFiUGF0aChtb2JpbGVQYW5lbFRhYnMsIGludHJvZHVjdGlvblBhbmVsVGFiSWQpO1xuICAgICAgaWYgKHJlc29sdmVkKSBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX01PQklMRV9QQU5FTF9QQVRIXCIsIHZhbHVlOiByZXNvbHZlZCB9KTtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfTU9CSUxFX1BBTkVMX1ZJU0lCTEVcIiwgdmFsdWU6IHRydWUgfSk7XG4gICAgICByZXR1cm47XG4gICAgfVxuICAgIGNvbnN0IHJlc29sdmVkID0gZmluZFBhbmVsVGFiUGF0aChkb2NrLmFuY2hvcnNbaW50cm9kdWN0aW9uUGFuZWxUYWJBbmNob3JdLCBpbnRyb2R1Y3Rpb25QYW5lbFRhYklkKTtcbiAgICBpZiAocmVzb2x2ZWQpIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfUEFORUxfUEFUSFwiLCBhbmNob3I6IGludHJvZHVjdGlvblBhbmVsVGFiQW5jaG9yLCB2YWx1ZTogcmVzb2x2ZWQgfSk7XG4gICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9QQU5FTF9WSVNJQkxFXCIsIGFuY2hvcjogaW50cm9kdWN0aW9uUGFuZWxUYWJBbmNob3IsIHZhbHVlOiB0cnVlIH0pO1xuICB9LCBbaW50cm9kdWN0aW9uUGFuZWxUYWJJZCwgaW50cm9kdWN0aW9uUGFuZWxUYWJBbmNob3IsIGRvY2ssIG1vYmlsZSwgbW9iaWxlUGFuZWxUYWJzXSk7XG5cbiAgLyoqIPCfjpPvuI8gUGFuZWwgaW50ZXJhY3Rpb25zIGNvbXBsZXRlIHdoZW4gdGhlaXIgbmFtZWQgcGFuZWwgdGFiIGlzIG9wZW4gYW5kIHZpc2libGUg4oCUIGNoZWNrZWQgZm9yIGV2ZXJ5XG4gICAqIGBwYW5lbGAgaW50ZXJhY3Rpb24gb2YgdGhlIGFjdGl2ZSBzdGVwLCBub3QganVzdCB0aGUgZmlyc3QsIHNvIGEgc3RlcCBjYW4gcmVxdWlyZSBvcGVuaW5nIHNldmVyYWwuICovXG4gIHVzZUVmZmVjdCgoKSA9PiB7XG4gICAgaWYgKCFhY3RpdmVJbnRyb2R1Y3Rpb25TdGVwKSByZXR1cm47XG4gICAgZm9yIChjb25zdCBpbnRlcmFjdGlvbiBvZiBhY3RpdmVJbnRyb2R1Y3Rpb25TdGVwLmludGVyYWN0aW9ucyA/PyBbXSkge1xuICAgICAgaWYgKGludGVyYWN0aW9uLm9uLmtpbmQgIT09IFwicGFuZWxcIikgY29udGludWU7XG4gICAgICBjb25zdCB0YWJJZCA9IGludGVyYWN0aW9uLm9uLmlkO1xuICAgICAgY29uc3QgbG9jYXRlZCA9IGZpbmRQYW5lbFRhYkluRG9jayhkb2NrLCB0YWJJZCk7XG4gICAgICBpZiAoIWxvY2F0ZWQpIGNvbnRpbnVlO1xuICAgICAgY29uc3QgcGFuZWwgPSBwYW5lbHNbbG9jYXRlZC5hbmNob3JdO1xuICAgICAgaWYgKCFwYW5lbC52aXNpYmxlIHx8ICFwYW5lbC5wYXRoLmluY2x1ZGVzKHRhYklkKSkgY29udGludWU7XG4gICAgICBjb21wbGV0ZUludHJvZHVjdGlvbkludGVyYWN0aW9uKChjYW5kaWRhdGUpID0+IGNhbmRpZGF0ZS5vbi5raW5kID09PSBcInBhbmVsXCIgJiYgY2FuZGlkYXRlLm9uLmlkID09PSB0YWJJZCk7XG4gICAgfVxuICB9LCBbYWN0aXZlSW50cm9kdWN0aW9uU3RlcCwgY29tcGxldGVJbnRyb2R1Y3Rpb25JbnRlcmFjdGlvbiwgZG9jaywgcGFuZWxzXSk7XG5cbiAgLyoqIPCfjpPvuI8gRXhwYW5kIGludGVyYWN0aW9ucyBzdGFydCB3aXRoIGV2ZXJ5IG5hbWVkIHRyZWUgc2VjdGlvbiBmb3JjZWQgY2xvc2VkIG9uIHN0ZXAgZW50cnksIHRoZW5cbiAgICogY29tcGxldGUgaW5kaXZpZHVhbGx5IGFzIHRoZSB1c2VyIG9wZW5zIGVhY2ggb25lLiAqL1xuICBjb25zdCBsYXN0SW50cm9kdWN0aW9uRXhwYW5kU3RlcElkUmVmID0gdXNlUmVmPHN0cmluZyB8IG51bGw+KG51bGwpO1xuICB1c2VFZmZlY3QoKCkgPT4ge1xuICAgIGNvbnN0IGV4cGFuZEludGVyYWN0aW9ucyA9IChhY3RpdmVJbnRyb2R1Y3Rpb25TdGVwPy5pbnRlcmFjdGlvbnMgPz8gW10pLmZpbHRlcigoaW50ZXJhY3Rpb24pID0+IGludGVyYWN0aW9uLm9uLmtpbmQgPT09IFwiZXhwYW5kXCIpO1xuICAgIGlmICghYWN0aXZlSW50cm9kdWN0aW9uU3RlcCB8fCBleHBhbmRJbnRlcmFjdGlvbnMubGVuZ3RoID09PSAwKSB7XG4gICAgICBsYXN0SW50cm9kdWN0aW9uRXhwYW5kU3RlcElkUmVmLmN1cnJlbnQgPSBudWxsO1xuICAgICAgcmV0dXJuO1xuICAgIH1cbiAgICBpZiAobGFzdEludHJvZHVjdGlvbkV4cGFuZFN0ZXBJZFJlZi5jdXJyZW50ICE9PSBhY3RpdmVJbnRyb2R1Y3Rpb25TdGVwLmlkKSB7XG4gICAgICBsYXN0SW50cm9kdWN0aW9uRXhwYW5kU3RlcElkUmVmLmN1cnJlbnQgPSBhY3RpdmVJbnRyb2R1Y3Rpb25TdGVwLmlkO1xuICAgICAgZm9yIChjb25zdCBpbnRlcmFjdGlvbiBvZiBleHBhbmRJbnRlcmFjdGlvbnMpIHtcbiAgICAgICAgY29uc3Qgc3RhdGVTdWZmaXggPSBgdHJlZS1zZWN0aW9uLSR7aW50ZXJhY3Rpb24ub24uaWR9YDtcbiAgICAgICAgY29uc3QgY2F0YWxvZ3VlS2V5ID0gYCR7RlJBTUVXT1JLX1BBTkVMX1RBQl9DQVRBTE9HVUVfSUR9LnRyZWU6JHtzdGF0ZVN1ZmZpeH1gO1xuICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1RSRUVfT1BFTl9TVEFURVwiLCBpZDogY2F0YWxvZ3VlS2V5LCBvcGVuOiBmYWxzZSB9KTtcbiAgICAgIH1cbiAgICAgIHJldHVybjtcbiAgICB9XG4gICAgZm9yIChjb25zdCBpbnRlcmFjdGlvbiBvZiBleHBhbmRJbnRlcmFjdGlvbnMpIHtcbiAgICAgIGNvbnN0IHNlY3Rpb25JZCA9IGludGVyYWN0aW9uLm9uLmlkO1xuICAgICAgY29uc3Qgc3RhdGVTdWZmaXggPSBgdHJlZS1zZWN0aW9uLSR7c2VjdGlvbklkfWA7XG4gICAgICBjb25zdCBleHBhbmRlZCA9IE9iamVjdC5lbnRyaWVzKHRyZWVPcGVuU3RhdGVzKS5zb21lKChba2V5LCBvcGVuXSkgPT4gb3BlbiAmJiBrZXkuZW5kc1dpdGgoc3RhdGVTdWZmaXgpKTtcbiAgICAgIGlmIChleHBhbmRlZCkgY29tcGxldGVJbnRyb2R1Y3Rpb25JbnRlcmFjdGlvbigoY2FuZGlkYXRlKSA9PiBjYW5kaWRhdGUub24ua2luZCA9PT0gXCJleHBhbmRcIiAmJiBjYW5kaWRhdGUub24uaWQgPT09IHNlY3Rpb25JZCk7XG4gICAgfVxuICB9LCBbYWN0aXZlSW50cm9kdWN0aW9uU3RlcCwgY29tcGxldGVJbnRyb2R1Y3Rpb25JbnRlcmFjdGlvbiwgdHJlZU9wZW5TdGF0ZXNdKTtcblxuICAvKiog8J+nre+4jyBQcm9ncmVzc2l2ZSByZXZlYWwgbWVhbnMgYSBzdG9yZWQgcGF0aCBjYW4gbGVnaXRpbWF0ZWx5IGVuZCBhdCBhIGJyYW5jaCAob3IgYmUgZW1wdHkpIOKAlCB0aGlzIGlzIG5vdyBhIHBsYWluIHBlci1hbmNob3IgdHJ1bmNhdGlvbi12YWxpZGF0ZSwgbm8gb3ZlcnJpZGUgcmVhc3NlcnRpb24gKHNlZSB0aGUgd3JpdGUtdGhyb3VnaCBlZmZlY3RzIGJlbG93KS4gKi9cbiAgY29uc3QgcGFuZWxBY3RpdmVQYXRocyA9IHVzZU1lbW8oKCk6IFJlY29yZDxBbmNob3IsIHJlYWRvbmx5IHN0cmluZ1tdPiA9PiB7XG4gICAgY29uc3QgcmVzdWx0ID0ge30gYXMgUmVjb3JkPEFuY2hvciwgcmVhZG9ubHkgc3RyaW5nW10+O1xuICAgIGZvciAoY29uc3QgYW5jaG9yIG9mIEFOQ0hPUlMpIHJlc3VsdFthbmNob3JdID0gcmVjb25jaWxlQWN0aXZlUGF0aChkb2NrLmFuY2hvcnNbYW5jaG9yXSwgcGFuZWxzW2FuY2hvcl0ucGF0aCwgcGFuZWxUYWJDaGlsZHJlbik7XG4gICAgcmV0dXJuIHJlc3VsdDtcbiAgfSwgW3BhbmVscywgZG9ja10pO1xuXG4gIC8qKlxuICAgKiDwn6et77iPIEdlbmVyYWxpemVzIHRoZSBvbGQgYGxlZnRQYW5lbEFjdGl2ZVBhdGhgL2ByaWdodFBhbmVsQWN0aXZlUGF0aGAgc3R1ZGlvL3BsdWdpbiBcInNuYXAgdG8gdGhlIGFjdGl2ZSBwYW5lbFxuICAgKiB0YWJcIiBvdmVycmlkZXMgYWNyb3NzIGFsbCBlaWdodCBhbmNob3JzLiBXcml0ZS10aHJvdWdoIHJhdGhlciB0aGFuIHJlYWQtdGltZTogZWFjaCBvdmVycmlkZSBkaXNwYXRjaGVzXG4gICAqIGBTRVRfUEFORUxfUEFUSGAgb25seSB3aGVuIGl0cyB0YXJnZXQgdGFiIGlkIGFjdHVhbGx5IGNoYW5nZXMsIHNvIGEgdXNlcidzIG93biBjb2xsYXBzZS9uYXZpZ2F0aW9uXG4gICAqIGFmdGVyd2FyZCBzdGlja3MgaW5zdGVhZCBvZiBiZWluZyByZWFzc2VydGVkIG9uIGV2ZXJ5IHJlbmRlciAocHJvZ3Jlc3NpdmUgcmV2ZWFsIG1hZGUgcmVhZC10aW1lIHJlYXNzZXJ0aW9uXG4gICAqIGZpZ2h0IHRoZSB1c2VyJ3Mgb3duIGNvbGxhcHNlcykuIFN0dWRpbyB3aW5zIG92ZXIgZGV0YWlscyB3aGVuIGJvdGggd291bGQgdG91Y2ggdGhlIHNhbWUgYW5jaG9yLlxuICAgKiovXG4gIGNvbnN0IGxhc3RTdHVkaW9PdmVycmlkZVRhYklkUmVmID0gdXNlUmVmPHN0cmluZyB8IHVuZGVmaW5lZD4odW5kZWZpbmVkKTtcbiAgdXNlRWZmZWN0KCgpID0+IHtcbiAgICBpZiAoIXN0dWRpb092ZXJyaWRlVGFiSWQgfHwgIXN0dWRpb092ZXJyaWRlQW5jaG9yKSB7XG4gICAgICBsYXN0U3R1ZGlvT3ZlcnJpZGVUYWJJZFJlZi5jdXJyZW50ID0gdW5kZWZpbmVkO1xuICAgICAgcmV0dXJuO1xuICAgIH1cbiAgICBpZiAobGFzdFN0dWRpb092ZXJyaWRlVGFiSWRSZWYuY3VycmVudCA9PT0gc3R1ZGlvT3ZlcnJpZGVUYWJJZCkgcmV0dXJuO1xuICAgIGxhc3RTdHVkaW9PdmVycmlkZVRhYklkUmVmLmN1cnJlbnQgPSBzdHVkaW9PdmVycmlkZVRhYklkO1xuICAgIGlmIChtb2JpbGUpIHtcbiAgICAgIGlmIChtb2JpbGVQYW5lbFBhdGhbMF0gPT09IEZSQU1FV09SS19DQVRFR09SWV9ESVNQTEFZX0lEKSByZXR1cm47XG4gICAgICBjb25zdCByZXNvbHZlZCA9IGZpbmRQYW5lbFRhYlBhdGgobW9iaWxlUGFuZWxUYWJzLCBzdHVkaW9PdmVycmlkZVRhYklkKTtcbiAgICAgIGlmIChyZXNvbHZlZCkgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9NT0JJTEVfUEFORUxfUEFUSFwiLCB2YWx1ZTogcmVzb2x2ZWQgfSk7XG4gICAgICByZXR1cm47XG4gICAgfVxuICAgIGlmIChwYW5lbHNbc3R1ZGlvT3ZlcnJpZGVBbmNob3JdLnBhdGhbMF0gPT09IEZSQU1FV09SS19DQVRFR09SWV9ESVNQTEFZX0lEKSByZXR1cm47XG4gICAgY29uc3QgcmVzb2x2ZWQgPSBmaW5kUGFuZWxUYWJQYXRoKGRvY2suYW5jaG9yc1tzdHVkaW9PdmVycmlkZUFuY2hvcl0sIHN0dWRpb092ZXJyaWRlVGFiSWQpO1xuICAgIGlmIChyZXNvbHZlZCkgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9QQU5FTF9QQVRIXCIsIGFuY2hvcjogc3R1ZGlvT3ZlcnJpZGVBbmNob3IsIHZhbHVlOiByZXNvbHZlZCB9KTtcbiAgfSwgW3N0dWRpb092ZXJyaWRlVGFiSWQsIHN0dWRpb092ZXJyaWRlQW5jaG9yLCBkb2NrLCBwYW5lbHMsIG1vYmlsZSwgbW9iaWxlUGFuZWxUYWJzLCBtb2JpbGVQYW5lbFBhdGhdKTtcblxuICBjb25zdCBsYXN0RGV0YWlsc092ZXJyaWRlVGFiSWRSZWYgPSB1c2VSZWY8c3RyaW5nIHwgdW5kZWZpbmVkPih1bmRlZmluZWQpO1xuICB1c2VFZmZlY3QoKCkgPT4ge1xuICAgIGlmICghZGV0YWlsc092ZXJyaWRlVGFiSWQgfHwgIWRldGFpbHNPdmVycmlkZUFuY2hvcikge1xuICAgICAgbGFzdERldGFpbHNPdmVycmlkZVRhYklkUmVmLmN1cnJlbnQgPSB1bmRlZmluZWQ7XG4gICAgICByZXR1cm47XG4gICAgfVxuICAgIGlmIChsYXN0RGV0YWlsc092ZXJyaWRlVGFiSWRSZWYuY3VycmVudCA9PT0gZGV0YWlsc092ZXJyaWRlVGFiSWQpIHJldHVybjtcbiAgICBsYXN0RGV0YWlsc092ZXJyaWRlVGFiSWRSZWYuY3VycmVudCA9IGRldGFpbHNPdmVycmlkZVRhYklkO1xuICAgIGlmIChkZXRhaWxzT3ZlcnJpZGVBbmNob3IgPT09IHN0dWRpb092ZXJyaWRlQW5jaG9yKSByZXR1cm47XG4gICAgLy8g8J+nre+4jyBTZXR0aW5ncyB0YWJzIHJlbmRlciBmbGF0IG5vdyAobm8gY2F0ZWdvcnkgYnJhbmNoIHRvIGNoZWNrIGFnYWluc3QpIOKAlCBza2lwIHRoZSBvdmVycmlkZSBpZiB0aGVcbiAgICAvLyBhbmNob3IncyBhY3RpdmUgbGVhZiBhbHJlYWR5IGJlbG9uZ3MgdG8gU2V0dGluZ3MsIHNvIGJyb3dzaW5nIFRoZW1lL1NldHRpbmdzIHRoZXJlIGRvZXNuJ3QgZ2V0IHN0b21wZWQuXG4gICAgaWYgKG1vYmlsZSkge1xuICAgICAgaWYgKHNldHRpbmdzUmlnaHRUYWJzLnNvbWUoKHRhYikgPT4gdGFiLmlkID09PSBtb2JpbGVQYW5lbFBhdGhbMF0pKSByZXR1cm47XG4gICAgICBjb25zdCByZXNvbHZlZCA9IGZpbmRQYW5lbFRhYlBhdGgobW9iaWxlUGFuZWxUYWJzLCBkZXRhaWxzT3ZlcnJpZGVUYWJJZCk7XG4gICAgICBpZiAocmVzb2x2ZWQpIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfTU9CSUxFX1BBTkVMX1BBVEhcIiwgdmFsdWU6IHJlc29sdmVkIH0pO1xuICAgICAgcmV0dXJuO1xuICAgIH1cbiAgICBpZiAoc2V0dGluZ3NSaWdodFRhYnMuc29tZSgodGFiKSA9PiB0YWIuaWQgPT09IHBhbmVsc1tkZXRhaWxzT3ZlcnJpZGVBbmNob3JdLnBhdGhbMF0pKSByZXR1cm47XG4gICAgY29uc3QgcmVzb2x2ZWQgPSBmaW5kUGFuZWxUYWJQYXRoKGRvY2suYW5jaG9yc1tkZXRhaWxzT3ZlcnJpZGVBbmNob3JdLCBkZXRhaWxzT3ZlcnJpZGVUYWJJZCk7XG4gICAgaWYgKHJlc29sdmVkKSBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1BBTkVMX1BBVEhcIiwgYW5jaG9yOiBkZXRhaWxzT3ZlcnJpZGVBbmNob3IsIHZhbHVlOiByZXNvbHZlZCB9KTtcbiAgfSwgW2RldGFpbHNPdmVycmlkZVRhYklkLCBkZXRhaWxzT3ZlcnJpZGVBbmNob3IsIHN0dWRpb092ZXJyaWRlQW5jaG9yLCBkb2NrLCBwYW5lbHMsIHNldHRpbmdzUmlnaHRUYWJzLCBtb2JpbGUsIG1vYmlsZVBhbmVsVGFicywgbW9iaWxlUGFuZWxQYXRoXSk7XG4gIC8vI2VuZHJlZ2lvbiDwn6et77iPRG9ja0Fzc2VtYmx5XG5cbiAgY29uc3QgbW9iaWxlUGFuZWwgPSB1c2VNZW1vKCgpID0+IHtcbiAgICBpZiAobW9iaWxlUGFuZWxUYWJzLmxlbmd0aCA9PT0gMCkgcmV0dXJuIHVuZGVmaW5lZDtcbiAgICByZXR1cm4ge1xuICAgICAgdmlzaWJsZTogbW9iaWxlUGFuZWxWaXNpYmxlLFxuICAgICAgdGFiczogbW9iaWxlUGFuZWxUYWJzLFxuICAgICAgYWN0aXZlVGFiUGF0aDogbW9iaWxlUGFuZWxQYXRoLFxuICAgICAgb25BY3RpdmVUYWJQYXRoQ2hhbmdlOiAocGF0aDogcmVhZG9ubHkgc3RyaW5nW10pID0+IHtcbiAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9NT0JJTEVfUEFORUxfUEFUSFwiLCB2YWx1ZTogcGF0aCB9KTtcbiAgICAgICAgY29uc3QgdGFiSWQgPSBwYXRoW3BhdGgubGVuZ3RoIC0gMV07XG4gICAgICAgIC8vIPCfjLHvuI8gUHJvZ3Jlc3NpdmUgcGF0aHMgb2Z0ZW4gZW5kIGF0IGEgYnJhbmNoIChvciBhcmUgZW1wdHkpIOKAlCBvbmx5IGxlYXZlcyBhcmUgbWVhbmluZ2Z1bCBcImFjdGl2ZSBwYW5lbCB0YWJcIiBzZWxlY3Rpb25zLlxuICAgICAgICBpZiAodGFiSWQgJiYgc3R1ZGlvTW9kZSAmJiBzZXNzaW9uPy5hcHAuaWQgPT09IGhvc3RBcHBJZCAmJiBmaW5kUGFuZWxUYWJOb2RlKG1vYmlsZVBhbmVsVGFicywgcGF0aCk/LmtpbmQgPT09IFwibGVhZlwiKSB7XG4gICAgICAgICAgb25BY3Rpb24oeyBjb250cm9sbGVySWQ6IHNlc3Npb24uYXBwLmNvbnRyb2xsZXJJZCwgYWN0aW9uOiBcInNldEFjdGl2ZVBhbmVsVGFiXCIsIGFyZ3M6IHsgdGFiSWQgfSB9KTtcbiAgICAgICAgfVxuICAgICAgfSxcbiAgICAgIHBhdGhNZW1vcnk6IHBhbmVsUGF0aE1lbW9yeSxcbiAgICAgIG9uUGF0aE1lbW9yeUNoYW5nZTogKHZhbHVlOiBSZWFkb25seTxSZWNvcmQ8c3RyaW5nLCBzdHJpbmc+PikgPT4gZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9QQU5FTF9QQVRIX01FTU9SWVwiLCB2YWx1ZSB9KSxcbiAgICAgIHRyZWVPcGVuU3RhdGVzLFxuICAgICAgb25UcmVlT3BlblN0YXRlQ2hhbmdlOiAoaWQ6IHN0cmluZywgb3BlbjogYm9vbGVhbikgPT4gZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9UUkVFX09QRU5fU1RBVEVcIiwgaWQsIG9wZW4gfSksXG4gICAgICAvLyDimbvvuI8gTGF6eSB0b29sL2NvbW1hbmQgdHJlZXMgcmVhZCBtZWFzdXJlcyArIGFjdGl2ZSB0b29sIGZyb20gcmVmcyDigJQgcmV2aXNpb24gZm9yY2VzIHJlLXJlc29sdmUuXG4gICAgICB0cmVlQ29udGVudFJldmlzaW9uOiB7IGFjdGl2ZVRvb2xJZCwgdG9vbE1lYXN1cmVzQnlUb29sSWQsIGFjdGlvblBhbmVTdGFnZWRBcmdzQnlLZXkgfSxcbiAgICB9O1xuICB9LCBbbW9iaWxlUGFuZWxWaXNpYmxlLCBtb2JpbGVQYW5lbFBhdGgsIG1vYmlsZVBhbmVsVGFicywgb25BY3Rpb24sIHBhbmVsUGF0aE1lbW9yeSwgc2Vzc2lvbiwgc3R1ZGlvTW9kZSwgdHJlZU9wZW5TdGF0ZXMsIGhvc3RBcHBJZCwgYWN0aXZlVG9vbElkLCB0b29sTWVhc3VyZXNCeVRvb2xJZCwgYWN0aW9uUGFuZVN0YWdlZEFyZ3NCeUtleV0pO1xuXG4gIHVzZUVmZmVjdCgoKSA9PiB7XG4gICAgaWYgKGV4YW1wbGVPcHRpb25zLmxlbmd0aCA9PT0gMCkgcmV0dXJuO1xuICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfQUNUSVZFX0VYQU1QTEVfSURcIiwgdmFsdWU6IChjdXJyZW50KSA9PiAoIWN1cnJlbnQgfHwgZXhhbXBsZU9wdGlvbnMuc29tZSgob3B0aW9uKSA9PiBvcHRpb24uaWQgPT09IGN1cnJlbnQpID8gY3VycmVudCA6IFwiXCIpIH0pO1xuICB9LCBbZXhhbXBsZU9wdGlvbnMsIHNlc3Npb24/LmFwcC5pZCwgc2Vzc2lvbj8ucGx1Z2luSWRdKTtcblxuICAvLyDwn46b77iPIEFubm91bmNlcyB0aGUgYm9vdCBleGFtcGxlIHRvIHRoZSBmcmVzaCBzZXNzaW9uIGV4YWN0bHkgb25jZSBwZXIgaW5zdGFuY2UuIFdoZW4gbm90aGluZyBpc1xuICAvLyBsb2NrZWQvZGVmYXVsdGVkLCBzZWVkIHRoZSBmaXJzdCByZWdpc3RlcmVkIGV4YW1wbGUgc28gdGhlIGRyb3Bkb3duIG1hdGNoZXMgdGhlIHBsdWdpbiBkZWZhdWx0XG4gIC8vIGRvY3VtZW50IChlLmcuIHByb2NlZHVyYWwzZCBoZXhhZ29uYWwgY29sdW1uKSDigJQgc2FtZSBydWxlIGFzIHdncHUgYHN5bmNfc2Vzc2lvbl9jaHJvbWVgLlxuICAvLyBTdHVkaW8tbW9kZSByb3V0ZXMgbG9hZCBkb2N1bWVudHMgdmlhIGBhcHBseVNoZWxsVXJpYC9gb3BlblNwYWNlYDsgbmV2ZXIgYm9vdC1vdmVycmlkZSB0aG9zZS5cbiAgdXNlRWZmZWN0KCgpID0+IHtcbiAgICBpZiAoZXhhbXBsZU9wdGlvbnMubGVuZ3RoID09PSAwIHx8ICFzZXNzaW9uKSByZXR1cm47XG4gICAgaWYgKHN0dWRpb01vZGUpIHtcbiAgICAgIG5vRXhhbXBsZVJlc2V0SW5zdGFuY2VJZFJlZi5jdXJyZW50ID0gc2Vzc2lvbi5pbnN0YW5jZUlkO1xuICAgICAgcmV0dXJuO1xuICAgIH1cbiAgICBpZiAobm9FeGFtcGxlUmVzZXRJbnN0YW5jZUlkUmVmLmN1cnJlbnQgPT09IHNlc3Npb24uaW5zdGFuY2VJZCkgcmV0dXJuO1xuICAgIG5vRXhhbXBsZVJlc2V0SW5zdGFuY2VJZFJlZi5jdXJyZW50ID0gc2Vzc2lvbi5pbnN0YW5jZUlkO1xuICAgIGNvbnN0IGV4YW1wbGVJZCA9IHJlc29sdmVCb290RXhhbXBsZUlkKGFjdGl2ZUV4YW1wbGVJZCwgZXhhbXBsZU9wdGlvbnMsIGRlZmF1bHRzLmV4YW1wbGVJZCk7XG4gICAgaWYgKGV4YW1wbGVJZCAhPT0gYWN0aXZlRXhhbXBsZUlkKSB7XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0FDVElWRV9FWEFNUExFX0lEXCIsIHZhbHVlOiBleGFtcGxlSWQgfSk7XG4gICAgfVxuICAgIGRpc3BhdGNoQWN0aXZlRXhhbXBsZShleGFtcGxlSWQpO1xuICB9LCBbYWN0aXZlRXhhbXBsZUlkLCBkZWZhdWx0cy5leGFtcGxlSWQsIGRpc3BhdGNoQWN0aXZlRXhhbXBsZSwgZXhhbXBsZU9wdGlvbnMsIHNlc3Npb24sIHN0dWRpb01vZGVdKTtcblxuICAvLyNyZWdpb24g8J+Om++4j1BhbmVsVGFiQmFySG9zdGluZyDigJQgYGJ1aWxkUGFuZWxTZWxlY3Rpb25Qcm9wc2AgaXMgdGhlIHNpbmdsZSBzb3VyY2Ugb2YgYW4gYW5jaG9yJ3MgdGFiXG4gIC8vIHNlbGVjdGlvbiBzdGF0ZSwgc2hhcmVkIGJ5IHRoZSBjaHJvbWUtaG9zdGVkIGBQYW5lbENocm9tZVRhYkJhcmAgKGJlbG93LCBmb3IgYW5jaG9ycyBpblxuICAvLyB7QGxpbmsgUEFORUxfVEFCX0JBUl9IT1NUU30pIGFuZCB0aGUgZmxvYXRpbmcgYFBhbmVsYCBpdHNlbGYgKGBidWlsZFBhbmVsUHJvcHNgKSDigJQgdGhlIHR3byBob3N0cyBvZiB0aGVcbiAgLy8gU0FNRSBhbmNob3IgYWx3YXlzIHJlYWQvd3JpdGUgdGhlIGV4YWN0IHNhbWUgY29udHJvbGxlZCBzdGF0ZS5cbiAgY29uc3QgYnVpbGRQYW5lbFNlbGVjdGlvblByb3BzID0gdXNlQ2FsbGJhY2soXG4gICAgKGFuY2hvcjogQW5jaG9yKTogUGFuZWxUYWJTZWxlY3Rpb25PcHRpb25zID0+ICh7XG4gICAgICB0YWJzOiBkb2NrLmFuY2hvcnNbYW5jaG9yXSxcbiAgICAgIHZpc2libGU6IHBhbmVsc1thbmNob3JdLnZpc2libGUsXG4gICAgICBvblZpc2libGVDaGFuZ2U6ICh2YWx1ZTogYm9vbGVhbikgPT4ge1xuICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1BBTkVMX1ZJU0lCTEVcIiwgYW5jaG9yLCB2YWx1ZSB9KTtcbiAgICAgICAgbm90ZVNoZWxsQ29tbWFuZChcInNoZWxsLnBhbmVsVG9nZ2xlXCIsIHNoZWxsTGFiZWwoXCJ1aS5zaGVsbENvbW1hbmQucGFuZWxUb2dnbGVcIiksIHsgYW5jaG9yLCB2aXNpYmxlOiB2YWx1ZSB9KTtcbiAgICAgIH0sXG4gICAgICBhY3RpdmVUYWJQYXRoOiBwYW5lbEFjdGl2ZVBhdGhzW2FuY2hvcl0sXG4gICAgICBvbkFjdGl2ZVRhYlBhdGhDaGFuZ2U6IChwYXRoOiByZWFkb25seSBzdHJpbmdbXSkgPT4ge1xuICAgICAgICBjb25zdCBwYXRoQ2hhbmdlZCA9IChwYW5lbEFjdGl2ZVBhdGhzW2FuY2hvcl0gPz8gW10pLmpvaW4oXCIvXCIpICE9PSBwYXRoLmpvaW4oXCIvXCIpO1xuICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1BBTkVMX1BBVEhcIiwgYW5jaG9yLCB2YWx1ZTogcGF0aCB9KTtcbiAgICAgICAgLy8g8J+Om++4jyBDb21tYW5kIHBhbGV0dGUgb25seTogc3dpdGNoaW5nIGNhdGVnb3J5IGxlYXZlcyBhbHdheXMgY29sbGFwc2VzIGFueSBleHBhbmRlZCBhcmcgZm9ybSDigJQgdGhlXG4gICAgICAgIC8vIG5leHQgaGllcmFyY2h5IGxldmVsIHVwIG9ubHkgbWFrZXMgc2Vuc2UgdW5kZXIgaXRzIG93biBjYXRlZ29yeSdzIGNvbW1hbmQgbGlzdCAobWlycm9ycyB0aGUgb2xkXG4gICAgICAgIC8vIGRlZGljYXRlZCBgU0VUX0NPTU1BTkRfQ0FURUdPUllgIHJlZHVjZXIgY2FzZSwgbm93IGV4cHJlc3NlZCBhdCB0aGUgZ2VuZXJpYyBwYXRoLWNoYW5nZSBjYWxsIHNpdGVcbiAgICAgICAgLy8gc2luY2UgY2F0ZWdvcnktYWN0aXZlIHN0YXRlIGl0c2VsZiBpcyBqdXN0IHRoaXMgYW5jaG9yJ3MgYGFjdGl2ZVRhYlBhdGhgKS4gQ2F0ZWdvcmllcyBzaXQgdW5kZXJcbiAgICAgICAgLy8gdGhlIENvbW1hbmQgYnJhbmNoLCBzbyBjb21wYXJlIHRoZSBjYXRlZ29yeSBzZWdtZW50IChwYXRoWzFdKSwgbm90IHRoZSBzaGFyZWQgYnJhbmNoIHJvb3QuXG4gICAgICAgIGlmIChhbmNob3IgPT09IFwiYm90dG9tLW1pZGRsZVwiICYmIHBhbmVsc1thbmNob3JdLnBhdGhbMV0gIT09IHBhdGhbMV0pIHtcbiAgICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0NPTU1BTkRfRVhQQU5ERURcIiwgdmFsdWU6IG51bGwgfSk7XG4gICAgICAgIH1cbiAgICAgICAgY29uc3QgdGFiSWQgPSBwYXRoW3BhdGgubGVuZ3RoIC0gMV07XG4gICAgICAgIC8vIPCfm6DvuI8gU2VsZWN0aW5nIGEgbW9kZS10b29sIGxlYWYgKGB0b29sLjxpZD5gKSBhY3RpdmF0ZXMgdGhhdCB0b29sIHNvIGl0cyBtZWFzdXJlcyByZW5kZXIgaW1tZWRpYXRlbHlcbiAgICAgICAgLy8gdW5kZXIgdGhlIHRhYiDigJQgbm8gbmVzdGVkIEZpbGwgdG9nZ2xlIGluc2lkZSB0aGUgdHJlZS5cbiAgICAgICAgaWYgKGFuY2hvciA9PT0gXCJib3R0b20tbWlkZGxlXCIgJiYgc2Vzc2lvbiAmJiBmaW5kUGFuZWxUYWJOb2RlKGRvY2suYW5jaG9yc1thbmNob3JdLCBwYXRoKT8ua2luZCA9PT0gXCJsZWFmXCIpIHtcbiAgICAgICAgICBjb25zdCBzZWxlY3RlZFRvb2xJZCA9IHRvb2xJZEZyb21QYW5lbFRhYklkKHRhYklkKTtcbiAgICAgICAgICBpZiAoc2VsZWN0ZWRUb29sSWQgJiYgc2VsZWN0ZWRUb29sSWQgIT09IGFjdGl2ZVRvb2xJZFJlZi5jdXJyZW50KSB7XG4gICAgICAgICAgICBvbkFjdGlvbih7IGNvbnRyb2xsZXJJZDogc2Vzc2lvbi5hcHAuY29udHJvbGxlcklkLCBhY3Rpb246IFNFVF9BQ1RJVkVfVE9PTF9BQ1RJT05fSUQsIGFyZ3M6IHsgdG9vbElkOiBzZWxlY3RlZFRvb2xJZCB9IH0pO1xuICAgICAgICAgIH1cbiAgICAgICAgfVxuICAgICAgICAvLyDwn4yx77iPIFByb2dyZXNzaXZlIHBhdGhzIG9mdGVuIGVuZCBhdCBhIGJyYW5jaCAob3IgYXJlIGVtcHR5KSDigJQgb25seSBsZWF2ZXMgYXJlIG1lYW5pbmdmdWwgXCJhY3RpdmUgcGFuZWwgdGFiXCIgc2VsZWN0aW9ucy5cbiAgICAgICAgaWYgKHRhYklkICYmIHN0dWRpb01vZGUgJiYgc2Vzc2lvbj8uYXBwLmlkID09PSBob3N0QXBwSWQgJiYgZmluZFBhbmVsVGFiTm9kZShkb2NrLmFuY2hvcnNbYW5jaG9yXSwgcGF0aCk/LmtpbmQgPT09IFwibGVhZlwiKSB7XG4gICAgICAgICAgb25BY3Rpb24oeyBjb250cm9sbGVySWQ6IHNlc3Npb24uYXBwLmNvbnRyb2xsZXJJZCwgYWN0aW9uOiBcInNldEFjdGl2ZVBhbmVsVGFiXCIsIGFyZ3M6IHsgdGFiSWQgfSB9KTtcbiAgICAgICAgfVxuICAgICAgICBpZiAocGF0aENoYW5nZWQgJiYgdGFiSWQpIG5vdGVTaGVsbENvbW1hbmQoXCJzaGVsbC5wYW5lbFRhYlwiLCBzaGVsbExhYmVsKFwidWkuc2hlbGxDb21tYW5kLnBhbmVsVGFiXCIpLCB7IGFuY2hvciwgdGFiSWQgfSk7XG4gICAgICB9LFxuICAgICAgcGF0aE1lbW9yeTogcGFuZWxQYXRoTWVtb3J5LFxuICAgICAgb25QYXRoTWVtb3J5Q2hhbmdlOiAodmFsdWU6IFJlYWRvbmx5PFJlY29yZDxzdHJpbmcsIHN0cmluZz4+KSA9PiBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1BBTkVMX1BBVEhfTUVNT1JZXCIsIHZhbHVlIH0pLFxuICAgIH0pLFxuICAgIFtkb2NrLCBvbkFjdGlvbiwgcGFuZWxBY3RpdmVQYXRocywgcGFuZWxQYXRoTWVtb3J5LCBwYW5lbHMsIHNlc3Npb24sIHN0dWRpb01vZGUsIGhvc3RBcHBJZCwgbm90ZVNoZWxsQ29tbWFuZF0sXG4gICk7XG4gIC8vI2VuZHJlZ2lvbiDwn46b77iPUGFuZWxUYWJCYXJIb3N0aW5nXG5cbiAgY29uc3QgbmF2YmFySXRlbXMgPSB1c2VNZW1vKCgpOiBOYXZiYXJJdGVtW10gPT4ge1xuICAgIGlmICghc2Vzc2lvbikgcmV0dXJuIFtdO1xuICAgIGNvbnN0IGxvZ29BbmRUaXRsZSA9IChcbiAgICAgIDxkaXYga2V5PVwibG9nb0FuZFRpdGxlXCIgY2xhc3NOYW1lPVwiZmxleCBtaW4tdy0wIHNocmluay0wIGl0ZW1zLWNlbnRlciBnYXAtc2luZ2xlXCI+XG4gICAgICAgIHticmFuZD8ubG9nb1N2ZyA/IDxTaGVsbEJyYW5kTG9nbyBzdmc9e2JyYW5kLmxvZ29Tdmd9IGNsYXNzTmFtZT1cInNpemUtd29ya2JlbmNoIHNocmluay0wXCIgLz4gOiA8U2VtaW9Mb2dvIGNsYXNzTmFtZT1cInNpemUtd29ya2JlbmNoIHNocmluay0wXCIgLz59XG4gICAgICAgIDxzcGFuIGRhdGEtc2xvdD1cImFwcC1uYW1lXCIgY2xhc3NOYW1lPXtjbihcInB4LXNpbmdsZVwiLCBzaGVsbENocm9tZVRpdGxlQ2xhc3NOYW1lKX0+XG4gICAgICAgICAge2FwcERvY3VtZW50TGFiZWwocmVzb2x2ZUFwcERvY3VtZW50KHNlc3Npb24uYXBwLCB1aVRlcm1pbm9sb2d5KSl9XG4gICAgICAgIDwvc3Bhbj5cbiAgICAgIDwvZGl2PlxuICAgICk7XG4gICAgY29uc3Qgc2hvd0V4YW1wbGVTZWxlY3QgPSBleGFtcGxlT3B0aW9ucy5sZW5ndGggPiAwICYmICFsb2Nrcy5leGFtcGxlSWQgJiYgKCFzdHVkaW9Nb2RlIHx8IHNlc3Npb24uYXBwLmlkICE9PSBsYW5kaW5nQXBwSWQpO1xuICAgIC8vIPCfk7HvuI8gTW9iaWxlIGhhcyBubyByb29tIGZvciB0YWIgYmFycywgZXhhbXBsZSBzZWxlY3Rvciwgb3IgbW9kZSBzd2l0Y2hlciBpbiB0aGUgbmF2YmFyIOKAlCBqdXN0IHRoZVxuICAgIC8vIGxvZ28vdGl0bGUgYW5kIHRoZSBzaW5nbGUgdG9nZ2xlIGZvciB0aGUgbWVyZ2VkIG1vYmlsZSBwYW5lbCAodGhlIHR3byBkcm9wcGVkIGNvbnRyb2xzIHJlc3VyZmFjZSBhc1xuICAgIC8vIHRoZSBwYW5lbCdzIHN5bnRoZXRpYyBcIkFwcFwiIHRhYiwgc2VlIGBtb2JpbGVQYW5lbFRhYnNgKS5cbiAgICBpZiAobW9iaWxlKSB7XG4gICAgICByZXR1cm4gW1xuICAgICAgICB7IGtleTogXCJsb2dvQW5kVGl0bGVcIiwgY29udGVudDogbG9nb0FuZFRpdGxlIH0sXG4gICAgICAgIG5hdmJhckZpbGxJdGVtKFwibmF2YmFyVHJhaWxpbmdGaWxsXCIpLFxuICAgICAgICB7XG4gICAgICAgICAga2V5OiBcIm1vYmlsZVBhbmVsVG9nZ2xlXCIsXG4gICAgICAgICAgY29udGVudDogPFRvZ2dsZSBpZD1cInVpLm1vYmlsZVBhbmVsLnRvZ2dsZVwiIHByZXNzZWQ9e21vYmlsZVBhbmVsVmlzaWJsZX0gb25QcmVzc2VkQ2hhbmdlPXsodmFsdWUpID0+IGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfTU9CSUxFX1BBTkVMX1ZJU0lCTEVcIiwgdmFsdWUgfSl9IGljb249XCJwYW5lbC1sZWZ0XCIgLz4sXG4gICAgICAgIH0sXG4gICAgICBdO1xuICAgIH1cbiAgICAvLyBMb2dvL3RpdGxlLCBleGFtcGxlIHNlbGVjdG9yLCBhbmQgbW9kZSBzd2l0Y2hlciByZW5kZXIgYXMgb25lIGNsdXN0ZXIsIGNlbnRlcmVkIGFzIGEgZ3JvdXAgaW4gdGhlIG5hdmJhclxuICAgIC8vICh2aWEgYGNlbnRlcmVkYCkgcmF0aGVyIHRoYW4gbGVmdC1hbmNob3JlZCB3aXRoIGZpbGwgc3BhY2VycyBwdXNoaW5nIHRoZSByZXN0IHRvd2FyZCB0aGUgdHJhaWxpbmcgZWRnZS5cbiAgICBjb25zdCBjZW50ZXJDb250ZW50OiBSZWFjdE5vZGVbXSA9IFtsb2dvQW5kVGl0bGVdO1xuICAgIGlmIChzaG93RXhhbXBsZVNlbGVjdCAmJiBleGFtcGxlU2VsZWN0RWxlbWVudCkgY2VudGVyQ29udGVudC5wdXNoKGV4YW1wbGVTZWxlY3RFbGVtZW50KTtcbiAgICBpZiAobW9kZVN3aXRjaGVyRWxlbWVudCkgY2VudGVyQ29udGVudC5wdXNoKG1vZGVTd2l0Y2hlckVsZW1lbnQpO1xuICAgIHJldHVybiBbXG4gICAgICB7IGtleTogXCJ0b3BMZWZ0UGFuZWxUYWJzXCIsIGNvbnRlbnQ6IDxQYW5lbENocm9tZVRhYkJhciBhbmNob3I9XCJ0b3AtbGVmdFwiIHsuLi5idWlsZFBhbmVsU2VsZWN0aW9uUHJvcHMoXCJ0b3AtbGVmdFwiKX0gLz4gfSxcbiAgICAgIG5hdmJhckZpbGxJdGVtKFwibmF2YmFyVHJhaWxpbmdGaWxsXCIpLFxuICAgICAgeyBrZXk6IFwidG9wUmlnaHRQYW5lbFRhYnNcIiwgY29udGVudDogPFBhbmVsQ2hyb21lVGFiQmFyIGFuY2hvcj1cInRvcC1yaWdodFwiIHsuLi5idWlsZFBhbmVsU2VsZWN0aW9uUHJvcHMoXCJ0b3AtcmlnaHRcIil9IC8+IH0sXG4gICAgICB7XG4gICAgICAgIGtleTogXCJjZW50ZXJcIixcbiAgICAgICAgY2VudGVyZWQ6IHRydWUsXG4gICAgICAgIGNvbnRlbnQ6IChcbiAgICAgICAgICA8ZGl2IGNsYXNzTmFtZT1cImZsZXggbWluLXctMCBpdGVtcy1jZW50ZXIgZ2FwLWRvdWJsZVwiPlxuICAgICAgICAgICAge2NlbnRlckNvbnRlbnR9XG4gICAgICAgICAgICA8UGFuZWxDaHJvbWVUYWJCYXIgYW5jaG9yPVwidG9wLW1pZGRsZVwiIHsuLi5idWlsZFBhbmVsU2VsZWN0aW9uUHJvcHMoXCJ0b3AtbWlkZGxlXCIpfSAvPlxuICAgICAgICAgIDwvZGl2PlxuICAgICAgICApLFxuICAgICAgfSxcbiAgICBdO1xuICB9LCBbYnJhbmQsIGJ1aWxkUGFuZWxTZWxlY3Rpb25Qcm9wcywgZXhhbXBsZU9wdGlvbnMsIGV4YW1wbGVTZWxlY3RFbGVtZW50LCBsb2Nrcy5leGFtcGxlSWQsIG1vYmlsZSwgbW9iaWxlUGFuZWxWaXNpYmxlLCBtb2RlU3dpdGNoZXJFbGVtZW50LCBzZXNzaW9uLCB1aVRlcm1pbm9sb2d5LCBzdHVkaW9Nb2RlLCBsYW5kaW5nQXBwSWRdKTtcblxuICBjb25zdCBzZWFyY2hJdGVtcyA9IHVzZU1lbW8oKCkgPT4ge1xuICAgIGlmICghc2Vzc2lvbikgcmV0dXJuIFtdO1xuICAgIGNvbnN0IGl0ZW1zOiBVSVNlYXJjaEl0ZW1bXSA9IFtdO1xuICAgIGZvciAoY29uc3QgdGFiIG9mIGZsYXR0ZW5QYW5lbFRhYkxlYXZlcyhzZXNzaW9uLmFwcC5wYW5lbFRhYnMpKSB7XG4gICAgICBjb25zdCB0YWJJZCA9IHBhbmVsVGFiS2luZElkKHRhYi5raW5kKTtcbiAgICAgIGl0ZW1zLnB1c2goe1xuICAgICAgICBpZDogYHBhbmVsLiR7dGFiSWR9YCxcbiAgICAgICAgbGFiZWw6IHJlc29sdmVQYW5lbFRhYkxhYmVsKGFwcExhYmVsc092ZXJsYXksIHRhYklkLCByZXNvbHZlTWFuaWZlc3RMYWJlbCh0YWIubGFiZWwsIHVpVGVybWlub2xvZ3ksIHVpTG9jYWxlKSksXG4gICAgICAgIGNhdGVnb3J5OiBzaGVsbExhYmVsKFwidWkuc2VhcmNoLmNhdGVnb3J5LnBhbmVsc1wiKSxcbiAgICAgICAgaWNvbjogPEljb24gaWNvbj1cInBhbmVsLWxlZnRcIiBzaXplPVwic21hbGxcIiAvPixcbiAgICAgICAgb25TZWxlY3Q6ICgpID0+IG9uQWN0aW9uKHsgY29udHJvbGxlcklkOiBzZXNzaW9uLmFwcC5jb250cm9sbGVySWQsIGFjdGlvbjogXCJzZXRBY3RpdmVQYW5lbFRhYlwiLCBhcmdzOiB7IHRhYklkIH0gfSksXG4gICAgICB9KTtcbiAgICB9XG4gICAgZm9yIChjb25zdCBraW5kIG9mIHNlc3Npb24uYXBwLndpbmRvd0tpbmRzKSB7XG4gICAgICBpdGVtcy5wdXNoKHtcbiAgICAgICAgaWQ6IGB3aW5kb3cuJHtraW5kLmlkfWAsXG4gICAgICAgIGxhYmVsOiByZXNvbHZlQXBwTGFiZWwoYXBwTGFiZWxzT3ZlcmxheSwgXCJ3aW5kb3dLaW5kXCIsIGtpbmQuaWQsIHJlc29sdmVNYW5pZmVzdExhYmVsKGtpbmQubGFiZWwsIHVpVGVybWlub2xvZ3ksIHVpTG9jYWxlKSksXG4gICAgICAgIGNhdGVnb3J5OiBzaGVsbExhYmVsKFwidWkuc2VhcmNoLmNhdGVnb3J5LndpbmRvd3NcIiksXG4gICAgICAgIGljb246IDxJY29uIGljb249XCJhcHAtd2luZG93XCIgc2l6ZT1cInNtYWxsXCIgLz4sXG4gICAgICAgIG9uU2VsZWN0OiAoKSA9PiBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0FDVElWRV9XSU5ET1dfSURcIiwgdmFsdWU6IGtpbmQuaWQgfSksXG4gICAgICB9KTtcbiAgICB9XG4gICAgY29uc3Qga2V5c0J5QWN0aW9uSWQgPSBuZXcgTWFwKHNlc3Npb24uYXBwLmtleWJpbmRpbmdzLm1hcCgoYmluZGluZykgPT4gW2JpbmRpbmcuYWN0aW9uLmFjdGlvbiwgYmluZGluZy5rZXlzXSkpO1xuICAgIGNvbnN0IGRlY2xhcmVkQWN0aW9uSWRzID0gbmV3IFNldDxzdHJpbmc+KCk7XG4gICAgLy8g8J+Th++4jyBGaXJzdCB3aW5kb3cga2luZCB3aG9zZSByZXNvbHZlZCBhY3Rpb25zIGluY2x1ZGUgdGhpcyBpZCAob3JwaGFuL2dsb2JhbCBhY3Rpb25zIGZhbGwgdGhyb3VnaCB0b1xuICAgIC8vIHRoZSBhY3RpdmUgd2luZG93LCB0aGVuIHRoZSBmaXJzdCB3aW5kb3cpIOKAlCB0aGUgcmVkaXJlY3QgdGFyZ2V0IGZvciBhcmctY2FycnlpbmcgcGFsZXR0ZSBlbnRyaWVzLlxuICAgIGNvbnN0IGhvc3RXaW5kb3dGb3JBY3Rpb24gPSAoYWN0aW9uSWQ6IHN0cmluZyk6IHN0cmluZyB8IHVuZGVmaW5lZCA9PiB7XG4gICAgICBmb3IgKGNvbnN0IGtpbmQgb2Ygc2Vzc2lvbi5hcHAud2luZG93S2luZHMpIHtcbiAgICAgICAgaWYgKHJlc29sdmVXaW5kb3dBY3Rpb25zKHNlc3Npb24uYXBwLCBraW5kKS5zb21lKChlbnRyeSkgPT4gZW50cnkuaWQgPT09IGFjdGlvbklkKSkgcmV0dXJuIGtpbmQuaWQ7XG4gICAgICB9XG4gICAgICByZXR1cm4gYWN0aXZlV2luZG93SWQgPz8gc2Vzc2lvbi5hcHAud2luZG93S2luZHNbMF0/LmlkO1xuICAgIH07XG4gICAgZm9yIChjb25zdCBhY3Rpb24gb2Ygc2Vzc2lvbi5hcHAuYWN0aW9ucyA/PyBbXSkge1xuICAgICAgaWYgKCFhY3Rpb24uaW5QYWxldHRlKSBjb250aW51ZTtcbiAgICAgIGRlY2xhcmVkQWN0aW9uSWRzLmFkZChhY3Rpb24uaWQpO1xuICAgICAgY29uc3QgYXJnQ2FycnlpbmcgPSBhY3Rpb25SZXF1aXJlc1N0YWdlZEZvcm0oYWN0aW9uKTtcbiAgICAgIGNvbnN0IHJlc29sdmVkQWN0aW9uTGFiZWwgPSByZXNvbHZlQXBwTGFiZWwoYXBwTGFiZWxzT3ZlcmxheSwgXCJhY3Rpb25cIiwgYWN0aW9uLmlkLCByZXNvbHZlTWFuaWZlc3RMYWJlbChhY3Rpb24ubGFiZWwsIHVpVGVybWlub2xvZ3ksIHVpTG9jYWxlKSk7XG4gICAgICBpdGVtcy5wdXNoKHtcbiAgICAgICAgaWQ6IGBhY3Rpb24uJHthY3Rpb24uaWR9YCxcbiAgICAgICAgLy8g4pyN77iPIEFyZy1jYXJyeWluZyBhY3Rpb25zIG5ldmVyIGZpcmUgZnJvbSB0aGUgcGFsZXR0ZSAoUDMpOiB0aGUgXCLigKZcIiBlbnRyeSBhY3RpdmF0ZXMgdGhlIGhvc3RpbmdcbiAgICAgICAgLy8gd2luZG93LCB1bmZvbGRzIGl0cyB0b3AtbGVmdCBBY3Rpb25zIHBhbmUsIGFuZCBleHBhbmRzIHRoaXMgYWN0aW9uJ3Mgc3RhZ2VkIGZvcm0gaW5zdGVhZCBvZiBkaXNwYXRjaGluZy5cbiAgICAgICAgbGFiZWw6IGFyZ0NhcnJ5aW5nID8gYCR7cmVzb2x2ZWRBY3Rpb25MYWJlbH3igKZgIDogcmVzb2x2ZWRBY3Rpb25MYWJlbCxcbiAgICAgICAgZGVzY3JpcHRpb246IGFjdGlvbi5rZXlzID8/IGtleXNCeUFjdGlvbklkLmdldChhY3Rpb24uaWQpLFxuICAgICAgICBjYXRlZ29yeTogYWN0aW9uLmNhdGVnb3J5ID8/IChhY3Rpb24ua2luZCA9PT0gXCJoaXN0b3J5XCIgPyBzaGVsbExhYmVsKFwidWkucmliYm9uLnBhcmVudC5oaXN0b3J5XCIpIDogc2hlbGxMYWJlbChcInVpLnJpYmJvbi5wYXJlbnQuYWN0aW9uc1wiKSksXG4gICAgICAgIG9uU2VsZWN0OiAoKSA9PiB7XG4gICAgICAgICAgaWYgKGFyZ0NhcnJ5aW5nKSB7XG4gICAgICAgICAgICBjb25zdCB3aW5kb3dJZCA9IGhvc3RXaW5kb3dGb3JBY3Rpb24oYWN0aW9uLmlkKTtcbiAgICAgICAgICAgIGlmICh3aW5kb3dJZCkge1xuICAgICAgICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0FDVElWRV9XSU5ET1dfSURcIiwgdmFsdWU6IHdpbmRvd0lkIH0pO1xuICAgICAgICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0FDVElPTl9QQU5FX0ZPTERFRFwiLCB3aW5kb3dJZCwgdmFsdWU6IGZhbHNlIH0pO1xuICAgICAgICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0FDVElPTl9QQU5FX0VYUEFOREVEXCIsIHdpbmRvd0lkLCB2YWx1ZTogYWN0aW9uLmlkIH0pO1xuICAgICAgICAgICAgfVxuICAgICAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9TRUFSQ0hfT1BFTlwiLCB2YWx1ZTogZmFsc2UgfSk7XG4gICAgICAgICAgICByZXR1cm47XG4gICAgICAgICAgfVxuICAgICAgICAgIG9uQWN0aW9uKHsgY29udHJvbGxlcklkOiBzZXNzaW9uLmFwcC5jb250cm9sbGVySWQsIGFjdGlvbjogYWN0aW9uLmlkIH0pO1xuICAgICAgICB9LFxuICAgICAgfSk7XG4gICAgfVxuICAgIGZvciAoY29uc3QgYmluZGluZyBvZiBzZXNzaW9uLmFwcC5rZXliaW5kaW5ncykge1xuICAgICAgaWYgKGRlY2xhcmVkQWN0aW9uSWRzLmhhcyhiaW5kaW5nLmFjdGlvbi5hY3Rpb24pKSBjb250aW51ZTtcbiAgICAgIGl0ZW1zLnB1c2goe1xuICAgICAgICBpZDogYGtleWJpbmRpbmcuJHtiaW5kaW5nLmtleXN9YCxcbiAgICAgICAgbGFiZWw6IGJpbmRpbmcuYWN0aW9uLmFjdGlvbixcbiAgICAgICAgZGVzY3JpcHRpb246IGJpbmRpbmcua2V5cyxcbiAgICAgICAgY2F0ZWdvcnk6IHNoZWxsTGFiZWwoXCJ1aS5yaWJib24ucGFyZW50LmFjdGlvbnNcIiksXG4gICAgICAgIG9uU2VsZWN0OiAoKSA9PiBvbkFjdGlvbihiaW5kaW5nLmFjdGlvbiksXG4gICAgICB9KTtcbiAgICB9XG4gICAgLy8g8J+Om++4jyBDb21tYW5kcyAob3MvcGx1Z2luL2FwcC9tb2RlKSDigJQgdGhlIGZvb3RlciB0d2luIG9mIHRoZSB3aW5kb3ctcmFpbCBQMyByZWRpcmVjdCBhYm92ZTogYW5cbiAgICAvLyBhcmctY2FycnlpbmcgY29tbWFuZCBuZXZlciBmaXJlcyBmcm9tIHRoZSBwYWxldHRlLCBpdCBvcGVucyB0aGUgYm90dG9tLW1pZGRsZSBjb21tYW5kIHBhbmVsIGF0IGl0c1xuICAgIC8vIGNhdGVnb3J5IGFuZCBleHBhbmRzIGl0cyBmb3JtIGluc3RlYWQuXG4gICAgZm9yIChjb25zdCB7IGRlZmluaXRpb24sIHNvdXJjZSB9IG9mIHJlc29sdmVkQ29tbWFuZHMpIHtcbiAgICAgIGlmICghZGVmaW5pdGlvbi5pblBhbGV0dGUpIGNvbnRpbnVlO1xuICAgICAgY29uc3QgYXJnQ2FycnlpbmcgPSAoZGVmaW5pdGlvbi5hcmdzPy5sZW5ndGggPz8gMCkgPiAwO1xuICAgICAgaXRlbXMucHVzaCh7XG4gICAgICAgIGlkOiBgY29tbWFuZC4ke2RlZmluaXRpb24uaWR9YCxcbiAgICAgICAgbGFiZWw6IGFyZ0NhcnJ5aW5nID8gYCR7ZGVmaW5pdGlvbi5sYWJlbH3igKZgIDogZGVmaW5pdGlvbi5sYWJlbCxcbiAgICAgICAgZGVzY3JpcHRpb246IGRlZmluaXRpb24ua2V5cyxcbiAgICAgICAgY2F0ZWdvcnk6IGNvbW1hbmRDYXRlZ29yeUxhYmVsKGRlZmluaXRpb24uY2F0ZWdvcnkpLFxuICAgICAgICBvblNlbGVjdDogKCkgPT4ge1xuICAgICAgICAgIGlmIChhcmdDYXJyeWluZykge1xuICAgICAgICAgICAgY29uc3QgY29tbWFuZFBhdGggPSBbRlJBTUVXT1JLX0NBVEVHT1JZX0NPTU1BTkRfSUQsIGBjb21tYW5kLmNhdGVnb3J5LiR7ZGVmaW5pdGlvbi5jYXRlZ29yeX1gXTtcbiAgICAgICAgICAgIC8vIPCfk7HvuI8gT24gbW9iaWxlIGV2ZXJ5IGFuY2hvcidzIHRhYnMgYXJlIG1lcmdlZCBpbnRvIHRoZSBzaW5nbGUgbW9iaWxlIHBhbmVsIOKAlCByb3V0ZSB0aGUgc2FtZVxuICAgICAgICAgICAgLy8gcGF0aCB0aGVyZSBpbnN0ZWFkIG9mIHRoZSAodW5yZW5kZXJlZCkgYm90dG9tLW1pZGRsZSBhbmNob3IsIGFuZCBvcGVuIHRoZSBtb2JpbGUgcGFuZWwgaXRzZWxmLlxuICAgICAgICAgICAgaWYgKG1vYmlsZSkge1xuICAgICAgICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX01PQklMRV9QQU5FTF9WSVNJQkxFXCIsIHZhbHVlOiB0cnVlIH0pO1xuICAgICAgICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX01PQklMRV9QQU5FTF9QQVRIXCIsIHZhbHVlOiBjb21tYW5kUGF0aCB9KTtcbiAgICAgICAgICAgIH0gZWxzZSB7XG4gICAgICAgICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfUEFORUxfVklTSUJMRVwiLCBhbmNob3I6IFwiYm90dG9tLW1pZGRsZVwiLCB2YWx1ZTogdHJ1ZSB9KTtcbiAgICAgICAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9QQU5FTF9QQVRIXCIsIGFuY2hvcjogXCJib3R0b20tbWlkZGxlXCIsIHZhbHVlOiBjb21tYW5kUGF0aCB9KTtcbiAgICAgICAgICAgIH1cbiAgICAgICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfQ09NTUFORF9FWFBBTkRFRFwiLCB2YWx1ZTogZGVmaW5pdGlvbi5pZCB9KTtcbiAgICAgICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfU0VBUkNIX09QRU5cIiwgdmFsdWU6IGZhbHNlIH0pO1xuICAgICAgICAgICAgcmV0dXJuO1xuICAgICAgICAgIH1cbiAgICAgICAgICBvbkNvbW1hbmQoc291cmNlLCBkZWZpbml0aW9uLmlkKTtcbiAgICAgICAgfSxcbiAgICAgIH0pO1xuICAgIH1cbiAgICBpZiAoc3R1ZGlvTW9kZSAmJiBwYW5lbCkge1xuICAgICAgZm9yIChjb25zdCBwcm9ncmFtIG9mIHBhbmVsLnByb2dyYW1zKSB7XG4gICAgICAgIGl0ZW1zLnB1c2goe1xuICAgICAgICAgIGlkOiBgc3Bhd24uJHtwcm9ncmFtLnBsdWdpbklkfWAsXG4gICAgICAgICAgbGFiZWw6IGAke3NoZWxsTGFiZWwoXCJ1aS5wYWxldHRlLnNwYXduUHJlZml4XCIpfSAke2FwcERvY3VtZW50TGFiZWwocmVzb2x2ZURvY3VtZW50QnlBcHBJZChsb2FkZWRQbHVnaW5zLCBwcm9ncmFtLmFwcElkLCBwcm9ncmFtLmRvY3VtZW50LCB1aVRlcm1pbm9sb2d5KSl9YCxcbiAgICAgICAgICBjYXRlZ29yeTogc2hlbGxMYWJlbChcInVpLnNlYXJjaC5jYXRlZ29yeS5jYXRhbG9ndWVcIiksXG4gICAgICAgICAgb25TZWxlY3Q6ICgpID0+IG9uQWN0aW9uKHsgY29udHJvbGxlcklkOiBob3N0Q29udHJvbGxlcklkID8/IFwiXCIsIGFjdGlvbjogXCJzcGF3bkFwcFwiLCBhcmdzOiB7IHBsdWdpbklkOiBwcm9ncmFtLnBsdWdpbklkIH0gfSksXG4gICAgICAgIH0pO1xuICAgICAgfVxuICAgICAgaXRlbXMucHVzaChcbiAgICAgICAge1xuICAgICAgICAgIGlkOiBcInN0dWRpby51bmRvXCIsXG4gICAgICAgICAgbGFiZWw6IHNoZWxsTGFiZWwoXCJ1aS5wYWxldHRlLnVuZG9cIiksXG4gICAgICAgICAgY2F0ZWdvcnk6IHNoZWxsTGFiZWwoXCJ1aS5zZWFyY2guY2F0ZWdvcnkuc3R1ZGlvXCIpLFxuICAgICAgICAgIGljb246IDxJY29uIGljb249XCJ1bmRvLTJcIiBzaXplPVwic21hbGxcIiAvPixcbiAgICAgICAgICBvblNlbGVjdDogKCkgPT4gb25BY3Rpb24oeyBjb250cm9sbGVySWQ6IGhvc3RDb250cm9sbGVySWQgPz8gXCJcIiwgYWN0aW9uOiBcInVuZG9cIiB9KSxcbiAgICAgICAgfSxcbiAgICAgICAge1xuICAgICAgICAgIGlkOiBcInN0dWRpby5yZWRvXCIsXG4gICAgICAgICAgbGFiZWw6IHNoZWxsTGFiZWwoXCJ1aS5wYWxldHRlLnJlZG9cIiksXG4gICAgICAgICAgY2F0ZWdvcnk6IHNoZWxsTGFiZWwoXCJ1aS5zZWFyY2guY2F0ZWdvcnkuc3R1ZGlvXCIpLFxuICAgICAgICAgIGljb246IDxJY29uIGljb249XCJyZWRvLTJcIiBzaXplPVwic21hbGxcIiAvPixcbiAgICAgICAgICBvblNlbGVjdDogKCkgPT4gb25BY3Rpb24oeyBjb250cm9sbGVySWQ6IGhvc3RDb250cm9sbGVySWQgPz8gXCJcIiwgYWN0aW9uOiBcInJlZG9cIiB9KSxcbiAgICAgICAgfSxcbiAgICAgICAge1xuICAgICAgICAgIGlkOiBcInN0dWRpby5ob21lXCIsXG4gICAgICAgICAgbGFiZWw6IHNoZWxsTGFiZWwoXCJ1aS5wYWxldHRlLmdvSG9tZVwiKSxcbiAgICAgICAgICBjYXRlZ29yeTogc2hlbGxMYWJlbChcInVpLnNlYXJjaC5jYXRlZ29yeS5uYXZpZ2F0aW9uXCIpLFxuICAgICAgICAgIG9uU2VsZWN0OiAoKSA9PiBvbkFjdGlvbih7IGNvbnRyb2xsZXJJZDogaG9zdENvbnRyb2xsZXJJZCA/PyBcIlwiLCBhY3Rpb246IFwiZ29Ib21lXCIgfSksXG4gICAgICAgIH0sXG4gICAgICApO1xuICAgIH1cbiAgICByZXR1cm4gaXRlbXM7XG4gIH0sIFthY3RpdmVXaW5kb3dJZCwgYXBwTGFiZWxzT3ZlcmxheSwgbG9hZGVkUGx1Z2lucywgbW9iaWxlLCBvbkFjdGlvbiwgb25Db21tYW5kLCBwYW5lbCwgcmVzb2x2ZWRDb21tYW5kcywgc2Vzc2lvbiwgc3R1ZGlvTW9kZSwgdWlMb2NhbGUsIHVpVGVybWlub2xvZ3ksIGhvc3RDb250cm9sbGVySWRdKTtcblxuICBjb25zdCBtb2RlV2luZG93cyA9IHVzZU1lbW8oKCk6IE1vZGVXaW5kb3dEZXNjcmlwdG9yW10gPT4ge1xuICAgIGlmICghc2Vzc2lvbikgcmV0dXJuIFtdO1xuICAgIGNvbnN0IGFjdGlvblBhbmVTbGljZTogQWN0aW9uUGFuZVNsaWNlID0geyBleHBhbmRlZEJ5V2luZG93SWQ6IGFjdGlvblBhbmVFeHBhbmRlZEJ5V2luZG93SWQsIHN0YWdlZEFyZ3NCeUtleTogYWN0aW9uUGFuZVN0YWdlZEFyZ3NCeUtleSwgYWN0aXZlVXRpbGl0eUJ5V2luZG93SWQgfTtcbiAgICBjb25zdCBhY3Rpb25zRm9sZGVkRm9yID0gKHdpbmRvd0lkOiBzdHJpbmcsIHdpbmRvd0tpbmRJZDogc3RyaW5nID0gd2luZG93SWQpID0+XG4gICAgICBpbnRyb2R1Y3Rpb25UYXJnZXRzV2luZG93KHdpbmRvd0lkLCB3aW5kb3dLaW5kSWQsIG51bGwsIGludHJvZHVjdGlvbkFjdGlvbldpbmRvd1NlZ21lbnQpID8gZmFsc2UgOiAoYWN0aW9uUGFuZUZvbGRlZEJ5V2luZG93SWRbd2luZG93SWRdID8/IHRydWUpO1xuICAgIC8vIPCfjpPvuI8gYHVuZGVmaW5lZGAga2VlcHMgdGhlIFdpbmRvdydzIG93biBpbnRlcm5hbCBmb2xkIHN0YXRlIOKAlCBvbmx5IHdpbmRvd3Mgb2YgdGhlIGludHJvZHVjdGlvbidzXG4gICAgLy8gdGFyZ2V0IGtpbmQgKGluY2x1ZGluZyBldmVyeSBvcGVuIGluc3RhbmNlKSBhcmUgZm9yY2UtY29udHJvbGxlZCB0byBgZmFsc2VgIHdoaWxlIGl0cyB1dGlsaXR5IHN0ZXBcbiAgICAvLyBpcyBhY3RpdmUuXG4gICAgY29uc3QgdXRpbGl0eUJhckZvbGRlZEZvciA9ICh3aW5kb3dJZDogc3RyaW5nLCB3aW5kb3dLaW5kSWQ6IHN0cmluZyA9IHdpbmRvd0lkKTogYm9vbGVhbiB8IHVuZGVmaW5lZCA9PlxuICAgICAgaW50cm9kdWN0aW9uVGFyZ2V0c1dpbmRvdyh3aW5kb3dJZCwgd2luZG93S2luZElkLCBpbnRyb2R1Y3Rpb25VdGlsaXR5V2luZG93SWQpID8gZmFsc2UgOiB1bmRlZmluZWQ7XG4gICAgY29uc3QgbWVhc3VyZXNGb2xkZWRGb3IgPSAod2luZG93SWQ6IHN0cmluZywgd2luZG93S2luZElkOiBzdHJpbmcgPSB3aW5kb3dJZCk6IGJvb2xlYW4gfCB1bmRlZmluZWQgPT5cbiAgICAgIGludHJvZHVjdGlvblRhcmdldHNXaW5kb3cod2luZG93SWQsIHdpbmRvd0tpbmRJZCwgaW50cm9kdWN0aW9uTWVhc3VyZVdpbmRvd0lkKSA/IGZhbHNlIDogdW5kZWZpbmVkO1xuICAgIGNvbnN0IG9uQWN0aW9uc0ZvbGRlZEZvciA9ICh3aW5kb3dJZDogc3RyaW5nKSA9PiAoZm9sZGVkOiBib29sZWFuKSA9PiBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0FDVElPTl9QQU5FX0ZPTERFRFwiLCB3aW5kb3dJZCwgdmFsdWU6IGZvbGRlZCB9KTtcbiAgICAvLyDwn5ax77iPIFdpbmRvdy1ib2R5IGN1cnNvciBmb2xsb3dzIHRoZSBhY3RpdmUgdXRpbGl0eSdzIGRlY2xhcmVkIGBjdXJzb3JgIChQNSkuXG4gICAgY29uc3QgY3Vyc29yRm9yID0gKGFwcDogQXBwRGVmaW5pdGlvbiwgd2luZG93SWQ6IHN0cmluZyk6IENTU1Byb3BlcnRpZXMgfCB1bmRlZmluZWQgPT4ge1xuICAgICAgY29uc3QgdXRpbGl0eUlkID0gYWN0aXZlVXRpbGl0eUJ5V2luZG93SWRbd2luZG93SWRdO1xuICAgICAgY29uc3QgY3Vyc29yID0gdXRpbGl0eUlkID8gKGFwcC51dGlsaXRpZXMgPz8gW10pLmZpbmQoKHV0aWxpdHkpID0+IHV0aWxpdHkuaWQgPT09IHV0aWxpdHlJZCk/LmN1cnNvciA6IHVuZGVmaW5lZDtcbiAgICAgIHJldHVybiBjdXJzb3IgPyB7IGN1cnNvciB9IDogdW5kZWZpbmVkO1xuICAgIH07XG4gICAgaWYgKHN0dWRpb01vZGUgJiYgc3Bhd25lZFdpbmRvd1VpICYmIHBhbmVsPy5hY3RpdmVTcGF3bmVkSWQpIHtcbiAgICAgIGNvbnN0IHNwYXduZWQgPSBwYW5lbC5zcGF3bmVkQXBwcy5maW5kKChlbnRyeSkgPT4gZW50cnkuaWQgPT09IHBhbmVsLmFjdGl2ZVNwYXduZWRJZCk7XG4gICAgICBpZiAoc3Bhd25lZCkge1xuICAgICAgICBjb25zdCBzcGF3bmVkQXBwID0gbG9hZGVkUGx1Z2lucy5maW5kKChlbnRyeSkgPT4gZW50cnkuaGFuZGxlLnBsdWdpbklkID09PSBzcGF3bmVkLnBsdWdpbklkKT8ubWFuaWZlc3QuYXBwcy5maW5kKChjYW5kaWRhdGUpID0+IGNhbmRpZGF0ZS5pZCA9PT0gc3Bhd25lZC5hcHBJZCk7XG4gICAgICAgIGNvbnN0IHdpbmRvd0tpbmQgPSBzcGF3bmVkQXBwPy53aW5kb3dLaW5kc1swXTtcbiAgICAgICAgY29uc3QgY2hyb21lID0gd2luZG93S2luZCA/IHNwYXduZWRXaW5kb3dDaHJvbWVGb3JLaW5kKHdpbmRvd0tpbmQsIHNwYXduZWQuaWQsIHNwYXduZWRXaW5kb3dFbmdhZ2VtZW50cywgc3Bhd25lZFdpbmRvd01lYXN1cmVzLCBhY3RpdmVVdGlsaXR5QnlXaW5kb3dJZFtzcGF3bmVkLmlkXSwgb25BY3Rpb25TdGFibGUpIDogdW5kZWZpbmVkO1xuICAgICAgICBjb25zdCBzcGF3bmVkVXRpbGl0aWVzID0gc3Bhd25lZEFwcCAmJiB3aW5kb3dLaW5kID8gcmVzb2x2ZVV0aWxpdHlOb2RlcyhzcGF3bmVkQXBwLCB3aW5kb3dLaW5kLCBhY3RpdmVVdGlsaXR5QnlXaW5kb3dJZFtzcGF3bmVkLmlkXSwgc3Bhd25lZC5pZCwgYXBwTGFiZWxzT3ZlcmxheSwgdWlUZXJtaW5vbG9neSwgdWlMb2NhbGUpIDogW107XG4gICAgICAgIHJldHVybiBbXG4gICAgICAgICAge1xuICAgICAgICAgICAgaWQ6IHNwYXduZWQuaWQsXG4gICAgICAgICAgICB0aXRsZTogd2lyZUxhYmVsKGFwcERvY3VtZW50TGFiZWwoc3Bhd25lZEFwcCA/IHJlc29sdmVBcHBEb2N1bWVudChzcGF3bmVkQXBwLCB1aVRlcm1pbm9sb2d5KSA6IHNwYXduZWQuZG9jdW1lbnQpKSxcbiAgICAgICAgICAgIGZpbGw6IHRydWUsXG4gICAgICAgICAgICBzaG93Q29udHJvbHM6IHRydWUsXG4gICAgICAgICAgICBtZWFzdXJlczogY2hyb21lPy5tZWFzdXJlcyxcbiAgICAgICAgICAgIG1lYXN1cmVzRm9sZGVkOiBtZWFzdXJlc0ZvbGRlZEZvcihzcGF3bmVkLmlkLCB3aW5kb3dLaW5kPy5pZCA/PyBzcGF3bmVkLmlkKSxcbiAgICAgICAgICAgIGVuZ2FnZW1lbnQ6IGNocm9tZT8uZW5nYWdlbWVudCxcbiAgICAgICAgICAgIHNlYXJjaDogY2hyb21lPy5zZWFyY2gsXG4gICAgICAgICAgICB1dGlsaXR5QmFyOiBzcGF3bmVkQXBwICYmIHdpbmRvd0tpbmQgPyB1dGlsaXR5QmFyTm9kZShzcGF3bmVkVXRpbGl0aWVzLCBzcGF3bmVkLmlkLCBvbkFjdGlvblN0YWJsZSwgaW50cm9kdWN0aW9uVXRpbGl0eUlkLCBjaHJvbWU/LnV0aWxpdHlPcHRpb25zKSA6IHVuZGVmaW5lZCxcbiAgICAgICAgICAgIHV0aWxpdHlCYXJGb2xkZWQ6IHV0aWxpdHlCYXJGb2xkZWRGb3Ioc3Bhd25lZC5pZCwgd2luZG93S2luZD8uaWQgPz8gc3Bhd25lZC5pZCksXG4gICAgICAgICAgICBhY3Rpb25QYW5lOiBzcGF3bmVkQXBwICYmIHdpbmRvd0tpbmQgPyB3aW5kb3dBY3Rpb25QYW5lTm9kZShzcGF3bmVkQXBwLCB3aW5kb3dLaW5kLCBzcGF3bmVkLmlkLCBhY3Rpb25QYW5lU2xpY2UsIG9uQWN0aW9uU3RhYmxlLCBkaXNwYXRjaCwgYXBwTGFiZWxzT3ZlcmxheSwgdWlUZXJtaW5vbG9neSwgdWlMb2NhbGUpIDogdW5kZWZpbmVkLFxuICAgICAgICAgICAgYWN0aW9uc0ZvbGRlZDogYWN0aW9uc0ZvbGRlZEZvcihzcGF3bmVkLmlkLCB3aW5kb3dLaW5kPy5pZCA/PyBzcGF3bmVkLmlkKSxcbiAgICAgICAgICAgIG9uQWN0aW9uc0ZvbGRlZENoYW5nZTogb25BY3Rpb25zRm9sZGVkRm9yKHNwYXduZWQuaWQpLFxuICAgICAgICAgICAgY2hpbGRyZW46IChcbiAgICAgICAgICAgICAgPENocm9tZUF3YXJlV2luZG93U2Nyb2xsU3VyZmFjZSBjbGFzc05hbWU9XCJyZWxhdGl2ZSBmbGV4IGgtZnVsbCBtaW4taC0wIG1pbi13LTAgZmxleC0xIGZsZXgtY29sIG92ZXJmbG93LWhpZGRlblwiIHN0eWxlPXtzcGF3bmVkQXBwID8gY3Vyc29yRm9yKHNwYXduZWRBcHAsIHNwYXduZWQuaWQpIDogdW5kZWZpbmVkfT5cbiAgICAgICAgICAgICAgICA8U2hlbGxGYXVsdEJvdW5kYXJ5IGJvdW5kYXJ5SWQ9e2B3aW5kb3ctJHtzcGF3bmVkLmlkfWB9IGZhbGxiYWNrTGFiZWw9e3NoZWxsTGFiZWwoXCJ1aS5jb21tb24ucmVuZGVyRXJyb3JcIil9PlxuICAgICAgICAgICAgICAgICAgPEludGVycHJldGVkVWlOb2RlIG5vZGU9e3NwYXduZWRXaW5kb3dVaX0gb25BY3Rpb249e29uQWN0aW9uU3RhYmxlfSAvPlxuICAgICAgICAgICAgICAgIDwvU2hlbGxGYXVsdEJvdW5kYXJ5PlxuICAgICAgICAgICAgICA8L0Nocm9tZUF3YXJlV2luZG93U2Nyb2xsU3VyZmFjZT5cbiAgICAgICAgICAgICksXG4gICAgICAgICAgfSxcbiAgICAgICAgXTtcbiAgICAgIH1cbiAgICB9XG4gICAgaWYgKE9iamVjdC5rZXlzKHdpbmRvd1VpQnlXaW5kb3dJZCkubGVuZ3RoID09PSAwKSByZXR1cm4gW107XG4gICAgY29uc3QgYmFzZVdpbmRvd3MgPSBzZXNzaW9uLmFwcC53aW5kb3dLaW5kcy5tYXAoKGtpbmQpID0+IHtcbiAgICAgIGNvbnN0IHV0aWxpdGllcyA9IHJlc29sdmVVdGlsaXR5Tm9kZXMoc2Vzc2lvbi5hcHAsIGtpbmQsIGFjdGl2ZVV0aWxpdHlCeVdpbmRvd0lkW2tpbmQuaWRdLCBraW5kLmlkLCBhcHBMYWJlbHNPdmVybGF5LCB1aVRlcm1pbm9sb2d5LCB1aUxvY2FsZSk7XG4gICAgICBjb25zdCBjaHJvbWUgPSB3aW5kb3dNZWFzdXJlc0Nocm9tZSh3aW5kb3dNZWFzdXJlc0J5V2luZG93SWRba2luZC5pZF0gPz8ga2luZC5vcHRpb25zLm1lYXN1cmVzLCBhY3RpdmVVdGlsaXR5QnlXaW5kb3dJZFtraW5kLmlkXSwga2luZC5pZCwgb25BY3Rpb25TdGFibGUpO1xuICAgICAgY29uc3QgcmVzb2x2ZWRFbmdhZ2VtZW50ID0gcmVzb2x2ZVdpbmRvd0VuZ2FnZW1lbnQoa2luZCwga2luZC5pZCwgd2luZG93RW5nYWdlbWVudHNCeVdpbmRvd0lkKTtcbiAgICAgIHJldHVybiB7XG4gICAgICAgIGlkOiBraW5kLmlkLFxuICAgICAgICBpY29uSWQ6IHdpbmRvd0ljb25zQnlJZFtraW5kLmlkXSA/PyBraW5kLmljb25JZCxcbiAgICAgICAgdGl0bGU6IHdpbmRvd1RpdGxlc0J5SWRba2luZC5pZF0gPz8gYXBwV2luZG93RG9jdW1lbnRMYWJlbChzZXNzaW9uLmFwcCwgdWlUZXJtaW5vbG9neSwgcmVzb2x2ZUFwcExhYmVsKGFwcExhYmVsc092ZXJsYXksIFwid2luZG93S2luZFwiLCBraW5kLmlkLCByZXNvbHZlTWFuaWZlc3RMYWJlbChraW5kLmxhYmVsLCB1aVRlcm1pbm9sb2d5LCB1aUxvY2FsZSkpLCB1aUxvY2FsZSksXG4gICAgICAgIGZpbGw6IHRydWUsXG4gICAgICAgIHNob3dDb250cm9sczogdHJ1ZSxcbiAgICAgICAgbWVhc3VyZXM6IGNocm9tZS5tZWFzdXJlcyxcbiAgICAgICAgbWVhc3VyZXNGb2xkZWQ6IG1lYXN1cmVzRm9sZGVkRm9yKGtpbmQuaWQsIGtpbmQuaWQpLFxuICAgICAgICBlbmdhZ2VtZW50OiB3aW5kb3dFbmdhZ2VtZW50VG9TcGVjKHJlc29sdmVkRW5nYWdlbWVudCwgb25BY3Rpb25TdGFibGUpLFxuICAgICAgICBzZWFyY2g6IHdpbmRvd0VuZ2FnZW1lbnRUb1NlYXJjaFNwZWMocmVzb2x2ZWRFbmdhZ2VtZW50LCBvbkFjdGlvblN0YWJsZSksXG4gICAgICAgIHV0aWxpdHlCYXI6IHV0aWxpdHlCYXJOb2RlKHV0aWxpdGllcywga2luZC5pZCwgb25BY3Rpb25TdGFibGUsIGludHJvZHVjdGlvblV0aWxpdHlJZCwgY2hyb21lLnV0aWxpdHlPcHRpb25zKSxcbiAgICAgICAgdXRpbGl0eUJhckZvbGRlZDogdXRpbGl0eUJhckZvbGRlZEZvcihraW5kLmlkLCBraW5kLmlkKSxcbiAgICAgICAgYWN0aW9uUGFuZTogd2luZG93QWN0aW9uUGFuZU5vZGUoc2Vzc2lvbi5hcHAsIGtpbmQsIGtpbmQuaWQsIGFjdGlvblBhbmVTbGljZSwgb25BY3Rpb25TdGFibGUsIGRpc3BhdGNoLCBhcHBMYWJlbHNPdmVybGF5LCB1aVRlcm1pbm9sb2d5LCB1aUxvY2FsZSksXG4gICAgICAgIGFjdGlvbnNGb2xkZWQ6IGFjdGlvbnNGb2xkZWRGb3Ioa2luZC5pZCwga2luZC5pZCksXG4gICAgICAgIG9uQWN0aW9uc0ZvbGRlZENoYW5nZTogb25BY3Rpb25zRm9sZGVkRm9yKGtpbmQuaWQpLFxuICAgICAgICBzdGF0dXM6IGRlY2xhcmF0aXZlU3VyZmFjZVN0YXR1cyh3aW5kb3dVaUJ5V2luZG93SWRba2luZC5pZF0pLFxuICAgICAgICBza2VsZXRvbjogPFdpbmRvd0JvZHlTa2VsZXRvbiAvPixcbiAgICAgICAgY2hpbGRyZW46IChcbiAgICAgICAgICA8Q2hyb21lQXdhcmVXaW5kb3dTY3JvbGxTdXJmYWNlIGlkPXtjaGlsZEVsZW1lbnRJZChcImZyYW1ld29yay53aW5kb3dcIiwga2luZC5pZCl9IGNsYXNzTmFtZT1cInJlbGF0aXZlIGZsZXggaC1mdWxsIG1pbi1oLTAgbWluLXctMCBmbGV4LTEgZmxleC1jb2wgb3ZlcmZsb3ctaGlkZGVuXCIgc3R5bGU9e2N1cnNvckZvcihzZXNzaW9uLmFwcCwga2luZC5pZCl9PlxuICAgICAgICAgICAgPFdpbmRvd0luc3RhbmNlSWRDb250ZXh0LlByb3ZpZGVyIHZhbHVlPXtraW5kLmlkfT5cbiAgICAgICAgICAgICAgPFNoZWxsRmF1bHRCb3VuZGFyeSBib3VuZGFyeUlkPXtgd2luZG93LSR7a2luZC5pZH1gfSBmYWxsYmFja0xhYmVsPXtzaGVsbExhYmVsKFwidWkuY29tbW9uLnJlbmRlckVycm9yXCIpfT5cbiAgICAgICAgICAgICAgICA8SW50ZXJwcmV0ZWRVaU5vZGUgbm9kZT17d2luZG93VWlCeVdpbmRvd0lkW2tpbmQuaWRdID8/IHBlbmRpbmdXaW5kb3dVaU5vZGUoKX0gb25BY3Rpb249e29uQWN0aW9uU3RhYmxlfSAvPlxuICAgICAgICAgICAgICA8L1NoZWxsRmF1bHRCb3VuZGFyeT5cbiAgICAgICAgICAgIDwvV2luZG93SW5zdGFuY2VJZENvbnRleHQuUHJvdmlkZXI+XG4gICAgICAgICAgPC9DaHJvbWVBd2FyZVdpbmRvd1Njcm9sbFN1cmZhY2U+XG4gICAgICAgICksXG4gICAgICB9O1xuICAgIH0pO1xuICAgIC8vIPCfqp/vuI8gRWFjaCBleHRyYSAoc3BsaXQvc3Bhd25lZCkgaW5zdGFuY2UgcmVuZGVycyBpdHMgT1dOIGB3aW5kb3dVaUJ5V2luZG93SWRbaW5zdGFuY2UuaWRdYCBib2R5LFxuICAgIC8vIG1lYXN1cmVzLCBhbmQgZW5nYWdlbWVudCDigJQgbmV2ZXIgdGhlIGJhc2Uga2luZCdzIHNoYXJlZCBlbnRyeSDigJQgc28gdHdvIGluc3RhbmNlcyBvZiB0aGUgc2FtZSBraW5kXG4gICAgLy8gKGUuZy4gc3BsaXQgdG9wL3BlcnNwZWN0aXZlIHBhbmVzKSBuZXZlciBzaG93IG9yIGFmZmVjdCBlYWNoIG90aGVyJ3Mgb3B0aW9ucy4gYGRhdGEtZWxlbWVudC1hbGlhc2BcbiAgICAvLyBhbGlhc2VzIHRoZSBpbnN0YW5jZSB0byBpdHMgd2luZG93IGtpbmQncyBlbGVtZW50IGlkIHNvIGFuIGludHJvZHVjdGlvbiBgc2hvd2AgdGFyZ2V0IG9mIHRoZSBraW5kXG4gICAgLy8gKG5vdCBhIHNwZWNpZmljIGluc3RhbmNlKSByYWlzZXMgZXZlcnkgb3BlbiBpbnN0YW5jZSBhYm92ZSB0aGUgZ2xhc3MsIG5vdCBvbmx5IHRoZSBiYXNlIG9uZS5cbiAgICBjb25zdCBleHRyYVdpbmRvd3MgPSBleHRyYVdpbmRvd0luc3RhbmNlcy5mbGF0TWFwKChpbnN0YW5jZSkgPT4ge1xuICAgICAgY29uc3Qga2luZCA9IHNlc3Npb24uYXBwLndpbmRvd0tpbmRzLmZpbmQoKGVudHJ5KSA9PiBlbnRyeS5pZCA9PT0gaW5zdGFuY2Uud2luZG93S2luZElkKTtcbiAgICAgIGlmICgha2luZCkgcmV0dXJuIFtdO1xuICAgICAgY29uc3QgdXRpbGl0aWVzID0gcmVzb2x2ZVV0aWxpdHlOb2RlcyhzZXNzaW9uLmFwcCwga2luZCwgYWN0aXZlVXRpbGl0eUJ5V2luZG93SWRbaW5zdGFuY2UuaWRdLCBpbnN0YW5jZS5pZCwgYXBwTGFiZWxzT3ZlcmxheSwgdWlUZXJtaW5vbG9neSwgdWlMb2NhbGUpO1xuICAgICAgY29uc3QgY2hyb21lID0gd2luZG93TWVhc3VyZXNDaHJvbWUod2luZG93TWVhc3VyZXNCeVdpbmRvd0lkW2luc3RhbmNlLmlkXSA/PyBraW5kLm9wdGlvbnMubWVhc3VyZXMsIGFjdGl2ZVV0aWxpdHlCeVdpbmRvd0lkW2luc3RhbmNlLmlkXSwgaW5zdGFuY2UuaWQsIG9uQWN0aW9uU3RhYmxlKTtcbiAgICAgIGNvbnN0IHJlc29sdmVkRW5nYWdlbWVudCA9IHJlc29sdmVXaW5kb3dFbmdhZ2VtZW50KGtpbmQsIGluc3RhbmNlLmlkLCB3aW5kb3dFbmdhZ2VtZW50c0J5V2luZG93SWQpO1xuICAgICAgcmV0dXJuIFtcbiAgICAgICAge1xuICAgICAgICAgIGlkOiBpbnN0YW5jZS5pZCxcbiAgICAgICAgICBpY29uSWQ6IHdpbmRvd0ljb25zQnlJZFtpbnN0YW5jZS5pZF0gPz8ga2luZC5pY29uSWQsXG4gICAgICAgICAgdGl0bGU6IHdpbmRvd1RpdGxlc0J5SWRbaW5zdGFuY2UuaWRdID8/IGluc3RhbmNlLnRpdGxlLFxuICAgICAgICAgIGZpbGw6IHRydWUsXG4gICAgICAgICAgc2hvd0NvbnRyb2xzOiB0cnVlLFxuICAgICAgICAgIG1lYXN1cmVzOiBjaHJvbWUubWVhc3VyZXMsXG4gICAgICAgICAgbWVhc3VyZXNGb2xkZWQ6IG1lYXN1cmVzRm9sZGVkRm9yKGluc3RhbmNlLmlkLCBpbnN0YW5jZS53aW5kb3dLaW5kSWQpLFxuICAgICAgICAgIGVuZ2FnZW1lbnQ6IHdpbmRvd0VuZ2FnZW1lbnRUb1NwZWMocmVzb2x2ZWRFbmdhZ2VtZW50LCBvbkFjdGlvblN0YWJsZSksXG4gICAgICAgICAgc2VhcmNoOiB3aW5kb3dFbmdhZ2VtZW50VG9TZWFyY2hTcGVjKHJlc29sdmVkRW5nYWdlbWVudCwgb25BY3Rpb25TdGFibGUpLFxuICAgICAgICAgIHV0aWxpdHlCYXI6IHV0aWxpdHlCYXJOb2RlKHV0aWxpdGllcywgaW5zdGFuY2UuaWQsIG9uQWN0aW9uU3RhYmxlLCBpbnRyb2R1Y3Rpb25VdGlsaXR5SWQsIGNocm9tZS51dGlsaXR5T3B0aW9ucyksXG4gICAgICAgICAgdXRpbGl0eUJhckZvbGRlZDogdXRpbGl0eUJhckZvbGRlZEZvcihpbnN0YW5jZS5pZCwgaW5zdGFuY2Uud2luZG93S2luZElkKSxcbiAgICAgICAgICBhY3Rpb25QYW5lOiB3aW5kb3dBY3Rpb25QYW5lTm9kZShzZXNzaW9uLmFwcCwga2luZCwgaW5zdGFuY2UuaWQsIGFjdGlvblBhbmVTbGljZSwgb25BY3Rpb25TdGFibGUsIGRpc3BhdGNoLCBhcHBMYWJlbHNPdmVybGF5LCB1aVRlcm1pbm9sb2d5LCB1aUxvY2FsZSksXG4gICAgICAgICAgYWN0aW9uc0ZvbGRlZDogYWN0aW9uc0ZvbGRlZEZvcihpbnN0YW5jZS5pZCwgaW5zdGFuY2Uud2luZG93S2luZElkKSxcbiAgICAgICAgICBvbkFjdGlvbnNGb2xkZWRDaGFuZ2U6IG9uQWN0aW9uc0ZvbGRlZEZvcihpbnN0YW5jZS5pZCksXG4gICAgICAgICAgc3RhdHVzOiBkZWNsYXJhdGl2ZVN1cmZhY2VTdGF0dXMod2luZG93VWlCeVdpbmRvd0lkW2luc3RhbmNlLmlkXSksXG4gICAgICAgICAgc2tlbGV0b246IDxXaW5kb3dCb2R5U2tlbGV0b24gLz4sXG4gICAgICAgICAgY2hpbGRyZW46IChcbiAgICAgICAgICAgIDxDaHJvbWVBd2FyZVdpbmRvd1Njcm9sbFN1cmZhY2VcbiAgICAgICAgICAgICAgaWQ9e2NoaWxkRWxlbWVudElkKFwiZnJhbWV3b3JrLndpbmRvd1wiLCBpbnN0YW5jZS5pZCl9XG4gICAgICAgICAgICAgIGRhdGEtZWxlbWVudC1hbGlhcz17Y2hpbGRFbGVtZW50SWQoXCJmcmFtZXdvcmsud2luZG93XCIsIGtpbmQuaWQpfVxuICAgICAgICAgICAgICBjbGFzc05hbWU9XCJyZWxhdGl2ZSBmbGV4IGgtZnVsbCBtaW4taC0wIG1pbi13LTAgZmxleC0xIGZsZXgtY29sIG92ZXJmbG93LWhpZGRlblwiXG4gICAgICAgICAgICAgIHN0eWxlPXtjdXJzb3JGb3Ioc2Vzc2lvbi5hcHAsIGluc3RhbmNlLmlkKX1cbiAgICAgICAgICAgID5cbiAgICAgICAgICAgICAgPFdpbmRvd0luc3RhbmNlSWRDb250ZXh0LlByb3ZpZGVyIHZhbHVlPXtpbnN0YW5jZS5pZH0+XG4gICAgICAgICAgICAgICAgPFNoZWxsRmF1bHRCb3VuZGFyeSBib3VuZGFyeUlkPXtgd2luZG93LSR7aW5zdGFuY2UuaWR9YH0gZmFsbGJhY2tMYWJlbD17c2hlbGxMYWJlbChcInVpLmNvbW1vbi5yZW5kZXJFcnJvclwiKX0+XG4gICAgICAgICAgICAgICAgICA8SW50ZXJwcmV0ZWRVaU5vZGUgbm9kZT17d2luZG93VWlCeVdpbmRvd0lkW2luc3RhbmNlLmlkXSA/PyBwZW5kaW5nV2luZG93VWlOb2RlKCl9IG9uQWN0aW9uPXtvbkFjdGlvblN0YWJsZX0gLz5cbiAgICAgICAgICAgICAgICA8L1NoZWxsRmF1bHRCb3VuZGFyeT5cbiAgICAgICAgICAgICAgPC9XaW5kb3dJbnN0YW5jZUlkQ29udGV4dC5Qcm92aWRlcj5cbiAgICAgICAgICAgIDwvQ2hyb21lQXdhcmVXaW5kb3dTY3JvbGxTdXJmYWNlPlxuICAgICAgICAgICksXG4gICAgICAgIH0sXG4gICAgICBdO1xuICAgIH0pO1xuICAgIHJldHVybiBbLi4uYmFzZVdpbmRvd3MsIC4uLmV4dHJhV2luZG93c107XG4gIH0sIFtcbiAgICBhY3Rpb25QYW5lRXhwYW5kZWRCeVdpbmRvd0lkLFxuICAgIGFjdGlvblBhbmVGb2xkZWRCeVdpbmRvd0lkLFxuICAgIGFjdGlvblBhbmVTdGFnZWRBcmdzQnlLZXksXG4gICAgYWN0aXZlVXRpbGl0eUJ5V2luZG93SWQsXG4gICAgYXBwTGFiZWxzT3ZlcmxheSxcbiAgICBleHRyYVdpbmRvd0luc3RhbmNlcyxcbiAgICBpbnRyb2R1Y3Rpb25BY3Rpb25XaW5kb3dTZWdtZW50LFxuICAgIGludHJvZHVjdGlvblV0aWxpdHlJZCxcbiAgICBpbnRyb2R1Y3Rpb25VdGlsaXR5V2luZG93SWQsXG4gICAgbG9hZGVkUGx1Z2lucyxcbiAgICBvbkFjdGlvblN0YWJsZSxcbiAgICBwYW5lbCxcbiAgICBzZXNzaW9uLFxuICAgIHNwYXduZWRXaW5kb3dFbmdhZ2VtZW50cyxcbiAgICBzcGF3bmVkV2luZG93TWVhc3VyZXMsXG4gICAgc3Bhd25lZFdpbmRvd1VpLFxuICAgIHN0dWRpb01vZGUsXG4gICAgdWlMb2NhbGUsXG4gICAgdWlUZXJtaW5vbG9neSxcbiAgICB3aW5kb3dFbmdhZ2VtZW50c0J5V2luZG93SWQsXG4gICAgd2luZG93TWVhc3VyZXNCeVdpbmRvd0lkLFxuICAgIHdpbmRvd1RpdGxlc0J5SWQsXG4gICAgd2luZG93SWNvbnNCeUlkLFxuICAgIHdpbmRvd1VpQnlXaW5kb3dJZCxcbiAgXSk7XG5cbiAgY29uc3QgZWZmZWN0aXZlTW9kZUxheW91dCA9IHVzZU1lbW8oXG4gICAgKCkgPT5cbiAgICAgIHNoZWxsTGF5b3V0ID8/XG4gICAgICAoc2Vzc2lvbiA/IHJlc29sdmVGcmFtZXdvcmtMYXlvdXRTZWVkKHNlc3Npb24uYXBwLmRlZmF1bHRMYXlvdXQsIHNlc3Npb24uYXBwLndpbmRvd0tpbmRzLCBhcHBMYWJlbHNPdmVybGF5LCB1aVRlcm1pbm9sb2d5LCB1aUxvY2FsZSkubW9kZUxheW91dCA6IHsga2luZDogXCJzdGFja1wiIGFzIGNvbnN0LCBjaGlsZHJlbjogW10gfSksXG4gICAgW2FwcExhYmVsc092ZXJsYXksIHNlc3Npb24sIHNoZWxsTGF5b3V0LCB1aVRlcm1pbm9sb2d5LCB1aUxvY2FsZV0sXG4gICk7XG5cbiAgY29uc3QgaGFuZGxlQWN0aXZlV2luZG93Q2hhbmdlID0gdXNlQ2FsbGJhY2soXG4gICAgKHZhbHVlOiBzdHJpbmcgfCBudWxsKSA9PiB7XG4gICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0FDVElWRV9XSU5ET1dfSURcIiwgdmFsdWUgfSk7XG4gICAgICBpZiAodmFsdWUpIG5vdGVTaGVsbENvbW1hbmQoXCJzaGVsbC53aW5kb3dBY3RpdmF0ZVwiLCBzaGVsbExhYmVsKFwidWkuc2hlbGxDb21tYW5kLndpbmRvd0FjdGl2YXRlXCIpLCB7IHdpbmRvd0lkOiB2YWx1ZSB9KTtcbiAgICB9LFxuICAgIFtub3RlU2hlbGxDb21tYW5kXSxcbiAgKTtcblxuICAvLyDwn6qf77iPIGBNb2RlLm9uTGF5b3V0Q2hhbmdlYCBmaXJlcyBjb250aW51b3VzbHkgZHVyaW5nIGEgbGl2ZSBkcmFnL3Jlc2l6ZSAob25lIGNhbGwgcGVyIGZyYW1lKSDigJQgY2xhc3NpZnlcbiAgLy8gZWFjaCBkZWx0YSBhZ2FpbnN0IHRoZSBsYXN0LXNlZW4gbGF5b3V0LCByZW1lbWJlciBvbmx5IHRoZSBsYXRlc3Qgbm9uLW51bGwgY2xhc3NpZmljYXRpb24sIGFuZCBub3RlIGFcbiAgLy8gc2luZ2xlIHNoZWxsIGNvbW1hbmQgb25jZSB0aGUgZHJhZyBzZXR0bGVzIChzZWUgYExBWU9VVF9DSEFOR0VfU0VUVExFX01TYCkuIEEgcHVyZSBhY3RpdmUtd2luZG93LWZsYWdcbiAgLy8gZWNobyBjbGFzc2lmaWVzIGBudWxsYCBhbmQgaXMgc2lsZW50bHkgc2tpcHBlZCBoZXJlIChoYW5kbGVkIGJ5IGBoYW5kbGVBY3RpdmVXaW5kb3dDaGFuZ2VgIGluc3RlYWQpLlxuICBjb25zdCBsYXlvdXRDaGFuZ2VTZXR0bGVUaW1lb3V0UmVmID0gdXNlUmVmPFJldHVyblR5cGU8dHlwZW9mIHNldFRpbWVvdXQ+IHwgbnVsbD4obnVsbCk7XG4gIGNvbnN0IGxheW91dENoYW5nZUNsYXNzaWZpY2F0aW9uUmVmID0gdXNlUmVmPFwicmVzaXplXCIgfCBcInJlYXJyYW5nZVwiIHwgbnVsbD4obnVsbCk7XG4gIGNvbnN0IGxheW91dENoYW5nZVByZXZpb3VzUmVmID0gdXNlUmVmPFdpbmRvd0xheW91dE5vZGUgfCBudWxsPihlZmZlY3RpdmVNb2RlTGF5b3V0KTtcbiAgdXNlRWZmZWN0KCgpID0+IHtcbiAgICBsYXlvdXRDaGFuZ2VQcmV2aW91c1JlZi5jdXJyZW50ID0gZWZmZWN0aXZlTW9kZUxheW91dDtcbiAgfSwgW2VmZmVjdGl2ZU1vZGVMYXlvdXRdKTtcbiAgdXNlRWZmZWN0KFxuICAgICgpID0+ICgpID0+IHtcbiAgICAgIGlmIChsYXlvdXRDaGFuZ2VTZXR0bGVUaW1lb3V0UmVmLmN1cnJlbnQpIGNsZWFyVGltZW91dChsYXlvdXRDaGFuZ2VTZXR0bGVUaW1lb3V0UmVmLmN1cnJlbnQpO1xuICAgIH0sXG4gICAgW10sXG4gICk7XG4gIGNvbnN0IGhhbmRsZU1vZGVMYXlvdXRDaGFuZ2UgPSB1c2VDYWxsYmFjayhcbiAgICAodmFsdWU6IFdpbmRvd0xheW91dE5vZGUpID0+IHtcbiAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfU0hFTExfTEFZT1VUXCIsIHZhbHVlIH0pO1xuICAgICAgY29uc3QgY2xhc3NpZmljYXRpb24gPSBjbGFzc2lmeVdpbmRvd0xheW91dENoYW5nZShsYXlvdXRDaGFuZ2VQcmV2aW91c1JlZi5jdXJyZW50LCB2YWx1ZSk7XG4gICAgICBsYXlvdXRDaGFuZ2VQcmV2aW91c1JlZi5jdXJyZW50ID0gdmFsdWU7XG4gICAgICBpZiAoY2xhc3NpZmljYXRpb24pIGxheW91dENoYW5nZUNsYXNzaWZpY2F0aW9uUmVmLmN1cnJlbnQgPSBjbGFzc2lmaWNhdGlvbjtcbiAgICAgIGlmIChsYXlvdXRDaGFuZ2VTZXR0bGVUaW1lb3V0UmVmLmN1cnJlbnQpIGNsZWFyVGltZW91dChsYXlvdXRDaGFuZ2VTZXR0bGVUaW1lb3V0UmVmLmN1cnJlbnQpO1xuICAgICAgbGF5b3V0Q2hhbmdlU2V0dGxlVGltZW91dFJlZi5jdXJyZW50ID0gc2V0VGltZW91dCgoKSA9PiB7XG4gICAgICAgIGxheW91dENoYW5nZVNldHRsZVRpbWVvdXRSZWYuY3VycmVudCA9IG51bGw7XG4gICAgICAgIGNvbnN0IGZpbmFsQ2xhc3NpZmljYXRpb24gPSBsYXlvdXRDaGFuZ2VDbGFzc2lmaWNhdGlvblJlZi5jdXJyZW50O1xuICAgICAgICBsYXlvdXRDaGFuZ2VDbGFzc2lmaWNhdGlvblJlZi5jdXJyZW50ID0gbnVsbDtcbiAgICAgICAgaWYgKGZpbmFsQ2xhc3NpZmljYXRpb24gPT09IFwicmVzaXplXCIpIG5vdGVTaGVsbENvbW1hbmQoXCJzaGVsbC53aW5kb3dSZXNpemVcIiwgc2hlbGxMYWJlbChcInVpLnNoZWxsQ29tbWFuZC53aW5kb3dSZXNpemVcIikpO1xuICAgICAgICBlbHNlIGlmIChmaW5hbENsYXNzaWZpY2F0aW9uID09PSBcInJlYXJyYW5nZVwiKSBub3RlU2hlbGxDb21tYW5kKFwic2hlbGwud2luZG93TW92ZVwiLCBzaGVsbExhYmVsKFwidWkuc2hlbGxDb21tYW5kLndpbmRvd01vdmVcIikpO1xuICAgICAgfSwgTEFZT1VUX0NIQU5HRV9TRVRUTEVfTVMpO1xuICAgIH0sXG4gICAgW25vdGVTaGVsbENvbW1hbmRdLFxuICApO1xuXG4gIGNvbnN0IGNhbnZhcyA9IHVzZU1lbW8oKCkgPT4ge1xuICAgIGlmIChzdHVkaW9Nb2RlICYmIHNoZWxsUm91dGUua2luZCA9PT0gXCJub3RGb3VuZFwiKSB7XG4gICAgICByZXR1cm4gPFNoZWxsUm91dGVOb3RGb3VuZFBhZ2UgcGF0aD17c2hlbGxSb3V0ZS5wYXRofSBvbkhvbWU9eygpID0+IG5hdmlnYXRlSGlzdG9yeShcIi9cIil9IC8+O1xuICAgIH1cbiAgICBjb25zdCBzdXBlcnZpc29yUGx1Z2luSWQgPSBwcmltYXJ5UGx1Z2luSWQ7XG4gICAgY29uc3Qgc3VwZXJ2aXNvclN0YXRlID0gc3VwZXJ2aXNvclBsdWdpbklkID8gcGx1Z2luU3VwZXJ2aXNvckJ5SWRbc3VwZXJ2aXNvclBsdWdpbklkXSA6IHVuZGVmaW5lZDtcbiAgICBpZiAoc3VwZXJ2aXNvclN0YXRlID09PSBcImNyYXNoZWRcIiB8fCBzdXBlcnZpc29yU3RhdGUgPT09IFwicXVhcmFudGluZWRcIikge1xuICAgICAgcmV0dXJuIChcbiAgICAgICAgPFBsdWdpblJlY292ZXJ5UGFuZWxcbiAgICAgICAgICBwbHVnaW5JZD17c3VwZXJ2aXNvclBsdWdpbklkIX1cbiAgICAgICAgICBxdWFyYW50aW5lZD17c3VwZXJ2aXNvclN0YXRlID09PSBcInF1YXJhbnRpbmVkXCJ9XG4gICAgICAgICAgb25SZXN0YXJ0PXsoKSA9PiB7XG4gICAgICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1BMVUdJTl9TVVBFUlZJU09SXCIsIHBsdWdpbklkOiBzdXBlcnZpc29yUGx1Z2luSWQhLCB2YWx1ZTogXCJyZXN0YXJ0aW5nXCIgfSk7XG4gICAgICAgICAgICB2b2lkIHJlbG9hZFBsdWdpbihzdXBlcnZpc29yUGx1Z2luSWQhKTtcbiAgICAgICAgICB9fVxuICAgICAgICAgIG9uRGlzYWJsZT17KCkgPT4ge1xuICAgICAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9QTFVHSU5fU1VQRVJWSVNPUlwiLCBwbHVnaW5JZDogc3VwZXJ2aXNvclBsdWdpbklkISwgdmFsdWU6IFwicXVhcmFudGluZWRcIiB9KTtcbiAgICAgICAgICAgIGlmIChzdXBlcnZpc29yUGx1Z2luSWQgIT09IHByaW1hcnlQbHVnaW5JZCkgdm9pZCB1bmluc3RhbGxQbHVnaW4oc3VwZXJ2aXNvclBsdWdpbklkISk7XG4gICAgICAgICAgfX1cbiAgICAgICAgLz5cbiAgICAgICk7XG4gICAgfVxuICAgIGlmIChlcnJvcilcbiAgICAgIHJldHVybiAoXG4gICAgICAgIDxwIGNsYXNzTmFtZT1cInAtZG91YmxlIHRleHQtc20gdGV4dC1kZXN0cnVjdGl2ZVwiIHJvbGU9XCJhbGVydFwiIGRhdGEtc2VtaW8tb3Mtc2hlbGwtZXJyb3I9XCJcIj5cbiAgICAgICAgICB7ZXJyb3J9XG4gICAgICAgIDwvcD5cbiAgICAgICk7XG4gICAgaWYgKCFzZXNzaW9uKSByZXR1cm4gPENhbnZhc1NrZWxldG9uIGxhYmVsPXtzaGVsbExhYmVsKFwidWkuY29tbW9uLmxvYWRpbmdQbHVnaW5zXCIpfSBjbGFzc05hbWU9e2NuKGxvYWRpbmdCb3JkZXJDbGFzcywgXCJoLWZ1bGwgdy1mdWxsXCIpfSAvPjtcbiAgICBjb25zdCBtb2RlcyA9IHNlc3Npb24uYXBwLm1vZGVzLmxlbmd0aCA+IDAgPyBzZXNzaW9uLmFwcC5tb2RlcyA6IFt7IGlkOiBzZXNzaW9uLmFwcC5pZCwgbGFiZWw6IGFwcERvY3VtZW50TGFiZWwocmVzb2x2ZUFwcERvY3VtZW50KHNlc3Npb24uYXBwLCB1aVRlcm1pbm9sb2d5KSkgfV07XG4gICAgY29uc3Qgc3R1ZGlvSG9tZUJhciA9XG4gICAgICBzdHVkaW9Nb2RlICYmIHNlc3Npb24uYXBwLmlkID09PSBob3N0QXBwSWQgJiYgIXBhbmVsPy5hY3RpdmVTcGF3bmVkSWQgPyAoXG4gICAgICAgIDxidXR0b25cbiAgICAgICAgICB0eXBlPVwiYnV0dG9uXCJcbiAgICAgICAgICBjbGFzc05hbWU9e2NuKGJvcmRlck5vcm1hbEJvdHRvbUNsYXNzLCBcInB4LXNpbmdsZSBweS1zaW5nbGUgdGV4dC1sZWZ0IHRleHQtc20gdGV4dC1tdXRlZC1mb3JlZ3JvdW5kIGhvdmVyOmJnLW11dGVkLzQwIGhvdmVyOnRleHQtZm9yZWdyb3VuZFwiKX1cbiAgICAgICAgICBvbkNsaWNrPXsoKSA9PiBvbkFjdGlvbih7IGNvbnRyb2xsZXJJZDogc2Vzc2lvbi5hcHAuY29udHJvbGxlcklkLCBhY3Rpb246IFwiZ29Ib21lXCIgfSl9XG4gICAgICAgID5cbiAgICAgICAgICDihpAge3NoZWxsTGFiZWwoXCJ1aS5jb21tb24uaG9tZVwiKX1cbiAgICAgICAgPC9idXR0b24+XG4gICAgICApIDogbnVsbDtcbiAgICBjb25zdCBmb2N1c2VkU3Bhd25lZCA9IHBhbmVsPy5hY3RpdmVTcGF3bmVkSWQgPyBwYW5lbC5zcGF3bmVkQXBwcy5maW5kKChlbnRyeSkgPT4gZW50cnkuaWQgPT09IHBhbmVsLmFjdGl2ZVNwYXduZWRJZCkgOiB1bmRlZmluZWQ7XG4gICAgY29uc3QgZm9jdXNlZEJhciA9IGZvY3VzZWRTcGF3bmVkID8gKFxuICAgICAgPGRpdiBjbGFzc05hbWU9e2NuKGJvcmRlck5vcm1hbEJvdHRvbUNsYXNzLCBcImZsZXggaXRlbXMtY2VudGVyIGdhcC1zaW5nbGUgcHgtc2luZ2xlIHB5LXNpbmdsZSB0ZXh0LXNtIHRleHQtbXV0ZWQtZm9yZWdyb3VuZFwiKX0+XG4gICAgICAgIDxidXR0b24gdHlwZT1cImJ1dHRvblwiIGNsYXNzTmFtZT1cImhvdmVyOnRleHQtZm9yZWdyb3VuZFwiIG9uQ2xpY2s9eygpID0+IChvcGVuU3BhY2VJZFJlZi5jdXJyZW50ID8gbmF2aWdhdGVIaXN0b3J5KGAvc3BhY2VzLyR7b3BlblNwYWNlSWRSZWYuY3VycmVudH1gKSA6IG9uQWN0aW9uKHsgY29udHJvbGxlcklkOiBzZXNzaW9uLmFwcC5jb250cm9sbGVySWQsIGFjdGlvbjogXCJjbG9zZUZvY3VzZWRJbnN0YW5jZVwiIH0pKX0+XG4gICAgICAgICAg4oaQIHtzaGVsbExhYmVsKFwidWkuY29tbW9uLmJhY2tUb1dvcmtmbG93XCIpfVxuICAgICAgICA8L2J1dHRvbj5cbiAgICAgICAgPHNwYW4+wrc8L3NwYW4+XG4gICAgICAgIDxzcGFuPnthcHBEb2N1bWVudExhYmVsKHJlc29sdmVEb2N1bWVudEJ5QXBwSWQobG9hZGVkUGx1Z2lucywgZm9jdXNlZFNwYXduZWQuYXBwSWQsIGZvY3VzZWRTcGF3bmVkLmRvY3VtZW50LCB1aVRlcm1pbm9sb2d5KSl9PC9zcGFuPlxuICAgICAgPC9kaXY+XG4gICAgKSA6IG51bGw7XG4gICAgcmV0dXJuIChcbiAgICAgIDxkaXYgY2xhc3NOYW1lPVwiZmxleCBoLWZ1bGwgbWluLWgtMCBmbGV4LWNvbCBvdmVyZmxvdy1oaWRkZW5cIj5cbiAgICAgICAge3N0dWRpb0hvbWVCYXJ9XG4gICAgICAgIHtmb2N1c2VkQmFyfVxuICAgICAgICA8aW5wdXRcbiAgICAgICAgICByZWY9e2ltcG9ydFNwYWNlSW5wdXRSZWZ9XG4gICAgICAgICAgdHlwZT1cImZpbGVcIlxuICAgICAgICAgIC8vIPCfk6bvuI8gYC5wYWNrYCBmaWxlcyBicmFuY2ggdG8gYHMvcGx1Z2luYCdzIHBhY2stYXdhcmUgYGltcG9ydFNwYWNlUGFja1BheWxvYWRgIGFjdGlvblxuICAgICAgICAgIC8vIChgc2VtaW9fZnJhbWV3b3JrX29zOjppbXBvcnRfb3Nfc3BhY2VfZnJvbV9wYWNrYCwgd2F2ZSAyIHMrc2hvbWUrc3N0dWRpbyBmYW1pbHkpIOKAlFxuICAgICAgICAgIC8vIHJlYWQgYXMgYSBkYXRhVXJsLCBzYW1lIHNoYXBlIGFzIHRoZSBnZW5lcmljIGBSZXF1ZXN0RmlsZU9wZW5gL2ByZWFkQXM6IFwiZGF0YVVybFwiYCBwYXRoXG4gICAgICAgICAgLy8gYmVsb3cuIEFueXRoaW5nIGVsc2Uga2VlcHMgcmVhZGluZyBhcyB0ZXh0IGFuZCBkaXNwYXRjaGluZyB0aGUgSlNPTi1lbnZlbG9wZSBcImltcG9ydFNwYWNlXCIuXG4gICAgICAgICAgYWNjZXB0PVwiLnNwaywuZHNsLC5vcHMsYXBwbGljYXRpb24vb2N0ZXQtc3RyZWFtXCJcbiAgICAgICAgICBjbGFzc05hbWU9XCJoaWRkZW5cIlxuICAgICAgICAgIG9uQ2hhbmdlPXsoZXZlbnQpID0+IHtcbiAgICAgICAgICAgIGNvbnN0IGZpbGUgPSBldmVudC50YXJnZXQuZmlsZXM/LlswXTtcbiAgICAgICAgICAgIGlmICghZmlsZSkgcmV0dXJuO1xuICAgICAgICAgICAgaWYgKGZpbGUubmFtZS50b0xvd2VyQ2FzZSgpLmVuZHNXaXRoKFwiLnBhY2tcIikpIHtcbiAgICAgICAgICAgICAgY29uc3QgcmVhZGVyID0gbmV3IEZpbGVSZWFkZXIoKTtcbiAgICAgICAgICAgICAgcmVhZGVyLm9ubG9hZCA9ICgpID0+IHtcbiAgICAgICAgICAgICAgICBjb25zdCBwYXlsb2FkID0gdHlwZW9mIHJlYWRlci5yZXN1bHQgPT09IFwic3RyaW5nXCIgPyByZWFkZXIucmVzdWx0IDogXCJcIjtcbiAgICAgICAgICAgICAgICBvbkFjdGlvbih7IGNvbnRyb2xsZXJJZDogbGFuZGluZ0NvbnRyb2xsZXJJZCA/PyBcIlwiLCBhY3Rpb246IFwiaW1wb3J0U3BhY2VQYWNrUGF5bG9hZFwiLCBhcmdzOiB7IHBheWxvYWQgfSB9KTtcbiAgICAgICAgICAgICAgICBldmVudC50YXJnZXQudmFsdWUgPSBcIlwiO1xuICAgICAgICAgICAgICB9O1xuICAgICAgICAgICAgICByZWFkZXIucmVhZEFzRGF0YVVSTChmaWxlKTtcbiAgICAgICAgICAgICAgcmV0dXJuO1xuICAgICAgICAgICAgfVxuICAgICAgICAgICAgdm9pZCBmaWxlLnRleHQoKS50aGVuKChqc29uKSA9PiB7XG4gICAgICAgICAgICAgIG9uQWN0aW9uKHsgY29udHJvbGxlcklkOiBsYW5kaW5nQ29udHJvbGxlcklkID8/IFwiXCIsIGFjdGlvbjogXCJpbXBvcnRTcGFjZVwiLCBhcmdzOiB7IGpzb24gfSB9KTtcbiAgICAgICAgICAgICAgZXZlbnQudGFyZ2V0LnZhbHVlID0gXCJcIjtcbiAgICAgICAgICAgIH0pO1xuICAgICAgICAgIH19XG4gICAgICAgIC8+XG4gICAgICAgIDxkaXYgY2xhc3NOYW1lPVwibWluLWgtMCBmbGV4LTFcIj5cbiAgICAgICAgICA8U2hlbGxGYXVsdEJvdW5kYXJ5IGJvdW5kYXJ5SWQ9XCJzZXNzaW9uLWNhbnZhc1wiIGZhbGxiYWNrTGFiZWw9e3NoZWxsTGFiZWwoXCJ1aS5jb21tb24ucmVuZGVyRXJyb3JcIil9PlxuICAgICAgICAgICAgPEFwcFxuICAgICAgICAgICAgbW9kZXM9e21vZGVzLm1hcCgobW9kZSkgPT4gKHsgaWQ6IG1vZGUuaWQsIGxhYmVsOiByZXNvbHZlQXBwTGFiZWwoYXBwTGFiZWxzT3ZlcmxheSwgXCJtb2RlXCIsIG1vZGUuaWQsIHJlc29sdmVNYW5pZmVzdExhYmVsKG1vZGUubGFiZWwsIHVpVGVybWlub2xvZ3ksIHVpTG9jYWxlKSksIGNoaWxkcmVuOiBudWxsIH0pKX1cbiAgICAgICAgICAgIGFjdGl2ZU1vZGVJZD17c2Vzc2lvbi52aWV3U3RhdGUuYWN0aXZlTW9kZUlkID8/IG1vZGVzWzBdPy5pZCA/PyBzZXNzaW9uLmFwcC5pZH1cbiAgICAgICAgICAgIG9uQWN0aXZlTW9kZUNoYW5nZT17YXBwbHlNb2RlQ2hhbmdlfVxuICAgICAgICAgICAgY2hyb21lPXtmYWxzZX1cbiAgICAgICAgICA+XG4gICAgICAgICAgICA8TW9kZVxuICAgICAgICAgICAgICBjbGFzc05hbWU9XCJoLWZ1bGwgdy1mdWxsXCJcbiAgICAgICAgICAgICAgbW9iaWxlPXttb2JpbGV9XG4gICAgICAgICAgICAgIHdpbmRvd3M9e21vZGVXaW5kb3dzfVxuICAgICAgICAgICAgICBsYXlvdXQ9e2VmZmVjdGl2ZU1vZGVMYXlvdXR9XG4gICAgICAgICAgICAgIGFjdGl2ZVdpbmRvd0lkPXthY3RpdmVXaW5kb3dJZH1cbiAgICAgICAgICAgICAgb25BY3RpdmVXaW5kb3dDaGFuZ2U9e2hhbmRsZUFjdGl2ZVdpbmRvd0NoYW5nZX1cbiAgICAgICAgICAgICAgb25MYXlvdXRDaGFuZ2U9e2hhbmRsZU1vZGVMYXlvdXRDaGFuZ2V9XG4gICAgICAgICAgICAgIG9uVGVtcGxhdGVEcm9wPXttb2JpbGUgPyB1bmRlZmluZWQgOiBoYW5kbGVUZW1wbGF0ZURyb3B9XG4gICAgICAgICAgICAgIG9uV2luZG93Q2xvc2U9eyh3aW5kb3dJZCkgPT4ge1xuICAgICAgICAgICAgICAgIG5vdGVTaGVsbENvbW1hbmQoXCJzaGVsbC53aW5kb3dDbG9zZVwiLCBzaGVsbExhYmVsKFwidWkuc2hlbGxDb21tYW5kLndpbmRvd0Nsb3NlXCIpLCB7IHdpbmRvd0lkIH0pO1xuICAgICAgICAgICAgICAgIGlmIChzdHVkaW9Nb2RlICYmIHBhbmVsPy5zcGF3bmVkQXBwcy5zb21lKChlbnRyeSkgPT4gZW50cnkuaWQgPT09IHdpbmRvd0lkKSkge1xuICAgICAgICAgICAgICAgICAgY29uc3QgY2xvc2VkU3Bhd25lZCA9IHBhbmVsLnNwYXduZWRBcHBzLmZpbmQoKGVudHJ5KSA9PiBlbnRyeS5pZCA9PT0gd2luZG93SWQpO1xuICAgICAgICAgICAgICAgICAgY29uc3QgbmV4dFNwYXduZWQgPSBwYW5lbC5zcGF3bmVkQXBwcy5maWx0ZXIoKGVudHJ5KSA9PiBlbnRyeS5pZCAhPT0gd2luZG93SWQpO1xuICAgICAgICAgICAgICAgICAgdXBkYXRlU3BhY2VQYW5lbChidWlsZFNwYWNlUGFuZWxTdGF0ZShwYW5lbC5wcm9ncmFtcywgbmV4dFNwYXduZWQsIHBhbmVsLmFjdGl2ZVBhbmVsVGFiLCBuZXh0U3Bhd25lZFswXT8uaWQpKTtcbiAgICAgICAgICAgICAgICAgIC8vIPCfqrbvuI8gQ2xvc2luZyBhIHNwYXduZWQgYXBwJ3Mgd2luZG93IHVzZWQgdG8gbGVhdmUgaXRzIHBsdWdpbiBpbnN0YW5jZSBydW5uaW5nIGZvcmV2ZXJcbiAgICAgICAgICAgICAgICAgIC8vIChzZWUgUkVEVUNFLURFTU9OU1RSQVRPUi1JRExFLU1FTU9SWS1GT09UUFJJTlQncyBkb2N1bWVudGVkIHRlYXJkb3duIGdhcCkg4oCUIHRoZSBwYW5lbFxuICAgICAgICAgICAgICAgICAgLy8gZW50cnkgd2FzIGRyb3BwZWQgZnJvbSB0aGUgVUksIGJ1dCBub3RoaW5nIGV2ZXIgdG9sZCB0aGUgZ3Vlc3QgdG8gZnJlZSBpdC5cbiAgICAgICAgICAgICAgICAgIGlmIChjbG9zZWRTcGF3bmVkKSB7XG4gICAgICAgICAgICAgICAgICAgIGNvbnN0IGNsb3NlZFBsdWdpbiA9IGxvYWRlZFBsdWdpbnMuZmluZCgoZW50cnkpID0+IGVudHJ5LmhhbmRsZS5wbHVnaW5JZCA9PT0gY2xvc2VkU3Bhd25lZC5wbHVnaW5JZCk/LmhhbmRsZTtcbiAgICAgICAgICAgICAgICAgICAgdm9pZCBjbG9zZWRQbHVnaW4/LmRlc3Ryb3lBcHAoY2xvc2VkU3Bhd25lZC5pbnN0YW5jZUlkKS5jYXRjaCgoKSA9PiB7fSk7XG4gICAgICAgICAgICAgICAgICB9XG4gICAgICAgICAgICAgICAgfVxuICAgICAgICAgICAgICAgIGNsZWFyUGVuZGluZ1dvcmxkUHJvamVjdGlvbih3aW5kb3dJZCk7XG4gICAgICAgICAgICAgICAgZGlzcGF0Y2goe1xuICAgICAgICAgICAgICAgICAgdHlwZTogXCJTRVRfRVhUUkFfV0lORE9XX0lOU1RBTkNFU1wiLFxuICAgICAgICAgICAgICAgICAgdmFsdWU6IChjdXJyZW50KSA9PiB7XG4gICAgICAgICAgICAgICAgICAgIGNvbnN0IG5leHQgPSBjdXJyZW50LmZpbHRlcigoZW50cnkpID0+IGVudHJ5LmlkICE9PSB3aW5kb3dJZCk7XG4gICAgICAgICAgICAgICAgICAgIGV4dHJhV2luZG93SW5zdGFuY2VzUmVmLmN1cnJlbnQgPSBuZXh0O1xuICAgICAgICAgICAgICAgICAgICByZXR1cm4gbmV4dDtcbiAgICAgICAgICAgICAgICAgIH0sXG4gICAgICAgICAgICAgICAgfSk7XG4gICAgICAgICAgICAgICAgZGlzcGF0Y2goe1xuICAgICAgICAgICAgICAgICAgdHlwZTogXCJTRVRfU0hFTExfTEFZT1VUXCIsXG4gICAgICAgICAgICAgICAgICB2YWx1ZTogKGN1cnJlbnQpID0+IGN1cnJlbnQgPz8gcmVzb2x2ZUZyYW1ld29ya0xheW91dFNlZWQoc2Vzc2lvbi5hcHAuZGVmYXVsdExheW91dCwgc2Vzc2lvbi5hcHAud2luZG93S2luZHMsIGFwcExhYmVsc092ZXJsYXksIHVpVGVybWlub2xvZ3ksIHVpTG9jYWxlKS5tb2RlTGF5b3V0LFxuICAgICAgICAgICAgICAgIH0pO1xuICAgICAgICAgICAgICB9fVxuICAgICAgICAgICAgLz5cbiAgICAgICAgICA8L0FwcD5cbiAgICAgICAgICA8L1NoZWxsRmF1bHRCb3VuZGFyeT5cbiAgICAgICAgPC9kaXY+XG4gICAgICA8L2Rpdj5cbiAgICApO1xuICB9LCBbYWN0aXZlV2luZG93SWQsIGVmZmVjdGl2ZU1vZGVMYXlvdXQsIGVycm9yLCBoYW5kbGVBY3RpdmVXaW5kb3dDaGFuZ2UsIGhhbmRsZU1vZGVMYXlvdXRDaGFuZ2UsIGhhbmRsZVRlbXBsYXRlRHJvcCwgbG9hZGVkUGx1Z2lucywgbW9iaWxlLCBtb2RlV2luZG93cywgbmF2aWdhdGVIaXN0b3J5LCBub3RlU2hlbGxDb21tYW5kLCBvbkFjdGlvbiwgcGFuZWwsIHBsdWdpblN1cGVydmlzb3JCeUlkLCBwcmltYXJ5UGx1Z2luSWQsIHJlbG9hZFBsdWdpbiwgc2Vzc2lvbiwgc2hlbGxSb3V0ZSwgc3R1ZGlvTW9kZSwgdWlMb2NhbGUsIHVpVGVybWlub2xvZ3ksIHVwZGF0ZVNwYWNlUGFuZWwsIGRpc3BhdGNoLCB1bmluc3RhbGxQbHVnaW5dKTtcblxuICBjb25zdCBmb290ZXJJdGVtcyA9IHVzZU1lbW8oKCk6IE5hdmJhckl0ZW1bXSA9PiB7XG4gICAgLy8g8J+Pm++4jyBNaXQgQmVzdGFuZCBBZ2dyZWdhdG9yIHBhcnRuZXIgY3JlZGl0czogbGVmdCBcIkVpbiBQcm9qZWt0IHZvbiBMVUggdW5kIFVkS1wiLCByaWdodCBcIkdlZsO2cmRlcnQgZHVyY2ggWnVrdW5mdCBCYXVcIi5cbiAgICAvLyBBIHNpbmdsZSBtaWRkbGUgZmxleC0xIGZpbGwgcHVzaGVzIHRoZSBmdW5kaW5nIGNyZWRpdCB0byB0aGUgdHJhaWxpbmcgZWRnZTsgZml4ZWQgYHctaHVnZWAgZ2FwcyBrZWVwIGVhY2ggY3JlZGl0XG4gICAgLy8gb2ZmIHRoZSBleGFjdCBjb3JuZXIgcGl4ZWwgdGhhdCBmbG9hdGluZyBjb3JuZXIgcGFuZWxzIGFsc28gYW5jaG9yIHRvIChhIHNlY29uZCBmbGV4LTEgd291bGQgY2VudGVyIHRoZSBmdW5kaW5nXG4gICAgLy8gY3JlZGl0IHVuZGVyIHRoZSBDb21tYW5kIG92ZXJsYXk7IGB3LWRvdWJsZWAgcmVhZHMgYXMgZmx1c2ggYWdhaW5zdCB0aGUgdG9nZ2xlIGdyb3VwKS5cbiAgICAvLyDwn5Ox77iPIFRoZSB0aHJlZSB0YWIgYmFycyBoYXZlIG5vIGFuY2hvciBvbiBtb2JpbGUgKGFsbCBhbmNob3JzIG1lcmdlIGludG8gdGhlIG1vYmlsZSBwYW5lbCkg4oCUIG9ubHkgdGhlIGNyZWRpdHMgc3RheS5cbiAgICBjb25zdCBpdGVtczogTmF2YmFySXRlbVtdID0gbW9iaWxlXG4gICAgICA/IFtdXG4gICAgICA6IFtcbiAgICAgICAgICB7IGtleTogXCJib3R0b21MZWZ0UGFuZWxUYWJzXCIsIGNvbnRlbnQ6IDxQYW5lbENocm9tZVRhYkJhciBhbmNob3I9XCJib3R0b20tbGVmdFwiIHsuLi5idWlsZFBhbmVsU2VsZWN0aW9uUHJvcHMoXCJib3R0b20tbGVmdFwiKX0gLz4gfSxcbiAgICAgICAgICB7IGtleTogXCJib3R0b21NaWRkbGVQYW5lbFRhYnNcIiwgY2VudGVyZWQ6IHRydWUsIGNvbnRlbnQ6IDxQYW5lbENocm9tZVRhYkJhciBhbmNob3I9XCJib3R0b20tbWlkZGxlXCIgey4uLmJ1aWxkUGFuZWxTZWxlY3Rpb25Qcm9wcyhcImJvdHRvbS1taWRkbGVcIil9IC8+IH0sXG4gICAgICAgIF07XG4gICAgaWYgKGJyYW5kPy5pZCAmJiAoRU5UV0VSRkVOX01JVF9CRVNUQU5EX0JSQU5EX0lEUyBhcyByZWFkb25seSBzdHJpbmdbXSkuaW5jbHVkZXMoYnJhbmQuaWQpKSB7XG4gICAgICBpdGVtcy5wdXNoKFxuICAgICAgICB7IGtleTogXCJmb290ZXJQcm9qZWN0T2ZHYXBcIiwgY2xhc3NOYW1lOiBcInctaHVnZVwiLCBjb250ZW50OiBudWxsIH0sXG4gICAgICAgIGFQcm9qZWN0T2ZMdWhVZGtGb290ZXJJdGVtKFwiYVByb2plY3RPZkx1aFVka1wiLCB1aUxvY2FsZSwgbW9iaWxlKSxcbiAgICAgICAgbmF2YmFyRmlsbEl0ZW0oXCJmb290ZXJMZWFkaW5nRmlsbFwiKSxcbiAgICAgICAgZnVuZGVkQnladWt1bmZ0QmF1Rm9vdGVySXRlbShcImZ1bmRlZEJ5WnVrdW5mdEJhdVwiLCB1aUxvY2FsZSwgbW9iaWxlKSxcbiAgICAgICAgeyBrZXk6IFwiZm9vdGVyRnVuZGVkQnlHYXBcIiwgY2xhc3NOYW1lOiBcInctaHVnZVwiLCBjb250ZW50OiBudWxsIH0sXG4gICAgICApO1xuICAgIH0gZWxzZSB7XG4gICAgICBpdGVtcy5wdXNoKG5hdmJhckZpbGxJdGVtKFwiZm9vdGVyTGVhZGluZ0ZpbGxcIikpO1xuICAgIH1cbiAgICBpZiAoIW1vYmlsZSkgaXRlbXMucHVzaCh7IGtleTogXCJib3R0b21SaWdodFBhbmVsVGFic1wiLCBjb250ZW50OiA8UGFuZWxDaHJvbWVUYWJCYXIgYW5jaG9yPVwiYm90dG9tLXJpZ2h0XCIgey4uLmJ1aWxkUGFuZWxTZWxlY3Rpb25Qcm9wcyhcImJvdHRvbS1yaWdodFwiKX0gLz4gfSk7XG4gICAgcmV0dXJuIGl0ZW1zO1xuICB9LCBbYnJhbmQ/LmlkLCBidWlsZFBhbmVsU2VsZWN0aW9uUHJvcHMsIG1vYmlsZSwgdWlMb2NhbGVdKTtcblxuICBjb25zdCBidWlsZFBhbmVsUHJvcHMgPSB1c2VDYWxsYmFjayhcbiAgICAoYW5jaG9yOiBBbmNob3IpID0+ICh7XG4gICAgICAuLi5idWlsZFBhbmVsU2VsZWN0aW9uUHJvcHMoYW5jaG9yKSxcbiAgICAgIHNpemU6IHBhbmVsc1thbmNob3JdLnNpemUsXG4gICAgICBvblNpemVDaGFuZ2U6ICh2YWx1ZTogbnVtYmVyKSA9PiBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1BBTkVMX1NJWkVcIiwgYW5jaG9yLCB2YWx1ZSB9KSxcbiAgICAgIHRhYkJhckhvc3Q6IChQQU5FTF9UQUJfQkFSX0hPU1RTW2FuY2hvcl0gPyBcImNocm9tZVwiIDogXCJwYW5lbFwiKSBhcyBcInBhbmVsXCIgfCBcImNocm9tZVwiLFxuICAgICAgdHJlZU9wZW5TdGF0ZXMsXG4gICAgICBvblRyZWVPcGVuU3RhdGVDaGFuZ2U6IChpZDogc3RyaW5nLCBvcGVuOiBib29sZWFuKSA9PiBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX1RSRUVfT1BFTl9TVEFURVwiLCBpZCwgb3BlbiB9KSxcbiAgICB9KSxcbiAgICBbYnVpbGRQYW5lbFNlbGVjdGlvblByb3BzLCBwYW5lbHMsIHRyZWVPcGVuU3RhdGVzXSxcbiAgKTtcblxuICAvLyAjcmVnaW9uIPCflJbvuI9SZWFkaW5lc3NCZWFjb25cbiAgLyoqIPCfmqbvuI8gRGV0ZXJtaW5pc3RpYyBET00gYmVhY29uIGZvciBoZWFkbGVzcyBzbW9rZSB0ZXN0cyAoZS5nLiBTdG9yeWJvb2sncyBPUy1zaGVsbCBwbHVnaW4tYm9vdCBtYXRyaXgpXG4gICAqIHRvIHdhaXQgb24gaW5zdGVhZCBvZiBzY3JlZW5zaG90cy90aW1lb3V0cyDigJQgc2V0IG9uY2UgYSBzZXNzaW9uIHJlc29sdmVzIG9yIGVycm9ycywgY2xlYXJlZCBvbiB1bm1vdW50LiAqL1xuICB1c2VFZmZlY3QoKCkgPT4ge1xuICAgIGNvbnN0IHJvb3QgPSBkb2N1bWVudC5kb2N1bWVudEVsZW1lbnQ7XG4gICAgY29uc3QgYmVhY29uSWQgPSBwbHVnaW5GaWx0ZXIgPz8gXCJ1bmtub3duXCI7XG4gICAgY29uc3Qgbm90Rm91bmQgPSBzdHVkaW9Nb2RlICYmIHNoZWxsUm91dGUua2luZCA9PT0gXCJub3RGb3VuZFwiO1xuICAgIGlmIChub3RGb3VuZCkge1xuICAgICAgcm9vdC5kYXRhc2V0LnNlbWlvT3NOb3RGb3VuZCA9IGJlYWNvbklkO1xuICAgICAgZGVsZXRlIHJvb3QuZGF0YXNldC5zZW1pb09zUmVhZHk7XG4gICAgICBkZWxldGUgcm9vdC5kYXRhc2V0LnNlbWlvT3NFcnJvcjtcbiAgICB9IGVsc2UgaWYgKGVycm9yKSB7XG4gICAgICByb290LmRhdGFzZXQuc2VtaW9Pc0Vycm9yID0gYmVhY29uSWQ7XG4gICAgICBkZWxldGUgcm9vdC5kYXRhc2V0LnNlbWlvT3NSZWFkeTtcbiAgICAgIGRlbGV0ZSByb290LmRhdGFzZXQuc2VtaW9Pc05vdEZvdW5kO1xuICAgIH0gZWxzZSBpZiAoc2Vzc2lvbikge1xuICAgICAgcm9vdC5kYXRhc2V0LnNlbWlvT3NSZWFkeSA9IGJlYWNvbklkO1xuICAgICAgZGVsZXRlIHJvb3QuZGF0YXNldC5zZW1pb09zRXJyb3I7XG4gICAgICBkZWxldGUgcm9vdC5kYXRhc2V0LnNlbWlvT3NOb3RGb3VuZDtcbiAgICB9XG4gICAgcmV0dXJuICgpID0+IHtcbiAgICAgIGRlbGV0ZSByb290LmRhdGFzZXQuc2VtaW9Pc1JlYWR5O1xuICAgICAgZGVsZXRlIHJvb3QuZGF0YXNldC5zZW1pb09zRXJyb3I7XG4gICAgICBkZWxldGUgcm9vdC5kYXRhc2V0LnNlbWlvT3NOb3RGb3VuZDtcbiAgICB9O1xuICB9LCBbc2Vzc2lvbiwgZXJyb3IsIHBsdWdpbkZpbHRlciwgc2hlbGxSb3V0ZS5raW5kLCBzdHVkaW9Nb2RlXSk7XG4gIC8vICNlbmRyZWdpb24g8J+Ulu+4j1JlYWRpbmVzc0JlYWNvblxuXG4gIC8vI3JlZ2lvbiDwn5ax77iPU2hlbGxDb250ZXh0TWVudVxuICAvKiog8J+Wse+4jyBEaXNwYXRjaCBzaW5rIGZvciB0aGUgc2hlbGwgZmFsbGJhY2sgbWVudSdzIGBDb250ZXh0TWVudUl0ZW1TcGVjYHMgKHNlZVxuICAgKiBgYnVpbGRTaGVsbENvbnRleHRNZW51SXRlbXNgKSDigJQgaW50ZXJjZXB0cyB0aGUgdHdvIHJlc2VydmVkIGlkcyB0aGUgYnVpbGRlciBlbWl0cyBpbiBwbGFjZSBvZiBhXG4gICAqIHJlYWwgZGlzcGF0Y2ggKGBcInNoZWxsLm9wZW5BY3Rpb25QYW5lXCJgL2BcInNoZWxsLm9wZW5QYWxldHRlXCJgKSBhbmQgZm9yd2FyZHMgZXZlcnl0aGluZyBlbHNlIHRvXG4gICAqIGBvbkFjdGlvbmAsIG1pcnJvcmluZyB0aGUgY29tbWFuZCBwYWxldHRlJ3Mgb3duIGFyZy1jYXJyeWluZyByZWRpcmVjdC4gKi9cbiAgY29uc3QgZGlzcGF0Y2hTaGVsbE1lbnVBY3Rpb24gPSB1c2VDYWxsYmFjayhcbiAgICAoYWN0aW9uOiBzdHJpbmcsIGFyZ3M/OiBSZWNvcmQ8c3RyaW5nLCB1bmtub3duPikgPT4ge1xuICAgICAgaWYgKCFzZXNzaW9uKSByZXR1cm47XG4gICAgICBpZiAoYWN0aW9uID09PSBcInNoZWxsLm9wZW5BY3Rpb25QYW5lXCIpIHtcbiAgICAgICAgY29uc3Qgd2luZG93S2luZCA9IHNlc3Npb24uYXBwLndpbmRvd0tpbmRzLmZpbmQoKGtpbmQpID0+IGtpbmQuaWQgPT09IGFjdGl2ZVdpbmRvd0lkKSA/PyBzZXNzaW9uLmFwcC53aW5kb3dLaW5kc1swXTtcbiAgICAgICAgY29uc3QgYWN0aW9uSWQgPSB0eXBlb2YgYXJncz8uYWN0aW9uSWQgPT09IFwic3RyaW5nXCIgPyBhcmdzLmFjdGlvbklkIDogdW5kZWZpbmVkO1xuICAgICAgICBpZiAoIXdpbmRvd0tpbmQgfHwgIWFjdGlvbklkKSByZXR1cm47XG4gICAgICAgIGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfQUNUSVZFX1dJTkRPV19JRFwiLCB2YWx1ZTogd2luZG93S2luZC5pZCB9KTtcbiAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9BQ1RJT05fUEFORV9GT0xERURcIiwgd2luZG93SWQ6IHdpbmRvd0tpbmQuaWQsIHZhbHVlOiBmYWxzZSB9KTtcbiAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9BQ1RJT05fUEFORV9FWFBBTkRFRFwiLCB3aW5kb3dJZDogd2luZG93S2luZC5pZCwgdmFsdWU6IGFjdGlvbklkIH0pO1xuICAgICAgICByZXR1cm47XG4gICAgICB9XG4gICAgICBpZiAoYWN0aW9uID09PSBcInNoZWxsLm9wZW5QYWxldHRlXCIpIHtcbiAgICAgICAgZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9TRUFSQ0hfT1BFTlwiLCB2YWx1ZTogdHJ1ZSB9KTtcbiAgICAgICAgcmV0dXJuO1xuICAgICAgfVxuICAgICAgb25BY3Rpb24oeyBjb250cm9sbGVySWQ6IHNlc3Npb24uYXBwLmNvbnRyb2xsZXJJZCwgYWN0aW9uIH0pO1xuICAgIH0sXG4gICAgW3Nlc3Npb24sIGFjdGl2ZVdpbmRvd0lkLCBvbkFjdGlvbiwgZGlzcGF0Y2hdLFxuICApO1xuXG4gIC8qKiDwn5ax77iPIEJ1aWxkcyB0aGUgc2hlbGwtbGV2ZWwgZmFsbGJhY2sgbWVudTogdGhlIGFjdGl2ZSB3aW5kb3cncyBkZWNsYXJlZCBhY3Rpb25zICh1bmRvL3JlZG8sIHZpZXdcbiAgICogYWN0aW9ucywgLi4uKSBwbHVzIGEgY29tbWFuZC1wYWxldHRlIG9wZW5lciDigJQgc2hvd24gZm9yIGFueSByaWdodC1jbGljayBubyBpbm5lciBzdXJmYWNlIGNsYWltZWRcbiAgICogKHdpbmRvdyBiYWNrZ3JvdW5kLCBlbXB0eSBwYW5lbC9uYXZiYXIvZm9vdGVyIHNwYWNlLCBhbiBhcHAgd2l0aCBubyBzY2VuZSBhdCBhbGwpLiBBcmctY2FycnlpbmdcbiAgICogYWN0aW9ucyByb3V0ZSB0aHJvdWdoIHRoZSByZXNlcnZlZCBgXCJzaGVsbC5vcGVuQWN0aW9uUGFuZVwiYCBpZCAocGFyaXR5IHdpdGggdGhlIHdncHUgc2hlbGwnc1xuICAgKiBgYnVpbGRfc2hlbGxfY29udGV4dF9tZW51X3NwZWNzYCksIHRoZSB3aG9sZSBzcGVjIGxpc3QgcnVucyB0aHJvdWdoIGBvcmdhbml6ZUNvbnRleHRNZW51YCwgdGhlblxuICAgKiBgbWFwQ29udGV4dE1lbnVTcGVjc2AgYmluZHMgaXQgdG8gYGRpc3BhdGNoU2hlbGxNZW51QWN0aW9uYC4gKi9cbiAgY29uc3QgYnVpbGRTaGVsbENvbnRleHRNZW51SXRlbXMgPSB1c2VDYWxsYmFjaygoKTogQ29udGV4dE1lbnVJdGVtW10gPT4ge1xuICAgIGlmICghc2Vzc2lvbikgcmV0dXJuIFtdO1xuICAgIGNvbnN0IHdpbmRvd0tpbmQgPSBzZXNzaW9uLmFwcC53aW5kb3dLaW5kcy5maW5kKChraW5kKSA9PiBraW5kLmlkID09PSBhY3RpdmVXaW5kb3dJZCkgPz8gc2Vzc2lvbi5hcHAud2luZG93S2luZHNbMF07XG4gICAgY29uc3Qgc3BlY3M6IENvbnRleHRNZW51SXRlbVNwZWNbXSA9IFtdO1xuICAgIGNvbnN0IGNhdGVnb3J5QnlBY3Rpb25JZCA9IG5ldyBNYXA8c3RyaW5nLCBzdHJpbmc+KCk7XG4gICAgaWYgKHdpbmRvd0tpbmQpIHtcbiAgICAgIGZvciAoY29uc3QgYWN0aW9uIG9mIHJlc29sdmVXaW5kb3dBY3Rpb25zKHNlc3Npb24uYXBwLCB3aW5kb3dLaW5kKSkge1xuICAgICAgICAvLyDwn6e577iPIFNhbWUgY3VyYXRpb24gYXMgdGhlIGNvbW1hbmQgcGFsZXR0ZSAoYGlmICghYWN0aW9uLmluUGFsZXR0ZSkgY29udGludWVgKSDigJQgbW9zdCBhcHBzXG4gICAgICAgIC8vIGRlY2xhcmUgaW50ZXJuYWwvcG9pbnRlci10cmFja2luZyB2aWV3IGFjdGlvbnMgKHdvcmxkSG92ZXIsIGVuZ2FnZW1lbnRJbnB1dCwgLi4uKSBhcyB3aW5kb3dcbiAgICAgICAgLy8gYWN0aW9ucyBwdXJlbHkgZm9yIGRpc3BhdGNoIHBsdW1iaW5nOyBvbmx5IHBhbGV0dGUtd29ydGh5IG9uZXMgYmVsb25nIGluIGEgdXNlci1mYWNpbmcgbWVudS5cbiAgICAgICAgaWYgKCFhY3Rpb24uaW5QYWxldHRlKSBjb250aW51ZTtcbiAgICAgICAgY29uc3QgYXJnQ2FycnlpbmcgPSBhY3Rpb25SZXF1aXJlc1N0YWdlZEZvcm0oYWN0aW9uKTtcbiAgICAgICAgY2F0ZWdvcnlCeUFjdGlvbklkLnNldChhY3Rpb24uaWQsIGFjdGlvbkNhdGVnb3J5SWQoYWN0aW9uKSk7XG4gICAgICAgIHNwZWNzLnB1c2goe1xuICAgICAgICAgIGlkOiBgc2hlbGwtbWVudS5hY3Rpb24uJHthY3Rpb24uaWR9YCxcbiAgICAgICAgICBsYWJlbDogcmVzb2x2ZUFwcExhYmVsKGFwcExhYmVsc092ZXJsYXksIFwiYWN0aW9uXCIsIGFjdGlvbi5pZCwgcmVzb2x2ZU1hbmlmZXN0TGFiZWwoYWN0aW9uLmxhYmVsLCB1aVRlcm1pbm9sb2d5LCB1aUxvY2FsZSkpICsgKGFyZ0NhcnJ5aW5nID8gXCLigKZcIiA6IFwiXCIpLFxuICAgICAgICAgIGljb246IGFjdGlvbi5pY29uSWQsXG4gICAgICAgICAgc2hvcnRjdXQ6IGFjdGlvbi5rZXlzID8/IGtleXNCeUFjdGlvbklkLmdldChhY3Rpb24uaWQpLFxuICAgICAgICAgIGRlc3RydWN0aXZlOiBhY3Rpb24ua2luZCA9PT0gXCJvcGVyYXRpb25cIiAmJiBhY3Rpb24uaWQudG9Mb3dlckNhc2UoKS5pbmNsdWRlcyhcImRlbGV0ZVwiKSxcbiAgICAgICAgICBhY3Rpb246IGFyZ0NhcnJ5aW5nID8gXCJzaGVsbC5vcGVuQWN0aW9uUGFuZVwiIDogYWN0aW9uLmlkLFxuICAgICAgICAgIGFyZ3M6IGFyZ0NhcnJ5aW5nID8geyBhY3Rpb25JZDogYWN0aW9uLmlkIH0gOiB1bmRlZmluZWQsXG4gICAgICAgIH0pO1xuICAgICAgfVxuICAgIH1cbiAgICBpZiAoc3BlY3MubGVuZ3RoID4gMCkgc3BlY3MucHVzaCh7IGlkOiBcInNoZWxsLW1lbnUuc2VwYXJhdG9yXCIsIHNlcGFyYXRvcjogdHJ1ZSB9KTtcbiAgICBzcGVjcy5wdXNoKHtcbiAgICAgIGlkOiBcInNoZWxsLm9wZW5QYWxldHRlXCIsXG4gICAgICBsYWJlbDogc2hlbGxMYWJlbChcInVpLnNlYXJjaC50b2dnbGVcIiksXG4gICAgICBpY29uOiBcInNlYXJjaFwiLFxuICAgICAgYWN0aW9uOiBcInNoZWxsLm9wZW5QYWxldHRlXCIsXG4gICAgfSk7XG4gICAgY29uc3Qgb3JnYW5pemVkID0gb3JnYW5pemVDb250ZXh0TWVudShzcGVjcywgKGlkKSA9PiBjYXRlZ29yeUJ5QWN0aW9uSWQuZ2V0KGlkKSk7XG4gICAgcmV0dXJuIG1hcENvbnRleHRNZW51U3BlY3Mob3JnYW5pemVkLCBkaXNwYXRjaFNoZWxsTWVudUFjdGlvbiwga2V5c0J5QWN0aW9uSWQpO1xuICB9LCBbc2Vzc2lvbiwgYWN0aXZlV2luZG93SWQsIGFwcExhYmVsc092ZXJsYXksIGtleXNCeUFjdGlvbklkLCBkaXNwYXRjaFNoZWxsTWVudUFjdGlvbiwgdWlUZXJtaW5vbG9neSwgdWlMb2NhbGVdKTtcblxuICB1c2VFZmZlY3QoKCkgPT4ge1xuICAgIGNvbnN0IGhhbmRsZUNvbnRleHRNZW51ID0gKGV2ZW50OiBNb3VzZUV2ZW50KSA9PiB7XG4gICAgICBpZiAoaXNDb250ZXh0TWVudVBvaW50ZXJUYXJnZXQoZXZlbnQudGFyZ2V0KSkgcmV0dXJuO1xuICAgICAgY29uc3QgaXRlbXMgPSBidWlsZFNoZWxsQ29udGV4dE1lbnVJdGVtcygpO1xuICAgICAgaWYgKGl0ZW1zLmxlbmd0aCA9PT0gMCkgcmV0dXJuO1xuICAgICAgZXZlbnQucHJldmVudERlZmF1bHQoKTtcbiAgICAgIHNldFNoZWxsQ29udGV4dE1lbnUoeyB4OiBldmVudC5jbGllbnRYLCB5OiBldmVudC5jbGllbnRZLCBpdGVtcyB9KTtcbiAgICB9O1xuICAgIHdpbmRvdy5hZGRFdmVudExpc3RlbmVyKFwiY29udGV4dG1lbnVcIiwgaGFuZGxlQ29udGV4dE1lbnUpO1xuICAgIHJldHVybiAoKSA9PiB3aW5kb3cucmVtb3ZlRXZlbnRMaXN0ZW5lcihcImNvbnRleHRtZW51XCIsIGhhbmRsZUNvbnRleHRNZW51KTtcbiAgfSwgW2J1aWxkU2hlbGxDb250ZXh0TWVudUl0ZW1zXSk7XG4gIC8vI2VuZHJlZ2lvbiDwn5ax77iPU2hlbGxDb250ZXh0TWVudVxuXG4gIHJldHVybiAoXG4gICAgPFNldFdpbmRvd1RpdGxlQ29udGV4dC5Qcm92aWRlciB2YWx1ZT17c2V0V2luZG93VGl0bGV9PlxuICAgIDxTZXRXaW5kb3dJY29uQ29udGV4dC5Qcm92aWRlciB2YWx1ZT17c2V0V2luZG93SWNvbn0+XG4gICAgPEFwcEtleWJpbmRpbmdzQ29udGV4dC5Qcm92aWRlciB2YWx1ZT17a2V5c0J5QWN0aW9uSWR9PlxuICAgIDxVaUtleWJpbmRpbmdzUHJvdmlkZXIgYmluZGluZ3M9e2NvbnRyb2xLZXliaW5kaW5nc30+XG4gICAgPFBsdWdpblN1cmZhY2VBY3Rpb25zQ29udGV4dC5Qcm92aWRlciB2YWx1ZT17cmVxdWVzdENvbnRleHRNZW51fT5cbiAgICA8U2hlbGxDb250ZXh0TWVudUZhbGxiYWNrQ29udGV4dC5Qcm92aWRlciB2YWx1ZT17YnVpbGRTaGVsbENvbnRleHRNZW51SXRlbXN9PlxuICAgIDxTaGVsbEZhdWx0Qm91bmRhcnkgYm91bmRhcnlJZD1cInNoZWxsLXJvb3RcIiBmYWxsYmFja0xhYmVsPXtzaGVsbExhYmVsKFwidWkuY29tbW9uLnJlbmRlckVycm9yXCIpfT5cbiAgICA8VUlGaW5kUHJvdmlkZXI+XG4gICAgICA8TGV2ZWxQcm92aWRlciBsZXZlbD1cImJhc2VcIj5cbiAgICAgICAgPGRpdiBjbGFzc05hbWU9XCJmbGV4IGgtc2NyZWVuIG1pbi1oLTAgdy1zY3JlZW4gZmxleC1jb2wgYmctdHJhbnNwYXJlbnRcIiBkYXRhLWxldmVsPVwiYmFzZVwiPlxuICAgICAgICAgIDxQYW5lbERvY2tQcm92aWRlciBkb2NrPXtkb2NrfSBvblRhYkRvY2tEcm9wPXtoYW5kbGVUYWJEb2NrRHJvcH0gb25UcmVlVW5pdERvY2tEcm9wPXtoYW5kbGVUcmVlVW5pdERvY2tEcm9wfT5cbiAgICAgICAgICAgIDxMYXlvdXRcbiAgICAgICAgICAgICAgbW9iaWxlPXttb2JpbGV9XG4gICAgICAgICAgICAgIG1vYmlsZVBhbmVsPXttb2JpbGVQYW5lbH1cbiAgICAgICAgICAgICAgbmF2YmFyPXs8TmF2YmFyIGl0ZW1zPXtuYXZiYXJJdGVtc30gc2hvd0Z1bGxzY3JlZW5Ub2dnbGU9eyFtb2JpbGV9IC8+fVxuICAgICAgICAgICAgICBzdWJuYXZiYXI9e1xuICAgICAgICAgICAgICAgIGFjdGl2ZVR1dG9yaWFsID8gKFxuICAgICAgICAgICAgICAgICAgPFR1dG9yaWFsQmFyXG4gICAgICAgICAgICAgICAgICAgIHRpdGxlPXtyZXNvbHZlTWFuaWZlc3RMYWJlbChhY3RpdmVUdXRvcmlhbC50aXRsZSwgdWlUZXJtaW5vbG9neSwgdWlMb2NhbGUpfVxuICAgICAgICAgICAgICAgICAgICBkdXJhdGlvbk1zPXthY3RpdmVUdXRvcmlhbC5kdXJhdGlvbk1zfVxuICAgICAgICAgICAgICAgICAgICBwbGF5aW5nPXt0dXRvcmlhbFBsYXlpbmd9XG4gICAgICAgICAgICAgICAgICAgIHJhdGU9e3R1dG9yaWFsUmF0ZX1cbiAgICAgICAgICAgICAgICAgICAgbXV0ZWQ9e3R1dG9yaWFsTXV0ZWR9XG4gICAgICAgICAgICAgICAgICAgIGNhcHRpb25zT249e3R1dG9yaWFsQ2FwdGlvbnNPbn1cbiAgICAgICAgICAgICAgICAgICAgcmVjb3JkaW5nPXt0dXRvcmlhbFJlY29yZGluZ31cbiAgICAgICAgICAgICAgICAgICAgcmVjb3JkQXZhaWxhYmxlPXt0dXRvcmlhbFJlY29yZGVyQXZhaWxhYmxlfVxuICAgICAgICAgICAgICAgICAgICBjaGFwdGVycz17dHV0b3JpYWxDaGFwdGVyTWFya2Vyc31cbiAgICAgICAgICAgICAgICAgICAgY2xvY2s9e3R1dG9yaWFsQ2xvY2t9XG4gICAgICAgICAgICAgICAgICAgIG9uUGxheVBhdXNlPXtwbGF5UGF1c2VUdXRvcmlhbH1cbiAgICAgICAgICAgICAgICAgICAgb25TdG9wPXtzdG9wVHV0b3JpYWx9XG4gICAgICAgICAgICAgICAgICAgIG9uU2Vlaz17c2Vla1R1dG9yaWFsfVxuICAgICAgICAgICAgICAgICAgICBvblJhdGVDaGFuZ2U9eyh2YWx1ZSkgPT4gZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9UVVRPUklBTF9SQVRFXCIsIHZhbHVlIH0pfVxuICAgICAgICAgICAgICAgICAgICBvbk11dGVkQ2hhbmdlPXsodmFsdWUpID0+IGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfVFVUT1JJQUxfTVVURURcIiwgdmFsdWUgfSl9XG4gICAgICAgICAgICAgICAgICAgIG9uQ2FwdGlvbnNDaGFuZ2U9eyh2YWx1ZSkgPT4gZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9UVVRPUklBTF9DQVBUSU9OU1wiLCB2YWx1ZSB9KX1cbiAgICAgICAgICAgICAgICAgICAgb25SZWNvcmRUb2dnbGU9e3RvZ2dsZVR1dG9yaWFsUmVjb3JkaW5nfVxuICAgICAgICAgICAgICAgICAgICBvbkFkZENoYXB0ZXI9e2FkZFR1dG9yaWFsQ2hhcHRlcn1cbiAgICAgICAgICAgICAgICAgIC8+XG4gICAgICAgICAgICAgICAgKSA6IHVuZGVmaW5lZFxuICAgICAgICAgICAgICB9XG4gICAgICAgICAgICAgIGZvb3Rlcj17PEZvb3RlciBpdGVtcz17Zm9vdGVySXRlbXN9IC8+fVxuICAgICAgICAgICAgICBwYW5lbHM9e09iamVjdC5mcm9tRW50cmllcyhBTkNIT1JTLm1hcCgoYW5jaG9yKSA9PiBbYW5jaG9yLCBidWlsZFBhbmVsUHJvcHMoYW5jaG9yKV0pKSBhcyBSZWNvcmQ8QW5jaG9yLCBSZXR1cm5UeXBlPHR5cGVvZiBidWlsZFBhbmVsUHJvcHM+Pn1cbiAgICAgICAgICAgICAgY2FudmFzU3RhdHVzPXtzaGVsbFBsdWdpbkNhbnZhc1N0YXR1c31cbiAgICAgICAgICAgICAgY2FudmFzU2tlbGV0b249ezxDYW52YXNTa2VsZXRvbiBsYWJlbD17c2hlbGxMYWJlbChcInVpLmNvbW1vbi5sb2FkaW5nUGx1Z2luc1wiKX0gLz59XG4gICAgICAgICAgICAgIGNhbnZhcz17XG4gICAgICAgICAgICAgICAgPFNoZWxsRmF1bHRCb3VuZGFyeSBib3VuZGFyeUlkPVwicm91dGUtY2FudmFzXCIgZmFsbGJhY2tMYWJlbD17c2hlbGxMYWJlbChcInVpLmNvbW1vbi5yZW5kZXJFcnJvclwiKX0+XG4gICAgICAgICAgICAgICAgICB7Y2FudmFzfVxuICAgICAgICAgICAgICAgIDwvU2hlbGxGYXVsdEJvdW5kYXJ5PlxuICAgICAgICAgICAgICB9XG4gICAgICAgICAgICAvPlxuICAgICAgICAgIDwvUGFuZWxEb2NrUHJvdmlkZXI+XG4gICAgICAgIDwvZGl2PlxuICAgICAgICA8VUlTZWFyY2ggaXRlbXM9e3NlYXJjaEl0ZW1zfSBvcGVuPXtzZWFyY2hPcGVufSBvbk9wZW5DaGFuZ2U9eyh2YWx1ZSkgPT4gZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9TRUFSQ0hfT1BFTlwiLCB2YWx1ZSB9KX0gLz5cbiAgICAgICAgPFVJRmluZCBvcGVuPXtmaW5kT3Blbn0gb25PcGVuQ2hhbmdlPXsodmFsdWUpID0+IGRpc3BhdGNoKHsgdHlwZTogXCJTRVRfRklORF9PUEVOXCIsIHZhbHVlIH0pfSAvPlxuICAgICAgICA8VGV4dFNlbGVjdGlvbkNvbnRleHRNZW51SG9zdCAvPlxuICAgICAgICA8Q29udGV4dE1lbnVDb250cm9sbGVyXG4gICAgICAgICAgdGl0bGU9e3NoZWxsQ29udGV4dE1lbnVUaXRsZUxhYmVsfVxuICAgICAgICAgIG9wZW49e3NoZWxsQ29udGV4dE1lbnUgIT0gbnVsbH1cbiAgICAgICAgICBwb3NpdGlvbj17c2hlbGxDb250ZXh0TWVudX1cbiAgICAgICAgICBpdGVtcz17c2hlbGxDb250ZXh0TWVudT8uaXRlbXMgPz8gW119XG4gICAgICAgICAgb25PcGVuQ2hhbmdlPXsob3BlbikgPT4ge1xuICAgICAgICAgICAgaWYgKCFvcGVuKSBzZXRTaGVsbENvbnRleHRNZW51KG51bGwpO1xuICAgICAgICAgIH19XG4gICAgICAgIC8+XG4gICAgICAgIHtzZXNzaW9uICYmIGFjdGl2ZUludHJvZHVjdGlvbiAmJiBpbnRyb2R1Y3Rpb25TdGVwSW5kZXggIT0gbnVsbCAmJiAoXG4gICAgICAgICAgPFVJSW50cm9kdWN0aW9uXG4gICAgICAgICAgICBpbnRyb2R1Y3Rpb249e2JyYW5kPy5pbnRyb2R1Y3Rpb24gPz8gcmVzb2x2ZUludHJvZHVjdGlvbkRlZmluaXRpb24oYWN0aXZlSW50cm9kdWN0aW9uLCBhcHBMYWJlbHNPdmVybGF5LCB1aVRlcm1pbm9sb2d5LCB1aUxvY2FsZSl9XG4gICAgICAgICAgICBzdGVwSW5kZXg9e2ludHJvZHVjdGlvblN0ZXBJbmRleH1cbiAgICAgICAgICAgIGNvbXBsZXRlZEludGVyYWN0aW9uSW5kaWNlcz17aW50cm9kdWN0aW9uQ29tcGxldGVkSW50ZXJhY3Rpb25zfVxuICAgICAgICAgICAgb25TdGVwSW5kZXhDaGFuZ2U9eyh2YWx1ZSkgPT4gZGlzcGF0Y2goeyB0eXBlOiBcIlNFVF9JTlRST0RVQ1RJT05fU1RFUFwiLCB2YWx1ZSB9KX1cbiAgICAgICAgICAgIG9uRGlzbWlzcz17ZGlzbWlzc0ludHJvZHVjdGlvbn1cbiAgICAgICAgICAvPlxuICAgICAgICApfVxuICAgICAgICB7YWN0aXZlVHV0b3JpYWwgJiYgKFxuICAgICAgICAgIDw+XG4gICAgICAgICAgICA8VHV0b3JpYWxDYXB0aW9uc0hvc3QgdHV0b3JpYWw9e2FjdGl2ZVR1dG9yaWFsfSBjbG9jaz17dHV0b3JpYWxDbG9ja30gY2FwdGlvbnNPbj17dHV0b3JpYWxDYXB0aW9uc09ufSB0ZXJtaW5vbG9neT17dWlUZXJtaW5vbG9neX0gbG9jYWxlPXt1aUxvY2FsZX0gLz5cbiAgICAgICAgICAgIDxUdXRvcmlhbFZpZGVvT3ZlcmxheUhvc3QgdHV0b3JpYWw9e2FjdGl2ZVR1dG9yaWFsfSBjbG9jaz17dHV0b3JpYWxDbG9ja30gbXV0ZWQ9e3R1dG9yaWFsTXV0ZWR9IHBsYXlpbmc9e3R1dG9yaWFsUGxheWluZ30gcmF0ZT17dHV0b3JpYWxSYXRlfSAvPlxuICAgICAgICAgICAgPFR1dG9yaWFsR2hvc3RQb2ludGVySG9zdCB0dXRvcmlhbD17YWN0aXZlVHV0b3JpYWx9IGNsb2NrPXt0dXRvcmlhbENsb2NrfSAvPlxuICAgICAgICAgIDwvPlxuICAgICAgICApfVxuICAgICAgICB7c2Vzc2lvbiAmJlxuICAgICAgICAgIG92ZXJsYXlEaWFsb2cgJiZcbiAgICAgICAgICAoKCkgPT4ge1xuICAgICAgICAgICAgY29uc3QgZGlhbG9nID0gc2Vzc2lvbi5hcHAuZGlhbG9ncz8uZmluZCgoZW50cnkpID0+IGVudHJ5LmlkID09PSBvdmVybGF5RGlhbG9nLmRpYWxvZ0lkKTtcbiAgICAgICAgICAgIGlmICghZGlhbG9nKSByZXR1cm4gbnVsbDtcbiAgICAgICAgICAgIHJldHVybiAoXG4gICAgICAgICAgICAgIDxVSURpYWxvZ1xuICAgICAgICAgICAgICAgIGRpYWxvZz17cmVzb2x2ZURpYWxvZ0RlZmluaXRpb24oZGlhbG9nLCBhcHBMYWJlbHNPdmVybGF5LCB1aVRlcm1pbm9sb2d5LCB1aUxvY2FsZSl9XG4gICAgICAgICAgICAgICAgc2VlZEFyZ3M9e292ZXJsYXlEaWFsb2cuc2VlZEFyZ3N9XG4gICAgICAgICAgICAgICAgcmVuZGVyRmllbGQ9eyhkZWYsIHZhbHVlLCBvbkNoYW5nZSkgPT4gcmVuZGVyU3RhZ2VkQXJnQ29udHJvbChkZWYsIHZhbHVlLCBvbkNoYW5nZSl9XG4gICAgICAgICAgICAgICAgb25TdWJtaXQ9eyhhcmdzKSA9PiB7XG4gICAgICAgICAgICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0RJQUxPR1wiLCB2YWx1ZTogbnVsbCB9KTtcbiAgICAgICAgICAgICAgICAgIG9uQWN0aW9uKHsgY29udHJvbGxlcklkOiBzZXNzaW9uLmFwcC5jb250cm9sbGVySWQsIGFjdGlvbjogZGlhbG9nLnN1Ym1pdEFjdGlvbiwgYXJncyB9KTtcbiAgICAgICAgICAgICAgICB9fVxuICAgICAgICAgICAgICAgIG9uQ2FuY2VsPXsoKSA9PiB7XG4gICAgICAgICAgICAgICAgICBkaXNwYXRjaCh7IHR5cGU6IFwiU0VUX0RJQUxPR1wiLCB2YWx1ZTogbnVsbCB9KTtcbiAgICAgICAgICAgICAgICAgIGlmIChkaWFsb2cuY2FuY2VsQWN0aW9uKSBvbkFjdGlvbih7IGNvbnRyb2xsZXJJZDogc2Vzc2lvbi5hcHAuY29udHJvbGxlcklkLCBhY3Rpb246IGRpYWxvZy5jYW5jZWxBY3Rpb24gfSk7XG4gICAgICAgICAgICAgICAgfX1cbiAgICAgICAgICAgICAgLz5cbiAgICAgICAgICAgICk7XG4gICAgICAgICAgfSkoKX1cbiAgICAgIDwvTGV2ZWxQcm92aWRlcj5cbiAgICA8L1VJRmluZFByb3ZpZGVyPlxuICAgIDwvU2hlbGxGYXVsdEJvdW5kYXJ5PlxuICAgIDwvU2hlbGxDb250ZXh0TWVudUZhbGxiYWNrQ29udGV4dC5Qcm92aWRlcj5cbiAgICA8L1BsdWdpblN1cmZhY2VBY3Rpb25zQ29udGV4dC5Qcm92aWRlcj5cbiAgICA8L1VpS2V5YmluZGluZ3NQcm92aWRlcj5cbiAgICA8L0FwcEtleWJpbmRpbmdzQ29udGV4dC5Qcm92aWRlcj5cbiAgICA8L1NldFdpbmRvd0ljb25Db250ZXh0LlByb3ZpZGVyPlxuICAgIDwvU2V0V2luZG93VGl0bGVDb250ZXh0LlByb3ZpZGVyPlxuICApO1xufVxuLy8jZW5kcmVnaW9uIEZyYW1ld29ya09zU2hlbGxcbiJdLCJmaWxlIjoiL1VzZXJzL3VlbGkvRG9jdW1lbnRzL3NlbWlvL/Cfp7DvuI9mcmFtZXdvcmsv8J+bje+4j3Byb2R1Y3RzL/CfkrvvuI9vcy/wn5So77iPbW9kdWxlcy/wn5O677iPcmVuZGVyZXIv8J+nke+4j+KAjfCfjqjvuI9lbmdpbmUv8J+nse+4j2VsZW1lbnRzL1NoZWxsSG9zdC/wn5+m77iPY29tcG9uZW50LnRzeCJ9