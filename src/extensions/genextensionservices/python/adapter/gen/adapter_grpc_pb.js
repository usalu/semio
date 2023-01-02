// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var adapter_pb = require('./adapter_pb.js');
var model_pb = require('./model_pb.js');

function serialize_semio_extension_adapter_v1_AttractionPointRequest(arg) {
  if (!(arg instanceof adapter_pb.AttractionPointRequest)) {
    throw new Error('Expected argument of type semio.extension.adapter.v1.AttractionPointRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_semio_extension_adapter_v1_AttractionPointRequest(buffer_arg) {
  return adapter_pb.AttractionPointRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_semio_extension_adapter_v1_RepresentationRequest(arg) {
  if (!(arg instanceof adapter_pb.RepresentationRequest)) {
    throw new Error('Expected argument of type semio.extension.adapter.v1.RepresentationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_semio_extension_adapter_v1_RepresentationRequest(buffer_arg) {
  return adapter_pb.RepresentationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_semio_extension_adapter_v1_RepresentationsRequest(arg) {
  if (!(arg instanceof adapter_pb.RepresentationsRequest)) {
    throw new Error('Expected argument of type semio.extension.adapter.v1.RepresentationsRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_semio_extension_adapter_v1_RepresentationsRequest(buffer_arg) {
  return adapter_pb.RepresentationsRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_semio_model_v1_Point(arg) {
  if (!(arg instanceof model_pb.Point)) {
    throw new Error('Expected argument of type semio.model.v1.Point');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_semio_model_v1_Point(buffer_arg) {
  return model_pb.Point.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_semio_model_v1_Representation(arg) {
  if (!(arg instanceof model_pb.Representation)) {
    throw new Error('Expected argument of type semio.model.v1.Representation');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_semio_model_v1_Representation(buffer_arg) {
  return model_pb.Representation.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_semio_model_v1_Representations(arg) {
  if (!(arg instanceof model_pb.Representations)) {
    throw new Error('Expected argument of type semio.model.v1.Representations');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_semio_model_v1_Representations(buffer_arg) {
  return model_pb.Representations.deserializeBinary(new Uint8Array(buffer_arg));
}


// An adapter service is an adapter for elements to a specific platform where your elements are (parameterically) defined in.
var AdapterService = exports.AdapterService = {
  // Request an attraction point for the attracted.
requestAttractionPoint: {
    path: '/semio.extension.adapter.v1.Adapter/RequestAttractionPoint',
    requestStream: false,
    responseStream: false,
    requestType: adapter_pb.AttractionPointRequest,
    responseType: model_pb.Point,
    requestSerialize: serialize_semio_extension_adapter_v1_AttractionPointRequest,
    requestDeserialize: deserialize_semio_extension_adapter_v1_AttractionPointRequest,
    responseSerialize: serialize_semio_model_v1_Point,
    responseDeserialize: deserialize_semio_model_v1_Point,
  },
  // Request a specific representation
requestRepresentation: {
    path: '/semio.extension.adapter.v1.Adapter/RequestRepresentation',
    requestStream: false,
    responseStream: false,
    requestType: adapter_pb.RepresentationRequest,
    responseType: model_pb.Representation,
    requestSerialize: serialize_semio_extension_adapter_v1_RepresentationRequest,
    requestDeserialize: deserialize_semio_extension_adapter_v1_RepresentationRequest,
    responseSerialize: serialize_semio_model_v1_Representation,
    responseDeserialize: deserialize_semio_model_v1_Representation,
  },
  // Request potentially all representations
requestRepresentations: {
    path: '/semio.extension.adapter.v1.Adapter/RequestRepresentations',
    requestStream: false,
    responseStream: false,
    requestType: adapter_pb.RepresentationsRequest,
    responseType: model_pb.Representations,
    requestSerialize: serialize_semio_extension_adapter_v1_RepresentationsRequest,
    requestDeserialize: deserialize_semio_extension_adapter_v1_RepresentationsRequest,
    responseSerialize: serialize_semio_model_v1_Representations,
    responseDeserialize: deserialize_semio_model_v1_Representations,
  },
};

exports.AdapterClient = grpc.makeGenericClientConstructor(AdapterService);
