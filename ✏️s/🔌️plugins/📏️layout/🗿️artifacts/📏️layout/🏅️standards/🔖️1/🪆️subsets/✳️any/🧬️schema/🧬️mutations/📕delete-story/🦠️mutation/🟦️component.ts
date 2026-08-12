/** 🗑️ `delete-story` — removes a {@link TextStory} by id; inverse recreates it via `create-story`. */
export interface DeleteStory {
  id: string;
}

/** 🔖️ Semantic descriptor mirror: verb=`delete` entity=`story` kind=`delete-story` record=`DeletedStory`. */
export const DeleteStoryKind = "delete-story" as const;
