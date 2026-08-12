/** 🕹️ `move-frame` — absolute spatial reposition of a frame's `bounds.x`/`bounds.y`. */
export interface MoveFrame {
  pageId: string;
  frameId: string;
  newX: number;
  newY: number;
}

/** 🔖️ Semantic descriptor mirror: verb=`move` entity=`frame` kind=`move-frame` record=`MovedFrame`. */
export const MoveFrameKind = "move-frame" as const;
