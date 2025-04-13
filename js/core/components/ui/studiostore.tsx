import * as Y from 'yjs';
import React, { createContext, useContext, useEffect, useState } from 'react';
import { UndoManager } from 'yjs';
import { IndexeddbPersistence } from 'y-indexeddb';

import { } from '@semio/js';
import { Generator } from '@semio/js/lib/utils';

// Public interfaces - these don't expose Yjs types
export interface KitNode {
    guid: string;
    name: string;
    designGuids: string[];
}

interface EditorPresenceState {
    selection: {
        highlight: string;
    };
}

// Internal type definitions used by the store implementation
type YjsKit = {
    name: string;
    designs: Y.Map<string>;
};

class Studio {
    private studioId: string;
    private userId: string;
    private studioDoc: Y.Doc;
    private undoManagers: Map<string, UndoManager>;
    private indexeddbProvider: IndexeddbPersistence;

    constructor(
        id: string = 'semio',
        userId: string = Generator.randomId()
    ) {
        this.studioId = id;
        this.userId = userId;
        this.studioDoc = new Y.Doc();
        this.undoManagers = new Map();
        this.indexeddbProvider = new IndexeddbPersistence(id, this.studioDoc);
        this.indexeddbProvider.whenSynced.then(() => {
            console.log(`Local changes are synchronized for ${id} with user (${this.userId}) with client (${this.studioDoc.clientID})`);
        });
    }

    // Internal method to get the Yjs kit
    private getYjsKit(kitGuid: string): Y.Map<YjsKit> {
        const kitsMap = this.studioDoc.getMap(kitGuid);
        if (!kitsMap.has('root')) {
            const rootKit = new Y.Map<any>();
            rootKit.set('name', '');
            rootKit.set('designs', new Y.Map());
            kitsMap.set('root', rootKit);
        }

        // Create or retrieve undo manager for this kit
        if (!this.undoManagers.has(kitGuid)) {
            // Track the specific shared types we want to observe
            // This is critical: we need to track the kitsMap itself
            const undoManager = new UndoManager(this.studioDoc.getMap(kitGuid), {
                captureTimeout: 0, // Increase capture timeout for better grouping
                trackedOrigins: new Set([this.userId]), // Only track changes from this client
            });

            this.undoManagers.set(kitGuid, undoManager);
            console.log(`Created UndoManager for ${kitGuid} with user ID ${this.studioDoc.clientID}`);

            // Manually trigger the undo stack to check if it's working
            undoManager.on('stack-item-added', () => {
                console.log(`Stack item added to ${kitGuid}, canUndo: ${undoManager.canUndo()}`);
            });

            undoManager.on('stack-item-popped', () => {
                console.log(`Stack item popped from ${kitGuid}, canUndo: ${undoManager.canUndo()}`);
            });
        }

        return kitsMap.get('root') as Y.Map<YjsKit>;
    }

    // Internal method to get a Yjs node
    private getYjsNode(kitGuid: string, guid: string): Y.Map<any> | undefined {
        const kitsMap = this.studioDoc.getMap(kitGuid);
        return kitsMap.get(guid) as Y.Map<any>;
    }

    // Internal method to create a Yjs node
    private createYjsNode(kitGuid: string, guid: string): Y.Map<any> {
        const kitsMap = this.studioDoc.getMap(kitGuid);
        if (!kitsMap.has(guid)) {
            const nodeKit = new Y.Map<any>();
            nodeKit.set('name', '');
            nodeKit.set('designs', new Y.Map());
            kitsMap.set(guid, nodeKit);
        }
        return kitsMap.get(guid) as Y.Map<any>;
    }

    // Public method that returns a KitNode
    getNode(kitGuid: string, guid: string): KitNode | null {
        const yjsNode = this.getYjsNode(kitGuid, guid);
        if (!yjsNode) return null;

        const designs = yjsNode.get('designs') as Y.Map<string>;
        const designGuids = designs ? Array.from(designs.keys()) : [];

        return {
            guid,
            name: yjsNode.get('name') || '',
            designGuids
        };
    }

    // Create a new node and return its public representation
    createNode(kitGuid: string, guid: string): KitNode {
        const yjsNode = this.createYjsNode(kitGuid, guid);
        return {
            guid,
            name: yjsNode.get('name') || '',
            designGuids: []
        };
    }

    // Update a node's name
    updateNodeName(kitGuid: string, guid: string, name: string): void {
        const yjsNode = this.getYjsNode(kitGuid, guid);
        if (yjsNode) {
            // Make sure we have an undo manager first
            if (!this.undoManagers.has(kitGuid)) {
                this.getYjsKit(kitGuid);
            }

            const undoManager = this.undoManagers.get(kitGuid);

            // Create a transaction to ensure this is tracked as one operation
            this.studioDoc.transact(() => {
                yjsNode.set('name', name);
            }, this.userId);

            console.log(`Updated node ${guid} in ${kitGuid}, checking canUndo: ${undoManager?.canUndo()}`);
        }
    }

    // Add a design to a node
    addDesign(kitGuid: string, parentId: string, designGuid: string): KitNode | null {
        const parentNode = this.getYjsNode(kitGuid, parentId);
        if (!parentNode) return null;

        // Make sure we have an undo manager first
        if (!this.undoManagers.has(kitGuid)) {
            this.getYjsKit(kitGuid);
        }

        const undoManager = this.undoManagers.get(kitGuid);

        // Create a transaction to ensure this is tracked as one operation
        this.studioDoc.transact(() => {
            const designNode = this.createYjsNode(kitGuid, designGuid);
            const designs = parentNode.get('designs') as Y.Map<string>;
            designs.set(designGuid, designGuid);
        }, this.userId);

        console.log(`Added design ${designGuid} to ${parentId} in ${kitGuid}, checking canUndo: ${undoManager?.canUndo()}`);

        return this.getNode(kitGuid, designGuid);
    }

    // Delete a design node from its parent
    deleteDesign(kitGuid: string, parentId: string, designGuid: string): boolean {
        const parentNode = this.getYjsNode(kitGuid, parentId);
        if (!parentNode) return false;

        // Make sure we have an undo manager first
        if (!this.undoManagers.has(kitGuid)) {
            this.getYjsKit(kitGuid);
        }

        const undoManager = this.undoManagers.get(kitGuid);
        let success = false;

        // Create a transaction to ensure this is tracked as one operation
        this.studioDoc.transact(() => {
            // Remove the design from the parent's designs map
            const designs = parentNode.get('designs') as Y.Map<string>;
            if (designs.has(designGuid)) {
                designs.delete(designGuid);
                success = true;
            }

            // Delete the design node from the kit map
            const kitsMap = this.studioDoc.getMap(kitGuid);
            if (kitsMap.has(designGuid)) {
                kitsMap.delete(designGuid);
            }
        }, this.userId);

        console.log(`Deleted design ${designGuid} from ${parentId} in ${kitGuid}, success: ${success}, checking canUndo: ${undoManager?.canUndo()}`);

        return success;
    }

    // Observe changes to the internal YDoc
    observeKit(kitGuid: string, callback: () => void): () => void {
        const kitMap = this.studioDoc.getMap(kitGuid);
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
     * @param kitGuid The ID of the kit to initialize
     * @param initialName Optional initial name for the root node
     * @returns The created root node
     */
    initializeKit(kitGuid: string, initialName: string = 'Root'): KitNode {
        // Make sure we have an undo manager first
        if (!this.undoManagers.has(kitGuid)) {
            // This will create the undo manager
            this.getYjsKit(kitGuid);
        }

        const undoManager = this.undoManagers.get(kitGuid);
        const kitsMap = this.studioDoc.getMap(kitGuid);

        // Create a fresh root node within a transaction to enable undo
        this.studioDoc.transact(() => {
            const rootKit = new Y.Map<any>();
            rootKit.set('name', initialName);
            rootKit.set('designs', new Y.Map());
            kitsMap.set('root', rootKit);
        }, this.userId);

        // Log the status of the undo manager
        console.log(`Initialized kit ${kitGuid}, checking canUndo: ${undoManager?.canUndo()}`);

        return this.getNode(kitGuid, 'root') as KitNode;
    }

    /**
     * Checks if a kit exists and has a root node
     * @param kitGuid The ID of the kit to check
     * @returns True if the kit exists and has a root node
     */
    hasKit(kitGuid: string): boolean {
        const kitsMap = this.studioDoc.getMap(kitGuid);
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
            this.indexeddbProvider = new IndexeddbPersistence(this.studioId, this.studioDoc);

            console.log(`Studio data for ${this.studioId} has been cleaned`);
            return this.indexeddbProvider.whenSynced;
        } catch (error) {
            console.error('Error cleaning studio:', error);
            throw error;
        }
    }

    // Focus method to ensure undo/redo capability is activated
    triggerUndoRedoCapability(kitGuid: string): void {
        // Get or create the undo manager
        if (!this.undoManagers.has(kitGuid)) {
            this.getYjsKit(kitGuid);
        }

        // Force a change that can be undone to test the undo functionality
        // This will be immediately discarded, but ensures the UndoManager is working
        const undoManager = this.undoManagers.get(kitGuid);
        if (undoManager) {
            // Create a temporary marker in the document that we'll immediately remove
            // This creates an undoable action
            const kitsMap = this.studioDoc.getMap(kitGuid);
            const tempKey = `_temp_${Date.now()}`;

            this.studioDoc.transact(() => {
                kitsMap.set(tempKey, "test");
            }, this.userId);

            this.studioDoc.transact(() => {
                kitsMap.delete(tempKey);
            }, this.userId);

            console.log(`Triggered undo capability for ${kitGuid}, canUndo: ${undoManager.canUndo()}`);
        }
    }
}

// Create a singleton instance of the Studio
const studioSingleton = new Studio();

// Create context with proper typing
const StudioContext = createContext<Studio | null>(null);

export const StudioProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
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

export function useKit(kitGuid: string) {
    const studio = useStudio();
    const fullKitId = `kit-${kitGuid}`;
    const [nodes, setNodes] = useState<Record<string, KitNode>>({});
    const [canUndo, setCanUndo] = useState<boolean>(false);
    const [canRedo, setCanRedo] = useState<boolean>(false);
    const [hasInitialized, setHasInitialized] = useState<boolean>(studio.hasKit(fullKitId));

    useEffect(() => {
        // Function to load a node and its designs recursively
        const loadNode = (id: string): void => {
            const node = studio.getNode(fullKitId, id);
            if (!node) return;

            setNodes(prev => ({
                ...prev,
                [id]: node
            }));

            // Load all designs
            node.designGuids.forEach(designGuid => {
                loadNode(designGuid);
            });
        };

        // Load the root node to start
        loadNode('root');

        // Update hasInitialized state
        setHasInitialized(studio.hasKit(fullKitId));

        // Update handler for kit changes
        const updateHandler = () => {
            loadNode('root');
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
        getNode: (id: string) => nodes[id] || null,
        updateNodeName: (id: string, name: string) =>
            studio.updateNodeName(fullKitId, id, name),
        addDesign: (parentId: string) => {
            const designGuid = `node-${Date.now()}-${Math.floor(Math.random() * 1000)}`;
            return studio.addDesign(fullKitId, parentId, designGuid);
        },
        deleteDesign: (parentId: string, designGuid: string) => {
            return studio.deleteDesign(fullKitId, parentId, designGuid);
        },
        initializeKit: (initialName: string = 'Root') => {
            const rootNode = studio.initializeKit(fullKitId, initialName);
            setHasInitialized(true);
            return rootNode;
        },
        hasInitialized,
        undo: () => studio.undo(fullKitId),
        redo: () => studio.redo(fullKitId),
        canUndo,
        canRedo,
    };
}