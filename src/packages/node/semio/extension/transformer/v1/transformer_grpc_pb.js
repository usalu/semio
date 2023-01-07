// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('grpc');
var extension_transformer_v1_transformer_pb = require('../../../extension/transformer/v1/transformer_pb.js');
var model_v1_model_pb = require('../../../model/v1/model_pb.js');

function serialize_semio_extension_transformer_v1_RewriteLayoutRequest(arg) {
  if (!(arg instanceof extension_transformer_v1_transformer_pb.RewriteLayoutRequest)) {
    throw new Error('Expected argument of type semio.extension.transformer.v1.RewriteLayoutRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_semio_extension_transformer_v1_RewriteLayoutRequest(buffer_arg) {
  return extension_transformer_v1_transformer_pb.RewriteLayoutRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_semio_model_v1_Layout(arg) {
  if (!(arg instanceof model_v1_model_pb.Layout)) {
    throw new Error('Expected argument of type semio.model.v1.Layout');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_semio_model_v1_Layout(buffer_arg) {
  return model_v1_model_pb.Layout.deserializeBinary(new Uint8Array(buffer_arg));
}


// A service for transforming (rewriting) layouts (graphs).
var TransformerService = exports.TransformerService = {
  rewriteLayout: {
    path: '/semio.extension.transformer.v1.Transformer/RewriteLayout',
    requestStream: false,
    responseStream: false,
    requestType: extension_transformer_v1_transformer_pb.RewriteLayoutRequest,
    responseType: model_v1_model_pb.Layout,
    requestSerialize: serialize_semio_extension_transformer_v1_RewriteLayoutRequest,
    requestDeserialize: deserialize_semio_extension_transformer_v1_RewriteLayoutRequest,
    responseSerialize: serialize_semio_model_v1_Layout,
    responseDeserialize: deserialize_semio_model_v1_Layout,
  },
};

exports.TransformerClient = grpc.makeGenericClientConstructor(TransformerService);
