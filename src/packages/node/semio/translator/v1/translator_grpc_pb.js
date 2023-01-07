// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('grpc');
var translator_v1_translator_pb = require('../../translator/v1/translator_pb.js');
var model_v1_model_pb = require('../../model/v1/model_pb.js');

function serialize_semio_server_v1_TranslateRepresentationRequest(arg) {
  if (!(arg instanceof translator_v1_translator_pb.TranslateRepresentationRequest)) {
    throw new Error('Expected argument of type semio.server.v1.TranslateRepresentationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_semio_server_v1_TranslateRepresentationRequest(buffer_arg) {
  return translator_v1_translator_pb.TranslateRepresentationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_semio_server_v1_TranslateRepresentationResponse(arg) {
  if (!(arg instanceof translator_v1_translator_pb.TranslateRepresentationResponse)) {
    throw new Error('Expected argument of type semio.server.v1.TranslateRepresentationResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_semio_server_v1_TranslateRepresentationResponse(buffer_arg) {
  return translator_v1_translator_pb.TranslateRepresentationResponse.deserializeBinary(new Uint8Array(buffer_arg));
}


// A translator service translates representations between different poses (coordinate systems).
var TranslatorServiceService = exports.TranslatorServiceService = {
  // Lay out a design from a layout and return a design.
translateRepresentation: {
    path: '/semio.server.v1.TranslatorService/TranslateRepresentation',
    requestStream: false,
    responseStream: false,
    requestType: translator_v1_translator_pb.TranslateRepresentationRequest,
    responseType: translator_v1_translator_pb.TranslateRepresentationResponse,
    requestSerialize: serialize_semio_server_v1_TranslateRepresentationRequest,
    requestDeserialize: deserialize_semio_server_v1_TranslateRepresentationRequest,
    responseSerialize: serialize_semio_server_v1_TranslateRepresentationResponse,
    responseDeserialize: deserialize_semio_server_v1_TranslateRepresentationResponse,
  },
};

exports.TranslatorServiceClient = grpc.makeGenericClientConstructor(TranslatorServiceService);
