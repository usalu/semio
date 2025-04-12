import * as Y from 'yjs';
import { Kit, Design, Type, Piece, Connection, Representation, Port } from '@semio/js';
import React, { createContext, useContext, useEffect, useState } from 'react';
import { UndoManager } from 'yjs';

// Core documents
const studioDoc = new Y.Doc();
const kitDoc = new Y.Doc();
const designDoc = new Y.Doc();
const typeDoc = new Y.Doc();
const pieceDoc = new Y.Doc();
const connectionDoc = new Y.Doc();
const representationDoc = new Y.Doc();
const portDoc = new Y.Doc();

// Editor state documents (for local undo/redo and selections)
const designEditorDoc = new Y.Doc();

interface DesignEditorState {
    selection: {
        pieces: string[];
        connections: string[];
    };
    diagram: {
        zoom: number;
        pan: { x: number; y: number };
    };
    model: {
        camera: {
            position: { x: number; y: number; z: number };
            rotation: { x: number; y: number; z: number };
        };
    };
}

class Studio {
    private studioDoc: Y.Doc;
    private kitsDocs: Map<string, Y.Doc>;
    private designsDocs: Map<string, Y.Doc>;
    private typesDocs: Map<string, Y.Doc>;
    private piecesDocs: Map<string, Y.Doc>;
    private connectionsDocs: Map<string, Y.Doc>;
    private representationsDocs: Map<string, Y.Doc>;
    private portsDocs: Map<string, Y.Doc>;
    private designEditorDocs: Map<string, Y.Doc>;
    private undoManagers: Map<string, UndoManager>;

    constructor() {
        this.studioDoc = new Y.Doc();
        this.kitsDocs = new Map();
        this.designsDocs = new Map();
        this.typesDocs = new Map();
        this.piecesDocs = new Map();
        this.connectionsDocs = new Map();
        this.representationsDocs = new Map();
        this.portsDocs = new Map();
        this.designEditorDocs = new Map();
        this.undoManagers = new Map();
    }

    getTypeDoc(typeId: string): Y.Doc {
        if (!this.typesDocs.has(typeId)) {
            const doc = new Y.Doc();
            this.typesDocs.set(typeId, doc);
            const type = doc.getMap('type');
            const undoManager = new UndoManager(type);
            this.undoManagers.set(`type-${typeId}`, undoManager);
        }
        return this.typesDocs.get(typeId)!;
    }

    getDesignDoc(designId: string): Y.Doc {
        if (!this.designsDocs.has(designId)) {
            const doc = new Y.Doc();
            this.designsDocs.set(designId, doc);
            const design = doc.getMap('design');
            const undoManager = new UndoManager(design);
            this.undoManagers.set(`design-${designId}`, undoManager);
        }
        return this.designsDocs.get(designId)!;
    }

    getDesignEditorDoc(designEditorId: string): Y.Doc {
        if (!this.designEditorDocs.has(designEditorId)) {
            const doc = new Y.Doc();
            this.designEditorDocs.set(designEditorId, doc);
            const state = doc.getMap('state');
            const undoManager = new UndoManager(state);
            this.undoManagers.set(`design-editor-${designEditorId}`, undoManager);
        }
        return this.designEditorDocs.get(designEditorId)!;
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
    const store = useContext(StudioContext);
    if (!store) throw new Error('Studio not found');
    return store;
}

export function useTypeEditor(typeId: string) {
    const store = useStudio();
    const [type, setType] = useState<Y.Map<any>>(store.getTypeDoc(typeId).getMap('type'));

    useEffect(() => {
        const doc = store.getTypeDoc(typeId);
        const typeMap = doc.getMap('type');
        const updateHandler = () => setType(new Y.Map(typeMap.toJSON()));

        typeMap.observe(updateHandler);
        return () => typeMap.unobserve(updateHandler);
    }, [store, typeId]);

    return {
        type,
        undo: () => store.undo(`type-${typeId}`),
        redo: () => store.redo(`type-${typeId}`),
    };
}

export function useDesignEditor(designId: string) {
    const store = useStudio();
    const [design, setDesign] = useState<Y.Map<any>>(store.getDesignDoc(designId).getMap('design'));

    useEffect(() => {
        const doc = store.getDesignDoc(designId);
        const designMap = doc.getMap('design');
        const updateHandler = () => setDesign(new Y.Map(designMap.toJSON()));

        designMap.observe(updateHandler);
        return () => designMap.unobserve(updateHandler);
    }, [store, designId]);

    return {
        design,
        undo: () => store.undo(`design-${designId}`),
        redo: () => store.redo(`design-${designId}`),
    };
}
