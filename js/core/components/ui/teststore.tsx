// #region Header

// teststore.tsx

// 2025 Ueli Saluz

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion
import React, { createContext, useContext, useEffect, useState } from 'react';
import { IndexeddbPersistence } from 'y-indexeddb';
import * as Y from 'yjs';
import { UndoManager } from 'yjs';

// Public interfaces - these don't expose Yjs types
export interface TreeNode {
    id: string;
    value: string;
    childIds: string[];
}

// Internal type definitions used by the store implementation
type YjsTree = {
    value: string;
    children: Y.Map<string>;
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

    // Internal method to get the Yjs tree
    private getYjsTree(treeId: string): Y.Map<YjsTree> {
        const treesMap = this.studioDoc.getMap(treeId);
        if (!treesMap.has('root')) {
            const rootTree = new Y.Map<any>();
            rootTree.set('value', '');
            rootTree.set('children', new Y.Map());
            treesMap.set('root', rootTree);
        }

        // Create or retrieve undo manager for this tree
        if (!this.undoManagers.has(treeId)) {
            // Track the specific shared types we want to observe
            // This is critical: we need to track the treesMap itself
            const undoManager = new UndoManager(this.studioDoc.getMap(treeId), {
                captureTimeout: 500, // Increase capture timeout for better grouping
                trackedOrigins: new Set([this.studioDoc.clientID]), // Only track changes from this client
            });

            this.undoManagers.set(treeId, undoManager);
            console.log(`Created UndoManager for ${treeId} with client ID ${this.studioDoc.clientID}`);

            // Manually trigger the undo stack to check if it's working
            undoManager.on('stack-item-added', () => {
                console.log(`Stack item added to ${treeId}, canUndo: ${undoManager.canUndo()}`);
            });

            undoManager.on('stack-item-popped', () => {
                console.log(`Stack item popped from ${treeId}, canUndo: ${undoManager.canUndo()}`);
            });
        }

        return treesMap.get('root') as Y.Map<YjsTree>;
    }

    // Internal method to get a Yjs node
    private getYjsNode(treeId: string, id: string): Y.Map<any> | undefined {
        const treesMap = this.studioDoc.getMap(treeId);
        return treesMap.get(id) as Y.Map<any>;
    }

    // Internal method to create a Yjs node
    private createYjsNode(treeId: string, id: string): Y.Map<any> {
        const treesMap = this.studioDoc.getMap(treeId);
        if (!treesMap.has(id)) {
            const nodeTree = new Y.Map<any>();
            nodeTree.set('value', '');
            nodeTree.set('children', new Y.Map());
            treesMap.set(id, nodeTree);
        }
        return treesMap.get(id) as Y.Map<any>;
    }

    // Public method that returns a TreeNode
    getNode(treeId: string, id: string): TreeNode | null {
        const yjsNode = this.getYjsNode(treeId, id);
        if (!yjsNode) return null;

        const children = yjsNode.get('children') as Y.Map<string>;
        const childIds = children ? Array.from(children.keys()) : [];

        return {
            id,
            value: yjsNode.get('value') || '',
            childIds
        };
    }

    // Create a new node and return its public representation
    createNode(treeId: string, id: string): TreeNode {
        const yjsNode = this.createYjsNode(treeId, id);
        return {
            id,
            value: yjsNode.get('value') || '',
            childIds: []
        };
    }

    // Update a node's value
    updateNodeValue(treeId: string, id: string, value: string): void {
        const yjsNode = this.getYjsNode(treeId, id);
        if (yjsNode) {
            // Make sure we have an undo manager first
            if (!this.undoManagers.has(treeId)) {
                this.getYjsTree(treeId);
            }

            const undoManager = this.undoManagers.get(treeId);

            // Create a transaction to ensure this is tracked as one operation
            this.studioDoc.transact(() => {
                yjsNode.set('value', value);
            }, undoManager?.trackedOrigins);

            console.log(`Updated node ${id} in ${treeId}, checking canUndo: ${undoManager?.canUndo()}`);
        }
    }

    // Add a child to a node
    addChild(treeId: string, parentId: string, childId: string): TreeNode | null {
        const parentNode = this.getYjsNode(treeId, parentId);
        if (!parentNode) return null;

        // Make sure we have an undo manager first
        if (!this.undoManagers.has(treeId)) {
            this.getYjsTree(treeId);
        }

        const undoManager = this.undoManagers.get(treeId);

        // Create a transaction to ensure this is tracked as one operation
        this.studioDoc.transact(() => {
            const childNode = this.createYjsNode(treeId, childId);
            const children = parentNode.get('children') as Y.Map<string>;
            children.set(childId, childId);
        }, undoManager?.trackedOrigins);

        console.log(`Added child ${childId} to ${parentId} in ${treeId}, checking canUndo: ${undoManager?.canUndo()}`);

        return this.getNode(treeId, childId);
    }

    // Observe changes to the internal YDoc
    observeTree(treeId: string, callback: () => void): () => void {
        const treeMap = this.studioDoc.getMap(treeId);
        treeMap.observeDeep(callback);
        return () => treeMap.unobserveDeep(callback);
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
     * Explicitly initializes a tree with a root node and optional initial value
     * @param treeId The ID of the tree to initialize
     * @param initialValue Optional initial value for the root node
     * @returns The created root node
     */
    initializeTree(treeId: string, initialValue: string = 'Root'): TreeNode {
        // Make sure we have an undo manager first
        if (!this.undoManagers.has(treeId)) {
            // This will create the undo manager
            this.getYjsTree(treeId);
        }

        const undoManager = this.undoManagers.get(treeId);
        const treesMap = this.studioDoc.getMap(treeId);

        // Create a fresh root node within a transaction to enable undo
        this.studioDoc.transact(() => {
            const rootTree = new Y.Map<any>();
            rootTree.set('value', initialValue);
            rootTree.set('children', new Y.Map());
            treesMap.set('root', rootTree);
        }, undoManager?.trackedOrigins);

        // Log the status of the undo manager
        console.log(`Initialized tree ${treeId}, checking canUndo: ${undoManager?.canUndo()}`);

        return this.getNode(treeId, 'root') as TreeNode;
    }

    /**
     * Checks if a tree exists and has a root node
     * @param treeId The ID of the tree to check
     * @returns True if the tree exists and has a root node
     */
    hasTree(treeId: string): boolean {
        const treesMap = this.studioDoc.getMap(treeId);
        return treesMap.has('root');
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
    triggerUndoRedoCapability(treeId: string): void {
        // Get or create the undo manager
        if (!this.undoManagers.has(treeId)) {
            this.getYjsTree(treeId);
        }

        // Force a change that can be undone to test the undo functionality
        // This will be immediately discarded, but ensures the UndoManager is working
        const undoManager = this.undoManagers.get(treeId);
        if (undoManager) {
            // Create a temporary marker in the document that we'll immediately remove
            // This creates an undoable action
            const treesMap = this.studioDoc.getMap(treeId);
            const tempKey = `_temp_${Date.now()}`;

            this.studioDoc.transact(() => {
                treesMap.set(tempKey, "test");
            }, undoManager.trackedOrigins);

            this.studioDoc.transact(() => {
                treesMap.delete(tempKey);
            }, undoManager.trackedOrigins);

            console.log(`Triggered undo capability for ${treeId}, canUndo: ${undoManager.canUndo()}`);
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
    const [nodes, setNodes] = useState<Record<string, TreeNode>>({});
    const [canUndo, setCanUndo] = useState<boolean>(false);
    const [canRedo, setCanRedo] = useState<boolean>(false);
    const [hasInitialized, setHasInitialized] = useState<boolean>(studio.hasTree(fullTreeId));

    useEffect(() => {
        // Function to load a node and its children recursively
        const loadNode = (id: string): void => {
            const node = studio.getNode(fullTreeId, id);
            if (!node) return;

            setNodes(prev => ({
                ...prev,
                [id]: node
            }));

            // Load all children
            node.childIds.forEach(childId => {
                loadNode(childId);
            });
        };

        // Load the root node to start
        loadNode('root');

        // Update hasInitialized state
        setHasInitialized(studio.hasTree(fullTreeId));

        // Update handler for tree changes
        const updateHandler = () => {
            loadNode('root');
            setHasInitialized(studio.hasTree(fullTreeId));
            // Check undo/redo state on every tree change
            updateUndoRedoState();
        };

        // Update undo/redo state
        const updateUndoRedoState = () => {
            const canUndoNow = studio.canUndo(fullTreeId);
            const canRedoNow = studio.canRedo(fullTreeId);

            if (canUndoNow !== canUndo) {
                setCanUndo(canUndoNow);
                console.log(`Updated canUndo to ${canUndoNow} for ${fullTreeId}`);
            }

            if (canRedoNow !== canRedo) {
                setCanRedo(canRedoNow);
                console.log(`Updated canRedo to ${canRedoNow} for ${fullTreeId}`);
            }
        };

        // Initial undo/redo state
        updateUndoRedoState();

        // Set up observers
        const unobserveTree = studio.observeTree(fullTreeId, updateHandler);

        // Create a more aggressive update interval for undo/redo state
        // This ensures UI stays in sync with the actual undo manager state
        const intervalId = setInterval(updateUndoRedoState, 500);

        const unsubscribeUndoChanges = studio.subscribeToUndoChanges(fullTreeId, updateUndoRedoState);

        return () => {
            unobserveTree();
            unsubscribeUndoChanges();
            clearInterval(intervalId);
        };
    }, [studio, fullTreeId, canUndo, canRedo]);

    return {
        nodes,
        getNode: (id: string) => nodes[id] || null,
        updateNodeValue: (id: string, value: string) =>
            studio.updateNodeValue(fullTreeId, id, value),
        addChild: (parentId: string) => {
            const childId = `node-${Date.now()}-${Math.floor(Math.random() * 1000)}`;
            return studio.addChild(fullTreeId, parentId, childId);
        },
        initializeTree: (initialValue: string = 'Root') => {
            const rootNode = studio.initializeTree(fullTreeId, initialValue);
            setHasInitialized(true);
            return rootNode;
        },
        hasInitialized,
        undo: () => studio.undo(fullTreeId),
        redo: () => studio.redo(fullTreeId),
        canUndo,
        canRedo,
    };
}