// package: semio.extension.converter.v1
// file: converter.proto

/* tslint:disable */
/* eslint-disable */

import * as jspb from "google-protobuf";
import * as model_pb from "./model_pb";
import * as google_protobuf_any_pb from "google-protobuf/google/protobuf/any_pb";

export class RepresentationConversionRequest extends jspb.Message { 

    hasRepresentation(): boolean;
    clearRepresentation(): void;
    getRepresentation(): model_pb.Representation | undefined;
    setRepresentation(value?: model_pb.Representation): RepresentationConversionRequest;
    getTargetType(): string;
    setTargetType(value: string): RepresentationConversionRequest;

    hasOptions(): boolean;
    clearOptions(): void;
    getOptions(): google_protobuf_any_pb.Any | undefined;
    setOptions(value?: google_protobuf_any_pb.Any): RepresentationConversionRequest;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): RepresentationConversionRequest.AsObject;
    static toObject(includeInstance: boolean, msg: RepresentationConversionRequest): RepresentationConversionRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: RepresentationConversionRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): RepresentationConversionRequest;
    static deserializeBinaryFromReader(message: RepresentationConversionRequest, reader: jspb.BinaryReader): RepresentationConversionRequest;
}

export namespace RepresentationConversionRequest {
    export type AsObject = {
        representation?: model_pb.Representation.AsObject,
        targetType: string,
        options?: google_protobuf_any_pb.Any.AsObject,
    }
}
