// #region 🧲Header
// 💻 .storybook/preview.ts
// Specs: Reuse the shared UI theme and level decorators for the root monorepo Storybook.
// Summary: Defines global Storybook preview parameters; loads CSS stacks only for the active `STORYBOOK_SCOPE` slice in dev.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { Preview } from "@storybook/react-vite";

import { Expertise } from "@ui/react";
import { withLevel } from "./withLevel";
import { withTheme } from "./withTheme";

declare const __STORYBOOK_LOAD_UI__: boolean;

//#region 🔖ScopeStyles
if (__STORYBOOK_LOAD_UI__) {
	await import("../ui/react/globals.css");
}
//#endregion 🔖ScopeStyles

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

enum Device {
	DESKTOP = "desktop",
	TABLET = "tablet",
	MOBILE = "mobile",
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
		device: {
			description: "Layout density / shell device (Elements + sketchpad parity)",
			toolbar: {
				title: "Device",
				icon: "mobile",
				items: [
					{ value: Device.DESKTOP, title: "Desktop" },
					{ value: Device.TABLET, title: "Tablet" },
					{ value: Device.MOBILE, title: "Mobile" },
				],
				dynamicTitle: true,
			},
		},
		expertise: {
			description: "Tooltip and label verbosity (Elements expertise provider)",
			toolbar: {
				title: "Expertise",
				icon: "user",
				items: [
					{ value: Expertise.BEGINNER, title: "Beginner" },
					{ value: Expertise.NORMAL, title: "Normal" },
					{ value: Expertise.EXPERT, title: "Expert" },
				],
				dynamicTitle: true,
			},
		},
	},
	initialGlobals: {
		theme: Theme.SYSTEM,
		level: Level.BASE,
		device: Device.DESKTOP,
		expertise: Expertise.NORMAL,
	},
	decorators: [withLevel, withTheme],
	tags: ["autodocs"],
};

export default preview;
