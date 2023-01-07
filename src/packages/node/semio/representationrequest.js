// source: extension/adapter/v1/adapter.proto
/**
 * @fileoverview
 * @enhanceable
 * @suppress {missingRequire} reports error on implicit type usages.
 * @suppress {messageConventions} JS Compiler reports an error if a variable or
 *     field starts with 'MSG_' and isn't a translatable message.
 * @public
 */
// GENERATED CODE -- DO NOT EDIT!
/* eslint-disable */
// @ts-nocheck

goog.provide('proto.semio.extension.adapter.v1.RepresentationRequest');

goog.require('jspb.BinaryReader');
goog.require('jspb.BinaryWriter');
goog.require('jspb.Message');
goog.require('proto.semio.model.v1.Sobject');

/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.semio.extension.adapter.v1.RepresentationRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.semio.extension.adapter.v1.RepresentationRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.semio.extension.adapter.v1.RepresentationRequest.displayName = 'proto.semio.extension.adapter.v1.RepresentationRequest';
}



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.semio.extension.adapter.v1.RepresentationRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.semio.extension.adapter.v1.RepresentationRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.semio.extension.adapter.v1.RepresentationRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.semio.extension.adapter.v1.RepresentationRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    sobject: (f = msg.getSobject()) && proto.semio.model.v1.Sobject.toObject(includeInstance, f),
    type: jspb.Message.getFieldWithDefault(msg, 2, ""),
    name: jspb.Message.getFieldWithDefault(msg, 3, ""),
    lod: jspb.Message.getFieldWithDefault(msg, 4, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.semio.extension.adapter.v1.RepresentationRequest}
 */
proto.semio.extension.adapter.v1.RepresentationRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.semio.extension.adapter.v1.RepresentationRequest;
  return proto.semio.extension.adapter.v1.RepresentationRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.semio.extension.adapter.v1.RepresentationRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.semio.extension.adapter.v1.RepresentationRequest}
 */
proto.semio.extension.adapter.v1.RepresentationRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.semio.model.v1.Sobject;
      reader.readMessage(value,proto.semio.model.v1.Sobject.deserializeBinaryFromReader);
      msg.setSobject(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setType(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setName(value);
      break;
    case 4:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setLod(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.semio.extension.adapter.v1.RepresentationRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.semio.extension.adapter.v1.RepresentationRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.semio.extension.adapter.v1.RepresentationRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.semio.extension.adapter.v1.RepresentationRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getSobject();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.semio.model.v1.Sobject.serializeBinaryToWriter
    );
  }
  f = message.getType();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getName();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getLod();
  if (f !== 0) {
    writer.writeInt64(
      4,
      f
    );
  }
};


/**
 * optional semio.model.v1.Sobject sobject = 1;
 * @return {?proto.semio.model.v1.Sobject}
 */
proto.semio.extension.adapter.v1.RepresentationRequest.prototype.getSobject = function() {
  return /** @type{?proto.semio.model.v1.Sobject} */ (
    jspb.Message.getWrapperField(this, proto.semio.model.v1.Sobject, 1));
};


/**
 * @param {?proto.semio.model.v1.Sobject|undefined} value
 * @return {!proto.semio.extension.adapter.v1.RepresentationRequest} returns this
*/
proto.semio.extension.adapter.v1.RepresentationRequest.prototype.setSobject = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.semio.extension.adapter.v1.RepresentationRequest} returns this
 */
proto.semio.extension.adapter.v1.RepresentationRequest.prototype.clearSobject = function() {
  return this.setSobject(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.semio.extension.adapter.v1.RepresentationRequest.prototype.hasSobject = function() {
  return jspb.Message.getField(this, 1) != null;
};


/**
 * optional string type = 2;
 * @return {string}
 */
proto.semio.extension.adapter.v1.RepresentationRequest.prototype.getType = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.semio.extension.adapter.v1.RepresentationRequest} returns this
 */
proto.semio.extension.adapter.v1.RepresentationRequest.prototype.setType = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string name = 3;
 * @return {string}
 */
proto.semio.extension.adapter.v1.RepresentationRequest.prototype.getName = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.semio.extension.adapter.v1.RepresentationRequest} returns this
 */
proto.semio.extension.adapter.v1.RepresentationRequest.prototype.setName = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional int64 lod = 4;
 * @return {number}
 */
proto.semio.extension.adapter.v1.RepresentationRequest.prototype.getLod = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 4, 0));
};


/**
 * @param {number} value
 * @return {!proto.semio.extension.adapter.v1.RepresentationRequest} returns this
 */
proto.semio.extension.adapter.v1.RepresentationRequest.prototype.setLod = function(value) {
  return jspb.Message.setProto3IntField(this, 4, value);
};

