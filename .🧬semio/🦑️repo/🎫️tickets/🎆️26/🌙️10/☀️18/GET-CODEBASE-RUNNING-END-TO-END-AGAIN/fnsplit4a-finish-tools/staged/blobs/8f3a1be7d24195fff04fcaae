/** 🟠️ The part3 phase of useAgentBridge. */
import { useEffect } from "react";
import type { useDiscoveredState } from "../🔶️discovered/🟦️.tsx";

/** 🟡 Runs the effects and registrations of useAgentBridge. */
export function usePart3({ status, send, instances }: Pick<ReturnType<typeof useDiscoveredState>, "status" | "send" | "instances">) {
  useEffect(() => {
    if (status !== "open") return;
    send({ variant: "instances", entries: [...instances] });
  }, [instances, status, send]);
}
