/** 🧪️ Handshake ladder: 3 tests covering useAgentBridge handshake; dials under the fixture's own constants; a different offer restarts the ladder after a terminal state. */
// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/AgentBridge/🤝️handshake-ladder.test.ts
/** 🤝️ AgentBridge handshake: the bounded dial ladder, the typed refusals and the foreign welcome, replayed from the language-neutral handshake fixture. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it, vi } from "vitest";
import { type AgentBridgeConfig, useAgentBridge } from "../../🟦️.tsx";
import { encodeGatewayToShell, type GatewayToShell } from "@semio-tech/framework/mcp/bridge";
import { BRIDGE_HANDSHAKE_DEADLINE_MS, BRIDGE_UNANSWERED_ATTEMPTS, type AgentBridgeStatus, type AgentBridgeVersionMismatch } from "../../🟢️shared/🟦️.tsx";
import { BRIDGE_VERSION } from "@semio-tech/framework/mcp/bridge";
// #endregion 🔌️Adapters

const here = dirname(fileURLToPath(import.meta.url));

type HandshakeDial = "silence" | "close" | { readonly frame: GatewayToShell; readonly thenClose?: boolean };
const handshakeFixture = JSON.parse(readFileSync(join(here, "../🧫️fixtures/🤝️handshake/🔣️.json"), "utf8")) as {
  readonly handshakeDeadlineMs: number;
  readonly unansweredAttempts: number;
  readonly shellVersion: number;
  readonly scenarios: readonly { readonly name: string; readonly dials: readonly HandshakeDial[]; readonly status: AgentBridgeStatus; readonly versionMismatch: AgentBridgeVersionMismatch | null }[];
};

//#region 🤝️Handshake
/** 🤝️ Ticket `26/09/23` G10 bridge item: every dialled gateway either answers the handshake or is
 * given up on. The scenarios in `🧫️fixtures/🤝️handshake/🔣️.json` (replayed by the wgpu twin's dialer
 * too) pin the bounded ladder: a silent or refusing gateway ends `unavailable` after
 * {@link BRIDGE_UNANSWERED_ATTEMPTS} dials, a typed refusal or a foreign welcome ends the offer at
 * once, and nothing is dialled again nor logged to the console afterwards. */
describe("useAgentBridge handshake", () => {
  it("dials under the fixture's own constants", () => {
    expect([BRIDGE_HANDSHAKE_DEADLINE_MS, BRIDGE_UNANSWERED_ATTEMPTS, BRIDGE_VERSION]).toEqual([handshakeFixture.handshakeDeadlineMs, handshakeFixture.unansweredAttempts, handshakeFixture.shellVersion]);
  });

  for (const scenario of handshakeFixture.scenarios) {
    it(scenario.name, async () => {
      const { renderHook, act } = await import("@testing-library/react");
      vi.useFakeTimers();
      const consoleError = vi.spyOn(console, "error");
      const consoleWarn = vi.spyOn(console, "warn");
      const sockets: Socket[] = [];
      class Socket {
        static CONNECTING = 0;
        static OPEN = 1;
        static CLOSED = 3;
        readyState = 0;
        onopen: (() => void) | null = null;
        onmessage: ((event: { data: ArrayBuffer }) => void) | null = null;
        onerror: (() => void) | null = null;
        onclose: (() => void) | null = null;
        binaryType = "arraybuffer";
        constructor() {
          sockets.push(this);
        }
        send(): void {}
        close(): void {
          if (this.readyState === 3) return;
          this.readyState = 3;
          setTimeout(() => this.onclose?.(), 0);
        }
        drop(): void {
          this.readyState = 3;
          this.onerror?.();
          this.onclose?.();
        }
      }
      vi.stubGlobal("WebSocket", Socket);
      const config: AgentBridgeConfig = { url: "ws://127.0.0.1:6300/bridge", admissionProof: "session.v1.handshake.proof" };
      const hook = renderHook(() => useAgentBridge({ config }));
      try {
        for (const [index, dial] of scenario.dials.entries()) {
          while (sockets.length <= index) await act(async () => void (await vi.advanceTimersToNextTimerAsync()));
          const socket = sockets[index]!;
          if (dial === "close") {
            act(() => socket.drop());
            continue;
          }
          act(() => {
            socket.readyState = 1;
            socket.onopen?.();
          });
          if (dial === "silence") {
            await act(async () => void (await vi.advanceTimersByTimeAsync(handshakeFixture.handshakeDeadlineMs + 1)));
            continue;
          }
          const wire = encodeGatewayToShell(dial.frame);
          act(() => socket.onmessage?.({ data: wire.buffer.slice(wire.byteOffset, wire.byteOffset + wire.byteLength) as ArrayBuffer }));
          if (dial.thenClose) act(() => socket.drop());
        }
        await act(async () => void (await vi.advanceTimersByTimeAsync(10 * 30_000)));
        expect(sockets.length).toBe(scenario.dials.length);
        expect(hook.result.current.status).toBe(scenario.status);
        expect(hook.result.current.versionMismatch).toEqual(scenario.versionMismatch);
        expect(consoleError).not.toHaveBeenCalled();
        expect(consoleWarn).not.toHaveBeenCalled();
      } finally {
        hook.unmount();
        consoleError.mockRestore();
        consoleWarn.mockRestore();
        vi.useRealTimers();
        vi.unstubAllGlobals();
      }
    });
  }

  it("a different offer restarts the ladder after a terminal state", async () => {
    const { renderHook, act } = await import("@testing-library/react");
    const sockets: { onopen: (() => void) | null; onmessage: ((event: { data: ArrayBuffer }) => void) | null; readyState: number }[] = [];
    class Socket {
      static OPEN = 1;
      readyState = 0;
      onopen: (() => void) | null = null;
      onmessage: ((event: { data: ArrayBuffer }) => void) | null = null;
      onerror: (() => void) | null = null;
      onclose: (() => void) | null = null;
      binaryType = "arraybuffer";
      constructor() {
        sockets.push(this);
      }
      send(): void {}
      close(): void {}
    }
    vi.stubGlobal("WebSocket", Socket);
    const refused = encodeGatewayToShell({ variant: "refused", reason: "version", gatewayVersion: 2 });
    const initialProps = { config: { url: "ws://127.0.0.1:6300/bridge", admissionProof: "session.v1.old" } as AgentBridgeConfig };
    const hook = renderHook(({ config }) => useAgentBridge({ config }), { initialProps });
    try {
      act(() => sockets[0]!.onmessage?.({ data: refused.buffer.slice(refused.byteOffset, refused.byteOffset + refused.byteLength) as ArrayBuffer }));
      expect(hook.result.current.status).toBe("incompatible");
      hook.rerender({ config: { url: "ws://127.0.0.1:6301/bridge", admissionProof: "session.v1.new" } });
      expect(sockets.length).toBe(2);
      expect(hook.result.current.status).toBe("connecting");
      expect(hook.result.current.versionMismatch).toBeNull();
    } finally {
      hook.unmount();
      vi.unstubAllGlobals();
    }
  });
});
//#endregion 🤝️Handshake
