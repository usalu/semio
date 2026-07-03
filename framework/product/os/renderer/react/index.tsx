// #region 🧲Header
/** @emoji 🖥️ `@semio-tech/framework-os-renderer-react` — generic os app definition resolution for embedded instances. */
// #endregion 🧲Header

import type { OsAppInstance } from "@semio-tech/framework-os-core";
import { resolveOsAppDefinition } from "@semio-tech/framework-os-core";

export {
	applyAllOsSurfaceContributions,
	ensureOsAppContribution,
	ensureOsAppContributionByKind,
	playgroundKindForProgramId,
	resolveInstanceHost,
} from "./app-contribution-registry.ts";
export {
	OsInstanceHostBridgeProvider,
	OsUpstreamBadge,
	useOsInstanceHostBridge,
	useOsInstanceMaterialization,
	type OsInstanceHostBridge,
	type OsInstanceResourceBundle,
} from "./os-instance-host.tsx";
export { useOsShellHistory } from "./os-shell-history.tsx";

/** @emoji 🧩 Resolves the static {@link AppDefinition} for an embedded os instance. */
export function resolveOsAppDefinitionForInstance(instance: OsAppInstance | null) {
	return instance ? resolveOsAppDefinition(instance) : undefined;
}
