#!/usr/bin/env python3
import os,re,sys
from pathlib import Path
REPO=Path("/Users/ueli/Documents/semio")
C=REPO/".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
IFC=REPO/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc"
DET=IFC/"🔎️details"
DOM=IFC/"📚️catalog/⚖️domain"
ORIG=C/"🗑️generated/1101/ifcsplit-orig"
R=(ORIG/"details/🦀️.rs").read_text().split("\n")
def L(a,b):return "\n".join(R[a-1:b])
def need(cond,msg):
    if not cond:sys.exit("ASSERT "+msg)
need(R[24].startswith("pub trait IfcDetailsProfile"),"trait")
need(R[13].startswith("pub struct IfcNormalizationAttribute"),"struct")
need(R[88].startswith("pub fn details_work_capacity"),"capacity")
need(R[93].startswith("fn value_error"),"value_error")
need(R[260].startswith("fn owned_entity_edit"),"owned")
need(R[540].startswith("const REFERENCE_WINDOW"),"consts")
need(R[561].startswith("fn key("),"key")
need(R[565].startswith("fn named"),"named")
need(R[573].startswith("fn requested_type"),"requested_type")
need(R[589].startswith("impl<'a, C: IfcDetailsProfile> IfcDetailsProvider"),"impl")
need(R[1252].startswith("fn reference_id"),"reference_id")
need(R[1643].startswith("#[cfg(test)]"),"tests")
HEADER=L(3,10).replace("use super::{IfcAttributeDeclaration, IfcCatalog, IfcTypeExpression};","use semio_s_artifact_stdio_ifc_declarations::{IfcAttributeDeclaration, IfcCatalog, IfcTypeExpression};")
need("semio_s_artifact_stdio_ifc_declarations" in HEADER,"header")
EXT_STORE="extern crate semio_framework_os_kernel as store;\nextern crate semio_framework_os_kernel as protocol;\n"
TESTS={p.name for p in (ORIG/"details").glob("*.rs") if re.search(r"tests?\.rs$",p.name)}
def xform(text,convert=True,rebase=True):
    if convert:text=text.replace("pub(super)","pub")
    if not rebase:return text
    def mount(m):
        name=m.group(2)
        return m.group(1)+("../"+name if name in TESTS else name)+m.group(3)
    return re.sub(r'(#\[path\s*=\s*")([^"/]+)("\s*\])',mount,text)
def put(path,text):
    path.parent.mkdir(parents=True,exist_ok=True)
    path.write_text(text)
def move(crate,names,convert=True):
    for n in names:
        src=ORIG/"details"/n
        need(src.exists(),"missing "+n)
        put(DET/crate/n,xform(src.read_text(),convert))
        old=DET/n
        if old.exists():old.unlink()
P="🧭️profile";W="📨️wire";T="🔤️type";E="✏️edit";V="👁️provider";G="🧶️guidance"
NAME={P:"semio_s_artifact_stdio_ifc_details_profile",W:"semio_s_artifact_stdio_ifc_details_wire",T:"semio_s_artifact_stdio_ifc_details_type",E:"semio_s_artifact_stdio_ifc_details_edit",V:"semio_s_artifact_stdio_ifc_details_provider",G:"semio_s_artifact_stdio_ifc_details_guidance"}
move(P,["type_metadata.rs"])
move(W,["pointer.rs","receive.rs","wire.rs","preimage.rs","inverse.rs","source_loan.rs"])
move(T,["type_input.rs","type_input_kind.rs","type_input_select.rs","type_remap.rs","constituent_roster.rs"])
move(E,["reference.rs","value_edit.rs","contextual.rs","contextual_edit.rs","contextual_render.rs","domain_edit.rs","domain_work.rs"])
move(G,["type-guidance.rs","type_input_choices.rs","type_input_list_copy.rs","type_input_typed_copy.rs","type_input_set_controls.rs","type_input_set_copy.rs","type_guidance_candidate.rs","type_guidance_request.rs"],convert=False)
for n in ["admission.rs","command_wire.rs","entity_remove.rs","work.rs"]:
    put(DET/n,(ORIG/"details"/n).read_text())
for n in ["candidate_rules.rs"]:
    old=DET/n
    if old.exists():old.unlink()
put(DET/P/"🦀️.rs","//! 🧭️ Profile authority: declaration roles, type metadata and local-rule candidates every IFC details layer shares.\n\n"+HEADER+"""
#[path="type_metadata.rs"]pub mod type_metadata;
pub use type_metadata::{IfcDetailsTypeMetadata,IfcTypeProjection};
#[path="candidate_rules.rs"]pub mod candidate_rules;
pub use candidate_rules::{IfcCandidateCheck,IfcCandidateRules,IfcCandidateRuleStep,IfcTypeCandidate};

"""+L(12,14)+"\n"+L(25,50)+"\n\n"+L(562,564)+"\n")
put(DET/W/"🦀️.rs","//! 📨️ Wire layer: borrowed pointers, owned receivers, publications and source loans over the retained IFC graph.\n\n"+HEADER+"\n"+EXT_STORE+"""pub use semio_s_artifact_stdio_ifc_details_profile::*;
#[path="receive.rs"]pub mod receive;
pub use receive::{IfcValueReceiveMode, IfcValueReceiver};
#[path="pointer.rs"]pub mod pointer;
#[path="wire.rs"]pub mod wire;
pub use wire::{IfcPublication, SnapshotSource};
#[path="source_loan.rs"]pub mod source_loan;
""")
put(DET/T/"🦀️.rs","//! 🔤️ Type layer: guided type input cursors, type remapping and the constituent roster over declaration metadata.\n\n"+HEADER+"""
pub use semio_s_artifact_stdio_ifc_details_wire::*;
pub use semio_s_artifact_stdio_ifc_domain as domain;
pub use semio_s_artifact_stdio_ifc_domain::cursor as domain_cursor;
#[path="type_remap.rs"]pub mod type_remap;
#[path="type_input.rs"]pub mod type_input;
#[path="type_input_kind.rs"]pub mod type_input_kind;
#[path="type_input_select.rs"]pub mod type_input_select;
#[path="constituent_roster.rs"]pub mod constituent_roster;

pub fn requested_type(value: &DslValue) -> Option<&str> {
    type_input::request(value).map(|(name,_)|name)
}
""")
put(DET/E/"🦀️.rs","//! ✏️ Edit layer: value, reference, enumeration and domain edits over the retained IFC graph.\n\n"+HEADER+"""
pub use semio_s_artifact_stdio_ifc_details_type::*;
#[path="reference.rs"]pub mod reference;
#[path="value_edit.rs"]pub mod value_edit;
#[path="../../📥️source/🗂️index/🦀️.rs"]mod source_index;
#[path="contextual.rs"]pub mod contextual;
pub use contextual::{IfcEnumerationAddress,IfcEnumerationDiagnostic,IfcEnumerationQuery,IfcEnumerationView};
#[path="contextual_render.rs"]pub mod contextual_render;
pub use contextual_render::IfcDetailsRenderWork;
#[path="contextual_edit.rs"]pub mod contextual_edit;
#[path="domain_work.rs"]pub mod domain_work;
#[path="domain_edit.rs"]pub mod domain_edit;
""")
prov=L(541,543)+"\n\n"+L(544,561)+"\n\n"+L(566,573)+"\n\n"+L(578,1252)
prov=prov.replace("\nfn derived_keys","\npub fn derived_keys",1)
for sig in ["fn attributes(","fn reference_slot(","fn normalized_argument(","fn template("]:
    need(prov.count("\n    "+sig)==1,"provider "+sig)
    prov=prov.replace("\n    "+sig,"\n    pub "+sig,1)
put(DET/V/"🦀️.rs","//! 👁️ Provider layer: the declaration-aware snapshot details provider.\n\n"+HEADER+"\npub use semio_s_artifact_stdio_ifc_details_edit::*;\n\n"+prov+"\n")
put(DET/G/"🦀️.rs","//! 🧶️ Guidance layer: staged type-input sources and guided candidates.\n\n"+HEADER+"""
pub use semio_s_artifact_stdio_ifc_details_edit::*;
#[path="type_guidance_request.rs"]pub mod type_guidance_request;
#[path="type_guidance_candidate.rs"]pub mod type_guidance_candidate;
#[path="type-guidance.rs"]pub mod type_guidance;
pub use type_guidance::{IfcTypeInputSourceWork,IfcPreparedTypeInputSource};
""")
top="//! 🧭️ Catalog-profile declaration-aware controls over the shared lossless IFC graph.\n\n"+HEADER+"\n"+EXT_STORE+"""
pub use semio_s_artifact_stdio_ifc_details_provider::*;
pub use semio_s_artifact_stdio_ifc_details_guidance::*;
#[path="admission.rs"]mod admission;
#[path="command_wire.rs"]mod command_wire;
#[path="entity_remove.rs"]mod entity_remove;
#[path="work.rs"]mod work;
pub use work::IfcDetailsWork;

"""+L(89,260)+"\n"+L(261,540)+"\n\n"+L(1253,1643)+"\n"+L(1644,len(R))
put(DET/"🦀️.rs",top)
print("OK")
