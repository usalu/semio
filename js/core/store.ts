import * as Y from 'yjs';
import { Kit, Design, Type, Piece, Connection, Representation, Port } from '@semio/js';
import React from 'react';
import { YSweetProvider, useMap, useArray, useYDoc } from '@y-sweet/react';
import { UndoManager } from 'yjs';
import { createContext, useContext } from 'react';

// Core documents
const studioDoc = useYDoc('studio-{studioId}')
const kitDoc = useYDoc('kit-{kitId}')
const designDoc = useYDoc('design-{designId}')
const typeDoc = useYDoc('type-{typeId}')
const pieceDoc = useYDoc('piece-{pieceId}')
const connectionDoc = useYDoc('connection-{connectionId}')
const representationDoc = useYDoc('representation-{representationId}')
const portDoc = useYDoc('port-{portId}')

// Editor state documents (for local undo/redo and selections)
const designEditorDoc = useYDoc('design-editor-{userId}-{editorId}')

interface DesignEditorState {
    selection: {
        pieces: string[];
        connections: string[];
    }
    diagram: {
        zoom: number;
        pan: { x: number, y: number };
    }
    model: {
        camera: {
            position: { x: number, y: number, z: number };
            rotation: { x: number, y: number, z: number };
        }
    }
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

    constructor(studioId: string) {
        // Initialize main studio document
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

    // G

    // Get or create a type document
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

    // Get or create a design document
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

    // Get or create an editor state document
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

// Custom hooks for different editors
export function useTypeEditor(typeId: string) {
    const store = useStudio();
    const typeDoc = store.getTypeDoc(typeId);
    const designEditorDoc = store.getDesignEditorDoc(`design-editor-${designEditorId}`);

    return {
        type: useMap(typeDoc, 'type'),
        editorState: useMap(designEditorDoc, 'state'),
        undo: () => store.undo(`type-${typeId}`),
        redo: () => store.redo(`type-${typeId}`)
    };
}

export function useDesignEditor(designId: string) {
    const store = useStudio();
    const designDoc = store.getDesignDoc(designId);
    const designEditorDoc = store.getDesignEditorDoc(`design-editor-${designId}`);

    return {
        design: useMap(designDoc, 'design'),
        editorState: useMap(designEditorDoc, 'state'),
        undo: () => store.undo(`design-${designId}`),
        redo: () => store.redo(`design-${designId}`)
    };
} 
