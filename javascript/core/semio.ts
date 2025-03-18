// TODOs
// Update to latest schema and unify docstrings

// Initially created from json-schema-to-typescript: https://app.quicktype.io/
// Manually edited.

// To parse this data:
//
//   import { Convert, Kit } from "./file";
//
//   const kit = Convert.toKit(json);
//
// These functions will throw an error if the JSON doesn't
// match the expected interface, even if the JSON is valid.

export const ICON_WIDTH = 50;

// ↗️ The output of a kit.
export type Kit = {
    // 🕒 The creation date of the kit.
    created?: Date;
    // 💬 The optional human-readable description of the kit.
    description?: string;
    // 🏙️ The designs of the kit.
    designs?: Design[];
    // 🏠 The optional url of the homepage of the kit.
    homepage?: string;
    // 🪙 The optional icon [ emoji | logogram | url ] of the kit. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 256x256 pixels and smaller than 1 MB. kit.
    icon?: string;
    // 🖼️ The optional url to the image of the kit. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 720x720 pixels and smaller than 5 MB.
    image?: string;
    // 🕒 The last update date of the kit.
    updated?: Date;
    // ⚖️ The optional license [ spdx id | url ] of the kit.
    license?: string;
    // 📛 The name of the kit.
    name: string;
    // 🔮 The optional url of the preview image of the kit. The url must point to a landscape image [ png | jpg | svg ] which will be cropped by a 2x1 rectangle. The image must be at least 1920x960 pixels and smaller than 15 MB.
    preview?: string;
    // ☁️ The optional Unique Resource Locator (URL) where to fetch the kit remotely.
    remote?: string;
    // 🧩 The types of the kit.
    types?: Type[];
    // 🆔 The uri of the kit.
    uri: string;
    // 🔀 The optional version of the kit. No version means the latest version.
    version?: string;
}

// 🏙️ A design is a collection of pieces that are connected.
export type Design = {
    authors?: Author[];
    connections?: Connection[];
    // 🕒 The creation date of the design.
    created?: Date;
    // 💬 The optional human-readable description of the design.
    description?: string;
    // 🪙 The optional icon [ emoji | logogram | url ] of the design. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 256x256 pixels and smaller than 1 MB. The image must be at least 256x256 pixels and smaller than 1 MB.
    icon?: string;
    // 🖼️ The optional url to the image of the design. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 720x720 pixels and smaller than 5 MB.
    image?: string;
    // 🕒 The last update date of the design.
    updated?: Date;
    // 📛 The name of the design.
    name: string;
    pieces?: Piece[];
    qualities?: Quality[];
    // 📏 The unit of the design.
    unit?: string;
    // 🔀 The optional variant of the design. No variant means the default variant.
    variant?: string;
    // 🥽 The optional view of the design. No view means the default view.
    view?: string;
}

// 📑 The output of an author.
export type Author = {
    // 📧 The email of the author.
    email: string;
    // 📛 The name of the author.
    name: string;
}

// 🖇️ A bidirectional connection between two pieces of a design.
export type Connection = {
    // 🧲 The connected side of the connection.
    connected: Side;
    // 🧲 The connecting side of the connection.
    connecting: Side;
    // 💬 The optional human-readable description of the connection.
    description?: string;
    // ↕️ The optional longitudinal gap (applied after rotation and tilt in port direction) between the connected and the connecting piece.
    gap?: number;
    // 🔄 The optional horizontal rotation in port direction between the connected and the connecting piece in degrees.
    rotation?: number;
    // ↔️ The optional lateral shift (applied after rotation and tilt in the plane) between the connected and the connecting piece..
    shift?: number;
    // ↗️ The optional horizontal tilt perpendicular to the port direction (applied after rotation) between the connected and the connecting piece in degrees.
    tilt?: number;
    // ➡️ The optional offset in x direction between the icons of the child and the parent piece in the diagram. One unit is equal the width of a piece icon.
    x?: number;
    // ⬆️ The optional offset in y direction between the icons of the child and the parent piece in the diagram. One unit is equal the width of a piece icon.
    y?: number;
}

// 🧱 A side of a piece in a connection.
export type Side = {
    piece: PieceID;
    port: PortID;
}

// 🪪 The props to identify the piece within the parent design.
export type PieceID = {
    // 🆔 The id of the piece.
    id_?: string;
}

// 🪪 The props to identify the port within the parent type.
export type PortID = {
    // 🆔 The id of the port.
    id_?: string;
}

// ⭕ A piece is a 3d-instance of a type in a design.
export type Piece = {
    // 📺 The optional center of the piece in the diagram. When pieces are connected only one piece can have a center.
    center?: null | DiagramPoint;
    // 🆔 The id of the piece.
    id_?: string;
    // 💬 The optional human-readable description of the piece.
    description?: string;
    // ◳ The optional plane of the piece. When pieces are connected only one piece can have a plane.
    plane?: null | Plane;
    // 🧩 The type of the piece.
    type: TypeID;
}

// 📺 A 2d-point (xy) of integers in screen coordinate system.
export type DiagramPoint = {
    // 🏁 The x-coordinate of the icon of the piece in the diagram. One unit is equal the width of a piece icon.
    x: number;
    // 🏁 The y-coordinate of the icon of the piece in the diagram. One unit is equal the width of a piece icon.
    y: number;
}

// ◳ A plane is an origin (point) and an orientation (x-axis and y-axis).
export type Plane = {
    // ⌱ The origin of the plane.
    origin: Point;
    // ➡️ The x-axis of the plane.
    xAxis: Vector;
    // ➡️ The y-axis of the plane.
    yAxis: Vector;
}

// ✖️ A 3d-point (xyz) of floating point numbers.
export type Point = {
    // 🎚️ The x-coordinate of the point.
    x: number;
    // 🎚️ The y-coordinate of the point.
    y: number;
    // 🎚️ The z-coordinate of the point.
    z: number;
}

// ➡️ A 3d-vector (xyz) of floating point numbers.
export type Vector = {
    // 🎚️ The x-coordinate of the vector.
    x: number;
    // 🎚️ The y-coordinate of the vector.
    y: number;
    // 🎚️ The z-coordinate of the vector.
    z: number;
}

// 🧩 The type of the piece.
//
// 🪪 The props to identify the type.
export type TypeID = {
    // 📛 The name of the type.
    name: string;
    // 🔀 The optional variant of the type. No variant means the default variant.
    variant?: string;
}

// ↗️ The output of a quality.
export type Quality = {
    // 📏 The optional definition [ text | url ] of the quality.
    definition?: string;
    // 📏 The name of the quality.
    name: string;
    // 📏 The optional unit of the value of the quality.
    unit?: string;
    // 📏 The optional value [ text | url ] of the quality. No value is equivalent to true for the name.
    value?: string;
}

// 🧩 A type is a reusable element that can be connected with other types over ports.
export type Type = {
    authors?: Author[];
    // 🕒 The creation date of the type.
    created?: Date;
    // 💬 The optional human-readable description of the type.
    description?: string;
    // 🪙 The optional icon [ emoji | logogram | url ] of the type. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 256x256 pixels and smaller than 1 MB.
    icon?: string;
    // 🖼️ The optional url to the image of the type. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 720x720 pixels and smaller than 5 MB.
    image?: string;
    // 🕒 The last update date of the type.
    updated?: Date;
    // 📛 The name of the type.
    name: string;
    ports?: Port[];
    qualities?: Quality[];
    representations?: Representation[];
    // Ⓜ️ The length unit of the point and the direction of the ports of the type.
    unit?: string;
    // 🔀 The optional variant of the type. No variant means the default variant.
    variant?: string;
}

// 🔌 A port is a connection point (with a direction) of a type.
export type Port = {
    // 💬 The optional human-readable description of the port.
    description?: string;
    // ➡️ The direction of the port. When another piece connects the direction of the other port is flipped and then the pieces are aligned.
    direction: Vector;
    // 🆔 The id of the port.
    id_?: string;
    // 🗺️ The locators of the port.
    locators?: Locator[];
    // ✖️ The connection point of the port that is attracted to another connection point.
    point: Point;
    // 💍 The parameter t [0,1[ where the port will be shown on the ring of a piece in the diagram. It starts at 12 o`clock and turns clockwise.
    t?: number;
    // 📏 The optional qualities of the port.
    qualities?: Quality[];
}

// 💾 A representation is a link to a resource that describes a type for a certain level of detail and tags.
export type Representation = {
    // 🔗 The Unique Resource Locator (URL) to the resource of the representation.
    url: string;
    // 💬 The optional human-readable description of the representation.
    description?: string;
    // ✉️ The Multipurpose Internet Mail Extensions (MIME) type of the content of the resource of the representation.
    mime: string;
    // 🏷️ The optional tags to group representations. No tags means default.
    tags?: string[];
    // 📏 The optional qualities of the representation.
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
        const d = new Date(val);
        if (isNaN(d.valueOf())) {
            return invalidValue(l("Date"), val, key, parent);
        }
        return d;
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
                result[key] = transform(val[key], additional, getProps, key, ref);
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
    let ref: any = undefined;
    while (typeof typ === "object" && typ.ref !== undefined) {
        ref = typ.ref;
        typ = typeMap[typ.ref];
    }
    if (Array.isArray(typ)) return transformEnum(typ, val);
    if (typeof typ === "object") {
        return typ.hasOwnProperty("unionMembers") ? transformUnion(typ.unionMembers, val)
            : typ.hasOwnProperty("arrayItems") ? transformArray(typ.arrayItems, val)
                : typ.hasOwnProperty("props") ? transformObject(getProps(typ), typ.additional, val)
                    : invalidValue(typ, val, key, parent);
    }
    // Numbers can be parsed by Date but shouldn't be.
    if (typ === Date && typeof val !== "number") return transformDate(val);
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

function o(props: any[], additional: any) {
    return { props, additional };
}

function m(additional: any) {
    return { props: [], additional };
}

function r(name: string) {
    return { ref: name };
}

const typeMap: any = {
    "Kit": o([
        { json: "created", js: "created", typ: u(undefined, Date) },
        { json: "description", js: "description", typ: u(undefined, "") },
        { json: "designs", js: "designs", typ: u(undefined, a(r("Design"))) },
        { json: "homepage", js: "homepage", typ: u(undefined, "") },
        { json: "icon", js: "icon", typ: u(undefined, "") },
        { json: "image", js: "image", typ: u(undefined, "") },
        { json: "updated", js: "updated", typ: u(undefined, Date) },
        { json: "license", js: "license", typ: u(undefined, "") },
        { json: "name", js: "name", typ: "" },
        { json: "preview", js: "preview", typ: u(undefined, "") },
        { json: "remote", js: "remote", typ: u(undefined, "") },
        { json: "types", js: "types", typ: u(undefined, a(r("Type"))) },
        { json: "uri", js: "uri", typ: "" },
        { json: "version", js: "version", typ: u(undefined, "") },
    ], "any"),
    "Design": o([
        { json: "authors", js: "authors", typ: u(undefined, a(r("Author"))) },
        { json: "connections", js: "connections", typ: u(undefined, a(r("Connection"))) },
        { json: "created", js: "created", typ: u(undefined, Date) },
        { json: "description", js: "description", typ: u(undefined, "") },
        { json: "icon", js: "icon", typ: u(undefined, "") },
        { json: "image", js: "image", typ: u(undefined, "") },
        { json: "updated", js: "updated", typ: u(undefined, Date) },
        { json: "name", js: "name", typ: "" },
        { json: "pieces", js: "pieces", typ: u(undefined, a(r("Piece"))) },
        { json: "qualities", js: "qualities", typ: u(undefined, a(r("Quality"))) },
        { json: "unit", js: "unit", typ: u(undefined, "") },
        { json: "variant", js: "variant", typ: u(undefined, "") },
        { json: "view", js: "view", typ: u(undefined, "") },
    ], "any"),
    "Author": o([
        { json: "email", js: "email", typ: "" },
        { json: "name", js: "name", typ: "" },
    ], "any"),
    "Connection": o([
        { json: "connected", js: "connected", typ: r("Side") },
        { json: "connecting", js: "connecting", typ: r("Side") },
        { json: "gap", js: "gap", typ: u(undefined, 3.14) },
        { json: "rotation", js: "rotation", typ: u(undefined, 3.14) },
        { json: "shift", js: "shift", typ: u(undefined, 3.14) },
        { json: "tilt", js: "tilt", typ: u(undefined, 3.14) },
        { json: "x", js: "x", typ: u(undefined, 3.14) },
        { json: "y", js: "y", typ: u(undefined, 3.14) },
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
        { json: "center", js: "center", typ: u(undefined, u(null, r("DiagramPoint"))) },
        { json: "description", js: "description", typ: u(undefined, "") },
        { json: "id_", js: "id_", typ: u(undefined, "") },
        { json: "plane", js: "plane", typ: u(undefined, u(null, r("Plane"))) },
        { json: "type", js: "type", typ: r("TypeID") },
    ], "any"),
    "DiagramPoint": o([
        { json: "x", js: "x", typ: 3.14 },
        { json: "y", js: "y", typ: 3.14 },
    ], "any"),
    "Plane": o([
        { json: "origin", js: "origin", typ: r("Point") },
        { json: "xAxis", js: "xAxis", typ: r("Vector") },
        { json: "yAxis", js: "yAxis", typ: r("Vector") },
    ], "any"),
    "Point": o([
        { json: "x", js: "x", typ: 3.14 },
        { json: "y", js: "y", typ: 3.14 },
        { json: "z", js: "z", typ: 3.14 },
    ], "any"),
    "Vector": o([
        { json: "x", js: "x", typ: 3.14 },
        { json: "y", js: "y", typ: 3.14 },
        { json: "z", js: "z", typ: 3.14 },
    ], "any"),
    "TypeID": o([
        { json: "name", js: "name", typ: "" },
        { json: "variant", js: "variant", typ: u(undefined, "") },
    ], "any"),
    "Quality": o([
        { json: "definition", js: "definition", typ: u(undefined, "") },
        { json: "name", js: "name", typ: "" },
        { json: "unit", js: "unit", typ: u(undefined, "") },
        { json: "value", js: "value", typ: u(undefined, "") },
    ], "any"),
    "Type": o([
        { json: "authors", js: "authors", typ: u(undefined, a(r("Author"))) },
        { json: "created", js: "created", typ: u(undefined, Date) },
        { json: "description", js: "description", typ: u(undefined, "") },
        { json: "icon", js: "icon", typ: u(undefined, "") },
        { json: "image", js: "image", typ: u(undefined, "") },
        { json: "updated", js: "updated", typ: u(undefined, Date) },
        { json: "name", js: "name", typ: "" },
        { json: "ports", js: "ports", typ: u(undefined, a(r("Port"))) },
        { json: "qualities", js: "qualities", typ: u(undefined, a(r("Quality"))) },
        { json: "representations", js: "representations", typ: u(undefined, a(r("Representation"))) },
        { json: "unit", js: "unit", typ: u(undefined, "") },
        { json: "variant", js: "variant", typ: u(undefined, "") },
    ], "any"),
    "Port": o([
        { json: "description", js: "description", typ: u(undefined, "") },
        { json: "direction", js: "direction", typ: r("Vector") },
        { json: "id_", js: "id_", typ: u(undefined, "") },
        { json: "point", js: "point", typ: r("Point") },
        { json: "t", js: "t", typ: u(undefined, 3.14) },
        { json: "qualities", js: "qualities", typ: u(undefined, a(r("Quality"))) },
    ], "any"),
    "Representation": o([
        { json: "url", js: "url", typ: "" },
        { json: "description", js: "description", typ: u(undefined, "") },
        { json: "mime", js: "mime", typ: "" },
        { json: "tags", js: "tags", typ: u(undefined, a("")) },
        { json: "qualities", js: "qualities", typ: u(undefined, a(r("Quality"))) },
    ], "any"),
};
