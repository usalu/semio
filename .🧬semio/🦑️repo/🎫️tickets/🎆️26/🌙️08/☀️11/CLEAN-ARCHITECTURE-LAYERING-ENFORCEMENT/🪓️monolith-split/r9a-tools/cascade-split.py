#!/usr/bin/env python3
"""Splits the concrete restoring half of mutations::cascade from its closure/diff half. The closure half stays below the leaves and states the restoring through the trait RemovalInverse; the concrete impl for the aggregate moves to the new module cascade_restore above the leaves; every delete companion that restored through ModelMutation becomes generic over Op with the bounds it needs. usage: cascade-split.py [--apply]"""
import re,sys,os,glob,subprocess
APPLY="--apply" in sys.argv
M="✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/"
P="crate::standards::v1::subsets::any::schema::mutations::"
cas=M+"🌊️cascade/🦀️.rs"
t=open(cas).read()
def snake(n): return re.sub(r"(?<!^)(?=[A-Z])","_",n).lower()

# 1. extract the old concrete method body and the list from the macro
m_list=re.search(r"removal! \{\n(.*?)\n\}\n//#endregion 🔖️Removal",t,re.S)
entries=re.findall(r"(\w+) => (\w+)\((\w+), (\w+)\);",m_list.group(1))
assert len(entries)==38,len(entries)
m_inv=re.search(r"\n            /// ↩️ One concrete row per removed record.*?\n                rows\.reverse\(\);\n                rows\n            \}\n",t,re.S)
assert m_inv
inv_text=m_inv.group(0)
t=t.replace(inv_text,"\n")
# macro header: only fields now
t=t.replace("($($field:ident => $variant:ident($module:ident, $record:ident);)*) => {","($($field:ident;)*) => {")
t=re.sub(r"removal! \{\n.*?\n\}\n//#endregion 🔖️Removal","removal! {\n"+"\n".join(f"    {f};" for f,_,_,_ in entries)+"\n}\n//#endregion 🔖️Removal",t,flags=re.S)
# imports
for line in ["use super::super::set_component_override::SetComponentOverride;\n","use super::super::set_element_classification::SetElementClassification;\n","use super::super::set_element_property::SetElementProperty;\n","use super::super::set_space_conditions::SetSpaceConditions;\n"]:
    assert line in t; t=t.replace(line,"")
old_use="use crate::standards::v1::subsets::any::schema::{diff::{Entry, KeyedDelta, ModelDiff, Patch}, mutations::ModelMutation, snapshot::{ModelSnapshot, TopConstraint}};"
assert old_use in t
t=t.replace(old_use,"use crate::standards::v1::subsets::any::schema::{diff::{Entry, KeyedDelta, ModelDiff, Patch}, snapshot::{ModelSnapshot, TopConstraint}};")
# trait + generic inverse/data_rows
old_inv=t[t.index("/// ↩️ The inverse rows of a delete leaf"):t.index("/// 🏷️ The sparse diff that deletes the properties")]
t=t.replace(old_inv,'''/// ↩️ An operation type that states the concrete creates and setters which give a removal back; the aggregate implements it (`cascade_restore`), so the delete leaves below it never name the leaves they restore.
pub trait RemovalInverse: Sized {
    /// 🧺️ One row per removed record and per removed property, classification, override and membership, in storage order.
    fn removal_inverse(removal: &Removal, base: &ModelSnapshot) -> Vec<Self>;
    /// 🏷️ One setter per property and per classification keyed by `id`.
    fn data_inverse(base: &ModelSnapshot, id: &str) -> Vec<Self>;
}

impl Removal {
    /// ↩️ The restoring rows of the removal as the operation type `Op` states them.
    pub fn inverse<Op: RemovalInverse>(&self, base: &ModelSnapshot) -> Vec<Op> {
        Op::removal_inverse(self, base)
    }
}

/// ↩️ The inverse rows of a delete leaf: empty when a root is unknown, the removal is refused or it restores more than [`INVERSE_ROWS`] rows, else the removal's concrete rows.
pub fn inverse<Op: RemovalInverse>(base: &ModelSnapshot, roots: &[String]) -> Vec<Op> {
    closure(base, roots).ok().filter(|removal| removal.rows(base) <= INVERSE_ROWS).map(|removal| removal.inverse(base)).unwrap_or_default()
}

''')
old_dr=t[t.index("/// ↩️ One concrete setter per property and per classification keyed by `id`"):t.index("/// 🗂️ The holders (element or type id, code)")]
dr_body=old_dr
t=t.replace(old_dr,'''/// ↩️ One concrete setter per property and per classification keyed by `id`: the rows that give a deleted holder its data back once it is recreated.
pub fn data_rows<Op: RemovalInverse>(base: &ModelSnapshot, id: &str) -> Vec<Op> {
    Op::data_inverse(base, id)
}

''')
TEST='''//! 🧪️ The restoring list of the aggregate covers every collection the removal closure can name: for each cascading delete case the rows the aggregate states equal the rows the closure counts.

use super::super::cascade::{closure, RemovalInverse};
use super::super::{kit, ModelMutation};

#[test]
fn restores_exactly_the_rows_the_removal_counts() {
    let mut checked = 0;
    for case in kit::cases().iter().filter(|case| case.dir.contains("cascades")) {
        let base = kit::before(case);
        for root in base.sites.keys().chain(base.buildings.keys()).chain(base.storeys.keys()) {
            if let Ok(removal) = closure(&base, std::slice::from_ref(root)) {
                let rows = <ModelMutation as RemovalInverse>::removal_inverse(&removal, &base).len();
                assert_eq!(rows, removal.rows(&base), "{}: restoring rows of {root}", case.dir);
                checked += 1;
            }
        }
    }
    assert!(checked > 0, "no cascading case was checked");
}
'''

# 2. the high module
rows_block=inv_text
# transform old method into trait method body
body=re.search(r"pub fn inverse\(&self, base: &ModelSnapshot\) -> Vec<ModelMutation> \{\n(.*)\n            \}\n$",rows_block,re.S).group(1)
body=body.replace("self.","removal.")
# dedent 4 spaces: original inside macro impl (16 spaces); target 8 spaces
body=body.replace("super::super::","super::")
# replace macro-substituted section by explicit macro list
body=body.replace("super::$module::$variant","super::$module::$variant")
dr_fn=re.search(r"pub fn data_rows\(base: &ModelSnapshot, id: &str\) -> Vec<ModelMutation> \{\n(.*?)\n\}\n",dr_body,re.S).group(1)
dr_fn="\n".join(("            "+l if l else l) for l in dr_fn.split("\n"))
list_lines="\n".join(f"        {f} => {v}({mod}, {rec});" for f,v,mod,rec in entries)
high=f'''//! 🔙️ The concrete restoring half of the shared cascade: the aggregate states the creates and setters that give a removal back, so the delete leaves, which only name [`RemovalInverse`], stay below the leaves they restore.

use super::cascade::{{Removal, RemovalInverse}};
use super::set_component_override::SetComponentOverride;
use super::set_element_classification::SetElementClassification;
use super::set_element_property::SetElementProperty;
use super::set_space_conditions::SetSpaceConditions;
use super::ModelMutation;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

macro_rules! restore {{
    ($($field:ident => $variant:ident($module:ident, $record:ident);)*) => {{
        impl RemovalInverse for ModelMutation {{
            /// ↩️ One concrete row per removed record in storage order: the store replays the vector reversed, so sites come last and are
            /// recreated first, then buildings, storeys, grid lines, storey contents, openings, and finally the property and classification setters.
            fn removal_inverse(removal: &Removal, base: &ModelSnapshot) -> Vec<Self> {{
{body}
            }}

            fn data_inverse(base: &ModelSnapshot, id: &str) -> Vec<Self> {{
{dr_fn}
            }}
        }}
    }};
}}

restore! {{
{list_lines}
}}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
'''
if APPLY:
    d=M+"🔙️cascade-restore"
    os.makedirs(d,exist_ok=True)
    open(d+"/🦀️.rs","w").write(high)
    os.makedirs(d+"/🧪️tests/🔬️unit",exist_ok=True)
    open(d+"/🧪️tests/🔬️unit/🦀️.rs","w").write(TEST)
    open(cas,"w").write(t)
    agg=M+"🦀️.rs"
    a=open(agg).read()
    anchor='#[path = "🌊️cascade/🦀️.rs"]\npub mod cascade;\n'
    assert a.count(anchor)==1
    a=a.replace(anchor,anchor+'#[path = "🔙️cascade-restore/🦀️.rs"]\npub mod cascade_restore;\n')
    open(agg,"w").write(a)
else:
    print(high[:4000]); print("..."); print(t[t.index("pub trait RemovalInverse")-200:t.index("pub trait RemovalInverse")+1800])

# 3. companions
files=glob.glob(M+"*/↩️inverse/🦀️.rs")
n=0
for f in sorted(files):
    s=open(f).read()
    if "cascade::" not in s or "ModelMutation" not in s and "removal.inverse" not in s: 
        continue
    o=s
    constructed=sorted(set(re.findall(r"ModelMutation::(\w+)\(",s)))
    bounds=[]
    uses_cascade=bool(re.search(r"cascade::(inverse|data_rows)|removal\.inverse",s))
    if uses_cascade: bounds.append(P+"cascade::RemovalInverse")
    for c in constructed: bounds.append(f"From<{P}{snake(c)}::{c}>")
    s=re.sub(r"ModelMutation::(\w+)\(","Op::from(",s)
    s=s.replace("mutations::ModelMutation, ","").replace(", mutations::ModelMutation","")
    s=re.sub(r"pub fn inverse\(([^)]*)\) -> Vec<ModelMutation> \{",lambda m:"pub fn inverse<Op>("+m.group(1)+") -> Vec<Op>\nwhere\n    Op: "+" + ".join(bounds)+",\n{",s)
    assert "ModelMutation" not in s.replace("//!",""),(f,)
    if s!=o:
        n+=1
        if APPLY: open(f,"w").write(s)
        elif n<=3: print("==",f.replace(M,"")); print(s)
print("companions",n)
