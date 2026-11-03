"""📝 Spec of a schema evolution: collection or leaf, typed fields with units, enum references and EN + DE labels."""
import json
from core import Names, Unsupported, camel, kebab, pascal, snake

INT_MAX = {"u8": 255, "u32": 4294967295}
RUST = {"f64": "f64", "String": "String", "u8": "u8", "u32": "u32"}
TS = {"f64": "number", "String": "string", "u8": "number", "u32": "number"}
UNITS = {
    "m": ("Metre", "m", None, None, 2, 0.1), "m2": ("SquareMetre", "m²", None, None, 2, 0.5), "m3": ("CubicMetre", "m³", None, None, 2, 0.5),
    "Pa": ("Pascal", "Pa", "kN/m²", 0.001, 2, 1000), "N": ("Newton", "N", "kN", 0.001, 1, 1000), "N*m": ("NewtonMetre", "N·m", "kN·m", 0.001, 1, 1000),
    "deg": ("Degree", "°", None, None, 1, 1), "rad": ("Radian", "rad", "°", 57.29577951308232, 1, 0.0174532925), "s": ("Second", "s", None, None, 1, 1),
    "kg": ("Kilogram", "kg", "t", 0.001, 0, 100), "m/s": ("MetrePerSecond", "m/s", None, None, 2, 0.05), "Hz": ("Hertz", "Hz", None, None, 2, 0.05),
    "N/m": ("NewtonPerMetre", "N/m", "kN/m", 0.001, 1, 1000), "W": ("Watt", "W", "MW", 1e-6, 2, 100000), "W/m2": ("WattPerSquareMetre", "W/m²", "kW/m²", 0.001, 1, 1000),
    "°C": ("Celsius", "°C", None, None, 1, 1),
    "": (None, None, None, None, 2, 0.1),
}
DSL_UNITS = {"m", "m2", "Pa", "N", "deg"}
UNIT_DSL = {"m": "m", "m2": "m2", "Pa": "Pa", "N": "N", "deg": "deg"}


def need_labels(label, where):
    if not isinstance(label, dict) or not label.get("en") or not label.get("de"):
        raise Unsupported(f"{where}: both an EN and a DE label are required (got {label!r})")
    return label


class Field:
    def __init__(self, raw, owner=None):
        self.raw = raw
        self.name = raw["name"]
        need_labels(raw.get("label"), f"field {self.name}")
        self.camel, self.pascal = camel(self.name), pascal(self.name)
        self.ty = raw["type"]
        self.unit = raw.get("unit", "")
        if self.ty not in RUST and self.ty != "enum":
            raise Unsupported(f"field {self.name}: type {self.ty!r} is not supported (f64 String u8 u32 enum)")
        if self.unit not in UNITS:
            raise Unsupported(f"field {self.name}: unit {self.unit!r} unknown (known: {', '.join(k for k in UNITS if k)})")
        self.en, self.de = raw["label"]["en"], raw["label"]["de"]
        self.doc = raw.get("doc")
        self.settable = raw.get("settable", True)
        self.enum = raw.get("enum")
        if self.ty == "enum":
            self.enum_name = self.enum["name"]
            self.values = self.enum.get("values", [])
            for v in self.values:
                need_labels(v.get("label"), f"enum {self.enum_name} value {v.get('token')}")
            self.set_values(self.values)
        self.sample = raw.get("sample", self.default_sample())

    def set_values(self, values, variants=None):
        self.values = values
        self.tokens = [v["token"] for v in values]
        self.variants = variants or [pascal(t) for t in self.tokens]
        self.enum_snake = snake(self.enum_name)
        self.enum_const = self.enum_snake.upper()

    def adopt(self, enum_def, variants=None):
        """🧲 An enum that exists already in the snapshot schema keeps its tokens, labels and variant names; the spec only names it."""
        opts = enum_def.get("x-semio-ui", {}).get("options", {})
        self.set_values([{"token": t, "label": opts.get(t, {"en": t, "de": t})} for t in enum_def["enum"]], variants)
        self.sample = self.raw.get("sample", self.tokens[0])

    def default_sample(self):
        return {"f64": 0.0, "String": "", "u8": 0, "u32": 0}.get(self.ty, self.tokens[0] if self.ty == "enum" and self.tokens else None)

    @property
    def is_real(self):
        return self.ty == "f64"

    @property
    def unit_id(self):
        return UNITS[self.unit][0]

    @property
    def rust_ty(self):
        return self.enum_name if self.ty == "enum" else RUST[self.ty]

    def ui(self):
        d = {"label": {"en": self.en, "de": self.de}}
        if self.ty == "String":
            d["widget"] = "text"
            return d
        if self.ty == "enum":
            return d
        d["widget"] = "stepper"
        if self.ty in INT_MAX:
            d.update({"precision": 0, "step": 1})
            return d
        _, sym, display, factor, precision, step = UNITS[self.unit]
        if sym:
            d["unit"] = sym
        if display:
            d["displayUnit"], d["displayFactor"] = display, factor
        d["precision"] = self.raw.get("precision", precision)
        d["step"] = self.raw.get("step", step)
        return d

    def snapshot_node(self):
        if self.ty == "enum":
            node = {"$ref": "#/$defs/" + self.enum_name, "x-semio-ui": self.ui()}
        elif self.ty == "String":
            node = {"type": "string", "x-semio-ui": self.ui()}
        elif self.ty in INT_MAX:
            node = {"type": "integer", "minimum": 0, "maximum": INT_MAX[self.ty], "x-semio-ui": self.ui()}
        else:
            node = {"type": "number", "x-semio-ui": self.ui()}
        if self.doc:
            node["x-semio-rust"] = {"doc": [self.doc]}
        if self.unit in UNIT_DSL:
            node["x-semio-dsl"] = {"unit": UNIT_DSL[self.unit]}
        return node

    def artifact_node(self):
        if self.ty == "enum":
            return {"$ref": "#/$defs/" + self.enum_name}
        if self.ty == "String":
            return {"type": "string"}
        if self.ty in INT_MAX:
            return {"type": "integer", "minimum": 0, "maximum": INT_MAX[self.ty]}
        return {"type": "number", "format": "double"}

    def enum_def(self):
        return {"title": self.enum_name, "type": "string", "enum": self.tokens, "x-semio-ui": {"options": {v["token"]: dict(v["label"]) for v in self.values}}}

    def meta_expr(self, range_=None):
        if self.ty == "enum":
            return f'FieldSpec::choice("{self.en}", "{self.de}", {self.enum_const})'
        if self.is_real and self.unit_id:
            expr = f'FieldSpec::quantity("{self.en}", "{self.de}", UnitId::{self.unit_id})'
        else:
            expr = f'FieldSpec::new("{self.en}", "{self.de}")'
        rng = self.raw.get("range")
        return f"ranged({expr}, {float(rng[0])!r}, {float(rng[1])!r})" if rng else expr


class Entity:
    """🧱 A row type: id column plus fields, optional nested row entities."""

    def __init__(self, raw, plural=None, parent=None):
        self.raw, self.parent = raw, parent
        self.plural = plural or raw["name"]
        self.entity = raw["entity"]
        self.names = Names(self.plural, self.entity)
        self.emoji = raw.get("emoji", "🧱")
        self.doc = raw["doc"]
        self.label = need_labels(raw.get("label"), f"entity {self.entity} label")
        self.id_label = need_labels(raw.get("id"), f"entity {self.entity} id label")
        need_labels(raw.get("singular"), f"entity {self.entity} singular (used in mutation labels)")
        self.fields = [Field(f) for f in raw["fields"]]
        self.nested = [Entity(n, parent=self) for n in raw.get("nested", [])]
        if parent is None and any(n.nested for n in self.nested):
            raise Unsupported("only one nesting level is supported")
        self.enums = [f for f in self.fields if f.ty == "enum"]
        sing = raw.get("singular", {})
        self.singular_en = sing.get("en", self.names.kebab.replace("-", " "))
        self.singular_de = sing.get("de", self.singular_en)

    @property
    def all_enums(self):
        return self.enums + [f for n in self.nested for f in n.enums]

    def sample_row(self):
        row = {"id": self.raw.get("sample_id", self.names.kebab + "-1")}
        for f in self.fields:
            row[f.camel] = f.sample
        for n in self.nested:
            row[camel(n.plural)] = [n.sample_row()]
        return row


class Spec:
    def __init__(self, path):
        self.raw = json.load(open(path, encoding="utf8"))
        self.collection = Entity(self.raw["collection"]) if "collection" in self.raw else None
        self.leaf = None
        if "leaf" in self.raw:
            self.leaf = Field(self.raw["leaf"]["field"])
            self.leaf_after = self.raw["leaf"].get("after")
            self.leaf_default = self.raw["leaf"].get("default", self.leaf.sample)
        self.enum_value = self.raw.get("enumValue")
        self.reference = self.raw.get("reference")
        self.nested_reference = self.raw.get("nestedReference")
        self.kit = self.raw.get("kit")
        self.kind_prefix = self.raw.get("kindPrefix", "change")
