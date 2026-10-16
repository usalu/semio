# 🔄️ Wrapper of oksh.py for the WP7 sync carve: file ownership overrides for the actor modules (new files of the pre-edit) and the shared test files.
# usage: oksql.py <carve.py arguments>
import os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + '/../oksh-tools')
import oksh
import model, plan as P, carve
SY = model.S + '/🔄️sync'
model.FILE_OVERRIDE[SY + '/🖥️native/🎭️actor/🦀️.rs'] = 'os-store-sync-native'
model.FILE_OVERRIDE[SY + '/🌐️wasm/🎭️actor/🦀️.rs'] = 'os-store-sync-wasm'
for _t in ('🔬️document-echo-suppression', '🔬️document-link-shortage', '🔬️document-socket-connect', '🔬️persistence-data-class', '🧪️command-rejection'):
    model.FILE_OVERRIDE[SY + '/🧪️tests/' + _t + '/🦀️.rs'] = 'os-store-sync'
for _p in ('🖥️native', '🌐️wasm'):
    model.FILE_OVERRIDE[SY + '/' + _p + '/🧪️tests/🔬️echo-suppression-pin/🦀️.rs'] = 'os-store-sync-native' if _p == '🖥️native' else 'os-store-sync-wasm'
if __name__ == '__main__':
    import textwrap
    src = open(HERE + '/../okwp23-tools/carve.py', encoding='utf8').read()
    main = textwrap.dedent(src.split("if __name__ == '__main__':\n", 1)[1])
    sys.argv = ['carve.py'] + sys.argv[1:]
    exec(compile(main, 'carve-main', 'exec'), carve.__dict__)
