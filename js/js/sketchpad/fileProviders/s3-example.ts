// #region Header

// s3-example.ts

// Example S3 file provider implementation (for separate package)

// #endregion

// This is an EXAMPLE implementation that should be published as a separate package
// Users who need S3 support should install: @semio/file-provider-s3 (or similar)

// Package: @semio/file-provider-s3
// Dependencies: @aws-sdk/client-s3

/*
import { S3Client, PutObjectCommand, GetObjectCommand, DeleteObjectCommand } from "@aws-sdk/client-s3";
import type { FileProviderFactory } from "@semio/js";

export interface S3Config {
  region: string;
  bucket: string;
  credentials: {
    accessKeyId: string;
    secretAccessKey: string;
  };
  endpoint?: string;
  forcePathStyle?: boolean;
}

export function createS3FileProvider(config: S3Config): FileProviderFactory {
  return async (kitId: string) => {
    const s3Client = new S3Client({
      region: config.region,
      credentials: config.credentials,
      endpoint: config.endpoint,
      forcePathStyle: config.forcePathStyle,
    });

    const getKey = (kitId: string, fileId: string, path: string): string => {
      return `${kitId}/${fileId}/${path}`;
    };

    const streamToBuffer = async (stream: any): Promise<Buffer> => {
      const chunks: any[] = [];
      for await (const chunk of stream) {
        chunks.push(chunk);
      }
      return Buffer.concat(chunks);
    };

    return {
      upload: async (kitId, fileId, path, blob) => {
        const key = getKey(kitId, fileId, path);
        const buffer = Buffer.from(await blob.arrayBuffer());

        await s3Client.send(new PutObjectCommand({
          Bucket: config.bucket,
          Key: key,
          Body: buffer,
          ContentType: blob.type,
        }));

        console.log(`[S3] Uploaded file ${path} (${blob.size} bytes)`);

        const baseUrl = config.endpoint
          ? `${config.endpoint}/${config.bucket}`
          : `https://${config.bucket}.s3.${config.region}.amazonaws.com`;

        return `${baseUrl}/${key}`;
      },

      download: async (kitId, fileId, path) => {
        const key = getKey(kitId, fileId, path);

        const response = await s3Client.send(new GetObjectCommand({
          Bucket: config.bucket,
          Key: key,
        }));

        if (!response.Body) {
          throw new Error(`File not found in S3: ${key}`);
        }

        const buffer = await streamToBuffer(response.Body);
        const blob = new Blob([buffer], { type: response.ContentType });

        console.log(`[S3] Downloaded file ${path} (${blob.size} bytes)`);
        return blob;
      },

      delete: async (kitId, fileId, path) => {
        const key = getKey(kitId, fileId, path);

        await s3Client.send(new DeleteObjectCommand({
          Bucket: config.bucket,
          Key: key,
        }));

        console.log(`[S3] Deleted file ${path}`);
      },

      getUrl: (kitId, fileId, path) => {
        const key = getKey(kitId, fileId, path);

        const baseUrl = config.endpoint
          ? `${config.endpoint}/${config.bucket}`
          : `https://${config.bucket}.s3.${config.region}.amazonaws.com`;

        return `${baseUrl}/${key}`;
      },
    };
  };
}

// Usage with composite provider
import { createCompositeFileProvider } from "@semio/js";

const fileProviderFactory = createCompositeFileProvider({
  memory: true,
  local: true,
  remote: {
    baseUrl: "https://api.example.com",
    headers: {
      "X-S3-Bucket": "my-bucket",
      "Authorization": "Bearer ...",
    },
  },
});

// Or use S3 directly as a remote provider by wrapping it
async function createS3RemoteProvider(config: S3Config, kitId: string) {
  const s3Provider = createS3FileProvider(config);
  return await s3Provider(kitId);
}

const fileProviderFactoryWithS3 = createCompositeFileProvider({
  memory: true,
  local: true,
  remote: await createS3RemoteProvider({
    region: "us-east-1",
    bucket: "my-bucket",
    credentials: {
      accessKeyId: process.env.AWS_ACCESS_KEY_ID!,
      secretAccessKey: process.env.AWS_SECRET_ACCESS_KEY!,
    },
  }, "my-kit-id"),
});
*/

// For MinIO (S3-compatible):
/*
const fileProviderFactory = createS3FileProvider({
  region: "us-east-1",
  bucket: "semio-files",
  credentials: {
    accessKeyId: "minioadmin",
    secretAccessKey: "minioadmin",
  },
  endpoint: "http://localhost:9000",
  forcePathStyle: true,
});
*/
