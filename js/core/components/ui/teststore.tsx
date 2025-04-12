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

    constructor(
        id: string = 'test-studio',
    ) {
        this.studioDoc = new Y.Doc();
        this.undoManagers = new Map();
        const indexeddbProvider = new IndexeddbPersistence(id, this.studioDoc);
        indexeddbProvider.whenSynced.then(() => {
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
            this.undoManagers.set(treeId, new UndoManager(treesMap));
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
        if (undoManager) undoManager.undo();
    }

    redo(scope: string) {
        const undoManager = this.undoManagers.get(scope);
        if (undoManager) undoManager.redo();
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

    useEffect(() => {
        const treeMap = studio.getTree(fullTreeId);
        const updateHandler = () => {
            setRootTree(treeMap);
        };

        treeMap.observe(updateHandler);
        return () => treeMap.unobserve(updateHandler);
    }, [studio, fullTreeId]);

    return {
        tree: rootTree,
        getNode: (id: string) => studio.getTreeNode(fullTreeId, id),
        createNode: (id: string) => studio.createTreeNode(fullTreeId, id),
        undo: () => studio.undo(fullTreeId),
        redo: () => studio.redo(fullTreeId),
    };
}