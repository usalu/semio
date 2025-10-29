// #region Header

// example.tsx

// File provider usage examples

// #endregion

import { Sketchpad, createCompositeFileProvider, createLocalFileProvider, createMemoryFileProvider, createRemoteFileProvider } from "@semio/js";

// #region Example 1: Temporary Kit (Memory Only)

/**
 * Files are stored in memory and lost on page reload.
 * Perfect for temporary/demo kits.
 */
export function TemporaryKitExample() {
  const fileProviderFactory = createCompositeFileProvider({
    memory: true,
  });

  return <Sketchpad fileProviderFactory={fileProviderFactory} />;
}

// #endregion Example 1

// #region Example 2: Local Kit (Memory + Local)

/**
 * Files are persisted in IndexedDB and survive page reloads.
 * Perfect for offline-first local kits.
 */
export function LocalKitExample() {
  const fileProviderFactory = createCompositeFileProvider({
    memory: true,
    local: true,
  });

  return <Sketchpad fileProviderFactory={fileProviderFactory} />;
}

// #endregion Example 2

// #region Example 3: Remote Kit (Memory + Local + Remote)

/**
 * Files are synced to a remote server but also cached locally.
 * Works offline and syncs when connection is restored.
 */
export function RemoteKitExample() {
  const fileProviderFactory = createCompositeFileProvider({
    memory: true,
    local: true,
    remote: {
      baseUrl: "https://api.example.com",
      headers: {
        Authorization: `Bearer ${localStorage.getItem("token")}`,
      },
    },
  });

  return <Sketchpad fileProviderFactory={fileProviderFactory} />;
}

// #endregion Example 3

// #region Example 4: Custom Local Configuration

/**
 * Customize IndexedDB database and store names.
 */
export function CustomLocalKitExample() {
  const fileProviderFactory = createCompositeFileProvider({
    memory: true,
    local: {
      dbName: "my-app-files",
      storeName: "kit-files",
    },
  });

  return <Sketchpad fileProviderFactory={fileProviderFactory} />;
}

// #endregion Example 4

// #region Example 5: Remote-Only (No Local Cache)

/**
 * Files are only stored remotely (not recommended).
 * No offline support.
 */
export function RemoteOnlyExample() {
  const fileProviderFactory = createCompositeFileProvider({
    remote: {
      baseUrl: "https://api.example.com",
      headers: {
        Authorization: `Bearer ${localStorage.getItem("token")}`,
      },
    },
  });

  return <Sketchpad fileProviderFactory={fileProviderFactory} />;
}

// #endregion Example 5

// #region Example 6: Using Individual Providers

/**
 * Use individual providers directly (not recommended).
 * Prefer the composite provider for production use.
 */
export function IndividualProvidersExample() {
  // Memory only
  const memoryProvider = createMemoryFileProvider();

  // Local only
  const localProvider = createLocalFileProvider({
    dbName: "semio-files",
    storeName: "files",
  });

  // Remote only
  const remoteProvider = createRemoteFileProvider({
    baseUrl: "https://api.example.com",
    headers: {
      Authorization: `Bearer ${localStorage.getItem("token")}`,
    },
  });

  // Use one of them
  return <Sketchpad fileProviderFactory={memoryProvider} />;
}

// #endregion Example 6

// #region Example 7: Dynamic Configuration

/**
 * Choose provider based on environment or user preferences.
 */
export function DynamicConfigExample() {
  const isDevelopment = process.env.NODE_ENV === "development";
  const isOfflineMode = navigator.onLine === false;
  const hasBackend = !!process.env.REACT_APP_API_URL;

  const fileProviderFactory = createCompositeFileProvider({
    memory: true,
    local: !isDevelopment, // Only use local storage in production
    remote:
      hasBackend && !isOfflineMode
        ? {
            baseUrl: process.env.REACT_APP_API_URL!,
            headers: {
              Authorization: `Bearer ${localStorage.getItem("token")}`,
            },
          }
        : undefined,
  });

  return <Sketchpad fileProviderFactory={fileProviderFactory} />;
}

// #endregion Example 7

// #region Example 8: With Drag and Drop

/**
 * Complete example with drag-and-drop file uploads.
 * The Kit App already handles this, but here's how it works.
 */
export function DragDropExample() {
  const fileProviderFactory = createCompositeFileProvider({
    memory: true,
    local: true,
    remote: {
      baseUrl: "https://api.example.com",
    },
  });

  return (
    <div>
      <h1>My Kits</h1>
      <Sketchpad fileProviderFactory={fileProviderFactory} />
      <p>Drop files on the canvas to upload them to the kit</p>
    </div>
  );
}

// #endregion Example 8

// #region Example 9: Recommended Production Setup

/**
 * Recommended setup for production:
 * - Memory for fast access
 * - Local for offline support
 * - Remote for collaboration
 */
export function ProductionExample() {
  const fileProviderFactory = createCompositeFileProvider({
    memory: true,
    local: true,
    remote: {
      baseUrl: process.env.REACT_APP_API_URL || "https://api.example.com",
      headers: {
        Authorization: `Bearer ${localStorage.getItem("token")}`,
        "X-Client-Version": process.env.REACT_APP_VERSION || "1.0.0",
      },
    },
  });

  return <Sketchpad fileProviderFactory={fileProviderFactory} />;
}

// #endregion Example 9
