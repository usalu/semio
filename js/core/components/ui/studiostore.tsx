import { v4 as uuidv4 } from 'uuid';
import * as Y from 'yjs';
import React, { createContext, useContext, useEffect, useState } from 'react';
import { UndoManager } from 'yjs';
import { IndexeddbPersistence } from 'y-indexeddb';

import { Generator } from '@semio/js/lib/utils';
import { Kit, Port, Representation, Piece, Connection, Type, Design } from '@semio/js/semio';


export interface DesignEditorState {
    selection: {
        pieceUuids: string[];
        connectionUuids: string[];
    };
}


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

    private createYKit(kit: Kit): Y.Map<any> {
        const types = kit.types?.map(t => this.createYType(t));
        const designs = kit.designs?.map(d => this.createYDesign(d));
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
        return yKit
    }

    createKit(kit: Kit): void {
        this.createYKit(kit);
    }

    getKit(uri: string): Kit | null {
        const yKit = this.getYKit(uri);
        if (!yKit) return null;
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
            designs: Array.from(yKit.get('designs').values()).map((design: any) => (this.getDesign(uri, design.get('name'), design.get('variant'), design.get('view')))),
            types: Array.from(yKit.get('types').values()).map((type: any) => (this.getType(uri, type.get('name'), type.get('variant'))))
        };
    }

    private getYKit(uri: string): Y.Map<any> | undefined {
        return this.studioDoc.getMap('kits').get(uri) as Y.Map<any> | undefined;
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
        return yKit;
    }


    private createYType(type: Type): Y.Map<any> {
        const yType = new Y.Map<any>();
        yType.set('name', type.name);
        yType.set('variant', type.variant || '');
        yType.set('description', type.description || '');
        yType.set('icon', type.icon || '');
        yType.set('image', type.image || '');
        yType.set('unit', type.unit || '');
        yType.set('ports', new Y.Map(type.ports?.map(p => [p.id_ || uuidv4(), this.createYPort(p)])));
        yType.set('qualities', new Y.Map(type.qualities?.map(q => [q.name, q])));
        yType.set('representations', new Y.Map(type.representations?.map(r => [`${r.mime}:${r.tags?.join(',')}`, this.createYRepresentation(r)])));
        return yType;
    }

    createType(kitUri: string, type: Type): Type {
        const yKit = this.getYKit(kitUri);
        if (!yKit) throw new Error(`Kit ${kitUri} not found`);

        const types = yKit.get('types') as Y.Map<any>;
        let variantMap = types.get(type.name) as Y.Map<any> | undefined;
        if (!variantMap) {
            variantMap = new Y.Map<any>();
            types.set(type.name, variantMap);
        }
        variantMap.set(type.variant || '', this.createYType(type));

        return this.getType(kitUri, type.name, type.variant);
    }

    getType(kitUri: string, typeName: string, variant: string = ''): Type | null {
        const yKit = this.getYKit(kitUri);
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
        const yKit = this.getYKit(kitUri);
        if (!yKit) return null;

        const types = yKit.get('types');
        const yType = types.get(type.name)?.get(type.variant || '');
        if (!yType) return null;

        if (type.description !== undefined) yType.set('description', type.description);
        if (type.icon !== undefined) yType.set('icon', type.icon);
        if (type.image !== undefined) yType.set('image', type.image);
        if (type.unit !== undefined) yType.set('unit', type.unit);

        if (type.ports !== undefined) {
            const ports = new Y.Map(type.ports.map(p => [p.id_, this.createYPort(p)]));
            yType.set('ports', ports);
        }
        if (type.qualities !== undefined) {
            const qualities = new Y.Map(type.qualities.map(q => [q.name, q]));
            yType.set('qualities', qualities);
        }
        if (type.representations !== undefined) {
            const representations = new Y.Map(type.representations.map(r => [`${r.mime}:${r.tags?.join(',')}`, this.createYRepresentation(r)]));
            yType.set('representations', representations);
        }

        return this.getType(kitUri, type.name, type.variant);
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
        yDesign.set('pieces', new Y.Map(design.pieces?.map(p => [p.id_ || uuidv4(), this.createYPiece(p)])));
        yDesign.set('connections', new Y.Map(design.connections?.map(c => [this.getConnectionId(c), this.createYConnection(c)])));
        yDesign.set('qualities', new Y.Map(design.qualities?.map(q => [q.name, q])));
        return yDesign;
    }

    createDesign(kitUri: string, design: Design): Design {
        const yKit = this.getYKit(kitUri);
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
        viewMap.set(design.view || '', this.createYDesign(design));

        return this.getDesign(kitUri, design.name, design.variant, design.view);
    }

    getDesign(kitUri: string, name: string, variant: string = '', view: string = ''): Design | null {
        const yKit = this.getYKit(kitUri);
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
        const yKit = this.getYKit(kitUri);
        if (!yKit) return null;

        const designs = yKit.get('designs');
        const yDesign = designs.get(design.name)?.get(design.variant || '')?.get(design.view || '');
        if (!yDesign) return null;

        if (design.description !== undefined) yDesign.set('description', design.description);
        if (design.icon !== undefined) yDesign.set('icon', design.icon);
        if (design.image !== undefined) yDesign.set('image', design.image);
        if (design.unit !== undefined) yDesign.set('unit', design.unit);

        if (design.pieces !== undefined) {
            const pieces = new Y.Map(design.pieces.map(p => [p.id_, this.createYPiece(p)]));
            yDesign.set('pieces', pieces);
        }
        if (design.connections !== undefined) {
            const connections = new Y.Map(design.connections.map(c => [this.getConnectionId(c), this.createYConnection(c)]));
            yDesign.set('connections', connections);
        }
        if (design.qualities !== undefined) {
            const qualities = new Y.Map(design.qualities.map(q => [q.name, q]));
            yDesign.set('qualities', qualities);
        }

        return this.getDesign(kitUri, design.name, design.variant, design.view);
    }

    deleteDesign(kitUri: string, name: string): boolean {
        const yKit = this.getYKit(kitUri);
        if (!yKit) return false;

        const designs = yKit.get('designs') as Y.Map<any>;
        return designs.delete(name);
    }

    private createYPiece(piece: Piece): Piece {
        return {
            id_: piece.id_ || uuidv4(),
            description: piece.description || '',
            type: piece.type,
            center: piece.center || null,
            plane: piece.plane ? this.createYPlane(piece.plane) : null
        };
    }

    private createYPlane(plane: Plane): Y.Map<any> {
        const yPlane = new Y.Map<any>();
        yPlane.set('origin', new Y.Map(plane.origin));
        yPlane.set('xAxis', new Y.Map(plane.xAxis));
        yPlane.set('yAxis', new Y.Map(plane.yAxis));
        return yPlane;
    }

    createPiece(kitUri: string, name: string, variant: string, view: string, piece: Piece): Piece {
        const yKit = this.getYKit(kitUri);
        if (!yKit) throw new Error(`Kit ${kitUri} not found`);

        const designs = yKit.get('designs');
        const yDesign = designs.get(name)?.get(variant)?.get(view);
        if (!yDesign) throw new Error(`Design ${name} not found in kit ${kitUri}`);

        const pieces = yDesign.get('pieces');
        const yPiece = this.createYPiece(piece);
        pieces.set(yPiece.id_, yPiece);

        return this.getPiece(kitUri, name, variant, view, yPiece.id_);
    }

    getPiece(kitUri: string, name: string, variant: string, view: string, pieceId: string): Piece | null {
        const yKit = this.getYKit(kitUri);
        if (!yKit) return null;

        const designs = yKit.get('designs');
        const yDesign = designs.get(name)?.get(variant)?.get(view);
        if (!yDesign) return null;

        const pieces = yDesign.get('pieces');
        const yPiece = pieces.get(pieceId);
        if (!yPiece) return null;

        return yPiece;
    }

    updatePiece(kitUri: string, name: string, variant: string, view: string, piece: Piece): Piece | null {
        const yKit = this.getYKit(kitUri);
        if (!yKit) return null;

        const designs = yKit.get('designs');
        const yDesign = designs.get(name)?.get(variant)?.get(view);
        if (!yDesign) return null;

        const pieces = yDesign.get('pieces');
        const yPiece = pieces.get(piece.id_);
        if (!yPiece) return null;

        if (piece.description !== undefined) yPiece.description = piece.description;
        if (piece.type !== undefined) yPiece.type = piece.type;
        if (piece.center !== undefined) yPiece.center = piece.center;
        if (piece.plane !== undefined) yPiece.plane = piece.plane;

        return this.getPiece(kitUri, name, variant, view, piece.id_);
    }

    deletePiece(kitUri: string, name: string, variant: string, view: string, pieceId: string): boolean {
        const yKit = this.getYKit(kitUri);
        if (!yKit) return false;

        const designs = yKit.get('designs');
        const yDesign = designs.get(name)?.get(variant)?.get(view);
        if (!yDesign) return false;

        const pieces = yDesign.get('pieces');
        return pieces.delete(pieceId);
    }

    private createYConnection(connection: Connection): Connection {
        return {
            description: connection.description || '',
            connected: connection.connected,
            connecting: connection.connecting,
            gap: connection.gap || 0,
            rotation: connection.rotation || 0,
            shift: connection.shift || 0,
            tilt: connection.tilt || 0,
            x: connection.x || 0,
            y: connection.y || 0
        };
    }

    createConnection(kitUri: string, name: string, variant: string, view: string, connection: Connection): Connection {
        const yKit = this.getYKit(kitUri);
        if (!yKit) throw new Error(`Kit ${kitUri} not found`);

        const designs = yKit.get('designs');
        const yDesign = designs.get(name)?.get(variant)?.get(view);
        if (!yDesign) throw new Error(`Design ${name} not found in kit ${kitUri}`);

        const connections = yDesign.get('connections');
        const yConnection = this.createYConnection(connection);
        const connectionId = this.getConnectionId(connection);
        connections.set(connectionId, yConnection);

        return this.getConnection(kitUri, name, variant, view, connectionId);
    }

    getConnection(kitUri: string, name: string, variant: string, view: string, connectionId: string): Connection | null {
        const yKit = this.getYKit(kitUri);
        if (!yKit) return null;

        const designs = yKit.get('designs');
        const yDesign = designs.get(name)?.get(variant)?.get(view);
        if (!yDesign) return null;

        const connections = yDesign.get('connections');
        const yConnection = connections.get(connectionId);
        if (!yConnection) return null;

        return yConnection;
    }

    updateConnection(kitUri: string, name: string, variant: string, view: string, connectionId: string, connection: Partial<Connection>): Connection | null {
        const yKit = this.getYKit(kitUri);
        if (!yKit) return null;

        const designs = yKit.get('designs');
        const yDesign = designs.get(name)?.get(variant)?.get(view);
        if (!yDesign) return null;

        const connections = yDesign.get('connections');
        const yConnection = connections.get(connectionId);
        if (!yConnection) return null;

        if (connection.description !== undefined) yConnection.description = connection.description;
        if (connection.connected !== undefined) yConnection.connected = connection.connected;
        if (connection.connecting !== undefined) yConnection.connecting = connection.connecting;
        if (connection.gap !== undefined) yConnection.gap = connection.gap;
        if (connection.rotation !== undefined) yConnection.rotation = connection.rotation;
        if (connection.shift !== undefined) yConnection.shift = connection.shift;
        if (connection.tilt !== undefined) yConnection.tilt = connection.tilt;
        if (connection.x !== undefined) yConnection.x = connection.x;
        if (connection.y !== undefined) yConnection.y = connection.y;

        return this.getConnection(kitUri, name, variant, view, connectionId);
    }

    deleteConnection(kitUri: string, name: string, variant: string, view: string, connectionId: string): boolean {
        const yKit = this.getYKit(kitUri);
        if (!yKit) return false;

        const designs = yKit.get('designs');
        const yDesign = designs.get(name)?.get(variant)?.get(view);
        if (!yDesign) return false;

        const connections = yDesign.get('connections');
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

    private getConnectionId(connection: Connection): string {
        const { connecting, connected } = connection;
        return `${connecting.piece.id_}:${connecting.port.id_}-${connected.piece.id_}:${connected.port.id_}`;
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

export function useKit(kitUri: string) {
    const studio = useStudio();
}

const KitContext = createContext<Kit | null>(null);

export const KitProvider: React.FC<{ kit: Kit, children: React.ReactNode }> = ({ kit, children }) => {
    return (
        <KitContext.Provider value={kit}>
            {children}
        </KitContext.Provider>
    );
};

const DesignContext = createContext<Design | null>(null);

export const DesignProvider: React.FC<{ design: Design, children: React.ReactNode }> = ({ design, children }) => {
    return (
        <DesignContext.Provider value={design}>
            {children}
        </DesignContext.Provider>
    );
};

const PieceContext = createContext<Piece | null>(null);

export const PieceProvider: React.FC<{ piece: Piece, children: React.ReactNode }> = ({ piece, children }) => {
    return (
        <PieceContext.Provider value={piece}>
            {children}
        </PieceContext.Provider>
    );
};

export function useDesign(name: string, variant: string = '', view: string = '') {
    const kit = useContext(KitContext);
    if (!kit) throw new Error('useDesign must be used within a KitProvider');

    const [design, setDesign] = useState<Design | null>(null);

    useEffect(() => {
        const yDesign = kit.designs.get(name)?.get(variant)?.get(view);
        if (yDesign) {
            setDesign({
                name: yDesign.get('name'),
                variant: yDesign.get('variant'),
                view: yDesign.get('view'),
                description: yDesign.get('description'),
                icon: yDesign.get('icon'),
                image: yDesign.get('image'),
                unit: yDesign.get('unit'),
                pieces: Array.from(yDesign.get('pieces').entries()).map(([id, piece]) => ({ ...piece, id_: id })),
                connections: Array.from(yDesign.get('connections').values()),
                qualities: Array.from(yDesign.get('qualities').values())
            });
        }
    }, [kit, name, variant, view]);

    return design;
}

export function usePiece(id: string) {
    const design = useContext(DesignContext);
    if (!design) throw new Error('usePiece must be used within a DesignProvider');

    const [piece, setPiece] = useState<Piece | null>(null);

    useEffect(() => {
        const yPiece = design.pieces.get(id);
        if (yPiece) {
            setPiece({
                id_: yPiece.get('id_'),
                description: yPiece.get('description'),
                type: yPiece.get('type'),
                center: yPiece.get('center')?.toJSON() || null,
                plane: yPiece.get('plane')?.toJSON() || null
            });
        }
    }, [design, id]);

    return piece;
}