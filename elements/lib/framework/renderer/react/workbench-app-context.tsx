// #region 🧲Header
/** @emoji 🧭 Workbench {@link AppContext} and {@link useApp} for declarative shell surfaces. */
// #endregion 🧲Header

import type { ResolvedWorkbenchAppState, Workbench, WorkbenchApp } from "@elements/framework";
import * as React from "react";

import type { FooterItem, SidePanelTabConfig, UIWindowKindDefinition } from "./shell-chrome-types.tsx";

/** @emoji 🧭 Props for {@link WorkbenchView} (navbar, panels, golden-layout canvas). */
export interface WorkbenchViewProps {
	workbench: Workbench;
	defaultAppId?: string;
	uri?: string;
	onNavigate?: (uri: string) => void;
	canGoBack?: boolean;
	onGoBack?: () => void;
	canGoForward?: boolean;
	onGoForward?: () => void;
	canGoUp?: boolean;
	onGoUp?: () => void;
	mobile?: boolean;
	mobileQuery?: string;
	className?: string;
	resolvedWindowKindsOverride?: UIWindowKindDefinition[];
	slotToolbar?: React.ReactNode;
	extraFooterItems?: FooterItem[];
	augmentPanelTabs?: Partial<Record<"workbench" | "details", SidePanelTabConfig[]>>;
	initialPanelVisibility?: UIPanelVisibility;
}

/** @emoji 🧭 @deprecated Use {@link WorkbenchViewProps}. */
export type AppProps = WorkbenchViewProps;

export interface UIPanelVisibility {
	leftSidePanel: boolean;
	rightSidePanel: boolean;
}

export interface AppContextValue {
	workbench: Workbench;
	activeAppId: string;
	setActiveAppId: (id: string) => void;
	activeApp: ResolvedWorkbenchAppState;
	activeModeId: string | null;
	setActiveModeId: (id: string) => void;
	apps: WorkbenchApp[];
	panelVisibility: UIPanelVisibility;
	togglePanel: (panel: keyof UIPanelVisibility) => void;
	uri: string;
	navigate: (uri: string) => void;
	canGoBack: boolean;
	goBack: () => void;
	canGoForward: boolean;
	goForward: () => void;
	canGoUp: boolean;
	goUp: () => void;
}

export const AppContext = React.createContext<AppContextValue | undefined>(undefined);

/** @emoji 🪝 Returns the active {@link Workbench} shell context from the nearest {@link AppContext}. */
export function useApp(): AppContextValue {
	const ctx = React.useContext(AppContext);
	if (!ctx) throw new Error("useApp must be used within a WorkbenchView");
	return ctx;
}
