// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var converter_pb = require('./converter_pb.js');
var model_pb = require('./model_pb.js');
var google_protobuf_any_pb = require('google-protobuf/google/protobuf/any_pb.js');

function serialize_semio_extension_converter_v1_RepresentationConversionRequest(arg) {
  if (!(arg instanceof converter_pb.RepresentationConversionRequest)) {
    throw new Error('Expected argument of type semio.extension.converter.v1.RepresentationConversionRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_semio_extension_converter_v1_RepresentationConversionRequest(buffer_arg) {
  return converter_pb.RepresentationConversionRequest.deserializeBinary(new Uint8Array(buffer_arg));
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


// A converter service can convert an element from one representation into another representation.
var ConverterService = exports.ConverterService = {
  convertRepresentation: {
    path: '/semio.extension.converter.v1.Converter/ConvertRepresentation',
    requestStream: false,
    responseStream: false,
    requestType: converter_pb.RepresentationConversionRequest,
    responseType: model_pb.Representation,
    requestSerialize: serialize_semio_extension_converter_v1_RepresentationConversionRequest,
    requestDeserialize: deserialize_semio_extension_converter_v1_RepresentationConversionRequest,
    responseSerialize: serialize_semio_model_v1_Representation,
    responseDeserialize: deserialize_semio_model_v1_Representation,
  },
};

exports.ConverterClient = grpc.makeGenericClientConstructor(ConverterService);
