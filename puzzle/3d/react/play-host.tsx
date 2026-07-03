// #region 🧲Header
/** @emoji 🛝 Playground play host for Puzzle3d — loaded only via `./play` subpath. */
// #endregion 🧲Header

import type { ReactElement } from "react";
import { type Playground, type PlaygroundChromeBoot, bootPlayground, mountPlaygroundApp, PlaygroundView, useApp, registerUiPuzzle3dSurfaceHost, registerUiWriterSurfaceHost, registerTabIcon, engagementSpecControlMirror, enforcePuzzle3dPlayWindowEngagement, puzzle3dPlayEngagementMirror, CommandBus, useControllerStore, useShellWindowInstance, registerWindowBody, enforcePlaygroundWindowEngagementInput, playgroundResolvedExampleId } from "@semio-tech/framework-playground-renderer-react";
import { reactHostPort, engagementCommandTokenEquals, normalizeEngagementCommandText } from "@semio-tech/ui-react";
import { type WindowEngagement, type WindowEngagementControl, UiPuzzle3dHostSurfaceNode } from "@semio-tech/framework-playground-core";
import * as React from "react";
// #region 🔌Adapters
import {
    PUZZLE_3D_FILL_COUNT_MAX,
    PUZZLE_3D_PLAY_APP_ID,
    PUZZLE_3D_PLAY_BODY_KEY_JACK,
    PUZZLE_3D_PLAY_CONTROLLER_ID,
    PUZZLE_3D_PLAY_EXAMPLE_CONCRETE_FOREST_ID,
    PUZZLE_3D_PLAY_ICON_HIERARCHY,
    PUZZLE_3D_PLAY_ICON_INSPECTOR,
    PUZZLE_3D_PLAY_ICON_KINDS,
    PUZZLE_3D_PLAY_ICON_SETTINGS,
    PUZZLE_3D_PLAY_IDLE_SNAPSHOT,
    PUZZLE_3D_PLAY_SNAPSHOT_PANEL_BODY_KEYS,
    PUZZLE_3D_PLAY_STORE_ID,
    PUZZLE_3D_PLAY_VIEWPORT_SURFACE_ID,
    PUZZLE_3D_PLAY_SURFACE_ID_JACK,
    PUZZLE_3D_PLAY_WINDOW_KIND_JACK,
    Puzzle3dPlayShellController,
    clearPuzzle3dFillSession,
    getPuzzle3dFillSessionReadyEpoch,
    installPuzzle3dPlayBrushHost,
    parseKindCatalogs,
    parseKindCompatibility,
    preparePuzzle3dFillSession,
    puzzle3dBrushMeshRootForFill,
    puzzle3dFillBuildProgressRef,
    puzzle3dFillPendingCountRef,
    puzzle3dFillSessionRef,
    puzzle3dPlayFixtureJson,
    rerollPuzzle3dFillTail,
    subscribePuzzle3dFillDistributionInvalidated,
    subscribePuzzle3dFillSessionReady,
    subscribePuzzle3dFillTargetVolumesInvalidated,
    type Puzzle3dPlayHostBridge,
    type Puzzle3dPlaySnapshot
} from "@semio-tech/puzzle-3d-core";

import { buildWriterWindowBody } from "@semio-tech/framework-platform-core";
import { createWriterDocument } from "@semio-tech/writer-core";
import { WriterCanvas } from "@semio-tech/writer-react";
import { sceneHostPort } from "@semio-tech/ui-react";
// #endregion 🔌Adapters

function usePuzzle3dPlayController(): Puzzle3dPlayShellController | undefined {
  const { runtime } = useApp();
  return runtime.getActiveApp()?.controller as Puzzle3dPlayShellController | undefined;
}

function usePuzzle3dPlaySnapshot(): Puzzle3dPlaySnapshot {
  const ctrl = usePuzzle3dPlayController();
  return useControllerStore(ctrl, PUZZLE_3D_PLAY_STORE_ID) ?? PUZZLE_3D_PLAY_IDLE_SNAPSHOT;
}

function puzzle3dPlaySelectionSnapshotKey(ctrl: Puzzle3dPlayShellController | undefined, bodyKey: string): string {
  if (!ctrl || !PUZZLE_3D_PLAY_SNAPSHOT_PANEL_BODY_KEYS.has(bodyKey)) {
    return "";
  }
  const snap = ctrl.getSnapshot();
  const selection = snap.selection;
  const hover = snap.hoverFocus;
  return `${selection.objectIds.join("\0")}\0${selection.vortexIds.join("\0")}\0${selection.attractionIds.join("\0")}\0${(selection.referenceIds ?? []).join("\0")}\0${(selection.targetVolumeIds ?? []).join("\0")}\0${hover.kindHover?.domain ?? ""}\0${hover.kindHover?.kindId ?? ""}`;
}

/** @emoji 🔔 Re-renders hierarchy/inspector panels on puzzle 3D selection without a shell generation bump. */
function usePuzzle3dPlaySnapshotPanelRefresh(bodyKey: string): void {
  const ctrl = usePuzzle3dPlayController();
  reactHostPort.useSyncExternalStore(
    (listener) => {
      if (!ctrl || !PUZZLE_3D_PLAY_SNAPSHOT_PANEL_BODY_KEYS.has(bodyKey)) {
        return () => {};
      }
      return ctrl.subscribeSnapshot(listener);
    },
    () => puzzle3dPlaySelectionSnapshotKey(ctrl, bodyKey),
    () => "",
  );
}

/** @emoji 💬 Enforces CAD-style puzzle 3D play engagement (command input row required). */
export function enforcePuzzle3dPlayWindowEngagement(engagement: WindowEngagement | undefined): void {
  if (!engagement) return;
  enforcePlaygroundWindowEngagementInput(engagement, "Puzzle 3D play viewport");
}

/** @emoji 💬 Mirrors live {@link EngagementSpec} into {@link WindowEngagement} with bus-routed engagement commands. */
export function puzzle3dPlayEngagementMirror(engagement: EngagementSpec | null): WindowEngagement | undefined {
  if (!engagement) return undefined;
  const options = engagement.options?.map((option) => ({
    id: option.id,
    label: option.label,
    pressed: option.pressed,
    disabled: option.disabled,
    command: { controllerId: PUZZLE_3D_PLAY_CONTROLLER_ID, command: "engagementOption", args: { optionId: option.id } },
  }));
  const input = engagement.input
    ? {
        id: engagement.input.id,
        value: engagement.input.value,
        placeholder: engagement.input.placeholder,
        disabled: engagement.input.disabled,
        onChange: { controllerId: PUZZLE_3D_PLAY_CONTROLLER_ID, command: "engagementInput", args: {} },
        onSubmit: { controllerId: PUZZLE_3D_PLAY_CONTROLLER_ID, command: "engagementSubmit", args: {} },
        onRepeatLast: engagement.input.onRepeatLast
          ? { controllerId: PUZZLE_3D_PLAY_CONTROLLER_ID, command: "engagementRepeatLast", args: {} }
          : undefined,
        onAbort: { controllerId: PUZZLE_3D_PLAY_CONTROLLER_ID, command: "engagementAbort", args: {} },
      }
    : undefined;
  const status = engagement.status?.map((row) => ({ id: row.id, text: typeof row.content === "string" ? row.content : String(row.content) }));
  const possibleEngagements = engagement.possibleEngagements?.map((row) => ({
    id: row.id,
    label: row.label,
    detail: row.detail,
    command: { controllerId: PUZZLE_3D_PLAY_CONTROLLER_ID, command: "engagementPossibleSelect", args: { possibleId: row.id } },
  }));
  const control = engagementSpecControlMirror(engagement.control, PUZZLE_3D_PLAY_CONTROLLER_ID, {});
  const controls = engagement.controls?.map((row) => engagementSpecControlMirror(row, PUZZLE_3D_PLAY_CONTROLLER_ID, {})).filter((row): row is WindowEngagementControl => row !== undefined);
  return { sessionActive: engagement.sessionActive, options, input, control, controls, status, possibleEngagements };
}

function Puzzle3dPlayEngagementPublisher(props: {
  readonly ctrl: Puzzle3dPlayShellController | undefined;
  readonly snap: Puzzle3dPlaySnapshot;
  readonly bus: CommandBus;
}): null {
  const { ctrl, snap, bus } = props;
  const kindCatalogs = reactHostPort.useMemo(() => parseKindCatalogs(snap.fixture.meta), [snap.fixture.meta]);
  const [cmdLine, setCmdLine] = reactHostPort.useState("");
  const [fillCount, setFillCount] = reactHostPort.useState(0);
  const fillSessionReadyEpoch = reactHostPort.useSyncExternalStore(
    subscribePuzzle3dFillSessionReady,
    getPuzzle3dFillSessionReadyEpoch,
    getPuzzle3dFillSessionReadyEpoch,
  );
  const engagementSpecRef = reactHostPort.useRef<EngagementSpec | null>(null);
  const brushEngagementEpoch = reactHostPort.useSyncExternalStore(subscribePuzzle3dBrushEngagementSource, getPuzzle3dBrushEngagementEpoch, getPuzzle3dBrushEngagementEpoch);
  const brushSource = puzzle3dBrushEngagementSourceRef.current;
  const selectionCount =
    snap.selection.objectIds.length +
    snap.selection.vortexIds.length +
    snap.selection.attractionIds.length +
    snap.selection.targetVolumeIds.length;
  const rememberEngagementRepeat = reactHostPort.useCallback(
    (key: string) => {
      bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "rememberEngagementRepeat", { key });
    },
    [bus],
  );
  const onSelectTool = reactHostPort.useCallback(() => {
    if (snap.activeTool === "fill") {
      const base = clearPuzzle3dFillSession();
      if (base && ctrl) {
        ctrl.patchFixture(() => structuredClone(base));
      }
      setFillCount(0);
    }
    bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "setActiveTool", { tool: "select" });
  }, [bus, ctrl, snap.activeTool]);
  const onBrushTool = reactHostPort.useCallback(() => {
    if (snap.activeTool === "fill") {
      const base = clearPuzzle3dFillSession();
      if (base && ctrl) {
        ctrl.patchFixture(() => structuredClone(base));
      }
      setFillCount(0);
    }
    bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "setActiveTool", { tool: "brush" });
  }, [bus, ctrl, snap.activeTool]);
  const onFillTool = reactHostPort.useCallback(() => {
    puzzle3dFillPendingCountRef.current = 0;
    setFillCount(0);
    bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "setActiveTool", { tool: "fill" });
  }, [bus]);
  const onFillCount = reactHostPort.useCallback(
    (count: number) => {
      const progress = puzzle3dFillBuildProgressRef.current;
      const maxAvailable = progress.done ? PUZZLE_3D_FILL_COUNT_MAX : progress.count;
      const prev = fillCount;
      const n = Math.max(0, Math.min(PUZZLE_3D_FILL_COUNT_MAX, Math.round(count), maxAvailable));
      puzzle3dFillPendingCountRef.current = n;
      setFillCount(n);
      bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "setFillCount", { count: n });
      if (n < prev) {
        const catalogs = parseKindCatalogs(snap.fixture.meta);
        const compatibility = parseKindCompatibility(snap.fixture.meta);
        rerollPuzzle3dFillTail(n, catalogs, compatibility, snap.brushPlacementOverlapBudget);
      }
    },
    [bus, fillCount, snap.brushPlacementOverlapBudget, snap.fixture.meta],
  );
  const fillAutoStartedRef = reactHostPort.useRef(false);
  reactHostPort.useEffect(() => {
    if (snap.activeTool !== "fill") {
      fillAutoStartedRef.current = false;
      return;
    }
    if (fillAutoStartedRef.current) {
      return;
    }
    const sequenceLength = puzzle3dFillSessionRef.current.sequence.length;
    if (sequenceLength === 0) {
      return;
    }
    fillAutoStartedRef.current = true;
    const pending = puzzle3dFillPendingCountRef.current;
    const nextCount = pending > 0 ? Math.min(pending, sequenceLength) : 1;
    puzzle3dFillPendingCountRef.current = nextCount;
    setFillCount(nextCount);
    bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "setFillCount", { count: nextCount });
  }, [bus, fillSessionReadyEpoch, snap.activeTool]);
  reactHostPort.useEffect(() => {
    if (snap.activeTool !== "fill") {
      return;
    }
    const progress = puzzle3dFillBuildProgressRef.current;
    if (progress.done) {
      return;
    }
    const maxAvailable = progress.count;
    if (fillCount <= maxAvailable) {
      return;
    }
    const capped = maxAvailable;
    puzzle3dFillPendingCountRef.current = capped;
    setFillCount(capped);
    bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "setFillCount", { count: capped });
  }, [bus, fillCount, fillSessionReadyEpoch, snap.activeTool]);
  const fillBuildProgress = puzzle3dFillBuildProgressRef.current;
  const onRepeatLastEngagement = reactHostPort.useCallback(() => {
    bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "engagementRepeatLast", {});
  }, [bus]);
  const onEngagementAbort = reactHostPort.useCallback(() => {
    setCmdLine("");
    if (snap.activeTool === "brush" || snap.activeTool === "fill") {
      onSelectTool();
    }
  }, [onSelectTool, snap.activeTool]);
  const onDeleteSelectedTargetVolume = reactHostPort.useCallback(() => {
    bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "deleteSelectedTargetVolume", {});
  }, [bus]);
  const onToggleFillEditTargetVolumes = reactHostPort.useCallback(() => {
    bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "setFillEditTargetVolumes", {});
  }, [bus]);
  const onVoxelBrushDimension = reactHostPort.useCallback(
    (axis: 0 | 1 | 2, value: number) => {
      bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "setVoxelBrushDimension", { axis, value });
    },
    [bus],
  );
  const onZoomToSelection = reactHostPort.useCallback(() => {
    requestPuzzle3dZoomToSelection(snap.selection);
  }, [snap.selection]);
  const onCmdLineSubmit = reactHostPort.useCallback(
    (value: string) => {
      const token = normalizeEngagementCommandText(value.trim());
      if (engagementCommandTokenEquals(token, "brush")) {
        onBrushTool();
        setCmdLine("");
        return;
      }
      if (engagementCommandTokenEquals(token, "fill")) {
        onFillTool();
        setCmdLine("");
        return;
      }
      if (engagementCommandTokenEquals(token, "select")) {
        onSelectTool();
        setCmdLine("");
        return;
      }
      if (engagementCommandTokenEquals(token, "zoom")) {
        rememberEngagementRepeat(PUZZLE_3D_ENGAGEMENT_ZOOM_ID);
        onZoomToSelection();
        setCmdLine("");
        return;
      }
      if (snap.activeTool === "brush") {
        const raw = value.trim();
        const idx = brushSource.candidates.findIndex(
          (candidate) => candidate.objectKindId === raw || engagementCommandTokenEquals(candidate.objectKindId, token),
        );
        if (idx >= 0) {
          const candidate = brushSource.candidates[idx]!;
          rememberEngagementRepeat(`puzzle3d.brush.${candidate.objectKindId}.${candidate.sourceVortexIndex}`);
          brushSource.pickCandidate(idx);
        }
      }
      setCmdLine("");
    },
    [brushSource, onBrushTool, onFillTool, onSelectTool, onZoomToSelection, rememberEngagementRepeat, snap.activeTool],
  );
  const spec = reactHostPort.useMemo(
    () =>
      buildPuzzle3dPlayEngagement({
        activeTool: snap.activeTool,
        cmdLine,
        fillCount,
        fillBuildProgress,
        fillEditTargetVolumes: snap.fillEditTargetVolumes,
        voxelBrushDimensions: snap.voxelBrushDimensions,
        selectedTargetVolumeCount: snap.selection.targetVolumeIds.length,
        selectionCount,
        onCmdLineChange: setCmdLine,
        onCmdLineSubmit,
        onRepeatLast: onRepeatLastEngagement,
        onAbort: onEngagementAbort,
        onSelectTool,
        onBrushTool,
        onFillTool,
        onFillCount,
        onToggleFillEditTargetVolumes,
        onDeleteSelectedTargetVolume,
        onVoxelBrushDimension,
        onCycleBrushCandidate: () => brushSource.cycleCandidate(),
        onPickBrushCandidate: (index) => {
          const candidate = brushSource.candidates[index];
          if (candidate) {
            rememberEngagementRepeat(`puzzle3d.brush.${candidate.objectKindId}.${candidate.sourceVortexIndex}`);
          }
          brushSource.pickCandidate(index);
        },
        onZoomToSelection,
        brushCandidates: brushSource.candidates,
        brushTargetActive: brushSource.targetActive,
        brushPlacementProbePending: brushSource.placementProbePending,
        kindCatalogs,
        sceneFixture: snap.fixture,
      }),
    [brushEngagementEpoch, brushSource, cmdLine, fillBuildProgress, fillCount, fillSessionReadyEpoch, kindCatalogs, onBrushTool, onCmdLineSubmit, onDeleteSelectedTargetVolume, onEngagementAbort, onFillCount, onFillTool, onRepeatLastEngagement, onSelectTool, onToggleFillEditTargetVolumes, onVoxelBrushDimension, onZoomToSelection, rememberEngagementRepeat, selectionCount, snap.activeTool, snap.fillEditTargetVolumes, snap.fixture, snap.selection.targetVolumeIds.length, snap.voxelBrushDimensions],
  );
  engagementSpecRef.current = spec;
  reactHostPort.useEffect(() => {
    const mirrored = puzzle3dPlayEngagementMirror(spec);
    enforcePuzzle3dPlayWindowEngagement(mirrored);
    ctrl?.setWindowEngagement(mirrored);
  }, [ctrl, spec]);
  reactHostPort.useLayoutEffect(() => {
    if (!ctrl) {
      return;
    }
    const bridge: Puzzle3dPlayHostBridge = {
      runHostCommand: (command, args) => {
        switch (command) {
          case "engagementOption": {
            const optionId = (args as { optionId?: string })?.optionId;
            engagementSpecRef.current?.options?.find((row) => row.id === optionId)?.onPress?.();
            break;
          }
          case "engagementInput": {
            const value = (args as { value?: string })?.value ?? "";
            engagementSpecRef.current?.input?.onChange?.(value);
            break;
          }
          case "engagementSubmit": {
            const value = (args as { value?: string })?.value ?? engagementSpecRef.current?.input?.value ?? "";
            engagementSpecRef.current?.input?.onSubmit?.(value);
            break;
          }
          case "engagementRepeatLast":
            engagementSpecRef.current?.input?.onRepeatLast?.();
            break;
          case "engagementAbort":
            engagementSpecRef.current?.input?.onAbort?.();
            break;
          case "engagementPossibleSelect": {
            const possibleId = (args as { possibleId?: string })?.possibleId;
            if (!possibleId) {
              break;
            }
            engagementSpecRef.current?.possibleEngagements?.find((row) => row.id === possibleId)?.onSelect?.();
            break;
          }
          case "engagementControlChange": {
            const value = (args as { value?: number; controlId?: string })?.value;
            const controlId = (args as { controlId?: string })?.controlId;
            const spec = engagementSpecRef.current;
            const control =
              controlId && spec?.controls?.length
                ? spec.controls.find((row) => row.id === controlId) ?? spec.control
                : spec?.control;
            if (value === undefined || !control || control.kind === "ring") break;
            control.onChange?.(value);
            break;
          }
          case "engagementControlCommit": {
            const value = (args as { value?: number; controlId?: string })?.value;
            const controlId = (args as { controlId?: string })?.controlId;
            const spec = engagementSpecRef.current;
            const control =
              controlId && spec?.controls?.length
                ? spec.controls.find((row) => row.id === controlId) ?? spec.control
                : spec?.control;
            if (value === undefined || !control || control.kind === "ring") break;
            control.onCommit?.(value);
            break;
          }
          case "engagementControlSelect": {
            const id = (args as { id?: string })?.id;
            const control = engagementSpecRef.current?.control;
            if (!id || !control || control.kind !== "ring") break;
            control.onSelect?.(id);
            break;
          }
          default:
            break;
        }
      },
    };
    ctrl.setHostBridge(bridge);
    return () => ctrl.setHostBridge(null);
  }, [ctrl]);
  return null;
}

/** @emoji 📷 Aligns shell viewport camera projection with a display-tree template on first paint. */
function puzzle3dPlayViewportCamera(base: CameraState, templateId?: string): CameraState {
  const view = templateId ? resolveOrbitCameraViewFromTemplateId(templateId) : null;
  if (!view) {
    return base;
  }
  const expectedProjection = orbitCameraProjectionForView(view);
  if ((base.projection ?? "perspective") === expectedProjection) {
    return base;
  }
  return computeOrbitCameraViewState(view, {
    target: base.target,
    distance: orbitCameraDistance({ ...base, projection: base.projection ?? "perspective" }),
    zoom: base.zoom,
  });
}

const Puzzle3dPlayViewportHost = reactHostPort.memo(function Puzzle3dPlayViewportHost({ node }: { readonly node: UiPuzzle3dHostSurfaceNode }): React.ReactElement {
  const { runtime } = useApp();
  const bus = runtime.commandBus;
  const ctrl = usePuzzle3dPlayController();
  const snap = usePuzzle3dPlaySnapshot();
  const shellInstance = useShellWindowInstance();
  const viewportCamera = reactHostPort.useMemo(() => {
    const base = ctrl?.cameraForInstance(shellInstance?.instanceId) ?? snap.fixture?.camera;
    if (!base) {
      return { position: [420, -420, 320] as const, target: [0, 0, 40] as const, zoom: 1 };
    }
    return puzzle3dPlayViewportCamera(base, shellInstance?.templateId);
  }, [ctrl, shellInstance?.instanceId, shellInstance?.templateId, snap.cameraSeedEpoch, snap.fixture?.camera]);
  reactHostPort.useLayoutEffect(() => {
    if (!ctrl || !shellInstance?.instanceId || !shellInstance.templateId) {
      return;
    }
    const view = resolveOrbitCameraViewFromTemplateId(shellInstance.templateId);
    if (!view) {
      return;
    }
    const current = ctrl.cameraForInstance(shellInstance.instanceId);
    if ((current.projection ?? "perspective") === orbitCameraProjectionForView(view)) {
      return;
    }
    ctrl.run(ORBIT_CAMERA_VIEW_COMMAND, { view, instanceId: shellInstance.instanceId });
  }, [ctrl, shellInstance?.instanceId, shellInstance?.templateId]);
  const cameraSeedKey = shellInstance ? `${shellInstance.instanceId}:${snap.cameraSeedEpoch}` : snap.cameraSeedEpoch;
  const engagementPublisher =
    ctrl && node.controllerId === PUZZLE_3D_PLAY_CONTROLLER_ID ? (
      <Puzzle3dPlayEngagementPublisher ctrl={ctrl} snap={snap} bus={bus} />
    ) : null;
  if (node.controllerId !== PUZZLE_3D_PLAY_CONTROLLER_ID) {
    return (
      <>
        {engagementPublisher}
        <div className="p-2 text-xs text-muted-foreground">Invalid puzzle 3D viewport binding</div>
      </>
    );
  }
  if (!snap.fixture) {
    return (
      <>
        {engagementPublisher}
        <div className="p-4 text-destructive">Invalid puzzle 3D fixture</div>
      </>
    );
  }
  const kindCompatibility = reactHostPort.useMemo(() => parseKindCompatibility(snap.fixture.meta), [snap.fixture]);
  const kindCatalogs = reactHostPort.useMemo(() => parseKindCatalogs(snap.fixture.meta), [snap.fixture]);
  reactHostPort.useEffect(() => {
    installPuzzle3dPlayBrushHost(snap.fixture.meta as Record<string, unknown> | undefined);
  }, [snap.fixture.meta]);
  const blockedVortexFullIds = reactHostPort.useMemo(() => blockedVortexFullIdsFromAttractions(snap.fixture.attractions), [snap.fixture]);
  const patchFixture = reactHostPort.useCallback(
    (updater: (prev: Fixture) => Fixture) => {
      ctrl?.patchFixture(updater);
    },
    [ctrl],
  );
  const onRelocatePersist = reactHostPort.useCallback(
    (payload: RelocatePayload, attractingByObjectId: ReadonlyMap<string, readonly string[]>) => {
      ctrl?.patchRelocate(payload, attractingByObjectId);
    },
    [ctrl],
  );
  const onReferenceRelocatePersist = reactHostPort.useCallback(
    (payload: import("@semio-tech/infinite-world-r3f").WorldReferenceRelocatePayload) => {
      ctrl?.patchReferenceRelocate(payload);
    },
    [ctrl],
  );
  const onTargetVolumeRelocatePersist = reactHostPort.useCallback(
    (payload: import("@semio-tech/infinite-world-r3f").WorldVolumeRelocatePayload) => {
      ctrl?.patchTargetVolumeRelocate(payload);
    },
    [ctrl],
  );
  const onVoxelBrushPaintPersist = reactHostPort.useCallback(
    (cad: import("../react/index.tsx").Vec3, scale: import("../react/index.tsx").Vec3) => {
      ctrl?.run("paintVoxel", { cad, scale });
    },
    [ctrl],
  );
  const proximityRelocateEnabled = snap.fixture.attractions.length > 0;
  const onCanvasHover = reactHostPort.useCallback(
    (payload: Puzzle3dHoverPayload) => {
      console.log("[DEBUG] puzzle3d hover", payload.kindHover?.domain, payload.kindHover?.kindId, payload.hoverTarget?.kind);
      ctrl?.setHoverFocus(payload);
    },
    [ctrl],
  );
  const handleFixtureDrop = reactHostPort.useCallback(
    (detail: Puzzle3dFixtureDropDetail) => {
      const result = resolvePuzzle3dFixtureDrop(detail, kindCatalogs, snap.fixture);
      if (result.kind === "palette-object") {
        patchFixture((fixture) => applyPaletteObjectDropToFixture(fixture, result.object));
        bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "setSelection", {
          selection: { objectIds: [result.object.id], vortexIds: [], attractionIds: [] },
        });
        return;
      }
      if (result.kind === "replace-fixture") {
        ctrl?.patchFixture(() => result.fixture);
      }
    },
    [bus, ctrl, kindCatalogs, patchFixture, snap.fixture],
  );
  const fillBaseCaptureRef = reactHostPort.useRef<Fixture | null>(null);
  const prevActiveToolRef = reactHostPort.useRef(snap.activeTool);
  reactHostPort.useLayoutEffect(() => {
    const prev = prevActiveToolRef.current;
    prevActiveToolRef.current = snap.activeTool;
    if (snap.activeTool === "fill" && prev !== "fill") {
      fillBaseCaptureRef.current = structuredClone(snap.fixture);
    }
    if (snap.activeTool !== "fill") {
      fillBaseCaptureRef.current = null;
    }
  }, [snap.activeTool, snap.fixture]);
  const fillPrepareTimerRef = reactHostPort.useRef<ReturnType<typeof setTimeout> | null>(null);
  const fillSessionPreparedRef = reactHostPort.useRef(false);
  const [fillDistributionEpoch, setFillDistributionEpoch] = reactHostPort.useState(0);
  const [fillTargetVolumesEpoch, setFillTargetVolumesEpoch] = reactHostPort.useState(0);
  reactHostPort.useEffect(() => {
    if (snap.activeTool !== "fill") {
      fillSessionPreparedRef.current = false;
    }
  }, [snap.activeTool]);
  reactHostPort.useEffect(
    () =>
      subscribePuzzle3dFillDistributionInvalidated(() => {
        fillSessionPreparedRef.current = false;
        setFillDistributionEpoch((epoch) => epoch + 1);
      }),
    [],
  );
  reactHostPort.useEffect(
    () =>
      subscribePuzzle3dFillTargetVolumesInvalidated(() => {
        fillSessionPreparedRef.current = false;
        setFillTargetVolumesEpoch((epoch) => epoch + 1);
      }),
    [],
  );
  const fillToleranceRef = reactHostPort.useRef(snap.brushPlacementOverlapBudget);
  reactHostPort.useEffect(() => {
    if (fillToleranceRef.current === snap.brushPlacementOverlapBudget) {
      return;
    }
    fillToleranceRef.current = snap.brushPlacementOverlapBudget;
    if (snap.activeTool !== "fill") {
      return;
    }
    fillSessionPreparedRef.current = false;
  }, [snap.activeTool, snap.brushPlacementOverlapBudget]);
  const onFillMeshesReady = reactHostPort.useCallback(() => {
    if (fillPrepareTimerRef.current !== null) {
      clearTimeout(fillPrepareTimerRef.current);
    }
    fillPrepareTimerRef.current = setTimeout(() => {
      fillPrepareTimerRef.current = null;
      const base = fillBaseCaptureRef.current;
      if (!base) {
        return;
      }
      if (!fillSessionPreparedRef.current) {
        preparePuzzle3dFillSession(base, kindCatalogs, kindCompatibility, snap.brushPlacementOverlapBudget, base.targetVolumes ?? []);
        fillSessionPreparedRef.current = true;
      }
    }, 0);
  }, [bus, kindCatalogs, kindCompatibility, snap.brushPlacementOverlapBudget]);
  reactHostPort.useEffect(() => {
    if (snap.activeTool !== "fill" || !fillBaseCaptureRef.current) {
      return;
    }
    if (fillSessionPreparedRef.current) {
      return;
    }
    onFillMeshesReady();
  }, [fillDistributionEpoch, fillTargetVolumesEpoch, onFillMeshesReady, snap.activeTool, snap.brushPlacementOverlapBudget]);
  reactHostPort.useEffect(
    () => () => {
      if (fillPrepareTimerRef.current !== null) {
        clearTimeout(fillPrepareTimerRef.current);
      }
    },
    [],
  );
  return (
    <>
      {engagementPublisher}
      <div className="absolute inset-0 min-h-0 min-w-0">
      <ObjectStateProvider
        fixture={snap.fixture}
        fixtureRevision={snap.fixtureRevision}
        onConnect={(payload) => {
          patchFixture((fixture) => applyConnectToFixture(fixture, payload));
          bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "noteConnect");
        }}
        onRelocate={onRelocatePersist}
      >
        <PlayCanvas
          fixture={snap.fixture}
          camera={viewportCamera}
          cameraSeedKey={cameraSeedKey}
          proximityRelocateEnabled={proximityRelocateEnabled}
          kindCatalogs={kindCatalogs}
          kindCompatibility={kindCompatibility}
          blockedVortexFullIds={blockedVortexFullIds}
          lodTag={snap.lodTag}
          lodProps={snap.lodProps}
          gumballConfig={snap.gumballConfig}
          selection={snap.selection}
          selectedId={snap.selectedId}
          selectedLabel={snap.selectedLabel}
          selectionMode={snap.selectionMode}
          selectionMethod={snap.selectionMethod}
          marqueeSelectableKinds={
            snap.fillEditTargetVolumes
              ? { object: false, vortex: false, attraction: false }
              : snap.selectableKinds
          }
          proximityRadius={snap.proximityRadius}
          chunkSize={snap.chunkSize}
          gridFactor={snap.gridFactor}
          showLodGrid={snap.showLodGrid}
          gridSnapEnabled={snap.gridSnapEnabled}
          hoverTarget={snap.hoverFocus.hoverTarget}
          kindHover={snap.hoverFocus.kindHover}
          onHover={onCanvasHover}
          setSelectedId={(id) => bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "setSelectedId", { id })}
          onSelect={(selection) => bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "noteSelection", selection)}
          onReferenceRelocate={onReferenceRelocatePersist}
          onTargetVolumeRelocate={onTargetVolumeRelocatePersist}
          onVoxelBrushPaint={onVoxelBrushPaintPersist}
          fillEditTargetVolumes={snap.fillEditTargetVolumes}
          voxelBrushDimensions={snap.voxelBrushDimensions}
          onToggleSelectionHidden={(value) => bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "setSelectionFlag", { flag: "hidden", value })}
          onToggleSelectionLocked={(value) => bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "setSelectionFlag", { flag: "locked", value })}
          onDeleteSelection={() => bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "deleteSelection")}
          onDuplicateSelection={() => bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "duplicateSelection")}
          onSelectSameKind={() => bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "selectSameKind")}
          onIndirectConnect={() => bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "noteIndirect")}
          onProximityConnect={() => bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "noteProximity")}
          onLodChange={(lod) => bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "setEffectiveLod", { lod })}
          onCamera={(camera) => ctrl?.setCamera(camera, shellInstance?.instanceId)}
          onAttractionCompatibleObjects={() => bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "noteCompatibleObjects")}
          onAttractionTargetRing={() => bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "noteTargetRing")}
          brushActive={snap.activeTool === "brush"}
          fillActive={snap.activeTool === "fill"}
          onFillMeshesReady={onFillMeshesReady}
          onBrushPlace={(payload) => bus.dispatch(PUZZLE_3D_PLAY_CONTROLLER_ID, "addBrushObject", payload)}
          brushPlacementOverlapBudget={snap.brushPlacementOverlapBudget}
          fixtureDragDrop
          onFixtureDrop={handleFixtureDrop}
        />
      </ObjectStateProvider>
      <div data-puzzle3d-play-probe className="pointer-events-none absolute left-0 top-0 select-none opacity-0" aria-hidden>
        <span data-e2e-selected>{snap.selectedLabel ?? "none"}</span>
        <span data-e2e-scene-lod>{snap.lodTag}</span>
        <span data-e2e-proximity-count>{snap.proximityCount}</span>
        <span data-e2e-connect-count>{snap.connectCount}</span>
        <span data-e2e-indirect-count>{snap.indirectCount}</span>
      </div>
    </div>
    </>
  );
}, (prev, next) => prev.node.surfaceId === next.node.surfaceId && prev.node.controllerId === next.node.controllerId);

function Puzzle3dPlayJackSurfaceHost({ node: _node }: { readonly node: import("@semio-tech/framework-platform-core").UiWriterHostSurfaceNode }): React.ReactElement {
  const ctrl = usePuzzle3dPlayController();
  void ctrl?.getHoverEpoch();
  void ctrl?.getSelectEpoch();
  const document = ctrl?.getWriterDocumentJack() ?? createWriterDocument({ id: "puzzle-3d-jack", languageId: "jack", text: "" });
  const onHoverChange = reactHostPort.useCallback((offset: number | null) => {
    ctrl?.run("setJackHover", { offset });
  }, [ctrl]);
  const onSelectionChange = reactHostPort.useCallback((range: { start: number; end: number }) => {
    ctrl?.run("setJackSelect", range);
  }, [ctrl]);
  return (
    <WriterCanvas
      document={document}
      className="h-full"
      onHoverChange={onHoverChange}
      onSelectionChange={onSelectionChange}
      externalHoverOccurrences={ctrl?.getJackHoverOccurrences()}
      externalHoverOccurrencesSignal={ctrl?.getHoverEpoch()}
      externalSelectionOccurrences={ctrl?.getJackSelectOccurrences()}
      externalSelectionOccurrencesSignal={ctrl?.getSelectEpoch()}
    />
  );
}

let puzzle3dPlayChromeRegistered = false;

/** @emoji 🧊 Registers puzzle 3D play surface host, tab icons, and mesh preload. */
export function registerPuzzle3dPlaySurfaceHosts(): void {
  if (puzzle3dPlayChromeRegistered) return;
  puzzle3dPlayChromeRegistered = true;
  registerUiPuzzle3dSurfaceHost(PUZZLE_3D_PLAY_VIEWPORT_SURFACE_ID, Puzzle3dPlayViewportHost);
  registerUiWriterSurfaceHost(PUZZLE_3D_PLAY_SURFACE_ID_JACK, Puzzle3dPlayJackSurfaceHost);
  registerWindowBody(PUZZLE_3D_PLAY_BODY_KEY_JACK, () =>
    buildWriterWindowBody(PUZZLE_3D_PLAY_SURFACE_ID_JACK, PUZZLE_3D_PLAY_CONTROLLER_ID, PUZZLE_3D_PLAY_WINDOW_KIND_JACK));
  registerTabIcon(PUZZLE_3D_PLAY_ICON_INSPECTOR, "clipboard-list");
  registerTabIcon(PUZZLE_3D_PLAY_ICON_KINDS, "tags");
  registerTabIcon(PUZZLE_3D_PLAY_ICON_HIERARCHY, "list-tree");
  registerTabIcon(PUZZLE_3D_PLAY_ICON_SETTINGS, "settings");
  const fixture = parseFixture(puzzle3dPlayFixtureJson(playgroundResolvedExampleId(PUZZLE_3D_PLAY_EXAMPLE_CONCRETE_FOREST_ID)) as unknown);
  if (fixture) {
    const catalogs = parseKindCatalogs(fixture.meta as Record<string, unknown> | undefined);
    const compatibility = parseKindCompatibility(fixture.meta as Record<string, unknown> | undefined);
    for (const url of brushMeshUrlsForFillSession(fixture, catalogs, compatibility)) {
      if (isLoadableMeshUrl(url)) {
        sceneHostPort.drei.useGLTF.preload(url);
      }
    }
  }
}

/** @emoji 🚀 Mounts puzzle 3d play via standard {@link PlaygroundView} (bodies registered in {@link Playground3d}). */
export function mountPuzzle3dPlayChrome(playground: Playground, rootId = "root"): void {
  mountPlaygroundApp(
    <PlaygroundView runtime={playground.runtime} defaultAppId={PUZZLE_3D_PLAY_APP_ID} playgroundKeybindings={playground.keybindings} />,
    rootId,
  );
}

const puzzle3dPlayChromeBoot: PlaygroundChromeBoot = {
  registerHosts: registerPuzzle3dPlaySurfaceHosts,
  mount: mountPuzzle3dPlayChrome,
};

/** @emoji 🛝 Puzzle 3D play entry: register hosts, bodies, mount chrome (from `puzzle/3d/play/index.ts`). */
export function bootPuzzle3dPlay(playground: Playground, rootId = "root"): void {
  bootPlayground(playground, puzzle3dPlayChromeBoot, rootId);
}
//#endregion 🔖Puzzle3dPlayHost