# File Provider Implementation Guide

This document provides technical details about the file provider system implementation.

## Architecture Overview

The file provider system uses a composable architecture with three layers:

1. **Memory Layer**: Fast in-memory storage using `Map`
2. **Local Layer**: Persistent storage using IndexedDB
3. **Remote Layer**: Remote synchronization using REST API

Each layer can be used independently or combined for different use cases:

- **Temporary kits**: Memory only
- **Local kits**: Memory + Local
- **Remote kits**: Memory + Local + Remote

## Core Interfaces

### FileProvider

```typescript
interface FileProvider {
  upload: (kitId: string, fileId: string, path: string, blob: Blob) => Promise<string>;
  download: (kitId: string, fileId: string, path: string) => Promise<Blob>;
  delete: (kitId: string, fileId: string, path: string) => Promise<void>;
  getUrl: (kitId: string, fileId: string, path: string) => string;
}
```

**Parameters:**

- `kitId`: Unique identifier for the kit
- `fileId`: Unique identifier for the file
- `path`: Original filename with extension
- `blob`: File content as a Blob

**Methods:**

- `upload`: Stores a file and returns its URL
- `download`: Retrieves a file as a Blob
- `delete`: Removes a file
- `getUrl`: Returns a URL for the file (may be virtual)

### FileProviderFactory

```typescript
type FileProviderFactory = (kitId: string) => Promise<FileProvider>;
```

A factory function that creates a `FileProvider` instance for a specific kit. This allows lazy initialization and per-kit configuration.

## Implementation Details

### Memory File Provider

**Storage:** `Map<string, Blob>`

**Key Format:** `{kitId}/{fileId}/{path}`

**Characteristics:**

- ✅ Fast access (in-memory)
- ✅ No async overhead
- ❌ Lost on page reload
- ❌ Limited by browser memory

**Implementation:**

```typescript
export function createMemoryFileProvider(): FileProviderFactory {
  const storage = new Map<string, Blob>();

  return async (kitId: string) => ({
    upload: async (kitId, fileId, path, blob) => {
      const key = `${kitId}/${fileId}/${path}`;
      storage.set(key, blob);
      return `memory://${key}`;
    },
    // ... other methods
  });
}
```

### Local File Provider

**Storage:** IndexedDB via `indexedDB` API

**Database Schema:**

- Database: `semio-files` (configurable)
- Object Store: `files` (configurable)
- Key: `{kitId}/{fileId}/{path}`
- Value: `Blob`

**Characteristics:**

- ✅ Persistent across sessions
- ✅ Large storage quota (>1GB typical)
- ✅ Works offline
- ⚠️ Async overhead
- ⚠️ May be cleared by browser

**Implementation:**

```typescript
export function createLocalFileProvider(config?: LocalFileProviderConfig): FileProviderFactory {
  const dbName = config?.dbName || "semio-files";
  const storeName = config?.storeName || "files";

  const openDB = (): Promise<IDBDatabase> => {
    return new Promise((resolve, reject) => {
      const request = indexedDB.open(dbName, 1);
      request.onupgradeneeded = (event) => {
        const db = (event.target as IDBOpenDBRequest).result;
        if (!db.objectStoreNames.contains(storeName)) {
          db.createObjectStore(storeName);
        }
      };
      request.onsuccess = () => resolve(request.result);
      request.onerror = () => reject(request.error);
    });
  };

  return async (kitId: string) => ({
    upload: async (kitId, fileId, path, blob) => {
      const db = await openDB();
      const key = `${kitId}/${fileId}/${path}`;

      return new Promise<string>((resolve, reject) => {
        const tx = db.transaction([storeName], "readwrite");
        const store = tx.objectStore(storeName);
        const request = store.put(blob, key);

        request.onsuccess = () => resolve(`local://${key}`);
        request.onerror = () => reject(request.error);
        tx.oncomplete = () => db.close();
      });
    },
    // ... other methods
  });
}
```

### Remote File Provider

**Storage:** Remote server via REST API

**API Endpoints:**

- `POST /kits/{kitId}/files/{fileId}` - Upload (multipart/form-data)
- `GET /kits/{kitId}/files/{fileId}` - Download
- `DELETE /kits/{kitId}/files/{fileId}` - Delete

**Characteristics:**

- ✅ Collaborative (shared across users)
- ✅ Durable (server-side backups)
- ✅ Scalable (CDN, S3, etc.)
- ❌ Requires internet connection
- ⚠️ Network latency

**Implementation:**

```typescript
export function createRemoteFileProvider(config: RemoteFileProviderConfig): FileProviderFactory {
  return async (kitId: string) => ({
    upload: async (kitId, fileId, path, blob) => {
      const formData = new FormData();
      formData.append("file", blob, path);

      const response = await fetch(`${config.baseUrl}/kits/${kitId}/files/${fileId}`, {
        method: "POST",
        headers: config.headers,
        body: formData,
      });

      if (!response.ok) {
        throw new Error(`Remote upload failed: ${response.statusText}`);
      }

      const result = await response.json();
      return result.url;
    },
    // ... other methods
  });
}
```

### Composite File Provider

**Strategy:** Write to all, read from first available

**Characteristics:**

- ✅ Best of all worlds
- ✅ Offline support (local fallback)
- ✅ Fast access (memory cache)
- ✅ Durability (remote sync)
- ⚠️ More complex error handling

**Implementation:**

```typescript
export function createCompositeFileProvider(config: CompositeFileProviderConfig): FileProviderFactory {
  return async (kitId: string) => {
    const providers: FileProvider[] = [];

    if (config.memory) {
      providers.push(await createMemoryFileProvider()(kitId));
    }
    if (config.local) {
      const localConfig = typeof config.local === "object" ? config.local : undefined;
      providers.push(await createLocalFileProvider(localConfig)(kitId));
    }
    if (config.remote) {
      providers.push(await createRemoteFileProvider(config.remote)(kitId));
    }

    return {
      upload: async (kitId, fileId, path, blob) => {
        // Write to all providers in parallel
        const results = await Promise.allSettled(providers.map((p) => p.upload(kitId, fileId, path, blob)));

        // Succeed if at least one succeeds
        const successful = results.filter((r) => r.status === "fulfilled");
        if (successful.length === 0) {
          throw new Error(`All providers failed to upload file ${path}`);
        }

        // Return last successful URL (remote > local > memory)
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
        // Delete from all providers (best effort)
        await Promise.allSettled(providers.map((p) => p.delete(kitId, fileId, path)));
      },

      getUrl: (kitId, fileId, path) => {
        // Return URL from last provider (remote if available)
        return providers[providers.length - 1].getUrl(kitId, fileId, path);
      },
    };
  };
}
```

## Integration with Sketchpad

### Props

```typescript
interface SketchpadProps {
  fileProviderFactory?: FileProviderFactory;
}
```

The `fileProviderFactory` prop is optional. If not provided, file operations will fail gracefully.

### KitStore Integration

```typescript
class KitStore {
  private fileProviderFactory?: FileProviderFactory;
  private fileProvider?: FileProvider;

  constructor(fileProviderFactory?: FileProviderFactory) {
    this.fileProviderFactory = fileProviderFactory;
  }

  async initializeFileProvider(kitId: string) {
    if (this.fileProviderFactory) {
      this.fileProvider = await this.fileProviderFactory(kitId);
      await this.syncFiles();
    }
  }

  private async syncFiles() {
    const files = this.kit?.files || [];
    await Promise.all(
      files.map(async (file) => {
        try {
          const blob = await this.fileProvider!.download(this.kitId, file.id, file.path);
          // File is now cached in memory/local
        } catch (error) {
          console.warn(`Failed to sync file ${file.path}:`, error);
        }
      }),
    );
  }

  async executeCommand(command: string, ...args: any[]) {
    if (command === "addFile") {
      const [path, blob] = args;
      const fileId = generateId();
      const url = await this.fileProvider!.upload(this.kitId, fileId, path, blob);

      // Add file to kit
      this.kit.files.push({ id: fileId, path, url });
    }
    // ... other commands
  }
}
```

## File Lifecycle

### Upload Flow

1. User drops file on canvas
2. `handleDrop` event handler called
3. `executeCommand("addFile", filename, blob)`
4. File uploaded to all configured providers
5. File metadata added to kit's Y.js document
6. URL stored for later retrieval

### Download Flow

1. Kit loaded with file references
2. `initializeFileProvider()` called
3. `syncFiles()` downloads all files
4. Files cached in memory/local for fast access
5. Components use `URL.createObjectURL()` for display

### Delete Flow

1. User deletes file from kit
2. `executeCommand("deleteFile", fileId)`
3. File deleted from all configured providers
4. File metadata removed from kit's Y.js document

## Error Handling

### Upload Errors

```typescript
try {
  const url = await fileProvider.upload(kitId, fileId, path, blob);
} catch (error) {
  console.error("Upload failed:", error);
  // Show user-friendly error
  // Optionally retry or fall back to local-only
}
```

### Download Errors

```typescript
// Composite provider automatically falls back to next provider
try {
  const blob = await fileProvider.download(kitId, fileId, path);
} catch (error) {
  console.error("All providers failed to download:", error);
  // Show error to user
}
```

### Network Errors

```typescript
// Remote provider should handle network errors
if (!navigator.onLine) {
  // Skip remote upload, rely on local
  console.warn("Offline: using local storage only");
}
```

## Best Practices

### 1. Always Use Composite Provider

```typescript
// ✅ Good
const fileProviderFactory = createCompositeFileProvider({
  memory: true,
  local: true,
  remote: { baseUrl: "..." },
});

// ❌ Bad (no offline support)
const fileProviderFactory = createRemoteFileProvider({ baseUrl: "..." });
```

### 2. Include All Three Layers for Remote Kits

```typescript
// ✅ Good
createCompositeFileProvider({
  memory: true, // Fast access
  local: true, // Offline support
  remote: {
    /* ... */
  }, // Collaboration
});

// ❌ Bad (no caching)
createCompositeFileProvider({
  remote: {
    /* ... */
  },
});
```

### 3. Clean Up Blob URLs

```typescript
// ✅ Good
const blobUrl = URL.createObjectURL(blob);
// Use the URL
URL.revokeObjectURL(blobUrl); // Clean up

// ❌ Bad (memory leak)
const blobUrl = URL.createObjectURL(blob);
// Never revoked
```

### 4. Validate Files Before Upload

```typescript
// ✅ Good
if (file.size > 100 * 1024 * 1024) {
  throw new Error("File too large (max 100MB)");
}
if (!allowedTypes.includes(file.type)) {
  throw new Error("File type not allowed");
}

// ❌ Bad (no validation)
await fileProvider.upload(kitId, fileId, path, blob);
```

## Testing

### Unit Tests

```typescript
describe("MemoryFileProvider", () => {
  it("should upload and download files", async () => {
    const provider = await createMemoryFileProvider()("test-kit");
    const blob = new Blob(["test"], { type: "text/plain" });

    const url = await provider.upload("test-kit", "file-1", "test.txt", blob);
    expect(url).toBe("memory://test-kit/file-1/test.txt");

    const downloaded = await provider.download("test-kit", "file-1", "test.txt");
    expect(downloaded.size).toBe(blob.size);
  });
});
```

### Integration Tests

```typescript
describe("CompositeFileProvider", () => {
  it("should fall back to local when remote fails", async () => {
    const provider = await createCompositeFileProvider({
      memory: true,
      local: true,
      remote: { baseUrl: "http://invalid" }, // Will fail
    })("test-kit");

    const blob = new Blob(["test"], { type: "text/plain" });

    // Should succeed with local even if remote fails
    await provider.upload("test-kit", "file-1", "test.txt", blob);

    // Should download from local
    const downloaded = await provider.download("test-kit", "file-1", "test.txt");
    expect(downloaded.size).toBe(blob.size);
  });
});
```

## Performance Considerations

### Memory Usage

- Memory provider: One Blob per file in memory
- Local provider: IndexedDB has large quota (>1GB typical)
- Composite provider: Files duplicated in memory + local

### Network Usage

- Files uploaded in parallel to all providers
- Large files may benefit from chunked uploads
- Consider compression for large files

### Caching Strategy

1. Memory: Always check first (fastest)
2. Local: Check if not in memory (still fast)
3. Remote: Only if not cached (slowest)

## Security Considerations

### Authentication

```typescript
const fileProviderFactory = createCompositeFileProvider({
  remote: {
    baseUrl: "https://api.example.com",
    headers: {
      Authorization: `Bearer ${token}`,
    },
  },
});
```

### File Validation

```typescript
// Validate on client
if (!isValidFileType(file.type)) {
  throw new Error("Invalid file type");
}

// Validate on server
app.post("/kits/:kitId/files/:fileId", (req, res) => {
  if (!isValidFileType(req.file.mimetype)) {
    return res.status(400).json({ error: "Invalid file type" });
  }
  // ... upload file
});
```

### Access Control

Implement in your remote provider:

```typescript
// Backend
app.get("/kits/:kitId/files/:fileId", authenticate, authorize, (req, res) => {
  if (!canAccessKit(req.user, req.params.kitId)) {
    return res.status(403).json({ error: "Forbidden" });
  }
  // ... serve file
});
```
