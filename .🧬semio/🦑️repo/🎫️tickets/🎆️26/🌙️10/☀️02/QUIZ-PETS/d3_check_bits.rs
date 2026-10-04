//! 🧮️ Ticket tool of work package D3, Rust half: recomputes every case of `🗑️generated/d3/d3-bits.json` with the Rust
//! feeling, effects and mischief twins and compares every result with the TypeScript twin's bit for bit.
//!
//! Mounted as the integration test `d3_bits` of the scratch crate (`[[test]] path = "../../../../../d3_check_bits.rs"`
//! in `🗑️generated/d3/crate/📦️packages/🦀️rust/Cargo.toml`, written by `d3_scratch.ts prepare`). Numbers arrive and are
//! compared as sixteen hexadecimal digits of their IEEE-754 pattern (`nan` for every NaN, `null` for a `None`); the
//! synthetic species and the menageries the cases point into are read from the head of the file. A case the
//! TypeScript twin throws on must panic here. The per-function counts and every mismatch are written to
//! `🗑️generated/d3/d3-bits-report.txt`.
//!
//! Run from the repository root, after `bun <ticket>/d3_dump_bits.ts`:
//!   bash <ticket>/rust_scratch.sh d3 test --offline --release --test d3_bits -- --nocapture
//!
//! @see ./d3_dump_bits.ts — the TypeScript half and the layout of every case
//! @see ./check_motion_bits.rs — the pattern (work package L)

use pets::effects::{self as effects, Emission, Particle};
use pets::feeling::{self as feeling, Character, Leaning, Sighting};
use pets::mischief::{self as mischief, Circumstances, Keyed};
use pets::schema::{Activity, Cooling, Facing, Feeling, Fixture, Footing, Menagerie, Mood, Perch, Pitch, Point, Rapport, Reaction, Species, SpeciesState, Text, Ticks, Trait, Trick, ACTIVITIES, CUES, DRIFTS, MOODS, PET_MODES, PLACEMENTS};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::PathBuf;

/// 🔢️ The number a bit pattern stands for.
fn decode(text: &str) -> f64 {
    if text == "nan" {
        f64::NAN
    } else {
        f64::from_bits(u64::from_str_radix(text, 16).unwrap_or_else(|error| panic!("{text}: {error}")))
    }
}

/// 🔡️ The bit pattern of an answer, `nan` for every NaN and `null` for none.
fn encode(value: Option<f64>) -> String {
    match value {
        None => "null".to_string(),
        Some(number) if number.is_nan() => "nan".to_string(),
        Some(number) => format!("{:016x}", number.to_bits()),
    }
}

/// 📚️ The documents the cases point into.
struct Documents {
    species: Vec<Species>,
    menageries: Vec<Menagerie>,
}

/// 📖️ The flat arguments of a case, read front to back.
struct Arguments {
    values: Vec<f64>,
    at: usize,
}

impl Arguments {
    /// ➡️ The next number.
    fn number(&mut self) -> f64 {
        let value = self.values[self.at];
        self.at += 1;
        value
    }

    /// 📇️ The next number as a count or an index.
    fn count(&mut self) -> usize {
        self.number() as usize
    }

    /// ⏱️ The next number as a whole number (ticks, an index that may be negative, a direction).
    fn whole(&mut self) -> i64 {
        self.number() as i64
    }

    /// 🧩️ The next number as an unsigned 32-bit word, modulo 2³².
    fn word(&mut self) -> u32 {
        self.number() as i64 as u32
    }

    /// ✅️ The next number as a flag.
    fn flag(&mut self) -> bool {
        self.number() == 1.0
    }

    /// 😊️ The next number as a mood.
    fn mood(&mut self) -> Mood {
        MOODS[self.count()]
    }

    /// 💓️ The next three numbers as a feeling.
    fn feeling(&mut self) -> Feeling {
        Feeling { mood: self.mood(), intensity: self.number(), since: self.whole() }
    }

    /// 🧬️ The next four numbers as a character.
    fn character(&mut self) -> Character {
        Character { mood: self.mood(), temperament: pets::schema::Temperament { energy: self.number(), sociability: self.number(), curiosity: self.number() } }
    }

    /// 🚿️ The next five numbers as an emission.
    fn emission(&mut self) -> Emission {
        Emission { motion: DRIFTS[self.count()], count: self.number() as u32, life: self.number(), speed: self.number(), spread: self.number() }
    }

    /// 🏁️ A flag and a tick as an optional tick.
    fn until(&mut self) -> Option<Ticks> {
        let has = self.flag();
        let until = self.whole();
        has.then_some(until)
    }

    /// 📦️ The next four numbers as a body of a sighting.
    fn geometric(&mut self) -> Sighting {
        Sighting { species: "geo".to_string(), state: "geo".to_string(), held: 0, mood: Mood::Content, intensity: 0.25, activity: Activity::Idle, trick: None, x: self.number(), y: self.number(), width: self.number(), height: self.number() }
    }

    /// 👀️ The next eleven numbers as a sighting of a menagerie.
    fn sighting(&mut self, menagerie: &Menagerie) -> Sighting {
        let which = self.whole();
        let species = usize::try_from(which).ok().map(|index| &menagerie.species[index]);
        let entered = self.whole();
        let held = self.whole();
        let mood = self.mood();
        let intensity = self.number();
        let activity = ACTIVITIES[self.count()];
        let trick = self.whole();
        Sighting {
            species: species.map_or_else(|| "stranger".to_string(), |species| species.id.clone()),
            state: match (species, usize::try_from(entered)) {
                (Some(species), Ok(index)) => species.states[index].id.clone(),
                _ => "nowhere".to_string(),
            },
            held,
            mood,
            intensity,
            activity,
            trick: match trick {
                -1 => None,
                -2 => Some("ghost".to_string()),
                index => species.map(|species| species.tricks[index as usize].id.clone()),
            },
            x: self.number(),
            y: self.number(),
            width: self.number(),
            height: self.number(),
        }
    }
}

/// 🔦️ The index of a state id in a species, −1 for an id it does not have.
fn state_index(species: &Species, id: &str) -> f64 {
    species.states.iter().position(|state| state.id == id).map_or(-1.0, |index| index as f64)
}

/// 🎯️ A state of a species by index, or an id it does not know for −1.
fn state_name(species: &Species, index: i64) -> String {
    usize::try_from(index).map_or_else(|_| "nowhere".to_string(), |index| species.states[index].id.clone())
}

/// 🎪️ The index of an answered trick in its species, −1 for none.
fn trick_index(species: &Species, trick: Option<&Trick>) -> f64 {
    trick.map_or(-1.0, |trick| species.tricks.iter().position(|candidate| std::ptr::eq(candidate, trick)).map_or(f64::NAN, |index| index as f64))
}

/// 🗂️ The index of an id in a list, −1 for none.
fn index_in(list: &[&str], id: Option<&str>) -> f64 {
    id.and_then(|id| list.iter().position(|known| *known == id)).map_or(-1.0, |index| index as f64)
}

/// 🫀️ The flat form of a feeling.
fn felt(value: Feeling) -> Vec<f64> {
    vec![MOODS.iter().position(|mood| *mood == value.mood).map_or(f64::NAN, |index| index as f64), value.intensity, value.since as f64]
}

/// 🧾️ Numbers as answers.
fn all(numbers: Vec<f64>) -> Vec<Option<f64>> {
    numbers.into_iter().map(Some).collect()
}

/// 🎭️ A trick that leaves `mood` (or none).
fn trick_of(mood: Option<Mood>) -> Trick {
    Trick { id: "trick".to_string(), name: Text { en: "Trick".to_string(), de: "Trick".to_string() }, clip: "clip".to_string(), cues: Vec::new(), emitter: None, from: None, to: None, mood }
}

/// 🫙️ A trait that asks for nothing.
fn nothing() -> Trait {
    Trait { species: None, state: None, held: None, mood: None, activity: None, trick: None }
}

/// 🏷️ An item that carries a key and its place in a list.
struct Item {
    key: String,
    position: usize,
}

impl Keyed for Item {
    /// 🔑️ The key of the item.
    fn key(&self) -> &str {
        &self.key
    }
}

/// 🎬️ A beat of a menagerie: the sightings, coolings, rapports, units and the tick.
fn beat(arguments: &mut Arguments, menagerie: &Menagerie) -> (Vec<Sighting>, Vec<Cooling>, Vec<Rapport>, Vec<f64>, Ticks) {
    let at = arguments.whole();
    let crowd: Vec<Sighting> = (0..arguments.count()).map(|_| arguments.sighting(menagerie)).collect();
    let coolings: Vec<Cooling> = (0..arguments.count())
        .map(|_| Cooling { reaction: menagerie.chemistry[arguments.count()].id.clone(), when: menagerie.species[arguments.count()].id.clone(), near: menagerie.species[arguments.count()].id.clone(), until: arguments.whole() })
        .collect();
    let rapports: Vec<Rapport> = (0..arguments.count()).map(|_| Rapport { between: [menagerie.species[arguments.count()].id.clone(), menagerie.species[arguments.count()].id.clone()], drift: arguments.number() }).collect();
    let units: Vec<f64> = (0..arguments.count()).map(|_| arguments.number()).collect();
    (crowd, coolings, rapports, units, at)
}

/// 🧱️ The constants of the three twins in the order the TypeScript half lists them.
fn constants() -> Vec<f64> {
    let mut numbers: Vec<f64> = feeling::MOOD_PRIORITIES.iter().map(|&rank| f64::from(rank)).collect();
    numbers.extend([feeling::MOOD_HOLD as f64, feeling::MOOD_FAINT, feeling::MOOD_REST, feeling::MOOD_RISE, feeling::MOOD_OVERRIDE]);
    numbers.extend(feeling::MOOD_DECAYS);
    numbers.push(feeling::PRONE_DECAY);
    for table in [feeling::MOOD_VALENCES, feeling::MOOD_LIDS, feeling::MOOD_SLANTS, feeling::MOOD_DROPS] {
        numbers.extend(table);
    }
    for stir in feeling::APPRAISALS {
        numbers.extend([feeling::OCCASIONS.iter().position(|occasion| *occasion == stir.occasion).map_or(f64::NAN, |index| index as f64), MOODS.iter().position(|mood| *mood == stir.mood).map_or(f64::NAN, |index| index as f64), stir.amount]);
    }
    numbers.extend(feeling::PRONENESS.iter().flatten());
    numbers.extend([feeling::PRONE_GAIN, feeling::TRICK_AMOUNT, feeling::DROWSY_ENERGY]);
    numbers.extend(feeling::MOOD_WEIGHTS.iter().flatten());
    numbers.extend(feeling::MOOD_AFFINITIES);
    numbers.extend(feeling::MOOD_SHARES.iter().flatten());
    numbers.extend(feeling::MOOD_SPREADS);
    numbers.extend([feeling::CONTAGION_BEAT as f64, feeling::CONTAGION_REACH]);
    numbers.extend(feeling::WILLING_MOODS.map(|willing| if willing { 1.0 } else { 0.0 }));
    numbers.extend([feeling::WHIM_FAVOR, feeling::CHEMISTRY_BEAT as f64, feeling::EFFECT_AMOUNT, feeling::LEAN_TICKS as f64]);
    numbers.extend([f64::from(effects::EMITTER_CAP), effects::STAGE_CAP as f64, effects::TURN_RADIANS, effects::AHEAD, effects::DOWN, effects::UP]);
    numbers.extend([effects::LANE_BIRTH, effects::LANE_HEADING, effects::LANE_PACE, effects::LANE_PHASE, effects::LANE_LOOK].map(f64::from));
    numbers.extend([
        effects::FALL_SWAY,
        effects::FALL_SWAY_SPEED,
        effects::FALL_SWAY_RATE,
        effects::FALL_FADE_IN as f64,
        effects::RISE_WANDER,
        effects::RISE_WANDER_RATE,
        effects::RISE_ROCK,
        effects::BURST_DRAG,
        effects::BURST_GRAVITY,
        effects::ORBIT_SQUASH,
        effects::ORBIT_DEPTH,
        effects::ORBIT_FADE as f64,
        effects::DRIFT_MEANDER,
        effects::DRIFT_RATE,
    ]);
    numbers.extend([
        mischief::MISCHIEF_WIDTH,
        mischief::MISCHIEF_PATIENCE as f64,
        mischief::MISCHIEF_PATIENCE_QUIET as f64,
        mischief::MISCHIEF_COOLDOWN_CALM as f64,
        mischief::MISCHIEF_COOLDOWN_LIVELY as f64,
        mischief::STATION_GAP,
        mischief::STATION_SLACK,
        mischief::STATION_STEP,
        mischief::LIFT_ROOM,
    ]);
    numbers.extend([mischief::LIFT_BRACE, mischief::LIFT_SHOVE, mischief::LIFT_WOBBLE, mischief::LIFT_HOLD, mischief::LIFT_RETURN, mischief::LIFT_FADE, mischief::LIFT_RETURNS, mischief::LIFT_TICKS, mischief::LIFT_LIMIT].map(|ticks| ticks as f64));
    numbers.extend([mischief::LIFT_LEAST, mischief::LIFT_GIVE, mischief::LIFT_RISE, mischief::LIFT_TILT, mischief::LIFT_WOBBLES, mischief::HALF_TURN_RADIANS, mischief::THROW_SPEED, mischief::THROW_SPREAD, mischief::THROW_LIFT]);
    numbers.extend([mischief::NO_LIFT.dx, mischief::NO_LIFT.dy, mischief::NO_LIFT.tilt, mischief::NO_LIFT.opacity]);
    numbers
}

/// ⚙️ What the Rust twins answer to one case.
#[allow(clippy::too_many_lines)]
fn answer(function: &str, arguments: &mut Arguments, texts: &[String], documents: &Documents) -> Vec<Option<f64>> {
    let flag = |truth: bool| vec![Some(if truth { 1.0 } else { 0.0 })];
    match function {
        "constants" => all(constants()),
        "at_rest" => {
            let mood = arguments.mood();
            all(felt(feeling::at_rest(mood, arguments.whole())))
        }
        "shown_mood" => {
            let mood = arguments.mood();
            let shown = feeling::shown_mood(mood, arguments.number());
            all(vec![MOODS.iter().position(|known| *known == shown).map_or(f64::NAN, |index| index as f64)])
        }
        "impulse" => {
            let (present, mood, amount) = (arguments.feeling(), arguments.mood(), arguments.number());
            all(felt(feeling::impulse(present, mood, amount, arguments.whole())))
        }
        "settled" => {
            let (present, resting) = (arguments.feeling(), arguments.mood());
            all(felt(feeling::settled(present, resting, arguments.whole())))
        }
        "settles_at" => {
            let present = arguments.feeling();
            all(vec![feeling::settles_at(present, arguments.mood()) as f64])
        }
        "calms_at" => {
            let present = arguments.feeling();
            all(vec![feeling::calms_at(present, arguments.mood()) as f64])
        }
        "valence_of" => all(vec![feeling::valence_of(arguments.mood())]),
        "spirits_of" => all(vec![feeling::spirits_of(arguments.feeling())]),
        "face_of" => {
            let face = feeling::face_of(arguments.feeling());
            all(vec![face.bend, face.lid, face.slant, face.drop])
        }
        "proneness_of" => {
            let character = arguments.character();
            all(vec![feeling::proneness_of(character, arguments.mood())])
        }
        "stirred" => {
            let (present, mood, amount, character) = (arguments.feeling(), arguments.mood(), arguments.number(), arguments.character());
            all(felt(feeling::stirred(present, mood, amount, character, arguments.whole())))
        }
        "appraised" => {
            let (present, occasion, character) = (arguments.feeling(), feeling::OCCASIONS[arguments.count()], arguments.character());
            all(felt(feeling::appraised(present, occasion, character, arguments.whole())))
        }
        "performed" => {
            let present = arguments.feeling();
            let mood = usize::try_from(arguments.whole()).ok().map(|index| MOODS[index]);
            let character = arguments.character();
            all(felt(feeling::performed(present, &trick_of(mood), character, arguments.whole())))
        }
        "drowsed" => {
            let (present, energy) = (arguments.feeling(), arguments.number());
            all(felt(feeling::drowsed(present, energy, arguments.whole())))
        }
        "mood_weights" => {
            let mood = arguments.mood();
            all(feeling::mood_weights(mood, arguments.number()).to_vec())
        }
        "encounter_bias" => {
            let leaning = feeling::encounter_bias(arguments.feeling(), arguments.feeling());
            all(vec![leaning.affinity, leaning.shares[0], leaning.shares[1], leaning.shares[2], leaning.show.map_or(-1.0, |side| side as f64)])
        }
        "swayed_shares" => {
            let shares = [arguments.number(), arguments.number(), arguments.number()];
            let affinity = arguments.number();
            let leaning_shares = [arguments.number(), arguments.number(), arguments.number()];
            let show = usize::try_from(arguments.whole()).ok();
            all(feeling::swayed_shares(shares, Leaning { affinity, shares: leaning_shares, show }).to_vec())
        }
        "caught" => {
            let (mine, theirs, affinity, sociability) = (arguments.feeling(), arguments.feeling(), arguments.number(), arguments.number());
            all(felt(feeling::caught(mine, theirs, affinity, sociability, arguments.whole())))
        }
        "lasting_ticks" => {
            let has = arguments.flag();
            let lasts = arguments.number();
            let state = SpeciesState { id: "s".to_string(), name: Text { en: "s".to_string(), de: "s".to_string() }, tint: None, clip: None, emitter: None, lasts: has.then_some(lasts), then: None };
            all(vec![feeling::lasting_ticks(&state) as f64])
        }
        "held_ticks" => all(vec![feeling::held_ticks(arguments.number()) as f64]),
        "every_ticks" => {
            let reaction = Reaction { id: "r".to_string(), when: nothing(), near: nothing(), within: 0.0, place: None, unless: None, affinity: None, every: arguments.number(), chance: None, then: Vec::new() };
            all(vec![feeling::every_ticks(&reaction) as f64])
        }
        "state_at" => {
            let species = &documents.species[arguments.count()];
            let state = state_name(species, arguments.whole());
            let (since, at) = (arguments.whole(), arguments.whole());
            let standing = feeling::state_at(species, &state, since, at);
            all(vec![state_index(species, &standing.state), standing.since as f64])
        }
        "state_ends" => {
            let species = &documents.species[arguments.count()];
            let state = state_name(species, arguments.whole());
            vec![feeling::state_ends(species, &state, arguments.whole()).map(|tick| tick as f64)]
        }
        "ladder_of" => {
            let species = &documents.species[arguments.count()];
            all(feeling::ladder_of(species).iter().map(|id| state_index(species, id)).collect())
        }
        "rungs_of" => {
            let species = &documents.species[arguments.count()];
            let trick = &species.tricks[arguments.count()];
            all(feeling::rungs_of(species, trick).iter().map(|id| state_index(species, id)).collect())
        }
        "state_after_trick" => {
            let species = &documents.species[arguments.count()];
            let state = state_name(species, arguments.whole());
            let trick = &species.tricks[arguments.count()];
            all(vec![state_index(species, &feeling::state_after_trick(species, &state, trick))])
        }
        "step_rung" => {
            let species = &documents.species[arguments.count()];
            let trick = &species.tricks[arguments.count()];
            let state = state_name(species, arguments.whole());
            all(vec![state_index(species, &feeling::step_rung(&feeling::rungs_of(species, trick), &state, arguments.whole()))])
        }
        "step_state" => {
            let species = &documents.species[arguments.count()];
            let state = state_name(species, arguments.whole());
            all(vec![state_index(species, &feeling::step_state(species, &state, arguments.whole()))])
        }
        "tricks_for" => {
            let species = &documents.species[arguments.count()];
            let cue = CUES[arguments.count()];
            let state = state_name(species, arguments.whole());
            all(feeling::tricks_for(species, cue, &state, arguments.feeling()).into_iter().map(|trick| trick_index(species, Some(trick))).collect())
        }
        "click_trick" | "whim_trick" | "show_trick" => {
            let species = &documents.species[arguments.count()];
            let state = state_name(species, arguments.whole());
            let present = arguments.feeling();
            let trick = match function {
                "click_trick" => feeling::click_trick(species, &state, present, arguments.whole()),
                "whim_trick" => feeling::whim_trick(species, &state, present, arguments.number()),
                _ => feeling::show_trick(species, &state, present, arguments.number()),
            };
            all(vec![trick_index(species, trick)])
        }
        "nearby" => {
            let (first, second) = (arguments.geometric(), arguments.geometric());
            flag(feeling::nearby(&first, &second, arguments.number()))
        }
        "seen" => {
            let (first, second) = (arguments.geometric(), arguments.geometric());
            flag(feeling::seen(&first, &second, PLACEMENTS[arguments.count()]))
        }
        "matches" => {
            let menagerie = &documents.menageries[arguments.count()];
            let reaction = &menagerie.chemistry[arguments.count()];
            let wanted = match arguments.count() {
                0 => &reaction.when,
                1 => &reaction.near,
                _ => reaction.unless.as_ref().unwrap_or(&reaction.when),
            };
            let sighting = arguments.sighting(menagerie);
            flag(feeling::matches(wanted, &sighting))
        }
        "trials_of" => {
            let menagerie = &documents.menageries[arguments.count()];
            let (crowd, coolings, rapports, _, at) = beat(arguments, menagerie);
            let trials = feeling::trials_of(menagerie, &crowd, &coolings, at, &rapports);
            let mut numbers = vec![trials.len() as f64];
            for trial in &trials {
                numbers.extend([trial.reaction as f64, trial.when as f64, trial.near as f64, if trial.chancy { 1.0 } else { 0.0 }]);
            }
            numbers.push(feeling::draws_of(&trials) as f64);
            all(numbers)
        }
        "reactions_of" => {
            let menagerie = &documents.menageries[arguments.count()];
            let (crowd, coolings, rapports, units, at) = beat(arguments, menagerie);
            let outcome = feeling::reactions_of(menagerie, &crowd, at, &coolings, &units, &rapports);
            let names: Vec<&str> = menagerie.species.iter().map(|species| species.id.as_str()).collect();
            let reactions: Vec<&str> = menagerie.chemistry.iter().map(|reaction| reaction.id.as_str()).collect();
            let mut numbers = vec![outcome.consequences.len() as f64];
            for consequence in &outcome.consequences {
                let species = menagerie.species.iter().find(|species| species.id == consequence.on).unwrap_or_else(|| panic!("no species {}", consequence.on));
                numbers.extend([
                    index_in(&reactions, Some(&consequence.reaction)),
                    index_in(&names, Some(&consequence.on)),
                    index_in(&names, Some(&consequence.other)),
                    consequence.state.as_ref().map_or(-1.0, |state| state_index(species, state)),
                    consequence.mood.map_or(-1.0, |mood| MOODS.iter().position(|known| *known == mood).map_or(f64::NAN, |index| index as f64)),
                    consequence.amount,
                    consequence.rapport,
                    consequence.encounter.map_or(-1.0, |encounter| ACTIVITIES.iter().position(|known| *known == encounter).map_or(f64::NAN, |index| index as f64)),
                    consequence.trick.as_ref().map_or(-1.0, |trick| species.tricks.iter().position(|offered| offered.id == *trick).map_or(-1.0, |index| index as f64)),
                    consequence.activity.map_or(-1.0, |activity| ACTIVITIES.iter().position(|known| *known == activity).map_or(f64::NAN, |index| index as f64)),
                ]);
            }
            numbers.push(outcome.coolings.len() as f64);
            for cooling in &outcome.coolings {
                numbers.extend([index_in(&reactions, Some(&cooling.reaction)), index_in(&names, Some(&cooling.when)), index_in(&names, Some(&cooling.near)), cooling.until as f64]);
            }
            numbers.push(outcome.drawn as f64);
            all(numbers)
        }
        "lowbias32" => all(vec![f64::from(effects::lowbias32(arguments.word()))]),
        "mix" => all(vec![f64::from(effects::mix(arguments.word(), arguments.word()))]),
        "unit" => all(vec![effects::unit(arguments.word())]),
        "scattered" => {
            let (key, index, lane) = (arguments.word(), arguments.whole(), arguments.word());
            all(vec![effects::scattered(key, index, lane)])
        }
        "emitter_key" => {
            let (seed, species, emitter, since) = (arguments.word(), arguments.count(), arguments.count(), arguments.whole());
            all(vec![f64::from(effects::emitter_key(seed, species, emitter, since))])
        }
        "life_ticks" => all(vec![effects::life_ticks(arguments.emission()) as f64]),
        "swarm_of" => all(vec![f64::from(effects::swarm_of(arguments.emission()))]),
        "period_of" => all(vec![effects::period_of(arguments.emission()) as f64]),
        "born_at" => {
            let (emission, since, key, index) = (arguments.emission(), arguments.whole(), arguments.word(), arguments.whole());
            all(vec![effects::born_at(emission, since, key, index) as f64])
        }
        "emitter_ends" => {
            let (emission, since, until) = (arguments.emission(), arguments.whole(), arguments.until());
            vec![effects::emitter_ends(emission, since, until).map(|tick| tick as f64)]
        }
        "particles_of" => {
            let emission = arguments.emission();
            let origin = Point { x: arguments.number(), y: arguments.number() };
            let facing = if arguments.number() > 0.0 { Facing::Right } else { Facing::Left };
            let (since, until, at, key) = (arguments.whole(), arguments.until(), arguments.whole(), arguments.word());
            let particles = effects::particles_of(emission, origin, facing, since, until, at, key);
            let mut numbers = vec![particles.len() as f64];
            for particle in &particles {
                numbers.extend([particle.x, particle.y, particle.scale, particle.rotation, particle.opacity, particle.age as f64]);
            }
            all(numbers)
        }
        "capped" => {
            let (cap, size) = (arguments.count(), arguments.count());
            let particles: Vec<Particle> = (0..size).map(|position| Particle { x: position as f64, y: 0.0, scale: 1.0, rotation: 0.0, opacity: 1.0, age: arguments.whole() }).collect();
            all(effects::capped(particles, cap).iter().map(|particle| particle.x).collect())
        }
        "fits" => flag(mischief::fits(&texts[0], &texts[1])),
        "fixture_for" => {
            let (grounds, keys) = (arguments.count(), arguments.count());
            let items: Vec<Item> = texts[grounds..grounds + keys].iter().enumerate().map(|(position, key)| Item { key: key.clone(), position }).collect();
            all(mischief::fixture_for(&texts[..grounds], &items).iter().map(|item| item.position as f64).collect())
        }
        "chosen_fixture" => {
            let positions: Vec<usize> = (0..arguments.count()).collect();
            all(vec![mischief::chosen_fixture(&positions, arguments.number()).map_or(-1.0, |&position| position as f64)])
        }
        "patience_of" => all(vec![mischief::patience_of(arguments.flag()) as f64]),
        "cooldown_of" => vec![mischief::cooldown_of(PET_MODES[arguments.count()]).map(|ticks| ticks as f64)],
        "allowed_from" | "allowed" => {
            let circumstances = Circumstances {
                permitted: arguments.flag(),
                fine: arguments.flag(),
                width: arguments.number(),
                mode: PET_MODES[arguments.count()],
                quiet: arguments.flag(),
                lifting: arguments.flag(),
                tick: arguments.whole(),
                stirred: arguments.whole(),
                rested: arguments.whole(),
            };
            if function == "allowed" {
                flag(mischief::allowed(circumstances))
            } else {
                vec![mischief::allowed_from(circumstances).map(|tick| tick as f64)]
            }
        }
        "station_for" => {
            let fixture = Fixture { id: "fixture".to_string(), key: "heating".to_string(), x: arguments.number(), y: arguments.number(), width: arguments.number(), height: arguments.number() };
            let pitches: Vec<Pitch> = (0..arguments.count())
                .map(|place| Pitch { wall: format!("w{place}"), surface: format!("s{place}"), side: if arguments.number() > 0.0 { Facing::Right } else { Facing::Left }, x: arguments.number(), y0: arguments.number(), y1: arguments.number() })
                .collect();
            let perches: Vec<Perch> = (0..arguments.count()).map(|place| Perch { surface: format!("p{place}"), x0: arguments.number(), x1: arguments.number(), y: arguments.number() }).collect();
            match mischief::station_for(&fixture, &pitches, &perches, arguments.number()) {
                None => all(vec![0.0]),
                Some(station) => {
                    let wall = station.footing == Footing::Wall;
                    let place = if wall { pitches.iter().position(|pitch| Some(&pitch.wall) == station.wall.as_ref()) } else { perches.iter().position(|perch| perch.surface == station.surface) };
                    all(vec![1.0, if wall { 1.0 } else { 0.0 }, place.map_or(-1.0, |index| index as f64), station.x, station.y, station.side.sign(), station.room])
                }
            }
        }
        "lift_at" => {
            let (since, at) = (arguments.whole(), arguments.whole());
            let side = if arguments.number() > 0.0 { Facing::Right } else { Facing::Left };
            let lift = mischief::lift_at(since, at, side, arguments.number(), arguments.number(), arguments.number());
            all(vec![lift.dx, lift.dy, lift.tilt, lift.opacity])
        }
        "lift_wake" => {
            let since = arguments.whole();
            vec![mischief::lift_wake(since, arguments.whole()).map(|tick| tick as f64)]
        }
        "lift_ends" => all(vec![mischief::lift_ends(arguments.whole()) as f64]),
        "thrown_off" => {
            let pusher = Point { x: arguments.number(), y: arguments.number() };
            let fixture = Fixture { id: "f".to_string(), key: "k".to_string(), x: arguments.number(), y: arguments.number(), width: arguments.number(), height: arguments.number() };
            let toss = mischief::thrown_off(pusher, &fixture, arguments.number());
            all(vec![toss.vx, toss.vy])
        }
        other => panic!("no Rust twin is wired for {other}"),
    }
}

#[test]
fn rust_twins_reproduce_every_bit_of_the_typescript_twins() {
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let source = directory.join("d3-bits.json");
    let document: Value = serde_json::from_str(&std::fs::read_to_string(&source).unwrap_or_else(|error| panic!("{}: {error} — run d3_dump_bits.ts first", source.display()))).unwrap_or_else(|error| panic!("{}: {error}", source.display()));
    let documents = Documents {
        species: serde_json::from_value(document["species"].clone()).unwrap_or_else(|error| panic!("species: {error}")),
        menageries: serde_json::from_value(document["menageries"].clone()).unwrap_or_else(|error| panic!("menageries: {error}")),
    };
    let cases = document["cases"].as_array().cloned().unwrap_or_default();
    let mut tally: BTreeMap<String, [usize; 4]> = BTreeMap::new();
    let mut mismatches: Vec<String> = Vec::new();
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    for (index, case) in cases.iter().enumerate() {
        let function = case["fn"].as_str().unwrap_or_default();
        let texts = |value: &Value| -> Vec<String> { value.as_array().into_iter().flatten().filter_map(Value::as_str).map(str::to_string).collect() };
        let arguments = texts(&case["args"]);
        let words = texts(&case["text"]);
        let expected = if case["out"] == "throws" { None } else { Some(texts(&case["out"])) };
        let produced = catch_unwind(AssertUnwindSafe(|| {
            let mut reader = Arguments { values: arguments.iter().map(|text| decode(text)).collect(), at: 0 };
            let answered = answer(function, &mut reader, &words, &documents);
            assert_eq!(reader.at, reader.values.len(), "arguments left over");
            answered.into_iter().map(encode).collect::<Vec<_>>()
        }))
        .ok();
        let counted = tally.entry(function.to_string()).or_default();
        counted[0] += 1;
        counted[1] += expected.as_ref().map_or(0, Vec::len);
        counted[2] += usize::from(expected.is_none());
        if produced != expected {
            counted[3] += 1;
            mismatches.push(format!("case {index} {function}({}; {words:?}): typescript {expected:?}, rust {produced:?}", arguments.join(", ")));
        }
    }
    std::panic::set_hook(hook);
    let mut report = String::new();
    let totals = tally.values().fold([0; 4], |sum, counted| [sum[0] + counted[0], sum[1] + counted[1], sum[2] + counted[2], sum[3] + counted[3]]);
    for (function, counted) in &tally {
        let _ = writeln!(report, "{function}: {} cases, {} result numbers, {} throwing, {} mismatches", counted[0], counted[1], counted[2], counted[3]);
    }
    let _ = writeln!(report, "total: {} cases, {} result numbers, {} throwing, {} mismatches", totals[0], totals[1], totals[2], totals[3]);
    for function in tally.keys() {
        for mismatch in mismatches.iter().filter(|mismatch| mismatch.split(' ').nth(2).is_some_and(|word| word.starts_with(&format!("{function}(")))).take(5) {
            let _ = writeln!(report, "{mismatch}");
        }
    }
    std::fs::write(directory.join("d3-bits-report.txt"), &report).unwrap_or_else(|error| panic!("{error}"));
    println!("{}", report.lines().take(80).collect::<Vec<_>>().join("\n"));
    assert!(cases.len() > 100_000, "{} cases", cases.len());
    assert!(mismatches.is_empty(), "{} of {} cases differ", mismatches.len(), cases.len());
}
