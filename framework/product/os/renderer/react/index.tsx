// #region 🧲Header
/** @emoji 🖥️ `@semio-tech/framework-os-renderer-react` — generic os app host resolution for embedded instances. */
// #endregion 🧲Header

import type { ReactElement } from "react";
import type { OsAppInstance } from "@semio-tech/framework-os-core";
import { resolveOsAppDefinition } from "@semio-tech/framework-os-core";
import { SAppHostRouter } from "@semio-tech/framework-playground-renderer-react";

/** @emoji 🧩 Resolves the static {@link AppDefinition} for an embedded os instance. */
export function resolveOsAppDefinitionForInstance(instance: OsAppInstance | null) {
	return instance ? resolveOsAppDefinition(instance) : undefined;
}

/** @emoji 🖥️ Generic os app host router — embeds any registered technology inside an os shell. */
export function OsAppHostRouter({ instance }: { readonly instance: OsAppInstance | null }): ReactElement {
	return <SAppHostRouter instance={instance} />;
}

export { SAppHostRouter };
