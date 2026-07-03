// #region 🧲Header
/** @emoji 🖥️ OS drill-in instance host bridge — studio store projection without app package coupling to s-react. */
// #endregion 🧲Header

import type { ReactElement, ReactNode } from "react";
import type { OsAppInstance } from "@semio-tech/framework-os-core";
import { appInstanceResourceProjection } from "@semio-tech/s-core";
import { reactHostPort } from "@semio-tech/ui-react";

//#region 🔖Bridge
export type OsInstanceResourceBundle = ReturnType<typeof appInstanceResourceProjection>;

/** @emoji 🌉 Studio store surface for app {@link AppRendererContribution.instanceHost} drill-in hosts. */
export interface OsInstanceHostBridge {
	readonly subscribe: (listener: () => void) => () => void;
	readonly getGeneration: () => number;
	readonly getInstances: () => readonly OsAppInstance[];
	readonly projectInstance: (instanceId: string) => OsInstanceResourceBundle;
	readonly dispatch: (command: unknown) => void;
}

const OsInstanceHostBridgeContext = reactHostPort.createContext<OsInstanceHostBridge | null>(null);

/** @emoji 🌉 Supplies studio projection and dispatch to nested OS instance hosts. */
export function OsInstanceHostBridgeProvider({
	bridge,
	children,
}: {
	readonly bridge: OsInstanceHostBridge;
	readonly children: ReactNode;
}): ReactElement {
	return <OsInstanceHostBridgeContext.Provider value={bridge}>{children}</OsInstanceHostBridgeContext.Provider>;
}

/** @emoji 🔎 Reads the OS instance host bridge from context. */
export function useOsInstanceHostBridge(): OsInstanceHostBridge {
	const bridge = reactHostPort.useContext(OsInstanceHostBridgeContext);
	if (!bridge) throw new Error("OsInstanceHostBridgeProvider is required");
	return bridge;
}

/** @emoji 📦 Resolves materialized projection for one app instance. */
export function useOsInstanceMaterialization(instance: OsAppInstance): OsInstanceResourceBundle {
	const bridge = useOsInstanceHostBridge();
	const generation = reactHostPort.useSyncExternalStore(bridge.subscribe, bridge.getGeneration, bridge.getGeneration);
	return reactHostPort.useMemo(() => {
		void generation;
		return bridge.projectInstance(instance.id);
	}, [bridge, generation, instance.id]);
}

/** @emoji ⬆️ Upstream media-graph badge for drilled-in app instances. */
export function OsUpstreamBadge({ upstreamInstanceId }: { readonly upstreamInstanceId: string | null }): ReactElement | null {
	const bridge = useOsInstanceHostBridge();
	if (!upstreamInstanceId) return null;
	const upstream = bridge.getInstances().find((entry) => entry.id === upstreamInstanceId);
	if (!upstream) return null;
	return (
		<div className="border-b border-border/60 bg-muted/40 px-3 py-1 text-xs text-muted-foreground">
			Upstream · {upstream.label} ({upstream.yields})
		</div>
	);
}
//#endregion 🔖Bridge
