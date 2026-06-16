import type { SlideFile } from "@framework/presentation/core";
import { entwurfswerkzeugKatalogArtifacts } from "../../../../spec.ts";

const { participants, embodiments, dispositions } = entwurfswerkzeugKatalogArtifacts();

export default {
	order: 0,
	participants,
	embodiments,
	arrangement: {
		id: "entwurfswerkzeug-katalog",
		name: "Katalog",
		dispositions,
	},
} satisfies SlideFile;
