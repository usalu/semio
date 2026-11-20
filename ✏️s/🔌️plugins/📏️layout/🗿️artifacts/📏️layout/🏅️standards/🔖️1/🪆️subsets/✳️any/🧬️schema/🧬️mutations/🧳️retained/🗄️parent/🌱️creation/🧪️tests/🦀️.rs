//! 🗄️ Parent insertion transfers its original typed rows and retires each cancelled cache.
use super::observe_test_turn as observe;
use crate::LayoutMutation;
use semio_s_artifact_layout_layout_diff::LayoutDiff;
use semio_s_artifact_layout_layout_model::{ParentPage,snapshot::LayoutSnapshot};
use semio_s_artifact_layout_layout_mutations_decisions::parent_creation::ParentCreationOutput;
use semio_framework_value::{RetirementDemand,RetainedCloneGrant};
use semio_framework_value_clone_source_testing::FixtureSource;

fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn decode<T:semio_framework_value::FromValue>(v:&serde_json::Value)->T{semio_framework_pack_json::from_json_str(&v.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()}
fn close(p:&mut ParentCreationOutput){
    p.begin_close();
    for _ in 0..100000{
        if p.terminal_is_empty(){return;}
        let body=256;
        let d=p.close_demand(body).unwrap();
        assert_eq!(observe(Default::default(),|effect,recover|p.close_step(None,body,effect,recover)).progress(),Default::default());
        observe(grant(d),|effect,recover|p.close_step(None,body,effect,recover));
    }
    panic!("parent creation close stalled")
}
#[test]
fn original_parent_rows_keep_physical_custody_and_independent_cache_witnesses(){let corpus:serde_json::Value=serde_json::from_str(&crate::test_fixture_text("../../🧳️retained/🗄️parent/🌱️creation/🧪️tests/🔣️.json")).unwrap();let common:serde_json::Value=serde_json::from_str(&crate::test_fixture_text("../../🧳️retained/🎚️simple/🧪️tests/🔣️.json")).unwrap();for row in corpus["cases"].as_array().unwrap(){let event=LayoutMutation::CreateParentPage(crate::create_parent_page::CreateParentPage{page:decode(&row["page"]),index:row["index"].as_u64().map(|i|i as usize)});for cut in std::iter::once(usize::MAX).chain(corpus["cuts"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as usize)){let authored=FixtureSource::from_owner(event.clone());let base=FixtureSource::from_owner(decode::<LayoutSnapshot>(&common["base"]));let mut prepared=vec![event.clone()];let(id,frames,layers)=match &prepared[0]{LayoutMutation::CreateParentPage(p)=>(p.page.id.as_ptr(),p.page.frames.as_ptr(),p.page.layers.as_ptr()),_=>unreachable!()};let mut diff=LayoutDiff::default();let mut owner=ParentCreationOutput::default();for at in 0..cut{if owner.ready(){break}assert!(at<100000);let d=owner.demand(authored.try_borrow().unwrap(),base.try_borrow().unwrap(),256).unwrap();assert_eq!(observe(Default::default(),|effect,recover|owner.advance(&mut prepared,authored.try_borrow().unwrap(),base.try_borrow().unwrap(),&mut diff,256,1024,effect,recover)).progress(),Default::default());observe(grant(d),|effect,recover|owner.advance(&mut prepared,authored.try_borrow().unwrap(),base.try_borrow().unwrap(),&mut diff,256,1024,effect,recover));}if cut==usize::MAX{assert!(owner.ready());let insertion=&diff.parent_pages.as_ref().unwrap().inserted[0];assert_eq!(insertion.row,decode::<ParentPage>(&row["expected"]));assert_eq!(insertion.row.id.as_ptr(),id);assert_eq!(insertion.row.frames.as_ptr(),frames);assert_eq!(insertion.row.layers.as_ptr(),layers);assert_eq!(insertion.index,row["index"].as_u64().map(|i|i as usize).unwrap_or(base.try_borrow().unwrap().get().parent_pages.len()));}close(&mut owner);drop(FixtureSource::from_owner(diff));drop(FixtureSource::from_owner(prepared));}}}
