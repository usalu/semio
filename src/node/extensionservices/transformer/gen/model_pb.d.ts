// package: semio.model.v1
// file: model.proto

/* tslint:disable */
/* eslint-disable */

import * as jspb from "google-protobuf";
import * as google_protobuf_any_pb from "google-protobuf/google/protobuf/any_pb";

export class Point extends jspb.Message { 
    getX(): number;
    setX(value: number): Point;
    getY(): number;
    setY(value: number): Point;
    getZ(): number;
    setZ(value: number): Point;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Point.AsObject;
    static toObject(includeInstance: boolean, msg: Point): Point.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: Point, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Point;
    static deserializeBinaryFromReader(message: Point, reader: jspb.BinaryReader): Point;
}

export namespace Point {
    export type AsObject = {
        x: number,
        y: number,
        z: number,
    }
}

export class Quaternion extends jspb.Message { 
    getW(): number;
    setW(value: number): Quaternion;
    getX(): number;
    setX(value: number): Quaternion;
    getY(): number;
    setY(value: number): Quaternion;
    getZ(): number;
    setZ(value: number): Quaternion;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Quaternion.AsObject;
    static toObject(includeInstance: boolean, msg: Quaternion): Quaternion.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: Quaternion, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Quaternion;
    static deserializeBinaryFromReader(message: Quaternion, reader: jspb.BinaryReader): Quaternion;
}

export namespace Quaternion {
    export type AsObject = {
        w: number,
        x: number,
        y: number,
        z: number,
    }
}

export class Pose extends jspb.Message { 

    hasPointOfView(): boolean;
    clearPointOfView(): void;
    getPointOfView(): Point | undefined;
    setPointOfView(value?: Point): Pose;

    hasView(): boolean;
    clearView(): void;
    getView(): Quaternion | undefined;
    setView(value?: Quaternion): Pose;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Pose.AsObject;
    static toObject(includeInstance: boolean, msg: Pose): Pose.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: Pose, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Pose;
    static deserializeBinaryFromReader(message: Pose, reader: jspb.BinaryReader): Pose;
}

export namespace Pose {
    export type AsObject = {
        pointOfView?: Point.AsObject,
        view?: Quaternion.AsObject,
    }
}

export class Representation extends jspb.Message { 
    getType(): string;
    setType(value: string): Representation;

    hasBody(): boolean;
    clearBody(): void;
    getBody(): google_protobuf_any_pb.Any | undefined;
    setBody(value?: google_protobuf_any_pb.Any): Representation;
    getName(): string;
    setName(value: string): Representation;
    getLod(): number;
    setLod(value: number): Representation;

    hasMetadata(): boolean;
    clearMetadata(): void;
    getMetadata(): google_protobuf_any_pb.Any | undefined;
    setMetadata(value?: google_protobuf_any_pb.Any): Representation;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Representation.AsObject;
    static toObject(includeInstance: boolean, msg: Representation): Representation.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: Representation, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Representation;
    static deserializeBinaryFromReader(message: Representation, reader: jspb.BinaryReader): Representation;
}

export namespace Representation {
    export type AsObject = {
        type: string,
        body?: google_protobuf_any_pb.Any.AsObject,
        name: string,
        lod: number,
        metadata?: google_protobuf_any_pb.Any.AsObject,
    }
}

export class Representations extends jspb.Message { 
    clearRepresentationsList(): void;
    getRepresentationsList(): Array<Representation>;
    setRepresentationsList(value: Array<Representation>): Representations;
    addRepresentations(value?: Representation, index?: number): Representation;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Representations.AsObject;
    static toObject(includeInstance: boolean, msg: Representations): Representations.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: Representations, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Representations;
    static deserializeBinaryFromReader(message: Representations, reader: jspb.BinaryReader): Representations;
}

export namespace Representations {
    export type AsObject = {
        representationsList: Array<Representation.AsObject>,
    }
}

export class Sobject extends jspb.Message { 
    getId(): string;
    setId(value: string): Sobject;

    hasPose(): boolean;
    clearPose(): void;
    getPose(): Pose | undefined;
    setPose(value?: Pose): Sobject;

    getParametersMap(): jspb.Map<string, google_protobuf_any_pb.Any>;
    clearParametersMap(): void;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Sobject.AsObject;
    static toObject(includeInstance: boolean, msg: Sobject): Sobject.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: Sobject, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Sobject;
    static deserializeBinaryFromReader(message: Sobject, reader: jspb.BinaryReader): Sobject;
}

export namespace Sobject {
    export type AsObject = {
        id: string,
        pose?: Pose.AsObject,

        parametersMap: Array<[string, google_protobuf_any_pb.Any.AsObject]>,
    }
}

export class AttractionStragegy extends jspb.Message { 

    hasRepresentation(): boolean;
    clearRepresentation(): void;
    getRepresentation(): Representation | undefined;
    setRepresentation(value?: Representation): AttractionStragegy;
    getPort(): string;
    setPort(value: string): AttractionStragegy;

    getParametersMap(): jspb.Map<string, google_protobuf_any_pb.Any>;
    clearParametersMap(): void;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): AttractionStragegy.AsObject;
    static toObject(includeInstance: boolean, msg: AttractionStragegy): AttractionStragegy.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: AttractionStragegy, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): AttractionStragegy;
    static deserializeBinaryFromReader(message: AttractionStragegy, reader: jspb.BinaryReader): AttractionStragegy;
}

export namespace AttractionStragegy {
    export type AsObject = {
        representation?: Representation.AsObject,
        port: string,

        parametersMap: Array<[string, google_protobuf_any_pb.Any.AsObject]>,
    }
}

export class AttractionParticipant extends jspb.Message { 
    getSobjectId(): string;
    setSobjectId(value: string): AttractionParticipant;

    hasStrategy(): boolean;
    clearStrategy(): void;
    getStrategy(): AttractionStragegy | undefined;
    setStrategy(value?: AttractionStragegy): AttractionParticipant;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): AttractionParticipant.AsObject;
    static toObject(includeInstance: boolean, msg: AttractionParticipant): AttractionParticipant.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: AttractionParticipant, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): AttractionParticipant;
    static deserializeBinaryFromReader(message: AttractionParticipant, reader: jspb.BinaryReader): AttractionParticipant;
}

export namespace AttractionParticipant {
    export type AsObject = {
        sobjectId: string,
        strategy?: AttractionStragegy.AsObject,
    }
}

export class Attraction extends jspb.Message { 

    hasAttractor(): boolean;
    clearAttractor(): void;
    getAttractor(): AttractionParticipant | undefined;
    setAttractor(value?: AttractionParticipant): Attraction;

    hasAttracted(): boolean;
    clearAttracted(): void;
    getAttracted(): AttractionParticipant | undefined;
    setAttracted(value?: AttractionParticipant): Attraction;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Attraction.AsObject;
    static toObject(includeInstance: boolean, msg: Attraction): Attraction.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: Attraction, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Attraction;
    static deserializeBinaryFromReader(message: Attraction, reader: jspb.BinaryReader): Attraction;
}

export namespace Attraction {
    export type AsObject = {
        attractor?: AttractionParticipant.AsObject,
        attracted?: AttractionParticipant.AsObject,
    }
}

export class Layout extends jspb.Message { 
    clearSobjectsList(): void;
    getSobjectsList(): Array<Sobject>;
    setSobjectsList(value: Array<Sobject>): Layout;
    addSobjects(value?: Sobject, index?: number): Sobject;
    clearAttractionsList(): void;
    getAttractionsList(): Array<Attraction>;
    setAttractionsList(value: Array<Attraction>): Layout;
    addAttractions(value?: Attraction, index?: number): Attraction;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Layout.AsObject;
    static toObject(includeInstance: boolean, msg: Layout): Layout.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: Layout, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Layout;
    static deserializeBinaryFromReader(message: Layout, reader: jspb.BinaryReader): Layout;
}

export namespace Layout {
    export type AsObject = {
        sobjectsList: Array<Sobject.AsObject>,
        attractionsList: Array<Attraction.AsObject>,
    }
}

export class AttractionChain extends jspb.Message { 
    clearAttractionsList(): void;
    getAttractionsList(): Array<Attraction>;
    setAttractionsList(value: Array<Attraction>): AttractionChain;
    addAttractions(value?: Attraction, index?: number): Attraction;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): AttractionChain.AsObject;
    static toObject(includeInstance: boolean, msg: AttractionChain): AttractionChain.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: AttractionChain, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): AttractionChain;
    static deserializeBinaryFromReader(message: AttractionChain, reader: jspb.BinaryReader): AttractionChain;
}

export namespace AttractionChain {
    export type AsObject = {
        attractionsList: Array<Attraction.AsObject>,
    }
}

export class Choreography extends jspb.Message { 
    clearSolitarySobjectsList(): void;
    getSolitarySobjectsList(): Array<Sobject>;
    setSolitarySobjectsList(value: Array<Sobject>): Choreography;
    addSolitarySobjects(value?: Sobject, index?: number): Sobject;
    clearAttractionchainsList(): void;
    getAttractionchainsList(): Array<AttractionChain>;
    setAttractionchainsList(value: Array<AttractionChain>): Choreography;
    addAttractionchains(value?: AttractionChain, index?: number): AttractionChain;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Choreography.AsObject;
    static toObject(includeInstance: boolean, msg: Choreography): Choreography.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: Choreography, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Choreography;
    static deserializeBinaryFromReader(message: Choreography, reader: jspb.BinaryReader): Choreography;
}

export namespace Choreography {
    export type AsObject = {
        solitarySobjectsList: Array<Sobject.AsObject>,
        attractionchainsList: Array<AttractionChain.AsObject>,
    }
}

export class Element extends jspb.Message { 

    hasPose(): boolean;
    clearPose(): void;
    getPose(): Pose | undefined;
    setPose(value?: Pose): Element;

    hasRepresentations(): boolean;
    clearRepresentations(): void;
    getRepresentations(): Representations | undefined;
    setRepresentations(value?: Representations): Element;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Element.AsObject;
    static toObject(includeInstance: boolean, msg: Element): Element.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: Element, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Element;
    static deserializeBinaryFromReader(message: Element, reader: jspb.BinaryReader): Element;
}

export namespace Element {
    export type AsObject = {
        pose?: Pose.AsObject,
        representations?: Representations.AsObject,
    }
}

export class Design extends jspb.Message { 
    clearElementsList(): void;
    getElementsList(): Array<Element>;
    setElementsList(value: Array<Element>): Design;
    addElements(value?: Element, index?: number): Element;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Design.AsObject;
    static toObject(includeInstance: boolean, msg: Design): Design.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: Design, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Design;
    static deserializeBinaryFromReader(message: Design, reader: jspb.BinaryReader): Design;
}

export namespace Design {
    export type AsObject = {
        elementsList: Array<Element.AsObject>,
    }
}

export class LayoutModification extends jspb.Message { 

    hasContextLayout(): boolean;
    clearContextLayout(): void;
    getContextLayout(): Layout | undefined;
    setContextLayout(value?: Layout): LayoutModification;

    hasModifiedContextLayout(): boolean;
    clearModifiedContextLayout(): void;
    getModifiedContextLayout(): Layout | undefined;
    setModifiedContextLayout(value?: Layout): LayoutModification;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): LayoutModification.AsObject;
    static toObject(includeInstance: boolean, msg: LayoutModification): LayoutModification.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: LayoutModification, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): LayoutModification;
    static deserializeBinaryFromReader(message: LayoutModification, reader: jspb.BinaryReader): LayoutModification;
}

export namespace LayoutModification {
    export type AsObject = {
        contextLayout?: Layout.AsObject,
        modifiedContextLayout?: Layout.AsObject,
    }
}

export class LayoutModificationStrategy extends jspb.Message { 
    getMatchCount(): number;
    setMatchCount(value: number): LayoutModificationStrategy;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): LayoutModificationStrategy.AsObject;
    static toObject(includeInstance: boolean, msg: LayoutModificationStrategy): LayoutModificationStrategy.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: LayoutModificationStrategy, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): LayoutModificationStrategy;
    static deserializeBinaryFromReader(message: LayoutModificationStrategy, reader: jspb.BinaryReader): LayoutModificationStrategy;
}

export namespace LayoutModificationStrategy {
    export type AsObject = {
        matchCount: number,
    }
}

export class Decision extends jspb.Message { 

    hasModification(): boolean;
    clearModification(): void;
    getModification(): LayoutModification | undefined;
    setModification(value?: LayoutModification): Decision;

    hasStrategy(): boolean;
    clearStrategy(): void;
    getStrategy(): LayoutModificationStrategy | undefined;
    setStrategy(value?: LayoutModificationStrategy): Decision;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): Decision.AsObject;
    static toObject(includeInstance: boolean, msg: Decision): Decision.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: Decision, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): Decision;
    static deserializeBinaryFromReader(message: Decision, reader: jspb.BinaryReader): Decision;
}

export namespace Decision {
    export type AsObject = {
        modification?: LayoutModification.AsObject,
        strategy?: LayoutModificationStrategy.AsObject,
    }
}
