/** 📖 `create-story` — brings a new {@link TextStory} into existence in the id-keyed `stories` collection. */
export interface CreateStory {
  story: unknown;
  index: number | null;
}

/** 🔖️ Semantic descriptor mirror: verb=`create` entity=`story` kind=`create-story` record=`CreatedStory`. */
export const CreateStoryKind = "create-story" as const;
