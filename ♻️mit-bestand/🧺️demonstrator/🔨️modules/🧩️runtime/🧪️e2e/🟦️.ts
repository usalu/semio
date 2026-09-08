import { join } from "node:path";

export const DEMONSTRATOR_E2E_OWNER = "@semio-tech/mit-bestand-demonstrator:e2e";

/** 🗂️ Locates ephemeral E2E service records outside cacheable application outputs. */
export function demonstratorE2eSessionRoot(workspace: string): string {
  return join(workspace, "♻️mit-bestand/🧺️demonstrator/dist/services/e2e");
}

/** 🧭️ Requires Nx's shared invocation identity before preparing a fresh service generation. */
export function demonstratorE2eInvocationPid(environment: Readonly<Record<string, string | undefined>>): number {
  const value = environment.NX_INVOCATION_ROOT_PID;
  if (!value || !/^[1-9][0-9]*$/.test(value) || !Number.isSafeInteger(Number(value))) throw new Error("Demonstrator E2E must run through Nx");
  return Number(value);
}
