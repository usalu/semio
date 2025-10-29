// #region Header

// s3FileProvider.ts

// Example implementation of FileProviderFactory for AWS S3

// #endregion

import type { FileProvider, FileProviderFactory } from "../store";

export interface S3Config {
  region: string;
  bucket: string;
  accessKeyId: string;
  secretAccessKey: string;
  endpoint?: string; // For S3-compatible services like MinIO
}

/**
 * Creates an S3-based file provider for a kit.
 * Files are stored in S3 with the structure: {bucket}/{kitId}/{fileId}/{filename}
 * 
 * @example
 * const fileProviderFactory = createS3FileProviderFactory({
 *   region: 'us-east-1',
 *   bucket: 'my-semio-kits',
 *   accessKeyId: process.env.AWS_ACCESS_KEY_ID!,
 *   secretAccessKey: process.env.AWS_SECRET_ACCESS_KEY!,
 * });
 * 
 * const sketchpad = <Sketchpad 
 *   id="my-sketchpad" 
 *   fileProviderFactory={fileProviderFactory}
 * />;
 */
export function createS3FileProviderFactory(config: S3Config): FileProviderFactory {
  return async (kitId: string): Promise<FileProvider> => {
    // Lazy load AWS SDK to avoid bundling it if not needed
    const { S3Client, PutObjectCommand, GetObjectCommand, DeleteObjectCommand } = await import("@aws-sdk/client-s3");
    
    const s3Client = new S3Client({
      region: config.region,
      credentials: {
        accessKeyId: config.accessKeyId,
        secretAccessKey: config.secretAccessKey,
      },
      ...(config.endpoint && { endpoint: config.endpoint }),
    });

    const getKey = (kitId: string, fileId: string, path: string): string => {
      // Use forward slashes for S3 keys
      return `${kitId}/${fileId}/${path.replace(/\\/g, '/')}`;
    };

    const getUrl = (kitId: string, fileId: string, path: string): string => {
      const key = getKey(kitId, fileId, path);
      if (config.endpoint) {
        // For custom endpoints (e.g., MinIO)
        return `${config.endpoint}/${config.bucket}/${key}`;
      }
      // Standard S3 URL
      return `https://${config.bucket}.s3.${config.region}.amazonaws.com/${key}`;
    };

    return {
      upload: async (kitId: string, fileId: string, path: string, blob: Blob): Promise<string> => {
        const key = getKey(kitId, fileId, path);
        const buffer = Buffer.from(await blob.arrayBuffer());
        
        const command = new PutObjectCommand({
          Bucket: config.bucket,
          Key: key,
          Body: buffer,
          ContentType: blob.type || 'application/octet-stream',
          Metadata: {
            'kit-id': kitId,
            'file-id': fileId,
            'original-path': path,
          },
        });

        await s3Client.send(command);
        return getUrl(kitId, fileId, path);
      },

      download: async (kitId: string, fileId: string, path: string): Promise<Blob> => {
        const key = getKey(kitId, fileId, path);
        
        const command = new GetObjectCommand({
          Bucket: config.bucket,
          Key: key,
        });

        const response = await s3Client.send(command);
        
        if (!response.Body) {
          throw new Error(`File not found: ${key}`);
        }

        // Convert stream to blob
        const chunks: Uint8Array[] = [];
        for await (const chunk of response.Body as any) {
          chunks.push(chunk);
        }
        
        const buffer = Buffer.concat(chunks);
        return new Blob([buffer], { type: response.ContentType || 'application/octet-stream' });
      },

      delete: async (kitId: string, fileId: string, path: string): Promise<void> => {
        const key = getKey(kitId, fileId, path);
        
        const command = new DeleteObjectCommand({
          Bucket: config.bucket,
          Key: key,
        });

        await s3Client.send(command);
      },

      getUrl: (kitId: string, fileId: string, path: string): string => {
        return getUrl(kitId, fileId, path);
      },
    };
  };
}

/**
 * Creates a simple in-memory file provider for testing or offline use.
 * Files are stored in memory using Map and will be lost on page reload.
 * 
 * @example
 * const fileProviderFactory = createInMemoryFileProviderFactory();
 * 
 * const sketchpad = <Sketchpad 
 *   id="my-sketchpad" 
 *   fileProviderFactory={fileProviderFactory}
 * />;
 */
export function createInMemoryFileProviderFactory(): FileProviderFactory {
  const storage = new Map<string, Blob>();

  return async (kitId: string): Promise<FileProvider> => {
    const getKey = (kitId: string, fileId: string, path: string): string => {
      return `${kitId}/${fileId}/${path}`;
    };

    return {
      upload: async (kitId: string, fileId: string, path: string, blob: Blob): Promise<string> => {
        const key = getKey(kitId, fileId, path);
        storage.set(key, blob);
        return `memory://${key}`;
      },

      download: async (kitId: string, fileId: string, path: string): Promise<Blob> => {
        const key = getKey(kitId, fileId, path);
        const blob = storage.get(key);
        
        if (!blob) {
          throw new Error(`File not found: ${key}`);
        }
        
        return blob;
      },

      delete: async (kitId: string, fileId: string, path: string): Promise<void> => {
        const key = getKey(kitId, fileId, path);
        storage.delete(key);
      },

      getUrl: (kitId: string, fileId: string, path: string): string => {
        return `memory://${getKey(kitId, fileId, path)}`;
      },
    };
  };
}
