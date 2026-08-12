/** ➖️ `delete-frame` — removes a {@link Frame} from a page by id (and every layer's `object_ids` referencing it); inverse recreates it via `create-frame`. */
export interface DeleteFrame {
  pageId: string;
  frameId: string;
}

/** 🔖️ Semantic descriptor mirror: verb=`delete` entity=`frame` kind=`delete-frame` record=`DeletedFrame`. */
export const DeleteFrameKind = "delete-frame" as const;
