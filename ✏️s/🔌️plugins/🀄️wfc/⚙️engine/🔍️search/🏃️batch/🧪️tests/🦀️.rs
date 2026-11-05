use super::*;
use semio_s_plugin_wfc_engine_oracle::step_context::TestJobDriver;
use crate::model::ModelBuilder;
use crate::topology::GraphTopologyBuilder;

#[test]
fn one_fuel_supervision_alternates_actual_child_work_and_paid_yield(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️phase/🔣️.json")).unwrap();
    let mut model=ModelBuilder::new();
    let a=model.add_pattern(1.0);
    let b=model.add_pattern(1.0);
    let adjacent=model.add_relation("adjacent");
    model.allow_mirrored(adjacent,a,b);
    let operation=Operation::new(allocate_operation_id(),RevisionId(1),Generation(1),7);
    let topology=GraphTopologyBuilder::new(4).build().unwrap();
    let mut job=SearchJob {inner:WfcJob::new(operation,model.compile().unwrap(),topology,WfcJobConfig::default(),None,Vec::new()),yield_next:false};
    let mut driver=TestJobDriver::new(operation.operation,operation.generation);
    let expected=fixture["turns"].as_array().unwrap();
    let mut observed=Vec::new();
    for _ in 0..4096 {
        let yielded=driver.step(&mut job,fixture["fuel"].as_u64().unwrap(),|outcome|matches!(outcome,Some(JobOutcomeBorrow::Yield {..})));
        if job.yield_next||!observed.is_empty() {observed.push(serde_json::json!({"yielded":yielded,"yieldNext":job.yield_next}));}
        if observed.len()==expected.len() {break;}
    }
    assert_eq!(&observed,expected);
    driver.close(&mut job);
    assert!(job.terminal_is_empty());
}
