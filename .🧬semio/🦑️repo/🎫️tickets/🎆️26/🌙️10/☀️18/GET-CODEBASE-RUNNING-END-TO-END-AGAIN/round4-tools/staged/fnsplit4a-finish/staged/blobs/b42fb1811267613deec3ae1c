import type { InkCore2 } from "../🎬️ink-gestures/🟦️.ts";
import { useAbortGestureState } from "./🔶️abort-gesture/🟦️.ts";
import { useHandlePointerMoveState } from "./🔸️handle-pointer-move/🟦️.ts";
import { useHandleWheelState } from "./🟠️handle-wheel/🟦️.ts";

// #region InkCanvasHost
/** 👆️ Builds the pointer, wheel and double click handlers of the ink canvas. */
export function useInkPointer(core: InkCore2) {
  const { rootRef, doc, presencePointerRef, publishInkPresenceView, gestureRecognizer, pinchCameraRef, setDraftDoc, dragState, interactive, publishInteractionHover, dispatch, liveGesture, setMarqueePoints, setDragState, commitGesture, takePendingLive, gestureActiveRef, marqueePoints, publishInteractionSelection, abortGesture, isNavigator, setTableEdit, setTextEdit, atomicGesture, handleBlockPointerDown, handlePointerDown, handlePointerDownCapture, handleResizePointerDown } = useAbortGestureState({ core });
  const { handlePointerCancel, handlePointerMove, handlePointerUp, releaseContact } = useHandlePointerMoveState({ rootRef, doc, presencePointerRef, publishInkPresenceView, gestureRecognizer, pinchCameraRef, setDraftDoc, dragState, interactive, publishInteractionHover, dispatch, liveGesture, setMarqueePoints, setDragState, commitGesture, takePendingLive, gestureActiveRef, marqueePoints, publishInteractionSelection, abortGesture });
  const { handleDoubleClick, handleWheel } = useHandleWheelState({ rootRef, doc, isNavigator, setDraftDoc, dispatch, interactive, setTableEdit, setTextEdit, publishInteractionSelection, atomicGesture });
  return { handleBlockPointerDown, handleDoubleClick, handlePointerCancel, handlePointerDown, handlePointerDownCapture, handlePointerMove, handlePointerUp, handleResizePointerDown, handleWheel, releaseContact };
}

export type InkCore3 = InkCore2 & ReturnType<typeof useInkPointer>;
// #endregion InkCanvasHost
