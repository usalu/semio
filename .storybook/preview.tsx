// #region 🧲Header
// 💻 .storybook/preview.ts
// Specs: Reuse the shared UI appearance and level decorators for the root monorepo Storybook.
// Summary: Defines global Storybook preview parameters; loads CSS stacks only for the active `STORYBOOK_SCOPE` slice in dev.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import type { Preview } from "@storybook/react-vite";

import { Expertise } from "@semio-tech/ui-react";

declare const __STORYBOOK_LOAD_UI__: boolean;
declare const __STORYBOOK_LOAD_COMPOSE__: boolean;
declare const __STORYBOOK_LOAD_PUZZLE__: boolean;

//#region 🔖ScopeStyles
if (__STORYBOOK_LOAD_UI__ || __STORYBOOK_LOAD_COMPOSE__ || __STORYBOOK_LOAD_PUZZLE__) {
	await import("./globals.css");
}
//#endregion 🔖ScopeStyles

enum Appearance {
	SYSTEM = "system",
	LIGHT = "light",
	DARK = "dark",
}

enum Level {
	BASE = "base",
	CANVAS = "canvas",
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
		appearance: {
			description: "Global appearance for components",
			toolbar: {
				title: "Appearance",
				icon: "circlehollow",
				items: [
					{ value: Appearance.SYSTEM, title: "System", icon: "browser" },
					{ value: Appearance.LIGHT, title: "Light", icon: "sun" },
					{ value: Appearance.DARK, title: "Dark", icon: "moon" },
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
					{ value: Level.CANVAS, title: "Canvas" },
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
		appearance: Appearance.SYSTEM,
		level: Level.BASE,
		device: Device.DESKTOP,
		expertise: Expertise.NORMAL,
	},
	decorators: [withLevel, withAppearance],
	tags: ["autodocs"],
};

export default preview;

//#region 🔖withLevel
import { type Level, LevelProvider, getLevelBgClass } from "@semio-tech/ui-react";
import type { Decorator } from "@storybook/react-vite";
import React from "react";

// #region 🧩LevelWrapper
/** Border + min width used when a story sets `level` via args (compose UI / algorithms). */
export const LevelWrapper: React.FC<{ level: Level; children: React.ReactNode }> = ({ level, children }) => {
	return (
		<div className={`p-4 ${getLevelBgClass(level)} border min-w-[200px]`}>
			<LevelProvider level={level}>{children}</LevelProvider>
		</div>
	);
};
// #endregion 🧩LevelWrapper

// #region 🧩WithLevel
export const withLevel: Decorator = (Story, context) => {
	const argLevel = context.args?.level as Level | undefined;
	if (argLevel) {
		return (
			<LevelWrapper level={argLevel}>
				<Story />
			</LevelWrapper>
		);
	}
	const level = context.globals.level as Level;
	return (
		<LevelProvider level={level}>
			<div className={`p-4 ${getLevelBgClass(level)}`}>
				<Story />
			</div>
		</LevelProvider>
	);
};
// #endregion 🧩WithLevel
//#endregion 🔖withLevel

//#region 🔖withAppearance
import {
	useElementsSurfaceChrome,
	type ElementsSurfaceDevice,
	type ElementsSurfaceAppearance,
} from "@semio-tech/ui-react";

// #region 🌈StorySurfaceHost
const StorySurfaceHost: React.FC<{
	children: React.ReactNode;
	globals: { appearance?: string; device?: string; expertise?: string };
}> = ({ children, globals }) => {
	const appearance = (globals.appearance as ElementsSurfaceAppearance | undefined) ?? "system";
	const device = (globals.device as ElementsSurfaceDevice | undefined) ?? "desktop";
	const expertise = (globals.expertise as Expertise | undefined) ?? Expertise.NORMAL;
	useElementsSurfaceChrome({ appearance, device, expertise });
	return <>{children}</>;
};
// #endregion 🌈StorySurfaceHost

// #region 🌈WithAppearance
export const withAppearance: Decorator = (Story, context) => (
	<StorySurfaceHost globals={context.globals as { appearance?: string; device?: string; expertise?: string }}>
		<Story />
	</StorySurfaceHost>
);
// #endregion 🌈WithAppearance
//#endregion 🔖withAppearance
