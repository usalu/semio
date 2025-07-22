// #region Header

// Test.tsx

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
import { FC, useState } from "react"
import * as Y from 'yjs';
import { UndoManager } from 'yjs';
import { IndexeddbPersistence } from 'y-indexeddb';

import { Kit, Design, Piece, Connection } from '@semio/js';

// store.tsx
interface DesignEditorSelection {
    selectedPieceIds: string[];
    selectedConnections: {
        connectingPieceId: string;
        connectedPieceId: string;
    }[];
}

interface DesignEditorState {
    selection: DesignEditorSelection;
}

class DesignEditorStore {
    private studio: Studio;
    private yKit: Y.Map<any>;
    private yDesign: Y.Map<any>;
    private undoManager: UndoManager;
    private state: DesignEditorState;

    constructor(studio: Studio, yKit: Y.Map<any>, yDesign: Y.Map<any>, undoManager: UndoManager) {
        this.studio = studio;
        this.yKit = yKit;
        this.yDesign = yDesign;
        this.undoManager = undoManager;
        this.state = {
            selection: {
                selectedPieceIds: [],
                selectedConnections: []
            }
        };
    }

    getState(): DesignEditorState {
        return this.state;
    }

    setState(state: DesignEditorState): void {
        this.state = state;
    }

    getDesignId(): [string, string, string] {
        return [this.yDesign.get('name'), this.yDesign.get('variant'), this.yDesign.get('view')];
    }

    getKitId(): string {
        return this.yKit.get('uri');
    }

    transact(operations: () => void) {
        this.studio.transact(() => {
            operations();
        }, { trackedOrigins: new Set([this.userId]) });
    }
}

class StudioStore {
    private userId: string;
    private yDoc: Y.Doc;
    private undoManager: UndoManager;
    private designEditors: Map<string, DesignEditor>;
    private indexeddbProvider: IndexeddbPersistence;

    constructor(userId: string) {
        this.userId = userId;
        this.yDoc = new Y.Doc();
        this.undoManager = new UndoManager(this.yDoc, { trackedOrigins: new Set([this.userId]) });
        this.designEditors = new Map();
        this.indexeddbProvider = new IndexeddbPersistence(userId, this.yDoc);
        this.indexeddbProvider.whenSynced.then(() => {
            console.log(`Local changes are synchronized for user (${this.userId}) with client (${this.yDoc.clientID})`);
        });
    }

    // Basic cruds that can be combined in transactions
    createKit(kit: Kit): void {
        const yKit = Y.Map<any>
        // …
        // this.yDoc.set(…)
    }
    createDesign(design: Design): void {
        // this.yDoc.set(
    }
    // …
    createPiece(kitUri: string, designName: string, designVariant: string, view: string, piece: Piece): void {
        // …
    }

    // Editor
    createDesignEditor(kitUri: string, designName: string, designVariant: string, view: string): string {
        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit ${kitUri} not found`);
        const yDesign = yKit.get('designs').get(designName)?.get(designVariant)?.get(view);
        if (!yDesign) throw new Error(`Design ${designName} not found in kit ${kitUri}`);
        const id = uuidv4();
        const undoManager = new UndoManager(yDesign, { trackedOrigins: new Set([id]) });
        const designEditor = new DesignEditor(this, yKit, yDesign, undoManager);
        this.designEditors.set(id, designEditor);
        return id;
    }
    // …

}

// commands/**
const CreatePieceArray: FC = () => {
    //transaction, createPiece, createConnection
    const [count, setCount] = useState(10);
    return (
        <Slider max={100} min={0} step={1} onChange={setCount(value)} />
        <button onClick={() => {
            transaction(() => {
                createPiece();
                createConnection();
            });
        }}>Create Piece</button>
    )
}

// DesignEditor.tsx
const DesignEditor: FC = () => {
    // design, undo, redo, updateSelection

    return (
        <div>
            <h1>{design.name}</h1>
            <button onClick={undo}>Undo</button>
            <button onClick={redo}>Redo</button>
            <CreatePieceArray />
            <Diagram design={design} onSelect={updateSelection} />
            <Model design={design onSelect={updateSelection}}
        </div>
    );
};

// Sketchpad.tsx
const Sketchpad: FC<SketchpadProps> = () => {
    // undo, redo, createKit, createDesign, designs
    const [viewUrl, setViewUrl] = useState('/');
    return (
        <Studio>
            <button onClick={undo}>Undo</button>
            <button onClick={redo}>Redo</button>
            <button onClick={() => {
                const id = createKit()
                setViewUrl(`/kit/${id}`)
            }}>Create Kit</button>
            <button onClick={() => {
                const id = createDesign()
                setViewUrl(`/design/${id}`)
            }}>Create Design</button>
            <View url={viewUrl} />
        </Studio>
    );
};
