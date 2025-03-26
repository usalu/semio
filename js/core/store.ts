import * as Y from 'yjs';
import { Kit, Design, Type, Piece, Connection, Representation, Port } from '@semio/js';
import React, { createContext, useContext, useEffect, useState } from 'react';
import { UndoManager } from 'yjs';
import { IndexeddbPersistence } from 'y-indexeddb';

export const useTypes = () => []

// type YKit = {
//     types: Y.Map<YType>;
//     designs: Y.Map<YDesign>;
// }

// type YStudio = Y.Doc & {
//     kits: Y.Map<YKit>;
// }

// interface DesignEditorState {
//     selection: {
//         pieces: string[];
//         connections: string[];
//     };
//     design: Y.Map<YDesign>;
// }

// interface DesignEditorPresenceState {
//     diagram: {
//         zoom: number;
//         pan: { x: number; y: number };
//     };
//     model: {
//         camera: {
//             position: { x: number; y: number; z: number };
//             rotation: { x: number; y: number; z: number };
//         };
//     };
// }

// class Studio {
//     private studioDoc: Y.Doc;
//     private undoManagers: Map<string, UndoManager>;

//     constructor(
//         id: string = 'studio',
//         createYDoc: (id: string) => Y.Doc = () => new Y.Doc(),
//         registerAddtionalProviders: (yDoc: Y.Doc, id: string) => void = (yDoc, id) => { }
//     ) {
//         this.studioDoc = createYDoc(id);
//         this.undoManagers = new Map();
//         const indexeddbProvider = new IndexeddbPersistence(id, this.studioDoc);
//         indexeddbProvider.whenSynced.then(() => {
//             console.log(`Local changes are synchronized for ${id}`);
//         });
//         registerAddtionalProviders(this.studioDoc, id);
//     }

//     get kits(): Y.Map<any> {
//         return this.studioDoc.getMap('kits');
//     }

//     getEditor(editorId: string): Y.Map<any> {
//         if (!this.studioDoc.getMap(editorId)) {
//             const editor = this.studioDoc.getMap(editorId);
//             const undoManager = new UndoManager(editor);
//             this.undoManagers.set(editorId, undoManager);
//         }
//         return this.studioDoc.getMap(editorId);
//     }

//     undo(scope: string) {
//         const undoManager = this.undoManagers.get(scope);
//         if (undoManager) undoManager.undo();
//     }

//     redo(scope: string) {
//         const undoManager = this.undoManagers.get(scope);
//         if (undoManager) undoManager.redo();
//     }
// }

// const StudioContext = createContext<Studio | null>(null);

// export function useStudio() {
//     const studio = useContext(StudioContext);
//     if (!studio) throw new Error('Studio not found');
//     return studio;
// }

// export function useTypeEditor(typeId: string) {
//     const studio = useStudio();
//     const [type, setType] = useState<Y.Map<any>>(studio.getMap(`type-${typeId}`));

//     useEffect(() => {
//         const typeMap = studio.getMap(`type-${typeId}`);
//         const updateHandler = () => setType(new Y.Map(typeMap.toJSON()));

//         typeMap.observe(updateHandler);
//         return () => typeMap.unobserve(updateHandler);
//     }, [studio, typeId]);

//     return {
//         type,
//         undo: () => studio.undo(`type-${typeId}`),
//         redo: () => studio.redo(`type-${typeId}`),
//     };
// }

// export function useDesignEditor(designId: string) {
//     const studio = useStudio();
//     const [design, setDesign] = useState<Y.Map<any>>(studio.getMap(`design-${designId}`));

//     useEffect(() => {
//         const designMap = studio.getMap(`design-${designId}`);
//         const updateHandler = () => setDesign(new Y.Map(designMap.toJSON()));

//         designMap.observe(updateHandler);
//         return () => designMap.unobserve(updateHandler);
//     }, [studio, designId]);

//     return {
//         design,
//         undo: () => studio.undo(`design-${designId}`),
//         redo: () => studio.redo(`design-${designId}`),
//     };
// }
