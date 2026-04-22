// @semio/js — graph kit commands (extracted from sketchpad)

/**
 * Context for kit commands including kit data, file URLs, and origin.
 **/
export interface KitCommandContext {
  kit: Kit;
  fileUrls: Map<Url, Url>;
  origin?: string;
}

/**
 * Result of a kit command with optional diff, files, and origin.
 **/
export interface KitCommandResult {
  diff?: KitDiff;
  files?: File[];
  origin?: string;
}
type KitFileState = {
  blobs: Map<string, Blob>;
  objectUrls: Map<string, string>;
  providerUrls: Map<string, string>;
  pendingBlobDownloads: Map<string, Promise<string | null>>;
  providerFactory?: FileProviderFactory;
  provider?: FileProvider;
  providerKitId?: string;
};

type KitBinaryStore = KitStore & {
  readFile?: (path: string) => Promise<Blob | null>;
  writeFile?: (path: string, blob: Blob) => Promise<void>;
  deleteFile?: (path: string) => Promise<void>;
  createDirectory?: (path: string) => Promise<void>;
  moveEntry?: (fromPath: string, toPath: string) => Promise<void>;
};

export function getOrCreateKitFileState(kitStore: KitStore): KitFileState {
  const storeWithFiles = kitStore as KitStore & { __semioKitFileState?: KitFileState };
  if (!storeWithFiles.__semioKitFileState) {
    storeWithFiles.__semioKitFileState = {
      blobs: new Map(),
      objectUrls: new Map(),
      providerUrls: new Map(),
      pendingBlobDownloads: new Map(),
    };
  }
  return storeWithFiles.__semioKitFileState;
}

export function getStoredKitFileUrls(kitStore: KitStore): Map<string, string> {
  const kit = kitStore.getSnapshot().kit;
  const fileState = getOrCreateKitFileState(kitStore);
  const fileUrls = new Map<string, string>();

  for (const file of kit.files ?? []) {
    const readableUrl = getReadableKitFileUrl(fileState, file);
    if (readableUrl) {
      fileUrls.set(getKitFileStoragePath(kit, file), readableUrl);
    }
  }

  return fileUrls;
}

const isBrowserReadableFileUrl = (url: string): boolean => /^(blob:|data:|https?:)/i.test(url);

const getReadableKitFileUrl = (fileState: KitFileState, file: SemioFile): string | null => {
  const cachedBlobUrl = fileState.objectUrls.get(file.id);
  if (cachedBlobUrl) {
    return cachedBlobUrl;
  }

  const cachedProviderUrl = fileState.providerUrls.get(file.id);
  if (cachedProviderUrl && isBrowserReadableFileUrl(cachedProviderUrl)) {
    return cachedProviderUrl;
  }

  if (file.blob && isBrowserReadableFileUrl(file.blob)) {
    return file.blob;
  }

  if (file.remote && isBrowserReadableFileUrl(file.remote)) {
    return file.remote;
  }

  return null;
};

const getKitFileStoragePath = (kit: Kit, file: SemioFile): string => {
  const foldersById = new Map((kit.folders ?? []).map((folder) => [folder.id, folder]));
  const pathSegments: string[] = [file.name];
  let currentFolderId = file.folder?.id;

  while (currentFolderId) {
    const folder = foldersById.get(currentFolderId);
    if (!folder) {
      break;
    }
    pathSegments.unshift(folder.name);
    currentFolderId = folder.parent?.id;
  }

  return pathSegments.join("/");
};

const getKitFolderStoragePath = (kit: Kit, folderLike: Pick<Folder, "id" | "name" | "parent"> | { id: string }): string => {
  const foldersById = new Map((kit.folders ?? []).map((folder) => [folder.id, folder]));
  const visited = new Set<string>();
  const pathSegments: string[] = [];
  let currentFolder: Pick<Folder, "id" | "name" | "parent"> | undefined = "name" in folderLike ? folderLike : foldersById.get(folderLike.id);

  while (currentFolder) {
    if (visited.has(currentFolder.id)) {
      break;
    }
    visited.add(currentFolder.id);
    pathSegments.unshift(currentFolder.name);
    const parentId = currentFolder.parent?.id;
    currentFolder = parentId ? foldersById.get(parentId) : undefined;
  }

  return pathSegments.join("/");
};

const revokeKitFileObjectUrl = (kitStore: KitStore, fileId: string): void => {
  const fileState = getOrCreateKitFileState(kitStore);
  const currentObjectUrl = fileState.objectUrls.get(fileId);
  if (currentObjectUrl) {
    URL.revokeObjectURL(currentObjectUrl);
    fileState.objectUrls.delete(fileId);
  }
};

const createKitFileObjectUrl = (kitStore: KitStore, fileId: string, blob: Blob): string => {
  const fileState = getOrCreateKitFileState(kitStore);
  revokeKitFileObjectUrl(kitStore, fileId);
  const objectUrl = URL.createObjectURL(blob);
  fileState.objectUrls.set(fileId, objectUrl);
  return objectUrl;
};

const getExistingKitFileProvider = (kitStore: KitStore): FileProvider | null => {
  return getOrCreateKitFileState(kitStore).provider ?? null;
};

const getKitFileProvider = async (kitStore: KitStore, kitId: string): Promise<FileProvider | null> => {
  const fileState = getOrCreateKitFileState(kitStore);
  if (fileState.provider && fileState.providerKitId === kitId) {
    return fileState.provider;
  }

  if (!fileState.providerFactory) {
    return null;
  }

  fileState.provider = await fileState.providerFactory(kitId);
  fileState.providerKitId = kitId;
  return fileState.provider;
};

const fetchReadableKitFileBlob = async (url: string): Promise<Blob | null> => {
  try {
    const response = await fetch(url);
    if (!response.ok) {
      return null;
    }
    return await response.blob();
  } catch {
    return null;
  }
};

const uploadKitFileToProvider = async (kitStore: KitStore, kit: Kit, file: SemioFile, blob: Blob): Promise<void> => {
  const fileState = getOrCreateKitFileState(kitStore);
  fileState.blobs.set(file.id, blob);
  revokeKitFileObjectUrl(kitStore, file.id);
  const storagePath = getKitFileStoragePath(kit, file);

  // 🔖EmbedInJsonFileKit
  // For file kits, embed the blob as a data URL in file.blob so everything
  // stays inside the single *.kit.semio.json file on save.
  // Specs: Use embedFileBlob presence, not instanceof JsonFileKitStore — desktop may load two bundle copies of the class so instanceof would skip embedding. kitStore may be a CollaborativeKitStore wrapper, so also check the inner store exposed via `.store`.
  const innerCandidate = (kitStore as { store?: unknown }).store;
  const embedTarget = typeof (kitStore as any)?.embedFileBlob === "function" ? (kitStore as any) : typeof (innerCandidate as any)?.embedFileBlob === "function" ? (innerCandidate as any) : null;
  if (embedTarget) {
    try {
      await embedTarget.embedFileBlob(file.id, blob);
    } catch (error) {
      console.error(`uploadKitFileToProvider: failed to embed blob for ${file.id}:`, error);
    }
    return;
  }

  const binaryStore = kitStore as KitBinaryStore;
  if (typeof binaryStore.writeFile === "function") {
    await binaryStore.writeFile(storagePath, blob);
  }

  const provider = await getKitFileProvider(kitStore, kit.id);
  if (!provider) {
    return;
  }

  await provider.upload(kit.id, file.id, storagePath, blob);
  const providerUrl = provider.getUrl(kit.id, file.id, storagePath);
  if (providerUrl) {
    fileState.providerUrls.set(file.id, providerUrl);
  }
};

const deleteKitFileFromProvider = async (kitStore: KitStore, kit: Kit, file: SemioFile | undefined): Promise<void> => {
  if (!file) {
    return;
  }

  const fileState = getOrCreateKitFileState(kitStore);
  fileState.blobs.delete(file.id);
  fileState.providerUrls.delete(file.id);
  revokeKitFileObjectUrl(kitStore, file.id);
  const storagePath = getKitFileStoragePath(kit, file);

  const binaryStore = kitStore as KitBinaryStore;
  if (typeof binaryStore.deleteFile === "function") {
    await binaryStore.deleteFile(storagePath);
  }

  const provider = await getKitFileProvider(kitStore, kit.id);
  if (!provider) {
    return;
  }

  await provider.delete(kit.id, file.id, storagePath);
};

const syncKitFileCommandResult = async (kitStore: KitStore, kit: Kit, command: string, args: any[], result: KitCommandResult): Promise<void> => {
  const binaryStore = kitStore as KitBinaryStore;
  const nextKit = result.diff ? applyKitDiff(kit, result.diff) : kit;

  if (command === "semio.kit.addFile") {
    const file = args[0] as SemioFile | undefined;
    const blob = args[1] as Blob | undefined;
    if (file && blob) {
      await uploadKitFileToProvider(kitStore, kit, file, blob);
    }
    return;
  }

  if (command === "semio.kit.addFiles") {
    const filesToAdd = (args[1] as { file: SemioFile; blob?: Blob }[] | undefined) ?? [];
    await Promise.all(
      filesToAdd.map(async ({ file, blob }) => {
        if (blob) {
          await uploadKitFileToProvider(kitStore, kit, file, blob);
        }
      }),
    );
    return;
  }

  if (command === "semio.kit.updateFile") {
    const fileId = args[0] as string | undefined;
    const fileDiff = args[1] as FileDiff | undefined;
    const blob = args[2] as Blob | undefined;
    if (!fileId || !blob) {
      return;
    }

    const existingFile = kit.files?.find((file) => file.id === fileId);
    if (!existingFile) {
      return;
    }

    const updatedFile = { ...existingFile, ...fileDiff };
    await uploadKitFileToProvider(kitStore, kit, updatedFile, blob);
    return;
  }

  if (command === "semio.kit.removeFile") {
    const fileId = args[0] as string | undefined;
    const existingFile = kit.files?.find((file) => file.id === fileId);
    await deleteKitFileFromProvider(kitStore, kit, existingFile);
    return;
  }

  if (command === "semio.kit.createFolder") {
    const folder = args[0] as Folder | undefined;
    if (!folder || typeof binaryStore.createDirectory !== "function") {
      return;
    }
    await binaryStore.createDirectory(getKitFolderStoragePath(nextKit, folder));
    return;
  }

  if (command === "semio.kit.updateFolder") {
    const folderId = args[0] as string | undefined;
    if (!folderId || typeof binaryStore.moveEntry !== "function") {
      return;
    }
    const currentFolder = kit.folders?.find((folder) => folder.id === folderId);
    const updatedFolder = nextKit.folders?.find((folder) => folder.id === folderId);
    if (!currentFolder || !updatedFolder) {
      return;
    }
    const currentPath = getKitFolderStoragePath(kit, currentFolder);
    const nextPath = getKitFolderStoragePath(nextKit, updatedFolder);
    if (currentPath && nextPath && currentPath !== nextPath) {
      await binaryStore.moveEntry(currentPath, nextPath);
    }
    return;
  }

  if (command === "semio.kit.import") {
    const importedFiles = result.diff?.files?.added ?? [];
    const importedBlobs = result.files ?? [];
    await Promise.all(
      importedFiles.map(async (file, index) => {
        const blob = importedBlobs[index];
        if (blob) {
          await uploadKitFileToProvider(kitStore, kit, file as SemioFile, blob);
        }
      }),
    );
    return;
  }

  if (command !== "semio.kit.moveToFolder") {
    return;
  }

  const artifactId = args[0] as string | undefined;
  const artifactKind = args[1] as "type" | "design" | "quality" | "file" | "folder" | undefined;
  if (!artifactId || !artifactKind) {
    return;
  }

  if (artifactKind === "file" && typeof binaryStore.moveEntry === "function") {
    const currentFile = kit.files?.find((file) => file.id === artifactId);
    const updatedFile = nextKit.files?.find((file) => file.id === artifactId);
    if (!currentFile || !updatedFile) {
      return;
    }
    const currentPath = getKitFileStoragePath(kit, currentFile);
    const nextPath = getKitFileStoragePath(nextKit, updatedFile);
    if (currentPath && nextPath && currentPath !== nextPath) {
      await binaryStore.moveEntry(currentPath, nextPath);
    }
    return;
  }

  if (artifactKind === "folder") {
    const currentFolder = kit.folders?.find((folder) => folder.id === artifactId);
    const updatedFolder = nextKit.folders?.find((folder) => folder.id === artifactId);
    if (!currentFolder || !updatedFolder) {
      return;
    }
    if (typeof binaryStore.moveEntry !== "function") {
      return;
    }
    const currentPath = getKitFolderStoragePath(kit, currentFolder);
    const nextPath = getKitFolderStoragePath(nextKit, updatedFolder);
    if (currentPath && nextPath && currentPath !== nextPath) {
      await binaryStore.moveEntry(currentPath, nextPath);
    }
  }
};

export const semioKitCommandHandlers = {
  "semio.kit.createAuthor": (context: KitCommandContext, author: Author): KitCommandResult => {
    return {
      diff: { authors: { added: [author] } },
    };
  },
  "semio.kit.updateAuthor": (context: KitCommandContext, id: Id, diff: AuthorDiff): KitCommandResult => {
    return {
      diff: { authors: { updated: [{ author: { id }, diff }] } },
    };
  },
  "semio.kit.deleteAuthor": (context: KitCommandContext, id: Id): KitCommandResult => {
    return {
      diff: { authors: { removed: [{ id }] } },
    };
  },
  "semio.kit.createType": (context: KitCommandContext, type: Type): KitCommandResult => {
    return {
      diff: { types: { added: [type] } },
    };
  },
  "semio.kit.updateType": (context: KitCommandContext, id: Id, diff: TypeDiff): KitCommandResult => {
    return {
      diff: { types: { updated: [{ type: { id }, diff }] } },
    };
  },
  "semio.kit.deleteType": (context: KitCommandContext, id: Id): KitCommandResult => {
    return {
      diff: { types: { removed: [{ id }] } },
    };
  },
  "semio.kit.createDesign": (context: KitCommandContext, design: Design): KitCommandResult => {
    return {
      diff: { designs: { added: [design] } },
    };
  },
  "semio.kit.updateDesign": (context: KitCommandContext, id: Id, diff: DesignDiff): KitCommandResult => {
    return {
      diff: { designs: { updated: [{ design: { id }, diff }] } },
    };
  },
  "semio.kit.deleteDesign": (context: KitCommandContext, id: Id): KitCommandResult => {
    return {
      diff: { designs: { removed: [{ id }] } },
    };
  },
  "semio.kit.createQuality": (context: KitCommandContext, quality: Quality): KitCommandResult => {
    return {
      diff: { qualities: { added: [quality] } },
    };
  },
  "semio.kit.updateQuality": (context: KitCommandContext, id: Id, diff: QualityDiff): KitCommandResult => {
    return {
      diff: { qualities: { updated: [{ quality: { id }, diff }] } },
    };
  },
  "semio.kit.deleteQuality": (context: KitCommandContext, id: Id): KitCommandResult => {
    return {
      diff: { qualities: { removed: [{ id }] } },
    };
  },
  "semio.kit.createPort": (context: KitCommandContext, iface: Port): KitCommandResult => {
    return {
      diff: { ports: { added: [iface] } },
    };
  },
  "semio.kit.updatePort": (context: KitCommandContext, id: Id, diff: PortDiff): KitCommandResult => {
    return {
      diff: { ports: { updated: [{ port: { id }, diff }] } },
    };
  },
  "semio.kit.deletePort": (context: KitCommandContext, id: Id): KitCommandResult => {
    return {
      diff: { ports: { removed: [{ id }] } },
    };
  },
  "semio.kit.createTag": (context: KitCommandContext, tag: Tag): KitCommandResult => {
    return {
      diff: { tags: { added: [tag] } },
    };
  },
  "semio.kit.updateTag": (context: KitCommandContext, id: Id, diff: TagDiff): KitCommandResult => {
    return {
      diff: { tags: { updated: [{ tag: { id }, diff }] } },
    };
  },
  "semio.kit.deleteTag": (context: KitCommandContext, id: Id): KitCommandResult => {
    return {
      diff: { tags: { removed: [{ id }] } },
    };
  },
  "semio.kit.createConcept": (context: KitCommandContext, concept: Concept): KitCommandResult => {
    return {
      diff: { concepts: { added: [concept] } },
    };
  },
  "semio.kit.updateConcept": (context: KitCommandContext, id: Id, diff: ConceptDiff): KitCommandResult => {
    return {
      diff: { concepts: { updated: [{ concept: { id }, diff }] } },
    };
  },
  "semio.kit.deleteConcept": (context: KitCommandContext, id: Id): KitCommandResult => {
    return {
      diff: { concepts: { removed: [{ id }] } },
    };
  },
  "semio.kit.addFile": (context: KitCommandContext, file: SemioFile, blob?: Blob): KitCommandResult => {
    const files: File[] = blob ? [new File([blob], file.name)] : [];
    return {
      diff: { files: { added: [file] } },
      files,
    };
  },
  "semio.kit.addFiles": (context: KitCommandContext, foldersToAdd: Folder[], filesToAdd: { file: SemioFile; blob?: Blob }[]): KitCommandResult => {
    const semioFiles: SemioFile[] = [];
    const files: File[] = [];
    for (const { file, blob } of filesToAdd) {
      semioFiles.push(file);
      if (blob) files.push(new File([blob], file.name));
    }
    return {
      diff: { folders: { added: foldersToAdd }, files: { added: semioFiles } },
      files,
    };
  },
  "semio.kit.updateFile": (context: KitCommandContext, fileId: Url, fileDiff: FileDiff, blob?: Blob): KitCommandResult => {
    const existing = context.kit.files?.find((f) => f.id === fileId);
    const fileName = fileDiff.name ?? existing?.name ?? "file";
    const files: File[] = blob ? [new File([blob], fileName)] : [];
    return {
      diff: { files: { updated: [{ file: { id: fileId }, diff: fileDiff }] } },
      files,
    };
  },
  "semio.kit.removeFile": (context: KitCommandContext, fileId: Url): KitCommandResult => {
    return {
      diff: { files: { removed: [{ id: fileId }] } },
    };
  },
  "semio.kit.createFolder": (context: KitCommandContext, folder: Folder): KitCommandResult => {
    return {
      diff: { folders: { added: [folder] } },
    };
  },
  "semio.kit.updateFolder": (context: KitCommandContext, id: Id, diff: FolderDiff): KitCommandResult => {
    return {
      diff: { folders: { updated: [{ folder: { id }, diff }] } },
    };
  },
  "semio.kit.deleteFolder": (context: KitCommandContext, id: Id): KitCommandResult => {
    return {
      diff: { folders: { removed: [{ id }] } },
    };
  },
  "semio.kit.moveToFolder": (context: KitCommandContext, artifactId: Id, artifactKind: "type" | "design" | "quality" | "file" | "folder", folderId?: Id): KitCommandResult => {
    switch (artifactKind) {
      case "type": {
        const type = context.kit.types?.find((t) => t.id === artifactId);
        if (!type) throw new Error(`Type ${artifactId} not found`);
        const folderDiff = { folder: folderId };
        return { diff: { types: { updated: [{ type: { id: artifactId }, diff: folderDiff }] } } };
      }
      case "design": {
        const design = context.kit.designs?.find((d) => d.id === artifactId);
        if (!design) throw new Error(`Design ${artifactId} not found`);
        if (design.parent) throw new Error("Only protodesigns (designs without parent) can be moved to folders");
        const folderDiff = { folder: folderId };
        return { diff: { designs: { updated: [{ design: { id: artifactId }, diff: folderDiff }] } } };
      }
      case "quality": {
        const folderDiff = { folder: folderId };
        return { diff: { qualities: { updated: [{ quality: { id: artifactId }, diff: folderDiff }] } } };
      }
      case "file": {
        const folderDiff = { folder: folderId ? { id: folderId } : undefined };
        return { diff: { files: { updated: [{ file: { id: artifactId }, diff: folderDiff }] } } };
      }
      case "folder": {
        const parentDiff = { parent: folderId ? { id: folderId } : undefined };
        return { diff: { folders: { updated: [{ folder: { id: artifactId }, diff: parentDiff }] } } };
      }
      default:
        throw new Error(`Unknown artifact kind: ${artifactKind}`);
    }
  },
  "semio.kit.import": (context: KitCommandContext, url: string): KitCommandResult => {
    (async () => {
      try {
        if (url.endsWith(".json")) {
          const response = await fetch(url);
          const kit: Kit = await response.json();
          const filesToFetch: { path: string; url: string }[] = [];
          const extractFileUrls = (obj: any) => {
            if (typeof obj === "object" && obj !== null) {
              if (Array.isArray(obj)) {
                obj.forEach((item) => extractFileUrls(item));
              } else {
                Object.entries(obj).forEach(([key, value]) => {
                  if (key === "url" && typeof value === "string" && !value.startsWith("http")) {
                    filesToFetch.push({ path: value, url: new URL(value, url).href });
                  }
                  extractFileUrls(value);
                });
              }
            }
          };
          extractFileUrls(kit);
          const files: KitCommandResult["files"] = [];
          for (const file of filesToFetch) {
            try {
              const fileResponse = await fetch(file.url);
              const fileBlob = await fileResponse.blob();
              const fileName = file.path.split("/").pop() || file.path;
              files.push(new File([fileBlob], fileName));
            } catch (error) {}
          }
          return {
            diff: {
              name: kit.name,
              description: kit.description,
              version: kit.version,
              types: kit.types ? { added: kit.types } : undefined,
              designs: kit.designs ? { added: kit.designs } : undefined,
              files: kit.files ? { added: kit.files } : undefined,
            },
            files,
          };
        } else {
          const { kit } = await importKit(url);

          return {
            diff: {
              name: kit.name,
              description: kit.description,
              version: kit.version,
              types: kit.types && kit.types.length > 0 ? { added: kit.types } : undefined,
              designs: kit.designs && kit.designs.length > 0 ? { added: kit.designs } : undefined,
              files: kit.files && kit.files.length > 0 ? { added: kit.files } : undefined,
            },
          };
        }
      } catch (error) {
        throw error;
      }
    })();
    return { diff: {} };
  },
  "semio.kit.export": (context: KitCommandContext): KitCommandResult => {
    (async () => {
      try {
        const kit = context.kit;
        const files = new Map<string, Blob>();

        for (const [path, url] of context.fileUrls.entries()) {
          try {
            const response = await fetch(url);
            if (response.ok) {
              const blob = await response.blob();
              files.set(path, blob);
            }
          } catch (error) {
            // File not accessible, skip
          }
        }

        const zipBlob = await exportKit(kit, files);
        const url = URL.createObjectURL(zipBlob);
        const a = document.createElement("a");
        a.href = url;
        a.download = `${kit.name}-${kit.version || "latest"}.semio.zip`;
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
        URL.revokeObjectURL(url);
      } catch (error) {
        throw error;
      }
    })();
    return { diff: {} };
  },
  "semio.kit.addPiece": (context: KitCommandContext, id: Id, piece: Piece): KitCommandResult => {
    return {
      diff: {
        designs: {
          updated: [
            {
              design: { id },
              diff: {
                pieces: {
                  added: [
                    piece.plane || (findDesignInKit(context.kit, id)?.connections ?? []).some((connection) => connection.connected.piece.id === piece.id || connection.connecting.piece.id === piece.id)
                      ? piece
                      : {
                          ...piece,
                          plane: {
                            origin: { x: 0, y: 0, z: 0 },
                            xAxis: { x: 1, y: 0, z: 0 },
                            yAxis: { x: 0, y: 1, z: 0 },
                          },
                        },
                  ],
                },
              },
            },
          ],
        },
      },
    };
  },
  "semio.kit.addPieces": (context: KitCommandContext, id: Id, pieces: Piece[]): KitCommandResult => {
    const design = findDesignInKit(context.kit, id);
    return {
      diff: {
        designs: {
          updated: [
            {
              design: { id },
              diff: {
                pieces: {
                  added: pieces.map((candidate) =>
                    candidate.plane || (design?.connections ?? []).some((connection) => connection.connected.piece.id === candidate.id || connection.connecting.piece.id === candidate.id)
                      ? candidate
                      : {
                          ...candidate,
                          plane: {
                            origin: { x: 0, y: 0, z: 0 },
                            xAxis: { x: 1, y: 0, z: 0 },
                            yAxis: { x: 0, y: 1, z: 0 },
                          },
                        },
                  ),
                },
              },
            },
          ],
        },
      },
    };
  },
  "semio.kit.removePiece": (context: KitCommandContext, id: Id, piece: Id): KitCommandResult => {
    return {
      diff: {
        designs: {
          updated: [
            {
              design: { id },
              diff: { pieces: { removed: [{ id: piece }] } },
            },
          ],
        },
      },
    };
  },
  "semio.kit.removePieces": (context: KitCommandContext, id: Id, pieces: Id[]): KitCommandResult => {
    return {
      diff: {
        designs: {
          updated: [
            {
              design: { id },
              diff: { pieces: { removed: pieces.map((p) => ({ id: p })) } },
            },
          ],
        },
      },
    };
  },
  "semio.kit.addConnection": (context: KitCommandContext, id: Id, connection: Connection): KitCommandResult => {
    return {
      diff: {
        designs: {
          updated: [
            {
              design: { id },
              diff: { connections: { added: [connection] } },
            },
          ],
        },
      },
    };
  },
  "semio.kit.addConnections": (context: KitCommandContext, id: Id, connections: Connection[]): KitCommandResult => {
    return {
      diff: {
        designs: {
          updated: [
            {
              design: { id },
              diff: { connections: { added: connections } },
            },
          ],
        },
      },
    };
  },
  "semio.kit.removeConnection": (context: KitCommandContext, id: Id, connectionId: Id): KitCommandResult => {
    const design = findDesignInKit(context.kit, id);
    const connection = design?.connections?.find((c) => c.id === connectionId);
    if (!connection) return { diff: {} };
    return {
      diff: {
        designs: {
          updated: [
            {
              design: { id },
              diff: { connections: { removed: [{ id: connection.id }] } },
            },
          ],
        },
      },
    };
  },
  "semio.kit.removeConnections": (context: KitCommandContext, id: Id, connectionIds: Id[]): KitCommandResult => {
    return {
      diff: {
        designs: {
          updated: [
            {
              design: { id },
              diff: { connections: { removed: connectionIds.map((connId) => ({ id: connId })) } },
            },
          ],
        },
      },
    };
  },
  "semio.kit.deleteSelected": (context: KitCommandContext, designId: Id, selectedPieces: Id[], selectedConnections: Id[]): KitCommandResult => {
    return {
      diff: {
        designs: {
          updated: [
            {
              design: { id: designId },
              diff: {
                pieces: { removed: selectedPieces.map((pid) => ({ id: pid })) },
                connections: { removed: selectedConnections.map((cid) => ({ id: cid })) },
              },
            },
          ],
        },
      },
    };
  },
};

export async function executeSemioKitCommand(kitStore: KitStore, command: string, origin?: string, ...args: any[]): Promise<KitCommandResult> {
  const callback = semioKitCommandHandlers[command as keyof typeof semioKitCommandHandlers];
  if (!callback) throw new Error(`Command "${command}" not found in kit commands`);
  const context: KitCommandContext = {
    kit: kitStore.getSnapshot().kit,
    fileUrls: getStoredKitFileUrls(kitStore) as Map<Url, Url>,
    origin,
  };
  const result = (callback as any)(context, ...args);
  if (result.diff) {
    kitStore.apply(result.diff, { origin });
  }
  await syncKitFileCommandResult(kitStore, context.kit, command, args, result);
  return result;
}