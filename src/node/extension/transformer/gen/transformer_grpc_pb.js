// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var transformer_pb = require('./transformer_pb.js');
var model_pb = require('./model_pb.js');

function serialize_semio_extension_transformer_v1_RewriteLayoutRequest(arg) {
  if (!(arg instanceof transformer_pb.RewriteLayoutRequest)) {
    throw new Error('Expected argument of type semio.extension.transformer.v1.RewriteLayoutRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_semio_extension_transformer_v1_RewriteLayoutRequest(buffer_arg) {
  return transformer_pb.RewriteLayoutRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_semio_model_v1_Layout(arg) {
  if (!(arg instanceof model_pb.Layout)) {
    throw new Error('Expected argument of type semio.model.v1.Layout');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_semio_model_v1_Layout(buffer_arg) {
  return model_pb.Layout.deserializeBinary(new Uint8Array(buffer_arg));
}


// A service for rewriting layouts (graphs).
var LayoutRewriterService = exports.LayoutRewriterService = {
  rewriteLayout: {
    path: '/semio.extension.transformer.v1.LayoutRewriter/RewriteLayout',
    requestStream: false,
    responseStream: false,
    requestType: transformer_pb.RewriteLayoutRequest,
    responseType: model_pb.Layout,
    requestSerialize: serialize_semio_extension_transformer_v1_RewriteLayoutRequest,
    requestDeserialize: deserialize_semio_extension_transformer_v1_RewriteLayoutRequest,
    responseSerialize: serialize_semio_model_v1_Layout,
    responseDeserialize: deserialize_semio_model_v1_Layout,
  },
};

exports.LayoutRewriterClient = grpc.makeGenericClientConstructor(LayoutRewriterService);
