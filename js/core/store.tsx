import { v4 as uuidv4 } from 'uuid';
import * as Y from 'yjs';
import React, { createContext, useContext, useEffect, useState } from 'react';
import { UndoManager } from 'yjs';
import { IndexeddbPersistence } from 'y-indexeddb';

import { Generator } from '@semio/js/lib/utils';
import { Kit, Port, Representation, Piece, Connection, Type, Design, Plane, DiagramPoint } from '@semio/js/semio';


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

    createKit(kit: Kit): void {
        const types = kit.types?.map(t => this.createType(kit.uri, t));
        const designs = kit.designs?.map(d => this.createDesign(kit.uri, d));
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
        yKit.set('types', types);
        yKit.set('designs', designs);
        this.studioDoc.getMap('kits').set(kit.uri, yKit);
    }

    getKit(uri: string): Kit | undefined {
        const yKit = this.studioDoc.getMap('kits').get(uri) as Y.Map<any>;
        if (!yKit) return undefined;

        const types = Array.from((yKit.get('types') as Y.Map<any>).entries()).map(([_id, variantMap]) => {
            return Array.from((variantMap as Y.Map<any>).entries()).map(([_variantId, type]) => {
                const typeMap = type as Y.Map<any>;
                const ports = Array.from((typeMap.get('ports') as Y.Map<any>)?.entries() || []).map(([_portId, port]) => port);
                const qualities = Array.from((typeMap.get('qualities') as Y.Map<any>)?.entries() || []).map(([_qualityId, quality]) => quality);
                const representations = Array.from((typeMap.get('representations') as Y.Map<any>)?.entries() || []).map(([_repId, rep]) => rep);
                return {
                    name: typeMap.get('name') || '',
                    variant: typeMap.get('variant') || '',
                    description: typeMap.get('description') || '',
                    icon: typeMap.get('icon') || '',
                    image: typeMap.get('image') || '',
                    unit: typeMap.get('unit') || '',
                    ports,
                    qualities,
                    representations
                };
            });
        }).flat();

        const designs = Array.from((yKit.get('designs') as Y.Map<any>).entries()).map(([_id, variantMap]) => {
            return Array.from((variantMap as Y.Map<any>).entries()).map(([_variantId, viewMap]) => {
                return Array.from((viewMap as Y.Map<any>).entries()).map(([_viewId, design]) => {
                    const designMap = design as Y.Map<any>;
                    const pieces = Array.from((designMap.get('pieces') as Y.Map<any>)?.entries() || []).map(([_pieceId, piece]) => piece);
                    const connections = Array.from((designMap.get('connections') as Y.Map<any>)?.entries() || []).map(([_connId, conn]) => conn);
                    const qualities = Array.from((designMap.get('qualities') as Y.Map<any>)?.entries() || []).map(([_qualityId, quality]) => quality);
                    return {
                        name: designMap.get('name') || '',
                        variant: designMap.get('variant') || '',
                        view: designMap.get('view') || '',
                        description: designMap.get('description') || '',
                        icon: designMap.get('icon') || '',
                        image: designMap.get('image') || '',
                        unit: designMap.get('unit') || '',
                        pieces,
                        connections,
                        qualities
                    };
                });
            });
        }).flat(2);

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
            designs,
            types
        };
    }

    updateKit(kit: Kit): Kit {
        const yKit = this.studioDoc.getMap('kits').get(kit.uri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit ${kit.uri} not found`);
        if (kit.name !== "") yKit.set('name', kit.name);
        if (kit.description !== undefined && kit.description !== "") yKit.set('description', kit.description);
        if (kit.icon !== undefined && kit.icon !== "") yKit.set('icon', kit.icon);
        if (kit.image !== undefined && kit.image !== "") yKit.set('image', kit.image);
        if (kit.preview !== undefined && kit.preview !== "") yKit.set('preview', kit.preview);
        if (kit.version !== undefined && kit.version !== "") yKit.set('version', kit.version);
        if (kit.remote !== undefined && kit.remote !== "") yKit.set('remote', kit.remote);
        if (kit.homepage !== undefined && kit.homepage !== "") yKit.set('homepage', kit.homepage);
        if (kit.license !== undefined && kit.license !== "") yKit.set('license', kit.license);
        return this.getKit(kit.uri);
    }

    deleteKit(uri: string): void {
        this.studioDoc.getMap('kits').delete(uri);
    }

    createType(kitUri: string, type: Type): void {
        const yKit = this.studioDoc.getMap('kits').get(kitUri) as Y.Map<any>;
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
        yType.set('ports', new Y.Map(type.ports?.map(p => [p.id_ || uuidv4(), this.createPort(kitUri, type.name, p)])));
        yType.set('qualities', new Y.Map(type.qualities?.map(q => [q.name, q])));
        yType.set('representations', new Y.Map(type.representations?.map(r => [`${r.mime}:${r.tags?.join(',')}`, this.createRepresentation(kitUri, type.name, r)])));
        variantMap.set(type.variant || '', yType);
    }

    getType(kitUri: string, typeName: string, variant: string = ''): Type | null {
        const yKit = this.studioDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) return null;

        const types = yKit.get('types') as Y.Map<any>;
        const yType = types.get(typeName)?.get(variant) as Y.Map<any> | undefined;
        if (!yType) return null;

        return {
            name: yType.get('name'),
            description: yType.get('description'),
            icon: yType.get('icon'),
            image: yType.get('image'),
            variant: yType.get('variant'),
            unit: yType.get('unit'),
            ports: Array.from(yType.get('ports').entries()).map(([id, port]) => ({ ...port, id_: id })),
            qualities: Array.from(yType.get('qualities').values()),
            representations: Array.from(yType.get('representations').entries()).map(([key, value]: [string, any]): Representation => {
                const [mime, tags] = key.split(':');
                return {
                    ...value,
                    mime,
                    tags: tags ? tags.split(',') : []
                };
            })
        };
    }

    updateType(kitUri: string, type: Type): Type | null {
        const yKit = this.studioDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) return null;

        const types = yKit.get('types');
        const yType = types.get(type.name)?.get(type.variant || '');
        if (!yType) return null;

        if (type.description !== undefined) yType.set('description', type.description);
        if (type.icon !== undefined) yType.set('icon', type.icon);
        if (type.image !== undefined) yType.set('image', type.image);
        if (type.unit !== undefined) yType.set('unit', type.unit);

        if (type.ports !== undefined) {
            const ports = new Y.Map(type.ports.map(p => [p.id_, this.createPort(kitUri, type.name, p)]));
            yType.set('ports', ports);
        }
        if (type.qualities !== undefined) {
            const qualities = new Y.Map(type.qualities.map(q => [q.name, q]));
            yType.set('qualities', qualities);
        }
        if (type.representations !== undefined) {
            const representations = new Y.Map(type.representations.map(r => [`${r.mime}:${r.tags?.join(',')}`, this.createRepresentation(kitUri, type.name, r)]));
            yType.set('representations', representations);
        }

        return this.getType(kitUri, type.name, type.variant);
    }

    deleteType(kitUri: string, typeName: string): void {
        const yKit = this.studioDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit ${kitUri} not found`);
        const types = yKit.get('types') as Y.Map<any>;
        types.delete(typeName);
    }

    createDesign(kitUri: string, design: Design): void {
        const yKit = this.studioDoc.getMap('kits').get(kitUri) as Y.Map<any>;
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
        yDesign.set('pieces', new Y.Map(design.pieces?.map(p => [p.id_ || uuidv4(), this.createPiece(kitUri, p)])));
        yDesign.set('connections', new Y.Map(design.connections?.map(c => [this.getConnectionId(c), this.createConnection(kitUri, c)])));
        yDesign.set('qualities', new Y.Map(design.qualities?.map(q => [q.name, q])));
        viewMap.set(design.view || '', yDesign);
    }

    getDesign(kitUri: string, name: string, variant: string = '', view: string = ''): Design | null {
        const yKit = this.studioDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) return null;

        const designs = yKit.get('designs') as Y.Map<any>;
        const yDesign = designs.get(name)?.get(variant)?.get(view) as Y.Map<any> | undefined;
        if (!yDesign) return null;

        return {
            name: yDesign.get('name'),
            description: yDesign.get('description'),
            icon: yDesign.get('icon'),
            image: yDesign.get('image'),
            variant: yDesign.get('variant'),
            view: yDesign.get('view'),
            unit: yDesign.get('unit'),
            pieces: Array.from(yDesign.get('pieces').entries()).map(([id, piece]) => ({ ...piece, id_: id })),
            connections: Array.from(yDesign.get('connections').values()),
            qualities: Array.from(yDesign.get('qualities').values())
        };
    }

    updateDesign(kitUri: string, design: Design): Design | null {
        const yKit = this.studioDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) return null;

        const designs = yKit.get('designs');
        const yDesign = designs.get(design.name)?.get(design.variant || '')?.get(design.view || '');
        if (!yDesign) return null;

        if (design.description !== undefined) yDesign.set('description', design.description);
        if (design.icon !== undefined) yDesign.set('icon', design.icon);
        if (design.image !== undefined) yDesign.set('image', design.image);
        if (design.unit !== undefined) yDesign.set('unit', design.unit);

        if (design.pieces !== undefined) {
            const pieces = new Y.Map(design.pieces.map(p => [p.id_, this.createPiece(kitUri, design.name, design.variant, design.view, p)]));
            yDesign.set('pieces', pieces);
        }
        if (design.connections !== undefined) {
            const connections = new Y.Map(design.connections.map(c => [this.getConnectionId(c), this.createConnection(kitUri, design.name, design.variant, design.view, c)]));
            yDesign.set('connections', connections);
        }
        if (design.qualities !== undefined) {
            const qualities = new Y.Map(design.qualities.map(q => [q.name, q]));
            yDesign.set('qualities', qualities);
        }

        return this.getDesign(kitUri, design.name, design.variant, design.view);
    }

    deleteDesign(kitUri: string, name: string): void {
        const yKit = this.studioDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit ${kitUri} not found`);

        const designs = yKit.get('designs') as Y.Map<any>;
        designs.delete(name);
    }

    createPiece(kitUri: string, designName: string, designVariant: string, view: string, piece: Piece): void {
        const yKit = this.studioDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit ${kitUri} not found`);
        const yDesigns = yKit.get('designs');
        const yDesign = yDesigns.get(designName)?.get(designVariant)?.get(view);
        if (!yDesign) throw new Error(`Design ${designName} not found in kit ${kitUri}`);
        const yPieces = yDesign.get('pieces');

        const yPiece = new Y.Map<any>();
        yPiece.set('id_', piece.id_ || Generator.randomId());
        yPiece.set('description', piece.description || '');
        const yType = new Y.Map<any>();
        yType.set('designName', piece.type.designName);
        yType.set('designVariant', piece.type.designVariant);
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
            yCenter.set('z', piece.center.z);
            yPiece.set('center', yCenter);
        }
        yPieces.set(yPiece.get('id_'), yPiece);
    }

    getPiece(kitUri: string, designName: string, designVariant: string, view: string, pieceId: string): Piece | null {
        const yKit = this.studioDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) return null;

        const designs = yKit.get('designs');
        const yDesign = designs.get(designName)?.get(designVariant)?.get(view);
        if (!yDesign) return null;

        const pieces = yDesign.get('pieces');
        const yPiece = pieces.get(pieceId);
        if (!yPiece) return null;

        const yPlane = yPiece.get('plane')
        const yOrigin = yPlane?.get('origin')
        const yXAxis = yPlane?.get('xAxis')
        const yYAxis = yPlane?.get('yAxis')
        const origin: DiagramPoint | null = yOrigin ? {
            x: yOrigin.get('x'),
            y: yOrigin.get('y'),
            z: yOrigin.get('z')
        } : null;
        const xAxis: DiagramPoint | null = yXAxis ? {
            x: yXAxis.get('x'),
            y: yXAxis.get('y'),
            z: yXAxis.get('z')
        } : null;
        const yAxis: DiagramPoint | null = yYAxis ? {
            x: yYAxis.get('x'),
            y: yYAxis.get('y'),
            z: yYAxis.get('z')
        } : null;
        const plane: Plane | null = origin && xAxis && yAxis ? {
            origin,
            xAxis,
            yAxis
        } : null;

        const yCenter = yPiece.get('center')
        const center: DiagramPoint | null = yCenter ? {
            x: yCenter.get('x'),
            y: yCenter.get('y'),
            z: yCenter.get('z')
        } : null;

        return {
            id_: yPiece.get('id_'),
            description: yPiece.get('description'),
            type: yPiece.get('type'),
            plane,
            center
        };
    }

    updatePiece(kitUri: string, designName: string, designVariant: string, view: string, piece: Piece): Piece | null {
        const yKit = this.studioDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit ${kitUri} not found`);
        const designs = yKit.get('designs');
        const yDesign = designs.get(designName)?.get(designVariant)?.get(view);
        if (!yDesign) throw new Error(`Design ${designName} not found in kit ${kitUri}`);
        const pieces = yDesign.get('pieces');
        const yPiece = pieces.get(piece.id_);
        if (!yPiece) throw new Error(`Piece ${piece.id_} not found in design ${designName} in kit ${kitUri}`);

        if (piece.description !== undefined) yPiece.set('description', piece.description);
        if (piece.type !== undefined) yPiece.set('type', piece.type);
        if (piece.center !== undefined && piece.center !== null) {
            const yCenter = new Y.Map<any>();
            yCenter.set('x', piece.center.x);
            yCenter.set('y', piece.center.y);
            yCenter.set('z', piece.center.z);
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
        const yKit = this.studioDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) return false;

        const designs = yKit.get('designs');
        const yDesign = designs.get(designName)?.get(designVariant)?.get(view);
        if (!yDesign) return false;

        const pieces = yDesign.get('pieces');
        return pieces.delete(pieceId);
    }

    createConnection(kitUri: string, designName: string, designVariant: string, view: string, connection: Connection): void {
        const yKit = this.studioDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit ${kitUri} not found`);

        const designs = yKit.get('designs');
        const yDesign = designs.get(designName)?.get(designVariant)?.get(view);
        if (!yDesign) throw new Error(`Design ${designName} not found in kit ${kitUri}`);

        const connections = yDesign.get('connections');
        const yConnection = new Y.Map<any>();
        yConnection.set('description', connection.description || '');
        yConnection.set('connected', connection.connected);
        yConnection.set('connecting', connection.connecting);
        yConnection.set('gap', connection.gap || 0);
        yConnection.set('rotation', connection.rotation || 0);
        yConnection.set('shift', connection.shift || 0);
        yConnection.set('tilt', connection.tilt || 0);
        yConnection.set('x', connection.x || 0);
        yConnection.set('y', connection.y || 0);

        const connectionId = `${connection.connecting.piece.id_}:${connection.connecting.port.id_}-${connection.connected.piece.id_}:${connection.connected.port.id_}`;
        connections.set(connectionId, yConnection);
    }

    getConnection(kitUri: string, designName: string, designVariant: string, view: string, connectionId: string): Connection | null {
        const yKit = this.studioDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) return null;

        const designs = yKit.get('designs');
        const yDesign = designs.get(designName)?.get(designVariant)?.get(view);
        if (!yDesign) return null;

        const connections = yDesign.get('connections');
        const yConnection = connections.get(connectionId);
        if (!yConnection) return null;

        return {
            description: yConnection.get('description'),
            connected: yConnection.get('connected'),
            connecting: yConnection.get('connecting'),
            gap: yConnection.get('gap'),
            rotation: yConnection.get('rotation'),
            shift: yConnection.get('shift'),
            tilt: yConnection.get('tilt'),
            x: yConnection.get('x'),
            y: yConnection.get('y')
        };
    }

    updateConnection(kitUri: string, designName: string, designVariant: string, view: string, connectionId: string, connection: Partial<Connection>): void {
        const yKit = this.studioDoc.getMap('kits').get(kitUri) as Y.Map<any>;
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
    }

    deleteConnection(kitUri: string, designName: string, designVariant: string, view: string, connectionId: string): void {
        const yKit = this.studioDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit ${kitUri} not found`);

        const designs = yKit.get('designs');
        const yDesign = designs.get(designName)?.get(designVariant)?.get(view);
        if (!yDesign) throw new Error(`Design ${designName} not found in kit ${kitUri}`);

        const connections = yDesign.get('connections');
        connections.delete(connectionId);
    }

    createRepresentation(kitUri: string, typeName: string, representation: Representation): void {
        const yKit = this.studioDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit ${kitUri} not found`);

        const types = yKit.get('types');
        const yType = types.get(typeName);
        if (!yType) throw new Error(`Type ${typeName} not found in kit ${kitUri}`);

        const representations = yType.get('representations');
        const yRepresentation = new Y.Map<any>();
        yRepresentation.set('url', representation.url);
        yRepresentation.set('description', representation.description || '');
        yRepresentation.set('mime', representation.mime);
        yRepresentation.set('tags', representation.tags || []);
        if (representation.qualities) {
            const yQualities = new Y.Map<any>();
            representation.qualities.forEach(q => yQualities.set(q.name, q));
            yRepresentation.set('qualities', yQualities);
        }

        const key = `${representation.mime}:${representation.tags?.join(',') || ''}`;
        representations.set(key, yRepresentation);
    }

    getRepresentation(kitUri: string, typeName: string, key: string): Representation | null {
        const yKit = this.studioDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) return null;

        const types = yKit.get('types');
        const yType = types.get(typeName);
        if (!yType) return null;

        const representations = yType.get('representations');
        const yRepresentation = representations.get(key);
        if (!yRepresentation) return null;

        const yQualities = yRepresentation.get('qualities');

        return {
            url: yRepresentation.get('url'),
            description: yRepresentation.get('description'),
            mime: yRepresentation.get('mime'),
            tags: yRepresentation.get('tags'),
            qualities: yQualities ? Array.from(yQualities.entries()).map(([name, q]) => ({ ...q })) : undefined
        };
    }

    updateRepresentation(kitUri: string, typeName: string, key: string, representation: Partial<Representation>): void {
        const yKit = this.studioDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit ${kitUri} not found`);

        const types = yKit.get('types');
        const yType = types.get(typeName);
        if (!yType) throw new Error(`Type ${typeName} not found in kit ${kitUri}`);

        const representations = yType.get('representations');
        const yRepresentation = representations.get(key);
        if (!yRepresentation) throw new Error(`Representation ${key} not found in type ${typeName}`);

        if (representation.description !== undefined) yRepresentation.set('description', representation.description);
        if (representation.mime !== undefined) yRepresentation.set('mime', representation.mime);
        if (representation.tags !== undefined) yRepresentation.set('tags', representation.tags);
        if (representation.qualities !== undefined) {
            const yQualities = new Y.Map<any>();
            representation.qualities.forEach(q => yQualities.set(q.name, q));
            yRepresentation.set('qualities', yQualities);
        }
    }

    deleteRepresentation(kitUri: string, typeName: string, key: string): void {
        const yKit = this.studioDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit ${kitUri} not found`);

        const types = yKit.get('types');
        const yType = types.get(typeName);
        if (!yType) throw new Error(`Type ${typeName} not found in kit ${kitUri}`);

        const representations = yType.get('representations');
        representations.delete(key);
    }

    createPort(kitUri: string, typeName: string, port: Port): void {
        const yKit = this.studioDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit ${kitUri} not found`);

        const types = yKit.get('types');
        const yType = types.get(typeName);
        if (!yType) throw new Error(`Type ${typeName} not found in kit ${kitUri}`);

        const ports = yType.get('ports');
        const yPort = new Y.Map<any>();
        yPort.set('id_', port.id_ || uuidv4());
        yPort.set('description', port.description || '');

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
        if (port.qualities) {
            const yQualities = new Y.Map<any>();
            port.qualities.forEach(q => {
                const yQuality = new Y.Map<any>();
                yQuality.set('name', q.name);
                if (q.value) yQuality.set('value', q.value);
                if (q.unit) yQuality.set('unit', q.unit);
                if (q.definition) yQuality.set('definition', q.definition);
                yQualities.set(q.name, yQuality);
            });
            yPort.set('qualities', yQualities);
        }

        ports.set(yPort.get('id_'), yPort);
    }

    getPort(kitUri: string, typeName: string, portId: string): Port | null {
        const yKit = this.studioDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) return null;

        const types = yKit.get('types');
        const yType = types.get(typeName);
        if (!yType) return null;

        const ports = yType.get('ports');
        const yPort = ports.get(portId);
        if (!yPort) return null;

        const yDirection = yPort.get('direction');
        const yPoint = yPort.get('point');
        const yQualities = yPort.get('qualities');

        return {
            id_: yPort.get('id_'),
            description: yPort.get('description'),
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
            qualities: yQualities ? Array.from(yQualities.entries()).map(([name, q]) => ({ ...q })) : undefined
        };
    }

    updatePort(kitUri: string, typeName: string, portId: string, port: Partial<Port>): void {
        const yKit = this.studioDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit ${kitUri} not found`);

        const types = yKit.get('types');
        const yType = types.get(typeName);
        if (!yType) throw new Error(`Type ${typeName} not found in kit ${kitUri}`);

        const ports = yType.get('ports');
        const yPort = ports.get(portId);
        if (!yPort) throw new Error(`Port ${portId} not found in type ${typeName}`);

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
            const yQualities = new Y.Map<any>();
            port.qualities.forEach(q => yQualities.set(q.name, q));
            yPort.set('qualities', yQualities);
        }
    }

    deletePort(kitUri: string, typeName: string, portId: string): void {
        const yKit = this.studioDoc.getMap('kits').get(kitUri) as Y.Map<any>;
        if (!yKit) throw new Error(`Kit ${kitUri} not found`);
        const types = yKit.get('types');
        const yType = types.get(typeName);
        if (!yType) throw new Error(`Type ${typeName} not found in kit ${kitUri}`);
        const ports = yType.get('ports');
        ports.delete(portId);
    }

    createDesignEditor(kitUri: string, designName: string, designVariant: string, view: string): void {
        const yDesignEditor = new Y.Map<any>();
        yDesignEditor.set('kitUri', kitUri);
        yDesignEditor.set('designName', designName);
        yDesignEditor.set('designVariant', designVariant);
        yDesignEditor.set('view', view);
    }

}

const studio = new Studio();
const StudioContext = createContext<Studio | null>(null);
export const StudioProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
    return (
        <StudioContext.Provider value={studio}>
            {children}
        </StudioContext.Provider>
    );
};

export function useStudio() {
    const studio = useContext(StudioContext);
    if (!studio) throw new Error('Studio not found');

    function createKit(kit: Kit) { return studio.createKit(kit); }
    function updateKit(kit: Kit) { return studio.updateKit(kit); }
    function deleteKit(uri: string) { return studio.deleteKit(uri); }

    return { studio, createKit, updateKit, deleteKit };
}

const KitContext = createContext<Kit | null>(null);
export const KitProvider: React.FC<{ kit: Kit, children: React.ReactNode }> = ({ kit, children }) => {
    return (
        <KitContext.Provider value={kit}>
            {children}
        </KitContext.Provider>
    );
};

export function useKit(uri?: string) {
    const studio = useStudio();
    const kitFromContext = useContext(KitContext);
    const [kit, setKit] = useState<Kit | null>(kitFromContext);

    useEffect(() => {
        if (kitFromContext) {
            setKit(kitFromContext);
        } else if (uri) {
            const updatedKit = studio.getKit(uri);
            setKit(updatedKit);
        }
    }, [studio, kitFromContext, uri]);

    function createType(type: Type) { return studio.createType(uri, type); }
    function updateType(type: Type) { return studio.updateType(uri, type); }
    function deleteType(typeName: string) { return studio.deleteType(uri, typeName); }

    function createDesign(design: Design) { return studio.createDesign(uri, design); }
    function updateDesign(design: Design) { return studio.updateDesign(uri, design); }
    function deleteDesign(name: string) { return studio.deleteDesign(uri, name); }

    return {
        kit,
        createDesign, updateDesign, deleteDesign,
        createType, updateType, deleteType
    };
}

const DesignContext = createContext<Design | null>(null);
export const DesignProvider: React.FC<{ design: Design, children: React.ReactNode }> = ({ design, children }) => {
    return (
        <DesignContext.Provider value={design}>
            {children}
        </DesignContext.Provider>
    );
};

export function useDesign(name?: string, variant?: string, view?: string) {
    const studio = useStudio();
    const kit = useKit();
    const designFromContext = useContext(DesignContext);
    const [design, setDesign] = useState<Design | null>(designFromContext);

    useEffect(() => {
        if (designFromContext) {
            setDesign(designFromContext);
        } else if (kit?.uri && name) {
            const updatedDesign = studio.getDesign(kit.uri, name, variant || '', view || '');
            setDesign(updatedDesign);
        }
    }, [studio, kit, designFromContext, name, variant, view]);

    function createPiece(piece: Piece) { return studio.createPiece(uri, designName, designVariant, view, piece); }
    function updatePiece(piece: Piece) { return studio.updatePiece(uri, designName, designVariant, view, piece); }
    function deletePiece(pieceId: string) { return studio.deletePiece(uri, designName, designVariant, view, pieceId); }

    function createConnection(connection: Connection) { return studio.createConnection(uri, designName, designVariant, view, connection); }
    function updateConnection(connectionId: string, connection: Partial<Connection>) { return studio.updateConnection(uri, designName, designVariant, view, connectionId, connection); }
    function deleteConnection(connectionId: string) { return studio.deleteConnection(uri, designName, designVariant, view, connectionId); }

    return {
        design,
        createPiece, updatePiece, deletePiece,
        createConnection, updateConnection, deleteConnection
    }
}

const TypeContext = createContext<Type | null>(null);
export const TypeProvider: React.FC<{ type: Type, children: React.ReactNode }> = ({ type, children }) => {
    return (
        <TypeContext.Provider value={type}>
            {children}
        </TypeContext.Provider>
    );
};

export function useType(name?: string, variant?: string) {
    const studio = useStudio();
    const kit = useKit();
    const typeFromContext = useContext(TypeContext);
    const [type, setType] = useState<Type | null>(typeFromContext);

    useEffect(() => {
        if (typeFromContext) {
            setType(typeFromContext);
        } else if (kit?.uri && name) {
            const updatedType = studio.getType(kit.uri, name, variant || '');
            setType(updatedType);
        }
    }, [studio, kit, typeFromContext, name, variant]);


    function createRepresentation(representation: Representation) { return studio.createRepresentation(kit.uri, name, representation); }
    function updateRepresentation(key: string, representation: Partial<Representation>) { return studio.updateRepresentation(kit.uri, name, key, representation); }
    function deleteRepresentation(key: string) { return studio.deleteRepresentation(kit.uri, name, key); }

    function createPort(port: Port) { return studio.createPort(kit.uri, name, port); }
    function updatePort(portId: string, port: Partial<Port>) { return studio.updatePort(kit.uri, name, portId, port); }
    function deletePort(portId: string) { return studio.deletePort(kit.uri, name, portId); }

    return {
        type,
        createRepresentation, updateRepresentation, deleteRepresentation,
        createPort, updatePort, deletePort
    };
}

const PieceContext = createContext<Piece | null>(null);
export const PieceProvider: React.FC<{ piece: Piece, children: React.ReactNode }> = ({ piece, children }) => {
    return (
        <PieceContext.Provider value={piece}>
            {children}
        </PieceContext.Provider>
    );
};

export function usePiece(id?: string) {
    const studio = useStudio();
    const kit = useKit();
    const design = useDesign();
    const pieceFromContext = useContext(PieceContext);
    const [piece, setPiece] = useState<Piece | null>(pieceFromContext);

    useEffect(() => {
        if (pieceFromContext) {
            setPiece(pieceFromContext);
        } else if (kit?.uri && design?.name && id) {
            const updatedPiece = studio.getPiece(
                kit.uri,
                design.name,
                design.variant || '',
                design.view || '',
                id
            );
            setPiece(updatedPiece);
        }
    }, [studio, kit, design, pieceFromContext, id]);

    return piece;
}

