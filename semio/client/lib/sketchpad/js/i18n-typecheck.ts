/** @emoji 🪁 Compile-time gate for sketchpad toolbar parent locale bundles (keep in sync with {@link semioSketchpadToolbarParentDe} in index.ts). */
import type { UiLabelValue, UiToolbarParentCategory } from "@ui/react/i18n-types";

type SemioSketchpadToolbarParentEntries = { readonly [K in UiToolbarParentCategory]: UiLabelValue };

const semioSketchpadToolbarParentDe: SemioSketchpadToolbarParentEntries = {
	history: { label: { normal: "Verlauf", beginner: "Verlauf" } },
	hand: { label: { normal: "Hand", beginner: "Hand" } },
	selection: { label: { normal: "Auswahl", beginner: "Auswahl" } },
	lasso: { label: { normal: "Lasso", beginner: "Lasso" } },
	filter: { label: { normal: "Filter", beginner: "Filter" } },
	open: { label: { normal: "Öffnen", beginner: "Öffnen" } },
	save: { label: { normal: "Speichern", beginner: "Speichern" } },
	transfer: { label: { normal: "Transfer", beginner: "Transfer" } },
	transform: { label: { normal: "Transformieren", beginner: "Transformieren" } },
	create: { label: { normal: "Erstellen", beginner: "Erstellen" } },
	view: { label: { normal: "Ansicht", beginner: "Ansicht" } },
	actions: { label: { normal: "Aktionen", beginner: "Aktionen" } },
	settings: { label: { normal: "Einstellungen", beginner: "Einstellungen" } },
};

const semioSketchpadToolbarParentEn: SemioSketchpadToolbarParentEntries = {
	history: { label: { normal: "History", beginner: "History" } },
	hand: { label: { normal: "Hand", beginner: "Hand" } },
	selection: { label: { normal: "Selection", beginner: "Selection" } },
	lasso: { label: { normal: "Lasso", beginner: "Lasso" } },
	filter: { label: { normal: "Filter", beginner: "Filter" } },
	open: { label: { normal: "Open", beginner: "Open" } },
	save: { label: { normal: "Save", beginner: "Save" } },
	transfer: { label: { normal: "Transfer", beginner: "Transfer" } },
	transform: { label: { normal: "Transform", beginner: "Transform" } },
	create: { label: { normal: "Create", beginner: "Create" } },
	view: { label: { normal: "View", beginner: "View" } },
	actions: { label: { normal: "Actions", beginner: "Actions" } },
	settings: { label: { normal: "Settings", beginner: "Settings" } },
};

const _sketchpadToolbarParentBundles = [semioSketchpadToolbarParentDe, semioSketchpadToolbarParentEn] as const;
