//! 📋️ The unitless scalar property projection of the IFC4 catalogue library.

use super::{IfcLibraryError, Part21Builder, V, global_id, label, refs};
use semio_s_artifact_norm_iso16757_model::CatalogueValue;
use semio_s_artifact_norm_iso16757_model::part_1::{Product, ProductSeries, PropertyDefinition};
use std::collections::{BTreeMap, BTreeSet};

pub(super) struct CatalogueProperties<'a> {
    definitions: BTreeMap<&'a str, &'a PropertyDefinition>,
    dictionary: Option<u64>,
    references: BTreeMap<&'a str, u64>,
}

impl<'a> CatalogueProperties<'a> {
    pub(super) fn new(definitions: &'a [PropertyDefinition], dictionary: Option<u64>) -> Result<Self, IfcLibraryError> {
        let mut by_id = BTreeMap::new();
        for definition in definitions {
            if by_id.insert(definition.id.as_str(), definition).is_some() {
                return Err(IfcLibraryError::PropertyDefinitionId(definition.id.clone()));
            }
        }
        Ok(Self { definitions: by_id, dictionary, references: BTreeMap::new() })
    }

    pub(super) fn export(&mut self, builder: &mut Part21Builder, catalogue: &str, series: &ProductSeries, product: &Product) -> Result<V, IfcLibraryError> {
        let mut values: BTreeMap<&str, &CatalogueValue> = series.shared_property_values.iter()
            .map(|(id, value)| (id.as_str(), value)).collect();
        let mut explicit = BTreeSet::new();
        for property in &product.static_properties {
            let id = property.definition_id.as_str();
            if !explicit.insert(id) {
                return Err(IfcLibraryError::PropertyValue(id.into()));
            }
            if property.function_id.is_none() {
                values.insert(id, &property.value);
            } else {
                values.remove(id);
            }
        }

        let mut properties = Vec::new();
        for (id, value) in values {
            let definition = self.definitions.get(id).copied();
            if definition.is_some_and(|entry| entry.unit.is_some()) {
                continue;
            }
            let Some(value) = scalar(value).map_err(|()| IfcLibraryError::PropertyValue(id.into()))? else { continue };
            let description = definition.map_or(V::Unset, |entry| V::Str(entry.names.preferred.text.clone()));
            let property = builder.alloc("IFCPROPERTYSINGLEVALUE", vec![label(id)?, description, value, V::Unset]);
            if let Some((identifier, library)) = definition.and_then(|entry| entry.dictionary_property_id.as_deref())
                .filter(|id| dictionary_guid(id)).zip(self.dictionary)
            {
                let reference = *self.references.entry(identifier).or_insert_with(|| builder.alloc("IFCLIBRARYREFERENCE", vec![
                    V::Unset, V::Str(identifier.into()), V::Unset, V::Unset, V::Unset, V::Ref(library),
                ]));
                builder.alloc("IFCEXTERNALREFERENCERELATIONSHIP", vec![V::Unset, V::Unset, V::Ref(reference), refs([property])]);
            }
            properties.push(property);
        }
        if properties.is_empty() {
            return Ok(V::Unset);
        }
        let set = builder.alloc("IFCPROPERTYSET", vec![
            global_id(catalogue, "scalar-properties", &product.id), V::Unset, V::Str("Catalogue properties".into()), V::Unset,
            refs(properties),
        ]);
        Ok(refs([set]))
    }
}

fn scalar(value: &CatalogueValue) -> Result<Option<V>, ()> {
    let (name, value) = match value {
        CatalogueValue::Boolean { value } => ("IFCBOOLEAN", V::Enum(if *value { "T" } else { "F" }.into())),
        CatalogueValue::Integer { value } => ("IFCINTEGER", V::Int(*value)),
        CatalogueValue::Decimal { value } if value.is_finite() => ("IFCREAL", V::Real((*value).into())),
        CatalogueValue::Decimal { .. } => return Err(()),
        CatalogueValue::Text { value } => ("IFCTEXT", V::Str(value.clone())),
        CatalogueValue::Identifier { value } | CatalogueValue::Enumeration { value } |
        CatalogueValue::Controlled { value, .. } if value.chars().count() <= 255 => ("IFCIDENTIFIER", V::Str(value.clone())),
        CatalogueValue::Identifier { .. } | CatalogueValue::Enumeration { .. } |
        CatalogueValue::Controlled { .. } => return Err(()),
        _ => return Ok(None),
    };
    Ok(Some(V::Typed { name: name.into(), items: vec![value] }))
}

fn dictionary_guid(value: &str) -> bool {
    value.len() == 36 && value.bytes().enumerate().all(|(i, byte)| match i {
        8 | 13 | 18 | 23 => byte == b'-',
        _ => byte.is_ascii_hexdigit(),
    })
}
