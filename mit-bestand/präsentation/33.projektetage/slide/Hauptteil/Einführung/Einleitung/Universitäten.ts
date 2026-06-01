import type { SlideFile } from "@framework/presentation/core";
import { introSlideFiles } from "@framework/presentation/core";
import { introSpec } from "../../../../spec.ts";

export default introSlideFiles(introSpec).find((slide) => slide.arrangement.id === "affiliations-2")! satisfies SlideFile;
