//! 🧰️ The IFC writer kit (IFC 2x3 and IFC4 ADD2 TC1): typed value constructors, a deterministic compressed GUID and an `Ifc` builder over the Part-21 instance allocator.
//! 📎 https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ and https://standards.buildingsmart.org/IFC/RELEASE/IFC4/ADD2_TC1/HTML/

use semio_s_artifact_stdio_ifc::part21::{Part21Builder, Part21Decimal, Part21Value};
use std::collections::BTreeMap;

pub type V = Part21Value;

/// 🔖️ The IFC schema a file is written in.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Schema {
    Ifc2x3,
    Ifc4,
}

impl Schema {
    /// 🏷️ The identifier in `FILE_SCHEMA`.
    pub fn id(self) -> &'static str {
        match self {
            Schema::Ifc2x3 => "IFC2X3",
            Schema::Ifc4 => "IFC4",
        }
    }

    /// 🎚️ `v2x3` for IFC 2x3, `v4` for IFC4.
    pub fn pick<T>(self, v2x3: T, v4: T) -> T {
        match self {
            Schema::Ifc2x3 => v2x3,
            Schema::Ifc4 => v4,
        }
    }
}

//#region 🔖️Values
/// 🔗️ A reference to instance `id`.
pub fn rf(id: u64) -> V {
    V::Ref(id)
}

/// 🔗️ A list of references.
pub fn refs(ids: &[u64]) -> V {
    V::List(ids.iter().map(|id| V::Ref(*id)).collect())
}

/// 🔤️ A string value.
pub fn text(value: impl AsRef<str>) -> V {
    V::Str(value.as_ref().to_string())
}

/// 🔤️ A string value, `$` when empty.
pub fn opt_text(value: &str) -> V {
    if value.is_empty() {
        V::Unset
    } else {
        text(value)
    }
}

/// 🏷️ An enumeration literal such as `.T.`.
pub fn en(value: &str) -> V {
    V::Enum(value.to_string())
}

/// ✅️ A boolean literal.
pub fn flag(value: bool) -> V {
    en(if value { "T" } else { "F" })
}

/// 🔢️ An integer value.
pub fn int(value: i64) -> V {
    V::Int(value)
}

/// 🔢️ A real value rounded to a nanometre so the output carries no float noise.
pub fn real(value: f64) -> V {
    let rounded = (value * 1e9).round() / 1e9;
    V::Real(Part21Decimal::from_f64(if rounded == 0.0 || !rounded.is_finite() { 0.0 } else { rounded }))
}

/// 🔢️ A list of reals.
pub fn reals(values: &[f64]) -> V {
    V::List(values.iter().map(|value| real(*value)).collect())
}

/// 📋️ A list value.
pub fn list(items: Vec<V>) -> V {
    V::List(items)
}

/// 🏷️ A defined-type wrapper such as `IFCLABEL('x')`.
pub fn typed(name: &str, item: V) -> V {
    V::Typed { name: name.to_string(), items: vec![item] }
}

/// 📏️ `IFCLENGTHMEASURE(value)`, for select-typed attributes only.
pub fn length(value: f64) -> V {
    typed("IFCLENGTHMEASURE", real(value))
}

/// 🚫️ The unset value `$`.
pub fn unset() -> V {
    V::Unset
}

/// 🧮️ The derived marker `*`.
pub fn derived() -> V {
    V::Derived
}
//#endregion 🔖️Values

//#region 🔖️GlobalId
const ALPHABET: &[u8; 64] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_$";

fn fnv1a(bytes: &[u8], seed: u64) -> u64 {
    let mut hash = seed;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

fn mix(mut value: u64) -> u64 {
    value ^= value >> 30;
    value = value.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value ^= value >> 27;
    value = value.wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}

/// 🔑️ The 22-character compressed IFC GUID of `key`: a stable 128-bit hash in the buildingSMART base-64 alphabet, so one element id always yields one GlobalId.
pub fn global_id(key: &str) -> String {
    let high = mix(fnv1a(key.as_bytes(), 0xcbf2_9ce4_8422_2325));
    let low = mix(fnv1a(key.as_bytes(), 0x8422_2325_cbf2_9ce4) ^ high.rotate_left(17));
    let mut number = (u128::from(high) << 64) | u128::from(low);
    let mut digits = [0u8; 22];
    for slot in digits.iter_mut().skip(1).rev() {
        *slot = ALPHABET[(number & 63) as usize];
        number >>= 6;
    }
    digits[0] = ALPHABET[(number & 3) as usize];
    digits.iter().map(|digit| char::from(*digit)).collect()
}
//#endregion 🔖️GlobalId

/// 🧭️ A unit vector orthogonal to `axis`: IFC requires `Axis` and `RefDirection` to be given together.
fn orthogonal_to(axis: [f64; 3]) -> [f64; 3] {
    let seed = if axis[0].abs() < 0.9 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
    let length = axis.iter().map(|part| part * part).sum::<f64>().sqrt().max(1e-12);
    let unit = axis.map(|part| part / length);
    let along: f64 = (0..3).map(|row| seed[row] * unit[row]).sum();
    let raw = [0, 1, 2].map(|row| seed[row] - along * unit[row]);
    let norm = raw.iter().map(|part| part * part).sum::<f64>().sqrt().max(1e-12);
    raw.map(|part| part / norm)
}

//#region 🔖️Builder
/// 🏗️ The IFC instance graph under construction plus the shared context entities every product refers to.
pub struct Ifc {
    builder: Part21Builder,
    shared: BTreeMap<String, u64>,
    pub schema: Schema,
    pub owner: u64,
    pub context: u64,
    pub body: u64,
    pub axis: u64,
    pub footprint: u64,
    pub origin: u64,
}

impl Ifc {
    /// 🌱️ The shared context of an IFC 2x3 file (see [`Ifc::in_schema`]).
    pub fn new(author: &str, organization: &str, true_north: f64) -> Self {
        Self::in_schema(Schema::Ifc2x3, author, organization, true_north)
    }

    /// 🌱️ Allocates the model context with its body, axis and footprint sub-contexts and the world origin; IFC 2x3 also allocates the owner history it requires on every rooted entity, IFC4 writes none (its `OwnerHistory` is optional and the author
    /// and organization travel in the file header).
    pub fn in_schema(schema: Schema, author: &str, organization: &str, true_north: f64) -> Self {
        let mut ifc = Self { builder: Part21Builder::new(), shared: BTreeMap::new(), schema, owner: 0, context: 0, body: 0, axis: 0, footprint: 0, origin: 0 };
        if schema == Schema::Ifc2x3 {
            let person = ifc.add("IFCPERSON", vec![unset(), text(author), unset(), unset(), unset(), unset(), unset(), unset()]);
            let organisation = ifc.add("IFCORGANIZATION", vec![unset(), text(organization), unset(), unset(), unset()]);
            let account = ifc.add("IFCPERSONANDORGANIZATION", vec![rf(person), rf(organisation), unset()]);
            let application = ifc.add("IFCAPPLICATION", vec![rf(organisation), text("1"), text("semio BIM"), text("semio.bim")]);
            ifc.owner = ifc.add("IFCOWNERHISTORY", vec![rf(account), rf(application), unset(), en("ADDED"), unset(), unset(), unset(), int(0)]);
        }
        ifc.origin = ifc.axis3([0.0, 0.0, 0.0], None, None);
        let north = ifc.dir2([-true_north.sin(), true_north.cos()]);
        ifc.context = ifc.add("IFCGEOMETRICREPRESENTATIONCONTEXT", vec![unset(), text("Model"), int(3), real(1e-6), rf(ifc.origin), rf(north)]);
        let context = ifc.context;
        let sub = |ifc: &mut Self, identifier: &str, view: &str| ifc.add("IFCGEOMETRICREPRESENTATIONSUBCONTEXT", vec![text(identifier), text("Model"), derived(), derived(), derived(), derived(), rf(context), unset(), en(view), unset()]);
        ifc.body = sub(&mut ifc, "Body", "MODEL_VIEW");
        ifc.axis = sub(&mut ifc, "Axis", "GRAPH_VIEW");
        ifc.footprint = sub(&mut ifc, "FootPrint", "PLAN_VIEW");
        ifc
    }

    /// 📏️ The `IfcUnitAssignment` of the SI units the file uses: metre, square metre, cubic metre and radian.
    pub fn units(&mut self) -> u64 {
        if let Some(id) = self.shared.get("units") {
            return *id;
        }
        let rows = [("LENGTHUNIT", "METRE"), ("AREAUNIT", "SQUARE_METRE"), ("VOLUMEUNIT", "CUBIC_METRE"), ("PLANEANGLEUNIT", "RADIAN")];
        let ids: Vec<u64> = rows.iter().map(|(unit, name)| self.add("IFCSIUNIT", vec![derived(), en(unit), unset(), en(name)])).collect();
        let assignment = self.add("IFCUNITASSIGNMENT", vec![refs(&ids)]);
        self.shared.insert("units".to_string(), assignment);
        assignment
    }

    /// ➕️ Allocates one instance.
    pub fn add(&mut self, entity: &str, args: Vec<V>) -> u64 {
        self.builder.alloc(entity, args)
    }

    /// ♻️ Allocates one instance, or returns the id of an identical earlier one.
    pub fn once(&mut self, entity: &str, args: Vec<V>) -> u64 {
        let key = format!("{entity}{args:?}");
        if let Some(id) = self.shared.get(&key) {
            return *id;
        }
        let id = self.add(entity, args);
        self.shared.insert(key, id);
        id
    }

    /// 🏷️ Allocates a rooted instance: `GlobalId` from `guid_key`, the shared owner history, `Name`, `Description`, then `tail`.
    pub fn rooted(&mut self, entity: &str, guid_key: &str, name: &str, description: &str, tail: Vec<V>) -> u64 {
        let owner = if self.owner == 0 { unset() } else { rf(self.owner) };
        let mut args = vec![text(global_id(&format!("{entity}:{guid_key}"))), owner, opt_text(name), opt_text(description)];
        args.extend(tail);
        self.add(entity, args)
    }

    /// 📍️ A shared `IfcCartesianPoint` in space.
    pub fn point3(&mut self, point: [f64; 3]) -> u64 {
        self.once("IFCCARTESIANPOINT", vec![reals(&point)])
    }

    /// 📍️ A shared `IfcCartesianPoint` in the plane.
    pub fn point2(&mut self, point: [f64; 2]) -> u64 {
        self.once("IFCCARTESIANPOINT", vec![reals(&point)])
    }

    /// 🧭️ A shared `IfcDirection` in space.
    pub fn dir3(&mut self, direction: [f64; 3]) -> u64 {
        self.once("IFCDIRECTION", vec![reals(&direction)])
    }

    /// 🧭️ A shared `IfcDirection` in the plane.
    pub fn dir2(&mut self, direction: [f64; 2]) -> u64 {
        self.once("IFCDIRECTION", vec![reals(&direction)])
    }

    /// 📐️ A shared `IfcAxis2Placement3D`; `axis` is the local z, `ref_direction` the local x.
    pub fn axis3(&mut self, location: [f64; 3], axis: Option<[f64; 3]>, ref_direction: Option<[f64; 3]>) -> u64 {
        let (axis, ref_direction) = match (axis, ref_direction) {
            (Some(axis), None) => (Some(axis), Some(orthogonal_to(axis))),
            (None, Some(reference)) => (Some([0.0, 0.0, 1.0]), Some(reference)),
            other => other,
        };
        let location = self.point3(location);
        let axis = axis.map_or(unset(), |value| rf(self.dir3(value)));
        let ref_direction = ref_direction.map_or(unset(), |value| rf(self.dir3(value)));
        self.once("IFCAXIS2PLACEMENT3D", vec![rf(location), axis, ref_direction])
    }

    /// 📐️ A shared `IfcAxis2Placement2D`.
    pub fn axis2(&mut self, location: [f64; 2], ref_direction: Option<[f64; 2]>) -> u64 {
        let location = self.point2(location);
        let ref_direction = ref_direction.map_or(unset(), |value| rf(self.dir2(value)));
        self.once("IFCAXIS2PLACEMENT2D", vec![rf(location), ref_direction])
    }

    /// 📌️ An `IfcLocalPlacement` of `placement` relative to `relative_to`.
    pub fn place(&mut self, relative_to: Option<u64>, placement: u64) -> u64 {
        self.add("IFCLOCALPLACEMENT", vec![relative_to.map_or(unset(), rf), rf(placement)])
    }

    /// 🧱️ An `IfcShapeRepresentation` in `context`.
    pub fn shape(&mut self, context: u64, identifier: &str, kind: &str, items: &[u64]) -> u64 {
        self.add("IFCSHAPEREPRESENTATION", vec![rf(context), text(identifier), text(kind), refs(items)])
    }

    /// 🧱️ An `IfcProductDefinitionShape` of `representations`.
    pub fn definition(&mut self, representations: &[u64]) -> u64 {
        self.add("IFCPRODUCTDEFINITIONSHAPE", vec![unset(), unset(), refs(representations)])
    }

    /// 🧊️ An `IfcExtrudedAreaSolid` of `profile` along `direction` at `position`.
    pub fn extrusion(&mut self, profile: u64, position: u64, direction: [f64; 3], depth: f64) -> u64 {
        let direction = self.dir3(direction);
        self.add("IFCEXTRUDEDAREASOLID", vec![rf(profile), rf(position), rf(direction), real(depth)])
    }

    /// ▭️ An `IfcRectangleProfileDef` centred at `centre`.
    pub fn rectangle(&mut self, centre: [f64; 2], width: f64, depth: f64) -> u64 {
        let position = self.axis2(centre, None);
        self.add("IFCRECTANGLEPROFILEDEF", vec![en("AREA"), unset(), rf(position), real(width), real(depth)])
    }

    /// ⭕️ An `IfcCircleProfileDef` centred at the origin.
    pub fn circle(&mut self, diameter: f64) -> u64 {
        let position = self.axis2([0.0, 0.0], None);
        self.add("IFCCIRCLEPROFILEDEF", vec![en("AREA"), unset(), rf(position), real(diameter / 2.0)])
    }

    /// 🏗️ An `IfcIShapeProfileDef` centred at the origin.
    pub fn i_shape(&mut self, width: f64, depth: f64, web: f64, flange: f64) -> u64 {
        let position = self.axis2([0.0, 0.0], None);
        let tail = self.schema.pick(vec![unset()], vec![unset(), unset(), unset()]);
        let mut args = vec![en("AREA"), unset(), rf(position), real(width), real(depth), real(web), real(flange)];
        args.extend(tail);
        self.add("IFCISHAPEPROFILEDEF", args)
    }

    /// 🔷️ An arbitrary profile bounded by `outer` with `inner` voids.
    pub fn curve_profile(&mut self, outer: u64, inner: &[u64]) -> u64 {
        if inner.is_empty() {
            self.add("IFCARBITRARYCLOSEDPROFILEDEF", vec![en("AREA"), unset(), rf(outer)])
        } else {
            self.add("IFCARBITRARYPROFILEDEFWITHVOIDS", vec![en("AREA"), unset(), rf(outer), refs(inner)])
        }
    }

    /// 〰️ A closed curve through bulged `vertices`: an `IfcPolyline` when every edge is straight, else an `IfcCompositeCurve` of lines and trimmed circles.
    pub fn loop_curve(&mut self, vertices: &[([f64; 2], f64)]) -> u64 {
        if vertices.iter().all(|(_, bulge)| bulge.abs() < 1e-12) {
            let mut points: Vec<u64> = vertices.iter().map(|(point, _)| self.point2(*point)).collect();
            if let Some(first) = points.first().copied() {
                points.push(first);
            }
            return self.add("IFCPOLYLINE", vec![refs(&points)]);
        }
        let mut segments = Vec::new();
        for (index, (start, bulge)) in vertices.iter().enumerate() {
            let end = vertices[(index + 1) % vertices.len()].0;
            let parent = self.edge_curve(*start, end, *bulge);
            segments.push(self.add("IFCCOMPOSITECURVESEGMENT", vec![en("CONTINUOUS"), flag(true), rf(parent)]));
        }
        self.add("IFCCOMPOSITECURVE", vec![refs(&segments), flag(false)])
    }

    /// 〰️ The curve of one bulged edge: a two-point polyline, or a trimmed circle through its end points.
    pub fn edge_curve(&mut self, start: [f64; 2], end: [f64; 2], bulge: f64) -> u64 {
        let (first, second) = (self.point2(start), self.point2(end));
        if bulge.abs() < 1e-12 {
            return self.add("IFCPOLYLINE", vec![refs(&[first, second])]);
        }
        let chord = (end[0] - start[0]).hypot(end[1] - start[1]);
        let sweep = 4.0 * bulge.atan();
        let radius = chord / (2.0 * (sweep / 2.0).sin().abs());
        let (mid, direction) = ([(start[0] + end[0]) / 2.0, (start[1] + end[1]) / 2.0], [(end[0] - start[0]) / chord, (end[1] - start[1]) / chord]);
        let distance = radius * (sweep / 2.0).cos();
        let normal = [-direction[1] * bulge.signum(), direction[0] * bulge.signum()];
        let centre = [mid[0] + normal[0] * distance, mid[1] + normal[1] * distance];
        let position = self.axis2(centre, None);
        let circle = self.add("IFCCIRCLE", vec![rf(position), real(radius)]);
        self.add("IFCTRIMMEDCURVE", vec![rf(circle), list(vec![rf(first)]), list(vec![rf(second)]), flag(bulge > 0.0), en("CARTESIAN")])
    }

    /// 🔗️ An `IfcRelAggregates` of `parts` under `whole`.
    pub fn aggregate(&mut self, key: &str, whole: u64, parts: &[u64]) -> u64 {
        self.rooted("IFCRELAGGREGATES", key, "", "", vec![rf(whole), refs(parts)])
    }

    /// 🧱️ The finished instance graph.
    pub fn finish(self, header: semio_s_artifact_stdio_ifc::part21::Part21Header) -> semio_s_artifact_stdio_ifc::part21::Part21Document {
        self.builder.build(header)
    }
}
//#endregion 🔖️Builder

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
