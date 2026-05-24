// #region 🧲Header
/** @emoji 🎈 UI depth {@link Level} context and Tailwind class helpers (pure React, no framework). */
// #endregion 🧲Header

import * as React from "react";

/** @emoji 📚 Semantic UI depth layer for background, hover, and z-index tokens. */
export type Level = "base" | "window" | "panel" | "overlay" | "temporary";

const LevelContext = React.createContext<Level>("base");

/** @emoji 🎈 Sets the current UI depth level for descendant chrome. */
export const LevelProvider: React.FC<{
	readonly level: Level;
	readonly children: React.ReactNode;
}> = ({ level, children }) => <LevelContext.Provider value={level}>{children}</LevelContext.Provider>;

/** @emoji 🪝 Returns the nearest {@link LevelProvider} level. */
export function useLevel(): Level {
	return React.useContext(LevelContext);
}

/** @emoji 🎨 Tailwind background class for a {@link Level}. */
export function getLevelBgClass(level: Level): string {
	switch (level) {
		case "window":
			return "bg-window";
		case "panel":
			return "bg-panel";
		case "overlay":
			return "bg-overlay";
		case "temporary":
			return "bg-temporary";
		default:
			return "bg-base";
	}
}

/** @emoji 🎨 Tailwind hover background class for a {@link Level}. */
export function getLevelHoverClass(level: Level): string {
	switch (level) {
		case "window":
			return "hover:bg-hover-window";
		case "panel":
			return "hover:bg-hover-panel";
		case "overlay":
			return "hover:bg-hover-overlay";
		case "temporary":
			return "hover:bg-hover-temporary";
		default:
			return "hover:bg-hover-base";
	}
}

/** @emoji 🎨 Tailwind active-hover class for a {@link Level}. */
export function getLevelActiveHoverClass(level: Level): string {
	switch (level) {
		case "window":
			return "data-[state=active]:bg-hover-window";
		case "panel":
			return "data-[state=active]:bg-hover-panel";
		case "overlay":
			return "data-[state=active]:bg-hover-overlay";
		case "temporary":
			return "data-[state=active]:bg-hover-temporary";
		default:
			return "data-[state=active]:bg-hover-base";
	}
}

/** @emoji 🎨 Tailwind z-index class for a {@link Level}. */
export function getLevelZClass(level: Level): string {
	switch (level) {
		case "window":
			return "z-window";
		case "panel":
			return "z-panel";
		case "overlay":
			return "z-overlay";
		case "temporary":
			return "z-temporary";
		default:
			return "z-base";
	}
}

/** @emoji 🎨 Tailwind border token class for a {@link Level}. */
export function getLevelBorderElementClass(level: Level): string {
	switch (level) {
		case "window":
			return "border-hover-window";
		case "panel":
			return "border-hover-panel";
		case "overlay":
			return "border-hover-overlay";
		case "temporary":
			return "border-hover-temporary";
		default:
			return "border-hover-base";
	}
}

/** @emoji 🎨 Tailwind divide token class for a {@link Level}. */
export function getLevelDivideElementClass(level: Level): string {
	switch (level) {
		case "window":
			return "divide-hover-window";
		case "panel":
			return "divide-hover-panel";
		case "overlay":
			return "divide-hover-overlay";
		case "temporary":
			return "divide-hover-temporary";
		default:
			return "divide-hover-base";
	}
}
