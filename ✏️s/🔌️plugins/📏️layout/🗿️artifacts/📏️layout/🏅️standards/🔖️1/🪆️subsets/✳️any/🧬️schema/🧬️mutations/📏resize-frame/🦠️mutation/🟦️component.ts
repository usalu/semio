/** 📏 `resize-frame` — changes a frame's `bounds.width`/`bounds.height` extent. */
export interface ResizeFrame {
  pageId: string;
  frameId: string;
  newWidth: number;
  newHeight: number;
}

/** 🔖️ Semantic descriptor mirror: verb=`resize` entity=`frame` kind=`resize-frame` record=`ResizedFrame`. */
export const ResizeFrameKind = "resize-frame" as const;
