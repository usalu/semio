// #region Header

// appType.ts

// Separate file to avoid circular dependency between store.tsx and apps/registry.tsx

// #endregion

import { useMemo } from "react";
import { appRegistry } from "./apps/registry";
import { AppType, useNavigation } from "./store";

export function getAppTypeFromPath(path: string): AppType {
  const pathParts = path.split("/").filter((p) => p);
  const app = appRegistry.getAppForPath(pathParts);
  return app?.id || "home";
}

export function useAppType(): AppType {
  const navigation = useNavigation();
  return useMemo(() => getAppTypeFromPath(navigation), [navigation]);
}
