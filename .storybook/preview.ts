// #region 🧲Header
// 💻 .storybook/preview.ts
// Specs: Reuse the shared UI theme and level decorators for the root monorepo Storybook.
// Summary: Defines global Storybook preview parameters for the aggregated workspace stories.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { Preview } from "@storybook/react-vite";

import { withLevel } from "../elements/client/lib/react/.storybook/withLevel";
import { withTheme } from "../elements/client/lib/react/.storybook/withTheme";

import "../elements/client/lib/react/globals.css";
import "../semio/client/lib/react/rendering/globals.css";

enum Theme {
	SYSTEM = "system",
	LIGHT = "light",
	DARK = "dark",
}

enum Level {
	BASE = "base",
	WINDOW = "window",
	PANEL = "panel",
	OVERLAY = "overlay",
	TEMPORARY = "temporary",
}

const preview: Preview = {
	parameters: {
		controls: {
			matchers: {
				color: /(background|color)$/i,
				date: /Date$/i,
			},
		},
	},
	globalTypes: {
		theme: {
			description: "Global theme for components",
			toolbar: {
				title: "Theme",
				icon: "circlehollow",
				items: [
					{ value: Theme.SYSTEM, title: "System", icon: "browser" },
					{ value: Theme.LIGHT, title: "Light", icon: "sun" },
					{ value: Theme.DARK, title: "Dark", icon: "moon" },
				],
				dynamicTitle: true,
			},
		},
		level: {
			description: "UI level for components",
			toolbar: {
				title: "Level",
				icon: "component",
				items: [
					{ value: Level.BASE, title: "Base" },
					{ value: Level.WINDOW, title: "Window" },
					{ value: Level.PANEL, title: "Panel" },
					{ value: Level.OVERLAY, title: "Overlay" },
					{ value: Level.TEMPORARY, title: "Temporary" },
				],
				dynamicTitle: true,
			},
		},
	},
	initialGlobals: {
		theme: Theme.SYSTEM,
		level: Level.BASE,
	},
	decorators: [withLevel, withTheme],
	tags: ["autodocs"],
};

export default preview;