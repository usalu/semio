import type { Plan } from "../split.ts";
const base = "✏️s/🧑‍💻dev/🏗️fem/📖️stories/🧭️coordination";
export const plan: Plan = {
  source: `${base}/🟦️.ts`,
  baseDir: base,
  modules: [
    { dir: "🔤|dsl-parser", ranges: [["FemDslDocument", "parseFemDsl"]] },
    { dir: "📸|snapshot-projection", ranges: [["FemVec2", "EMPTY_FEM3D_SNAPSHOT"]] },
    { dir: "🎞|scenes", ranges: [["FEM2D_SCALE", "buildFem3dSceneNode"]] },
    { dir: "🔁|story-reducers", ranges: [["FemStoryResultDisplay", "femResultCaptionLine"]] },
    { dir: ".", ranges: [["FemStoryLocale", "femStoryOmissions"]] },
  ],
};
