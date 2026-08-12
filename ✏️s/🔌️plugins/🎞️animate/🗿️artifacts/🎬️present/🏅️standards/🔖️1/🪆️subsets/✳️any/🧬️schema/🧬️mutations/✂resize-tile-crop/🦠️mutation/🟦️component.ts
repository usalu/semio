/** 🔹 `resize-tile-crop` mutation payload — recrops a figure tile's normalized x,y,width,height frame. */
export interface ResizeTileCrop {
  id: string;
  newCrop: unknown;
}

/** 🔖️ Semantic descriptor mirror: verb=`resize` entity=`tile-crop` kind=`resize-tile-crop` record=`ResizedTileCrop`. */
export const ResizeTileCropKind = "resize-tile-crop" as const;
