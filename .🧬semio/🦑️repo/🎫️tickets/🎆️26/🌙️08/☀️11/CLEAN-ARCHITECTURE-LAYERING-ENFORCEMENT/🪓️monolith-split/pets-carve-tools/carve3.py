import os,re,sys,json
from plan import *
from deps import analyse

DOC={
 'contract':('📜️','The pets contract: the hand-written Rust twin of the normative schema `🧬️schema/🔣️.json` (menageries of rigged species, the stage, its events and its frames) and the validation of documents against the rules serde cannot express. Every other pets crate builds on these types; this crate depends on no other pets crate.'),
 'kinematics':('⚙️','The numerical and kinematic ground of the pets: deterministic trigonometry in turns, counter-based randomness, the forward kinematics of a rig, animation (clips, springs, blinks) and terrain (perches, walls, strides, falls and hops). Pure functions over the contract, bit for bit the results of the TypeScript twin.'),
 'traversal':('🧭️','How pets get around where walking and hopping do not reach: everything that hangs (a pet in the hand, under its parachute, on a reeled rope) and climbing (walls, ladders, grappling ropes, routes).'),
 'interaction':('🖱️','How the learner and the page act on the pets: the press, the heat and the gestures of the learner\'s hand, the particles of the effects and the mischief with the things of the page.'),
 'character':('🎭️','The character of a pet in numbers: behaviour (mode limits, activity weights, dwells, the activity graph, casts, encounters, needs) and feeling (moods, states, tricks and the chemistry between species).'),
 'staging':('🎬️','The stage that folds events into frames: the draft it works on, spacing, schedule, attention, locomotion, sociability, choice, population, the clock and the projection of a stage into a frame. The stage is the only module the packages export; the others are its parts.'),
}
DESC={
 'contract':'Pets contract: the Rust twin of the normative pets schema (menageries of rigged species, the stage, its events and frames) and the validation of documents. Pure functions only, bit for bit the results of the TypeScript twin @semio-tech/pets.',
 'kinematics':'Pets numerical and kinematic ground: deterministic trigonometry in turns, counter-based randomness, the forward kinematics of a rig, animation (clips, springs, blinks) and terrain (perches, walls, strides, falls, hops). Pure functions only.',
 'clearance':'Pets clearance: the bodies and claimed corridors that keep pets apart, and the proof that no two overlap. Pure functions only, bit for bit the results of the TypeScript twin @semio-tech/pets.',
 'traversal':'Pets traversal: the swing of ropes and parachutes and climbing (walls, ladders, grappling ropes, routes). Pure functions only, bit for bit the results of the TypeScript twin @semio-tech/pets.',
 'interaction':'Pets interaction: gesture recognition of the learner\'s hand, effects (particles) and mischief with the page. Pure functions only, bit for bit the results of the TypeScript twin @semio-tech/pets.',
 'character':'Pets character: behaviour (weights, dwells, encounters, needs, casts) and feeling (moods, states, chemistry). Pure functions only, bit for bit the results of the TypeScript twin @semio-tech/pets.',
 'staging':'Pets staging: the stage that folds events into frames, with its draft, spacing, schedule, attention, locomotion, sociability, choice, population, clock and projection. Pure functions only, bit for bit the results of the TypeScript twin @semio-tech/pets.',
}
PUBLIC_STAGING={'stage'}

def rel(frm,to): return os.path.relpath(to,frm)

def glue(c):
    emoji,doc=DOC[c]
    node=node_dir(c)
    out=[f'//! {emoji} {doc}','//!','//! @see ../../README.md — the domain model','//! @see ../../🧬️schema/🔣️.json — the normative contract','']
    for m in CRATES[c]['mods']:
        vis='pub mod' if (c!='staging' or m in PUBLIC_STAGING) else 'mod'
        out+= [f'#[path = "{rel(node,src(m))}"]',f'{vis} {m};','']
    if c=='contract':
        out+=['#[cfg(any(test, feature = "testkit"))]',f'#[path = "{rel(node,"/".join([PETS,"🧬️schema/🧪️tests/🧰️testkit/🦀️.rs"]))}"]','pub mod testkit;','']
    return '\n'.join(out).rstrip('\n')+'\n'

def manifest(c,srcd,tstd,sj,kit):
    pkg=pkg_dir(c)
    up='/'.join(['..']*len(os.path.relpath(pkg,'/Users/ueli/Documents/semio').split('/')))
    lib_rel='../../🦀️.rs'
    lines=['[package]',f'workspace = "{up}"',f'name = "{crate_name(c)}"','version = "0.1.0"','edition = "2021"','rust-version.workspace = true',f'description = "{DESC[c]}"','',
      '[package.metadata.semio]','role = "product"',f'id = "pets-{c}"','',
      '[package.metadata.semio.rust-source-boundary]','maxSourceBytes = 262144','maxSourceFiles = 32','maxActiveLines = 4000','',
      '[lints]','workspace = true','',
      '[lib]',f'path = "{lib_rel}"','']
    if c=='contract':
        lines+=['[features]','testkit = ["dep:serde_json"]','']
    deps=[]
    uses_serde=c in ('contract','kinematics','traversal','interaction','character')
    if uses_serde: deps.append('serde = { workspace = true }')
    for d in sorted(srcd[c]):
        deps.append(f'{crate_name(d)} = {{ path = "{rel(pkg,pkg_dir(d))}" }}')
    if c=='contract':
        deps.append('# 🎯️ `float_roundtrip`: without it serde_json reads a long decimal one unit in the last place off, while JSON.parse and Python round correctly; the test kit feeds the unit tests the very doubles the other languages read.')
        deps.append('serde_json = { workspace = true, features = ["float_roundtrip"], optional = true }')
    lines+=['[dependencies]']+deps+['']
    dev=[]
    for d in sorted(tstd[c]-srcd[c]):
        dev.append(f'{crate_name(d)} = {{ path = "{rel(pkg,pkg_dir(d))}" }}')
    if kit[c] and c!='contract':
        dev.append(f'{crate_name("contract")} = {{ path = "{rel(pkg,pkg_dir("contract"))}", features = ["testkit"] }}')
    if sj[c]:
        dev.append('serde_json = { workspace = true, features = ["float_roundtrip"] }')
    if dev: lines+=['[dev-dependencies]']+dev+['']
    return '\n'.join(lines)

def project_json(c):
    return json.dumps({"name":f"@semio-tech/pets-{c}-rs","tags":["lang:rust","role:product","family:pets"]},indent=2,ensure_ascii=False)+'\n'

def main():
    srcd,tstd,sj,kit=analyse()
    for c in CRATES:
        os.makedirs(pkg_dir(c),exist_ok=True)
        if CRATES[c].get('root')!='module':
            open(node_dir(c)+'/🦀️.rs','w').write(glue(c))
        open(pkg_dir(c)+'/Cargo.toml','w').write(manifest(c,srcd,tstd,sj,kit))
        open(pkg_dir(c)+'/📋️project.json','w').write(project_json(c))
        print(c,'written')
main()
