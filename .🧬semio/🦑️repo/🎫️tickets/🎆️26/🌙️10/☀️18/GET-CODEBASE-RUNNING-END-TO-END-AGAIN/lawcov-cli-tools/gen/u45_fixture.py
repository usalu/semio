BASE = "schema/mutations"
def leaf(name, struct, extra, inverse_body):
    comp = "\n".join([f"//! {name} leaf", "use crate::schema::mutations::DemoMutation;", "use crate::schema::{Diff, Snap};", "use protocol::{MutationKind, SemanticDescriptor};", "", "#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf)]", "#[mutation_leaf(contract = ::protocol)]", f"pub struct {struct} {{ pub id: String }}", "",
        f"pub fn {name}(id: String) -> DemoMutation {{", f"    DemoMutation::{struct}({struct} {{ id }})", "}", "", f"impl MutationKind<Snap, DemoMutation> for {struct} {{",
        '    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "x", entity: "y", kind: "z", record: "R" };', "    fn diff(&self, base: &Snap) -> protocol::MutationOutcome<Diff> {", "        super::diff::diff(self, base)", "    }",
        "    fn inverse(&self, base: &Snap) -> Result<Vec<DemoMutation>, semio_framework_value::ValueError> {", "        Ok(super::inverse::inverse(self, base)?)", "    }", "}", extra, ""])
    diff = f"use super::{struct};\nuse crate::schema::{{Diff, Snap}};\npub fn diff(_payload: &{struct}, _base: &Snap) -> protocol::MutationOutcome<Diff> {{ protocol::MutationOutcome::empty() }}\n"
    inv = "\n".join([f"use super::{struct};", "use crate::schema::mutations::DemoMutation;", "use crate::schema::Snap;", f"pub fn inverse(payload: &{struct}, base: &Snap) -> Result<Vec<DemoMutation>, semio_framework_value::ValueError> {{", inverse_body, "}", ""])
    return {f"{BASE}/{name}/mod.rs": comp, f"{BASE}/{name}/diff/mod.rs": diff, f"{BASE}/{name}/inverse/mod.rs": inv}
def mount(name, tests):
    rows = ['        #[path = "."]', f"        pub mod {name} {{", f'            #[path = "{BASE}/{name}/mod.rs"]', "            mod component;", f'            #[path = "{BASE}/{name}/diff/mod.rs"]', "            pub mod diff;", f'            #[path = "{BASE}/{name}/inverse/mod.rs"]', "            pub mod inverse;", "            pub use component::*;"]
    if tests:
        rows += ["            #[cfg(test)]", f'            #[path = "{BASE}/{name}/tests/mod.rs"]', f"            mod tests_{name};"]
    return "\n".join(rows + ["        }"])
ROOT = "\n".join(["//! demo root.", "extern crate semio_framework_os_kernel as protocol;", '#[path = "."]', "pub mod schema {", '    #[path = "schema/mod.rs"]', "    mod component;", "    pub use component::*;", '    #[path = "."]', "    pub mod mutations {", f'        #[path = "{BASE}/mod.rs"]', "        mod component;", "        pub use component::*;",
    mount("alpha", True), mount("beta", True), mount("gamma", False), "    }", "}", ""])
SCHEMA = "//! schema\n#[derive(Clone, Debug, Default, PartialEq)]\npub struct Snap { pub id: String }\n#[derive(Clone, Debug, Default, PartialEq)]\npub struct Diff { pub id: String }\n"
AGG = "\n".join(["//! aggregate", "use crate::schema::{Diff, Snap};", "", "#[derive(Clone, Debug, PartialEq, dsl::Mutations)]", '#[mutations(snapshot = Snap, diff = Diff, schema = "demo.demo")]', "pub enum DemoMutation {", "    Alpha(Alpha),", "    Beta(Beta),", "    Gamma(Gamma),", "}", "",
    "pub use crate::schema::mutations::alpha::{alpha, Alpha};", "pub use crate::schema::mutations::beta::{beta, Beta};", "pub use crate::schema::mutations::gamma::{gamma, Gamma};", ""])
FILES = {"lib.rs": ROOT, "schema/mod.rs": SCHEMA, f"{BASE}/mod.rs": AGG}
FILES.update(leaf("alpha", "Alpha", "", '    Ok(vec![DemoMutation::Alpha(Alpha { id: format!("{}-{}", payload.id, base.id) })])'))
FILES.update(leaf("beta", "Beta", "", "    Ok(vec![DemoMutation::Beta(Beta { id: payload.id.clone() }), crate::schema::mutations::alpha::alpha(base.id.clone())])"))
FILES.update(leaf("gamma", "Gamma", "\n#[cfg(test)]\nmod inline_tests {\n    use super::*;\n    #[test]\n    fn builds() { let _ = gamma(String::new()); let _: DemoMutation = gamma(String::new()); }\n}", "    Ok(vec![DemoMutation::Gamma(Gamma { id: payload.id.clone() })])"))
FILES[f"{BASE}/beta/tests/mod.rs"] = "use super::*;\nuse crate::schema::mutations::DemoMutation;\n#[test]\nfn builds() { let _: DemoMutation = beta(String::new()); }\n"
FILES[f"{BASE}/alpha/tests/mod.rs"] = "use crate::schema::mutations::{alpha, DemoMutation};\n#[test]\nfn builds() { let _: DemoMutation = alpha(String::new()); }\n"
def with_files(**extra):
    out = dict(FILES)
    for k, v in extra.items():
        out[k.replace("__", "/")] = v
    return out
def add_leaf(files, name, struct, extra="", inverse_body="    Ok(vec![])", variant=None, aggregate_use=True):
    f = dict(files)
    f.update(leaf(name, struct, extra, inverse_body))
    f["lib.rs"] = f["lib.rs"].replace(mount("gamma", False), mount("gamma", False) + "\n" + mount(name, False))
    f[f"{BASE}/mod.rs"] = f[f"{BASE}/mod.rs"].replace("    Gamma(Gamma),", "    Gamma(Gamma),\n    " + (variant or f"{struct}({struct}),")).replace("pub use crate::schema::mutations::gamma::{gamma, Gamma};", "pub use crate::schema::mutations::gamma::{gamma, Gamma};\n" + (f"pub use crate::schema::mutations::{name}::{{{name}, {struct}}};" if aggregate_use else ""))
    return f
