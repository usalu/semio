import { v4 as uuidv4 } from 'uuid';
import * as Y from 'yjs';
import React, { createContext, useContext, useEffect, useState } from 'react';
import { UndoManager } from 'yjs';
import { IndexeddbPersistence } from 'y-indexeddb';

import { Generator } from '@semio/js/lib/utils';
import { Kit } from '@semio/js/semio';


export interface DesignEditorState {
    selection: {
        pieceUuids: string[];
        connectionUuids: string[];
    };
}

type YQuality = {
    name: string;

}

type YType = {
    name: string;
}

type YDesign = {
    name: string;
}

type YKit = {
    name: string;
    types: Y.Map<Y.Map<YType>>;
    designs: Y.Map<Y.Map<Y.Map<YDesign>>>;
    qualities: Y.Map<YQuality>;
};

type YStudio = {
    kits: Y.Map<string, YKit>;
    designEditorStates: Y.Map<DesignEditorState>;
}

class Studio {
    private userId: string;
    private studioDoc: Y.Doc;
    private undoManagers: Map<string, Map<string, UndoManager>>;
    private indexeddbProvider: IndexeddbPersistence;

    constructor(
        userId: string = Generator.randomId()
    ) {
        this.userId = userId;
        this.studioDoc = new Y.Doc();
        this.undoManagers = new Map();
        this.indexeddbProvider = new IndexeddbPersistence(userId, this.studioDoc);
        this.indexeddbProvider.whenSynced.then(() => {
            console.log(`Local changes are synchronized for user (${this.userId}) with client (${this.studioDoc.clientID})`);
        });
    }

    private createYKit(kit: Kit): Y.Map<any> {
        const yKit = new Y.Map<any>();
        yKit.set('uri', kit.uri);
        yKit.set('name', kit.name);
        yKit.set('description', kit.description || '');
        yKit.set('icon', kit.icon || '');
        yKit.set('image', kit.image || '');
        yKit.set('preview', kit.preview || '');
        yKit.set('version', kit.version || '');
        yKit.set('remote', kit.remote || '');
        yKit.set('homepage', kit.homepage || '');
        yKit.set('license', kit.license || []);
        yKit.set('designs', new Y.Map());
        yKit.set('types', new Y.Map());
        yKit.set('qualities', new Y.Map());
        this.studioDoc.getMap('kits').set(kit.uri, yKit);
        return yKit
    }

    private updateYKit(kit: Kit): Y.Map<any> | null {
        const yKit = this.getYKit(kit.uri);
        if (!yKit) return null;
        if (kit.name !== "") yKit.set('name', kit.name);
        if (kit.description !== undefined && kit.description !== "") yKit.set('description', kit.description);
        if (kit.icon !== undefined && kit.icon !== "") yKit.set('icon', kit.icon);
        if (kit.image !== undefined && kit.image !== "") yKit.set('image', kit.image);
        if (kit.preview !== undefined && kit.preview !== "") yKit.set('preview', kit.preview);
        if (kit.version !== undefined && kit.version !== "") yKit.set('version', kit.version);
        if (kit.remote !== undefined && kit.remote !== "") yKit.set('remote', kit.remote);
        if (kit.homepage !== undefined && kit.homepage !== "") yKit.set('homepage', kit.homepage);
        if (kit.license !== undefined && kit.license !== "") yKit.set('license', kit.license);
        // TODO: Update designs and types
        // if (kit.designs !== undefined && kit.designs.length > 0) {
        //     const designs = yKit.get('designs') as Y.Map<any>;
        //     kit.designs.forEach(design => {
        //         if (designs.has(design.name)) {
        //             const sameDesigns = designs.get(design.name) as Y.Map<any>;
        //             if (sameDesigns.has(design.variant || '')) {
        //                 updateYDesign(design);
        //             }
        //             else {
        //                 createYDesign(design);
        //             }
        //         }
        //         else {
        //             createYDesign(d)
        //         }
        //     });
        // }
        if (kit.qualities)
    }

    getKit(uri: string): Kit | null {
        const yKit = this.getYKit(uri);
        if (!yKit) return null;
        return {
            name: yKit.get('name'),
            description: yKit.get('description'),
            icon: yKit.get('icon'),
            image: yKit.get('image'),
            preview: yKit.get('preview'),
            version: yKit.get('version'),
            remote: yKit.get('remote'),
            homepage: yKit.get('homepage'),
            license: yKit.get('license'),
            uri: uri,
            designs: Array.from(yKit.get('designs').keys()),
            types: Array.from(yKit.get('types').keys())
        };
    }

    private getYKit(uri: string): Y.Map<any> | undefined {
        return this.studioDoc.getMap('kits').get(uri) as Y.Map<any> | undefined;
    }

    createKit(kit: Kit): KitNode {


        return {
            uuid,
            name: yKit.get('name') || '',
            designUuids: []
        };
    }

    // Add a design to a kit
    addDesign(kitUuid: string, parentId: string, designUuid: string): KitNode | null {
        const parentKit = this.getYKit(kitUuid, parentId);
        if (!parentKit) return null;

        // Make sure we have an undo manager first
        if (!this.undoManagers.has(kitUuid)) {
            this.getYKit(kitUuid);
        }

        const undoManager = this.undoManagers.get(kitUuid);

        // Create a transaction to ensure this is tracked as one operation
        this.studioDoc.transact(() => {
            const designKit = this.createYKit(kitUuid, designUuid);
            const designs = parentKit.get('designs') as Y.Map<string>;
            designs.set(designUuid, designUuid);
        }, this.userId);

        console.log(`Added design ${designUuid} to ${parentId} in ${kitUuid}, checking canUndo: ${undoManager?.canUndo()}`);

        return this.getKit(kitUuid, designUuid);
    }

    // Delete a design kit from its parent
    deleteDesign(kitUuid: string, parentId: string, designUuid: string): boolean {
        const parentKit = this.getYKit(kitUuid, parentId);
        if (!parentKit) return false;

        // Make sure we have an undo manager first
        if (!this.undoManagers.has(kitUuid)) {
            this.getYKit(kitUuid);
        }

        const undoManager = this.undoManagers.get(kitUuid);
        let success = false;

        // Create a transaction to ensure this is tracked as one operation
        this.studioDoc.transact(() => {
            // Remove the design from the parent's designs map
            const designs = parentKit.get('designs') as Y.Map<string>;
            if (designs.has(designUuid)) {
                designs.delete(designUuid);
                success = true;
            }

            // Delete the design kit from the kit map
            const kitsMap = this.studioDoc.getMap(kitUuid);
            if (kitsMap.has(designUuid)) {
                kitsMap.delete(designUuid);
            }
        }, this.userId);

        console.log(`Deleted design ${designUuid} from ${parentId} in ${kitUuid}, success: ${success}, checking canUndo: ${undoManager?.canUndo()}`);

        return success;
    }

    // Observe changes to the internal YDoc
    observeKit(kitUuid: string, callback: () => void): () => void {
        const kitMap = this.studioDoc.getMap(kitUuid);
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
     * Explicitly initializes a kit with a root kit and optional initial name
     * @param kitUuid The ID of the kit to initialize
     * @param initialName Optional initial name for the root kit
     * @returns The created root kit
     */
    initializeKit(kitUuid: string, initialName: string = 'Root'): KitNode {
        // Make sure we have an undo manager first
        if (!this.undoManagers.has(kitUuid)) {
            // This will create the undo manager
            this.getYKit(kitUuid);
        }

        const undoManager = this.undoManagers.get(kitUuid);
        const kitsMap = this.studioDoc.getMap(kitUuid);

        // Create a fresh root kit within a transaction to enable undo
        this.studioDoc.transact(() => {
            const rootKit = new Y.Map<any>();
            rootKit.set('name', initialName);
            rootKit.set('designs', new Y.Map());
            kitsMap.set('root', rootKit);
        }, this.userId);

        // Log the status of the undo manager
        console.log(`Initialized kit ${kitUuid}, checking canUndo: ${undoManager?.canUndo()}`);

        return this.getKit(kitUuid, 'root') as KitNode;
    }

    /**
     * Checks if a kit exists and has a root kit
     * @param kitUuid The ID of the kit to check
     * @returns True if the kit exists and has a root kit
     */
    hasKit(kitUuid: string): boolean {
        const kitsMap = this.studioDoc.getMap(kitUuid);
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
            this.indexeddbProvider = new IndexeddbPersistence(this.userId, this.studioDoc);

            console.log(`Studio data for ${this.userId} has been cleaned`);
            return this.indexeddbProvider.whenSynced;
        } catch (error) {
            console.error('Error cleaning studio:', error);
            throw error;
        }
    }

    // Focus method to ensure undo/redo capability is activated
    triggerUndoRedoCapability(kitUuid: string): void {
        // Get or create the undo manager
        if (!this.undoManagers.has(kitUuid)) {
            this.getYKit(kitUuid);
        }

        // Force a change that can be undone to test the undo functionality
        // This will be immediately discarded, but ensures the UndoManager is working
        const undoManager = this.undoManagers.get(kitUuid);
        if (undoManager) {
            // Create a temporary marker in the document that we'll immediately remove
            // This creates an undoable action
            const kitsMap = this.studioDoc.getMap(kitUuid);
            const tempKey = `_temp_${Date.now()}`;

            this.studioDoc.transact(() => {
                kitsMap.set(tempKey, "test");
            }, this.userId);

            this.studioDoc.transact(() => {
                kitsMap.delete(tempKey);
            }, this.userId);

            console.log(`Triggered undo capability for ${kitUuid}, canUndo: ${undoManager.canUndo()}`);
        }
    }

    private createYType(type: Type): Y.Map<any> {
        const yType = new Y.Map<any>();
        yType.set('name', type.name);
        yType.set('description', type.description || '');
        yType.set('icon', type.icon || '');
        yType.set('image', type.image || '');
        yType.set('variant', type.variant || '');
        yType.set('unit', type.unit || '');
        yType.set('ports', new Y.Map());
        yType.set('qualities', new Y.Map());
        yType.set('representations', new Y.Map());
        return yType;
    }

    createType(kitUri: string, type: Type): Type {
        const yKit = this.getYKit(kitUri);
        if (!yKit) throw new Error(`Kit ${kitUri} not found`);

        const types = yKit.get('types') as Y.Map<any>;
        const yType = this.createYType(type);
        types.set(type.name, yType);

        return this.getType(kitUri, type.name);
    }

    getType(kitUri: string, typeName: string): Type | null {
        const yKit = this.getYKit(kitUri);
        if (!yKit) return null;

        const types = yKit.get('types') as Y.Map<any>;
        const yType = types.get(typeName) as Y.Map<any> | undefined;
        if (!yType) return null;

        return {
            name: yType.get('name'),
            description: yType.get('description'),
            icon: yType.get('icon'),
            image: yType.get('image'),
            variant: yType.get('variant'),
            unit: yType.get('unit'),
            ports: Array.from(yType.get('ports').keys()),
            qualities: Array.from(yType.get('qualities').keys()),
            representations: Array.from(yType.get('representations').keys())
        };
    }

    updateType(kitUri: string, type: Type): Type | null {
        const yKit = this.getYKit(kitUri);
        if (!yKit) return null;

        const types = yKit.get('types') as Y.Map<any>;
        const yType = types.get(type.name) as Y.Map<any> | undefined;
        if (!yType) return null;

        if (type.description !== undefined) yType.set('description', type.description);
        if (type.icon !== undefined) yType.set('icon', type.icon);
        if (type.image !== undefined) yType.set('image', type.image);
        if (type.variant !== undefined) yType.set('variant', type.variant);
        if (type.unit !== undefined) yType.set('unit', type.unit);
        // TODO: Update ports, qualities, representations

        return this.getType(kitUri, type.name);
    }

    deleteType(kitUri: string, typeName: string): boolean {
        const yKit = this.getYKit(kitUri);
        if (!yKit) return false;

        const types = yKit.get('types') as Y.Map<any>;
        return types.delete(typeName);
    }

    private createYDesign(design: Design): Y.Map<any> {
        const yDesign = new Y.Map<any>();
        yDesign.set('name', design.name);
        yDesign.set('description', design.description || '');
        yDesign.set('icon', design.icon || '');
        yDesign.set('image', design.image || '');
        yDesign.set('variant', design.variant || '');
        yDesign.set('view', design.view || '');
        yDesign.set('unit', design.unit || '');
        yDesign.set('pieces', new Y.Map());
        yDesign.set('connections', new Y.Map());
        yDesign.set('qualities', new Y.Map());
        return yDesign;
    }

    createDesign(kitUri: string, design: Design): Design {
        const yKit = this.getYKit(kitUri);
        if (!yKit) throw new Error(`Kit ${kitUri} not found`);

        const designs = yKit.get('designs') as Y.Map<any>;
        const yDesign = this.createYDesign(design);
        designs.set(design.name, yDesign);

        return this.getDesign(kitUri, design.name);
    }

    getDesign(kitUri: string, designName: string): Design | null {
        const yKit = this.getYKit(kitUri);
        if (!yKit) return null;

        const designs = yKit.get('designs') as Y.Map<any>;
        const yDesign = designs.get(designName) as Y.Map<any> | undefined;
        if (!yDesign) return null;

        return {
            name: yDesign.get('name'),
            description: yDesign.get('description'),
            icon: yDesign.get('icon'),
            image: yDesign.get('image'),
            variant: yDesign.get('variant'),
            view: yDesign.get('view'),
            unit: yDesign.get('unit'),
            pieces: Array.from(yDesign.get('pieces').keys()),
            connections: Array.from(yDesign.get('connections').keys()),
            qualities: Array.from(yDesign.get('qualities').keys())
        };
    }

    updateDesign(kitUri: string, design: Design): Design | null {
        const yKit = this.getYKit(kitUri);
        if (!yKit) return null;

        const designs = yKit.get('designs') as Y.Map<any>;
        const yDesign = designs.get(design.name) as Y.Map<any> | undefined;
        if (!yDesign) return null;

        if (design.description !== undefined) yDesign.set('description', design.description);
        if (design.icon !== undefined) yDesign.set('icon', design.icon);
        if (design.image !== undefined) yDesign.set('image', design.image);
        if (design.variant !== undefined) yDesign.set('variant', design.variant);
        if (design.view !== undefined) yDesign.set('view', design.view);
        if (design.unit !== undefined) yDesign.set('unit', design.unit);
        // TODO: Update pieces, connections, qualities

        return this.getDesign(kitUri, design.name);
    }

    deleteDesign(kitUri: string, designName: string): boolean {
        const yKit = this.getYKit(kitUri);
        if (!yKit) return false;

        const designs = yKit.get('designs') as Y.Map<any>;
        return designs.delete(designName);
    }

    private createYPiece(piece: Piece): Y.Map<any> {
        const yPiece = new Y.Map<any>();
        yPiece.set('id_', piece.id_ || '');
        yPiece.set('description', piece.description || '');
        yPiece.set('type', piece.type);
        yPiece.set('center', piece.center ? new Y.Map(piece.center) : null);
        yPiece.set('plane', piece.plane ? new Y.Map(piece.plane) : null);
        return yPiece;
    }

    createPiece(kitUri: string, designName: string, piece: Piece): Piece {
        const yKit = this.getYKit(kitUri);
        if (!yKit) throw new Error(`Kit ${kitUri} not found`);

        const designs = yKit.get('designs') as Y.Map<any>;
        const yDesign = designs.get(designName) as Y.Map<any> | undefined;
        if (!yDesign) throw new Error(`Design ${designName} not found in kit ${kitUri}`);

        const pieces = yDesign.get('pieces') as Y.Map<any>;
        const yPiece = this.createYPiece(piece);
        pieces.set(piece.id_ || uuidv4(), yPiece);

        return this.getPiece(kitUri, designName, piece.id_!);
    }

    getPiece(kitUri: string, designName: string, pieceId: string): Piece | null {
        const yKit = this.getYKit(kitUri);
        if (!yKit) return null;

        const designs = yKit.get('designs') as Y.Map<any>;
        const yDesign = designs.get(designName) as Y.Map<any> | undefined;
        if (!yDesign) return null;

        const pieces = yDesign.get('pieces') as Y.Map<any>;
        const yPiece = pieces.get(pieceId) as Y.Map<any> | undefined;
        if (!yPiece) return null;

        return {
            id_: yPiece.get('id_'),
            description: yPiece.get('description'),
            type: yPiece.get('type'),
            center: yPiece.get('center') ? yPiece.get('center').toJSON() : null,
            plane: yPiece.get('plane') ? yPiece.get('plane').toJSON() : null
        };
    }

    updatePiece(kitUri: string, designName: string, piece: Piece): Piece | null {
        const yKit = this.getYKit(kitUri);
        if (!yKit) return null;

        const designs = yKit.get('designs') as Y.Map<any>;
        const yDesign = designs.get(designName) as Y.Map<any> | undefined;
        if (!yDesign) return null;

        const pieces = yDesign.get('pieces') as Y.Map<any>;
        const yPiece = pieces.get(piece.id_!) as Y.Map<any> | undefined;
        if (!yPiece) return null;

        if (piece.description !== undefined) yPiece.set('description', piece.description);
        if (piece.type !== undefined) yPiece.set('type', piece.type);
        if (piece.center !== undefined) yPiece.set('center', piece.center ? new Y.Map(piece.center) : null);
        if (piece.plane !== undefined) yPiece.set('plane', piece.plane ? new Y.Map(piece.plane) : null);

        return this.getPiece(kitUri, designName, piece.id_!);
    }

    deletePiece(kitUri: string, designName: string, pieceId: string): boolean {
        const yKit = this.getYKit(kitUri);
        if (!yKit) return false;

        const designs = yKit.get('designs') as Y.Map<any>;
        const yDesign = designs.get(designName) as Y.Map<any> | undefined;
        if (!yDesign) return false;

        const pieces = yDesign.get('pieces') as Y.Map<any>;
        return pieces.delete(pieceId);
    }

    private createYConnection(connection: Connection): Y.Map<any> {
        const yConnection = new Y.Map<any>();
        yConnection.set('description', connection.description || '');
        yConnection.set('connected', new Y.Map(connection.connected));
        yConnection.set('connecting', new Y.Map(connection.connecting));
        yConnection.set('gap', connection.gap || 0);
        yConnection.set('rotation', connection.rotation || 0);
        yConnection.set('shift', connection.shift || 0);
        yConnection.set('tilt', connection.tilt || 0);
        yConnection.set('x', connection.x || 0);
        yConnection.set('y', connection.y || 0);
        return yConnection;
    }

    createConnection(kitUri: string, designName: string, connection: Connection): Connection {
        const yKit = this.getYKit(kitUri);
        if (!yKit) throw new Error(`Kit ${kitUri} not found`);

        const designs = yKit.get('designs') as Y.Map<any>;
        const yDesign = designs.get(designName) as Y.Map<any> | undefined;
        if (!yDesign) throw new Error(`Design ${designName} not found in kit ${kitUri}`);

        const connections = yDesign.get('connections') as Y.Map<any>;
        const yConnection = this.createYConnection(connection);
        const connectionId = uuidv4();
        connections.set(connectionId, yConnection);

        return this.getConnection(kitUri, designName, connectionId);
    }

    getConnection(kitUri: string, designName: string, connectionId: string): Connection | null {
        const yKit = this.getYKit(kitUri);
        if (!yKit) return null;

        const designs = yKit.get('designs') as Y.Map<any>;
        const yDesign = designs.get(designName) as Y.Map<any> | undefined;
        if (!yDesign) return null;

        const connections = yDesign.get('connections') as Y.Map<any>;
        const yConnection = connections.get(connectionId) as Y.Map<any> | undefined;
        if (!yConnection) return null;

        return {
            description: yConnection.get('description'),
            connected: yConnection.get('connected').toJSON(),
            connecting: yConnection.get('connecting').toJSON(),
            gap: yConnection.get('gap'),
            rotation: yConnection.get('rotation'),
            shift: yConnection.get('shift'),
            tilt: yConnection.get('tilt'),
            x: yConnection.get('x'),
            y: yConnection.get('y')
        };
    }

    updateConnection(kitUri: string, designName: string, connectionId: string, connection: Partial<Connection>): Connection | null {
        const yKit = this.getYKit(kitUri);
        if (!yKit) return null;

        const designs = yKit.get('designs') as Y.Map<any>;
        const yDesign = designs.get(designName) as Y.Map<any> | undefined;
        if (!yDesign) return null;

        const connections = yDesign.get('connections') as Y.Map<any>;
        const yConnection = connections.get(connectionId) as Y.Map<any> | undefined;
        if (!yConnection) return null;

        if (connection.description !== undefined) yConnection.set('description', connection.description);
        if (connection.connected !== undefined) yConnection.set('connected', new Y.Map(connection.connected));
        if (connection.connecting !== undefined) yConnection.set('connecting', new Y.Map(connection.connecting));
        if (connection.gap !== undefined) yConnection.set('gap', connection.gap);
        if (connection.rotation !== undefined) yConnection.set('rotation', connection.rotation);
        if (connection.shift !== undefined) yConnection.set('shift', connection.shift);
        if (connection.tilt !== undefined) yConnection.set('tilt', connection.tilt);
        if (connection.x !== undefined) yConnection.set('x', connection.x);
        if (connection.y !== undefined) yConnection.set('y', connection.y);

        return this.getConnection(kitUri, designName, connectionId);
    }

    deleteConnection(kitUri: string, designName: string, connectionId: string): boolean {
        const yKit = this.getYKit(kitUri);
        if (!yKit) return false;

        const designs = yKit.get('designs') as Y.Map<any>;
        const yDesign = designs.get(designName) as Y.Map<any> | undefined;
        if (!yDesign) return false;

        const connections = yDesign.get('connections') as Y.Map<any>;
        return connections.delete(connectionId);
    }

    private createYRepresentation(representation: Representation): Y.Map<any> {
        const yRepresentation = new Y.Map<any>();
        yRepresentation.set('url', representation.url);
        yRepresentation.set('description', representation.description || '');
        yRepresentation.set('mime', representation.mime);
        yRepresentation.set('tags', representation.tags ? new Y.Array(representation.tags) : new Y.Array());
        yRepresentation.set('qualities', representation.qualities ? new Y.Array(representation.qualities.map(q => q.name)) : new Y.Array());
        return yRepresentation;
    }

    createRepresentation(kitUri: string, typeName: string, representation: Representation): Representation {
        const yKit = this.getYKit(kitUri);
        if (!yKit) throw new Error(`Kit ${kitUri} not found`);

        const types = yKit.get('types') as Y.Map<any>;
        const yType = types.get(typeName) as Y.Map<any> | undefined;
        if (!yType) throw new Error(`Type ${typeName} not found in kit ${kitUri}`);

        const representations = yType.get('representations') as Y.Map<any>;
        const yRepresentation = this.createYRepresentation(representation);
        representations.set(representation.url, yRepresentation);

        return this.getRepresentation(kitUri, typeName, representation.url);
    }

    getRepresentation(kitUri: string, typeName: string, representationUrl: string): Representation | null {
        const yKit = this.getYKit(kitUri);
        if (!yKit) return null;

        const types = yKit.get('types') as Y.Map<any>;
        const yType = types.get(typeName) as Y.Map<any> | undefined;
        if (!yType) return null;

        const representations = yType.get('representations') as Y.Map<any>;
        const yRepresentation = representations.get(representationUrl) as Y.Map<any> | undefined;
        if (!yRepresentation) return null;

        return {
            url: yRepresentation.get('url'),
            description: yRepresentation.get('description'),
            mime: yRepresentation.get('mime'),
            tags: Array.from(yRepresentation.get('tags')),
            qualities: yRepresentation.get('qualities').map((name: string) => ({ name }))
        };
    }

    updateRepresentation(kitUri: string, typeName: string, representationUrl: string, representation: Partial<Representation>): Representation | null {
        const yKit = this.getYKit(kitUri);
        if (!yKit) return null;

        const types = yKit.get('types') as Y.Map<any>;
        const yType = types.get(typeName) as Y.Map<any> | undefined;
        if (!yType) return null;

        const representations = yType.get('representations') as Y.Map<any>;
        const yRepresentation = representations.get(representationUrl) as Y.Map<any> | undefined;
        if (!yRepresentation) return null;

        if (representation.description !== undefined) yRepresentation.set('description', representation.description);
        if (representation.mime !== undefined) yRepresentation.set('mime', representation.mime);
        if (representation.tags !== undefined) yRepresentation.set('tags', new Y.Array(representation.tags));
        if (representation.qualities !== undefined) yRepresentation.set('qualities', new Y.Array(representation.qualities.map(q => q.name)));

        return this.getRepresentation(kitUri, typeName, representationUrl);
    }

    deleteRepresentation(kitUri: string, typeName: string, representationUrl: string): boolean {
        const yKit = this.getYKit(kitUri);
        if (!yKit) return false;

        const types = yKit.get('types') as Y.Map<any>;
        const yType = types.get(typeName) as Y.Map<any> | undefined;
        if (!yType) return false;

        const representations = yType.get('representations') as Y.Map<any>;
        return representations.delete(representationUrl);
    }

    private createYPort(port: Port): Y.Map<any> {
        const yPort = new Y.Map<any>();
        yPort.set('id_', port.id_ || uuidv4());
        yPort.set('description', port.description || '');
        yPort.set('direction', new Y.Map(port.direction));
        yPort.set('point', new Y.Map(port.point));
        yPort.set('t', port.t || 0);
        yPort.set('qualities', port.qualities ? new Y.Array(port.qualities.map(q => q.name)) : new Y.Array());
        return yPort;
    }

    createPort(kitUri: string, typeName: string, port: Port): Port {
        const yKit = this.getYKit(kitUri);
        if (!yKit) throw new Error(`Kit ${kitUri} not found`);

        const types = yKit.get('types') as Y.Map<any>;
        const yType = types.get(typeName) as Y.Map<any> | undefined;
        if (!yType) throw new Error(`Type ${typeName} not found in kit ${kitUri}`);

        const ports = yType.get('ports') as Y.Map<any>;
        const yPort = this.createYPort(port);
        ports.set(yPort.get('id_'), yPort);

        return this.getPort(kitUri, typeName, yPort.get('id_'));
    }

    getPort(kitUri: string, typeName: string, portId: string): Port | null {
        const yKit = this.getYKit(kitUri);
        if (!yKit) return null;

        const types = yKit.get('types') as Y.Map<any>;
        const yType = types.get(typeName) as Y.Map<any> | undefined;
        if (!yType) return null;

        const ports = yType.get('ports') as Y.Map<any>;
        const yPort = ports.get(portId) as Y.Map<any> | undefined;
        if (!yPort) return null;

        return {
            id_: yPort.get('id_'),
            description: yPort.get('description'),
            direction: yPort.get('direction').toJSON(),
            point: yPort.get('point').toJSON(),
            t: yPort.get('t'),
            qualities: yPort.get('qualities').map((name: string) => ({ name }))
        };
    }

    updatePort(kitUri: string, typeName: string, portId: string, port: Partial<Port>): Port | null {
        const yKit = this.getYKit(kitUri);
        if (!yKit) return null;

        const types = yKit.get('types') as Y.Map<any>;
        const yType = types.get(typeName) as Y.Map<any> | undefined;
        if (!yType) return null;

        const ports = yType.get('ports') as Y.Map<any>;
        const yPort = ports.get(portId) as Y.Map<any> | undefined;
        if (!yPort) return null;

        if (port.description !== undefined) yPort.set('description', port.description);
        if (port.direction !== undefined) yPort.set('direction', new Y.Map(port.direction));
        if (port.point !== undefined) yPort.set('point', new Y.Map(port.point));
        if (port.t !== undefined) yPort.set('t', port.t);
        if (port.qualities !== undefined) yPort.set('qualities', new Y.Array(port.qualities.map(q => q.name)));

        return this.getPort(kitUri, typeName, portId);
    }

    deletePort(kitUri: string, typeName: string, portId: string): boolean {
        const yKit = this.getYKit(kitUri);
        if (!yKit) return false;

        const types = yKit.get('types') as Y.Map<any>;
        const yType = types.get(typeName) as Y.Map<any> | undefined;
        if (!yType) return false;

        const ports = yType.get('ports') as Y.Map<any>;
        return ports.delete(portId);
    }
}

// Create a singleton instance of the Studio
const studioSingleton = new Studio();

// Create context with proper typing
const StudioContext = createContext<Studio | null>(null);

export const StudioProvider: React.FC<{ children: React.ReactKit }> = ({ children }) => {
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

export function useKit(uri: string) {
    const studio = useStudio();
    const [kits, setKits] = useState<Record<string, Kit>>({});
    const [canUndo, setCanUndo] = useState<boolean>(false);
    const [canRedo, setCanRedo] = useState<boolean>(false);
    const [hasInitialized, setHasInitialized] = useState<boolean>(studio.hasKit(uri));

    useEffect(() => {
        const loadKit = (uri: string): void => {
            const kit = studio.getKit(uri);
            if (!kit) return;

            setKits(prev => ({
                ...prev,
                [uri]: kit
            }));

        };
        setHasInitialized(studio.hasKit(uri));

        // Update handler for kit changes
        const updateHandler = () => {
            loadKit('root');
            setHasInitialized(studio.hasKit(uri));
            // Check undo/redo state on every kit change
            updateUndoRedoState();
        };

        // Update undo/redo state
        const updateUndoRedoState = () => {
            const canUndoNow = studio.canUndo(uri);
            const canRedoNow = studio.canRedo(uri);

            if (canUndoNow !== canUndo) {
                setCanUndo(canUndoNow);
                console.log(`Updated canUndo to ${canUndoNow} for ${uri}`);
            }

            if (canRedoNow !== canRedo) {
                setCanRedo(canRedoNow);
                console.log(`Updated canRedo to ${canRedoNow} for ${uri}`);
            }
        };

        updateUndoRedoState();
        const unobserveKit = studio.observeKit(uri, updateHandler);

        const unsubscribeUndoChanges = studio.subscribeToUndoChanges(uri, updateUndoRedoState);

        return () => {
            unobserveKit();
            unsubscribeUndoChanges();
        };
    }, [studio, uri, canUndo, canRedo]);

    return {
        kits,
        getKit: (uri: string) => kits[uri] || null,
        hasInitialized,
        undo: () => studio.undo(uri),
        redo: () => studio.redo(uri),
        canUndo,
        canRedo,
    };
}