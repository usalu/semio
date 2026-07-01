import type { SlideFile } from "@semio-tech/framework-presentation-core";
import { baukomponentenGridArtifacts } from "../../../../index.ts";

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
