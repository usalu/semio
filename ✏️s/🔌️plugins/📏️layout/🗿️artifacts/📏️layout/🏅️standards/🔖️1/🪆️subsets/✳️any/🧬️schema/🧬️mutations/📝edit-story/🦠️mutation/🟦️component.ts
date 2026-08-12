/** 📝 `edit-story` — replaces a story's authored `content` body. */
export interface EditStory {
  id: string;
  newContent: string;
}

/** 🔖️ Semantic descriptor mirror: verb=`edit` entity=`story` kind=`edit-story` record=`EditedStory`. */
export const EditStoryKind = "edit-story" as const;
