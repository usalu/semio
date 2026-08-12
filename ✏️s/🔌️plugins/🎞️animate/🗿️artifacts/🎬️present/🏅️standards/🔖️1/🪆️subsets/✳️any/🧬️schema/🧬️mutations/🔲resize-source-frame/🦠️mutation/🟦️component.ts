/** 🔹 `resize-source-frame` mutation payload — recrops the shared figure source's normalized x,y,width,height frame. */
export interface ResizeSourceFrame {
  newFrame: unknown;
}

/** 🔖️ Semantic descriptor mirror: verb=`resize` entity=`source-frame` kind=`resize-source-frame` record=`ResizedSourceFrame`. */
export const ResizeSourceFrameKind = "resize-source-frame" as const;
