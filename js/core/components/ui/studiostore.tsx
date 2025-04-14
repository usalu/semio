import { v4 as uuidv4 } from 'uuid';
import * as Y from 'yjs';
import React, { createContext, useContext, useEffect, useState } from 'react';
import { UndoManager } from 'yjs';
import { IndexeddbPersistence } from 'y-indexeddb';

import { Generator } from '@semio/js/lib/utils';
import { Kit } from '@semio/js/semio';


export interface TypeNode {
    uuid: string;
    name: string;
}

export interface DesignNode {
    uuid: string;
    name: string;
}

export interface KitNode {
    uuid: string;
    name: string;
    designUuids: string[];
    typeUuids?: string[];
}

export interface DesignEditorState {
    selection: {
        pieceUuids: string[];
        connectionUuids: string[];
    };
}

type YType = {
    name: string;
}

type YDesign = {
    name: string;
}

type YKit = {
    name: string;
    types: Y.Map<YType>;
    designs: Y.Map<YDesign>;
};

type YStudio = {
    kits: Y.Map<YKit>;
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

    // Internal method to get the Yjs kit
    private getYKit(kitUuid: string): YKit | null {
        const kitsMap = this.studioDoc.getMap('kits') as Y.Map<YKit>;
        if (!kitsMap.has(kitUuid)) return null;

        // TODO: Track each editor separately
        if (!this.undoManagers.has(kitUuid)) {
            const undoManager = new UndoManager(this.studioDoc.getMap(kitUuid), {
                captureTimeout: 0,
                trackedOrigins: new Set([this.userId]),
            });
            this.undoManagers.set(kitUuid, undoManager);
            console.log(`Created UndoManager for ${kitUuid} with user ID ${this.studioDoc.clientID}`);
        }

        return kitsMap.get(kitUuid) as YKit;
    }


    private createYKit(uuid: string): Y.Map<any> {
        const kitsMap = this.studioDoc.getMap(uuid);
        if (!kitsMap.has(uuid)) {
            const kitNode = new Y.Map<any>();
            kitNode.set('name', '');
            kitNode.set('designs', new Y.Map());
            kitNode.set('types', new Y.Map());
            kitsMap.set(uuid, kitNode);
        }
        return kitsMap.get(uuid) as Y.Map<any>;
    }

    getKit(uuid: string): KitNode | null {
        const yKit = this.getYKit(uuid);
        if (!yKit) return null;
        return {
            uuid,
            name: yKit.get('name') || '',
            designUuids: Array.from(yKit.get('designs').keys()),
            typeUuids: Array.from(yKit.get('types').keys())
        };
    }

    createKit(kit: Kit): KitNode {
        const uuid = uuidv4();

        return {
            uuid,
            name: yKit.get('name') || '',
            designUuids: []
        };
    }

    // Update a node's name
    updateKitName(kitUuid: string, uuid: string, name: string): void {
        const yKit = this.getYKit(kitUuid, uuid);
        if (yKit) {
            // Make sure we have an undo manager first
            if (!this.undoManagers.has(kitUuid)) {
                this.getYKit(kitUuid);
            }

            const undoManager = this.undoManagers.get(kitUuid);

            // Create a transaction to ensure this is tracked as one operation
            this.studioDoc.transact(() => {
                yKit.set('name', name);
            }, this.userId);

            console.log(`Updated node ${uuid} in ${kitUuid}, checking canUndo: ${undoManager?.canUndo()}`);
        }
    }

    // Add a design to a node
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

    // Delete a design node from its parent
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

            // Delete the design node from the kit map
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
     * Explicitly initializes a kit with a root node and optional initial name
     * @param kitUuid The ID of the kit to initialize
     * @param initialName Optional initial name for the root node
     * @returns The created root node
     */
    initializeKit(kitUuid: string, initialName: string = 'Root'): KitNode {
        // Make sure we have an undo manager first
        if (!this.undoManagers.has(kitUuid)) {
            // This will create the undo manager
            this.getYKit(kitUuid);
        }

        const undoManager = this.undoManagers.get(kitUuid);
        const kitsMap = this.studioDoc.getMap(kitUuid);

        // Create a fresh root node within a transaction to enable undo
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
     * Checks if a kit exists and has a root node
     * @param kitUuid The ID of the kit to check
     * @returns True if the kit exists and has a root node
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

export function useKit(kitUuid: string) {
    const studio = useStudio();
    const fullKitId = `kit-${kitUuid}`;
    const [nodes, setKits] = useState<Record<string, KitNode>>({});
    const [canUndo, setCanUndo] = useState<boolean>(false);
    const [canRedo, setCanRedo] = useState<boolean>(false);
    const [hasInitialized, setHasInitialized] = useState<boolean>(studio.hasKit(fullKitId));

    useEffect(() => {
        // Function to load a node and its designs recursively
        const loadKit = (id: string): void => {
            const node = studio.getKit(fullKitId, id);
            if (!node) return;

            setKits(prev => ({
                ...prev,
                [id]: node
            }));

            // Load all designs
            node.designUuids.forEach(designUuid => {
                loadKit(designUuid);
            });
        };

        // Load the root node to start
        loadKit('root');

        // Update hasInitialized state
        setHasInitialized(studio.hasKit(fullKitId));

        // Update handler for kit changes
        const updateHandler = () => {
            loadKit('root');
            setHasInitialized(studio.hasKit(fullKitId));
            // Check undo/redo state on every kit change
            updateUndoRedoState();
        };

        // Update undo/redo state
        const updateUndoRedoState = () => {
            const canUndoNow = studio.canUndo(fullKitId);
            const canRedoNow = studio.canRedo(fullKitId);

            if (canUndoNow !== canUndo) {
                setCanUndo(canUndoNow);
                console.log(`Updated canUndo to ${canUndoNow} for ${fullKitId}`);
            }

            if (canRedoNow !== canRedo) {
                setCanRedo(canRedoNow);
                console.log(`Updated canRedo to ${canRedoNow} for ${fullKitId}`);
            }
        };

        // Initial undo/redo state
        updateUndoRedoState();

        // Set up observers
        const unobserveKit = studio.observeKit(fullKitId, updateHandler);

        // Create a more aggressive update interval for undo/redo state
        // This ensures UI stays in sync with the actual undo manager state
        const intervalId = setInterval(updateUndoRedoState, 500);

        const unsubscribeUndoChanges = studio.subscribeToUndoChanges(fullKitId, updateUndoRedoState);

        return () => {
            unobserveKit();
            unsubscribeUndoChanges();
            clearInterval(intervalId);
        };
    }, [studio, fullKitId, canUndo, canRedo]);

    return {
        nodes,
        getKit: (id: string) => nodes[id] || null,
        updateKitName: (id: string, name: string) =>
            studio.updateKitName(fullKitId, id, name),
        addDesign: (parentId: string) => {
            const designUuid = `node-${Date.now()}-${Math.floor(Math.random() * 1000)}`;
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