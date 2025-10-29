# File Provider System

The file provider system allows Semio kits to store and sync files with remote storage backends (like S3) while keeping them separate from the Y.js document for better performance and scalability.

## Overview

Files in a kit can be too large to efficiently store in Y.js documents. The file provider system:
- Separates file storage from Y.js document state
- Provides automatic file syncing when `fileProviderFactory` is provided
- Creates blob URLs for in-memory file access
- Supports any storage backend through a simple interface

## Architecture

### FileProvider Interface

```typescript
interface FileProvider {
  upload: (kitId: string, fileId: string, path: string, blob: Blob) => Promise<string>;
  download: (kitId: string, fileId: string, path: string) => Promise<Blob>;
  delete: (kitId: string, fileId: string, path: string) => Promise<void>;
  getUrl: (kitId: string, fileId: string, path: string) => string;
}
```

### FileProviderFactory

```typescript
type FileProviderFactory = (kitId: string) => Promise<FileProvider>;
```

## Usage

### With S3

```typescript
import { Sketchpad, createS3FileProviderFactory } from '@semio/js';

const fileProviderFactory = createS3FileProviderFactory({
  region: 'us-east-1',
  bucket: 'my-semio-kits',
  accessKeyId: process.env.AWS_ACCESS_KEY_ID!,
  secretAccessKey: process.env.AWS_SECRET_ACCESS_KEY!,
});

function App() {
  return (
    <Sketchpad 
      id="my-sketchpad" 
      fileProviderFactory={fileProviderFactory}
    />
  );
}
```

### With MinIO or S3-Compatible Storage

```typescript
const fileProviderFactory = createS3FileProviderFactory({
  region: 'us-east-1',
  bucket: 'semio',
  accessKeyId: 'minioadmin',
  secretAccessKey: 'minioadmin',
  endpoint: 'http://localhost:9000',
});
```

### In-Memory (Testing/Offline)

```typescript
import { Sketchpad, createInMemoryFileProviderFactory } from '@semio/js';

const fileProviderFactory = createInMemoryFileProviderFactory();

function App() {
  return (
    <Sketchpad 
      id="my-sketchpad" 
      fileProviderFactory={fileProviderFactory}
    />
  );
}
```

### Without File Provider (Local Only)

```typescript
// Files will be stored as blob URLs in memory only
// They will be lost on page reload
<Sketchpad id="my-sketchpad" />
```

## File Operations

### Adding Files via Drag & Drop

Files can be dragged and dropped onto the Kit App canvas. They will be:
1. Added to the kit's file list in Y.js
2. Uploaded to the file provider (if configured)
3. Made available via blob URLs for immediate use

### Adding Files Programmatically

```typescript
import { guid } from '@semio/js';

const file: SemioFile = {
  guid: guid(),
  path: 'models/building.ifc',
  size: blob.size,
  createdAt: new Date(),
  updatedAt: new Date(),
};

await kitCommands.addFile(file, blob);
```

## File Storage Structure

Files are stored with the following key structure:
```
{bucket}/{kitId}/{fileId}/{filename}
```

For example:
```
my-semio-kits/
  abc-123-def/
    file-456/
      model.ifc
    file-789/
      texture.png
```

## Custom File Providers

You can implement your own file provider for any storage backend:

```typescript
const customFileProviderFactory: FileProviderFactory = async (kitId) => {
  return {
    upload: async (kitId, fileId, path, blob) => {
      // Upload to your storage
      // Return the remote URL
      return `https://my-storage.com/${kitId}/${fileId}/${path}`;
    },
    
    download: async (kitId, fileId, path) => {
      // Download from your storage
      // Return as Blob
      const response = await fetch(`https://my-storage.com/${kitId}/${fileId}/${path}`);
      return await response.blob();
    },
    
    delete: async (kitId, fileId, path) => {
      // Delete from your storage
      await fetch(`https://my-storage.com/${kitId}/${fileId}/${path}`, {
        method: 'DELETE'
      });
    },
    
    getUrl: (kitId, fileId, path) => {
      // Return the public URL for the file
      return `https://my-storage.com/${kitId}/${fileId}/${path}`;
    },
  };
};
```

## File Syncing

When a kit is loaded with a file provider:
1. The kit store initializes the file provider
2. All files in the kit are automatically downloaded
3. Blob URLs are created for each file
4. Files become immediately available to components

When files are added/removed:
1. Changes are recorded in the Y.js document
2. Files are uploaded/deleted to/from the remote storage
3. Blob URLs are created/revoked accordingly

## Requirements

For S3 file provider:
```bash
npm install @aws-sdk/client-s3
```

## Notes

- File metadata (path, size, hash, etc.) is stored in Y.js
- File content (blobs) is stored separately via the file provider
- Blob URLs are created locally for efficient access
- File operations are asynchronous and non-blocking
- The file provider is optional - kits work without it (local/in-memory only)
