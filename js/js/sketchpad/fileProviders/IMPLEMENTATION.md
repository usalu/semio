# File Provider Implementation Summary

## Overview
Implemented a comprehensive file provider system for Semio that allows kits to store and sync files with remote storage backends (like S3) while keeping them separate from Y.js documents.

## Key Features

### 1. File Provider Architecture
- **FileProvider Interface**: Defines upload, download, delete, and getUrl operations
- **FileProviderFactory**: Factory function that creates provider instances per kit
- **Separation of Concerns**: File metadata in Y.js, file content in external storage

### 2. Integration Points

#### Sketchpad Component
- Added optional `fileProviderFactory` prop
- Passes factory through to SketchpadStore via SketchpadScopeProvider
- Works with or without file provider (graceful degradation)

#### SketchpadStore
- Accepts `fileProviderFactory` in constructor
- Passes factory to KitStore instances when creating kits
- Maintains backwards compatibility

#### KitStore
- Initializes file provider on construction (if factory provided)
- Automatically syncs existing files on initialization
- Handles file operations (upload/download/delete) in executeCommand
- Creates and manages blob URLs for local file access
- Cleans up resources (revokeObjectURL) when files are deleted

### 3. Drag & Drop Support
Added to Kit App Canvas:
- Drag over detection with visual feedback
- File drop handling
- Automatic file addition to kit with upload

### 4. Provider Implementations

#### S3 File Provider (`createS3FileProviderFactory`)
- Full AWS S3 integration
- Support for S3-compatible services (MinIO, etc.)
- Configurable region, bucket, credentials, and endpoint
- Proper key structure: `{bucket}/{kitId}/{fileId}/{filename}`
- Metadata storage in S3 object headers

#### In-Memory File Provider (`createInMemoryFileProviderFactory`)
- Simple Map-based storage for testing
- No external dependencies
- Useful for development and offline scenarios

## File Structure

```
js/js/sketchpad/
├── store.tsx                     # Added FileProvider & FileProviderFactory types
├── Sketchpad.tsx                 # Added fileProviderFactory prop
├── kits/
│   ├── store.tsx                 # Updated KitStore with file provider logic
│   └── commands.ts               # File operations (addFile, removeFile, updateFile)
├── apps/
│   └── kit/
│       └── App.tsx               # Added drag & drop for files
└── fileProviders/
    ├── index.ts                  # Barrel export
    ├── s3FileProvider.ts         # S3 & in-memory implementations
    └── README.md                 # Documentation
```

## Usage Examples

### Basic S3 Setup
```typescript
import { Sketchpad, createS3FileProviderFactory } from '@semio/js';

const fileProviderFactory = createS3FileProviderFactory({
  region: 'us-east-1',
  bucket: 'my-semio-kits',
  accessKeyId: process.env.AWS_ACCESS_KEY_ID!,
  secretAccessKey: process.env.AWS_SECRET_ACCESS_KEY!,
});

<Sketchpad 
  id="my-sketchpad" 
  fileProviderFactory={fileProviderFactory}
/>
```

### With MinIO
```typescript
const fileProviderFactory = createS3FileProviderFactory({
  region: 'us-east-1',
  bucket: 'semio',
  accessKeyId: 'minioadmin',
  secretAccessKey: 'minioadmin',
  endpoint: 'http://localhost:9000',
});
```

### In-Memory (Testing)
```typescript
import { createInMemoryFileProviderFactory } from '@semio/js';

const fileProviderFactory = createInMemoryFileProviderFactory();

<Sketchpad id="my-sketchpad" fileProviderFactory={fileProviderFactory} />
```

## How It Works

### 1. Kit Initialization
```
SketchpadStore creates KitStore
  → KitStore constructor receives fileProviderFactory
  → initializeFileProvider() called
  → FileProvider instance created for this kit
  → syncFiles() downloads all existing files
  → Blob URLs created for each file
```

### 2. Adding Files
```
User drops file on Kit App canvas
  → handleDrop() processes files
  → kitCommands.addFile(file, blob)
  → Command creates FileDiff
  → KitStore.executeCommand() processes diff
  → File metadata added to Y.js
  → FileProvider.upload() uploads to storage
  → Blob URL created locally
  → File immediately available
```

### 3. Removing Files
```
User deletes file
  → kitCommands.removeFile(fileId)
  → Command creates FileDiff with removed
  → KitStore.executeCommand() processes diff
  → FileProvider.delete() removes from storage
  → Blob URL revoked
  → File metadata removed from Y.js
```

## Benefits

1. **Scalability**: Large files don't bloat Y.js documents
2. **Performance**: Files loaded on-demand, not all at once
3. **Flexibility**: Any storage backend can be used
4. **Reliability**: File operations are asynchronous and non-blocking
5. **Developer Experience**: Simple drag & drop interface
6. **Backwards Compatible**: Works with or without file provider

## Technical Decisions

1. **Lazy Loading AWS SDK**: S3 provider dynamically imports AWS SDK to avoid bundling when not needed
2. **Blob URLs**: Files are accessed via blob URLs for consistent API across providers
3. **Per-Kit Providers**: Each kit gets its own provider instance for isolation
4. **Graceful Degradation**: System works without file provider (local-only mode)
5. **Metadata in Y.js**: File paths, sizes, dates stay in sync with Y.js for consistency

## Testing Strategy

1. **Unit Tests**: Test file provider implementations
2. **Integration Tests**: Test KitStore file operations
3. **E2E Tests**: Test drag & drop and file sync
4. **Manual Testing**: 
   - Test with actual S3/MinIO
   - Test with in-memory provider
   - Test without provider
   - Test large files
   - Test concurrent uploads

## Future Enhancements

1. **Progress Tracking**: Show upload/download progress
2. **Error Handling**: Better error messages and retry logic
3. **File Versioning**: Keep multiple versions of files
4. **Compression**: Automatic compression for certain file types
5. **CDN Integration**: Serve files through CDN
6. **Presigned URLs**: Generate temporary access URLs
7. **File Validation**: Validate file types and sizes before upload
8. **Batch Operations**: Upload/download multiple files efficiently
