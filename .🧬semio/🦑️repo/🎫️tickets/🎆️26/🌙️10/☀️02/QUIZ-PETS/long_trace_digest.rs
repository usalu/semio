//! 🧪️ Ticket tool of work package M, the Rust half of the long-run proof that the Rust twin of the pets stage yields the same bits as `@semio-tech/pets`: it replays the sessions `long_trace_digest.ts` composed and played, digests them the same way and compares every checkpoint.
//!
//! Per session and simulated second two FNV-1a digests over IEEE-754 bit patterns (little-endian doubles): the running
//! digest of every frame so far (the digest of the stage-trace case) and the digest of every number of the stage at
//! that tick. The tool prints one line per session, the number of checkpoints compared and the first mismatch, writes
//! `rust-digests.json` beside the input (unless `--no-record` is given) and exits with 1 when anything differs.
//! `--probe <session> <from> <to>` instead writes the stage and the frame of one session after every tick of a window
//! (`probe-rust.jsonl`); the same option of the TypeScript half then names the first tick and member that differ.
//!
//! An example of the scratch crate `🗑️generated/wp-m/crate` (`stage_scratch.ts prepare`); from the repository root:
//!   bash .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/rust_scratch.sh wp-m run --release --offline --features sut --example long_trace_digest
//!
//! @see ./long_trace_digest.ts — the TypeScript half, which also composes the sessions

use pets::serde_json::{self, Map, Value};
use pets::{advance, frame_of, open_stage, Frame, Menagerie, Stage, StageEvent, Ticked, Ticks, ACTIVITIES, PET_MODES, TICKS_PER_SECOND};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const FNV_OFFSET_BASIS: u32 = 2_166_136_261;
const FNV_PRIME: u32 = 16_777_619;

/// 🔢️ FNV-1a over the eight bytes of a double, little-endian.
fn fold(hash: u32, value: f64) -> u32 {
    value.to_le_bytes().into_iter().fold(hash, |folded, byte| (folded ^ u32::from(byte)).wrapping_mul(FNV_PRIME))
}

/// 🔎️ An index as the number the TypeScript half folds: the position, or −1 for none.
fn found(position: Option<usize>) -> f64 {
    position.map_or(-1.0, |index| index as f64)
}

/// 🧮️ The running digest after one more frame: the digest of the stage-trace case.
fn fold_frame(hash: u32, menagerie: &Menagerie, frame: &Frame) -> u32 {
    let mut folded = [frame.tick as f64, f64::from(u8::from(frame.rate)), frame.wake.map_or(-1.0, |wake| wake as f64), frame.actors.len() as f64].into_iter().fold(hash, fold);
    for actor in &frame.actors {
        folded = fold(folded, found(menagerie.species.iter().position(|species| species.id == actor.species)));
        folded = [actor.x, actor.y, actor.facing.sign(), found(ACTIVITIES.iter().position(|activity| *activity == actor.activity)), actor.opacity].into_iter().fold(folded, fold);
        folded = actor.bones.iter().copied().fold(folded, fold);
        folded = actor.eyes.iter().flat_map(|eye| [eye.x, eye.y, eye.lid]).fold(folded, fold);
        folded = fold(folded, actor.mood);
    }
    folded
}

/// 🧾️ The digest of every number of a stage; names become indices (species in the menagerie, surfaces in the survey, clips in the species, −1 for none).
fn fold_stage(menagerie: &Menagerie, stage: &Stage) -> u32 {
    let kind = |id: &str| found(menagerie.species.iter().position(|species| species.id == id));
    let surface = |id: &str| found(stage.surfaces.iter().position(|candidate| candidate.id == id));
    let flag = |set: bool| if set { 1.0 } else { 0.0 };
    let mut numbers: Vec<f64> = vec![
        f64::from(stage.seed),
        stage.tick as f64,
        found(PET_MODES.iter().position(|mode| *mode == stage.mode)),
        flag(stage.quiet),
        stage.width,
        stage.height,
        flag(stage.pointer.is_some()),
        stage.pointer.map_or(0.0, |pointer| pointer.x),
        stage.pointer.map_or(0.0, |pointer| pointer.y),
        stage.pointed as f64,
        stage.glances.len() as f64,
    ];
    numbers.extend(stage.glances.iter().flat_map(|glance| [glance.x, glance.y]));
    numbers.extend([stage.surfaces.len() as f64, stage.keepouts.len() as f64, stage.perches.len() as f64]);
    numbers.extend(stage.perches.iter().flat_map(|perch| [surface(&perch.surface), perch.x0, perch.x1, perch.y]));
    numbers.push(stage.wanted.len() as f64);
    numbers.extend(stage.wanted.iter().map(|wanted| kind(wanted)));
    numbers.push(stage.actors.len() as f64);
    for actor in &stage.actors {
        let species = menagerie.species.iter().find(|species| species.id == actor.species);
        numbers.extend([
            kind(&actor.species),
            actor.perch.as_deref().map_or(-1.0, surface),
            actor.x,
            actor.y,
            actor.vx,
            actor.vy,
            actor.facing.sign(),
            actor.faced as f64,
            found(ACTIVITIES.iter().position(|activity| *activity == actor.activity)),
            actor.since as f64,
            actor.until as f64,
            actor.goal,
        ]);
        numbers.extend([actor.partner.as_deref().map_or(-1.0, kind), actor.clip.as_deref().zip(species).map_or(-1.0, |(id, species)| found(species.clips.iter().position(|clip| clip.id == id)))]);
        numbers.extend([actor.gaze.x, actor.gaze.y, actor.gaze.vx, actor.gaze.vy, actor.blink as f64, actor.mood, actor.needs.energy, actor.needs.sociability, actor.needs.curiosity, actor.opacity, flag(actor.leaving), f64::from(actor.draws)]);
    }
    numbers.push(stage.rapports.len() as f64);
    numbers.extend(stage.rapports.iter().flat_map(|rapport| [kind(&rapport.between[0]), kind(&rapport.between[1]), rapport.drift]));
    numbers.extend([stage.met as f64, f64::from(stage.draws)]);
    numbers.into_iter().fold(FNV_OFFSET_BASIS, fold)
}

/// 📖️ A JSON file of the long-trace folder.
fn document(folder: &Path, name: &str) -> Result<Value, String> {
    let path = folder.join(name);
    let text = std::fs::read_to_string(&path).map_err(|error| format!("{}: {error}", path.display()))?;
    serde_json::from_str(&text).map_err(|error| format!("{}: {error}", path.display()))
}

/// 🎬️ The digests of one session per second, `[frames so far, stage]` flat, replayed tick by tick like the TypeScript half played it.
fn replayed(menagerie: &Menagerie, session: &Value) -> Result<Vec<u32>, String> {
    let seed = session["seed"].as_u64().and_then(|seed| u32::try_from(seed).ok()).ok_or("a session carries no seed")?;
    let ticks: Ticks = session["ticks"].as_i64().ok_or("a session carries no ticks")?;
    let mut events: BTreeMap<Ticks, Vec<StageEvent>> = BTreeMap::new();
    for step in session["steps"].as_array().ok_or("a session carries no steps")? {
        let at = step["at"].as_i64().ok_or("a step carries no tick")?;
        events.entry(at).or_default().extend(serde_json::from_value::<Vec<StageEvent>>(step["events"].clone()).map_err(|error| format!("events at {at}: {error}"))?);
    }
    let mut digests = Vec::new();
    let mut stage = open_stage(seed);
    let mut hash = FNV_OFFSET_BASIS;
    for tick in 0..ticks {
        if let Some(events) = events.get(&tick) {
            stage = advance(menagerie, stage, events);
        }
        stage = advance(menagerie, stage, &[StageEvent::Ticked(Ticked { ticks: 1 })]);
        hash = fold_frame(hash, menagerie, &frame_of(menagerie, &stage));
        if (tick + 1) % TICKS_PER_SECOND == 0 {
            digests.extend([hash, fold_stage(menagerie, &stage)]);
        }
    }
    Ok(digests)
}

/// 🏁️ Replays every session, compares every checkpoint with the TypeScript half and answers whether all agree; `record` writes the digests of this replay beside the input.
fn compared(folder: &Path, record: bool) -> Result<bool, String> {
    let sessions = document(folder, "sessions.json")?;
    let typescript = document(folder, "typescript-digests.json")?;
    let mut menageries: BTreeMap<String, Menagerie> = BTreeMap::new();
    for (name, menagerie) in sessions["menageries"].as_object().ok_or("sessions.json carries no menageries")? {
        menageries.insert(name.clone(), serde_json::from_value(menagerie.clone()).map_err(|error| format!("menagerie {name}: {error}"))?);
    }
    let mut recorded = Map::new();
    let (mut checkpoints, mut mismatches, mut first) = (0_usize, 0_usize, None);
    let listed = sessions["sessions"].as_array().ok_or("sessions.json carries no sessions")?;
    for session in listed {
        let id = session["id"].as_str().ok_or("a session carries no id")?;
        let menagerie = session["menagerie"].as_str().and_then(|name| menageries.get(name)).ok_or_else(|| format!("{id}: unknown menagerie"))?;
        let digests = replayed(menagerie, session)?;
        let expected: Vec<u32> = serde_json::from_value(typescript[id].clone()).map_err(|error| format!("{id}: no TypeScript digests: {error}"))?;
        let mut differing = usize::from(digests.len() != expected.len());
        for (second, (rust, twin)) in digests.chunks(2).zip(expected.chunks(2)).enumerate() {
            checkpoints += 1;
            if rust == twin {
                continue;
            }
            differing += 1;
            first.get_or_insert_with(|| format!("{id} at second {} (tick {}): frames rust {} typescript {}, stage rust {} typescript {}", second + 1, (second as Ticks + 1) * TICKS_PER_SECOND, rust[0], twin[0], rust[1], twin[1]));
        }
        mismatches += differing;
        println!("[DEBUG] {id}: {} checkpoints, {differing} mismatches, last {} {}", digests.len() / 2, digests[digests.len() - 2], digests[digests.len() - 1]);
        recorded.insert(id.to_string(), serde_json::json!(digests));
    }
    if record {
        std::fs::write(folder.join("rust-digests.json"), Value::Object(recorded).to_string()).map_err(|error| format!("rust-digests.json: {error}"))?;
    }
    println!("[DEBUG] rust against typescript: {} sessions, {checkpoints} checkpoints compared (two digests each), {mismatches} mismatches; first mismatch: {}", listed.len(), first.as_deref().unwrap_or("none"));
    Ok(mismatches == 0)
}

/// 🩺️ The probe: replays one recorded session and writes its stage and frame after every tick of a window as JSON lines (`probe-rust.jsonl`), for the `--probe` of the TypeScript half to compare with.
fn probed(folder: &Path, id: &str, from: Ticks, to: Ticks) -> Result<bool, String> {
    let sessions = document(folder, "sessions.json")?;
    let session = sessions["sessions"].as_array().and_then(|listed| listed.iter().find(|session| session["id"] == id)).ok_or_else(|| format!("no recorded session {id}"))?;
    let menagerie: Menagerie = serde_json::from_value(sessions["menageries"][session["menagerie"].as_str().unwrap_or_default()].clone()).map_err(|error| format!("menagerie of {id}: {error}"))?;
    let mut events: BTreeMap<Ticks, Vec<StageEvent>> = BTreeMap::new();
    for step in session["steps"].as_array().ok_or("a session carries no steps")? {
        events.entry(step["at"].as_i64().ok_or("a step carries no tick")?).or_default().extend(serde_json::from_value::<Vec<StageEvent>>(step["events"].clone()).map_err(|error| format!("events: {error}"))?);
    }
    let mut lines = String::new();
    let mut stage = open_stage(session["seed"].as_u64().and_then(|seed| u32::try_from(seed).ok()).ok_or("a session carries no seed")?);
    for tick in 0..Ticks::min(to, session["ticks"].as_i64().ok_or("a session carries no ticks")?) {
        if let Some(events) = events.get(&tick) {
            stage = advance(&menagerie, stage, events);
        }
        stage = advance(&menagerie, stage, &[StageEvent::Ticked(Ticked { ticks: 1 })]);
        if tick + 1 >= from {
            lines.push_str(&serde_json::json!({ "tick": tick + 1, "stage": stage, "frame": frame_of(&menagerie, &stage) }).to_string());
            lines.push('\n');
        }
    }
    std::fs::write(folder.join("probe-rust.jsonl"), lines).map_err(|error| format!("probe-rust.jsonl: {error}"))?;
    println!("[DEBUG] probe: ticks {from}…{to} of {id} written");
    Ok(true)
}

fn main() -> ExitCode {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let folder = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../long-trace");
    let outcome = match arguments.iter().position(|argument| argument == "--probe") {
        Some(at) if arguments.len() > at + 3 => probed(&folder, &arguments[at + 1], arguments[at + 2].parse().unwrap_or(0), arguments[at + 3].parse().unwrap_or(0)),
        _ => compared(&folder, !arguments.iter().any(|argument| argument == "--no-record")),
    };
    match outcome {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(error) => {
            eprintln!("[DEBUG] {error}");
            ExitCode::from(2)
        }
    }
}
