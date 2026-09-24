import pathlib
p = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs")
t = p.read_text()
def swap(old, new, count=1):
    global t
    assert t.count(old) == count, (t.count(old), old[:100])
    t = t.replace(old, new)
swap('''pub enum ShardOutcome {
    Turn {
        actor: u64,
        result: semio_framework_actor::TurnResult,
    },''', '''pub enum ShardOutcome {
    /// 🔁️ One granted turn's result, with the step authority of every live job the turn's
    /// `Effect::SpawnJob`s admitted — the host needs a job's exact `JobTurn` to grant its steps
    /// (`Payload::JobStep`), and only this shard mints it.
    Turn {
        actor: u64,
        result: semio_framework_actor::TurnResult,
        jobs: Vec<JobTurn>,
    },''')
swap('''            Self::Turn { actor, result } => {
                semio_framework_actor::pack::write_u8(out, 0).await;
                semio_framework_actor::pack::write_u64(out, *actor).await;
                result.pack_encode(out).await?;
            }''', '''            Self::Turn { actor, result, jobs } => {
                semio_framework_actor::pack::write_u8(out, 0).await;
                semio_framework_actor::pack::write_u64(out, *actor).await;
                result.pack_encode(out).await?;
                semio_framework_actor::pack::write_u64(out, jobs.len() as u64).await;
                for job in jobs {
                    job.pack_encode(out).await;
                }
            }''')
swap('''            0 => Ok(Self::Turn { actor, result: semio_framework_actor::TurnResult::pack_decode(bytes, pos).await? }),''', '''            0 => {
                let result = semio_framework_actor::TurnResult::pack_decode(bytes, pos).await?;
                let count = semio_framework_actor::pack::read_u64(bytes, pos, "ShardOutcome::Turn::jobs").await?;
                if count > SHARD_TURN_ADMITTED_JOBS_MAXIMUM as u64 {
                    return Err(semio_framework_actor::pack::PackError::InvalidTag { what: "ShardOutcome::Turn::jobs", tag: u8::MAX, offset: *pos });
                }
                let mut jobs = Vec::with_capacity(count as usize);
                for _ in 0..count {
                    jobs.push(JobTurn::pack_decode(bytes, pos).await?);
                }
                Ok(Self::Turn { actor, result, jobs })
            }''')
swap('''/// 📤️ Owned pack-coded outcome sent from a shard to its scheduler-side consumer.''', '''/// 🧮️ Most live jobs one turn can admit: one per granted effect.
const SHARD_TURN_ADMITTED_JOBS_MAXIMUM: usize = 1_024;

/// 📤️ Owned pack-coded outcome sent from a shard to its scheduler-side consumer.''')
# admission collects authorities
swap('''                let bridged = to_actor_turn_result_in_place(&mut result, actor_id, 0, 0).await;''', '''                let bridged = to_actor_turn_result_in_place(&mut result, actor_id, 0, 0).await;
                let mut admitted_jobs = Vec::new();''')
swap('''                            match mounted {
                                Ok(seed) => {
                                    let slot = self.replay_seeds.iter_mut().find(|slot| slot.is_none()).expect("preflighted fixed replay seed slot");
                                    *slot = Some(seed);
                                }''', '''                            match mounted {
                                Ok(seed) => {
                                    let slot = self.replay_seeds.iter_mut().find(|slot| slot.is_none()).expect("preflighted fixed replay seed slot");
                                    *slot = Some(seed);
                                    admitted_jobs.push(authority);
                                }''')
swap('''                match bridged {
                    Ok(result) => ShardOutcome::Turn { actor: actor_id, result },
                    Err(fault) => ShardOutcome::Fault { actor: actor_id, message: fault.message },
                }
            }''', '''                match bridged {
                    Ok(result) => ShardOutcome::Turn { actor: actor_id, result, jobs: admitted_jobs },
                    Err(fault) => ShardOutcome::Fault { actor: actor_id, message: fault.message },
                }
            }''')
swap('''                match to_actor_turn_result(result, actor_id, 0, 0).await {
                    Ok(result) => ShardOutcome::Turn { actor: actor_id, result },''', '''                match to_actor_turn_result(result, actor_id, 0, 0).await {
                    Ok(result) => ShardOutcome::Turn { actor: actor_id, result, jobs: Vec::new() },''')
# a step for a still-seeding job drives its seed first
swap('''                DeferredAuthority::JobStep { actor, turn } => {
                    self.accept_job_turn(actor, turn)?;
                    selected_step = Some((actor, turn));
                }''', '''                DeferredAuthority::JobStep { actor, turn } => {
                    while self.replay_seeds.iter().flatten().any(|seed| seed.actor == actor && seed.job == turn.job && !matches!(seed.phase, ReplaySeedPhase::Retained)) {
                        if !self.drive_replay_seed().await? {
                            break;
                        }
                    }
                    self.accept_job_turn(actor, turn)?;
                    selected_step = Some((actor, turn));
                }''')
p.write_text(t)
print("shard edited")
