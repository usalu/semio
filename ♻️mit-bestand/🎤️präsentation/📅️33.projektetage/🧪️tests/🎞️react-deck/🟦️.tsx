// @vitest-environment jsdom
import { describe, afterEach } from "vitest";
import { unmountPresentation } from "@semio-tech/presentation-react";
import { registerFocusToLabelsMorphTests } from "./🎬️focus-to-labels-morph/🟦️.tsx";
import { registerGhostAndWrapperTests } from "./👻️ghosts-and-wrappers/🟦️.tsx";
import { registerLabelFramesAndAdvanceTests } from "./🧭️label-frames-and-advance/🟦️.tsx";
afterEach(() => { unmountPresentation(); document.body.innerHTML = ""; });
describe("Projektetage presentation interaction", () => {
  registerFocusToLabelsMorphTests();
  registerGhostAndWrapperTests();
  registerLabelFramesAndAdvanceTests();
});
