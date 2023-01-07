// source: extension/transformer/v1/transformer.proto
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

goog.provide('proto.semio.extension.transformer.v1.RewriteLayoutRequest');

goog.require('jspb.BinaryReader');
goog.require('jspb.BinaryWriter');
goog.require('jspb.Message');
goog.require('proto.semio.model.v1.Decision');
goog.require('proto.semio.model.v1.Layout');

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
proto.semio.extension.transformer.v1.RewriteLayoutRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.semio.extension.transformer.v1.RewriteLayoutRequest.repeatedFields_, null);
};
goog.inherits(proto.semio.extension.transformer.v1.RewriteLayoutRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.semio.extension.transformer.v1.RewriteLayoutRequest.displayName = 'proto.semio.extension.transformer.v1.RewriteLayoutRequest';
}

/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.semio.extension.transformer.v1.RewriteLayoutRequest.repeatedFields_ = [1];



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
proto.semio.extension.transformer.v1.RewriteLayoutRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.semio.extension.transformer.v1.RewriteLayoutRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.semio.extension.transformer.v1.RewriteLayoutRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.semio.extension.transformer.v1.RewriteLayoutRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
    decisionsList: jspb.Message.toObjectList(msg.getDecisionsList(),
    proto.semio.model.v1.Decision.toObject, includeInstance),
    initialLayout: (f = msg.getInitialLayout()) && proto.semio.model.v1.Layout.toObject(includeInstance, f)
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
 * @return {!proto.semio.extension.transformer.v1.RewriteLayoutRequest}
 */
proto.semio.extension.transformer.v1.RewriteLayoutRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.semio.extension.transformer.v1.RewriteLayoutRequest;
  return proto.semio.extension.transformer.v1.RewriteLayoutRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.semio.extension.transformer.v1.RewriteLayoutRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.semio.extension.transformer.v1.RewriteLayoutRequest}
 */
proto.semio.extension.transformer.v1.RewriteLayoutRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.semio.model.v1.Decision;
      reader.readMessage(value,proto.semio.model.v1.Decision.deserializeBinaryFromReader);
      msg.addDecisions(value);
      break;
    case 2:
      var value = new proto.semio.model.v1.Layout;
      reader.readMessage(value,proto.semio.model.v1.Layout.deserializeBinaryFromReader);
      msg.setInitialLayout(value);
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
proto.semio.extension.transformer.v1.RewriteLayoutRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.semio.extension.transformer.v1.RewriteLayoutRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.semio.extension.transformer.v1.RewriteLayoutRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.semio.extension.transformer.v1.RewriteLayoutRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getDecisionsList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      1,
      f,
      proto.semio.model.v1.Decision.serializeBinaryToWriter
    );
  }
  f = message.getInitialLayout();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      proto.semio.model.v1.Layout.serializeBinaryToWriter
    );
  }
};


/**
 * repeated semio.model.v1.Decision decisions = 1;
 * @return {!Array<!proto.semio.model.v1.Decision>}
 */
proto.semio.extension.transformer.v1.RewriteLayoutRequest.prototype.getDecisionsList = function() {
  return /** @type{!Array<!proto.semio.model.v1.Decision>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.semio.model.v1.Decision, 1));
};


/**
 * @param {!Array<!proto.semio.model.v1.Decision>} value
 * @return {!proto.semio.extension.transformer.v1.RewriteLayoutRequest} returns this
*/
proto.semio.extension.transformer.v1.RewriteLayoutRequest.prototype.setDecisionsList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 1, value);
};


/**
 * @param {!proto.semio.model.v1.Decision=} opt_value
 * @param {number=} opt_index
 * @return {!proto.semio.model.v1.Decision}
 */
proto.semio.extension.transformer.v1.RewriteLayoutRequest.prototype.addDecisions = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 1, opt_value, proto.semio.model.v1.Decision, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.semio.extension.transformer.v1.RewriteLayoutRequest} returns this
 */
proto.semio.extension.transformer.v1.RewriteLayoutRequest.prototype.clearDecisionsList = function() {
  return this.setDecisionsList([]);
};


/**
 * optional semio.model.v1.Layout initial_layout = 2;
 * @return {?proto.semio.model.v1.Layout}
 */
proto.semio.extension.transformer.v1.RewriteLayoutRequest.prototype.getInitialLayout = function() {
  return /** @type{?proto.semio.model.v1.Layout} */ (
    jspb.Message.getWrapperField(this, proto.semio.model.v1.Layout, 2));
};


/**
 * @param {?proto.semio.model.v1.Layout|undefined} value
 * @return {!proto.semio.extension.transformer.v1.RewriteLayoutRequest} returns this
*/
proto.semio.extension.transformer.v1.RewriteLayoutRequest.prototype.setInitialLayout = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.semio.extension.transformer.v1.RewriteLayoutRequest} returns this
 */
proto.semio.extension.transformer.v1.RewriteLayoutRequest.prototype.clearInitialLayout = function() {
  return this.setInitialLayout(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.semio.extension.transformer.v1.RewriteLayoutRequest.prototype.hasInitialLayout = function() {
  return jspb.Message.getField(this, 2) != null;
};

