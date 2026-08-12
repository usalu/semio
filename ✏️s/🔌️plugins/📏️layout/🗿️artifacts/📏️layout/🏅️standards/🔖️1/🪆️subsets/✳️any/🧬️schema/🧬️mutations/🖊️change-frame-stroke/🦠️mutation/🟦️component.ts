/** 🖊️ `change-frame-stroke` — sets a `Frame::Rect`'s `stroke` color (`None` clears it). A no-op on non-rect frames, matching the pre-migration `PatchFrame`'s `stroke` handling. */
export interface ChangeFrameStroke {
  pageId: string;
  frameId: string;
  newStroke: [number, number, number, number] | null;
}

/** 🔖️ Semantic descriptor mirror: verb=`change` entity=`frame-stroke` kind=`change-frame-stroke` record=`ChangedFrameStroke`. */
export const ChangeFrameStrokeKind = "change-frame-stroke" as const;
