/** 🔶️ The active utility phase of useWorldHoverAndPlacement. */
import { useCallback, useEffect, useMemo, useRef } from "../../../⚛️react-port/🟦️.ts";
import { isContextMenuPointerTarget } from "@semio-tech/ui-react";
import { createCoalescingActionDispatcher } from "../../../🛠️ShellHelpers/🛑️extension-io/🟦️.ts";
import { leftoverBrushRetainGuestHoverV1, leftoverWorldWindowOverlayV1, WORLD3D_DEFAULT_MARKER_GRANULARITY, world3dComponentInteractionTarget, world3dInstanceInteractionTarget, world3dMarkerInteractionTarget, world3dSuggestionsAltHeld, world3dSuggestionsGestureArmed, type World3dSuggestionsRightDownRoute, world3dSuggestionsRightDownRoutes } from "../../🔗️instance-selection/🟦️.ts";
import { world3dGumballSelectionArgsV1 } from "../../🧮️gumball-math/🟦️.ts";
import { resolveWorldMergeMode } from "../../🎯️marquee-and-bridges/🟦️.tsx";
import { world3dHoverActionArgs, world3dSelectionTargetsActionArgs } from "../../🖱️interaction-domain-args/🟦️.ts";
import type { WorldCore3 } from "../../🏗️brush-mesh-and-menus/🟦️.ts";

/** 🔷 Builds activeUtility, altHeldRef, brushMode, cameraRef of useWorldHoverAndPlacement. */
export function useActiveUtilityState({ core }: { readonly core: WorldCore3 }) {
  const { activeUtility, altHeldRef, brushMode, cameraRef, connectDropConsumedRef, dispatch, dispatchSettled, gridFactor, hostRef, hoveredInstanceInteractionTargetRef, hoveredVortexFullIdRef, instances, instancesRef, interactionDomainId, interactionGranularity, meshesRef, persistentSelectionMode, selection, selectionMode, setConnectDragHoverPosition, setConnectDragSource, setVortexPointerArm, setVoxelHoverOrigin, suggestionMenuOpen, suggestionMenuOwnsThisWindow, volumeBrushMode, vortices, vorticesRef, windowInstanceId } = core;
  const handleWorldOrbitRightPointerDown = useCallback(
    (event: PointerEvent) => {
      const alt = world3dSuggestionsAltHeld(event.altKey, altHeldRef.current);
      const brushArmed = Boolean(brushMode && hoveredVortexFullIdRef.current);
      if (world3dSuggestionsGestureArmed(alt, hoveredVortexFullIdRef.current) || brushArmed) {
        setVortexPointerArm(null);
        setConnectDragSource(null);
        setConnectDragHoverPosition(null);
        dispatch("openVortexSuggestions", { fullId: hoveredVortexFullIdRef.current, x: event.clientX, y: event.clientY, windowId: windowInstanceId ?? undefined });
        return false;
      }
      return true;
    },
    [brushMode, dispatch, windowInstanceId],
  );
  const suggestionsRightDownDispatchedAtRef = useRef(0);
  useEffect(() => {
    const route: World3dSuggestionsRightDownRoute = {
      host: () => hostRef.current,
      handle: (event) => {
        const allowOrbit = handleWorldOrbitRightPointerDown(event);
        if (allowOrbit === false) suggestionsRightDownDispatchedAtRef.current = performance.now();
        return allowOrbit;
      },
    };
    world3dSuggestionsRightDownRoutes.add(route);
    return () => {
      world3dSuggestionsRightDownRoutes.delete(route);
    };
  }, [handleWorldOrbitRightPointerDown]);

  const handleSuggestionClose = useCallback(() => {
    setVortexPointerArm(null);
    setConnectDragSource(null);
    setConnectDragHoverPosition(null);
    dispatch("closeVortexSuggestions");
  }, [dispatch]);

  useEffect(() => {
    if (!suggestionMenuOpen || suggestionMenuOwnsThisWindow) return undefined;
    const handlePointerDown = (event: globalThis.PointerEvent): void => {
      if (isContextMenuPointerTarget(event.target)) return;
      handleSuggestionClose();
    };
    const handleKeyDown = (event: globalThis.KeyboardEvent): void => {
      if (event.key === "Escape") handleSuggestionClose();
    };
    window.addEventListener("pointerdown", handlePointerDown);
    window.addEventListener("keydown", handleKeyDown);
    return () => {
      window.removeEventListener("pointerdown", handlePointerDown);
      window.removeEventListener("keydown", handleKeyDown);
    };
  }, [handleSuggestionClose, suggestionMenuOpen, suggestionMenuOwnsThisWindow]);

  const selectionArgs = useCallback(() => world3dGumballSelectionArgsV1(selection), [selection]);

  const handleInstancePointerDown = useCallback(
    (id: string, index: number, event: { shiftKey: boolean; ctrlKey: boolean; metaKey: boolean }) => {
      const merge = resolveWorldMergeMode(selection.selectionMergeMode, event, persistentSelectionMode);
      const record = instances.find((entry) => entry.id === id);
      if (record?.disabled) {
        dispatch("worldPick", { granularity: selectionMode, id: null, merge });
        return;
      }
      if (interactionDomainId) {
        dispatch("interactionSelect", world3dSelectionTargetsActionArgs(interactionDomainId, [world3dInstanceInteractionTarget(instances, id, interactionGranularity)], merge));
        return;
      }
      if (selectionMode === "mesh" || selectionMode === "object") {
        dispatch("worldPick", { granularity: "mesh", id: index, merge });
        return;
      }
      dispatch("worldSelect", {
        ids: [id],
        merge,
      });
    },
    [dispatch, instances, interactionDomainId, interactionGranularity, persistentSelectionMode, selection.selectionMergeMode, selectionMode],
  );

  const dispatchInstanceHover = useMemo(
    () =>
      createCoalescingActionDispatcher<string | null>((id) => {
        if (interactionDomainId) {
          if (id == null && leftoverBrushRetainGuestHoverV1(activeUtility, leftoverWorldWindowOverlayV1(windowInstanceId))) return undefined;
          if (id == null) return dispatchSettled("interactionHover", world3dHoverActionArgs(interactionDomainId, interactionGranularity, null));
          const target = world3dInstanceInteractionTarget(instancesRef.current, id, interactionGranularity);
          return dispatchSettled("interactionHover", world3dHoverActionArgs(interactionDomainId, target.granularity, target.id));
        }
        if (id == null) return dispatchSettled("setHover", {});
        return dispatchSettled("setHover", { objectId: id, mode: "mesh", id: 0 });
      }),
    [activeUtility, dispatchSettled, interactionDomainId, interactionGranularity, windowInstanceId],
  );

  const dispatchVortexHover = useMemo(
    () =>
      createCoalescingActionDispatcher<string | null>((fullId) => {
        if (interactionDomainId) {
          if (!fullId && leftoverBrushRetainGuestHoverV1(activeUtility, leftoverWorldWindowOverlayV1(windowInstanceId))) return undefined;
          const target = fullId ? world3dMarkerInteractionTarget("vortex", fullId, vorticesRef.current.find((entry) => entry.fullId === fullId)) : null;
          const args = world3dHoverActionArgs(interactionDomainId, target?.granularity ?? WORLD3D_DEFAULT_MARKER_GRANULARITY.vortex, target?.id);
          return dispatchSettled("interactionHover", args);
        }
        if (!fullId) return dispatchSettled("worldVortexHover", {});
        return dispatchSettled("worldVortexHover", { fullId });
      }),
    [activeUtility, dispatchSettled, interactionDomainId, windowInstanceId],
  );

  const localHoverReleaseListeners = useRef(new Set<() => void>());
  const subscribeLocalHoverRelease = useCallback((listener: () => void) => {
    localHoverReleaseListeners.current.add(listener);
    return () => {
      localHoverReleaseListeners.current.delete(listener);
    };
  }, []);
  const releaseLocalHover = useCallback(() => {
    hoveredInstanceInteractionTargetRef.current = null;
    for (const listener of localHoverReleaseListeners.current) listener();
  }, []);
  const publishHoverPaint = useCallback((id: string | null) => {
    const root = hostRef.current;
    if (!root) return;
    if (id === null) root.removeAttribute("data-hover-paint-id");
    else root.setAttribute("data-hover-paint-id", id);
  }, []);

  const handleInstancePointerMove = useCallback(
    (id: string | null) => {
      hoveredInstanceInteractionTargetRef.current = id == null ? null : world3dInstanceInteractionTarget(instancesRef.current, id, interactionGranularity);
      dispatchInstanceHover(id);
    },
    [dispatchInstanceHover, interactionGranularity],
  );

  const dispatchComponentHover = useMemo(
    () =>
      createCoalescingActionDispatcher<{ objectId: string; mode: string; id: number }>((args) => {
        const target = world3dComponentInteractionTarget(instancesRef.current, meshesRef.current, args.objectId, args.mode, args.id);
        return dispatchSettled("interactionHover", world3dHoverActionArgs(interactionDomainId ?? "", target?.granularity ?? args.mode, target?.id ?? null));
      }),
    [dispatchSettled, interactionDomainId],
  );
  return { interactionDomainId, dispatchComponentHover, dispatchInstanceHover, dispatch, hoveredVortexFullIdRef, dispatchVortexHover, selection, persistentSelectionMode, vortices, setVortexPointerArm, setConnectDragSource, setConnectDragHoverPosition, connectDropConsumedRef, hostRef, cameraRef, gridFactor, volumeBrushMode, setVoxelHoverOrigin, instancesRef, meshesRef, handleInstancePointerDown, handleInstancePointerMove, handleSuggestionClose, handleWorldOrbitRightPointerDown, publishHoverPaint, releaseLocalHover, selectionArgs, subscribeLocalHoverRelease, suggestionsRightDownDispatchedAtRef };
}
