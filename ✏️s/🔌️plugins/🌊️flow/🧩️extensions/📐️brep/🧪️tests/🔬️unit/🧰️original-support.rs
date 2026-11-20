fn observe<T>(body:impl FnOnce()->T)->(T,usize,usize){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(body);(value,heap.requested_bytes,heap.released_bytes)}

fn stock()->Grant{Grant{maximum_items:65536,maximum_copy_bytes:128<<20,maximum_capacity_bytes:256<<20,maximum_release_bytes:512<<20,maximum_depth:4096}}

fn run(caller:&mut OriginalJobFixtureCaller,g:Grant,born:&mut usize,free:&mut usize,body:impl FnOnce(&mut StepContext<'_>)->Result<(),OriginalTurnFailure>){let(_,a,r)=observe(||caller.turn(g,|cx|cx.with_original_retirement(|cx|cx.with_original_work_loan(1,1500,body))).unwrap());let p=caller.receipt();assert!(p.fits(g));assert_eq!((a,r),(p.retained_capacity_bytes,p.released_bytes));*born+=a;*free+=r;}

fn number(value:f64)->Value{Value::Dictionary(Dictionary::with_schema("number").insert("value",Value::Atom(Atom::Decimal(value))))}

fn demand(job:&dyn OperatorJob,copy:usize)->RetirementDemand{RetirementDemand{copy_bytes:job.next_close_copy_byte_demand().unwrap(),capacity_bytes:job.next_close_capacity_byte_demand(copy).unwrap(),release_bytes:job.next_close_release_byte_demand().unwrap(),depth:job.next_close_depth_demand().unwrap()}}

fn drive_close(job:&mut Option<Box<dyn OperatorJob>>,d:RetirementDemand,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
 if !cx.retained_work_can_enter(d){return Ok(())}if job.as_ref().unwrap().terminal_is_empty(){let bytes=std::mem::size_of_val(&**job.as_ref().unwrap());drop(job.take());cx.consume_fuel(1);cx.consume_retained(Progress{copied_items:1,released_bytes:bytes,..Default::default()})?;}else{close_original_effect_step(cx,d,|effect|{let turn=job.as_mut().unwrap().close_step(effect);let work=match &turn{RetainedWorkPoll::Complete(outcome)=>outcome.work,RetainedWorkPoll::Pending{work}=>*work};assert!(work<=1);turn})?;}Ok(())
}

fn fixture_xyz(raw: &serde_json::Value) -> [f64; 3] {
    std::array::from_fn(|axis| raw[axis].as_f64().unwrap())
}

fn evaluate_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../../../🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🎯️brep-evaluate/🔣️.json")).unwrap()
}

fn run_funded_preview(
    session: &Session,
    identity: flow_extension_sdk::geometry::GeometryWorkIdentity,
    handle: &str,
    caller: &mut OriginalJobFixtureCaller,
    born: &mut usize,
    free: &mut usize,
) -> semio_framework::MeshData {
    use flow_extension_sdk::geometry::GeometryStep;
    let tolerance = 0.025;
    for _ in 0..20000 {
        let demand = session.tessellation_demands(identity, handle, tolerance, 128).unwrap();
        let grant = Grant {
            maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(128),
            maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1),
        };
        let mut output = None;
        run(caller, grant, born, free, |cx| {
            let (step, work) = session.tessellate_step(identity, handle, tolerance, cx).unwrap();
            assert!(work.consumed_units <= 1);
            assert!(work.retained.fits(grant));
            match step {
                GeometryStep::Ready(mesh) => output = Some(mesh),
                GeometryStep::Working { .. } => {},
                _ => panic!("geometry preview stopped before completion"),
            }
            Ok(())
        });
        if let Some(output) = output { return output; }
    }
    panic!("geometry preview did not complete")
}

fn preview_area_volume(mesh: &semio_framework::MeshData) -> (f64, f64) {
    use semio_framework_3d_brep_vector::Vec3;
    let point = |index: u32| {
        let i = index as usize * 3;
        Vec3::new(mesh.positions[i].into(), mesh.positions[i + 1].into(), mesh.positions[i + 2].into())
    };
    mesh.indices.chunks_exact(3).fold((0., 0.), |(area, volume), triangle| {
        let (a, b, c) = (point(triangle[0]), point(triangle[1]), point(triangle[2]));
        (area + (b-a).cross(c-a).norm() * 0.5, volume + a.dot(b.cross(c)) / 6.)
    })
}

fn polygon_fixture_input(raw: &serde_json::Value) -> Dictionary {
    raw.as_object().unwrap().iter().fold(Dictionary::new(), |parameters, (key, value)| {
        let value = if let Some(points) = value.as_array() {
            let list = points.iter().enumerate().fold(Dictionary::with_schema("list"), |list, (index, point)| {
                list.insert(index.to_string(), Value::Dictionary(semio_s_spatial_kernel_semio_session::point_dictionary(fixture_xyz(point))))
            });
            Value::Dictionary(list)
        } else {
            number(value.as_f64().unwrap())
        };
        parameters.insert(key.as_str(), value)
    })
}

fn run_live_query(registry: &Registry, operator: &str, input: Dictionary, caller: &mut OriginalJobFixtureCaller, born: &mut usize, free: &mut usize) -> neural_engine::ColdOwner<Dictionary> {
    let mut job = Some(admit_live_job(registry, operator, input));
    let mut output = None;
    for _ in 0..10000 {
        run(caller, Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() }, born, free, |cx| {
            let (step, receipt) = job.as_mut().unwrap().step(cx).unwrap();
            assert_eq!(receipt, job.as_ref().unwrap().normal_step_progress());
            if let neural_engine::OperatorJobStep::Done(value) = step { output = Some(value); }
            Ok(())
        });
        if output.is_some() { break; }
    }
    close_live_job(&mut job, caller, born, free);
    neural_engine::ColdOwner::new(output.unwrap_or_else(|| panic!("{operator} should complete with one fuel")))
}

fn admit_live_job(registry: &Registry, operator: &str, input: Dictionary) -> Box<dyn OperatorJob> {
    let mut input = Some(input);
    let mut plan = OperatorPlanCursor::new();
    for _ in 0..10000 {
        match registry.dispatch_job(operator, &mut input, &mut plan, stock()).unwrap() {
            OperatorPlanStep::Working => {}
            OperatorPlanStep::Admitted(OperatorPlanAdmission::Job(job)) => return job,
            _ => panic!("{operator} must admit a funded job"),
        }
    }
    panic!("{operator} never admitted a funded job")
}

fn close_live_job(job: &mut Option<Box<dyn OperatorJob>>, caller: &mut OriginalJobFixtureCaller, born: &mut usize, free: &mut usize) {
    for _ in 0..10000 {
        let Some(owner) = job.as_ref() else { break };
        let d = if owner.terminal_is_empty() {
            RetirementDemand { release_bytes: std::mem::size_of_val(&**owner), depth: 1, ..Default::default() }
        } else {
            demand(&**owner, stock().maximum_copy_bytes)
        };
        run(caller, stock(), born, free, |cx| drive_close(job, d, cx));
    }
    assert!(job.is_none());
}

fn retained_boolean_result(
    registry: &Registry, operator: &str, input: Dictionary,
    caller: &mut OriginalJobFixtureCaller, born: &mut usize, free: &mut usize,
) -> neural_engine::ColdOwner<Dictionary> {
    let mut job = Some(admit_live_job(registry, operator, input));
    let grant = Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() };
    let mut output = None;
    for _ in 0..2000 {
        run(caller, grant, born, free, |cx| {
            if let neural_engine::OperatorJobStep::Done(value) = job.as_mut().unwrap().step(cx).unwrap().0 { output = Some(value); }
            Ok(())
        });
        if output.is_some() { break; }
    }
    close_live_job(&mut job, caller, born, free);
    neural_engine::ColdOwner::new(output.unwrap_or_else(|| panic!("{operator} must complete under original one-item stock")))
}
