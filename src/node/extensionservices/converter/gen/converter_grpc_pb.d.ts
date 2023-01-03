// package: semio.extension.converter.v1
// file: converter.proto

/* tslint:disable */
/* eslint-disable */

import * as grpc from "@grpc/grpc-js";
import * as converter_pb from "./converter_pb";
import * as model_pb from "./model_pb";
import * as google_protobuf_any_pb from "google-protobuf/google/protobuf/any_pb";

interface IConverterService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
    convertRepresentation: IConverterService_IConvertRepresentation;
}

interface IConverterService_IConvertRepresentation extends grpc.MethodDefinition<converter_pb.RepresentationConversionRequest, model_pb.Representation> {
    path: "/semio.extension.converter.v1.Converter/ConvertRepresentation";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<converter_pb.RepresentationConversionRequest>;
    requestDeserialize: grpc.deserialize<converter_pb.RepresentationConversionRequest>;
    responseSerialize: grpc.serialize<model_pb.Representation>;
    responseDeserialize: grpc.deserialize<model_pb.Representation>;
}

export const ConverterService: IConverterService;

export interface IConverterServer extends grpc.UntypedServiceImplementation {
    convertRepresentation: grpc.handleUnaryCall<converter_pb.RepresentationConversionRequest, model_pb.Representation>;
}

export interface IConverterClient {
    convertRepresentation(request: converter_pb.RepresentationConversionRequest, callback: (error: grpc.ServiceError | null, response: model_pb.Representation) => void): grpc.ClientUnaryCall;
    convertRepresentation(request: converter_pb.RepresentationConversionRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: model_pb.Representation) => void): grpc.ClientUnaryCall;
    convertRepresentation(request: converter_pb.RepresentationConversionRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: model_pb.Representation) => void): grpc.ClientUnaryCall;
}

export class ConverterClient extends grpc.Client implements IConverterClient {
    constructor(address: string, credentials: grpc.ChannelCredentials, options?: Partial<grpc.ClientOptions>);
    public convertRepresentation(request: converter_pb.RepresentationConversionRequest, callback: (error: grpc.ServiceError | null, response: model_pb.Representation) => void): grpc.ClientUnaryCall;
    public convertRepresentation(request: converter_pb.RepresentationConversionRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: model_pb.Representation) => void): grpc.ClientUnaryCall;
    public convertRepresentation(request: converter_pb.RepresentationConversionRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: model_pb.Representation) => void): grpc.ClientUnaryCall;
}
