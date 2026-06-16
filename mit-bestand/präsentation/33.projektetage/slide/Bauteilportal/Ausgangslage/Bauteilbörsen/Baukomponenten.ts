import type { SlideFile } from "@framework/presentation/core";
import { baukomponentenGridArtifacts } from "../../../../spec.ts";

const { participants, embodiments, dispositions } = baukomponentenGridArtifacts();

export default {
	order: 0,
	participants,
	embodiments,
	arrangement: {
		id: "baukomponenten",
		name: "Baukomponenten",
		dispositions,
	},
} satisfies SlideFile;
