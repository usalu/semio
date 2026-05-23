// #region 🧲Header
// 💻 .storybook/withTheme.tsx — Storybook decorator: theme (system/light/dark), device (desktop/tablet/mobile), expertise via {@link useElementsSurfaceChrome} from `@elements/ui`.
// #endregion 🧲Header

import {
	Expertise,
	useElementsSurfaceChrome,
	type ElementsSurfaceDevice,
	type ElementsSurfaceTheme,
} from "@elements/ui";
import type { Decorator } from "@storybook/react-vite";
import * as React from "react";

// #region 🌈StorySurfaceHost
const StorySurfaceHost: React.FC<{
	children: React.ReactNode;
	globals: { theme?: string; device?: string; expertise?: string };
}> = ({ children, globals }) => {
	const theme = (globals.theme as ElementsSurfaceTheme | undefined) ?? "system";
	const device = (globals.device as ElementsSurfaceDevice | undefined) ?? "desktop";
	const expertise = (globals.expertise as Expertise | undefined) ?? Expertise.NORMAL;
	useElementsSurfaceChrome({ theme, device, expertise });
	return <>{children}</>;
};
// #endregion 🌈StorySurfaceHost

// #region 🌈WithTheme
export const withTheme: Decorator = (Story, context) => (
	<StorySurfaceHost globals={context.globals as { theme?: string; device?: string; expertise?: string }}>
		<Story />
	</StorySurfaceHost>
);
// #endregion 🌈WithTheme
