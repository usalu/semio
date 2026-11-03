#!/usr/bin/env python3
"""puzzle3d's Puzzle3dWindowConfigOwner never declared its original initial-state / native-pair birth (trait defaults refuse: "Window owner must declare its original initial state birth"), so every puzzle3d window command faulted at window-config birth. Mirrors the energy owner (fail-closed single anchor)."""
p="✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs"
s=open(p).read()
a="    const MAXIMUM_PREPARATION_DEPTH: usize = 64;\n    fn build_retained_edit() -> std::sync::Arc<Self::Edit> { std::sync::Arc::new(semio_framework_plugin::WindowConfigApplyEdit::new()) }"
assert s.count(a)==1
b="""    const MAXIMUM_PREPARATION_DEPTH: usize = 64;
    fn initial_state_demands()->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Ok(semio_framework_value::RetirementDemand{copy_bytes:2*std::mem::size_of::<Self::State>(),depth:1,..Default::default()})}
    fn initial_state(grant:semio_framework_value::RetainedCloneGrant)->Result<(Self::State,semio_framework_value::RetainedCloneProgress),semio_framework_value::ValueError>{let d=Self::initial_state_demands()?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original Puzzle 3D window config birth is not funded"))}Ok((Self::State::default(),semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
    fn initial_state_pack_source(state:&Self::State)->Result<(&dyn semio_framework_dsl_record::native_encoding::FieldProjectionSource,semio_framework_dsl_record::BorrowedRecordSpec),semio_framework_value::ValueError>{Ok((state,<Self::State as semio_framework_dsl_record::BorrowedDslRecord>::RECORD))}
    fn build_retained_edit() -> std::sync::Arc<Self::Edit> { std::sync::Arc::new(semio_framework_plugin::WindowConfigApplyEdit::new()) }"""
open(p,"w").write(s.replace(a,b));print("applied")
