/** 🔸️ The handle component hover phase of useWorldHoverAndPlacement. */
import { useCallback, useEffect } from "../../../⚛️react-port/🟦️.ts";
import { type MergeMode } from "@semio-tech/framework";
import { world3dComponentInteractionTarget, world3dMarkerInteractionTarget } from "../../🔗️instance-selection/🟦️.ts";
import { MARQUEE_DRAG_THRESHOLD_PX, raycastGroundPoint, resolveWorldMergeMode } from "../../🎯️marquee-and-bridges/🟦️.tsx";
import { world3dVolumeBrushOriginV1 } from "../../🗂️catalogue-drop/🟦️.tsx";
import { world3dSelectionActionArgs, world3dSelectionTargetsActionArgs } from "../../🖱️interaction-domain-args/🟦️.ts";
import type { useActiveUtilityState } from "../🔶️active-utility/🟦️.ts";

/** 🔹 Builds handleComponentHover, handleVortexHover, handleVortexSelect, handleConnectDragStart of useWorldHoverAndPlacement. */
export function useHandleComponentHoverState({ interactionDomainId, dispatchComponentHover, dispatchInstanceHover, dispatch, hoveredVortexFullIdRef, dispatchVortexHover, selection, persistentSelectionMode, vortices, setVortexPointerArm, setConnectDragSource, setConnectDragHoverPosition, connectDropConsumedRef, hostRef, cameraRef, gridFactor, volumeBrushMode, setVoxelHoverOrigin, instancesRef, meshesRef }: Pick<ReturnType<typeof useActiveUtilityState>, "interactionDomainId" | "dispatchComponentHover" | "dispatchInstanceHover" | "dispatch" | "hoveredVortexFullIdRef" | "dispatchVortexHover" | "selection" | "persistentSelectionMode" | "vortices" | "setVortexPointerArm" | "setConnectDragSource" | "setConnectDragHoverPosition" | "connectDropConsumedRef" | "hostRef" | "cameraRef" | "gridFactor" | "volumeBrushMode" | "setVoxelHoverOrigin" | "instancesRef" | "meshesRef">) {
  const handleComponentHover = useCallback(
    (args: { objectId: string; mode: string; id: number } | null) => {
      if (interactionDomainId) {
        if (args && args.mode !== "mesh" && args.mode !== "object") dispatchComponentHover(args);
        else dispatchInstanceHover(args?.objectId ?? null);
        return;
      }
      if (!args) {
        dispatch("setHover", {});
        return;
      }
      dispatch("setHover", args);
    },
    [dispatch, dispatchComponentHover, dispatchInstanceHover, interactionDomainId],
  );

  const handleVortexHover = useCallback(
    (fullId: string | null) => {
      if (fullId) hoveredVortexFullIdRef.current = fullId;
      if (fullId) dispatchVortexHover(fullId);
    },
    [dispatchVortexHover],
  );

  const handleVortexSelect = useCallback(
    (fullId: string, event?: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => {
      const merge = resolveWorldMergeMode(selection.selectionMergeMode, event ?? {}, persistentSelectionMode);
      if (interactionDomainId) {
        const target = world3dMarkerInteractionTarget("vortex", fullId, vortices.find((entry) => entry.fullId === fullId));
        dispatch("interactionSelect", world3dSelectionActionArgs(interactionDomainId, target.granularity, [target.id], merge));
        return;
      }
      dispatch("worldVortexSelect", { fullId, merge });
    },
    [dispatch, interactionDomainId, persistentSelectionMode, selection.selectionMergeMode, vortices],
  );

  const handleConnectDragStart = useCallback(
    (fullId: string, position: readonly [number, number, number]) => {
      setVortexPointerArm(null);
      const record = vortices.find((vortex) => vortex.fullId === fullId);
      setConnectDragSource({ fullId, position, record });
      setConnectDragHoverPosition(position);
    },
    [vortices],
  );

  const handleVortexPointerArm = useCallback(
    (arm: {
      readonly fullId: string;
      readonly position: readonly [number, number, number];
      readonly clientX: number;
      readonly clientY: number;
      readonly event: { readonly shiftKey?: boolean; readonly ctrlKey?: boolean; readonly metaKey?: boolean };
    }) => {
      setVortexPointerArm(arm);
    },
    [],
  );

  const handleVortexPointerMove = useCallback(
    (fullId: string, clientX: number, clientY: number) => {
      setVortexPointerArm((arm) => {
        if (!arm || arm.fullId !== fullId) return arm;
        const distance = Math.hypot(clientX - arm.clientX, clientY - arm.clientY);
        if (distance > MARQUEE_DRAG_THRESHOLD_PX) {
          handleConnectDragStart(arm.fullId, arm.position);
          return null;
        }
        return arm;
      });
    },
    [handleConnectDragStart],
  );

  const handleVortexPointerUp = useCallback(
    (fullId: string, event?: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => {
      setVortexPointerArm((arm) => {
        if (arm && arm.fullId === fullId) {
          handleVortexSelect(fullId, arm.event ?? event);
        }
        return null;
      });
    },
    [handleVortexSelect],
  );

  const handleConnectDragHover = useCallback((position: readonly [number, number, number]) => {
    setConnectDragHoverPosition(position);
  }, []);

  const handleConnectDragDrop = useCallback(
    (targetFullId: string, event?: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => {
      connectDropConsumedRef.current = true;
      setConnectDragSource((source) => {
        if (source) {
          if (source.fullId === targetFullId) {
            handleVortexSelect(targetFullId, event);
          } else {
            dispatch("createAttraction", { attracting: source.fullId, attracted: targetFullId });
          }
        }
        return null;
      });
      setConnectDragHoverPosition(null);
    },
    [dispatch, handleVortexSelect],
  );

  const handleConnectDragCancel = useCallback(() => {
    setVortexPointerArm(null);
    setConnectDragSource(null);
    setConnectDragHoverPosition(null);
  }, []);

  const handleVoxelPlace = useCallback(
    (origin: readonly [number, number, number]) => {
      dispatch("addTargetVolume", { origin });
    },
    [dispatch],
  );

  /** 🧊️ The Volume Brush's ground read, from the host's own pointer coordinates — one raycast shared by
   * the hover ghost and the Alt+click commit, so the box the user sees is exactly the volume that lands. */
  const voxelGroundOriginAt = useCallback(
    (clientX: number, clientY: number): [number, number, number] | null => {
      const host = hostRef.current;
      const camera = cameraRef.current;
      if (!host || !camera) return null;
      return world3dVolumeBrushOriginV1(raycastGroundPoint(clientX, clientY, host.getBoundingClientRect(), camera), gridFactor);
    },
    [gridFactor],
  );

  useEffect(() => {
    if (!volumeBrushMode) setVoxelHoverOrigin(null);
  }, [volumeBrushMode]);

  /** ✅️ A brush click places the current free candidate the brush suggestions run found for the vortex under the
   * pointer — the one-shot `acceptSuggestion`, which answers with a notice when nothing free is known yet. */
  const handleBrushPlace = useCallback(() => {
    const fullId = hoveredVortexFullIdRef.current;
    if (fullId) dispatch("acceptSuggestion", { fullId });
  }, [dispatch]);

  const handleWorldPick = useCallback(
    (args: { granularity: string; id: number; merge: string; objectId?: string }) => {
      if (interactionDomainId && args.objectId != null && args.id != null) {
        const target = world3dComponentInteractionTarget(instancesRef.current, meshesRef.current, args.objectId, args.granularity, args.id);
        dispatch("interactionSelect", world3dSelectionTargetsActionArgs(interactionDomainId, target ? [target] : [], args.merge as MergeMode));
        return;
      }
      dispatch("worldPick", args);
    },
    [dispatch, interactionDomainId],
  );
  return { handleBrushPlace, handleComponentHover, handleConnectDragCancel, handleConnectDragDrop, handleConnectDragHover, handleVortexHover, handleVortexPointerArm, handleVortexPointerMove, handleVortexPointerUp, handleVortexSelect, handleVoxelPlace, handleWorldPick, voxelGroundOriginAt };
}
