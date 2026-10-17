# 🌉️ Moves the wasm32-only guest bridge mount of the sequence editor from the editor nest to the artifact root (the facade keeps it above every layer: it names the host and its own protocol child). Idempotent.
R='/Users/ueli/Documents/semio/'
N=R+'✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🦀️.rs'
n=open(N,encoding='utf8').read()
old='''        #[cfg(target_arch = "wasm32")]
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧩️component.rs"]
        pub mod wasm;
'''
if old in n:
    n=n.replace(old,'')
    anchor='''pub mod editor_unit_tests;
'''
    assert n.count(anchor)==1
    n=n.replace(anchor,anchor+'''
/// 🌉️ The wasm32 guest bridge of the sequence editor (byte/message ABI plus its protocol child): it drives `SequenceHost`, so the
/// facade mounts it above every layer instead of the editor nest.
#[cfg(target_arch = "wasm32")]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧩️component.rs"]
pub mod editor_wasm;
''')
    open(N,'w',encoding='utf8').write(n)
    print('moved')
