// #region 🧲Header
// 💻 .storybook/withLevel.tsx
// Specs: Single level decorator for monorepo Storybook: globals.level (elements) and args.level (compose stories).
// Summary: Wraps stories with LevelProvider and optional bordered chrome for arg-driven level.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import { type Level, LevelProvider, getLevelBgClass } from "@ui/react";
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
