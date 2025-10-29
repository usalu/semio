// #region Header

// example.tsx

// Example usage of Semio file provider system

// #endregion

import React from 'react';
import { Sketchpad, createS3FileProviderFactory, createInMemoryFileProviderFactory } from '@semio/js';

// Example 1: Production S3 Setup
export function ProductionApp() {
  const fileProviderFactory = createS3FileProviderFactory({
    region: process.env.AWS_REGION || 'us-east-1',
    bucket: process.env.AWS_S3_BUCKET || 'my-semio-kits',
    accessKeyId: process.env.AWS_ACCESS_KEY_ID!,
    secretAccessKey: process.env.AWS_SECRET_ACCESS_KEY!,
  });

  return (
    <Sketchpad 
      id="production-sketchpad" 
      fileProviderFactory={fileProviderFactory}
    />
  );
}

// Example 2: Development with MinIO
export function DevelopmentApp() {
  const fileProviderFactory = createS3FileProviderFactory({
    region: 'us-east-1',
    bucket: 'semio-dev',
    accessKeyId: 'minioadmin',
    secretAccessKey: 'minioadmin',
    endpoint: 'http://localhost:9000',
  });

  return (
    <Sketchpad 
      id="dev-sketchpad" 
      fileProviderFactory={fileProviderFactory}
    />
  );
}

// Example 3: Testing with In-Memory Provider
export function TestApp() {
  const fileProviderFactory = createInMemoryFileProviderFactory();

  return (
    <Sketchpad 
      id="test-sketchpad" 
      fileProviderFactory={fileProviderFactory}
    />
  );
}

// Example 4: Offline/Local Only (No File Provider)
export function OfflineApp() {
  // Files will be stored as blob URLs in memory only
  // They will be lost on page reload
  return <Sketchpad id="offline-sketchpad" />;
}

// Example 5: Conditional File Provider (Based on Environment)
export function AdaptiveApp() {
  const fileProviderFactory = React.useMemo(() => {
    if (process.env.NODE_ENV === 'production') {
      return createS3FileProviderFactory({
        region: process.env.AWS_REGION!,
        bucket: process.env.AWS_S3_BUCKET!,
        accessKeyId: process.env.AWS_ACCESS_KEY_ID!,
        secretAccessKey: process.env.AWS_SECRET_ACCESS_KEY!,
      });
    } else if (process.env.NODE_ENV === 'development') {
      return createInMemoryFileProviderFactory();
    }
    return undefined;
  }, []);

  return (
    <Sketchpad 
      id="adaptive-sketchpad" 
      fileProviderFactory={fileProviderFactory}
    />
  );
}

// Example 6: Custom File Provider
import type { FileProviderFactory, FileProvider } from '@semio/js';

function createCustomFileProviderFactory(baseUrl: string): FileProviderFactory {
  return async (kitId: string): Promise<FileProvider> => {
    return {
      upload: async (kitId, fileId, path, blob) => {
        const formData = new FormData();
        formData.append('file', blob, path);
        
        const response = await fetch(`${baseUrl}/kits/${kitId}/files/${fileId}`, {
          method: 'POST',
          body: formData,
        });
        
        const { url } = await response.json();
        return url;
      },

      download: async (kitId, fileId, path) => {
        const response = await fetch(`${baseUrl}/kits/${kitId}/files/${fileId}`);
        return await response.blob();
      },

      delete: async (kitId, fileId, path) => {
        await fetch(`${baseUrl}/kits/${kitId}/files/${fileId}`, {
          method: 'DELETE',
        });
      },

      getUrl: (kitId, fileId, path) => {
        return `${baseUrl}/kits/${kitId}/files/${fileId}`;
      },
    };
  };
}

export function CustomBackendApp() {
  const fileProviderFactory = createCustomFileProviderFactory('https://api.example.com');

  return (
    <Sketchpad 
      id="custom-backend-sketchpad" 
      fileProviderFactory={fileProviderFactory}
    />
  );
}
