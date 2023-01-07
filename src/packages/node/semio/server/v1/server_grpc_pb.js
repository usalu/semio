// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('grpc');
var server_v1_server_pb = require('../../server/v1/server_pb.js');
var model_v1_model_pb = require('../../model/v1/model_pb.js');

function serialize_semio_model_v1_Design(arg) {
  if (!(arg instanceof model_v1_model_pb.Design)) {
    throw new Error('Expected argument of type semio.model.v1.Design');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_semio_model_v1_Design(buffer_arg) {
  return model_v1_model_pb.Design.deserializeBinary(new Uint8Array(buffer_arg));
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


// The server service is the gateway for all other apis of semio.
var ServerServiceService = exports.ServerServiceService = {
  // Lay out a design from a layout and return a design.
layoutDesign: {
    path: '/semio.server.v1.ServerService/LayoutDesign',
    requestStream: false,
    responseStream: false,
    requestType: model_v1_model_pb.Layout,
    responseType: model_v1_model_pb.Design,
    requestSerialize: serialize_semio_model_v1_Layout,
    requestDeserialize: deserialize_semio_model_v1_Layout,
    responseSerialize: serialize_semio_model_v1_Design,
    responseDeserialize: deserialize_semio_model_v1_Design,
  },
  // option (google.api.http) = {
//   post: "v1/layout-design"
//   body: "*"
// };
};

exports.ServerServiceClient = grpc.makeGenericClientConstructor(ServerServiceService);
//   option (google.api.default_host) = "localhost:50000";
