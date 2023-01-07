// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('grpc');
var model_v1_model_pb = require('../../model/v1/model_pb.js');
var extension_adapter_v1_adapter_pb = require('../../extension/adapter/v1/adapter_pb.js');
var extension_converter_v1_converter_pb = require('../../extension/converter/v1/converter_pb.js');
var extension_transformer_v1_transformer_pb = require('../../extension/transformer/v1/transformer_pb.js');

function serialize_semio_extension_adapter_v1_AttractionPointRequest(arg) {
  if (!(arg instanceof extension_adapter_v1_adapter_pb.AttractionPointRequest)) {
    throw new Error('Expected argument of type semio.extension.adapter.v1.AttractionPointRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_semio_extension_adapter_v1_AttractionPointRequest(buffer_arg) {
  return extension_adapter_v1_adapter_pb.AttractionPointRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_semio_extension_adapter_v1_RepresentationRequest(arg) {
  if (!(arg instanceof extension_adapter_v1_adapter_pb.RepresentationRequest)) {
    throw new Error('Expected argument of type semio.extension.adapter.v1.RepresentationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_semio_extension_adapter_v1_RepresentationRequest(buffer_arg) {
  return extension_adapter_v1_adapter_pb.RepresentationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_semio_model_v1_Point(arg) {
  if (!(arg instanceof model_v1_model_pb.Point)) {
    throw new Error('Expected argument of type semio.model.v1.Point');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_semio_model_v1_Point(buffer_arg) {
  return model_v1_model_pb.Point.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_semio_model_v1_Representation(arg) {
  if (!(arg instanceof model_v1_model_pb.Representation)) {
    throw new Error('Expected argument of type semio.model.v1.Representation');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_semio_model_v1_Representation(buffer_arg) {
  return model_v1_model_pb.Representation.deserializeBinary(new Uint8Array(buffer_arg));
}


// A manager service is responsible for calling extensions, storing/caching results while offering a cleaner interface to the server.
var ManagerServiceService = exports.ManagerServiceService = {
  requestRepresentation: {
    path: '/semio.server.v1.ManagerService/RequestRepresentation',
    requestStream: false,
    responseStream: false,
    requestType: extension_adapter_v1_adapter_pb.RepresentationRequest,
    responseType: model_v1_model_pb.Representation,
    requestSerialize: serialize_semio_extension_adapter_v1_RepresentationRequest,
    requestDeserialize: deserialize_semio_extension_adapter_v1_RepresentationRequest,
    responseSerialize: serialize_semio_model_v1_Representation,
    responseDeserialize: deserialize_semio_model_v1_Representation,
  },
  requestAttractionPoint: {
    path: '/semio.server.v1.ManagerService/RequestAttractionPoint',
    requestStream: false,
    responseStream: false,
    requestType: extension_adapter_v1_adapter_pb.AttractionPointRequest,
    responseType: model_v1_model_pb.Point,
    requestSerialize: serialize_semio_extension_adapter_v1_AttractionPointRequest,
    requestDeserialize: deserialize_semio_extension_adapter_v1_AttractionPointRequest,
    responseSerialize: serialize_semio_model_v1_Point,
    responseDeserialize: deserialize_semio_model_v1_Point,
  },
};

exports.ManagerServiceClient = grpc.makeGenericClientConstructor(ManagerServiceService);
