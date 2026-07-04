import type { ReactElement } from "react";

export function FlowCanvas(): ReactElement {
	return <div data-testid="flow-canvas-stub" />;
}

export class FlowExtensionHost {
	getRevision(): number {
		return 0;
	}
	subscribe(): () => void {
		return () => {};
	}
	registerContributions(): void {}
	unregisterContributions(): void {}
}

export function createEphemeralFlowStore() {
	return {};
}

export function buildFlowContextMenuItems(): never[] {
	return [];
}

export type FlowCanvasCommandRequest = { readonly command: string; readonly argsJson: string; readonly epoch: number };
export type FlowCanvasContextMenuContext = Record<string, never>;
export type FlowContextMenuDispatch = (command: string, args?: Record<string, unknown>) => void;
