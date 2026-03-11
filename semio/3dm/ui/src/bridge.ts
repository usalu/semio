// #region 🔖Header
// [👤semio📚3dm🖱️ui🗃️src💻bridge](semiorepo://p/u/semio/b/u/3dm/fd/req/ui/fd/org/src/f/bridge.ts)

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Bridge client for JSON-RPC style communication with the native Rhino C# host.

// #endregion 🔖Header

// #region 🔖BridgeProtocol
// [👤semio📚3dm🖱️ui🗃️src💻bridge🔖bridgeprotocol](semiorepo://p/u/semio/b/u/3dm/fd/req/ui/fd/org/src/f/bridge.ts/s/BridgeProtocol)
// Bridge protocol types MUST match the C# BridgeRequest/BridgeResponse/BridgeEvent types.

export interface BridgeRequest {
  id: string;
  binding: string;
  method: string;
  params?: unknown;
}

export interface BridgeResponse {
  id: string;
  ok: boolean;
  result?: unknown;
  error?: {
    code: string;
    message: string;
    details?: unknown;
  };
}

export interface BridgeEvent {
  event: string;
  payload?: unknown;
}

// #endregion 🔖BridgeProtocol

// #region 🔖BridgeClient
// [👤semio📚3dm🖱️ui🗃️src💻bridge🔖bridgeclient](semiorepo://p/u/semio/b/u/3dm/fd/req/ui/fd/org/src/f/bridge.ts/s/BridgeClient)
// Bridge client MUST route all native calls through WebView2 postMessage.

let requestCounter = 0;
const pendingRequests = new Map<string, { resolve: (value: unknown) => void; reject: (error: Error) => void }>();
const eventListeners = new Map<string, Set<(payload: unknown) => void>>();

/**
 * Initializes the bridge message listener.
 * MUST be called once at app startup.
 */
export function initBridge(): void {
  window.chrome?.webview?.addEventListener("message", (event: MessageEvent) => {
    const data = event.data as BridgeResponse | BridgeEvent;

    if ("event" in data && !("id" in data)) {
      const evt = data as BridgeEvent;
      const listeners = eventListeners.get(evt.event);
      if (listeners) {
        for (const listener of listeners) {
          listener(evt.payload);
        }
      }
      return;
    }

    const response = data as BridgeResponse;
    const pending = pendingRequests.get(response.id);
    if (!pending) return;

    pendingRequests.delete(response.id);
    if (response.ok) {
      pending.resolve(response.result);
    } else {
      pending.reject(new Error(response.error?.message ?? "Unknown bridge error"));
    }
  });
}

/**
 * Sends a typed bridge request to the native host.
 * MUST return a promise that resolves with the typed result.
 */
export async function callBridge<T>(binding: string, method: string, params?: unknown): Promise<T> {
  const id = `req_${++requestCounter}`;
  const request: BridgeRequest = { id, binding, method, params };

  return new Promise<T>((resolve, reject) => {
    pendingRequests.set(id, {
      resolve: resolve as (value: unknown) => void,
      reject,
    });

    if (window.chrome?.webview) {
      window.chrome.webview.postMessage(request);
    } else {
      // Fallback for development outside WebView2
      pendingRequests.delete(id);
      reject(new Error(`Bridge not available: ${binding}.${method}`));
    }
  });
}

/**
 * Subscribes to a native bridge event.
 */
export function onBridgeEvent(event: string, callback: (payload: unknown) => void): () => void {
  if (!eventListeners.has(event)) {
    eventListeners.set(event, new Set());
  }
  eventListeners.get(event)!.add(callback);

  return () => {
    eventListeners.get(event)?.delete(callback);
  };
}

// #endregion 🔖BridgeClient

// #region 🔖TypedApis
// [👤semio📚3dm🖱️ui🗃️src💻bridge🔖typedapis](semiorepo://p/u/semio/b/u/3dm/fd/req/ui/fd/org/src/f/bridge.ts/s/TypedApis)
// Typed API wrappers MUST expose domain-specific operations only.

export const appApi = {
  ping: () => callBridge<string>("app", "ping"),
  getVersion: () => callBridge<string>("app", "getVersion"),
  getBridgeInfo: () => callBridge<{ protocolVersion: string; pluginVersion: string; rhinoVersion: string }>("app", "getBridgeInfo"),
};

export const documentApi = {
  getInfo: () => callBridge<{ name: string; path: string; isModified: boolean }>("document", "getInfo"),
  getUnits: () => callBridge<{ system: string }>("document", "getUnits"),
  getLayers: () => callBridge<Array<{ name: string; fullPath: string; id: string; color: string; visible: boolean }>>("document", "getLayers"),
};

export const importApi = {
  importModel: (params: {
    kitName: string;
    typeName: string;
    modelGuid: string;
    fileUrl: string;
    tags: string[];
  }) => callBridge<{ layerPath: string; objectCount: number }>("import", "importModel", params),
  openImportKitDialog: () => callBridge<{ dialogKind: string }>("import", "openImportKitDialog"),
};

// #endregion 🔖TypedApis

// #region 🔖WebViewGlobal
// [👤semio📚3dm🖱️ui🗃️src💻bridge🔖webviewglobal](semiorepo://p/u/semio/b/u/3dm/fd/req/ui/fd/org/src/f/bridge.ts/s/WebViewGlobal)
// Global type augmentation for the WebView2 chrome.webview API.

declare global {
  interface Window {
    chrome?: {
      webview?: {
        postMessage: (message: unknown) => void;
        addEventListener: (type: string, listener: (event: MessageEvent) => void) => void;
        removeEventListener: (type: string, listener: (event: MessageEvent) => void) => void;
      };
    };
  }
}

// #endregion 🔖WebViewGlobal
