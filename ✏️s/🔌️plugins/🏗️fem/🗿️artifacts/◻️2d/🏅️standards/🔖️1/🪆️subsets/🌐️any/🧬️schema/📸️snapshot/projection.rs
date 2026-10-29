//! 🔎️ Canonical record paths borrow persistent rows and authored fields without constructing a record mirror.
use super::Fem2dSnapshot;
use semio_framework_dsl_record::{
    DslField, DslVariants,
    native_encoding::{FieldProjectionSource, FieldProjectionView as V, projection_path_error},
};
use semio_framework_value::ValueError;
fn flat<'a>(texts: &[&'a str], numbers: &[f64], path: &[usize], ids: &'static [u16]) -> Result<V<'a>, ValueError> {
    if path.is_empty() {
        return Ok(V::Record(ids));
    }
    if path.len() != 1 {
        return Err(projection_path_error());
    }
    let i = path[0];
    if i < texts.len() { Ok(V::Text(texts[i])) } else { numbers.get(i - texts.len()).copied().map(V::Float).ok_or_else(projection_path_error) }
}
impl FieldProjectionSource for Fem2dSnapshot {
    fn projection_view(&self, path: &[usize]) -> Result<V<'_>, ValueError> {
        let Some((&field, path)) = path.split_first() else {
            return Ok(V::Record(&[0, 1, 2, 3, 4, 5, 6, 7, 8]));
        };
        match field {
            0 => {
                let Some((&i, tail)) = path.split_first() else {
                    return Ok(V::List(self.nodes.len()));
                };
                let n = self.nodes.get(i).ok_or_else(projection_path_error)?;
                flat(&[n.id.as_str()], &[n.x, n.y], tail, &[0, 1, 2])
            }
            1 => {
                if path.is_empty() {
                    return Ok(V::Block);
                }
                if path[0] != 0 {
                    return Err(projection_path_error());
                }
                let path = &path[1..];
                let Some((&i, tail)) = path.split_first() else {
                    return Ok(V::Statements(self.elements.len()));
                };
                DslVariants::projected_variant_view(self.elements.get(i).ok_or_else(projection_path_error)?, tail)
            }
            2 => DslField::projection_view(&self.regions, path),
            3 => {
                let Some((&i, tail)) = path.split_first() else {
                    return Ok(V::List(self.materials.len()));
                };
                let m = self.materials.get(i).ok_or_else(projection_path_error)?;
                flat(&[m.id.as_str(), m.name.as_str()], &[m.e, m.nu, m.rho], tail, &[0, 1, 2, 3, 4])
            }
            4 => DslField::projection_view(&self.sections, path),
            5 => DslField::projection_view(&self.supports, path),
            6 => DslField::projection_view(&self.load_cases, path),
            7 => DslField::projection_view(&self.combinations, path),
            8 => {
                if path.is_empty() {
                    Ok(V::Block)
                } else if path[0] == 0 {
                    DslField::projection_view(&self.analysis, &path[1..])
                } else {
                    Err(projection_path_error())
                }
            }
            _ => Err(projection_path_error()),
        }
    }
    fn projection_key(&self, path: &[usize], index: usize) -> Result<&str, ValueError> {
        let Some((&field, path)) = path.split_first() else {
            return Err(projection_path_error());
        };
        match field {
            1 if path.first() == Some(&0) => {
                let path = &path[1..];
                if path.is_empty() {
                    Ok(DslVariants::projected_variant_identity(self.elements.get(index).ok_or_else(projection_path_error)?).0)
                } else {
                    DslVariants::projected_variant_key(self.elements.get(path[0]).ok_or_else(projection_path_error)?, &path[1..], index)
                }
            }
            2 => DslField::projection_key(&self.regions, path, index),
            4 => DslField::projection_key(&self.sections, path, index),
            5 => DslField::projection_key(&self.supports, path, index),
            6 => DslField::projection_key(&self.load_cases, path, index),
            7 => DslField::projection_key(&self.combinations, path, index),
            8 if path.first() == Some(&0) => DslField::projection_key(&self.analysis, &path[1..], index),
            _ => Err(projection_path_error()),
        }
    }
}
