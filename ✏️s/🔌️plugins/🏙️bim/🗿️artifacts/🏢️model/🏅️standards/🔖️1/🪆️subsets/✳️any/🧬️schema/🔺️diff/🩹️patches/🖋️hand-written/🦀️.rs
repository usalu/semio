//! 🖋️ Hand-written sparse patches that the model generator does not emit: option groups, design options, worksets, element memberships, structural supports, load cases and structural loads.

use super::*;

/// 🩹️ Sparse authored OptionGroup fields.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct OptionGroupPatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Patch<OptionGroup> for OptionGroupPatch {
    fn write(&self, base: &OptionGroup) -> OptionGroup { OptionGroup { name: take(&self.name, &base.name) } }
    fn restoring(&self, base: &OptionGroup) -> Self { Self { name: restore(&self.name, &base.name) } }
    fn minimal(&self, base: &OptionGroup) -> Self { Self { name: changed(&self.name, &base.name) } }
    fn merge(&mut self, later: Self) { merge_slot(&mut self.name, later.name); }
    fn touched(&self) -> Vec<String> { let mut fields = Vec::new(); if self.name.is_some() { fields.push("name".into()); } fields }
    fn is_empty(&self) -> bool { self == &Self::default() }
}

/// 🩹️ Sparse authored DesignOption fields.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct DesignOptionPatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
}

impl Patch<DesignOption> for DesignOptionPatch {
    fn write(&self, base: &DesignOption) -> DesignOption { DesignOption { group: take(&self.group, &base.group), name: take(&self.name, &base.name), primary: take(&self.primary, &base.primary) } }
    fn restoring(&self, base: &DesignOption) -> Self { Self { group: restore(&self.group, &base.group), name: restore(&self.name, &base.name), primary: restore(&self.primary, &base.primary) } }
    fn minimal(&self, base: &DesignOption) -> Self { Self { group: changed(&self.group, &base.group), name: changed(&self.name, &base.name), primary: changed(&self.primary, &base.primary) } }
    fn merge(&mut self, later: Self) { merge_slot(&mut self.group, later.group); merge_slot(&mut self.name, later.name); merge_slot(&mut self.primary, later.primary); }
    fn touched(&self) -> Vec<String> { let mut fields = Vec::new(); if self.group.is_some() { fields.push("group".into()); } if self.name.is_some() { fields.push("name".into()); } if self.primary.is_some() { fields.push("primary".into()); } fields }
    fn is_empty(&self) -> bool { self == &Self::default() }
}

/// 🩹️ Sparse authored Workset fields.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct WorksetPatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub default_visible: Option<bool>,
}

impl Patch<Workset> for WorksetPatch {
    fn write(&self, base: &Workset) -> Workset { Workset { name: take(&self.name, &base.name), default_visible: take(&self.default_visible, &base.default_visible) } }
    fn restoring(&self, base: &Workset) -> Self { Self { name: restore(&self.name, &base.name), default_visible: restore(&self.default_visible, &base.default_visible) } }
    fn minimal(&self, base: &Workset) -> Self { Self { name: changed(&self.name, &base.name), default_visible: changed(&self.default_visible, &base.default_visible) } }
    fn merge(&mut self, later: Self) { merge_slot(&mut self.name, later.name); merge_slot(&mut self.default_visible, later.default_visible); }
    fn touched(&self) -> Vec<String> { let mut fields = Vec::new(); if self.name.is_some() { fields.push("name".into()); } if self.default_visible.is_some() { fields.push("default_visible".into()); } fields }
    fn is_empty(&self) -> bool { self == &Self::default() }
}

/// 🩹️ Sparse authored ElementMembership fields.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct ElementMembershipPatch {
    #[value(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

impl Patch<ElementMembership> for ElementMembershipPatch {
    fn write(&self, base: &ElementMembership) -> ElementMembership { ElementMembership { target: take(&self.target, &base.target) } }
    fn restoring(&self, base: &ElementMembership) -> Self { Self { target: restore(&self.target, &base.target) } }
    fn minimal(&self, base: &ElementMembership) -> Self { Self { target: changed(&self.target, &base.target) } }
    fn merge(&mut self, later: Self) { merge_slot(&mut self.target, later.target); }
    fn touched(&self) -> Vec<String> { let mut fields = Vec::new(); if self.target.is_some() { fields.push("target".into()); } fields }
    fn is_empty(&self) -> bool { self == &Self::default() }
}

/// 🩹️ Sparse authored patch of StructuralSupport.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct StructuralSupportPatch {
 #[value(skip_serializing_if = "Option::is_none")]
 pub name: Option<String>,
 #[value(skip_serializing_if = "Option::is_none")]
 pub member: Option<String>,
 #[value(skip_serializing_if = "Option::is_none")]
 pub location: Option<StructuralLocation>,
 #[value(skip_serializing_if = "Option::is_none")]
 pub offset: Option<Point3>,
 #[value(skip_serializing_if = "Option::is_none")]
 pub restraints: Option<Restraints>,
}

impl Patch<StructuralSupport> for StructuralSupportPatch {
 fn write(&self, base: &StructuralSupport) -> StructuralSupport { StructuralSupport { name: take(&self.name, &base.name), member: take(&self.member, &base.member), location: take(&self.location, &base.location), offset: take(&self.offset, &base.offset), restraints: take(&self.restraints, &base.restraints) } }
 fn restoring(&self, base: &StructuralSupport) -> Self { Self { name: restore(&self.name, &base.name), member: restore(&self.member, &base.member), location: restore(&self.location, &base.location), offset: restore(&self.offset, &base.offset), restraints: restore(&self.restraints, &base.restraints) } }
 fn merge(&mut self, later: Self) { merge_slot(&mut self.name, later.name); merge_slot(&mut self.member, later.member); merge_slot(&mut self.location, later.location); merge_slot(&mut self.offset, later.offset); merge_slot(&mut self.restraints, later.restraints); }
 fn minimal(&self, base: &StructuralSupport) -> Self { Self { name: changed(&self.name, &base.name), member: changed(&self.member, &base.member), location: changed(&self.location, &base.location), offset: changed(&self.offset, &base.offset), restraints: changed(&self.restraints, &base.restraints) } }
 fn touched(&self) -> Vec<String> { let mut out = Vec::new(); if self.name.is_some() { out.push("name".into()); } if self.member.is_some() { out.push("member".into()); } if self.location.is_some() { out.push("location".into()); } if self.offset.is_some() { out.push("offset".into()); } if self.restraints.is_some() { out.push("restraints".into()); } out }
 fn is_empty(&self) -> bool { self == &Self::default() }
}

/// 🩹️ Sparse authored patch of LoadCase.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct LoadCasePatch {
 #[value(skip_serializing_if = "Option::is_none")]
 pub name: Option<String>,
 #[value(skip_serializing_if = "Option::is_none")]
 pub category: Option<String>,
 #[value(skip_serializing_if = "Option::is_none")]
 pub factor: Option<f64>,
}

impl Patch<LoadCase> for LoadCasePatch {
 fn write(&self, base: &LoadCase) -> LoadCase { LoadCase { name: take(&self.name, &base.name), category: take(&self.category, &base.category), factor: take(&self.factor, &base.factor) } }
 fn restoring(&self, base: &LoadCase) -> Self { Self { name: restore(&self.name, &base.name), category: restore(&self.category, &base.category), factor: restore(&self.factor, &base.factor) } }
 fn merge(&mut self, later: Self) { merge_slot(&mut self.name, later.name); merge_slot(&mut self.category, later.category); merge_slot(&mut self.factor, later.factor); }
 fn minimal(&self, base: &LoadCase) -> Self { Self { name: changed(&self.name, &base.name), category: changed(&self.category, &base.category), factor: changed(&self.factor, &base.factor) } }
 fn touched(&self) -> Vec<String> { let mut out = Vec::new(); if self.name.is_some() { out.push("name".into()); } if self.category.is_some() { out.push("category".into()); } if self.factor.is_some() { out.push("factor".into()); } out }
 fn is_empty(&self) -> bool { self == &Self::default() }
}

/// 🩹️ Sparse authored patch of StructuralLoad.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(default)]
pub struct StructuralLoadPatch {
 #[value(skip_serializing_if = "Option::is_none")]
 pub name: Option<String>,
 #[value(skip_serializing_if = "Option::is_none")]
 pub load_case: Option<String>,
 #[value(skip_serializing_if = "Option::is_none")]
 pub member: Option<String>,
 #[value(skip_serializing_if = "Option::is_none")]
 pub location: Option<StructuralLocation>,
 #[value(skip_serializing_if = "Option::is_none")]
 pub force: Option<Point3>,
 #[value(skip_serializing_if = "Option::is_none")]
 pub moment: Option<Point3>,
}

impl Patch<StructuralLoad> for StructuralLoadPatch {
 fn write(&self, base: &StructuralLoad) -> StructuralLoad { StructuralLoad { name: take(&self.name, &base.name), load_case: take(&self.load_case, &base.load_case), member: take(&self.member, &base.member), location: take(&self.location, &base.location), force: take(&self.force, &base.force), moment: take(&self.moment, &base.moment) } }
 fn restoring(&self, base: &StructuralLoad) -> Self { Self { name: restore(&self.name, &base.name), load_case: restore(&self.load_case, &base.load_case), member: restore(&self.member, &base.member), location: restore(&self.location, &base.location), force: restore(&self.force, &base.force), moment: restore(&self.moment, &base.moment) } }
 fn merge(&mut self, later: Self) { merge_slot(&mut self.name, later.name); merge_slot(&mut self.load_case, later.load_case); merge_slot(&mut self.member, later.member); merge_slot(&mut self.location, later.location); merge_slot(&mut self.force, later.force); merge_slot(&mut self.moment, later.moment); }
 fn minimal(&self, base: &StructuralLoad) -> Self { Self { name: changed(&self.name, &base.name), load_case: changed(&self.load_case, &base.load_case), member: changed(&self.member, &base.member), location: changed(&self.location, &base.location), force: changed(&self.force, &base.force), moment: changed(&self.moment, &base.moment) } }
 fn touched(&self) -> Vec<String> { let mut out = Vec::new(); if self.name.is_some() { out.push("name".into()); } if self.load_case.is_some() { out.push("load_case".into()); } if self.member.is_some() { out.push("member".into()); } if self.location.is_some() { out.push("location".into()); } if self.force.is_some() { out.push("force".into()); } if self.moment.is_some() { out.push("moment".into()); } out }
 fn is_empty(&self) -> bool { self == &Self::default() }
}
