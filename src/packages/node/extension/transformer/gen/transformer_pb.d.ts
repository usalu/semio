// package: semio.extension.transformer.v1
// file: transformer.proto

/* tslint:disable */
/* eslint-disable */

import * as jspb from "google-protobuf";
import * as model_pb from "./model_pb";

export class RewriteLayoutRequest extends jspb.Message { 
    clearDecisionsList(): void;
    getDecisionsList(): Array<model_pb.Decision>;
    setDecisionsList(value: Array<model_pb.Decision>): RewriteLayoutRequest;
    addDecisions(value?: model_pb.Decision, index?: number): model_pb.Decision;

    hasInitialLayout(): boolean;
    clearInitialLayout(): void;
    getInitialLayout(): model_pb.Layout | undefined;
    setInitialLayout(value?: model_pb.Layout): RewriteLayoutRequest;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): RewriteLayoutRequest.AsObject;
    static toObject(includeInstance: boolean, msg: RewriteLayoutRequest): RewriteLayoutRequest.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: RewriteLayoutRequest, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): RewriteLayoutRequest;
    static deserializeBinaryFromReader(message: RewriteLayoutRequest, reader: jspb.BinaryReader): RewriteLayoutRequest;
}

export namespace RewriteLayoutRequest {
    export type AsObject = {
        decisionsList: Array<model_pb.Decision.AsObject>,
        initialLayout?: model_pb.Layout.AsObject,
    }
}
