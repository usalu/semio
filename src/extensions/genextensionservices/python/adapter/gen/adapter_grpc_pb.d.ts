// package: semio.extension.adapter.v1
// file: adapter.proto

/* tslint:disable */
/* eslint-disable */

import * as grpc from "@grpc/grpc-js";
import * as adapter_pb from "./adapter_pb";
import * as model_pb from "./model_pb";

interface IAdapterService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
    requestAttractionPoint: IAdapterService_IRequestAttractionPoint;
    requestRepresentation: IAdapterService_IRequestRepresentation;
    requestRepresentations: IAdapterService_IRequestRepresentations;
}

interface IAdapterService_IRequestAttractionPoint extends grpc.MethodDefinition<adapter_pb.AttractionPointRequest, model_pb.Point> {
    path: "/semio.extension.adapter.v1.Adapter/RequestAttractionPoint";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<adapter_pb.AttractionPointRequest>;
    requestDeserialize: grpc.deserialize<adapter_pb.AttractionPointRequest>;
    responseSerialize: grpc.serialize<model_pb.Point>;
    responseDeserialize: grpc.deserialize<model_pb.Point>;
}
interface IAdapterService_IRequestRepresentation extends grpc.MethodDefinition<adapter_pb.RepresentationRequest, model_pb.Representation> {
    path: "/semio.extension.adapter.v1.Adapter/RequestRepresentation";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<adapter_pb.RepresentationRequest>;
    requestDeserialize: grpc.deserialize<adapter_pb.RepresentationRequest>;
    responseSerialize: grpc.serialize<model_pb.Representation>;
    responseDeserialize: grpc.deserialize<model_pb.Representation>;
}
interface IAdapterService_IRequestRepresentations extends grpc.MethodDefinition<adapter_pb.RepresentationsRequest, model_pb.Representations> {
    path: "/semio.extension.adapter.v1.Adapter/RequestRepresentations";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<adapter_pb.RepresentationsRequest>;
    requestDeserialize: grpc.deserialize<adapter_pb.RepresentationsRequest>;
    responseSerialize: grpc.serialize<model_pb.Representations>;
    responseDeserialize: grpc.deserialize<model_pb.Representations>;
}

export const AdapterService: IAdapterService;

export interface IAdapterServer extends grpc.UntypedServiceImplementation {
    requestAttractionPoint: grpc.handleUnaryCall<adapter_pb.AttractionPointRequest, model_pb.Point>;
    requestRepresentation: grpc.handleUnaryCall<adapter_pb.RepresentationRequest, model_pb.Representation>;
    requestRepresentations: grpc.handleUnaryCall<adapter_pb.RepresentationsRequest, model_pb.Representations>;
}

export interface IAdapterClient {
    requestAttractionPoint(request: adapter_pb.AttractionPointRequest, callback: (error: grpc.ServiceError | null, response: model_pb.Point) => void): grpc.ClientUnaryCall;
    requestAttractionPoint(request: adapter_pb.AttractionPointRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: model_pb.Point) => void): grpc.ClientUnaryCall;
    requestAttractionPoint(request: adapter_pb.AttractionPointRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: model_pb.Point) => void): grpc.ClientUnaryCall;
    requestRepresentation(request: adapter_pb.RepresentationRequest, callback: (error: grpc.ServiceError | null, response: model_pb.Representation) => void): grpc.ClientUnaryCall;
    requestRepresentation(request: adapter_pb.RepresentationRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: model_pb.Representation) => void): grpc.ClientUnaryCall;
    requestRepresentation(request: adapter_pb.RepresentationRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: model_pb.Representation) => void): grpc.ClientUnaryCall;
    requestRepresentations(request: adapter_pb.RepresentationsRequest, callback: (error: grpc.ServiceError | null, response: model_pb.Representations) => void): grpc.ClientUnaryCall;
    requestRepresentations(request: adapter_pb.RepresentationsRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: model_pb.Representations) => void): grpc.ClientUnaryCall;
    requestRepresentations(request: adapter_pb.RepresentationsRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: model_pb.Representations) => void): grpc.ClientUnaryCall;
}

export class AdapterClient extends grpc.Client implements IAdapterClient {
    constructor(address: string, credentials: grpc.ChannelCredentials, options?: Partial<grpc.ClientOptions>);
    public requestAttractionPoint(request: adapter_pb.AttractionPointRequest, callback: (error: grpc.ServiceError | null, response: model_pb.Point) => void): grpc.ClientUnaryCall;
    public requestAttractionPoint(request: adapter_pb.AttractionPointRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: model_pb.Point) => void): grpc.ClientUnaryCall;
    public requestAttractionPoint(request: adapter_pb.AttractionPointRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: model_pb.Point) => void): grpc.ClientUnaryCall;
    public requestRepresentation(request: adapter_pb.RepresentationRequest, callback: (error: grpc.ServiceError | null, response: model_pb.Representation) => void): grpc.ClientUnaryCall;
    public requestRepresentation(request: adapter_pb.RepresentationRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: model_pb.Representation) => void): grpc.ClientUnaryCall;
    public requestRepresentation(request: adapter_pb.RepresentationRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: model_pb.Representation) => void): grpc.ClientUnaryCall;
    public requestRepresentations(request: adapter_pb.RepresentationsRequest, callback: (error: grpc.ServiceError | null, response: model_pb.Representations) => void): grpc.ClientUnaryCall;
    public requestRepresentations(request: adapter_pb.RepresentationsRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: model_pb.Representations) => void): grpc.ClientUnaryCall;
    public requestRepresentations(request: adapter_pb.RepresentationsRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: model_pb.Representations) => void): grpc.ClientUnaryCall;
}
