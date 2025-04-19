import { v4 as uuidv4 } from 'uuid';
import * as Y from 'yjs';
import React, { createContext, useContext, useEffect, useState, useMemo, FC } from 'react';
import { UndoManager } from 'yjs';
import { IndexeddbPersistence } from 'y-indexeddb';

import { Generator } from '@semio/js/lib/utils';
import { Kit, Port, Representation, Piece, Connection, Type, Design, Plane, DiagramPoint, Point, Vector, Quality, Author, Side } from '@semio/js/semio';

import { default as metabolism } from '@semio/assets/semio/kit_metabolism.json';
import { default as nakaginCapsuleTower } from '@semio/assets/semio/design_nakagin-capsule-tower_flat.json';


// type YType = {
//     name: string;
//     variant: string;
//     description: string;
//     icon: string;
//     image: string;
//     unit: string;
//     ports: Y.Map<Port>;
//     qualities: Y.Map<YQuality>;
//     representations: Y.Map<YRepresentation>;
// }

// type YDesign = {
//     name: string;
//     variant: string;
//     view: string;
//     description: string;
//     icon: string;
//     image: string;
//     unit: string;
//     pieces: Y.Map<YPiece>;
//     connections: Y.Map<YConnection>;
//     qualities: Y.Map<YQuality>;
// }

// type YKit = {
//     uri: string;
//     name: string;
//     description: string;
//     icon: string;
//     image: string;
//     preview: string;
//     version: string;
//     remote: string;
//     homepage: string;
//     license: string[];
//     types: Y.Map<Y.Map<YType>>;
//     designs: Y.Map<Y.Map<Y.Map<YDesign>>>;
//     qualities: Y.Map<YQuality>;
// };

// type YStudio = {
//     kits: Y.Map<YKit>;
//     designEditorStates: Y.Map<DesignEditorState>;
// }

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
    private id: string;
    private yDoc: Y.Doc;
    private yKit: Y.Map<any>;
    private yDesign: Y.Map<any>;
    private undoManager: UndoManager;
    private state: DesignEditorState;

    constructor(id: string, yDoc: Y.Doc, yKit: Y.Map<any>, yDesign: Y.Map<any>, undoManager: UndoManager) {
        this.id = id;
        this.yDoc = yDoc;
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

    undo(): void {
        this.undoManager.undo();
    }

    redo(): void {
        this.undoManager.redo();
    }

    transact(operations: () => void): void {
        this.yDoc.transact(operations, { trackedOrigins: new Set([this.id]) });
    }
}

class StudioStore {
    private userId: string;
    private yDoc: Y.Doc;
    private undoManager: UndoManager;
    private designEditorStores: Map<string, DesignEditorStore>;
    // private indexeddbProvider: IndexeddbPersistence;

    constructor(userId: string) {
        this.userId = userId;
        this.yDoc = new Y.Doc();
        this.undoManager = new UndoManager(this.yDoc, { trackedOrigins: new Set([this.userId]) });
        this.designEditorStores = new Map();
        // this.indexeddbProvider = new IndexeddbPersistence(userId, this.yDoc);
        // this.indexeddbProvider.whenSynced.then(() => {
        //     console.log(`Local changes are synchronized for user (${this.userId}) with client (${this.yDoc.clientID})`);
        // });
    }

    private createQuality(quality: Quality): Y.Map<any> {
        const yQuality = new Y.Map<any>();
        yQuality.set('name', quality.name);
        yQuality.set('value', quality.value || '');
        yQuality.set('unit', quality.unit || '');
        yQuality.set('definition', quality.definition || '');
        return yQuality;
    }

    private getQuality(yMap: Y.Map<any>): Quality | null {
        const name = yMap.get('name');
        const value = yMap.get('value');
        const unit = yMap.get('unit');
        const definition = yMap.get('definition');
        return { name, value, unit, definition };
    }

    private createAuthor(author: Author): Y.Map<any> {
        const yAuthor = new Y.Map<any>();
        yAuthor.set('name', author.name);
        yAuthor.set('email', author.email);
        yAuthor.set('rank', author.rank);
        return yAuthor;
    }

    createKit(kit: Kit): void {
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
        yKit.set('types', new Y.Map<any>());
        yKit.set('designs', new Y.Map<any>());
        yKit.set('qualities', kit.qualities?.map(q => this.createQuality(q)));
        this.yDoc.getMap('kits').set(kit.uri, yKit);
        kit.types?.map(t => this.createType(kit.uri, t));
        kit.designs?.map(d => this.createDesign(kit.uri, d));
    }

    getKit(uri: string): Kit | undefined {
        const yKit = this.yDoc.getMap('kits').get(uri) as Y.Map<any>;
        if (!yKit) return undefined;

        const yTypesMap = yKit.get('types') as Y.Map<Y.Map<any>>;
        const types = yTypesMap ? Array.from(yTypesMap.values()).flatMap(variantMap =>
            Array.from(variantMap.values()).map((t: Y.Map<any>) => this.getType(uri, t.get('name'), t.get('variant')))
        ).filter((t): t is Type => t !== null) : [];

        const yDesignsMap = yKit.get('designs') as Y.Map<Y.Map<Y.Map<any>>>;
        const designs = yDesignsMap ? Array.from(yDesignsMap.values()).flatMap(variantMap =>
            Array.from(variantMap.values()).flatMap(viewMap =>
                Array.from(viewMap.values()).map((d: Y.Map<any>) => this.getDesign(uri, d.get('name'), d.get('variant'), d.get('view')))
            )
        ).filter((d): d is Design => d !== null) : [];

        const yQualitiesArray = yKit.get('qualities') as Y.Array<Y.Map<any>>;
        const qualities = yQualitiesArray ? yQualitiesArray.toArray().map(qMap => this.getQuality(qMap)).filter((q): q is Quality => q !== null) : [];

        return {
            uri: yKit.get('uri'),
            name: yKit.get('name'),
            description: yKit.get('description'),
            icon: yKit.get('icon'),
            image: yKit.get('image'),
            preview: yKit.get('preview'),
            version: yKit.get('version'),
            remote: yKit.get('remote'),
            homepage: yKit.get('homepage'),
            license: yKit.get('license'),
            created: new Date(),
            updated: new Date(),
            designs,
            types,
            qualities
        };
    }

    updateKit(kit: Partial<Kit>): Kit | undefined {
        if (!kit.uri) throw new Error("Kit URI is required for update.");
        const yKit = this.yDoc.getMap('kits').get(kit.uri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit ${kit.uri} not found`);
        if (kit.name !== undefined && kit.name !== "") yKit.set('name', kit.name);
        if (kit.description !== undefined && kit.description !== "") yKit.set('description', kit.description);
        if (kit.icon !== undefined && kit.icon !== "") yKit.set('icon', kit.icon);
        if (kit.image !== undefined && kit.image !== "") yKit.set('image', kit.image);
        if (kit.preview !== undefined && kit.preview !== "") yKit.set('preview', kit.preview);
        if (kit.version !== undefined && kit.version !== "") yKit.set('version', kit.version);
        if (kit.remote !== undefined && kit.remote !== "") yKit.set('remote', kit.remote);
        if (kit.homepage !== undefined && kit.homepage !== "") yKit.set('homepage', kit.homepage);
        if (kit.license !== undefined) yKit.set('license', kit.license);
        return this.getKit(kit.uri);
    }

    deleteKit(uri: string): void {
        this.yDoc.getMap('kits').delete(uri);
    }

    createType(kitUri: string, type: Type): void {
        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit ${kitUri} not found`);

        const types = yKit.get('types') as Y.Map<any>;
        let variantMap = types.get(type.name) as Y.Map<any> | undefined;
        if (!variantMap) {
            variantMap = new Y.Map<any>();
            types.set(type.name, variantMap);
        }
        const yType = new Y.Map<any>();
        yType.set('name', type.name);
        yType.set('variant', type.variant || '');
        yType.set('description', type.description || '');
        yType.set('icon', type.icon || '');
        yType.set('image', type.image || '');
        yType.set('unit', type.unit || '');
        yType.set('ports', new Y.Map());
        yType.set('qualities', type.qualities?.map(q => this.createQuality(q)));
        yType.set('representations', new Y.Map());
        variantMap.set(type.variant || '', yType);
        type.ports?.map(p => { this.createPort(kitUri, type.name, type.variant || '', p) });
        type.representations?.map(r => { this.createRepresentation(kitUri, type.name, type.variant || '', r) });
    }

    getType(kitUri: string, typeName: string, variant: string = ''): Type | null {
        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) return null;

        const types = yKit.get('types') as Y.Map<any>;
        const yType = types.get(typeName)?.get(variant) as Y.Map<any> | undefined;
        if (!yType) return null;

        const yPortsMap = yType.get('ports') as Y.Map<Y.Map<any>>;
        const ports = yPortsMap ? Array.from(yPortsMap.values()).map(pMap => this.getPort(kitUri, typeName, variant, pMap.get('id_'))).filter((p): p is Port => p !== null) : [];

        const yQualitiesArray = yType.get('qualities') as Y.Array<Y.Map<any>>;
        const qualities = yQualitiesArray ? yQualitiesArray.toArray().map(qMap => this.getQuality(qMap)).filter((q): q is Quality => q !== null) : [];

        const yRepresentationsMap = yType.get('representations') as Y.Map<Y.Map<any>>;
        const representations = yRepresentationsMap ? Array.from(yRepresentationsMap.values()).map(rMap => this.getRepresentation(kitUri, typeName, variant, rMap.get('mime'), rMap.get('tags'))).filter((r): r is Representation => r !== null) : [];

        return {
            name: yType.get('name'),
            description: yType.get('description'),
            icon: yType.get('icon'),
            image: yType.get('image'),
            variant: yType.get('variant'),
            unit: yType.get('unit'),
            ports,
            qualities,
            representations,
            updated: new Date(),
            created: new Date(),
            authors: [],
        };
    }

    updateType(kitUri: string, type: Type): Type | null {
        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) return null;

        const types = yKit.get('types');
        const yType = types.get(type.name)?.get(type.variant || '');
        if (!yType) return null;

        if (type.description !== undefined) yType.set('description', type.description);
        if (type.icon !== undefined) yType.set('icon', type.icon);
        if (type.image !== undefined) yType.set('image', type.image);
        if (type.unit !== undefined) yType.set('unit', type.unit);

        if (type.ports !== undefined) {
            const validPorts = type.ports.filter(p => p.id_ !== undefined);
            const portsMap = new Y.Map(validPorts.map(p => [p.id_!, this.createPort(kitUri, type.name, type.variant || '', p)]));
            yType.set('ports', portsMap);
        }
        if (type.qualities !== undefined) {
            const qualities = new Y.Map(type.qualities.map(q => [q.name, q]));
            yType.set('qualities', qualities);
        }
        if (type.representations !== undefined) {
            const representations = new Y.Map(type.representations.map(r => [`${r.mime}:${r.tags?.join(',')}`, this.createRepresentation(kitUri, type.name, type.variant || '', r)]));
            yType.set('representations', representations);
        }

        return this.getType(kitUri, type.name, type.variant);
    }

    deleteType(kitUri: string, typeName: string, variant: string = ''): void {
        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit ${kitUri} not found`);
        const types = yKit.get('types') as Y.Map<any>;
        const variantMap = types.get(typeName) as Y.Map<any> | undefined;
        if (!variantMap) throw new Error(`Type ${typeName} not found in kit ${kitUri}`);
        variantMap.delete(variant);
        if (variantMap.size === 0) {
            types.delete(typeName);
        }
    }

    createDesign(kitUri: string, design: Design): void {
        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit ${kitUri} not found`);

        const designs = yKit.get('designs') as Y.Map<any>;
        let variantMap = designs.get(design.name) as Y.Map<any> | undefined;
        if (!variantMap) {
            variantMap = new Y.Map<any>();
            designs.set(design.name, variantMap);
        }
        let viewMap = variantMap.get(design.variant || '') as Y.Map<any> | undefined;
        if (!viewMap) {
            viewMap = new Y.Map<any>();
            variantMap.set(design.variant || '', viewMap);
        }
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
        yDesign.set('qualities', design.qualities?.map(q => this.createQuality(q)));
        viewMap.set(design.view || '', yDesign);
        design.pieces?.map(p => this.createPiece(kitUri, design.name, design.variant, design.view, p));
        design.connections?.map(c => this.createConnection(kitUri, design.name, design.variant, design.view, c));
    }

    getDesign(kitUri: string, name: string, variant: string = '', view: string = ''): Design | null {
        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) return null;

        const designs = yKit.get('designs') as Y.Map<any>;
        const yDesign = designs.get(name)?.get(variant)?.get(view) as Y.Map<any> | undefined;
        if (!yDesign) return null;

        const pieces = yDesign.get('pieces');
        const connections = yDesign.get('connections');
        const qualities = yDesign.get('qualities');

        return {
            name: yDesign.get('name'),
            description: yDesign.get('description'),
            icon: yDesign.get('icon'),
            image: yDesign.get('image'),
            variant: yDesign.get('variant'),
            view: yDesign.get('view'),
            unit: yDesign.get('unit'),
            created: new Date(),
            updated: new Date(),
            authors: [],
            pieces,
            connections,
            qualities
        };
    }

    updateDesign(kitUri: string, design: Design): Design | null {
        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) return null;

        const designs = yKit.get('designs');
        const yDesign = designs.get(design.name)?.get(design.variant || '')?.get(design.view || '');
        if (!yDesign) return null;

        if (design.description !== undefined) yDesign.set('description', design.description);
        if (design.icon !== undefined) yDesign.set('icon', design.icon);
        if (design.image !== undefined) yDesign.set('image', design.image);
        if (design.unit !== undefined) yDesign.set('unit', design.unit);

        if (design.pieces !== undefined) {
            const validPieces = design.pieces.filter(p => p.id_ !== undefined);
            const piecesMap = new Y.Map(validPieces.map(p => [p.id_!, this.createPiece(kitUri, design.name, design.variant || '', design.view || '', p)]));
            yDesign.set('pieces', piecesMap);
        }
        if (design.connections !== undefined) {
            const validConnections = design.connections.filter(c =>
                c.connected?.piece?.id_ && c.connecting?.piece?.id_ && c.connected?.port?.id_ && c.connecting?.port?.id_
            );
            const getConnectionId = (c: Connection) => `${c.connected.piece.id_}--${c.connecting.piece.id_}`;
            const connectionsMap = new Y.Map(validConnections.map(c => [getConnectionId(c), this.createConnection(kitUri, design.name, design.variant || '', design.view || '', c)]));
            yDesign.set('connections', connectionsMap);
        }
        if (design.qualities !== undefined) {
            const qualities = new Y.Map(design.qualities.map(q => [q.name, q]));
            yDesign.set('qualities', qualities);
        }

        return this.getDesign(kitUri, design.name, design.variant, design.view);
    }

    deleteDesign(kitUri: string, name: string, variant: string = '', view: string = ''): void {
        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit ${kitUri} not found`);

        const designs = yKit.get('designs') as Y.Map<any>;
        const variantMap = designs.get(name) as Y.Map<any> | undefined;
        if (!variantMap) throw new Error(`Design ${name} not found in kit ${kitUri}`);
        const viewMap = variantMap.get(variant) as Y.Map<any> | undefined;
        if (!viewMap) throw new Error(`Design ${name} not found in kit ${kitUri}`);
        viewMap.delete(view);
        if (viewMap.size === 0) {
            variantMap.delete(variant);
        }
        if (variantMap.size === 0) {
            designs.delete(name);
        }
    }

    createPiece(kitUri: string, designName: string, designVariant: string, view: string, piece: Piece): void {
        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit ${kitUri} not found`);
        const yDesigns = yKit.get('designs');
        const yDesign = yDesigns.get(designName)?.get(designVariant)?.get(view);
        if (!yDesign) throw new Error(`Design ${designName} not found in kit ${kitUri}`);
        const yPieces = yDesign.get('pieces');

        const yPiece = new Y.Map<any>();
        yPiece.set('id_', piece.id_ || Generator.randomId());
        yPiece.set('description', piece.description || '');
        const yType = new Y.Map<any>();
        yType.set('name', piece.type.name);
        yType.set('variant', piece.type.variant);
        yPiece.set('type', yType);
        if (piece.plane) {
            const yPlane = new Y.Map<any>();
            const yOrigin = new Y.Map<any>();
            yOrigin.set('x', piece.plane.origin.x);
            yOrigin.set('y', piece.plane.origin.y);
            yOrigin.set('z', piece.plane.origin.z);
            yPlane.set('origin', yOrigin);
            const yXAxis = new Y.Map<any>();
            yXAxis.set('x', piece.plane.xAxis.x);
            yXAxis.set('y', piece.plane.xAxis.y);
            yXAxis.set('z', piece.plane.xAxis.z);
            yPlane.set('xAxis', yXAxis);
            const yYAxis = new Y.Map<any>();
            yYAxis.set('x', piece.plane.yAxis.x);
            yYAxis.set('y', piece.plane.yAxis.y);
            yYAxis.set('z', piece.plane.yAxis.z);
            yPlane.set('yAxis', yYAxis);
            yPiece.set('plane', yPlane);
        }
        if (piece.center) {
            const yCenter = new Y.Map<any>();
            yCenter.set('x', piece.center.x);
            yCenter.set('y', piece.center.y);
            yPiece.set('center', yCenter);
        }
        yPiece.set('qualities', piece.qualities?.map(q => yPiece.set('qualities', this.createQuality(q))));
        yPieces.set(yPiece.get('id_'), yPiece);
    }

    getPiece(kitUri: string, designName: string, designVariant: string, view: string, pieceId: string): Piece | null {
        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) return null;

        const designs = yKit.get('designs');
        const yDesign = designs.get(designName)?.get(designVariant)?.get(view);
        if (!yDesign) return null;

        const pieces = yDesign.get('pieces');
        const yPiece = pieces.get(pieceId);
        if (!yPiece) return null;

        const type = yPiece.get('type');
        const typeName = type.get('name');
        const typeVariant = type.get('variant');

        const yPlane = yPiece.get('plane') as Y.Map<any> | undefined;
        const yOrigin = yPlane?.get('origin');
        const yXAxis = yPlane?.get('xAxis');
        const yYAxis = yPlane?.get('yAxis');
        const origin: Point | null = yOrigin ? {
            x: yOrigin.get('x'),
            y: yOrigin.get('y'),
            z: yOrigin.get('z')
        } : null;
        const xAxis: Vector | null = yXAxis ? {
            x: yXAxis.get('x'),
            y: yXAxis.get('y'),
            z: yXAxis.get('z')
        } : null;
        const yAxis: Vector | null = yYAxis ? {
            x: yYAxis.get('x'),
            y: yYAxis.get('y'),
            z: yYAxis.get('z')
        } : null;
        const plane: Plane | null = origin && xAxis && yAxis ? {
            origin,
            xAxis,
            yAxis
        } : null;

        const yCenter = yPiece.get('center') as Y.Map<any> | undefined;
        const center: DiagramPoint | null = yCenter ? {
            x: yCenter.get('x'),
            y: yCenter.get('y'),
        } : null;

        const yQualitiesArray = yPiece.get('qualities') as Y.Array<Y.Map<any>> | undefined;
        const qualities = yQualitiesArray ? yQualitiesArray.toArray().map(qMap => this.getQuality(qMap)).filter((q): q is Quality => q !== null) : [];

        if (!center) {
            console.warn(`Piece ${pieceId} in design ${designName} is missing center data.`);
            return null;
        }

        return {
            id_: yPiece.get('id_'),
            description: yPiece.get('description'),
            type: { name: typeName, variant: typeVariant },
            plane: plane ?? undefined,
            center: center,
            qualities,
        };
    }

    updatePiece(kitUri: string, designName: string, designVariant: string, view: string, piece: Piece): Piece | null {
        if (!piece.id_) throw new Error("Piece ID is required for update.");
        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit ${kitUri} not found`);
        const designs = yKit.get('designs');
        const yDesign = designs.get(designName)?.get(designVariant)?.get(view);
        if (!yDesign) throw new Error(`Design ${designName} not found in kit ${kitUri}`);
        const pieces = yDesign.get('pieces');
        const yPiece = pieces.get(piece.id_);
        if (!yPiece) throw new Error(`Piece ${piece.id_} not found in design ${designName} in kit ${kitUri}`);

        if (piece.description !== undefined) yPiece.set('description', piece.description);
        if (piece.type !== undefined) {
            const yType = new Y.Map<any>();
            yType.set('name', piece.type.name);
            yType.set('variant', piece.type.variant);
            yPiece.set('type', yType);
        }
        if (piece.center !== undefined && piece.center !== null) {
            const yCenter = new Y.Map<any>();
            yCenter.set('x', piece.center.x);
            yCenter.set('y', piece.center.y);
            yPiece.set('center', yCenter);
        }
        if (piece.plane !== undefined && piece.plane !== null) {
            const yPlane = new Y.Map<any>();
            const yOrigin = new Y.Map<any>();
            yOrigin.set('x', piece.plane.origin.x);
            yOrigin.set('y', piece.plane.origin.y);
            yOrigin.set('z', piece.plane.origin.z);
            yPlane.set('origin', yOrigin);
            const yXAxis = new Y.Map<any>();
            yXAxis.set('x', piece.plane.xAxis.x);
            yXAxis.set('y', piece.plane.xAxis.y);
            yXAxis.set('z', piece.plane.xAxis.z);
            yPlane.set('xAxis', yXAxis);
            const yYAxis = new Y.Map<any>();
            yYAxis.set('x', piece.plane.yAxis.x);
            yYAxis.set('y', piece.plane.yAxis.y);
            yYAxis.set('z', piece.plane.yAxis.z);
            yPlane.set('yAxis', yYAxis);
            yPiece.set('plane', yPlane);
        }

        return this.getPiece(kitUri, designName, designVariant, view, piece.id_);
    }

    deletePiece(kitUri: string, designName: string, designVariant: string, view: string, pieceId: string): boolean {
        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) return false;

        const designs = yKit.get('designs');
        const yDesign = designs.get(designName)?.get(designVariant)?.get(view);
        if (!yDesign) return false;

        const pieces = yDesign.get('pieces');
        return pieces.delete(pieceId);
    }

    createConnection(kitUri: string, designName: string, designVariant: string, view: string, connection: Connection): void {
        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit ${kitUri} not found`);

        const designs = yKit.get('designs');
        const yDesign = designs.get(designName)?.get(designVariant)?.get(view);
        if (!yDesign) throw new Error(`Design(${designName}, ${designVariant}, ${view}) not found in kit(${kitUri})`);

        const connections = yDesign.get('connections');
        const yConnection = new Y.Map<any>();
        const yConnectedSide = new Y.Map<any>();
        const yConnectedSidePiece = new Y.Map<any>();
        yConnectedSidePiece.set('id_', connection.connected.piece.id_);
        const yConnectedSidePort = new Y.Map<any>();
        yConnectedSidePort.set('id_', connection.connected.port.id_);
        yConnectedSide.set('piece', yConnectedSidePiece);
        yConnectedSide.set('port', yConnectedSidePort);
        const yConnectingSide = new Y.Map<any>();
        const yConnectingSidePiece = new Y.Map<any>();
        yConnectingSidePiece.set('id_', connection.connecting.piece.id_);
        const yConnectingSidePort = new Y.Map<any>();
        yConnectingSidePort.set('id_', connection.connecting.port.id_);
        yConnection.set('connected', yConnectedSide);
        yConnection.set('connecting', yConnectingSide);
        yConnection.set('description', connection.description || '');
        yConnection.set('gap', connection.gap || 0);
        yConnection.set('shift', connection.shift || 0);
        yConnection.set('raise_', connection.raise_ || 0);
        yConnection.set('rotation', connection.rotation || 0);
        yConnection.set('turn', connection.turn || 0);
        yConnection.set('tilt', connection.tilt || 0);
        yConnection.set('x', connection.x || 0);
        yConnection.set('y', connection.y || 0);
        yConnection.set('qualities', connection.qualities?.map(q => this.createQuality(q)));

        const connectionId = `${connection.connected.piece.id_}--${connection.connecting.piece.id_}`;
        connections.set(connectionId, yConnection);
    }

    getConnection(kitUri: string, designName: string, designVariant: string, view: string, connectingPieceId: string, connectedPieceId: string): Connection | null {
        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) return null;

        const designs = yKit.get('designs');
        const yDesign = designs.get(designName)?.get(designVariant)?.get(view);
        if (!yDesign) return null;

        const connections = yDesign.get('connections');
        const yConnection = connections.get(`${connectedPieceId}--${connectingPieceId}`);
        if (!yConnection) return null;

        const yConnected = yConnection.get('connected') as Y.Map<any>;
        const yConnecting = yConnection.get('connecting') as Y.Map<any>;

        const connectedPieceIdActual = yConnected?.get('piece')?.get('id_');
        const connectedPortId = yConnected?.get('port')?.get('id_');
        const connectingPieceIdActual = yConnecting?.get('piece')?.get('id_');
        const connectingPortId = yConnecting?.get('port')?.get('id_');

        if (!connectedPieceId || !connectedPortId || !connectingPieceIdActual || !connectingPortId) {
            console.warn(`Connection ${connectedPieceId}--${connectingPieceIdActual} has incomplete side data.`);
            return null; // Invalid connection data
        }

        const connectedSide: Side = {
            piece: { id_: connectedPieceId },
            port: { id_: connectedPortId }
        }
        const connectingSide: Side = {
            piece: { id_: connectingPieceIdActual },
            port: { id_: connectingPortId }
        }

        const yQualitiesArray = yConnection.get('qualities') as Y.Array<Y.Map<any>> | undefined;
        const qualities = yQualitiesArray ? yQualitiesArray.toArray().map(qMap => this.getQuality(qMap)).filter((q): q is Quality => q !== null) : [];

        return {
            description: yConnection.get('description'),
            connected: connectedSide,
            connecting: connectingSide,
            gap: yConnection.get('gap'),
            shift: yConnection.get('shift'),
            raise_: yConnection.get('raise_'),
            rotation: yConnection.get('rotation'),
            turn: yConnection.get('turn'),
            tilt: yConnection.get('tilt'),
            x: yConnection.get('x'),
            y: yConnection.get('y'),
            qualities
        };
    }

    updateConnection(kitUri: string, designName: string, designVariant: string, view: string, connectionId: string, connection: Partial<Connection>): void {
        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit ${kitUri} not found`);

        const designs = yKit.get('designs');
        const yDesign = designs.get(designName)?.get(designVariant)?.get(view);
        if (!yDesign) throw new Error(`Design ${designName} not found in kit ${kitUri}`);

        const connections = yDesign.get('connections');
        const yConnection = connections.get(connectionId);
        if (!yConnection) throw new Error(`Connection ${connectionId} not found in design ${designName}`);

        if (connection.description !== undefined) yConnection.set('description', connection.description);
        if (connection.connected !== undefined) yConnection.set('connected', connection.connected);
        if (connection.connecting !== undefined) yConnection.set('connecting', connection.connecting);
        if (connection.gap !== undefined) yConnection.set('gap', connection.gap);
        if (connection.rotation !== undefined) yConnection.set('rotation', connection.rotation);
        if (connection.shift !== undefined) yConnection.set('shift', connection.shift);
        if (connection.tilt !== undefined) yConnection.set('tilt', connection.tilt);
        if (connection.x !== undefined) yConnection.set('x', connection.x);
        if (connection.y !== undefined) yConnection.set('y', connection.y);

        const yQualities = yConnection.get('qualities') || new Y.Array<Y.Map<any>>();
        if (connection.qualities) {
            yQualities.delete(0, yQualities.length);
            connection.qualities.forEach(q => yQualities.push([this.createQuality(q)]));
        }
        yConnection.set('qualities', yQualities);
    }

    deleteConnection(kitUri: string, designName: string, designVariant: string, view: string, connectionId: string): void {
        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit ${kitUri} not found`);

        const designs = yKit.get('designs');
        const yDesign = designs.get(designName)?.get(designVariant)?.get(view);
        if (!yDesign) throw new Error(`Design ${designName} not found in kit ${kitUri}`);

        const connections = yDesign.get('connections');
        connections.delete(connectionId);
    }

    createRepresentation(kitUri: string, typeName: string, typeVariant: string, representation: Representation): void {
        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit ${kitUri} not found`);

        const types = yKit.get('types');
        const yType = types.get(typeName)?.get(typeVariant);
        if (!yType) throw new Error(`Type (${typeName}, ${typeVariant}) not found in kit (${kitUri})`);

        const representations = yType.get('representations');
        const yRepresentation = new Y.Map<any>();
        yRepresentation.set('url', representation.url);
        yRepresentation.set('description', representation.description || '');
        yRepresentation.set('mime', representation.mime);
        const yTags = Y.Array.from(representation.tags || []);
        yRepresentation.set('tags', yTags);
        yRepresentation.set('qualities', representation.qualities?.map(q => this.createQuality(q)));

        const id = `${representation.mime}:${representation.tags?.join(',') || ''}`;
        representations.set(id, yRepresentation);
    }

    getRepresentation(kitUri: string, typeName: string, typeVariant: string, mime: string, tags: string[]): Representation | null {
        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) return null;

        const types = yKit.get('types');
        const yType = types.get(typeName);
        if (!yType) return null;

        const representations = yType.get('representations');
        const yRepresentation = representations.get(`${mime}:${tags?.join(',') || ''}`);
        if (!yRepresentation) return null;

        const yQualitiesArray = yRepresentation.get('qualities') as Y.Array<Y.Map<any>> | undefined;
        const qualities = yQualitiesArray ? yQualitiesArray.toArray().map(qMap => this.getQuality(qMap)).filter((q): q is Quality => q !== null) : [];

        return {
            url: yRepresentation.get('url'),
            description: yRepresentation.get('description'),
            mime: yRepresentation.get('mime'),
            tags: yRepresentation.get('tags'),
            qualities
        };
    }

    updateRepresentation(kitUri: string, typeName: string, typeVariant: string, representation: Partial<Representation>): void {
        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit (${kitUri}) not found`);
        const types = yKit.get('types');
        const yType = types.get(typeName)?.get(typeVariant);
        if (!yType) throw new Error(`Type (${typeName}, ${typeVariant}) not found in kit (${kitUri})`);
        const representations = yType.get('representations');
        const id = `${representation.mime}:${representation.tags?.join(',') || ''}`;
        const yRepresentation = representations.get(id);
        if (!yRepresentation) throw new Error(`Representation (${id}) not found in type (${typeName}, ${typeVariant}) in kit (${kitUri})`);

        if (representation.description !== undefined) yRepresentation.set('description', representation.description);
        if (representation.mime !== undefined) yRepresentation.set('mime', representation.mime);
        if (representation.tags !== undefined) {
            const yTags = Y.Array.from(representation.tags || []);
            yRepresentation.set('tags', yTags);
        }
        if (representation.qualities !== undefined) {
            const yQualities = yRepresentation.get('qualities') || new Y.Array<Y.Map<any>>();
            yQualities.delete(0, yQualities.length);
            representation.qualities.forEach(q => yQualities.push([this.createQuality(q)]));
            yRepresentation.set('qualities', yQualities);
        }
    }

    deleteRepresentation(kitUri: string, typeName: string, typeVariant: string, mime: string, tags: string[]): void {
        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit (${kitUri}) not found`);
        const types = yKit.get('types');
        const yType = types.get(typeName)?.get(typeVariant);
        if (!yType) throw new Error(`Type (${typeName}, ${typeVariant}) not found in kit (${kitUri})`);

        const representations = yType.get('representations');
        representations.delete(`${mime}:${tags?.join(',') || ''}`);
    }

    createPort(kitUri: string, typeName: string, typeVariant: string, port: Port): void {
        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit (${kitUri}) not found`);

        const types = yKit.get('types');
        const yType = types.get(typeName)?.get(typeVariant);
        if (!yType) throw new Error(`Type (${typeName}, ${typeVariant}) not found in kit (${kitUri})`);

        const ports = yType.get('ports');
        const yPort = new Y.Map<any>();
        yPort.set('id_', port.id_ || "");
        yPort.set('description', port.description || '');
        yPort.set('family', port.family || '');

        const yCompatibleFamilies = Y.Array.from(port.compatibleFamilies || []);
        yPort.set('compatibleFamilies', yCompatibleFamilies);

        const yDirection = new Y.Map<any>();
        yDirection.set('x', port.direction.x);
        yDirection.set('y', port.direction.y);
        yDirection.set('z', port.direction.z);
        yPort.set('direction', yDirection);

        const yPoint = new Y.Map<any>();
        yPoint.set('x', port.point.x);
        yPoint.set('y', port.point.y);
        yPoint.set('z', port.point.z);
        yPort.set('point', yPoint);

        yPort.set('t', port.t || 0);
        yPort.set('qualities', port.qualities?.map(q => this.createQuality(q)));

        ports.set(yPort.get('id_'), yPort);
    }

    getPort(kitUri: string, typeName: string, typeVariant: string, id?: string): Port {

        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit (${kitUri}) not found`);
        const types = yKit.get('types') as Y.Map<any>;
        const yType = types.get(typeName)?.get(typeVariant) as Y.Map<any>;
        if (!yType) throw new Error(`Type (${typeName}, ${typeVariant}) not found in kit (${kitUri})`);
        const ports = yType.get('ports');
        const yPort = ports.get(id);
        if (!yPort) throw new Error(`Port (${id}) not found in type (${typeName}, ${typeVariant})`);

        const yCompatibleFamilies = yType.get('compatibleFamilies');
        const yDirection = yPort.get('direction');
        const yPoint = yPort.get('point');

        const yQualitiesArray = yPort.get('qualities') as Y.Array<Y.Map<any>> | undefined;
        const qualities = yQualitiesArray ? yQualitiesArray.toArray().map(qMap => this.getQuality(qMap)).filter((q): q is Quality => q !== null) : [];

        return {
            id_: yPort.get('id_'),
            description: yPort.get('description'),
            family: yPort.get('family'),
            compatibleFamilies: yCompatibleFamilies.toArray(),
            direction: {
                x: yDirection.get('x'),
                y: yDirection.get('y'),
                z: yDirection.get('z')
            },
            point: {
                x: yPoint.get('x'),
                y: yPoint.get('y'),
                z: yPoint.get('z')
            },
            t: yPort.get('t'),
            qualities
        };
    }

    updatePort(kitUri: string, typeName: string, typeVariant: string, portId: string, port: Partial<Port>): void {
        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit ${kitUri} not found`);

        const types = yKit.get('types');
        const yType = types.get(typeName)?.get(typeVariant);
        if (!yType) throw new Error(`Type (${typeName}, ${typeVariant}) not found in kit (${kitUri})`);

        const ports = yType.get('ports');
        const yPort = ports.get(portId);
        if (!yPort) throw new Error(`Port (${portId}) not found in type (${typeName}, ${typeVariant})`);

        if (port.description !== undefined) yPort.set('description', port.description);
        if (port.direction !== undefined) {
            const yDirection = new Y.Map<any>();
            yDirection.set('x', port.direction.x);
            yDirection.set('y', port.direction.y);
            yDirection.set('z', port.direction.z);
            yPort.set('direction', yDirection);
        }
        if (port.point !== undefined) {
            const yPoint = new Y.Map<any>();
            yPoint.set('x', port.point.x);
            yPoint.set('y', port.point.y);
            yPoint.set('z', port.point.z);
            yPort.set('point', yPoint);
        }
        if (port.t !== undefined) yPort.set('t', port.t);
        if (port.qualities !== undefined) {
            const yQualities = yPort.get('qualities') || new Y.Array<Y.Map<any>>();
            yQualities.delete(0, yQualities.length);
            port.qualities.forEach(q => yQualities.push([this.createQuality(q)]));
            yPort.set('qualities', yQualities);
        }
    }

    deletePort(kitUri: string, typeName: string, typeVariant: string, portId: string): void {
        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit (${kitUri}) not found`);
        const types = yKit.get('types');
        const yType = types.get(typeName)?.get(typeVariant);
        if (!yType) throw new Error(`Type (${typeName}, ${typeVariant}) not found in kit (${kitUri})`);
        const ports = yType.get('ports');
        ports.delete(portId);
    }

    createDesignEditorStore(kitUri: string, designName: string, designVariant: string, view: string): string {
        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit (${kitUri}) not found`);
        const yDesign = yKit.get('designs').get(designName)?.get(designVariant)?.get(view);
        if (!yDesign) throw new Error(`Design (${designName}, ${designVariant}, ${view}) not found in kit (${kitUri})`);
        const id = uuidv4();
        const undoManager = new UndoManager(yDesign, { trackedOrigins: new Set([id]) });
        const designEditorStore = new DesignEditorStore(id, this.yDoc, yKit, yDesign, undoManager);
        this.designEditorStores.set(id, designEditorStore);
        return id;
    }

    getDesignEditorStore(id: string): DesignEditorStore | null {
        const designEditorStore = this.designEditorStores.get(id);
        if (!designEditorStore) return null;
        return designEditorStore;
    }

    updateDesignEditorSelection(id: string, selection: DesignEditorSelection): DesignEditorSelection {
        const designEditorStore = this.designEditorStores.get(id);
        if (!designEditorStore) throw new Error(`Design editor store ${id} not found`);
        designEditorStore.setState({ ...designEditorStore.getState(), selection });
        return selection;
    }

    deleteDesignEditorStore(id: string): void {
        this.designEditorStores.delete(id);
    }

    importKit(url: string): void {
        const importedKitData = metabolism as unknown as Omit<Kit, 'created' | 'updated'>;
        const kitToCreate: Kit = {
            ...importedKitData,
            created: new Date(),
            updated: new Date(),
        };
        this.createKit(kitToCreate);
    }

    undo(): void {
        this.undoManager.undo();
    }

    redo(): void {
        this.undoManager.redo();
    }

    transact(commands: () => void): void {
        this.yDoc.transact(commands, new Set([this.userId]));
    }
}


const StudioStoreContext = createContext<StudioStore | null>(null);

export const useStudioStore = () => {
    const studioStore = useContext(StudioStoreContext);
    if (!studioStore) {
        throw new Error('useStudioStore must be used within a StudioStoreProvider');
    }
    return studioStore;
};

export const StudioStoreProvider: FC<{ userId: string, children: React.ReactNode }> = ({ userId, children }) => {
    const studioStore = useMemo(() => new StudioStore(userId), [userId]);
    return (
        <StudioStoreContext.Provider value={studioStore}>
            {children}
        </StudioStoreContext.Provider>
    );
};

const DesignEditorStoreContext = createContext<DesignEditorStore | null>(null);

export const useDesignEditorStore = () => {
    const designEditorStore = useContext(DesignEditorStoreContext);
    if (!designEditorStore) {
        throw new Error('useDesignEditorStore must be used within a DesignEditorStoreProvider');
    }
    return designEditorStore;
};

export const DesignEditorStoreProvider: FC<{ designEditorId: string, children: React.ReactNode }> = ({ designEditorId, children }) => {
    const studioStore = useStudioStore();
    const designEditorStore = useMemo(() => studioStore.getDesignEditorStore(designEditorId), [studioStore, designEditorId]);
    return (
        <DesignEditorStoreContext.Provider value={designEditorStore}>
            {children}
        </DesignEditorStoreContext.Provider>
    );
};