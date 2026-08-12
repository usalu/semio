/** 🎨 `change-frame-fill` — sets a `Frame::Rect`'s `fill` color (`None` clears it). A no-op on non-rect frames, matching the pre-migration `PatchFrame`'s `fill` handling. */
export interface ChangeFrameFill {
  pageId: string;
  frameId: string;
  newFill: [number, number, number, number] | null;
}

/** 🔖️ Semantic descriptor mirror: verb=`change` entity=`frame-fill` kind=`change-frame-fill` record=`ChangedFrameFill`. */
export const ChangeFrameFillKind = "change-frame-fill" as const;
