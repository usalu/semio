import * as Y from 'yjs';
import React, { createContext, useContext, useEffect, useState } from 'react';
import { UndoManager } from 'yjs';
import { IndexeddbPersistence } from 'y-indexeddb';

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
            yjsNode.set('value', value);
        }
    }

    // Add a child to a node
    addChild(treeId: string, parentId: string, childId: string): TreeNode | null {
        const parentNode = this.getYjsNode(treeId, parentId);
        if (!parentNode) return null;

        const childNode = this.createYjsNode(treeId, childId);
        const children = parentNode.get('children') as Y.Map<string>;
        children.set(childId, childId);

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
        const treesMap = this.studioDoc.getMap(treeId);

        // Create a fresh root node
        const rootTree = new Y.Map<any>();
        rootTree.set('value', initialValue);
        rootTree.set('children', new Y.Map());
        treesMap.set('root', rootTree);

        // Ensure undo manager is created
        this.getYjsTree(treeId);

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
        };

        // Update undo/redo state
        const updateUndoRedoState = () => {
            setCanUndo(studio.canUndo(fullTreeId));
            setCanRedo(studio.canRedo(fullTreeId));
        };

        // Initial undo/redo state
        updateUndoRedoState();

        // Set up observers
        const unobserveTree = studio.observeTree(fullTreeId, updateHandler);
        const unsubscribeUndoChanges = studio.subscribeToUndoChanges(fullTreeId, updateUndoRedoState);

        return () => {
            unobserveTree();
            unsubscribeUndoChanges();
        };
    }, [studio, fullTreeId]);

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