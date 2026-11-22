"""🏛️ Independently verify the IFC4 projection of authored catalogue scalar properties."""
import json
import os
from pathlib import Path
import subprocess
import sys

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
output = ticket / "🗑️generated/ifc-library"
fixture = json.loads((ticket / "iso16757-ifc-scalar-properties.json").read_text())
r = next((root/"✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757").glob("🏅*/🔖*/🪆*/✳*"))
exporter = r/"🧬️schema/💡️inferences/⚖️checks/🧰️check-builders/🧩️part-libraries/🏛️ifc-library/🦀️.rs"
part21 = root/"✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📐️part21/✍️writer/🦀️.rs"
hashes = root/"🧰️framework/🔨️modules/🔏️hash/🦀️.rs"
header_source = (part21.parent.parent/"📄️document-model/🦀️.rs").read_text()
begin = header_source.index("pub struct Part21Header")
header = "#[derive(Clone)]\n" + header_source[begin:header_source.index("impl Default for Part21Header",begin)].replace("pub(super)", "pub(crate)")

quoted = lambda value: json.dumps(value,ensure_ascii=False)
probe = output/"scalar-properties.rs"
binary = output/"scalar-properties"
step = output/"scalar-properties.ifc"
probe.write_text(r'''
extern crate self as semio_framework_hash;
extern crate self as semio_s_artifact_norm_iso16757_model;
extern crate self as semio_s_artifact_stdio_contract_part21;
#[allow(dead_code)]
pub enum CatalogueValue {
    Boolean { value: bool }, Integer { value: i64 }, Decimal { value: f64 },
    Text { value: String }, Identifier { value: String }, Enumeration { value: String },
    Controlled { value: String, list_id: String }, Quantity { value: f64 },
}
pub mod part_1 {
    use super::CatalogueValue;
    pub struct Id(pub String);
    pub struct DisplayName { pub text: String }
    pub struct Names { pub preferred: DisplayName }
    pub struct Metadata { pub names: Names }
    pub struct Class { pub id: String, pub names: Names }
    pub struct ProductSeries { pub id: String, pub class_id: String, pub shared_property_values: Vec<(String, CatalogueValue)> }
    pub struct PropertyDefinition { pub id: String, pub names: Names, pub unit: Option<()>, pub dictionary_property_id: Option<String> }
    pub struct PropertyValue { pub definition_id: String, pub value: CatalogueValue, pub function_id: Option<String> }
    pub struct Product { pub id: String, pub series_id: String, pub names: Names, pub static_properties: Vec<PropertyValue> }
    pub struct Dictionary { pub id: String, pub version: String }
    pub struct Catalogue {
        pub dictionary: Dictionary, pub id: Id, pub metadata: Metadata, pub product_classes: Vec<Class>,
        pub product_series: Vec<ProductSeries>, pub products: Vec<Product>, pub property_definitions: Vec<PropertyDefinition>,
    }
}
#[derive(Clone)]
pub enum Part21Value { Str(String), Binary(String), Enum(String), Int(i64), Real(f64), List(Vec<Self>), Typed { name: String, items: Vec<Self> }, Unset, Derived, Ref(u64) }
pub struct Part21Instance { pub id: u64, pub entities: Vec<(String, Vec<Part21Value>)> }
pub struct Part21Document { pub header: Part21Header, pub instances: Vec<Part21Instance> }
use std::fmt::Write as _;
''' + header + r'''
pub struct Part21Builder { pub instances: Vec<Part21Instance>, next_id: u64 }
impl Part21Builder {
    pub fn new() -> Self { Self { instances: Vec::new(), next_id: 1 } }
    pub fn alloc(&mut self, name: &str, args: Vec<Part21Value>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.instances.push(Part21Instance { id, entities: vec![(name.into(), args)] });
        id
    }
    pub fn build(self, header: Part21Header) -> Part21Document { Part21Document { header, instances: self.instances } }
}
''' + f'#[path={quoted(str(part21))}] mod writer;\npub use writer::*;\n'
    + f'#[path={quoted(str(hashes))}] mod hashes;\npub use hashes::Sha256;\n'
    + f'#[path={quoted(str(exporter))}] mod library;\n'
    + r'''
use part_1::*;
fn names(text: &str) -> Names { Names { preferred: DisplayName { text: text.into() } } }
fn value(id: &str, value: CatalogueValue) -> PropertyValue {
    PropertyValue { definition_id: id.into(), value, function_id: None }
}
fn main() {
    use CatalogueValue::*;
    let mut doc = Catalogue {
        id: Id("heating-ventilation".into()),
        metadata: Metadata { names: names("Scalar catalogue") },
        dictionary: Dictionary { id: DICTIONARY.into(), version: "2025.1".into() },
        product_classes: vec![Class { id: "sensors".into(), names: names(SOURCE) }],
        product_series: vec![ProductSeries {
            id: "sensor-series".into(), class_id: "sensors".into(),
            shared_property_values: vec![("enabled".into(), Boolean { value: true }), ("sensors".into(), Integer { value: SHARED })],
        }],
        property_definitions: vec![
            PropertyDefinition { id: "sensors".into(), names: names("Count"), unit: None, dictionary_property_id: Some(GUID.into()) },
            PropertyDefinition { id: "status".into(), names: names("Status"), unit: None, dictionary_property_id: Some(INVALID_GUID.into()) },
            PropertyDefinition { id: "physical".into(), names: names("Length"), unit: Some(()), dictionary_property_id: None },
        ],
        products: vec![
            Product { id: "probe-a".into(), series_id: "sensor-series".into(), names: names("Probe A"), static_properties: vec![
                value("sensors", Integer { value: OVERRIDE }),
                value("description", Text { value: TEXT.into() }),
                value("measurement", Decimal { value: DECIMAL }),
                value("physical", Decimal { value: DECIMAL }),
                value("enabled", Boolean { value: false }),
                value("status", Enumeration { value: STATUS.into() }),
            ] },
            Product { id: "probe-b".into(), series_id: "sensor-series".into(), names: names("Probe B"), static_properties: Vec::new() },
        ],
    };
    let rendered = library::export_ifc_step(&doc).unwrap();
    doc.products[0].static_properties.reverse();
    doc.products.reverse();
    doc.product_series[0].shared_property_values.reverse();
    assert_eq!(rendered, library::export_ifc_step(&doc).unwrap());
    doc.products[0].static_properties.push(value("invalid", Decimal { value: f64::NAN }));
    assert!(matches!(library::export_ifc_step(&doc), Err(library::IfcLibraryError::PropertyValue(_))));
    print!("{rendered}");
}
''' + "\n".join(f"const {name}: {kind} = {quoted(fixture[key])};" for name, key, kind in [
    ("DICTIONARY","dictionary","&str"),("SOURCE","source","&str"),("GUID","guid","&str"),
    ("INVALID_GUID","invalidDictionaryId","&str"),("SHARED","sharedInteger","i64"),("OVERRIDE","overrideInteger","i64"),
    ("DECIMAL","rawDecimal","f64"),("TEXT","description","&str"),("STATUS","status","&str"),
]))
subprocess.run(["repo","tools","exec","--light","rustup","--","run","stable","rustc",
    "--edition","2024",str(probe),"-o",str(binary)], check=True,cwd=root)
with step.open("w") as target:
    subprocess.run([binary],check=True,stdout=target)
sys.path.insert(0,str(ticket/"🗑️generated/ifc-python"))
import ifcopenshell
import ifcopenshell.validate
model = ifcopenshell.open(str(step))
result = ifcopenshell.validate.json_logger()
ifcopenshell.validate.validate(model,result,express_rules=True)
assert not result.statements,result.statements
actual = {product.Tag: {p.Name:p.NominalValue.wrappedValue for p in product.HasPropertySets[0].HasProperties}
          for product in model.by_type("IfcBuildingElementProxyType")}
assert actual == fixture["values"],actual
references = model.by_type("IfcLibraryReference")
assert len(references)==1 and references[0].Identification==fixture["guid"],references
assert references[0].ReferencedLibrary.Name == fixture["dictionary"]
relationships = model.by_type("IfcExternalReferenceRelationship")
assert len(relationships)==2 and all(r.RelatingReference==references[0] and r.RelatedResourceObjects[0].Name=="sensors" for r in relationships)
assert not model.by_type("IfcPropertyTemplate")
print(f"[DEBUG] IfcOpenShell {ifcopenshell.__version__} validates scalar IFC4 properties, inherited values, product overrides and both GUID links.")
