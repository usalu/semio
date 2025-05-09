import cytoscape from 'cytoscape'
import * as THREE from 'three'

import { jaccard } from '@semio/js/lib/utils';
// TODOs
// Update to latest schema and unify docstrings

// Initially created from json-schema-to-typescript: https://app.quicktype.io/
// Manually edited to align with GraphQL schema.

// To parse this data:
//
//   import { Convert, Kit } from "./file";
//
//   const kit = Convert.toKit(json);
//
// These functions will throw an error if the JSON doesn't
// match the expected interface, even if the JSON is valid.

export const ICON_WIDTH = 50;
export const TOLERANCE = 1e-5;

// ↗️ Represents a Kit, the top-level container for types and designs.
export type Kit = {
    // 📛 The name of the kit
    name: string;
    // 💬 The human-readable description of the kit
    description: string;
    // 🪙 The icon [ emoji | logogram | url ] of the kit
    icon: string;
    // 🖼️ The URL to the image of the kit
    image: string;
    // 🔮 The URL of the preview image of the kit
    preview: string;
    // 🔀 The version of the kit
    version: string;
    // ☁️ The Unique Resource Locator (URL) where to fetch the kit remotely
    remote: string;
    // 🏠 The URL of the homepage of the kit
    homepage: string;
    // ⚖️ The license [ spdx id | url ] of the kit
    license: string;
    // 🕒 The creation date of the kit
    created: Date;
    // 🕒 The last update date of the kit
    updated: Date;
    // 🧩 The types defined within the kit
    types?: Type[];
    // 🏙️ The designs defined within the kit
    designs?: Design[];
    // 📏 The qualities associated with the kit
    qualities?: Quality[];
}

// 🏙️ A design is a collection of connected pieces.
export type Design = {
    // 📛 The name of the design
    name: string;
    // 💬 The human-readable description of the design
    description: string;
    // 🪙 The icon [ emoji | logogram | url ] of the design
    icon: string;
    // 🖼️ The URL to the image of the design
    image: string;
    // 🔀 The variant of the design
    variant: string;
    // 🥽 The view of the design
    view: string;
    // 📏 The unit of the design
    unit: string;
    // 🕒 The creation date of the design
    created: Date;
    // 🕒 The last update date of the design
    updated: Date;
    // 🧩 The pieces included in the design
    pieces?: Piece[];
    // 🖇️ The connections between pieces in the design
    connections?: Connection[];
    // 📑 The authors of the design
    authors: Author[];
    // 📏 The qualities associated with the design
    qualities?: Quality[];
}

// 📑 Represents an author.
export type Author = {
    // 📛 The name of the author
    name: string;
    // 📧 The email of the author
    email: string;
    // #️⃣ The rank of the author
    rank: number;
}

// 🖇️ A bidirectional connection between two pieces of a design.
export type Connection = {
    // 🧲 The connected side of the connection
    connected: Side;
    // 🧲 The connecting side of the connection
    connecting: Side;
    // 💬 The human-readable description of the connection
    description: string;
    // ↕️ The longitudinal gap between connected pieces
    gap: number;
    // ↔️ The lateral shift between connected pieces
    shift: number;
    // 🪜 The vertical raise between connected pieces
    raise_: number;
    // 🔄 The horizontal rotation between connected pieces in degrees
    rotation: number;
    // 🛞 The turn between connected pieces in degrees
    turn: number;
    // ↗️ The horizontal tilt between connected pieces in degrees
    tilt: number;
    // ➡️ The offset in x direction in the diagram
    x: number;
    // ⬆️ The offset in y direction in the diagram
    y: number;
    // 📏 The qualities associated with the connection
    qualities?: Quality[];
}

// 🧱 A side of a piece in a connection, identifying a specific port on a specific piece.
export type Side = {
    // ⭕ The piece involved in this side of the connection
    piece: PieceID; // Represents Piece identifier
    // 🔌 The port involved in this side of the connection
    port: PortID;   // Represents Port identifier
}

// 🪪 Identifier for a piece within a design.
export type PieceID = {
    // 🆔 The id of the piece
    id_: string;
}

// 🪪 Identifier for a port within a type.
export type PortID = {
    // 🆔 The id of the port
    id_?: string;
}

// ⭕ A piece is a 3D instance of a type within a design.
export type Piece = {
    // 🆔 The id of the piece
    id_: string;
    // 💬 The human-readable description of the piece
    description?: string;
    // 🧩 The type defining this piece
    type: TypeID; // Represents Type identifier
    // ◳ The optional plane (position and orientation) of the piece
    plane?: Plane;
    // 📺 The optional center of the piece in the diagram
    center?: DiagramPoint;
    // 📏 The qualities associated with the piece
    qualities?: Quality[];
}

// 📺 A 2D point (xy) in the diagram coordinate system.
export type DiagramPoint = {
    // 🏁 The x-coordinate in the diagram
    x: number;
    // 🏁 The y-coordinate in the diagram
    y: number;
}

// ◳ A plane defined by an origin point and two axes vectors.
export type Plane = {
    // ⌱ The origin point of the plane
    origin: Point;
    // ➡️ The x-axis vector of the plane
    xAxis: Vector;
    // ➡️ The y-axis vector of the plane
    yAxis: Vector;
}

// ✖️ A 3D point (xyz) with floating-point coordinates.
export type Point = {
    // 🎚️ The x-coordinate of the point
    x: number;
    // 🎚️ The y-coordinate of the point
    y: number;
    // 🎚️ The z-coordinate of the point
    z: number;
}

// ➡️ A 3D vector (xyz) with floating-point coordinates.
export type Vector = {
    // 🎚️ The x-coordinate of the vector
    x: number;
    // 🎚️ The y-coordinate of the vector
    y: number;
    // 🎚️ The z-coordinate of the vector
    z: number;
}

// 🪪 Identifier for a type, potentially including a variant.
export type TypeID = {
    // 📛 The name of the type
    name: string;
    // 🔀 The optional variant of the type
    variant?: string;
}

// 📏 Represents a quality, a named property with an optional value, unit, and definition.
export type Quality = {
    // 📛 The name of the quality
    name: string;
    // ❓ The optional value of the quality
    value?: string;
    // 📐 The optional unit of the quality's value
    unit?: string;
    // 📖 The optional definition [ text | url ] of the quality
    definition?: string;
}

// 🧩 A type is a reusable element blueprint with ports for connection.
export type Type = {
    // 📛 The name of the type
    name: string;
    // 💬 The human-readable description of the type
    description: string;
    // 🪙 The icon [ emoji | logogram | url ] of the type
    icon: string;
    // 🖼️ The URL to the image of the type
    image: string;
    // 🔀 The variant of the type
    variant: string;
    // Ⓜ️ The length unit used by the type's geometry
    unit: string;
    // 🕒 The creation date of the type
    created: Date;
    // 🕒 The last update date of the type
    updated: Date;
    // 💾 Representations (e.g., CAD files) of the type
    representations?: Representation[];
    // 🔌 Connection points (ports) of the type
    ports?: Port[];
    // 📑 Authors of the type
    authors: Author[];
    // 📏 Qualities associated with the type
    qualities?: Quality[];
}

// 🔌 A port is a connection point on a type, defined by a point and direction.
export type Port = {
    // 🆔 The id of the port
    id_?: string;
    // 💬 The human-readable description of the port
    description: string;
    // 👨‍👩‍👧‍👦 The family of the port for compatibility checks
    family: string;
    // 💍 The parameter t [0,1[ for diagram visualization
    t: number;
    // ✅ Other compatible port families
    compatibleFamilies: string[];
    // ✖️ The connection point geometry
    point: Point;
    // ➡️ The connection direction vector
    direction: Vector;
    // 📏 Qualities associated with the port
    qualities?: Quality[];
}

// 💾 A representation links to a resource (e.g., file) describing a type.
export type Representation = {
    // 🔗 The URL to the resource
    url: string;
    // 💬 The human-readable description of the representation
    description: string;
    // ✉️ The MIME type of the resource
    mime: string;
    // 🏷️ Tags to group or filter representations
    tags: string[];
    // 📏 Qualities associated with the representation
    qualities?: Quality[];
}

// Converts JSON strings to/from your types
// and asserts the results of JSON.parse at runtime
export class Convert {
    public static toKit(json: string): Kit {
        return cast(JSON.parse(json), r("Kit"));
    }

    public static kitToJson(value: Kit): string {
        return JSON.stringify(uncast(value, r("Kit")), null, 2);
    }

    // Add similar methods for other top-level types if needed
}

function invalidValue(typ: any, val: any, key: any, parent: any = ''): never {
    const prettyTyp = prettyTypeName(typ);
    const parentText = parent ? ` on ${parent}` : '';
    const keyText = key ? ` for key "${key}"` : '';
    throw Error(`Invalid value${keyText}${parentText}. Expected ${prettyTyp} but got ${JSON.stringify(val)}`);
}

function prettyTypeName(typ: any): string {
    if (Array.isArray(typ)) {
        if (typ.length === 2 && typ[0] === undefined) {
            return `an optional ${prettyTypeName(typ[1])}`;
        } else {
            return `one of [${typ.map(a => { return prettyTypeName(a); }).join(", ")}]`;
        }
    } else if (typeof typ === "object" && typ.literal !== undefined) {
        return typ.literal;
    } else {
        return typeof typ;
    }
}

function jsonToJSProps(typ: any): any {
    if (typ.jsonToJS === undefined) {
        const map: any = {};
        typ.props.forEach((p: any) => map[p.json] = { key: p.js, typ: p.typ });
        typ.jsonToJS = map;
    }
    return typ.jsonToJS;
}

function jsToJSONProps(typ: any): any {
    if (typ.jsToJSON === undefined) {
        const map: any = {};
        typ.props.forEach((p: any) => map[p.js] = { key: p.json, typ: p.typ });
        typ.jsToJSON = map;
    }
    return typ.jsToJSON;
}

function transform(val: any, typ: any, getProps: any, key: any = '', parent: any = ''): any {
    function transformPrimitive(typ: string, val: any): any {
        if (typeof typ === typeof val) return val;
        // Allow numbers to be parsed as strings explicitly for flexibility if needed downstream
        // if (typ === "string" && typeof val === "number") return String(val);
        return invalidValue(typ, val, key, parent);
    }

    function transformUnion(typs: any[], val: any): any {
        // val must validate against one typ in typs
        const l = typs.length;
        for (let i = 0; i < l; i++) {
            const typ = typs[i];
            try {
                return transform(val, typ, getProps);
            } catch (_) { }
        }
        return invalidValue(typs, val, key, parent);
    }

    function transformEnum(cases: string[], val: any): any {
        if (cases.indexOf(val) !== -1) return val;
        return invalidValue(cases.map(a => { return l(a); }), val, key, parent);
    }

    function transformArray(typ: any, val: any): any {
        // val must be an array with no invalid elements
        if (!Array.isArray(val)) return invalidValue(l("array"), val, key, parent);
        return val.map(el => transform(el, typ, getProps));
    }

    function transformDate(val: any): any {
        if (val === null) {
            return null;
        }
        // Allow strings matching ISO 8601 format
        if (typeof val === "string") {
            const d = new Date(val);
            if (!isNaN(d.valueOf())) return d;
        }
        // Allow numbers representing milliseconds since epoch
        if (typeof val === "number") {
            const d = new Date(val);
            if (!isNaN(d.valueOf())) return d;
        }
        // Disallow direct Date objects unless converting back (jsToJSON)
        if (val instanceof Date && getProps === jsToJSONProps) {
            return val.toISOString(); // Convert Date back to ISO string for JSON
        }

        return invalidValue(l("Date"), val, key, parent);
    }

    function transformObject(props: { [k: string]: any }, additional: any, val: any): any {
        if (val === null || typeof val !== "object" || Array.isArray(val)) {
            return invalidValue(l(ref || "object"), val, key, parent);
        }
        const result: any = {};
        Object.getOwnPropertyNames(props).forEach(key => {
            const prop = props[key];
            const v = Object.prototype.hasOwnProperty.call(val, key) ? val[key] : undefined;
            result[prop.key] = transform(v, prop.typ, getProps, key, ref);
        });
        Object.getOwnPropertyNames(val).forEach(key => {
            if (!Object.prototype.hasOwnProperty.call(props, key)) {
                // Handle additional properties if necessary, based on `additional` definition
                // Default behavior here might discard unknown properties or pass them through if `additional` allows
                if (additional && additional !== false) {
                    result[key] = transform(val[key], additional, getProps, key, ref);
                } else if (!additional) {
                    // If additional properties are not allowed, either ignore or throw error
                    // console.warn(`Ignoring additional property "${key}" during transform for ${ref || 'object'}`);
                } else { // additional === false
                    invalidValue("no additional properties", key, key, ref)
                }
            }
        });
        return result;
    }

    if (typ === "any") return val;
    if (typ === null) {
        if (val === null) return val;
        return invalidValue(typ, val, key, parent);
    }
    if (typ === false) return invalidValue(typ, val, key, parent);
    let ref: string | undefined = undefined;
    while (typeof typ === "object" && typ.ref !== undefined) {
        ref = typ.ref;
        typ = typeMap[typ.ref];
    }
    if (Array.isArray(typ)) return transformEnum(typ, val); // Check for enums if defined as arrays of literals
    if (typeof typ === "object") {
        return typ.hasOwnProperty("unionMembers") ? transformUnion(typ.unionMembers, val)
            : typ.hasOwnProperty("arrayItems") ? transformArray(typ.arrayItems, val)
                : typ.hasOwnProperty("props") ? transformObject(getProps(typ), typ.additional, val)
                    : invalidValue(typ, val, key, parent);
    }
    // Handle Date transformation
    if (typ === Date) return transformDate(val); // Use updated transformDate
    // Handle primitive types
    return transformPrimitive(typ, val);
}


function cast<T>(val: any, typ: any): T {
    return transform(val, typ, jsonToJSProps);
}

function uncast<T>(val: T, typ: any): any {
    return transform(val, typ, jsToJSONProps);
}

function l(typ: any) {
    return { literal: typ };
}

function a(typ: any) {
    return { arrayItems: typ };
}

function u(...typs: any[]) {
    return { unionMembers: typs };
}

// `o` defines an object structure.
// `props` is an array of property definitions: { json: string, js: string, typ: any }
// `additional` defines how to handle extra properties found in JSON (can be a type, `false`, or `undefined`/`true`)
function o(props: any[], additional: any) {
    return { props, additional };
}

// `m` is currently unused but was likely intended for map-like structures.
// function m(additional: any) {
//     return { props: [], additional };
// }

function r(name: string) {
    return { ref: name };
}

// Type definitions for the Convert class based on GraphQL schema
// Note: `any` is used for `additional` properties to simplify, assuming flexibility. Adjust if strictness is needed.
const typeMap: any = {
    "Kit": o([
        { json: "uri", js: "uri", typ: "" },
        { json: "name", js: "name", typ: "" },
        { json: "description", js: "description", typ: "" },
        { json: "icon", js: "icon", typ: "" },
        { json: "image", js: "image", typ: "" },
        { json: "preview", js: "preview", typ: "" },
        { json: "version", js: "version", typ: "" },
        { json: "remote", js: "remote", typ: "" },
        { json: "homepage", js: "homepage", typ: "" },
        { json: "license", js: "license", typ: "" },
        { json: "created", js: "created", typ: Date },
        { json: "updated", js: "updated", typ: Date },
        { json: "types", js: "types", typ: u(undefined, a(r("Type"))) },
        { json: "designs", js: "designs", typ: u(undefined, a(r("Design"))) },
        { json: "qualities", js: "qualities", typ: u(undefined, a(r("Quality"))) },
    ], "any"), // Allow additional properties for flexibility
    "Design": o([
        { json: "name", js: "name", typ: "" },
        { json: "description", js: "description", typ: "" },
        { json: "icon", js: "icon", typ: "" },
        { json: "image", js: "image", typ: "" },
        { json: "variant", js: "variant", typ: "" },
        { json: "view", js: "view", typ: "" },
        { json: "unit", js: "unit", typ: "" },
        { json: "created", js: "created", typ: Date },
        { json: "updated", js: "updated", typ: Date },
        { json: "pieces", js: "pieces", typ: u(undefined, a(r("Piece"))) },
        { json: "connections", js: "connections", typ: u(undefined, a(r("Connection"))) },
        { json: "authors", js: "authors", typ: a(r("Author")) },
        { json: "qualities", js: "qualities", typ: u(undefined, a(r("Quality"))) },
    ], "any"),
    "Author": o([
        { json: "name", js: "name", typ: "" },
        { json: "email", js: "email", typ: "" },
        { json: "rank", js: "rank", typ: 0 }, // Use 0 for number type
    ], "any"),
    "Connection": o([
        { json: "description", js: "description", typ: "" },
        { json: "gap", js: "gap", typ: 0 },
        { json: "shift", js: "shift", typ: 0 },
        { json: "raise_", js: "raise_", typ: 0 },
        { json: "rotation", js: "rotation", typ: 0 },
        { json: "turn", js: "turn", typ: 0 },
        { json: "tilt", js: "tilt", typ: 0 },
        { json: "x", js: "x", typ: 0 },
        { json: "y", js: "y", typ: 0 },
        { json: "connected", js: "connected", typ: r("Side") },
        { json: "connecting", js: "connecting", typ: r("Side") },
        { json: "qualities", js: "qualities", typ: u(undefined, a(r("Quality"))) },
    ], "any"),
    "Side": o([
        { json: "piece", js: "piece", typ: r("PieceID") },
        { json: "port", js: "port", typ: r("PortID") },
    ], "any"),
    "PieceID": o([
        { json: "id_", js: "id_", typ: u(undefined, "") },
    ], "any"),
    "PortID": o([
        { json: "id_", js: "id_", typ: u(undefined, "") },
    ], "any"),
    "Piece": o([
        { json: "id_", js: "id_", typ: u(undefined, "") },
        { json: "description", js: "description", typ: "" },
        { json: "type", js: "type", typ: r("TypeID") },
        { json: "plane", js: "plane", typ: u(undefined, r("Plane")) }, // Now optional object
        { json: "center", js: "center", typ: r("DiagramPoint") }, // Now required object
        { json: "qualities", js: "qualities", typ: u(undefined, a(r("Quality"))) },
        { json: "connections", js: "connections", typ: a(r("Connection")) }, // Now required array
    ], "any"),
    "DiagramPoint": o([
        { json: "x", js: "x", typ: 0 },
        { json: "y", js: "y", typ: 0 },
    ], "any"),
    "Plane": o([
        { json: "origin", js: "origin", typ: r("Point") },
        { json: "xAxis", js: "xAxis", typ: r("Vector") },
        { json: "yAxis", js: "yAxis", typ: r("Vector") },
    ], "any"),
    "Point": o([
        { json: "x", js: "x", typ: 0 },
        { json: "y", js: "y", typ: 0 },
        { json: "z", js: "z", typ: 0 },
    ], "any"),
    "Vector": o([
        { json: "x", js: "x", typ: 0 },
        { json: "y", js: "y", typ: 0 },
        { json: "z", js: "z", typ: 0 },
    ], "any"),
    "TypeID": o([
        { json: "name", js: "name", typ: "" },
        { json: "variant", js: "variant", typ: u(undefined, "") },
    ], "any"),
    "Quality": o([
        { json: "name", js: "name", typ: "" },
        { json: "value", js: "value", typ: "" },
        { json: "unit", js: "unit", typ: "" },
        { json: "definition", js: "definition", typ: "" },
    ], "any"),
    "Type": o([
        { json: "name", js: "name", typ: "" },
        { json: "description", js: "description", typ: "" },
        { json: "icon", js: "icon", typ: "" },
        { json: "image", js: "image", typ: "" },
        { json: "variant", js: "variant", typ: "" },
        { json: "unit", js: "unit", typ: "" },
        { json: "created", js: "created", typ: Date },
        { json: "updated", js: "updated", typ: Date },
        { json: "representations", js: "representations", typ: u(undefined, a(r("Representation"))) },
        { json: "ports", js: "ports", typ: u(undefined, a(r("Port"))) },
        { json: "authors", js: "authors", typ: a(r("Author")) },
        { json: "qualities", js: "qualities", typ: u(undefined, a(r("Quality"))) },
        { json: "pieces", js: "pieces", typ: u(undefined, a(r("Piece"))) },
    ], "any"),
    "Port": o([
        { json: "id_", js: "id_", typ: u(undefined, "") },
        { json: "description", js: "description", typ: "" },
        { json: "family", js: "family", typ: "" },
        { json: "t", js: "t", typ: 0 },
        { json: "compatibleFamilies", js: "compatibleFamilies", typ: a("") }, // Required array of strings
        { json: "point", js: "point", typ: r("Point") },
        { json: "direction", js: "direction", typ: r("Vector") },
        { json: "qualities", js: "qualities", typ: u(undefined, a(r("Quality"))) },
        { json: "connections", js: "connections", typ: a(r("Connection")) }, // Required array
    ], "any"),
    "Representation": o([
        { json: "url", js: "url", typ: "" },
        { json: "description", js: "description", typ: "" },
        { json: "mime", js: "mime", typ: "" },
        { json: "tags", js: "tags", typ: a("") }, // Required array of strings
        { json: "qualities", js: "qualities", typ: u(undefined, a(r("Quality"))) },
    ], "any"),
};


const round = (value: number): number => {
    return Math.round(value / TOLERANCE) * TOLERANCE;
};

const roundPlane = (plane: Plane): Plane => {
    return {
        origin: { x: round(plane.origin.x), y: round(plane.origin.y), z: round(plane.origin.z) },
        xAxis: { x: round(plane.xAxis.x), y: round(plane.xAxis.y), z: round(plane.xAxis.z) },
        yAxis: { x: round(plane.yAxis.x), y: round(plane.yAxis.y), z: round(plane.yAxis.z) },
    };
};


const planeToMatrix = (plane: Plane): THREE.Matrix4 => {
    const origin = new THREE.Vector3(plane.origin.x, plane.origin.y, plane.origin.z);
    const xAxis = new THREE.Vector3(plane.xAxis.x, plane.xAxis.y, plane.xAxis.z);
    const yAxis = new THREE.Vector3(plane.yAxis.x, plane.yAxis.y, plane.yAxis.z);
    const zAxis = new THREE.Vector3().crossVectors(xAxis, yAxis).normalize();
    const orthoYAxis = new THREE.Vector3().crossVectors(zAxis, xAxis).normalize();
    const matrix = new THREE.Matrix4();
    matrix.makeBasis(xAxis.normalize(), orthoYAxis, zAxis);
    matrix.setPosition(origin);
    return matrix;
};

const matrixToPlane = (matrix: THREE.Matrix4): Plane => {
    const origin = new THREE.Vector3();
    const xAxis = new THREE.Vector3();
    const yAxis = new THREE.Vector3();
    const zAxis = new THREE.Vector3();

    matrix.decompose(origin, new THREE.Quaternion(), new THREE.Vector3());
    matrix.extractBasis(xAxis, yAxis, zAxis);

    return {
        origin: { x: origin.x, y: origin.y, z: origin.z },
        xAxis: { x: xAxis.x, y: xAxis.y, z: xAxis.z },
        yAxis: { x: yAxis.x, y: yAxis.y, z: yAxis.z },
    };
};


const semioVectorToThree = (v: Point | Vector): THREE.Vector3 => {
    return new THREE.Vector3(v.x, v.y, v.z);
};

const computeChildPlane = (
    parentPlane: Plane,
    parentPort: Port,
    childPort: Port,
    connection: Connection
): Plane => {

    const parentMatrix = planeToMatrix(parentPlane);
    const parentPoint = semioVectorToThree(parentPort.point);
    const parentDirection = semioVectorToThree(parentPort.direction).normalize();
    const childPoint = semioVectorToThree(childPort.point);
    const childDirection = semioVectorToThree(childPort.direction).normalize();

    const { gap, shift, raise_, rotation, turn, tilt } = connection;
    const rotationRad = THREE.MathUtils.degToRad(rotation);
    const turnRad = THREE.MathUtils.degToRad(turn);
    const tiltRad = THREE.MathUtils.degToRad(tilt);

    const targetDirection = parentDirection.clone();
    const sourceDirection = childDirection.clone().negate();

    const alignQuat = new THREE.Quaternion().setFromUnitVectors(sourceDirection, targetDirection);
    const alignMatrix = new THREE.Matrix4().makeRotationFromQuaternion(alignQuat);

    const parentXAxis = new THREE.Vector3();
    const parentYAxis = new THREE.Vector3();
    const parentZAxis = new THREE.Vector3();
    parentMatrix.extractBasis(parentXAxis, parentYAxis, parentZAxis);

    const rotationQuat = new THREE.Quaternion().setFromAxisAngle(parentDirection, -rotationRad);

    const turnAxis = new THREE.Vector3().crossVectors(parentXAxis, parentDirection).normalize();
    const turnQuat = new THREE.Quaternion().setFromAxisAngle(turnAxis, turnRad);

    const tiltQuat = new THREE.Quaternion().setFromAxisAngle(parentXAxis, tiltRad);

    const totalRotationQuat = new THREE.Quaternion()
        .multiplyQuaternions(tiltQuat, turnQuat)
        .multiply(rotationQuat)
        .multiply(alignQuat);

    const orientationMatrix = new THREE.Matrix4().makeRotationFromQuaternion(totalRotationQuat);
    const childLocalMatrix = new THREE.Matrix4().makeTranslation(childPoint.x, childPoint.y, childPoint.z).invert();
    const gapVec = parentDirection.clone().multiplyScalar(gap);
    const shiftVec = parentXAxis.clone().multiplyScalar(shift);
    const raiseVec = turnAxis.clone().multiplyScalar(raise_);
    const displacement = new THREE.Vector3().add(gapVec).add(shiftVec).add(raiseVec);
    const parentPortWorldPos = parentPoint.clone().applyMatrix4(parentMatrix);
    const childOriginWorldPos = parentPortWorldPos.clone().add(displacement);
    const translationMatrix = new THREE.Matrix4().makeTranslation(
        childOriginWorldPos.x,
        childOriginWorldPos.y,
        childOriginWorldPos.z
    );
    const finalChildMatrix = new THREE.Matrix4().multiplyMatrices(translationMatrix, orientationMatrix);
    const finalMatrix = new THREE.Matrix4().multiplyMatrices(finalChildMatrix, childLocalMatrix);

    return matrixToPlane(finalMatrix);
};


export const flattenDesign = (design: Design, types: Type[]): Design => {
    if (!design.pieces || design.pieces.length === 0) return design;

    const typesDict: { [key: string]: { [key: string]: Type } } = {};
    types.forEach(t => {
        if (!typesDict[t.name]) typesDict[t.name] = {};
        typesDict[t.name][t.variant || ''] = t;
    });
    const getType = (typeId: TypeID): Type | undefined => {
        return typesDict[typeId.name]?.[typeId.variant || ''];
    };
    const getPort = (type: Type | undefined, portId: PortID | undefined): Port | undefined => {
        if (!type?.ports) return undefined;
        return portId?.id_ ? type.ports.find(p => p.id_ === portId.id_) : type.ports[0];
    };

    const flatDesign: Design = JSON.parse(JSON.stringify(design));

    const piecePlanes: { [pieceId: string]: Plane } = {};
    const pieceMap: { [pieceId: string]: Piece } = {};
    flatDesign.pieces!.forEach(p => { if (p.id_) pieceMap[p.id_] = p });

    const cy = cytoscape({
        elements: {
            nodes: flatDesign.pieces!.map((piece) => ({
                data: { id: piece.id_, label: piece.id_ }
            })),
            edges: flatDesign.connections?.map((connection, index) => {
                const sourceId = connection.connected.piece.id_;
                const targetId = connection.connecting.piece.id_;
                return {
                    data: {
                        id: `${sourceId}--${targetId}`,
                        source: sourceId,
                        target: targetId,
                        connectionData: connection
                    }
                };
            }) ?? []
        },
        headless: true,
    });

    const components = cy.elements().components();
    let isFirstRoot = true;

    components.forEach((component) => {
        let roots = component.nodes().filter(node => {
            const piece = pieceMap[node.id()];
            return piece?.plane !== undefined;
        });
        let rootNode = roots.length > 0 ? roots[0] : component.nodes().length > 0 ? component.nodes()[0] : undefined;
        if (!rootNode) return;
        const rootPiece = pieceMap[rootNode.id()];
        if (!rootPiece || !rootPiece.id_) return;
        let rootPlane: Plane;
        if (rootPiece.plane) {
            rootPlane = rootPiece.plane;
        } else if (isFirstRoot) {
            const identityMatrix = new THREE.Matrix4().identity();
            rootPlane = matrixToPlane(identityMatrix);
            isFirstRoot = false;
        } else {
            console.warn(`Root piece ${rootPiece.id_} has no defined plane and is not the first root. Defaulting to identity plane.`);
            const identityMatrix = new THREE.Matrix4().identity();
            rootPlane = matrixToPlane(identityMatrix);
        }

        piecePlanes[rootPiece.id_] = rootPlane;
        const flatRootPiece: Piece = {
            ...rootPiece,
            plane: rootPlane,
        };
        flatDesign.pieces?.push(flatRootPiece);

        const bfs = cy.elements().bfs({
            roots: `#${rootNode.id()}`,
            visit: (v, e, u, i, depth) => {
                if (!e) return;
                const edgeData = e.data();
                const connection: Connection | undefined = edgeData.connectionData;
                if (!connection) return;
                const parentNode = u;
                const childNode = v;
                const parentId = parentNode.id();
                const childId = childNode.id();
                const parentPiece = pieceMap[parentId];
                const childPiece = pieceMap[childId];
                if (!parentPiece || !childPiece || !parentPiece.id_ || !childPiece.id_) return;
                if (piecePlanes[childPiece.id_]) return;
                const parentPlane = piecePlanes[parentPiece.id_];
                if (!parentPlane) {
                    console.error(`Error during flatten: Parent piece ${parentPiece.id_} plane not found.`);
                    return;
                }
                const parentSide = connection.connected.piece.id_ === parentId ? connection.connected : connection.connecting;
                const childSide = connection.connecting.piece.id_ === childId ? connection.connecting : connection.connected;
                const parentType = getType(parentPiece.type);
                const childType = getType(childPiece.type);
                const parentPort = getPort(parentType, parentSide.port);
                const childPort = getPort(childType, childSide.port);
                if (!parentPort || !childPort) {
                    console.error(`Error during flatten: Ports not found for connection between ${parentId} and ${childId}. Parent Port: ${parentSide.port.id_}, Child Port: ${childSide.port.id_}`);
                    return;
                }
                const childPlane = roundPlane(computeChildPlane(parentPlane, parentPort, childPort, connection));
                piecePlanes[childPiece.id_] = childPlane;
                const direction = semioVectorToThree({ x: connection.x, y: connection.y, z: 0 }).normalize();
                const childCenter = {
                    x: round(parentPiece.center!.x + connection.x + direction.x),
                    y: round(parentPiece.center!.y + connection.y + direction.y),
                }

                const flatChildPiece: Piece = {
                    ...childPiece,
                    plane: childPlane,
                    center: childCenter,
                    qualities: [...(childPiece.qualities ?? []),
                    {
                        name: 'semio',
                        value: JSON.stringify({
                            parentPieceId: parentPiece.id_,
                            depth: depth,
                        })
                    }],
                };
                pieceMap[childId] = flatChildPiece;
            },
            directed: false
        });
    });
    flatDesign.pieces = flatDesign.pieces?.map(p => pieceMap[p.id_ ?? '']);
    flatDesign.connections = [];
    return flatDesign;
}

const selectRepresentation = (representations: Representation[], mime: string, tags: string[]): Representation => {
    const filteredRepresentations = representations.filter(r => r.mime === mime);
    const indices = filteredRepresentations.map(r => jaccard(r.tags, tags));
    const maxIndex = Math.max(...indices);
    const maxIndexIndex = indices.indexOf(maxIndex);
    return filteredRepresentations[maxIndexIndex];
}

/**
 * 🔗 Returns a map of piece ids to representation urls for the given design and types.
 * @param design - The design with the pieces to get the representation urls for. 
 * @param types - The types of the pieces with the representations.
 * @returns A map of piece ids to representation urls.
 */
export const getPieceRepresentationUrls = (design: Design, types: Type[], mime: string = 'model/gltf-binary', tags: string[] = []): Map<string, string> => {
    const representationUrls = new Map<string, string>();
    design.pieces?.forEach(p => {
        const type = types.find(t => t.name === p.type.name && t.variant === p.type.variant);
        if (!type) throw new Error(`Type (${p.type.name}, ${p.type.variant}) for piece ${p.id_} not found`);
        if (!type.representations) throw new Error(`Type (${p.type.name}, ${p.type.variant}) for piece ${p.id_} has no representations`);
        const representation = selectRepresentation(type.representations, mime, tags);
        representationUrls.set(p.id_, representation.url);
    });
    return representationUrls;
}
