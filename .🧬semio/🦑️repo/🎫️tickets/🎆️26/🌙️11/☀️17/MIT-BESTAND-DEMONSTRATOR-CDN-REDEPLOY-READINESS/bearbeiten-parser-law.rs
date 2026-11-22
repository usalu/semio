//! 📥️ Quantifies actual Process contribution parsing under the component's original actor Source.
use semio_framework_actor::{OriginalActorTurnWork, RetainedTurnInput, retained_context::OriginalActorContextSlot};
use semio_framework_job::{OriginalConstructionAdmission, OriginalTurnFailure, StepBudget, StepContext};
use semio_framework_value::{DslValue, RetainedCloneGrant, RetainedCloneProgress};
use semio_plugin_decl_emission::{OriginalContributionProjection, OriginalHostConfigurationDeclaration as Declaration, OriginalHostConfigurationProjection as Projection};
use semio_plugin_host_configuration::OriginalHostConfiguration;

static PROCESS: Declaration<String> = Declaration {
    maximum_input_bytes: 262144, maximum_output_bytes: 24576,
    projection: Projection::Contributions(OriginalContributionProjection {
        maximum_depth:32, maximum_items:4096, maximum_string_bytes:16384,
        topic:"process.machines", app_id:"process3d-play", payload_fields:&["appId","moduleId","label","iconId","machinesJson"],
        deduplicate_modules:false,excluded_module_ids:&[],maximum_modules:usize::MAX,
    }), mutation: |text| text,
};
const TURN: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items:64, maximum_copy_bytes:32768, maximum_capacity_bytes:16<<20,
    maximum_release_bytes:1<<20, maximum_depth:64,
};

fn clock() -> Option<u64> { Some(100) }
fn budget(input:RetainedTurnInput) -> StepBudget { StepBudget::new(1<<16,input.work.absolute_deadline_us,input.grant) }
fn update(input:&mut RetainedTurnInput,returned:semio_framework_actor::RetainedTurnReceipt){
    input.epoch += 1;
    input.grant = returned.remaining;
    input.work.fuel = returned.remaining_fuel;
}

fn turn<T: Copy>(actor:&mut OriginalActorContextSlot,input:&mut RetainedTurnInput,run:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->(T,RetainedCloneProgress){
    let mut spent=Default::default();
    let call=actor.dispatch_step(*input,budget(*input),&mut spent,clock,|cx|{
        cx.acknowledge_return_rows_locally();
        cx.with_retained_loan(TURN,run)
    });
    update(input,call.receipt().expect("actual caller receipt"));
    (call.result.expect("host configuration caller"),spent)
}

fn run(name:&str,text:&str,expected:&str) {
    let mut input=RetainedTurnInput{
        operation:92,generation:1,epoch:1,
        work:OriginalActorTurnWork{clock_binding:92,absolute_deadline_us:30_000_000,fuel:50_000_000},
        grant:RetainedCloneGrant{maximum_items:1<<20,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<28,maximum_release_bytes:1<<28,maximum_depth:64},
    };
    let mut actor=OriginalActorContextSlot::empty();
    assert!((0..64).any(|_|{
        let call=actor.admit_step(input,budget(input),clock);
        update(&mut input,call.receipt().unwrap());
        call.result.unwrap().ready
    }));
    let mut source=None;
    let mut admission=OriginalConstructionAdmission::default();
    let mut args=Some(DslValue::object([("json".into(),DslValue::String(text.into()))]));
    assert!((0..64).any(|_|turn(&mut actor,&mut input,|cx|OriginalHostConfiguration::admit(&PROCESS,&mut args,cx,&mut admission,&mut source)).0));
    turn(&mut actor,&mut input,|cx|{
        let before=(cx.retained_progress(),cx.original_work_units_spent());
        let denied=RetainedCloneGrant{maximum_items:0,..TURN};
        assert!(!cx.with_retained_loan(denied,|cx|source.as_mut().unwrap().advance(cx))?);
        assert_eq!((cx.retained_progress(),cx.original_work_units_spent()),before);
        Ok(())
    });
    let mut empty_streak=0;
    let mut maximum_empty=0;
    let steps=(0usize..120_000).position(|index|{
        let (ready,spent)=turn(&mut actor,&mut input,|cx|source.as_mut().unwrap().advance(cx));
        if spent==Default::default(){empty_streak+=1;}else{empty_streak=0;}
        maximum_empty=maximum_empty.max(empty_streak);
        if (index+1).is_power_of_two(){eprintln!("[DEBUG] parser {name} turn {} ready {ready} empty {empty_streak}",index+1);}
        ready
    }).expect("bounded original host producer settlement")+1;
    let mut mutation=None;
    assert!((0..64).any(|_|turn(&mut actor,&mut input,|cx|source.as_mut().unwrap().take_into(&mut mutation,cx)).0));
    assert_eq!(mutation.as_deref(),Some(expected));
    eprintln!("[DEBUG] parser {name} completed {steps} owner turns (max consecutive wholly empty {maximum_empty}), {} input bytes, {} output bytes",text.len(),expected.len());
    assert!((0..120_000).any(|_|turn(&mut actor,&mut input,|cx|cx.with_original_retirement(|cx|source.as_mut().unwrap().close_step(cx))).0));
    let mut release=Some((source,mutation));
    assert!((0..64).any(|_|turn(&mut actor,&mut input,|cx|cx.with_original_retirement(|cx|{
        let progress=RetainedCloneProgress{copied_items:1,released_bytes:OriginalHostConfiguration::<String>::release_demands().release_bytes+release.as_ref().unwrap().1.as_ref().unwrap().capacity(),..Default::default()};
        if !progress.fits(cx.retained_grant())||cx.retirement_should_yield(){return Ok(false)}
        drop(release.take());
        cx.consume_retained(progress)?;
        cx.consume_fuel(1);
        Ok(true)
    })).0));
    assert!((0..8192).any(|_|{
        let call=actor.close_step(input,budget(input)); update(&mut input,call.receipt().unwrap());
        call.result.unwrap();
        actor.terminal_is_empty()
    }));
}

fn main(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!(env!("SEMIO_BEARBEITEN_JSON_FIXTURE"))).unwrap();
    for case in fixture["cases"].as_array().unwrap(){
        let name=case["name"].as_str().unwrap();
        if case["policy"]=="process"{
            run(name,case["input"].as_str().unwrap(),case["expected"].as_str().unwrap());
        }
    }
}
