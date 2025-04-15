import { v4 as uuidv4 } from 'uuid';
import * as Y from 'yjs';
import React, { createContext, useContext, useEffect, useState } from 'react';
import { UndoManager } from 'yjs';
import { IndexeddbPersistence } from 'y-indexeddb';

import { Generator } from '@semio/js/lib/utils';
import { Kit } from '@semio/js/semio';


export interface DesignEditorState {
    selection: {
        pieceUuids: string[];
        connectionUuids: string[];
    };
}

type YQuality = {
    name: string;

}

type YType = {
    name: string;
}

type YDesign = {
    name: string;
}

type YKit = {
    name: string;
    types: Y.Array<YType>;
    designs: Y.Array<YDesign>;
    qualities: Y.Array<YQuality>;
};

type YStudio = {
    kits: Y.Array<YKit>;
    designEditorStates: Y.Map<DesignEditorState>;
}

class Studio {
    private userId: string;
    private studioDoc: Y.Doc;
    private undoManagers: Map<string, Map<string, UndoManager>>;
    private indexeddbProvider: IndexeddbPersistence;

    constructor(
        userId: string = Generator.randomId()
    ) {
        this.userId = userId;
        this.studioDoc = new Y.Doc();
        this.undoManagers = new Map();
        this.indexeddbProvider = new IndexeddbPersistence(userId, this.studioDoc);
        this.indexeddbProvider.whenSynced.then(() => {
            console.log(`Local changes are synchronized for user (${this.userId}) with client (${this.studioDoc.clientID})`);
        });
    }

    private createYKit(kit: Kit): Y.Map<any> {
        const yKit = new Y.Map<any>();
        yKit.set('uri', kit.uri);
        yKit.set('name', kit.name);
        yKit.set('description', kit.description || '');
        yKit.set('icon', kit.icon || '');
        yKit.set('image', kit.image || '');
        yKit.set('preview', kit.preview || '');
        yKit.set('version', kit.version || '');
        yKit.set('remote', kit.remote || '');
        yKit.set('homepage', kit.homepage || '');
        yKit.set('license', kit.license || []);
        yKit.set('designs', new Y.Map());
        yKit.set('types', new Y.Map());
        yKit.set('qualities', new Y.Map());
        this.studioDoc.getMap('kits').set(kit.uri, yKit);
        return yKit
    }

    private updateYKit(kit: Kit): Y.Map<any> | null {
        const yKit = this.getYKit(kit.uri);
        if (!yKit) return null;
        if (kit.name !== "") yKit.set('name', kit.name);
        if (kit.description !== undefined && kit.description !== "") yKit.set('description', kit.description);
        if (kit.icon !== undefined && kit.icon !== "") yKit.set('icon', kit.icon);
        if (kit.image !== undefined && kit.image !== "") yKit.set('image', kit.image);
        if (kit.preview !== undefined && kit.preview !== "") yKit.set('preview', kit.preview);
        if (kit.version !== undefined && kit.version !== "") yKit.set('version', kit.version);
        if (kit.remote !== undefined && kit.remote !== "") yKit.set('remote', kit.remote);
        if (kit.homepage !== undefined && kit.homepage !== "") yKit.set('homepage', kit.homepage);
        if (kit.license !== undefined && kit.license !== "") yKit.set('license', kit.license);
        // TODO: Update designs and types
        // if (kit.designs !== undefined && kit.designs.length > 0) {
        //     const designs = yKit.get('designs') as Y.Map<any>;
        //     kit.designs.forEach(design => {
        //         if (designs.has(design.name)) {
        //             const sameDesigns = designs.get(design.name) as Y.Map<any>;
        //             if (sameDesigns.has(design.variant || '')) {
        //                 updateYDesign(design);
        //             }
        //             else {
        //                 createYDesign(design);
        //             }
        //         }
        //         else {
        //             createYDesign(d)
        //         }
        //     });
        // }
        if (kit.qualities)
    }

    getKit(uri: string): Kit | null {
        const yKit = this.getYKit(uri);
        if (!yKit) return null;
        return {
            name: yKit.get('name'),
            description: yKit.get('description'),
            icon: yKit.get('icon'),
            image: yKit.get('image'),
            preview: yKit.get('preview'),
            version: yKit.get('version'),
            remote: yKit.get('remote'),
            homepage: yKit.get('homepage'),
            license: yKit.get('license'),
            uri: uri,
            designs: Array.from(yKit.get('designs').keys()),
            types: Array.from(yKit.get('types').keys())
        };
    }

    private getYKit(uri: string): Y.Map<any> | undefined {
        return this.studioDoc.getMap('kits').get(uri) as Y.Map<any> | undefined;
    }

    createKit(kit: Kit): KitNode {


        return {
            uuid,
            name: yKit.get('name') || '',
            designUuids: []
        };
    }

    // Add a design to a kit
    addDesign(kitUuid: string, parentId: string, designUuid: string): KitNode | null {
        const parentKit = this.getYKit(kitUuid, parentId);
        if (!parentKit) return null;

        // Make sure we have an undo manager first
        if (!this.undoManagers.has(kitUuid)) {
            this.getYKit(kitUuid);
        }

        const undoManager = this.undoManagers.get(kitUuid);

        // Create a transaction to ensure this is tracked as one operation
        this.studioDoc.transact(() => {
            const designKit = this.createYKit(kitUuid, designUuid);
            const designs = parentKit.get('designs') as Y.Map<string>;
            designs.set(designUuid, designUuid);
        }, this.userId);

        console.log(`Added design ${designUuid} to ${parentId} in ${kitUuid}, checking canUndo: ${undoManager?.canUndo()}`);

        return this.getKit(kitUuid, designUuid);
    }

    // Delete a design kit from its parent
    deleteDesign(kitUuid: string, parentId: string, designUuid: string): boolean {
        const parentKit = this.getYKit(kitUuid, parentId);
        if (!parentKit) return false;

        // Make sure we have an undo manager first
        if (!this.undoManagers.has(kitUuid)) {
            this.getYKit(kitUuid);
        }

        const undoManager = this.undoManagers.get(kitUuid);
        let success = false;

        // Create a transaction to ensure this is tracked as one operation
        this.studioDoc.transact(() => {
            // Remove the design from the parent's designs map
            const designs = parentKit.get('designs') as Y.Map<string>;
            if (designs.has(designUuid)) {
                designs.delete(designUuid);
                success = true;
            }

            // Delete the design kit from the kit map
            const kitsMap = this.studioDoc.getMap(kitUuid);
            if (kitsMap.has(designUuid)) {
                kitsMap.delete(designUuid);
            }
        }, this.userId);

        console.log(`Deleted design ${designUuid} from ${parentId} in ${kitUuid}, success: ${success}, checking canUndo: ${undoManager?.canUndo()}`);

        return success;
    }

    // Observe changes to the internal YDoc
    observeKit(kitUuid: string, callback: () => void): () => void {
        const kitMap = this.studioDoc.getMap(kitUuid);
        kitMap.observeDeep(callback);
        return () => kitMap.unobserveDeep(callback);
    }

    // Undo/redo operations
    undo(scope: string) {
        const undoManager = this.undoManagers.get(scope);
        if (undoManager && undoManager.canUndo()) {
            console.log(`Undoing change in ${scope}`);
            undoManager.undo();
        }
    }

    redo(scope: string) {
        const undoManager = this.undoManagers.get(scope);
        if (undoManager && undoManager.canRedo()) {
            console.log(`Redoing change in ${scope}`);
            undoManager.redo();
        }
    }

    canUndo(scope: string): boolean {
        const undoManager = this.undoManagers.get(scope);
        return undoManager ? undoManager.canUndo() : false;
    }

    canRedo(scope: string): boolean {
        const undoManager = this.undoManagers.get(scope);
        return undoManager ? undoManager.canRedo() : false;
    }

    subscribeToUndoChanges(
        scope: string,
        callback: () => void
    ): () => void {
        const undoManager = this.undoManagers.get(scope);
        if (!undoManager) return () => { };

        undoManager.on('stack-item-added', callback);
        undoManager.on('stack-item-popped', callback);

        return () => {
            undoManager.off('stack-item-added', callback);
            undoManager.off('stack-item-popped', callback);
        };
    }

    /**
     * Explicitly initializes a kit with a root kit and optional initial name
     * @param kitUuid The ID of the kit to initialize
     * @param initialName Optional initial name for the root kit
     * @returns The created root kit
     */
    initializeKit(kitUuid: string, initialName: string = 'Root'): KitNode {
        // Make sure we have an undo manager first
        if (!this.undoManagers.has(kitUuid)) {
            // This will create the undo manager
            this.getYKit(kitUuid);
        }

        const undoManager = this.undoManagers.get(kitUuid);
        const kitsMap = this.studioDoc.getMap(kitUuid);

        // Create a fresh root kit within a transaction to enable undo
        this.studioDoc.transact(() => {
            const rootKit = new Y.Map<any>();
            rootKit.set('name', initialName);
            rootKit.set('designs', new Y.Map());
            kitsMap.set('root', rootKit);
        }, this.userId);

        // Log the status of the undo manager
        console.log(`Initialized kit ${kitUuid}, checking canUndo: ${undoManager?.canUndo()}`);

        return this.getKit(kitUuid, 'root') as KitNode;
    }

    /**
     * Checks if a kit exists and has a root kit
     * @param kitUuid The ID of the kit to check
     * @returns True if the kit exists and has a root kit
     */
    hasKit(kitUuid: string): boolean {
        const kitsMap = this.studioDoc.getMap(kitUuid);
        return kitsMap.has('root');
    }

    /**
     * Cleans the studio by clearing IndexedDB storage and creating a fresh document
     * @returns Promise that resolves when cleaning is complete
     */
    async clean(): Promise<void> {
        try {
            // Clean up undo managers
            this.undoManagers.forEach(manager => manager.destroy());

            // Destroy the current document
            this.studioDoc.destroy();

            // Clear IndexedDB for this studio
            await this.indexeddbProvider.clearData();

            // Reinitialize with a fresh document
            this.studioDoc = new Y.Doc();
            this.undoManagers = new Map();
            this.indexeddbProvider = new IndexeddbPersistence(this.userId, this.studioDoc);

            console.log(`Studio data for ${this.userId} has been cleaned`);
            return this.indexeddbProvider.whenSynced;
        } catch (error) {
            console.error('Error cleaning studio:', error);
            throw error;
        }
    }

    // Focus method to ensure undo/redo capability is activated
    triggerUndoRedoCapability(kitUuid: string): void {
        // Get or create the undo manager
        if (!this.undoManagers.has(kitUuid)) {
            this.getYKit(kitUuid);
        }

        // Force a change that can be undone to test the undo functionality
        // This will be immediately discarded, but ensures the UndoManager is working
        const undoManager = this.undoManagers.get(kitUuid);
        if (undoManager) {
            // Create a temporary marker in the document that we'll immediately remove
            // This creates an undoable action
            const kitsMap = this.studioDoc.getMap(kitUuid);
            const tempKey = `_temp_${Date.now()}`;

            this.studioDoc.transact(() => {
                kitsMap.set(tempKey, "test");
            }, this.userId);

            this.studioDoc.transact(() => {
                kitsMap.delete(tempKey);
            }, this.userId);

            console.log(`Triggered undo capability for ${kitUuid}, canUndo: ${undoManager.canUndo()}`);
        }
    }

    private createYType(type: Type): Y.Map<any> {
        const yType = new Y.Map<any>();
        yType.set('name', type.name);
        yType.set('description', type.description || '');
        yType.set('icon', type.icon || '');
        yType.set('image', type.image || '');
        yType.set('variant', type.variant || '');
        yType.set('unit', type.unit || '');
        yType.set('ports', new Y.Map());
        yType.set('qualities', new Y.Map());
        yType.set('representations', new Y.Map());
        return yType;
    }

    createType(kitUri: string, type: Type): Type {
        const yKit = this.getYKit(kitUri);
        if (!yKit) throw new Error(`Kit ${kitUri} not found`);

        const types = yKit.get('types') as Y.Map<any>;
        const yType = this.createYType(type);
        types.set(type.name, yType);

        return this.getType(kitUri, type.name);
    }

    getType(kitUri: string, typeName: string): Type | null {
        const yKit = this.getYKit(kitUri);
        if (!yKit) return null;

        const types = yKit.get('types') as Y.Map<any>;
        const yType = types.get(typeName) as Y.Map<any> | undefined;
        if (!yType) return null;

        return {
            name: yType.get('name'),
            description: yType.get('description'),
            icon: yType.get('icon'),
            image: yType.get('image'),
            variant: yType.get('variant'),
            unit: yType.get('unit'),
            ports: Array.from(yType.get('ports').keys()),
            qualities: Array.from(yType.get('qualities').keys()),
            representations: Array.from(yType.get('representations').keys())
        };
    }

    updateType(kitUri: string, type: Type): Type | null {
        const yKit = this.getYKit(kitUri);
        if (!yKit) return null;

        const types = yKit.get('types') as Y.Map<any>;
        const yType = types.get(type.name) as Y.Map<any> | undefined;
        if (!yType) return null;

        if (type.description !== undefined) yType.set('description', type.description);
        if (type.icon !== undefined) yType.set('icon', type.icon);
        if (type.image !== undefined) yType.set('image', type.image);
        if (type.variant !== undefined) yType.set('variant', type.variant);
        if (type.unit !== undefined) yType.set('unit', type.unit);
        // TODO: Update ports, qualities, representations

        return this.getType(kitUri, type.name);
    }

    deleteType(kitUri: string, typeName: string): boolean {
        const yKit = this.getYKit(kitUri);
        if (!yKit) return false;

        const types = yKit.get('types') as Y.Map<any>;
        return types.delete(typeName);
    }
}

// Create a singleton instance of the Studio
const studioSingleton = new Studio();

// Create context with proper typing
const StudioContext = createContext<Studio | null>(null);

export const StudioProvider: React.FC<{ children: React.ReactKit }> = ({ children }) => {
    return (
        <StudioContext.Provider value={studioSingleton}>
            {children}
        </StudioContext.Provider>
    );
};

export function useStudio() {
    const studio = useContext(StudioContext);
    if (!studio) throw new Error('Test studio not found');
    return studio;
}

export function useKit(uri: string) {
    const studio = useStudio();
    const [kits, setKits] = useState<Record<string, Kit>>({});
    const [canUndo, setCanUndo] = useState<boolean>(false);
    const [canRedo, setCanRedo] = useState<boolean>(false);
    const [hasInitialized, setHasInitialized] = useState<boolean>(studio.hasKit(uri));

    useEffect(() => {
        const loadKit = (uri: string): void => {
            const kit = studio.getKit(uri);
            if (!kit) return;

            setKits(prev => ({
                ...prev,
                [uri]: kit
            }));

        };
        setHasInitialized(studio.hasKit(uri));

        // Update handler for kit changes
        const updateHandler = () => {
            loadKit('root');
            setHasInitialized(studio.hasKit(uri));
            // Check undo/redo state on every kit change
            updateUndoRedoState();
        };

        // Update undo/redo state
        const updateUndoRedoState = () => {
            const canUndoNow = studio.canUndo(uri);
            const canRedoNow = studio.canRedo(uri);

            if (canUndoNow !== canUndo) {
                setCanUndo(canUndoNow);
                console.log(`Updated canUndo to ${canUndoNow} for ${uri}`);
            }

            if (canRedoNow !== canRedo) {
                setCanRedo(canRedoNow);
                console.log(`Updated canRedo to ${canRedoNow} for ${uri}`);
            }
        };

        // Initial undo/redo state
        updateUndoRedoState();

        // Set up observers
        const unobserveKit = studio.observeKit(uri, updateHandler);

        // Create a more aggressive update interval for undo/redo state
        // This ensures UI stays in sync with the actual undo manager state
        const intervalId = setInterval(updateUndoRedoState, 500);

        const unsubscribeUndoChanges = studio.subscribeToUndoChanges(uri, updateUndoRedoState);

        return () => {
            unobserveKit();
            unsubscribeUndoChanges();
            clearInterval(intervalId);
        };
    }, [studio, uri, canUndo, canRedo]);

    return {
        kits,
        getKit: (id: string) => kits[id] || null,
        updateKitName: (id: string, name: string) =>
            studio.updateKitName(fullKitId, id, name),
        addDesign: (parentId: string) => {
            const designUuid = `kit-${Date.now()}-${Math.floor(Math.random() * 1000)}`;
            return studio.addDesign(fullKitId, parentId, designUuid);
        },
        deleteDesign: (parentId: string, designUuid: string) => {
            return studio.deleteDesign(fullKitId, parentId, designUuid);
        },
        initializeKit: (initialName: string = 'Root') => {
            const rootKit = studio.initializeKit(fullKitId, initialName);
            setHasInitialized(true);
            return rootKit;
        },
        hasInitialized,
        undo: () => studio.undo(fullKitId),
        redo: () => studio.redo(fullKitId),
        canUndo,
        canRedo,
    };
}