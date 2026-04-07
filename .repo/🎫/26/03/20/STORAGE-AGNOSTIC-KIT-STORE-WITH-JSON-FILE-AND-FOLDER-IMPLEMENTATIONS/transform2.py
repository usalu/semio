#!/usr/bin/env python3
"""Phase 2: Fix remaining CollaborativeKitStore references and rewrite hooks."""
import re

FILE = "/workspaces/semio/semio/sketchpad/sketchpad/Sketchpad.tsx"

with open(FILE, "r") as f:
    content = f.read()

total_before = content.count('\n')
print(f"Read {total_before} lines")

# 1. Replace CollaborativeKitStore type references with KitStore
# In type annotations and casts
content = content.replace(
    'as CollaborativeKitStore | null',
    'as KitStore | null'
)
content = content.replace(
    'as CollaborativeKitStore',
    'as KitStore'
)
content = content.replace(
    '(store: CollaborativeKitStore)',
    '(store: KitStore)'
)
content = content.replace(
    'selector?: (store: CollaborativeKitStore) => T',
    'selector?: (store: KitStore) => T'
)
content = content.replace(
    'T | CollaborativeKitStore | null',
    'T | KitStore | null'
)
content = content.replace(
    'T | CollaborativeKitStore',
    'T | KitStore'
)
content = content.replace(
    'abstract kit(): CollaborativeKitStore',
    'abstract kit(): KitStore'
)
content = content.replace(
    "Map<string, CollaborativeKitStore>",
    "Map<string, KitStore>"
)
content = content.replace(
    ": CollaborativeKitStore {",
    ": KitStore {"
)
content = content.replace(
    "kitStore: CollaborativeKitStore",
    "kitStore: KitStore"
)

# 2. Replace new CollaborativeKitStore(...) with InMemoryKitStore
# In SketchpadStore.createKit:
content = content.replace(
    'new CollaborativeKitStore(this, kit, local, remote, this.remote, this.persistenceFactory)',
    'new InMemoryKitStore(kit)'
)
content = content.replace(
    'new CollaborativeKitStore(this, kit, false, false)',
    'new InMemoryKitStore(kit)'
)
content = content.replace(
    'new CollaborativeKitStore(this, kit, local, remote)',
    'new InMemoryKitStore(kit)'
)

# 3. Fix SketchpadStore kitStore wrapper for injected store
# Remove the CollaborativeKitStore wrapping; use the injected store directly
old_inject = """        // Wrap the injected KitStore as a CollaborativeKitStore for internal compatibility.
        // The injected store provides the data; we register it so all existing hooks work.
        const wrappedStore = new InMemoryKitStore(kit);
        this.kits.set(kit.guid, wrappedStore);

        this.syncDoc.transact(() => {
          const kitMetadata = createSyncDocFactory()().createMap<string | boolean>();
          kitMetadata.set("guid", kit.guid);
          kitMetadata.set("local", false);
          kitMetadata.set("remote", false);
          this.syncKits.push([kitMetadata as any]);
        });

        // Forward changes from injected store to internal collaborative store
        this.injectedKitStore.subscribe(() => {
          const newSnap = this.injectedKitStore!.getSnapshot();
          const diff = getKitDiff(wrappedStore.snapshot(), newSnap.kit);
          wrappedStore.change(diff);
        });"""

new_inject = """        // Use the injected KitStore directly.
        this.kits.set(kit.guid, this.injectedKitStore);"""

content = content.replace(old_inject, new_inject)

# 4. Fix useKit to not use useSyncDeep/useSyncOptional (Yjs-specific)
old_useKit = """export function useKit<T>(selector?: (kit: KitShallow | Kit) => T, guid?: Guid, deep: boolean = false): T | KitShallow | Kit | null {
  const store = useSketchpadStore();
  const kitScope = useKitScope();
  const resolvedGuid = guid ?? kitScope?.guid;
  const kitStore = useKitStore(identitySelector, resolvedGuid ?? undefined) as KitStore | null;
  const syncedDeep = useSyncDeep<Kit, T>(kitStore, selector ? selector : (identitySelector as any));
  const synced = useSyncOptional<KitShallow, T>(kitStore as any, selector ? selector : (identitySelector as any));
  if (!resolvedGuid || !kitStore) return null;
  return deep ? syncedDeep : synced;
}"""

new_useKit = """export function useKit<T>(selector?: (kit: Kit) => T, guid?: Guid): T | Kit | null {
  const kitScope = useKitScope();
  const resolvedGuid = guid ?? kitScope?.guid;
  const kitStore = useKitStore(identitySelector, resolvedGuid ?? undefined) as KitStore | null;
  const subscribe = useCallback((cb: () => void) => {
    if (!kitStore) return () => {};
    return kitStore.subscribe(cb);
  }, [kitStore]);
  const getSnapshot = useCallback(() => {
    if (!kitStore) return null;
    const kit = kitStore.getSnapshot().kit;
    return selector ? selector(kit) : kit;
  }, [kitStore, selector]);
  return useSyncExternalStore(subscribe, getSnapshot) as T | Kit | null;
}"""

content = content.replace(old_useKit, new_useKit)

# 5. Fix targeted kit hooks to use KitStore.subscribe + getSnapshot instead of entity-specific methods
# Pattern for each: replace kitStore.onXxxChanged + kitStore.snapshotXxx with kitStore.subscribe + getSnapshot

# useKitTypes
old_types = """  const subscribe = useCallback(
    (callback: () => void) => {
      if (!kitStore) return () => {};
      return kitStore.onTypesChanged((cb: () => void) => {
        cb();
        callback();
        return () => {};
      }, true);
    },
    [kitStore],
  );

  const getSnapshot = useCallback(() => {
    if (!kitStore) return EMPTY_TYPES;
    return kitStore.snapshotTypes();
  }, [kitStore]);"""

new_types = """  const subscribe = useCallback(
    (callback: () => void) => {
      if (!kitStore) return () => {};
      return kitStore.subscribe(callback);
    },
    [kitStore],
  );

  const getSnapshot = useCallback(() => {
    if (!kitStore) return EMPTY_TYPES;
    return kitStore.getSnapshot().kit.types ?? EMPTY_TYPES;
  }, [kitStore]);"""

content = content.replace(old_types, new_types)

# useKitName
old_name = """  const subscribe = useCallback(
    (callback: () => void) => {
      if (!kitStore) return () => {};
      return kitStore.onScalarFieldChanged("name", () => {
        callback();
        return () => {};
      });
    },
    [kitStore],
  );

  const getSnapshot = useCallback(() => {
    if (!kitStore) return "";
    return kitStore.name;
  }, [kitStore]);"""

new_name = """  const subscribe = useCallback(
    (callback: () => void) => {
      if (!kitStore) return () => {};
      return kitStore.subscribe(callback);
    },
    [kitStore],
  );

  const getSnapshot = useCallback(() => {
    if (!kitStore) return "";
    return kitStore.getSnapshot().kit.name ?? "";
  }, [kitStore]);"""

content = content.replace(old_name, new_name)

# useKitDescription
old_desc = """  const subscribe = useCallback(
    (callback: () => void) => {
      if (!kitStore) return () => {};
      return kitStore.onScalarFieldChanged("description", () => {
        callback();
        return () => {};
      });
    },
    [kitStore],
  );

  const getSnapshot = useCallback(() => {
    if (!kitStore) return undefined;
    return kitStore.description;
  }, [kitStore]);"""

new_desc = """  const subscribe = useCallback(
    (callback: () => void) => {
      if (!kitStore) return () => {};
      return kitStore.subscribe(callback);
    },
    [kitStore],
  );

  const getSnapshot = useCallback(() => {
    if (!kitStore) return undefined;
    return kitStore.getSnapshot().kit.description;
  }, [kitStore]);"""

content = content.replace(old_desc, new_desc)

# Generic pattern for onXxxChanged + snapshotXxx hooks
hook_replacements = [
    ("onAuthorsChanged", "snapshotAuthors", "EMPTY_AUTHORS", "kit.authors ?? EMPTY_AUTHORS"),
    ("onFilesChanged", "snapshotFiles", "EMPTY_FILES", "kit.files ?? EMPTY_FILES"),
    ("onQualitiesChanged", "snapshotQualities", "EMPTY_QUALITIES", "kit.qualities ?? EMPTY_QUALITIES"),
    ("onDesignsChanged", "snapshotDesigns", "EMPTY_DESIGNS", "kit.designs ?? EMPTY_DESIGNS"),
    ("onFoldersChanged", "snapshotFolders", "EMPTY_FOLDERS", "kit.folders ?? EMPTY_FOLDERS"),
]

for on_method, snap_method, empty, new_snap_expr in hook_replacements:
    old_pattern = f"""  const subscribe = useCallback(
    (callback: () => void) => {{
      if (!kitStore) return () => {{}};
      return kitStore.{on_method}((cb: () => void) => {{
        cb();
        callback();
        return () => {{}};
      }}, true);
    }},
    [kitStore],
  );

  const getSnapshot = useCallback(() => {{
    if (!kitStore) return {empty};
    return kitStore.{snap_method}();
  }}, [kitStore]);"""

    new_pattern = f"""  const subscribe = useCallback(
    (callback: () => void) => {{
      if (!kitStore) return () => {{}};
      return kitStore.subscribe(callback);
    }},
    [kitStore],
  );

  const getSnapshot = useCallback(() => {{
    if (!kitStore) return {empty};
    const kit = kitStore.getSnapshot().kit;
    return {new_snap_expr};
  }}, [kitStore]);"""

    content = content.replace(old_pattern, new_pattern)

# 6. Fix useFileUrls to return empty map
old_fileUrls = """export function useFileUrls(): Map<Url, Url> {
  const kitStore = useKitStore() as KitStore | null;
  if (!kitStore) {
    return new Map();
  }
  return kitStore.fileUrls;
}"""

new_fileUrls = """export function useFileUrls(): Map<Url, Url> {
  return useMemo(() => new Map<Url, Url>(), []);
}"""

content = content.replace(old_fileUrls, new_fileUrls)

# 7. Fix useKitTransaction to use KitStore.transact
old_transaction = """export function useKitTransaction(): Transaction {
  const store = useSketchpadStore();
  const kitScope = useKitScope();
  const kitGuid = kitScope?.guid;
  const getOrigin = useOrigin();

  if (!kitGuid || !store.hasKit(kitGuid)) {
    return {};
  }

  const kitStore = store.kit(kitGuid);
  return {
    start: () => {
      kitStore.syncDoc.transact(() => {}, getOrigin());
    },
    finalize: () => {},
    abort: () => {},
  };
}"""

new_transaction = """export function useKitTransaction(): Transaction {
  const store = useSketchpadStore();
  const kitScope = useKitScope();
  const kitGuid = kitScope?.guid;

  if (!kitGuid || !store.hasKit(kitGuid)) {
    return {};
  }

  return {
    start: () => {},
    finalize: () => {},
    abort: () => {},
  };
}"""

content = content.replace(old_transaction, new_transaction)

# 8. Fix useKitCommands to use executeKitCommand
old_commands_body = """  const kitStore = store.kit(kitGuid);
  return {
    importKit: (url: string) => kitStore.execute("semio.kit.import", getOrigin(), url),
    exportKit: () => kitStore.execute("semio.kit.export", getOrigin()),
    createAuthor: (author: Author) => kitStore.execute("semio.kit.createAuthor", getOrigin(), author),
    updateAuthor: (Guid: Guid, authorDiff: AuthorDiff) => kitStore.execute("semio.kit.updateAuthor", getOrigin(), Guid, authorDiff),
    deleteAuthor: (Guid: Guid) => kitStore.execute("semio.kit.deleteAuthor", getOrigin(), Guid),
    createType: (type: Type) => kitStore.execute("semio.kit.createType", getOrigin(), type),
    updateType: (guid: Guid, diff: TypeDiff) => kitStore.execute("semio.kit.updateType", getOrigin(), guid, diff),
    deleteType: (guid: Guid) => kitStore.execute("semio.kit.deleteType", getOrigin(), guid),
    createDesign: (design: Design) => kitStore.execute("semio.kit.createDesign", getOrigin(), design),
    updateDesign: (guid: Guid, diff: DesignDiff) => kitStore.execute("semio.kit.updateDesign", getOrigin(), guid, diff),
    deleteDesign: (guid: Guid) => kitStore.execute("semio.kit.deleteDesign", getOrigin(), guid),
    createQuality: (quality: Quality) => kitStore.execute("semio.kit.createQuality", getOrigin(), quality),
    updateQuality: (guid: Guid, diff: QualityDiff) => kitStore.execute("semio.kit.updateQuality", getOrigin(), guid, diff),
    deleteQuality: (guid: Guid) => kitStore.execute("semio.kit.deleteQuality", getOrigin(), guid),
    createPort: (iface: Port) => kitStore.execute("semio.kit.createPort", getOrigin(), iface),
    updatePort: (guid: Guid, diff: PortDiff) => kitStore.execute("semio.kit.updatePort", getOrigin(), guid, diff),
    deletePort: (guid: Guid) => kitStore.execute("semio.kit.deletePort", getOrigin(), guid),
    createTag: (tag: Tag) => kitStore.execute("semio.kit.createTag", getOrigin(), tag),
    updateTag: (guid: Guid, diff: TagDiff) => kitStore.execute("semio.kit.updateTag", getOrigin(), guid, diff),
    deleteTag: (guid: Guid) => kitStore.execute("semio.kit.deleteTag", getOrigin(), guid),
    createConcept: (concept: Concept) => kitStore.execute("semio.kit.createConcept", getOrigin(), concept),
    deleteConcept: (guid: Guid) => kitStore.execute("semio.kit.deleteConcept", getOrigin(), guid),
    addFile: (file: SemioFile, blob?: Blob) => kitStore.execute("semio.kit.addFile", getOrigin(), file, blob),
    updateFile: (url: Url, fileDiff: FileDiff, blob?: Blob) => kitStore.execute("semio.kit.updateFile", getOrigin(), url, fileDiff, blob),
    removeFile: (url: Url) => kitStore.execute("semio.kit.removeFile", getOrigin(), url),
    createFolder: (folder: Folder) => kitStore.execute("semio.kit.createFolder", getOrigin(), folder),
    updateFolder: (guid: Guid, folderDiff: FolderDiff) => kitStore.execute("semio.kit.updateFolder", getOrigin(), guid, folderDiff),
    deleteFolder: (guid: Guid) => kitStore.execute("semio.kit.deleteFolder", getOrigin(), guid),
    moveToFolder: (artifactKind: string, artifactGuid: Guid, folderGuid: Guid | null) => kitStore.execute("semio.kit.moveToFolder", getOrigin(), artifactGuid, artifactKind, folderGuid),
    addPiece: (design: Guid, piece: Piece) => kitStore.execute("semio.kit.addPiece", getOrigin(), design, piece),
    addPieces: (design: Guid, pieces: Piece[]) => kitStore.execute("semio.kit.addPieces", getOrigin(), design, pieces),
    removePiece: (design: Guid, piece: Guid) => kitStore.execute("semio.kit.removePiece", getOrigin(), design, piece),
    removePieces: (design: Guid, pieces: Guid[]) => kitStore.execute("semio.kit.removePieces", getOrigin(), design, pieces),
    addConnection: (design: Guid, connection: Connection) => kitStore.execute("semio.kit.addConnection", getOrigin(), design, connection),
    addConnections: (design: Guid, connections: Connection[]) => kitStore.execute("semio.kit.addConnections", getOrigin(), design, connections),
    removeConnection: (design: Guid, connection: Guid) => kitStore.execute("semio.kit.removeConnection", getOrigin(), design, connection),
    removeConnections: (design: Guid, connections: Guid[]) => kitStore.execute("semio.kit.removeConnections", getOrigin(), design, connections),
    deleteSelected: (design: Guid, selectedPieces: Guid[], selectedConnections: Guid[]) => kitStore.execute("semio.kit.deleteSelected", getOrigin(), design, selectedPieces, selectedConnections),
  };"""

new_commands_body = """  const kitStore = store.kit(kitGuid);
  return {
    importKit: (url: string) => executeKitCommand(kitStore, "semio.kit.import", getOrigin(), url),
    exportKit: () => executeKitCommand(kitStore, "semio.kit.export", getOrigin()),
    createAuthor: (author: Author) => executeKitCommand(kitStore, "semio.kit.createAuthor", getOrigin(), author),
    updateAuthor: (Guid: Guid, authorDiff: AuthorDiff) => executeKitCommand(kitStore, "semio.kit.updateAuthor", getOrigin(), Guid, authorDiff),
    deleteAuthor: (Guid: Guid) => executeKitCommand(kitStore, "semio.kit.deleteAuthor", getOrigin(), Guid),
    createType: (type: Type) => executeKitCommand(kitStore, "semio.kit.createType", getOrigin(), type),
    updateType: (guid: Guid, diff: TypeDiff) => executeKitCommand(kitStore, "semio.kit.updateType", getOrigin(), guid, diff),
    deleteType: (guid: Guid) => executeKitCommand(kitStore, "semio.kit.deleteType", getOrigin(), guid),
    createDesign: (design: Design) => executeKitCommand(kitStore, "semio.kit.createDesign", getOrigin(), design),
    updateDesign: (guid: Guid, diff: DesignDiff) => executeKitCommand(kitStore, "semio.kit.updateDesign", getOrigin(), guid, diff),
    deleteDesign: (guid: Guid) => executeKitCommand(kitStore, "semio.kit.deleteDesign", getOrigin(), guid),
    createQuality: (quality: Quality) => executeKitCommand(kitStore, "semio.kit.createQuality", getOrigin(), quality),
    updateQuality: (guid: Guid, diff: QualityDiff) => executeKitCommand(kitStore, "semio.kit.updateQuality", getOrigin(), guid, diff),
    deleteQuality: (guid: Guid) => executeKitCommand(kitStore, "semio.kit.deleteQuality", getOrigin(), guid),
    createPort: (iface: Port) => executeKitCommand(kitStore, "semio.kit.createPort", getOrigin(), iface),
    updatePort: (guid: Guid, diff: PortDiff) => executeKitCommand(kitStore, "semio.kit.updatePort", getOrigin(), guid, diff),
    deletePort: (guid: Guid) => executeKitCommand(kitStore, "semio.kit.deletePort", getOrigin(), guid),
    createTag: (tag: Tag) => executeKitCommand(kitStore, "semio.kit.createTag", getOrigin(), tag),
    updateTag: (guid: Guid, diff: TagDiff) => executeKitCommand(kitStore, "semio.kit.updateTag", getOrigin(), guid, diff),
    deleteTag: (guid: Guid) => executeKitCommand(kitStore, "semio.kit.deleteTag", getOrigin(), guid),
    createConcept: (concept: Concept) => executeKitCommand(kitStore, "semio.kit.createConcept", getOrigin(), concept),
    deleteConcept: (guid: Guid) => executeKitCommand(kitStore, "semio.kit.deleteConcept", getOrigin(), guid),
    addFile: (file: SemioFile, blob?: Blob) => executeKitCommand(kitStore, "semio.kit.addFile", getOrigin(), file, blob),
    updateFile: (url: Url, fileDiff: FileDiff, blob?: Blob) => executeKitCommand(kitStore, "semio.kit.updateFile", getOrigin(), url, fileDiff, blob),
    removeFile: (url: Url) => executeKitCommand(kitStore, "semio.kit.removeFile", getOrigin(), url),
    createFolder: (folder: Folder) => executeKitCommand(kitStore, "semio.kit.createFolder", getOrigin(), folder),
    updateFolder: (guid: Guid, folderDiff: FolderDiff) => executeKitCommand(kitStore, "semio.kit.updateFolder", getOrigin(), guid, folderDiff),
    deleteFolder: (guid: Guid) => executeKitCommand(kitStore, "semio.kit.deleteFolder", getOrigin(), guid),
    moveToFolder: (artifactKind: string, artifactGuid: Guid, folderGuid: Guid | null) => executeKitCommand(kitStore, "semio.kit.moveToFolder", getOrigin(), artifactGuid, artifactKind, folderGuid),
    addPiece: (design: Guid, piece: Piece) => executeKitCommand(kitStore, "semio.kit.addPiece", getOrigin(), design, piece),
    addPieces: (design: Guid, pieces: Piece[]) => executeKitCommand(kitStore, "semio.kit.addPieces", getOrigin(), design, pieces),
    removePiece: (design: Guid, piece: Guid) => executeKitCommand(kitStore, "semio.kit.removePiece", getOrigin(), design, piece),
    removePieces: (design: Guid, pieces: Guid[]) => executeKitCommand(kitStore, "semio.kit.removePieces", getOrigin(), design, pieces),
    addConnection: (design: Guid, connection: Connection) => executeKitCommand(kitStore, "semio.kit.addConnection", getOrigin(), design, connection),
    addConnections: (design: Guid, connections: Connection[]) => executeKitCommand(kitStore, "semio.kit.addConnections", getOrigin(), design, connections),
    removeConnection: (design: Guid, connection: Guid) => executeKitCommand(kitStore, "semio.kit.removeConnection", getOrigin(), design, connection),
    removeConnections: (design: Guid, connections: Guid[]) => executeKitCommand(kitStore, "semio.kit.removeConnections", getOrigin(), design, connections),
    deleteSelected: (design: Guid, selectedPieces: Guid[], selectedConnections: Guid[]) => executeKitCommand(kitStore, "semio.kit.deleteSelected", getOrigin(), design, selectedPieces, selectedConnections),
  };"""

content = content.replace(old_commands_body, new_commands_body)

# 9. Fix SketchpadStore.createKit - remove entity store observer and syncDoc.transact
old_createKit = """  createKit = (kit: Kit, local?: boolean, remote?: boolean) => {
    const kitStore = new InMemoryKitStore(kit);
    this.kits.set(kit.guid, kitStore);
    kitStore.onChanged((cb: () => void) => {
      cb();
      this.kitShallowsVersion++;
      return () => {};
    });

    this.syncDoc.transact(() => {
      const kitMetadata = createSyncDocFactory()().createMap<string | boolean>();
      kitMetadata.set("guid", kit.guid);
      kitMetadata.set("local", local || false);
      kitMetadata.set("remote", remote || false);
      this.syncKits.push([kitMetadata as any]);
    });

    this.kitShallowsVersion++;
    this.kitCreatedSubscribers.forEach((subscriber) => subscriber());
  };"""

new_createKit = """  createKit = (kit: Kit, local?: boolean, remote?: boolean) => {
    const kitStore = new InMemoryKitStore(kit);
    this.kits.set(kit.guid, kitStore);

    kitStore.subscribe(() => {
      this.kitShallowsVersion++;
    });

    this.kitShallowsVersion++;
    this.kitCreatedSubscribers.forEach((subscriber) => subscriber());
  };"""

content = content.replace(old_createKit, new_createKit)

# 10. Fix SketchpadStore.kit() return type
content = content.replace(
    'kit(guid: string): KitStore {',
    'kit(guid: string): KitStore {'
)

# 11. Fix SketchpadStore.executeCommand importKit dispatch to use executeKitCommand
old_import_cmd = """    if (command === "semio.sketchpad.importKit") {
      const Guid = rest[0] as Guid;
      const url = rest[1] as string;
      const kitStore = this.kits.get(Guid);
      if (kitStore) {
        await kitStore.execute("semio.kit.import", origin, url);
      }
      return {} as T;
    }"""

new_import_cmd = """    if (command === "semio.sketchpad.importKit") {
      const Guid = rest[0] as Guid;
      const url = rest[1] as string;
      const kitStore = this.kits.get(Guid);
      if (kitStore) {
        await executeKitCommand(kitStore, "semio.kit.import", origin, url);
      }
      return {} as T;
    }"""

content = content.replace(old_import_cmd, new_import_cmd)

# 12. Fix SketchpadStore.dumpState - kitStore.snapshot() -> kitStore.getSnapshot().kit  
content = content.replace(
    'kit: kitStore.snapshot(),',
    'kit: kitStore.getSnapshot().kit,'
)

# 13. Fix loadState - uses loadKitFilesFromPublic with CollaborativeKitStore param
# Change this.kit(kit.guid) which returns KitStore now
old_loadState_kit = """        this.createKit(kit, local, remote);
        const kitStore = this.kit(kit.guid);
        this.loadKitFilesFromPublic(kit.guid, kitStore);"""
new_loadState_kit = """        this.createKit(kit, local, remote);
        this.loadKitFilesFromPublic(kit.guid);"""
content = content.replace(old_loadState_kit, new_loadState_kit)

# 14. Fix loadKitFilesFromPublic signature - remove CollaborativeKitStore param
old_loadKit = "private loadKitFilesFromPublic = async (kitGuid: string, kitStore: KitStore) => {"
new_loadKit = "private loadKitFilesFromPublic = async (kitGuid: string) => {"
content = content.replace(old_loadKit, new_loadKit)

# And fix the execute call inside loadKitFilesFromPublic
old_loadKit_exec = 'await kitStore.execute("semio.kit.addFile", "system.loadKitFiles", file, fileBlob);'
new_loadKit_exec = """const loadKitStore = this.kits.get(kitGuid);
              if (loadKitStore) await executeKitCommand(loadKitStore, "semio.kit.addFile", "system.loadKitFiles", file, fileBlob);"""
content = content.replace(old_loadKit_exec, new_loadKit_exec)

# 15. Fix SketchpadStore serialization - kitStore.snapshot()
old_ser = """      const firstKit = this.kits.values().next();
          serializedState = firstKit.done ? {} : firstKit.value.snapshot();"""
new_ser = """      const firstKit = this.kits.values().next();
          serializedState = firstKit.done ? {} : firstKit.value.getSnapshot().kit;"""
content = content.replace(old_ser, new_ser)

with open(FILE, "w") as f:
    f.write(content)

remaining = content.count("CollaborativeKitStore")
print(f"Remaining CollaborativeKitStore references: {remaining}")
total_after = content.count('\n')
print(f"Final line count: {total_after}")
