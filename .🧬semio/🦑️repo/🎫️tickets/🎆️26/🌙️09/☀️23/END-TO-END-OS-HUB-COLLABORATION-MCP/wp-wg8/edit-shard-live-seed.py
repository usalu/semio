import pathlib

path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs")
text = path.read_text()


def replace(old, new):
    global text
    assert text.count(old) == 1, (text.count(old), old[:100])
    text = text.replace(old, new)


replace(
    "struct MountedReplaySeed {\n    actor: u64,\n    job: u64,\n    authority: JobTurn,\n    placement: JobPlacement,\n",
    """/// 🌱️ One admitted `Effect::SpawnJob` from capture to its end. A `replayable` seed captures the
/// spawning instance's checkpoint before it starts the job, so a host can later replay it on another
/// worker; a framework reserved tool job (`semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND`) is
/// live-only and starts right after its kind and input pages — its hosts never replay it, and a whole
/// guest checkpoint outgrows the fixed checkpoint pages (block2d's undo, ticket 26/09/23 slice WG8 §1.4).
struct MountedReplaySeed {
    actor: u64,
    job: u64,
    authority: JobTurn,
    placement: JobPlacement,
    replayable: bool,
""",
)

replace(
    "        Ok(Self {\n            actor,\n            job,\n            authority,\n            placement,\n            worker_count: 0,\n",
    "        Ok(Self {\n            actor,\n            job,\n            authority,\n            placement,\n            replayable: kind != semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND,\n            worker_count: 0,\n",
)

replace(
    '                    Ok(true) => self.replay_seeds[index].as_mut().expect("capture seed").phase = ReplaySeedPhase::Checkpoint,\n',
    """                    Ok(true) => {
                        let seed = self.replay_seeds[index].as_mut().expect("capture seed");
                        seed.phase = if seed.replayable { ReplaySeedPhase::Checkpoint } else { ReplaySeedPhase::Start };
                    }
""",
)

replace(
    """        let retained = seed.seed.as_ref().ok_or_else(|| PluginHostError::Plugin(format!("ShardLoop::replay: seed for actor {actor}, job {} is closing", turn.job)))?;
""",
    """        if !seed.replayable {
            return Err(PluginHostError::Plugin(format!("ShardLoop::replay: job {} of actor {actor} is a live-only framework reserved tool job with no restore checkpoint", turn.job)));
        }
        let retained = seed.seed.as_ref().ok_or_else(|| PluginHostError::Plugin(format!("ShardLoop::replay: seed for actor {actor}, job {} is closing", turn.job)))?;
""",
)

path.write_text(text)
print("ok")
