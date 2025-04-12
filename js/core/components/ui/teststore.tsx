import * as Y from 'yjs';
import React, { createContext, useContext, useEffect, useState } from 'react';
import { UndoManager } from 'yjs';
import { IndexeddbPersistence } from 'y-indexeddb';

export type Tree = {
    value: string;
    children: Y.Map<string>; // Map of child IDs to child nodes
};

class Studio {
    private studioDoc: Y.Doc;
    private undoManagers: Map<string, UndoManager>;
    private indexeddbProvider: IndexeddbPersistence;
    private studioId: string;

    constructor(
        id: string = 'test-studio',
    ) {
        this.studioId = id;
        this.studioDoc = new Y.Doc();
        this.undoManagers = new Map();
        this.indexeddbProvider = new IndexeddbPersistence(id, this.studioDoc);
        this.indexeddbProvider.whenSynced.then(() => {
            console.log(`Local changes are synchronized for ${id}`);
        });
    }

    getTree(treeId: string): Y.Map<Tree> {
        const treesMap = this.studioDoc.getMap(treeId);
        if (!treesMap.has('root')) {
            const rootTree = new Y.Map<any>();
            rootTree.set('value', '');
            rootTree.set('children', new Y.Map());
            treesMap.set('root', rootTree);
        }

        // Create or retrieve undo manager for this tree
        if (!this.undoManagers.has(treeId)) {
            // Create undo manager that tracks the entire YDoc
            // This is crucial - we need to track all shared types across the document
            const undoManager = new UndoManager([treesMap], {
                // Keep track of more operations for complex trees
                captureTimeout: 300,
                // Set a document scope for the UndoManager
                document: this.studioDoc
            });
            this.undoManagers.set(treeId, undoManager);
            console.log(`Created UndoManager for ${treeId}`);
        }

        return treesMap.get('root') as Y.Map<Tree>;
    }

    createTreeNode(treeId: string, id: string): Y.Map<any> {
        const treesMap = this.studioDoc.getMap(treeId);
        if (!treesMap.has(id)) {
            const nodeTree = new Y.Map<any>();
            nodeTree.set('value', '');
            nodeTree.set('children', new Y.Map());
            treesMap.set(id, nodeTree);
        }
        return treesMap.get(id) as Y.Map<any>;
    }

    getTreeNode(treeId: string, id: string): Y.Map<any> | undefined {
        const treesMap = this.studioDoc.getMap(treeId);
        return treesMap.get(id) as Y.Map<any>;
    }

    undo(scope: string) {
        const undoManager = this.undoManagers.get(scope);
        if (undoManager) {
            if (undoManager.canUndo()) {
                console.log(`Undoing change in ${scope}`);
                undoManager.undo();
            } else {
                console.log(`Cannot undo - no more history in ${scope}`);
            }
        } else {
            console.warn(`No UndoManager found for ${scope}`);
        }
    }

    redo(scope: string) {
        const undoManager = this.undoManagers.get(scope);
        if (undoManager) {
            if (undoManager.canRedo()) {
                console.log(`Redoing change in ${scope}`);
                undoManager.redo();
            } else {
                console.log(`Cannot redo - no more history in ${scope}`);
            }
        } else {
            console.warn(`No UndoManager found for ${scope}`);
        }
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

export function useTree(treeId: string) {
    const studio = useStudio();
    const fullTreeId = `tree-${treeId}`;
    const [rootTree, setRootTree] = useState<Y.Map<Tree>>(studio.getTree(fullTreeId));
    const [canUndo, setCanUndo] = useState<boolean>(false);
    const [canRedo, setCanRedo] = useState<boolean>(false);

    useEffect(() => {
        const treeMap = studio.studioDoc.getMap(fullTreeId);

        // Create a more responsive update handler
        const updateHandler = () => {
            setRootTree(treeMap.get('root') as Y.Map<Tree>);
        };

        // Get the undo manager for this tree
        const undoManager = studio['undoManagers'].get(fullTreeId);

        // Initial update
        updateHandler();

        if (undoManager) {
            // Update undo/redo state
            const updateUndoRedoState = () => {
                setCanUndo(undoManager.canUndo());
                setCanRedo(undoManager.canRedo());
            };

            // Initial state
            updateUndoRedoState();

            // Subscribe to undo manager events
            undoManager.on('stack-item-added', updateUndoRedoState);
            undoManager.on('stack-item-popped', updateUndoRedoState);

            // Debug logging
            console.log(`Initialized undo manager for ${fullTreeId}`);
        }

        // Observe all changes to the tree map and its descendants
        treeMap.observeDeep(updateHandler);

        // No need for the interval - proper observation is better

        return () => {
            treeMap.unobserveDeep(updateHandler);
            if (undoManager) {
                undoManager.off('stack-item-added', updateUndoRedoState);
                undoManager.off('stack-item-popped', updateUndoRedoState);
            }
        };
    }, [studio, fullTreeId]);

    return {
        tree: rootTree,
        getNode: (id: string) => studio.getTreeNode(fullTreeId, id),
        createNode: (id: string) => studio.createTreeNode(fullTreeId, id),
        undo: () => studio.undo(fullTreeId),
        redo: () => studio.redo(fullTreeId),
        canUndo,
        canRedo,
    };
}