import * as Y from 'yjs';
import React, { createContext, useContext, useEffect, useState } from 'react';
import { UndoManager } from 'yjs';
import { IndexeddbPersistence } from 'y-indexeddb';

export type Tree = {
    value: string;
    children: Y.Map<Tree>;
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

    undo(scope: string) {
        const undoManager = this.undoManagers.get(scope);
        if (undoManager) undoManager.undo();
    }

    redo(scope: string) {
        const undoManager = this.undoManagers.get(scope);
        if (undoManager) undoManager.redo();
    }
}


const StudioContext = createContext<Studio | null>(null);

export function useStudio() {
    const studio = useContext(StudioContext);
    if (!studio) throw new Error('Test studio not found');
    return studio;
}

export function useTree(treeId: string) {
    const studio = useStudio();
    const [tree, setType] = useState<Y.Map<any>>(studio.getMap(`tree-${treeId}`));

    useEffect(() => {
        const treeMap = studio.getMap(`tree-${treeId}`);
        const updateHandler = () => setType(new Y.Map(treeMap.toJSON()));

        treeMap.observe(updateHandler);
        return () => treeMap.unobserve(updateHandler);
    }, [studio, treeId]);

    return {
        tree,
        undo: () => studio.undo(`tree-${treeId}`),
        redo: () => studio.redo(`tree-${treeId}`),
    };
}