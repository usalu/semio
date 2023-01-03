// package: semio.extension.adapter.v1
// file: adapter.proto

/* tslint:disable */
/* eslint-disable */

import * as jspb from "google-protobuf";
import * as model_pb from "./model_pb";

export class AttractionPointRequest extends jspb.Message { 
    getAttractorUrl(): string;
    setAttractorUrl(value: string): AttractionPointRequest;

    hasAttractedAttractionstrategy(): boolean;
    clearAttractedAttractionstrategy(): void;
    getAttractedAttractionstrategy(): model_pb.AttractionStragegy | undefined;
    setAttractedAttractionstrategy(value?: model_pb.AttractionStragegy): AttractionPointRequest;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): AttractionPointRequest.AsObject;
    static toObject(includeInstance: boolean, msg: AttractionPointRequest): AttractionPointRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: AttractionPointRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): AttractionPointRequest;
    static deserializeBinaryFromReader(message: AttractionPointRequest, reader: jspb.BinaryReader): AttractionPointRequest;
}

export namespace AttractionPointRequest {
    export type AsObject = {
        attractorUrl: string,
        attractedAttractionstrategy?: model_pb.AttractionStragegy.AsObject,
    }
}

export class RepresentationRequest extends jspb.Message { 

    hasSobject(): boolean;
    clearSobject(): void;
    getSobject(): model_pb.Sobject | undefined;
    setSobject(value?: model_pb.Sobject): RepresentationRequest;
    getType(): string;
    setType(value: string): RepresentationRequest;
    getName(): string;
    setName(value: string): RepresentationRequest;
    getLod(): number;
    setLod(value: number): RepresentationRequest;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): RepresentationRequest.AsObject;
    static toObject(includeInstance: boolean, msg: RepresentationRequest): RepresentationRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: RepresentationRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): RepresentationRequest;
    static deserializeBinaryFromReader(message: RepresentationRequest, reader: jspb.BinaryReader): RepresentationRequest;
}

export namespace RepresentationRequest {
    export type AsObject = {
        sobject?: model_pb.Sobject.AsObject,
        type: string,
        name: string,
        lod: number,
    }
}

export class RepresentationsRequest extends jspb.Message { 

    hasSobject(): boolean;
    clearSobject(): void;
    getSobject(): model_pb.Sobject | undefined;
    setSobject(value?: model_pb.Sobject): RepresentationsRequest;
    clearTypesList(): void;
    getTypesList(): Array<string>;
    setTypesList(value: Array<string>): RepresentationsRequest;
    addTypes(value: string, index?: number): string;
    clearNamesList(): void;
    getNamesList(): Array<string>;
    setNamesList(value: Array<string>): RepresentationsRequest;
    addNames(value: string, index?: number): string;
    clearLodsList(): void;
    getLodsList(): Array<number>;
    setLodsList(value: Array<number>): RepresentationsRequest;
    addLods(value: number, index?: number): number;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): RepresentationsRequest.AsObject;
    static toObject(includeInstance: boolean, msg: RepresentationsRequest): RepresentationsRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: RepresentationsRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): RepresentationsRequest;
    static deserializeBinaryFromReader(message: RepresentationsRequest, reader: jspb.BinaryReader): RepresentationsRequest;
}

export namespace RepresentationsRequest {
    export type AsObject = {
        sobject?: model_pb.Sobject.AsObject,
        typesList: Array<string>,
        namesList: Array<string>,
        lodsList: Array<number>,
    }
}
