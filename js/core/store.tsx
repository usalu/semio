import { v4 as uuidv4 } from 'uuid';
import * as Y from 'yjs';
import React, { createContext, useContext, useEffect, useState, useMemo, FC } from 'react';
import { UndoManager } from 'yjs';
import { IndexeddbPersistence } from 'y-indexeddb';
import JSZip from 'jszip';
// Import initSqlJs
import initSqlJs from 'sql.js';
import sqlWasm from 'sql.js/dist/sql-wasm.wasm';

import { Generator } from '@semio/js/lib/utils';
import { Kit, Port, Representation, Piece, Connection, Type, Design, Plane, DiagramPoint, Point, Vector, Quality, Author, Side } from '@semio/js/semio';

// import { default as metabolism } from '@semio/assets/semio/kit_metabolism.json';


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
//     files: Y.Map<Uint8Array>;
// }

class StudioStore {
    private userId: string;
    private yDoc: Y.Doc;
    private undoManager: UndoManager;
    private designEditorStores: Map<string, DesignEditorStore>;
    private indexeddbProvider: IndexeddbPersistence;
    private listeners: Set<() => void> = new Set();

    constructor(userId: string) {
        this.userId = userId;
        this.yDoc = new Y.Doc();
        this.yDoc.getMap('kits');
        this.yDoc.getMap('files');
        this.undoManager = new UndoManager(this.yDoc, { trackedOrigins: new Set([this.userId]) });
        this.designEditorStores = new Map();
        this.indexeddbProvider = new IndexeddbPersistence(userId, this.yDoc);
        this.indexeddbProvider.whenSynced.then(() => {
            console.log(`Local changes are synchronized for user (${this.userId}) with client (${this.yDoc.clientID})`);
            this.importKit('metabolism.zip');
        });
        this.yDoc.on('update', () => this.notifyListeners());
    }

    private notifyListeners(): void {
        this.listeners.forEach(listener => listener());
    }

    private createQuality(quality: Quality): Y.Map<any> {
        const yQuality = new Y.Map<any>();
        yQuality.set('name', quality.name);
        yQuality.set('value', quality.value || '');
        yQuality.set('unit', quality.unit || '');
        yQuality.set('definition', quality.definition || '');
        return yQuality;
    }

    private createQualities(qualities: Quality[] | undefined): Y.Map<any> {
        const yQualities = new Y.Map<any>();
        if (qualities && qualities.length > 0) {
            qualities.forEach(q => yQualities.set(q.name, this.createQuality(q)));
        }
        return yQualities;
    }

    private getQuality(yMap: Y.Map<any>): Quality | null {
        const name = yMap.get('name');
        const value = yMap.get('value');
        const unit = yMap.get('unit');
        const definition = yMap.get('definition');
        return { name, value, unit, definition };
    }

    private getQualities(yQualitiesMap: Y.Map<any> | undefined): Quality[] {
        const qualities: Quality[] = [];
        if (yQualitiesMap && yQualitiesMap.size > 0) {
            Array.from(yQualitiesMap.values()).forEach(qMap => {
                const quality = this.getQuality(qMap);
                if (quality) {
                    qualities.push(quality);
                }
            });
        }
        return qualities;
    }

    private createAuthor(author: Author): Y.Map<any> {
        const yAuthor = new Y.Map<any>();
        yAuthor.set('name', author.name);
        yAuthor.set('email', author.email);
        yAuthor.set('rank', author.rank);
        return yAuthor;
    }

    private createAuthors(authors: Author[] | undefined): Y.Map<any> {
        const yAuthors = new Y.Map<any>();
        if (authors && authors.length > 0) {
            authors.forEach(a => yAuthors.set(a.name, this.createAuthor(a)));
        }
        return yAuthors;
    }

    private getAuthor(yMap: Y.Map<any>): Author | null {
        const name = yMap.get('name');
        const email = yMap.get('email');
        const rank = yMap.get('rank');
        return { name, email, rank };
    }

    private getAuthors(yAuthorsMap: Y.Map<any> | undefined): Author[] {
        const authors: Author[] = [];
        if (yAuthorsMap && yAuthorsMap.size > 0) {
            Array.from(yAuthorsMap.values()).forEach(aMap => {
                const author = this.getAuthor(aMap);
                if (author) {
                    authors.push(author);
                }
            });
        }
        return authors;
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
        yKit.set('qualities', this.createQualities(kit.qualities) || []);

        this.yDoc.getMap('kits').set(kit.uri, yKit);
        kit.types?.map(t => this.createType(kit.uri, t));
        kit.designs?.map(d => this.createDesign(kit.uri, d));
    }

    getKit(uri: string): Kit {
        const yKit = this.yDoc.getMap('kits').get(uri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit ${uri} not found`);

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
            qualities: this.getQualities(yKit.get('qualities'))
        };
    }

    updateKit(kit: Partial<Kit>): Kit {
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
        yType.set('representations', new Y.Map());
        yType.set('ports', new Y.Map());
        yType.set('authors', this.createAuthors(type.authors));
        yType.set('qualities', this.createQualities(type.qualities) || []);
        variantMap.set(type.variant || '', yType);
        type.representations?.map(r => { this.createRepresentation(kitUri, type.name, type.variant || '', r) });
        type.ports?.map(p => { this.createPort(kitUri, type.name, type.variant || '', p) });
        yType.set('created', new Date().toISOString());
        yType.set('updated', new Date().toISOString());
    }

    getType(kitUri: string, name: string, variant: string = ''): Type {
        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit (${kitUri}) not found`);
        const types = yKit.get('types') as Y.Map<any>;
        const yType = types.get(name)?.get(variant) as Y.Map<any> | undefined;
        if (!yType) throw new Error(`Type (${name}, ${variant}) not found in kit (${kitUri})`);

        const yRepresentationsMap = yType.get('representations') as Y.Map<Y.Map<any>>;
        const representations = yRepresentationsMap ? Array.from(yRepresentationsMap.values()).map(rMap => this.getRepresentation(kitUri, name, variant, rMap.get('mime'), rMap.get('tags')?.toArray() || [])).filter((r): r is Representation => r !== null) : [];
        const yPortsMap = yType.get('ports') as Y.Map<Y.Map<any>>;
        const ports = yPortsMap ? Array.from(yPortsMap.values()).map(pMap => this.getPort(kitUri, name, variant, pMap.get('id_'))).filter((p): p is Port => p !== null) : [];
        return {
            name: yType.get('name'),
            description: yType.get('description'),
            icon: yType.get('icon'),
            image: yType.get('image'),
            variant: yType.get('variant'),
            unit: yType.get('unit'),
            ports,
            representations,
            qualities: this.getQualities(yType.get('qualities')),
            authors: this.getAuthors(yType.get('authors')),
            created: yType.get('created'),
            updated: yType.get('updated'),
        };
    }

    updateType(kitUri: string, type: Type): Type {
        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit (${kitUri}) not found`);
        const types = yKit.get('types');
        const yType = types.get(type.name)?.get(type.variant || '');
        if (!yType) throw new Error(`Type (${type.name}, ${type.variant}) not found in kit (${kitUri})`);

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
        if (type.authors !== undefined) {
            const authors = new Y.Map(type.authors.map(a => [a.name, this.createAuthor(a)]));
            yType.set('authors', authors);
        }

        yType.set('updated', new Date().toISOString());
        return this.getType(kitUri, type.name, type.variant);
    }

    deleteType(kitUri: string, name: string, variant: string = ''): void {
        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit ${kitUri} not found`);
        const types = yKit.get('types') as Y.Map<any>;
        const variantMap = types.get(name) as Y.Map<any> | undefined;
        if (!variantMap) throw new Error(`Type (${name}, ${variant}) not found in kit (${kitUri})`);
        variantMap.delete(variant);
        if (variantMap.size === 0) {
            types.delete(name);
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
        yDesign.set('qualities', this.createQualities(design.qualities) || []);
        yDesign.set('authors', this.createAuthors(design.authors));
        viewMap.set(design.view || '', yDesign);
        design.pieces?.map(p => this.createPiece(kitUri, design.name, design.variant, design.view, p));
        design.connections?.map(c => this.createConnection(kitUri, design.name, design.variant, design.view, c));
        yDesign.set('created', new Date().toISOString());
        yDesign.set('updated', new Date().toISOString());
    }

    getDesign(kitUri: string, name: string, variant: string = '', view: string = ''): Design {
        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit (${kitUri}) not found`);
        const designs = yKit.get('designs') as Y.Map<any>;
        const yDesign = designs.get(name)?.get(variant)?.get(view) as Y.Map<any> | undefined;
        if (!yDesign) throw new Error(`Design (${name}, ${variant}, ${view}) not found in kit (${kitUri})`);

        const yPieces = yDesign.get('pieces') as Y.Map<any>;
        const pieces = yPieces ? Array.from(yPieces.values()).map(pMap => this.getPiece(kitUri, name, variant, view, pMap.get('id_'))).filter((p): p is Piece => p !== null) : [];
        const yConnections = yDesign.get('connections') as Y.Map<any>;
        const connections = yConnections ? Array.from(yConnections.values()).map(cMap => this.getConnection(kitUri, name, variant, view, cMap.get('connected').get('piece').get('id_'), cMap.get('connecting').get('piece').get('id_'))).filter((c): c is Connection => c !== null) : [];
        return {
            name: yDesign.get('name'),
            description: yDesign.get('description'),
            icon: yDesign.get('icon'),
            image: yDesign.get('image'),
            variant: yDesign.get('variant'),
            view: yDesign.get('view'),
            unit: yDesign.get('unit'),
            created: yDesign.get('created'),
            updated: yDesign.get('updated'),
            authors: this.getAuthors(yDesign.get('authors')),
            pieces,
            connections,
            qualities: this.getQualities(yDesign.get('qualities')),
        };
    }

    updateDesign(kitUri: string, design: Design): Design {
        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit (${kitUri}) not found`);

        const designs = yKit.get('designs');
        const yDesign = designs.get(design.name)?.get(design.variant || '')?.get(design.view || '');
        if (!yDesign) throw new Error(`Design (${design.name}, ${design.variant}, ${design.view}) not found in kit (${kitUri})`);

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
        yPiece.set('qualities', this.createQualities(piece.qualities) || []);
        yPieces.set(piece.id_, yPiece);
    }

    getPiece(kitUri: string, designName: string, designVariant: string, view: string, pieceId: string): Piece {
        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit (${kitUri}) not found`);
        const designs = yKit.get('designs');
        const yDesign = designs.get(designName)?.get(designVariant)?.get(view);
        if (!yDesign) throw new Error(`Design (${designName}, ${designVariant}, ${view}) not found in kit (${kitUri})`);
        const pieces = yDesign.get('pieces');
        const yPiece = pieces.get(pieceId);
        if (!yPiece) throw new Error(`Piece (${pieceId}) not found in design (${designName}, ${designVariant}, ${view}) in kit (${kitUri})`);

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

        return {
            id_: yPiece.get('id_'),
            description: yPiece.get('description'),
            type: { name: typeName, variant: typeVariant },
            plane: plane ?? undefined,
            center: center ?? undefined,
            qualities: this.getQualities(yPiece.get('qualities')),
        };
    }

    updatePiece(kitUri: string, designName: string, designVariant: string, designView: string, piece: Piece): Piece {
        if (!piece.id_) throw new Error("Piece ID is required for update.");
        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit ${kitUri} not found`);
        const designs = yKit.get('designs');
        const yDesign = designs.get(designName)?.get(designVariant)?.get(designView);
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

        return this.getPiece(kitUri, designName, designVariant, designView, piece.id_);
    }

    deletePiece(kitUri: string, designName: string, designVariant: string, designView: string, pieceId: string): boolean {
        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit (${kitUri}) not found`);
        const designs = yKit.get('designs');
        const yDesign = designs.get(designName)?.get(designVariant)?.get(designView);
        if (!yDesign) throw new Error(`Design (${designName}, ${designVariant}, ${designView}) not found in kit (${kitUri})`);

        const pieces = yDesign.get('pieces');
        return pieces.delete(pieceId);
    }

    createConnection(kitUri: string, designName: string, designVariant: string, designView: string, connection: Connection): void {
        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit (${kitUri}) not found`);
        const designs = yKit.get('designs');
        const yDesign = designs.get(designName)?.get(designVariant)?.get(designView);
        if (!yDesign) throw new Error(`Design(${designName}, ${designVariant}, ${designView}) not found in kit(${kitUri})`);
        const connectionId = `${connection.connected.piece.id_}--${connection.connecting.piece.id_}`;
        const reverseConnectionId = `${connection.connecting.piece.id_}--${connection.connected.piece.id_}`;
        const connections = yDesign.get('connections');
        if (connections.get(connectionId) || connections.get(reverseConnectionId)) {
            throw new Error(`Connection (${connectionId}) already exists in design (${designName}, ${designVariant}, ${designView}) in kit (${kitUri})`);
        }

        const yConnection = new Y.Map<any>();
        const yConnectedSide = new Y.Map<any>();
        const yConnectedSidePiece = new Y.Map<any>();
        yConnectedSidePiece.set('id_', connection.connected.piece.id_);
        const yConnectedSidePort = new Y.Map<any>();
        yConnectedSidePort.set('id_', connection.connected.port.id_);
        yConnectedSide.set('piece', yConnectedSidePiece);
        yConnectedSide.set('port', yConnectedSidePort);
        yConnection.set('connected', yConnectedSide);
        const yConnectingSide = new Y.Map<any>();
        const yConnectingSidePiece = new Y.Map<any>();
        yConnectingSidePiece.set('id_', connection.connecting.piece.id_);
        const yConnectingSidePort = new Y.Map<any>();
        yConnectingSidePort.set('id_', connection.connecting.port.id_);
        yConnectingSide.set('piece', yConnectingSidePiece);
        yConnectingSide.set('port', yConnectingSidePort);
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
        yConnection.set('qualities', this.createQualities(connection.qualities));
        connections.set(connectionId, yConnection);
    }

    getConnection(kitUri: string, designName: string, designVariant: string, designView: string, connectedPieceId: string, connectingPieceId: string): Connection {
        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit (${kitUri}) not found`);
        const designs = yKit.get('designs');
        const yDesign = designs.get(designName)?.get(designVariant)?.get(designView);
        if (!yDesign) throw new Error(`Design (${designName}, ${designVariant}, ${designView}) not found in kit (${kitUri})`);
        const connections = yDesign.get('connections');
        const yConnection = connections.get(`${connectedPieceId}--${connectingPieceId}`);
        if (!yConnection) throw new Error(`Connection (${connectedPieceId}, ${connectingPieceId}) not found in design (${designName}, ${designVariant}, ${designView}) in kit (${kitUri})`);

        const yConnected = yConnection.get('connected') as Y.Map<any>;
        const connectedSide: Side = {
            piece: { id_: yConnected?.get('piece')?.get('id_') || '' },
            port: { id_: yConnected?.get('port')?.get('id_') || '' }
        }
        const yConnecting = yConnection.get('connecting') as Y.Map<any>;
        const connectingSide: Side = {
            piece: { id_: yConnecting?.get('piece')?.get('id_') || '' },
            port: { id_: yConnecting?.get('port')?.get('id_') || '' }
        }
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
            qualities: this.getQualities(yConnection.get('qualities'))
        };
    }

    updateConnection(kitUri: string, designName: string, designVariant: string, designView: string, connection: Partial<Connection>): void {
        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit (${kitUri}) not found`);
        const designs = yKit.get('designs');
        const yDesign = designs.get(designName)?.get(designVariant)?.get(designView);
        if (!yDesign) throw new Error(`Design (${designName}, ${designVariant}, ${designView}) not found in kit (${kitUri})`);
        const connections = yDesign.get('connections');
        const yConnection = connections.get(`${connection.connected?.piece.id_}--${connection.connecting?.piece.id_}`);
        if (!yConnection) throw new Error(`Connection (${connection.connected?.piece.id_}, ${connection.connecting?.piece.id_}) not found in design (${designName}, ${designVariant}, ${designView}) in kit (${kitUri})`);

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

    deleteConnection(kitUri: string, designName: string, designVariant: string, designView: string, connectedPieceId: string, connectingPieceId: string): void {
        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit (${kitUri}) not found`);

        const designs = yKit.get('designs');
        const yDesign = designs.get(designName)?.get(designVariant)?.get(designView);
        if (!yDesign) throw new Error(`Design (${designName}, ${designVariant}, ${designView}) not found in kit (${kitUri})`);

        const connections = yDesign.get('connections');
        connections.delete(`${connectedPieceId}--${connectingPieceId}`);
    }

    createRepresentation(kitUri: string, typeName: string, typeVariant: string, representation: Representation): void {
        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit (${kitUri}) not found`);
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
        yRepresentation.set('qualities', this.createQualities(representation.qualities || []));

        const id = `${representation.mime}:${representation.tags?.join(',') || ''}`;
        representations.set(id, yRepresentation);
    }

    getRepresentation(kitUri: string, typeName: string, typeVariant: string, mime: string, tags: string[]): Representation {
        const yKit = this.yDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit (${kitUri}) not found`);
        const types = yKit.get('types');
        const yType = types.get(typeName)?.get(typeVariant);
        if (!yType) throw new Error(`Type (${typeName}, ${typeVariant}) not found in kit (${kitUri})`);
        const representations = yType.get('representations');
        const yRepresentation = representations.get(`${mime}:${tags?.join(',') || ''}`);
        if (!yRepresentation) throw new Error(`Representation (${mime}, ${tags?.join(',') || ''}) not found in type (${typeName}, ${typeVariant}) in kit (${kitUri})`);

        return {
            url: yRepresentation.get('url'),
            description: yRepresentation.get('description'),
            mime: yRepresentation.get('mime'),
            tags: yRepresentation.get('tags').toArray(),
            qualities: this.getQualities(yRepresentation.get('qualities'))
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
        yPort.set('qualities', this.createQualities(port.qualities || []));

        ports.set(port.id_, yPort);
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

        const yDirection = yPort.get('direction');
        const yPoint = yPort.get('point');
        return {
            id_: yPort.get('id_'),
            description: yPort.get('description'),
            family: yPort.get('family'),
            compatibleFamilies: yPort.get('compatibleFamilies')?.toArray() || [],
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
            qualities: this.getQualities(yPort.get('qualities'))
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
        const designEditorStore = new DesignEditorStore(id, this.yDoc, yKit, yDesign);
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

    undo(): void {
        this.undoManager.undo();
    }

    redo(): void {
        this.undoManager.redo();
    }

    transact(commands: () => void): void {
        this.yDoc.transact(commands, new Set([this.userId]));
    }

    subscribe(callback: () => void): () => void {
        this.listeners.add(callback);
        return () => {
            this.listeners.delete(callback);
        };
    }

    async importKit(url: string, complete = false): Promise<void> {
        let SQL: any;
        let kitDb: any; // Define kitDb here to access in finally block
        try {
            // Initialize SQL.js specifically for this import operation
            SQL = await initSqlJs({ locateFile: () => sqlWasm });
            console.log("SQL.js initialized for import.");
        } catch (err) {
            console.error("Failed to initialize SQL.js for import:", err);
            throw new Error("SQL.js failed to initialize for import."); // Rethrow or handle as appropriate
        }

        const zipData = await fetch(url).then(res => res.arrayBuffer());
        const zip = await JSZip.loadAsync(zipData);

        const kitDbFileEntry = zip.file("semio/kit.db");
        if (!kitDbFileEntry) {
            throw new Error("kit.db not found in the zip file at path semio/kit.db");
        }
        const kitDbFile = await kitDbFileEntry.async("uint8array");
        kitDb = new SQL.Database(kitDbFile); // Assign to outer scope variable


        let kit: Kit | null = null;
        let types: Type[] = [];
        let designs: Design[] = [];
        const filesMap = this.yDoc.getMap('files');

        try {
            // Extract kit
            const kitRes = kitDb.exec("SELECT uri, name, description, icon, image, preview, version, remote, homepage, license, created, updated FROM kit LIMIT 1");
            if (!kitRes || kitRes.length === 0 || !kitRes[0].values || kitRes[0].values.length === 0) throw new Error("Kit data not found in database.");
            const kitRow: any[] = kitRes[0].values[0];
            kit = {
                uri: kitRow[0], name: kitRow[1], description: kitRow[2], icon: kitRow[3], image: kitRow[4], preview: kitRow[5],
                version: kitRow[6], remote: kitRow[7], homepage: kitRow[8], license: kitRow[9],
                created: new Date(kitRow[10]), updated: new Date(kitRow[11]),
                types: [], designs: [], qualities: [] // Initialize arrays
            };
            const kitIdRes = kitDb.exec("SELECT id FROM kit WHERE uri = ?", [kit.uri]);
            const kitId = kitIdRes[0].values[0][0];


            // --- Helper function to get Qualities ---
            const getQualities = (tableName: string, fkColumn: string, fkValue: number | string): Quality[] => {
                const query = `SELECT name, value, unit, definition FROM quality WHERE ${fkColumn} = ?`;
                // Use kitDb directly
                const res = kitDb.exec(query, [fkValue]);
                if (!res || res.length === 0 || !res[0].values) return [];
                return res[0].values.map((row: any[]) => ({ name: row[0], value: row[1], unit: row[2], definition: row[3] }));
            };
            // --- Helper function to get Authors ---
            const getAuthors = (tableName: string, fkColumn: string, fkValue: number | string): Author[] => {
                const query = `SELECT name, email, rank FROM author WHERE ${fkColumn} = ?`;
                // Use kitDb directly
                const res = kitDb.exec(query, [fkValue]);
                if (!res || res.length === 0 || !res[0].values) return [];
                return res[0].values.map((row: any[]) => ({ name: row[0], email: row[1], rank: row[2] }));
            };

            // Extract kit qualities
            kit.qualities = getQualities('quality', 'kit_id', kitId);


            // Extract types
            const typeRes = kitDb.exec("SELECT id, name, description, icon, image, variant, unit, created, updated FROM type WHERE kit_id = ?", [kitId]);
            if (typeRes && typeRes.length > 0 && typeRes[0].values) {
                for (const typeRow of typeRes[0].values) {
                    const typeId = typeRow[0];
                    const type: Type = {
                        name: typeRow[1], description: typeRow[2], icon: typeRow[3], image: typeRow[4], variant: typeRow[5],
                        unit: typeRow[6], created: new Date(typeRow[7]), updated: new Date(typeRow[8]),
                        representations: [], ports: [], qualities: [], authors: []
                    };

                    // Extract representations for this type
                    const repRes = kitDb.exec("SELECT id, url, description, mime FROM representation WHERE type_id = ?", [typeId]);
                    if (repRes && repRes.length > 0 && repRes[0].values) {
                        for (const repRow of repRes[0].values) {
                            const repId = repRow[0];
                            const url = repRow[1];
                            const representation: Representation = {
                                url: url, description: repRow[2], mime: repRow[3], tags: [], qualities: []
                            };
                            // Extract tags for this representation
                            const tagRes = kitDb.exec("SELECT name FROM tag WHERE representation_id = ? ORDER BY \"order\"", [repId]);
                            if (tagRes && tagRes.length > 0 && tagRes[0].values) {
                                representation.tags = tagRes[0].values.map((row: any[]) => row[0]);
                            }
                            representation.qualities = getQualities('quality', 'representation_id', repId);

                            // Extract file data from zip and store in filesMap
                            const fileEntry = zip.file(url);
                            if (fileEntry) {
                                const fileData = await fileEntry.async("uint8array");
                                filesMap.set(url, fileData);
                            } else if (complete && !url.startsWith("http")) {
                                console.warn(`Representation file not found in zip: ${url}`);
                            }
                            type.representations!.push(representation);
                        }
                    }

                    // Extract ports for this type
                    const portRes = kitDb.exec("SELECT id, local_id, description, family, t, point_x, point_y, point_z, direction_x, direction_y, direction_z FROM port WHERE type_id = ?", [typeId]);
                    if (portRes && portRes.length > 0 && portRes[0].values) {
                        for (const portRow of portRes[0].values) {
                            const portId = portRow[0];
                            const port: Port = {
                                id_: portRow[1], description: portRow[2], family: portRow[3], t: portRow[4],
                                point: { x: portRow[5], y: portRow[6], z: portRow[7] },
                                direction: { x: portRow[8], y: portRow[9], z: portRow[10] },
                                compatibleFamilies: [], qualities: []
                            };
                            // Extract compatible families for this port
                            const compFamRes = kitDb.exec("SELECT name FROM compatible_family WHERE port_id = ? ORDER BY \"order\"", [portId]);
                            if (compFamRes && compFamRes.length > 0 && compFamRes[0].values) {
                                port.compatibleFamilies = compFamRes[0].values.map((row: any[]) => row[0]);
                            }
                            port.qualities = getQualities('quality', 'port_id', portId);
                            type.ports!.push(port);
                        }
                    }
                    type.qualities = getQualities('quality', 'type_id', typeId);
                    type.authors = getAuthors('author', 'type_id', typeId);
                    types.push(type);
                }
            }
            kit.types = types;


            // Extract designs
            const designRes = kitDb.exec("SELECT id, name, description, icon, image, variant, \"view\", unit, created, updated FROM design WHERE kit_id = ?", [kitId]);
            if (designRes && designRes.length > 0 && designRes[0].values) {
                for (const designRow of designRes[0].values) {
                    const designId = designRow[0];
                    const design: Design = {
                        name: designRow[1], description: designRow[2], icon: designRow[3], image: designRow[4], variant: designRow[5],
                        view: designRow[6], unit: designRow[7], created: new Date(designRow[8]), updated: new Date(designRow[9]),
                        pieces: [], connections: [], qualities: [], authors: []
                    };

                    // Extract pieces for this design
                    const pieceRes = kitDb.exec("SELECT p.id, p.local_id, p.description, t.name, t.variant, pl.origin_x, pl.origin_y, pl.origin_z, pl.x_axis_x, pl.x_axis_y, pl.x_axis_z, pl.y_axis_x, pl.y_axis_y, pl.y_axis_z, p.center_x, p.center_y FROM piece p JOIN type t ON p.type_id = t.id LEFT JOIN plane pl ON p.plane_id = pl.id WHERE p.design_id = ?", [designId]);
                    const pieceMap: { [key: string]: Piece } = {}; // Map local_id to piece for connections
                    const pieceIdMap: { [dbId: number]: string } = {}; // Map db id to local_id
                    if (pieceRes && pieceRes.length > 0 && pieceRes[0].values) {
                        for (const pieceRow of pieceRes[0].values) {
                            const pieceId = pieceRow[0];
                            const localId = pieceRow[1];
                            const plane: Plane | undefined = pieceRow[5] !== null ? { // Check if plane data exists
                                origin: { x: pieceRow[5], y: pieceRow[6], z: pieceRow[7] },
                                xAxis: { x: pieceRow[8], y: pieceRow[9], z: pieceRow[10] },
                                yAxis: { x: pieceRow[11], y: pieceRow[12], z: pieceRow[13] }
                            } : undefined;
                            const piece: Piece = {
                                id_: localId,
                                description: pieceRow[2],
                                type: { name: pieceRow[3], variant: pieceRow[4] },
                                plane: plane,
                                center: pieceRow[14] !== null ? { x: pieceRow[14], y: pieceRow[15] } : undefined,
                                qualities: []
                            };
                            piece.qualities = getQualities('quality', 'piece_id', pieceId);
                            design.pieces!.push(piece);
                            pieceMap[localId] = piece;
                            pieceIdMap[pieceId] = localId;
                        }
                    }

                    // Extract connections for this design
                    const connRes = kitDb.exec("SELECT c.id, c.description, c.gap, c.shift, c.raise_, c.rotation, c.turn, c.tilt, c.x, c.y, c.connected_piece_id, cp.local_id AS connected_port_id, c.connecting_piece_id, cnp.local_id AS connecting_port_id FROM connection c JOIN port cp ON c.connected_port_id = cp.id JOIN port cnp ON c.connecting_port_id = cnp.id WHERE c.design_id = ?", [designId]);
                    if (connRes && connRes.length > 0 && connRes[0].values) {
                        for (const connRow of connRes[0].values) {
                            const connId = connRow[0];
                            const connectedPieceLocalId = pieceIdMap[connRow[10]];
                            const connectingPieceLocalId = pieceIdMap[connRow[12]];
                            if (!connectedPieceLocalId || !connectingPieceLocalId) {
                                console.warn(`Could not find piece local IDs for connection ${connId}`);
                                continue;
                            }
                            const connection: Connection = {
                                description: connRow[1], gap: connRow[2], shift: connRow[3], raise_: connRow[4], rotation: connRow[5],
                                turn: connRow[6], tilt: connRow[7], x: connRow[8], y: connRow[9],
                                connected: { piece: { id_: connectedPieceLocalId }, port: { id_: connRow[11] } },
                                connecting: { piece: { id_: connectingPieceLocalId }, port: { id_: connRow[13] } },
                                qualities: []
                            };
                            connection.qualities = getQualities('quality', 'connection_id', connId);
                            design.connections!.push(connection);
                        }
                    }
                    design.qualities = getQualities('quality', 'design_id', designId);
                    design.authors = getAuthors('author', 'design_id', designId);
                    designs.push(design);
                }
            }
            kit.designs = designs;

            // Create the kit in the store
            this.createKit(kit);
            console.log(`Kit "${kit.name}" imported successfully from ${url}`);

        } catch (error) {
            console.error("Error importing kit:", error);
            throw error; // Re-throw the error after logging
        } finally {
            // Ensure the database is closed for this specific import
            if (kitDb) {
                kitDb.close();
                console.log("SQL.js database closed for import.");
            }
        }
    }

    async exportKit(kitUri: string, complete = false): Promise<Blob> {
        let SQL: any;
        let db: any; // Define db here to access in finally block
        try {
            // Initialize SQL.js specifically for this export operation
            SQL = await initSqlJs({ locateFile: file => `/${file}` });
            console.log("SQL.js initialized for export.");
        } catch (err) {
            console.error("Failed to initialize SQL.js for export:", err);
            throw new Error("SQL.js failed to initialize for export.");
        }

        const kit = this.getKit(kitUri);
        db = new SQL.Database(); // Assign to outer scope variable
        const zip = new JSZip();
        const filesMap = this.yDoc.getMap('files');

        // Use schema from sqlite/schema.sql
        const schema = `
            CREATE TABLE kit ( uri VARCHAR(2048) NOT NULL UNIQUE, name VARCHAR(64) NOT NULL, description VARCHAR(512) NOT NULL, icon VARCHAR(1024) NOT NULL, image VARCHAR(1024) NOT NULL, preview VARCHAR(1024) NOT NULL, version VARCHAR(64) NOT NULL, remote VARCHAR(1024) NOT NULL, homepage VARCHAR(1024) NOT NULL, license VARCHAR(1024) NOT NULL, created DATETIME NOT NULL, updated DATETIME NOT NULL, id INTEGER NOT NULL PRIMARY KEY );
            CREATE TABLE type ( name VARCHAR(64) NOT NULL, description VARCHAR(512) NOT NULL, icon VARCHAR(1024) NOT NULL, image VARCHAR(1024) NOT NULL, variant VARCHAR(64) NOT NULL, unit VARCHAR(64) NOT NULL, created DATETIME NOT NULL, updated DATETIME NOT NULL, id INTEGER NOT NULL PRIMARY KEY, kit_id INTEGER, CONSTRAINT "Unique name and variant" UNIQUE (name, variant, kit_id), FOREIGN KEY(kit_id) REFERENCES kit (id) );
            CREATE TABLE design ( name VARCHAR(64) NOT NULL, description VARCHAR(512) NOT NULL, icon VARCHAR(1024) NOT NULL, image VARCHAR(1024) NOT NULL, variant VARCHAR(64) NOT NULL, "view" VARCHAR(64) NOT NULL, unit VARCHAR(64) NOT NULL, created DATETIME NOT NULL, updated DATETIME NOT NULL, id INTEGER NOT NULL PRIMARY KEY, kit_id INTEGER, UNIQUE (name, variant, "view", kit_id), FOREIGN KEY(kit_id) REFERENCES kit (id) );
            CREATE TABLE representation ( url VARCHAR(1024) NOT NULL, description VARCHAR(512) NOT NULL, mime VARCHAR(64) NOT NULL, id INTEGER NOT NULL PRIMARY KEY, type_id INTEGER, FOREIGN KEY(type_id) REFERENCES type (id) );
            CREATE TABLE tag ( name VARCHAR(64) NOT NULL, "order" INTEGER NOT NULL, id INTEGER NOT NULL PRIMARY KEY, representation_id INTEGER, FOREIGN KEY(representation_id) REFERENCES representation (id) );
            CREATE TABLE port ( description VARCHAR(512) NOT NULL, family VARCHAR(64) NOT NULL, t FLOAT NOT NULL, id INTEGER NOT NULL PRIMARY KEY, local_id VARCHAR(128), point_x FLOAT, point_y FLOAT, point_z FLOAT, direction_x FLOAT, direction_y FLOAT, direction_z FLOAT, type_id INTEGER, CONSTRAINT "Unique local_id" UNIQUE (local_id, type_id), FOREIGN KEY(type_id) REFERENCES type (id) );
            CREATE TABLE compatible_family ( name VARCHAR(64) NOT NULL, "order" INTEGER NOT NULL, id INTEGER NOT NULL PRIMARY KEY, port_id INTEGER, FOREIGN KEY(port_id) REFERENCES port (id) );
            CREATE TABLE plane ( id INTEGER NOT NULL PRIMARY KEY, origin_x FLOAT, origin_y FLOAT, origin_z FLOAT, x_axis_x FLOAT, x_axis_y FLOAT, x_axis_z FLOAT, y_axis_x FLOAT, y_axis_y FLOAT, y_axis_z FLOAT );
            CREATE TABLE piece ( description VARCHAR(512) NOT NULL, id INTEGER NOT NULL PRIMARY KEY, local_id VARCHAR(128), type_id INTEGER, plane_id INTEGER, center_x FLOAT, center_y FLOAT, design_id INTEGER, UNIQUE (local_id, design_id), FOREIGN KEY(type_id) REFERENCES type (id), FOREIGN KEY(plane_id) REFERENCES plane (id), FOREIGN KEY(design_id) REFERENCES design (id) );
            CREATE TABLE connection ( description VARCHAR(512) NOT NULL, gap FLOAT NOT NULL, shift FLOAT NOT NULL, raise_ FLOAT NOT NULL, rotation FLOAT NOT NULL, turn FLOAT NOT NULL, tilt FLOAT NOT NULL, x FLOAT NOT NULL, y FLOAT NOT NULL, id INTEGER NOT NULL PRIMARY KEY, connected_piece_id INTEGER, connected_port_id INTEGER, connecting_piece_id INTEGER, connecting_port_id INTEGER, design_id INTEGER, CONSTRAINT "no reflexive connection" CHECK (connecting_piece_id != connected_piece_id), FOREIGN KEY(connected_piece_id) REFERENCES piece (id), FOREIGN KEY(connected_port_id) REFERENCES port (id), FOREIGN KEY(connecting_piece_id) REFERENCES piece (id), FOREIGN KEY(connecting_port_id) REFERENCES port (id), FOREIGN KEY(design_id) REFERENCES design (id) );
            CREATE TABLE quality ( name VARCHAR(64) NOT NULL, value VARCHAR(64) NOT NULL, unit VARCHAR(64) NOT NULL, definition VARCHAR(512) NOT NULL, id INTEGER NOT NULL PRIMARY KEY, representation_id INTEGER, port_id INTEGER, type_id INTEGER, piece_id INTEGER, connection_id INTEGER, design_id INTEGER, kit_id INTEGER, FOREIGN KEY(representation_id) REFERENCES representation (id), FOREIGN KEY(port_id) REFERENCES port (id), FOREIGN KEY(type_id) REFERENCES type (id), FOREIGN KEY(piece_id) REFERENCES piece (id), FOREIGN KEY(connection_id) REFERENCES connection (id), FOREIGN KEY(design_id) REFERENCES design (id), FOREIGN KEY(kit_id) REFERENCES kit (id) );
            CREATE TABLE author ( name VARCHAR(64) NOT NULL, email VARCHAR(128) NOT NULL, rank INTEGER NOT NULL, id INTEGER NOT NULL PRIMARY KEY, type_id INTEGER, design_id INTEGER, FOREIGN KEY(type_id) REFERENCES type (id), FOREIGN KEY(design_id) REFERENCES design (id) );
        `;

        try {
            db.run(schema); // Create tables

            // --- Helper function to insert Qualities ---
            const insertQualities = (qualities: Quality[] | undefined, fkColumn: string, fkValue: number) => {
                if (!qualities) return;
                // Use db directly
                const stmt = db.prepare(`INSERT INTO quality (name, value, unit, definition, ${fkColumn}) VALUES (?, ?, ?, ?, ?)`);
                qualities.forEach(q => stmt.run([q.name, q.value, q.unit, q.definition, fkValue]));
                stmt.free();
            };
            // --- Helper function to insert Authors ---
            const insertAuthors = (authors: Author[] | undefined, fkColumn: string, fkValue: number) => {
                if (!authors) return;
                // Use db directly
                const stmt = db.prepare(`INSERT INTO author (name, email, rank, ${fkColumn}) VALUES (?, ?, ?, ?)`);
                authors.forEach(a => stmt.run([a.name, a.email, a.rank, fkValue]));
                stmt.free();
            };

            // Insert Kit
            const kitStmt = db.prepare("INSERT INTO kit (uri, name, description, icon, image, preview, version, remote, homepage, license, created, updated) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)");
            kitStmt.run([kit.uri, kit.name, kit.description, kit.icon, kit.image, kit.preview, kit.version, kit.remote, kit.homepage, kit.license, kit.created.toISOString(), kit.updated.toISOString()]);
            kitStmt.free();
            const kitId = db.exec("SELECT last_insert_rowid()")[0].values[0][0];
            insertQualities(kit.qualities, 'kit_id', kitId);


            // Mappings for foreign keys
            const typeIdMap: { [key: string]: number } = {}; // key: "name:variant"
            const portIdMap: { [typeDbId: number]: { [localId: string]: number } } = {}; // port local_id to db id per type
            const repIdMap: { [typeDbId: number]: { [key: string]: number } } = {}; // key: "mime:tags"


            // Insert Types, Representations, Ports
            if (kit.types) {
                const typeStmt = db.prepare("INSERT INTO type (name, description, icon, image, variant, unit, created, updated, kit_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)");
                const repStmt = db.prepare("INSERT INTO representation (url, description, mime, type_id) VALUES (?, ?, ?, ?)");
                const tagStmt = db.prepare("INSERT INTO tag (name, \"order\", representation_id) VALUES (?, ?, ?)");
                const portStmt = db.prepare("INSERT INTO port (local_id, description, family, t, point_x, point_y, point_z, direction_x, direction_y, direction_z, type_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)");
                const compFamStmt = db.prepare("INSERT INTO compatible_family (name, \"order\", port_id) VALUES (?, ?, ?)");

                for (const type of kit.types) {
                    const typeKey = `${type.name}:${type.variant || ''}`;
                    typeStmt.run([type.name, type.description, type.icon, type.image, type.variant || '', type.unit, type.created.toISOString(), type.updated.toISOString(), kitId]);
                    const typeDbId = db.exec("SELECT last_insert_rowid()")[0].values[0][0] as number;
                    typeIdMap[typeKey] = typeDbId;
                    portIdMap[typeDbId] = {};
                    repIdMap[typeDbId] = {};
                    insertQualities(type.qualities, 'type_id', typeDbId);
                    insertAuthors(type.authors, 'type_id', typeDbId);


                    // Insert Representations and Tags
                    if (type.representations) {
                        for (const rep of type.representations) {
                            const repKey = `${rep.mime}:${rep.tags?.join(',') || ''}`;
                            repStmt.run([rep.url, rep.description, rep.mime, typeDbId]);
                            const repDbId = db.exec("SELECT last_insert_rowid()")[0].values[0][0] as number;
                            repIdMap[typeDbId][repKey] = repDbId;
                            insertQualities(rep.qualities, 'representation_id', repDbId);

                            if (rep.tags) {
                                rep.tags.forEach((tag, index) => tagStmt.run([tag, index, repDbId]));
                            }
                            // Add file to zip
                            const fileData = filesMap.get(rep.url) as Uint8Array | undefined; // Explicit cast
                            if (fileData) {
                                zip.file(rep.url, fileData);
                            } else if (!complete && !rep.url.startsWith("http")) {
                                console.warn(`File data for representation ${rep.url} not found in store.`);
                            } else if (complete && !rep.url.startsWith("http")) {
                                // If complete export requested and file missing locally, try fetching
                                try {
                                    const fetchedData = await fetch(rep.url).then(res => res.arrayBuffer());
                                    zip.file(rep.url, fetchedData);
                                } catch (fetchErr) {
                                    console.error(`Could not fetch representation ${rep.url} for complete export`, fetchErr);
                                }
                            }
                        }
                    }

                    // Insert Ports and Compatible Families
                    if (type.ports) {
                        for (const port of type.ports) {
                            if (!port.id_) {
                                console.warn(`Skipping port without local_id in type ${type.name}:${type.variant}`);
                                continue;
                            }
                            portStmt.run([port.id_, port.description, port.family, port.t, port.point.x, port.point.y, port.point.z, port.direction.x, port.direction.y, port.direction.z, typeDbId]);
                            const portDbId = db.exec("SELECT last_insert_rowid()")[0].values[0][0] as number;
                            portIdMap[typeDbId][port.id_] = portDbId;
                            insertQualities(port.qualities, 'port_id', portDbId);

                            if (port.compatibleFamilies) {
                                port.compatibleFamilies.forEach((fam, index) => compFamStmt.run([fam, index, portDbId]));
                            }
                        }
                    }
                }
                typeStmt.free();
                repStmt.free();
                tagStmt.free();
                portStmt.free();
                compFamStmt.free();
            }


            // Mappings for designs
            const designIdMap: { [key: string]: number } = {}; // key: "name:variant:view"
            const pieceIdMap: { [designDbId: number]: { [localId: string]: number } } = {}; // piece local_id to db id per design
            const planeIdMap: { [pieceDbId: number]: number } = {}; // map piece db id to plane db id
            let nextPlaneId = 1; // Simple counter for plane IDs


            // Insert Designs, Pieces, Connections
            if (kit.designs) {
                const designStmt = db.prepare("INSERT INTO design (name, description, icon, image, variant, \"view\", unit, created, updated, kit_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)");
                const planeStmt = db.prepare("INSERT INTO plane (id, origin_x, origin_y, origin_z, x_axis_x, x_axis_y, x_axis_z, y_axis_x, y_axis_y, y_axis_z) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)");
                const pieceStmt = db.prepare("INSERT INTO piece (local_id, description, type_id, plane_id, center_x, center_y, design_id) VALUES (?, ?, ?, ?, ?, ?, ?)");
                const connStmt = db.prepare("INSERT INTO connection (description, gap, shift, raise_, rotation, turn, tilt, x, y, connected_piece_id, connected_port_id, connecting_piece_id, connecting_port_id, design_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)");


                for (const design of kit.designs) {
                    const designKey = `${design.name}:${design.variant || ''}:${design.view || ''}`;
                    designStmt.run([design.name, design.description, design.icon, design.image, design.variant || '', design.view || '', design.unit, design.created.toISOString(), design.updated.toISOString(), kitId]);
                    const designDbId = db.exec("SELECT last_insert_rowid()")[0].values[0][0] as number;
                    designIdMap[designKey] = designDbId;
                    pieceIdMap[designDbId] = {};
                    insertQualities(design.qualities, 'design_id', designDbId);
                    insertAuthors(design.authors, 'design_id', designDbId);


                    // Insert Pieces and Planes
                    if (design.pieces) {
                        for (const piece of design.pieces) {
                            if (!piece.id_) {
                                console.warn(`Skipping piece without local_id in design ${design.name}:${design.variant}:${design.view}`);
                                continue;
                            }
                            const typeKey = `${piece.type.name}:${piece.type.variant || ''}`;
                            const typeDbId = typeIdMap[typeKey];
                            if (typeDbId === undefined) {
                                console.warn(`Could not find type DB ID for piece ${piece.id_} (type: ${typeKey})`);
                                continue;
                            }

                            let planeDbId: number | null = null;
                            if (piece.plane) {
                                planeDbId = nextPlaneId++;
                                planeStmt.run([planeDbId, piece.plane.origin.x, piece.plane.origin.y, piece.plane.origin.z, piece.plane.xAxis.x, piece.plane.xAxis.y, piece.plane.xAxis.z, piece.plane.yAxis.x, piece.plane.yAxis.y, piece.plane.yAxis.z]);
                            }

                            pieceStmt.run([piece.id_, piece.description, typeDbId, planeDbId, piece.center?.x, piece.center?.y, designDbId]);
                            const pieceDbId = db.exec("SELECT last_insert_rowid()")[0].values[0][0] as number;
                            pieceIdMap[designDbId][piece.id_] = pieceDbId;
                            insertQualities(piece.qualities, 'piece_id', pieceDbId);
                            if (planeDbId !== null) {
                                planeIdMap[pieceDbId] = planeDbId;
                            }
                        }
                    }

                    // Insert Connections
                    if (design.connections) {
                        for (const conn of design.connections) {
                            const connectedPieceDbId = pieceIdMap[designDbId][conn.connected.piece.id_];
                            const connectingPieceDbId = pieceIdMap[designDbId][conn.connecting.piece.id_];

                            const connectedPiece = design.pieces?.find(p => p.id_ === conn.connected.piece.id_);
                            const connectingPiece = design.pieces?.find(p => p.id_ === conn.connecting.piece.id_);

                            if (!connectedPieceDbId || !connectingPieceDbId || !connectedPiece || !connectingPiece) {
                                console.warn(`Could not find piece DB IDs for connection between ${conn.connected.piece.id_} and ${conn.connecting.piece.id_}`);
                                continue;
                            }

                            const connectedTypeKey = `${connectedPiece.type.name}:${connectedPiece.type.variant || ''}`;
                            const connectingTypeKey = `${connectingPiece.type.name}:${connectingPiece.type.variant || ''}`;
                            const connectedTypeDbId = typeIdMap[connectedTypeKey];
                            const connectingTypeDbId = typeIdMap[connectingTypeKey];

                            if (connectedTypeDbId === undefined || connectingTypeDbId === undefined) {
                                console.warn(`Could not find type DB IDs for connection pieces`);
                                continue;
                            }

                            const connectedPortDbId = conn.connected.port.id_ ? portIdMap[connectedTypeDbId]?.[conn.connected.port.id_] : undefined;
                            const connectingPortDbId = conn.connecting.port.id_ ? portIdMap[connectingTypeDbId]?.[conn.connecting.port.id_] : undefined;


                            if (connectedPortDbId === undefined || connectingPortDbId === undefined) {
                                console.warn(`Could not find port DB IDs for connection between ${conn.connected.piece.id_}:${conn.connected.port.id_} and ${conn.connecting.piece.id_}:${conn.connecting.port.id_}`);
                                continue;
                            }

                            connStmt.run([conn.description, conn.gap, conn.shift, conn.raise_, conn.rotation, conn.turn, conn.tilt, conn.x, conn.y, connectedPieceDbId, connectedPortDbId, connectingPieceDbId, connectingPortDbId, designDbId]);
                            const connDbId = db.exec("SELECT last_insert_rowid()")[0].values[0][0] as number;
                            insertQualities(conn.qualities, 'connection_id', connDbId);
                        }
                    }
                }
                designStmt.free();
                planeStmt.free();
                pieceStmt.free();
                connStmt.free();
            }

            // Export DB to Uint8Array
            const dbData = db.export();
            zip.file("semio/kit.db", dbData); // Adjusted path

            // Generate Zip Blob
            const zipBlob = await zip.generateAsync({ type: 'blob' });
            console.log(`Kit "${kit.name}" exported successfully.`);
            return zipBlob;

        } catch (error) {
            console.error("Error exporting kit:", error);
            throw error; // Re-throw after logging
        } finally {
            // Ensure database is closed for this specific export
            if (db) {
                db.close();
                console.log("SQL.js database closed for export.");
            }
        }
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

export interface DesignEditorSelection {
    selectedPieceIds: string[];
    selectedConnections: {
        connectingPieceId: string;
        connectedPieceId: string;
    }[];
}

export interface DesignEditorState {
    selection: DesignEditorSelection;
}

class DesignEditorStore {
    private id: string;
    private yDoc: Y.Doc;
    private yKit: Y.Map<any>;
    private yDesign: Y.Map<any>;
    private undoManager: UndoManager;
    private state: DesignEditorState;
    private listeners: Set<() => void> = new Set();

    constructor(id: string, yDoc: Y.Doc, yKit: Y.Map<any>, yDesign: Y.Map<any>) {
        this.id = id;
        this.yDoc = yDoc;
        this.yKit = yKit;
        this.yDesign = yDesign;
        this.undoManager = new UndoManager(yDesign, { captureTimeout: 0, trackedOrigins: new Set([id]) });
        // this.undoManager = new UndoManager(yDesign, { captureTimeout: 0 });
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
        this.listeners.forEach(listener => listener());
    }

    getDesignId(): [string, string, string] {
        return [this.yDesign.get('name'), this.yDesign.get('variant'), this.yDesign.get('view')];
    }

    getKitId(): string {
        return this.yKit.get('uri');
    }

    updateDesignEditorSelection = (selection: DesignEditorSelection): void => {
        this.setState({ ...this.getState(), selection });
    }

    undo(): void {
        this.undoManager.undo();
        this.listeners.forEach(listener => listener());
    }

    redo(): void {
        this.undoManager.redo();
        this.listeners.forEach(listener => listener());
    }

    transact(operations: () => void): void {
        this.yDoc.transact(operations, this.id);
        this.listeners.forEach(listener => listener());
    }

    subscribe(callback: () => void): () => void {
        this.listeners.add(callback);
        return () => {
            this.listeners.delete(callback);
        };
    }
}

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