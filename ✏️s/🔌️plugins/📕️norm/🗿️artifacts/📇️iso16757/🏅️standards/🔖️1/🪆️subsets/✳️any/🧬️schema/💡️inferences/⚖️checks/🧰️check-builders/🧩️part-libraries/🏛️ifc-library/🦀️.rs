//! 🏛️ IFC4 ADD2 TC1 catalogue library projection. https://standards.buildingsmart.org/IFC/RELEASE/IFC4/ADD2_TC1/EXPRESS/IFC4.exp

use semio_framework_hash::Sha256;
use semio_s_artifact_norm_iso16757_model::part_1::Catalogue;
use semio_s_artifact_stdio_contract_part21::{Part21Builder, Part21Header, Part21Value as V, write_part21};
use std::collections::{BTreeMap, BTreeSet};

#[path = "📋️properties.rs"]
mod properties;

/// 🆔 IFC compressed identifier derived from the source namespace and record key.
fn global_id(catalogue_id: &str, role: &str, id: &str) -> V {
    const ALPHABET: &[u8; 64] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_$";
    let mut hash = Sha256::default();
    for part in ["semio.iso16757.ifc4", catalogue_id, role, id] {
        hash.update(&(part.len() as u64).to_be_bytes());
        hash.update(part.as_bytes());
    }
    let mut bytes: [u8; 16] = hash.finalize()[..16].try_into().expect("128 bits");
    bytes[6] = (bytes[6] & 0x0f) | 0x80;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    let mut number = u128::from_be_bytes(bytes);
    let mut digits = [b'0'; 22];
    for digit in digits.iter_mut().rev() {
        *digit = ALPHABET[(number & 63) as usize];
        number >>= 6;
    }
    V::Str(digits.into_iter().map(char::from).collect())
}

/// 🚧 A catalogue value that the IFC4 library cannot represent.
#[derive(Debug)]
pub enum IfcLibraryError {
    Label(String),
    ClassId(String),
    SeriesId(String),
    ProductId(String),
    ProductMembership(String),
    PropertyDefinitionId(String),
    PropertyValue(String),
}

impl IfcLibraryError {
    /// 🌐 Localized descriptions include the source value that needs correction.
    pub fn messages(&self) -> (String, String) {
        let (en, de, value) = match self {
            Self::Label(value) => ("An IFC label must contain between 1 and 255 characters", "Eine IFC-Bezeichnung muss zwischen 1 und 255 Zeichen enthalten", value),
            Self::ClassId(value) => ("Missing or duplicate catalogue class ID", "Fehlende oder doppelte Katalogklassenkennung", value),
            Self::SeriesId(value) => ("Duplicate catalogue series ID", "Doppelte Katalogserienkennung", value),
            Self::ProductId(value) => ("Duplicate catalogue product ID", "Doppelte Katalogproduktkennung", value),
            Self::ProductMembership(value) => ("Missing class or series for catalogue product", "Fehlende Klasse oder Serie für Katalogprodukt", value),
            Self::PropertyDefinitionId(value) => ("Duplicate catalogue property definition ID", "Doppelte Katalogmerkmalskennung", value),
            Self::PropertyValue(value) => ("Invalid or duplicate static value for an exported property", "Ungültiger oder doppelter statischer Wert eines exportierten Merkmals", value),
        };
        (format!("{en} ({value:?})."), format!("{de} ({value:?})."))
    }
}

fn label(name: &str) -> Result<V, IfcLibraryError> {
    if name.trim().is_empty() || name.chars().count() > 255 {
        return Err(IfcLibraryError::Label(name.into()));
    }
    Ok(V::Str(name.into()))
}

fn refs(ids: impl IntoIterator<Item = u64>) -> V {
    V::List(ids.into_iter().map(V::Ref).collect())
}

/// 📤 Exports the catalogue identity, names, product IDs, class memberships, scalar properties and dictionary links as a scoped IFC4 type library.
pub fn export_ifc_step(catalogue: &Catalogue) -> Result<String, IfcLibraryError> {
    let catalogue_id = &catalogue.id.0;
    let root_id = |role, id| global_id(catalogue_id, role, id);
    let mut builder = Part21Builder::new();
    let library = builder.alloc("IFCPROJECTLIBRARY", vec![
        root_id("library", ""), V::Unset, label(&catalogue.metadata.names.preferred.text)?,
        V::Str(catalogue_id.clone()), V::Unset, V::Unset, V::Unset, V::Unset, V::Unset,
    ]);
    let dictionary = if !catalogue.dictionary.id.trim().is_empty() {
        let version = if catalogue.dictionary.version.trim().is_empty() {
            V::Unset
        } else {
            label(&catalogue.dictionary.version)?
        };
        let dictionary = builder.alloc("IFCLIBRARYINFORMATION", vec![
            label(&catalogue.dictionary.id)?, version, V::Unset, V::Unset, V::Unset, V::Unset,
        ]);
        builder.alloc("IFCRELASSOCIATESLIBRARY", vec![
            root_id("dictionary", &catalogue.dictionary.id), V::Unset, V::Unset, V::Unset,
            refs([library]), V::Ref(dictionary),
        ]);
        Some(dictionary)
    } else {
        None
    };
    let mut property_sets = properties::CatalogueProperties::new(&catalogue.property_definitions, dictionary)?;
    let mut classes = BTreeMap::new();
    for class in &catalogue.product_classes {
        if class.id.trim().is_empty() || classes.insert(class.id.as_str(), class).is_some() {
            return Err(IfcLibraryError::ClassId(class.id.clone()));
        }
    }
    let mut definitions = Vec::new();
    let mut groups: BTreeMap<&str, (u64, Vec<u64>)> = BTreeMap::new();
    for (id, class) in classes {
        let group = builder.alloc("IFCGROUP", vec![
            root_id("class", id), V::Unset, label(&class.names.preferred.text)?,
            V::Str(id.into()), V::Unset,
        ]);
        definitions.push(group);
        groups.insert(id, (group, Vec::new()));
    }
    let mut series = BTreeMap::new();
    for row in &catalogue.product_series {
        if series.insert(row.id.as_str(), row).is_some() {
            return Err(IfcLibraryError::SeriesId(row.id.clone()));
        }
    }
    let mut products: Vec<_> = catalogue.products.iter().collect();
    products.sort_unstable_by(|left, right| left.id.cmp(&right.id));
    let mut product_ids = BTreeSet::new();
    for product in products {
        if !product_ids.insert(&product.id) {
            return Err(IfcLibraryError::ProductId(product.id.clone()));
        }
        let series = series.get(product.series_id.as_str())
            .ok_or_else(|| IfcLibraryError::ProductMembership(product.id.clone()))?;
        let class = groups.get_mut(series.class_id.as_str())
            .ok_or_else(|| IfcLibraryError::ProductMembership(product.id.clone()))?;
        let property_set = property_sets.export(&mut builder, catalogue_id, series, product)?;
        let entity = builder.alloc("IFCBUILDINGELEMENTPROXYTYPE", vec![
            root_id("product", &product.id), V::Unset, label(&product.names.preferred.text)?,
            V::Unset, V::Unset, property_set, V::Unset, label(&product.id)?,
            V::Unset, V::Enum("NOTDEFINED".into()),
        ]);
        definitions.push(entity);
        class.1.push(entity);
    }
    for (id, (group, members)) in groups {
        if !members.is_empty() {
            builder.alloc("IFCRELASSIGNSTOGROUP", vec![
                root_id("membership", id), V::Unset, V::Unset, V::Unset, refs(members), V::Unset, V::Ref(group),
            ]);
        }
    }
    if !definitions.is_empty() {
        builder.alloc("IFCRELDECLARES", vec![
            root_id("declaration", ""), V::Unset, V::Unset, V::Unset, V::Ref(library), refs(definitions),
        ]);
    }
    let mut header = Part21Header::iso_10303_21_minimum();
    header.file_description = vec![V::List(vec![V::Str("IFC4 catalogue type library".into())]), V::Str("2;1".into())];
    header.file_name[0] = V::Str("catalogue.ifc".into());
    header.file_name[1] = V::Str("1970-01-01T00:00:00".into());
    header.file_name[5] = V::Str("semio ISO 16757 catalogue".into());
    header.file_schema = vec![V::List(vec![V::Str("IFC4".into())])];
    Ok(write_part21(&builder.build(header)))
}
