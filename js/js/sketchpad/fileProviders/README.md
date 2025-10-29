# File Provider System

The file provider system allows Sketchpad to manage large files outside of the Y.js document, with a composable architecture that supports memory, local, and remote storage.

## Overview

Files in Semio kits can be large (3D models, images, documents) and should not be stored directly in the Y.js document. Instead, files are:

1. **Stored separately** using a file provider
2. **Referenced** in the kit's Y.js document by ID and path
3. **Consumed** by components using `URL.createObjectURL()` for blob URLs

## Architecture

The file provider system is **composable**: memory → local → remote

- **Memory only**: Files in memory, lost on reload (temporary kits)
- **Memory + Local**: Files persisted in IndexedDB (local kits)
- **Memory + Local + Remote**: Files synced to server, persisted locally (remote kits)

```typescript
interface FileProvider {
  upload: (kitId: string, fileId: string, path: string, blob: Blob) => Promise<string>;
  download: (kitId: string, fileId: string, path: string) => Promise<Blob>;
  delete: (kitId: string, fileId: string, path: string) => Promise<void>;
  getUrl: (kitId: string, fileId: string, path: string) => string;
}

type FileProviderFactory = (kitId: string) => Promise<FileProvider>;
```

## Usage

### Temporary Kit (Memory Only)

```tsx
import { Sketchpad, createCompositeFileProvider } from "@semio/js";

function App() {
  const fileProviderFactory = createCompositeFileProvider({
    memory: true,
  });

  return <Sketchpad fileProviderFactory={fileProviderFactory} />;
}
```

### Local Kit (Memory + Local)

```tsx
import { Sketchpad, createCompositeFileProvider } from "@semio/js";

function App() {
  const fileProviderFactory = createCompositeFileProvider({
    memory: true,
    local: true,
  });

  return <Sketchpad fileProviderFactory={fileProviderFactory} />;
}
```

### Remote Kit (Memory + Local + Remote)

```tsx
import { Sketchpad, createCompositeFileProvider } from "@semio/js";

function App() {
  const fileProviderFactory = createCompositeFileProvider({
    memory: true,
    local: true,
    remote: {
      baseUrl: "https://api.example.com",
      headers: {
        Authorization: `Bearer ${token}`,
      },
    },
  });

  return <Sketchpad fileProviderFactory={fileProviderFactory} />;
}
```

## Built-in Providers

### Composite File Provider (Recommended)

The composite provider is the recommended way to create file providers. It automatically manages the composition of memory, local, and remote storage:

```typescript
const fileProviderFactory = createCompositeFileProvider({
  memory: true, // Fast access
  local: true, // Persistent across sessions
  remote: {
    // Synchronized with server
    baseUrl: "https://api.example.com",
    headers: {
      /* auth */
    },
  },
});
```

**Behavior:**

- **Upload**: Writes to all configured providers in parallel
- **Download**: Reads from first available (memory → local → remote)
- **Delete**: Removes from all providers
- **Works offline**: Uses local storage when remote is unavailable

### Memory File Provider

Stores files in memory using `Map`:

```typescript
const fileProviderFactory = createMemoryFileProvider();
```

**Use cases:**

- Temporary kits
- Testing and development
- Preview/demo environments

### Local File Provider

Stores files in IndexedDB:

```typescript
const fileProviderFactory = createLocalFileProvider({
  dbName: "semio-files",
  storeName: "files",
});
```

**Use cases:**

- Local kits
- Offline-first applications
- Persistent storage without backend

### Remote File Provider

Syncs files with a REST API:

```typescript
const fileProviderFactory = createRemoteFileProvider({
  baseUrl: "https://api.example.com",
  headers: {
    Authorization: "Bearer ...",
  },
});
```

**API Contract:**

- `POST /kits/{kitId}/files/{fileId}` - Upload file (multipart/form-data)
- `GET /kits/{kitId}/files/{fileId}` - Download file
- `DELETE /kits/{kitId}/files/{fileId}` - Delete file

## File Operations

### Adding Files

Files are automatically handled when added to a kit:

1. User drops files onto the Kit App canvas
2. Files are uploaded using the file provider
3. File metadata (ID, path, URL) is added to the kit's Y.js document
4. Remote URL is stored for later retrieval

### Syncing Files

Files are automatically synced when a kit is loaded:

```typescript
// In KitStore.initializeFileProvider()
await this.syncFiles();

// Downloads all kit files from the provider
private async syncFiles() {
  const files = this.kit?.files || [];
  await Promise.all(
    files.map(async (file) => {
      const blob = await this.fileProvider.download(this.kitId, file.id, file.path);
      // File is now available locally (in memory and/or IndexedDB)
    })
  );
}
```

### Consuming Files

Components can use files via blob URLs:

```typescript
// In a component
const fileUrl = kit.files[0].url; // From file provider
const blobUrl = URL.createObjectURL(blob); // For local display

// Later, revoke the URL to free memory
URL.revokeObjectURL(blobUrl);
```

## Custom Providers

### Implementing a Custom Remote Provider

The remote provider expects a REST API. Here's an example backend implementation:

```typescript
// Backend (Express)
app.post("/kits/:kitId/files/:fileId", upload.single("file"), async (req, res) => {
  const { kitId, fileId } = req.params;
  const file = req.file;

  // Store file (S3, filesystem, etc.)
  await storage.upload(`${kitId}/${fileId}`, file.buffer);

  res.json({
    url: `https://cdn.example.com/${kitId}/${fileId}`,
  });
});

app.get("/kits/:kitId/files/:fileId", async (req, res) => {
  const { kitId, fileId } = req.params;
  const buffer = await storage.download(`${kitId}/${fileId}`);
  res.send(buffer);
});

app.delete("/kits/:kitId/files/:fileId", async (req, res) => {
  const { kitId, fileId } = req.params;
  await storage.delete(`${kitId}/${fileId}`);
  res.sendStatus(204);
});
```

### Example: S3 Backend

For S3 storage, see `s3-example.ts` for a complete implementation that can be published as a separate package (`@semio/file-provider-s3`).

Key points:

- Keep AWS SDK in separate package to avoid bundling it in core
- Users install only if they need S3 support
- Can be used with composite provider as the remote layer

## File Structure

Files are organized by kit, file ID, and path:

```
{storage}/
  ├── {kitId}/
  │   ├── {fileId}/
  │   │   └── {filename}
  │   └── {fileId}/
  │       └── {filename}
  └── {kitId}/
      └── ...
```

## Drag and Drop

The Kit App supports drag-and-drop file uploads:

1. User drags files over the canvas
2. Drop zone appears with visual feedback
3. Files are automatically uploaded and added to the kit

## Error Handling

The composite provider automatically handles errors:

```typescript
// Upload: Succeeds if at least one provider succeeds
// Download: Tries providers in order (memory → local → remote)
// Delete: Best effort (doesn't fail if some providers fail)
```

## Best Practices

1. **Use composite provider** with all three layers for remote kits
2. **Always include memory** for fast access
3. **Always include local** for offline support
4. **Implement retry logic** in remote provider
5. **Clean up blob URLs** with `URL.revokeObjectURL()` when done
6. **Validate file types and sizes** before upload
7. **Show upload progress** for better UX

## Offline Support

The system works offline by design:

- **Memory**: Always available
- **Local**: Available offline (IndexedDB)
- **Remote**: Gracefully degrades (uses local cache)

When online again, changes sync automatically via Y.js and file providers.

## Limitations

- File providers are **optional** - Sketchpad works without them
- Files are **not versioned** - updates overwrite existing files
- **No conflict resolution** - last write wins
- **No access control** - implement in your provider if needed
