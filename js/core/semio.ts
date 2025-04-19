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

// ↗️ Represents a Kit, the top-level container for types and designs.
export type Kit = {
    // 🆔 The URI of the kit (GraphQL: uri: String!)
    uri: string;
    // 📛 The name of the kit (GraphQL: name: String!)
    name: string;
    // 💬 The human-readable description of the kit (GraphQL: description: String!)
    description: string;
    // 🪙 The icon [ emoji | logogram | url ] of the kit (GraphQL: icon: String!)
    icon: string;
    // 🖼️ The URL to the image of the kit (GraphQL: image: String!)
    image: string;
    // 🔮 The URL of the preview image of the kit (GraphQL: preview: String!)
    preview: string;
    // 🔀 The version of the kit (GraphQL: version: String!)
    version: string;
    // ☁️ The Unique Resource Locator (URL) where to fetch the kit remotely (GraphQL: remote: String!)
    remote: string;
    // 🏠 The URL of the homepage of the kit (GraphQL: homepage: String!)
    homepage: string;
    // ⚖️ The license [ spdx id | url ] of the kit (GraphQL: license: String!)
    license: string;
    // 🕒 The creation date of the kit (GraphQL: created: DateTime!)
    created: Date;
    // 🕒 The last update date of the kit (GraphQL: updated: DateTime!)
    updated: Date;
    // 🧩 The types defined within the kit (GraphQL: types: TypeNodeConnection)
    types?: Type[];
    // 🏙️ The designs defined within the kit (GraphQL: designs: DesignNodeConnection)
    designs?: Design[];
    // 📏 The qualities associated with the kit (GraphQL: qualities: QualityNodeConnection)
    qualities?: Quality[];
}

// 🏙️ A design is a collection of connected pieces.
export type Design = {
    // 📛 The name of the design (GraphQL: name: String!)
    name: string;
    // 💬 The human-readable description of the design (GraphQL: description: String!)
    description: string;
    // 🪙 The icon [ emoji | logogram | url ] of the design (GraphQL: icon: String!)
    icon: string;
    // 🖼️ The URL to the image of the design (GraphQL: image: String!)
    image: string;
    // 🔀 The variant of the design (GraphQL: variant: String!)
    variant: string;
    // 🥽 The view of the design (GraphQL: view: String!)
    view: string;
    // 📏 The unit of the design (GraphQL: unit: String!)
    unit: string;
    // 🕒 The creation date of the design (GraphQL: created: DateTime!)
    created: Date;
    // 🕒 The last update date of the design (GraphQL: updated: DateTime!)
    updated: Date;
    // 🧩 The pieces included in the design (GraphQL: pieces: PieceNodeConnection)
    pieces?: Piece[];
    // 🖇️ The connections between pieces in the design (GraphQL: connections: ConnectionNodeConnection)
    connections?: Connection[];
    // 📑 The authors of the design (GraphQL: authors: [Author!]!)
    authors: Author[];
    // 📏 The qualities associated with the design (GraphQL: qualities: QualityNodeConnection)
    qualities?: Quality[];
}

// 📑 Represents an author.
export type Author = {
    // 📛 The name of the author (GraphQL: name: String!)
    name: string;
    // 📧 The email of the author (GraphQL: email: String!)
    email: string;
    // #️⃣ The rank of the author (GraphQL: rank: Int!) - Added field
    rank: number;
}

// 🖇️ A bidirectional connection between two pieces of a design.
export type Connection = {
    // 💬 The human-readable description of the connection (GraphQL: description: String!)
    description: string;
    // ↕️ The longitudinal gap between connected pieces (GraphQL: gap: Float!)
    gap: number;
    // ↔️ The lateral shift between connected pieces (GraphQL: shift: Float!)
    shift: number;
    // 🪜 The vertical raise between connected pieces (GraphQL: raise_: Float!) - Added field
    raise_: number;
    // 🔄 The horizontal rotation between connected pieces in degrees (GraphQL: rotation: Float!)
    rotation: number;
    // 🛞 The turn between connected pieces in degrees (GraphQL: turn: Float!) - Added field
    turn: number;
    // ↗️ The horizontal tilt between connected pieces in degrees (GraphQL: tilt: Float!)
    tilt: number;
    // ➡️ The offset in x direction in the diagram (GraphQL: x: Float!)
    x: number;
    // ⬆️ The offset in y direction in the diagram (GraphQL: y: Float!)
    y: number;
    // 🧲 The connected side of the connection (GraphQL: connected: Side!)
    connected: Side;
    // 🧲 The connecting side of the connection (GraphQL: connecting: Side!)
    connecting: Side;
    // 📏 The qualities associated with the connection (GraphQL: qualities: QualityNodeConnection) - Added field
    qualities?: Quality[];
}

// 🧱 A side of a piece in a connection, identifying a specific port on a specific piece.
export type Side = {
    // ⭕ The piece involved in this side of the connection (GraphQL: piece: Piece!)
    piece: PieceID; // Represents Piece identifier
    // 🔌 The port involved in this side of the connection (GraphQL: port: Port!)
    port: PortID;   // Represents Port identifier
}

// 🪪 Identifier for a piece within a design.
export type PieceID = {
    // 🆔 The id of the piece (GraphQL: id_: String)
    id_?: string;
}

// 🪪 Identifier for a port within a type.
export type PortID = {
    // 🆔 The id of the port (GraphQL: id_: String)
    id_?: string;
}

// ⭕ A piece is a 3D instance of a type within a design.
export type Piece = {
    // 🆔 The id of the piece (GraphQL: id_: String)
    id_?: string;
    // 💬 The human-readable description of the piece (GraphQL: description: String!)
    description: string;
    // 🧩 The type defining this piece (GraphQL: type: Type)
    type: TypeID; // Represents Type identifier
    // ◳ The optional plane (position and orientation) of the piece (GraphQL: plane: Plane)
    plane?: Plane;
    // 📺 The center of the piece in the diagram (GraphQL: center: DiagramPoint!)
    center: DiagramPoint;
    // 📏 The qualities associated with the piece (GraphQL: qualities: QualityNodeConnection) - Added field
    qualities?: Quality[];
    // 🖇️ Connections involving this piece (GraphQL: connections: [Connection!]!) - Added field
    connections: Connection[];
}

// 📺 A 2D point (xy) in the diagram coordinate system.
export type DiagramPoint = {
    // 🏁 The x-coordinate in the diagram (GraphQL: x: Float!)
    x: number;
    // 🏁 The y-coordinate in the diagram (GraphQL: y: Float!)
    y: number;
}

// ◳ A plane defined by an origin point and two axes vectors.
export type Plane = {
    // ⌱ The origin point of the plane (GraphQL: origin: Point!)
    origin: Point;
    // ➡️ The x-axis vector of the plane (GraphQL: xAxis: Vector!)
    xAxis: Vector;
    // ➡️ The y-axis vector of the plane (GraphQL: yAxis: Vector!)
    yAxis: Vector;
}

// ✖️ A 3D point (xyz) with floating-point coordinates.
export type Point = {
    // 🎚️ The x-coordinate of the point (GraphQL: x: Float!)
    x: number;
    // 🎚️ The y-coordinate of the point (GraphQL: y: Float!)
    y: number;
    // 🎚️ The z-coordinate of the point (GraphQL: z: Float!)
    z: number;
}

// ➡️ A 3D vector (xyz) with floating-point coordinates.
export type Vector = {
    // 🎚️ The x-coordinate of the vector (GraphQL: x: Float!)
    x: number;
    // 🎚️ The y-coordinate of the vector (GraphQL: y: Float!)
    y: number;
    // 🎚️ The z-coordinate of the vector (GraphQL: z: Float!)
    z: number;
}

// 🪪 Identifier for a type, potentially including a variant.
export type TypeID = {
    // 📛 The name of the type (GraphQL: name: String!)
    name: string;
    // 🔀 The optional variant of the type (GraphQL: variant: String)
    variant?: string;
}

// 📏 Represents a quality, a named property with an optional value, unit, and definition.
export type Quality = {
    // 📛 The name of the quality (GraphQL: name: String!)
    name: string;
    // ❓ The value of the quality (GraphQL: value: String!)
    value: string;
    // 📐 The unit of the quality's value (GraphQL: unit: String!)
    unit: string;
    // 📖 The definition [ text | url ] of the quality (GraphQL: definition: String!)
    definition: string;
}

// 🧩 A type is a reusable element blueprint with ports for connection.
export type Type = {
    // 📛 The name of the type (GraphQL: name: String!)
    name: string;
    // 💬 The human-readable description of the type (GraphQL: description: String!)
    description: string;
    // 🪙 The icon [ emoji | logogram | url ] of the type (GraphQL: icon: String!)
    icon: string;
    // 🖼️ The URL to the image of the type (GraphQL: image: String!)
    image: string;
    // 🔀 The variant of the type (GraphQL: variant: String!)
    variant: string;
    // Ⓜ️ The length unit used by the type's geometry (GraphQL: unit: String!)
    unit: string;
    // 🕒 The creation date of the type (GraphQL: created: DateTime!)
    created: Date;
    // 🕒 The last update date of the type (GraphQL: updated: DateTime!)
    updated: Date;
    // 💾 Representations (e.g., CAD files) of the type (GraphQL: representations: RepresentationNodeConnection)
    representations?: Representation[];
    // 🔌 Connection points (ports) of the type (GraphQL: ports: PortNodeConnection)
    ports?: Port[];
    // 📑 Authors of the type (GraphQL: authors: [Author!]!)
    authors: Author[];
    // 📏 Qualities associated with the type (GraphQL: qualities: QualityNodeConnection)
    qualities?: Quality[];
    // ⭕ Pieces instances of this type (GraphQL: pieces: PieceNodeConnection) - Added field
    pieces?: Piece[];
}

// 🔌 A port is a connection point on a type, defined by a point and direction.
export type Port = {
    // 🆔 The id of the port (GraphQL: id_: String)
    id_?: string;
    // 💬 The human-readable description of the port (GraphQL: description: String!)
    description: string;
    // 👨‍👩‍👧‍👦 The family of the port for compatibility checks (GraphQL: family: String!) - Added field
    family: string;
    // 💍 The parameter t [0,1[ for diagram visualization (GraphQL: t: Float!)
    t: number;
    // ✅ Other compatible port families (GraphQL: compatibleFamilies: [String!]!) - Added field
    compatibleFamilies: string[];
    // ✖️ The connection point geometry (GraphQL: point: Point!)
    point: Point;
    // ➡️ The connection direction vector (GraphQL: direction: Vector!)
    direction: Vector;
    // 📏 Qualities associated with the port (GraphQL: qualities: QualityNodeConnection)
    qualities?: Quality[];
    // 🖇️ Connections involving this port (GraphQL: connections: [Connection!]!) - Added field
    connections: Connection[];
}

// 💾 A representation links to a resource (e.g., file) describing a type.
export type Representation = {
    // 🔗 The URL to the resource (GraphQL: url: String!)
    url: string;
    // 💬 The human-readable description of the representation (GraphQL: description: String!)
    description: string;
    // ✉️ The MIME type of the resource (GraphQL: mime: String!)
    mime: string;
    // 🏷️ Tags to group or filter representations (GraphQL: tags: [String!]!)
    tags: string[];
    // 📏 Qualities associated with the representation (GraphQL: qualities: QualityNodeConnection)
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
