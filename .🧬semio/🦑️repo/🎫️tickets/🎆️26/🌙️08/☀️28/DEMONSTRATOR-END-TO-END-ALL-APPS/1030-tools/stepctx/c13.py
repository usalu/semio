import glob,re,os
from rep import rep
R='/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/'
G=glob.glob(R+'🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry')[0]
rep(G+'/⚙️compute/🦀️.rs',[("#[cfg(test)]\n#[path = \"../💎️value/🧪️tests/🦀️.rs\"]","""/// 🧪️ Test drivers: a job stepped under a context that funds exactly the requested units.
#[cfg(any(test, feature = "testkit"))]
pub mod testing {
    use super::{WidgetJob, WidgetStep};

    /// 🏃️ Runs one turn of `job` funded with `fuel` units; a turn that admitted no progress reports `Working` at zero.
    pub fn step_with_fuel<J: WidgetJob + ?Sized>(job: &mut J, fuel: usize) -> WidgetStep {
        const GRANT: semio_framework_value::RetainedCloneGrant = semio_framework_value::RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 2 << 20, maximum_depth: 128 };
        let (mut sequence, mut receipt) = (0, semio_framework_value::RetainedCloneProgress::default());
        let budget = semio_framework_job::StepBudget::new(fuel as u64, u64::MAX, GRANT);
        let mut cx = semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(1), budget, semio_framework_job::root_cancel_token(), semio_framework_job::default_now_us, &mut sequence, &mut receipt);
        job.step(&mut cx).expect("the funded test turn is admitted").unwrap_or(WidgetStep::Working { progress: 0.0 })
    }
}

#[cfg(test)]
#[path = "../💎️value/🧪️tests/🦀️.rs\"]""".replace('\\"]','"]'))])
# Cargo features on support crates
for d in ['⏱️geometry-phased-job-support','🏗️geometry-construction-support','🔮️geometry-oracle-support','🧱️geometry-mesh-harness']:
    for f in glob.glob(R+'🧪️tests/'+d+'/**/Cargo.toml',recursive=True):
        s=open(f,encoding='utf8').read()
        old='semio-s-artifact-procedural-3d-geometry-compute = { workspace = true }'
        if old in s:
            open(f,'w',encoding='utf8').write(s.replace(old,'semio-s-artifact-procedural-3d-geometry-compute = { workspace = true, features = ["testkit"] }',1)); print('cargo',f[-60:])
