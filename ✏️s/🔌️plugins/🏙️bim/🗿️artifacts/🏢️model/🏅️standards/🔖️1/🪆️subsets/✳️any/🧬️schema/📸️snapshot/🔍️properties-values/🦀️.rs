//! 🔍️ Property and schedule value types of `s.bim.model@1`: property definitions and values, phases and schedule keys.

/// 🏷️ The value type of a property definition: one of the kinds of a property value (text, real, integer, boolean, length in metres, area in square metres, volume in cubic metres, angle in radians).
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum PropertyKind {
    Text,
    Real,
    Integer,
    Boolean,
    Length,
    Area,
    Volume,
    Angle,
}

/// 🎯️ What a property template applies to: an element kind or a type kind. A type's properties are inherited by every instance of the type unless the instance overrides them.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum TemplateTarget {
    Site,
    Building,
    Storey,
    Wall,
    CurtainWall,
    Column,
    Beam,
    Slab,
    Ceiling,
    Roof,
    Window,
    Door,
    Void,
    Stair,
    Ramp,
    Railing,
    Space,
    Zone,
    WallType,
    SlabType,
    CeilingType,
    RoofType,
    ColumnType,
    BeamType,
    WindowType,
    DoorType,
}

/// 🕰️ Construction phase of an element.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum Phase {
    Existing,
    New,
    Demolished,
    Temporary,
}

/// ↪️ Swing side of a door.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum Swing {
    Left,
    Right,
}

/// 📋️ What the rows of a schedule are: the elements of one kind, or (finish) one row per finished surface of every room, or (material) one row per layer or material run of every element.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum ScheduleCategory {
    Wall,
    CurtainWall,
    Slab,
    Roof,
    Column,
    Beam,
    Window,
    Door,
    Void,
    Stair,
    Railing,
    Space,
    Finish,
    Material,
}

/// 🔑️ The closed vocabulary of the built-in schedule fields: authored fields of the element (id, name, kind, storey, level, type, phase, material, host, number, usage, swing, leaves, panes), the measures of its quantity take-off, for finish rows the surface and its area and for material rows the measures of one layer.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum ScheduleField {
    Id,
    Name,
    Kind,
    Storey,
    Level,
    Type,
    Phase,
    Material,
    Host,
    Number,
    Usage,
    Surface,
    Swing,
    Leaves,
    Panes,
    Count,
    Length,
    Width,
    Height,
    Perimeter,
    GrossSideArea,
    OpeningArea,
    NetSideArea,
    GrossArea,
    NetArea,
    SurfaceArea,
    GrossVolume,
    NetVolume,
    Mass,
    Risers,
    Thickness,
    LayerArea,
    LayerVolume,
    LayerMass,
    FinishArea,
    UValue,
    GValue,
}

/// 🔎️ How a schedule filter compares a cell with its value: numbers compare as numbers, everything else as text without regard to case.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum ScheduleOp {
    Equals,
    NotEquals,
    Contains,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
    Empty,
    NotEmpty,
}

/// 🏷️ Typed property value: exactly one measure of the kind the variant names (text, real, integer, boolean, length in metres, area in square metres, volume in cubic metres, angle in radians).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum PropertyValue {
    Text {
        value: String,
    },
    Real {
        value: f64,
    },
    Integer {
        value: i32,
    },
    Boolean {
        value: bool,
    },
    Length {
        value: f64,
    },
    Area {
        value: f64,
    },
    Volume {
        value: f64,
    },
    Angle {
        value: f64,
    },
}

/// 🧾️ The value travels through the text language as its schema value: the externally tagged object of the JSON form.
impl semio_framework_dsl_record::DslField for PropertyValue {
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Value
    }
    fn shape_controlled<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<semio_framework_dsl_record::Shape, semio_framework_value::ValueError> {
        control.checkpoint()?;
        Ok(semio_framework_dsl_record::Shape::Value)
    }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        semio_framework_dsl_record::FieldValue::Value(semio_framework_value::ToValue::to_value(self))
    }
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<semio_framework_dsl_record::FieldValue, semio_framework_value::ValueError> {
        <Self as semio_framework_value::ToValue>::to_value_controlled(self, control).map(semio_framework_dsl_record::FieldValue::Value)
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        match value {
            semio_framework_dsl_record::FieldValue::Value(value) => <Self as semio_framework_value::FromValue>::from_value(value.clone()).map_err(|error| error.to_string()),
            other => Err(format!("expected Value, found {other:?}")),
        }
    }
    fn from_value_controlled(value: &semio_framework_dsl_record::FieldValue, control: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<Self, semio_framework_value::ValueError> {
        match value {
            semio_framework_dsl_record::FieldValue::Value(value) => <Self as semio_framework_value::FromValue>::from_value_controlled(value, control),
            _ => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an intrinsic Value field")),
        }
    }
}

/// 🫳️ The borrowed text schema of an intrinsic value field.
impl semio_framework_dsl_record::BorrowedDslField for PropertyValue {
    const SHAPE: semio_framework_dsl_record::BorrowedShape = semio_framework_dsl_record::BorrowedShape::Value;
}

/// 🔑️ What a schedule cell reads: a built-in field, or one property `property` of the property set `set` of the element (empty where the element has none).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum ScheduleKey {
    Field {
        field: ScheduleField,
    },
    Property {
        set: String,
        name: String,
    },
}

/// 🧾️ The definition of one property of a property set template: its name and value type, an optional unit label, a default, an enumeration of allowed values, a numeric range and whether a value is required.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct PropertyDef {
    /// The property name inside the property set; unique within the template.
    pub name: String,
    /// The value type every value of the property must have.
    pub kind: PropertyKind,
    /// Optional unit label printed beside the value (for example W/(m2.K)); length, area, volume and angle values are in SI units.
    #[value(default, skip_serializing_if = "Option::is_none")] pub unit: Option<String>,
    /// Optional help text of the property.
    #[value(default, skip_serializing_if = "Option::is_none")] pub description: Option<String>,
    /// Whether every element the template applies to must have a value, own, inherited from its type or a default.
    pub required: bool,
    /// Optional value an element has while neither it nor its type states one.
    #[value(default, skip_serializing_if = "Option::is_none")] pub default_value: Option<PropertyValue>,
    /// The enumeration of allowed values; empty allows every value of the kind.
    #[value(default)] pub allowed: Vec<PropertyValue>,
    /// Optional lower bound of a numeric value.
    #[value(default, skip_serializing_if = "Option::is_none")] pub minimum: Option<f64>,
    /// Optional upper bound of a numeric value.
    #[value(default, skip_serializing_if = "Option::is_none")] pub maximum: Option<f64>,
}

/// 🗂️ One row of the entry table of a classification system: a code, its title and the code of its parent row (absent for a root).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct ClassificationItem {
    /// The code inside the system; unique within it.
    pub code: String,
    /// The title of the entry.
    pub title: String,
    /// The code of the parent row; absent for a root; the parents form a forest.
    #[value(default, skip_serializing_if = "Option::is_none")] pub parent: Option<String>,
}

/// 📋️ One column of a schedule: what it reads, an optional heading that replaces the localized name of the key, and whether the column is summed in the group and grand totals.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct ScheduleColumn {
    #[dsl(statements, block)] pub key: ScheduleKey,
    #[value(default, skip_serializing_if = "Option::is_none")] pub heading: Option<String>,
    pub total: bool,
}

/// ↕️ One sort key of a schedule, in order of precedence.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct ScheduleSort {
    #[dsl(statements, block)] pub key: ScheduleKey,
    pub descending: bool,
}

/// 🔎️ One filter of a schedule: a row stays when every filter holds.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct ScheduleFilter {
    #[dsl(statements, block)] pub key: ScheduleKey,
    pub op: ScheduleOp,
    pub value: String,
}

/// 🗂️ One grouping level of a schedule, outermost first.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct ScheduleGroup {
    #[dsl(statements, block)] pub key: ScheduleKey,
}

/// 🏷️ All property sets of one element: property set name → property name → value.
pub type PropertySet = semio_framework_value::retained_clone::ordered_map::RetainedOrderedMap<String, semio_framework_value::retained_clone::ordered_map::RetainedOrderedMap<String, PropertyValue>>;

/// 🗂️ All classifications of one element: classification system id → code in that system (one per system).
pub type ClassificationSet = semio_framework_value::retained_clone::ordered_map::RetainedOrderedMap<String, String>;
