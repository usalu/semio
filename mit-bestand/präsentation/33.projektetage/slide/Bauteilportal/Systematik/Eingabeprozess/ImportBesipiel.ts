import {
	MEDIA_SCROLL_ORIGIN_TOP_LEFT,
	splitFigureGrid,
	type SlideFile,
} from "@framework/presentation/core";

const FRAME = { x: 0.04, y: 0.06, width: 0.92, height: 0.88 };
const GAP = 0.012;

const ITEMS = [
	{ id: "import-besipiel-holzbalken", src: "/bauteilbörse-holzbalken.png", alt: "Holzbalken" },
	{ id: "import-besipiel-rippenplatte", src: "/bauteilbörse-rippenplatte.png", alt: "Rippenplatte" },
	{ id: "import-besipiel-träger-heb", src: "/bauteilbörse-träger-heb.png", alt: "Träger HEB" },
] as const;

const cells = splitFigureGrid({ rows: 1, columns: 3, frame: FRAME, gap: GAP });

export default {
	order: 4,
	participants: ITEMS.map((item) => ({ id: item.id })),
	embodiments: ITEMS.map((item) => ({
		kind: "figure" as const,
		id: `${item.id}--figure`,
		src: item.src,
		alt: item.alt,
		scrollOrigin: MEDIA_SCROLL_ORIGIN_TOP_LEFT,
	})),
	arrangement: {
		id: "eingabeprozess-import-besipiel",
		name: "Import Besipiel",
		dispositions: ITEMS.map((item, index) => ({
			participantId: item.id,
			embodimentId: `${item.id}--figure`,
			emphasis: "active" as const,
			position: cells[index]!.position,
		})),
	},
} satisfies SlideFile;
