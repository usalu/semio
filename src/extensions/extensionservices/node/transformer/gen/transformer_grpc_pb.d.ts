// package: semio.extension.transformer.v1
// file: transformer.proto

/* tslint:disable */
/* eslint-disable */

import * as grpc from "@grpc/grpc-js";
import * as transformer_pb from "./transformer_pb";
import * as model_pb from "./model_pb";

interface ILayoutRewriterServiceService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
    rewriteLayout: ILayoutRewriterServiceService_IRewriteLayout;
}

interface ILayoutRewriterServiceService_IRewriteLayout extends grpc.MethodDefinition<transformer_pb.RewriteLayoutRequest, model_pb.Layout> {
    path: "/semio.extension.transformer.v1.LayoutRewriterService/RewriteLayout";
    requestStream: false;
    responseStream: false;
    requestSerialize: grpc.serialize<transformer_pb.RewriteLayoutRequest>;
    requestDeserialize: grpc.deserialize<transformer_pb.RewriteLayoutRequest>;
    responseSerialize: grpc.serialize<model_pb.Layout>;
    responseDeserialize: grpc.deserialize<model_pb.Layout>;
}

export const LayoutRewriterServiceService: ILayoutRewriterServiceService;

export interface ILayoutRewriterServiceServer extends grpc.UntypedServiceImplementation {
    rewriteLayout: grpc.handleUnaryCall<transformer_pb.RewriteLayoutRequest, model_pb.Layout>;
}

export interface ILayoutRewriterServiceClient {
    rewriteLayout(request: transformer_pb.RewriteLayoutRequest, callback: (error: grpc.ServiceError | null, response: model_pb.Layout) => void): grpc.ClientUnaryCall;
    rewriteLayout(request: transformer_pb.RewriteLayoutRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: model_pb.Layout) => void): grpc.ClientUnaryCall;
    rewriteLayout(request: transformer_pb.RewriteLayoutRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: model_pb.Layout) => void): grpc.ClientUnaryCall;
}

export class LayoutRewriterServiceClient extends grpc.Client implements ILayoutRewriterServiceClient {
    constructor(address: string, credentials: grpc.ChannelCredentials, options?: Partial<grpc.ClientOptions>);
    public rewriteLayout(request: transformer_pb.RewriteLayoutRequest, callback: (error: grpc.ServiceError | null, response: model_pb.Layout) => void): grpc.ClientUnaryCall;
    public rewriteLayout(request: transformer_pb.RewriteLayoutRequest, metadata: grpc.Metadata, callback: (error: grpc.ServiceError | null, response: model_pb.Layout) => void): grpc.ClientUnaryCall;
    public rewriteLayout(request: transformer_pb.RewriteLayoutRequest, metadata: grpc.Metadata, options: Partial<grpc.CallOptions>, callback: (error: grpc.ServiceError | null, response: model_pb.Layout) => void): grpc.ClientUnaryCall;
}
