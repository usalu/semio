use semio_s_artifact_stdio_ifc4x3_express as express;
mod old_program {
    use semio_s_artifact_stdio_ifc4x3_express::ast::*;
    include!("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/program.rs");
    pub fn program() -> &'static ExpressProgram { &PROGRAM }
}
mod old_metadata {
    use semio_s_artifact_stdio_ifc4x3_express::metadata::*;
    include!("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/metadata-data.rs");
}
mod old_index {
    pub struct OwnerIndex { pub owner: &'static str, pub rules: &'static [usize], pub derived: &'static [usize], pub inverse: &'static [usize], pub bounds: &'static [usize], pub unique: &'static [usize] }
    include!("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/program-index-data.rs");
}

mod tpl {
    include!("t_types.rs");
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🏷️templates/pack.rs"]
    mod pack;
    pub use pack::{decode_templates, embedded_templates};
    include!("t_data.rs");
    pub fn old() -> &'static IfcTemplateCatalog { &CATALOG }
}
mod mvd {
    include!("m_types.rs");
    #[path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/👁️mvd/pack.rs"]
    mod pack;
    pub use pack::{decode_mvd, embedded_mvd};
    include!("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/👁️mvd/data.rs");
    pub fn old() -> &'static IfcMvdCatalog { &CATALOG }
}
fn main() {
    let new = express::ifc4x3_express_program();
    let old = old_program::program();
    println!("program equal: {}", new == old);
    let ne = express::metadata::entities();
    assert_eq!(ne.len(), old_metadata::ENTITIES.len());
    let mut bad = 0;
    for (a, b) in ne.iter().zip(old_metadata::ENTITIES) { if format!("{a:?}") != format!("{b:?}") { bad += 1; } }
    let nt = express::metadata::types();
    assert_eq!(nt.len(), old_metadata::TYPES.len());
    for (a, b) in nt.iter().zip(old_metadata::TYPES) { if format!("{a:?}") != format!("{b:?}") { bad += 1; } }
    println!("metadata entities {} types {} mismatches {}", ne.len(), nt.len(), bad);
    let mut obad = 0;
    assert_eq!(old_index::OWNERS.len(), 0usize.max(old_index::OWNERS.len()));
    for row in old_index::OWNERS {
        let n = express::program_index::owner(row.owner).expect("owner");
        if n.rules != row.rules || n.derived != row.derived || n.inverse != row.inverse || n.bounds != row.bounds || n.unique != row.unique { obad += 1; }
    }
    for row in old_index::ENTITIES {
        let n = express::program_index::entity(row.owner).expect("entity");
        if n.rules != row.rules || n.derived != row.derived || n.inverse != row.inverse || n.bounds != row.bounds || n.unique != row.unique { obad += 1; }
    }
    println!("index owners {} entities {} mismatches {}", old_index::OWNERS.len(), old_index::ENTITIES.len(), obad);
    let (n, o) = (tpl::embedded_templates(), tpl::old());
    println!("templates equal (debug): {} count {}", format!("{n:?}") == format!("{o:?}"), n.templates.len());
    let (n, o) = (mvd::embedded_mvd(), mvd::old());
    println!("mvd equal: {} entities {} concepts {}", n == o, n.entities.len(), n.concepts.len());
}
