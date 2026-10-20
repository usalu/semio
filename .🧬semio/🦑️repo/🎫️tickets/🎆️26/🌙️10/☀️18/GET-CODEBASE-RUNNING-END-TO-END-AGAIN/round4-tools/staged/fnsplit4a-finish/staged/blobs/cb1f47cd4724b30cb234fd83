/** 🔸️ The part2 phase of useAgentBridge. */
import { bridgeProtocols } from "../🛰️offer/🟦️.ts";
import { useEffect } from "react";
import { BRIDGE_VERSION, decodeGatewayToShell, encodeShellToGateway, type GatewayToShell } from "@semio-tech/framework/mcp/bridge";
import { decodeShellAppCommand, encodeShellAppFrame, shellAppFault } from "@semio-tech/framework/mcp/channel";
import type { useDiscoveredState } from "../🔶️discovered/🟦️.tsx";
import { BRIDGE_HANDSHAKE_DEADLINE_MS, BRIDGE_UNANSWERED_ATTEMPTS, PING_INTERVAL_MS, RECONNECT_BASE_MS, RECONNECT_MAX_MS, answerAgentAppCommand, appendConversationEntry, applyInboundShellCommand, buildShellStateFrame, updateConversationEntry } from "../🟢️shared/🟦️.tsx";
import type { AgentAppCommandRequest, AgentBridgeVersionMismatch } from "../🟢️shared/🟦️.tsx";

/** 🔹 Runs the effects and registrations of useAgentBridge. */
export function usePart2({ setVersionMismatch, reconnectAttemptRef, config, setStatus, socketRef, reconnectTimerRef, pingTimerRef, setLastError, send, shellStateRef, instancesRef, setShellState, onCommandAppliedRef, setPendingApprovals, setConversation, setPresence, onAppCommandRef, shellKind, shellSessionId, principalActor, flags }: Pick<ReturnType<typeof useDiscoveredState>, "setVersionMismatch" | "reconnectAttemptRef" | "config" | "setStatus" | "socketRef" | "reconnectTimerRef" | "pingTimerRef" | "setLastError" | "send" | "shellStateRef" | "instancesRef" | "setShellState" | "onCommandAppliedRef" | "setPendingApprovals" | "setConversation" | "setPresence" | "onAppCommandRef" | "shellKind" | "shellSessionId" | "principalActor" | "flags">) {
  useEffect(() => {
    setVersionMismatch(null);
    reconnectAttemptRef.current = 0;
    if (!config) {
      setStatus("disabled");
      return;
    }
    const admittedConfig = config;
    let disposed = false;
    let settled = false;
    let welcomed = false;
    let unanswered = 0;
    let handshakeTimer: ReturnType<typeof setTimeout> | null = null;
    const clearHandshakeTimer = () => {
      if (handshakeTimer !== null) {
        clearTimeout(handshakeTimer);
        handshakeTimer = null;
      }
    };
    const settle = (terminal: "unavailable" | "incompatible", mismatch: AgentBridgeVersionMismatch | null) => {
      settled = true;
      clearHandshakeTimer();
      clearReconnectTimer();
      clearPingTimer();
      setVersionMismatch(mismatch);
      setStatus(terminal);
      const socket = socketRef.current;
      socketRef.current = null;
      if (socket) {
        socket.onclose = null;
        socket.onerror = null;
        socket.onmessage = null;
        socket.close();
      }
    };

    const clearReconnectTimer = () => {
      if (reconnectTimerRef.current !== null) {
        clearTimeout(reconnectTimerRef.current);
        reconnectTimerRef.current = null;
      }
    };
    const clearPingTimer = () => {
      if (pingTimerRef.current !== null) {
        clearInterval(pingTimerRef.current);
        pingTimerRef.current = null;
      }
    };

    const scheduleReconnect = () => {
      if (disposed || settled) return;
      clearPingTimer();
      const attempt = reconnectAttemptRef.current + 1;
      reconnectAttemptRef.current = attempt;
      const delay = Math.min(RECONNECT_BASE_MS * 2 ** (attempt - 1), RECONNECT_MAX_MS);
      setStatus("reconnecting");
      reconnectTimerRef.current = setTimeout(connect, delay);
    };

    const handleFrame = (frame: GatewayToShell) => {
      switch (frame.variant) {
        case "welcome": {
          clearHandshakeTimer();
          if (frame.bridgeVersion !== BRIDGE_VERSION) {
            settle("incompatible", { gateway: frame.bridgeVersion, shell: BRIDGE_VERSION });
            break;
          }
          welcomed = true;
          unanswered = 0;
          reconnectAttemptRef.current = 0;
          setVersionMismatch(null);
          setStatus("open");
          setLastError(null);
          send(buildShellStateFrame(shellStateRef.current));
          send({ variant: "instances", entries: [...instancesRef.current] });
          break;
        }
        case "shellCommand": {
          const { command, result, resultFrame } = applyInboundShellCommand(shellStateRef.current, frame.seq, frame.command, Date.now());
          if (result?.ok) {
            shellStateRef.current = result.state;
            setShellState(result.state);
          }
          onCommandAppliedRef.current?.(command, result);
          send(resultFrame);
          break;
        }
        case "approvalRequested": {
          setPendingApprovals((current) => [...current.filter((approval) => approval.approvalId !== frame.approvalId), { approvalId: frame.approvalId, summary: frame.summary, requestedAtMs: Date.now() }]);
          setConversation((current) => appendConversationEntry(current, { kind: "approval", id: frame.approvalId, summary: frame.summary, state: "pending", decision: null, withdrawal: null, atMs: Date.now() }));
          break;
        }
        case "approvalResolved": {
          setPendingApprovals((current) => current.filter((approval) => approval.approvalId !== frame.approvalId));
          setConversation((current) => updateConversationEntry(current, frame.approvalId, (entry) => (entry.kind === "approval" ? { ...entry, state: "resolved", decision: frame.decision } : entry)));
          break;
        }
        case "approvalWithdrawn": {
          setPendingApprovals((current) => current.filter((approval) => approval.approvalId !== frame.approvalId));
          setConversation((current) => updateConversationEntry(current, frame.approvalId, (entry) => (entry.kind === "approval" && entry.state === "pending" ? { ...entry, state: "withdrawn", withdrawal: frame.reason } : entry)));
          break;
        }
        case "agentToolCall": {
          setConversation((current) => appendConversationEntry(current, { kind: "toolCall", id: frame.invocationId, toolName: frame.toolName, args: frame.arguments, state: "running", summary: null, atMs: Date.now() }));
          break;
        }
        case "agentToolResult": {
          setConversation((current) => updateConversationEntry(current, frame.invocationId, (entry) => (entry.kind === "toolCall" ? { ...entry, state: frame.ok ? "ok" : "failed", summary: frame.summary } : entry)));
          break;
        }
        case "agentReply": {
          setConversation((current) => {
            const existing = current.find((entry) => entry.kind === "agentMessage" && entry.id === frame.replyId);
            if (!existing) return appendConversationEntry(current, { kind: "agentMessage", id: frame.replyId, text: frame.text, state: frame.complete ? "complete" : "streaming", inReplyTo: frame.inReplyTo, atMs: Date.now() });
            return updateConversationEntry(current, frame.replyId, (entry) => (entry.kind === "agentMessage" ? { ...entry, text: `${entry.text}${frame.text}`, state: frame.complete ? "complete" : "streaming" } : entry));
          });
          break;
        }
        case "agentPresence": {
          setPresence({ active: frame.active, label: frame.label, invocationId: frame.invocationId });
          break;
        }
        case "pong":
          break;
        case "bye": {
          setLastError(frame.reason || null);
          break;
        }
        case "refused": {
          settle(frame.reason === "version" ? "incompatible" : "unavailable", frame.reason === "version" ? { gateway: frame.gatewayVersion, shell: BRIDGE_VERSION } : null);
          break;
        }
        case "appCommand": {
          let request: AgentAppCommandRequest;
          try {
            request = { seq: frame.seq, instanceId: frame.instanceId, command: decodeShellAppCommand(frame.command) };
          } catch (error) {
            send({ variant: "appFrames", inReplyTo: frame.seq, instanceId: frame.instanceId, frames: [encodeShellAppFrame(shellAppFault("channel.not-wired", error instanceof Error ? error.message : "malformed AppCommand payload"))] });
            break;
          }
          void answerAgentAppCommand(request, onAppCommandRef.current).then(send);
          break;
        }
      }
    };

    function connect(): void {
      if (disposed || settled) return;
      welcomed = false;
      setStatus(reconnectAttemptRef.current > 0 ? "reconnecting" : "connecting");
      let socket: WebSocket;
      try {
        socket = new WebSocket(admittedConfig.url, [...bridgeProtocols(admittedConfig)]);
      } catch (error) {
        setLastError(error instanceof Error ? error.message : "failed to open bridge socket");
        scheduleReconnect();
        return;
      }
      socket.binaryType = "arraybuffer";
      socketRef.current = socket;
      clearHandshakeTimer();
      handshakeTimer = setTimeout(() => {
        handshakeTimer = null;
        if (!welcomed && socketRef.current === socket) socket.close();
      }, BRIDGE_HANDSHAKE_DEADLINE_MS);

      socket.onopen = () => {
        if (disposed) return;
        send({ variant: "hello", bridgeVersion: BRIDGE_VERSION, shellKind, shellSessionId, principalActor, flags });
        pingTimerRef.current = setInterval(() => send({ variant: "ping" }), PING_INTERVAL_MS);
      };
      socket.onmessage = (event) => {
        if (disposed) return;
        try {
          const bytes = event.data instanceof ArrayBuffer ? new Uint8Array(event.data) : new Uint8Array(0);
          handleFrame(decodeGatewayToShell(bytes));
        } catch (error) {
          setLastError(error instanceof Error ? error.message : "failed to decode bridge frame");
        }
      };
      socket.onerror = () => {
        if (disposed) return;
        setLastError("bridge socket error");
      };
      socket.onclose = () => {
        socketRef.current = null;
        clearPingTimer();
        clearHandshakeTimer();
        if (disposed || settled) return;
        if (!welcomed) unanswered += 1;
        if (unanswered >= BRIDGE_UNANSWERED_ATTEMPTS) {
          settle("unavailable", null);
          return;
        }
        scheduleReconnect();
      };
    }

    connect();

    return () => {
      disposed = true;
      clearReconnectTimer();
      clearPingTimer();
      clearHandshakeTimer();
      const socket = socketRef.current;
      socketRef.current = null;
      if (socket) {
        socket.onopen = null;
        socket.onmessage = null;
        socket.onerror = null;
        socket.onclose = null;
        try {
          socket.send(encodeShellToGateway({ variant: "bye" }));
        } catch {}
        socket.close();
      }
      setStatus("disabled");
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [config?.url, config?.admissionProof, shellKind, shellSessionId, principalActor, send]);
}
