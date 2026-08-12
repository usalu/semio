/** 🔤 `change-frame-wrap-mode` — sets a `Frame::Text`'s `wrap_mode`. A no-op on non-text frames. */
export interface ChangeFrameWrapMode {
  pageId: string;
  frameId: string;
  newWrapMode: string;
}

/** 🔖️ Semantic descriptor mirror: verb=`change` entity=`frame-wrap-mode` kind=`change-frame-wrap-mode` record=`ChangedFrameWrapMode`. */
export const ChangeFrameWrapModeKind = "change-frame-wrap-mode" as const;
