// @generated by protobuf-ts 2.8.2 with parameter server_grpc1,generate_dependencies
// @generated from protobuf file "geometry/v1/geometry.proto" (package "semio.geometry.v1", syntax proto3)
// tslint:disable
import type { BinaryWriteOptions } from "@protobuf-ts/runtime";
import type { IBinaryWriter } from "@protobuf-ts/runtime";
import { WireType } from "@protobuf-ts/runtime";
import type { BinaryReadOptions } from "@protobuf-ts/runtime";
import type { IBinaryReader } from "@protobuf-ts/runtime";
import { UnknownFieldHandler } from "@protobuf-ts/runtime";
import type { PartialMessage } from "@protobuf-ts/runtime";
import { reflectionMergePartial } from "@protobuf-ts/runtime";
import { MESSAGE_TYPE } from "@protobuf-ts/runtime";
import { MessageType } from "@protobuf-ts/runtime";
/**
 * A 3d point with x,y,z coordinates.
 *
 * @generated from protobuf message semio.geometry.v1.Point
 */
export interface Point {
    /**
     * X coordinate.
     *
     * @generated from protobuf field: double x = 1;
     */
    x: number;
    /**
     * Y coordinate
     *
     * @generated from protobuf field: double y = 2;
     */
    y: number;
    /**
     * Z coordinate
     *
     * @generated from protobuf field: double z = 3;
     */
    z: number;
}
/**
 * A 3d vector with x,y,z coordinates.
 *
 * @generated from protobuf message semio.geometry.v1.Vector
 */
export interface Vector {
    /**
     * X coordinate.
     *
     * @generated from protobuf field: double x = 1;
     */
    x: number;
    /**
     * Y coordinate
     *
     * @generated from protobuf field: double y = 2;
     */
    y: number;
    /**
     * Z coordinate
     *
     * @generated from protobuf field: double z = 3;
     */
    z: number;
}
/**
 * (Unit) quaternions represent (here) (rotational) orientation. It can be interpreted as the view of an object.
 *
 * @generated from protobuf message semio.geometry.v1.Quaternion
 */
export interface Quaternion {
    /**
     * @generated from protobuf field: double w = 1;
     */
    w: number;
    /**
     * @generated from protobuf field: double x = 2;
     */
    x: number;
    /**
     * @generated from protobuf field: double y = 3;
     */
    y: number;
    /**
     * @generated from protobuf field: double z = 4;
     */
    z: number;
}
// @generated message type with reflection information, may provide speed optimized methods
class Point$Type extends MessageType<Point> {
    constructor() {
        super("semio.geometry.v1.Point", [
            { no: 1, name: "x", kind: "scalar", T: 1 /*ScalarType.DOUBLE*/ },
            { no: 2, name: "y", kind: "scalar", T: 1 /*ScalarType.DOUBLE*/ },
            { no: 3, name: "z", kind: "scalar", T: 1 /*ScalarType.DOUBLE*/ }
        ]);
    }
    create(value?: PartialMessage<Point>): Point {
        const message = { x: 0, y: 0, z: 0 };
        globalThis.Object.defineProperty(message, MESSAGE_TYPE, { enumerable: false, value: this });
        if (value !== undefined)
            reflectionMergePartial<Point>(this, message, value);
        return message;
    }
    internalBinaryRead(reader: IBinaryReader, length: number, options: BinaryReadOptions, target?: Point): Point {
        let message = target ?? this.create(), end = reader.pos + length;
        while (reader.pos < end) {
            let [fieldNo, wireType] = reader.tag();
            switch (fieldNo) {
                case /* double x */ 1:
                    message.x = reader.double();
                    break;
                case /* double y */ 2:
                    message.y = reader.double();
                    break;
                case /* double z */ 3:
                    message.z = reader.double();
                    break;
                default:
                    let u = options.readUnknownField;
                    if (u === "throw")
                        throw new globalThis.Error(`Unknown field ${fieldNo} (wire type ${wireType}) for ${this.typeName}`);
                    let d = reader.skip(wireType);
                    if (u !== false)
                        (u === true ? UnknownFieldHandler.onRead : u)(this.typeName, message, fieldNo, wireType, d);
            }
        }
        return message;
    }
    internalBinaryWrite(message: Point, writer: IBinaryWriter, options: BinaryWriteOptions): IBinaryWriter {
        /* double x = 1; */
        if (message.x !== 0)
            writer.tag(1, WireType.Bit64).double(message.x);
        /* double y = 2; */
        if (message.y !== 0)
            writer.tag(2, WireType.Bit64).double(message.y);
        /* double z = 3; */
        if (message.z !== 0)
            writer.tag(3, WireType.Bit64).double(message.z);
        let u = options.writeUnknownFields;
        if (u !== false)
            (u == true ? UnknownFieldHandler.onWrite : u)(this.typeName, message, writer);
        return writer;
    }
}
/**
 * @generated MessageType for protobuf message semio.geometry.v1.Point
 */
export const Point = new Point$Type();
// @generated message type with reflection information, may provide speed optimized methods
class Vector$Type extends MessageType<Vector> {
    constructor() {
        super("semio.geometry.v1.Vector", [
            { no: 1, name: "x", kind: "scalar", T: 1 /*ScalarType.DOUBLE*/ },
            { no: 2, name: "y", kind: "scalar", T: 1 /*ScalarType.DOUBLE*/ },
            { no: 3, name: "z", kind: "scalar", T: 1 /*ScalarType.DOUBLE*/ }
        ]);
    }
    create(value?: PartialMessage<Vector>): Vector {
        const message = { x: 0, y: 0, z: 0 };
        globalThis.Object.defineProperty(message, MESSAGE_TYPE, { enumerable: false, value: this });
        if (value !== undefined)
            reflectionMergePartial<Vector>(this, message, value);
        return message;
    }
    internalBinaryRead(reader: IBinaryReader, length: number, options: BinaryReadOptions, target?: Vector): Vector {
        let message = target ?? this.create(), end = reader.pos + length;
        while (reader.pos < end) {
            let [fieldNo, wireType] = reader.tag();
            switch (fieldNo) {
                case /* double x */ 1:
                    message.x = reader.double();
                    break;
                case /* double y */ 2:
                    message.y = reader.double();
                    break;
                case /* double z */ 3:
                    message.z = reader.double();
                    break;
                default:
                    let u = options.readUnknownField;
                    if (u === "throw")
                        throw new globalThis.Error(`Unknown field ${fieldNo} (wire type ${wireType}) for ${this.typeName}`);
                    let d = reader.skip(wireType);
                    if (u !== false)
                        (u === true ? UnknownFieldHandler.onRead : u)(this.typeName, message, fieldNo, wireType, d);
            }
        }
        return message;
    }
    internalBinaryWrite(message: Vector, writer: IBinaryWriter, options: BinaryWriteOptions): IBinaryWriter {
        /* double x = 1; */
        if (message.x !== 0)
            writer.tag(1, WireType.Bit64).double(message.x);
        /* double y = 2; */
        if (message.y !== 0)
            writer.tag(2, WireType.Bit64).double(message.y);
        /* double z = 3; */
        if (message.z !== 0)
            writer.tag(3, WireType.Bit64).double(message.z);
        let u = options.writeUnknownFields;
        if (u !== false)
            (u == true ? UnknownFieldHandler.onWrite : u)(this.typeName, message, writer);
        return writer;
    }
}
/**
 * @generated MessageType for protobuf message semio.geometry.v1.Vector
 */
export const Vector = new Vector$Type();
// @generated message type with reflection information, may provide speed optimized methods
class Quaternion$Type extends MessageType<Quaternion> {
    constructor() {
        super("semio.geometry.v1.Quaternion", [
            { no: 1, name: "w", kind: "scalar", T: 1 /*ScalarType.DOUBLE*/ },
            { no: 2, name: "x", kind: "scalar", T: 1 /*ScalarType.DOUBLE*/ },
            { no: 3, name: "y", kind: "scalar", T: 1 /*ScalarType.DOUBLE*/ },
            { no: 4, name: "z", kind: "scalar", T: 1 /*ScalarType.DOUBLE*/ }
        ]);
    }
    create(value?: PartialMessage<Quaternion>): Quaternion {
        const message = { w: 0, x: 0, y: 0, z: 0 };
        globalThis.Object.defineProperty(message, MESSAGE_TYPE, { enumerable: false, value: this });
        if (value !== undefined)
            reflectionMergePartial<Quaternion>(this, message, value);
        return message;
    }
    internalBinaryRead(reader: IBinaryReader, length: number, options: BinaryReadOptions, target?: Quaternion): Quaternion {
        let message = target ?? this.create(), end = reader.pos + length;
        while (reader.pos < end) {
            let [fieldNo, wireType] = reader.tag();
            switch (fieldNo) {
                case /* double w */ 1:
                    message.w = reader.double();
                    break;
                case /* double x */ 2:
                    message.x = reader.double();
                    break;
                case /* double y */ 3:
                    message.y = reader.double();
                    break;
                case /* double z */ 4:
                    message.z = reader.double();
                    break;
                default:
                    let u = options.readUnknownField;
                    if (u === "throw")
                        throw new globalThis.Error(`Unknown field ${fieldNo} (wire type ${wireType}) for ${this.typeName}`);
                    let d = reader.skip(wireType);
                    if (u !== false)
                        (u === true ? UnknownFieldHandler.onRead : u)(this.typeName, message, fieldNo, wireType, d);
            }
        }
        return message;
    }
    internalBinaryWrite(message: Quaternion, writer: IBinaryWriter, options: BinaryWriteOptions): IBinaryWriter {
        /* double w = 1; */
        if (message.w !== 0)
            writer.tag(1, WireType.Bit64).double(message.w);
        /* double x = 2; */
        if (message.x !== 0)
            writer.tag(2, WireType.Bit64).double(message.x);
        /* double y = 3; */
        if (message.y !== 0)
            writer.tag(3, WireType.Bit64).double(message.y);
        /* double z = 4; */
        if (message.z !== 0)
            writer.tag(4, WireType.Bit64).double(message.z);
        let u = options.writeUnknownFields;
        if (u !== false)
            (u == true ? UnknownFieldHandler.onWrite : u)(this.typeName, message, writer);
        return writer;
    }
}
/**
 * @generated MessageType for protobuf message semio.geometry.v1.Quaternion
 */
export const Quaternion = new Quaternion$Type();