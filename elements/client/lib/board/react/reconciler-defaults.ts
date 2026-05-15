/** @emoji 🧷 Default `react-reconciler@0.33` host hooks not used by the board scene (mirrors `@react-three/fiber` stubs). */
import React from "react";
import { ContinuousEventPriority, DefaultEventPriority, DiscreteEventPriority, NoEventPriority } from "react-reconciler/constants";

let boardSchedulerPriority = NoEventPriority;

/** @emoji 🧩 Static host surface required by `react-reconciler` beyond board scene mutations. */
export const BOARD_RECONCILER_DEFAULTS: Record<string, unknown> = {
	HostTransitionContext: React.createContext(null) as never,
	NotPendingTransition: null,
	acquireResource: () => null,
	acquireSingletonInstance: () => null,
	appendChildToContainerChildSet: () => {},
	bindToConsole: () => () => undefined,
	canHydrateActivityInstance: () => false,
	canHydrateFormStateMarker: () => false,
	canHydrateInstance: () => false,
	canHydrateSuspenseInstance: () => false,
	canHydrateTextInstance: () => false,
	clearSuspenseBoundary: () => {},
	cloneHiddenInstance: () => {
		throw new Error("Board host: cloneHiddenInstance unsupported");
	},
	cloneHiddenTextInstance: () => {
		throw new Error("Board host: cloneHiddenTextInstance unsupported");
	},
	cloneInstance: () => {
		throw new Error("Board host: cloneInstance unsupported");
	},
	commitHydratedActivityInstance: () => null,
	commitHydratedContainer: () => null,
	commitHydratedInstance: () => null,
	commitHydratedSuspenseInstance: () => null,
	commitTextUpdate: () => {},
	createContainerChildSet: () => ({}),
	createHoistableInstance: () => null,
	diffHydratedPropsForDevWarnings: () => {},
	diffHydratedTextForDevWarnings: () => null,
	describeHydratableInstanceForDevWarnings: () => {},
	extraDevToolsConfig: {},
	finalizeContainerChildren: () => {},
	finalizeHydratedChildren: () => null,
	findFiberRoot: () => null,
	flushHydrationEvents: () => null,
	getBoundingRect: () => null,
	getFirstHydratableChild: () => null,
	getFirstHydratableChildWithinActivityInstance: () => null,
	getFirstHydratableChildWithinContainer: () => null,
	getFirstHydratableChildWithinSingleton: () => null,
	getFirstHydratableChildWithinSuspenseInstance: () => null,
	getHoistableRoot: () => null,
	getNextHydratableInstanceAfterActivityInstance: () => null,
	getNextHydratableInstanceAfterSuspenseInstance: () => null,
	getNextHydratableSibling: () => null,
	getNextHydratableSiblingAfterSingleton: () => null,
	getResource: () => null,
	getSuspendedCommitReason: () => null,
	getSuspenseInstanceFallbackErrorDetails: () => null,
	getTextContent: () => null,
	hideDehydratedBoundary: () => null,
	hideInstance: () => {},
	hideTextInstance: () => {},
	hydrateActivityInstance: () => null,
	hydrateHoistable: () => null,
	hydrateInstance: () => null,
	hydrateSuspenseInstance: () => null,
	hydrateTextInstance: () => null,
	isFormStateMarkerMatching: () => false,
	isHiddenSubtree: () => false,
	isHostHoistableType: () => false,
	isHostSingletonType: () => false,
	isSingletonScope: () => false,
	isSuspenseInstanceFallback: () => false,
	isSuspenseInstancePending: () => false,
	matchAccessibilityRole: () => false,
	mayResourceSuspendCommit: () => false,
	maySuspendCommit: () => false,
	maySuspendCommitInSyncRender: () => false,
	maySuspendCommitOnUpdate: () => false,
	mountHoistable: () => null,
	preloadInstance: () => true,
	preloadResource: () => false,
	prepareToCommitHoistables: () => null,
	registerSuspenseInstanceRetry: () => {},
	releaseResource: () => null,
	releaseSingletonInstance: () => null,
	rendererPackageName: "@elements/board",
	rendererVersion: "0.1.0",
	replaceContainerChildren: () => {},
	resetFormInstance: () => {},
	resetTextContent: () => {},
	resolveEventTimeStamp: () => -1.1,
	resolveEventType: () => null,
	resolveSingletonInstance: () => null,
	setCurrentUpdatePriority(p: number) {
		boardSchedulerPriority = p;
	},
	getCurrentUpdatePriority() {
		return boardSchedulerPriority;
	},
	resolveUpdatePriority() {
		if (boardSchedulerPriority !== NoEventPriority) {
			return boardSchedulerPriority;
		}
		const w = globalThis as typeof globalThis & { event?: Event };
		const t = w.event?.type;
		if (
			t === "click" ||
			t === "contextmenu" ||
			t === "dblclick" ||
			t === "pointercancel" ||
			t === "pointerdown" ||
			t === "pointerup"
		) {
			return DiscreteEventPriority;
		}
		if (
			t === "pointermove" ||
			t === "pointerout" ||
			t === "pointerover" ||
			t === "pointerenter" ||
			t === "pointerleave" ||
			t === "wheel"
		) {
			return ContinuousEventPriority;
		}
		return DefaultEventPriority;
	},
	setFocusIfFocusable: () => false,
	setupIntersectionObserver: () => () => undefined,
	shouldAttemptEagerTransition: () => false,
	shouldDeleteUnhydratedTailInstances: () => false,
	startSuspendingCommit: () => null,
	supportsResources: false,
	supportsSingletons: false,
	supportsTestSelectors: false,
	suspendInstance: () => {},
	suspendResource: () => false,
	trackSchedulerEvent: () => {},
	unhideDehydratedBoundary: () => null,
	unhideInstance: () => {},
	unhideTextInstance: () => {},
	unmountHoistable: () => null,
	validateHydratableInstance: () => {},
	validateHydratableTextInstance: () => {},
	waitForCommitToBeReady: () => null,
	clearSuspenseBoundaryFromContainer: () => {},
};
