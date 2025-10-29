// #region Header

// providers.ts

// Core file provider implementations

// #endregion

import type { FileProvider, FileProviderFactory } from "../store";

// #region Memory File Provider

export interface MemoryFileProviderConfig { }

/**
 * Creates an in-memory file provider.
 * Files are stored in memory using Map and will be lost on page reload.
 * Used for temporary kits.
 */
export function createMemoryFileProvider(config?: MemoryFileProviderConfig): FileProviderFactory {
    const storage = new Map<string, Blob>();

    return async (kitId: string): Promise<FileProvider> => {
        const getKey = (kitId: string, fileId: string, path: string): string => {
            return `${kitId}/${fileId}/${path}`;
        };

        return {
            upload: async (kitId, fileId, path, blob) => {
                const key = getKey(kitId, fileId, path);
                storage.set(key, blob);
                console.log(`[MEMORY] Uploaded file ${path} (${blob.size} bytes)`);
                return `memory://${key}`;
            },

            download: async (kitId, fileId, path) => {
                const key = getKey(kitId, fileId, path);
                const blob = storage.get(key);

                if (!blob) {
                    throw new Error(`File not found in memory: ${key}`);
                }

                console.log(`[MEMORY] Downloaded file ${path} (${blob.size} bytes)`);
                return blob;
            },

            delete: async (kitId, fileId, path) => {
                const key = getKey(kitId, fileId, path);
                storage.delete(key);
                console.log(`[MEMORY] Deleted file ${path}`);
            },

            getUrl: (kitId, fileId, path) => {
                return `memory://${getKey(kitId, fileId, path)}`;
            },
        };
    };
}

// #endregion Memory File Provider

// #region Local File Provider (IndexedDB)

export interface LocalFileProviderConfig {
    dbName?: string;
    storeName?: string;
}

/**
 * Creates a local file provider using IndexedDB.
 * Files are persisted locally in the browser.
 * Used for local kits.
 */
export function createLocalFileProvider(config?: LocalFileProviderConfig): FileProviderFactory {
    const dbName = config?.dbName || "semio-files";
    const storeName = config?.storeName || "files";

    const openDB = (): Promise<IDBDatabase> => {
        return new Promise((resolve, reject) => {
            const request = indexedDB.open(dbName, 1);

            request.onerror = () => reject(request.error);
            request.onsuccess = () => resolve(request.result);

            request.onupgradeneeded = (event) => {
                const db = (event.target as IDBOpenDBRequest).result;
                if (!db.objectStoreNames.contains(storeName)) {
                    db.createObjectStore(storeName);
                }
            };
        });
    };

    return async (kitId: string): Promise<FileProvider> => {
        const getKey = (kitId: string, fileId: string, path: string): string => {
            return `${kitId}/${fileId}/${path}`;
        };

        return {
            upload: async (kitId, fileId, path, blob) => {
                const key = getKey(kitId, fileId, path);
                const db = await openDB();

                return new Promise<string>((resolve, reject) => {
                    const transaction = db.transaction([storeName], "readwrite");
                    const store = transaction.objectStore(storeName);
                    const request = store.put(blob, key);

                    request.onsuccess = () => {
                        console.log(`[LOCAL] Uploaded file ${path} (${blob.size} bytes)`);
                        resolve(`local://${key}`);
                    };
                    request.onerror = () => reject(request.error);

                    transaction.oncomplete = () => db.close();
                });
            },

            download: async (kitId, fileId, path) => {
                const key = getKey(kitId, fileId, path);
                const db = await openDB();

                return new Promise<Blob>((resolve, reject) => {
                    const transaction = db.transaction([storeName], "readonly");
                    const store = transaction.objectStore(storeName);
                    const request = store.get(key);

                    request.onsuccess = () => {
                        const blob = request.result;
                        if (!blob) {
                            reject(new Error(`File not found in IndexedDB: ${key}`));
                        } else {
                            console.log(`[LOCAL] Downloaded file ${path} (${blob.size} bytes)`);
                            resolve(blob);
                        }
                    };
                    request.onerror = () => reject(request.error);

                    transaction.oncomplete = () => db.close();
                });
            },

            delete: async (kitId, fileId, path) => {
                const key = getKey(kitId, fileId, path);
                const db = await openDB();

                return new Promise<void>((resolve, reject) => {
                    const transaction = db.transaction([storeName], "readwrite");
                    const store = transaction.objectStore(storeName);
                    const request = store.delete(key);

                    request.onsuccess = () => {
                        console.log(`[LOCAL] Deleted file ${path}`);
                        resolve();
                    };
                    request.onerror = () => reject(request.error);

                    transaction.oncomplete = () => db.close();
                });
            },

            getUrl: (kitId, fileId, path) => {
                return `local://${getKey(kitId, fileId, path)}`;
            },
        };
    };
}

// #endregion Local File Provider

// #region Remote File Provider

export interface RemoteFileProviderConfig {
    baseUrl: string;
    headers?: Record<string, string>;
}

/**
 * Creates a remote file provider using HTTP/REST API.
 * Files are synchronized with a remote server.
 * Used for remote kits.
 */
export function createRemoteFileProvider(config: RemoteFileProviderConfig): FileProviderFactory {
    return async (kitId: string): Promise<FileProvider> => {
        const getUrl = (kitId: string, fileId: string, path: string): string => {
            return `${config.baseUrl}/kits/${kitId}/files/${fileId}`;
        };

        const headers = {
            ...config.headers,
        };

        return {
            upload: async (kitId, fileId, path, blob) => {
                const formData = new FormData();
                formData.append("file", blob, path);

                const response = await fetch(getUrl(kitId, fileId, path), {
                    method: "POST",
                    headers,
                    body: formData,
                });

                if (!response.ok) {
                    throw new Error(`Remote upload failed: ${response.statusText}`);
                }

                const result = await response.json();
                console.log(`[REMOTE] Uploaded file ${path} (${blob.size} bytes)`);
                return result.url || getUrl(kitId, fileId, path);
            },

            download: async (kitId, fileId, path) => {
                const response = await fetch(getUrl(kitId, fileId, path), {
                    method: "GET",
                    headers,
                });

                if (!response.ok) {
                    throw new Error(`Remote download failed: ${response.statusText}`);
                }

                const blob = await response.blob();
                console.log(`[REMOTE] Downloaded file ${path} (${blob.size} bytes)`);
                return blob;
            },

            delete: async (kitId, fileId, path) => {
                const response = await fetch(getUrl(kitId, fileId, path), {
                    method: "DELETE",
                    headers,
                });

                if (!response.ok) {
                    throw new Error(`Remote delete failed: ${response.statusText}`);
                }

                console.log(`[REMOTE] Deleted file ${path}`);
            },

            getUrl: (kitId, fileId, path) => {
                return getUrl(kitId, fileId, path);
            },
        };
    };
}

// #endregion Remote File Provider

// #region Composite File Provider

export interface CompositeFileProviderConfig {
    memory?: boolean;
    local?: boolean | LocalFileProviderConfig;
    remote?: RemoteFileProviderConfig;
}

/**
 * Creates a composite file provider that combines memory, local, and remote storage.
 * This is the recommended way to create file providers.
 * 
 * Behavior:
 * - memory only: Files in memory, lost on reload (temporary kits)
 * - memory + local: Files persisted locally (local kits)
 * - memory + local + remote: Files synced to remote, persisted locally (remote kits)
 * 
 * @example
 * // Temporary kit
 * createCompositeFileProvider({ memory: true })
 * 
 * // Local kit
 * createCompositeFileProvider({ memory: true, local: true })
 * 
 * // Remote kit
 * createCompositeFileProvider({ 
 *   memory: true, 
 *   local: true, 
 *   remote: { baseUrl: 'https://api.example.com' }
 * })
 */
export function createCompositeFileProvider(config: CompositeFileProviderConfig): FileProviderFactory {
    return async (kitId: string): Promise<FileProvider> => {
        const providers: FileProvider[] = [];

        // Initialize providers in order: memory, local, remote
        if (config.memory) {
            const memoryProvider = await createMemoryFileProvider()(kitId);
            providers.push(memoryProvider);
        }

        if (config.local) {
            const localConfig = typeof config.local === "object" ? config.local : undefined;
            const localProvider = await createLocalFileProvider(localConfig)(kitId);
            providers.push(localProvider);
        }

        if (config.remote) {
            const remoteProvider = await createRemoteFileProvider(config.remote)(kitId);
            providers.push(remoteProvider);
        }

        if (providers.length === 0) {
            throw new Error("At least one provider must be configured");
        }

        // Composite provider that writes to all providers and reads from the first available
        return {
            upload: async (kitId, fileId, path, blob) => {
                // Write to all providers in parallel
                const results = await Promise.allSettled(
                    providers.map((p) => p.upload(kitId, fileId, path, blob))
                );

                // Log any errors but don't fail if at least one succeeds
                const successful = results.filter((r) => r.status === "fulfilled");
                if (successful.length === 0) {
                    throw new Error(`All providers failed to upload file ${path}`);
                }

                // Return the last successful URL (remote if available, otherwise local/memory)
                const lastSuccessful = results.reverse().find((r) => r.status === "fulfilled") as PromiseFulfilledResult<string>;
                return lastSuccessful.value;
            },

            download: async (kitId, fileId, path) => {
                // Try providers in order: memory > local > remote
                for (const provider of providers) {
                    try {
                        return await provider.download(kitId, fileId, path);
                    } catch (error) {
                        console.warn(`Provider failed to download ${path}, trying next:`, error);
                    }
                }
                throw new Error(`All providers failed to download file ${path}`);
            },

            delete: async (kitId, fileId, path) => {
                // Delete from all providers in parallel
                await Promise.allSettled(
                    providers.map((p) => p.delete(kitId, fileId, path))
                );
            },

            getUrl: (kitId, fileId, path) => {
                // Return URL from the last provider (remote if available)
                return providers[providers.length - 1].getUrl(kitId, fileId, path);
            },
        };
    };
}

// #endregion Composite File Provider
