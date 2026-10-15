import os,re
PETS=os.environ.get('PETS_ROOT','/Users/ueli/Documents/semio/🧰️framework/🛍️products/🐾️pets')
MODS=PETS+'/🔨️modules'
def mod_path(name):
    if name=='schema': return PETS+'/🧬️schema'
    for d in os.listdir(MODS):
        if re.sub(r'[^\x00-\x7f]','',d)==name: return MODS+'/'+d
    raise KeyError(name)
CRATES={
 'contract':   dict(node='📜️contract', mods=['schema','validation'], deps=[], emoji='📜️'),
 'kinematics': dict(node='⚙️kinematics', mods=['trigonometry','randomness','rig','animation','terrain'], deps=['contract'], emoji='⚙️'),
 'clearance':  dict(node='🚧️clearance', mods=['clearance'], deps=['contract','kinematics'], emoji='🚧️', root='module'),
 'traversal':  dict(node='🧭️traversal', mods=['swing','climbing'], deps=['contract','kinematics'], emoji='🧭️'),
 'interaction':dict(node='🖱️interaction', mods=['gesture','effects','mischief'], deps=['contract','kinematics','traversal'], emoji='🖱️'),
 'character':  dict(node='🎭️character', mods=['behavior','feeling'], deps=['contract','kinematics'], emoji='🎭️'),
 'staging':    dict(node='🎬️staging', mods=['draft','spacing','schedule','attention','locomotion','sociability','choice','population','clock','projection','stage'], deps=['contract','kinematics','character','interaction'], emoji='🎬️'),
}
OWNER={m:c for c,v in CRATES.items() for m in v['mods']}
def ident(c): return 'semio_framework_pets_'+c
def crate_name(c): return 'semio-framework-pets-'+c
def node_dir(c): return MODS+'/'+CRATES[c]['node']
def pkg_dir(c): return node_dir(c)+'/📦️packages/🦀️rust'
def src(m): return mod_path(m)+'/🦀️.rs'
def tst(m): return mod_path(m)+'/🧪️tests/🔬️unit/🦀️.rs'
